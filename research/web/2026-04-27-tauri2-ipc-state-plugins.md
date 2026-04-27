---
source_url: https://v2.tauri.app/
fetched_at: 2026-04-27
fetch_method: html-parse (playwright-cli)
topic: Tauri 2.x command/IPC system, state management, plugin architecture, async commands, event emission
---

# Tauri 2.x: IPC, State, Plugins, Async Commands, Events

Source pages fetched:
- https://v2.tauri.app/develop/calling-rust/
- https://v2.tauri.app/develop/calling-frontend/
- https://v2.tauri.app/develop/state-management/
- https://v2.tauri.app/develop/plugins/
- https://v2.tauri.app/concept/inter-process-communication/
- https://v2.tauri.app/concept/architecture/

## Commands / IPC

### Basic #[tauri::command]

```rust
#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JavaScript!");
}
```

Registered via `invoke_handler`:
```rust
tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![my_custom_command])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
```

### Arguments & serde::Deserialize

Arguments passed as camelCase JSON object; any type implementing `serde::Deserialize` is valid.

### Return values & serde::Serialize

Return any type implementing `serde::Serialize`. Use `tauri::ipc::Response` for raw binary (bypasses JSON serialization).

### Error handling pattern

```rust
#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error)
}
impl serde::Serialize for Error { ... }
#[tauri::command]
fn my_cmd() -> Result<(), Error> { ... }
```

### Async commands

```rust
#[tauri::command]
async fn my_custom_command(value: String) -> String {
  some_async_function().await;
  value
}
```

Executed via `async_runtime::spawn`. Borrowed args (`&str`, `State<'_, T>`) require workarounds: convert to owned type OR wrap return in `Result`.

### Channels (streaming)

```rust
use tokio::io::AsyncReadExt;
#[tauri::command]
async fn load_image(path: std::path::PathBuf, reader: tauri::ipc::Channel<&[u8]>) {
  let mut file = tokio::fs::File::open(path).await.unwrap();
  let mut chunk = vec![0; 4096];
  loop {
    let len = file.read(&mut chunk).await.unwrap();
    if len == 0 { break; }
    reader.send(&chunk).unwrap();
  }
}
```

### Raw request access

```rust
#[tauri::command]
fn upload(request: tauri::ipc::Request) -> Result<(), Error> {
  let tauri::ipc::InvokeBody::Raw(data) = request.body() else { ... };
  let auth = request.headers().get("Authorization").ok_or(...)?;
  Ok(())
}
```

## State Management

### Registering state

```rust
use tauri::{Builder, Manager};
Builder::default()
  .setup(|app| {
    app.manage(AppData { welcome_message: "Welcome!" });
    Ok(())
  })
```

### Accessing state

```rust
let data = app.state::<AppData>();
```

### Mutability pattern

```rust
use std::sync::Mutex;
app.manage(Mutex::new(AppState::default()));
// In command:
#[tauri::command]
fn increase_counter(state: State<'_, Mutex<AppState>>) -> u32 {
  let mut state = state.lock().unwrap();
  state.counter += 1;
  state.counter
}
```

For async commands with Tokio Mutex:
```rust
#[tauri::command]
async fn increase_counter(state: State<'_, Mutex<AppState>>) -> Result<u32, ()> {
  let mut state = state.lock().await;
  state.counter += 1;
  Ok(state.counter)
}
```

### Manager trait for non-command contexts

```rust
fn on_window_event(window: &Window, _event: &WindowEvent) {
  let state = window.app_handle().state::<Mutex<AppState>>();
  state.lock().unwrap().counter += 1;
}
```

Note: No need for `Arc` wrapping—Tauri manages that internally. Use type aliases to prevent mismatched-type runtime panics.

## Event System (Calling Frontend from Rust)

### Emitter trait

```rust
use tauri::{AppHandle, Emitter};
#[tauri::command]
fn download(app: AppHandle, url: String) {
  app.emit("download-started", &url).unwrap();
  app.emit("download-finished", &url).unwrap();
}
```

### Targeted emission

```rust
app.emit_to("login", "login-result", result).unwrap();
app.emit_filter("open-file", path, |target| match target {
  EventTarget::WebviewWindow { label } => label == "main",
  _ => false,
}).unwrap();
```

### Payloads must be Clone + Serialize

```rust
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct DownloadStarted<'a> {
  url: &'a str,
  download_id: usize,
  content_length: usize,
}
```

### Rust-side listeners

```rust
use tauri::Listener;
app.listen("download-started", |event| {
  if let Ok(payload) = serde_json::from_str::<DownloadStarted>(&event.payload()) { ... }
});
app.unlisten(event_id);
app.once("ready", |event| { println!("app is ready"); });
```

### Channels vs Events

- Events: fire-and-forget, multi-consumer/producer, small JSON payloads only, no capability enforcement
- Channels: ordered, optimized for streaming, used for large/binary data; recommended for download progress, child process output, WebSocket messages

## Plugin Architecture

### Plugin structure

```
tauri-plugin-[name]/
├── src/
│   ├── commands.rs    - Tauri commands (webview-callable)
│   ├── desktop.rs     - Desktop-specific implementation
│   ├── lib.rs         - Re-exports, state setup, plugin builder
│   ├── mobile.rs      - Mobile-specific implementation
│   └── models.rs      - Shared structs
├── permissions/       - Auto/manual permission files
├── Cargo.toml
└── package.json
```

### Plugin builder

```rust
use serde::Deserialize;
use tauri::plugin::{Builder, TauriPlugin};

#[derive(Deserialize)]
pub struct Config { timeout: usize }

pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
  Builder::<R, Config>::new("<plugin-name>")
    .setup(|app, api| {
      let timeout = api.config().timeout;
      app.manage(DummyStore(Default::default()));
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![commands::upload])
    .build()
}
```

### Lifecycle hooks

- `setup`: initialization, state management, background tasks
- `on_navigation`: URL validation/change tracking (return `false` to cancel)
- `on_webview_ready`: per-window initialization scripts
- `on_event`: handle `RunEvent::ExitRequested`, `RunEvent::Exit`, etc.
- `on_drop`: cleanup on plugin destruction

### Plugin commands

```rust
// src/commands.rs
use tauri::{command, ipc::Channel, AppHandle, Runtime, Window};
#[command]
pub async fn upload<R: Runtime>(app: AppHandle<R>, window: Window<R>, on_progress: Channel, url: String) {
  on_progress.send(100).unwrap();
}
```

### Permission system

```toml
# permissions/start-server.toml
[[permission]]
identifier = "allow-start-server"
description = "Enables the start_server command."
commands.allow = ["start_server"]
```

COMMANDS autogeneration in `build.rs`:
```rust
const COMMANDS: &[&str] = &["upload"];
fn main() { tauri_plugin::Builder::new(COMMANDS).build(); }
```

### Command scope

```rust
async fn spawn<R: tauri::Runtime>(app: AppHandle<R>, command_scope: CommandScope<'_, Entry>) -> Result<()> {
  let allowed = command_scope.allows();
  let denied = command_scope.denies();
  todo!()
}
```
