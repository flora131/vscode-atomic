# Partition 25 of 79 — Findings

## Scope
`extensions/json-language-features/` (19 files, 3,042 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Porting JSON Language Features: TypeScript/Electron to Tauri/Rust

## Implementation

### Client-Side LSP Integration (TypeScript)
- `extensions/json-language-features/client/src/jsonClient.ts` - Core LanguageClient configuration and setup
- `extensions/json-language-features/client/src/node/jsonClientMain.ts` - Node.js/Electron entry point with LanguageClient initialization
- `extensions/json-language-features/client/src/browser/jsonClientMain.ts` - Browser/Web worker entry point variant
- `extensions/json-language-features/client/src/languageStatus.ts` - Language status UI integration
- `extensions/json-language-features/client/src/languageParticipants.ts` - Event handling for language client
- `extensions/json-language-features/client/src/node/schemaCache.ts` - File-based schema caching for Node.js environment

### Server-Side LSP Implementation (TypeScript)
- `extensions/json-language-features/server/src/jsonServer.ts` - Core language server protocol implementation
- `extensions/json-language-features/server/src/node/jsonServerMain.ts` - Node.js entry point for server initialization
- `extensions/json-language-features/server/src/node/jsonServerNodeMain.ts` - Extended Node.js configuration
- `extensions/json-language-features/server/src/browser/jsonServerMain.ts` - Browser/Web worker entry point variant
- `extensions/json-language-features/server/src/browser/jsonServerWorkerMain.ts` - Worker-specific implementation

### Utilities
- `extensions/json-language-features/client/src/utils/hash.ts` - Hash functions for schema caching
- `extensions/json-language-features/client/src/utils/urlMatch.ts` - URL pattern matching for file associations
- `extensions/json-language-features/server/src/utils/validation.ts` - JSON validation utilities
- `extensions/json-language-features/server/src/utils/runner.ts` - Command runner utilities
- `extensions/json-language-features/server/src/utils/strings.ts` - String processing utilities
- `extensions/json-language-features/server/src/languageModelCache.ts` - In-memory document caching with LRU eviction

## Configuration

### Build Configuration
- `extensions/json-language-features/esbuild.mts` - Production build configuration for client and server
- `extensions/json-language-features/esbuild.browser.mts` - Browser/Web worker build configuration
- `extensions/json-language-features/client/tsconfig.json` - Client TypeScript configuration
- `extensions/json-language-features/client/tsconfig.browser.json` - Browser-specific TypeScript config
- `extensions/json-language-features/server/tsconfig.json` - Server TypeScript configuration
- `extensions/json-language-features/server/tsconfig.browser.json` - Browser server TypeScript config

### Package Management
- `extensions/json-language-features/package.json` - Root extension manifest and dependencies
- `extensions/json-language-features/package-lock.json` - Root dependency lock file
- `extensions/json-language-features/server/package.json` - Server-specific npm configuration
- `extensions/json-language-features/server/package-lock.json` - Server dependency lock file
- `extensions/json-language-features/.npmrc` - NPM configuration for root
- `extensions/json-language-features/server/.npmrc` - NPM configuration for server
- `extensions/json-language-features/server/.npmignore` - Files to exclude from npm publication

### VS Code Configuration
- `extensions/json-language-features/.vscode/launch.json` - Debug launch configurations for extension testing
- `extensions/json-language-features/.vscode/tasks.json` - Compile and build tasks
- `extensions/json-language-features/server/.vscode/launch.json` - Server-specific debug configuration
- `extensions/json-language-features/server/.vscode/tasks.json` - Server build tasks
- `extensions/json-language-features/.vscodeignore` - Files excluded from extension package
- `extensions/json-language-features/package.nls.json` - Localization strings

### Server Distribution
- `extensions/json-language-features/server/bin/vscode-json-languageserver` - Executable entry point for standalone server

## Documentation

- `extensions/json-language-features/README.md` - Extension overview and feature documentation
- `extensions/json-language-features/CONTRIBUTING.md` - Development setup, debugging, and contribution guide for vscode-json-languageservice integration
- `extensions/json-language-features/server/README.md` - Comprehensive server documentation covering: capabilities (completion, hover, document symbols, color decorators, formatting, folding ranges, goto definition, diagnostics), configuration options (initialization, settings, schema configuration), integration instructions, and dependencies on jsonc-parser and vscode-json-languageservice

## Types / Interfaces

The following files define or extend TypeScript interfaces for the language server protocol and custom extensions:
- `extensions/json-language-features/server/README.md` - Contains interface definitions for `ISchemaAssociations` and `ISchemaAssociation` (lines 158-188)
- `extensions/json-language-features/client/src/jsonClient.ts` - LanguageClient configuration types
- `extensions/json-language-features/client/src/languageParticipants.ts` - Event participant type definitions
- `extensions/json-language-features/server/src/jsonServer.ts` - Server protocol handler types

## Notable Clusters

### Client Architecture (5 files)
`extensions/json-language-features/client/src/` contains the VS Code extension client that establishes the LanguageClient connection, with platform variants for Node.js and browser environments, plus utilities for schema caching, URL matching, and hashing.

### Server Architecture (5 files)
`extensions/json-language-features/server/src/` implements the Language Server Protocol with document caching, JSON validation, command execution, and platform-specific initialization for both Node.js and browser/Worker environments.

### Build System (4 files)
esbuild configuration (root and browser variants) plus separate TypeScript configurations per platform (client/server × node/browser) enable multi-target compilation from a single TypeScript codebase.

### External Dependencies (3 libraries)
According to server/README.md, the implementation delegates to:
- `jsonc-parser` - JSON parsing and tokenization
- `vscode-json-languageservice` - Reusable library implementing all language features (completion, validation, formatting, etc.)
- `vscode-languageserver-node` - LSP server implementation for Node.js

## Summary

The JSON Language Features extension consists of a TypeScript-based Language Server Protocol implementation split into client (VS Code extension) and server (standalone executable) components. The architecture supports both Node.js/Electron and browser/Web Worker runtimes through dual entry points and build configurations. The server implements comprehensive LSP capabilities including validation, completion, formatting, and diagnostics, delegating core JSON language analysis to the external vscode-json-languageservice library. Porting this to Tauri/Rust would require: (1) translating the client communication layer to Tauri's frontend-backend bridge, (2) rewriting the server in Rust or embedding a Rust JSON parser, (3) reimplementing all LSP protocol handlers and document management, (4) replicating the schema caching and validation logic, and (5) establishing inter-process communication patterns compatible with Tauri's architecture instead of Node.js IPC/stdio channels.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Porting VS Code JSON Language Features to Tauri/Rust

## Research Scope
This document analyzes patterns in `extensions/json-language-features/` (17 TypeScript files, ~3,000 LOC) to identify what would be required to port VS Code's core JSON LSP client and server from TypeScript/Electron to Tauri/Rust.

---

## Core Architecture Patterns

### Pattern: Language Client Initialization (Platform-Specific)

**Where:** `extensions/json-language-features/client/src/node/jsonClientMain.ts:20-57`

Node.js variant using IPC transport:
```typescript
const serverOptions: ServerOptions = {
  run: { module: serverModule, transport: TransportKind.ipc },
  debug: { module: serverModule, transport: TransportKind.ipc, options: debugOptions }
};

const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
  return new LanguageClient(id, name, serverOptions, clientOptions);
};
```

**Where:** `extensions/json-language-features/client/src/browser/jsonClientMain.ts:14-22`

Browser variant using Web Workers:
```typescript
const worker = new Worker(serverMain.toString());
worker.postMessage({ i10lLocation: l10n.uri?.toString(false) ?? '' });

const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
  return new LanguageClient(id, name, worker, clientOptions);
};
```

**Variations / call-sites:** Two transport implementations (IPC vs Worker) passed through `LanguageClientConstructor` function type, allowing both to coexist. Activation hooks in both `node/` and `browser/` directories.

---

### Pattern: Language Client Middleware Chain

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:281-384`

Complex middleware stack for intercepting LSP operations:
```typescript
const clientOptions: LanguageClientOptions = {
  documentSelector,
  middleware: {
    workspace: {
      didChangeConfiguration: () => client.sendNotification(DidChangeConfigurationNotification.type, { settings: getSettings(true) })
    },
    provideDiagnostics: async (uriOrDoc, previousResolutId, token, next) => {
      const diagnostics = await next(uriOrDoc, previousResolutId, token);
      if (diagnostics && diagnostics.kind === DocumentDiagnosticReportKind.Full) {
        const uri = uriOrDoc instanceof Uri ? uriOrDoc : uriOrDoc.uri;
        diagnostics.items = handleSchemaErrorDiagnostics(uri, diagnostics.items);
      }
      return diagnostics;
    },
    provideCompletionItem(document, position, context, token, next) {
      // ... range adjustment logic
      const r = next(document, position, context, token);
      if (isThenable<CompletionItem[] | CompletionList | null | undefined>(r)) {
        return r.then(updateProposals);
      }
      return updateProposals(r);
    }
  }
};
```

**Variations / call-sites:** Middleware hooks for: `provideDiagnostics`, `handleDiagnostics`, `provideCompletionItem`, `provideHover`, `provideFoldingRanges`, `provideDocumentColors`, `provideDocumentSymbols`. Each intercepts LSP feature before/after server communication.

---

### Pattern: Bidirectional Request/Notification System

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:395-443`

Client handling server requests (reverse RPC):
```typescript
client.onRequest(VSCodeContentRequest.type, async (uriPath: string) => {
  const uri = Uri.parse(uriPath);
  const uriString = uri.toString(true);
  
  if (uri.scheme === 'untitled') {
    throw new ResponseError(SchemaRequestServiceErrors.UntitledAccessError, l10n.t('Unable to load {0}', uriString));
  }
  
  if (uri.scheme === 'vscode') {
    try {
      runtime.logOutputChannel.info('read schema from vscode: ' + uriString);
      ensureFilesystemWatcherInstalled(uri);
      const content = await workspace.fs.readFile(uri);
      return new TextDecoder().decode(content);
    } catch (e) {
      throw new ResponseError(SchemaRequestServiceErrors.VSCodeAccessError, e.toString(), e);
    }
  }
  // ... more scheme handlers
});
```

**Where:** `extensions/json-language-features/server/src/jsonServer.ts:296-316`

Server handling notifications and requests:
```typescript
connection.onNotification(SchemaAssociationNotification.type, associations => {
  schemaAssociations = associations;
  updateConfiguration();
});

connection.onNotification(SchemaContentChangeNotification.type, uriOrUris => {
  let needsRevalidation = false;
  if (Array.isArray(uriOrUris)) {
    for (const uri of uriOrUris) {
      if (languageService.resetSchema(uri)) {
        needsRevalidation = true;
      }
    }
  } else {
    needsRevalidation = languageService.resetSchema(uriOrUris);
  }
  if (needsRevalidation) {
    diagnosticsSupport?.requestRefresh();
  }
});

connection.onRequest(ForceValidateRequest.type, async uri => {
  const document = documents.get(uri);
  if (document) {
    updateConfiguration();
    return await validateTextDocument(document);
  }
  return [];
});
```

**Variations / call-sites:** Eight custom request/notification types defined via `namespace`: `VSCodeContentRequest`, `SchemaContentChangeNotification`, `ForceValidateRequest`, `ForceValidateAllRequest`, `LanguageStatusRequest`, `ValidateContentRequest`, `SchemaAssociationNotification`, `DocumentSortingRequest`.

---

### Pattern: Provider Registration & Dynamic Capability Negotiation

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:504-543`

Dynamic code action provider registration:
```typescript
toDispose.push(languages.registerCodeActionsProvider(documentSelector, {
  provideCodeActions(_document: TextDocument, _range: Range, context: CodeActionContext): CodeAction[] {
    const codeActions: CodeAction[] = [];
    for (const diagnostic of context.diagnostics) {
      if (typeof diagnostic.code !== 'number') {
        continue;
      }
      switch (diagnostic.code) {
        case ErrorCodes.UntrustedSchemaError: {
          const title = l10n.t('Configure Trusted Domains...');
          const action = new CodeAction(title, CodeActionKind.QuickFix);
          // ... action setup
          codeActions.push(action);
        }
        break;
      }
    }
    return codeActions;
  }
}, {
  providedCodeActionKinds: [CodeActionKind.QuickFix]
}));
```

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:578-608`

Dynamic formatter registration based on settings:
```typescript
function updateFormatterRegistration() {
  const formatEnabled = workspace.getConfiguration().get(SettingIds.enableFormatter);
  if (!formatEnabled && rangeFormatting) {
    rangeFormatting.dispose();
    rangeFormatting = undefined;
  } else if (formatEnabled && !rangeFormatting) {
    rangeFormatting = languages.registerDocumentRangeFormattingEditProvider(documentSelector, {
      provideDocumentRangeFormattingEdits(document: TextDocument, range: Range, options: FormattingOptions, token: CancellationToken) {
        const filesConfig = workspace.getConfiguration('files', document);
        const fileFormattingOptions = {
          trimTrailingWhitespace: filesConfig.get<boolean>('trimTrailingWhitespace'),
          trimFinalNewlines: filesConfig.get<boolean>('trimFinalNewlines'),
          insertFinalNewline: filesConfig.get<boolean>('insertFinalNewline'),
        };
        const params: DocumentRangeFormattingParams = {
          textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(document),
          range: client.code2ProtocolConverter.asRange(range),
          options: client.code2ProtocolConverter.asFormattingOptions(options, fileFormattingOptions)
        };
        return client.sendRequest(DocumentRangeFormattingRequest.type, params, token);
      }
    });
  }
}
```

**Variations / call-sites:** Server-side dynamic formatter registration in `jsonServer.ts:278-291` mirrors this pattern using `connection.client.register()`.

---

### Pattern: Runtime Abstraction Layer

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:146-153`

Client-side runtime interface:
```typescript
export interface Runtime {
  schemaRequests: SchemaRequestService;
  telemetry?: TelemetryReporter;
  readonly timer: {
    setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable;
  };
  logOutputChannel: LogOutputChannel;
}
```

**Where:** `extensions/json-language-features/server/src/jsonServer.ts:73-85`

Server-side runtime interface:
```typescript
export interface RuntimeEnvironment {
  file?: RequestService;
  http?: RequestService;
  configureHttpRequests?(proxy: string | undefined, strictSSL: boolean): void;
  readonly timer: {
    setImmediate(callback: (...args: any[]) => void, ...args: any[]): Disposable;
    setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable;
  };
}
```

**Where:** `extensions/json-language-features/server/src/node/jsonServerMain.ts:58-72`

Node.js runtime implementation:
```typescript
const runtime: RuntimeEnvironment = {
  timer: {
    setImmediate(callback: (...args: any[]) => void, ...args: any[]): Disposable {
      const handle = setImmediate(callback, ...args);
      return { dispose: () => clearImmediate(handle) };
    },
    setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable {
      const handle = setTimeout(callback, ms, ...args);
      return { dispose: () => clearTimeout(handle) };
    }
  },
  file: getFileRequestService(),
  http: getHTTPRequestService(),
  configureHttpRequests
};
```

**Where:** `extensions/json-language-features/server/src/browser/jsonServerMain.ts:18-29`

Browser runtime implementation (no file/http, stub setImmediate):
```typescript
const runtime: RuntimeEnvironment = {
  timer: {
    setImmediate(callback: (...args: any[]) => void, ...args: any[]): Disposable {
      const handle = setTimeout(callback, 0, ...args);
      return { dispose: () => clearTimeout(handle) };
    },
    setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable {
      const handle = setTimeout(callback, ms, ...args);
      return { dispose: () => clearTimeout(handle) };
    }
  }
};
```

**Variations / call-sites:** Runtime injected into `startServer()` and `startClient()`. Allows environment-specific (Node vs Browser) implementations without changing core logic.

---

### Pattern: Document/Language Lifecycle Management

**Where:** `extensions/json-language-features/server/src/jsonServer.ts:427-437`

Language model cache with document lifecycle:
```typescript
const jsonDocuments = getLanguageModelCache<JSONDocument>(10, 60, document => languageService.parseJSONDocument(document));
documents.onDidClose(e => {
  jsonDocuments.onDocumentRemoved(e.document);
});
connection.onShutdown(() => {
  jsonDocuments.dispose();
});

function getJSONDocument(document: TextDocument): JSONDocument {
  return jsonDocuments.get(document);
}
```

**Where:** `extensions/json-language-features/client/src/languageParticipants.ts:31-78`

Client-side language participant discovery and tracking:
```typescript
export function getLanguageParticipants(): LanguageParticipants {
  const onDidChangeEmmiter = new EventEmitter<void>();
  let languages = new Set<string>();
  let comments = new Set<string>();

  function update() {
    const oldLanguages = languages, oldComments = comments;
    languages = new Set();
    languages.add('json');
    languages.add('jsonc');
    languages.add('snippets');
    comments = new Set();
    comments.add('jsonc');
    comments.add('snippets');

    for (const extension of extensions.allAcrossExtensionHosts) {
      const jsonLanguageParticipants = extension.packageJSON?.contributes?.jsonLanguageParticipants as LanguageParticipantContribution[];
      if (Array.isArray(jsonLanguageParticipants)) {
        for (const jsonLanguageParticipant of jsonLanguageParticipants) {
          const languageId = jsonLanguageParticipant.languageId;
          if (typeof languageId === 'string') {
            languages.add(languageId);
            if (jsonLanguageParticipant.comments === true) {
              comments.add(languageId);
            }
          }
        }
      }
    }
    return !isEqualSet(languages, oldLanguages) || !isEqualSet(comments, oldComments);
  }
  update();

  const changeListener = extensions.onDidChange(_ => {
    if (update()) {
      onDidChangeEmmiter.fire();
    }
  });

  return {
    onDidChange: onDidChangeEmmiter.event,
    get documentSelector() { return Array.from(languages); },
    hasLanguage(languageId: string) { return languages.has(languageId); },
    useComments(languageId: string) { return comments.has(languageId); },
    dispose: () => changeListener.dispose()
  };
}
```

**Variations / call-sites:** `TextDocuments` from LSP manages document lifecycle; paired with custom cache layer for parsed AST reuse.

---

### Pattern: Configuration Cascading & Settings Sync

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:643-905`

Client-side settings computation and distribution:
```typescript
function computeSettings(): Settings {
  const configuration = workspace.getConfiguration();
  const httpSettings = workspace.getConfiguration('http');

  const normalizeLimit = (settingValue: any) => Math.trunc(Math.max(0, Number(settingValue))) || 5000;

  resultLimit = normalizeLimit(workspace.getConfiguration().get(SettingIds.maxItemsComputed));
  const editorJSONSettings = workspace.getConfiguration(SettingIds.editorSection, { languageId: 'json' });
  const editorJSONCSettings = workspace.getConfiguration(SettingIds.editorSection, { languageId: 'jsonc' });

  jsonFoldingLimit = normalizeLimit(editorJSONSettings.get(SettingIds.foldingMaximumRegions));
  jsoncFoldingLimit = normalizeLimit(editorJSONCSettings.get(SettingIds.foldingMaximumRegions));
  // ... more limits
  
  const schemas: JSONSchemaSettings[] = [];

  const settings: Settings = {
    http: {
      proxy: httpSettings.get('proxy'),
      proxyStrictSSL: httpSettings.get('proxyStrictSSL')
    },
    json: {
      validate: { enable: configuration.get(SettingIds.enableValidation) },
      format: { enable: configuration.get(SettingIds.enableFormatter) },
      // ... more settings
    }
  };

  const collectSchemaSettings = (schemaSettings: JSONSchemaSettings[] | undefined, folderUri: string | undefined, settingsLocation: Uri | undefined) => {
    if (schemaSettings) {
      for (const setting of schemaSettings) {
        const url = getSchemaId(setting, settingsLocation);
        if (url) {
          const schemaSetting: JSONSchemaSettings = { url, fileMatch: setting.fileMatch, folderUri, schema: setting.schema };
          schemas.push(schemaSetting);
        }
      }
    }
  };

  const folders = workspace.workspaceFolders ?? [];
  const schemaConfigInfo = workspace.getConfiguration('json', null).inspect<JSONSchemaSettings[]>('schemas');
  
  if (schemaConfigInfo) {
    collectSchemaSettings(schemaConfigInfo.globalValue, undefined, undefined);
    if (workspace.workspaceFile) {
      if (schemaConfigInfo.workspaceValue) {
        const settingsLocation = Uri.joinPath(workspace.workspaceFile, '..');
        collectSchemaSettings(schemaConfigInfo.workspaceValue, undefined, settingsLocation);
      }
      for (const folder of folders) {
        const folderUri = folder.uri;
        const folderSchemaConfigInfo = workspace.getConfiguration('json', folderUri).inspect<JSONSchemaSettings[]>('schemas');
        collectSchemaSettings(folderSchemaConfigInfo?.workspaceFolderValue, folderUri.toString(false), folderUri);
      }
    }
  }
  return settings;
}
```

**Where:** `extensions/json-language-features/server/src/jsonServer.ts:258-293`

Server-side configuration change handling:
```typescript
connection.onDidChangeConfiguration((change) => {
  const settings = <Settings>change.settings;
  runtime.configureHttpRequests?.(settings?.http?.proxy, !!settings.http?.proxyStrictSSL);
  jsonConfigurationSettings = settings.json?.schemas;
  validateEnabled = !!settings.json?.validate?.enable;
  commentsSeverity = settings.json?.validate?.comments;
  trailingCommasSeverity = settings.json?.validate?.trailingCommas;
  schemaValidationSeverity = settings.json?.validate?.schemaValidation;
  schemaRequestSeverity = settings.json?.validate?.schemaRequest;
  keepLinesEnabled = settings.json?.keepLines?.enable || false;
  updateConfiguration();

  // Dynamic formatter registration
  if (dynamicFormatterRegistration) {
    const enableFormatter = settings.json?.format?.enable;
    if (enableFormatter) {
      if (!formatterRegistrations) {
        const documentSelector = [{ language: 'json' }, { language: 'jsonc' }];
        formatterRegistrations = [
          connection.client.register(DocumentRangeFormattingRequest.type, { documentSelector }),
          connection.client.register(DocumentFormattingRequest.type, { documentSelector })
        ];
      }
    } else if (formatterRegistrations) {
      formatterRegistrations.forEach(p => p.then(r => r.dispose()));
      formatterRegistrations = null;
    }
  }
});
```

**Variations / call-sites:** Multi-level configuration inspection (global, workspace, folder); folder-relative schema URL resolution; per-language settings (json vs jsonc).

---

## Key Integration Points Requiring Ports

**Custom Request/Notification Protocol** (8 types):
- `VSCodeContentRequest` - Server reverse-calls client for schema content
- `SchemaAssociationNotification` - Extension schema contributions
- `SchemaContentChangeNotification` - Schema cache invalidation
- `ForceValidateRequest/ForceValidateAllRequest` - Explicit validation triggers
- `LanguageStatusRequest` - Status query
- `ValidateContentRequest` - Adhoc validation
- `DocumentSortingRequest` - JSON sort operation

**Platform Bridging**:
- IPC transport layer (Node.js)
- Web Worker transport layer (Browser)
- File system request service
- HTTP request service with proxy support
- Timer abstraction (setImmediate, setTimeout)

**UI Integration** (14+ registrations):
- Command registration (cache clear, validate, sort, retry, trusted domain config)
- Language status items (schema validation status)
- Code action providers (quick fixes)
- Document range formatting providers
- File system watchers (schema change detection)
- Configuration change listeners
- Extension lifecycle listeners

---

## Summary

The JSON language features extension demonstrates a well-layered architecture separating concerns across:

1. **Transport**: Abstracted through `LanguageClientConstructor` and connection implementations
2. **Runtime**: Environment-specific implementations (Node/Browser) injected as `RuntimeEnvironment`
3. **Middleware**: LSP feature interception for client-side transformations
4. **Bidirectional Communication**: Custom request/notification types for editor-specific operations
5. **Lifecycle Management**: Document parsing cache with automatic cleanup tied to document events
6. **Configuration**: Multi-scope cascade (global → workspace → folder) with dynamic re-registration

Porting to Tauri/Rust would require:
- Rust implementation of LSP client protocol and custom message types
- Tauri message passing bridge replacing IPC/Worker transports
- Native file/HTTP request services
- Configuration management tied to Tauri settings/file system
- Document cache implementation equivalent to `LanguageModelCache<T>`
- Provider registration system for Rust-based language features

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
