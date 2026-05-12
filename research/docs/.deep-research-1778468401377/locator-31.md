# Media Preview Extension - Webview Custom Editor Analysis

## Research Question
Porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust requires understanding the webview-based custom editor architecture, particularly how media previews implement the CustomEditorProvider contract and communicate with webview processes.

## Summary
The media-preview extension demonstrates VS Code's webview architecture through three custom editor implementations (image, audio, video) that implement the `CustomReadonlyEditorProvider` interface. These providers showcase critical webview patterns: DOM rendering, bidirectional messaging (postMessage/onDidReceiveMessage), resource URI handling, Content Security Policy (CSP), and state persistence. All communication occurs through structured message passing between extension host (TypeScript) and webview content (JavaScript/DOM).

---

## Implementation

### Core Extension Entry Point
- `extensions/media-preview/src/extension.ts` — Extension activation registers three preview support modules; demonstrates vscode.ExtensionContext usage

### Custom Editor Providers (CustomReadonlyEditorProvider Interface)
- `extensions/media-preview/src/imagePreview/index.ts` — ImagePreviewManager implements CustomReadonlyEditorProvider; manages multiple image preview instances; handles zoom scaling, image dimensions, copy-to-clipboard; demonstrates multi-instance editor state management
- `extensions/media-preview/src/audioPreview.ts` — AudioPreviewProvider implements CustomReadonlyEditorProvider; provides media playback UI through webview; handles configuration (workspace settings); demonstrates async resource path resolution
- `extensions/media-preview/src/videoPreview.ts` — VideoPreviewProvider implements CustomReadonlyEditorProvider; similar to audio with video-specific settings (autoplay, loop); uses workspace configuration API

### Base MediaPreview Class
- `extensions/media-preview/src/mediaPreview.ts` — Abstract base class for all media previews; manages WebviewPanel lifecycle, webview options (enableScripts: true, localResourceRoots), file system watching, Git LFS detection; handles reopening as text via `vscode.commands.executeCommand('vscode.openWith')`

### Status Bar Integration
- `extensions/media-preview/src/ownedStatusBarEntry.ts` — PreviewStatusBarEntry base class managing owned status bar items; demonstrates status bar creation and visibility control
- `extensions/media-preview/src/binarySizeStatusBarEntry.ts` — Formats file size display (B, KB, MB, GB, TB); uses vscode.l10n for localization
- `extensions/media-preview/src/imagePreview/zoomStatusBarEntry.ts` — EventEmitter-based zoom level selector; registers command via `vscode.commands.registerCommand`; demonstrates status bar with quick pick integration
- `extensions/media-preview/src/imagePreview/sizeStatusBarEntry.ts` — Image dimension display in status bar

### Utility Modules
- `extensions/media-preview/src/util/dom.ts` — HTML attribute escaping for URI values; prevents XSS in webview HTML
- `extensions/media-preview/src/util/dispose.ts` — Disposable base class with subscription management; pattern for resource cleanup in VS Code extensions
- `extensions/media-preview/src/util/uuid.ts` — UUID generation using crypto.randomUUID; used for CSP nonce values in webview HTML

### Webview Content Scripts (Browser-side)
- `extensions/media-preview/media/imagePreview.js` — Complex client-side image viewer; uses `acquireVsCodeApi()` to obtain VSCode API; handles zoom levels, pan/scroll, pixelation rendering, scale-to-fit logic; bidirectional messaging (receives setScale/setActive, sends size/zoom messages)
- `extensions/media-preview/media/audioPreview.js` — HTML5 audio player wrapper; loads audio via vscode.asWebviewUri; postMessage-based communication
- `extensions/media-preview/media/videoPreview.js` — HTML5 video player wrapper; similar structure to audio preview

### Webview Styling
- `extensions/media-preview/media/imagePreview.css` — Styles for image container, zoom controls, pixelated rendering for scaled images
- `extensions/media-preview/media/audioPreview.css` — Audio player styling
- `extensions/media-preview/media/videoPreview.css` — Video player styling

### SVG Assets
- `extensions/media-preview/media/loading.svg` — Loading spinner
- `extensions/media-preview/media/loading-dark.svg` — Dark theme variant
- `extensions/media-preview/media/loading-hc.svg` — High contrast variant

---

## Configuration

### Extension Manifest
- `extensions/media-preview/package.json` — Declares three customEditors (imagePreview.previewEditor, vscode.audioPreview, vscode.videoPreview) with file pattern selectors (*.{jpg,jpeg,png...}, *.{mp3,wav...}, *.{mp4,webm}); commands (zoomIn, zoomOut, copyImage, reopen); menus (commandPalette, webview/context, editor/title); configuration schema for mediaPreview.video (autoPlay, loop); extensionKind [ui, workspace]; untrustedWorkspaces support

### Localization
- `extensions/media-preview/package.nls.json` — Display name and description translations for customEditors and commands

### Build Configuration
- `extensions/media-preview/tsconfig.json` — Compiles to ./out; includes vscode-dts/vscode.d.ts; extends base config
- `extensions/media-preview/tsconfig.browser.json` — Browser target for webview compilation
- `extensions/media-preview/esbuild.mts` — Node.js target build configuration for extension host
- `extensions/media-preview/esbuild.browser.mts` — Browser target build configuration for webview content
- `extensions/media-preview/.npmrc` — NPM configuration
- `extensions/media-preview/.vscodeignore` — Excludes source files from packaged extension

### Dependencies
- `extensions/media-preview/package.json` — @vscode/extension-telemetry (0.9.8), vscode-uri (3.0.6)

---

## Types / Interfaces

### Webview Panel Lifecycle
- `WebviewPanel` interface with properties: webview.html, webview.postMessage(), onDidReceiveMessage(), onDidChangeViewState(), onDidDispose(), active, viewColumn

### Custom Document Protocol
- `CustomDocument` — Simple document with uri and dispose()
- `CustomReadonlyEditorProvider<T>` — openCustomDocument(uri) and resolveCustomEditor(document, webviewEditor)

### Webview Options
- `webviewOptions.enableScripts: true` — Required for postMessage functionality
- `webviewOptions.localResourceRoots` — Array of Uri; restricts resource loading to extension paths
- `webviewOptions.retainContextWhenHidden: true` — Preserves webview DOM when tab is hidden

### Message Types
- Custom message protocol for image preview: { type: 'size'|'zoom'|'reopen-as-text', value?: any }
- Status bar messages: { type: 'setScale', scale: number }; { type: 'setActive', value: boolean }; { type: 'copyImage' }; { type: 'zoomIn'|'zoomOut' }

---

## Notable Clusters

### Multi-Editor State Management Pattern
- ImagePreviewManager tracks Set<ImagePreview> instances; tracks _activePreview; manages onDidChangeViewState and onDidDispose callbacks to sync state across multiple editor windows

### Webview Security Model
- Content Security Policy: default-src 'none'; img-src/media-src for resource URIs; script-src 'nonce-{uuid}'; style-src with CSP source
- HTML attributes escaped via escapeAttribute() to prevent injection
- enableScripts: true + localResourceRoots + CSP nonce-based script loading

### Resource URI Translation
- Extension resources: webviewEditor.webview.asWebviewUri(vscode.Uri.joinPath(extensionRoot, 'media', 'imagePreview.js'))
- User file resources: webviewEditor.webview.asWebviewUri(resource).with({ query: `version=${timestamp}` })
- Cache-busting via query parameter for file changes

### Disposable Resource Management
- Disposable base class with _register() pattern; automatic cleanup of EventEmitters and subscriptions
- MediaPreview registers file system watchers, webview listeners, and status bar entries; all disposed when editor closes

### Configuration Access Pattern
- vscode.workspace.getConfiguration('mediaPreview.video').get('autoPlay') — runtime settings
- Settings passed to webview via HTML data attribute: JSON.stringify({ src, isGitLfs, autoplay, loop })

---

## Architecture Implications for Tauri Port

### Critical Webview Adapter
The media-preview extension reveals that a Tauri port requires:
1. **WebviewPanel equivalent** — Full lifecycle management (create, show, postMessage, onMessage, viewColumn, onDidDispose)
2. **Resource URI translation** — asWebviewUri() for both extension resources and workspace files with cache-busting
3. **Message passing protocol** — Typed message objects between extension host and webview content
4. **Security boundaries** — CSP headers, nonce-based script execution, script enablement toggle

### Webview Content Scripts
- Browser-side code uses `acquireVsCodeApi()` to obtain vscode object; in Tauri this API surface must be replicated
- postMessage/onMessage bidirectional communication is fundamental to preview interactions

### Disposition Tracking
- Extension maintains Set of editor instances; Tauri equivalent must support multi-instance management per document

### File System Integration
- workspace.fs.stat() and workspace.fs.readFile() for resource inspection (size, Git LFS detection)
- workspace.createFileSystemWatcher() for real-time updates
- Dependency on file system API parity
