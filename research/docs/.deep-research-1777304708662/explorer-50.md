# Partition 50 of 79 — Findings

## Scope
`src/server-main.ts/` (1 files, 285 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Analysis: VS Code Server Entry Point (src/server-main.ts)
## Research: Porting IDE Core from TypeScript/Electron to Tauri/Rust

**Scope:** Single file analysis of server entry point (~285 LOC)
**Focus:** HTTP server bootstrap, argument parsing, initialization, lifecycle management
**Key Finding:** The server reveals 6 critical architectural patterns that any Rust/Tauri replacement must reproduce to maintain API compatibility with client expectations.

---

## Patterns Identified

#### Pattern 1: Global Initialization Chain with Side-Effect Ordering
**Where:** `src/server-main.ts:6-22`
**What:** Explicit bootstrap sequence ensuring global state is configured before module loading proceeds.

```typescript
import './bootstrap-server.js'; // this MUST come before other imports as it changes global state
import * as path from 'node:path';
import * as http from 'node:http';
import type { AddressInfo } from 'node:net';
import * as os from 'node:os';
import * as readline from 'node:readline';
import { performance } from 'node:perf_hooks';
import minimist from 'minimist';
import { devInjectNodeModuleLookupPath, removeGlobalNodeJsModuleLookupPaths } from './bootstrap-node.js';
import { bootstrapESM } from './bootstrap-esm.js';
import { resolveNLSConfiguration } from './vs/base/node/nls.js';
import { product } from './bootstrap-meta.js';
import * as perf from './vs/base/common/performance.js';

perf.mark('code/server/start');
(globalThis as { vscodeServerStartTime?: number }).vscodeServerStartTime = performance.now();
```

**Variations / call-sites:**

- `bootstrap-server.ts` deletes `ELECTRON_RUN_AS_NODE` environment variable to prevent fs redirection
- `bootstrap-node.ts` (lines 14-100) sets stack trace limits (100 frames), installs SIGPIPE handlers, configures working directory
- `bootstrap-esm.ts` (lines 14-29) registers module resolution hooks to map 'fs' to 'node:original-fs' in Electron contexts
- `bootstrap-meta.ts` loads product.json and package.json, applies overrides from dev/product.sub.json
- Performance marks are collected to track startup phases: `code/server/start`, `code/server/firstRequest`, `code/server/firstWebSocket`, `code/server/started`

**Critical detail:** The comment "this MUST come before other imports" indicates ordering is enforced at runtime. In a Rust port, this translates to initializing global state handlers (memory management, environment variables, signal handlers) before the core server instantiation.

---

#### Pattern 2: Conditional CLI vs Server Mode Branching
**Where:** `src/server-main.ts:25-51`
**What:** Argument parsing determines whether process spawns CLI or HTTP server, with lazy server initialization.

```typescript
const parsedArgs = minimist(process.argv.slice(2), {
	boolean: ['start-server', 'list-extensions', 'print-ip-address', 'help', 'version', 'accept-server-license-terms', 'update-extensions'],
	string: ['install-extension', 'install-builtin-extension', 'uninstall-extension', 'locate-extension', 'socket-path', 'host', 'port', 'compatibility', 'agent-host-port', 'agent-host-path'],
	alias: { help: 'h', version: 'v' }
});
['host', 'port', 'accept-server-license-terms'].forEach(e => {
	if (!parsedArgs[e]) {
		const envValue = process.env[`VSCODE_SERVER_${e.toUpperCase().replace('-', '_')}`];
		if (envValue) {
			parsedArgs[e] = envValue;
		}
	}
});

const extensionLookupArgs = ['list-extensions', 'locate-extension'];
const extensionInstallArgs = ['install-extension', 'install-builtin-extension', 'uninstall-extension', 'update-extensions'];
const shouldSpawnCli = parsedArgs.help || parsedArgs.version || extensionLookupArgs.some(a => !!parsedArgs[a]) || (extensionInstallArgs.some(a => !!parsedArgs[a]) && !parsedArgs['start-server']);

if (shouldSpawnCli) {
	loadCode(nlsConfiguration).then((mod) => {
		mod.spawnCli();
	});
} else {
	// ... HTTP server initialization
}
```

**Variations / call-sites:**

- CLI mode triggers: `--help`, `--version`, extension management flags without `--start-server`
- Args loaded from both CLI and environment: `VSCODE_SERVER_HOST`, `VSCODE_SERVER_PORT`, `VSCODE_SERVER_ACCEPT_SERVER_LICENSE_TERMS`
- Environment variable key transformation: dashes converted to underscores, prefixed with `VSCODE_SERVER_`
- Lazy initialization: `loadCode()` async promise pattern defers module loading until needed

---

#### Pattern 3: Server Event Handlers for HTTP and WebSocket Upgrade
**Where:** `src/server-main.ts:88-108`
**What:** Three-handler pattern for request, upgrade (WebSocket), and error events, all delegating to `IServerAPI` interface methods.

```typescript
let address: string | AddressInfo | null = null;
const server = http.createServer(async (req, res) => {
	if (firstRequest) {
		firstRequest = false;
		perf.mark('code/server/firstRequest');
	}
	const remoteExtensionHostAgentServer = await getRemoteExtensionHostAgentServer();
	return remoteExtensionHostAgentServer.handleRequest(req, res);
});
server.on('upgrade', async (req, socket) => {
	if (firstWebSocket) {
		firstWebSocket = false;
		perf.mark('code/server/firstWebSocket');
	}
	const remoteExtensionHostAgentServer = await getRemoteExtensionHostAgentServer();
	// @ts-expect-error
	return remoteExtensionHostAgentServer.handleUpgrade(req, socket);
});
server.on('error', async (err) => {
	const remoteExtensionHostAgentServer = await getRemoteExtensionHostAgentServer();
	return remoteExtensionHostAgentServer.handleServerError(err);
});
```

**Variations / call-sites:**

- `IServerAPI` interface contract (lines 558-575 in remoteExtensionHostAgentServer.ts):
  - `handleRequest(req: http.IncomingMessage, res: http.ServerResponse): Promise<void>`
  - `handleUpgrade(req: http.IncomingMessage, socket: net.Socket): void`
  - `handleServerError(err: Error): void`
  - `dispose(): void`
- All three handlers await lazy initialization via `getRemoteExtensionHostAgentServer()` promise
- First-occurrence perf marks for tracking request latency
- `@ts-expect-error` on handleUpgrade indicates type system mismatch but functional necessity

---

#### Pattern 4: Port Discovery and Binding with Range Support
**Where:** `src/server-main.ts:110-225`
**What:** Multi-step port resolution supporting single port, port ranges, socket paths, and free port detection.

```typescript
const host = sanitizeStringArg(parsedArgs['host']) || (parsedArgs['compatibility'] !== '1.63' ? 'localhost' : undefined);
const nodeListenOptions = (
	parsedArgs['socket-path']
		? { path: sanitizeStringArg(parsedArgs['socket-path']) }
		: { host, port: await parsePort(host, sanitizeStringArg(parsedArgs['port'])) }
);

async function parsePort(host: string | undefined, strPort: string | undefined): Promise<number> {
	if (strPort) {
		let range: { start: number; end: number } | undefined;
		if (strPort.match(/^\d+$/)) {
			return parseInt(strPort, 10);
		} else if (range = parseRange(strPort)) {
			const port = await findFreePort(host, range.start, range.end);
			if (port !== undefined) {
				return port;
			}
			console.warn(`--port: Could not find free port in range: ${range.start} - ${range.end} (inclusive).`);
			process.exit(1);
		} else {
			console.warn(`--port "${strPort}" is not a valid number or range. Ranges must be in the form 'from-to' with 'from' an integer larger than 0 and not larger than 'end'.`);
			process.exit(1);
		}
	}
	return 8000;
}

function parseRange(strRange: string): { start: number; end: number } | undefined {
	const match = strRange.match(/^(\d+)-(\d+)$/);
	if (match) {
		const start = parseInt(match[1], 10), end = parseInt(match[2], 10);
		if (start > 0 && start <= end && end <= 65535) {
			return { start, end };
		}
	}
	return undefined;
}

async function findFreePort(host: string | undefined, start: number, end: number): Promise<number | undefined> {
	const testPort = (port: number) => {
		return new Promise((resolve) => {
			const server = http.createServer();
			server.listen(port, host, () => {
				server.close();
				resolve(true);
			}).on('error', () => {
				resolve(false);
			});
		});
	};
	for (let port = start; port <= end; port++) {
		if (await testPort(port)) {
			return port;
		}
	}
	return undefined;
}
```

**Variations / call-sites:**

- Three port configuration modes:
  - Single port: `--port 3000` → direct integer parsing
  - Port range: `--port 3000-3100` → regex validation, range bounds check (1-65535)
  - Socket path: `--socket-path /tmp/vscode.sock` → Unix domain socket
- Default port: 8000 (if neither port nor socket-path specified)
- Compatibility flag check: `--compatibility 1.63` affects host binding behavior (undefined vs 'localhost')
- Free port detection: test-bind each port sequentially, server.close() after successful bind

---

#### Pattern 5: Server Startup Signaling with Network Metadata
**Where:** `src/server-main.ts:116-144`
**What:** Callback on server.listen() outputs startup status with address binding confirmation and IP discovery.

```typescript
server.listen(nodeListenOptions, async () => {
	let output = Array.isArray(product.serverGreeting) && product.serverGreeting.length ? `\n\n${product.serverGreeting.join('\n')}\n\n` : ``;

	if (typeof nodeListenOptions.port === 'number' && parsedArgs['print-ip-address']) {
		const ifaces = os.networkInterfaces();
		Object.keys(ifaces).forEach(function (ifname) {
			ifaces[ifname]?.forEach(function (iface) {
				if (!iface.internal && iface.family === 'IPv4') {
					output += `IP Address: ${iface.address}\n`;
				}
			});
		});
	}

	address = server.address();
	if (address === null) {
		throw new Error('Unexpected server address');
	}

	output += `Server bound to ${typeof address === 'string' ? address : `${address.address}:${address.port} (${address.family})`}\n`;
	// Do not change this line. VS Code looks for this in the output.
	output += `Extension host agent listening on ${typeof address === 'string' ? address : address.port}\n`;
	console.log(output);

	perf.mark('code/server/started');
	(globalThis as { vscodeServerListenTime?: number }).vscodeServerListenTime = performance.now();

	await getRemoteExtensionHostAgentServer();
});
```

**Variations / call-sites:**

- Product metadata: `product.serverGreeting` array (if present) printed before binding info
- Network discovery: `os.networkInterfaces()` filtered for non-internal IPv4 (only if `--print-ip-address`)
- Address handling: supports both socket path (string) and TCP address (AddressInfo object with host/port/family)
- **Critical output line:** "Extension host agent listening on" — VS Code client specifically looks for this string in stdout to confirm server startup
- Server address capture: `server.address()` returns null-check after binding
- Performance timing: captures `vscodeServerListenTime` on globalThis

---

#### Pattern 6: Lifecycle Cleanup and Graceful Shutdown
**Where:** `src/server-main.ts:146-151`
**What:** Process exit handler ensuring server closure and resource disposal.

```typescript
process.on('exit', () => {
	server.close();
	if (_remoteExtensionHostAgentServer) {
		_remoteExtensionHostAgentServer.dispose();
	}
});
```

**Variations / call-sites:**

- Conditional disposal: only calls dispose() if server was initialized (lazy-initialized by first request)
- Two-step cleanup: http.Server.close() stops accepting connections, then IServerAPI.dispose() releases application-level resources
- No `unref()` or explicit signal handling (SIGTERM/SIGINT) in entry point — delegated to Node.js default behavior
- `server.close()` callback not used; cleanup occurs synchronously on exit event

---

## Additional Patterns: License Terms and TTY Handling

#### Pattern 7 (Minor): Interactive License Prompt with TTY Detection
**Where:** `src/server-main.ts:65-82`
**What:** Conditional license acceptance flow, skipping prompt if `--accept-server-license-terms` or stdin is not TTY.

```typescript
if (Array.isArray(product.serverLicense) && product.serverLicense.length) {
	console.log(product.serverLicense.join('\n'));
	if (product.serverLicensePrompt && parsedArgs['accept-server-license-terms'] !== true) {
		if (hasStdinWithoutTty()) {
			console.log('To accept the license terms, start the server with --accept-server-license-terms');
			process.exit(1);
		}
		try {
			const accept = await prompt(product.serverLicensePrompt);
			if (!accept) {
				process.exit(1);
			}
		} catch (e) {
			console.log(e);
			process.exit(1);
		}
	}
}

function hasStdinWithoutTty(): boolean {
	try {
		return !process.stdin.isTTY;
	} catch (error) {
		// Windows workaround for https://github.com/nodejs/node/issues/11656
	}
	return false;
}

function prompt(question: string): Promise<boolean> {
	const rl = readline.createInterface({
		input: process.stdin,
		output: process.stdout
	});
	return new Promise((resolve, reject) => {
		rl.question(question + ' ', async function (data) {
			rl.close();
			const str = data.toString().trim().toLowerCase();
			if (str === '' || str === 'y' || str === 'yes') {
				resolve(true);
			} else if (str === 'n' || str === 'no') {
				resolve(false);
			} else {
				process.stdout.write('\nInvalid Response. Answer either yes (y, yes) or no (n, no)\n');
				resolve(await prompt(question));
			}
		});
	});
}
```

**Variations / call-sites:**

- TTY safety: Windows try-catch handles https://github.com/nodejs/node/issues/11656 where process.stdin.isTTY can throw
- Recursive prompt: invalid answers trigger re-prompt rather than hard exit
- Readline interface: uses `readline.createInterface` for interactive input/output

---

## Architectural Implications for Rust/Tauri Port

**Must-Implement Server Contracts:**

1. **IServerAPI interface** with exact method signatures:
   - `handleRequest(req: HttpRequest, res: HttpResponse) -> Future<()>`
   - `handleUpgrade(req: HttpRequest, socket: Socket) -> ()`
   - `handleServerError(err: Error) -> ()`
   - `dispose() -> ()`

2. **Exact stdout output format** for client discovery:
   - Line containing "Extension host agent listening on {port}"

3. **Argument parsing** supporting:
   - Boolean flags (case-sensitive): start-server, list-extensions, help, etc.
   - String args with environment variable overrides (VSCODE_SERVER_* pattern)
   - Port ranges with free port detection
   - Socket path as alternative to TCP binding

4. **Performance timing globals** for startup metrics:
   - `vscodeServerStartTime`
   - `vscodeServerListenTime`

5. **Bootstrap-like initialization ordering** to configure:
   - Signal handlers (SIGPIPE)
   - Module resolution hooks (if applicable)
   - Working directory setup
   - Global state before server instantiation

6. **Lazy server initialization** pattern where:
   - Request handlers trigger server creation
   - Server promise cached for subsequent requests
   - Connection token validation before serving content

**Critical Constraint:** The startup sequence, argument handling, and output format are tightly coupled with VS Code client expectations. The "Extension host agent listening on" line is explicitly checked by the client, making that string output non-negotiable for client-server discovery.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
