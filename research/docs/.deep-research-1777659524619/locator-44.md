# Partition 44: VS Code Tunnel Forwarding Extension & Port Forwarding Porting Guide

## Overview
The `extensions/tunnel-forwarding/` directory (4 source files, 474 LOC) implements VS Code's port-forwarding extension as a TypeScript/Electron plugin. It interfaces with the Rust CLI `cli/src/tunnels/` module (25 files, 7579 LOC) which provides the underlying tunnel and port-forwarding infrastructure. Together, they manage local port forwarding to internet-accessible URLs via VS Code's tunnel service.

---

## Implementation Files

### TypeScript Extension (4 files)

**Primary Entry Point:**
- `extensions/tunnel-forwarding/src/extension.ts` (344 LOC)
  - `TunnelProvider` class implements `vscode.TunnelProvider` API
  - Spawns CLI child process via `child_process.spawn(cliPath, args)`
  - State machine manages: Inactive → Starting → Active → Error
  - Handles tunnel lifecycle (create, update, dispose) via event emitters
  - Receives JSON port format config from CLI stdout: `{ port_format: string }`
  - Manages cleanup timeout (10s) to avoid CLI restart thrashing
  - Prompts GitHub OAuth via `vscode.authentication.getSession('github', ...)`
  - Sends port list updates to CLI stdin as JSON: `[{number, privacy, protocol}]`

**Utility Modules:**
- `extensions/tunnel-forwarding/src/deferredPromise.ts` (62 LOC)
  - Promise wrapper with explicit resolve/reject callbacks
  - Copied from `src/vs/base/common/async.ts`
  - Provides `DeferredPromise<T>` with state tracking (isResolved/isRejected/isSettled)

- `extensions/tunnel-forwarding/src/split.ts` (52 LOC)
  - Stream utility for splitting newline-delimited data
  - `StreamSplitter extends Transform` for processing CLI stdout/stderr
  - Copied and simplified from `src/vs/base/node/nodeStreams.ts`

### Build Configuration
- `extensions/tunnel-forwarding/esbuild.mts` (19 LOC)
  - esbuild config for TypeScript compilation
  - Entry point: `src/extension.ts` → output: `dist/extension`
  - Platform: Node.js (not browser)

---

## Configuration Files

**Extension Manifest:**
- `extensions/tunnel-forwarding/package.json`
  - Display name: "Local Tunnel Port Forwarding"
  - Version: 10.0.0
  - Publisher: vscode
  - VS Code engine: ^1.82.0
  - API proposals enabled:
    - `resolvers` (for custom resolver API)
    - `tunnelFactory` (for tunnel provider factory)
  - Activation event: `onTunnel`
  - Commands:
    - `tunnel-forwarding.showLog` - Show CLI output channel
    - `tunnel-forwarding.restart` - Force CLI restart
  - Main entry: `./out/extension`

**Localization:**
- `extensions/tunnel-forwarding/package.nls.json`
  - displayName: "Local Tunnel Port Forwarding"
  - description: "Allows forwarding local ports to be accessible over the internet."
  - Category: "Port Forwarding"

**TypeScript Compilation:**
- `extensions/tunnel-forwarding/tsconfig.json`
  - Extends `../tsconfig.base.json`
  - Root dir: `./src`, output: `./out`
  - Type roots: Node.js types + three proposed VS Code APIs:
    - `vscode.d.ts` (base API)
    - `vscode.proposed.resolvers.d.ts`
    - `vscode.proposed.tunnelFactory.d.ts`

**Runtime Configuration:**
- `extensions/tunnel-forwarding/.npmrc`
  - Legacy peer dependency support
  - 180s npm operation timeout
  - Minimum release age: 1 day

**Package Exclusions:**
- `extensions/tunnel-forwarding/.vscodeignore`
  - Excludes source files, tsconfig, build outputs, esbuild config, and lock files from packaged extension

**Debug Configuration:**
- `extensions/tunnel-forwarding/.vscode/launch.json`
  - Extension host launch config
  - Dev flag: `VSCODE_FORWARDING_IS_DEV=1` loads CLI from OSS build
  - Load CLI from `cli/target/debug/code` during development

---

## Rust CLI Tunnel Infrastructure (Integration Boundary)

### Core Port Forwarding (6 files, ~1200 LOC)
- `cli/src/tunnels/port_forwarder.rs`
  - `PortForwardingProcessor` with async message handler pattern
  - Enum: `PortForwardingRec::Forward(port, privacy, oneshot_sender)` / `::Unforward(...)`
  - Port blocking: prevents forwarding control ports (AGENT_HOST_PORT, CONTROL_PORT)
  - Manages HashSet of active forwarded ports
  - Integration point: Called from within `ActiveTunnel` context

- `cli/src/tunnels/local_forwarding.rs` (~250 LOC)
  - `PortCount` struct tracks public/private port counts
  - Singleton server for local port forwarding requests
  - Handles JSON RPC protocol: parses port lists from stdin, responds via protocol

### Protocol & Data Contracts (2 files)
- `cli/src/tunnels/protocol.rs` (~400 LOC)
  - Enum `ClientRequestMethod`: servermsg, serverclose, serverlog, makehttpreq, version
  - Structs: `ForwardParams` (port, public), `UnforwardParams`, `ForwardResult` (uri)
  - Structs: `PortProtocol` (Auto/Http/Https), `PortPrivacy` (Public/Private)
  - HTTP request/response protocol serialization

- `cli/src/tunnels/dev_tunnels.rs` (~2000+ LOC, largest module)
  - `ActiveTunnel` struct: represents live tunnel connection
  - `add_port_tcp(port, privacy, protocol)` - initiates port forwarding
  - `remove_port(port)` - closes forwarding
  - Methods: `add_port_tcp()`, `remove_port()`, manage tunnel lifecycle
  - Trait: `AccessTokenProvider` - token refresh pattern
  - OAuth integration for GitHub authentication
  - Tunnel state persistence: `PersistedTunnel` serialization

### Service & Connectivity (12 files, ~4000+ LOC)
- `cli/src/tunnels/agent_host.rs` - Agent connection management
- `cli/src/tunnels/code_server.rs` - VS Code server integration
- `cli/src/tunnels/control_server.rs` - Control plane communication
- `cli/src/tunnels/server_bridge.rs` - Bridge between local and tunnel server
- `cli/src/tunnels/server_multiplexer.rs` - Multiplex connections
- `cli/src/tunnels/socket_signal.rs` - Signal handling
- `cli/src/tunnels/singleton_server.rs` / `singleton_client.rs` - IPC pattern
- `cli/src/tunnels/challenge.rs` - Authentication challenge protocol
- `cli/src/tunnels/shutdown_signal.rs` - Graceful shutdown signaling

### Platform-Specific & Utilities (9 files)
- `cli/src/tunnels/service.rs` - Core service abstraction
- `cli/src/tunnels/service_windows.rs` - Windows service integration
- `cli/src/tunnels/service_macos.rs` - macOS service integration
- `cli/src/tunnels/service_linux.rs` - Linux service integration
- `cli/src/tunnels/nosleep.rs` - Prevent system sleep
- `cli/src/tunnels/nosleep_windows.rs` / `nosleep_macos.rs` / `nosleep_linux.rs` - Platform sleep prevention
- `cli/src/tunnels/wsl_detect.rs` - WSL environment detection
- `cli/src/tunnels/paths.rs` - Path resolution
- `cli/src/tunnels/legal.rs` - License/legal text

---

## Test Files
None present in TypeScript extension (no test files found with .test.ts/.spec.ts patterns).

---

## Notable Clusters & Architecture

### Data Flow (TypeScript → Rust CLI)
1. **Extension Activation** (`activationEvents: ["onTunnel"]`)
   - Registers with VS Code via `vscode.workspace.registerTunnelProvider(provider)`
   - Subscribes to tunnel requests via `provideTunnel(tunnelOptions)`

2. **Process Spawning**
   - CLI path resolution: dev (`cli/target/debug/code`) or prod (bundled `code-tunnel`/`code-tunnel-insiders`)
   - Spawn args: `['--verbose', 'tunnel', 'forward-internal', '--provider', 'github']`
   - Environment: GitHub OAuth token passed via `VSCODE_CLI_ACCESS_TOKEN`

3. **Protocol (JSON Lines over stdin/stdout)**
   - **Ext→CLI (stdin)**: Port configuration array each update
     ```
     [{"number":3000,"privacy":"private","protocol":"http"}]
     ```
   - **CLI→Ext (stderr)**: JSON log lines parsed for `port_format`
     ```
     {"port_format":"http://localhost:{port}"}
     ```
   - **CLI→Ext (stdout)**: Verbose logging (piped via splitNewLines)

4. **State Transitions**
   - Starting: Child process spawned, progress indicator shown
   - Active: Received `port_format` from CLI, tunnels initialized
   - Error: Process exit or error event, state holds error message
   - Inactive: All tunnels disposed and cleanup timeout expires

### Extension Integration Points
- **API Proposals**: Uses unstable `resolvers` and `tunnelFactory` APIs (may change)
- **VS Code Context**: Sets `tunnelForwardingIsRunning` and `tunnelForwardingHasLog` context keys
- **GitHub Auth**: Automatic session creation via OAuth; error if unavailable
- **Output Channel**: Async log output channel created on first log message

### CLI Integration Contract
- **Input**: Port list updates (JSON) on stdin
- **Output**: Port format string and logs on stderr/stdout
- **Lifecycle**: Stays running while ports exist; cleans up after 10s idle timeout
- **Error Handling**: Process exit/error triggers state transition; displayed to user

---

## Porting Considerations for Tauri/Rust

### What Would Need to Change

**Elimination of Intermediate Layer:**
- Current: VS Code Extension (TypeScript) ↔ Child Process (Rust CLI) ↔ Tunnel Service
- Porting: Embed tunnel logic directly in Tauri app (single-process, no IPC)
- Benefit: No JSON serialization overhead, direct Rust-to-Rust API calls

**API Surface Shifts:**
- TypeScript `vscode.TunnelProvider` API → Custom Tauri command/event system
- Port format strings, privacy enums → Native Rust structures
- GitHub OAuth integration → Tauri plugin or native HTTP client

**Process Management:**
- Current: Extension spawns CLI subprocess, manages streams, handles signals
- Porting: Call Rust tunnel functions directly; no child_process module needed
- Signal handling: Use Tauri lifecycle events instead of OS signals

**Type System Benefits:**
- Current: JSON parsing/validation on each update (overhead)
- Porting: Type-safe function calls with serde serialization only at IPC boundary

### Leverageable Rust Modules
All 25 Rust CLI tunnel files are directly reusable:
1. `port_forwarder.rs` - Core forwarding logic (copy directly)
2. `dev_tunnels.rs` - Tunnel lifecycle (drop CLI parsing, keep tunnel ops)
3. `protocol.rs` - Message types and serialization contracts
4. `local_forwarding.rs` - Port list management
5. Platform services (`service_*.rs`, `nosleep_*.rs`) - Keep as-is for background service

### Remaining Challenges
1. **GitHub OAuth**: Currently handled via CLI child process; would need Tauri oauth plugin
2. **Service Mode**: Windows/macOS/Linux service registration; Tauri has platform-specific APIs
3. **Logging**: Replace VS Code Output Channel with Tauri logging system
4. **Auto-Update**: CLI currently auto-updates; Tauri handles differently (built-in updater)

---

## Compilation & Build

- TypeScript: `gulp compile-extension:tunnel-forwarding` (main), `gulp watch-extension:tunnel-forwarding` (dev watch)
- Output: Compiled to `out/` directory, esbuild config in `esbuild.mts`
- DevDependencies: Only `@types/node@22.x` (minimal)
- Prettier config: 100-char line width, trailing commas, single quotes

---

## Summary

The tunnel-forwarding extension is a **thin TypeScript wrapper** (~470 LOC) around a mature Rust CLI (~7500 LOC) that handles port forwarding via VS Code's tunnels infrastructure. The extension:
- Manages process lifecycle and bidirectional IPC
- Handles OAuth authentication (delegated to CLI)
- Presents port privacy/protocol options in VS Code UI
- Buffers port updates and sends to CLI via JSON serialization

For a Tauri/Rust port, the extension layer could be **eliminated entirely** by embedding the Rust tunnel modules directly in the app. The 25 Rust modules are well-separated and mostly independent of the CLI harness. Key integration points are the `TunnelProvider` API (replaced by Tauri commands) and OAuth (needs Tauri plugin). The port forwarding, protocol, and service logic require minimal adaptation.

