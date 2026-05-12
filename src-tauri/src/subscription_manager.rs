use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex,
    },
};

use serde_json::Value;
use tokio::sync::mpsc;

use crate::contracts::{ChannelEventMessage, SubscriptionId};
use crate::observability::{
    metric_event, record_metric, TraceId, CHANNEL_EVENT_DELIVERED_METRIC,
    CHANNEL_EVENT_DROPPED_METRIC,
};

pub const CHANNEL_EVENT_NAME: &str = "channel_event";
pub const DEFAULT_SUBSCRIPTION_BUFFER_CAPACITY: usize = 64;

pub type SubscriptionPayload = Option<Value>;
pub type SubscriptionPayloadSender = mpsc::Sender<SubscriptionPayload>;
pub type SubscriptionPayloadReceiver = mpsc::Receiver<SubscriptionPayload>;
pub type EventSink = SubscriptionPayloadSender;
pub type SubscriptionHandle = SubscriptionCancellationHandle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriptionError {
    DuplicateSubscription(SubscriptionId),
    EmitFailed(String),
}

impl SubscriptionError {
    pub fn code(&self) -> &'static str {
        match self {
            SubscriptionError::DuplicateSubscription(_) => "subscription.duplicateId",
            SubscriptionError::EmitFailed(_) => "subscription.emitFailed",
        }
    }

    pub fn message(&self) -> String {
        match self {
            SubscriptionError::DuplicateSubscription(subscription_id) => {
                format!("subscription already registered: {}", subscription_id.0)
            }
            SubscriptionError::EmitFailed(error) => {
                format!("failed to emit subscription event: {error}")
            }
        }
    }
}

#[derive(Clone)]
pub struct SubscriptionManager {
    inner: Arc<Mutex<SubscriptionState>>,
    next_generated_id: Arc<AtomicU64>,
    buffer_capacity: usize,
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new(DEFAULT_SUBSCRIPTION_BUFFER_CAPACITY)
    }
}

impl SubscriptionManager {
    pub fn new(buffer_capacity: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(SubscriptionState::default())),
            next_generated_id: Arc::new(AtomicU64::new(1)),
            buffer_capacity: buffer_capacity.max(1),
        }
    }

    pub fn bounded_channel(&self) -> (SubscriptionPayloadSender, SubscriptionPayloadReceiver) {
        mpsc::channel(self.buffer_capacity)
    }

    pub fn subscribe(
        &self,
        request: SubscriptionRequest,
        events: SubscriptionPayloadReceiver,
        cancellation: SubscriptionCancellationHandle,
        emitter: Arc<dyn ChannelEventEmitter>,
    ) -> Result<SubscriptionId, SubscriptionError> {
        let subscription_id = self.resolve_subscription_id(request.subscription_id);

        let disposed = Arc::new(AtomicBool::new(false));
        let sequence = {
            let mut state = self.inner.lock().expect("subscription state lock poisoned");
            let key = SubscriptionKey::new(request.window_label.clone(), subscription_id.clone());
            if state.records.contains_key(&key) {
                cancellation.cancel();
                return Err(SubscriptionError::DuplicateSubscription(subscription_id));
            }

            state.next_sequence += 1;
            let sequence = state.next_sequence;
            state.records.insert(
                key.clone(),
                SubscriptionRecord {
                    subscription_id: subscription_id.clone(),
                    channel: request.channel.clone(),
                    event: request.event.clone(),
                    window_label: request.window_label.clone(),
                    cancellation,
                    disposed: disposed.clone(),
                    sequence,
                },
            );
            sequence
        };

        let task = FanOutTask {
            inner: self.inner.clone(),
            key: SubscriptionKey::new(request.window_label.clone(), subscription_id.clone()),
            subscription_id: subscription_id.clone(),
            channel: request.channel,
            event: request.event,
            window_label: request.window_label,
            disposed,
            sequence,
            events,
            emitter,
        };
        tokio::spawn(task.run());

        Ok(subscription_id)
    }

    fn resolve_subscription_id(&self, subscription_id: Option<SubscriptionId>) -> SubscriptionId {
        subscription_id.unwrap_or_else(|| {
            SubscriptionId(format!(
                "rust-subscription-{}",
                self.next_generated_id.fetch_add(1, Ordering::Relaxed)
            ))
        })
    }

    pub fn dispose(&self, subscription_id: &SubscriptionId) -> bool {
        let record = {
            let mut state = self.inner.lock().expect("subscription state lock poisoned");
            let key = state
                .records
                .keys()
                .find(|key| key.subscription_id == *subscription_id)
                .cloned();
            key.and_then(|key| state.records.remove(&key))
        };

        dispose_record(record)
    }

    pub fn dispose_for_window(&self, window_label: &str, subscription_id: &SubscriptionId) -> bool {
        let record = self
            .inner
            .lock()
            .expect("subscription state lock poisoned")
            .records
            .remove(&SubscriptionKey::new(
                window_label.to_string(),
                subscription_id.clone(),
            ));

        dispose_record(record)
    }

    pub fn contains_for_window(
        &self,
        window_label: &str,
        subscription_id: &SubscriptionId,
    ) -> bool {
        self.inner
            .lock()
            .expect("subscription state lock poisoned")
            .records
            .contains_key(&SubscriptionKey::new(
                window_label.to_string(),
                subscription_id.clone(),
            ))
    }

    pub fn contains(&self, subscription_id: &SubscriptionId) -> bool {
        self.inner
            .lock()
            .expect("subscription state lock poisoned")
            .records
            .keys()
            .any(|key| key.subscription_id == *subscription_id)
    }

    pub fn subscription(&self, subscription_id: &SubscriptionId) -> Option<SubscriptionInfo> {
        self.inner
            .lock()
            .expect("subscription state lock poisoned")
            .records
            .values()
            .find(|record| record.subscription_id == *subscription_id)
            .map(|record| SubscriptionInfo {
                subscription_id: record.subscription_id.clone(),
                channel: record.channel.clone(),
                event: record.event.clone(),
                window_label: record.window_label.clone(),
                disposed: record.disposed.load(Ordering::SeqCst),
            })
    }

    pub fn len(&self) -> usize {
        self.inner
            .lock()
            .expect("subscription state lock poisoned")
            .records
            .len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn dispose_record(record: Option<SubscriptionRecord>) -> bool {
    if let Some(record) = record {
        record.disposed.store(true, Ordering::SeqCst);
        record.cancellation.cancel();
        true
    } else {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionRequest {
    pub subscription_id: Option<SubscriptionId>,
    pub channel: String,
    pub event: String,
    pub window_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionInfo {
    pub subscription_id: SubscriptionId,
    pub channel: String,
    pub event: String,
    pub window_label: String,
    pub disposed: bool,
}

pub trait ChannelEventEmitter: Send + Sync {
    fn emit_channel_event(
        &self,
        window_label: &str,
        message: ChannelEventMessage,
    ) -> Result<(), SubscriptionError>;
}

#[cfg(feature = "runtime")]
#[derive(Clone)]
pub struct TauriChannelEventEmitter {
    app_handle: tauri::AppHandle,
}

#[cfg(feature = "runtime")]
impl TauriChannelEventEmitter {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self { app_handle }
    }
}

#[cfg(feature = "runtime")]
impl ChannelEventEmitter for TauriChannelEventEmitter {
    fn emit_channel_event(
        &self,
        window_label: &str,
        message: ChannelEventMessage,
    ) -> Result<(), SubscriptionError> {
        tauri::Emitter::emit_to(&self.app_handle, window_label, CHANNEL_EVENT_NAME, message)
            .map_err(|error| SubscriptionError::EmitFailed(error.to_string()))
    }
}

pub struct SubscriptionCancellationHandle {
    on_cancel: Mutex<Option<Box<dyn FnOnce() + Send + 'static>>>,
}

impl fmt::Debug for SubscriptionCancellationHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SubscriptionCancellationHandle")
            .finish_non_exhaustive()
    }
}

impl Default for SubscriptionCancellationHandle {
    fn default() -> Self {
        Self::noop()
    }
}

impl SubscriptionCancellationHandle {
    pub fn noop() -> Self {
        Self {
            on_cancel: Mutex::new(None),
        }
    }

    pub fn new(on_cancel: impl FnOnce() + Send + 'static) -> Self {
        Self {
            on_cancel: Mutex::new(Some(Box::new(on_cancel))),
        }
    }

    pub fn cancel(self) {
        if let Some(on_cancel) = self
            .on_cancel
            .lock()
            .expect("subscription cancellation lock poisoned")
            .take()
        {
            on_cancel();
        }
    }
}

#[derive(Default)]
struct SubscriptionState {
    records: HashMap<SubscriptionKey, SubscriptionRecord>,
    next_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SubscriptionKey {
    window_label: String,
    subscription_id: SubscriptionId,
}

impl SubscriptionKey {
    fn new(window_label: String, subscription_id: SubscriptionId) -> Self {
        Self {
            window_label,
            subscription_id,
        }
    }
}

struct SubscriptionRecord {
    subscription_id: SubscriptionId,
    channel: String,
    event: String,
    window_label: String,
    cancellation: SubscriptionCancellationHandle,
    disposed: Arc<AtomicBool>,
    sequence: u64,
}

struct FanOutTask {
    inner: Arc<Mutex<SubscriptionState>>,
    key: SubscriptionKey,
    subscription_id: SubscriptionId,
    channel: String,
    event: String,
    window_label: String,
    disposed: Arc<AtomicBool>,
    sequence: u64,
    events: SubscriptionPayloadReceiver,
    emitter: Arc<dyn ChannelEventEmitter>,
}

impl FanOutTask {
    async fn run(mut self) {
        while let Some(payload) = self.events.recv().await {
            if self.disposed.load(Ordering::SeqCst) {
                self.record_event_metric(CHANNEL_EVENT_DROPPED_METRIC, None);
                continue;
            }

            let message = ChannelEventMessage {
                subscription_id: self.subscription_id.clone(),
                channel: self.channel.clone(),
                event: self.event.clone(),
                payload,
            };

            if let Err(error) = self.emitter.emit_channel_event(&self.window_label, message) {
                tracing::warn!(?error, "failed to emit channel subscription event");
                self.record_event_metric(CHANNEL_EVENT_DROPPED_METRIC, Some(error.code()));
            } else {
                self.record_event_metric(CHANNEL_EVENT_DELIVERED_METRIC, None);
            }
        }

        let mut state = self.inner.lock().expect("subscription state lock poisoned");
        if state
            .records
            .get(&self.key)
            .is_some_and(|record| record.sequence == self.sequence)
        {
            state.records.remove(&self.key);
        }
    }

    fn record_event_metric(&self, name: &str, error_code: Option<&str>) {
        record_metric(&metric_event(
            TraceId::for_command("channel_event", self.subscription_id.0.as_str()),
            name,
            1.0,
            crate::observability::MetricUnit::Count,
            Some(serde_json::json!({
                "channel": self.channel.as_str(),
                "event": self.event.as_str(),
                "subscriptionId": self.subscription_id.0.as_str(),
                "windowLabel": self.window_label.as_str(),
                "errorCode": error_code,
            })),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    };

    #[derive(Default)]
    struct RecordingEmitter {
        messages: Mutex<Vec<(String, ChannelEventMessage)>>,
    }

    impl RecordingEmitter {
        fn messages(&self) -> Vec<(String, ChannelEventMessage)> {
            self.messages
                .lock()
                .expect("messages lock poisoned")
                .clone()
        }
    }

    impl ChannelEventEmitter for RecordingEmitter {
        fn emit_channel_event(
            &self,
            window_label: &str,
            message: ChannelEventMessage,
        ) -> Result<(), SubscriptionError> {
            self.messages
                .lock()
                .expect("messages lock poisoned")
                .push((window_label.to_string(), message));
            Ok(())
        }
    }

    fn subscription_request(
        subscription_id: &str,
        channel: &str,
        event: &str,
        window_label: &str,
    ) -> SubscriptionRequest {
        SubscriptionRequest {
            subscription_id: Some(SubscriptionId(subscription_id.to_string())),
            channel: channel.to_string(),
            event: event.to_string(),
            window_label: window_label.to_string(),
        }
    }

    #[tokio::test]
    async fn fan_out_emits_channel_event_messages() {
        let manager = SubscriptionManager::new(2);
        let (sender, receiver) = manager.bounded_channel();
        let emitter = Arc::new(RecordingEmitter::default());

        let subscription_id = manager
            .subscribe(
                subscription_request("sub-1", "workbench", "changed", "main"),
                receiver,
                SubscriptionCancellationHandle::noop(),
                emitter.clone(),
            )
            .expect("subscription registered");

        sender
            .send(Some(json!({ "scope": "test" })))
            .await
            .expect("send event");
        drop(sender);

        tokio::task::yield_now().await;
        tokio::task::yield_now().await;

        assert_eq!(subscription_id, SubscriptionId("sub-1".to_string()));
        assert_eq!(
            emitter.messages(),
            vec![(
                "main".to_string(),
                ChannelEventMessage {
                    subscription_id: SubscriptionId("sub-1".to_string()),
                    channel: "workbench".to_string(),
                    event: "changed".to_string(),
                    payload: Some(json!({ "scope": "test" })),
                }
            )]
        );
        assert!(manager.is_empty());
    }

    #[tokio::test]
    async fn dispose_is_idempotent_and_ignores_late_events() {
        let manager = SubscriptionManager::default();
        let (sender, receiver) = manager.bounded_channel();
        let emitter = Arc::new(RecordingEmitter::default());
        let cancel_count = Arc::new(AtomicUsize::new(0));
        let cancel_count_for_handle = cancel_count.clone();

        manager
            .subscribe(
                subscription_request("sub-1", "workbench", "changed", "main"),
                receiver,
                SubscriptionCancellationHandle::new(move || {
                    cancel_count_for_handle.fetch_add(1, Ordering::SeqCst);
                }),
                emitter.clone(),
            )
            .expect("subscription registered");

        assert!(manager.dispose(&SubscriptionId("sub-1".to_string())));
        assert!(!manager.contains(&SubscriptionId("sub-1".to_string())));
        assert!(manager
            .subscription(&SubscriptionId("sub-1".to_string()))
            .is_none());
        assert!(!manager.dispose(&SubscriptionId("sub-1".to_string())));

        sender
            .send(Some(json!({ "late": true })))
            .await
            .expect("send late event");
        drop(sender);
        tokio::task::yield_now().await;

        assert_eq!(cancel_count.load(Ordering::SeqCst), 1);
        assert!(emitter.messages().is_empty());
    }

    #[tokio::test]
    async fn duplicate_subscription_id_is_rejected() {
        let manager = SubscriptionManager::default();
        let (_sender, receiver) = manager.bounded_channel();
        let emitter = Arc::new(RecordingEmitter::default());
        let duplicate_cancel_count = Arc::new(AtomicUsize::new(0));
        let duplicate_cancel_count_for_handle = duplicate_cancel_count.clone();

        manager
            .subscribe(
                subscription_request("sub-1", "workbench", "changed", "main"),
                receiver,
                SubscriptionCancellationHandle::noop(),
                emitter.clone(),
            )
            .expect("subscription registered");

        let (_sender, receiver) = manager.bounded_channel();
        let error = manager
            .subscribe(
                subscription_request("sub-1", "workbench", "changed", "main"),
                receiver,
                SubscriptionCancellationHandle::new(move || {
                    duplicate_cancel_count_for_handle.fetch_add(1, Ordering::SeqCst);
                }),
                emitter,
            )
            .expect_err("duplicate rejected");

        assert_eq!(
            error,
            SubscriptionError::DuplicateSubscription(SubscriptionId("sub-1".to_string()))
        );
        assert_eq!(duplicate_cancel_count.load(Ordering::SeqCst), 1);
        assert_eq!(manager.len(), 1);
        assert_eq!(
            manager
                .subscription(&SubscriptionId("sub-1".to_string()))
                .expect("original subscription retained")
                .window_label,
            "main"
        );
    }

    #[tokio::test]
    async fn events_emit_only_to_owning_window_label() {
        let manager = SubscriptionManager::new(2);
        let (main_sender, main_receiver) = manager.bounded_channel();
        let (secondary_sender, secondary_receiver) = manager.bounded_channel();
        let emitter = Arc::new(RecordingEmitter::default());

        manager
            .subscribe(
                subscription_request("sub-main", "workbench", "changed", "main"),
                main_receiver,
                SubscriptionCancellationHandle::noop(),
                emitter.clone(),
            )
            .expect("main subscription registered");
        manager
            .subscribe(
                subscription_request("sub-secondary", "workbench", "changed", "secondary"),
                secondary_receiver,
                SubscriptionCancellationHandle::noop(),
                emitter.clone(),
            )
            .expect("secondary subscription registered");

        main_sender
            .send(Some(json!({ "window": "main" })))
            .await
            .expect("send main event");
        secondary_sender
            .send(Some(json!({ "window": "secondary" })))
            .await
            .expect("send secondary event");
        drop(main_sender);
        drop(secondary_sender);

        tokio::task::yield_now().await;
        tokio::task::yield_now().await;

        let messages = emitter.messages();
        assert_eq!(messages.len(), 2);
        assert!(messages.contains(&(
            "main".to_string(),
            ChannelEventMessage {
                subscription_id: SubscriptionId("sub-main".to_string()),
                channel: "workbench".to_string(),
                event: "changed".to_string(),
                payload: Some(json!({ "window": "main" })),
            },
        )));
        assert!(messages.contains(&(
            "secondary".to_string(),
            ChannelEventMessage {
                subscription_id: SubscriptionId("sub-secondary".to_string()),
                channel: "workbench".to_string(),
                event: "changed".to_string(),
                payload: Some(json!({ "window": "secondary" })),
            },
        )));
    }

    #[tokio::test]
    async fn same_subscription_id_is_scoped_per_window() {
        let manager = SubscriptionManager::new(2);
        let (main_sender, main_receiver) = manager.bounded_channel();
        let (secondary_sender, secondary_receiver) = manager.bounded_channel();
        let emitter = Arc::new(RecordingEmitter::default());
        let main_cancel_count = Arc::new(AtomicUsize::new(0));
        let secondary_cancel_count = Arc::new(AtomicUsize::new(0));
        let main_cancel_count_for_handle = main_cancel_count.clone();
        let secondary_cancel_count_for_handle = secondary_cancel_count.clone();

        manager
            .subscribe(
                subscription_request("shared-sub", "workbench", "changed", "main"),
                main_receiver,
                SubscriptionCancellationHandle::new(move || {
                    main_cancel_count_for_handle.fetch_add(1, Ordering::SeqCst);
                }),
                emitter.clone(),
            )
            .expect("main subscription registered");
        manager
            .subscribe(
                subscription_request("shared-sub", "workbench", "changed", "secondary"),
                secondary_receiver,
                SubscriptionCancellationHandle::new(move || {
                    secondary_cancel_count_for_handle.fetch_add(1, Ordering::SeqCst);
                }),
                emitter.clone(),
            )
            .expect("secondary subscription registered");

        assert!(manager.contains_for_window("main", &SubscriptionId("shared-sub".to_string())));
        assert!(manager.contains_for_window("secondary", &SubscriptionId("shared-sub".to_string())));

        assert!(manager.dispose_for_window("main", &SubscriptionId("shared-sub".to_string())));
        assert!(!manager.contains_for_window("main", &SubscriptionId("shared-sub".to_string())));
        assert!(manager.contains_for_window("secondary", &SubscriptionId("shared-sub".to_string())));
        assert_eq!(main_cancel_count.load(Ordering::SeqCst), 1);
        assert_eq!(secondary_cancel_count.load(Ordering::SeqCst), 0);

        main_sender
            .send(Some(json!({ "window": "main" })))
            .await
            .expect("send main event");
        secondary_sender
            .send(Some(json!({ "window": "secondary" })))
            .await
            .expect("send secondary event");
        drop(main_sender);
        drop(secondary_sender);

        tokio::task::yield_now().await;
        tokio::task::yield_now().await;

        assert_eq!(
            emitter.messages(),
            vec![(
                "secondary".to_string(),
                ChannelEventMessage {
                    subscription_id: SubscriptionId("shared-sub".to_string()),
                    channel: "workbench".to_string(),
                    event: "changed".to_string(),
                    payload: Some(json!({ "window": "secondary" })),
                }
            )]
        );
    }
}
