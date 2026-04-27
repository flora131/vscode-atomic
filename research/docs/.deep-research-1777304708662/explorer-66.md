# Partition 66 of 79 — Findings

## Scope
`src/server-cli.ts/` (1 files, 30 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator Report: Partition 66 — Server CLI (Tauri/Rust Port Research)

## Scope
- `src/server-cli.ts` (1 file, 30 LOC)

## Implementation
- `src/server-cli.ts` — Bootstrap entry point for VS Code server mode; loads Node.js runtime configuration (NLS, ESM), module paths, and delegates to core server implementation (`vs/server/node/server.cli.js`)

## Summary
The single file in this partition is `src/server-cli.ts`, a thin TypeScript bootstrapper for VS Code's server CLI mode. It handles:
- Node.js global state initialization via `bootstrap-server.js`
- National Language Support (NLS) configuration
- Development environment path injection for module resolution
- ES Module bootstrapping
- Delegation to the actual server CLI implementation

**Relevance to Tauri/Rust port:** This file is a Node.js runtime initialization shim. In a Tauri/Rust port, the entire bootstrap and runtime initialization sequence would be replaced with Rust initialization code. This represents the JavaScript side of process startup logic that would need architectural redesign for a compiled Rust target. The server CLI module itself (`vs/server/node/server.cli.js`) is outside this partition's scope but would be the critical piece containing the actual server logic needing porting.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

1. `/Users/norinlavaee/vscode-atomic/src/server-cli.ts` — Top-level bootstrap entry point for VS Code server CLI mode (30 LOC)
2. `/Users/norinlavaee/vscode-atomic/src/bootstrap-server.ts` — Pre-bootstrap global state guard (1 meaningful LOC)
3. `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts` — Node.js environment setup, module path injection, portable mode (191 LOC)
4. `/Users/norinlavaee/vscode-atomic/src/bootstrap-esm.ts` — ESM module loader with NLS setup and optional Electron `fs` redirect hook (113 LOC)
5. `/Users/norinlavaee/vscode-atomic/src/bootstrap-meta.ts` — Product and package JSON loader (56 LOC)
6. `/Users/norinlavaee/vscode-atomic/src/bootstrap-import.ts` — Node.js loader hook for redirecting node_modules in dev mode (102 LOC)
7. `/Users/norinlavaee/vscode-atomic/src/vs/server/node/server.cli.ts` — Actual server CLI implementation handling pipe/command dispatch (525 LOC)

---

### Per-File Notes

#### 1. `src/server-cli.ts` (30 LOC) — Bootstrap Entry Point

This is the top-level entry point for VS Code's "server CLI" mode. It is an ES module (`import.meta.dirname` is used) and is expected to be executed directly by Node.js. The sequence of operations is strictly ordered:

- **Line 6**: `import './bootstrap-server.js'` — must come first; this modifies global state (specifically deletes `process.env['ELECTRON_RUN_AS_NODE']`).
- **Line 8**: Imports `devInjectNodeModuleLookupPath` from `bootstrap-node.js`.
- **Line 9**: Imports `bootstrapESM` from `bootstrap-esm.js`.
- **Line 10**: Imports `resolveNLSConfiguration` from `vs/base/node/nls.js`.
- **Line 11**: Imports `product` from `bootstrap-meta.js`.

**NLS initialization (lines 14–15)**: Calls `resolveNLSConfiguration` with `userLocale: 'en'`, `osLocale: 'en'`, `product.commit`, empty `userDataPath`, and `import.meta.dirname` as `nlsMetadataPath`. The resulting config is serialized to `process.env['VSCODE_NLS_CONFIG']` so that `bootstrap-esm` can read NLS messages during its own setup.

**Dev mode path injection (lines 17–24)**: If `process.env['VSCODE_DEV']` is set, it sets `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH` to `<src_dir>/../remote/node_modules` (so that Node.js-compiled native modules are found instead of Electron ones) and calls `devInjectNodeModuleLookupPath()`. In production, this env variable is deleted.

**ESM bootstrap (line 27)**: `await bootstrapESM()` — loads NLS messages into `globalThis._VSCODE_NLS_MESSAGES` and registers an optional `fs → original-fs` module hook for Electron.

**Delegation (line 30)**: `await import('./vs/server/node/server.cli.js')` — hands over execution to the actual server CLI implementation. This dynamic import means the implementation module's top-level code runs at that point.

---

#### 2. `src/bootstrap-server.ts` (7 LOC) — Global State Guard

Located at `src/bootstrap-server.ts`. Its entire functional content is one line:

- **Line 7**: `delete process.env['ELECTRON_RUN_AS_NODE']`

This prevents `bootstrap-esm.ts` from registering the `fs → original-fs` redirect hook (which is only needed when running inside Electron), ensuring the server-mode bootstrap runs as a pure Node.js process.

---

#### 3. `src/bootstrap-node.ts` (191 LOC) — Node.js Environment Setup

This file sets up the broader Node.js runtime environment. Key responsibilities:

- **Lines 17–30 (`SIGPIPE` handler)**: Registers a `process.on('SIGPIPE', ...)` handler unless `VSCODE_HANDLES_SIGPIPE` is set — a workaround for an Electron bug where `console.error` in a SIGPIPE handler can cause infinite loops.

- **Lines 35–55 (`setupCurrentWorkingDirectory`)**: Stores the original `process.cwd()` in `VSCODE_CWD` env variable; on Windows, calls `process.chdir(path.dirname(process.execPath))` to standardize the working directory. Called immediately at line 55.

- **Lines 62–74 (`devInjectNodeModuleLookupPath`)**: Registers the `./bootstrap-import.js` module hook via `node:module`'s `Module.register()`, passing `injectPath` as data. This is only active in dev mode (`VSCODE_DEV` env var must be set). The loader hook (`bootstrap-import.ts`) intercepts module resolution and redirects named packages to files inside `remote/node_modules`.

- **Lines 76–128 (`removeGlobalNodeJsModuleLookupPaths`)**: Patches `Module._resolveLookupPaths` and `Module._nodeModulePaths` to strip global Node.js search paths (and Windows-specific drive/user paths). Not called from `server-cli.ts` directly.

- **Lines 133–190 (`configurePortable`)**: Sets up portable mode — reads `VSCODE_PORTABLE` env or derives a path from the application location; if the portable data dir exists, sets `VSCODE_PORTABLE` env and optionally redirects `TMP`/`TEMP`/`TMPDIR`. Not called from `server-cli.ts` directly.

---

#### 4. `src/bootstrap-esm.ts` (113 LOC) — ESM Loader and NLS Setup

- **Lines 13–30**: If running inside Electron (`ELECTRON_RUN_AS_NODE` or `process.versions.electron`), registers an inline ESM loader hook via `node:module`'s `register()` with a `data:` URL containing JavaScript that maps the `fs` specifier to `node:original-fs`. This ensures the unpatched original Node.js `fs` is used rather than Electron's override. Because `bootstrap-server.ts` deletes `ELECTRON_RUN_AS_NODE` before this runs, the server CLI path skips this hook entirely.

- **Lines 33–36**: Sets three globals from product/package metadata: `globalThis._VSCODE_PRODUCT_JSON`, `globalThis._VSCODE_PACKAGE_JSON`, and `globalThis._VSCODE_FILE_ROOT` (set to `import.meta.dirname`). These are the global anchors used throughout VS Code's runtime for product identification.

- **Lines 39–104 (NLS helpers)**: `setupNLS()` (line 41) is a singleton wrapper around `doSetupNLS()` (line 49). `doSetupNLS` reads `process.env['VSCODE_NLS_CONFIG']` (the JSON set by `server-cli.ts` at line 15), extracts a `messagesFile` path, reads it with `fs.promises.readFile`, and stores the parsed messages in `globalThis._VSCODE_NLS_MESSAGES` (line 78). On error it optionally writes a `corruptMarkerFile` and falls back to `nlsConfig.defaultMessagesFile`. In dev mode (`VSCODE_DEV`) or if no `messagesFile` is found, returns `undefined` without loading any NLS messages.

- **Lines 108–112 (`bootstrapESM`)**: The single export. Awaits `setupNLS()`.

---

#### 5. `src/bootstrap-meta.ts` (56 LOC) — Product/Package JSON Loading

- **Lines 12–15**: `productObj` is initialized with a sentinel `BUILD_INSERT_PRODUCT_CONFIGURATION` property. At build time the build system patches this to inline the real product configuration; when running from source (the sentinel survives), it falls back to `require('../product.json')`.

- **Lines 17–20**: Same pattern for `pkgObj` / `package.json`.

- **Lines 23–44**: For embedded apps (`process.isEmbeddedApp`), reads `product.sub.json` and `package.sub.json`, merging sub-objects and preserving the parent policy config (for `win32RegValueName`, `darwinBundleIdentifier`, `urlProtocol` at lines 27–31).

- **Lines 46–52**: In dev mode, attempts to load `product.overrides.json` and merges it into `productObj`.

- **Lines 54–55**: Exports `product` and `pkg`.

---

#### 6. `src/bootstrap-import.ts` (102 LOC) — Node.js Module Hook (Dev Mode Redirector)

This file implements a Node.js module loader hook (ESM hooks API). It is registered via `Module.register()` in `bootstrap-node.ts:73`.

- **Lines 22–85 (`initialize`)**: Called once by the Node.js loader infrastructure when the hook is registered. Reads the `package.json` at `injectPath/../package.json`, iterates over its `dependencies`, and for each dependency reads its `package.json` to determine the entry-point file (checking `exports["."].import`, `exports["."].default`, and `main` fields in priority order). Populates `_specifierToUrl` (mapping package name → absolute file URL) and `_specifierToFormat` (mapping package name → `'module'` or `'commonjs'` based on `.mjs`/`.cjs` suffix or `type` field).

- **Lines 87–101 (`resolve`)**: The hook function called for every module resolution. If `specifier` matches a key in `_specifierToUrl`, returns the pre-computed URL and format with `shortCircuit: true`. Otherwise defers to `nextResolve`.

---

#### 7. `src/vs/server/node/server.cli.ts` (525 LOC) — Server CLI Implementation

This is the actual VS Code CLI logic delegated to from `server-cli.ts:30`. It is a standalone CLI program for operating VS Code from a remote terminal.

**Environment detection (lines 86–90)**: Reads four environment variables at module load time:
- `VSCODE_IPC_HOOK_CLI` → `cliPipe` — a Unix domain socket path for pipe-mode communication
- `VSCODE_CLIENT_COMMAND` → `cliCommand` — path to the VS Code executable (WSL/Windows mode)
- `VSCODE_CLIENT_COMMAND_CWD` → `cliCommandCwd`
- `VSCODE_CLI_AUTHORITY` → `cliRemoteAuthority`
- `VSCODE_STDIN_FILE_PATH` → `cliStdInFilePath`

**Option filtering (lines 39–84)**: Two allow-lists determine which CLI options are valid for each transport:
- `isSupportedForCmd` (lines 39–53) — blocks server-only options like `user-data-dir`, `extensions-dir`, `telemetry` when running via `VSCODE_CLIENT_COMMAND`
- `isSupportedForPipe` (lines 55–84) — only allows `version`, `help`, `folder-uri`, `file-uri`, `add`, `diff`, `merge`, `wait`, `goto`, `reuse-window`, `new-window`, `status`, `install-extension`, `uninstall-extension`, `update-extensions`, `list-extensions`, `force`, `verbose`, `remote`, `locate-shell-integration-path` when running via `VSCODE_IPC_HOOK_CLI`

**`main()` function (lines 92–373)**: Entry point invoked at line 522 with `productName`, `version`, `commit`, `executableName` from `process.argv`.

1. **Lines 93–96**: Exits immediately if neither `cliPipe` nor `cliCommand` is set (not in a VS Code terminal or WSL).
2. **Lines 98–126**: Builds the filtered options object; constructs an `ErrorReporter` that logs to console.
3. **Lines 127–129**: Calls `parseArgs(args, options, errorReporter)` to get `parsedArgs`. Sets `mapFileUri` to `mapFileToRemoteUri` if `cliRemoteAuthority` is set.
4. **Lines 132–155**: Handles `--help`, `--version`, and `--locate-shell-integration-path` flags early-exit. For shell integration, it computes the script path under `out/vs/workbench/contrib/terminal/common/scripts/` (line 153).
5. **Lines 163–214**: Processes path arguments. `translatePath()` (lines 490–511) resolves each path relative to `preferredCwd` and calls `fs.lstatSync(fs.realpathSync(input))` to classify as file or directory URI. Handles stdin (`-` argument) by calling `readFromStdin()` to write stdin content to a temp file.
6. **Lines 230–301 (cliCommand branch)**: When `VSCODE_CLIENT_COMMAND` is set (WSL scenario):
   - Extension management commands (install/uninstall/list/update) fork `server-main` via `cp.fork(FileAccess.asFileUri('server-main').fsPath, cmdLine, { stdio: 'inherit' })` (line 245).
   - Reconstructs a command-line array from `parsedArgs` (lines 250–264).
   - For `.bat`/`.cmd` extensions spawns `cmd.exe /C` (lines 275–276).
   - For other executables (Electron binary), sets `ELECTRON_RUN_AS_NODE=1` and spawns the binary pointing at `cli.js` (lines 280–300). Special-cases WSL2 (`runningInWSL2()`) to pipe stdout/stderr explicitly (lines 295–297).
7. **Lines 302–372 (cliPipe branch)**: When `VSCODE_IPC_HOOK_CLI` is set (integrated terminal scenario), communicates with the running VS Code server via HTTP over a Unix domain socket.
   - `parsedArgs.status` → sends `{ type: 'status' }` (lines 303–312)
   - Extension management → sends `{ type: 'extensionManagement', list, install, uninstall, force }` (lines 314–327)
   - File/folder open → sends `{ type: 'open', fileURIs, folderURIs, diffMode, mergeMode, addMode, gotoLineMode, forceReuseWindow, forceNewWindow, waitMarkerFilePath, remoteAuthority }` (lines 338–353)

**`sendToPipe()` (lines 415–469)**: Sends a JSON-serialized `PipeCommand` object via `http.request()` to the Unix domain socket at `cliPipe`. The socket path is set as `socketPath` in `http.RequestOptions` (line 429). Collects response chunks, parses JSON, resolves or rejects the promise based on `statusCode`.

**`mapFileToRemoteUri()` (line 513–515)**: Replaces `file://` prefix with `vscode-remote://<cliRemoteAuthority>` in URIs when `VSCODE_CLI_AUTHORITY` is set.

**`runningInWSL2()` (lines 375–384)**: Checks `WSL_DISTRO_NAME` env and runs `uname -r` to detect if the kernel version contains `-microsoft-`.

**Top-level invocation (line 522)**: `const [, , productName, version, commit, executableName, ...remainingArgs] = process.argv` — extracts product metadata passed as positional argv entries by the caller.

---

### Cross-Cutting Synthesis

`src/server-cli.ts` is a pure sequencing harness: it applies Node.js global state preconditions (via `bootstrap-server.ts` and `bootstrap-node.ts`), configures the NLS subsystem (serializing config into `VSCODE_NLS_CONFIG` before ESM loading so that `bootstrap-esm.ts` can load locale-specific message bundles into `globalThis._VSCODE_NLS_MESSAGES`), conditionally registers a dev-mode module redirector (via `bootstrap-import.ts`) pointing at `remote/node_modules` for Node.js-compiled native binaries, bootstraps the ESM runtime, then dynamically imports the actual implementation (`server.cli.ts`). The actual CLI in `server.cli.ts` is entirely Node.js-based and communicates with a running VS Code desktop instance either through a Unix domain socket HTTP API (`VSCODE_IPC_HOOK_CLI`) or by spawning the VS Code Electron binary with `ELECTRON_RUN_AS_NODE=1` (`VSCODE_CLIENT_COMMAND`). There is no Rust, no Tauri, and no native bindings in this partition; the entire stack depends on Node.js `child_process`, `http`, `fs`, and `node:module` loader hooks.

For a Tauri/Rust port, this bootstrap chain represents a substantial boundary: the NLS system, ESM loader hooks, Unix domain socket IPC, and `child_process` spawning are all Node.js-specific mechanisms that would need equivalent replacements in Rust/Tauri (e.g., Tauri's IPC system, Rust's `std::process::Command`, a Rust NLS crate, and native dynamic loading rather than Node.js's `module.register()`).

---

### Out-of-Partition References

The following files are referenced from within this partition's files but reside outside the partition scope:

- `src/vs/base/node/nls.ts` — `resolveNLSConfiguration()` used at `server-cli.ts:14`
- `src/vs/base/common/performance.ts` — `performance.mark()` called in `bootstrap-esm.ts:50,101`
- `src/vs/nls.ts` — `INLSConfiguration` type used in `bootstrap-esm.ts:12`
- `src/vs/base/common/product.ts` — `IProductConfiguration` type used in `bootstrap-node.ts:9`, `bootstrap-meta.ts:7`
- `src/vs/base/common/platform.ts` — `INodeProcess` type used in `bootstrap-meta.ts:8`
- `src/vs/platform/environment/node/argv.ts` — `parseArgs`, `buildHelpMessage`, `buildVersionMessage`, `OPTIONS`, `OptionDescriptions`, `ErrorReporter` used in `server.cli.ts:12`
- `src/vs/platform/environment/common/argv.ts` — `NativeParsedArgs` type used in `server.cli.ts:13`
- `src/vs/platform/environment/node/wait.ts` — `createWaitMarkerFileSync` used in `server.cli.ts:14`
- `src/vs/workbench/api/node/extHostCLIServer.ts` — `PipeCommand` type used in `server.cli.ts:15`
- `src/vs/platform/environment/node/stdin.ts` — `hasStdinWithoutTty`, `getStdinFilePath`, `readFromStdin` used in `server.cli.ts:16`
- `src/vs/base/common/async.ts` — `DeferredPromise` used in `server.cli.ts:17`
- `src/vs/base/common/network.ts` — `FileAccess` used in `server.cli.ts:18`
- `src/vs/base/common/process.ts` — `cwd` used in `server.cli.ts:10`
- `src/vs/base/common/path.ts` — `dirname`, `extname`, `resolve`, `join` used in `server.cli.ts:11`
- `../product.json` — loaded by `bootstrap-meta.ts:14` when running from source
- `../package.json` — loaded by `bootstrap-meta.ts:19` when running from source
- `../product.overrides.json` — loaded by `bootstrap-meta.ts:49` in dev mode
- `../product.sub.json` / `../package.sub.json` — loaded by `bootstrap-meta.ts:33,41` for embedded apps

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: VS Code Core IDE Functionality Port Analysis

## Scope Analysis
File: `src/server-cli.ts` (30 LOC bootstrap shim)

This file is a minimal server-side CLI entry point for VS Code, primarily demonstrating initialization and module-loading patterns rather than core IDE feature implementations. The scope is explicitly a bootstrap shim with limited direct implementation of core IDE features (editing, language intelligence, debugging, source control, terminal, navigation).

## Patterns Found

#### Pattern: Sequential Bootstrap with Initialization Order Enforcement
**Where:** `src/server-cli.ts:6-14`
**What:** Strict ordering of initialization steps with comments enforcing prerequisites.
```typescript
import './bootstrap-server.js'; // this MUST come before other imports as it changes global state
import { join } from 'node:path';
import { devInjectNodeModuleLookupPath } from './bootstrap-node.js';
import { bootstrapESM } from './bootstrap-esm.js';
import { resolveNLSConfiguration } from './vs/base/node/nls.js';
import { product } from './bootstrap-meta.js';

const nlsConfiguration = await resolveNLSConfiguration({ 
  userLocale: 'en', 
  osLocale: 'en', 
  commit: product.commit, 
  userDataPath: '', 
  nlsMetadataPath: import.meta.dirname 
});
process.env['VSCODE_NLS_CONFIG'] = JSON.stringify(nlsConfiguration);
```
**Variations / call-sites:** Environment variable `VSCODE_NLS_CONFIG` is set after NLS resolution; order-dependent initialization is critical for state setup.

#### Pattern: Conditional Dev-Mode Path Resolution
**Where:** `src/server-cli.ts:17-24`
**What:** Environment-dependent module path injection for development vs. production builds.
```typescript
if (process.env['VSCODE_DEV']) {
  process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH'] = 
    process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH'] || 
    join(import.meta.dirname, '..', 'remote', 'node_modules');
  devInjectNodeModuleLookupPath(process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH']);
} else {
  delete process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH'];
}
```
**Variations / call-sites:** Fallback to remote node_modules when in dev mode; cleanup in production.

#### Pattern: Lazy Async Module Loading via Dynamic Import
**Where:** `src/server-cli.ts:27-30`
**What:** Top-level await with deferred actual server module loading after bootstrap completion.
```typescript
// Bootstrap ESM
await bootstrapESM();

// Load Server
await import('./vs/server/node/server.cli.js');
```
**Variations / call-sites:** Two-phase initialization: bootstrap setup, then actual server code import.

#### Pattern: NLS (Internationalization) Configuration Pre-Setup
**Where:** `src/server-cli.ts:10-15`
**What:** Localization framework initialization with metadata path resolution before main module loads.
```typescript
import { resolveNLSConfiguration } from './vs/base/node/nls.js';
import { product } from './bootstrap-meta.js';

const nlsConfiguration = await resolveNLSConfiguration({ 
  userLocale: 'en', 
  osLocale: 'en', 
  commit: product.commit, 
  userDataPath: '', 
  nlsMetadataPath: import.meta.dirname 
});
```
**Variations / call-sites:** Configuration passed via environment variable for downstream modules; import.meta.dirname used for module-relative paths.

#### Pattern: Global State Initialization Guard
**Where:** `src/server-cli.ts:6`
**What:** Single synchronous import with explicit side-effect comment indicating global state mutation precondition.
```typescript
import './bootstrap-server.js'; // this MUST come before other imports as it changes global state
```
**Variations / call-sites:** Blocking synchronous import required to establish baseline state before all other operations.

## Summary

The scope file (`src/server-cli.ts`) is a minimal 30-line bootstrap shim that demonstrates **initialization orchestration patterns** rather than core IDE feature implementations. Patterns present include:

1. **Strict initialization ordering** with global state preconditions
2. **Conditional dev/prod path handling** for module resolution
3. **Top-level async/await for deferred module loading**
4. **Pre-setup of cross-cutting concerns** (NLS/i18n) before core modules load
5. **Environment variable-based configuration** for runtime behavior

**Critical limitation:** This scope does not contain implementations of core VS Code IDE features (editing, language intelligence, debugging, source control, terminal, navigation). Those would be located in referenced modules like `./vs/server/node/server.cli.js` (not in scope). The file serves as a **entry point orchestrator** rather than a feature implementation reference.

To fully address the research question about porting core IDE functionality, investigation would need to extend to the actual server module and its subcomponents, which are outside the current scope.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
