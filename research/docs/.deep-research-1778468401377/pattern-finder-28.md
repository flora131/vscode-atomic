# LSP-Client Patterns in CSS Language Features Extension

## Research Overview

This analysis identifies the concrete LSP-client/language-feature patterns in VS Code's CSS language features extension (`extensions/css-language-features/`, 2,261 LOC) that any Rust/Tauri port must replicate to support core IDE functionality.

The architecture separates into **client** (UI/editor-facing) and **server** (language logic) with LSP communication over IPC (Node.js) or Web Workers (browser).

---

## Pattern 1: LanguageClient Initialization

**Where:** `extensions/css-language-features/client/src/node/cssClientMain.ts:26-32`

**What:** Establishes the LSP client over Node.js IPC transport with debug options.

```typescript
const serverOptions: ServerOptions = {
	run: { module: serverModule, transport: TransportKind.ipc },
	debug: { module: serverModule, transport: TransportKind.ipc, options: debugOptions }
};

const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
	return new LanguageClient(id, name, serverOptions, clientOptions);
};
```

**Variations:**

- Browser variant (`extensions/css-language-features/client/src/browser/cssClientMain.ts:17-23`): Uses `Worker` transport instead of IPC.
- Abstract constructor passed to `startClient()` to decouple platform transport from language-agnostic setup logic.

---

## Pattern 2: Language Client Document Selection & Middleware

**Where:** `extensions/css-language-features/client/src/cssClient.ts:39-97`

**What:** Configures LSP synchronization, initialization options, and middleware to adapt LSP responses for VS Code UI (e.g., completion item label formatting, range handling).

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
	},
	middleware: {
		provideCompletionItem(document, position, context, token, next) {
			// Adapt range format: split into insert/replace
			// Adapt color labels with descriptions
			const r = next(document, position, context, token);
			return isThenable(r) ? r.then(updateProposals) : updateProposals(r);
		}
	}
};
```

**Key aspects:**
- `documentSelector` narrows activation to CSS-like languages.
- `synchronize.configurationSection` triggers server updates on config changes.
- `initializationOptions` passes server capabilities and constraints.
- `middleware` transforms LSP responses for UI compatibility.

---

## Pattern 3: Server-Side Connection Lifecycle & Capability Registration

**Where:** `extensions/css-language-features/server/src/cssServer.ts:69-139`

**What:** Server initializes on `connection.onInitialize()`, interrogates client capabilities, configures language services, and declares LSP capabilities.

```typescript
connection.onInitialize((params: InitializeParams): InitializeResult => {
	const initializationOptions = params.initializationOptions || {};
	workspaceFolders = params.workspaceFolders || [];
	requestService = getRequestService(initializationOptions?.handledSchemas || ['file'], connection, runtime);

	// Detect client capabilities
	const snippetSupport = !!getClientCapability('textDocument.completion.completionItem.snippetSupport', false);
	scopedSettingsSupport = !!getClientCapability('workspace.configuration', false);

	// Initialize language services
	languageServices.css = getCSSLanguageService({ fileSystemProvider: requestService, clientCapabilities: params.capabilities });
	languageServices.scss = getSCSSLanguageService({ ... });
	languageServices.less = getLESSLanguageService({ ... });

	// Register diagnostics support (push or pull)
	const supportsDiagnosticPull = getClientCapability('textDocument.diagnostic', undefined);
	diagnosticsSupport = supportsDiagnosticPull === undefined 
		? registerDiagnosticsPushSupport(documents, connection, runtime, validateTextDocument)
		: registerDiagnosticsPullSupport(documents, connection, runtime, validateTextDocument);

	const capabilities: ServerCapabilities = {
		textDocumentSync: TextDocumentSyncKind.Incremental,
		completionProvider: snippetSupport ? { resolveProvider: false, triggerCharacters: ['/', '-', ':'] } : undefined,
		hoverProvider: true,
		documentSymbolProvider: true,
		// ... 12 more capabilities
	};
	return { capabilities };
});
```

**Key aspects:**
- Negotiates client capabilities at handshake.
- Conditionally enables features (snippets, diagnostics) based on client support.
- Language services instantiated once with runtime config.

---

## Pattern 4: LSP Request Handlers (on* Patterns)

**Where:** `extensions/css-language-features/server/src/cssServer.ts:198-357`

**What:** Register handlers for LSP requests/notifications with cancellation-aware async error wrapping.

```typescript
connection.onCompletion((textDocumentPosition, token) => {
	return runSafeAsync(runtime, async () => {
		const document = documents.get(textDocumentPosition.textDocument.uri);
		if (document) {
			const [settings,] = await Promise.all([getDocumentSettings(document), dataProvidersReady]);
			const styleSheet = stylesheets.get(document);
			const documentContext = getDocumentContext(document.uri, workspaceFolders);
			return getLanguageService(document).doComplete2(document, textDocumentPosition.position, styleSheet, documentContext, settings?.completion);
		}
		return null;
	}, null, `Error while computing completions for ${textDocumentPosition.textDocument.uri}`, token);
});

// Similar handlers for:
// - onHover, onDocumentSymbol, onDefinition, onDocumentHighlight, onDocumentLinks
// - onReferences, onCodeAction, onDocumentColor, onColorPresentation
// - onRenameRequest, onFoldingRanges, onSelectionRanges
// - onDocumentRangeFormatting, onDocumentFormatting
```

**Key aspects:**
- All handlers wrapped in `runSafeAsync()` for error/cancellation handling.
- Document retrieval via `TextDocuments` manager.
- Compose settings + cached stylesheets to avoid re-parsing.
- Language service method dispatches to vscode-css-languageservice.

**Handlers present:** 16 LSP request/notification handlers.

---

## Pattern 5: File System Request Service (Client ↔ Server Bridge)

**Where:** `extensions/css-language-features/client/src/requests.ts:21-45`

**What:** Client-side listener for server-initiated file system requests (fs/content, fs/stat, fs/readDir).

```typescript
export function serveFileSystemRequests(client: BaseLanguageClient, runtime: Runtime) {
	client.onRequest(FsContentRequest.type, (param: { uri: string; encoding?: string }) => {
		const uri = Uri.parse(param.uri);
		if (uri.scheme === 'file' && runtime.fs) {
			return runtime.fs.getContent(param.uri);  // Use native FS if available
		}
		return workspace.fs.readFile(uri).then(buffer => {
			return new runtime.TextDecoder(param.encoding).decode(buffer);
		});
	});
	client.onRequest(FsReadDirRequest.type, (uriString: string) => { ... });
	client.onRequest(FsStatRequest.type, (uriString: string) => { ... });
}
```

**Server-side counterpart:** `extensions/css-language-features/server/src/requests.ts:66-99`

```typescript
export function getRequestService(handledSchemas: string[], connection: Connection, runtime: RuntimeEnvironment): RequestService {
	const builtInHandlers: { [protocol: string]: RequestService | undefined } = {};
	for (const protocol of handledSchemas) {
		if (protocol === 'file') {
			builtInHandlers[protocol] = runtime.file;  // Direct file I/O if trusted
		} else if (protocol === 'http' || protocol === 'https') {
			builtInHandlers[protocol] = runtime.http;
		}
	}
	return {
		async stat(uri: string): Promise<FileStat> {
			const handler = builtInHandlers[getScheme(uri)];
			if (handler) return handler.stat(uri);
			return connection.sendRequest(FsStatRequest.type, uri.toString());  // Fallback to client
		},
		// Similar for readDirectory, getContent
	};
}
```

**Key aspects:**
- Named RequestTypes (`FsContentRequest`, `FsStatRequest`, `FsReadDirRequest`) define RPC contract.
- Server tries local handlers first, falls back to client.
- Abstracts file I/O to decouple language logic from platform FS.

---

## Pattern 6: Error Handling & Cancellation-Aware Async Wrapper

**Where:** `extensions/css-language-features/server/src/utils/runner.ts:21-45`

**What:** Wraps all LSP handler execution with cancellation checking and error formatting.

```typescript
export function runSafeAsync<T>(
	runtime: RuntimeEnvironment,
	func: () => Thenable<T>,
	errorVal: T,
	errorMessage: string,
	token: CancellationToken
): Thenable<T | ResponseError<any>> {
	return new Promise<T | ResponseError<any>>((resolve) => {
		runtime.timer.setImmediate(() => {
			if (token.isCancellationRequested) {
				resolve(cancelValue());
				return;
			}
			return func().then(result => {
				if (token.isCancellationRequested) {
					resolve(cancelValue());
					return;
				} else {
					resolve(result);
				}
			}, e => {
				console.error(formatError(errorMessage, e));
				resolve(errorVal);  // Return default on error
			});
		});
	});
}

function cancelValue<E>() {
	return new ResponseError<E>(LSPErrorCodes.RequestCancelled, 'Request cancelled');
}
```

**Key aspects:**
- Defers execution to next tick via `runtime.timer.setImmediate`.
- Checks cancellation **before** and **after** async work.
- Logs exceptions, returns neutral fallback value on error.
- Returns LSP ResponseError on cancellation.

---

## Pattern 7: Custom Notification (One-Way Server Push)

**Where:** `extensions/css-language-features/server/src/cssServer.ts:18-20` and `client/src/cssClient.ts:11-13`

**What:** Defines and sends custom notifications (css/customDataChanged) for out-of-band server state updates.

```typescript
// Server-side definition & send
namespace CustomDataChangedNotification {
	export const type: NotificationType<string[]> = new NotificationType('css/customDataChanged');
}

connection.onNotification(CustomDataChangedNotification.type, updateDataProviders);

// Client-side definition & send
client.sendNotification(CustomDataChangedNotification.type, customDataSource.uris);
customDataSource.onDidChange(() => {
	client.sendNotification(CustomDataChangedNotification.type, customDataSource.uris);
});
```

**Key aspects:**
- Custom namespace isolates protocol definition.
- `NotificationType<T>` parameterizes payload type.
- Client pushes new custom data paths on configuration/extension changes.
- Server updates language service data providers reactively.

---

## Pattern 8: Diagnostic Push vs. Pull (Capability Negotiation)

**Where:** `extensions/css-language-features/server/src/utils/validation.ts:17-100`

**What:** Supports both push diagnostics (server initiates) and pull diagnostics (client requests), selected during init based on capability.

**Push variant** (lines 17-75):
```typescript
export function registerDiagnosticsPushSupport(documents: TextDocuments<TextDocument>, connection: Connection, runtime: RuntimeEnvironment, validate: Validator): DiagnosticsSupport {
	const pendingValidationRequests: { [uri: string]: Disposable } = {};
	const validationDelayMs = 500;

	documents.onDidChangeContent(change => {
		triggerValidation(change.document);
	});
	documents.onDidClose(event => {
		connection.sendDiagnostics({ uri: event.document.uri, diagnostics: [] });
	});

	function triggerValidation(textDocument: TextDocument): void {
		cleanPendingValidation(textDocument);
		const request = runtime.timer.setTimeout(async () => {
			const diagnostics = await validate(textDocument);
			connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
		}, validationDelayMs);
	}
	// ...
}
```

**Pull variant** (lines 77-95):
```typescript
export function registerDiagnosticsPullSupport(documents: TextDocuments<TextDocument>, connection: Connection, runtime: RuntimeEnvironment, validate: Validator): DiagnosticsSupport {
	const registration = connection.languages.diagnostics.on(async (params: DocumentDiagnosticParams, token: CancellationToken) => {
		return runSafeAsync(runtime, async () => {
			const document = documents.get(params.textDocument.uri);
			if (document) {
				return newDocumentDiagnosticReport(await validate(document));
			}
			return newDocumentDiagnosticReport([]);
		}, newDocumentDiagnosticReport([]), `Error while computing diagnostics...`, token);
	});

	function requestRefresh(): void {
		connection.languages.diagnostics.refresh();
	}
	// ...
}
```

**Key aspects:**
- Single validator function, dual transport.
- Push: debounces on content change, server-initiated send.
- Pull: handler registered with `connection.languages.diagnostics.on()`, client-initiated fetch.
- Same `DiagnosticsSupport` interface for both.

---

## Pattern 9: Middleware Provider for Client-Side Completion Adaptation

**Where:** `extensions/css-language-features/client/src/cssClient.ts:60-96`

**What:** Intercepts LSP completion responses to adapt them for VS Code UI (range formats, label descriptions).

```typescript
middleware: {
	provideCompletionItem(document: TextDocument, position: Position, context: CompletionContext, token: CancellationToken, next: ProvideCompletionItemsSignature): ProviderResult<CompletionItem[] | CompletionList> {
		function updateRanges(item: CompletionItem) {
			const range = item.range;
			if (range instanceof Range && range.end.isAfter(position) && range.start.isBeforeOrEqual(position)) {
				// Split into insert/replace ranges (VS Code 1.67+)
				item.range = { inserting: new Range(range.start, position), replacing: range };
			}
		}
		function updateLabel(item: CompletionItem) {
			// Add description to color completions
			if (item.kind === CompletionItemKind.Color) {
				item.label = {
					label: item.label as string,
					description: (item.documentation as string)
				};
			}
		}
		// Chain to next provider, transform result
		const r = next(document, position, context, token);
		return isThenable(r) ? r.then(updateProposals) : updateProposals(r);
	}
}
```

**Key aspects:**
- Middleware intercepts before returning to editor.
- Handles async promise chains with `isThenable` check.
- Adapts LSP ranges and labels per VS Code conventions.

---

## Pattern 10: Drop/Paste Resource Handler (UI Feature Registration)

**Where:** `extensions/css-language-features/client/src/dropOrPaste/dropOrPasteResource.ts:11-153`

**What:** Registers document drop/paste edit providers for CSS URL insertion, handling file URI conversion.

```typescript
class DropOrPasteResourceProvider implements vscode.DocumentDropEditProvider, vscode.DocumentPasteEditProvider {
	readonly kind = vscode.DocumentDropOrPasteEditKind.Empty.append('css', 'link', 'url');

	async provideDocumentDropEdits(document, position, dataTransfer, token) {
		const uriList = await this.getUriList(dataTransfer);
		if (!uriList.entries.length || token.isCancellationRequested) return;

		const snippet = await this.createUriListSnippet(document.uri, uriList);
		return {
			kind: this.kind,
			title: snippet.label,
			insertText: snippet.snippet.value,
			yieldTo: this.pasteAsCssUrlByDefault(document, position) ? [] : [...]
		};
	}

	private async createUriListSnippet(docUri, uriList) {
		const snippet = new vscode.SnippetString();
		for (const uri of uriList.entries) {
			const relativePath = getRelativePath(getDocumentDir(docUri), uri.uri);
			snippet.appendText(`url(${relativePath ?? uri.str})`);
		}
		return { snippet, label: '...' };
	}
}

export function registerDropOrPasteResourceSupport(selector) {
	return vscode.Disposable.from(
		vscode.languages.registerDocumentDropEditProvider(selector, provider, { ... }),
		vscode.languages.registerDocumentPasteEditProvider(selector, provider, { ... })
	);
}
```

**Key aspects:**
- Leverages native VS Code Drop/Paste API (1.76+).
- Converts file URIs to relative paths for insertion.
- Registered per language selector.

---

## Pattern 11: Configuration Section Synchronization

**Where:** `extensions/css-language-features/client/src/cssClient.ts:50-54` and `server/src/cssServer.ts:155-166`

**What:** Client and server synchronize settings via `configurationSection` and explicit `ConfigurationRequest`.

```typescript
// Client-side sync declaration
const clientOptions: LanguageClientOptions = {
	synchronize: {
		configurationSection: ['css', 'scss', 'less']
	}
};

// Server-side fetch (scoped to document if client supports it)
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

// Server-side notification listener
connection.onDidChangeConfiguration(change => {
	updateConfiguration(change.settings as { [languageId: string]: LanguageSettings });
});
```

**Key aspects:**
- LSP declares which config sections trigger server updates.
- Server caches per-document settings keyed by URI.
- Fallback to undefined if client doesn't support scoped config.

---

## Summary: Core LSP Patterns for Rust/Tauri Port

A Rust/Tauri port must support:

1. **LanguageClient Abstraction**: Pluggable transport (IPC, Web Worker, Tauri IPC) with same clientOptions interface.
2. **Connection Lifecycle**: onInitialize handshake with capability negotiation and ServerCapabilities response.
3. **16 LSP Handlers**: Completion, Hover, DocumentSymbol, Definition, DocumentHighlight, DocumentLinks, References, CodeAction, DocumentColor, ColorPresentation, Rename, FoldingRanges, SelectionRanges, DocumentFormatting, DocumentRangeFormatting, plus Shutdown.
4. **Middleware Layer**: Intercept client-side responses to adapt for UI (ranges, labels, filtering).
5. **Custom RPC Types**: RequestType<P, R, E> and NotificationType<P> for typed protocol definitions.
6. **Error Handling**: runSafeAsync pattern with cancellation token checks and error fallback values.
7. **Diagnostic Dual-Mode**: Push (server-initiated) and Pull (client-initiated) variants selected via capability.
8. **File System Abstraction**: Named RequestTypes for fs/content, fs/stat, fs/readDir with protocol-based routing.
9. **Settings Sync**: ConfigurationSection in synchronize block + ConfigurationRequest for scoped document settings.
10. **Drop/Paste Providers**: Register language-specific content adapters for file URI→relative path transformation.
11. **TextDocuments Manager**: Listen for open/change/close events, maintain in-memory document cache.
12. **Custom Notifications**: Untyped push from either side (e.g., css/customDataChanged for data reloads).

All handlers use the `runSafeAsync` wrapper to ensure cancellation safety and error resilience. The architecture cleanly separates **platform transport** (IPC vs. Worker) from **language protocol** (LSP handlers).

