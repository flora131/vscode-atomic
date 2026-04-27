# Pattern Analysis: Simple Browser Extension (Webview-Based)

Scope: `extensions/simple-browser/` (10 files, 636 LOC)
Focus: Webview embedding and host-to-webview communication patterns relevant to Tauri porting.

---

#### Pattern: Webview Panel Creation and Configuration

**Where:** `extensions/simple-browser/src/simpleBrowserView.ts:40-53`

**What:** Creates a VS Code webview panel with security settings (scripts enabled, forms enabled, local resource roots).

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

private static getWebviewOptions(extensionUri: vscode.Uri): vscode.WebviewOptions {
  return {
    enableScripts: true,
    enableForms: true,
    localResourceRoots: SimpleBrowserView.getWebviewLocalResourceRoots(extensionUri),
  };
}
```

**Variations / call-sites:**
- Used in `SimpleBrowserManager.show()` (line 27) to create new webview instances.
- `restore()` method (line 55-61) reuses existing panels on extension restart.
- Options include `retainContextWhenHidden: true` to preserve state when switching tabs.

---

#### Pattern: Webview HTML Generation with CSP and Nonce

**Where:** `extensions/simple-browser/src/simpleBrowserView.ts:113-175`

**What:** Dynamically generates HTML with Content-Security-Policy, nonce-based script loading, and embedded configuration data.

```typescript
private getHtml(url: string) {
  const configuration = vscode.workspace.getConfiguration('simpleBrowser');
  const nonce = generateUuid();
  const mainJs = this.extensionResourceUrl('media', 'index.js');
  const mainCss = this.extensionResourceUrl('media', 'main.css');
  const codiconsUri = this.extensionResourceUrl('media', 'codicon.css');

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
      
      <meta id="simple-browser-settings" data-settings="${escapeAttribute(JSON.stringify({
        url: url,
        focusLockEnabled: configuration.get<boolean>('focusLockIndicator.enabled', true)
      }))}">
      
      <link rel="stylesheet" type="text/css" href="${mainCss}">
      <script src="${mainJs}" nonce="${nonce}"></script>
    </head>
    <body>
      <iframe sandbox="allow-scripts allow-forms allow-same-origin allow-downloads"></iframe>
    </body>
    </html>`;
}
```

**Variations / call-sites:**
- Called in `show()` method (line 109) via `this._webviewPanel.webview.html = this.getHtml(url)`.
- Resource URLs use `asWebviewUri()` for security (line 178).
- Configuration injected as JSON in data attribute for webview script access.

---

#### Pattern: Bidirectional Message Communication

**Where:** `extensions/simple-browser/src/simpleBrowserView.ts:73-84`

**What:** Host listens for webview messages and responds with postMessage, enabling event-driven communication.

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

// Later, in configuration change handler:
this._webviewPanel.webview.postMessage({
  type: 'didChangeFocusLockIndicatorEnabled',
  focusLockEnabled: configuration.get<boolean>('focusLockIndicator.enabled', true)
});
```

**Variations / call-sites:**
- Webview sends `openExternal` message with `url` property (line 74-82).
- Host sends `didChangeFocusLockIndicatorEnabled` configuration changes (line 93-96).
- Message types used as dispatch keys; no shared type definitions visible.

---

#### Pattern: Webview-Side Message Handling with acquireVsCodeApi

**Where:** `extensions/simple-browser/preview-src/index.ts:1-45`

**What:** Webview script acquires VS Code API, parses embedded settings, and listens for host messages.

```typescript
const vscode = acquireVsCodeApi();

function getSettings() {
  const element = document.getElementById('simple-browser-settings');
  if (element) {
    const data = element.getAttribute('data-settings');
    if (data) {
      return JSON.parse(data);
    }
  }
  throw new Error(`Could not load settings`);
}

const settings = getSettings();

window.addEventListener('message', e => {
  switch (e.data.type) {
    case 'focus':
      {
        iframe.focus();
        break;
      }
    case 'didChangeFocusLockIndicatorEnabled':
      {
        toggleFocusLockIndicatorEnabled(e.data.enabled);
        break;
      }
  }
});

// Later, webview posts back to host:
vscode.postMessage({
  type: 'openExternal',
  url: input.value
});
```

**Variations / call-sites:**
- `acquireVsCodeApi()` is a global function injected by VS Code (no import).
- Settings parsed from data attribute on page load (line 22).
- Both host→webview and webview→host use message event handlers with type switching.

---

#### Pattern: Disposable Resource Management

**Where:** `extensions/simple-browser/src/dispose.ts:15-40`

**What:** Abstract disposable base class with registration pattern for cleanup on disposal.

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
- `SimpleBrowserView` extends `Disposable` (line 16 of simpleBrowserView.ts).
- Event listeners registered via `this._register(this._webviewPanel.webview.onDidReceiveMessage(...))` (line 73).
- Ensures cleanup of event listeners and webview panels on disposal.

---

#### Pattern: Configuration Change Listening and Dynamic Updates

**Where:** `extensions/simple-browser/src/simpleBrowserView.ts:90-98`

**What:** Listens for configuration changes and pushes updates to webview via postMessage.

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
- Checked in `getHtml()` for initial state (line 137).
- Webview handler at `preview-src/index.ts:39-43` toggles CSS class on receipt.
- Configuration schema defined in `package.json:53-65`.

---

#### Pattern: Webview Panel Serialization and Restoration

**Where:** `extensions/simple-browser/src/extension.ts:55-59`

**What:** Registers serializer to restore webview state across VS Code reloads.

```typescript
context.subscriptions.push(vscode.window.registerWebviewPanelSerializer(SimpleBrowserView.viewType, {
  deserializeWebviewPanel: async (panel, state) => {
    manager.restore(panel, state);
  }
}));
```

**Variations / call-sites:**
- `SimpleBrowserManager.restore()` (line 34-39 of simpleBrowserManager.ts) reconstructs view from serialized state.
- State includes `url` property set via `vscode.setState()` in webview (preview-src/index.ts:107).
- `retainContextWhenHidden: true` option (line 49 of simpleBrowserView.ts) preserves webview context.

---

## Summary

The simple-browser extension demonstrates five key webview patterns relevant to Tauri porting:

1. **Panel/Window Creation**: Configures security permissions (scripts, forms), resource roots, and visibility options at creation time.
2. **Dynamic HTML with CSP**: Generates HTML server-side with strict CSP, nonce-based script tags, and embedded JSON configuration.
3. **Bidirectional Messaging**: Uses `onDidReceiveMessage`/`postMessage` for event-driven host-webview communication with typed message objects.
4. **Webview-side API**: Webview accesses host via `acquireVsCodeApi()` global; parses settings from DOM; mirrors host's message dispatch pattern.
5. **Configuration Management**: Host listens for config changes, webview receives updates via postMessage, enabling reactive UI updates.
6. **Resource Management**: Disposable pattern with registered cleanup ensures all event listeners are disposed when webview closes.
7. **State Persistence**: Serializer pattern allows webview state (URL) to survive extension reload/restore.

All patterns use TypeScript with vscode.d.ts types. Webview build process uses esbuild to transpile `preview-src/*.ts` to `media/index.js`.
