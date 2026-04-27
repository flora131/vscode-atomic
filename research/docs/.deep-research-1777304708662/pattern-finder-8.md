# Pattern Analysis: TypeScript Language Features Extension (Partition 8/79)
## LSP Integration via tsserver in VS Code

### Research Question
What patterns exist for porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust?

**Focus Area:** `extensions/typescript-language-features/` — 168 files, 22,571 LOC
**Scope:** LSP-ish integration via tsserver, provider registration, and request handling

---

## Discovered Patterns

#### Pattern 1: Hierarchical Provider Registration with Lazy Loading
**Where:** `extensions/typescript-language-features/src/languageProvider.ts:64-100`
**What:** Each language feature is registered dynamically via Promise.all with conditional lazy imports, centered on documentSelector (semantic vs syntax).

```typescript
private async registerProviders(): Promise<void> {
	const selector = this.documentSelector;
	const cachedNavTreeResponse = new CachedResponse();

	await Promise.all([
		import('./languageFeatures/callHierarchy').then(provider => 
			this._register(provider.register(selector, this.client))),
		import('./languageFeatures/codeLens/implementationsCodeLens').then(provider => 
			this._register(provider.register(selector, this.description, this.client, cachedNavTreeResponse))),
		import('./languageFeatures/definitions').then(provider => 
			this._register(provider.register(selector, this.client))),
		// ... 25+ more providers
	]);
}
```

**Variations / call-sites:**
- `extensions/typescript-language-features/src/languageFeatures/definitions.ts:63` — Definition provider registration
- `extensions/typescript-language-features/src/languageFeatures/hover.ts:105` — Hover provider registration
- `extensions/typescript-language-features/src/languageFeatures/completions.ts:929` — Completion provider registration
- All 32 provider modules follow the same `export function register(selector, client, ...)` pattern

---

#### Pattern 2: Conditional Provider Registration Based on Server Capabilities
**Where:** `extensions/typescript-language-features/src/languageFeatures/util/dependentRegistration.ts:75-81`
**What:** A Condition-based system enables/disables provider registration dynamically when server capabilities or configurations change.

```typescript
export function conditionalRegistration(
	conditions: readonly Condition[],
	doRegister: () => vscode.Disposable,
	elseDoRegister?: () => vscode.Disposable
): vscode.Disposable {
	return new ConditionalRegistration(conditions, doRegister, elseDoRegister);
}

// Used in definitions.ts:67-72:
export function register(selector: DocumentSelector, client: ITypeScriptServiceClient) {
	return conditionalRegistration([
		requireSomeCapability(client, ClientCapability.EnhancedSyntax, ClientCapability.Semantic),
	], () => {
		return vscode.languages.registerDefinitionProvider(selector.syntax,
			new TypeScriptDefinitionProvider(client));
	});
}
```

**Variations / call-sites:**
- `extensions/typescript-language-features/src/languageFeatures/hover.ts:110` — Requires EnhancedSyntax or Semantic
- `extensions/typescript-language-features/src/languageFeatures/completions.ts:942` — Requires Semantic
- `extensions/typescript-language-features/src/languageFeatures/implementations.ts:25` — Requires Semantic

---

#### Pattern 3: Client.execute() for Async tsserver Commands with Token Cancellation
**Where:** `extensions/typescript-language-features/src/languageFeatures/definitions.ts:17-60`
**What:** Providers use `client.execute(command, args, cancellationToken)` to send typed requests to tsserver, handling response.type checking and conversion.

```typescript
public async provideDefinition(
	document: vscode.TextDocument,
	position: vscode.Position,
	token: vscode.CancellationToken
): Promise<vscode.DefinitionLink[] | vscode.Definition | undefined> {
	const filepath = this.client.toOpenTsFilePath(document);
	if (!filepath) {
		return undefined;
	}

	const args = typeConverters.Position.toFileLocationRequestArgs(filepath, position);
	const response = await this.client.execute('definitionAndBoundSpan', args, token);
	if (response.type !== 'response' || !response.body) {
		return undefined;
	}

	return response.body.definitions.map((location): vscode.DefinitionLink => {
		const target = typeConverters.Location.fromTextSpan(this.client.toResource(location.file), location);
		return {
			originSelectionRange: span,
			targetRange: typeConverters.Range.fromLocations(location.contextStart, location.contextEnd),
			targetUri: target.uri,
			targetSelectionRange: target.range,
		};
	});
}
```

**Variations / call-sites:**
- `extensions/typescript-language-features/src/languageFeatures/hover.ts:45` — quickinfo request
- `extensions/typescript-language-features/src/languageFeatures/completions.ts:761` — completionInfo request
- `extensions/typescript-language-features/src/languageFeatures/rename.ts:136` — rename request
- 40+ call-sites across all language features

---

#### Pattern 4: interruptGetErr() Pattern for Prioritizing User-Facing Requests
**Where:** `extensions/typescript-language-features/src/languageFeatures/hover.ts:42-46`
**What:** Client.interruptGetErr wraps requests that should interrupt background diagnostic checks, prioritizing interactive user actions.

```typescript
const response = await this.client.interruptGetErr(async () => {
	await this.fileConfigurationManager.ensureConfigurationForDocument(document, token);
	return this.client.execute('quickinfo', args, token);
});
```

**Variations / call-sites:**
- `extensions/typescript-language-features/src/languageFeatures/completions.ts:192` — completionEntryDetails
- `extensions/typescript-language-features/src/languageFeatures/completions.ts:761` — completionInfo
- `extensions/typescript-language-features/src/languageFeatures/copyPaste.ts:102` — preparePasteEdits
- `extensions/typescript-language-features/src/languageFeatures/refactor.ts:599` — interruptGetErrIfNeeded wrapper
- 15+ interactive provider call-sites

---

#### Pattern 5: Cached Responses for Repeated Calls on Same Document
**Where:** `extensions/typescript-language-features/src/tsServer/cachedResponse.ts:15-48`
**What:** CachedResponse caches promise-based tsserver responses keyed by document URI and version, reusing results until the document changes.

```typescript
export class CachedResponse<T extends Proto.Response> {
	private response?: Promise<ServerResponse.Response<T>>;
	private version: number = -1;
	private document: string = '';

	public execute(
		document: vscode.TextDocument,
		resolve: Resolve<T>
	): Promise<ServerResponse.Response<T>> {
		if (this.response && this.matches(document)) {
			return this.response = this.response.then(result => 
				result.type === 'cancelled' ? resolve() : result);
		}
		return this.reset(document, resolve);
	}

	private matches(document: vscode.TextDocument): boolean {
		return this.version === document.version && this.document === document.uri.toString();
	}
}
```

**Variations / call-sites:**
- `extensions/typescript-language-features/src/languageProvider.ts:67` — Shared instance for codeLens providers
- `extensions/typescript-language-features/src/languageFeatures/documentSymbol.ts:52` — navto caching
- `extensions/typescript-language-features/src/languageFeatures/refactor.ts:540` — getApplicableRefactors caching

---

#### Pattern 6: Browser/Worker-Based tsserver via MessagePort
**Where:** `extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts:61-187`
**What:** Web worker implementation of TsServerProcess uses MessagePorts for three channels: tsserver (sync), watcher (file events), and syncFs (filesystem).

```typescript
class WorkerServerProcess implements TsServerProcess {
	private readonly _tsserver: MessagePort;
	private readonly _watcher: MessagePort;
	private readonly _syncFs: MessagePort;

	constructor(...) {
		const tsserverChannel = new MessageChannel();
		const watcherChannel = new MessageChannel();
		const syncChannel = new MessageChannel();
		this._tsserver = tsserverChannel.port2;
		this._watcher = watcherChannel.port2;
		this._syncFs = syncChannel.port2;

		this._tsserver.onmessage = (event) => {
			for (const handler of this._onDataHandlers) {
				handler(event.data);
			}
		};

		this._worker.postMessage(
			{ args, extensionUri },
			[syncChannel.port1, tsserverChannel.port1, watcherChannel.port1]
		);
	}

	write(serverRequest: Proto.Request): void {
		this._tsserver.postMessage(serverRequest);
	}
}
```

**Variations / call-sites:**
- `extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts:100-127` — Message dispatch for three channels
- `extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts:147-154` — Worker initialization with ApiService

---

#### Pattern 7: Request Queue with Priority Levels and Reordering
**Where:** `extensions/typescript-language-features/src/tsServer/requestQueue.ts:35-97`
**What:** RequestQueue implements request prioritization: Normal requests can jump ahead of LowPriority requests, but Fence requests block all reordering.

```typescript
export enum RequestQueueingType {
	Normal = 1,        // Can reorder
	LowPriority = 2,   // Gets pushed behind Normal
	Fence = 3,         // Blocks reordering, goes to end
}

export class RequestQueue {
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
- `extensions/typescript-language-features/src/tsServer/server.ts:91-92` — SingleTsServer uses RequestQueue
- `extensions/typescript-language-features/src/tsServer/cancellation.ts` — OngoingRequestCanceller integrates with queue

---

## Port Feasibility Analysis

### Direct Transferable Patterns
1. **Provider registration hierarchy** — Directly portable: Provider registration with capability checks is language-agnostic
2. **Conditional registration based on capabilities** — Portable: The Condition-based system has no platform dependencies
3. **Request/response command pattern** — Portable: Execute-with-args-and-cancellation is standard LSP
4. **Cached responses** — Portable: Document version tracking is implementation-agnostic
5. **Request queue prioritization** — Portable: The algorithm is pure data structure logic

### Architecture Dependencies Requiring Redesign
- **Worker-based tsserver communication** — Tauri would use native plugins/IPC instead of Web Workers
- **interruptGetErr diagnostics interruption** — Would need Rust-side diagnostic manager redesign
- **ConditionalRegistration with vscode events** — Would need equivalent capability change event system in Tauri API

### LSP-Specific Requirements for Porting
- tsserver protocol definitions must map to LSP equivalents or be reimplemented in Rust
- 32 provider types (definition, completion, hover, etc.) require LSP server implementations
- DocumentSelector filtering by language/scheme needs Tauri equivalent
- Cancellation tokens require async/await cancellation handling in Rust

---

## Summary

The TypeScript Language Features extension demonstrates a **modular, capability-driven architecture** for IDE features:

- **Modular providers**: 32 independent feature modules with a standard registration interface
- **Capability-based delivery**: Features are conditionally available based on server capabilities
- **Priority-aware request handling**: Three-tier queue system ensures interactive features stay responsive
- **Response caching**: Document-version-keyed caching reduces redundant computations
- **Worker isolation**: Browser/web deployment uses MessagePort channels for three concurrent data flows

The core request/response and provider registration patterns are highly portable to Rust + Tauri, but the diagnostic interruption strategy, worker communication model, and event-driven capability system would require substantial redesign. The LSP protocol itself provides a migration path, though VS Code's extended protocol features (like getCombinedCodeFix) may not have direct equivalents.

