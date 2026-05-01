# VS Code Merge Conflict Extension: Core IDE Porting Patterns

## Research Question
What patterns would need to be ported from TypeScript/Electron to Tauri/Rust for merge conflict handling?

## Scope Analysis
The `extensions/merge-conflict/` extension (13 files, 1,463 LOC) provides CodeLens providers, text decorations, and conflict resolution UI. This requires porting several core VS Code abstraction layers.

---

#### Pattern: CodeLensProvider Registration and Lifecycle
**Where:** `extensions/merge-conflict/src/codelensProvider.ts:9-108`
**What:** Implements the VS Code CodeLensProvider interface with dynamic registration based on configuration changes.
```typescript
export default class MergeConflictCodeLensProvider implements vscode.CodeLensProvider, vscode.Disposable {
	private codeLensRegistrationHandle?: vscode.Disposable | null;
	private config?: interfaces.IExtensionConfiguration;
	private tracker: interfaces.IDocumentMergeConflictTracker;

	constructor(trackerService: interfaces.IDocumentMergeConflictTrackerService) {
		this.tracker = trackerService.createTracker('codelens');
	}

	begin(config: interfaces.IExtensionConfiguration) {
		this.config = config;
		if (this.config.enableCodeLens) {
			this.registerCodeLensProvider();
		}
	}

	configurationUpdated(updatedConfig: interfaces.IExtensionConfiguration) {
		if (updatedConfig.enableCodeLens === false && this.codeLensRegistrationHandle) {
			this.codeLensRegistrationHandle.dispose();
			this.codeLensRegistrationHandle = null;
		}
		else if (updatedConfig.enableCodeLens === true && !this.codeLensRegistrationHandle) {
			this.registerCodeLensProvider();
		}
		this.config = updatedConfig;
	}

	async provideCodeLenses(document: vscode.TextDocument, _token: vscode.CancellationToken): Promise<vscode.CodeLens[] | null> {
		if (!this.config || !this.config.enableCodeLens) {
			return null;
		}
		const conflicts = await this.tracker.getConflicts(document);
		const conflictsCount = conflicts?.length ?? 0;
		vscode.commands.executeCommand('setContext', 'mergeConflictsCount', conflictsCount);
		if (!conflictsCount) { return null; }
		
		const items: vscode.CodeLens[] = [];
		conflicts.forEach(conflict => {
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

	private registerCodeLensProvider() {
		this.codeLensRegistrationHandle = vscode.languages.registerCodeLensProvider([
			{ scheme: 'file' },
			{ scheme: 'vscode-vfs' },
			{ scheme: 'untitled' },
			{ scheme: 'vscode-userdata' },
		], this);
	}
}
```

**Variations / call-sites:**
- `services.ts:35` - CodeLensProvider instantiated in service wrapper
- `mergeConflictMain.ts:9-14` - Extension activation hook

**Porting implications:** Requires implementing a language provider abstraction layer that:
- Registers document processors per scheme (file, VFS, untitled, userdata)
- Supports dynamic enable/disable via configuration
- Handles cancellation tokens
- Returns inline commands with arguments

---

#### Pattern: Text Editor Decorations with Theme Colors
**Where:** `extensions/merge-conflict/src/mergeDecorator.ts:55-125`
**What:** Creates and manages reusable decoration types with theme color bindings and range application.
```typescript
private registerDecorationTypes(config: interfaces.IExtensionConfiguration) {
	Object.keys(this.decorations).forEach(k => this.decorations[k].dispose());
	this.decorations = {};

	if (!config.enableDecorations || !config.enableEditorOverview) {
		return;
	}

	this.decorations['current.content'] = vscode.window.createTextEditorDecorationType(
		this.generateBlockRenderOptions('merge.currentContentBackground', 'editorOverviewRuler.currentContentForeground', config)
	);

	this.decorations['incoming.header'] = vscode.window.createTextEditorDecorationType({
		backgroundColor: new vscode.ThemeColor('merge.incomingHeaderBackground'),
		color: new vscode.ThemeColor('editor.foreground'),
		outlineStyle: 'solid',
		outlineWidth: '1pt',
		outlineColor: new vscode.ThemeColor('merge.border'),
		isWholeLine: true,
		after: {
			contentText: ' ' + vscode.l10n.t("(Incoming Change)"),
			color: new vscode.ThemeColor('descriptionForeground')
		}
	});
}

private async applyDecorations(editor: vscode.TextEditor) {
	const conflicts = await this.tracker.getConflicts(editor.document);
	if (conflicts.length === 0) {
		this.removeDecorations(editor);
		return;
	}

	const matchDecorations: { [key: string]: vscode.Range[] } = {};
	conflicts.forEach(conflict => {
		matchDecorations['current.content'] = matchDecorations['current.content'] || [];
		matchDecorations['current.content'].push(conflict.current.decoratorContent);
		matchDecorations['current.header'] = matchDecorations['current.header'] || [];
		matchDecorations['current.header'].push(conflict.current.header);
	});

	Object.keys(matchDecorations).forEach(decorationKey => {
		const decorationType = this.decorations[decorationKey];
		if (decorationType) {
			editor.setDecorations(decorationType, matchDecorations[decorationKey]);
		}
	});
}
```

**Variations / call-sites:**
- `mergeDecorator.ts:28-41` - Event-based re-decoration on document open/change
- `mergeDecorator.ts:163-236` - Conflict detection and range application
- `mergeDecorator.ts:44-53` - Configuration update triggers re-registration

**Porting implications:** Requires:
- Decoration registry with disposal management
- Theme color system (theme: strings vs. direct colors)
- Range-to-decoration mapping (batch operations)
- Overview ruler support (minimap-like UI element)
- Whole-line decorations with text affixes (before/after content)

---

#### Pattern: Conflict Detection via Regex Scanning
**Where:** `extensions/merge-conflict/src/mergeConflictParser.ts:10-86`
**What:** Stateful line-by-line parser detecting merge conflict markers and building conflict descriptors.
```typescript
const startHeaderMarker = '<<<<<<<';
const commonAncestorsMarker = '|||||||';
const splitterMarker = '=======';
const endFooterMarker = '>>>>>>>';

static scanDocument(document: vscode.TextDocument, telemetryReporter: TelemetryReporter): interfaces.IDocumentMergeConflict[] {
	let currentConflict: IScanMergedConflict | null = null;
	const conflictDescriptors: interfaces.IDocumentMergeConflictDescriptor[] = [];

	for (let i = 0; i < document.lineCount; i++) {
		const line = document.lineAt(i);
		if (!line || line.isEmptyOrWhitespace) { continue; }

		if (line.text.startsWith(startHeaderMarker)) {
			if (currentConflict !== null) {
				currentConflict = null;
				break;
			}
			currentConflict = { startHeader: line, commonAncestors: [] };
		}
		else if (currentConflict && !currentConflict.splitter && line.text.startsWith(commonAncestorsMarker)) {
			currentConflict.commonAncestors.push(line);
		}
		else if (currentConflict && !currentConflict.splitter && line.text === splitterMarker) {
			currentConflict.splitter = line;
		}
		else if (currentConflict && line.text.startsWith(endFooterMarker)) {
			currentConflict.endFooter = line;
			const completeDescriptor = MergeConflictParser.scanItemTolMergeConflictDescriptor(document, currentConflict);
			if (completeDescriptor !== null) {
				conflictDescriptors.push(completeDescriptor);
			}
			currentConflict = null;
		}
	}

	return conflictDescriptors
		.filter(Boolean)
		.map(descriptor => new DocumentMergeConflict(descriptor, telemetryReporter));
}

static containsConflict(document: vscode.TextDocument): boolean {
	if (!document) { return false; }
	const text = document.getText();
	// Quick check before full scan
	return text.includes(startHeaderMarker) && text.includes(endFooterMarker);
}
```

**Variations / call-sites:**
- `documentTracker.ts:118-124` - Uses containsConflict() guard before full scan
- `mergeConflictParser.ts:88-143` - Range computation for conflict regions

**Porting implications:** Requires:
- Line-by-line text access API (not just full document getText())
- TextLine abstraction with range/rangeIncludingLineBreak properties
- State machine for nested conflict handling
- Quick string containment checks (optimization)
- Range computation with character offset handling

---

#### Pattern: Document Caching with Debounced Re-scanning
**Where:** `extensions/merge-conflict/src/documentTracker.ts:47-115`
**What:** Implements a cache with debouncing to prevent redundant conflict rescans on rapid edits.
```typescript
export default class DocumentMergeConflictTracker implements vscode.Disposable, interfaces.IDocumentMergeConflictTrackerService {
	private cache: Map<string, ScanTask> = new Map();
	private delayExpireTime: number = 0;

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

	private getConflictsOrEmpty(document: vscode.TextDocument, _origins: string[]): interfaces.IDocumentMergeConflict[] {
		const containsConflict = MergeConflictParser.containsConflict(document);
		if (!containsConflict) { return []; }
		
		const conflicts = MergeConflictParser.scanDocument(document, this.telemetryReporter);
		const key = document.uri.toString();
		if (!this.seenDocumentsWithConflicts.has(key)) {
			this.seenDocumentsWithConflicts.add(key);
			this.telemetryReporter.sendTelemetryEvent('mergeMarkers.documentWithConflictMarkersOpened', {}, {
				conflictCount: conflicts.length,
			});
		}
		return conflicts;
	}

	forget(document: vscode.TextDocument) {
		const key = this.getCacheKey(document);
		if (key) { this.cache.delete(key); }
	}
}
```

**Variations / call-sites:**
- `codelensProvider.ts:53` - Calls tracker.getConflicts()
- `mergeDecorator.ts:178` - Calls tracker.getConflicts()
- `commandHandler.ts:259` - Calls tracker.getConflicts()

**Porting implications:** Requires:
- Multi-origin request deduplication (CodeLens, Decorator, Commands may all request simultaneously)
- Delayer abstraction (debounce/throttle primitive)
- Document URI as stable cache key
- Lazy evaluation of cache tasks
- Manual cache invalidation on edits

---

#### Pattern: Command Registration with Context Variables
**Where:** `extensions/merge-conflict/src/commandHandler.ts:19-51`
**What:** Registers commands with polymorphic handlers (editor-based or resource-based) and context variable updates.
```typescript
begin() {
	this.disposables.push(
		this.registerTextEditorCommand('merge-conflict.accept.current', this.acceptCurrent),
		this.registerTextEditorCommand('merge-conflict.accept.incoming', this.acceptIncoming),
		this.registerTextEditorCommand('merge-conflict.accept.both', this.acceptBoth),
		this.registerTextEditorCommand('merge-conflict.accept.all-current', this.acceptAllCurrent, this.acceptAllCurrentResources),
		this.registerTextEditorCommand('merge-conflict.accept.all-incoming', this.acceptAllIncoming, this.acceptAllIncomingResources),
		this.registerTextEditorCommand('merge-conflict.next', this.navigateNext),
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

async acceptCurrent(editor: vscode.TextEditor, ...args: any[]): Promise<void> {
	return this.accept(interfaces.CommitType.Current, editor, ...args);
}

private async accept(type: interfaces.CommitType, editor: vscode.TextEditor, ...args: any[]): Promise<void> {
	let conflict: interfaces.IDocumentMergeConflict | null;

	if (args[0] === 'known-conflict') {
		conflict = args[1];
	}
	else {
		conflict = await this.findConflictContainingSelection(editor);
	}

	if (!conflict) {
		vscode.window.showWarningMessage(vscode.l10n.t("Editor cursor is not within a merge conflict"));
		return;
	}

	this.tracker.forget(editor.document);
	conflict.commitEdit(type, editor);

	const mergeConflictConfig = vscode.workspace.getConfiguration('merge-conflict');
	if (mergeConflictConfig.get<boolean>('autoNavigateNextConflict.enabled')) {
		this.navigateNext(editor);
	}
}
```

**Variations / call-sites:**
- `codelensProvider.ts:55` - Sets context via executeCommand('setContext', 'mergeConflictsCount', ...)
- `package.json:38-110` - Command declarations with enablement conditions
- `package.json:112-136` - Menu contributions with 'when' expressions

**Porting implications:** Requires:
- Command palette registration
- Context variable system (when expressions)
- Polymorphic command routing (file vs. resource vs. editor context)
- Menu contributions (editor/title, scm/resourceState/context)
- Editor.activeTextEditor access
- Message box dialogs (warning/error/info)

---

#### Pattern: Multi-scheme TextDocumentContentProvider
**Where:** `extensions/merge-conflict/src/contentProvider.ts:8-53`
**What:** Custom content provider for diff view generation using Virtual Document scheme.
```typescript
export default class MergeConflictContentProvider implements vscode.TextDocumentContentProvider, vscode.Disposable {
	static scheme = 'merge-conflict.conflict-diff';

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

**Variations / call-sites:**
- `commandHandler.ts:111-118` - Constructs URI with JSON-serialized ranges
- `commandHandler.ts:147` - Calls vscode.diff with synthetic URIs

**Porting implications:** Requires:
- Virtual filesystem abstraction for non-file content providers
- JSON serialization in URI query parameters
- Position/Range factories
- Diff view command integration
- Error handling with user notifications

---

#### Pattern: Extension Service Composition with Lifecycle Hooks
**Where:** `extensions/merge-conflict/src/services.ts:16-71`
**What:** Centralizes service instantiation and configuration propagation via common interface.
```typescript
export default class ServiceWrapper implements vscode.Disposable {
	private services: vscode.Disposable[] = [];
	private telemetryReporter: TelemetryReporter;

	constructor(private context: vscode.ExtensionContext) {
		const { aiKey } = context.extension.packageJSON as { aiKey: string };
		this.telemetryReporter = new TelemetryReporter(aiKey);
		context.subscriptions.push(this.telemetryReporter);
	}

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

	createExtensionConfiguration(): interfaces.IExtensionConfiguration {
		const workspaceConfiguration = vscode.workspace.getConfiguration(ConfigurationSectionName);
		const codeLensEnabled: boolean = workspaceConfiguration.get('codeLens.enabled', true);
		const decoratorsEnabled: boolean = workspaceConfiguration.get('decorators.enabled', true);

		return {
			enableCodeLens: codeLensEnabled,
			enableDecorations: decoratorsEnabled,
			enableEditorOverview: decoratorsEnabled
		};
	}

	dispose() {
		this.services.forEach(disposable => disposable.dispose());
		this.services = [];
	}
}
```

**Variations / call-sites:**
- `mergeConflictMain.ts:9-14` - Activation entry point
- `package.json:25-26` - Main/browser entry points

**Porting implications:** Requires:
- Extension context API (subscriptions, extension metadata)
- Configuration system with change notifications
- Telemetry reporter initialization
- Duck-typing service interface (begin(), configurationUpdated())
- Lifecycle management via subscription arrays

---

## Summary

Porting the merge-conflict extension from TypeScript/Electron to Tauri/Rust would require implementing 7 major abstraction layers:

1. **Language Provider Registry** — CodeLensProvider interface and multi-scheme registration
2. **Text Decoration Engine** — Theme-aware decoration types with range batching and overview rulers
3. **Document Scanning** — Line-by-line access, range computation, conflict detection
4. **Caching/Debouncing** — Deduplication cache with delayer primitives for multi-origin requests
5. **Command System** — Command palette, context variables, when expressions, polymorphic handlers
6. **Virtual Document System** — Custom content providers with JSON-serialized metadata in URIs
7. **Configuration/Telemetry** — Workspace config reading, change notifications, event reporting

Each layer would require mapping VS Code's TypeScript API surface to Rust equivalents or Tauri IPC calls to a webview/window. The conflict resolution logic itself (state machine for merge markers, edit operations) is language-agnostic, but all IDE integration points depend on VS Code's extension architecture.
