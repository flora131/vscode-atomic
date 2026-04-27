# Electron to Tauri/Rust Port Analysis: src/main.ts

## Overview
This document catalogs the Electron-based main process entry point and the 46+ electron-main dependent files that would require porting to achieve a TypeScript/Electron → Tauri/Rust architecture migration. The analysis focuses on custom protocol handlers, app lifecycle management, and critical abstractions.

---

## Implementation

### Main Entry Point
- `src/main.ts` — Electron app initialization, CLI argument parsing, custom protocol registration (vscode-webview://, vscode-file://), crash reporting, NLS configuration, sandbox enablement

### Bootstrap & Initialization
- `src/bootstrap-node.ts` — Node.js runtime setup, SIGPIPE handling, working directory management, module loader hooks for dev mode
- `src/bootstrap-esm.ts` — ES modules bootstrap
- `src/bootstrap-meta.ts` — Product configuration and package metadata loading from product.json/product.sub.json
- `src/vs/code/electron-main/main.ts` — CodeMain class, service instantiation, app readiness coordination
- `src/vs/code/electron-main/app.ts` — CodeApplication class, 150+ lines of Electron API imports, service orchestration

### Protocol & IPC
- `src/vs/platform/protocol/electron-main/protocolMainService.ts` — Handles vscode-file:// and file:// protocol interception, manages valid file roots, request validation
- `src/vs/platform/protocol/electron-main/protocol.ts` — IProtocolMainService interface definition, IIPCObjectUrl contract

### Electron Lifecycle & Event Handling
- `src/vs/platform/lifecycle/electron-main/lifecycleMainService.ts` — Main app lifecycle phases (willShutdown, shouldShutdown, willClose), signal handling, exit coordination

### Window Management
- `src/vs/platform/windows/electron-main/windows.ts` — IWindowsMainService, getAllWindowsExcludingOffscreen, OpenContext enum
- `src/vs/platform/windows/electron-main/windowsMainService.ts` — Electron BrowserWindow management
- `src/vs/platform/windows/electron-main/windowImpl.ts` — Window implementation details
- `src/vs/platform/windows/electron-main/windowsFinder.ts` — Window lookup and matching logic
- `src/vs/platform/windows/electron-main/windowsStateHandler.ts` — Window state persistence

### IPC Infrastructure (46 files across partitions)
- `src/vs/base/parts/ipc/electron-main/ipc.electron.ts` — Electron main ↔ renderer channel
- `src/vs/base/parts/ipc/electron-main/ipc.mp.ts` — MessagePort-based IPC
- `src/vs/base/parts/ipc/electron-main/ipcMain.ts` — validatedIpcMain for secure message routing
- `src/vs/base/parts/ipc/common/ipc.electron.ts` — Shared Electron IPC types
- `src/vs/base/parts/ipc/node/ipc.net.ts` — Node.js socket-based IPC (for shared process)
- `src/vs/base/parts/ipc/node/ipc.cp.ts` — Child process communication

### Core Services (Platform Abstractions)
- `src/vs/platform/environment/electron-main/environmentMainService.ts` — Environment variables, paths, argument parsing
- `src/vs/platform/native/electron-main/nativeHostMainService.ts` — Native OS integration
- `src/vs/platform/native/electron-main/auth.ts` — Proxy authentication
- `src/vs/platform/dialogs/electron-main/dialogMainService.ts` — File/message dialogs
- `src/vs/platform/url/electron-main/electronUrlListener.ts` — URL protocol handler (vscode://)
- `src/vs/platform/webview/electron-main/webviewMainService.ts` — Webview management
- `src/vs/platform/webview/electron-main/webviewProtocolProvider.ts` — Custom webview protocol (vscode-webview://)

### Window Lifecycle & UI
- `src/vs/platform/window/electron-main/window.ts` — ICodeWindow interface
- `src/vs/platform/menubar/electron-main/menubar.ts` — MenubarMainService, native menu handling
- `src/vs/platform/menubar/electron-main/menubarMainService.ts` — macOS/Windows menu coordination
- `src/vs/base/parts/contextmenu/electron-main/contextmenu.ts` — Context menu registration

### Storage & Configuration
- `src/vs/platform/storage/electron-main/storageMainService.ts` — Application/global/workspace storage
- `src/vs/platform/storage/electron-main/storageIpc.ts` — StorageDatabaseChannel
- `src/vs/platform/state/node/state.ts` — IStateService interface
- `src/vs/platform/backup/electron-main/backup.ts` — Backup/recovery service

### Update & System Integration
- `src/vs/platform/update/electron-main/abstractUpdateService.ts` — Cross-platform update base
- `src/vs/platform/update/electron-main/updateService.darwin.ts` — macOS updater
- `src/vs/platform/update/electron-main/updateService.linux.ts` — Linux updater
- `src/vs/platform/update/electron-main/updateService.win32.ts` — Windows updater
- `src/vs/platform/update/electron-main/updateService.snap.ts` — Snap updater
- `src/vs/platform/update/electron-main/crossAppUpdateIpc.ts` — Code/Agents cross-app coordination
- `src/vs/platform/secrets/electron-main/macOSCrossAppSecretSharing.ts` — Keychain integration

### Process Management
- `src/vs/platform/extensions/electron-main/extensionHostStarter.ts` — Extension host process spawning
- `src/vs/platform/terminal/electron-main/electronPtyHostStarter.ts` — Terminal pty process
- `src/vs/platform/agentHost/electron-main/electronAgentHostStarter.ts` — Agent process starter
- `src/vs/platform/utilityProcess/electron-main/utilityProcess.ts` — Utility process API
- `src/vs/platform/sharedProcess/electron-main/sharedProcess.ts` — Shared singleton process

### Debugging & Diagnostics
- `src/vs/platform/diagnostics/electron-main/diagnosticsMainService.ts` — GPU info, system diagnostics
- `src/vs/platform/debug/electron-main/extensionHostDebugIpc.ts` — Extension debug broadcast
- `src/vs/platform/profiling/electron-main/windowProfiling.ts` — Window performance profiling

### File System & Resources
- `src/vs/platform/files/electron-main/diskFileSystemProviderServer.ts` — DiskFileSystemProviderChannel
- `src/vs/platform/launch/electron-main/launchMainService.ts` — Open file/folder handlers
- `src/vs/platform/workspaces/electron-main/workspacesMainService.ts` — Workspace storage
- `src/vs/platform/workspaces/electron-main/workspacesManagementMainService.ts` — Workspace CRUD
- `src/vs/platform/workspaces/electron-main/workspacesHistoryMainService.ts` — Recent workspaces

### Auxiliary/Specialized Services
- `src/vs/platform/auxiliaryWindow/electron-main/auxiliaryWindows.ts` — IAuxiliaryWindowsMainService
- `src/vs/platform/auxiliaryWindow/electron-main/auxiliaryWindowsMainService.ts` — Secondary window coordination
- `src/vs/platform/browserView/electron-main/browserViewMainService.ts` — BrowserView embeddings
- `src/vs/platform/browserView/electron-main/browserSession.ts` — Session-level security
- `src/vs/platform/webContentExtractor/electron-main/webPageLoader.ts` — Web content fetching
- `src/vs/platform/externalTerminal/electron-main/externalTerminal.ts` — External terminal launching

### Modern Features
- `src/vs/platform/mcp/node/mcpGatewayService.ts` — Model Context Protocol gateway
- `src/vs/platform/mcp/node/nativeMcpDiscoveryHelperService.ts` — MCP discovery
- `src/vs/platform/crossAppIpc/electron-main/crossAppIpcService.ts` — Cross-process communication
- `src/vs/platform/networkFilter/common/networkFilterService.ts` — Agent network isolation

### Logging & Telemetry
- `src/vs/platform/log/electron-main/loggerService.ts` — ILoggerMainService
- `src/vs/platform/log/electron-main/logIpc.ts` — LoggerChannel
- `src/vs/platform/telemetry/electron-main/telemetryUtils.ts` — Machine ID, telemetry resolution
- `src/vs/platform/telemetry/electron-main/errorTelemetry.js` — Error telemetry handler

### Sandbox & Security
- `src/vs/base/parts/sandbox/electron-browser/preload.ts` — Preload script injections
- `src/vs/base/parts/sandbox/electron-browser/preload-aux.ts` — Auxiliary window preload
- `src/vs/platform/sandbox/electron-main/sandboxHelperService.ts` — Sandbox configuration

---

## Tests

- `src/vs/platform/environment/test/electron-main/environmentMainService.test.ts`
- `src/vs/platform/storage/test/electron-main/storageMainService.test.ts`
- `src/vs/platform/workspaces/test/electron-main/workspacesManagementMainService.test.ts`
- `src/vs/platform/browserView/test/electron-main/browserSessionTrust.test.ts`
- `src/vs/platform/webContentExtractor/test/electron-main/webPageLoader.test.ts`
- `src/vs/base/parts/ipc/test/electron-browser/ipc.mp.test.ts`
- `src/vs/platform/test/electron-main/workbenchTestServices.ts`

---

## Configuration

- `product.json` — App metadata (appCenter, darwinBundleIdentifier, urlProtocol, etc.)
- `package.json` — Build/runtime dependencies
- `argv.json` — Runtime switches (disable-hardware-acceleration, log-level, etc.) persisted in userData

---

## Notable Clusters

### Electron App Lifecycle Calls in src/main.ts
```
app.enableSandbox()              (line 46)
app.setPath('userData', ...)     (line 64)
app.setAppLogsPath(...)          (line 92)
app.getPreferredSystemLanguages() (line 121)
app.once('ready', ...)           (line 147)
app.commandLine.appendSwitch()   (lines 50, 143, 269, etc.)
app.disableHardwareAcceleration() (line 267)
app.setPath('crashDumps', ...)   (line 472)
app.getLocale()                  (line 700)
app.exit()                       (lines 457, 465)
```

### Custom Protocol Registration in src/main.ts (lines 96–105)
```typescript
protocol.registerSchemesAsPrivileged([
  {
    scheme: 'vscode-webview',
    privileges: { standard: true, secure: true, supportFetchAPI: true, corsEnabled: true, allowServiceWorkers: true, codeCache: true }
  },
  {
    scheme: 'vscode-file',
    privileges: { secure: true, standard: true, supportFetchAPI: true, corsEnabled: true, codeCache: true }
  }
]);
```

### Protocol Handlers in electron-main/protocolMainService.ts
- `defaultSession.protocol.registerFileProtocol(vscode-file://)` — Maps vscode-file:// URLs to validated filesystem paths
- `defaultSession.protocol.interceptFileProtocol(file://)` — Blocks direct file:// access, prevents security bypass

### Event Listeners in src/main.ts (lines 589–621)
```
app.on('open-file', ...)        — macOS drag-drop files
app.on('will-finish-launching', ...) — Early startup hooks
app.on('open-url', ...)         — vscode:// protocol invocation
app.removeListener('open-url', ...) — Cleanup after collection
```

### File 46+ Electron Imports
**core/lifecycle:** app, protocol, crashReporter, session, powerMonitor, systemPreferences, dialog
**per-platform:** Details (GPU), GPUFeatureStatus, WebFrameMain

---

## Porting Implications

1. **Protocol Handlers** — vscode-file:// and vscode-webview:// are custom Electron protocol handlers that must be reimplemented in Tauri as custom command handlers or invoke system.
2. **IPC Abstraction** — Electron's ipcMain/ipcRenderer must map to Tauri's invoke()/listen() or a bridge pattern.
3. **Window Lifecycle** — BrowserWindow creation, sandboxing, preload injection require Tauri Window API and Rust backend.
4. **Main Process API** — Electron APIs (app, protocol, session, powerMonitor) map to Tauri plugins or Rust backend services.
5. **Sandbox/Preload** — Electron's preload.ts scripts must become Tauri JavaScript injections or Rust-side validation.
6. **Process Management** — Spawning extension hosts, pty hosts, shared processes requires Tauri's Command API or external process spawning.
7. **Event Broadcasting** — Electron event listeners become Tauri listener patterns; lifecycle hooks map to Tauri plugin initialization.

---

## Summary

The `src/main.ts` entry point and accompanying 46+ electron-main files implement VS Code's Electron main process, encompassing app lifecycle, custom protocol handlers, IPC infrastructure, and 20+ platform-specific services. A Tauri port requires rewriting:
- **Protocol handlers** (vscode-file, vscode-webview, vscode://) as Tauri commands or custom URI schemes
- **IPC layer** (electron-main ipc.*.ts) using Tauri invoke/listen patterns
- **App lifecycle** (app.once, app.on, protocol.register) via Tauri initialization and plugin system
- **Platform services** (menus, dialogs, storage, updates, extensions) as Tauri plugins + Rust backend
- **Process spawning** (extension host, pty, shared process) via Tauri Command or std::process APIs

No existing Tauri equivalents found in this codebase partition.

