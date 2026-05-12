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
