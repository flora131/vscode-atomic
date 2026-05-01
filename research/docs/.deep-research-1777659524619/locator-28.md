# CSS Language Features Extension - File Locator

**Partition:** 28/79 | **Scope:** `extensions/css-language-features/` (30 files, ~2,261 LOC)

**Research Question:** What would it take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust?

**Relevance:** This partition documents the CSS LSP client/server architecture as a reference implementation. Any Tauri/Rust port must preserve the LSP client wiring contract for built-in language extensions.

---

## Implementation

### Client-Side (Extension Host Entry Points)

**Platform-specific activation:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/node/cssClientMain.ts` — Node.js environment; instantiates `LanguageClient` with IPC transport; registers drop-or-paste resource support
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/browser/cssClientMain.ts` — Browser/web environment; spawns Worker thread; uses Worker-based transport instead of IPC

**Core LSP Client Bootstrap:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/cssClient.ts` — Shared LSP client initialization logic (both platforms). Exports `startClient()` function and `LanguageClientConstructor` type. Registers completion providers, formatter registration, custom data change notifications, and document formatting support.

**Request/Response Contracts:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/requests.ts` — Defines LSP filesystem request types: `FsContentRequest`, `FsStatRequest`, `FsReadDirRequest`. Implements bidirectional request handling (client serves FS requests to server).

**Feature Implementations:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/dropOrPaste/dropOrPasteResource.ts` — Document drop/paste edit provider for CSS URLs (relative path handling, snippet generation)
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/dropOrPaste/shared.ts` — Shared utilities for drop/paste (mime types, schemes, document directory resolution)
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/dropOrPaste/uriList.ts` — URI list parsing from clipboard/drag data
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/customData.ts` — Custom CSS data loader; monitors workspace config changes and extension contributions
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/node/nodeFs.ts` — Node.js filesystem request service implementation

### Server-Side (Language Server)

**Core Server Logic:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/cssServer.ts` — Main LSP server. Implements initialization protocol, handler registration for LSP methods (completion, hover, document symbols, definition, highlights, links, references, code actions, color, formatting, folding ranges, selection ranges, diagnostics). Manages stylesheet caching, settings management, and data provider lifecycle.

**Platform-specific Entry Points:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/node/cssServerMain.ts` — Node.js server entry point; creates IPC connection
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/node/cssServerNodeMain.ts` — Wrapper for ESM build
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/browser/cssServerMain.ts` — Browser worker entry point; uses BrowserMessageReader/Writer
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/browser/cssServerWorkerMain.ts` — Worker thread variant

**Request/Response Contracts:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/requests.ts` — Server-side filesystem request handler; dispatches to built-in handlers (file, http/https) or delegates to client via LSP protocol

**Supporting Utilities:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/languageModelCache.ts` — LRU cache for parsed CSS/SCSS/LESS stylesheets with time-based eviction
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/customData.ts` — Server-side custom CSS data provider fetching
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/node/nodeFs.ts` — Node.js filesystem provider
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/utils/strings.ts` — String utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/utils/documentContext.ts` — Document context builder for path resolution
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/utils/validation.ts` — Diagnostic push/pull support (both LSP 3.16+ pull and push semantics)
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/utils/runner.ts` — Safe async execution wrapper with error handling

---

## Tests

### Server Tests (Node.js)
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/test/completion.test.ts` — Completion feature tests using node:test framework; validates CSS url() path completion, fixture-based assertions
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/test/links.test.ts` — Document link tests

### Test Fixtures
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/` — Path completion test data (HTML, CSS, SCSS, JS files; nested directories)
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/linksTestFixtures/` — Links test fixtures

### Test Runner
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/index.js` — Node.js native test runner (node:test module); outputs spec/JUnit formats

---

## Types / Interfaces

### Client Types
- `LanguageClientConstructor` (exported from `cssClient.ts`) — Function signature for platform-specific language client creation
- `Runtime` (exported from `cssClient.ts`) — Abstraction for platform-specific runtime: TextDecoder and optional RequestService (filesystem)

### Server Types
- `Settings` (exported from `cssServer.ts`) — Nested language settings for css/scss/less
- `RuntimeEnvironment` (exported from `cssServer.ts`) — Server-side runtime: file/http RequestService, timer (setImmediate/setTimeout)

### Shared Request/Response Types
- `FsContentRequest.type` — RequestType<{uri, encoding?}, string>
- `FsStatRequest.type` — RequestType<string, FileStat>
- `FsReadDirRequest.type` — RequestType<string, [string, FileType][]>
- `FileStat` interface — {type, ctime, mtime, size}
- `FileType` enum — Unknown, File, Directory, SymbolicLink
- `RequestService` interface — {getContent, stat, readDirectory}
- `ItemDescription` interface (in tests) — {label, resultText?}

### Cache Types
- `LanguageModelCache<T>` interface — {get, onDocumentRemoved, dispose}

---

## Configuration

### Extension Manifest
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/package.json` — Defines 3 language configurations (css, scss, less) with ~40 lint rules, format settings, completion options. Activation events: onLanguage:css/scss/less. Entry points: `client/out/node/cssClientMain` (node), `client/dist/browser/cssClientMain` (browser)

### Language Server Manifest
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/package.json` — Declares vscode-css-languageservice and vscode-languageserver dependencies; provides ESM module type

### Build Configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/esbuild.mts` — esbuild configuration for dual platform builds (node client/server). Uses esm format for server, builds with vscode-uri external
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/esbuild.browser.mts` — Browser build configuration (if present)

### TypeScript Configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/tsconfig.json` — Client TypeScript settings
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/tsconfig.browser.json` — Browser client TypeScript settings
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/tsconfig.json` — Server TypeScript settings
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/tsconfig.browser.json` — Browser server TypeScript settings

### VSCode Dev Workspace
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/.vscode/launch.json` — Debug launch configurations
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/.vscode/tasks.json` — Build/compile tasks
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/.vscode/settings.json` — Workspace settings

### Metadata
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/.npmrc` — NPM configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/.vscodeignore` — Files excluded from extension packaging
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/package-lock.json` — Locked dependencies
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/package-lock.json` — Server dependencies locked
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/package.nls.json` — Localization strings

### JSON Schema
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/schemas/package.schema.json` — JSON validation schema for package.json

---

## Examples / Fixtures

### Test Fixtures (Path Completion)
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/about/about.html`
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/about/about.css`
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/scss/_foo.scss`
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/scss/main.scss`
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/src/test.js`
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/src/data/foo.asar`
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/src/feature.js`
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/index.html`

### Asset
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/icons/css.png` — Extension icon

---

## Documentation

- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/README.md` — Basic readme (bundled extension notice, links to main docs)
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/CONTRIBUTING.md` — Contribution guidelines

---

## Notable Clusters

**LSP Client Platform Abstraction:**
The extension implements dual-platform LSP client activation (Node.js and Browser). The core `startClient()` function is platform-agnostic and accepts a `LanguageClientConstructor` callback, allowing `cssClientMain.ts` (node) and `cssClientMain.ts` (browser) to provide platform-specific LanguageClient instances with appropriate transport (IPC vs. Worker).

**Bidirectional Request Contract:**
The filesystem request service establishes a bidirectional request pattern: the client exposes `FsContentRequest`, `FsStatRequest`, and `FsReadDirRequest` handlers, while the server uses these to read import paths, stylesheets, and custom data files. This allows the server to remain platform-agnostic while delegating filesystem access to the client.

**Stylesheet Caching Layer:**
The `LanguageModelCache<Stylesheet>` maintains an LRU cache of parsed stylesheets (max 10 entries, 60-second eviction). This optimization prevents reparsing unchanged documents across multiple language service operations (completion, hover, validation).

**Configuration Synchronization:**
The client monitors both workspace configuration (`css.customData`, `css.format.*`, `css.lint.*`) and extension contribution points. Changes trigger server notifications via `CustomDataChangedNotification`, allowing dynamic hot-reload of custom CSS property definitions without server restart.

**Diagnostics Dual-Mode Support:**
The server registers either pull-based (LSP 3.16+) or push-based diagnostic support depending on client capabilities, allowing graceful fallback to older clients while supporting modern diagnostic pull semantics.

**Multi-Format Language Server:**
The same server (cssServer.ts) handles CSS, SCSS, and LESS languages. Document language ID routes to the appropriate language service (getCSSLanguageService, getSCSSLanguageService, getLESSLanguageService).

---

## Porting Implications for Tauri/Rust

**Critical LSP Client Wiring Contract:**
- The CSS extension uses `vscode-languageclient/node` (TypeScript) which wraps LSP protocol over IPC. A Tauri/Rust port must provide equivalent LSP client scaffolding for built-in extensions, exposing the same initialization, activation, and request/response interfaces.

**Platform Abstraction Patterns:**
- Dual entry points (Node.js + Browser) demonstrate how VS Code abstracts platforms. Tauri would require a similar abstraction layer for browser/tauri-specific transport (likely WebSocket or message-passing).

**Runtime Dependencies:**
- The TextDecoder abstraction and filesystem RequestService are platform bridges. Tauri would need to expose equivalent APIs to the LSP client layer.

**Filesystem and IPC:**
- Current: IPC for Node.js process communication, Worker for browser. Tauri: Would require WebSocket, shared memory, or Tauri command protocol for client-server communication within same process or separate threads.

**Configuration/Settings Propagation:**
- Currently: LSP ConfigurationRequest and workspace change events. Tauri/Rust port must preserve this two-way settings sync.

---

## Summary

The CSS language features extension (30 files, ~2,261 LOC) exemplifies VS Code's LSP client/server architecture. It uses a clean abstraction where the core `startClient()` function accepts a `LanguageClientConstructor` callback, enabling both Node.js (IPC transport) and Browser (Worker transport) implementations from a single codebase. The server manages three CSS-variant languages (CSS, SCSS, LESS) through a delegating language service pattern and implements bidirectional filesystem requests to remain platform-agnostic. Configuration and diagnostics support LSP 3.16+ modern semantics with fallback support. Any Tauri/Rust port must preserve the LSP client wiring contract—specifically the initialization sequence, request/response types, and the ability for built-in extensions to activate and communicate with language servers via standardized LSP methods.

