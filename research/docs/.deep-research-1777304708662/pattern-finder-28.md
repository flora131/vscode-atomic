# Partition 28: CSS Language Features Extension — LSP Porting Patterns

## Overview

The CSS language features extension (`extensions/css-language-features/`) provides a complete LSP implementation split between client (TypeScript/Electron) and server (Node.js process). This partition is critical for understanding how VS Code bridges IDE and LSP protocols, handles process communication, manages language-specific data, and synchronizes settings.

**Key dimensions for Tauri/Rust porting:**

1. **Client-Server Communication**: IPC transport over stdout/stderr
2. **LanguageClient Construction**: Parametric factory pattern enabling runtime selection
3. **Middleware Layer**: Client-side request/response transformation
4. **Custom Notifications**: Beyond-LSP communication (custom data sync)
5. **Settings Synchronization**: Two-channel pattern (client-side + document-scoped)
6. **Runtime Abstraction**: Platform-specific fs operations via RequestService interface
7. **Dual-Target Deployment**: Node.js server vs. browser worker server

---

## Patterns

#### Pattern 1: LanguageClient Constructor as Dependency Injection

**Where:** `extensions/css-language-features/client/src/cssClient.ts:15`

**What:** Abstract LanguageClient creation into a factory function to decouple client activation from implementation (Node.js IPC vs. browser Worker).

```typescript
export type LanguageClientConstructor = (name: string, description: string, clientOptions: LanguageClientOptions) => BaseLanguageClient;

export async function startClient(context: ExtensionContext, newLanguageClient: LanguageClientConstructor, runtime: Runtime): Promise<BaseLanguageClient> {
	const client = newLanguageClient('css', l10n.t('CSS Language Server'), clientOptions);
	client.registerProposedFeatures();
	await client.start();
	// ...
}
```

**Variations / call-sites:**

- **Node variant** (`cssClientMain.ts:31-33`): Factory returns `new LanguageClient(id, name, serverOptions, clientOptions)` with IPC transport.
- **Browser variant** (`cssClientMain.ts:21-23`): Factory returns `new LanguageClient(id, name, worker, clientOptions)` with Worker message transport.

**Porting implication:** Tauri/Rust port must support both local LSP subprocess (Rust-based) and WASM worker (browser). Use trait objects or enums to abstract over transport.

---

#### Pattern 2: ServerOptions with Transport Kind and Debug Configuration

**Where:** `extensions/css-language-features/client/src/node/cssClientMain.ts:26-29`

**What:** Dual-mode server configuration (run vs. debug) with explicit transport kind (IPC) and isolated debug port assignment.

```typescript
const debugOptions = { execArgv: ['--nolazy', '--inspect=' + (7000 + Math.round(Math.random() * 999))] };

const serverOptions: ServerOptions = {
	run: { module: serverModule, transport: TransportKind.ipc },
	debug: { module: serverModule, transport: TransportKind.ipc, options: debugOptions }
};
```

**Variations / call-sites:**

- `BaseLanguageClient` accepts `ServerOptions` dict with `run` and `debug` keys, enabling hot-swap based on activation context.

**Porting implication:** Tauri/Rust subprocess spawn needs equivalent run/debug modes. Consider environment variables (`RUST_LOG`, `--backtrace`) and port allocation for debugger attachment (e.g., via `--listen` for Rust debugger).

---

#### Pattern 3: DocumentSelector and Multi-Language Support

**Where:** `extensions/css-language-features/client/src/cssClient.ts:43-51`

**What:** Static array of language IDs drives document selector, initialization options, and formatter registrations uniformly.

```typescript
const documentSelector = ['css', 'scss', 'less'];

const clientOptions: LanguageClientOptions = {
	documentSelector,
	synchronize: {
		configurationSection: ['css', 'scss', 'less']
	},
	initializationOptions: {
		handledSchemas: ['file'],
		provideFormatter: false,
		customCapabilities: { rangeFormatting: { editLimit: 10000 } }
	}
};
```

**Variations / call-sites:**

- `formatterRegistrations` array built from `documentSelector` (line 45-47), enabling per-language enable/disable.
- Configuration sync maps section names directly (e.g., `css.format.enable`).

**Porting implication:** LSP `DocumentSelector` must be declarative and extensible. Server-side language service registry (`cssServer.ts:62`) mirrors client-side declarations. Tauri/Rust port needs same bidirectional mapping.

---

#### Pattern 4: Middleware for Completion Item Post-Processing

**Where:** `extensions/css-language-features/client/src/cssClient.ts:60-96`

**What:** Client-side middleware intercepts LSP response stream to apply client-specific transformations (completion item ranges, color labels) before returning to editor.

```typescript
middleware: {
	provideCompletionItem(document: TextDocument, position: Position, context: CompletionContext, token: CancellationToken, next: ProvideCompletionItemsSignature): ProviderResult<CompletionItem[] | CompletionList> {
		const r = next(document, position, context, token);
		if (isThenable<CompletionItem[] | CompletionList | null | undefined>(r)) {
			return r.then(updateProposals);
		}
		return updateProposals(r);
	}
}
```

**Variations / call-sites:**

- `updateRanges`: Converts range to insert/replace pair (VS Code's new completion mode).
- `updateLabel`: Adds color descriptions from documentation (accessibility).
- Middleware chain allows multiple handlers; executed sequentially before client.start().

**Porting implication:** Tauri/Rust client needs post-processing hooks on LSP response stream. This is not LSP spec but client-side convenience layer. Can be implemented as interceptor in transport layer or separate middleware queue.

---

#### Pattern 5: Custom Notifications Beyond LSP Spec

**Where:** `extensions/css-language-features/client/src/cssClient.ts:11-13, 105-108`

**What:** Extend LSP with custom notification type for out-of-band data sync (CSS custom data paths) without blocking main request/response cycle.

```typescript
namespace CustomDataChangedNotification {
	export const type: NotificationType<string[]> = new NotificationType('css/customDataChanged');
}

// Client sends on startup and on custom data source change
client.sendNotification(CustomDataChangedNotification.type, customDataSource.uris);
customDataSource.onDidChange(() => {
	client.sendNotification(CustomDataChangedNotification.type, customDataSource.uris);
});

// Server receives (cssServer.ts:380)
connection.onNotification(CustomDataChangedNotification.type, updateDataProviders);
```

**Variations / call-sites:**

- Server-side receiver (`cssServer.ts:190-196`): Refetches data providers asynchronously; existing requests wait on `dataProvidersReady` promise.
- Custom data paths sourced from workspaces and extensions; change events trigger updates.

**Porting implication:** Tauri/Rust LSP must support custom notification types beyond standard LSP spec. Design custom message envelope (namespace + type string) compatible with JSON-RPC 2.0 over IPC.

---

#### Pattern 6: RuntimeEnvironment Abstraction for Platform I/O

**Where:** `extensions/css-language-features/server/src/cssServer.ts:28-35`

**What:** Abstract platform-specific I/O (timers, fs) behind interface; pass to server to avoid direct Node.js/browser module imports.

```typescript
export interface RuntimeEnvironment {
	readonly file?: RequestService;
	readonly http?: RequestService;
	readonly timer: {
		setImmediate(callback: (...args: any[]) => void, ...args: any[]): Disposable;
		setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable;
	};
}
```

**Variations / call-sites:**

- **Node variant** (`cssServerNodeMain.ts:18`): Not shown; fs/http from Node, timers are native.
- **Browser variant** (`cssServerMain.ts:17-28`): Browser-safe timers via setTimeout (no setImmediate natively); no file system access.
- Server uses `runtime.timer.setTimeout` (validation.ts:46) instead of `setTimeout` directly.

**Porting implication:** Rust server must not directly use platform APIs. Inject abstraction for timers, file I/O, and HTTP. Critical for cross-platform (native + WASM) deployment.

---

#### Pattern 7: RequestService Dual-Handler Pattern (Client-Bound + Server-Initiated)

**Where:** `extensions/css-language-features/server/src/requests.ts:66-99`

**What:** Server checks for built-in handlers (file, http protocols); falls back to client RPC for custom schemes. Prevents round-trip for local fs access.

```typescript
export function getRequestService(handledSchemas: string[], connection: Connection, runtime: RuntimeEnvironment): RequestService {
	const builtInHandlers: { [protocol: string]: RequestService | undefined } = {};
	for (const protocol of handledSchemas) {
		if (protocol === 'file') {
			builtInHandlers[protocol] = runtime.file;
		} else if (protocol === 'http' || protocol === 'https') {
			builtInHandlers[protocol] = runtime.http;
		}
	}
	return {
		async stat(uri: string): Promise<FileStat> {
			const handler = builtInHandlers[getScheme(uri)];
			if (handler) {
				return handler.stat(uri);
			}
			const res = await connection.sendRequest(FsStatRequest.type, uri.toString());
			return res;
		},
		// ...
	};
}
```

**Variations / call-sites:**

- Client supplies handler list via `initializationOptions.handledSchemas` (cssClient.ts:56).
- Client-side handler (requests.ts:21-45): On RPC request, delegates to `workspace.fs` or `runtime.fs`.
- Three custom request types: `fs/content`, `fs/stat`, `fs/readDir` (parity with vscode-uri types).

**Porting implication:** Tauri/Rust server must maintain two I/O paths: local (via injected runtime) and remote (via custom LSP requests). Critical for plugin isolation and permission boundaries.

---

#### Pattern 8: LanguageModelCache with LRU Eviction

**Where:** `extensions/css-language-features/server/src/languageModelCache.ts:14-82`

**What:** Generic cache for parsed documents with configurable max entries, cleanup interval, and version tracking. Enables lazy parsing and memory bounds.

```typescript
export function getLanguageModelCache<T>(maxEntries: number, cleanupIntervalTimeInSec: number, parse: (document: TextDocument) => T): LanguageModelCache<T> {
	let languageModels: { [uri: string]: { version: number; languageId: string; cTime: number; languageModel: T } } = {};
	let nModels = 0;

	let cleanupInterval: NodeJS.Timeout | undefined = undefined;
	if (cleanupIntervalTimeInSec > 0) {
		cleanupInterval = setInterval(() => {
			const cutoffTime = Date.now() - cleanupIntervalTimeInSec * 1000;
			const uris = Object.keys(languageModels);
			for (const uri of uris) {
				const languageModelInfo = languageModels[uri];
				if (languageModelInfo.cTime < cutoffTime) {
					delete languageModels[uri];
					nModels--;
				}
			}
		}, cleanupIntervalTimeInSec * 1000);
	}

	return {
		get(document: TextDocument): T {
			const version = document.version;
			const languageModelInfo = languageModels[document.uri];
			if (languageModelInfo && languageModelInfo.version === version && languageModelInfo.languageId === languageId) {
				languageModelInfo.cTime = Date.now();
				return languageModelInfo.languageModel;
			}
			const languageModel = parse(document);
			languageModels[document.uri] = { languageModel, version, languageId, cTime: Date.now() };
			if (nModels === maxEntries) {
				let oldestTime = Number.MAX_VALUE;
				let oldestUri = null;
				for (const uri in languageModels) {
					const languageModelInfo = languageModels[uri];
					if (languageModelInfo.cTime < oldestTime) {
						oldestUri = uri;
						oldestTime = languageModelInfo.cTime;
					}
				}
				if (oldestUri) {
					delete languageModels[oldestUri];
					nModels--;
				}
			}
			return languageModel;
		}
	};
}
```

**Variations / call-sites:**

- Instantiated in `cssServer.ts:45`: `getLanguageModelCache<Stylesheet>(10, 60, document => getLanguageService(document).parseStylesheet(document))`.
- Documents removed explicitly (cssServer.ts:46-48); cleanup interval in seconds.

**Porting implication:** Tauri/Rust server needs equivalent LRU/time-decay cache. Consider using standard Rust crates (e.g., `lru`, `moka`) or hand-rolled for tight memory control. Version tracking critical for incremental parsing.

---

#### Pattern 9: Settings Synchronization with Scoped Configuration

**Where:** `extensions/css-language-features/server/src/cssServer.ts:155-165`

**What:** Server requests per-document settings via ConfigurationRequest if client supports scoped config; falls back to global if not. Enables workspace-folder-level overrides.

```typescript
function getDocumentSettings(textDocument: TextDocument): Thenable<LanguageSettings | undefined> {
	if (scopedSettingsSupport) {
		let promise = documentSettings[textDocument.uri];
		if (!promise) {
			const configRequestParam = { items: [{ scopeUri: textDocument.uri, section: textDocument.languageId }] };
			promise = connection.sendRequest(ConfigurationRequest.type, configRequestParam).then(s => s[0] as LanguageSettings | undefined);
			documentSettings[textDocument.uri] = promise;
		}
		return promise;
	}
	return Promise.resolve(undefined);
}
```

**Variations / call-sites:**

- Client-side: `synchronize.configurationSection` (cssClient.ts:52-54) declares watched sections.
- Server-side: `connection.onDidChangeConfiguration` (cssServer.ts:169-171) handles bulk sync; resets per-document cache.
- Document settings cached (cssServer.ts:150); cleared on document close (cssServer.ts:152-154).

**Porting implication:** Tauri/Rust server must support dual-mode settings: (1) global broadcast on change, (2) per-document RPC for scoped overrides. Use event listener + request-response.

---

#### Pattern 10: Initialization Options as Capability Negotiation

**Where:** `extensions/css-language-features/server/src/cssServer.ts:69-139`

**What:** Client and server negotiate capabilities during LSP initialize; server reads `params.initializationOptions` to determine behavior (formatter, custom limits, snippets).

```typescript
connection.onInitialize((params: InitializeParams): InitializeResult => {
	const initializationOptions = params.initializationOptions || {};

	// Client capability detection
	const snippetSupport = !!getClientCapability('textDocument.completion.completionItem.snippetSupport', false);
	scopedSettingsSupport = !!getClientCapability('workspace.configuration', false);
	foldingRangeLimit = getClientCapability('textDocument.foldingRange.rangeLimit', Number.MAX_VALUE);

	// Custom capabilities
	formatterMaxNumberOfEdits = initializationOptions?.customCapabilities?.rangeFormatting?.editLimit || Number.MAX_VALUE;

	// Service initialization
	languageServices.css = getCSSLanguageService({ fileSystemProvider: requestService, clientCapabilities: params.capabilities });
	languageServices.scss = getSCSSLanguageService({ fileSystemProvider: requestService, clientCapabilities: params.capabilities });
	languageServices.less = getLESSLanguageService({ fileSystemProvider: requestService, clientCapabilities: params.capabilities });

	// Diagnostic mode (pull vs. push)
	const supportsDiagnosticPull = getClientCapability('textDocument.diagnostic', undefined);
	if (supportsDiagnosticPull === undefined) {
		diagnosticsSupport = registerDiagnosticsPushSupport(documents, connection, runtime, validateTextDocument);
	} else {
		diagnosticsSupport = registerDiagnosticsPullSupport(documents, connection, runtime, validateTextDocument);
	}

	const capabilities: ServerCapabilities = {
		textDocumentSync: TextDocumentSyncKind.Incremental,
		completionProvider: snippetSupport ? { resolveProvider: false, triggerCharacters: ['/', '-', ':'] } : undefined,
		// ...
		documentRangeFormattingProvider: initializationOptions?.provideFormatter === true,
		// ...
	};
	return { capabilities };
});
```

**Variations / call-sites:**

- Client-side initialization (cssClient.ts:50-59): Passes `provideFormatter: false` to suppress server-side formatting, then registers client-side provider (cssClient.ts:156-188).
- Capability helpers (`getClientCapability`) nested in initialize handler; results stored as closure variables.

**Porting implication:** Tauri/Rust LSP must perform full capability negotiation in initialize. Use feature flags or environment variables for custom capabilities. Server-side language services must accept clientCapabilities struct.

---

## Summary for Tauri/Rust Port

**Critical Patterns to Implement:**

1. **Process Communication** (Pattern 2): Replace Node IPC with Rust subprocess + stdio, supporting run/debug modes.
2. **Abstraction Layers** (Patterns 1, 6): Factory patterns + RuntimeEnvironment trait for platform-agnostic I/O.
3. **Bidirectional Messaging** (Patterns 5, 7, 9): Support custom notifications, request-response, and change notifications beyond LSP spec.
4. **Language Service Registry** (Pattern 3, 10): Mirror client document selector in server; negotiate capabilities at init.
5. **Middleware/Hooks** (Pattern 4): Post-process LSP responses client-side; consider interceptor pattern in transport.
6. **Memory Management** (Pattern 8): LRU cache with version tracking for parsed ASTs; time-based cleanup.
7. **Settings Sync** (Pattern 9): Two-channel (global broadcast + per-document RPC); cache invalidation on change.
8. **Custom Data** (Pattern 5): Out-of-band notification sync for extensions/workspace-provided data.

**Key Porting Challenges:**

- TypeScript middleware layer → Rust traits/async closures
- Node.js process spawning + IPC → Rust subprocess crates (e.g., tokio::process)
- Browser Worker dual-target → WASM + native code paths
- Event emitters + callbacks → Rust async/await + channels
- Dynamic configuration sync → Config watching + request-response in LSP layer

**Files to Reference During Port:**

- `/Users/norinlavaee/vscode-atomic/extensions/css-language-features/client/src/cssClient.ts` — Core client logic, middleware patterns
- `/Users/norinlavaee/vscode-atomic/extensions/css-language-features/client/src/node/cssClientMain.ts` — Node.js client activation
- `/Users/norinlavaee/vscode-atomic/extensions/css-language-features/server/src/cssServer.ts` — Server architecture, capability negotiation
- `/Users/norinlavaee/vscode-atomic/extensions/css-language-features/server/src/requests.ts` — Custom request/response protocol
- `/Users/norinlavaee/vscode-atomic/extensions/css-language-features/server/src/languageModelCache.ts` — Cache pattern for parsed ASTs
