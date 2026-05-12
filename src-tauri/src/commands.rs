use crate::{
    cancellation_manager::CancellationManager,
    contracts::{
        CancelRequest, ChannelCallRequest, ChannelError, ChannelListenRequest, ChannelResponse,
    },
    service_registry::ServiceRegistry,
    subscription_manager::{ChannelEventEmitter, SubscriptionManager, SubscriptionRequest},
};

use std::{sync::Arc, time::Instant};

#[cfg(feature = "runtime")]
use crate::contracts::{
    ChannelDisposeRequest, FileDeleteRequest, FileDirEntryDto, FileMkdirRequest, FileReadRequest,
    FileReadResponse, FileReaddirRequest, FileStatDto, FileStatRequest, FileWriteRequest,
};
#[cfg(feature = "runtime")]
use crate::file_service::{FileWatchRequest, FileWatchResponse};
use crate::observability::{
    command_trace_id, metric_event, record_metric, MetricUnit, BRIDGE_ROUND_TRIP_MS_METRIC,
    CHANNEL_LISTEN_ACTIVE_METRIC,
};
#[cfg(feature = "runtime")]
use crate::observability::{CHANNEL_CALL_INVOCATIONS_METRIC, FILE_WATCH_ACTIVE_METRIC};
#[cfg(feature = "runtime")]
use crate::service_registry::ServiceId;
#[cfg(feature = "runtime")]
use crate::subscription_manager::TauriChannelEventEmitter;
#[cfg(feature = "runtime")]
use serde::Serialize;

#[cfg(feature = "runtime")]
#[tauri::command]
pub async fn channel_call(
    registry: tauri::State<'_, ServiceRegistry>,
    cancellations: tauri::State<'_, CancellationManager>,
    request: ChannelCallRequest,
) -> ChannelResponse {
    channel_call_impl(&registry, &cancellations, request).await
}

#[cfg_attr(not(feature = "runtime"), allow(dead_code))]
async fn channel_call_impl(
    registry: &ServiceRegistry,
    cancellations: &CancellationManager,
    request: ChannelCallRequest,
) -> ChannelResponse {
    let request_id = request.request_id.clone();
    let started = Instant::now();
    let trace_id = command_trace_id("channel_call", &request_id);
    let cancellation = cancellations.begin(request_id.clone(), request.cancellation_id.clone());
    #[cfg(feature = "runtime")]
    {
        let channel = request.channel.clone();
        let command = request.command.clone();
        let service_id = ServiceId::try_from(channel.as_str())
            .map(|service_id| service_id.as_str())
            .unwrap_or("unknownService");
        tracing::info!(
            trace_id = trace_id.as_str(),
            channel = channel.as_str(),
            command = command.as_str(),
            request_id = request_id.as_str(),
            service_id,
            "tauri command received"
        );

        record_metric(&metric_event(
            trace_id.clone(),
            CHANNEL_CALL_INVOCATIONS_METRIC,
            1.0,
            MetricUnit::Count,
            Some(serde_json::json!({
                "channel": channel,
                "command": command,
            })),
        ));
    }

    let response = if cancellation.is_cancelled() {
        canceled_response(request_id)
    } else {
        match registry.dispatch_channel_call(request).await {
            Ok(_) if cancellation.is_cancelled() => canceled_response(request_id),
            Ok(result) => success_response(request_id, result),
            Err(_) if cancellation.is_cancelled() => canceled_response(request_id),
            Err(error) => error_response(request_id, error.into()),
        }
    };

    record_metric(&metric_event(
        trace_id,
        BRIDGE_ROUND_TRIP_MS_METRIC,
        started.elapsed().as_secs_f64() * 1000.0,
        MetricUnit::Milliseconds,
        Some(serde_json::json!({ "requestId": response.request_id })),
    ));

    response
}

#[cfg(feature = "runtime")]
#[tauri::command]
pub async fn channel_listen(
    registry: tauri::State<'_, ServiceRegistry>,
    subscriptions: tauri::State<'_, SubscriptionManager>,
    app_handle: tauri::AppHandle,
    window: tauri::Window,
    request: ChannelListenRequest,
) -> ChannelResponse {
    channel_listen_impl(
        &registry,
        &subscriptions,
        Arc::new(TauriChannelEventEmitter::new(app_handle)),
        window.label().to_string(),
        request,
    )
    .await
}

#[cfg(feature = "runtime")]
#[tauri::command]
pub async fn channel_dispose(
    subscriptions: tauri::State<'_, SubscriptionManager>,
    window: tauri::Window,
    request: ChannelDisposeRequest,
) {
    let window_label = window.label().to_string();
    let disposed = subscriptions.dispose_for_window(&window_label, &request.subscription_id);
    record_metric(&metric_event(
        command_trace_id("channel_dispose", request.subscription_id.0.as_str()),
        CHANNEL_LISTEN_ACTIVE_METRIC,
        subscriptions.len() as f64,
        MetricUnit::Count,
        Some(serde_json::json!({ "disposed": disposed, "windowLabel": window_label.as_str() })),
    ));
    tracing::debug!(
        subscription_id = request.subscription_id.0.as_str(),
        window_label = window_label.as_str(),
        disposed,
        "channel subscription dispose requested"
    );
}

#[cfg(feature = "runtime")]
#[tauri::command]
pub async fn cancel_request(
    cancellations: tauri::State<'_, CancellationManager>,
    request: CancelRequest,
) {
    cancel_request_impl(&cancellations, request).await;
}

#[cfg_attr(not(feature = "runtime"), allow(dead_code))]
async fn cancel_request_impl(cancellations: &CancellationManager, request: CancelRequest) {
    let cancelled = cancellations.cancel(&request);
    tracing::debug!(
        cancellation_id = request.cancellation_id.0.as_str(),
        request_id = request.request_id.as_deref(),
        cancelled,
        "channel request cancellation requested"
    );
}

fn canceled_response(request_id: String) -> ChannelResponse {
    error_response(
        request_id,
        ChannelError {
            code: "Canceled".to_string(),
            message: "Canceled".to_string(),
            details: None,
        },
    )
}

fn success_response(request_id: String, result: serde_json::Value) -> ChannelResponse {
    ChannelResponse {
        request_id,
        result: Some(result),
        error: None,
    }
}

fn error_response(request_id: String, error: ChannelError) -> ChannelResponse {
    ChannelResponse {
        request_id,
        result: None,
        error: Some(error),
    }
}

#[cfg(feature = "runtime")]
#[tauri::command]
pub async fn fs_stat(
    registry: tauri::State<'_, ServiceRegistry>,
    request: FileStatRequest,
) -> Result<FileStatDto, ChannelError> {
    dispatch_file_command(&registry, "fs_stat", request).await
}

#[cfg(feature = "runtime")]
#[tauri::command]
pub async fn fs_read_file(
    registry: tauri::State<'_, ServiceRegistry>,
    request: FileReadRequest,
) -> Result<FileReadResponse, ChannelError> {
    dispatch_file_command(&registry, "fs_read_file", request).await
}

#[cfg(feature = "runtime")]
#[tauri::command]
pub async fn fs_write_file(
    registry: tauri::State<'_, ServiceRegistry>,
    request: FileWriteRequest,
) -> Result<FileStatDto, ChannelError> {
    dispatch_file_command(&registry, "fs_write_file", request).await
}

#[cfg(feature = "runtime")]
#[tauri::command]
pub async fn fs_delete(
    registry: tauri::State<'_, ServiceRegistry>,
    request: FileDeleteRequest,
) -> Result<(), ChannelError> {
    dispatch_file_command(&registry, "fs_delete", request).await
}

#[cfg(feature = "runtime")]
#[tauri::command]
pub async fn fs_mkdir(
    registry: tauri::State<'_, ServiceRegistry>,
    request: FileMkdirRequest,
) -> Result<FileStatDto, ChannelError> {
    dispatch_file_command(&registry, "fs_mkdir", request).await
}

#[cfg(feature = "runtime")]
#[tauri::command]
pub async fn fs_readdir(
    registry: tauri::State<'_, ServiceRegistry>,
    request: FileReaddirRequest,
) -> Result<Vec<FileDirEntryDto>, ChannelError> {
    dispatch_file_command(&registry, "fs_readdir", request).await
}

#[cfg(feature = "runtime")]
#[tauri::command]
pub async fn fs_watch(
    registry: tauri::State<'_, ServiceRegistry>,
    request: FileWatchRequest,
) -> Result<FileWatchResponse, ChannelError> {
    let response: FileWatchResponse = dispatch_file_command(&registry, "fs_watch", request).await?;
    record_metric(&metric_event(
        command_trace_id("fs_watch", response.watch_id.0.as_str()),
        FILE_WATCH_ACTIVE_METRIC,
        1.0,
        MetricUnit::Count,
        Some(serde_json::json!({ "watchId": response.watch_id.0.as_str() })),
    ));
    Ok(response)
}

#[cfg(feature = "runtime")]
async fn dispatch_file_command<T>(
    registry: &ServiceRegistry,
    command: &str,
    request: impl Serialize,
) -> Result<T, ChannelError>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let arg = serde_json::to_value(request).map_err(|error| ChannelError {
        code: "fs.invalidArgument".to_string(),
        message: format!("failed to serialize file command argument: {error}"),
        details: None,
    })?;

    let value = registry
        .dispatch_channel_call(ChannelCallRequest {
            request_id: "tauri-command".to_string(),
            channel: "files".to_string(),
            command: command.to_string(),
            args: vec![arg],
            cancellation_id: None,
        })
        .await
        .map_err(ChannelError::from)?;

    serde_json::from_value(value).map_err(|error| ChannelError {
        code: "fs.invalidResponse".to_string(),
        message: format!("failed to deserialize file command response: {error}"),
        details: None,
    })
}

#[cfg_attr(not(feature = "runtime"), allow(dead_code))]
async fn channel_listen_impl(
    registry: &ServiceRegistry,
    subscriptions: &SubscriptionManager,
    emitter: Arc<dyn ChannelEventEmitter>,
    window_label: String,
    request: ChannelListenRequest,
) -> ChannelResponse {
    let request_id = request.request_id.clone();
    let Some(subscription_id) = request.subscription_id.clone() else {
        return error_response(
            request_id,
            ChannelError {
                code: "subscription.missingId".to_string(),
                message: "channel_listen requires subscriptionId".to_string(),
                details: None,
            },
        );
    };

    let (sink, events) = subscriptions.bounded_channel();
    let handle = match registry
        .dispatch_channel_listen(request.clone(), sink)
        .await
    {
        Ok(handle) => handle,
        Err(error) => {
            return error_response(request_id, error.into());
        }
    };

    match subscriptions.subscribe(
        SubscriptionRequest {
            subscription_id: Some(subscription_id),
            channel: request.channel.clone(),
            event: request.event.clone(),
            window_label,
        },
        events,
        handle,
        emitter,
    ) {
        Ok(_) => {
            record_metric(&metric_event(
                command_trace_id("channel_listen", &request_id),
                CHANNEL_LISTEN_ACTIVE_METRIC,
                subscriptions.len() as f64,
                MetricUnit::Count,
                Some(serde_json::json!({
                    "channel": request.channel,
                    "event": request.event,
                })),
            ));
            success_response(request_id, serde_json::Value::Null)
        }
        Err(error) => error_response(
            request_id,
            ChannelError {
                code: error.code().to_string(),
                message: error.message(),
                details: None,
            },
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{CancellationId, ChannelCallRequest, SubscriptionId};
    use crate::service_registry::{FileService, PlatformService, ServiceError, ServiceId};
    use async_trait::async_trait;
    use serde_json::Value;
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    };
    use tokio::sync::oneshot;

    struct RecordingEmitter;

    impl ChannelEventEmitter for RecordingEmitter {
        fn emit_channel_event(
            &self,
            _window_label: &str,
            _message: crate::contracts::ChannelEventMessage,
        ) -> Result<(), crate::subscription_manager::SubscriptionError> {
            Ok(())
        }
    }

    struct ListenFileService {
        cancelled: Arc<AtomicBool>,
    }

    #[async_trait]
    impl PlatformService for ListenFileService {
        fn service_id(&self) -> ServiceId {
            ServiceId::File
        }

        async fn listen(
            &self,
            _request: ChannelListenRequest,
            _sink: crate::subscription_manager::EventSink,
        ) -> Result<crate::subscription_manager::SubscriptionHandle, ServiceError> {
            let cancelled = self.cancelled.clone();
            Ok(crate::subscription_manager::SubscriptionHandle::new(
                move || {
                    cancelled.store(true, Ordering::SeqCst);
                },
            ))
        }
    }

    impl FileService for ListenFileService {}

    struct PendingCallFileService {
        started: Mutex<Option<oneshot::Sender<()>>>,
        finish: tokio::sync::Mutex<Option<oneshot::Receiver<()>>>,
    }

    #[async_trait]
    impl PlatformService for PendingCallFileService {
        fn service_id(&self) -> ServiceId {
            ServiceId::File
        }

        async fn call(&self, _request: ChannelCallRequest) -> Result<Value, ServiceError> {
            if let Some(started) = self.started.lock().expect("started lock poisoned").take() {
                let _ = started.send(());
            }

            if let Some(finish) = self.finish.lock().await.take() {
                let _ = finish.await;
            }

            Ok(serde_json::json!({ "ok": true }))
        }
    }

    impl FileService for PendingCallFileService {}

    #[tokio::test]
    async fn channel_listen_requires_subscription_id() {
        let registry = ServiceRegistry::new();
        let subscriptions = SubscriptionManager::default();

        let response = channel_listen_impl(
            &registry,
            &subscriptions,
            Arc::new(RecordingEmitter),
            "main".to_string(),
            ChannelListenRequest {
                request_id: "req-1".to_string(),
                channel: "workbench".to_string(),
                event: "changed".to_string(),
                args: vec![serde_json::json!({ "scope": "test" })],
                subscription_id: None,
            },
        )
        .await;

        assert_eq!(response.request_id, "req-1");
        assert_eq!(response.result, None);
        assert_eq!(
            response.error,
            Some(ChannelError {
                code: "subscription.missingId".to_string(),
                message: "channel_listen requires subscriptionId".to_string(),
                details: None,
            })
        );
    }

    #[tokio::test]
    async fn channel_listen_registers_backend_before_returning_success() {
        let mut registry = ServiceRegistry::new();
        let cancelled = Arc::new(AtomicBool::new(false));
        registry.register_file_service(Arc::new(ListenFileService {
            cancelled: cancelled.clone(),
        }));
        let subscriptions = SubscriptionManager::default();

        let response = channel_listen_impl(
            &registry,
            &subscriptions,
            Arc::new(RecordingEmitter),
            "main".to_string(),
            ChannelListenRequest {
                request_id: "req-1".to_string(),
                channel: "files".to_string(),
                event: "watch".to_string(),
                args: vec![serde_json::json!({ "scope": "test" })],
                subscription_id: Some(SubscriptionId("sub-1".to_string())),
            },
        )
        .await;

        assert_eq!(response.request_id, "req-1");
        assert_eq!(response.result, Some(Value::Null));
        assert!(response.error.is_none());
        assert!(subscriptions.contains(&SubscriptionId("sub-1".to_string())));

        assert!(subscriptions.dispose(&SubscriptionId("sub-1".to_string())));
        assert!(!subscriptions.dispose(&SubscriptionId("sub-1".to_string())));
        assert!(cancelled.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn cancel_stub_completes_without_error() {
        let cancellations = CancellationManager::default();

        cancel_request_impl(
            &cancellations,
            CancelRequest {
                cancellation_id: CancellationId("cancel-1".to_string()),
                request_id: Some("req-1".to_string()),
            },
        )
        .await;
    }

    #[tokio::test]
    async fn channel_call_returns_canceled_error_when_cancel_request_arrives() {
        let mut registry = ServiceRegistry::new();
        let cancellations = CancellationManager::default();
        let (started_sender, started_receiver) = oneshot::channel();
        let (finish_sender, finish_receiver) = oneshot::channel();

        registry.register_file_service(Arc::new(PendingCallFileService {
            started: Mutex::new(Some(started_sender)),
            finish: tokio::sync::Mutex::new(Some(finish_receiver)),
        }));

        let response = channel_call_impl(
            &registry,
            &cancellations,
            ChannelCallRequest {
                request_id: "req-1".to_string(),
                channel: "files".to_string(),
                command: "slow".to_string(),
                args: Vec::new(),
                cancellation_id: Some(CancellationId("cancel-1".to_string())),
            },
        );
        tokio::pin!(response);

        tokio::select! {
            _ = &mut response => panic!("channel call completed before service was released"),
            _ = started_receiver => {}
        }

        cancel_request_impl(
            &cancellations,
            CancelRequest {
                cancellation_id: CancellationId("cancel-1".to_string()),
                request_id: Some("req-1".to_string()),
            },
        )
        .await;
        let _ = finish_sender.send(());

        let response = response.await;
        assert_eq!(response.request_id, "req-1");
        assert_eq!(response.result, None);
        assert_eq!(
            response.error,
            Some(ChannelError {
                code: "Canceled".to_string(),
                message: "Canceled".to_string(),
                details: None,
            })
        );
    }
}
