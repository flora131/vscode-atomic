### Files Analysed

1. `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/ipynbMain.ts` (140 lines)
2. `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/notebookSerializer.ts` (89 lines)
3. `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/notebookSerializer.node.ts` (92 lines)
4. `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/notebookSerializer.web.ts` (86 lines)
5. `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/deserializers.ts` (374 lines)
6. `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/serializers.ts` (493 lines)
7. `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/notebookSerializerWorker.ts` (20 lines)
8. `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/notebookModelStoreSync.ts` (263 lines)
9. `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/notebookImagePaste.ts` (318 lines)
10. `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/notebookAttachmentCleaner.ts` (391 lines)
11. `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/common.ts` (101 lines)
12. `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/constants.ts` (25 lines)

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/ipynbMain.ts`

- **Role:** Extension entry point. Registers the `NotebookSerializer` for two notebook types (`jupyter-notebook` and `interactive`), wires up CodeLens, commands, image-paste, and attachment cleaning.
- **Key symbols:**
  - `activate` (`ipynbMain.ts:35`) — receives a `vscode.NotebookSerializer` instance (injected by the platform-specific module) and calls `vscode.workspace.registerNotebookSerializer` twice (`ipynbMain.ts:53`, `ipynbMain.ts:67`).
  - `NotebookMetadata` type (`ipynbMain.ts:14`) — mirrors `nbformat.INotebookMetadata` from `@jupyterlab/nbformat`.
  - `OptionsWithCellContentMetadata` (`ipynbMain.ts:32`) — extends `vscode.NotebookDocumentContentOptions` with a `cellContentMetadata.attachments` flag.
  - `setNotebookMetadata` (`ipynbMain.ts:120`) — public API surface returned from `activate`; uses `vscode.WorkspaceEdit` + `NotebookEdit.updateNotebookMetadata` to write kernel/language metadata back to an open document.
  - `exportNotebook` (`ipynbMain.ts:117`) — thin wrapper over `serializeNotebookToString` from `./serializers`.
- **Control flow:** `activate` → registers serializer + CodeLens provider → registers two commands (`ipynb.newUntitledIpynb`, `ipynb.openIpynbInNotebookEditor`) → calls `notebookImagePasteSetup()` → conditionally instantiates `AttachmentCleaner` behind `ipynb.pasteImagesAsAttachments.enabled` configuration flag → returns public API object.
- **Data flow:** Inbound `vscode.NotebookSerializer` instance is stored by the VS Code host; the returned object (`exportNotebook`, `setNotebookMetadata`, `dropCustomMetadata`) is consumed by external callers such as the Jupyter extension.
- **Dependencies:** `./notebookModelStoreSync`, `./notebookImagePaste`, `./notebookAttachmentCleaner`, `./serializers`, `./constants`, `vscode`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/notebookSerializer.ts`

- **Role:** Abstract base class (`NotebookSerializerBase`) implementing `vscode.NotebookSerializer`. Handles both `deserializeNotebook` and `serializeNotebook` in a synchronous-on-main-thread manner. Subclasses in node/web override `serializeNotebook` to delegate to a worker.
- **Key symbols:**
  - `NotebookSerializerBase` (`notebookSerializer.ts:13`) — abstract class extending `vscode.Disposable`.
  - `deserializeNotebook` (`notebookSerializer.ts:24`) — decodes `Uint8Array` → UTF-8 string → JSON parse; handles a `__webview_backup` field that redirects to a backup file stored in `context.globalStorageUri` using FNV hash (`fnv.fast1a32hex`) (`notebookSerializer.ts:36-41`); rejects notebooks with `nbformat < 4` (`notebookSerializer.ts:49`); calls `detectIndent` on first 1 KB (`notebookSerializer.ts:54`); delegates to `jupyterNotebookModelToNotebookData` (`notebookSerializer.ts:69`).
  - `serializeNotebook` (`notebookSerializer.ts:79`) — guard-checks `disposed`, then calls `serializeNotebookToString` and `TextEncoder.encode`.
- **Control flow:** `deserializeNotebook`: bytes → string → JSON → optional backup redirect → version check → indent detection → `getPreferredLanguage` → `jupyterNotebookModelToNotebookData` → attach `indentAmount` → return `NotebookData`. `serializeNotebook`: `NotebookData` → `serializeNotebookToString` → `TextEncoder` → `Uint8Array`.
- **Data flow:** Raw `.ipynb` bytes flow in; `vscode.NotebookData` (cells + metadata) flows out (deserialize). Reverse for serialize.
- **Dependencies:** `@jupyterlab/nbformat`, `detect-indent`, `@enonic/fnv-plus`, `vscode`, `./deserializers`, `./serializers`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/notebookSerializer.node.ts`

- **Role:** Node.js-specific concrete `NotebookSerializer`. Overrides `serializeNotebook` to offload JSON serialization to a Node.js `worker_threads` Worker when `ipynb.experimental.serialization` is enabled (default: `true`).
- **Key symbols:**
  - `NotebookSerializer` (`notebookSerializer.node.ts:10`) — extends `NotebookSerializerBase`.
  - `startWorker` (`notebookSerializer.node.ts:45`) — lazily imports `node:worker_threads` and spawns `notebookSerializerWorker.js` from the extension's `dist/` or `out/` directory (`notebookSerializer.node.ts:54`).
  - `serializeViaWorker` (`notebookSerializer.node.ts:75`) — posts `{ data: NotebookData, id: uuid }` to the worker; stores a `DeferredPromise<Uint8Array>` in `this.tasks` keyed by id; resolves when the `message` event returns with the same id (`notebookSerializer.node.ts:61-65`).
  - `getOutputDir` (`notebookSerializer.node.ts:88`) — inspects `packageJSON.main` to select `dist` vs `out`.
- **Control flow:** `serializeNotebook` → check `experimentalSave` → `serializeViaWorker` → `startWorker` (lazy) → `worker.postMessage` → await `DeferredPromise` → return bytes. Falls back to `super.serializeNotebook` when flag is off.
- **Data flow:** `NotebookData` is sent via structured clone to the Worker thread; the Worker calls `serializeNotebookToString`, encodes with `TextEncoder`, and posts back `Uint8Array`.
- **Dependencies:** `node:worker_threads` (dynamic import), `./helper` (`DeferredPromise`, `generateUuid`), `./notebookSerializer`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/notebookSerializer.web.ts`

- **Role:** Web-specific concrete `NotebookSerializer`. Identical logic to the Node variant but uses the browser `Worker` API with `dist/browser/notebookSerializerWorker.js` (`notebookSerializer.web.ts:52`). Uses `worker.onmessage`/`worker.onerror` instead of `worker.on`.
- **Key symbols:** `NotebookSerializer` (`notebookSerializer.web.ts:10`), `startWorker` (`notebookSerializer.web.ts:45`), `serializeViaWorker` (`notebookSerializer.web.ts:75`).
- **Control flow:** Same pattern as Node variant; only the Worker construction and event binding differ.
- **Dependencies:** Browser `Worker` global, `./helper`, `./notebookSerializer`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/notebookSerializerWorker.ts`

- **Role:** Runs inside the Worker thread (both Node and Web variants share this source). Listens for `{ id, data }` messages, calls `serializeNotebookToString`, encodes to `Uint8Array`, and posts back `{ id, data }`.
- **Key symbols:** Worker message handler (`notebookSerializerWorker.ts:12`) — entry guard `if (parentPort)` ensures it only runs in a Worker context.
- **Control flow:** `parentPort.on('message')` → `serializeNotebookToString(data)` → `TextEncoder().encode(json)` → `parentPort.postMessage({ id, data: bytes })`.
- **Data flow:** `NotebookData` (structured-cloned from parent) → JSON string → UTF-8 bytes → posted back to parent thread.
- **Dependencies:** `node:worker_threads` (`parentPort`), `./serializers`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/deserializers.ts`

- **Role:** Converts `nbformat.INotebookContent` (raw Jupyter JSON) into VS Code `NotebookData`. Handles all Jupyter output types and language mapping.
- **Key symbols:**
  - `jupyterNotebookModelToNotebookData` (`deserializers.ts:357`) — top-level export; iterates `notebookContent.cells`, calling `createNotebookCellDataFromJupyterCell` for each.
  - `getPreferredLanguage` (`deserializers.ts:20`) — reads `language_info.name` or `kernelspec.language` from notebook metadata; falls back to `python` if `ms-python.python` is installed, else `csharp` if .NET extension present.
  - `translateKernelLanguageToMonaco` (`deserializers.ts:35`) — normalises Jupyter language IDs (e.g. `c#` → `csharp`, `q#` → `qsharp`) using `jupyterLanguageToMonacoLanguageMapping` map.
  - `jupyterCellOutputToCellOutput` (`deserializers.ts:264`) — dispatches on `output.output_type` via `cellOutputMappers` (a `Map<nbformat.OutputType, fn>`) to `translateDisplayDataOutput`, `translateStreamOutput`, or `translateErrorOutput`.
  - `convertJupyterOutputToBuffer` (`deserializers.ts:119`) — converts Jupyter mime-keyed data to `NotebookCellOutputItem`; base64-decodes images with `Buffer.from(value, 'base64')` on Node, `atob` on Web (`deserializers.ts:133-136`).
  - `sortOutputItemsBasedOnDisplayOrder` (`deserializers.ts:75`) — sorts outputs by a priority array `orderOfMimeTypes`; vendored mime types with empty data get index -1.
  - `getNotebookCellMetadata` (`deserializers.ts:153`) — extracts `execution_count`, `metadata`, `id`, and `attachments` from each raw cell into a `CellMetadata` object.
- **Control flow:** `jupyterNotebookModelToNotebookData` → for each cell → `createNotebookCellDataFromJupyterCell` (dispatches by `cell_type`) → raw/markdown/code cell factories → for code cells, maps outputs via `jupyterCellOutputToCellOutput`.
- **Data flow:** `Partial<nbformat.INotebookContent>` → `NotebookData` with `cells: NotebookCellData[]`; each `NotebookCellData` holds source string, language ID, `NotebookCellOutput[]`, execution summary, and `CellMetadata`.
- **Dependencies:** `@jupyterlab/nbformat`, `vscode`, `./common` (`CellMetadata`, `CellOutputMetadata`), `./constants`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/serializers.ts`

- **Role:** Converts VS Code `NotebookData` / `NotebookDocument` back to Jupyter JSON string. Inverse of `deserializers.ts`. Also exports utility functions used by `notebookModelStoreSync.ts`.
- **Key symbols:**
  - `serializeNotebookToString` (`serializers.ts:462`) — extracts metadata via `getNotebookMetadata`, maps cells via `createJupyterCellFromNotebookCell` + `pruneCell`, detects `indentAmount` from metadata, calls `serializeNotebookToJSON`.
  - `serializeNotebookToJSON` (`serializers.ts:477`) — applies `sortObjectPropertiesRecursively` then `JSON.stringify(sorted, undefined, indentAmount)` + trailing newline.
  - `sortObjectPropertiesRecursively` (`serializers.ts:34`) — recursive alphabetical key sort to match Jupyter Lab's serialization order.
  - `createJupyterCellFromNotebookCell` (`serializers.ts:13`) — dispatches by `vscCell.kind` and `languageId` to `createMarkdownCellFromNotebookCell`, `createRawCellFromNotebookCell`, or `createCodeCellFromNotebookCell`.
  - `createCodeCellFromNotebookCell` (`serializers.ts:87`) — builds `nbformat.ICodeCell`; writes `execution_count` from metadata (not `executionSummary`) to ensure reverted changes are respected (`serializers.ts:99-108`); calls `splitCellSourceIntoMultilineString` and `translateCellDisplayOutput` for each output.
  - `translateCellDisplayOutput` (`serializers.ts:161`) — switches on `customMetadata.outputType`; handles `error`, `stream`, `display_data`, `execute_result`, `update_display_data`, and unrecognised types with fallback detection.
  - `convertOutputMimeToJupyterOutput` (`serializers.ts:344`) — inverse of `convertJupyterOutputToBuffer`; encodes image bytes back to base64 using `Buffer` or `btoa`.
  - `splitCellSourceIntoMultilineString` (`serializers.ts:137`) — normalises CRLF → LF, then splits on `\n` adding `\n` to all but the last non-empty line.
  - `getCellMetadata` (`serializers.ts:51`) — type-dispatches on `hasKey(options, { cell: true })` to pull metadata from either a live `NotebookCell` or a `NotebookCellData`.
  - `getVSCodeCellLanguageId` / `setVSCodeCellLanguageId` / `removeVSCodeCellLanguageId` (`serializers.ts:74-85`) — manage the `metadata.vscode.languageId` field used to persist per-cell language overrides in the ipynb JSON.
  - `pruneCell` (`serializers.ts:392`) — strips disallowed keys from each output object using `AllowedCellOutputKeys` allow-sets keyed by `output_type`.
  - `getNotebookMetadata` (`serializers.ts:484`) — normalises document-level metadata; defaults `nbformat` to `4`, `nbformat_minor` to `5` from `defaultNotebookFormat`.
- **Control flow:** `serializeNotebookToString` → `getNotebookMetadata` → for each `NotebookCellData`: `createJupyterCellFromNotebookCell` → `pruneCell` → `serializeNotebookToJSON` (sort + stringify).
- **Data flow:** `NotebookData` (VS Code internal model) → `Partial<nbformat.INotebookContent>` → JSON string.
- **Dependencies:** `@jupyterlab/nbformat`, `vscode`, `./common`, `./constants`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/notebookModelStoreSync.ts`

- **Role:** Keeps the in-memory `NotebookDocument` model metadata in sync with what will be serialised to disk, so that diff views and metadata accesses are consistent with the saved ipynb JSON.
- **Key symbols:**
  - `activate` (`notebookModelStoreSync.ts:26`) — subscribes to `workspace.onDidChangeNotebookDocument` and `workspace.onWillSaveNotebookDocument`.
  - `onDidChangeNotebookCells` (`notebookModelStoreSync.ts:127`) — core logic: for each `cellChange` in `e.cellChanges`, reconciles `execution_count` (handles clear-all, execution, and reverted states via `pendingCellUpdates` WeakSet); manages `vscode.languageId` in cell metadata via `setVSCodeCellLanguageId`/`removeVSCodeCellLanguageId`; for `contentChanges.addedCells`, assigns UUIDs for nbformat ≥ 4.5 via `generateCellId`.
  - `trackAndUpdateCellMetadata` (`notebookModelStoreSync.ts:106`) — batches metadata edits into a single `WorkspaceEdit` + `NotebookEdit.updateCellMetadata`; tracks pending promises in `pendingNotebookCellModelUpdates` (WeakMap) so that `waitForPendingModelUpdates` can block saves.
  - `debounceOnDidChangeNotebookDocument` (`notebookModelStoreSync.ts:47`) — exported debounce helper; merges consecutive events for the same notebook within a 200 ms window (`notebookModelStoreSync.ts:71`).
  - `generateCellId` (`notebookModelStoreSync.ts:241`) — generates a unique 8-char hex ID (truncated UUID, dashes removed) by checking all existing cell IDs for uniqueness.
  - `isCellIdRequired` (`notebookModelStoreSync.ts:231`) — returns `true` for `nbformat >= 5` or `nbformat == 4 && nbformat_minor >= 5`.
- **Control flow:** `workspace.onDidChangeNotebookDocument` → (optionally debounced) → `onDidChangeNotebookCells` → collect metadata updates → `trackAndUpdateCellMetadata` → `workspace.applyEdit`. On `onWillSaveNotebookDocument`: flush debounce, await all `pendingNotebookCellModelUpdates`.
- **Data flow:** `NotebookDocumentChangeEvent` (cell changes, content changes) → computes `CellMetadata` deltas → applies `WorkspaceEdit` → updates in-memory model metadata.
- **Dependencies:** `vscode`, `./serializers` (`getCellMetadata`, `getVSCodeCellLanguageId`, `removeVSCodeCellLanguageId`, `setVSCodeCellLanguageId`, `sortObjectPropertiesRecursively`, `getNotebookMetadata`), `./common`, `@jupyterlab/nbformat`, `./helper` (`generateUuid`).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/notebookImagePaste.ts`

- **Role:** Implements image paste/drop into Jupyter notebook Markdown cells as base64-encoded `attachment:` metadata entries.
- **Key symbols:**
  - `DropOrPasteEditProvider` (`notebookImagePaste.ts:49`) — implements both `vscode.DocumentPasteEditProvider` and `vscode.DocumentDropEditProvider`.
  - `createInsertImageAttachmentEdit` (`notebookImagePaste.ts:94`) — private method that (1) calls `getDroppedImageData`, (2) finds the cell via `getCellFromCellDocument`, (3) calls `buildAttachment`, (4) constructs a `WorkspaceEdit` using `NotebookEdit.updateCellMetadata` and a `SnippetString` for the markdown reference.
  - `getDroppedImageData` (`notebookImagePaste.ts:136`) — iterates `DataTransfer` items; prefers direct image mime types; falls back to `text/uri-list` which is split on `\r?\n`, each URI resolved via `vscode.workspace.fs.readFile`.
  - `buildAttachment` (`notebookImagePaste.ts:256`) — encodes image bytes to base64 using `encodeBase64` (`notebookImagePaste.ts:211`); handles filename collision by appending `-2`, `-3`, etc.; stores as `{ [mimeType]: b64 }` under `cellMetadata.attachments[filename]`.
  - `notebookImagePasteSetup` (`notebookImagePaste.ts:299`) — exported factory; registers the provider for `JUPYTER_NOTEBOOK_MARKDOWN_SELECTOR` with `pasteMimeTypes: [png, text/uri-list]` and drop mime types including all `imageExtToMime` values.
  - `encodeBase64` (`notebookImagePaste.ts:211`) — custom base64 encoder (copied from VS Code core `buffer.ts`); operates on `Uint8Array`.
- **Control flow:** User paste/drop → `provideDocumentPasteEdits`/`provideDocumentDropEdits` → `createInsertImageAttachmentEdit` → `getDroppedImageData` → `buildAttachment` → return `DocumentPasteEdit`/`DocumentDropEdit` with `additionalEdit` (metadata) and `insertText` (snippet).
- **Dependencies:** `vscode`, `./constants` (`JUPYTER_NOTEBOOK_MARKDOWN_SELECTOR`), Node `path` module (`basename`, `extname`).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/notebookAttachmentCleaner.ts`

- **Role:** Garbage-collects unused image attachments in Jupyter Markdown cell metadata; provides diagnostics and code actions for dangling attachment references.
- **Key symbols:**
  - `AttachmentCleaner` (`notebookAttachmentCleaner.ts:29`) — implements `vscode.CodeActionProvider`; maintains `_attachmentCache` (3-level nested Map: notebook URI → cell fragment → filename → `IAttachmentData`).
  - `cleanNotebookAttachments` (`notebookAttachmentCleaner.ts:174`) — core private method; calls `getAttachmentNames` to parse `attachment:` markdown references via regex (`/!\[.*?\]\(<?attachment:(?<filename>.*?)>?\)/gm`) (`notebookAttachmentCleaner.ts:370`); cross-references with `cell.metadata.attachments`; caches unused attachments; emits diagnostics for references with no metadata entry; returns a `NotebookEdit.updateCellMetadata` if any change occurred.
  - `analyzeMissingAttachments` (`notebookAttachmentCleaner.ts:242`) — scans open text documents for missing attachment metadata at open/close time; updates `_imageDiagnosticCollection`.
  - `provideCodeActions` (`notebookAttachmentCleaner.ts:144`) — offers a `QuickFix` command (`ipynb.cleanInvalidImageAttachment`) to delete the invalid markdown reference text.
  - `_delayer` (`notebookAttachmentCleaner.ts:35`) — `Delayer(750)` from `./helper`; debounces `onDidChangeNotebookDocument` handler.
- **Control flow:** `onDidChangeNotebookDocument` (debounced 750 ms) → `cleanNotebookAttachments` per changed Markup cell → apply `WorkspaceEdit`. `onWillSaveNotebookDocument` (manual save) → flush delayer → `cleanNotebookAttachments` for all Markup cells → `e.waitUntil(workspaceEdit)`.
- **Data flow:** `vscode.TextDocument` content (markdown source) → regex parse → compare with `cell.metadata.attachments` → produce `NotebookEdit` removing stale keys or cache-restoring temporarily removed ones → diagnostic `IAttachmentDiagnostic[]` → `DiagnosticCollection`.
- **Dependencies:** `vscode`, `./constants` (`ATTACHMENT_CLEANUP_COMMANDID`, `JUPYTER_NOTEBOOK_MARKDOWN_SELECTOR`), `./helper` (`deepClone`, `objectEquals`, `Delayer`).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/common.ts`

- **Role:** Shared TypeScript interfaces (`CellOutputMetadata`, `CellMetadata`) and the `hasKey` type-narrowing utility used across serializers and deserializers.
- **Key symbols:**
  - `CellOutputMetadata` (`common.ts:12`) — stores `outputType`, `metadata`, `transient.display_id`, `executionCount`, `__isJson`; mirrors Jupyter output metadata in VS Code's `NotebookCellOutput.metadata`.
  - `CellMetadata` (`common.ts:49`) — stores `id`, `attachments` (typed as `nbformat.IAttachments`), `metadata` (typed as `Partial<nbformat.ICellMetadata> & { vscode?: { languageId? } }`), `execution_count`.
  - `hasKey` (`common.ts:93`) — generic type guard; checks presence of keys at runtime to discriminate union types.
- **Dependencies:** `@jupyterlab/nbformat`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/src/constants.ts`

- **Role:** Shared constants used across the extension.
- **Key symbols:**
  - `defaultNotebookFormat` (`constants.ts:8`) — `{ major: 4, minor: 5 }`.
  - `JUPYTER_NOTEBOOK_MARKDOWN_SELECTOR` (`constants.ts:11`) — `{ notebookType: 'jupyter-notebook', language: 'markdown' }`.
  - `CellOutputMimeTypes` (`constants.ts:18`) — VS Code-specific MIME type enum: `error = 'application/vnd.code.notebook.error'`, `stderr = 'application/vnd.code.notebook.stderr'`, `stdout = 'application/vnd.code.notebook.stdout'`.
  - `NotebookCellKindMarkup = 1` (`constants.ts:14`) and `NotebookCellKindCode = 2` (`constants.ts:16`) — copied from VS Code enum because `vscode` cannot be imported in Worker threads.
- **Dependencies:** `vscode` (type-only, `DocumentSelector`).

---

### Cross-Cutting Synthesis

The `extensions/ipynb` partition implements a self-contained VS Code extension that bridges the Jupyter `.ipynb` file format to VS Code's Notebook API. The architecture is built around a single abstract serializer class (`NotebookSerializerBase` in `notebookSerializer.ts:13`) that is subclassed platform-specifically: `notebookSerializer.node.ts` uses `node:worker_threads`, while `notebookSerializer.web.ts` uses the browser `Worker` API — both offloading JSON serialization via `notebookSerializerWorker.ts` to avoid blocking the extension host thread. The worker-task matching uses a UUID-keyed `DeferredPromise` map pattern (`notebookSerializer.node.ts:13`, `notebookSerializer.node.ts:61-65`). Deserialization (`deserializers.ts`) converts `nbformat.INotebookContent` into VS Code `NotebookData` by mapping every Jupyter output type (`display_data`, `execute_result`, `stream`, `error`) through a `Map<OutputType, fn>` dispatch table (`deserializers.ts:257-262`), handling base64 image decode dually via `Buffer` (Node) or `atob` (browser) (`deserializers.ts:133-136`). Serialization (`serializers.ts`) is the inverse: it reconstructs Jupyter JSON via `createJupyterCellFromNotebookCell`, enforces alphabetical key order via `sortObjectPropertiesRecursively` (`serializers.ts:34`), and normalises multiline cell source to arrays-of-lines. Model state sync (`notebookModelStoreSync.ts`) reacts to every notebook change event to keep `execution_count`, per-cell language IDs, and cell UUIDs (nbformat 4.5+) in sync with what the serialiser will write, using a debounced event merge and a WeakMap of pending promises to block saves until metadata edits complete. Image paste (`notebookImagePaste.ts`) and attachment cleanup (`notebookAttachmentCleaner.ts`) operate orthogonally on Markdown cell `attachments` metadata. The entire extension is registered from `ipynbMain.ts:35`, which receives a pre-constructed serializer and wires up all providers. Porting this to Tauri/Rust would require reimplementing the `vscode.NotebookSerializer` interface boundary (a Tauri command surface), the worker-thread serialization pipeline (Rust async tasks or threads), the `WorkspaceEdit`/`NotebookEdit` mutation API, all four VS Code provider interfaces (CodeLens, DocumentPasteEdit, DocumentDropEdit, CodeAction), and the `DataTransfer` API — plus a Rust-native JSON round-trip for nbformat that preserves key ordering and multiline string splitting semantics.

---

### Out-of-Partition References

- `vscode` extension API (all of `vscode.*`) — the central abstraction layer; `NotebookSerializer`, `NotebookData`, `NotebookCellData`, `NotebookCellOutput`, `NotebookCellOutputItem`, `NotebookEdit`, `WorkspaceEdit`, `DocumentPasteEditProvider`, `DocumentDropEditProvider`, `CodeActionProvider`, `DiagnosticCollection`, `CancellationToken`, `Disposable`, `ExtensionContext` — all from VS Code host process, not in-partition.
- `@jupyterlab/nbformat` — external npm package defining `INotebookContent`, `ICell`, `ICodeCell`, `IMarkdownCell`, `IRawCell`, `IOutput`, `IDisplayData`, `IStream`, `IError`, `IExecuteResult`, `ICellMetadata`, `INotebookMetadata`, `IAttachments`, `OutputType`.
- `detect-indent` — external npm package used in `notebookSerializer.ts:54`.
- `@enonic/fnv-plus` — external npm package for FNV-1a hash in `notebookSerializer.ts:10,37`.
- `ms-python.python` extension — checked via `vscode.extensions.getExtension('ms-python.python')` in `deserializers.ts:27` to select default language.
- `ms-dotnettools.dotnet-interactive-vscode` extension — fallback check in `deserializers.ts:28`.
- `node:worker_threads` (`parentPort`, `Worker`) — Node.js built-in used in `notebookSerializer.node.ts:52` and `notebookSerializerWorker.ts:6`.
- Browser `Worker` global — Web platform API used in `notebookSerializer.web.ts:53`.
- VS Code core buffer utility — `encodeBase64` in `notebookImagePaste.ts:211` is explicitly documented as copied from `src/vs/base/common/buffer.ts:350-387`.
