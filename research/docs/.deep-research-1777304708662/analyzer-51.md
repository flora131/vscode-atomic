### Files Analysed

1. `/Users/norinlavaee/vscode-atomic/src/bootstrap-fork.ts` — Primary subject; the extension-host child-process bootstrapper (229 LOC).
2. `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts` — Companion node-specific bootstrap (191 LOC); provides `removeGlobalNodeJsModuleLookupPaths`, `devInjectNodeModuleLookupPath`, `configurePortable`.
3. `/Users/norinlavaee/vscode-atomic/src/bootstrap-esm.ts` — Companion ESM bootstrap (113 LOC); provides `bootstrapESM()`, NLS configuration loader.
4. `/Users/norinlavaee/vscode-atomic/src/vs/base/parts/ipc/node/ipc.cp.ts` — Child-process IPC channel layer (first 80 LOC read); shows the `Server` class that wraps `process.send`/`process.on('message')` in a higher-level channel.
5. `/Users/norinlavaee/vscode-atomic/src/vs/platform/utilityProcess/electron-main/utilityProcess.ts` — Main-process side of the utility/extension-host process spawner (first 120 LOC read); shows `IUtilityProcessConfiguration` and Electron's `utilityProcess` API usage.
6. `/Users/norinlavaee/vscode-atomic/src/vs/base/common/performance.ts` — Polyfill-based performance mark API (first 60 LOC read); used for startup timing marks.

---

### Per-File Notes

#### 1. `src/bootstrap-fork.ts` (229 lines total)

**Role**: This is the single entry point executed inside every forked/utility child process (extension host, PTY host, watcher, telemetry service, etc.). It runs before any application logic and installs the low-level plumbing the child process needs.

**Startup sequence** (lines 10–229):

- `performance.mark('code/fork/start')` at line 10 records the earliest timing mark visible across the multi-process system.
- `configureCrashReporter()` (line 201, defined lines 183–196) reads `VSCODE_CRASH_REPORTER_PROCESS_TYPE` from `process.env` and calls Electron-specific `process['crashReporter'].addExtraParameter(...)` (line 189) if available; the call is guarded with `@ts-expect-error` because this API is not part of standard Node.js.
- `removeGlobalNodeJsModuleLookupPaths()` (line 204) delegates to `bootstrap-node.ts` to narrow Node.js module resolution scope.
- `devInjectNodeModuleLookupPath(...)` (line 207) is called only when `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH` is set; it registers a loader hook via `bootstrap-import.js`.
- `pipeLoggingToParent()` (line 212) is called only when `process.send` exists AND `VSCODE_PIPE_LOGGING === 'true'`.
- `handleExceptions()` (line 217) is called only when `VSCODE_HANDLES_UNCAUGHT_ERRORS` is NOT set.
- `terminateWhenParentTerminates()` (line 222) is called only when `VSCODE_PARENT_PID` is set.
- `await bootstrapESM()` (line 226) loads NLS configuration.
- Dynamic `await import(...)` (line 229) loads the actual worker entry point named in `VSCODE_ESM_ENTRYPOINT`.

**`pipeLoggingToParent()` — lines 14–154**:

- Constants: `MAX_STREAM_BUFFER_LENGTH = 1 MB` (line 15), `MAX_LENGTH = 100 000` (line 16).
- `safeToString(args)` (lines 21–75): Converts variadic console arguments to a JSON string. Handles three special cases: `undefined` → literal string `'undefined'` (lines 33–35); `Error` instances → `err.stack` or `err.toString()` (lines 39–46); circular references → `'[Circular]'` via a `seen[]` array (lines 57–60). Truncates to `MAX_LENGTH` (lines 67–69).
- `safeSend(arg)` (lines 77–85): Calls `process.send(arg)` inside a `try/catch`; the catch is intentionally empty because the parent channel may be closed.
- `safeSendConsoleMessage(severity, args)` (lines 95–97): Wraps `safeSend` with a typed envelope `{ type: '__$console', severity, arguments }`.
- `wrapConsoleMethod(method, severity)` (lines 105–110): Uses `Object.defineProperty` to replace `console[method]` with a getter that returns a closure calling `safeSendConsoleMessage`. The setter is a no-op to prevent errors from code that tries to overwrite `console.log`.
- `wrapStream(streamName, severity)` (lines 118–137): Replaces `process.stdout.write` / `process.stderr.write` with a closure that buffers output into `buf`, flushes complete lines to `console[severity]` (which is itself already wrapped), and then still calls the `original.call(stream, ...)` to pass data to the real OS stream. Line buffering is done via `buf.lastIndexOf('\n')` (line 128); if the buffer exceeds `MAX_STREAM_BUFFER_LENGTH` the entire buffer is flushed (line 128).
- Conditional wiring (lines 140–153): When `VSCODE_VERBOSE_LOGGING === 'true'`, all four console methods are wrapped; otherwise `log/warn/info` are replaced with no-ops and only `error` is forwarded. `stderr` and `stdout` streams are always wrapped (lines 152–153).

**`handleExceptions()` — lines 156–167**:

- Registers `process.on('uncaughtException', ...)` (line 159) and `process.on('unhandledRejection', ...)` (line 164).
- Both handlers delegate to `console.error`, which at this point is already wired to `safeSendConsoleMessage`, so uncaught errors are forwarded to the parent process.

**`terminateWhenParentTerminates()` — lines 169–181**:

- Reads parent PID from `process.env['VSCODE_PARENT_PID']` and casts it to `Number` (line 170).
- Starts a `setInterval` polling every 5 000 ms (line 173).
- Inside the interval, calls `process.kill(parentPid, 0)` (line 175). Signal 0 does not send a real signal; it only checks whether the target process exists. If an exception is thrown (process gone), calls `process.exit()` (line 177).

**`configureCrashReporter()` — lines 183–196**:

- Reads `VSCODE_CRASH_REPORTER_PROCESS_TYPE` from env (line 184).
- Guarded by existence check and `typeof addExtraParameter === 'function'` (line 188) with comment `/* Electron only */`.
- The Electron-specific API `process['crashReporter']` is accessed via bracket notation and suppressed with `@ts-expect-error`.

**Dynamic ESM entry point loading — line 229**:

```
await import([`./${process.env['VSCODE_ESM_ENTRYPOINT']}.js`].join('/'));
```

The array-join trick (`[...].join('/')`) is noted in a comment as a workaround for an esbuild warning. The actual ESM module path is fully controlled by the environment variable, making this file a generic runner for any child-process worker type in the VS Code multi-process model.

---

#### 2. `src/bootstrap-node.ts` (191 lines)

**Role**: Common Node.js setup routines imported by `bootstrap-fork.ts` and likely by the main process too.

**`removeGlobalNodeJsModuleLookupPaths()`** (lines 76–128):
- Early-returns when `process.versions.electron` is a string (line 77), because Electron already strips global paths.
- Patches `Module._resolveLookupPaths` (lines 84–98) to strip the suffix of paths that match `Module.globalPaths`.
- On Windows only, also patches `Module._nodeModulePaths` (lines 100–127) to remove drive root paths and the user's home directory `HOMEDRIVE/HOMEPATH` from lookup candidates.

**`devInjectNodeModuleLookupPath(injectPath)`** (lines 62–74):
- Guards on `VSCODE_DEV` env var (line 63).
- Calls `Module.register('./bootstrap-import.js', { parentURL: import.meta.url, data: injectPath })` (line 73), registering a Node.js ESM loader hook.

**`configurePortable(product)`** (lines 133–190):
- Used by other process entry points (not by `bootstrap-fork.ts` directly).
- Resolves portable data path and sets `VSCODE_PORTABLE`, `TMP`/`TEMP`/`TMPDIR` accordingly.

**SIGPIPE handler** (lines 17–30): Installs `process.on('SIGPIPE', ...)` with a one-shot guard (`didLogAboutSIGPIPE`) to avoid infinite async logging loops caused by a broken console pipe.

---

#### 3. `src/bootstrap-esm.ts` (113 lines)

**Role**: Sets up global state required before any VS Code ESM module is imported: product/package metadata, NLS messages, and — on Electron — the `fs` → `original-fs` module hook.

**`fs` redirect hook** (lines 14–30): When `ELECTRON_RUN_AS_NODE` or `process.versions.electron` is set, registers an inline ESM loader hook encoded as a base64 data URL that intercepts `import 'fs'` and redirects it to `node:original-fs`.

**Globals set** (lines 33–35):
- `globalThis._VSCODE_PRODUCT_JSON`
- `globalThis._VSCODE_PACKAGE_JSON`
- `globalThis._VSCODE_FILE_ROOT = import.meta.dirname`

**`doSetupNLS()`** (lines 49–104):
- Reads `VSCODE_NLS_CONFIG` from `process.env` (line 55) and parses it as JSON.
- Determines `messagesFile` from `nlsConfig.languagePack.messagesFile` or `nlsConfig.defaultMessagesFile`.
- Sets `globalThis._VSCODE_NLS_LANGUAGE` (line 64).
- Reads the messages JSON file via `fs.promises.readFile` (line 78) and stores it in `globalThis._VSCODE_NLS_MESSAGES`.
- On read failure, writes a corrupt-marker file (line 85) and attempts the default messages file as a fallback (lines 92–98).
- Uses performance marks `code/willLoadNls` (line 50) and `code/didLoadNls` (line 101).

**`bootstrapESM()`** (lines 108–112): The single exported async function; just awaits `setupNLS()` which is memoised via `setupNLSResult`.

---

#### 4. `src/vs/base/parts/ipc/node/ipc.cp.ts` (partial, first 80 lines)

**Role**: Provides the `Server` class used inside child processes to expose IPC channels over the `process.send` / `process.on('message')` channel.

**`Server` class** (lines 24–37):
- Extends `IPCServer` (from `ipc.common.ts`).
- In the constructor, passes a `send` callback that calls `process.send?.(buffer.toString('base64'))` (line 29).
- Passes an `onMessage` event created by `Event.fromNodeEventEmitter(process, 'message', msg => VSBuffer.wrap(Buffer.from(msg, 'base64')))` (line 32).
- Listens to `process.once('disconnect', ...)` to dispose itself (line 35).
- All IPC data is base64-encoded; the comment on lines 21–23 notes this is a performance concern and documents the intent to migrate to named IPC sockets (`ipc.net`).

**`IIPCOptions` interface** (lines 39–80): Describes the parent-side spawn configuration: `serverName`, `timeout`, `args`, `env`, `debug`, `debugBrk`, `freshExecArgv`, and a `useQueue` flag for `createQueuedSender`.

---

#### 5. `src/vs/platform/utilityProcess/electron-main/utilityProcess.ts` (partial, first 120 lines)

**Role**: Main-process-side launcher for all utility/extension-host child processes using Electron's `utilityProcess` API.

**`IUtilityProcessConfiguration` interface** (lines 22–85): Key fields:
- `entryPoint: string` — the JS module to load (maps to `VSCODE_ESM_ENTRYPOINT`).
- `env?: { [key: string]: string | undefined }` — environment variable override map.
- `parentLifecycleBound?: number` — optional parent PID for the same lifecycle-supervision pattern implemented inside `bootstrap-fork.ts:terminateWhenParentTerminates`.
- `execArgv?: string[]` — passed to the Electron utility process.

**`IWindowUtilityProcessConfiguration`** (lines 87–110): Adds fields for message-port-based communication back to a renderer window: `responseWindowId`, `responseChannel`, `responseNonce`, `windowLifecycleBound`, and `windowLifecycleGraceTime` (a grace period before force-kill on window close).

---

#### 6. `src/vs/base/common/performance.ts` (partial, first 60 lines)

**Role**: Cross-environment performance mark polyfill used to record startup timing.

**`_definePolyfillMarks()`** (lines 8–44): Stores `[name, timestamp]` pairs in a flat `_data` array. Exposes `mark(name, options?)`, `getMarks()`, and `clearMarks(name?)`. When `name` is omitted from `clearMarks`, preserves a `code/timeOrigin` entry if present.

- `bootstrap-fork.ts` calls `performance.mark('code/fork/start')` at line 10, recording the moment the fork process begins execution.

---

### Cross-Cutting Synthesis

`bootstrap-fork.ts` is a thin orchestration file that, before loading any application code, wires together five responsibilities that are each deeply coupled to Node.js and Electron primitives. The IPC layer relies on `process.send` / `process.on('message')` with base64 encoding (`ipc.cp.ts:29-32`). Console and stream interception replaces `Object.defineProperty` descriptors on global objects to forward log data upstream (`bootstrap-fork.ts:105-137`). Parent-process supervision uses POSIX `kill(pid, 0)` polling rather than an OS-level watch (`bootstrap-fork.ts:175`). Crash reporting calls an Electron-exclusive API guarded by runtime type-checking (`bootstrap-fork.ts:188`). ESM module loading uses the Node.js `register()` loader-hook API and dynamic `import()` (`bootstrap-esm.ts:29`, `bootstrap-fork.ts:229`). To port this to a Tauri/Rust host, every one of these five mechanisms would need a corresponding replacement: Tauri's command/event IPC or OS pipes for message passing; Rust `tracing` or logging channels instead of console wrapping; OS-level process supervision (e.g., `prctl(PR_SET_PDEATHSIG)` on Linux or job objects on Windows) instead of kill-signal polling; native crash reporting integration instead of Electron's crashReporter; and Rust's static crate linking instead of dynamic ESM entry-point injection via environment variable.

---

### Out-of-Partition References

The following files are referenced in `bootstrap-fork.ts` or its immediate imports but lie outside partition 51:

- `/Users/norinlavaee/vscode-atomic/src/vs/base/common/performance.ts` — performance mark utility.
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts` — Node.js module path helpers and portable-mode setup.
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-esm.ts` — ESM bootstrap and NLS loader.
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-import.js` — ESM loader hook for dev-time module injection (registered at `bootstrap-node.ts:73`).
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-meta.ts` — Provides `product` and `pkg` globals used in `bootstrap-esm.ts:8`.
- `/Users/norinlavaee/vscode-atomic/src/vs/nls.ts` — `INLSConfiguration` type used in `bootstrap-esm.ts:11`.
- `/Users/norinlavaee/vscode-atomic/src/vs/base/parts/ipc/node/ipc.cp.ts` — Child-process IPC channel server (consumer of the `process.send` infrastructure established in `bootstrap-fork.ts`).
- `/Users/norinlavaee/vscode-atomic/src/vs/platform/utilityProcess/electron-main/utilityProcess.ts` — Main-process spawner that sets the env vars consumed by `bootstrap-fork.ts`.
- `/Users/norinlavaee/vscode-atomic/src/vs/workbench/services/extensions/electron-browser/localProcessExtensionHost.ts` — Sets `VSCODE_ESM_ENTRYPOINT`, `VSCODE_PIPE_LOGGING`, `VSCODE_PARENT_PID`, and `VSCODE_HANDLES_UNCAUGHT_ERRORS` for extension-host forks.
- `/Users/norinlavaee/vscode-atomic/src/vs/platform/terminal/electron-main/electronPtyHostStarter.ts` — Sets the same env vars for the PTY host process.
- `/Users/norinlavaee/vscode-atomic/src/vs/platform/files/node/watcher/watcherClient.ts` — Sets the same env vars for file-watcher child processes.
- `/Users/norinlavaee/vscode-atomic/src/vs/base/common/platform.ts` — `INodeProcess` type referenced in `performance.ts`.
