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

