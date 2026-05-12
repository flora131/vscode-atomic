# Partition 39 of 80 — Findings

## Scope
`extensions/simple-browser/` (10 files, 636 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator 39: simple-browser Extension - Webview Panel API Usage

## Implementation

**Primary webview-panel API usage:**
- `extensions/simple-browser/src/simpleBrowserView.ts` — Contains the core `SimpleBrowserView` class using `vscode.window.createWebviewPanel()` at line 45. Demonstrates:
  - Webview panel creation with viewType and title
  - `retainContextWhenHidden: true` option
  - Webview options configuration (enableScripts, enableForms, localResourceRoots)
  - Message handling via `onDidReceiveMessage`
  - CSP policy configuration for webview content
  - Webview HTML rendering with content updates

- `extensions/simple-browser/src/extension.ts` — Extension activation and command registration:
  - Calls `vscode.window.registerWebviewPanelSerializer()` for panel restoration (line 55)
  - Registers external URI opener using webview panels for localhost/127.0.0.1 URLs
  - Command handlers for `simpleBrowser.show` and `simpleBrowser.api.open`

- `extensions/simple-browser/src/simpleBrowserManager.ts` — Webview panel lifecycle management:
  - `show()` method creates panels via `SimpleBrowserView.create()` or reuses active panel
  - `restore()` method handles deserialization of persisted panels
  - Tracks active webview with disposal listeners

**Webview content (rendered in iframe):**
- `extensions/simple-browser/preview-src/index.ts` — Client-side webview script handling:
  - Acquires VS Code API via `acquireVsCodeApi()`
  - Posts messages back to extension (`vscode.postMessage()`)
  - Manages iframe navigation, browser controls (back/forward/reload)
  - Listens for configuration changes from parent extension
  - State persistence via `vscode.setState()`

- `extensions/simple-browser/preview-src/events.ts` — Document lifecycle utility

## Types / Interfaces

- `extensions/simple-browser/src/simpleBrowserView.ts` — `ShowOptions` interface (lines 11-14):
  - `preserveFocus?: boolean`
  - `viewColumn?: vscode.ViewColumn`

## Configuration

- `extensions/simple-browser/package.json` — Extension manifest:
  - Activation events: `onCommand:simpleBrowser.api.open`, `onOpenExternalUri:http|https`, `onWebviewPanel:simpleBrowser.view`
  - Configuration property: `simpleBrowser.focusLockIndicator.enabled` (boolean, default true)
  - API proposals: `externalUriOpener`
  - Engine requirement: `vscode: ^1.70.0`

- `extensions/simple-browser/tsconfig.json` — TypeScript configuration extending base, includes vscode.d.ts and proposed API types

## Examples / Fixtures

- `extensions/simple-browser/media/main.css` — Styling for webview UI (header, controls, iframe container)
- `extensions/simple-browser/media/icon.png` — Extension icon

## Documentation

- `extensions/simple-browser/README.md` — Notes that extension is bundled with VS Code; provides basic iframe-based browser preview for webview content

## Notable Clusters

**Message protocol between webview and extension:**
- Extension → Webview: `didChangeFocusLockIndicatorEnabled` (config change notifications)
- Webview → Extension: `openExternal` (open URL in system browser)

**Build configuration:**
- `extensions/simple-browser/esbuild.mts` — Extension build (node platform)
- `extensions/simple-browser/esbuild.webview.mts` — Referenced but not examined (webview script bundling)
- `extensions/simple-browser/esbuild.browser.mts` — Web platform build

**Disposal pattern:**
- `extensions/simple-browser/src/dispose.ts` — Abstract `Disposable` base class managing lifecycle; used by SimpleBrowserView for resource cleanup

**UUID generation:**
- `extensions/simple-browser/src/uuid.ts` — Generates nonces for CSP inline script tags

---

## Summary

The simple-browser extension demonstrates a complete, production-grade webview panel pattern in VS Code. Key findings for Tauri porting:

1. **Webview Panel API Surface**: Panel creation requires viewType, title, view column/focus options, and multi-option configuration (script/form/resource-root enablement, context retention). Tauri webview equivalents would need to support similar creation semantics and lifecycle management.

2. **State Serialization**: VS Code provides `registerWebviewPanelSerializer()` for persistent panel restoration. Tauri would need comparable panel state save/restore infrastructure.

3. **Message Channel**: Bidirectional message passing (extension ↔ webview) via `postMessage()` / `onDidReceiveMessage`. Tauri's IPC mechanisms would need similar request-response and event notification patterns.

4. **Content Security Policy**: Inline CSP meta-tags with nonce-based script execution. This pattern is fundamental to webview content sandboxing and would require careful mapping to Tauri's security model.

5. **Configuration Integration**: Real-time workspace configuration changes propagated to webviews. Tauri would need equivalent configuration change notification infrastructure.

The extension's reliance on `vscode.window.createWebviewPanel()` and the associated API (WebviewPanel, WebviewOptions, ViewColumn) represents the core abstraction layer that would need Tauri equivalents for functional parity.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Webview Panel Usage Patterns: Simple Browser Extension

Research question: What webview-panel usage patterns exist in VS Code that would inform a Tauri/Rust port?

## Patterns Found

#### Pattern: Panel Creation with Static Factory Method
**Where:** `extensions/simple-browser/src/simpleBrowserView.ts:40-53`
**What:** Static factory creates webview panel with view type, title, options, and webview options configuration.
```typescript
public static create(
	extensionUri: vscode.Uri,
	url: string,
	showOptions?: ShowOptions
): SimpleBrowserView {
	const webview = vscode.window.createWebviewPanel(SimpleBrowserView.viewType, SimpleBrowserView.title, {
		viewColumn: showOptions?.viewColumn ?? vscode.ViewColumn.Active,
		preserveFocus: showOptions?.preserveFocus
	}, {
		retainContextWhenHidden: true,
		...SimpleBrowserView.getWebviewOptions(extensionUri)
	});
	return new SimpleBrowserView(extensionUri, url, webview);
}
```

**Variations / call-sites:**
- Called from `simpleBrowserManager.ts:27` during `show()` method
- Uses spread operator to merge webview options
- Accepts optional ShowOptions parameter to control panel behavior
- View type stored as static constant (`viewType = 'simpleBrowser.view'`)

---

#### Pattern: Webview Options Configuration
**Where:** `extensions/simple-browser/src/simpleBrowserView.ts:21-33`
**What:** Centralized webview options with local resource roots and security settings, computed at panel creation time.
```typescript
private static getWebviewLocalResourceRoots(extensionUri: vscode.Uri): readonly vscode.Uri[] {
	return [
		vscode.Uri.joinPath(extensionUri, 'media')
	];
}

private static getWebviewOptions(extensionUri: vscode.Uri): vscode.WebviewOptions {
	return {
		enableScripts: true,
		enableForms: true,
		localResourceRoots: SimpleBrowserView.getWebviewLocalResourceRoots(extensionUri),
	};
}
```

**Variations / call-sites:**
- Options applied during panel creation (`createWebviewPanel` call)
- Options reassigned after construction (`this._webviewPanel.webview.options = ...`)
- Both `enableScripts` and `enableForms` required for interactive browser
- Allows access to extension media directory only

---

#### Pattern: Disposable Resource Registration
**Where:** `extensions/simple-browser/src/dispose.ts:15-40`
**What:** Disposable base class registers panel and listeners, tracking cleanup order with `_register()` pattern.
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
}
```

**Variations / call-sites:**
- WebviewPanel registered: `this._webviewPanel = this._register(webviewPanel)` (line 70)
- Event listeners registered: `this._register(this._webviewPanel.webview.onDidReceiveMessage(...))` (line 73)
- Config listeners registered: `this._register(vscode.workspace.onDidChangeConfiguration(...))` (line 90)
- `onDidDispose` event fires: `this._onDidDispose.fire()` (line 104)

---

#### Pattern: Message-Based Communication (Extension → Webview)
**Where:** `extensions/simple-browser/src/simpleBrowserView.ts:90-98`
**What:** Configuration changes broadcast to webview via `postMessage()` with typed message payload.
```typescript
this._register(vscode.workspace.onDidChangeConfiguration(e => {
	if (e.affectsConfiguration('simpleBrowser.focusLockIndicator.enabled')) {
		const configuration = vscode.workspace.getConfiguration('simpleBrowser');
		this._webviewPanel.webview.postMessage({
			type: 'didChangeFocusLockIndicatorEnabled',
			focusLockEnabled: configuration.get<boolean>('focusLockIndicator.enabled', true)
		});
	}
}));
```

**Variations / call-sites:**
- Used for config updates from extension context
- Webview receives via `window.addEventListener('message', e => {...})` (preview-src/index.ts:32)
- Payload uses `type` field for message discrimination
- Includes data properties alongside type field

---

#### Pattern: Message-Based Communication (Webview → Extension)
**Where:** `extensions/simple-browser/src/simpleBrowserView.ts:73-84`
**What:** Webview messages received via event listener, routed by message type with switch statement.
```typescript
this._register(this._webviewPanel.webview.onDidReceiveMessage(e => {
	switch (e.type) {
		case 'openExternal':
			try {
				const url = vscode.Uri.parse(e.url);
				vscode.env.openExternal(url);
			} catch {
				// Noop
			}
			break;
	}
}));
```

**Variations / call-sites:**
- Webview sends via `vscode.postMessage({type: 'openExternal', url: ...})` (preview-src/index.ts:71-74)
- Extension handles user interactions (external link opens)
- Error handling wraps potentially invalid URLs
- Type-based routing allows future message types

---

#### Pattern: Panel Reveal with ViewColumn Control
**Where:** `extensions/simple-browser/src/simpleBrowserView.ts:108-111`
**What:** Panel visibility controlled via `reveal()` method with optional viewColumn and preserveFocus parameters.
```typescript
public show(url: string, options?: ShowOptions) {
	this._webviewPanel.webview.html = this.getHtml(url);
	this._webviewPanel.reveal(options?.viewColumn, options?.preserveFocus);
}
```

**Variations / call-sites:**
- Default view column is `vscode.ViewColumn.Active` (set during creation)
- Alternative: `vscode.ViewColumn.Beside` (used in extension.ts:106)
- Called during manager's `show()` method for existing panels
- Updates HTML before reveal to avoid flicker

---

#### Pattern: Panel Serialization and Restoration
**Where:** `extensions/simple-browser/src/extension.ts:55-59`
**What:** Webview panel serializer restores persisted panels on VS Code restart.
```typescript
context.subscriptions.push(vscode.window.registerWebviewPanelSerializer(SimpleBrowserView.viewType, {
	deserializeWebviewPanel: async (panel, state) => {
		manager.restore(panel, state);
	}
}));
```

**Variations / call-sites:**
- Static factory `create()` creates new panels during user action
- Static method `restore()` reconstitutes from saved state (simpleBrowserView.ts:55-61)
- Restore extracts URL from state: `const url = state?.url ?? ''` (simpleBrowserManager.ts:35)
- State set via `vscode.setState({url: rawUrl})` from webview (preview-src/index.ts:107)

---

#### Pattern: HTML Content with CSP and Nonce
**Where:** `extensions/simple-browser/src/simpleBrowserView.ts:113-174`
**What:** Dynamic HTML generation with Content Security Policy using nonce-based script injection and webview CSP source.
```typescript
private getHtml(url: string) {
	const nonce = generateUuid();
	const mainJs = this.extensionResourceUrl('media', 'index.js');
	const mainCss = this.extensionResourceUrl('media', 'main.css');

	return /* html */ `<!DOCTYPE html>
		<html>
		<head>
			<meta http-equiv="Content-Security-Policy" content="
				default-src 'none';
				font-src data:;
				style-src ${this._webviewPanel.webview.cspSource};
				script-src 'nonce-${nonce}';
				frame-src *;
			">
			<!-- embedded config as data attribute -->
			<meta id="simple-browser-settings" data-settings="${escapeAttribute(JSON.stringify({
				url: url,
				focusLockEnabled: configuration.get<boolean>('focusLockIndicator.enabled', true)
			}))}">
			<!-- style and script references -->
			<link rel="stylesheet" href="${mainCss}">
			<script src="${mainJs}" nonce="${nonce}"></script>
		</head>
		<body><!-- content --></body>
		</html>`;
}
```

**Variations / call-sites:**
- `cspSource` used for stylesheet origins (line 130)
- Nonce requirement for inline/external scripts
- Resource URIs converted via `asWebviewUri()` (line 178)
- Configuration embedded as JSON in meta tag (lines 135-138)
- Called every time panel is shown (line 109)

---

#### Pattern: Manager-Based Panel Lifecycle
**Where:** `extensions/simple-browser/src/simpleBrowserManager.ts:9-48`
**What:** Manager maintains single active panel instance, creating on demand and restoring from saved state.
```typescript
export class SimpleBrowserManager {
	private _activeView?: SimpleBrowserView;

	public show(inputUri: string | vscode.Uri, options?: ShowOptions): void {
		const url = typeof inputUri === 'string' ? inputUri : inputUri.toString(true);
		if (this._activeView) {
			this._activeView.show(url, options);
		} else {
			const view = SimpleBrowserView.create(this.extensionUri, url, options);
			this.registerWebviewListeners(view);
			this._activeView = view;
		}
	}

	public restore(panel: vscode.WebviewPanel, state: any): void {
		const url = state?.url ?? '';
		const view = SimpleBrowserView.restore(this.extensionUri, url, panel);
		this.registerWebviewListeners(view);
		this._activeView ??= view;
	}

	private registerWebviewListeners(view: SimpleBrowserView) {
		view.onDispose(() => {
			if (this._activeView === view) {
				this._activeView = undefined;
			}
		});
	}
}
```

**Variations / call-sites:**
- Enforces single active view pattern (reuse vs. create)
- Called from extension commands: `simpleBrowser.show` (extension.ts:61-76)
- Called from API command: `simpleBrowser.api.open` (extension.ts:78-87)
- Registered with external URI opener (extension.ts:89-113)
- Cleanup via `onDispose` listener (lines 41-47)

---

## Summary

The simple-browser extension demonstrates a layered webview architecture for a Tauri port:

1. **Creation Pattern**: Static factory methods (`SimpleBrowserView.create()`) encapsulate `vscode.window.createWebviewPanel()` calls with consistent options.

2. **Configuration**: Webview options (scripts, forms, resource roots) and HTML/CSP setup happen at creation and can be updated post-creation.

3. **Resource Management**: Disposable base class (`_register()` pattern) ensures event listeners, panels, and configuration watchers are cleaned up in the correct order.

4. **Communication**: Bidirectional messaging via `postMessage()` and `onDidReceiveMessage()` with type-discriminated payloads for extension↔webview interaction.

5. **Panel Visibility**: `reveal()` method controls which view column the panel appears in; view column defaults to `Active` or can be overridden to `Beside`.

6. **Persistence**: Webview panels serialized to state via `registerWebviewPanelSerializer()`, allowing re-creation after VS Code restart. State passed to restore factory.

7. **Lifecycle Management**: Manager class (`SimpleBrowserManager`) enforces single active view pattern, delegating creation/restoration to view class and listening to disposal events.

All patterns use TypeScript's type system, event emitters from `vscode` module, and disposable resource tracking that would need translation to Rust/Tauri's lifecycle model, async task handling, and inter-process messaging.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
