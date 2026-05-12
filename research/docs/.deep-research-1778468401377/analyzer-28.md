# Analysis: extensions/css-language-features — LSP Client/Server, Dual-Runtime

## Files Analysed

1. `extensions/css-language-features/client/src/cssClient.ts`
2. `extensions/css-language-features/client/src/node/cssClientMain.ts`
3. `extensions/css-language-features/client/src/browser/cssClientMain.ts`
4. `extensions/css-language-features/client/src/customData.ts`
5. `extensions/css-language-features/client/src/requests.ts`
6. `extensions/css-language-features/server/src/cssServer.ts`
7. `extensions/css-language-features/server/src/node/cssServerMain.ts`
8. `extensions/css-language-features/server/src/browser/cssServerMain.ts`
9. `extensions/css-language-features/server/src/languageModelCache.ts`
10. `extensions/css-language-features/server/src/utils/validation.ts`
11. `extensions/css-language-features/server/src/customData.ts`
12. `extensions/css-language-features/server/src/node/nodeFs.ts`
13. `extensions/css-language-features/server/src/utils/documentContext.ts`
14. `extensions/css-language-features/server/src/utils/runner.ts`

---

## Per-File Notes

### `extensions/css-language-features/client/src/cssClient.ts`

- **Role:** Shared client initialization for both Node and browser runtimes. Creates the LSP client, registers middleware, manages formatter registrations, and connects custom data change notifications.
- **Key symbols:**
  - `startClient` (line 39) — async factory function; accepts `context`, `newLanguageClient: LanguageClientConstructor`, and `runtime: Runtime`; returns `BaseLanguageClient`
  - `LanguageClientConstructor` (line 15) — type alias for `(name, description, clientOptions) => BaseLanguageClient`
  - `Runtime` interface (lines 17–20) — carries `TextDecoder` and optional `fs: RequestService`; used to abstract Node vs. browser I/O
  - `CustomDataChangedNotification.type` (line 12) — `NotificationType<string[]>` with method `'css/customDataChanged'`
  - `initCompletionProvider` (line 122) — registers `#region`/`#endregion` snippet completions directly via `languages.registerCompletionItemProvider`
  - `updateFormatterRegistration` (line 150) — lazily registers/disposes `languages.registerDocumentRangeFormattingEditProvider` per language (`css`, `scss`, `less`) based on the `*.format.enable` setting
- **Control flow:**
  1. `startClient` calls `getCustomDataSource` for the initial URI list (line 41).
  2. Builds `clientOptions` (lines 50–97) with document selector `['css','scss','less']`, synchronizes `['css','scss','less']` config sections, and installs the `provideCompletionItem` middleware.
  3. Calls `newLanguageClient(...)` to construct a runtime-specific `BaseLanguageClient` (line 100).
  4. Calls `client.start()` (line 103).
  5. Sends `css/customDataChanged` notification immediately, then re-sends whenever `customDataSource.onDidChange` fires (lines 105–108).
  6. Iterates `formatterRegistrations` (line 111) — calls `updateFormatterRegistration`, subscribes to `onDidChangeConfiguration` for dynamic enable/disable.
  7. Calls `serveFileSystemRequests(client, runtime)` (line 117) to wire `fs/*` request handlers.
  8. Registers snippet completion provider (line 120).
- **Data flow:** `customDataSource.uris` (array of `string`) → `css/customDataChanged` notification → server. `DocumentRangeFormattingRequest` response → `protocol2CodeConverter.asTextEdits` → returned `TextEdit[]`.
- **Dependencies:** `vscode`, `vscode-languageclient`, `./customData`, `./requests`

---

### `extensions/css-language-features/client/src/node/cssClientMain.ts`

- **Role:** Node-runtime entry point (`activate`/`deactivate` extension lifecycle). Resolves server module path, constructs `ServerOptions` for IPC child-process launch, passes `getNodeFSRequestService()` as the `runtime.fs` provider.
- **Key symbols:**
  - `activate` (line 15) — resolves `serverModule` from disk (line 18–19), builds `ServerOptions` with `TransportKind.ipc` (lines 26–29), then invokes `startClient` via closure `newLanguageClient` (lines 31–33)
  - `serverMain` path (line 18) — switches between `dist` and `out` based on whether `clientMain` path contains `/dist/`
  - `debugOptions` (line 22) — injects `--inspect` flag with a random port in range 7000–7999
  - `VSCODE_L10N_BUNDLE_LOCATION` env var (line 36) — passes localization bundle URI to the server process
- **Control flow:** `activate` → resolve server path → create `LanguageClient` with IPC server options → `startClient` → `registerDropOrPasteResourceSupport`. `deactivate` stops the client.
- **Data flow:** Server module path (string) + IPC channel → `LanguageClient` constructor → handed to `startClient` as the `newLanguageClient` constructor.
- **Dependencies:** `vscode`, `vscode-languageclient/node`, `../cssClient`, `./nodeFs`, `../dropOrPaste/dropOrPasteResource`

---

### `extensions/css-language-features/client/src/browser/cssClientMain.ts`

- **Role:** Browser/Worker-runtime entry point. Instead of a Node child process, spawns a `Worker` pointing at `server/dist/browser/cssServerMain.js`, communicates via the Worker's message channel.
- **Key symbols:**
  - `activate` (line 15) — constructs `serverMain` as a `Uri` relative to `context.extensionUri` (line 16), calls `new Worker(serverMain.toString())` (line 18), posts `i10lLocation` message for localization (line 19)
  - `newLanguageClient` closure (lines 21–23) — wraps `new LanguageClient(id, name, worker, clientOptions)` from `vscode-languageclient/browser`
  - No `runtime.fs` provided (line 25) — browser runtime has no native file system access
- **Control flow:** `activate` → create Worker → wrap in `LanguageClient` → `startClient` (no fs) → `registerDropOrPasteResourceSupport`. `deactivate` stops the client.
- **Data flow:** Worker message port replaces IPC pipe; `TextDecoder` is provided from the browser global; `runtime.fs` is absent, so `requests.ts` falls back to `workspace.fs.readFile`.
- **Dependencies:** `vscode`, `vscode-languageclient`, `vscode-languageclient/browser`, `../cssClient`, `../dropOrPaste/dropOrPasteResource`

---

### `extensions/css-language-features/client/src/customData.ts`

- **Role:** Tracks and aggregates custom CSS data provider URIs from workspace settings (`css.customData`) and from all installed extensions' `package.json` `contributes.css.customData` arrays.
- **Key symbols:**
  - `getCustomDataSource` (line 9) — takes a `Disposable[]` to register subscriptions; returns object with `uris` getter and `onDidChange` event
  - `getCustomDataPathsInAllWorkspaces` (line 40) — reads `css.customData` from workspace, workspace-folder, and global scopes; resolves relative paths via `Utils.resolvePath`
  - `getCustomDataPathsFromAllExtensions` (line 77) — iterates `extensions.all` and reads `extension.packageJSON?.contributes?.css?.customData`
- **Control flow:** On `extensions.onDidChange` (line 15) or `workspace.onDidChangeConfiguration` for `css.customData` (line 22), recomputes respective list, fires `onChange` event. The `uris` getter concatenates both lists fresh each access.
- **Data flow:** Returns string[] of absolute URIs; these are sent to the server as `css/customDataChanged` notification payload.
- **Dependencies:** `vscode`, `vscode-uri`

---

### `extensions/css-language-features/client/src/requests.ts`

- **Role:** Bridges the server's filesystem needs back to the VS Code extension host. The server sends `fs/content`, `fs/stat`, `fs/readDir` LSP requests; this file handles them on the client side.
- **Key symbols:**
  - `FsContentRequest.type` (line 11) — `RequestType<{uri, encoding?}, string>`
  - `FsStatRequest.type` (line 13) — `RequestType<string, FileStat>`
  - `FsReadDirRequest.type` (line 17) — `RequestType<string, [string, FileType][]>`
  - `serveFileSystemRequests` (line 21) — called from `cssClient.ts:117`; registers three `client.onRequest` handlers
- **Control flow:** For `file://` URIs with a `runtime.fs` available, delegates to `runtime.fs.getContent/stat/readDirectory`. Otherwise, uses VS Code's `workspace.fs.readFile / stat / readDirectory` APIs (for virtual filesystems).
- **Data flow:** Server sends LSP request → client handler receives it → calls Node fs or `workspace.fs` → returns result back over LSP channel to server.
- **Dependencies:** `vscode`, `vscode-languageclient`, `./cssClient`

---

### `extensions/css-language-features/server/src/cssServer.ts`

- **Role:** Core server logic. Single `startServer(connection, runtime)` function registers all LSP handlers (completion, hover, definition, references, highlights, links, codeActions, colors, rename, folding, selection, formatting, diagnostics). Holds the `LanguageModelCache`, language services map, and settings cache.
- **Key symbols:**
  - `startServer` (line 37) — main entry; wires all handlers onto `connection`
  - `documents: TextDocuments<TextDocument>` (line 40) — document manager from `vscode-languageserver`
  - `stylesheets: LanguageModelCache<Stylesheet>` (line 45) — cache of parsed stylesheets, max 10 entries, 60-second TTL; parse function delegates to `getLanguageService(document).parseStylesheet(document)`
  - `languageServices: {[id: string]: LanguageService}` (line 62) — keyed by `'css'`, `'scss'`, `'less'`; created in `onInitialize` (lines 101–103)
  - `getLanguageService(document)` (line 141) — dispatches to correct `LanguageService` based on `document.languageId`, falls back to `'css'`
  - `validateTextDocument` (line 182) — async; retrieves settings + stylesheet + calls `languageService.doValidation`
  - `updateDataProviders` (line 190) — triggered by `css/customDataChanged`; calls `fetchDataProviders` then `languageServices[lang].setDataProviders(true, customDataProviders)`
  - `onFormat` (line 359) — collapses edits to a single replacement if count exceeds `formatterMaxNumberOfEdits`
- **Control flow:**
  1. `connection.onInitialize` (line 69): sets workspace folders, initializes request service, creates three `LanguageService` instances, registers push or pull diagnostics based on client capability.
  2. `connection.onDidChangeConfiguration` (line 169): reconfigures all language services, clears settings cache, requests diagnostics refresh.
  3. Per-request handlers (lines 198–378): each follows pattern `runSafeAsync(runtime, async () => { document = documents.get(uri); stylesheet = stylesheets.get(document); return languageService.doXxx(...) }, fallback, errorMsg, token)`.
  4. `connection.onNotification(CustomDataChangedNotification.type, updateDataProviders)` (line 380): server reacts to client-sent URI lists.
  5. `connection.listen()` (line 383): starts the connection.
- **Data flow:** LSP request → `documents.get(uri)` → `stylesheets.get(document)` (cached parse) → `languageService.doXxx(document, position, stylesheet, settings)` → LSP response.
- **Dependencies:** `vscode-languageserver`, `vscode-uri`, `vscode-css-languageservice`, `./languageModelCache`, `./utils/runner`, `./utils/validation`, `./utils/documentContext`, `./customData`, `./requests`

---

### `extensions/css-language-features/server/src/node/cssServerMain.ts`

- **Role:** Node-runtime server entry point. Creates an IPC `Connection` via `createConnection()` from `vscode-languageserver/node`, redirects `console.log/error` to the connection console, builds `RuntimeEnvironment` with real `setImmediate`/`setTimeout` and a Node FS request service.
- **Key symbols:**
  - `connection` (line 12) — `createConnection()` using Node's IPC stdin/stdout
  - `runtime: RuntimeEnvironment` (lines 21–33) — `timer.setImmediate` wraps Node's `setImmediate`; `file` is `getNodeFSRequestService()`
- **Control flow:** Module-level setup → `startServer(connection, runtime)` (line 35).
- **Data flow:** Node IPC stdio → `Connection` → `startServer`.
- **Dependencies:** `vscode-languageserver/node`, `../utils/runner`, `../cssServer`, `./nodeFs`

---

### `extensions/css-language-features/server/src/browser/cssServerMain.ts`

- **Role:** Browser/Worker server entry point. Runs inside a Web Worker (`self`). Creates a `Connection` using `BrowserMessageReader(self)` + `BrowserMessageWriter(self)` from `vscode-languageserver/browser`. No `file` FS service (browser has none).
- **Key symbols:**
  - `messageReader/Writer` (lines 9–10) — wrap `self` (the Worker global)
  - `connection` (line 12) — `createConnection(messageReader, messageWriter)`
  - `runtime` (lines 17–28) — `timer.setImmediate` polyfilled as `setTimeout(..., 0)` (no native `setImmediate` in browsers); no `file` property
- **Control flow:** Module-level setup → `startServer(connection, runtime)` (line 30).
- **Data flow:** Worker `postMessage` channel → `BrowserMessageReader/Writer` → `Connection` → `startServer`.
- **Dependencies:** `vscode-languageserver/browser`, `../cssServer`

---

### `extensions/css-language-features/server/src/languageModelCache.ts`

- **Role:** Generic LRU cache for parsed language models (specifically `Stylesheet` objects). Avoids re-parsing unchanged documents.
- **Key symbols:**
  - `LanguageModelCache<T>` interface (lines 8–12) — `get`, `onDocumentRemoved`, `dispose`
  - `getLanguageModelCache<T>(maxEntries, cleanupIntervalTimeInSec, parse)` (line 14) — factory; returns cache object
  - `languageModels` (line 15) — `{[uri: string]: {version, languageId, cTime, languageModel}}` dictionary
  - `cleanupInterval` (line 18) — `setInterval` that evicts entries older than `cleanupIntervalTimeInSec`
- **Control flow:**
  - `get(document)` (line 34): checks version and languageId match; if stale or absent, calls `parse(document)`, stores result; if `nModels === maxEntries`, evicts the entry with the oldest `cTime`.
  - `onDocumentRemoved` (line 66): deletes by URI.
  - `dispose` (line 73): clears interval, empties map.
- **Data flow:** `document` (version + languageId as cache key) → parse callback (e.g. `languageService.parseStylesheet(document)`) → stored `Stylesheet` object returned on subsequent calls without re-parse.
- **Dependencies:** `vscode-css-languageservice` (for `TextDocument` type)

---

### `extensions/css-language-features/server/src/utils/validation.ts`

- **Role:** Provides two diagnostic registration strategies—push (server-driven) and pull (client-driven, LSP 3.17+)—both delegating to a `Validator` callback.
- **Key symbols:**
  - `registerDiagnosticsPushSupport` (line 17) — subscribes to `documents.onDidChangeContent`; debounces 500 ms via `runtime.timer.setTimeout`; calls `connection.sendDiagnostics`
  - `registerDiagnosticsPullSupport` (line 77) — registers `connection.languages.diagnostics.on` handler; returns `DocumentDiagnosticReport`; calls `connection.languages.diagnostics.refresh()` for `requestRefresh`
  - `triggerValidation` (line 44) — debounce logic: cancels existing pending timer per URI, schedules new one; checks request identity before sending to handle racing re-triggers
  - `validationDelayMs = 500` (line 20) — debounce delay constant
- **Control flow (push):** `onDidChangeContent` → `triggerValidation` → `setTimeout(500ms)` → `validate(document)` → `connection.sendDiagnostics`. `onDidClose` → `cleanPendingValidation` + clear diagnostics.
- **Control flow (pull):** Client requests `textDocument/diagnostic` → `connection.languages.diagnostics.on` callback → `validate(document)` → `DocumentDiagnosticReport`.
- **Data flow:** `TextDocument` → `Validator` callback → `Diagnostic[]` → pushed or returned in LSP response.
- **Dependencies:** `vscode-languageserver`, `vscode-css-languageservice`, `./runner`, `../cssServer`

---

### `extensions/css-language-features/server/src/customData.ts`

- **Role:** Fetches and parses custom CSS data JSON files from a list of URIs using the server's `RequestService`. Converts JSON to `ICSSDataProvider` instances understood by `vscode-css-languageservice`.
- **Key symbols:**
  - `fetchDataProviders(dataPaths, requestService)` (line 9) — maps URIs to `requestService.getContent(p)` + `parseCSSData`; returns `Promise<ICSSDataProvider[]>`
  - `parseCSSData(source)` (line 22) — parses JSON; calls `newCSSDataProvider({version, properties, atDirectives, pseudoClasses, pseudoElements})`; returns empty provider on parse failure
- **Control flow:** `cssServer.ts:updateDataProviders` → `fetchDataProviders(dataPaths, requestService)` → `Promise.all(providers)` → resolved array passed to `languageServices[lang].setDataProviders(true, customDataProviders)`.
- **Data flow:** URI string → `requestService.getContent` → raw JSON string → `parseCSSData` → `ICSSDataProvider` → injected into language services.
- **Dependencies:** `vscode-css-languageservice` (`ICSSDataProvider`, `newCSSDataProvider`), `./requests`

---

### `extensions/css-language-features/server/src/node/nodeFs.ts`

- **Role:** Implements `RequestService` using Node.js `fs` module callbacks. Only handles `file://` URIs; throws for any other scheme.
- **Key symbols:**
  - `getNodeFSRequestService()` (line 12) — returns `{getContent, stat, readDirectory}`
  - `getContent` (line 19) — `fs.readFile(uri.fsPath, encoding, ...)` → `buf.toString()`
  - `stat` (line 32) — `fs.stat` → maps to `{type: FileType, ctime, mtime, size}`; returns `{type: Unknown}` for ENOENT
  - `readDirectory` (line 63) — `fs.readdir({withFileTypes: true})` → `[name, FileType][]`
- **Data flow:** `file://` URI → Node `fs` → normalized stat/content/directory response.
- **Dependencies:** Node.js built-in `fs`, `vscode-uri`, `vscode-css-languageservice` (for `FileType` enum), `../requests`

---

### `extensions/css-language-features/server/src/utils/runner.ts`

- **Role:** Provides `runSafeAsync` — a cancellation-aware async wrapper used by every LSP request handler in `cssServer.ts`.
- **Key symbols:**
  - `runSafeAsync<T>(runtime, func, errorVal, errorMessage, token)` (line 21) — schedules `func` via `runtime.timer.setImmediate`; checks `token.isCancellationRequested` before and after; catches errors and resolves to `errorVal`; returns `ResponseError(RequestCancelled)` when cancelled
- **Control flow:** `setImmediate` defers execution to avoid blocking; double cancellation check (before + after async) prevents delivering stale results.
- **Data flow:** `func()` result → resolved promise value OR `ResponseError`.
- **Dependencies:** `vscode-languageserver`, `../cssServer`

---

### `extensions/css-language-features/server/src/utils/documentContext.ts`

- **Role:** Constructs a `DocumentContext` object for `vscode-css-languageservice` so it can resolve relative and absolute `@import` / `url()` references.
- **Key symbols:**
  - `getDocumentContext(documentUri, workspaceFolders)` (line 11) — returns `{resolveReference(ref, base)}`
  - `resolveReference` (line 26) — for absolute refs starting with `/`, finds the matching workspace folder and prepends its URI; for relative refs, uses `Utils.resolvePath(dirname(baseUri), ref)`
- **Data flow:** Absolute or relative path string → resolved full URI string.
- **Dependencies:** `vscode-css-languageservice`, `vscode-uri`, `vscode-languageserver`, `../utils/strings`

---

## Cross-Cutting Synthesis

The CSS language features extension follows the canonical VS Code LSP pattern: a thin client shell delegates all language intelligence to a separate server process (or Worker). The architecture is bifurcated at two symmetric seams — **client-side** and **server-side** — each with a `node/` and `browser/` subdirectory that supplies runtime-specific wiring, while the shared logic lives in the parent directory.

On the **client side**, `cssClient.ts:startClient` is the single shared initialization path. It receives a `LanguageClientConstructor` factory and a `Runtime` interface from the caller. The Node entry (`node/cssClientMain.ts`) passes a constructor that wraps a `LanguageClient` with IPC `ServerOptions` pointing at a child process; the browser entry (`browser/cssClientMain.ts`) passes a constructor that wraps a `LanguageClient` communicating over a `Worker` message channel. The `Runtime.fs` field carries a Node `fs`-backed `RequestService` in the Node path and is absent in the browser path, causing `requests.ts:serveFileSystemRequests` to fall back to `workspace.fs` (VS Code's virtual file system API) for the browser.

On the **server side**, `cssServer.ts:startServer` is similarly runtime-agnostic. It receives a `Connection` and a `RuntimeEnvironment`. The Node entry (`node/cssServerMain.ts`) creates the connection via `vscode-languageserver/node`'s IPC `createConnection()` and supplies real `setImmediate` and `nodeFs`. The browser entry (`browser/cssServerMain.ts`) uses `vscode-languageserver/browser`'s `BrowserMessageReader/Writer` on `self` and polyfills `setImmediate` as `setTimeout(..., 0)`.

The `languageModelCache.ts` provides an LRU cache (max 10 entries, 60-second TTL) for parsed `Stylesheet` objects, keyed by URI + version + languageId. All request handlers in `cssServer.ts` call `stylesheets.get(document)` which re-parses only on document change.

Custom CSS data (user-defined properties, at-rules, pseudo-classes) flows from the VS Code workspace/extension settings → `client/customData.ts` → `css/customDataChanged` LSP notification → `cssServer.ts:updateDataProviders` → `server/customData.ts:fetchDataProviders` → `languageService.setDataProviders(...)`. The server requests file content back through the client via reverse LSP requests (`fs/content`, `fs/stat`, `fs/readDir`) handled in `client/requests.ts`.

Diagnostics are delivered via either push (server sends `textDocument/publishDiagnostics` after a 500 ms debounce) or pull (client requests `textDocument/diagnostic`; server responds inline), with `validation.ts` registering the appropriate mode based on the client capability detected at `onInitialize`.

For a Tauri/Rust port, the entire `vscode-css-languageservice` npm package (parser, validators, completion providers) would need to be replaced with a Rust CSS language server. The dual-runtime transport abstraction (`Connection` interface with pluggable reader/writer, the `Runtime`/`RuntimeEnvironment` interfaces) shows the seams that would need to be re-implemented: in Tauri, the server could run as a Rust sidecar process communicating over stdio rather than IPC or Worker messages. The `fs/` reverse-request mechanism could be replaced by giving the Rust server direct filesystem access, eliminating that round-trip. The client middleware in `cssClient.ts` (completion range transforms, formatter lazy registration, custom data notifications) would need re-implementation in whatever client framework the Tauri port used.

---

## Out-of-Partition References

### `vscode-css-languageservice` (npm package — not in this partition)
- Imported in `server/src/cssServer.ts:10` — provides `getCSSLanguageService`, `getSCSSLanguageService`, `getLESSLanguageService`, `LanguageSettings`, `LanguageService`, `Stylesheet`, `TextDocument`, `Position`, `CodeActionKind`
- Imported in `server/src/languageModelCache.ts:6` — `TextDocument` type
- Imported in `server/src/utils/validation.ts:7` — `TextDocument` type
- Imported in `server/src/customData.ts:6` — `ICSSDataProvider`, `newCSSDataProvider`
- Imported in `server/src/node/nodeFs.ts:10` — `FileType` enum
- Imported in `server/src/utils/documentContext.ts:6` — `DocumentContext`
- This is the core intelligence library; all parse, validate, complete, hover, rename, format, color, fold, and selection-range operations are delegated to it entirely.

### `vscode-languageclient` / `vscode-languageclient/node` / `vscode-languageclient/browser` (npm package)
- `vscode-languageclient` base: `client/src/cssClient.ts:7` — `Disposable`, `LanguageClientOptions`, `ProvideCompletionItemsSignature`, `NotificationType`, `BaseLanguageClient`, `DocumentRangeFormattingParams`, `DocumentRangeFormattingRequest`; also `client/src/requests.ts:7`
- `vscode-languageclient/node`: `client/src/node/cssClientMain.ts:7` — `LanguageClient`, `ServerOptions`, `TransportKind`
- `vscode-languageclient/browser`: `client/src/browser/cssClientMain.ts:9` — `LanguageClient` (Worker-transport variant)

### `vscode-languageserver` / `vscode-languageserver/node` / `vscode-languageserver/browser` (npm package)
- `vscode-languageserver`: `server/src/cssServer.ts:7–8` — `Connection`, `TextDocuments`, `InitializeParams`, `InitializeResult`, `ServerCapabilities`, etc.; also `server/src/utils/validation.ts:6`, `server/src/utils/runner.ts:6`
- `vscode-languageserver/node`: `server/src/node/cssServerMain.ts:6` — `createConnection` (IPC)
- `vscode-languageserver/browser`: `server/src/browser/cssServerMain.ts:6` — `createConnection`, `BrowserMessageReader`, `BrowserMessageWriter`

### `vscode-uri` (npm package)
- `server/src/cssServer.ts:9`, `server/src/node/nodeFs.ts:7`, `server/src/utils/documentContext.ts:9`, `client/src/customData.ts:7`

### `vscode` API (extension host runtime — not a package but VS Code's built-in API)
- `client/src/cssClient.ts:6` — `CompletionItem`, `languages`, `workspace`, `l10n`, etc.
- `client/src/customData.ts:6` — `workspace`, `extensions`, `Uri`, `EventEmitter`
- `client/src/requests.ts:6` — `Uri`, `workspace`
- `client/src/node/cssClientMain.ts:6` — `ExtensionContext`, `extensions`, `l10n`
- `client/src/browser/cssClientMain.ts:6` — `ExtensionContext`, `Uri`, `l10n`
