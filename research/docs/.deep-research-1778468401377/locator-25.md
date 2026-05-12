# JSON Language Features Extension – File Locator

## Research Question
Porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust, focusing on LSP host pattern and language-server lifecycle management.

## Scope
`extensions/json-language-features/` (19 files, 3,042 LOC)

---

## Implementation

### Client Entry Points (Multi-Runtime)
- `extensions/json-language-features/client/src/node/jsonClientMain.ts` – Node.js/Electron runtime initialization with TelemetryReporter, schema caching, HTTP/IPC transport setup
- `extensions/json-language-features/client/src/browser/jsonClientMain.ts` – Web worker runtime, Worker-based LSP client, fetch-based schema requests

### Core LSP Client Logic
- `extensions/json-language-features/client/src/jsonClient.ts` – BaseLanguageClient wrapper, request/notification handlers (VSCodeContentRequest, SchemaContentChangeNotification, ForceValidateRequest, ValidateContentRequest, DocumentSortingRequest), middleware for diagnostics, completion, hover, folding, document symbols, formatting, color decorators; schema lifecycle management; settings synchronization
- `extensions/json-language-features/client/src/languageParticipants.ts` – Runtime extension discovery for language registrations (json, jsonc, snippets + plugin contributions), observer pattern for extension changes
- `extensions/json-language-features/client/src/languageStatus.ts` – UI status bar items for schema resolution errors, document symbol limits, schema load diagnostics (80+ lines of schema association resolution)

### Client Utilities
- `extensions/json-language-features/client/src/utils/hash.ts` – Hash utility for schema ID generation
- `extensions/json-language-features/client/src/utils/urlMatch.ts` – URL pattern matching for trusted domains
- `extensions/json-language-features/client/src/node/schemaCache.ts` – ETag-based schema caching with filesystem storage, cache invalidation, retry logic

### Server Entry Points (Multi-Runtime)
- `extensions/json-language-features/server/src/node/jsonServerMain.ts` – Node.js server, Connection from vscode-languageserver/node, HTTP/File RequestService setup, error handling
- `extensions/json-language-features/server/src/node/jsonServerNodeMain.ts` – Node process entry point (likely server launcher)
- `extensions/json-language-features/server/src/browser/jsonServerMain.ts` – Web Worker server, BrowserMessageReader/Writer, Runtime environment setup
- `extensions/json-language-features/server/src/browser/jsonServerWorkerMain.ts` – Worker script entry point

### Core LSP Server Logic
- `extensions/json-language-features/server/src/jsonServer.ts` – Connection initialization, TextDocuments manager, request handlers (ForceValidateRequest, ValidateContentRequest, LanguageStatusRequest, DocumentSortingRequest), notification handlers (SchemaAssociation, SchemaContentChange), language service configuration, diagnostics (push/pull support), completion, hover, symbol navigation, formatting, folding, color decorators, code actions (sort JSON)
- `extensions/json-language-features/server/src/utils/validation.ts` – registerDiagnosticsPullSupport and registerDiagnosticsPushSupport, validation request scheduling with debounce (500ms)
- `extensions/json-language-features/server/src/languageModelCache.ts` – LRU cache (maxEntries=10, TTL=60s) for parsed JSON documents with version tracking and auto-cleanup

### Server Utilities
- `extensions/json-language-features/server/src/utils/runner.ts` – Error formatting and safe execution wrappers (runSafe, runSafeAsync)
- `extensions/json-language-features/server/src/utils/strings.ts` – String utilities
- `extensions/json-language-features/server/bin/vscode-json-languageserver` – Binary launcher script

---

## Configuration

### Extension Manifest
- `extensions/json-language-features/package.json` – Extension metadata (v10.0.0), activation events (onLanguage:json, onLanguage:jsonc, onLanguage:snippets, onCommand:json.validate), main entry points (Node/Browser), declares capabilities (virtualWorkspaces, untrustedWorkspaces), contributes schema validation config, commands (clear cache, sort), dependencies (vscode-languageclient@^10.0.0-next.20, request-light, @vscode/extension-telemetry)

### Build & Tooling
- `extensions/json-language-features/esbuild.mts` – Node.js build config
- `extensions/json-language-features/esbuild.browser.mts` – Browser build config
- `extensions/json-language-features/client/tsconfig.json` – Client TypeScript config
- `extensions/json-language-features/client/tsconfig.browser.json` – Client browser-specific config
- `extensions/json-language-features/server/tsconfig.json` – Server TypeScript config
- `extensions/json-language-features/server/tsconfig.browser.json` – Server browser-specific config
- `extensions/json-language-features/.npmrc`, `extensions/json-language-features/.vscodeignore` – NPM and packaging configs
- `extensions/json-language-features/.vscode/launch.json` – Debug launch configurations
- `extensions/json-language-features/.vscode/tasks.json` – Build tasks

### Localization & Dependencies
- `extensions/json-language-features/package-lock.json`, `extensions/json-language-features/server/package-lock.json` – Dependency locks
- `extensions/json-language-features/package.nls.json` – i18n strings
- `extensions/json-language-features/server/.npmrc`, `extensions/json-language-features/server/.npmignore` – Server-specific NPM config

---

## Documentation

- `extensions/json-language-features/README.md` – Basic description (bundled with VS Code, links to official docs)
- `extensions/json-language-features/CONTRIBUTING.md` – Contribution guidelines
- `extensions/json-language-features/server/.vscode/tasks.json`, `extensions/json-language-features/server/.vscode/launch.json` – Server debug configs
- `extensions/json-language-features/server/README.md` – Server-specific documentation

---

## Notable Clusters

### LSP Lifecycle Management Pattern
The extension demonstrates a production-grade LSP host lifecycle:
1. **Client initialization** (node/browser-specific): Spawns/creates server, passes initialization options including capability negotiation
2. **Server handshake** (jsonServer.ts, jsonServerMain.ts): Receives InitializeParams, introspects client capabilities, registers language service with schema request strategy
3. **Configuration synchronization**: Two-way binding via DidChangeConfigurationNotification and onDidChangeConfiguration
4. **Schema lifecycle**: Notifications for schema content changes trigger revalidation across open documents
5. **Diagnostics modes**: Server supports both push (send after parse) and pull (client-driven document diagnostic requests) diagnostics
6. **Graceful shutdown**: Dispose patterns with TextDocuments cleanup and language model cache eviction

### Multi-Runtime Architecture
The extension is split into node and browser runtimes at both client and server levels:
- **Node client** (jsonClientMain.ts): IPC transport, filesystem schema cache with ETag support
- **Browser client** (browser/jsonClientMain.ts): Worker API, fetch-based schema requests
- **Node server** (node/jsonServerMain.ts): process-based with stdio/IPC, full filesystem access
- **Browser server** (browser/jsonServerMain.ts): Worker message passing, no file I/O

### Extension Participation Model
- Runtime discovery of language contributions via extension.packageJSON.contributes.jsonLanguageParticipants
- Dynamic client restart when extensions change (2-second debounce in startClient)
- Schema association aggregation from multiple sources: built-in, workspace config, extension manifest, dynamic schemas

### Request/Notification Patterns
Custom LSP extensions beyond the standard protocol:
- `vscode/content` – Client-to-server request for schema content (bypasses server file I/O for non-file protocols)
- `json/schemaContent` – Notification for schema cache invalidation
- `json/validate` – Forced validation request with diagnostics response
- `json/sort` – Document sorting with TextEdit array response
- `json/languageStatus` – Queried to populate status bar (schemas in use)
- `json/schemaAssociations` – Notification for associating patterns to schemas

### Middleware Transformation Pipeline
Client middleware modifies LSP protocol on the fly:
- Completion: Range-based insertion/replacement bounds adjustment
- Hover: MarkdownString update for code snippet sanitization
- Diagnostics: Schema error filtering based on schemaDownloadEnabled and trusted domains
- Document symbols: Result limit checking with status bar feedback

---

## Summary

The JSON language extension provides a reference implementation of VS Code's LSP host pattern. The codebase is organized around a **two-tier client-server split** that abstracts transport (Node IPC vs Web Worker) and runtime (filesystem access, HTTP schemes, schema caching). The **LanguageClient** class (from vscode-languageclient) wraps all LSP communication and middleware hooks.

Key porting considerations for Tauri/Rust:
1. **Transport**: Replace IPC with Tauri command invocation or in-process function calls
2. **Schema lifecycle**: Implement ETag-based caching and content change notifications (critical for large schema sets)
3. **Middleware**: Schema download trust gates, URL pattern matching, and diagnostics filtering must be at parity
4. **Diagnostics modes**: Both push and pull must be supported for compatibility with different LSP client capabilities
5. **Extension participants**: Dynamic language discovery needs equivalent in Rust (likely plugin system or module registry)
6. **Multi-environment**: Maintain separation between Node (full I/O) and browser (fetch) or equivalent native/WASM boundaries in Rust

