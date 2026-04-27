//! DebugAdapter trait + ExecutableDebugAdapter.
//!
//! Mirrors node/debugAdapter.ts: spawn a child process and wire its
//! stdin/stdout to DAP framing.

use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;
use std::process::Stdio;
use tokio::{
    io::BufReader,
    process::{Child, Command},
    sync::mpsc,
};
use tokio_util::sync::CancellationToken;
use tracing::instrument;

use super::{
    framing::{read_message, write_message},
    protocol::ProtocolMessage,
};

// ─────────────────────────────────────────────────────────────────────────────
// DebugAdapter trait
// ─────────────────────────────────────────────────────────────────────────────

/// Abstraction over the transport to a debug adapter.
///
/// `message_stream` takes `&mut self` and returns a static-lifetime stream so
/// it can be moved into a spawned task independently of the write-side.
/// Implementors must back the stream with an owned channel (Receiver).
#[async_trait]
pub trait DebugAdapter: Send + Sync {
    /// Send a request message.
    async fn send_request(&mut self, msg: ProtocolMessage) -> Result<()>;
    /// Send a response message.
    async fn send_response(&mut self, msg: ProtocolMessage) -> Result<()>;
    /// Send an event message.
    async fn send_event(&mut self, msg: ProtocolMessage) -> Result<()>;
    /// Consume the inbound stream from the adapter.
    ///
    /// May only be called once. Panics on second call.
    fn message_stream(&mut self) -> BoxStream<'static, Result<ProtocolMessage>>;
}

// ─────────────────────────────────────────────────────────────────────────────
// ExecutableDebugAdapter
// ─────────────────────────────────────────────────────────────────────────────

/// A debug adapter that communicates via a spawned child process.
pub struct ExecutableDebugAdapter {
    child: Child,
    stdin_tx: mpsc::Sender<ProtocolMessage>,
    message_rx: Option<mpsc::Receiver<Result<ProtocolMessage>>>,
}

impl ExecutableDebugAdapter {
    /// Spawn a debug adapter process and wire up framing.
    #[instrument(skip_all)]
    pub async fn spawn(
        program: impl AsRef<std::ffi::OsStr>,
        args: &[&str],
        cancel: CancellationToken,
    ) -> Result<Self> {
        let mut child = Command::new(program)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        let mut stdin = child.stdin.take().expect("stdin piped");
        let stdout = child.stdout.take().expect("stdout piped");

        // Channel for outbound messages (to adapter stdin)
        let (stdin_tx, mut stdin_rx) = mpsc::channel::<ProtocolMessage>(64);
        // Channel for inbound messages (from adapter stdout)
        let (msg_tx, msg_rx) = mpsc::channel::<Result<ProtocolMessage>>(64);

        // Writer task: stdin_rx -> framing -> stdin
        tokio::spawn(async move {
            while let Some(msg) = stdin_rx.recv().await {
                if write_message(&mut stdin, &msg).await.is_err() {
                    break;
                }
            }
        });

        // Reader task: stdout -> framing -> msg_tx
        let cancel2 = cancel.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            loop {
                tokio::select! {
                    _ = cancel2.cancelled() => break,
                    result = read_message(&mut reader) => {
                        let done = result.is_err();
                        if msg_tx.send(result).await.is_err() || done {
                            break;
                        }
                    }
                }
            }
        });

        Ok(Self {
            child,
            stdin_tx,
            message_rx: Some(msg_rx),
        })
    }

    /// Kill the child process.
    pub async fn kill(&mut self) -> Result<()> {
        self.child.kill().await?;
        Ok(())
    }
}

#[async_trait]
impl DebugAdapter for ExecutableDebugAdapter {
    async fn send_request(&mut self, msg: ProtocolMessage) -> Result<()> {
        self.stdin_tx.send(msg).await?;
        Ok(())
    }

    async fn send_response(&mut self, msg: ProtocolMessage) -> Result<()> {
        self.stdin_tx.send(msg).await?;
        Ok(())
    }

    async fn send_event(&mut self, msg: ProtocolMessage) -> Result<()> {
        self.stdin_tx.send(msg).await?;
        Ok(())
    }

    fn message_stream(&mut self) -> BoxStream<'static, Result<ProtocolMessage>> {
        let rx = self.message_rx.take().expect("message_stream called twice");
        Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    async fn executable_adapter_spawns_cat_echo() {
        // Use `cat` as a pass-through: sends messages and reads them back.
        // This only works on Unix where `cat` is available.
        if std::env::consts::OS != "linux" && std::env::consts::OS != "macos" {
            return;
        }

        let cancel = CancellationToken::new();
        let mut adapter =
            ExecutableDebugAdapter::spawn("cat", &[], cancel.clone()).await.unwrap();

        let msg = ProtocolMessage::request(1, "initialize", None);
        adapter.send_request(msg.clone()).await.unwrap();

        let mut stream = adapter.message_stream();
        let received = stream.next().await.unwrap().unwrap();
        assert_eq!(received.seq, 1);
        assert_eq!(received.command.as_deref(), Some("initialize"));

        cancel.cancel();
        adapter.kill().await.ok();
    }
}
