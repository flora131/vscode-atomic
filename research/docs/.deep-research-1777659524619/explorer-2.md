# Partition 2 of 79 — Findings

## Scope
`extensions/copilot/` (2880 files, 682,973 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Location Index: VS Code Copilot Extension (extensions/copilot/)

## Scope Summary
- **Directory**: `extensions/copilot/` 
- **Size**: 2,880 files, 682,973 LOC
- **Language**: TypeScript/JavaScript with JSX/TSX components
- **Focus**: Copilot chat, tools, agents, completions, and IDE integration

Research question: Porting VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

---

## Implementation

### Core Extension Architecture
- `extensions/copilot/src/extension/extension/vscode-node/extension.ts` - Node.js extension entry point (43 lines)
- `extensions/copilot/src/extension/extension/vscode-worker/extension.ts` - Worker extension variant
- `extensions/copilot/src/extension/extension/vscode/extension.ts` - Base VSCode extension (all runtimes)
- `extensions/copilot/src/extension/api/vscode/extensionApi.ts` - CopilotExtensionApi implementation
- `extensions/copilot/src/extension/api/vscode/api.d.ts` - Type definitions for CopilotExtensionApi
- `extensions/copilot/src/extension/api/vscode/vscodeContextProviderApi.ts` - Context provider API (V1)

### Chat & Conversation System
- `extensions/copilot/src/extension/conversation/vscode-node/chatParticipants.ts` - Chat participant registration
- `extensions/copilot/src/extension/chatSessions/` (contains 7 main subdirectories)
  - `chatSessions/vscode-node/chatSessions.ts` - Main chat session management
  - `chatSessions/vscode-node/copilotCloudSessionsProvider.ts` - Cloud sessions
  - `chatSessions/claude/` - Claude-specific session handling (common, node, vscode-node variants)
  - `chatSessions/copilotcli/` - CLI integration for chat
  - Claude session subsystem: `claudeCodeAgent.ts`, `claudeLanguageModelServer.ts`
- `extensions/copilot/src/platform/chat/common/chatAgents.ts` - Chat agent definitions
- `extensions/copilot/src/extension/chat/vscode-node/chatHookService.ts` - Chat hook lifecycle management

### Tools & Tool Registry
- `extensions/copilot/src/extension/tools/common/toolsRegistry.ts` - Central tool registry
- `extensions/copilot/src/extension/tools/common/toolDeferralService.ts` - Tool deferral handling
- `extensions/copilot/src/extension/tools/node/` - 40+ tool implementations:
  - File operations: `readFileTool.tsx`, `createFileTool.tsx`, `findFilesTool.tsx`
  - Code operations: `replaceStringTool.tsx`, `insertEditTool.tsx`, `applyPatchTool.tsx`
  - Search: `findTextInFilesTool.tsx`, `searchWorkspaceSymbolsTool.tsx`, `githubTextSearchTool.tsx`
  - Execution: `executionSubagentTool.ts`, `vscodeCmdTool.tsx`, `vscodeAPITool.ts`
  - Development: `getErrorsTool.tsx`, `scmChangesTool.ts`, `searchWorkspaceSymbolsTool.tsx`
  - Workspace: `listDirTool.tsx`, `createDirectoryTool.tsx`, `findTestsFilesTool.tsx`
  - Memory: `memoryTool.tsx`, `manageTodoListTool.tsx`, `sessionStoreSqlTool.ts`
  - Notebooks: `editNotebookTool.tsx`, `runNotebookCellTool.tsx`, `notebookSummaryTool.tsx`
  - Install/Setup: `installExtensionTool.tsx`, `newWorkspaceTool.tsx`, `codebaseTool.tsx`
- `extensions/copilot/src/extension/tools/vscode-node/` - VSCode-specific tools
  - `fetchWebPageTool.tsx` - Web fetching
  - `switchAgentTool.ts` - Agent switching
  - `tools.ts` - Tool orchestration

### Agents System
- `extensions/copilot/src/extension/agents/vscode-node/` - Agent provider implementations
  - `askAgentProvider.ts` - Ask agent
  - `editModeAgentProvider.ts` - Edit mode agent
  - `planAgentProvider.ts` - Plan agent
  - `exploreAgentProvider.ts` - Explore agent
  - `githubOrgCustomAgentProvider.ts` - GitHub org agents
- `extensions/copilot/src/extension/agents/node/langModelServer.ts` - Language model server
- `extensions/copilot/src/extension/agents/node/adapters/` - LLM adapters (Anthropic, OpenAI)

### Language Intelligence & Support
- `extensions/copilot/src/platform/languages/` - Language service abstraction
  - `languageFeaturesService.ts` - Language features (vscode & common)
  - `languageDiagnosticsService.ts` - Diagnostics (vscode & common)
- `extensions/copilot/src/platform/languageServer/` - LSP integration
  - `languageContextService.ts` - Language context
- `extensions/copilot/src/platform/languageContextProvider/` - Context for languages
- `extensions/copilot/src/platform/parser/` - Language parsing (tree-sitter integration)
- `extensions/copilot/src/extension/typescriptContext/` - TypeScript-specific context
  - `serverPlugin/` - TypeScript server plugin integration

### Debugging & Error Handling
- `extensions/copilot/src/platform/debug/` - Debug service abstraction
  - `debugOutputService.ts` - Common interface
  - `debugOutputServiceImpl.ts` - VSCode implementation
  - `debugOutputListener.ts` - Debug listener
- `extensions/copilot/src/extension/agentDebug/` - Agent debugging
  - `toolResultContentRenderer.ts` - Tool result rendering
  - `toolResultRenderer.ts` - Rendering logic
- `extensions/copilot/src/extension/onboardDebug/` - Debug onboarding
- `extensions/copilot/src/extension/agentDebug/` - Contains 7+ debug-related files

### Terminal Integration
- `extensions/copilot/src/platform/terminal/` - Terminal service abstraction
  - `terminalService.ts` - Common interface
  - `terminalServiceImpl.ts` - VSCode implementation (vscode/)
  - `terminalBufferListener.ts` - Listen to terminal buffer
- `extensions/copilot/src/util/common/test/shims/terminal.ts` - Terminal test shim
- `extensions/copilot/src/util/vs/workbench/contrib/terminalContrib/` - Terminal contributions

### Source Control & Git Integration
- `extensions/copilot/src/platform/git/` - Git service abstraction
  - `gitService.ts` - Common interface
  - `gitServiceImpl.ts` - Node implementation (vscode-node/)
  - `gitExtensionService.ts` - Extension-based git
  - `gitDiffService.ts` - Diff functionality
  - `gitCommitMessageService.ts` - Commit message generation
- `extensions/copilot/src/extension/git/` - Git-related features

### Code Editing & Transformations
- `extensions/copilot/src/extension/completions/` - Code completions system
- `extensions/copilot/src/extension/completions-core/` - Core completions engine (TypeScript/React focus)
  - `vscode-node/lib/src/` - Core logic
  - `vscode-node/prompt/src/` - Prompt generation
  - `vscode-node/extension/src/` - Extension integration
- `extensions/copilot/src/extension/inlineEdits/` - Inline edit handling (6 directories)
- `extensions/copilot/src/extension/inlineChat/` - Inline chat interface
- `extensions/copilot/src/extension/inlineChat2/` - Inline chat v2
- `extensions/copilot/src/platform/multiFileEdit/` - Multi-file edit support

### Navigation & Search
- `extensions/copilot/src/extension/search/` - Search functionality
- `extensions/copilot/src/extension/workspaceSemanticSearch/` - Semantic search
- `extensions/copilot/src/extension/workspaceChunkSearch/` - Chunk-based search
- `extensions/copilot/src/extension/workspaceRecorder/` - Workspace recording/indexing
- `extensions/copilot/src/platform/remoteSearch/` - Remote search service
- `extensions/copilot/src/platform/remoteCodeSearch/` - Code search via remote
- `extensions/copilot/src/platform/remoteRepositories/` - Remote repos support

### Model & Completion Providers
- `extensions/copilot/src/extension/byok/` - BYOK (Bring Your Own Key) providers
  - `vscode-node/` - VSCode BYOK providers
    - `openAIProvider.ts`, `azureProvider.ts`, `anthropicProvider.ts`
    - `geminiNativeProvider.ts`, `ollamaProvider.ts`, `openRouterProvider.ts`
  - `common/` - Message/function converters (Anthropic, Gemini, etc.)
- `extensions/copilot/src/platform/endpoint/` - LLM endpoint abstraction
  - `copilotChatEndpoint.ts` - Chat endpoint
  - `proxyAgenticEndpoint.ts` - Agentic proxy
  - `autoChatEndpoint.ts` - Auto-routing
- `extensions/copilot/src/platform/proxyModels/` - Proxy model handling

### Workspace & Project Context
- `extensions/copilot/src/extension/context/` - Context resolution system
- `extensions/copilot/src/extension/chatSessionContext/` - Session-specific context
- `extensions/copilot/src/extension/diagnosticsContext/` - Diagnostics context
- `extensions/copilot/src/extension/promptFileContext/` - File-based prompts
- `extensions/copilot/src/extension/languageContextProvider/` - Language context

### Prompts & Response Generation
- `extensions/copilot/src/extension/prompts/node/` - Large prompt system
  - `agent/` - Agent-specific prompts (40+ variants)
  - `panel/`, `inline/`, `feedback/` - UI-specific prompts
  - `base/` - Base prompt components
- `extensions/copilot/src/extension/prompt/` - Low-level prompt utilities

### Telemetry & Observability
- `extensions/copilot/src/platform/telemetry/` - Telemetry abstraction
  - `telemetry.ts`, `telemetry2.ts`, `experimentation.ts` (node implementations)
- `extensions/copilot/src/platform/otel/` - OpenTelemetry integration
  - `node/`, `common/`, `vscode/` variants
  - `sqlite/` - Local span storage
- `extensions/copilot/src/extension/otel/` - Extension-level telemetry
- `extensions/copilot/src/extension/telemetry/` - Copilot-specific telemetry
- `extensions/copilot/src/extension/trajectory/` - Span/trajectory tracking

### MCP (Model Context Protocol) Integration
- `extensions/copilot/src/extension/mcp/` - MCP support
  - `vscode-node/mcpToolCallingLoop.tsx` - MCP tool loop
  - `vscode-node/mcpToolCallingLoopPrompt.tsx` - MCP prompts
  - `vscode-node/mcpToolCallingTools.tsx` - MCP tools
- `extensions/copilot/src/extension/chatSessions/claude/common/mcpServers/` - MCP servers (IDE server)
- `extensions/copilot/src/extension/githubMcp/` - GitHub MCP integration

### Skill & Intent System
- `extensions/copilot/src/extension/intents/` - Intent detection and execution
  - `askAgentIntent.ts`, `testIntent.tsx`, `refactorIntent.tsx`, etc.
  - `node/` - Intent implementations
- `extensions/copilot/src/extension/chatSessions/claude/node/claudeSkills.ts` - Skill definitions

### Notebook Support
- `extensions/copilot/src/extension/notebook/` - Jupyter notebook integration
- Notebook prompts in `extensions/copilot/src/extension/prompts/node/inline/inlineChat*Notebook*`

### Configuration & Storage
- `extensions/copilot/src/platform/configuration/` - Configuration service
  - `configurationService.ts` - Common interface
  - Implementation variants: `vscode/`, `common/`
  - `validator.ts` - Configuration validation
- `extensions/copilot/src/extension/configuration/` - Extension configuration
- `extensions/copilot/src/extension/conversationStore/` - Conversation persistence
- `extensions/copilot/src/extension/chronicle/` - Chronicle (conversation history)

### File System & Workspace
- `extensions/copilot/src/platform/filesystem/` - FS abstraction
  - Node implementation in `node/`
- `extensions/copilot/src/extension/workspaceChunkSearch/` - Chunk indexing
- `extensions/copilot/src/extension/workspaceRecorder/` - Workspace recording

### Testing & Debugging Intents
- `extensions/copilot/src/extension/intents/node/testIntent/` - Test-related intents
- `extensions/copilot/src/extension/testing/` - Testing utilities
- `extensions/copilot/src/platform/testing/` - Test detection service

### Authentication
- `extensions/copilot/src/extension/authentication/vscode-node/authentication.contribution.ts` - Auth setup
- `extensions/copilot/src/platform/authentication/` - Auth service abstraction
  - `authentication.ts` - Common interface
  - `copilotToken.ts` - Token management
  - Node implementations in `node/`

### External Services & APIs
- `extensions/copilot/src/platform/github/` - GitHub API service
- `extensions/copilot/src/platform/openai/` - OpenAI integration
- `extensions/copilot/src/platform/nesFetch/` - Custom fetch utilities
- `extensions/copilot/src/platform/networking/` - Network service

---

## Tests

### Unit Tests (451 total across codebase)

#### Platform Tests
- `extensions/copilot/src/platform/chat/test/node/hookExecutor.spec.ts`
- `extensions/copilot/src/platform/log/test/common/subLogger.spec.ts`
- `extensions/copilot/src/platform/embeddings/test/node/` (2 test files)
- `extensions/copilot/src/platform/tokenizer/test/node/tokenizer.spec.ts`
- `extensions/copilot/src/platform/git/test/node/gitService.spec.ts`
- `extensions/copilot/src/platform/configuration/test/` (multiple test files)
- `extensions/copilot/src/platform/telemetry/test/node/` (3 test files)
- `extensions/copilot/src/platform/endpoint/node/test/` (2+ test files)
- `extensions/copilot/src/platform/authentication/test/node/` (2 test files)
- `extensions/copilot/src/platform/byok/node/test/` (3+ test files)
- `extensions/copilot/src/platform/otel/` (multiple test directories)
- `extensions/copilot/src/platform/workbench/test/vscode-node/workbenchServiceImpl.test.ts`
- `extensions/copilot/src/platform/customInstructions/test/node/customInstructionsService.spec.ts`

#### Extension Tests
- `extensions/copilot/src/extension/agents/vscode-node/test/` (5+ test files)
- `extensions/copilot/src/extension/byok/vscode-node/test/` (3+ test files)
- `extensions/copilot/src/extension/chat/vscode-node/test/chatHookService.spec.ts`
- `extensions/copilot/src/extension/chatSessions/claude/node/test/` (3+ test files)
- `extensions/copilot/src/extension/chatSessions/vscode-node/test/copilotCLIChatSessions.spec.ts`
- `extensions/copilot/src/extension/completions-core/vscode-node/` (multiple test directories)
- `extensions/copilot/src/extension/conversation/vscode-node/test/conversationFeature.test.ts`
- `extensions/copilot/src/extension/intents/test/node/validateToolMessages.spec.ts`
- `extensions/copilot/src/extension/inlineChat2/test/node/inlineChat2Prompt.spec.tsx`

#### Utility Tests
- `extensions/copilot/src/util/node/test/` (6 test files)
- `extensions/copilot/src/util/common/test/` (10+ test files)

#### Integration & E2E Tests
- `extensions/copilot/test/inline/multiFileEdit.stest.ts`
- `extensions/copilot/test/inline/inlineGenerateCode.stest.ts`
- `extensions/copilot/test/e2e/terminal.stest.ts`
- `extensions/copilot/test/e2e/cli.stest.ts`
- `extensions/copilot/test/intent/` - Intent testing
- `extensions/copilot/test/simulation/` - Simulation tests with fixtures

---

## Types / Interfaces

### Core Type Definitions
- `extensions/copilot/src/extension/api/vscode/api.d.ts` - CopilotExtensionApi interface
- `extensions/copilot/src/extension/vscode-api.d.ts` - VSCode API extensions
- `extensions/copilot/src/util/common/types.ts` - Common type utilities
- `extensions/copilot/src/util/common/result.ts` - Result<T> type

### Type Shims (for testing)
- `extensions/copilot/src/util/common/test/shims/` - Contains 11 shim files
  - `vscodeTypesShim.ts`, `chatTypes.ts`, `editing.ts`, `enums.ts`, `l10n.ts`
  - `terminal.ts`, `themes.ts`, `notebookDocument.ts`, `textDocument.ts`, `textEditor.ts`

### Domain-Specific Types
- `extensions/copilot/src/extension/agents/vscode-node/agentTypes.ts` - Agent types
- `extensions/copilot/src/extension/agents/node/adapters/types.ts` - Adapter types
- `extensions/copilot/src/platform/inlineEdits/common/dataTypes/` - Edit types
- `extensions/copilot/src/util/common/globals.d.ts` - Global declarations

---

## Configuration

### Project Configuration
- `extensions/copilot/package.json` - Main manifest (2000+ lines)
- `extensions/copilot/tsconfig.json` - Main TypeScript config
- `extensions/copilot/tsconfig.base.json` - Base config
- `extensions/copilot/tsconfig.worker.json` - Worker config
- `extensions/copilot/vite.config.ts` - Build config
- `extensions/copilot/.esbuild.mts` - Build script
- `extensions/copilot/tsfmt.json` - Formatter config

### Development Configuration
- `extensions/copilot/.vscode/settings.json` - VSCode settings
- `extensions/copilot/.vscode/launch.json` - Debug launch configs
- `extensions/copilot/.vscode/tasks.json` - Task definitions
- `extensions/copilot/.devcontainer/devcontainer.json` - Dev container
- `extensions/copilot/.vscode/mcp.json` - MCP configuration

### Metadata
- `extensions/copilot/package-lock.json` - Dependency lock
- `extensions/copilot/cgmanifest.json` - Component governance
- `extensions/copilot/package.nls.json` - Localization strings

---

## Examples / Fixtures

### Simulation Test Fixtures
- `extensions/copilot/test/simulation/fixtures/` - 40+ fixture directories
  - `vscode/` - VSCode API mocks
  - `edit/`, `generate/`, `doc/`, `multiFileEdit/` - Feature test scenarios

### Scenario Tests
- `extensions/copilot/test/scenarios/` - 15+ scenario directories
  - `test-terminal/`, `test-tools/`, `test-system/`, etc.

### Language & TypeScript Fixtures
- `extensions/copilot/src/extension/typescriptContext/serverPlugin/fixtures/` - TS plugin fixtures
  - `context/` - 14+ context test projects
  - `nes/` - 2+ NES fixture projects

### Mock Services
- `extensions/copilot/src/platform/test/node/testChatAgentService.ts` - Mock chat agent
- `extensions/copilot/src/lib/vscode-node/test/` - Library mocks
- `extensions/copilot/src/util/common/test/mocks/` - Various mocks

---

## Documentation

### Project Documentation
- `extensions/copilot/README.md` - Main readme
- `extensions/copilot/CHANGELOG.md` - Version history
- `extensions/copilot/CONTRIBUTING.md` - Contribution guidelines
- `extensions/copilot/SECURITY.md` - Security policy

### Feature Documentation
- `extensions/copilot/docs/tools.md` - Tools documentation
- `extensions/copilot/docs/NES_EXPECTED_EDIT_CAPTURE.md` - NES documentation
- `extensions/copilot/chat-lib/README.md` - Chat library overview

### Development Guides
- `extensions/copilot/src/extension/typescriptContext/DEVELOPMENT.md` - TS context development
- `extensions/copilot/src/extension/trajectory/ARCHITECTURE.md` - Span architecture
- `extensions/copilot/src/extension/chatSessions/claude/CLAUDE_SESSION_USER_GUIDE.md` - Claude sessions
- `extensions/copilot/.github/instructions/` - Multiple instruction files (6+)

---

## Notable Clusters

### Chat System
**Location**: `extensions/copilot/src/extension/chatSessions/` + related
**Contains**: 7 directories, 40+ files (Claude, CLI, Copilot Cloud)
**Scope**: Complete chat session abstraction with multiple backends

### Tools System
**Location**: `extensions/copilot/src/extension/tools/`
**Contains**: ~50 tool implementations + registry
**Organization**: `node/`, `vscode-node/`, `common/` patterns

### Agents System
**Location**: `extensions/copilot/src/extension/agents/`
**Contains**: 6+ agent types with LLM adapters
**Key Files**: Agent providers, language model server

### Completions Engine
**Location**: `extensions/copilot/src/extension/completions/` + `completions-core/`
**Contains**: Multi-level context, prompt generation
**Organization**: `lib/`, `prompt/`, `extension/` subdirectories

### Prompt System
**Location**: `extensions/copilot/src/extension/prompts/node/`
**Contains**: 40+ specialized prompts (agent, panel, inline, feedback)
**Format**: JSX/TSX composition

### BYOK System
**Location**: `extensions/copilot/src/extension/byok/`
**Contains**: 8+ LLM providers (OpenAI, Azure, Anthropic, Gemini, Ollama)
**Scope**: Pluggable model provider system with converters

### Language Services
**Location**: `extensions/copilot/src/platform/languages/` + `languageServer/`
**Contains**: Language detection, diagnostics, parser integration
**Pattern**: Abstraction layer with VSCode implementation

### Debugging & Telemetry
**Location**: `extensions/copilot/src/extension/trajectory/` + `platform/otel/`
**Contains**: OpenTelemetry, span tracking, SQLite-based storage
**Scope**: Full observability infrastructure

### MCP Integration
**Location**: `extensions/copilot/src/extension/mcp/` + `chatSessions/claude/common/mcpServers/`
**Contains**: Tool calling loop, IDE MCP server
**Purpose**: Model Context Protocol support

### Git/Terminal Integration
**Location**: `extensions/copilot/src/platform/git/` + `platform/terminal/`
**Contains**: Service abstractions for version control and terminal
**Scope**: Multi-backend support

---

## Summary

The `extensions/copilot/` directory shows a highly modular extension system built on VSCode APIs. For Tauri/Rust porting:

1. **Layered Abstraction**: Most subsystems use `common/` + platform-specific layers (`vscode-node/`, `vscode-worker/`)
2. **Tool-Centric**: Registry-based 50+ tools for IDE manipulation
3. **LLM Integration**: Pluggable backends (OpenAI, Anthropic, Azure, Gemini, Ollama)
4. **Service-Based**: Git, terminal, filesystem, config exposed via abstractions
5. **Observability**: OpenTelemetry with local SQLite span storage
6. **Language Support**: LSP, tree-sitter, multi-language diagnostics
7. **Prompt Engineering**: 40+ specialized JSX/TSX-based prompts
8. **Chat Sessions**: Abstracted lifecycle supporting multiple backends

**Port Complexity**: Tool system, language services, and platform abstractions would be primary porting targets. VSCode extension API dependencies require complete reimplementation.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Core IDE Functionality - Pattern Research

**Scope**: `extensions/copilot/` - Chat participants, LM registration, tool registration patterns

## Patterns Found

### Pattern 1: Chat Participant Creation and Registration
**Where:** `extensions/copilot/src/extension/chatSessions/vscode-node/chatSessions.ts:158`

**What:** Creating and registering chat participants with VS Code's chat API. Chat participants are the agents/assistants visible in the chat view, each with a unique scheme/ID and handler function.

```typescript
const chatParticipant = vscode.chat.createChatParticipant(ClaudeSessionUri.scheme, chatSessionContentProvider.createHandler());
chatParticipant.iconPath = new vscode.ThemeIcon('claude');
this._register(vscode.chat.registerChatSessionContentProvider(ClaudeSessionUri.scheme, chatSessionContentProvider, chatParticipant));
const claudeCustomizationProvider = this._register(claudeAgentInstaService.createInstance(ClaudeCustomizationProvider));
this._register(vscode.chat.registerChatSessionCustomizationProvider(ClaudeSessionUri.scheme, ClaudeCustomizationProvider.metadata, claudeCustomizationProvider));
```

**Variations / call-sites:**
- `extensions/copilot/src/extension/chatSessions/vscode-node/chatSessions.ts:232` - Copilot CLI participant creation
- `extensions/copilot/src/extension/chatSessions/vscode-node/copilotCloudSessionsProvider.ts:266` - Cloud sessions participant with async handler
- `extensions/copilot/src/extension/conversation/vscode-node/chatParticipants.ts:98` - Generic agent creation with intent-based routing

---

### Pattern 2: Chat Participant Handler Implementation
**Where:** `extensions/copilot/src/extension/conversation/vscode-node/chatParticipants.ts:204-276`

**What:** Chat participant handlers implement the request-response loop for chat interactions. They receive requests, manage context history, handle model switching, and return ChatResult objects with streaming support.

```typescript
private getChatParticipantHandler(id: string, name: string, defaultIntentIdOrGetter: IntentOrGetter): vscode.ChatExtendedRequestHandler {
    return async (request, context, stream, token): Promise<vscode.ChatResult> => {
        markChatExt(request.sessionId, ChatExtPerfMark.WillHandleParticipant);
        try {
            // Switch to base model if needed
            request = await this.switchToBaseModel(request, stream);
            
            // Handle rate limit confirmations
            const switchToAutoConfirmation = getSwitchToAutoOnRateLimitConfirmation(request);
            if (switchToAutoConfirmation) {
                this.telemetryService.sendMSFTTelemetryEvent('chatRateLimitAction', { action, modelId: request.model?.id });
                request = await this.switchToAutoModel(request, stream, switchToAutoConfirmation.alwaysSwitchToAuto);
            }
            
            // Create handler for actual processing
            const handler = this.instantiationService.createInstance(ChatParticipantRequestHandler, context.history, request, stream, token, { agentName: name, agentId: id, intentId }, () => context.yieldRequested, telemetryMessageId);
            let result = await handler.getResult();
            
            return result;
        } finally {
            markChatExt(request.sessionId, ChatExtPerfMark.DidHandleParticipant);
            clearChatExtMarks(request.sessionId);
        }
    };
}
```

---

### Pattern 3: Language Model Registration
**Where:** `extensions/copilot/src/extension/chatSessions/claude/node/claudeCodeModels.ts:61-78`

**What:** Language models (LMs) are registered with VS Code's language model system via a provider that furnishes model metadata and handles model information queries. This makes models appear in VS Code's model picker.

```typescript
public registerLanguageModelChatProvider(lm: typeof vscode['lm']): void {
    const provider: vscode.LanguageModelChatProvider = {
        onDidChangeLanguageModelChatInformation: this._onDidChange.event,
        provideLanguageModelChatInformation: async (_options, _token) => {
            return this._provideLanguageModelChatInfo();
        },
        provideLanguageModelChatResponse: async (_model, _messages, _options, _progress, _token) => {
            // Implemented via chat participants.
        },
        provideTokenCount: async (_model, _text, _token) => {
            // Token counting not currently supported for claude provider.
            return 0;
        }
    };
    this._register(lm.registerLanguageModelChatProvider('claude-code', provider));
    void this._getEndpoints().then(() => this._onDidChange.fire());
}
```

**Variations / call-sites:**
- `extensions/copilot/src/extension/chatSessions/vscode-node/chatSessions.ts:156,230,330` - Multiple chat session types registering providers
- `extensions/copilot/src/extension/byok/vscode-node/byokContribution.ts:77` - BYOK (Bring Your Own Key) model registration

---

### Pattern 4: Tool Registry and Registration
**Where:** `extensions/copilot/src/extension/tools/common/toolsRegistry.ts:122-153`

**What:** Tools are collected in a ToolRegistry and registered via a static pattern. The registry maintains tool constructors and tracks non-deferred tools. Tools can be model-specific or global.

```typescript
public registerTool(tool: ICopilotToolCtor) {
    this._tools.push(tool);
    if (tool.nonDeferred) {
        this._nonDeferredToolNames.add(tool.toolName);
    }
}

public registerModelSpecificTool(definition: vscode.LanguageModelToolDefinition, tool: IModelSpecificToolCtor): IDisposable {
    if (this._modelSpecificTools.has(definition.name)) {
        throw new Error(`Model specific tool for ${definition.name} is already registered`);
    }
    this._modelSpecificTools.set(definition.name, { definition, tool });
    return {
        dispose: () => {
            this._modelSpecificTools.delete(definition.name);
        }
    };
}
```

**Variations / call-sites:**
- `extensions/copilot/src/extension/tools/vscode-node/switchAgentTool.ts:65` - Tool static registration
- `extensions/copilot/src/extension/tools/node/readFileTool.tsx:389` - ReadFile tool registration
- `extensions/copilot/src/extension/tools/node/memoryTool.tsx:846` - Memory tool registration
- 40+ additional tool registrations across the codebase

---

### Pattern 5: VS Code Tool Registration and Invocation
**Where:** `extensions/copilot/src/extension/tools/vscode-node/tools.ts:36-56`

**What:** Tools implementing the `vscode.LanguageModelTool` interface are registered with `vscode.lm.registerTool()`. Model-specific tools use `vscode.lm.registerToolDefinition()` with dynamic management via disposable subscriptions.

```typescript
for (const [name, tool] of toolsService.copilotTools) {
    if (isVscodeLanguageModelTool(tool)) {
        this._register(vscode.lm.registerTool(getContributedToolName(name), tool));
    }
}

const modelSpecificTools = this._register(new DisposableMap<string>());
this._register(autorunIterableDelta(
    reader => toolsService.modelSpecificTools.read(reader),
    ({ addedValues, removedValues }) => {
        for (const { definition } of removedValues) {
            modelSpecificTools.deleteAndDispose(definition.name);
        }
        for (const { definition, tool } of addedValues) {
            if (isVscodeLanguageModelTool(tool)) {
                modelSpecificTools.set(definition.name, vscode.lm.registerToolDefinition(definition, tool));
            }
        }
    },
    v => v.definition,
));
```

---

### Pattern 6: MCP Server Tool Registration (Model Context Protocol)
**Where:** `extensions/copilot/src/extension/chatSessions/copilotcli/vscode-node/tools/getVscodeInfo.ts:11-26`

**What:** MCP (Model Context Protocol) servers register tools using the standard MCP interface. Tools are registered with name, config (description, inputSchema), and async handler function that returns structured results.

```typescript
export function registerGetVscodeInfoTool(server: McpServer, logger: ILogger): void {
    server.registerTool('get_vscode_info', { description: 'Get information about the current VS Code instance' }, async () => {
        logger.debug('Getting VS Code info');
        logger.trace(`VS Code version: ${vscode.version}, app: ${vscode.env.appName}`);
        return makeTextResult({
            version: vscode.version,
            appName: vscode.env.appName,
            appRoot: vscode.env.appRoot,
            language: vscode.env.language,
            machineId: vscode.env.machineId,
            sessionId: vscode.env.sessionId,
            uriScheme: vscode.env.uriScheme,
            shell: vscode.env.shell,
        });
    });
}
```

---

### Pattern 7: Copilot Tool Interface and Invocation
**Where:** `extensions/copilot/src/extension/tools/node/readFileTool.tsx:19-199`

**What:** Copilot tools implement `ICopilotTool<T>` with generic input parameters. The `invoke()` method receives options including input parameters, tokenization budget, and cancellation token. Tools return `LanguageModelToolResult` containing prompt elements (TSX, text, markdown).

```typescript
export class ReadFileTool implements ICopilotTool<ReadFileParams> {
    public static readonly toolName = ToolName.ReadFile;
    public static readonly nonDeferred = true;

    constructor(
        @IWorkspaceService private readonly workspaceService: IWorkspaceService,
        @INotebookService private readonly notebookService: INotebookService,
        @IAlternativeNotebookContentService private readonly alternativeNotebookContent: IAlternativeNotebookContentService,
        @IPromptPathRepresentationService private readonly promptPathRepresentationService: IPromptPathRepresentationService,
        @IInstantiationService private readonly instantiationService: IInstantiationService,
        @IEndpointProvider private readonly endpointProvider: IEndpointProvider,
        @ITelemetryService private readonly telemetryService: ITelemetryService,
        @IConfigurationService private readonly configurationService: IConfigurationService,
        @IExperimentationService private readonly experimentationService: IExperimentationService,
        @ICustomInstructionsService private readonly customInstructionsService: ICustomInstructionsService,
        @IFileSystemService private readonly fileSystemService: IFileSystemService,
        @IExtensionsService private readonly extensionsService: IExtensionsService,
    ) { }

    async invoke(options: vscode.LanguageModelToolInvocationOptions<ReadFileParams>, token: vscode.CancellationToken) {
        let ranges: IParamRanges | undefined;
        let uri: URI | undefined;
        try {
            uri = resolveToolInputPath(options.input.filePath, this.promptPathRepresentationService);
            
            const binary = await hexdumpIfBinary(this.fileSystemService, uri);
            if (binary) {
                // Handle binary files
                return new LanguageModelToolResult([
                    new LanguageModelPromptTsxPart(
                        await renderPromptElementJSON(...)
                    )
                ]);
            }
            
            const documentSnapshot = await this.getSnapshot(uri);
            ranges = getParamRanges(options.input, documentSnapshot);
            
            return new LanguageModelToolResult([
                new LanguageModelPromptTsxPart(
                    await renderPromptElementJSON(
                        this.instantiationService,
                        ReadFileResult,
                        { uri, startLine: ranges.start, endLine: ranges.end, truncated: ranges.truncated, snapshot: documentSnapshot, languageModel: this._promptContext?.request?.model, useCodeFences },
                        options.tokenizationOptions ?? {
                            tokenBudget: 600,
                            countTokens: (t) => Promise.resolve(t.length * 3 / 4)
                        },
                        token,
                    ),
                )
            ]);
        } catch (err) {
            // Error handling
        }
    }
}

ToolRegistry.registerTool(ReadFileTool);
```

---

## Summary

The VS Code Copilot extension implements core IDE chat and tool functionality through:

1. **Chat Participants**: Created via `vscode.chat.createChatParticipant()` with scheme IDs and async handler functions, supporting customization and session management providers
2. **Language Model Registration**: Models register via `vscode.lm.registerLanguageModelChatProvider()`, providing metadata and integration points
3. **Tool Registry Pattern**: Internal tool registry collects tool constructors and tracks non-deferred tools; tools are then exposed to VS Code and MCP
4. **Dual Tool APIs**: Both VS Code's `vscode.lm.registerTool()` (for standard and model-specific tools) and MCP's `server.registerTool()` are used
5. **Tool Implementation**: Tools implement `ICopilotTool<T>` interface with dependency injection, input validation, and structured result generation using prompt TSX elements
6. **Request Lifecycle**: Chat handlers manage request context, model switching, rate limiting, telemetry, and yield management across turns

The architecture separates concerns: registration/provisioning (Models, Participants), tool definitions and routing (ToolRegistry), and execution (specific tool classes). This enables flexible composition of chat agents with multiple tools across different model backends (Claude, Copilot, BYOK).

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
