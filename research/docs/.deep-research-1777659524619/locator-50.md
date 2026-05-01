# Locator Report: Server Main Entry Point (src/server-main.ts)

## Scope
Single file analysis: `src/server-main.ts` (285 LOC)

## Implementation

**Entry Point & Server Initialization**
- `src/server-main.ts` lines 1-152: Headless server launcher that bootstraps the VS Code remote extension host agent

**Core Server Creation Pattern**
- Line 57: `const server = await mod.createServer(address);` — Dynamic import-based server instantiation from `./vs/server/node/server.main.js`
- Line 88: `const server = http.createServer(async (req, res) => {...})` — Node.js HTTP server wrapping the remote agent server
- Lines 96-107: HTTP upgrade handler and error handler delegation to the underlying server API

**Port Management Functions**
- Lines 170-190: `parsePort()` — Parses port arguments as single port or range (e.g., `8000-8010`)
- Lines 192-201: `parseRange()` — Validates port range format and bounds (1-65535)
- Lines 207-225: `findFreePort()` — Iterative port availability testing using temporary HTTP servers

**Server Configuration & Lifecycle**
- Lines 26-38: Argument parsing with minimist for `--host`, `--port`, `--socket-path`, `--start-server`, and extension management flags
- Lines 110-115: Server listening options configuration (TCP or Unix socket)
- Lines 116-144: Server startup with address binding, greeting output, and performance marks
- Lines 146-151: Process exit handler with graceful shutdown and resource disposal

**License & CLI Routing**
- Lines 47-50: CLI spawn path for help, version, or extension operations
- Lines 65-82: Server license prompt and acceptance logic
- Lines 84-85: Performance tracking for first request and WebSocket upgrade

**Bootstrap & Module Loading**
- Line 6: Bootstrap server state setup (must be first import)
- Lines 227-255: `loadCode()` function configuring NLS, SIGPIPE handling, dev mode node module paths, and ESM bootstrap before dynamic server module import
- Lines 238-248: Development vs. production module resolution paths

**Utility Functions**
- Lines 154-159: `sanitizeStringArg()` — Handles multi-value arguments from minimist
- Lines 257-264: `hasStdinWithoutTty()` — TTY detection for interactive prompts
- Lines 266-285: `prompt()` — Interactive readline-based yes/no confirmation

## Types / Interfaces

**Type Imports**
- Line 20: `IServerAPI` from `./vs/server/node/remoteExtensionHostAgentServer.js` — Server interface contract defining:
  - `handleRequest(req: IncomingMessage, res: ServerResponse)` — HTTP request handler
  - `handleUpgrade(req: IncomingMessage, socket: Socket)` — WebSocket upgrade handler
  - `handleServerError(err: Error)` — Error handler
  - `dispose()` — Cleanup method

**Inferred Interfaces**
- `AddressInfo` from `node:net` — Server address information after binding
- Module export from `./vs/server/node/server.main.js` with:
  - `createServer(address: string | AddressInfo | null): Promise<IServerAPI>` — Factory function
  - `spawnCli()` — CLI execution method

## Documentation

**Inline Comments**
- Line 136: "Do not change this line. VS Code looks for this in the output." — Parser contract for extension host agent listener output
- Lines 180-181: Remote-SSH extension dependency note on port error message format
- Lines 162-169: Port argument behavior documentation
- Lines 203-206: Free port search algorithm documentation
- Lines 232-236: SIGPIPE handling explanation with GitHub issue reference
- Lines 238-241: Development mode module resolution comment

## Prose Summary

`src/server-main.ts` serves as the headless server launcher for VS Code's remote extension host agent. It implements a three-stage initialization: (1) argument parsing and routing to either CLI or server mode, (2) lazy server instantiation through dynamic module loading with NLS and ESM bootstrap, and (3) HTTP server wrapping with delegation to the underlying `IServerAPI` implementation. The core contract for Rust/Tauri porting is the `IServerAPI` interface, which requires implementations of `handleRequest()`, `handleUpgrade()`, `handleServerError()`, and `dispose()`. Port management includes single port, range parsing, and free port discovery. The file enforces critical contracts: HTTP server listening address output format (line 136-137) is parsed by dependent tools, and license prompts use interactive readline. Development paths support source-based execution with injected node module lookup paths. All performance metrics are marked globally for startup profiling.
