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
