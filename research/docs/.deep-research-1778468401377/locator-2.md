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

