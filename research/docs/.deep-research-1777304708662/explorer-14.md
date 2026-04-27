# Partition 14 of 79 — Findings

## Scope
`extensions/markdown-language-features/` (85 files, 8,661 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locations: VS Code Markdown Language Features Extension

## Research Context
This partition investigates the markdown-language-features extension as a reference implementation for understanding VS Code's core IDE functionality integration points. The key research focus is on webview panel creation (`createWebviewPanel`), language server protocol usage, and extension activation patterns—all critical architectural patterns that would require porting when moving from Electron/TypeScript to Tauri/Rust.

## Implementation

### Extension Entry Points
- `extensions/markdown-language-features/src/extension.ts` — Main extension entry point, activate/deactivate functions
- `extensions/markdown-language-features/src/extension.shared.ts` — Shared extension logic, handles vscode API registration
- `extensions/markdown-language-features/src/extension.browser.ts` — Browser-specific extension logic

### Webview Panel Integration (Critical for Tauri Porting)
- `extensions/markdown-language-features/src/preview/preview.ts` — **Central webview panel management**, implements `vscode.WebviewPanel`, handles message posting and reception between extension and webview
- `extensions/markdown-language-features/src/preview/previewManager.ts` — Manages webview panel serialization and lifecycle, implements `WebviewPanelSerializer` and `CustomTextEditorProvider`

### Preview Rendering
- `extensions/markdown-language-features/src/preview/documentRenderer.ts` — Converts markdown documents to HTML for rendering
- `extensions/markdown-language-features/src/preview/scrolling.ts` — Synchronizes scroll position between editor and preview
- `extensions/markdown-language-features/src/preview/topmostLineMonitor.ts` — Tracks topmost visible line for scroll synchronization
- `extensions/markdown-language-features/src/preview/security.ts` — Content security policy implementation for webviews
- `extensions/markdown-language-features/src/preview/previewConfig.ts` — Configuration management for preview behavior

### Language Server Integration
- `extensions/markdown-language-features/src/client/client.ts` — Language server client setup, communicates with vscode-markdown-languageserver
- `extensions/markdown-language-features/src/client/workspace.ts` — Workspace document management for language server
- `extensions/markdown-language-features/src/client/protocol.ts` — Protocol definitions for language server communication
- `extensions/markdown-language-features/src/client/fileWatchingManager.ts` — File watching for workspace changes
- `extensions/markdown-language-features/src/client/inMemoryDocument.ts` — In-memory document representation for language server

### Language Features
- `extensions/markdown-language-features/src/languageFeatures/diagnostics.ts` — Diagnostic reporting (linting)
- `extensions/markdown-language-features/src/languageFeatures/fileReferences.ts` — File reference finding
- `extensions/markdown-language-features/src/languageFeatures/linkUpdater.ts` — Auto-updating markdown links on file operations
- `extensions/markdown-language-features/src/languageFeatures/updateLinksOnPaste.ts` — Link updating during paste operations
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/copyFiles.ts` — File copy/drop handling
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/dropOrPasteResource.ts` — Drop and paste resource insertion
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/newFilePathGenerator.ts` — Generates new file paths for copied resources
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/pasteUrlProvider.ts` — URL paste handling
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/smartDropOrPaste.ts` — Smart insertion logic
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/shared.ts` — Shared copy/paste utilities
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/snippets.ts` — Snippet template handling

### Commands
- `extensions/markdown-language-features/src/commands/showPreview.ts` — Command handler for opening markdown preview
- `extensions/markdown-language-features/src/commands/showPreviewSecuritySelector.ts` — Security selector command
- `extensions/markdown-language-features/src/commands/showSource.ts` — Show source command
- `extensions/markdown-language-features/src/commands/refreshPreview.ts` — Refresh preview command
- `extensions/markdown-language-features/src/commands/toggleLock.ts` — Toggle preview lock state
- `extensions/markdown-language-features/src/commands/renderDocument.ts` — Render markdown document command
- `extensions/markdown-language-features/src/commands/reloadPlugins.ts` — Reload markdown plugins
- `extensions/markdown-language-features/src/commands/copyImage.ts` — Copy image command
- `extensions/markdown-language-features/src/commands/openImage.ts` — Open image command
- `extensions/markdown-language-features/src/commands/insertResource.ts` — Insert resource command
- `extensions/markdown-language-features/src/commands/index.ts` — Command registration index

### Core Utilities
- `extensions/markdown-language-features/src/commandManager.ts` — Command registration and dispatch management
- `extensions/markdown-language-features/src/markdownEngine.ts` — Markdown rendering engine setup
- `extensions/markdown-language-features/src/markdownExtensions.ts` — Extension loading for markdown
- `extensions/markdown-language-features/src/logging.ts` — Logging infrastructure
- `extensions/markdown-language-features/src/telemetryReporter.ts` — Telemetry reporting
- `extensions/markdown-language-features/src/slugify.ts` — URL slug generation

### Utility Functions
- `extensions/markdown-language-features/src/util/openDocumentLink.ts` — Opens document links via vscode.workspace APIs
- `extensions/markdown-language-features/src/util/file.ts` — File operations
- `extensions/markdown-language-features/src/util/document.ts` — Document utilities
- `extensions/markdown-language-features/src/util/url.ts` — URL handling
- `extensions/markdown-language-features/src/util/uriList.ts` — URI list utilities
- `extensions/markdown-language-features/src/util/resourceMap.ts` — Resource mapping
- `extensions/markdown-language-features/src/util/dom.ts` — DOM utilities for webview content
- `extensions/markdown-language-features/src/util/resources.ts` — Resource URI handling with asAbsolutePath
- `extensions/markdown-language-features/src/util/schemes.ts` — URI scheme utilities
- `extensions/markdown-language-features/src/util/mimes.ts` — MIME type utilities
- `extensions/markdown-language-features/src/util/arrays.ts` — Array utilities
- `extensions/markdown-language-features/src/util/async.ts` — Async utilities
- `extensions/markdown-language-features/src/util/cancellation.ts` — Cancellation token utilities
- `extensions/markdown-language-features/src/util/dispose.ts` — Disposable utilities

### Webview Rendering (Browser Context)
- `extensions/markdown-language-features/preview-src/index.ts` — **Main webview entry point**, initializes preview with vscode API messaging
- `extensions/markdown-language-features/preview-src/messaging.ts` — **Message protocol handler**, receives postMessage from extension, sends responses
- `extensions/markdown-language-features/preview-src/events.ts` — Event handling in webview context
- `extensions/markdown-language-features/preview-src/activeLineMarker.ts` — Active line highlighting in preview
- `extensions/markdown-language-features/preview-src/scroll-sync.ts` — Scroll synchronization logic
- `extensions/markdown-language-features/preview-src/loading.ts` — Loading state management
- `extensions/markdown-language-features/preview-src/settings.ts` — Settings handling in webview
- `extensions/markdown-language-features/preview-src/strings.ts` — Localized strings in webview
- `extensions/markdown-language-features/preview-src/csp.ts` — Content security policy meta tag injection
- `extensions/markdown-language-features/preview-src/pre.ts` — Pre-rendering utilities

### Notebook Support
- `extensions/markdown-language-features/notebook/index.ts` — Notebook markdown renderer implementation

## Tests

- `extensions/markdown-language-features/src/test/copyFile.test.ts` — Tests for file copy functionality
- `extensions/markdown-language-features/src/test/documentLink.test.ts` — Tests for document link handling
- `extensions/markdown-language-features/src/test/engine.test.ts` — Tests for markdown engine
- `extensions/markdown-language-features/src/test/pasteUrl.test.ts` — Tests for URL paste handling
- `extensions/markdown-language-features/src/test/urlToUri.test.ts` — Tests for URL to URI conversion
- `extensions/markdown-language-features/src/test/engine.ts` — Test engine helper
- `extensions/markdown-language-features/src/test/index.ts` — Test entry point
- `extensions/markdown-language-features/src/test/nulLogging.ts` — No-op logger for tests
- `extensions/markdown-language-features/src/test/util.ts` — Test utilities

## Types / Interfaces

- `extensions/markdown-language-features/types/previewMessaging.d.ts` — **Message protocol type definitions** for extension-webview communication, defines all message types and interfaces
- `extensions/markdown-language-features/src/types/textDocument.ts` — Text document type definitions for language server

## Configuration

- `extensions/markdown-language-features/package.json` — Extension manifest with full vscode API usage: activation events, webview panel registration (`markdown.preview`), commands, menus, configuration schema, customEditors support
- `extensions/markdown-language-features/package.nls.json` — Localized strings/labels
- `extensions/markdown-language-features/tsconfig.json` — TypeScript configuration for main extension
- `extensions/markdown-language-features/tsconfig.browser.json` — TypeScript configuration for browser/webview builds
- `extensions/markdown-language-features/preview-src/tsconfig.json` — TypeScript configuration for preview webview source
- `extensions/markdown-language-features/notebook/tsconfig.json` — TypeScript configuration for notebook renderer
- `extensions/markdown-language-features/schemas/package.schema.json` — JSON schema for package.json validation
- `extensions/markdown-language-features/esbuild.mts` — Build configuration for extension
- `extensions/markdown-language-features/esbuild.browser.mts` — Build configuration for browser bundle
- `extensions/markdown-language-features/esbuild.notebook.mts` — Build configuration for notebook renderer

## Documentation

- `extensions/markdown-language-features/README.md` — Extension documentation

## Media / Assets

- `extensions/markdown-language-features/media/markdown.css` — Preview styling for rendered markdown
- `extensions/markdown-language-features/media/highlight.css` — Syntax highlighting stylesheet
- `extensions/markdown-language-features/media/preview-light.svg` — Light theme preview icon
- `extensions/markdown-language-features/media/preview-dark.svg` — Dark theme preview icon

## Notable Clusters

### Preview System (`extensions/markdown-language-features/src/preview/`)
- 7 files managing webview panel lifecycle, rendering, scrolling, security, and configuration
- **Critical for Tauri porting**: Demonstrates how Electron webview panels map to a message-based architecture—Tauri would require replacing `vscode.WebviewPanel` APIs with Tauri window/webview equivalents

### Webview Communication Layer (`extensions/markdown-language-features/preview-src/`)
- 10 files implementing the browser-side of the extension-webview protocol
- Demonstrates bidirectional message passing via `postMessage`/`onDidReceiveMessage`
- Security handling (CSP) and settings synchronization

### Language Server Integration (`extensions/markdown-language-features/src/client/`)
- 5 files wrapping the vscode-languageclient library
- Demonstrates how language servers integrate with VS Code's diagnostic, completion, and reference APIs
- **For Tauri porting**: LSP protocol itself is language-agnostic, but client library would need reimplementation

### Copy/Paste & Drag-Drop Features (`extensions/markdown-language-features/src/languageFeatures/copyFiles/`)
- 7 files handling file operations triggered by editor interactions
- Uses vscode.workspace APIs for file operations and URI handling
- **For Tauri porting**: Would need Tauri-native equivalents for workspace file system access

### Language Features (`extensions/markdown-language-features/src/languageFeatures/`)
- 6 files implementing diagnostics, link validation, link updating
- Demonstrates integration with vscode language features APIs (registerCodeActionProvider, etc.)

### Commands (`extensions/markdown-language-features/src/commands/`)
- 11 files for command implementations
- Shows pattern of vscode.commands.registerCommand with context-dependent handlers
- Examples include menu integration, keybinding support, when-clause evaluation

## Summary

The markdown-language-features extension is a comprehensive reference for understanding VS Code's IDE functionality integration patterns. It demonstrates critical architectural concepts for Tauri porting:

1. **Webview Integration**: The preview system (7 files) shows how Electron's webview panels are managed via message passing—Tauri's window/webview system would require complete architectural replacement of the `vscode.WebviewPanel` API.

2. **Language Server Protocol**: The client layer (5 files) shows LSP integration patterns using vscode-languageclient, which would need a Rust-native LSP client library in a Tauri port.

3. **Extension API Surface**: Package.json reveals heavy usage of vscode APIs: activation events, command registration, menu contributions, workspace operations, file watching, and URI handling—all would require Tauri equivalents.

4. **Message-Based IPC**: Preview-src files (10 files) demonstrate the bidirectional message protocol between extension and webview—this pattern is directly applicable to Tauri but requires reimplementation of the security model and message format.

5. **File System Access**: Multiple files use vscode.workspace and URI handling for file operations—Tauri would need to provide equivalent APIs or a custom file system abstraction layer.

6. **Lifecycle Management**: The extension.ts entry point shows standard extension activation/deactivation patterns that Tauri would need to replicate through its plugin system.

The 85-file extension spans 8,661 lines of TypeScript with clear separation between extension logic (src/), webview/UI logic (preview-src/), and build infrastructure, providing a detailed blueprint of how a complex IDE feature translates between frameworks.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
# Analyzer-14: extensions/markdown-language-features/

## Research Question
Port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

---

### Files Analysed

| # | File | LOC (approx) |
|---|------|-------------|
| 1 | `extensions/markdown-language-features/src/extension.ts` | 56 |
| 2 | `extensions/markdown-language-features/src/extension.shared.ts` | 68 |
| 3 | `extensions/markdown-language-features/src/preview/preview.ts` | 858 |
| 4 | `extensions/markdown-language-features/src/preview/documentRenderer.ts` | 263 |
| 5 | `extensions/markdown-language-features/src/preview/previewManager.ts` | 331 |
| 6 | `extensions/markdown-language-features/src/client/client.ts` | 180 |
| 7 | `extensions/markdown-language-features/src/client/protocol.ts` | 41 |
| 8 | `extensions/markdown-language-features/src/languageFeatures/diagnostics.ts` | 134 |
| 9 | `extensions/markdown-language-features/preview-src/index.ts` | 464 |
| 10 | `extensions/markdown-language-features/preview-src/messaging.ts` | 33 |
| 11 | `extensions/markdown-language-features/types/previewMessaging.d.ts` | 88 |
| 12 | `extensions/markdown-language-features/src/markdownEngine.ts` | 455 |

---

### Per-File Notes

---

#### 1. `src/extension.ts` — Node.js Entry Point & Activation

**Role:** The main extension activation function for the Node.js (non-browser) host. It bootstraps the language server child process via IPC and then delegates all shared registration to `extension.shared.ts`.

**Key symbols:**

- `activate(context)` — `extension.ts:15`. Async export. Creates `MarkdownItEngine`, calls `startServer`, then calls `activateShared`.
- `startServer(context, parser)` — `extension.ts:29`. Selects the server module path based on `context.extension.packageJSON.main`: debug builds use the `out/` variant; production builds use `dist/serverWorkerMain`. Sets `process.env['VSCODE_L10N_BUNDLE_LOCATION']` to pass the l10n bundle path. Constructs a `ServerOptions` object using `TransportKind.ipc` (both run and debug variants). Invokes `startClient()` from `./client/client.ts` passing a factory that calls `new LanguageClient(...)`.

**Control flow:**
```
activate()
  → getMarkdownExtensionContributions()
  → new MarkdownItEngine(contributions, githubSlugifier, logger)
  → startServer() → startClient() [spawns IPC worker]
  → activateShared(context, client, engine, logger, contributions)
```

**Dependencies on VS Code APIs:** `vscode.ExtensionContext`, `vscode.l10n`, `vscode-languageclient/node.LanguageClient`, `TransportKind`.

**Porting notes:** `TransportKind.ipc` relies on Node.js `child_process` IPC. The debug port selection (`7000 + Math.random() * 999`) at `extension.ts:41` is Node-specific. The `VSCODE_L10N_BUNDLE_LOCATION` env variable injection at `extension.ts:51` is a process-global side-effect.

---

#### 2. `src/extension.shared.ts` — Shared Activation (Node + Browser)

**Role:** Handles all feature registration that is platform-agnostic between the Node.js and browser builds. Instantiates `MdDocumentRenderer`, `MarkdownPreviewManager`, link opener, CSP arbiter, telemetry, and command manager. Registers all language feature providers.

**Key symbols:**

- `activateShared(context, client, engine, logger, contributions)` — `extension.shared.ts:26`. Called from both `extension.ts` (Node) and `extension.browser.ts` (web). Instantiates all services and registers them as disposables.
- `registerMarkdownLanguageFeatures(client, commandManager, parser)` — `extension.shared.ts:53`. Returns a composite `vscode.Disposable` assembling six sub-registrations:
  - `registerDiagnosticSupport` — code actions and status bar for link validation
  - `registerFindFileReferenceSupport` — workspace-level file reference search
  - `registerResourceDropOrPasteSupport` — drag/drop and paste of resources
  - `registerPasteUrlSupport` — URL paste provider
  - `registerUpdateLinksOnRename` — link fixup on file rename
  - `registerUpdatePastedLinks` — link update on paste

**Data flow:**
- `ExtensionContentSecurityPolicyArbiter` wraps `context.globalState` and `context.workspaceState` to persist per-resource CSP decisions.
- `MdDocumentRenderer` is created with `engine`, `cspArbiter`, and `contributions`.
- `MarkdownPreviewManager` receives the renderer and manages panel lifecycles.
- A `vscode.workspace.onDidChangeConfiguration` handler at `extension.shared.ts:48` calls `previewManager.updateConfiguration()`.

---

#### 3. `src/preview/preview.ts` — Core Preview Panel Logic

**Role:** Defines three classes — `MarkdownPreview` (private inner class managing a single `vscode.WebviewPanel`), `StaticMarkdownPreview` (custom text editor mode), and `DynamicMarkdownPreview` (side-by-side preview following the active editor). This file is the densest single unit of VS Code API surface in the extension.

**Key symbols:**

- `MarkdownPreview` (class) — `preview.ts:43`. Holds `#webviewPanel: vscode.WebviewPanel` at `preview.ts:53`. All communication to/from the webview passes through this panel.
  - `refresh(forceUpdate)` — `preview.ts:208`. Debounces updates using `#throttleTimer` with a 300 ms delay; the very first call bypasses the timer.
  - `#updatePreview(forceUpdate)` — `preview.ts:255`. Opens the document with `vscode.workspace.openTextDocument`, compares `PreviewDocumentVersion`, calls either `renderDocument` (full HTML reload) or `renderBody` (content update only), then updates `#webviewPanel.webview.html` or sends an `updateContent` postMessage.
  - `#onDidScrollPreview(line)` — `preview.ts:308`. Receives `revealLine` messages from the webview, fires `#onScrollEmitter`, and conditionally calls `scrollEditorToLine`.
  - `#onDidClickPreview(line)` — `preview.ts:332`. Handles `didClick` from webview, executes `markdown.showSource` command, sets editor selection.
  - `#onDidClickPreviewLink(href)` — `preview.ts:432`. Resolves links via `MdLinkOpener.resolveDocumentLink`; if in-preview mode and the target is a markdown file, delegates to the preview delegate's `openPreviewLinkToMarkdownFile`.
  - `postMessage(msg)` — `preview.ts:226`. Thin guard around `#webviewPanel.webview.postMessage`.
  - `#getWebviewOptions()` — `preview.ts:408`. Sets `enableScripts: true`, `enableForms: false`, and restricts `localResourceRoots` to workspace folders and extension contribution roots.

- `PreviewDocumentVersion` — `preview.ts:21`. Wraps `vscode.Uri` and `document.version` for equality testing to skip redundant refreshes.

- `StaticMarkdownPreview` — `preview.ts:495`. Registered as a custom text editor via `vscode.window.registerCustomEditorProvider` under view type `"vscode.markdown.preview.editor"`. Revived via `revive()` at `preview.ts:499`. Delegates scroll tracking to `TopmostLineMonitor`.

- `DynamicMarkdownPreview` — `preview.ts:619`. Registered as a webview panel serializer under view type `"markdown.preview"`. Created via `DynamicMarkdownPreview.create()` at `preview.ts:645` which calls `vscode.window.createWebviewPanel`. Tracks `#locked` state for pinning. Responds to `vscode.window.onDidChangeActiveTextEditor` at `preview.ts:723` to switch displayed resource when the active editor changes (if not locked).
  - `update(newResource, scrollLocation)` — `preview.ts:782`. Disposes the inner `MarkdownPreview` and creates a new one for the new resource.
  - `toggleLock()` — `preview.ts:802`. Flips `#locked`, updates panel title.

**Webview message dispatch (inbound):** `MarkdownPreview` registers `#webviewPanel.webview.onDidReceiveMessage` at `preview.ts:141`. Dispatched on `e.type`:
- `cacheImageSizes` → stores image dimension metadata
- `revealLine` → `#onDidScrollPreview`
- `didClick` → `#onDidClickPreview`
- `openLink` → `#onDidClickPreviewLink`
- `showPreviewSecuritySelector` → executes command
- `previewStyleLoadError` → `vscode.window.showWarningMessage`

**File watcher management:** `#updateImageWatchers(srcs)` at `preview.ts:385` maintains a `Map<string, vscode.FileSystemWatcher>` keyed by image `src` URLs. Watchers are created for local (non-http/https/data) image sources and trigger `refresh(true)` on change.

---

#### 4. `src/preview/documentRenderer.ts` — Markdown-to-HTML Rendering

**Role:** `MdDocumentRenderer` is the bridge between the `MarkdownItEngine` and the webview. It produces complete HTML page strings (for full reloads) and partial body HTML (for incremental updates).

**Key symbols:**

- `MdDocumentRenderer` (class) — `documentRenderer.ts:43`. Constructor at `documentRenderer.ts:51` accepts engine, context, cspArbiter, contributionProvider, logger. Sets `iconPath` pointing to `media/preview-dark.svg` and `media/preview-light.svg`.
- `renderDocument(markdownDocument, resourceProvider, previewConfigurations, initialLine, selectedLine, state, imageInfo, token)` — `documentRenderer.ts:71`. Produces the full `<!DOCTYPE html>` page. Steps:
  1. Loads config via `previewConfigurations.loadAndCacheConfiguration(sourceUri)` at `documentRenderer.ts:82`.
  2. Builds `initialData` object at `documentRenderer.ts:83` containing source URI, fragment, scroll settings, CSP warning flag, webview resource root — serialized into `data-settings` meta attribute.
  3. Generates a nonce via `generateUuid()` at `documentRenderer.ts:98`.
  4. Computes CSP string via `#getCsp()` at `documentRenderer.ts:99`.
  5. Calls `renderBody()` to get body HTML and containing-image set.
  6. Injects `pre.js` script, user/contribution stylesheets, and contribution scripts into the `<head>` and `<body>`.
  7. Embeds the rendered body HTML into `data-initial-md-content` meta attribute — the webview reads this attribute on load rather than from the DOM directly, avoiding a flash.
- `renderBody(markdownDocument, resourceProvider)` — `documentRenderer.ts:130`. Calls `engine.render(markdownDocument, resourceProvider)`, wraps in `<div class="markdown-body" dir="auto">`, returns `{html, containingImages}`.
- `renderFileNotFoundDocument(resource)` — `documentRenderer.ts:142`. Returns a minimal HTML body for 404-like cases.
- `#getCsp(provider, resource, nonce)` — `documentRenderer.ts:241`. Branches on `MarkdownPreviewSecurityLevel` enum values: `Strict`, `AllowInsecureContent`, `AllowInsecureLocalContent`, `AllowScriptsAndAllContent`. `AllowScriptsAndAllContent` returns an empty CSP string (no restrictions). Others return explicit `default-src 'none'` policies.
- `#fixHref(resourceProvider, resource, href)` — `documentRenderer.ts:159`. Resolves stylesheet hrefs to webview URIs using `resourceProvider.asWebviewUri`. Handles absolute, workspace-relative, and document-relative paths.
- `#getSettingsOverrideStyles(config)` — `documentRenderer.ts:194`. Emits CSS custom property overrides as inline `style` on `<html>`: `--markdown-font-family`, `--markdown-font-size`, `--markdown-line-height`.

**Porting:** The entire output of this class is platform-standard HTML. The only VS Code API coupling is `vscode.Uri.joinPath` for path resolution and `WebviewResourceProvider.asWebviewUri` for URI-to-webview-URL conversion.

---

#### 5. `src/preview/previewManager.ts` — Preview Lifecycle Manager

**Role:** `MarkdownPreviewManager` implements both `vscode.WebviewPanelSerializer` (for `DynamicMarkdownPreview` persistence across restarts) and `vscode.CustomTextEditorProvider` (for `StaticMarkdownPreview`). It owns the collections of active previews and arbitrates which one is "active".

**Key symbols:**

- `MarkdownPreviewManager` (class) — `previewManager.ts:72`. Extends `Disposable`. Holds two `PreviewStore<T>` instances: `#dynamicPreviews` and `#staticPreviews`.
- Constructor — `previewManager.ts:87`. Registers:
  - `vscode.window.registerWebviewPanelSerializer(DynamicMarkdownPreview.viewType, this)` at `previewManager.ts:100`
  - `vscode.window.registerCustomEditorProvider(StaticMarkdownPreview.customEditorViewType, this, ...)` at `previewManager.ts:102`
  - `vscode.window.onDidChangeActiveTextEditor` handler at `previewManager.ts:106` to restore scroll position when switching to a markdown file.
- `openDynamicPreview(resource, settings)` — `previewManager.ts:135`. Checks `#dynamicPreviews.get(resource, settings)` for a match; if found, reveals it; if not, calls `#createNewDynamicPreview`. Then calls `preview.update(resource, ...)`.
- `deserializeWebviewPanel(webview, state)` — `previewManager.ts:188`. Implements `WebviewPanelSerializer`. Parses state from `state.resource`, `state.locked`, `state.line`, `state.resourceColumn` at `previewManager.ts:193-196`. Calls `DynamicMarkdownPreview.revive()` then `#registerDynamicPreview`.
- `resolveCustomTextEditor(document, webview)` — `previewManager.ts:246`. Implements `CustomTextEditorProvider`. Calls `StaticMarkdownPreview.revive()` then `#registerStaticPreview`. Sets `#activePreview`.
- `toggleLock()` — `previewManager.ts:169`. Delegates to `DynamicMarkdownPreview.toggleLock()` and disposes now-redundant duplicates.

- `PreviewStore<T>` (class) — `previewManager.ts:25`. Internal Set-backed store. `get(resource, previewSettings)` at `previewManager.ts:41` iterates previews calling `matchesResource(resource, previewColumn, locked)`. `#resolvePreviewColumn()` at `previewManager.ts:59` handles `ViewColumn.Active` and `ViewColumn.Beside` by reading from `vscode.window.tabGroups.activeTabGroup.viewColumn`.

---

#### 6. `src/client/client.ts` — Language Server Client Setup

**Role:** Wraps the `vscode-languageclient` `BaseLanguageClient` to create `MdLanguageClient`, and implements the bidirectional RPC bridge between the VS Code extension host and the `vscode-markdown-languageserver` worker process.

**Key symbols:**

- `MdLanguageClient` (class) — `client.ts:18`. Thin wrapper holding `#client: BaseLanguageClient` and `#workspace: VsCodeMdWorkspace`. Public methods proxy `sendRequest` calls to `protocol.*` request types:
  - `resolveLinkTarget` — `client.ts:36`
  - `getEditForFileRenames` — `client.ts:40`
  - `getReferencesToFileInWorkspace` — `client.ts:44`
  - `prepareUpdatePastedLinks` — `client.ts:48`
  - `getUpdatePastedLinksEdit` — `client.ts:55`

- `startClient(factory, parser)` — `client.ts:64`. Async. Configures `LanguageClientOptions` at `client.ts:68`:
  - `documentSelector`: `markdownLanguageIds`
  - `synchronize.fileEvents`: glob watcher for `**/*.{md,...}`
  - `initializationOptions`: passes `markdownFileExtensions` and `i10lLocation`
  - `diagnosticPullOptions`: enables pull-based diagnostics on change and tab-switch for markdown files

- **Server-to-client request handlers** (the server calls into the extension for file system access):
  - `proto.parse` — `client.ts:109`. Tokenizes a document. If `e.text` is provided, wraps it in `InMemoryDocument`; otherwise fetches from `VsCodeMdWorkspace`. Calls `parser.tokenize(doc)`.
  - `proto.fs_readFile` — `client.ts:123`. Calls `vscode.workspace.fs.readFile`, returns `Array.from(bytes)`.
  - `proto.fs_stat` — `client.ts:128`. Returns `{ isDirectory: boolean }` or `undefined`.
  - `proto.fs_readDirectory` — `client.ts:138`. Returns `[string, {isDirectory}][]`.
  - `proto.findMarkdownFilesInWorkspace` — `client.ts:144`. Uses `vscode.workspace.findFiles`.
  - `proto.fs_watcher_create` — `client.ts:150`. Delegates to `FileWatcherManager.create()`, sends `proto.fs_watcher_onChange` back on file events.
  - `proto.fs_watcher_delete` — `client.ts:165`. Delegates to `FileWatcherManager.delete()`.

- **Command proxies** registered at `client.ts:169`:
  - `vscodeMarkdownLanguageservice.open` → `vscode.open`
  - `vscodeMarkdownLanguageservice.rename` → `editor.action.rename`

- Registers notebook document sync for `{ notebook: '*', cells: [{ language: 'markdown' }] }` at `client.ts:96`.

**Porting:** The file system handler pattern (server calls client for FS access) is an inversion of the typical LSP flow. In a Tauri port, these handlers would need to be replaced with a Tauri IPC bridge or the language server would need direct filesystem access.

---

#### 7. `src/client/protocol.ts` — Custom LSP Request Definitions

**Role:** Declares all custom (non-standard LSP) `RequestType` objects used in the markdown language server protocol. These extend the base LSP wire protocol.

**From-server requests (server calls extension host):**
- `parse` — `protocol.ts:18`. `markdown/parse`. Input: `{ uri, text? }`. Output: `md.Token[]`.
- `fs_readFile` — `protocol.ts:20`. `markdown/fs/readFile`. Output: `number[]` (bytes).
- `fs_readDirectory` — `protocol.ts:21`. `markdown/fs/readDirectory`. Output: file/dir listing.
- `fs_stat` — `protocol.ts:22`. `markdown/fs/stat`.
- `fs_watcher_create` — `protocol.ts:24`. `markdown/fs/watcher/create`. Input includes `md.FileWatcherOptions`.
- `fs_watcher_delete` — `protocol.ts:25`. `markdown/fs/watcher/delete`.
- `findMarkdownFilesInWorkspace` — `protocol.ts:27`. `markdown/findMarkdownFilesInWorkspace`.

**To-server requests (extension host calls server):**
- `getReferencesToFileInWorkspace` — `protocol.ts:31`. `markdown/getReferencesToFileInWorkspace`. Output: `lsp.Location[]`.
- `getEditForFileRenames` — `protocol.ts:32`. `markdown/getEditForFileRenames`. Output: `{ participatingRenames, edit }`.
- `prepareUpdatePastedLinks` — `protocol.ts:34`. `markdown/prepareUpdatePastedLinks`. Output: opaque `string` metadata.
- `getUpdatePastedLinksEdit` — `protocol.ts:35`. `markdown/getUpdatePastedLinksEdit`. Output: `lsp.TextEdit[]`.
- `fs_watcher_onChange` — `protocol.ts:37`. `markdown/fs/watcher/onChange`. Sent from extension host to server when a watched file changes.
- `resolveLinkTarget` — `protocol.ts:39`. `markdown/resolveLinkTarget`. Output: `ResolvedDocumentLinkTarget` discriminated union (`file | folder | external`).

**Type definitions:**
- `ResolvedDocumentLinkTarget` — `protocol.ts:12`. Three-case discriminated union using `kind` discriminant.

---

#### 8. `src/languageFeatures/diagnostics.ts` — Link Validation Diagnostics

**Role:** Provides the client-side surface for link validation diagnostics: a `CodeActionProvider` offering "ignore link" quick fixes, and a language status bar item to toggle validation on/off.

**Key symbols:**

- `DiagnosticCode` (enum) — `diagnostics.ts:12`. Four codes: `link_noSuchReferences`, `link_noSuchHeaderInOwnFile`, `link_noSuchFile`, `link_noSuchHeaderInFile`. These values match codes produced by the language server.

- `AddToIgnoreLinksQuickFixProvider` (class) — `diagnostics.ts:20`. Implements `vscode.CodeActionProvider`. Registered via `vscode.languages.registerCodeActionsProvider` at `diagnostics.ts:31`.
  - `provideCodeActions()` — `diagnostics.ts:45`. Iterates `context.diagnostics`, matches on `DiagnosticCode`, extracts `hrefText` from `diagnostic.data.hrefText` (cast via `unknown`). Creates a `vscode.CodeAction` of kind `QuickFix` wired to command `_markdown.addToIgnoreLinks`.
  - The registered command `_markdown.addToIgnoreLinks` at `diagnostics.ts:34` reads config `markdown.validate.ignoredLinks`, adds the path, writes it back via `config.update(..., ConfigurationTarget.WorkspaceFolder)`.

- `registerMarkdownStatusItem(selector, commandManager)` — `diagnostics.ts:76`. Creates a `vscode.languages.createLanguageStatusItem('markdownStatus', selector)`. Updates status text based on `markdown.validate.enabled` config. Command `_markdown.toggleValidation` at `diagnostics.ts:83` calls `config.update('validate.enabled', enabled)`. Subscribes to `onDidChangeConfiguration` at `diagnostics.ts:117`.

- `registerDiagnosticSupport(selector, commandManager)` — `diagnostics.ts:125`. Public export. Composes both registrations.

**Note:** Diagnostic *production* is handled entirely server-side; this file only handles client-side presentation and quick-fix actions.

---

#### 9. `preview-src/index.ts` — Webview Entry Point (Browser-side)

**Role:** This script runs inside the VS Code webview (in a sandboxed browser context). It is the webview-side half of the preview bi-directional communication protocol. It initializes scroll sync, handles incoming messages from the extension, and posts outbound messages back to the extension.

**Key symbols:**

- `acquireVsCodeApi()` — `index.ts:25`. The VS Code webview global function; provides `postMessage`, `getState`, `setState`.
- `createPosterForVsCode(vscode, settings)` — imported from `messaging.ts`. Returns a `MessagePoster` used for all outbound messages. The source URI is injected from `settings.settings.source`.
- `onceDocumentLoaded` callback — `index.ts:72`. After DOM is ready:
  1. Parses initial HTML from `data-initial-md-content` meta attribute via `DOMParser` at `index.ts:74`.
  2. Appends parsed elements to `document.body`.
  3. Calls `domEval()` to re-execute any `<script>` tags.
  4. Restores scroll position from `state.scrollProgress` if present.
  5. Scrolls to fragment or line from settings.

- **Inbound message handler** (`window.addEventListener('message', ...)`) — `index.ts:215`:
  - `copyImage` — locates `<img>` by `data.id`, calls `copyImage()` which uses `navigator.clipboard.write` with a Canvas-rendered PNG blob.
  - `onDidChangeTextEditorSelection` — calls `marker.onDidChangeTextEditorSelection(data.line, documentVersion)`.
  - `updateView` — calls `onUpdateView(data.line)` which throttles calls to `scrollToRevealSourceLine`.
  - `updateContent` — parses new HTML with `DOMParser` then calls `morphdom()` at `index.ts:266` to reconcile DOM differences. Custom `onBeforeElUpdated` at `index.ts:268` preserves `data-line` attributes and `<details open>` state. Increments `documentVersion`, dispatches `vscode.markdown.updateContent` custom event, calls `addImageContexts()`.

- **Outbound event handlers:**
  - `document.addEventListener('dblclick', ...)` — `index.ts:316`. Computes source line from `getEditorLineNumberForPageOffset(event.pageY)`, posts `didClick` message.
  - `document.addEventListener('click', ...)` — `index.ts:342`. Intercepts non-anchor-scheme link clicks, posts `openLink` message. Pass-through schemes: `http:`, `https:`, `mailto:`, `vscode:`, `vscode-insiders:`.
  - `window.addEventListener('scroll', throttle(..., 50))` — `index.ts:377`. Computes topmost visible source line, posts `revealLine` message. `scrollDisabledCount` guard prevents feedback loops.

- `addImageContexts()` — `index.ts:159`. Assigns unique IDs (`image-0`, `image-1`, ...) to all `<img>` elements and sets `data-vscode-context` attribute for webview context menus.

- `domEval(el)` — `index.ts:438`. Re-executes scripts injected by morphdom by cloning `<script>` tags via `document.createElement('script')`.

---

#### 10. `preview-src/messaging.ts` — Outbound Webview Message Abstraction

**Role:** Provides the `MessagePoster` interface and its factory for the webview side. Injects `source` (the document URI) into every outbound message.

**Key symbols:**

- `MessagePoster` (interface) — `messaging.ts:10`. Single method `postMessage<T extends FromWebviewMessage.Type>(type, body)`.
- `createPosterForVsCode(vscode, settingsManager)` — `messaging.ts:20`. Returns an object whose `postMessage` merges `type`, `source: settingsManager.settings!.source`, and `body` into a single object passed to `vscode.postMessage`.

---

#### 11. `types/previewMessaging.d.ts` — Preview Message Type Contract

**Role:** The shared type contract for the bi-directional postMessage protocol between the extension host and the webview. Consumed by both `src/preview/preview.ts` (host side) and `preview-src/index.ts` / `preview-src/messaging.ts` (webview side).

**FromWebviewMessage (webview → host):**
- `CacheImageSizes` — `previewMessaging.d.ts:12`. Carries `imageData: Array<{id, width, height}>`.
- `RevealLine` — `previewMessaging.d.ts:17`. Carries `line: number`.
- `DidClick` — `previewMessaging.d.ts:22`. Carries `line: number`.
- `ClickLink` — `previewMessaging.d.ts:27`. Carries `href: string`.
- `ShowPreviewSecuritySelector` — `previewMessaging.d.ts:32`.
- `PreviewStyleLoadError` — `previewMessaging.d.ts:37`. Carries `unloadedStyles: readonly string[]`.
- `Type` union — `previewMessaging.d.ts:41`.

**ToWebviewMessage (host → webview):**
- `OnDidChangeTextEditorSelection` — `previewMessaging.d.ts:52`. Carries `line: number`.
- `UpdateView` — `previewMessaging.d.ts:57`. Carries `line: number`, `source: string`.
- `UpdateContent` — `previewMessaging.d.ts:62`. Carries `content: string` (partial HTML body).
- `CopyImageContent` — `previewMessaging.d.ts:67`. Carries `id: string`.
- `OpenImageContent` — `previewMessaging.d.ts:73`. Carries `imageSource: string`.
- `Type` union — `previewMessaging.d.ts:80`.

All messages extend `BaseMessage` which carries `source: string` — the stringified document URI — used as a routing key to reject messages intended for other preview instances.

---

#### 12. `src/markdownEngine.ts` — Markdown-It Rendering Engine

**Role:** `MarkdownItEngine` wraps the `markdown-it` library, adding VS Code-specific plugins, a token cache, and resource URI resolution for webview contexts. It also implements `IMdParser` so the language server can reuse the same tokenizer.

**Key symbols:**

- `MarkdownItEngine` (class) — `markdownEngine.ts:99`. Implements `IMdParser`. Lazily initializes `#md: Promise<MarkdownIt>`.
- `#getEngine(config)` — `markdownEngine.ts:132`. On first call, dynamically imports `markdown-it`, configures highlight.js via `getMarkdownOptions()` at `markdownEngine.ts:410`, applies contribution plugins from `contributionProvider.contributions.markdownItPlugins`, installs `markdown-it-front-matter` (extracted as a block rule), then applies six custom renderer patches, and finally uses `pluginSourceMap`.
- `render(input, resourceProvider)` — `markdownEngine.ts:208`. Accepts either a `ITextDocument` or a plain string. Returns `{html, containingImages}`.
- `tokenize(document)` — `markdownEngine.ts:234`. Implements `IMdParser.tokenize`. Used by the language server client handler in `client.ts:109`.
- `#tokenizeDocument(document, config, engine)` — `markdownEngine.ts:182`. Checks `TokenCache` before calling `engine.parse`.

**Renderer plugins installed in `#getEngine`:**
- `#addImageRenderer` — `markdownEngine.ts:253`. Adds `data-src` attribute preserving original URL; resolves `src` to webview URI via `#toResourceUri`. Collects all image sources into `env.containingImages`.
- `#addFencedRenderer` — `markdownEngine.ts:275`. Adds `hljs` class to fenced code blocks.
- `#addLinkNormalizer` — `markdownEngine.ts:291`. Rewrites `vscode://` and `vscode-insiders://` scheme URIs to `vscode.env.uriScheme`.
- `#addLinkValidator` — `markdownEngine.ts:307`. Extends link validation to allow `vscode:` and `data:image/` URIs.
- `#addNamedHeaders` — `markdownEngine.ts:317`. Computes heading slugs via `SlugBuilder` (GitHub-compatible) and sets `id` attribute.
- `#addLinkRenderer` — `markdownEngine.ts:347`. Copies `href` to `data-href` attribute for click interception in the webview.
- `pluginSourceMap` — `markdownEngine.ts:19`. Core ruler plugin that adds `data-line` and `code-line` class to all block-level tokens for scroll sync.

**Token cache:** `TokenCache` at `markdownEngine.ts:46` caches `MarkdownIt.Token[]` keyed by URI, document version, and `{breaks, linkify}` config. Cache invalidated when contribution plugins change.

**Config:** `#getConfig(resource)` at `markdownEngine.ts:244` reads `MarkdownPreviewConfiguration.getForResource(resource)` to obtain `breaks`, `linkify`, `typographer` settings.

---

### Cross-Cutting Synthesis

The markdown-language-features extension is structured around three independent but interconnected subsystems. The **language server subsystem** (`client/client.ts`, `client/protocol.ts`) connects to an external `vscode-markdown-languageserver` worker via IPC (Node) or web workers (browser), using a custom bidirectional RPC layer on top of LSP where the server calls back into the extension host for all file system access. The **preview subsystem** (`preview/preview.ts`, `preview/previewManager.ts`, `preview/documentRenderer.ts`, `markdownEngine.ts`) renders markdown through `markdown-it` to HTML, injects it into a `vscode.WebviewPanel`, and maintains two-way scroll synchronization through a typed `postMessage` protocol typed in `types/previewMessaging.d.ts`. The **webview runtime** (`preview-src/index.ts`, `preview-src/messaging.ts`) runs inside the sandboxed webview, using `acquireVsCodeApi()` for message passing and `morphdom` for incremental DOM diffing on content updates. For a Tauri/Rust port, the three most VS Code-specific couplings are: (1) `vscode.WebviewPanel` and its `postMessage`/`onDidReceiveMessage` APIs used throughout `preview.ts`; (2) `vscode-languageclient`'s `LanguageClient` / `BaseLanguageClient` types and `TransportKind.ipc` used in `extension.ts` and `client.ts`; (3) the `WebviewPanelSerializer` / `CustomTextEditorProvider` registration surface in `previewManager.ts`. The markdown-to-HTML rendering pipeline itself (`markdownEngine.ts`, `documentRenderer.ts`) is largely self-contained TypeScript/JavaScript and could run in a Tauri-hosted WebView with minimal changes, but the IPC and webview management layers require full replacement.

---

### Out-of-Partition References

- `vscode-markdown-languageserver` (npm package) — the external language server binary/worker loaded by `extension.ts:32-38`. Not in this partition.
- `extensions/markdown-language-features/src/util/openDocumentLink.ts` — `MdLinkOpener` class used by `preview.ts` and `previewManager.ts` for link resolution; not analysed in depth.
- `extensions/markdown-language-features/src/preview/topmostLineMonitor.ts` — `TopmostLineMonitor` used for cross-panel scroll position persistence; not analysed in depth.
- `extensions/markdown-language-features/src/preview/previewConfig.ts` — `MarkdownPreviewConfigurationManager` / `MarkdownPreviewConfiguration`; drives per-resource config caching.
- `extensions/markdown-language-features/src/preview/security.ts` — `ContentSecurityPolicyArbiter` / `MarkdownPreviewSecurityLevel`; provides CSP level decisions consumed by `documentRenderer.ts`.
- `extensions/markdown-language-features/src/client/workspace.ts` — `VsCodeMdWorkspace`; tracks open documents for the language server.
- `extensions/markdown-language-features/src/client/fileWatchingManager.ts` — `FileWatcherManager`; multiplexes `vscode.FileSystemWatcher` instances for the server-requested watchers.
- `extensions/markdown-language-features/src/markdownExtensions.ts` — `MarkdownContributionProvider` / `getMarkdownExtensionContributions`; aggregates preview styles, scripts, and markdown-it plugins from other extensions.
- `extensions/markdown-language-features/src/extension.browser.ts` — Browser entry point (not read); uses a web worker transport instead of IPC but calls the same `activateShared`.
- `extensions/markdown-language-features/preview-src/scroll-sync.ts` — `scrollToRevealSourceLine`, `getEditorLineNumberForPageOffset`; uses `data-line` attributes for bidirectional scroll sync; called from `index.ts`.
- `extensions/markdown-language-features/preview-src/activeLineMarker.ts` — `ActiveLineMarker`; highlights the active editor line in the preview DOM.
- `highlight.js` (npm) — dynamically imported in `markdownEngine.ts:411` for syntax highlighting in fenced code blocks.
- `morphdom` (npm) — used in `preview-src/index.ts:266` for incremental DOM patching on content updates.
- `lodash.throttle` (npm) — used in `preview-src/index.ts` for scroll event throttling.
- `markdown-it` / `markdown-it-front-matter` (npm) — the core markdown parsing libraries wrapped by `MarkdownItEngine`.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Webview API Integration Patterns: Markdown Language Features Extension

## Overview
This analysis documents concrete code patterns used in the markdown-language-features extension (85 files, 8,661 LOC) that demonstrate VS Code's Webview API usage. These patterns are critical for understanding the abstraction layer that would need to be ported when migrating from TypeScript/Electron to Tauri/Rust.

---

## Pattern 1: Webview Panel Lifecycle Management

**Where:** `extensions/markdown-language-features/src/preview/preview.ts:655-658`
**What:** Creating a webview panel with configuration and title management for dynamic markdown previews.

```typescript
const webview = vscode.window.createWebviewPanel(
	DynamicMarkdownPreview.viewType,
	DynamicMarkdownPreview.#getPreviewTitle(input.resource, input.locked),
	previewColumn, { enableFindWidget: true, });

webview.iconPath = contentProvider.iconPath;
```

**Key aspects:**
- `createWebviewPanel()` is the primary constructor for panel-based webviews
- Accepts viewType identifier, title, column position, and options object
- Options include `enableFindWidget`, `enableScripts`, `enableForms`, `localResourceRoots`
- Panel lifecycle tied to disposal and view state changes

**Call sites:**
- `src/preview/preview.ts:645-664` - `DynamicMarkdownPreview.create()`
- Registered with serializer at `src/preview/previewManager.ts:100`
- Custom editor registration at `src/preview/previewManager.ts:102`

---

## Pattern 2: Bidirectional Message Passing Between Extension and Webview

**Where:** `extensions/markdown-language-features/src/preview/preview.ts:141-172`
**What:** Receiving and dispatching messages from webview to extension, with source routing.

```typescript
this._register(this.#webviewPanel.webview.onDidReceiveMessage((e: FromWebviewMessage.Type) => {
	if (e.source !== this.#resource.toString()) {
		return;
	}

	switch (e.type) {
		case 'cacheImageSizes':
			this.#imageInfo = e.imageData;
			break;

		case 'revealLine':
			this.#onDidScrollPreview(e.line);
			break;

		case 'didClick':
			this.#onDidClickPreview(e.line);
			break;

		case 'openLink':
			this.#onDidClickPreviewLink(e.href);
			break;

		case 'showPreviewSecuritySelector':
			vscode.commands.executeCommand('markdown.showPreviewSecuritySelector', e.source);
			break;

		case 'previewStyleLoadError':
			vscode.window.showWarningMessage(
				vscode.l10n.t("Could not load 'markdown.styles': {0}", e.unloadedStyles.join(', ')));
			break;
	}
}));
```

**Key aspects:**
- `onDidReceiveMessage()` returns an event emitter for webview→extension messages
- Source field routes messages to correct preview instance
- Type-safe message handling via discriminated union
- Each message type handles different interaction (scroll, click, link, error)

**Message types defined:** `types/previewMessaging.d.ts:10-49`
- `CacheImageSizes`, `RevealLine`, `DidClick`, `ClickLink`, `ShowPreviewSecuritySelector`, `PreviewStyleLoadError`

---

## Pattern 3: Sending Messages to Webview from Extension

**Where:** `extensions/markdown-language-features/src/preview/preview.ts:226-230`
**What:** Wrapper method to post messages from extension to webview with disposal check.

```typescript
public postMessage(msg: ToWebviewMessage.Type) {
	if (!this.#disposed) {
		this.#webviewPanel.webview.postMessage(msg);
	}
}
```

**Usage examples:**
- Scroll position updates at `src/preview/preview.ts:244-248`
- Content updates at `src/preview/preview.ts:377-381`
- Image copy commands at `src/preview/preview.ts:567-571`
- Text editor selection at `src/preview/preview.ts:715-720`

**Key aspects:**
- Disposal check prevents sending to destroyed panels
- Type-safe message objects sent to webview
- Messages include source identifier for routing
- Part of larger `ToWebviewMessage` namespace with types: `UpdateView`, `UpdateContent`, `CopyImageContent`, `OnDidChangeTextEditorSelection`

---

## Pattern 4: Webview Options and Security Configuration

**Where:** `extensions/markdown-language-features/src/preview/preview.ts:408-430`
**What:** Setting webview options including script execution, form handling, and resource roots.

```typescript
#getWebviewOptions(): vscode.WebviewOptions {
	return {
		enableScripts: true,
		enableForms: false,
		localResourceRoots: this.#getLocalResourceRoots()
	};
}

#getLocalResourceRoots(): ReadonlyArray<vscode.Uri> {
	const baseRoots = Array.from(this.#contributionProvider.contributions.previewResourceRoots);

	const folder = vscode.workspace.getWorkspaceFolder(this.#resource);
	if (folder) {
		const workspaceRoots = vscode.workspace.workspaceFolders?.map(folder => folder.uri);
		if (workspaceRoots) {
			baseRoots.push(...workspaceRoots);
		}
	} else {
		baseRoots.push(uri.Utils.dirname(this.#resource));
	}

	return baseRoots;
}
```

**Applied at:** `src/preview/preview.ts:372`

**Key aspects:**
- `enableScripts: true` for interactive content
- `enableForms: false` restricts form submission
- `localResourceRoots` restricts file access to specific directories
- Roots include workspace folders, extension contributions, and document directory
- Security-critical configuration for sandboxed webview

---

## Pattern 5: Resource URI Transformation for Webview Serving

**Where:** `extensions/markdown-language-features/src/util/resources.ts:8-12`
**What:** Interface abstraction for converting file URIs to webview-accessible URIs.

```typescript
export interface WebviewResourceProvider {
	asWebviewUri(resource: vscode.Uri): vscode.Uri;

	readonly cspSource: string;
}
```

**Implementation at:** `src/preview/preview.ts:454-470`
```typescript
asWebviewUri(resource: vscode.Uri) {
	return this.#webviewPanel.webview.asWebviewUri(resource);
}

get cspSource() {
	return [
		this.#webviewPanel.webview.cspSource,

		// On web, we also need to allow loading of resources from contributed extensions
		...this.#contributionProvider.contributions.previewResourceRoots
			.filter(root => root.scheme === 'http' || root.scheme === 'https')
			.map(root => {
				const dirRoot = root.path.endsWith('/') ? root : root.with({ path: root.path + '/' });
				return dirRoot.toString();
			}),
	].join(' ');
}
```

**Usage sites:**
- Image loading at `src/markdownEngine.ts:371, 393`
- Base href at `src/preview/documentRenderer.ts:118`
- Extension resources at `src/preview/documentRenderer.ts:154-156`
- Style sheet links at `src/preview/documentRenderer.ts:222`
- Script inclusion at `src/preview/documentRenderer.ts:234`

**Key aspects:**
- Transforms `file://` to webview-safe URIs
- CSP source generation for inline security policies
- Handles both local files and web-based resources
- Essential for serving extension media files

---

## Pattern 6: File System Watching with Targeted Refresh

**Where:** `extensions/markdown-language-features/src/preview/preview.ts:128-139`
**What:** Watching document directory for changes and triggering preview updates.

```typescript
if (vscode.workspace.fs.isWritableFileSystem(resource.scheme)) {
	const watcher = this._register(vscode.workspace.createFileSystemWatcher(new vscode.RelativePattern(resource, '*')));
	this._register(watcher.onDidChange(uri => {
		if (this.isPreviewOf(uri)) {
			// Only use the file system event when VS Code does not already know about the file.
			// This is needed to avoid duplicate refreshes
			if (!vscode.workspace.textDocuments.some(doc => areUrisEqual(doc.uri, uri))) {
				this.refresh();
			}
		}
	}));
}
```

**Related patterns:**
- Image watcher at `src/preview/preview.ts:396-406`
- Global markdown watcher at `src/client/workspace.ts:29-40`
- Client-level file watcher at `src/client/client.ts:72`
- Manager-level watchers at `src/client/fileWatchingManager.ts:36-54`

**Key aspects:**
- `createFileSystemWatcher()` with glob patterns
- `RelativePattern` for workspace-relative watching
- Deduplication to prevent redundant refreshes
- Scheme checking for writable file systems
- Events: `onDidChange`, `onDidCreate`, `onDidDelete`

---

## Pattern 7: Webview Panel Serialization and Restoration

**Where:** `extensions/markdown-language-features/src/preview/previewManager.ts:188-244`
**What:** Implementing `WebviewPanelSerializer` to restore webview state across VS Code restarts.

```typescript
public async deserializeWebviewPanel(
	webview: vscode.WebviewPanel,
	state: any
): Promise<void> {
	try {
		const resource = vscode.Uri.parse(state.resource);
		const locked = state.locked;
		const line = state.line;
		const resourceColumn = state.resourceColumn;

		const preview = DynamicMarkdownPreview.revive(
			{ resource, locked, line, resourceColumn },
			webview,
			this.#contentProvider,
			this.#previewConfigurations,
			this.#logger,
			this.#topmostLineMonitor,
			this.#contributions,
			this.#opener);

		this.#registerDynamicPreview(preview);
	} catch (e) {
		console.error(e);

		webview.webview.html = /* html */`<!DOCTYPE html>
		<html lang="en">
		<head>
			<meta charset="UTF-8">

			<!-- Disable pinch zooming -->
			<meta name="viewport"
				content="width=device-width, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0, user-scalable=no">

			<title>Markdown Preview</title>

			<style>
				html, body {
					min-height: 100%;
					height: 100%;
				}

				.error-container {
					display: flex;
					justify-content: center;
					align-items: center;
					text-align: center;
				}
			</style>

			<meta http-equiv="Content-Security-Policy" content="default-src 'none';">
		</head>
		<body class="error-container">
			<p>${vscode.l10n.t("An unexpected error occurred while restoring the Markdown preview.")}</p>
		</body>
		</html>`;
	}
}
```

**Registration at:** `src/preview/previewManager.ts:100`
```typescript
this._register(vscode.window.registerWebviewPanelSerializer(DynamicMarkdownPreview.viewType, this));
```

**State preservation at:** `src/preview/preview.ts:195-202`
```typescript
public get state() {
	return {
		resource: this.#resource.toString(),
		line: this.#line,
		fragment: this.#scrollToFragment,
		...this.#delegate.getAdditionalState(),
	};
}
```

**Key aspects:**
- Serializable state includes resource URI, scroll position, and view settings
- Error handling with fallback UI
- Type-safe deserialization from stored state
- Revive pattern reconstitutes full object from serialized data
- WebviewPanelSerializer interface implementation

---

## Pattern 8: Custom Editor Provider for Text Document Editing

**Where:** `extensions/markdown-language-features/src/preview/previewManager.ts:246-264`
**What:** Implementing custom editor provider for static markdown preview editing.

```typescript
public async resolveCustomTextEditor(
	document: vscode.TextDocument,
	webview: vscode.WebviewPanel
): Promise<void> {
	const lineNumber = this.#topmostLineMonitor.getPreviousStaticTextEditorLineByUri(document.uri);
	const preview = StaticMarkdownPreview.revive(
		document.uri,
		webview,
		this.#contentProvider,
		this.#previewConfigurations,
		this.#topmostLineMonitor,
		this.#logger,
		this.#contributions,
		this.#opener,
		lineNumber
	);
	this.#registerStaticPreview(preview);
	this.#activePreview = preview;
}
```

**Registration at:** `src/preview/previewManager.ts:102-104`
```typescript
this._register(vscode.window.registerCustomEditorProvider(StaticMarkdownPreview.customEditorViewType, this, {
	webviewOptions: { enableFindWidget: true }
}));
```

**Key aspects:**
- `CustomTextEditorProvider` interface for editor-like webviews
- Receives `vscode.TextDocument` for content synchronization
- Webview panel provided by VS Code
- State restoration via line number monitor
- Enablement of find widget for search/replace

---

## Pattern 9: Event-Driven Synchronization Between Editor and Preview

**Where:** `extensions/markdown-language-features/src/preview/preview.ts:713-733`
**What:** Listening to editor events and propagating changes to webview and vice versa.

```typescript
this._register(vscode.window.onDidChangeTextEditorSelection(event => {
	if (this.#preview.isPreviewOf(event.textEditor.document.uri)) {
		this.#preview.postMessage({
			type: 'onDidChangeTextEditorSelection',
			line: event.selections[0].active.line,
			source: this.#preview.resource.toString()
		});
	}
}));

this._register(vscode.window.onDidChangeActiveTextEditor(editor => {
	// Only allow previewing normal text editors which have a viewColumn: See #101514
	if (typeof editor?.viewColumn === 'undefined') {
		return;
	}

	if (isMarkdownFile(editor.document) && !this.#locked && !this.#preview.isPreviewOf(editor.document.uri)) {
		const line = getVisibleLine(editor);
		this.update(editor.document.uri, line ? new StartingScrollLine(line) : undefined);
	}
}));
```

**Related handlers:**
- Scroll sync at `src/preview/preview.ts:116-120, 308-330`
- Document change at `src/preview/preview.ts:116-126`
- Configuration changes at `src/extension.shared.ts:48-50`

**Key aspects:**
- Event listeners bridge editor and webview state
- Message posting updates preview based on editor changes
- Prevents redundant synchronization with state checks
- Handles file open, active editor change, selection change
- Scroll position bidirectional sync

---

## Pattern 10: Command Registration and Execution Framework

**Where:** `extensions/markdown-language-features/src/commandManager.ts:14-38`
**What:** Centralized command registration system for extension commands.

```typescript
export class CommandManager {
	readonly #commands = new Map<string, vscode.Disposable>();

	public dispose() {
		for (const registration of this.#commands.values()) {
			registration.dispose();
		}
		this.#commands.clear();
	}

	public register<T extends Command>(command: T): vscode.Disposable {
		this.#registerCommand(command.id, command.execute, command);
		return new vscode.Disposable(() => {
			this.#commands.delete(command.id);
		});
	}

	#registerCommand(id: string, impl: (...args: any[]) => void, thisArg?: any) {
		if (this.#commands.has(id)) {
			return;
		}

		this.#commands.set(id, vscode.commands.registerCommand(id, impl, thisArg));
	}
}
```

**Usage in commands:**
- `src/commands/showPreview.ts:67-74` - Preview display commands
- Execution at `src/preview/preview.ts:164, 334`

**Key aspects:**
- Deduplication of command registrations
- Command object interface for type safety
- Centralized disposal management
- Wrapper around `vscode.commands.registerCommand()`

---

## Pattern 11: Language Feature Provider Registration

**Where:** `extensions/markdown-language-features/src/extension.shared.ts:53-68`
**What:** Registering multiple language intelligence providers for markdown.

```typescript
function registerMarkdownLanguageFeatures(
	client: MdLanguageClient,
	commandManager: CommandManager,
	parser: IMdParser,
): vscode.Disposable {
	const selector: vscode.DocumentSelector = markdownLanguageIds;
	return vscode.Disposable.from(
		// Language features
		registerDiagnosticSupport(selector, commandManager),
		registerFindFileReferenceSupport(commandManager, client),
		registerResourceDropOrPasteSupport(selector, parser),
		registerPasteUrlSupport(selector, parser),
		registerUpdateLinksOnRename(client),
		registerUpdatePastedLinks(selector, client),
	);
}
```

**Specific provider registrations:**
- Code actions at `src/languageFeatures/diagnostics.ts:31`
- Document paste at `src/languageFeatures/copyFiles/pasteUrlProvider.ts:83`
- Document drop/paste at `src/languageFeatures/copyFiles/dropOrPasteResource.ts:296-303`

**Key aspects:**
- Document selector for language scoping
- Multiple provider types: diagnostics, code actions, paste/drop edits
- Composable registration via `vscode.Disposable.from()`
- Providers execute synchronously or asynchronously
- Declarative selector for language-specific features

---

## Pattern 12: Configuration Management with Caching

**Where:** `extensions/markdown-language-features/src/preview/previewConfig.ts:75-99`
**What:** Loading and caching configuration per workspace with change detection.

```typescript
export class MarkdownPreviewConfigurationManager {
	readonly #previewConfigurationsForWorkspaces = new Map<string, MarkdownPreviewConfiguration>();

	public loadAndCacheConfiguration(
		resource: vscode.Uri
	): MarkdownPreviewConfiguration {
		const config = MarkdownPreviewConfiguration.getForResource(resource);
		this.#previewConfigurationsForWorkspaces.set(this.#getKey(resource), config);
		return config;
	}

	public hasConfigurationChanged(resource: vscode.Uri): boolean {
		const key = this.#getKey(resource);
		const currentConfig = this.#previewConfigurationsForWorkspaces.get(key);
		const newConfig = MarkdownPreviewConfiguration.getForResource(resource);
		return !currentConfig?.isEqualTo(newConfig);
	}

	#getKey(
		resource: vscode.Uri
	): string {
		const folder = vscode.workspace.getWorkspaceFolder(resource);
		return folder ? folder.uri.toString() : '';
	}
}
```

**Configuration keys accessed:** `src/preview/previewConfig.ts:31-57`
- `editor.scrollBeyondLastLine`, `editor.wordWrap`
- `markdown.preview.*` settings (scrolling, line breaks, linkify, typographer)
- `markdown.styles` (custom CSS)

**Key aspects:**
- Per-workspace caching to minimize repeated reads
- Equality checking to detect changes
- Workspace-relative keying for multi-folder support
- Settings accessed via `vscode.workspace.getConfiguration()`

---

## Summary

These 12 patterns demonstrate the key integration points between VS Code's extension API and Webview/UI layer:

1. **Panel Lifecycle** - Creation, configuration, disposal
2. **Bidirectional Messaging** - Type-safe message contracts with source routing
3. **Resource Transformation** - Converting file URIs for sandboxed webview access
4. **File System Integration** - Watching files and triggering updates
5. **State Persistence** - Serialization/deserialization across sessions
6. **Editor Synchronization** - Two-way event propagation
7. **Command Framework** - Registering and executing IDE commands
8. **Language Features** - Multiple provider types for IDE intelligence
9. **Configuration Management** - Workspace-aware settings caching

**Critical for Tauri migration:**
- Message passing architecture must map to Tauri's invoke/listen system
- Webview panel lifecycle has no direct Tauri equivalent (requires new abstraction)
- File watching can use Tauri's file system watchers
- Resource URIs require different serving mechanism (Tauri asset protocol)
- Configuration system can remain similar with workspace awareness

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
