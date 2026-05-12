use std::{
    collections::HashMap,
    env, fmt,
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    time::Instant,
};

use async_trait::async_trait;
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    contracts::{ChannelCallRequest, ChannelListenRequest, ExtHostRpcEnvelope},
    observability::{
        metric_event, record_metric, redacted_sidecar_token, MetricUnit, TraceId,
        EXTENSION_HOST_READY_MS_METRIC, REDACTED_SECRET, SIDECAR_TOKEN_ENV_NAME,
    },
    service_registry::{ExtensionHostService, PlatformService, ServiceError, ServiceId},
    subscription_manager::{EventSink, SubscriptionHandle},
};

const EVENT_HANDSHAKE: &str = "handshake";
const EVENT_LIFECYCLE: &str = "lifecycle";
const EVENT_CALLBACK: &str = "callback";
const EVENT_RPC: &str = "rpc";
const TRANSPORT_ENV: &str = "VSCODE_ATOMIC_EXTENSION_HOST_TRANSPORT";
const NODE_EXECUTABLE_ENV: &str = "VSCODE_ATOMIC_EXTENSION_HOST_NODE";
const EXTENSION_HOST_ENTRYPOINT_ENV: &str = "VSCODE_ATOMIC_EXTENSION_HOST_ENTRYPOINT";
const VSCODE_ESM_ENTRYPOINT_ENV: &str = "VSCODE_ESM_ENTRYPOINT";
const VSCODE_ESM_EXTENSION_HOST_ENTRYPOINT: &str = "vs/workbench/api/node/extensionHostProcess";
const ELECTRON_RUN_AS_NODE_ENV: &str = "ELECTRON_RUN_AS_NODE";
const VSCODE_HANDLES_UNCAUGHT_ERRORS_ENV: &str = "VSCODE_HANDLES_UNCAUGHT_ERRORS";
const HANDSHAKE_TOKEN_PREFIX: &str = "vscode-atomic-";
const STARTUP_READINESS_GRACE: std::time::Duration = std::time::Duration::from_millis(25);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtensionSidecarState {
    NotStarted,
    Starting,
    Ready,
    Crashed,
    Restarting,
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SidecarTransportEndpoint {
    UnixSocket { path: PathBuf },
    NamedPipe { name: String },
    Tcp { host: String, port: u16 },
}

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct SidecarHandshake {
    pub token: String,
    pub transport: SidecarTransportEndpoint,
}

impl fmt::Debug for SidecarHandshake {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SidecarHandshake")
            .field("token", &redacted_sidecar_token())
            .field("transport", &self.transport)
            .finish()
    }
}

impl Serialize for SidecarHandshake {
    fn serialize<Serializer>(
        &self,
        serializer: Serializer,
    ) -> Result<Serializer::Ok, Serializer::Error>
    where
        Serializer: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("SidecarHandshake", 2)?;
        state.serialize_field("token", redacted_sidecar_token())?;
        state.serialize_field("transport", &self.transport)?;
        state.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionSidecarSpawnConfig {
    pub executable: PathBuf,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub handshake: SidecarHandshake,
}

impl ExtensionSidecarSpawnConfig {
    pub fn packaged_extension_host(handshake: SidecarHandshake) -> Self {
        let mut env = HashMap::new();
        env.insert(
            VSCODE_ESM_ENTRYPOINT_ENV.to_string(),
            VSCODE_ESM_EXTENSION_HOST_ENTRYPOINT.to_string(),
        );
        env.insert(ELECTRON_RUN_AS_NODE_ENV.to_string(), "1".to_string());
        env.insert(
            VSCODE_HANDLES_UNCAUGHT_ERRORS_ENV.to_string(),
            "true".to_string(),
        );

        Self {
            executable: node_executable_path(),
            args: vec![
                extension_host_entrypoint_path()
                    .to_string_lossy()
                    .into_owned(),
                "--type=extensionHost".to_string(),
                "--transformURIs".to_string(),
            ],
            env,
            handshake,
        }
    }

    pub fn minimal_node(handshake: SidecarHandshake) -> Self {
        Self {
            executable: PathBuf::from("node"),
            args: vec![
                "-e".to_string(),
                "setInterval(() => {}, 2147483647);".to_string(),
            ],
            env: HashMap::new(),
            handshake,
        }
    }
}

#[derive(Debug)]
pub struct ExtensionSidecarProcess {
    child: Child,
    handshake: SidecarHandshake,
}

impl ExtensionSidecarProcess {
    pub fn id(&self) -> u32 {
        self.child.id()
    }

    pub fn handshake(&self) -> &SidecarHandshake {
        &self.handshake
    }

    pub fn try_wait(&mut self) -> std::io::Result<Option<std::process::ExitStatus>> {
        self.child.try_wait()
    }

    pub fn stop(&mut self) -> std::io::Result<()> {
        match self.child.try_wait()? {
            Some(_) => Ok(()),
            None => {
                self.child.kill()?;
                let _ = self.child.wait();
                Ok(())
            }
        }
    }
}

impl Drop for ExtensionSidecarProcess {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

pub trait ExtensionSidecarSpawner: Send + Sync {
    fn spawn(
        &self,
        config: ExtensionSidecarSpawnConfig,
    ) -> std::io::Result<ExtensionSidecarProcess>;
}

#[derive(Default)]
pub struct NodeExtensionSidecarSpawner;

impl ExtensionSidecarSpawner for NodeExtensionSidecarSpawner {
    fn spawn(
        &self,
        config: ExtensionSidecarSpawnConfig,
    ) -> std::io::Result<ExtensionSidecarProcess> {
        let mut command = Command::new(&config.executable);
        command
            .args(&config.args)
            .envs(&config.env)
            .env(SIDECAR_TOKEN_ENV_NAME, &config.handshake.token)
            .env(
                TRANSPORT_ENV,
                transport_env_value(&config.handshake.transport),
            )
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let child = command.spawn()?;

        tracing::info!(
            service_id = ServiceId::EXTENSION_HOST,
            command = "spawnSidecar",
            "extension sidecar process spawned"
        );

        Ok(ExtensionSidecarProcess {
            child,
            handshake: config.handshake,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionSidecarSnapshot {
    pub state: ExtensionSidecarState,
    pub handshake: Option<SidecarHandshake>,
    pub process_id: Option<u32>,
}

struct ExtensionSidecarInner {
    state: ExtensionSidecarState,
    process: Option<ExtensionSidecarProcess>,
    last_handshake: Option<SidecarHandshake>,
    subscriptions: HashMap<u64, SidecarEventSubscription>,
    next_subscription_key: u64,
}

impl Default for ExtensionSidecarInner {
    fn default() -> Self {
        Self {
            state: ExtensionSidecarState::NotStarted,
            process: None,
            last_handshake: None,
            subscriptions: HashMap::new(),
            next_subscription_key: 1,
        }
    }
}

struct SidecarEventSubscription {
    event: SidecarEventKind,
    sink: EventSink,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SidecarEventKind {
    Handshake,
    Lifecycle,
    Callback,
    Rpc,
}

impl SidecarEventKind {
    fn parse(service_id: ServiceId, event: &str) -> Result<Self, ServiceError> {
        match event {
            EVENT_HANDSHAKE => Ok(Self::Handshake),
            EVENT_LIFECYCLE => Ok(Self::Lifecycle),
            EVENT_CALLBACK => Ok(Self::Callback),
            EVENT_RPC => Ok(Self::Rpc),
            other => Err(ServiceError::unsupported_listen(service_id, other)),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Handshake => EVENT_HANDSHAKE,
            Self::Lifecycle => EVENT_LIFECYCLE,
            Self::Callback => EVENT_CALLBACK,
            Self::Rpc => EVENT_RPC,
        }
    }
}

pub struct ExtensionSidecarServiceImpl<S = NodeExtensionSidecarSpawner> {
    spawner: S,
    inner: Arc<Mutex<ExtensionSidecarInner>>,
}

impl Default for ExtensionSidecarServiceImpl<NodeExtensionSidecarSpawner> {
    fn default() -> Self {
        Self::new(NodeExtensionSidecarSpawner)
    }
}

impl<S> ExtensionSidecarServiceImpl<S>
where
    S: ExtensionSidecarSpawner,
{
    pub fn new(spawner: S) -> Self {
        Self {
            spawner,
            inner: Arc::new(Mutex::new(ExtensionSidecarInner::default())),
        }
    }

    pub fn snapshot(&self) -> ExtensionSidecarSnapshot {
        let mut inner = self.inner.lock().expect("extension sidecar lock poisoned");
        refresh_process_state(&mut inner);
        snapshot_from_inner(&inner)
    }

    pub fn start(&self) -> Result<ExtensionSidecarSnapshot, ServiceError> {
        let started = Instant::now();
        let mut inner = self.inner.lock().expect("extension sidecar lock poisoned");
        refresh_process_state(&mut inner);

        match inner.state {
            ExtensionSidecarState::Ready | ExtensionSidecarState::Starting => {
                return Ok(snapshot_from_inner(&inner));
            }
            ExtensionSidecarState::Crashed => inner.state = ExtensionSidecarState::Restarting,
            ExtensionSidecarState::NotStarted
            | ExtensionSidecarState::Stopped
            | ExtensionSidecarState::Restarting => inner.state = ExtensionSidecarState::Starting,
        }

        let transition_snapshot = snapshot_from_inner(&inner);
        emit_event(
            &inner,
            SidecarEventKind::Lifecycle,
            lifecycle_event_payload(&transition_snapshot),
        );

        let handshake = SidecarHandshake {
            token: generate_handshake_token(),
            transport: preferred_transport_endpoint(),
        };
        let config = ExtensionSidecarSpawnConfig::packaged_extension_host(handshake.clone());

        match self.spawner.spawn(config) {
            Ok(mut process) => {
                if let Some(status) = wait_for_startup_readiness(&mut process) {
                    inner.last_handshake = Some(handshake);
                    inner.process = None;
                    inner.state = ExtensionSidecarState::Crashed;
                    let snapshot = snapshot_from_inner(&inner);
                    emit_snapshot_events(&inner, &snapshot);
                    return Err(ServiceError {
                        code: "extensionHost.sidecarNotReady",
                        message: format!(
                            "extension host sidecar exited before readiness: {status}"
                        ),
                    });
                }
                inner.last_handshake = Some(handshake);
                inner.process = Some(process);
                inner.state = ExtensionSidecarState::Ready;
                let snapshot = snapshot_from_inner(&inner);
                emit_snapshot_events(&inner, &snapshot);
                record_metric(&metric_event(
                    TraceId::new("extension-host-ready"),
                    EXTENSION_HOST_READY_MS_METRIC,
                    started.elapsed().as_secs_f64() * 1000.0,
                    MetricUnit::Milliseconds,
                    Some(json!({
                        "phase": "sidecarReady",
                        "processId": snapshot.process_id,
                        "transport": snapshot.handshake.as_ref().map(|handshake| &handshake.transport),
                    })),
                ));
                Ok(snapshot)
            }
            Err(error) => {
                inner.last_handshake = Some(handshake);
                inner.process = None;
                inner.state = ExtensionSidecarState::Crashed;
                let message = redact_token(error.to_string(), inner.last_handshake.as_ref());
                let snapshot = snapshot_from_inner(&inner);
                emit_snapshot_events(&inner, &snapshot);
                Err(ServiceError {
                    code: "extensionHost.sidecarSpawnFailed",
                    message,
                })
            }
        }
    }

    pub fn stop(&self) -> Result<ExtensionSidecarSnapshot, ServiceError> {
        let mut inner = self.inner.lock().expect("extension sidecar lock poisoned");

        if let Some(process) = inner.process.as_mut() {
            process.stop().map_err(|error| ServiceError {
                code: "extensionHost.sidecarStopFailed",
                message: error.to_string(),
            })?;
        }

        inner.process = None;
        inner.last_handshake = None;
        inner.state = ExtensionSidecarState::Stopped;
        let snapshot = snapshot_from_inner(&inner);
        emit_snapshot_events(&inner, &snapshot);
        Ok(snapshot)
    }

    pub fn mark_crashed(&self) -> ExtensionSidecarSnapshot {
        let mut inner = self.inner.lock().expect("extension sidecar lock poisoned");
        if let Some(process) = inner.process.as_mut() {
            let _ = process.stop();
        }
        inner.process = None;
        inner.state = ExtensionSidecarState::Crashed;
        let snapshot = snapshot_from_inner(&inner);
        emit_snapshot_events(&inner, &snapshot);
        snapshot
    }

    pub fn emit_callback(&self, token: &str, payload: Value) -> Result<(), ServiceError> {
        let mut inner = self.inner.lock().expect("extension sidecar lock poisoned");
        refresh_process_state(&mut inner);

        if inner.state != ExtensionSidecarState::Ready || inner.process.is_none() {
            return Err(ServiceError {
                code: "extensionHost.sidecarNotReady",
                message: "extension host sidecar is not ready".to_string(),
            });
        }

        let expected_token = inner
            .last_handshake
            .as_ref()
            .map(|handshake| handshake.token.as_str())
            .ok_or_else(|| ServiceError {
                code: "extensionHost.sidecarNotReady",
                message: "extension host sidecar has no active handshake".to_string(),
            })?;

        if token != expected_token {
            return Err(ServiceError {
                code: "extensionHost.invalidSidecarToken",
                message: "extension host sidecar callback token rejected".to_string(),
            });
        }

        emit_event(
            &inner,
            SidecarEventKind::Callback,
            json!({
                "event": EVENT_CALLBACK,
                "payload": payload,
            }),
        );

        Ok(())
    }
}

#[async_trait]
impl<S> PlatformService for ExtensionSidecarServiceImpl<S>
where
    S: ExtensionSidecarSpawner,
{
    fn service_id(&self) -> ServiceId {
        ServiceId::ExtensionHost
    }

    async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
        tracing::info!(
            channel = ServiceId::EXTENSION_HOST,
            command = request.command.as_str(),
            request_id = request.request_id.as_str(),
            service_id = self.service_id().as_str(),
            "extension sidecar command dispatch"
        );

        match request.command.as_str() {
            "startSidecar" | "start" => Ok(snapshot_to_json(self.start()?)),
            "stopSidecar" | "stop" => Ok(snapshot_to_json(self.stop()?)),
            "status" => Ok(snapshot_to_json(self.snapshot())),
            "markCrashed" => Ok(snapshot_to_json(self.mark_crashed())),
            "rpc" | "sendRpc" => self.send_rpc(&request),
            command => Err(ServiceError::unsupported(self.service_id(), command)),
        }
    }

    async fn listen(
        &self,
        request: ChannelListenRequest,
        sink: EventSink,
    ) -> Result<SubscriptionHandle, ServiceError> {
        let event = SidecarEventKind::parse(self.service_id(), &request.event)?;
        tracing::info!(
            channel = ServiceId::EXTENSION_HOST,
            command = "listen",
            event = request.event.as_str(),
            request_id = request.request_id.as_str(),
            service_id = self.service_id().as_str(),
            "extension sidecar listen dispatch"
        );
        let mut inner = self.inner.lock().expect("extension sidecar lock poisoned");
        let subscription_key = inner.next_subscription_key;
        inner.next_subscription_key += 1;

        match event {
            SidecarEventKind::Handshake => {
                let snapshot = snapshot_from_inner(&inner);
                if let Some(payload) = handshake_event_payload(&snapshot) {
                    send_sidecar_event(event, &sink, payload);
                }
            }
            SidecarEventKind::Lifecycle => {
                let snapshot = snapshot_from_inner(&inner);
                send_sidecar_event(event, &sink, lifecycle_event_payload(&snapshot));
            }
            SidecarEventKind::Callback | SidecarEventKind::Rpc => {}
        }

        inner
            .subscriptions
            .insert(subscription_key, SidecarEventSubscription { event, sink });

        let inner = self.inner.clone();

        Ok(SubscriptionHandle::new(move || {
            inner
                .lock()
                .expect("extension sidecar lock poisoned")
                .subscriptions
                .remove(&subscription_key);
        }))
    }
}

impl<S> ExtensionHostService for ExtensionSidecarServiceImpl<S> where S: ExtensionSidecarSpawner {}

impl<S> ExtensionSidecarServiceImpl<S>
where
    S: ExtensionSidecarSpawner,
{
    fn send_rpc(&self, request: &ChannelCallRequest) -> Result<Value, ServiceError> {
        let envelope = ext_host_rpc_payload(request)?;
        let mut inner = self.inner.lock().expect("extension sidecar lock poisoned");
        refresh_process_state(&mut inner);

        emit_event(
            &inner,
            SidecarEventKind::Rpc,
            json!({
                "event": EVENT_RPC,
                "envelope": envelope,
            }),
        );

        Ok(envelope)
    }
}

fn ext_host_rpc_payload(request: &ChannelCallRequest) -> Result<Value, ServiceError> {
    let payload = request.args.first().ok_or_else(|| ServiceError {
        code: "extensionHost.rpcMissingEnvelope",
        message: "extension host rpc requires extHost.protocol envelope".to_string(),
    })?;

    let envelope: ExtHostRpcEnvelope =
        serde_json::from_value(payload.clone()).map_err(|error| ServiceError {
            code: "extensionHost.rpcInvalidEnvelope",
            message: format!("extension host rpc envelope is invalid: {error}"),
        })?;

    if envelope.protocol != ExtHostRpcEnvelope::PROTOCOL {
        return Err(ServiceError {
            code: "extensionHost.rpcInvalidProtocol",
            message: format!(
                "extension host rpc protocol must be {}",
                ExtHostRpcEnvelope::PROTOCOL
            ),
        });
    }

    serde_json::to_value(envelope).map_err(|error| ServiceError {
        code: "extensionHost.rpcSerializationFailed",
        message: error.to_string(),
    })
}

pub fn generate_handshake_token() -> String {
    let mut random_bytes = [0_u8; 32];
    getrandom::getrandom(&mut random_bytes).expect("OS random source unavailable");

    let mut token = String::with_capacity(HANDSHAKE_TOKEN_PREFIX.len() + random_bytes.len() * 2);
    token.push_str(HANDSHAKE_TOKEN_PREFIX);
    append_lower_hex(&mut token, &random_bytes);
    token
}

fn append_lower_hex(output: &mut String, bytes: &[u8]) {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
}

pub fn preferred_transport_endpoint() -> SidecarTransportEndpoint {
    #[cfg(unix)]
    {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "vscode-atomic-ext-{}.sock",
            transport_name_suffix()
        ));
        return SidecarTransportEndpoint::UnixSocket { path };
    }

    #[cfg(windows)]
    {
        return SidecarTransportEndpoint::NamedPipe {
            name: format!(r"\\.\pipe\vscode-atomic-ext-{}", transport_name_suffix()),
        };
    }

    #[allow(unreachable_code)]
    SidecarTransportEndpoint::Tcp {
        host: "127.0.0.1".to_string(),
        port: 0,
    }
}

fn transport_name_suffix() -> String {
    let mut random_bytes = [0_u8; 16];
    getrandom::getrandom(&mut random_bytes).expect("OS random source unavailable");

    let mut suffix = String::with_capacity(random_bytes.len() * 2);
    append_lower_hex(&mut suffix, &random_bytes);
    suffix
}

fn node_executable_path() -> PathBuf {
    env::var_os(NODE_EXECUTABLE_ENV)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("node"))
}

fn extension_host_entrypoint_path() -> PathBuf {
    env::var_os(EXTENSION_HOST_ENTRYPOINT_ENV)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(default_extension_host_entrypoint_path)
}

fn default_extension_host_entrypoint_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(|repo_root| repo_root.join("out").join("bootstrap-fork.js"))
        .unwrap_or_else(|| PathBuf::from("out").join("bootstrap-fork.js"))
}

fn snapshot_from_inner(inner: &ExtensionSidecarInner) -> ExtensionSidecarSnapshot {
    ExtensionSidecarSnapshot {
        state: inner.state,
        handshake: inner.last_handshake.clone(),
        process_id: inner.process.as_ref().map(ExtensionSidecarProcess::id),
    }
}

fn refresh_process_state(inner: &mut ExtensionSidecarInner) {
    let Some(process) = inner.process.as_mut() else {
        return;
    };

    match process.try_wait() {
        Ok(Some(status)) => {
            tracing::warn!(
                status = status.to_string(),
                "extension sidecar process exited"
            );
            inner.process = None;
            inner.state = ExtensionSidecarState::Crashed;
        }
        Ok(None) => {}
        Err(error) => {
            let message = redact_token(error.to_string(), inner.last_handshake.as_ref());
            tracing::warn!(error = message, "failed to poll extension sidecar process");
            inner.process = None;
            inner.state = ExtensionSidecarState::Crashed;
        }
    }
}

fn wait_for_startup_readiness(
    process: &mut ExtensionSidecarProcess,
) -> Option<std::process::ExitStatus> {
    let deadline = std::time::Instant::now() + STARTUP_READINESS_GRACE;
    loop {
        match process.try_wait() {
            Ok(Some(status)) => return Some(status),
            Ok(None) => {
                if std::time::Instant::now() >= deadline {
                    return None;
                }
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            Err(error) => {
                tracing::warn!(
                    error = error.to_string(),
                    "failed to poll extension sidecar startup readiness"
                );
                return None;
            }
        }
    }
}

fn redact_token(message: String, handshake: Option<&SidecarHandshake>) -> String {
    let Some(handshake) = handshake else {
        return message;
    };

    message.replace(&handshake.token, REDACTED_SECRET)
}

fn snapshot_to_json(snapshot: ExtensionSidecarSnapshot) -> Value {
    json!({
        "state": snapshot.state,
        "handshake": snapshot.handshake,
        "processId": snapshot.process_id,
    })
}

fn emit_snapshot_events(inner: &ExtensionSidecarInner, snapshot: &ExtensionSidecarSnapshot) {
    if let Some(payload) = handshake_event_payload(snapshot) {
        emit_event(inner, SidecarEventKind::Handshake, payload);
    }
    emit_event(
        inner,
        SidecarEventKind::Lifecycle,
        lifecycle_event_payload(snapshot),
    );
}

fn handshake_event_payload(snapshot: &ExtensionSidecarSnapshot) -> Option<Value> {
    snapshot.handshake.as_ref().map(|handshake| {
        json!({
            "event": EVENT_HANDSHAKE,
            "state": snapshot.state,
            "handshake": handshake,
            "processId": snapshot.process_id,
        })
    })
}

fn lifecycle_event_payload(snapshot: &ExtensionSidecarSnapshot) -> Value {
    json!({
        "event": EVENT_LIFECYCLE,
        "state": snapshot.state,
        "processId": snapshot.process_id,
    })
}

fn emit_event(inner: &ExtensionSidecarInner, event: SidecarEventKind, payload: Value) {
    for subscription in inner
        .subscriptions
        .values()
        .filter(|subscription| subscription.event == event)
    {
        send_sidecar_event(event, &subscription.sink, payload.clone());
    }
}

fn send_sidecar_event(event: SidecarEventKind, sink: &EventSink, payload: Value) {
    if let Err(error) = sink.try_send(Some(payload)) {
        tracing::warn!(
            event = event.as_str(),
            error = error.to_string(),
            "failed to queue extension sidecar event"
        );
    }
}

fn transport_env_value(transport: &SidecarTransportEndpoint) -> String {
    match transport {
        SidecarTransportEndpoint::UnixSocket { path } => format!("unix:{}", path.display()),
        SidecarTransportEndpoint::NamedPipe { name } => format!("pipe:{name}"),
        SidecarTransportEndpoint::Tcp { host, port } => format!("tcp:{host}:{port}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_generation_produces_random_hex_tokens() {
        let tokens = (0..16)
            .map(|_| generate_handshake_token())
            .collect::<Vec<_>>();

        for token in &tokens {
            let hex = token
                .strip_prefix(HANDSHAKE_TOKEN_PREFIX)
                .expect("token has vscode-atomic prefix");

            assert_eq!(hex.len(), 64);
            assert_eq!(token.len(), HANDSHAKE_TOKEN_PREFIX.len() + 64);
            assert!(hex.bytes().all(|byte| byte.is_ascii_hexdigit()));
            assert!(hex.bytes().all(|byte| !byte.is_ascii_uppercase()));
        }

        let unique_tokens = tokens.iter().collect::<std::collections::HashSet<_>>();
        assert_eq!(unique_tokens.len(), tokens.len());
    }

    #[test]
    fn handshake_debug_and_json_redact_token() {
        let handshake = SidecarHandshake {
            token: "secret-token".to_string(),
            transport: SidecarTransportEndpoint::Tcp {
                host: "127.0.0.1".to_string(),
                port: 0,
            },
        };

        let debug = format!("{handshake:?}");
        let json = serde_json::to_value(&handshake).expect("handshake serializes");

        assert!(!debug.contains("secret-token"));
        assert!(debug.contains(REDACTED_SECRET));
        assert_eq!(json["token"], REDACTED_SECRET);
    }

    #[test]
    fn packaged_extension_host_config_uses_vscode_node_entrypoint() {
        let handshake = test_handshake();

        let config = ExtensionSidecarSpawnConfig::packaged_extension_host(handshake.clone());

        assert_eq!(config.executable, PathBuf::from("node"));
        assert_ne!(config.args.first().map(String::as_str), Some("-e"));
        assert!(config
            .args
            .first()
            .expect("entrypoint arg present")
            .ends_with("out/bootstrap-fork.js"));
        assert!(config.args.iter().any(|arg| arg == "--type=extensionHost"));
        assert!(config.args.iter().any(|arg| arg == "--transformURIs"));
        assert_eq!(
            config.env.get(VSCODE_ESM_ENTRYPOINT_ENV),
            Some(&VSCODE_ESM_EXTENSION_HOST_ENTRYPOINT.to_string())
        );
        assert_eq!(
            config.env.get(ELECTRON_RUN_AS_NODE_ENV),
            Some(&"1".to_string())
        );
        assert_eq!(
            config.env.get(VSCODE_HANDLES_UNCAUGHT_ERRORS_ENV),
            Some(&"true".to_string())
        );
        assert!(!config.env.contains_key(SIDECAR_TOKEN_ENV_NAME));
        assert!(!config.env.contains_key(TRANSPORT_ENV));
        assert_eq!(config.handshake, handshake);
    }

    #[test]
    fn transport_env_value_encodes_authenticated_endpoint_without_token() {
        let handshake = test_handshake();

        let value = transport_env_value(&handshake.transport);

        assert_eq!(value, "tcp:127.0.0.1:0");
        assert!(!value.contains(&handshake.token));
    }

    #[test]
    fn preferred_transport_endpoint_does_not_embed_sidecar_token_value() {
        let endpoint = preferred_transport_endpoint();
        let value = transport_env_value(&endpoint);

        assert!(!value.contains("vscode-atomic-ext-vscode-atomic-"));
    }

    #[test]
    fn spawn_failure_redacts_token_from_error_message() {
        let service = ExtensionSidecarServiceImpl::new(TokenEchoingFailingSpawner);

        let error = service.start().expect_err("spawn fails");
        let snapshot = service.snapshot();
        let token = snapshot
            .handshake
            .as_ref()
            .expect("failed start retains redacted handshake context internally")
            .token
            .as_str();

        assert_eq!(error.code, "extensionHost.sidecarSpawnFailed");
        assert!(!error.message.contains(token));
        assert!(error.message.contains(REDACTED_SECRET));
    }

    #[test]
    fn start_transitions_not_started_to_ready() {
        let service = ExtensionSidecarServiceImpl::new(FakeSpawner);

        let snapshot = service.start().expect("sidecar starts");

        assert_eq!(snapshot.state, ExtensionSidecarState::Ready);
        assert!(snapshot.handshake.is_some());
        assert!(snapshot.process_id.is_some());

        service.stop().expect("sidecar stops");
    }

    #[test]
    fn start_spawns_packaged_extension_host_not_minimal_stub() {
        let seen = Arc::new(Mutex::new(None));
        let service = ExtensionSidecarServiceImpl::new(CapturingSpawner { seen: seen.clone() });

        let snapshot = service.start().expect("sidecar starts");
        let config = seen
            .lock()
            .expect("captured config lock")
            .clone()
            .expect("spawn config captured");

        assert_eq!(snapshot.state, ExtensionSidecarState::Ready);
        assert_eq!(
            config.handshake.token,
            snapshot.handshake.expect("handshake").token
        );
        assert_ne!(config.args.first().map(String::as_str), Some("-e"));
        assert!(config
            .args
            .first()
            .expect("entrypoint arg present")
            .ends_with("out/bootstrap-fork.js"));
        assert!(config.args.iter().any(|arg| arg == "--type=extensionHost"));

        service.stop().expect("sidecar stops");
    }

    #[test]
    fn stop_transitions_ready_to_stopped() {
        let service = ExtensionSidecarServiceImpl::new(FakeSpawner);
        service.start().expect("sidecar starts");

        let snapshot = service.stop().expect("sidecar stops");

        assert_eq!(snapshot.state, ExtensionSidecarState::Stopped);
        assert!(snapshot.handshake.is_none());
        assert!(snapshot.process_id.is_none());
    }

    #[test]
    fn snapshot_marks_exited_process_crashed() {
        let service = ExtensionSidecarServiceImpl::new(DelayedExitSpawner);
        service.start().expect("sidecar starts");
        std::thread::sleep(std::time::Duration::from_millis(75));

        let snapshot = service.snapshot();

        assert_eq!(snapshot.state, ExtensionSidecarState::Crashed);
        assert!(snapshot.process_id.is_none());
    }

    #[test]
    fn spawn_failure_transitions_to_crashed() {
        let service = ExtensionSidecarServiceImpl::new(FailingSpawner);

        let error = service.start().expect_err("spawn fails");
        let snapshot = service.snapshot();

        assert_eq!(error.code, "extensionHost.sidecarSpawnFailed");
        assert_eq!(snapshot.state, ExtensionSidecarState::Crashed);
        assert!(snapshot.handshake.is_some());
    }

    #[test]
    fn start_rejects_process_that_exits_before_readiness() {
        let service = ExtensionSidecarServiceImpl::new(ShortLivedSpawner);

        let error = service
            .start()
            .expect_err("early sidecar exit fails startup readiness");
        let snapshot = service.snapshot();

        assert_eq!(error.code, "extensionHost.sidecarNotReady");
        assert!(error
            .message
            .contains("extension host sidecar exited before readiness"));
        assert_eq!(snapshot.state, ExtensionSidecarState::Crashed);
        assert!(snapshot.process_id.is_none());
    }

    #[test]
    fn crashed_start_uses_restart_path_to_ready() {
        let service = ExtensionSidecarServiceImpl::new(FakeSpawner);

        assert_eq!(service.mark_crashed().state, ExtensionSidecarState::Crashed);

        let snapshot = service.start().expect("sidecar restarts");

        assert_eq!(snapshot.state, ExtensionSidecarState::Ready);
        assert!(snapshot.process_id.is_some());

        service.stop().expect("sidecar stops");
    }

    #[tokio::test]
    async fn platform_call_reports_status_and_marks_crashed() {
        let service = ExtensionSidecarServiceImpl::new(FakeSpawner);

        let initial = service
            .call(ChannelCallRequest {
                request_id: "req-status".to_string(),
                channel: "extensionHost".to_string(),
                command: "status".to_string(),
                args: Vec::new(),
                cancellation_id: None,
            })
            .await
            .expect("status dispatch succeeds");
        assert_eq!(initial["state"], "NotStarted");
        assert!(initial["handshake"].is_null());

        let crashed = service
            .call(ChannelCallRequest {
                request_id: "req-crash".to_string(),
                channel: "extensionHost".to_string(),
                command: "markCrashed".to_string(),
                args: Vec::new(),
                cancellation_id: None,
            })
            .await
            .expect("markCrashed dispatch succeeds");
        assert_eq!(crashed["state"], "Crashed");
    }

    #[tokio::test]
    async fn platform_call_roundtrips_ext_host_rpc_envelope() {
        let service = ExtensionSidecarServiceImpl::new(FakeSpawner);
        let (sink, mut events) = tokio::sync::mpsc::channel(1);
        let handle = service
            .listen(listen_request(EVENT_RPC), sink)
            .await
            .expect("rpc listen succeeds");
        let envelope = crate::contracts::ExtHostRpcEnvelope::request(
            "rpc-1".to_string(),
            "trace-rpc-1".to_string(),
            crate::contracts::ExtHostRpcDirection::ExtHostToMain,
            crate::contracts::ExtHostRpcRequest {
                actor: "MainThreadCommands".to_string(),
                method: "$executeCommand".to_string(),
                args: vec![json!("workbench.action.files.save")],
                cancellation_id: Some(crate::contracts::CancellationId("cancel-rpc-1".to_string())),
            },
        );

        let result = service
            .call(ChannelCallRequest {
                request_id: "req-rpc".to_string(),
                channel: "extensionHost".to_string(),
                command: "rpc".to_string(),
                args: vec![serde_json::to_value(envelope).expect("serialize rpc envelope")],
                cancellation_id: None,
            })
            .await
            .expect("rpc dispatch accepts extHost.protocol envelope");

        assert_eq!(result["protocol"], "extHost.protocol");
        assert_eq!(result["requestId"], "rpc-1");
        assert_eq!(result["traceId"], "trace-rpc-1");
        assert_eq!(result["type"], "request");
        assert_eq!(result["request"]["actor"], "MainThreadCommands");
        assert_eq!(result["request"]["cancellationId"], "cancel-rpc-1");

        let event = events
            .try_recv()
            .expect("rpc dispatch queues protocol event")
            .expect("rpc event has payload");
        assert_eq!(event["event"], EVENT_RPC);
        assert_eq!(event["envelope"], result);

        handle.cancel();
    }

    #[tokio::test]
    async fn platform_call_roundtrips_ext_host_rpc_response_error_and_cancel() {
        let service = ExtensionSidecarServiceImpl::new(FakeSpawner);
        let envelopes = [
            crate::contracts::ExtHostRpcEnvelope::response(
                "rpc-response".to_string(),
                "trace-response".to_string(),
                crate::contracts::ExtHostRpcDirection::MainToExtHost,
                Some(json!({ "ok": true })),
            ),
            crate::contracts::ExtHostRpcEnvelope::error(
                "rpc-error".to_string(),
                "trace-error".to_string(),
                crate::contracts::ExtHostRpcDirection::MainToExtHost,
                crate::contracts::ChannelError {
                    code: "ENOENT".to_string(),
                    message: "Missing".to_string(),
                    details: Some(json!({ "path": "/missing" })),
                },
            ),
            crate::contracts::ExtHostRpcEnvelope::cancel(
                "rpc-cancel".to_string(),
                "trace-cancel".to_string(),
                crate::contracts::ExtHostRpcDirection::ExtHostToMain,
                Some(crate::contracts::CancellationId("cancel-rpc".to_string())),
            ),
        ];

        for envelope in envelopes {
            let result = service
                .call(ChannelCallRequest {
                    request_id: format!("dispatch-{}", envelope.request_id),
                    channel: "extensionHost".to_string(),
                    command: "sendRpc".to_string(),
                    args: vec![serde_json::to_value(&envelope).expect("serialize rpc envelope")],
                    cancellation_id: None,
                })
                .await
                .expect("rpc dispatch preserves envelope");

            assert_eq!(result["protocol"], "extHost.protocol");
            assert_eq!(result["requestId"], envelope.request_id);
            assert_eq!(result["traceId"], envelope.trace_id);
        }
    }

    #[tokio::test]
    async fn platform_call_rejects_invalid_ext_host_rpc_protocol() {
        let service = ExtensionSidecarServiceImpl::new(FakeSpawner);

        let error = service
            .call(ChannelCallRequest {
                request_id: "req-rpc".to_string(),
                channel: "extensionHost".to_string(),
                command: "rpc".to_string(),
                args: vec![json!({
                    "protocol": "wrong.protocol",
                    "requestId": "rpc-1",
                    "traceId": "trace-rpc-1",
                    "direction": "extHostToMain",
                    "type": "cancel"
                })],
                cancellation_id: None,
            })
            .await
            .expect_err("invalid rpc protocol rejected");

        assert_eq!(error.code, "extensionHost.rpcInvalidProtocol");
    }

    #[tokio::test]
    async fn listen_lifecycle_sends_initial_and_transition_events() {
        let service = ExtensionSidecarServiceImpl::new(FakeSpawner);
        let (sink, mut events) = tokio::sync::mpsc::channel(4);

        let handle = service
            .listen(listen_request(EVENT_LIFECYCLE), sink)
            .await
            .expect("lifecycle listen succeeds");

        let initial = events
            .try_recv()
            .expect("lifecycle listen queues initial event")
            .expect("initial lifecycle event has payload");
        assert_eq!(initial["event"], EVENT_LIFECYCLE);
        assert_eq!(initial["state"], "NotStarted");

        service.start().expect("sidecar starts");

        let starting = events
            .try_recv()
            .expect("start queues starting lifecycle event")
            .expect("starting lifecycle event has payload");
        assert_eq!(starting["event"], EVENT_LIFECYCLE);
        assert_eq!(starting["state"], "Starting");
        assert!(starting["processId"].is_null());

        let ready = events
            .try_recv()
            .expect("start queues ready lifecycle event")
            .expect("ready lifecycle event has payload");
        assert_eq!(ready["event"], EVENT_LIFECYCLE);
        assert_eq!(ready["state"], "Ready");
        assert!(ready["processId"].is_number());

        handle.cancel();
        service.stop().expect("sidecar stops");
    }

    #[tokio::test]
    async fn crashed_start_emits_restarting_then_ready() {
        let service = ExtensionSidecarServiceImpl::new(FakeSpawner);
        assert_eq!(service.mark_crashed().state, ExtensionSidecarState::Crashed);
        let (sink, mut events) = tokio::sync::mpsc::channel(4);

        let handle = service
            .listen(listen_request(EVENT_LIFECYCLE), sink)
            .await
            .expect("lifecycle listen succeeds");

        let initial = events
            .try_recv()
            .expect("lifecycle listen queues crashed state")
            .expect("crashed lifecycle event has payload");
        assert_eq!(initial["state"], "Crashed");

        service.start().expect("sidecar restarts");

        let restarting = events
            .try_recv()
            .expect("restart queues restarting lifecycle event")
            .expect("restarting lifecycle event has payload");
        assert_eq!(restarting["state"], "Restarting");

        let ready = events
            .try_recv()
            .expect("restart queues ready lifecycle event")
            .expect("ready lifecycle event has payload");
        assert_eq!(ready["state"], "Ready");

        handle.cancel();
        service.stop().expect("sidecar stops");
    }

    #[tokio::test]
    async fn listen_callback_bridges_token_verified_payloads() {
        let service = ExtensionSidecarServiceImpl::new(FakeSpawner);
        let snapshot = service.start().expect("sidecar starts");
        let token = snapshot
            .handshake
            .as_ref()
            .expect("started sidecar exposes handshake")
            .token
            .clone();
        let (sink, mut events) = tokio::sync::mpsc::channel(4);

        let handle = service
            .listen(listen_request(EVENT_CALLBACK), sink)
            .await
            .expect("callback listen succeeds");

        let error = service
            .emit_callback("wrong-token", json!({ "ignored": true }))
            .expect_err("invalid callback token rejected");
        assert_eq!(error.code, "extensionHost.invalidSidecarToken");
        assert!(events.try_recv().is_err());

        service
            .emit_callback(&token, json!({ "requestId": "callback-1" }))
            .expect("valid callback token accepted");

        let callback = events
            .try_recv()
            .expect("callback queues payload")
            .expect("callback event has payload");
        assert_eq!(callback["event"], EVENT_CALLBACK);
        assert_eq!(callback["payload"]["requestId"], "callback-1");

        handle.cancel();
        service.stop().expect("sidecar stops");
    }

    #[tokio::test]
    async fn stopped_sidecar_rejects_previous_callback_token() {
        let service = ExtensionSidecarServiceImpl::new(FakeSpawner);
        let snapshot = service.start().expect("sidecar starts");
        let token = snapshot
            .handshake
            .as_ref()
            .expect("started sidecar exposes handshake")
            .token
            .clone();

        service.stop().expect("sidecar stops");

        let error = service
            .emit_callback(&token, json!({ "ignored": true }))
            .expect_err("stopped sidecar rejects stale callback token");

        assert_eq!(error.code, "extensionHost.sidecarNotReady");
    }

    #[tokio::test]
    async fn listen_rejects_unsupported_sidecar_event() {
        let service = ExtensionSidecarServiceImpl::new(FakeSpawner);
        let (sink, _events) = tokio::sync::mpsc::channel(1);

        let error = service
            .listen(listen_request("stdout"), sink)
            .await
            .expect_err("unsupported listen rejected");

        assert_eq!(error.code, "service.unsupportedListen");
        assert_eq!(
            error.message,
            "extensionHostService does not implement event: stdout"
        );
    }

    fn listen_request(event: &str) -> ChannelListenRequest {
        ChannelListenRequest {
            request_id: format!("req-{event}"),
            channel: ServiceId::EXTENSION_HOST.to_string(),
            event: event.to_string(),
            args: Vec::new(),
            subscription_id: None,
        }
    }

    struct FailingSpawner;

    struct FakeSpawner;

    struct ShortLivedSpawner;

    struct DelayedExitSpawner;

    struct TokenEchoingFailingSpawner;

    struct CapturingSpawner {
        seen: Arc<Mutex<Option<ExtensionSidecarSpawnConfig>>>,
    }

    impl ExtensionSidecarSpawner for FakeSpawner {
        fn spawn(
            &self,
            config: ExtensionSidecarSpawnConfig,
        ) -> std::io::Result<ExtensionSidecarProcess> {
            spawn_shell(config.handshake, "sleep 30")
        }
    }

    impl ExtensionSidecarSpawner for FailingSpawner {
        fn spawn(
            &self,
            _config: ExtensionSidecarSpawnConfig,
        ) -> std::io::Result<ExtensionSidecarProcess> {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "spawn failed",
            ))
        }
    }

    impl ExtensionSidecarSpawner for ShortLivedSpawner {
        fn spawn(
            &self,
            config: ExtensionSidecarSpawnConfig,
        ) -> std::io::Result<ExtensionSidecarProcess> {
            spawn_shell(config.handshake, "exit 7")
        }
    }

    impl ExtensionSidecarSpawner for DelayedExitSpawner {
        fn spawn(
            &self,
            config: ExtensionSidecarSpawnConfig,
        ) -> std::io::Result<ExtensionSidecarProcess> {
            spawn_shell(config.handshake, "sleep 0.05; exit 7")
        }
    }

    impl ExtensionSidecarSpawner for TokenEchoingFailingSpawner {
        fn spawn(
            &self,
            config: ExtensionSidecarSpawnConfig,
        ) -> std::io::Result<ExtensionSidecarProcess> {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "failed with {SIDECAR_TOKEN_ENV_NAME}={}",
                    config.handshake.token
                ),
            ))
        }
    }

    impl ExtensionSidecarSpawner for CapturingSpawner {
        fn spawn(
            &self,
            config: ExtensionSidecarSpawnConfig,
        ) -> std::io::Result<ExtensionSidecarProcess> {
            *self.seen.lock().expect("captured config lock") = Some(config.clone());
            spawn_shell(config.handshake, "sleep 30")
        }
    }

    fn test_handshake() -> SidecarHandshake {
        SidecarHandshake {
            token: "secret-token".to_string(),
            transport: SidecarTransportEndpoint::Tcp {
                host: "127.0.0.1".to_string(),
                port: 0,
            },
        }
    }

    fn spawn_shell(
        handshake: SidecarHandshake,
        script: &str,
    ) -> std::io::Result<ExtensionSidecarProcess> {
        let child = Command::new("sh")
            .arg("-c")
            .arg(script)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        Ok(ExtensionSidecarProcess { child, handshake })
    }
}
