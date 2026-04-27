# VS Code Core IDE Functionality Porting Research - File Location Index (Partition 1)

## Scope
Searching `src/vs/` (5906 files, 1.9M LOC) for files relevant to porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust. Focus on partition 1 seeds: DI services, Disposable lifecycle, Electron IPC, and Node process couplings.

---

## Implementation

### Dependency Injection & Service Architecture
- `src/vs/platform/instantiation/common/instantiation.ts` — Core DI system, `createDecorator`, `IInstantiationService`, service registration
- `src/vs/platform/instantiation/common/serviceCollection.ts` — Service collection management for DI
- `src/vs/platform/instantiation/common/descriptors.js` — Service descriptors for instantiation
- `src/vs/platform/instantiation/common/extensions.ts` — DI extensions, `registerSingleton` patterns
- `src/vs/platform/ipc/common/services.ts` — IPC-specific service definitions

### Lifecycle Management (Disposable Pattern)
- `src/vs/base/common/lifecycle.ts` — Core `IDisposable`, `Disposable`, `DisposableStore` lifecycle patterns, leak tracking
- `src/vs/platform/lifecycle/common/lifecycle.ts` — Platform-level lifecycle management service interface
- `src/vs/workbench/services/lifecycle/common/lifecycle.ts` — Workbench lifecycle service
- `src/vs/platform/lifecycle/electron-main/lifecycleMainService.ts` — Electron main process lifecycle hooks
- `src/vs/workbench/services/lifecycle/electron-browser/lifecycleService.ts` — Electron renderer lifecycle

### Electron IPC Layer
- `src/vs/base/parts/ipc/common/ipc.ts` — Core IPC protocol, channel abstraction
- `src/vs/base/parts/ipc/common/ipc.electron.ts` — Electron-specific IPC types
- `src/vs/base/parts/ipc/common/ipc.mp.ts` — Message port IPC
- `src/vs/base/parts/ipc/common/ipc.net.ts` — Network-based IPC
- `src/vs/base/parts/ipc/electron-main/ipcMain.ts` — Main process IPC registration
- `src/vs/base/parts/ipc/electron-main/ipc.electron.ts` — Main process Electron IPC
- `src/vs/base/parts/ipc/electron-main/ipc.mp.ts` — Main process message port IPC
- `src/vs/base/parts/ipc/electron-browser/ipc.electron.ts` — Renderer Electron IPC
- `src/vs/base/parts/ipc/electron-browser/ipc.mp.ts` — Renderer message port IPC
- `src/vs/base/parts/ipc/node/ipc.cp.ts` — Child process IPC for Node
- `src/vs/base/parts/ipc/node/ipc.mp.ts` — Node message port IPC
- `src/vs/base/parts/ipc/node/ipc.net.ts` — Node network IPC

### Sandbox & Preload (Electron Context Isolation)
- `src/vs/base/parts/sandbox/electron-browser/preload.ts` — Main preload script with contextBridge bindings
- `src/vs/base/parts/sandbox/electron-browser/preload-aux.ts` — Auxiliary window preload
- `src/vs/base/parts/sandbox/electron-browser/globals.ts` — Global context setup in sandbox
- `src/vs/base/parts/sandbox/electron-browser/electronTypes.ts` — Electron type definitions for sandbox
- `src/vs/base/parts/sandbox/common/electronTypes.ts` — Common Electron type interfaces

### Entry Points & App Initialization
- `src/vs/code/electron-main/main.ts` — Main Electron entry point
- `src/vs/code/electron-main/app.ts` — Electron app bootstrap, process.argv, environment setup
- `src/vs/code/electron-browser/workbench/workbench.ts` — Renderer workbench entry
- `src/vs/code/browser/workbench/workbench.ts` — Web workbench entry
- `src/vs/code/node/cli.ts` — CLI entry point (Node couplings)
- `src/vs/code/node/cliProcessMain.ts` — CLI process bootstrap
- `src/vs/code/electron-utility/sharedProcess/sharedProcessMain.ts` — Shared utility process

### Platform Services (Node/Electron-specific)
- `src/vs/platform/environment/electron-main/environmentMainService.ts` — Electron env, process.env, app paths
- `src/vs/platform/environment/node/userDataPath.ts` — Node fs-based user data paths
- `src/vs/platform/environment/node/argvHelper.ts` — process.argv parsing
- `src/vs/platform/storage/electron-main/storageMainService.ts` — Electron storage with IPC
- `src/vs/platform/storage/electron-main/storageIpc.ts` — Storage IPC channel
- `src/vs/platform/native/common/native.ts` — Native bindings interface
- `src/vs/platform/native/electron-main/nativeHostMainService.ts` — Native host main (macOS specific)
- `src/vs/platform/process/electron-main/processMainService.ts` — Electron process management
- `src/vs/platform/launch/electron-main/launchMainService.ts` — App launch service

### File System & Watcher Services
- `src/vs/platform/files/common/fileService.ts` — Unified file service (abstract)
- `src/vs/platform/files/common/diskFileSystemProvider.ts` — Disk provider interface
- `src/vs/platform/files/node/diskFileSystemProvider.ts` — Node fs implementation
- `src/vs/platform/files/electron-main/diskFileSystemProviderServer.ts` — IPC-based server
- `src/vs/platform/files/node/watcher/nodejs/nodejsWatcher.ts` — Node fs.watch wrapper
- `src/vs/platform/files/node/watcher/parcel/parcelWatcher.ts` — Parcel-based watcher
- `src/vs/platform/files/node/watcher/watcherClient.ts` — Watcher client
- `src/vs/platform/files/node/watcher/watcherMain.ts` — Watcher process main
- `src/vs/workbench/services/files/electron-browser/watcherClient.ts` — Workbench watcher integration
- `src/vs/workbench/services/files/electron-browser/diskFileSystemProvider.ts` — IPC proxy for disk FS

### Text File & Encoding Services
- `src/vs/workbench/services/textfile/common/textfiles.ts` — Text file service interface
- `src/vs/workbench/services/textfile/browser/textFileService.ts` — Browser text file impl
- `src/vs/workbench/services/textfile/electron-browser/nativeTextFileService.ts` — Native file handling
- `src/vs/workbench/services/textfile/common/encoding.ts` — Node fs encoding detection
- `src/vs/base/parts/storage/node/storage.ts` — Node-based storage primitives

### Extension Host & RPC Protocol
- `src/vs/workbench/services/extensions/common/extensionHostProtocol.ts` — Extension host IPC protocol definition
- `src/vs/workbench/services/extensions/common/rpcProtocol.ts` — RPC protocol for extensions ↔ main
- `src/vs/workbench/services/extensions/common/extensionHostProxy.ts` — Remote extension proxy
- `src/vs/workbench/services/extensions/electron-browser/localProcessExtensionHost.ts` — Electron process host
- `src/vs/workbench/services/extensions/electron-browser/nativeExtensionService.ts` — Native extension manager
- `src/vs/workbench/api/common/extHost.protocol.ts` — Extension host RPC protocol (mainThread*/extHost* methods)
- `src/vs/server/node/extensionHostConnection.ts` — Server-side extension host connection

### Editor & Language Services
- `src/vs/editor/common/languages.ts` — Language feature registry
- `src/vs/editor/common/languageFeatureRegistry.ts` — Feature registry for completions, definitions
- `src/vs/editor/common/languages/languageConfiguration.ts` — Language config (brackets, indentation)
- `src/vs/editor/browser/services/abstractCodeEditorService.ts` — Code editor service interface
- `src/vs/editor/browser/services/codeEditorService.ts` — Code editor service implementation
- `src/vs/workbench/api/browser/mainThreadLanguageFeatures.ts` — Extension API for language features
- `src/vs/workbench/api/common/extHostLanguageFeatures.ts` — Extension host language feature stubs

### Debugging Services
- `src/vs/workbench/contrib/debug/common/debug.ts` — Debug service interface
- `src/vs/workbench/contrib/debug/browser/debugService.ts` — Debug service implementation
- `src/vs/workbench/contrib/debug/browser/debugSession.ts` — Debug session management
- `src/vs/workbench/contrib/debug/browser/rawDebugSession.ts` — Raw Debug Adapter Protocol (DAP) session
- `src/vs/workbench/contrib/debug/node/debugAdapter.ts` — DAP adapter spawning (Node)
- `src/vs/platform/debug/common/extensionHostDebugIpc.ts` — Extension host debug IPC
- `src/vs/platform/debug/electron-main/extensionHostDebugIpc.ts` — Electron debug IPC

### Source Control Services
- `src/vs/workbench/contrib/scm/common/scm.ts` — SCM service interface
- `src/vs/workbench/contrib/scm/common/scmService.ts` — SCM registration
- `src/vs/workbench/contrib/scm/browser/scmViewService.ts` — SCM UI service
- `src/vs/workbench/contrib/git/common/gitService.ts` — Git service (spawns git processes)
- `src/vs/platform/git/common/localGitService.ts` — Local git integration

### Terminal Services
- `src/vs/workbench/contrib/terminal/common/terminal.ts` — Terminal service interface
- `src/vs/workbench/contrib/terminal/browser/terminalProcessManager.ts` — Process lifecycle
- `src/vs/workbench/contrib/terminal/electron-browser/localPty.ts` — Local PTY for Electron
- `src/vs/workbench/contrib/terminal/electron-browser/terminalProfileResolverService.ts` — Shell detection
- `src/vs/platform/terminal/node/terminalProcess.ts` — PTY spawning (Node pty module)
- `src/vs/platform/terminal/electron-main/electronPtyHostStarter.ts` — Electron PTY host process
- `src/vs/server/node/remoteTerminalChannel.ts` — Server terminal channel

### Model & Document Services
- `src/vs/workbench/services/model/common/modelService.ts` — Text model service
- `src/vs/editor/common/model.ts` — Core text model interface
- `src/vs/workbench/services/textmodelResolver/common/textModelResolverService.ts` — Model resolver

### Search Services
- `src/vs/workbench/services/search/common/search.ts` — Search service interface
- `src/vs/workbench/services/search/browser/searchService.ts` — Browser search
- `src/vs/workbench/services/search/electron-browser/searchService.ts` — Native search
- `src/vs/workbench/services/search/node/rawSearchService.ts` — Raw search worker
- `src/vs/workbench/services/search/node/ripgrepSearchProvider.ts` — ripgrep integration
- `src/vs/workbench/services/search/worker/localFileSearch.ts` — Web worker search

### Configuration & Settings
- `src/vs/platform/configuration/common/configurationService.ts` — Configuration service
- `src/vs/platform/environment/common/environment.ts` — Environment config interface
- `src/vs/platform/policy/common/policy.ts` — Policy service
- `src/vs/workbench/services/configuration/common/jsonEditing.ts` — Settings file editing

### Theme & UI Services
- `src/vs/platform/theme/electron-main/themeMainService.ts` — Theme (Electron native integration)
- `src/vs/workbench/services/themes/electron-browser/nativeHostColorSchemeService.ts` — Native theme sync
- `src/vs/workbench/services/themes/browser/workbenchThemeService.ts` — Workbench theme service

### Window & Shell Integration
- `src/vs/platform/windows/electron-main/windows.ts` — Electron window management
- `src/vs/platform/windows/electron-main/windowsMainService.ts` — Window service main
- `src/vs/platform/window/electron-main/window.ts` — Single window interface
- `src/vs/workbench/electron-browser/window.ts` — Renderer window
- `src/vs/platform/window/electron-browser/window.ts` — Renderer window service

### Protocol & URL Handling
- `src/vs/platform/protocol/electron-main/protocol.ts` — Custom protocol registration
- `src/vs/platform/protocol/electron-main/protocolMainService.ts` — Protocol service
- `src/vs/platform/url/electron-main/electronUrlListener.ts` — URL listener

### Menu & Dialogs
- `src/vs/platform/menubar/electron-main/menubar.ts` — Native menu bar
- `src/vs/platform/dialogs/electron-main/dialogMainService.ts` — Native file dialogs
- `src/vs/base/parts/contextmenu/electron-browser/contextmenu.ts` — Context menu renderer

### Secrets & Encryption
- `src/vs/platform/secrets/electron-main/macOSCrossAppSecretSharing.ts` — macOS Keychain integration
- `src/vs/platform/encryption/electron-main/encryptionMainService.ts` — Encryption service

### Utility & Update Services
- `src/vs/platform/update/electron-main/abstractUpdateService.ts` — Update service base
- `src/vs/platform/update/electron-main/updateService.win32.ts` — Windows update integration
- `src/vs/platform/update/electron-main/updateService.darwin.ts` — macOS update integration
- `src/vs/platform/diagnostics/electron-main/diagnosticsMainService.ts` — Diagnostics
- `src/vs/platform/utilityProcess/electron-main/utilityProcess.ts` — Electron utility process management
- `src/vs/platform/utilityProcess/electron-main/utilityProcessWorkerMainService.ts` — Worker processes

### Server-Side (Remote/Web)
- `src/vs/server/node/server.main.ts` — VS Code Server entry
- `src/vs/server/node/serverServices.ts` — Server service initialization
- `src/vs/server/node/remoteAgentEnvironmentImpl.ts` — Remote env service
- `src/vs/server/node/remoteTerminalChannel.ts` — Remote terminal channel

### Workbench Main Contributions
- `src/vs/workbench/workbench.desktop.main.ts` — Desktop workbench bootstrap (Electron)
- `src/vs/workbench/workbench.common.main.ts` — Common workbench bootstrap
- `src/vs/workbench/workbench.web.main.ts` — Web workbench bootstrap
- `src/vs/workbench/electron-browser/desktop.main.ts` — Electron desktop main
- `src/vs/workbench/electron-browser/desktop.contribution.ts` — Electron-specific contributions
- `src/vs/workbench/browser/web.main.ts` — Web main

---

## Tests

### IPC Tests
- `src/vs/base/parts/ipc/test/common/ipc.test.ts` — Core IPC protocol
- `src/vs/base/parts/ipc/test/electron-browser/ipc.mp.test.ts` — Electron message port IPC
- `src/vs/base/parts/ipc/test/browser/ipc.mp.test.ts` — Browser message port IPC
- `src/vs/base/parts/ipc/test/node/ipc.cp.integrationTest.ts` — Node child process IPC integration
- `src/vs/base/parts/ipc/test/node/ipc.net.test.ts` — Network IPC tests

### Lifecycle Tests
- `src/vs/base/test/common/lifecycle.test.ts` — Disposable lifecycle tests
- `src/vs/workbench/services/lifecycle/test/electron-browser/lifecycleService.test.ts` — Lifecycle service tests

### File System Tests
- `src/vs/platform/files/test/node/diskFileService.integrationTest.ts` — Disk FS integration
- `src/vs/platform/files/test/node/nodejsWatcher.test.ts` — Node watcher tests
- `src/vs/platform/files/test/node/parcelWatcher.test.ts` — Parcel watcher tests
- `src/vs/workbench/services/textfile/test/electron-browser/nativeTextFileService.test.ts` — Native text file tests

### Service Tests
- `src/vs/platform/instantiation/test/common/instantiationServiceMock.ts` — DI test utilities
- `src/vs/workbench/test/electron-browser/workbenchTestServices.ts` — Workbench test services
- `src/vs/workbench/test/common/workbenchTestServices.ts` — Common workbench test services

### Search Tests
- `src/vs/workbench/services/search/test/node/search.integrationTest.ts` — Search integration tests
- `src/vs/workbench/services/search/test/node/fileSearch.integrationTest.ts` — File search integration
- `src/vs/workbench/services/search/test/node/textSearch.integrationTest.ts` — Text search integration
- `src/vs/workbench/services/search/test/node/ripgrepFileSearch.test.ts` — Ripgrep tests

### Debug Tests
- `src/vs/workbench/contrib/debug/test/browser/debugSession.test.ts` — Debug session tests
- `src/vs/workbench/contrib/debug/test/node/debugger.test.ts` — Debugger adapter tests
- `src/vs/workbench/contrib/debug/test/node/terminals.test.ts` — Debug terminal tests

### Terminal Tests
- `src/vs/workbench/contrib/terminal/test/browser/terminalProcessManager.test.ts` — Terminal process tests
- `src/vs/workbench/contrib/terminal/test/node/terminalProfiles.test.ts` — Terminal profile discovery

### Extension Host Tests
- `src/vs/workbench/services/extensions/test/common/rpcProtocol.test.ts` — RPC protocol tests
- `src/vs/workbench/services/extensions/test/browser/extensionService.test.ts` — Extension service tests

---

## Types / Interfaces

### Core DI Types
- `src/vs/platform/instantiation/common/instantiation.ts` — `IInstantiationService`, `ServiceIdentifier<T>`, `BrandedService`
- `src/vs/platform/instantiation/common/serviceCollection.ts` — `ServiceCollection` type

### Lifecycle Types
- `src/vs/base/common/lifecycle.ts` — `IDisposable`, `DisposableStore`, `Disposable`, `AsyncDisposable`

### IPC Protocol Types
- `src/vs/base/parts/ipc/common/ipc.ts` — `IChannel`, `IMessagePassingProtocol`, `IServer`, `IClientOptions`
- `src/vs/base/parts/ipc/common/ipc.electron.ts` — Electron-specific IPC types
- `src/vs/base/parts/ipc/common/ipc.net.ts` — Network socket IPC types

### Service Interfaces (Major)
- `src/vs/platform/files/common/files.ts` — `IFileService`, `IFileSystemProvider`, `FileSystemEvent`
- `src/vs/platform/environment/common/environment.ts` — `IEnvironmentService`
- `src/vs/platform/storage/common/storage.ts` — `IStorageService`, `IStorage`
- `src/vs/platform/configuration/common/configuration.ts` — `IConfigurationService`
- `src/vs/workbench/services/model/common/modelService.ts` — `IModelService`
- `src/vs/editor/common/model.ts` — `ITextModel`, `ITextSnapshot`
- `src/vs/workbench/services/extensions/common/extensions.ts` — `IExtensionService`, `ExtensionHostKind`
- `src/vs/workbench/contrib/debug/common/debug.ts` — `IDebugService`, `IDebugSession`
- `src/vs/workbench/contrib/terminal/common/terminal.ts` — `ITerminalService`, `ITerminalInstance`
- `src/vs/workbench/services/search/common/search.ts` — `ISearchService`, `ITextQuery`

### Extension Host Types
- `src/vs/workbench/api/common/extHost.protocol.ts` — RPC method definitions, service contracts
- `src/vs/workbench/api/common/extHostTypes.ts` — Extension API types

### Debug Adapter Types
- `src/vs/workbench/contrib/debug/common/debugProtocol.d.ts` — DAP type definitions

---

## Configuration

### Service Registrations
- `src/vs/workbench/services/extensions/common/extensionsRegistry.ts` — Extension point registry
- `src/vs/editor/common/languages/modesRegistry.ts` — Language registration
- `src/vs/platform/actions/common/menuService.ts` — Menu registration
- `src/vs/workbench/contrib/debug/browser/debug.service.contribution.ts` — Debug service registration
- `src/vs/workbench/contrib/terminal/browser/terminal.contribution.ts` — Terminal contributions
- `src/vs/workbench/contrib/scm/browser/scm.service.contribution.ts` — SCM service registration

### Main Entry Points & Contribution Loaders
- `src/vs/code/electron-main/main.ts` — Electron main bootstrap
- `src/vs/workbench/electron-browser/desktop.contribution.ts` — Electron contributions

---

## Notable Clusters

### IPC & RPC Infrastructure (20+ files)
- `src/vs/base/parts/ipc/` — Comprehensive IPC layer supporting Electron, message ports, network sockets, child processes
  - Separation by transport: electron-main, electron-browser, node (cp, mp, net), browser (mp)
  - Protocol-level abstractions: `Channel`, `MessagePassingProtocol`, `Server`

### Lifecycle Management (5+ files)
- `src/vs/base/common/` + `src/vs/platform/lifecycle/` + `src/vs/workbench/services/lifecycle/` — Disposable pattern with leak tracking, lifecycle hooks (electron-main)

### File System Abstraction (30+ files)
- `src/vs/platform/files/` — Unified FileService with multiple providers
  - Node implementations: disk provider, file watcher (nodejs, parcel)
  - Browser implementations: IndexedDB, HTML5 FS
  - IPC proxies for remote FS access
  - Watcher clients and servers for change detection

### Extension Host & RPC (15+ files)
- `src/vs/workbench/services/extensions/common/` — RPC protocol, proxy patterns, extension running locations
- `src/vs/workbench/api/` — MainThread* and ExtHost* IPC method definitions

### Electron Platform Services (60+ files)
- `src/vs/platform/` + `src/vs/code/electron-*` + `src/vs/workbench/services/` with `electron-main`, `electron-browser` subdirs
  - Window management, menu bar, native dialogs, native file watcher, PTY host
  - Process management (main, utility, shared process)
  - Native integrations (Keychain, Spotlight, system notifications)
  - Update checking, theme synchronization

### Search Infrastructure (15+ files)
- `src/vs/workbench/services/search/` — Unified search API with workers and Node-based ripgrep adapter
  - Browser web worker: `localFileSearch`
  - Node backend: ripgrep file search, text search, raw service
  - Integration layer for repository scanning

### Terminal Management (50+ files)
- `src/vs/workbench/contrib/terminal/` — Terminal UI, profile resolution, PTY spawning
  - Platform-specific: electron-browser (local PTY), common remote handling
  - xterm.js integration with decorations, links, shell integration
  - Process manager for lifecycle, capabilities, data buffering

### Debug Adapter Protocol (40+ files)
- `src/vs/workbench/contrib/debug/` — Debug service, session management, DAP protocol
  - Browser: session lifecycle, UI (breakpoints, call stack, variables)
  - Node: DAP spawning, terminal handling
  - Extension integration via RPC

---

## Documentation

### Architecture & Organization
- `src/vs/sessions/LAYOUT.md` — Sessions module layout and architecture
- `src/vs/sessions/README.md` — Sessions overview
- `src/vs/workbench/contrib/terminalContrib/README.md` — Terminal contribution pattern
- `src/vs/workbench/contrib/chat/chatCodeOrganization.md` — Chat feature organization

### Extension Host & Services
- `src/vs/workbench/contrib/chat/common/plugins/AGENTS_PLUGINS.md` — Agent plugins
- `src/vs/platform/agentHost/common/state/AGENTS.md` — Agent state management

### Debugging & Scenarios
- `src/vs/sessions/test/e2e/README.md` — End-to-end test scenarios
- `src/vs/editor/test/node/diffing/README.md` — Diff algorithm tests

---

## Summary

This partition maps the core **dependency injection architecture**, **Disposable lifecycle system**, **Electron IPC infrastructure**, and **Node.js process couplings** in VS Code. 

Key findings:
1. **DI System**: Centralized in `src/vs/platform/instantiation/` using `createDecorator` and service registration
2. **Lifecycle**: Resource management via `IDisposable` with `DisposableStore` tracking
3. **IPC Abstraction**: 20+ file cluster supporting Electron, message ports, sockets, child processes with platform-specific implementations
4. **Platform Services**: 60+ Electron-specific files in main/browser processes (windows, menus, storage, native integrations)
5. **Extension Host**: RPC-based protocol in `src/vs/workbench/api/` with `extHost.protocol.ts` mapping service contracts
6. **File System**: Abstraction layer with Node disk provider, watchers, and browser alternatives
7. **Search, Terminal, Debug**: Major subsystems with Node backend integration (ripgrep, PTY, DAP)
8. **Electron/Node Couplings**: ~100+ files contain platform-specific code in electron-main, electron-browser, node subdirectories

A Tauri/Rust port would need to replace the Electron IPC layer, process spawning (PTY, ripgrep, DAP), and native integrations while preserving the DI and Disposable abstractions at the TypeScript level.
