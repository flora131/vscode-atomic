# Mermaid Chat Features Extension - File Locator (Partition 35/79)

## Implementation Files

### Main Extension Logic
- `extensions/mermaid-chat-features/src/extension.ts` — Extension activation, command registration, webview management initialization
- `extensions/mermaid-chat-features/src/webviewManager.ts` — Tracks and manages mermaid webview lifecycle (both chat and editor types)
- `extensions/mermaid-chat-features/src/chatOutputRenderer.ts` — Chat output renderer implementation using VS Code API; registers custom renderer for MIME type `text/vnd.mermaid`; handles encoding/decoding mermaid data
- `extensions/mermaid-chat-features/src/editorManager.ts` — Manages mermaid diagram preview panels; implements `WebviewPanelSerializer` for persistence; ensures single editor per diagram

### Webview Source (Client-side)
- `extensions/mermaid-chat-features/chat-webview-src/index.ts` — Main entry point for chat webview; initializes mermaid rendering and open-in-editor button
- `extensions/mermaid-chat-features/chat-webview-src/index-editor.ts` — Entry point for editor webview; wires up zoom control buttons
- `extensions/mermaid-chat-features/chat-webview-src/mermaidWebview.ts` — Core webview rendering logic (410 lines); contains `PanZoomHandler` class with pan/zoom state management, event listeners for mouse/wheel/keyboard, SVG centering, persistence via VS Code API
- `extensions/mermaid-chat-features/chat-webview-src/vscodeApi.ts` — Type definition for VsCodeApi interface exposing getState/setState/postMessage

### Utilities
- `extensions/mermaid-chat-features/src/util/html.ts` — HTML text escaping utility
- `extensions/mermaid-chat-features/src/util/uuid.ts` — UUID generation using crypto.randomUUID with fallback
- `extensions/mermaid-chat-features/src/util/dispose.ts` — Disposable pattern implementation; provides base `Disposable` class and `disposeAll` function

## Configuration Files

- `extensions/mermaid-chat-features/package.json` — Extension manifest; defines chat output renderer (viewType: `vscode.chat-mermaid-features.chatOutputItem`), language model tool `renderMermaidDiagram`, activation event, capabilities, dependencies (mermaid 11.12.3, dompurify 3.4.0, @vscode/codicons)
- `extensions/mermaid-chat-features/tsconfig.json` — TypeScript configuration
- `extensions/mermaid-chat-features/tsconfig.browser.json` — TypeScript configuration for browser/webview builds
- `extensions/mermaid-chat-features/chat-webview-src/tsconfig.json` — TypeScript configuration for webview source

## Build Configuration

- `extensions/mermaid-chat-features/esbuild.webview.mts` — Builds chat webview bundles (index.ts → index.js, index-editor.ts → index-editor.js, codicon.css); uses esbuild-webview-common
- `extensions/mermaid-chat-features/esbuild.browser.mts` — Browser/web build configuration
- `extensions/mermaid-chat-features/esbuild.mts` — Extension build configuration

## Metadata & Configuration

- `extensions/mermaid-chat-features/cgmanifest.json` — Component governance manifest
- `extensions/mermaid-chat-features/.npmrc` — npm configuration
- `extensions/mermaid-chat-features/.vscodeignore` — Files to exclude from packaged extension
- `extensions/mermaid-chat-features/.gitignore` — Git ignore patterns
- `extensions/mermaid-chat-features/package-lock.json` — Locked dependencies
- `extensions/mermaid-chat-features/package.nls.json` — Localization strings (displayName, description, config descriptions)

## Documentation

- `extensions/mermaid-chat-features/README.md` — Simple notice that extension is bundled with VS Code and describes basic function

## Notable Architecture Patterns

**Webview-Heavy Design**: The extension uses two separate webview entry points (chat vs editor) compiled to separate JS bundles. The webview client code uses `acquireVsCodeApi()` to communicate bidirectionally with the extension host.

**State Persistence**: Pan/zoom state is persisted via VS Code's webview state mechanism (`vscode.getState()/setState()`), allowing state recovery across sessions.

**Dual Registration System**: Webviews are tracked by the `MermaidWebviewManager` which maintains a map of all active webviews (both chat and editor types), with an active webview concept for command routing.

**MIME Type Routing**: Chat integration uses custom MIME type `text/vnd.mermaid` to identify mermaid output requiring special rendering; chat output renderer is registered via `vscode.chat.registerChatOutputRenderer()`.

**Pan/Zoom Implementation**: Client-side `PanZoomHandler` class in webview provides comprehensive pan/zoom via Alt+drag, Alt+click, Alt+Shift+click, pinch gestures, and wheel events; maintains SVG centering and cursor feedback based on keyboard modifiers.

**HTML Templating in Extensions**: The extension generates inline HTML with embedded scripts in TypeScript (webviewManager and editorManager), setting nonce-based CSP policy and rendering mermaid diagrams via `<pre class="mermaid">` tags.

---

**Partition Summary**: The mermaid-chat-features extension demonstrates webview-intensive patterns critical to understanding VS Code IDE functionality ports to alternative frameworks. Key porting considerations include: (1) webview creation and lifecycle management through extension API, (2) bidirectional messaging protocol between extension host and webview, (3) web-based pannable/zoomable SVG rendering, (4) state persistence mechanisms, and (5) CSS/DOM-driven theming and styling tied to VS Code color tokens.
