# File Locator: Jupyter Notebook Extensions (ipynb)

## Research Question
Porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust - specifically examining the Jupyter notebook extension for relevant APIs and patterns.

## Scope
`extensions/ipynb/` (25 files, ~4,925 LOC)

---

## Implementation

### Core Extension Activation & Serialization
- `extensions/ipynb/src/ipynbMain.ts` — Main extension entry point; registers NotebookSerializer API, handles cell commands, notebook metadata operations; demonstrates vscode.workspace.registerNotebookSerializer() pattern
- `extensions/ipynb/src/notebookSerializer.ts` — Abstract base class implementing vscode.NotebookSerializer interface; handles Jupyter notebook deserialization from Uint8Array and serialization of vscode.NotebookData
- `extensions/ipynb/src/notebookSerializer.node.ts` — Node.js platform-specific serializer implementation; extends NotebookSerializerBase
- `extensions/ipynb/src/notebookSerializer.web.ts` — Browser platform-specific serializer implementation; extends NotebookSerializerBase
- `extensions/ipynb/src/ipynbMain.node.ts` — Node entry point; instantiates NotebookSerializer.node and delegates to ipynbMain.activate()
- `extensions/ipynb/src/ipynbMain.browser.ts` — Browser entry point; instantiates NotebookSerializer.web and delegates to ipynbMain.activate()

### Notebook Data Processing
- `extensions/ipynb/src/serializers.ts` — Converts vscode.NotebookCellData to Jupyter nbformat cell structures (IRawCell, IMarkdownCell, ICodeCell); JSON serialization with alphabetical property sorting; cell metadata handling
- `extensions/ipynb/src/deserializers.ts` — Converts Jupyter nbformat notebook structures to vscode.NotebookData; handles kernel language mapping, MIME type ordering, cell output rendering
- `extensions/ipynb/src/notebookModelStoreSync.ts` — Synchronizes notebook model state with ipynb file; handles cell changes and metadata persistence; uses vscode.workspace.onDidChangeNotebookDocument event
- `extensions/ipynb/src/notebookSerializerWorker.ts` — Worker thread for notebook serialization (Node.js); receives NotebookData via parentPort and encodes to UTF-8 bytes
- `extensions/ipynb/src/notebookSerializerWorker.web.ts` — Browser-based worker stub for serialization

### Editing Features
- `extensions/ipynb/src/notebookImagePaste.ts` — Implements vscode.DocumentPasteEditProvider and vscode.DocumentDropEditProvider; enables paste-as-attachment for images in Jupyter markdown cells
- `extensions/ipynb/src/notebookAttachmentCleaner.ts` — vscode.CodeActionProvider for diagnosing and cleaning invalid image attachments; tracks attachment metadata and provides quick fixes

### Utilities & Configuration
- `extensions/ipynb/src/common.ts` — Type definitions (CellMetadata, CellOutputMetadata) matching nbformat structures; type guard utility hasKey() for narrowing union types
- `extensions/ipynb/src/constants.ts` — Default notebook format version (4.5), MIME type enumerations, notebook cell selectors, constants for worker thread environments
- `extensions/ipynb/src/helper.ts` — Utilities: deepClone, objectEquals, Delayer (debounce), generateUuid, DeferredPromise; code reused from VS Code core
- `extensions/ipynb/src/types.d.ts` — Type declarations (contents not provided in scope)

### Notebook Renderer
- `extensions/ipynb/notebook-src/cellAttachmentRenderer.ts` — Notebook renderer plugin (vscode-notebook-renderer API); extends markdown-it renderer to resolve attachment: URIs to base64-encoded image data

---

## Tests

- `extensions/ipynb/src/test/serializers.test.ts` — Unit tests for deserializer and serializer functions; tests Jupyter cell-to-NotebookCellData conversion, output handling, metadata preservation
- `extensions/ipynb/src/test/notebookModelStoreSync.test.ts` — Tests for notebook model synchronization logic
- `extensions/ipynb/src/test/clearOutputs.test.ts` — Tests for cell output cleanup
- `extensions/ipynb/src/test/index.ts` — Test suite entry point

---

## Types / Interfaces

- `extensions/ipynb/src/common.ts` — CellMetadata, CellOutputMetadata interfaces (match Jupyter nbformat structures)
- `extensions/ipynb/src/notebookSerializer.ts` — Implements vscode.NotebookSerializer with deserializeNotebook() and serializeNotebook() methods
- `extensions/ipynb/src/notebookAttachmentCleaner.ts` — AttachmentCleaner class implements vscode.CodeActionProvider
- `extensions/ipynb/src/notebookImagePaste.ts` — DropOrPasteEditProvider implements both vscode.DocumentPasteEditProvider and vscode.DocumentDropEditProvider

---

## Configuration

- `extensions/ipynb/package.json` — VS Code extension manifest; defines:
  - Activation events: `onNotebook:jupyter-notebook`, `onNotebookSerializer:interactive`, `onNotebookSerializer:repl`
  - Contributes: notebook type "jupyter-notebook", notebook renderer "vscode.markdown-it-cell-attachment-renderer"
  - Configuration properties: `ipynb.pasteImagesAsAttachments.enabled`, `ipynb.experimental.serialization`
  - Commands: newUntitledIpynb, openIpynbInNotebookEditor, cleanInvalidImageAttachment, cellOutput.copy/addToChat/openInTextEditor
  - Menu contributions: File > New File, Command Palette, Webview context menus
  - Version: 10.0.0, requires vscode >=1.57.0

- `extensions/ipynb/tsconfig.json` — TypeScript config for Node.js builds; targets ES2024, includes vscode type definitions
- `extensions/ipynb/tsconfig.browser.json` — TypeScript config for browser/web builds

- `extensions/ipynb/.vscode/launch.json` — Debug configuration for extension development

---

## Examples / Fixtures

- `extensions/ipynb/package-lock.json` — Dependency lock file
- `extensions/ipynb/package.nls.json` — Localization strings for UI text

---

## Documentation

- `extensions/ipynb/README.md` — Brief description; notes bundled nature and support for .ipynb file opening/editing

---

## Notable Clusters

### File Organization by Concern
1. **Platform Abstraction**: Dual implementations (node/web) for serializer and entry points; allows extension to run in desktop (Electron) and web (VS Code Web) environments
2. **Serialization Pipeline**: Clear separation between deserializers (Jupyter→VSCode), serializers (VSCode→Jupyter), and workers for async processing
3. **Edit Integration**: Image paste/drop and attachment cleanup demonstrates deep integration with vscode.DocumentPasteEditProvider, DocumentDropEditProvider, and CodeActionProvider APIs
4. **Metadata Alignment**: Uses vscode.NotebookDocument.metadata and vscode.NotebookCell.metadata to preserve Jupyter cell metadata (execution_count, kernel info, etc.)

### Key API Dependencies
- vscode.workspace.registerNotebookSerializer() — Core API for notebook file format support
- vscode.workspace.onDidChangeNotebookDocument — For tracking model changes
- vscode.NotebookCellData, NotebookData, NotebookCellOutput, NotebookCellOutputItem — Core notebook data structures
- vscode.DocumentPasteEditProvider, DocumentDropEditProvider — For rich paste/drop editing
- vscode.CodeActionProvider — For quick fixes (attachment cleanup)
- vscode.languages.registerCodeLensProvider — For cell-level commands (Open in Notebook Editor)

### Dependencies
- @jupyterlab/nbformat (^3.2.9) — Jupyter notebook format types and structures
- detect-indent (^6.0.0) — Preserves file indentation style
- markdown-it (12.2.3) — For rendering cell attachments in markdown
- @enonic/fnv-plus (^1.3.0) — FNV hash for backup file naming

---

## Relevance to Tauri/Rust Port

This extension demonstrates several aspects of VS Code's IDE architecture critical for a Tauri port:

1. **Extensibility Model**: Shows how VS Code's extension API surfaces notebook serialization, document editing, diagnostics, and code actions; any Tauri port must preserve these extension points.

2. **Multi-Platform Support**: Demonstrates dual Node.js/Web implementations suggesting VS Code's architecture handles both desktop (Electron) and web platforms; Tauri would need equivalent platform abstraction.

3. **Data Serialization**: Illustrates complex bidirectional conversion between external formats (Jupyter's nbformat JSON) and VS Code's internal data models (NotebookData, NotebookCell); any port must support similar serialization patterns.

4. **Notebook Protocol**: Uses vscode.workspace.registerNotebookSerializer() which abstracts file format handling; a Rust port would need equivalent protocol for custom document types beyond plain text.

5. **Rich Editing Features**: Integration with paste/drop providers and code actions shows VS Code's event-driven editing architecture; Rust implementation would need IPC mechanisms for similar interactivity.

6. **Worker Threads**: NotebookSerializerWorker demonstrates off-main-thread processing; relevant for Tauri's threading model and Rust async/await patterns.

