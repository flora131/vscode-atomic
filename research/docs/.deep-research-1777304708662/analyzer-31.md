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
