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
