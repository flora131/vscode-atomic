# File Location Index: Markdown Language Features Extension

**Scope**: `extensions/markdown-language-features/` (110 files, ~8,704 LOC)

**Research Question**: Porting VS Code's core IDE functionality (webview-based preview system) from TypeScript/Electron to Tauri/Rust.

**Key Hot Spots Identified**:
- Custom editor provider registration (`registerCustomEditorProvider`)
- MarkdownPreview class (webview-based preview implementation)
- Webview messaging protocol (extension ↔ webview IPC)

---

## Implementation

### Extension Core & Activation
- `extensions/markdown-language-features/src/extension.ts` — Main entry point; activates language server and shared features
- `extensions/markdown-language-features/src/extension.shared.ts` — Shared activation logic across desktop/browser; registers preview manager and language features
- `extensions/markdown-language-features/src/extension.browser.ts` — Browser-specific extension behavior

### Preview System (Primary Porting Hot Spot)
- `extensions/markdown-language-features/src/preview/preview.ts` — **MarkdownPreview class**: Core webview-based preview implementation; handles webview panel lifecycle, message routing, scroll sync, file watching
- `extensions/markdown-language-features/src/preview/previewManager.ts` — **MarkdownPreviewManager**: Manages preview instances; implements `vscode.WebviewPanelSerializer` and `vscode.CustomTextEditorProvider`; registers custom editor (`vscode.markdown.preview.editor`)
- `extensions/markdown-language-features/src/preview/documentRenderer.ts` — **MdDocumentRenderer**: Generates HTML content from markdown; injects CSP, initial data, styling; prepares webview payload
- `extensions/markdown-language-features/src/preview/previewConfig.ts` — Configuration management for preview behavior (scroll sync, font, theme)
- `extensions/markdown-language-features/src/preview/security.ts` — Content Security Policy arbitration for preview rendering
- `extensions/markdown-language-features/src/preview/scrolling.ts` — Scroll synchronization logic between editor and preview
- `extensions/markdown-language-features/src/preview/topmostLineMonitor.ts` — Tracks scroll position across previews

### Webview-Side Code (Client-Side Preview Script)
- `extensions/markdown-language-features/preview-src/index.ts` — **Main webview entry point**: Initializes DOM, handles messages from extension, manages content updates with morphdom
- `extensions/markdown-language-features/preview-src/messaging.ts` — Creates message poster for extension communication; handles `FromWebviewMessage` protocol
- `extensions/markdown-language-features/preview-src/scroll-sync.ts` — Scroll synchronization on webview side
- `extensions/markdown-language-features/preview-src/activeLineMarker.ts` — Visual indicator for active line in preview
- `extensions/markdown-language-features/preview-src/events.ts` — Event helpers (onceDocumentLoaded)
- `extensions/markdown-language-features/preview-src/settings.ts` — Retrieves and caches preview settings from initial data
- `extensions/markdown-language-features/preview-src/loading.ts` — Loading state management
- `extensions/markdown-language-features/preview-src/csp.ts` — Content Security Policy alerts in webview
- `extensions/markdown-language-features/preview-src/strings.ts` — String utilities
- `extensions/markdown-language-features/preview-src/pre.ts` — Pre-processing script

### Markdown Rendering Engine
- `extensions/markdown-language-features/src/markdownEngine.ts` — **MarkdownItEngine**: Wraps markdown-it library; provides parsing and HTML generation
- `extensions/markdown-language-features/src/markdownExtensions.ts` — Plugin/contribution system for markdown parsing

### Command System
- `extensions/markdown-language-features/src/commandManager.ts` — Command registration and dispatch
- `extensions/markdown-language-features/src/commands/index.ts` — Markdown command registration hub
- `extensions/markdown-language-features/src/commands/showPreview.ts` — Open preview in current column
- `extensions/markdown-language-features/src/commands/showPreviewToSide.ts` (implied) — Open preview in side column
- `extensions/markdown-language-features/src/commands/showSource.ts` — Navigate from preview back to source
- `extensions/markdown-language-features/src/commands/refreshPreview.ts` — Manually refresh preview
- `extensions/markdown-language-features/src/commands/toggleLock.ts` — Lock/unlock preview tracking
- `extensions/markdown-language-features/src/commands/reopenAsPreview.ts` — Switch editor to custom editor view
- `extensions/markdown-language-features/src/commands/showPreviewSecuritySelector.ts` — CSP override dialog
- `extensions/markdown-language-features/src/commands/copyImage.ts` — Copy image from preview
- `extensions/markdown-language-features/src/commands/openImage.ts` — Open image file
- `extensions/markdown-language-features/src/commands/reloadPlugins.ts` — Reload markdown extensions
- `extensions/markdown-language-features/src/commands/renderDocument.ts` — API: render markdown to HTML
- `extensions/markdown-language-features/src/commands/insertResource.ts` — API: insert resource references

### Language Server Integration
- `extensions/markdown-language-features/src/client/client.ts` — LanguageClient initialization and configuration
- `extensions/markdown-language-features/src/client/fileWatchingManager.ts` — File watcher for markdown files
- `extensions/markdown-language-features/src/client/protocol.ts` — Protocol definitions for language server
- `extensions/markdown-language-features/src/client/workspace.ts` — Workspace abstraction for language server
- `extensions/markdown-language-features/src/client/inMemoryDocument.ts` — In-memory document representation

### Language Features
- `extensions/markdown-language-features/src/languageFeatures/diagnostics.ts` — Link validation diagnostics
- `extensions/markdown-language-features/src/languageFeatures/fileReferences.ts` — Find file references
- `extensions/markdown-language-features/src/languageFeatures/linkUpdater.ts` — Update links on file rename
- `extensions/markdown-language-features/src/languageFeatures/updateLinksOnPaste.ts` — Auto-fix links when pasting
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/copyFiles.ts` — Handle copy/paste of files
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/dropOrPasteResource.ts` — Drag-drop and paste resources
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/pasteUrlProvider.ts` — Paste URL as markdown link
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/newFilePathGenerator.ts` — Generate paths for copied files
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/snippets.ts` — File-to-markdown snippet generation
- `extensions/markdown-language-features/src/languageFeatures/copyFiles/shared.ts` — Shared copy/paste utilities

### Utilities
- `extensions/markdown-language-features/src/util/resources.ts` — Webview resource URI conversion
- `extensions/markdown-language-features/src/util/openDocumentLink.ts` — Link opening (markdown links in preview)
- `extensions/markdown-language-features/src/util/document.ts` — Document helpers
- `extensions/markdown-language-features/src/util/dom.ts` — DOM utilities (script evaluation)
- `extensions/markdown-language-features/src/util/file.ts` — File type detection
- `extensions/markdown-language-features/src/util/schemes.ts` — URI scheme handling
- `extensions/markdown-language-features/src/util/url.ts` — URL parsing and conversion
- `extensions/markdown-language-features/src/util/uuid.ts` — UUID generation (CSP nonce)
- `extensions/markdown-language-features/src/util/arrays.ts` — Array utilities
- `extensions/markdown-language-features/src/util/dispose.ts` — Disposable pattern utilities
- `extensions/markdown-language-features/src/util/mimes.ts` — MIME type mapping
- `extensions/markdown-language-features/src/util/cancellation.ts` — Cancellation token helpers
- `extensions/markdown-language-features/src/util/resourceMap.ts` — URI-keyed map
- `extensions/markdown-language-features/src/util/async.ts` — Async utilities
- `extensions/markdown-language-features/src/util/uriList.ts` — URI list parsing (drag-drop)

### Logging & Telemetry
- `extensions/markdown-language-features/src/logging.ts` — **ILogger interface**: Output channel logging
- `extensions/markdown-language-features/src/telemetryReporter.ts` — Telemetry event reporting
- `extensions/markdown-language-features/src/slugify.ts` — Heading slug generation

### Notebook Renderer
- `extensions/markdown-language-features/notebook/index.ts` — Notebook markdown renderer entry point

---

## Tests

### Unit Tests
- `extensions/markdown-language-features/src/test/engine.test.ts` — Markdown engine tests
- `extensions/markdown-language-features/src/test/documentLink.test.ts` — Document link resolution tests
- `extensions/markdown-language-features/src/test/urlToUri.test.ts` — URL-to-URI conversion tests
- `extensions/markdown-language-features/src/test/pasteUrl.test.ts` — Paste URL feature tests
- `extensions/markdown-language-features/src/test/copyFile.test.ts` — Copy/paste file handling tests

### Test Utilities
- `extensions/markdown-language-features/src/test/index.ts` — Test runner/setup
- `extensions/markdown-language-features/src/test/engine.ts` — Test markdown engine instance
- `extensions/markdown-language-features/src/test/nulLogging.ts` — No-op logger for tests
- `extensions/markdown-language-features/src/test/util.ts` — Test utilities

---

## Types / Interfaces

### Message Protocol (Critical for Tauri IPC Porting)
- `extensions/markdown-language-features/types/previewMessaging.d.ts` — TypeScript types for webview ↔ extension messaging
  - **FromWebviewMessage**: Messages sent by webview to extension
    - `CacheImageSizes` — Image dimensions for layout
    - `RevealLine` — Scroll position in preview
    - `DidClick` — Click event with line number
    - `ClickLink` — Link click from preview
    - `ShowPreviewSecuritySelector` — CSP override request
    - `PreviewStyleLoadError` — Failed stylesheet load
  - **ToWebviewMessage**: Messages sent by extension to webview
    - `OnDidChangeTextEditorSelection` — Highlight line in preview
    - `UpdateView` — Update view with line
    - `UpdateContent` — Full HTML content update
    - `CopyImageContent` — Copy image action
    - `OpenImageContent` — Open image action

### Type Definitions
- `extensions/markdown-language-features/src/typings/ref.d.ts` — Reference type definitions

---

## Configuration

### Build Configuration
- `extensions/markdown-language-features/tsconfig.json` — TypeScript configuration (main extension)
- `extensions/markdown-language-features/tsconfig.browser.json` — TypeScript configuration (browser/web)
- `extensions/markdown-language-features/preview-src/tsconfig.json` — TypeScript configuration (webview)
- `extensions/markdown-language-features/notebook/tsconfig.json` — TypeScript configuration (notebook renderer)

### Build Scripts (esbuild)
- `extensions/markdown-language-features/esbuild.mts` — Main extension bundling
- `extensions/markdown-language-features/esbuild.webview.mts` — **Webview bundling** (preview-src → media/)
- `extensions/markdown-language-features/esbuild.browser.mts` — Browser-specific bundling
- `extensions/markdown-language-features/esbuild.notebook.mts` — Notebook renderer bundling

### Extension Manifest
- `extensions/markdown-language-features/package.json` — Extension metadata, commands, menus, configuration, activation events
- `extensions/markdown-language-features/package.nls.json` — Localization strings

### JSON Schemas
- `extensions/markdown-language-features/schemas/package.schema.json` — Schema validation

---

## Examples / Fixtures

### Test Workspace
- `extensions/markdown-language-features/test-workspace/a.md`
- `extensions/markdown-language-features/test-workspace/b.md`
- `extensions/markdown-language-features/test-workspace/sub/c.md`
- `extensions/markdown-language-features/test-workspace/sub/d.md`
- `extensions/markdown-language-features/test-workspace/sub/file with space.md`
- `extensions/markdown-language-features/test-workspace/sub with space/file.md`

---

## Documentation

- `extensions/markdown-language-features/README.md` — Extension overview and configuration

---

## Assets

### Styling
- `extensions/markdown-language-features/media/markdown.css` — Preview styling
- `extensions/markdown-language-features/media/highlight.css` — Syntax highlighting (highlight.js)

### Icons
- `extensions/markdown-language-features/media/preview-light.svg` — Light theme icon
- `extensions/markdown-language-features/media/preview-dark.svg` — Dark theme icon

---

## Notable Clusters

### Webview-Based Preview System (Core Porting Area)
The preview system is split across three layers:
1. **Extension Host** (`src/preview/`): Manages webview lifecycle, handles content generation, routes messages
2. **Webview Client** (`preview-src/`): Renders content, syncs scroll, sends user interactions back
3. **Protocol** (`types/previewMessaging.d.ts`): Typed message contract for IPC

### Message-Driven Architecture
Key for Tauri porting: The entire preview ↔ extension communication is message-based:
- Extension sends `ToWebviewMessage` types: content updates, selection changes, image actions
- Webview sends `FromWebviewMessage` types: scroll events, clicks, CSP overrides
- Uses `vscode.postMessage()` on webview side, `webview.postMessage()` on extension side

### Custom Editor Provider Pattern
`MarkdownPreviewManager` implements `vscode.CustomTextEditorProvider`:
- Provides preview when `.md` file opened in custom editor view
- Alternative to dynamic panel previews
- Critical for integrating preview into editor tabs

### Build Separation
Three separate builds:
- **Extension**: `esbuild.mts` → bundles extension code
- **Webview**: `esbuild.webview.mts` → bundles preview-src/ → media/index.js
- **Browser**: `esbuild.browser.mts` → web version of extension

### File Watching & Auto-Refresh
- `FileWatchingManager`: Language server file watcher
- `MarkdownPreview`: Local file system watcher for referenced assets
- Triggers refresh on content or dependency changes

### Scroll Synchronization
Bidirectional scroll sync between editor and preview:
- `TopmostLineMonitor`: Tracks visible line in preview
- `scrolling.ts`: Calculates line-to-scroll-position mapping
- `scroll-sync.ts` (webview): Maps scroll events back to line numbers

### Content Security Policy (CSP)
Security model for preview rendering:
- `ExtensionContentSecurityPolicyArbiter`: Decides CSP level (strict/moderate/allowLocal/allowScripts)
- CSP injected as `<meta>` tag in generated HTML
- User can override via command (`showPreviewSecuritySelector`)

---

## Summary for Tauri/Rust Porting

**Total Extension Files**: 110

**Critical for Porting**:
1. **Preview Manager** (`previewManager.ts`) — Implements editor provider registration and lifecycle
2. **MarkdownPreview Class** (`preview.ts`) — Core webview container, message routing, state management
3. **Message Protocol** (`previewMessaging.d.ts`) — Must be recreated in Rust/Tauri; defines IPC contract
4. **Document Renderer** (`documentRenderer.ts`) — HTML generation pipeline; CSS/CSP injection
5. **Webview Scripts** (`preview-src/`) — Client-side rendering logic; must be ported or integrated into Tauri window

**Key Ports Required**:
- Webview panel lifecycle (create, show, hide, dispose, restore from state)
- Message routing (typed IPC between Tauri backend and webview)
- HTML content generation (markdown → HTML with styling and CSP)
- Scroll synchronization (line-to-scroll mapping in both directions)
- File watching (detect markdown and asset changes)
- Security model (CSP enforcement in webview)

**Estimated Complexity**: High. The webview-based preview is deeply integrated with VS Code's webview API. Tauri's webview differs in architecture (native OS webview vs. Chromium). Message protocol can be reused, but webview lifecycle, resource loading, and security model require careful mapping.

