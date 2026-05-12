# Partition 31 of 80 — Findings

## Scope
`extensions/media-preview/` (17 files, 1,553 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

1. `extensions/media-preview/src/extension.ts`
2. `extensions/media-preview/src/mediaPreview.ts`
3. `extensions/media-preview/src/imagePreview/index.ts`
4. `extensions/media-preview/src/audioPreview.ts`
5. `extensions/media-preview/src/videoPreview.ts`
6. `extensions/media-preview/src/binarySizeStatusBarEntry.ts`
7. `extensions/media-preview/src/ownedStatusBarEntry.ts`
8. `extensions/media-preview/src/imagePreview/zoomStatusBarEntry.ts`
9. `extensions/media-preview/media/imagePreview.js`
10. `extensions/media-preview/src/util/dispose.ts`
11. `extensions/media-preview/src/util/dom.ts`
12. `extensions/media-preview/src/util/uuid.ts`

---

### Per-File Notes

#### `extensions/media-preview/src/extension.ts`

- **Role:** Activation entry point. Instantiates shared infrastructure and delegates registration to the three media-type modules.
- **Key symbols:**
  - `activate(context)` at line 12 — sole exported function; called by the VS Code host when the extension activates.
  - `BinarySizeStatusBarEntry` instantiated at line 13 — a single shared instance passed to all three preview registrars.
- **Control flow:** `activate` constructs one `BinarySizeStatusBarEntry`, then calls `registerImagePreviewSupport`, `registerAudioPreviewSupport`, and `registerVideoPreviewSupport` (lines 16–18), each returning a `vscode.Disposable` that is pushed onto `context.subscriptions`.
- **Data flow:** `context.extensionUri` flows into each `register*` call; `binarySizeStatusBarEntry` is shared so all three media types write to the same status-bar slot.
- **Dependencies:** `vscode` API, `./binarySizeStatusBarEntry`, `./imagePreview`, `./audioPreview`, `./videoPreview`.

---

#### `extensions/media-preview/src/mediaPreview.ts`

- **Role:** Abstract base class `MediaPreview` holding all behaviour common to image, audio, and video previews: webview option configuration, file-system watching, state transitions, binary-size tracking, and the render pipeline.
- **Key symbols:**
  - `PreviewState` enum at line 36 — three values: `Disposed`, `Visible`, `Active`.
  - `MediaPreview` abstract class at line 42.
  - Constructor at line 47 — sets `webview.options` (line 57–64): `enableScripts: true`, `enableForms: false`, `localResourceRoots` restricted to the resource's parent directory and the extension root.
  - `render()` at line 106 — guards on `PreviewState.Disposed`, calls abstract `getWebviewContents()`, assigns result to `this._webviewEditor.webview.html` (line 116).
  - `getWebviewContents()` abstract at line 119 — must be overridden by subclasses to produce the HTML string.
  - `updateState()` at line 121 — reads `_webviewEditor.active` to transition between `Active` and `Visible`, showing/hiding the `BinarySizeStatusBarEntry`.
  - `updateBinarySize()` at line 99 — calls `vscode.workspace.fs.stat` asynchronously to get the file size in bytes, stores to `_binarySize`, then calls `updateState`.
  - File-system watcher at line 75 — `vscode.workspace.createFileSystemWatcher` on a `RelativePattern` matching the resource; `onDidChange` calls `updateBinarySize` then `render`; `onDidDelete` disposes the webview panel (line 86).
  - `reopenAsText()` at line 11 — standalone async function; executes the `vscode.openWith` command with `'default'` editor id.
  - `isGitLfsPointer()` at line 17 — checks `resource.scheme === 'git'`, stats the file (max 1024 bytes), reads content, checks for the LFS pointer prefix string.
- **Control flow:** Constructor → configure webview options → register view-state change listener → register dispose listener → register filesystem watcher. `render()` is called by subclass constructors and by the watcher's `onDidChange` handler.
- **Data flow:** `vscode.Uri` resource flows in at construction. `vscode.workspace.fs.stat` provides the byte count. `getWebviewContents()` returns an HTML string written directly to `webviewEditor.webview.html`.
- **Dependencies:** `vscode`, `vscode-uri` (Utils.dirname), `./binarySizeStatusBarEntry`, `./util/dispose`.

---

#### `extensions/media-preview/src/imagePreview/index.ts`

- **Role:** Implements the `vscode.CustomReadonlyEditorProvider` interface for image files; also registers five commands (`zoomIn`, `zoomOut`, `copyImage`, `reopenAsText`, `reopenAsPreview`).
- **Key symbols:**
  - `ImagePreviewManager` class at line 15 — implements `vscode.CustomReadonlyEditorProvider`.
  - `viewType = 'imagePreview.previewEditor'` at line 17.
  - `openCustomDocument(uri)` at line 29 — minimal implementation returning `{ uri, dispose: () => {} }`.
  - `resolveCustomEditor(document, webviewEditor)` at line 33 — constructs `ImagePreview`, tracks it in `_previews` Set, sets the active preview.
  - `ImagePreview` class at line 73 — extends `MediaPreview`.
  - Message handler at line 88: handles `'size'` (stores dimension string, calls `updateState`), `'zoom'` (stores scale, calls `updateState`), `'reopen-as-text'` (delegates to `reopenAsText`).
  - Bidirectional message at line 107: `zoomStatusBarEntry.onDidChangeScale` fires `postMessage({ type: 'setScale', scale })` to the webview; `onDidChangeViewState` fires `postMessage({ type: 'setActive', value })`.
  - `getWebviewContents()` at line 180 — builds the HTML string with nonce, CSP header, settings encoded in a `<meta data-settings>` attribute.
  - CSP string at line 204: `default-src 'none'; img-src data: ${cspSource}; connect-src ${cspSource}; script-src 'nonce-${nonce}'; style-src ${cspSource} 'nonce-${nonce}'`.
  - `getResourcePath()` at line 222 — calls `isGitLfsPointer`; if LFS returns `null`; otherwise calls `webviewEditor.webview.asWebviewUri(resource)` and appends a `version` query parameter for cache busting.
  - `extensionResource()` at line 234 — wraps `vscode.Uri.joinPath(extensionRoot, ...parts)` through `asWebviewUri` to translate extension-local paths into webview-accessible URIs.
  - `registerImagePreviewSupport()` at line 245 — creates status bar entries, the manager, calls `vscode.window.registerCustomEditorProvider` (line 256), registers five commands, returns `vscode.Disposable.from(...disposables)`.
- **Control flow:** `registerImagePreviewSupport` → `registerCustomEditorProvider` (viewType, manager, `{supportsMultipleEditorsPerDocument: true}`) → on open: `resolveCustomEditor` → `new ImagePreview` → `updateBinarySize`, `render`, `updateState`.
- **Data flow:** File URI → `getResourcePath` → webview `src` → JSON-serialized `settings` embedded in `<meta data-settings>` → read by `imagePreview.js:19-28`. Webview posts `{ type: 'size', value: 'WxH' }` back → stored in `_imageSize` → displayed by `SizeStatusBarEntry`. Webview posts `{ type: 'zoom', value: scale }` → `_imageZoom` → `ZoomStatusBarEntry`.
- **Dependencies:** `vscode`, `../binarySizeStatusBarEntry`, `../mediaPreview`, `../util/dom`, `../util/uuid`, `./sizeStatusBarEntry`, `./zoomStatusBarEntry`.

---

#### `extensions/media-preview/src/audioPreview.ts`

- **Role:** Implements `vscode.CustomReadonlyEditorProvider` for audio files. Simpler than image preview — no zoom or size status bar entries.
- **Key symbols:**
  - `AudioPreviewProvider` class at line 12 — viewType `'vscode.audioPreview'`.
  - `AudioPreview` class at line 31 — extends `MediaPreview`.
  - Message handler at line 41: only handles `'reopen-as-text'`.
  - `getWebviewContents()` at line 55 — structure identical to image preview but CSP includes `media-src ${cspSource}` instead of `connect-src`, and references `audioPreview.css` and `audioPreview.js`.
  - `registerAudioPreviewSupport()` at line 114 — passes `retainContextWhenHidden: true` in `webviewOptions` (so audio keeps playing when the tab is not visible).
- **Control flow:** Provider → `resolveCustomEditor` → `new AudioPreview` → `updateBinarySize`, `render`, `updateState`.
- **Data flow:** Same settings meta-tag pattern; `src` may be `null` for LFS pointers. No message-back channels beyond `reopen-as-text`.
- **Dependencies:** `vscode`, `./binarySizeStatusBarEntry`, `./mediaPreview`, `./util/dom`, `./util/uuid`.

---

#### `extensions/media-preview/src/videoPreview.ts`

- **Role:** Implements `vscode.CustomReadonlyEditorProvider` for video files. Adds two workspace-configuration reads for autoplay and loop.
- **Key symbols:**
  - `VideoPreviewProvider` class at line 13 — viewType `'vscode.videoPreview'`.
  - `VideoPreview` class at line 32 — extends `MediaPreview`.
  - `getWebviewContents()` at line 56 — reads `mediaPreview.video` configuration at line 58: `autoPlay` and `loop` booleans added to the `settings` object embedded in the meta tag.
  - CSP at line 83: same as audio, `media-src ${cspSource}`.
  - `registerVideoPreviewSupport()` at line 118 — `retainContextWhenHidden: true`.
- **Control flow:** Same as audio preview.
- **Data flow:** Configuration values flow into the HTML settings meta tag, which the `videoPreview.js` script reads to configure the `<video>` element.
- **Dependencies:** `vscode`, `./binarySizeStatusBarEntry`, `./mediaPreview`, `./util/dom`, `./util/uuid`.

---

#### `extensions/media-preview/src/binarySizeStatusBarEntry.ts`

- **Role:** Concrete status bar entry that formats a raw byte count into a human-readable string and delegates display to `PreviewStatusBarEntry`.
- **Key symbols:**
  - `BinarySize` helper class at line 10 — static constants KB/MB/GB/TB and `formatSize(size)` at line 16.
  - `BinarySizeStatusBarEntry` at line 37 — constructed with id `'status.imagePreview.binarySize'`, alignment Right, priority 100.
  - `show(owner, size)` at line 43 — only shows if `typeof size === 'number'`; otherwise hides.
- **Control flow:** `MediaPreview.updateState` calls `show(this, this._binarySize)` when active; calls `hide(this)` when not active or on dispose.
- **Data flow:** Byte count integer → `formatSize` → string passed to `PreviewStatusBarEntry.showItem`.
- **Dependencies:** `vscode`, `./ownedStatusBarEntry`.

---

#### `extensions/media-preview/src/ownedStatusBarEntry.ts`

- **Role:** Base class for all status bar entries in this extension. Implements an "ownership" pattern: only the last caller to `showItem` can hide the entry via `hide`.
- **Key symbols:**
  - `PreviewStatusBarEntry` abstract class at line 9.
  - `_showOwner: unknown` at line 10 — stores the reference that last called `showItem`.
  - `showItem(owner, text)` at line 20 — sets `_showOwner`, updates `entry.text`, calls `entry.show()`.
  - `hide(owner)` at line 26 — only hides if `owner === this._showOwner`; clears `_showOwner`.
  - Status bar item created at line 16 via `vscode.window.createStatusBarItem(id, alignment, priority)` and registered for disposal.
- **Control flow:** Multiple `ImagePreview` instances may exist for the same document; the ownership check ensures that when a non-active preview tries to hide the entry, it does not override the active preview's display.
- **Dependencies:** `vscode`, `./util/dispose`.

---

#### `extensions/media-preview/src/imagePreview/zoomStatusBarEntry.ts`

- **Role:** Zoom-level status bar entry that doubles as a command trigger. Clicking the item opens a `QuickPick` to select a zoom level; selection fires an `EventEmitter` that `ImagePreview` listens to.
- **Key symbols:**
  - `Scale` type at line 12: `number | 'fit'`.
  - `ZoomStatusBarEntry` at line 14 — extends `OwnedStatusBarEntry`, priority 102.
  - `_onDidChangeScale` / `onDidChangeScale` at lines 16–17 — `vscode.EventEmitter<{ scale: Scale }>`.
  - `selectZoomLevelCommandId = '_imagePreview.selectZoomLevel'` at line 10 — internal command registered at line 22.
  - QuickPick items at line 25: scales `[10, 5, 2, 1, 0.5, 0.2, 'fit']`.
  - `this.entry.command = selectZoomLevelCommandId` at line 39 — clicking the status bar item triggers the QuickPick.
  - `show(owner, scale)` at line 42 — delegates to `showItem` with formatted label.
  - `zoomLabel(scale)` at line 46 — returns `'Whole Image'` for `'fit'`, or `'${Math.round(scale * 100)}%'`.
- **Control flow:** User clicks status bar → command fires → QuickPick shown → user picks → `_onDidChangeScale.fire({ scale })` → `ImagePreview` listener at `index.ts:107` posts `{ type: 'setScale', scale }` to webview.
- **Dependencies:** `vscode`, `../ownedStatusBarEntry`.

---

#### `extensions/media-preview/media/imagePreview.js`

- **Role:** Webview content script. Runs inside the sandboxed webview iframe. Handles image display, zoom interactions (click, wheel/pinch, keyboard), state persistence via `vscode.getState`/`setState`, clipboard copy, and bidirectional messaging with the extension host.
- **Key symbols:**
  - IIFE wrapper at line 8.
  - `getSettings()` at line 19 — reads `document.getElementById('image-preview-settings').getAttribute('data-settings')` and JSON-parses it.
  - Constants at lines 34–59: `PIXELATION_THRESHOLD = 3`, `SCALE_PINCH_FACTOR = 0.075`, `MAX_SCALE = 20`, `MIN_SCALE = 0.1`, `zoomLevels` array.
  - `acquireVsCodeApi()` at line 65 — VS Code webview bridge injection.
  - State restoration at line 67: `vscode.getState() || { scale: 'fit', offsetX: 0, offsetY: 0 }`.
  - `updateScale(newScale)` at line 81 — CSS zoom manipulation: for `'fit'` mode removes explicit dimensions and sets `image.style.zoom = 'normal'` (line 93), clears state (line 95); for numeric scale clamps to range, applies `image.style.zoom = scale` (line 120), scroll-centres (lines 122–125), persists state via `vscode.setState` (line 127), and posts `{ type: 'zoom', value: scale }` back to host (line 130).
  - `firstZoom()` at line 154 — converts from `'fit'` to a numeric scale by computing `image.clientWidth / image.naturalWidth` (line 160); falls back to `1` for SVGs with only a viewBox.
  - `zoomIn()` / `zoomOut()` at lines 170 / 184 — walk the `zoomLevels` array to find the next discrete level.
  - Mouse/key/wheel listeners at lines 198–296 — coordinate `ctrlPressed`/`altPressed` state, invoke zoom in/out, prevent browser pinch-to-zoom via `e.preventDefault()` (line 268).
  - `image.addEventListener('load', ...)` at line 303 — sets `hasLoadedImage = true`, posts `{ type: 'size', value: '${naturalWidth}x${naturalHeight}' }` (lines 309–312), removes `loading` class, appends image to body, calls `updateScale`.
  - LFS/error handling at lines 337–347: sets body class directly from `settings.isGitLfs` / `settings.src === null`.
  - Incoming message handler at line 356 — origin check at line 357, switches on `e.data.type`: `'setScale'` → `updateScale`, `'setActive'` → `setActive`, `'zoomIn'` → `zoomIn`, `'zoomOut'` → `zoomOut`, `'copyImage'` → `copyImage`.
  - `copyImage(retries)` at line 390 — retry-with-delay loop waiting for `document.hasFocus()`; uses `navigator.clipboard.write` with a `canvas`-derived PNG blob.
- **Control flow:** On load → read settings → if LFS/null: set error class; else set `image.src`. On load event: post `size` message, call `updateScale(initialState.scale)`. On messages from host: dispatch to appropriate handler.
- **Data flow:** Extension injects `src` URL and `isGitLfs` flag via `<meta data-settings>`. Script reads, sets `image.src`. Image dimensions reported back via `postMessage`. Zoom state round-trips: host → webview via `'setScale'` message; webview → host via `'zoom'` message. State persisted across reloads via `vscode.getState/setState`.
- **Dependencies:** `acquireVsCodeApi` (injected by VS Code webview runtime), `navigator.clipboard`, `crypto`, browser DOM APIs.

---

#### `extensions/media-preview/src/util/dispose.ts`

- **Role:** Generic disposable base class used throughout the extension for subscription lifecycle management.
- **Key symbols:**
  - `disposeAll(disposables)` at line 8 — drains array calling `dispose()` on each item.
  - `Disposable` abstract class at line 17 — `_register<T>(value)` at line 30 pushes into `_disposables` or immediately disposes if already disposed; `dispose()` at line 22 sets `_isDisposed`, calls `disposeAll`.
- **Dependencies:** `vscode` (for `vscode.Disposable` type).

---

#### `extensions/media-preview/src/util/dom.ts`

- **Role:** Single utility function for HTML attribute escaping.
- **Key symbols:**
  - `escapeAttribute(value)` at line 7 — replaces `"` with `&quot;` in URI or string values before embedding in HTML attribute context.
- **Dependencies:** `vscode` (for `vscode.Uri` type parameter).

---

#### `extensions/media-preview/src/util/uuid.ts`

- **Role:** UUID v4 generator; used to produce a per-render nonce for the Content Security Policy.
- **Key symbols:**
  - `generateUuid()` at line 9 — uses `crypto.randomUUID()` if available; otherwise manually generates using `crypto.getRandomValues` with RFC 4122 version/variant bits set at lines 31–32.
- **Dependencies:** global `crypto` Web API.

---

### Cross-Cutting Synthesis

The media-preview extension exposes a three-tier architecture whose central abstraction is the VS Code `CustomReadonlyEditorProvider` contract. Each media type (image/audio/video) has a Provider class that handles the two-method lifecycle (`openCustomDocument` / `resolveCustomEditor`) and an inner Preview class that extends the shared `MediaPreview` base. The base class owns webview option configuration — crucially the `localResourceRoots` sandbox restriction and script-enabling flag — along with a file-system watcher that re-renders on change and disposes the panel on deletion. Rendering is a one-shot HTML string write: the extension host generates a full HTML document containing a nonce-locked CSP header, a `<meta data-settings>` tag carrying JSON-serialised configuration (resource URI, LFS flag, optional video autoPlay/loop), and a `<script nonce=...>` tag loading the compiled webview script. The resource URI is translated from a `vscode.Uri` into a webview-accessible `https://` URI through `webviewPanel.webview.asWebviewUri`, both for the media asset and for extension-local JS/CSS files. All interactive state (zoom level, scroll offsets) is persisted across reloads via `vscode.getState`/`setState` from within the webview, and communicated to the host via `vscode.postMessage`, with the host responding through `webview.postMessage`. Status bar entries use an ownership token pattern (`PreviewStatusBarEntry._showOwner`) to prevent non-active preview instances from hiding entries owned by the active one. For a Tauri port, every element of this contract must be reproduced: the two-phase document/editor lifecycle, the URI translation system mapping local files to webview-safe URIs, the bidirectional JSON message bus, the CSP nonce mechanism, `retainContextWhenHidden` semantics for media continuity, and the `vscode.workspace.fs.stat`/`readFile` interface for LFS pointer detection.

---

### Out-of-Partition References

- `vscode` API namespace (entire extension API surface) — defined in `src/vscode-dts/vscode.d.ts` and implemented in `src/vs/workbench/api/`.
- `vscode-uri` package (`Utils.dirname`) — third-party utility used in `mediaPreview.ts:7`.
- `extensions/media-preview/media/audioPreview.js` and `extensions/media-preview/media/videoPreview.js` — webview content scripts analogous to `imagePreview.js` but not read in this partition (not among the most central for Tauri porting analysis).
- `extensions/media-preview/media/imagePreview.css`, `audioPreview.css`, `videoPreview.css` — CSS loaded inside the webview under the nonce-locked CSP.
- `vscode.window.registerCustomEditorProvider` implementation — lives in `src/vs/workbench/api/browser/mainThreadCustomEditors.ts`.
- `vscode.workspace.createFileSystemWatcher` implementation — `src/vs/workbench/api/common/extHostFileSystemEventService.ts`.
- `vscode.commands.executeCommand('vscode.openWith', ...)` and `'reopenActiveEditorWith'` — handled inside VS Code's editor service (`src/vs/workbench/services/editor/`).

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Custom Editor Provider Patterns - Media Preview Extension

## Pattern Overview

The media-preview extension implements VS Code's custom editor provider system to render image, audio, and video files in specialized webview-based editors. These patterns demonstrate how to build custom editor integrations that replace the default binary editor with domain-specific UIs.

---

## Patterns Found

#### Pattern: Basic Custom Editor Provider Implementation
**Where:** `extensions/media-preview/src/audioPreview.ts:12-28`
**What:** Minimal CustomReadonlyEditorProvider implementing two required methods for stateless preview handlers.
```typescript
class AudioPreviewProvider implements vscode.CustomReadonlyEditorProvider {

	public static readonly viewType = 'vscode.audioPreview';

	constructor(
		private readonly extensionRoot: vscode.Uri,
		private readonly binarySizeStatusBarEntry: BinarySizeStatusBarEntry,
	) { }

	public async openCustomDocument(uri: vscode.Uri) {
		return { uri, dispose: () => { } };
	}

	public async resolveCustomEditor(document: vscode.CustomDocument, webviewEditor: vscode.WebviewPanel): Promise<void> {
		new AudioPreview(this.extensionRoot, document.uri, webviewEditor, this.binarySizeStatusBarEntry);
	}
}
```
**Variations / call-sites:** `extensions/media-preview/src/videoPreview.ts:13-29` (identical pattern for video).

---

#### Pattern: Provider Registration with Options
**Where:** `extensions/media-preview/src/audioPreview.ts:114-122`
**What:** Registering a custom editor provider with webview persistence options to control context retention.
```typescript
export function registerAudioPreviewSupport(context: vscode.ExtensionContext, binarySizeStatusBarEntry: BinarySizeStatusBarEntry): vscode.Disposable {
	const provider = new AudioPreviewProvider(context.extensionUri, binarySizeStatusBarEntry);
	return vscode.window.registerCustomEditorProvider(AudioPreviewProvider.viewType, provider, {
		supportsMultipleEditorsPerDocument: true,
		webviewOptions: {
			retainContextWhenHidden: true,
		}
	});
}
```
**Variations / call-sites:** `extensions/media-preview/src/videoPreview.ts:118-126` (same pattern); `extensions/media-preview/src/imagePreview/index.ts:256-258` (simpler options without retainContextWhenHidden).

---

#### Pattern: Stateful Preview Manager with Active Tracking
**Where:** `extensions/media-preview/src/imagePreview/index.ts:15-70`
**What:** Custom editor provider that tracks multiple preview instances and maintains active editor state across webview panels.
```typescript
export class ImagePreviewManager implements vscode.CustomReadonlyEditorProvider {

	public static readonly viewType = 'imagePreview.previewEditor';

	private readonly _previews = new Set<ImagePreview>();
	private _activePreview: ImagePreview | undefined;

	constructor(
		private readonly extensionRoot: vscode.Uri,
		private readonly sizeStatusBarEntry: SizeStatusBarEntry,
		private readonly binarySizeStatusBarEntry: BinarySizeStatusBarEntry,
		private readonly zoomStatusBarEntry: ZoomStatusBarEntry,
	) { }

	public async openCustomDocument(uri: vscode.Uri) {
		return { uri, dispose: () => { } };
	}

	public async resolveCustomEditor(
		document: vscode.CustomDocument,
		webviewEditor: vscode.WebviewPanel,
	): Promise<void> {
		const preview = new ImagePreview(this.extensionRoot, document.uri, webviewEditor, this.sizeStatusBarEntry, this.binarySizeStatusBarEntry, this.zoomStatusBarEntry);
		this._previews.add(preview);
		this.setActivePreview(preview);

		webviewEditor.onDidDispose(() => { this._previews.delete(preview); });

		webviewEditor.onDidChangeViewState(() => {
			if (webviewEditor.active) {
				this.setActivePreview(preview);
			} else if (this._activePreview === preview && !webviewEditor.active) {
				this.setActivePreview(undefined);
			}
		});
	}
```
**Variations / call-sites:** Audio and video providers use simpler stateless approach (single instance per document); image preview shows managing multiple panels per document.

---

#### Pattern: Webview Message Handling and Command Integration
**Where:** `extensions/media-preview/src/imagePreview/index.ts:88-105`
**What:** Handling bidirectional webview-to-extension messaging and mapping to extension commands.
```typescript
this._register(webviewEditor.webview.onDidReceiveMessage(message => {
	switch (message.type) {
		case 'size': {
			this._imageSize = message.value;
			this.updateState();
			break;
		}
		case 'zoom': {
			this._imageZoom = message.value;
			this.updateState();
			break;
		}
		case 'reopen-as-text': {
			reopenAsText(resource, webviewEditor.viewColumn);
			break;
		}
	}
}));
```
**Variations / call-sites:** `extensions/media-preview/src/audioPreview.ts:41-48` (simpler single-case handler); `extensions/media-preview/src/videoPreview.ts:42-49` (same pattern as audio).

---

#### Pattern: Webview Content Generation with Security Headers
**Where:** `extensions/media-preview/src/imagePreview/index.ts:180-220`
**What:** Generating HTML content for webview with Content-Security-Policy, nonce-based script injection, and resource URI conversion.
```typescript
protected override async getWebviewContents(): Promise<string> {
	const version = Date.now().toString();
	const src = await this.getResourcePath(this._webviewEditor, this._resource, version);
	const settings = {
		src,
		isGitLfs: src === null,
	};

	const nonce = generateUuid();

	const cspSource = this._webviewEditor.webview.cspSource;
	return /* html */`<!DOCTYPE html>
<html lang="en">
<head>
	<meta charset="UTF-8">

	<!-- Disable pinch zooming -->
	<meta name="viewport"
		content="width=device-width, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0, user-scalable=no">

	<title>Image Preview</title>

	<link rel="stylesheet" href="${escapeAttribute(this.extensionResource('media', 'imagePreview.css'))}" type="text/css" media="screen" nonce="${nonce}">

	<meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src data: ${cspSource}; connect-src ${cspSource}; script-src 'nonce-${nonce}'; style-src ${cspSource} 'nonce-${nonce}';">
	<meta id="image-preview-settings" data-settings="${escapeAttribute(JSON.stringify(settings))}">
</head>
<body class="container image scale-to-fit loading" data-vscode-context='{ "preventDefaultContextMenuItems": true }'>
	<div class="loading-indicator"></div>
	<script src="${escapeAttribute(this.extensionResource('media', 'imagePreview.js'))}" nonce="${nonce}"></script>
</body>
</html>`;
}
```
**Variations / call-sites:** `extensions/media-preview/src/audioPreview.ts:55-95` (same pattern with audio media-src CSP); `extensions/media-preview/src/videoPreview.ts:56-98` (adds video configuration settings).

---

#### Pattern: Resource URI Conversion with Cache Busting
**Where:** `extensions/media-preview/src/imagePreview/index.ts:222-232`
**What:** Converting file URIs to webview-accessible URIs with optional version query parameter for cache invalidation.
```typescript
private async getResourcePath(webviewEditor: vscode.WebviewPanel, resource: vscode.Uri, version: string): Promise<string | null> {
	if (await isGitLfsPointer(resource)) {
		return null;
	}

	// Avoid adding cache busting if there is already a query string
	if (resource.query) {
		return webviewEditor.webview.asWebviewUri(resource).toString();
	}
	return webviewEditor.webview.asWebviewUri(resource).with({ query: `version=${version}` }).toString();
}
```
**Variations / call-sites:** `extensions/media-preview/src/audioPreview.ts:97-107` (identical); `extensions/media-preview/src/videoPreview.ts:101-111` (identical).

---

#### Pattern: Extension Registration and Activation
**Where:** `extensions/media-preview/src/extension.ts:12-19`
**What:** Activating multiple custom editor providers from a single extension entry point using shared status bar utilities.
```typescript
export function activate(context: vscode.ExtensionContext) {
	const binarySizeStatusBarEntry = new BinarySizeStatusBarEntry();
	context.subscriptions.push(binarySizeStatusBarEntry);

	context.subscriptions.push(registerImagePreviewSupport(context, binarySizeStatusBarEntry));
	context.subscriptions.push(registerAudioPreviewSupport(context, binarySizeStatusBarEntry));
	context.subscriptions.push(registerVideoPreviewSupport(context, binarySizeStatusBarEntry));
}
```
**Variations / call-sites:** Each media type (image, audio, video) has its own register function called here.

---

#### Pattern: Base Media Preview with Disposable Pattern
**Where:** `extensions/media-preview/src/mediaPreview.ts:42-88`
**What:** Abstract base class providing common lifecycle, file watching, and webview configuration for all preview types.
```typescript
export abstract class MediaPreview extends Disposable {

	protected previewState = PreviewState.Visible;
	private _binarySize: number | undefined;

	constructor(
		extensionRoot: vscode.Uri,
		protected readonly _resource: vscode.Uri,
		protected readonly _webviewEditor: vscode.WebviewPanel,
		private readonly _binarySizeStatusBarEntry: BinarySizeStatusBarEntry,
	) {
		super();

		const resourceRoot = Utils.dirname(_resource).with({ query: '', fragment: '' });

		_webviewEditor.webview.options = {
			enableScripts: true,
			enableForms: false,
			localResourceRoots: [
				resourceRoot,
				extensionRoot,
			]
		};

		this._register(_webviewEditor.onDidChangeViewState(() => {
			this.updateState();
		}));

		this._register(_webviewEditor.onDidDispose(() => {
			this.previewState = PreviewState.Disposed;
			this.dispose();
		}));

		const watcher = this._register(vscode.workspace.createFileSystemWatcher(new vscode.RelativePattern(_resource, '*')));
		this._register(watcher.onDidChange(e => {
			if (e.toString() === this._resource.toString()) {
				this.updateBinarySize();
				this.render();
			}
		}));
	}
```
**Variations / call-sites:** Extended by `ImagePreview`, `AudioPreview`, `VideoPreview` classes.

---

## Key Architectural Insights

### Webview-to-Tauri Bridge Implications

The custom editor pattern establishes a clear **two-way communication channel**:

1. **Registration Contract**: Provider implements `CustomReadonlyEditorProvider` interface with `openCustomDocument()` and `resolveCustomEditor()` methods
2. **Lifecycle Management**: Each preview instance manages webview lifecycle events (viewState changes, disposal, file watching)
3. **Message Protocol**: Uses `onDidReceiveMessage()` for webview→extension communication and `postMessage()` for extension→webview commands
4. **Security Model**: CSP headers, nonce-based script injection, and restricted resource roots demonstrate defense requirements
5. **State Synchronization**: Tracks preview state (Disposed/Visible/Active) across multiple panels and communicates to status bar UI

### Porting Considerations

- The `CustomReadonlyEditorProvider` contract maps to Tauri webview lifecycle hooks
- Resource URI conversion (`asWebviewUri`) requires equivalent path handling in Tauri
- Message passing is native to both Tauri webviews and VS Code; protocol identical
- CSP and nonce patterns directly transfer to Tauri; security model aligns
- File watcher integration (`createFileSystemWatcher`) needs Tauri file system monitoring equivalent
- Status bar integration requires Tauri-based UI composition

---

## File References

- `/home/norinlavaee/projects/vscode-atomic/extensions/media-preview/src/extension.ts` — Extension entry point
- `/home/norinlavaee/projects/vscode-atomic/extensions/media-preview/src/imagePreview/index.ts` — Stateful image preview with manager
- `/home/norinlavaee/projects/vscode-atomic/extensions/media-preview/src/audioPreview.ts` — Simple audio provider
- `/home/norinlavaee/projects/vscode-atomic/extensions/media-preview/src/videoPreview.ts` — Video provider with configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/media-preview/src/mediaPreview.ts` — Base preview class with lifecycle
- `/home/norinlavaee/projects/vscode-atomic/extensions/media-preview/src/util/dispose.ts` — Disposable pattern implementation

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
