# VS Code TypeScript Language Features Extension - Architecture Locator

## Implementation

### Core Extension Infrastructure
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/extension.ts` - Primary extension entry point (114 LOC)
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/extension.browser.ts` - Browser-specific extension initialization
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/api.ts` - Public API surface for plugins and consumers

### TypeScript Service Client & Server Management
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/typescriptServiceClient.ts` - Main client for communicating with tsserver (1309 LOC)
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/typescriptService.ts` - TypeScript service abstraction layer
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/typeScriptServiceClientHost.ts` - Client host management

### Process & Server Spawning (Critical for Porting)
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/server.ts` - Core server implementation with request routing (703 LOC). Contains:
  - `SingleTsServer` class - manages individual tsserver instances
  - `GetErrRoutingTsServer` - routes error checking to separate server
  - `SyntaxRoutingTsServer` - routes syntax requests separately
  - EventEmitter for request/response lifecycle
  - Protocol event handling
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/spawner.ts` - Process spawning orchestration
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/serverProcess.electron.ts` - **Electron-specific process spawning with child_process.fork()**. Uses:
  - Stream-based I/O (Reader/Writer pattern on stdout/stderr)
  - IPC channels via `stdio: ['pipe', 'pipe', 'pipe', 'ipc']`
  - Process management and error handling
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts` - Browser-specific server (WebWorker)
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/nodeManager.ts` - Node.js process management
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/versionManager.ts` - TypeScript version management
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/versionProvider.ts` - Version discovery
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/versionProvider.electron.ts` - Electron-specific version discovery

### Language Features Registration (28 providers)
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageProvider.ts` - Central language feature registration
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/completions.ts` - Completion provider (32.6 KB)
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/definitions.ts` - Go to definition
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/hover.ts` - Hover information
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/references.ts` - Find references
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/rename.ts` - Rename symbol
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/implementations.ts` - Find implementations
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/typeDefinitions.ts` - Go to type definition
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/quickFix.ts` - Quick fix provider (19.4 KB)
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/refactor.ts` - Refactoring provider (27.5 KB)
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/formatting.ts` - Code formatting
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/signatureHelp.ts` - Function signature help
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/semanticTokens.ts` - Semantic token coloring
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/documentSymbol.ts` - Document symbol outline
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/workspaceSymbols.ts` - Workspace symbol search
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/organizeImports.ts` - Organize imports command
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/documentHighlight.ts` - Highlight document occurrences
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/diagnostics.ts` - Diagnostic reporting (14.2 KB)
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/fixAll.ts` - Fix all issues
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/folding.ts` - Code folding
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/smartSelect.ts` - Smart selection
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/callHierarchy.ts` - Call hierarchy
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/inlayHints.ts` - Inlay hints
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/jsDocCompletions.ts` - JSDoc completions
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/linkedEditing.ts` - Linked editing support
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/tagClosing.ts` - JSX tag closing
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/sourceDefinition.ts` - Source map support
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/directiveCommentCompletions.ts` - Directive completions
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/copyPaste.ts` - Paste special handling

### Code Lens Providers
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/codeLens/baseCodeLensProvider.ts` - Base class
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/codeLens/referencesCodeLens.ts` - References count
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/codeLens/implementationsCodeLens.ts` - Implementations count

### Server Protocol & Communication
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/protocol/protocol.d.ts` - TypeScript protocol type definitions
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/protocol/protocol.const.ts` - Protocol constants and message types
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/protocol/errorCodes.ts` - Error code constants
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/protocol/fixNames.ts` - Quick fix names
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/protocol/modifiers.ts` - Symbol modifiers

### Request Queue & Synchronization
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/requestQueue.ts` - Request queuing and prioritization
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/bufferSyncSupport.ts` - Buffer synchronization between client and server
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/fileWatchingManager.ts` - File watching coordination
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/cachedResponse.ts` - Response caching

### Configuration & Discovery
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/configuration/configuration.ts` - Configuration management
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/configuration/configuration.electron.ts` - Electron-specific config
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/configuration/configuration.browser.ts` - Browser-specific config
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/configuration/documentSelector.ts` - Document selector configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/configuration/languageIds.ts` - Supported language IDs
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/configuration/languageDescription.ts` - Language descriptions
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/configuration/fileSchemes.ts` - Supported file schemes

### File Configuration Manager
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/fileConfigurationManager.ts` - Per-file tsserver configuration (18 KB)

### Commands
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/commands/index.ts` - Command registration (12 commands)
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/commands/commandManager.ts` - Command execution
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/commands/restartTsServer.ts` - Restart tsserver
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/commands/reloadProject.ts` - Reload project config
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/commands/selectTypeScriptVersion.ts` - Select TS version
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/commands/openTsServerLog.ts` - Open server logs
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/commands/goToProjectConfiguration.ts` - Navigate to tsconfig
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/commands/tsserverRequests.ts` - Direct tsserver requests
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/commands/openJsDocLink.ts` - JSDoc link navigation
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/commands/learnMoreAboutRefactorings.ts` - Refactoring documentation
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/commands/configurePlugin.ts` - Plugin configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/commands/useTsgo.ts` - TSGO server integration

### UI & Status
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/ui/intellisenseStatus.ts` - Intellisense status bar
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/ui/typingsStatus.ts` - Typings installation status
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/ui/versionStatus.ts` - TypeScript version status
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/ui/largeProjectStatus.ts` - Large project warnings
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/ui/activeJsTsEditorTracker.ts` - Active editor tracking
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/ui/managedFileContext.ts` - File context management
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/ui/suggestNativePreview.ts` - Completion preview

### Logging & Telemetry
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/logging/logger.ts` - Logging infrastructure
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/logging/telemetry.ts` - Telemetry reporting
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/logging/tracer.ts` - Request tracing
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/logging/logLevelMonitor.ts` - Log level monitoring

### Browser/Web-specific Implementation
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/web/src/serverHost.ts` - Browser server host (13.8 KB)
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/web/src/webServer.ts` - Web server integration
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/web/src/workerSession.ts` - WebWorker session management
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/web/src/wasmCancellationToken.ts` - WASM cancellation support
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/web/src/fileWatcherManager.ts` - Browser file watching
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/web/src/pathMapper.ts` - Path mapping for web
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/web/src/logging.ts` - Browser logging
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/web/src/typingsInstaller/typingsInstaller.ts` - Types installation
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/web/src/typingsInstaller/jsTyping.ts` - JS typing resolution
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/web/src/util/args.ts` - CLI argument parsing
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/web/src/util/hrtime.ts` - High-resolution timing

### Utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/async.ts` - Async utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/arrays.ts` - Array utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/cancellation.ts` - Cancellation token utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/configuration.ts` - Config utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/dispose.ts` - Disposable utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/fs.ts` - Filesystem utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/fs.electron.ts` - Electron-specific FS
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/hash.ts` - Hashing utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/lazy.ts` - Lazy loading
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/objects.ts` - Object utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/packageInfo.ts` - Package info
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/platform.ts` - Platform detection
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/regexp.ts` - Regex utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/resourceMap.ts` - Resource mapping
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/relativePathResolver.ts` - Path resolution
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/utils/temp.electron.ts` - Temp file management

### Task Providers
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/task/taskProvider.ts` - Task provider for build tasks
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/task/tsconfigProvider.ts` - TSConfig discovery for tasks

### Plugin System
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/plugins.ts` - Plugin loading and management
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/pluginPathsProvider.ts` - Plugin path discovery

### Language Feature Utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/util/codeAction.ts` - Code action utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/util/dependentRegistration.ts` - Registration helpers
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/util/snippetForFunctionCall.ts` - Function signature snippets
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/util/textRendering.ts` - Text formatting utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/util/copilot.ts` - Copilot integration

### Other Features
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/lazyClientHost.ts` - Lazy client initialization
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/typeConverters.ts` - Protocol to VS Code API type conversion
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/experimentationService.ts` - A/B testing service
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/experimentTelemetryReporter.ts` - Experiment telemetry
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/remoteRepositories.browser.ts` - Remote repo support
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsconfig.ts` - TSConfig parsing
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/definitionProviderBase.ts` - Definition provider base
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/fileReferences.ts` - File-level references
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/languageFeatures/updatePathsOnRename.ts` - Path updates on rename
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/cancellation.ts` - Request cancellation
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/cancellation.electron.ts` - Electron-specific cancellation
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/callbackMap.ts` - Callback mapping
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/serverError.ts` - Error handling
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/logDirectoryProvider.ts` - Log directory discovery
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/tsServer/logDirectoryProvider.electron.ts` - Electron-specific logging

### Filesystem Abstractions
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/filesystems/memFs.ts` - In-memory filesystem
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/filesystems/autoInstallerFs.ts` - Auto-installer filesystem
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/filesystems/ata.ts` - Abstract filesystem

## Tests

### Smoke Tests (Integration/E2E)
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/smoke/completions.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/smoke/fixAll.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/smoke/implementationsCodeLens.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/smoke/jsDocCompletions.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/smoke/quickFix.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/smoke/referencesCodeLens.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/smoke/index.ts` - Test suite entry

### Unit Tests
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/unit/cachedResponse.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/unit/functionCallSnippet.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/unit/jsdocSnippet.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/unit/onEnter.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/unit/requestQueue.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/unit/server.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/unit/textRendering.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/unit/index.ts` - Unit test suite entry

### Test Infrastructure
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/index.ts` - Main test entry
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/testUtils.ts` - Test utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test/suggestTestHelpers.ts` - Completion test helpers
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/src/test-all.ts` - Full test runner
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/test-workspace/` - Contains test fixtures (4 files)

## Configuration

### Build Configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/package.json` - Extension manifest and dependencies
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/package-lock.json` - Locked dependencies
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/tsconfig.json` - TypeScript configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/tsconfig.browser.json` - Browser build config
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/esbuild.mts` - Esbuild configuration (Electron)
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/esbuild.browser.mts` - Esbuild configuration (Browser)

### Manifest & Package Info
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/package.nls.json` - Localization strings
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/cgmanifest.json` - Component governance manifest

### JSON Schemas
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/schemas/tsconfig.schema.json`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/schemas/jsconfig.schema.json`
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/schemas/package.schema.json`

### Web Build Configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/web/tsconfig.json` - Web build TypeScript config
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/web/README.md` - Web implementation docs

## Documentation

### README Files
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/README.md` - Extension overview
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/web/README.md` - Browser/web runtime documentation

## Notable Clusters

### Process Spawning Architecture (Porting Critical)
The extension uses a sophisticated multi-process architecture:
- **Electron Main**: `/src/tsServer/spawner.ts`, `/src/tsServer/serverProcess.electron.ts`
  - Spawns TypeScript server via `child_process.fork()` with stdio pipes and IPC
  - Manages request/response lifecycle via streams (Reader/Writer pattern)
  - Implements routing: `SingleTsServer` → `GetErrRoutingTsServer` → `SyntaxRoutingTsServer`
  
- **Browser**: `/src/tsServer/serverProcess.browser.ts`, `/web/src/workerSession.ts`
  - WebWorker-based execution
  - WASM cancellation tokens
  - In-memory filesystem abstractions

### Language Features (28 providers covering)
- **Completions** (32 KB)
- **Quick Fixes & Refactoring** (47 KB combined)
- **Diagnostics** (14 KB) 
- **Symbol Navigation** (definitions, references, implementations, type definitions)
- **Code Lens** (3 providers)
- **Semantic Tokens** for coloring
- **Full formatting support** (formatter, file config)
- **Advanced Features** (call hierarchy, inlay hints, linked editing)

### Electron vs Browser Split Pattern
Multiple file pairs handle platform-specific logic:
- `configuration.electron.ts` / `configuration.browser.ts`
- `serverProcess.electron.ts` / `serverProcess.browser.ts`
- `extension.ts` / `extension.browser.ts`
- `fs.electron.ts` / `fs.ts` (generic)
- Separate `/web/src/` subtree for browser implementation

### Command System (12 commands)
- Server lifecycle: restart, reload
- Navigation: go to config, open logs
- Version: select TS version, typings status
- Plugin: configuration
- User-facing: learn more about refactorings

### Plugin Infrastructure
- `/src/tsServer/plugins.ts` - Plugin discovery and loading
- `/src/api.ts` - Public API v0 for plugins (plugin configuration)
- Plugin manager integrated with `typescriptServiceClient`
