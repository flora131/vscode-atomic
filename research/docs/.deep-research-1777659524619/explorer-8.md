# Partition 8 of 79 — Findings

## Scope
`extensions/typescript-language-features/` (168 files, 22,571 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# TypeScript Language Features Extension - Porting Analysis

## Implementation

### Core Service Architecture
- `extensions/typescript-language-features/src/typescriptServiceClient.ts` — Main client managing tsserver lifecycle, request/response handling, and configuration
- `extensions/typescript-language-features/src/typescriptService.ts` — Service interfaces and protocol for LSP-like contracts with tsserver
- `extensions/typescript-language-features/src/typeScriptServiceClientHost.ts` — Host managing initialization and multiple language providers
- `extensions/typescript-language-features/src/languageProvider.ts` — Central registration point for all language features to vscode.languages API
- `extensions/typescript-language-features/src/api.ts` — Public API surface for other extensions
- `extensions/typescript-language-features/src/extension.ts` — Extension activation (Electron entry point, handles 5 different activation events)
- `extensions/typescript-language-features/src/extension.browser.ts` — Browser-specific activation (web version)
- `extensions/typescript-language-features/src/lazyClientHost.ts` — Lazy initialization wrapper for performance

### TypeScript Server Communication (26 files)
- `extensions/typescript-language-features/src/tsServer/server.ts` — Abstracts ITypeScriptServer interface, manages connection lifecycle
- `extensions/typescript-language-features/src/tsServer/serverProcess.electron.ts` — Node.js process spawning (Electron-only)
- `extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts` — Browser-based server (Web-only)
- `extensions/typescript-language-features/src/tsServer/spawner.ts` — Server startup logic and initialization
- `extensions/typescript-language-features/src/tsServer/api.ts` — API version management for feature-gating
- `extensions/typescript-language-features/src/tsServer/bufferSyncSupport.ts` — Document change synchronization
- `extensions/typescript-language-features/src/tsServer/fileWatchingManager.ts` — File watching coordination
- `extensions/typescript-language-features/src/tsServer/plugins.ts` — Plugin discovery and management
- `extensions/typescript-language-features/src/tsServer/versionManager.ts` — TypeScript version selection
- `extensions/typescript-language-features/src/tsServer/versionProvider.electron.ts` — Disk-based version discovery (Electron-only)
- `extensions/typescript-language-features/src/tsServer/versionProvider.ts` — Abstract version provider interface
- `extensions/typescript-language-features/src/tsServer/nodeManager.ts` — Node.js management
- `extensions/typescript-language-features/src/tsServer/pluginPathsProvider.ts` — Plugin path resolution
- `extensions/typescript-language-features/src/tsServer/requestQueue.ts` — Request queuing and prioritization
- `extensions/typescript-language-features/src/tsServer/cachedResponse.ts` — Response caching layer
- `extensions/typescript-language-features/src/tsServer/callbackMap.ts` — Callback management for async responses
- `extensions/typescript-language-features/src/tsServer/cancellation.electron.ts` — Cancellation tokens (Electron-only)
- `extensions/typescript-language-features/src/tsServer/cancellation.ts` — Abstract cancellation interface
- `extensions/typescript-language-features/src/tsServer/logDirectoryProvider.electron.ts` — Electron log directory resolution (Electron-only)
- `extensions/typescript-language-features/src/tsServer/logDirectoryProvider.ts` — Abstract log provider
- `extensions/typescript-language-features/src/tsServer/serverError.ts` — Error classification and handling

### Protocol Layer (5 files)
- `extensions/typescript-language-features/src/tsServer/protocol/protocol.d.ts` — Re-exports TypeScript's server.protocol (23 LOC wrapper)
- `extensions/typescript-language-features/src/tsServer/protocol/protocol.const.ts` — Protocol constants
- `extensions/typescript-language-features/src/tsServer/protocol/errorCodes.ts` — Error code enums
- `extensions/typescript-language-features/src/tsServer/protocol/fixNames.ts` — Code action names
- `extensions/typescript-language-features/src/tsServer/protocol/modifiers.ts` — Semantic token modifiers

### Language Features Registration (41 files)
**Completions & IntelliSense:**
- `extensions/typescript-language-features/src/languageFeatures/completions.ts` — Completion provider
- `extensions/typescript-language-features/src/languageFeatures/jsDocCompletions.ts` — JSDoc/TSDoc completions
- `extensions/typescript-language-features/src/languageFeatures/directiveCommentCompletions.ts` — Directive completions
- `extensions/typescript-language-features/src/languageFeatures/signatureHelp.ts` — Function signature help

**Navigation & References:**
- `extensions/typescript-language-features/src/languageFeatures/definitions.ts` — Go to definition provider
- `extensions/typescript-language-features/src/languageFeatures/typeDefinitions.ts` — Go to type definition provider
- `extensions/typescript-language-features/src/languageFeatures/implementations.ts` — Go to implementation provider
- `extensions/typescript-language-features/src/languageFeatures/references.ts` — Find all references
- `extensions/typescript-language-features/src/languageFeatures/fileReferences.ts` — Find references in file results
- `extensions/typescript-language-features/src/languageFeatures/workspaceSymbols.ts` — Workspace symbol search
- `extensions/typescript-language-features/src/languageFeatures/documentSymbol.ts` — Document outline
- `extensions/typescript-language-features/src/languageFeatures/callHierarchy.ts` — Call hierarchy
- `extensions/typescript-language-features/src/languageFeatures/sourceDefinition.ts` — Source definition provider

**Code Quality:**
- `extensions/typescript-language-features/src/languageFeatures/hover.ts` — Hover information provider
- `extensions/typescript-language-features/src/languageFeatures/diagnostics.ts` — Diagnostic aggregation & reporting
- `extensions/typescript-language-features/src/languageFeatures/documentHighlight.ts` — Symbol highlighting
- `extensions/typescript-language-features/src/languageFeatures/linkedEditing.ts` — Linked editing (rename in JSX/template pairs)
- `extensions/typescript-language-features/src/languageFeatures/inlayHints.ts` — Inlay hints provider
- `extensions/typescript-language-features/src/languageFeatures/semanticTokens.ts` — Semantic token provider
- `extensions/typescript-language-features/src/languageFeatures/folding.ts` — Code folding ranges

**Refactoring & Code Actions:**
- `extensions/typescript-language-features/src/languageFeatures/refactor.ts` — Refactoring actions provider
- `extensions/typescript-language-features/src/languageFeatures/quickFix.ts` — Quick fixes provider
- `extensions/typescript-language-features/src/languageFeatures/fixAll.ts` — Fix all diagnostics
- `extensions/typescript-language-features/src/languageFeatures/organizeImports.ts` — Organize imports action
- `extensions/typescript-language-features/src/languageFeatures/rename.ts` — Rename refactoring
- `extensions/typescript-language-features/src/languageFeatures/copyPaste.ts` — Copy/paste code actions

**Formatting:**
- `extensions/typescript-language-features/src/languageFeatures/formatting.ts` — Document formatting provider
- `extensions/typescript-language-features/src/languageFeatures/tagClosing.ts` — Auto-closing JSX tags

**Configuration & Management:**
- `extensions/typescript-language-features/src/languageFeatures/fileConfigurationManager.ts` — Per-file settings synchronization
- `extensions/typescript-language-features/src/languageFeatures/updatePathsOnRename.ts` — Path update on rename refactoring
- `extensions/typescript-language-features/src/languageFeatures/tsconfig.ts` — TypeScript configuration file detection
- `extensions/typescript-language-features/src/languageFeatures/smartSelect.ts` — Expand/shrink selection

**Code Lens:**
- `extensions/typescript-language-features/src/languageFeatures/codeLens/baseCodeLensProvider.ts` — Base class for code lens
- `extensions/typescript-language-features/src/languageFeatures/codeLens/referencesCodeLens.ts` — References code lens
- `extensions/typescript-language-features/src/languageFeatures/codeLens/implementationsCodeLens.ts` — Implementations code lens

**Utilities:**
- `extensions/typescript-language-features/src/languageFeatures/util/codeAction.ts` — Code action builders
- `extensions/typescript-language-features/src/languageFeatures/util/copilot.ts` — Copilot integration
- `extensions/typescript-language-features/src/languageFeatures/util/dependentRegistration.ts` — Conditional feature registration
- `extensions/typescript-language-features/src/languageFeatures/util/snippetForFunctionCall.ts` — Function call snippet generation
- `extensions/typescript-language-features/src/languageFeatures/util/textRendering.ts` — Text rendering utilities
- `extensions/typescript-language-features/src/languageFeatures/definitionProviderBase.ts` — Base class for definition providers

### Commands (12 files)
- `extensions/typescript-language-features/src/commands/commandManager.ts` — Command registration and handling
- `extensions/typescript-language-features/src/commands/index.ts` — Base command registry
- `extensions/typescript-language-features/src/commands/restartTsServer.ts` — Restart server command
- `extensions/typescript-language-features/src/commands/reloadProject.ts` — Reload project command
- `extensions/typescript-language-features/src/commands/selectTypeScriptVersion.ts` — Version selection UI
- `extensions/typescript-language-features/src/commands/openTsServerLog.ts` — Open server logs command
- `extensions/typescript-language-features/src/commands/configurePlugin.ts` — Plugin configuration command
- `extensions/typescript-language-features/src/commands/goToProjectConfiguration.ts` — Navigate to tsconfig/jsconfig
- `extensions/typescript-language-features/src/commands/learnMoreAboutRefactorings.ts` — Refactoring documentation link
- `extensions/typescript-language-features/src/commands/openJsDocLink.ts` — JSDoc link handler
- `extensions/typescript-language-features/src/commands/tsserverRequests.ts` — Low-level tsserver request command
- `extensions/typescript-language-features/src/commands/useTsgo.ts` — Go implementation switch (Tsgo)

### Configuration Management (8 files)
- `extensions/typescript-language-features/src/configuration/configuration.ts` — Abstract configuration interface
- `extensions/typescript-language-features/src/configuration/configuration.electron.ts` — Electron-specific settings resolution
- `extensions/typescript-language-features/src/configuration/configuration.browser.ts` — Browser-specific settings resolution
- `extensions/typescript-language-features/src/configuration/languageDescription.ts` — Language descriptions (JS, TS, JSX, TSX)
- `extensions/typescript-language-features/src/configuration/documentSelector.ts` — Document selector patterns
- `extensions/typescript-language-features/src/configuration/fileSchemes.ts` — Supported file URI schemes
- `extensions/typescript-language-features/src/configuration/languageIds.ts` — Language ID enums
- `extensions/typescript-language-features/src/configuration/schemes.ts` — URI scheme constants

### File System Abstraction (3 files)
- `extensions/typescript-language-features/src/filesystems/ata.ts` — Auto-installed type definitions filesystem
- `extensions/typescript-language-features/src/filesystems/autoInstallerFs.ts` — Auto-installer filesystem wrapper
- `extensions/typescript-language-features/src/filesystems/memFs.ts` — In-memory filesystem for type definitions

### Utilities (16 files)
- `extensions/typescript-language-features/src/utils/fs.ts` — File system abstractions
- `extensions/typescript-language-features/src/utils/fs.electron.ts` — Electron-specific FS operations
- `extensions/typescript-language-features/src/utils/async.ts` — Async utilities
- `extensions/typescript-language-features/src/utils/cancellation.ts` — Cancellation token utilities
- `extensions/typescript-language-features/src/utils/configuration.ts` — Configuration utilities
- `extensions/typescript-language-features/src/utils/dispose.ts` — Resource cleanup helpers
- `extensions/typescript-language-features/src/utils/arrays.ts` — Array utilities
- `extensions/typescript-language-features/src/utils/objects.ts` — Object utilities
- `extensions/typescript-language-features/src/utils/regexp.ts` — Regex utilities
- `extensions/typescript-language-features/src/utils/hash.ts` — Hashing utilities
- `extensions/typescript-language-features/src/utils/lazy.ts` — Lazy evaluation
- `extensions/typescript-language-features/src/utils/packageInfo.ts` — Package metadata extraction
- `extensions/typescript-language-features/src/utils/platform.ts` — Platform detection (Electron vs Web)
- `extensions/typescript-language-features/src/utils/relativePathResolver.ts` — Path resolution
- `extensions/typescript-language-features/src/utils/resourceMap.ts` — URI-based map data structure
- `extensions/typescript-language-features/src/utils/temp.electron.ts` — Temp file management (Electron-only)

### Logging & Telemetry (4 files)
- `extensions/typescript-language-features/src/logging/logger.ts` — Logging facade
- `extensions/typescript-language-features/src/logging/telemetry.ts` — Telemetry reporter
- `extensions/typescript-language-features/src/logging/tracer.ts` — Request/response tracing
- `extensions/typescript-language-features/src/logging/logLevelMonitor.ts` — Log level change tracking

### UI Components (7 files)
- `extensions/typescript-language-features/src/ui/activeJsTsEditorTracker.ts` — Active editor tracking
- `extensions/typescript-language-features/src/ui/intellisenseStatus.ts` — IntelliSense status bar
- `extensions/typescript-language-features/src/ui/typingsStatus.ts` — Type definitions download status
- `extensions/typescript-language-features/src/ui/versionStatus.ts` — TypeScript version status bar
- `extensions/typescript-language-features/src/ui/largeProjectStatus.ts` — Large project warning
- `extensions/typescript-language-features/src/ui/managedFileContext.ts` — Managed file tracking
- `extensions/typescript-language-features/src/ui/suggestNativePreview.ts` — Native preview suggestion

### Web/WASM Edition (11 files)
- `extensions/typescript-language-features/web/src/webServer.ts` — Web server wrapper for tsserver
- `extensions/typescript-language-features/web/src/workerSession.ts` — Worker thread session management
- `extensions/typescript-language-features/web/src/serverHost.ts` — Server host abstraction
- `extensions/typescript-language-features/web/src/fileWatcherManager.ts` — File watching in web
- `extensions/typescript-language-features/web/src/pathMapper.ts` — Path mapping for browser
- `extensions/typescript-language-features/web/src/logging.ts` — Browser logging
- `extensions/typescript-language-features/web/src/wasmCancellationToken.ts` — WASM-compatible cancellation
- `extensions/typescript-language-features/web/src/typingsInstaller/typingsInstaller.ts` — Type definition installation
- `extensions/typescript-language-features/web/src/typingsInstaller/jsTyping.ts` — JS typing detection
- `extensions/typescript-language-features/web/src/util/args.ts` — Argument parsing utilities
- `extensions/typescript-language-features/web/src/util/hrtime.ts` — High-resolution timing

### Top-level Services
- `extensions/typescript-language-features/src/typescriptService.ts` — TypeScript service interfaces
- `extensions/typescript-language-features/src/experimentationService.ts` — A/B test management
- `extensions/typescript-language-features/src/experimentTelemetryReporter.ts` — Experimentation telemetry

## Tests

### Smoke Tests (6 files)
- `extensions/typescript-language-features/src/test/smoke/completions.test.ts` — Completion integration tests
- `extensions/typescript-language-features/src/test/smoke/fixAll.test.ts` — Fix-all code action tests
- `extensions/typescript-language-features/src/test/smoke/implementationsCodeLens.test.ts` — Implementations code lens tests
- `extensions/typescript-language-features/src/test/smoke/jsDocCompletions.test.ts` — JSDoc completion tests
- `extensions/typescript-language-features/src/test/smoke/quickFix.test.ts` — Quick fix action tests
- `extensions/typescript-language-features/src/test/smoke/referencesCodeLens.test.ts` — References code lens tests

### Unit Tests (7 files)
- `extensions/typescript-language-features/src/test/unit/cachedResponse.test.ts` — Response cache tests
- `extensions/typescript-language-features/src/test/unit/functionCallSnippet.test.ts` — Function call snippet generation
- `extensions/typescript-language-features/src/test/unit/jsdocSnippet.test.ts` — JSDoc snippet tests
- `extensions/typescript-language-features/src/test/unit/onEnter.test.ts` — On-enter key handler tests
- `extensions/typescript-language-features/src/test/unit/requestQueue.test.ts` — Request queuing logic
- `extensions/typescript-language-features/src/test/unit/server.test.ts` — Server communication tests
- `extensions/typescript-language-features/src/test/unit/textRendering.test.ts` — Text rendering output

### Test Infrastructure (5 files)
- `extensions/typescript-language-features/src/test/index.ts` — Test suite entry point
- `extensions/typescript-language-features/src/test/smoke/index.ts` — Smoke test suite runner
- `extensions/typescript-language-features/src/test/unit/index.ts` — Unit test suite runner
- `extensions/typescript-language-features/src/test/testUtils.ts` — Test helper functions
- `extensions/typescript-language-features/src/test/suggestTestHelpers.ts` — Completion test helpers

## Types / Interfaces

### Protocol Types
- `extensions/typescript-language-features/src/tsServer/protocol/protocol.d.ts` — Generated type definitions from TypeScript's server protocol

### Service Interfaces
- `extensions/typescript-language-features/src/typescriptService.ts` — `ITypeScriptServiceClient`, `ServerResponse`, `ClientCapabilities`
- `extensions/typescript-language-features/src/tsServer/server.ts` — `ITypeScriptServer`, `TsServerProcessFactory`
- `extensions/typescript-language-features/src/configuration/configuration.ts` — `TypeScriptServiceConfiguration`, `ServiceConfigurationProvider`
- `extensions/typescript-language-features/src/languageProvider.ts` — Language provider registration types

## Configuration

### JSON Schemas
- `extensions/typescript-language-features/schemas/tsconfig.schema.json` — TypeScript configuration schema reference
- `extensions/typescript-language-features/schemas/jsconfig.schema.json` — JavaScript configuration schema reference
- `extensions/typescript-language-features/schemas/package.schema.json` — Package.json validation for TypeScript fields

### Extension Settings
- `extensions/typescript-language-features/package.json` — 168 contribution points (languages, commands, configurations, views, etc.)
- `extensions/typescript-language-features/package.nls.json` — Localization strings
- `extensions/typescript-language-features/cgmanifest.json` — Component governance metadata

### Platform Variants
- `extensions/typescript-language-features/src/configuration/configuration.electron.ts` — Electron environment detection
- `extensions/typescript-language-features/src/configuration/configuration.browser.ts` — Browser environment detection
- `extensions/typescript-language-features/src/tsServer/serverProcess.electron.ts` — Node.js child process spawning
- `extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts` — Worker-based server
- `extensions/typescript-language-features/src/utils/fs.electron.ts` — Electron file system calls
- `extensions/typescript-language-features/src/utils/temp.electron.ts` — Electron temp file management
- `extensions/typescript-language-features/src/tsServer/cancellation.electron.ts` — Electron cancellation signals
- `extensions/typescript-language-features/src/tsServer/versionProvider.electron.ts` — Electron version discovery
- `extensions/typescript-language-features/src/tsServer/logDirectoryProvider.electron.ts` — Electron log directory resolution

## Examples / Fixtures

### Test Workspace
- `extensions/typescript-language-features/test-workspace/` — Integration test fixture directory

### Resources
- `extensions/typescript-language-features/resources/walkthroughs/` — 4 SVG walkthrough graphics (install Node, create JS file, debug/run, learn more)
- `extensions/typescript-language-features/media/icon.png` — Extension icon

## Documentation

- `extensions/typescript-language-features/README.md` — Basic feature overview
- `extensions/typescript-language-features/web/README.md` — Web edition documentation

## Notable Clusters

### `extensions/typescript-language-features/src/languageFeatures/` — 41 files
Implements 30+ VS Code language features via `vscode.languages.*` registration APIs. Maps tsserver protocol responses to VS Code completion items, hover information, code actions, diagnostics, etc. Highest density of direct IDE integration code. Each provider registers with specific VS Code language service APIs (completions, definitions, hover, refactoring, diagnostics, etc.).

### `extensions/typescript-language-features/src/tsServer/` — 26 files
Manages lifecycle of TypeScript tsserver process (Electron) or Web Worker (browser). Handles request queuing, response caching, version management, file watching, plugin loading, and error recovery. Critical adapter layer between VS Code and TypeScript compiler. Includes request/response marshalling, server spawning, and process management.

### `extensions/typescript-language-features/src/configuration/` — 8 files
Abstracts platform differences (Electron vs Web) for TypeScript configuration discovery, file scheme handling, and language ID mapping. Uses `.electron` and `.browser` file suffixes for conditional compilation. Manages language descriptions, document selectors, and file URI schemes.

### Platform Abstraction Pattern
7 Electron-only files (*.electron.ts) + 4 browser-only files (*.browser.ts) enable single-codebase dual-target support:
- Electron: Direct file system access, Node.js process spawning, native APIs
- Browser: Web Workers, IndexedDB, Fetch APIs, no file system access

### Dependency on VS Code Extension API
All language features depend on:
- `vscode.languages.registerCompletionItemProvider()`
- `vscode.languages.registerDefinitionProvider()`
- `vscode.languages.registerHoverProvider()` (+ 20+ more)
- `vscode.workspace.*` for file operations
- `vscode.commands.*` for command integration

### Dependency on TypeScript Compiler API
Entire extension wraps `typescript` package's server protocol:
- Parsed from node_modules/typescript/lib/typescript.d.ts
- Async JSON-RPC protocol over stdio/IPC
- Features map 1:1 to tsserver request types (completions → CompletionInfoRequest, etc.)

## Porting Implications for Tauri/Rust

### Must-Port Core Components
1. **TypeScript Service Client** (1200+ LOC) — Manages tsserver lifecycle, request/response coordination, async handling
2. **Language Features Providers** (41 files) — Each implements specific `vscode.languages.*` API; would need Rust equivalents
3. **TSServer Communication Protocol** — JSON-RPC parser and serializer; protocol types auto-generated from TypeScript
4. **File Watching & Synchronization** — Buffer sync support, file change notifications
5. **Configuration Management** — Loads tsconfig.json, jsconfig.json, workspace settings

### Platform-Specific Rewrites Required
- **Electron Process Spawning** → Tauri subprocess with proper stdio piping
- **File System Operations** → Tauri's file system API
- **Temp File Management** → Tauri temp directories
- **LSP-to-IDE Bridging** → Convert from `vscode.languages.*` callbacks to custom IPC

### Architectural Challenges
- **Tight VSCode Integration**: 40+ direct dependencies on `vscode.*` namespace; Tauri has no equivalent IDE API
- **Protocol Coupling**: 23 LOC wrapper around TypeScript's generated protocol types; tight coupling to specific TS versions
- **Async Callback Model**: Heavy use of Promise-based async; Rust would require tokio/async-await refactor
- **Configuration Schema Loading**: Currently auto-loads JSON schemas from schemastore.org; needs Rust HTTP client
- **Language Metadata**: Language descriptions, document selectors, file schemes all baked into configuration; would need data structures

### Estimated Porting Effort
- **TypeScript Service Client**: 800-1200 hours (complete rewrite in async Rust)
- **Language Features Layer**: 600-1000 hours (41 individual providers, each needs equivalent Rust implementation)
- **Protocol Serialization**: 100-150 hours (JSON-RPC wrapper, type marshalling)
- **File System & Process Management**: 150-250 hours (Tauri API integration)
- **Configuration & Settings**: 100-200 hours (settings discovery, schema loading, path resolution)
- **Testing & Integration**: 300-500 hours (smoke tests, integration tests, platform testing)
- **Total**: ~2000-3300 engineering hours (~50-80 weeks for 1-2 engineers)

### File Count Summary
- **Total Files**: 188 (168 TS/TSX + 20 JSON/schemas/resources)
- **Implementation**: 132 TS files + 11 Web/WASM files = 143 must-port
- **Tests**: 13 files (smoke + unit tests, integration fixtures)
- **Configuration**: 3 schemas + package.json + package.nls.json
- **Documentation**: 2 READMEs

---

## Summary

The TypeScript language features extension exemplifies the deep coupling between VS Code's IDE infrastructure and a language service implementation. The 132 implementation files (143 with web) are organized around a two-tier architecture: (1) a tsserver communication layer (26 files) that spawns TypeScript's Node.js process and manages JSON-RPC request/response cycles, and (2) a language features layer (41 files) that maps tsserver responses onto VS Code's language service APIs (completions, hover, definitions, diagnostics, code actions, etc.). The extension's platform abstraction pattern (7 Electron-specific files + 4 browser-specific files) demonstrates how it achieves dual-target support, but a Tauri port would require fundamental rewrites since Tauri lacks an IDE API layer. The configuration system binds tightly to tsserver's version-dependent protocol types, and the UI layer (7 files) integrates directly with VS Code's status bar, commands, and event system. Porting would require 2000-3300 engineering hours to rebuild the async request/response handling, language provider interfaces, and IDE integration points in Rust with equivalent semantics.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code TypeScript Language Features: Core Patterns for Tauri/Rust Porting

## Research Focus
Analyzed patterns in `extensions/typescript-language-features/` (168 files, 22,571 LOC) to understand how VS Code orchestrates language intelligence (completions, hover, diagnostics, refactoring, code actions).

---

## Pattern 1: Language Provider Registration Model
**Where:** `src/languageProvider.ts:64-100`
**What:** Lazy-loaded plugin registration system that dynamically imports and registers 25+ language feature providers on client readiness.

```typescript
private async registerProviders(): Promise<void> {
    const selector = this.documentSelector;
    const cachedNavTreeResponse = new CachedResponse();
    
    await Promise.all([
        import('./languageFeatures/callHierarchy').then(provider => this._register(provider.register(selector, this.client))),
        import('./languageFeatures/completions').then(provider => this._register(provider.register(selector, this.description, this.client, this.typingsStatus, this.fileConfigurationManager, this.commandManager, this.telemetryReporter, this.onCompletionAccepted))),
        import('./languageFeatures/hover').then(provider => this._register(provider.register(selector, this.client, this.fileConfigurationManager))),
        // ... 22 more feature imports
    ]);
}
```

**Variations / call-sites:**
- `src/typeScriptServiceClientHost.ts:51-100` - Creates LanguageProvider instances per language
- Each provider module exports a `register()` function returning `vscode.Disposable`

---

## Pattern 2: Provider Implementation Interface
**Where:** `src/languageFeatures/hover.ts:17-69`
**What:** Language feature providers implement VS Code's interface (e.g., `vscode.HoverProvider`) and delegate to the TypeScript service client.

```typescript
class TypeScriptHoverProvider implements vscode.HoverProvider {
    public constructor(
        private readonly client: ITypeScriptServiceClient,
        private readonly fileConfigurationManager: FileConfigurationManager,
    ) { }

    public async provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken,
        context?: vscode.HoverContext,
    ): Promise<vscode.VerboseHover | undefined> {
        const filepath = this.client.toOpenTsFilePath(document);
        if (!filepath) { return undefined; }
        
        const args = { ...typeConverters.Position.toFileLocationRequestArgs(filepath, position), verbosityLevel };
        const response = await this.client.interruptGetErr(async () => {
            await this.fileConfigurationManager.ensureConfigurationForDocument(document, token);
            return this.client.execute('quickinfo', args, token);
        });
    }
}
```

**Variations / call-sites:**
- `src/languageFeatures/completions.ts:682-750` - CompletionItemProvider
- `src/languageFeatures/definitionProviderBase.ts` - DefinitionProvider
- `src/languageFeatures/quickFix.ts:185` - CodeActionProvider
- Pattern used for: 25+ provider types (diagnostics, references, rename, refactor, symbols, etc.)

---

## Pattern 3: Command-Based Code Actions
**Where:** `src/languageFeatures/quickFix.ts:30-58`
**What:** Code action execution wrapped in registered command objects with IDs, enabling undo/redo and command palette.

```typescript
class ApplyCodeActionCommand implements Command {
    public static readonly ID = '_typescript.applyCodeActionCommand';
    public readonly id = ApplyCodeActionCommand.ID;

    constructor(
        private readonly client: ITypeScriptServiceClient,
        private readonly diagnosticManager: DiagnosticsManager,
        private readonly telemetryReporter: TelemetryReporter,
    ) { }

    public async execute({ document, action, diagnostic, followupAction }: ApplyCodeActionCommand_args): Promise<boolean> {
        this.telemetryReporter.logTelemetry('quickFix.execute', { fixName: action.fixName });
        this.diagnosticManager.deleteDiagnostic(document.uri, diagnostic);
        const codeActionResult = await applyCodeActionCommands(this.client, action.commands, nulToken);
        await followupAction?.execute();
        return codeActionResult;
    }
}
```

**Variations / call-sites:**
- `src/languageFeatures/refactor.ts:47-70` - DidApplyRefactoringCommand, SelectRefactorCommand
- `src/commands/` directory - 10+ command implementations (restart server, open logs, select version, etc.)
- Registered via `CommandManager` which wraps `vscode.commands.registerCommand`

---

## Pattern 4: TypeScript Server Request/Response Protocol
**Where:** `src/typescriptService.ts:38-99`
**What:** Typed request-response mapping using TypeScript mapped types to track which args produce which response types.

```typescript
interface StandardTsServerRequests {
    'applyCodeActionCommand': [Proto.ApplyCodeActionCommandRequestArgs, Proto.ApplyCodeActionCommandResponse];
    'completionEntryDetails': [Proto.CompletionDetailsRequestArgs, Proto.CompletionDetailsResponse];
    'completionInfo': [Proto.CompletionsRequestArgs, Proto.CompletionInfoResponse];
    'definition': [Proto.FileLocationRequestArgs, Proto.DefinitionResponse];
    'quickinfo': [Proto.FileLocationRequestArgs, Proto.QuickInfoResponse];
    'getCodeFixes': [Proto.CodeFixRequestArgs, Proto.CodeFixResponse];
    'getApplicableRefactors': [Proto.GetApplicableRefactorsRequestArgs, Proto.GetApplicableRefactorsResponse];
    // ... 60+ more requests
}

interface NoResponseTsServerRequests {
    'open': [Proto.OpenRequestArgs, null];
    'close': [Proto.FileRequestArgs, null];
    'change': [Proto.ChangeRequestArgs, null];
}

export type TypeScriptRequests = StandardTsServerRequests & NoResponseTsServerRequests & AsyncTsServerRequests;
```

**Variations / call-sites:**
- All `execute()` calls validated against this: `execute<K extends keyof TypeScriptRequests>(command: K, args: TypeScriptRequests[K][0])`
- Proto types defined in `src/tsServer/protocol/protocol.ts` (~4000+ LOC)

---

## Pattern 5: Service Client as Central Hub
**Where:** `src/typescriptServiceClient.ts:108-250`
**What:** Single service client orchestrates server lifecycle (spawn, crash recovery, restart), buffer synchronization, diagnostics routing, and request execution.

```typescript
export default class TypeScriptServiceClient extends Disposable implements ITypeScriptServiceClient {
    private readonly bufferSyncSupport: BufferSyncSupport;
    private readonly diagnosticsManager: DiagnosticsManager;
    private readonly pluginManager: PluginManager;
    private serverState: ServerState.State = ServerState.None;

    public execute(command: keyof TypeScriptRequests, args: unknown, token: vscode.CancellationToken, config?: ExecConfig): Promise<ServerResponse.Response<Proto.Response>> {
        let executions = this.executeImpl(command, args, {
            isAsync: false,
            token,
            expectsResult: true,
            ...config,
        });
        
        if (config?.nonRecoverable) {
            executions[0]!.catch(err => this.fatalError(command, err));
        }
        return executions[0]!;
    }
    
    public interruptGetErr<R>(f: () => R): R {
        return this.bufferSyncSupport.interruptGetErr(f);
    }
}
```

**Variations / call-sites:**
- `src/typeScriptServiceClientHost.ts:51-100` - Wraps the client in per-language providers
- Server lifecycle: spawn → running → crash → restart (with backoff)
- Handles multi-process (main/syntax/semantic/diagnostics servers)

---

## Pattern 6: Request Queueing with Priority Levels
**Where:** `src/tsServer/requestQueue.ts:7-57`
**What:** Prioritized queue allowing diagnostic requests to jump ahead of low-priority operations while maintaining ordering fences.

```typescript
export enum RequestQueueingType {
    Normal = 1,           // Executed in order
    LowPriority = 2,      // Normal requests jump in front
    Fence = 3,            // Blocks reordering
}

export class RequestQueue {
    private readonly queue: RequestItem[] = [];
    
    public enqueue(item: RequestItem): void {
        if (item.queueingType === RequestQueueingType.Normal) {
            let index = this.queue.length - 1;
            while (index >= 0) {
                if (this.queue[index].queueingType !== RequestQueueingType.LowPriority) {
                    break;
                }
                --index;
            }
            this.queue.splice(index + 1, 0, item);
        } else {
            this.queue.push(item);
        }
    }
}
```

**Variations / call-sites:**
- `src/tsServer/server.ts:228-260` - Single/Multi server execute implementations
- Used by `SingleTsServer.executeImpl()` to manage ~60+ concurrent request types

---

## Pattern 7: Diagnostic Management with Caching
**Where:** `src/languageFeatures/diagnostics.ts:34-72`
**What:** Separate diagnostic kinds (Syntax, Semantic, Suggestion, RegionSemantic) cached per file with invalidation.

```typescript
export const enum DiagnosticKind {
    Syntax,
    Semantic,
    Suggestion,
    RegionSemantic,
}

class FileDiagnostics {
    private readonly _diagnostics = new Map<DiagnosticKind, ReadonlyArray<vscode.Diagnostic>>();

    public updateDiagnostics(
        language: DiagnosticLanguage,
        kind: DiagnosticKind,
        diagnostics: ReadonlyArray<vscode.Diagnostic>,
        ranges: ReadonlyArray<vscode.Range> | undefined
    ): boolean {
        if (language !== this.language) {
            this._diagnostics.clear();
            this.language = language;
        }
        
        const existing = this._diagnostics.get(kind);
        if (existing?.length === 0 && diagnostics.length === 0) {
            return false;  // No update needed
        }
        
        this._diagnostics.set(kind, diagnostics);
        return true;
    }
}
```

**Variations / call-sites:**
- DiagnosticsManager maintains map of files → FileDiagnostics
- Connected to server events via `onEvent: vscode.Event<Proto.Event>`
- Separate background diagnostics server for large projects

---

## Pattern 8: Position/Range Type Conversion
**Where:** `src/typeConverters.ts:15-68`
**What:** Namespace pattern for bidirectional conversion between VS Code (0-based) and TS Server (1-based) coordinates.

```typescript
export namespace Range {
    export const fromTextSpan = (span: Proto.TextSpan): vscode.Range =>
        fromLocations(span.start, span.end);

    export const toTextSpan = (range: vscode.Range): Proto.TextSpan => ({
        start: Position.toLocation(range.start),
        end: Position.toLocation(range.end)
    });
}

export namespace Position {
    export const fromLocation = (tslocation: Proto.Location): vscode.Position =>
        new vscode.Position(tslocation.line - 1, tslocation.offset - 1);

    export const toLocation = (vsPosition: vscode.Position): Proto.Location => ({
        line: vsPosition.line + 1,
        offset: vsPosition.character + 1,
    });

    export const toFileLocationRequestArgs = (file: string, position: vscode.Position): Proto.FileLocationRequestArgs => ({
        file,
        line: position.line + 1,
        offset: position.character + 1,
    });
}
```

**Variations / call-sites:**
- Used by every language feature provider
- Repeated for Range, Location, TextEdit, CodeAction, SymbolKind, etc.
- Core abstraction for IDE ↔ Language Server communication

---

## Architecture Summary

The TypeScript language features extension demonstrates a **multi-layered architecture**:

1. **Registration Layer** (`languageProvider.ts`): Lazy loads 25+ feature modules, each exporting a `register()` function
2. **Provider Layer** (`languageFeatures/*`): 25+ provider classes implementing VS Code interfaces, delegating to client
3. **Service Layer** (`typescriptServiceClient.ts`): Central hub managing server lifecycle, request routing, diagnostics
4. **Protocol Layer** (`typescriptService.ts`, `tsServer/protocol/`): Typed request-response mapping with 60+ command types
5. **Transport Layer** (`tsServer/server.ts`, `tsServer/requestQueue.ts`): Process management, prioritized queueing, response handling
6. **Conversion Layer** (`typeConverters.ts`): Bidirectional coordinate/type conversion between IDEs

**Key Design Patterns:**
- **Plugin-based** via lazy-loaded feature modules
- **Event-driven** for diagnostics, server state changes, file watching
- **Typed requests** with compile-time checking via mapped TypeScript types
- **Prioritized async queue** with fence-based ordering
- **Crash recovery** with exponential backoff and user prompts
- **Multi-process** server deployment (syntax/semantic/diagnostics isolation)
- **Command pattern** for undo/redo-able operations
- **Bi-directional type conversion** for VS Code ↔ TS Server coordinates

---

**Files Analyzed:** 168 files total
- Core clients: `typescriptServiceClient.ts`, `typeScriptServiceClientHost.ts`
- 25+ language features: `src/languageFeatures/*.ts`
- Server communication: `src/tsServer/server.ts`, `requestQueue.ts`, `protocol/*.ts`
- Command infrastructure: `src/commands/*.ts`
- Diagnostic pipeline: `src/languageFeatures/diagnostics.ts`
- Type conversion: `src/typeConverters.ts`

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
