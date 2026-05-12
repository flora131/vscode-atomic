### Files Analysed

1. `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/src/extension.ts`
2. `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/src/chatOutputRenderer.ts`
3. `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/src/webviewManager.ts`
4. `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/src/editorManager.ts`
5. `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/chat-webview-src/mermaidWebview.ts`
6. `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/chat-webview-src/index.ts`
7. `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/chat-webview-src/index-editor.ts`
8. `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/chat-webview-src/vscodeApi.ts`
9. `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/src/util/dispose.ts`
10. `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/package.json`

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/src/extension.ts`

- **Role:** Extension entry point. Instantiates all subsystem managers, registers commands, wires them together, and launches the chat/LM support.
- **Key symbols:**
  - `activate` (line 11) — sole exported function; called by VS Code on extension activation.
  - `MermaidWebviewManager` (line 12) — shared singleton tracking all live webviews.
  - `MermaidEditorManager` (line 14) — handles serialization/deserialization of editor panels; pushed to `context.subscriptions`.
  - `registerChatSupport` (line 18) — sets up LM tool + chat output renderer.
  - `_mermaid-chat.resetPanZoom` command (line 22) — delegates to `webviewManager.resetPanZoom`.
  - `_mermaid-chat.copySource` command (line 28) — reads `mermaidSource` from `MermaidWebviewInfo` and writes to clipboard via `vscode.env.clipboard.writeText`.
- **Control flow:** `activate` → creates `MermaidWebviewManager` (no arguments) → creates `MermaidEditorManager(context.extensionUri, webviewManager)` → calls `registerChatSupport(context, webviewManager, editorManager)` → registers two commands with the `ctx?.mermaidWebviewId` routing pattern.
- **Data flow:** The `mermaidWebviewId` string passes from the HTML `data-vscode-context` attribute (rendered in `chatOutputRenderer.ts` and `editorManager.ts`) through VS Code's context menu infrastructure into the command handler's `ctx` argument, which is used to look up the correct `MermaidWebviewInfo` in `webviewManager`.
- **Dependencies:** `vscode` API, `./chatOutputRenderer`, `./editorManager`, `./webviewManager`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/src/chatOutputRenderer.ts`

- **Role:** Implements the MIME-typed chat output renderer for `text/vnd.mermaid` data items and the language model tool `renderMermaidDiagram`. Acts as the bridge between the AI model output and the webview rendering layer.
- **Key symbols:**
  - `mime = 'text/vnd.mermaid'` (line 15) — MIME type that routes chat output to this renderer.
  - `viewType = 'vscode.chat-mermaid-features.chatOutputItem'` (line 20) — view type identifier for the registered renderer.
  - `MermaidChatOutputRenderer` (line 22) — implements `vscode.ChatOutputRenderer`; the single method is `renderChatOutput`.
  - `renderChatOutput` (line 29) — decodes the `value: Uint8Array` payload, generates a UUID webview ID, registers with `_webviewManager`, sets `webview.options`, builds the inline HTML with nonce-guarded CSP, injects `index.js` as a module script.
  - `registerChatSupport` (line 129) — top-level factory; registers `_mermaid-chat.openInEditor`, the LM tool, and calls `vscode.chat.registerChatOutputRenderer(viewType, renderer)` (line 160).
  - `writeMermaidToolOutput` (line 165) — constructs a `vscode.LanguageModelToolResult` combining a markdown mermaid code fence (line 169) and a `toolResultDetails2` blob (proposed API, line 176) with MIME `text/vnd.mermaid` and JSON-encoded `{source, title}`.
  - `decodeMermaidData` (line 199) — decodes `Uint8Array` → UTF-8 string → tries JSON parse for `{source, title}`, falls back to treating the entire string as plain source (legacy format).
  - `getFenceForContent` (line 184) — scans content for longest backtick run, returns a fence at least 3 characters longer.
- **Control flow:**
  1. LM invokes `renderMermaidDiagram` tool → `writeMermaidToolOutput` encodes diagram as `LanguageModelToolResult` with both a markdown text part and a `toolResultDetails2` MIME blob.
  2. Chat infrastructure passes the MIME blob to `renderChatOutput` (matching on `text/vnd.mermaid`).
  3. `renderChatOutput` decodes, generates `webviewId`, calls `_webviewManager.registerWebview`, sets CSP/HTML, attaches `onDidReceiveMessage` for `openInEditor`, registers `chatOutputWebview.onDidDispose` cleanup.
- **Data flow:** `Uint8Array` (JSON `{source, title}`) → `decodeMermaidData` → `MermaidData` struct → `escapeHtmlText(mermaidSource)` injected into `<pre class="mermaid">` in HTML string → mermaid.js processes in webview.
- **Dependencies:** `vscode`, `./editorManager`, `./webviewManager`, `./util/html`, `./util/uuid`, `./util/dispose`. Uses proposed API `vscode.ExtendedLanguageModelToolResult2` for `toolResultDetails2`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/src/webviewManager.ts`

- **Role:** Central registry and message router for all live mermaid webviews (both chat output and editor panel types). Tracks which webview is currently active.
- **Key symbols:**
  - `MermaidWebviewInfo` interface (line 7) — readonly fields: `id: string`, `webview: vscode.Webview`, `mermaidSource: string`, `title: string | undefined`, `type: 'chat' | 'editor'`.
  - `MermaidWebviewManager` class (line 19):
    - `_activeWebviewId: string | undefined` (line 21) — tracks the currently focused webview ID.
    - `_webviews: Map<string, MermaidWebviewInfo>` (line 22) — keyed by the UUID/hash webview ID.
    - `registerWebview` (line 31) — adds an entry and returns a `Disposable` whose `dispose` calls `unregisterWebview`.
    - `unregisterWebview` (line 47) — deletes from map; nulls `_activeWebviewId` if that was active.
    - `setActiveWebview` (line 56) — sets `_activeWebviewId` only if the id exists in the map.
    - `activeWebview` getter (line 27) — returns the `MermaidWebviewInfo` for the currently active id.
    - `resetPanZoom` (line 69) — calls `target?.webview.postMessage({ type: 'resetPanZoom' })`.
- **Control flow:** Registration happens when a chat renderer or editor panel is created. Commands look up the target webview by ID or fall back to `activeWebview`. Disposal cascades through the returned `Disposable`.
- **Data flow:** `mermaidSource` is stored in the registry and exposed through `MermaidWebviewInfo` for clipboard copy and "open in editor" flows. `postMessage` carries `{type: 'resetPanZoom'}` to the webview JS layer.
- **Dependencies:** `vscode` (for `vscode.Webview`, `vscode.Disposable`).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/src/editorManager.ts`

- **Role:** Manages `WebviewPanel`-based full editor previews. Implements `vscode.WebviewPanelSerializer` to restore panels across restarts. Deduplicates panels by content hash so the same diagram is never opened twice.
- **Key symbols:**
  - `mermaidEditorViewType = 'vscode.chat-mermaid-features.preview'` (line 11) — view type registered with `vscode.window.registerWebviewPanelSerializer`.
  - `MermaidPreviewState` interface (line 13) — `{webviewId: string, mermaidSource: string}` — persisted panel state.
  - `MermaidEditorManager` (line 21) — extends `Disposable`, implements `vscode.WebviewPanelSerializer`.
    - `_previews: Map<string, MermaidPreview>` (line 23) — keyed by content-hash `diagramId`.
    - `openPreview` (line 39) — computes `getWebviewId(mermaidSource)`, checks map for existing preview (calls `.reveal()` if found), otherwise calls `MermaidPreview.create`.
    - `deserializeWebviewPanel` (line 58) — called by VS Code on reload with saved state; reconstructs `MermaidPreview` via `MermaidPreview.revive`.
    - `dispose` (line 112) — disposes all `_previews`.
  - `MermaidPreview` (line 122) — extends `Disposable`:
    - `create` static (line 127) — calls `vscode.window.createWebviewPanel` with `retainContextWhenHidden: false`, then `new MermaidPreview(...)`.
    - `revive` static (line 147) — wraps an existing panel provided by the serializer.
    - Constructor (line 157) — sets `localResourceRoots` to `chat-webview-out/`, renders HTML via `_getHtml()`, registers with `_webviewManager`, wires `onDidChangeViewState` to call `webviewManager.setActiveWebview`.
    - `_getHtml` (line 204) — builds HTML loading `index-editor.js` (not `index.js`) with nonce CSP; includes `.zoom-controls` buttons DOM.
  - `getWebviewId` (line 290) — djb2-style 32-bit hash of source string, returned as hex; provides content-addressable deduplication.
- **Control flow:** `openPreview` → hash source → check `_previews` → if miss: `MermaidPreview.create` → `_registerPreview` → `preview.onDispose` → delete from map. Panel focus change fires `setActiveWebview`.
- **Data flow:** `mermaidSource` is hashed to a stable `diagramId`, stored in both `_previews` and `_webviewManager`. Panel state `{webviewId, mermaidSource}` is saved by VS Code's webview persistence layer (via `vscode.WebviewPanelSerializer`) and restored as `MermaidPreviewState`.
- **Dependencies:** `vscode`, `./util/uuid`, `./webviewManager`, `./util/html`, `./util/dispose`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/chat-webview-src/mermaidWebview.ts`

- **Role:** Core webview runtime: initializes the `mermaid` library, wraps the diagram DOM element for pan/zoom, persists state via `vscodeApi`, and handles re-rendering on VS Code theme changes. Shared between chat and editor entry points.
- **Key symbols:**
  - `PanZoomHandler` class (line 14):
    - State: `scale`, `translateX`, `translateY`, `isPanning`, `hasDragged`, `hasInteracted` (lines 16–21).
    - `applyTransform` (line 198) — writes CSS `transform: translate(...)px scale(...)` directly to `content.style.transform`.
    - `saveState` (line 202) — merges `{panZoom: {scale, translateX, translateY}}` into `vscode.getState()` / `vscode.setState()`.
    - `restoreState` (line 215) — reads `state.panZoom` from `vscode.getState()`; returns `true` if state was found and applied.
    - `centerContent` (line 238) — queries the rendered `<svg>` element's `getBoundingClientRect()` to compute centering offsets.
    - `handleWheel` (line 120) — distinguishes pinch (`e.ctrlKey`) from Alt+scroll; zoom-toward-pointer math at lines 146–149.
    - `zoomAtPoint` (line 283) — clamps to `[minScale=0.1, maxScale=5]`, then adjusts translate to zoom toward a point.
    - `reset` (line 255) — resets all transform state, deletes `panZoom` key from persisted state, re-centers via `requestAnimationFrame`.
  - `getMermaidTheme` (line 294) — reads `document.body.classList` for `vscode-dark` / `vscode-high-contrast` to return `'dark'` or `'default'`.
  - `initializeMermaidWebview` (line 332) — async export; the entire runtime bootstrap:
    1. Queries `.mermaid` element (line 333).
    2. Reads `diagramText` from `diagram.textContent` (line 341).
    3. Persists `mermaidSource` to `vscode.setState` (lines 347–351).
    4. Creates `wrapper`+`content` DOM structure and relocates `diagram` inside it (lines 354–364).
    5. Calls `mermaid.initialize({startOnLoad: false, theme})` then `mermaid.run({nodes: [diagram]})` (lines 367–372).
    6. Adds `rendered` class (line 375) to make diagram visible.
    7. Constructs `PanZoomHandler(wrapper, content, vscode)` and calls `.initialize()` (lines 377–378).
    8. Adds `window.addEventListener('message', ...)` for `resetPanZoom` (lines 381–386).
    9. Installs a `MutationObserver` on `document.body` watching `class` attribute changes to re-render on theme change (lines 389–406).
  - `rerenderMermaidDiagram` (line 316) — resets `textContent`, deletes `dataset.processed`, re-initializes and re-runs mermaid with new theme.
  - `LocalState` interface (line 303) — `{mermaidSource, theme}` (unpersisted, only in JS closure).
  - `PersistedState` interface (line 308) — `{mermaidSource, panZoom?}` stored via `vscode.setState`.
- **Control flow:** `initializeMermaidWebview` → DOM manipulation → mermaid render → `PanZoomHandler.initialize` (tries restore else centers) → event listeners. Theme change → `MutationObserver` callback → `rerenderMermaidDiagram`.
- **Data flow:** Diagram text flows from `<pre class="mermaid">` textContent → mermaid.js → SVG inside the pre element. Pan/zoom state cycles through `vscode.getState()`/`vscode.setState()` for persistence across webview hide/show. `resetPanZoom` message arrives from `MermaidWebviewManager.resetPanZoom` → `postMessage` → `window.message` event → `panZoomHandler.reset()`.
- **Dependencies:** `mermaid` (npm), `./vscodeApi`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/chat-webview-src/index.ts`

- **Role:** Entry point for chat output renderer webviews (`chat-webview-out/index.js`). Acquires the VS Code API, runs the shared mermaid initializer, and wires the "Open in Editor" button.
- **Key symbols:**
  - `acquireVsCodeApi()` (line 8) — global function injected by VS Code webview host.
  - `main` (line 12) — `await initializeMermaidWebview(vscode)` → attaches click listener on `.open-in-editor-btn` that calls `vscode.postMessage({ type: 'openInEditor' })` (line 18).
- **Control flow:** Module loads → `main()` called synchronously → initializes webview → button listener active.
- **Data flow:** Button click → `postMessage('openInEditor')` → received by `chatOutputRenderer.ts` `onDidReceiveMessage` handler (line 44) → `vscode.commands.executeCommand('_mermaid-chat.openInEditor', {mermaidWebviewId})`.
- **Dependencies:** `./mermaidWebview`, `./vscodeApi`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/chat-webview-src/index-editor.ts`

- **Role:** Entry point for full editor panel webviews (`chat-webview-out/index-editor.js`). Acquires VS Code API, runs the shared initializer, then wires the DOM zoom control buttons to `PanZoomHandler` methods.
- **Key symbols:**
  - `panZoomHandler` (line 12) — return value of `initializeMermaidWebview`; used to attach `.zoomIn()`, `.zoomOut()`, `.reset()` to button click events (lines 22–24).
- **Control flow:** Module loads → `initializeMermaidWebview(vscode).then(...)` → queries `.zoom-in-btn`, `.zoom-out-btn`, `.zoom-reset-btn` → adds `addEventListener('click', ...)` for each.
- **Data flow:** Button DOM events → `PanZoomHandler` instance methods → CSS transform updates + `vscode.setState` persistence.
- **Dependencies:** `./mermaidWebview`, `./vscodeApi`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/chat-webview-src/vscodeApi.ts`

- **Role:** TypeScript interface declaration only. Defines the contract for the `acquireVsCodeApi()` return object.
- **Key symbols:**
  - `VsCodeApi` interface (line 6): `getState(): any`, `setState(state: any): void`, `postMessage(message: any): void`.
- **Dependencies:** None.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/src/util/dispose.ts`

- **Role:** Shared disposal utilities used by `MermaidEditorManager` and `MermaidPreview`.
- **Key symbols:**
  - `disposeAll(disposables: vscode.Disposable[])` (line 8) — pops and disposes all items from an array.
  - `Disposable` abstract class (line 15) — `_isDisposed: boolean`, `_disposables: vscode.Disposable[]`. `_register<T>(value: T): T` (line 28) — adds to array or immediately disposes if already disposed. `dispose()` (line 20) — guards re-entrancy via `_isDisposed`, calls `disposeAll`.
- **Dependencies:** `vscode`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/package.json`

- **Role:** Extension manifest. Declares contribution points, API proposals, and dependency versions.
- **Key symbols:**
  - `enabledApiProposals: ["chatOutputRenderer"]` (line 16) — the proposed API this extension depends on for `vscode.chat.registerChatOutputRenderer` and `vscode.ChatOutputRenderer`.
  - `activationEvents: ["onWebviewPanel:vscode.chat-mermaid-features.preview"]` (line 27) — triggers extension activation when a serialized editor panel is restored.
  - `chatOutputRenderers` contribution (line 82) — binds `viewType: "vscode.chat-mermaid-features.chatOutputItem"` to `mimeTypes: ["text/vnd.mermaid"]`.
  - `languageModelTools` contribution (line 90) — declares `renderMermaidDiagram` tool with JSON input schema for `markup` (string) and `title` (string); guarded by `when: "config.mermaid-chat.enabled"`.
  - `dependencies: {dompurify, mermaid: "^11.12.3"}` (line 133) — runtime npm packages bundled into the webview script.

---

### Cross-Cutting Synthesis

The `mermaid-chat-features` extension implements a two-stage rendering pipeline connecting VS Code's language model and chat systems to HTML webviews. At the host side, `chatOutputRenderer.ts` registers a language model tool (`renderMermaidDiagram`) that encodes diagram markup as a `LanguageModelToolResult` carrying both a markdown fence (for the chat transcript) and a `toolResultDetails2` MIME blob (`text/vnd.mermaid`, JSON `{source, title}`) for the custom renderer. The registered `MermaidChatOutputRenderer` receives the MIME blob, decodes it, and injects the escaped source into a nonce-protected HTML shell that loads `index.js` (chat) or `index-editor.js` (editor panel) as module scripts. Both entry points call the shared `initializeMermaidWebview` function, which queries `.mermaid` DOM text, runs `mermaid.js` render, wraps the result in a `PanZoomHandler` for CSS-transform-based pan/zoom, and persists state through `vscode.setState`/`vscode.getState`. The `MermaidWebviewManager` provides a shared registry that decouples the webview identity (UUID for chat, content-hash for editor) from the command routing layer, enabling clipboard copy and "open in editor" to resolve the correct source regardless of which webview is active. The `MermaidEditorManager` further implements `vscode.WebviewPanelSerializer` so editor panels survive window reloads via the `onWebviewPanel` activation event. Porting this to Tauri/Rust requires replacing: `vscode.chat.registerChatOutputRenderer` and `vscode.lm.registerTool` (proposed chat/LM APIs), `vscode.window.createWebviewPanel`/`WebviewPanelSerializer` (VS Code webview lifecycle), `vscode.env.clipboard`, `vscode.commands`, `webview.postMessage`/`onDidReceiveMessage`, `acquireVsCodeApi` in the browser context, and `vscode.ChatOutputWebview.onDidDispose` — all of which are deep VS Code host APIs with no direct Tauri equivalents and must be re-implemented as Tauri commands, IPC channels, and Tauri webview windows.

---

### Out-of-Partition References

- `vscode.ChatOutputRenderer` / `vscode.chat.registerChatOutputRenderer` — proposed API defined in VS Code core (`src/vscode-dts/vscode.proposed.chatOutputRenderer.d.ts` or similar); not in this partition.
- `vscode.lm.registerTool` / `vscode.LanguageModelToolResult` / `vscode.LanguageModelTextPart` — language model API defined in VS Code core extension host.
- `vscode.ExtendedLanguageModelToolResult2` (cast at `chatOutputRenderer.ts:176`) — another proposed API type, defined outside this extension.
- `acquireVsCodeApi()` — global function injected by the VS Code webview host (`src/vs/workbench/browser/parts/editor/webviewEditor.ts` or the webview preload layer).
- `mermaid` npm package (`^11.12.3`) and `dompurify` (`^3.4.1`) — third-party runtime libraries bundled by `esbuild.webview.mts`.
- `@vscode/codicons` — icon font referenced as `codicon.css` in both HTML templates; built output at `chat-webview-out/codicon.css`.
- Build pipeline: `esbuild.webview.mts` (chat webview bundler) and `esbuild.browser.mts` (browser extension bundler) — build scripts in the extension root not included in the analysed files.
