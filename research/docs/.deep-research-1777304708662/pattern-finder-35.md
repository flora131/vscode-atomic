# VS Code Webview Patterns: Mermaid Chat Features Extension

## Research Question
What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

## Scope Analysis
The `extensions/mermaid-chat-features/` directory (14 files, 1,261 LOC) provides concrete examples of:
- Webview lifecycle management (creation, registration, disposal)
- Bidirectional messaging between extension code and webview code
- HTML/CSS/JavaScript content generation with security considerations (CSP)
- State persistence and restoration
- UI interaction patterns (pan/zoom, button controls)

These patterns are fundamental to understanding how VS Code manages UI through webviews, which would need substantial reimplementation in a Tauri port.

---

## Pattern 1: Webview Panel Creation and Configuration

**Found in**: `extensions/mermaid-chat-features/src/editorManager.ts:135-142`

**What**: Creating a webview panel with configuration options and security constraints.

```typescript
const webviewPanel = vscode.window.createWebviewPanel(
	mermaidEditorViewType,
	title ?? vscode.l10n.t('Mermaid Diagram'),
	viewColumn,
	{
		retainContextWhenHidden: false,
	}
);
```

**Key aspects**:
- Uses `vscode.window.createWebviewPanel()` - the primary Electron API for webview creation
- Passes view type identifier (unique string per webview kind)
- Supports localization (`vscode.l10n.t()`)
- Configuration object controls retention behavior
- Returns `vscode.WebviewPanel` object for lifecycle management

**Variations**:
- Chat output renderers use `vscode.chat.registerChatOutputRenderer()` instead
- Both patterns rely on centralized VS Code webview APIs

**Related**: Pattern 2 covers webview option configuration separately.

---

## Pattern 2: Webview Security Configuration (CSP and Resource Roots)

**Found in**: `extensions/mermaid-chat-features/src/chatOutputRenderer.ts:57-60` and `editorManager.ts:168-173`

**What**: Setting up Content Security Policy (CSP) and sandboxing for webviews.

```typescript
webview.options = {
	enableScripts: true,
	localResourceRoots: [mediaRoot],
};
```

And HTML embedding with CSP headers:

```typescript
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'nonce-${nonce}'; style-src ${webview.cspSource} 'unsafe-inline'; font-src data:;" />
```

**Key aspects**:
- `enableScripts: true` required for interactive functionality
- `localResourceRoots` restricts file system access to specific directories
- `nonce` attribute prevents inline script execution without nonce match
- `webview.cspSource` is VS Code's computed safe stylesheet origin
- Default-deny CSP (`default-src 'none'`) with specific allowlists

**Challenge for Tauri port**:
- Tauri has different security model (uses IPC, not Electron's native message passing)
- CSP enforcement would need adaptation to Tauri's `tauri://` protocol
- Resource path translation (`asWebviewUri`) has no direct Tauri equivalent

---

## Pattern 3: HTML Content Generation with Dynamic Resource URIs

**Found in**: `extensions/mermaid-chat-features/src/chatOutputRenderer.ts:67-124` and `editorManager.ts:215-281`

**What**: Generating webview HTML with transformed resource URIs and dynamically injected configuration.

```typescript
const nonce = generateUuid();
const mermaidScript = vscode.Uri.joinPath(mediaRoot, 'index.js');
const codiconsUri = webview.asWebviewUri(vscode.Uri.joinPath(mediaRoot, 'codicon.css'));

webview.html = `
	<!DOCTYPE html>
	<html lang="en">
	<head>
		<meta charset="UTF-8">
		<meta name="viewport" content="width=device-width, initial-scale=1.0">
		<title>Mermaid Diagram</title>
		<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'nonce-${nonce}'; style-src ${webview.cspSource} 'unsafe-inline'; font-src data:;" />
		<link rel="stylesheet" type="text/css" href="${codiconsUri}">
	</head>
	<body data-vscode-context='${JSON.stringify({ preventDefaultContextMenuItems: true, mermaidWebviewId: webviewId })}'>
		<button class="open-in-editor-btn" title="${vscode.l10n.t('Open in Editor')}"><i class="codicon codicon-open-preview"></i></button>
		<pre class="mermaid">
			${escapeHtmlText(mermaidSource)}
		</pre>
		<script type="module" nonce="${nonce}" src="${webview.asWebviewUri(mermaidScript)}"></script>
	</body>
	</html>`;
```

**Key aspects**:
- `webview.asWebviewUri()` transforms extension resource paths to webview-accessible URIs
- Dynamic nonce injection prevents CSRF/XSS
- Data attributes embed context for webview-to-extension messaging
- HTML escaping via `escapeHtmlText()` prevents injection
- Supports `data:` URLs for icons and fonts
- Module scripts with nonce enforcement

**Challenge for Tauri port**:
- `asWebviewUri()` is Electron-specific; Tauri uses `asset://` protocol differently
- No equivalent "webview context" metadata system
- Resource path handling fundamentally different (Rust/file system vs. Node.js paths)

---

## Pattern 4: Webview Lifecycle and Registration Management

**Found in**: `extensions/mermaid-chat-features/src/webviewManager.ts:19-73`

**What**: Central manager maintaining webview references, active state, and disposal.

```typescript
export class MermaidWebviewManager {
	private _activeWebviewId: string | undefined;
	private readonly _webviews = new Map<string, MermaidWebviewInfo>();

	public registerWebview(id: string, webview: vscode.Webview, mermaidSource: string, title: string | undefined, type: 'chat' | 'editor'): vscode.Disposable {
		if (this._webviews.has(id)) {
			throw new Error(`Webview with id ${id} is already registered.`);
		}
		const info: MermaidWebviewInfo = {
			id,
			webview,
			mermaidSource,
			title,
			type
		};
		this._webviews.set(id, info);
		return { dispose: () => this.unregisterWebview(id) };
	}

	private unregisterWebview(id: string): void {
		this._webviews.delete(id);
		if (this._activeWebviewId === id) {
			this._activeWebviewId = undefined;
		}
	}

	public setActiveWebview(id: string): void {
		if (this._webviews.has(id)) {
			this._activeWebviewId = id;
		}
	}

	public resetPanZoom(id: string | undefined): void {
		const target = id ? this._webviews.get(id) : this.activeWebview;
		target?.webview.postMessage({ type: 'resetPanZoom' });
	}
}
```

**Key aspects**:
- Registry pattern with string IDs for webview identification
- Tracks active webview state globally
- Returns `Disposable` for cleanup subscription
- Bidirectional access (query by ID or get active)
- Sends messages via `postMessage()` API

**Variations**:
- Chat output uses disposables for cleanup on `chatOutputWebview.onDidDispose()`
- Editor manager uses `WebviewPanelSerializer` for state restoration

**Challenge for Tauri port**:
- No built-in webview lifecycle events or messaging API
- Would require custom IPC bridge implementation
- State management would be application-level, not framework-provided

---

## Pattern 5: Bidirectional Messaging (Extension ↔ Webview)

**Found in**: `extensions/mermaid-chat-features/src/chatOutputRenderer.ts:44-47` and `chat-webview-src/index.ts:8-24`

**Extension side**:

```typescript
disposables.push(webview.onDidReceiveMessage(message => {
	if (message.type === 'openInEditor') {
		vscode.commands.executeCommand('_mermaid-chat.openInEditor', { mermaidWebviewId: webviewId });
	}
}));
```

**Webview side**:

```typescript
declare function acquireVsCodeApi(): VsCodeApi;
const vscode = acquireVsCodeApi();

const openBtn = document.querySelector('.open-in-editor-btn');
if (openBtn) {
	openBtn.addEventListener('click', e => {
		e.stopPropagation();
		vscode.postMessage({ type: 'openInEditor' });
	});
}
```

**Key aspects**:
- `acquireVsCodeApi()` provides scoped communication handle
- Messages are plain JSON objects with `type` field
- `webview.onDidReceiveMessage()` event-based on extension side
- One-way messaging pattern (postMessage without direct response)
- Commands as RPC target (`vscode.commands.executeCommand`)

**Challenge for Tauri port**:
- `acquireVsCodeApi()` is injected global function - needs reimplementation
- Tauri uses `invoke()` for IPC with explicit function names
- Would require wrapper layer to match VS Code's message-passing semantics

---

## Pattern 6: Webview State Persistence and Restoration

**Found in**: `extensions/mermaid-chat-features/chat-webview-src/mermaidWebview.ts:202-227` and `editorManager.ts:58-78`

**Webview state save/restore**:

```typescript
private saveState(): void {
	this.hasInteracted = true;
	const currentState = this.vscode.getState() || {};
	this.vscode.setState({
		...currentState,
		panZoom: {
			scale: this.scale,
			translateX: this.translateX,
			translateY: this.translateY
		}
	});
}

private restoreState(): boolean {
	const state = this.vscode.getState();
	if (state?.panZoom) {
		const panZoom = state.panZoom as PanZoomState;
		this.scale = panZoom.scale ?? 1;
		this.translateX = panZoom.translateX ?? 0;
		this.translateY = panZoom.translateY ?? 0;
		this.hasInteracted = true;
		this.applyTransform();
		return true;
	}
	return false;
}
```

**Extension serialization**:

```typescript
public async deserializeWebviewPanel(
	webviewPanel: vscode.WebviewPanel,
	state: MermaidPreviewState
): Promise<void> {
	if (!state?.mermaidSource) {
		webviewPanel.webview.html = this._getErrorHtml();
		return;
	}
	const webviewId = getWebviewId(state.mermaidSource);
	const preview = MermaidPreview.revive(
		webviewPanel,
		webviewId,
		state.mermaidSource,
		this._extensionUri,
		this._webviewManager
	);
	this._registerPreview(preview);
}
```

**Key aspects**:
- `vscode.getState()/setState()` on webview side for client-side storage
- `WebviewPanelSerializer` interface for persistence on extension side
- State round-trips through extension context (file storage)
- Separate concerns: UI state (pan/zoom) vs. content state (mermaid source)
- Error handling for missing/corrupted state

**Challenge for Tauri port**:
- Would need custom storage backend (IndexedDB? localStorage?)
- Serialization format not standardized across platform
- No framework support for webview state snapshots

---

## Pattern 7: Interactive UI with Event Delegation and Dynamic Behavior

**Found in**: `extensions/mermaid-chat-features/chat-webview-src/mermaidWebview.ts:54-74` and `index-editor.ts:12-25`

**Chat output (simple)**:

```typescript
const openBtn = document.querySelector('.open-in-editor-btn');
if (openBtn) {
	openBtn.addEventListener('click', e => {
		e.stopPropagation();
		vscode.postMessage({ type: 'openInEditor' });
	});
}
```

**Editor preview (complex)**:

```typescript
initializeMermaidWebview(vscode).then(panZoomHandler => {
	if (!panZoomHandler) {
		return;
	}

	// Wire up zoom controls
	const zoomInBtn = document.querySelector('.zoom-in-btn');
	const zoomOutBtn = document.querySelector('.zoom-out-btn');
	const zoomResetBtn = document.querySelector('.zoom-reset-btn');

	zoomInBtn?.addEventListener('click', () => panZoomHandler.zoomIn());
	zoomOutBtn?.addEventListener('click', () => panZoomHandler.zoomOut());
	zoomResetBtn?.addEventListener('click', () => panZoomHandler.reset());
});
```

**Pan/zoom state management**:

```typescript
private setupEventListeners(): void {
	this.container.addEventListener('mousedown', e => this.handleMouseDown(e));
	document.addEventListener('mousemove', e => this.handleMouseMove(e));
	document.addEventListener('mouseup', () => this.handleMouseUp());
	this.container.addEventListener('click', e => this.handleClick(e));
	this.container.addEventListener('wheel', e => this.handleWheel(e), { passive: false });
	window.addEventListener('keydown', e => this.handleKeyChange(e));
	window.addEventListener('keyup', e => this.handleKeyChange(e));
	window.addEventListener('resize', () => this.handleResize());
}

private handleWheel(e: WheelEvent): void {
	const isPinchZoom = e.ctrlKey;
	if (!e.altKey && !isPinchZoom) {
		return;
	}
	e.preventDefault();
	e.stopPropagation();
	// ... zoom calculation
}
```

**Key aspects**:
- Standard DOM event listeners for user interaction
- Async initialization with promise-based setup
- Modifier key detection (Alt, Shift, Ctrl) for context-sensitive behavior
- Passive/non-passive event listener handling
- State tracking across multiple events (isPanning, hasDragged, etc.)
- Cursor feedback based on interaction state

**Characteristics**:
- Fully client-side interaction logic
- No dependency on extension-side event handling
- Browser-standard APIs (no Electron-specific features)
- Encapsulated state machine (PanZoomHandler class)

---

## Cross-Pattern: Disposable Pattern for Resource Management

**Found in**: `extensions/mermaid-chat-features/src/util/dispose.ts:15-40`

**What**: Base class pattern for managing lifecycle and preventing resource leaks.

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

**Usage in MermaidEditorManager**:

```typescript
export class MermaidEditorManager extends Disposable implements vscode.WebviewPanelSerializer {
	// ...
	private _registerPreview(preview: MermaidPreview): void {
		this._previews.set(preview.diagramId, preview);
		preview.onDispose(() => {
			this._previews.delete(preview.diagramId);
		});
	}

	public override dispose(): void {
		super.dispose();
		for (const preview of this._previews.values()) {
			preview.dispose();
		}
		this._previews.clear();
	}
}
```

**Key aspects**:
- Template Method pattern for guaranteed cleanup
- Guards against double-disposal
- Automatic child disposal on parent disposal
- Type-safe generic registration
- Used throughout VS Code extensions

**Challenge for Tauri port**:
- Rust's ownership system makes this pattern partially unnecessary
- But application lifecycle management would still need equivalent
- RAII (Resource Acquisition Is Initialization) is Rust's native approach

---

## Summary: Key Findings for Tauri Port

### Core Challenges

1. **Webview Creation API**: VS Code uses Electron's native webview; Tauri embeds webviews differently through WebKit/WRY.
   - No `createWebviewPanel()` equivalent
   - No `WebviewPanelSerializer` for state restoration
   - Different resource path handling

2. **Messaging Protocol**: VS Code's `postMessage()`/`onDidReceiveMessage()` with `acquireVsCodeApi()` would need reimplementation.
   - Tauri uses explicit `invoke()` calls with function registry
   - Would require wrapper layer or fundamental architecture change

3. **Security Model**: CSP, nonce-based script execution, and resource sandboxing work differently.
   - Tauri's `tauri://` protocol has different trust model
   - No equivalent to VS Code's computed `webview.cspSource`

4. **State Persistence**: `getState()/setState()` on webview side is Electron-specific.
   - Would need custom storage layer (localStorage or equivalent)
   - Serialization format would be application-defined

5. **Lifecycle Management**: VS Code's `Disposable` pattern and event-based cleanup.
   - Tauri would rely on Rust's ownership semantics
   - Still need equivalent application-level lifecycle management

### Transferable Patterns

1. **Architecture**: Separation of extension code and webview code is sound and portable.
2. **State machine design**: Pan/zoom handler and interaction logic are browser-standard (fully portable).
3. **Event handling**: DOM-based event listeners and keyboard/mouse handling are standard.
4. **HTML generation**: Template-based HTML strings with dynamic content injection (portable with security adjustments).
5. **Disposable pattern**: Concept maps well to Rust RAII, though syntax differs significantly.

### Porting Effort Estimate

- **Webview system**: 70-80% rewrite required (API layer completely different)
- **Messaging layer**: 90% rewrite required (IPC protocol fundamentally different)
- **Security/CSP**: 60-70% rewrite required (model differs significantly)
- **State management**: 50-60% rewrite required (needs custom storage backend)
- **UI logic**: 5-10% rewrite required (mostly portable browser code)

**Total impact**: A Tauri port would require substantial reimplementation of the webview subsystem, roughly equivalent to rewriting 30-40% of the core IDE architecture.

