# Online Research: extensions/simple-browser/ (Partition 39)

**Research question:** Port VS Code core IDE from TypeScript/Electron to Tauri/Rust — `extensions/simple-browser/` (10 files, 636 LOC).

---

## Verdict

(no external research applicable)

**Justification:** The extension is entirely self-contained within the VS Code WebView API surface — it opens a sandboxed `<iframe>`, posts messages via `acquireVsCodeApi()`, applies a tight Content Security Policy, and loads only the bundled Codicon font from a `data:` URI — none of which requires consulting Tauri `WebviewWindow` or iframe-sandboxing docs because the iframe lives inside the VS Code webview host and that host boundary does not change when the outer shell moves from Electron to Tauri.

---

## Supporting Analysis

### What the extension actually does

`extensions/simple-browser/` is a minimal in-editor browser tab. Its sole mechanism is:

1. **`SimpleBrowserView.ts`** — creates a `vscode.WebviewPanel` with `enableScripts: true`, `enableForms: true`, `retainContextWhenHidden: true`, and restricted `localResourceRoots` pointing only to `media/`. It injects HTML whose CSP reads:

   ```
   default-src 'none';
   font-src data:;
   style-src <webview-csp-source>;
   script-src 'nonce-<uuid>';
   frame-src *;
   ```

   The content area is a single `<iframe sandbox="allow-scripts allow-forms allow-same-origin allow-downloads">`.

2. **`preview-src/index.ts`** — the webview-side script. It calls `acquireVsCodeApi()`, listens for `window.message` events from the host, and drives iframe navigation by setting `iframe.src`. Navigation state is persisted with `vscode.setState()`.

3. **`SimpleBrowserManager.ts`** — thin lifecycle manager; keeps at most one active view, delegates to `SimpleBrowserView.restore()` for panel serialization.

4. **`extension.ts`** — registers three commands (`simpleBrowser.show`, `simpleBrowser.api.open`, `workbench.action.browser.open`) and a `vscode.window.registerExternalUriOpener` for `http`/`https` schemes limited to localhost addresses.

### Why no external library docs are needed

- **All WebView API calls** (`createWebviewPanel`, `postMessage`, `onDidReceiveMessage`, `asWebviewUri`, `cspSource`, `registerWebviewPanelSerializer`, `registerExternalUriOpener`) are VS Code extension API, not Tauri API. When the outer shell migrates to Tauri, these calls are handled by whatever VS Code WebView abstraction layer the port provides — the simple-browser code itself is unchanged.

- **The `<iframe>` sandbox** is a standard HTML attribute whose semantics are defined by the browser engine (Chromium / WebKit), not by Tauri or Electron. Tauri's `WebviewWindow` wraps the same Chromium/WebKit iframe engine; the sandbox flags (`allow-scripts allow-forms allow-same-origin allow-downloads`) behave identically.

- **Codicon font** is loaded via a `data:` URI (`font-src data:`), so no external network fetch or Tauri asset-protocol handling is required.

- **No native IPC beyond VS Code postMessage.** The extension never calls `window.__TAURI__` or any Electron remote module; all host communication is through the VS Code WebView message channel, which the VS Code host layer owns.

- **No node.js-specific code.** The `isWeb()` guard in `extension.ts` checks `process.versions.node` as a boolean — that code path does not interact with any Tauri primitive.

### The one migration note (not requiring external research)

The CSP directive `style-src ${this._webviewPanel.webview.cspSource}` depends on the VS Code host setting the correct `cspSource` origin for the webview. In a Tauri-hosted VS Code, that origin will be whatever scheme the Tauri webview layer exposes (e.g. `tauri://` or `vscode-webview://`). This is a concern for the VS Code WebView host layer, not for this extension, and it is documented in the VS Code API itself — no Tauri-specific research is warranted here.

---

## Summary

`extensions/simple-browser/` is a pure consumer of the VS Code WebView API. Its entire complexity sits inside a sandboxed `<iframe>` navigated by plain DOM manipulation. There is no direct dependency on Electron APIs, Tauri APIs, Node.js native modules, or any external library beyond VS Code itself and the bundled Codicon CSS font (served as a `data:` URI). The iframe sandbox flags and Content Security Policy are standard HTML constructs whose semantics do not change between Electron and Tauri backends. Consulting Tauri `WebviewWindow` or iframe-sandboxing documentation would not yield actionable porting guidance for this partition; the porting work is entirely about ensuring the VS Code WebView host abstraction layer (which lives elsewhere in the codebase) correctly initialises `cspSource` and `asWebviewUri` under Tauri. No external research is applicable to this partition.
