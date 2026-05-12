# Extension-Host Bootstrap Patterns (bootstrap-fork.ts)

## Pattern: Process Event Isolation Handlers

**Where:** `src/bootstrap-fork.ts:159-166`
**What:** Registers Node.js process event handlers for uncaught exceptions and unhandled promise rejections.

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

**Variations/call-sites:** Invoked conditionally at line 217 based on `VSCODE_HANDLES_UNCAUGHT_ERRORS` environment variable. The port must implement equivalent signal/error handling in Rust to catch unhandled panics and async failures.

---

## Pattern: Parent Process Liveliness Polling

**Where:** `src/bootstrap-fork.ts:169-181`
**What:** Monitors parent process death by periodically signaling it; exits child when parent is gone.

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

**Variations/call-sites:** Triggered at line 222 if `VSCODE_PARENT_PID` is set. The port must replicate this with OS-level process polling (via parent PID checks or handle ownership).

---

## Pattern: IPC Logging via process.send()

**Where:** `src/bootstrap-fork.ts:77-85`
**What:** Safely sends structured log objects to parent process via IPC, with error recovery.

```typescript
function safeSend(arg: { type: string; severity: string; arguments: string }): void {
	try {
		if (process.send) {
			process.send(arg);
		}
	} catch (error) {
		// Can happen if the parent channel is closed meanwhile
	}
}
```

**Variations/call-sites:** Called by `safeSendConsoleMessage()` at line 96 with message type `__$console`. The port must implement a cross-process message channel (likely via IPC or websocket in Tauri).

---

## Pattern: Console Output Interception via Object.defineProperty

**Where:** `src/bootstrap-fork.ts:105-110`
**What:** Intercepts console method calls by replacing their getter to send logs to parent.

```typescript
function wrapConsoleMethod(method: 'log' | 'info' | 'warn' | 'error', severity: 'log' | 'warn' | 'error'): void {
	Object.defineProperty(console, method, {
		set: () => { },
		get: () => function () { safeSendConsoleMessage(severity, safeToString(arguments)); },
	});
}
```

**Variations/call-sites:** Invoked at lines 141-149. Selective wrapping based on `VSCODE_VERBOSE_LOGGING` flag. In Rust, this requires hooking into the logging system at initialization.

---

## Pattern: Stream Buffering and Line-Splitting for Redirection

**Where:** `src/bootstrap-fork.ts:118-137`
**What:** Intercepts stdout/stderr writes, buffers incomplete lines, splits on newlines, and forwards to console.

```typescript
function wrapStream(streamName: 'stdout' | 'stderr', severity: 'log' | 'warn' | 'error'): void {
	const stream = process[streamName];
	const original = stream.write;

	let buf = '';

	Object.defineProperty(stream, 'write', {
		set: () => { },
		get: () => (chunk: string | Buffer | Uint8Array, encoding: BufferEncoding | undefined, callback: ((err?: Error | null) => void) | undefined) => {
			buf += chunk.toString(encoding);
			const eol = buf.length > MAX_STREAM_BUFFER_LENGTH ? buf.length : buf.lastIndexOf('\n');
			if (eol !== -1) {
				console[severity](buf.slice(0, eol));
				buf = buf.slice(eol + 1);
			}

			original.call(stream, chunk, encoding, callback);
		},
	});
}
```

**Variations/call-sites:** Applied to both `stdout` and `stderr` at lines 152-153. The port must capture writes from child processes and implement line-buffering in Rust.

---

## Pattern: Circular Reference Handling in Log Serialization

**Where:** `src/bootstrap-fork.ts:52-75`
**What:** Prevents stack overflow when stringifying objects with circular references; truncates oversized output.

```typescript
try {
	const res = JSON.stringify(argsArray, function (key, value: unknown) {

		// Objects get special treatment to prevent circles
		if (isObject(value) || Array.isArray(value)) {
			if (seen.indexOf(value) !== -1) {
				return '[Circular]';
			}

			seen.push(value);
		}

		return value;
	});

	if (res.length > MAX_LENGTH) {
		return 'Output omitted for a large object that exceeds the limits';
	}

	return res;
} catch (error) {
	return `Output omitted for an object that cannot be inspected ('${error.toString()}')`;
}
```

**Variations/call-sites:** Used by `safeToString()` at line 21. Limits: `MAX_LENGTH = 100000`, `MAX_STREAM_BUFFER_LENGTH = 1024 * 1024`. The port must implement equivalent serialization safety in Rust.

---

## Pattern: Environment-Driven Bootstrap Feature Flags

**Where:** `src/bootstrap-fork.ts:200-229`
**What:** Conditionally activates subsystems based on environment variables; defers ESM loading until all initialization completes.

```typescript
// Crash reporter
configureCrashReporter();

// Remove global paths from the node module lookup (node.js only)
removeGlobalNodeJsModuleLookupPaths();

if (process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH']) {
	devInjectNodeModuleLookupPath(process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH']);
}

// Configure: pipe logging to parent process
if (!!process.send && process.env['VSCODE_PIPE_LOGGING'] === 'true') {
	pipeLoggingToParent();
}

// Handle Exceptions
if (!process.env['VSCODE_HANDLES_UNCAUGHT_ERRORS']) {
	handleExceptions();
}

// Terminate when parent terminates
if (process.env['VSCODE_PARENT_PID']) {
	terminateWhenParentTerminates();
}

// Bootstrap ESM
await bootstrapESM();

// Load ESM entry point
await import([`./${process.env['VSCODE_ESM_ENTRYPOINT']}.js`].join('/') /* workaround: esbuild prints some strange warnings when trying to inline? */);
```

**Variations/call-sites:** Primary bootstrap sequence. Flags used: `VSCODE_VERBOSE_LOGGING`, `VSCODE_PARENT_PID`, `VSCODE_PIPE_LOGGING`, `VSCODE_HANDLES_UNCAUGHT_ERRORS`, `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH`, `VSCODE_CRASH_REPORTER_PROCESS_TYPE`, `VSCODE_ESM_ENTRYPOINT`. The port must support this same configuration interface via command-line args or embedded config.

---

## Pattern: Performance Markers for Tracing

**Where:** `src/bootstrap-fork.ts:6, 10`
**What:** Logs a high-resolution timing mark at fork start.

```typescript
import * as performance from './vs/base/common/performance.js';

performance.mark('code/fork/start');
```

**Variations/call-sites:** Mark enables timeline profiling. Rust port should use `std::time::Instant` or equivalent to instrument startup phases.

---

## Summary

The bootstrap-fork.ts file encapsulates seven core patterns essential for extension-host isolation:

1. **Process event isolation** — catches unhandled exceptions and promise rejections
2. **Parent liveness polling** — ensures child terminates when parent dies
3. **IPC message passing** — bidirectional structured message channel to parent
4. **Console interception** — redirects logging via property descriptor replacement
5. **Stream buffering** — handles partial writes and line reassembly
6. **Serialization safety** — guards against circular refs and size limits
7. **Feature flags** — environment variables control subsystem initialization

A Tauri/Rust port must replicate these patterns using: process management APIs (libc), IPC/channel mechanisms (likely JSON-RPC over pipes or websocket), global logging hooks, stream redirection (Unix file descriptors or Windows handles), and serialization (serde with custom error handling). The sequential bootstrap ordering—crash reporting, module paths, logging setup, error handlers, parent monitoring, then ESM init—must be preserved.
