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
