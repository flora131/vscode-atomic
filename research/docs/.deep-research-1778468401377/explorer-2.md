# Partition 2 of 80 — Findings

## Scope
`extensions/copilot/` (2908 files, 694,728 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Copilot Extension (`extensions/copilot/`) - File Locations for VS Code Core IDE Porting Research

## Summary
The Copilot extension (695k LOC across 2908 files) is a downstream consumer of VS Code's API surface. It abstracts platform interactions through ~548 TypeScript files in `src/platform/` directory with a consistent pattern: `common/` (interface definitions) → `vscode/` or `vscode-node/` (VS Code implementation) → optional `node/` (Node.js implementation). This structure reveals the scope of IDE APIs needed for a hypothetical Tauri/Rust port.

## Implementation

### Platform Service Abstractions (Core IDE Dependencies)
These files define interfaces that abstract VS Code IDE functionality:

- `extensions/copilot/src/platform/terminal/common/terminalService.ts` — Terminal access interface
- `extensions/copilot/src/platform/languages/common/languageFeaturesService.ts` — Language intelligence (definitions, refs, symbols)
- `extensions/copilot/src/platform/languages/vscode/languageFeaturesServicesImpl.ts` — Delegates to vscode.commands for definition providers
- `extensions/copilot/src/platform/git/vscode-node/gitServiceImpl.ts` — Git source control integration
- `extensions/copilot/src/platform/workspace/vscode/workspaceServiceImpl.ts` — Workspace folder access
- `extensions/copilot/src/platform/tabs/vscode/tabsAndEditorsServiceImpl.ts` — Editor tab management
- `extensions/copilot/src/platform/configuration/vscode/configurationServiceImpl.ts` — Settings/configuration
- `extensions/copilot/src/platform/commands/vscode/runCommandExecutionServiceImpl.ts` — Command execution
- `extensions/copilot/src/platform/tasks/vscode/tasksService.ts` — Task execution (11 vscode.task references)
- `extensions/copilot/src/platform/debug/vscode/debugOutputServiceImpl.ts` — Debug console output
- `extensions/copilot/src/platform/filesystem/vscode/fileSystemServiceImpl.ts` — File system operations
- `extensions/copilot/src/platform/dialog/vscode/dialogServiceImpl.ts` — UI dialogs
- `extensions/copilot/src/platform/notification/vscode/notificationServiceImpl.ts` — Notifications
- `extensions/copilot/src/platform/review/vscode/reviewServiceImpl.ts` — Code review integration (12 vscode refs)
- `extensions/copilot/src/platform/notebook/vscode/notebookServiceImpl.ts` — Notebook operations (7 references)
- `extensions/copilot/src/platform/notebook/vscode/notebookExectionServiceImpl.ts` — Notebook execution
- `extensions/copilot/src/platform/search/vscode/baseSearchServiceImpl.ts` — Text search
- `extensions/copilot/src/platform/survey/vscode/surveyServiceImpl.ts` — User survey interactions
- `extensions/copilot/src/platform/terminal/vscode/terminalServiceImpl.ts` — Terminal operations (window.terminals, shell integration)
- `extensions/copilot/src/platform/workbench/vscode/workbenchServiceImpt.ts` — Workbench state
- `extensions/copilot/src/platform/mcp/vscode/mcpServiceImpl.ts` — MCP server integration

### Language & Debug Context Services
- `extensions/copilot/src/extension/typescriptContext/vscode-node/languageContextService.ts` — TypeScript context provider (5 vscode refs)
- `extensions/copilot/src/extension/typescriptContext/vscode-node/nesRenameService.ts` — Rename refactoring (7 references)
- `extensions/copilot/src/extension/onboardDebug/vscode/launchConfigService.ts` — Debug launch config
- `extensions/copilot/src/extension/onboardDebug/vscode-node/copilotDebugCommandSession.ts` — Debug session management (3 vscode refs)

### Editor & Inline Completion Features
- `extensions/copilot/src/extension/inlineEdits/vscode-node/jointInlineCompletionProvider.ts` — Inline completion provider (9 executeCommand calls)
- `extensions/copilot/src/extension/inlineEdits/vscode-node/components/expectedEditCaptureController.ts` — Edit capture (3 vscode refs)
- `extensions/copilot/src/extension/inlineEdits/vscode-node/inlineEditProviderFeature.ts` — Inline edit feature (10 calls)
- `extensions/copilot/src/extension/completions-core/vscode-node/completionsServiceBridges.ts` — Completions bridges (9 executeCommand refs)

### Chat & Conversation Features
- `extensions/copilot/src/extension/conversation/vscode-node/conversationFeature.ts` — Core conversation feature (18 executeCommand calls)
- `extensions/copilot/src/extension/conversation/vscode-node/chatParticipants.ts` — Chat participant registration (2 vscode refs)
- `extensions/copilot/src/extension/conversation/vscode-node/feedbackReporter.ts` — Feedback collection (1 ref)
- `extensions/copilot/src/extension/conversation/vscode-node/terminalFixGenerator.ts` — Terminal command suggestions from chat
- `extensions/copilot/src/extension/chatSessions/vscode-node/copilotCLIChatSessions.ts` — Chat sessions (31 executeCommand refs)

### Workspace & File Operations
- `extensions/copilot/src/extension/workspaceRecorder/vscode-node/workspaceRecorderFeature.ts` — Workspace mutation tracking (4 refs)
- `extensions/copilot/src/extension/workspaceChunkSearch/vscode-node/workspaceIndexingStatus.ts` — Workspace indexing UI (3 refs)
- `extensions/copilot/src/extension/getting-started/vscode-node/newWorkspaceInitializer.ts` — Workspace initialization (2 refs)
- `extensions/copilot/src/extension/tools/node/editFileToolUtils.tsx` — File editing tools
- `extensions/copilot/src/extension/tools/node/editNotebookTool.tsx` — Notebook editing

### Code Navigation & Linkification
- `extensions/copilot/src/extension/linkify/vscode-node/symbolLinkifier.ts` — Symbol linking
- `extensions/copilot/src/extension/linkify/vscode-node/findSymbol.ts` — Symbol finding
- `extensions/copilot/src/extension/linkify/vscode-node/findWord.ts` — Word finding

### Terminal & Process Integration
- `extensions/copilot/src/extension/chatSessions/vscode-node/copilotCLITerminalIntegration.ts` — Terminal integration
- `extensions/copilot/src/extension/prompt/vscode-node/debugCommands.ts` — Debug commands (3 vscode refs)

### Proposed API Usage
- `extensions/copilot/src/extension/vscode-api.d.ts` — Custom Copilot API definitions
- `extensions/copilot/src/extension/api/vscode/api.d.ts` — Main extension API surface (selectScope method)
- `extensions/copilot/src/extension/prompts/node/test/fixtures/vscode.proposed.chatParticipantAdditions.d.ts` — Proposed chat API

## Tests

### Platform Service Tests (80+ files)
- `extensions/copilot/src/platform/terminal/test/` — Terminal service tests
- `extensions/copilot/src/platform/languages/test/` — Language features tests
- `extensions/copilot/src/platform/git/test/node/gitService.spec.ts` — Git integration tests
- `extensions/copilot/src/platform/tasks/vscode/test/tasksService.spec.ts` — Task execution tests
- `extensions/copilot/src/platform/configuration/vscode/test/configurationServiceImpl.spec.ts` — Configuration tests
- `extensions/copilot/src/platform/filesystem/test/` — File system tests
- `extensions/copilot/src/platform/workspace/test/` — Workspace tests
- `extensions/copilot/src/platform/notebook/test/` — Notebook tests
- `extensions/copilot/src/platform/search/test/` — Search tests
- `extensions/copilot/src/platform/debug/test/` — Debug output tests

### Extension Feature Tests (392+ spec/test files)
- `extensions/copilot/src/extension/conversation/vscode-node/test/conversationFeature.test.ts` — Conversation feature
- `extensions/copilot/src/extension/chatSessions/vscode-node/test/copilotCLIChatSessions.spec.ts` — Chat sessions (31 command refs)
- `extensions/copilot/src/extension/inlineEdits/vscode-node/test/` — Inline edits tests
- `extensions/copilot/src/extension/linkify/test/vscode-node/symbolLinkifier.test.ts` — Symbol linkification
- `extensions/copilot/src/extension/workspaceRecorder/vscode-node/test/` — Workspace recorder

### Simulation Tests (test/simulation/)
- `extensions/copilot/test/simulation/fixtures/` — Comprehensive test fixtures (150+ files)
- Editor, generation, review, editing, multi-file scenarios

## Types / Interfaces

### Type Definitions
- `extensions/copilot/src/extension/api/vscode/api.d.ts` — Copilot extension API interface
- `extensions/copilot/src/extension/vscode-api.d.ts` — VS Code API type declarations
- `extensions/copilot/src/extension/githubPullRequest.d.ts` — GitHub PR types
- `extensions/copilot/src/util/vs/base/common/observableInternal/logging/debugger/debuggerApi.d.ts` — Debugger API types
- `extensions/copilot/src/util/vs/crypto.d.ts` — Crypto utilities
- `extensions/copilot/src/util/vs/vscode-globals-product.d.ts` — Product globals
- `extensions/copilot/src/util/vs/vscode-globals-nls.d.ts` — NLS globals
- `extensions/copilot/src/util/vs/base-common.d.ts` — Base common utilities

### Service Interfaces
- `extensions/copilot/src/platform/terminal/common/terminalService.ts` — ITerminalService interface
- `extensions/copilot/src/platform/languages/common/languageFeaturesService.ts` — ILanguageFeaturesService
- `extensions/copilot/src/platform/git/common/gitService.ts` — IGitService
- `extensions/copilot/src/platform/workspace/common/workspaceService.ts` — IWorkspaceService
- `extensions/copilot/src/platform/tasks/common/tasksService.ts` — ITasksService
- `extensions/copilot/src/platform/debug/common/debugService.ts` — Debug service interfaces
- `extensions/copilot/src/platform/notebook/common/notebookService.ts` — INotebookService

## Configuration

- `extensions/copilot/package.json` — Extension manifest with 90+ proposed API requirements
- `extensions/copilot/tsconfig.json` — TypeScript configuration
- `extensions/copilot/tsconfig.base.json` — Base TypeScript config
- `extensions/copilot/tsconfig.worker.json` — Worker TypeScript config
- `extensions/copilot/cgmanifest.json` — Component governance manifest
- `extensions/copilot/tsfmt.json` — TypeScript formatting
- `extensions/copilot/chat-lib/` — Separate chat library package

## Documentation

- `extensions/copilot/README.md` — Extension overview
- `extensions/copilot/chat-lib/README.md` — Chat library documentation
- `extensions/copilot/src/extension/chatSessions/claude/node/sessionParser/README.md` — Claude session parser
- `extensions/copilot/src/platform/inlineEdits/common/dataTypes/textEditLengthHelper/README.md` — Text edit helper
- `extensions/copilot/test/simulation/tools/README.md` — Simulation tools

## Notable Clusters

### Platform Service Implementations (548 files across 40+ directories)
Each major IDE feature has abstraction layers at `src/platform/{feature}/`:
- **common/** — Interface definitions (Platform-agnostic contracts)
- **vscode/** or **vscode-node/** — VS Code implementation using vscode module
- **node/** — Optional Node.js-specific implementation
- **test/** — Service-level tests

Directory structure reveals porting scope:
- `extensions/copilot/src/platform/terminal/` — 3 files (Terminal window, shell integration, buffer)
- `extensions/copilot/src/platform/languages/` — 5 files (Definitions, diagnostics, code lenses)
- `extensions/copilot/src/platform/git/` — 12 files (Git status, diffs, blame, extensions)
- `extensions/copilot/src/platform/tasks/` — 3 files (Task execution)
- `extensions/copilot/src/platform/debug/` — 3 files (Debug output, launch configs)
- `extensions/copilot/src/platform/filesystem/` — 5 files (File I/O, virtual documents)
- `extensions/copilot/src/platform/workspace/` — 3 files (Workspace folders, state)
- `extensions/copilot/src/platform/tabs/` — 2 files (Editor tabs, active editor)
- `extensions/copilot/src/platform/notebook/` — 5 files (Notebook execution, cells)
- `extensions/copilot/src/platform/commands/` — 3 files (Command execution)

### Extension Features (2,328 source files)
- `extensions/copilot/src/extension/conversation/vscode-node/` — Chat conversation (18+ executeCommand calls)
- `extensions/copilot/src/extension/chatSessions/` — Session management (31+ executeCommand refs in CLI variant)
- `extensions/copilot/src/extension/inlineEdits/vscode-node/` — Inline completions (9+ calls)
- `extensions/copilot/src/extension/completions-core/vscode-node/` — Code completions bridge (9+ calls)
- `extensions/copilot/src/extension/tools/` — Tool execution for file/terminal operations
- `extensions/copilot/src/extension/onboardDebug/` — Debug workflow integration

### Enabled Proposed APIs (package.json)
Extension requires 54 proposed API features:
- Chat: `chatHooks`, `chatDebug`, `chatProvider`, `chatParticipantAdditions`, `defaultChatParticipant`, `chatSessionsProvider`, `chatSessionCustomizationProvider`, `chatStatusItem`, `chatInputNotification`
- Editing: `mappedEditsProvider`, `inlineCompletionsAdditions`, `codeActionAI`, `textSearchProvider`, `textSearchProvider2`, `findTextInFiles`, `findTextInFiles2`, `findFiles2`, `aiTextSearchProvider`
- Debugging: `contribDebugCreateConfiguration`, `taskExecutionTerminal`
- Terminal: `terminalDataWriteEvent`, `terminalExecuteCommandEvent`, `terminalSelection`, `terminalQuickFixProvider`
- Language: `languageModelToolSupportsModel`, `languageModelSystem`, `languageModelCapabilities`, `languageModelPricing`, `languageModelThinkingPart`
- Others: `embeddings`, `aiRelatedInformation`, `aiSettingsSearch`, `workspaceTrust`, `environmentPower`, `terminalTitle`, `testObserver`, `newSymbolNamesProvider`, `agentSessionsWorkspace`, `agentsWindowConfiguration`

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Analysis: Copilot Extension Integration with VS Code Core IDE APIs

## Research Question
What it would take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

## Key Finding
The Copilot extension (`extensions/copilot/`) cleanly depends **only on the public `vscode` API surface** (`vscode.d.ts`). No imports from internal `vs/` modules found in actual extension code. All interactions with VS Code core functionality follow the published extension API contract.

---

## Pattern Examples

#### Pattern 1: Editor State & Tabs Management via API
**Where:** `src/platform/tabs/vscode/tabsAndEditorsServiceImpl.ts:1-80`
**What:** Extension accesses active editor and tab group changes through vscode.window public API events and state.

```typescript
import * as vscode from 'vscode';

export class TabsAndEditorsServiceImpl implements ITabsAndEditorsService {
  private readonly _tabGroupsUseInfo = new Map<vscode.TabGroup, number>();
  private _tabClock: number = 0;

  readonly onDidChangeActiveTextEditor: vscode.Event<vscode.TextEditor | undefined> = 
    vscode.window.onDidChangeActiveTextEditor;
  
  constructor() {
    const updateActiveTabGroup = () => 
      this._tabGroupsUseInfo.set(vscode.window.tabGroups.activeTabGroup, this._tabClock++);
    
    updateActiveTabGroup();
    this._store.add(vscode.window.tabGroups.onDidChangeTabGroups(e => {
      e.closed.forEach(item => this._tabGroupsUseInfo.delete(item));
      updateActiveTabGroup();
    }));

    this._store.add(vscode.window.tabGroups.onDidChangeTabs(e => {
      this._onDidChangeTabs.fire({
        changed: e.changed.map(t => this._asTabInfo(t)),
        closed: e.closed.map(t => this._asTabInfo(t)),
        opened: e.opened.map(t => this._asTabInfo(t))
      });
    }));
  }

  get activeTextEditor(): vscode.TextEditor | undefined {
    const candidate = vscode.window.activeTextEditor;
    if (candidate && candidate.document.uri.scheme !== 'output') {
      return candidate;
    }
    const allEditors = new ResourceMap<vscode.TextEditor>();
    vscode.window.visibleTextEditors.forEach(e => allEditors.set(e.document.uri, e));
    // Priority logic to find active editor...
  }
}
```

**Variations / call-sites:**
- `src/extension/inlineEdits/vscode-node/jointInlineCompletionProvider.ts:171` - accesses activeNotebookEditor
- `src/extension/conversation/vscode-node/userActions.ts:59-60` - accesses activeTextEditor and selection

---

#### Pattern 2: Language Intelligence Provider Registration
**Where:** `src/extension/inlineEdits/vscode-node/inlineCompletionProvider.ts:6-8, 224-239`
**What:** Extension registers inline completion providers and executes language features via vscode.languages API.

```typescript
import {
  CancellationToken, InlineCompletionContext, InlineCompletionItemProvider,
  InlineCompletionList, NotebookCell, Position, Range, TextDocument, 
  Uri, window, workspace
} from 'vscode';

export class NesInlineCompletionProvider implements InlineCompletionItemProvider {
  public async provideInlineCompletionItems(
    document: TextDocument,
    position: Position,
    context: InlineCompletionContext,
    token: CancellationToken
  ): Promise<NesCompletionList | undefined> {
    // Core provider logic that serves inline completions
    const isCompletionsEnabled = this._isCompletionsEnabled(document);
    const isInlineEditsEnabled = this._configurationService.getExperimentBasedConfig(
      ConfigKey.InlineEditsEnabled, 
      this._expService, 
      { languageId: document.languageId }
    );
    // ... completion provision logic
  }
}
```

Registration pattern found in:
- `src/extension/inlineEdits/vscode-node/inlineEditProviderFeature.ts:165` - `languages.registerInlineCompletionItemProvider('*', provider)`
- `src/extension/codeBlocks/vscode-node/provider.ts:294-297` - registers definition, type definition, implementation, and hover providers
- `src/extension/inlineChat/vscode-node/inlineChatCommands.ts:298-301` - registers code action providers

---

#### Pattern 3: Workspace & File System Operations
**Where:** `src/platform/filesystem/vscode/fileSystemServiceImpl.ts:16-53`
**What:** Extension abstracts vscode.workspace.fs for all file I/O operations (read, write, delete, rename).

```typescript
import { Uri, workspace } from 'vscode';

export class FileSystemServiceImpl implements IFileSystemService {
  stat(uri: Uri): Promise<vscode.FileStat> {
    return vscode.workspace.fs.stat(uri);
  }

  readDirectory(uri: Uri): Promise<[string, vscode.FileType][]> {
    return vscode.workspace.fs.readDirectory(uri);
  }

  createDirectory(uri: Uri): Promise<void> {
    return vscode.workspace.fs.createDirectory(uri);
  }

  readFile(uri: Uri): Promise<Uint8Array> {
    return vscode.workspace.fs.readFile(uri);
  }

  writeFile(uri: Uri, content: Uint8Array): Promise<void> {
    return vscode.workspace.fs.writeFile(uri, content);
  }

  delete(uri: Uri, options?: vscode.FileDeleteOptions): Promise<void> {
    return vscode.workspace.fs.delete(uri, options);
  }

  rename(oldURI: Uri, newURI: Uri, options?: vscode.FileRenameOptions): Promise<void> {
    return vscode.workspace.fs.rename(oldURI, newURI, options);
  }

  copy(source: Uri, destination: Uri, options?: vscode.FileCopyOptions): Promise<void> {
    return vscode.workspace.fs.copy(source, destination, options);
  }

  isWritableFileSystem(scheme: string): boolean {
    return !!vscode.workspace.fs.isWritableFileSystem(scheme);
  }

  createFileSystemWatcher(glob: string): vscode.FileSystemWatcher {
    return vscode.workspace.createFileSystemWatcher(glob);
  }
}
```

**Variations / call-sites:**
- `src/extension/chatSessions/vscode-node/copilotCLIChatSessions.ts:1342` - fs.stat on folder selection
- `src/extension/conversation/vscode-node/terminalFixGenerator.ts:147` - fs.stat for existence check
- `src/platform/search/vscode/baseSearchServiceImpl.ts:20` - findFiles2 for file pattern search

---

#### Pattern 4: Debugging Integration via Debug API
**Where:** `src/extension/onboardDebug/vscode-node/copilotDebugCommandSession.ts:15-123`
**What:** Extension manages debug sessions using vscode.debug API: starting sessions, tracking execution, handling adapter events.

```typescript
import * as vscode from 'vscode';

export const handleDebugSession = (
  launchConfigService: ILaunchConfigService,
  workspaceFolder: vscode.WorkspaceFolder | undefined,
  config: vscode.DebugConfiguration,
  handle: CopilotDebugCommandHandle,
) => {
  const trackedId = generateUuid();
  const sessions = new Set<vscode.DebugSession>();

  // Register debug adapter tracker to monitor session lifecycle
  store.add(vscode.debug.registerDebugAdapterTrackerFactory('*', {
    createDebugAdapterTracker(session) {
      if (session.configuration[TRACKED_SESSION_KEY] !== trackedId && 
          (!session.parentSession || !sessions.has(session.parentSession))) {
        return;
      }

      const isRoot = !gotRoot;
      gotRoot = true;
      sessions.add(session);

      return {
        onWillStartSession() {
          if (isRoot) {
            handle.printLabel('blue', l10n.t('Debug session starting...'));
          }
        },
        onDidSendMessage(message) {
          if (message.type === 'event' && message.event === 'output' && message.body.output) {
            handle.output(message.body.category, message.body.output);
          }
        },
        onExit(code, signal) {
          if (isRoot) ended(code ?? 0, signal);
        },
        onWillStopSession() {
          if (isRoot) ended(0);
        },
      };
    },
  }));

  // Start debugging with tracked configuration
  vscode.debug.startDebugging(workspaceFolder, { 
    ...config, 
    [TRACKED_SESSION_KEY]: trackedId 
  }).then(ok => {
    if (!ok) ended(1);
  });
};
```

**Variations / call-sites:**
- `src/extension/onboardDebug/vscode/launchConfigService.ts:94` - vscode.debug.startDebugging
- `src/extension/onboardDebug/vscode-node/copilotDebugCommandSession.ts:77` - vscode.debug.stopDebugging

---

#### Pattern 5: Terminal Integration & Shell Execution
**Where:** `src/platform/terminal/vscode/terminalServiceImpl.ts:1-175`
**What:** Extension exposes terminal creation, execution monitoring, and buffer access via vscode.window terminal API.

```typescript
import {
  Event, ExtensionTerminalOptions, Terminal, TerminalExecutedCommand, 
  TerminalOptions, TerminalShellExecutionEndEvent, TerminalShellIntegrationChangeEvent, 
  window, TerminalDataWriteEvent
} from 'vscode';

export class TerminalServiceImpl extends Disposable implements ITerminalService {
  declare readonly _serviceBrand: undefined;

  constructor(@IVSCodeExtensionContext private readonly context: IVSCodeExtensionContext) {
    super();
    this.context.environmentVariableCollection.delete('GH_TOKEN');
    for (const l of installTerminalBufferListeners()) {
      this._register(l);
    }
  }

  get terminals(): readonly Terminal[] {
    return window.terminals;
  }

  get onDidChangeTerminalShellIntegration(): Event<TerminalShellIntegrationChangeEvent> {
    return window.onDidChangeTerminalShellIntegration;
  }

  get onDidEndTerminalShellExecution(): Event<TerminalShellExecutionEndEvent> {
    return window.onDidEndTerminalShellExecution;
  }

  get onDidCloseTerminal(): Event<Terminal> {
    return window.onDidCloseTerminal;
  }

  get onDidWriteTerminalData(): Event<TerminalDataWriteEvent> {
    return window.onDidWriteTerminalData;
  }

  createTerminal(name?: string, shellPath?: string, shellArgs?: readonly string[] | string): Terminal;
  createTerminal(options: TerminalOptions): Terminal;
  createTerminal(options: ExtensionTerminalOptions): Terminal;
  createTerminal(name?: any, shellPath?: any, shellArgs?: any): Terminal {
    return window.createTerminal(name, shellPath, shellArgs);
  }

  contributePath(contributor: string, pathLocation: string, description?: string | { command: string }, prepend: boolean = false): void {
    const entry = this.pathContributions.find(c => c.contributor === contributor);
    if (entry) {
      entry.path = pathLocation;
    } else {
      this.pathContributions.push({ contributor, path: pathLocation, description, prepend });
    }
    this.updateEnvironmentPath();
  }

  private updateEnvironmentPath(): void {
    const pathVariable = 'PATH';
    this.context.environmentVariableCollection.delete(pathVariable);
    const allPaths = this.pathContributions.map(c => c.path);
    if (this.pathContributions.some(c => c.prepend)) {
      const pathVariableChange = allPaths.join(path.delimiter) + path.delimiter;
      this.context.environmentVariableCollection.prepend(pathVariable, pathVariableChange);
    } else {
      const pathVariableChange = path.delimiter + allPaths.join(path.delimiter);
      this.context.environmentVariableCollection.append(pathVariable, pathVariableChange);
    }
  }
}
```

**Variations / call-sites:**
- `src/extension/conversation/vscode-node/terminalFixGenerator.ts:90` - createQuickPick for selection
- `src/extension/conversation/vscode-node/terminalFixGenerator.ts:120` - activeTerminal?.sendText
- `src/extension/onboardDebug/vscode-node/onboardTerminalTestsContribution.ts:25-30` - registerTerminalQuickFixProvider, onDidCloseTerminal, onDidStartTerminalShellExecution

---

#### Pattern 6: Source Control (Git) Integration
**Where:** `src/platform/git/vscode/gitExtensionServiceImpl.ts:1-91`
**What:** Extension loads the VS Code Git extension API dynamically, handling extension activation and lifecycle.

```typescript
import * as vscode from 'vscode';
import { API, GitExtension } from './git';

export class GitExtensionServiceImpl implements IGitExtensionService {
  declare readonly _serviceBrand: undefined;

  private readonly _onDidChange = new vscode.EventEmitter<{ enabled: boolean }>();
  readonly onDidChange: vscode.Event<{ enabled: boolean }> = this._onDidChange.event;

  private _api: API | undefined;
  private _extensionEnablement: boolean | undefined = undefined;

  getExtensionApi(): API | undefined {
    return this._api;
  }

  private readonly _disposables: vscode.Disposable[] = [];

  constructor(@ILogService private readonly _logService: ILogService) {
    this._logService.info('[GitExtensionServiceImpl] Initializing Git extension service.');
    this._disposables.push(...this._initializeExtensionApi());
  }

  get extensionAvailable(): boolean {
    if (this._extensionEnablement === undefined) {
      return !!vscode.extensions.getExtension<GitExtension>('vscode.git');
    } else {
      return this._extensionEnablement;
    }
  }

  private _initializeExtensionApi(): vscode.Disposable[] {
    const disposables: vscode.Disposable[] = [];
    let gitExtension = vscode.extensions.getExtension<GitExtension>('vscode.git');

    const initialize = async () => {
      try {
        const extension = await gitExtension!.activate();
        this._logService.info('[GitExtensionServiceImpl] Successfully activated vscode.git extension.');
        
        const onDidChangeGitExtensionEnablement = (enabled: boolean) => {
          this._extensionEnablement = enabled;
          if (enabled) {
            this._api = extension.getAPI(1);
            this._onDidChange.fire({ enabled: true });
          } else {
            this._api = undefined;
            this._onDidChange.fire({ enabled: false });
          }
        };

        disposables.push(extension.onDidChangeEnablement(onDidChangeGitExtensionEnablement));
        onDidChangeGitExtensionEnablement(extension.enabled);
      } catch (e) {
        this._logService.error(e, '[GitExtensionServiceImpl] Failed to activate vscode.git extension.');
      }
    };

    if (gitExtension) {
      initialize();
    } else {
      const listener = vscode.extensions.onDidChange(() => {
        if (!gitExtension && vscode.extensions.getExtension<GitExtension>('vscode.git')) {
          gitExtension = vscode.extensions.getExtension<GitExtension>('vscode.git');
          initialize();
          listener.dispose();
        }
      });
    }

    return disposables;
  }
}
```

**Variations / call-sites:**
- `src/platform/git/vscode-node/gitServiceImpl.ts:214-227` - reads .git config file via workspace.fs.readFile

---

#### Pattern 7: Notebook Support & Execution Tracking
**Where:** `src/platform/notebook/vscode/notebookServiceImpl.ts:1-80`
**What:** Extension accesses notebook documents, tracks cell execution, and monitors notebook state via vscode APIs.

```typescript
import { 
  commands, DocumentSymbol, extensions, NotebookCell, Uri, window, workspace 
} from 'vscode';

export class NotebookService implements INotebookService {
  declare readonly _serviceBrand: undefined;

  private _cellExecution: Map<string, ICellExecution[]> = new Map();
  private _cellSymbols = new WeakMap<NotebookCell, DocumentSymbol[]>();
  private readonly _executionService = new NotebookExecutionServiceImpl();

  private _hasJupyterExtension() {
    return extensions.getExtension('ms-toolsai.jupyter')?.isActive;
  }

  public trackAgentUsage(): void {
    commands.executeCommand('setContext', NOTEBOOK_AGENT_USAGE_KEY, true);
  }

  async getVariables(notebook: Uri): Promise<VariablesResult[]> {
    if (!this._hasJupyterExtension()) {
      try {
        const results = await commands.executeCommand<Variable | VariablesResult>(
          'vscode.executeNotebookVariableProvider', 
          notebook
        );
        if (results && Array.isArray(results)) {
          const variableResults = results.map(this._convertResult);
          return this._filterVariables(notebook, variableResults);
        }
        return [];
      } catch (_ex) {
        this._logger.error(`Failed to get notebook variables: ${_ex}`);
      }
    }
  }
}
```

**Variations / call-sites:**
- `src/extension/inlineEdits/vscode-node/jointInlineCompletionProvider.ts:171` - vscode.window.activeNotebookEditor
- `src/extension/inlineEdits/vscode-node/inlineCompletionProvider.ts:281` - workspace.notebookDocuments access

---

#### Pattern 8: Command & Context Management
**Where:** `src/extension/notebook/vscode-node/followActions.ts:37-47`
**What:** Extension registers and manages context keys and commands for UI state synchronization.

```typescript
import * as vscode from 'vscode';

const NOTEBOOK_FOLLOW_IN_SESSION_KEY = 'github.copilot.chat.notebook.followCellExecution';

export class FollowActions {
  constructor(context: vscode.ExtensionContext) {
    this._register(vscode.commands.registerCommand(
      'github.copilot.chat.notebook.enableFollowCellExecution', 
      () => {
        this.setFollowState(true);
      }
    ));

    this._register(vscode.commands.registerCommand(
      'github.copilot.chat.notebook.disableFollowCellExecution', 
      () => {
        this.setFollowState(false);
      }
    ));
  }

  setFollowState(value: boolean): void {
    vscode.commands.executeCommand('setContext', NOTEBOOK_FOLLOW_IN_SESSION_KEY, value);
  }
}
```

**Variations / call-sites:**
- `src/extension/inlineEdits/vscode-node/jointInlineCompletionProvider.ts:222` - setContext for extensionUnification.activated
- Multiple files: Command registration for UI features across the extension

---

#### Pattern 9: Language Model Access (Advanced API)
**Where:** `src/extension/log/vscode-node/extensionStateCommand.ts:79-88`
**What:** Extension queries available language models using vscode.lm API (proposed/stable depending on version).

```typescript
const copilotModels = await vscode.lm.selectChatModels({ vendor: 'copilot' });

const copilotEmbeddings = vscode.lm.embeddingModels.filter(m => m.startsWith('copilot.'));

// From src/extension/byok/vscode-node/test/geminiNativeProvider.spec.ts:110-128
const model: vscode.LanguageModelChatInformation = {
  vendor: 'custom',
  family: 'custom-model',
  id: 'custom/model-id',
  name: 'Custom Model',
  maxInputTokens: 1000,
  // ... additional model metadata
};

const messages: vscode.LanguageModelChatMessage[] = [
  new vscode.LanguageModelChatMessage(vscode.LanguageModelChatMessageRole.User, 'hello')
];

const chatResponse = await model.sendChatRequest(
  messages,
  { requestInitiator: 'test', tools: [], toolMode: vscode.LanguageModelChatToolMode.Auto },
  cancellationToken
);
```

**Variations / call-sites:**
- `src/extension/chatSessionContext/vscode-node/chatSessionContextProvider.ts:189-192` - selectChatModels with vendor/family filters
- `src/extension/tools/common/test/toolService.spec.ts:30-132` - LanguageModelToolInformation registration

---

## Summary

The Copilot extension demonstrates clean architectural separation:

1. **Public API Only**: All 695K LOC of extension code depends exclusively on the `vscode` module, never importing internal `vs/` modules
2. **Provider Registration**: Inline completions, code actions, debugger trackers, hover providers all registered via vscode.languages/window/debug APIs
3. **File System Abstraction**: All workspace I/O goes through vscode.workspace.fs (extensible, cross-platform)
4. **Event-Driven Architecture**: Uses vscode event emitters for tabs, terminals, documents, source control state changes
5. **Extension Composition**: Dynamically loads Git extension via vscode.extensions API, handles activation lifecycle
6. **Terminal & Shell Integration**: Uses vscode.window terminal API with shell execution tracking
7. **Notebook Support**: Accesses notebook documents and cell execution via vscode APIs
8. **Language Models**: Leverages vscode.lm for chat models, embeddings, and tool definitions

This clean separation means porting core VS Code to Tauri/Rust would require implementing an equivalent surface-level API that extensions expect—a bridging layer rather than re-architecting the extension substrate itself.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
