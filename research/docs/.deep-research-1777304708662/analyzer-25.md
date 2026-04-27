# Partition 25 Analysis: `extensions/json-language-features/`

## Files Analysed

1. `extensions/json-language-features/client/src/jsonClient.ts` (940 lines)
2. `extensions/json-language-features/server/src/jsonServer.ts` (583 lines)
3. `extensions/json-language-features/client/src/node/jsonClientMain.ts` (176 lines)
4. `extensions/json-language-features/client/src/browser/jsonClientMain.ts` (55 lines)
5. `extensions/json-language-features/server/src/node/jsonServerMain.ts` (77 lines)
6. `extensions/json-language-features/server/src/browser/jsonServerMain.ts` (31 lines)
7. `extensions/json-language-features/server/src/browser/jsonServerWorkerMain.ts` (36 lines)
8. `extensions/json-language-features/server/src/languageModelCache.ts` (82 lines)
9. `extensions/json-language-features/client/src/languageParticipants.ts` (91 lines)
10. `extensions/json-language-features/client/src/node/schemaCache.ts` (148 lines)
11. `extensions/json-language-features/client/src/languageStatus.ts` (363 lines)
12. `extensions/json-language-features/server/src/utils/validation.ts` (109 lines)

---

## Per-File Notes (file:line)

### `client/src/jsonClient.ts`

**Role**: Shared client-side orchestration layer consumed by both Node and browser entry points. Sets up the `LanguageClient`, registers all VS Code providers via middleware, and manages schema content fetching.

Key symbols:
- `startClient` (line 182): Entry function called by both Node and browser `activate()`. Instantiates `LanguageParticipants`, then calls `startClientWithParticipants`. Installs a 2-second debounce restart on `languageParticipants.onDidChange` (lines 189–203).
- `startClientWithParticipants` (line 213): Main wiring function. Constructs `LanguageClientOptions` with a `middleware` block (lines 294–384) intercepting completions, hover, folding, colors, and document symbols. Registers commands, code-action providers, format providers, schema-association watchers, and configuration watchers.
- `VSCodeContentRequest` (line 27–29): Custom LSP request `vscode/content` — the server calls back to the client to fetch schema content that cannot be fetched directly.
- `SchemaContentChangeNotification` (line 31–33): Notification `json/schemaContent` sent to server when a tracked schema document changes.
- `SchemaAssociationNotification` (line 81–83): Notification `json/schemaAssociations` sent on startup and when extensions change, carrying file-to-schema URL mappings.
- `ForceValidateRequest` (line 35–37): Request `json/validate` — triggers server to re-validate a single document.
- `ValidateContentRequest` (line 43–45): Request `json/validateContent` — validates arbitrary content against a schema URI.
- `DocumentSortingRequest` (line 61–70): Request `json/sort` — returns `TextEdit[]` for sorting a JSON document.
- `LanguageStatusRequest` (line 39–41): Request `json/languageStatus` — fetches active schema list for status bar.
- `SettingIds` namespace (line 110–123): Centralises configuration key strings (`json.format.enable`, `json.validate.enable`, `json.schemaDownload.enable`, `json.schemaDownload.trustedDomains`, `json.maxItemsComputed`, etc.).
- `CommandIds` namespace (line 125–134): Centralises command ID strings.
- `Runtime` interface (line 146–153): Platform abstraction injected by entry points, providing `schemaRequests`, optional `telemetry`, `timer`, and `logOutputChannel`.
- `SchemaRequestService` interface (line 155–158): `getContent(uri): Promise<string>` and optional `clearCache()`.
- `getSettings` / `computeSettings` (lines 643, 829): Assembles the `Settings` object sent to the server via `DidChangeConfigurationNotification`. Collects `json.schemas` from global, workspace, and per-folder scopes. Builds limit values for result, folding, and color-decorator counts by reading `editor.foldingMaximumRegions` and `editor.colorDecoratorsLimit` per-language.
- `getSchemaAssociations` / `computeSchemaAssociations` (lines 650, 768): Combines `getSchemaExtensionAssociations()` (reads `contributes.jsonValidation` from all extension `package.json`s, lines 773–807) with `getDynamicSchemaAssociations()` (reads `vscode://schemas-associations/schemas-associations.json`, lines 809–825).
- `isTrusted` (line 657): Checks an HTTP schema URI against the `json.schemaDownload.trustedDomains` setting and known associations before permitting download.
- `handleSchemaErrorDiagnostics` (line 272): Intercepted in both push (`handleDiagnostics`) and pull (`provideDiagnostics`) middleware; delegates to `schemaLoadStatusItem.update` and optionally filters schema-resolve errors when download is disabled.
- `updateFormatterRegistration` (line 578): Dynamically registers/unregisters `DocumentRangeFormattingEditProvider` based on `json.format.enable`. The provider manually constructs `DocumentRangeFormattingParams` and forwards the request to the server (lines 591–607).
- `ensureFilesystemWatcherInstalled` (line 471): Creates per-URI `FileSystemWatcher` for `vscode://` schema URIs to push `SchemaContentChangeNotification` on file change.
- `LanguageClientConstructor` type (line 144): `(name, description, clientOptions) => BaseLanguageClient` — allows entry points to supply Node or browser `LanguageClient` instances without this file knowing the transport.

---

### `server/src/jsonServer.ts`

**Role**: Single transport-agnostic LSP server implementation. Receives a `Connection` and `RuntimeEnvironment`, wires all LSP handlers, and delegates all language intelligence to `vscode-json-languageservice`.

Key symbols:
- `startServer(connection, runtime)` (line 89): Top-level function, creates the JSON language service, `TextDocuments` manager, and registers all LSP handlers.
- `getSchemaRequestService` (line 91): Builds a function `(uri: string) => Thenable<string>`. For `file`, `http`, `https` protocols uses `runtime.file` / `runtime.http`; for all others sends `VSCodeContentRequest` back over the connection to the client (lines 107–112).
- `languageService` (line 116): Instance of `getLanguageService` from `vscode-json-languageservice`. Re-created in `onInitialize` with the correct `schemaRequestService` and client capabilities (lines 152–157).
- `onInitialize` (line 146): Negotiates capabilities — `textDocumentSync: Incremental`, `completionProvider` with trigger chars `"` and `:` (if snippets supported), `hoverProvider`, `documentSymbolProvider`, `documentRangeFormattingProvider`, `colorProvider`, `foldingRangeProvider`, `selectionRangeProvider`, `documentLinkProvider`, `diagnosticProvider`, and `codeActionProvider`.
- `diagnosticsSupport` (line 141): Conditionally instantiated as either push (`registerDiagnosticsPushSupport`) or pull (`registerDiagnosticsPullSupport`) based on whether the client advertises `textDocument.diagnostic` capability (lines 177–182).
- `updateConfiguration` (line 360): Merges `schemaAssociations` and `jsonConfigurationSettings` into a `languageSettings` object and calls `languageService.configure(languageSettings)`, then calls `diagnosticsSupport?.requestRefresh()`.
- `validateTextDocument` (line 400): Parses the document via `getJSONDocument`, sets per-language-id severity for comments (`jsonc` → `ignore`; `json` → `error`) and trailing commas, then calls `languageService.doValidation`.
- `jsonDocuments` (line 427): `LanguageModelCache<JSONDocument>` instance with max 10 entries and 60-second eviction. All LSP handlers call `getJSONDocument(document)` which returns cached or newly parsed `JSONDocument`.
- LSP handler registrations (lines 439–570): `onCompletion` → `languageService.doComplete`; `onHover` → `languageService.doHover`; `onDocumentSymbol` → `languageService.findDocumentSymbols2` or `findDocumentSymbols`; `onCodeAction` → returns sort code action; `onDocumentRangeFormatting` / `onDocumentFormatting` → `languageService.format`; `onDocumentColor` → `languageService.findDocumentColors`; `onColorPresentation` → `languageService.getColorPresentations`; `onFoldingRanges` → `languageService.getFoldingRanges`; `onSelectionRanges` → `languageService.getSelectionRanges`; `onDocumentLinks` → `languageService.findLinks`.
- `onFormat` (line 491): Applies `keepLines` option, delegates to `languageService.format`. If edit count exceeds `formatterMaxNumberOfEdits` (negotiated during init as `customCapabilities.rangeFormatting.editLimit`, default 10000), collapses all edits into a single full-range replace (lines 497–501).
- `DocumentSortingRequest` handler (line 350): Calls `languageService.sort(document, options)`.
- `ValidateContentRequest` handler (line 332): Creates a synthetic `TextDocument` at a `vscode://schemas/temp/` URI, temporarily adds an extra schema association, and validates it.
- `workspaceContext` (line 66): Provides `resolveRelativePath` for the language service using `vscode-uri` `Utils.resolvePath`.
- `RuntimeEnvironment` interface (line 77): `file?: RequestService`, `http?: RequestService`, optional `configureHttpRequests`, and `timer` with `setImmediate`/`setTimeout`.

---

### `client/src/node/jsonClientMain.ts`

**Role**: Node/Electron entry point (`activate` / `deactivate`). Spawns the language server as a separate Node.js process via IPC.

Key symbols:
- `activate` (line 20): Reads `package.json` for `aiKey` and server module path. Constructs `ServerOptions` with `TransportKind.ipc` for both run and debug modes (lines 36–39). Debug mode appends `--inspect=<random-port>` (line 32). Passes `VSCODE_L10N_BUNDLE_LOCATION` env var to the server process (line 53). Calls `getSchemaRequestService` and then `startClient`.
- `newLanguageClient` (line 41): Factory using `LanguageClient` from `vscode-languageclient/node`.
- `getSchemaRequestService` (line 86): Constructs a `SchemaRequestService` with ETag-based HTTP caching. Initialises `JSONSchemaCache` in `context.globalStorageUri/json-schema-cache` (lines 91–103). For `schema.management.azure.com` URLs uses time-based cache (lines 161–170). Uses `request-light` `xhr` for actual HTTP fetches with `Accept-Encoding: gzip, deflate` (lines 108–156).
- `retryTimeoutInHours` (line 84): 48 hours — schemas from SchemaStore.org are returned from cache without a network request if last accessed within this window.

---

### `client/src/browser/jsonClientMain.ts`

**Role**: Browser/web entry point. Spawns the language server as a Web Worker.

Key symbols:
- `activate` (line 14): Constructs a `Worker` from `server/dist/browser/jsonServerMain.js`. Posts `{ i10lLocation }` to configure l10n before the worker initialises the LSP server (line 17–18). Uses `LanguageClient` from `vscode-languageclient/browser`.
- `schemaRequests.getContent` (line 25): Uses the browser `fetch` API with `mode: 'cors'`. No caching or ETag support — contrast with the Node entry point.

---

### `server/src/node/jsonServerMain.ts`

**Role**: Node-specific LSP server bootstrap. Creates the IPC `Connection` and supplies Node-specific `RuntimeEnvironment` to `startServer`.

Key symbols:
- `createConnection()` (line 18): From `vscode-languageserver/node` — sets up stdio/IPC connection.
- `getHTTPRequestService` (line 27): Uses `request-light` `xhr`.
- `getFileRequestService` (line 40): Uses `fs.readFile` on `URI.parse(location).fsPath`, with `ENOENT`/`EISDIR` error mapping (lines 45–55).
- `configureHttpRequests` (line 72): Passed through from `requestLight.configure` — sets proxy and strictSSL for all HTTP requests from the server process.
- `console.log` / `console.error` redirected to `connection.console` (lines 20–21).

---

### `server/src/browser/jsonServerMain.ts`

**Role**: Browser-specific LSP server bootstrap. Creates a connection via `BrowserMessageReader`/`BrowserMessageWriter` operating on the Web Worker `self` global.

Key symbols:
- `createConnection(messageReader, messageWriter)` (line 13): From `vscode-languageserver/browser`.
- `RuntimeEnvironment` (line 18): `timer` only — no `file` or `http` services, meaning all schema resolution for non-`file` URIs falls back to `VSCodeContentRequest` over the worker message channel back to the client.
- `setImmediate` polyfill (line 21): Emulated as `setTimeout(callback, 0)` since Web Workers lack `setImmediate`.

---

### `server/src/browser/jsonServerWorkerMain.ts`

**Role**: Initial `onmessage` handler for the Web Worker. Defers loading `jsonServerMain.ts` until the first message (carrying `i10lLocation`) is received, ensuring l10n is configured before any LSP traffic arrives.

Key symbols:
- `messageHandler` (line 10): Sets `initialized = true` on first message, calls `l10n.config({ uri: i10lLocation })`, then dynamically imports `./jsonServerMain.js` (line 25). Buffers any messages that arrive before init completes in `pendingMessages` (line 33).

---

### `server/src/languageModelCache.ts`

**Role**: LRU-style cache for parsed `JSONDocument` objects keyed by document URI and version.

Key symbols:
- `getLanguageModelCache<T>(maxEntries, cleanupIntervalTimeInSec, parse)` (line 14): Returns a `LanguageModelCache<T>` object.
- `get(document)` (line 34): Returns cached model if `version` and `languageId` both match. Otherwise calls `parse(document)`, stores result with `cTime = Date.now()`. If `nModels > maxEntries`, evicts the entry with the oldest `cTime` (lines 48–61).
- `setInterval` (line 20): Runs every `cleanupIntervalTimeInSec` seconds, deleting entries whose `cTime` is older than the interval — prevents unbounded growth across sessions.
- `onDocumentRemoved` (line 66): Explicit eviction on `documents.onDidClose`.
- In `jsonServer.ts` instantiated as `getLanguageModelCache(10, 60, ...)` — max 10 entries, 60-second GC interval.

---

### `client/src/languageParticipants.ts`

**Role**: Tracks which language IDs participate in the JSON language server (e.g., `json`, `jsonc`, `snippets`, plus contributions from other extensions via `jsonLanguageParticipants` in their `package.json`).

Key symbols:
- `getLanguageParticipants()` (line 31): Returns a `LanguageParticipants` object.
- `update()` (line 36): Rebuilds `languages` and `comments` Sets. Always includes `json`, `jsonc`, `snippets` (lines 40–45). Reads `extension.packageJSON.contributes.jsonLanguageParticipants` from all extensions across all hosts (lines 47–59). Returns `true` if any set changed.
- `extensions.onDidChange` (line 65): Calls `update()` and fires `onDidChangeEmmiter` if changed, which triggers a 2-second debounced restart of the language client in `jsonClient.ts`.
- `documentSelector` getter (line 73): Returns `Array.from(languages)` — the full list used for all provider registrations.

---

### `client/src/node/schemaCache.ts`

**Role**: Disk + Memento-backed ETag cache for HTTP JSON schemas (Node only).

Key symbols:
- `JSONSchemaCache` class (line 23): Constructor reads `MEMENTO_KEY` from `globalState`, validates structure, stores as `cacheInfo`.
- `putSchema(uri, etag, content)` (line 50): Writes content to `SHA-256(uri).schema.json` in `schemaCacheLocation` (line 53), updates `cacheInfo`, persists via `updateMemento`.
- `getSchemaIfUpdatedSince(uri, expirationDurationInHours)` (line 63): Returns cached content only if `lastUpdatedInHours < expirationDurationInHours` — used for SchemaStore.org schemas with 48-hour TTL.
- `getSchema(uri, etag, etagValid)` (line 71): Validates ETag match before reading file. If ETag differs, calls `deleteSchemaFile`.
- `clearCache()` (line 124): Deletes all files from `schemaCacheLocation`, clears `cacheInfo`, updates Memento. Returns list of evicted URIs (sent back to server via `SchemaContentChangeNotification`).
- `getCacheFileName(uri)` (line 145): `sha256(uri).hex + '.schema.json'`.

---

### `client/src/languageStatus.ts`

**Role**: Creates and manages language status bar items for schema validation status, document-symbol limit warnings, and schema download issues.

Key symbols:
- `createLanguageStatusItem(documentSelector, statusRequest)` (line 166): Calls `statusRequest(uri)` (which sends `LanguageStatusRequest` to server) on every active editor change. Displays `No schema validation`, `Schema validated`, or `multiple JSON schemas configured` (lines 187–196). Shows a `Show Schemas` command that opens `showSchemaList` quick-pick.
- `showSchemaList(input)` (line 112): Classifies each schema URI into extension-contributed, settings-configured, or plain URL, using `getExtensionSchemaAssociations()` and `getSettingsSchemaAssociations()`. Displays a `QuickPick` with navigation buttons to open schema files or settings.
- `createLimitStatusItem(newItem)` (line 219): Generic factory for limit-warning status items. Tracks per-document `activeLimits` map; creates/destroys status item as active editor changes.
- `createSchemaLoadStatusItem(newItem)` (line 282): Similarly tracks per-document `fileSchemaErrors`. Shows `Schema download issue` with contextual actions (workspace trust, download enable, trusted domains, retry) based on the error code.
- `createSchemaLoadIssueItem` (line 338): Creates the actual language status item with severity `Error`; command depends on whether trust is missing, download is disabled, schema location is untrusted, or a generic resolution failure.

---

### `server/src/utils/validation.ts`

**Role**: Provides two diagnostics support strategies — push (server-initiated, debounced) and pull (client-requested).

Key symbols:
- `registerDiagnosticsPushSupport` (line 17): Listens on `documents.onDidChangeContent`, debounces validation with a 500ms timer (`validationDelayMs`, line 22). Sends results via `connection.sendDiagnostics`. On document close, clears pending request and sends empty diagnostics.
- `registerDiagnosticsPullSupport` (line 77): Registers `connection.languages.diagnostics.on` handler. `requestRefresh()` calls `connection.languages.diagnostics.refresh()` to ask the client to re-pull.
- `Validator` type (line 11): `(textDocument: TextDocument) => Promise<Diagnostic[]>` — both modes call back into `validateTextDocument` in `jsonServer.ts`.
- `DiagnosticsSupport` type (line 12): `{ dispose(): void; requestRefresh(): void }` — uniform interface allowing `jsonServer.ts` to call `diagnosticsSupport?.requestRefresh()` regardless of mode.

---

## Cross-Cutting Synthesis

The JSON language features extension implements a clean client/server split over the Language Server Protocol. The `jsonServer.ts` is a single transport-agnostic implementation: it receives a `Connection` (IPC for Node, `BrowserMessageReader`/`BrowserMessageWriter` for Web Worker) and a `RuntimeEnvironment` that abstracts file I/O and HTTP. All language intelligence is delegated entirely to the external `vscode-json-languageservice` npm package — the server itself contains zero JSON-parsing or schema-validation logic.

The client side (`jsonClient.ts`) is likewise shared between Node and browser entries, differentiated only by the injected `LanguageClientConstructor` and `Runtime`. The critical architectural seam is schema content fetching: `file://` URIs are handled server-side by `RuntimeEnvironment.file`; `http://https://` by `RuntimeEnvironment.http` (with Node adding ETag disk caching via `JSONSchemaCache`); all other URI schemes (including `vscode://`) are forwarded via a `vscode/content` request back to the client, which can use the full VS Code workspace API. This back-channel is a key dependency on VS Code's extension host IPC. Dynamic language participation (non-`json`/`jsonc` files joining the server) is handled by `languageParticipants.ts`, which scans all extension `package.json` manifests for `jsonLanguageParticipants` contributions and triggers a debounced full client restart when the set changes.

For a Tauri/Rust port, the entire language-server subprocess is reusable as-is (it has no Electron or VS Code API dependencies), but all client-side components directly depend on the VS Code extension API: `workspace`, `window`, `languages`, `commands`, `extensions`, `Uri`, `StatusBarItem`, `LanguageClient`, etc. These would each require purpose-built Tauri equivalents.

---

## Out-of-Partition References

- `vscode-json-languageservice` (npm): Provides `getLanguageService`, `JSONDocument`, `JSONSchema`, `doValidation`, `doComplete`, `doHover`, `format`, `sort`, `getFoldingRanges`, `findDocumentColors`, `getColorPresentations`, `findDocumentSymbols2`, `findLinks`, `getSelectionRanges`, `getLanguageStatus`. All language intelligence lives here, outside this partition.
- `vscode-languageclient` / `vscode-languageclient/node` / `vscode-languageclient/browser` (npm): `BaseLanguageClient`, `LanguageClient`, transport layer. Referenced in `jsonClient.ts:18`, `jsonClientMain.ts:8`, `browser/jsonClientMain.ts:9`.
- `vscode-languageserver` / `vscode-languageserver/node` / `vscode-languageserver/browser` (npm): `Connection`, `createConnection`, server-side LSP infrastructure. Referenced in `jsonServer.ts:10`, `node/jsonServerMain.ts:6`, `browser/jsonServerMain.ts:6`.
- `request-light` (npm): HTTP client used in both `node/jsonClientMain.ts:12` and `node/jsonServerMain.ts:10`.
- `@vscode/extension-telemetry` (npm): `TelemetryReporter` used in `node/jsonClientMain.ts:14`.
- `vscode-uri` (npm): `URI`, `Utils` used in `jsonServer.ts:16`.
- `jsonc-parser` (npm): Implicitly consumed inside `vscode-json-languageservice`; not directly imported in this partition.
- `src/vs/workbench/` and core VS Code extension host: The `vscode` module API (`workspace`, `window`, `languages`, `commands`, `extensions`, `Uri`, `Memento`, etc.) are all provided by the VS Code extension host — the fundamental host dependency this extension is built on.
