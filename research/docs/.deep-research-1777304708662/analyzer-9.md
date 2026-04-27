### Files Analysed

1. `/Users/norinlavaee/vscode-atomic/cli/Cargo.toml`
2. `/Users/norinlavaee/vscode-atomic/cli/src/bin/code/main.rs`
3. `/Users/norinlavaee/vscode-atomic/cli/src/lib.rs`
4. `/Users/norinlavaee/vscode-atomic/cli/src/rpc.rs`
5. `/Users/norinlavaee/vscode-atomic/cli/src/json_rpc.rs`
6. `/Users/norinlavaee/vscode-atomic/cli/src/msgpack_rpc.rs`
7. `/Users/norinlavaee/vscode-atomic/cli/src/async_pipe.rs`
8. `/Users/norinlavaee/vscode-atomic/cli/src/state.rs`
9. `/Users/norinlavaee/vscode-atomic/cli/src/tunnels/control_server.rs` (lines 1–520)
10. `/Users/norinlavaee/vscode-atomic/cli/src/tunnels/protocol.rs`
11. `/Users/norinlavaee/vscode-atomic/cli/src/tunnels/code_server.rs` (lines 1–320)
12. `/Users/norinlavaee/vscode-atomic/cli/src/auth.rs` (lines 1–320)

---

### Per-File Notes (file.rs:line)

#### `Cargo.toml`

- **tokio 1.38 with `features = ["full"]`** (line 19): full async runtime including `io`, `net`, `sync`, `process`, `time`, `fs`.
- **Dual serialization**: `serde_json = "1.0.96"` (line 27) for JSON-RPC over line-delimited streams; `rmp-serde = "1.1.1"` (line 28) for msgpack-RPC over binary frames. Both share the same `serde` derive infrastructure.
- **`async-trait = "0.1.68"`** (line 45): used to box async methods in trait objects, a pattern Tauri's `commands` macro would replace.
- **`keyring = "2.0.3"`** (line 38): platform-native secret store (macOS Keychain, Windows Credential Manager, Linux `libsecret`/`keyutils`). Feature-gated per platform.
- **`tunnels`** (line 37): external `dev-tunnels` crate pinned to a specific git rev, providing the underlying relay transport; not replicated inside the repo.
- **`hyper = "0.14.26"`** (line 40): embedded HTTP/1 server for the agent-host port (`AGENT_HOST_PORT`).
- **`cfg-if = "1.0.0"`** (line 52) + `target.'cfg(windows)'` / `target.'cfg(target_os = "linux")'` sections (lines 63–71): compile-time platform gating without separate feature flags.
- **`pin-project = "1.1.0"`** (line 53): safe `Pin` projections on the Windows `AsyncPipe` enum; mirrors what Tauri's async bridge must also do.
- **`opentelemetry`** (line 32): distributed tracing hooked into the tunnel connection spans (`control_server.rs:316`).

---

#### `main.rs`

- **`#[tokio::main]`** entry point (line 25): spawns the full tokio multi-thread executor; the function signature returns `Result<(), std::convert::Infallible>` (line 26) so process exit is handled explicitly via `std::process::exit` (line 135).
- **`CommandContext`** struct (lines 43–51): aggregates `reqwest::Client`, `LauncherPaths`, `log::Logger`, and `args` — the dependency-injection carrier threaded through every sub-command. In a Tauri port this maps to Tauri's `State<T>` managed state.
- **Dual-CLI dispatch** (lines 28–36): `try_parse_legacy` wraps old Node-style args, then falls back to `clap`-derived `AnyCli::Integrated` / `AnyCli::Standalone`. The integrated variant is used when embedded inside Electron; the standalone variant is the pure Rust CLI binary.
- **`context!()` macro** (lines 54–60): creates `CommandContext` + installs a global logger; avoids repeating boilerplate across every `match` arm.
- **`start_code()`** (lines 164–191): the non-tunnel path. Resolves a VS Code desktop installation via `CodeVersionManager`, then calls `std::process::Command::new(&binary).args(args).status()`. This is the main bridge to the Electron app and is the integration seam that a Tauri port replaces entirely.

---

#### `lib.rs`

- Declares public modules `auth`, `commands`, `desktop`, `tunnels`, `state`, `util`, `log`, `constants`, `options` (lines 8–18).
- Private modules `async_pipe`, `json_rpc`, `msgpack_rpc`, `rpc`, `singleton` (lines 21–26) form the internal IPC substrate.
- The visibility split (public vs private) is intentional: the RPC/pipe layer is considered internal infrastructure, not an external API surface.

---

#### `rpc.rs`

This is the most architecturally significant file for a Tauri port.

- **`Serialization` trait** (lines 42–45): a two-method interface (`serialize`, `deserialize`) that decouples the transport-framing from the message encoding. Both `JsonRpcSerializer` and `MsgPackSerializer` implement it. A Tauri IPC serializer would implement the same trait.

- **Three method kinds** (lines 25–38):
  - `SyncMethod`: `Arc<dyn Fn(Option<u32>, &[u8]) -> Option<Vec<u8>>>` — returns immediately.
  - `AsyncMethod`: returns `BoxFuture<'static, Option<Vec<u8>>>` — returns a pinned, heap-allocated future. The `'static` bound is enforced because tokio `spawn` requires it.
  - `Duplex`: returns `(Option<StreamDto>, BoxFuture<…>)` — used when the call creates a bidirectional byte stream (e.g., `fs_read`, `fs_write`, `spawn`).

- **`RpcBuilder<S>` builder pattern** (lines 49–84): collects method registrations before building a `RpcDispatcher`. `get_caller()` (lines 67–73) returns an `RpcCaller` bound to an `mpsc::UnboundedSender<Vec<u8>>` so callers can be created before the dispatcher is wired to a transport.

- **`RpcMethodBuilder::register_sync/async/duplex`** (lines 101–272): each registration closure captures `Arc<S>` (serializer) and `Arc<C>` (context) by clone, making the closures `'static + Send + Sync`. Deserialisation happens inside the closure; errors are marshalled into `ErrorResponse` structs.

- **`RpcDispatcher::dispatch()`** (line 409): deserialises a `PartialIncoming` (just `id`, `method`, `error`) to route without fully deserialising the payload, then dispatches to the correct registered closure. Returns `MaybeSync` (line 708–712) so the caller decides whether to block the read loop or `tokio::spawn` the future.

- **`AtomicU32 MESSAGE_ID_COUNTER`** (line 397): lock-free monotonic ID generator used both here and in `control_server.rs:114` (a second static counter).

- **`Streams` internal type** (lines 538–585): manages `WriteHalf<DuplexStream>` keyed by `u32` stream ID. A queue-draining `write_loop` (lines 596–631) is spawned as a task per stream; the loop parks the `WriteHalf` back into the map when the queue is empty, avoiding a permanently alive background task.

- **`oneshot` channel for call results** (lines 342–381): `RpcCaller::call()` inserts a `Box<dyn FnOnce(Outcome)>` into the calls map, returns a `oneshot::Receiver`. When the response arrives in `dispatch_with_partial`, the closure is removed from the map and called with `Outcome::Success` or `Outcome::Error`.

---

#### `json_rpc.rs`

- **`JsonRpcSerializer`** (lines 22–37): `serialize` appends `\n` to the JSON bytes, making the framing newline-delimited.
- **`start_json_rpc()`** (lines 46–106): wraps the read side in `tokio::io::BufReader` and calls `read_line` in a `tokio::select!` loop alongside write_rx, msg_rx, and a `shutdown_rx` barrier. On `MaybeSync::Future` and `MaybeSync::Stream`, the future is detached with `tokio::spawn` so the read loop is never blocked.

---

#### `msgpack_rpc.rs`

- **`MsgPackSerializer`** (lines 25–35): calls `rmp_serde::to_vec_named` / `rmp_serde::from_slice`. No framing suffix; instead uses a length-prefix codec.
- **`MsgPackCodec<T>`** (lines 118–163): implements `tokio_util::codec::Decoder`. Reads via a `Cursor` on `BytesMut`; on `UnexpectedEof` reserves 1024 more bytes and returns `Ok(None)`. On success, calls `src.advance(len)` to consume exactly the bytes for one message. This is cancellation-safe because `read_buf` is the only async call in the loop.
- **`start_msgpack_rpc()`** (lines 46–110): unlike the JSON variant, it returns `(Option<X>, Read, Write)` — the original I/O halves are handed back so the caller can reuse them after shutdown (used by the control server when handing the pipe to a child VS Code server process).

---

#### `async_pipe.rs`

- **`cfg_if!` macro** (lines 17–177): the entire cross-platform abstraction for named pipes. On Unix (line 19–46): `AsyncPipe = tokio::net::UnixStream`; on Windows (lines 47–176): a `#[pin_project]` enum wrapping `NamedPipeClient | NamedPipeServer`, manually implementing `AsyncRead` and `AsyncWrite` by match-projecting to the inner pin.
- **Windows connection retry loop** (lines 122–131): loops on `ERROR_PIPE_BUSY` (OS error 231) with `tokio::time::sleep(100ms)` until the pipe is available.
- **`PollableAsyncListener`** (lines 188–228): wraps `AsyncPipeListener` in a `hyper::server::accept::Accept` impl using `tokio_util::sync::ReusableBoxFuture` to store the pending `accept()` future in a way that is polled by Hyper's connection loop.
- **`AsyncRWAccepter` trait** (lines 247–270): object-safe trait (`async_trait`) abstracting over both `AsyncPipeListener` and `tokio::net::TcpListener`, returning boxed `AsyncRead + AsyncWrite` pairs. This is the abstraction boundary used in `serve_web` and `serve_stream`.
- **`get_socket_name()`** (lines 231–239): `cfg_if!` for OS: on Unix `$TMPDIR/{APPLICATION_NAME}-{uuid4}`; on Windows `\\.\pipe\{APPLICATION_NAME}-{uuid4}`.

---

#### `state.rs`

- **`PersistedState<T>`** (lines 89–134): generic JSON-on-disk state container. The inner `PersistedStateContainer` holds an in-memory cache (`Option<T>`) and writes via `fs::OpenOptions` with `create(true).write(true).truncate(true)`. On non-Windows, `OpenOptionsExt::mode()` sets Unix file permissions (line 81).
- **`PersistedState::update()`** (lines 128–133): load → mutate via closure → save, all under a `Mutex` lock. The returned `R` is the closure's return value, enabling combined read-modify-write with a single lock acquisition.
- **`LauncherPaths`** (lines 26–29): holds `DownloadCache` for servers and CLI binaries plus a root `PathBuf`. Paths for the tunnel lockfile (`tunnel-{quality}.lock`), forwarding lockfile, and service log file are computed from root (lines 207–225).
- **`LauncherPaths::migrate()`** (lines 138–162): one-time migration from `~/.vscode-cli` to `~/{DEFAULT_DATA_PARENT_DIR}/cli` using `std::fs::rename`. The result feeds directly into `CommandContext.paths` in `main.rs:39`.

---

#### `tunnels/control_server.rs` (lines 1–520)

- **`HandlerContext`** struct (lines 77–102): the RPC context (`C`) passed to every registered handler. Contains `Arc<Mutex<Option<SocketCodeServer>>>` (the running server cell), `ServerMultiplexer` (multi-stream bridge to the VS Code server), `Arc<AtomicBool>` for update tracking, `Arc<std::sync::Mutex<AuthState>>` for per-connection authentication state, and `mpsc::Sender<SocketSignal>` for signalling back to the socket accept loop.
- **`AuthState` enum** (lines 105–112): three states: `WaitingForChallenge(Option<String>)`, `ChallengeIssued(String)`, `Authenticated`. Drives a challenge–response handshake before any protected RPC method is served.
- **`make_socket_rpc()`** (lines 383–393): constructs `RpcBuilder::new(MsgPackSerializer {}).methods(HandlerContext {...})` and then calls `register_sync/register_async/register_duplex` for every protocol method. Each handler closure calls `ensure_auth(&c.auth_state)?` first before doing work.
- **`serve()` main loop** (lines 179–344): a `tokio::select!` over shutdown barrier, `ServerSignal` channel, port-forwarding events, agent-host connections (served via `hyper::server::conn::Http`), and control-port incoming connections. Each control-port connection is detached with `tokio::spawn`.
- **OpenTelemetry span** (lines 316–340): each spawned connection task starts a span `"server.socket"` of kind `SpanKind::Consumer` and records `tx`, `rx`, `duration_ms` attributes.

---

#### `tunnels/protocol.rs`

- **`ClientRequestMethod<'a>` tagged enum** (lines 14–23): uses `#[serde(tag = "method", content = "params", rename_all = "camelCase")]` — the enum variant name becomes the `"method"` field value and `"params"` carries the content. Zero-copy borrowing via lifetime `'a` on `RefServerMessageParams<'a>` (line 101) and `ServerLog<'a>` (line 120).
- **`serde_bytes` annotation** (lines 29, 92, 103): used on all raw byte payloads (`segment`, `body`) so msgpack serialises them as binary objects rather than arrays of integers.
- **`ToClientRequest<'a>`** (lines 113–118): wraps `ClientRequestMethod` with a `#[serde(flatten)]` field, producing the envelope `{ id, method, params }` without an intermediate nesting level.
- **`FsFileKind` enum with `From<std::fs::FileType>`** (lines 163–185): the serde rename (`"dir"`, `"file"`, `"link"`) maps to the string values expected by the TypeScript client without any manual mapping code.

---

#### `tunnels/code_server.rs` (lines 1–320)

- **`CodeServerArgs`** (lines 50–78): a plain `Clone + Default` struct mapping 1:1 to VS Code Server CLI flags. `command_arguments()` (lines 94–165) serialises the struct back into `Vec<String>` for `tokio::process::Command`.
- **`ServerParamsRaw::resolve()`** (lines 196–231): async, calls `UpdateService::get_latest_commit()` if no commit ID is provided. Returns `ResolvedServerParams` containing a `Release` struct.
- **`CodeServerOrigin` enum** (lines 265–270): distinguishes between a newly spawned `tokio::process::Child` and an already-running process tracked by PID. `wait_for_exit()` (lines 273–285) polls either `child.wait()` or an interval loop checking `process_exists(pid)`.
- **`lazy_static!` regexes** (lines 43–47): compiled once at first use; used in the server-startup log-parsing loop to detect the `"Extension host agent listening on …"` line and the Web UI URL.

---

#### `auth.rs`

- **`AuthProvider` enum** (lines 53–57): `#[derive(clap::ValueEnum, Serialize, Deserialize)]` — dual role as a CLI argument and a serialised credential field. Carries `client_id`, `code_uri`, `grant_uri`, `get_default_scopes()` associated to each variant.
- **`StoredCredential`** (lines 104–114): serde rename-all `p/a/r/e` minimises keyring payload size. `expires_at: Option<DateTime<Utc>>` from `chrono`.
- **`StorageImplementation` trait** (lines 188–192): sync, object-safe. Implemented by `KeyringStorage` (OS keyring), `FileStorage` (fallback JSON file), and `ThreadKeyringStorage` (lines 229–285) which runs keyring calls on a dedicated thread with a 5-second timeout to avoid blocking the tokio executor (pattern: `std::sync::mpsc::channel` + `thread::spawn` + `thread::sleep` timer thread).
- **`seal/unseal` functions** (lines 195–218): encrypt credentials using a platform-specific function before storing in keyring. On deserialization, falls back to plain `serde_json::from_str` for back-compat with old unencrypted entries.
- **`KeyringStorage` multi-chunk reads** (lines 307–331): loops over sequentially-named keyring entries (prefixed `CONTINUE_MARKER = "<MORE>"`) to reassemble values larger than `KEYCHAIN_ENTRY_LIMIT` (1024 bytes on Windows, 128 KB elsewhere).

---

### Cross-Cutting Synthesis (≤200 words)

The existing Rust CLI provides substantial, production-validated infrastructure that directly maps to Tauri port requirements.

The **`rpc.rs` generic `RpcBuilder<S>`** (lines 49–301) is already transport-agnostic: swapping the serializer to Tauri's JSON IPC format or to a `postMessage` bridge requires only a new `Serialization` impl. The three-tier method system (`Sync/Async/Duplex`) cleanly covers Tauri command patterns, event streams, and binary data channels.

The **`async_pipe.rs` `cfg_if!` cross-platform abstraction** (lines 17–177) — UnixStream on macOS/Linux, NamedPipe on Windows — is exactly the kind of OS-seam management Tauri relies on internally, and its `AsyncRWAccepter` trait (line 247) can be reused directly.

The **`PersistedState<T>`** pattern (state.rs:89) already implements the thread-safe, JSON-on-disk store that Tauri's `tauri-plugin-store` replicates.

**Gaps**: The code still spawns a separate Node.js/Electron process (`main.rs:184`) as the UI layer — the entire `desktop` module and `start_code()` path would be replaced by Tauri's `WebviewWindow`. The `tunnels` external crate (Cargo.toml:37) is pinned to a private Microsoft git rev with no published crate, making it an integration risk. The `hyper 0.14` server (agent host) predates `hyper 1.x` and Tauri's axum-based patterns, requiring an upgrade path.

---

### Out-of-Partition References

- **`cli/src/tunnels/socket_signal.rs`** — `SocketSignal`, `ServerMessageSink`, `ClientMessageDecoder` types imported by `control_server.rs:70`; defines how messages are routed between the tunnel socket and the VS Code server process.
- **`cli/src/tunnels/server_multiplexer.rs`** — `ServerMultiplexer` (used at `control_server.rs:75`); manages concurrent "websocket" connections to a single code server process.
- **`cli/src/tunnels/dev_tunnels.rs`** — `ActiveTunnel` type returned by `serve()` at `control_server.rs:149`; wraps the external `tunnels` crate relay management.
- **`cli/src/util/sync.rs`** — `Barrier<S>`, `new_barrier()`, `Receivable` trait used throughout `json_rpc.rs:17`, `msgpack_rpc.rs:19`, `control_server.rs:28`; the custom shutdown-signal primitive.
- **`cli/src/desktop.rs`** — `CodeVersionManager`, `RequestedVersion`, `prompt_to_install` called in `main.rs:170–180`; the module managing local VS Code desktop installation discovery — the primary Electron integration seam a Tauri port eliminates.
