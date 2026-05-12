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
