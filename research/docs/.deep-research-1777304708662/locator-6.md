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

