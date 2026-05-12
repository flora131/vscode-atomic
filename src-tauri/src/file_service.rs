use std::{
    collections::HashMap,
    io::ErrorKind,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex as StdMutex,
    },
    time::SystemTime,
};

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use notify::{
    event::{ModifyKind, RenameMode},
    EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::{fs, io::AsyncWriteExt, sync::Mutex};

use crate::{
    contracts::{
        ChannelCallRequest, FileDeleteRequest, FileDirEntryDto, FileMkdirRequest, FileReadRequest,
        FileReadResponse, FileReaddirRequest, FileStatDto, FileStatRequest, FileTypeDto,
        FileWriteRequest, UriDto,
    },
    observability::{
        metric_event, record_metric, MetricUnit, TraceId, CHANNEL_EVENT_DROPPED_METRIC,
        FILE_WATCH_ACTIVE_METRIC, FILE_WRITE_FAILURES_METRIC,
    },
    service_registry::{FileService, PlatformService, ServiceError, ServiceId, WatcherService},
    subscription_manager::{EventSink, SubscriptionHandle},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WatchId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileWatchRequest {
    pub resource: UriDto,
    #[serde(default)]
    pub recursive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileWatchResponse {
    pub watch_id: WatchId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUnwatchRequest {
    pub watch_id: WatchId,
}

pub const FILE_CHANGE_TYPE_UPDATED: u8 = 0;
pub const FILE_CHANGE_TYPE_ADDED: u8 = 1;
pub const FILE_CHANGE_TYPE_DELETED: u8 = 2;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileChangeDto {
    pub resource: UriDto,
    pub r#type: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileWatchEventDto {
    pub changes: Vec<FileChangeDto>,
    pub kind: String,
}

#[derive(Default)]
pub struct TauriFileService {
    watches: Mutex<HashMap<String, RecommendedWatcher>>,
    next_watch_id: AtomicU64,
}

pub struct TauriWatcherService {
    files: Arc<TauriFileService>,
}

impl TauriWatcherService {
    pub fn new(files: Arc<TauriFileService>) -> Arc<Self> {
        Arc::new(Self { files })
    }
}

pub fn register_file_watcher_services(
    registry: &mut crate::service_registry::ServiceRegistry,
) -> Arc<TauriFileService> {
    let files = TauriFileService::new();
    registry.register_file_service(files.clone());
    registry.register_watcher_service(TauriWatcherService::new(files.clone()));
    files
}

impl TauriFileService {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub async fn stat(&self, request: FileStatRequest) -> Result<FileStatDto, ServiceError> {
        let path = path_from_uri(&request.resource)?;
        let metadata = fs::symlink_metadata(&path)
            .await
            .map_err(|error| map_io_error("fs.statFailed", "stat", &path, error))?;

        Ok(stat_from_metadata(request.resource, metadata))
    }

    pub async fn read_file(
        &self,
        request: FileReadRequest,
    ) -> Result<FileReadResponse, ServiceError> {
        let path = path_from_uri(&request.resource)?;
        reject_directory_read(&path).await?;
        let data = fs::read(&path)
            .await
            .map_err(|error| map_io_error("fs.readFailed", "read", &path, error))?;

        Ok(FileReadResponse {
            data_base64: STANDARD.encode(data),
        })
    }

    pub async fn write_file(&self, request: FileWriteRequest) -> Result<FileStatDto, ServiceError> {
        let path = path_from_uri(&request.resource)?;
        let trace_id = TraceId::for_command("fs_write_file", write_path_class(&path));
        let data = match STANDARD.decode(&request.data_base64) {
            Ok(data) => data,
            Err(error) => {
                let error = file_error(
                    "fs.invalidData",
                    format!("data_base64 is not valid base64: {error}"),
                );
                record_write_failure_metric(&trace_id, &path, error.code);
                return Err(error);
            }
        };

        let mut file = match open_write_target(&path, request.create, request.overwrite).await {
            Ok(file) => file,
            Err(error) => {
                record_write_failure_metric(&trace_id, &path, error.code);
                return Err(error);
            }
        };

        if let Err(error) = file.write_all(&data).await {
            let error = map_io_error("fs.writeFailed", "write", &path, error);
            record_write_failure_metric(&trace_id, &path, error.code);
            return Err(error);
        }
        if let Err(error) = file.flush().await {
            let error = map_io_error("fs.writeFailed", "flush", &path, error);
            record_write_failure_metric(&trace_id, &path, error.code);
            return Err(error);
        }
        drop(file);

        self.stat(FileStatRequest {
            resource: request.resource,
        })
        .await
    }

    pub async fn delete(&self, request: FileDeleteRequest) -> Result<(), ServiceError> {
        let path = path_from_uri(&request.resource)?;
        if request.use_trash {
            return trash_delete(&path).await;
        }

        let metadata = fs::symlink_metadata(&path).await.map_err(|error| {
            if error.kind() == ErrorKind::NotFound {
                file_error(
                    "fs.fileNotFound",
                    format!("file not found: {}", path.display()),
                )
            } else {
                map_io_error("fs.deleteFailed", "stat before delete", &path, error)
            }
        })?;

        if metadata.file_type().is_dir() {
            if request.recursive {
                fs::remove_dir_all(&path).await
            } else {
                fs::remove_dir(&path).await
            }
        } else {
            fs::remove_file(&path).await
        }
        .map_err(|error| map_io_error("fs.deleteFailed", "delete", &path, error))
    }

    pub async fn mkdir(&self, request: FileMkdirRequest) -> Result<FileStatDto, ServiceError> {
        let path = path_from_uri(&request.resource)?;
        fs::create_dir_all(&path)
            .await
            .map_err(|error| map_io_error("fs.mkdirFailed", "create directory", &path, error))?;

        self.stat(FileStatRequest {
            resource: request.resource,
        })
        .await
    }

    pub async fn readdir(
        &self,
        request: FileReaddirRequest,
    ) -> Result<Vec<FileDirEntryDto>, ServiceError> {
        let path = path_from_uri(&request.resource)?;
        let mut entries = fs::read_dir(&path)
            .await
            .map_err(|error| map_io_error("fs.readdirFailed", "read directory", &path, error))?;

        let mut result = Vec::new();
        while let Some(entry) = entries.next_entry().await.map_err(|error| {
            map_io_error("fs.readdirFailed", "read directory entry in", &path, error)
        })? {
            let name = entry.file_name().to_string_lossy().to_string();
            let entry_path = entry.path();
            let metadata = fs::symlink_metadata(&entry_path).await.map_err(|error| {
                map_io_error(
                    "fs.readdirFailed",
                    "stat directory entry",
                    &entry_path,
                    error,
                )
            })?;
            result.push(FileDirEntryDto {
                name,
                r#type: file_type_from_metadata(&metadata),
            });
        }

        result.sort_by(|left, right| left.name.cmp(&right.name));
        Ok(result)
    }

    pub async fn watch(
        &self,
        request: FileWatchRequest,
    ) -> Result<FileWatchResponse, ServiceError> {
        let path = path_from_uri(&request.resource)?;
        let watch_id = format!(
            "watch-{}",
            self.next_watch_id.fetch_add(1, Ordering::Relaxed) + 1
        );
        let mode = watch_mode(request.recursive);

        let mut watcher = notify::recommended_watcher(|result: notify::Result<notify::Event>| {
            if let Err(error) = result {
                tracing::warn!(%error, "file watch error");
            }
        })
        .map_err(|error| {
            file_error(
                "fs.watchFailed",
                format!("failed to create watcher: {error}"),
            )
        })?;

        watcher.watch(&path, mode).map_err(|error| {
            file_error(
                "fs.watchFailed",
                format!("failed to watch {}: {error}", path.display()),
            )
        })?;

        let active = {
            let mut watches = self.watches.lock().await;
            watches.insert(watch_id.clone(), watcher);
            watches.len()
        };
        record_metric(&metric_event(
            TraceId::for_command("file_watch", &watch_id),
            FILE_WATCH_ACTIVE_METRIC,
            active as f64,
            MetricUnit::Count,
            Some(json!({ "watchId": watch_id.as_str(), "recursive": request.recursive })),
        ));

        Ok(FileWatchResponse {
            watch_id: WatchId(watch_id),
        })
    }

    pub async fn unwatch(&self, request: FileUnwatchRequest) -> Result<(), ServiceError> {
        let mut watches = self.watches.lock().await;
        let Some(watcher) = watches.remove(&request.watch_id.0) else {
            return Err(file_error(
                "fs.watchNotFound",
                format!("file watch not found: {}", request.watch_id.0),
            ));
        };
        let active = watches.len();
        drop(watches);

        drop(watcher);
        record_metric(&metric_event(
            TraceId::for_command("file_watch", &request.watch_id.0),
            FILE_WATCH_ACTIVE_METRIC,
            active as f64,
            MetricUnit::Count,
            Some(json!({ "watchId": request.watch_id.0.as_str(), "active": false })),
        ));
        Ok(())
    }

    pub async fn listen_watch(
        &self,
        request: FileWatchRequest,
        sink: EventSink,
    ) -> Result<SubscriptionHandle, ServiceError> {
        let path = path_from_uri(&request.resource)?;
        let watched_path = path.clone();
        let mode = watch_mode(request.recursive);
        let event_sink = WatchEventSink::new(sink);

        let callback_sink = event_sink.clone();
        let mut watcher =
            notify::recommended_watcher(move |result: notify::Result<notify::Event>| {
                if callback_sink.is_closed() {
                    return;
                }

                match result {
                    Ok(event) => {
                        let payload = watch_event_payload(event);
                        if watch_payload_has_changes(&payload) {
                            callback_sink.try_send(payload);
                        }
                    }
                    Err(error) => {
                        tracing::warn!(%error, "file watch error");
                    }
                }
            })
            .map_err(|error| {
                file_error(
                    "fs.watchFailed",
                    format!("failed to create watcher: {error}"),
                )
            })?;

        watcher.watch(&path, mode).map_err(|error| {
            file_error(
                "fs.watchFailed",
                format!("failed to watch {}: {error}", path.display()),
            )
        })?;

        record_metric(&metric_event(
            TraceId::for_command("file_watch", path.to_string_lossy().as_ref()),
            FILE_WATCH_ACTIVE_METRIC,
            1.0,
            MetricUnit::Count,
            Some(json!({ "recursive": request.recursive })),
        ));

        Ok(SubscriptionHandle::new(move || {
            event_sink.close();
            record_metric(&metric_event(
                TraceId::for_command("file_watch", watched_path.to_string_lossy().as_ref()),
                FILE_WATCH_ACTIVE_METRIC,
                0.0,
                MetricUnit::Count,
                Some(json!({ "active": false })),
            ));
            if let Err(error) = watcher.unwatch(&watched_path) {
                tracing::warn!(%error, path = %watched_path.display(), "failed to unwatch file path");
            }
            drop(watcher);
        }))
    }
}

fn record_write_failure_metric(trace_id: &TraceId, path: &Path, reason: &str) {
    record_metric(&metric_event(
        trace_id.clone(),
        FILE_WRITE_FAILURES_METRIC,
        1.0,
        MetricUnit::Count,
        Some(json!({
            "reason": reason,
            "pathClass": write_path_class(path),
            "commandSource": "fileService",
        })),
    ));
}

fn write_path_class(path: &Path) -> &'static str {
    if path.is_absolute() {
        "absolute"
    } else {
        "relative"
    }
}

#[derive(Clone)]
struct WatchEventSink {
    cancelled: Arc<AtomicBool>,
    sink: Arc<StdMutex<Option<EventSink>>>,
}

impl WatchEventSink {
    fn new(sink: EventSink) -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
            sink: Arc::new(StdMutex::new(Some(sink))),
        }
    }

    fn is_closed(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    fn try_send(&self, payload: Value) {
        if self.is_closed() {
            return;
        }

        let sink = self.sink.lock().expect("file watch sink lock poisoned");
        let Some(sink) = sink.as_ref() else {
            return;
        };

        if self.is_closed() {
            return;
        }

        if let Err(error) = sink.try_send(Some(payload)) {
            record_metric(&metric_event(
                TraceId::new("file-watch-dropped"),
                CHANNEL_EVENT_DROPPED_METRIC,
                1.0,
                MetricUnit::Count,
                Some(json!({ "error": error.to_string() })),
            ));
            tracing::warn!(%error, "failed to queue file watch event");
        }
    }

    fn close(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
        drop(
            self.sink
                .lock()
                .expect("file watch sink lock poisoned")
                .take(),
        );
    }
}

async fn open_write_target(
    path: &Path,
    create: bool,
    overwrite: bool,
) -> Result<fs::File, ServiceError> {
    match (create, overwrite) {
        (true, false) => open_write_options()
            .create_new(true)
            .open(path)
            .await
            .map_err(|error| {
                if error.kind() == ErrorKind::AlreadyExists {
                    file_error(
                        "fs.fileExists",
                        format!("file already exists: {}", path.display()),
                    )
                } else {
                    write_open_error(path, error)
                }
            }),
        (true, true) => open_write_options()
            .create(true)
            .truncate(true)
            .open(path)
            .await
            .map_err(|error| write_open_error(path, error)),
        (false, true) => {
            require_existing_file(path).await?;

            open_write_options()
                .truncate(true)
                .open(path)
                .await
                .map_err(|error| {
                    if error.kind() == ErrorKind::NotFound {
                        file_error(
                            "fs.fileNotFound",
                            format!("file not found: {}", path.display()),
                        )
                    } else {
                        write_open_error(path, error)
                    }
                })
        }
        (false, false) => match fs::symlink_metadata(path).await {
            Ok(_) => Err(file_error(
                "fs.fileExists",
                format!("file already exists: {}", path.display()),
            )),
            Err(error) if error.kind() == ErrorKind::NotFound => Err(file_error(
                "fs.fileNotFound",
                format!("file not found: {}", path.display()),
            )),
            Err(error) => Err(write_open_error(path, error)),
        },
    }
}

fn open_write_options() -> fs::OpenOptions {
    let mut options = fs::OpenOptions::new();
    options.write(true);
    options
}

fn watch_mode(recursive: bool) -> RecursiveMode {
    if recursive {
        RecursiveMode::Recursive
    } else {
        RecursiveMode::NonRecursive
    }
}

async fn trash_delete(path: &Path) -> Result<(), ServiceError> {
    fs::symlink_metadata(path).await.map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            file_error(
                "fs.fileNotFound",
                format!("file not found: {}", path.display()),
            )
        } else {
            map_io_error("fs.deleteFailed", "stat before trash", path, error)
        }
    })?;

    platform_trash_delete(path).await
}

#[cfg(unix)]
async fn platform_trash_delete(path: &Path) -> Result<(), ServiceError> {
    let trash_home = xdg_data_home()
        .map_err(|error| {
            file_error(
                "fs.trashFailed",
                format!("failed to resolve trash home: {error}"),
            )
        })?
        .join("Trash");
    let files_dir = trash_home.join("files");
    let info_dir = trash_home.join("info");
    fs::create_dir_all(&files_dir).await.map_err(|error| {
        map_io_error(
            "fs.trashFailed",
            "create trash files directory",
            &files_dir,
            error,
        )
    })?;
    fs::create_dir_all(&info_dir).await.map_err(|error| {
        map_io_error(
            "fs.trashFailed",
            "create trash info directory",
            &info_dir,
            error,
        )
    })?;

    let trash_name = unique_trash_name(path, &files_dir).await?;
    let trash_path = files_dir.join(&trash_name);
    let info_path = info_dir.join(format!("{trash_name}.trashinfo"));
    let info = trash_info_contents(path);

    fs::write(&info_path, info)
        .await
        .map_err(|error| map_io_error("fs.trashFailed", "write trash info", &info_path, error))?;

    match fs::rename(path, &trash_path).await {
        Ok(()) => Ok(()),
        Err(error) => {
            let _ = fs::remove_file(&info_path).await;
            Err(map_io_error("fs.trashFailed", "move to trash", path, error))
        }
    }
}

#[cfg(not(unix))]
async fn platform_trash_delete(path: &Path) -> Result<(), ServiceError> {
    Err(file_error(
        "fs.trashUnsupported",
        format!(
            "delete with useTrash is not supported on this platform: {}",
            path.display()
        ),
    ))
}

#[cfg(unix)]
fn xdg_data_home() -> Result<PathBuf, std::env::VarError> {
    if let Ok(value) = std::env::var("XDG_DATA_HOME") {
        if !value.is_empty() {
            return Ok(PathBuf::from(value));
        }
    }

    std::env::var("HOME").map(|home| PathBuf::from(home).join(".local/share"))
}

#[cfg(unix)]
async fn unique_trash_name(path: &Path, files_dir: &Path) -> Result<String, ServiceError> {
    let base_name = path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "deleted".to_string());

    for attempt in 0..1000_u32 {
        let name = if attempt == 0 {
            base_name.clone()
        } else {
            format!("{base_name}.{attempt}")
        };
        match fs::symlink_metadata(files_dir.join(&name)).await {
            Ok(_) => continue,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(name),
            Err(error) => {
                return Err(map_io_error(
                    "fs.trashFailed",
                    "stat trash candidate",
                    files_dir,
                    error,
                ))
            }
        }
    }

    Err(file_error(
        "fs.trashFailed",
        format!("failed to find unused trash name for {}", path.display()),
    ))
}

#[cfg(unix)]
fn trash_info_contents(path: &Path) -> String {
    let path = path.to_string_lossy();
    format!(
        "[Trash Info]\nPath={}\nDeletionDate=1970-01-01T00:00:00\n",
        percent_encode_uri_component(path.as_bytes())
    )
}

#[cfg(unix)]
fn percent_encode_uri_component(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len());
    for &byte in bytes {
        if matches!(
            byte,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' | b'/'
        ) {
            encoded.push(byte as char);
        } else {
            encoded.push('%');
            encoded.push(HEX[(byte >> 4) as usize] as char);
            encoded.push(HEX[(byte & 0x0f) as usize] as char);
        }
    }
    encoded
}

async fn require_existing_file(path: &Path) -> Result<(), ServiceError> {
    fs::symlink_metadata(path)
        .await
        .map(|_| ())
        .map_err(|error| {
            if error.kind() == ErrorKind::NotFound {
                file_error(
                    "fs.fileNotFound",
                    format!("file not found: {}", path.display()),
                )
            } else {
                write_open_error(path, error)
            }
        })
}

fn write_open_error(path: &Path, error: std::io::Error) -> ServiceError {
    file_error(
        "fs.writeFailed",
        format!("failed to open {} for write: {error}", path.display()),
    )
}

#[async_trait]
impl PlatformService for TauriFileService {
    fn service_id(&self) -> ServiceId {
        ServiceId::File
    }

    async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
        let arg = request.args.into_iter().next().unwrap_or_else(|| json!({}));

        match request.command.as_str() {
            "fs_stat" | "stat" => {
                serde_json::to_value(self.stat(parse_arg(arg)?).await?).map_err(serialize_error)
            }
            "fs_read_file" | "readFile" => {
                serde_json::to_value(self.read_file(parse_arg(arg)?).await?)
                    .map_err(serialize_error)
            }
            "fs_write_file" | "writeFile" => {
                serde_json::to_value(self.write_file(parse_arg(arg)?).await?)
                    .map_err(serialize_error)
            }
            "fs_delete" | "delete" => {
                self.delete(parse_arg(arg)?).await?;
                Ok(Value::Null)
            }
            "fs_mkdir" | "mkdir" => {
                serde_json::to_value(self.mkdir(parse_arg(arg)?).await?).map_err(serialize_error)
            }
            "fs_readdir" | "readdir" => {
                serde_json::to_value(self.readdir(parse_arg(arg)?).await?).map_err(serialize_error)
            }
            "fs_watch" | "watch" => {
                serde_json::to_value(self.watch(parse_arg(arg)?).await?).map_err(serialize_error)
            }
            "fs_unwatch" | "unwatch" => {
                self.unwatch(parse_arg(arg)?).await?;
                Ok(Value::Null)
            }
            command => Err(ServiceError::unsupported(ServiceId::File, command)),
        }
    }

    async fn listen(
        &self,
        request: crate::contracts::ChannelListenRequest,
        sink: EventSink,
    ) -> Result<SubscriptionHandle, ServiceError> {
        let arg = request.args.into_iter().next().unwrap_or_else(|| json!({}));

        match request.event.as_str() {
            "fs_watch" | "watch" => self.listen_watch(parse_arg(arg)?, sink).await,
            event => Err(ServiceError::unsupported_listen(ServiceId::File, event)),
        }
    }
}

impl FileService for TauriFileService {}

#[async_trait]
impl PlatformService for TauriWatcherService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Watcher
    }

    async fn call(&self, request: ChannelCallRequest) -> Result<Value, ServiceError> {
        let arg = request.args.into_iter().next().unwrap_or_else(|| json!({}));

        match request.command.as_str() {
            "watch" | "startWatching" | "fs_watch" => {
                serde_json::to_value(self.files.watch(parse_arg(arg)?).await?)
                    .map_err(serialize_error)
            }
            "unwatch" | "stopWatching" | "fs_unwatch" => {
                self.files.unwatch(parse_arg(arg)?).await?;
                Ok(Value::Null)
            }
            command => Err(ServiceError::unsupported(ServiceId::Watcher, command)),
        }
    }

    async fn listen(
        &self,
        request: crate::contracts::ChannelListenRequest,
        sink: EventSink,
    ) -> Result<SubscriptionHandle, ServiceError> {
        let arg = request.args.into_iter().next().unwrap_or_else(|| json!({}));
        match request.event.as_str() {
            "watch" | "fileChanges" | "onDidChangeFile" => {
                self.files.listen_watch(parse_arg(arg)?, sink).await
            }
            event => Err(ServiceError::unsupported_listen(ServiceId::Watcher, event)),
        }
    }
}

impl WatcherService for TauriWatcherService {}

fn parse_arg<T: for<'de> Deserialize<'de>>(value: Value) -> Result<T, ServiceError> {
    serde_json::from_value(value).map_err(|error| {
        file_error(
            "fs.invalidArgument",
            format!("invalid file service argument: {error}"),
        )
    })
}

fn serialize_error(error: serde_json::Error) -> ServiceError {
    file_error(
        "fs.serializeFailed",
        format!("failed to serialize file service response: {error}"),
    )
}

fn file_error(code: &'static str, message: String) -> ServiceError {
    ServiceError { code, message }
}

fn map_io_error(
    default_code: &'static str,
    operation: &'static str,
    path: &Path,
    error: std::io::Error,
) -> ServiceError {
    let code = match error.kind() {
        ErrorKind::NotFound => "fs.fileNotFound",
        ErrorKind::AlreadyExists => "fs.fileExists",
        ErrorKind::PermissionDenied => "fs.noPermissions",
        _ => default_code,
    };

    file_error(
        code,
        format!("failed to {operation} {}: {error}", path.display()),
    )
}

async fn reject_directory_read(path: &Path) -> Result<(), ServiceError> {
    let metadata = fs::symlink_metadata(path)
        .await
        .map_err(|error| map_io_error("fs.readFailed", "stat before read", path, error))?;
    if metadata.file_type().is_dir() {
        return Err(file_error(
            "fs.fileIsDirectory",
            format!("cannot read directory as file: {}", path.display()),
        ));
    }

    Ok(())
}

fn watch_event_payload(event: notify::Event) -> Value {
    let kind = format!("{:?}", event.kind);
    let changes = changes_from_watch_event(&event);

    serde_json::to_value(FileWatchEventDto { changes, kind }).unwrap_or_else(
        |error| json!({ "changes": [], "kind": "Other", "error": error.to_string() }),
    )
}

fn watch_payload_has_changes(payload: &Value) -> bool {
    payload
        .get("changes")
        .and_then(Value::as_array)
        .is_some_and(|changes| !changes.is_empty())
}

fn changes_from_watch_event(event: &notify::Event) -> Vec<FileChangeDto> {
    match &event.kind {
        EventKind::Access(_) => Vec::new(),
        EventKind::Modify(ModifyKind::Name(RenameMode::Both)) if event.paths.len() >= 2 => {
            vec![
                FileChangeDto {
                    resource: uri_from_path(event.paths[0].clone()),
                    r#type: FILE_CHANGE_TYPE_DELETED,
                },
                FileChangeDto {
                    resource: uri_from_path(event.paths[1].clone()),
                    r#type: FILE_CHANGE_TYPE_ADDED,
                },
            ]
        }
        EventKind::Modify(ModifyKind::Name(RenameMode::From)) => {
            sorted_changes(event.paths.clone(), FILE_CHANGE_TYPE_DELETED)
        }
        EventKind::Modify(ModifyKind::Name(RenameMode::To)) => {
            sorted_changes(event.paths.clone(), FILE_CHANGE_TYPE_ADDED)
        }
        kind => sorted_changes(event.paths.clone(), change_type_from_event_kind(kind)),
    }
}

fn sorted_changes(mut paths: Vec<PathBuf>, change_type: u8) -> Vec<FileChangeDto> {
    paths.sort();
    paths.dedup();
    paths
        .into_iter()
        .map(|path| FileChangeDto {
            resource: uri_from_path(path),
            r#type: change_type,
        })
        .collect()
}

fn change_type_from_event_kind(kind: &EventKind) -> u8 {
    match kind {
        EventKind::Create(_) => FILE_CHANGE_TYPE_ADDED,
        EventKind::Remove(_) => FILE_CHANGE_TYPE_DELETED,
        EventKind::Modify(_) | EventKind::Access(_) | EventKind::Any | EventKind::Other => {
            FILE_CHANGE_TYPE_UPDATED
        }
    }
}

fn uri_from_path(path: PathBuf) -> UriDto {
    match path_to_file_uri(&path) {
        Ok(uri) => uri,
        Err(error) => {
            tracing::warn!(%error.message, path = %path.display(), "failed to encode file watch path as URI");
            UriDto {
                scheme: "file".to_string(),
                authority: None,
                path: path.to_string_lossy().to_string(),
                query: None,
                fragment: None,
            }
        }
    }
}

pub fn path_from_uri(resource: &UriDto) -> Result<PathBuf, ServiceError> {
    file_uri_to_path(resource)
}

pub fn file_uri_to_path(resource: &UriDto) -> Result<PathBuf, ServiceError> {
    if resource.scheme != "file" {
        return Err(file_error(
            "fs.invalidUri",
            format!(
                "unsupported URI scheme for file service: {}",
                resource.scheme
            ),
        ));
    }

    if resource.path.is_empty() {
        return Err(file_error(
            "fs.invalidUri",
            "file URI path is empty".to_string(),
        ));
    }

    file_uri_to_platform_path(resource)
}

#[cfg(unix)]
fn file_uri_to_platform_path(resource: &UriDto) -> Result<PathBuf, ServiceError> {
    if resource
        .authority
        .as_ref()
        .is_some_and(|authority| !authority.is_empty())
    {
        return Err(file_error(
            "fs.invalidUri",
            "file URI authority is not supported".to_string(),
        ));
    }

    let decoded_path = percent_decode_utf8(&resource.path, "file URI path")?;
    let path = Path::new(&decoded_path);
    if !path.is_absolute() {
        return Err(file_error(
            "fs.invalidUri",
            format!("file URI path must be absolute: {}", resource.path),
        ));
    }

    Ok(path.to_path_buf())
}

#[cfg(windows)]
fn file_uri_to_platform_path(resource: &UriDto) -> Result<PathBuf, ServiceError> {
    windows_file_uri_to_path_string(resource).map(PathBuf::from)
}

#[cfg(any(windows, test))]
fn windows_file_uri_to_path_string(resource: &UriDto) -> Result<String, ServiceError> {
    let decoded_path = percent_decode_utf8(&resource.path, "file URI path")?;
    let authority = resource.authority.as_deref().unwrap_or_default();

    if !authority.is_empty() {
        if !decoded_path.starts_with('/') {
            return Err(file_error(
                "fs.invalidUri",
                format!("UNC file URI path must be absolute: {}", resource.path),
            ));
        }

        let unc_tail = decoded_path.trim_start_matches('/');
        if unc_tail.is_empty() || unc_tail.split('/').next().map_or(true, str::is_empty) {
            return Err(file_error(
                "fs.invalidUri",
                format!("UNC file URI path must include a share: {}", resource.path),
            ));
        }

        return Ok(format!(r"\\{}\{}", authority, unc_tail.replace('/', r"\")));
    }

    let bytes = decoded_path.as_bytes();
    if bytes.len() < 4
        || bytes[0] != b'/'
        || !bytes[1].is_ascii_alphabetic()
        || bytes[2] != b':'
        || bytes[3] != b'/'
    {
        return Err(file_error(
            "fs.invalidUri",
            format!(
                "Windows file URI path must include an absolute drive path: {}",
                resource.path
            ),
        ));
    }

    let drive = (bytes[1] as char).to_ascii_uppercase();
    let tail = decoded_path[3..].replace('/', r"\");
    Ok(format!("{drive}:{tail}"))
}

pub fn path_to_file_uri(path: &Path) -> Result<UriDto, ServiceError> {
    path_to_platform_file_uri(path)
}

#[cfg(unix)]
fn path_to_platform_file_uri(path: &Path) -> Result<UriDto, ServiceError> {
    if !path.is_absolute() {
        return Err(file_error(
            "fs.invalidUri",
            format!("file path must be absolute: {}", path.display()),
        ));
    }

    Ok(UriDto {
        scheme: "file".to_string(),
        authority: None,
        path: percent_encode_path_bytes(path.as_os_str().as_bytes()),
        query: None,
        fragment: None,
    })
}

#[cfg(windows)]
fn path_to_platform_file_uri(path: &Path) -> Result<UriDto, ServiceError> {
    let path_str = path.to_str().ok_or_else(|| {
        file_error(
            "fs.invalidUri",
            format!("file path is not valid UTF-8: {}", path.display()),
        )
    })?;

    windows_path_str_to_file_uri(path_str, &path.display().to_string())
}

#[cfg(any(windows, test))]
fn windows_path_str_to_file_uri(
    path_str: &str,
    display_path: &str,
) -> Result<UriDto, ServiceError> {
    if let Some(stripped) = path_str.strip_prefix(r"\\") {
        let mut parts = stripped.splitn(3, |ch| ch == '\\' || ch == '/');
        let server = parts.next().unwrap_or_default();
        let share = parts.next().unwrap_or_default();
        let rest = parts.next().unwrap_or_default();

        if server.is_empty() || share.is_empty() {
            return Err(file_error(
                "fs.invalidUri",
                format!("UNC file path must include server and share: {display_path}"),
            ));
        }

        let uri_path = if rest.is_empty() {
            format!("/{share}")
        } else {
            format!("/{share}/{}", rest.replace('\\', "/"))
        };

        return Ok(UriDto {
            scheme: "file".to_string(),
            authority: Some(server.to_string()),
            path: percent_encode_path_str(&uri_path),
            query: None,
            fragment: None,
        });
    }

    let bytes = path_str.as_bytes();
    if bytes.len() < 3
        || !bytes[0].is_ascii_alphabetic()
        || bytes[1] != b':'
        || !matches!(bytes[2], b'\\' | b'/')
    {
        return Err(file_error(
            "fs.invalidUri",
            format!("Windows file path must be absolute: {display_path}"),
        ));
    }

    let drive = (bytes[0] as char).to_ascii_lowercase();
    let tail = path_str[2..].replace('\\', "/");
    Ok(UriDto {
        scheme: "file".to_string(),
        authority: None,
        path: percent_encode_path_str(&format!("/{drive}:{tail}")),
        query: None,
        fragment: None,
    })
}

fn percent_decode_utf8(value: &str, label: &str) -> Result<String, ServiceError> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'%' => {
                if index + 2 >= bytes.len() {
                    return Err(invalid_percent_escape(label, value));
                }
                let high = hex_value(bytes[index + 1])
                    .ok_or_else(|| invalid_percent_escape(label, value))?;
                let low = hex_value(bytes[index + 2])
                    .ok_or_else(|| invalid_percent_escape(label, value))?;
                decoded.push((high << 4) | low);
                index += 3;
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }

    String::from_utf8(decoded).map_err(|error| {
        file_error(
            "fs.invalidUri",
            format!("{label} is not valid UTF-8 after percent decoding: {error}"),
        )
    })
}

fn invalid_percent_escape(label: &str, value: &str) -> ServiceError {
    file_error(
        "fs.invalidUri",
        format!("{label} contains invalid percent escape: {value}"),
    )
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(any(windows, test))]
fn percent_encode_path_str(value: &str) -> String {
    percent_encode_path_bytes(value.as_bytes())
}

fn percent_encode_path_bytes(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len());
    for &byte in bytes {
        if is_file_uri_path_byte(byte) {
            encoded.push(byte as char);
        } else {
            encoded.push('%');
            encoded.push(HEX[(byte >> 4) as usize] as char);
            encoded.push(HEX[(byte & 0x0f) as usize] as char);
        }
    }

    encoded
}

fn is_file_uri_path_byte(byte: u8) -> bool {
    matches!(
        byte,
        b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'/'
            | b'-'
            | b'.'
            | b'_'
            | b'~'
            | b':'
    )
}

const HEX: &[u8; 16] = b"0123456789ABCDEF";

fn stat_from_metadata(resource: UriDto, metadata: std::fs::Metadata) -> FileStatDto {
    FileStatDto {
        resource,
        r#type: file_type_from_metadata(&metadata),
        ctime: system_time_millis(metadata.created().ok()),
        mtime: system_time_millis(metadata.modified().ok()),
        size: metadata.len(),
        readonly: Some(metadata.permissions().readonly()),
    }
}

fn file_type_from_metadata(metadata: &std::fs::Metadata) -> FileTypeDto {
    let metadata_type = metadata.file_type();
    if metadata_type.is_symlink() {
        FileTypeDto::SymbolicLink
    } else if metadata_type.is_dir() {
        FileTypeDto::Directory
    } else if metadata_type.is_file() {
        FileTypeDto::File
    } else {
        FileTypeDto::Unknown
    }
}

fn system_time_millis(time: Option<SystemTime>) -> u64 {
    time.and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn file_uri(path: &Path) -> UriDto {
        UriDto {
            scheme: "file".to_string(),
            authority: None,
            path: path.to_string_lossy().to_string(),
            query: None,
            fragment: None,
        }
    }

    fn unique_temp_path(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("vscode-atomic-{name}-{nonce}"))
    }

    fn uri(scheme: &str, authority: Option<&str>, path: &str) -> UriDto {
        UriDto {
            scheme: scheme.to_string(),
            authority: authority.map(str::to_string),
            path: path.to_string(),
            query: None,
            fragment: None,
        }
    }

    async fn write_text(
        service: &TauriFileService,
        path: &Path,
        contents: &str,
        create: bool,
        overwrite: bool,
    ) -> Result<FileStatDto, ServiceError> {
        service
            .write_file(FileWriteRequest {
                resource: file_uri(path),
                data_base64: STANDARD.encode(contents),
                create,
                overwrite,
            })
            .await
    }

    #[tokio::test]
    async fn write_read_and_stat_temp_file() {
        let service = TauriFileService::default();
        let path = unique_temp_path("file-service.txt");
        let resource = file_uri(&path);

        let stat = service
            .write_file(FileWriteRequest {
                resource: resource.clone(),
                data_base64: STANDARD.encode("hello file service"),
                create: true,
                overwrite: false,
            })
            .await
            .expect("write succeeds");

        assert_eq!(stat.r#type, FileTypeDto::File);
        assert_eq!(stat.size, 18);

        let read = service
            .read_file(FileReadRequest {
                resource: resource.clone(),
            })
            .await
            .expect("read succeeds");
        assert_eq!(read.data_base64, STANDARD.encode("hello file service"));

        let stat = service
            .stat(FileStatRequest { resource })
            .await
            .expect("stat succeeds");
        assert_eq!(stat.size, 18);

        let _ = std::fs::remove_file(path);
    }

    #[tokio::test]
    async fn platform_call_dispatches_file_command_aliases() {
        let service = TauriFileService::default();
        let path = unique_temp_path("file-service-dispatch.txt");
        let resource = file_uri(&path);

        let write_value = service
            .call(ChannelCallRequest {
                request_id: "req-write".to_string(),
                channel: "files".to_string(),
                command: "writeFile".to_string(),
                args: vec![json!({
                    "resource": resource.clone(),
                    "dataBase64": STANDARD.encode("dispatch"),
                    "create": true,
                    "overwrite": false
                })],
                cancellation_id: None,
            })
            .await
            .expect("writeFile dispatch succeeds");
        assert_eq!(write_value["type"], "file");
        assert_eq!(write_value["size"], 8);

        let read_value = service
            .call(ChannelCallRequest {
                request_id: "req-read".to_string(),
                channel: "files".to_string(),
                command: "readFile".to_string(),
                args: vec![json!({ "resource": resource })],
                cancellation_id: None,
            })
            .await
            .expect("readFile dispatch succeeds");
        assert_eq!(read_value["dataBase64"], STANDARD.encode("dispatch"));

        let _ = std::fs::remove_file(path);
    }

    #[tokio::test]
    async fn mkdir_readdir_and_delete_directory_tree() {
        let service = TauriFileService::default();
        let root = unique_temp_path("file-service-tree");
        let nested = root.join("nested");

        let stat = service
            .mkdir(FileMkdirRequest {
                resource: file_uri(&nested),
            })
            .await
            .expect("mkdir succeeds recursively");
        assert_eq!(stat.r#type, FileTypeDto::Directory);
        assert!(nested.is_dir());

        std::fs::write(root.join("b.txt"), "b").expect("write b.txt");
        std::fs::write(root.join("a.txt"), "a").expect("write a.txt");

        let entries = service
            .readdir(FileReaddirRequest {
                resource: file_uri(&root),
            })
            .await
            .expect("readdir succeeds");

        assert_eq!(
            entries
                .iter()
                .map(|entry| (entry.name.as_str(), entry.r#type))
                .collect::<Vec<_>>(),
            vec![
                ("a.txt", FileTypeDto::File),
                ("b.txt", FileTypeDto::File),
                ("nested", FileTypeDto::Directory),
            ]
        );

        let non_recursive_error = service
            .delete(FileDeleteRequest {
                resource: file_uri(&root),
                recursive: false,
                use_trash: false,
            })
            .await
            .expect_err("non-recursive directory delete rejects non-empty directory");
        assert_eq!(non_recursive_error.code, "fs.deleteFailed");

        service
            .delete(FileDeleteRequest {
                resource: file_uri(&root),
                recursive: true,
                use_trash: false,
            })
            .await
            .expect("recursive delete succeeds");
        assert!(!root.exists());
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn stat_readdir_and_delete_preserve_symlink_metadata() {
        let service = TauriFileService::default();
        let root = unique_temp_path("file-service-symlink");
        let target = root.join("target.txt");
        let link = root.join("link.txt");
        std::fs::create_dir(&root).expect("create symlink root");
        std::fs::write(&target, "target").expect("write symlink target");
        std::os::unix::fs::symlink(&target, &link).expect("create symlink");

        let stat = service
            .stat(FileStatRequest {
                resource: file_uri(&link),
            })
            .await
            .expect("stat symlink succeeds");
        assert_eq!(stat.r#type, FileTypeDto::SymbolicLink);

        let entries = service
            .readdir(FileReaddirRequest {
                resource: file_uri(&root),
            })
            .await
            .expect("readdir symlink succeeds");
        assert!(entries
            .iter()
            .any(|entry| entry.name == "link.txt" && entry.r#type == FileTypeDto::SymbolicLink));

        service
            .delete(FileDeleteRequest {
                resource: file_uri(&link),
                recursive: false,
                use_trash: false,
            })
            .await
            .expect("delete symlink succeeds");
        assert!(!link.exists());
        assert!(target.exists());

        let _ = std::fs::remove_file(target);
        let _ = std::fs::remove_dir(root);
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn delete_with_use_trash_moves_target_to_xdg_trash() {
        let service = TauriFileService::default();
        let path = unique_temp_path("file-service-trash.txt");
        let data_home = unique_temp_path("file-service-trash-home");
        let original_xdg_data_home = std::env::var_os("XDG_DATA_HOME");
        std::env::set_var("XDG_DATA_HOME", &data_home);
        std::fs::write(&path, "trash").expect("seed trash target");

        service
            .delete(FileDeleteRequest {
                resource: file_uri(&path),
                recursive: false,
                use_trash: true,
            })
            .await
            .expect("useTrash delete succeeds");

        let trash_name = path.file_name().expect("trash target has file name");
        let trashed_file = data_home.join("Trash/files").join(trash_name);
        let trash_info = data_home
            .join("Trash/info")
            .join(format!("{}.trashinfo", trash_name.to_string_lossy()));
        assert!(!path.exists());
        assert_eq!(
            std::fs::read_to_string(&trashed_file).expect("trashed file readable"),
            "trash"
        );
        assert!(std::fs::read_to_string(&trash_info)
            .expect("trash info readable")
            .contains("[Trash Info]"));

        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir_all(data_home);
        match original_xdg_data_home {
            Some(value) => std::env::set_var("XDG_DATA_HOME", value),
            None => std::env::remove_var("XDG_DATA_HOME"),
        }
    }

    #[tokio::test]
    async fn delete_removes_file_and_reports_missing_targets() {
        let service = TauriFileService::default();
        let path = unique_temp_path("file-service-delete-file.txt");
        std::fs::write(&path, "delete me").expect("seed delete target");

        service
            .delete(FileDeleteRequest {
                resource: file_uri(&path),
                recursive: false,
                use_trash: false,
            })
            .await
            .expect("file delete succeeds");
        assert!(!path.exists());

        let error = service
            .delete(FileDeleteRequest {
                resource: file_uri(&path),
                recursive: false,
                use_trash: false,
            })
            .await
            .expect_err("missing delete target rejected");

        assert_eq!(error.code, "fs.fileNotFound");
    }

    #[tokio::test]
    async fn platform_call_dispatches_delete_mkdir_readdir_aliases() {
        let service = TauriFileService::default();
        let root = unique_temp_path("file-service-dispatch-tree");
        let child = root.join("child");

        let mkdir_value = service
            .call(ChannelCallRequest {
                request_id: "req-mkdir".to_string(),
                channel: "files".to_string(),
                command: "mkdir".to_string(),
                args: vec![json!({ "resource": file_uri(&child) })],
                cancellation_id: None,
            })
            .await
            .expect("mkdir dispatch succeeds");
        assert_eq!(mkdir_value["type"], "directory");

        std::fs::write(root.join("file.txt"), "entry").expect("write dispatched entry");
        let entries_value = service
            .call(ChannelCallRequest {
                request_id: "req-readdir".to_string(),
                channel: "files".to_string(),
                command: "fs_readdir".to_string(),
                args: vec![json!({ "resource": file_uri(&root) })],
                cancellation_id: None,
            })
            .await
            .expect("fs_readdir dispatch succeeds");
        assert_eq!(entries_value[0]["name"], "child");
        assert_eq!(entries_value[1]["name"], "file.txt");

        let delete_value = service
            .call(ChannelCallRequest {
                request_id: "req-delete".to_string(),
                channel: "files".to_string(),
                command: "delete".to_string(),
                args: vec![json!({
                    "resource": file_uri(&root),
                    "recursive": true,
                    "useTrash": false
                })],
                cancellation_id: None,
            })
            .await
            .expect("delete dispatch succeeds");
        assert!(delete_value.is_null());
        assert!(!root.exists());
    }

    #[tokio::test]
    async fn platform_call_dispatches_stat_and_watch_aliases() {
        let service = TauriFileService::default();
        let root = unique_temp_path("file-service-dispatch-watch");
        std::fs::create_dir(&root).expect("create watch target");
        let file = root.join("stat.txt");
        std::fs::write(&file, "stat").expect("seed stat target");

        let stat_value = service
            .call(ChannelCallRequest {
                request_id: "req-stat".to_string(),
                channel: "files".to_string(),
                command: "fs_stat".to_string(),
                args: vec![json!({ "resource": file_uri(&file) })],
                cancellation_id: None,
            })
            .await
            .expect("fs_stat dispatch succeeds");
        assert_eq!(stat_value["type"], "file");
        assert_eq!(stat_value["size"], 4);

        let watch_value = service
            .call(ChannelCallRequest {
                request_id: "req-watch".to_string(),
                channel: "files".to_string(),
                command: "watch".to_string(),
                args: vec![json!({ "resource": file_uri(&root), "recursive": false })],
                cancellation_id: None,
            })
            .await
            .expect("watch dispatch succeeds");
        assert_eq!(watch_value["watchId"], "watch-1");

        let fs_watch_value = service
            .call(ChannelCallRequest {
                request_id: "req-fs-watch".to_string(),
                channel: "files".to_string(),
                command: "fs_watch".to_string(),
                args: vec![json!({ "resource": file_uri(&root), "recursive": true })],
                cancellation_id: None,
            })
            .await
            .expect("fs_watch dispatch succeeds");
        assert_eq!(fs_watch_value["watchId"], "watch-2");
        assert_eq!(service.watches.lock().await.len(), 2);

        let _ = std::fs::remove_file(file);
        let _ = std::fs::remove_dir(root);
    }

    #[tokio::test]
    async fn unwatch_removes_registered_watch_and_rejects_unknown_id() {
        let service = TauriFileService::default();
        let root = unique_temp_path("file-service-unwatch");
        std::fs::create_dir(&root).expect("create watch target");

        let watch = service
            .watch(FileWatchRequest {
                resource: file_uri(&root),
                recursive: false,
            })
            .await
            .expect("watch succeeds");
        assert_eq!(service.watches.lock().await.len(), 1);

        service
            .unwatch(FileUnwatchRequest {
                watch_id: watch.watch_id.clone(),
            })
            .await
            .expect("unwatch succeeds");
        assert!(service.watches.lock().await.is_empty());

        let error = service
            .unwatch(FileUnwatchRequest {
                watch_id: watch.watch_id,
            })
            .await
            .expect_err("unknown watch rejected");
        assert_eq!(error.code, "fs.watchNotFound");

        let _ = std::fs::remove_dir(root);
    }

    #[tokio::test]
    async fn platform_call_dispatches_unwatch_alias() {
        let service = TauriFileService::default();
        let root = unique_temp_path("file-service-dispatch-unwatch");
        std::fs::create_dir(&root).expect("create watch target");

        let watch_value = service
            .call(ChannelCallRequest {
                request_id: "req-watch".to_string(),
                channel: "files".to_string(),
                command: "fs_watch".to_string(),
                args: vec![json!({ "resource": file_uri(&root), "recursive": false })],
                cancellation_id: None,
            })
            .await
            .expect("fs_watch dispatch succeeds");

        let unwatch_value = service
            .call(ChannelCallRequest {
                request_id: "req-unwatch".to_string(),
                channel: "files".to_string(),
                command: "fs_unwatch".to_string(),
                args: vec![json!({ "watchId": watch_value["watchId"] })],
                cancellation_id: None,
            })
            .await
            .expect("fs_unwatch dispatch succeeds");
        assert!(unwatch_value.is_null());
        assert!(service.watches.lock().await.is_empty());

        let _ = std::fs::remove_dir(root);
    }

    #[tokio::test]
    async fn write_file_rejects_invalid_base64_payload() {
        let service = TauriFileService::default();
        let path = unique_temp_path("file-service-invalid.txt");

        let error = service
            .write_file(FileWriteRequest {
                resource: file_uri(&path),
                data_base64: "not-base64%%%".to_string(),
                create: true,
                overwrite: false,
            })
            .await
            .expect_err("invalid base64 rejected");

        assert_eq!(error.code, "fs.invalidData");
        assert!(!path.exists());
    }

    #[tokio::test]
    async fn file_operations_return_stable_error_codes() {
        let service = TauriFileService::default();
        let missing = unique_temp_path("file-service-missing.txt");

        let stat_error = service
            .stat(FileStatRequest {
                resource: file_uri(&missing),
            })
            .await
            .expect_err("missing stat rejected");
        assert_eq!(stat_error.code, "fs.fileNotFound");

        let read_error = service
            .read_file(FileReadRequest {
                resource: file_uri(&missing),
            })
            .await
            .expect_err("missing read rejected");
        assert_eq!(read_error.code, "fs.fileNotFound");

        let readdir_error = service
            .readdir(FileReaddirRequest {
                resource: file_uri(&missing),
            })
            .await
            .expect_err("missing readdir rejected");
        assert_eq!(readdir_error.code, "fs.fileNotFound");
    }

    #[tokio::test]
    async fn read_file_rejects_directory_with_stable_error_code() {
        let service = TauriFileService::default();
        let dir = unique_temp_path("file-service-read-dir");
        std::fs::create_dir(&dir).expect("create read dir target");

        let error = service
            .read_file(FileReadRequest {
                resource: file_uri(&dir),
            })
            .await
            .expect_err("directory read rejected");
        assert_eq!(error.code, "fs.fileIsDirectory");

        let _ = std::fs::remove_dir(dir);
    }

    #[tokio::test]
    async fn write_file_create_overwrite_matrix_for_missing_and_existing_targets() {
        let cases = [
            (true, true, false, None),
            (true, true, true, None),
            (true, false, false, None),
            (true, false, true, Some("fs.fileExists")),
            (false, true, false, Some("fs.fileNotFound")),
            (false, true, true, None),
            (false, false, false, Some("fs.fileNotFound")),
            (false, false, true, Some("fs.fileExists")),
        ];

        for (index, (create, overwrite, preexisting, expected_error_code)) in
            cases.into_iter().enumerate()
        {
            let service = TauriFileService::default();
            let path = unique_temp_path(&format!("file-service-write-matrix-{index}.txt"));
            if preexisting {
                std::fs::write(&path, "before").expect("seed existing target");
            }

            let result = write_text(&service, &path, "after", create, overwrite).await;

            match expected_error_code {
                Some(expected_error_code) => {
                    let error = result.expect_err("write rejected with expected code");
                    assert_eq!(error.code, expected_error_code);
                    if preexisting {
                        assert_eq!(
                            std::fs::read_to_string(&path).expect("existing file remains readable"),
                            "before"
                        );
                    } else {
                        assert!(!path.exists());
                    }
                }
                None => {
                    let stat = result.expect("write succeeds");
                    assert_eq!(stat.r#type, FileTypeDto::File);
                    assert_eq!(stat.size, 5);
                    assert_eq!(
                        std::fs::read_to_string(&path).expect("written file readable"),
                        "after"
                    );
                }
            }

            let _ = std::fs::remove_file(path);
        }
    }

    #[tokio::test]
    async fn write_file_overwrite_truncates_existing_file_when_new_content_is_shorter() {
        let service = TauriFileService::default();
        let path = unique_temp_path("file-service-truncate.txt");
        std::fs::write(&path, "longer existing contents").expect("seed existing target");

        let stat = write_text(&service, &path, "tiny", false, true)
            .await
            .expect("overwrite succeeds");

        assert_eq!(stat.size, 4);
        assert_eq!(
            std::fs::read_to_string(&path).expect("overwritten file readable"),
            "tiny"
        );

        let _ = std::fs::remove_file(path);
    }

    #[tokio::test]
    async fn listen_watch_sends_payload_and_stops_after_dispose() {
        let service = TauriFileService::default();
        let dir = unique_temp_path("file-service-watch");
        std::fs::create_dir(&dir).expect("create watched temp directory");
        let path = dir.join("watched.txt");
        let (sink, mut events) = tokio::sync::mpsc::channel(16);

        let handle = service
            .listen_watch(
                FileWatchRequest {
                    resource: file_uri(&dir),
                    recursive: false,
                },
                sink,
            )
            .await
            .expect("watch listen succeeds");

        write_text(&service, &path, "first", true, false)
            .await
            .expect("write watched file");

        let payload = recv_watch_payload(&mut events).expect("watch event payload received");
        let changes = payload["changes"].as_array().expect("changes is array");
        let expected_path = path.to_string_lossy();
        assert!(changes.iter().any(|change| {
            change["resource"]["path"].as_str() == Some(expected_path.as_ref())
                && matches!(change["type"].as_u64(), Some(0 | 1))
        }));

        while events.try_recv().is_ok() {}
        handle.cancel();
        write_text(&service, &path, "second", false, true)
            .await
            .expect("write after dispose");
        std::thread::sleep(std::time::Duration::from_millis(250));
        assert!(events.try_recv().is_err());

        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(dir);
    }

    #[tokio::test]
    async fn listen_watch_recursive_reports_nested_changes() {
        let service = TauriFileService::default();
        let dir = unique_temp_path("file-service-watch-recursive");
        let nested = dir.join("nested");
        let path = nested.join("watched.txt");
        std::fs::create_dir(&dir).expect("create watched temp directory");
        std::fs::create_dir(&nested).expect("create nested watched directory");

        let (non_recursive_sink, mut non_recursive_events) = tokio::sync::mpsc::channel(16);
        let non_recursive_handle = service
            .listen_watch(
                FileWatchRequest {
                    resource: file_uri(&dir),
                    recursive: false,
                },
                non_recursive_sink,
            )
            .await
            .expect("non-recursive watch listen succeeds");

        let (recursive_sink, mut recursive_events) = tokio::sync::mpsc::channel(16);
        let recursive_handle = service
            .listen_watch(
                FileWatchRequest {
                    resource: file_uri(&dir),
                    recursive: true,
                },
                recursive_sink,
            )
            .await
            .expect("recursive watch listen succeeds");

        write_text(&service, &path, "nested", true, false)
            .await
            .expect("write nested watched file");

        let recursive_payload =
            recv_watch_payload(&mut recursive_events).expect("recursive watch event received");
        let recursive_changes = recursive_payload["changes"]
            .as_array()
            .expect("recursive changes is array");
        let expected_path = path.to_string_lossy();
        assert!(recursive_changes.iter().any(|change| {
            change["resource"]["path"].as_str() == Some(expected_path.as_ref())
                && matches!(change["type"].as_u64(), Some(0 | 1))
        }));
        assert_no_watch_event(
            &mut non_recursive_events,
            std::time::Duration::from_millis(250),
        );

        non_recursive_handle.cancel();
        recursive_handle.cancel();
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir(nested);
        let _ = std::fs::remove_dir(dir);
    }

    #[tokio::test]
    async fn listen_watch_dispose_race_does_not_emit_late_events() {
        const ITERATIONS: usize = 25;

        let service = TauriFileService::default();
        let dir = unique_temp_path("file-service-watch-dispose-race");
        std::fs::create_dir(&dir).expect("create watched temp directory");

        for iteration in 0..ITERATIONS {
            let path = dir.join(format!("watched-{iteration}.txt"));
            let (sink, mut events) = tokio::sync::mpsc::channel(16);
            let handle = service
                .listen_watch(
                    FileWatchRequest {
                        resource: file_uri(&dir),
                        recursive: false,
                    },
                    sink,
                )
                .await
                .expect("watch listen succeeds");

            write_text(&service, &path, "before dispose", true, false)
                .await
                .expect("write before dispose");
            recv_watch_payload(&mut events).expect("watch event payload received before dispose");
            drain_watch_events_until_quiet(&mut events, std::time::Duration::from_millis(75));

            handle.cancel();

            write_text(&service, &path, "after dispose", false, true)
                .await
                .expect("write after dispose");
            assert_no_watch_event(&mut events, std::time::Duration::from_millis(125));

            let _ = std::fs::remove_file(path);
        }

        let _ = std::fs::remove_dir(dir);
    }

    #[test]
    fn watch_event_payload_maps_rename_to_delete_and_add() {
        let old_path = PathBuf::from("/tmp/vscode-atomic-watch-old.txt");
        let new_path = PathBuf::from("/tmp/vscode-atomic-watch-new.txt");
        let event = notify::Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::Both)))
            .add_path(old_path.clone())
            .add_path(new_path.clone());

        let payload = watch_event_payload(event);
        let changes = payload["changes"].as_array().expect("changes is array");

        assert_eq!(changes.len(), 2);
        assert_eq!(
            changes[0]["resource"]["path"].as_str(),
            Some(old_path.to_string_lossy().as_ref())
        );
        assert_eq!(changes[0]["type"], FILE_CHANGE_TYPE_DELETED);
        assert_eq!(
            changes[1]["resource"]["path"].as_str(),
            Some(new_path.to_string_lossy().as_ref())
        );
        assert_eq!(changes[1]["type"], FILE_CHANGE_TYPE_ADDED);
    }

    #[test]
    fn watch_event_payload_ignores_access_only_notifications() {
        let event = notify::Event::new(EventKind::Access(notify::event::AccessKind::Any))
            .add_path(PathBuf::from("/tmp/vscode-atomic-watch-access.txt"));

        let payload = watch_event_payload(event);
        let changes = payload["changes"].as_array().expect("changes is array");

        assert!(changes.is_empty());
    }

    fn recv_watch_payload(
        events: &mut tokio::sync::mpsc::Receiver<Option<Value>>,
    ) -> Option<Value> {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
        loop {
            match events.try_recv() {
                Ok(Some(payload)) => return Some(payload),
                Ok(None) => continue,
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
                    if std::time::Instant::now() >= deadline {
                        return None;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(25));
                }
                Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => return None,
            }
        }
    }

    fn drain_watch_events_until_quiet(
        events: &mut tokio::sync::mpsc::Receiver<Option<Value>>,
        quiet_for: std::time::Duration,
    ) {
        let mut quiet_since = std::time::Instant::now();
        loop {
            match events.try_recv() {
                Ok(_) => quiet_since = std::time::Instant::now(),
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
                    if quiet_since.elapsed() >= quiet_for {
                        return;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
                Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => return,
            }
        }
    }

    fn assert_no_watch_event(
        events: &mut tokio::sync::mpsc::Receiver<Option<Value>>,
        timeout: std::time::Duration,
    ) {
        let deadline = std::time::Instant::now() + timeout;
        loop {
            match events.try_recv() {
                Ok(event) => panic!("unexpected watch event after dispose: {event:?}"),
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
                    if std::time::Instant::now() >= deadline {
                        return;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
                Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => return,
            }
        }
    }

    #[test]
    fn path_validation_rejects_non_file_scheme() {
        let error = path_from_uri(&uri("vscode", None, "/tmp/file.txt"))
            .expect_err("non-file scheme rejected");

        assert_eq!(error.code, "fs.invalidUri");
    }

    #[test]
    fn path_validation_rejects_relative_file_path() {
        let error =
            path_from_uri(&uri("file", None, "relative.txt")).expect_err("relative path rejected");

        assert_eq!(error.code, "fs.invalidUri");
    }

    #[test]
    #[cfg(unix)]
    fn posix_file_uri_decodes_and_encodes_reserved_path_bytes() {
        let resource = uri(
            "file",
            None,
            "/tmp/name%20with%20spaces/%23hash%3Fquery%25percent.txt",
        );

        let path = file_uri_to_path(&resource).expect("POSIX file URI decodes");
        assert_eq!(
            path,
            PathBuf::from("/tmp/name with spaces/#hash?query%percent.txt")
        );

        let encoded = path_to_file_uri(&path).expect("POSIX path encodes");
        assert_eq!(encoded.scheme, "file");
        assert_eq!(encoded.authority, None);
        assert_eq!(
            encoded.path,
            "/tmp/name%20with%20spaces/%23hash%3Fquery%25percent.txt"
        );
    }

    #[test]
    #[cfg(unix)]
    fn empty_authority_is_treated_as_no_authority() {
        let path = file_uri_to_path(&uri("file", Some(""), "/tmp/file.txt"))
            .expect("empty authority accepted");

        assert_eq!(path, PathBuf::from("/tmp/file.txt"));
    }

    #[test]
    fn file_uri_rejects_invalid_percent_encoding() {
        let error = file_uri_to_path(&uri("file", None, "/tmp/%GG.txt"))
            .expect_err("invalid percent escape rejected");

        assert_eq!(error.code, "fs.invalidUri");
    }

    #[test]
    fn windows_file_uri_decodes_drive_letter_case() {
        for path in ["/c:/Users/Alice/file.txt", "/C:/Users/Alice/file.txt"] {
            let decoded = windows_file_uri_to_path_string(&uri("file", None, path))
                .expect("Windows drive file URI decodes");

            assert_eq!(decoded, r"C:\Users\Alice\file.txt");
        }
    }

    #[test]
    fn windows_file_uri_decodes_empty_authority_as_drive_path() {
        let decoded =
            windows_file_uri_to_path_string(&uri("file", Some(""), "/c:/Users/Alice/file.txt"))
                .expect("empty authority accepted");

        assert_eq!(decoded, r"C:\Users\Alice\file.txt");
    }

    #[test]
    fn windows_unc_file_uri_decodes_and_encodes_authority() {
        let decoded = windows_file_uri_to_path_string(&uri("file", Some("server"), "/share/f.txt"))
            .expect("UNC file URI decodes");
        assert_eq!(decoded, r"\\server\share\f.txt");

        let encoded =
            windows_path_str_to_file_uri(r"\\server\share\f.txt", r"\\server\share\f.txt")
                .expect("UNC path encodes");
        assert_eq!(encoded.scheme, "file");
        assert_eq!(encoded.authority.as_deref(), Some("server"));
        assert_eq!(encoded.path, "/share/f.txt");
    }

    #[test]
    fn windows_file_uri_decodes_percent_encoding() {
        let decoded = windows_file_uri_to_path_string(&uri(
            "file",
            None,
            "/c:/Users/Alice/name%20%23%3F%25.txt",
        ))
        .expect("Windows percent-encoded file URI decodes");

        assert_eq!(decoded, r"C:\Users\Alice\name #?%.txt");
    }

    #[test]
    fn windows_file_uri_rejects_relative_path() {
        let error = windows_file_uri_to_path_string(&uri("file", None, "relative.txt"))
            .expect_err("Windows relative path rejected");

        assert_eq!(error.code, "fs.invalidUri");
    }

    #[test]
    #[cfg(unix)]
    fn posix_file_uri_roundtrips_supported_examples() {
        for resource in [
            uri("file", None, "/tmp/file.txt"),
            uri("file", Some(""), "/tmp/name%20%23%3F%25.txt"),
        ] {
            let path = file_uri_to_path(&resource).expect("file URI decodes");
            let encoded = path_to_file_uri(&path).expect("path encodes");

            assert_eq!(encoded, uri("file", None, &resource.path));
        }
    }

    #[test]
    fn windows_file_uri_roundtrips_supported_examples() {
        for (resource, expected_path) in [
            (
                uri("file", None, "/c:/Users/Alice/file.txt"),
                r"C:\Users\Alice\file.txt",
            ),
            (
                uri("file", Some("server"), "/share/name%20%23%3F%25.txt"),
                r"\\server\share\name #?%.txt",
            ),
        ] {
            let path = windows_file_uri_to_path_string(&resource).expect("file URI decodes");
            assert_eq!(path, expected_path);

            let encoded = windows_path_str_to_file_uri(&path, &path).expect("path encodes");
            let expected_path = if resource.authority.is_none() {
                resource.path.replacen("/C:", "/c:", 1)
            } else {
                resource.path.clone()
            };

            assert_eq!(
                encoded,
                uri("file", resource.authority.as_deref(), &expected_path)
            );
        }
    }
}
