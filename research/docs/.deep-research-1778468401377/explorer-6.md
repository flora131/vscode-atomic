# Partition 6 of 80 — Findings

## Scope
`src/vscode-dts/` (174 files, 33,614 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# VS Code API Surface (vscode-dts/) — File Locator

## Overview
The `src/vscode-dts/` directory (174 files, ~33.6K LOC) contains the complete TypeScript Declaration definition of VS Code's public extension API. This forms the **contract boundary** that any Tauri/Rust port must preserve or translate into a Rust-side ABI.

### Key Statistics
- **1 core file**: `vscode.d.ts` — the main API namespace
- **173 proposed files**: `vscode.proposed.*.d.ts` — experimental/evolving API additions
- **710+ exported interfaces** — types and contracts for plugins
- **19 exported namespaces** — top-level API areas (window, workspace, commands, debug, scm, terminal, languages, tasks, chat, lm, tests, extensions, etc.)

### Research Relevance
A port to Tauri/Rust would need to:
1. **Preserve or translate** all interfaces in this namespace to Rust structs/traits
2. **Reimplement all namespaced APIs** (window.showInputBox, workspace.openTextDocument, etc.) in a Rust-compatible protocol
3. **Define ABI boundaries** for extension/plugin communication across the Electron→Tauri transition

---

## Implementation

### Core API Definition
- `src/vscode-dts/vscode.d.ts` — Main extension API module; exports ~200+ interfaces, 19 namespaces, enums for editors, text operations, UI controls, workspace operations

### Text Editing & Editor Operations
- `src/vscode-dts/vscode.proposed.customEditorDiffs.d.ts` — Custom editor diff support
- `src/vscode-dts/vscode.proposed.customEditorMove.d.ts` — Custom editor move operations
- `src/vscode-dts/vscode.proposed.customEditorPriority.d.ts` — Custom editor priority/selection
- `src/vscode-dts/vscode.proposed.editorHoverVerbosityLevel.d.ts` — Hover information verbosity control
- `src/vscode-dts/vscode.proposed.editorInsets.d.ts` — Editor inset rendering
- `src/vscode-dts/vscode.proposed.textDocumentChangeReason.d.ts` — Change reason tracking
- `src/vscode-dts/vscode.proposed.textEditorDiffInformation.d.ts` — Diff state in text editors

### Search & Find
- `src/vscode-dts/vscode.proposed.fileSearchProvider.d.ts` — File search provider interface
- `src/vscode-dts/vscode.proposed.fileSearchProvider2.d.ts` — File search provider v2
- `src/vscode-dts/vscode.proposed.findFiles2.d.ts` — File finder with advanced options
- `src/vscode-dts/vscode.proposed.findTextInFiles.d.ts` — Text search across files
- `src/vscode-dts/vscode.proposed.findTextInFiles2.d.ts` — Text search v2
- `src/vscode-dts/vscode.proposed.aiTextSearchProvider.d.ts` — AI-assisted text search
- `src/vscode-dts/vscode.proposed.textSearchProvider.d.ts` — Text search provider
- `src/vscode-dts/vscode.proposed.textSearchProvider2.d.ts` — Text search provider v2
- `src/vscode-dts/vscode.proposed.textSearchComplete2.d.ts` — Text search completion

### Language Features & Intelligence
- `src/vscode-dts/vscode.proposed.inlineCompletionsAdditions.d.ts` — Inline completions (LSP)
- `src/vscode-dts/vscode.proposed.newSymbolNamesProvider.d.ts` — Symbol renaming provider
- `src/vscode-dts/vscode.proposed.tokenInformation.d.ts` — Token semantic analysis
- `src/vscode-dts/vscode.proposed.multiDocumentHighlightProvider.d.ts` — Multi-document highlights
- `src/vscode-dts/vscode.proposed.mappedEditsProvider.d.ts` — Source map-aware edits
- `src/vscode-dts/vscode.proposed.languageStatusText.d.ts` — Language server status display

### Terminal & Shell
- `src/vscode-dts/vscode.proposed.terminalCompletionProvider.d.ts` — Terminal command completion
- `src/vscode-dts/vscode.proposed.terminalDataWriteEvent.d.ts` — Terminal data write events
- `src/vscode-dts/vscode.proposed.terminalDimensions.d.ts` — Terminal dimensions/resizing
- `src/vscode-dts/vscode.proposed.terminalExecuteCommandEvent.d.ts` — Terminal command execution
- `src/vscode-dts/vscode.proposed.terminalQuickFixProvider.d.ts` — Terminal error quick fixes
- `src/vscode-dts/vscode.proposed.terminalSelection.d.ts` — Terminal text selection
- `src/vscode-dts/vscode.proposed.terminalShellEnv.d.ts` — Shell environment variables
- `src/vscode-dts/vscode.proposed.terminalTitle.d.ts` — Terminal title customization
- `src/vscode-dts/vscode.proposed.taskExecutionTerminal.d.ts` — Task execution terminal binding

### Source Control Management
- `src/vscode-dts/vscode.proposed.scmActionButton.d.ts` — SCM action button in UI
- `src/vscode-dts/vscode.proposed.scmArtifactProvider.d.ts` — SCM artifact metadata
- `src/vscode-dts/vscode.proposed.scmHistoryProvider.d.ts` — SCM commit/history provider
- `src/vscode-dts/vscode.proposed.scmMultiDiffEditor.d.ts` — Multi-diff view for SCM
- `src/vscode-dts/vscode.proposed.scmProviderOptions.d.ts` — SCM provider configuration
- `src/vscode-dts/vscode.proposed.scmSelectedProvider.d.ts` — Active SCM provider selection
- `src/vscode-dts/vscode.proposed.scmTextDocument.d.ts` — SCM-aware text documents
- `src/vscode-dts/vscode.proposed.scmValidation.d.ts` — SCM validation/status

### Debugging
- `src/vscode-dts/vscode.proposed.debugVisualization.d.ts` — Debug visualization (data display)
- `src/vscode-dts/vscode.proposed.chatDebug.d.ts` — Chat integration with debugger

### Tasks & Problem Matching
- `src/vscode-dts/vscode.proposed.taskExecutionTerminal.d.ts` — Task terminal integration
- `src/vscode-dts/vscode.proposed.taskPresentationGroup.d.ts` — Task presentation grouping
- `src/vscode-dts/vscode.proposed.taskProblemMatcherStatus.d.ts` — Problem matcher status
- `src/vscode-dts/vscode.proposed.taskRunOptions.d.ts` — Task execution options

### Notebook Support
- `src/vscode-dts/vscode.proposed.notebookCellExecution.d.ts` — Notebook cell execution
- `src/vscode-dts/vscode.proposed.notebookControllerAffinityHidden.d.ts` — Notebook kernel affinity
- `src/vscode-dts/vscode.proposed.notebookDeprecated.d.ts` — Deprecated notebook APIs
- `src/vscode-dts/vscode.proposed.notebookExecution.d.ts` — Notebook-wide execution
- `src/vscode-dts/vscode.proposed.notebookKernelSource.d.ts` — Kernel source provider
- `src/vscode-dts/vscode.proposed.notebookLiveShare.d.ts` — Live Share notebook extensions
- `src/vscode-dts/vscode.proposed.notebookMessaging.d.ts` — Notebook communication
- `src/vscode-dts/vscode.proposed.notebookMime.d.ts` — MIME type handling
- `src/vscode-dts/vscode.proposed.notebookReplDocument.d.ts` — REPL notebook support
- `src/vscode-dts/vscode.proposed.notebookVariableProvider.d.ts` — Debug variable provider

### AI & Language Models
- `src/vscode-dts/vscode.proposed.languageModelCapabilities.d.ts` — LM capability querying
- `src/vscode-dts/vscode.proposed.languageModelPricing.d.ts` — LM pricing/token tracking
- `src/vscode-dts/vscode.proposed.languageModelProxy.d.ts` — Proxy for language models
- `src/vscode-dts/vscode.proposed.languageModelSystem.d.ts` — Core LM system API
- `src/vscode-dts/vscode.proposed.languageModelThinkingPart.d.ts` — LM reasoning/thinking
- `src/vscode-dts/vscode.proposed.languageModelToolResultAudience.d.ts` — Tool result visibility
- `src/vscode-dts/vscode.proposed.languageModelToolSupportsModel.d.ts` — Tool model selection
- `src/vscode-dts/vscode.proposed.embeddings.d.ts` — Vector embeddings
- `src/vscode-dts/vscode.proposed.mcpServerDefinitions.d.ts` — Model Context Protocol servers
- `src/vscode-dts/vscode.proposed.mcpToolDefinitions.d.ts` — MCP tool definitions

### Chat & Interactive Features
- `src/vscode-dts/vscode.proposed.chatContextProvider.d.ts` — Chat context injection
- `src/vscode-dts/vscode.proposed.chatHooks.d.ts` — Chat lifecycle hooks
- `src/vscode-dts/vscode.proposed.chatInputNotification.d.ts` — Chat input notifications
- `src/vscode-dts/vscode.proposed.chatOutputRenderer.d.ts` — Custom chat rendering
- `src/vscode-dts/vscode.proposed.chatParticipantAdditions.d.ts` — Chat participant extensions
- `src/vscode-dts/vscode.proposed.chatParticipantPrivate.d.ts` — Private chat participant API
- `src/vscode-dts/vscode.proposed.chatPromptFiles.d.ts` — Chat prompt file references
- `src/vscode-dts/vscode.proposed.chatProvider.d.ts` — Chat provider interface
- `src/vscode-dts/vscode.proposed.chatReferenceBinaryData.d.ts` — Binary data in chat
- `src/vscode-dts/vscode.proposed.chatReferenceDiagnostic.d.ts` — Diagnostic references in chat
- `src/vscode-dts/vscode.proposed.chatSessionCustomizationProvider.d.ts` — Session customization
- `src/vscode-dts/vscode.proposed.chatSessionsProvider.d.ts` — Chat session management
- `src/vscode-dts/vscode.proposed.chatStatusItem.d.ts` — Chat status bar item
- `src/vscode-dts/vscode.proposed.chatTab.d.ts` — Chat in tab/sidebar
- `src/vscode-dts/vscode.proposed.defaultChatParticipant.d.ts` — Default chat participant
- `src/vscode-dts/vscode.proposed.interactive.d.ts` — Interactive window API

### UI & Window Operations
- `src/vscode-dts/vscode.proposed.browser.d.ts` — Browser-specific APIs (webview)
- `src/vscode-dts/vscode.proposed.contribChatEditorInlineGutterMenu.d.ts` — Chat gutter menu
- `src/vscode-dts/vscode.proposed.contribCommentEditorActionsMenu.d.ts` — Comment menu
- `src/vscode-dts/vscode.proposed.contribCommentPeekContext.d.ts` — Comment peek UI
- `src/vscode-dts/vscode.proposed.contribCommentsViewThreadMenus.d.ts` — Comments view menu
- `src/vscode-dts/vscode.proposed.contribCommentThreadAdditionalMenu.d.ts` — Comment thread menu
- `src/vscode-dts/vscode.proposed.contribDiffEditorGutterToolBarMenus.d.ts` — Diff gutter toolbar
- `src/vscode-dts/vscode.proposed.contribEditorContentMenu.d.ts` — Editor context menu
- `src/vscode-dts/vscode.proposed.contribLabelFormatterWorkspaceTooltip.d.ts` — Tooltip formatter
- `src/vscode-dts/vscode.proposed.contribMenuBarHome.d.ts` — Home menu bar
- `src/vscode-dts/vscode.proposed.contribMergeEditorMenus.d.ts` — Merge editor menu
- `src/vscode-dts/vscode.proposed.contribMultiDiffEditorMenus.d.ts` — Multi-diff menu
- `src/vscode-dts/vscode.proposed.contribShareMenu.d.ts` — Share action menu
- `src/vscode-dts/vscode.proposed.contribSourceControlHistoryItemMenu.d.ts` — History context menu
- `src/vscode-dts/vscode.proposed.contribSourceControlHistoryTitleMenu.d.ts` — History title menu
- `src/vscode-dts/vscode.proposed.contribSourceControlInputBoxMenu.d.ts` — SCM input menu
- `src/vscode-dts/vscode.proposed.contribSourceControlTitleMenu.d.ts` — SCM title menu
- `src/vscode-dts/vscode.proposed.contribStatusBarItems.d.ts` — Status bar items
- `src/vscode-dts/vscode.proposed.codiconDecoration.d.ts` — Codicon (icon) rendering
- `src/vscode-dts/vscode.proposed.quickDiffProvider.d.ts` — Quick diff provider
- `src/vscode-dts/vscode.proposed.quickPickItemTooltip.d.ts` — Quick pick tooltips
- `src/vscode-dts/vscode.proposed.quickPickSortByLabel.d.ts` — Quick pick sorting
- `src/vscode-dts/vscode.proposed.statusBarItemTooltip.d.ts` — Status bar tooltips
- `src/vscode-dts/vscode.proposed.valueSelectionInQuickPick.d.ts` — Quick pick selection

### Authentication & Security
- `src/vscode-dts/vscode.proposed.authenticationChallenges.d.ts` — Auth challenge handling
- `src/vscode-dts/vscode.proposed.authIssuers.d.ts` — Auth issuer detection
- `src/vscode-dts/vscode.proposed.authLearnMore.d.ts` — Auth help/learning links
- `src/vscode-dts/vscode.proposed.authProviderSpecific.d.ts` — Provider-specific auth
- `src/vscode-dts/vscode.proposed.authSession.d.ts` — Auth session management
- `src/vscode-dts/vscode.proposed.workspaceTrust.d.ts` — Workspace trust model

### Workspace & File Management
- `src/vscode-dts/vscode.proposed.agentSessionsWorkspace.d.ts` — Agent workspace isolation
- `src/vscode-dts/vscode.proposed.canonicalUriProvider.d.ts` — URI normalization
- `src/vscode-dts/vscode.proposed.documentDiff.d.ts` — Document diff provider
- `src/vscode-dts/vscode.proposed.editSessionIdentityProvider.d.ts` — Edit session tracking
- `src/vscode-dts/vscode.proposed.fsChunks.d.ts` — Chunked file operations
- `src/vscode-dts/vscode.proposed.profileContentHandlers.d.ts` — Profile serialization
- `src/vscode-dts/vscode.proposed.resolvers.d.ts` — Workspace/folder resolvers
- `src/vscode-dts/vscode.proposed.timeline.d.ts` — File timeline provider
- `src/vscode-dts/vscode.proposed.tunnelFactory.d.ts` — Port tunnel provider
- `src/vscode-dts/vscode.proposed.tunnels.d.ts` — Port forwarding API

### Testing
- `src/vscode-dts/vscode.proposed.testObserver.d.ts` — Test results observer
- `src/vscode-dts/vscode.proposed.testRelatedCode.d.ts` — Test-code relationships

### Comments & Collaboration
- `src/vscode-dts/vscode.proposed.activeComment.d.ts` — Active comment state
- `src/vscode-dts/vscode.proposed.commentingRangeHint.d.ts` — Commenting range UI
- `src/vscode-dts/vscode.proposed.commentReactor.d.ts` — Comment reaction system
- `src/vscode-dts/vscode.proposed.commentReveal.d.ts` — Comment reveal/navigation
- `src/vscode-dts/vscode.proposed.commentThreadApplicability.d.ts` — Comment thread filtering
- `src/vscode-dts/vscode.proposed.commentsDraftState.d.ts` — Draft comment tracking

### Configuration & Extensions
- `src/vscode-dts/vscode.proposed.agentsWindowConfiguration.d.ts` — Agent configuration
- `src/vscode-dts/vscode.proposed.contribAccessibilityHelpContent.d.ts` — Accessibility help
- `src/vscode-dts/vscode.proposed.contribLanguageModelToolSets.d.ts` — Tool grouping
- `src/vscode-dts/vscode.proposed.contribRemoteHelp.d.ts` — Remote help
- `src/vscode-dts/vscode.proposed.contribViewContainerTitle.d.ts` — View container titles
- `src/vscode-dts/vscode.proposed.contribViewsRemote.d.ts` — Remote view trees
- `src/vscode-dts/vscode.proposed.contribViewsWelcome.d.ts` — Welcome view contributions
- `src/vscode-dts/vscode.proposed.extensionAffinity.d.ts` — Extension compatibility
- `src/vscode-dts/vscode.proposed.extensionRuntime.d.ts` — Extension runtime info
- `src/vscode-dts/vscode.proposed.extensionsAny.d.ts` — Extension metadata
- `src/vscode-dts/vscode.proposed.externalUriOpener.d.ts` — External URI handling
- `src/vscode-dts/vscode.proposed.profileContentHandlers.d.ts` — Profile management

### Environment & System
- `src/vscode-dts/vscode.proposed.devDeviceId.d.ts` — Device identification
- `src/vscode-dts/vscode.proposed.environmentPower.d.ts` — Power state API
- `src/vscode-dts/vscode.proposed.envIsConnectionMetered.d.ts` — Connection metering detection
- `src/vscode-dts/vscode.proposed.nativeWindowHandle.d.ts` — Native window integration
- `src/vscode-dts/vscode.proposed.speech.d.ts` — Speech recognition/synthesis
- `src/vscode-dts/vscode.proposed.telemetry.d.ts` — Telemetry collection

### Remote & Agent Features
- `src/vscode-dts/vscode.proposed.remoteCodingAgents.d.ts` — Remote agent APIs
- `src/vscode-dts/vscode.proposed.shareProvider.d.ts` — Code sharing provider

### Data & Utilities
- `src/vscode-dts/vscode.proposed.css.d.ts` — CSS utilities
- `src/vscode-dts/vscode.proposed.dataChannels.d.ts` — Data channel abstraction
- `src/vscode-dts/vscode.proposed.diffContentOptions.d.ts` — Diff rendering options
- `src/vscode-dts/vscode.proposed.documentFiltersExclusive.d.ts` — Document filter logic
- `src/vscode-dts/vscode.proposed.ipc.d.ts` — IPC message passing
- `src/vscode-dts/vscode.proposed.tabInputMultiDiff.d.ts` — Multi-diff tab type
- `src/vscode-dts/vscode.proposed.tabInputTextMerge.d.ts` — Merge editor tab type
- `src/vscode-dts/vscode.proposed.treeItemMarkdownLabel.d.ts` — Tree item markdown
- `src/vscode-dts/vscode.proposed.treeViewActiveItem.d.ts` — Tree view selection
- `src/vscode-dts/vscode.proposed.treeViewMarkdownMessage.d.ts` — Tree view messages
- `src/vscode-dts/vscode.proposed.treeViewReveal.d.ts` — Tree item reveal
- `src/vscode-dts/vscode.proposed.toolInvocationApproveCombination.d.ts` — Tool approval UI
- `src/vscode-dts/vscode.proposed.toolProgress.d.ts` — Tool execution progress

### AI Settings & Search
- `src/vscode-dts/vscode.proposed.aiRelatedInformation.d.ts` — AI result context
- `src/vscode-dts/vscode.proposed.aiSettingsSearch.d.ts` — AI settings search
- `src/vscode-dts/vscode.proposed.codeActionAI.d.ts` — AI code actions
- `src/vscode-dts/vscode.proposed.contribEditSessions.d.ts` — Edit session sync

### Miscellaneous
- `src/vscode-dts/vscode.proposed.codeActionRanges.d.ts` — Code action range info
- `src/vscode-dts/vscode.proposed.contribDebugCreateConfiguration.d.ts` — Debug config UI
- `src/vscode-dts/vscode.proposed.contribNotebookStaticPreloads.d.ts` — Notebook preloading
- `src/vscode-dts/vscode.proposed.contribSourceControlArtifactGroupMenu.d.ts` — Artifact group menu
- `src/vscode-dts/vscode.proposed.contribSourceControlArtifactMenu.d.ts` — Artifact menu
- `src/vscode-dts/vscode.proposed.contribViewContainerTitle.d.ts` — View container naming
- `src/vscode-dts/vscode.proposed.diffCommand.d.ts` — Diff command protocol
- `src/vscode-dts/vscode.proposed.markdownAlertSyntax.d.ts` — Markdown alert styling

---

## Notable Clusters

### Core API Container
- `src/vscode-dts/vscode.d.ts` — 1 file, contains all primary type definitions and the `declare module 'vscode'` block. This is the entry point for extension developers.

### Namespace Distribution
- **19 namespaces** across 89 files: `window`, `workspace`, `commands`, `debug`, `scm`, `languages`, `tasks`, `chat`, `lm`, `tests`, `extensions`, `ai`, `authentication`, `env`, `interactive`, `notebooks`, `power`, `speech` — each encapsulates a major IDE feature area.

### Feature-Specific Families
- **Terminal subsystem**: 9 files focused on terminal operations (completion, I/O, dimensions, shell env, quick fixes, title, etc.)
- **Language Models / AI**: 10 files for LM capabilities, pricing, tool invocation, embeddings, MCP integration
- **Chat & Interactive**: 15 files for chat participants, sessions, context, rendering, debugging
- **Source Control**: 8 files for SCM providers, diff, history, artifacts, validation
- **Search Providers**: 9 files for file search, text search, AI text search across versions
- **Notebook**: 11 files for kernel, execution, messaging, MIME types, REPL
- **UI Menus & Contributions**: 22 files for context menus, status bar, toolbars, quick pick, views

### Version Progression
- Many features have v1 and v2 API versions (e.g., `fileSearchProvider.d.ts` and `fileSearchProvider2.d.ts`, `textSearchProvider.d.ts` and `textSearchProvider2.d.ts`) reflecting iterative refinement.

---

## Key Interfaces & Type Families (Sample)

The 710+ exported interfaces include:

### Text Model
- `TextDocument`, `TextLine`, `TextEditor`, `TextEditorEdit`, `TextChange`, `TextDocumentChangeEvent`

### Position & Range
- `Position`, `Range`, `Selection`, `TextDocumentContentProvider`

### Languages & IntelliSense
- `HoverProvider`, `CompletionItemProvider`, `SignatureHelpProvider`, `DocumentSymbolProvider`, `ReferenceProvider`, `DefinitionProvider`, `ImplementationProvider`, `TypeDefinitionProvider`, `DocumentHighlightProvider`, `WorkspaceSymbolProvider`, `RenameProvider`, `CodeActionProvider`, `CodeLensProvider`, `FormattingEditProvider`, `FoldingRangeProvider`, `InlineValuesProvider`, `InlineCompletionItemProvider`, `DocumentSemanticTokensProvider`

### UI & Window
- `TextDocumentShowOptions`, `WebviewPanel`, `WebviewView`, `QuickPickItem`, `MessageItem`, `InputBoxOptions`, `OpenDialogOptions`, `SaveDialogOptions`, `StatusBarItem`, `ThemableDecorationRenderOptions`, `DecorationOptions`

### Debug
- `DebugSession`, `DebugConfiguration`, `DebugAdapterDescriptorFactory`, `DebugAdapterNamedPipeServer`, `DebugAdapterExecutable`, `Breakpoint`, `SourceBreakpoint`, `FunctionBreakpoint`, `LogPoint`

### SCM
- `SourceControlResourceState`, `SourceControlResourceDecorations`, `SourceControl`, `SourceControlResourceGroup`, `SourceControlProvider`

### Tasks
- `Task`, `TaskDefinition`, `TaskProvider`, `TaskExecution`, `TaskProblemMatcher`, `CustomExecution`, `Pseudoterminal`

### Terminal
- `Terminal`, `TerminalOptions`, `ExtensionTerminalOptions`, `Pseudoterminal`, `TerminalLink`, `TerminalLinkProvider`

---

## Summary

This scope (174 TypeScript declaration files) represents the **complete public extension API surface** of VS Code. Any Tauri/Rust port would require:

1. **Direct translation** of all 710+ interfaces into Rust structs/traits/enums
2. **Protocol design** for IPC communication with plugins (replacing the current Node.js/Electron extension host)
3. **Namespace reorganization** in Rust (e.g., `vscode::window`, `vscode::workspace`, etc.)
4. **ABI stability guarantees** for version-to-version compatibility, as extensions depend on this contract

The breadth of features—from text editing, LSP integration, debugging, terminal I/O, source control, notebooks, AI/chat, to authentication and remote tunneling—illustrates the complexity of a full port.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `src/vscode-dts/vscode.d.ts` — Main stable extension API (21,235 lines)
- `src/vscode-dts/vscode.proposed.debugVisualization.d.ts` — Proposed debug visualization provider API
- `src/vscode-dts/vscode.proposed.languageModelSystem.d.ts` — Proposed LLM system message role enum
- `src/vscode-dts/vscode.proposed.scmHistoryProvider.d.ts` — Proposed SCM history / git graph API
- `src/vscode-dts/vscode.proposed.chatProvider.d.ts` — Proposed language-model chat-provider registration API
- `src/vscode-dts/vscode.proposed.terminalCompletionProvider.d.ts` — Proposed terminal completion provider API
- `src/vscode-dts/vscode.proposed.fileSearchProvider.d.ts` — Proposed file search provider API
- `src/vscode-dts/vscode.proposed.resolvers.d.ts` — Proposed remote-authority resolver API (tunnels, exec servers, remote FS)
- `src/vscode-dts/vscode.proposed.notebookKernelSource.d.ts` — Proposed notebook kernel source action provider
- `src/vscode-dts/vscode.proposed.workspaceTrust.d.ts` — Proposed workspace/resource trust API

---

### Per-File Notes

#### `src/vscode-dts/vscode.d.ts`

- **Role:** The single canonical public contract for all VS Code extensions; every top-level capability that must be preserved (or replaced by Rust ABI) in a Tauri port lives here.

- **Key symbols:**

  **Primitive model types**
  - `TextDocument` (`vscode.d.ts:88`) — immutable document handle: `uri`, `languageId`, `version`, `getText()`, `lineAt()`, `offsetAt()`, `positionAt()`.  All LSP interactions pass through this type.
  - `TextEditor` (`vscode.d.ts:1258`) — bidirectional editor handle: `edit(callback)` at line 1304 takes an `TextEditorEdit` builder (replace/insert/delete/setEndOfLine); `insertSnippet()` at line 1326; `setDecorations()` at line 1352.
  - `Position` / `Range` / `Selection` (`vscode.d.ts:269`, 408, 518) — immutable value objects used ubiquitously as coordinates in documents and the editor.
  - `Uri` (`vscode.d.ts:1439`) — scheme-authority-path-query-fragment abstraction; static factories `Uri.file()` (line 1479) and `Uri.parse()` (line 1454).

  **`namespace window` (`vscode.d.ts:11069`)**
  - State properties: `activeTextEditor` (11081), `visibleTextEditors` (11086), `terminals` (11161), `activeTerminal` (11167), `state: WindowState` (11214).
  - Event bus: `onDidChangeActiveTextEditor` (11093), `onDidChangeTextEditorSelection` (11104), `onDidOpenTerminal` (11180), `onDidCloseTerminal` (11185), `onDidChangeTerminalShellIntegration` (11195), `onDidStartTerminalShellExecution` (11202), `onDidEndTerminalShellExecution` (11209).
  - Factories: `showTextDocument()` (11233), `createTextEditorDecorationType()` (11272), `showQuickPick()` (11418), `showInputBox()` (11488), `createQuickPick()` (11499), `createInputBox()` (11510), `createOutputChannel()` (11523), `createWebviewPanel()` (11544), `createStatusBarItem()` (11638), `createTerminal()` (11662).

  **`namespace workspace` (`vscode.d.ts:13799`)**
  - `fs: FileSystem` (13807) — virtual file system interface.
  - `workspaceFolders: readonly WorkspaceFolder[] | undefined` (13827).
  - `createFileSystemWatcher()` (14076) — file-change event subscription.
  - `findFiles()` (14095), `openTextDocument()` (14172), `applyEdit()` (14146).
  - `textDocuments: readonly TextDocument[]` (14151).

  **`namespace languages` (`vscode.d.ts:14724`)**
  - Provider registration surface: `registerCompletionItemProvider()` (14849), `registerInlineCompletionItemProvider()` (14862), `registerCodeActionsProvider()` (14876), `registerCodeLensProvider()` (14889), `registerDefinitionProvider()` (14902), `registerImplementationProvider()` (14915), `registerTypeDefinitionProvider()` (14928), `registerHoverProvider()` (14954), `registerDocumentHighlightProvider()` (14994), `registerDocumentSymbolProvider()` (15008), `registerWorkspaceSymbolProvider()` (15020), `registerReferenceProvider()` (15033), `registerRenameProvider()` (15046), `registerDocumentSemanticTokensProvider()` (15059), `registerDocumentRangeSemanticTokensProvider()` (15078), `registerDocumentFormattingEditProvider()` (15091), `registerDocumentRangeFormattingEditProvider()` (15108), `registerOnTypeFormattingEditProvider()` (15123), `registerEvaluatableExpressionProvider()` (14966), `registerInlineValuesProvider()` (14981).
  - `getDiagnostics()` (14804/14811), `createDiagnosticCollection()` (14819).

  **`namespace debug` (`vscode.d.ts:17285`)**
  - State: `activeDebugSession: DebugSession | undefined` (17292), `breakpoints: readonly Breakpoint[]` (17303), `activeStackItem: DebugThread | DebugStackFrame | undefined` (17338).
  - Events: `onDidChangeActiveDebugSession` (17310), `onDidStartDebugSession` (17315), `onDidTerminateDebugSession` (17325), `onDidChangeBreakpoints` (17330), `onDidChangeActiveStackItem` (17343).
  - Actions: `registerDebugConfigurationProvider()` (17359), `registerDebugAdapterDescriptorFactory()` (17370), `startDebugging()` (17392), `stopDebugging()` (17400), `addBreakpoints()` (17406), `asDebugSourceUri()` (17425).
  - DAP bridge types: `DebugAdapterExecutable` (16845), `DebugAdapterServer` (16896), `DebugAdapterNamedPipeServer` (16917), `DebugAdapter` interface (16932), `DebugSession.customRequest()` (16764).

  **`namespace scm` (`vscode.d.ts:16654`)**
  - `createSourceControl(id, label, rootUri?)` (16672) — factory for `SourceControl` objects that expose resource groups, commit input, status bar commands, and (proposed) `historyProvider`.

  **`namespace tasks` (`vscode.d.ts:9347`)**
  - `registerTaskProvider()` (9356), `fetchTasks()` (9366), `executeTask()` (9379).
  - Events: `onDidStartTask` (9389), `onDidEndTask` (9394), `onDidStartTaskProcess` (9401), `onDidEndTaskProcess` (9408).

  **`namespace env` (`vscode.d.ts:10739`)**
  - `appName`, `appRoot`, `appHost`, `uriScheme`, `language`, `clipboard`, `machineId`, `shell`, `uiKind: UIKind` (10744–10854).
  - `openExternal()` (10869), `asExternalUri()` (10924) — URI resolution / external link handling critical for both local and remote scenarios.
  - `remoteName: string | undefined` (10840).

  **`namespace lm` (`vscode.d.ts:20734`)**
  - `selectChatModels()` (20766), `registerTool()` (20774), `invokeTool()` (20808), `registerLanguageModelChatProvider()` (20847), `registerMcpServerDefinitionProvider()` (20838).

  **`namespace chat` (`vscode.d.ts:20113`)**
  - `createChatParticipant()` (20121) — entry point for Copilot-style chat integration.

  **`FileSystemProvider` interface** (referenced throughout workspace and `languages` namespaces) — defines `stat`, `readDirectory`, `readFile`, `writeFile`, `rename`, `delete`, `createDirectory` as `Thenable<...>` calls.

- **Control flow:**
  - Extensions activate (`activate(context: ExtensionContext)`) and receive a context with `subscriptions: Disposable[]`. All registrations return `Disposable`s pushed into that array.
  - Provider registrations add handlers into per-language / per-type registries inside the editor host; events propagate back to extensions via `Event<T>` (EventEmitter pattern).
  - Text edits flow: extension calls `TextEditor.edit(builder => ...)` → builder collects operations → returned `Thenable<boolean>` resolves after the edit transaction is applied by the editor core.
  - Debug flow: `debug.startDebugging()` → resolves `DebugConfiguration` via chain of `DebugConfigurationProvider.resolveDebugConfiguration()` calls → spawns `DebugAdapter` (executable, socket, or named pipe) → DAP messages exchanged via `DebugAdapter.handleMessage()` / `onDidSendMessage`.

- **Data flow:**
  - Documents enter as `Uri` → `workspace.openTextDocument()` returns `TextDocument` → `window.showTextDocument()` returns `TextEditor`.
  - Language intelligence: `DocumentSelector` filters providers → provider returns `ProviderResult<CompletionList | Hover | Location[] | …>` → editor core consumes and renders.
  - Filesystem: reads/writes as `Uint8Array` through `FileSystemProvider`; path coordinates expressed as `Uri`.

- **Dependencies:** No imports; this is a `declare module 'vscode'` ambient declaration consumed by extension authors and implemented by `src/vs/workbench/` internals.

---

#### `src/vscode-dts/vscode.proposed.debugVisualization.d.ts`

- **Role:** Extends `namespace debug` with a custom variable-visualization provider API, revealing how debugging state (DAP variable objects) flows to UI components.

- **Key symbols:**
  - `debug.registerDebugVisualizationProvider<T>(id, provider)` (line 15) — registers provider against a `package.json` contribution point `debugVisualizers`.
  - `debug.registerDebugVisualizationTreeProvider<T>(id, provider)` (line 25) — registers a tree used as an in-panel or hover visualization.
  - `DebugVisualizationContext` (line 141) — carries the raw DAP `variable` object, `containerId`, `frameId`, `threadId`, and `session: DebugSession` reference. This is the ABI boundary between the DAP wire format and extension-land.
  - `DebugVisualizationProvider.provideDebugVisualization(context, token)` (line 128) — called every time a session stops; returns `DebugVisualization[]`.
  - `DebugVisualization.visualization` (line 110) — may be a `Command` or `{ treeId: string }` reference.

- **Control flow:** Session pauses → editor iterates registered providers whose `when` clause matches → `provideDebugVisualization()` called → result shown in UI; user picks visualizer → `resolveDebugVisualization()` called → optional `TreeView` returned.

- **Data flow:** Raw DAP JSON (`variable: any` at line 146) in, typed `DebugVisualization[]` out. No document or filesystem involvement.

- **Dependencies:** Uses `DebugSession`, `TreeItemCollapsibleState`, `Command`, `Uri`, `ThemeIcon`, `CancellationToken`, `ProviderResult` from the stable API.

---

#### `src/vscode-dts/vscode.proposed.scmHistoryProvider.d.ts`

- **Role:** Augments `SourceControl` with a `historyProvider` property exposing a git-log–style history graph API, defining the full data contract for SCM history views.

- **Key symbols:**
  - `SourceControl.historyProvider?: SourceControlHistoryProvider` (line 10).
  - `SourceControlHistoryProvider` (line 13): `currentHistoryItemRef` / `currentHistoryItemRemoteRef` / `currentHistoryItemBaseRef` — branch/commit refs; `onDidChangeCurrentHistoryItemRefs`, `onDidChangeHistoryItemRefs` events.
  - `provideHistoryItems(options, token)` (line 30) — paged commit log; `options` carries `skip`, `limit`, `historyItemRefs`, `filterText`.
  - `provideHistoryItemChanges(historyItemId, historyItemParentId, token)` (line 31) — diff for a commit.
  - `resolveHistoryItemChatContext` / `resolveHistoryItemChangeRangeChatContext` (lines 34–35) — AI-specific: returns plain-text context strings for chat participants.
  - `SourceControlHistoryItem` (line 52): `id`, `parentIds`, `subject`, `author`, `timestamp`, `statistics`, `references`.

- **Data flow:** `scm.createSourceControl()` → assign `.historyProvider` → editor calls `provideHistoryItems()` → `SourceControlHistoryItem[]` rendered in Source Control History view; diff requests call `provideHistoryItemChanges()` → `SourceControlHistoryItemChange[]` (uri triples: current/original/modified).

- **Dependencies:** `Uri`, `Event`, `CancellationToken`, `ProviderResult`, `IconPath`, `MarkdownString` from stable API.

---

#### `src/vscode-dts/vscode.proposed.chatProvider.d.ts`

- **Role:** Defines the provider-side contract for registering a language model with the editor's `lm.registerLanguageModelChatProvider()` entry point, including capability negotiation, streaming responses, and per-model configuration schemas.

- **Key symbols:**
  - `LanguageModelChatProvider<T>` (line 134): `provideLanguageModelChatInformation(options, token)` → `T[]`; `provideLanguageModelChatResponse(model, messages, options, progress, token)` → `Thenable<void>` (pushes `LanguageModelResponsePart2` via `progress`).
  - `LanguageModelChatInformation` (line 34): `requiresAuthorization`, `multiplierNumeric`, `isDefault`, `isUserSelectable`, `category`, `configurationSchema`.
  - `LanguageModelChatCapabilities.editTools: string[]` (line 96) — hints which edit tools (`find-replace`, `apply-patch`, `code-rewrite`, etc.) the model prefers; editor selects the appropriate edit flow.
  - `LanguageModelConfigurationSchema` (line 117) — JSON Schema fragment with `properties` and `enumItemLabels` for the model settings UI.
  - `ProvideLanguageModelChatResponseOptions.requestInitiator` (line 19) — identifies calling extension.

- **Data flow:** `lm.registerLanguageModelChatProvider(vendor, provider)` → provider's `provideLanguageModelChatInformation()` called to enumerate models → user selects model → `provideLanguageModelChatResponse()` streams `LanguageModelResponsePart2` (text, tool calls, thinking parts, data parts) via `Progress`.

- **Dependencies:** `LanguageModelChatRequestMessage`, `LanguageModelResponsePart`, `LanguageModelDataPart`, `LanguageModelThinkingPart`, `Progress`, `CancellationToken`, `ThemeIcon`, `ChatLocation`, `ChatRequest` from stable and other proposed APIs.

---

#### `src/vscode-dts/vscode.proposed.terminalCompletionProvider.d.ts`

- **Role:** Adds shell-completion capability to the terminal via `window.registerTerminalCompletionProvider()`, defining the data types for command-line aware completions.

- **Key symbols:**
  - `window.registerTerminalCompletionProvider<T>(provider, ...triggerCharacters)` (line 283) — added to the stable `window` namespace.
  - `TerminalCompletionProvider.provideTerminalCompletions(terminal, context, token)` (line 31) — returns `T[] | TerminalCompletionList<T>`.
  - `TerminalCompletionItem` (line 48): `label`, `replacementRange: readonly [number, number]`, `detail`, `documentation`, `kind: TerminalCompletionItemKind`.
  - `TerminalCompletionItemKind` (line 96) — 17 values: `File`, `Folder`, `Method`, `Alias`, `Argument`, `Option`, `OptionValue`, `Flag`, `SymbolicLinkFile`, `SymbolicLinkFolder`, `ScmCommit`, `ScmBranch`, `ScmTag`, `ScmStash`, `ScmRemote`, `PullRequest`, `PullRequestDone`.
  - `TerminalCompletionContext` (line 189): `commandLine: string`, `cursorIndex: number`.
  - `TerminalCompletionList.resourceOptions?: TerminalCompletionResourceOptions` (line 215) — instructs the terminal to surface file/folder entries from a given `cwd: Uri` with a glob pattern.

- **Data flow:** Keystroke in terminal (or trigger character) → editor calls `provideTerminalCompletions(terminal, { commandLine, cursorIndex }, token)` → provider returns `TerminalCompletionList` → editor merges with resource completions and renders inline.

- **Dependencies:** `Terminal`, `CancellationToken`, `ProviderResult`, `MarkdownString`, `Uri`, `CompletionItemLabel` from stable API.

---

#### `src/vscode-dts/vscode.proposed.fileSearchProvider.d.ts`

- **Role:** Allows extensions to back the Quick Open file picker for custom URI schemes by registering a `FileSearchProvider`.

- **Key symbols:**
  - `workspace.registerFileSearchProvider(scheme, provider)` (line 71) — one provider per scheme.
  - `FileSearchProvider.provideFileSearchResults(query, options, token)` (line 58) — returns `Uri[]`.
  - `FileSearchQuery.pattern: string` (line 23) — raw Quick Open text; provider must apply its own scoring/filtering.
  - `FileSearchOptions extends SearchOptions` (line 29): `maxResults?`, `session?: CancellationToken` for cache keying.

- **Data flow:** Quick Open keystroke → `provideFileSearchResults()` called per scheme with current pattern → `Uri[]` merged with built-in results → displayed in picker.

- **Dependencies:** `Uri`, `CancellationToken`, `ProviderResult`, `SearchOptions` (from stable `workspace` types).

---

#### `src/vscode-dts/vscode.proposed.resolvers.d.ts`

- **Role:** Defines the complete remote-authority resolution ABI — including TCP/pipe tunnels, an `ExecServer` for spawning processes on remote machines, a `RemoteFileSystem`, and the `RemoteAuthorityResolver` interface — which is the most complex platform-integration contract any port must replicate.

- **Key symbols:**
  - `RemoteAuthorityResolver` (line 381): `resolve(authority, context)` → `ResolverResult` (a union of `ResolvedAuthority | ManagedResolvedAuthority` with options and tunnel info).
  - `ResolvedAuthority` (line 28): `host`, `port`, `connectionToken` — TCP handoff to the remote server.
  - `ManagedResolvedAuthority` (line 46): `makeConnection: () => Thenable<ManagedMessagePassing>` — for stream-based transports (WebSocket, named pipe).
  - `ManagedMessagePassing` (line 36): `onDidReceiveMessage: Event<Uint8Array>`, `send(data: Uint8Array)`, `end()`, `drain?()` — the raw byte-stream IPC abstraction.
  - `ExecServer` (line 161): `spawn()` (line 169), `spawnRemoteServerConnector?()` (line 180), `downloadCliExecutable?()` (line 191), `env()` (line 197), `kill()` (line 204), `tcpConnect()` (line 213), `fs: RemoteFileSystem` (line 222).
  - `RemoteFileSystem` (line 291): `stat`, `mkdirp`, `rm`, `read`, `write`, `connect`, `rename`, `readdir` — all returning `Thenable<...>` with `ReadStream`/`WriteStream` (line 255–263).
  - `RemoteServerConnector` (line 239): `connect(params: ServeParams)` → `Thenable<ManagedMessagePassing>` — bootstraps a server-side VS Code CLI.
  - `workspace.registerRemoteAuthorityResolver()` (line 456), `workspace.getRemoteExecServer()` (line 458).
  - `env.remoteAuthority: string | undefined` (line 472).

- **Control flow:** Editor detects `vscode-remote://` URI → calls `RemoteAuthorityResolver.resolve()` → gets `ResolverResult` → establishes either TCP connection (`ResolvedAuthority`) or managed stream (`ManagedResolvedAuthority`) → extension host communicates over that channel → extensions in remote host run against `RemoteFileSystem` for all I/O.

- **Data flow:** All data crosses the wire as `Uint8Array` streams (ReadStream / WriteStream). `ServeParams` (line 265) carries `socketId`, `commit`, `quality`, `extensions`, `compress`, `connectionToken`. `ExecEnvironment` (line 283) returns `env: ProcessEnv`, `osPlatform`, `osRelease`.

- **Dependencies:** `Uri`, `Event`, `Thenable`, `FileStat`, `FileType`, `AuthenticationSession` from stable API.

---

#### `src/vscode-dts/vscode.proposed.notebookKernelSource.d.ts`

- **Role:** Adds discovery and selection of notebook kernels (language runtimes) via a `NotebookKernelSourceActionProvider`, extending the `notebooks` namespace.

- **Key symbols:**
  - `notebooks.createNotebookControllerDetectionTask(notebookType)` (line 39) — signals ongoing kernel discovery.
  - `notebooks.registerKernelSourceActionProvider(notebookType, provider)` (line 44).
  - `NotebookKernelSourceActionProvider.provideNotebookKernelSourceActions(token)` (line 32) — returns `NotebookKernelSourceAction[]`.
  - `NotebookKernelSourceAction` (line 14): `label`, `description`, `detail`, `command: string | Command`, `documentation?: Uri`.

- **Dependencies:** `Command`, `Uri`, `CancellationToken`, `ProviderResult`, `Event` from stable API.

---

#### `src/vscode-dts/vscode.proposed.workspaceTrust.d.ts`

- **Role:** Adds workspace/resource trust gating — allowing extensions to check and request trust before performing sensitive file operations.

- **Key symbols:**
  - `workspace.onDidChangeWorkspaceTrustedFolders: Event<void>` (line 40).
  - `workspace.isResourceTrusted(resource)` (line 46) → `Thenable<boolean>`.
  - `workspace.requestResourceTrust(options)` (line 52) — shows trust dialog, returns `Thenable<boolean | undefined>`.
  - `workspace.requestWorkspaceTrust(options?)` (line 59).

- **Data flow:** Extension checks `isResourceTrusted(uri)` before reading; if false, calls `requestResourceTrust()` → user sees dialog → resolves to `true/false/undefined` → extension conditionally proceeds.

---

### Cross-Cutting Synthesis

The 174 files in `src/vscode-dts/` collectively define the **complete porting contract** for VS Code's IDE surface. The stable `vscode.d.ts` organises this into ~15 top-level namespaces. The porting challenge breaks into distinct layers:

1. **Editor kernel** (`window`, `TextDocument`, `TextEditor`, `TextEditorEdit`, Range/Position/Selection) — ~1,500 lines of type definitions mapping directly to the Monaco editor core. A Tauri/Rust port must either embed Monaco in a WebView or replace it entirely; all decoration, selection, and snippet types must be preserved.

2. **Language intelligence** (`namespace languages`) — 20+ provider registration points each return `ProviderResult<T>` (a Promise or direct value). These are the LSP feature points; a Tauri host must relay provider calls over an IPC bridge to extension host processes (Node.js or native Rust LSP clients).

3. **Debugger** (`namespace debug`) — built entirely on the Debug Adapter Protocol (`DebugProtocolMessage`, `DebugAdapter`, `DebugAdapterExecutable/Server/NamedPipeServer`). A Rust host can implement DAP relay natively; the `DebugVisualizationProvider` proposed API adds a tree-based custom variable viewer on top.

4. **Source control** (`namespace scm`, `scmHistoryProvider`) — `SourceControl` + `SourceControlResourceGroup` + the proposed `historyProvider` interface provide the git-graph, diff, and commit UI contracts.

5. **Terminal** (`window.createTerminal`, `TerminalCompletionProvider`) — terminals are spawned as child processes; the proposed completion provider adds shell-aware autocomplete on the command line.

6. **Remote transport** (`resolvers.d.ts`) — the most Rust-friendly layer: `ManagedMessagePassing` is a raw `Uint8Array` stream, `ExecServer` maps cleanly to Rust's `tokio::process::Command`, and `RemoteFileSystem` maps to `tokio::fs`. This is already the closest to a Rust ABI.

7. **AI/LM** (`chat`, `lm`, `chatProvider`) — streaming provider model with `Progress<LanguageModelResponsePart>`, tool-call round-trips, and MCP server spawning (stdio child process).

A Tauri/Rust port must either faithfully reimplement all `Disposable`-based registration, `Event<T>` pub/sub, `Thenable<T>` async, and `ProviderResult<T>` optional-async patterns in a Rust IPC layer, or maintain a Node.js extension host and expose these APIs over a message bus — the same architecture VS Code's remote development already uses.

---

### Out-of-Partition References

- `src/vs/workbench/api/common/extHost.api.impl.ts` — The TypeScript runtime implementation of every namespace (window, workspace, languages, debug, scm, etc.) declared in `vscode.d.ts`; this is where Disposable registrations, Event emitters, and ProviderResult wrappers are actually wired to core services.
- `src/vs/workbench/api/common/extHostLanguageFeatures.ts` — Implements all `languages.register*Provider()` calls and relays provider results to the language feature subsystem.
- `src/vs/workbench/api/common/extHostDebugService.ts` — Implements `namespace debug` including DAP session lifecycle and `DebugAdapter` proxy.
- `src/vs/workbench/api/common/extHostTerminalService.ts` — Implements `window.createTerminal`, terminal lifecycle events, and shell integration.
- `src/vs/workbench/api/common/extHostSCM.ts` — Implements `scm.createSourceControl()` and the `SourceControl`/`SourceControlResourceGroup` object graph.
- `src/vs/workbench/api/common/extHostFileSystem.ts` — Bridges `FileSystemProvider` registrations to the internal VFS.
- `src/vs/workbench/services/remote/common/remoteAuthorityResolverService.ts` — Hosts the `RemoteAuthorityResolver` call chain and manages connection token / tunnel lifecycle.
- `src/vs/platform/debug/common/debugProtocol.ts` — Defines the full typed DAP message schema that `DebugProtocolMessage`/`DebugProtocolSource`/`DebugProtocolBreakpoint` opaque types stand in for.
- `extensions/git/src/api/git.d.ts` — The concrete Git extension's own API surface, built on top of `scm` namespace and the `scmHistoryProvider` proposed API.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Core IDE Functionality Porting Patterns

Research of what it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

## Scope Analyzed
- `src/vscode-dts/` - VS Code type definitions (174 files, 33,614 LOC)

---

## Pattern 1: Text Editor Core Interface

**Where:** `src/vscode-dts/vscode.d.ts:1258-1379`

**What:** Defines the primary text editor abstraction with document, selections, visibility, and edit capabilities.

```typescript
export interface TextEditor {
	readonly document: TextDocument;
	selection: Selection;
	readonly selections: readonly Selection[];
	readonly visibleRanges: readonly Range[];
	options: TextEditorOptions;
	readonly viewColumn: ViewColumn | undefined;

	edit(callback: (editBuilder: TextEditorEdit) => void, options?: {
		readonly undoStopBefore: boolean;
		readonly undoStopAfter: boolean;
	}): Thenable<boolean>;

	insertSnippet(snippet: SnippetString, location?: Position | Range | readonly Position[] | readonly Range[], options?: {
		readonly undoStopBefore: boolean;
		readonly undoStopAfter: boolean;
		readonly keepWhitespace?: boolean;
	}): Thenable<boolean>;

	setDecorations(decorationType: TextEditorDecorationType, rangesOrOptions: readonly Range[] | readonly DecorationOptions[]): void;
	revealRange(range: Range, revealType?: TextEditorRevealType): void;
}
```

**Variations / call-sites:** 
- `src/vscode-dts/vscode.d.ts:11081` - `window.activeTextEditor` property
- `src/vscode-dts/vscode.d.ts:11086` - `window.visibleTextEditors` property
- `src/vscode-dts/vscode.d.ts:1400-1433` - `TextEditorEdit` interface for batch edits

---

## Pattern 2: Text Document Abstraction

**Where:** `src/vscode-dts/vscode.d.ts:88-259`

**What:** Core document model with metadata, line access, position/offset conversion, and validation.

```typescript
export interface TextDocument {
	readonly uri: Uri;
	readonly fileName: string;
	readonly isUntitled: boolean;
	readonly languageId: string;
	readonly encoding: string;
	readonly version: number;
	readonly isDirty: boolean;
	readonly isClosed: boolean;
	readonly eol: EndOfLine;
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
```

**Variations / call-sites:**
- `src/vscode-dts/vscode.d.ts:49-82` - `TextLine` interface for immutable line access
- `src/vscode-dts/vscode.d.ts:269-398` - `Position` class with comparison and translation methods
- `src/vscode-dts/vscode.d.ts:408-495` - `Range` class with intersection/union operations

---

## Pattern 3: Language Intelligence Providers

**Where:** `src/vscode-dts/vscode.d.ts:2925-2997`

**What:** Provider-based architecture for language features like definitions, implementations, type definitions, and declarations.

```typescript
export interface DefinitionProvider {
	provideDefinition(document: TextDocument, position: Position, token: CancellationToken): ProviderResult<Definition | DefinitionLink[]>;
}

export interface ImplementationProvider {
	provideImplementation(document: TextDocument, position: Position, token: CancellationToken): ProviderResult<Definition | DefinitionLink[]>;
}

export interface TypeDefinitionProvider {
	provideTypeDefinition(document: TextDocument, position: Position, token: CancellationToken): ProviderResult<Definition | DefinitionLink[]>;
}

export interface DeclarationProvider {
	provideDeclaration(document: TextDocument, position: Position, token: CancellationToken): ProviderResult<Declaration>;
}
```

**Variations / call-sites:**
- `src/vscode-dts/vscode.d.ts:3144-3176` - `HoverProvider` interface
- `src/vscode-dts/vscode.d.ts:5189-5224` - `CompletionItemProvider` with resolve capability
- `src/vscode-dts/vscode.d.ts:3638-3654` - `DocumentSymbolProvider` interface

---

## Pattern 4: Completion Items Provider

**Where:** `src/vscode-dts/vscode.d.ts:5189-5224`

**What:** IntelliSense completion provider with lazy resolution capability.

```typescript
export interface CompletionItemProvider<T extends CompletionItem = CompletionItem> {
	provideCompletionItems(document: TextDocument, position: Position, token: CancellationToken, context: CompletionContext): ProviderResult<T[] | CompletionList<T>>;

	resolveCompletionItem?(item: T, token: CancellationToken): ProviderResult<T>;
}

export class CompletionItem {
	label: string;
	kind?: CompletionItemKind;
	detail?: string;
	documentation?: string | MarkdownString;
	sortText?: string;
	filterText?: string;
	preselect?: boolean;
	insertText?: string | SnippetString;
	range?: Range | { inserting: Range; replacing: Range };
	commitCharacters?: string[];
	additionalTextEdits?: TextEdit[];
	command?: Command;
	tags?: readonly CompletionItemTag[];
}
```

**Variations / call-sites:**
- `src/vscode-dts/vscode.d.ts:5233-5249` - `InlineCompletionItemProvider` interface
- `src/vscode-dts/vscode.d.ts:5116-5140` - `CompletionList` for batched results

---

## Pattern 5: Source Control Integration

**Where:** `src/vscode-dts/vscode.d.ts:16580-16649`

**What:** Multi-level abstraction for source control systems with resources, groups, and input handling.

```typescript
export interface SourceControl {
	readonly id: string;
	readonly label: string;
	readonly rootUri: Uri | undefined;
	readonly inputBox: SourceControlInputBox;
	count?: number;
	quickDiffProvider?: QuickDiffProvider;
	commitTemplate?: string;
	acceptInputCommand?: Command;
	statusBarCommands?: Command[];

	createResourceGroup(id: string, label: string): SourceControlResourceGroup;
	dispose(): void;
}

export interface SourceControlResourceGroup {
	readonly id: string;
	label: string;
	hideWhenEmpty?: boolean;
	contextValue?: string;
	resourceStates: SourceControlResourceState[];
	dispose(): void;
}

export interface SourceControlResourceState {
	readonly resourceUri: Uri;
	readonly command?: Command;
	readonly decorations?: SourceControlResourceDecorations;
	readonly contextValue?: string;
}
```

**Variations / call-sites:**
- `src/vscode-dts/vscode.d.ts:16654-16673` - `scm` namespace with `createSourceControl()` factory
- `src/vscode-dts/vscode.d.ts:16390-16411` - `SourceControlInputBox` interface

---

## Pattern 6: Debug Adapter Protocol Integration

**Where:** `src/vscode-dts/vscode.d.ts:16699-16840`

**What:** Debug session management with configuration providers and debug adapter factories.

```typescript
export interface DebugConfiguration {
	type: string;
	name: string;
	request: string;
	[key: string]: any;
}

export interface DebugSession {
	readonly id: string;
	readonly type: string;
	readonly parentSession?: DebugSession;
	name: string;
	readonly workspaceFolder: WorkspaceFolder | undefined;
	readonly configuration: DebugConfiguration;

	customRequest(command: string, args?: any): Thenable<any>;
	getDebugProtocolBreakpoint(breakpoint: Breakpoint): Thenable<DebugProtocolBreakpoint | undefined>;
}

export interface DebugConfigurationProvider {
	provideDebugConfigurations?(folder: WorkspaceFolder | undefined, token?: CancellationToken): ProviderResult<DebugConfiguration[]>;
	resolveDebugConfiguration?(folder: WorkspaceFolder | undefined, debugConfiguration: DebugConfiguration, token?: CancellationToken): ProviderResult<DebugConfiguration>;
	resolveDebugConfigurationWithSubstitutedVariables?(folder: WorkspaceFolder | undefined, debugConfiguration: DebugConfiguration, token?: CancellationToken): ProviderResult<DebugConfiguration>;
}
```

**Variations / call-sites:**
- `src/vscode-dts/vscode.d.ts:16845-16873` - `DebugAdapterExecutable` for subprocess spawning
- `src/vscode-dts/vscode.d.ts:16896-17002` - `DebugAdapterServer` and `DebugAdapterTracker` interfaces

---

## Pattern 7: Terminal Integration

**Where:** `src/vscode-dts/vscode.d.ts:7669-7746`

**What:** Integrated terminal with process management, shell integration, and execution tracking.

```typescript
export interface Terminal {
	readonly name: string;
	readonly processId: Thenable<number | undefined>;
	readonly creationOptions: Readonly<TerminalOptions | ExtensionTerminalOptions>;
	readonly exitStatus: TerminalExitStatus | undefined;
	readonly state: TerminalState;
	readonly shellIntegration: TerminalShellIntegration | undefined;

	sendText(text: string, shouldExecute?: boolean): void;
	show(preserveFocus?: boolean): void;
	hide(): void;
	dispose(): void;
}

export interface TerminalShellIntegration {
	readonly cwd: Uri | undefined;
	executeCommand(commandLine: string): TerminalShellExecution;
}

export interface TerminalState {
	readonly isInteractedWith: boolean;
	readonly shell: string | undefined;
}
```

**Variations / call-sites:**
- `src/vscode-dts/vscode.d.ts:8162-8179` - `TerminalLinkProvider` interface
- `src/vscode-dts/vscode.d.ts:7828-7856` - `TerminalShellIntegration` with command execution
- `src/vscode-dts/vscode.d.ts:11159-11209` - Terminal event streams in `window` namespace

---

## Pattern 8: File System Provider Abstraction

**Where:** `src/vscode-dts/vscode.d.ts:9600-9764`

**What:** Pluggable file system with watch events, metadata, and CRUD operations.

```typescript
export interface FileSystemProvider {
	readonly onDidChangeFile: Event<FileChangeEvent[]>;

	watch(uri: Uri, options: {
		readonly recursive: boolean;
		readonly excludes: readonly string[];
	}): Disposable;

	stat(uri: Uri): FileStat | Thenable<FileStat>;
	readDirectory(uri: Uri): [string, FileType][] | Thenable<[string, FileType][]>;
	createDirectory(uri: Uri): void | Thenable<void>;
	readFile(uri: Uri): Uint8Array | Thenable<Uint8Array>;
	writeFile(uri: Uri, content: Uint8Array, options: {
		readonly create: boolean;
		readonly overwrite: boolean;
	}): void | Thenable<void>;
	delete(uri: Uri, options: { readonly recursive: boolean }): void | Thenable<void>;
	rename(oldUri: Uri, newUri: Uri, options: { readonly overwrite: boolean }): void | Thenable<void>;
	copy?(source: Uri, destination: Uri, options: { readonly overwrite: boolean }): void | Thenable<void>;
}

export interface FileSystem {
	stat(uri: Uri): Thenable<FileStat>;
	readDirectory(uri: Uri): Thenable<[string, FileType][]>;
	createDirectory(uri: Uri): Thenable<void>;
	readFile(uri: Uri): Thenable<Uint8Array>;
	writeFile(uri: Uri, content: Uint8Array): Thenable<void>;
	delete(uri: Uri, options?: { recursive?: boolean; useTrash?: boolean }): Thenable<void>;
	rename(source: Uri, target: Uri, options?: { overwrite?: boolean }): Thenable<void>;
}
```

**Variations / call-sites:**
- `src/vscode-dts/vscode.d.ts:9774-9849` - `FileSystem` convenience API
- `src/vscode-dts/vscode.d.ts:13807` - `workspace.fs` as public API instance

---

## Pattern 9: Diagnostic Collection Management

**Where:** `src/vscode-dts/vscode.d.ts:7168-7243`

**What:** Diagnostic collection for language problems with URI-based organization.

```typescript
export interface DiagnosticCollection extends Iterable<[uri: Uri, diagnostics: readonly Diagnostic[]]> {
	readonly name: string;

	set(uri: Uri, diagnostics: readonly Diagnostic[] | undefined): void;
	set(entries: ReadonlyArray<[Uri, readonly Diagnostic[] | undefined]>): void;

	delete(uri: Uri): void;
	clear(): void;
	forEach(callback: (uri: Uri, diagnostics: readonly Diagnostic[], collection: DiagnosticCollection) => any, thisArg?: any): void;
	get(uri: Uri): readonly Diagnostic[] | undefined;
	has(uri: Uri): boolean;
	dispose(): void;
}

export class Diagnostic {
	range: Range;
	message: string;
	source?: string;
	code?: string | number | { value: string | number; target: Uri };
	severity?: DiagnosticSeverity;
	relatedInformation?: DiagnosticRelatedInformation[];
	tags?: readonly DiagnosticTag[];
	codeDescription?: CodeDescription;
}
```

**Variations / call-sites:**
- `src/vscode-dts/vscode.d.ts:14816-14819` - `languages.createDiagnosticCollection()` factory
- `src/vscode-dts/vscode.d.ts:7096-7161` - `Diagnostic` class definition

---

## Pattern 10: Workspace Document Event Streams

**Where:** `src/vscode-dts/vscode.d.ts:14277-14316`

**What:** Document lifecycle event subscription pattern for tracking open, close, change, save, and will-save events.

```typescript
export const onDidOpenTextDocument: Event<TextDocument>;
export const onDidCloseTextDocument: Event<TextDocument>;
export const onDidChangeTextDocument: Event<TextDocumentChangeEvent>;
export const onWillSaveTextDocument: Event<TextDocumentWillSaveEvent>;
export const onDidSaveTextDocument: Event<TextDocument>;

export interface TextDocumentChangeEvent {
	readonly document: TextDocument;
	readonly contentChanges: readonly TextDocumentContentChangeEvent[];
}

export interface TextDocumentWillSaveEvent {
	readonly document: TextDocument;
	readonly reason: TextDocumentSaveReason;
	waitUntil(thenable: Thenable<TextEdit[]>): void;
	waitUntil(thenable: Thenable<any>): void;
}
```

**Variations / call-sites:**
- `src/vscode-dts/vscode.d.ts:13799-14600` - Full `workspace` namespace definition
- `src/vscode-dts/vscode.d.ts:11069-12500` - Full `window` namespace definition

---

## Pattern 11: Window/Editor State Management Namespace

**Where:** `src/vscode-dts/vscode.d.ts:11069-11244`

**What:** Window state API with active editor tracking and event streams.

```typescript
export namespace window {
	export let activeTextEditor: TextEditor | undefined;
	export let visibleTextEditors: readonly TextEditor[];
	export const onDidChangeActiveTextEditor: Event<TextEditor | undefined>;
	export const onDidChangeVisibleTextEditors: Event<readonly TextEditor[]>;
	export const onDidChangeTextEditorSelection: Event<TextEditorSelectionChangeEvent>;
	export const onDidChangeTextEditorVisibleRanges: Event<TextEditorVisibleRangesChangeEvent>;
	export const onDidChangeTextEditorOptions: Event<TextEditorOptionsChangeEvent>;
	export const onDidChangeTextEditorViewColumn: Event<TextEditorViewColumnChangeEvent>;

	export const visibleNotebookEditors: readonly NotebookEditor[];
	export const onDidChangeVisibleNotebookEditors: Event<readonly NotebookEditor[]>;
	export const activeNotebookEditor: NotebookEditor | undefined;
	export const onDidChangeActiveNotebookEditor: Event<NotebookEditor | undefined>;

	export function showTextDocument(document: TextDocument, column?: ViewColumn, preserveFocus?: boolean): Thenable<TextEditor>;
	export function showTextDocument(document: TextDocument, options?: TextDocumentShowOptions): Thenable<TextEditor>;
	export function showInformationMessage<T extends string>(message: string, ...items: T[]): Thenable<T | undefined>;
	export function createTextEditorDecorationType(options: DecorationRenderOptions): TextEditorDecorationType;
}
```

**Variations / call-sites:**
- `src/vscode-dts/vscode.d.ts:11159-11210` - Terminal management in window namespace
- `src/vscode-dts/vscode.d.ts:11074` - `tabGroups: TabGroups` for editor tab management

---

## Pattern 12: Commands Registration and Execution

**Where:** `src/vscode-dts/vscode.d.ts:10973`

**What:** Command registration and execution namespace used throughout architecture.

```typescript
export namespace commands {
	export function registerCommand(command: string, callback: (...args: any[]) => any, thisArg?: any): Disposable;
	export function registerTextEditorCommand(command: string, callback: (textEditor: TextEditor, edit: TextEditorEdit, ...args: any[]) => void, thisArg?: any): Disposable;
	export function executeCommand<T>(command: string, ...rest: any[]): Thenable<T | undefined>;
	export function getCommands(filterInternal?: boolean): Thenable<string[]>;
}
```

**Variations / call-sites:**
- `src/vscode-dts/vscode.d.ts:19` - `Command` interface definition
- Used throughout all major APIs for action binding

---

## Summary

The `src/vscode-dts/` contains comprehensive type definitions representing VS Code's core IDE abstractions:

1. **Text Editing Core**: TextEditor, TextDocument, Position, Range, TextLine - fundamental model for text content
2. **Language Intelligence**: Provider-based architecture for hover, completion, definitions, implementations
3. **Source Control**: Multi-level abstraction with resource groups and quick diff support
4. **Debugging**: Debug adapter protocol integration with configuration resolution and session management
5. **Terminal**: Integrated terminal with shell integration, execution tracking, and PTY management
6. **File System**: Pluggable filesystem with watch events and CRUD operations
7. **Diagnostics**: URI-based diagnostic collection for language problems
8. **Workspace/Events**: Rich event streams for document lifecycle, editor state, and file changes
9. **Commands**: Command registration and execution model used throughout

Porting these to Tauri/Rust would require:
- Language-agnostic RPC protocol (likely use Debug Adapter Protocol pattern more broadly)
- Pluggable provider system mapping to Rust trait objects
- Event subscription system with cancellation tokens
- Multi-threaded async operations with Thenable-like promises
- File system abstraction layer (virtual filesystems for remote/custom schemes)
- Diagnostic aggregation and reporting system
- Terminal PTY management integration
- Provider resolution and caching mechanisms

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
