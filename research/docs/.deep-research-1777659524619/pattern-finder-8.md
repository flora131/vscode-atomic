# VS Code TypeScript Language Features: Core Patterns for Tauri/Rust Porting

## Research Focus
Analyzed patterns in `extensions/typescript-language-features/` (168 files, 22,571 LOC) to understand how VS Code orchestrates language intelligence (completions, hover, diagnostics, refactoring, code actions).

---

## Pattern 1: Language Provider Registration Model
**Where:** `src/languageProvider.ts:64-100`
**What:** Lazy-loaded plugin registration system that dynamically imports and registers 25+ language feature providers on client readiness.

```typescript
private async registerProviders(): Promise<void> {
    const selector = this.documentSelector;
    const cachedNavTreeResponse = new CachedResponse();
    
    await Promise.all([
        import('./languageFeatures/callHierarchy').then(provider => this._register(provider.register(selector, this.client))),
        import('./languageFeatures/completions').then(provider => this._register(provider.register(selector, this.description, this.client, this.typingsStatus, this.fileConfigurationManager, this.commandManager, this.telemetryReporter, this.onCompletionAccepted))),
        import('./languageFeatures/hover').then(provider => this._register(provider.register(selector, this.client, this.fileConfigurationManager))),
        // ... 22 more feature imports
    ]);
}
```

**Variations / call-sites:**
- `src/typeScriptServiceClientHost.ts:51-100` - Creates LanguageProvider instances per language
- Each provider module exports a `register()` function returning `vscode.Disposable`

---

## Pattern 2: Provider Implementation Interface
**Where:** `src/languageFeatures/hover.ts:17-69`
**What:** Language feature providers implement VS Code's interface (e.g., `vscode.HoverProvider`) and delegate to the TypeScript service client.

```typescript
class TypeScriptHoverProvider implements vscode.HoverProvider {
    public constructor(
        private readonly client: ITypeScriptServiceClient,
        private readonly fileConfigurationManager: FileConfigurationManager,
    ) { }

    public async provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken,
        context?: vscode.HoverContext,
    ): Promise<vscode.VerboseHover | undefined> {
        const filepath = this.client.toOpenTsFilePath(document);
        if (!filepath) { return undefined; }
        
        const args = { ...typeConverters.Position.toFileLocationRequestArgs(filepath, position), verbosityLevel };
        const response = await this.client.interruptGetErr(async () => {
            await this.fileConfigurationManager.ensureConfigurationForDocument(document, token);
            return this.client.execute('quickinfo', args, token);
        });
    }
}
```

**Variations / call-sites:**
- `src/languageFeatures/completions.ts:682-750` - CompletionItemProvider
- `src/languageFeatures/definitionProviderBase.ts` - DefinitionProvider
- `src/languageFeatures/quickFix.ts:185` - CodeActionProvider
- Pattern used for: 25+ provider types (diagnostics, references, rename, refactor, symbols, etc.)

---

## Pattern 3: Command-Based Code Actions
**Where:** `src/languageFeatures/quickFix.ts:30-58`
**What:** Code action execution wrapped in registered command objects with IDs, enabling undo/redo and command palette.

```typescript
class ApplyCodeActionCommand implements Command {
    public static readonly ID = '_typescript.applyCodeActionCommand';
    public readonly id = ApplyCodeActionCommand.ID;

    constructor(
        private readonly client: ITypeScriptServiceClient,
        private readonly diagnosticManager: DiagnosticsManager,
        private readonly telemetryReporter: TelemetryReporter,
    ) { }

    public async execute({ document, action, diagnostic, followupAction }: ApplyCodeActionCommand_args): Promise<boolean> {
        this.telemetryReporter.logTelemetry('quickFix.execute', { fixName: action.fixName });
        this.diagnosticManager.deleteDiagnostic(document.uri, diagnostic);
        const codeActionResult = await applyCodeActionCommands(this.client, action.commands, nulToken);
        await followupAction?.execute();
        return codeActionResult;
    }
}
```

**Variations / call-sites:**
- `src/languageFeatures/refactor.ts:47-70` - DidApplyRefactoringCommand, SelectRefactorCommand
- `src/commands/` directory - 10+ command implementations (restart server, open logs, select version, etc.)
- Registered via `CommandManager` which wraps `vscode.commands.registerCommand`

---

## Pattern 4: TypeScript Server Request/Response Protocol
**Where:** `src/typescriptService.ts:38-99`
**What:** Typed request-response mapping using TypeScript mapped types to track which args produce which response types.

```typescript
interface StandardTsServerRequests {
    'applyCodeActionCommand': [Proto.ApplyCodeActionCommandRequestArgs, Proto.ApplyCodeActionCommandResponse];
    'completionEntryDetails': [Proto.CompletionDetailsRequestArgs, Proto.CompletionDetailsResponse];
    'completionInfo': [Proto.CompletionsRequestArgs, Proto.CompletionInfoResponse];
    'definition': [Proto.FileLocationRequestArgs, Proto.DefinitionResponse];
    'quickinfo': [Proto.FileLocationRequestArgs, Proto.QuickInfoResponse];
    'getCodeFixes': [Proto.CodeFixRequestArgs, Proto.CodeFixResponse];
    'getApplicableRefactors': [Proto.GetApplicableRefactorsRequestArgs, Proto.GetApplicableRefactorsResponse];
    // ... 60+ more requests
}

interface NoResponseTsServerRequests {
    'open': [Proto.OpenRequestArgs, null];
    'close': [Proto.FileRequestArgs, null];
    'change': [Proto.ChangeRequestArgs, null];
}

export type TypeScriptRequests = StandardTsServerRequests & NoResponseTsServerRequests & AsyncTsServerRequests;
```

**Variations / call-sites:**
- All `execute()` calls validated against this: `execute<K extends keyof TypeScriptRequests>(command: K, args: TypeScriptRequests[K][0])`
- Proto types defined in `src/tsServer/protocol/protocol.ts` (~4000+ LOC)

---

## Pattern 5: Service Client as Central Hub
**Where:** `src/typescriptServiceClient.ts:108-250`
**What:** Single service client orchestrates server lifecycle (spawn, crash recovery, restart), buffer synchronization, diagnostics routing, and request execution.

```typescript
export default class TypeScriptServiceClient extends Disposable implements ITypeScriptServiceClient {
    private readonly bufferSyncSupport: BufferSyncSupport;
    private readonly diagnosticsManager: DiagnosticsManager;
    private readonly pluginManager: PluginManager;
    private serverState: ServerState.State = ServerState.None;

    public execute(command: keyof TypeScriptRequests, args: unknown, token: vscode.CancellationToken, config?: ExecConfig): Promise<ServerResponse.Response<Proto.Response>> {
        let executions = this.executeImpl(command, args, {
            isAsync: false,
            token,
            expectsResult: true,
            ...config,
        });
        
        if (config?.nonRecoverable) {
            executions[0]!.catch(err => this.fatalError(command, err));
        }
        return executions[0]!;
    }
    
    public interruptGetErr<R>(f: () => R): R {
        return this.bufferSyncSupport.interruptGetErr(f);
    }
}
```

**Variations / call-sites:**
- `src/typeScriptServiceClientHost.ts:51-100` - Wraps the client in per-language providers
- Server lifecycle: spawn → running → crash → restart (with backoff)
- Handles multi-process (main/syntax/semantic/diagnostics servers)

---

## Pattern 6: Request Queueing with Priority Levels
**Where:** `src/tsServer/requestQueue.ts:7-57`
**What:** Prioritized queue allowing diagnostic requests to jump ahead of low-priority operations while maintaining ordering fences.

```typescript
export enum RequestQueueingType {
    Normal = 1,           // Executed in order
    LowPriority = 2,      // Normal requests jump in front
    Fence = 3,            // Blocks reordering
}

export class RequestQueue {
    private readonly queue: RequestItem[] = [];
    
    public enqueue(item: RequestItem): void {
        if (item.queueingType === RequestQueueingType.Normal) {
            let index = this.queue.length - 1;
            while (index >= 0) {
                if (this.queue[index].queueingType !== RequestQueueingType.LowPriority) {
                    break;
                }
                --index;
            }
            this.queue.splice(index + 1, 0, item);
        } else {
            this.queue.push(item);
        }
    }
}
```

**Variations / call-sites:**
- `src/tsServer/server.ts:228-260` - Single/Multi server execute implementations
- Used by `SingleTsServer.executeImpl()` to manage ~60+ concurrent request types

---

## Pattern 7: Diagnostic Management with Caching
**Where:** `src/languageFeatures/diagnostics.ts:34-72`
**What:** Separate diagnostic kinds (Syntax, Semantic, Suggestion, RegionSemantic) cached per file with invalidation.

```typescript
export const enum DiagnosticKind {
    Syntax,
    Semantic,
    Suggestion,
    RegionSemantic,
}

class FileDiagnostics {
    private readonly _diagnostics = new Map<DiagnosticKind, ReadonlyArray<vscode.Diagnostic>>();

    public updateDiagnostics(
        language: DiagnosticLanguage,
        kind: DiagnosticKind,
        diagnostics: ReadonlyArray<vscode.Diagnostic>,
        ranges: ReadonlyArray<vscode.Range> | undefined
    ): boolean {
        if (language !== this.language) {
            this._diagnostics.clear();
            this.language = language;
        }
        
        const existing = this._diagnostics.get(kind);
        if (existing?.length === 0 && diagnostics.length === 0) {
            return false;  // No update needed
        }
        
        this._diagnostics.set(kind, diagnostics);
        return true;
    }
}
```

**Variations / call-sites:**
- DiagnosticsManager maintains map of files → FileDiagnostics
- Connected to server events via `onEvent: vscode.Event<Proto.Event>`
- Separate background diagnostics server for large projects

---

## Pattern 8: Position/Range Type Conversion
**Where:** `src/typeConverters.ts:15-68`
**What:** Namespace pattern for bidirectional conversion between VS Code (0-based) and TS Server (1-based) coordinates.

```typescript
export namespace Range {
    export const fromTextSpan = (span: Proto.TextSpan): vscode.Range =>
        fromLocations(span.start, span.end);

    export const toTextSpan = (range: vscode.Range): Proto.TextSpan => ({
        start: Position.toLocation(range.start),
        end: Position.toLocation(range.end)
    });
}

export namespace Position {
    export const fromLocation = (tslocation: Proto.Location): vscode.Position =>
        new vscode.Position(tslocation.line - 1, tslocation.offset - 1);

    export const toLocation = (vsPosition: vscode.Position): Proto.Location => ({
        line: vsPosition.line + 1,
        offset: vsPosition.character + 1,
    });

    export const toFileLocationRequestArgs = (file: string, position: vscode.Position): Proto.FileLocationRequestArgs => ({
        file,
        line: position.line + 1,
        offset: position.character + 1,
    });
}
```

**Variations / call-sites:**
- Used by every language feature provider
- Repeated for Range, Location, TextEdit, CodeAction, SymbolKind, etc.
- Core abstraction for IDE ↔ Language Server communication

---

## Architecture Summary

The TypeScript language features extension demonstrates a **multi-layered architecture**:

1. **Registration Layer** (`languageProvider.ts`): Lazy loads 25+ feature modules, each exporting a `register()` function
2. **Provider Layer** (`languageFeatures/*`): 25+ provider classes implementing VS Code interfaces, delegating to client
3. **Service Layer** (`typescriptServiceClient.ts`): Central hub managing server lifecycle, request routing, diagnostics
4. **Protocol Layer** (`typescriptService.ts`, `tsServer/protocol/`): Typed request-response mapping with 60+ command types
5. **Transport Layer** (`tsServer/server.ts`, `tsServer/requestQueue.ts`): Process management, prioritized queueing, response handling
6. **Conversion Layer** (`typeConverters.ts`): Bidirectional coordinate/type conversion between IDEs

**Key Design Patterns:**
- **Plugin-based** via lazy-loaded feature modules
- **Event-driven** for diagnostics, server state changes, file watching
- **Typed requests** with compile-time checking via mapped TypeScript types
- **Prioritized async queue** with fence-based ordering
- **Crash recovery** with exponential backoff and user prompts
- **Multi-process** server deployment (syntax/semantic/diagnostics isolation)
- **Command pattern** for undo/redo-able operations
- **Bi-directional type conversion** for VS Code ↔ TS Server coordinates

---

**Files Analyzed:** 168 files total
- Core clients: `typescriptServiceClient.ts`, `typeScriptServiceClientHost.ts`
- 25+ language features: `src/languageFeatures/*.ts`
- Server communication: `src/tsServer/server.ts`, `requestQueue.ts`, `protocol/*.ts`
- Command infrastructure: `src/commands/*.ts`
- Diagnostic pipeline: `src/languageFeatures/diagnostics.ts`
- Type conversion: `src/typeConverters.ts`

