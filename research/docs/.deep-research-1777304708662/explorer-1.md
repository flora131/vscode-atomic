# Partition 1 of 79 — Findings

## Scope
`src/vs/` (5906 files, 1,918,153 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Core IDE Architecture Patterns
## Partition 1: DI, Lifecycle, IPC, Process Integration

Research into concrete patterns for porting VS Code core (editing, language intelligence, debugging, source control, terminal, navigation) from TypeScript/Electron to Tauri/Rust.

---

## Pattern 1: Service Decoration & Dependency Injection

#### Pattern: createDecorator<T>(NAME) DI Service Interfaces

**Where:** `src/vs/platform/instantiation/common/instantiation.ts:109-126`

**What:** Creates a typed service identifier that functions as both a parameter decorator and service registry key, enabling compile-time safe dependency injection across the codebase.

```typescript
export function createDecorator<T>(serviceId: string): ServiceIdentifier<T> {
	if (_util.serviceIds.has(serviceId)) {
		return _util.serviceIds.get(serviceId)!;
	}

	const id = function (target: Function, key: string, index: number) {
		if (arguments.length !== 3) {
			throw new Error('@IServiceName-decorator can only be used to decorate a parameter');
		}
		storeServiceDependency(id, target, index);
	} as ServiceIdentifier<T>;

	id.toString = () => serviceId;

	_util.serviceIds.set(serviceId, id);
	return id;
}
```

**Variations / call-sites:**
- `src/vs/platform/secrets/common/secrets.ts:86` - `ISecretStorageService = createDecorator<ISecretStorageService>('secretStorageService')`
- `src/vs/workbench/services/search/common/search.ts:40` - `ISearchService = createDecorator<ISearchService>('searchService')`
- `src/vs/workbench/contrib/terminal/browser/terminal.ts:39-45` - Multiple terminal services using this pattern

#### Pattern: Service Implementation with Constructor Injection

**Where:** `src/vs/platform/secrets/common/secrets.ts:101-120`

**What:** Service classes extend Disposable and declare injected dependencies as decorated constructor parameters, enabling automatic service resolution and lifecycle management.

```typescript
export class BaseSecretStorageService extends Disposable implements ISecretStorageService {
	declare readonly _serviceBrand: undefined;

	protected readonly onDidChangeSecretEmitter = this._register(new Emitter<string>());
	readonly onDidChangeSecret: Event<string> = this.onDidChangeSecretEmitter.event;

	protected readonly _sequencer = new SequencerByKey<string>();

	constructor(
		private readonly _useInMemoryStorage: boolean,
		@IStorageService private _storageService: IStorageService,
		@IEncryptionService protected _encryptionService: IEncryptionService,
		@ILogService protected readonly _logService: ILogService,
	) {
		super();
	}
	// ...
}
```

**Variations / call-sites:**
- `src/vs/platform/native/electron-main/nativeHostMainService.ts:58-76` - Platform service with 10+ injected dependencies
- `src/vs/editor/browser/services/editorWorkerService.ts:58-79` - Editor worker service pattern
- `src/vs/code/electron-main/app.ts` - Main application bootstrap (100+ services)

---

## Pattern 2: Resource Lifecycle Management

#### Pattern: class extends Disposable with _register() for subresources

**Where:** `src/vs/base/common/lifecycle.ts:526-557`

**What:** Base class that provides tracked resource disposal through a DisposableStore, enabling automatic cleanup of event listeners, timers, and child resources via _register() method.

```typescript
export abstract class Disposable implements IDisposable {
	static readonly None = Object.freeze<IDisposable>({ dispose() { } });

	protected readonly _store = new DisposableStore();

	constructor() {
		trackDisposable(this);
		setParentOfDisposable(this._store, this);
	}

	public dispose(): void {
		markAsDisposed(this);
		this._store.dispose();
	}

	protected _register<T extends IDisposable>(o: T): T {
		if ((o as unknown as Disposable) === this) {
			throw new Error('Cannot register a disposable on itself!');
		}
		return this._store.add(o);
	}
}
```

**Variations / call-sites:**
- `src/vs/editor/browser/services/editorWorkerService.ts:58` - EditorWorkerService extends Disposable
- `src/vs/platform/native/electron-main/nativeHostMainService.ts:58` - NativeHostMainService extends Disposable
- `src/vs/platform/secrets/common/secrets.ts:101` - BaseSecretStorageService extends Disposable

#### Pattern: DisposableStore for managing collections

**Where:** `src/vs/base/common/lifecycle.ts:416-485`

**What:** Container for managing multiple IDisposable resources with tracking of disposed state and automatic cleanup on children when parent is disposed.

```typescript
export class DisposableStore implements IDisposable {
	static DISABLE_DISPOSED_WARNING = false;

	private readonly _toDispose = new Set<IDisposable>();
	private _isDisposed = false;

	public dispose(): void {
		if (this._isDisposed) {
			return;
		}
		markAsDisposed(this);
		this._isDisposed = true;
		this.clear();
	}

	public get isDisposed(): boolean {
		return this._isDisposed;
	}

	public add<T extends IDisposable>(o: T): T {
		if (!o || o === Disposable.None) {
			return o;
		}
		setParentOfDisposable(o, this);
		if (this._isDisposed) {
			if (!DisposableStore.DISABLE_DISPOSED_WARNING) {
				console.warn(new Error('Trying to add a disposable to a DisposableStore that has already been disposed of...').stack);
			}
		} else {
			this._toDispose.add(o);
		}
		return o;
	}

	public delete<T extends IDisposable>(o: T): void {
		this._toDispose.delete(o);
		o.dispose();
	}
}
```

**Variations / call-sites:**
- Used internally by all Disposable subclasses
- Used for event listener management across all major subsystems
- Used for async operation tracking (timers, promises)

---

## Pattern 3: Electron IPC Integration

#### Pattern: ipcRenderer message passing from renderer process

**Where:** `src/vs/workbench/services/lifecycle/electron-browser/lifecycleService.ts:33-77`

**What:** Renderer process listens to IPC messages from main process, handling application lifecycle events (shutdown, unload) with event reply channels.

```typescript
private registerListeners(): void {
	const windowId = this.nativeHostService.windowId;

	// Main side indicates that window is about to unload, check for vetos
	ipcRenderer.on('vscode:onBeforeUnload', async (event: unknown, ...args: unknown[]) => {
		const reply = args[0] as { okChannel: string; cancelChannel: string; reason: ShutdownReason };
		this.logService.trace(`[lifecycle] onBeforeUnload (reason: ${reply.reason})`);

		// trigger onBeforeShutdown events and veto collecting
		const veto = await this.handleBeforeShutdown(reply.reason);

		// veto: cancel unload
		if (veto) {
			this.logService.trace('[lifecycle] onBeforeUnload prevented via veto');
			this._onShutdownVeto.fire();
			ipcRenderer.send(reply.cancelChannel, windowId);
		}
		// no veto: allow unload
		else {
			this.logService.trace('[lifecycle] onBeforeUnload continues without veto');
			this.shutdownReason = reply.reason;
			ipcRenderer.send(reply.okChannel, windowId);
		}
	});

	// Main side indicates that we will indeed shutdown
	ipcRenderer.on('vscode:onWillUnload', async (event: unknown, ...args: unknown[]) => {
		const reply = args[0] as { replyChannel: string; reason: ShutdownReason };
		this.logService.trace(`[lifecycle] onWillUnload (reason: ${reply.reason})`);

		await this.handleWillShutdown(reply.reason);
		this._onDidShutdown.fire();
		ipcRenderer.send(reply.replyChannel, windowId);
	});
}
```

**Variations / call-sites:**
- `src/vs/base/parts/ipc/electron-browser/ipc.mp.ts:17-36` - MessagePort channel acquisition from main process
- `src/vs/base/parts/sandbox/electron-browser/globals.ts` - Safe ipcRenderer proxy through preload

#### Pattern: ValidatedIpcMain handler registration in main process

**Where:** `src/vs/base/parts/ipc/electron-main/ipcMain.ts:13-81`

**What:** Main process wraps Electron ipcMain with validation, supporting .on(), .once(), and .handle() patterns for both send/on and invoke/await patterns.

```typescript
class ValidatedIpcMain implements Event.NodeEventEmitter {
	private readonly mapListenerToWrapper = new WeakMap<ipcMainListener, ipcMainListener>();

	on(channel: string, listener: ipcMainListener): this {
		const wrappedListener = (event: electron.IpcMainEvent, ...args: unknown[]) => {
			if (this.validateEvent(channel, event)) {
				listener(event, ...args);
			}
		};

		this.mapListenerToWrapper.set(listener, wrappedListener);
		electron.ipcMain.on(channel, wrappedListener);
		return this;
	}

	once(channel: string, listener: ipcMainListener): this {
		electron.ipcMain.once(channel, (event: electron.IpcMainEvent, ...args: unknown[]) => {
			if (this.validateEvent(channel, event)) {
				listener(event, ...args);
			}
		});
		return this;
	}

	handle(channel: string, listener: (event: electron.IpcMainInvokeEvent, ...args: any[]) => Promise<unknown>): this {
		electron.ipcMain.handle(channel, (event: electron.IpcMainInvokeEvent, ...args: unknown[]) => {
			if (this.validateEvent(channel, event)) {
				return listener(event, ...args);
			}
			return Promise.reject(`Invalid channel '${channel}' or sender for ipcMain.handle() usage.`);
		});
		return this;
	}

	removeHandler(channel: string): this {
		electron.ipcMain.removeHandler(channel);
		return this;
	}

	removeListener(channel: string, listener: ipcMainListener): this {
		const wrappedListener = this.mapListenerToWrapper.get(listener);
		if (wrappedListener) {
			electron.ipcMain.removeListener(channel, wrappedListener);
			this.mapListenerToWrapper.delete(listener);
		}
		return this;
	}
}
```

**Variations / call-sites:**
- Used by all main→renderer communication in lifecycle, storage, window management
- Referenced in `src/vs/code/electron-main/app.ts` as `validatedIpcMain`

---

## Pattern 4: Node.js Process Integration

#### Pattern: ChildProcess spawning for CLI & subprocess management

**Where:** `src/vs/code/node/cli.ts:44-88`

**What:** CLI launcher spawns child processes with environment variable injection, stdio piping, and exit/error handling for tunnel and server subcommands.

```typescript
export async function main(argv: string[]): Promise<void> {
	let args: NativeParsedArgs;

	try {
		args = parseCLIProcessArgv(argv);
	} catch (err) {
		console.error(err.message);
		return;
	}

	for (const subcommand of NATIVE_CLI_COMMANDS) {
		if (args[subcommand]) {
			if (!product.tunnelApplicationName) {
				console.error(`'${subcommand}' command not supported in ${product.applicationName}`);
				return;
			}
			const env: IProcessEnvironment = {
				...process.env
			};
			// bootstrap-esm.js determines the electron environment based
			// on the following variable. For the server we need to unset
			// it to prevent importing any electron specific modules.
			delete env['ELECTRON_RUN_AS_NODE'];

			const tunnelArgs = argv.slice(argv.indexOf(subcommand) + 1);
			return new Promise((resolve, reject) => {
				let tunnelProcess: ChildProcess;
				const stdio: StdioOptions = ['ignore', 'pipe', 'pipe'];
				if (process.env['VSCODE_DEV']) {
					tunnelProcess = spawn('cargo', ['run', '--', subcommand, ...tunnelArgs], { cwd: join(getAppRoot(), 'cli'), stdio, env });
				} else {
					const appPath = process.platform === 'darwin'
						? join(dirname(dirname(process.execPath)), 'Resources', 'app')
						: dirname(process.execPath);
					const tunnelCommand = join(appPath, 'bin', `${product.tunnelApplicationName}${isWindows ? '.exe' : ''}`);
					tunnelProcess = spawn(tunnelCommand, [subcommand, ...tunnelArgs], { cwd: cwd(), stdio, env });
				}

				tunnelProcess.stdout!.pipe(process.stdout);
				tunnelProcess.stderr!.pipe(process.stderr);
				tunnelProcess.on('exit', resolve);
				tunnelProcess.on('error', reject);
			});
		}
	}
	// ...
}
```

**Variations / call-sites:**
- `src/vs/code/electron-main/app.ts:1-120` - Electron app initialization with process event handling
- `src/vs/platform/native/electron-main/nativeHostMainService.ts:1-50` - Native services using child_process exec/spawn
- Process environment accessed via `process.env`, `process.execPath`, `process.platform`, `process.pid`

#### Pattern: Platform-aware process environment handling

**Where:** `src/vs/code/node/cli.ts:60-81`

**What:** Process environment is cloned and modified before spawning subprocesses, with platform-specific executable paths derived from process.execPath and process.platform.

```typescript
const env: IProcessEnvironment = {
	...process.env
};
delete env['ELECTRON_RUN_AS_NODE'];

const tunnelArgs = argv.slice(argv.indexOf(subcommand) + 1);
return new Promise((resolve, reject) => {
	let tunnelProcess: ChildProcess;
	const stdio: StdioOptions = ['ignore', 'pipe', 'pipe'];
	if (process.env['VSCODE_DEV']) {
		tunnelProcess = spawn('cargo', ['run', '--', subcommand, ...tunnelArgs], { cwd: join(getAppRoot(), 'cli'), stdio, env });
	} else {
		const appPath = process.platform === 'darwin'
			? join(dirname(dirname(process.execPath)), 'Resources', 'app')
			: dirname(process.execPath);
		const tunnelCommand = join(appPath, 'bin', `${product.tunnelApplicationName}${isWindows ? '.exe' : ''}`);
		tunnelProcess = spawn(tunnelCommand, [subcommand, ...tunnelArgs], { cwd: cwd(), stdio, env });
	}

	tunnelProcess.stdout!.pipe(process.stdout);
	tunnelProcess.stderr!.pipe(process.stderr);
	tunnelProcess.on('exit', resolve);
	tunnelProcess.on('error', reject);
});
```

**Variations / call-sites:**
- File path resolution based on `process.platform` (darwin/win32/linux)
- Executable discovery using `process.execPath` as anchor point
- Working directory via `cwd()` utility function wrapping `process.cwd()`

---

## Pattern 5: Service Bootstrap & Configuration

#### Pattern: Main service collection assembly in electron-main

**Where:** `src/vs/code/electron-main/app.ts:1-120` (excerpt)

**What:** Central bootstrap location that instantiates and registers 100+ core services via ServiceCollection, including file system, storage, window management, IPC, lifecycle, and platform services.

```typescript
import { app, Details, GPUFeatureStatus, powerMonitor, protocol, session, Session, systemPreferences, WebFrameMain } from 'electron';
import { validatedIpcMain } from '../../base/parts/ipc/electron-main/ipcMain.js';
import { execFile } from 'child_process';
import { hostname, release } from 'os';
// ... 100+ more imports ...
import { IInstantiationService, ServicesAccessor } from '../../platform/instantiation/common/instantiation.js';
import { ServiceCollection } from '../../platform/instantiation/common/serviceCollection.js';
import { IBackupMainService } from '../../platform/backup/electron-main/backup.js';
import { BackupMainService } from '../../platform/backup/electron-main/backupMainService.js';
// ... service registrations follow ServiceCollection pattern ...
```

**Key service categories registered:**
- File system (DiskFileSystemProvider)
- Storage (ApplicationStorageMainService, StorageMainService)
- Window management (WindowsMainService)
- IPC coordination (multiple channel types)
- Lifecycle (LifecycleMainService)
- Telemetry (TelemetryService)
- Update (platform-specific UpdateService)
- Native integration (NativeHostMainService)

**Variations / call-sites:**
- `src/vs/workbench/test/electron-browser/workbenchTestServices.ts` - Test service collection setup
- `src/vs/sessions/sessions.desktop.main.ts` - Desktop/main process bootstrapping

---

## Key Architectural Insights for Tauri/Rust Port

### 1. **Dependency Injection as Core Pattern**
Every major subsystem (editor, terminal, search, source control, debugging) uses the `createDecorator<T>()` pattern for compile-time safe service registration. This would need Rust equivalent (likely trait-based DI container like `shaku` or custom implementation).

### 2. **Lifecycle Management is Pervasive**
The `Disposable` base class appears in 100+ classes. Resource cleanup through `_register()` and `DisposableStore` is the standard pattern. Rust's ownership system could simplify this but would require rewriting all event/listener registration patterns.

### 3. **IPC is the Process Boundary**
Electron's `ipcRenderer` / `ipcMain` with validated wrapper is the only cross-process communication. In Tauri, this maps to invoke/listen (request/response) or emit (events). Channel names like `'vscode:onBeforeUnload'` are hard-coded strings—would need registry.

### 4. **Process Environment is Mutable**
`process.env` is cloned and modified before subprocess spawning. `process.execPath`, `process.platform`, `process.cwd()` are used for platform-specific path resolution and subprocess discovery.

### 5. **Service Bootstrap Happens Once**
Main process instantiates entire service graph at startup via ServiceCollection. All renderer processes receive proxied access to main services via IPC. No hot reload of services.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
