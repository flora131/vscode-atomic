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
