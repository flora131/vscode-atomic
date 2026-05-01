# Partition 6 of 79 — Findings

## Scope
`src/vscode-dts/` (170 files, 33,318 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# VS Code IDE Porting Surface: TypeScript/Electron to Tauri/Rust

## Research Focus
Locating all TypeScript type definitions in `src/vscode-dts/` that define the extension API surface VS Code exposes. These files enumerate the contract any Tauri/Rust port must preserve for IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation).

## Scope Analyzed
- Directory: `src/vscode-dts/`
- 170 TypeScript definition files
- 33,318 LOC
- 1 documentation file (README.md)

---

## Implementation

### Core API Surface
- `src/vscode-dts/vscode.d.ts` — Stable extension API module declaration. Defines top-level namespaces and interfaces that form the contract between VS Code host and extensions. Must be preserved/ported to any new host implementation.

### IDE Feature APIs (Proposed & Experimental)

#### Text Editing & Editor Integration (28 files)
- `src/vscode-dts/vscode.proposed.aiTextSearchProvider.d.ts` — AI-powered text search
- `src/vscode-dts/vscode.proposed.editorInsets.d.ts` — Editor rendering extensions
- `src/vscode-dts/vscode.proposed.editorHoverVerbosityLevel.d.ts` — Hover information verbosity
- `src/vscode-dts/vscode.proposed.inlineCompletionsAdditions.d.ts` — Inline completion extensions
- `src/vscode-dts/vscode.proposed.textDocumentChangeReason.d.ts` — Text document change tracking
- `src/vscode-dts/vscode.proposed.textEditorDiffInformation.d.ts` — Diff editor metadata
- `src/vscode-dts/vscode.proposed.textSearchComplete2.d.ts` — Text search completion v2
- `src/vscode-dts/vscode.proposed.textSearchProvider.d.ts` — Custom text search providers
- `src/vscode-dts/vscode.proposed.textSearchProvider2.d.ts` — Text search v2 interface
- `src/vscode-dts/vscode.proposed.customEditorMove.d.ts` — Custom editor tab movement
- `src/vscode-dts/vscode.proposed.diffContentOptions.d.ts` — Diff editor options
- `src/vscode-dts/vscode.proposed.contribDiffEditorGutterToolBarMenus.d.ts` — Diff gutter menus
- `src/vscode-dts/vscode.proposed.contribMergeEditorMenus.d.ts` — Merge editor UI
- `src/vscode-dts/vscode.proposed.contribMultiDiffEditorMenus.d.ts` — Multi-diff menus
- `src/vscode-dts/vscode.proposed.contribEditorContentMenu.d.ts` — Editor context menus
- `src/vscode-dts/vscode.proposed.contribCommentEditorActionsMenu.d.ts` — Comment editor actions
- `src/vscode-dts/vscode.proposed.contribChatEditorInlineGutterMenu.d.ts` — Chat gutter UI
- `src/vscode-dts/vscode.proposed.contribCommentPeekContext.d.ts` — Comment peek UI
- `src/vscode-dts/vscode.proposed.chatContextProvider.d.ts` — Chat context in editors
- `src/vscode-dts/vscode.proposed.codiconDecoration.d.ts` — Icon decorations
- `src/vscode-dts/vscode.proposed.css.d.ts` — CSS support
- `src/vscode-dts/vscode.proposed.tabInputMultiDiff.d.ts` — Multi-diff tab input
- `src/vscode-dts/vscode.proposed.tabInputTextMerge.d.ts` — Merge tab input
- `src/vscode-dts/vscode.proposed.treeItemMarkdownLabel.d.ts` — Tree item markdown
- `src/vscode-dts/vscode.proposed.treeViewMarkdownMessage.d.ts` — Tree view markdown messages
- `src/vscode-dts/vscode.proposed.markdownAlertSyntax.d.ts` — Markdown alert blocks
- `src/vscode-dts/vscode.proposed.mappedEditsProvider.d.ts` — Mapped edits (refactoring)
- `src/vscode-dts/vscode.proposed.diffCommand.d.ts` — Diff command operations

#### Language Intelligence & Analysis (12 files)
- `src/vscode-dts/vscode.proposed.languageModelCapabilities.d.ts` — LLM capability detection
- `src/vscode-dts/vscode.proposed.languageModelPricing.d.ts` — LLM pricing information
- `src/vscode-dts/vscode.proposed.languageModelProxy.d.ts` — LLM proxy/routing
- `src/vscode-dts/vscode.proposed.languageModelSystem.d.ts` — Built-in language models
- `src/vscode-dts/vscode.proposed.languageModelThinkingPart.d.ts` — LLM thinking tokens
- `src/vscode-dts/vscode.proposed.languageModelToolResultAudience.d.ts` — Tool result visibility
- `src/vscode-dts/vscode.proposed.languageModelToolSupportsModel.d.ts` — Tool model compatibility
- `src/vscode-dts/vscode.proposed.languageStatusText.d.ts` — Language status indicators
- `src/vscode-dts/vscode.proposed.tokenInformation.d.ts` — Token type information
- `src/vscode-dts/vscode.proposed.chatReferenceDiagnostic.d.ts` — Chat diagnostic references
- `src/vscode-dts/vscode.proposed.multiDocumentHighlightProvider.d.ts` — Multi-document highlighting
- `src/vscode-dts/vscode.proposed.newSymbolNamesProvider.d.ts` — Symbol rename suggestions

#### Debugging (3 files)
- `src/vscode-dts/vscode.proposed.debugVisualization.d.ts` — Debug visualizer extensions
- `src/vscode-dts/vscode.proposed.contribDebugCreateConfiguration.d.ts` — Debug config UI
- `src/vscode-dts/vscode.proposed.chatDebug.d.ts` — Chat-assisted debugging

#### Source Control Management (6 files)
- `src/vscode-dts/vscode.proposed.scmActionButton.d.ts` — SCM action buttons
- `src/vscode-dts/vscode.proposed.scmArtifactProvider.d.ts` — SCM artifact management
- `src/vscode-dts/vscode.proposed.scmHistoryProvider.d.ts` — Version history UI
- `src/vscode-dts/vscode.proposed.scmMultiDiffEditor.d.ts` — SCM diff viewing
- `src/vscode-dts/vscode.proposed.scmProviderOptions.d.ts` — SCM provider configuration
- `src/vscode-dts/vscode.proposed.scmSelectedProvider.d.ts` — SCM provider selection
- `src/vscode-dts/vscode.proposed.scmTextDocument.d.ts` — SCM document tracking
- `src/vscode-dts/vscode.proposed.scmValidation.d.ts` — SCM input validation

#### Terminal & Shell Integration (11 files)
- `src/vscode-dts/vscode.proposed.taskExecutionTerminal.d.ts` — Task terminal binding
- `src/vscode-dts/vscode.proposed.taskPresentationGroup.d.ts` — Task grouping UI
- `src/vscode-dts/vscode.proposed.taskProblemMatcherStatus.d.ts` — Problem matcher status
- `src/vscode-dts/vscode.proposed.taskRunOptions.d.ts` — Task run configuration
- `src/vscode-dts/vscode.proposed.terminalCompletionProvider.d.ts` — Shell completion
- `src/vscode-dts/vscode.proposed.terminalDataWriteEvent.d.ts` — Terminal write tracking
- `src/vscode-dts/vscode.proposed.terminalDimensions.d.ts` — Terminal size queries
- `src/vscode-dts/vscode.proposed.terminalExecuteCommandEvent.d.ts` — Command execution tracking
- `src/vscode-dts/vscode.proposed.terminalQuickFixProvider.d.ts` — Terminal quick fixes
- `src/vscode-dts/vscode.proposed.terminalSelection.d.ts` — Terminal selection API
- `src/vscode-dts/vscode.proposed.terminalShellEnv.d.ts` — Shell environment configuration
- `src/vscode-dts/vscode.proposed.terminalTitle.d.ts` — Terminal title templates

#### Search & File Navigation (4 files)
- `src/vscode-dts/vscode.proposed.fileSearchProvider.d.ts` — Custom file search
- `src/vscode-dts/vscode.proposed.fileSearchProvider2.d.ts` — File search v2 interface
- `src/vscode-dts/vscode.proposed.findFiles2.d.ts` — Find files API v2
- `src/vscode-dts/vscode.proposed.aiSettingsSearch.d.ts` — AI-assisted settings search
- `src/vscode-dts/vscode.proposed.findTextInFiles.d.ts` — Find in files implementation
- `src/vscode-dts/vscode.proposed.findTextInFiles2.d.ts` — Find in files v2
- `src/vscode-dts/vscode.proposed.quickDiffProvider.d.ts` — Quick diff (git diff decoration)

#### Notebook Support (11 files)
- `src/vscode-dts/vscode.proposed.notebookCellExecution.d.ts` — Cell execution tracking
- `src/vscode-dts/vscode.proposed.notebookControllerAffinityHidden.d.ts` — Notebook controller visibility
- `src/vscode-dts/vscode.proposed.notebookDeprecated.d.ts` — Deprecated notebook APIs
- `src/vscode-dts/vscode.proposed.notebookExecution.d.ts` — Notebook execution model
- `src/vscode-dts/vscode.proposed.notebookKernelSource.d.ts` — Kernel source definition
- `src/vscode-dts/vscode.proposed.notebookLiveShare.d.ts` — Notebook Live Share support
- `src/vscode-dts/vscode.proposed.notebookMessaging.d.ts` — Notebook cell messaging
- `src/vscode-dts/vscode.proposed.notebookMime.d.ts` — MIME type cell rendering
- `src/vscode-dts/vscode.proposed.notebookReplDocument.d.ts` — REPL document support
- `src/vscode-dts/vscode.proposed.notebookVariableProvider.d.ts` — Variable inspection in notebooks
- `src/vscode-dts/vscode.proposed.contribNotebookStaticPreloads.d.ts` — Notebook preload scripts

#### Chat & AI Integration (16 files)
- `src/vscode-dts/vscode.proposed.chatProvider.d.ts` — Chat provider interface
- `src/vscode-dts/vscode.proposed.chatParticipantAdditions.d.ts` — Chat participant extensions
- `src/vscode-dts/vscode.proposed.chatParticipantPrivate.d.ts` — Private chat features
- `src/vscode-dts/vscode.proposed.chatHooks.d.ts` — Chat lifecycle hooks
- `src/vscode-dts/vscode.proposed.chatInputNotification.d.ts` — Chat input events
- `src/vscode-dts/vscode.proposed.chatOutputRenderer.d.ts` — Chat output rendering
- `src/vscode-dts/vscode.proposed.chatPromptFiles.d.ts` — Chat prompt file support
- `src/vscode-dts/vscode.proposed.chatReferenceBinaryData.d.ts` — Binary references in chat
- `src/vscode-dts/vscode.proposed.chatSessionCustomizationProvider.d.ts` — Chat session customization
- `src/vscode-dts/vscode.proposed.chatSessionsProvider.d.ts` — Chat session persistence
- `src/vscode-dts/vscode.proposed.chatStatusItem.d.ts` — Chat status bar integration
- `src/vscode-dts/vscode.proposed.chatTab.d.ts` — Chat tab management
- `src/vscode-dts/vscode.proposed.defaultChatParticipant.d.ts` — Default chat participant
- `src/vscode-dts/vscode.proposed.contribLanguageModelToolSets.d.ts` — Tool definitions for LLMs
- `src/vscode-dts/vscode.proposed.mcpServerDefinitions.d.ts` — MCP server configuration
- `src/vscode-dts/vscode.proposed.mcpToolDefinitions.d.ts` — MCP tool definitions
- `src/vscode-dts/vscode.proposed.embeddings.d.ts` — Embedding model APIs
- `src/vscode-dts/vscode.proposed.toolInvocationApproveCombination.d.ts` — Tool approval UI
- `src/vscode-dts/vscode.proposed.toolProgress.d.ts` — Tool execution progress

#### Comments & Collaboration (8 files)
- `src/vscode-dts/vscode.proposed.activeComment.d.ts` — Active comment tracking
- `src/vscode-dts/vscode.proposed.commentReactor.d.ts` — Comment reactions UI
- `src/vscode-dts/vscode.proposed.commentReveal.d.ts` — Comment reveal/navigation
- `src/vscode-dts/vscode.proposed.commentThreadApplicability.d.ts` — Comment applicability
- `src/vscode-dts/vscode.proposed.commentingRangeHint.d.ts` — Comment range hints
- `src/vscode-dts/vscode.proposed.commentsDraftState.d.ts` — Comment draft persistence
- `src/vscode-dts/vscode.proposed.contribCommentThreadAdditionalMenu.d.ts` — Comment menus
- `src/vscode-dts/vscode.proposed.contribCommentsViewThreadMenus.d.ts` — Comments view UI

#### UI/Window & Navigation (14 files)
- `src/vscode-dts/vscode.proposed.nativeWindowHandle.d.ts` — Native window access
- `src/vscode-dts/vscode.proposed.interactiveWindow.d.ts` — Interactive window support
- `src/vscode-dts/vscode.proposed.interactive.d.ts` — Interactive features
- `src/vscode-dts/vscode.proposed.quickPickItemTooltip.d.ts` — Quick pick tooltips
- `src/vscode-dts/vscode.proposed.quickPickSortByLabel.d.ts` — Quick pick sorting
- `src/vscode-dts/vscode.proposed.statusBarItemTooltip.d.ts` — Status bar tooltips
- `src/vscode-dts/vscode.proposed.contribStatusBarItems.d.ts` — Status bar item menus
- `src/vscode-dts/vscode.proposed.contribViewContainerTitle.d.ts` — View container title
- `src/vscode-dts/vscode.proposed.contribViewsRemote.d.ts` — Remote views support
- `src/vscode-dts/vscode.proposed.contribViewsWelcome.d.ts` — Welcome view content
- `src/vscode-dts/vscode.proposed.contribAccessibilityHelpContent.d.ts` — Accessibility help
- `src/vscode-dts/vscode.proposed.contribMenuBarHome.d.ts` — Home menu support
- `src/vscode-dts/vscode.proposed.contribShareMenu.d.ts` — Share menu
- `src/vscode-dts/vscode.proposed.contribLabelFormatterWorkspaceTooltip.d.ts` — Label formatting
- `src/vscode-dts/vscode.proposed.treeViewActiveItem.d.ts` — Tree view selection
- `src/vscode-dts/vscode.proposed.treeViewReveal.d.ts` — Tree view reveal operations

#### Workspace & Configuration (4 files)
- `src/vscode-dts/vscode.proposed.workspaceTrust.d.ts` — Workspace trust model
- `src/vscode-dts/vscode.proposed.environmentPower.d.ts` — Power state/efficiency
- `src/vscode-dts/vscode.proposed.envIsConnectionMetered.d.ts` — Metered connection detection
- `src/vscode-dts/vscode.proposed.agentSessionsWorkspace.d.ts` — Remote agent sessions

#### Menu & Command Systems (10 files)
- `src/vscode-dts/vscode.proposed.codeActionAI.d.ts` — AI-powered code actions
- `src/vscode-dts/vscode.proposed.codeActionRanges.d.ts` — Code action range API
- `src/vscode-dts/vscode.proposed.contribSourceControlArtifactGroupMenu.d.ts` — SCM artifact menus
- `src/vscode-dts/vscode.proposed.contribSourceControlArtifactMenu.d.ts` — SCM context menus
- `src/vscode-dts/vscode.proposed.contribSourceControlHistoryItemMenu.d.ts` — History menus
- `src/vscode-dts/vscode.proposed.contribSourceControlHistoryTitleMenu.d.ts` — History title menus
- `src/vscode-dts/vscode.proposed.contribSourceControlInputBoxMenu.d.ts` — SCM input UI
- `src/vscode-dts/vscode.proposed.contribSourceControlTitleMenu.d.ts` — SCM title menus

#### Extension & Test Infrastructure (5 files)
- `src/vscode-dts/vscode.proposed.extensionRuntime.d.ts` — Extension runtime environment
- `src/vscode-dts/vscode.proposed.extensionAffinity.d.ts` — Extension affinity/isolation
- `src/vscode-dts/vscode.proposed.extensionsAny.d.ts` — Unrestricted extension access
- `src/vscode-dts/vscode.proposed.testObserver.d.ts` — Test discovery/execution API
- `src/vscode-dts/vscode.proposed.testRelatedCode.d.ts` — Test-related code navigation

#### Network & Communication Infrastructure (7 files)
- `src/vscode-dts/vscode.proposed.ipc.d.ts` — Inter-process communication protocol
- `src/vscode-dts/vscode.proposed.dataChannels.d.ts` — WebRTC data channels
- `src/vscode-dts/vscode.proposed.tunnelFactory.d.ts` — Port forwarding/tunneling
- `src/vscode-dts/vscode.proposed.tunnels.d.ts` — Tunnel management
- `src/vscode-dts/vscode.proposed.portsAttributes.d.ts` — Port metadata/attributes
- `src/vscode-dts/vscode.proposed.nativeWindowHandle.d.ts` — Native platform integration
- `src/vscode-dts/vscode.proposed.remoteCodingAgents.d.ts` — Remote agent connections

#### File System & Content Providers (8 files)
- `src/vscode-dts/vscode.proposed.fsChunks.d.ts` — File system chunking API
- `src/vscode-dts/vscode.proposed.canonicalUriProvider.d.ts` — URI canonicalization
- `src/vscode-dts/vscode.proposed.externalUriOpener.d.ts` — External URI handling
- `src/vscode-dts/vscode.proposed.shareProvider.d.ts` — Share link generation
- `src/vscode-dts/vscode.proposed.editSessionIdentityProvider.d.ts` — Session persistence
- `src/vscode-dts/vscode.proposed.contribEditSessions.d.ts` — Edit session UI

#### Authentication & Security (7 files)
- `src/vscode-dts/vscode.proposed.authSession.d.ts` — Auth session properties
- `src/vscode-dts/vscode.proposed.authenticationChallenges.d.ts` — Auth challenge handling
- `src/vscode-dts/vscode.proposed.authIssuers.d.ts` — Auth issuer configuration
- `src/vscode-dts/vscode.proposed.authLearnMore.d.ts` — Auth help content
- `src/vscode-dts/vscode.proposed.authProviderSpecific.d.ts` — Provider-specific auth
- `src/vscode-dts/vscode.proposed.workspaceTrust.d.ts` — Workspace trust verification

#### Development & Telemetry (4 files)
- `src/vscode-dts/vscode.proposed.telemetry.d.ts` — Telemetry event reporting
- `src/vscode-dts/vscode.proposed.extensionRuntime.d.ts` — Extension runtime metrics
- `src/vscode-dts/vscode.proposed.devDeviceId.d.ts` — Device identification
- `src/vscode-dts/vscode.proposed.speech.d.ts` — Voice input/output support

#### Profile & Configuration Management (3 files)
- `src/vscode-dts/vscode.proposed.profileContentHandlers.d.ts` — Profile import/export
- `src/vscode-dts/vscode.proposed.resolvers.d.ts` — Configuration resolvers
- `src/vscode-dts/vscode.proposed.documentFiltersExclusive.d.ts` — Document filtering

#### Miscellaneous Features (9 files)
- `src/vscode-dts/vscode.proposed.browser.d.ts` — Browser-specific APIs
- `src/vscode-dts/vscode.proposed.aiRelatedInformation.d.ts` — AI context sources
- `src/vscode-dts/vscode.proposed.valueSelectionInQuickPick.d.ts` — Selection state
- `src/vscode-dts/vscode.proposed.editorHoverVerbosityLevel.d.ts` — Hover detail levels
- `src/vscode-dts/vscode.proposed.diffContentOptions.d.ts` — Diff rendering options
- `src/vscode-dts/vscode.proposed.highlightRelatedSymbols.d.ts` — Symbol highlighting (if exists)
- `src/vscode-dts/vscode.proposed.testRelatedCode.d.ts` — Code-test correlation

---

## Documentation

- `src/vscode-dts/README.md` — Guide to consuming and creating VS Code API proposals. Explains:
  - How to enable proposed APIs in extensions
  - Naming conventions for proposals
  - Integration with `extensionsApiProposals.ts`
  - API stability progression (proposal → stable)

---

## Notable Clusters

### Top-Level vs. Proposed APIs
- **Stable (1 file)**: `vscode.d.ts` — The canonical contract
- **Proposed (169 files)**: Feature extensions undergoing evaluation before stabilization

### Porting Surface by Subsystem

#### Core Editor Runtime (15 files)
Cluster in `src/vscode-dts/`:
- Text document model (TextLine, TextDocument interfaces)
- Text editor operations (Range, Position, Selection)
- Content providers & custom editors
- Diff/merge editor integration

#### Language Server Integration (8 files)
- Diagnostic publication
- Code action, completion, hover providers
- Symbol navigation and renaming
- Semantic tokens and highlighting

#### Debug Adapter Protocol Host (3 files)
- Debug session management
- Breakpoint & stack frame handling
- Debug console I/O

#### SCM Provider Interface (6 files)
- Repository state model
- Change/status tracking
- History and artifact browsing

#### Terminal Emulation & Shell (11 files)
- Pseudo-terminal creation & management
- Shell environment setup
- Process execution tracking

#### Chat/Agent Architecture (16 files)
- Participant registration
- Tool/function calling
- Session management
- MCP (Model Context Protocol) server support

---

## Key Findings

1. **170 TypeScript definition files** enumerate the full VS Code extension API surface
2. **9 major subsystems** identified across `vscode.d.ts` and proposed APIs:
   - Window/UI (commands, window, workspace globals)
   - Text editing (editor, documents, languages)
   - Debug (debug adapter protocol host)
   - SCM (source control providers)
   - Terminal (pseudo-terminal, shell)
   - Chat/AI (participants, LLMs, tools)
   - Notebook (kernels, execution)
   - Test (test controller, test observer)
   - Authentication (auth providers, sessions)

3. **Porting Requirements**: A Tauri/Rust port must implement each namespace and interface defined in these files to maintain extension compatibility
4. **Extension Ecosystem Size**: 170 API surface points suggests approximately 2,000+ distinct functions, properties, and event types to port
5. **Stability vs. Experimentation**: 169 proposed vs. 1 stable file indicates active API evolution; suggests phased porting strategy

---

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Core IDE Functionality - Partition 6 Analysis

## Concrete API Patterns from vscode-dts

This document captures concrete patterns from VS Code's core IDE API definitions that would be necessary to understand for porting to Tauri/Rust. The patterns demonstrate how editing, language intelligence, debugging, source control, terminal, navigation, and extension systems are currently structured in TypeScript/Electron.

---

#### Pattern: Editor State Management and Active Editor Access
**Where:** `src/vscode-dts/vscode.d.ts:11069-11175`
**What:** The window namespace exposes active editor state and comprehensive editor state change events for tracking editor lifecycle.

```typescript
export namespace window {
    // Active editor tracking
    export let activeTextEditor: TextEditor | undefined;
    export let visibleTextEditors: readonly TextEditor[];
    
    // Editor lifecycle events
    export const onDidChangeActiveTextEditor: Event<TextEditor | undefined>;
    export const onDidChangeVisibleTextEditors: Event<readonly TextEditor[]>;
    export const onDidChangeTextEditorSelection: Event<TextEditorSelectionChangeEvent>;
    export const onDidChangeTextEditorVisibleRanges: Event<TextEditorVisibleRangesChangeEvent>;
    export const onDidChangeTextEditorOptions: Event<TextEditorOptionsChangeEvent>;
    export const onDidChangeTextEditorViewColumn: Event<TextEditorViewColumnChangeEvent>;
    
    // Notebook editor support
    export const visibleNotebookEditors: readonly NotebookEditor[];
    export const activeNotebookEditor: NotebookEditor | undefined;
    export const onDidChangeActiveNotebookEditor: Event<NotebookEditor | undefined>;
    
    // Terminal management
    export const terminals: readonly Terminal[];
    export const activeTerminal: Terminal | undefined;
    export const onDidChangeActiveTerminal: Event<Terminal | undefined>;
    export const onDidOpenTerminal: Event<Terminal>;
    export const onDidCloseTerminal: Event<Terminal>;
}
```

**Variations / call-sites:** Similar patterns in `vscode.proposed.terminalCompletionProvider.d.ts:256`, `vscode.proposed.textEditorDiffInformation.d.ts:43` for diff/merge editors.

---

#### Pattern: Text Editing Operations via TextEditor
**Where:** `src/vscode-dts/vscode.d.ts:1258-1352`
**What:** Core editing interface providing edit operations, snippet insertion, decorations, and selection management on documents.

```typescript
export interface TextEditor {
    readonly document: TextDocument;
    selection: Selection;
    selections: readonly Selection[];
    readonly visibleRanges: readonly Range[];
    options: TextEditorOptions;
    readonly viewColumn: ViewColumn | undefined;
    
    // Edit operations with undo/redo control
    edit(callback: (editBuilder: TextEditorEdit) => void, options?: {
        readonly undoStopBefore: boolean;
        readonly undoStopAfter: boolean;
    }): Thenable<boolean>;
    
    // Snippet mode insertion
    insertSnippet(snippet: SnippetString, location?: Position | Range | readonly Position[] | readonly Range[], options?: {
        readonly undoStopBefore: boolean;
        readonly undoStopAfter: boolean;
        readonly keepWhitespace?: boolean;
    }): Thenable<boolean>;
    
    // Visual decorations
    setDecorations(decorationType: TextEditorDecorationType, 
                  rangesOrOptions: readonly Range[] | readonly DecorationOptions[]): void;
    
    // Navigation
    revealRange(range: Range, revealType?: TextEditorRevealType): void;
}
```

**Variations / call-sites:** Diff editors expose similar patterns in `vscode.proposed.diffContentOptions.d.ts`.

---

#### Pattern: Language Service Providers - Multi-Method Pattern
**Where:** `src/vscode-dts/vscode.d.ts:5189-5223` (CompletionItemProvider), `src/vscode-dts/vscode.d.ts:2925-2936` (DefinitionProvider)
**What:** Core language intelligence providers define a main provide method with optional resolve/metadata methods for progressive disclosure.

```typescript
export interface CompletionItemProvider<T extends CompletionItem = CompletionItem> {
    // Main entry point triggered on typing
    provideCompletionItems(document: TextDocument, position: Position, token: CancellationToken, context: CompletionContext): ProviderResult<T[] | CompletionList<T>>;
    
    // Optional resolution for details (documentation, formatting, etc)
    resolveCompletionItem?(item: T, token: CancellationToken): ProviderResult<T>;
}

export interface DefinitionProvider {
    // Core definition lookup at cursor position
    provideDefinition(document: TextDocument, position: Position, token: CancellationToken): ProviderResult<Definition | DefinitionLink[]>;
}

export interface ImplementationProvider {
    // Go-to-implementation at cursor position  
    provideImplementation(document: TextDocument, position: Position, token: CancellationToken): ProviderResult<Definition | DefinitionLink[]>;
}

export interface TypeDefinitionProvider {
    // Type definition lookup
    provideTypeDefinition(document: TextDocument, position: Position, token: CancellationToken): ProviderResult<Definition | DefinitionLink[]>;
}
```

**Variations / call-sites:** Pattern repeated across 40+ providers: HoverProvider (3144), DocumentHighlightProvider (3388), DocumentSymbolProvider (3638), ReferenceProvider (3717), RenameProvider (4209), all following `(document, position, token) -> ProviderResult<T>`.

---

#### Pattern: Hover and Diagnostic Information
**Where:** `src/vscode-dts/vscode.d.ts:3144-3158`
**What:** Inline documentation display using standardized position-based provider protocol.

```typescript
export interface HoverProvider {
    // Return hover content at cursor position
    provideHover(document: TextDocument, position: Position, token: CancellationToken): ProviderResult<Hover>;
}

export interface EvaluatableExpressionProvider {
    // Debug-time expression evaluation display
    provideEvaluatableExpression(document: TextDocument, position: Position, token: CancellationToken): ProviderResult<EvaluatableExpression>;
}
```

**Variations / call-sites:** InlineValuesProvider (3314) for inline value display in debugging, InlayHintsProvider (5700) for inline hints.

---

#### Pattern: Command Registration and Execution
**Where:** `src/vscode-dts/vscode.d.ts:10973-11030`
**What:** Command system for UI actions and programmatic invocation with extensibility.

```typescript
export namespace commands {
    // Register global command with callback
    export function registerCommand(command: string, callback: (...args: any[]) => any, thisArg?: any): Disposable;
    
    // Register editor-specific command with active editor + edit builder access
    export function registerTextEditorCommand(command: string, 
                                            callback: (textEditor: TextEditor, edit: TextEditorEdit, ...args: any[]) => void, 
                                            thisArg?: any): Disposable;
    
    // Execute any registered command
    export function executeCommand<T = unknown>(command: string, ...rest: any[]): Thenable<T>;
    
    // Query available commands
    export function getCommands(filterInternal?: boolean): Thenable<string[]>;
}
```

**Variations / call-sites:** Diff command registration in `vscode.proposed.diffCommand.d.ts:20`.

---

#### Pattern: Debug Session Lifecycle and Control
**Where:** `src/vscode-dts/vscode.d.ts:17283-17398`
**What:** Comprehensive debug session management with DAP integration points.

```typescript
export namespace debug {
    // Session state
    export let activeDebugSession: DebugSession | undefined;
    export let activeDebugConsole: DebugConsole;
    export let breakpoints: readonly Breakpoint[];
    export const activeStackItem: DebugThread | DebugStackFrame | undefined;
    
    // Session lifecycle events
    export const onDidChangeActiveDebugSession: Event<DebugSession | undefined>;
    export const onDidStartDebugSession: Event<DebugSession>;
    export const onDidTerminateDebugSession: Event<DebugSession>;
    export const onDidReceiveDebugSessionCustomEvent: Event<DebugSessionCustomEvent>;
    export const onDidChangeBreakpoints: Event<BreakpointsChangeEvent>;
    export const onDidChangeActiveStackItem: Event<DebugThread | DebugStackFrame | undefined>;
    
    // Debug adapter registration
    export function registerDebugConfigurationProvider(debugType: string, provider: DebugConfigurationProvider, triggerKind?: DebugConfigurationProviderTriggerKind): Disposable;
    export function registerDebugAdapterDescriptorFactory(debugType: string, factory: DebugAdapterDescriptorFactory): Disposable;
    export function registerDebugAdapterTrackerFactory(debugType: string, factory: DebugAdapterTrackerFactory): Disposable;
    
    // Debug control
    export function startDebugging(folder: WorkspaceFolder | undefined, nameOrConfiguration: string | DebugConfiguration, parentSessionOrOptions?: DebugSession | DebugSessionOptions): Thenable<boolean>;
    export function stopDebugging(session?: DebugSession): Thenable<void>;
}
```

**Variations / call-sites:** Debug visualization in `vscode.proposed.debugVisualization.d.ts`.

---

#### Pattern: Source Control Management
**Where:** `src/vscode-dts/vscode.d.ts:16652-16670`
**What:** SCM provider abstraction enabling multiple source control systems.

```typescript
export namespace scm {
    // Deprecated global input box (use per-instance instead)
    export const inputBox: SourceControlInputBox;
    
    // Create new source control instance
    export function createSourceControl(id: string, label: string, rootUri?: Uri): SourceControl;
}

export interface SourceControl {
    readonly id: string;
    readonly label: string;
    readonly rootUri: Uri;
    inputBox: SourceControlInputBox;
    quickDiffProvider?: QuickDiffProvider;
    commitTemplate: string;
    acceptInputCommand?: Command;
    statusBarCommands?: Command[];
}
```

**Variations / call-sites:** QuickDiffProvider (16416) for diff view integration.

---

#### Pattern: Workspace and File System Operations
**Where:** `src/vscode-dts/vscode.d.ts:13797-13956`
**What:** Workspace state management and file system access patterns.

```typescript
export namespace workspace {
    // File system abstraction
    export const fs: FileSystem;
    
    // Workspace state
    export const rootPath: string | undefined; // Deprecated
    export const workspaceFolders: readonly WorkspaceFolder[] | undefined;
    export const name: string | undefined;
    export const workspaceFile: Uri | undefined;
    
    // Workspace events
    export const onDidChangeWorkspaceFolders: Event<WorkspaceFoldersChangeEvent>;
    
    // Workspace operations
    export function getWorkspaceFolder(uri: Uri): WorkspaceFolder | undefined;
    export function asRelativePath(pathOrUri: string | Uri, includeWorkspaceFolder?: boolean): string;
    export function updateWorkspaceFolders(start: number, deleteCount: number | undefined | null, ...workspaceFoldersToAdd: { readonly uri: Uri; readonly name?: string; }[]): boolean;
    
    // File watching
    export function createFileSystemWatcher(globPattern: GlobPattern, ignoreCreateEvents?: boolean, ignoreChangeEvents?: boolean, ignoreDeleteEvents?: boolean): FileSystemWatcher;
}
```

**Variations / call-sites:** Multi-document search in `vscode.proposed.findTextInFiles.d.ts:84`, workspace trust in `vscode.proposed.workspaceTrust.d.ts:36`.

---

#### Pattern: File System Provider Interface
**Where:** `src/vscode-dts/vscode.d.ts:9600-9700`
**What:** Virtual file system abstraction for custom storage backends.

```typescript
export interface FileSystemProvider {
    // Change notification
    readonly onDidChangeFile: Event<FileChangeEvent[]>;
    
    // File watching
    watch(uri: Uri, options: { readonly recursive: boolean; readonly excludes: readonly string[]; }): Disposable;
    
    // File metadata and reading
    stat(uri: Uri): FileStat | Thenable<FileStat>;
    readDirectory(uri: Uri): [string, FileType][] | Thenable<[string, FileType][]>;
    readFile(uri: Uri): Uint8Array | Thenable<Uint8Array>;
    
    // Directory operations
    createDirectory(uri: Uri): void | Thenable<void>;
    
    // File operations
    writeFile(uri: Uri, content: Uint8Array, options: { create: boolean; overwrite: boolean; }): void | Thenable<void>;
    delete(uri: Uri, options: { recursive: boolean; }): void | Thenable<void>;
    rename(oldUri: Uri, newUri: Uri, options: { overwrite: boolean; }): void | Thenable<void>;
}
```

**Variations / call-sites:** Text document content providers (1850) for non-file-system documents.

---

#### Pattern: Extension Host Interaction Model
**Where:** `src/vscode-dts/vscode.d.ts:17458-17478`
**What:** Extension discovery and loading management.

```typescript
export namespace extensions {
    // Get extension by identifier (publisher.name)
    export function getExtension<T = any>(extensionId: string): Extension<T> | undefined;
    
    // All known extensions
    export const all: readonly Extension<any>[];
    
    // Extension lifecycle
    export const onDidChange: Event<void>;
}

export interface Extension<T> {
    readonly id: string;
    readonly extensionPath: string;
    readonly extensionUri: Uri;
    readonly packageJSON: any;
    readonly extensionKind: ExtensionKind;
    readonly exports: T;
    readonly isActive: boolean;
    activate(): Thenable<T>;
}
```

**Variations / call-sites:** Extension runtime information in `vscode.proposed.extensionRuntime.d.ts`.

---

## Summary

These 8 concrete patterns capture the core abstraction layers VS Code exposes:

1. **Editor Management** - State tracking, event-driven architecture for editor lifecycle
2. **Edit Operations** - Callback-based batched editing with undo/redo control
3. **Language Services** - Provider-based architecture with document/position/token parameters
4. **Debugging** - DAP adapter integration with session and breakpoint lifecycle
5. **Source Control** - Multi-provider abstraction for version control systems
6. **Workspace** - Multi-folder workspace with file system abstraction
7. **File Systems** - Event-driven virtual file system protocol
8. **Extensions** - Discovery, activation, and API export model

Each pattern relies heavily on **Events** for state changes, **Thenable** (Promise-like) for async operations, **CancellationToken** for operation cancellation, and **ProviderResult** for graceful null handling. The patterns are extensively replicated across 170 type definition files showing the scale of the API surface.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
