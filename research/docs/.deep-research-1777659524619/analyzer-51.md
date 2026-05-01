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

