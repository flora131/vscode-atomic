# Pattern Analysis: Headless Server Entrypoint (src/server-main.ts)

## Overview
This document catalogs patterns found in `src/server-main.ts` (285 LOC), the headless server entrypoint for VS Code. This file orchestrates server initialization, CLI spawning, licensing, network binding, and lazy server instantiation—analogous to code-server's bootstrap flow but integrated with VS Code's ecosystem.

---

## Patterns

#### Pattern 1: Bootstrap-first initialization with global state mutation
**Where:** src/server-main.ts:1-23
**What:** Synchronous bootstrap module imported first to alter global state; followed by performance marking to track startup timeline. This ensures all downstream code operates in correct environment.

```typescript
import './bootstrap-server.js'; // this MUST come before other imports as it changes global state
import * as path from 'node:path';
import * as http from 'node:http';
// ... other imports ...
import { performance } from 'node:perf_hooks';

perf.mark('code/server/start');
(globalThis as { vscodeServerStartTime?: number }).vscodeServerStartTime = performance.now();
```

**Variations:** 
- src/main.ts:6-30 — Electron desktop variant with portable configuration and module setup
- src/vs/platform/agentHost/node/agentHostServerMain.ts:1-20 — Standalone agent host variant with explicit file root setup

---

#### Pattern 2: Argument parsing with environment variable fallback
**Where:** src/server-main.ts:25-38
**What:** Minimist parses command-line arguments with type coercion (boolean/string), then environment variables override defaults for critical config keys (host, port, license acceptance). Follows pattern: CLI args → env vars → defaults.

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
```

**Variations:**
- src/vs/platform/agentHost/node/agentHostServerMain.ts:74-80 — Manual parsing of argv with hardcoded port default (8081) and direct env var access

---

#### Pattern 3: Dual entrypoint routing (CLI vs Server)
**Where:** src/server-main.ts:40-51
**What:** Determines whether to spawn CLI tool or start server by checking aggregated conditions: help/version flags, extension lookup/install args, and --start-server flag. Lazy loads code module and calls appropriate method (spawnCli vs createServer).

```typescript
const extensionLookupArgs = ['list-extensions', 'locate-extension'];
const extensionInstallArgs = ['install-extension', 'install-builtin-extension', 'uninstall-extension', 'update-extensions'];

const shouldSpawnCli = parsedArgs.help || parsedArgs.version || extensionLookupArgs.some(a => !!parsedArgs[a]) || (extensionInstallArgs.some(a => !!parsedArgs[a]) && !parsedArgs['start-server']);

if (shouldSpawnCli) {
	loadCode(nlsConfiguration).then((mod) => {
		mod.spawnCli();
	});
} else {
	// ... server initialization ...
}
```

**Variations:**
- src/vs/server/node/server.main.ts:62-71 — Actual module exports; two public functions (spawnCli, createServer) delegating to internal implementations

---

#### Pattern 4: Lazy server instantiation with memoization
**Where:** src/server-main.ts:52-63
**What:** Defers expensive server creation until first request. Maintains both a promise (for concurrent request deduplication) and cached instance reference. Async/await pattern ensures single instantiation even with concurrent calls.

```typescript
let _remoteExtensionHostAgentServer: IServerAPI | null = null;
let _remoteExtensionHostAgentServerPromise: Promise<IServerAPI> | null = null;
const getRemoteExtensionHostAgentServer = () => {
	if (!_remoteExtensionHostAgentServerPromise) {
		_remoteExtensionHostAgentServerPromise = loadCode(nlsConfiguration).then(async (mod) => {
			const server = await mod.createServer(address);
			_remoteExtensionHostAgentServer = server;
			return server;
		});
	}
	return _remoteExtensionHostAgentServerPromise;
};
```

**Variations:**
- Request-level caching common in singleton service patterns; this variant is unique in combining promise memoization with direct instance caching

---

#### Pattern 5: License prompt with TTY detection
**Where:** src/server-main.ts:65-82
**What:** Displays license text from product config. If interactive prompt required and no TTY (e.g., CI/systemd), exit. Otherwise, use readline-based prompt to accept/reject. Recursive retry on invalid response.

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
```

**Variations:**
- src/vs/platform/agentHost/node/agentHostServerMain.ts — No license prompt; simpler startup path for agent server

---

#### Pattern 6: HTTP server with delegated request/upgrade/error handling
**Where:** src/server-main.ts:84-108
**What:** Creates bare http.Server and attaches handlers for HTTP requests, WebSocket upgrades, and errors. All handlers await lazy server instance and delegate. Tracks first request/WebSocket with markers for perf telemetry.

```typescript
let firstRequest = true;
let firstWebSocket = true;

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
	return remoteExtensionHostAgentServer.handleUpgrade(req, socket);
});
server.on('error', async (err) => {
	const remoteExtensionHostAgentServer = await getRemoteExtensionHostAgentServer();
	return remoteExtensionHostAgentServer.handleServerError(err);
});
```

**Variations:**
- src/vs/platform/agentHost/node/agentHostServerMain.ts — WebSocketProtocolServer instead of bare http.Server; different transport
- src/vs/workbench/api/node/loopbackServer.ts — Similar pattern but without lazy initialization

---

#### Pattern 7: Port binding with socket path or dynamic port selection
**Where:** src/server-main.ts:110-115
**What:** Constructs listen options: if socket-path provided, use Unix socket; otherwise use TCP with host and parsed port. Port can be single number or range; range triggers free port search.

```typescript
const host = sanitizeStringArg(parsedArgs['host']) || (parsedArgs['compatibility'] !== '1.63' ? 'localhost' : undefined);
const nodeListenOptions = (
	parsedArgs['socket-path']
		? { path: sanitizeStringArg(parsedArgs['socket-path']) }
		: { host, port: await parsePort(host, sanitizeStringArg(parsedArgs['port'])) }
);
server.listen(nodeListenOptions, async () => {
	// ...
});
```

**Variations:**
- src/vs/code/node/cli.ts:386-388 — Uses findFreePort utility function directly with fixed try counts; server-main has inline implementation

---

#### Pattern 8: Network interface enumeration for IP printing
**Where:** src/server-main.ts:119-128
**What:** On --print-ip-address flag, iterates os.networkInterfaces() and lists non-internal IPv4 addresses. Conditional output appended to startup message.

```typescript
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
```

**Variations:**
- src/vs/platform/agentHost/node/agentHostServerMain.ts — Does not expose IP address; only prints port

---

#### Pattern 9: Graceful cleanup on exit
**Where:** src/server-main.ts:146-151
**What:** Registers process exit handler that closes http.Server and disposes remote extension host server if instantiated. Ensures resource cleanup even on forced shutdown.

```typescript
process.on('exit', () => {
	server.close();
	if (_remoteExtensionHostAgentServer) {
		_remoteExtensionHostAgentServer.dispose();
	}
});
```

**Variations:**
- src/vs/platform/agentHost/node/agentHostServerMain.ts:415-425 — Similar pattern with DisposableStore for aggregated cleanup

---

#### Pattern 10: Port range parsing and free port search
**Where:** src/server-main.ts:170-225
**What:** parsePort validates string port (single digit or range "from-to"). For ranges, invokes findFreePort to locate available TCP port. findFreePort creates test servers and attempts connection; returns undefined if exhausted.

```typescript
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
			console.warn(`--port "${strPort}" is not a valid number or range. Ranges must be in the form 'from-to'...`);
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

**Variations:**
- src/vs/base/node/ports.ts:12-30 — Shared utility; uses socket connection instead of test server; supports timeout and stride (skip interval)
- src/vs/platform/tunnel/node/tunnelService.ts:91 — Uses findFreePortFaster variant with hostname parameter

---

#### Pattern 11: Environment variable setup for NLS, SIGPIPE, and node modules
**Where:** src/server-main.ts:227-255
**What:** loadCode() function configures process.env before module dynamic import. Sets VSCODE_NLS_CONFIG for i18n, VSCODE_HANDLES_SIGPIPE to prevent async logging loops, and node module lookup paths for dev builds.

```typescript
async function loadCode(nlsConfiguration: INLSConfiguration) {
	// required for `bootstrap-esm` to pick up NLS messages
	process.env['VSCODE_NLS_CONFIG'] = JSON.stringify(nlsConfiguration);

	// See https://github.com/microsoft/vscode-remote-release/issues/6543
	// We would normally install a SIGPIPE listener in bootstrap-node.js
	// But in certain situations, the console itself can be in a broken pipe state
	// so logging SIGPIPE to the console will cause an infinite async loop
	process.env['VSCODE_HANDLES_SIGPIPE'] = 'true';

	if (process.env['VSCODE_DEV']) {
		process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH'] = process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH'] || path.join(import.meta.dirname, '..', 'remote', 'node_modules');
		devInjectNodeModuleLookupPath(process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH']);
	} else {
		delete process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH'];
	}

	removeGlobalNodeJsModuleLookupPaths();
	await bootstrapESM();
	return import('./vs/server/node/server.main.js');
}
```

**Variations:**
- src/main.ts — Sets crash reporter, portable paths, and UNC allowlist instead of server-specific env vars

---

#### Pattern 12: TTY detection and interactive prompt
**Where:** src/server-main.ts:257-285
**What:** hasStdinWithoutTty() checks process.stdin.isTTY with try/catch for Windows. prompt() creates readline interface, asks question, validates y/n/yes/no responses (empty string = yes), and recursively retries on invalid input.

```typescript
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

**Variations:**
- No other TTY detection found in codebase for server context; pattern is server-main specific

---

## Summary

The `src/server-main.ts` file implements a sophisticated headless server bootstrap that:

1. **Enforces initialization order** via synchronous bootstrap module import
2. **Supports flexible configuration** through CLI args, environment variables, and defaults
3. **Routes to CLI or server** based on argument combinations
4. **Defers expensive initialization** via lazy server instantiation with promise memoization
5. **Validates licensing** interactively on non-TTY-capable environments
6. **Provides HTTP/WebSocket transport** with delegated handlers and performance tracking
7. **Supports flexible binding** (Unix socket or dynamic TCP port selection)
8. **Implements robust port allocation** with range support and free port detection
9. **Configures process environment** for NLS, SIGPIPE handling, and module resolution
10. **Handles user interaction** through readline-based prompts with retry logic
11. **Ensures cleanup** on process exit

This pattern is distinct from Electron desktop (`src/main.ts`) and the standalone agent host server, making it a template-worthy reference for Tauri-based server initialization.
