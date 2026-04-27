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
