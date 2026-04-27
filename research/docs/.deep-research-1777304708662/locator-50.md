# Server Entry Point: Partition 50 - src/server-main.ts

## Implementation

### Primary Entry Point
- `src/server-main.ts` (~285 LOC) — Main server entry for remote/web deployments. Handles CLI argument parsing, HTTP server creation, WebSocket upgrade events, and lazy-loads the actual server implementation.

### Core Server Components Loaded On Demand
- `src/vs/server/node/server.main.ts` — Exports `spawnCli()` for CLI operations and `createServer(address)` factory function that returns an `IServerAPI` instance. Initializes server data directories, environment setup, and delegates to the remote extension host agent server.
- `src/vs/server/node/remoteExtensionHostAgentServer.ts` — Implements `RemoteExtensionHostAgentServer` class (extends `Disposable`, implements `IServerAPI`). Manages extension host connections, management connections, socket lifecycle, web client serving, connection token validation, and UNC path restrictions.
- `src/vs/server/node/webClientServer.ts` — Serves static web UI files, handles MIME types, ETags, cache control headers, and connection token cookie/query param handling for browser clients.

### Server Services & Infrastructure
- `src/vs/server/node/serverServices.ts` (~80+ imported services) — Service collection setup: `IConfigurationService`, `IFileService`, `ILogService`, `IRequestService`, `IPtyService` (terminal), `IExtensionManagementService`, `ILanguagePackService`, telemetry channels, user profile services, extension scanner services, logger channels.
- `src/vs/server/node/serverEnvironmentService.ts` — Defines `serverOptions` configuration schema and `ServerEnvironmentService` (extends `NativeEnvironmentService`). Handles server-specific options: host, port, socket-path, server-base-path, connection-token, connection-token-file, without-connection-token, websocket compression, telemetry level, server-data-dir.
- `src/vs/server/node/serverConnectionToken.ts` — Connection token validation and parsing: `ServerConnectionToken`, `ServerConnectionTokenType`, `determineServerConnectionToken()`, `requestHasValidConnectionToken()`.
- `src/vs/server/node/extensionHostConnection.ts` — Manages individual extension host connection lifecycle.
- `src/vs/server/node/remoteExtensionManagement.ts` — `ManagementConnection` class for extension management operations.
- `src/vs/server/node/remoteExtensionHostAgentCli.ts` — CLI entry for extension operations (list, install, uninstall, update).
- `src/vs/server/node/extensionHostStatusService.ts` — Tracks extension host status.
- `src/vs/server/node/extensionsScannerService.ts` — Scans available extensions.
- `src/vs/server/node/remoteExtensionsScanner.ts` — Remote extension scanner channel and service.
- `src/vs/server/node/remoteFileSystemProviderServer.ts` — File system provider for remote clients.
- `src/vs/server/node/remoteTerminalChannel.ts` — Terminal/PTY channel for remote terminal operations.
- `src/vs/server/node/remoteAgentEnvironmentImpl.ts` — Remote agent environment channel.
- `src/vs/server/node/serverLifetimeService.ts` — Server lifetime management.
- `src/vs/server/node/remoteLanguagePacks.ts` — Language pack support.
- `src/vs/server/node/serverAgentHostManager.ts` — Agent host management.
- `src/vs/server/node/remoteExtensionHostAgentCli.ts` — Extension management CLI operations.

### Bootstrap & Environment Setup
- `src/bootstrap-server.ts` — Prevents Electron runtime from interfering with Node.js FS access.
- `src/bootstrap-node.ts` — Node module path manipulation (dev mode injection, removal of global paths).
- `src/bootstrap-esm.ts` — ESM module bootstrap.
- `src/bootstrap-meta.ts` — Product metadata.
- `src/bootstrap-import.ts` — Import utilities.

## Types / Interfaces

### IServerAPI Contract
```
export interface IServerAPI {
  handleRequest(req: http.IncomingMessage, res: http.ServerResponse): Promise<void>;
  handleUpgrade(req: http.IncomingMessage, socket: net.Socket): void;
  handleServerError(err: Error): void;
  dispose(): void;
}
```

The interface is explicitly marked "Do not remove!!. Called from server-main.js" — this is the stability contract between the HTTP handler in server-main.ts and the loaded server implementation.

### Key Type Dependencies
- `http.IncomingMessage`, `http.ServerResponse` from Node.js `http` module (req/res handling).
- `net.Socket`, `net.AddressInfo` from Node.js `net` module (socket/address management).
- `INLSConfiguration` — NLS message configuration.
- `ServerParsedArgs` — Parsed command-line arguments specific to server mode.
- `RemoteAgentConnectionContext` — Context for remote connections.
- `ServerConnectionToken` — Connection authentication token.

## Configuration

### Command-Line Arguments (via minimist parsing)
Lines 26-38 of server-main.ts define supported flags:
- Boolean: `start-server`, `list-extensions`, `print-ip-address`, `help`, `version`, `accept-server-license-terms`, `update-extensions`
- String: `install-extension`, `install-builtin-extension`, `uninstall-extension`, `locate-extension`, `socket-path`, `host`, `port`, `compatibility`, `agent-host-port`, `agent-host-path`

Environment variable overrides for host, port, accept-server-license-terms (prefixed `VSCODE_SERVER_`).

### Server Options Schema
From `serverEnvironmentService.ts`:
- Setup: host, port, socket-path, server-base-path, connection-token, connection-token-file, without-connection-token, disable-websocket-compression, print-startup-performance, print-ip-address, accept-server-license-terms, server-data-dir, telemetry-level
- VS Code options: user-data-dir, enable-smoke-test-driver, disable-telemetry, disable-experiments, disable-workspace-trust, file-watcher-polling, log, force-disable-user-env, enable-proposed-api
- Web options: default-folder, default-workspace, enable-sync, github-auth, use-test-resolver

### Server License & Acceptance
Lines 65-82: Displays product.serverLicense array to console, optionally prompts for `--accept-server-license-terms` flag or stdin input before proceeding.

### Server Greeting
Lines 117-118: Outputs product.serverGreeting array if defined.

## Notable Clusters

### HTTP Server Lifecycle
- **Creation** (lines 88-104): `http.createServer()` with async req/res handler and async WebSocket upgrade handler. Lazy-loads `RemoteExtensionHostAgentServer` on first request.
- **Startup** (lines 116-144): Calls `server.listen()`, detects bound address, outputs server greeting and listening port/socket info.
- **Cleanup** (lines 146-151): `process.on('exit')` closes server and disposes extension host agent server.

### Port Resolution
Lines 170-225: Multi-step port discovery:
- `parsePort()` — Accepts single port number, port range (e.g., "8000-9000"), or defaults to 8000.
- `parseRange()` — Validates range format and port bounds.
- `findFreePort()` — Iterates through range using `http.createServer()` test to find first available port. Remote-SSH extension depends on specific error message at line 181.

### CLI vs Server Mode Branching
Lines 43-51: Quick parse determines whether to spawn CLI (`spawnCli()` from server.main.js) or start HTTP server. CLI triggered by: help, version, or any extension lookup/install operation without `--start-server` flag.

### Lazy Server Initialization
Lines 54-63: `getRemoteExtensionHostAgentServer()` memoizes promise chain:
- Loads code via `loadCode(nlsConfiguration)` once on first request/WebSocket.
- Creates server instance via `createServer(address)`.
- Caches result in `_remoteExtensionHostAgentServer`.

### Performance Markers
- Line 22: `perf.mark('code/server/start')` — Process start.
- Line 91: `perf.mark('code/server/firstRequest')` — First HTTP request.
- Line 99: `perf.mark('code/server/firstWebSocket')` — First WebSocket upgrade.
- Line 140: `perf.mark('code/server/started')` — Server listening.
- server.main.ts: `perf.mark('code/server/codeLoaded')` — Code module loaded.

### Bootstrap Sequence (loadCode function, lines 227-254)
1. Sets `VSCODE_NLS_CONFIG` from resolved NLS configuration.
2. Sets `VSCODE_HANDLES_SIGPIPE` to avoid console broken-pipe loops.
3. In dev mode: injects node_modules path from `remote/node_modules`.
4. Removes global Node.js module lookup paths.
5. Calls `bootstrapESM()` to set up ES modules.
6. Dynamically imports `./vs/server/node/server.main.js`.

### String Argument Sanitization
Lines 154-159: `sanitizeStringArg()` handles minimist arrays (multiple same flags) by taking the last value.

### TTY & License Prompt
Lines 257-285: 
- `hasStdinWithoutTty()` detects non-interactive stdin.
- `prompt()` uses readline.createInterface for user interaction; validates y/yes/n/no responses recursively.

## Hidden Dependencies

The server-main.ts entry point depends critically on:
1. **minimist** — CLI argument parsing
2. **Node.js http module** — HTTP server creation and socket handling
3. **Node.js net module** — Socket and address info types
4. **Node.js path, os, readline, perf_hooks modules** — Utilities
5. **Remote extension host agent server** (lazy-loaded) — Core server logic

The interface contract `IServerAPI` is the **only stable public API** — any Rust replacement must implement these 4 methods with identical signatures and semantics.

---

## Summary

The `src/server-main.ts` file (~285 lines) is the thin entry point for VS Code's remote/web server deployment. It orchestrates:

1. **CLI vs Server mode dispatch** — Determines if user wants extension management operations (delegated to CLI) or HTTP server startup.
2. **HTTP server bootstrapping** — Creates Node.js http.createServer, attaches upgrade handler for WebSocket, binds to host/port/socket (with free port discovery), and outputs listening information.
3. **Lazy initialization** — Defers loading of the heavy `RemoteExtensionHostAgentServer` until first HTTP request, caching the result.
4. **License handling** — Optionally displays server license and prompts for acceptance before startup.
5. **Performance instrumentation** — Marks startup, first request, and listening milestones.

A Rust/Tauri replacement would need to:
- Parse the same command-line arguments and environment variable overrides.
- Create a native HTTP/WebSocket server (hyper, actix-web, or similar).
- Implement the `IServerAPI` interface contract with `handleRequest()`, `handleUpgrade()`, `handleServerError()`, and `dispose()` methods.
- Replicate the port resolution logic (single port, range, socket path, free port discovery).
- Handle license prompts and server greeting output.
- Set up the same service collection and bootstrap sequence (or Rust equivalents).
- Maintain the performance markers and error telemetry hooks.

The actual IDE functionality (editing, debugging, language servers, terminal, source control, etc.) would be delegated to the loaded server implementation. The complexity lies not in server-main.ts itself, but in replacing the downstream `RemoteExtensionHostAgentServer` and its 15+ dependent service modules.
