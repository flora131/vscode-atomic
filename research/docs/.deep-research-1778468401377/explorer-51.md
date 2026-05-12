# Partition 51 of 80 — Findings

## Scope
`src/bootstrap-fork.ts/` (1 files, 229 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator Report: bootstrap-fork.ts

## Scope Analyzed
- `src/bootstrap-fork.ts` (229 LOC)

## Findings

### Implementation
- `src/bootstrap-fork.ts` — Electron process bootstrap entry point for forked worker processes; handles logging, error handling, parent process termination tracking, and crash reporting for child processes

## Summary

This single file in the scope represents the bootstrapping logic for Electron's forked child processes in VS Code. It is directly tied to Electron's process model and IPC mechanisms (`process.send`, `process.kill`, `process.env`). Porting VS Code from Electron to Tauri/Rust would require replacing this entire process fork orchestration with Tauri's command/event-based IPC model. No additional tests, types, configuration, or documentation related to this mechanism exist within this scope.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `src/bootstrap-fork.ts` (229 LOC)

---

### Per-File Notes

#### `src/bootstrap-fork.ts`

- **Role:** Top-level module executed in every Electron-forked child process (extension host, shared process, pty host, etc.). It is responsible for: recording a startup timing mark, configuring logging to the parent process via IPC, installing global unhandled-error handlers, polling to detect parent-process death, attaching an Electron crash-reporter tag, hardening the Node.js module-resolution paths, and finally chaining to the common ESM bootstrap (`bootstrapESM`) before dynamically importing the worker-specific entry point named by `VSCODE_ESM_ENTRYPOINT`.

- **Key symbols:**
  - `performance.mark('code/fork/start')` (`src/bootstrap-fork.ts:10`) — writes the first timing mark for the forked process into the shared performance buffer.
  - `pipeLoggingToParent()` (`src/bootstrap-fork.ts:14`) — outer function that defines and installs all console/stream interception.
  - `safeToString(args)` (`src/bootstrap-fork.ts:21`) — inner helper; serialises an `arguments`-like object to JSON with circular-reference detection (`[Circular]` sentinel at `src/bootstrap-fork.ts:58`), `undefined`-to-string coercion (`src/bootstrap-fork.ts:33`), and Error-to-stack unwrapping (`src/bootstrap-fork.ts:39–46`); truncates at 100 000 characters (`src/bootstrap-fork.ts:67`).
  - `safeSend(arg)` (`src/bootstrap-fork.ts:77`) — wraps `process.send()` in a try/catch to tolerate a closed IPC channel.
  - `isObject(obj)` (`src/bootstrap-fork.ts:87`) — narrow type-guard excluding arrays, RegExp, and Date from "plain object" classification.
  - `safeSendConsoleMessage(severity, args)` (`src/bootstrap-fork.ts:95`) — composes the IPC message envelope `{ type: '__$console', severity, arguments }` and hands it to `safeSend`.
  - `wrapConsoleMethod(method, severity)` (`src/bootstrap-fork.ts:105`) — redefines a `console` property via `Object.defineProperty` so both `get` (returns the wrapped function) and `set` (no-op) are overridden, avoiding the `writable:false` error described in the GitHub comment at line 103.
  - `wrapStream(streamName, severity)` (`src/bootstrap-fork.ts:118`) — patches `process.stdout.write` / `process.stderr.write`; accumulates output in a line buffer `buf` (`src/bootstrap-fork.ts:122`), flushes complete lines to the wrapped console on each newline (or when `buf` exceeds `MAX_STREAM_BUFFER_LENGTH = 1 MB` at `src/bootstrap-fork.ts:128`), and still passes data through to the original stream (`src/bootstrap-fork.ts:134`).
  - `handleExceptions()` (`src/bootstrap-fork.ts:156`) — registers `process.on('uncaughtException')` and `process.on('unhandledRejection')` handlers that both forward to `console.error`.
  - `terminateWhenParentTerminates()` (`src/bootstrap-fork.ts:169`) — reads `VSCODE_PARENT_PID` from the environment, converts to a number, and starts a `setInterval` every 5 000 ms that calls `process.kill(parentPid, 0)` (signal-0, existence probe); if the call throws the child calls `process.exit()`.
  - `configureCrashReporter()` (`src/bootstrap-fork.ts:183`) — reads `VSCODE_CRASH_REPORTER_PROCESS_TYPE`; if set, calls `process['crashReporter'].addExtraParameter('processType', …)` guarded by an `@ts-expect-error` because `crashReporter` is Electron-only and absent from Node types.
  - `removeGlobalNodeJsModuleLookupPaths()` (`src/bootstrap-fork.ts:204`) — imported from `./bootstrap-node.js`; cleans up `NODE_PATH` / global lookup directories so the child cannot accidentally load host-system packages.
  - `devInjectNodeModuleLookupPath(path)` (`src/bootstrap-fork.ts:207`) — imported from `./bootstrap-node.js`; conditionally prepends a dev-mode path when `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH` is set.
  - `bootstrapESM()` (`src/bootstrap-fork.ts:226`) — imported from `./bootstrap-esm.js`; awaited; sets up NLS, product globals, and the `original-fs` ESM hook.
  - Dynamic `import(…VSCODE_ESM_ENTRYPOINT…)` (`src/bootstrap-fork.ts:229`) — the final top-level await that loads the worker-specific module whose path is injected at fork time.

- **Control flow:**
  1. `performance.mark('code/fork/start')` executes immediately on module load (`src/bootstrap-fork.ts:10`).
  2. `configureCrashReporter()` is called unconditionally (`src/bootstrap-fork.ts:201`).
  3. `removeGlobalNodeJsModuleLookupPaths()` is called unconditionally (`src/bootstrap-fork.ts:204`).
  4. `devInjectNodeModuleLookupPath` is called only if `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH` is present (`src/bootstrap-fork.ts:206–208`).
  5. `pipeLoggingToParent()` is called only when both `process.send` exists (IPC channel open) **and** `VSCODE_PIPE_LOGGING === 'true'` (`src/bootstrap-fork.ts:211`).
     - Inside, the verbose vs. silent console branching occurs at `src/bootstrap-fork.ts:140–149`: if `VSCODE_VERBOSE_LOGGING === 'true'`, all four console methods are wrapped; otherwise `log`, `warn`, `info` are suppressed and only `error` is forwarded.
     - `wrapStream('stderr', 'error')` and `wrapStream('stdout', 'log')` are always called within `pipeLoggingToParent` (`src/bootstrap-fork.ts:152–153`).
  6. `handleExceptions()` is called when `VSCODE_HANDLES_UNCAUGHT_ERRORS` is **absent** (`src/bootstrap-fork.ts:216–218`).
  7. `terminateWhenParentTerminates()` is called when `VSCODE_PARENT_PID` is present (`src/bootstrap-fork.ts:221–223`).
  8. `await bootstrapESM()` (`src/bootstrap-fork.ts:226`) — sets up product/NLS globals.
  9. `await import(…VSCODE_ESM_ENTRYPOINT…)` (`src/bootstrap-fork.ts:229`) — loads the actual worker entry point.

- **Data flow:**
  - Console output path: child `console.*` call → `safeToString` → `safeSendConsoleMessage` builds `{ type: '__$console', severity, arguments: <json-string> }` → `safeSend` → `process.send()` → parent process IPC channel.
  - Stream output path: `process.stdout`/`process.stderr` `.write(chunk)` → line buffer `buf` → flushed via `console[severity]` (which itself is now the IPC-forwarding function) → **also** passes the raw chunk to the original `write` implementation so the OS stream still works.
  - Crash metadata path: `VSCODE_CRASH_REPORTER_PROCESS_TYPE` env var → `process['crashReporter'].addExtraParameter('processType', value)`.
  - Parent liveness path: `VSCODE_PARENT_PID` → `Number(…)` → `process.kill(pid, 0)` every 5 s → if exception, `process.exit()`.
  - Entry-point path: `VSCODE_ESM_ENTRYPOINT` env var → template string `./${value}.js` → dynamic `import()`.

- **Dependencies:**
  - `./vs/base/common/performance.js` — `performance.mark` (timing utility).
  - `./bootstrap-node.js` — `removeGlobalNodeJsModuleLookupPaths`, `devInjectNodeModuleLookupPath`.
  - `./bootstrap-esm.js` — `bootstrapESM` (NLS / product-global setup, `original-fs` hook for Electron).
  - Node.js built-ins: `process.send`, `process.kill`, `process.exit`, `process.env`, `process.stdout`, `process.stderr`, `setInterval`.
  - Electron-only runtime API: `process['crashReporter'].addExtraParameter` (conditionally present, guarded at `src/bootstrap-fork.ts:188`).

---

### Cross-Cutting Synthesis

`src/bootstrap-fork.ts` is the universal preamble for every Electron-forked worker in VS Code. Its design reflects a strict layering: it runs pure environment-setup code (no application logic) before handing off to `bootstrapESM` and then to the real entry module. The IPC logging bridge (`pipeLoggingToParent`) is the most structurally notable piece: it intercepts both the high-level console API and the low-level stream `.write` methods, serialises everything with circular-reference-safe JSON, and emits structured `__$console` IPC messages so the main/renderer process can display child-process logs in the DevTools console. The parent-liveness polling (`terminateWhenParentTerminates`) is a fallback orphan-prevention mechanism: because POSIX `fork` does not provide automatic child death on parent exit, the child polls with signal-0 every 5 s. The entire file is effectively a configuration DSL driven by environment variables (`VSCODE_PIPE_LOGGING`, `VSCODE_VERBOSE_LOGGING`, `VSCODE_HANDLES_UNCAUGHT_ERRORS`, `VSCODE_PARENT_PID`, `VSCODE_CRASH_REPORTER_PROCESS_TYPE`, `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH`, `VSCODE_ESM_ENTRYPOINT`), all injected by the main process at fork time.

---

### Out-of-Partition References

- `src/bootstrap-node.ts` — provides `removeGlobalNodeJsModuleLookupPaths` and `devInjectNodeModuleLookupPath`; Node.js path and module utilities; not part of this partition.
- `src/bootstrap-esm.ts` — provides `bootstrapESM`; handles NLS configuration, product/package globals, and the `node:original-fs` ESM loader hook; not part of this partition.
- `src/vs/base/common/performance.ts` — shared performance-mark ring-buffer used across main, renderer, and worker processes; not part of this partition.
- The module identified by `VSCODE_ESM_ENTRYPOINT` at runtime (e.g. `vs/workbench/api/node/extensionHostProcess`, `vs/platform/terminal/node/ptyHostMain`, etc.) — the actual worker loaded at `src/bootstrap-fork.ts:229`; not part of this partition.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## External References
<!-- Source: codebase-online-researcher sub-agent -->
#### Electron crashReporter — `process.crashReporter` for Node child processes (Electron v35+)
**Docs:** https://www.electronjs.org/docs/latest/api/crash-reporter
**Relevant behaviour:** In Electron-forked Node child processes (spawned with `utilityProcess` or `child_process.fork`), the full `require('electron')` module is unavailable, so Electron injects a subset of the `crashReporter` API directly onto the global `process` object. The available surface is `process.crashReporter.start(options)`, `process.crashReporter.getParameters()`, `process.crashReporter.addExtraParameter(key, value)`, and `process.crashReporter.removeExtraParameter(key)`. The method `addExtraParameter` attaches per-process string annotations (key ≤ 39 bytes, value ≤ 20320 bytes) to any minidump crash reports generated in that child process. These annotations are strictly local to the calling process: "Adding extra parameters in the main process will not cause those parameters to be sent along with crashes from renderer or other child processes." VS Code uses this to stamp the forked process's crash reports with a `processType` label (e.g. `extensionHost`, `watcherService`) so that minidumps uploaded to the Electron Crashpad collection server carry a human-readable process-type tag. The access pattern is guarded at runtime with `process['crashReporter'] && typeof process['crashReporter'].addExtraParameter === 'function'` because this property is absent in plain Node.js. In a Tauri port there is no equivalent: Tauri's crash-handling is done at the Rust level via the OS crash handler or an external SDK (e.g. Sentry's `sentry-rust`); there is no `process.crashReporter` injected into JS workers or sidecar processes, and no built-in per-process annotation API. Any Tauri replacement would need a bespoke IPC call from the JS/WebView layer to a Rust command that writes Crashpad/Breakpad extra-parameters, or an entirely different telemetry pipeline.
**Where used:** `src/bootstrap-fork.ts:188-190`

`bootstrap-fork.ts` is almost entirely plain Node.js (IPC via `process.send`, parent-liveness polling via `process.kill(pid, 0)`, console/stream wrapping, and ESM bootstrapping). The single Electron-specific dependency is the four-line `configureCrashReporter()` function that calls `process.crashReporter.addExtraParameter('processType', ...)`, which the code already guards with a feature-detect comment marking it `/* Electron only */`; replacing it in a Tauri port reduces to either a no-op stub or a Rust-side sidecar annotation call.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
