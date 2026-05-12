### Files Analysed

- `cli/Cargo.toml` — dependency manifest (92 lines)
- `cli/src/bin/code/main.rs` — `#[tokio::main]` entry point, command dispatch (204 lines)
- `cli/src/lib.rs` — module re-export surface (28 lines)
- `cli/src/rpc.rs` — generic transport-agnostic RPC dispatcher with pluggable serializers (755 lines)
- `cli/src/async_pipe.rs` — cross-platform IPC abstraction: Unix sockets vs Windows named pipes (232 lines)
- `cli/src/state.rs` — `LauncherPaths` (directory layout) + `PersistedState<T>` (JSON-on-disk state) (252 lines)
- `cli/src/tunnels/code_server.rs` — `ServerBuilder`, process spawn/monitor, `CodeServerOrigin` (898 lines)
- `cli/src/tunnels/control_server.rs` — msgpack RPC multiplexer: fs ops, spawn, port forward, serve (1479 lines)
- `cli/src/commands/agent_host.rs` — agent host launcher, WebSocket proxy, lockfile discovery (369 lines)
- `cli/src/commands/serve_web.rs` — hyper HTTP server proxying requests to VS Code server socket (960 lines)

---

### Per-File Notes

#### `cli/Cargo.toml`

- **Role:** Declares all production dependencies for the `code-cli` binary and `cli` library crate.
- **Key symbols:** `[lib]` at line 7 (name = "cli", path = "src/lib.rs"); `[[bin]]` at line 11 (name = "code"); `default-run = "code"` at line 5.
- **Control flow:** Not applicable (build manifest).
- **Data flow:** Not applicable.
- **Dependencies:** `tokio = { version = "1.52", features = ["full"] }` (line 19); `clap = { version = "4.3.0", features = ["derive", "env"] }` (line 16); `reqwest = { version = "0.13", features = ["json", "stream", "native-tls"] }` (line 18); `hyper = { version = "1", features = ["server", "http1", "client"] }` (line 39); `hyper-util` (line 40); `tokio-tungstenite` (line 42); `rmp-serde = "1.1.1"` (line 27) for msgpack; `tunnels` from `https://github.com/microsoft/dev-tunnels` rev `64048c1` (line 36) for dev-tunnels relay; `keyring = "2.0.3"` (line 37) for OS credential store; `serde`/`serde_json` (lines 25–26); `ahp`/`ahp-types`/`ahp-ws` (lines 59–61) for the agent host protocol; platform-specific: `winreg`/`winapi`/`windows-sys` (lines 69–71), `core-foundation` (line 74), `zbus` (line 77). No Tauri dependency anywhere in the file.

---

#### `cli/src/bin/code/main.rs`

- **Role:** The binary entry point. Parses CLI arguments, builds a `CommandContext`, and dispatches to command handler functions.
- **Key symbols:** `main()` (`main.rs:27`) annotated `#[tokio::main]`; `CommandContext` struct (defined in `cli/src/commands/context.rs:10`) carries `log`, `paths`, `args`, `http`; `context!()` macro (`main.rs:55`) constructs the context and installs a global logger; `start_code()` (`main.rs:177`) spawns the VS Code desktop binary via `std::process::Command`.
- **Control flow:** Raw OS args are collected at line 28 → `try_parse_legacy` attempts legacy arg mapping → falls back to `clap` parse via `args::AnyCli::Integrated` or `args::AnyCli::Standalone` → the `match parsed` block at line 63 dispatches on subcommand enum variants to one of: `update::update`, `tunnels::serve`, `serve_web::serve_web`, `agent_host::agent_host`, `version::switch_to`/`version::show`, `tunnels::command_shell`, `agent_ps::agent_ps`, `agent_stop::agent_stop`, `agent_kill::agent_kill`, `agent_logs::agent_logs`, or `start_code()`. All commands return `Result<i32, AnyError>`; errors call `print_and_exit` at line 169.
- **Data flow:** `LauncherPaths::migrate(core.global_options.cli_data_dir.clone())` at line 40 resolves the data directory early. The `reqwest::Client` is built once at line 45 with the product user agent string. The resolved `LauncherPaths` and `reqwest::Client` are threaded through all commands via `CommandContext`.
- **Dependencies:** `cli` library crate (all command modules); `reqwest`; `clap`.

---

#### `cli/src/lib.rs`

- **Role:** The public module surface of the `cli` crate. Controls what is re-exported.
- **Key symbols:** `pub mod auth` (line 9); `pub mod commands` (line 13); `pub mod tunnels` (line 18); `pub mod state` (line 17); `mod rpc` (line 26, private); `mod async_pipe` (line 22, private); `mod msgpack_rpc` (line 25, private).
- **Control flow:** Not applicable.
- **Data flow:** Not applicable.
- **Dependencies:** All modules listed above.

---

#### `cli/src/rpc.rs`

- **Role:** A transport-agnostic, serializer-pluggable RPC framework. Callers register sync, async, or duplex-stream methods; the dispatcher deserializes incoming bytes and routes them to the correct handler, returning serialized responses via an mpsc channel.
- **Key symbols:**
  - `Serialization` trait (`rpc.rs:42`) — `serialize(&self, value: impl Serialize) -> Vec<u8>` and `deserialize<P: DeserializeOwned>(&self, b: &[u8]) -> Result<P, AnyError>`.
  - `RpcBuilder<S>` (`rpc.rs:49`) — builder pattern; holds a `HashMap<&'static str, Method>` of registered methods and an `Arc<Mutex<HashMap<u32, DispatchMethod>>>` for pending outbound calls.
  - `Method` enum (`rpc.rs:34`) — `Sync(SyncMethod)`, `Async(AsyncMethod)`, `Duplex(Duplex)`.
  - `RpcMethodBuilder<S, C>` (`rpc.rs:86`) — wraps builder with a typed context `Arc<C>`.
  - `register_sync` (`rpc.rs:101`) — closure is boxed as a `SyncMethod`; deserializes `RequestParams<P>`, calls callback, serializes `SuccessResponse` or `ErrorResponse`.
  - `register_async` (`rpc.rs:148`) — same but callback returns a `Future`; wrapped in `BoxFuture`.
  - `register_duplex` (`rpc.rs:203`) — allocates N `tokio::io::duplex(8192)` pairs (one per stream), returns client ends via `StreamDto`, passes server ends to callback.
  - `RpcDispatcher<S, C>` (`rpc.rs:388`) — clone-able; `dispatch(&self, body: &[u8]) -> MaybeSync` at line 409.
  - `MaybeSync` enum (`rpc.rs:708`) — `Sync(Option<Vec<u8>>)`, `Future(BoxFuture<...>)`, `Stream((Option<StreamDto>, BoxFuture<...>))`.
  - `RpcCaller<S>` (`rpc.rs:310`) — `notify<M, A>` (fire-and-forget, line 331), `call<M, A, R>` (request-reply via `oneshot::channel`, line 342).
  - `Streams` (`rpc.rs:538`) — manages in-flight duplex streams by id; `write_loop` (`rpc.rs:596`) drains a per-stream write queue from a spawned task.
  - Built-in methods `METHOD_STREAM_DATA` / `METHOD_STREAM_ENDED` / `METHOD_STREAMS_STARTED` (`rpc.rs:633–635`) are registered automatically in `RpcMethodBuilder::build` at line 275.
  - `MESSAGE_ID_COUNTER: AtomicU32` (`rpc.rs:397`), incremented via `next_message_id()` with `SeqCst` ordering.
- **Control flow:** `dispatch` deserializes to `PartialIncoming` (only `id`, `method`, `error` fields) at line 410 → if `method` is present, looks up in `methods` HashMap and returns the matching `MaybeSync` variant → if no method but there is an `error`, pops the `calls` map and calls the dispatch closure with `Outcome::Error` → otherwise pops with `Outcome::Success`. Callers are responsible for awaiting futures returned by `dispatch`.
- **Data flow:** Incoming bytes → `Serialization::deserialize::<PartialIncoming>` → method lookup → full `RequestParams<P>` deserialized inside the method closure → callback produces `R` → `SuccessResponse { id, result }` serialized back to `Vec<u8>`.
- **Dependencies:** `tokio::io::DuplexStream`, `tokio::sync::{mpsc, oneshot}`, `serde`, `futures::future::BoxFuture`.

---

#### `cli/src/async_pipe.rs`

- **Role:** Cross-platform IPC abstraction that presents a uniform `AsyncPipe` read/write type regardless of OS. On Unix it is a `tokio::net::UnixStream`; on Windows it is an enum wrapping `NamedPipeClient` or `NamedPipeServer`.
- **Key symbols:**
  - `AsyncPipe` type alias (Unix, `async_pipe.rs:18`) = `tokio::net::UnixStream`; (Windows, `async_pipe.rs:53`) = enum `{ PipeClient(NamedPipeClient), PipeServer(NamedPipeServer) }`.
  - `AsyncPipeWriteHalf` / `AsyncPipeReadHalf` type aliases (lines 19–20 Unix, 116–117 Windows).
  - `get_socket_rw_stream(path: &Path) -> Result<AsyncPipe, CodeError>` — Unix connects via `tokio::net::UnixStream::connect` (line 22); Windows loops on `ClientOptions::new().open(path)` with 100 ms retry on `ERROR_PIPE_BUSY` (lines 119–131).
  - `listen_socket_rw_stream(path: &Path) -> Result<AsyncPipeListener, CodeError>` — Unix binds `tokio::net::UnixListener` (line 28); Windows creates `ServerOptions::new().first_pipe_instance(true).create(path)` (line 164).
  - `AsyncPipeListener::accept(&mut self)` — Unix returns next `UnixStream` (line 37); Windows calls `server.connect().await`, immediately creates a replacement `NamedPipeServer` to avoid a gap (line 155), and returns the old server as `AsyncPipe::PipeServer`.
  - `socket_stream_split(pipe: AsyncPipe) -> (AsyncPipeReadHalf, AsyncPipeWriteHalf)` — Unix uses `pipe.into_split()` (line 42); Windows uses `tokio::io::split(pipe)` (line 173).
  - `get_socket_name() -> PathBuf` (`async_pipe.rs:180`) — Unix: `$TMPDIR/{APPLICATION_NAME}-{uuid}`; Windows: `\\.\pipe\{APPLICATION_NAME}-{uuid}`.
  - `AsyncRWAccepter` trait (`async_pipe.rs:195`) — `accept_rw() -> Pin<Box<dyn Future<...>>>` implemented for both `AsyncPipeListener` and `tokio::net::TcpListener` (lines 201, 216).
- **Control flow:** Named pipe Windows accept loop is designed so that a fresh server is constructed before the old connection is dispatched (`std::mem::replace` at line 160), preventing a window where the server is unreachable.
- **Data flow:** Socket path → platform-specific open/bind → `AsyncPipe` → split into read/write halves → consumed by RPC read loop or write loop.
- **Dependencies:** `tokio::net::{UnixStream, UnixListener}` (Unix); `tokio::net::windows::named_pipe::{ClientOptions, ServerOptions}` (Windows); `pin-project` crate for the Windows `AsyncPipe` enum's `AsyncRead`/`AsyncWrite` dispatch.

---

#### `cli/src/state.rs`

- **Role:** Manages the CLI's on-disk data directory layout (`LauncherPaths`) and a generic JSON-persisted state container (`PersistedState<T>`).
- **Key symbols:**
  - `LauncherPaths` struct (`state.rs:26`) — fields: `server_cache: DownloadCache`, `cli_cache: DownloadCache`, `root: PathBuf`.
  - `LauncherPaths::migrate(root: Option<String>)` (`state.rs:138`) — migrates `~/.vscode-cli` to `~/{DEFAULT_DATA_PARENT_DIR}/cli` via `std::fs::rename`.
  - `LauncherPaths::new_without_replacements(root: PathBuf)` (`state.rs:189`) — initializes caches at `root/servers` and `root/cli`.
  - Methods returning paths: `tunnel_lockfile()` (line 207), `forwarding_lockfile()` (line 214), `service_log_file()` (line 222), `agent_host_lockfile()` (line 241), `web_server_storage()` (line 249).
  - `PersistedStateContainer<T>` (`state.rs:32`) — private; holds `path: PathBuf`, `state: Option<T>` (in-memory cache), `mode: u32` (Unix permissions).
  - `PersistedStateContainer::load_or_get` (`state.rs:46`) — reads JSON from disk with `serde_json::from_str::<T>`, falls back to `T::default()` on any error.
  - `PersistedStateContainer::save` (`state.rs:61`) — serializes with `serde_json::to_string`, writes via `OpenOptions` with `truncate(true)`; on non-Windows sets the file mode with `OpenOptionsExt::mode`.
  - `PersistedState<T>` (`state.rs:89`) — public wrapper; `Arc<Mutex<PersistedStateContainer<T>>>` so it is `Clone + Send + Sync`.
  - `PersistedState::update<R>(mutator: impl FnOnce(&mut T) -> R)` (`state.rs:128`) — load-mutate-save under a single lock acquisition.
- **Control flow:** All access to `PersistedStateContainer` is serialized through `Mutex::lock().unwrap()`. The in-memory `state` field is populated lazily on first `load_or_get` call.
- **Data flow:** `T: Clone + Serialize + DeserializeOwned + Default` ↔ JSON text on disk at `path`.
- **Dependencies:** `serde_json`; `dirs` crate for `home_dir()`; `DownloadCache` (`cli/src/download_cache.rs`).

---

#### `cli/src/tunnels/code_server.rs`

- **Role:** Manages the lifecycle of the VS Code server process — downloading/installing, spawning, health-checking, and representing the running server as typed handles.
- **Key symbols:**
  - `CodeServerArgs` (`code_server.rs:48`) — `#[derive(Clone, Debug, Default)]`; collects all `--socket-path`, `--port`, `--connection-token`, `--install-extension`, etc.; `command_arguments(&self) -> Vec<String>` at line 92 serializes them back to CLI flags.
  - `ServerParamsRaw` (`code_server.rs:169`) / `ResolvedServerParams` (`code_server.rs:179`) — two-phase resolution: `resolve()` at line 194 fetches the latest commit ID via `UpdateService` if `commit_id` is `None`.
  - `SocketCodeServer` (`code_server.rs:243`) — commit id, socket `PathBuf`, `Arc<CodeServerOrigin>`.
  - `PortCodeServer` (`code_server.rs:250`) — commit id, port `u16`, `Arc<CodeServerOrigin>`.
  - `CodeServerOrigin` enum (`code_server.rs:263`) — `New(Box<Child>)` (tokio process) or `Existing(u32)` (PID); `wait_for_exit()` at line 271 either awaits `child.wait()` or polls `process_exists(pid)` in a 30-second interval loop; `kill()` at line 285 calls `child.kill()` or `kill_tree(pid)`.
  - `ServerBuilder<'a>` (`code_server.rs:328`) — `get_running()` at line 355 reads PID from `server_paths.pidfile` then scans the log file for listening address via `parse_socket_from` / `parse_port_from`; `setup()` at line 412 creates a temp dir, streams the download archive, unzips, runs `--version` to verify integrity; `listen_on_default_socket()` at line 546 calls `get_socket_name()` then `_listen_on_socket()`.
  - `_listen_on_socket()` (`code_server.rs:555`) — removes any stale socket file, runs `get_base_command()` with `--start-server --enable-remote-auto-shutdown --socket-path=...`, calls `spawn_server_process()`, then calls `monitor_server::<SocketMatcher, PathBuf>` with a 30-second `timeout`.
  - `spawn_server_process()` (`code_server.rs:591`) — on Windows sets `CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP | CREATE_BREAKAWAY_FROM_JOB` creation flags (lines 602–611); writes the child PID to `server_paths.pidfile` at line 619.
  - `monitor_server<M, R>` (`code_server.rs:645`) — spawns a task that selects over `stderr_reader.next_line()` and `stdout_reader.next_line()` in a loop; on each line calls `M::match_line(&l)` and sends result on a `oneshot` channel; continues logging afterwards.
  - `ServerOutputMatcher<R>` trait (`code_server.rs:738`) — `match_line(line: &str) -> Option<R>`. Implementations: `SocketMatcher` (line 748, matches `"Extension host agent listening on (.+)"`), `PortMatcher` (line 756), `WebUiMatcher` (line 766), `NoOpMatcher` (line 777).
  - `LISTENING_PORT_RE` / `WEB_UI_RE` — `LazyLock<Regex>` at lines 42–45.
- **Control flow:** `serve` RPC handler calls `ServerBuilder::get_running()` → if none found, calls `setup()` → then `listen_on_default_socket()` → `monitor_server` task scans stdout/stderr → socket path arrives on `oneshot` → `SocketCodeServer` returned.
- **Data flow:** `ResolvedServerParams` → `ServerBuilder` → child `Command` + piped stdio → `BufReader` line scanning → matched address/port → typed handle (`SocketCodeServer` or `PortCodeServer`).
- **Dependencies:** `tokio::process::{Child, Command}`; `tokio::io::{AsyncBufReadExt, BufReader}`; `tokio::sync::oneshot`; `regex`; `UpdateService` (`cli/src/update_service.rs`); `DownloadCache` (`cli/src/download_cache.rs`); `async_pipe::get_socket_name`.

---

#### `cli/src/tunnels/control_server.rs`

- **Role:** The central RPC server for a tunnel connection. Registers ~20 RPC methods (fs ops, process spawn, VS Code server management, port forwarding, HTTP proxy, challenge-response auth), wires them to a MsgPack dispatcher, and multiplexes multiple WebSocket bridges through a single socket.
- **Key symbols:**
  - `HandlerContext` struct (`control_server.rs:74`) — carries `log`, `did_update: Arc<AtomicBool>`, `auth_state: Arc<Mutex<AuthState>>`, `socket_tx: mpsc::Sender<SocketSignal>`, `launcher_paths`, `code_server: CodeServerCell` (lazily started), `server_bridges: ServerMultiplexer`, `port_forwarding: Option<PortForwarding>`, `platform`, `http: Arc<FallbackSimpleHttp>`, `http_requests: HttpRequestsMap`.
  - `AuthState` enum (`control_server.rs:102`) — `WaitingForChallenge(Option<String>)`, `ChallengeIssued(String)`, `Authenticated`.
  - `serve()` function (`control_server.rs:176`) — top-level loop; calls `tunnel.add_port_direct(CONTROL_PORT)` and `tunnel.add_port_direct(AGENT_HOST_PORT)` at lines 184–185, spawns background update checker, selects over: shutdown signal, `ServerSignal::Respawn`, port-forwarding events, agent-host socket connections (served via hyper HTTP1), and control-port socket connections (served via `process_socket`).
  - `make_socket_rpc()` (`control_server.rs:364`) — constructs `RpcBuilder::new(MsgPackSerializer{})`, registers all methods, calls `.build(log)`. Methods registered: `ping`, `gethostname`, `sys_kill`, `fs_stat`, `fs_read` (duplex), `fs_write` (duplex), `fs_connect` (duplex), `net_connect` (duplex), `fs_rm`, `fs_mkdirp`, `fs_rename`, `fs_readdir`, `get_env`, `challenge_issue`, `challenge_verify`, `serve`, `update`, `servermsg`, `prune`, `callserverhttp`, `forward`, `unforward`, `acquire_cli`, `spawn` (3-stream duplex), `spawn_cli` (3-stream duplex), `httpheaders`, `httpbody`, `version`.
  - `ensure_auth()` (`control_server.rs:552`) — checks `AuthState::Authenticated` or returns `CodeError::ServerAuthRequired`.
  - `process_socket()` (`control_server.rs:561`) — selects in a loop over: `exit_barrier.wait()`, `http_rx.recv()` (delegated HTTP requests outbound to client), and `socket_rx.recv()` (messages to write to socket). Calls `handle_socket_read` in a spawned task.
  - `handle_socket_read()` (`control_server.rs:691`) — `MsgPackCodec::decode` frames from a `BufReader`; calls `rpc.dispatch_with_partial()` on each frame → `MaybeSync::Sync` responses sent directly; `MaybeSync::Future` and `MaybeSync::Stream` are spawned as tasks.
  - `handle_serve()` (`control_server.rs:768`) — acquires `c.code_server` mutex; if server not yet started, calls `ServerBuilder::get_running()` → `setup()` → `listen_on_default_socket()`; then calls `attach_server_bridge()` at line 852.
  - `attach_server_bridge()` (`control_server.rs:864`) — constructs `ServerMessageSink` (plain or compressed) and `ClientMessageDecoder`, creates a `ServerBridge` to the socket, registers it with the `ServerMultiplexer`.
  - `handle_spawn()` (`control_server.rs:1263`) — constructs `tokio::process::Command`, spawns with piped stdin/stdout/stderr, copies bidirectionally between the three `DuplexStream` streams and the child's stdio.
  - `ServerOutputSink` (`control_server.rs:748`) — `impl log::LogSink`; sends log lines as `ToClientRequest::serverlog` on the socket channel.
- **Control flow:** `serve()` select loop → incoming tunnel socket → `tokio::spawn(process_socket)` → `handle_socket_read` task decodes MsgPack frames → `rpc.dispatch_with_partial()` → method handlers execute synchronously or in spawned futures → `SocketSignal::Send(bytes)` queued to write loop.
- **Data flow:** Raw bytes from tunnel socket → `MsgPackCodec` framing → `PartialIncoming` dispatch → typed handler deserializing `RequestParams<P>` → handler result serialized to MsgPack → `SocketSignal::Send` → `writehalf.write_all`.
- **Dependencies:** `rpc::{RpcBuilder, RpcDispatcher, RpcCaller, MaybeSync}`; `msgpack_rpc::{MsgPackSerializer, MsgPackCodec}`; `async_pipe::get_socket_rw_stream`; `tunnels::code_server::ServerBuilder`; `tunnels::server_multiplexer::ServerMultiplexer`; `tunnels::server_bridge::ServerBridge`; `tunnels::port_forwarder::PortForwarding`; `hyper`; `tokio::process`.

---

#### `cli/src/commands/agent_host.rs`

- **Role:** Starts a local HTTP/WebSocket proxy that front-ends the VS Code agent host. Downloads the latest server version eagerly, starts a background update loop, binds a TCP listener (and optionally a dev tunnel), and dispatches each connection to hyper.
- **Key symbols:**
  - `AgentHostLockData` (`agent_host.rs:41`) — serialized JSON lockfile: `address` (ws:// URL), `pid`, optional `connection_token`, optional `tunnel_name`.
  - `agent_host()` (`agent_host.rs:59`) — main async function; reads or mints a connection token into `ctx.paths.root()/agent-host-token` (line 71); creates `AgentHostManager`; calls `manager.get_latest_release()` and `manager.ensure_downloaded()` eagerly; spawns `manager.run_update_loop()` in background (line 109); binds `TcpListener` at line 123; optionally creates a dev tunnel and registers `tunnel.add_port_direct(AGENT_HOST_PORT)` at line 157.
  - `handle_request_with_auth()` (`agent_host.rs:262`) — parses `?tkn=` query param from URI; returns `403 Forbidden` if absent or wrong.
  - Accept loop (`agent_host.rs:209`) — `tokio::select!` over `shutdown.wait()` (Ctrl-C) and `listener.accept()`; each accepted stream spawns a hyper `ServerBuilder::new(TokioExecutor::new()).serve_connection_with_upgrades` task.
  - `mint_connection_token()` (`agent_host.rs:283`) — opens file with `0o600` perms, reads existing token if no preference, or writes new UUID.
  - `write_agent_host_lockfile()` (`agent_host.rs:310`) — writes `AgentHostLockData` as JSON to path with `0o600` perms.
- **Control flow:** Start → resolve + download server → bind TCP → write lockfile → select loop → on connection spawn hyper handler → on Ctrl-C signal: kill server, close tunnel, remove lockfile, return `Ok(0)`.
- **Data flow:** `AgentHostManager` encapsulates server version resolution and lifecycle; incoming HTTP/WS connections forwarded to the server's Unix socket via `handle_request` from `tunnels::agent_host`.
- **Dependencies:** `hyper`/`hyper-util`; `tunnels::agent_host::{handle_request, AgentHostManager, AgentHostConfig}`; `tunnels::dev_tunnels::DevTunnels`; `auth::Auth`; `util::prereqs::PreReqChecker`.

---

#### `cli/src/commands/serve_web.rs`

- **Role:** Implements a "server of servers" HTTP proxy. Starts VS Code server processes on demand, routes requests to the version-matched server via a Unix socket, and handles WebSocket upgrades with bidirectional byte splicing. Implements a shared secret-key handshake for workbench security.
- **Key symbols:**
  - `serve_web()` (`serve_web.rs:75`) — verifies legal consent, mints connection token, creates `Arc<ConnectionManager>`, starts background update checker (line 98 or 103), calls `get_server_key_half`, then enters select loop over socket/TCP accept.
  - `ConnectionManager` struct — manages connection lifecycle, caching of server releases, and idle-timeout logic; not shown in excerpts but referenced at lines 95, 98, 228.
  - `handle()` (`serve_web.rs:195`) — routes `SECRET_KEY_MINT_PATH` to `handle_secret_mint()`, everything else to `handle_proxied()`; then calls `append_secret_headers()`.
  - `handle_proxied()` (`serve_web.rs:215`) — extracts version from URL path via `get_release_from_path()` or falls back to `cm.get_release_from_cache()`; calls `cm.get_connection(release)` to get an `AsyncPipe`; if `Upgrade` header present calls `forward_ws_req_to_server`, else `forward_http_req_to_server`.
  - `forward_http_req_to_server()` (`serve_web.rs:312`) — does HTTP/1 handshake with `hyper::client::conn::http1::handshake(TokioIo::new(rw))`, spawns the connection future, sends the request, returns the response.
  - `forward_ws_req_to_server()` (`serve_web.rs:339`) — same handshake, sends request, waits for `101 Switching Protocols`, then joins `hyper::upgrade::on(&mut req)` and `hyper::upgrade::on(&mut res)`, copies bidirectionally with `tokio::io::copy_bidirectional`.
  - `get_release_from_path()` (`serve_web.rs:281`) — parses `/<quality>-<commit>/rest` URL prefix, validates commit hash length with `is_commit_hash(commit)`.
  - `handle_secret_mint()` (`serve_web.rs:241`) — SHA-256 hashes server key half + client key half, returns first 32 bytes.
  - Constants: `SERVER_IDLE_TIMEOUT_SECS = 3600` (line 54), `SERVER_ACTIVE_TIMEOUT_SECS = 3600 * 24 * 30 * 12` (line 57), `RELEASE_CHECK_INTERVAL = 3600` (line 59), `SECRET_KEY_BYTES = 32` (line 62).
- **Control flow:** `serve_web()` → bind socket or TCP → select loop → accepted connection → spawn hyper `http1::Builder::serve_connection(...).with_upgrades()` task → `handle()` → route by path → proxy to server socket.
- **Data flow:** HTTP/WS request → path-based version extraction → `AsyncPipe` connection to server Unix socket → hyper client handshake → response piped back; secret key is SHA-256(server_half ++ client_half) derived per-request and set as cookies.
- **Dependencies:** `hyper`/`hyper-util`/`http-body-util`; `async_pipe::{get_socket_name, get_socket_rw_stream, listen_socket_rw_stream, AsyncPipe}`; `sha2`; `state::{LauncherPaths, PersistedState}`; `update_service::{Platform, Release, UpdateService}`; `tunnels::legal`.

---

#### `cli/src/auth.rs` (partial read, lines 1–599)

- **Role:** OAuth device-code flow for Microsoft and GitHub accounts, backed by OS keyring or file storage. Provides `Auth::get_credential()`, `Auth::login()`, `Auth::login_with_scopes()`.
- **Key symbols:**
  - `AuthProvider` enum (`auth.rs:51`) — `Microsoft` / `Github`; methods `client_id()`, `code_uri()`, `grant_uri()`, `get_default_scopes()` at lines 66–98.
  - `StoredCredential` (`auth.rs:101`) — serialized with short keys `p`/`a`/`r`/`e`; `expires_at: Option<jiff::Timestamp>`.
  - `StorageImplementation` trait (`auth.rs:195`) — `read`, `store`, `clear`.
  - `KeyringStorage` (`auth.rs:298`) — chunked into `KEYCHAIN_ENTRY_LIMIT`-byte keyring entries (1024 B on Windows, 128 KB elsewhere) with `CONTINUE_MARKER = "<MORE>"` suffix to chain chunks (line 232–234).
  - `ThreadKeyringStorage` (`auth.rs:237`, Linux only) — wraps `KeyringStorage` operations in a dedicated thread with 5-second timeout to handle indefinitely-blocking keyring calls.
  - `FileStorage` (`auth.rs:386`) — wraps `PersistedState<Option<String>>`; tokens are `seal()`-ed (encrypted then base64) before writing.
  - `Auth::with_storage<T, F>()` (`auth.rs:439`) — lazy-initializes `StorageWithLastRead`; prefers keyring unless `VSCODE_CLI_USE_FILE_KEYCHAIN` env var is set or file already exists.
  - `Auth::get_credential()` (`auth.rs:570`) — checks existing credential → calls `maybe_refresh_token()` → if expired, re-runs device code flow.
- **Control flow:** `get_credential()` → read from storage → if expired/missing → `do_device_code_flow_with_provider()` → POST to `code_uri` → poll `grant_uri` until token arrived → store via `store_credentials`.
- **Data flow:** OAuth JSON responses deserialized to `AuthenticationResponse` → converted to `StoredCredential` → `seal()` encrypts with AES or base64 → stored in keyring chunks or file.
- **Dependencies:** `keyring` crate; `reqwest`; `jiff` for timestamps; `tunnels::management::{Authorization, AuthorizationProvider}`; `state::PersistedState`.

---

### Cross-Cutting Synthesis

The VS Code CLI Rust codebase implements a complete IDE remote-access infrastructure built on four interlocking layers. The generic RPC framework in `rpc.rs` is transport-agnostic — it receives raw bytes, dispatches to registered method closures, and returns serialized bytes — requiring only a `Serialization` impl to switch between JSON (`json_rpc.rs`) and MsgPack (`msgpack_rpc.rs`). The cross-platform IPC layer in `async_pipe.rs` normalizes Unix domain sockets and Windows named pipes behind a single `AsyncPipe` type, allowing all higher-level code to be OS-independent. `control_server.rs` wires these two together into a full multiplexing RPC server that handles ~20 method types — spanning filesystem ops, process spawn with three bidirectional streams (stdin/stdout/stderr), port forwarding, and VS Code server lifecycle management — all under a `tokio::select!` event loop per socket. `code_server.rs` manages the actual VS Code server subprocess: downloading, verifying, spawning with `tokio::process::Command`, and detecting the listening address by scanning stdout/stderr lines against `LazyLock<Regex>`. State is persisted to disk as JSON via `PersistedState<T>` (a mutex-guarded, lazily-loaded, atomically-written container), and authentication tokens are chunked across OS keyring entries or a mode-`0o600` file. There is zero Tauri dependency in the entire `cli/` partition; the code demonstrates production patterns directly portable to a Tauri backend: async tokio runtime, `Arc<Mutex<>>` shared state, trait-based serialization abstraction, and hyper HTTP/WS proxying.

---

### Out-of-Partition References

- `/home/norinlavaee/projects/vscode-atomic/cli/src/commands/context.rs` — defines `CommandContext { log, paths, args, http }` used as the root dependency carrier by all command handlers
- `/home/norinlavaee/projects/vscode-atomic/cli/src/msgpack_rpc.rs` — `MsgPackSerializer` impl of `rpc::Serialization` and `MsgPackCodec` (tokio-util `Decoder`) used by `control_server.rs`
- `/home/norinlavaee/projects/vscode-atomic/cli/src/json_rpc.rs` — JSON-based `Serialization` impl, used by lighter-weight command paths
- `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/agent_host.rs` — `AgentHostManager` and `handle_request` function that owns VS Code agent-host server lifecycle; imported by both `commands/agent_host.rs` and `tunnels/control_server.rs`
- `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/dev_tunnels.rs` — `DevTunnels`, `ActiveTunnel`, `add_port_direct` — the dev-tunnel relay integration used by `control_server::serve()` and `commands/agent_host.rs`
- `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/server_bridge.rs` — `ServerBridge`: connects the control-server RPC socket to the VS Code server Unix socket via `ServerMessageSink`/`ClientMessageDecoder`
- `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/server_multiplexer.rs` — `ServerMultiplexer`: manages multiple logical WebSocket bridge connections over a single physical socket
- `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/port_forwarder.rs` — `PortForwarding`, `PortForwardingProcessor`: handles `forward`/`unforward` RPC methods
- `/home/norinlavaee/projects/vscode-atomic/cli/src/update_service.rs` — `UpdateService`, `Platform`, `Release`, `TargetKind`: abstracts version resolution and archive download
- `/home/norinlavaee/projects/vscode-atomic/cli/src/download_cache.rs` — `DownloadCache`: LRU directory cache for downloaded server/CLI archives; referenced by `LauncherPaths`
- `/home/norinlavaee/projects/vscode-atomic/cli/src/util/command.rs` — `new_script_command`, `kill_tree`, `capture_command_and_check_status`: used by `code_server.rs` for subprocess management
- `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/protocol.rs` — all RPC message types (`ServeParams`, `ForwardParams`, `SpawnParams`, `HttpRequestParams`, etc.) consumed by `control_server.rs` handler functions
