use std::{env, sync::Arc};

use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::sync::Mutex;

use crate::{
    contracts::ChannelCallRequest,
    observability::{metric_event, record_metric, MetricUnit, TraceId},
    service_registry::{
        DialogService, LifecycleService, NativeHostService, PlatformService, ServiceError,
        ServiceId, ServiceRegistry, TelemetryService,
    },
};

pub fn register_platform_parity_services(registry: &mut ServiceRegistry) {
    registry.register_native_host_service(Arc::new(RustNativeHostService));
    registry.register_dialog_service(Arc::new(RustDialogService));
    registry.register_lifecycle_service(Arc::new(RustLifecycleService));
    registry.register_telemetry_service(Arc::new(RustTelemetryService::default()));
}

#[derive(Debug, Default)]
pub struct RustNativeHostService;

#[async_trait]
impl PlatformService for RustNativeHostService {
    fn service_id(&self) -> ServiceId {
        ServiceId::NativeHost
    }

    async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
        match request.command.as_str() {
            "getOSProperties" | "osProperties" => Ok(json!({
                "platform": env::consts::OS,
                "arch": env::consts::ARCH,
                "family": env::consts::FAMILY,
                "pid": std::process::id(),
                "cwd": env::current_dir()
                    .ok()
                    .map(|path| path.to_string_lossy().into_owned()),
            })),
            "getWindowCount" | "windowCount" => Ok(json!(1)),
            command => Err(ServiceError::unsupported(ServiceId::NativeHost, command)),
        }
    }
}

impl NativeHostService for RustNativeHostService {}

#[derive(Debug, Default)]
pub struct RustDialogService;

#[async_trait]
impl PlatformService for RustDialogService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Dialog
    }

    async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
        match request.command.as_str() {
            "showMessageBox" | "showMessageBoxSync" => Ok(json!({
                "response": 0,
                "checkboxChecked": false,
            })),
            command => Err(ServiceError::unsupported(ServiceId::Dialog, command)),
        }
    }
}

impl DialogService for RustDialogService {}

#[derive(Debug, Default)]
pub struct RustLifecycleService;

#[async_trait]
impl PlatformService for RustLifecycleService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Lifecycle
    }

    async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
        match request.command.as_str() {
            "getLifecyclePhase" | "phase" | "when" => Ok(json!({
                "phase": "ready",
                "startupKind": "newWindow",
            })),
            command => Err(ServiceError::unsupported(ServiceId::Lifecycle, command)),
        }
    }
}

impl LifecycleService for RustLifecycleService {}

#[derive(Debug, Default)]
pub struct RustTelemetryService {
    events: Mutex<Vec<Value>>,
}

impl RustTelemetryService {
    pub async fn events(&self) -> Vec<Value> {
        self.events.lock().await.clone()
    }
}

#[async_trait]
impl PlatformService for RustTelemetryService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Telemetry
    }

    async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
        match request.command.as_str() {
            "publicLog" | "publicLog2" | "log" => {
                let event = request.args.first().cloned().unwrap_or(Value::Null);
                self.events.lock().await.push(event.clone());
                record_metric(&metric_event(
                    TraceId::new("telemetry-adapter"),
                    "vscode_tauri_telemetry_events_total",
                    1.0,
                    MetricUnit::Count,
                    Some(json!({ "event": event })),
                ));
                Ok(Value::Null)
            }
            "getTelemetryInfo" | "telemetryInfo" => Ok(json!({
                "sessionId": "tauri",
                "machineId": "tauri",
                "sqmId": "",
                "devDeviceId": "",
            })),
            "getTelemetryLevel" | "telemetryLevel" => Ok(json!("off")),
            command => Err(ServiceError::unsupported(ServiceId::Telemetry, command)),
        }
    }
}

impl TelemetryService for RustTelemetryService {}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(channel: &str, command: &str, arg: Value) -> ChannelCallRequest {
        ChannelCallRequest {
            request_id: format!("{channel}-{command}-req"),
            channel: channel.to_string(),
            command: command.to_string(),
            args: vec![arg],
            cancellation_id: None,
        }
    }

    #[tokio::test]
    async fn native_host_reports_os_properties_and_denies_unknown_commands() {
        let service = RustNativeHostService;

        let properties = service
            .call(request("nativeHost", "getOSProperties", Value::Null))
            .await
            .expect("os properties returned");
        assert_eq!(properties["platform"], env::consts::OS);

        let error = service
            .call(request("nativeHost", "openExternal", Value::Null))
            .await
            .expect_err("side effect command denied");
        assert_eq!(error.code, "service.unsupportedCommand");
    }

    #[tokio::test]
    async fn dialog_lifecycle_and_telemetry_cover_known_and_unsupported_commands() {
        let dialog = RustDialogService;
        assert_eq!(
            dialog
                .call(request(
                    "dialog",
                    "showMessageBox",
                    json!({ "message": "hi" })
                ))
                .await
                .expect("message box fallback succeeds")["response"],
            0
        );
        assert_eq!(
            dialog
                .call(request("dialog", "showOpenDialog", Value::Null))
                .await
                .expect_err("open dialog denied")
                .code,
            "service.unsupportedCommand"
        );

        let lifecycle = RustLifecycleService;
        assert_eq!(
            lifecycle
                .call(request("lifecycle", "getLifecyclePhase", Value::Null))
                .await
                .expect("phase returned")["phase"],
            "ready"
        );
        assert_eq!(
            lifecycle
                .call(request("lifecycle", "quit", Value::Null))
                .await
                .expect_err("quit denied")
                .code,
            "service.unsupportedCommand"
        );

        let telemetry = RustTelemetryService::default();
        telemetry
            .call(request(
                "telemetry",
                "publicLog",
                json!({ "eventName": "startup" }),
            ))
            .await
            .expect("telemetry log no-op succeeds");
        assert_eq!(
            telemetry.events().await,
            vec![json!({ "eventName": "startup" })]
        );
        assert_eq!(
            telemetry
                .call(request("telemetry", "flush", Value::Null))
                .await
                .expect_err("unknown telemetry command denied")
                .code,
            "service.unsupportedCommand"
        );
    }

    #[test]
    fn register_platform_parity_services_installs_all_adapters() {
        let mut registry = ServiceRegistry::new();
        register_platform_parity_services(&mut registry);

        assert!(registry.get(ServiceId::NativeHost).is_some());
        assert!(registry.get(ServiceId::Dialog).is_some());
        assert!(registry.get(ServiceId::Lifecycle).is_some());
        assert!(registry.get(ServiceId::Telemetry).is_some());
    }
}
