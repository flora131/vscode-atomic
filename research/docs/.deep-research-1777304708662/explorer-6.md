# Partition 6 of 79 — Findings

## Scope
`src/vscode-dts/` (168 files, 33,096 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Partition 6: VS Code Extension API Surface (`src/vscode-dts/`)

## Overview
This partition contains the complete public extension API definitions for VS Code. For a Tauri/Rust port, these files define the stable contract that must be preserved—extensions depend entirely on this API surface. The main file is `vscode.d.ts` (21,233 LOC), which defines 15 core namespaces covering editing, language intelligence, debugging, source control, terminal, navigation, and other IDE functionalities.

The 167 proposed API files represent experimental features not yet committed to stable API.

---

## Implementation

### Core Extension API Definition
- `src/vscode-dts/vscode.d.ts` — Primary public API surface with 15 main namespaces: `window`, `workspace`, `languages`, `notebooks`, `scm`, `debug`, `commands`, `tasks`, `extensions`, `authentication`, `chat`, `lm`, `env`, `l10n`, `tests` (21,233 LOC)

### Proposed/Experimental Features (Grouped by Porting Priority)

#### Editor Core & Text Manipulation (Critical)
- `src/vscode-dts/vscode.proposed.editorInsets.d.ts` — Inline editor rendering
- `src/vscode-dts/vscode.proposed.inlineCompletionsAdditions.d.ts` — IntelliSense completions API extensions (290 LOC)
- `src/vscode-dts/vscode.proposed.textSearchProvider.d.ts` — Text search in files (281 LOC)
- `src/vscode-dts/vscode.proposed.textSearchProvider2.d.ts` — Enhanced text search (295 LOC)
- `src/vscode-dts/vscode.proposed.textSearchComplete2.d.ts` — Search completion events

#### Terminal/Shell Integration (Critical)
- `src/vscode-dts/vscode.proposed.terminalCompletionProvider.d.ts` — Terminal command completion (286 LOC)
- `src/vscode-dts/vscode.proposed.terminalExecuteCommandEvent.d.ts` — Terminal command execution tracking
- `src/vscode-dts/vscode.proposed.terminalQuickFixProvider.d.ts` — Terminal error quick-fixes
- `src/vscode-dts/vscode.proposed.terminalShellEnv.d.ts` — Shell environment control
- `src/vscode-dts/vscode.proposed.terminalDataWriteEvent.d.ts` — Terminal data stream events
- `src/vscode-dts/vscode.proposed.terminalDimensions.d.ts` — Terminal sizing
- `src/vscode-dts/vscode.proposed.terminalSelection.d.ts` — Terminal text selection
- `src/vscode-dts/vscode.proposed.terminalTitle.d.ts` — Terminal window title control

#### Debugging (Critical)
- `src/vscode-dts/vscode.proposed.debugVisualization.d.ts` — Debug visualizer extensions (170 LOC)

#### Source Control/Version Control (Critical)
- `src/vscode-dts/vscode.proposed.scmHistoryProvider.d.ts` — Git history API (95 LOC)
- `src/vscode-dts/vscode.proposed.scmArtifactProvider.d.ts` — SCM artifact tracking
- `src/vscode-dts/vscode.proposed.scmMultiDiffEditor.d.ts` — Multi-file diff editing
- `src/vscode-dts/vscode.proposed.scmActionButton.d.ts` — Custom SCM actions
- `src/vscode-dts/vscode.proposed.scmProviderOptions.d.ts` — SCM provider configuration
- `src/vscode-dts/vscode.proposed.scmSelectedProvider.d.ts` — SCM provider selection
- `src/vscode-dts/vscode.proposed.scmTextDocument.d.ts` — SCM document interaction
- `src/vscode-dts/vscode.proposed.scmValidation.d.ts` — Input validation for SCM

#### Language Intelligence & AI (Critical)
- `src/vscode-dts/vscode.proposed.languageModelCapabilities.d.ts` — LM capability discovery
- `src/vscode-dts/vscode.proposed.languageModelSystem.d.ts` — System language model access
- `src/vscode-dts/vscode.proposed.languageModelProxy.d.ts` — Language model proxying
- `src/vscode-dts/vscode.proposed.languageModelThinkingPart.d.ts` — Chain-of-thought reasoning (111 LOC)
- `src/vscode-dts/vscode.proposed.languageModelToolSupportsModel.d.ts` — Tool-model compatibility
- `src/vscode-dts/vscode.proposed.languageModelToolResultAudience.d.ts` — Tool visibility control

#### Chat & Copilot Integration (High Priority)
- `src/vscode-dts/vscode.proposed.chatParticipantAdditions.d.ts` — Chat participant extensions (1,089 LOC)
- `src/vscode-dts/vscode.proposed.chatSessionsProvider.d.ts` — Chat session management (781 LOC)
- `src/vscode-dts/vscode.proposed.chatDebug.d.ts` — Debug chat integration (802 LOC)
- `src/vscode-dts/vscode.proposed.chatContextProvider.d.ts` — Chat context providers (221 LOC)
- `src/vscode-dts/vscode.proposed.chatProvider.d.ts` — Custom chat providers (169 LOC)
- `src/vscode-dts/vscode.proposed.chatSessionCustomizationProvider.d.ts` — Session customization (170 LOC)
- `src/vscode-dts/vscode.proposed.chatPromptFiles.d.ts` — Prompt file integration (477 LOC)
- `src/vscode-dts/vscode.proposed.chatHooks.d.ts` — Chat lifecycle hooks (126 LOC)
- `src/vscode-dts/vscode.proposed.chatOutputRenderer.d.ts` — Custom output rendering (101 LOC)
- `src/vscode-dts/vscode.proposed.chatStatusItem.d.ts` — Chat status indicators
- `src/vscode-dts/vscode.proposed.chatReferenceBinaryData.d.ts` — Binary reference handling
- `src/vscode-dts/vscode.proposed.chatReferenceDiagnostic.d.ts` — Diagnostic references
- `src/vscode-dts/vscode.proposed.chatTab.d.ts` — Chat tab UI
- `src/vscode-dts/vscode.proposed.defaultChatParticipant.d.ts` — Default participant selection

#### Notebook Support (High Priority)
- `src/vscode-dts/vscode.proposed.notebookExecution.d.ts` — Notebook cell execution (10 files total)
- `src/vscode-dts/vscode.proposed.notebookCellExecution.d.ts` — Cell-level execution control
- `src/vscode-dts/vscode.proposed.notebookKernelSource.d.ts` — Kernel source resolution
- `src/vscode-dts/vscode.proposed.notebookMessaging.d.ts` — Notebook-extension communication
- `src/vscode-dts/vscode.proposed.notebookVariableProvider.d.ts` — Variable inspection in notebooks
- `src/vscode-dts/vscode.proposed.notebookMime.d.ts` — MIME type rendering
- `src/vscode-dts/vscode.proposed.notebookReplDocument.d.ts` — REPL document support
- `src/vscode-dts/vscode.proposed.notebookControllerAffinityHidden.d.ts` — Kernel affinity control
- `src/vscode-dts/vscode.proposed.notebookDeprecated.d.ts` — Deprecated notebook APIs
- `src/vscode-dts/vscode.proposed.notebookLiveShare.d.ts` — Live Share integration

#### File Search & Navigation (High Priority)
- `src/vscode-dts/vscode.proposed.fileSearchProvider.d.ts` — File search providers
- `src/vscode-dts/vscode.proposed.fileSearchProvider2.d.ts` — Enhanced file search (104 LOC)
- `src/vscode-dts/vscode.proposed.findTextInFiles.d.ts` — Find-in-files API (104 LOC)
- `src/vscode-dts/vscode.proposed.findTextInFiles2.d.ts` — Enhanced find (150 LOC)
- `src/vscode-dts/vscode.proposed.findFiles2.d.ts` — Find files API (129 LOC)

#### Authentication & Security (High Priority)
- `src/vscode-dts/vscode.proposed.authenticationChallenges.d.ts` — Auth challenge handling
- `src/vscode-dts/vscode.proposed.authProviderSpecific.d.ts` — Provider-specific auth
- `src/vscode-dts/vscode.proposed.authIssuers.d.ts` — Auth issuer configuration
- `src/vscode-dts/vscode.proposed.authSession.d.ts` — Session management
- `src/vscode-dts/vscode.proposed.authLearnMore.d.ts` — Auth help/learning

#### Comments & Code Review (Medium Priority)
- `src/vscode-dts/vscode.proposed.commentThreadApplicability.d.ts` — Comment scope control
- `src/vscode-dts/vscode.proposed.commentReveal.d.ts` — Comment navigation
- `src/vscode-dts/vscode.proposed.commentsDraftState.d.ts` — Draft comment state
- `src/vscode-dts/vscode.proposed.commentingRangeHint.d.ts` — Comment range hints
- `src/vscode-dts/vscode.proposed.commentReactor.d.ts` — Comment reactions

#### UI Contributions & Menus (Medium Priority - 28 files)
- `src/vscode-dts/vscode.proposed.contribDebugCreateConfiguration.d.ts` — Debug config UI
- `src/vscode-dts/vscode.proposed.contribSourceControlTitleMenu.d.ts` — SCM menu items
- `src/vscode-dts/vscode.proposed.contribSourceControlInputBoxMenu.d.ts` — SCM input menus
- `src/vscode-dts/vscode.proposed.contribCommentThreadAdditionalMenu.d.ts` — Comment menus
- `src/vscode-dts/vscode.proposed.contribEditSessions.d.ts` — Edit session menus
- `src/vscode-dts/vscode.proposed.contribMergeEditorMenus.d.ts` — Merge editor menus
- `src/vscode-dts/vscode.proposed.contribStatusBarItems.d.ts` — Status bar contributions
- `src/vscode-dts/vscode.proposed.contribAccessibilityHelpContent.d.ts` — Accessibility help
- `src/vscode-dts/vscode.proposed.contribViewsRemote.d.ts` — Remote view contributions
- `src/vscode-dts/vscode.proposed.contribViewsWelcome.d.ts` — Welcome tab views
- `src/vscode-dts/vscode.proposed.contribNotebookStaticPreloads.d.ts` — Notebook preloads
- `src/vscode-dts/vscode.proposed.contribLanguageModelToolSets.d.ts` — Tool set UI
- `src/vscode-dts/vscode.proposed.contribShareMenu.d.ts` — Share menu items
- Plus 14 additional menu contribution files

#### Tool Integration & MCP (Medium Priority)
- `src/vscode-dts/vscode.proposed.resolvers.d.ts` — Task/debug resolver providers (475 LOC)
- `src/vscode-dts/vscode.proposed.mcpToolDefinitions.d.ts` — MCP tool definitions (98 LOC)
- `src/vscode-dts/vscode.proposed.mcpServerDefinitions.d.ts` — MCP server definitions

#### Testing Framework (Medium Priority)
- `src/vscode-dts/vscode.proposed.testObserver.d.ts` — Test state observation (199 LOC)
- `src/vscode-dts/vscode.proposed.testRelatedCode.d.ts` — Test-code relationships

#### Other Experimental Features (Lower Priority - 73+ files)
- Audio/Speech: `vscode.proposed.speech.d.ts` — Voice input/output
- Environment: `vscode.proposed.environmentPower.d.ts` — Power management (166 LOC)
- Diff/Merge: `vscode.proposed.diffCommand.d.ts`, `vscode.proposed.diffContentOptions.d.ts`
- Timeline: `vscode.proposed.timeline.d.ts` — Source control history timeline (163 LOC)
- Ports: `vscode.proposed.portsAttributes.d.ts` — Forwarded port management (100 LOC)
- Inline operations: `vscode.proposed.editorHoverVerbosityLevel.d.ts`, `vscode.proposed.editorInsets.d.ts`
- Mappings: `vscode.proposed.mappedEditsProvider.d.ts` — Auto-fix mapping (110 LOC)
- Tree views: `vscode.proposed.treeItemMarkdownLabel.d.ts`, `vscode.proposed.treeViewActiveItem.d.ts`, etc.
- URI operations: `vscode.proposed.externalUriOpener.d.ts` — External URI handling (163 LOC)
- Tunnels: `vscode.proposed.tunnels.d.ts`, `vscode.proposed.tunnelFactory.d.ts`
- Workspace/Settings: `vscode.proposed.workspaceTrust.d.ts`, various contrib files
- Data channels: `vscode.proposed.dataChannels.d.ts`
- Embeddings: `vscode.proposed.embeddings.d.ts`
- Edit sessions: `vscode.proposed.editSessionIdentityProvider.d.ts`
- File chunks: `vscode.proposed.fsChunks.d.ts`

---

## Types / Interfaces

**Core data structures defined in `vscode.d.ts` (stable API):**
- Editor abstractions: `TextDocument`, `TextLine`, `TextEditor`, `TextEditorEdit`
- Position/Range: `Position`, `Range`, `Selection`
- Symbols: `SymbolInformation`, `DocumentSymbol`, `WorkspaceSymbol`
- Diagnostics: `Diagnostic`, `DiagnosticRelatedInformation`, `DiagnosticCollection`
- Completions: `CompletionItem`, `CompletionItemProvider`, `CompletionList`
- Hover/InlineValue: `Hover`, `InlineValue`, `InlineValueProvider`
- Signatures: `SignatureHelp`, `SignatureInformation`, `ParameterInformation`
- References: `Location`, `ReferenceContext`, `ReferenceProvider`
- Formatting: `FormattingOptions`, `DocumentFormattingEditProvider`, `RangeFormattingEditProvider`
- Refactoring: `CodeAction`, `CodeActionProvider`, `CodeActionContext`
- Tasks: `Task`, `TaskProvider`, `TaskExecution`, `TaskDefinition`
- Debug: `DebugSession`, `DebugConsole`, `DebugAdapterDescriptorFactory`
- SCM: `SourceControl`, `SourceControlResourceGroup`, `SourceControlResourceState`
- Workspace: `WorkspaceFolder`, `RelativePattern`, `FileSystemProvider`
- Terminal: `Terminal`, `TerminalOptions`, `Pseudoterminal`
- Window: `Window`, `StatusBarItem`, `QuickPickItem`, `InputBoxOptions`
- Chat: `ChatParticipant`, `ChatMessage`, `ChatRequest`, `ChatResponse`
- LM: `LanguageModel`, `LanguageModelCreateOptions`

**Key interface stability concerns for porting:**
- All interfaces in `vscode.d.ts` are stable and must be preserved identically
- Proposed APIs have stability tags but experimental changes are expected
- Breaking changes to stable interfaces break all extensions using that API

---

## Configuration

No configuration files in this partition (they're in other partitions). This directory is purely API definitions.

---

## Documentation

**API documentation is embedded as JSDoc comments** within each `.d.ts` file:
- Each namespace, interface, function, and property has `/**` documentation blocks
- Documentation includes usage examples, parameter descriptions, and deprecation notices
- Links to related APIs using `@see`, `@link`, `{@link}`
- Platform/environment notes (e.g., "Only available on macOS")

Key documentation patterns:
- `vscode.d.ts` includes comprehensive overview docs for each namespace at lines 9347+
- Proposed files include stability/proposal stage documentation
- `@deprecated` tags mark APIs nearing removal
- `@since` tags indicate when APIs were introduced

---

## Notable Clusters

### Core Namespace Definitions (vscode.d.ts: 1 file, 21,233 LOC)
- **Location:** `src/vscode-dts/vscode.d.ts`
- **Content:** 15 primary namespaces covering editor, workspace, languages, notebooks, debugging, source control, terminal, tasks, chat, language models, extensions, authentication, tests, localization, environment
- **Lines by namespace:** window (2,728), workspace (925), languages (1,628), debug (175), chat (621), lm (542), tasks (1,392), notebook (257), scm (631), commands (96), extensions (633), authentication (101), tests (1,840), env (234), l10n (79)
- **Porting complexity:** HIGHEST - This is the contract all extensions depend on

### Chat & Copilot APIs (14 files, ~6,000 LOC)
- **Location:** `src/vscode-dts/vscode.proposed.chat*.d.ts`
- **Largest files:** chatParticipantAdditions (1,089 LOC), chatSessionsProvider (781 LOC), chatDebug (802 LOC)
- **Porting relevance:** Essential for AI/LM integration, chat participant ecosystem
- **Key interfaces:** ChatParticipant, ChatMessage, ChatRequest, ChatResponse, ChatContext, ChatVariable

### Terminal/Shell Integration (8 files, ~850 LOC)
- **Location:** `src/vscode-dts/vscode.proposed.terminal*.d.ts`
- **Largest file:** terminalCompletionProvider (286 LOC)
- **Porting relevance:** Critical for integrated terminal functionality
- **Key interfaces:** Pseudoterminal, TerminalOptions, TerminalExitStatus, completion/quick-fix providers

### Notebook Support (10 files, ~1,200 LOC)
- **Location:** `src/vscode-dts/vscode.proposed.notebook*.d.ts`
- **Porting relevance:** Essential for Jupyter/interactive notebook support
- **Key interfaces:** NotebookDocument, NotebookCell, NotebookKernel, NotebookController, NotebookController

### Source Control (8 files, ~800 LOC)
- **Location:** `src/vscode-dts/vscode.proposed.scm*.d.ts` + main `vscode.d.ts` SCM namespace
- **Largest file:** scmHistoryProvider (95 LOC) + core SCM namespace (631 LOC)
- **Porting relevance:** Critical for Git/VCS integration
- **Key interfaces:** SourceControl, SourceControlResourceGroup, SourceControlResourceState, SCMProvider

### Language Intelligence APIs (6 files, ~500 LOC)
- **Location:** `src/vscode-dts/vscode.proposed.languageModel*.d.ts` + main `lm` namespace (542 LOC)
- **Porting relevance:** Critical for language features, AI model integration
- **Key interfaces:** LanguageModel, LanguageModelRequest, LanguageModelMessageRole, LanguageModelChatMessage

### Search & Navigation (8 files, ~1,000 LOC)
- **Location:** `src/vscode-dts/vscode.proposed.find*.d.ts`, `vscode.proposed.textSearch*.d.ts`, `vscode.proposed.fileSearch*.d.ts`
- **Largest files:** textSearchProvider2 (295 LOC), textSearchProvider (281 LOC)
- **Porting relevance:** High - for global find/replace, file search, navigation
- **Key interfaces:** FileSearchProvider, TextSearchProvider, SearchOptions, SearchResultMetadataProvider

### UI Contributions & Menus (28 files, ~400 LOC)
- **Location:** `src/vscode-dts/vscode.proposed.contrib*.d.ts`
- **Porting relevance:** Medium - extends menu/UI contribution system, not critical for core IDE
- **Pattern:** Each file extends a specific menu context with new contribution point

### Testing Framework (2 files, ~300 LOC)
- **Location:** `src/vscode-dts/vscode.proposed.test*.d.ts` + main `tests` namespace (1,840 LOC)
- **Porting relevance:** Medium - for test explorer and test provider integration

---

## Summary

Partition 6 defines VS Code's entire public extension API surface across 168 TypeScript definition files. The **critical porting requirement is vscode.d.ts (21,233 LOC)**, which contains 15 namespaces representing core IDE functionality: editing, language intelligence, debugging, source control, terminal, navigation, tasks, notebooks, chat/AI, authentication, testing, and localization. 

For a Tauri/Rust port, the stable API surface in vscode.d.ts must be preserved to maintain extension compatibility. The 167 proposed files represent experimental features organized into clusters: Chat/Copilot (14 files), UI Contributions (28 files), Notebooks (10 files), Terminal (8 files), Source Control (8 files), Search/Navigation (8 files), and 85+ other extension/feature APIs. The biggest porting challenges are: (1) preserving the exact interface contracts, (2) implementing async/event-driven patterns across all namespaces, (3) supporting the extension marketplace ecosystem, and (4) maintaining language server protocol bridges for language intelligence features.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.d.ts` (21,233 LOC) — stable public API
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.terminalCompletionProvider.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.debugVisualization.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.scmHistoryProvider.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.languageModelCapabilities.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.languageModelSystem.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.languageModelThinkingPart.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.languageModelProxy.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.notebookExecution.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.fileSearchProvider2.d.ts`

Total proposed API files in partition: 167 (169 total files - 2 non-`.d.ts`)

---

### Per-File Notes

#### `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.d.ts`

- **Role:** Single `declare module 'vscode'` block (~21 K lines) that is the entire stable public contract between the VS Code host and all extensions. Every namespace, class, interface, enum, and type alias used by extensions is declared here. It is consumed at extension compile time and injected by the host at runtime via the extension host IPC bridge.

- **Key symbols:**
  - `Uri` class (`vscode.d.ts:1439`) — five-component (scheme, authority, path, query, fragment) immutable value that is the universal identifier crossing every host/extension call. Factory methods `Uri.parse`, `Uri.file`, `Uri.joinPath`, `Uri.from` (`vscode.d.ts:1454–1531`).
  - `Position` class (`vscode.d.ts:269`) — zero-based `{line, character}` pair; character offsets in UTF-16 code units.
  - `Range` class (`vscode.d.ts:408`) — ordered `{start: Position, end: Position}` pair, immutable.
  - `Disposable` class (`vscode.d.ts:1712`) — RAII handle; `dispose()` unregisters a provider or listener.
  - `Event<T>` interface (`vscode.d.ts:1755`) — callable `(listener, thisArgs?, disposables?) => Disposable`; all host-to-extension notifications are modeled as typed events.
  - `CancellationToken` interface (`vscode.d.ts:1659`) — `isCancellationRequested: boolean` + `onCancellationRequested: Event<any>`; every async provider call receives one.
  - `ProviderResult<T>` type alias (`vscode.d.ts:2450`) — `T | undefined | null | Thenable<T | undefined | null>`; the universal return type for every extension-side provider method.
  - `TextDocument` interface (`vscode.d.ts:88`) — read-only model of an open document; fields include `uri: Uri`, `languageId: string`, `version: number`, `isDirty: boolean`, `encoding: string` (IANA charset string list at lines 130–133).
  - `FileSystem` interface (`vscode.d.ts:9774`) — `workspace.fs`; exposes `stat`, `readDirectory`, `createDirectory`, `readFile`, `writeFile`, `delete`, `rename`, `copy` (`vscode.d.ts:9782–9861`), all returning `Thenable<T>`.
  - `Extension<T>` interface (`vscode.d.ts:8333`) — reflection on an installed extension: `id`, `extensionUri: Uri`, `isActive`, `packageJSON: any`, `extensionKind: ExtensionKind`, `exports: T`, `activate(): Thenable<T>`.
  - `ExtensionContext` interface (`vscode.d.ts:8415`) — per-extension container of `subscriptions`, storage memento, secrets, extension URI, log output channel, etc.

- **Namespaces and their line numbers:**
  - `tasks` (`vscode.d.ts:9347`) — `registerTaskProvider`, `fetchTasks`, `executeTask`, `taskExecutions`, `onDidStartTask`, `onDidEndTask`, `onDidStartTaskProcess`, `onDidEndTaskProcess`.
  - `env` (`vscode.d.ts:10739`) — `appName`, `appRoot`, `appHost`, `uriScheme`, `language`, `clipboard`, `machineId`, `sessionId`, `isNewAppInstall`, `isAppPortable`, `isTelemetryEnabled`, `onDidChangeTelemetryEnabled`, `onDidChangeShell`, `createTelemetryLogger`.
  - `commands` (`vscode.d.ts:10973`) — `registerCommand`, `registerTextEditorCommand`, `executeCommand<T>`, `getCommands`.
  - `window` (`vscode.d.ts:11069`) — `tabGroups`, `activeTextEditor`, `visibleTextEditors`, `activeNotebookEditor`, `terminals`, `activeTerminal`, `state: WindowState`, `onDidChangeWindowState`, `showTextDocument`, `showNotebookDocument`, `createTextEditorDecorationType`, `showInformationMessage`, `showWarningMessage`, `showErrorMessage`, `showInputBox`, `showQuickPick`, `showOpenDialog`, `showSaveDialog`, `createStatusBarItem`, `createOutputChannel`, `createTerminal`, `registerTreeDataProvider`, `createTreeView`, `registerWebviewPanelSerializer`, `createWebviewPanel`, `withProgress`.
  - `workspace` (`vscode.d.ts:13797`) — `fs: FileSystem`, `workspaceFolders`, `workspaceFile`, `onDidChangeWorkspaceFolders`, `updateWorkspaceFolders`, `createFileSystemWatcher`, `findFiles`, `openTextDocument`, `registerTextDocumentContentProvider`, `applyEdit`, `saveAll`, `getConfiguration`, `registerFileSystemProvider`, `onDidOpenTextDocument`, `onDidCloseTextDocument`, `onDidChangeTextDocument`, `onDidSaveTextDocument`, `onDidCreateFiles`, `onDidDeleteFiles`, `onDidRenameFiles`.
  - `languages` (`vscode.d.ts:14722`) — `getLanguages`, `setTextDocumentLanguage`, `match`, `onDidChangeDiagnostics`, `getDiagnostics`, `createDiagnosticCollection`, `createLanguageStatusItem`, `registerCompletionItemProvider`, `registerInlineCompletionItemProvider`, `registerCodeActionsProvider`, `registerCodeLensProvider`, `registerDefinitionProvider`, `registerImplementationProvider`, `registerTypeDefinitionProvider`, `registerDeclarationProvider`, `registerHoverProvider`, `registerDocumentHighlightProvider`, `registerDocumentSymbolProvider`, `registerWorkspaceSymbolProvider`, `registerReferenceProvider`, `registerRenameProvider`, `registerDocumentFormattingEditProvider`, `registerDocumentRangeFormattingEditProvider`, `registerOnTypeFormattingEditProvider`, `registerSignatureHelpProvider`, `registerDocumentLinkProvider`, `registerColorProvider`, `registerFoldingRangeProvider`, `registerSelectionRangeProvider`, `registerCallHierarchyProvider`, `registerTypeHierarchyProvider`, `registerLinkedEditingRangeProvider`, `registerInlayHintsProvider`, `registerEvaluatableExpressionProvider`, `registerInlineValuesProvider`.
  - `notebooks` (`vscode.d.ts:16350`) — `createNotebookController`, `registerNotebookCellStatusBarItemProvider`, `createRendererMessaging`.
  - `scm` (`vscode.d.ts:16652`) — `inputBox` (deprecated), `createSourceControl`.
  - `debug` (`vscode.d.ts:17283`) — `activeDebugSession`, `activeDebugConsole`, `breakpoints`, `activeStackItem`, `onDidChangeActiveDebugSession`, `onDidStartDebugSession`, `onDidReceiveDebugSessionCustomEvent`, `onDidTerminateDebugSession`, `onDidChangeBreakpoints`, `onDidChangeActiveStackItem`, `registerDebugConfigurationProvider`, `registerDebugAdapterDescriptorFactory`, `registerDebugAdapterTrackerFactory`, `startDebugging`, `stopDebugging`, `addBreakpoints`, `removeBreakpoints`, `asDebugSourceUri`.
  - `extensions` (`vscode.d.ts:17458`) — `getExtension<T>`, `all: readonly Extension<any>[]`, `onDidChange`.
  - `authentication` (`vscode.d.ts:18091`) — `getSession`, `getAccounts`, `onDidChangeSessions`, `registerAuthenticationProvider`.
  - `l10n` (`vscode.d.ts:18192`) — `t(message, ...args)` localization function.
  - `tests` (`vscode.d.ts:18271`) — `createTestController`.
  - `chat` (`vscode.d.ts:20111`) — `createChatParticipant(id, handler): ChatParticipant`.
  - `lm` (`vscode.d.ts:20732`) — `onDidChangeChatModels`, `selectChatModels`, `registerTool`, `tools`, `invokeTool`, `registerMcpServerDefinitionProvider`, `registerLanguageModelChatProvider`.

- **Control flow:** Extensions call `activate(context: ExtensionContext)` at host initialization. All registrations return `Disposable` objects; the host disposes them when the extension deactivates. Events fire from host to extension over the IPC bridge. Provider calls flow host → extension as `Thenable`-returning callbacks, with results flowing back. `CancellationToken` travels with every provider invocation so the host can abort in-flight calls.

- **Data flow:**
  - Host serializes structured values (Uri, Position, Range, TextDocument metadata, etc.) across the extension host IPC boundary; extensions receive deserialized TypeScript class instances.
  - `Thenable<T>` return values from providers are awaited by the host; the extension host (Node.js process in Electron, or web worker in browser) resolves them and posts results back.
  - Events are fire-and-forget from the host; the `Event<T>` listener receives a single typed payload `T`.
  - `workspace.fs` methods pass `Uri` to the host which dispatches to the appropriate `FileSystemProvider` (registered by a different extension or the built-in FS), then returns `Uint8Array` / `FileStat` / `[string, FileType][]` back over IPC.

- **Dependencies:** Declares no external imports; this file is a pure ambient declaration. At runtime it depends on the VS Code host's extension host IPC protocol (JSON-RPC or binary protocol in `src/vs/workbench/services/extensions/`), and on the Node.js runtime provided by the Electron renderer/Node extension host.

---

#### `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.terminalCompletionProvider.d.ts`

- **Role:** Proposed API augmenting `window` namespace with `registerTerminalCompletionProvider` to allow extensions to supply shell-completion suggestions for terminal command lines. Extends the stable `vscode.d.ts` via a second `declare module 'vscode'` block.

- **Key symbols:**
  - `TerminalCompletionProvider<T>` interface (`terminalCompletionProvider.d.ts:23`) — single method `provideTerminalCompletions(terminal: Terminal, context: TerminalCompletionContext, token: CancellationToken): ProviderResult<T[] | TerminalCompletionList<T>>`.
  - `TerminalCompletionItem` class (`terminalCompletionProvider.d.ts:48`) — fields: `label: string | CompletionItemLabel`, `replacementRange: readonly [number, number]`, `detail?: string`, `documentation?: string | MarkdownString`, `kind?: TerminalCompletionItemKind`.
  - `TerminalCompletionItemKind` enum (`terminalCompletionProvider.d.ts:96`) — 17 values: `File=0`, `Folder=1`, `Method=2`, `Alias=3`, `Argument=4`, `Option=5`, `OptionValue=6`, `Flag=7`, `SymbolicLinkFile=8`, `SymbolicLinkFolder=9`, `ScmCommit=10`, `ScmBranch=11`, `ScmTag=12`, `ScmStash=13`, `ScmRemote=14`, `PullRequest=15`, `PullRequestDone=16`.
  - `TerminalCompletionContext` interface (`terminalCompletionProvider.d.ts:189`) — `commandLine: string`, `cursorIndex: number`.
  - `TerminalCompletionList<T>` class (`terminalCompletionProvider.d.ts:210`) — `items: T[]`, `resourceOptions?: TerminalCompletionResourceOptions`.
  - `TerminalCompletionResourceOptions` interface (`terminalCompletionProvider.d.ts:237`) — `showFiles: boolean`, `showDirectories: boolean`, `globPattern?: string`, `cwd: Uri`.
  - Registration: `window.registerTerminalCompletionProvider<T>(provider, ...triggerCharacters: string[]): Disposable` (`terminalCompletionProvider.d.ts:283`).

- **Control flow:** Host invokes `provideTerminalCompletions` on every keystroke in a terminal that matches the registered trigger characters. Returns either a flat array or a `TerminalCompletionList`; if `resourceOptions` is set, the host also injects file/folder entries from the specified `cwd` glob before showing the list.

- **Data flow:** Host → extension: `Terminal` object reference + `TerminalCompletionContext {commandLine, cursorIndex}` + `CancellationToken`. Extension → host: array of `TerminalCompletionItem` (label, `replacementRange [start, end]`, kind) or `TerminalCompletionList` carrying both items and `TerminalCompletionResourceOptions`. Host uses `replacementRange` as indices into `commandLine` to splice the accepted completion into the terminal input buffer.

- **Dependencies:** `Terminal` (from `window` namespace stable API), `CompletionItemLabel` (from `languages` namespace), `MarkdownString`, `Uri`, `CancellationToken`, `Disposable`, `ProviderResult` — all from stable `vscode.d.ts`.

---

#### `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.debugVisualization.d.ts`

- **Role:** Proposed API augmenting the `debug` namespace to allow extensions to register custom variable visualizers shown in the Debug Console / hover panels when a debug session is paused. Wires DAP (Debug Adapter Protocol) variable metadata to extension-provided tree views or commands.

- **Key symbols:**
  - `debug.registerDebugVisualizationProvider<T>(id: string, provider: DebugVisualizationProvider<T>): Disposable` (`debugVisualization.d.ts:15`).
  - `debug.registerDebugVisualizationTreeProvider<T>(id: string, provider: DebugVisualizationTree<T>): Disposable` (`debugVisualization.d.ts:25`).
  - `DebugVisualizationProvider<T>` interface (`debugVisualization.d.ts:119`) — `provideDebugVisualization(context, token): ProviderResult<T[]>`, optional `resolveDebugVisualization(visualization, token): ProviderResult<T>`.
  - `DebugVisualization` class (`debugVisualization.d.ts:94`) — `name: string`, `iconPath?`, `visualization?: Command | { treeId: string }`.
  - `DebugVisualizationContext` interface (`debugVisualization.d.ts:141`) — raw DAP `variable: any`, `containerId?: number`, `frameId?: number`, `threadId: number`, `session: DebugSession`.
  - `DebugTreeItem` interface (`debugVisualization.d.ts:34`) — `label`, `description?`, `collapsibleState?`, `contextValue?`, `canEdit?`.
  - `DebugVisualizationTree<T>` interface (`debugVisualization.d.ts:79`) — `getTreeItem(context): ProviderResult<T>`, `getChildren(element): ProviderResult<T[]>`, optional `editItem(item, value): ProviderResult<T>`.

- **Control flow:** Host evaluates the `when` clause from the extension's `package.json` `debugVisualizers` contribution for each variable when a DAP session stops. If satisfied, it calls `provideDebugVisualization(context, token)`. The returned `DebugVisualization` array enumerates available visualizers; when the user selects one, `resolveDebugVisualization` is called. If the visualization has `{ treeId }`, the host drives the registered `DebugVisualizationTree` for children and edits.

- **Data flow:** Host → extension: raw DAP `variable` payload (any JSON), DAP IDs (`containerId`, `frameId`, `threadId`), and the live `DebugSession` proxy. Extension → host: `DebugVisualization[]` objects containing either a `Command` (dispatched back to the host command registry) or a `treeId` reference to a registered tree provider. Tree item traversal (`getTreeItem`, `getChildren`) flows bidirectionally over IPC.

- **Dependencies:** `DebugSession`, `Command`, `TreeItemCollapsibleState`, `ThemeIcon`, `Uri`, `CancellationToken`, `Disposable`, `ProviderResult` — all from stable `vscode.d.ts`.

---

#### `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.scmHistoryProvider.d.ts`

- **Role:** Proposed API augmenting `SourceControl` (from stable `scm.createSourceControl`) with a `historyProvider` hook. Allows SCM extensions (e.g., Git) to expose commit history, branch/tag refs, file-level diff data, and AI chat context for individual history items.

- **Key symbols:**
  - `SourceControl.historyProvider?: SourceControlHistoryProvider` (`scmHistoryProvider.d.ts:10`).
  - `SourceControlHistoryProvider` interface (`scmHistoryProvider.d.ts:13`) — properties: `currentHistoryItemRef`, `currentHistoryItemRemoteRef`, `currentHistoryItemBaseRef` (all `SourceControlHistoryItemRef | undefined`), `onDidChangeCurrentHistoryItemRefs: Event<void>`, `onDidChangeHistoryItemRefs: Event<SourceControlHistoryItemRefsChangeEvent>`. Methods: `provideHistoryItemRefs(historyItemRefs: string[] | undefined, token): ProviderResult<SourceControlHistoryItemRef[]>`, `provideHistoryItems(options: SourceControlHistoryOptions, token): ProviderResult<SourceControlHistoryItem[]>`, `provideHistoryItemChanges(historyItemId, historyItemParentId | undefined, token): ProviderResult<SourceControlHistoryItemChange[]>`, `resolveHistoryItem(id, token): ProviderResult<SourceControlHistoryItem>`, `resolveHistoryItemChatContext(id, token): ProviderResult<string>`, `resolveHistoryItemChangeRangeChatContext(historyItemId, historyItemParentId, path, token): ProviderResult<string>`, `resolveHistoryItemRefsCommonAncestor(refs: string[], token): ProviderResult<string>`.
  - `SourceControlHistoryOptions` interface (`scmHistoryProvider.d.ts:39`) — `skip?: number`, `limit?: number | { id?: string }`, `historyItemRefs?: readonly string[]`, `filterText?: string`.
  - `SourceControlHistoryItem` interface (`scmHistoryProvider.d.ts:52`) — `id: string`, `parentIds: string[]`, `subject: string`, `message: string`, `displayId?`, `author?`, `authorEmail?`, `authorIcon?: IconPath`, `timestamp?: number`, `statistics?: SourceControlHistoryItemStatistics`, `references?: SourceControlHistoryItemRef[]`, `tooltip?: MarkdownString | MarkdownString[]`.
  - `SourceControlHistoryItemRef` interface (`scmHistoryProvider.d.ts:67`) — `id`, `name`, `description?`, `revision?`, `category?`, `icon?: IconPath`.
  - `SourceControlHistoryItemChange` interface (`scmHistoryProvider.d.ts:76`) — `uri: Uri`, `originalUri: Uri | undefined`, `modifiedUri: Uri | undefined`.
  - `SourceControlHistoryItemRefsChangeEvent` interface (`scmHistoryProvider.d.ts:82`) — `added`, `removed`, `modified` (all `readonly SourceControlHistoryItemRef[]`), `silent: boolean`.

- **Control flow:** Host calls `provideHistoryItems` to populate the timeline/history view; pagination is managed via `skip`/`limit` options. When the user expands a commit's diff, host calls `provideHistoryItemChanges`. AI chat context flows via `resolveHistoryItemChatContext` (whole commit) and `resolveHistoryItemChangeRangeChatContext` (per-file range). `onDidChangeHistoryItemRefs` pushes change events; `silent: true` suppresses automatic UI refresh.

- **Data flow:** Host → extension: raw string IDs (commit hashes / ref names), pagination options, file path strings. Extension → host: structured `SourceControlHistoryItem[]` (with author metadata, timestamps, statistics), `SourceControlHistoryItemRef[]` (branch/tag objects), `SourceControlHistoryItemChange[]` (before/after `Uri` pairs for diff computation), and plain `string` chat context snippets.

- **Dependencies:** `SourceControl` (from stable `scm.createSourceControl`), `Uri`, `MarkdownString`, `CancellationToken`, `Event`, `ProviderResult`, `IconPath` (from stable `vscode.d.ts`).

---

#### `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.languageModelCapabilities.d.ts`, `vscode.proposed.languageModelSystem.d.ts`, `vscode.proposed.languageModelThinkingPart.d.ts`, `vscode.proposed.languageModelProxy.d.ts`

- **Role:** Four small proposed APIs that extend the stable `lm`/`LanguageModelChat` surface in `vscode.d.ts`. Together they cover: model capability discovery, system-role messages, streaming reasoning ("thinking") tokens, and a proxy URI for exposing the local Copilot Chat model endpoint to non-VS Code consumers.

- **Key symbols:**
  - `LanguageModelChat.capabilities` (`languageModelCapabilities.d.ts:14`) — `{ supportsToolCalling: boolean, supportsImageToText: boolean, editToolsHint?: readonly string[] }`. Extends the stable `LanguageModelChat` interface.
  - `LanguageModelChatMessageRole.System = 3` (`languageModelSystem.d.ts:15`) — adds a third value to the stable `LanguageModelChatMessageRole` enum (stable has `User=1`, `Assistant=2`).
  - `LanguageModelThinkingPart` class (`languageModelThinkingPart.d.ts:15`) — `value: string | string[]`, `id?: string`, `metadata?: { [key: string]: any }`. Constructor `(value, id?, metadata?)`. Extended `LanguageModelChatResponse.stream` type includes this part (`languageModelThinkingPart.d.ts:47`).
  - `LanguageModelChatMessage2` class (`languageModelThinkingPart.d.ts:58`) — parallel to stable `LanguageModelChatMessage` but `content` array also allows `LanguageModelThinkingPart`. Static `User(...)` and `Assistant(...)` factory methods.
  - `LanguageModelToolResultPart2` and `LanguageModelToolResult2` (`languageModelThinkingPart.d.ts:105–110`) — temporary alias subclasses for backwards compatibility.
  - `LanguageModelProxy` interface (`languageModelProxy.d.ts:7`) — `Disposable` + `uri: Uri` + `key: string`. Represents a running local model proxy server.
  - `lm.isModelProxyAvailable: boolean`, `lm.onDidChangeModelProxyAvailability: Event<void>`, `lm.getModelProxy(): Thenable<LanguageModelProxy>` (`languageModelProxy.d.ts:13–29`).

- **Control flow (languageModelThinkingPart):** `LanguageModelChat.sendRequest` (stable, `vscode.d.ts:20297`) returns `LanguageModelChatResponse`. With this proposed API, the `response.stream` async iterable can now yield `LanguageModelThinkingPart` items interleaved with `LanguageModelTextPart` items before the final answer. Extensions iterate with `for await` and dispatch on instance type.

- **Data flow (languageModelProxy):** Host checks Copilot extension auth state. If `isModelProxyAvailable` is true, calling `getModelProxy()` starts an HTTP proxy server and returns `{ uri: Uri, key: string }`. The caller uses `uri` and `key` to make OpenAI-compatible requests directly to the local server, bypassing the normal `lm` IPC bridge.

- **Dependencies:** Stable `LanguageModelChat`, `LanguageModelChatResponse`, `LanguageModelChatMessageRole`, `LanguageModelChatMessage`, `LanguageModelTextPart`, `LanguageModelToolCallPart`, `LanguageModelDataPart`, `LanguageModelToolResultPart`, `LanguageModelToolResult`, `CancellationToken`, `Disposable`, `Event`, `Uri` (all from `vscode.d.ts`).

---

#### `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.notebookExecution.d.ts`

- **Role:** Tiny proposed API augmenting `NotebookController` (stable `vscode.d.ts:16041`) with `createNotebookExecution(notebook): NotebookExecution`. Allows a notebook controller to signal that it is "busy" at the notebook level — not tied to any individual cell — so the UI enters an executing state without a cell-level task.

- **Key symbols:**
  - `NotebookExecution` interface (`notebookExecution.d.ts:14`) — `start(): void`, `end(): void`.
  - `NotebookController.createNotebookExecution(notebook: NotebookDocument): NotebookExecution` (`notebookExecution.d.ts:37`). Throws if another `NotebookExecution` or `NotebookCellExecution` is already active for the notebook.

- **Control flow:** Extension calls `createNotebookExecution(notebook)`, then `execution.start()` to transition the notebook to executing state. When done, calls `execution.end()` to return to idle. Only one execution (cell or notebook level) may be active at a time.

- **Data flow:** Host → extension: `NotebookDocument` reference. Extension → host: `start()` and `end()` signals with no payload, triggering UI state transitions.

- **Dependencies:** `NotebookController` (augmented stable interface), `NotebookDocument` (stable `vscode.d.ts`).

---

#### `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.fileSearchProvider2.d.ts`

- **Role:** Proposed API augmenting `workspace` namespace with `registerFileSearchProvider2(scheme, provider)`. Allows extensions to supply file-name fuzzy-search results for custom URI schemes (e.g., virtual file systems, remote file trees) invoked from Quick Open.

- **Key symbols:**
  - `FileSearchProvider2` interface (`fileSearchProvider2.d.ts:76`) — single method `provideFileSearchResults(pattern: string, options: FileSearchProviderOptions, token: CancellationToken): ProviderResult<Uri[]>`.
  - `FileSearchProviderOptions` interface (`fileSearchProvider2.d.ts:13`) — `folderOptions: Array<{ folder: Uri, includes: string[], excludes: GlobPattern[], followSymlinks: boolean, useIgnoreFiles: { local, parent, global: boolean } }>`, `session: object` (WeakRef-safe cache key), `maxResults: number`.
  - `workspace.registerFileSearchProvider2(scheme: string, provider: FileSearchProvider2): Disposable` (`fileSearchProvider2.d.ts:102`).

- **Control flow:** On every Quick Open keystroke, for each workspace folder whose URI scheme matches the registered scheme, host calls `provideFileSearchResults(pattern, options, token)`. The provider should apply a relaxed (case-insensitive, subsequence) match and return up to `maxResults` URIs. The `session` object serves as a cache key; providers should use `WeakRef`/`WeakMap` to avoid memory leaks.

- **Data flow:** Host → extension: fuzzy `pattern` string, per-folder include/exclude glob lists, `maxResults`, and a `session` object. Extension → host: flat `Uri[]` of matching file paths. Host applies its own scoring/highlighting on returned URIs.

- **Dependencies:** `Uri`, `GlobPattern`, `CancellationToken`, `Disposable`, `ProviderResult` (all from stable `vscode.d.ts`).

---

### Cross-Cutting Synthesis

The `src/vscode-dts/` partition defines a **provider-pattern RPC contract** between the VS Code host and any extension. Every interaction follows one of three shapes:

1. **Registration** — extension calls `register*` or `create*` on a namespace, receiving a `Disposable`. The host stores the provider handle keyed by a string ID and extension identity. `Disposable.dispose()` deregisters it.
2. **Provider call** — host invokes a method on the registered provider interface, passing structured value types (`Uri`, `Position`, `Range`, context objects) and a `CancellationToken`. The provider returns `ProviderResult<T>` — either a synchronous value, `undefined`/`null`, or a `Thenable`.
3. **Event push** — host fires a typed `Event<T>` to all subscribers; subscribers are `(e: T) => any` functions attached via `event(listener, thisArgs?, disposables?)`.

For a Rust/Tauri host to reproduce this contract, it must implement:

- A **serialization layer** for all value types that cross the boundary: `Uri` (5-field struct), `Position` (line/character i32 pair, UTF-16 offsets), `Range`, `TextDocument` metadata, `FileStat`, `Diagnostic`, `DebugSession`, `NotebookDocument`, etc. All of these arrive as JSON in the current Electron IPC protocol.
- A **provider registry** per namespace: commands, languages, SCM, debug, notebooks, tasks, authentication, lm, tests, chat — each keyed by a string ID or `DocumentSelector`. The registry dispatches calls to the correct extension-host-side handler.
- An **async call protocol**: every provider method is async (Thenable-returning). The Tauri host must support sending a request with a correlation ID and resolving the response when it arrives, with cancellation propagated via a `CancellationToken` whose `isCancellationRequested` flip is delivered to the extension side.
- **Streaming support** for `LanguageModelChatResponse.stream` — an `AsyncIterable<LanguageModelTextPart | LanguageModelToolCallPart | LanguageModelDataPart | LanguageModelThinkingPart | unknown>`. The host must be able to push incremental parts to the awaiting extension code.
- **FileSystem dispatch**: `workspace.fs` methods must route through any registered `FileSystemProvider` for the relevant URI scheme before falling back to the native FS.
- **DAP proxy**: the `debug` namespace exposes raw DAP messages (`DebugProtocolMessage`, `DebugProtocolSource`, `DebugProtocolBreakpoint`) as opaque pass-through objects; the Tauri host must bridge these between extension and the actual debug adapter process.
- **Notebook execution state machine**: `NotebookCellExecution.start()` / `end()` and `NotebookExecution.start()` / `end()` must atomically flip cell/notebook UI state; the host enforces single-execution-at-a-time invariants.

The 167 proposed API files each augment one of these existing namespaces via additional `declare module 'vscode'` blocks; they do not introduce new IPC patterns, only extend the set of registered provider types and event payloads.

---

### Out-of-Partition References

The types declared in `src/vscode-dts/vscode.d.ts` are implemented in these other partitions:

- **Extension Host IPC / API adapter** — `src/vs/workbench/api/common/extHost*.ts` (e.g., `extHostCommands.ts`, `extHostLanguageFeatures.ts`, `extHostDebugService.ts`, `extHostNotebook.ts`, `extHostLanguageModels.ts`). These translate the TypeScript API surface into IPC message calls.
- **Main thread API stubs** — `src/vs/workbench/api/browser/mainThread*.ts` — receive IPC messages on the workbench side and dispatch to internal services.
- **IPC protocol definitions** — `src/vs/workbench/services/extensions/common/extHostProtocol.ts` — defines the `ProxyIdentifier`s and message schemas that correspond to every registered provider type.
- **Debug Adapter Protocol bridge** — `src/vs/workbench/contrib/debug/` — implements `DebugSession.customRequest`, `startDebugging`, `asDebugSourceUri`, and the breakpoint registry.
- **Notebook execution engine** — `src/vs/workbench/contrib/notebook/` — implements `NotebookController`, `NotebookCellExecution`, and the proposed `NotebookExecution` state machine.
- **Language Model / Copilot service** — `src/vs/workbench/contrib/chat/` and `src/vs/workbench/contrib/languageModels/` — back `lm.selectChatModels`, `lm.invokeTool`, `lm.registerLanguageModelChatProvider`, and the proxy feature.
- **SCM service** — `src/vs/workbench/contrib/scm/` — backs `scm.createSourceControl` and the `historyProvider` hook.
- **Terminal service** — `src/vs/workbench/contrib/terminal/` — backs `window.registerTerminalCompletionProvider` and the `TerminalCompletionContext` delivery.
- **File search service** — `src/vs/workbench/services/search/` — backs `workspace.registerFileSearchProvider2` and drives Quick Open file search.
- **Extension manifest / package.json schema** — `src/vs/workbench/services/extensions/common/extensionsRegistry.ts` — the `debugVisualizers`, `languageModelTools`, `mcpServerDefinitionProviders`, `languageModelChatProviders` contribution points referenced in the proposed API JSDoc are validated here.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder 6: vscode.d.ts - Public Extension API Surface

## Research Question
What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

## Analysis Focus
Partition 6 examines `src/vscode-dts/` — the canonical TypeScript type definitions that define the public extension API. Any Rust/Tauri port must preserve these interfaces to maintain extension compatibility.

---

## Core Patterns Found

#### Pattern 1: Top-Level Namespaces (API Module Organization)
**Where:** `src/vscode-dts/vscode.d.ts:10973`, `11069`, `13797`, `14722`, `17283`, `16652`, `18091`
**What:** Named namespaces that group related functionality (commands, window, workspace, languages, debug, scm, authentication, etc.). These are the primary extension API entry points.

```typescript
export namespace commands {
	export function registerCommand(command: string, callback: (...args: any[]) => any, thisArg?: any): Disposable;
	export function registerTextEditorCommand(command: string, callback: (textEditor: TextEditor, edit: TextEditorEdit, ...args: any[]) => void, thisArg?: any): Disposable;
	export function executeCommand(command: string, ...rest: any[]): Thenable<any>;
}

export namespace window {
	export const tabGroups: TabGroups;
	export let activeTextEditor: TextEditor | undefined;
	export let visibleTextEditors: readonly TextEditor[];
	export const onDidChangeActiveTextEditor: Event<TextEditor | undefined>;
	export const onDidChangeVisibleTextEditors: Event<readonly TextEditor[]>;
	export const onDidChangeTextEditorSelection: Event<TextEditorSelectionChangeEvent>;
}

export namespace workspace {
	export const fs: FileSystem;
	export const workspaceFolders: readonly WorkspaceFolder[] | undefined;
	export const name: string | undefined;
	export function openTextDocument(uri: Uri): Thenable<TextDocument>;
}

export namespace languages {
	export function getLanguages(): Thenable<string[]>;
	export function setTextDocumentLanguage(document: TextDocument, languageId: string): Thenable<TextDocument>;
	export function registerCompletionItemProvider(selector: DocumentSelector, provider: CompletionItemProvider, ...triggerCharacters: string[]): Disposable;
	export function registerDefinitionProvider(selector: DocumentSelector, provider: DefinitionProvider): Disposable;
}

export namespace debug {
	export let activeDebugSession: DebugSession | undefined;
	export let activeDebugConsole: DebugConsole;
	export let breakpoints: readonly Breakpoint[];
	export const onDidChangeActiveDebugSession: Event<DebugSession | undefined>;
	export const onDidStartDebugSession: Event<DebugSession>;
}

export namespace scm {
	export const inputBox: SourceControlInputBox;
	export function createSourceControl(id: string, label: string, rootUri?: Uri): SourceControl;
}

export namespace authentication {
	export function getSession(providerId: string, scopeListOrRequest: ReadonlyArray<string> | AuthenticationWwwAuthenticateRequest, options: AuthenticationGetSessionOptions & { createIfNone: true }): Thenable<AuthenticationSession>;
	export function registerAuthenticationProvider(id: string, label: string, provider: AuthenticationProvider): Disposable;
}
```

**Variations / call-sites:** All major IDE features (tasks:9347, env:10739, l10n:18192, tests:18271, chat:20111, lm:20732) follow this namespace pattern.

---

#### Pattern 2: Event-Driven Architecture with Event<T> Interface
**Where:** `src/vscode-dts/vscode.d.ts:1755`
**What:** Defines the contract for observable state changes. Extensions subscribe to events using a callable interface pattern.

```typescript
export interface Event<T> {
	(listener: (e: T) => any, thisArgs?: any, disposables?: Disposable[]): Disposable;
}

export class EventEmitter<T> {
	event: Event<T>;
	fire(data: T): void;
	dispose(): void;
}
```

Used throughout all namespaces:
- `window.onDidChangeActiveTextEditor: Event<TextEditor | undefined>`
- `window.onDidChangeTextEditorSelection: Event<TextEditorSelectionChangeEvent>`
- `debug.onDidChangeActiveDebugSession: Event<DebugSession | undefined>`
- `debug.onDidStartDebugSession: Event<DebugSession>`
- `workspace.onDidChangeTextDocument: Event<TextDocumentChangeEvent>`

**Variations / call-sites:** 50+ event definitions across all namespaces; EventEmitter used for extension-provided events.

---

#### Pattern 3: Provider Interface Pattern (Language Intelligence)
**Where:** `src/vscode-dts/vscode.d.ts:2925`, `5189`, `3144`, `2745`
**What:** Interfaces that extensions implement to provide IDE capabilities (completion, definition, hover, code actions, etc.). Registration functions return Disposable for cleanup.

```typescript
// Definition Provider
export interface DefinitionProvider {
	provideDefinition(document: TextDocument, position: Position, token: CancellationToken): ProviderResult<Definition | DefinitionLink[]>;
}

// Completion Item Provider
export interface CompletionItemProvider<T extends CompletionItem = CompletionItem> {
	provideCompletionItems(document: TextDocument, position: Position, token: CancellationToken, context: CompletionContext): ProviderResult<T[] | CompletionList<T>>;
	resolveCompletionItem?(item: T, token: CancellationToken): ProviderResult<T>;
}

// Hover Provider
export interface HoverProvider {
	provideHover(document: TextDocument, position: Position, token: CancellationToken): ProviderResult<Hover>;
}

// Code Action Provider
export interface CodeActionProvider<T extends CodeAction = CodeAction> {
	provideCodeActions(document: TextDocument, range: Range, context: CodeActionContext, token: CancellationToken): ProviderResult<(Command | T)[]>;
	resolveCodeAction?(codeAction: T, token: CancellationToken): ProviderResult<T>;
}

// Registration pattern
export function registerCompletionItemProvider(selector: DocumentSelector, provider: CompletionItemProvider, ...triggerCharacters: string[]): Disposable;
export function registerDefinitionProvider(selector: DocumentSelector, provider: DefinitionProvider): Disposable;
export function registerHoverProvider(selector: DocumentSelector, provider: HoverProvider): Disposable;
export function registerCodeActionsProvider(selector: DocumentSelector, provider: CodeActionProvider, metadata?: CodeActionProviderMetadata): Disposable;
```

**Variations / call-sites:** 20+ provider types including ImplementationProvider, TypeDefinitionProvider, ReferenceProvider, RenameProvider, DocumentHighlightProvider, DocumentSymbolProvider, CodeLensProvider, SignatureHelpProvider, OnTypeFormattingEditProvider, DocumentFormattingEditProvider, etc. (lines 2925-5800).

---

#### Pattern 4: Core Data Types (Position, Range, Uri, TextDocument, TextEditor)
**Where:** `src/vscode-dts/vscode.d.ts:269`, `408`, `1439`, `88`, `1258`
**What:** Immutable value types and reference types that represent code coordinates and document state. These are the primitive building blocks for editor operations.

```typescript
export class Position {
	readonly line: number;
	readonly character: number;
	constructor(line: number, character: number);
	isBefore(other: Position): boolean;
	isBeforeOrEqual(other: Position): boolean;
	isAfter(other: Position): boolean;
	isAfterOrEqual(other: Position): boolean;
	isEqual(other: Position): boolean;
	compareTo(other: Position): number;
	translate(lineDelta?: number, characterDelta?: number): Position;
	with(line?: number, character?: number): Position;
}

export class Range {
	readonly start: Position;
	readonly end: Position;
	constructor(start: Position | number, end: Position | number);
	isEmpty: boolean;
	isSingleLine: boolean;
	contains(positionOrRange: Position | Range): boolean;
	isEqual(other: Range): boolean;
	intersection(range: Range): Range | undefined;
	union(other: Range): Range;
	with(start?: Position, end?: Position): Range;
}

export class Uri {
	static parse(value: string, strict?: boolean): Uri;
	static file(path: string): Uri;
	static joinPath(base: Uri, ...pathSegments: string[]): Uri;
	static from(components: { scheme?: string; authority?: string; path?: string; query?: string; fragment?: string }): Uri;
	readonly scheme: string;
	readonly authority: string;
	readonly path: string;
	readonly query: string;
	readonly fragment: string;
	readonly fsPath: string;
	with(change: { scheme?: string; authority?: string; path?: string; query?: string; fragment?: string }): Uri;
	toString(skipEncoding?: boolean): string;
}

export interface TextDocument {
	readonly uri: Uri;
	readonly fileName: string;
	readonly isUntitled: boolean;
	readonly languageId: string;
	readonly version: number;
	readonly isDirty: boolean;
	readonly isClosed: boolean;
	readonly lineCount: number;
	save(): Thenable<boolean>;
	lineAt(line: number): TextLine;
	lineAt(position: Position): TextLine;
	offsetAt(position: Position): number;
	positionAt(offset: number): Position;
	getText(range?: Range): string;
	getWordRangeAtPosition(position: Position, regex?: RegExp): Range | undefined;
	validateRange(range: Range): Range;
	validatePosition(position: Position): Position;
}

export interface TextEditor {
	readonly document: TextDocument;
	selection: Selection;
	selections: readonly Selection[];
	readonly visibleRanges: readonly Range[];
	options: TextEditorOptions;
	readonly viewColumn: ViewColumn | undefined;
	edit(callback: (editBuilder: TextEditorEdit) => void, options?: { undoStopBefore: boolean; undoStopAfter: boolean }): Thenable<boolean>;
	insertSnippet(snippet: SnippetString, location?: Position | Range | readonly Position[], options?: { undoStopBefore: boolean; undoStopAfter: boolean }): Thenable<boolean>;
	setDecorations(decorationType: TextEditorDecorationType, rangesOrOptions: Range[] | TextEditorDecoration[]): void;
	revealRange(range: Range, revealType?: TextEditorRevealType): void;
	show(column?: ViewColumn): void;
	hide(): void;
}
```

**Variations / call-sites:** Selection:448, TextLine:49, TextEditorEdit:1400, Range used in 200+ interface definitions.

---

#### Pattern 5: Terminal Integration (Shell Execution & Management)
**Where:** `src/vscode-dts/vscode.d.ts:7669`, `7828`, `7944`
**What:** Interfaces for terminal creation, execution, and shell integration. Extensions can create terminals, execute commands, and respond to shell events.

```typescript
export interface Terminal {
	readonly name: string;
	readonly processId: Thenable<number | undefined>;
	readonly creationOptions: Readonly<TerminalOptions | ExtensionTerminalOptions>;
	readonly exitStatus: TerminalExitStatus | undefined;
	readonly state: TerminalState;
	readonly shellIntegration: TerminalShellIntegration | undefined;
	sendText(text: string, addNewLine?: boolean): void;
	show(preserveFocus?: boolean): void;
	hide(): void;
	dispose(): void;
}

export interface TerminalShellIntegration {
	readonly nonce: string;
	readonly executeCommand(commandLine: string | TerminalShellExecutionCommandLine, eventEmitter?: EventEmitter<void>): Promise<number>;
	readonly onDidStartTerminalShellExecution: Event<TerminalShellExecutionStartEvent>;
	readonly onDidEndTerminalShellExecution: Event<TerminalShellExecutionEndEvent>;
}

export interface TerminalShellExecution {
	readonly commandLine: TerminalShellExecutionCommandLine;
	readonly exitCode: number | undefined;
	readonly exitReason: TerminalShellExecutionExitReason | undefined;
}

export namespace window {
	export const terminals: readonly Terminal[];
	export let activeTerminal: Terminal | undefined;
	export const onDidOpenTerminal: Event<Terminal>;
	export const onDidCloseTerminal: Event<Terminal>;
	export const onDidChangeActiveTerminal: Event<Terminal | undefined>;
	export const onDidChangeTerminalState: Event<Terminal>;
	export const onDidChangeTerminalDimensions: Event<TerminalDimensionsChangeEvent>;
	export const onDidChangeTerminalShellIntegration: Event<TerminalShellIntegrationChangeEvent>;
	export function createTerminal(name?: string, shellPath?: string, shellArgs?: string[] | string): Terminal;
	export function createTerminalFromOptions(options: TerminalOptions | ExtensionTerminalOptions): Terminal;
}
```

**Variations / call-sites:** TerminalOptions:12462, TerminalExitStatus:12785, TerminalDimensions:12770, TerminalProfileProvider:8220.

---

#### Pattern 6: Debugging Protocol Integration
**Where:** `src/vscode-dts/vscode.d.ts:17283`, `16673`
**What:** DAP (Debug Adapter Protocol) opaque types and session management. Extensions register debug providers and interact with active debug sessions.

```typescript
export namespace debug {
	export let activeDebugSession: DebugSession | undefined;
	export let activeDebugConsole: DebugConsole;
	export let breakpoints: readonly Breakpoint[];
	export const onDidChangeActiveDebugSession: Event<DebugSession | undefined>;
	export const onDidStartDebugSession: Event<DebugSession>;
	export const onDidReceiveDebugSessionCustomEvent: Event<DebugSessionCustomEvent>;
	export const onDidTerminateDebugSession: Event<DebugSession>;
	export const onDidChangeBreakpoints: Event<BreakpointsChangeEvent>;
	export function registerDebugAdapterDescriptorFactory(debugType: string, factory: DebugAdapterDescriptorFactory): Disposable;
	export function registerDebugAdapterTrackerFactory(debugType: string, factory: DebugAdapterTrackerFactory): Disposable;
}

export interface DebugProtocolMessage {
	// Properties: see [ProtocolMessage details](https://microsoft.github.io/debug-adapter-protocol/specification#Base_Protocol_ProtocolMessage).
}

export interface DebugProtocolSource {
	// Properties: see [Source details](https://microsoft.github.io/debug-adapter-protocol/specification#Types_Source).
}

export interface DebugProtocolBreakpoint {
	// Properties: see [Breakpoint details](https://microsoft.github.io/debug-adapter-protocol/specification#Types_Breakpoint).
}

export interface DebugConfiguration {
	type: string;
	name: string;
	request: string;
	[key: string]: any;
}
```

**Variations / call-sites:** DebugSession, DebugConsole, Breakpoint, BreakpointsChangeEvent, DebugAdapterDescriptorFactory, DebugAdapterTrackerFactory.

---

#### Pattern 7: Source Control (SCM) Provider Pattern
**Where:** `src/vscode-dts/vscode.d.ts:16652`
**What:** Abstraction for source control systems (Git, etc.). Extensions create SourceControl instances with InputBox, ResourceGroups, and Commands.

```typescript
export namespace scm {
	export const inputBox: SourceControlInputBox;
	export function createSourceControl(id: string, label: string, rootUri?: Uri): SourceControl;
}

export interface SourceControl {
	readonly id: string;
	readonly label: string;
	readonly rootUri: Uri;
	inputBox: SourceControlInputBox;
	readonly count?: number;
	readonly statusBarCommands?: Command[];
	readonly quickDiffProvider?: QuickDiffProvider;
	createResourceGroup(id: string, label: string): SourceControlResourceGroup;
	createResourceGroup(id: string, label: string, hideWhenEmpty: boolean): SourceControlResourceGroup;
	dispose(): void;
}

export interface SourceControlResourceGroup {
	readonly id: string;
	label: string;
	hideWhenEmpty?: boolean;
	resourceStates: SourceControlResourceState[];
	dispose(): void;
}
```

**Variations / call-sites:** SourceControlInputBox, SourceControlResourceState, QuickDiffProvider.

---

#### Pattern 8: Task Provider & Execution
**Where:** `src/vscode-dts/vscode.d.ts:9347`
**What:** Extension-provided task definitions and execution model. Extensions register providers that return Task objects; execution returns TaskExecution for monitoring.

```typescript
export namespace tasks {
	export function registerTaskProvider(type: string, provider: TaskProvider): Disposable;
	export function fetchTasks(filter?: TaskFilter): Thenable<Task[]>;
	export function executeTask(task: Task): Thenable<TaskExecution>;
	export const taskExecutions: readonly TaskExecution[];
	export const onDidStartTask: Event<TaskStartEvent>;
	export const onDidEndTask: Event<TaskEndEvent>;
	export const onDidStartTaskProcess: Event<TaskProcessStartEvent>;
	export const onDidEndTaskProcess: Event<TaskProcessEndEvent>;
}

export interface TaskProvider {
	provideTasks(token: CancellationToken): ProviderResult<Task[]>;
	resolveTask(task: Task, token: CancellationToken): ProviderResult<Task>;
}

export interface Task {
	definition: TaskDefinition;
	name: string;
	detail?: string;
	execution?: TaskExecution | ProcessExecution | ShellExecution | CustomExecution;
	isBackground?: boolean;
	source: string;
	group?: TaskGroup;
	presentationOptions?: TaskPresentationOptions;
	problemMatchers: string[];
	runOptions?: RunOptions;
}
```

**Variations / call-sites:** TaskExecution, ProcessExecution, ShellExecution, CustomExecution, TaskGroup, TaskDefinition, TaskFilter.

---

#### Pattern 9: Authentication Provider Pattern
**Where:** `src/vscode-dts/vscode.d.ts:18091`
**What:** Extensible authentication system. Extensions implement AuthenticationProvider interface; built-in and third-party providers are queried via namespace.

```typescript
export namespace authentication {
	export function getSession(providerId: string, scopeListOrRequest: ReadonlyArray<string> | AuthenticationWwwAuthenticateRequest, options: AuthenticationGetSessionOptions & { createIfNone: true | AuthenticationGetSessionPresentationOptions }): Thenable<AuthenticationSession>;
	export function registerAuthenticationProvider(id: string, label: string, provider: AuthenticationProvider, options?: AuthenticationProviderOptions): Disposable;
	export const onDidChangeSessions: Event<AuthenticationSessionsChangeEvent>;
}

export interface AuthenticationProvider {
	readonly onDidChangeSessions: Event<AuthenticationSessionsChangeEvent>;
	getSessions(scopes?: string[]): Thenable<readonly AuthenticationSession[]>;
	getSession(scopes: string[], options: AuthenticationGetSessionOptions): Thenable<AuthenticationSession | undefined>;
	createSession(scopes: string[], options: AuthenticationCreateSessionOptions): Thenable<AuthenticationSession>;
	removeSession(sessionId: string): Thenable<void>;
}

export interface AuthenticationSession {
	readonly id: string;
	readonly accessToken: string;
	readonly refreshToken?: string;
	readonly idToken?: string;
	readonly account: AuthenticationSessionAccountInformation;
	readonly scopes: readonly string[];
}
```

**Variations / call-sites:** AuthenticationGetSessionOptions, AuthenticationWwwAuthenticateRequest, AuthenticationSessionsChangeEvent.

---

#### Pattern 10: Disposable Resource Management
**Where:** `src/vscode-dts/vscode.d.ts:1712`
**What:** Universal cleanup pattern. All registration functions return Disposable; extensions manage subscriptions and provider lifetimes via dispose().

```typescript
export class Disposable {
	static from(...disposableLikes: { dispose: () => any }[]): Disposable;
	constructor(callOnDispose: () => any);
	dispose(): any;
}
```

Used in:
- `registerCommand(...): Disposable`
- `registerTextEditorCommand(...): Disposable`
- `registerCompletionItemProvider(...): Disposable`
- `registerDefinitionProvider(...): Disposable`
- `registerHoverProvider(...): Disposable`
- `createSourceControl(...): SourceControl` (has `dispose()` method)
- Event subscriptions: `event(listener): Disposable`

**Variations / call-sites:** 100+ registration and subscription functions return Disposable for cleanup.

---

## Summary

The vscode.d.ts API surface demonstrates 10 core architectural patterns that any Rust/Tauri port must replicate:

1. **Namespace organization** for IDE features (10+ namespaces covering commands, window, workspace, languages, debug, scm, authentication, tasks, tests, chat, LM)
2. **Event-driven reactivity** with typed Event<T> interfaces and EventEmitter
3. **Provider pattern** for language intelligence (20+ provider types for LSP-like capabilities)
4. **Core immutable value types** (Position, Range, Uri, Selection) for coordinate and location handling
5. **Terminal management** with shell integration and execution tracking
6. **Debug protocol abstraction** (DAP opaque types) for debugger integration
7. **SCM abstraction** for version control providers
8. **Task provider pattern** for build/run task registration and execution
9. **Pluggable authentication** for multi-provider credential management
10. **Universal Disposable pattern** for resource lifecycle management

These patterns define the host-to-extension boundary that must be preserved. A Tauri/Rust port would need to:
- Serialize/deserialize these types across process boundaries (host in Rust, extensions in Wasm or RPC)
- Implement async/event bridging for the Event<T> callback pattern
- Map provider registrations to the new host architecture
- Maintain binary/protocol compatibility with Position, Range, Uri serialization
- Implement terminal, debug, and SCM subsystems in Rust while exposing identical APIs

The TypeScript definitions themselves (in src/vscode-dts/) would serve as the canonical API contract that both old (Electron) and new (Tauri) runtimes must honor.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
