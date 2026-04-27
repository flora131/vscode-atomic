# Partition 9 of 79 — Findings

## Scope
`cli/` (70 files, 18,723 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Rust CLI Codebase Structure - Partition 9

## Overview
The VS Code CLI (`cli/` directory) is a mature Rust implementation of the launcher and tunnel infrastructure. It demonstrates comprehensive patterns for async I/O, cross-platform support, serialization, and RPC communication that would be directly applicable to a Tauri-based IDE redesign.

**Scope:** 68 Rust files, 18,723 LOC across 4 major subsystems

---

## Implementation

### Core Architecture

**Entry Points & Binary:**
- `cli/src/bin/code/main.rs` — Single entry point with `#[tokio::main]` attribute; handles argument parsing (legacy + clap), command routing, and context creation
- `cli/src/lib.rs` — Library root exposing 8 public modules (auth, commands, constants, log, options, state, tunnels, update_service, util, desktop)

**Main Modules:**
- `cli/src/commands/` (8 files) — Command handlers: tunnels, serve_web, agent_host, update, version; uses clap for argument parsing
  - `commands/args.rs` — CLI argument definitions using clap derive macros (IntegratedCli, StandaloneCli, CliCore structs)
  - `commands/context.rs` — CommandContext struct bundling HTTP client, paths, logging, args
  - `commands/serve_web.rs` — Web server command with tokio::spawn patterns (11 async functions, 5 tokio::spawn calls)
  - `commands/tunnels.rs` — Tunnel management with control server integration
  - `commands/agent_host.rs` — Agent host implementation with RPC

### Serialization & RPC (5 files)
- `cli/src/rpc.rs` — Generic RPC framework with pluggable Serialization trait; supports sync/async/duplex methods
  - RpcBuilder<S>, RpcCaller<S>, RpcDispatcher<S, C> for flexible method registration
  - Handles concurrent method calls with Arc<Mutex<HashMap<u32, DispatchMethod>>>
  - 6 async functions, tokio::spawn for write_loop management
- `cli/src/json_rpc.rs` — JSON serialization wrapper; uses serde_json, adds newline framing
- `cli/src/msgpack_rpc.rs` — MessagePack serialization using rmp-serde; binary framing with length prefix
- `cli/src/msgpack_rpc.rs` — Supports streaming over AsyncRead/AsyncWrite
- `cli/src/async_pipe.rs` — Duplex channel-based piping; 10 async functions, 3 tokio::spawn calls

### Tunneling Infrastructure (25 files)
- `cli/src/tunnels.rs` — Module root re-exporting public interfaces (serve, serve_stream, SleepInhibitor, ServiceManager)
- `cli/src/tunnels/control_server.rs` — Main tunnel control protocol (25 async functions, 8 tokio::spawn calls); handles HTTP forwarding, port management, process spawning
  - Implements command handlers: forward, unforward, serve, sys_kill, fs operations, net_connect
  - Uses hyper for HTTP tunneling; integrates with dev-tunnels crate
- `cli/src/tunnels/agent_host.rs` — Agent host for code server connections (11 async functions, 3 tokio::spawn)
- `cli/src/tunnels/code_server.rs` — Code Server integration (16 async functions); manages spawning, streaming stdio
- `cli/src/tunnels/dev_tunnels.rs` — Microsoft dev-tunnels integration (27 async functions, 1 tokio::spawn); authentication, tunnel creation, management
- `cli/src/tunnels/protocol.rs` — Protocol definitions; 47 serde-derived structs/enums covering:
  - ClientRequestMethod, ForwardParams, ServeParams, SpawnParams, PortPrivacy, PortProtocol
  - Status enums, LogMessage, ServerMessage types for bidirectional communication
- `cli/src/tunnels/server_bridge.rs` — Stream bridging for tunnel connections (3 async)
- `cli/src/tunnels/server_multiplexer.rs` — Connection multiplexing (2 async, 1 tokio::spawn)

**Platform-Specific Services:**
- `cli/src/tunnels/service.rs` — Abstract service manager trait
- `cli/src/tunnels/service_linux.rs` — systemd integration via zbus (7 async, platform-cfg)
- `cli/src/tunnels/service_macos.rs` — launchd integration (5 async, cfg)
- `cli/src/tunnels/service_windows.rs` — Windows Service integration (5 async, cfg)

**Signal Handling & Lifecycle:**
- `cli/src/tunnels/shutdown_signal.rs` — Cross-platform shutdown signaling (1 async, cfg-gated)
- `cli/src/tunnels/socket_signal.rs` — Unix socket based signaling (4 async functions)
- `cli/src/tunnels/singleton_server.rs` — Singleton lock via file/socket (2 async, 2 tokio::spawn)
- `cli/src/tunnels/singleton_client.rs` — Singleton client connection
- `cli/src/tunnels/nosleep.rs` — Sleep inhibitor abstraction
- `cli/src/tunnels/nosleep_linux.rs` — D-Bus sleep inhibition (cfg)
- `cli/src/tunnels/nosleep_macos.rs` — IOKit assertion-based sleep inhibition (cfg)
- `cli/src/tunnels/nosleep_windows.rs` — SetThreadExecutionState sleep inhibition (cfg)

**Other Tunneling:**
- `cli/src/tunnels/local_forwarding.rs` — Local port forwarding (4 async, 1 tokio::spawn)
- `cli/src/tunnels/port_forwarder.rs` — Port management and forwarding
- `cli/src/tunnels/challenge.rs` — Challenge/response authentication
- `cli/src/tunnels/legal.rs` — License/legal message display (1 serde struct)
- `cli/src/tunnels/paths.rs` — Tunnel state directory management (2 serde)
- `cli/src/tunnels/wsl_detect.rs` — WSL environment detection

### Authentication & State (3 files)
- `cli/src/auth.rs` — OAuth device flow for Microsoft/GitHub (14 async functions, 5 serde structs)
  - DeviceCodeResponse, AuthenticationResponse for OAuth parsing
  - AuthProvider enum (Microsoft/GitHub) with client_ids and token endpoints
  - Integration with tunnels crate's Authorization trait
- `cli/src/state.rs` — Persisted state management with serde_json
  - LauncherPaths for cache/server management
  - PersistedStateContainer<T> generic for any serde type
  - Platform-specific file permissions (Unix 0o600, Windows secure)
- `cli/src/download_cache.rs` — LRU cache for downloaded artifacts
  - Generic async create() with staging directory pattern
  - State tracked via PersistedState<Vec<String>>

### Utilities (14 files)
- `cli/src/util/http.rs` — HTTP client wrapper abstractions
  - SimpleResponse struct with streaming read (AsyncRead)
  - download_into_file() with progress callback
  - reqwest + hyper integration
- `cli/src/util/errors.rs` — Error types with thiserror
  - WrappedError for context; StatusError for HTTP responses
  - wrap() and wrapdbg() helpers
  - Custom AnyError type
- `cli/src/util/command.rs` — Process spawning utilities (5 async, 1 tokio::spawn)
- `cli/src/util/os.rs` — OS detection and platform checks
- `cli/src/util/machine.rs` — Machine/CPU info via sysinfo
- `cli/src/util/is_integrated.rs` — Integration detection
- `cli/src/util/sync.rs` — Synchronization utilities: Barrier, RwLockValueExt (10 async, 3 tokio::spawn)
- `cli/src/util/app_lock.rs` — File-based app locking
- `cli/src/util/file_lock.rs` — File lock abstractions (3 impl)
- `cli/src/util/input.rs` — Interactive input via dialoguer
- `cli/src/util/io.rs` — Async I/O helpers: copy_async_progress, ReadBuffer (4 async, 1 tokio::spawn)
- `cli/src/util/tar.rs` — Tar extraction
- `cli/src/util/zipper.rs` — ZIP extraction
- `cli/src/util/ring_buffer.rs` — Ring buffer data structure
- `cli/src/util/prereqs.rs` — Prerequisites checking (8 async)

### Configuration & Constants
- `cli/src/constants.rs` — Compile-time constants from environment
  - CONTROL_PORT, AGENT_HOST_PORT, PROTOCOL_VERSION (5)
  - Application naming via option_env!() from build.rs
  - Editor web URLs, update endpoints
- `cli/src/options.rs` — CLI quality enum (Stable, Insiders, Exploration)
- `cli/src/desktop.rs` — Desktop/Electron interop
  - `desktop/version_manager.rs` — Version detection for bundled Electron (6 async)
- `cli/src/log.rs` — Structured logging with opentelemetry, log crate
- `cli/src/self_update.rs` — Self-update logic (2 async)
- `cli/src/update_service.rs` — Update service abstraction (3 async)

### Private RPC & Communication
- `cli/src/singleton.rs` — Singleton pattern for avoiding multiple instances (8 async, 1 tokio::spawn)
- `cli/src/json_rpc.rs` — JSON-line RPC (async start function)
- `cli/src/msgpack_rpc.rs` — Binary msgpack RPC (async start function)

---

## Tests

**No dedicated test files found in scope.** Testing patterns are embedded in modules via:
- `#[test]` attributes found in 14 files (mostly config/utility modules)
- Inline test modules (uncommon in this codebase)

Appears to rely on integration tests at repository level rather than unit test organization.

---

## Types / Interfaces

### Serialized Protocol Messages (42+ structs/enums)
Located in `cli/src/tunnels/protocol.rs`:
- **Request/Response pairs**: ClientRequestMethod, HttpBodyParams, HttpRequestParams, ForwardParams, UnforwardParams, ForwardResult, ServeParams
- **Server notifications**: ServerMessageParams, ServerClosedParams, ToClientRequest, ServerLog
- **System operations**: SysKillRequest/Response, FsSinglePathRequest, FsFileKind, FsStatResponse, FsReadDirResponse
- **Network**: NetConnectRequest, CallServerHttpParams/Result
- **Port management**: PortPrivacy, PortProtocol, SetPortsParams/Response
- **Status tracking**: Status, StatusWithTunnelName, TunnelState enum
- **Agent/Spawn**: SpawnParams/Result, ChallengeIssueParams/Response, AcquireCliParams

### RPC Framework Types
- `rpc::Serialization` trait — pluggable serialization
- `rpc::SyncMethod`, `rpc::AsyncMethod`, `rpc::Duplex` — method signatures with Box<[u8]> I/O
- `rpc::RpcBuilder<S>`, `rpc::RpcCaller<S>`, `rpc::RpcDispatcher<S, C>` — builder pattern with context

### Error Hierarchy
- `AnyError` — type alias wrapping Box<dyn Error>
- `WrappedError` — display-based error context
- `StatusError` — HTTP response errors
- Custom errors via thiserror: OAuthError, RefreshTokenNotAvailableError, NoHomeForLauncherError, InvalidRpcDataError, CodeError

### State & Configuration
- `CommandContext` — bundles reqwest::Client, LauncherPaths, Logger, CliArgs
- `LauncherPaths` — server_cache, cli_cache DownloadCache instances
- `PersistedState<T>` — generic JSON-backed state container
- `DownloadCache` — LRU with Vec<String> state

### Authentication
- `AuthProvider` enum — Microsoft, Github variants with client_ids
- Device flow structs: DeviceCodeResponse, AuthenticationResponse, AuthenticationError

---

## Configuration

### Build Configuration
- `cli/Cargo.toml` — Workspace with 25+ dependencies
  - Async: tokio (full), futures, async-trait
  - Serialization: serde, serde_json, rmp-serde, serde_bytes
  - HTTP: reqwest (native-tls), hyper
  - CLI: clap (derive), clap_lex
  - Logging: log, opentelemetry
  - Cross-platform: cfg-if, zbus (linux), core-foundation (macos), winreg/winapi (windows)
  - External crate: dev-tunnels (git: microsoft/dev-tunnels)
  - Patch: russh variants from microsoft/vscode-russh

- `cli/.cargo/config.toml` — Platform-specific compiler flags
  - Windows x86/x64: CETCOMPAT, /guard:cf for control flow guard
  - MSVC runtime linking strategy

- `cli/build.rs` — Complex build script
  - Reads package.json version
  - Loads product.json for compile-time constants (APPLICATION_NAME, PRODUCT_NAME_LONG, TUNNEL_SERVER_QUALITIES)
  - Ensures copyright headers on all .rs files
  - Generates Windows version resources (FileVersion, ProductVersion)

### Serialization Formats
- **JSON-RPC**: line-delimited JSON (serde_json, adds \n framing)
- **MessagePack**: binary format (rmp-serde, length-prefixed via tokio_util::codec)

---

## Examples / Fixtures

**Protocol Examples in `protocol.rs`:**
- Command messages with serde derive: ForwardParams { local_port, host, port }
- Status updates: Status { state: TunnelState, tunnel_name: String }
- Server logs: LogMessage { level: String, message: &'a str, target: &'a str }
- Port privacy: enum PortPrivacy { Public, Private }
- File system enum: FsFileKind { File, Directory, Symlink, Unknown }

**Auth Examples in `auth.rs`:**
- OAuth device code flow responses
- Refresh token persistence via serde_json
- User agent construction with product constants

**State Examples:**
- DownloadCache managing LRU of server versions
- PersistedState<CliAuthTokens> for cached credentials
- LauncherPaths combining server_cache and cli_cache

---

## Documentation

- `cli/CONTRIBUTING.md` — Setup instructions
  - Rust analyzer + CodeLLDB extensions
  - Windows OpenSSL dependency via vcpkg
  - ED25519 migration note for Chromium basis
  - Debug tasks configured in VS Code
- Inline code comments indicate protocol version evolution (5 versions tracked)
- Feature gates documented via cfg attributes (linux-secret-service, platform-windows, platform-macos)

---

## Notable Clusters

### 1. **Tunneling Subsystem** (25 files)
   - Core: control_server.rs (hub), protocol.rs (schema), dev_tunnels.rs (auth)
   - Transports: server_bridge.rs, server_multiplexer.rs, local_forwarding.rs
   - Platform services: service*.rs (4 variants), nosleep*.rs (4 variants), shutdown_signal.rs
   - Utility: challenge.rs, wsl_detect.rs, paths.rs, legal.rs
   - Client: singleton_server.rs, singleton_client.rs, code_server.rs, agent_host.rs

**Integration pattern**: control_server.rs is the central message dispatcher, calls into subsystems via registered handlers, uses tokio::spawn for per-connection tasks.

### 2. **RPC & Serialization** (5 files)
   - Generic framework: rpc.rs (pluggable serialization, method registry)
   - Concrete implementations: json_rpc.rs, msgpack_rpc.rs (with framing)
   - Utilities: async_pipe.rs (channel-based duplex), singleton.rs (IPC via file/socket)

**Integration pattern**: RpcBuilder accepts S: Serialization, yields RpcCaller for clients and RpcDispatcher for servers; supports both sync and async method handlers.

### 3. **Async I/O & Process Management** (6 files)
   - Streaming: util/http.rs (AsyncRead wrappers), util/io.rs (copy_async_progress)
   - Processes: util/command.rs (tokio::process spawning with timeout/kill)
   - Concurrency: util/sync.rs (Barrier, RwLockValueExt, WaitGroup), util/ring_buffer.rs
   - Utilities: util/app_lock.rs, util/file_lock.rs (cross-platform)

**Integration pattern**: Heavy use of tokio-util, futures trait objects, Pin<Box<dyn AsyncRead>> for streaming, tokio::time::timeout for deadline handling.

### 4. **State & Persistence** (3 files)
   - Generic container: state.rs (PersistedStateContainer with serde_json backend)
   - Download cache: download_cache.rs (LRU with staging pattern)
   - Auth persistence: auth.rs (device flow + refresh tokens)

**Integration pattern**: All state is JSON-backed via serde, file permissions are platform-controlled (0o600 on Unix), atomic move pattern for crash-safety.

### 5. **Platform Abstraction** (7 files)
   - Services: service.rs (trait), service_{linux,macos,windows}.rs
   - Sleep inhibition: nosleep.rs (trait), nosleep_{linux,macos,windows}.rs
   - Detection: os.rs, machine.rs, is_integrated.rs, wsl_detect.rs

**Integration pattern**: Compile-time cfg gating with fallback impls, trait-based abstraction for alternative implementations (e.g., systemd vs launchd).

### 6. **CLI & Command Dispatch** (9 files)
   - Argument parsing: commands/args.rs (clap derive for IntegratedCli, StandaloneCli)
   - Context: commands/context.rs (CommandContext bundling HTTP, paths, logging)
   - Handlers: commands/{tunnels,serve_web,agent_host,update,version}.rs (async fn matching Command enum)
   - Entry: bin/code/main.rs (tokio::main router)

**Integration pattern**: clap Parser derives generate help/validation; main() routes to handler functions that accept CommandContext; logging installed globally via macro.

---

## Dependency Summary by Concern

| Concern | Crates | Notes |
|---------|--------|-------|
| **Async Runtime** | tokio (full), futures, async-trait | tokio::spawn extensively used; tokio_util for codecs |
| **Serialization** | serde, serde_json, rmp-serde, serde_bytes | JSON for RPC, msgpack for binary; custom Serialization trait |
| **HTTP** | reqwest (native-tls), hyper (0.14) | reqwest for clients, hyper for server; SimpleResponse wrapper |
| **CLI** | clap (derive), clap_lex, shell-escape | Clap for arg parsing; minimal custom shell handling |
| **Logging** | log, opentelemetry | opentelemetry for tracing; log macro for structured logging |
| **Crypto/Auth** | sha2, base64, keyring, tunnels::contracts | OAuth device flow; system keyring integration |
| **IPC** | zbus (linux), hyper, custom socket code | systemd integration via zbus; Unix sockets for singleton |
| **Process** | std::process::Command (wrapped) | Spawning with stdio/env capture; kill signal handling |
| **OS Abstraction** | cfg-if, platform-specific cfg, core-foundation, winapi, winreg | Extensive cfg gating; NSWindow via core-foundation on macOS |
| **File I/O** | tokio::fs, tar, zip, flate2 | Async file ops; archive extraction |
| **Utilities** | dirs, rand, chrono, uuid, gethostname, regex, lazy_static, dialoguer, indicatif | Standard ecosystem; dialoguer for prompts, indicatif for progress |

---

## Code Organization Insights

1. **No tests directory**: Testing is handled at repo level, not within cli/. Suggests end-to-end testing approach.
2. **Heavy tokio usage**: 44+ tokio::spawn calls across codebase; every subsystem is async-first with explicit spawn management.
3. **Pluggable serialization**: RPC framework accepts any Serialization impl; allows JSON and msgpack to coexist.
4. **Platform traits**: Services, sleep inhibition, signal handling all use trait-based abstraction with cfg-gated impls.
5. **Generic state persistence**: PersistedState<T> pattern eliminates boilerplate; all config is serde-json.
6. **External tunnel protocol**: Heavy reliance on microsoft/dev-tunnels crate for tunnel creation/management; not reimplemented.
7. **Build-time configuration**: Constants injected via build.rs from product.json (versioning, branding, capabilities).
8. **Lightweight dependency model**: No ORM, no web framework, minimal abstraction layers—direct use of tokio + hyperio.

---

## Migration Readiness Assessment

This codebase demonstrates **high maturity** for Tauri migration:
- ✓ Cross-platform abstractions (linux, macos, windows via cfg)
- ✓ Async-first architecture suitable for Tauri's wry/webview backends
- ✓ Serialization framework (serde) compatible with IPC to frontends
- ✓ RPC patterns that could bridge Rust core to TypeScript UI
- ✓ Error handling and retry logic for network operations
- ✓ Singleton/lifecycle patterns for process management
- ✓ OAuth integration patterns reusable in IDE
- ✓ Streaming I/O patterns for subprocess stdio/file operations

Key gaps to address: No UI rendering (obviously—CLI only), no DOM/WebGL integration, no CSS/layout engine. Tauri would add wry for webview, while this codebase provides the foundation for backend services, IPC, and cross-platform integration.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Rust Patterns in VS Code CLI: Tauri/Rust Porting Patterns

## Partition 9: `cli/` Directory Analysis

This analysis focuses on key Rust patterns found in the VS Code CLI that would be directly applicable to porting core IDE functionality from TypeScript/Electron to Tauri/Rust.

---

#### Pattern: Serializable RPC Method Type Aliases

**Where:** `cli/src/rpc.rs:25-32`

**What:** Type aliases defining sync, async, and duplex RPC method handlers using trait objects with Arc wrapping.

```rust
pub type SyncMethod = Arc<dyn Send + Sync + Fn(Option<u32>, &[u8]) -> Option<Vec<u8>>>;
pub type AsyncMethod =
	Arc<dyn Send + Sync + Fn(Option<u32>, &[u8]) -> BoxFuture<'static, Option<Vec<u8>>>>;
pub type Duplex = Arc<
	dyn Send
		+ Sync
		+ Fn(Option<u32>, &[u8]) -> (Option<StreamDto>, BoxFuture<'static, Option<Vec<u8>>>),
>;
```

**Variations / call-sites:**
- `cli/src/rpc.rs:115-143` - Sync method registration wrapping callbacks
- `cli/src/rpc.rs:159-180` - Async method registration using `.boxed()` futures
- `cli/src/rpc.rs:483` - `tokio::spawn` invocation for async method execution

---

#### Pattern: Builder Pattern with Generic Serialization

**Where:** `cli/src/rpc.rs:49-84`

**What:** RpcBuilder uses generics and builder chaining to configure RPC methods with custom serialization strategies, returning specialized builders for method registration.

```rust
pub struct RpcBuilder<S> {
	serializer: Arc<S>,
	methods: HashMap<&'static str, Method>,
	calls: Arc<Mutex<HashMap<u32, DispatchMethod>>>,
}

impl<S: Serialization> RpcBuilder<S> {
	pub fn new(serializer: S) -> Self {
		Self {
			serializer: Arc::new(serializer),
			methods: HashMap::new(),
			calls: Arc::new(std::sync::Mutex::new(HashMap::new())),
		}
	}

	pub fn get_caller(&mut self, sender: mpsc::UnboundedSender<Vec<u8>>) -> RpcCaller<S> {
		RpcCaller {
			serializer: self.serializer.clone(),
			calls: self.calls.clone(),
			sender,
		}
	}

	pub fn methods<C: Send + Sync + 'static>(self, context: C) -> RpcMethodBuilder<S, C> {
		RpcMethodBuilder {
			context: Arc::new(context),
			serializer: self.serializer,
			methods: self.methods,
			calls: self.calls,
		}
	}
}
```

**Variations / call-sites:**
- `cli/src/commands/serve_web.rs:550` - Instantiating connection managers with Arc
- `cli/src/tunnels/control_server.rs:77-87` - AgentHostManager using Arc<Self> pattern

---

#### Pattern: Async Trait with BoxedFuture Return Types

**Where:** `cli/src/tunnels/dev_tunnels.rs:94-122`

**What:** Using `#[async_trait]` macro for traits with async methods, returning `BoxFuture<'static>` for trait objects and dynamic dispatch.

```rust
#[async_trait]
trait AccessTokenProvider: Send + Sync {
	async fn refresh_token(&self) -> Result<String, WrappedError>;

	fn keep_alive(&self) -> BoxFuture<'static, Result<(), AnyError>>;
}

struct StaticAccessTokenProvider(String);

#[async_trait]
impl AccessTokenProvider for StaticAccessTokenProvider {
	async fn refresh_token(&self) -> Result<String, WrappedError> {
		Ok(self.0.clone())
	}

	fn keep_alive(&self) -> BoxFuture<'static, Result<(), AnyError>> {
		futures::future::pending().boxed()
	}
}
```

**Variations / call-sites:**
- `cli/src/util/sync.rs:38-43` - Custom `Receivable<T>` async trait
- `cli/src/tunnels/dev_tunnels.rs:125-142` - LookupAccessTokenProvider implementation

---

#### Pattern: Tokio Spawn for Concurrent Task Management

**Where:** `cli/src/commands/serve_web.rs:284-310`

**What:** Spawning HTTP connections and response forwarding in separate tokio tasks, dropping handles after request completes for lifecycle management.

```rust
async fn forward_http_req_to_server(
	(rw, handle): (AsyncPipe, ConnectionHandle),
	req: Request<Body>,
) -> Response<Body> {
	let (mut request_sender, connection) =
		match hyper::client::conn::Builder::new().handshake(rw).await {
			Ok(r) => r,
			Err(e) => return response::connection_err(e),
		};

	tokio::spawn(connection);

	let res = request_sender
		.send_request(req)
		.await
		.unwrap_or_else(response::connection_err);

	drop(handle);

	res
}
```

**Variations / call-sites:**
- `cli/src/rpc.rs:483-519` - Streaming data handling in spawned task
- `cli/src/tunnels/agent_host.rs:137-140` - Running server process in spawned task
- `cli/src/tunnels/control_server.rs:211-237` - Multiple concurrent connection handlers

---

#### Pattern: Serde Tagging and Serialization Attributes

**Where:** `cli/src/tunnels/protocol.rs:14-23`

**What:** Enum serialization with serde tagging for protocol messages, using `#[serde(tag, content, rename_all)]` for compact JSON representation.

```rust
#[derive(Serialize, Debug)]
#[serde(tag = "method", content = "params", rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub enum ClientRequestMethod<'a> {
	servermsg(RefServerMessageParams<'a>),
	serverclose(ServerClosedParams),
	serverlog(ServerLog<'a>),
	makehttpreq(HttpRequestParams<'a>),
	version(VersionResponse),
}
```

**Variations / call-sites:**
- `cli/src/tunnels/protocol.rs:26-50` - Struct deserialization with `serde_bytes` for binary data
- `cli/src/options.rs:12-14` - ValueEnum with Serialize/Deserialize for CLI arguments
- `cli/src/auth.rs:104-106` - Credential storage serialization

---

#### Pattern: Arc<Mutex<T>> for Shared Mutable State

**Where:** `cli/src/rpc.rs:49-52, 90`

**What:** Thread-safe shared mutable state using `Arc<Mutex<HashMap>>` for managing concurrent RPC calls and method dispatch tracking.

```rust
pub struct RpcBuilder<S> {
	serializer: Arc<S>,
	methods: HashMap<&'static str, Method>,
	calls: Arc<Mutex<HashMap<u32, DispatchMethod>>>,
}

pub struct RpcMethodBuilder<S, C> {
	context: Arc<C>,
	serializer: Arc<S>,
	methods: HashMap<&'static str, Method>,
	calls: Arc<Mutex<HashMap<u32, DispatchMethod>>>,
}
```

**Variations / call-sites:**
- `cli/src/commands/serve_web.rs:515` - ConnectionStateMap type alias
- `cli/src/tunnels/local_forwarding.rs:97` - Watch sender wrapped in Arc<Mutex>
- `cli/src/tunnels/dev_tunnels.rs:131` - Initial token storage for access provider

---

#### Pattern: Custom Synchronization Primitives with Tokio Channels

**Where:** `cli/src/util/sync.rs:12-70`

**What:** Building reusable synchronization barriers using `watch::channel`, supporting one-time opening with optional value.

```rust
#[derive(Clone)]
pub struct Barrier<T>(watch::Receiver<Option<T>>)
where
	T: Clone;

impl<T> Barrier<T> where T: Clone {
	pub async fn wait(&mut self) -> Result<T, RecvError> {
		loop {
			self.0.changed().await?;
			if let Some(v) = self.0.borrow().clone() {
				return Ok(v);
			}
		}
	}

	pub fn is_open(&self) -> bool {
		self.0.borrow().is_some()
	}
}

pub fn new_barrier<T>() -> (Barrier<T>, BarrierOpener<T>)
where
	T: Clone,
{
	let (closed_tx, closed_rx) = watch::channel(None);
	(Barrier(closed_rx), BarrierOpener(Arc::new(closed_tx)))
}
```

**Variations / call-sites:**
- `cli/src/tunnels/agent_host.rs:65-67` - Barrier for server readiness signaling
- `cli/src/tunnels/agent_host.rs:137-147` - Awaiting barrier in lifecycle management

---

## Cross-Cutting Patterns

**Error Handling:** Custom error types using `thiserror::Error` (cli/src/util/errors.rs) with Display and From trait implementations for wrapping.

**Async Closures:** Using `move` keyword in `tokio::spawn` closures to capture Arc-wrapped state (cli/src/rpc.rs:483).

**Protocol Composition:** Modular protocol structures combining serde derives with lifetime parameters for zero-copy serialization (cli/src/tunnels/protocol.rs).

**Trait Object Types:** Preferred over generics when performance at call-site doesn't matter; used extensively in RPC method registration to allow dynamic dispatch.

## Takeaway for IDE Porting

The CLI demonstrates mature Rust patterns for:
1. Building async RPC systems with trait objects and type aliases
2. Spawning and managing concurrent tasks with tokio
3. Serializing complex protocols with serde tagging
4. Thread-safe state management with Arc<Mutex> and watch channels
5. Custom synchronization primitives for coordination

These patterns directly transfer to Tauri/Rust IDE development for IPC, plugin systems, and background task management.

## External References
<!-- Source: codebase-online-researcher sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
