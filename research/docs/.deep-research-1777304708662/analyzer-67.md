# Analyzer 67 — CLI Bootstrap Shim (`src/cli.ts` partition)

## Files Analysed

| File | LOC (read) | Role |
|------|-----------|------|
| `src/cli.ts` | 27 | Top-level CLI entry point; orchestrates every bootstrap phase |
| `src/bootstrap-cli.ts` | 11 | Side-effect-only shim; deletes `VSCODE_CWD` from `process.env` |
| `src/bootstrap-node.ts` | 191 | Node.js runtime configuration helpers (CWD, SIGPIPE, module paths, portable mode) |
| `src/bootstrap-esm.ts` | 113 | ESM loader setup; loads NLS message bundle into `globalThis` |
| `src/bootstrap-meta.ts` | 55 | Reads `product.json` / `package.json` at startup; exports `product` and `pkg` |
| `src/vs/code/node/cli.ts` | 611 | Full CLI dispatcher; handles every `code` sub-command |

---

## Per-File Notes

### `src/cli.ts`

**Role:** Thin async top-level entry that sequences all bootstrap phases and hands off to the real CLI implementation.

**Import order is critical** (`src/cli.ts:6`): `bootstrap-cli.js` is imported first as a pure side-effect module (its single statement deletes `VSCODE_CWD` from the environment). Only then do the remaining imports execute.

**Execution sequence** (all `await`-ed, sequential):

1. `resolveNLSConfiguration` called at line 13 with hard-coded `userLocale: 'en'`, `osLocale: 'en'`, and `product.commit`. The returned object is serialised to JSON and written into `process.env['VSCODE_NLS_CONFIG']` (`src/cli.ts:14`). This env-var is consumed later by `bootstrapESM`.

2. `configurePortable(product)` called at line 17. Passes the product object into the portable-mode logic (see `bootstrap-node.ts` below).

3. `process.env['VSCODE_CLI'] = '1'` set at line 20. This sentinel tells child processes (e.g. extension host) that they were launched from the CLI wrapper rather than Electron directly.

4. `bootstrapESM()` awaited at line 23. This completes NLS message loading into `globalThis._VSCODE_NLS_MESSAGES`.

5. `import('./vs/code/node/cli.js')` at line 26. Dynamic ESM import triggers that module's top-level `main(process.argv)` call (see `vs/code/node/cli.ts:605`).

**Dependencies (out-of-partition):**
- `./vs/base/node/nls.js` — `resolveNLSConfiguration`
- `./vs/code/node/cli.js` — full CLI dispatcher

---

### `src/bootstrap-cli.ts`

**Role:** Single-statement cleanup shim.

`src/bootstrap-cli.ts:11` — `delete process.env['VSCODE_CWD']`. The comment (lines 6–10) explains this prevents a stale `VSCODE_CWD` from leaking into a child process when users run `code .` from a shell where a previous run left the variable set. The file has no exports and no other logic.

---

### `src/bootstrap-node.ts`

**Role:** Node.js process-level configuration called during bootstrap.

**Top-level side effects (executed on import):**

- `Error.stackTraceLimit = 100` at line 15 — increases V8 stack depth from the default of 10.
- SIGPIPE handler registered at lines 21–29 (conditional on `VSCODE_HANDLES_SIGPIPE` env var being absent) to swallow broken-pipe signals without infinite recursion.
- `setupCurrentWorkingDirectory()` called at line 55:
  - Stores `process.cwd()` into `process.env['VSCODE_CWD']` if not already set (line 43). Note: `bootstrap-cli.ts` runs *first* and deletes the value; this re-populates it from the actual cwd of the current process.
  - On Windows, calls `process.chdir(path.dirname(process.execPath))` (line 48) to normalise the working directory to the application folder.

**Exported functions:**

- `devInjectNodeModuleLookupPath(injectPath)` (`src/bootstrap-node.ts:62–74`): Dev-only hook; registers a loader hook via `node:module`'s `register` API pointing at `./bootstrap-import.js` with the `injectPath` as data. No-ops when `VSCODE_DEV` is not set.

- `removeGlobalNodeJsModuleLookupPaths()` (`src/bootstrap-node.ts:76–128`): Patches `Module._resolveLookupPaths` and `Module._nodeModulePaths` to strip global `node_modules` paths from resolution. On Windows additionally removes drive-root paths and the user home directory from the search list (lines 109–126). This function is not called from `src/cli.ts`; it is intended for other entry points (e.g. Electron main process).

- `configurePortable(product)` (`src/bootstrap-node.ts:133–190`): Determines whether portable mode is active and, if so, redirects temp dirs.
  - `getApplicationPath()` (nested, lines 136–151): computes the VS Code app root relative to `import.meta.dirname`, accounting for `darwin` (3 levels up through `.app` bundle) and `win32` with versioned updates (also 3 levels).
  - `getPortableDataPath()` (nested, lines 153–164): resolves the portable data directory. Checks `VSCODE_PORTABLE` env var first; falls back to `<appPath>/data` on Win32/Linux or `<appParent>/<portableDataName>` on macOS.
  - Portability detection (lines 166–169): `isPortable` is `true` when no `target` key is present in `product` AND the portable data path exists on disk.
  - Side effects on `process.env` (lines 171–184): sets `VSCODE_PORTABLE` (or deletes it) and redirects `TMP`/`TEMP`/`TMPDIR` to the portable temp path when applicable.
  - Returns `{ portableDataPath, isPortable }`.

---

### `src/bootstrap-esm.ts`

**Role:** ESM module-loader setup and NLS message hydration.

**Top-level side effects (executed on import):**

- Lines 14–30: When running inside Electron (`ELECTRON_RUN_AS_NODE` or `process.versions['electron']`), registers an inline ESM loader hook via a `data:` URI. The hook intercepts `import 'fs'` and redirects it to `node:original-fs`, ensuring the real filesystem (not Electron's intercepted version) is used.

- Lines 33–35: Populates three globals:
  - `globalThis._VSCODE_PRODUCT_JSON` — shallow copy of `product` from `bootstrap-meta.ts`.
  - `globalThis._VSCODE_PACKAGE_JSON` — shallow copy of `pkg` from `bootstrap-meta.ts`.
  - `globalThis._VSCODE_FILE_ROOT` — `import.meta.dirname`, the directory containing bootstrap files.

**`bootstrapESM()` export** (`src/bootstrap-esm.ts:108–112`): Async function that delegates entirely to `setupNLS()` (lines 41–47), which is a once-only promise wrapper around `doSetupNLS()`.

**`doSetupNLS()`** (`src/bootstrap-esm.ts:49–103`):

1. Marks `code/willLoadNls` performance point (line 50).
2. Reads `VSCODE_NLS_CONFIG` from the environment (set by `src/cli.ts:14`); parses JSON (line 57).
3. Resolves the messages file path: prefers `nlsConfig.languagePack.messagesFile`, falls back to `nlsConfig.defaultMessagesFile` (lines 58–62).
4. Skips NLS loading in dev mode or when no messages file is found (lines 71–75).
5. Reads the messages JSON file with `fs.promises.readFile` and assigns it to `globalThis._VSCODE_NLS_MESSAGES` (line 78).
6. On read error: writes a `corrupted` marker file at `nlsConfig.languagePack.corruptMarkerFile` (lines 83–88) and retries with the default messages file (lines 91–97).
7. Marks `code/didLoadNls` performance point (line 101).

---

### `src/bootstrap-meta.ts`

**Role:** Provides `product` and `pkg` exports containing product configuration and package metadata.

**Build-time patching pattern** (`src/bootstrap-meta.ts:12`): `productObj` is initialised with the sentinel string `BUILD_INSERT_PRODUCT_CONFIGURATION`. During the production build, this entire initialiser is replaced by the actual inlined product JSON. When running from source (sentinel still present), the code falls through to `require('../product.json')` at line 14.

Same pattern applies to `pkgObj` / `BUILD_INSERT_PACKAGE_CONFIGURATION` at lines 17–20.

**Embedded app support** (`src/bootstrap-meta.ts:23–44`): When `process.isEmbeddedApp` is set (non-standard INodeProcess extension), the code:
- Stashes `win32RegValueName`, `darwinBundleIdentifier`, and `urlProtocol` into `productObj.parentPolicyConfig` (lines 26–30) to preserve the host app's policy identity before overrides.
- Deep-merges `product.sub.json` into `productObj`, with special handling to merge the `embedded` key rather than replace it (lines 32–43).
- Merges `package.sub.json` into `pkgObj` (lines 41–43).

**Dev overrides** (`src/bootstrap-meta.ts:46–51`): When `VSCODE_DEV` is set, attempts to load `product.overrides.json` and shallow-merges it over `productObj`.

Exports `product` (line 54) and `pkg` (line 55).

---

### `src/vs/code/node/cli.ts`

**Role:** Implements all CLI sub-commands dispatched by the shim. Entry triggered by the dynamic import at `src/cli.ts:26`; module-level `main(process.argv)` call at line 605.

**`shouldSpawnCliProcess(argv)`** (`src/vs/code/node/cli.ts:33–42`): Returns `true` for extension-management and telemetry arguments (`--install-extension`, `--list-extensions`, `--uninstall-extension`, `--update-extensions`, `--locate-extension`, `--add-mcp`, `--telemetry`, `--install-source`). These require the heavy `cliProcessMain` module.

**`main(argv)`** (`src/vs/code/node/cli.ts:44–595`): Large if/else chain dispatching on parsed `NativeParsedArgs`:

1. **Argument parsing** (lines 48–52): `parseCLIProcessArgv(argv)` from `argv-helper.js`; errors are logged and function returns.

2. **Tunnel / server sub-commands** (lines 54–90): Iterates `NATIVE_CLI_COMMANDS`. When matched, spawns either `cargo run -- <subcommand>` (dev mode, line 74) or the prebuilt `<appPath>/bin/<tunnelApplicationName>` binary (production, line 80). Stdio is `['ignore', 'pipe', 'pipe']`; stdout/stderr are piped to the parent process.

3. **`--help`** (lines 93–96): Prints `buildHelpMessage` using product name/version/options.

4. **`--help` for chat** (lines 98–101): Prints chat-specific help subset.

5. **`--version`** (lines 104–106): Prints `buildVersionMessage`.

6. **`--locate-shell-integration-path`** (lines 109–124): Maps shell name (`bash`, `pwsh`, `zsh`, `fish`) to a script filename and prints its absolute path under `out/vs/workbench/contrib/terminal/common/scripts/`.

7. **Extension management** (`shouldSpawnCliProcess`, lines 127–145): Dynamically imports `./cliProcessMain.js` (path differs in dev vs production, lines 135–139) and calls its `main(args)`.

8. **`--file-write`** (lines 148–219): Reads a JSON arguments file containing `{ source, target }` paths. Validates both paths exist and are absolute files. On Windows, adds UNC host allowlist entries. Copies source to target using `readFileSync`/`writeFileSync`; on Windows uses `truncateSync` + `r+` flag to preserve alternate data streams. Restores `chmod` if `--file-chmod` was passed.

9. **Default — launch Electron app** (lines 223–594): The `else` branch that spawns the VS Code Electron process.
   - Environment built from `process.env` with `ELECTRON_NO_ATTACH_CONSOLE=1` and `ELECTRON_RUN_AS_NODE` deleted (lines 224–229).
   - `--transient` flag (lines 247–264): Creates temp directories for all data dirs and adds their paths to `argv`.
   - Stdin handling (lines 266–329): When `-` is in args and `hasStdinWithoutTty()`, reads stdin into a temp file via `readFromStdin`, then adds the temp file path to `argv` (or `--add-file` for chat mode).
   - `--wait` flag (lines 337–378): Creates a wait-marker file; a `processCallbacks` entry uses `Promise.race` between `whenDeleted(waitMarkerFilePath)`, child `error`, and child `exit` to block until the file is closed.
   - `--prof-startup` (lines 384–480): Finds three free ports; appends `--inspect-brk`, `--remote-debugging-port`, `--inspect-brk-extensions` to argv; starts three `v8-inspect-profiler` sessions (main, extHost, renderer); waits for the renderer to delete the filename-prefix marker; writes `.cpuprofile` files.
   - **Non-macOS spawn** (lines 492–508): Calls `spawn(execToLaunch, argv.slice(2), { detached: true })`. On Windows with `--agents`, resolves a sibling executable via `resolveSiblingWindowsExePath`.
   - **macOS spawn** (lines 509–591): Uses `open -n -g -a <execPath> --args ...argv` (or `-b <bundleIdentifier>` with `--agents`). For verbose/status mode, redirects stdout/stderr to temp files and uses `watchFileContents` to stream them back. All environment variables are forwarded as repeated `--env KEY=VALUE` flags.
   - All `processCallbacks` are awaited in parallel at line 593.

**`getAppRoot()`** (`src/vs/code/node/cli.ts:597–599`): Returns the directory of `FileAccess.asFileUri('')`, i.e. the `resources/app` root.

**`eventuallyExit(code)`** (`src/vs/code/node/cli.ts:601–603`): Wraps `process.exit(code)` in `setTimeout(..., 0)` to allow the event loop to drain before exit.

---

## Cross-Cutting Synthesis

The CLI bootstrap is a strictly ordered, five-phase pipeline expressed as a linear async module. Phase 1 (`bootstrap-cli.ts`) deletes any inherited `VSCODE_CWD`. Phase 2 (`bootstrap-node.ts`) re-establishes CWD from the current process and configures platform-specific portable paths. Phase 3 (`bootstrap-meta.ts`) loads product and package metadata, supporting build-time patching and embedded-app overrides. Phase 4 (`bootstrap-esm.ts`) installs the ESM `fs` redirect hook, populates three `globalThis` values consumed by downstream modules, and hydrates NLS messages from the path encoded in `VSCODE_NLS_CONFIG`. Phase 5 (`vs/code/node/cli.ts`) is the actual command router: for native sub-commands it spawns a prebuilt Rust binary; for extension management it lazily loads `cliProcessMain.js`; for the default case it re-spawns the Electron executable (or `open` on macOS) with a detached child process, forwarding all argv and env. The entire pipeline is built on Node.js ESM `import.meta` APIs, `node:module` loader hooks, and `process.env` as the inter-phase communication channel.

For a Tauri/Rust port, the five phases map directly onto work that must be replicated: (1) env cleanup, (2) portable-path resolution, (3) product-metadata loading, (4) NLS hydration, and (5) the command dispatcher. Phases 1–4 are straightforward Rust equivalents. Phase 5 has the deepest TypeScript/Node.js coupling: `spawn` of Electron, `watchFileContents`, the `v8-inspect-profiler` integration, and the macOS `open` command pattern are all Node.js/Electron-specific surface area that has no direct Tauri equivalent and would require re-architecting.

---

## Out-of-Partition References

The following symbols are imported by the files in this partition but reside outside the `src/cli.ts/` scope boundary:

| Symbol / Module | Source file | Used in |
|-----------------|-------------|---------|
| `resolveNLSConfiguration` | `src/vs/base/node/nls.ts` | `src/cli.ts:9,13` |
| `IProductConfiguration` | `src/vs/base/common/product.ts` | `src/bootstrap-node.ts:9`, `src/bootstrap-meta.ts:7` |
| `INodeProcess` | `src/vs/base/common/platform.ts` | `src/bootstrap-meta.ts:8` |
| `INLSConfiguration` | `src/vs/nls.ts` | `src/bootstrap-esm.ts:11` |
| `performance.mark` | `src/vs/base/common/performance.ts` | `src/bootstrap-esm.ts:10,50,101` |
| Full CLI dispatcher | `src/vs/code/node/cli.ts` | `src/cli.ts:26` (dynamic import) |
| `cliProcessMain` | `src/vs/code/node/cliProcessMain.ts` | `src/vs/code/node/cli.ts:141` (dynamic import) |
| `parseCLIProcessArgv`, `addArg` | `src/vs/platform/environment/node/argvHelper.ts` | `src/vs/code/node/cli.ts:19` |
| `buildHelpMessage`, `buildVersionMessage`, `NATIVE_CLI_COMMANDS`, `OPTIONS` | `src/vs/platform/environment/node/argv.ts` | `src/vs/code/node/cli.ts:18` |
| `hasStdinWithoutTty`, `readFromStdin`, `stdinDataListener`, `getStdinFilePath` | `src/vs/platform/environment/node/stdin.ts` | `src/vs/code/node/cli.ts:20` |
| `createWaitMarkerFileSync` | `src/vs/platform/environment/node/wait.ts` | `src/vs/code/node/cli.ts:21` |
| `product` (singleton) | `src/vs/platform/product/common/product.ts` | `src/vs/code/node/cli.ts:22` |
| `resolveSiblingWindowsExePath` | `src/vs/platform/native/node/siblingApp.ts` | `src/vs/code/node/cli.ts:23` |
| `watchFileContents` | `src/vs/platform/files/node/watcher/nodejs/nodejsWatcherLib.ts` | `src/vs/code/node/cli.ts:16` |
| `FileAccess` | `src/vs/base/common/network.ts` | `src/vs/code/node/cli.ts:27` |
| `whenDeleted`, `writeFileSync` | `src/vs/base/node/pfs.ts` | `src/vs/code/node/cli.ts:14` |
| `findFreePort` | `src/vs/base/node/ports.ts` | `src/vs/code/node/cli.ts:15` |
| `Utils` (profiling) | `src/vs/platform/profiling/common/profiling.ts` | `src/vs/code/node/cli.ts:26` |
| `v8-inspect-profiler` | npm package (external) | `src/vs/code/node/cli.ts:409` |
