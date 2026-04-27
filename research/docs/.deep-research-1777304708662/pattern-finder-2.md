# VS Code API Contract Patterns in Copilot Extension

## Research Scope
**Directory**: `extensions/copilot/` (2868 files, 676,837 LOC)
**Focus**: API contract expression patterns for inline completion providers and context management

---

## API Contract Patterns

### Pattern 1: Inline Completion Item Provider Registration

**Where:** `extensions/copilot/src/extension/inlineEdits/vscode-node/inlineEditProviderFeature.ts:165-171`

**What:** Core pattern for registering inline completion providers via VS Code's languages API with configuration options.

```typescript
reader.store.add(languages.registerInlineCompletionItemProvider('*', provider, {
	displayName: provider.displayName,
	yieldTo: this._yieldToCopilot.read(reader) ? ['github.copilot'] : undefined,
	debounceDelayMs: 0, // set 0 debounce to ensure consistent delays/timings
	groupId: 'nes',
	excludes,
}));
```

**Variations / call-sites:**
- `extensions/copilot/src/extension/completions/vscode-node/completionsCoreContribution.ts:39-48`: Alternative with pattern selector and groupId 'completions'
- `extensions/copilot/src/extension/inlineEdits/vscode-node/jointInlineCompletionProvider.ts:238-246`: Nested provider registration with yieldTo priority
- `extensions/copilot/src/extension/typescriptContext/vscode-node/languageContextService.ts:1639`: Language-specific TypeScript registration

---

### Pattern 2: Commands Registration and Execution

**Where:** `extensions/copilot/src/extension/inlineEdits/vscode-node/inlineEditProviderFeature.ts:173-213`

**What:** Pattern for registering extension commands and executing built-in vscode commands, combining both command creation and context execution.

```typescript
reader.store.add(commands.registerCommand(learnMoreCommandId, () => {
	this._envService.openExternal(URI.parse(learnMoreLink));
}));

reader.store.add(commands.registerCommand(clearCacheCommandId, () => {
	model.nextEditProvider.clearCache();
}));

// Context setting via command execution
void commands.executeCommand('setContext', useEnhancedNotebookNESContextKey, enableEnhancedNotebookNES);
void commands.executeCommand('setContext', 'github.copilot.inlineEditsEnabled', enabled);
```

**Variations / call-sites:**
- `extensions/copilot/src/extension/conversation/vscode-node/conversationFeature.ts:226-232`: Complex command registration with async handlers
- `extensions/copilot/src/extension/conversation/vscode-node/conversationFeature.ts:256-272`: Commands with external resource operations (vscode.open, workbench actions)
- `extensions/copilot/src/platform/review/vscode/reviewServiceImpl.ts:44-59`: Context-only pattern for configuration-driven context updates

---

### Pattern 3: Language Provider Interface Implementation

**Where:** `extensions/copilot/src/extension/codeBlocks/vscode-node/provider.ts:49-66`

**What:** Implementing multiple VS Code provider interfaces (DefinitionProvider, HoverProvider, etc.) with async method signatures accepting documents, positions, and cancellation tokens.

```typescript
class CodeBlockIntelliSenseProvider implements vscode.DefinitionProvider, vscode.ImplementationProvider, vscode.TypeDefinitionProvider, vscode.HoverProvider {
	async provideDefinition(document: vscode.TextDocument, position: vscode.Position, token: vscode.CancellationToken): Promise<vscode.LocationLink[] | undefined> {
		return this.goTo('vscode.experimental.executeDefinitionProvider_recursive', document, position, token);
	}

	async provideImplementation(document: vscode.TextDocument, position: vscode.Position, token: vscode.CancellationToken): Promise<vscode.LocationLink[] | undefined> {
		return this.goTo('vscode.experimental.executeImplementationProvider_recursive', document, position, token);
	}

	async provideTypeDefinition(document: vscode.TextDocument, position: vscode.Position, token: vscode.CancellationToken): Promise<vscode.LocationLink[] | undefined> {
		return this.goTo('vscode.experimental.executeTypeDefinitionProvider_recursive', document, position, token);
	}

	async provideHover(document: vscode.TextDocument, position: vscode.Position, token: vscode.CancellationToken): Promise<vscode.Hover | undefined> {
		// Implementation
	}
}
```

**Variations / call-sites:**
- `extensions/copilot/src/extension/inlineEdits/vscode-node/inlineCompletionProvider.ts:1-100`: InlineCompletionItemProvider with complex NesCompletionList and NesCompletionItem types
- `extensions/copilot/src/extension/inlineChat/vscode-node/inlineChatCommands.ts:298-301`: CodeActionsProvider registration for multiple languages

---

### Pattern 4: Extension Contribution Pattern with Disposables

**Where:** `extensions/copilot/src/extension/conversation/vscode-node/conversationFeature.ts:50-100`

**What:** Base pattern for IExtensionContribution with DisposableStore for lifecycle management, constructor injection via @ServiceId decorators, and activation blocking.

```typescript
export class ConversationFeature implements IExtensionContribution {
	private readonly _disposables = new DisposableStore();
	private readonly _activatedDisposables = new DisposableStore();
	public _enabled;
	private _activated;

	readonly id = 'conversationFeature';
	readonly activationBlocker?: Promise<void>;

	constructor(
		@IInstantiationService private instantiationService: IInstantiationService,
		@ILogService private readonly logService: ILogService,
		@IConfigurationService private configurationService: IConfigurationService,
		// ... more service injections
	) {
		this._enabled = false;
		this._activated = false;
		this.registerCopilotTokenListener();
		
		const activationBlockerDeferred = new DeferredPromise<void>();
		this.activationBlocker = activationBlockerDeferred.p;
		if (authenticationService.copilotToken) {
			this.activated = true;
			activationBlockerDeferred.complete();
		}
	}
}
```

**Variations / call-sites:**
- `extensions/copilot/src/extension/inlineEdits/vscode-node/inlineEditProviderFeature.ts:37-54`: Simpler contribution pattern extending Disposable
- `extensions/copilot/src/extension/completions/vscode-node/completionsCoreContribution.ts:16-65`: Contribution using autorun with reader/store pattern
- `extensions/copilot/src/extension/byok/vscode-node/byokContribution.ts:25+`: BYOKContrib extends Disposable pattern

---

### Pattern 5: Context-Driven Reactive Registration

**Where:** `extensions/copilot/src/extension/completions/vscode-node/completionsCoreContribution.ts:30-64`

**What:** Using observables and autorun for reactive provider registration based on state changes (unification state, configuration, authentication).

```typescript
this._register(autorun(reader => {
	const unificationStateValue = unificationState.read(reader);
	const configEnabled = configurationService.getExperimentBasedConfigObservable<boolean>(ConfigKey.TeamInternal.InlineEditsEnableGhCompletionsProvider, experimentationService).read(reader);
	const extensionUnification = unificationStateValue?.extensionUnification ?? false;

	let hasInstantiatedProvider = false;
	if (unificationStateValue?.codeUnification || extensionUnification || configEnabled || this._copilotToken.read(reader)?.isNoAuthUser) {
		const provider = _copilotInlineCompletionItemProviderService.getOrCreateProvider();
		reader.store.add(
			languages.registerInlineCompletionItemProvider(
				{ pattern: '**' },
				provider,
				{
					debounceDelayMs: 0,
					excludes: ['github.copilot'],
					groupId: 'completions'
				}
			)
		);
		hasInstantiatedProvider = true;
	}

	void commands.executeCommand('setContext', 'github.copilot.extensionUnification.activated', extensionUnification);
}));
```

**Variations / call-sites:**
- `extensions/copilot/src/extension/inlineEdits/vscode-node/inlineEditProviderFeature.ts:109-215`: Multi-state reactive pattern with derived observables
- `extensions/copilot/src/platform/configuration/vscode/configurationServiceImpl.ts:32`: Configuration change listener pattern
- `extensions/copilot/src/platform/workspace/vscode/workspaceServiceImpl.ts:29-34`: Event-based reactive patterns

---

### Pattern 6: API Surface Exposure via Re-exports

**Where:** `extensions/copilot/src/vscodeTypes.ts:1-126`

**What:** Typed re-export pattern for VS Code types, creating a centralized type contract for the extension. Covers chat, language models, inline completions, and workspace types.

```typescript
import * as vscode from 'vscode';

export import Position = vscode.Position;
export import Range = vscode.Range;
export import Selection = vscode.Selection;
export import EventEmitter = vscode.EventEmitter;
export import CancellationTokenSource = vscode.CancellationTokenSource;
export import Diagnostic = vscode.Diagnostic;
export import TextEdit = vscode.TextEdit;
export import WorkspaceEdit = vscode.WorkspaceEdit;
export import Uri = vscode.Uri;
// ... 100+ more type re-exports
export import ChatResponseFileTreePart = vscode.ChatResponseFileTreePart;
export import LanguageModelToolInformation = vscode.LanguageModelToolInformation;
export import InlineCompletionItem = vscode.InlineCompletionItem;
```

**Variations / call-sites:**
- `extensions/copilot/src/extension/api/vscode/extensionApi.ts:1-32`: Custom API definition pattern with interface wrapping
- `extensions/copilot/src/extension/api/vscode/vscodeContextProviderApi.ts`: API wrapper providing additional context provider methods
- `extensions/copilot/src/lib/node/chatLibMain.ts:177-203`: Internal interface definitions extending vscode types

---

### Pattern 7: Event Subscription and Listener Management

**Where:** `extensions/copilot/src/extension/conversation/vscode-node/newWorkspaceFollowup.ts:21-64`

**What:** Workspace and window API event subscription pattern with registered file system providers and text document content providers.

```typescript
workspace.registerFileSystemProvider(CopilotWorkspaceScheme,
	this.instantiationService.createInstance(CopilotFileSystemProvider),
	{ isCaseSensitive: true }
);
workspace.registerFileSystemProvider(GithubWorkspaceScheme,
	this.instantiationService.createInstance(GithubFileSystemProvider),
	{ isCaseSensitive: true }
);
workspace.registerTextDocumentContentProvider(CopilotFileScheme, copilotTextDocumentProvider);

// Document operations
const document = await workspace.openTextDocument(Uri.parse(pathStr));
await window.showTextDocument(document, { preview: false });
const content = await workspace.fs.readFile(Uri.joinPath(fileTreePart.baseUri, file));
await workspace.fs.writeFile(fileUri, content);
await workspace.fs.createDirectory(Uri.joinPath(fileUri, '..'));
```

**Variations / call-sites:**
- `extensions/copilot/src/platform/workspace/vscode/workspaceServiceImpl.ts:29-34`: Workspace event listening pattern
- `extensions/copilot/src/extension/conversation/vscode-node/feedbackCollection.ts:35`: onDidChangeTextDocument pattern
- `extensions/copilot/src/extension/conversation/vscode-node/logWorkspaceState.ts:30-52`: window.showInputBox and workspace.fs patterns

---

## Key Observations

1. **Disposable Pattern**: All VS Code API registrations return Disposable objects that are tracked via DisposableStore or reader.store.add() for automatic cleanup.

2. **Observable/Reactive Pattern**: Heavy use of observables and autorun for reactive provider registration based on authentication, configuration, and unification state.

3. **Service Injection**: Constructor-based dependency injection with @ServiceId decorators enables loose coupling and testability.

4. **Type Safety**: Centralized type re-exports in vscodeTypes.ts maintain contract consistency across 2868 files.

5. **Command/Context API**: Context is managed through `commands.executeCommand('setContext', key, value)` for UI state synchronization.

6. **Provider Interfaces**: Language providers implement well-defined interfaces (DefinitionProvider, HoverProvider, etc.) with consistent async/CancellationToken patterns.

7. **File System Abstraction**: Custom file system providers abstract virtual workspaces through standard workspace.registerFileSystemProvider API.

8. **Contribution Pattern**: Extensions implement IExtensionContribution with explicit lifecycle management (activationBlocker, id property).

