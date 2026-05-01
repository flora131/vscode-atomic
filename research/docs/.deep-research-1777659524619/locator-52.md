# Bootstrap and Global Runtime Contract - Partition 52

## Overview
This document maps the global hooks and runtime initialization chain defined in `src/bootstrap-node.ts` and related bootstrap files. These establish critical runtime contracts that would require fundamental re-engineering for a Tauri/Rust port.

### Implementation

- `src/bootstrap-node.ts` — Core node process initialization: SIGPIPE handling, stack trace limits, CWD setup, module lookup path manipulation, and portable mode configuration
- `src/bootstrap-esm.ts` — ESM-level bootstrap: loads product/package metadata, NLS initialization, registers module resolution hooks for `fs` → `original-fs` mapping in Electron
- `src/bootstrap-import.ts` — Module loader hook for dev-mode node_modules redirection; parses package.json exports and resolves ESM/CJS format
- `src/bootstrap-fork.ts` — Subprocess bootstrap: configures crash reporter, logging pipes, exception handling, parent process monitoring, environment validation
- `src/bootstrap-cli.ts` — Deletes `VSCODE_CWD` early to prevent shell environment variable leakage
- `src/bootstrap-meta.ts` — Loads product.json and package.json configuration, handles embedded app overrides and dev overrides
- `src/bootstrap-server.ts` — Minimal stub that prevents Electron flag propagation to server mode

### Type Definitions / Configuration

- `src/typings/vscode-globals-product.d.ts` — Type contract for `globalThis._VSCODE_FILE_ROOT`, `_VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON`, `_VSCODE_CSS_LOAD`, `_VSCODE_DISABLE_CSS_IMPORT_MAP`, `_VSCODE_USE_RELATIVE_IMPORTS`
- `src/typings/vscode-globals-nls.d.ts` — Type contract for `globalThis._VSCODE_NLS_MESSAGES` (string array) and `globalThis._VSCODE_NLS_LANGUAGE` used by all NLS localization across electron main, renderer, utility processes, Node.js, browser, and web workers

### Notable Clusters / Dependencies

Files that depend on bootstrap-established globals (13 core consumers):
- `src/vs/platform/product/common/product.ts` — Reads `_VSCODE_PRODUCT_JSON` and `_VSCODE_PACKAGE_JSON` to establish product configuration; checks for native sandbox context first, then globalThis fallback
- `src/vs/nls.ts` — Accesses `_VSCODE_NLS_MESSAGES` and `_VSCODE_NLS_LANGUAGE` to resolve localization strings
- `src/vs/base/common/network.ts` — Uses `_VSCODE_FILE_ROOT` for module path resolution
- `src/vs/amdX.ts` — Legacy AMD loader uses `_VSCODE_PRODUCT_JSON` for configuration
- `src/vs/platform/agentHost/node/agentHostServerMain.ts` — Manually sets `_VSCODE_FILE_ROOT` for non-bootstrap contexts
- `src/vs/platform/webWorker/browser/webWorkerServiceImpl.ts` — Injects NLS globals and file root into web worker scope
- `src/vs/code/electron-browser/workbench/workbench.ts` — Sets NLS and file root from sandbox configuration
- `src/vs/sessions/electron-browser/sessions.ts` — Updates NLS globals during session configuration
- `src/vs/workbench/services/extensions/browser/webWorkerExtensionHost.ts` — Passes file root to web worker extension host
- Worker/iframe contexts (3 files) — All inject NLS and file root globals into isolated scopes

### Key Initialization Chain

1. **Entry**: `main.ts` or `server-main.ts` imports bootstrap chain
2. **bootstrap-node.ts loads first**:
   - Sets `Error.stackTraceLimit = 100`
   - Registers SIGPIPE handler (Electron/Node.js compatibility)
   - Runs `setupCurrentWorkingDirectory()` → manages `VSCODE_CWD` environment variable
   - Exports `devInjectNodeModuleLookupPath()` and `removeGlobalNodeJsModuleLookupPaths()` for module hooking
   - Exports `configurePortable()` to set `VSCODE_PORTABLE` and temp paths
3. **bootstrap-esm.ts runs next**:
   - Registers module resolution hook via `node:module#register()` (data URI with base64 encoded hook)
   - Sets `globalThis._VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON`, `_VSCODE_FILE_ROOT`
   - Loads NLS configuration from `VSCODE_NLS_CONFIG` env var or file
   - Sets `globalThis._VSCODE_NLS_MESSAGES` and `_VSCODE_NLS_LANGUAGE`
4. **bootstrap-fork.ts** (subprocess only):
   - Wraps console methods for logging pipes to parent process
   - Configures crash reporter via Electron API
   - Sets up exception handlers and parent process lifecycle monitoring
5. **At runtime**: All code accesses pre-initialized globals without re-initialization

---

## Porting Implications for Tauri/Rust

Porting VS Code's bootstrap layer to Tauri/Rust would require:

1. **Signal Handling**: Replace `process.on('SIGPIPE')` with Rust signal handlers (via `signal-hook` crate or OS syscalls)

2. **Module System Replacement**: The entire node ESM loader hook chain (`Module.register`, loader initialization hooks) must be replaced with Rust-side symbol resolution and FFI boundary management

3. **Global State Management**: Instead of `globalThis` (JavaScript object), establish equivalent in Rust via:
   - Static or thread-local storage for product/NLS metadata
   - IPC mechanism to distribute globals across Rust→WebView bridge
   - Serialization format for product.json and NLS message arrays

4. **Environment/Path Management**: Emulate `VSCODE_CWD`, `VSCODE_PORTABLE`, `VSCODE_NLS_CONFIG` using Rust environment and path utilities; coordinate between main process and renderer

5. **Process Lifecycle**: Replace Node.js subprocess hooking with Tauri's `tauri::api::process::Command` or native child process management; implement logging pipes and parent termination detection in Rust

6. **Localization Pipeline**: Redesign NLS message injection—currently done via `globalThis._VSCODE_NLS_MESSAGES` array populated by bootstrap, would need Rust → WebView message passing with proper serialization

7. **Portable Mode**: Reimplement the `configurePortable()` logic for Tauri's data directory structure and temp path management

The bootstrap layer is deeply integrated with Node.js/Electron process model and cannot be trivially ported; it represents a significant architectural change point for any Rust migration.
