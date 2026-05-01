# Partition 11 of 79 — Findings

## Scope
`extensions/vscode-api-tests/` (50 files, 11,425 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator Report: Partition 11 — vscode-api-tests Extension

## Research Question
What it would take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

## Scope Summary
- **Directory**: `extensions/vscode-api-tests/` (75 total files, ~13,958 LOC)
- **Focus**: Extension API test suite that exercises core IDE surfaces

## Key Finding
This partition is a comprehensive integration test extension that validates VS Code's public extension API across all major IDE features. It exercises 40+ distinct API modules through 38 test files organized by feature domain. The test corpus directly maps to core IDE functionality that would need to be ported.

---

## Implementation Files

### Test Infrastructure
- `extensions/vscode-api-tests/src/extension.ts` — Entry point; activates and exposes ExtensionContext as global for tests
- `extensions/vscode-api-tests/src/utils.ts` — Common test utilities (file creation, editor management, logging, polling helpers)
- `extensions/vscode-api-tests/src/memfs.ts` — In-memory filesystem provider (implements vscode.FileSystemProvider interface)
- `extensions/vscode-api-tests/src/singlefolder-tests/index.ts` — Test runner configuration for Mocha; handles multiple execution environments (Electron, Web, Remote)
- `extensions/vscode-api-tests/src/workspace-tests/index.ts` — Workspace-scoped test runner configuration

---

## Tests

### Editor & Document Management
- `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts` — Editor surface: snippets, selections, cursor styles, line numbers, clipboard integration
- `extensions/vscode-api-tests/src/singlefolder-tests/documentPaste.test.ts` — Document paste handlers and paste content providers
- `extensions/vscode-api-tests/src/singlefolder-tests/types.test.ts` — Type system (Position, Range, Selection, Uri, etc.)

### Language Intelligence & Diagnostics
- `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts` — Language APIs: setTextDocumentLanguage, diagnostics, document language associations
- `extensions/vscode-api-tests/src/singlefolder-tests/languagedetection.test.ts` — Automatic language detection/inference
- `extensions/vscode-api-tests/src/singlefolder-tests/lm.test.ts` — Language model integration (AI chat/completions)
- `extensions/vscode-api-tests/src/singlefolder-tests/chat.test.ts` — Chat participant APIs, chat requests, context handling
- `extensions/vscode-api-tests/src/singlefolder-tests/chat.runInTerminal.test.ts` — Chat tool execution in terminal context

### Debug & Breakpoints
- `extensions/vscode-api-tests/src/singlefolder-tests/debug.test.ts` — Debug API: breakpoints, debug sessions, debug adapter protocol tracking

### Terminal & Shell Integration
- `extensions/vscode-api-tests/src/singlefolder-tests/terminal.test.ts` — Terminal creation, PTY terminals, environment variables, terminal dimensions, exit handling
- `extensions/vscode-api-tests/src/singlefolder-tests/terminal.shellIntegration.test.ts` — Shell integration, command tracking, terminal execution events

### File System & Workspace Navigation
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.fs.test.ts` — Workspace file system APIs (openTextDocument, findFiles, file watching)
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.watcher.test.ts` — File system watchers and change events
- `extensions/vscode-api-tests/src/singlefolder-tests/readonlyFileSystem.test.ts` — Read-only filesystem provider registration
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.test.ts` — Workspace structure (rootPath, workspaceFolders, workspaceFile)
- `extensions/vscode-api-tests/src/workspace-tests/workspace.test.ts` — Multi-folder workspace configuration

### Source Control Integration (Not Found)
- Note: SCM APIs are NOT exercised in this partition's test files. No `scm.test.ts` file exists.

### Notebook/Interactive Computing
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.api.test.ts` — Notebook API surface (lifecycle, kernels)
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.document.test.ts` — Notebook document model and cell editing
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.editor.test.ts` — Notebook editor surface
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.kernel.test.ts` — Notebook kernel execution and communication
- `extensions/vscode-api-tests/src/singlefolder-tests/ipynb.test.ts` — Jupyter notebook serialization
- `extensions/vscode-api-tests/src/singlefolder-tests/interactiveWindow.test.ts` — Interactive window/REPL surface

### Tasks & Commands
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.tasks.test.ts` — Task API: ShellExecution, CustomExecution, task presentation
- `extensions/vscode-api-tests/src/singlefolder-tests/commands.test.ts` — Command registration and execution

### UI & Window Management
- `extensions/vscode-api-tests/src/singlefolder-tests/window.test.ts` — Window API: dialogs, messages, progress, quick input
- `extensions/vscode-api-tests/src/singlefolder-tests/quickInput.test.ts` — Quick pick and input box
- `extensions/vscode-api-tests/src/singlefolder-tests/tree.test.ts` — Tree view and custom view providers

### Configuration & State Management
- `extensions/vscode-api-tests/src/singlefolder-tests/configuration.test.ts` — Settings/configuration API
- `extensions/vscode-api-tests/src/singlefolder-tests/state.test.ts` — Extension context state (globalState, workspaceState)

### Environment & Extensions
- `extensions/vscode-api-tests/src/singlefolder-tests/env.test.ts` — Environment variables, UIKind detection, extension API introspection
- `extensions/vscode-api-tests/src/singlefolder-tests/env.power.test.ts` — Power management/battery status APIs
- `extensions/vscode-api-tests/src/singlefolder-tests/extensions.test.ts` — Extension activation and lifecycle

### Browser & Platform-Specific
- `extensions/vscode-api-tests/src/singlefolder-tests/browser.test.ts` — Browser environment detection and behavior
- `extensions/vscode-api-tests/src/singlefolder-tests/browser.cdp.test.ts` — Chrome DevTools Protocol integration
- `extensions/vscode-api-tests/src/singlefolder-tests/browser.tools.test.ts` — Browser developer tools

### Advanced/Experimental Features
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.event.test.ts` — Workspace event emissions and ordering
- `extensions/vscode-api-tests/src/singlefolder-tests/module.test.ts` — Module/require system and API versioning
- `extensions/vscode-api-tests/src/singlefolder-tests/rpc.test.ts` — RPC communication protocol
- `extensions/vscode-api-tests/src/singlefolder-tests/proxy.test.ts` — Proxy/network configuration

---

## Types / Interfaces

- `extensions/vscode-api-tests/src/singlefolder-tests/types.test.ts` — Tests for built-in types (no separate .d.ts file in this extension)

---

## Configuration

- `extensions/vscode-api-tests/package.json` — Extension manifest with:
  - 62 enabledApiProposals (experimental/unstable features)
  - Language model chat providers (test vendors)
  - Chat participant definitions
  - Notebook type definitions (notebookCoreTest, nbdtest, nbdserializer)
  - Task definitions (custombuildscript)
  - Debug configuration (mock debugger)
  - Status bar items
  - Configuration schema (farboo.* properties for testing)

- `extensions/vscode-api-tests/.vscode/launch.json` — Debug launch configuration
- `extensions/vscode-api-tests/.vscode/tasks.json` — VS Code tasks for test running
- `extensions/vscode-api-tests/tsconfig.json` — TypeScript compilation configuration

---

## Examples / Fixtures

### Test Workspace
- `extensions/vscode-api-tests/testWorkspace/` — Primary workspace for integration tests:
  - `10linefile.ts`, `30linefile.ts`, `myFile.ts` — Source code fixtures
  - `index.html`, `simple.txt`, `lorem.txt` — Text document fixtures
  - `debug.js`, `far.js`, `worker.js` — Script fixtures for debug/execution tests
  - `test.ipynb` — Jupyter notebook fixture
  - `.vscode/settings.json`, `.vscode/launch.json` — Workspace configuration
  - `files-exclude/`, `search-exclude/` — Exclusion pattern test directories
  - `sub/image.png`, `image*.png` — Binary/image fixtures for file system tests
  - `bower.json` — Dependency manifest fixture

- `extensions/vscode-api-tests/testWorkspace2/` — Secondary workspace for multi-folder tests
  - `.vscode/settings.json` — Separate workspace settings

- `extensions/vscode-api-tests/testworkspace.code-workspace` — Multi-root workspace file

---

## Notable Clusters

### API Module Coverage by Suite
The test suite organizes tests into 38 distinct test modules (suites), each exercising a specific vscode.* namespace or feature:

**Core Editor & Document APIs** (5 files)
- editor, types, documentPaste, languages, editor

**Terminal & Shell** (2 files)
- terminal, terminal.shellIntegration

**Debugging** (1 file)
- debug

**Language Intelligence** (3 files)
- languages, languagedetection, lm

**Chat & AI** (2 files)
- chat, chat.runInTerminal

**Notebooks** (6 files)
- notebook.api, notebook.document, notebook.editor, notebook.kernel, ipynb, interactiveWindow

**File System & Workspace** (7 files)
- workspace, workspace.fs, workspace.watcher, workspace.event, workspace.tasks, readonlyFileSystem

**UI Components** (3 files)
- window, quickInput, tree

**Configuration & State** (2 files)
- configuration, state

**Commands** (1 file)
- commands

**Environment** (3 files)
- env, env.power, extensions

**Browser** (3 files)
- browser, browser.cdp, browser.tools

**Advanced** (3 files)
- module, rpc, proxy

### Extension API Proposals Tested
The `package.json` lists 62 `enabledApiProposals`, indicating this extension exercises experimental/upcoming API surfaces including:
- Chat-related: chatParticipantPrivate, chatProvider, chatPromptFiles, defaultChatParticipant
- Notebooks: notebookDeprecated, notebookLiveShare, notebookMessaging, notebookMime
- Terminal: terminalDataWriteEvent, terminalDimensions
- File system: fileSearchProvider, findFiles2, findTextInFiles, fsChunks
- Terminal: terminalDataWriteEvent, terminalDimensions
- SCM: scmActionButton, scmSelectedProvider, scmTextDocument, scmValidation
- Debug/Test: testObserver, textSearchProvider
- Workspace: workspaceTrust
- Language models: languageModelProxy, inlineCompletionsAdditions

### Gap: No Source Control Testing
Despite SCM APIs being enabled in proposals (scmActionButton, scmSelectedProvider, scmTextDocument, scmValidation), **no source control test file exists**. This is a gap in coverage of a stated core IDE feature.

---

## Documentation

- No dedicated README or markdown documentation exists within this partition. Test documentation is embedded in:
  - Test file comments (/// remarks explaining behavior)
  - package.json manifest descriptions
  - Configuration schema comments

---

## Relevance to Research Question

This partition is **highly relevant** to porting VS Code to Tauri/Rust. It directly maps the extension-facing API surface that any IDE built on VS Code's architecture would need to support:

### What Exists (API Contracts to Port)
1. **Editor Surface**: Text editing, snippets, selections, clipboard
2. **Terminal Integration**: Process creation, PTY management, shell integration
3. **Debugging**: Breakpoint management, debug adapter protocol
4. **Language Services**: Syntax highlighting, diagnostics, language detection
5. **File System Abstraction**: Virtual filesystem providers, watchers, glob patterns
6. **Workspace Model**: Multi-folder workspaces, folder-aware configuration
7. **UI Primitives**: Dialogs, quick input, tree views, status bar
8. **Configuration Management**: Settings, state persistence (global/workspace)
9. **Task Execution**: Shell and custom task runners
10. **Notebook/REPL**: Notebook documents, kernel execution, cell evaluation

### What's Missing (Known Gaps)
- **Source Control**: Despite enabled SCM proposals, zero test coverage for SCM APIs
- **Custom Editors**: Not explicitly tested
- **WebView Integration**: Not found in this partition
- **Settings Sync**: Not found in this partition

### Core Porting Challenges Revealed
1. **RPC Layer**: `rpc.test.ts` shows VS Code uses internal RPC for cross-boundary communication
2. **Browser/Desktop Dual Path**: `browser.*.test.ts` files indicate API must work in both Electron and Web contexts
3. **Complex State Management**: Global vs workspace-scoped state requires distributed consistency
4. **Terminal PTY Complexity**: Direct PTY creation and environment variable mutation (EnvironmentVariableMutator)
5. **File System Abstraction Depth**: Custom filesystem providers with full CRUD semantics
6. **Notebook Kernel Protocol**: Multi-step kernel communication and stream handling

---

## Statistics

- **Total Files**: 75 (50 .ts files, 3 .js files, multiple config/fixture files)
- **Total LOC**: ~13,958
- **Test Files**: 38 distinct `*.test.ts` modules
- **Test Suites Defined**: 40+ suites (some files have nested suites)
- **API Modules Exercised**: 40+ distinct vscode.* namespaces
- **Enabled API Proposals**: 62 experimental features

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code API Test Patterns: Core IDE Functionality

This document catalogs concrete code patterns from the vscode-api-tests extension scope (50 files, 11,425 LOC). These patterns demonstrate how VS Code's IDE functionality is currently used across editing, language intelligence, debugging, source control, terminal, navigation, and other subsystems.

---

## Pattern 1: Test Suite Structure with API Initialization

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts:10-35`

**What:** Standard test suite pattern with setup/teardown for workspace and editor tests, ensuring RPC cleanup.

```typescript
suite('vscode API - editors', () => {

	teardown(async function () {
		assertNoRpc();
		await closeAllEditors();
	});

	function withRandomFileEditor(initialContents: string, run: (editor: TextEditor, doc: TextDocument) => Thenable<void>): Thenable<boolean> {
		return createRandomFile(initialContents).then(file => {
			return workspace.openTextDocument(file).then(doc => {
				return window.showTextDocument(doc).then((editor) => {
					return run(editor, doc).then(_ => {
						if (doc.isDirty) {
							return doc.save().then(saved => {
								assert.ok(saved);
								assert.ok(!doc.isDirty);
								return deleteFile(file);
							});
						} else {
							return deleteFile(file);
						}
					});
				});
			});
		});
	}
});
```

**Variations / call-sites:** All test files use `suite()` (e.g., `commands.test.ts:12`, `languages.test.ts:11`, `debug.test.ts:11`, `terminal.test.ts:12`, `workspace.test.ts:13`).

---

## Pattern 2: Window and Text Editor Manipulation

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts:37-51`

**What:** Editor API usage for inserting snippets and managing document state.

```typescript
test('insert snippet', () => {
	const snippetString = new SnippetString()
		.appendText('This is a ')
		.appendTabstop()
		.appendPlaceholder('placeholder')
		.appendText(' snippet');

	return withRandomFileEditor('', (editor, doc) => {
		return editor.insertSnippet(snippetString).then(inserted => {
			assert.ok(inserted);
			assert.strictEqual(doc.getText(), 'This is a placeholder snippet');
			assert.ok(doc.isDirty);
		});
	});
});
```

**Variations / call-sites:** `window.test.ts:19-52` (activeTextEditor, viewColumn), `editor.test.ts:79-95` (selection and replacement).

---

## Pattern 3: Language Intelligence via vscode.languages API

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts:29-62`

**What:** Language service registration and text document language changes with event handling.

```typescript
test('setTextDocumentLanguage -> close/open event', async function () {
	const file = await createRandomFile('foo\nbar\nbar');
	const doc = await vscode.workspace.openTextDocument(file);
	const langIdNow = doc.languageId;
	let clock = 0;
	const disposables: vscode.Disposable[] = [];

	const close = new Promise<void>(resolve => {
		disposables.push(vscode.workspace.onDidCloseTextDocument(e => {
			if (e === doc) {
				assert.strictEqual(doc.languageId, langIdNow);
				assert.strictEqual(clock, 0);
				clock += 1;
				resolve();
			}
		}));
	});
	const change = vscode.languages.setTextDocumentLanguage(doc, 'json');
	await Promise.all([change, close, open]);
	assert.strictEqual(clock, 2);
	assert.strictEqual(doc.languageId, 'json');
	disposables.forEach(disposable => disposable.dispose());
});
```

**Variations / call-sites:** `languages.test.ts:76-97` (diagnostics collection), `languages.test.ts:101-120` (DocumentLinkProvider registration).

---

## Pattern 4: Debugging API with Breakpoints and Sessions

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/debug.test.ts:28-53`

**What:** Debug breakpoint management and session tracking with lifecycle events.

```typescript
test('breakpoints', async function () {
	assert.strictEqual(debug.breakpoints.length, 0);
	let onDidChangeBreakpointsCounter = 0;
	const toDispose: Disposable[] = [];

	toDispose.push(debug.onDidChangeBreakpoints(() => {
		onDidChangeBreakpointsCounter++;
	}));

	debug.addBreakpoints([{ id: '1', enabled: true }, { id: '2', enabled: false, condition: '2 < 5' }]);
	assert.strictEqual(onDidChangeBreakpointsCounter, 1);
	assert.strictEqual(debug.breakpoints.length, 2);
	assert.strictEqual(debug.breakpoints[0].id, '1');
	assert.strictEqual(debug.breakpoints[1].condition, '2 < 5');

	debug.removeBreakpoints([{ id: '1', enabled: true }]);
	assert.strictEqual(onDidChangeBreakpointsCounter, 2);
	assert.strictEqual(debug.breakpoints.length, 1);
	disposeAll(toDispose);
});
```

**Variations / call-sites:** `debug.test.ts:55-64` (FunctionBreakpoint), `debug.test.ts:66-145` (debug.startDebugging with session tracking).

---

## Pattern 5: Terminal Creation and Process Management

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/terminal.test.ts:43-62`

**What:** Terminal instantiation with event-driven lifecycle and text I/O.

```typescript
test('sendText immediately after createTerminal should not throw', async () => {
	const terminal = window.createTerminal();
	const result = await new Promise<Terminal>(r => {
		disposables.push(window.onDidOpenTerminal(t => {
			if (t === terminal) {
				r(t);
			}
		}));
	});
	equal(result, terminal);
	doesNotThrow(terminal.sendText.bind(terminal, 'echo "foo"'));
	await new Promise<void>(r => {
		disposables.push(window.onDidCloseTerminal(t => {
			if (t === terminal) {
				r();
			}
		}));
		terminal.dispose();
	});
});
```

**Variations / call-sites:** `terminal.test.ts:64-103` (onDidWriteTerminalData), `terminal.test.ts:125-145` (processId access), `terminal.test.ts:147-175` (terminal naming and configuration).

---

## Pattern 6: Workspace File Operations via vscode.workspace.fs

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/workspace.fs.test.ts:21-40`

**What:** File system operations (stat, read, write, delete) with error handling.

```typescript
test('fs.stat', async function () {
	const stat = await vscode.workspace.fs.stat(root);
	assert.strictEqual(stat.type, vscode.FileType.Directory);

	assert.strictEqual(typeof stat.size, 'number');
	assert.strictEqual(typeof stat.mtime, 'number');
	assert.strictEqual(typeof stat.ctime, 'number');

	assert.ok(stat.mtime > 0);
	assert.ok(stat.ctime > 0);

	const entries = await vscode.workspace.fs.readDirectory(root);
	assert.ok(entries.length > 0);

	const tuple = entries.find(tuple => tuple[0] === 'far.js')!;
	assert.ok(tuple);
	assert.strictEqual(tuple[0], 'far.js');
	assert.strictEqual(tuple[1], vscode.FileType.File);
});
```

**Variations / call-sites:** `workspace.fs.test.ts:60-79` (write/read/delete), `workspace.fs.test.ts:81-125` (recursive deletion).

---

## Pattern 7: Workspace Events and File Operations

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/workspace.event.test.ts:20-44`

**What:** File creation/deletion event handling with WorkspaceEdit application.

```typescript
test('onWillCreate/onDidCreate', withLogDisabled(async function () {

	const base = await createRandomFile();
	const newUri = base.with({ path: base.path + '-foo' });

	let onWillCreate: vscode.FileWillCreateEvent | undefined;
	let onDidCreate: vscode.FileCreateEvent | undefined;

	disposables.push(vscode.workspace.onWillCreateFiles(e => onWillCreate = e));
	disposables.push(vscode.workspace.onDidCreateFiles(e => onDidCreate = e));

	const edit = new vscode.WorkspaceEdit();
	edit.createFile(newUri);

	const success = await vscode.workspace.applyEdit(edit);
	assert.ok(success);

	assert.ok(onWillCreate);
	assert.strictEqual(onWillCreate?.files.length, 1);
	assert.strictEqual(onWillCreate?.files[0].toString(), newUri.toString());
}));
```

**Variations / call-sites:** `workspace.event.test.ts:46-66` (with file modifications), `workspace.event.test.ts:90-113` (delete events).

---

## Pattern 8: Tree View Data Provider and Navigation

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/tree.test.ts:21-106`

**What:** TreeDataProvider implementation with async element resolution and refresh events.

```typescript
test('TreeView - element already registered', async function () {
	this.timeout(60_000);

	type TreeElement = { readonly kind: 'leaf' };

	class QuickRefreshTreeDataProvider implements vscode.TreeDataProvider<TreeElement> {
		private readonly changeEmitter = new vscode.EventEmitter<TreeElement | undefined>();
		private readonly requestEmitter = new vscode.EventEmitter<number>();
		private readonly pendingRequests: DeferredPromise<TreeElement[]>[] = [];
		private readonly element: TreeElement = { kind: 'leaf' };

		readonly onDidChangeTreeData = this.changeEmitter.event;

		getChildren(element?: TreeElement): Thenable<TreeElement[]> {
			if (!element) {
				const deferred = new DeferredPromise<TreeElement[]>();
				this.pendingRequests.push(deferred);
				this.requestEmitter.fire(this.pendingRequests.length);
				return deferred.p;
			}
			return Promise.resolve([]);
		}

		getTreeItem(): vscode.TreeItem {
			const item = new vscode.TreeItem('duplicate', vscode.TreeItemCollapsibleState.None);
			item.id = 'dup';
			return item;
		}

		getParent(): TreeElement | undefined {
			return undefined;
		}
	}

	const provider = new QuickRefreshTreeDataProvider();
	const treeView = vscode.window.createTreeView('test.treeId', { treeDataProvider: provider });
});
```

**Variations / call-sites:** `tree.test.ts:108-180` (concurrent refresh race conditions), tree reveal operations.

---

## Pattern 9: Notebook API with Kernels and Executors

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/notebook.api.test.ts:39-77`

**What:** NotebookController creation and cell execution with output generation.

```typescript
export class Kernel {

	readonly controller: vscode.NotebookController;
	readonly associatedNotebooks = new Set<string>();

	constructor(id: string, label: string, viewType: string = notebookType) {
		this.controller = vscode.notebooks.createNotebookController(id, viewType, label);
		this.controller.executeHandler = this._execute.bind(this);
		this.controller.supportsExecutionOrder = true;
		this.controller.supportedLanguages = ['typescript', 'javascript'];
		this.controller.onDidChangeSelectedNotebooks(e => {
			if (e.selected) {
				this.associatedNotebooks.add(e.notebook.uri.toString());
			} else {
				this.associatedNotebooks.delete(e.notebook.uri.toString());
			}
		});
	}

	protected async _execute(cells: vscode.NotebookCell[]): Promise<void> {
		for (const cell of cells) {
			await this._runCell(cell);
		}
	}

	protected async _runCell(cell: vscode.NotebookCell) {
		const task = this.controller.createNotebookCellExecution(cell);
		task.start(Date.now());
		task.executionOrder = 1;
		await sleep(10);
		await task.replaceOutput([new vscode.NotebookCellOutput([
			vscode.NotebookCellOutputItem.text(cell.document.getText() || cell.document.uri.toString(), 'text/plain')
		])]);
		task.end(true);
	}
}
```

**Variations / call-sites:** `notebook.document.test.ts:40-163` (NotebookSerializer registration), `notebook.api.test.ts:84-120` (serialization patterns).

---

## Pattern 10: Commands Execution and Registration

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/commands.test.ts:19-46`

**What:** Command discovery and registration with context manipulation.

```typescript
test('getCommands', function (done) {

	const p1 = commands.getCommands().then(commands => {
		let hasOneWithUnderscore = false;
		for (const command of commands) {
			if (command[0] === '_') {
				hasOneWithUnderscore = true;
				break;
			}
		}
		assert.ok(hasOneWithUnderscore);
	}, done);

	const p2 = commands.getCommands(true).then(commands => {
		let hasOneWithUnderscore = false;
		for (const command of commands) {
			if (command[0] === '_') {
				hasOneWithUnderscore = true;
				break;
			}
		}
		assert.ok(!hasOneWithUnderscore);
	}, done);

	Promise.all([p1, p2]).then(() => {
		done();
	}, done);
});
```

**Variations / call-sites:** `commands.test.ts:48-60` (registerCommand with args), `commands.test.ts:62-81` (registerTextEditorCommand), `window.test.ts:90-103` (executeCommand for UI actions).

---

## Pattern 11: Configuration API with Settings Hierarchy

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/configuration.test.ts:15-39`

**What:** Configuration retrieval with language-specific defaults and nested property access.

```typescript
test('configuration, defaults', () => {
	const config = vscode.workspace.getConfiguration('farboo');

	assert.ok(config.has('config0'));
	assert.strictEqual(config.get('config0'), true);
	assert.strictEqual(config.get('config4'), '');
	assert.strictEqual(config['config0'], true);
	assert.strictEqual(config['config4'], '');

	assert.throws(() => (config as Mutable<typeof config>)['config4'] = 'valuevalue');

	assert.ok(config.has('nested.config1'));
	assert.strictEqual(config.get('nested.config1'), 42);
	assert.ok(config.has('nested.config2'));
	assert.strictEqual(config.get('nested.config2'), 'Das Pferd frisst kein Reis.');
});
```

**Variations / call-sites:** `configuration.test.ts:15-22` (language defaults), `terminal.test.ts:20-30` (config.update for integration settings).

---

## Pattern 12: Chat and Language Model API

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/chat.test.ts:50-91`

**What:** Chat participant creation and language model provider registration.

```typescript
function setupParticipant(second?: boolean): Event<{ request: ChatRequest; context: ChatContext }> {
	const emitter = new EventEmitter<{ request: ChatRequest; context: ChatContext }>();
	disposables.push(emitter);

	const id = second ? 'api-test.participant2' : 'api-test.participant';
	const participant = chat.createChatParticipant(id, (request, context, _progress, _token) => {
		emitter.fire({ request, context });
	});
	disposables.push(participant);
	return emitter.event;
}

test('participant and slash command history', async () => {
	const onRequest = setupParticipant();
	await commands.executeCommand('workbench.action.chat.newChat');
	commands.executeCommand('workbench.action.chat.open', { query: '@participant /hello friend' });

	const deferred = new DeferredPromise<void>();
	let i = 0;
	disposables.push(onRequest(request => {
		if (i === 0) {
			assert.deepStrictEqual(request.request.command, 'hello');
			assert.strictEqual(request.request.prompt, 'friend');
		}
	}));
});
```

**Variations / call-sites:** `chat.test.ts:20-41` (language model provider registration), `chat.runInTerminal.test.ts:121-183` (chat tool invocation).

---

## Utility Patterns

### Workspace and RPC Management

**Where:** `extensions/vscode-api-tests/src/utils.ts:49-134`

**What:** Helper functions for test cleanup and RPC validation.

```typescript
export function closeAllEditors(): Thenable<any> {
	return vscode.commands.executeCommand('workbench.action.closeAllEditors');
}

export function disposeAll(disposables: vscode.Disposable[]) {
	vscode.Disposable.from(...disposables).dispose();
}

export async function asPromise<T>(event: vscode.Event<T>, timeout = vscode.env.uiKind === vscode.UIKind.Desktop ? 5000 : 15000): Promise<T> {
	const error = new Error('asPromise TIMEOUT reached');
	return new Promise<T>((resolve, reject) => {
		const handle = setTimeout(() => {
			sub.dispose();
			reject(error);
		}, timeout);

		const sub = event(e => {
			clearTimeout(handle);
			sub.dispose();
			resolve(e);
		});
	});
}

export function assertNoRpc() {
	assertNoRpcFromEntry([vscode, 'vscode']);
}
```

**Variations / call-sites:** Used in all test teardown methods (e.g., `debug.test.ts:13`, `languages.test.ts:13`).

---

## Summary of Integrated Systems

The vscode-api-tests scope demonstrates these core IDE subsystems are heavily API-driven:

1. **Text Editing**: `window.showTextDocument()`, `TextEditor.insertSnippet()`, `TextDocument` manipulation
2. **Language Services**: `vscode.languages.setTextDocumentLanguage()`, diagnostic collections, link providers
3. **Debugging**: Breakpoint management, debug sessions, adapter tracking
4. **Terminal Integration**: `window.createTerminal()`, pseudo-terminals, environment variables, process management
5. **File System**: `workspace.fs.*` operations (stat, read, write, delete, mkdir)
6. **Workspace Events**: File lifecycle (create/delete), text document changes, notebook changes
7. **Navigation**: TreeView providers, element reveal, hierarchical structures
8. **Notebooks**: Serialization, kernel execution, cell output, execution order
9. **Command Execution**: Command discovery, registration, context-aware execution
10. **Configuration**: Settings hierarchy, language-specific config, runtime updates
11. **Chat & LM**: Participant creation, language model invocation, tool execution
12. **Resource Management**: Disposable patterns, event subscription cleanup, teardown verification

All patterns use asynchronous Promise-based APIs with event emitters for state changes. The test infrastructure validates against unintended RPC proxying via `assertNoRpc()` checks, ensuring clean API boundaries.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
