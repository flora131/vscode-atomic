# Partition 39: `extensions/simple-browser/` — Webview Panel Serialization

## Overview
The simple-browser extension (10 files, 636 LOC) implements a lightweight web content viewer using VS Code's webview API. This partition focuses on webview panel serialization patterns, which handle restoration of webview state across editor sessions.

## Implementation Files

### Core Extension Logic
- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/src/extension.ts` — Main entry point that registers the webview panel serializer at line 55 using `vscode.window.registerWebviewPanelSerializer()`. Handles command registration, external URI opener setup, and integrated browser fallback logic.

### Manager & View Classes
- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/src/simpleBrowserManager.ts` — Manages the active webview instance lifecycle. The `restore()` method (line 34) receives the webview panel and serialized state, reconstructing the view with the stored URL from `state?.url`.

- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/src/simpleBrowserView.ts` — View class extending a custom `Disposable` class. Contains factory methods `create()` and `restore()` that construct webview panels with proper options (`retainContextWhenHidden: true` for state preservation). Uses `vscode.setState()` to persist URL state on navigation (line 107 in preview/index.ts).

### Utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/src/dispose.ts` — Disposable pattern implementation with registration tracking. Base class for resource cleanup.

- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/src/uuid.ts` — UUID generation for CSP nonce values, uses `crypto.randomUUID()` with fallback to manual Uint8Array construction.

## Webview Preview Code
- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/preview-src/index.ts` — Client-side webview script that:
  - Parses initial settings from data-settings attribute (URL, focus lock config)
  - Posts navigation state via `vscode.setState({ url: rawUrl })` on each page load
  - Handles iframe-based content display with back/forward/reload controls
  - Receives and responds to VS Code API messages (focus, configuration changes)

- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/preview-src/events.ts` — DOM ready helper for webview initialization.

## Configuration & Manifests

### Extension Manifest
- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/package.json` — Declares:
  - Activation event: `onWebviewPanel:simpleBrowser.view` (line 29) for deserialization triggers
  - Enabled API proposals: `externalUriOpener` (line 6)
  - ViewType identifier: `simpleBrowser.view` (referenced throughout)
  - Webview entry points: `./out/extension` (Node) and `./dist/browser/extension` (browser)
  - Configuration schema for `simpleBrowser.focusLockIndicator.enabled`

### TypeScript Configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/tsconfig.json` — Extends base config, targets Node types, includes vscode.d.ts and proposed externalUriOpener definitions.

- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/tsconfig.browser.json` — Browser-specific compilation (imported during esbuild).

### Build Scripts
- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/esbuild.webview.mts` — Bundles preview-src for iframe content.
- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/esbuild.browser.mts` — Bundles extension for web context.

## Assets & Styling
- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/media/` — Contains:
  - `main.css` — Webview styling (navigation bar, iframe container, focus lock indicator)
  - `icon.png`, `preview-light.svg`, `preview-dark.svg` — Extension icons
  - `codicon.css` — Icon font imported at runtime

## Serialization Pattern Details

**Key Architecture:**
1. **Registration** (extension.ts:55): Serializer installed via `registerWebviewPanelSerializer(viewType, deserializer)` during activation
2. **State Object**: Passed as second parameter to `deserializeWebviewPanel` contains `{ url, focusLockEnabled? }`
3. **Restoration Flow**: `deserializeWebviewPanel` → `manager.restore()` → `SimpleBrowserView.restore()` → new instance with same webview panel
4. **Client State Persistence**: Webview script calls `vscode.setState()` with current URL whenever navigation occurs

**Implementation Dependencies:**
- `vscode.window.createWebviewPanel()` — Creates new panels with options like `retainContextWhenHidden`
- `vscode.WebviewPanel.onDidDispose` — Cleanup triggers
- `vscode.workspace.onDidChangeConfiguration` — Configuration propagation to webview
- `vscode.env.openExternal()` — External link handling

## Documentation
- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/README.md` — Minimal readme noting it's a bundled extension providing basic iframe-based webview content display.

---

## Tauri/Rust Port Considerations

This partition reveals several webview serialization abstractions that would require Rust equivalents in a Tauri port:

1. **Serializer Registration Model**: VS Code's `registerWebviewPanelSerializer()` API provides a declarative hook for deserialization. Tauri's webview model (Windows/WKWebView/WebKitGTK) lacks this built-in, requiring manual state management via storage layer (JSON files, SQLite, etc.).

2. **State Object Structure**: The simple pattern of `{ url, focusLockEnabled }` is easily serializable, but VS Code's hidden complexity lies in retaining webview context across restarts. Tauri requires explicit serialization of HTML/CSS and JavaScript execution context state.

3. **Activation Events**: The `onWebviewPanel:simpleBrowser.view` activation trigger implies session restoration can spawn extension processes. Tauri would need an explicit restore handler bound to app lifecycle hooks (e.g., `CombinedWindowsManager::restore()`).

4. **View Type Identification**: The string-based `viewType` identifier drives the serializer mapping. A Rust implementation would use enums or string-keyed registries with type-safe deserialization.

5. **Client-Side State API**: The webview's `vscode.setState()/getState()` API persists state within the webview context. Tauri's Invoke API (`tauri::invoke()`) or persistent storage would replace this bidirectional communication.

6. **Lifecycle Coordination**: The manager pattern (SimpleBrowserManager) orchestrates single-instance lifecycle. Rust would benefit from interior mutability (Arc/Mutex) for thread-safe restoration across async operations.

7. **Message Passing Architecture**: The webview uses `postMessage`/`onDidReceiveMessage` for configuration changes. Tauri's event system or async command invocation would need equivalent guarantees for ordering and delivery.

8. **CSP & Resource Loading**: Dynamically generated CSP headers and local resource roots need runtime enforcement. Tauri's webview configuration would need custom middleware or protocol handlers to inject CSP during page load.

