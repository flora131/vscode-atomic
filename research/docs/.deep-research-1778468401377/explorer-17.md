# Partition 17 of 80 — Findings

## Scope
`extensions/ipynb/` (25 files, 4,925 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Notebook Extension Patterns (Partition 17/80)

## Overview
This document catalogs concrete code patterns from `extensions/ipynb/` (25 files, ~4,925 LOC) focused on the NotebookSerializer, NotebookCell, and NotebookDocument API surface that would require porting from TypeScript/Electron to Tauri/Rust.

---

## Pattern 1: NotebookSerializer Registration & Activation
**Where:** `extensions/ipynb/src/ipynbMain.ts:35-67`
**What:** Core extension activation pattern registering serializers for notebook types with content options.

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
}
```

**Variations / call-sites:** 
- `ipynbMain.ts:53` - jupyter-notebook registration
- `ipynbMain.ts:67` - interactive notebook registration
- `notebookSerializer.web.ts:11-19` - configuration-driven serialization strategy

---

## Pattern 2: NotebookSerializer Interface Implementation
**Where:** `extensions/ipynb/src/notebookSerializer.ts:13-87`
**What:** Abstract base class extending vscode.Disposable and implementing vscode.NotebookSerializer interface with async deserialize/serialize.

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

**Variations / call-sites:**
- `notebookSerializer.node.ts` - Node.js-specific implementation
- `notebookSerializer.web.ts` - Web/browser-specific implementation with worker thread support

---

## Pattern 3: Cell Data Transformation - Code Cell
**Where:** `extensions/ipynb/src/serializers.ts:87-114`
**What:** Creates Jupyter ICodeCell from VS Code NotebookCellData, managing execution counts, language IDs, and outputs.

```typescript
function createCodeCellFromNotebookCell(cell: NotebookCellData, preferredLanguage: string | undefined): nbformat.ICodeCell {
	const cellMetadata: CellMetadata = JSON.parse(JSON.stringify(getCellMetadata({ cell })));
	cellMetadata.metadata = cellMetadata.metadata || {}; // This cannot be empty.
	if (cell.languageId !== preferredLanguage) {
		setVSCodeCellLanguageId(cellMetadata, cell.languageId);
	} else {
		removeVSCodeCellLanguageId(cellMetadata);
	}

	const codeCell: nbformat.ICodeCell = {
		cell_type: 'code',
		execution_count: cellMetadata.execution_count ?? null,
		source: splitCellSourceIntoMultilineString(cell.value),
		outputs: (cell.outputs || []).map(translateCellDisplayOutput),
		metadata: cellMetadata.metadata
	};
	if (cellMetadata?.id) {
		codeCell.id = cellMetadata.id;
	}
	return codeCell;
}
```

**Variations / call-sites:**
- `serializers.ts:18-26` - Dispatches to different cell type creators
- `serializers.ts:116-130` - Raw cell creation
- `serializers.ts:376-390` - Markdown cell creation

---

## Pattern 4: Output Translation - Type Dispatch Pattern
**Where:** `extensions/ipynb/src/serializers.ts:161-264`
**What:** Pattern-matches on output type with switch statement, translating between Jupyter and VS Code representations.

```typescript
function translateCellDisplayOutput(output: NotebookCellOutput): JupyterOutput {
	const customMetadata = output.metadata as CellOutputMetadata | undefined;
	let result: JupyterOutput;
	const outputType = customMetadata?.outputType as nbformat.OutputType;
	switch (outputType) {
		case 'error': {
			result = translateCellErrorOutput(output);
			break;
		}
		case 'stream': {
			result = convertStreamOutput(output);
			break;
		}
		case 'display_data': {
			result = {
				output_type: 'display_data',
				data: output.items.reduce((prev: any, curr) => {
					prev[curr.mime] = convertOutputMimeToJupyterOutput(curr.mime, curr.data as Uint8Array);
					return prev;
				}, {}),
				metadata: customMetadata?.metadata || {}
			};
			break;
		}
		case 'execute_result': {
			result = {
				output_type: 'execute_result',
				data: output.items.reduce((prev: any, curr) => {
					prev[curr.mime] = convertOutputMimeToJupyterOutput(curr.mime, curr.data as Uint8Array);
					return prev;
				}, {}),
				metadata: customMetadata?.metadata || {},
				execution_count: typeof customMetadata?.executionCount === 'number' ? customMetadata?.executionCount : null
			};
			break;
		}
		default: {
			// Fallback handling for unknown types
		}
	}
	if (result && customMetadata && customMetadata.transient) {
		result.transient = customMetadata.transient;
	}
	return result;
}
```

**Variations / call-sites:**
- `serializers.ts:266-290` - Error output translation
- `serializers.ts:308-342` - Stream output conversion
- `deserializers.ts:257-263` - Reverse mapper registration pattern

---

## Pattern 5: Notebook Document Change Event Handling & Metadata Sync
**Where:** `extensions/ipynb/src/notebookModelStoreSync.ts:26-95`
**What:** Debounced event handling for cell changes with pending update tracking via WeakMap.

```typescript
export const pendingNotebookCellModelUpdates = new WeakMap<NotebookDocument, Set<Thenable<void>>>();
export function activate(context: ExtensionContext) {
	workspace.onDidChangeNotebookDocument(onDidChangeNotebookCells, undefined, context.subscriptions);
	workspace.onWillSaveNotebookDocument(waitForPendingModelUpdates, undefined, context.subscriptions);
}

type NotebookDocumentChangeEventEx = Omit<NotebookDocumentChangeEvent, 'metadata'>;
let mergedEvents: NotebookDocumentChangeEventEx | undefined;
let timer: NodeJS.Timeout;

function triggerDebouncedNotebookDocumentChangeEvent() {
	if (timer) {
		clearTimeout(timer);
	}
	if (!mergedEvents) {
		return;
	}
	const args = mergedEvents;
	mergedEvents = undefined;
	onDidChangeNotebookCells(args);
}

export function debounceOnDidChangeNotebookDocument() {
	const disposable = workspace.onDidChangeNotebookDocument(e => {
		if (!isSupportedNotebook(e.notebook)) {
			return;
		}
		if (!mergedEvents) {
			mergedEvents = e;
		} else if (mergedEvents.notebook === e.notebook) {
			mergedEvents = {
				cellChanges: e.cellChanges.concat(mergedEvents.cellChanges),
				contentChanges: e.contentChanges.concat(mergedEvents.contentChanges),
				notebook: e.notebook
			};
		} else {
			triggerDebouncedNotebookDocumentChangeEvent();
			mergedEvents = e;
		}
		if (timer) {
			clearTimeout(timer);
		}
		timer = setTimeout(triggerDebouncedNotebookDocumentChangeEvent, 200);
	});

	return Disposable.from(disposable, new Disposable(() => {
		clearTimeout(timer);
	}));
}

function waitForPendingModelUpdates(e: NotebookDocumentWillSaveEvent) {
	if (!isSupportedNotebook(e.notebook)) {
		return;
	}
	triggerDebouncedNotebookDocumentChangeEvent();
	const promises = pendingNotebookCellModelUpdates.get(e.notebook);
	if (!promises) {
		return;
	}
	e.waitUntil(Promise.all(promises));
}
```

**Variations / call-sites:**
- `notebookModelStoreSync.ts:106-124` - Cell metadata update tracking
- `notebookModelStoreSync.ts:127-225` - Cell change processing with language/execution_count sync

---

## Pattern 6: Code Actions for Diagnostic Quick Fixes
**Where:** `extensions/ipynb/src/notebookAttachmentCleaner.ts:29-167`
**What:** CodeActionProvider implementation for attachment validation with diagnostic collection.

```typescript
export class AttachmentCleaner implements vscode.CodeActionProvider {
	private _attachmentCache: Map<string, Map<string, Map<string, IAttachmentData>>> = new Map();
	private _disposables: vscode.Disposable[];
	private _imageDiagnosticCollection: vscode.DiagnosticCollection;
	private readonly _delayer = new Delayer(750);

	constructor() {
		this._disposables = [];
		this._imageDiagnosticCollection = vscode.languages.createDiagnosticCollection('Notebook Image Attachment');
		this._disposables.push(this._imageDiagnosticCollection);

		this._disposables.push(vscode.commands.registerCommand(ATTACHMENT_CLEANUP_COMMANDID, async (document: vscode.Uri, range: vscode.Range) => {
			const workspaceEdit = new vscode.WorkspaceEdit();
			workspaceEdit.delete(document, range);
			await vscode.workspace.applyEdit(workspaceEdit);
		}));

		this._disposables.push(vscode.languages.registerCodeActionsProvider(JUPYTER_NOTEBOOK_MARKDOWN_SELECTOR, this, {
			providedCodeActionKinds: [vscode.CodeActionKind.QuickFix],
		}));

		this._disposables.push(vscode.workspace.onDidChangeNotebookDocument(e => {
			this._delayer.trigger(() => {
				e.cellChanges.forEach(change => {
					if (!change.document) {
						return;
					}
					if (change.cell.kind !== vscode.NotebookCellKind.Markup) {
						return;
					}
					const metadataEdit = this.cleanNotebookAttachments({
						notebook: e.notebook,
						cell: change.cell,
						document: change.document
					});
					if (metadataEdit) {
						const workspaceEdit = new vscode.WorkspaceEdit();
						workspaceEdit.set(e.notebook.uri, [metadataEdit]);
						vscode.workspace.applyEdit(workspaceEdit);
					}
				});
			});
		}));
	}

	provideCodeActions(document: vscode.TextDocument, _range: vscode.Range | vscode.Selection, context: vscode.CodeActionContext, _token: vscode.CancellationToken): vscode.ProviderResult<(vscode.CodeAction | vscode.Command)[]> {
		const fixes: vscode.CodeAction[] = [];

		for (const diagnostic of context.diagnostics) {
			switch (diagnostic.code) {
				case DiagnosticCode.missing_attachment:
					{
						const fix = new vscode.CodeAction(
							'Remove invalid image attachment reference',
							vscode.CodeActionKind.QuickFix);

						fix.command = {
							command: ATTACHMENT_CLEANUP_COMMANDID,
							title: 'Remove invalid image attachment reference',
							arguments: [document.uri, diagnostic.range],
						};
						fixes.push(fix);
					}
					break;
			}
		}

		return fixes;
	}
}
```

**Variations / call-sites:**
- `notebookAttachmentCleaner.ts:174-240` - Cell attachment cleaning logic
- `notebookAttachmentCleaner.ts:282-293` - Diagnostic collection update

---

## Pattern 7: Document Drop/Paste Edit Provider
**Where:** `extensions/ipynb/src/notebookImagePaste.ts:49-134`
**What:** Unified provider for paste and drop events with snippet generation and metadata updates.

```typescript
class DropOrPasteEditProvider implements vscode.DocumentPasteEditProvider, vscode.DocumentDropEditProvider {

	public static readonly kind = vscode.DocumentDropOrPasteEditKind.Empty.append('markdown', 'link', 'image', 'attachment');

	async provideDocumentPasteEdits(
		document: vscode.TextDocument,
		_ranges: readonly vscode.Range[],
		dataTransfer: vscode.DataTransfer,
		_context: vscode.DocumentPasteEditContext,
		token: vscode.CancellationToken,
	): Promise<vscode.DocumentPasteEdit[] | undefined> {
		const enabled = vscode.workspace.getConfiguration('ipynb', document).get('pasteImagesAsAttachments.enabled', true);
		if (!enabled) {
			return;
		}

		const insert = await this.createInsertImageAttachmentEdit(document, dataTransfer, token);
		if (!insert) {
			return;
		}

		const pasteEdit = new vscode.DocumentPasteEdit(insert.insertText, vscode.l10n.t('Insert Image as Attachment'), DropOrPasteEditProvider.kind);
		pasteEdit.yieldTo = [vscode.DocumentDropOrPasteEditKind.Text];
		pasteEdit.additionalEdit = insert.additionalEdit;
		return [pasteEdit];
	}

	async provideDocumentDropEdits(
		document: vscode.TextDocument,
		_position: vscode.Position,
		dataTransfer: vscode.DataTransfer,
		token: vscode.CancellationToken,
	): Promise<vscode.DocumentDropEdit | undefined> {
		const insert = await this.createInsertImageAttachmentEdit(document, dataTransfer, token);
		if (!insert) {
			return;
		}

		const dropEdit = new vscode.DocumentDropEdit(insert.insertText);
		dropEdit.yieldTo = [vscode.DocumentDropOrPasteEditKind.Text];
		dropEdit.additionalEdit = insert.additionalEdit;
		dropEdit.title = vscode.l10n.t('Insert Image as Attachment');
		return dropEdit;
	}

	private async createInsertImageAttachmentEdit(
		document: vscode.TextDocument,
		dataTransfer: vscode.DataTransfer,
		token: vscode.CancellationToken,
	): Promise<{ insertText: vscode.SnippetString; additionalEdit: vscode.WorkspaceEdit } | undefined> {
		const imageData = await getDroppedImageData(dataTransfer, token);
		if (!imageData.length || token.isCancellationRequested) {
			return;
		}

		const currentCell = getCellFromCellDocument(document);
		if (!currentCell) {
			return undefined;
		}

		const newAttachment = buildAttachment(currentCell, imageData);
		if (!newAttachment) {
			return;
		}

		const additionalEdit = new vscode.WorkspaceEdit();
		const nbEdit = vscode.NotebookEdit.updateCellMetadata(currentCell.index, newAttachment.metadata);
		const notebookUri = currentCell.notebook.uri;
		additionalEdit.set(notebookUri, [nbEdit]);

		const insertText = new vscode.SnippetString();
		newAttachment.filenames.forEach((filename, i) => {
			insertText.appendText('![');
			insertText.appendPlaceholder(`${filename}`);
			insertText.appendText(`](${/\s/.test(filename) ? `<attachment:${filename}>` : `attachment:${filename}`})`);
			if (i !== newAttachment.filenames.length - 1) {
				insertText.appendText(' ');
			}
		});

		return { insertText, additionalEdit };
	}
}

export function notebookImagePasteSetup(): vscode.Disposable {
	const provider = new DropOrPasteEditProvider();
	return vscode.Disposable.from(
		vscode.languages.registerDocumentPasteEditProvider(JUPYTER_NOTEBOOK_MARKDOWN_SELECTOR, provider, {
			providedPasteEditKinds: [DropOrPasteEditProvider.kind],
			pasteMimeTypes: [MimeType.png, MimeType.uriList],
		}),
		vscode.languages.registerDocumentDropEditProvider(JUPYTER_NOTEBOOK_MARKDOWN_SELECTOR, provider, {
			providedDropEditKinds: [DropOrPasteEditProvider.kind],
			dropMimeTypes: [...Object.values(imageExtToMime), MimeType.uriList],
		})
	);
}
```

**Variations / call-sites:**
- `notebookImagePaste.ts:136-189` - Image data extraction from DataTransfer
- `notebookImagePaste.ts:256-297` - Attachment metadata building with base64 encoding

---

## Pattern 8: Bidirectional Data Conversion - Jupyter to NotebookData
**Where:** `extensions/ipynb/src/deserializers.ts:314-373`
**What:** Cell type dispatcher with language detection and output translation.

```typescript
function createNotebookCellDataFromCodeCell(cell: nbformat.ICodeCell, cellLanguage: string): NotebookCellData {
	const cellOutputs = Array.isArray(cell.outputs) ? cell.outputs : [];
	const outputs = cellOutputs.map(jupyterCellOutputToCellOutput);
	const hasExecutionCount = typeof cell.execution_count === 'number' && cell.execution_count > 0;

	const source = concatMultilineCellSource(cell.source);

	const executionSummary: NotebookCellExecutionSummary = hasExecutionCount
		? { executionOrder: cell.execution_count as number }
		: {};

	const vscodeCustomMetadata = cell.metadata?.['vscode'] as { [key: string]: any } | undefined;
	const cellLanguageId = vscodeCustomMetadata && vscodeCustomMetadata.languageId && typeof vscodeCustomMetadata.languageId === 'string' ? vscodeCustomMetadata.languageId : cellLanguage;
	const cellData = new NotebookCellData(NotebookCellKind.Code, source, cellLanguageId);

	cellData.outputs = outputs;
	cellData.metadata = getNotebookCellMetadata(cell);
	cellData.executionSummary = executionSummary;
	return cellData;
}

function createNotebookCellDataFromJupyterCell(
	cellLanguage: string,
	cell: nbformat.IBaseCell
): NotebookCellData | undefined {
	switch (cell.cell_type) {
		case 'raw': {
			return createNotebookCellDataFromRawCell(cell as nbformat.IRawCell);
		}
		case 'markdown': {
			return createNotebookCellDataFromMarkdownCell(cell as nbformat.IMarkdownCell);
		}
		case 'code': {
			return createNotebookCellDataFromCodeCell(cell as nbformat.ICodeCell, cellLanguage);
		}
	}

	return;
}

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

**Variations / call-sites:**
- `deserializers.ts:298-312` - Raw/Markdown cell creation
- `deserializers.ts:153-179` - Notebook cell metadata extraction

---

## Pattern 9: Cell ID Generation and Nbformat Version Checking
**Where:** `extensions/ipynb/src/notebookModelStoreSync.ts:228-262`
**What:** Version-gated cell ID generation with uniqueness validation.

```typescript
function isCellIdRequired(metadata: Pick<Partial<nbformat.INotebookContent>, 'nbformat' | 'nbformat_minor'>) {
	if ((metadata.nbformat || 0) >= 5) {
		return true;
	}
	if ((metadata.nbformat || 0) === 4 && (metadata.nbformat_minor || 0) >= 5) {
		return true;
	}
	return false;
}

function generateCellId(notebook: NotebookDocument) {
	while (true) {
		const id = generateUuid().replace(/-/g, '').substring(0, 8);
		let duplicate = false;
		for (let index = 0; index < notebook.cellCount; index++) {
			const cell = notebook.cellAt(index);
			const existingId = getCellMetadata({ cell })?.id;
			if (!existingId) {
				continue;
			}
			if (existingId === id) {
				duplicate = true;
				break;
			}
		}
		if (!duplicate) {
			return id;
		}
	}
}
```

**Variations / call-sites:**
- `notebookModelStoreSync.ts:193-220` - New cell ID assignment during content changes
- `deserializers.ts:20-33` - Preferred language detection from nbformat metadata

---

## Pattern 10: Output MIME Type Ordering and Rendering Priority
**Where:** `extensions/ipynb/src/deserializers.ts:43-91`
**What:** Declarative MIME type priority list with pattern matching for rendering preference.

```typescript
const orderOfMimeTypes = [
	'application/vnd.*',
	'application/vdom.*',
	'application/geo+json',
	'application/x-nteract-model-debug+json',
	'text/html',
	'application/javascript',
	'image/gif',
	'text/latex',
	'text/markdown',
	'image/png',
	'image/svg+xml',
	'image/jpeg',
	'application/json',
	'text/plain'
];

function isMimeTypeMatch(value: string, compareWith: string) {
	if (value.endsWith('.*')) {
		value = value.substr(0, value.indexOf('.*'));
	}
	return compareWith.startsWith(value);
}

function sortOutputItemsBasedOnDisplayOrder(outputItems: NotebookCellOutputItem[]): NotebookCellOutputItem[] {
	return outputItems
		.map(item => {
			let index = orderOfMimeTypes.findIndex((mime) => isMimeTypeMatch(mime, item.mime));
			if (isEmptyVendoredMimeType(item)) {
				index = -1;
			}
			index = index === -1 ? 100 : index;
			return {
				item, index
			};
		})
		.sort((outputItemA, outputItemB) => outputItemA.index - outputItemB.index).map(item => item.item);
}
```

**Variations / call-sites:**
- `deserializers.ts:119-151` - MIME-aware output buffer conversion
- `deserializers.ts:206-235` - Display data translation with items sorting

---

## Summary

### 7 Distinct Patterns Identified:

1. **NotebookSerializer Registration** - Extension activation with type-specific serializers
2. **Serializer Interface Implementation** - Abstract base with dual async methods
3. **Cell Data Transformation** - Language ID and execution state management
4. **Output Type Dispatch** - Switch-based MIME conversion
5. **Document Change Event Handling** - Debounced WeakMap-based metadata sync
6. **Code Action Providers** - Diagnostic quick-fix patterns
7. **Drop/Paste Edit Providers** - DataTransfer handling with snippet generation
8. **Bidirectional Conversion** - Jupyter ↔ VS Code with language detection
9. **Cell ID Generation** - Version-gated UUIDs with deduplication
10. **MIME Type Ordering** - Priority-based output rendering

### Core APIs Required for Porting:

- `vscode.workspace.registerNotebookSerializer(type, serializer, options)`
- `vscode.workspace.onDidChangeNotebookDocument(handler)`
- `vscode.workspace.onWillSaveNotebookDocument(handler)`
- `NotebookEdit.updateCellMetadata(index, metadata)`
- `vscode.languages.registerCodeActionsProvider(selector, provider)`
- `vscode.languages.registerDocumentPasteEditProvider(selector, provider)`
- `vscode.languages.registerDocumentDropEditProvider(selector, provider)`
- `NotebookData`, `NotebookCellData`, `NotebookCellOutput`, `NotebookCellOutputItem`
- `WorkspaceEdit` with notebook URI support
- `DataTransfer` API for clipboard/drag-drop

### Architecture Notes:

- **Platform abstraction**: `.node.ts` vs `.web.ts` implementations show environment-specific serialization strategies
- **Debouncing pattern**: Uses `setTimeout` with event merging to batch metadata updates
- **WeakMap usage**: Prevents memory leaks for pending updates tied to notebook documents
- **Binary data handling**: Base64 encoding for images, Uint8Array for output items
- **Metadata layering**: Cells store both Jupyter metadata and VS Code extensions metadata (`vscode.languageId`)

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
