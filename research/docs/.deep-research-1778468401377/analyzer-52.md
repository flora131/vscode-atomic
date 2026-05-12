### Files Analysed

- `src/bootstrap-node.ts` (190 LOC)

---

### Per-File Notes

#### `src/bootstrap-node.ts`

- **Role:** Module-level bootstrap that runs once when any Node.js/Electron process in VS Code starts. It installs a SIGPIPE guard, normalises the current working directory, and exports three utilities consumed downstream: `devInjectNodeModuleLookupPath` (dev-only module redirection), `removeGlobalNodeJsModuleLookupPaths` (scope-narrowing for the Node module resolver), and `configurePortable` (portable-install detection and environment setup).

- **Key symbols:**
  - `Error.stackTraceLimit = 100` (`src/bootstrap-node.ts:15`) — module-level side effect raising V8 stack-trace depth.
  - `setupCurrentWorkingDirectory` (`src/bootstrap-node.ts:35`) — IIFE-style private function called immediately at line 55.
  - `devInjectNodeModuleLookupPath(injectPath: string): void` (`src/bootstrap-node.ts:62`) — exported; registers a Node ESM loader hook via `Module.register`.
  - `removeGlobalNodeJsModuleLookupPaths(): void` (`src/bootstrap-node.ts:76`) — exported; monkey-patches `Module._resolveLookupPaths` and `Module._nodeModulePaths`.
  - `configurePortable(product: Partial<IProductConfiguration>): { portableDataPath: string; isPortable: boolean }` (`src/bootstrap-node.ts:133`) — exported; returns portable state and mutates env vars.
  - `isWindows` (`src/bootstrap-node.ts:12`) — module-scoped boolean used in `removeGlobalNodeJsModuleLookupPaths` and `configurePortable`.

- **Control flow:**

  1. **SIGPIPE guard** (`src/bootstrap-node.ts:17-30`): If `process.env['VSCODE_HANDLES_SIGPIPE']` is absent, registers a `process.on('SIGPIPE', …)` handler. The handler uses a `didLogAboutSIGPIPE` boolean latch (declared at line 20) so only the first SIGPIPE event logs an error, preventing an infinite async loop caused by a broken console pipe.

  2. **`setupCurrentWorkingDirectory()`** (`src/bootstrap-node.ts:35-53`): If `VSCODE_CWD` env var is not already a string, writes `process.cwd()` into it. On Windows, additionally calls `process.chdir(path.dirname(process.execPath))` to force the process directory to the application folder. The whole body is wrapped in a `try/catch` that logs to stderr.

  3. **`devInjectNodeModuleLookupPath`** (`src/bootstrap-node.ts:62-74`): Returns early when `VSCODE_DEV` is not set. When active, calls `Module.register('./bootstrap-import.js', { parentURL: import.meta.url, data: injectPath })` at line 73 to attach an ESM loader hook that rewrites module resolution to the injected path.

  4. **`removeGlobalNodeJsModuleLookupPaths`** (`src/bootstrap-node.ts:76-128`): Returns early when running inside Electron (`process.versions.electron` is a string). Otherwise:
     - Saves a reference to `Module.globalPaths` and `Module._resolveLookupPaths` (lines 82-84).
     - Replaces `Module._resolveLookupPaths` with a wrapper (lines 86-98) that strips the common suffix of global paths from any returned paths array by counting `commonSuffixLength` from the tail.
     - Replaces `Module._nodeModulePaths` with a wrapper (lines 100-127) that, on Windows only, filters out drive-root directories (paths whose parent ends with `:\`) unless the originating `from` path is itself a drive root, and also filters out the Windows users directory (`HOMEDRIVE` + `HOMEPATH` parent) unless `from` is that directory.

  5. **`configurePortable`** (`src/bootstrap-node.ts:133-190`):
     - `getApplicationPath()` (line 136): Returns the install root depending on platform/product config. Dev mode returns `appRoot` directly; macOS returns three `path.dirname` levels up; Windows versioned-update and Linux return two or three levels up.
     - `getPortableDataPath()` (line 153): If `VSCODE_PORTABLE` env var is set, returns it directly. On Win32/Linux returns `<applicationPath>/data`. On macOS computes a sibling directory named by `product.portable` or `<applicationName>-portable-data`.
     - Checks `isPortable` at line 167: true when the product has no `target` field **and** the `portableDataPath` directory exists on disk (`fs.existsSync`).
     - Checks `isTempPortable` at line 169: true when portable and a `tmp` sub-directory exists inside `portableDataPath`.
     - Mutates environment: sets or deletes `VSCODE_PORTABLE` (lines 171-175); sets `TMP`/`TEMP` (Win32) or `TMPDIR` (others) to `portableTempPath` when `isTempPortable` (lines 177-184).
     - Returns `{ portableDataPath, isPortable }`.

- **Data flow:**
  - `setupCurrentWorkingDirectory` reads `process.env['VSCODE_CWD']`, `process.cwd()`, and `process.execPath`; writes `process.env['VSCODE_CWD']`; may mutate `process.cwd` via `chdir`.
  - `devInjectNodeModuleLookupPath` receives `injectPath` as a string argument and forwards it as loader `data` to the `bootstrap-import.js` ESM hook via `Module.register`.
  - `removeGlobalNodeJsModuleLookupPaths` reads `process.versions.electron`, `Module.globalPaths`, `process.env.HOMEDRIVE`, `process.env.HOMEPATH`; produces no return value; side effect is patched `Module` internals.
  - `configurePortable` receives `product: Partial<IProductConfiguration>` (specifically fields `win32VersionedUpdate`, `portable`, `applicationName`, `target`); reads `import.meta.dirname`, `process.env['VSCODE_DEV']`, `process.env['VSCODE_PORTABLE']`, `process.platform`; calls `fs.existsSync` twice; writes `process.env['VSCODE_PORTABLE']`, `process.env['TMP']`, `process.env['TEMP']`, or `process.env['TMPDIR']`; returns `{ portableDataPath, isPortable }`.

- **Dependencies:**
  - `node:path` (`src/bootstrap-node.ts:6`) — `path.dirname`, `path.join`, `path.relative`.
  - `node:fs` (`src/bootstrap-node.ts:7`) — `fs.existsSync`.
  - `node:module` (`src/bootstrap-node.ts:8`) — `createRequire` (used to create the module-scoped `require` at line 11); `Module` internals accessed dynamically inside functions.
  - `IProductConfiguration` from `src/vs/base/common/product.ts` (`src/bootstrap-node.ts:9`) — type-only import used as the parameter type for `configurePortable`.

---

### Cross-Cutting Synthesis

`src/bootstrap-node.ts` is a thin but critical process-initialisation layer that fires before any application logic. Its design is entirely imperative: three module-level side effects (stack limit, SIGPIPE handler, CWD normalisation) execute unconditionally at import time, while the three exported functions act as configurable, call-site-driven extensions that callers invoke during their own startup sequences. The SIGPIPE guard and CWD setup paper over OS and Electron differences so that all downstream code can rely on stable process state. `devInjectNodeModuleLookupPath` bridges the ESM loader hook mechanism to dev-time source overrides, delegating the actual path rewriting logic to `bootstrap-import.js`. `removeGlobalNodeJsModuleLookupPaths` enforces hermetic module resolution by surgically trimming Node's built-in global search path lists, with additional Windows-specific filtering for drive roots and user directories. `configurePortable` is the most stateful function: it performs two filesystem probes and then commits portable-mode decisions into environment variables that every other subsystem will observe thereafter. All three exported functions accept the fact that they may be called from different process types (renderer, extension host, CLI) and guard themselves appropriately (dev-only checks, Electron detection).

---

### Out-of-Partition References

The following sibling bootstrap files are referenced by or closely related to `src/bootstrap-node.ts` but are covered by other partitions:

- `src/bootstrap-import.ts` — the ESM loader hook registered at `src/bootstrap-node.ts:73` via `Module.register('./bootstrap-import.js', …)`.
- `src/bootstrap-meta.ts` — sibling bootstrap providing `import.meta`-level metadata (out of scope for this partition).
- `src/bootstrap-esm.ts` — ESM entry-point bootstrap (out of scope).
- `src/bootstrap-fork.ts` — fork-process bootstrap (out of scope).
- `src/bootstrap-cli.ts` — CLI-process bootstrap (out of scope).
- `src/bootstrap-server.ts` — server-process bootstrap (out of scope).
- `src/vs/base/common/product.ts:67` — defines `IProductConfiguration`, the interface used as the parameter type of `configurePortable`.
