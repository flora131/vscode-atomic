# Partition 8 of 79 — Findings

## Scope
`extensions/typescript-language-features/` (168 files, 22,571 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# TypeScript Language Features Extension (Partition 8)

## Implementation

### Core Language Provider Architecture
- `extensions/typescript-language-features/src/languageProvider.ts` — Central dispatcher registering all language feature providers with VSCode
- `extensions/typescript-language-features/src/typescriptServiceClient.ts` — LSP-like client for communicating with tsserver process
- `extensions/typescript-language-features/src/typeScriptServiceClientHost.ts` — Manages lifecycle and routing of service client
- `extensions/typescript-language-features/src/lazyClientHost.ts` — Lazy initialization wrapper for the client host
- `extensions/typescript-language-features/src/api.ts` — Public extension API interface

### TSServer Process Management
- `extensions/typescript-language-features/src/tsServer/server.ts` — Main tsserver orchestration and request/response handling
- `extensions/typescript-language-features/src/tsServer/serverProcess.electron.ts` — Process spawning for Electron environment
- `extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts` — WASM-based tsserver for browser/web context
- `extensions/typescript-language-features/src/tsServer/spawner.ts` — Node process spawner with stdio management
- `extensions/typescript-language-features/src/tsServer/nodeManager.ts` — Node.js version/runtime management
- `extensions/typescript-language-features/src/tsServer/versionManager.ts` — TypeScript version resolution and switching
- `extensions/typescript-language-features/src/tsServer/versionProvider.ts` — Provider for available TypeScript versions
- `extensions/typescript-language-features/src/tsServer/versionProvider.electron.ts` — Electron-specific version discovery

### Request/Response Pipeline
- `extensions/typescript-language-features/src/tsServer/requestQueue.ts` — Async request queuing and sequencing
- `extensions/typescript-language-features/src/tsServer/callbackMap.ts` — Maps request IDs to response callbacks
- `extensions/typescript-language-features/src/tsServer/cachedResponse.ts` — Caches frequently-accessed responses
- `extensions/typescript-language-features/src/tsServer/api.ts` — Type-safe request/response marshaling
- `extensions/typescript-language-features/src/tsServer/bufferSyncSupport.ts` — Synchronizes buffer state with tsserver

### Language Features (28 providers registering with vscode.languages.register*)
- `extensions/typescript-language-features/src/languageFeatures/completions.ts` — Autocomplete via CompletionItemProvider
- `extensions/typescript-language-features/src/languageFeatures/definitions.ts` — Go-to-definition provider
- `extensions/typescript-language-features/src/languageFeatures/typeDefinitions.ts` — Go-to-type-definition provider
- `extensions/typescript-language-features/src/languageFeatures/implementations.ts` — Go-to-implementation provider
- `extensions/typescript-language-features/src/languageFeatures/references.ts` — Find references provider
- `extensions/typescript-language-features/src/languageFeatures/hover.ts` — Hover tooltips
- `extensions/typescript-language-features/src/languageFeatures/signatureHelp.ts` — Parameter hints
- `extensions/typescript-language-features/src/languageFeatures/rename.ts` — Rename refactoring
- `extensions/typescript-language-features/src/languageFeatures/refactor.ts` — Code refactorings
- `extensions/typescript-language-features/src/languageFeatures/quickFix.ts` — Quick fixes via CodeActionProvider
- `extensions/typescript-language-features/src/languageFeatures/fixAll.ts` — Fix-all code actions
- `extensions/typescript-language-features/src/languageFeatures/diagnostics.ts` — Linting and error reporting
- `extensions/typescript-language-features/src/languageFeatures/formatting.ts` — Document and range formatting
- `extensions/typescript-language-features/src/languageFeatures/smartSelect.ts` — Expand/shrink selection
- `extensions/typescript-language-features/src/languageFeatures/documentSymbol.ts` — Document outline/breadcrumb
- `extensions/typescript-language-features/src/languageFeatures/workspaceSymbol.ts` — Workspace symbol search
- `extensions/typescript-language-features/src/languageFeatures/documentHighlight.ts` — Highlight matching symbols
- `extensions/typescript-language-features/src/languageFeatures/inlayHints.ts` — Inline type hints
- `extensions/typescript-language-features/src/languageFeatures/semanticTokens.ts` — Semantic syntax highlighting
- `extensions/typescript-language-features/src/languageFeatures/folding.ts` — Code folding regions
- `extensions/typescript-language-features/src/languageFeatures/callHierarchy.ts` — Call hierarchy navigation
- `extensions/typescript-language-features/src/languageFeatures/linkedEditing.ts` — Synchronized editing of related symbols
- `extensions/typescript-language-features/src/languageFeatures/organizeImports.ts` — Import reorganization
- `extensions/typescript-language-features/src/languageFeatures/fileReferences.ts` — References across files
- `extensions/typescript-language-features/src/languageFeatures/jsDocCompletions.ts` — JSDoc template completion
- `extensions/typescript-language-features/src/languageFeatures/directiveCommentCompletions.ts` — Directive comments
- `extensions/typescript-language-features/src/languageFeatures/copyPaste.ts` — Paste-with-imports behavior
- `extensions/typescript-language-features/src/languageFeatures/tagClosing.ts` — JSX tag auto-closing
- `extensions/typescript-language-features/src/languageFeatures/tsconfig.ts` — Tsconfig.json document links
- `extensions/typescript-language-features/src/languageFeatures/sourceDefinition.ts` — Source location navigation
- `extensions/typescript-language-features/src/languageFeatures/updatePathsOnRename.ts` — Path updates on file rename

### Code Lens Implementations
- `extensions/typescript-language-features/src/languageFeatures/codeLens/baseCodeLensProvider.ts` — Base class for all code lens providers
- `extensions/typescript-language-features/src/languageFeatures/codeLens/referencesCodeLens.ts` — "X references" lens
- `extensions/typescript-language-features/src/languageFeatures/codeLens/implementationsCodeLens.ts` — "X implementations" lens

### Commands
- `extensions/typescript-language-features/src/commands/commandManager.ts` — Command registration and dispatch
- `extensions/typescript-language-features/src/commands/restartTsServer.ts` — Restart server command
- `extensions/typescript-language-features/src/commands/reloadProject.ts` — Reload project command
- `extensions/typescript-language-features/src/commands/selectTypeScriptVersion.ts` — Version selector command
- `extensions/typescript-language-features/src/commands/openTsServerLog.ts` — Debug log viewer command
- `extensions/typescript-language-features/src/commands/openJsDocLink.ts` — JSDoc link handler
- `extensions/typescript-language-features/src/commands/configurePlugin.ts` — Plugin configuration
- `extensions/typescript-language-features/src/commands/goToProjectConfiguration.ts` — Navigate to tsconfig.json
- `extensions/typescript-language-features/src/commands/learnMoreAboutRefactorings.ts` — Refactoring documentation
- `extensions/typescript-language-features/src/commands/tsserverRequests.ts` — Direct tsserver request execution
- `extensions/typescript-language-features/src/commands/useTsgo.ts` — Tsgo integration command
- `extensions/typescript-language-features/src/commands/index.ts` — Command registry

### Configuration & Environment
- `extensions/typescript-language-features/src/configuration/configuration.ts` — Settings loader and schema
- `extensions/typescript-language-features/src/configuration/configuration.electron.ts` — Electron-specific settings
- `extensions/typescript-language-features/src/configuration/configuration.browser.ts` — Browser-specific settings
- `extensions/typescript-language-features/src/configuration/documentSelector.ts` — Language document selector configuration
- `extensions/typescript-language-features/src/configuration/languageDescription.ts` — TS/JS language metadata
- `extensions/typescript-language-features/src/configuration/languageIds.ts` — Language ID constants
- `extensions/typescript-language-features/src/configuration/fileSchemes.ts` — Supported URI schemes (file, vscode-notebook-cell, etc.)
- `extensions/typescript-language-features/src/configuration/schemes.ts` — Scheme constants

### File System & Auto-Install
- `extensions/typescript-language-features/src/filesystems/ata.ts` — Auto Typings Acquisition (ATA) filesystem
- `extensions/typescript-language-features/src/filesystems/autoInstallerFs.ts` — Type definitions auto-install manager
- `extensions/typescript-language-features/src/filesystems/memFs.ts` — In-memory virtual filesystem

### Logging & Monitoring
- `extensions/typescript-language-features/src/logging/logger.ts` — Debug logging infrastructure
- `extensions/typescript-language-features/src/logging/telemetry.ts` — Telemetry event reporting
- `extensions/typescript-language-features/src/logging/tracer.ts` — Request tracing
- `extensions/typescript-language-features/src/logging/logLevelMonitor.ts` — Log level control

### Server Error Handling
- `extensions/typescript-language-features/src/tsServer/serverError.ts` — Error parsing and reporting

### File Watching & Project Management
- `extensions/typescript-language-features/src/tsServer/fileWatchingManager.ts` — File watcher for project changes
- `extensions/typescript-language-features/src/tsServer/pluginPathsProvider.ts` — Plugin resolution

### Tasks & Build
- `extensions/typescript-language-features/src/task/taskProvider.ts` — Task provider for TypeScript build tasks
- `extensions/typescript-language-features/src/task/tsconfigProvider.ts` — Tsconfig.json discovery for tasks

### Type Converters & Utilities
- `extensions/typescript-language-features/src/typeConverters.ts` — Maps tsserver types to VSCode protocol types
- `extensions/typescript-language-features/src/typescriptService.ts` — High-level TypeScript service interface
- `extensions/typescript-language-features/src/tsconfig.ts` — Tsconfig.json parsing utilities

### Extension Entry Points
- `extensions/typescript-language-features/src/extension.ts` — Main extension activation for Node/Electron
- `extensions/typescript-language-features/src/extension.browser.ts` — Browser/Web extension entry point

### Browser/Web Support
- `extensions/typescript-language-features/web/src/webServer.ts` — TypeScript language server for web environments
- `extensions/typescript-language-features/web/src/workerSession.ts` — Worker-based session management
- `extensions/typescript-language-features/web/src/serverHost.ts` — Browser-compatible server host
- `extensions/typescript-language-features/web/src/fileWatcherManager.ts` — Browser file watching
- `extensions/typescript-language-features/web/src/typingsInstaller/typingsInstaller.ts` — Type definitions installer for web
- `extensions/typescript-language-features/web/src/typingsInstaller/jsTyping.ts` — JS typing utilities
- `extensions/typescript-language-features/web/src/util/args.ts` — Argument parsing
- `extensions/typescript-language-features/web/src/util/hrtime.ts` — High-resolution timer polyfill
- `extensions/typescript-language-features/web/src/pathMapper.ts` — URL-to-filesystem path mapping
- `extensions/typescript-language-features/web/src/wasmCancellationToken.ts` — Cancellation support for WASM
- `extensions/typescript-language-features/web/src/logging.ts` — Browser logging

### Experimentation & Telemetry
- `extensions/typescript-language-features/src/experimentationService.ts` — A/B testing framework integration
- `extensions/typescript-language-features/src/experimentTelemetryReporter.ts` — Experiment result reporting

### Remote Repository Support
- `extensions/typescript-language-features/src/remoteRepositories.browser.ts` — GitHub Copilot remote repository handling

### Utility Helpers
- `extensions/typescript-language-features/src/languageFeatures/util/dependentRegistration.ts` — Conditional provider registration
- `extensions/typescript-language-features/src/languageFeatures/util/codeAction.ts` — Code action utilities
- `extensions/typescript-language-features/src/languageFeatures/util/snippetForFunctionCall.ts` — Function snippet generation
- `extensions/typescript-language-features/src/languageFeatures/util/textRendering.ts` — Text formatting utilities
- `extensions/typescript-language-features/src/languageFeatures/util/copilot.ts` — Copilot integration utilities

## Tests

### Unit Tests
- `extensions/typescript-language-features/src/test/unit/server.test.ts` — TSServer request/response handling
- `extensions/typescript-language-features/src/test/unit/requestQueue.test.ts` — Request queuing logic
- `extensions/typescript-language-features/src/test/unit/cachedResponse.test.ts` — Response caching behavior
- `extensions/typescript-language-features/src/test/unit/functionCallSnippet.test.ts` — Snippet generation
- `extensions/typescript-language-features/src/test/unit/jsdocSnippet.test.ts` — JSDoc snippet generation
- `extensions/typescript-language-features/src/test/unit/onEnter.test.ts` — On-enter formatting
- `extensions/typescript-language-features/src/test/unit/textRendering.test.ts` — Text rendering

### Smoke Tests (Integration)
- `extensions/typescript-language-features/src/test/smoke/completions.test.ts` — Completion provider integration
- `extensions/typescript-language-features/src/test/smoke/quickFix.test.ts` — Quick fix integration
- `extensions/typescript-language-features/src/test/smoke/fixAll.test.ts` — Fix-all code actions
- `extensions/typescript-language-features/src/test/smoke/jsDocCompletions.test.ts` — JSDoc completion integration
- `extensions/typescript-language-features/src/test/smoke/referencesCodeLens.test.ts` — Code lens integration
- `extensions/typescript-language-features/src/test/smoke/implementationsCodeLens.test.ts` — Implementations code lens

### Test Utilities
- `extensions/typescript-language-features/src/test/testUtils.ts` — Test helpers and fixtures
- `extensions/typescript-language-features/src/test/suggestTestHelpers.ts` — Completion testing utilities
- `extensions/typescript-language-features/src/test/index.ts` — Test suite entry point
- `extensions/typescript-language-features/src/test/smoke/index.ts` — Smoke test suite entry point
- `extensions/typescript-language-features/src/test/unit/index.ts` — Unit test suite entry point

## Types / Interfaces

### Protocol Definitions
- `extensions/typescript-language-features/src/tsServer/protocol/protocol.d.ts` — Complete TypeScript language service protocol (auto-generated from TypeScript compiler)
- `extensions/typescript-language-features/src/tsServer/protocol/protocol.const.ts` — Protocol constants and command names
- `extensions/typescript-language-features/src/tsServer/protocol/errorCodes.ts` — Error code enumerations
- `extensions/typescript-language-features/src/tsServer/protocol/modifiers.ts` — Type modifier constants
- `extensions/typescript-language-features/src/tsServer/protocol/fixNames.ts` — Quick fix names

## Configuration

### Build Configuration
- `extensions/typescript-language-features/esbuild.mts` — Electron/Node bundler configuration
- `extensions/typescript-language-features/esbuild.browser.mts` — Browser/Web bundler configuration
- `extensions/typescript-language-features/package.json` — Extension metadata, dependencies, and activation events
- `extensions/typescript-language-features/package-lock.json` — Locked dependency tree
- `extensions/typescript-language-features/.npmrc` — NPM configuration
- `extensions/typescript-language-features/cgmanifest.json` — Component governance manifest

### Web Configuration
- `extensions/typescript-language-features/web/tsconfig.json` — TypeScript compiler options for web bundle

### JSON Schemas
- `extensions/typescript-language-features/schemas/tsconfig.schema.json` — TypeScript configuration schema
- `extensions/typescript-language-features/schemas/jsconfig.schema.json` — JavaScript configuration schema
- `extensions/typescript-language-features/schemas/package.schema.json` — Package.json schema

### NLS (Internationalization)
- `extensions/typescript-language-features/package.nls.json` — Localized string resources

## Documentation

### Main Documentation
- `extensions/typescript-language-features/README.md` — Extension overview and contributing guide
- `extensions/typescript-language-features/web/README.md` — Web/browser variant documentation

### Resource Assets
- `extensions/typescript-language-features/resources/walkthroughs/` — Walkthrough SVG assets for onboarding

## Notable Clusters

### Language Features Directory
- `extensions/typescript-language-features/src/languageFeatures/` — 31 files, implements all IDE capabilities (completions, refactoring, navigation, diagnostics, formatting, etc.) by wrapping tsserver protocol calls into VSCode language provider interfaces

### TSServer Protocol & Client
- `extensions/typescript-language-features/src/tsServer/` — 21 files, manages bidirectional communication with TypeScript language server via stdio/IPC; handles process lifecycle, request queuing, response caching, cancellation, and file synchronization

### Test Suite
- `extensions/typescript-language-features/src/test/` — 18 files across unit and smoke test directories; validates provider behavior, protocol marshaling, and integration with VSCode APIs

### Configuration Layer
- `extensions/typescript-language-features/src/configuration/` — 8 files, centralizes all settings and document selector configuration with platform-specific overrides (Electron vs. Browser)

### Web/Browser Runtime
- `extensions/typescript-language-features/web/src/` — 11 files, provides WASM-based TypeScript server for browser environments with polyfills for Node APIs (fs, path, process) and worker-based session management

### Commands Interface
- `extensions/typescript-language-features/src/commands/` — 12 files, implements all user-facing commands (server restart, version selection, diagnostics, refactoring triggers)

---

## Summary

This partition documents the flagship TypeScript Language Features extension (168 files, ~22.5K LOC), which serves as a reference implementation of IDE intelligence via language server protocol. The extension demonstrates the core pattern for porting to Tauri/Rust:

**Key Architecture Patterns:**
1. **Language Provider Registration**: 28+ providers register with VSCode's `vscode.languages.register*` APIs, each wrapping tsserver protocol calls
2. **Bidirectional RPC**: `TypeScriptServiceClient` manages async request/response mapping with tsserver over stdio/IPC
3. **Platform Abstraction**: Configuration, process spawning, and filesystem access split between `.electron.ts` and `.browser.ts` variants
4. **Protocol Marshaling**: `typeConverters.ts` maps between TSServer types and VSCode protocol types; `protocol.d.ts` defines the full service interface
5. **Web/WASM Support**: Complete alternative implementation in `web/src/` swaps Node process with WASM server and implements browser-compatible filesystem/IPC
6. **Caching & Optimization**: Request caching, buffer synchronization, and response deduplication minimize server roundtrips

**Critical Porting Considerations:**
- Language provider registration system (vscode.languages.registerXxxProvider) requires VSCode API compatibility layer
- TSServer protocol (200+ request/response types) would need Rust serialization/deserialization
- Stdio-based IPC can be preserved; WASM server architecture already demonstrates non-Node alternatives
- Browser variant shows pattern for async execution in constrained environments
- Extensive test coverage validates protocol correctness and provider integration

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

1. `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/extension.ts`
2. `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/typescriptServiceClient.ts`
3. `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/tsServer/server.ts`
4. `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/tsServer/serverProcess.electron.ts`
5. `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts`
6. `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/tsServer/requestQueue.ts`
7. `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/tsServer/bufferSyncSupport.ts`
8. `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/typeConverters.ts`
9. `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/languageProvider.ts`
10. `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/tsServer/spawner.ts` (supporting)

---

### Per-File Notes (file:line)

#### `src/extension.ts`

- **Extension entry point** (`extension.ts:32`): `activate()` creates the `ElectronServiceProcessFactory` (`extension.ts:86`) and passes it via a service-object bag into `createLazyClientHost()`.
- **Platform factory injection** (`extension.ts:80–95`): All platform-specific objects — `ElectronServiceProcessFactory`, `NodeLogDirectoryProvider`, `nodeRequestCancellerFactory`, `DiskTypeScriptVersionProvider`, `ElectronServiceConfigurationProvider` — are instantiated here and injected into the client host. This is the seam where Electron-specific process spawning enters the pipeline.
- **TSGO conditional path** (`extension.ts:62–73`): When the experimental native TSGO extension is present, normal tsserver registration is bypassed entirely; only a `DisableTsgoCommand` is registered.
- **Temp-dir cleanup on deactivate** (`extension.ts:112–114`): `temp.instanceTempDir.value` is removed recursively on deactivation; this is Electron/Node-specific (`fs.rmSync`).

#### `src/typescriptServiceClient.ts`

- **ServerState enum** (`typescriptServiceClient.ts:49–96`): Three-state machine — `None`, `Running` (holds the live `ITypeScriptServer` handle and API version), `Errored`. All execute calls check this state before dispatching.
- **Server startup** (`typescriptServiceClient.ts:380–525`): `startService()` calls `typescriptServerSpawner.spawn(...)` (`typescriptServiceClient.ts:411`) to get an `ITypeScriptServer` handle, then wires `onError`, `onExit`, and `onEvent` callbacks. On server start, `serviceStarted()` is called which immediately fires a `configure` request with `hostInfo: 'vscode'` (`typescriptServiceClient.ts:601`).
- **Public execute methods** (`typescriptServiceClient.ts:858–938`):
  - `execute(command, args, token, config)` (`typescriptServiceClient.ts:858`): synchronous request expecting a response; handles `cancelOnResourceChange` by wrapping with a `CancellationTokenSource`.
  - `executeWithoutWaitingForResponse(command, args)` (`typescriptServiceClient.ts:914`): fire-and-forget; calls `executeImpl` with `expectsResult: false`.
  - `executeAsync(command, args, token)` (`typescriptServiceClient.ts:922`): for diagnostic requests (`geterr`).
  - All three delegate to private `executeImpl()` (`typescriptServiceClient.ts:930`), which calls `bufferSyncSupport.beforeCommand(command)` then `serverState.server.executeImpl(...)`.
- **Event dispatch** (`typescriptServiceClient.ts:971–1091`): `dispatchEvent()` is a large switch on `event.event` string — `syntaxDiag`, `semanticDiag`, `suggestionDiag`, `regionSemanticDiag` fire `_onDiagnosticsReceived`; `createDirectoryWatcher`/`createFileWatcher` events create VS Code `FileSystemWatcher` instances; `requestCompleted` events trigger performance telemetry.
- **Watch change aggregation** (`typescriptServiceClient.ts:1093–1107`): File-system events from VS Code are coalesced in a 100ms setTimeout window, then batched into a single `watchChange` RPC call to reduce IPC overhead.
- **URI conversion** (`typescriptServiceClient.ts:759–826`): `toTsFilePath()` maps VS Code URIs to tsserver-compatible path strings; non-`file` scheme URIs are prefixed with `inMemoryResourcePrefix` (`^`). `toResource()` reverses the mapping.
- **Crash recovery** (`typescriptServiceClient.ts:632–757`): After >5 restarts within 10s, `hasServerFatallyCrashedTooManyTimes` is set to `true` and restarts cease. Between 5 restarts in 5 minutes, a warning is shown.

#### `src/tsServer/server.ts`

- **`ITypeScriptServer` interface** (`server.ts:39–55`): Defines the contract any server wrapper must satisfy — `onEvent`, `onExit`, `onError` events; `kill()`; `executeImpl()` returning `Array<Promise<ServerResponse>>`.
- **`TsServerProcess` interface** (`server.ts:80–88`): Low-level I/O contract — `write(request)`, `onData(handler)`, `onExit(handler)`, `onError(handler)`, `kill()`. This is the boundary between the protocol layer and the actual process/worker.
- **`SingleTsServer`** (`server.ts:90–378`): The primary implementation:
  - Constructor (`server.ts:96–120`): wires `_process.onData` → `dispatchMessage`, `_process.onExit` → fires `_onExit` + destroys callbacks, `_process.onError` → fires `_onError` + destroys callbacks.
  - `dispatchMessage()` (`server.ts:147–185`): switches on `message.type`: `'response'` goes to `dispatchResponse()`; `'event'` with `requestCompleted` event resolves the matching callback via `seq`; all other events fire `_onEvent`. At the end of every dispatch, `sendNextRequests()` is called.
  - `dispatchResponse()` (`server.ts:209–226`): fetches the `CallbackItem` by `response.request_seq`, calls `callback.onSuccess(response)` on success or `callback.onError(TypeScriptServerError.create(...))` on failure.
  - `executeImpl()` (`server.ts:228–308`): Creates a `Proto.Request` via `_requestQueue.createRequest(command, args)`, wraps it in a `RequestItem`, optionally creates a `Promise` with a `CallbackItem` keyed by `seq`, enqueues it, then calls `sendNextRequests()`. If a SAB (SharedArrayBuffer) cancellation pointer is available, it attaches it to the request for synchronous cancellation on the browser.
  - `sendNextRequests()` (`server.ts:325–332`): Drains the queue **only while `_pendingResponses.size === 0`** — i.e., one request at a time for synchronous requests. Async requests (`isAsync: true`) do not block the queue (`sendRequest()` at `server.ts:338` only adds to `_pendingResponses` if `!requestItem.isAsync`).
  - `tryCancelRequest()` (`server.ts:187–207`): First tries to remove from the pending queue (pre-send); if already sent, calls `_requestCanceller.tryCancelOngoingRequest(seq)` (which writes to a named pipe on Electron).
- **`RequestRouter`** (`server.ts:389–471`): Routes commands to either syntax or semantic servers. Shared commands (`change`, `close`, `open`, `updateOpen`, `configure`) are dispatched to **all** servers but the first result is returned as primary. If one server resolves and another errors, `delegate.onFatalError()` is triggered.
- **`SyntaxRoutingTsServer`** (`server.ts:547–685`): Manages syntax + semantic server pair. `syntaxAlwaysCommands` (`server.ts:552–560`) always go to syntax server; `semanticCommands` (`server.ts:566–571`) always go to semantic server; `syntaxAllowedCommands` (`server.ts:576–589`) go to syntax during project loading, then upgrade to semantic after `projectLoadingFinish`/first diagnostics event. Uses `RequestRouter` internally.
- **`GetErrRoutingTsServer`** (`server.ts:474–544`): Routes `geterr`/`geterrForProject` to a dedicated diagnostics server; all other commands go to the main server. Diagnostic events from the getErr server and non-diagnostic events from the main server are selectively forwarded.

#### `src/tsServer/serverProcess.electron.ts`

- **`ProtocolBuffer`** (`serverProcess.electron.ts:34–98`): Stateful byte-buffer that parses the tsserver wire protocol. `tryReadContentLength()` (`serverProcess.electron.ts:60–83`) scans for `Content-Length: <N>\r\n\r\n` headers. `tryReadContent(length)` (`serverProcess.electron.ts:85–97`) extracts exactly `length` bytes as a UTF-8 string. The buffer uses `Buffer.allocUnsafe` with a default size of 8192 bytes and doubles when needed.
- **`Reader<T>`** (`serverProcess.electron.ts:100–142`): Wraps a Node.js `Readable` stream. On each `data` event, appends to `ProtocolBuffer`, then loops: reads content length, reads content bytes, parses JSON, fires `_onData`. This converts the raw stdio byte stream into typed `Proto.Response` objects.
- **`IpcChildServerProcess`** (`serverProcess.electron.ts:215–271`): Used when `useIpc` is true (TS >= 4.6.0 without custom node path). Sends requests via `child_process.send(serverRequest)` (IPC channel, structured clone); receives via `process.on('message', handler)`. Graceful shutdown sends an `exit` request then waits 5s before force-killing.
- **`StdioChildServerProcess`** (`serverProcess.electron.ts:273–340`): Used for stdio transport. `write()` (`serverProcess.electron.ts:287–289`) serializes requests as `JSON.stringify(serverRequest) + '\r\n'` to `stdin`. Data is received from `stdout` via the `Reader` class.
- **`ElectronServiceProcessFactory.fork()`** (`serverProcess.electron.ts:342–387`):
  - Determines `useIpc` = `!execPath && version.apiVersion?.gte(API.v460)` (`serverProcess.electron.ts:367`).
  - If a custom node path is set: `child_process.spawn(execPath, [...execArgv, tsServerPath, ...runtimeArgs])` (`serverProcess.electron.ts:372–376`).
  - Otherwise: `child_process.fork(tsServerPath, runtimeArgs, { silent: true, stdio: useIpc ? ['pipe','pipe','pipe','ipc'] : undefined })` (`serverProcess.electron.ts:377–383`).
  - Sets `ELECTRON_RUN_AS_NODE=1` in env when no custom exec path (`serverProcess.electron.ts:147–148`).
  - Returns either `IpcChildServerProcess` or `StdioChildServerProcess`.

#### `src/tsServer/serverProcess.browser.ts`

- **`WorkerServerProcess`** (`serverProcess.browser.ts:61–187`): Spawns tsserver as a `Worker` (`serverProcess.browser.ts:89`). Communication uses three `MessageChannel` pairs:
  - `_tsserver` channel (`serverProcess.browser.ts:75`): synchronous tsserver RPC — `write()` uses `this._tsserver.postMessage(serverRequest)` (`serverProcess.browser.ts:157`); responses arrive on `_tsserver.onmessage` and are dispatched to `_onDataHandlers`.
  - `_watcher` channel (`serverProcess.browser.ts:76`): asynchronous file-watching events from the worker; `onmessage` handler creates/disposes VS Code file watchers and posts watch events back.
  - `_syncFs` channel (`serverProcess.browser.ts:77`): synchronous filesystem access via `@vscode/sync-api-service`'s `ApiService` (`serverProcess.browser.ts:152–154`).
- The initial handshake: `worker.postMessage({ args, extensionUri }, [syncChannel.port1, tsserverChannel.port1, watcherChannel.port1])` transfers the three `MessagePort` objects to the worker (`serverProcess.browser.ts:147–150`).
- `kill()` terminates the worker and closes all three ports (`serverProcess.browser.ts:174–180`).
- `onExit` is not implemented in the browser variant (`serverProcess.browser.ts:169–171`).

#### `src/tsServer/requestQueue.ts`

- **Three queueing types** (`requestQueue.ts:8–26`): `Normal` (1), `LowPriority` (2), `Fence` (3).
- **`enqueue()`** (`requestQueue.ts:43–57`): `Normal` items insert before the last contiguous block of `LowPriority` items using `splice`; all other types append to tail.
- **`createRequest()`** (`requestQueue.ts:89–96`): Stamps a monotonically incrementing `sequenceNumber` onto the request as `seq`, sets `type: 'request'`.
- **Fence commands** (`server.ts:367`): `change`, `close`, `open`, `updateOpen` are `Fence` type — they never jump ahead of `LowPriority` items, ensuring document-mutation operations are not reordered.

#### `src/tsServer/bufferSyncSupport.ts`

- **`BufferSynchronizer`** (`bufferSyncSupport.ts:67–158`): Batches open/close/change operations in a `ResourceMap<BufferOperation>`. `beforeCommand()` (`bufferSyncSupport.ts:111–117`) flushes the pending map (except for `updateOpen` itself). `flush()` (`bufferSyncSupport.ts:119–134`) accumulates all pending ops and fires a single `updateOpen` RPC with arrays of `closedFiles`, `openFiles`, and `changedFiles`.
- **`SyncedBuffer`** (`bufferSyncSupport.ts:160–248`): Wraps a VS Code `TextDocument`. `open()` (`bufferSyncSupport.ts:171–192`) builds an `OpenRequestArgs` including full `fileContent` and `scriptKindName`. `onContentChanged()` (`bufferSyncSupport.ts:241–247`) maps `vscode.TextDocumentContentChangeEvent[]` to `Proto.CodeEdit[]` (via `typeConverters.Position.toLocation()`), **reversing** the edits array to send end-of-document first (`bufferSyncSupport.ts:103`).
- **`GetErrRequest`** (`bufferSyncSupport.ts:275–368`): Encapsulates a `geterr` or `geterrForProject` call. On TS >= 5.6 (`API.v560`), sends `geterr` with `Proto.FileRangesRequestArgs` for visible viewport ranges.
- **Diagnostic delay** (`bufferSyncSupport.ts:710–713`): Delay is clamped: `Math.min(Math.max(Math.ceil(lineCount / 20), 300), 800)` ms, scaling with file size between 300ms and 800ms.
- **`interruptGetErr()`** (`bufferSyncSupport.ts:631–643`): Cancels any in-flight `geterr`, runs the provided function synchronously, then re-triggers diagnostics.

#### `src/typeConverters.ts`

- **Coordinate system transformation** (`typeConverters.ts:16–69`): tsserver uses 1-based line/offset; VS Code uses 0-based line/character.
  - `Position.fromLocation()` (`typeConverters.ts:56–57`): subtracts 1 from both dimensions.
  - `Position.toLocation()` (`typeConverters.ts:59–62`): adds 1 to both dimensions.
  - `Range.fromLocations()` (`typeConverters.ts:24–27`): clamps to `Math.max(0, ...)` to handle edge cases.
- **`WorkspaceEdit.fromFileCodeEdits()`** (`typeConverters.ts:84–106`): Converts tsserver `FileCodeEdits[]` (array of `{fileName, textChanges[]}`) into a VS Code `WorkspaceEdit` by calling `client.toResource(edit.fileName)` to resolve filenames back to URIs, then applying `Range.fromTextSpan()` + `newText` for each change.
- **`SymbolKind.fromProtocolScriptElementKind()`** (`typeConverters.ts:110–136`): Maps tsserver `ScriptElementKind` strings to `vscode.SymbolKind` enum values.

#### `src/languageProvider.ts`

- **Provider registration** (`languageProvider.ts:64–99`): `registerProviders()` dynamically `import()`s ~25 language feature modules (completions, definitions, hover, references, rename, etc.) in parallel via `Promise.all`. Each module exports a `register(selector, client, ...)` function that returns a `Disposable`.
- **Document selector** (`languageProvider.ts:51–62`): Builds separate `semantic` and `syntax` selector arrays. Semantic selectors restrict to `fileSchemes.getSemanticSupportedSchemes()` (file, vscode-vfs, etc.); syntax selectors have no scheme restriction.
- **Config-driven validation** (`languageProvider.ts:40–46`): Uses `UnifiedConfigValue` to read `validate.enabled` with a fallback to `validate.enable`, watching for config changes and calling `updateValidate()`.

#### `src/tsServer/spawner.ts`

- **Composite server topology** (`spawner.ts:24–36`): Four modes — `Single`, `SeparateSyntax`, `DynamicSeparateSyntax`, `SyntaxOnly`.
- **Version-gated routing** (`spawner.ts:110–122`): `DynamicSeparateSyntax` is used for TS >= 4.0 with `Auto` syntax server config; older versions use `SeparateSyntax`.
- **`spawnTsServer()`** (`spawner.ts:130–173`): Calls `_factory.fork(version, args, kind, ...)` (the injected `TsServerProcessFactory`), then wraps the result in a `SingleTsServer`. The `factory.fork()` call is the sole site where the process is actually created.
- **Args assembly** (`spawner.ts:188+`): Assembles CLI args including `--cancellationPipeName`, `--logFile`, `--logVerbosity`, `--globalPlugins`, `--pluginProbeLocations`, `--locale`, `--noGetErrOnBackgroundUpdate`, `--validateDefaultNpmLocation`, etc.

---

### Cross-Cutting Synthesis (≤200 words)

The extension implements a full request/response RPC pipeline to tsserver. Requests flow: `LanguageProvider` → `TypeScriptServiceClient.execute()` → `executeImpl()` (checks `bufferSyncSupport.beforeCommand()` to flush buffered edits first) → `SingleTsServer.executeImpl()` (stamps a monotonic `seq`, enqueues in `RequestQueue`, stores a `CallbackItem<Promise>` keyed by `seq`) → `sendRequest()` → `TsServerProcess.write()`. On Electron, `write()` is either `process.send()` (IPC) or `stdin.write(JSON + '\r\n')` (stdio). tsserver responds with `Content-Length: N\r\n\r\nJSON`, parsed by `ProtocolBuffer`/`Reader`. The `dispatchMessage()` loop matches `response.request_seq` to the stored callback and resolves the promise. Events (diagnostics, watchers) arrive out-of-band and are dispatched through `TypeScriptServiceClient.dispatchEvent()`.

**Porting to Rust/Tauri implications:** tsserver is a Node.js/JS process; a Rust host must still spawn it as a child process (using `std::process::Command` or Tauri's sidecar API) or run it in WASM. The stdio/IPC wire protocol is straightforward JSON with `Content-Length` framing, implementable in Rust with `serde_json`. Type marshaling requires reimplementing `typeConverters.ts` (1-based/0-based coordinate transforms) and all `Proto.*` types as Rust structs. The `RequestQueue` priority logic, `CallbackMap` (seq → future), `BufferSynchronizer` (batched `updateOpen`), and server-topology routing (`SyntaxRoutingTsServer`, `GetErrRoutingTsServer`) must each be ported. The browser WASM path (running tsserver as a Web Worker with `MessageChannel` and `@vscode/sync-api-service` for synchronous FS) would need replacement with Tauri's async FS bridge. Cancellation uses named pipes (Electron) or SAB (browser); Rust would need an equivalent — likely named pipes via `tokio`.

---

### Out-of-Partition References

- `src/typescriptService.ts` — defines `ITypeScriptServiceClient`, `ServerResponse`, `TypeScriptRequests`, `ClientCapability`, `ExecConfig` interfaces used throughout.
- `src/tsServer/callbackMap.ts` — `CallbackMap<T>` keying `seq → CallbackItem`; used in `SingleTsServer`.
- `src/tsServer/cancellation.ts` / `src/tsServer/cancellation.electron.ts` — `OngoingRequestCancellerFactory` creates named-pipe cancellers; referenced in `spawner.ts` and `server.ts`.
- `src/tsServer/cachedResponse.ts` — `CachedResponse` wraps navtree requests with a cache; used in `languageProvider.ts`.
- `src/languageFeatures/diagnostics.ts` — `DiagnosticsManager`; receives `TsDiagnostics` fired by `typescriptServiceClient.ts`.
- `src/tsServer/fileWatchingManager.ts` — browser file-watcher lifecycle; used in `serverProcess.browser.ts`.
- `@vscode/sync-api-common` / `@vscode/sync-api-service` — synchronous FS API bridge for the browser WASM path; imported in `serverProcess.browser.ts`.
- `extensions/typescript-language-features/src/lazyClientHost.ts` — `createLazyClientHost()` wires `TypeScriptServiceClient` + `LanguageProvider`; called from `extension.ts:80`.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Analysis: TypeScript Language Features Extension (Partition 8/79)
## LSP Integration via tsserver in VS Code

### Research Question
What patterns exist for porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust?

**Focus Area:** `extensions/typescript-language-features/` — 168 files, 22,571 LOC
**Scope:** LSP-ish integration via tsserver, provider registration, and request handling

---

## Discovered Patterns

#### Pattern 1: Hierarchical Provider Registration with Lazy Loading
**Where:** `extensions/typescript-language-features/src/languageProvider.ts:64-100`
**What:** Each language feature is registered dynamically via Promise.all with conditional lazy imports, centered on documentSelector (semantic vs syntax).

```typescript
private async registerProviders(): Promise<void> {
	const selector = this.documentSelector;
	const cachedNavTreeResponse = new CachedResponse();

	await Promise.all([
		import('./languageFeatures/callHierarchy').then(provider => 
			this._register(provider.register(selector, this.client))),
		import('./languageFeatures/codeLens/implementationsCodeLens').then(provider => 
			this._register(provider.register(selector, this.description, this.client, cachedNavTreeResponse))),
		import('./languageFeatures/definitions').then(provider => 
			this._register(provider.register(selector, this.client))),
		// ... 25+ more providers
	]);
}
```

**Variations / call-sites:**
- `extensions/typescript-language-features/src/languageFeatures/definitions.ts:63` — Definition provider registration
- `extensions/typescript-language-features/src/languageFeatures/hover.ts:105` — Hover provider registration
- `extensions/typescript-language-features/src/languageFeatures/completions.ts:929` — Completion provider registration
- All 32 provider modules follow the same `export function register(selector, client, ...)` pattern

---

#### Pattern 2: Conditional Provider Registration Based on Server Capabilities
**Where:** `extensions/typescript-language-features/src/languageFeatures/util/dependentRegistration.ts:75-81`
**What:** A Condition-based system enables/disables provider registration dynamically when server capabilities or configurations change.

```typescript
export function conditionalRegistration(
	conditions: readonly Condition[],
	doRegister: () => vscode.Disposable,
	elseDoRegister?: () => vscode.Disposable
): vscode.Disposable {
	return new ConditionalRegistration(conditions, doRegister, elseDoRegister);
}

// Used in definitions.ts:67-72:
export function register(selector: DocumentSelector, client: ITypeScriptServiceClient) {
	return conditionalRegistration([
		requireSomeCapability(client, ClientCapability.EnhancedSyntax, ClientCapability.Semantic),
	], () => {
		return vscode.languages.registerDefinitionProvider(selector.syntax,
			new TypeScriptDefinitionProvider(client));
	});
}
```

**Variations / call-sites:**
- `extensions/typescript-language-features/src/languageFeatures/hover.ts:110` — Requires EnhancedSyntax or Semantic
- `extensions/typescript-language-features/src/languageFeatures/completions.ts:942` — Requires Semantic
- `extensions/typescript-language-features/src/languageFeatures/implementations.ts:25` — Requires Semantic

---

#### Pattern 3: Client.execute() for Async tsserver Commands with Token Cancellation
**Where:** `extensions/typescript-language-features/src/languageFeatures/definitions.ts:17-60`
**What:** Providers use `client.execute(command, args, cancellationToken)` to send typed requests to tsserver, handling response.type checking and conversion.

```typescript
public async provideDefinition(
	document: vscode.TextDocument,
	position: vscode.Position,
	token: vscode.CancellationToken
): Promise<vscode.DefinitionLink[] | vscode.Definition | undefined> {
	const filepath = this.client.toOpenTsFilePath(document);
	if (!filepath) {
		return undefined;
	}

	const args = typeConverters.Position.toFileLocationRequestArgs(filepath, position);
	const response = await this.client.execute('definitionAndBoundSpan', args, token);
	if (response.type !== 'response' || !response.body) {
		return undefined;
	}

	return response.body.definitions.map((location): vscode.DefinitionLink => {
		const target = typeConverters.Location.fromTextSpan(this.client.toResource(location.file), location);
		return {
			originSelectionRange: span,
			targetRange: typeConverters.Range.fromLocations(location.contextStart, location.contextEnd),
			targetUri: target.uri,
			targetSelectionRange: target.range,
		};
	});
}
```

**Variations / call-sites:**
- `extensions/typescript-language-features/src/languageFeatures/hover.ts:45` — quickinfo request
- `extensions/typescript-language-features/src/languageFeatures/completions.ts:761` — completionInfo request
- `extensions/typescript-language-features/src/languageFeatures/rename.ts:136` — rename request
- 40+ call-sites across all language features

---

#### Pattern 4: interruptGetErr() Pattern for Prioritizing User-Facing Requests
**Where:** `extensions/typescript-language-features/src/languageFeatures/hover.ts:42-46`
**What:** Client.interruptGetErr wraps requests that should interrupt background diagnostic checks, prioritizing interactive user actions.

```typescript
const response = await this.client.interruptGetErr(async () => {
	await this.fileConfigurationManager.ensureConfigurationForDocument(document, token);
	return this.client.execute('quickinfo', args, token);
});
```

**Variations / call-sites:**
- `extensions/typescript-language-features/src/languageFeatures/completions.ts:192` — completionEntryDetails
- `extensions/typescript-language-features/src/languageFeatures/completions.ts:761` — completionInfo
- `extensions/typescript-language-features/src/languageFeatures/copyPaste.ts:102` — preparePasteEdits
- `extensions/typescript-language-features/src/languageFeatures/refactor.ts:599` — interruptGetErrIfNeeded wrapper
- 15+ interactive provider call-sites

---

#### Pattern 5: Cached Responses for Repeated Calls on Same Document
**Where:** `extensions/typescript-language-features/src/tsServer/cachedResponse.ts:15-48`
**What:** CachedResponse caches promise-based tsserver responses keyed by document URI and version, reusing results until the document changes.

```typescript
export class CachedResponse<T extends Proto.Response> {
	private response?: Promise<ServerResponse.Response<T>>;
	private version: number = -1;
	private document: string = '';

	public execute(
		document: vscode.TextDocument,
		resolve: Resolve<T>
	): Promise<ServerResponse.Response<T>> {
		if (this.response && this.matches(document)) {
			return this.response = this.response.then(result => 
				result.type === 'cancelled' ? resolve() : result);
		}
		return this.reset(document, resolve);
	}

	private matches(document: vscode.TextDocument): boolean {
		return this.version === document.version && this.document === document.uri.toString();
	}
}
```

**Variations / call-sites:**
- `extensions/typescript-language-features/src/languageProvider.ts:67` — Shared instance for codeLens providers
- `extensions/typescript-language-features/src/languageFeatures/documentSymbol.ts:52` — navto caching
- `extensions/typescript-language-features/src/languageFeatures/refactor.ts:540` — getApplicableRefactors caching

---

#### Pattern 6: Browser/Worker-Based tsserver via MessagePort
**Where:** `extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts:61-187`
**What:** Web worker implementation of TsServerProcess uses MessagePorts for three channels: tsserver (sync), watcher (file events), and syncFs (filesystem).

```typescript
class WorkerServerProcess implements TsServerProcess {
	private readonly _tsserver: MessagePort;
	private readonly _watcher: MessagePort;
	private readonly _syncFs: MessagePort;

	constructor(...) {
		const tsserverChannel = new MessageChannel();
		const watcherChannel = new MessageChannel();
		const syncChannel = new MessageChannel();
		this._tsserver = tsserverChannel.port2;
		this._watcher = watcherChannel.port2;
		this._syncFs = syncChannel.port2;

		this._tsserver.onmessage = (event) => {
			for (const handler of this._onDataHandlers) {
				handler(event.data);
			}
		};

		this._worker.postMessage(
			{ args, extensionUri },
			[syncChannel.port1, tsserverChannel.port1, watcherChannel.port1]
		);
	}

	write(serverRequest: Proto.Request): void {
		this._tsserver.postMessage(serverRequest);
	}
}
```

**Variations / call-sites:**
- `extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts:100-127` — Message dispatch for three channels
- `extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts:147-154` — Worker initialization with ApiService

---

#### Pattern 7: Request Queue with Priority Levels and Reordering
**Where:** `extensions/typescript-language-features/src/tsServer/requestQueue.ts:35-97`
**What:** RequestQueue implements request prioritization: Normal requests can jump ahead of LowPriority requests, but Fence requests block all reordering.

```typescript
export enum RequestQueueingType {
	Normal = 1,        // Can reorder
	LowPriority = 2,   // Gets pushed behind Normal
	Fence = 3,         // Blocks reordering, goes to end
}

export class RequestQueue {
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
- `extensions/typescript-language-features/src/tsServer/server.ts:91-92` — SingleTsServer uses RequestQueue
- `extensions/typescript-language-features/src/tsServer/cancellation.ts` — OngoingRequestCanceller integrates with queue

---

## Port Feasibility Analysis

### Direct Transferable Patterns
1. **Provider registration hierarchy** — Directly portable: Provider registration with capability checks is language-agnostic
2. **Conditional registration based on capabilities** — Portable: The Condition-based system has no platform dependencies
3. **Request/response command pattern** — Portable: Execute-with-args-and-cancellation is standard LSP
4. **Cached responses** — Portable: Document version tracking is implementation-agnostic
5. **Request queue prioritization** — Portable: The algorithm is pure data structure logic

### Architecture Dependencies Requiring Redesign
- **Worker-based tsserver communication** — Tauri would use native plugins/IPC instead of Web Workers
- **interruptGetErr diagnostics interruption** — Would need Rust-side diagnostic manager redesign
- **ConditionalRegistration with vscode events** — Would need equivalent capability change event system in Tauri API

### LSP-Specific Requirements for Porting
- tsserver protocol definitions must map to LSP equivalents or be reimplemented in Rust
- 32 provider types (definition, completion, hover, etc.) require LSP server implementations
- DocumentSelector filtering by language/scheme needs Tauri equivalent
- Cancellation tokens require async/await cancellation handling in Rust

---

## Summary

The TypeScript Language Features extension demonstrates a **modular, capability-driven architecture** for IDE features:

- **Modular providers**: 32 independent feature modules with a standard registration interface
- **Capability-based delivery**: Features are conditionally available based on server capabilities
- **Priority-aware request handling**: Three-tier queue system ensures interactive features stay responsive
- **Response caching**: Document-version-keyed caching reduces redundant computations
- **Worker isolation**: Browser/web deployment uses MessagePort channels for three concurrent data flows

The core request/response and provider registration patterns are highly portable to Rust + Tauri, but the diagnostic interruption strategy, worker communication model, and event-driven capability system would require substantial redesign. The LSP protocol itself provides a migration path, though VS Code's extended protocol features (like getCombinedCodeFix) may not have direct equivalents.

## External References
<!-- Source: codebase-online-researcher sub-agent -->
# Library Card: TypeScript tsserver Protocol (`typescript/src/server/protocol.ts`)

## Identity

| Field | Value |
|---|---|
| **Library / Spec** | TypeScript Language Server Protocol (`tsserver`) |
| **Canonical source** | `typescript/src/server/protocol.ts` (in the TypeScript compiler repo) |
| **Local mirror** | `extensions/typescript-language-features/src/tsServer/protocol/protocol.d.ts` (thin re-export: `export = ts.server.protocol`) |
| **Version sampled** | TypeScript `main` branch (April 2026) — 3 321-line file |
| **Why central** | Every language intelligence feature in `extensions/typescript-language-features/` — completions, hover, go-to-definition, references, rename, diagnostics, inlay hints, call hierarchy, code actions, refactors — is implemented by sending JSON messages that conform to the interfaces defined in this single file. |

---

## Transport Layer (wire format)

The extension forks `tsserver` as a child process (or worker) and communicates over its **stdin/stdout** using an HTTP-header-style framing protocol identical to LSP:

```
Content-Length: <byte-count>\r\n
\r\n
<JSON body>
```

Key implementation is in `extensions/typescript-language-features/src/tsServer/serverProcess.electron.ts`. The `ProtocolBuffer` class parses the `Content-Length:` header, then slices the exact byte count from a growing `Buffer`. JSON is then parsed and dispatched. This is **not** LSP — it is tsserver's own binary-framing over stdio, predating LSP.

---

## Top-level Message Shape

Every message (request, response, or event) is a subtype of `Message`:

```typescript
export interface Message {
    seq: number;
    type: "request" | "response" | "event";
}
```

**Client-to-server (`Request`):**
```typescript
export interface Request extends Message {
    type: "request";
    command: string;   // value from CommandTypes enum
    arguments?: any;
}
```

**Server-to-client (`Response`):**
```typescript
export interface Response extends Message {
    type: "response";
    request_seq: number;
    success: boolean;
    command: string;
    message?: string;
    body?: any;
    metadata?: unknown;
    performanceData?: PerformanceData;
}
```

**Server-pushed (`Event`):**
```typescript
export interface Event extends Message {
    type: "event";
    event: string;   // e.g. "semanticDiag", "syntaxDiag", "suggestionDiag"
    body?: any;
}
```

---

## `CommandTypes` Enum (full surface, selected)

The complete public API is encoded as `const enum CommandTypes`. Key values used by the VS Code extension (from `typescriptService.ts`'s `TypeScriptRequests` map):

| Command string | Purpose |
|---|---|
| `"completionInfo"` | Trigger completion list |
| `"completionEntryDetails"` | Detailed completion item docs |
| `"quickinfo"` | Hover / type-at-cursor |
| `"definition"` | Go to definition |
| `"definitionAndBoundSpan"` | Go to definition + highlight span |
| `"typeDefinition"` | Go to type definition |
| `"implementation"` | Go to implementation |
| `"references"` | Find all references |
| `"fileReferences"` | Find file-level references |
| `"rename"` | Rename symbol |
| `"signatureHelp"` | Parameter hints |
| `"documentHighlights"` | Highlight all occurrences |
| `"navto"` | Workspace symbol search |
| `"navtree"` | Document symbol outline |
| `"format"` / `"formatonkey"` | Formatting |
| `"getApplicableRefactors"` | List refactors at cursor |
| `"getEditsForRefactor"` | Execute refactor |
| `"organizeImports"` | Organize imports |
| `"getCodeFixes"` | Code fixes (quick fixes) |
| `"getCombinedCodeFix"` | Batch code fix |
| `"geterr"` / `"geterrForProject"` | Pull diagnostics (async) |
| `"semanticDiagnosticsSync"` | Sync semantic diagnostics |
| `"syntacticDiagnosticsSync"` | Sync syntactic diagnostics |
| `"provideInlayHints"` | Inlay hints |
| `"prepareCallHierarchy"` | Call hierarchy root |
| `"provideCallHierarchyIncomingCalls"` | Callers |
| `"provideCallHierarchyOutgoingCalls"` | Callees |
| `"selectionRange"` | Smart selection expand |
| `"open"` / `"close"` / `"change"` / `"updateOpen"` | File lifecycle |
| `"configure"` | Set host/format preferences |
| `"compilerOptionsForInferredProjects"` | Configure inferred project |
| `"getEditsForFileRename"` | Rename file side-effects |
| `"mapCode"` | Copilot code mapping |
| `"getPasteEdits"` / `"preparePasteEdits"` | Smart paste |

---

## Selected Critical Request/Response Pairs

### File Location (basis for most requests)
```typescript
export interface FileLocationRequestArgs extends FileRequestArgs {
    line: number;   // 1-based
    offset: number; // 1-based (character offset on the line)
}
```
Note: **1-based** line and character, unlike LSP which uses 0-based. The `typeConverters.ts` file bridges this:
```typescript
// vscode Position (0-based) -> tsserver Location (1-based)
export const toLocation = (vsPosition: vscode.Position): Proto.Location => ({
    line: vsPosition.line + 1,
    offset: vsPosition.character + 1,
});
```

### Quickinfo (Hover)
```typescript
export interface QuickInfoResponseBody {
    kind: ScriptElementKind;
    kindModifiers: string;
    start: Location;
    end: Location;
    displayString: string;
    documentation: string | SymbolDisplayPart[];
    tags: JSDocTagInfo[];
    canIncreaseVerbosityLevel?: boolean;
}
```

### Completions
```typescript
export interface CompletionsRequestArgs extends FileLocationRequestArgs {
    prefix?: string;
    triggerCharacter?: CompletionsTriggerCharacter;
    triggerKind?: CompletionTriggerKind;
    includeExternalModuleExports?: boolean;
    includeInsertTextCompletions?: boolean;
}
```

### Diagnostics (push, event-based)
Diagnostics are NOT returned synchronously per request. The client sends `geterr` or `geterrForProject`; the server pushes back `Event` messages with `event` set to:
- `"syntaxDiag"` — syntax errors
- `"semanticDiag"` — type errors
- `"suggestionDiag"` — suggestions / hints

### Go-to-definition response
```typescript
export interface DefinitionInfo extends FileSpanWithContext {
    unverified?: boolean;
}
export interface DefinitionInfoAndBoundSpan {
    definitions: readonly DefinitionInfo[];
    textSpan: TextSpan;   // the span at the call site
}
```

### Code Edit (used in all write-back features)
```typescript
export interface CodeEdit {
    start: Location;
    end: Location;
    newText: string;
}
export interface FileCodeEdits {
    fileName: string;
    textChanges: CodeEdit[];
}
```

### References
```typescript
export interface ReferencesResponseItem extends FileSpanWithContext {
    lineText?: string;
    isWriteAccess: boolean;
    isDefinition?: boolean;
}
export interface ReferencesResponseBody {
    refs: readonly ReferencesResponseItem[];
    symbolName: string;
    symbolStartOffset: number;
    symbolDisplayString: string;
}
```

---

## TypeScript Requests Map (complete, from `typescriptService.ts`)

The extension declares its full tsserver API surface in a single TypeScript type:

```typescript
interface StandardTsServerRequests {
    'applyCodeActionCommand': [ApplyCodeActionCommandRequestArgs, ApplyCodeActionCommandResponse];
    'completionEntryDetails': [CompletionDetailsRequestArgs, CompletionDetailsResponse];
    'completionInfo': [CompletionsRequestArgs, CompletionInfoResponse];
    'completions': [CompletionsRequestArgs, CompletionsResponse];
    'configure': [ConfigureRequestArguments, ConfigureResponse];
    'definition': [FileLocationRequestArgs, DefinitionResponse];
    'definitionAndBoundSpan': [FileLocationRequestArgs, DefinitionInfoAndBoundSpanResponse];
    'docCommentTemplate': [FileLocationRequestArgs, DocCommandTemplateResponse];
    'documentHighlights': [DocumentHighlightsRequestArgs, DocumentHighlightsResponse];
    'format': [FormatRequestArgs, FormatResponse];
    'formatonkey': [FormatOnKeyRequestArgs, FormatResponse];
    'getApplicableRefactors': [GetApplicableRefactorsRequestArgs, GetApplicableRefactorsResponse];
    'getCodeFixes': [CodeFixRequestArgs, CodeFixResponse];
    'getCombinedCodeFix': [GetCombinedCodeFixRequestArgs, GetCombinedCodeFixResponse];
    'getEditsForFileRename': [GetEditsForFileRenameRequestArgs, GetEditsForFileRenameResponse];
    'getEditsForRefactor': [GetEditsForRefactorRequestArgs, GetEditsForRefactorResponse];
    'getOutliningSpans': [FileRequestArgs, OutliningSpansResponse];
    'getSupportedCodeFixes': [null, GetSupportedCodeFixesResponse];
    'implementation': [FileLocationRequestArgs, ImplementationResponse];
    'jsxClosingTag': [JsxClosingTagRequestArgs, JsxClosingTagResponse];
    'navto': [NavtoRequestArgs, NavtoResponse];
    'navtree': [FileRequestArgs, NavTreeResponse];
    'organizeImports': [OrganizeImportsRequestArgs, OrganizeImportsResponse];
    'projectInfo': [ProjectInfoRequestArgs, ProjectInfoResponse];
    'quickinfo': [FileLocationRequestArgs, QuickInfoResponse];
    'references': [FileLocationRequestArgs, ReferencesResponse];
    'rename': [RenameRequestArgs, RenameResponse];
    'selectionRange': [SelectionRangeRequestArgs, SelectionRangeResponse];
    'signatureHelp': [SignatureHelpRequestArgs, SignatureHelpResponse];
    'typeDefinition': [FileLocationRequestArgs, TypeDefinitionResponse];
    'updateOpen': [UpdateOpenRequestArgs, Response];
    'prepareCallHierarchy': [FileLocationRequestArgs, PrepareCallHierarchyResponse];
    'provideCallHierarchyIncomingCalls': [FileLocationRequestArgs, ProvideCallHierarchyIncomingCallsResponse];
    'provideCallHierarchyOutgoingCalls': [FileLocationRequestArgs, ProvideCallHierarchyOutgoingCallsResponse];
    'fileReferences': [FileRequestArgs, FileReferencesResponse];
    'provideInlayHints': [InlayHintsRequestArgs, InlayHintsResponse];
    'encodedSemanticClassifications-full': [EncodedSemanticClassificationsRequestArgs, EncodedSemanticClassificationsResponse];
    'findSourceDefinition': [FileLocationRequestArgs, DefinitionResponse];
    'getMoveToRefactoringFileSuggestions': [GetMoveToRefactoringFileSuggestionsRequestArgs, GetMoveToRefactoringFileSuggestions];
    'linkedEditingRange': [FileLocationRequestArgs, LinkedEditingRangeResponse];
    'mapCode': [MapCodeRequestArgs, MapCodeResponse];
    'getPasteEdits': [GetPasteEditsRequestArgs, GetPasteEditsResponse];
    'preparePasteEdits': [PreparePasteEditsRequestArgs, PreparePasteEditsResponse];
}

interface NoResponseTsServerRequests {
    'open': [OpenRequestArgs, null];
    'close': [FileRequestArgs, null];
    'change': [ChangeRequestArgs, null];
    'compilerOptionsForInferredProjects': [SetCompilerOptionsForInferredProjectsArgs, null];
    'reloadProjects': [null, null];
    'configurePlugin': [ConfigurePluginRequest, ConfigurePluginResponse];
    'watchChange': [Request, null];
}

interface AsyncTsServerRequests {
    'geterr': [GeterrRequestArgs, Response];
    'geterrForProject': [GeterrForProjectRequestArgs, Response];
}
```

---

## Relationship to LSP

The tsserver protocol predates and differs from LSP in several ways:

| Dimension | tsserver | LSP |
|---|---|---|
| Transport | stdin/stdout with `Content-Length` framing | Same framing, or sockets |
| Position encoding | 1-based line and 1-based character offset | 0-based line and 0-based character |
| Diagnostics | Push-based events (`geterr` triggers async events) | Push-based notifications (`textDocument/publishDiagnostics`) |
| Method dispatch | `command` string field (`CommandTypes` enum) | `method` string field (`textDocument/completion`, etc.) |
| File lifecycle | Explicit `open`/`close`/`change` commands | `textDocument/didOpen`, `didChange`, `didClose` |
| Spec | Not publicly versioned; defined by `protocol.ts` in TypeScript repo | Versioned JSON Schema at microsoft.github.io/language-server-protocol |

The extension does NOT use LSP internally; it speaks tsserver's own protocol directly. A separate `typescript-language-server` (community project, `typescript-language-features` does not use it) wraps tsserver behind a true LSP facade.

---

## Implications for Tauri/Rust Port

1. **tsserver process must still run**: TypeScript analysis requires the TypeScript compiler runtime. A Tauri/Rust shell cannot replace `tsserver` itself; it must still spawn Node.js running `tsserver` (or the bundled TypeScript worker in the web variant).

2. **IPC bridge required**: The Rust process (Tauri backend) or its WebView (frontend) must replicate the `ProtocolBuffer` framing logic: write `Content-Length: N\r\n\r\n{JSON}` to tsserver's stdin, and parse the same framing from stdout.

3. **Coordinator split**: Today `typescriptServiceClient.ts` runs in the extension host (Node.js). In Tauri it would need to live in either the WebView's JS context (calling through Tauri's IPC bridge to a Rust-owned child process) or in a Rust async task managing the child process.

4. **Coordinate system translation**: All position conversions (1-based tsserver ↔ 0-based VS Code `Position`) are concentrated in `typeConverters.ts`. This translation layer must be preserved.

5. **Multiple server instances**: VS Code runs up to 3 concurrent tsserver instances (`main`, `syntax`, `semantic`, `diagnostics` — see `TsServerProcessKind`). Each instance speaks the same protocol but handles different request subsets. The Tauri port must manage multiple long-lived child processes.

6. **No LSP migration shortcut without loss**: Migrating to `typescript-language-server` (true LSP) would simplify the bridge but drops tsserver-specific commands (`mapCode`, `preparePasteEdits`, `encodedSemanticClassifications-full`, Copilot-related internals) that have no LSP equivalent.

---

## Source References

- `typescript/src/server/protocol.ts` — https://github.com/microsoft/TypeScript/blob/main/src/server/protocol.ts (3 321 lines; canonical)
- Local re-export: `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/tsServer/protocol/protocol.d.ts` (23 lines; thin shim)
- TypeScript request surface: `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/typescriptService.ts` (100 lines; `TypeScriptRequests` type)
- Wire framing: `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/tsServer/serverProcess.electron.ts`
- Coordinate converters: `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/typeConverters.ts`

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
