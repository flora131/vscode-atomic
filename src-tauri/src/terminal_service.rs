use std::{
    collections::HashMap,
    io::{BufReader, Read, Write},
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{sync_channel, Receiver, SyncSender, TryRecvError, TrySendError},
        Arc, Mutex,
    },
    thread,
    time::Instant,
};

use async_trait::async_trait;
#[cfg(feature = "terminal-pty")]
use portable_pty::{native_pty_system, Child as PtyChild, CommandBuilder, MasterPty, PtySize};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::broadcast;

use crate::{
    contracts::{ChannelCallRequest, ChannelListenRequest},
    observability::{metric_event, record_metric, MetricUnit, TraceId, TERMINAL_SPAWN_MS_METRIC},
    service_registry::{
        PlatformService, ServiceError, ServiceId, ServiceRegistry, TerminalService,
    },
    subscription_manager::{EventSink, SubscriptionHandle},
};

const TERMINAL_STREAM_BUFFER: usize = 256;
const TERMINAL_BROADCAST_BUFFER: usize = 256;
const SHELL_INTEGRATION_ENV: &str = "VSCODE_SHELL_INTEGRATION";

pub type TerminalId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TerminalLifecycleState {
    Created,
    Launching,
    Running,
    Exited,
    Reconnected,
    Disposed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalSize {
    pub cols: u16,
    pub rows: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnTerminalRequest {
    #[serde(default)]
    pub id: Option<TerminalId>,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub cols: Option<u16>,
    #[serde(default)]
    pub rows: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResizeTerminalRequest {
    pub id: TerminalId,
    pub cols: u16,
    pub rows: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KillTerminalRequest {
    pub id: TerminalId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteTerminalRequest {
    pub id: TerminalId,
    pub data: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalStatus {
    pub id: TerminalId,
    pub state: TerminalLifecycleState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<TerminalSize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalStreamEvent {
    pub id: TerminalId,
    pub stream: TerminalStreamKind,
    pub data: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalLifecycleEvent {
    pub id: TerminalId,
    pub state: TerminalLifecycleState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<TerminalSize>,
}

impl From<TerminalStatus> for TerminalLifecycleEvent {
    fn from(status: TerminalStatus) -> Self {
        Self {
            id: status.id,
            state: status.state,
            size: status.size,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TerminalStreamKind {
    Stdout,
    Stderr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendSpawnRequest {
    pub id: TerminalId,
    pub command: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
    pub env: HashMap<String, String>,
    pub size: Option<TerminalSize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalServiceError {
    pub code: &'static str,
    pub message: String,
}

impl TerminalServiceError {
    fn backend(message: impl Into<String>) -> Self {
        Self {
            code: "terminal.backend",
            message: message.into(),
        }
    }

    fn missing(id: &str) -> Self {
        Self {
            code: "terminal.missing",
            message: format!("terminal not found: {id}"),
        }
    }

    fn invalid(message: impl Into<String>) -> Self {
        Self {
            code: "terminal.invalidRequest",
            message: message.into(),
        }
    }

    fn exists(id: &str) -> Self {
        Self {
            code: "terminal.exists",
            message: format!("terminal already exists: {id}"),
        }
    }
}

impl From<TerminalServiceError> for ServiceError {
    fn from(error: TerminalServiceError) -> Self {
        Self {
            code: error.code,
            message: error.message,
        }
    }
}

pub trait TerminalChild: Send {
    fn kill(&mut self) -> Result<(), TerminalServiceError>;
    fn write_input(&mut self, data: &[u8]) -> Result<(), TerminalServiceError>;
    fn take_stdout(&mut self) -> Option<Box<dyn Read + Send>>;
    fn take_stderr(&mut self) -> Option<Box<dyn Read + Send>>;
}

pub trait TerminalProcessBackend: Send + Sync {
    fn spawn(
        &self,
        request: BackendSpawnRequest,
    ) -> Result<Box<dyn TerminalChild>, TerminalServiceError>;

    fn resize(&self, _id: &str, _size: TerminalSize) -> Result<(), TerminalServiceError> {
        Ok(())
    }

    fn cleanup(&self, _id: &str) -> Result<(), TerminalServiceError> {
        Ok(())
    }
}

#[derive(Default)]
pub struct PortableProcessBackend;

impl TerminalProcessBackend for PortableProcessBackend {
    fn spawn(
        &self,
        request: BackendSpawnRequest,
    ) -> Result<Box<dyn TerminalChild>, TerminalServiceError> {
        let mut command = Command::new(&request.command);
        command.args(&request.args);
        command.envs(&request.env);
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        if let Some(cwd) = request.cwd {
            command.current_dir(cwd);
        }

        let child = command.spawn().map_err(|error| {
            TerminalServiceError::backend(format!("failed to spawn terminal process: {error}"))
        })?;

        Ok(Box::new(PortableTerminalChild { child }))
    }
}

#[cfg(feature = "terminal-pty")]
#[derive(Default)]
pub struct PortablePtyBackend {
    masters: Mutex<HashMap<TerminalId, Arc<Mutex<Box<dyn MasterPty + Send>>>>>,
}

#[cfg(feature = "terminal-pty")]
impl TerminalProcessBackend for PortablePtyBackend {
    fn spawn(
        &self,
        request: BackendSpawnRequest,
    ) -> Result<Box<dyn TerminalChild>, TerminalServiceError> {
        let pty_system = native_pty_system();
        let size = request.size.unwrap_or(TerminalSize { cols: 80, rows: 24 });
        let pair = pty_system.openpty(to_pty_size(size)).map_err(|error| {
            TerminalServiceError::backend(format!("failed to open pty: {error}"))
        })?;

        let mut command = CommandBuilder::new(&request.command);
        for arg in &request.args {
            command.arg(arg);
        }
        if let Some(cwd) = request.cwd.as_deref() {
            command.cwd(cwd);
        }
        for (key, value) in &request.env {
            command.env(key, value);
        }

        let child = pair.slave.spawn_command(command).map_err(|error| {
            TerminalServiceError::backend(format!("failed to spawn terminal pty: {error}"))
        })?;
        drop(pair.slave);

        let stdout = pair.master.try_clone_reader().map_err(|error| {
            TerminalServiceError::backend(format!("failed to clone pty reader: {error}"))
        })?;
        let stdin = pair.master.take_writer().map_err(|error| {
            TerminalServiceError::backend(format!("failed to open pty writer: {error}"))
        })?;
        let master = Arc::new(Mutex::new(pair.master));
        self.masters
            .lock()
            .expect("terminal pty masters lock poisoned")
            .insert(request.id.clone(), master);

        Ok(Box::new(PortablePtyChild {
            child,
            stdout: Some(stdout),
            stdin,
        }))
    }

    fn resize(&self, id: &str, size: TerminalSize) -> Result<(), TerminalServiceError> {
        let master = {
            let masters = self
                .masters
                .lock()
                .expect("terminal pty masters lock poisoned");
            let Some(master) = masters.get(id) else {
                return Ok(());
            };
            master.clone()
        };

        let resize_result = master
            .lock()
            .expect("terminal pty master lock poisoned")
            .resize(to_pty_size(size));

        resize_result.map_err(|error| {
            TerminalServiceError::backend(format!("failed to resize pty: {error}"))
        })
    }

    fn cleanup(&self, id: &str) -> Result<(), TerminalServiceError> {
        self.masters
            .lock()
            .expect("terminal pty masters lock poisoned")
            .remove(id);
        Ok(())
    }
}

#[cfg(feature = "terminal-pty")]
fn to_pty_size(size: TerminalSize) -> PtySize {
    PtySize {
        rows: size.rows,
        cols: size.cols,
        pixel_width: 0,
        pixel_height: 0,
    }
}

#[cfg(feature = "terminal-pty")]
struct PortablePtyChild {
    child: Box<dyn PtyChild + Send + Sync>,
    stdout: Option<Box<dyn Read + Send>>,
    stdin: Box<dyn Write + Send>,
}

#[cfg(feature = "terminal-pty")]
impl TerminalChild for PortablePtyChild {
    fn kill(&mut self) -> Result<(), TerminalServiceError> {
        self.child.kill().map_err(|error| {
            TerminalServiceError::backend(format!("failed to kill terminal pty: {error}"))
        })
    }

    fn write_input(&mut self, data: &[u8]) -> Result<(), TerminalServiceError> {
        self.stdin.write_all(data).map_err(|error| {
            TerminalServiceError::backend(format!("failed to write terminal pty input: {error}"))
        })
    }

    fn take_stdout(&mut self) -> Option<Box<dyn Read + Send>> {
        self.stdout.take()
    }

    fn take_stderr(&mut self) -> Option<Box<dyn Read + Send>> {
        None
    }
}

pub struct DefaultTerminalBackend {
    #[cfg(feature = "terminal-pty")]
    pty: PortablePtyBackend,
    process: PortableProcessBackend,
}

impl Default for DefaultTerminalBackend {
    fn default() -> Self {
        Self {
            #[cfg(feature = "terminal-pty")]
            pty: PortablePtyBackend::default(),
            process: PortableProcessBackend,
        }
    }
}

impl TerminalProcessBackend for DefaultTerminalBackend {
    fn spawn(
        &self,
        request: BackendSpawnRequest,
    ) -> Result<Box<dyn TerminalChild>, TerminalServiceError> {
        #[cfg(feature = "terminal-pty")]
        match self.pty.spawn(request.clone()) {
            Ok(child) => Ok(child),
            Err(error) => {
                tracing::warn!(
                    terminal_id = request.id.as_str(),
                    code = error.code,
                    message = error.message.as_str(),
                    "pty unavailable; falling back to process terminal backend"
                );
                self.process.spawn(request)
            }
        }

        #[cfg(not(feature = "terminal-pty"))]
        self.process.spawn(request)
    }

    fn resize(&self, id: &str, size: TerminalSize) -> Result<(), TerminalServiceError> {
        #[cfg(feature = "terminal-pty")]
        return self.pty.resize(id, size);

        #[cfg(not(feature = "terminal-pty"))]
        {
            let _ = (id, size);
            Ok(())
        }
    }

    fn cleanup(&self, id: &str) -> Result<(), TerminalServiceError> {
        #[cfg(feature = "terminal-pty")]
        return self.pty.cleanup(id);

        #[cfg(not(feature = "terminal-pty"))]
        {
            let _ = id;
            Ok(())
        }
    }
}

struct PortableTerminalChild {
    child: Child,
}

impl TerminalChild for PortableTerminalChild {
    fn kill(&mut self) -> Result<(), TerminalServiceError> {
        self.child.kill().map_err(|error| {
            TerminalServiceError::backend(format!("failed to kill terminal process: {error}"))
        })
    }

    fn write_input(&mut self, data: &[u8]) -> Result<(), TerminalServiceError> {
        let stdin = self.child.stdin.as_mut().ok_or_else(|| {
            TerminalServiceError::backend("terminal process stdin is unavailable")
        })?;

        stdin.write_all(data).map_err(|error| {
            TerminalServiceError::backend(format!(
                "failed to write terminal process input: {error}"
            ))
        })
    }

    fn take_stdout(&mut self) -> Option<Box<dyn Read + Send>> {
        self.child
            .stdout
            .take()
            .map(|stdout| Box::new(stdout) as Box<dyn Read + Send>)
    }

    fn take_stderr(&mut self) -> Option<Box<dyn Read + Send>> {
        self.child
            .stderr
            .take()
            .map(|stderr| Box::new(stderr) as Box<dyn Read + Send>)
    }
}

struct TerminalSession {
    id: TerminalId,
    state: TerminalLifecycleState,
    size: Option<TerminalSize>,
    child: Box<dyn TerminalChild>,
    events: Receiver<TerminalStreamEvent>,
    stream_broadcast: broadcast::Sender<TerminalStreamEvent>,
    lifecycle_broadcast: broadcast::Sender<TerminalLifecycleEvent>,
}

impl TerminalSession {
    fn status(&self) -> TerminalStatus {
        TerminalStatus {
            id: self.id.clone(),
            state: self.state,
            size: self.size,
        }
    }
}

pub struct RustTerminalService {
    backend: Arc<dyn TerminalProcessBackend>,
    sessions: Mutex<HashMap<TerminalId, TerminalSession>>,
    next_id: AtomicU64,
}

pub fn register_terminal_services(registry: &mut ServiceRegistry) {
    registry.register_terminal_service(Arc::new(RustTerminalService::default()));
}

impl Default for RustTerminalService {
    fn default() -> Self {
        Self::new(Arc::new(DefaultTerminalBackend::default()))
    }
}

impl RustTerminalService {
    pub fn new(backend: Arc<dyn TerminalProcessBackend>) -> Self {
        Self {
            backend,
            sessions: Mutex::new(HashMap::new()),
            next_id: AtomicU64::new(1),
        }
    }

    pub fn spawn(
        &self,
        mut request: SpawnTerminalRequest,
    ) -> Result<TerminalStatus, TerminalServiceError> {
        let started = Instant::now();
        let id = request.id.take().unwrap_or_else(|| {
            format!("terminal-{}", self.next_id.fetch_add(1, Ordering::Relaxed))
        });
        let trace_id = TraceId::for_command("terminal_spawn", &id);

        validate_command(&request.command)?;

        if self
            .sessions
            .lock()
            .expect("terminal sessions lock poisoned")
            .contains_key(&id)
        {
            return Err(TerminalServiceError::exists(&id));
        }

        request
            .env
            .insert(SHELL_INTEGRATION_ENV.to_string(), "1".to_string());

        let size = match (request.cols, request.rows) {
            (Some(cols), Some(rows)) => Some(validate_size(TerminalSize { cols, rows })?),
            (None, None) => None,
            _ => {
                return Err(TerminalServiceError::invalid(
                    "terminal size requires both cols and rows",
                ))
            }
        };

        let backend_request = BackendSpawnRequest {
            id: id.clone(),
            command: request.command,
            args: request.args,
            cwd: request.cwd,
            env: request.env,
            size,
        };

        let mut child = match self.backend.spawn(backend_request) {
            Ok(child) => child,
            Err(error) => {
                record_metric(&metric_event(
                    trace_id,
                    TERMINAL_SPAWN_MS_METRIC,
                    started.elapsed().as_secs_f64() * 1000.0,
                    MetricUnit::Milliseconds,
                    Some(serde_json::json!({ "terminalId": id.as_str(), "success": false })),
                ));
                return Err(error);
            }
        };
        let (event_sender, events) = sync_channel(TERMINAL_STREAM_BUFFER);
        let (stream_broadcast, _) = broadcast::channel(TERMINAL_BROADCAST_BUFFER);
        let (lifecycle_broadcast, _) = broadcast::channel(TERMINAL_BROADCAST_BUFFER);
        spawn_stream_router(
            &id,
            TerminalStreamKind::Stdout,
            child.take_stdout(),
            event_sender.clone(),
            stream_broadcast.clone(),
        );
        spawn_stream_router(
            &id,
            TerminalStreamKind::Stderr,
            child.take_stderr(),
            event_sender.clone(),
            stream_broadcast.clone(),
        );

        if self
            .sessions
            .lock()
            .expect("terminal sessions lock poisoned")
            .contains_key(&id)
        {
            let _ = child.kill();
            let _ = self.backend.cleanup(&id);
            return Err(TerminalServiceError::exists(&id));
        }

        let session = TerminalSession {
            id: id.clone(),
            state: TerminalLifecycleState::Running,
            size,
            child,
            events,
            stream_broadcast,
            lifecycle_broadcast,
        };
        let status = session.status();

        self.sessions
            .lock()
            .expect("terminal sessions lock poisoned")
            .insert(id.clone(), session);

        record_metric(&metric_event(
            trace_id,
            TERMINAL_SPAWN_MS_METRIC,
            started.elapsed().as_secs_f64() * 1000.0,
            MetricUnit::Milliseconds,
            Some(serde_json::json!({ "terminalId": status.id.as_str(), "success": true })),
        ));

        Ok(status)
    }

    pub fn resize(
        &self,
        request: ResizeTerminalRequest,
    ) -> Result<TerminalStatus, TerminalServiceError> {
        let size = TerminalSize {
            cols: request.cols,
            rows: request.rows,
        };
        let size = validate_size(size)?;

        let should_resize_backend = {
            let sessions = self
                .sessions
                .lock()
                .expect("terminal sessions lock poisoned");
            let session = sessions
                .get(&request.id)
                .ok_or_else(|| TerminalServiceError::missing(&request.id))?;
            matches!(
                session.state,
                TerminalLifecycleState::Created
                    | TerminalLifecycleState::Launching
                    | TerminalLifecycleState::Running
                    | TerminalLifecycleState::Reconnected
            )
        };

        if should_resize_backend {
            self.backend.resize(&request.id, size)?;
        }

        let mut sessions = self
            .sessions
            .lock()
            .expect("terminal sessions lock poisoned");
        let session = sessions
            .get_mut(&request.id)
            .ok_or_else(|| TerminalServiceError::missing(&request.id))?;
        session.size = Some(size);
        emit_lifecycle(session);

        Ok(session.status())
    }

    pub fn kill(
        &self,
        request: KillTerminalRequest,
    ) -> Result<TerminalStatus, TerminalServiceError> {
        let mut sessions = self
            .sessions
            .lock()
            .expect("terminal sessions lock poisoned");
        let session = sessions
            .get_mut(&request.id)
            .ok_or_else(|| TerminalServiceError::missing(&request.id))?;
        session.child.kill()?;
        self.backend.cleanup(&request.id)?;
        session.state = TerminalLifecycleState::Exited;
        emit_lifecycle(session);

        Ok(session.status())
    }

    pub fn write(
        &self,
        request: WriteTerminalRequest,
    ) -> Result<TerminalStatus, TerminalServiceError> {
        let mut sessions = self
            .sessions
            .lock()
            .expect("terminal sessions lock poisoned");
        let session = sessions
            .get_mut(&request.id)
            .ok_or_else(|| TerminalServiceError::missing(&request.id))?;

        if !matches!(
            session.state,
            TerminalLifecycleState::Created
                | TerminalLifecycleState::Launching
                | TerminalLifecycleState::Running
                | TerminalLifecycleState::Reconnected
        ) {
            return Err(TerminalServiceError::invalid(format!(
                "terminal is not accepting input: {}",
                request.id
            )));
        }

        session.child.write_input(request.data.as_bytes())?;

        Ok(session.status())
    }

    pub fn reconnect(&self, id: &str) -> Result<TerminalStatus, TerminalServiceError> {
        let mut sessions = self
            .sessions
            .lock()
            .expect("terminal sessions lock poisoned");
        let session = sessions
            .get_mut(id)
            .ok_or_else(|| TerminalServiceError::missing(id))?;
        session.state = TerminalLifecycleState::Reconnected;
        emit_lifecycle(session);

        Ok(session.status())
    }

    pub fn dispose(&self, id: &str) -> Result<TerminalStatus, TerminalServiceError> {
        let mut sessions = self
            .sessions
            .lock()
            .expect("terminal sessions lock poisoned");
        let mut session = sessions
            .remove(id)
            .ok_or_else(|| TerminalServiceError::missing(id))?;
        let _ = session.child.kill();
        self.backend.cleanup(id)?;
        session.state = TerminalLifecycleState::Disposed;
        emit_lifecycle(&session);

        Ok(session.status())
    }

    pub fn drain_events(&self, id: &str) -> Result<Vec<TerminalStreamEvent>, TerminalServiceError> {
        let mut sessions = self
            .sessions
            .lock()
            .expect("terminal sessions lock poisoned");
        let session = sessions
            .get_mut(id)
            .ok_or_else(|| TerminalServiceError::missing(id))?;

        let mut drained = Vec::new();
        loop {
            match session.events.try_recv() {
                Ok(event) => drained.push(event),
                Err(TryRecvError::Empty | TryRecvError::Disconnected) => return Ok(drained),
            }
        }
    }

    pub fn status(&self, id: &str) -> Result<TerminalStatus, TerminalServiceError> {
        let sessions = self
            .sessions
            .lock()
            .expect("terminal sessions lock poisoned");
        sessions
            .get(id)
            .map(TerminalSession::status)
            .ok_or_else(|| TerminalServiceError::missing(id))
    }

    fn decode_arg<T: for<'de> Deserialize<'de>>(
        request: &ChannelCallRequest,
    ) -> Result<T, TerminalServiceError> {
        let value = request.args.first().ok_or_else(|| {
            TerminalServiceError::invalid(format!("{} requires one argument", request.command))
        })?;
        serde_json::from_value(value.clone()).map_err(|error| {
            TerminalServiceError::invalid(format!("invalid {} payload: {error}", request.command))
        })
    }

    fn required_id(request: &ChannelCallRequest) -> Result<&str, TerminalServiceError> {
        request.args.first().and_then(Value::as_str).ok_or_else(|| {
            TerminalServiceError::invalid(format!("{} requires terminal id", request.command))
        })
    }

    fn listen_id(request: &ChannelListenRequest) -> Result<TerminalId, TerminalServiceError> {
        let value = request.args.first().ok_or_else(|| {
            TerminalServiceError::invalid(format!("{} requires terminal id", request.event))
        })?;

        if let Some(id) = value.as_str() {
            return Ok(id.to_string());
        }

        if let Some(id) = value.get("id").and_then(Value::as_str) {
            return Ok(id.to_string());
        }

        Err(TerminalServiceError::invalid(format!(
            "{} requires terminal id",
            request.event
        )))
    }
}

fn emit_lifecycle(session: &TerminalSession) {
    let _ = session
        .lifecycle_broadcast
        .send(TerminalLifecycleEvent::from(session.status()));
}

fn validate_command(command: &str) -> Result<(), TerminalServiceError> {
    if command.trim().is_empty() {
        Err(TerminalServiceError::invalid(
            "terminal command is required",
        ))
    } else {
        Ok(())
    }
}

fn validate_size(size: TerminalSize) -> Result<TerminalSize, TerminalServiceError> {
    if size.cols == 0 || size.rows == 0 {
        Err(TerminalServiceError::invalid(
            "terminal size cols and rows must be greater than zero",
        ))
    } else {
        Ok(size)
    }
}

fn serialize_terminal_value<T: Serialize>(value: T) -> Result<Value, ServiceError> {
    serde_json::to_value(value).map_err(|error| ServiceError {
        code: "terminal.serializationFailed",
        message: error.to_string(),
    })
}

#[async_trait]
impl PlatformService for RustTerminalService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Terminal
    }

    async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
        match request.command.as_str() {
            "spawn" => serialize_terminal_value(self.spawn(Self::decode_arg(&request)?)?),
            "resize" => serialize_terminal_value(self.resize(Self::decode_arg(&request)?)?),
            "write" => serialize_terminal_value(self.write(Self::decode_arg(&request)?)?),
            "kill" => serialize_terminal_value(self.kill(Self::decode_arg(&request)?)?),
            "reconnect" => {
                let id = Self::required_id(&request)?;
                serialize_terminal_value(self.reconnect(id)?)
            }
            "dispose" => {
                let id = Self::required_id(&request)?;
                serialize_terminal_value(self.dispose(id)?)
            }
            "drainEvents" => {
                let id = Self::required_id(&request)?;
                serialize_terminal_value(self.drain_events(id)?)
            }
            "status" => {
                let id = Self::required_id(&request)?;
                serialize_terminal_value(self.status(id)?)
            }
            _ => Err(ServiceError::unsupported(
                ServiceId::Terminal,
                &request.command,
            )),
        }
    }

    async fn listen(
        &self,
        request: ChannelListenRequest,
        sink: EventSink,
    ) -> Result<SubscriptionHandle, ServiceError> {
        let id = Self::listen_id(&request)?;
        match request.event.as_str() {
            "stdout" => self.listen_stream(&id, Some(TerminalStreamKind::Stdout), sink),
            "stderr" => self.listen_stream(&id, Some(TerminalStreamKind::Stderr), sink),
            "stream" => self.listen_stream(&id, None, sink),
            "lifecycle" => self.listen_lifecycle(&id, sink),
            _ => Err(ServiceError::unsupported_listen(
                ServiceId::Terminal,
                &request.event,
            )),
        }
    }
}

impl TerminalService for RustTerminalService {}

fn spawn_stream_router(
    id: &str,
    stream: TerminalStreamKind,
    reader: Option<Box<dyn Read + Send>>,
    sender: SyncSender<TerminalStreamEvent>,
    broadcast_sender: broadcast::Sender<TerminalStreamEvent>,
) {
    let Some(reader) = reader else {
        return;
    };
    let id = id.to_string();

    thread::spawn(move || {
        let mut reader = BufReader::new(reader);
        let mut buffer = [0_u8; 4096];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => return,
                Ok(count) => {
                    let data = String::from_utf8_lossy(&buffer[..count]).to_string();
                    let event = TerminalStreamEvent {
                        id: id.clone(),
                        stream,
                        data,
                    };
                    let _ = broadcast_sender.send(event.clone());
                    match sender.try_send(event) {
                        Ok(()) => {}
                        Err(TrySendError::Full(_)) => {}
                        Err(TrySendError::Disconnected(_)) => return,
                    }
                }
                Err(_) => return,
            }
        }
    });
}

impl RustTerminalService {
    fn listen_stream(
        &self,
        id: &str,
        stream_filter: Option<TerminalStreamKind>,
        sink: EventSink,
    ) -> Result<SubscriptionHandle, ServiceError> {
        let mut receiver = {
            let sessions = self
                .sessions
                .lock()
                .expect("terminal sessions lock poisoned");
            sessions
                .get(id)
                .ok_or_else(|| TerminalServiceError::missing(id))?
                .stream_broadcast
                .subscribe()
        };
        let cancelled = Arc::new(AtomicBool::new(false));
        let task_cancelled = cancelled.clone();

        tokio::spawn(async move {
            while !task_cancelled.load(Ordering::SeqCst) {
                match receiver.recv().await {
                    Ok(event) if stream_filter.map_or(true, |filter| filter == event.stream) => {
                        let payload = match serde_json::to_value(event) {
                            Ok(payload) => payload,
                            Err(error) => {
                                tracing::warn!(?error, "failed to serialize terminal stream event");
                                continue;
                            }
                        };
                        if sink.send(Some(payload)).await.is_err() {
                            return;
                        }
                    }
                    Ok(_) => {}
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => return,
                }
            }
        });

        Ok(SubscriptionHandle::new(move || {
            cancelled.store(true, Ordering::SeqCst);
        }))
    }

    fn listen_lifecycle(
        &self,
        id: &str,
        sink: EventSink,
    ) -> Result<SubscriptionHandle, ServiceError> {
        let (mut receiver, current_status) = {
            let sessions = self
                .sessions
                .lock()
                .expect("terminal sessions lock poisoned");
            let session = sessions
                .get(id)
                .ok_or_else(|| TerminalServiceError::missing(id))?;
            (session.lifecycle_broadcast.subscribe(), session.status())
        };
        let cancelled = Arc::new(AtomicBool::new(false));
        let task_cancelled = cancelled.clone();

        tokio::spawn(async move {
            if let Ok(payload) = serde_json::to_value(TerminalLifecycleEvent::from(current_status))
            {
                if sink.send(Some(payload)).await.is_err() {
                    return;
                }
            }

            while !task_cancelled.load(Ordering::SeqCst) {
                match receiver.recv().await {
                    Ok(event) => {
                        let payload = match serde_json::to_value(event) {
                            Ok(payload) => payload,
                            Err(error) => {
                                tracing::warn!(
                                    ?error,
                                    "failed to serialize terminal lifecycle event"
                                );
                                continue;
                            }
                        };
                        if sink.send(Some(payload)).await.is_err() {
                            return;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => return,
                }
            }
        });

        Ok(SubscriptionHandle::new(move || {
            cancelled.store(true, Ordering::SeqCst);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::{io::Cursor, sync::Mutex, thread, time::Duration};

    #[derive(Default)]
    struct MockBackend {
        spawns: Mutex<Vec<BackendSpawnRequest>>,
        resizes: Mutex<Vec<(TerminalId, TerminalSize)>>,
        cleanups: Mutex<Vec<TerminalId>>,
    }

    impl TerminalProcessBackend for MockBackend {
        fn spawn(
            &self,
            request: BackendSpawnRequest,
        ) -> Result<Box<dyn TerminalChild>, TerminalServiceError> {
            self.spawns
                .lock()
                .expect("mock spawns lock poisoned")
                .push(request);
            Ok(Box::new(MockChild::default()))
        }

        fn resize(&self, id: &str, size: TerminalSize) -> Result<(), TerminalServiceError> {
            self.resizes
                .lock()
                .expect("mock resizes lock poisoned")
                .push((id.to_string(), size));
            Ok(())
        }

        fn cleanup(&self, id: &str) -> Result<(), TerminalServiceError> {
            self.cleanups
                .lock()
                .expect("mock cleanups lock poisoned")
                .push(id.to_string());
            Ok(())
        }
    }

    #[derive(Default)]
    struct MockChild {
        killed: bool,
    }

    impl TerminalChild for MockChild {
        fn kill(&mut self) -> Result<(), TerminalServiceError> {
            self.killed = true;
            Ok(())
        }

        fn write_input(&mut self, _data: &[u8]) -> Result<(), TerminalServiceError> {
            Ok(())
        }

        fn take_stdout(&mut self) -> Option<Box<dyn Read + Send>> {
            Some(Box::new(Cursor::new(Vec::<u8>::new())))
        }

        fn take_stderr(&mut self) -> Option<Box<dyn Read + Send>> {
            Some(Box::new(Cursor::new(Vec::<u8>::new())))
        }
    }

    #[test]
    fn spawn_injects_shell_integration_env_and_enters_running() {
        let backend = Arc::new(MockBackend::default());
        let service = RustTerminalService::new(backend.clone());

        let status = service
            .spawn(SpawnTerminalRequest {
                id: Some("term-1".to_string()),
                command: "shell".to_string(),
                args: vec!["--login".to_string()],
                cwd: Some("/tmp".to_string()),
                env: HashMap::from([("PATH".to_string(), "/bin".to_string())]),
                cols: Some(120),
                rows: Some(30),
            })
            .expect("spawn succeeds");

        assert_eq!(status.id, "term-1");
        assert_eq!(status.state, TerminalLifecycleState::Running);
        assert_eq!(
            status.size,
            Some(TerminalSize {
                cols: 120,
                rows: 30
            })
        );

        let spawns = backend.spawns.lock().expect("mock spawns lock poisoned");
        assert_eq!(
            spawns[0].env.get(SHELL_INTEGRATION_ENV),
            Some(&"1".to_string())
        );
        assert_eq!(spawns[0].env.get("PATH"), Some(&"/bin".to_string()));
    }

    #[test]
    fn resize_coalesces_to_latest_size() {
        let backend = Arc::new(MockBackend::default());
        let service = RustTerminalService::new(backend.clone());
        service
            .spawn(SpawnTerminalRequest {
                id: Some("term-1".to_string()),
                command: "shell".to_string(),
                args: Vec::new(),
                cwd: None,
                env: HashMap::new(),
                cols: None,
                rows: None,
            })
            .expect("spawn succeeds");

        service
            .resize(ResizeTerminalRequest {
                id: "term-1".to_string(),
                cols: 80,
                rows: 24,
            })
            .expect("first resize succeeds");
        let status = service
            .resize(ResizeTerminalRequest {
                id: "term-1".to_string(),
                cols: 132,
                rows: 43,
            })
            .expect("second resize succeeds");

        assert_eq!(
            status.size,
            Some(TerminalSize {
                cols: 132,
                rows: 43
            })
        );
        assert_eq!(
            service.status("term-1").expect("status succeeds").size,
            status.size
        );
    }

    #[test]
    fn resize_unknown_terminal_does_not_call_backend() {
        let backend = Arc::new(MockBackend::default());
        let service = RustTerminalService::new(backend.clone());

        let error = service
            .resize(ResizeTerminalRequest {
                id: "missing-term".to_string(),
                cols: 80,
                rows: 24,
            })
            .expect_err("missing terminal rejected");

        assert_eq!(error.code, "terminal.missing");
        assert!(backend
            .resizes
            .lock()
            .expect("mock resizes lock poisoned")
            .is_empty());
    }

    #[test]
    fn kill_cleans_backend_and_exited_resize_stays_cached() {
        let backend = Arc::new(MockBackend::default());
        let service = RustTerminalService::new(backend.clone());
        service
            .spawn(SpawnTerminalRequest {
                id: Some("term-1".to_string()),
                command: "shell".to_string(),
                args: Vec::new(),
                cwd: None,
                env: HashMap::new(),
                cols: Some(80),
                rows: Some(24),
            })
            .expect("spawn succeeds");

        service
            .kill(KillTerminalRequest {
                id: "term-1".to_string(),
            })
            .expect("kill succeeds");
        let status = service
            .resize(ResizeTerminalRequest {
                id: "term-1".to_string(),
                cols: 120,
                rows: 40,
            })
            .expect("exited resize succeeds");

        assert_eq!(status.state, TerminalLifecycleState::Exited);
        assert_eq!(
            status.size,
            Some(TerminalSize {
                cols: 120,
                rows: 40
            })
        );
        assert_eq!(
            *backend
                .cleanups
                .lock()
                .expect("mock cleanups lock poisoned"),
            vec!["term-1".to_string()]
        );
        assert!(backend
            .resizes
            .lock()
            .expect("mock resizes lock poisoned")
            .is_empty());
    }

    #[test]
    fn write_forwards_input_to_running_terminal() {
        let writes = Arc::new(Mutex::new(Vec::new()));
        let service = RustTerminalService::new(Arc::new(InputBackend {
            writes: writes.clone(),
        }));
        service
            .spawn(SpawnTerminalRequest {
                id: Some("term-1".to_string()),
                command: "shell".to_string(),
                args: Vec::new(),
                cwd: None,
                env: HashMap::new(),
                cols: None,
                rows: None,
            })
            .expect("spawn succeeds");

        let status = service
            .write(WriteTerminalRequest {
                id: "term-1".to_string(),
                data: "echo ok\n".to_string(),
            })
            .expect("write succeeds");

        assert_eq!(status.state, TerminalLifecycleState::Running);
        assert_eq!(
            *writes.lock().expect("writes lock poisoned"),
            b"echo ok\n".to_vec()
        );
    }

    #[test]
    fn write_rejects_exited_terminal() {
        let service = RustTerminalService::new(Arc::new(MockBackend::default()));
        service
            .spawn(SpawnTerminalRequest {
                id: Some("term-1".to_string()),
                command: "shell".to_string(),
                args: Vec::new(),
                cwd: None,
                env: HashMap::new(),
                cols: None,
                rows: None,
            })
            .expect("spawn succeeds");
        service
            .kill(KillTerminalRequest {
                id: "term-1".to_string(),
            })
            .expect("kill succeeds");

        let error = service
            .write(WriteTerminalRequest {
                id: "term-1".to_string(),
                data: "ignored".to_string(),
            })
            .expect_err("exited write rejected");

        assert_eq!(error.code, "terminal.invalidRequest");
        assert_eq!(error.message, "terminal is not accepting input: term-1");
    }

    #[test]
    fn spawn_rejects_duplicate_id_and_invalid_size() {
        let backend = Arc::new(MockBackend::default());
        let service = RustTerminalService::new(backend.clone());
        service
            .spawn(SpawnTerminalRequest {
                id: Some("term-1".to_string()),
                command: "shell".to_string(),
                args: Vec::new(),
                cwd: None,
                env: HashMap::new(),
                cols: Some(80),
                rows: Some(24),
            })
            .expect("first spawn succeeds");

        let duplicate = service
            .spawn(SpawnTerminalRequest {
                id: Some("term-1".to_string()),
                command: "shell".to_string(),
                args: Vec::new(),
                cwd: None,
                env: HashMap::new(),
                cols: Some(80),
                rows: Some(24),
            })
            .expect_err("duplicate id rejected");
        assert_eq!(duplicate.code, "terminal.exists");

        let invalid_size = service
            .spawn(SpawnTerminalRequest {
                id: Some("term-2".to_string()),
                command: "shell".to_string(),
                args: Vec::new(),
                cwd: None,
                env: HashMap::new(),
                cols: Some(0),
                rows: Some(24),
            })
            .expect_err("zero size rejected");
        assert_eq!(invalid_size.code, "terminal.invalidRequest");

        assert_eq!(
            backend
                .spawns
                .lock()
                .expect("mock spawns lock poisoned")
                .len(),
            1
        );
    }

    #[test]
    fn resize_rejects_zero_dimensions_before_backend_call() {
        let backend = Arc::new(MockBackend::default());
        let service = RustTerminalService::new(backend.clone());
        service
            .spawn(SpawnTerminalRequest {
                id: Some("term-1".to_string()),
                command: "shell".to_string(),
                args: Vec::new(),
                cwd: None,
                env: HashMap::new(),
                cols: None,
                rows: None,
            })
            .expect("spawn succeeds");

        let error = service
            .resize(ResizeTerminalRequest {
                id: "term-1".to_string(),
                cols: 120,
                rows: 0,
            })
            .expect_err("zero rows rejected");

        assert_eq!(error.code, "terminal.invalidRequest");
        assert!(backend
            .resizes
            .lock()
            .expect("mock resizes lock poisoned")
            .is_empty());
    }

    #[test]
    fn lifecycle_transitions_through_exit_reconnect_and_dispose() {
        let service = RustTerminalService::new(Arc::new(MockBackend::default()));
        service
            .spawn(SpawnTerminalRequest {
                id: Some("term-1".to_string()),
                command: "shell".to_string(),
                args: Vec::new(),
                cwd: None,
                env: HashMap::new(),
                cols: None,
                rows: None,
            })
            .expect("spawn succeeds");

        assert_eq!(
            service
                .kill(KillTerminalRequest {
                    id: "term-1".to_string(),
                })
                .expect("kill succeeds")
                .state,
            TerminalLifecycleState::Exited
        );
        assert_eq!(
            service
                .reconnect("term-1")
                .expect("reconnect succeeds")
                .state,
            TerminalLifecycleState::Reconnected
        );
        assert_eq!(
            service.dispose("term-1").expect("dispose succeeds").state,
            TerminalLifecycleState::Disposed
        );
        assert!(matches!(
            service.status("term-1"),
            Err(TerminalServiceError {
                code: "terminal.missing",
                ..
            })
        ));
    }

    #[test]
    fn drain_events_returns_stdout_and_stderr_chunks() {
        let service = RustTerminalService::new(Arc::new(OutputBackend));

        service
            .spawn(SpawnTerminalRequest {
                id: Some("term-1".to_string()),
                command: "shell".to_string(),
                args: Vec::new(),
                cwd: None,
                env: HashMap::new(),
                cols: None,
                rows: None,
            })
            .expect("spawn succeeds");

        let mut events = Vec::new();
        for _ in 0..20 {
            events.extend(service.drain_events("term-1").expect("drain succeeds"));
            if events.len() >= 2 {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }

        assert!(events.iter().any(
            |event| event.stream == TerminalStreamKind::Stdout && event.data == "stdout chunk"
        ));
        assert!(events.iter().any(
            |event| event.stream == TerminalStreamKind::Stderr && event.data == "stderr chunk"
        ));
    }

    #[tokio::test]
    async fn platform_call_dispatches_spawn_status_and_dispose() {
        let service = RustTerminalService::new(Arc::new(MockBackend::default()));

        let spawned = service
            .call(ChannelCallRequest {
                request_id: "req-spawn".to_string(),
                channel: "terminal".to_string(),
                command: "spawn".to_string(),
                args: vec![json!({
                    "id": "term-1",
                    "command": "shell",
                    "args": [],
                    "env": {},
                    "cols": 90,
                    "rows": 25
                })],
                cancellation_id: None,
            })
            .await
            .expect("spawn dispatch succeeds");
        assert_eq!(spawned["id"], "term-1");
        assert_eq!(spawned["state"], "running");

        let status = service
            .call(ChannelCallRequest {
                request_id: "req-status".to_string(),
                channel: "terminal".to_string(),
                command: "status".to_string(),
                args: vec![json!("term-1")],
                cancellation_id: None,
            })
            .await
            .expect("status dispatch succeeds");
        assert_eq!(status["size"], json!({ "cols": 90, "rows": 25 }));

        let disposed = service
            .call(ChannelCallRequest {
                request_id: "req-dispose".to_string(),
                channel: "terminal".to_string(),
                command: "dispose".to_string(),
                args: vec![json!("term-1")],
                cancellation_id: None,
            })
            .await
            .expect("dispose dispatch succeeds");
        assert_eq!(disposed["state"], "disposed");
    }

    #[tokio::test]
    async fn listen_stdout_publishes_matching_stream_events() {
        let service = RustTerminalService::new(Arc::new(MockBackend::default()));
        service
            .spawn(SpawnTerminalRequest {
                id: Some("term-1".to_string()),
                command: "shell".to_string(),
                args: Vec::new(),
                cwd: None,
                env: HashMap::new(),
                cols: None,
                rows: None,
            })
            .expect("spawn succeeds");
        let (sink, mut events) = tokio::sync::mpsc::channel(1);

        let handle = service
            .listen(
                ChannelListenRequest {
                    request_id: "req-1".to_string(),
                    channel: "terminal".to_string(),
                    event: "stdout".to_string(),
                    args: vec![json!({ "id": "term-1" })],
                    subscription_id: None,
                },
                sink,
            )
            .await
            .expect("listen succeeds");

        {
            let sessions = service.sessions.lock().expect("sessions lock poisoned");
            let session = sessions.get("term-1").expect("session exists");
            let _ = session.stream_broadcast.send(TerminalStreamEvent {
                id: "term-1".to_string(),
                stream: TerminalStreamKind::Stderr,
                data: "ignored".to_string(),
            });
            let _ = session.stream_broadcast.send(TerminalStreamEvent {
                id: "term-1".to_string(),
                stream: TerminalStreamKind::Stdout,
                data: "stdout chunk".to_string(),
            });
        }

        let payload = events.recv().await.expect("stdout payload sent").unwrap();
        assert_eq!(payload["id"], "term-1");
        assert_eq!(payload["stream"], "stdout");
        assert_eq!(payload["data"], "stdout chunk");
        handle.cancel();
    }

    #[tokio::test]
    async fn listen_lifecycle_sends_initial_and_transition_events() {
        let service = RustTerminalService::new(Arc::new(MockBackend::default()));
        service
            .spawn(SpawnTerminalRequest {
                id: Some("term-1".to_string()),
                command: "shell".to_string(),
                args: Vec::new(),
                cwd: None,
                env: HashMap::new(),
                cols: Some(80),
                rows: Some(24),
            })
            .expect("spawn succeeds");
        let (sink, mut events) = tokio::sync::mpsc::channel(2);

        let handle = service
            .listen(
                ChannelListenRequest {
                    request_id: "req-1".to_string(),
                    channel: "terminal".to_string(),
                    event: "lifecycle".to_string(),
                    args: vec![json!("term-1")],
                    subscription_id: None,
                },
                sink,
            )
            .await
            .expect("listen succeeds");

        let initial = events.recv().await.expect("initial event sent").unwrap();
        assert_eq!(initial["state"], "running");
        service
            .kill(KillTerminalRequest {
                id: "term-1".to_string(),
            })
            .expect("kill succeeds");
        let exited = events.recv().await.expect("exit event sent").unwrap();
        assert_eq!(exited["id"], "term-1");
        assert_eq!(exited["state"], "exited");
        handle.cancel();
    }

    #[tokio::test]
    async fn listen_rejects_unknown_terminal_event() {
        let service = RustTerminalService::new(Arc::new(MockBackend::default()));
        let (sink, _events) = tokio::sync::mpsc::channel(1);

        let error = service
            .listen(
                ChannelListenRequest {
                    request_id: "req-1".to_string(),
                    channel: "terminal".to_string(),
                    event: "pty".to_string(),
                    args: vec![json!("term-1")],
                    subscription_id: None,
                },
                sink,
            )
            .await
            .expect_err("unsupported listen rejected");

        assert_eq!(error.code, "service.unsupportedListen");
        assert_eq!(
            error.message,
            "terminalService does not implement event: pty"
        );
    }

    struct OutputBackend;

    impl TerminalProcessBackend for OutputBackend {
        fn spawn(
            &self,
            _request: BackendSpawnRequest,
        ) -> Result<Box<dyn TerminalChild>, TerminalServiceError> {
            Ok(Box::new(OutputChild))
        }
    }

    struct OutputChild;

    impl TerminalChild for OutputChild {
        fn kill(&mut self) -> Result<(), TerminalServiceError> {
            Ok(())
        }

        fn write_input(&mut self, _data: &[u8]) -> Result<(), TerminalServiceError> {
            Ok(())
        }

        fn take_stdout(&mut self) -> Option<Box<dyn Read + Send>> {
            Some(Box::new(Cursor::new(b"stdout chunk".to_vec())))
        }

        fn take_stderr(&mut self) -> Option<Box<dyn Read + Send>> {
            Some(Box::new(Cursor::new(b"stderr chunk".to_vec())))
        }
    }

    struct InputBackend {
        writes: Arc<Mutex<Vec<u8>>>,
    }

    impl TerminalProcessBackend for InputBackend {
        fn spawn(
            &self,
            _request: BackendSpawnRequest,
        ) -> Result<Box<dyn TerminalChild>, TerminalServiceError> {
            Ok(Box::new(InputChild {
                writes: self.writes.clone(),
            }))
        }
    }

    struct InputChild {
        writes: Arc<Mutex<Vec<u8>>>,
    }

    impl TerminalChild for InputChild {
        fn kill(&mut self) -> Result<(), TerminalServiceError> {
            Ok(())
        }

        fn write_input(&mut self, data: &[u8]) -> Result<(), TerminalServiceError> {
            self.writes
                .lock()
                .expect("writes lock poisoned")
                .extend_from_slice(data);
            Ok(())
        }

        fn take_stdout(&mut self) -> Option<Box<dyn Read + Send>> {
            Some(Box::new(Cursor::new(Vec::<u8>::new())))
        }

        fn take_stderr(&mut self) -> Option<Box<dyn Read + Send>> {
            Some(Box::new(Cursor::new(Vec::<u8>::new())))
        }
    }
}
