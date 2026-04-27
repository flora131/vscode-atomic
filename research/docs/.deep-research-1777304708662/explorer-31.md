# Partition 31 of 79 — Findings

## Scope
`extensions/media-preview/` (17 files, 1,507 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Partition 31: Media Preview Extension — Tauri/Rust Port Research

## Overview
This partition covers the media preview extension (image, audio, video custom editors) which demonstrates core VS Code webview infrastructure patterns critical to understanding what a Tauri/Rust port would need to replicate.

---

## Implementation

### Core Extension Logic
- `extensions/media-preview/src/extension.ts` — Extension activation entry point; registers three custom editor providers (image, audio, video) and initializes status bar entries

### Custom Editor Providers & Infrastructure
- `extensions/media-preview/src/imagePreview/index.ts` — ImagePreviewManager implementing `vscode.CustomReadonlyEditorProvider`; manages multiple image preview instances, handles zoom/size state via status bar; webview message passing for zoom/pan interactions
- `extensions/media-preview/src/audioPreview.ts` — AudioPreviewProvider implementing `vscode.CustomReadonlyEditorProvider`; lightweight webview-based audio player with HTML5 audio element
- `extensions/media-preview/src/videoPreview.ts` — VideoPreviewProvider implementing `vscode.CustomReadonlyEditorProvider`; webview-based video player with configurable autoplay/loop settings from workspace config

### Base Classes & Patterns
- `extensions/media-preview/src/mediaPreview.ts` — Abstract MediaPreview class extending Disposable; shared webview lifecycle management (onDidChangeViewState, onDidDispose), file system watching, webview options configuration with CSP, enableScripts, localResourceRoots
- `extensions/media-preview/src/ownedStatusBarEntry.ts` — PreviewStatusBarEntry abstract base class; status bar item lifecycle management
- `extensions/media-preview/src/binarySizeStatusBarEntry.ts` — BinarySizeStatusBarEntry for displaying file size; extends PreviewStatusBarEntry with binary size formatting (B/KB/MB/GB/TB)

### Image Preview Specific
- `extensions/media-preview/src/imagePreview/sizeStatusBarEntry.ts` — SizeStatusBarEntry displays image dimensions (width × height)
- `extensions/media-preview/src/imagePreview/zoomStatusBarEntry.ts` — ZoomStatusBarEntry with Scale type (number | 'fit'); manages zoom level selection via quick pick, fires onDidChangeScale events, registers select zoom level command

### Utilities
- `extensions/media-preview/src/util/dispose.ts` — Disposable base class for resource lifecycle management; `disposeAll()` utility function
- `extensions/media-preview/src/util/dom.ts` — `escapeAttribute()` function for XSS prevention when building webview HTML
- `extensions/media-preview/src/util/uuid.ts` — `generateUuid()` function using crypto.randomUUID or fallback Uint8Array implementation; used for CSP nonce generation

### Webview Content (JavaScript)
- `extensions/media-preview/media/imagePreview.js` — Client-side image viewer: zoom (10x to 0.1x, fit-to-window), pan, keyboard shortcuts (ctrl/cmd+scroll, arrow keys), state persistence via `vscode.setState()`, `vscode.postMessage()` for reopen-as-text command, `window.addEventListener('message')` for state updates from extension
- `extensions/media-preview/media/audioPreview.js` — Client-side audio player; minimal logic, mainly HTML5 audio element with error handling and fallback to text editor
- `extensions/media-preview/media/videoPreview.js` — Client-side video player; HTML5 video element with error handling, configuration-driven autoplay/loop settings

### Webview Content (CSS)
- `extensions/media-preview/media/imagePreview.css` — Styles for image viewer (fullscreen container, loading indicator, pixelation at scale > 3x)
- `extensions/media-preview/media/audioPreview.css` — Styles for audio player (centered container, dark theme support)
- `extensions/media-preview/media/videoPreview.css` — Styles for video player (responsive container, loading indicator)

---

## Configuration

### Extension Manifest
- `extensions/media-preview/package.json` — Declares three customEditors with viewTypes (imagePreview.previewEditor, vscode.audioPreview, vscode.videoPreview), priority "builtin", file selector patterns; contributes zoom/copy/reopen commands; settings for video autoPlay and loop; supports virtual/untrusted workspaces

### TypeScript Build Configuration
- `extensions/media-preview/tsconfig.json` — Extends tsconfig.base.json; targets Node.js runtime; includes vscode.d.ts type definitions
- `extensions/media-preview/tsconfig.browser.json` — Extends tsconfig.json (no additional changes in this project)

### Build Scripts
- `extensions/media-preview/esbuild.mts` — esbuild configuration for Node.js platform; compiles src/extension.ts to dist/extension
- `extensions/media-preview/esbuild.browser.mts` — esbuild configuration for browser platform; compiles src/extension.ts to dist/browser/extension.js using tsconfig.browser.json

### Localization
- `extensions/media-preview/package.nls.json` — i18n strings for display names, command titles, status messages (Image/Audio/Video Preview, zoom commands, reopen actions)

### Development Configuration
- `extensions/media-preview/.npmrc` — Legacy peer deps mode; 180s timeout; min release age 1
- `extensions/media-preview/.vscodeignore` — Excludes dev files from bundled extension

---

## Types / Interfaces

### TypeScript Type Definitions
- `extensions/media-preview/src/imagePreview/zoomStatusBarEntry.ts` — `type Scale = number | 'fit'` (zoom level union type)
- `extensions/media-preview/src/mediaPreview.ts` — `const enum PreviewState { Disposed, Visible, Active }` (lifecycle states)

### VS Code API Interfaces Used (Not Defined Here)
- `vscode.CustomReadonlyEditorProvider` — Implemented by ImagePreviewManager, AudioPreviewProvider, VideoPreviewProvider
- `vscode.WebviewPanel` — Type passed to resolveCustomEditor methods
- `vscode.ExtensionContext` — Type passed to activate function and register functions
- `vscode.CustomDocument` — Type passed to openCustomDocument and resolveCustomEditor
- `vscode.Disposable` — Base type for lifecycle management (imported from vscode module)

---

## Documentation

- `extensions/media-preview/README.md` — User-facing documentation; lists supported image formats (jpg, png, bmp, gif, ico, webp, avif, svg), audio formats (mp3, wav, ogg), video formats (mp4, webm); notes bundled extension status, no uninstall capability

---

## Notable Clusters

### `extensions/media-preview/src/imagePreview/` — 3 files
- Implements full image preview with zoom/pan; demonstrates complex webview state management, status bar integration, and command handling

### `extensions/media-preview/media/` — 6 files (CSS + JS + SVG loaders)
- Client-side webview implementations for image/audio/video players; shows postMessage API, event handling, DOM manipulation within webview context; loading spinner SVG assets (loading.svg, loading-dark.svg, loading-hc.svg)

### `extensions/media-preview/src/util/` — 3 files
- Cross-cutting utility patterns: lifecycle management (Disposable), DOM safety (escapeAttribute), cryptographic UUID generation; importable throughout extension

---

## Tauri/Rust Port Relevance

This extension is highly relevant to a Tauri/Rust port because:

1. **Webview Architecture Pattern**: Demonstrates core VS Code pattern of webview-based custom editors with message passing (`postMessage`/`onDidReceiveMessage`) for asynchronous editor ↔ webview communication

2. **CSP & Security**: Shows how VS Code enforces Content Security Policy in webviews (nonce-based inline scripts, localResourceRoots restrictions), critical to replicate in Tauri's webview security model

3. **State Management**: Demonstrates vscode.setState/getState persistence API and how previews maintain state across visibility changes—Tauri would need equivalent serialization layer

4. **Status Bar Integration**: Shows command/event-driven UI updates to status bar from webview interactions (zoom level, image size)—indicates rich extension ↔ IDE communication needs

5. **Resource Loading**: Illustrates asWebviewUri URI rewriting for webview resource access with localResourceRoots isolation—a critical security boundary in both Electron and Tauri

6. **Configuration Access**: Uses vscode.workspace.getConfiguration for runtime settings (video autoplay/loop), showing extension-level config binding—Tauri would need config provider API

7. **Workspace FS API**: Uses vscode.workspace.fs.stat and createFileSystemWatcher for file monitoring and metadata—indicates Tauri needs robust async file I/O layer

8. **Disposable Pattern**: Core Disposable base class shows how VS Code manages resource cleanup; would map to Rust's Drop trait and Arc<Mutex<>> patterns

9. **Localization (l10n)**: Uses vscode.l10n.t() for i18n strings—Tauri port needs i18n integration at API level

10. **Zero Test Files**: No unit tests in this partition indicates test infrastructure would need to be designed for Tauri model (unlike Electron's Jest/Mocha integration)

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
8. `extensions/media-preview/src/util/dispose.ts`
9. `extensions/media-preview/media/imagePreview.js`
10. `extensions/media-preview/media/audioPreview.js`
11. `extensions/media-preview/media/videoPreview.js`
12. `extensions/media-preview/package.json`

---

### Per-File Notes

#### `extensions/media-preview/src/extension.ts`

- **Role:** Extension entry point. Instantiates shared infrastructure and delegates registration of all three media preview types (image, audio, video).
- **Key symbols:**
  - `activate` (`extension.ts:12`) — sole exported function; called by VS Code on extension activation.
  - `BinarySizeStatusBarEntry` (`extension.ts:13`) — instantiated once and shared across all preview registrations.
- **Control flow:** `activate` creates a single `BinarySizeStatusBarEntry`, then calls `registerImagePreviewSupport`, `registerAudioPreviewSupport`, and `registerVideoPreviewSupport` in sequence, pushing all returned disposables onto `context.subscriptions`.
- **Data flow:** `extensionContext` → shared `BinarySizeStatusBarEntry` instance → passed by reference into each register function → stored on `context.subscriptions` for lifetime management.
- **Dependencies:** `vscode` API (ExtensionContext, Disposable), `./audioPreview`, `./binarySizeStatusBarEntry`, `./imagePreview`, `./videoPreview`.

---

#### `extensions/media-preview/src/mediaPreview.ts`

- **Role:** Abstract base class `MediaPreview` providing common lifecycle, file watching, binary-size tracking, and webview HTML injection used by all three concrete preview types.
- **Key symbols:**
  - `MediaPreview` (`mediaPreview.ts:21`) — abstract class extending `Disposable`.
  - `PreviewState` enum (`mediaPreview.ts:15`) — three states: `Disposed`, `Visible`, `Active`.
  - `render` (`mediaPreview.ts:83`) — calls abstract `getWebviewContents()` then sets `webviewEditor.webview.html`.
  - `updateBinarySize` (`mediaPreview.ts:76`) — calls `vscode.workspace.fs.stat` to get file size, then calls `updateState`.
  - `updateState` (`mediaPreview.ts:98`) — shows or hides `BinarySizeStatusBarEntry` depending on whether `_webviewEditor.active` is true.
  - `getWebviewContents` (`mediaPreview.ts:96`) — abstract; subclasses must return a full HTML string.
  - `reopenAsText` (`mediaPreview.ts:11`) — module-level function; executes `vscode.openWith` command with `'default'` editor.
- **Control flow:**
  - Constructor registers three event listeners on `_webviewEditor`: `onDidChangeViewState` → `updateState`; `onDidDispose` → `previewState = Disposed` + `dispose()`. Also registers a `FileSystemWatcher` on the resource's directory.
  - `FileSystemWatcher.onDidChange` at `mediaPreview.ts:53`: checks URI equality, then calls `updateBinarySize()` and `render()`.
  - `FileSystemWatcher.onDidDelete` at `mediaPreview.ts:60`: disposes the webview panel.
- **Data flow:** `vscode.Uri` resource → `workspace.fs.stat` → raw byte size stored in `_binarySize` → formatted by `BinarySizeStatusBarEntry.formatSize` → text shown in status bar. File path is passed to subclass as `_resource` for HTML generation.
- **Dependencies:** `vscode`, `vscode-uri` (Utils.dirname), `./binarySizeStatusBarEntry`, `./util/dispose`.

---

#### `extensions/media-preview/src/imagePreview/index.ts`

- **Role:** Registers `imagePreview.previewEditor` as a `CustomReadonlyEditorProvider`. Manages a live set of `ImagePreview` instances, tracks the active one, and registers commands (`zoomIn`, `zoomOut`, `copyImage`, `reopenAsText`, `reopenAsPreview`).
- **Key symbols:**
  - `ImagePreviewManager` (`index.ts:15`) — implements `vscode.CustomReadonlyEditorProvider`.
  - `ImagePreviewManager.viewType` (`index.ts:17`) — static `'imagePreview.previewEditor'`.
  - `ImagePreviewManager.resolveCustomEditor` (`index.ts:33`) — creates an `ImagePreview`, adds it to `_previews` Set, wires up active-tracking events.
  - `ImagePreview` (`index.ts:73`) — extends `MediaPreview`; handles bidirectional postMessage with the webview.
  - `ImagePreview.getWebviewContents` (`index.ts:182`) — constructs the full HTML document injecting a CSP nonce, `data-settings` JSON containing the cache-busted resource URL, and script/style tags pointing to `media/imagePreview.js` and `media/imagePreview.css`.
  - `ImagePreview.getResourcePath` (`index.ts:218`) — handles git-scheme resources (returns 1×1 empty PNG data URI if size is 0), otherwise returns `webview.asWebviewUri(resource)` with `?version=<timestamp>` cache-busting query.
  - `registerImagePreviewSupport` (`index.ts:244`) — factory that wires together all disposables and returns them.
- **Control flow:**
  - On message `'size'` from webview: stores `_imageSize`, calls `updateState` → `sizeStatusBarEntry.show`.
  - On message `'zoom'` from webview: stores `_imageZoom`, calls `updateState` → `zoomStatusBarEntry.show`.
  - On message `'reopen-as-text'` from webview: calls `reopenAsText(resource, viewColumn)`.
  - `zoomStatusBarEntry.onDidChangeScale` fires → `_webviewEditor.webview.postMessage({ type: 'setScale', scale })` sent to webview JS.
  - `webviewEditor.onDidChangeViewState` → posts `{ type: 'setActive', value: webviewEditor.active }` to webview.
- **Data flow:** Resource URI → `asWebviewUri` → embedded as `settings.src` in HTML `<meta>` tag → read by `imagePreview.js` via `document.getElementById('image-preview-settings').getAttribute('data-settings')` → used as `image.src`.
- **Dependencies:** `vscode`, `../binarySizeStatusBarEntry`, `../mediaPreview`, `../util/dom`, `../util/uuid`, `./sizeStatusBarEntry`, `./zoomStatusBarEntry`.

---

#### `extensions/media-preview/src/audioPreview.ts`

- **Role:** Registers `vscode.audioPreview` custom editor for `.mp3`, `.wav`, `.ogg`, `.oga` files. Simpler than image preview — no zoom or size state, only binary size and a `reopen-as-text` message.
- **Key symbols:**
  - `AudioPreviewProvider` (`audioPreview.ts:12`) — implements `vscode.CustomReadonlyEditorProvider`.
  - `AudioPreview` (`audioPreview.ts:31`) — extends `MediaPreview`; CSP includes `media-src ${cspSource}`.
  - `AudioPreview.getWebviewContents` (`audioPreview.ts:55`) — injects settings JSON via `<meta id="settings" data-settings="...">` and loads `media/audioPreview.js`.
  - `registerAudioPreviewSupport` (`audioPreview.ts:112`) — registers the provider with `retainContextWhenHidden: true` and `supportsMultipleEditorsPerDocument: true`.
- **Control flow:** Only one inbound message type: `'reopen-as-text'` → `reopenAsText(resource, viewColumn)`. No outbound messages to the webview after initial load.
- **Data flow:** `resource.scheme === 'git'` check at `audioPreview.ts:92`: if `stat.size === 0`, returns `null` as `settings.src`, which the webview reads and passes `undefined` to `new Audio()`, calling `onLoaded()` immediately (LFS sentinel path).
- **Dependencies:** `vscode`, `./binarySizeStatusBarEntry`, `./mediaPreview`, `./util/dom`, `./util/uuid`.

---

#### `extensions/media-preview/src/videoPreview.ts`

- **Role:** Registers `vscode.videoPreview` custom editor for `.mp4` and `.webm` files. Structurally identical to `AudioPreview` but adds `autoplay` and `loop` configuration read from `vscode.workspace.getConfiguration('mediaPreview.video')`.
- **Key symbols:**
  - `VideoPreviewProvider` (`videoPreview.ts:13`) — implements `vscode.CustomReadonlyEditorProvider`.
  - `VideoPreview.getWebviewContents` (`videoPreview.ts:56`) — reads `mediaPreview.video.autoPlay` and `mediaPreview.video.loop` from workspace configuration, embeds them in settings JSON.
  - `registerVideoPreviewSupport` (`videoPreview.ts:116`) — identical registration pattern to audio, with `retainContextWhenHidden: true`.
- **Control flow:** Only inbound message: `'reopen-as-text'`. Settings are read synchronously at render time via `vscode.workspace.getConfiguration`.
- **Data flow:** `vscode.workspace.getConfiguration` → `autoplay`, `loop` values → serialized into HTML `<meta data-settings>` → read in `videoPreview.js` at `video.autoplay = settings.autoplay`.
- **Dependencies:** `vscode`, `./binarySizeStatusBarEntry`, `./mediaPreview`, `./util/dom`, `./util/uuid`.

---

#### `extensions/media-preview/src/binarySizeStatusBarEntry.ts`

- **Role:** Concrete status bar entry that formats raw byte counts into human-readable size strings (B, KB, MB, GB, TB) for display in the editor status bar.
- **Key symbols:**
  - `BinarySize.formatSize` (`binarySizeStatusBarEntry.ts:16`) — static method applying threshold-based unit formatting using `vscode.l10n.t` for i18n.
  - `BinarySizeStatusBarEntry` (`binarySizeStatusBarEntry.ts:37`) — extends `PreviewStatusBarEntry`; registered with `StatusBarAlignment.Right`, priority 100.
  - `show` (`binarySizeStatusBarEntry.ts:43`) — accepts `owner` (for ownership-guarding) and raw byte `size`; calls `super.showItem` only if `size` is a number.
- **Control flow:** If `size === undefined`, calls `this.hide(owner)` instead of showing. This happens when `updateBinarySize` has not yet resolved its `workspace.fs.stat` promise.
- **Data flow:** Raw `number` bytes → `BinarySize.formatSize` → localized string → `entry.text` on the VS Code `StatusBarItem`.
- **Dependencies:** `vscode`, `./ownedStatusBarEntry`.

---

#### `extensions/media-preview/src/ownedStatusBarEntry.ts`

- **Role:** Abstract base that wraps `vscode.StatusBarItem` with ownership semantics: only the owner that last called `showItem` can subsequently hide the entry, preventing one preview from hiding another's status bar text.
- **Key symbols:**
  - `PreviewStatusBarEntry` (`ownedStatusBarEntry.ts:9`) — abstract class extending `Disposable`.
  - `_showOwner` (`ownedStatusBarEntry.ts:10`) — private field tracking which preview instance currently owns the status bar item.
  - `showItem` (`ownedStatusBarEntry.ts:20`) — sets `_showOwner`, updates `entry.text`, calls `entry.show()`.
  - `hide` (`ownedStatusBarEntry.ts:26`) — only hides if `owner === _showOwner`; resets `_showOwner` to `undefined`.
- **Control flow:** All three subclasses (`BinarySizeStatusBarEntry`, `SizeStatusBarEntry`, `ZoomStatusBarEntry`) call `this.showItem(owner, text)` / `this.hide(owner)`. Guard at `hide` ensures multi-panel scenarios don't cross-contaminate.
- **Data flow:** `owner` reference (the `ImagePreview`/`AudioPreview`/`VideoPreview` instance) passed through as an opaque identity token; compared by reference (`===`).
- **Dependencies:** `vscode`, `./util/dispose`.

---

#### `extensions/media-preview/src/util/dispose.ts`

- **Role:** Provides `Disposable` abstract base class and `disposeAll` utility that drains a `vscode.Disposable[]` array.
- **Key symbols:**
  - `Disposable` (`dispose.ts:17`) — tracks `_isDisposed` flag; `_register<T>(value)` at `dispose.ts:30` adds to `_disposables` or immediately disposes if already disposed.
  - `disposeAll` (`dispose.ts:8`) — pops and disposes items from array in LIFO order.
- **Control flow:** `_register` used throughout the extension to attach event subscriptions to instance lifetime. `dispose()` at `dispose.ts:22` is idempotent via `_isDisposed` guard.
- **Dependencies:** `vscode` (for `vscode.Disposable` interface).

---

#### `extensions/media-preview/media/imagePreview.js`

- **Role:** Webview-side JavaScript for image preview. Runs inside the sandboxed webview `<iframe>` context, handles user input (clicks, wheel, keyboard, touch pinch), manages zoom state, persists state via `vscode.setState`/`vscode.getState`, and communicates with the extension host via `vscode.postMessage`.
- **Key symbols:**
  - `acquireVsCodeApi()` (`imagePreview.js:65`) — webview API injection providing `postMessage`, `setState`, `getState`.
  - `updateScale(newScale)` (`imagePreview.js:81`) — core zoom function; applies CSS `zoom` property to `<img>`, manages `pixelated` class above `PIXELATION_THRESHOLD = 3` (`imagePreview.js:34`), and persists scroll position via `vscode.setState`.
  - `zoomIn` / `zoomOut` (`imagePreview.js:170/184`) — step through discrete `zoomLevels` array (`imagePreview.js:40`).
  - `copyImage` (`imagePreview.js:380`) — uses `navigator.clipboard.write` + `ClipboardItem` + canvas `toBlob` to copy the image as PNG; retries up to 5 times to handle focus timing.
  - `window.addEventListener('message', ...)` (`imagePreview.js:346`) — receives messages from extension host: `setScale`, `setActive`, `zoomIn`, `zoomOut`, `copyImage`.
  - `image.addEventListener('load', ...)` (`imagePreview.js:303`) — posts `{ type: 'size', value: '${w}x${h}' }` back to extension host.
- **Control flow:**
  - Initial state loaded from `vscode.getState()` at `imagePreview.js:67`; defaults to `{ scale: 'fit', offsetX: 0, offsetY: 0 }`.
  - Image `src` set at `imagePreview.js:337` to `settings.src` parsed from the `<meta>` tag.
  - On `load` event: removes `loading` CSS class, appends `<img>` to body, calls `updateScale(scale)`.
  - Wheel event listener at `imagePreview.js:265` marked `{ passive: false }` to allow `preventDefault()` for pinch-zoom suppression.
  - `setActive(false)` at `imagePreview.js:136` resets `ctrlPressed`/`altPressed` state and removes zoom-cursor CSS classes.
- **Data flow:** `settings.src` (URI string from extension host) → `image.src` → browser loads resource → `image.naturalWidth`/`naturalHeight` posted back as `'size'` message → stored as `_imageSize` in `ImagePreview` → displayed via `SizeStatusBarEntry`.
- **Dependencies:** `acquireVsCodeApi` (injected by VS Code webview runtime), `navigator.clipboard`, `canvas` API, `navigator.platform`.

---

#### `extensions/media-preview/media/audioPreview.js`

- **Role:** Webview-side JavaScript for audio preview. Creates an `<audio controls>` element, appends it on `canplaythrough`, handles LFS null-src case, and sends `reopen-as-text` message on link click.
- **Key symbols:**
  - `acquireVsCodeApi()` (`audioPreview.js:10`) — webview API.
  - `new Audio(settings.src === null ? undefined : settings.src)` (`audioPreview.js:34`) — creates HTML5 audio element; `undefined` src skips network fetch for LFS placeholder.
  - `onLoaded` (`audioPreview.js:37`) — removes `loading` class, adds `ready`, appends `<audio>` to container.
- **Control flow:** If `settings.src === null` → calls `onLoaded()` immediately (LFS case). Otherwise waits for `canplaythrough` event before calling `onLoaded()`.
- **Data flow:** `settings.src` (URI or null) → `Audio` constructor → browser media pipeline → `canplaythrough` → displayed.
- **Dependencies:** `acquireVsCodeApi`, HTML5 `Audio` API.

---

#### `extensions/media-preview/media/videoPreview.js`

- **Role:** Webview-side JavaScript for video preview. Structurally identical to `audioPreview.js` but creates a `<video>` element and applies `autoplay`, `muted`, and `loop` from settings.
- **Key symbols:**
  - `video.muted = settings.autoplay` (`videoPreview.js:37`) — video is auto-muted when autoplay is enabled (browser autoplay policy compliance).
  - `video.playsInline = true` (`videoPreview.js:34`) — prevents full-screen on mobile.
- **Control flow:** Same LFS null-src guard as audio: if `settings.src === null`, `onLoaded()` called immediately. Otherwise waits for `canplaythrough`.
- **Dependencies:** `acquireVsCodeApi`, HTML5 `video` element API.

---

#### `extensions/media-preview/package.json`

- **Role:** Extension manifest declaring custom editor registrations, commands, menus, configuration schema, and build configuration.
- **Key symbols:**
  - `customEditors` array (`package.json:46`) — declares three editors with `"priority": "builtin"` and glob selectors: image (`*.{jpg,jpe,jpeg,png,bmp,gif,ico,webp,avif,svg}`), audio (`*.{mp3,wav,ogg,oga}`), video (`*.{mp4,webm}`).
  - `extensionKind: ["ui", "workspace"]` (`package.json:5`) — extension can run in both local and remote workspace processes.
  - `"browser": "./dist/browser/extension.js"` (`package.json:18`) — separate web worker build for VS Code for the Web.
  - `capabilities.virtualWorkspaces: true` (`package.json:24`) — works with virtual file systems (e.g., github.dev).
  - Configuration keys `mediaPreview.video.autoPlay` and `mediaPreview.video.loop` (`package.json:34,39`) — both default to `false`.
  - Commands: `imagePreview.zoomIn`, `imagePreview.zoomOut`, `imagePreview.copyImage`, `imagePreview.reopenAsPreview`, `imagePreview.reopenAsText` (`package.json:79`–`106`).
- **Dependencies:** `@vscode/extension-telemetry ^0.9.8`, `vscode-uri ^3.0.6`.

---

### Cross-Cutting Synthesis

The media-preview extension implements a classic VS Code webview-based custom editor architecture. Each media type (image, audio, video) follows the same structure: a `CustomReadonlyEditorProvider` creates a concrete `MediaPreview` subclass that generates a full HTML document and injects it into a `WebviewPanel`. The two-process message boundary — extension host TypeScript ↔ sandboxed webview JavaScript via `postMessage`/`onDidReceiveMessage` — is the central communication mechanism. Settings flow from extension host to webview via JSON embedded in a `<meta data-settings>` tag at render time; events (size, zoom, user gestures) flow back from webview to extension host as typed message objects. File-system access (stat, read) is performed exclusively in the extension host using `vscode.workspace.fs`, never in the webview.

For a Tauri/Rust port, the key surface areas this extension exposes are: (1) a custom editor provider API (`vscode.CustomReadonlyEditorProvider`, `openCustomDocument`, `resolveCustomEditor`) that would need a Tauri equivalent — likely a Tauri webview window or `tauri::WebviewBuilder`; (2) the `WebviewPanel` abstraction providing `postMessage`/`onDidReceiveMessage`, `asWebviewUri`, `cspSource`, and `webview.html` setter — Tauri's `tauri::ipc` and webview `eval`/`emit` channels are the closest analogues; (3) `vscode.workspace.fs.stat` for VFS-aware file access — Tauri uses Rust `std::fs` or custom VFS plugins; (4) `vscode.workspace.createFileSystemWatcher` for reactive file change events — Tauri's `tauri-plugin-fs-watch` covers this; (5) `StatusBarItem` for status bar display — no direct Tauri equivalent exists and would require a custom sidebar/overlay UI; and (6) `vscode.workspace.getConfiguration` for user settings — replaceable with Tauri's `tauri::Config` or a settings store plugin. The webview-side JS (`imagePreview.js`, `audioPreview.js`, `videoPreview.js`) is largely portable as-is since it targets standard browser APIs (`<img>`, `<audio>`, `<video>`, `navigator.clipboard`, `canvas`), with only the `acquireVsCodeApi()` injection needing replacement by a Tauri IPC bridge.

---

### Out-of-Partition References

- `vscode.CustomReadonlyEditorProvider` — defined in VS Code's extension host API (`src/vscode-dts/vscode.d.ts` or `src/vs/workbench/api/common/extHostCustomEditors.ts`).
- `vscode.WebviewPanel` / `webview.postMessage` / `webview.asWebviewUri` / `webview.cspSource` — implemented in `src/vs/workbench/api/common/extHostWebview.ts` and `extHostWebviewPanel.ts`.
- `vscode.workspace.fs.stat` / `vscode.workspace.createFileSystemWatcher` — `src/vs/workbench/api/common/extHostFileSystem.ts` and `extHostFileSystemEventService.ts`.
- `vscode.window.registerCustomEditorProvider` — `src/vs/workbench/api/common/extHostCustomEditors.ts`.
- `vscode.window.createStatusBarItem` — `src/vs/workbench/api/common/extHostStatusBar.ts`.
- `vscode.commands.registerCommand` / `executeCommand` — `src/vs/workbench/api/common/extHostCommands.ts`.
- `vscode.l10n.t` — `src/vs/workbench/api/common/extHostLocalization.ts`.
- `acquireVsCodeApi()` — injected into webview context by `src/vs/workbench/contrib/webview/browser/pre/main.js`.
- `vscode-uri` (Utils.dirname) — external npm package, not in this repo partition.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: VS Code Core IDE Porting to Tauri/Rust
## Scope: extensions/media-preview/

This document catalogs concrete patterns from the media-preview extension that demonstrate how VS Code implements custom editors and webview communication—core infrastructure relevant to porting IDE functionality from TypeScript/Electron to Tauri/Rust.

---

## Pattern 1: Custom Editor Provider Registration

**Where:** `extensions/media-preview/src/videoPreview.ts:118-123`

**What:** Custom editor providers implement the VS Code extension API pattern for registering content editors with file-type selectors.

```typescript
export function registerVideoPreviewSupport(context: vscode.ExtensionContext, binarySizeStatusBarEntry: BinarySizeStatusBarEntry): vscode.Disposable {
	const provider = new VideoPreviewProvider(context.extensionUri, binarySizeStatusBarEntry);
	return vscode.window.registerCustomEditorProvider(VideoPreviewProvider.viewType, provider, {
		supportsMultipleEditorsPerDocument: true,
		webviewOptions: {
			retainContextWhenHidden: true,
		}
	});
}
```

**Variations / call-sites:**
- Audio preview: `extensions/media-preview/src/audioPreview.ts:114-119`
- Image preview: `extensions/media-preview/src/imagePreview/index.ts:255-257`

---

## Pattern 2: Custom Editor Interface Implementation

**Where:** `extensions/media-preview/src/audioPreview.ts:12-28`

**What:** Custom editor providers must implement two async lifecycle methods: `openCustomDocument` (minimal for read-only) and `resolveCustomEditor` (instantiate the preview UI).

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

**Variations / call-sites:**
- Video provider: `extensions/media-preview/src/videoPreview.ts:13-28`
- Image provider: `extensions/media-preview/src/imagePreview/index.ts:15-50`

---

## Pattern 3: Webview Options and Security Configuration

**Where:** `extensions/media-preview/src/mediaPreview.ts:34-41`

**What:** Webview security and capability configuration via options object—scripts enabled, forms disabled, and local resource roots for sandboxed asset loading.

```typescript
_webviewEditor.webview.options = {
	enableScripts: true,
	enableForms: false,
	localResourceRoots: [
		Utils.dirname(_resource),
		extensionRoot,
	]
};
```

**Variations / call-sites:**
- All preview types inherit this configuration from the base `MediaPreview` class.

---

## Pattern 4: Bidirectional Message Passing (Extension ↔ Webview)

**Where:** `extensions/media-preview/src/imagePreview/index.ts:90-106`

**What:** Extension receives messages from webview via typed message switch; sends messages via `postMessage` method. Messages are typed with a discriminated union pattern.

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

**Variations / call-sites:**
- Video preview message handling: `extensions/media-preview/src/videoPreview.ts:42-49`
- Audio preview message handling: `extensions/media-preview/src/audioPreview.ts:41-48`

---

## Pattern 5: Extension → Webview Command Dispatch

**Where:** `extensions/media-preview/src/imagePreview/index.ts:144-151`

**What:** Extension sends structured commands to webview UI via `postMessage` to trigger client-side actions (zoom, visibility state, etc.).

```typescript
public zoomIn() {
	if (this.previewState === PreviewState.Active) {
		this._webviewEditor.webview.postMessage({ type: 'zoomIn' });
	}
}

public zoomOut() {
	if (this.previewState === PreviewState.Active) {
		this._webviewEditor.webview.postMessage({ type: 'zoomOut' });
	}
}

public copyImage() {
	if (this.previewState === PreviewState.Active) {
		this._webviewEditor.reveal();
		this._webviewEditor.webview.postMessage({ type: 'copyImage' });
	}
}
```

**Variations / call-sites:**
- Scale/active state updates: `extensions/media-preview/src/imagePreview/index.ts:111-117`

---

## Pattern 6: Webview-Side Message Reception and Dispatch

**Where:** `extensions/media-preview/media/imagePreview.js:346-374`

**What:** Webview listens to `message` events from extension; validates origin; dispatches based on message type to UI update functions.

```javascript
window.addEventListener('message', e => {
	if (e.origin !== window.origin) {
		console.error('Dropping message from unknown origin in image preview');
		return;
	}

	switch (e.data.type) {
		case 'setScale': {
			updateScale(e.data.scale);
			break;
		}
		case 'setActive': {
			setActive(e.data.value);
			break;
		}
		case 'zoomIn': {
			zoomIn();
			break;
		}
		case 'zoomOut': {
			zoomOut();
			break;
		}
		case 'copyImage': {
			copyImage();
			break;
		}
	}
});
```

**Variations / call-sites:**
- Video preview: No incoming messages (simpler player UI)
- Audio preview: No incoming messages

---

## Pattern 7: Webview → Extension Message Sending

**Where:** `extensions/media-preview/media/imagePreview.js:309-312`

**What:** Webview acquires VS Code API via `acquireVsCodeApi()` global, then sends messages up to extension via `vscode.postMessage()`.

```javascript
image.addEventListener('load', () => {
	if (hasLoadedImage) {
		return;
	}
	hasLoadedImage = true;

	vscode.postMessage({
		type: 'size',
		value: `${image.naturalWidth}x${image.naturalHeight}`,
	});

	document.body.classList.remove('loading');
	document.body.classList.add('ready');
	document.body.append(image);

	updateScale(scale);

	if (initialState.scale !== 'fit') {
		window.scrollTo(initialState.offsetX, initialState.offsetY);
	}
});
```

**Variations / call-sites:**
- Reopen-as-text message: `extensions/media-preview/media/imagePreview.js:339-344`
- Zoom update messages: `extensions/media-preview/media/imagePreview.js:130-133`
- Video reopen message: `extensions/media-preview/media/videoPreview.js:69-74`

---

## Pattern 8: HTML Content Generation with Nonce Security

**Where:** `extensions/media-preview/src/videoPreview.ts:56-92`

**What:** Webview HTML is generated server-side with CSP headers, nonce tokens for scripts, and dynamic asset URI rewriting via `asWebviewUri()`.

```typescript
protected async getWebviewContents(): Promise<string> {
	const version = Date.now().toString();
	const configurations = vscode.workspace.getConfiguration('mediaPreview.video');
	const settings = {
		src: await this.getResourcePath(this._webviewEditor, this._resource, version),
		autoplay: configurations.get('autoPlay'),
		loop: configurations.get('loop'),
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

	<title>Video Preview</title>

	<link rel="stylesheet" href="${escapeAttribute(this.extensionResource('media', 'videoPreview.css'))}" type="text/css" media="screen" nonce="${nonce}">

	<meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src data: ${cspSource}; media-src ${cspSource}; script-src 'nonce-${nonce}'; style-src ${cspSource} 'nonce-${nonce}';">
	<meta id="settings" data-settings="${escapeAttribute(JSON.stringify(settings))}">
</head>
<body class="loading" data-vscode-context='{ "preventDefaultContextMenuItems": true }'>
	<div class="loading-indicator"></div>
	<div class="loading-error">
		<p>${vscode.l10n.t("An error occurred while loading the video file.")}</p>
		<a href="#" class="open-file-link">${vscode.l10n.t("Open file using VS Code's standard text/binary editor?")}</a>
	</div>
	<script src="${escapeAttribute(this.extensionResource('media', 'videoPreview.js'))}" nonce="${nonce}"></script>
</body>
</html>`;
}
```

**Variations / call-sites:**
- Image preview with inline settings: `extensions/media-preview/src/imagePreview/index.ts:182-215`
- Audio preview: `extensions/media-preview/src/audioPreview.ts:55-88`

---

## Pattern 9: File Resource URI Rewriting for Webview Sandbox

**Where:** `extensions/media-preview/src/videoPreview.ts:95-108`

**What:** Media resources must be rewritten via `webviewEditor.webview.asWebviewUri()` to serve from webview's sandboxed protocol; cache-busting via query parameters.

```typescript
private async getResourcePath(webviewEditor: vscode.WebviewPanel, resource: vscode.Uri, version: string): Promise<string | null> {
	if (resource.scheme === 'git') {
		const stat = await vscode.workspace.fs.stat(resource);
		if (stat.size === 0) {
			// The file is stored on git lfs
			return null;
		}
	}

	// Avoid adding cache busting if there is already a query string
	if (resource.query) {
		return webviewEditor.webview.asWebviewUri(resource).toString();
	}
	return webviewEditor.webview.asWebviewUri(resource).with({ query: `version=${version}` }).toString();
}
```

**Variations / call-sites:**
- Image preview: `extensions/media-preview/src/imagePreview/index.ts:218-230`

---

## Pattern 10: Lifecycle and Disposal Management

**Where:** `extensions/media-preview/src/util/dispose.ts:17-41`

**What:** Reusable `Disposable` base class provides `_register()` helper to automatically track and dispose resources; prevents memory leaks on view close.

```typescript
export abstract class Disposable {
	private _isDisposed = false;

	protected _disposables: vscode.Disposable[] = [];

	public dispose(): any {
		if (this._isDisposed) {
			return;
		}
		this._isDisposed = true;
		disposeAll(this._disposables);
	}

	protected _register<T extends vscode.Disposable>(value: T): T {
		if (this._isDisposed) {
			value.dispose();
		} else {
			this._disposables.push(value);
		}
		return value;
	}

	protected get isDisposed() {
		return this._isDisposed;
	}
}
```

**Variations / call-sites:**
- Used in all preview classes: `extensions/media-preview/src/mediaPreview.ts:21-65`
- Status bar entry: `extensions/media-preview/src/ownedStatusBarEntry.ts:9-32`

---

## Pattern 11: Status Bar Integration for Active Editor State

**Where:** `extensions/media-preview/src/imagePreview/index.ts:142-175`

**What:** Active editor state drives status bar visibility; show/hide status items based on webview active status; emit events for external consumers.

```typescript
protected override updateState() {
	super.updateState();

	if (this.previewState === PreviewState.Disposed) {
		return;
	}

	if (this._webviewEditor.active) {
		this.sizeStatusBarEntry.show(this, this._imageSize || '');
		this.zoomStatusBarEntry.show(this, this._imageZoom || 'fit');
	} else {
		this.sizeStatusBarEntry.hide(this);
		this.zoomStatusBarEntry.hide(this);
	}
}
```

**Variations / call-sites:**
- Zoom status bar event emission: `extensions/media-preview/src/imagePreview/zoomStatusBarEntry.ts:14-27`
- Binary size formatting: `extensions/media-preview/src/binarySizeStatusBarEntry.ts:10-50`

---

## Pattern 12: Command Registration and Context-Aware Menus

**Where:** `extensions/media-preview/src/imagePreview/index.ts:259-272`

**What:** Commands registered per preview manager; menu conditions filter visibility based on `activeCustomEditorId` context.

```typescript
disposables.push(vscode.commands.registerCommand('imagePreview.zoomIn', () => {
	previewManager.activePreview?.zoomIn();
}));

disposables.push(vscode.commands.registerCommand('imagePreview.zoomOut', () => {
	previewManager.activePreview?.zoomOut();
}));

disposables.push(vscode.commands.registerCommand('imagePreview.copyImage', () => {
	previewManager.activePreview?.copyImage();
}));

disposables.push(vscode.commands.registerCommand('imagePreview.reopenAsText', async () => {
	return previewManager.activePreview?.reopenAsText();
}));
```

**Variations / call-sites:**
- Context check in package.json: `extensions/media-preview/package.json:111-117` (menu conditions like `"when": "activeCustomEditorId == 'imagePreview.previewEditor'"`)

---

## Pattern 13: File System Watching for Live Reloads

**Where:** `extensions/media-preview/src/mediaPreview.ts:52-64`

**What:** Workspace file watcher detects changes to displayed resource; triggers re-render on change, disposes preview on delete.

```typescript
const watcher = this._register(vscode.workspace.createFileSystemWatcher(new vscode.RelativePattern(_resource, '*')));
this._register(watcher.onDidChange(e => {
	if (e.toString() === this._resource.toString()) {
		this.updateBinarySize();
		this.render();
	}
}));

this._register(watcher.onDidDelete(e => {
	if (e.toString() === this._resource.toString()) {
		this._webviewEditor.dispose();
	}
}));
```

**Variations / call-sites:**
- Used uniformly across all preview types via base class inheritance.

---

## Pattern 14: Webview State Persistence (Scroll Position, Zoom)

**Where:** `extensions/media-preview/media/imagePreview.js:64-67`

**What:** VS Code webview API provides `getState()` / `setState()` for per-resource client state persistence across session boundaries.

```javascript
// @ts-ignore
const vscode = acquireVsCodeApi();

const initialState = vscode.getState() || { scale: 'fit', offsetX: 0, offsetY: 0 };
```

**And later, update on scroll:**

```javascript
window.addEventListener('scroll', e => {
	if (!image || !hasLoadedImage || !image.parentElement || scale === 'fit') {
		return;
	}

	const entry = vscode.getState();
	if (entry) {
		vscode.setState({ scale: entry.scale, offsetX: window.scrollX, offsetY: window.scrollY });
	}
}, { passive: true });
```

**Variations / call-sites:**
- Scale update persistence: `extensions/media-preview/media/imagePreview.js:127`

---

## Pattern 15: Localization via vscode.l10n

**Where:** `extensions/media-preview/src/binarySizeStatusBarEntry.ts:18-33`

**What:** Strings externalized via `vscode.l10n.t()` for multi-language support; format placeholders like `{0}` for dynamic values.

```typescript
static formatSize(size: number): string {
	if (size < BinarySize.KB) {
		return vscode.l10n.t("{0}B", size);
	}

	if (size < BinarySize.MB) {
		return vscode.l10n.t("{0}KB", (size / BinarySize.KB).toFixed(2));
	}

	if (size < BinarySize.GB) {
		return vscode.l10n.t("{0}MB", (size / BinarySize.MB).toFixed(2));
	}

	if (size < BinarySize.TB) {
		return vscode.l10n.t("{0}GB", (size / BinarySize.GB).toFixed(2));
	}

	return vscode.l10n.t("{0}TB", (size / BinarySize.TB).toFixed(2));
}
```

**Variations / call-sites:**
- Used in status bar labels, error messages throughout.

---

## Summary

The media-preview extension demonstrates the core architectural patterns for porting VS Code's IDE functionality to Tauri/Rust:

1. **Custom Editor Framework**: Provider interface, document/editor lifecycle separation, multi-document support
2. **Webview Sandbox Model**: Strict message-passing protocol, CSP security, resource URI rewriting, origin validation
3. **State Management**: Persistent view state, lifecycle disposal, active editor tracking
4. **Command Integration**: Command palette registration, context-aware menu visibility
5. **File System Integration**: Workspace watchers, resource monitoring, live reloads
6. **IPC Architecture**: Bidirectional TypeScript↔JavaScript messaging via `postMessage` and `onDidReceiveMessage`
7. **Resource Management**: Asset bundling, local resource root restrictions, cache-busting

For a Tauri port, the highest-friction areas would be:

- **Webview Bridge**: Replacing VS Code's `postMessage` IPC with Tauri's invoke system (requires async/await refactoring)
- **FileSystem Watcher**: Migrating from `vscode.workspace.createFileSystemWatcher` to Tauri's file watcher plugin
- **URI Schemes**: Custom schemes (`git://`, `file://` rewrite) require Tauri protocol handler setup
- **CSS/Security Model**: CSP nonce-based injection differs from Tauri's resource loading model
- **Settings/Config**: Replacing `vscode.workspace.getConfiguration()` with Tauri config system
- **Localization**: `vscode.l10n` would need Tauri i18n integration
- **Status Bar**: No native Tauri equivalent; would require custom UI implementation

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
