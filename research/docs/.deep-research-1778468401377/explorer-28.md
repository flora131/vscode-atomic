# Partition 28 of 80 — Findings

## Scope
`extensions/css-language-features/` (30 files, 2,261 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# CSS Language Features Extension — Codebase Locator

**Extension**: `extensions/css-language-features/` — Language server for CSS, SCSS, and LESS  
**Scope**: 44 files, 2,261 LOC (implementation + tests)  
**Architecture**: Client-Server LSP pattern with Node/Browser dual-runtime support

---

## Implementation — Client

The client is a VS Code extension that activates on CSS/SCSS/LESS files and manages the LanguageClient.

- `client/src/node/cssClientMain.ts` — Node entry point; spawns server subprocess via IPC, registers drop/paste handlers
- `client/src/browser/cssClientMain.ts` — Web/browser entry point; launches server in a Web Worker
- `client/src/cssClient.ts` — Shared client initialization; wires up LanguageClient with middleware for completion item adjustments, custom data tracking, and formatter registration
- `client/src/customData.ts` — Watches `css.customData` settings and extension contributions; emits change notifications to server
- `client/src/requests.ts` — File system request handlers (FsContentRequest, FsStatRequest, FsReadDirRequest); bridges client-side file access with server
- `client/src/node/nodeFs.ts` — Node.js filesystem implementation for file requests
- `client/src/dropOrPaste/dropOrPasteResource.ts` — Registers DocumentDropEditProvider and DocumentPasteEditProvider for CSS; converts URI lists to `url()` snippets
- `client/src/dropOrPaste/shared.ts` — Utility to extract document directory URI
- `client/src/dropOrPaste/uriList.ts` — Parses and formats URI list MIME type

---

## Implementation — Server

The server implements LSP handlers for CSS/SCSS/LESS. Uses `vscode-css-languageservice` for language smarts.

- `server/src/node/cssServerMain.ts` — Node entry point; creates LSP connection, injects Node.js filesystem runtime, starts server
- `server/src/node/cssServerNodeMain.ts` — Alternative Node main (compiled entry point for esbuild)
- `server/src/browser/cssServerMain.ts` — Browser/Worker entry point; uses BrowserMessageReader/Writer for postMessage comm
- `server/src/browser/cssServerWorkerMain.ts` — Worker initialization (if needed)
- `server/src/cssServer.ts` — Core server logic; initializes connection, manages language services, registers diagnostic/document/code action/completion handlers, handles config changes
- `server/src/languageModelCache.ts` — LRU cache for parsed stylesheets (10 entries, 60 sec eviction)
- `server/src/requests.ts` — File system request types and multiplexer; routes fs calls to built-in handlers or relays via LSP
- `server/src/customData.ts` — Fetches and caches CSS data providers from custom data URIs (extends CSS properties/at-rules)
- `server/src/node/nodeFs.ts` — Node.js fs module wrapper; implements RequestService interface for file I/O

---

## Utilities

Core support libraries used by both client and server.

- `server/src/utils/validation.ts` — Registers push-based (on-change) and pull-based (on-demand) diagnostic providers with 500ms debounce
- `server/src/utils/documentContext.ts` — Resolves relative CSS imports against workspace folders; wraps vscode-uri
- `server/src/utils/runner.ts` — Async error handling wrapper (runSafeAsync); formats errors for LSP logging
- `server/src/utils/strings.ts` — String utilities (startsWith, endsWith) with optional case-sensitivity

---

## Tests

Test files using Node's native test runner.

- `server/src/test/completion.test.ts` — Tests CSS property/value completion, selector completion, URL path completion
- `server/src/test/links.test.ts` — Tests document link detection (e.g., `@import` URLs, custom property links)
- `server/test/index.js` — Test harness; discovers `.test.js` files in `out/test/`, runs via `node:test`, outputs JUnit XML for CI

Test fixtures:
- `server/test/pathCompletionFixtures/` — Sample HTML/CSS/SCSS files for import/path completion tests
- `server/test/linksTestFixtures/` — Package.json fixture for relative path resolution tests

---

## Configuration

Extension manifest and build config.

- `package.json` — Root extension manifest; declares CSS/SCSS/LESS language activation, configuration sections (40+ lint/format options per language), entry points (node + browser)
- `server/package.json` — Server package; exports main (Node) and browser entry points; dependencies on vscode-languageservice, vscode-languageserver, vscode-uri
- `.npmrc` — Likely sets npm registry or cache behavior
- `.vscodeignore` — Excludes build artifacts, node_modules from vsix package
- `schemas/package.schema.json` — JSON schema for validating css-data vendor extensions

Build:
- `esbuild.mts` — Esbuild config for Node.js build; bundles client and server ESM + CJS compat
- `esbuild.browser.mts` — Browser build config (web worker bundle)
- `client/tsconfig.json` — Client TypeScript config
- `client/tsconfig.browser.json` — Browser-specific TypeScript config (web lib targets)
- `server/tsconfig.json` — Server TypeScript config
- `server/tsconfig.browser.json` — Server browser TypeScript config

---

## Configuration & Documentation

- `package.nls.json` — I18n strings for display names and descriptions
- `.vscode/settings.json` — Extension development settings
- `.vscode/launch.json` — Debug configurations ("Launch Extension", "Attach to Node Process")
- `.vscode/tasks.json` — Build tasks
- `README.md` — Brief overview; directs users to main VS Code docs
- `CONTRIBUTING.md` — Setup instructions (compile, watch, debug), workflow for linking vscode-css-languageservice for local development
- `icons/css.png` — Extension icon

---

## Notable Clusters

### Client Distribution
- `client/src/node/` — 2 files; Desktop/Node entry point
- `client/src/browser/` — 2 files; Web/Worker entry point
- `client/src/dropOrPaste/` — 3 files; Drop/paste URI handling (new feature)

### Server Distribution
- `server/src/node/` — 2 files; Node file I/O, IPC connection setup
- `server/src/browser/` — 2 files; Worker postMessage, Worker main
- `server/src/utils/` — 4 files; Validation, document context, error handling, string utilities

### Test Infrastructure
- `server/test/` — 1 test harness file, 2 fixture directories
- `server/src/test/` — 2 test files (completion, links)

---

## Architecture Summary

**Dual-Runtime Design**: The extension supports both Node.js (desktop) and browser (web) contexts. Separate entry points (`node/` vs `browser/`) instantiate language clients and servers using the appropriate transport layer (IPC vs postMessage).

**LSP Client-Server**: Uses `vscode-languageclient` (client) and `vscode-languageserver` (server). Shared request/notification types bridge file system operations and custom data updates.

**Lazy Language Service**: The server parses CSS on-demand, caching parsed stylesheets. Language operations (completion, diagnostics, folding) delegate to `vscode-css-languageservice`.

**Custom CSS Data**: Extensible via `css.customData` setting and extension contributions. Allows third-party CSS property definitions (e.g., vendor prefixes, design tokens).

**Drop/Paste Integration**: New feature (2024+) allowing users to drop files into CSS editors and have them converted to `url()` references.

**Multi-Language Support**: Unified server handles CSS, SCSS, and LESS with language-specific settings and diagnostics rules.

---

## Key Dependencies

- **vscode-languageclient** (^10.0.0-next.20) — Client-side LSP integration
- **vscode-languageserver** (^10.0.0-next.16) — Server-side LSP protocol
- **vscode-css-languageservice** (^7.0.0-next.1) — Language intelligence (parsing, validation, completion)
- **vscode-uri** (^3.1.0) — URI parsing and manipulation
- **@vscode/l10n** (^0.0.18) — Localization bundle support

---

## Porting Considerations

For a Tauri/Rust port of this CSS language server:

1. **Language Service**: Replace `vscode-css-languageservice` with an equivalent Rust CSS parser (e.g., `cssparser` crate or `swc_css_parser`)
2. **LSP Bridge**: Implement LSP server in Rust; use `lsp-server` or `tower-lsp` crate
3. **IPC/Worker**: Use Tauri's command system (or stdio pipes) instead of Node IPC; spawn Rust server as subprocess
4. **Custom Data**: Keep custom data as JSON; load via URI resolution (same pattern)
5. **Async Runtime**: Use Tokio for async I/O; map Node promises to Rust Futures
6. **File System**: Leverage Tauri's fs scope API for secure file access
7. **Drop/Paste**: Implement via Tauri clipboard APIs and editor commands
8. **Testing**: Use Rust test harness (cargo test) instead of Node test runner
9. **Configuration**: Parse VSCode settings JSON; store diagnostics rules as structured config

The dual-runtime pattern (Node/browser) is less relevant in Tauri; focus on a single Rust backend with multiple IPC transports if needed (named pipes for stdio, or Tauri commands).

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# LSP-Client Patterns in CSS Language Features Extension

## Research Overview

This analysis identifies the concrete LSP-client/language-feature patterns in VS Code's CSS language features extension (`extensions/css-language-features/`, 2,261 LOC) that any Rust/Tauri port must replicate to support core IDE functionality.

The architecture separates into **client** (UI/editor-facing) and **server** (language logic) with LSP communication over IPC (Node.js) or Web Workers (browser).

---

## Pattern 1: LanguageClient Initialization

**Where:** `extensions/css-language-features/client/src/node/cssClientMain.ts:26-32`

**What:** Establishes the LSP client over Node.js IPC transport with debug options.

```typescript
const serverOptions: ServerOptions = {
	run: { module: serverModule, transport: TransportKind.ipc },
	debug: { module: serverModule, transport: TransportKind.ipc, options: debugOptions }
};

const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
	return new LanguageClient(id, name, serverOptions, clientOptions);
};
```

**Variations:**

- Browser variant (`extensions/css-language-features/client/src/browser/cssClientMain.ts:17-23`): Uses `Worker` transport instead of IPC.
- Abstract constructor passed to `startClient()` to decouple platform transport from language-agnostic setup logic.

---

## Pattern 2: Language Client Document Selection & Middleware

**Where:** `extensions/css-language-features/client/src/cssClient.ts:39-97`

**What:** Configures LSP synchronization, initialization options, and middleware to adapt LSP responses for VS Code UI (e.g., completion item label formatting, range handling).

```typescript
const documentSelector = ['css', 'scss', 'less'];
const clientOptions: LanguageClientOptions = {
	documentSelector,
	synchronize: {
		configurationSection: ['css', 'scss', 'less']
	},
	initializationOptions: {
		handledSchemas: ['file'],
		provideFormatter: false,
		customCapabilities: { rangeFormatting: { editLimit: 10000 } }
	},
	middleware: {
		provideCompletionItem(document, position, context, token, next) {
			// Adapt range format: split into insert/replace
			// Adapt color labels with descriptions
			const r = next(document, position, context, token);
			return isThenable(r) ? r.then(updateProposals) : updateProposals(r);
		}
	}
};
```

**Key aspects:**
- `documentSelector` narrows activation to CSS-like languages.
- `synchronize.configurationSection` triggers server updates on config changes.
- `initializationOptions` passes server capabilities and constraints.
- `middleware` transforms LSP responses for UI compatibility.

---

## Pattern 3: Server-Side Connection Lifecycle & Capability Registration

**Where:** `extensions/css-language-features/server/src/cssServer.ts:69-139`

**What:** Server initializes on `connection.onInitialize()`, interrogates client capabilities, configures language services, and declares LSP capabilities.

```typescript
connection.onInitialize((params: InitializeParams): InitializeResult => {
	const initializationOptions = params.initializationOptions || {};
	workspaceFolders = params.workspaceFolders || [];
	requestService = getRequestService(initializationOptions?.handledSchemas || ['file'], connection, runtime);

	// Detect client capabilities
	const snippetSupport = !!getClientCapability('textDocument.completion.completionItem.snippetSupport', false);
	scopedSettingsSupport = !!getClientCapability('workspace.configuration', false);

	// Initialize language services
	languageServices.css = getCSSLanguageService({ fileSystemProvider: requestService, clientCapabilities: params.capabilities });
	languageServices.scss = getSCSSLanguageService({ ... });
	languageServices.less = getLESSLanguageService({ ... });

	// Register diagnostics support (push or pull)
	const supportsDiagnosticPull = getClientCapability('textDocument.diagnostic', undefined);
	diagnosticsSupport = supportsDiagnosticPull === undefined 
		? registerDiagnosticsPushSupport(documents, connection, runtime, validateTextDocument)
		: registerDiagnosticsPullSupport(documents, connection, runtime, validateTextDocument);

	const capabilities: ServerCapabilities = {
		textDocumentSync: TextDocumentSyncKind.Incremental,
		completionProvider: snippetSupport ? { resolveProvider: false, triggerCharacters: ['/', '-', ':'] } : undefined,
		hoverProvider: true,
		documentSymbolProvider: true,
		// ... 12 more capabilities
	};
	return { capabilities };
});
```

**Key aspects:**
- Negotiates client capabilities at handshake.
- Conditionally enables features (snippets, diagnostics) based on client support.
- Language services instantiated once with runtime config.

---

## Pattern 4: LSP Request Handlers (on* Patterns)

**Where:** `extensions/css-language-features/server/src/cssServer.ts:198-357`

**What:** Register handlers for LSP requests/notifications with cancellation-aware async error wrapping.

```typescript
connection.onCompletion((textDocumentPosition, token) => {
	return runSafeAsync(runtime, async () => {
		const document = documents.get(textDocumentPosition.textDocument.uri);
		if (document) {
			const [settings,] = await Promise.all([getDocumentSettings(document), dataProvidersReady]);
			const styleSheet = stylesheets.get(document);
			const documentContext = getDocumentContext(document.uri, workspaceFolders);
			return getLanguageService(document).doComplete2(document, textDocumentPosition.position, styleSheet, documentContext, settings?.completion);
		}
		return null;
	}, null, `Error while computing completions for ${textDocumentPosition.textDocument.uri}`, token);
});

// Similar handlers for:
// - onHover, onDocumentSymbol, onDefinition, onDocumentHighlight, onDocumentLinks
// - onReferences, onCodeAction, onDocumentColor, onColorPresentation
// - onRenameRequest, onFoldingRanges, onSelectionRanges
// - onDocumentRangeFormatting, onDocumentFormatting
```

**Key aspects:**
- All handlers wrapped in `runSafeAsync()` for error/cancellation handling.
- Document retrieval via `TextDocuments` manager.
- Compose settings + cached stylesheets to avoid re-parsing.
- Language service method dispatches to vscode-css-languageservice.

**Handlers present:** 16 LSP request/notification handlers.

---

## Pattern 5: File System Request Service (Client ↔ Server Bridge)

**Where:** `extensions/css-language-features/client/src/requests.ts:21-45`

**What:** Client-side listener for server-initiated file system requests (fs/content, fs/stat, fs/readDir).

```typescript
export function serveFileSystemRequests(client: BaseLanguageClient, runtime: Runtime) {
	client.onRequest(FsContentRequest.type, (param: { uri: string; encoding?: string }) => {
		const uri = Uri.parse(param.uri);
		if (uri.scheme === 'file' && runtime.fs) {
			return runtime.fs.getContent(param.uri);  // Use native FS if available
		}
		return workspace.fs.readFile(uri).then(buffer => {
			return new runtime.TextDecoder(param.encoding).decode(buffer);
		});
	});
	client.onRequest(FsReadDirRequest.type, (uriString: string) => { ... });
	client.onRequest(FsStatRequest.type, (uriString: string) => { ... });
}
```

**Server-side counterpart:** `extensions/css-language-features/server/src/requests.ts:66-99`

```typescript
export function getRequestService(handledSchemas: string[], connection: Connection, runtime: RuntimeEnvironment): RequestService {
	const builtInHandlers: { [protocol: string]: RequestService | undefined } = {};
	for (const protocol of handledSchemas) {
		if (protocol === 'file') {
			builtInHandlers[protocol] = runtime.file;  // Direct file I/O if trusted
		} else if (protocol === 'http' || protocol === 'https') {
			builtInHandlers[protocol] = runtime.http;
		}
	}
	return {
		async stat(uri: string): Promise<FileStat> {
			const handler = builtInHandlers[getScheme(uri)];
			if (handler) return handler.stat(uri);
			return connection.sendRequest(FsStatRequest.type, uri.toString());  // Fallback to client
		},
		// Similar for readDirectory, getContent
	};
}
```

**Key aspects:**
- Named RequestTypes (`FsContentRequest`, `FsStatRequest`, `FsReadDirRequest`) define RPC contract.
- Server tries local handlers first, falls back to client.
- Abstracts file I/O to decouple language logic from platform FS.

---

## Pattern 6: Error Handling & Cancellation-Aware Async Wrapper

**Where:** `extensions/css-language-features/server/src/utils/runner.ts:21-45`

**What:** Wraps all LSP handler execution with cancellation checking and error formatting.

```typescript
export function runSafeAsync<T>(
	runtime: RuntimeEnvironment,
	func: () => Thenable<T>,
	errorVal: T,
	errorMessage: string,
	token: CancellationToken
): Thenable<T | ResponseError<any>> {
	return new Promise<T | ResponseError<any>>((resolve) => {
		runtime.timer.setImmediate(() => {
			if (token.isCancellationRequested) {
				resolve(cancelValue());
				return;
			}
			return func().then(result => {
				if (token.isCancellationRequested) {
					resolve(cancelValue());
					return;
				} else {
					resolve(result);
				}
			}, e => {
				console.error(formatError(errorMessage, e));
				resolve(errorVal);  // Return default on error
			});
		});
	});
}

function cancelValue<E>() {
	return new ResponseError<E>(LSPErrorCodes.RequestCancelled, 'Request cancelled');
}
```

**Key aspects:**
- Defers execution to next tick via `runtime.timer.setImmediate`.
- Checks cancellation **before** and **after** async work.
- Logs exceptions, returns neutral fallback value on error.
- Returns LSP ResponseError on cancellation.

---

## Pattern 7: Custom Notification (One-Way Server Push)

**Where:** `extensions/css-language-features/server/src/cssServer.ts:18-20` and `client/src/cssClient.ts:11-13`

**What:** Defines and sends custom notifications (css/customDataChanged) for out-of-band server state updates.

```typescript
// Server-side definition & send
namespace CustomDataChangedNotification {
	export const type: NotificationType<string[]> = new NotificationType('css/customDataChanged');
}

connection.onNotification(CustomDataChangedNotification.type, updateDataProviders);

// Client-side definition & send
client.sendNotification(CustomDataChangedNotification.type, customDataSource.uris);
customDataSource.onDidChange(() => {
	client.sendNotification(CustomDataChangedNotification.type, customDataSource.uris);
});
```

**Key aspects:**
- Custom namespace isolates protocol definition.
- `NotificationType<T>` parameterizes payload type.
- Client pushes new custom data paths on configuration/extension changes.
- Server updates language service data providers reactively.

---

## Pattern 8: Diagnostic Push vs. Pull (Capability Negotiation)

**Where:** `extensions/css-language-features/server/src/utils/validation.ts:17-100`

**What:** Supports both push diagnostics (server initiates) and pull diagnostics (client requests), selected during init based on capability.

**Push variant** (lines 17-75):
```typescript
export function registerDiagnosticsPushSupport(documents: TextDocuments<TextDocument>, connection: Connection, runtime: RuntimeEnvironment, validate: Validator): DiagnosticsSupport {
	const pendingValidationRequests: { [uri: string]: Disposable } = {};
	const validationDelayMs = 500;

	documents.onDidChangeContent(change => {
		triggerValidation(change.document);
	});
	documents.onDidClose(event => {
		connection.sendDiagnostics({ uri: event.document.uri, diagnostics: [] });
	});

	function triggerValidation(textDocument: TextDocument): void {
		cleanPendingValidation(textDocument);
		const request = runtime.timer.setTimeout(async () => {
			const diagnostics = await validate(textDocument);
			connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
		}, validationDelayMs);
	}
	// ...
}
```

**Pull variant** (lines 77-95):
```typescript
export function registerDiagnosticsPullSupport(documents: TextDocuments<TextDocument>, connection: Connection, runtime: RuntimeEnvironment, validate: Validator): DiagnosticsSupport {
	const registration = connection.languages.diagnostics.on(async (params: DocumentDiagnosticParams, token: CancellationToken) => {
		return runSafeAsync(runtime, async () => {
			const document = documents.get(params.textDocument.uri);
			if (document) {
				return newDocumentDiagnosticReport(await validate(document));
			}
			return newDocumentDiagnosticReport([]);
		}, newDocumentDiagnosticReport([]), `Error while computing diagnostics...`, token);
	});

	function requestRefresh(): void {
		connection.languages.diagnostics.refresh();
	}
	// ...
}
```

**Key aspects:**
- Single validator function, dual transport.
- Push: debounces on content change, server-initiated send.
- Pull: handler registered with `connection.languages.diagnostics.on()`, client-initiated fetch.
- Same `DiagnosticsSupport` interface for both.

---

## Pattern 9: Middleware Provider for Client-Side Completion Adaptation

**Where:** `extensions/css-language-features/client/src/cssClient.ts:60-96`

**What:** Intercepts LSP completion responses to adapt them for VS Code UI (range formats, label descriptions).

```typescript
middleware: {
	provideCompletionItem(document: TextDocument, position: Position, context: CompletionContext, token: CancellationToken, next: ProvideCompletionItemsSignature): ProviderResult<CompletionItem[] | CompletionList> {
		function updateRanges(item: CompletionItem) {
			const range = item.range;
			if (range instanceof Range && range.end.isAfter(position) && range.start.isBeforeOrEqual(position)) {
				// Split into insert/replace ranges (VS Code 1.67+)
				item.range = { inserting: new Range(range.start, position), replacing: range };
			}
		}
		function updateLabel(item: CompletionItem) {
			// Add description to color completions
			if (item.kind === CompletionItemKind.Color) {
				item.label = {
					label: item.label as string,
					description: (item.documentation as string)
				};
			}
		}
		// Chain to next provider, transform result
		const r = next(document, position, context, token);
		return isThenable(r) ? r.then(updateProposals) : updateProposals(r);
	}
}
```

**Key aspects:**
- Middleware intercepts before returning to editor.
- Handles async promise chains with `isThenable` check.
- Adapts LSP ranges and labels per VS Code conventions.

---

## Pattern 10: Drop/Paste Resource Handler (UI Feature Registration)

**Where:** `extensions/css-language-features/client/src/dropOrPaste/dropOrPasteResource.ts:11-153`

**What:** Registers document drop/paste edit providers for CSS URL insertion, handling file URI conversion.

```typescript
class DropOrPasteResourceProvider implements vscode.DocumentDropEditProvider, vscode.DocumentPasteEditProvider {
	readonly kind = vscode.DocumentDropOrPasteEditKind.Empty.append('css', 'link', 'url');

	async provideDocumentDropEdits(document, position, dataTransfer, token) {
		const uriList = await this.getUriList(dataTransfer);
		if (!uriList.entries.length || token.isCancellationRequested) return;

		const snippet = await this.createUriListSnippet(document.uri, uriList);
		return {
			kind: this.kind,
			title: snippet.label,
			insertText: snippet.snippet.value,
			yieldTo: this.pasteAsCssUrlByDefault(document, position) ? [] : [...]
		};
	}

	private async createUriListSnippet(docUri, uriList) {
		const snippet = new vscode.SnippetString();
		for (const uri of uriList.entries) {
			const relativePath = getRelativePath(getDocumentDir(docUri), uri.uri);
			snippet.appendText(`url(${relativePath ?? uri.str})`);
		}
		return { snippet, label: '...' };
	}
}

export function registerDropOrPasteResourceSupport(selector) {
	return vscode.Disposable.from(
		vscode.languages.registerDocumentDropEditProvider(selector, provider, { ... }),
		vscode.languages.registerDocumentPasteEditProvider(selector, provider, { ... })
	);
}
```

**Key aspects:**
- Leverages native VS Code Drop/Paste API (1.76+).
- Converts file URIs to relative paths for insertion.
- Registered per language selector.

---

## Pattern 11: Configuration Section Synchronization

**Where:** `extensions/css-language-features/client/src/cssClient.ts:50-54` and `server/src/cssServer.ts:155-166`

**What:** Client and server synchronize settings via `configurationSection` and explicit `ConfigurationRequest`.

```typescript
// Client-side sync declaration
const clientOptions: LanguageClientOptions = {
	synchronize: {
		configurationSection: ['css', 'scss', 'less']
	}
};

// Server-side fetch (scoped to document if client supports it)
function getDocumentSettings(textDocument: TextDocument): Thenable<LanguageSettings | undefined> {
	if (scopedSettingsSupport) {
		let promise = documentSettings[textDocument.uri];
		if (!promise) {
			const configRequestParam = { items: [{ scopeUri: textDocument.uri, section: textDocument.languageId }] };
			promise = connection.sendRequest(ConfigurationRequest.type, configRequestParam).then(s => s[0] as LanguageSettings | undefined);
			documentSettings[textDocument.uri] = promise;
		}
		return promise;
	}
	return Promise.resolve(undefined);
}

// Server-side notification listener
connection.onDidChangeConfiguration(change => {
	updateConfiguration(change.settings as { [languageId: string]: LanguageSettings });
});
```

**Key aspects:**
- LSP declares which config sections trigger server updates.
- Server caches per-document settings keyed by URI.
- Fallback to undefined if client doesn't support scoped config.

---

## Summary: Core LSP Patterns for Rust/Tauri Port

A Rust/Tauri port must support:

1. **LanguageClient Abstraction**: Pluggable transport (IPC, Web Worker, Tauri IPC) with same clientOptions interface.
2. **Connection Lifecycle**: onInitialize handshake with capability negotiation and ServerCapabilities response.
3. **16 LSP Handlers**: Completion, Hover, DocumentSymbol, Definition, DocumentHighlight, DocumentLinks, References, CodeAction, DocumentColor, ColorPresentation, Rename, FoldingRanges, SelectionRanges, DocumentFormatting, DocumentRangeFormatting, plus Shutdown.
4. **Middleware Layer**: Intercept client-side responses to adapt for UI (ranges, labels, filtering).
5. **Custom RPC Types**: RequestType<P, R, E> and NotificationType<P> for typed protocol definitions.
6. **Error Handling**: runSafeAsync pattern with cancellation token checks and error fallback values.
7. **Diagnostic Dual-Mode**: Push (server-initiated) and Pull (client-initiated) variants selected via capability.
8. **File System Abstraction**: Named RequestTypes for fs/content, fs/stat, fs/readDir with protocol-based routing.
9. **Settings Sync**: ConfigurationSection in synchronize block + ConfigurationRequest for scoped document settings.
10. **Drop/Paste Providers**: Register language-specific content adapters for file URI→relative path transformation.
11. **TextDocuments Manager**: Listen for open/change/close events, maintain in-memory document cache.
12. **Custom Notifications**: Untyped push from either side (e.g., css/customDataChanged for data reloads).

All handlers use the `runSafeAsync` wrapper to ensure cancellation safety and error resilience. The architecture cleanly separates **platform transport** (IPC vs. Worker) from **language protocol** (LSP handlers).

## External References
<!-- Source: codebase-online-researcher sub-agent -->
# Partition 28: extensions/css-language-features/ — LSP Dependency Research

## TypeScript / npm Libraries (used in vscode-atomic)

#### vscode-languageclient (v10.0.0-next.20)
**Docs:** https://github.com/Microsoft/vscode-languageserver-node#readme
**Relevant behaviour:** Runs inside the VS Code extension host process (the client side). Provides `LanguageClient` / `BaseLanguageClient`, `LanguageClientOptions`, `ServerOptions`, `TransportKind`, middleware hooks (`ProvideCompletionItemsSignature`), and notification/request types. It spawns the language server child process over IPC (node) or a Web Worker (browser), performs capability negotiation, routes JSON-RPC messages, and re-exposes LSP responses as VS Code API calls. The `^10.0.0-next` line targets the unreleased v10 branch (compiled for ES2022 / Node 22, uses package.json `exports` instead of `main`).
**Where used:**
- `extensions/css-language-features/client/src/cssClient.ts:7` — imports `Disposable, LanguageClientOptions, ProvideCompletionItemsSignature, NotificationType, BaseLanguageClient, DocumentRangeFormattingParams, DocumentRangeFormattingRequest`
- `extensions/css-language-features/client/src/node/cssClientMain.ts:7` — imports `BaseLanguageClient, LanguageClient, LanguageClientOptions, ServerOptions, TransportKind` from `vscode-languageclient/node`; constructs `new LanguageClient(id, name, serverOptions, clientOptions)` with `TransportKind.ipc`

---

#### vscode-languageserver (v10.0.0-next.16)
**Docs:** https://github.com/Microsoft/vscode-languageserver-node#readme
**Relevant behaviour:** Runs inside the server child process. Exposes `createConnection()`, `Connection`, `TextDocuments`, `InitializeParams/Result`, `ServerCapabilities`, `TextDocumentSyncKind`, `Diagnostic`, `WorkspaceFolder`, and all LSP method handler wiring. `createConnection()` auto-detects the transport (stdio / IPC / socket) from `process.argv`. `TextDocuments` implements the server-side open/change/close document store with incremental sync. The server calls `connection.onInitialize`, `connection.onCompletion`, `connection.onHover`, etc., and calls `connection.listen()` to start the event loop. Same v10 pre-release as the client; they are co-versioned in the monorepo.
**Where used:**
- `extensions/css-language-features/server/src/cssServer.ts:6-8` — imports `Connection, TextDocuments, InitializeParams, InitializeResult, ServerCapabilities, ConfigurationRequest, WorkspaceFolder, TextDocumentSyncKind, NotificationType, Disposable, TextDocumentIdentifier, Range, FormattingOptions, TextEdit, Diagnostic`
- `extensions/css-language-features/server/src/node/cssServerMain.ts:6` — imports `createConnection, Connection, Disposable` from `vscode-languageserver/node`; line 12 `const connection: Connection = createConnection()`
- `extensions/css-language-features/server/src/utils/documentContext.ts:8` — imports `WorkspaceFolder`

---

#### vscode-css-languageservice (v7.0.0-next.1)
**Docs:** https://github.com/Microsoft/vscode-css-languageservice#readme
**Relevant behaviour:** Pure-TypeScript language intelligence layer for CSS/SCSS/Less. Provides `getCSSLanguageService()`, `getSCSSLanguageService()`, `getLESSLanguageService()`, each returning a `LanguageService` object with methods: `doValidation`, `doComplete`, `doHover`, `findDefinition`, `findReferences`, `findDocumentHighlights`, `findDocumentLinks`, `findDocumentSymbols`, `doCodeActions`, `findDocumentColors`, `getColorPresentations`, `doRename`, `getFoldingRanges`, `getSelectionRanges`, `parseStylesheet`. Also exports shared types: `TextDocument`, `Position`, `Stylesheet`, `LanguageSettings`, `DocumentContext`, `CodeActionKind`, `FileType`. This is the semantic engine that is transport-agnostic — it does not know about LSP at all; the server layer in `cssServer.ts` bridges its output into LSP response shapes.
**Where used:**
- `extensions/css-language-features/server/src/cssServer.ts:10` — imports `getCSSLanguageService, getSCSSLanguageService, getLESSLanguageService, LanguageSettings, LanguageService, Stylesheet, TextDocument, Position, CodeActionKind`
- `extensions/css-language-features/server/src/utils/documentContext.ts:6` — imports `DocumentContext`
- `extensions/css-language-features/server/src/node/nodeFs.ts:10` — imports `FileType`

---

#### vscode-uri (v3.1.0)
**Docs:** https://github.com/microsoft/vscode-uri#readme
**Relevant behaviour:** RFC 3986-compliant URI implementation used consistently across VS Code and its extensions. Provides an immutable `URI` class with `.parse(string)`, `.file(path)`, `.toString()`, `.fsPath`, `.scheme`, `.authority`, `.path`, `.query`, `.fragment`. Also exports `Utils` helpers: `Utils.joinPath`, `Utils.basename`, `Utils.dirname`, `Utils.extname`, `Utils.resolvePath`. Used in the server for converting `file://` URIs to filesystem paths and for resolving relative @import paths into absolute document URIs.
**Where used:**
- `extensions/css-language-features/server/src/cssServer.ts:9` — `import { URI } from 'vscode-uri'` for constructing `file://` URIs from `params.rootPath` during `onInitialize`
- `extensions/css-language-features/server/src/utils/documentContext.ts:9` — `import { Utils, URI } from 'vscode-uri'`; line 33 `URI.parse(base)` to resolve CSS `@import` links
- `extensions/css-language-features/server/src/node/nodeFs.ts:7` — `import { URI as Uri } from 'vscode-uri'` for FS path conversion in the Node filesystem request service
- `extensions/css-language-features/client/src/customData.ts:7` — `import { Utils } from 'vscode-uri'` for resolving custom data file paths

---

## Rust Equivalents

#### tower-lsp (v0.20.0)
**Docs:** https://docs.rs/tower-lsp / https://github.com/ebkalderon/tower-lsp
**Relevant behaviour:** Async LSP server framework built on top of `tower` (the `Service` abstraction) and `tokio`. Defines the `LanguageServer` trait whose methods (`initialize`, `initialized`, `shutdown`, `completion`, `hover`, `goto_definition`, etc.) map 1-to-1 to LSP requests. `LspService::new(|client| Backend { client })` wraps the impl; `Server::new(stdin, stdout, socket).serve(service)` runs the stdio loop. The `Client` handle allows server-to-client push (diagnostics, `window/logMessage`, `workspace/applyEdit`). Bundles `lsp-types` re-exports. Last release 0.20.0 (August 2023); actively maintained but the v0.x version indicates pre-stable API. Closest Rust equivalent to `vscode-languageserver`.
**Maps to:** `vscode-languageserver` (server side) and partially `vscode-languageclient` (the client push path)

---

#### lsp-server (v0.7.9)
**Docs:** https://docs.rs/lsp-server / https://github.com/rust-lang/rust-analyzer (lib/lsp-server)
**Relevant behaviour:** Synchronous, crossbeam-channel-based LSP scaffold extracted from rust-analyzer. Exposes `Connection`, `Message`, `Request`, `Response`, `Notification`, `ReqQueue`. The caller owns the dispatch loop — there is no async runtime dependency. Suitable when you want full control over scheduling (e.g., a single-threaded server or integration with a custom executor). Less abstracted than `tower-lsp`; you manually match request method strings and dispatch. Updated August 2025 (0.7.9). Closest Rust equivalent to the lower-level JSON-RPC plumbing in `vscode-languageserver` / `vscode-jsonrpc`.
**Maps to:** `vscode-languageserver` (lower-level alternative to tower-lsp)

---

#### lsp-types (v0.97.0)
**Docs:** https://docs.rs/lsp-types / https://github.com/gluon-lang/lsp-types
**Relevant behaviour:** Serde-serializable Rust structs and enums for the full LSP 3.17 type system: `InitializeParams`, `InitializeResult`, `ServerCapabilities`, `TextDocumentSyncKind`, `CompletionItem`, `Diagnostic`, `WorkspaceFolder`, `Position`, `Range`, `TextEdit`, `Url` (via `url` crate), etc. Used as the type layer by both `tower-lsp` and `lsp-server`. Proposed 3.18 features available behind a `proposed` feature flag. Equivalent to `vscode-languageserver-types` and `vscode-languageserver-protocol` on the TS side.
**Maps to:** `vscode-languageserver-types` + `vscode-languageserver-protocol`

---

## Prose Summary

The `extensions/css-language-features/` partition implements a two-process LSP architecture. The **client** (`vscode-languageclient ^10.0.0-next.20`) lives in the VS Code extension host and manages spawning the server process, the JSON-RPC transport over IPC, and bridging LSP capabilities back into VS Code API calls. The **server** (`vscode-languageserver ^10.0.0-next.16`) runs in a separate Node.js process; it calls `createConnection()`, wires up `onInitialize`/`onCompletion`/`onHover` handlers, and delegates all semantic work to `vscode-css-languageservice`. That service library is entirely transport-agnostic: it takes a `TextDocument` + cursor offset and returns strongly-typed completions, diagnostics, hover text, symbol lists, etc. for CSS/SCSS/Less. `vscode-uri` underpins all three layers, providing RFC 3986 URI parsing and `file://`-to-path conversion used for resolving `@import` links and workspace folder roots.

For a Tauri/Rust port the mapping is direct: **`tower-lsp`** (or **`lsp-server`** for a lower-level, sync approach) replaces the combined `vscode-languageserver` + `vscode-languageclient` JSON-RPC layer, and **`lsp-types`** replaces `vscode-languageserver-types`/`vscode-languageserver-protocol`. The hardest substitution is `vscode-css-languageservice` itself — there is no mature Rust equivalent; the real porting work is either wrapping the TypeScript service via a Node.js sidecar (keeping it as a subprocess, talking LSP to it) or rewriting CSS parsing/intelligence in Rust (e.g., building on `lightningcss` for parsing and writing custom completion/hover logic). The `vscode-uri` library maps trivially to Rust's `url` crate (already a transitive dependency of `lsp-types`).

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
