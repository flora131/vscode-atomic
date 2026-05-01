# Partition 17 of 79 — Findings

## Scope
`extensions/ipynb/` (25 files, 4,925 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Partition 17: extensions/ipynb/ (Notebook Serialization)

## Overview
The `extensions/ipynb/` scope contains 25 files (4,925 LOC) focused on Jupyter Notebook integration into VS Code. This partition is relevant to porting core IDE functionality as it demonstrates how VS Code extends and integrates document serialization, language support, and editing capabilities for a non-native file format.

### Implementation
- `extensions/ipynb/src/ipynbMain.ts` — Entry point, registers NotebookSerializer with vscode.workspace API
- `extensions/ipynb/src/notebookSerializer.ts` — Core NotebookSerializer class implementing VS Code's notebook serialization contract
- `extensions/ipynb/src/notebookSerializer.node.ts` — Node.js-specific notebook serializer implementation
- `extensions/ipynb/src/notebookSerializer.web.ts` — Web-specific notebook serializer implementation
- `extensions/ipynb/src/serializers.ts` — Serialization utilities for notebook cells and outputs
- `extensions/ipynb/src/deserializers.ts` — Deserialization utilities for notebook cell content
- `extensions/ipynb/src/notebookSerializerWorker.ts` — Worker thread orchestration for serialization tasks
- `extensions/ipynb/src/notebookSerializerWorker.web.ts` — Web worker implementation for serialization
- `extensions/ipynb/src/notebookModelStoreSync.ts` — Syncs notebook model state with storage layer
- `extensions/ipynb/src/notebookImagePaste.ts` — Handles image paste operations in notebook cells
- `extensions/ipynb/src/notebookAttachmentCleaner.ts` — Manages cleanup of unused notebook attachments
- `extensions/ipynb/src/common.ts` — Shared utilities and constants
- `extensions/ipynb/src/helper.ts` — Helper functions for notebook processing
- `extensions/ipynb/src/constants.ts` — Magic strings and configuration constants
- `extensions/ipynb/notebook-src/cellAttachmentRenderer.ts` — Renders cell attachments in notebook view

### Tests
- `extensions/ipynb/src/test/index.ts` — Test harness and utilities
- `extensions/ipynb/src/test/serializers.test.ts` — Tests for serialization logic
- `extensions/ipynb/src/test/notebookModelStoreSync.test.ts` — Tests for model sync functionality
- `extensions/ipynb/src/test/clearOutputs.test.ts` — Tests for output clearing behavior

### Types / Interfaces
- `extensions/ipynb/src/types.d.ts` — TypeScript type definitions for notebook models

### Configuration
- `extensions/ipynb/package.json` — Extension manifest, dependencies, activation events
- `extensions/ipynb/package-lock.json` — Dependency lock file
- `extensions/ipynb/package.nls.json` — Localization strings
- `extensions/ipynb/tsconfig.json` — Base TypeScript configuration
- `extensions/ipynb/tsconfig.browser.json` — TypeScript configuration for browser builds
- `extensions/ipynb/.vscode/launch.json` — Debug launch configuration

### Documentation
- `extensions/ipynb/README.md` — Extension overview and usage documentation

### Notable Clusters
- `extensions/ipynb/src/` — 22 files, implements notebook serialization, deserialization, attachment management, and model synchronization with dual Node.js and Web build targets
- `extensions/ipynb/src/test/` — 4 files, comprehensive test coverage for serialization, model state sync, and output handling

## Research Relevance

This scope demonstrates key cross-platform abstraction patterns in VS Code:
- **Platform-specific implementations** (notebookSerializer.ts with .node.ts and .web.ts variants) show how VS Code abstracts platform differences
- **Workspace API integration** (registerNotebookSerializer) shows document handling and IDE integration points
- **Worker thread abstraction** (notebookSerializerWorker variants) shows concurrency patterns and performance optimization for resource-intensive operations
- **Dual-target build system** (tsconfig.json and tsconfig.browser.json) shows tooling required for cross-platform JavaScript/TypeScript development in VS Code

Porting this to Tauri/Rust would require:
1. Translating VSCode API calls (vscode.workspace, vscode.notebook) to Rust equivalents
2. Implementing Rust serialization/deserialization logic (replacing TypeScript/JavaScript implementations)
3. Managing cross-platform code paths (platform-specific modules) at the Rust level
4. Adapting worker/threading patterns from JavaScript Workers to Rust async/threading primitives

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Research: NotebookSerializer Patterns in VS Code IPynb Extension

## Overview
This research examines patterns related to VS Code's NotebookSerializer implementation within the `extensions/ipynb/` directory, focusing on what it would take to port this functionality from TypeScript/Electron to Tauri/Rust.

---

## Pattern 1: Abstract Serializer Base Class

**Where:** `extensions/ipynb/src/notebookSerializer.ts:13-88`

**What:** Abstract base class implementing the VS Code NotebookSerializer interface with core serialization/deserialization logic.

```typescript
export abstract class NotebookSerializerBase extends vscode.Disposable implements vscode.NotebookSerializer {
	protected disposed: boolean = false;
	constructor(protected readonly context: vscode.ExtensionContext) {
		super(() => { });
	}

	override dispose() {
		this.disposed = true;
		super.dispose();
	}

	public async deserializeNotebook(content: Uint8Array, _token: vscode.CancellationToken): Promise<vscode.NotebookData> {
		let contents = '';
		try {
			contents = new TextDecoder().decode(content);
		} catch {
		}

		let json = contents && /\S/.test(contents) ? (JSON.parse(contents) as Partial<nbformat.INotebookContent>) : {};

		if (json.__webview_backup) {
			const backupId = json.__webview_backup;
			const uri = this.context.globalStorageUri;
			const folder = uri.with({ path: this.context.globalStorageUri.path.replace('vscode.ipynb', 'ms-toolsai.jupyter') });
			const fileHash = fnv.fast1a32hex(backupId) as string;
			const fileName = `${fileHash}.ipynb`;
			const file = vscode.Uri.joinPath(folder, fileName);
			const data = await vscode.workspace.fs.readFile(file);
			json = data ? JSON.parse(data.toString()) : {};

			if (json.contents && typeof json.contents === 'string') {
				contents = json.contents;
				json = JSON.parse(contents) as Partial<nbformat.INotebookContent>;
			}
		}

		if (json.nbformat && json.nbformat < 4) {
			throw new Error('Only Jupyter notebooks version 4+ are supported');
		}

		const indentAmount = contents ? detectIndent(contents.substring(0, 1_000)).indent : ' ';

		const preferredCellLanguage = getPreferredLanguage(json.metadata);
		if ((json.cells || []).length === 0) {
			json.cells = [];
		}

		if (!json.metadata || (!json.metadata.kernelspec && !json.metadata.language_info)) {
			json.metadata = json.metadata || {};
			json.metadata.language_info = json.metadata.language_info || { name: preferredCellLanguage };
		}

		const data = jupyterNotebookModelToNotebookData(json, preferredCellLanguage);
		data.metadata = data.metadata || {};
		data.metadata.indentAmount = indentAmount;

		return data;
	}

	public async serializeNotebook(data: vscode.NotebookData, _token: vscode.CancellationToken): Promise<Uint8Array> {
		if (this.disposed) {
			return new Uint8Array(0);
		}

		const serialized = serializeNotebookToString(data);
		return new TextEncoder().encode(serialized);
	}
}
```

**Key aspects:**
- Extends both `vscode.Disposable` and implements `vscode.NotebookSerializer` interface
- Handles backup recovery from global storage URI
- Validates nbformat version (4+)
- Delegates to helper functions for format conversion
- Manages disposal state to prevent serialization after disposal

**Variations / call-sites:**
- `extensions/ipynb/src/notebookSerializer.node.ts:10` - Platform-specific Node.js subclass
- `extensions/ipynb/src/notebookSerializer.web.ts:10` - Platform-specific Web subclass

---

## Pattern 2: Platform-Specific Serializer with Worker Threading

**Where:** `extensions/ipynb/src/notebookSerializer.node.ts:10-91`

**What:** Node.js implementation of NotebookSerializer that offloads serialization to worker threads for performance.

```typescript
export class NotebookSerializer extends NotebookSerializerBase {
	private experimentalSave = vscode.workspace.getConfiguration('ipynb').get('experimental.serialization', true);
	private worker?: import('node:worker_threads').Worker;
	private tasks = new Map<string, DeferredPromise<Uint8Array>>();

	constructor(context: vscode.ExtensionContext) {
		super(context);
		context.subscriptions.push(vscode.workspace.onDidChangeConfiguration(e => {
			if (e.affectsConfiguration('ipynb.experimental.serialization')) {
				this.experimentalSave = vscode.workspace.getConfiguration('ipynb').get('experimental.serialization', true);
			}
		}));
	}

	override dispose() {
		try {
			void this.worker?.terminate();
		} catch {
			//
		}
		super.dispose();
	}

	public override async serializeNotebook(data: vscode.NotebookData, token: vscode.CancellationToken): Promise<Uint8Array> {
		if (this.disposed) {
			return new Uint8Array(0);
		}

		if (this.experimentalSave) {
			return this.serializeViaWorker(data);
		}

		return super.serializeNotebook(data, token);
	}

	private async startWorker() {
		if (this.disposed) {
			throw new Error('Serializer disposed');
		}
		if (this.worker) {
			return this.worker;
		}
		const { Worker } = await import('node:worker_threads');
		const outputDir = getOutputDir(this.context);
		this.worker = new Worker(vscode.Uri.joinPath(this.context.extensionUri, outputDir, 'notebookSerializerWorker.js').fsPath, {});
		this.worker.on('exit', (exitCode) => {
			if (!this.disposed) {
				console.error(`IPynb Notebook Serializer Worker exited unexpectedly`, exitCode);
			}
			this.worker = undefined;
		});
		this.worker.on('message', (result: { data: Uint8Array; id: string }) => {
			const task = this.tasks.get(result.id);
			if (task) {
				task.complete(result.data);
				this.tasks.delete(result.id);
			}
		});
		this.worker.on('error', (err) => {
			if (!this.disposed) {
				console.error(`IPynb Notebook Serializer Worker errored unexpectedly`, err);
			}
		});
		return this.worker;
	}

	private async serializeViaWorker(data: vscode.NotebookData): Promise<Uint8Array> {
		const worker = await this.startWorker();
		const id = generateUuid();

		const deferred = new DeferredPromise<Uint8Array>();
		this.tasks.set(id, deferred);
		worker.postMessage({ data, id });

		return deferred.p;
	}
}
```

**Key aspects:**
- Configuration-driven worker usage via `ipynb.experimental.serialization`
- Lazy initialization of worker thread
- UUID-based task tracking for async message passing
- Event listeners for worker lifecycle (exit, message, error)
- Graceful fallback to main thread when worker unavailable

**Variations / call-sites:**
- `extensions/ipynb/src/notebookSerializer.web.ts:10-85` - Web implementation using browser `Worker` instead of `node:worker_threads`

---

## Pattern 3: Worker Thread Message Passing Protocol

**Where:** `extensions/ipynb/src/notebookSerializerWorker.ts:1-19`

**What:** Minimal worker entry point that receives serialization tasks via message passing and returns results by ID.

```typescript
import { parentPort } from 'worker_threads';
import { serializeNotebookToString } from './serializers';
import type { NotebookData } from 'vscode';


if (parentPort) {
	parentPort.on('message', ({ id, data }: { id: string; data: NotebookData }) => {
		if (parentPort) {
			const json = serializeNotebookToString(data);
			const bytes = new TextEncoder().encode(json);
			parentPort.postMessage({ id, data: bytes });
		}
	});
}
```

**Key aspects:**
- Checks for `parentPort` availability (safety guard)
- Expects message format: `{ id: string; data: NotebookData }`
- Performs synchronous serialization work
- Returns result with matching `id` for correlation
- Minimal overhead - single responsibility

---

## Pattern 4: Serialization and Deserialization Delegation

**Where:** `extensions/ipynb/src/serializers.ts:462-482` and `extensions/ipynb/src/deserializers.ts:357-373`

**What:** Conversion functions that translate between VS Code NotebookData and Jupyter nbformat representations.

```typescript
// Serialization
export function serializeNotebookToString(data: NotebookData): string {
	const notebookContent = getNotebookMetadata(data);
	const preferredCellLanguage = notebookContent.metadata?.language_info?.name ?? data.cells.find(cell => cell.kind === 2)?.languageId;

	notebookContent.cells = data.cells
		.map(cell => createJupyterCellFromNotebookCell(cell, preferredCellLanguage))
		.map(pruneCell);

	const indentAmount = data.metadata && typeof data.metadata.indentAmount === 'string' ?
		data.metadata.indentAmount :
		' ';

	return serializeNotebookToJSON(notebookContent, indentAmount);
}

// Deserialization
export function jupyterNotebookModelToNotebookData(
	notebookContent: Partial<nbformat.INotebookContent>,
	preferredLanguage: string
): NotebookData {
	const notebookContentWithoutCells = { ...notebookContent, cells: [] };
	if (!Array.isArray(notebookContent.cells)) {
		throw new Error('Notebook content is missing cells');
	}

	const cells = notebookContent.cells
		.map(cell => createNotebookCellDataFromJupyterCell(preferredLanguage, cell))
		.filter((item): item is NotebookCellData => !!item);

	const notebookData = new NotebookData(cells);
	notebookData.metadata = notebookContentWithoutCells;
	return notebookData;
}
```

**Key aspects:**
- Bidirectional transformation between formats
- Uses cell-level helper functions for detailed conversion
- Preserves metadata through transformation
- Handles missing cells gracefully
- Maintains preferred language detection

**Variations / call-sites:**
- Cell creation helpers: `createJupyterCellFromNotebookCell`, `createNotebookCellDataFromJupyterCell`
- Output handling: `translateCellDisplayOutput`, `jupyterCellOutputToCellOutput`

---

## Pattern 5: Notebook Registration with Options

**Where:** `extensions/ipynb/src/ipynbMain.ts:35-67`

**What:** Extension activation pattern that registers NotebookSerializer with specific content options.

```typescript
export function activate(context: vscode.ExtensionContext, serializer: vscode.NotebookSerializer) {
	keepNotebookModelStoreInSync(context);
	const notebookSerializerOptions: OptionsWithCellContentMetadata = {
		transientOutputs: false,
		transientDocumentMetadata: {
			cells: true,
			indentAmount: true
		},
		transientCellMetadata: {
			breakpointMargin: true,
			id: false,
			metadata: false,
			attachments: false
		},
		cellContentMetadata: {
			attachments: true
		}
	};
	context.subscriptions.push(vscode.workspace.registerNotebookSerializer('jupyter-notebook', serializer, notebookSerializerOptions));

	const interactiveSerializeOptions: OptionsWithCellContentMetadata = {
		transientOutputs: false,
		transientCellMetadata: {
			breakpointMargin: true,
			id: false,
			metadata: false,
			attachments: false
		},
		cellContentMetadata: {
			attachments: true
		}
	};
	context.subscriptions.push(vscode.workspace.registerNotebookSerializer('interactive', serializer, interactiveSerializeOptions));
```

**Key aspects:**
- Registers same serializer instance for multiple notebook types
- Precise control over transient vs. persistent metadata
- Attachment handling at cell content level
- Subscriptions pushed to context for lifecycle management
- Separate configurations for different notebook formats

---

## Pattern 6: DeferredPromise for Async Task Tracking

**Where:** `extensions/ipynb/src/helper.ts:216-260` (inferred from usage)

**What:** Deferred promise pattern for correlating async worker responses with pending requests.

```typescript
// Usage pattern in notebookSerializer.node.ts:
private tasks = new Map<string, DeferredPromise<Uint8Array>>();

private async serializeViaWorker(data: vscode.NotebookData): Promise<Uint8Array> {
	const worker = await this.startWorker();
	const id = generateUuid();

	const deferred = new DeferredPromise<Uint8Array>();
	this.tasks.set(id, deferred);
	worker.postMessage({ data, id });

	return deferred.p;
}

// Worker response handler:
this.worker.on('message', (result: { data: Uint8Array; id: string }) => {
	const task = this.tasks.get(result.id);
	if (task) {
		task.complete(result.data);
		this.tasks.delete(result.id);
	}
});
```

**Key aspects:**
- UUID-based correlation between request and response
- Pending tasks tracked in Map<id, promise>
- Deferred object resolves when worker responds
- Automatic cleanup after task completion
- Handles multi-threaded async patterns

---

## Pattern 7: Type-Safe Metadata Interfaces

**Where:** `extensions/ipynb/src/common.ts:12-100`

**What:** Strongly-typed interfaces for cell and output metadata with custom type guards.

```typescript
export interface CellOutputMetadata {
	metadata?: any;
	transient?: {
		display_id?: string;
	} & any;
	outputType: nbformat.OutputType | string;
	executionCount?: nbformat.IExecuteResult['ExecutionCount'];
	__isJson?: boolean;
}

export interface CellMetadata {
	id?: string;
	attachments?: nbformat.IAttachments;
	metadata?: Partial<nbformat.ICellMetadata> & { vscode?: { languageId?: string } };
	execution_count?: number | null;
}

// Type guard for discriminating union types
export function hasKey<T extends object, TKeys>(
	x: T,
	key: TKeys & MakeOptionalAndBool<T>
): x is FilterType<T, { [K in KeysOfUnionType<T> & keyof TKeys]: unknown }> {
	for (const k in key) {
		if (!(k in x)) {
			return false;
		}
	}
	return true;
}
```

**Key aspects:**
- Extends Jupyter nbformat types with VS Code-specific metadata
- Custom `vscode` property for language overrides
- Transient display tracking for output updates
- Advanced TypeScript generics for type-safe guards
- Used for discriminating between different metadata structures

---

## Test Patterns

**Found in:** `extensions/ipynb/src/test/serializers.test.ts:22-100`

```typescript
suite(`ipynb serializer`, () => {
	let disposables: vscode.Disposable[] = [];
	setup(() => {
		disposables = [];
	});
	teardown(async () => {
		disposables.forEach(d => d.dispose());
		disposables = [];
		sinon.restore();
	});

	const base64EncodedImage =
		'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mOUlZL6DwAB/wFSU1jVmgAAAABJRU5ErkJggg==';
	test('Deserialize', async () => {
		const cells: nbformat.ICell[] = [
			{
				cell_type: 'code',
				execution_count: 10,
				outputs: [],
				source: 'print(1)',
				metadata: {}
			},
			{
				cell_type: 'code',
				outputs: [],
				source: 'print(2)',
				metadata: {}
			},
			{
				cell_type: 'markdown',
				source: '# HEAD',
				metadata: {}
			}
		];
		const notebook = jupyterNotebookModelToNotebookData({ cells }, 'python');
		assert.ok(notebook);

		const expectedCodeCell = new vscode.NotebookCellData(vscode.NotebookCellKind.Code, 'print(1)', 'python');
		expectedCodeCell.outputs = [];
		expectedCodeCell.metadata = { execution_count: 10, metadata: {} };
		expectedCodeCell.executionSummary = { executionOrder: 10 };

		const expectedCodeCell2 = new vscode.NotebookCellData(vscode.NotebookCellKind.Code, 'print(2)', 'python');
		expectedCodeCell2.outputs = [];
		expectedCodeCell2.metadata = { execution_count: null, metadata: {} };
		expectedCodeCell2.executionSummary = {};

		const expectedMarkdownCell = new vscode.NotebookCellData(vscode.NotebookCellKind.Markup, '# HEAD', 'markdown');
		expectedMarkdownCell.outputs = [];
		expectedMarkdownCell.metadata = {
			metadata: {}
		};

		assert.deepStrictEqual(notebook.cells, [expectedCodeCell, expectedCodeCell2, expectedMarkdownCell]);
	});
});
```

**Test patterns used:**
- Setup/teardown for resource management and sinon restoration
- Direct construction of nbformat.ICell test data
- Deep equality assertions for complex nested structures
- Separate test cases for edge cases (cells without metadata)

---

## Key Architectural Patterns for Rust/Tauri Port

Based on the patterns found, a Rust/Tauri port would need to address:

1. **Abstract trait implementation**: Rust trait analogous to `NotebookSerializer` interface
2. **Platform-specific implementations**: Conditional compilation for Tauri (web-based), avoiding Node worker threads
3. **Async task correlation**: ID-based tracking mechanism in Tauri's invoke/listen pattern
4. **Format conversion**: Core serialization/deserialization logic (largely platform-independent)
5. **Type safety**: Rust's strong typing for metadata preservation
6. **Configuration management**: Tauri command system for configuration changes
7. **Worker threading**: Leverage Tauri's async/tokio runtime instead of Node's worker_threads
8. **Backup recovery**: File system operations via Tauri's API

---

## Related Files and Utilities

- `extensions/ipynb/src/notebookModelStoreSync.ts` - Notebook document change tracking
- `extensions/ipynb/src/notebookAttachmentCleaner.ts` - Attachment lifecycle management
- `extensions/ipynb/src/notebookImagePaste.ts` - Image paste/drop handling
- `extensions/ipynb/src/constants.ts` - Mime type definitions and defaults
- `extensions/ipynb/src/deserializers.ts` - Language mapping and output translation
- `extensions/ipynb/src/serializers.ts` - Cell/output serialization with JSON property sorting

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
