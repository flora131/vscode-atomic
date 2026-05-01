# Pattern Research: VS Code to Tauri/Rust IDE Porting
## Partition 31: Custom Editor Providers & Webview Integration

### Scope
`extensions/media-preview/` (17 files, 1,507 LOC)

---

#### Pattern: CustomReadonlyEditorProvider Registration & Lifecycle

**Where:** `extensions/media-preview/src/imagePreview/index.ts:244-281`

**What:** Registers a custom editor provider with VS Code's extension API, implementing the two-stage lifecycle (openCustomDocument → resolveCustomEditor).

```typescript
export function registerImagePreviewSupport(context: vscode.ExtensionContext, binarySizeStatusBarEntry: BinarySizeStatusBarEntry): vscode.Disposable {
	const disposables: vscode.Disposable[] = [];

	const sizeStatusBarEntry = new SizeStatusBarEntry();
	disposables.push(sizeStatusBarEntry);

	const zoomStatusBarEntry = new ZoomStatusBarEntry();
	disposables.push(zoomStatusBarEntry);

	const previewManager = new ImagePreviewManager(context.extensionUri, sizeStatusBarEntry, binarySizeStatusBarEntry, zoomStatusBarEntry);

	disposables.push(vscode.window.registerCustomEditorProvider(ImagePreviewManager.viewType, previewManager, {
		supportsMultipleEditorsPerDocument: true,
	}));

	disposables.push(vscode.commands.registerCommand('imagePreview.zoomIn', () => {
		previewManager.activePreview?.zoomIn();
	}));

	return vscode.Disposable.from(...disposables);
}
```

**Variations / call-sites:**
- `extensions/media-preview/src/audioPreview.ts:112-120` — Similar pattern with webviewOptions for context retention
- `extensions/media-preview/src/videoPreview.ts:116-124` — Video-specific variant with additional configuration

---

#### Pattern: Webview Message Protocol (Post/Receive)

**Where:** `extensions/media-preview/src/imagePreview/index.ts:90-107`

**What:** Establishes bidirectional message passing between TypeScript extension code and webview JavaScript, using typed message objects with switch dispatch.

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

this._register(zoomStatusBarEntry.onDidChangeScale(e => {
	if (this.previewState === PreviewState.Active) {
		this._webviewEditor.webview.postMessage({ type: 'setScale', scale: e.scale });
	}
}));
```

**Variations / call-sites:**
- `extensions/media-preview/src/audioPreview.ts:41-48` — Minimal variant (only reopen-as-text)
- `extensions/media-preview/src/videoPreview.ts:42-49` — Identical pattern to audio
- `extensions/media-preview/media/imagePreview.js:346-374` — Webview-side receiver (window message listener)

---

#### Pattern: HTML Webview Content Generation with CSP & Nonce

**Where:** `extensions/media-preview/src/imagePreview/index.ts:182-215`

**What:** Dynamically generates HTML content for webview with Content Security Policy, nonce injection for inline scripts, and resource URI translation via asWebviewUri.

```typescript
protected override async getWebviewContents(): Promise<string> {
	const version = Date.now().toString();
	const settings = {
		src: await this.getResourcePath(this._webviewEditor, this._resource, version),
	};

	const nonce = generateUuid();

	const cspSource = this._webviewEditor.webview.cspSource;
	return /* html */`<!DOCTYPE html>
<html lang="en">
<head>
	<meta charset="UTF-8">
	<meta name="viewport"
		content="width=device-width, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0, user-scalable=no">
	<title>Image Preview</title>

	<link rel="stylesheet" href="${escapeAttribute(this.extensionResource('media', 'imagePreview.css'))}" type="text/css" media="screen" nonce="${nonce}">

	<meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src data: ${cspSource}; connect-src ${cspSource}; script-src 'nonce-${nonce}'; style-src ${cspSource} 'nonce-${nonce}';">
	<meta id="image-preview-settings" data-settings="${escapeAttribute(JSON.stringify(settings))}">
</head>
<body class="container image scale-to-fit loading" data-vscode-context='{ "preventDefaultContextMenuItems": true }'>
	<div class="loading-indicator"></div>
	<div class="image-load-error">
		<p>${vscode.l10n.t("An error occurred while loading the image.")}</p>
		<a href="#" class="open-file-link">${vscode.l10n.t("Open file using VS Code's standard text/binary editor?")}</a>
	</div>
	<script src="${escapeAttribute(this.extensionResource('media', 'imagePreview.js'))}" nonce="${nonce}"></script>
</body>
</html>`;
}
```

**Variations / call-sites:**
- `extensions/media-preview/src/audioPreview.ts:55-88` — Variant with media-src instead of img-src
- `extensions/media-preview/src/videoPreview.ts:56-92` — Includes configuration reading for autoplay/loop settings

---

#### Pattern: Webview API Bootstrap & State Management

**Where:** `extensions/media-preview/media/imagePreview.js:8-95`

**What:** Bootstraps VS Code webview API (acquireVsCodeApi), retrieves JSON settings from DOM metadata, manages local state with vscode.getState/setState for persistence.

```javascript
const vscode = acquireVsCodeApi();

const initialState = vscode.getState() || { scale: 'fit', offsetX: 0, offsetY: 0 };

// State
let scale = initialState.scale;
let ctrlPressed = false;
let altPressed = false;
let hasLoadedImage = false;
let consumeClick = true;
let isActive = false;

// Elements
const container = document.body;
const image = document.createElement('img');

function updateScale(newScale) {
	if (!image || !hasLoadedImage || !image.parentElement) {
		return;
	}

	if (newScale === 'fit') {
		scale = 'fit';
		image.classList.add('scale-to-fit');
		image.classList.remove('pixelated');
		// @ts-ignore Non-standard CSS property
		image.style.zoom = 'normal';
		image.style.minWidth = '';
		image.style.minHeight = '';
		vscode.setState(undefined);
	} else {
		scale = clamp(newScale, MIN_SCALE, MAX_SCALE);
		// ... zoom handling ...
		vscode.setState({ scale: scale, offsetX: newScrollX, offsetY: newScrollY });
	}

	vscode.postMessage({
		type: 'zoom',
		value: scale
	});
}

function getSettings() {
	const element = document.getElementById('image-preview-settings');
	if (element) {
		const data = element.getAttribute('data-settings');
		if (data) {
			return JSON.parse(data);
		}
	}

	throw new Error(`Could not load settings`);
}
```

**Variations / call-sites:**
- `extensions/media-preview/media/audioPreview.js:9-24` — Simpler variant without state management
- `extensions/media-preview/media/videoPreview.js` — Expected similar pattern (not fully examined)

---

#### Pattern: Resource URI Caching with Version Query Parameter

**Where:** `extensions/media-preview/src/imagePreview/index.ts:218-230`

**What:** Translates extension-relative and workspace resource URIs to webview-accessible URIs, appends version timestamp to bypass browser cache when file changes.

```typescript
private async getResourcePath(webviewEditor: vscode.WebviewPanel, resource: vscode.Uri, version: string): Promise<string> {
	if (resource.scheme === 'git') {
		const stat = await vscode.workspace.fs.stat(resource);
		if (stat.size === 0) {
			return this.emptyPngDataUri;
		}
	}

	// Avoid adding cache busting if there is already a query string
	if (resource.query) {
		return webviewEditor.webview.asWebviewUri(resource).toString();
	}
	return webviewEditor.webview.asWebviewUri(resource).with({ query: `version=${version}` }).toString();
}

private extensionResource(...parts: string[]) {
	return this._webviewEditor.webview.asWebviewUri(vscode.Uri.joinPath(this.extensionRoot, ...parts));
}
```

**Variations / call-sites:**
- `extensions/media-preview/src/audioPreview.ts:91-104` — Identical pattern with null handling for LFS files
- `extensions/media-preview/src/videoPreview.ts:95-108` — Identical implementation

---

#### Pattern: File System Watcher for Auto-Reload on Change

**Where:** `extensions/media-preview/src/mediaPreview.ts:52-64`

**What:** Monitors a resource file for changes and deletions using VS Code's workspace file system watcher, triggers re-render on modification and disposal on deletion.

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
- Embedded in `extensions/media-preview/src/mediaPreview.ts:21-112` base class for all preview types

---

#### Pattern: EventEmitter-based Status Bar Integration

**Where:** `extensions/media-preview/src/imagePreview/zoomStatusBarEntry.ts:14-50`

**What:** Exposes an EventEmitter for state changes (zoom scale), allows quick-pick UI for selecting values, and manages status bar visibility tied to active preview ownership.

```typescript
export class ZoomStatusBarEntry extends OwnedStatusBarEntry {

	private readonly _onDidChangeScale = this._register(new vscode.EventEmitter<{ scale: Scale }>());
	public readonly onDidChangeScale = this._onDidChangeScale.event;

	constructor() {
		super('status.imagePreview.zoom', vscode.l10n.t("Image Zoom"), vscode.StatusBarAlignment.Right, 102);

		this._register(vscode.commands.registerCommand(selectZoomLevelCommandId, async () => {
			type MyPickItem = vscode.QuickPickItem & { scale: Scale };

			const scales: Scale[] = [10, 5, 2, 1, 0.5, 0.2, 'fit'];
			const options = scales.map((scale): MyPickItem => ({
				label: this.zoomLabel(scale),
				scale
			}));

			const pick = await vscode.window.showQuickPick(options, {
				placeHolder: vscode.l10n.t("Select zoom level")
			});
			if (pick) {
				this._onDidChangeScale.fire({ scale: pick.scale });
			}
		}));

		this.entry.command = selectZoomLevelCommandId;
	}

	public show(owner: unknown, scale: Scale) {
		this.showItem(owner, this.zoomLabel(scale));
	}
}
```

**Variations / call-sites:**
- `extensions/media-preview/src/binarySizeStatusBarEntry.ts:37-50` — Simpler variant without EventEmitter
- `extensions/media-preview/src/ownedStatusBarEntry.ts:9-32` — Base class managing ownership-based visibility

---

#### Pattern: Disposable Resource Management via Abstract Base Class

**Where:** `extensions/media-preview/src/util/dispose.ts:17-42`

**What:** Abstract base class enforcing consistent disposal pattern via _register method, ensuring all subscriptions are cleaned up when parent disposes, preventing memory leaks.

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
- Inherited by `MediaPreview` in `extensions/media-preview/src/mediaPreview.ts:21`
- Extended by `PreviewStatusBarEntry` in `extensions/media-preview/src/ownedStatusBarEntry.ts:9`
- Extended by `ZoomStatusBarEntry` in `extensions/media-preview/src/imagePreview/zoomStatusBarEntry.ts:14`

---

## Summary

The media-preview extension demonstrates seven core patterns for porting VS Code's IDE functionality to Tauri:

1. **Custom Editor Registration** — Two-phase lifecycle (openCustomDocument/resolveCustomEditor) with provider pattern
2. **Webview IPC** — Type-safe message protocol between extension host and webview renderer
3. **Secure HTML Generation** — CSP-aware template generation with nonce injection and URI sanitization
4. **Webview Bootstrap** — Acquisition of VS Code API, settings injection via DOM metadata, persistent state management
5. **Resource URI Translation** — Cache-busting for binary files with version query parameters and git scheme handling
6. **File Monitoring** — Reactive updates via FileSystemWatcher triggering re-renders
7. **UI Integration** — EventEmitter patterns for status bar, quick-pick dialogs, and ownership-based visibility

All patterns use TypeScript/Electron abstractions (vscode.*) that would require Rust/Tauri equivalents for core IDE porting: webview message bridges, file system APIs, status bar registration, command registration, and localization (l10n).

