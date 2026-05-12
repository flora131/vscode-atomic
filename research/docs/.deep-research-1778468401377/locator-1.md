# VS Code Port to Tauri/Rust — File Locator Index (Partition 1: src/vs/)

## Summary

This partition (src/vs/: 1.99M LOC, 6,048 files) contains VS Code's core IDE architecture across five nested layers: base/, platform/, editor/, workbench/, and code/. Key findings: 874 IPC/process files, 851 editor components, 3,395 workbench services/contribs, extensive Electron/Node.js coupling, and service-based architecture with decorators.

---

## Implementation

### Core Entry Points & App Initialization
- `src/vs/code/electron-main/main.ts` — Electron app entry point, bootstrap
- `src/vs/code/electron-main/app.ts` — Main process lifecycle (1,709 LOC), window creation, IPC setup
- `src/vs/workbench/workbench.desktop.main.ts` — Workbench instantiation for desktop
- `src/vs/workbench/workbench.common.main.ts` — Common workbench services, DI registration
- `src/vs/workbench/workbench.web.main.ts` — Web variant entry
- `src/vs/workbench/browser/web.main.ts` — Browser entrypoint
- `src/vs/workbench/electron-browser/desktop.main.ts` — Desktop-specific workbench setup
- `src/vs/code/node/cli.ts` — CLI argument parsing for Node.js processes

### Electron & Node.js Process Management
- `src/vs/platform/agentHost/electron-main/electronAgentHostStarter.ts` — Agent process startup
- `src/vs/platform/agentHost/node/agentHostMain.ts` — Node agent host lifecycle
- `src/vs/platform/agentHost/node/agentHostServerMain.ts` — Agent server entry
- `src/vs/platform/windows/electron-main/windows.ts` — Window management service
- `src/vs/platform/windows/electron-main/windowImpl.ts` — Electron window wrapper
- `src/vs/platform/utilityProcess/electron-main/utilityProcess.ts` — Utility process spawning
- `src/vs/platform/utilityProcess/electron-main/utilityProcessWorkerMainService.ts` — Worker process management

### IPC & Process Communication Layer
- `src/vs/base/parts/ipc/common/ipc.ts` — Core IPC protocol abstraction
- `src/vs/base/parts/ipc/electron-main/ipcMain.ts` — Electron main process IPC handler
- `src/vs/base/parts/ipc/electron-main/ipc.electron.ts` — Electron IPC implementation
- `src/vs/base/parts/ipc/electron-main/ipc.mp.ts` — Message port IPC for main process
- `src/vs/base/parts/ipc/electron-browser/ipc.electron.ts` — Renderer process IPC
- `src/vs/base/parts/ipc/electron-browser/ipc.mp.ts` — Renderer message port IPC
- `src/vs/base/parts/ipc/common/ipc.electron.ts` — Shared Electron IPC definitions
- `src/vs/base/parts/ipc/common/ipc.mp.ts` — Message port IPC protocol
- `src/vs/base/parts/ipc/common/ipc.net.ts` — Network socket IPC
- `src/vs/base/parts/ipc/node/ipc.cp.ts` — Child process IPC (Node.js)
- `src/vs/base/parts/ipc/node/ipc.net.ts` — Network IPC (Node.js)
- `src/vs/platform/ipc/common/mainProcessService.ts` — Main process RPC service
- `src/vs/platform/ipc/electron-browser/mainProcessService.ts` — Renderer-to-main RPC client
- `src/vs/platform/ipc/common/services.ts` — Service definitions for IPC
- `src/vs/platform/ipc/electron-browser/services.ts` — Browser-side IPC services

### Dependency Injection & Service Registry
- `src/vs/platform/instantiation/common/instantiation.ts` — DI container (createDecorator, InstantiationService)
- `src/vs/platform/registry/common/registry.ts` — Service registry and contribution points

### Editor Core (851 files)
- `src/vs/editor/common/services/languageFeaturesService.ts` — Registry for language features (completion, hover, etc.)
- `src/vs/editor/common/services/languageFeatures.ts` — Feature interface definitions
- `src/vs/editor/common/services/languageService.ts` — Language mode management
- `src/vs/editor/common/services/editorWorkerService.ts` — Web worker for text operations
- `src/vs/editor/common/languages.ts` — Language interface and registration
- `src/vs/editor/common/languages/languageConfigurationRegistry.ts` — Language config storage
- `src/vs/editor/browser/services/codeEditorService.ts` — Editor instance registry
- `src/vs/editor/browser/services/abstractCodeEditorService.ts` — Common editor code
- `src/vs/editor/browser/view.ts` — Main editor view (rendering, input)
- `src/vs/editor/browser/editorBrowser.ts` — Editor public API
- `src/vs/editor/browser/gpu/gpu.ts` — GPU rendering engine
- `src/vs/editor/contrib/` — 70+ editor contributions (suggest, folding, hover, inlayHints, etc.)

### Language Intelligence & LSP Integration
- `src/vs/editor/contrib/suggest/browser/suggestWidget.ts` — Autocomplete widget
- `src/vs/editor/contrib/hover/browser/hoverController.ts` — Hover provider
- `src/vs/editor/contrib/gotoSymbol/browser/goToDefinitionAtPosition.ts` — Definition navigation
- `src/vs/editor/contrib/rename/browser/renameController.ts` — Symbol renaming
- `src/vs/editor/contrib/inlineCompletions/browser/inlineCompletionsController.ts` — Inline completions
- `src/vs/editor/contrib/semanticTokens/browser/documentSemanticTokens.ts` — Token coloring
- `src/vs/workbench/api/browser/mainThreadLanguageFeatures.ts` — Extension host bridge for language features
- `src/vs/workbench/contrib/inlineCompletions/browser/inlineCompletions.contribution.ts` — Inline completion wiring

### File System & Workspace
- `src/vs/platform/files/common/fileService.ts` — Core file abstraction with providers
- `src/vs/platform/files/node/diskFileSystemProvider.ts` — Local disk file provider
- `src/vs/platform/files/browser/indexedDBFileSystemProvider.ts` — Browser-based file storage
- `src/vs/platform/files/node/watcher/watcher.ts` — File change watching
- `src/vs/platform/files/node/watcher/watcherMain.ts` — Watcher process entry
- `src/vs/platform/workspace/common/workspace.ts` — Workspace interface (folders, settings)
- `src/vs/workbench/services/workspaces/common/workspacesManagementService.ts` — Workspace management
- `src/vs/workbench/services/workingCopy/common/workingCopyService.ts` — Working copy tracking (dirty/clean state)
- `src/vs/workbench/services/files/electron-browser/diskFileSystemProvider.ts` — Desktop file access via main process

### Configuration & Settings
- `src/vs/platform/configuration/common/configuration.ts` — Configuration service interface
- `src/vs/platform/environment/common/environment.ts` — App environment (paths, args, version)
- `src/vs/platform/environment/node/userDataPath.ts` — User data directory logic
- `src/vs/workbench/services/configuration/browser/configuration.ts` — Workbench configuration resolver
- `src/vs/workbench/services/configuration/common/configuration.ts` — Settings & configuration service
- `src/vs/workbench/services/configurationResolver/common/configurationResolver.ts` — Variable substitution (${workspaceFolder}, etc.)

### Terminal & PTY
- `src/vs/platform/terminal/common/terminal.ts` — Terminal interfaces (ITerminalChildProcess, etc.)
- `src/vs/platform/terminal/node/ptyService.ts` — PTY service (pseudo-terminal)
- `src/vs/platform/terminal/node/ptyHostMain.ts` — PTY host process entry
- `src/vs/platform/terminal/node/terminalProcess.ts` — Process spawning wrapper
- `src/vs/platform/terminal/node/terminalProfiles.ts` — Profile detection (shells)
- `src/vs/platform/terminal/electron-main/electronPtyHostStarter.ts` — Electron PTY host launcher
- `src/vs/workbench/contrib/terminal/browser/terminal.ts` — Workbench terminal interfaces
- `src/vs/workbench/contrib/terminal/browser/terminalInstance.ts` — Terminal instance (client side)
- `src/vs/workbench/contrib/terminal/electron-browser/localTerminalBackend.ts` — Desktop terminal backend
- `src/vs/workbench/services/terminal/common/embedderTerminalService.ts` — Terminal embedding API

### Debugging Support
- `src/vs/workbench/contrib/debug/common/debug.ts` — Debug service interfaces
- `src/vs/workbench/contrib/debug/browser/debugService.ts` — Debug session management
- `src/vs/workbench/contrib/debug/common/debugger.ts` — Debugger adapter registry
- `src/vs/workbench/contrib/debug/node/debugAdapter.ts` — Debug adapter process spawning
- `src/vs/workbench/contrib/debug/node/telemetryApp.ts` — DAP telemetry
- `src/vs/workbench/contrib/debug/browser/debugSession.ts` — Debug session control (launch, breakpoints, etc.)
- `src/vs/workbench/api/browser/mainThreadDebugService.ts` — Extension API for debugging

### Source Control (SCM)
- `src/vs/workbench/contrib/scm/common/scm.ts` — SCM provider interfaces
- `src/vs/workbench/contrib/scm/browser/scmViewlet.ts` — SCM UI
- `src/vs/platform/git/common/git.ts` — Git service interface
- `src/vs/platform/git/node/git.ts` — Git CLI wrapper
- `src/vs/workbench/contrib/git/browser/gitActions.ts` — Git commands

### Workbench Services (3,395 files)
- `src/vs/workbench/services/editor/common/editorService.ts` — Editor opening/management
- `src/vs/workbench/services/editor/common/editorGroupsService.ts` — Editor group/layout management
- `src/vs/workbench/services/editor/browser/editorResolverService.ts` — Editor input resolution
- `src/vs/workbench/services/extensions/common/extensions.ts` — Extension management (loading, enable/disable)
- `src/vs/workbench/services/extensionManagement/browser/extensionManagementService.ts` — Extension install/uninstall
- `src/vs/workbench/services/search/common/search.ts` — File search/replace
- `src/vs/workbench/services/textfile/common/textfiles.ts` — Text file operations (read, write, encoding)
- `src/vs/workbench/services/themes/common/workbenchThemeService.ts` — Theme management
- `src/vs/workbench/services/dialogs/browser/fileDialogService.ts` — File picker dialogs
- `src/vs/workbench/services/dialogs/electron-browser/fileDialogService.ts` — Native file dialogs
- `src/vs/workbench/services/lifecycle/electron-browser/lifecycleService.ts` — App lifecycle events
- `src/vs/workbench/services/log/electron-browser/logService.ts` — Logging
- `src/vs/workbench/services/clipboard/browser/clipboardService.ts` — Clipboard access
- `src/vs/workbench/services/encryption/electron-browser/encryptionService.ts` — Credential encryption
- `src/vs/workbench/services/secrets/common/secrets.ts` — System keyring integration

### Workbench Contrib Modules (100+ subdirectories)
- `src/vs/workbench/contrib/debug/` — 40+ files for debugging UI and protocol
- `src/vs/workbench/contrib/terminal/` — Terminal UI and backend integration
- `src/vs/workbench/contrib/scm/` — Source control integration
- `src/vs/workbench/contrib/search/` — File search UI
- `src/vs/workbench/contrib/files/` — File explorer UI
- `src/vs/workbench/contrib/extensions/` — Extension marketplace and management UI
- `src/vs/workbench/contrib/tasks/` — Task runner (build, custom tasks)
- `src/vs/workbench/contrib/chat/` — Chat/copilot UI integration
- `src/vs/workbench/contrib/preferences/` — Settings UI
- `src/vs/workbench/contrib/markers/` — Problem panel
- `src/vs/workbench/contrib/notebook/` — Notebook editor support
- `src/vs/workbench/contrib/testing/` — Testing UI
- `src/vs/workbench/contrib/themes/` — Theme picker

### Extension Host & Plugin Architecture
- `src/vs/workbench/api/browser/mainThreadExtensionService.ts` — Extension host process management
- `src/vs/workbench/api/common/extHost.protocol.ts` — RPC protocol for extension host (200+ messages)
- `src/vs/workbench/api/node/extensionHostProcess.ts` — Extension host process entry
- `src/vs/workbench/api/node/extHostExtensionService.ts` — Ext host service impl
- `src/vs/workbench/api/worker/extensionHostWorkerMain.ts` — Web worker extension host
- `src/vs/workbench/api/browser/mainThreadLanguageFeatures.ts` — LSP bridge
- `src/vs/workbench/api/browser/mainThreadTask.ts` — Task execution bridge
- `src/vs/workbench/api/browser/mainThreadTerminalService.ts` — Terminal API bridge

### Platform Services Layer (874 files)
- `src/vs/platform/files/` — File system abstraction
- `src/vs/platform/storage/` — App storage (settings, state)
- `src/vs/platform/terminal/` — Terminal interfaces and implementations
- `src/vs/platform/debug/` — Debug adapter interfaces
- `src/vs/platform/request/` — HTTP/network requests
- `src/vs/platform/shell/` — Shell environment detection
- `src/vs/platform/native/electron-main/nativeHostMainService.ts` — Native operations (shell, clipboard, etc.)
- `src/vs/platform/encryption/electron-main/encryptionMainService.ts` — OS-level encryption
- `src/vs/platform/keyboardLayout/electron-main/keyboardLayoutService.ts` — Keyboard detection

### Remote / SSH Support
- `src/vs/platform/remote/node/wsl.ts` — WSL detection and integration
- `src/vs/platform/remoteTunnel/node/remoteTunnelService.ts` — Tunnel creation
- `src/vs/server/node/server.main.ts` — Remote server entry point
- `src/vs/server/node/remoteExtensionHostAgentServer.ts` — Remote extension host agent

### Agent Host Services
- `src/vs/platform/agentHost/common/agentService.ts` — Agent interface
- `src/vs/platform/agentHost/node/agentHostMain.ts` — Agent host lifecycle
- `src/vs/platform/agentHost/common/remoteAgentHostService.ts` — Remote agent connection
- `src/vs/platform/agentHost/browser/remoteAgentHostServiceImpl.ts` — Browser-side agent client

---

## Tests

### Editor Tests
- `src/vs/editor/test/common/services/languagesRegistry.test.ts` — Language registry tests
- `src/vs/editor/test/common/services/testLanguageConfigurationService.ts` — Language config tests

### IPC Tests
- `src/vs/base/parts/ipc/test/common/ipc.test.ts` — Core IPC protocol tests
- `src/vs/base/parts/ipc/test/electron-browser/ipc.mp.test.ts` — Message port tests
- `src/vs/base/parts/ipc/test/node/ipc.cp.integrationTest.ts` — Child process IPC tests
- `src/vs/base/parts/ipc/test/node/ipc.net.test.ts` — Network IPC tests

### Workbench Tests
- `src/vs/workbench/test/common/workbenchTestServices.ts` — Common test services
- `src/vs/workbench/test/browser/workbenchTestServices.ts` — Browser test doubles
- `src/vs/workbench/test/electron-browser/workbenchTestServices.ts` — Electron test doubles
- `src/vs/workbench/services/files/test/node/parcelWatcher.test.ts` — File watcher tests
- `src/vs/workbench/contrib/debug/test/browser/debugService.test.ts` — Debug service tests
- `src/vs/workbench/contrib/terminal/test/browser/terminalInstance.test.ts` — Terminal tests

### Integration Tests
- `src/vs/platform/files/test/node/diskFileService.integrationTest.ts` — Disk I/O tests
- `src/vs/platform/environment/test/node/userDataPath.test.ts` — Path resolution tests
- `src/vs/workbench/contrib/policyExport/test/node/policyExport.integrationTest.ts` — Policy integration tests

---

## Types / Interfaces

### Core Abstractions
- `src/vs/platform/instantiation/common/instantiation.ts` — IInstantiationService, createDecorator
- `src/vs/base/common/event.ts` — Event<T> abstraction
- `src/vs/base/common/async.ts` — Promise utilities, cancelToken
- `src/vs/base/common/uri.ts` — URI parsing and manipulation

### Editor Types
- `src/vs/editor/common/languages.ts` — ILanguage, ILanguageExtensionPoint
- `src/vs/editor/common/services/languageFeatures.ts` — ICompletionProvider, IHoverProvider, etc.
- `src/vs/editor/common/model.ts` — ITextModel (editor document)
- `src/vs/editor/common/core/position.ts` — Position, Range
- `src/vs/editor/common/core/selection.ts` — Selection

### Workbench Types
- `src/vs/workbench/common/editor.ts` — EditorInput, EditorPane
- `src/vs/workbench/contrib/terminal/common/terminal.ts` — ITerminalInstance, ITerminal interface
- `src/vs/workbench/contrib/debug/common/debug.ts` — IDebugSession, IStackFrame, IBreakpoint
- `src/vs/workbench/contrib/scm/common/scm.ts` — ISCMProvider, IResource
- `src/vs/workbench/common/views.ts` — IViewContainersRegistry, IViewDescriptor
- `src/vs/workbench/api/common/extHost.protocol.ts` — RPC protocol types (200+ message types)

### Platform Types
- `src/vs/platform/files/common/files.ts` — IFileService, IFile, FileChangeType
- `src/vs/platform/workspace/common/workspace.ts` — IWorkspace, IWorkspaceFolder
- `src/vs/platform/terminal/common/terminal.ts` — ITerminalLaunchConfig, ITerminalChildProcess
- `src/vs/platform/configuration/common/configuration.ts` — IConfiguration

---

## Configuration

### Contribution Points & Registry
- `src/vs/platform/configuration/common/configurationRegistry.ts` — Settings schema registry
- `src/vs/platform/registry/common/registry.ts` — Generic registry for extensions
- `src/vs/platform/jsonschemas/common/jsonContributionRegistry.ts` — JSON schema registry
- `src/vs/platform/actions/common/actions.ts` — Keybinding and command registry

### Build & Runtime Config
- `src/vs/base/common/product.ts` — Product identifier and build config
- `src/vs/code/electron-main/app.ts` — Electron app config (app name, version, etc.)
- `src/vs/platform/environment/node/argvHelper.ts` — CLI argument parsing

### Settings Schema Files
- `src/vs/workbench/contrib/preferences/browser/keybindingsEditorInput.ts` — Keybindings schema
- `src/vs/workbench/contrib/terminal/common/terminalExtensionPoints.ts` — Terminal profile schema

---

## Examples / Fixtures

### Editor Fixtures
- `src/vs/workbench/test/browser/componentFixtures/editor/inlineCompletions/other.fixture.ts` — Test editor setup
- `src/vs/workbench/test/browser/componentFixtures/editor/multiDiffEditor.fixture.ts` — Multi-diff editor test

### Service Mocks
- `src/vs/workbench/contrib/debug/test/common/mockDebug.ts` — Debug service mock
- `src/vs/workbench/contrib/debug/common/nullDebugService.ts` — No-op debug service
- `src/vs/workbench/test/common/workbenchTestServices.ts` — 100+ test service stubs

### Test Helpers
- `src/vs/platform/agentHost/test/node/mockAgent.ts` — Agent test double
- `src/vs/base/parts/ipc/test/node/testService.ts` — IPC test service

---

## Documentation

### Code Comments & READMEs
- `src/vs/editor/common/services/languageFeatures.ts` — Language features service documentation
- `src/vs/base/parts/ipc/common/ipc.ts` — IPC protocol documentation
- `src/vs/platform/instantiation/common/instantiation.ts` — DI container documentation
- `src/vs/workbench/api/common/extHost.protocol.ts` — Extension host RPC protocol spec (inline)

### Type Definition Comments
- `src/vs/editor/common/languages.ts` — Language interface JSDoc
- `src/vs/workbench/common/editor.ts` — Editor pane interface documentation
- `src/vs/platform/files/common/files.ts` — File service API documentation

---

## Notable Clusters

### Editor Architecture (851 files)
- `src/vs/editor/common/` — 200+ files for editor model, languages, tokens, diff
- `src/vs/editor/browser/` — 300+ files for DOM, rendering, input, view
- `src/vs/editor/contrib/` — 300+ files spread across 70+ feature subdirectories (suggest, folding, hover, inlayHints, rename, etc.)
- Why: Complete code editor with syntax highlighting, intellisense, refactoring support, multi-cursor, language features

### Workbench Services (300+ files)
- `src/vs/workbench/services/` — Organized into 90+ subdirectories (editor, terminal, files, extensions, configuration, etc.)
- Why: Service layer providing IDE functionality to the UI; includes file system, storage, dialogs, lifecycle management

### Workbench Contributions (3,000+ files)
- `src/vs/workbench/contrib/` — 100+ subdirectories covering major IDE features
- Key clusters: debug (150+ files), terminal (200+ files), extensions (300+ files), tasks (200+ files), chat (400+ files)
- Why: Plugin-based contributions for all IDE features; each registers via contribution points

### IPC & Process Management (100+ files)
- `src/vs/base/parts/ipc/` — Core IPC protocol (electron, message port, network, child process)
- `src/vs/platform/ipc/` — High-level RPC service wrappers
- Why: Critical for main/renderer split, extension host isolation, PTY service, LSP communication

### Platform Abstraction Layer (874 files)
- `src/vs/platform/` — Base interfaces and electron-specific implementations
- Covers: files, terminal, storage, configuration, environment, launch, windows, dialogs, etc.
- Why: Separates business logic from platform (Electron, Node, Browser); key for porting

### Extension Host & API (200+ files)
- `src/vs/workbench/api/browser/` — 50+ mainThread* files bridging main and extension host
- `src/vs/workbench/api/common/extHost.protocol.ts` — 200+ RPC message types
- `src/vs/workbench/api/node/` — Extension host process and utilities
- Why: Extension ecosystem is core to VS Code; full reimplementation needed for Tauri port

### Terminal Subsystem (150+ files)
- `src/vs/platform/terminal/` — PTY service, profiles, environment
- `src/vs/workbench/contrib/terminal/` — UI, backend integration, shell detection
- Why: Terminal access requires OS-level process spawning; critical for Rust port

### Debug Adapter Protocol (120+ files)
- `src/vs/workbench/contrib/debug/` — Debug session, DAP client, breakpoint management
- `src/vs/workbench/contrib/debug/node/debugAdapter.ts` — DAP spawning
- Why: DAP is network-based; can be reused but process spawning must be Rust-native

### File System Abstraction (100+ files)
- `src/vs/platform/files/` — File service, disk provider, watcher, browser providers
- Why: Core abstraction; enables same code to run on desktop, web, remote

### Remote / Server Support (50+ files)
- `src/vs/server/node/` — Remote server entry points and services
- `src/vs/platform/remote/` — Remote connection management
- Why: Understanding existing remote architecture helps design Tauri-based remote

---

## Key Architectural Patterns Identified

### 1. Service Locator Pattern via DI
- **Where**: `src/vs/platform/instantiation/common/instantiation.ts`
- **Pattern**: `createDecorator<IService>()` → `@inject(IService)` → `InstantiationService.createInstance(Class)`
- **Impact**: All ~200 platform services follow this pattern; porting requires Rust DI container

### 2. Electron IPC Coupling
- **Where**: `src/vs/base/parts/ipc/electron-*`, `src/vs/code/electron-main/app.ts`, renderer process
- **Pattern**: Electron's `ipcMain.handle()` / `ipcRenderer.invoke()` for RPC between main and renderer
- **Impact**: ~100 IPC handlers throughout codebase; Tauri uses different IPC (Tauri invoke, events)

### 3. Process Hierarchy
- **Main Process**: `src/vs/code/electron-main/` — Electron main, window management, native access
- **Renderer Process**: Workbench, editor, all UI — runs in browser context
- **Extension Host**: Separate Node.js process (`src/vs/workbench/api/node/extensionHostProcess.ts`)
- **PTY Host**: Separate process (`src/vs/platform/terminal/node/ptyHostMain.ts`)
- **Watchers**: File watcher child process (`src/vs/platform/files/node/watcher/watcherMain.ts`)
- **Impact**: Tauri can simplify to backend (Rust) + frontend (web); extension host needs rethinking

### 4. Plugin Architecture via Contribution Points
- **Where**: `src/vs/platform/registry/common/registry.ts`, service interfaces
- **Pattern**: Registry-based contributions; extensions register at well-defined points
- **Files**: `src/vs/workbench/contrib/*/browser/*.contribution.ts` (100+ files)
- **Impact**: Plugin system must be preserved; requires Rust FFI or RPC for ext language features

### 5. File System Abstraction
- **Where**: `src/vs/platform/files/common/fileService.ts`
- **Pattern**: IFileService with pluggable IFileSystemProvider (disk, indexedDB, etc.)
- **Impact**: Already abstracted; Tauri can implement Rust-native disk provider

### 6. Language Features via Registry
- **Where**: `src/vs/editor/common/services/languageFeaturesService.ts`
- **Pattern**: Central registry for completion, hover, definition, etc.
- **Impact**: Can be preserved; wired to LSP or native language servers

### 7. Configuration & Storage Layer
- **Where**: `src/vs/platform/configuration/`, `src/vs/platform/storage/`
- **Pattern**: IConfigurationService, IStorageService as platform abstractions
- **Impact**: Desktop implementation exists; can map to Rust-based storage

### 8. Extension Host IPC Protocol
- **Where**: `src/vs/workbench/api/common/extHost.protocol.ts`
- **Pattern**: 200+ message types defining RPC between main and extension host
- **Impact**: Largest RPC surface; must be preserved for extension compatibility

---

## Critical Dependencies for Porting

### Electron Modules (Hard Requirement → Tauri Equivalent)
- `electron.ipcMain` → Tauri's command/event system
- `electron.app` → Tauri app lifecycle
- `electron.BrowserWindow` → Tauri window
- `electron.dialog` → Tauri dialogs
- `electron.shell` → Tauri shell module or custom Rust code
- `electron.nativeImage` → Tauri's native image handling

### Node.js Modules (Must Reimplement in Rust)
- `child_process.spawn()` → std::process::Command (terminal, debuggers, extensions)
- `fs` → std::fs / tokio::fs (file operations)
- `path` → path crate
- `crypto` → ring, sodiumoxide
- `stream` → tokio streams

### TypeScript/JavaScript Runtime Assumptions
- Worker threads (`new Worker()`) → Web Workers (browser) or Tauri's multithreading
- Node.js buffer/stream APIs → Rust bytes, futures
- Promise-based async → Rust futures/async-await

---

## Conclusion

VS Code's 1.99M LOC in src/vs/ is organized into five abstraction layers with heavy reliance on Electron/Node.js. Porting to Tauri/Rust requires:

1. **IPC Redesign**: Replace Electron IPC (150+ invocations) with Tauri commands/events.
2. **Process Model**: Consolidate main/renderer into single Tauri backend (Rust); extension host becomes RPC client.
3. **Service Reimplementation**: DI-based services must map to Rust structs; focus on platform/ layer first.
4. **Editor Core**: Editor itself is ~850 files, mostly language-agnostic; porting is feasible but requires WASM or JS binding.
5. **Extension Host**: 200+ RPC messages; must remain or use simplified VSCode API bridge.
6. **Incremental Path**: Port platform/ → editor/ → workbench services → contrib modules.

