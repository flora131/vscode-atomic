# Partition 9: `cli/` (Rust) — Tauri 2.x Port Research

> Mission: map VS Code's `cli/` Rust codebase to Tauri 2.x primitives (command/IPC, state, plugins, async commands, event emission) plus companion crates `tokio`, `serde`, `hyper`/`reqwest`.
>
> Sources: https://v2.tauri.app/ (live, 2026-04-27), `/Users/norinlavaee/vscode-atomic/cli/Cargo.toml`, `/Users/norinlavaee/vscode-atomic/cli/src/` (all files read directly).

---

## Library Card 1 — `tauri` 2.x: Command / IPC System

**Crate**: `tauri` (latest 2.x)
**Authoritative docs**: https://v2.tauri.app/develop/calling-rust/ (fetched 2026-04-27)
**IPC concept**: https://v2.tauri.app/concept/inter-process-communication/

### What it does
Tauri's IPC layer uses **Asynchronous Message Passing** (JSON-RPC-like protocol under the hood). The frontend calls Rust via `invoke()`, which is resolved as a promise. All arguments and return values must be `serde::Serialize` / `serde::Deserialize`.

### Core macro: `#[tauri::command]`

```rust
// src-tauri/src/lib.rs
#[tauri::command]
fn my_custom_command(invoke_message: String) -> String {
    format!("Got: {invoke_message}")
}
```

Registration — only ONE call to `invoke_handler` is valid; use `generate_handler![]` for multiple:

```rust
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![cmd_a, cmd_b])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
```

### Error handling pattern (matches `thiserror` already in `cli/Cargo.toml` line 51)

```rust
// cli/Cargo.toml:51  thiserror = "1.0.40"
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
impl serde::Serialize for Error {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}
#[tauri::command]
fn my_cmd() -> Result<(), Error> { Ok(()) }
```

### Accessing managed state from a command

```rust
#[tauri::command]
fn my_cmd(state: tauri::State<MyState>) {
    assert_eq!(state.0, "some state value");
}
```

### Accessing `AppHandle` / `WebviewWindow` in commands (dependency injection)

```rust
#[tauri::command]
async fn my_cmd<R: Runtime>(app: AppHandle<R>, window: WebviewWindow<R>) { ... }
```

### Raw binary response (avoids JSON overhead — useful for file streaming)

```rust
use tauri::ipc::Response;
#[tauri::command]
fn read_file() -> Response {
    let data = std::fs::read("/path/to/file").unwrap();
    tauri::ipc::Response::new(data)
}
```

### Raw request body + headers

```rust
#[tauri::command]
fn upload(request: tauri::ipc::Request) -> Result<(), Error> {
    let tauri::ipc::InvokeBody::Raw(data) = request.body() else { ... };
    let auth = request.headers().get("Authorization").ok_or(...)?;
    Ok(())
}
```

### Mapping to existing `cli/` code

| Existing CLI pattern | Tauri 2.x equivalent |
|---|---|
| `rpc::RpcBuilder::register_sync` (`cli/src/rpc.rs:101`) | `#[tauri::command]` (sync fn) |
| `rpc::RpcBuilder::register_async` (`cli/src/rpc.rs:148`) | `#[tauri::command]` on `async fn` |
| `rpc::RpcBuilder::register_duplex` (`cli/src/rpc.rs:203`) | `tauri::ipc::Channel<T>` parameter |
| `JsonRpcSerializer` in `cli/src/json_rpc.rs:24` | Replaced by Tauri's internal JSON-RPC codec |
| `MsgPackSerializer` in `cli/src/msgpack_rpc.rs:25` | No direct equivalent; must use JSON or `tauri::ipc::Response` for binary |
| `RpcDispatcher::dispatch` (`cli/src/rpc.rs:409`) | Replaced by `invoke_handler` + `generate_handler![]` |
| `MaybeSync` enum (`cli/src/rpc.rs:708`) | Not needed; Tauri handles sync/async dispatch automatically |

---

## Library Card 2 — `tauri` 2.x: State Management

**Docs**: https://v2.tauri.app/develop/state-management/ (fetched 2026-04-27)

### Pattern

State is registered once with `app.manage(T)` and injected into commands or retrieved via the `Manager` trait.

```rust
use tauri::{Builder, Manager};
#[derive(Default)]
struct AppState { counter: u32 }

Builder::default()
    .setup(|app| {
        app.manage(Mutex::new(AppState::default()));
        Ok(())
    })
```

### Mutability — `std::sync::Mutex` (preferred over `tokio::sync::Mutex`)

Per Tokio docs (quoted in Tauri docs): "it is ok and often preferred to use the ordinary Mutex from the standard library in asynchronous code."  Only use async mutex if you need to hold the guard across `.await` points.

```rust
#[tauri::command]
fn increase_counter(state: tauri::State<Mutex<AppState>>) -> u32 {
    let mut s = state.lock().unwrap();
    s.counter += 1;
    s.counter
}
```

### No `Arc` needed

Tauri wraps managed state in `Arc` internally. `AppHandle` is cheap to clone and can be moved into threads:

```rust
let handle = app.handle().clone();
std::thread::spawn(move || {
    let state = handle.state::<Mutex<AppState>>();
    state.lock().unwrap().counter += 1;
});
```

### Type-alias pattern to prevent runtime panic

```rust
type AppState = Mutex<AppStateInner>;   // never double-wrap
```

### Mapping to existing `cli/` code

| Existing CLI pattern | Tauri 2.x equivalent |
|---|---|
| `PersistedState<T>` with `Arc<Mutex<PersistedStateContainer<T>>>` (`cli/src/state.rs:94`) | `app.manage(Mutex::new(...))` + `tauri::State<Mutex<T>>` |
| `LauncherPaths` passed via `CommandContext` (`cli/src/commands/context.rs:10`) | Manage as Tauri state; inject via `tauri::State<LauncherPaths>` |
| `CommandContext` struct (`cli/src/commands/context.rs:11`) | Decompose into individual `app.manage()` calls; each field becomes a managed state type |
| `reqwest::Client` in `CommandContext` (`cli/src/commands/context.rs:15`) | `app.manage(reqwest::Client::new())` |

---

## Library Card 3 — `tauri` 2.x: Plugin Architecture

**Docs**: https://v2.tauri.app/develop/plugins/ (fetched 2026-04-27, last updated Apr 6 2026)

### Plugin scaffolding

```bash
npx @tauri-apps/cli plugin new [name]
```

Generates `tauri-plugin-[name]/` with `src/commands.rs`, `src/lib.rs`, `src/desktop.rs`, `src/mobile.rs`, `permissions/`.

### Plugin builder

```rust
use tauri::plugin::{Builder, TauriPlugin};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config { timeout: usize }

pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
    Builder::<R, Config>::new("plugin-name")
        .setup(|app, api| {
            let _timeout = api.config().timeout;
            app.manage(SomeState::default());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::upload])
        .build()
}
```

### Lifecycle hooks for port

| Hook | When | Port use case |
|---|---|---|
| `setup` | Plugin initialized | Register tunnel state, spawn background tokio tasks |
| `on_navigation` | Webview navigates | Block unauthorized URLs |
| `on_webview_ready` | New window created | Inject per-window JS |
| `on_event` | Event loop events | Handle `RunEvent::ExitRequested` to stop tunnels cleanly |
| `on_drop` | Plugin destroyed | Graceful cleanup |

### Plugin commands with Channel (streaming)

```rust
// src/commands.rs
use tauri::{command, ipc::Channel, AppHandle, Runtime, Window};

#[command]
pub async fn upload<R: Runtime>(
    app: AppHandle<R>,
    window: Window<R>,
    on_progress: Channel,
    url: String,
) {
    on_progress.send(100).unwrap();
}
```

### Permission autogeneration

```rust
// build.rs
const COMMANDS: &[&str] = &["upload", "start_tunnel", "stop_tunnel"];
fn main() { tauri_plugin::Builder::new(COMMANDS).build(); }
```

Generates `allow-upload`, `deny-upload`, etc. automatically.

### Scope system for CLI commands

```rust
// src/commands.rs
use tauri::ipc::CommandScope;
async fn spawn<R: Runtime>(app: AppHandle<R>, scope: CommandScope<'_, Entry>) -> Result<()> {
    let allowed = scope.allows();
    // ...
}
```

### Mapping VS Code CLI modules to plugins

| CLI module | Suggested Tauri plugin |
|---|---|
| `cli/src/tunnels/` (`tunnels.rs`, `dev_tunnels.rs`) | `tauri-plugin-vscode-tunnel` |
| `cli/src/auth.rs` | `tauri-plugin-vscode-auth` |
| `cli/src/commands/serve_web.rs` | `tauri-plugin-vscode-serve-web` |
| `cli/src/commands/update.rs` / `self_update.rs` | `tauri-plugin-vscode-update` |
| `cli/src/commands/version.rs` | Part of core app commands |

---

## Library Card 4 — `tauri` 2.x: Async Commands

**Docs**: https://v2.tauri.app/develop/calling-rust/#async-commands (fetched 2026-04-27)

### Declaration

```rust
#[tauri::command]
async fn my_custom_command(value: String) -> String {
    some_async_function().await;
    value
}
```

Executed via `tauri::async_runtime::spawn` (wraps Tokio internally).

### Borrowed arguments workaround

Commands without `async` run on the main thread. Async commands cannot use borrowed types directly:

**Option A** — Convert to owned:
```rust
#[tauri::command]
async fn cmd(value: String) -> String { value }
```

**Option B** — Wrap return in `Result`:
```rust
#[tauri::command]
async fn cmd(value: &str) -> Result<String, ()> {
    Ok(value.to_string())
}
```

### Channels for streaming (CLI tunnel output → frontend)

```rust
#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data", rename_all_fields = "camelCase")]
enum TunnelEvent<'a> {
    Started { url: &'a str },
    Progress { bytes: usize },
    Finished,
}

#[tauri::command]
async fn run_tunnel(url: String, on_event: tauri::ipc::Channel<TunnelEvent<'_>>) {
    on_event.send(TunnelEvent::Started { url: &url }).unwrap();
    // ... stream progress events
    on_event.send(TunnelEvent::Finished).unwrap();
}
```

### Mapping to existing CLI patterns

| CLI pattern | File:line | Tauri async equivalent |
|---|---|---|
| `pub async fn command_shell(ctx: CommandContext, args: CommandShellArgs) -> Result<i32, AnyError>` | `cli/src/commands/tunnels.rs:145` | `#[tauri::command] async fn command_shell(state: State<...>, ...) -> Result<i32, String>` |
| `tokio::select!` loop in `start_json_rpc` | `cli/src/json_rpc.rs:61` | Replaced by Tauri's built-in command/event dispatch; no manual select loop needed |
| `tokio::spawn` in `RpcDispatcher::register_stream` | `cli/src/rpc.rs:483` | `tauri::ipc::Channel<T>` with internal tokio spawn |
| `register_async` with `BoxFuture` | `cli/src/rpc.rs:148` | `#[tauri::command] async fn` |

---

## Library Card 5 — `tauri` 2.x: Event Emission (Calling Frontend from Rust)

**Docs**: https://v2.tauri.app/develop/calling-frontend/ (fetched 2026-04-27, last updated May 12 2025)

### Emitter trait

`AppHandle` and `WebviewWindow` both implement `Emitter`. Payloads must be `Clone + Serialize`.

```rust
use tauri::{AppHandle, Emitter};

#[tauri::command]
fn download(app: AppHandle, url: String) {
    app.emit("download-started", &url).unwrap();
    for progress in [1, 15, 50, 80, 100] {
        app.emit("download-progress", progress).unwrap();
    }
    app.emit("download-finished", &url).unwrap();
}
```

### Targeted emission

```rust
// To a specific webview label:
app.emit_to("main", "tunnel-status", status).unwrap();

// To a filtered set of webviews:
app.emit_filter("file-changed", path, |target| match target {
    EventTarget::WebviewWindow { label } => label == "editor",
    _ => false,
}).unwrap();
```

### Listening on the Rust side

```rust
use tauri::Listener;
app.listen("user-action", |event| {
    if let Ok(payload) = serde_json::from_str::<UserAction>(&event.payload()) { ... }
});
// Once-only listener:
app.once("ready", |_event| { println!("app ready"); });
// Unlisten:
let id = app.listen("tick", |_| {});
app.unlisten(id);
```

### Events vs Channels: decision guide for CLI port

| Scenario | Recommendation |
|---|---|
| Tunnel status updates (small JSON, push) | `app.emit(...)` via event system |
| File download stream / chunk progress | `tauri::ipc::Channel<T>` |
| Auth token update | `app.emit(...)` |
| Log streaming from child process | `tauri::ipc::Channel<T>` |
| WebSocket message relay | `tauri::ipc::Channel<T>` |

### Mapping CLI patterns

| CLI pattern | File | Tauri equivalent |
|---|---|---|
| `RpcCaller::notify(method, params)` | `cli/src/rpc.rs:331` | `app.emit("event-name", payload)` |
| `RpcCaller::call(method, params)` | `cli/src/rpc.rs:342` | `#[tauri::command]` round-trip via `invoke()` |
| Duplex stream writes via `write_tx.send(...)` | `cli/src/rpc.rs:458` | `Channel::send(data)` |
| `BroadcastLogSink` in singleton server | `cli/src/tunnels/singleton_server.rs` | `AppHandle::emit(...)` to all webviews |

---

## Library Card 6 — `tokio` 1.x (already in `cli/Cargo.toml:18`)

**Cargo.toml line 18**: `tokio = { version = "1.38.2", features = ["full"] }`

### How Tauri integrates Tokio

Tauri uses Tokio as its async runtime. `#[tauri::command]` async functions are dispatched via `tauri::async_runtime::spawn`, which wraps `tokio::task::spawn`. The CLI's existing `tokio::spawn`, `tokio::select!`, `tokio::sync::mpsc`, and `tokio::io` patterns are all directly compatible.

### Key CLI patterns and their Tauri context

| Pattern | File:line | Notes for port |
|---|---|---|
| `tokio::net::TcpListener::bind` | `cli/src/commands/tunnels.rs:197` | Usable unchanged inside `#[tauri::command]` async fns |
| `tokio::io::stdin()` / `stderr()` | `cli/src/commands/tunnels.rs:208` | Use for sidecar / subprocess piping; Tauri has `tauri-plugin-shell` |
| `tokio::sync::watch` | `cli/src/commands/tunnels.rs:19` | Use for internal state broadcast between tasks |
| `tokio::io::AsyncReadExt` / `AsyncWriteExt` | `cli/src/msgpack_rpc.rs:9` | Unchanged; `tauri::ipc::Channel` uses tokio internally |
| `tokio::sync::oneshot` | `cli/src/rpc.rs:20` | Still valid for command result futures |
| `tokio::sync::mpsc` | `cli/src/rpc.rs:21` | Still valid; used internally by Tauri's Channel |

### Async runtime preference

Tauri docs: "it is ok and often preferred to use `std::sync::Mutex` instead of `tokio::sync::Mutex` in async code." Use `tokio::sync::Mutex` only when holding guards across `.await` points (e.g., async DB connections).

---

## Library Card 7 — `serde` + `serde_json` + `rmp-serde` (already in `cli/Cargo.toml`)

**Cargo.toml lines**: `serde:16 → 1.0.163`, `serde_json:34 → 1.0.96`, `rmp-serde:29 → 1.1.1`

### serde in Tauri IPC

Every Tauri command argument and return value goes through `serde`. The `#[tauri::command]` macro derives the necessary glue code from `serde::Deserialize` and `serde::Serialize` automatically.

### Existing CLI serde usage that maps cleanly

| CLI usage | File | Tauri compatibility |
|---|---|---|
| `#[derive(Serialize, Deserialize)]` on RPC types | `cli/src/rpc.rs:656` | Direct; no changes needed |
| `serde_json::from_str` in event listeners | `cli/src/rpc.rs:131` | Direct; Tauri calls this internally for event payloads |
| `serde_bytes` for binary segments | `cli/src/rpc.rs:652` | Replace with `tauri::ipc::Response` or `Channel<Vec<u8>>` |
| `rmp_serde::to_vec_named` (MsgPack) | `cli/src/msgpack_rpc.rs:29` | No direct Tauri equivalent; must switch to JSON or binary `Response` |
| `serde(rename_all = "camelCase")` | Various | Required for Tauri IPC since JS uses camelCase |

### MsgPack migration path

The CLI uses MsgPack RPC (`cli/src/msgpack_rpc.rs`) for the server communication protocol. When porting:
1. Replace `MsgPackSerializer` with Tauri's JSON codec for command parameters.
2. For bulk binary data (server artifacts, extensions), use `tauri::ipc::Response::new(bytes)` which bypasses JSON serialization entirely.
3. `rmp-serde` can remain for off-Tauri-IPC uses (e.g., local protocol with spawned server child process).

---

## Library Card 8 — `hyper` 0.14 + `reqwest` 0.11 (already in `cli/Cargo.toml`)

**Cargo.toml lines**: `hyper:40 → 0.14.26 [server, http1, runtime]`, `reqwest:18 → 0.11.22 [json, stream, native-tls]`

### Role in CLI

- `hyper`: embedded HTTP/1 server (`cli/src/util/http.rs:13`) — used for the local serve-web command and the extension server
- `reqwest`: HTTP client (`cli/src/util/http.rs:136`) — used for API calls to VS Code marketplace, update checks, tunnel control plane

### Tauri compatibility

Tauri does not replace or conflict with `hyper` or `reqwest`. Both can be used as-is inside Tauri commands and plugin setup closures. However, Tauri provides `tauri-plugin-http` as an optional wrapper over `reqwest` for frontend-accessible HTTP; for backend-only use the raw crates are preferred.

### Managing the `reqwest::Client` in Tauri

The CLI's `CommandContext.http` field (`cli/src/commands/context.rs:15`) should become managed state:

```rust
// In plugin setup or app setup:
app.manage(
    reqwest::ClientBuilder::new()
        .user_agent(get_default_user_agent())
        .build()
        .unwrap()
);

// In commands:
#[tauri::command]
async fn fetch_update(client: tauri::State<'_, reqwest::Client>, url: String) -> Result<String, String> {
    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
    resp.text().await.map_err(|e| e.to_string())
}
```

### `hyper` server lifecycle under Tauri

The `serve_web` command spawns an embedded hyper server (`cli/src/commands/serve_web.rs`). In Tauri, this spawns inside `setup` or inside an async command, and the server handle can be stored as managed state for graceful shutdown:

```rust
use std::sync::Mutex;
struct ServeWebHandle(Option<tokio::task::JoinHandle<()>>);
app.manage(Mutex::new(ServeWebHandle(None)));
```

On `RunEvent::ExitRequested`, the plugin `on_event` hook aborts the task.

---

## Dependency Version Gap Analysis

The CLI's `Cargo.toml` pins older versions of several crates that conflict with Tauri 2.x requirements:

| Crate | CLI version | Tauri 2.x typically requires | Action |
|---|---|---|---|
| `reqwest` | `0.11.22` | `0.12.x` (Tauri 2 uses reqwest 0.12 internally) | Upgrade; breaking changes in builder API |
| `hyper` | `0.14.26` | `1.x` (used by reqwest 0.12) | Upgrade; major API rewrite |
| `tokio` | `1.38.2` | `1.x` (compatible) | No change needed |
| `serde` | `1.0.163` | `1.x` (compatible) | No change needed |
| `serde_json` | `1.0.96` | `1.x` (compatible) | No change needed |
| `thiserror` | `1.0.40` | `1.x` or `2.x` | Minor upgrade may be needed |
| `opentelemetry` | `0.19.0` | `0.19` is old; `0.25+` current | Upgrade if telemetry is retained |

---

## Tauri 2.x Architecture Summary for `cli/` Port

```
cli/src/
├── rpc.rs          → REPLACED by tauri::command + tauri::ipc::Channel
├── json_rpc.rs     → REPLACED by Tauri's internal JSON-RPC codec
├── msgpack_rpc.rs  → KEEP for non-Tauri IPC; binary payloads use tauri::ipc::Response
├── state.rs        → REPLACE PersistedState with app.manage(Mutex<T>); keep LauncherPaths
├── commands/
│   ├── context.rs  → DECOMPOSE into managed state types
│   ├── tunnels.rs  → PORT as tauri-plugin-vscode-tunnel commands
│   ├── serve_web.rs→ PORT as tauri-plugin-vscode-serve-web commands
│   └── update.rs  → PORT as tauri-plugin-vscode-update commands
├── auth.rs         → PORT as tauri-plugin-vscode-auth (setup manages keyring state)
└── util/http.rs    → KEEP reqwest/hyper; manage reqwest::Client as Tauri state
```

---

## Key File References

- `/Users/norinlavaee/vscode-atomic/cli/Cargo.toml` — full dependency manifest
- `/Users/norinlavaee/vscode-atomic/cli/src/rpc.rs` — custom JSON-RPC/MsgPack RPC dispatcher (lines 1–756)
- `/Users/norinlavaee/vscode-atomic/cli/src/json_rpc.rs` — JSON serializer + `start_json_rpc` loop (lines 1–107)
- `/Users/norinlavaee/vscode-atomic/cli/src/msgpack_rpc.rs` — MsgPack serializer (lines 1–60+)
- `/Users/norinlavaee/vscode-atomic/cli/src/state.rs` — `PersistedState<T>` with `Arc<Mutex<...>>` (lines 88–134)
- `/Users/norinlavaee/vscode-atomic/cli/src/commands/context.rs` — `CommandContext` (lines 10–15)
- `/Users/norinlavaee/vscode-atomic/cli/src/util/http.rs` — `reqwest` + `hyper` HTTP utilities (lines 1–340+)
- `/Users/norinlavaee/vscode-atomic/cli/src/commands/tunnels.rs` — async tunnel commands (lines 145–427+)
- `/Users/norinlavaee/vscode-atomic/cli/src/lib.rs` — module re-exports (lines 1–27)

---

## Prose Summary

Porting VS Code's `cli/` crate from a standalone Tokio binary to a Tauri 2.x application requires substituting three of the CLI's most fundamental infrastructure layers with Tauri's built-in equivalents, while leaving the business logic largely intact.

**IPC and RPC dispatch** (`cli/src/rpc.rs`, `json_rpc.rs`, `msgpack_rpc.rs`) is the most invasive change. The CLI implements its own JSON-RPC and MsgPack-RPC protocol from scratch—a `RpcBuilder` that registers sync, async, and duplex stream handlers, dispatches incoming bytes, and manages outbound call correlation tables. In Tauri 2.x, this entire layer collapses to `#[tauri::command]` annotations and `tauri::generate_handler![]` registration. The `MaybeSync` dispatch enum, `PartialIncoming` deserialization, and the `write_loop` for duplex streams all disappear; Tauri handles them internally. The one exception is the MsgPack protocol used to communicate with the spawned VS Code server child process—that can remain as a non-Tauri off-channel protocol since `rmp-serde` has no conflict with Tauri.

**State management** (`cli/src/state.rs`, `cli/src/commands/context.rs`) maps cleanly. The CLI's `PersistedState<T>` wraps an `Arc<Mutex<PersistedStateContainer<T>>>` for thread-safe disk-persisted state. In Tauri, the same pattern is `app.manage(Mutex::new(T))` accessed via `tauri::State<Mutex<T>>` injection. The `CommandContext` struct—which bundles `Logger`, `LauncherPaths`, `CliCore` args, and a `reqwest::Client`—should be decomposed into individual managed state types so that each command can declare only the dependencies it needs.

**Event emission** replaces the CLI's `RpcCaller::notify` pattern. For lightweight push events (tunnel status, auth state changes, log lines), `app.emit("event-name", payload)` via the `Emitter` trait delivers to all frontend listeners. For high-throughput ordered streams (download chunks, child-process stdout, WebSocket relay), `tauri::ipc::Channel<T>` provides the same semantics as the CLI's `register_duplex` / `Streams` write-loop with far less boilerplate and proper backpressure.

**Tokio**, `serde`, and `serde_json` require no changes—they are already compatible with Tauri 2.x and their usage patterns within the CLI (spawn, select!, mpsc channels, derive macros) work identically inside Tauri commands and plugin setup closures. The notable upgrade burden is `reqwest` (0.11 → 0.12) and `hyper` (0.14 → 1.x), which Tauri 2.x pulls in transitively at newer versions, requiring manual client builder API adjustments in `cli/src/util/http.rs`.

The recommended decomposition partitions the CLI's existing modules into Tauri plugins: `tauri-plugin-vscode-tunnel` (wrapping the tunnels module), `tauri-plugin-vscode-auth` (wrapping `auth.rs` and `keyring`), and `tauri-plugin-vscode-serve-web` (wrapping the embedded hyper server). Each plugin's `setup` hook manages its own state and registers its commands; `on_event` handles graceful shutdown on `RunEvent::ExitRequested`. This structure preserves the existing Rust business logic while integrating cleanly with Tauri's security model (permissions per command, capability scopes).
