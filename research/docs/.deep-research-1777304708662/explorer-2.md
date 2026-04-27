# Partition 2 of 79 — Findings

## Scope
`extensions/copilot/` (2868 files, 676,837 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# VS Code Core IDE Functionality Port: File Locations in Copilot Extension

## Scope
This index documents files in `extensions/copilot/` (2868 files, 676,837 LOC) that relate to porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust. The Copilot extension is significant because it exercises substantial portions of the VS Code API surface that a Rust host would need to serve.

## Implementation

### Completions & InlineEdit Features
- `extensions/copilot/src/extension/completions/vscode-node/copilotInlineCompletionItemProviderService.ts` — Core provider for inline completions via `registerInlineCompletionItemProvider`
- `extensions/copilot/src/extension/completions/vscode-node/completionsCoreContribution.ts` — Contributions setup for completions feature
- `extensions/copilot/src/extension/completions-core/vscode-node/extension/src/vscodeInlineCompletionItemProvider.ts` — VSCode-specific inline completion provider
- `extensions/copilot/src/extension/inlineEdits/vscode-node/inlineCompletionProvider.ts` — Inline edit completion provider
- `extensions/copilot/src/extension/inlineEdits/vscode-node/jointInlineCompletionProvider.ts` — Joint inline completion handling
- `extensions/copilot/src/extension/inlineEdits/vscode-node/inlineEditProviderFeature.ts` — Feature integration for inline edits

### Language Features & Intelligence
- `extensions/copilot/src/platform/languages/vscode/languageFeaturesServicesImpl.ts` — Language features service implementation using `vscode.languages` API
- `extensions/copilot/src/platform/languages/vscode/languageDiagnosticsServiceImpl.ts` — Diagnostics service for language features
- `extensions/copilot/src/platform/languages/common/languageFeaturesService.ts` — Common base for language feature services
- `extensions/copilot/src/extension/typescriptContext/vscode-node/languageContextService.ts` — TypeScript-specific language context
- `extensions/copilot/src/extension/languageContextProvider/vscode-node/languageContextProviderService.ts` — Language context provider service
- `extensions/copilot/src/extension/diagnosticsContext/vscode/diagnosticsContextProvider.ts` — Diagnostics context provider
- `extensions/copilot/src/platform/languageServer/common/languageContextService.ts` — Language server context base
- `extensions/copilot/src/platform/languageContextProvider/common/languageContextProviderService.ts` — Language context provider base

### Debugging
- `extensions/copilot/src/extension/onboardDebug/vscode/launchConfigService.ts` — Debug launch configuration service
- `extensions/copilot/src/extension/onboardDebug/vscode-node/copilotDebugCommandSession.ts` — Debug command session management
- `extensions/copilot/src/extension/onboardDebug/vscode-node/copilotDebugCommandContribution.ts` — Debug command contributions
- `extensions/copilot/src/extension/prompt/vscode-node/debugCommands.ts` — Debug command handling
- `extensions/copilot/src/platform/debug/common/debugOutputService.ts` — Debug output service base
- `extensions/copilot/src/platform/debug/vscode/debugOutputListener.ts` — VSCode debug output listener
- `extensions/copilot/src/platform/debug/vscode/debugOutputServiceImpl.ts` — VSCode debug output implementation

### Git & Source Control
- `extensions/copilot/src/platform/git/vscode-node/gitServiceImpl.ts` — Git service implementation integrating with vscode.scm
- `extensions/copilot/src/extension/git/vscode/scmContextprovider.ts` — Source control context provider
- `extensions/copilot/src/extension/git/vscode/mergeConflictServiceImpl.ts` — Merge conflict handling service
- `extensions/copilot/src/extension/prompt/vscode-node/gitDiffService.ts` — Git diff service for prompts
- `extensions/copilot/src/extension/prompt/vscode-node/gitCommitMessageServiceImpl.ts` — Git commit message generation
- `extensions/copilot/src/extension/tools/node/scmChangesTool.ts` — Tool for accessing SCM changes

### Terminal
- `extensions/copilot/src/platform/terminal/common/terminalService.ts` — Terminal service base
- `extensions/copilot/src/platform/terminal/vscode/terminalServiceImpl.ts` — VSCode terminal service implementation
- `extensions/copilot/src/extension/xtab/common/terminalOutput.ts` — Terminal output handling
- `extensions/copilot/src/extension/intents/node/terminalIntent.ts` — Terminal-related intent handling
- `extensions/copilot/src/extension/chatSessions/claude/vscode-node/slashCommands/terminalCommand.ts` — Terminal slash command

### Workspace & Navigation
- `extensions/copilot/src/platform/workspace/vscode/workspaceServiceImpl.ts` — Workspace service via `vscode.workspace`
- `extensions/copilot/src/platform/workspace/common/workspaceService.ts` — Common workspace service base
- `extensions/copilot/src/extension/workspaceRecorder/vscode-node/workspaceRecorderFeature.ts` — Workspace change recording
- `extensions/copilot/src/extension/workspaceRecorder/vscode-node/workspaceRecorder.ts` — Workspace change tracking
- `extensions/copilot/src/extension/workspaceChunkSearch/vscode-node/workspaceChunkSearch.contribution.ts` — Workspace indexing contribution
- `extensions/copilot/src/platform/workspaceChunkSearch/node/workspaceChunkSearchService.ts` — Workspace chunk search service

### Editing
- `extensions/copilot/src/platform/editing/common/edit.ts` — Core edit data structure
- `extensions/copilot/src/platform/editing/common/edits.ts` — Edit operations
- `extensions/copilot/src/platform/editing/common/textDocumentSnapshot.ts` — Text document snapshot model
- `extensions/copilot/src/platform/editing/common/notebookDocumentSnapshot.ts` — Notebook document snapshot
- `extensions/copilot/src/extension/inlineEdits/common/editRebase.ts` — Edit conflict resolution

### File System
- `extensions/copilot/src/platform/filesystem/vscode/fileSystemServiceImpl.ts` — Filesystem service via `vscode.workspace.fs`
- `extensions/copilot/src/platform/filesystem/common/fileSystemService.ts` — Common filesystem base
- `extensions/copilot/src/platform/filesystem/common/fileTypes.ts` — File type definitions

### Search & Symbol Navigation
- `extensions/copilot/src/extension/tools/node/searchWorkspaceSymbolsTool.tsx` — Workspace symbol search tool
- `extensions/copilot/src/platform/search/vscode/baseSearchServiceImpl.ts` — Base search service
- `extensions/copilot/src/extension/prompts/node/panel/definitionAtPosition.tsx` — Definition lookup
- `extensions/copilot/src/extension/prompts/node/panel/referencesAtPosition.tsx` — References lookup

### Notebook Support
- `extensions/copilot/src/extension/notebook/vscode-node/notebookFeature.ts` — Notebook feature support
- `extensions/copilot/src/platform/notebook/vscode/notebookServiceImpl.ts` — Notebook service implementation

### Tasks & Testing
- `extensions/copilot/src/platform/tasks/vscode/tasksService.ts` — Task execution service via `vscode.tasks`
- `extensions/copilot/src/platform/testing/vscode/testingServiceImpl.ts` — Testing service
- `extensions/copilot/src/extension/testing/vscode/setupTestContributions.ts` — Test setup contributions

### Dialog & User Interaction
- `extensions/copilot/src/platform/dialog/vscode/dialogServiceImpl.ts` — Dialog service for user prompts
- `extensions/copilot/src/platform/notification/vscode/notificationServiceImpl.ts` — Notification service

### Configuration & State
- `extensions/copilot/src/platform/configuration/vscode/configurationServiceImpl.ts` — Settings/configuration via `vscode.workspace.getConfiguration`
- `extensions/copilot/src/platform/workspaceState/common/workspaceStateService.ts` — Workspace state storage

### Commands
- `extensions/copilot/src/platform/commands/vscode/runCommandExecutionServiceImpl.ts` — Command execution via `vscode.commands.executeCommand`

### Workbench & UI
- `extensions/copilot/src/platform/workbench/vscode/workbenchServiceImpl.ts` — Workbench UI service
- `extensions/copilot/src/platform/tabs/vscode/tabsAndEditorsServiceImpl.ts` — Tab and editor management

## Tests

### Unit Tests
- `extensions/copilot/src/extension/completions-core/vscode-node/lib/src/test/completionState.test.ts` — Completion state tests
- `extensions/copilot/src/extension/inlineEdits/test/common/editRebase.spec.ts` — Edit rebase logic tests
- `extensions/copilot/src/extension/inlineEdits/test/node/debugRecorder.spec.ts` — Debug recording tests
- `extensions/copilot/src/extension/prompt/vscode-node/test/gitDiffService.spec.ts` — Git diff service tests
- `extensions/copilot/src/platform/chat/test/node/hookExecutor.spec.ts` — Hook executor tests

### Simulation/Integration Tests
- `extensions/copilot/test/simulation/language/simulationLanguageFeatureService.ts` — Language feature simulation
- `extensions/copilot/test/simulation/language/tsServerClient.ts` — TypeScript server client mock
- `extensions/copilot/test/simulation/language/lsifLanguageFeatureService.ts` — LSIF language feature mock
- `extensions/copilot/test/simulation/fixtures/vscode/extHost.api.impl.ts` — VSCode extension host API simulation

### Test Fixtures
- `extensions/copilot/test/simulation/fixtures/multiFileEdit/` — Multi-file edit fixtures (3 files)
- `extensions/copilot/test/simulation/fixtures/ghpr/` — GitHub PR fixtures
- `extensions/copilot/test/simulation/fixtures/tests/simple-ts-proj/` — Simple TypeScript project fixture
- `extensions/copilot/test/simulation/fixtures/tests/simple-js-proj/` — Simple JavaScript project fixture

## Types & Interfaces

### API Definitions
- `extensions/copilot/src/extension/vscode-api.d.ts` — VSCode API type references (includes 60+ proposed API types)
- `extensions/copilot/src/extension/api/vscode/api.d.ts` — Copilot extension API surface definition
- `extensions/copilot/src/extension/api/vscode/extensionApi.ts` — Extension API implementation
- `extensions/copilot/src/extension/api/vscode/vscodeContextProviderApi.ts` — Context provider API
- `extensions/copilot/src/extension/githubPullRequest.d.ts` — GitHub PR extension API

### Shared Type Definitions
- `extensions/copilot/src/platform/languages/common/languageFeaturesService.ts` — Language feature types
- `extensions/copilot/src/platform/editing/common/abstractText.ts` — Abstract text interface
- `extensions/copilot/src/platform/chat/common/commonTypes.ts` — Common chat types
- `extensions/copilot/src/util/common/languages.ts` — Language utilities

## Configuration

### Extension Configuration
- `extensions/copilot/src/extension/completions-core/vscode-node/extension/src/config.ts` — Completions configuration
- `extensions/copilot/src/platform/configuration/common/configurationService.ts` — Configuration service interface
- `extensions/copilot/src/platform/customInstructions/common/customInstructionsService.ts` — Custom instructions config

### Extension Manifest
- `extensions/copilot/package.json` — Main extension manifest (not scoped but defines entry points)

## Documentation

### Architecture & Design
- `extensions/copilot/src/extension/trajectory/ARCHITECTURE.md` — Architecture documentation
- `extensions/copilot/src/extension/chatSessions/claude/node/sessionParser/README.md` — Claude session parser documentation
- `extensions/copilot/src/platform/inlineEdits/common/dataTypes/textEditLengthHelper/README.md` — Text edit helper documentation

### Contributed Menu Items & Commands
- Multiple `contribution.ts` files define VSCode UI contributions (menus, commands, keybindings)

## Notable Clusters

### Language Features & Diagnostics (10+ files)
- `extensions/copilot/src/platform/languages/` — Implements `vscode.languages` API surface including completion providers, hover providers, code actions, refactoring, code lens

### Inline Completions & Edits (15+ files)
- `extensions/copilot/src/extension/inlineEdits/` — Complex implementation of `registerInlineCompletionItemProvider` with caching, telemetry, and conflict resolution
- `extensions/copilot/src/extension/completions/` — Completions unification and core contribution layer
- `extensions/copilot/src/extension/completions-core/` — Upstream completions-core submodule integration with language detection and ghost text rendering

### Git Integration (8+ files)
- `extensions/copilot/src/platform/git/` — Git service wrapping VS Code's SCM API
- `extensions/copilot/src/extension/git/` — Git-specific features (conflict resolution, context)
- `extensions/copilot/src/extension/prompt/vscode-node/` — Git data extraction (diffs, commits, branches)

### Chat & Agents (40+ files)
- `extensions/copilot/src/extension/chatSessions/` — Chat session management across multiple providers (vscode, claude, copilotcli)
- `extensions/copilot/src/extension/conversation/` — Conversation features, language model access, participants
- `extensions/copilot/src/extension/agents/` — Agent providers (ask, edit, explore) with custom agent support

### Workspace Analysis (15+ files)
- `extensions/copilot/src/platform/workspaceChunkSearch/` — Workspace-wide code search and semantic indexing
- `extensions/copilot/src/extension/workspaceRecorder/` — Real-time workspace change tracking

### Context & Prompting (20+ files)
- `extensions/copilot/src/extension/context/` — Context resolution for multiple scopes (selection, file, workspace)
- `extensions/copilot/src/extension/prompts/` — Prompt generation with language-specific templates
- `extensions/copilot/src/extension/xtab/` — Cross-tab state and recent file context

### Platform Abstraction Layer (80+ service files)
- `extensions/copilot/src/platform/` — Contains 73 subdirectories abstracting VS Code APIs into platform services:
  - authentication, chat, configuration, debug, filesystem, git, languages, terminal, workspace, etc.
  - Each typically has common/, vscode-node/ (Electron impl), and test/ subdirectories
  - 429 test files across the extension demonstrate expected behaviors for API surface

## Summary

The Copilot extension (2,868 files) exercises extensive VS Code API surface through a careful platform abstraction layer. Key findings:

**IDE Core Features Exercised:**
- Inline completions (registerInlineCompletionItemProvider)
- Language diagnostics and quick fixes (vscode.languages API)
- Git integration (vscode.scm, merge conflict handling)
- Debug launch configuration (vscode.debug)
- Terminal interaction (vscode.terminal)
- Workspace file system and symbol navigation
- Notebook document support
- Test discovery and execution
- Task running
- Configuration management
- Command palette and keybindings

**Architecture Pattern:**
The extension uses a three-tier architecture:
1. Common interfaces/types in `*/common/` directories
2. VSCode-specific implementations in `*/vscode*/` directories  
3. Node-specific logic in `*/node/` directories
4. Test mocks and fixtures in `*/test/` directories

This structure demonstrates how a Rust host would need to surface equivalent capabilities. The 80+ platform services show the breadth of IDE functionality being wrapped—from low-level file I/O to high-level workbench UI state management.

**API Surface Expansion:**
The extension declares 60+ VSCode proposed API dependencies in vscode-api.d.ts, including several relevant to core IDE:
- Chat APIs (participantAdditions, sessionProvider, hooks)
- Language model tools and capabilities
- Terminal extensions (dataWriteEvent, executeCommandEvent)
- Inline completions additions
- Debug contributions
- Task execution terminal
- Mapped edits provider
- Test observer

This breadth indicates VS Code's API surface continues expanding beyond stable APIs, suggesting a Rust port would require ongoing alignment with proposal stages.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
