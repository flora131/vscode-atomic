# Merge Conflict Extension: Core Pattern Analysis

Analysis of VS Code's merge-conflict extension (`extensions/merge-conflict/`) to identify architectural and implementation patterns relevant to IDE functionality porting considerations.

## Key Findings

### Pattern 1: CodeLens Provider Registration with Multi-Scheme Support
**Where:** `extensions/merge-conflict/src/codelensProvider.ts:100-106`
**What:** Registers CodeLens provider across multiple URI schemes to support different document types.

```typescript
private registerCodeLensProvider() {
	this.codeLensRegistrationHandle = vscode.languages.registerCodeLensProvider([
		{ scheme: 'file' },
		{ scheme: 'vscode-vfs' },
		{ scheme: 'untitled' },
		{ scheme: 'vscode-userdata' },
	], this);
}
```

**Variations / call-sites:** 
- Conditionally registered in `begin()` (line 18-23)
- Dynamically registered/disposed on config changes in `configurationUpdated()` (lines 28-34)
- Implementation pattern shows provider implements `vscode.CodeLensProvider` interface (line 9)

---

### Pattern 2: CodeLens Provider Implementation with Command Arguments
**Where:** `extensions/merge-conflict/src/codelensProvider.ts:47-98`
**What:** Provides array of CodeLens objects with commands, passing conflict objects as arguments.

```typescript
async provideCodeLenses(document: vscode.TextDocument, _token: vscode.CancellationToken): Promise<vscode.CodeLens[] | null> {
	if (!this.config || !this.config.enableCodeLens) {
		return null;
	}

	const conflicts = await this.tracker.getConflicts(document);
	const conflictsCount = conflicts?.length ?? 0;
	vscode.commands.executeCommand('setContext', 'mergeConflictsCount', conflictsCount);

	if (!conflictsCount) {
		return null;
	}

	const items: vscode.CodeLens[] = [];

	conflicts.forEach(conflict => {
		const acceptCurrentCommand: vscode.Command = {
			command: 'merge-conflict.accept.current',
			title: vscode.l10n.t("Accept Current Change"),
			arguments: ['known-conflict', conflict]
		};
		// ... similar commands for incoming, both, diff ...
		const range = document.lineAt(conflict.range.start.line).range;
		items.push(
			new vscode.CodeLens(range, acceptCurrentCommand),
			new vscode.CodeLens(range, acceptIncomingCommand),
			new vscode.CodeLens(range, acceptBothCommand),
			new vscode.CodeLens(range, diffCommand)
		);
	});

	return items;
}
```

**Key aspects:**
- Returns null conditionally (feature disabled or no conflicts)
- Updates VS Code context via `setContext` for conditional UI
- Creates multiple CodeLens objects at same range with different commands
- Passes structured data as command arguments

---

### Pattern 3: TextEditorDecorationType Management with Theme Colors
**Where:** `extensions/merge-conflict/src/mergeDecorator.ts:55-125`
**What:** Creates and manages multiple decoration types with theme-aware colors and outline styling.

```typescript
private registerDecorationTypes(config: interfaces.IExtensionConfiguration) {
	// Dispose of existing decorations
	Object.keys(this.decorations).forEach(k => this.decorations[k].dispose());
	this.decorations = {};

	if (!config.enableDecorations || !config.enableEditorOverview) {
		return;
	}

	if (config.enableDecorations || config.enableEditorOverview) {
		this.decorations['current.content'] = vscode.window.createTextEditorDecorationType(
			this.generateBlockRenderOptions('merge.currentContentBackground', 'editorOverviewRuler.currentContentForeground', config)
		);
		this.decorations['incoming.content'] = vscode.window.createTextEditorDecorationType(
			this.generateBlockRenderOptions('merge.incomingContentBackground', 'editorOverviewRuler.incomingContentForeground', config)
		);
	}

	if (config.enableDecorations) {
		this.decorations['current.header'] = vscode.window.createTextEditorDecorationType({
			isWholeLine: this.decorationUsesWholeLine,
			backgroundColor: new vscode.ThemeColor('merge.currentHeaderBackground'),
			color: new vscode.ThemeColor('editor.foreground'),
			outlineStyle: 'solid',
			outlineWidth: '1pt',
			outlineColor: new vscode.ThemeColor('merge.border'),
			after: {
				contentText: ' ' + vscode.l10n.t("(Current Change)"),
				color: new vscode.ThemeColor('descriptionForeground')
			}
		});
	}
}
```

**Variations / call-sites:**
- Called in `begin()` during initialization (line 23-25)
- Re-called in `configurationUpdated()` with full re-decoration of visible editors (lines 44-53)
- Disposed in `dispose()` method (lines 127-135)
- Applied per-editor in `applyDecorations()` (line 229)

---

### Pattern 4: Document Event-Driven Decoration Application
**Where:** `extensions/merge-conflict/src/mergeDecorator.ts:23-42`
**What:** Attaches workspace and window event listeners to track and apply decorations across document lifecycle.

```typescript
begin(config: interfaces.IExtensionConfiguration) {
	this.config = config;
	this.registerDecorationTypes(config);

	// Check if we already have a set of active windows, attempt to track these.
	vscode.window.visibleTextEditors.forEach(e => this.applyDecorations(e));

	vscode.workspace.onDidOpenTextDocument(event => {
		this.applyDecorationsFromEvent(event);
	}, null, this.context.subscriptions);

	vscode.workspace.onDidChangeTextDocument(event => {
		this.applyDecorationsFromEvent(event.document);
	}, null, this.context.subscriptions);

	vscode.window.onDidChangeVisibleTextEditors((e) => {
		// Any of which could be new (not just the active one).
		e.forEach(e => this.applyDecorations(e));
	}, null, this.context.subscriptions);
}
```

**Key aspects:**
- Initializes decorations on currently visible editors
- Tracks three event types: document open, document change, editor visibility
- Passes `this.context.subscriptions` to automatically dispose listeners
- Deferred application: calls `applyDecorationsFromEvent()` which finds matching editor

---

### Pattern 5: Disposable Service Registration with Begin/Configuration Pattern
**Where:** `extensions/merge-conflict/src/services.ts:27-53`
**What:** Service wrapper that initializes multiple disposable components with uniform lifecycle and config update handling.

```typescript
begin() {
	const configuration = this.createExtensionConfiguration();
	const documentTracker = new DocumentTracker(this.telemetryReporter);

	this.services.push(
		documentTracker,
		new CommandHandler(documentTracker),
		new CodeLensProvider(documentTracker),
		new ContentProvider(this.context),
		new Decorator(this.context, documentTracker),
	);

	this.services.forEach((service: any) => {
		if (service.begin && service.begin instanceof Function) {
			service.begin(configuration);
		}
	});

	vscode.workspace.onDidChangeConfiguration(() => {
		this.services.forEach((service: any) => {
			if (service.configurationUpdated && service.configurationUpdated instanceof Function) {
				service.configurationUpdated(this.createExtensionConfiguration());
			}
		});
	});
}
```

**Key aspects:**
- Creates shared `DocumentTracker` service injected into multiple providers
- Uses duck-typing pattern checking for `begin` and `configurationUpdated` methods
- Single configuration creation point passed to all services
- Configuration refresh coordinated globally on workspace config change

---

### Pattern 6: Deferred Computation with Debouncing Using Delayer
**Where:** `extensions/merge-conflict/src/documentTracker.ts:53-79`
**What:** Uses Delayer utility to debounce conflict scan requests from multiple origins with cache invalidation.

```typescript
getConflicts(document: vscode.TextDocument, origin: string): PromiseLike<interfaces.IDocumentMergeConflict[]> {
	const key = this.getCacheKey(document);

	if (!key) {
		return Promise.resolve(this.getConflictsOrEmpty(document, [origin]));
	}

	let cacheItem = this.cache.get(key);
	if (!cacheItem) {
		cacheItem = new ScanTask(this.delayExpireTime, origin);
		this.cache.set(key, cacheItem);
	}
	else {
		cacheItem.addOrigin(origin);
	}

	return cacheItem.delayTask.trigger(() => {
		const conflicts = this.getConflictsOrEmpty(document, Array.from(cacheItem!.origins));
		this.cache?.delete(key!);
		return conflicts;
	});
}
```

**Delayer Implementation:** `extensions/merge-conflict/src/delayer.ts:10-79`
- Simple generic delay mechanism using setTimeout
- Supports forced delivery without waiting
- Tracks triggered state
- Allows cancellation

**Variations / call-sites:**
- Called from CodeLensProvider (codelensProvider.ts:53)
- Called from CommandHandler (commandHandler.ts:98, 259, 279, 299)
- Called from Decorator (mergeDecorator.ts:178)

---

### Pattern 7: Command Registration with Multiple Entry Points
**Where:** `extensions/merge-conflict/src/commandHandler.ts:28-51`
**What:** Registers text editor commands with optional resource-based alternative handler for multi-resource operations.

```typescript
begin() {
	this.disposables.push(
		this.registerTextEditorCommand('merge-conflict.accept.current', this.acceptCurrent),
		this.registerTextEditorCommand('merge-conflict.accept.incoming', this.acceptIncoming),
		this.registerTextEditorCommand('merge-conflict.accept.selection', this.acceptSelection),
		this.registerTextEditorCommand('merge-conflict.accept.both', this.acceptBoth),
		this.registerTextEditorCommand('merge-conflict.accept.all-current', this.acceptAllCurrent, this.acceptAllCurrentResources),
		this.registerTextEditorCommand('merge-conflict.accept.all-incoming', this.acceptAllIncoming, this.acceptAllIncomingResources),
		this.registerTextEditorCommand('merge-conflict.accept.all-both', this.acceptAllBoth),
		this.registerTextEditorCommand('merge-conflict.next', this.navigateNext),
		this.registerTextEditorCommand('merge-conflict.previous', this.navigatePrevious),
		this.registerTextEditorCommand('merge-conflict.compare', this.compare)
	);
}

private registerTextEditorCommand(command: string, cb: (editor: vscode.TextEditor, ...args: any[]) => Promise<void>, resourceCB?: (uris: vscode.Uri[]) => Promise<void>) {
	return vscode.commands.registerCommand(command, (...args) => {
		if (resourceCB && args.length && args.every(arg => arg && arg.resourceUri)) {
			return resourceCB.call(this, args.map(arg => arg.resourceUri));
		}
		const editor = vscode.window.activeTextEditor;
		return editor && cb.call(this, editor, ...args);
	});
}
```

**Key aspects:**
- Dual-mode handler: resource-based vs editor-based
- Detects invocation context by inspecting argument structure
- All commands return Promises for async operations

---

### Pattern 8: TextDocumentContentProvider for Virtual Documents
**Where:** `extensions/merge-conflict/src/contentProvider.ts:8-53`
**What:** Implements custom content provider for diff view virtual documents reconstructing content from conflict regions.

```typescript
export default class MergeConflictContentProvider implements vscode.TextDocumentContentProvider, vscode.Disposable {

	static scheme = 'merge-conflict.conflict-diff';

	constructor(private context: vscode.ExtensionContext) {
	}

	begin() {
		this.context.subscriptions.push(
			vscode.workspace.registerTextDocumentContentProvider(MergeConflictContentProvider.scheme, this)
		);
	}

	async provideTextDocumentContent(uri: vscode.Uri): Promise<string | null> {
		try {
			const { scheme, ranges } = JSON.parse(uri.query) as { scheme: string; ranges: [{ line: number; character: number }[], { line: number; character: number }[]][] };

			const document = await vscode.workspace.openTextDocument(uri.with({ scheme, query: '' }));

			let text = '';
			let lastPosition = new vscode.Position(0, 0);

			ranges.forEach(rangeObj => {
				const [conflictRange, fullRange] = rangeObj;
				const [start, end] = conflictRange;
				const [fullStart, fullEnd] = fullRange;

				text += document.getText(new vscode.Range(lastPosition.line, lastPosition.character, fullStart.line, fullStart.character));
				text += document.getText(new vscode.Range(start.line, start.character, end.line, end.character));
				lastPosition = new vscode.Position(fullEnd.line, fullEnd.character);
			});

			const documentEnd = document.lineAt(document.lineCount - 1).range.end;
			text += document.getText(new vscode.Range(lastPosition.line, lastPosition.character, documentEnd.line, documentEnd.character));

			return text;
		}
		catch (ex) {
			await vscode.window.showErrorMessage('Unable to show comparison');
			return null;
		}
	}
}
```

**Key aspects:**
- Custom URI scheme isolation
- Encodes ranges as JSON in URI query string
- Reconstructs document content from fragments
- Error handling with user notification

---

## Architecture Summary

The merge-conflict extension exemplifies a modular, event-driven architecture for IDE features:

1. **Lifecycle Management:** Service wrapper coordinates initialization and configuration updates across multiple providers
2. **Event-Driven Updates:** Workspace and editor events trigger automatic re-computation and re-decoration
3. **Shared State:** DocumentTracker service provides single source of truth for conflict detection with debouncing
4. **UI Integration:** CodeLens and decorations register via VS Code APIs with multi-scheme support
5. **Virtual Documents:** Custom content providers enable diff views through URI-based content generation
6. **Command Routing:** Flexible command handlers support both editor and resource contexts
7. **Resource Management:** Systematic disposal of listeners, decorations, and registrations

## Porting Implications

From a Tauri/Rust perspective, these patterns suggest:

- **Multi-document abstraction** needed (file, vfs, untitled, userdata schemes)
- **Event broker** required for document/editor lifecycle coordination
- **Decoration system** needs theme color resolution and renderer integration
- **Command dispatch** requires context detection (active editor vs file resource)
- **Debouncing primitives** for expensive operations (parsing/scanning)
- **Virtual document protocol** for synthetic content generation
- **Disposable pattern** throughout for memory management

The extension demonstrates that IDE functionality depends heavily on event-driven architecture, with features coordinated through a pub-sub model and centralized configuration management rather than direct module dependencies.
