# Partition 9 of 80 — Findings

## Scope
`cli/` (75 files, 20,107 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Rust CLI Codebase Locator — Partition 9

## Executive Summary

The VS Code CLI is a substantial (~20K LOC) production Rust codebase demonstrating how to port core IDE functionality to Rust. It implements tunneling, remote agents, updates, authentication, HTTP/RPC infrastructure, and cross-platform services—all patterns directly applicable to porting the full IDE from TypeScript/Electron to Tauri/Rust.

---

## Implementation

### Core Infrastructure

**RPC Layer** — `/home/norinlavaee/projects/vscode-atomic/cli/src/rpc.rs` (~290 LOC)
- Generic, transport-agnostic RPC dispatcher supporting sync/async/duplex methods
- Supports both JSON-RPC and msgpack serialization via pluggable serializer trait
- `RpcBuilder<S>` pattern for registering methods and creating callers/dispatchers
- Used throughout to decouple message formats from business logic

**Serialization Adapters**
- `/home/norinlavaee/projects/vscode-atomic/cli/src/json_rpc.rs` — JSON-RPC wrapper
- `/home/norinlavaee/projects/vscode-atomic/cli/src/msgpack_rpc.rs` — MessagePack serialization
- Both implement `Serialization` trait for pluggable format support

**HTTP Client** — `/home/norinlavaee/projects/vscode-atomic/cli/src/util/http.rs` (~330 LOC)
- Built on `reqwest` (0.13) for async HTTP with native-tls
- `BoxedHttp` trait for abstract HTTP layer; `ReqwestSimpleHttp` implementation
- Download streaming with progress reporting
- Header/status code handling for various protocols
- Used for updates, auth, and control operations

**Authentication** — `/home/norinlavaee/projects/vscode-atomic/cli/src/auth.rs` (~760 LOC)
- Device code flow for Microsoft/GitHub OAuth
- Token refresh with `jiff` time library
- Keyring integration via `keyring` crate (platform-specific: openssl-rt-tokio-crypto-openssl on Linux, native on Windows/macOS)
- Integration with tunnels management API

**State Management** — `/home/norinlavaee/projects/vscode-atomic/cli/src/state.rs` (~280 LOC)
- `LauncherPaths` struct for managing cache and data directories
- `PersistedStateContainer<T>` for serializing/deserializing JSON state with file locking
- Cross-platform paths handling via `dirs` crate
- Download cache management

### Command Framework

**Command Context** — `/home/norinlavaee/projects/vscode-atomic/cli/src/commands/context.rs`
- `CommandContext` struct wrapping HTTP client, paths, logger
- Used by all subcommands for common dependencies

**Command Args** — `/home/norinlavaee/projects/vscode-atomic/cli/src/commands/args.rs` (~850 LOC)
- Clap-based CLI arg parsing with integrated/standalone modes
- `AnyCli` enum supporting different command sets
- Per-command arg types: `AgentHostArgs`, `ServeWebArgs`, `TunnelsArgs`, etc.

**Output Formatting** — `/home/norinlavaee/projects/vscode-atomic/cli/src/commands/output.rs` (~380 LOC)
- `OutputFormatter` trait with plain/json/quiet implementations
- Progress indicators via `indicatif` crate
- Table rendering for process listings, tunnels, etc.

### Server & Agent Management

**Agent Host** — `/home/norinlavaee/projects/vscode-atomic/cli/src/commands/agent_host.rs` (~460 LOC)
- Launches and manages VS Code server instances
- WebSocket proxy from local TCP to server's agent socket
- Auto-update checking in background
- Connection token management
- `AgentHostLockData` for inter-process discovery

**Server Management** — `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/code_server.rs` (~860 LOC)
- `CodeServer` trait for abstract server operations
- `SocketCodeServer` implementation wrapping spawned process
- Async command execution via `tokio::process::Command`
- Stdio/stderr capture with buffering
- Graceful shutdown and error handling
- Platform-specific path handling

**Version Management** — `/home/norinlavaee/projects/vscode-atomic/cli/src/desktop/version_manager.rs` (~1.2K LOC)
- Tracks and manages downloaded server versions
- Download caching with concurrent access protection
- Version validation and cleanup
- Uses `tokio::fs` for async file operations

### Tunneling & Remote

**Dev Tunnels** — `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/dev_tunnels.rs` (~1.2K LOC)
- Integrates Microsoft dev-tunnels library
- Manages tunnel lifecycle (creation, auth, relay)
- `ActiveTunnel` tracking with persistence
- Connection state management
- Relay tunnel host abstraction

**Control Server** — `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/control_server.rs` (~1.5K LOC)
- Main multiplex server handling client connections
- Implements protocol methods for:
  - Server lifecycle (spawn, acquire, update)
  - Filesystem operations (read/write/stat/rm/mkdir)
  - Process execution (spawn, kill, get env)
  - HTTP forwarding
  - Port forwarding
  - Challenge-response auth
- Uses `tokio::select!` for concurrent operation handling
- `ServerMultiplexer` for managing multiple client bridges

**Port Forwarding** — `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/port_forwarder.rs` (~130 LOC)
- Bidirectional TCP/Unix socket forwarding
- `PortForwarding` struct managing forward rules
- `PortForwardingProcessor` for active forwarding

**Server Bridge** — `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/server_bridge.rs` (~60 LOC)
- Per-client proxy connection to server
- Async read/write from WebSocket to server stdio

**Protocol Definitions** — `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/protocol.rs` (~340 LOC)
- Serde-based request/response types for all server operations
- Enums for operation types, port privacy, client message variants
- RPC method constants (METHOD_CHALLENGE_ISSUE, etc.)

**Authentication** — `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/challenge.rs` (~50 LOC)
- Challenge-response crypto (ED25519 signing via `tunnels` crate)
- Token-based auth flow

### Async I/O & Pipes

**Async Pipes** — `/home/norinlavaee/projects/vscode-atomic/cli/src/async_pipe.rs` (~300 LOC)
- Cross-platform abstraction over IPC: Unix sockets on Unix, named pipes on Windows
- `AsyncPipe` enum with `AsyncRead`/`AsyncWrite` implementations
- `AsyncPipeListener` for server-side acceptance
- Retry logic for transient pipe binding failures
- TCP fallback for Windows when named pipes unavailable

**Async I/O Utilities** — `/home/norinlavaee/projects/vscode-atomic/cli/src/util/io.rs` (~360 LOC)
- `copy_async_progress<T: ReportCopyProgress>` for streaming with progress callbacks
- `ReadBuffer` for buffered async reading
- Async pipe handling with silent/progress modes
- Ring buffer for buffering

### Process Management

**Command Execution** — `/home/norinlavaee/projects/vscode-atomic/cli/src/util/command.rs` (~180 LOC)
- `new_tokio_command(exe)` wrapper for spawning processes
- `CommandContext` for stdio capture
- Platform-specific signal handling (SIGTERM on Unix, TerminateProcess on Windows)
- Batch process killing

**Platform Services** — `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/service*.rs` (4 files, ~1.5K LOC)
- `service.rs` — Generic interface for platform service registration
- `service_linux.rs` — systemd integration via D-Bus (zbus crate)
- `service_macos.rs` — LaunchAgent integration
- `service_windows.rs` — Windows service helper script generation
- Abstraction for auto-launch on login

**Singleton Lock** — `/home/norinlavaee/projects/vscode-atomic/cli/src/singleton.rs` (~270 LOC)
- IPC singleton pattern preventing multiple instances
- File-based locking with timeout retry logic
- Async socket connection pooling

### Utilities

**Error Handling** — `/home/norinlavaee/projects/vscode-atomic/cli/src/util/errors.rs` (~570 LOC)
- `CodeError` enum with context-specific variants
- `WrappedError` for error chain tracking
- `AnyError` type alias for `Box<dyn Error>`
- thiserror integration for ergonomic error definition
- Integration with HTTP status codes

**Logging** — `/home/norinlavaee/projects/vscode-atomic/cli/src/log.rs` (~340 LOC)
- File-based logging with rotation
- `log` crate facade with custom formatting
- Timestamp integration via `jiff`
- Cross-platform log file paths

**Synchronization** — `/home/norinlavaee/projects/vscode-atomic/cli/src/util/sync.rs` (~220 LOC)
- `Barrier<T>` primitive for coordinating async tasks
- `BarrierOpener` for opening barriers from elsewhere

**Machine/Hardware** — `/home/norinlavaee/projects/vscode-atomic/cli/src/util/machine.rs` (~140 LOC)
- CPU/memory detection via `sysinfo`
- Platform-specific process APIs

**Archive Handling**
- `/home/norinlavaee/projects/vscode-atomic/cli/src/util/tar.rs` (~140 LOC) — tar extraction
- `/home/norinlavaee/projects/vscode-atomic/cli/src/util/zipper.rs` (~200 LOC) — zip extraction

**Misc Utilities**
- `/home/norinlavaee/projects/vscode-atomic/cli/src/util/input.rs` — Interactive prompts via `dialoguer`
- `/home/norinlavaee/projects/vscode-atomic/cli/src/util/file_lock.rs` — File-based locks
- `/home/norinlavaee/projects/vscode-atomic/cli/src/util/app_lock.rs` — Exclusive app locks
- `/home/norinlavaee/projects/vscode-atomic/cli/src/util/prereqs.rs` (~420 LOC) — System prerequisites checking
- `/home/norinlavaee/projects/vscode-atomic/cli/src/util/os.rs` — OS detection
- `/home/norinlavaee/projects/vscode-atomic/cli/src/util/is_integrated.rs` — VS Code integration detection

**Updates** — `/home/norinlavaee/projects/vscode-atomic/cli/src/update_service.rs` (~360 LOC)
- `UpdateService` for checking/downloading new versions
- Platform enum mapping to release artifacts
- Update release structs with serde support
- Version comparison logic

---

## Tests

Tests are embedded inline with `#[tokio::test]` attribute (tokio runtime injection):

**RPC Tests** — `/home/norinlavaee/projects/vscode-atomic/cli/src/rpc.rs` (lines 718-760)
- Duplex stream testing
- Serialization round-trip validation

**I/O Tests** — `/home/norinlavaee/projects/vscode-atomic/cli/src/util/io.rs` (3 test functions)
- Progress tracking validation
- Buffering behavior

**Singleton Tests** — `/home/norinlavaee/projects/vscode-atomic/cli/src/singleton.rs` (2 test functions)
- Barrier blocking/opening coordination

**Sync Tests** — `/home/norinlavaee/projects/vscode-atomic/cli/src/util/sync.rs` (2 test functions)
- Barrier spawn coordination

**Version Manager Tests** — `/home/norinlavaee/projects/vscode-atomic/cli/src/desktop/version_manager.rs` (3 test functions)
- Version caching logic
- Concurrent access

All tests use `tokio::spawn` for concurrent task spawning within async contexts.

---

## Types / Interfaces

### Core Traits

**Serialization** — `/home/norinlavaee/projects/vscode-atomic/cli/src/rpc.rs`
```rust
pub trait Serialization: Send + Sync + 'static {
  fn serialize(&self, value: impl Serialize) -> Vec<u8>;
  fn deserialize<P: DeserializeOwned>(&self, b: &[u8]) -> Result<P, AnyError>;
}
```

**SimpleHttp** — `/home/norinlavaee/projects/vscode-atomic/cli/src/util/http.rs`
- Abstract HTTP client interface
- Implementations: `ReqwestSimpleHttp`, `DelegatedSimpleHttp`, `FallbackSimpleHttp`

**CodeServer** — `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/code_server.rs`
- Abstract server interface with spawn/wait/kill operations
- Implementation: `SocketCodeServer` wrapping `tokio::process::Child`

**Logger** — `/home/norinlavaee/projects/vscode-atomic/cli/src/log.rs`
- File-based logging with rotating file handles
- Used throughout via `crate::log` macro

**Platform** — `/home/norinlavaee/projects/vscode-atomic/cli/src/update_service.rs`
- Enum mapping host architectures to release artifact names
- Methods: `archive()`, `headless()`, `cli()`, `web()`, `env_default()`

### Command Types

- `CommandContext` — HTTP client, paths, logger wrapper
- `AgentHostArgs`, `ServeWebArgs`, `TunnelsArgs`, etc. — Per-command clap structs
- `AnyCli` — Union type for integrated vs. standalone modes

### Message Types

All in `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/protocol.rs`:
- `HttpRequestParams`, `HttpHeadersParams`, `HttpBodyParams`
- `SpawnParams`, `SpawnResult`
- `NetConnectRequest`, `ForwardParams`, `ForwardResult`
- `FsStatResponse`, `FsReadDirEntry`, `FsReadDirResponse`
- `ServerLog`, `ServerMessageParams`
- `ChallengeIssueParams`, `ChallengeIssueResponse`, `ChallengeVerifyParams`
- `ClientRequestMethod`, `ToClientRequest` (protocol enum)

### Structs

- `AgentHostLockData` — Persisted agent host metadata (address, PID, token, tunnel name)
- `ServerBuilder`, `SocketCodeServer` — Server management
- `ActiveTunnel` — Tunnel lifecycle state
- `PortForwarding`, `PortForwardingProcessor` — Port routing
- `RpcBuilder<S>`, `RpcCaller<S>`, `RpcDispatcher<S>` — RPC infrastructure
- `LauncherPaths`, `PersistedStateContainer<T>` — State management
- `Auth`, various Auth*-specific types — Authentication

---

## Configuration

**Build Configuration** — `/home/norinlavaee/projects/vscode-atomic/cli/build.rs` (~250 LOC)
- Reads `product.json` and `product.overrides.json` at build time
- Sets cargo rustc-env variables from product configuration
- Generates Windows resource manifests with version info
- Ensures all Rust files have Microsoft copyright headers
- Validates build integrity before compilation

**Cargo Dependencies** — `/home/norinlavaee/projects/vscode-atomic/cli/Cargo.toml`
- **Async runtime**: tokio (1.52, full features)
- **HTTP**: reqwest (0.13), hyper (1), hyper-util, http-body-util
- **Serialization**: serde, serde_json, rmp-serde (msgpack), serde_bytes
- **CLI**: clap (4.3), dialoguer (0.10)
- **Networking**: tokio-tungstenite (websockets), zbus (D-Bus on Linux)
- **Authentication**: keyring (2.0.3, platform-specific), jiff (time), uuid
- **Compression**: flate2, zip, tar
- **Tunnels**: Microsoft dev-tunnels fork (git dependency, no tauri)
- **System**: sysinfo, libc, gethostname, dirs
- **Utilities**: log, regex, rand, shell-escape, futures, pin-project, console, bytes, indicatif, tempfile
- **Windows-specific**: winreg, winapi, windows-sys
- **macOS-specific**: core-foundation
- **Linux-specific**: zbus (D-Bus)

**Patched Dependencies**:
- russh, russh-cryptovec, russh-keys — SSH implementation (Microsoft fork)

**No Tauri Dependency** — The CLI does not use Tauri; it's a pure server/daemon application.

---

## Examples / Fixtures

### Demonstrative Patterns

**Main Entry** — `/home/norinlavaee/projects/vscode-atomic/cli/src/bin/code/main.rs` (~200 LOC)
- `#[tokio::main]` async runtime setup
- Legacy arg parsing for backward compatibility
- Integrated vs. standalone CLI mode dispatch
- Per-command handler routing

**Legacy Args** — `/home/norinlavaee/projects/vscode-atomic/cli/src/bin/code/legacy_args.rs`
- Backward compatibility parsing for older command formats

**Serve Web Command** — `/home/norinlavaee/projects/vscode-atomic/cli/src/commands/serve_web.rs` (~840 LOC)
- HTTP server using hyper + hyper-util
- WebSocket upgrade handling with `tokio::spawn`
- Client tracking via `tokio::sync::watch::channel`
- Server idle timeout logic with `tokio::select!`
- Complex async orchestration example

**Tunnels Command** — `/home/norinlavaee/projects/vscode-atomic/cli/src/commands/tunnels.rs` (~700 LOC)
- Full tunnel setup, upgrade flow
- Control server integration
- Bridge multiplexing

**Agent Commands**
- `agent_host.rs` — Agent host launcher (shown above)
- `agent_stop.rs`, `agent_kill.rs`, `agent_logs.rs`, `agent_ps.rs` — Agent lifecycle management

**Download Cache** — `/home/norinlavaee/projects/vscode-atomic/cli/src/download_cache.rs` (~160 LOC)
- Atomic file operations for concurrent safety
- Rename-based atomic writes on all platforms

**Self-Update** — `/home/norinlavaee/projects/vscode-atomic/cli/src/self_update.rs` (~200 LOC)
- CLI self-update mechanism
- Version/quality/platform tracking
- Old binary cleanup

---

## Documentation

- **CONTRIBUTING.md** — Setup instructions for Windows (OpenSSL vcpkg), debugging (rust-analyzer, CodeLLDB), workspace configuration
- **Inline comments** — Microsoft copyright headers on all files; sparse but present task comments (e.g., "todo: reduce exported surface area")
- **Test names** — Descriptive test function names (e.g., `test_barrier_close_after_spawn`)
- **Type/method documentation** — Minimal rustdoc comments; relies on code clarity

---

## Notable Clusters

### Async Patterns
- **Ubiquitous tokio usage** — 100+ `tokio::` references across codebase
- **tokio::select!** — For concurrent operation handling (control flow), heavily used in server loops
- **tokio::spawn** — Task spawning for background operations (update checks, connection handling)
- **Async trait methods** — Allowed via `#![allow(async_fn_in_trait)]` in lib.rs
- **Pin<> and Futures** — Used in RPC layer for boxed futures, custom async trait objects

### RPC & Multiplexing
- **Generic RPC dispatch** — Pluggable serializers (JSON, msgpack) enable protocol flexibility
- **Method registration** — Builder pattern for declaring RPC methods with context injection
- **Duplex streams** — Support for bidirectional communication on single connection
- **Socket signal protocol** — Custom binary protocol multiplexing multiple logical connections over single wire

### Cross-Platform Abstraction
- **cfg_if! macros** — Conditional compilation for Unix (UnixStream) vs. Windows (named pipes)
- **Platform enum** — Maps host arch to release artifact variants
- **Service abstraction** — Pluggable platform service registration (systemd, LaunchAgent, Windows service helper)
- **Async pipes** — Unified IPC interface across platforms

### HTTP & Networking
- **Streaming downloads** — Progress reporting with `ReportCopyProgress` trait
- **HTTP delegation** — Abstract HTTP interface allowing fallback implementations
- **WebSocket upgrade** — Hyper HTTP/1.1 upgrade to bidirectional streams
- **Port forwarding** — Abstraction for routing multiple forwarded ports

### Concurrency Patterns
- **Arc<Mutex<>>** — For shared mutable state across tasks
- **mpsc channels** — For task-to-task communication (Sender/Receiver)
- **oneshot channels** — For one-time signaling between tasks
- **Barrier<T>** — Custom synchronization primitive for coordinating multiple waiters
- **watch::Sender** — For broadcasting updates to multiple subscribers

### Error Handling
- **thiserror crate** — Ergonomic error enum derivation
- **Error wrapping chain** — `WrappedError` preserving context across function boundaries
- **Status error mapping** — HTTP status codes → `CodeError` variants
- **Result<T, AnyError>** — Ubiquitous use of boxed errors for simplicity

### Code Organization
- **Module-per-feature** — commands/, tunnels/, util/ directories contain related functionality
- **Trait-based design** — Serialization, SimpleHttp, CodeServer traits enable pluggability
- **Builder pattern** — RpcBuilder, ServerBuilder for fluent API construction
- **Arc/clone-heavy** — Shared ownership pervasive due to async task spawning

### State Management
- **Persisted JSON state** — PersistedStateContainer<T> for durable configuration
- **File-based locks** — For inter-process coordination (singleton, agent host discovery)
- **Download cache** — Deduplicated version downloads with atomic operations
- **Version tracking** — Platform-specific artifact naming and caching

### Performance Considerations
- **Streaming/buffering** — Async copy with progress to avoid loading large files
- **Connection pooling** — HTTP client reuse across commands
- **Selective spawning** — Background tasks (updates) spawned asynchronously to avoid blocking
- **Atomic writes** — File-based operations use rename to prevent corruption
- **Timeout logic** — Server idle shutdown, connection establishment timeouts

---

## Technical Debt & Notes

1. **No Tauri** — This is a server/daemon application; a full IDE port would need to wrap this backend with a Tauri GUI layer.

2. **Incomplete Module Export** — `lib.rs` comment: "we should reduce the exported surface area over time as things are moved into a common CLI".

3. **Async Trait** — Nightly feature allowed to simplify async method signatures; may need stabilization workarounds for production.

4. **Error Context** — Error handling is functional but lacks detailed context in some code paths; wrapping is encouraged but inconsistently applied.

5. **Documentation** — Sparse rustdoc comments; code readability is prioritized over documentation strings.

6. **Platform Services** — Windows service integration is via helper script, not direct API; some launch-on-login scenarios may be incomplete.

7. **Tunnel Management** — Relies on external `tunnels` crate (Microsoft fork); updates to that dependency require repo synchronization.

8. **Test Coverage** — Unit tests present but focused on infrastructure (RPC, I/O, sync); business logic testing is limited.

---

## Summary for IDE Porting

**What exists in this CLI that's applicable to a full IDE port:**

1. **Async/tokio foundation** — The CLI demonstrates production-grade async patterns suitable for an IDE's edit engine, language server integration, and debugging backend.

2. **RPC & protocol abstraction** — The pluggable serialization and multiplex protocol enable efficient IDE-client communication, especially for remote scenarios.

3. **Cross-platform abstraction** — IPC pipes, service registration, and file operations show how to abstract Unix/Windows/macOS differences.

4. **HTTP/download infrastructure** — Streaming downloads, progress reporting, and retry logic are reusable for extension installation, update delivery.

5. **Process & terminal management** — The code server spawning and stdio capture patterns are directly applicable to integrated terminals and language server processes.

6. **Authentication** — Token-based auth with refresh and device code flow provides a foundation for user identity in remote IDE scenarios.

7. **State & caching** — Persisted state management and download caching handle offline scenarios and version management.

8. **Error handling & logging** — Error chain tracking and file-based logging are production patterns ready for IDE adoption.

**Major gaps for IDE:**

1. **No Tauri bindings** — This CLI is server-only; a full IDE would need Tauri FFI for GUI integration.
2. **No editor core** — The CLI doesn't implement text editing; that would be a new Rust module (potentially ported from TypeScript).
3. **No LSP client** — While RPC infrastructure exists, LSP protocol support isn't in the CLI; that's an additional layer.
4. **No debug adapter** — Debug protocol isn't implemented; would be a new subsystem.
5. **No extension system** — Plugin architecture is absent; would require significant new infrastructure.

**Effort estimate for a Tauri-based IDE from this codebase:** The CLI provides perhaps 20-30% of the low-level infrastructure. A full IDE would require re-architecting the editor core, language features, and UI, then integrating this async foundation as the backend.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Partition 9: Rust CLI Precedent Patterns for Tauri/Rust VS Code Port

## Analysis Summary

The VS Code CLI (`cli/src/`) provides significant precedent for a hypothetical Tauri/Rust port of VS Code core functionality. Key findings show **transport-agnostic async RPC**, **duplex streaming**, **error handling via typed enums**, and **multiplatform async I/O abstractions** suitable for IDE-level demands.

---

## Pattern Findings

#### Pattern 1: Error Handling via Typed Enum (`#[derive(Error)]`)
**Where:** `cli/src/util/errors.rs:440-524`
**What:** Comprehensive error types for all system operations (RPC, processes, filesystem, auth, networking).

```rust
#[derive(Error, Debug)]
pub enum CodeError {
    #[error("could not connect to socket/pipe: {0:?}")]
    AsyncPipeFailed(std::io::Error),
    #[error("could not listen on socket/pipe: {0:?}")]
    AsyncPipeListenerFailed(std::io::Error),
    #[error("could not create singleton lock file: {0:?}")]
    SingletonLockfileOpenFailed(std::io::Error),
    #[error("rpc call failed: {0:?}")]
    TunnelRpcCallFailed(ResponseError),
    #[error("failed to run command \"{command}\" (code {code}): {output}")]
    CommandFailed {
        command: String,
        code: i32,
        output: String,
    },
    #[error("platform not currently supported: {0}")]
    UnsupportedPlatform(String),
    // ... more variants
}
```

**Variations / call-sites:** `cli/src/util/errors.rs:526-555` (macro-based AnyError aggregation).

#### Pattern 2: Async RPC with Duplex Streaming
**Where:** `cli/src/rpc.rs:147-199`
**What:** Async RPC method registration that returns boxed futures; supports sync, async, and duplex (bidirectional stream) patterns.

```rust
pub fn register_async<P, R, Fut, F>(&mut self, method_name: &'static str, callback: F)
where
    P: DeserializeOwned + Send + 'static,
    R: Serialize + Send + Sync + 'static,
    Fut: Future<Output = Result<R, AnyError>> + Send,
    F: (Fn(P, Arc<C>) -> Fut) + Clone + Send + Sync + 'static,
{
    let serial = self.serializer.clone();
    let context = self.context.clone();
    self.methods.insert(
        method_name,
        Method::Async(Arc::new(move |id, body| {
            let param = match serial.deserialize::<RequestParams<P>>(body) {
                Ok(p) => p,
                Err(err) => {
                    return future::ready(id.map(|id| {
                        serial.serialize(ErrorResponse {
                            id,
                            error: ResponseError {
                                code: 0,
                                message: format!("{err:?}"),
                            },
                        })
                    }))
                    .boxed();
                }
            };
            let callback = callback.clone();
            let serial = serial.clone();
            let context = context.clone();
            let fut = async move {
                match callback(param.params, context).await {
                    Ok(result) => {
                        id.map(|id| serial.serialize(&SuccessResponse { id, result }))
                    }
                    Err(err) => id.map(|id| {
                        serial.serialize(ErrorResponse {
                            id,
                            error: ResponseError {
                                code: -1,
                                message: format!("{err:?}"),
                            },
                        })
                    }),
                }
            };
            fut.boxed()
        })),
    );
}
```

**Variations / call-sites:** 
- `cli/src/rpc.rs:101-145` (sync variant)
- `cli/src/rpc.rs:203-272` (duplex streaming with `tokio::io::duplex()`)

#### Pattern 3: Transport-Agnostic Async I/O (Unix/Windows Abstraction)
**Where:** `cli/src/async_pipe.rs:16-176`
**What:** Cross-platform abstraction for Unix sockets and Windows named pipes using cfg_if; provides unified AsyncRead/AsyncWrite interface.

```rust
cfg_if::cfg_if! {
    if #[cfg(unix)] {
        pub type AsyncPipe = tokio::net::UnixStream;
        pub async fn get_socket_rw_stream(path: &Path) -> Result<AsyncPipe, CodeError> {
            tokio::net::UnixStream::connect(path)
                .await
                .map_err(CodeError::AsyncPipeFailed)
        }
    } else {
        pub enum AsyncPipe {
            PipeClient(#[pin] NamedPipeClient),
            PipeServer(#[pin] NamedPipeServer),
        }
        impl AsyncRead for AsyncPipe {
            fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) 
                -> Poll<io::Result<()>> {
                match self.project() {
                    AsyncPipeProj::PipeClient(c) => c.poll_read(cx, buf),
                    AsyncPipeProj::PipeServer(c) => c.poll_read(cx, buf),
                }
            }
        }
    }
}
```

**Variations / call-sites:** `cli/src/async_pipe.rs:180-188` (socket naming for temp files), `cli/src/async_pipe.rs:195-232` (trait impl for AcceptedRW).

#### Pattern 4: Tokio Select Loop with Multiple Event Sources
**Where:** `cli/src/json_rpc.rs:46-105`
**What:** RPC dispatcher with `tokio::select!` coordinating reads, writes, and shutdown.

```rust
pub async fn start_json_rpc<C: Send + Sync + 'static, S: Clone>(
    dispatcher: rpc::RpcDispatcher<JsonRpcSerializer, C>,
    read: impl AsyncRead + Unpin,
    mut write: impl AsyncWrite + Unpin,
    mut msg_rx: impl Receivable<Vec<u8>>,
    mut shutdown_rx: Barrier<S>,
) -> io::Result<Option<S>> {
    let (write_tx, mut write_rx) = mpsc::channel::<Vec<u8>>(8);
    let mut read = BufReader::new(read);
    let mut read_buf = String::new();
    let shutdown_fut = shutdown_rx.wait();
    pin!(shutdown_fut);

    loop {
        tokio::select! {
            r = &mut shutdown_fut => return Ok(r.ok()),
            Some(w) = write_rx.recv() => {
                write.write_all(&w).await?;
            },
            Some(w) = msg_rx.recv_msg() => {
                write.write_all(&w).await?;
            },
            n = read.read_line(&mut read_buf) => {
                let r = match n {
                    Ok(0) => return Ok(None),
                    Ok(n) => dispatcher.dispatch(&read_buf.as_bytes()[..n]),
                    Err(e) => return Err(e)
                };
                read_buf.truncate(0);
                match r {
                    MaybeSync::Sync(Some(v)) => {
                        write.write_all(&v).await?;
                    },
                    MaybeSync::Future(fut) => {
                        let write_tx = write_tx.clone();
                        tokio::spawn(async move {
                            if let Some(v) = fut.await {
                                let _ = write_tx.send(v).await;
                            }
                        });
                    },
                    // ...
                }
            }
        }
    }
}
```

**Variations / call-sites:** 
- `cli/src/msgpack_rpc.rs:46-110` (msgpack variant with codec buffering)
- `cli/src/util/sync.rs:157-179` (MergedReceivable with tokio::select!)

#### Pattern 5: Serialization-Agnostic RPC (Trait-Based Dispatch)
**Where:** `cli/src/rpc.rs:40-45`
**What:** Trait for pluggable serialization (JSON/msgpack) with request/response envelope handling.

```rust
pub trait Serialization: Send + Sync + 'static {
    fn serialize(&self, value: impl Serialize) -> Vec<u8>;
    fn deserialize<P: DeserializeOwned>(&self, b: &[u8]) -> Result<P, AnyError>;
}
```

**Variations / call-sites:**
- `cli/src/json_rpc.rs:24-37` (JsonRpcSerializer)
- `cli/src/msgpack_rpc.rs:25-35` (MsgPackSerializer)

#### Pattern 6: Tokio::spawn for Concurrent Task Management
**Where:** `cli/src/json_rpc.rs:84-100` and `cli/src/msgpack_rpc.rs:79-94`
**What:** Spawning async tasks for RPC response handling without blocking the select loop.

```rust
MaybeSync::Future(fut) => {
    let write_tx = write_tx.clone();
    tokio::spawn(async move {
        if let Some(v) = fut.await {
            let _ = write_tx.send(v).await;
        }
    });
},
MaybeSync::Stream((stream, fut)) => {
    if let Some(stream) = stream {
        dispatcher.register_stream(write_tx.clone(), stream).await;
    }
    let write_tx = write_tx.clone();
    tokio::spawn(async move {
        if let Some(v) = fut.await {
            let _ = write_tx.send(v).await;
        }
    });
}
```

**Variations / call-sites:** `cli/src/rpc.rs:453-524` (stream registration with read/write loops), `cli/src/util/sync.rs:191-215` (tests).

#### Pattern 7: Barrier/OneShot Coordination for Process Lifecycle
**Where:** `cli/src/util/sync.rs:11-68`
**What:** Watch-based barrier for coordinating shutdown signals across multiple async tasks.

```rust
#[derive(Clone)]
pub struct Barrier<T>(watch::Receiver<Option<T>>)
where
    T: Clone;

impl<T> Barrier<T>
where
    T: Clone,
{
    pub async fn wait(&mut self) -> Result<T, RecvError> {
        loop {
            self.0.changed().await?;
            if let Some(v) = self.0.borrow().clone() {
                return Ok(v);
            }
        }
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

**Variations / call-sites:** `cli/src/util/sync.rs:37-41` (Receivable trait impl), `cli/src/util/sync.rs:183-219` (tests with tokio::spawn coordination).

#### Pattern 8: Async Command Execution with Capture
**Where:** `cli/src/util/command.rs:13-71`
**What:** Tokio-based process spawning with output capture and error handling.

```rust
pub async fn capture_command_and_check_status(
    command_str: impl AsRef<OsStr>,
    args: &[impl AsRef<OsStr>],
) -> Result<std::process::Output, CodeError> {
    let output = capture_command(&command_str, args).await?;
    check_output_status(output, || {
        format!(
            "{} {}",
            command_str.as_ref().to_string_lossy(),
            args.iter()
                .map(|a| a.as_ref().to_string_lossy())
                .collect::<Vec<Cow<'_, str>>>()
                .join(" ")
        )
    })
}

pub async fn capture_command<A, I, S>(
    command_str: A,
    args: I,
) -> Result<std::process::Output, CodeError>
where
    A: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    new_tokio_command(&command_str)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .await
        .map_err(|e| CodeError::CommandFailed {
            command: command_str.as_ref().to_string_lossy().to_string(),
            code: -1,
            output: e.to_string(),
        })
}
```

**Variations / call-sites:** `cli/src/util/command.rs:74-79` (platform-specific: Windows CREATE_NO_WINDOW flag).

#### Pattern 9: Arc + Mutex State Management (Thread-Safe Persistence)
**Where:** `cli/src/state.rs:89-100`
**What:** Persisted state using Arc<Mutex<>> for concurrent access with serde JSON serialization.

```rust
pub struct PersistedState<T>
where
    T: Clone + Serialize + DeserializeOwned + Default,
{
    container: Arc<Mutex<PersistedStateContainer<T>>>,
}

impl<T> PersistedState<T>
where
    T: Clone + Serialize + DeserializeOwned + Default,
{
    // load_or_get, save, write_state methods...
}
```

**Variations / call-sites:** `cli/src/state.rs:32-86` (PersistedStateContainer internals), `cli/src/singleton.rs:27-36` (SingletonServer using AsyncPipeListener).

#### Pattern 10: Stream Write Loop Pattern (Backpressure Management)
**Where:** `cli/src/rpc.rs:596-631`
**What:** Write loop coordinating async stream output while managing a write queue under mutex.

```rust
async fn write_loop(
    id: u32,
    mut w: WriteHalf<DuplexStream>,
    streams: Arc<std::sync::Mutex<HashMap<u32, StreamRec>>>,
) {
    let mut items_vec = vec![];
    loop {
        {
            let mut lock = streams.lock().unwrap();
            let stream_rec = match lock.get_mut(&id) {
                Some(b) => b,
                None => break,
            };
            if stream_rec.q.is_empty() {
                if stream_rec.ended {
                    lock.remove(&id);
                    break;
                } else {
                    stream_rec.write = Some(w);
                    return;
                }
            }
            std::mem::swap(&mut stream_rec.q, &mut items_vec);
        }
        for item in items_vec.drain(..) {
            if w.write_all(&item).await.is_err() {
                break;
            }
        }
    }
    let _ = w.shutdown().await;
}
```

**Variations / call-sites:** `cli/src/tunnels/server_multiplexer.rs:60-84` (write_message queue management).

---

## Cross-Cutting Patterns

### Async Runtime Conventions
- **Tokio runtime**: All async/await code uses `tokio` tasks and channels
- **Select loops**: Multi-source event coordination via `tokio::select!`
- **Spawn pattern**: Background tasks spawned with `tokio::spawn()`, results sent via mpsc channels

### IPC & Serialization
- **Transport abstraction**: Unified AsyncRead/AsyncWrite over pipes/sockets
- **Message framing**: JSON (line-delimited) and msgpack (codec-based) serialization
- **Duplex streaming**: `tokio::io::duplex()` for bidirectional RPC channels

### Error Handling
- **Typed errors**: `#[derive(Error)]` for exhaustive pattern matching
- **Error aggregation**: Macro-generated `AnyError` enum combining domain-specific types
- **Context preservation**: Error messages include command names, codes, output

### Concurrency Primitives
- **Channels**: `mpsc::unbounded_channel()`, `mpsc::channel()`, `broadcast::Receiver`
- **Watch barriers**: One-shot synchronization via `watch::channel()` for lifecycle coordination
- **Mutex + Arc**: For shared mutable state (queues, maps, files)

---

## Estimated Applicability to Tauri/Rust VS Code Port

1. **Language server communication**: Async RPC patterns directly applicable; use msgpack for efficiency
2. **Debug protocol bridging**: Duplex streaming suitable for DAP forwarding
3. **Terminal/process management**: `tokio::process::Command` proven for subprocess handling
4. **Source control operations**: Async command execution with output capture
5. **Extension IPC**: RPC framework scalable to dozens of concurrent streams
6. **Cross-platform UX**: Async pipe abstraction eliminates platform-specific I/O code

**Key limitation**: No WebSocket or HTTP server patterns evident (CLI focuses on stdio/socket). A Tauri port would need to add frontend messaging middleware.

## External References
<!-- Source: codebase-online-researcher sub-agent -->
# Online Research: Rust Crate Documentation for VS Code → Tauri/Rust Port (`cli/` partition)

**Partition:** 9 of 80 — `cli/` (Rust, 74 source files, ~15,320 LOC)
**Date:** 2026-05-11

---

## Inventory: `cli/Cargo.toml` dependencies

Key production dependencies from `/home/norinlavaee/projects/vscode-atomic/cli/Cargo.toml`:

| Crate | Version pinned | Feature flags |
|---|---|---|
| `tokio` | 1.52 | `full` |
| `tokio-util` | 0.7.8 | `compat`, `codec` |
| `hyper` | 1 | `server`, `http1`, `client` |
| `hyper-util` | 0.1 | `tokio`, `server-auto` |
| `reqwest` | 0.13 | `json`, `stream`, `native-tls` |
| `tokio-tungstenite` | 0.29 | `native-tls` |
| `tunnels` | git (microsoft/dev-tunnels rev `64048c1`) | `connections` |
| `clap` | 4.3 | `derive`, `env` |
| `serde` / `serde_json` / `rmp-serde` | stable | — |
| `futures` | 0.3.28 | — |
| `zbus` (Linux only) | 3.13.1 | `tokio` |
| `russh` family | git (microsoft/vscode-russh) | patch override |

**Tauri is not a current dependency.** The CLI is a headless daemon/RPC server; all GUI currently lives in the Electron layer of VS Code proper.

---

## Library Research

#### Tauri (v2.x — target framework, not currently used)

**Docs:** https://v2.tauri.app/concept/architecture

**Relevant behaviour:**

Tauri 2.x is a polyglot application framework built on Rust. Its stack is:

- **Core process (Rust):** Entry point with full OS access. Manages windows, system-tray menus, IPC routing, and global state. Implemented via the `tauri` crate, which wraps `tauri-runtime-wry` (itself wrapping `wry` + `tao`).
- **WebView process:** Renders HTML/CSS/JS in the OS-native webview. Communication with Core is exclusively via Asynchronous Message Passing — no shared memory, no direct FFI from JS to Rust.
- **IPC primitives (https://v2.tauri.app/concept/inter-process-communication):**
  - **Commands** — invoke-style, JSON-RPC-like; `invoke()` in JS calls `#[tauri::command]` Rust functions. All arguments and return values must be JSON-serializable.
  - **Events** — fire-and-forget, bi-directional. AppHandle/WebviewWindow implement `Emitter` + `Listener`. Designed for low-volume state-change signals, not high-throughput streaming.
  - **Channels** — optimized for streaming data; lower overhead than events for bulk payloads.
- **Sidecar / Embedding External Binaries (https://v2.tauri.app/develop/sidecar):** Tauri can bundle external executables via `tauri.conf.json` → `bundle.externalBin`. Each architecture variant must be named `<binary>-<TARGET_TRIPLE>`. The sidecar is spawned and controlled from Rust using `tauri_plugin_shell`. This is the primary mechanism for embedding a Node.js runtime (for VS Code's extension host) alongside a Tauri app, directly analogous to how Electron bundles Node.
- **Plugin system (https://v2.tauri.app/develop/plugins):** Plugins hook into Tauri lifecycle (`setup`, `on_navigation`, `on_webview_ready`, `on_event`, `on_drop`), expose Rust commands, and optionally ship a corresponding npm package with JS bindings. Official plugins cover: filesystem, shell, notifications, global shortcuts, HTTP client, websocket, updater, single-instance, OS info, store, stronghold, and more.
- **Multi-window:** Each `WebviewWindow` has a label string. Events can be targeted to specific webview labels or broadcast globally. Window configuration via `tauri.conf.json`, the Rust `WebviewWindowBuilder`, or the JavaScript `Window` API.
- **Security/Capabilities (https://v2.tauri.app/security/capabilities):** Fine-grained permission system. Each capability file grants specific command permissions to named windows/webviews. All plugin commands are **blocked by default** until explicitly allowed in capability JSON files under `src-tauri/capabilities/`. This is a hard departure from Electron's model where Node APIs are available to the renderer by default.
- **Process model (https://v2.tauri.app/concept/process-model):** Multi-process, similar to Electron — Core process + one or more WebView processes. Crashes in one don't take down the other. Core process should own all global state.

**Tauri vs Electron — key tradeoffs for a VS Code port:**

Source: https://www.dolthub.com/blog/2025-11-13-electron-vs-tauri/ (Dolthub engineering blog, Nov 2025)

> "The biggest difference between Electron and Tauri comes from how they render the UI. The Electron framework comes with a full Chromium browser engine bundled in your application... Tauri solves this problem by leveraging the system's native webview."

> "In Electron, the main process runs in a Node.js environment. This means you get access to all the typical Node APIs... Tauri, on the other hand, uses Rust."

> "With Electron, if you need the main process to do something, you must always use inter-process communication, even for the simplest of tasks... With Tauri, you can just use Tauri's filesystem API directly in your application code."

The critical limitation for a VS Code port: **WebView inconsistency across platforms.** Tauri uses WebView2 (Chromium-based) on Windows, WebKitGTK on Linux, and WKWebView (WebKit) on macOS — three different rendering engines. VS Code's workbench is heavily optimized for Chromium, using APIs and behaviors that may not be present in WebKit or WebKitGTK. Electron's bundled Chromium eliminates this concern entirely. Specific risks:

- WebKit (macOS/Linux) lags Chrome on modern CSS and JS APIs.
- WebKitGTK on Linux is version-dependent; requires system package `libwebkit2gtk-4.1-dev`.
- An experimental **Tauri + Verso integration** (Verso is a Servo-based browser engine) exists to provide a consistent, bundled WebView, but was labeled experimental as of early 2026.

**Bundle size vs capability:** Tauri apps are 3–10 MB vs Electron's 100–250 MB. For VS Code, this advantage is secondary to functional completeness.

**Where used / relevance to port:** Tauri is the *target* framework — it is absent from `cli/Cargo.toml` today. The `cli/` binary is currently a headless command-line tool (`#[tokio::main] async fn main()` in `cli/src/bin/code/main.rs:29`). A Tauri port would replace the Electron shell: `cli/` logic (tunnel management, agent hosting, JSON-RPC dispatch) would move into the Tauri **Core process**, while VS Code's workbench HTML would load inside the Tauri WebView. The sidecar mechanism would house the Node.js extension host.

---

#### tokio (v1.52)

**Docs:** https://docs.rs/tokio/1.52.0/tokio/

**Relevant behaviour:**

> "Tokio is an event-driven, non-blocking I/O platform for writing asynchronous applications with the Rust programming language. At a high level, it provides a few major components: Tools for working with asynchronous tasks, including synchronization primitives and channels and timeouts, sleeps, and intervals. APIs for performing asynchronous I/O, including TCP and UDP sockets, filesystem operations, and process and signal management."

Primitive inventory used in `cli/`:

- **`tokio::spawn`** — Fire-and-forget async tasks. Used pervasively: `rpc.rs:483`, `rpc.rs:570`, `commands/serve_web.rs:322,378,629,761,776`, `commands/agent_host.rs:109,160,164,222`, `tunnels/server_bridge.rs:29`, etc.
- **`tokio::select!`** — Race multiple futures; used for shutdown-signal-or-data patterns throughout: `util/io.rs:166`, `util/sync.rs:162`, `commands/serve_web.rs:117,164,888`, `commands/tunnels.rs:214,651`, `commands/agent_host.rs:210`, `tunnels/local_forwarding.rs:269,297`, `commands/agent_logs.rs:52`.
- **`tokio::sync::mpsc`** — Multi-producer single-consumer channels. Used in `json_rpc.rs` for write queuing, `desktop/version_manager.rs:110`, `tunnels/socket_signal.rs:7`.
- **`tokio::sync::oneshot`** — Single-shot reply channels: `util/sync.rs:189,203,204`.
- **`tokio::sync::watch`** — Single-writer multi-reader channels: `commands/serve_web.rs:881`.
- **`tokio::net::TcpListener`** — Used in `commands/serve_web.rs` for the local web server.
- **`#[tokio::main]`** — Entry point macro in `cli/src/bin/code/main.rs:29`.

**Where used / relevance to port:** Tokio is the runtime backbone of the entire CLI. If migrated into Tauri, the tokio runtime coexists with Tauri's event loop: Tauri's `setup` hook runs in a sync context, but `tauri::async_runtime::spawn` (which internally uses tokio) is the idiomatic way to spawn async tasks from within a Tauri app. All existing `tokio::spawn` / `tokio::select!` / channel code in `cli/` is directly reusable inside a Tauri Core process — Tauri's async runtime is tokio. No rewrite needed for the async layer itself.

---

#### tao (v0.35) + wry (v0.55) — Tauri's upstream windowing/WebView crates

**Docs:**
- tao: https://docs.rs/tao/0.35.2/tao/
- wry: https://docs.rs/wry/0.55.1/wry/
- wry README: https://raw.githubusercontent.com/tauri-apps/wry/refs/heads/dev/README.md

**Relevant behaviour:**

`tao` is a cross-platform application window creation and event loop management library (a fork of `winit` with added system-tray and menu support). `wry` wraps each platform's native WebView:

| Platform | WebView engine | Dependency |
|---|---|---|
| Linux | WebKitGTK (`webkit2gtk-4.1`) | GTK required; `sudo apt install libwebkit2gtk-4.1-dev` |
| macOS | WKWebView (WebKit, native) | No extra deps; "everything should be fine" |
| Windows | WebView2 (Edge/Chromium, via `webview2-com`) | Supports Win 7, 8, 10, 11 |
| Android | Android WebView via JNI/Kotlin bindings | Complex setup required |
| iOS | WKWebView | Swift/Obj-C integration |

From the wry README:
> "Wry is a cross-platform WebView rendering library. The webview requires a running event loop and a window type that implements HasWindowHandle, or a gtk container widget if you need to support X11 and Wayland."

**Child webviews:** `WebViewBuilder::build_as_child` creates a webview embedded as a child inside another window. Supported on macOS, Windows, and Linux (X11 only — not Wayland). For Wayland, `WebViewBuilderExtUnix::new_gtk` with a `gtk::Fixed` is required.

**Key Linux nuance for VS Code:** Wayland support in wry requires GTK (`gtk::Fixed`), not the generic `HasWindowHandle` path. The VS Code workbench on Linux currently runs under X11 (via Chromium) or native Wayland through Electron. A Tauri port on Wayland Linux would need the GTK code path.

**Where used / relevance to port:** `tao` and `wry` are not in `cli/Cargo.toml` today (the CLI is headless). They would become the window/rendering layer if a Tauri GUI shell wraps the existing CLI logic. From the porting perspective, the most significant impact is the WebKit engine on macOS and Linux: VS Code's workbench relies on Monaco editor, which is Chromium-optimized. Testing on WebKit (Safari-equivalent) would be required and may expose rendering gaps.

---

#### reqwest (v0.13)

**Docs:** https://docs.rs/reqwest/0.13.0/reqwest/

**Relevant behaviour:** `reqwest` is the high-level async HTTP client. Used with `native-tls` for TLS and `json` + `stream` for typed JSON responses and streaming bodies.

In `cli/`:
- `util/http.rs` — `ReqwestSimpleHttp` wraps `reqwest::Client` for the `SimpleHttp` trait, used for update service calls and authentication.
- `auth.rs:116–184` — OAuth token refresh and authentication flows use `reqwest::Client`.
- `commands/context.rs:14` — `CommandContext` carries a shared `reqwest::Client` built with custom user-agent (`cli/src/bin/code/main.rs`).
- `tunnels/code_server.rs:766` — URL parsing via `reqwest::Url`.

**Where used / relevance to port:** `reqwest` is entirely backend (no webview dependency) and carries over unchanged into a Tauri Core process. Tauri also ships its own `tauri-plugin-http` for making HTTP requests *from the frontend JS layer*, but the Rust-side `reqwest` usage in CLI is unaffected. Version 0.13 uses `hyper` v1 under the hood (matching the direct `hyper` v1 dependency in `Cargo.toml`).

---

#### hyper (v1) + hyper-util (v0.1)

**Docs:** https://docs.rs/hyper/1.0.0/hyper/

**Relevant behaviour:** `hyper` v1 is a low-level HTTP library. The CLI uses it for both server and client roles:

- **`commands/serve_web.rs`** — Runs a local HTTP/1.1 server (via `hyper::server::conn::http1` + `TokioIo`) that serves the VS Code web workbench. Handles HTTP upgrade for WebSocket proxy: `tokio::join!(hyper::upgrade::on(&mut req), hyper::upgrade::on(&mut res))` (`serve_web.rs:380`).
- **`commands/agent_host.rs`** — Accept loop using `hyper_util::server::conn::auto::Builder` (`ServerBuilder`) for HTTP/1+HTTP/2 auto-negotiation over TCP. `TokioExecutor` + `TokioIo` bridge tokio's `AsyncRead`/`AsyncWrite` to hyper's traits.
- **`tunnels/agent_host.rs`** — Client-side `hyper::client::conn::http1::handshake` for proxying requests through the dev tunnel.

**Where used / relevance to port:** The embedded HTTP server in `serve_web.rs` is how the current CLI serves VS Code's web workbench to a browser. In a Tauri port this server *may be replaceable* by Tauri's built-in `tauri://localhost` custom protocol (which serves static assets directly into the webview without a TCP port). However, the WebSocket proxy (`hyper::upgrade`) for the language server / extension host connection is more nuanced and would likely remain as a sidecar TCP service.

---

#### tunnels (microsoft/dev-tunnels, git rev `64048c1`)

**Docs:** https://github.com/microsoft/dev-tunnels (README)

**Relevant behaviour:** Microsoft's dev-tunnels Rust SDK provides:
- `tunnels::connections::{RelayTunnelHost, RelayTunnelClient, PortConnection, ForwardedPortConnection, ClientRelayHandle}` — Relay-based tunnel host and client.
- `tunnels::management::{TunnelManagementClient, new_tunnel_management}` — REST management API.
- `tunnels::contracts` — Tunnel data types (`Tunnel`, `TunnelPort`, `TunnelEndpoint`, etc.).

From the dev-tunnels SDK feature matrix (README): The Rust SDK supports Management API, Tunnel Client Connections, and Tunnel Host Connections, but **lacks** Reconnection, SSH-level Reconnection, Automatic token refresh, and SSH Keep-alive — features present in the C# and TypeScript SDKs. This means `cli/` has custom reconnect logic layered on top.

In `cli/`:
- `auth.rs:23` — `tunnels::{...}` imports for token integration.
- `commands/tunnels.rs` — `DevTunnels` struct orchestrates tunnel lifecycle.
- `tunnels/dev_tunnels.rs` — `RelayTunnelHost` and `RelayTunnelClient` are the primary connection types; `tokio::sync::{mpsc, watch}` channels coordinate shutdown.
- `commands/agent.rs:95` — `tunnels::connections::PortConnectionRW` wraps the WebSocket stream for the agent protocol.

**Where used / relevance to port:** The dev-tunnels integration is independent of the GUI framework. This subsystem would transfer unchanged to a Tauri Core process. The Rust SDK's reconnection gap (no automatic SSH-level reconnect) is an existing limitation independent of the Tauri porting question — tracked in `cli/src/commands/tunnels.rs` with custom retry logic.

---

## Port Architecture Summary

The current `cli/` binary is a **headless Rust daemon** — it does not use any GUI toolkit. It:
1. Parses CLI args via `clap` (`main.rs`).
2. Dispatches to commands: `serve_web`, `tunnels`, `agent_host`, `agent_logs`, etc.
3. Runs an embedded `hyper` HTTP server (`serve_web.rs`) that serves VS Code's web workbench to a browser.
4. Communicates via a JSON-RPC layer (`json_rpc.rs`, `rpc.rs`) over tokio mpsc/oneshot channels and async streams.
5. Manages dev-tunnel connections via the `tunnels` crate.

**What a Tauri port would mean for `cli/`:**

A Tauri port of VS Code would not simply port the `cli/` binary — it would add a Tauri GUI shell *around* the existing CLI infrastructure:

- The tokio async runtime, hyper server, reqwest client, dev-tunnels, and JSON-RPC machinery in `cli/` are all **directly reusable** in a Tauri Core process. Tauri's async runtime is tokio; `tauri::async_runtime::spawn` delegates to the same tokio threadpool.
- The **Electron shell** (the `src/` TypeScript + Chromium workbench) would be replaced by a Tauri `WebviewWindow` loading the workbench HTML from either `tauri://localhost` (static bundle) or a local hyper server.
- The **Node.js extension host** (`extensionHostProcess`) would become a **Tauri sidecar**: a `node` binary bundled via `tauri.conf.json → bundle.externalBin`, spawned and communicated with via the `tauri-plugin-shell` crate's `Command` API (which wraps `std::process::Command` and stdio pipes).
- The **IPC model shifts**: where Electron renders used `ipcRenderer.send/invoke`, Tauri renderers would use `invoke()` (JSON-only) or Tauri channels for streaming. VS Code's internal `IPC` channel protocol (over `MessagePort`) has no direct analog in Tauri — this is the largest rewrite surface.
- The **capability permission system** requires auditing every OS API call the workbench makes from JS and explicitly granting it in capability JSON files. VS Code makes extensive use of filesystem, shell, process, and clipboard APIs from the renderer, all of which are blocked by default in Tauri.
- **WebKit rendering gaps** on macOS and Linux are the most significant technical risk. Monaco editor and the workbench shell assume Chromium-level CSS/JS compatibility. An experimental Tauri+Verso integration (using a Servo-based bundled engine) could resolve this but is not production-ready.

The `cli/` partition's Rust async patterns — `tokio::spawn`, `tokio::select!`, mpsc/oneshot/watch channels, hyper server/client, reqwest — are production-grade and directly portable into Tauri's Core process with minimal modification. The porting challenge lies almost entirely in the Electron-to-Tauri GUI surface, not in the `cli/` Rust layer.

Sources:
- https://v2.tauri.app/concept/architecture
- https://v2.tauri.app/concept/process-model
- https://v2.tauri.app/concept/inter-process-communication
- https://v2.tauri.app/develop/sidecar
- https://v2.tauri.app/develop/plugins
- https://v2.tauri.app/develop/calling-frontend
- https://v2.tauri.app/security/capabilities
- https://v2.tauri.app/learn/window-customization
- https://docs.rs/tokio/1.52.0/tokio/
- https://docs.rs/wry/0.55.1/wry/
- https://docs.rs/tao/0.35.2/tao/
- https://raw.githubusercontent.com/tauri-apps/wry/refs/heads/dev/README.md
- https://github.com/microsoft/dev-tunnels
- https://www.dolthub.com/blog/2025-11-13-electron-vs-tauri/
- https://www.gethopp.app/blog/tauri-vs-electron

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
