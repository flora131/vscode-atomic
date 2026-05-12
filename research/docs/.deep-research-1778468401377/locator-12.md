# Partition 12: extensions/markdown-language-features — Language Feature API Surface

This partition catalogs the **markdown-language-features** in-tree extension (5,308 LOC across 88 files), which is a primary consumer of VS Code's language-feature registration APIs. It demonstrates the API surface that a Tauri/Rust port must preserve or replace.

## Implementation

### Extension Entry Points
- `extensions/markdown-language-features/src/extension.ts` — Main activation, Electron-only (LSP server process spawning via Node IPC)
- `extensions/markdown-language-features/src/extension.shared.ts` — Shared feature registration (85 lines; language features, commands, preview system)
- `extensions/markdown-language-features/src/extension.browser.ts` — Web/browser adaptation entry point

### Core Client Infrastructure
- `extensions/markdown-language-features/src/client/client.ts` — `MdLanguageClient` wrapping `vscode-languageclient` BaseLanguageClient; sends LSP requests (link target resolution, file rename edits, diagnostics, paste link updates)
- `extensions/markdown-language-features/src/client/workspace.ts` — `VsCodeMdWorkspace` adapter for workspace document operations
- `extensions/markdown-language-features/src/client/fileWatchingManager.ts` — File watching integration
- `extensions/markdown-language-features/src/client/protocol.ts` — LSP protocol extensions (custom requests for markdown-specific operations)
- `extensions/markdown-language-features/src/client/inMemoryDocument.ts` — In-memory document abstraction for virtual files

### Language Features (Using vscode.languages.register*)
- `extensions/markdown-language-features/src/languageFeatures/diagnostics.ts` — **Registers code actions** via `vscode.languages.registerCodeActionsProvider()` for quick fixes (e.g., "add to ignore links")
- `extensions/markdown-language-features/src/languageFeatures/fileReferences.ts` — Find all file references using `vscode.commands.executeCommand('editor.action.showReferences', ...)`
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/dropOrPasteResource.ts` — **Registers document drop & paste edit providers** via `vscode.languages.registerDocumentDropEditProvider()` and `vscode.languages.registerDocumentPasteEditProvider()`
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/pasteUrlProvider.ts` — **Registers document paste provider** for URL-to-link conversion
- `extensions/markdown-language-features/src/languageFeatures/updateLinksOnPaste.ts` — **Registers document paste edit provider** for updating link references
- `extensions/markdown-language-features/src/languageFeatures/linkUpdater.ts` — Link updating on file rename
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/shared.ts` — Shared logic for drop/paste operations
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/smartDropOrPaste.ts` — Smart detection for file/URL drops
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/snippets.ts` — Snippet generation for dropped resources
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/newFilePathGenerator.ts` — Path generation for copied files

### Preview & Rendering System
- `extensions/markdown-language-features/src/preview/previewManager.ts` — **Implements `vscode.WebviewPanelSerializer` & `vscode.CustomTextEditorProvider`**; registers custom editor via `vscode.window.registerCustomEditorProvider()`; manages both dynamic and static previews
- `extensions/markdown-language-features/src/preview/preview.ts` — **Creates webview panels** via `vscode.window.createWebviewPanel()`; message protocol with preview webview
- `extensions/markdown-language-features/src/preview/documentRenderer.ts` — HTML rendering engine for markdown documents
- `extensions/markdown-language-features/src/preview/previewConfig.ts` — Preview configuration management
- `extensions/markdown-language-features/src/preview/security.ts` — Content Security Policy and workspace trust handling
- `extensions/markdown-language-features/src/preview/scrolling.ts` — Editor-preview scroll synchronization
- `extensions/markdown-language-features/src/preview/topmostLineMonitor.ts` — Tracks topmost visible line for sync
- `extensions/markdown-language-features/src/preview/lineDiff.ts` — Line diff computation for preview updates

### Command Management & Utilities
- `extensions/markdown-language-features/src/commandManager.ts` — Centralized command registration via `vscode.commands.registerCommand()`
- `extensions/markdown-language-features/src/commands/index.ts` — Command exports and coordination
- `extensions/markdown-language-features/src/commands/showPreview.ts` — Open preview command
- `extensions/markdown-language-features/src/commands/showSource.ts` — Show source command
- `extensions/markdown-language-features/src/commands/showPreviewSecuritySelector.ts` — Workspace trust selector
- `extensions/markdown-language-features/src/commands/refreshPreview.ts` — Preview refresh
- `extensions/markdown-language-features/src/commands/toggleLock.ts` — Lock preview toggle
- `extensions/markdown-language-features/src/commands/reopenAsPreview.ts` — Reopen as custom editor
- `extensions/markdown-language-features/src/commands/insertResource.ts` — Insert resource (link/image)
- `extensions/markdown-language-features/src/commands/copyImage.ts` — Copy image from preview
- `extensions/markdown-language-features/src/commands/openImage.ts` — Open image from preview
- `extensions/markdown-language-features/src/commands/renderDocument.ts` — Render document command

### Markdown Engine & Extensions
- `extensions/markdown-language-features/src/markdownEngine.ts` — Markdown-it rendering engine
- `extensions/markdown-language-features/src/markdownExtensions.ts` — Extension contribution loading (from package.json)

### Utilities
- `extensions/markdown-language-features/src/util/openDocumentLink.ts` — Opens document links; uses `vscode.commands.executeCommand('vscode.open', uri)`
- `extensions/markdown-language-features/src/util/document.ts` — Document utilities
- `extensions/markdown-language-features/src/util/file.ts` — File path and language ID utilities (markdown language IDs: markdown, prompt, instructions, chatagent, skill)
- `extensions/markdown-language-features/src/util/resources.ts` — Resource URI utilities
- `extensions/markdown-language-features/src/util/url.ts` — URL parsing and conversion
- `extensions/markdown-language-features/src/util/dispose.ts` — Disposable pattern helpers
- `extensions/markdown-language-features/src/util/schemes.ts` — URI scheme utilities
- `extensions/markdown-language-features/src/util/arrays.ts` — Array utilities
- `extensions/markdown-language-features/src/util/async.ts` — Async utilities
- `extensions/markdown-language-features/src/util/cancellation.ts` — Cancellation token utilities
- `extensions/markdown-language-features/src/util/mimes.ts` — MIME type utilities
- `extensions/markdown-language-features/src/util/resourceMap.ts` — Resource-keyed map
- `extensions/markdown-language-features/src/util/uriList.ts` — URI list parsing
- `extensions/markdown-language-features/src/util/uuid.ts` — UUID generation
- `extensions/markdown-language-features/src/util/dom.ts` — DOM utilities for webview
- `extensions/markdown-language-features/src/logging.ts` — Logging abstraction
- `extensions/markdown-language-features/src/telemetryReporter.ts` — Telemetry via `@vscode/extension-telemetry`
- `extensions/markdown-language-features/src/slugify.ts` — Heading slug generation

## Tests

- `extensions/markdown-language-features/src/test/documentLink.test.ts` — Document link provider tests (uses `vscode.executeLinkProvider` command)
- `extensions/markdown-language-features/src/test/engine.test.ts` — Markdown engine tests
- `extensions/markdown-language-features/src/test/copyFile.test.ts` — File copy/paste behavior tests
- `extensions/markdown-language-features/src/test/pasteUrl.test.ts` — URL paste tests
- `extensions/markdown-language-features/src/test/urlToUri.test.ts` — URL-to-URI conversion tests
- `extensions/markdown-language-features/src/test/index.ts` — Test suite entry
- `extensions/markdown-language-features/src/test/util.ts` — Test utilities
- `extensions/markdown-language-features/src/test/nulLogging.ts` — No-op logger for tests
- `extensions/markdown-language-features/src/test/engine.ts` — Test markdown engine instance

## Types / Interfaces

- `extensions/markdown-language-features/types/previewMessaging.d.ts` — Webview ↔ main process message protocol (DiffScrollSyncData, MarkdownPreviewLineChanges, FromWebviewMessage, ToWebviewMessage)
- `extensions/markdown-language-features/src/types/textDocument.ts` — Text document interface
- `extensions/markdown-language-features/src/typings/ref.d.ts` — TypeScript reference declarations

## Configuration

- `extensions/markdown-language-features/package.json` — **Key extension manifest**
  - **enabledApiProposals**: `customEditorDiffs`, `documentDiff` (experimental APIs used)
  - **activationEvents**: `onLanguage:markdown`, `onLanguage:prompt`, `onLanguage:instructions`, `onLanguage:chatagent`, `onLanguage:skill`, `onCommand:markdown.api.render`, `onCommand:markdown.api.reloadPlugins`, `onWebviewPanel:markdown.preview`
  - **contributes.commands**: 13 commands (preview, source, lock, refresh, security, reopen, find references, insert link/image, copy/open image)
  - **contributes.menus**: editor title, explorer, command palette, webview context menus
  - **contributes.keybindings**: Ctrl+K V (preview), Shift+Ctrl+V (toggle)
  - **contributes.configuration**: Markdown editor, validation, preview, advanced settings
  - **contributes.notebookRenderer**: Markdown-it renderer for notebook cells (95+ MIME types supported)
  - **contributes.customEditors**: `vscode.markdown.preview.editor` (custom editor for .md files)

- `extensions/markdown-language-features/tsconfig.json` — TypeScript compilation config
- `extensions/markdown-language-features/tsconfig.browser.json` — Browser/web build config
- `extensions/markdown-language-features/esbuild.mts` — Extension build (Electron)
- `extensions/markdown-language-features/esbuild.webview.mts` — Webview build
- `extensions/markdown-language-features/esbuild.browser.mts` — Browser build
- `extensions/markdown-language-features/esbuild.notebook.mts` — Notebook renderer build
- `extensions/markdown-language-features/schemas/package.schema.json` — Package schema validation

## Examples / Fixtures

- `extensions/markdown-language-features/notebook/index.ts` — Notebook renderer entry (15 lines)
- `extensions/markdown-language-features/notebook/tsconfig.json` — Notebook build config
- `extensions/markdown-language-features/preview-src/index.ts` — Webview script entry (loads preview app)
- `extensions/markdown-language-features/preview-src/csp.ts` — CSP meta tag injection
- `extensions/markdown-language-features/preview-src/messaging.ts` — Webview message protocol client
- `extensions/markdown-language-features/preview-src/events.ts` — Webview event handlers
- `extensions/markdown-language-features/preview-src/loading.ts` — Loading state management
- `extensions/markdown-language-features/preview-src/scroll-sync.ts` — Editor-preview scroll sync (webview side)
- `extensions/markdown-language-features/preview-src/diffScrollSync.ts` — Diff view scroll sync
- `extensions/markdown-language-features/preview-src/activeLineMarker.ts` — Active line highlighting
- `extensions/markdown-language-features/preview-src/strings.ts` — Localization string handler
- `extensions/markdown-language-features/preview-src/settings.ts` — Preview settings from webview
- `extensions/markdown-language-features/preview-src/pre.ts` — Pre-rendering utilities
- `extensions/markdown-language-features/test-workspace/` — Fixture markdown files for testing

## Documentation

- `extensions/markdown-language-features/README.md` — Extension readme
- `extensions/markdown-language-features/package.nls.json` — Localization strings

## Notable Clusters

### `extensions/markdown-language-features/src/languageFeatures/` — 6 files (495 LOC)
Implements language features via `vscode.languages.register*()` APIs:
- Code actions (diagnostics quick fixes)
- Document drop/paste edit providers
- File reference finding
- Link updating on file operations

### `extensions/markdown-language-features/src/preview/` — 8 files (1,400+ LOC)
Implements webview-based markdown preview system:
- Custom text editor provider (`vscode.CustomTextEditorProvider`)
- Webview panel creation and lifecycle
- Preview-editor synchronization (scroll, selection, line tracking)
- Content security policy and workspace trust
- Document rendering and diff computation

### `extensions/markdown-language-features/src/commands/` — 10 files (250+ LOC)
Command implementations registered via `vscode.commands.registerCommand()`:
- Preview lifecycle (show, hide, lock, refresh)
- Security and trust UI
- Document navigation (reopen as preview/source)
- Resource insertion and inspection

### `extensions/markdown-language-features/src/client/` — 5 files (300+ LOC)
LSP client infrastructure:
- Wraps `vscode-languageclient` for node process
- Workspace document tracking
- Custom LSP protocol extensions
- File watching and virtual document support

### `extensions/markdown-language-features/src/util/` — 14 files (500+ LOC)
Foundational utilities supporting all layers:
- Document/URI/file handling
- Disposable pattern enforcement
- Async/cancellation helpers
- Telemetry and logging abstractions

### `extensions/markdown-language-features/preview-src/` — 13 files (400+ LOC)
Webview-side JavaScript/TypeScript:
- Message protocol for IPC with extension
- DOM manipulation and rendering
- Event handling and scroll synchronization
- Settings and localization management

---

## API Surface Summary for Tauri/Rust Port

**Critical VS Code APIs this extension demonstrates:**

1. **Language Feature Registration** (must be preserved/ported):
   - `vscode.languages.registerCodeActionsProvider()` — for diagnostics quick fixes
   - `vscode.languages.registerDocumentDropEditProvider()` — for drag-drop editing
   - `vscode.languages.registerDocumentPasteEditProvider()` — for paste intelligence

2. **Webview System** (UI layer challenge):
   - `vscode.window.createWebviewPanel()` — opens preview in a panel
   - `vscode.window.registerCustomEditorProvider()` — registers custom editor (markdown preview as editor)
   - `vscode.WebviewPanelSerializer` — persistence/restore
   - Webview ↔ extension message protocol (postMessage, onDidReceiveMessage)

3. **Command System**:
   - `vscode.commands.registerCommand()` — custom command registration
   - `vscode.commands.executeCommand()` — inter-extension command invocation

4. **Workspace/Editor APIs**:
   - `vscode.workspace.onDidChangeConfiguration()` — config change listening
   - `vscode.window.tabGroups.activeTabGroup` — editor group detection
   - Document selection and URI handling

5. **LSP Integration**:
   - `vscode-languageclient` for spawning/communicating with language server
   - Custom LSP request/response types (protocol.ts)

6. **Extension Activation Events**:
   - `onLanguage:markdown` and variant language IDs
   - `onCommand:*` for lazy activation
   - `onWebviewPanel:*` for preview restore

**API Proposal Surface** (experimental features in use):
   - `customEditorDiffs` — diff preview for custom editors
   - `documentDiff` — line-level diff tracking

This extension is representative of **API-consuming extensions** — it does not implement core IDE functionality but demonstrates how a port must expose stable, predictable language feature and UI extension points for third-party integrations.

