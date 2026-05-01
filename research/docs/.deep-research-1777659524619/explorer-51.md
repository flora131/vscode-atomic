# Partition 51 of 79 — Findings

## Scope
`src/bootstrap-fork.ts/` (1 files, 229 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator 51: bootstrap-fork.ts Analysis

## Research Focus
Port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust, focusing on multi-process architecture and fork lifecycle management.

## Scope Coverage
- `src/bootstrap-fork.ts` — 229 LOC, single file analyzed

## Implementation

- `src/bootstrap-fork.ts` — Fork process bootstrap for extension host child process; handles logging pipes, exception handlers, parent process monitoring, crash reporter configuration, ESM bootstrapping, and inter-process communication via `process.send()` and environment variables; central to VS Code's multi-process architecture that a Rust/Tauri port must replicate.

## Key Architectural Components Identified

### Process Lifecycle Management
- Parent-child process communication via `process.send()` (line 80)
- Parent process monitoring with `process.kill(parentPid, 0)` polling (line 175)
- Environment variable-based process type configuration (line 184)
- Graceful termination when parent process dies (lines 169-181)

### Logging & I/O Redirection
- Console method interception (log, info, warn, error) (lines 105-110)
- Stream wrapping for stdout/stderr (lines 118-137)
- Message serialization with circular reference detection (lines 53-75)
- Environment-controlled verbose logging (line 140)

### Error & Exception Handling
- Uncaught exception handler via `process.on('uncaughtException')` (line 159)
- Unhandled promise rejection handler via `process.on('unhandledRejection')` (line 164)
- Conditional error handling based on `VSCODE_HANDLES_UNCAUGHT_ERRORS` (line 216)

### Electron Integration
- Crash reporter configuration for Electron-only features (line 188)
- Detection of Electron's `process.crashReporter` API

### Module Loading
- Node.js global module path manipulation (line 204)
- Development-time module lookup injection (lines 206-208)
- ESM module bootstrapping and entry point loading (lines 226-229)

## Critical IPC Patterns for Tauri/Rust Port

The file demonstrates several patterns essential to replicate in a Rust/Tauri port:

1. **Message Passing Protocol**: Simple JSON-based messages with `{ type, severity, arguments }` structure for console routing (line 96)
2. **Environment Variable Configuration**: Extensive use of environment variables for feature flags and process metadata (VSCODE_VERBOSE_LOGGING, VSCODE_PARENT_PID, VSCODE_PIPE_LOGGING, VSCODE_ESM_ENTRYPOINT, etc.)
3. **Process Monitoring**: Polling-based parent process health check with 5-second intervals (line 173)
4. **Stream Buffering**: Custom buffering logic with 1MB max stream buffer and line-based flushing (lines 15, 128-131)

---

**Summary**: This single-file scope documents the fork bootstrap process, which is a foundational piece of VS Code's multi-process architecture. A Tauri/Rust port would need to replicate the IPC protocol, process lifecycle management, logging redirection, exception handling, and environment-based configuration mechanisms shown here. The file demonstrates both Node.js-specific APIs (process module) and Electron-specific integrations (crash reporter) that would require platform-specific reimplementation in Rust.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `src/bootstrap-fork.ts` (229 LOC)

---

### Per-File Notes

#### `src/bootstrap-fork.ts`

- **Role:**  
  This is the universal entry point for every forked child process in VS Code's multi-process architecture. It runs before any feature-specific code and sets up the environment, I/O routing, exception handling, parent-process lifecycle coupling, crash reporting, and ESM module loading. It acts as a harness that all spawned Node.js workers pass through before reaching their actual workload.

- **Key symbols:**

  | Symbol | Location | Purpose |
  |---|---|---|
  | `pipeLoggingToParent()` | line 14 | Redirects console/stream output to the parent process via IPC |
  | `safeToString(args)` | line 21 | Circular-reference-safe JSON serializer for log arguments |
  | `safeSend(arg)` | line 77 | Wraps `process.send()` with try/catch for closed-channel safety |
  | `safeSendConsoleMessage(severity, args)` | line 95 | Formats and sends a `__$console` typed IPC message to parent |
  | `wrapConsoleMethod(method, severity)` | line 105 | Replaces `console.*` via `Object.defineProperty` to intercept log calls |
  | `wrapStream(streamName, severity)` | line 118 | Intercepts `process.stdout/stderr.write` to buffer and forward output |
  | `handleExceptions()` | line 156 | Registers `uncaughtException` and `unhandledRejection` handlers |
  | `terminateWhenParentTerminates()` | line 169 | Polls parent PID every 5 seconds; calls `process.exit()` if parent is gone |
  | `configureCrashReporter()` | line 183 | Tags the Electron crash reporter with the process type string |

- **Control flow:**

  Execution is entirely top-level sequential (no class, no default export). After imports, the file runs a fixed initialization sequence:

  1. **Performance mark** — `performance.mark('code/fork/start')` at line 10 records process start for telemetry.
  2. **Crash reporter** — `configureCrashReporter()` called unconditionally at line 201. Reads `process.env['VSCODE_CRASH_REPORTER_PROCESS_TYPE']` (line 184) and invokes `process['crashReporter'].addExtraParameter('processType', ...)` if the Electron-only `crashReporter` object is present (line 188-190).
  3. **Module path cleanup** — `removeGlobalNodeJsModuleLookupPaths()` called at line 204 (imported from `./bootstrap-node.js`). Conditionally, `devInjectNodeModuleLookupPath(...)` is called if `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH` is set (lines 206-208).
  4. **Logging pipe** — `pipeLoggingToParent()` is called at line 212 only when both `process.send` is defined (confirms IPC channel exists) AND `VSCODE_PIPE_LOGGING === 'true'`. Inside `pipeLoggingToParent()`:
     - If `VSCODE_VERBOSE_LOGGING === 'true'`, all four console methods are wrapped (lines 141-144); otherwise `log`, `warn`, `info` are silenced and only `error` is forwarded (lines 146-149).
     - `wrapStream('stderr', 'error')` and `wrapStream('stdout', 'log')` are called unconditionally within the pipe block (lines 152-153).
  5. **Exception handling** — `handleExceptions()` called at line 217 only if `VSCODE_HANDLES_UNCAUGHT_ERRORS` is NOT set. Registers `process.on('uncaughtException', ...)` (line 159) and `process.on('unhandledRejection', ...)` (line 164), both routing to `console.error`.
  6. **Parent watchdog** — `terminateWhenParentTerminates()` called at line 222 if `VSCODE_PARENT_PID` is set. Reads the PID with `Number(process.env['VSCODE_PARENT_PID'])` (line 170) and runs `setInterval` at 5000 ms (line 173). Inside the interval, `process.kill(parentPid, 0)` (line 175) probes liveness; on exception, calls `process.exit()` (line 177).
  7. **ESM bootstrap** — `await bootstrapESM()` at line 226 (imported from `./bootstrap-esm.js`).
  8. **Dynamic entry load** — `await import(...)` at line 229 loads the actual worker module. The path is constructed as `./${process.env['VSCODE_ESM_ENTRYPOINT']}.js`, making the entrypoint fully runtime-determined via environment variable.

- **Data flow:**

  - **Inbound (parent → child):** Configuration arrives entirely through environment variables before execution begins: `VSCODE_VERBOSE_LOGGING`, `VSCODE_PIPE_LOGGING`, `VSCODE_HANDLES_UNCAUGHT_ERRORS`, `VSCODE_PARENT_PID`, `VSCODE_CRASH_REPORTER_PROCESS_TYPE`, `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH`, `VSCODE_ESM_ENTRYPOINT`.
  - **Outbound (child → parent):** Console output is serialized through `safeToString` (line 21), packaged as `{ type: '__$console', severity, arguments }` at line 96, and sent via `process.send()` at line 80. Stream writes are also routed back through `console[severity]` (line 130), which is already wrapped, so they flow through the same IPC path.
  - **Stream buffering:** `wrapStream` maintains a per-stream string buffer `buf` (line 122) that accumulates chunks. It scans for the last newline (`buf.lastIndexOf('\n')`, line 128) or flushes when the buffer exceeds `MAX_STREAM_BUFFER_LENGTH` (1 MB, line 15). Complete lines are forwarded; the remainder stays in `buf`.
  - **Circular reference detection:** `safeToString` maintains a `seen` array (line 23). For every object/array encountered during `JSON.stringify` replacer execution (line 53-65), it checks `seen.indexOf(value)` and either inserts `'[Circular]'` or pushes the value into `seen`.
  - **Error serialization:** `safeToString` converts `Error` instances to their `.stack` string (lines 39-46) and `undefined` values to the string `'undefined'` (lines 33-35) before JSON serialization.
  - **Output truncation:** `safeToString` returns a fixed message string if `res.length > MAX_LENGTH` (100,000 chars, line 16 and line 67-69).

- **Dependencies:**

  | Import | Source |
  |---|---|
  | `performance` namespace | `./vs/base/common/performance.js` (line 6) |
  | `removeGlobalNodeJsModuleLookupPaths` | `./bootstrap-node.js` (line 7) |
  | `devInjectNodeModuleLookupPath` | `./bootstrap-node.js` (line 7) |
  | `bootstrapESM` | `./bootstrap-esm.js` (line 8) |

  Runtime: depends on Node.js built-ins `process`, `console`, `setInterval`, `Buffer` (all globals). Uses Node.js IPC channel (`process.send`). Uses Electron's `process.crashReporter` when present.

---

### Cross-Cutting Synthesis

`src/bootstrap-fork.ts` is the shared initialization harness for all forked VS Code worker processes. It establishes a controlled, observable sub-process environment before any feature code runs. The architecture is purely environment-variable-driven: the parent spawns a child with specific `VSCODE_*` variables and the child self-configures accordingly — no command-line argument parsing, no config files. IPC is unidirectional in this file (child-to-parent only), using the structured `__$console` message type to relay console output. The parent watchdog poll (`process.kill(parentPid, 0)` every 5 seconds) implements a simple but effective orphan-process prevention pattern at lines 173-179. ESM bootstrapping is deferred to the very end (line 226), and the actual worker entrypoint is dynamically imported by name from `VSCODE_ESM_ENTRYPOINT` (line 229), making this a generic launcher reused across all worker types (extension host, language server, terminal, etc.) without any worker-specific branching. For a Tauri/Rust port, this entire harness — IPC channel, stream interception, PID polling, environment-variable-based dispatch, ESM dynamic loading — would need to be re-implemented in terms of Rust's process primitives and Tauri's inter-process communication layer, as there is no direct analog in the Tauri/Rust ecosystem for Node.js's `process.send()` IPC or dynamic ESM import.

---

### Out-of-Partition References

- `src/vs/base/common/performance.ts` — performance mark utility used at line 6/10
- `src/bootstrap-node.ts` — provides `removeGlobalNodeJsModuleLookupPaths` and `devInjectNodeModuleLookupPath`, called at lines 204 and 207
- `src/bootstrap-esm.ts` — provides `bootstrapESM()`, called at line 226; sets up NLS, product globals, and optional Electron `fs` → `original-fs` module hook
- `src/bootstrap-meta.ts` — referenced transitively through `bootstrap-esm.ts` (product/pkg globals)
- `src/vs/nls.ts` — `INLSConfiguration` type referenced in `bootstrap-esm.ts`

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Analysis: VS Code Fork Lifecycle & Multi-Process Architecture
## Research Partition 51 of 79

### Scope
`src/bootstrap-fork.ts` — Fork lifecycle (extension host child process) — central to multi-process architecture a Rust port must replicate.

---

#### Pattern: Process Lifecycle Hooks - Exception Handling
**Where:** `src/bootstrap-fork.ts:159-166`
**What:** Registers handlers for uncaught exceptions and unhandled promise rejections to log errors before process termination.
```typescript
function handleExceptions(): void {
	// Handle uncaught exceptions
	process.on('uncaughtException', function (err) {
		console.error('Uncaught Exception: ', err);
	});

	// Handle unhandled promise rejections
	process.on('unhandledRejection', function (reason) {
		console.error('Unhandled Promise Rejection: ', reason);
	});
}
```
**Variations / call-sites:**
- `src/vs/workbench/api/node/extensionHostProcess.ts:403-436` — Extension host variant with tracking of unhandled promises and SIGPIPE filtering
- `src/vs/platform/telemetry/electron-main/errorTelemetry.ts:23-29` — Main process telemetry integration
- `build/gulpfile.ts:50` — Build system error handling

---

#### Pattern: Parent Process Liveness Monitoring
**Where:** `src/bootstrap-fork.ts:169-181`
**What:** Child process periodically checks parent process liveness via kill(pid, 0) and exits if parent dies.
```typescript
function terminateWhenParentTerminates(): void {
	const parentPid = Number(process.env['VSCODE_PARENT_PID']);

	if (typeof parentPid === 'number' && !isNaN(parentPid)) {
		setInterval(function () {
			try {
				process.kill(parentPid, 0); // throws an exception if the main process doesn't exist anymore.
			} catch (e) {
				process.exit();
			}
		}, 5000);
	}
}
```
**Variations / call-sites:**
- `src/vs/workbench/api/node/extensionHostProcess.ts:351-384` — Extension host with EPERM error handling (3-strike policy for antivirus interference) and native watchdog
- `src/vs/platform/utilityProcess/electron-main/utilityProcess.ts:282` — Sets `VSCODE_PARENT_PID` env var for utility processes

---

#### Pattern: IPC Logging Pipe to Parent
**Where:** `src/bootstrap-fork.ts:14-154`
**What:** Intercepts console methods and process streams, serializes output with circular reference detection, sends messages back to parent via process.send().
```typescript
function pipeLoggingToParent(): void {
	const MAX_STREAM_BUFFER_LENGTH = 1024 * 1024;
	const MAX_LENGTH = 100000;

	function safeToString(args: ArrayLike<unknown>): string {
		const seen: unknown[] = [];
		// ... circular reference tracking ...
		return JSON.stringify(argsArray, function (key, value: unknown) {
			if (isObject(value) || Array.isArray(value)) {
				if (seen.indexOf(value) !== -1) {
					return '[Circular]';
				}
				seen.push(value);
			}
			return value;
		});
	}

	function safeSend(arg: { type: string; severity: string; arguments: string }): void {
		try {
			if (process.send) {
				process.send(arg);
			}
		} catch (error) {
			// Can happen if the parent channel is closed meanwhile
		}
	}

	function wrapConsoleMethod(method: 'log' | 'info' | 'warn' | 'error', severity: 'log' | 'warn' | 'error'): void {
		Object.defineProperty(console, method, {
			set: () => { },
			get: () => function () { safeSendConsoleMessage(severity, safeToString(arguments)); },
		});
	}
}
```
**Variations / call-sites:**
- `src/vs/base/parts/ipc/node/ipc.cp.ts:24-37` — Server-side IPC listening on process 'message' and 'disconnect' events
- `src/vs/workbench/api/node/extensionHostProcess.ts:269` — Extension host sends ready message via `process.send?()`
- `test/unit/analyzeSnapshot.js:81-82` — Test worker sends results back via process.send()

---

#### Pattern: Conditional Configuration via Environment Variables
**Where:** `src/bootstrap-fork.ts:140-229`
**What:** Fork initialization is driven entirely by environment variables, enabling flexible bootstrap behavior without code paths.
```typescript
// Logging
if (process.env['VSCODE_VERBOSE_LOGGING'] === 'true') {
	wrapConsoleMethod('info', 'log');
	wrapConsoleMethod('log', 'log');
	wrapConsoleMethod('warn', 'warn');
	wrapConsoleMethod('error', 'error');
}

// Crash reporting
const crashReporterProcessType = process.env['VSCODE_CRASH_REPORTER_PROCESS_TYPE'];

// Node module path injection
if (process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH']) {
	devInjectNodeModuleLookupPath(process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH']);
}

// IPC logging
if (!!process.send && process.env['VSCODE_PIPE_LOGGING'] === 'true') {
	pipeLoggingToParent();
}

// Error handling
if (!process.env['VSCODE_HANDLES_UNCAUGHT_ERRORS']) {
	handleExceptions();
}

// Parent monitoring
if (process.env['VSCODE_PARENT_PID']) {
	terminateWhenParentTerminates();
}

// ESM entrypoint
await import([`./${process.env['VSCODE_ESM_ENTRYPOINT']}.js`].join('/'));
```
**Variations / call-sites:**
- `src/vs/base/parts/ipc/node/ipc.cp.ts:180` — Parent sets `VSCODE_PARENT_PID` when forking
- `src/vs/workbench/api/node/extensionHostProcess.ts:41-52` — Extension host removes experimental warning listeners
- `src/bootstrap-esm.ts:14-30` — Module resolution hook for fs→original-fs mapping

---

#### Pattern: Child Process Fork and IPC Setup
**Where:** `src/vs/base/parts/ipc/node/ipc.cp.ts:175-250`
**What:** Parent process forks child, configures execArgv for debugging, sets environment variables, and establishes IPC channels.
```typescript
private get client(): IPCClient {
	if (!this._client) {
		const args = this.options.args || [];
		const forkOpts: ForkOptions = Object.create(null);

		forkOpts.env = { ...deepClone(process.env), 'VSCODE_PARENT_PID': String(process.pid) };

		if (this.options.env) {
			forkOpts.env = { ...forkOpts.env, ...this.options.env };
		}

		if (this.options.freshExecArgv) {
			forkOpts.execArgv = [];
		}

		if (typeof this.options.debug === 'number') {
			forkOpts.execArgv = ['--nolazy', '--inspect=' + this.options.debug];
		}

		if (typeof this.options.debugBrk === 'number') {
			forkOpts.execArgv = ['--nolazy', '--inspect-brk=' + this.options.debugBrk];
		}

		if (forkOpts.execArgv === undefined) {
			forkOpts.execArgv = process.execArgv
				.filter(a => !/^--inspect(-brk)?=/.test(a))
				.filter(a => !a.startsWith('--vscode-'));
		}

		removeDangerousEnvVariables(forkOpts.env);

		this.child = fork(this.modulePath, args, forkOpts);

		const onMessageEmitter = new Emitter<VSBuffer>();
		const onRawMessage = Event.fromNodeEventEmitter(this.child, 'message', msg => msg);

		const rawMessageDisposable = onRawMessage(msg => {
			if (isRemoteConsoleLog(msg)) {
				log(msg, `IPC Library: ${this.options.serverName}`);
				return;
			}
			onMessageEmitter.fire(VSBuffer.wrap(Buffer.from(msg, 'base64')));
		});

		const sender = this.options.useQueue ? createQueuedSender(this.child) : this.child;
		const send = (r: VSBuffer) => this.child?.connected && sender.send((<Buffer>r.buffer).toString('base64'));
		const onMessage = onMessageEmitter.event;
		const protocol = { send, onMessage };

		this._client = new IPCClient(protocol);

		this.child.on('error', err => console.warn('IPC "' + this.options.serverName + '" errored with ' + err));

		this.child.on('exit', (code: any, signal: any) => {
			rawMessageDisposable.dispose();
			this.activeRequests.forEach(r => dispose(r));
			this.activeRequests.clear();

			if (code !== 0 && signal !== 'SIGTERM') {
				console.warn('IPC "' + this.options.serverName + '" crashed with exit code ' + code + ' and signal ' + signal);
			}

			this.disposeDelayer?.cancel();
			this.disposeClient();
			this._onDidProcessExit.fire({ code, signal });
		});
	}

	return this._client;
}
```
**Variations / call-sites:**
- Extension host process creation uses similar patterns but with socket/MessagePort protocols instead of message-based IPC
- `src/vs/workbench/services/extensions/electron-browser/localProcessExtensionHost.ts` — Desktop extension host setup

---

#### Pattern: Process Signal Handling and Cleanup
**Where:** `src/bootstrap-node.ts:17-30` and `src/server-main.ts:146-151`
**What:** Registers signal handlers to ensure graceful shutdown; parent attaches exit listener to child process.
```typescript
// bootstrap-node.ts - SIGPIPE handling
if (!process.env['VSCODE_HANDLES_SIGPIPE']) {
	let didLogAboutSIGPIPE = false;
	process.on('SIGPIPE', () => {
		if (!didLogAboutSIGPIPE) {
			didLogAboutSIGPIPE = true;
			console.error(new Error(`Unexpected SIGPIPE`));
		}
	});
}

// server-main.ts - exit cleanup
process.on('exit', () => {
	server.close();
	if (_remoteExtensionHostAgentServer) {
		_remoteExtensionHostAgentServer.dispose();
	}
});
```
**Variations / call-sites:**
- `scripts/code-server.js:56-65` — Script-level signal handlers for SIGINT and SIGTERM with process.exit codes
- `src/vs/platform/agentHost/node/agentHostServerMain.ts:268-269` — Agent host SIGTERM/SIGINT handlers
- `src/vs/base/parts/ipc/node/ipc.cp.ts:230-236` — Child process exit listener cleanup

---

#### Pattern: Extension Host Protocol with Reconnection Grace Time
**Where:** `src/vs/workbench/api/node/extensionHostProcess.ts:180-270`
**What:** Establishes IPC connection with reconnection support, grace time before termination if parent disconnects, and socket-level message framing.
```typescript
function _createExtHostProtocol(): Promise<IMessagePassingProtocol> {
	// ... MessagePort and Socket variants ...
	
	return new Promise<PersistentProtocol>((resolve, reject) => {
		let protocol: PersistentProtocol | null = null;

		const timer = setTimeout(() => {
			onTerminate('VSCODE_EXTHOST_IPC_SOCKET timeout');
		}, 60000);

		const reconnectionGraceTime = readReconnectionValue('VSCODE_RECONNECTION_GRACE_TIME', ProtocolConstants.ReconnectionGraceTime);
		const reconnectionShortGraceTime = reconnectionGraceTime > 0 ? Math.min(ProtocolConstants.ReconnectionShortGraceTime, reconnectionGraceTime) : 0;
		const disconnectRunner1 = new ProcessTimeRunOnceScheduler(() => onTerminate('renderer disconnected for too long (1)'), reconnectionGraceTime);
		const disconnectRunner2 = new ProcessTimeRunOnceScheduler(() => onTerminate('renderer disconnected for too long (2)'), reconnectionShortGraceTime);

		process.on('message', (msg: IExtHostSocketMessage | IExtHostReduceGraceTimeMessage, handle: net.Socket) => {
			if (msg && msg.type === 'VSCODE_EXTHOST_IPC_SOCKET') {
				handle.setNoDelay(true);
				const initialDataChunk = VSBuffer.wrap(Buffer.from(msg.initialDataChunk, 'base64'));
				// ... socket setup ...
				if (protocol) {
					// reconnection case
					protocol.beginAcceptReconnection(socket, initialDataChunk);
					protocol.endAcceptReconnection();
					protocol.sendResume();
				} else {
					protocol = new PersistentProtocol({ socket, initialChunk: initialDataChunk });
					protocol.sendResume();
					Event.once(protocol.onDidDispose)(() => onTerminate('renderer disconnected'));
					resolve(protocol);

					protocol.onSocketClose(() => {
						disconnectRunner1.schedule();
					});
				}
			}
			if (msg && msg.type === 'VSCODE_EXTHOST_IPC_REDUCE_GRACE_TIME') {
				if (disconnectRunner2.isScheduled()) {
					return;
				}
				if (disconnectRunner1.isScheduled()) {
					disconnectRunner2.schedule();
				}
			}
		});

		const req: IExtHostReadyMessage = { type: 'VSCODE_EXTHOST_IPC_READY' };
		process.send?.(req);
	});
}
```
**Variations / call-sites:**
- MessagePort variant for utility processes (modern approach)
- Named pipe variant for remote scenarios

---

## Summary

VS Code's fork lifecycle demonstrates a sophisticated multi-process architecture built on Node.js primitives:

1. **Bootstrap Pattern**: Child processes are initialized entirely via environment variables passed during fork, enabling dynamic behavior without binary recompilation.

2. **Bidirectional IPC**: Communication flows both ways—parent sends initialization messages and configuration, child sends logs and console output back via base64-encoded process.send().

3. **Liveness Monitoring**: Both parent→child (timeout-based for socket/port connections) and child→parent (periodic kill(pid, 0) checks) detect process failures.

4. **Signal Safety**: SIGPIPE is explicitly handled; exit handlers ensure cleanup of sockets and server resources.

5. **Graceful Degradation**: IPC failures don't crash processes—they log warnings and continue. Parent-child disconnections trigger reconnection grace periods (configurable via env vars).

6. **Environment-Driven Configuration**: All behavioral switches (`VSCODE_PIPE_LOGGING`, `VSCODE_VERBOSE_LOGGING`, `VSCODE_HANDLES_UNCAUGHT_ERRORS`, etc.) flow through process.env.

A Rust port must replicate:
- Process spawning with inherited/filtered execArgv
- IPC channel establishment (socket, pipe, or MessagePort equivalent)
- Parent PID tracking and liveness polls
- Exception/promise rejection handlers
- Graceful reconnection windows with configurable grace periods
- Environment variable injection for feature flags and configuration
- Signal handlers for SIGPIPE, SIGINT, SIGTERM

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
