# Analyzer-14: extensions/markdown-language-features/

## Research Question
Port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

---

### Files Analysed

| # | File | LOC (approx) |
|---|------|-------------|
| 1 | `extensions/markdown-language-features/src/extension.ts` | 56 |
| 2 | `extensions/markdown-language-features/src/extension.shared.ts` | 68 |
| 3 | `extensions/markdown-language-features/src/preview/preview.ts` | 858 |
| 4 | `extensions/markdown-language-features/src/preview/documentRenderer.ts` | 263 |
| 5 | `extensions/markdown-language-features/src/preview/previewManager.ts` | 331 |
| 6 | `extensions/markdown-language-features/src/client/client.ts` | 180 |
| 7 | `extensions/markdown-language-features/src/client/protocol.ts` | 41 |
| 8 | `extensions/markdown-language-features/src/languageFeatures/diagnostics.ts` | 134 |
| 9 | `extensions/markdown-language-features/preview-src/index.ts` | 464 |
| 10 | `extensions/markdown-language-features/preview-src/messaging.ts` | 33 |
| 11 | `extensions/markdown-language-features/types/previewMessaging.d.ts` | 88 |
| 12 | `extensions/markdown-language-features/src/markdownEngine.ts` | 455 |

---

### Per-File Notes

---

#### 1. `src/extension.ts` — Node.js Entry Point & Activation

**Role:** The main extension activation function for the Node.js (non-browser) host. It bootstraps the language server child process via IPC and then delegates all shared registration to `extension.shared.ts`.

**Key symbols:**

- `activate(context)` — `extension.ts:15`. Async export. Creates `MarkdownItEngine`, calls `startServer`, then calls `activateShared`.
- `startServer(context, parser)` — `extension.ts:29`. Selects the server module path based on `context.extension.packageJSON.main`: debug builds use the `out/` variant; production builds use `dist/serverWorkerMain`. Sets `process.env['VSCODE_L10N_BUNDLE_LOCATION']` to pass the l10n bundle path. Constructs a `ServerOptions` object using `TransportKind.ipc` (both run and debug variants). Invokes `startClient()` from `./client/client.ts` passing a factory that calls `new LanguageClient(...)`.

**Control flow:**
```
activate()
  → getMarkdownExtensionContributions()
  → new MarkdownItEngine(contributions, githubSlugifier, logger)
  → startServer() → startClient() [spawns IPC worker]
  → activateShared(context, client, engine, logger, contributions)
```

**Dependencies on VS Code APIs:** `vscode.ExtensionContext`, `vscode.l10n`, `vscode-languageclient/node.LanguageClient`, `TransportKind`.

**Porting notes:** `TransportKind.ipc` relies on Node.js `child_process` IPC. The debug port selection (`7000 + Math.random() * 999`) at `extension.ts:41` is Node-specific. The `VSCODE_L10N_BUNDLE_LOCATION` env variable injection at `extension.ts:51` is a process-global side-effect.

---

#### 2. `src/extension.shared.ts` — Shared Activation (Node + Browser)

**Role:** Handles all feature registration that is platform-agnostic between the Node.js and browser builds. Instantiates `MdDocumentRenderer`, `MarkdownPreviewManager`, link opener, CSP arbiter, telemetry, and command manager. Registers all language feature providers.

**Key symbols:**

- `activateShared(context, client, engine, logger, contributions)` — `extension.shared.ts:26`. Called from both `extension.ts` (Node) and `extension.browser.ts` (web). Instantiates all services and registers them as disposables.
- `registerMarkdownLanguageFeatures(client, commandManager, parser)` — `extension.shared.ts:53`. Returns a composite `vscode.Disposable` assembling six sub-registrations:
  - `registerDiagnosticSupport` — code actions and status bar for link validation
  - `registerFindFileReferenceSupport` — workspace-level file reference search
  - `registerResourceDropOrPasteSupport` — drag/drop and paste of resources
  - `registerPasteUrlSupport` — URL paste provider
  - `registerUpdateLinksOnRename` — link fixup on file rename
  - `registerUpdatePastedLinks` — link update on paste

**Data flow:**
- `ExtensionContentSecurityPolicyArbiter` wraps `context.globalState` and `context.workspaceState` to persist per-resource CSP decisions.
- `MdDocumentRenderer` is created with `engine`, `cspArbiter`, and `contributions`.
- `MarkdownPreviewManager` receives the renderer and manages panel lifecycles.
- A `vscode.workspace.onDidChangeConfiguration` handler at `extension.shared.ts:48` calls `previewManager.updateConfiguration()`.

---

#### 3. `src/preview/preview.ts` — Core Preview Panel Logic

**Role:** Defines three classes — `MarkdownPreview` (private inner class managing a single `vscode.WebviewPanel`), `StaticMarkdownPreview` (custom text editor mode), and `DynamicMarkdownPreview` (side-by-side preview following the active editor). This file is the densest single unit of VS Code API surface in the extension.

**Key symbols:**

- `MarkdownPreview` (class) — `preview.ts:43`. Holds `#webviewPanel: vscode.WebviewPanel` at `preview.ts:53`. All communication to/from the webview passes through this panel.
  - `refresh(forceUpdate)` — `preview.ts:208`. Debounces updates using `#throttleTimer` with a 300 ms delay; the very first call bypasses the timer.
  - `#updatePreview(forceUpdate)` — `preview.ts:255`. Opens the document with `vscode.workspace.openTextDocument`, compares `PreviewDocumentVersion`, calls either `renderDocument` (full HTML reload) or `renderBody` (content update only), then updates `#webviewPanel.webview.html` or sends an `updateContent` postMessage.
  - `#onDidScrollPreview(line)` — `preview.ts:308`. Receives `revealLine` messages from the webview, fires `#onScrollEmitter`, and conditionally calls `scrollEditorToLine`.
  - `#onDidClickPreview(line)` — `preview.ts:332`. Handles `didClick` from webview, executes `markdown.showSource` command, sets editor selection.
  - `#onDidClickPreviewLink(href)` — `preview.ts:432`. Resolves links via `MdLinkOpener.resolveDocumentLink`; if in-preview mode and the target is a markdown file, delegates to the preview delegate's `openPreviewLinkToMarkdownFile`.
  - `postMessage(msg)` — `preview.ts:226`. Thin guard around `#webviewPanel.webview.postMessage`.
  - `#getWebviewOptions()` — `preview.ts:408`. Sets `enableScripts: true`, `enableForms: false`, and restricts `localResourceRoots` to workspace folders and extension contribution roots.

- `PreviewDocumentVersion` — `preview.ts:21`. Wraps `vscode.Uri` and `document.version` for equality testing to skip redundant refreshes.

- `StaticMarkdownPreview` — `preview.ts:495`. Registered as a custom text editor via `vscode.window.registerCustomEditorProvider` under view type `"vscode.markdown.preview.editor"`. Revived via `revive()` at `preview.ts:499`. Delegates scroll tracking to `TopmostLineMonitor`.

- `DynamicMarkdownPreview` — `preview.ts:619`. Registered as a webview panel serializer under view type `"markdown.preview"`. Created via `DynamicMarkdownPreview.create()` at `preview.ts:645` which calls `vscode.window.createWebviewPanel`. Tracks `#locked` state for pinning. Responds to `vscode.window.onDidChangeActiveTextEditor` at `preview.ts:723` to switch displayed resource when the active editor changes (if not locked).
  - `update(newResource, scrollLocation)` — `preview.ts:782`. Disposes the inner `MarkdownPreview` and creates a new one for the new resource.
  - `toggleLock()` — `preview.ts:802`. Flips `#locked`, updates panel title.

**Webview message dispatch (inbound):** `MarkdownPreview` registers `#webviewPanel.webview.onDidReceiveMessage` at `preview.ts:141`. Dispatched on `e.type`:
- `cacheImageSizes` → stores image dimension metadata
- `revealLine` → `#onDidScrollPreview`
- `didClick` → `#onDidClickPreview`
- `openLink` → `#onDidClickPreviewLink`
- `showPreviewSecuritySelector` → executes command
- `previewStyleLoadError` → `vscode.window.showWarningMessage`

**File watcher management:** `#updateImageWatchers(srcs)` at `preview.ts:385` maintains a `Map<string, vscode.FileSystemWatcher>` keyed by image `src` URLs. Watchers are created for local (non-http/https/data) image sources and trigger `refresh(true)` on change.

---

#### 4. `src/preview/documentRenderer.ts` — Markdown-to-HTML Rendering

**Role:** `MdDocumentRenderer` is the bridge between the `MarkdownItEngine` and the webview. It produces complete HTML page strings (for full reloads) and partial body HTML (for incremental updates).

**Key symbols:**

- `MdDocumentRenderer` (class) — `documentRenderer.ts:43`. Constructor at `documentRenderer.ts:51` accepts engine, context, cspArbiter, contributionProvider, logger. Sets `iconPath` pointing to `media/preview-dark.svg` and `media/preview-light.svg`.
- `renderDocument(markdownDocument, resourceProvider, previewConfigurations, initialLine, selectedLine, state, imageInfo, token)` — `documentRenderer.ts:71`. Produces the full `<!DOCTYPE html>` page. Steps:
  1. Loads config via `previewConfigurations.loadAndCacheConfiguration(sourceUri)` at `documentRenderer.ts:82`.
  2. Builds `initialData` object at `documentRenderer.ts:83` containing source URI, fragment, scroll settings, CSP warning flag, webview resource root — serialized into `data-settings` meta attribute.
  3. Generates a nonce via `generateUuid()` at `documentRenderer.ts:98`.
  4. Computes CSP string via `#getCsp()` at `documentRenderer.ts:99`.
  5. Calls `renderBody()` to get body HTML and containing-image set.
  6. Injects `pre.js` script, user/contribution stylesheets, and contribution scripts into the `<head>` and `<body>`.
  7. Embeds the rendered body HTML into `data-initial-md-content` meta attribute — the webview reads this attribute on load rather than from the DOM directly, avoiding a flash.
- `renderBody(markdownDocument, resourceProvider)` — `documentRenderer.ts:130`. Calls `engine.render(markdownDocument, resourceProvider)`, wraps in `<div class="markdown-body" dir="auto">`, returns `{html, containingImages}`.
- `renderFileNotFoundDocument(resource)` — `documentRenderer.ts:142`. Returns a minimal HTML body for 404-like cases.
- `#getCsp(provider, resource, nonce)` — `documentRenderer.ts:241`. Branches on `MarkdownPreviewSecurityLevel` enum values: `Strict`, `AllowInsecureContent`, `AllowInsecureLocalContent`, `AllowScriptsAndAllContent`. `AllowScriptsAndAllContent` returns an empty CSP string (no restrictions). Others return explicit `default-src 'none'` policies.
- `#fixHref(resourceProvider, resource, href)` — `documentRenderer.ts:159`. Resolves stylesheet hrefs to webview URIs using `resourceProvider.asWebviewUri`. Handles absolute, workspace-relative, and document-relative paths.
- `#getSettingsOverrideStyles(config)` — `documentRenderer.ts:194`. Emits CSS custom property overrides as inline `style` on `<html>`: `--markdown-font-family`, `--markdown-font-size`, `--markdown-line-height`.

**Porting:** The entire output of this class is platform-standard HTML. The only VS Code API coupling is `vscode.Uri.joinPath` for path resolution and `WebviewResourceProvider.asWebviewUri` for URI-to-webview-URL conversion.

---

#### 5. `src/preview/previewManager.ts` — Preview Lifecycle Manager

**Role:** `MarkdownPreviewManager` implements both `vscode.WebviewPanelSerializer` (for `DynamicMarkdownPreview` persistence across restarts) and `vscode.CustomTextEditorProvider` (for `StaticMarkdownPreview`). It owns the collections of active previews and arbitrates which one is "active".

**Key symbols:**

- `MarkdownPreviewManager` (class) — `previewManager.ts:72`. Extends `Disposable`. Holds two `PreviewStore<T>` instances: `#dynamicPreviews` and `#staticPreviews`.
- Constructor — `previewManager.ts:87`. Registers:
  - `vscode.window.registerWebviewPanelSerializer(DynamicMarkdownPreview.viewType, this)` at `previewManager.ts:100`
  - `vscode.window.registerCustomEditorProvider(StaticMarkdownPreview.customEditorViewType, this, ...)` at `previewManager.ts:102`
  - `vscode.window.onDidChangeActiveTextEditor` handler at `previewManager.ts:106` to restore scroll position when switching to a markdown file.
- `openDynamicPreview(resource, settings)` — `previewManager.ts:135`. Checks `#dynamicPreviews.get(resource, settings)` for a match; if found, reveals it; if not, calls `#createNewDynamicPreview`. Then calls `preview.update(resource, ...)`.
- `deserializeWebviewPanel(webview, state)` — `previewManager.ts:188`. Implements `WebviewPanelSerializer`. Parses state from `state.resource`, `state.locked`, `state.line`, `state.resourceColumn` at `previewManager.ts:193-196`. Calls `DynamicMarkdownPreview.revive()` then `#registerDynamicPreview`.
- `resolveCustomTextEditor(document, webview)` — `previewManager.ts:246`. Implements `CustomTextEditorProvider`. Calls `StaticMarkdownPreview.revive()` then `#registerStaticPreview`. Sets `#activePreview`.
- `toggleLock()` — `previewManager.ts:169`. Delegates to `DynamicMarkdownPreview.toggleLock()` and disposes now-redundant duplicates.

- `PreviewStore<T>` (class) — `previewManager.ts:25`. Internal Set-backed store. `get(resource, previewSettings)` at `previewManager.ts:41` iterates previews calling `matchesResource(resource, previewColumn, locked)`. `#resolvePreviewColumn()` at `previewManager.ts:59` handles `ViewColumn.Active` and `ViewColumn.Beside` by reading from `vscode.window.tabGroups.activeTabGroup.viewColumn`.

---

#### 6. `src/client/client.ts` — Language Server Client Setup

**Role:** Wraps the `vscode-languageclient` `BaseLanguageClient` to create `MdLanguageClient`, and implements the bidirectional RPC bridge between the VS Code extension host and the `vscode-markdown-languageserver` worker process.

**Key symbols:**

- `MdLanguageClient` (class) — `client.ts:18`. Thin wrapper holding `#client: BaseLanguageClient` and `#workspace: VsCodeMdWorkspace`. Public methods proxy `sendRequest` calls to `protocol.*` request types:
  - `resolveLinkTarget` — `client.ts:36`
  - `getEditForFileRenames` — `client.ts:40`
  - `getReferencesToFileInWorkspace` — `client.ts:44`
  - `prepareUpdatePastedLinks` — `client.ts:48`
  - `getUpdatePastedLinksEdit` — `client.ts:55`

- `startClient(factory, parser)` — `client.ts:64`. Async. Configures `LanguageClientOptions` at `client.ts:68`:
  - `documentSelector`: `markdownLanguageIds`
  - `synchronize.fileEvents`: glob watcher for `**/*.{md,...}`
  - `initializationOptions`: passes `markdownFileExtensions` and `i10lLocation`
  - `diagnosticPullOptions`: enables pull-based diagnostics on change and tab-switch for markdown files

- **Server-to-client request handlers** (the server calls into the extension for file system access):
  - `proto.parse` — `client.ts:109`. Tokenizes a document. If `e.text` is provided, wraps it in `InMemoryDocument`; otherwise fetches from `VsCodeMdWorkspace`. Calls `parser.tokenize(doc)`.
  - `proto.fs_readFile` — `client.ts:123`. Calls `vscode.workspace.fs.readFile`, returns `Array.from(bytes)`.
  - `proto.fs_stat` — `client.ts:128`. Returns `{ isDirectory: boolean }` or `undefined`.
  - `proto.fs_readDirectory` — `client.ts:138`. Returns `[string, {isDirectory}][]`.
  - `proto.findMarkdownFilesInWorkspace` — `client.ts:144`. Uses `vscode.workspace.findFiles`.
  - `proto.fs_watcher_create` — `client.ts:150`. Delegates to `FileWatcherManager.create()`, sends `proto.fs_watcher_onChange` back on file events.
  - `proto.fs_watcher_delete` — `client.ts:165`. Delegates to `FileWatcherManager.delete()`.

- **Command proxies** registered at `client.ts:169`:
  - `vscodeMarkdownLanguageservice.open` → `vscode.open`
  - `vscodeMarkdownLanguageservice.rename` → `editor.action.rename`

- Registers notebook document sync for `{ notebook: '*', cells: [{ language: 'markdown' }] }` at `client.ts:96`.

**Porting:** The file system handler pattern (server calls client for FS access) is an inversion of the typical LSP flow. In a Tauri port, these handlers would need to be replaced with a Tauri IPC bridge or the language server would need direct filesystem access.

---

#### 7. `src/client/protocol.ts` — Custom LSP Request Definitions

**Role:** Declares all custom (non-standard LSP) `RequestType` objects used in the markdown language server protocol. These extend the base LSP wire protocol.

**From-server requests (server calls extension host):**
- `parse` — `protocol.ts:18`. `markdown/parse`. Input: `{ uri, text? }`. Output: `md.Token[]`.
- `fs_readFile` — `protocol.ts:20`. `markdown/fs/readFile`. Output: `number[]` (bytes).
- `fs_readDirectory` — `protocol.ts:21`. `markdown/fs/readDirectory`. Output: file/dir listing.
- `fs_stat` — `protocol.ts:22`. `markdown/fs/stat`.
- `fs_watcher_create` — `protocol.ts:24`. `markdown/fs/watcher/create`. Input includes `md.FileWatcherOptions`.
- `fs_watcher_delete` — `protocol.ts:25`. `markdown/fs/watcher/delete`.
- `findMarkdownFilesInWorkspace` — `protocol.ts:27`. `markdown/findMarkdownFilesInWorkspace`.

**To-server requests (extension host calls server):**
- `getReferencesToFileInWorkspace` — `protocol.ts:31`. `markdown/getReferencesToFileInWorkspace`. Output: `lsp.Location[]`.
- `getEditForFileRenames` — `protocol.ts:32`. `markdown/getEditForFileRenames`. Output: `{ participatingRenames, edit }`.
- `prepareUpdatePastedLinks` — `protocol.ts:34`. `markdown/prepareUpdatePastedLinks`. Output: opaque `string` metadata.
- `getUpdatePastedLinksEdit` — `protocol.ts:35`. `markdown/getUpdatePastedLinksEdit`. Output: `lsp.TextEdit[]`.
- `fs_watcher_onChange` — `protocol.ts:37`. `markdown/fs/watcher/onChange`. Sent from extension host to server when a watched file changes.
- `resolveLinkTarget` — `protocol.ts:39`. `markdown/resolveLinkTarget`. Output: `ResolvedDocumentLinkTarget` discriminated union (`file | folder | external`).

**Type definitions:**
- `ResolvedDocumentLinkTarget` — `protocol.ts:12`. Three-case discriminated union using `kind` discriminant.

---

#### 8. `src/languageFeatures/diagnostics.ts` — Link Validation Diagnostics

**Role:** Provides the client-side surface for link validation diagnostics: a `CodeActionProvider` offering "ignore link" quick fixes, and a language status bar item to toggle validation on/off.

**Key symbols:**

- `DiagnosticCode` (enum) — `diagnostics.ts:12`. Four codes: `link_noSuchReferences`, `link_noSuchHeaderInOwnFile`, `link_noSuchFile`, `link_noSuchHeaderInFile`. These values match codes produced by the language server.

- `AddToIgnoreLinksQuickFixProvider` (class) — `diagnostics.ts:20`. Implements `vscode.CodeActionProvider`. Registered via `vscode.languages.registerCodeActionsProvider` at `diagnostics.ts:31`.
  - `provideCodeActions()` — `diagnostics.ts:45`. Iterates `context.diagnostics`, matches on `DiagnosticCode`, extracts `hrefText` from `diagnostic.data.hrefText` (cast via `unknown`). Creates a `vscode.CodeAction` of kind `QuickFix` wired to command `_markdown.addToIgnoreLinks`.
  - The registered command `_markdown.addToIgnoreLinks` at `diagnostics.ts:34` reads config `markdown.validate.ignoredLinks`, adds the path, writes it back via `config.update(..., ConfigurationTarget.WorkspaceFolder)`.

- `registerMarkdownStatusItem(selector, commandManager)` — `diagnostics.ts:76`. Creates a `vscode.languages.createLanguageStatusItem('markdownStatus', selector)`. Updates status text based on `markdown.validate.enabled` config. Command `_markdown.toggleValidation` at `diagnostics.ts:83` calls `config.update('validate.enabled', enabled)`. Subscribes to `onDidChangeConfiguration` at `diagnostics.ts:117`.

- `registerDiagnosticSupport(selector, commandManager)` — `diagnostics.ts:125`. Public export. Composes both registrations.

**Note:** Diagnostic *production* is handled entirely server-side; this file only handles client-side presentation and quick-fix actions.

---

#### 9. `preview-src/index.ts` — Webview Entry Point (Browser-side)

**Role:** This script runs inside the VS Code webview (in a sandboxed browser context). It is the webview-side half of the preview bi-directional communication protocol. It initializes scroll sync, handles incoming messages from the extension, and posts outbound messages back to the extension.

**Key symbols:**

- `acquireVsCodeApi()` — `index.ts:25`. The VS Code webview global function; provides `postMessage`, `getState`, `setState`.
- `createPosterForVsCode(vscode, settings)` — imported from `messaging.ts`. Returns a `MessagePoster` used for all outbound messages. The source URI is injected from `settings.settings.source`.
- `onceDocumentLoaded` callback — `index.ts:72`. After DOM is ready:
  1. Parses initial HTML from `data-initial-md-content` meta attribute via `DOMParser` at `index.ts:74`.
  2. Appends parsed elements to `document.body`.
  3. Calls `domEval()` to re-execute any `<script>` tags.
  4. Restores scroll position from `state.scrollProgress` if present.
  5. Scrolls to fragment or line from settings.

- **Inbound message handler** (`window.addEventListener('message', ...)`) — `index.ts:215`:
  - `copyImage` — locates `<img>` by `data.id`, calls `copyImage()` which uses `navigator.clipboard.write` with a Canvas-rendered PNG blob.
  - `onDidChangeTextEditorSelection` — calls `marker.onDidChangeTextEditorSelection(data.line, documentVersion)`.
  - `updateView` — calls `onUpdateView(data.line)` which throttles calls to `scrollToRevealSourceLine`.
  - `updateContent` — parses new HTML with `DOMParser` then calls `morphdom()` at `index.ts:266` to reconcile DOM differences. Custom `onBeforeElUpdated` at `index.ts:268` preserves `data-line` attributes and `<details open>` state. Increments `documentVersion`, dispatches `vscode.markdown.updateContent` custom event, calls `addImageContexts()`.

- **Outbound event handlers:**
  - `document.addEventListener('dblclick', ...)` — `index.ts:316`. Computes source line from `getEditorLineNumberForPageOffset(event.pageY)`, posts `didClick` message.
  - `document.addEventListener('click', ...)` — `index.ts:342`. Intercepts non-anchor-scheme link clicks, posts `openLink` message. Pass-through schemes: `http:`, `https:`, `mailto:`, `vscode:`, `vscode-insiders:`.
  - `window.addEventListener('scroll', throttle(..., 50))` — `index.ts:377`. Computes topmost visible source line, posts `revealLine` message. `scrollDisabledCount` guard prevents feedback loops.

- `addImageContexts()` — `index.ts:159`. Assigns unique IDs (`image-0`, `image-1`, ...) to all `<img>` elements and sets `data-vscode-context` attribute for webview context menus.

- `domEval(el)` — `index.ts:438`. Re-executes scripts injected by morphdom by cloning `<script>` tags via `document.createElement('script')`.

---

#### 10. `preview-src/messaging.ts` — Outbound Webview Message Abstraction

**Role:** Provides the `MessagePoster` interface and its factory for the webview side. Injects `source` (the document URI) into every outbound message.

**Key symbols:**

- `MessagePoster` (interface) — `messaging.ts:10`. Single method `postMessage<T extends FromWebviewMessage.Type>(type, body)`.
- `createPosterForVsCode(vscode, settingsManager)` — `messaging.ts:20`. Returns an object whose `postMessage` merges `type`, `source: settingsManager.settings!.source`, and `body` into a single object passed to `vscode.postMessage`.

---

#### 11. `types/previewMessaging.d.ts` — Preview Message Type Contract

**Role:** The shared type contract for the bi-directional postMessage protocol between the extension host and the webview. Consumed by both `src/preview/preview.ts` (host side) and `preview-src/index.ts` / `preview-src/messaging.ts` (webview side).

**FromWebviewMessage (webview → host):**
- `CacheImageSizes` — `previewMessaging.d.ts:12`. Carries `imageData: Array<{id, width, height}>`.
- `RevealLine` — `previewMessaging.d.ts:17`. Carries `line: number`.
- `DidClick` — `previewMessaging.d.ts:22`. Carries `line: number`.
- `ClickLink` — `previewMessaging.d.ts:27`. Carries `href: string`.
- `ShowPreviewSecuritySelector` — `previewMessaging.d.ts:32`.
- `PreviewStyleLoadError` — `previewMessaging.d.ts:37`. Carries `unloadedStyles: readonly string[]`.
- `Type` union — `previewMessaging.d.ts:41`.

**ToWebviewMessage (host → webview):**
- `OnDidChangeTextEditorSelection` — `previewMessaging.d.ts:52`. Carries `line: number`.
- `UpdateView` — `previewMessaging.d.ts:57`. Carries `line: number`, `source: string`.
- `UpdateContent` — `previewMessaging.d.ts:62`. Carries `content: string` (partial HTML body).
- `CopyImageContent` — `previewMessaging.d.ts:67`. Carries `id: string`.
- `OpenImageContent` — `previewMessaging.d.ts:73`. Carries `imageSource: string`.
- `Type` union — `previewMessaging.d.ts:80`.

All messages extend `BaseMessage` which carries `source: string` — the stringified document URI — used as a routing key to reject messages intended for other preview instances.

---

#### 12. `src/markdownEngine.ts` — Markdown-It Rendering Engine

**Role:** `MarkdownItEngine` wraps the `markdown-it` library, adding VS Code-specific plugins, a token cache, and resource URI resolution for webview contexts. It also implements `IMdParser` so the language server can reuse the same tokenizer.

**Key symbols:**

- `MarkdownItEngine` (class) — `markdownEngine.ts:99`. Implements `IMdParser`. Lazily initializes `#md: Promise<MarkdownIt>`.
- `#getEngine(config)` — `markdownEngine.ts:132`. On first call, dynamically imports `markdown-it`, configures highlight.js via `getMarkdownOptions()` at `markdownEngine.ts:410`, applies contribution plugins from `contributionProvider.contributions.markdownItPlugins`, installs `markdown-it-front-matter` (extracted as a block rule), then applies six custom renderer patches, and finally uses `pluginSourceMap`.
- `render(input, resourceProvider)` — `markdownEngine.ts:208`. Accepts either a `ITextDocument` or a plain string. Returns `{html, containingImages}`.
- `tokenize(document)` — `markdownEngine.ts:234`. Implements `IMdParser.tokenize`. Used by the language server client handler in `client.ts:109`.
- `#tokenizeDocument(document, config, engine)` — `markdownEngine.ts:182`. Checks `TokenCache` before calling `engine.parse`.

**Renderer plugins installed in `#getEngine`:**
- `#addImageRenderer` — `markdownEngine.ts:253`. Adds `data-src` attribute preserving original URL; resolves `src` to webview URI via `#toResourceUri`. Collects all image sources into `env.containingImages`.
- `#addFencedRenderer` — `markdownEngine.ts:275`. Adds `hljs` class to fenced code blocks.
- `#addLinkNormalizer` — `markdownEngine.ts:291`. Rewrites `vscode://` and `vscode-insiders://` scheme URIs to `vscode.env.uriScheme`.
- `#addLinkValidator` — `markdownEngine.ts:307`. Extends link validation to allow `vscode:` and `data:image/` URIs.
- `#addNamedHeaders` — `markdownEngine.ts:317`. Computes heading slugs via `SlugBuilder` (GitHub-compatible) and sets `id` attribute.
- `#addLinkRenderer` — `markdownEngine.ts:347`. Copies `href` to `data-href` attribute for click interception in the webview.
- `pluginSourceMap` — `markdownEngine.ts:19`. Core ruler plugin that adds `data-line` and `code-line` class to all block-level tokens for scroll sync.

**Token cache:** `TokenCache` at `markdownEngine.ts:46` caches `MarkdownIt.Token[]` keyed by URI, document version, and `{breaks, linkify}` config. Cache invalidated when contribution plugins change.

**Config:** `#getConfig(resource)` at `markdownEngine.ts:244` reads `MarkdownPreviewConfiguration.getForResource(resource)` to obtain `breaks`, `linkify`, `typographer` settings.

---

### Cross-Cutting Synthesis

The markdown-language-features extension is structured around three independent but interconnected subsystems. The **language server subsystem** (`client/client.ts`, `client/protocol.ts`) connects to an external `vscode-markdown-languageserver` worker via IPC (Node) or web workers (browser), using a custom bidirectional RPC layer on top of LSP where the server calls back into the extension host for all file system access. The **preview subsystem** (`preview/preview.ts`, `preview/previewManager.ts`, `preview/documentRenderer.ts`, `markdownEngine.ts`) renders markdown through `markdown-it` to HTML, injects it into a `vscode.WebviewPanel`, and maintains two-way scroll synchronization through a typed `postMessage` protocol typed in `types/previewMessaging.d.ts`. The **webview runtime** (`preview-src/index.ts`, `preview-src/messaging.ts`) runs inside the sandboxed webview, using `acquireVsCodeApi()` for message passing and `morphdom` for incremental DOM diffing on content updates. For a Tauri/Rust port, the three most VS Code-specific couplings are: (1) `vscode.WebviewPanel` and its `postMessage`/`onDidReceiveMessage` APIs used throughout `preview.ts`; (2) `vscode-languageclient`'s `LanguageClient` / `BaseLanguageClient` types and `TransportKind.ipc` used in `extension.ts` and `client.ts`; (3) the `WebviewPanelSerializer` / `CustomTextEditorProvider` registration surface in `previewManager.ts`. The markdown-to-HTML rendering pipeline itself (`markdownEngine.ts`, `documentRenderer.ts`) is largely self-contained TypeScript/JavaScript and could run in a Tauri-hosted WebView with minimal changes, but the IPC and webview management layers require full replacement.

---

### Out-of-Partition References

- `vscode-markdown-languageserver` (npm package) — the external language server binary/worker loaded by `extension.ts:32-38`. Not in this partition.
- `extensions/markdown-language-features/src/util/openDocumentLink.ts` — `MdLinkOpener` class used by `preview.ts` and `previewManager.ts` for link resolution; not analysed in depth.
- `extensions/markdown-language-features/src/preview/topmostLineMonitor.ts` — `TopmostLineMonitor` used for cross-panel scroll position persistence; not analysed in depth.
- `extensions/markdown-language-features/src/preview/previewConfig.ts` — `MarkdownPreviewConfigurationManager` / `MarkdownPreviewConfiguration`; drives per-resource config caching.
- `extensions/markdown-language-features/src/preview/security.ts` — `ContentSecurityPolicyArbiter` / `MarkdownPreviewSecurityLevel`; provides CSP level decisions consumed by `documentRenderer.ts`.
- `extensions/markdown-language-features/src/client/workspace.ts` — `VsCodeMdWorkspace`; tracks open documents for the language server.
- `extensions/markdown-language-features/src/client/fileWatchingManager.ts` — `FileWatcherManager`; multiplexes `vscode.FileSystemWatcher` instances for the server-requested watchers.
- `extensions/markdown-language-features/src/markdownExtensions.ts` — `MarkdownContributionProvider` / `getMarkdownExtensionContributions`; aggregates preview styles, scripts, and markdown-it plugins from other extensions.
- `extensions/markdown-language-features/src/extension.browser.ts` — Browser entry point (not read); uses a web worker transport instead of IPC but calls the same `activateShared`.
- `extensions/markdown-language-features/preview-src/scroll-sync.ts` — `scrollToRevealSourceLine`, `getEditorLineNumberForPageOffset`; uses `data-line` attributes for bidirectional scroll sync; called from `index.ts`.
- `extensions/markdown-language-features/preview-src/activeLineMarker.ts` — `ActiveLineMarker`; highlights the active editor line in the preview DOM.
- `highlight.js` (npm) — dynamically imported in `markdownEngine.ts:411` for syntax highlighting in fenced code blocks.
- `morphdom` (npm) — used in `preview-src/index.ts:266` for incremental DOM patching on content updates.
- `lodash.throttle` (npm) — used in `preview-src/index.ts` for scroll event throttling.
- `markdown-it` / `markdown-it-front-matter` (npm) — the core markdown parsing libraries wrapped by `MarkdownItEngine`.
