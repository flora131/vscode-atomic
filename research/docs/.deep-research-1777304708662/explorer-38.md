# Partition 38 of 79 — Findings

## Scope
`src/main.ts/` (1 files, 741 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

| File | LOC | Role |
|------|-----|------|
| `/Users/norinlavaee/vscode-atomic/src/main.ts` | 741 | Electron main entry point — bootstraps the entire process |
| `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts` | 191 | Node.js / portable environment setup, module resolution patching |
| `/Users/norinlavaee/vscode-atomic/src/bootstrap-esm.ts` | 113 | ESM loader hooks, NLS message file loading into `globalThis` |
| `/Users/norinlavaee/vscode-atomic/src/bootstrap-meta.ts` | 55 | Product / package JSON resolution, patched at build time |
| `/Users/norinlavaee/vscode-atomic/src/vs/code/electron-main/main.ts` | 654 | `CodeMain` class — IPC server, service container, `CodeApplication` launch |

---

### Per-File Notes

#### `src/main.ts`

**Performance marks (synchronous, top-level)**
- `code/didStartMain` — line 23, marks the moment `main.ts` is evaluated.
- `code/willLoadMainBundle` / `code/didLoadMainBundle` — lines 25–31, bracket the bundle load time using `performance.timeOrigin`.

**Portable mode (line 34)**
`configurePortable(product)` is called synchronously. It returns `{ portableDataPath, isPortable }` and may set `VSCODE_PORTABLE`, `TMP`/`TEMP`/`TMPDIR` environment variables if a portable `data/tmp` directory exists.

**CLI argument parsing (line 36)**
`parseCLIArgs()` (line 569) calls `minimist(process.argv, ...)` with:
- string keys: `user-data-dir`, `locale`, `js-flags`, `crash-reporter-directory`
- boolean key: `disable-chromium-sandbox`
- default `sandbox: true`; alias `no-sandbox` → `sandbox`

**Sandbox gate (lines 43–54)**
Three branches:
1. If `args['sandbox']` is truthy and neither `--disable-chromium-sandbox` nor the argv.json key is set → `app.enableSandbox()` (line 46).
2. Else if `--no-sandbox` present but not `--disable-gpu-sandbox` → `app.commandLine.appendSwitch('disable-gpu-sandbox')` (line 50).
3. Else → append both `no-sandbox` and `disable-gpu-sandbox` (lines 52–53).

**userData path (lines 57–64)**
`getUserDataPath(args, product.nameShort)` computes the path. On Windows, if the path is on a UNC share, `addUNCHostToAllowlist` is called (lines 59–62). Then `app.setPath('userData', userDataPath)` (line 64) is called before `ready`.

**Code cache path (line 67)**
`getCodeCachePath()` (line 623) returns `undefined` if `--no-cached-data`, `VSCODE_DEV`, or no commit. Otherwise returns `path.join(userDataPath, 'CachedData', commit)`.

**Menu suppression (line 70)**
`Menu.setApplicationMenu(null)` disables Electron's default menu before `ready`.

**Crash reporter (lines 72–85)**
`configureCrashReporter()` (line 449) is called if `--crash-reporter-directory` is present or if `argv.json` has `enable-crash-reporter: true` and `--disable-crash-reporter` is absent.
Inside `configureCrashReporter`:
- If `--crash-reporter-directory` is given, the directory is normalised, validated as absolute (calls `app.exit(1)` lines 457, 465 on failure), created if missing, then `app.setPath('crashDumps', crashReporterDirectory)` (line 472).
- Otherwise reads `product.appCenter` per-platform URL, concatenates `&uid=&iid=&sid=` crash reporter ID, then calls `crashReporter.start(...)` (line 531) with `compress: true`, `ignoreSystemCrashHandler: true`.

**Portable log path (lines 91–93)**
If portable: `app.setAppLogsPath(path.join(userDataPath, 'logs'))` (line 92) forces logs inside the portable directory.

**Custom protocol registration (lines 96–105)**
`protocol.registerSchemesAsPrivileged(...)` is called before `ready` (Electron requirement):
- `vscode-webview`: `standard, secure, supportFetchAPI, corsEnabled, allowServiceWorkers, codeCache`
- `vscode-file`: `secure, standard, supportFetchAPI, corsEnabled, codeCache`

**Global app event listeners (line 108) — `registerListeners()` (line 589)**
- `open-file` (line 597): pushed into `globalThis.macOpenFiles[]` before `ready` — handles macOS file-drop on dock icon.
- `will-finish-launching` (line 612): registers `open-url` handler that pushes URLs into `openUrls[]` and calls `event.preventDefault()`.
- `getOpenUrls` (line 616) is exposed on `globalThis`; when called it removes the `open-url` listener and returns accumulated URLs.

**NLS pre-resolution (lines 115–131)**
`osLocale` is derived from `app.getPreferredSystemLanguages()?.[0]` (line 121), passed through `processZhLocale()` to normalise zh-hans/zh-hant/zh-XY codes (line 660–681). If `userLocale` is defined from argv.json or `--locale`, `resolveNLSConfiguration(...)` is called immediately as a promise (line 124–130) before `ready`.

**Electron locale switch (lines 141–144)**
On Windows / Linux, `app.commandLine.appendSwitch('lang', electronLocale)` is appended; `qps-ploc` (Microsoft Pseudo Language) is mapped to `'en'`.

**`configureCommandlineSwitchesSync` (line 215)**
Reads `argv.json` synchronously via `readArgvConfigSync()` (line 380). Iterates keys:
- `SUPPORTED_ELECTRON_SWITCHES` keys → `app.commandLine.appendSwitch()` or `app.disableHardwareAcceleration()` for `disable-hardware-acceleration` (line 267).
- `password-store: gnome|gnome-keyring` → migrated to `gnome-libsecret` (lines 274–279).
- `SUPPORTED_MAIN_PROCESS_SWITCHES` keys → pushed to `process.argv` (e.g. `--log`, `--use-inmemory-secretstorage`, `--enable-rdp-display-tracking`).
- Chromium `enable-features` (line 328): appends `NetAdapterMaxBufSizeFeature:...,DocumentPolicyIncludeJSCallStacksInCrashReports,EarlyEstablishGpuChannel,EstablishGpuChannelAsync`.
- `disable-features` (line 334): appends `CalculateNativeWinOcclusion`.
- `disable-blink-features` (line 341): appends `FontMatchingCTMigration,StandardizedBrowserZoom`.
- `xdg-portal-required-version` → `'4'` (line 353).
- `max-active-webgl-contexts` → `'32'` (line 357).
- JS flags merged from CLI and argv.json; on Linux adds `--nodecommit_pooled_pages` (line 563).

**`readArgvConfigSync` (line 380)**
Reads `argv.json` via `original-fs` (bypasses Electron's overlay FS). Calls `createDefaultArgvConfigSync` (line 403) if the file does not exist, writing a commented template.

**`app.once('ready', ...)` handler (line 147)**
If `--trace` is given, `contentTracing.startRecording(...)` is called first (line 179), then `onReady()`.

**`onReady()` (line 185)**
- Marks `code/mainAppReady`.
- Concurrently: `mkdirpIgnoreError(codeCachePath)` + `resolveNlsConfiguration()`.
- Then calls `startup(codeCachePath, nlsConfig)`.

**`startup()` (line 203)**
- Sets `VSCODE_NLS_CONFIG` (stringified JSON) and `VSCODE_CODE_CACHE_PATH` in `process.env`.
- Calls `bootstrapESM()` (awaits NLS message load).
- Dynamic `import('./vs/code/electron-main/main.js')` (line 211) loads `CodeMain`.
- Marks `code/didRunMainBundle`.

---

#### `src/bootstrap-node.ts`

- **`Error.stackTraceLimit = 100`** (line 15): increases V8 stack depth.
- **SIGPIPE handler** (lines 17–30): installs a process-level `SIGPIPE` listener that logs once and prevents infinite error loops in broken-pipe scenarios.
- **`setupCurrentWorkingDirectory()`** (line 35): sets `VSCODE_CWD` to `process.cwd()` once; on Windows calls `process.chdir(path.dirname(process.execPath))` to anchor the CWD to the application folder.
- **`devInjectNodeModuleLookupPath(injectPath)`** (line 62): only active under `VSCODE_DEV`; registers an ESM loader hook via `Module.register('./bootstrap-import.js', ...)` to redirect module resolution.
- **`removeGlobalNodeJsModuleLookupPaths()`** (line 76): patches `Module._resolveLookupPaths` and `Module._nodeModulePaths` to strip global and users-dir paths from resolution, narrowing the module search scope.
- **`configurePortable(product)`** (line 133): resolves `portableDataPath`, checks for the `data/` directory's existence, sets `VSCODE_PORTABLE` / temp env vars, returns `{ portableDataPath, isPortable }`.

---

#### `src/bootstrap-esm.ts`

- **`fs` → `original-fs` hook** (lines 14–30): when running inside Electron, registers an in-memory ESM loader that intercepts `import 'fs'` and redirects to `node:original-fs`, avoiding Electron's patched `fs`.
- **Global setup** (lines 33–35): copies `product` and `pkg` into `globalThis._VSCODE_PRODUCT_JSON` / `globalThis._VSCODE_PACKAGE_JSON`; sets `globalThis._VSCODE_FILE_ROOT` to the source directory path.
- **`doSetupNLS()`** (line 49): reads `VSCODE_NLS_CONFIG` from env, determines the `messagesFile` (from `languagePack.messagesFile` or `defaultMessagesFile`), and loads it into `globalThis._VSCODE_NLS_MESSAGES` via `fs.promises.readFile`. On read failure, writes a corrupt marker file and falls back to the default messages file. Marks `code/willLoadNls` and `code/didLoadNls`.
- **`bootstrapESM()`** (line 108): exported async function, sole body is `await setupNLS()`.

---

#### `src/bootstrap-meta.ts`

- **Product JSON** (lines 12–15): `productObj` is a sentinel object with key `BUILD_INSERT_PRODUCT_CONFIGURATION`. At build time this sentinel is replaced with the actual product JSON inline. Out of sources it falls through to `require('../product.json')`.
- **Package JSON** (lines 17–20): same sentinel pattern for `pkgObj`, falls through to `require('../package.json')` out of sources.
- **Embedded app sub-files** (lines 23–44): if `process.isEmbeddedApp`, preserves the parent policy config, then loads `product.sub.json` and `package.sub.json` via `require`, merging them with `Object.assign`.
- **Dev overrides** (lines 46–52): if `VSCODE_DEV`, loads `product.overrides.json` and merges it last.
- Exports `product` and `pkg` (lines 54–55).

---

#### `src/vs/code/electron-main/main.ts` — `CodeMain` class

**Entry (lines 652–654)**
Module-level: instantiates `new CodeMain()` and calls `.main()`.

**`main()` (line 88)**
Calls `this.startup()` inside a try/catch; on error logs to console and calls `app.exit(1)`.

**`startup()` (line 97)**
1. Installs `setUnexpectedErrorHandler` to log errors instead of showing Electron dialog.
2. `createServices()` (line 162) builds a `ServiceCollection` with: `IProductService`, `IEnvironmentMainService`, `ILoggerMainService`, `ILogService` (buffered), `IFileService` + `DiskFileSystemProvider`, `IUriIdentityService`, `IStateService` (DELAYED save strategy), `IUserDataProfilesMainService`, `FileUserDataProvider` for `vscode-user-data` scheme, `IPolicyService` (platform-specific: `NativePolicyService` on Windows/macOS, `FilePolicyService` on Linux, `NullPolicyService` otherwise), `IConfigurationService`, `ILifecycleMainService`, `IRequestService`, `IThemeMainService`, `ISignService`, `ITunnelService`, `IProtocolMainService`.
3. `initServices()` (line 270): in parallel, creates all required directories (`extensionsPath`, `logsHome`, `globalStorageHome`, `workspaceStorageHome`, `localHistoryHome`, `backupHome`), initialises `StateService` and `ConfigurationService`.
4. `claimInstance()` (line 306): attempts `nodeIPCServe(environmentMainService.mainIPCHandle)`. On `EADDRINUSE`, connects as a client (`nodeIPCConnect`) and forwards the current args/env to the existing instance via `ILaunchMainService.start()`, then throws `ExpectedError` to terminate.
5. Writes `mainLockfile` (line 133), wires spdlog logger to the buffer logger.
6. Checks `checkInnoSetupMutex()` on Windows (line 148) — if active, quits immediately.
7. `instantiationService.createInstance(CodeApplication, mainProcessNodeIpcServer, instanceEnvironment).startup()` (line 155) — hands off to `CodeApplication`.

**`resolveArgs()` / `validatePaths()` / `doValidatePaths()` (lines 518–647)**
Parses `process.argv` with `parseMainProcessArgv`, handles `--wait` marker file creation, maps `--chat` sub-args, normalises and sanitises file paths (trims quotes and whitespace on Windows, resolves relative to cwd, deduplicates case-insensitively).

---

### Cross-Cutting Synthesis

`src/main.ts` operates as a two-phase synchronous/asynchronous bootstrap. The synchronous phase runs entirely before Electron's `ready` event: it calls `configurePortable`, parses CLI args, decides on sandbox mode (`app.enableSandbox()` or GPU-sandbox switches), sets `userData` and optionally `crashDumps`/`AppLogs` paths, registers custom URL schemes (`vscode-webview://`, `vscode-file://`) with `protocol.registerSchemesAsPrivileged`, suppresses the default menu, installs macOS open-file/open-url listeners, and fires off an early NLS resolution promise if a user locale is known. The asynchronous phase starts in `app.once('ready')`, forks into optional content tracing, then `onReady()` concurrently creates the code-cache directory and resolves the final NLS configuration. `startup()` injects `VSCODE_NLS_CONFIG` and `VSCODE_CODE_CACHE_PATH` into `process.env`, calls `bootstrapESM()` (which loads NLS messages into `globalThis` and installs the `fs`→`original-fs` hook), and then dynamically imports `vs/code/electron-main/main.js`. That module immediately instantiates `CodeMain`, which builds the full dependency-injection service container, claims the singleton IPC socket (or forwards to an existing instance), and delegates to `CodeApplication`. The four supporting modules form a strict hierarchy: `bootstrap-meta.ts` provides build-time product/package identity; `bootstrap-node.ts` sets up the Node.js environment (CWD, SIGPIPE, portable mode, module path narrowing); `bootstrap-esm.ts` installs the Electron-specific ESM hooks and materialises NLS strings; and `vs/code/electron-main/main.ts` owns all VS Code application services and the single-instance arbitration protocol.

---

### Out-of-Partition References

The following files are directly called from the analysed partition but fall outside its scope:

- `/Users/norinlavaee/vscode-atomic/src/vs/platform/environment/node/userDataPath.ts` — `getUserDataPath()` used at `main.ts:57`
- `/Users/norinlavaee/vscode-atomic/src/vs/base/node/nls.ts` — `resolveNLSConfiguration()` used at `main.ts:124`, `main.ts:717`
- `/Users/norinlavaee/vscode-atomic/src/vs/base/node/unc.ts` — `getUNCHost()`, `addUNCHostToAllowlist()` used at `main.ts:59–61`
- `/Users/norinlavaee/vscode-atomic/src/vs/base/common/performance.ts` — `perf.mark()` called throughout `main.ts`
- `/Users/norinlavaee/vscode-atomic/src/vs/base/common/jsonc.ts` — `parse()` used to read `argv.json` at `main.ts:386`
- `/Users/norinlavaee/vscode-atomic/src/vs/code/electron-main/app.ts` — `CodeApplication` instantiated at `electron-main/main.ts:155`
- `/Users/norinlavaee/vscode-atomic/src/vs/platform/protocol/electron-main/protocolMainService.ts` — `ProtocolMainService` (handles `vscode-file://` / `vscode-webview://` at the renderer layer) at `electron-main/main.ts:248`
- `/Users/norinlavaee/vscode-atomic/src/vs/platform/lifecycle/electron-main/lifecycleMainService.ts` — `LifecycleMainService` registered at `electron-main/main.ts:233`
- `/Users/norinlavaee/vscode-atomic/src/vs/platform/environment/electron-main/environmentMainService.ts` — `EnvironmentMainService` with all path derivations at `electron-main/main.ts:172`
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-import.js` — ESM loader hook data URL target, registered from `bootstrap-node.ts:73`

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Analysis: VS Code Electron Main Process (Partition 38)

## Research Question
What patterns in VS Code's Electron main process would need to be ported to Tauri/Rust for core IDE functionality?

## Analysis Scope
File: `src/main.ts` (741 LOC) — Electron application entry point

---

#### Pattern: App Lifecycle Management with Event-Driven Initialization
**Where:** `src/main.ts:147-183`, `src/main.ts:185-213`
**What:** Electron app waits for 'ready' event before loading main bundle and performing initialization tasks.

```typescript
app.once('ready', function () {
	if (args['trace']) {
		let traceOptions: Electron.TraceConfig | Electron.TraceCategoriesAndOptions;
		if (args['trace-memory-infra']) {
			const customCategories = args['trace-category-filter']?.split(',') || [];
			customCategories.push('disabled-by-default-memory-infra', 'disabled-by-default-memory-infra.v8.code_stats');
			traceOptions = {
				included_categories: customCategories,
				excluded_categories: ['*'],
				memory_dump_config: {
					allowed_dump_modes: ['light', 'detailed'],
					triggers: [
						{
							type: 'periodic_interval',
							mode: 'detailed',
							min_time_between_dumps_ms: 10000
						}
					]
				}
			};
		} else {
			traceOptions = {
				categoryFilter: args['trace-category-filter'] || '*',
				traceOptions: args['trace-options'] || 'record-until-full,enable-sampling'
			};
		}

		contentTracing.startRecording(traceOptions).finally(() => onReady());
	} else {
		onReady();
	}
});

async function onReady() {
	perf.mark('code/mainAppReady');

	try {
		const [, nlsConfig] = await Promise.all([
			mkdirpIgnoreError(codeCachePath),
			resolveNlsConfiguration()
		]);

		await startup(codeCachePath, nlsConfig);
	} catch (error) {
		console.error(error);
	}
}
```

**Variations / call-sites:** 
- `app.once('ready')` at line 147 — single-invocation lifecycle hook
- Nested conditional logic for tracing configuration (lines 148-177)
- Sequential async operations requiring coordination (Promise.all pattern)
- Error handling with console fallback (line 196)

---

#### Pattern: Custom Protocol Registration with Privileges
**Where:** `src/main.ts:96-105`
**What:** Registers custom URI schemes with security privileges before app ready, enabling webview and file protocol handling.

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

**Variations / call-sites:**
- Two custom schemes defined: `vscode-webview` and `vscode-file`
- Each scheme has granular privilege configuration (fetch, CORS, service workers, caching)
- Must be called before app ready event (pre-initialization requirement)

---

#### Pattern: macOS Event Handler Registration with Global State
**Where:** `src/main.ts:589-621`
**What:** Registers macOS-specific file opening and URL scheme handlers that buffer events before app is ready, providing getter functions for deferred access.

```typescript
function registerListeners(): void {
	const macOpenFiles: string[] = [];
	(globalThis as { macOpenFiles?: string[] }).macOpenFiles = macOpenFiles;
	app.on('open-file', function (event, path) {
		macOpenFiles.push(path);
	});

	const openUrls: string[] = [];
	const onOpenUrl =
		function (event: { preventDefault: () => void }, url: string) {
			event.preventDefault();

			openUrls.push(url);
		};

	app.on('will-finish-launching', function () {
		app.on('open-url', onOpenUrl);
	});

	(globalThis as { getOpenUrls?: () => string[] }).getOpenUrls = function () {
		app.removeListener('open-url', onOpenUrl);

		return openUrls;
	};
}
```

**Variations / call-sites:**
- `app.on('open-file')` at line 597 — file drop/open events (macOS)
- `app.on('will-finish-launching')` at line 612 — early app phase hook
- `app.on('open-url')` at line 613 — URI scheme handler registration
- `app.removeListener()` at line 617 — listener cleanup pattern
- Global state attachment for cross-process communication (lines 596, 616)

---

#### Pattern: Crash Reporter Configuration with Platform-Specific URL Building
**Where:** `src/main.ts:449-539`
**What:** Configures crash reporting with platform detection, UUID validation, and dynamic submitURL construction for AppCenter integration.

```typescript
function configureCrashReporter(): void {
	let crashReporterDirectory = args['crash-reporter-directory'];
	let submitURL = '';
	if (crashReporterDirectory) {
		crashReporterDirectory = path.normalize(crashReporterDirectory);

		if (!path.isAbsolute(crashReporterDirectory)) {
			console.error(`The path '${crashReporterDirectory}' specified for --crash-reporter-directory must be absolute.`);
			app.exit(1);
		}

		if (!fs.existsSync(crashReporterDirectory)) {
			try {
				fs.mkdirSync(crashReporterDirectory, { recursive: true });
			} catch (error) {
				console.error(`The path '${crashReporterDirectory}' specified for --crash-reporter-directory does not seem to exist or cannot be created.`);
				app.exit(1);
			}
		}

		app.setPath('crashDumps', crashReporterDirectory);
	} else {
		const appCenter = product.appCenter;
		if (appCenter) {
			const isWindows = (process.platform === 'win32');
			const isLinux = (process.platform === 'linux');
			const isDarwin = (process.platform === 'darwin');
			const crashReporterId = argvConfig['crash-reporter-id'];
			const uuidPattern = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
			if (crashReporterId && uuidPattern.test(crashReporterId)) {
				if (isWindows) {
					switch (process.arch) {
						case 'x64':
							submitURL = appCenter['win32-x64'];
							break;
						case 'arm64':
							submitURL = appCenter['win32-arm64'];
							break;
					}
				}
				submitURL = submitURL.concat('&uid=', crashReporterId, '&iid=', crashReporterId, '&sid=', crashReporterId);
			}
		}
	}

	const productName = (product.crashReporter ? product.crashReporter.productName : undefined) || product.nameShort;
	const companyName = (product.crashReporter ? product.crashReporter.companyName : undefined) || 'Microsoft';
	const uploadToServer = Boolean(!process.env['VSCODE_DEV'] && submitURL && !crashReporterDirectory);
	crashReporter.start({
		companyName,
		productName: process.env['VSCODE_DEV'] ? `${productName} Dev` : productName,
		submitURL,
		uploadToServer,
		compress: true,
		ignoreSystemCrashHandler: true
	});
}
```

**Variations / call-sites:**
- Path validation and directory creation (lines 455-467)
- Platform detection (isWindows, isLinux, isDarwin) at lines 479-481
- Architecture-specific URL mapping (x64 vs arm64) at lines 486-493
- UUID validation with regex pattern at line 483
- Product configuration fallback pattern at lines 528-529
- Environment-based behavior branching (`VSCODE_DEV`) at line 530

---

#### Pattern: Electron Command-Line Switch Configuration with Feature Flags
**Where:** `src/main.ts:215-359`
**What:** Synchronous configuration of Electron/Chromium switches from argv.json before app ready, handling supported switches, feature flags, and JS flags.

```typescript
function configureCommandlineSwitchesSync(cliArgs: NativeParsedArgs) {
	const SUPPORTED_ELECTRON_SWITCHES = [
		'disable-hardware-acceleration',
		'force-color-profile',
		'disable-lcd-text',
		'proxy-bypass-list',
		'remote-debugging-port'
	];

	if (process.platform === 'linux') {
		SUPPORTED_ELECTRON_SWITCHES.push('force-renderer-accessibility');
		SUPPORTED_ELECTRON_SWITCHES.push('password-store');
	}

	const SUPPORTED_MAIN_PROCESS_SWITCHES = [
		'enable-proposed-api',
		'log-level',
		'use-inmemory-secretstorage',
		'enable-rdp-display-tracking',
	];

	const argvConfig = readArgvConfigSync();

	Object.keys(argvConfig).forEach(argvKey => {
		const argvValue = argvConfig[argvKey];

		if (SUPPORTED_ELECTRON_SWITCHES.indexOf(argvKey) !== -1) {
			if (argvValue === true || argvValue === 'true') {
				if (argvKey === 'disable-hardware-acceleration') {
					app.disableHardwareAcceleration();
				} else {
					app.commandLine.appendSwitch(argvKey);
				}
			} else if (typeof argvValue === 'string' && argvValue) {
				app.commandLine.appendSwitch(argvKey, argvValue);
			}
		} else if (SUPPORTED_MAIN_PROCESS_SWITCHES.indexOf(argvKey) !== -1) {
			switch (argvKey) {
				case 'enable-proposed-api':
					if (Array.isArray(argvValue)) {
						argvValue.forEach(id => id && typeof id === 'string' && process.argv.push('--enable-proposed-api', id));
					}
					break;
				// ... more cases
			}
		}
	});

	const featuresToEnable =
		`NetAdapterMaxBufSizeFeature:NetAdapterMaxBufSize/8192,DocumentPolicyIncludeJSCallStacksInCrashReports,EarlyEstablishGpuChannel,EstablishGpuChannelAsync,${app.commandLine.getSwitchValue('enable-features')}`;
	app.commandLine.appendSwitch('enable-features', featuresToEnable);

	const featuresToDisable =
		`CalculateNativeWinOcclusion,${app.commandLine.getSwitchValue('disable-features')}`;
	app.commandLine.appendSwitch('disable-features', featuresToDisable);

	const blinkFeaturesToDisable =
		`FontMatchingCTMigration,StandardizedBrowserZoom,${app.commandLine.getSwitchValue('disable-blink-features')}`;
	app.commandLine.appendSwitch('disable-blink-features', blinkFeaturesToDisable);

	const jsFlags = getJSFlags(cliArgs, argvConfig);
	if (jsFlags) {
		app.commandLine.appendSwitch('js-flags', jsFlags);
	}

	return argvConfig;
}
```

**Variations / call-sites:**
- Static switch allowlist approach (SUPPORTED_ELECTRON_SWITCHES, lines 216-231)
- Platform-conditional switch addition (Linux at lines 233-240)
- Type-specific handling (booleans vs strings) at lines 265-283
- Special case handling for hardware acceleration (line 267 requires explicit method call)
- Feature flag accumulation pattern (lines 327-342) with existing value merge
- Call-site at line 38 during early initialization

---

#### Pattern: Sandbox and GPU Configuration with Platform-Specific Branching
**Where:** `src/main.ts:39-54`
**What:** Enables or disables sandbox based on CLI arguments and configuration, with cascading GPU sandbox logic.

```typescript
if (args['sandbox'] &&
	!args['disable-chromium-sandbox'] &&
	!argvConfig['disable-chromium-sandbox']) {
	app.enableSandbox();
} else if (app.commandLine.hasSwitch('no-sandbox') &&
	!app.commandLine.hasSwitch('disable-gpu-sandbox')) {
	app.commandLine.appendSwitch('disable-gpu-sandbox');
} else {
	app.commandLine.appendSwitch('no-sandbox');
	app.commandLine.appendSwitch('disable-gpu-sandbox');
}
```

**Variations / call-sites:**
- Three-way branching logic for sandbox states (enabled, partial disable, full disable)
- Method call pattern: `app.enableSandbox()` (line 46) vs `app.commandLine.appendSwitch()` (lines 50, 52-53)
- Requires argvConfig availability (from line 38)

---

#### Pattern: User Data Path and Localization Setup with Platform-Specific UNC Handling
**Where:** `src/main.ts:56-64`, `src/main.ts:732-738`
**What:** Sets user data path and handles Windows UNC hosts; resolves user-defined or OS locale with fallback chain.

```typescript
const userDataPath = getUserDataPath(args, product.nameShort ?? 'code-oss-dev');
if (process.platform === 'win32') {
	const userDataUNCHost = getUNCHost(userDataPath);
	if (userDataUNCHost) {
		addUNCHostToAllowlist(userDataUNCHost);
	}
}
app.setPath('userData', userDataPath);
```

**Variations / call-sites:**
- Windows-only UNC host handling at lines 58-62
- `app.setPath('userData')` at line 64 (must occur before app ready)
- Locale resolution fallback chain (user-provided → argv.json → OS → 'en') at lines 732-738

---

## Summary

The `src/main.ts` file demonstrates **7 core patterns** essential for porting VS Code's Electron main process to Tauri/Rust:

1. **Lifecycle Management** — Deferred initialization from event hooks with async coordination
2. **Protocol Registration** — Custom URI schemes with granular privilege configuration
3. **Platform-Native Events** — macOS-specific file/URL handlers with event buffering
4. **Crash Reporting** — Dynamic configuration with platform/architecture detection and UUID validation
5. **CLI/Config Switch Management** — Synchronous configuration of renderer/process switches with feature flags
6. **Sandbox/GPU Configuration** — State-based branching with cascading fallback logic
7. **Path & Localization Setup** — User data directory management with platform-specific (Windows UNC) handling

**Key Porting Challenges:**
- Electron's event-driven lifecycle (especially `app.ready()`) requires reimplementation in Tauri's initialization model
- Custom protocol privileges (`registerSchemesAsPrivileged`) need Tauri equivalents for webview sandboxing
- Platform-specific hooks (`open-file`, `open-url` on macOS) require native interop layers
- Chromium feature flag management is Electron-specific; Tauri would delegate to WebKit/WRY configuration
- Crash reporting integration with AppCenter is Electron/Chromium-dependent
- UNC path allowlisting is Windows-specific security modeling that may not have direct Tauri equivalent

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
