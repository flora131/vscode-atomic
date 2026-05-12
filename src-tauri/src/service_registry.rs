use std::{collections::HashMap, fmt, sync::Arc};

use async_trait::async_trait;
use serde_json::Value;

use crate::{
    contracts::{ChannelCallRequest, ChannelError, ChannelListenRequest},
    observability::{command_trace_id, ext_host_rpc_trace_id},
    subscription_manager::{EventSink, SubscriptionHandle},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServiceId {
    Auth,
    File,
    Watcher,
    Terminal,
    Configuration,
    Storage,
    NativeHost,
    Dialog,
    Telemetry,
    ExtensionHost,
    Tunnel,
    Lifecycle,
}

impl ServiceId {
    pub const AUTH: &'static str = "authService";
    pub const FILE: &'static str = "fileService";
    pub const WATCHER: &'static str = "watcherService";
    pub const TERMINAL: &'static str = "terminalService";
    pub const CONFIGURATION: &'static str = "configurationService";
    pub const STORAGE: &'static str = "storageService";
    pub const NATIVE_HOST: &'static str = "nativeHostService";
    pub const DIALOG: &'static str = "dialogService";
    pub const TELEMETRY: &'static str = "telemetryService";
    pub const EXTENSION_HOST: &'static str = "extensionHostService";
    pub const TUNNEL: &'static str = "tunnelService";
    pub const LIFECYCLE: &'static str = "lifecycleService";

    pub fn as_str(self) -> &'static str {
        match self {
            ServiceId::Auth => Self::AUTH,
            ServiceId::File => Self::FILE,
            ServiceId::Watcher => Self::WATCHER,
            ServiceId::Terminal => Self::TERMINAL,
            ServiceId::Configuration => Self::CONFIGURATION,
            ServiceId::Storage => Self::STORAGE,
            ServiceId::NativeHost => Self::NATIVE_HOST,
            ServiceId::Dialog => Self::DIALOG,
            ServiceId::Telemetry => Self::TELEMETRY,
            ServiceId::ExtensionHost => Self::EXTENSION_HOST,
            ServiceId::Tunnel => Self::TUNNEL,
            ServiceId::Lifecycle => Self::LIFECYCLE,
        }
    }
}

impl fmt::Display for ServiceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl TryFrom<&str> for ServiceId {
    type Error = ServiceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            Self::AUTH | "auth" | "authentication" => Ok(ServiceId::Auth),
            Self::FILE | "files" => Ok(ServiceId::File),
            Self::WATCHER | "watcher" | "watchers" | "fileWatcher" | "fileWatcherService" => {
                Ok(ServiceId::Watcher)
            }
            Self::TERMINAL | "terminal" | "terminals" => Ok(ServiceId::Terminal),
            Self::CONFIGURATION | "configuration" => Ok(ServiceId::Configuration),
            Self::STORAGE | "storage" => Ok(ServiceId::Storage),
            Self::NATIVE_HOST | "nativeHost" | "native" => Ok(ServiceId::NativeHost),
            Self::DIALOG | "dialog" | "dialogs" => Ok(ServiceId::Dialog),
            Self::TELEMETRY | "telemetry" => Ok(ServiceId::Telemetry),
            Self::EXTENSION_HOST | "extensionHost" => Ok(ServiceId::ExtensionHost),
            Self::TUNNEL | "tunnel" | "tunnels" => Ok(ServiceId::Tunnel),
            Self::LIFECYCLE | "lifecycle" => Ok(ServiceId::Lifecycle),
            other => Err(ServiceError::missing(other)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceError {
    pub code: &'static str,
    pub message: String,
}

impl ServiceError {
    pub fn missing(service_id: impl Into<String>) -> Self {
        Self {
            code: "service.missing",
            message: format!("service not registered: {}", service_id.into()),
        }
    }

    pub fn unsupported(service_id: ServiceId, command: &str) -> Self {
        Self {
            code: "service.unsupportedCommand",
            message: format!("{} does not implement command: {}", service_id, command),
        }
    }

    pub fn unsupported_listen(service_id: ServiceId, event: &str) -> Self {
        Self {
            code: "service.unsupportedListen",
            message: format!("{} does not implement event: {}", service_id, event),
        }
    }
}

impl From<ServiceError> for ChannelError {
    fn from(error: ServiceError) -> Self {
        Self {
            code: error.code.to_string(),
            message: error.message,
            details: None,
        }
    }
}

#[async_trait]
pub trait PlatformService: Send + Sync {
    fn service_id(&self) -> ServiceId;

    async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
        Err(ServiceError::unsupported(
            self.service_id(),
            &request.command,
        ))
    }

    async fn listen(
        &self,
        request: ChannelListenRequest,
        _sink: EventSink,
    ) -> Result<SubscriptionHandle, ServiceError> {
        Err(ServiceError::unsupported_listen(
            self.service_id(),
            &request.event,
        ))
    }
}

pub trait AuthService: PlatformService {}
pub trait FileService: PlatformService {}
pub trait WatcherService: PlatformService {}
pub trait TerminalService: PlatformService {}
pub trait ConfigurationService: PlatformService {}
pub trait StorageService: PlatformService {}
pub trait NativeHostService: PlatformService {}
pub trait DialogService: PlatformService {}
pub trait TelemetryService: PlatformService {}
pub trait ExtensionHostService: PlatformService {}
pub trait TunnelService: PlatformService {}
pub trait LifecycleService: PlatformService {}

#[derive(Clone)]
pub enum RegisteredService {
    Auth(Arc<dyn AuthService>),
    File(Arc<dyn FileService>),
    Watcher(Arc<dyn WatcherService>),
    Terminal(Arc<dyn TerminalService>),
    Configuration(Arc<dyn ConfigurationService>),
    Storage(Arc<dyn StorageService>),
    NativeHost(Arc<dyn NativeHostService>),
    Dialog(Arc<dyn DialogService>),
    Telemetry(Arc<dyn TelemetryService>),
    ExtensionHost(Arc<dyn ExtensionHostService>),
    Tunnel(Arc<dyn TunnelService>),
    Lifecycle(Arc<dyn LifecycleService>),
}

impl RegisteredService {
    pub fn service_id(&self) -> ServiceId {
        match self {
            RegisteredService::Auth(service) => service.service_id(),
            RegisteredService::File(service) => service.service_id(),
            RegisteredService::Watcher(service) => service.service_id(),
            RegisteredService::Terminal(service) => service.service_id(),
            RegisteredService::Configuration(service) => service.service_id(),
            RegisteredService::Storage(service) => service.service_id(),
            RegisteredService::NativeHost(service) => service.service_id(),
            RegisteredService::Dialog(service) => service.service_id(),
            RegisteredService::Telemetry(service) => service.service_id(),
            RegisteredService::ExtensionHost(service) => service.service_id(),
            RegisteredService::Tunnel(service) => service.service_id(),
            RegisteredService::Lifecycle(service) => service.service_id(),
        }
    }

    pub async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
        match self {
            RegisteredService::Auth(service) => service.call(request).await,
            RegisteredService::File(service) => service.call(request).await,
            RegisteredService::Watcher(service) => service.call(request).await,
            RegisteredService::Terminal(service) => service.call(request).await,
            RegisteredService::Configuration(service) => service.call(request).await,
            RegisteredService::Storage(service) => service.call(request).await,
            RegisteredService::NativeHost(service) => service.call(request).await,
            RegisteredService::Dialog(service) => service.call(request).await,
            RegisteredService::Telemetry(service) => service.call(request).await,
            RegisteredService::ExtensionHost(service) => service.call(request).await,
            RegisteredService::Tunnel(service) => service.call(request).await,
            RegisteredService::Lifecycle(service) => service.call(request).await,
        }
    }

    pub async fn listen(
        &self,
        request: ChannelListenRequest,
        sink: EventSink,
    ) -> Result<SubscriptionHandle, ServiceError> {
        match self {
            RegisteredService::Auth(service) => service.listen(request, sink).await,
            RegisteredService::File(service) => service.listen(request, sink).await,
            RegisteredService::Watcher(service) => service.listen(request, sink).await,
            RegisteredService::Terminal(service) => service.listen(request, sink).await,
            RegisteredService::Configuration(service) => service.listen(request, sink).await,
            RegisteredService::Storage(service) => service.listen(request, sink).await,
            RegisteredService::NativeHost(service) => service.listen(request, sink).await,
            RegisteredService::Dialog(service) => service.listen(request, sink).await,
            RegisteredService::Telemetry(service) => service.listen(request, sink).await,
            RegisteredService::ExtensionHost(service) => service.listen(request, sink).await,
            RegisteredService::Tunnel(service) => service.listen(request, sink).await,
            RegisteredService::Lifecycle(service) => service.listen(request, sink).await,
        }
    }
}

#[derive(Default)]
pub struct ServiceRegistry {
    services: HashMap<ServiceId, RegisteredService>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, service: RegisteredService) -> Option<RegisteredService> {
        self.services.insert(service.service_id(), service)
    }

    pub fn register_auth_service(
        &mut self,
        service: Arc<dyn AuthService>,
    ) -> Option<RegisteredService> {
        self.register(RegisteredService::Auth(service))
    }

    pub fn register_file_service(
        &mut self,
        service: Arc<dyn FileService>,
    ) -> Option<RegisteredService> {
        self.register(RegisteredService::File(service))
    }

    pub fn register_watcher_service(
        &mut self,
        service: Arc<dyn WatcherService>,
    ) -> Option<RegisteredService> {
        self.register(RegisteredService::Watcher(service))
    }

    pub fn register_terminal_service(
        &mut self,
        service: Arc<dyn TerminalService>,
    ) -> Option<RegisteredService> {
        self.register(RegisteredService::Terminal(service))
    }

    pub fn register_configuration_service(
        &mut self,
        service: Arc<dyn ConfigurationService>,
    ) -> Option<RegisteredService> {
        self.register(RegisteredService::Configuration(service))
    }

    pub fn register_storage_service(
        &mut self,
        service: Arc<dyn StorageService>,
    ) -> Option<RegisteredService> {
        self.register(RegisteredService::Storage(service))
    }

    pub fn register_native_host_service(
        &mut self,
        service: Arc<dyn NativeHostService>,
    ) -> Option<RegisteredService> {
        self.register(RegisteredService::NativeHost(service))
    }

    pub fn register_dialog_service(
        &mut self,
        service: Arc<dyn DialogService>,
    ) -> Option<RegisteredService> {
        self.register(RegisteredService::Dialog(service))
    }

    pub fn register_telemetry_service(
        &mut self,
        service: Arc<dyn TelemetryService>,
    ) -> Option<RegisteredService> {
        self.register(RegisteredService::Telemetry(service))
    }

    pub fn register_extension_host_service(
        &mut self,
        service: Arc<dyn ExtensionHostService>,
    ) -> Option<RegisteredService> {
        self.register(RegisteredService::ExtensionHost(service))
    }

    pub fn register_tunnel_service(
        &mut self,
        service: Arc<dyn TunnelService>,
    ) -> Option<RegisteredService> {
        self.register(RegisteredService::Tunnel(service))
    }

    pub fn register_lifecycle_service(
        &mut self,
        service: Arc<dyn LifecycleService>,
    ) -> Option<RegisteredService> {
        self.register(RegisteredService::Lifecycle(service))
    }

    pub fn get(&self, service_id: ServiceId) -> Option<RegisteredService> {
        self.services.get(&service_id).cloned()
    }

    pub fn require(&self, service_id: ServiceId) -> Result<RegisteredService, ServiceError> {
        self.get(service_id)
            .ok_or_else(|| ServiceError::missing(service_id.as_str()))
    }

    pub async fn dispatch_channel_call(
        &self,
        request: ChannelCallRequest,
    ) -> Result<Value, ServiceError> {
        let service_id = ServiceId::try_from(request.channel.as_str())?;
        let trace_id = command_trace_id("channel_dispatch", &request.request_id);
        tracing::info!(
            trace_id = trace_id.as_str(),
            channel = request.channel.as_str(),
            command = request.command.as_str(),
            request_id = request.request_id.as_str(),
            service_id = service_id.as_str(),
            "channel call dispatch"
        );

        if service_id == ServiceId::ExtensionHost {
            let trace_id = ext_host_rpc_trace_id(&request.request_id);
            tracing::info!(
                trace_id = trace_id.as_str(),
                channel = request.channel.as_str(),
                request_id = request.request_id.as_str(),
                command = request.command.as_str(),
                service_id = service_id.as_str(),
                "extension host rpc dispatch"
            );
        }

        self.require(service_id)?.call(request).await
    }

    pub async fn dispatch_channel_listen(
        &self,
        request: ChannelListenRequest,
        sink: EventSink,
    ) -> Result<SubscriptionHandle, ServiceError> {
        let service_id = ServiceId::try_from(request.channel.as_str())?;
        let trace_id = command_trace_id("channel_listen", &request.request_id);
        tracing::info!(
            trace_id = trace_id.as_str(),
            channel = request.channel.as_str(),
            command = "listen",
            event = request.event.as_str(),
            request_id = request.request_id.as_str(),
            service_id = service_id.as_str(),
            "channel listen dispatch"
        );

        self.require(service_id)?.listen(request, sink).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll, Wake, Waker},
    };

    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}
    }

    fn block_on<F: Future>(future: F) -> F::Output {
        let waker = Waker::from(Arc::new(NoopWake));
        let mut context = Context::from_waker(&waker);
        let mut future = Box::pin(future);

        loop {
            match Pin::new(&mut future).poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => std::thread::yield_now(),
            }
        }
    }

    struct TestFileService;

    #[async_trait]
    impl PlatformService for TestFileService {
        fn service_id(&self) -> ServiceId {
            ServiceId::File
        }

        async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
            Ok(json!({ "command": request.command }))
        }

        async fn listen(
            &self,
            request: ChannelListenRequest,
            sink: EventSink,
        ) -> Result<SubscriptionHandle, ServiceError> {
            sink.send(Some(json!({ "event": request.event })))
                .await
                .map_err(|error| ServiceError {
                    code: "test.sendFailed",
                    message: error.to_string(),
                })?;
            Ok(SubscriptionHandle::noop())
        }
    }

    impl FileService for TestFileService {}

    #[test]
    fn service_id_parses_parity_service_identifiers() {
        let cases = [
            (ServiceId::AUTH, ServiceId::Auth),
            ("auth", ServiceId::Auth),
            (ServiceId::FILE, ServiceId::File),
            ("files", ServiceId::File),
            (ServiceId::WATCHER, ServiceId::Watcher),
            ("fileWatcherService", ServiceId::Watcher),
            (ServiceId::TERMINAL, ServiceId::Terminal),
            ("terminals", ServiceId::Terminal),
            (ServiceId::CONFIGURATION, ServiceId::Configuration),
            ("configuration", ServiceId::Configuration),
            (ServiceId::STORAGE, ServiceId::Storage),
            ("storage", ServiceId::Storage),
            (ServiceId::NATIVE_HOST, ServiceId::NativeHost),
            ("nativeHost", ServiceId::NativeHost),
            (ServiceId::DIALOG, ServiceId::Dialog),
            ("dialogs", ServiceId::Dialog),
            (ServiceId::TELEMETRY, ServiceId::Telemetry),
            ("telemetry", ServiceId::Telemetry),
            (ServiceId::EXTENSION_HOST, ServiceId::ExtensionHost),
            ("extensionHost", ServiceId::ExtensionHost),
            (ServiceId::TUNNEL, ServiceId::Tunnel),
            ("tunnels", ServiceId::Tunnel),
            (ServiceId::LIFECYCLE, ServiceId::Lifecycle),
            ("lifecycle", ServiceId::Lifecycle),
        ];

        for (input, expected) in cases {
            assert_eq!(ServiceId::try_from(input), Ok(expected));
        }
    }

    #[test]
    fn service_id_rejects_unknown_identifier() {
        let error = ServiceId::try_from("unknownService").expect_err("unknown service rejected");

        assert_eq!(error.code, "service.missing");
        assert_eq!(error.message, "service not registered: unknownService");
    }

    #[test]
    fn register_and_get_service_by_stable_id() {
        let mut registry = ServiceRegistry::new();
        registry.register_file_service(Arc::new(TestFileService));

        let service = registry
            .get(ServiceId::File)
            .expect("file service registered");
        assert_eq!(service.service_id().as_str(), ServiceId::FILE);
    }

    #[test]
    fn get_missing_service_returns_none() {
        let registry = ServiceRegistry::new();

        assert!(registry.get(ServiceId::Terminal).is_none());
    }

    #[test]
    fn require_missing_service_returns_error() {
        let registry = ServiceRegistry::new();

        let error = match registry.require(ServiceId::Storage) {
            Ok(_) => panic!("storage service missing"),
            Err(error) => error,
        };
        assert_eq!(error.code, "service.missing");
        assert_eq!(error.message, "service not registered: storageService");
    }

    #[test]
    fn dispatch_channel_call_uses_registered_service() {
        let mut registry = ServiceRegistry::new();
        registry.register_file_service(Arc::new(TestFileService));

        let result = block_on(registry.dispatch_channel_call(ChannelCallRequest {
            request_id: "req-1".to_string(),
            channel: "files".to_string(),
            command: "stat".to_string(),
            args: Vec::new(),
            cancellation_id: None,
        }))
        .expect("dispatch succeeds");

        assert_eq!(result, json!({ "command": "stat" }));
    }

    #[test]
    fn dispatch_channel_listen_uses_registered_service() {
        let mut registry = ServiceRegistry::new();
        registry.register_file_service(Arc::new(TestFileService));
        let (sink, mut events) = tokio::sync::mpsc::channel(1);

        let handle = block_on(registry.dispatch_channel_listen(
            ChannelListenRequest {
                request_id: "req-1".to_string(),
                channel: "files".to_string(),
                event: "watch".to_string(),
                args: Vec::new(),
                subscription_id: None,
            },
            sink,
        ))
        .expect("dispatch listen succeeds");

        handle.cancel();
        assert_eq!(
            events.try_recv().expect("event payload sent"),
            Some(json!({ "event": "watch" }))
        );
    }

    #[test]
    fn default_listen_returns_unsupported_error() {
        struct UnsupportedStorageService;

        #[async_trait]
        impl PlatformService for UnsupportedStorageService {
            fn service_id(&self) -> ServiceId {
                ServiceId::Storage
            }
        }

        impl StorageService for UnsupportedStorageService {}

        let mut registry = ServiceRegistry::new();
        registry.register_storage_service(Arc::new(UnsupportedStorageService));
        let (sink, _events) = tokio::sync::mpsc::channel(1);

        let error = block_on(registry.dispatch_channel_listen(
            ChannelListenRequest {
                request_id: "req-1".to_string(),
                channel: "storage".to_string(),
                event: "changed".to_string(),
                args: Vec::new(),
                subscription_id: None,
            },
            sink,
        ))
        .expect_err("unsupported listen rejected");

        assert_eq!(error.code, "service.unsupportedListen");
        assert_eq!(
            error.message,
            "storageService does not implement event: changed"
        );
    }

    #[test]
    fn default_call_returns_unsupported_command_error() {
        struct UnsupportedDialogService;

        #[async_trait]
        impl PlatformService for UnsupportedDialogService {
            fn service_id(&self) -> ServiceId {
                ServiceId::Dialog
            }
        }

        impl DialogService for UnsupportedDialogService {}

        let mut registry = ServiceRegistry::new();
        registry.register_dialog_service(Arc::new(UnsupportedDialogService));

        let error = block_on(registry.dispatch_channel_call(ChannelCallRequest {
            request_id: "req-1".to_string(),
            channel: "dialogService".to_string(),
            command: "showOpenDialog".to_string(),
            args: Vec::new(),
            cancellation_id: None,
        }))
        .expect_err("unsupported command rejected");

        assert_eq!(error.code, "service.unsupportedCommand");
        assert_eq!(
            error.message,
            "dialogService does not implement command: showOpenDialog"
        );
    }
}
