# ESM Bootstrap / Loader Strategy - File Locator (Partition 55)

## Implementation

### Core Bootstrap Files
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts` (112 LOC) — ESM module bootstrapping with NLS setup
  - Installs Node.js module resolution hooks to map `fs` to `original-fs` in Electron contexts
  - Sets up global VS Code configuration (`_VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON`, `_VSCODE_FILE_ROOT`)
  - Handles asynchronous NLS (National Language Support) loading from environment configuration
  - Exports `bootstrapESM()` async function for module initialization

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-meta.ts` (31 LOC) — Static metadata loader
  - Loads `product.json` and `package.json` using CommonJS require
  - Handles build-time configuration injection via marker strings
  - Applies product overrides from `product.overrides.json` in dev mode
  - Exports `product` and `pkg` objects globally

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-node.ts` (191 LOC) — Node.js runtime setup
  - Configures working directory handling across platforms (Windows-specific logic)
  - Provides `devInjectNodeModuleLookupPath()` to register custom module resolution hooks
  - Implements `removeGlobalNodeJsModuleLookupPaths()` to restrict module search paths
  - Provides `configurePortable()` for portable VS Code installations
  - Exports utilities for module path injection and global path cleanup

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-import.ts` (102 LOC) — Custom module resolver hook
  - Implements Node.js loader hook protocol (`initialize`, `resolve` functions)
  - Maps package dependencies to node_modules via file URL paths
  - Handles conditional exports (ESM `.import`, fallback to `.main`)
  - Distinguishes module format (ESM vs CommonJS) via `.mjs`, `.cjs`, or `type: "module"` field
  - Redirects specifiers to actual filesystem locations, enabling dev-mode node_modules overrides

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-fork.ts` (230 LOC) — Child process bootstrap
  - Pipes logging from forked process back to parent via `process.send()`
  - Handles uncaught exceptions and unhandled promise rejections
  - Monitors parent process termination and exits accordingly
  - Configures crash reporter for Electron utility processes
  - Chains to `bootstrapESM()` before loading ESM entrypoint specified by `VSCODE_ESM_ENTRYPOINT` env var

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-server.ts` (8 LOC) — Server mode configuration
  - Removes Electron-specific behavior by deleting `ELECTRON_RUN_AS_NODE` environment variable

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-cli.ts` (12 LOC) — CLI mode configuration
  - Cleans up `VSCODE_CWD` environment variable to prevent cross-shell contamination

### Entry Points Using Bootstrap
- `/home/norinlavaee/projects/vscode-atomic/src/main.ts` — Electron main process entry
  - Imports `bootstrapESM()` from `bootstrap-esm.js`
  - Imports `configurePortable()` from `bootstrap-node.js`
  - Imports `product` from `bootstrap-meta.js`

- `/home/norinlavaee/projects/vscode-atomic/src/server-main.ts` — Server/remote mode entry
- `/home/norinlavaee/projects/vscode-atomic/src/server-cli.ts` — Server CLI entry
- `/home/norinlavaee/projects/vscode-atomic/src/cli.ts` — Local CLI entry

### Legacy Loader Infrastructure
- `/home/norinlavaee/projects/vscode-atomic/build/loader.min` — Minified AMD loader
  - Legacy AMD (Asynchronous Module Definition) module system
  - Contains `AMDLoader` module manager for module resolution and loading
  - Supports both Node.js and browser environments
  - Includes script loading via DOM (`<script>` tags) or Node.js `vm.Script`
  - Handles cached data for V8 code caching in Node.js
  - Supports plugins and custom resolution paths

## Configuration

### Environment Variables Used in Bootstrap
- `ELECTRON_RUN_AS_NODE` — Triggers fs → original-fs mapping in bootstrap-esm.ts
- `VSCODE_NLS_CONFIG` — JSON config for NLS setup (languagePack, defaultMessagesFile, resolvedLanguage)
- `VSCODE_DEV` — Development mode flag (disables NLS, enables product overrides)
- `VSCODE_PORTABLE` — Path to portable data directory
- `VSCODE_CWD` — Cached current working directory for consistent lookups
- `VSCODE_HANDLES_SIGPIPE` — Signal handling configuration
- `VSCODE_PARENT_PID` — Parent process monitoring for forked processes
- `VSCODE_PIPE_LOGGING` — Enable logging via parent process pipe
- `VSCODE_HANDLES_UNCAUGHT_ERRORS` — Error handling delegation
- `VSCODE_CRASH_REPORTER_PROCESS_TYPE` — Crash reporter configuration
- `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH` — Dev mode module path override
- `VSCODE_ESM_ENTRYPOINT` — Dynamic ESM module entry point for forked processes
- `VSCODE_VERBOSE_LOGGING` — Verbose console logging flag

### Build-Time Markers
- `BUILD_INSERT_PRODUCT_CONFIGURATION` — Replaced during build with actual product.json
- `BUILD_INSERT_PACKAGE_CONFIGURATION` — Replaced during build with actual package.json

### Runtime Globals Injected by Bootstrap
- `globalThis._VSCODE_PRODUCT_JSON` — Immutable product configuration object
- `globalThis._VSCODE_PACKAGE_JSON` — Immutable package metadata
- `globalThis._VSCODE_FILE_ROOT` — Current module's directory (via `import.meta.dirname`)
- `globalThis._VSCODE_NLS_LANGUAGE` — Resolved language for NLS
- `globalThis._VSCODE_NLS_MESSAGES` — Loaded translation messages object

## Types / Interfaces

### IProductConfiguration (referenced in bootstrap-meta.ts)
- Located in `src/vs/base/common/product.js`
- Used for type annotations in TypeScript bootstrap files

### INLSConfiguration (referenced in bootstrap-esm.ts)
- Located in `src/vs/nls.js`
- Defines NLS config structure with languagePack, defaultMessagesFile, resolvedLanguage

## Notable Clusters

### Module Resolution Layers
1. **ESM Module System** (bootstrap-esm.ts) — Top-level ESM entry, fs mapping
2. **Node.js Hook System** (bootstrap-import.ts) — Custom loader hook for dev mode
3. **Node Module Paths** (bootstrap-node.ts) — Global and local path manipulation
4. **AMD Loader** (loader.min) — Legacy fallback for web/AMD modules

### Process Initialization Sequence
1. `main.ts` / `server-main.ts` / `cli.ts` — Entry point
2. `bootstrap-meta.ts` — Load static configuration
3. `bootstrap-node.ts` — Configure working directory and portable mode
4. `bootstrap-esm.ts` — Set up NLS and global configuration
5. `bootstrap-fork.ts` (if child process) — Set up logging and error handling
6. Dynamic entrypoint — Load application code

### NLS (Internationalization) Pipeline
- `VSCODE_NLS_CONFIG` env var parsed
- `bootstrap-esm.ts::setupNLS()` reads language pack from disk
- `_VSCODE_NLS_MESSAGES` populated with translations
- Fallback to default English messages on errors
- Corruption marker file written on load failure for cache invalidation

### Portable Mode Support
- `bootstrap-node.ts::configurePortable()` detects portable installations
- Redirects temp directory (`TMP`, `TEMP`, `TMPDIR`) to portable data folder
- Sets `VSCODE_PORTABLE` environment variable

---

## Portability Notes for Tauri/Rust Replacement

The ESM bootstrap and loader strategy handles several critical concerns that a Tauri/Rust port would need to address:

### 1. Module Resolution
The current implementation uses Node.js ESM loader hooks (`register()`) to dynamically intercept module resolution. Tauri/Rust would need to:
- Pre-resolve all module paths at build time (static linking)
- Replace dynamic `devInjectNodeModuleLookupPath` with build-time configuration
- Eliminate the need for runtime module hooks via bundling

### 2. Global Configuration Injection
Currently done via `globalThis` mutations in synchronous bootstrap code. Rust would need to:
- Expose configuration through a structured API instead of globals
- Ensure NLS configuration is loaded before application code runs
- Consider pre-loading translations at compile time

### 3. NLS (Localization) Loading
Async file I/O pattern reading language packs. Rust implementation should:
- Load translations at binary startup before running JS/TS code
- Consider embedding translations in the binary or using memory-mapped files
- Handle corruption detection differently (not via marker files)

### 4. Process Management & Forking
`bootstrap-fork.ts` handles child process setup including logging redirection and error handling. Rust would:
- Use native threading or subprocess APIs instead of Node.js forking
- Replace `process.send()` piping with native IPC
- Simplify exception handling (no Promise rejections in Rust)

### 5. Portable Mode Detection
Runtime filesystem checks. Rust should:
- Detect portable mode at binary startup
- Use environment variables or configuration files instead of disk probing
- Pre-compute paths at initialization

### 6. fs → original-fs Mapping
Electron-specific workaround for fs access. Rust implementation:
- Directly access filesystem without need for mapping
- May need wrapper abstractions for file access in some contexts

The loader is essentially a compatibility layer for ES modules in a Node.js/Electron environment. A Tauri/Rust rewrite would simplify this significantly since Rust has native module support and more direct filesystem/process control.
