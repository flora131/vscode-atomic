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
