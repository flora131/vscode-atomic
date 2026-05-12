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
