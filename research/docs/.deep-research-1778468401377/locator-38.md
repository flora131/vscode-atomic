# Locator 38: Electron Main Process Bootstrap (src/main.ts)

## Implementation

- `src/main.ts` — Electron application entry point (741 LOC). Primary bootstrap for VS Code's main process with core Electron app lifecycle management, command-line argument parsing, sandbox configuration, crash reporting, and NLS initialization. Critical Electron→Tauri transition seam.

### Key Bootstrap Functions
- `configurePortable()` — Portable mode configuration
- `parseCLIArgs()` — Command-line argument processing
- `configureCommandlineSwitchesSync()` — Electron flags and feature toggles
- `onReady()` — Main Electron app ready handler
- `startup()` — ESM module bootstrap and main electron-main module load
- `registerListeners()` — macOS file/URL event handlers
- `configureCrashReporter()` — Telemetry crash reporting setup

### Related Implementation Files
- `src/vs/code/electron-main/main.ts` — Entry point loaded after ESM bootstrap (line 211)
- `src/vs/code/electron-main/app.ts` — Main application controller with service instantiation (40 imported services)
- `src/vs/platform/windows/electron-main/windowImpl.ts` — BrowserWindow wrapper and window lifecycle
- `src/vs/platform/windows/electron-main/windowsMainService.ts` — Window management service
- `src/vs/platform/windows/electron-main/windows.ts` — Window state validation and defaults
- `src/vs/platform/windows/electron-main/windowsFinder.ts` — Window lookup utilities
- `src/vs/platform/windows/electron-main/windowsStateHandler.ts` — Window persistence

### Bootstrap Modules
- `src/bootstrap-esm.ts` — ESM loader setup
- `src/bootstrap-node.ts` — Node portable mode configuration
- `src/bootstrap-meta.ts` — Product metadata loading
- `src/bootstrap-server.ts` — Server bootstrap (if needed)
- `src/bootstrap-fork.ts` — Child process fork bootstrapping
- `src/bootstrap-cli.ts` — CLI mode initialization
- `src/bootstrap-import.ts` — Import-related bootstrap

## Electron→Tauri Seam Points

### 1. App Lifecycle (Lines 147-182)
```
app.once('ready', function () { ... })
```
Controls Electron app initialization, optional content tracing, and startup sequencing. Tauri equivalent: `tauri::Builder::new()` and app event handlers.

### 2. Protocol Registration (Lines 96-105)
```
protocol.registerSchemesAsPrivileged([...])
```
Registers custom URI schemes (`vscode-webview`, `vscode-file`) with privileges for fetch/CORS/service workers. Tauri requires custom protocol handlers via `tauri::plugin::Builder`.

### 3. Menu Configuration (Line 70)
```
Menu.setApplicationMenu(null)
```
Disables native menu. Tauri needs explicit menu setup via `tauri::menu::Menu` or custom implementations.

### 4. Sandbox & GPU Settings (Lines 43-54)
```
app.enableSandbox()
app.commandLine.appendSwitch('...')
```
Controls Chromium sandbox and GPU acceleration. Tauri's webview configuration handles equivalent security settings.

### 5. Crash Reporter (Lines 531-538)
```
crashReporter.start({ companyName, productName, submitURL, uploadToServer, ... })
```
Telemetry crash reporting. Tauri has no built-in equivalent; requires custom crash handling implementation.

### 6. File & URL Event Listeners (Lines 589-621)
```
app.on('open-file', ...)
app.on('open-url', ...)
app.on('will-finish-launching', ...)
```
macOS file/URL association handling. Tauri requires custom deep-link handlers.

### 7. Window Creation (Line 211, delegated to app.ts)
Actual BrowserWindow creation happens in:
- `src/vs/platform/windows/electron-main/windowImpl.ts` — BaseWindow class wrapping BrowserWindow
- Window creation options defined in `src/vs/platform/windows/electron-main/windows.ts` → `defaultBrowserWindowOptions`

## Configuration

- `argv.json` — Runtime configuration (user data path, hardware acceleration, crash reporting, secret storage, etc.). Generated at `${userDataPath}/${dataFolderName}/argv.json` with safe defaults.
- Environment variables:
  - `VSCODE_PORTABLE` — Portable installation mode
  - `VSCODE_NLS_CONFIG` — NLS configuration (set line 204)
  - `VSCODE_CODE_CACHE_PATH` — V8 code cache directory (set line 205)
  - `VSCODE_DEV` — Development mode flag

## Types / Interfaces

- `IArgvConfig` (lines 362-378) — Runtime configuration schema
- `IWindowCreationOptions` (src/vs/platform/windows/electron-main/windowImpl.ts:51-56)
- `INativeWindowConfiguration` (imported from platform/window/common/window.js)
- `ICodeWindow` (platform/window/electron-main/window.js) — Window abstraction

## Notable Clusters

### Window Management Services (~5 files)
- `src/vs/platform/windows/electron-main/` — Complete window lifecycle, state persistence, and restoration for multi-window support.

### Main Process Services (app.ts imports ~40 services)
- Backup, configuration, dialogs, encryption, file system, extensions, keyboard, launch, lifecycle, logging, menubar, native host, protocol, storage, telemetry, update, URL handling, webview, and workspace services.

### Bootstrap Chain
1. `src/main.ts` — Electron app entry point (this file)
2. `src/bootstrap-esm.ts` — ESM module loader
3. `src/vs/code/electron-main/main.ts` — Actual application startup (imported line 211)
4. `src/vs/code/electron-main/app.ts` — Service instantiation and initialization

## Summary

`src/main.ts` is the critical Electron→Tauri seam, handling Electron app initialization, command-line parsing, sandbox/security configuration, crash reporting, and delegation to the main application service layer. Key hardpoints for porting:

1. **App Lifecycle**: `app.once('ready')` → Tauri event system
2. **Custom Protocols**: `protocol.registerSchemesAsPrivileged()` → Tauri protocol plugin
3. **Crash Reporting**: Electron's crashReporter → Custom implementation needed
4. **macOS Deep Links**: `app.on('open-file/url')` → Tauri deep-link handlers
5. **Window Creation**: Deferred to platform/windows/ services; BrowserWindow instances replaced with Tauri webview handles
6. **Configuration**: argv.json schema preserved; injected via environment or startup params

The actual window creation code lives in `src/vs/platform/windows/electron-main/windowImpl.ts` and related services, where BrowserWindow objects are instantiated and managed. Tauri port requires wrapping this entire subsystem.
