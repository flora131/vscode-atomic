use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::{json, Value};
use vscode_atomic_tauri::{
    cancellation_manager::CancellationManager,
    contracts::{
        CancelRequest, CancellationId, ChannelCallRequest, ChannelEventMessage,
        ChannelListenRequest, FileDeleteRequest, FileMkdirRequest, FileReadRequest,
        FileReaddirRequest, FileStatRequest, FileTypeDto, FileWriteRequest, SubscriptionId, UriDto,
    },
    file_service::{FileWatchRequest, TauriFileService},
    parity_services::{
        RustDialogService, RustLifecycleService, RustNativeHostService, RustTelemetryService,
    },
    service_registry::{PlatformService, ServiceId, ServiceRegistry},
    storage_config_service::{JsonConfigurationService, JsonStorageService, UserDataPaths},
    subscription_manager::{
        ChannelEventEmitter, SubscriptionCancellationHandle, SubscriptionError,
        SubscriptionManager, SubscriptionRequest,
    },
    terminal_service::{
        ResizeTerminalRequest, RustTerminalService, SpawnTerminalRequest, TerminalLifecycleState,
        TerminalStreamKind,
    },
};

fn unique_temp_path(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("vscode-atomic-parity-{name}-{nonce}"))
}

#[tokio::test]
async fn platform_parity_services_cover_known_safe_commands_and_deny_unknown_side_effects() {
    let native = RustNativeHostService;
    let os = native
        .call(request("nativeHost", "getOSProperties", Value::Null))
        .await
        .expect("native host os properties succeeds");
    assert_eq!(os["platform"], std::env::consts::OS);
    assert_eq!(
        native
            .call(request("nativeHost", "openExternal", Value::Null))
            .await
            .expect_err("native side-effect denied")
            .code,
        "service.unsupportedCommand"
    );

    let dialog = RustDialogService;
    assert_eq!(
        dialog
            .call(request(
                "dialog",
                "showMessageBox",
                json!({ "message": "hi" })
            ))
            .await
            .expect("dialog message fallback succeeds")["response"],
        0
    );
    assert_eq!(
        dialog
            .call(request("dialog", "showOpenDialog", Value::Null))
            .await
            .expect_err("native file dialog denied")
            .code,
        "service.unsupportedCommand"
    );

    let lifecycle = RustLifecycleService;
    assert_eq!(
        lifecycle
            .call(request("lifecycle", "getLifecyclePhase", Value::Null))
            .await
            .expect("lifecycle phase succeeds")["phase"],
        "ready"
    );
    assert_eq!(
        lifecycle
            .call(request("lifecycle", "quit", Value::Null))
            .await
            .expect_err("lifecycle quit denied")
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
        .expect("telemetry no-op log succeeds");
    assert_eq!(
        telemetry.events().await,
        vec![json!({ "eventName": "startup" })]
    );
    assert_eq!(
        telemetry
            .call(request("telemetry", "flush", Value::Null))
            .await
            .expect_err("telemetry unknown command denied")
            .code,
        "service.unsupportedCommand"
    );
}

fn file_uri(path: &Path) -> UriDto {
    UriDto {
        scheme: "file".to_string(),
        authority: None,
        path: file_uri_path(path),
        query: None,
        fragment: None,
    }
}

#[cfg(unix)]
fn file_uri_path(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

#[cfg(windows)]
fn file_uri_path(path: &Path) -> String {
    format!("/{}", path.to_string_lossy().replace('\\', "/"))
}

fn invalid_file_uri(scheme: &str, path: &str) -> UriDto {
    UriDto {
        scheme: scheme.to_string(),
        authority: None,
        path: path.to_string(),
        query: None,
        fragment: None,
    }
}

fn request(channel: &str, command: &str, arg: serde_json::Value) -> ChannelCallRequest {
    ChannelCallRequest {
        request_id: format!("{channel}-{command}-req"),
        channel: channel.to_string(),
        command: command.to_string(),
        args: vec![arg],
        cancellation_id: None,
    }
}

fn listen_request(channel: &str, event: &str, arg: serde_json::Value) -> ChannelListenRequest {
    ChannelListenRequest {
        request_id: format!("{channel}-{event}-listen-req"),
        channel: channel.to_string(),
        event: event.to_string(),
        args: vec![arg],
        subscription_id: None,
    }
}

#[tokio::test]
async fn channel_parity_covers_cancellation_error_listen_and_dispose_basics() {
    let cancellations = CancellationManager::default();
    let guard = cancellations.begin(
        "channel-req-1".to_string(),
        Some(CancellationId("channel-cancel-1".to_string())),
    );
    assert!(cancellations.cancel(&CancelRequest {
        cancellation_id: CancellationId("channel-cancel-1".to_string()),
        request_id: Some("channel-req-1".to_string()),
    }));
    assert!(guard.is_cancelled());

    let missing = ServiceRegistry::new()
        .dispatch_channel_call(ChannelCallRequest {
            request_id: "missing-req".to_string(),
            channel: "files".to_string(),
            command: "stat".to_string(),
            args: vec![],
            cancellation_id: None,
        })
        .await
        .expect_err("missing service dispatch rejects");
    assert_eq!(missing.code, "service.missing");

    let subscriptions = SubscriptionManager::new(4);
    let (sink, events) = subscriptions.bounded_channel();
    let cancelled = Arc::new(AtomicBool::new(false));
    let cancellation = SubscriptionCancellationHandle::new({
        let cancelled = cancelled.clone();
        move || cancelled.store(true, Ordering::SeqCst)
    });
    let emitter = Arc::new(RecordingEmitter::default());
    let subscription_id = subscriptions
        .subscribe(
            SubscriptionRequest {
                subscription_id: Some(SubscriptionId("channel-sub-1".to_string())),
                channel: "files".to_string(),
                event: "watch".to_string(),
                window_label: "main".to_string(),
            },
            events,
            cancellation,
            emitter.clone(),
        )
        .expect("subscription registered");

    sink.send(Some(json!({ "ok": true })))
        .await
        .expect("subscription event queued");
    for _ in 0..20 {
        if emitter.messages().len() == 1 {
            break;
        }
        tokio::task::yield_now().await;
    }

    assert_eq!(emitter.messages().len(), 1);
    assert!(subscriptions.dispose(&subscription_id));
    assert!(cancelled.load(Ordering::SeqCst));
    assert!(!subscriptions.contains(&subscription_id));
}

#[derive(Default)]
struct RecordingEmitter {
    messages: Mutex<Vec<(String, ChannelEventMessage)>>,
}

impl RecordingEmitter {
    fn messages(&self) -> Vec<(String, ChannelEventMessage)> {
        self.messages
            .lock()
            .expect("recording emitter lock poisoned")
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
            .expect("recording emitter lock poisoned")
            .push((window_label.to_string(), message));
        Ok(())
    }
}

#[tokio::test]
async fn file_service_parity_covers_stat_read_write_delete_mkdir_readdir() {
    let service = TauriFileService::default();
    let root = unique_temp_path("files");
    let nested = root.join("nested");
    let file = nested.join("hello.txt");

    let mkdir_stat = service
        .mkdir(FileMkdirRequest {
            resource: file_uri(&nested),
        })
        .await
        .expect("mkdir succeeds");
    assert_eq!(mkdir_stat.r#type, FileTypeDto::Directory);

    let write_stat = service
        .write_file(FileWriteRequest {
            resource: file_uri(&file),
            data_base64: STANDARD.encode("hello parity"),
            create: true,
            overwrite: false,
        })
        .await
        .expect("write succeeds");
    assert_eq!(write_stat.r#type, FileTypeDto::File);
    assert_eq!(write_stat.size, 12);

    let read = service
        .read_file(FileReadRequest {
            resource: file_uri(&file),
        })
        .await
        .expect("read succeeds");
    assert_eq!(read.data_base64, STANDARD.encode("hello parity"));

    let stat = service
        .stat(FileStatRequest {
            resource: file_uri(&file),
        })
        .await
        .expect("stat succeeds");
    assert_eq!(stat.r#type, FileTypeDto::File);
    assert_eq!(stat.size, 12);

    let entries = service
        .readdir(FileReaddirRequest {
            resource: file_uri(&nested),
        })
        .await
        .expect("readdir succeeds");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "hello.txt");
    assert_eq!(entries[0].r#type, FileTypeDto::File);

    service
        .delete(FileDeleteRequest {
            resource: file_uri(&file),
            recursive: false,
            use_trash: false,
        })
        .await
        .expect("delete succeeds");
    assert!(!file.exists());

    service
        .delete(FileDeleteRequest {
            resource: file_uri(&root),
            recursive: true,
            use_trash: false,
        })
        .await
        .expect("recursive directory delete succeeds");
    assert!(!root.exists());
}

#[tokio::test]
async fn file_service_parity_covers_error_codes_and_dispatch_aliases() {
    let service = TauriFileService::default();
    let root = unique_temp_path("files-errors");
    let nested = root.join("nested");
    let file = nested.join("hello.txt");

    service
        .call(request(
            "files",
            "mkdir",
            json!({ "resource": file_uri(&nested) }),
        ))
        .await
        .expect("mkdir alias succeeds");
    service
        .call(request(
            "files",
            "fs_write_file",
            json!({
                "resource": file_uri(&file),
                "dataBase64": STANDARD.encode("hello parity"),
                "create": true,
                "overwrite": false,
            }),
        ))
        .await
        .expect("fs_write_file alias succeeds");

    let read = service
        .call(request(
            "files",
            "readFile",
            json!({ "resource": file_uri(&file) }),
        ))
        .await
        .expect("readFile alias succeeds");
    assert_eq!(read["dataBase64"], STANDARD.encode("hello parity"));

    let exists_error = service
        .write_file(FileWriteRequest {
            resource: file_uri(&file),
            data_base64: STANDARD.encode("duplicate"),
            create: true,
            overwrite: false,
        })
        .await
        .expect_err("duplicate create rejected");
    assert_eq!(exists_error.code, "fs.fileExists");

    let invalid_data_error = service
        .write_file(FileWriteRequest {
            resource: file_uri(&nested.join("invalid.txt")),
            data_base64: "not-valid-base64".to_string(),
            create: true,
            overwrite: false,
        })
        .await
        .expect_err("invalid base64 rejected");
    assert_eq!(invalid_data_error.code, "fs.invalidData");

    let missing_read_error = service
        .read_file(FileReadRequest {
            resource: file_uri(&nested.join("missing.txt")),
        })
        .await
        .expect_err("missing read rejected");
    assert_eq!(missing_read_error.code, "fs.fileNotFound");

    let directory_read_error = service
        .read_file(FileReadRequest {
            resource: file_uri(&nested),
        })
        .await
        .expect_err("directory read rejected");
    assert_eq!(directory_read_error.code, "fs.fileIsDirectory");

    let readdir_file_error = service
        .readdir(FileReaddirRequest {
            resource: file_uri(&file),
        })
        .await
        .expect_err("file readdir rejected");
    assert_eq!(readdir_file_error.code, "fs.readdirFailed");

    let delete_missing_error = service
        .delete(FileDeleteRequest {
            resource: file_uri(&nested.join("missing.txt")),
            recursive: false,
            use_trash: false,
        })
        .await
        .expect_err("missing delete rejected");
    assert_eq!(delete_missing_error.code, "fs.fileNotFound");

    service
        .call(request(
            "files",
            "delete",
            json!({ "resource": file_uri(&root), "recursive": true, "useTrash": false }),
        ))
        .await
        .expect("delete alias succeeds");
}

#[tokio::test]
async fn file_watch_parity_emits_changes_and_reports_missing_watch() {
    let service = TauriFileService::default();
    let root = unique_temp_path("watch-events");
    std::fs::create_dir_all(&root).expect("watch root created");
    let (sink, mut events) = tokio::sync::mpsc::channel(16);

    let handle = service
        .listen(
            listen_request(
                "files",
                "watch",
                json!({ "resource": file_uri(&root), "recursive": false }),
            ),
            sink,
        )
        .await
        .expect("file watch listen succeeds");

    let created = root.join("created.txt");
    std::fs::write(&created, "watch parity").expect("watched file created");

    let mut saw_change = false;
    for _ in 0..40 {
        if let Ok(Some(payload)) = events.try_recv() {
            let changes = payload
                .get("changes")
                .and_then(Value::as_array)
                .expect("watch payload changes array");
            if changes.iter().any(|change| {
                change["resource"]["path"] == file_uri(&created).path
                    && change["type"].as_u64().is_some()
            }) {
                saw_change = true;
                break;
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    assert!(saw_change);
    handle.cancel();

    let missing_watch_error = service
        .call(request(
            "files",
            "unwatch",
            json!({ "watchId": "watch-missing" }),
        ))
        .await
        .expect_err("missing watch rejected");
    assert_eq!(missing_watch_error.code, "fs.watchNotFound");

    let _ = std::fs::remove_dir_all(root);
}

#[tokio::test]
async fn file_service_parity_rejects_invalid_paths_for_stat_and_watch() {
    let service = TauriFileService::default();

    let stat_error = service
        .stat(FileStatRequest {
            resource: invalid_file_uri("untitled", "/tmp/missing.txt"),
        })
        .await
        .expect_err("non-file stat URI rejected");
    assert_eq!(stat_error.code, "fs.invalidUri");

    let watch_error = service
        .watch(FileWatchRequest {
            resource: invalid_file_uri("file", "relative/path"),
            recursive: false,
        })
        .await
        .expect_err("relative watch URI rejected");
    assert_eq!(watch_error.code, "fs.invalidUri");
}

#[tokio::test]
async fn watcher_service_parity_dispatches_watch_and_unwatch_aliases() {
    let root = unique_temp_path("watcher-service");
    std::fs::create_dir_all(&root).expect("watch root created");
    let mut registry = ServiceRegistry::new();
    vscode_atomic_tauri::file_service::register_file_watcher_services(&mut registry);

    assert!(registry.get(ServiceId::File).is_some());
    assert!(registry.get(ServiceId::Watcher).is_some());

    let watched = registry
        .dispatch_channel_call(request(
            "watcherService",
            "startWatching",
            json!({ "resource": file_uri(&root), "recursive": false }),
        ))
        .await
        .expect("watcher start succeeds");
    let watch_id = watched["watchId"].as_str().expect("watch id string");
    assert!(watch_id.starts_with("watch-"));

    registry
        .dispatch_channel_call(request(
            "fileWatcherService",
            "stopWatching",
            json!({ "watchId": watch_id }),
        ))
        .await
        .expect("watcher stop succeeds");

    let _ = std::fs::remove_dir_all(root);
}

#[cfg(windows)]
fn output_command() -> (String, Vec<String>) {
    (
        "cmd".to_string(),
        vec!["/C".to_string(), "echo terminal-parity".to_string()],
    )
}

#[cfg(not(windows))]
fn output_command() -> (String, Vec<String>) {
    (
        "sh".to_string(),
        vec!["-c".to_string(), "printf terminal-parity".to_string()],
    )
}

#[test]
fn terminal_service_parity_spawns_resizes_and_drains_output() {
    let service = RustTerminalService::default();
    let (command, args) = output_command();

    let status = service
        .spawn(SpawnTerminalRequest {
            id: Some("core-parity-terminal".to_string()),
            command,
            args,
            cwd: None,
            env: Default::default(),
            cols: Some(80),
            rows: Some(24),
        })
        .expect("spawn succeeds");
    assert_eq!(status.id, "core-parity-terminal");
    assert_eq!(status.state, TerminalLifecycleState::Running);

    let resized = service
        .resize(ResizeTerminalRequest {
            id: "core-parity-terminal".to_string(),
            cols: 120,
            rows: 40,
        })
        .expect("resize succeeds");
    assert_eq!(resized.size.expect("size present").cols, 120);

    let mut output = String::new();
    for _ in 0..40 {
        for event in service
            .drain_events("core-parity-terminal")
            .expect("drain events succeeds")
        {
            if event.stream == TerminalStreamKind::Stdout {
                output.push_str(&event.data);
            }
        }
        if output.contains("terminal-parity") {
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }

    assert!(output.contains("terminal-parity"));
}

#[test]
fn terminal_service_parity_covers_duplicate_empty_and_size_guards() {
    let service = RustTerminalService::default();
    let (command, args) = output_command();

    service
        .spawn(SpawnTerminalRequest {
            id: Some("guarded-terminal".to_string()),
            command: command.clone(),
            args: args.clone(),
            cwd: None,
            env: Default::default(),
            cols: Some(80),
            rows: Some(24),
        })
        .expect("first spawn succeeds");

    let duplicate = service
        .spawn(SpawnTerminalRequest {
            id: Some("guarded-terminal".to_string()),
            command: command.clone(),
            args: args.clone(),
            cwd: None,
            env: Default::default(),
            cols: Some(80),
            rows: Some(24),
        })
        .expect_err("duplicate terminal rejected");
    assert_eq!(duplicate.code, "terminal.exists");

    let empty_command = service
        .spawn(SpawnTerminalRequest {
            id: Some("empty-command-terminal".to_string()),
            command: "   ".to_string(),
            args: Vec::new(),
            cwd: None,
            env: Default::default(),
            cols: Some(80),
            rows: Some(24),
        })
        .expect_err("empty command rejected");
    assert_eq!(empty_command.code, "terminal.invalidRequest");

    let partial_size = service
        .spawn(SpawnTerminalRequest {
            id: Some("partial-size-terminal".to_string()),
            command: command.clone(),
            args: args.clone(),
            cwd: None,
            env: Default::default(),
            cols: Some(80),
            rows: None,
        })
        .expect_err("partial size rejected");
    assert_eq!(partial_size.code, "terminal.invalidRequest");

    let zero_size = service
        .resize(ResizeTerminalRequest {
            id: "guarded-terminal".to_string(),
            cols: 0,
            rows: 24,
        })
        .expect_err("zero cols rejected");
    assert_eq!(zero_size.code, "terminal.invalidRequest");
}

#[tokio::test]
async fn storage_and_configuration_parity_persist_across_service_instances() {
    let root = unique_temp_path("storage-config");
    let paths = UserDataPaths::new(root.clone());

    let config = JsonConfigurationService::new(paths.clone());
    config
        .call(request(
            "configuration",
            "updateValue",
            json!({ "key": "editor.tabSize", "value": 4 }),
        ))
        .await
        .expect("setting update succeeds");

    let persisted_config = JsonConfigurationService::new(paths.clone())
        .call(request(
            "configuration",
            "getValue",
            json!({ "key": "editor.tabSize" }),
        ))
        .await
        .expect("persisted setting read succeeds");
    assert_eq!(persisted_config, json!(4));
    assert!(paths.settings_resource().exists());

    let storage = JsonStorageService::new(paths.clone());
    storage
        .call(request(
            "storage",
            "setItem",
            json!({ "key": "welcome", "value": true }),
        ))
        .await
        .expect("global storage write succeeds");
    storage
        .call(request(
            "storage",
            "setItem",
            json!({ "scope": "workspace", "workspace": "file:///workspace", "key": "recent", "value": ["main.rs"] }),
        ))
        .await
        .expect("workspace storage write succeeds");

    let storage = JsonStorageService::new(paths.clone());
    let global = storage
        .call(request("storage", "getItem", json!({ "key": "welcome" })))
        .await
        .expect("global storage read succeeds");
    let workspace = storage
        .call(request(
            "storage",
            "getItem",
            json!({ "scope": "workspace", "workspace": "file:///workspace", "key": "recent" }),
        ))
        .await
        .expect("workspace storage read succeeds");

    assert_eq!(global, json!(true));
    assert_eq!(workspace, json!(["main.rs"]));
    assert!(paths.global_state_resource().exists());
    assert!(paths.workspace_state_resource("file:///workspace").exists());

    storage
        .call(request(
            "storage",
            "setItems",
            json!({ "items": { "a": 1, "b": 2 } }),
        ))
        .await
        .expect("batch storage write succeeds");
    let batch = storage
        .call(request(
            "storage",
            "getItems",
            json!({ "keys": ["a", "missing", "b"] }),
        ))
        .await
        .expect("batch storage read succeeds");
    assert_eq!(batch, json!({ "a": 1, "missing": null, "b": 2 }));

    storage
        .call(request("storage", "removeItems", json!({ "keys": ["a"] })))
        .await
        .expect("batch storage remove succeeds");
    assert_eq!(
        storage
            .call(request("storage", "getItem", json!({ "key": "a" })))
            .await
            .expect("removed key read succeeds"),
        Value::Null
    );

    storage
        .call(request("storage", "clear", json!({})))
        .await
        .expect("storage clear succeeds");
    assert_eq!(
        storage
            .call(request("storage", "keys", json!({})))
            .await
            .expect("keys after clear succeeds"),
        json!([])
    );

    let _ = std::fs::remove_dir_all(root);
}

#[tokio::test]
async fn storage_and_configuration_parity_covers_aliases_paths_and_invalid_arguments() {
    let root = unique_temp_path("storage-config-aliases");
    let paths = UserDataPaths::new(root.clone());
    let config = JsonConfigurationService::new(paths.clone());
    let storage = JsonStorageService::new(paths.clone());

    let config_paths = config
        .call(request("configuration", "getStatePaths", json!({})))
        .await
        .expect("configuration paths succeeds");
    assert_eq!(config_paths["root"], root.to_string_lossy().to_string());
    assert_eq!(
        config_paths["settingsResource"],
        paths.settings_resource().to_string_lossy().to_string()
    );

    config
        .call(request(
            "configuration",
            "setValue",
            json!({ "key": "workbench.colorTheme", "value": "Default Dark Modern" }),
        ))
        .await
        .expect("configuration setValue alias succeeds");
    assert_eq!(
        config
            .call(request(
                "configuration",
                "get",
                json!({ "key": "workbench.colorTheme" }),
            ))
            .await
            .expect("configuration get alias succeeds"),
        json!("Default Dark Modern")
    );

    let invalid_config = config
        .call(request(
            "configuration",
            "updateValue",
            json!({ "key": "", "value": true }),
        ))
        .await
        .expect_err("empty configuration key rejected");
    assert_eq!(invalid_config.code, "config.invalidKey");

    let storage_paths = storage
        .call(request("storage", "paths", json!({})))
        .await
        .expect("storage paths succeeds");
    assert_eq!(
        storage_paths["globalStateResource"],
        paths.global_state_resource().to_string_lossy().to_string()
    );

    storage
        .call(request(
            "storage",
            "store",
            json!({ "key": "alias", "value": 1 }),
        ))
        .await
        .expect("storage store alias succeeds");
    assert_eq!(
        storage
            .call(request("storage", "get", json!({ "key": "alias" })))
            .await
            .expect("storage get alias succeeds"),
        json!(1)
    );
    storage
        .call(request(
            "storage",
            "mset",
            json!({ "items": { "a": true, "b": false } }),
        ))
        .await
        .expect("storage mset alias succeeds");
    assert_eq!(
        storage
            .call(request(
                "storage",
                "mget",
                json!({ "keys": ["a", "b", "missing"] })
            ))
            .await
            .expect("storage mget alias succeeds"),
        json!({ "a": true, "b": false, "missing": null })
    );
    storage
        .call(request("storage", "mdelete", json!({ "keys": ["a", "b"] })))
        .await
        .expect("storage mdelete alias succeeds");
    assert_eq!(
        storage
            .call(request("storage", "getKeys", json!({})))
            .await
            .expect("storage getKeys alias succeeds"),
        json!(["alias"])
    );

    let workspace_error = storage
        .call(request(
            "storage",
            "setItem",
            json!({ "scope": "workspace", "key": "missing-workspace", "value": true }),
        ))
        .await
        .expect_err("workspace storage without workspace rejected");
    assert_eq!(workspace_error.code, "storage.invalidArgument");

    let _ = std::fs::remove_dir_all(root);
}
