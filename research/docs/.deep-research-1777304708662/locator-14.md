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
