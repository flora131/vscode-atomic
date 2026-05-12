# Remote Server Entrypoint Patterns
## Partition 50/80: `src/server-main.ts` Analysis

### Pattern: HTTP Server Bootstrap with Request Delegation

**Where:** `src/server-main.ts:88-94`

**What:** Core HTTP server creation that delegates actual request handling to a lazily-loaded remote extension host agent server. Tracks first request for performance metrics.

```typescript
let firstRequest = true;
const server = http.createServer(async (req, res) => {
	if (firstRequest) {
		firstRequest = false;
		perf.mark('code/server/firstRequest');
	}
	const remoteExtensionHostAgentServer = await getRemoteExtensionHostAgentServer();
	return remoteExtensionHostAgentServer.handleRequest(req, res);
});
```

**Variations:**
- Server creation is synchronous, but request handling is async (awaits lazy initialization)
- Delegation pattern requires `IServerAPI` interface with `handleRequest(req, res)` method
- First-request tracking used for telemetry (`perf.mark()`)

---

### Pattern: WebSocket Upgrade Handler

**Where:** `src/server-main.ts:96-104`

**What:** HTTP upgrade handler for WebSocket connections routed through same server instance. Mirrors first-request tracking pattern.

```typescript
let firstWebSocket = true;
server.on('upgrade', async (req, socket) => {
	if (firstWebSocket) {
		firstWebSocket = false;
		perf.mark('code/server/firstWebSocket');
	}
	const remoteExtensionHostAgentServer = await getRemoteExtensionHostAgentServer();
	// @ts-expect-error
	return remoteExtensionHostAgentServer.handleUpgrade(req, socket);
});
```

**Variations:**
- Uses `server.on('upgrade')` listener (standard Node.js HTTP upgrade pattern)
- Raw socket object passed to handler (not wrapped WebSocket)
- Delegates to `handleUpgrade(req, socket)` on server API
- TypeScript error suppression needed for type mismatch

---

### Pattern: Server Error Handler Chain

**Where:** `src/server-main.ts:105-108`

**What:** Centralized error handler that delegates to server implementation.

```typescript
server.on('error', async (err) => {
	const remoteExtensionHostAgentServer = await getRemoteExtensionHostAgentServer();
	return remoteExtensionHostAgentServer.handleServerError(err);
});
```

**Variations:**
- Wraps all server errors (binding failures, stream errors, etc.)
- Also uses lazy-loading pattern for initialization

---

### Pattern: Socket vs. Port Configuration

**Where:** `src/server-main.ts:110-115`

**What:** Dual-mode listen configuration supporting both Unix domain sockets and TCP network sockets.

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
- Unix socket takes priority if `--socket-path` provided
- TCP falls back to `localhost` (or undefined for older compatibility)
- Port resolution is async (may scan for free port in range)
- `server.listen()` callback fires on successful binding

---

### Pattern: Port Range Parsing and Free Port Discovery

**Where:** `src/server-main.ts:170-189` (parsePort) and `src/server-main.ts:207-225` (findFreePort)

**What:** Multi-step port resolution: single port, range scan, or default 8000. Includes free port detection via temporary server creation.

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
			// Remote-SSH extension relies on this exact port error message, treat as an API
			console.warn(`--port: Could not find free port in range: ${range.start} - ${range.end} (inclusive).`);
			process.exit(1);
		} else {
			console.warn(`--port "${strPort}" is not a valid number or range.`);
			process.exit(1);
		}
	}
	return 8000;
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
- Regex validation: `/^\d+$/` for single port, `/^(\d+)-(\d+)$/` for range
- Range bounds: `start > 0 && start <= end && end <= 65535`
- Free port test creates/closes ephemeral server for each port
- Error message locked for API compatibility (Remote-SSH extension dependency)

---

### Pattern: Lazy Server Initialization with Promise Caching

**Where:** `src/server-main.ts:52-63`

**What:** Double-checked pattern for lazy initialization of heavy server component, caching promise to avoid multiple initializations.

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
- Two-stage caching: promise and resolved value
- Depends on `address` being set after server binding
- Loads code dynamically via `loadCode()` before creating server
- Multiple listeners (request, upgrade, error) all trigger lazy init

---

### Pattern: Process Lifecycle and Cleanup

**Where:** `src/server-main.ts:146-151`

**What:** Exit handler for graceful shutdown of server and agent.

```typescript
process.on('exit', () => {
	server.close();
	if (_remoteExtensionHostAgentServer) {
		_remoteExtensionHostAgentServer.dispose();
	}
});
```

**Variations:**
- Single exit handler
- Conditional disposal (only if server was initialized)
- No signal handlers (SIGTERM/SIGINT) — relies on default Node.js behavior
- Calls `server.close()` to stop accepting connections
- Calls `.dispose()` on agent if available (custom cleanup interface)

---

## Summary

The remote server entrypoint follows a **delegation architecture** where `server-main.ts` bootstraps Node.js HTTP infrastructure and delegates protocol handling to a lazily-loaded `IServerAPI` implementation. Key patterns a Rust port must reproduce:

1. **HTTP + WebSocket on single port** — Node's `http.createServer()` with `.on('upgrade')` handler
2. **Dual listen mode** — Unix domain socket vs. TCP (host/port)
3. **Dynamic port resolution** — Range parsing with free port discovery via test binding
4. **Lazy initialization** — Server code loaded on first client connection, promise cached
5. **Instrumentation** — First-request/first-websocket markers for perf tracking
6. **Graceful shutdown** — Exit handler with conditional cleanup
7. **Environment + CLI args** — Minimist parsing of `--host`, `--port`, `--socket-path`, with fallback to `VSCODE_SERVER_*` env vars

The wire protocol (HTTP/WebSocket) stays protocol-agnostic; only the transport and initialization bootstrap need Rust equivalents. The `IServerAPI` interface definition would define what the Rust server must implement.
