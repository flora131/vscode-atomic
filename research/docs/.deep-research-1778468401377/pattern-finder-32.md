# VS Code Merge-Conflict Extension: Porting Patterns & API Usage

**Research Partition**: 32 of 80  
**Scope**: `extensions/merge-conflict/` (13 files, 1,463 LOC)  
**Focus**: CodeLens API usage and core IDE functionality patterns used in VS Code extensions

---

## Core Patterns Found

#### Pattern: CodeLens Provider Registration
**Where:** `extensions/merge-conflict/src/codelensProvider.ts:100-107`
**What:** Registers a CodeLens provider for multiple document schemes with dynamic lifecycle management.
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
- `codelensProvider.ts:21-23` - Registration during `begin()` initialization
- `codelensProvider.ts:28-34` - Dynamic re-registration on configuration change
- Implementation disposal: `codelensProvider.ts:40-45`

---

#### Pattern: CodeLens Provision & Command Binding
**Where:** `extensions/merge-conflict/src/codelensProvider.ts:47-98`
**What:** Implements `vscode.CodeLensProvider` interface to provide code lenses with associated commands for each conflict range.
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
		// ... more commands
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
**Variations / call-sites:** Single implementation in extension; each conflict may have 4 associated code lenses

---

#### Pattern: Command Handler Registration
**Where:** `extensions/merge-conflict/src/commandHandler.ts:28-51`
**What:** Registers text editor commands with dual callback support for editor context and resource URI arguments.
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
**Variations / call-sites:** 10 commands registered with polymorphic callback dispatch

---

#### Pattern: Text Decoration Type Management
**Where:** `extensions/merge-conflict/src/mergeDecorator.ts:55-125`
**What:** Creates and manages themed text editor decoration types with color/style overrides, stored by key for reuse.
```typescript
private registerDecorationTypes(config: interfaces.IExtensionConfiguration) {
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
- `mergeDecorator.ts:23-26` - Initialization in `begin()`
- `mergeDecorator.ts:44-53` - Re-registration on config change
- Applied to editors via: `mergeDecorator.ts:225-231`

---

#### Pattern: Document Event Listener Registration
**Where:** `extensions/merge-conflict/src/mergeDecorator.ts:23-42`
**What:** Subscribes to workspace/window events with callback handlers, storing disposables in context subscriptions.
```typescript
begin(config: interfaces.IExtensionConfiguration) {
	this.config = config;
	this.registerDecorationTypes(config);

	vscode.window.visibleTextEditors.forEach(e => this.applyDecorations(e));

	vscode.workspace.onDidOpenTextDocument(event => {
		this.applyDecorationsFromEvent(event);
	}, null, this.context.subscriptions);

	vscode.workspace.onDidChangeTextDocument(event => {
		this.applyDecorationsFromEvent(event.document);
	}, null, this.context.subscriptions);

	vscode.window.onDidChangeVisibleTextEditors((e) => {
		e.forEach(e => this.applyDecorations(e));
	}, null, this.context.subscriptions);
}
```
**Variations / call-sites:**
- `codelensProvider.ts` - No explicit event subscriptions (relies on provider callbacks)
- `services.ts:46-52` - Workspace configuration change monitoring

---

#### Pattern: Content Provider Registration
**Where:** `extensions/merge-conflict/src/contentProvider.ts:15-19`
**What:** Registers a custom URI scheme content provider for displaying synthetic documents (diff views).
```typescript
begin() {
	this.context.subscriptions.push(
		vscode.workspace.registerTextDocumentContentProvider(MergeConflictContentProvider.scheme, this)
	);
}
```
**Implementation interface:** Implements `vscode.TextDocumentContentProvider` with `provideTextDocumentContent(uri)` method
- Provider supplies text content for custom URIs: `contentProvider.ts:24-53`
- Used by diff command: `commandHandler.ts:111-118` creates custom URIs with JSON query parameters

---

#### Pattern: Service Lifecycle & Configuration Management
**Where:** `extensions/merge-conflict/src/services.ts:27-53`
**What:** Centralizes service initialization, configuration propagation, and lifecycle events through a ServiceWrapper.
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
**Configuration pattern:**
```typescript
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
```
**Variations / call-sites:**
- All services implement optional `begin(config)` and `configurationUpdated(config)` hooks
- Entry point: `mergeConflictMain.ts:9-14` activates via `activate(context)` extension API

---

## API Surface Summary

### vscode.languages API
- **`registerCodeLensProvider()`** - Bind CodeLens provider to document schemes
- **`CodeLensProvider` interface** - Implement `provideCodeLenses(document, token)`

### vscode.commands API
- **`registerCommand(id, callback)`** - Register command with variadic arguments
- **`executeCommand(command, ...args)`** - Execute command (used for `setContext`)

### vscode.window API
- **`visibleTextEditors`** - Array of currently visible editors
- **`activeTextEditor`** - Current editor with focus
- **`createTextEditorDecorationType(options)`** - Create reusable decoration style
- **`onDidChangeVisibleTextEditors()`** - Event fired when editor visibility changes

### vscode.workspace API
- **`registerTextDocumentContentProvider(scheme, provider)`** - Register custom URI scheme handler
- **`onDidOpenTextDocument()`** - Event for new documents
- **`onDidChangeTextDocument()`** - Event for document edits
- **`onDidChangeConfiguration()`** - Event for settings changes
- **`getConfiguration(section)`** - Read workspace/user settings
- **`openTextDocument(uri)`** - Open document by URI
- **`applyEdit(workspaceEdit)`** - Apply bulk edits across workspace

### vscode.TextDocument API
- **`lineAt(line)`** - Get TextLine at index
- **`getText(range?)`** - Extract text
- **`lineCount`** - Line count

### vscode.TextEditor API
- **`edit(callback)`** - Apply edits to document
- **`setDecorations(type, ranges)`** - Apply decoration type to ranges
- **`revealRange(range, type)`** - Scroll editor to show range
- **`selection`** - Current cursor selection

### vscode.Range & vscode.Position API
- **Constructors**: `new vscode.Range(startLine, startChar, endLine, endChar)`
- **Methods**: `contains()`, `isEqual()`, `start`, `end`

---

## Key Extension Patterns

1. **Provider Dual-Dispatch**: Commands support both editor context and resource URI context (commandHandler)
2. **Cached Conflict Detection**: Document merge conflict parsing cached with delayer to debounce updates (documentTracker)
3. **Decoration Type Pooling**: Decoration types created once, reused across all editors (mergeDecorator)
4. **Configuration Push Pattern**: Services implement `configurationUpdated()` hook for reactive config changes
5. **Custom URI Schemes**: Synthetic documents for diff views use JSON-encoded query parameters
6. **Telemetry Integration**: TelemetryReporter passed to core services for usage tracking

---

## Porting Implications for Tauri/Rust

**UI/Decoration Layer**: Complex theme color resolution and decoration API would require:
- Tauri webview CSS-in-JS or style injection layer
- Theme color token resolution system
- Range-to-coordinate mapping for decoration rendering

**CodeLens/Commands**: Would need equivalent IPC layer:
- Command registration -> RPC handler mapping
- CodeLens provider callbacks -> Async request/response protocol
- Context variable management system

**Document/Event Streaming**: Real-time conflict detection requires:
- Efficient document diff computation
- Event subscription mechanism over IPC
- Debounced update strategy (Delayer pattern used here)

**Content Providers**: Custom URI schemes need:
- URI scheme routing layer
- Synthetic document generation protocol
- Query parameter handling in Rust layer

**Configuration**: Settings propagation pattern would map to:
- Workspace/User config file reading
- Change notification system
- Reactive service initialization

