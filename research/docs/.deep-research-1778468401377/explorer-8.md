# Partition 8 of 80 — Findings

## Scope
`extensions/typescript-language-features/` (168 files, 22,571 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `extensions/typescript-language-features/src/extension.ts` (114 LOC) — Electron entry point
- `extensions/typescript-language-features/src/lazyClientHost.ts` (101 LOC) — lazy activation orchestrator
- `extensions/typescript-language-features/src/typescriptServiceClient.ts` (1309 LOC) — central hub / client
- `extensions/typescript-language-features/src/tsServer/server.ts` (703 LOC) — request routing + server abstractions
- `extensions/typescript-language-features/src/tsServer/serverProcess.electron.ts` (387 LOC) — Electron process spawn
- `extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts` (188 LOC) — WebWorker counterpart
- `extensions/typescript-language-features/src/tsServer/spawner.ts` (305 LOC) — composite server orchestration
- `extensions/typescript-language-features/src/languageProvider.ts` (175 LOC) — 28 language provider registrations
- `extensions/typescript-language-features/src/tsServer/bufferSyncSupport.ts` (771 LOC) — buffer ↔ tsserver sync
- `extensions/typescript-language-features/src/tsServer/requestQueue.ts` (97 LOC) — request priority queue
- `extensions/typescript-language-features/src/typeConverters.ts` (157 LOC) — vscode ↔ tsserver type bridge

---

### Per-File Notes

#### `extensions/typescript-language-features/src/extension.ts`

- **Role:** Electron-platform entry point for the `vscode.typescript-language-features` built-in extension. Exported `activate()` wires platform-specific factories, constructs the lazy client, and returns the public API surface.
- **Key symbols:**
  - `activate(context)` (`extension.ts:32`) — sole exported function; called by VS Code runtime on extension activation
  - `ElectronServiceProcessFactory` (`extension.ts:86`) — injected into `createLazyClientHost` as the `processFactory`; this is the object that later calls `child_process.fork`
  - `conditionalRegistration` (`extension.ts:62`) — chooses between the TSGO path (minimal, no tsserver) and the normal path (full tsserver-backed providers)
  - `createLazyClientHost` (`extension.ts:80`) — returns a `Lazy<TypeScriptServiceClientHost>` that defers actual tsserver startup until a JS/TS document is opened
  - `lazilyActivateClient` (`extension.ts:104`) — registers `onDidOpenTextDocument` listeners to trigger the lazy host
  - `deactivate()` (`extension.ts:112`) — deletes the temp directory used by tsserver log files on shutdown
- **Control flow:** `activate` → creates `PluginManager`, `NodeLogDirectoryProvider`, `DiskTypeScriptVersionProvider` → runs `conditionalRegistration` which, in the normal branch, calls `createLazyClientHost` and `lazilyActivateClient`; the actual tsserver process is not spawned until the first TS/JS document triggers the lazy value.
- **Data flow:** `ElectronServiceProcessFactory` flows into `TypeScriptServiceClientHost` → `TypeScriptServiceClient` → `TypeScriptServerSpawner.spawn()` → ultimately to `fork()`. The `onCompletionAccepted` event emitter is surfaced in the returned `Api` object.
- **Dependencies:** `ElectronServiceConfigurationProvider`, `nodeRequestCancellerFactory`, `NodeLogDirectoryProvider`, `ElectronServiceProcessFactory`, `DiskTypeScriptVersionProvider` — all Electron-specific files.

---

#### `extensions/typescript-language-features/src/lazyClientHost.ts`

- **Role:** Creates the `TypeScriptServiceClientHost` lazily (inside a `Lazy<>` wrapper) and provides `lazilyActivateClient` which triggers materialization on the first compatible document.
- **Key symbols:**
  - `createLazyClientHost()` (`lazyClientHost.ts:23`) — wraps `new TypeScriptServiceClientHost(standardLanguageDescriptions, ...)` in a `Lazy`
  - `lazilyActivateClient()` (`lazyClientHost.ts:50`) — registers `vscode.workspace.onDidOpenTextDocument`; when a TS/JS document is opened, forces `lazyClientHost.value` which triggers the spawner
  - `standardLanguageDescriptions` (`lazyClientHost.ts:42`) — list of language descriptors for TypeScript and JavaScript, imported from `configuration/languageDescription`
- **Control flow:** On extension load, only a listener is installed. First matching `onDidOpenTextDocument` event fires `lazyClientHost.value`, which instantiates `TypeScriptServiceClientHost` which in turn calls `TypeScriptServiceClient.ensureServiceStarted()`.
- **Dependencies:** `TypeScriptServiceClientHost` (out-of-partition), `standardLanguageDescriptions`.

---

#### `extensions/typescript-language-features/src/typescriptServiceClient.ts`

- **Role:** Central orchestrator. Owns the live server handle (`serverState`), starts/restarts tsserver via `TypeScriptServerSpawner`, routes all requests through `ITypeScriptServer.executeImpl`, dispatches incoming events, manages filesystem watchers delegated by tsserver, and holds `BufferSyncSupport` and `DiagnosticsManager`.
- **Key symbols:**
  - `TypeScriptServiceClient` (`typescriptServiceClient.ts:108`) — the main class; implements `ITypeScriptServiceClient`
  - `serverState: ServerState.State` (`typescriptServiceClient.ts:120`) — discriminated union (`None | Running | Errored`); holds the live `ITypeScriptServer` when running
  - `startService(resendModels)` (`typescriptServiceClient.ts:380`) — spawns a new server via `typescriptServerSpawner.spawn()`, wires `onError`/`onExit`/`onEvent` handlers, resolves `_onReady`
  - `restartTsServer()` (`typescriptServiceClient.ts:318`) — kills current server, calls `startService(true)`, re-sends open models
  - `execute(command, args, token, config)` (`typescriptServiceClient.ts:858`) — public API for feature providers; delegates to `executeImpl`
  - `executeImpl()` (`typescriptServiceClient.ts:930`) — calls `bufferSyncSupport.beforeCommand(command)` then `serverState.server.executeImpl(command, args, executeInfo)`
  - `dispatchEvent(event)` (`typescriptServiceClient.ts:971`) — switch on `event.event` covering diagnostic events, file watcher commands (`createDirectoryWatcher`, `createFileWatcher`, `closeFileWatcher`), telemetry, project loading states
  - `createFileSystemWatcher()` (`typescriptServiceClient.ts:1144`) — translates tsserver watch requests into `vscode.workspace.createFileSystemWatcher`, aggregates changes with 100ms debounce via `scheduleExecuteWatchChangeRequest()` (`typescriptServiceClient.ts:1093`)
  - `serviceExited(restart, tsVersion)` (`typescriptServiceClient.ts:632`) — crash detection; after >5 restarts within 10s, sets `hasServerFatallyCrashedTooManyTimes = true`
  - `toTsFilePath(resource)` (`typescriptServiceClient.ts:759`) — converts `vscode.Uri` to a file path string for tsserver; on web, encodes scheme+authority into a virtual path with `inMemoryResourcePrefix` (`^`)
  - `serviceStarted(resendModels)` (`typescriptServiceClient.ts:584`) — sends initial `configure` and `compilerOptionsForInferredProjects` requests; if restarting, fires `_onResendModelsRequested` and calls `bufferSyncSupport.reinitialize()`
- **Control flow:** `startService` → `typescriptServerSpawner.spawn()` returns an `ITypeScriptServer` → stored in `ServerState.Running` → event/exit/error handlers wired → `serviceStarted()` sends initial configure requests → `_onReady.resolve()` unblocks feature providers waiting via `onReady()`.
- **Data flow:** Feature providers call `client.execute(command, args, token)` → `executeImpl` → `bufferSyncSupport.beforeCommand` flushes pending buffer ops → `serverState.server.executeImpl` → queued in `RequestQueue` → written to process stdin or IPC → response dispatched back to the waiting `Promise` callback. Events from tsserver arrive via `handle.onEvent` → `dispatchEvent` → fired on typed EventEmitters (e.g., `_onDiagnosticsReceived`).
- **Dependencies:** `TypeScriptServerSpawner`, `BufferSyncSupport`, `DiagnosticsManager`, `ITypeScriptServer` (from `server.ts`), `TsServerProcessFactory` (injected).

---

#### `extensions/typescript-language-features/src/tsServer/server.ts`

- **Role:** Defines the `ITypeScriptServer` interface and three concrete implementations — `SingleTsServer`, `SyntaxRoutingTsServer`, `GetErrRoutingTsServer` — plus the internal `RequestRouter`. This is the request/response engine between client code and the OS-level process.
- **Key symbols:**
  - `ITypeScriptServer` interface (`server.ts:39`) — `executeImpl`, `onEvent`, `onExit`, `onError`, `kill()`
  - `TsServerProcess` interface (`server.ts:80`) — `write(request)`, `onData(handler)`, `onExit(handler)`, `onError(handler)`, `kill()` — the raw process abstraction
  - `SingleTsServer` (`server.ts:90`) — holds a `RequestQueue`, `CallbackMap`, and `Set<number>` of pending responses; one-to-one with a process
    - `executeImpl()` (`server.ts:228`) — creates a `Proto.Request` with a monotonic seq number, enqueues it, stores a resolve/reject callback keyed by seq, begins sending via `sendNextRequests()`
    - `dispatchMessage(message)` (`server.ts:147`) — called on every inbound message; routes `response` type to `dispatchResponse`, `event` type to `_onEvent.fire` or callback resolution for `requestCompleted` events
    - `dispatchResponse(response)` (`server.ts:209`) — fetches callback by `response.request_seq`, calls `callback.onSuccess(response)` or `callback.onError(...)`
    - `sendNextRequests()` (`server.ts:325`) — drains the queue only while `_pendingResponses.size === 0` (serial execution of non-async requests)
    - `fenceCommands` (`server.ts:367`) — static set `{'change','close','open','updateOpen'}` — these always get `RequestQueueingType.Fence`
  - `RequestRouter` (`server.ts:389`) — dispatches commands to multiple `ITypeScriptServer` instances; `sharedCommands` (`server.ts:391`) `{'change','close','open','updateOpen','configure'}` are sent to **all** servers simultaneously
  - `SyntaxRoutingTsServer` (`server.ts:547`) — wraps a syntax + semantic server pair; uses three command sets to decide routing:
    - `syntaxAlwaysCommands` (`server.ts:552`) — always go to syntax server: `navtree`, `getOutliningSpans`, `jsxClosingTag`, `selectionRange`, `format`, `formatonkey`, `docCommentTemplate`, `linkedEditingRange`
    - `semanticCommands` (`server.ts:566`) — always go to semantic: `geterr`, `geterrForProject`, `projectInfo`, `configurePlugin`
    - `syntaxAllowedCommands` (`server.ts:576`) — can go to syntax during project loading: `completions`, `definition`, `hover`, `references`, `rename`, etc.
    - `_projectLoading` flag (`server.ts:595`) — starts `true`; set to `false` when `semanticDiag`/`syntaxDiag`/`projectLoadingFinish` events arrive
  - `GetErrRoutingTsServer` (`server.ts:474`) — routes `geterr`/`geterrForProject` to a dedicated diagnostics server, all other commands to the primary
- **Control flow (SingleTsServer):** `executeImpl` → `_requestQueue.enqueue` → `sendNextRequests` → `sendRequest` → `_process.write(request)` → process produces response → `dispatchMessage` → `dispatchResponse` → stored callback resolved.
- **Data flow:** Requests carry a monotonic `seq` number. Callbacks stored in `CallbackMap<Proto.Response>` keyed by seq. Response's `request_seq` field is used to look up and remove the callback.
- **Dependencies:** `RequestQueue`, `CallbackMap`, `OngoingRequestCanceller`, `Tracer`, `TelemetryReporter`.

---

#### `extensions/typescript-language-features/src/tsServer/serverProcess.electron.ts`

- **Role:** Electron-platform implementation of `TsServerProcessFactory`. Spawns `tsserver` as a child process using either `child_process.fork` (Node IPC or stdio) or `child_process.spawn` (custom Node binary).
- **Key symbols:**
  - `ElectronServiceProcessFactory.fork()` (`serverProcess.electron.ts:342`) — primary entry; decides between `spawn` and `fork` based on whether a custom Node path is set
  - `useIpc` flag (`serverProcess.electron.ts:367`) — `true` when no custom execPath and tsserver API >= v4.6.0; enables `--useNodeIpc` arg and `stdio: ['pipe','pipe','pipe','ipc']`
  - `child_process.fork(tsServerPath, runtimeArgs, { silent: true, stdio: useIpc ? [...,'ipc'] : undefined, execArgv })` (`serverProcess.electron.ts:377`) — the actual process fork
  - `IpcChildServerProcess` (`serverProcess.electron.ts:215`) — wraps a forked process using Node IPC (`process.send` / `process.on('message')`); `write()` calls `this._process.send(serverRequest)` (`serverProcess.electron.ts:228`)
  - `StdioChildServerProcess` (`serverProcess.electron.ts:273`) — wraps a forked process using stdio; `write()` serializes as `JSON.stringify(request) + '\r\n'` to stdin (`serverProcess.electron.ts:288`); reads via `Reader<Proto.Response>` which wraps stdout
  - `ProtocolBuffer` (`serverProcess.electron.ts:34`) — ring-buffer that parses the LSP-style `Content-Length: N\r\n\r\n<body>` framing; `tryReadContentLength()` (`serverProcess.electron.ts:60`) strips the header and returns body length; `tryReadContent(length)` (`serverProcess.electron.ts:85`) extracts the JSON body string
  - `Reader<T>` (`serverProcess.electron.ts:100`) — attaches to `stdout.on('data')`, feeds data into `ProtocolBuffer`, fires `onData` events with parsed `Proto.Response` objects
  - `generatePatchedEnv()` (`serverProcess.electron.ts:144`) — sets `ELECTRON_RUN_AS_NODE=1` (when no custom execPath) and `NODE_PATH` so tsserver can resolve node_modules
  - `getExecArgv()` (`serverProcess.electron.ts:158`) — builds `--inspect`/`--inspect-brk`, `--max-old-space-size`, `--diagnostic-dir`, `--heapsnapshot-near-heap-limit`, `--heap-prof` flags
  - Kill protocol (`serverProcess.electron.ts:243`): when `useGracefulShutdown`, sends `{seq:0, type:'request', command:'exit'}` then waits 5000ms before force-kill
- **Control flow:** `fork()` → either `child_process.spawn(execPath, ...)` or `child_process.fork(tsServerPath, ...)` → wraps result in `IpcChildServerProcess` or `StdioChildServerProcess` → returned to `TypeScriptServerSpawner.spawnTsServer()` which wraps it in `SingleTsServer`.
- **Data flow (stdio path):** `SingleTsServer.write(request)` → `StdioChildServerProcess.write()` → `process.stdin.write(JSON.stringify(request) + '\r\n')` → tsserver stdout → `Reader.onLengthData` → `ProtocolBuffer.tryReadContentLength` / `tryReadContent` → `JSON.parse(msg)` → `_onData.fire(json)` → `SingleTsServer.dispatchMessage(msg)`.
- **Dependencies:** `child_process` (Node built-in), `ITypeScriptServer`, `TsServerProcess`, `TsServerProcessFactory`.

---

#### `extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts`

- **Role:** Browser/web platform implementation of `TsServerProcessFactory`. Instead of `child_process.fork`, it creates a `Worker` with three `MessageChannel` ports for tsserver protocol, file watching, and synchronous filesystem access.
- **Key symbols:**
  - `WorkerServerProcessFactory.fork()` (`serverProcess.browser.ts:39`) — creates `WorkerServerProcess`; passes `--executingFilePath` and type acquisition flags
  - `WorkerServerProcess` (`serverProcess.browser.ts:61`) — implements `TsServerProcess` using a Web Worker
  - Three `MessageChannel` pairs (`serverProcess.browser.ts:93-98`): `tsserverChannel` (JSON protocol), `watcherChannel` (file watch events), `syncChannel` (synchronous fs via `@vscode/sync-api-service`)
  - `this._worker.postMessage({args, extensionUri}, [syncChannel.port1, tsserverChannel.port1, watcherChannel.port1])` (`serverProcess.browser.ts:147`) — bootstraps the worker with its three ports via transferable ownership
  - `write(serverRequest)` (`serverProcess.browser.ts:157`) — `this._tsserver.postMessage(serverRequest)` — sends JSON over `MessagePort`
  - `_tsserver.onmessage` handler (`serverProcess.browser.ts:100`) — receives responses from the worker and calls all registered `_onDataHandlers`
  - `_watcher.onmessage` handler (`serverProcess.browser.ts:110`) — handles `watchDirectory`/`watchFile`/`dispose` messages from tsserver by delegating to `FileWatcherManager`
  - `ServiceConnection` + `ApiService` (`serverProcess.browser.ts:152-154`) — sets up the synchronous filesystem bridge using `@vscode/sync-api-common` and `@vscode/sync-api-service`
  - `kill()` (`serverProcess.browser.ts:174`) — `this._worker.terminate()`, closes all three ports
- **Data flow:** Request JSON → `_tsserver.postMessage` → Worker MessagePort → tsserver (running in Worker) produces response → `_tsserver.onmessage` → `_onDataHandlers` callbacks → `SingleTsServer.dispatchMessage`.
- **Dependencies:** `@vscode/sync-api-common/browser`, `@vscode/sync-api-service`, `FileWatcherManager`, Web Worker API.

---

#### `extensions/typescript-language-features/src/tsServer/spawner.ts`

- **Role:** Orchestrates which server topology to create based on version capabilities and configuration. Calls `TsServerProcessFactory.fork()` via `spawnTsServer()` and wraps results in routing servers.
- **Key symbols:**
  - `TypeScriptServerSpawner.spawn()` (`spawner.ts:56`) — top-level call from `TypeScriptServiceClient.startService()`
  - `CompositeServerType` enum (`spawner.ts:24`) — `Single`, `SeparateSyntax`, `DynamicSeparateSyntax`, `SyntaxOnly`
  - `getCompositeServerType()` (`spawner.ts:101`) — selects topology: if API >= v4.0.0 and `useSyntaxServer === Auto`, returns `DynamicSeparateSyntax`; if `SyntaxServer === Never`, returns `Single`
  - `shouldUseSeparateDiagnosticsServer()` (`spawner.ts:124`) — returns `configuration.enableProjectDiagnostics`
  - `spawnTsServer(kind, ...)` (`spawner.ts:130`) — calls `_factory.fork(version, args, kind, ...)` → wraps result in `new SingleTsServer(...)`
  - `getTsServerArgs()` (`spawner.ts:188`) — builds the full CLI arg array: `--serverMode partialSemantic`, `--useInferredProjectPerProjectRoot`, `--cancellationPipeName`, `--logVerbosity`, `--logFile`, `--globalPlugins`, `--pluginProbeLocations`, `--locale`, `--noGetErrOnBackgroundUpdate`, `--canUseWatchEvents` (API >= v5.44), `--enableProjectWideIntelliSenseOnWeb`
  - Topology assembly (`spawner.ts:68-98`): `DynamicSeparateSyntax` → `SyntaxRoutingTsServer({syntax, semantic})`; then if diagnostics server needed, wraps in `GetErrRoutingTsServer({getErr, primary})`
- **Control flow:** `spawn()` → `getCompositeServerType()` → `spawnTsServer()` 1-3 times → wraps in composite routers → returns single `ITypeScriptServer` to `TypeScriptServiceClient`.
- **Dependencies:** `TsServerProcessFactory`, `SingleTsServer`, `SyntaxRoutingTsServer`, `GetErrRoutingTsServer`, `OngoingRequestCancellerFactory`.

---

#### `extensions/typescript-language-features/src/languageProvider.ts`

- **Role:** Registers all 28 language feature providers for one language (TypeScript or JavaScript). Each provider is dynamically `import()`-ed and registered with `vscode.languages.register*` via its exported `register()` function.
- **Key symbols:**
  - `LanguageProvider` class (`languageProvider.ts:25`) — one instance per language description; `constructor` defers `registerProviders()` until `client.onReady()`
  - `registerProviders()` (`languageProvider.ts:64`) — `Promise.all` of 28 dynamic `import()` calls; each resolves to a module with a `register()` function that calls `vscode.languages.register*`
  - The 28 providers registered (`languageProvider.ts:70-99`): `callHierarchy`, `implementationsCodeLens`, `referencesCodeLens`, `completions`, `copyPaste`, `definitions`, `directiveCommentCompletions`, `documentHighlight`, `documentSymbol`, `fileReferences`, `fixAll`, `folding`, `formatting`, `hover`, `implementations`, `inlayHints`, `jsDocCompletions`, `linkedEditing`, `organizeImports`, `quickFix`, `refactor`, `references`, `rename`, `semanticTokens`, `signatureHelp`, `smartSelect`, `sourceDefinition`, `tagClosing`, `typeDefinitions`
  - `documentSelector` getter (`languageProvider.ts:51`) — builds `{semantic, syntax}` document filter arrays; semantic filters include only `fileSchemes.getSemanticSupportedSchemes()` scheme prefixes
  - `diagnosticsReceived()` (`languageProvider.ts:140`) — validates diagnostic kind against client capabilities (e.g., suppresses semantic diags on web without shared array buffers) then calls `client.diagnosticsManager.updateDiagnostics()`
  - `triggerAllDiagnostics()` (`languageProvider.ts:137`) — calls `client.bufferSyncSupport.requestAllDiagnostics()`
- **Control flow:** Construction → config listeners set up → `client.onReady(() => registerProviders())` → all 28 modules loaded in parallel → each `register()` returns a `Disposable` which is tracked in the parent `DisposableStore`.
- **Dependencies:** `TypeScriptServiceClient`, `DiagnosticsManager`, `FileConfigurationManager`, `TypingsStatus`, all 28 `languageFeatures/*.ts` modules.

---

#### `extensions/typescript-language-features/src/tsServer/bufferSyncSupport.ts`

- **Role:** Keeps every open TS/JS document synchronized with tsserver by sending `open`/`close`/`change` operations, and drives diagnostic requests (`geterr`) with debouncing.
- **Key symbols:**
  - `BufferSyncSupport` (`bufferSyncSupport.ts:462`) — top-level class; created in `TypeScriptServiceClient` constructor
  - `BufferSynchronizer` (`bufferSyncSupport.ts:67`) — accumulates pending `OpenOperation`, `CloseOperation`, `ChangeOperation` per resource; `flush()` (`bufferSyncSupport.ts:119`) batches them into a single `execute('updateOpen', {changedFiles, closedFiles, openFiles})` call
  - `beforeCommand(command)` (`bufferSyncSupport.ts:111`) — called by `TypeScriptServiceClient.executeImpl` before every request; flushes pending buffer ops so tsserver has up-to-date content
  - `SyncedBuffer` (`bufferSyncSupport.ts:160`) — wraps a `vscode.TextDocument`; `open()` (`bufferSyncSupport.ts:171`) sends `OpenRequestArgs` with full `fileContent`; `onContentChanged(events)` (`bufferSyncSupport.ts:241`) calls `synchronizer.change()` which converts `vscode.TextDocumentContentChangeEvent[]` to `Proto.CodeEdit[]` (reversed to end-of-document order)
  - `listen()` (`bufferSyncSupport.ts:527`) — registers `onDidOpenTextDocument`, `onDidCloseTextDocument`, `onDidChangeTextDocument`, `onDidChangeVisibleTextEditors`
  - `openTextDocument(document)` (`bufferSyncSupport.ts:594`) — creates `SyncedBuffer`, adds to `syncedBuffers` map, calls `syncedBuffer.open()` and schedules `requestDiagnostic`
  - `requestDiagnostic(buffer)` (`bufferSyncSupport.ts:704`) — sets `pendingDiagnostics` entry; delay scales with line count: `min(max(ceil(lineCount/20), 300), 800)` ms
  - `sendPendingDiagnostics()` (`bufferSyncSupport.ts:720`) — merges pending set with visible ranges, executes `GetErrRequest.executeGetErrRequest(client, orderedFileSet, onDone)`
  - `GetErrRequest` (`bufferSyncSupport.ts:275`) — fires either `geterrForProject` or `geterr` (with optional region ranges for API >= v5.6.0) via `client.executeAsync`
  - `TabResourceTracker` (`bufferSyncSupport.ts:370`) — tracks which URIs have open editor tabs (via `vscode.window.tabGroups`) to gate whether to validate a buffer
  - `interruptGetErr<R>(f)` (`bufferSyncSupport.ts:631`) — cancels any in-flight `geterr` request, runs `f()`, then re-triggers diagnostics; used by `TypeScriptServiceClient.execute` to avoid blocking user-facing requests
- **Data flow:** `vscode.TextDocumentContentChangeEvent` → `BufferSynchronizer.change()` → `Proto.FileCodeEdits` stored pending → `beforeCommand` flushes → `execute('updateOpen', ...)` → tsserver processes edits → tsserver emits `syntaxDiag`/`semanticDiag` events → `TypeScriptServiceClient.dispatchEvent` → `_onDiagnosticsReceived` → `LanguageProvider.diagnosticsReceived` → `DiagnosticsManager.updateDiagnostics`.
- **Dependencies:** `ITypeScriptServiceClient`, `typeConverters.Position.toLocation`, `ResourceMap`, `Delayer`.

---

#### `extensions/typescript-language-features/src/tsServer/requestQueue.ts`

- **Role:** Priority queue for outbound tsserver requests. Implements three priority tiers: `Normal`, `LowPriority`, `Fence`.
- **Key symbols:**
  - `RequestQueueingType` enum (`requestQueue.ts:8`) — `Normal=1`, `LowPriority=2`, `Fence=3`
  - `RequestQueue.enqueue(item)` (`requestQueue.ts:43`) — for `Normal` items, scans backward past `LowPriority` items and inserts in front of them; all other types pushed to end
  - `RequestQueue.createRequest(command, args)` (`requestQueue.ts:89`) — assigns monotonically incrementing `seq` number; produces `{seq, type:'request', command, arguments: args}`
  - `tryDeletePendingRequest(seq)` (`requestQueue.ts:79`) — linear scan to cancel a not-yet-sent request
- **Control flow:** `SingleTsServer.executeImpl` → `createRequest` → `enqueue` → `sendNextRequests` dequeues one at a time (FIFO after priority reordering) → `write` to process.
- **Dependencies:** `Proto.Request` type only.

---

#### `extensions/typescript-language-features/src/typeConverters.ts`

- **Role:** Stateless conversion utilities between `vscode.*` types and `Proto.*` (tsserver protocol) types.
- **Key symbols:**
  - `Range.fromTextSpan(span)` (`typeConverters.ts:16`) — converts `Proto.TextSpan` (1-based line/offset) to `vscode.Range` (0-based line/character)
  - `Range.toTextSpan(range)` (`typeConverters.ts:19`) — inverse: `vscode.Range` → `Proto.TextSpan`
  - `Position.fromLocation(tslocation)` (`typeConverters.ts:56`) — `{line-1, offset-1}` → `vscode.Position`; note the -1 offset adjustment throughout
  - `Position.toLocation(vsPosition)` (`typeConverters.ts:59`) — `vscode.Position` → `{line+1, offset+1}`
  - `WorkspaceEdit.fromFileCodeEdits(client, edits)` (`typeConverters.ts:85`) — iterates `Proto.FileCodeEdits[]`, calls `client.toResource(edit.fileName)` to get `vscode.Uri`, builds `vscode.WorkspaceEdit`
  - `SymbolKind.fromProtocolScriptElementKind(kind)` (`typeConverters.ts:110`) — maps `Proto.ScriptElementKind` strings to `vscode.SymbolKind` enum values
- **Data flow:** Used extensively by all 28 feature providers when constructing request arguments (converting `vscode.Position` → `Proto.Location`) and when converting tsserver responses back into `vscode.*` types for the VS Code API.
- **Dependencies:** `vscode` API, `Proto` protocol types, `PConst` protocol constants, `ITypeScriptServiceClient`.

---

### Cross-Cutting Synthesis

The `typescript-language-features` extension implements a layered pipeline: the Electron `activate()` entry point injects platform-specific factories (`ElectronServiceProcessFactory`) into `TypeScriptServiceClient` via `TypeScriptServiceClientHost`; `TypeScriptServerSpawner` uses those factories to call `child_process.fork` (or `Worker` on web) and construct up to three `SingleTsServer` instances composed into routing wrappers (`SyntaxRoutingTsServer`, `GetErrRoutingTsServer`). Every feature provider calls `client.execute(command, args, token)`, which flows through `BufferSyncSupport.beforeCommand` (ensuring buffer state is flushed to tsserver), then into `SingleTsServer.executeImpl` where requests are queued with three-tier priority, serialized as JSON (or via IPC), and matched back to their `Promise` callbacks by monotonic sequence number. Buffer changes from VS Code editors travel through `BufferSynchronizer` → batched `updateOpen` requests → tsserver → `syntaxDiag`/`semanticDiag` events → `LanguageProvider.diagnosticsReceived`. The `typeConverters.ts` module is the universal type bridge, handling the ±1 coordinate system difference between VS Code (0-based) and tsserver (1-based). Platform duality is achieved by the `TsServerProcessFactory` interface: Electron uses `child_process.fork` with `ProtocolBuffer` framing; the browser uses a Web Worker with three `MessageChannel` ports and `@vscode/sync-api-service` for synchronous filesystem access.

---

### Out-of-Partition References

- `extensions/typescript-language-features/src/typeScriptServiceClientHost.ts` — instantiates `TypeScriptServiceClient` and `LanguageProvider` instances for each language description
- `extensions/typescript-language-features/src/configuration/configuration.ts` — `TypeScriptServiceConfiguration`, `ServiceConfigurationProvider`, `SyntaxServerConfiguration`
- `extensions/typescript-language-features/src/configuration/languageDescription.ts` — `standardLanguageDescriptions`, `LanguageDescription` type
- `extensions/typescript-language-features/src/configuration/documentSelector.ts` — `DocumentSelector` type used in provider registration
- `extensions/typescript-language-features/src/configuration/fileSchemes.ts` — `getSemanticSupportedSchemes()`, `disabledSchemes`
- `extensions/typescript-language-features/src/tsServer/cancellation.ts` — `OngoingRequestCanceller`, `OngoingRequestCancellerFactory` interfaces
- `extensions/typescript-language-features/src/tsServer/cancellation.electron.ts` — `nodeRequestCancellerFactory`; implements pipe-based cancellation
- `extensions/typescript-language-features/src/tsServer/callbackMap.ts` — `CallbackMap<T>` used in `SingleTsServer`
- `extensions/typescript-language-features/src/tsServer/api.ts` — `API` version class with `gte()`/`lt()` comparison
- `extensions/typescript-language-features/src/tsServer/versionManager.ts` — `TypeScriptVersionManager`
- `extensions/typescript-language-features/src/tsServer/versionProvider.electron.ts` — `DiskTypeScriptVersionProvider`
- `extensions/typescript-language-features/src/tsServer/nodeManager.ts` — `NodeVersionManager`
- `extensions/typescript-language-features/src/tsServer/plugins.ts` — `PluginManager`
- `extensions/typescript-language-features/src/tsServer/protocol/protocol.ts` — full tsserver protocol type definitions (`Proto.*`)
- `extensions/typescript-language-features/src/tsServer/protocol/protocol.const.ts` — `EventName`, `PConst.Kind`, `PConst.KindModifiers`
- `extensions/typescript-language-features/src/tsServer/fileWatchingManager.ts` — `FileWatcherManager` used in browser `WorkerServerProcess`
- `extensions/typescript-language-features/src/languageFeatures/diagnostics.ts` — `DiagnosticsManager`, `DiagnosticKind`
- `extensions/typescript-language-features/src/languageFeatures/fileConfigurationManager.ts` — `FileConfigurationManager`
- `extensions/typescript-language-features/src/languageFeatures/completions.ts` — `MyCompletionItem`, `register()` (28KB feature provider)
- `extensions/typescript-language-features/src/languageFeatures/refactor.ts` — refactor provider (27KB)
- `extensions/typescript-language-features/src/languageFeatures/quickFix.ts` — quick-fix provider (19KB)
- `extensions/typescript-language-features/src/typescriptService.ts` — `ITypeScriptServiceClient`, `ClientCapability`, `ServerResponse`, `TypeScriptRequests` map type
- `extensions/typescript-language-features/src/utils/resourceMap.ts` — `ResourceMap<T>` used throughout
- `extensions/typescript-language-features/src/utils/async.ts` — `Delayer` used in `BufferSyncSupport`
- `extensions/typescript-language-features/src/utils/dispose.ts` — `Disposable` base class, `DisposableStore`
- `extensions/typescript-language-features/src/utils/lazy.ts` — `Lazy<T>` wrapper
- `extensions/typescript-language-features/src/logging/tracer.ts` — `Tracer` used in `SingleTsServer`
- `extensions/typescript-language-features/src/logging/telemetry.ts` — `TelemetryReporter`, `VSCodeTelemetryReporter`
- `extensions/typescript-language-features/src/ui/typingsStatus.ts` — `TypingsStatus` passed to `LanguageProvider`
- `extensions/typescript-language-features/src/ui/activeJsTsEditorTracker.ts` — `ActiveJsTsEditorTracker`
- `@vscode/sync-api-common/browser` — synchronous API bridge for web Worker filesystem access
- `@vscode/sync-api-service` — `ApiService`, `Requests` for web Worker

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder - Partition 8: TypeScript Language Features Extension
## Process Forking and Language Server Registration Patterns

### Overview
The TypeScript language features extension (extensions/typescript-language-features/) demonstrates patterns for:
1. Spawning and managing TypeScript server processes across different platforms (Electron, Browser/Web)
2. Registering language providers with VS Code's language API
3. Conditional registration based on capabilities and configuration
4. Process communication via IPC and stdio channels

---

## Patterns Found

#### Pattern 1: Process Factory Interface with Platform Implementations
**Where:** `src/tsServer/server.ts:68-78`
**What:** Abstraction for spawning TypeScript server processes with implementations for Electron and Browser/Web.

```typescript
export interface TsServerProcessFactory {
	fork(
		version: TypeScriptVersion,
		args: readonly string[],
		kind: TsServerProcessKind,
		configuration: TypeScriptServiceConfiguration,
		versionManager: TypeScriptVersionManager,
		nodeVersionManager: NodeVersionManager,
		tsServerLog: TsServerLog | undefined,
	): TsServerProcess;
}
```

**Variations / call-sites:** 
- `src/tsServer/serverProcess.electron.ts:342` - ElectronServiceProcessFactory implementation
- `src/tsServer/serverProcess.browser.ts:33` - WorkerServerProcessFactory implementation
- `src/tsServer/spawner.ts:161` - Usage in spawnTsServer method

---

#### Pattern 2: Electron Process Forking with IPC/Stdio Abstraction
**Where:** `src/tsServer/serverProcess.electron.ts:343-386`
**What:** Creates child processes using node.js child_process.fork() with environment setup, graceful shutdown, and dual communication modes (IPC or stdio).

```typescript
fork(
	version: TypeScriptVersion,
	args: readonly string[],
	kind: TsServerProcessKind,
	configuration: TypeScriptServiceConfiguration,
	versionManager: TypeScriptVersionManager,
	nodeVersionManager: NodeVersionManager,
	_tsserverLog: TsServerLog | undefined,
): TsServerProcess {
	let tsServerPath = version.tsServerPath;

	if (!fs.existsSync(tsServerPath)) {
		vscode.window.showWarningMessage(vscode.l10n.t("The path {0} doesn\'t point to a valid tsserver install. Falling back to bundled TypeScript version.", tsServerPath));
		versionManager.reset();
		tsServerPath = versionManager.currentVersion.tsServerPath;
	}

	const execPath = nodeVersionManager.currentVersion;
	const env = generatePatchedEnv(process.env, tsServerPath, !!execPath);
	const runtimeArgs = [...args];
	const execArgv = getExecArgv(kind, configuration);
	const useGracefulShutdown = configuration.heapProfile.enabled;
	const useIpc = !execPath && version.apiVersion?.gte(API.v460);
	if (useIpc) {
		runtimeArgs.push('--useNodeIpc');
	}

	const childProcess = execPath ?
		child_process.spawn(execPath, [...execArgv, tsServerPath, ...runtimeArgs], {
			windowsHide: true,
			cwd: undefined,
			env,
		}) :
		child_process.fork(tsServerPath, runtimeArgs, {
			silent: true,
			cwd: undefined,
			env,
			execArgv,
			stdio: useIpc ? ['pipe', 'pipe', 'pipe', 'ipc'] : undefined,
		});

	return useIpc ? new IpcChildServerProcess(childProcess, useGracefulShutdown) : new StdioChildServerProcess(childProcess, useGracefulShutdown);
}
```

**Variations / call-sites:**
- Uses `child_process.fork()` for direct node process spawning
- Uses `child_process.spawn()` when custom Node version is provided
- `src/tsServer/serverProcess.electron.ts:158-190` - getExecArgv builds debug/memory/profiling arguments

---

#### Pattern 3: Web Worker-Based Process Implementation
**Where:** `src/tsServer/serverProcess.browser.ts:39-59`
**What:** Web/Browser variant using SharedArrayBuffer, Web Workers, and MessageChannel for communication with FileWatcherManager for filesystem events.

```typescript
public fork(
	version: TypeScriptVersion,
	args: readonly string[],
	kind: TsServerProcessKind,
	configuration: TypeScriptServiceConfiguration,
	_versionManager: TypeScriptVersionManager,
	_nodeVersionManager: NodeVersionManager,
	tsServerLog: TsServerLog | undefined,
) {
	const tsServerPath = version.tsServerPath;
	const launchArgs = [
		...args,
		// Explicitly give TS Server its path so it can load local resources
		'--executingFilePath', tsServerPath,
		// Enable/disable web type acquisition
		(configuration.webTypeAcquisitionEnabled && supportsReadableByteStreams() ? '--experimentalTypeAcquisition' : '--disableAutomaticTypingAcquisition'),
	];

	return new WorkerServerProcess(kind, tsServerPath, this._extensionUri, launchArgs, tsServerLog, this._logger);
}
```

**Variations / call-sites:**
- `src/tsServer/serverProcess.browser.ts:61-187` - WorkerServerProcess creates Worker, FileWatcherManager, MessageChannels for tsserver/watcher/syncFs
- Uses MessageChannel for three separate communication channels: sync TS server communication, async watcher events, filesystem sync

---

#### Pattern 4: Graceful Process Shutdown with Timeout
**Where:** `src/tsServer/serverProcess.electron.ts:215-271` (IpcChildServerProcess) and `273-340` (StdioChildServerProcess)
**What:** Implements graceful shutdown protocol with 5-second timeout fallback to force kill.

```typescript
class IpcChildServerProcess extends Disposable implements TsServerProcess {
	private _killTimeout: NodeJS.Timeout | undefined;
	private _isShuttingDown = false;

	kill(): void {
		if (!this._useGracefulShutdown) {
			this._process.kill();
			return;
		}

		if (this._isShuttingDown) {
			return;
		}
		this._isShuttingDown = true;

		try {
			this._process.send(tsServerExitRequest);
		} catch {
			this._process.kill();
			return;
		}

		this._killTimeout = setTimeout(() => this._process.kill(), gracefulExitTimeout);
		this._killTimeout.unref?.();
	}
}
```

**Variations / call-sites:**
- Both IPC and Stdio variants implement identical shutdown protocol
- `src/tsServer/serverProcess.electron.ts:27` - gracefulExitTimeout = 5000ms
- `src/tsServer/serverProcess.electron.ts:28-32` - tsServerExitRequest protocol definition

---

#### Pattern 5: Composite Server Architecture (Semantic/Syntax Routing)
**Where:** `src/tsServer/spawner.ts:56-99`
**What:** Spawns multiple TypeScript server instances (Main/Semantic/Syntax/Diagnostics) and routes requests based on capability and configuration.

```typescript
public spawn(
	version: TypeScriptVersion,
	capabilities: ClientCapabilities,
	configuration: TypeScriptServiceConfiguration,
	pluginManager: PluginManager,
	cancellerFactory: OngoingRequestCancellerFactory,
	delegate: TsServerDelegate,
): ITypeScriptServer {
	let primaryServer: ITypeScriptServer;
	const serverType = this.getCompositeServerType(version, capabilities, configuration);
	const shouldUseSeparateDiagnosticsServer = this.shouldUseSeparateDiagnosticsServer(configuration);

	switch (serverType) {
		case CompositeServerType.SeparateSyntax:
		case CompositeServerType.DynamicSeparateSyntax:
			{
				const enableDynamicRouting = !shouldUseSeparateDiagnosticsServer && serverType === CompositeServerType.DynamicSeparateSyntax;
				primaryServer = new SyntaxRoutingTsServer({
					syntax: this.spawnTsServer(TsServerProcessKind.Syntax, version, configuration, pluginManager, cancellerFactory),
					semantic: this.spawnTsServer(TsServerProcessKind.Semantic, version, configuration, pluginManager, cancellerFactory),
				}, delegate, enableDynamicRouting);
				break;
			}
		case CompositeServerType.Single:
			{
				primaryServer = this.spawnTsServer(TsServerProcessKind.Main, version, configuration, pluginManager, cancellerFactory);
				break;
			}
		case CompositeServerType.SyntaxOnly:
			{
				primaryServer = this.spawnTsServer(TsServerProcessKind.Syntax, version, configuration, pluginManager, cancellerFactory);
				break;
			}
	}

	if (shouldUseSeparateDiagnosticsServer) {
		return new GetErrRoutingTsServer({
			getErr: this.spawnTsServer(TsServerProcessKind.Diagnostics, version, configuration, pluginManager, cancellerFactory),
			primary: primaryServer,
		}, delegate);
	}

	return primaryServer;
}
```

**Variations / call-sites:**
- `src/tsServer/spawner.ts:101-122` - CompositeServerType selection logic
- Routing strategies: SeparateSyntax, DynamicSeparateSyntax (with optional routing), Single, SyntaxOnly, with optional GetErrRoutingTsServer layer

---

#### Pattern 6: Conditional Language Provider Registration
**Where:** `src/languageFeatures/definitions.ts:63-73`
**What:** Wraps vscode.languages.registerDefinitionProvider in conditional registration based on client capabilities and configuration.

```typescript
export function register(
	selector: DocumentSelector,
	client: ITypeScriptServiceClient,
) {
	return conditionalRegistration([
		requireSomeCapability(client, ClientCapability.EnhancedSyntax, ClientCapability.Semantic),
	], () => {
		return vscode.languages.registerDefinitionProvider(selector.syntax,
			new TypeScriptDefinitionProvider(client));
	});
}
```

**Variations / call-sites:**
- Multiple provider types with same pattern:
  - `hover.ts:105-116` - registerHoverProvider with requireSomeCapability
  - `completions.ts:930-946` - registerCompletionItemProvider with requireSomeCapability
  - `refactor.ts:774-782` - registerCodeActionsProvider with requireSomeCapability
  - `formatting.ts:90-103` - registerOnTypeFormattingEditProvider with requireGlobalUnifiedConfig
  - `semanticTokens.ts:15-25` - registerDocumentRangeSemanticTokensProvider

---

#### Pattern 7: Multi-Feature Provider Registration with Disposable Composition
**Where:** `src/languageFeatures/formatting.ts:87-103`
**What:** Registers multiple related providers (on-type formatting + document range formatting) using vscode.Disposable.from() composition.

```typescript
export function register(
	selector: DocumentSelector,
	language: LanguageDescription,
	client: ITypeScriptServiceClient,
	fileConfigurationManager: FileConfigurationManager
) {
	return conditionalRegistration([
		requireGlobalUnifiedConfig('format.enabled', { fallbackSection: language.id, fallbackSubSectionNameOverride: 'format.enable' }),
	], () => {
		const formattingProvider = new TypeScriptFormattingProvider(client, fileConfigurationManager);
		return vscode.Disposable.from(
			vscode.languages.registerOnTypeFormattingEditProvider(selector.syntax, formattingProvider, ';', '}', '\n'),
			vscode.languages.registerDocumentRangeFormattingEditProvider(selector.syntax, formattingProvider),
		);
	});
}
```

**Variations / call-sites:**
- `documentHighlight.ts:86-87` - Registers both registerDocumentHighlightProvider and registerMultiDocumentHighlightProvider

---

#### Pattern 8: Condition-Based Dynamic Registration System
**Where:** `src/languageFeatures/util/dependentRegistration.ts:12-141`
**What:** Implements reactive registration that responds to capability/configuration changes without requiring extension reload.

```typescript
export class Condition extends Disposable {
	private _value: boolean;

	constructor(
		private readonly getValue: () => boolean,
		onUpdate: (handler: () => void) => void,
	) {
		super();
		this._value = this.getValue();

		onUpdate(() => {
			const newValue = this.getValue();
			if (newValue !== this._value) {
				this._value = newValue;
				this._onDidChange.fire();
			}
		});
	}

	public get value(): boolean { return this._value; }

	private readonly _onDidChange = this._register(new vscode.EventEmitter<void>());
	public readonly onDidChange = this._onDidChange.event;
}

export function conditionalRegistration(
	conditions: readonly Condition[],
	doRegister: () => vscode.Disposable,
	elseDoRegister?: () => vscode.Disposable
): vscode.Disposable {
	return new ConditionalRegistration(conditions, doRegister, elseDoRegister);
}

export function requireSomeCapability(
	client: ITypeScriptServiceClient,
	...capabilities: readonly ClientCapability[]
) {
	return new Condition(
		() => capabilities.some(requiredCapability => client.capabilities.has(requiredCapability)),
		client.onDidChangeCapabilities
	);
}
```

**Variations / call-sites:**
- `requireMinVersion()` - Version-based conditions tied to `client.onTsServerStarted`
- `requireHasModifiedUnifiedConfig()` - Configuration modification detection
- `requireGlobalUnifiedConfig()` - Global config value presence
- `requireHasVsCodeExtension()` - Extension availability checks

---

## Summary: Implications for Tauri/Rust Port

### Process Management (High Priority)
The codebase demonstrates sophisticated process management patterns:
- **Platform abstraction**: ElectronServiceProcessFactory vs WorkerServerProcessFactory shows how to abstract platform-specific launching
- **IPC vs Stdio**: Supports both message-based (IPC) and stream-based (stdio) communication, important for choosing Rust communication strategy
- **Graceful shutdown**: 5-second timeout protocol before force kill - would need Rust equivalent using process signals/IPC
- **Composite routing**: Multiple specialized processes (Semantic, Syntax, Diagnostics) running simultaneously with request routing

### Language Provider Registration Pattern
The vscode.languages.register* API is heavily used across 30+ call sites with:
- **Conditional registration**: All providers check capabilities before registering (must translate to Tauri equivalent)
- **Disposable management**: Providers return disposables for cleanup (would map to Rust Drop trait)
- **Multiple trigger patterns**: Providers register with trigger characters, metadata, document selectors

### Process Arguments and Configuration
- Server spawning passes extensive configuration: debug ports, memory limits, heap profiling, tracing directories, locale, plugins, npm locations
- Environmental variable patching for NODE_PATH, ELECTRON_RUN_AS_NODE
- Version-specific features (API version gating: v400, v401, v460, v470, v544, v590)

### Worker/Multi-Process Architecture
Three distinct server types (Main, Syntax, Semantic, Diagnostics) with routing logic based on:
- Version capabilities
- Configuration (useSyntaxServer: Always/Never/Auto)
- Client capabilities (Semantic support)
- enableProjectDiagnostics flag

### Critical APIs to Replace
- `child_process.fork()` / `child_process.spawn()`
- `vscode.languages.register*Provider()` family (30+ variations)
- `vscode.Disposable`, event emitters
- File watching and configuration monitoring

## External References
<!-- Source: codebase-online-researcher sub-agent -->
#### TypeScript `ts.server.protocol` (bundled with `typescript` npm package v6.x)

**Docs:** https://github.com/microsoft/TypeScript/wiki/Standalone-Server-%28tsserver%29
**Protocol source:** https://raw.githubusercontent.com/microsoft/TypeScript/main/src/server/protocol.ts

**Relevant behaviour — message framing and transport:**

The extension does NOT use LSP or `vscode-languageclient`. It communicates directly with tsserver using TypeScript's own proprietary JSON protocol. The local protocol shim at `extensions/typescript-language-features/src/tsServer/protocol/protocol.d.ts:5` re-exports the entire `ts.server.protocol` namespace from the installed `typescript` package (currently v6.0.0-dev.20260416):

```ts
import type ts from '../../../../node_modules/typescript/lib/typescript';
export = ts.server.protocol;
```

**Wire format — tsserver vs LSP:**

tsserver's framing is superficially similar to LSP (both use HTTP-style `Content-Length` headers before a JSON body over stdio) but differs in every semantic detail:

| Dimension | tsserver | LSP (JSON-RPC 2.0) |
|---|---|---|
| Envelope field | `seq` (auto-incrementing int) | `id` (int or string or null) |
| Message type tag | `"type": "request" \| "response" \| "event"` | method name encodes direction; no `type` field |
| Protocol version | none | `"jsonrpc": "2.0"` required |
| Command/method names | `quickinfo`, `completionInfo`, `geterr`, `updateOpen`, … | `textDocument/completion`, `textDocument/definition`, … |
| Response correlation | `request_seq` in response | `id` echoed in response |
| Cancellation | Named pipe: `--cancellationPipeName` arg + writing a sentinel file | `$/cancelRequest` JSON-RPC notification |
| Server-push events | `type: "event"` messages with same Content-Length framing | `type: "notification"` (no `id`) |
| Async diagnostics | `geterr` fires `semanticDiag` / `syntaxDiag` events | `textDocument/publishDiagnostics` push notification |

The `ProtocolBuffer` / `Reader<T>` classes in `serverProcess.electron.ts:34-141` implement the Content-Length framing parser on the client side. The `StdioChildServerProcess.write()` method at line 288 serialises each request as `JSON.stringify(request) + '\r\n'` (no header prefix required for requests — only responses and events from the server carry `Content-Length`).

**Transport modes (Electron path):**

Two transport options are forked at `serverProcess.electron.ts:367-385`:
- **IPC mode** (TypeScript ≥ 4.6.0, no custom Node path): `child_process.fork` with `--useNodeIpc`, communication via `process.send()` / `process.on('message')` — JSON is serialised by Node's IPC channel automatically (`IpcChildServerProcess`).
- **Stdio mode**: `child_process.spawn` (or `fork` with `silent: true`) plus the `Reader<T>` Content-Length framing parser (`StdioChildServerProcess`).

**Transport modes (browser/WASM path):**

`serverProcess.browser.ts` runs tsserver as a `Worker`. Communication uses three `MessageChannel` ports: `_tsserver` for synchronous request/response, `_watcher` for file-system watch events, and `_syncFs` for synchronous filesystem access. The `@vscode/sync-api-common` / `@vscode/sync-api-service` packages mediate that synchronous fs channel via a `SharedArrayBuffer`-based RPC.

**Commands used:**

`typescriptService.ts:37-99` enumerates all 40+ commands the VS Code client issues, covering completions (`completionInfo`), diagnostics (`geterr`, `geterrForProject`), navigation (`definition`, `references`, `navtree`), refactoring (`getApplicableRefactors`, `getEditsForRefactor`), organise imports, inlay hints, call hierarchy, and more. None map to LSP methods.

**Server multiplexing:**

`spawner.ts` forks between one and three concurrent tsserver processes: a `syntax` process (partialSemantic mode via `--serverMode partialSemantic` or `--syntaxOnly`) for fast IDE features while loading, a `semantic` process for full analysis, and optionally a separate `diagnostics` process. `SyntaxRoutingTsServer` and `GetErrRoutingTsServer` in `server.ts` fan requests out to the appropriate process based on the command's declared routing — fence commands (`change`, `close`, `open`, `updateOpen`) are sent to both servers to keep state in sync.

**Cancellation:**

Electron path: `cancellation.electron.ts` writes an empty file at `<cancellationPipeName><seq>` to signal cancellation; tsserver polls for that file. Browser/web path: `Cancellation.addData()` from `@vscode/sync-api-common` sets a flag in a `SharedArrayBuffer` that tsserver reads synchronously from its worker thread.

**Version gating:**

`api.ts` tracks TypeScript version constants from v3.8.0 through v5.9.0 and gates individual features behind `apiVersion.gte(API.vXYZ)` checks (e.g. IPC transport requires ≥ 4.6.0, `--serverMode partialSemantic` requires ≥ 4.0.1).

**Where used:**

- `src/tsServer/protocol/protocol.d.ts:5` — protocol type re-export
- `src/tsServer/serverProcess.electron.ts:22-97` — Content-Length frame parser (`ProtocolBuffer`, `Reader<T>`)
- `src/tsServer/serverProcess.electron.ts:286-388` — two `TsServerProcess` implementations (IPC vs stdio)
- `src/tsServer/serverProcess.browser.ts:61-187` — WASM Worker transport
- `src/typescriptService.ts:37-99` — full enumeration of all tsserver commands used
- `src/tsServer/server.ts:79-88` — `TsServerProcess` interface (write/onData abstraction)
- `src/tsServer/spawner.ts:188-260` — argument assembly (`--cancellationPipeName`, `--serverMode`, `--logFile`, etc.)

**Porting implications for Tauri/Rust:**

The `typescript-language-features` extension is the single most tsserver-specific component in VS Code. Porting it to Tauri presents a hard architectural choice: (a) keep tsserver as an external Node.js subprocess and re-implement the Content-Length framing + request/response/event dispatch in Rust (the wire protocol is documented and stable); or (b) migrate to the `typescript-language-server` bridge project (https://github.com/typescript-language-server/typescript-language-server, maintained, v5.2.0 released May 2026) which wraps tsserver in a standard LSP server, allowing a Rust host to use generic `tower-lsp` or `lsp-types` crates instead of reimplementing the proprietary protocol. Option (b) forfeits some tsserver-exclusive features (e.g. `mapCode`, `preparePasteEdits`, region semantic diagnostics) that have no LSP equivalent yet, but option (a) requires maintaining a bespoke protocol adapter. A third path is to wait for TypeScript 7 (currently being rewritten in Go as `typescript-go`), which Microsoft states will include a native LSP implementation and is intended to supersede the bridge layer entirely.

**Sources consulted:**

- https://github.com/microsoft/TypeScript/wiki/Standalone-Server-%28tsserver%29
- https://raw.githubusercontent.com/microsoft/TypeScript/main/src/server/protocol.ts (3,321 lines, 346 exported types)
- https://github.com/typescript-language-server/typescript-language-server
- https://www.chia1104.dev/en-US/posts/typescript-tsserver-lsp-development-mindset (HTTP 403; content summarised via WebSearch)
- WebSearch: "tsserver protocol vs LSP Language Server Protocol differences JSON framing stdio 2024"

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
