# VS Code Core IDE Functionality: API Test Patterns

Research Question: What would it take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust?

This document catalogs concrete API test patterns from `extensions/vscode-api-tests/` that define the behavioral contracts that a Tauri/Rust host would need to satisfy.

---

## Pattern 1: Text Editor Manipulation & Snippet Insertion

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts:37-51`

**What:** Basic snippet insertion with cursor/selection support—foundational editing capability required for any IDE.

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

**Variations / call-sites:**
- `editor.test.ts:79-95` — Snippet with selection replacement
- `editor.test.ts:110-126` — Snippet with indentation preservation
- `editor.test.ts:128-144` — Snippet with explicit selection argument

---

## Pattern 2: Document Edit Operations (Batch Edits)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts:146-156`

**What:** Atomic, batch text editor operations via `editor.edit()` callback—core for incremental modifications.

```typescript
test('make edit', () => {
	return withRandomFileEditor('', (editor, doc) => {
		return editor.edit((builder) => {
			builder.insert(new Position(0, 0), 'Hello World');
		}).then(applied => {
			assert.ok(applied);
			assert.strictEqual(doc.getText(), 'Hello World');
			assert.ok(doc.isDirty);
		});
	});
});
```

**Variations / call-sites:**
- `editor.test.ts:158-168` — Range replacement with `Number.MAX_VALUE` bounds
- `editor.test.ts:198-215` — Overlapping edit detection (must reject)

---

## Pattern 3: Editor Options & Configuration Management

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts:170-196`

**What:** Runtime editor option querying and modification (tab size, insert spaces, cursor style, line numbers).

```typescript
test('issue #16573: Extension API: insertSpaces and tabSize are undefined', () => {
	return withRandomFileEditor('Hello world!\n\tHello world!', (editor, _doc) => {
		assert.strictEqual(editor.options.tabSize, 4);
		assert.strictEqual(editor.options.insertSpaces, false);
		assert.strictEqual(editor.options.cursorStyle, TextEditorCursorStyle.Line);
		assert.strictEqual(editor.options.lineNumbers, TextEditorLineNumbersStyle.On);

		editor.options = { tabSize: 2 };
		assert.strictEqual(editor.options.tabSize, 2);

		return Promise.resolve();
	});
});
```

---

## Pattern 4: Debugging API: Breakpoints & Debug Sessions

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/debug.test.ts:28-53`

**What:** Breakpoint lifecycle management and tracking via event emitters and state queries.

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

**Variations / call-sites:**
- `debug.test.ts:55-64` — Function breakpoints with hit conditions and log messages

---

## Pattern 5: Language Services: Diagnostics & Code Actions

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts:76-97`

**What:** Diagnostic collection creation, registration, and querying across document URIs.

```typescript
test('diagnostics, read & event', function () {
	const uri = vscode.Uri.file('/foo/bar.txt');
	const col1 = vscode.languages.createDiagnosticCollection('foo1');
	col1.set(uri, [new vscode.Diagnostic(new vscode.Range(0, 0, 0, 12), 'error1')]);

	const col2 = vscode.languages.createDiagnosticCollection('foo2');
	col2.set(uri, [new vscode.Diagnostic(new vscode.Range(0, 0, 0, 12), 'error1')]);

	const diag = vscode.languages.getDiagnostics(uri);
	assert.strictEqual(diag.length, 2);

	const tuples = vscode.languages.getDiagnostics();
	let found = false;
	for (const [thisUri,] of tuples) {
		if (thisUri.toString() === uri.toString()) {
			found = true;
			break;
		}
	}
	assert.ok(tuples.length >= 1);
	assert.ok(found);
});
```

---

## Pattern 6: Code Actions Provider Registration & Execution

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts:126-170`

**What:** Registration of code action providers filtered by document patterns, and synchronous invocation by URI and range.

```typescript
test('diagnostics & CodeActionProvider', async function () {
	class D2 extends vscode.Diagnostic {
		customProp = { complex() { } };
		constructor() {
			super(new vscode.Range(0, 2, 0, 7), 'sonntag');
		}
	}

	const diag1 = new vscode.Diagnostic(new vscode.Range(0, 0, 0, 5), 'montag');
	const diag2 = new D2();

	let ran = false;
	const uri = vscode.Uri.parse('ttt:path.far');

	const r1 = vscode.languages.registerCodeActionsProvider({ pattern: '*.far', scheme: 'ttt' }, {
		provideCodeActions(_document, _range, ctx): vscode.Command[] {
			assert.strictEqual(ctx.diagnostics.length, 2);
			const [first, second] = ctx.diagnostics;
			assert.ok(first === diag1);
			assert.ok(second === diag2);
			ran = true;
			return [];
		}
	});

	const r2 = vscode.workspace.registerTextDocumentContentProvider('ttt', {
		provideTextDocumentContent() {
			return 'this is some text';
		}
	});

	const r3 = vscode.languages.createDiagnosticCollection();
	r3.set(uri, [diag1]);

	const r4 = vscode.languages.createDiagnosticCollection();
	r4.set(uri, [diag2]);

	await vscode.workspace.openTextDocument(uri);
	await vscode.commands.executeCommand('vscode.executeCodeActionProvider', uri, new vscode.Range(0, 0, 0, 10));
	assert.ok(ran);
	vscode.Disposable.from(r1, r2, r3, r4).dispose();
});
```

**Variations / call-sites:**
- `languages.test.ts:101-124` — Document link provider registration and execution

---

## Pattern 7: Completion Item Provider (IntelliSense)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts:172-192`

**What:** Multi-document-filter completion provider registration and execution via command protocol.

```typescript
test('completions with document filters', async function () {
	let ran = false;
	const uri = vscode.Uri.file(join(vscode.workspace.rootPath || '', './bower.json'));

	const jsonDocumentFilter = [
		{ language: 'json', pattern: '**/package.json' },
		{ language: 'json', pattern: '**/bower.json' },
		{ language: 'json', pattern: '**/.bower.json' }
	];

	const r1 = vscode.languages.registerCompletionItemProvider(jsonDocumentFilter, {
		provideCompletionItems: (_document: vscode.TextDocument, _position: vscode.Position, _token: vscode.CancellationToken): vscode.CompletionItem[] => {
			const proposal = new vscode.CompletionItem('foo');
			proposal.kind = vscode.CompletionItemKind.Property;
			ran = true;
			return [proposal];
		}
	});

	await vscode.workspace.openTextDocument(uri);
	const result = await vscode.commands.executeCommand<vscode.CompletionList>('vscode.executeCompletionItemProvider', uri, new vscode.Position(1, 0));
	r1.dispose();
	assert.ok(ran, 'Provider has not been invoked');
	assert.ok(result!.items.some(i => i.label === 'foo'), 'Results do not include "foo"');
});
```

---

## Pattern 8: Language Document State Changes (onDidCloseTextDocument, onDidOpenTextDocument)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts:29-62`

**What:** Document lifecycle event ordering when changing language via `setTextDocumentLanguage`.

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
	const open = new Promise<void>(resolve => {
		disposables.push(vscode.workspace.onDidOpenTextDocument(e => {
			if (e === doc) {
				assert.strictEqual(doc.languageId, 'json');
				assert.strictEqual(clock, 1);
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

---

## Pattern 9: Terminal Creation & Output Events

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/terminal.test.ts:43-62`

**What:** Terminal lifecycle management: creation, event subscription, text sending, and disposal.

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

**Variations / call-sites:**
- `terminal.test.ts:64-103` — Echo output data capture via `onDidWriteTerminalData` event
- `terminal.test.ts:105-123` — Terminal close event firing on disposal

---

## Pattern 10: Shell Integration (Terminal Execution Events)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/terminal.shellIntegration.test.ts:83-107`

**What:** Shell integration activation and command execution event sequencing (start, output, end).

```typescript
test('window.onDidChangeTerminalShellIntegration should activate for the default terminal', async () => {
	const terminal = await new Promise<Terminal>(r => {
		disposables.push(window.onDidOpenTerminal(t => {
			if (t === terminal) {
				r(terminal);
			}
		}));
		const terminal = window.createTerminal({
			env: { TEST: '`' }
		});
		terminal.show();
	});

	let data = '';
	await new Promise<void>(r => {
		disposables.push(window.onDidWriteTerminalData(e => {
			if (e.terminal === terminal) {
				data += e.data;
				if (data.indexOf('`') !== 0) {
					r();
				}
			}
		}));
		if (process.platform === 'win32') {
			terminal.sendText(`$env:TEST`);
		} else {
			terminal.sendText(`echo $TEST`);
		}
	});

	await new Promise<void>(r => {
		terminal.dispose();
		disposables.push(window.onDidCloseTerminal(t => {
			strictEqual(terminal, t);
			r();
		}));
	});
});
```

**Variations / call-sites:**
- `terminal.shellIntegration.test.ts:121-143` — Exit code reporting in execution events
- `terminal.shellIntegration.test.ts:167-180` — Command output iteration via `TerminalShellExecution.read()`

---

## Pattern 11: Workspace File System (stat, read, write, delete)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/workspace.fs.test.ts:21-40`

**What:** File system operations: stat (metadata), readDirectory, type checking (File vs Directory).

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

**Variations / call-sites:**
- `workspace.fs.test.ts:60-79` — Write, stat, read, delete workflow
- `workspace.fs.test.ts:81-99` — Recursive delete with non-empty folder protection
- `workspace.fs.test.ts:220-250` — Recursive directory creation

---

## Pattern 12: Workspace Events (onWillCreateFiles, onDidCreateFiles, onWillDeleteFiles, onDidDeleteFiles)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/workspace.event.test.ts:20-44`

**What:** Workspace-level file mutation events with event waiting (promise-based lifecycle).

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

	assert.ok(onDidCreate);
	assert.strictEqual(onDidCreate?.files.length, 1);
	assert.strictEqual(onDidCreate?.files[0].toString(), newUri.toString());
}));
```

**Variations / call-sites:**
- `workspace.event.test.ts:46-66` — Event interception with cross-document edit mutations
- `workspace.event.test.ts:90-104` — Delete event lifecycle

---

## Pattern 13: Tree Views (Data Providers & Item Rendering)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/tree.test.ts:21-106`

**What:** TreeDataProvider implementation with async element fetching, state management, and reveal/refresh semantics.

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

		async waitForRequestCount(count: number): Promise<void> {
			while (this.pendingRequests.length < count) {
				await asPromise(this.requestEmitter.event);
			}
		}

		async resolveNextRequest(): Promise<void> {
			const next = this.pendingRequests.shift();
			if (!next) {
				return;
			}
			await next.complete([this.element]);
		}
	}

	const provider = new QuickRefreshTreeDataProvider();
	disposables.push(provider);

	const treeView = vscode.window.createTreeView('test.treeId', { treeDataProvider: provider });
	disposables.push(treeView);

	const revealFirst = (treeView.reveal(provider.getElement(), { expand: true })
		.then(() => ({ error: undefined as Error | undefined })) as Promise<{ error: Error | undefined }>)
		.catch(error => ({ error }));
	const revealSecond = (treeView.reveal(provider.getElement(), { expand: true })
		.then(() => ({ error: undefined as Error | undefined })) as Promise<{ error: Error | undefined }>)
		.catch(error => ({ error }));

	await provider.waitForRequestCount(2);
	await provider.resolveNextRequest();
	await delay(0);
	await provider.resolveNextRequest();

	const [firstResult, secondResult] = await Promise.all([revealFirst, revealSecond]);
	const errors = [firstResult.error, secondResult.error].filter((e): e is Error => !!e);
	assert.strictEqual(errors.length, 1, 'Exactly one reveal should fail from the stale fetch');
	assert.ok(/Cannot resolve tree item/.test(errors[0].message));
});
```

---

## Pattern 14: Window / Editor Group Management

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/window.test.ts:19-34`

**What:** Active editor tracking and document opening via window API.

```typescript
test('editor, active text editor', async () => {
	const doc = await workspace.openTextDocument(join(workspace.rootPath || '', './far.js'));
	await window.showTextDocument(doc);
	const active = window.activeTextEditor;
	assert.ok(active);
	assert.ok(pathEquals(active!.document.uri.fsPath, doc.uri.fsPath));
});
```

**Variations / call-sites:**
- `window.test.ts:27-34` — Document opening via resource URI
- `window.test.ts:41-53` — Editor view column assignment (One, Two, Three)
- `window.test.ts:55-72` — `onDidChangeVisibleTextEditors` event tracking

---

## Pattern 15: Notebook API (Kernel Registration & Cell Execution)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/notebook.api.test.ts:39-77`

**What:** Notebook kernel controller registration with cell execution handler and output rendering.

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

---

## Pattern 16: Command Registration & Execution

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/commands.test.ts:48-60`

**What:** Command registration with variable arguments and filtering (getCommands with includeAllEndpointsCommands flag).

```typescript
test('command with args', async function () {
	let args: IArguments;
	const registration = commands.registerCommand('t1', function () {
		args = arguments;
	});

	await commands.executeCommand('t1', 'start');
	registration.dispose();
	assert.ok(args!);
	assert.strictEqual(args!.length, 1);
	assert.strictEqual(args![0], 'start');
});
```

**Variations / call-sites:**
- `commands.test.ts:19-46` — getCommands() with include-internal flag
- `commands.test.ts:62-80` — Text editor commands with injected editor + extra arguments

---

## Pattern 17: Test Harness Utilities (File Creation, Editor Cleanup)

**Where:** `extensions/vscode-api-tests/src/utils.ts:19-29`

**What:** Memory file system for test isolation; deterministic file creation and cleanup.

```typescript
export async function createRandomFile(contents: string | Uint8Array = '', dir: vscode.Uri | undefined = undefined, ext = ''): Promise<vscode.Uri> {
	let fakeFile: vscode.Uri;
	if (dir) {
		assert.strictEqual(dir.scheme, testFs.scheme);
		fakeFile = dir.with({ path: dir.path + '/' + rndName() + ext });
	} else {
		fakeFile = vscode.Uri.parse(`${testFs.scheme}:/${rndName() + ext}`);
	}
	testFs.writeFile(fakeFile, typeof contents === 'string' ? Buffer.from(contents) : Buffer.from(contents), { create: true, overwrite: true });
	return fakeFile;
}
```

**Variations / call-sites:**
- `utils.ts:49-51` — closeAllEditors() command
- `utils.ts:53-55` — saveAllEditors() command
- `utils.ts:57-59` — revertAllDirty() internal command

---

## Summary

The VS Code API test suite defines **17 core patterns** spanning:

1. **Text Editing**: Snippet insertion, batch edits, range operations, editor options
2. **Debugging**: Breakpoint lifecycle, debug sessions, condition/log management
3. **Language Services**: Diagnostics, code actions, completions, document links, folding
4. **Terminal**: Creation, I/O events, shell integration, execution tracking
5. **File System**: stat, read, write, delete, recursive operations, watchers
6. **Workspace Events**: File mutations (create/delete/rename) with event interception
7. **UI Components**: Tree views, editor groups, window state, tab management
8. **Notebooks**: Kernel controllers, cell execution, output rendering
9. **Commands**: Registration, execution, filtering, argument passing

A Tauri/Rust host would need to implement **async event-driven APIs** mirroring these patterns, with particular emphasis on:
- Promise/Promise-like futures for all async operations
- Event emitters for lifecycle tracking (onDidChange, onDidCreate, onDidDelete, etc.)
- Type-safe provider registration by document filter
- Atomic multi-edit transactions
- Precise string/buffer boundary handling in file operations

All patterns use mocha test structure (`suite()`, `test()`, `teardown()`) and assert library for contract validation.
