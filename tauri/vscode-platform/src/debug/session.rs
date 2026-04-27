//! RawDebugSession — sequence numbering, request/response correlation,
//! cancellation, and DebugAdapterTracker hooks.
//!
//! Mirrors rawDebugSession.ts + node/debugAdapter.ts patterns.

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};

use anyhow::{anyhow, Context, Result};
use futures::StreamExt;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;
use tracing::instrument;

use super::{
    adapter::DebugAdapter,
    protocol::{MessageKind, ProtocolMessage},
};

// ─────────────────────────────────────────────────────────────────────────────
// DebugAdapterTracker
// ─────────────────────────────────────────────────────────────────────────────

/// Hooks invoked for every outbound and inbound DAP message.
///
/// Mirrors `IDebugAdapterTracker` / `registerDebugAdapterTrackerFactory`.
pub trait DebugAdapterTracker: Send + Sync {
    fn on_will_send(&self, msg: &ProtocolMessage);
    fn on_did_receive(&self, msg: &ProtocolMessage);
}

/// No-op tracker used when none is registered.
pub struct NoopTracker;

impl DebugAdapterTracker for NoopTracker {
    fn on_will_send(&self, _msg: &ProtocolMessage) {}
    fn on_did_receive(&self, _msg: &ProtocolMessage) {}
}

// ─────────────────────────────────────────────────────────────────────────────
// RawDebugSession
// ─────────────────────────────────────────────────────────────────────────────

type PendingMap = Arc<Mutex<HashMap<u64, oneshot::Sender<ProtocolMessage>>>>;

/// Manages one debug adapter connection: seq numbering, request correlation,
/// cancellation, and tracker hooks.
pub struct RawDebugSession {
    seq: Arc<AtomicU64>,
    pending: PendingMap,
    tracker: Arc<dyn DebugAdapterTracker>,
    /// Send-side channel to the adapter write loop.
    outbound_tx: tokio::sync::mpsc::Sender<ProtocolMessage>,
    /// Background task handle for the reader dispatch loop.
    _reader_task: tokio::task::JoinHandle<()>,
    /// Background task handle for the writer loop.
    _writer_task: tokio::task::JoinHandle<()>,
}

impl RawDebugSession {
    /// Create a session wrapping `adapter`.
    ///
    /// Spawns two background tasks:
    /// 1. writer: reads from internal channel, sends to adapter (fires `on_will_send`).
    /// 2. reader: reads from adapter stream, routes responses (fires `on_did_receive`).
    #[instrument(skip_all)]
    pub fn create(
        mut adapter: impl DebugAdapter + 'static,
        tracker: Arc<dyn DebugAdapterTracker>,
        cancel: CancellationToken,
    ) -> Self {
        let seq = Arc::new(AtomicU64::new(1));
        let pending: PendingMap = Arc::new(Mutex::new(HashMap::new()));

        // Obtain the inbound stream *before* moving adapter into the write task.
        let stream = adapter.message_stream(); // BoxStream<'static, ...>

        let (outbound_tx, mut outbound_rx) =
            tokio::sync::mpsc::channel::<ProtocolMessage>(64);

        let tracker_w = Arc::clone(&tracker);
        let tracker_r = Arc::clone(&tracker);
        let pending_r = Arc::clone(&pending);
        let cancel_r = cancel.clone();

        // Writer task: outbound_rx -> adapter
        let writer_task = tokio::spawn(async move {
            while let Some(msg) = outbound_rx.recv().await {
                // NOTE: tracker fires in custom_request before enqueue; we could
                // fire here too for other callers but the spec says "once per direction"
                // so we fire in custom_request to avoid double-firing.
                let result = match msg.kind {
                    MessageKind::Request => adapter.send_request(msg).await,
                    MessageKind::Response => adapter.send_response(msg).await,
                    MessageKind::Event => adapter.send_event(msg).await,
                };
                if let Err(e) = result {
                    tracing::error!("adapter send error: {e}");
                    break;
                }
            }
        });

        // Reader dispatch task: adapter stream -> pending senders
        let reader_task = tokio::spawn(async move {
            futures::pin_mut!(stream);
            loop {
                tokio::select! {
                    _ = cancel_r.cancelled() => {
                        pending_r.lock().unwrap().clear();
                        break;
                    }
                    item = stream.next() => {
                        match item {
                            None => break,
                            Some(Err(e)) => {
                                tracing::error!("adapter read error: {e}");
                                break;
                            }
                            Some(Ok(msg)) => {
                                tracker_r.on_did_receive(&msg);
                                if msg.kind == MessageKind::Response {
                                    if let Some(req_seq) = msg.request_seq {
                                        let sender = pending_r
                                            .lock()
                                            .unwrap()
                                            .remove(&req_seq);
                                        if let Some(tx) = sender {
                                            let _ = tx.send(msg);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        Self {
            seq,
            pending,
            tracker,
            outbound_tx,
            _reader_task: reader_task,
            _writer_task: writer_task,
        }
    }

    /// Allocate the next sequence number.
    fn next_seq(&self) -> u64 {
        self.seq.fetch_add(1, Ordering::SeqCst)
    }

    /// Send a typed request and await the response body deserialized as `R`.
    ///
    /// If `cancel` fires, the pending entry is removed and the future resolves
    /// with `Err("request cancelled")`.
    pub async fn custom_request<A, R>(
        &self,
        command: impl Into<String>,
        args: Option<A>,
        cancel: Option<CancellationToken>,
    ) -> Result<R>
    where
        A: Serialize + Send,
        R: DeserializeOwned,
    {
        let seq = self.next_seq();
        let arguments = match args {
            Some(a) => Some(serde_json::to_value(a).context("serialize args")?),
            None => None,
        };
        let msg = ProtocolMessage::request(seq, command, arguments);

        // Register pending before sending to avoid race
        let (tx, rx) = oneshot::channel();
        self.pending.lock().unwrap().insert(seq, tx);

        // Fire tracker hook
        self.tracker.on_will_send(&msg);

        // Enqueue for write task
        self.outbound_tx
            .send(msg)
            .await
            .map_err(|_| anyhow!("adapter channel closed"))?;

        // Await response
        let response = if let Some(ct) = cancel {
            tokio::select! {
                _ = ct.cancelled() => {
                    self.pending.lock().unwrap().remove(&seq);
                    return Err(anyhow!("request cancelled"));
                }
                res = rx => res.map_err(|_| anyhow!("adapter dropped pending"))?,
            }
        } else {
            rx.await.map_err(|_| anyhow!("adapter dropped pending"))?
        };

        if response.success == Some(false) {
            let err_msg = response
                .message
                .unwrap_or_else(|| "adapter returned error".into());
            return Err(anyhow!("DAP error: {err_msg}"));
        }

        let body = response.body.unwrap_or(serde_json::Value::Null);
        serde_json::from_value::<R>(body).context("deserialize response body")
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::{
        adapter::DebugAdapter,
        protocol::{Capabilities, ProtocolMessage},
    };
    use async_trait::async_trait;
    use futures::stream::BoxStream;
    use tokio::sync::mpsc;

    // ─── Mock adapter ────────────────────────────────────────────────────────

    struct MockAdapterHandle {
        /// Receive requests that the session sends to the adapter.
        request_rx: mpsc::Receiver<ProtocolMessage>,
        /// Inject response messages into the session.
        response_tx: mpsc::Sender<Result<ProtocolMessage>>,
    }

    struct ChannelAdapter {
        req_tx: mpsc::Sender<ProtocolMessage>,
        resp_rx: Option<mpsc::Receiver<Result<ProtocolMessage>>>,
    }

    #[async_trait]
    impl DebugAdapter for ChannelAdapter {
        async fn send_request(&mut self, msg: ProtocolMessage) -> Result<()> {
            self.req_tx.send(msg).await.map_err(|e| anyhow!("{e}"))
        }
        async fn send_response(&mut self, msg: ProtocolMessage) -> Result<()> {
            self.req_tx.send(msg).await.map_err(|e| anyhow!("{e}"))
        }
        async fn send_event(&mut self, msg: ProtocolMessage) -> Result<()> {
            self.req_tx.send(msg).await.map_err(|e| anyhow!("{e}"))
        }
        fn message_stream(&mut self) -> BoxStream<'static, Result<ProtocolMessage>> {
            let rx = self.resp_rx.take().expect("message_stream called twice");
            Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx))
        }
    }

    fn make_mock() -> (MockAdapterHandle, ChannelAdapter) {
        let (req_tx, req_rx) = mpsc::channel::<ProtocolMessage>(32);
        let (resp_tx, resp_rx) = mpsc::channel::<Result<ProtocolMessage>>(32);

        let handle = MockAdapterHandle {
            request_rx: req_rx,
            response_tx: resp_tx,
        };
        let adapter = ChannelAdapter {
            req_tx,
            resp_rx: Some(resp_rx),
        };
        (handle, adapter)
    }

    // ─── Tracker spy ─────────────────────────────────────────────────────────

    #[derive(Default)]
    struct SpyTracker {
        sent: Arc<Mutex<Vec<String>>>,
        received: Arc<Mutex<Vec<String>>>,
    }

    impl DebugAdapterTracker for SpyTracker {
        fn on_will_send(&self, msg: &ProtocolMessage) {
            let label = msg.command.clone()
                .or_else(|| msg.event.clone())
                .unwrap_or_default();
            self.sent.lock().unwrap().push(label);
        }
        fn on_did_receive(&self, msg: &ProtocolMessage) {
            let label = msg.command.clone()
                .or_else(|| msg.event.clone())
                .unwrap_or_default();
            self.received.lock().unwrap().push(label);
        }
    }

    // ─── Test 1: initialize request returns Capabilities ─────────────────────

    #[tokio::test]
    async fn initialize_request_returns_capabilities() {
        let (mut handle, adapter) = make_mock();
        let cancel = CancellationToken::new();
        let session = RawDebugSession::create(adapter, Arc::new(NoopTracker), cancel.clone());

        // Respond to initialize in background
        tokio::spawn(async move {
            let req = handle.request_rx.recv().await.unwrap();
            assert_eq!(req.command.as_deref(), Some("initialize"));

            let caps = Capabilities {
                supports_configuration_done_request: Some(true),
                ..Default::default()
            };
            let resp = ProtocolMessage::response_ok(
                100,
                req.seq,
                "initialize",
                Some(serde_json::to_value(&caps).unwrap()),
            );
            handle.response_tx.send(Ok(resp)).await.unwrap();
        });

        let caps: Capabilities = session
            .custom_request("initialize", Option::<()>::None, None)
            .await
            .unwrap();

        assert_eq!(caps.supports_configuration_done_request, Some(true));
    }

    // ─── Test 2: cancellation cancels pending request ────────────────────────

    #[tokio::test]
    async fn cancellation_cancels_pending_request() {
        let (handle, adapter) = make_mock();
        let cancel = CancellationToken::new();
        let session = RawDebugSession::create(adapter, Arc::new(NoopTracker), cancel.clone());
        let session = Arc::new(session);

        let req_cancel = CancellationToken::new();
        let req_cancel2 = req_cancel.clone();
        let session2 = Arc::clone(&session);

        let fut = tokio::spawn(async move {
            session2
                .custom_request::<(), serde_json::Value>("launch", None, Some(req_cancel2))
                .await
        });

        // Cancel the per-request token after a short delay
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        req_cancel.cancel();

        let result = fut.await.unwrap();
        assert!(result.is_err(), "expected error, got ok");
        let err_str = result.unwrap_err().to_string();
        assert!(
            err_str.contains("cancel"),
            "expected 'cancel' in error, got: {err_str}"
        );

        drop(handle);
    }

    // ─── Test 3: tracker hooks invoked once per direction ────────────────────

    #[tokio::test]
    async fn tracker_hooks_invoked_per_direction() {
        let spy = Arc::new(SpyTracker::default());
        let sent_ref = Arc::clone(&spy.sent);
        let recv_ref = Arc::clone(&spy.received);

        let (mut handle, adapter) = make_mock();
        let cancel = CancellationToken::new();
        let session = RawDebugSession::create(adapter, spy, cancel.clone());

        tokio::spawn(async move {
            let req = handle.request_rx.recv().await.unwrap();
            let caps = Capabilities::default();
            let resp = ProtocolMessage::response_ok(
                100,
                req.seq,
                "initialize",
                Some(serde_json::to_value(&caps).unwrap()),
            );
            handle.response_tx.send(Ok(resp)).await.unwrap();
        });

        let _: Capabilities = session
            .custom_request("initialize", Option::<()>::None, None)
            .await
            .unwrap();

        // Allow reader task to deliver the message
        tokio::time::sleep(std::time::Duration::from_millis(30)).await;

        let sent = sent_ref.lock().unwrap().clone();
        let received = recv_ref.lock().unwrap().clone();

        assert_eq!(sent.len(), 1, "expected 1 outbound, got: {sent:?}");
        assert_eq!(sent[0], "initialize");
        assert_eq!(received.len(), 1, "expected 1 inbound, got: {received:?}");
        assert_eq!(received[0], "initialize");
    }
}
