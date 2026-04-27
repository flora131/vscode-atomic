# Partition 39 of 79 — Findings

## Scope
`extensions/simple-browser/` (10 files, 636 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Partition 39: Simple Browser Extension (WebView-Based Browser)

## Implementation

Extension entry point and core browser management:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/extension.ts` - Extension activation, command registration (simpleBrowser.api.open, simpleBrowser.show), external URI opener registration for http/https schemes targeting localhost and IPv4/IPv6 loopback addresses
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/simpleBrowserManager.ts` - Manager class handling lifecycle of browser views, show/restore functionality
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/simpleBrowserView.ts` - Core WebView panel implementation. Generates HTML with iframe sandbox, Content-Security-Policy headers, manages webview options (enableScripts, enableForms), handles configuration changes, and communication between extension and webview
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/dispose.ts` - Base Disposable class for resource cleanup pattern
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/uuid.ts` - UUID generation for nonce values in CSP headers

Webview frontend implementation:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/preview-src/index.ts` - Client-side browser logic: iframe navigation, history (back/forward/reload), URL input handling, focus lock indicator toggle, message passing via vscode.postMessage()
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/preview-src/events.ts` - DOM ready helper function

## Configuration

Extension manifest and metadata:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/package.json` - Extension metadata, version 10.0.0, activation events (onCommand, onOpenExternalUri, onWebviewPanel), configuration schema for focusLockIndicator.enabled, build scripts
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/package.nls.json` - Localization strings

Build configuration:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/tsconfig.json` - TypeScript config for src/ (Node types, VSCode API definitions)
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/tsconfig.browser.json` - Browser-specific TypeScript config extending base
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/preview-src/tsconfig.json` - TypeScript config for webview source

Build scripts:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/esbuild.mts` - Main extension build (references common build runner)
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/esbuild.webview.mts` - Webview bundle build (entry point: preview-src/index.ts, output: media/, includes codicon.css)
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/esbuild.browser.mts` - Browser platform extension build

Utility configs:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/.vscodeignore` - Extension packaging exclusions
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/.npmrc` - NPM configuration
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/.gitignore` - Git exclusions

## Documentation

- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/README.md` - Basic overview describing Simple Browser as iframe-embedded preview for other extensions

## Examples / Fixtures

Media assets:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/media/main.css` - Webview UI styling: header with navigation controls (back/forward/reload/open-external buttons), URL input field, iframe container, focus lock indicator alert box
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/media/icon.png` - Extension icon
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/media/preview-light.svg` - Light theme preview
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/media/preview-dark.svg` - Dark theme preview

Lock file:
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/package-lock.json` - NPM dependency lock

## Notable Clusters

**WebView HTML Generation** (`simpleBrowserView.ts` lines 113-174): Dynamic HTML construction with:
- CSP enforcement: `default-src 'none'`, `frame-src *`, script nonce injection
- Settings injection via data attribute (`simple-browser-settings`)
- Iframe sandbox attributes: `allow-scripts allow-forms allow-same-origin allow-downloads`
- Codicon icon font integration

**Focus Lock Detection** (implemented across components): Periodic DOM polling (50ms interval) checking if iframe is focused element, toggling CSS class for visual indicator

**Message Bridge**: Two-way communication between extension and webview:
- Extension → Webview: `focus`, `didChangeFocusLockIndicatorEnabled` messages
- Webview → Extension: `openExternal` message to open URLs in system browser

**Build Output Targets**: 
- Desktop/Node: `out/extension.js` (from src/)
- Browser/Web: `dist/browser/extension.js` (from src/)
- Webview: `media/index.js` (from preview-src/)

## Summary

The Simple Browser extension is a lightweight webview-based browser preview component (636 LOC across 10 primary source files) that embeds HTML content in iframe sandboxes with strict CSP policies. It demonstrates VS Code's webview API for iframe embedding, command and URI handler registration, bidirectional messaging between extension and webview contexts, and configuration-driven UI behavior (focus lock indicator). The architecture separates extension logic (TypeScript), webview frontend (TypeScript compiled to client-side JavaScript), and build pipeline optimization for both desktop and web platforms. This serves as a reference implementation for Tauri's webview embedding capabilities, showing how to securely instantiate nested web contexts with controlled CSP headers and cross-context messaging.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

1. `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/extension.ts` (119 LOC)
2. `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/simpleBrowserManager.ts` (49 LOC)
3. `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/simpleBrowserView.ts` (185 LOC)
4. `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/preview-src/index.ts` (114 LOC)
5. `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/dispose.ts` (40 LOC)
6. `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/src/uuid.ts` (58 LOC)
7. `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/preview-src/events.ts` (12 LOC)

---

### Per-File Notes

#### `src/extension.ts`

**Activation and command registration.**

- Line 19–31: `enabledHosts` is a hardcoded `Set<string>` containing `localhost`, `127.0.0.1`, IPv6 loopback variants `[::1]`, `[0:0:0:0:0:0:0:1]`, and all-interfaces addresses `0.0.0.0`, `[::]`, `[0:0:0:0:0:0:0:0]`. This set gates whether the external URI opener intercepts a link.
- Line 38–41: `shouldUseIntegratedBrowser()` calls `vscode.commands.getCommands(true)` and checks for presence of `workbench.action.browser.open`. When true, all three entry points (`showCommand`, `openApiCommand`, `openExternalUri`) delegate to `openInIntegratedBrowser` at line 46–48 instead.
- Line 61–76: `simpleBrowser.show` command. Prompts for a URL via `showInputBox` if none is provided (lines 67–70), then calls `manager.show(url)` at line 74.
- Line 78–87: `simpleBrowser.api.open` command accepts a `vscode.Uri` plus an optional `ShowOptions` object with `preserveFocus` and `viewColumn`.
- Line 89–113: `registerExternalUriOpener` with id `simpleBrowser.open` on schemes `['http', 'https']`. `canOpenExternalUri` at line 90 returns `ExternalUriOpenerPriority.Default` on web (line 94) or `ExternalUriOpenerPriority.Option` on desktop (line 96) if the hostname is in `enabledHosts`, otherwise `None`. `openExternalUri` at line 101 resolves position beside the active text editor if one is open (line 106).
- Line 116–118: `isWeb()` detects the web environment by checking `typeof process === 'object'` is false AND `vscode.env.uiKind === vscode.UIKind.Web`.

#### `src/simpleBrowserManager.ts`

**Single-panel lifecycle manager.**

- Line 11: `_activeView` is a single nullable `SimpleBrowserView` reference. The manager enforces at most one panel at a time.
- Line 22–31: `show()` reuses the existing view by calling `this._activeView.show(url, options)` (line 25) if one exists; otherwise creates a new panel via `SimpleBrowserView.create(...)` at line 27.
- Line 34–39: `restore()` is called by the `WebviewPanelSerializer` (registered at `extension.ts:55–58`). Reads `state?.url` from the persisted state and calls `SimpleBrowserView.restore(...)`. Uses `??=` at line 38 so the first restored view becomes active without overwriting an already-active view.
- Line 41–46: `registerWebviewListeners()` subscribes to `view.onDispose` and nulls out `_activeView` only if the disposed view is the current one. This prevents a race where a stale restored view clears a newer active view.

#### `src/simpleBrowserView.ts`

**WebviewPanel wrapper and HTML generation.**

- Line 21–25: `getWebviewLocalResourceRoots` restricts local resource access to `extensionUri/media` only.
- Line 27–33: `getWebviewOptions` enables scripts and forms (`enableScripts: true`, `enableForms: true`) while limiting `localResourceRoots` to the `media` folder.
- Line 40–53: `create()` static factory calls `vscode.window.createWebviewPanel` with `retainContextWhenHidden: true` (line 49), preserving JavaScript state across panel hide/show cycles.
- Line 63–101: Constructor. Registers three listeners:
  - Line 73–84: `onDidReceiveMessage` handler. Only handles `type: 'openExternal'`; parses `e.url` into a `vscode.Uri` at line 77 and calls `vscode.env.openExternal(url)`.
  - Line 86–88: `onDidDispose` — calls `this.dispose()`.
  - Line 90–98: `onDidChangeConfiguration` — watches `simpleBrowser.focusLockIndicator.enabled`. On change, posts `{ type: 'didChangeFocusLockIndicatorEnabled', focusLockEnabled: boolean }` to the webview.
- Line 108–111: `show()` regenerates the full webview HTML on every call (line 109) and then reveals the panel. There is no diffing; the entire HTML is replaced.
- Line 113–175: `getHtml(url)` template. Key details:
  - Line 116: `nonce = generateUuid()` — fresh nonce per HTML generation.
  - Lines 127–133: CSP: `default-src 'none'`, `font-src data:`, `style-src <cspSource>`, `script-src 'nonce-${nonce}'`, `frame-src *`. The wildcard `frame-src *` allows the iframe to load any URL.
  - Line 135–138: Settings serialized into a `data-settings` attribute on a `<meta id="simple-browser-settings">` element as escaped JSON containing `url` and `focusLockEnabled`.
  - Line 169: `<iframe sandbox="allow-scripts allow-forms allow-same-origin allow-downloads">` — no `allow-top-navigation` or `allow-popups`, constraining what the embedded page can do.
  - Line 172: `<script src="${mainJs}" nonce="${nonce}">` — the compiled `preview-src/index.ts` bundle.
- Line 177–179: `extensionResourceUrl()` calls `webview.asWebviewUri(Uri.joinPath(extensionUri, ...parts))` to produce a webview-safe URI for bundled assets.
- Line 182–184: `escapeAttribute()` replaces `"` with `&quot;` to safely inline JSON into an HTML attribute.

#### `preview-src/index.ts`

**In-webview browser chrome logic.**

- Line 8: `acquireVsCodeApi()` — standard VS Code webview bootstrap; the returned `vscode` handle is used for `postMessage` and `setState`.
- Line 10–19: `getSettings()` reads the `data-settings` attribute from `#simple-browser-settings` and JSON-parses it to get `{ url, focusLockEnabled }`.
- Line 32–45: `window.addEventListener('message', ...)` handles two inbound message types from the extension host:
  - `focus`: calls `iframe.focus()`.
  - `didChangeFocusLockIndicatorEnabled`: calls `toggleFocusLockIndicatorEnabled(e.data.enabled)`.
- Line 47–51: Polling loop via `setInterval(..., 50)` checks whether `document.activeElement?.tagName === 'IFRAME'` and toggles the `iframe-focused` CSS class on `document.body` every 50 ms.
- Line 57–59: URL input `change` event calls `navigateTo(url)`.
- Line 62–64: Forward button calls `history.forward()`.
- Line 65–67: Back button calls `history.back()`.
- Line 70–75: "Open external" button posts `{ type: 'openExternal', url: input.value }` via `vscode.postMessage`.
- Line 77–85: Reload button calls `navigateTo(input.value)` (not `location.reload()` or `history.go(0)`, per inline comment at lines 79–82 noting those approaches are unreliable).
- Line 92–108: `navigateTo(rawUrl)` constructs a `new URL(rawUrl)`, then appends two query params: `id` (copied from current `location.search`) and `vscodeBrowserReqId` set to `Date.now()` (line 99) to bust the iframe cache. Sets `iframe.src` at line 102. Then calls `vscode.setState({ url: rawUrl })` at line 107 to persist state for panel restore.

#### `src/dispose.ts`

**Disposable base class.**

- Line 15–40: Abstract `Disposable` class. `_isDisposed` guard at line 17 prevents double-dispose. `_register<T>(value)` at line 28 adds to `_disposables`; if already disposed, immediately disposes the incoming value. `disposeAll` at line 8 pops-and-disposes the array in reverse order.

#### `src/uuid.ts`

**Nonce generation.**

- Line 11–17: Prefers `crypto.randomUUID()` when available.
- Line 21–57: Fallback uses `crypto.getRandomValues` on a 16-byte array. Sets version bits: `_data[6] = (_data[6] & 0x0f) | 0x40` (UUID v4 marker) and `_data[8] = (_data[8] & 0x3f) | 0x80` (RFC 4122 variant bits). Builds the hex string manually. Used exclusively in `simpleBrowserView.ts:116` to generate the per-render CSP nonce.

#### `preview-src/events.ts`

**DOMContentLoaded helper.**

- Line 6–11: `onceDocumentLoaded(f)` checks `document.readyState` for `'loading'` or `'uninitialized'`; in those cases attaches a `DOMContentLoaded` listener. Otherwise invokes `f()` immediately. Called at `index.ts:47` to gate all DOM interaction setup.

---

### Cross-Cutting Synthesis

The simple-browser extension implements a single-panel embedded web browser inside a VS Code WebviewPanel. Activation in `extension.ts` registers three entry points: a user-facing `simpleBrowser.show` command, a programmatic `simpleBrowser.api.open` command, and an external URI opener intercepting `http`/`https` URLs destined for localhost addresses. All three gates check for `workbench.action.browser.open` first and delegate to that integrated browser command if present. The `SimpleBrowserManager` enforces a single active `SimpleBrowserView` at a time, reusing or creating panels and clearing the reference on dispose. `SimpleBrowserView` generates the full WebviewPanel HTML on every `show()` call, embedding the target URL and a `focusLockEnabled` flag in a `data-settings` meta attribute, applying a strict CSP (`default-src 'none'`, `frame-src *`) with a per-render UUID nonce for the bundled script. The iframe uses a `sandbox` attribute allowing scripts, forms, same-origin, and downloads but not navigation or popups. The in-webview script (`preview-src/index.ts`) reads settings from the meta element, wires navigation controls to `iframe.src` mutation and browser `history` APIs, polls focus state every 50 ms to toggle a CSS class, and communicates outbound (`openExternal`) via `vscode.postMessage` and inbound (`focus`, `didChangeFocusLockIndicatorEnabled`) via `window.message`. Panel state is persisted through `vscode.setState({ url })` and restored via the `WebviewPanelSerializer` path through `SimpleBrowserManager.restore`.

For a Tauri port, the VS Code `WebviewPanel` API maps to Tauri's `webview` window or embedded `WebviewWindow`. The two-way messaging channel (`postMessage` / `onDidReceiveMessage`) maps to Tauri's `invoke`/`emit`/`listen` IPC. The CSP and sandbox attributes would need to be replicated in Tauri's webview configuration (`tauri.conf.json` CSP field) since Tauri does not automatically enforce them. The `retainContextWhenHidden: true` semantics would require keeping the webview alive in the background rather than destroying it on hide. The `vscode.setState` / `deserializeWebviewPanel` persistence pattern would need replacement with Tauri's own state-store mechanism or explicit file persistence.

---

### Out-of-Partition References

- `workbench.action.browser.open` command — defined outside this extension, in VS Code core or another extension. Checked at `extension.ts:17` and `39`. Presence of this command gates all three entry points.
- `vscode.window.registerExternalUriOpener` — VS Code API surface consumed at `extension.ts:89`. The opener ID `simpleBrowser.open` is the string by which the platform dispatches to this handler.
- `vscode.window.registerWebviewPanelSerializer` — consumed at `extension.ts:55`. The platform calls `deserializeWebviewPanel` on reload/restart with the persisted state written by `vscode.setState` in `preview-src/index.ts:107`.
- `vscode.workspace.getConfiguration('simpleBrowser')` — reads the `simpleBrowser.focusLockIndicator.enabled` setting defined in the extension's `package.json` contribution point (outside the analysed TypeScript source files).
- `media/index.js` — the compiled output of `preview-src/index.ts`, bundled and served as a local resource from `extensionUri/media`. Referenced at `simpleBrowserView.ts:118` and injected at line 172.
- `media/main.css` and `media/codicon.css` — static assets loaded into the webview at `simpleBrowserView.ts:119–120` and `140–141`. CSS variables and class names such as `iframe-focused` and `enable-focus-lock-indicator` toggled by `preview-src/index.ts:50,112` are defined in `media/main.css`.
- `src/vs/base/common/uuid.ts` — the canonical implementation that `src/uuid.ts:8` explicitly notes it is copied from.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
