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

