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

