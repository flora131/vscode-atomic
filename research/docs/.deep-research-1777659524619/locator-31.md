# Partition 31: Media Preview Extension
## Custom Editor & Webview Integration for Binary Previews

### Implementation
- `extensions/media-preview/src/extension.ts` — Extension activation entry point; registers three custom editor providers for binary media (image, audio, video)
- `extensions/media-preview/src/mediaPreview.ts` — Abstract base class `MediaPreview` implementing webview lifecycle, file watching, state management, and Content-Security-Policy setup for media preview editors
- `extensions/media-preview/src/imagePreview/index.ts` — Image preview manager implementing `CustomReadonlyEditorProvider`; registers `vscode.window.registerCustomEditorProvider()` with webview panel resolution; handles zoom, size reporting, message passing
- `extensions/media-preview/src/videoPreview.ts` — Video preview provider implementing `CustomReadonlyEditorProvider`; uses webview API for media-src CSP and configuration (autoplay, loop)
- `extensions/media-preview/src/audioPreview.ts` — Audio preview provider implementing `CustomReadonlyEditorProvider`; webview-based audio player with message passing protocol
- `extensions/media-preview/src/binarySizeStatusBarEntry.ts` — Status bar display component for file size reporting; extends `PreviewStatusBarEntry`
- `extensions/media-preview/src/ownedStatusBarEntry.ts` — Abstract base class for status bar items owned by preview instances; manages show/hide lifecycle
- `extensions/media-preview/src/imagePreview/sizeStatusBarEntry.ts` — Image-specific status bar entry for dimension display
- `extensions/media-preview/src/imagePreview/zoomStatusBarEntry.ts` — Image zoom control status bar entry; emits scale change events via `EventEmitter`
- `extensions/media-preview/src/util/dom.ts` — HTML attribute escaping utility for CSP nonce injection
- `extensions/media-preview/src/util/dispose.ts` — Resource disposal pattern base class; manages lifecycle of registered disposables
- `extensions/media-preview/src/util/uuid.ts` — UUID generation for CSP nonce values (uses `crypto.randomUUID()` with fallback)

### Configuration
- `extensions/media-preview/package.json` — Extension manifest defining three `customEditors` contributions (imagePreview, audioPreview, videoPreview); file patterns for jpg/jpeg/png/bmp/gif/ico/webp/avif/mp3/wav/ogg/mp4/webm; configuration properties for video autoplay/loop; menu contributions
- `extensions/media-preview/tsconfig.json` — TypeScript compiler config extending base configuration; targets Node.js environment
- `extensions/media-preview/tsconfig.browser.json` — Browser-specific TypeScript config for web platform build
- `extensions/media-preview/esbuild.mts` — Node.js platform build configuration using esbuild
- `extensions/media-preview/esbuild.browser.mts` — Browser platform build configuration for webview/web scenarios
- `extensions/media-preview/.vscodeignore` — Exclusion patterns for extension packaging
- `extensions/media-preview/.npmrc` — NPM registry configuration
- `extensions/media-preview/package-lock.json` — Dependency lock file

### Webview Assets / Media
- `extensions/media-preview/media/imagePreview.js` — Client-side webview script for image rendering; implements zoom (fit/percentage), pan, pixel-perfect scaling, copy-to-clipboard, size reporting via message protocol; uses `acquireVsCodeApi()` 
- `extensions/media-preview/media/videoPreview.js` — Client-side webview script for HTML5 video playback; `acquireVsCodeApi()` integration for control messages
- `extensions/media-preview/media/audioPreview.js` — Client-side webview script for HTML5 audio player; `acquireVsCodeApi()` message handling
- `extensions/media-preview/media/imagePreview.css` — Styling for image preview container, zoom UI, loading states
- `extensions/media-preview/media/videoPreview.css` — Styling for video player container and loading states
- `extensions/media-preview/media/audioPreview.css` — Styling for audio player container
- `extensions/media-preview/media/loading.svg` — Loading indicator (default theme)
- `extensions/media-preview/media/loading-dark.svg` — Loading indicator (dark theme)
- `extensions/media-preview/media/loading-hc.svg` — Loading indicator (high contrast theme)

### Documentation
- `extensions/media-preview/README.md` — Extension overview listing supported image formats (jpg, jpe, jpeg, png, bmp, gif, ico, webp, avif, svg), audio formats (mp3, wav, ogg, oga), video formats (mp4, webm)

### Notable Patterns & Relevance to Tauri/Rust Porting

**Custom Editor Provider Pattern**: All three media types implement `vscode.CustomReadonlyEditorProvider` interface with:
- `openCustomDocument(uri: vscode.Uri)` — Minimal custom document wrapper
- `resolveCustomEditor(document, webviewPanel)` — WebviewPanel injection point where binary preview UI is instantiated

**WebviewPanel Integration Points**:
- `webviewEditor.webview.html` assignment (line 93 in mediaPreview.ts) — Sets HTML content
- `webviewEditor.webview.postMessage()` — Extension-to-webview messaging (zoom, scale, state updates)
- `webviewEditor.webview.onDidReceiveMessage()` — Webview-to-extension messaging (size reports, reopen commands)
- `webviewEditor.webview.asWebviewUri()` — URI scheme conversion for resource loading
- `webviewEditor.webview.cspSource` — CSP source value for inline script nonces

**Security Model**:
- Content-Security-Policy meta tags in HTML templates (CSP no eval, img-src/media-src restricted)
- Nonce-based script execution using UUID generation
- `localResourceRoots` configured to allow access to extension root and file directory only

**State Management**:
- Disposable pattern for cleanup on editor close/file deletion
- File system watcher integration (`vscode.workspace.createFileSystemWatcher`)
- Preview state enum (Disposed, Visible, Active) tracking

**Status Bar Integration**:
- Status bar item creation via `vscode.window.createStatusBarItem()`
- Owner-based visibility (only show when preview is active)
- Priority-based positioning (101, 102 priorities for layering)

**Key File Size**: Extensions span 17 files across 1,507 LOC, demonstrating medium-complexity custom editor implementation with webview lifecycle, message passing, and resource management.

### Mapping to Porting Considerations

1. **Webview Replacement**: Tauri would need equivalent webview panel/window abstraction supporting message protocol (postMessage/onMessage bidirectional communication)
2. **Custom Editor Registration**: Extension system would need equivalent provider registration mechanism for file type associations
3. **Status Bar System**: Status bar item creation and lifecycle management needs Rust/Tauri equivalent
4. **URI Handling**: webviewUri() conversion and localResourceRoots ACL patterns would need Tauri equivalent
5. **Message Protocol**: Binary media preview uses JSON message passing; Tauri's invoke/listen mechanism provides parallel capability
6. **CSP & Security**: Nonce-based CSP enforcement would be replicated in Rust-generated webview initialization
7. **File System Integration**: File watching and stat operations use vscode.workspace API; would map to Rust fs crate with similar event patterns
8. **Async/Event Patterns**: Extensive use of EventEmitter and Promise-based operations; Rust async traits and channels would provide equivalents
