# File Location Index: HTML Language Features Extension

Partition 13 scope: `extensions/html-language-features/` (84 files, ~9,248 LOC)

Research Question: What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

## Implementation

### Client Architecture
- `extensions/html-language-features/client/src/htmlClient.ts` — LSP client bootstrapping and feature registration; manages language client lifecycle, providers (semantic tokens, formatting, completion), custom data loading, and middleware for protocol conversion
- `extensions/html-language-features/client/src/node/htmlClientMain.ts` — Node.js client entry point; creates IPC-based LanguageClient with server process spawning and telemetry setup
- `extensions/html-language-features/client/src/browser/htmlClientMain.ts` — Browser client entry point; creates Worker-based LanguageClient for web execution; demonstrates LSP over Web Worker
- `extensions/html-language-features/client/src/requests.ts` — Custom request types (FsStatRequest, FsReadDirRequest); FileSystemProvider abstraction for platform-specific fs operations
- `extensions/html-language-features/client/src/node/nodeFs.ts` — Node.js filesystem adapter; wraps native fs module with FileSystemProvider interface

### Server Architecture  
- `extensions/html-language-features/server/src/htmlServer.ts` — LSP server core; handles connection lifecycle, document synchronization, custom notifications (CustomDataChanged), custom requests (AutoInsert, SemanticTokens), and dispatches to language modes
- `extensions/html-language-features/server/src/node/htmlServerMain.ts` — Node.js server entry point; creates IPC Connection, sets up runtime environment (timers, fs), captures console output
- `extensions/html-language-features/server/src/node/htmlServerNodeMain.ts` — Server CLI wrapper; referenced in build as main entry
- `extensions/html-language-features/server/src/browser/htmlServerMain.ts` — Browser server entry point; creates connection from BrowserMessageReader/Writer
- `extensions/html-language-features/server/src/browser/htmlServerWorkerMain.ts` — Web Worker bootstrap; handles async l10n loading before delegating to htmlServerMain

### Language Mode System
- `extensions/html-language-features/server/src/modes/languageModes.ts` — LanguageMode interface and factory; defines protocol for completions, hover, validation, diagnostics, formatting, semantic tokens, folding ranges, selection ranges across embedded HTML/CSS/JS
- `extensions/html-language-features/server/src/modes/htmlMode.ts` — HTML language mode implementation; wraps vscode-html-languageservice for completions, hover, formatting, symbols, links, folding
- `extensions/html-language-features/server/src/modes/cssMode.ts` — CSS mode for embedded style blocks
- `extensions/html-language-features/server/src/modes/javascriptMode.ts` — JavaScript mode for embedded script blocks; integrates TypeScript library definitions
- `extensions/html-language-features/server/src/modes/embeddedSupport.ts` — Region extraction logic; parses HTML to identify CSS/JS regions and their language contexts
- `extensions/html-language-features/server/src/modes/formatting.ts` — Unified formatting dispatcher for multi-language documents
- `extensions/html-language-features/server/src/modes/htmlFolding.ts` — Code folding regions for HTML structures
- `extensions/html-language-features/server/src/modes/selectionRanges.ts` — Semantic selection range expansion
- `extensions/html-language-features/server/src/modes/semanticTokens.ts` — Token classification provider; integrates HTML and CSS token types
- `extensions/html-language-features/server/src/modes/javascriptLibs.ts` — Inlined TypeScript library definitions for browser environments (handles lib.d.ts and jQuery)
- `extensions/html-language-features/server/src/modes/javascriptSemanticTokens.ts` — JS semantic token classification layer

### Client Features
- `extensions/html-language-features/client/src/autoInsertion.ts` — Auto-quote and auto-close tag insertion; listens to text changes, sends requests to server for intelligent insertion
- `extensions/html-language-features/client/src/languageParticipants.ts` — Extension participant registry pattern; allows other extensions to register languages that integrate with HTML server (e.g., handlebars)
- `extensions/html-language-features/client/src/customData.ts` — Custom data loader; discovers and loads HTML element/attribute definitions from workspace configs and extensions

### Server Utilities
- `extensions/html-language-features/server/src/languageModelCache.ts` — LRU cache for parsed documents; stores HTMLDocument/CSS models with version tracking and automatic cleanup
- `extensions/html-language-features/server/src/requests.ts` — Server-side request handlers (FsStatRequest, FsReadDirRequest); bridges client fs requests to runtime filesystem
- `extensions/html-language-features/server/src/node/nodeFs.ts` — Node.js filesystem implementation; async fs.stat and fs.readdir adapters
- `extensions/html-language-features/server/src/utils/documentContext.ts` — Document context provider for path resolution and completion context
- `extensions/html-language-features/server/src/utils/positions.ts` — Position/offset conversion utilities
- `extensions/html-language-features/server/src/utils/strings.ts` — String manipulation helpers
- `extensions/html-language-features/server/src/utils/arrays.ts` — Array utility functions
- `extensions/html-language-features/server/src/utils/validation.ts` — Diagnostic support; registers push or pull diagnostic providers with debouncing
- `extensions/html-language-features/server/src/utils/runner.ts` — Error formatting and safe execution wrapper

### Server Custom Data
- `extensions/html-language-features/server/src/customData.ts` — Custom data fetching from URIs; mirrors client pattern for server-side extension discovery

## Tests

### Server Tests
- `extensions/html-language-features/server/src/test/completions.test.ts` — Completion scenarios (tags, attributes, values)
- `extensions/html-language-features/server/src/test/formatting.test.ts` — Multi-language formatting
- `extensions/html-language-features/server/src/test/folding.test.ts` — Folding region detection
- `extensions/html-language-features/server/src/test/selectionRanges.test.ts` — Selection range expansion
- `extensions/html-language-features/server/src/test/semanticTokens.test.ts` — Token classification
- `extensions/html-language-features/server/src/test/rename.test.ts` — Element rename operations
- `extensions/html-language-features/server/src/test/words.test.ts` — Word boundary detection
- `extensions/html-language-features/server/src/test/embedded.test.ts` — Embedded language region detection
- `extensions/html-language-features/server/src/test/documentContext.test.ts` — Document context path resolution

### Test Fixtures
- `extensions/html-language-features/server/src/test/pathCompletionFixtures/` — File hierarchy for path completion tests
- `extensions/html-language-features/server/src/test/fixtures/inputs/` — Input HTML files for formatting tests
- `extensions/html-language-features/server/src/test/fixtures/expected/` — Expected output for formatting tests

### Test Infrastructure
- `extensions/html-language-features/server/test/index.js` — Test runner

## Types / Interfaces

### Client Abstractions
- `extensions/html-language-features/client/tsconfig.json` — TypeScript config for client compilation
- `extensions/html-language-features/client/tsconfig.browser.json` — Browser-specific TS config

### Server Abstractions  
- `extensions/html-language-features/server/tsconfig.json` — Server TypeScript config
- `extensions/html-language-features/server/tsconfig.browser.json` — Browser-specific server TS config

### Type Definitions
- `extensions/html-language-features/server/lib/jquery.d.ts` — jQuery type definitions for embedded JS contexts

## Configuration

### Extension Manifest
- `extensions/html-language-features/package.json` — Extension metadata; declares activation events (onLanguage:html, onLanguage:handlebars), client/browser entry points, 30+ HTML settings, contribution schemas for custom data

### Server Package
- `extensions/html-language-features/server/package.json` — Server dependencies (vscode-html-languageservice, vscode-languageserver, vscode-css-languageservice)

### Build Configuration
- `extensions/html-language-features/esbuild.mts` — ESM build script for Node.js client and server; defines entry points and tsconfig paths
- `extensions/html-language-features/esbuild.browser.mts` — ESM build script for browser client and server; includes javaScriptLibsPlugin for inlining TypeScript libs
- `extensions/html-language-features/.npmrc` — NPM config

### Schema
- `extensions/html-language-features/schemas/package.schema.json` — Validation schema for package.json custom data field

### VSCode Config
- `extensions/html-language-features/.vscode/launch.json` — Debug launcher for extension
- `extensions/html-language-features/.vscode/tasks.json` — Build tasks
- `extensions/html-language-features/.vscode/settings.json` — Editor settings
- `extensions/html-language-features/server/.vscode/launch.json` — Server debug launcher
- `extensions/html-language-features/server/.vscode/tasks.json` — Server build tasks

## Documentation

- `extensions/html-language-features/README.md` — Feature overview and extension bundling note
- `extensions/html-language-features/CONTRIBUTING.md` — Development setup, linking vscode-html-languageservice, debugging client/server processes
- `extensions/html-language-features/build/bundleTypeScriptLibraries.js` — Script for bundling TypeScript library definitions

## Examples / Fixtures

- `extensions/html-language-features/icons/html.png` — Extension icon
- `extensions/html-language-features/.vscodeignore` — Package exclusions
- `extensions/html-language-features/cgmanifest.json` — Component governance manifest
- `extensions/html-language-features/server/lib/cgmanifest.json` — Server dependencies governance
- `extensions/html-language-features/package-lock.json` — Client dependency lock
- `extensions/html-language-features/server/package-lock.json` — Server dependency lock
- `extensions/html-language-features/package.nls.json` — Localized strings

## Notable Clusters

- `extensions/html-language-features/client/src/` — 9 files: LSP client bridge layer; connects VS Code extension API to language server via LanguageClient abstraction (IPC or Worker transport); handles custom requests, file system delegation, auto-insertion, and language participant discovery
- `extensions/html-language-features/server/src/modes/` — 9 files: Multi-language mode system; abstracts HTML/CSS/JavaScript parsing and feature provisioning through LanguageMode interface; enables embedded language support via region extraction and document projection
- `extensions/html-language-features/server/src/utils/` — 6 files: Utility layer; provides document caching, validation scheduling, error handling, string/array manipulation, and position conversion
- `extensions/html-language-features/server/src/test/` — 9 test files + fixtures: Comprehensive test coverage for completions, formatting, folding, semantic tokens, diagnostics, and embedded language region detection

---

## Summary

The HTML language features extension exemplifies VS Code's LSP client-server architecture with multi-transport support (Node.js IPC, Web Worker). For a Tauri/Rust port, this structure reveals: (1) **LSP Protocol Adoption** — the extension relies entirely on vscode-languageserver protocol abstractions, making Rust LSP crates (tower-lsp, lsp-types) viable replacements; (2) **Runtime Abstraction** — filesystem operations are abstracted through FileSystemProvider, timer APIs through RuntimeEnvironment, enabling platform-specific Rust implementations; (3) **Multi-Platform Code Paths** — separate node/browser entry points with dual TypeScript configs show dual-platform requirements that Tauri must satisfy (native fs + web platform); (4) **Embedded Language Support** — region extraction and document projection patterns (embeddedSupport.ts, languageModes.ts) form the core complexity of multi-language editing and would require careful porting to Rust's type system; (5) **Dynamic Feature Discovery** — language participants and custom data loaders leverage VS Code's extension registry, requiring Tauri to maintain similar extension integration mechanisms. The server's tight coupling to vscode-html-languageservice and vscode-css-languageservice libraries indicates that Rust ports would either need equivalent language service implementations or FFI bindings to existing services.
