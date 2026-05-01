# Pattern Research: VS Code JSON LSP Extension Architecture
## Partition 25 — `extensions/json-language-features/`

### Overview
The JSON language features extension demonstrates VS Code's canonical built-in language extension pattern. It implements a full LSP client-server architecture with:
- Platform-aware client initialization (Node.js + IPC vs. Browser + Web Worker)
- Type-safe request/response protocol definitions
- Middleware layer for client-side capability handling
- Dynamic language participant registration
- Schema caching and trusted domain validation

---

## Pattern 1: LSP Client Instantiation (Node.js IPC Transport)

**Where:** `extensions/json-language-features/client/src/node/jsonClientMain.ts:41-42`

**What:** Factory pattern for creating platform-specific LanguageClient with IPC transport and debug capabilities.

```typescript
const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
	return new LanguageClient(id, name, serverOptions, clientOptions);
};
```

**Context (lines 36-42):**
```typescript
const serverOptions: ServerOptions = {
	run: { module: serverModule, transport: TransportKind.ipc },
	debug: { module: serverModule, transport: TransportKind.ipc, options: debugOptions }
};

const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
	return new LanguageClient(id, name, serverOptions, clientOptions);
};
```

**Key aspects:**
- `ServerOptions` configured with `TransportKind.ipc` for Node.js process communication
- Debug mode adds `--nolazy --inspect=6000+random` for remote debugging
- Server module path resolved at runtime via `context.asAbsolutePath()`
- Factory function allows test injection and platform abstraction

---

## Pattern 2: LSP Client Instantiation (Browser Web Worker Transport)

**Where:** `extensions/json-language-features/client/src/browser/jsonClientMain.ts:20-21`

**What:** Browser variant using Web Worker as LSP server with fetch-based schema requests.

```typescript
const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
	return new LanguageClient(id, name, worker, clientOptions);
};
```

**Full initialization context (lines 14-43):**
```typescript
export async function activate(context: ExtensionContext) {
	const serverMain = Uri.joinPath(context.extensionUri, 'server/dist/browser/jsonServerMain.js');
	try {
		const worker = new Worker(serverMain.toString());
		worker.postMessage({ i10lLocation: l10n.uri?.toString(false) ?? '' });

		const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
			return new LanguageClient(id, name, worker, clientOptions);
		};

		const schemaRequests: SchemaRequestService = {
			getContent(uri: string) {
				return fetch(uri, { mode: 'cors' })
					.then(function (response: any) {
						return response.text();
					});
			}
		};
```

**Key aspects:**
- `Worker` API wraps server WASM/JS in browser
- Explicit `postMessage()` for localization bootstrap
- CORS mode for cross-origin schema fetches
- Error handling with try-catch around worker initialization

---

## Pattern 3: Language Client Options Configuration with Middleware

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:281-385`

**What:** Comprehensive middleware chain for protocol conversions and capability negotiation.

```typescript
const clientOptions: LanguageClientOptions = {
	documentSelector,
	initializationOptions: {
		handledSchemaProtocols: ['file'],
		provideFormatter: false,
		customCapabilities: { rangeFormatting: { editLimit: 10000 } }
	},
	synchronize: {
		configurationSection: ['json', 'http'],
		fileEvents: workspace.createFileSystemWatcher('**/*.json')
	},
	middleware: {
		workspace: {
			didChangeConfiguration: () => client.sendNotification(DidChangeConfigurationNotification.type, { settings: getSettings(true) })
		},
		provideDiagnostics: async (uriOrDoc, previousResolutId, token, next) => {
			const diagnostics = await next(uriOrDoc, previousResolutId, token);
			if (diagnostics && diagnostics.kind === DocumentDiagnosticReportKind.Full) {
				const uri = uriOrDoc instanceof Uri ? uriOrDoc : uriOrDoc.uri;
				diagnostics.items = handleSchemaErrorDiagnostics(uri, diagnostics.items);
			}
			return diagnostics;
		},
		provideCompletionItem(document: TextDocument, position: Position, context: CompletionContext, token: CancellationToken, next: ProvideCompletionItemsSignature): ProviderResult<CompletionItem[] | CompletionList> {
			// Range fixing for partial completions
			function update(item: CompletionItem) {
				const range = item.range;
				if (range instanceof Range && range.end.isAfter(position) && range.start.isBeforeOrEqual(position)) {
					item.range = { inserting: new Range(range.start, position), replacing: range };
				}
				if (item.documentation instanceof MarkdownString) {
					item.documentation = updateMarkdownString(item.documentation);
				}
			}
		}
	}
};
```

**Key aspects:**
- `initializationOptions` communicate client capabilities to server
- `synchronize.fileEvents` creates filesystem watcher for `.json` files
- `middleware.provideDiagnostics` intercepts diagnostics to filter schema errors
- `middleware.provideCompletionItem` adjusts range for replacement-vs-insert semantics
- Markdown documentation sanitization in middleware

---

## Pattern 4: Runtime Environment Abstraction (Node.js)

**Where:** `extensions/json-language-features/server/src/node/jsonServerMain.ts:58-72`

**What:** Platform-specific runtime capabilities injected into language server.

```typescript
const runtime: RuntimeEnvironment = {
	timer: {
		setImmediate(callback: (...args: any[]) => void, ...args: any[]): Disposable {
			const handle = setImmediate(callback, ...args);
			return { dispose: () => clearImmediate(handle) };
		},
		setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable {
			const handle = setTimeout(callback, ms, ...args);
			return { dispose: () => clearTimeout(handle) };
		}
	},
	file: getFileRequestService(),
	http: getHTTPRequestService(),
	configureHttpRequests
};

startServer(connection, runtime);
```

**RequestService implementations:**
```typescript
function getHTTPRequestService(): RequestService {
	return {
		getContent(uri: string, _encoding?: string) {
			const headers = { 'Accept-Encoding': 'gzip, deflate' };
			return xhr({ url: uri, followRedirects: 5, headers }).then(response => {
				return response.responseText;
			}, (error: XHRResponse) => {
				return Promise.reject(error.responseText || getErrorStatusDescription(error.status) || error.toString());
			});
		}
	};
}

function getFileRequestService(): RequestService {
	return {
		async getContent(location: string, encoding?: BufferEncoding) {
			try {
				const uri = Uri.parse(location);
				return (await fs.readFile(uri.fsPath, encoding)).toString();
			} catch (e) {
				if (e.code === 'ENOENT') {
					throw new Error(l10n.t('Schema not found: {0}', location));
				} else if (e.code === 'EISDIR') {
					throw new Error(l10n.t('{0} is a directory, not a file', location));
				}
				throw e;
			}
		}
	};
}
```

**Key aspects:**
- `Disposable` pattern wraps timers for lifecycle management
- Separate HTTP/file request services allow protocol routing
- HTTP service uses `request-light` for redirect/header handling
- File service resolves URI to fsPath with proper error codes

---

## Pattern 5: Runtime Environment Abstraction (Browser)

**Where:** `extensions/json-language-features/server/src/browser/jsonServerMain.ts:18-31`

**What:** Minimal browser runtime without HTTP/file services (delegate to client).

```typescript
const runtime: RuntimeEnvironment = {
	timer: {
		setImmediate(callback: (...args: any[]) => void, ...args: any[]): Disposable {
			const handle = setTimeout(callback, 0, ...args);  // no setImmediate in browser
			return { dispose: () => clearTimeout(handle) };
		},
		setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable {
			const handle = setTimeout(callback, ms, ...args);
			return { dispose: () => clearTimeout(handle) };
		}
	}
};

startServer(connection, runtime);
```

**Key aspects:**
- No HTTP/file service (browser sandboxing)
- `setImmediate` simulated with `setTimeout(cb, 0)`
- Connection delegates content requests back to client
- Lighter initialization than Node.js variant

---

## Pattern 6: Language Participant Discovery and Registration

**Where:** `extensions/json-language-features/client/src/languageParticipants.ts:31-78`

**What:** Dynamic discovery of language extensions that want JSON service support.

```typescript
export function getLanguageParticipants(): LanguageParticipants {
	const onDidChangeEmmiter = new EventEmitter<void>();
	let languages = new Set<string>();
	let comments = new Set<string>();

	function update() {
		const oldLanguages = languages, oldComments = comments;

		languages = new Set();
		languages.add('json');
		languages.add('jsonc');
		languages.add('snippets');
		comments = new Set();
		comments.add('jsonc');
		comments.add('snippets');

		for (const extension of extensions.allAcrossExtensionHosts) {
			const jsonLanguageParticipants = extension.packageJSON?.contributes?.jsonLanguageParticipants as LanguageParticipantContribution[];
			if (Array.isArray(jsonLanguageParticipants)) {
				for (const jsonLanguageParticipant of jsonLanguageParticipants) {
					const languageId = jsonLanguageParticipant.languageId;
					if (typeof languageId === 'string') {
						languages.add(languageId);
						if (jsonLanguageParticipant.comments === true) {
							comments.add(languageId);
						}
					}
				}
			}
		}
		return !isEqualSet(languages, oldLanguages) || !isEqualSet(comments, oldComments);
	}
	update();

	const changeListener = extensions.onDidChange(_ => {
		if (update()) {
			onDidChangeEmmiter.fire();
		}
	});

	return {
		onDidChange: onDidChangeEmmiter.event,
		get documentSelector() { return Array.from(languages); },
		hasLanguage(languageId: string) { return languages.has(languageId); },
		useComments(languageId: string) { return comments.has(languageId); },
		dispose: () => changeListener.dispose()
	};
}
```

**Key aspects:**
- Built-in languages (json, jsonc, snippets) hard-coded
- Extension manifest scanned for `contributes.jsonLanguageParticipants`
- `onDidChange` event fires when extension set changes
- Language set diffing prevents redundant notifications
- `commentSupport` tracked per-language for JSONC-like variants

---

## Pattern 7: Server Initialization and Capability Negotiation

**Where:** `extensions/json-language-features/server/src/jsonServer.ts:146-208`

**What:** LSP server initialization hook for capabilities and client feature detection.

```typescript
connection.onInitialize((params: InitializeParams): InitializeResult => {

	const initializationOptions = params.initializationOptions || {};
	const handledProtocols = initializationOptions?.handledSchemaProtocols;

	languageService = getLanguageService({
		schemaRequestService: getSchemaRequestService(handledProtocols),
		workspaceContext,
		contributions: [],
		clientCapabilities: params.capabilities
	});

	function getClientCapability<T>(name: string, def: T) {
		const keys = name.split('.');
		let c: any = params.capabilities;
		for (let i = 0; c && i < keys.length; i++) {
			if (!c.hasOwnProperty(keys[i])) {
				return def;
			}
			c = c[keys[i]];
		}
		return c;
	}

	clientSnippetSupport = getClientCapability('textDocument.completion.completionItem.snippetSupport', false);
	dynamicFormatterRegistration = getClientCapability('textDocument.rangeFormatting.dynamicRegistration', false) && (typeof initializationOptions.provideFormatter !== 'boolean');
	foldingRangeLimitDefault = getClientCapability('textDocument.foldingRange.rangeLimit', Number.MAX_VALUE);
	hierarchicalDocumentSymbolSupport = getClientCapability('textDocument.documentSymbol.hierarchicalDocumentSymbolSupport', false);
	formatterMaxNumberOfEdits = initializationOptions.customCapabilities?.rangeFormatting?.editLimit || Number.MAX_VALUE;

	const supportsDiagnosticPull = getClientCapability('textDocument.diagnostic', undefined);
	if (supportsDiagnosticPull === undefined) {
		diagnosticsSupport = registerDiagnosticsPushSupport(documents, connection, runtime, validateTextDocument);
	} else {
		diagnosticsSupport = registerDiagnosticsPullSupport(documents, connection, runtime, validateTextDocument);
	}

	const capabilities: ServerCapabilities = {
		textDocumentSync: TextDocumentSyncKind.Incremental,
		completionProvider: clientSnippetSupport ? {
			resolveProvider: false,
			triggerCharacters: ['"', ':']
		} : undefined,
		hoverProvider: true,
		documentSymbolProvider: true,
		documentRangeFormattingProvider: initializationOptions.provideFormatter === true,
		documentFormattingProvider: initializationOptions.provideFormatter === true,
		colorProvider: {},
		foldingRangeProvider: true,
		selectionRangeProvider: true,
		documentLinkProvider: {},
		diagnosticProvider: {
			documentSelector: null,
			interFileDependencies: false,
			workspaceDiagnostics: false
		},
		codeActionProvider: {
			codeActionKinds: [sortCodeActionKind]
		}
	};

	return { capabilities };
});
```

**Key aspects:**
- Probe client capabilities with dotted path lookup
- Route diagnostics via pull or push based on client support
- Conditional completion/formatting based on client snippetSupport
- FormatterMaxNumberOfEdits from custom capability for large edits
- ServerCapabilities returned as initialization response

---

## Pattern 8: Custom Request/Response Protocol Definitions

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:27-70`

**What:** Type-safe custom LSP requests/notifications for JSON-specific operations.

```typescript
namespace VSCodeContentRequest {
	export const type: RequestType<string, string, any> = new RequestType('vscode/content');
}

namespace SchemaContentChangeNotification {
	export const type: NotificationType<string | string[]> = new NotificationType('json/schemaContent');
}

namespace ForceValidateRequest {
	export const type: RequestType<string, Diagnostic[], any> = new RequestType('json/validate');
}

namespace LanguageStatusRequest {
	export const type: RequestType<string, JSONLanguageStatus, any> = new RequestType('json/languageStatus');
}

namespace ValidateContentRequest {
	export const type: RequestType<{ schemaUri: string; content: string }, LSPDiagnostic[], any> = new RequestType('json/validateContent');
}

namespace DocumentSortingRequest {
	export interface ITextEdit {
		range: {
			start: { line: number; character: number };
			end: { line: number; character: number };
		};
		newText: string;
	}
	export const type: RequestType<DocumentSortingParams, ITextEdit[], any> = new RequestType('json/sort');
}

namespace SchemaAssociationNotification {
	export const type: NotificationType<ISchemaAssociations | ISchemaAssociation[]> = new NotificationType('json/schemaAssociations');
}
```

**Key aspects:**
- Each request/notification wrapped in namespace for organization
- `RequestType<Params, Result, Error>` for bidirectional requests
- `NotificationType<Params>` for one-way notifications
- Type safety via generic parameters
- Protocol methods: `client.sendRequest()`, `client.sendNotification()`

---

## Pattern 9: Request Handler Registration (Server-side)

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:395-443`

**What:** Custom request handler for VS Code-specific protocol (vscode/content).

```typescript
client.onRequest(VSCodeContentRequest.type, async (uriPath: string) => {
	const uri = Uri.parse(uriPath);
	const uriString = uri.toString(true);
	
	if (uri.scheme === 'untitled') {
		throw new ResponseError(SchemaRequestServiceErrors.UntitledAccessError, l10n.t('Unable to load {0}', uriString));
	}
	if (uri.scheme === 'vscode') {
		try {
			runtime.logOutputChannel.info('read schema from vscode: ' + uriString);
			ensureFilesystemWatcherInstalled(uri);
			const content = await workspace.fs.readFile(uri);
			return new TextDecoder().decode(content);
		} catch (e) {
			throw new ResponseError(SchemaRequestServiceErrors.VSCodeAccessError, e.toString(), e);
		}
	} else if (uri.scheme !== 'http' && uri.scheme !== 'https') {
		try {
			const document = await workspace.openTextDocument(uri);
			schemaDocuments[uriString] = true;
			return document.getText();
		} catch (e) {
			throw new ResponseError(SchemaRequestServiceErrors.OpenTextDocumentAccessError, e.toString(), e);
		}
	} else if (schemaDownloadEnabled) {
		if (!workspace.isTrusted) {
			throw new ResponseError(SchemaRequestServiceErrors.UntrustedWorkspaceError, l10n.t('Downloading schemas is disabled in untrusted workspaces'));
		}
		if (!await isTrusted(uri)) {
			throw new ResponseError(SchemaRequestServiceErrors.UntrustedSchemaError, l10n.t('Location {0} is untrusted', uriString));
		}
		if (runtime.telemetry && uri.authority === 'schema.management.azure.com') {
			runtime.telemetry.sendTelemetryEvent('json.schema', { schemaURL: uriString });
		}
		try {
			return await runtime.schemaRequests.getContent(uriString);
		} catch (e) {
			throw new ResponseError(SchemaRequestServiceErrors.HTTPError, e.toString(), e);
		}
	} else {
		throw new ResponseError(SchemaRequestServiceErrors.HTTPDisabledError, l10n.t('Downloading schemas is disabled through setting \'{0}\'', SettingIds.enableSchemaDownload));
	}
});
```

**Key aspects:**
- Type-safe handler via `client.onRequest(type, async handler)`
- Scheme-based routing (vscode://, file://, http://)
- Trust boundary checks for remote schemas
- URI caching for watched documents
- Telemetry injection for schema usage tracking

---

## Pattern 10: Extension Package.json Contribution Points

**Where:** `extensions/json-language-features/package.json:39-189`

**What:** Schema-driven extension manifest with contributes block.

**Configuration properties (subset):**
```json
"contributes": {
  "configuration": {
    "id": "json",
    "properties": {
      "json.schemas": {
        "type": "array",
        "scope": "resource",
        "items": {
          "properties": {
            "url": { "type": "string" },
            "fileMatch": { "type": "array" },
            "schema": { "$ref": "http://json-schema.org/draft-07/schema#" }
          }
        }
      },
      "json.validate.enable": { "type": "boolean", "default": true },
      "json.format.enable": { "type": "boolean", "default": true },
      "json.maxItemsComputed": { "type": "number", "default": 5000 },
      "json.schemaDownload.enable": { "type": "boolean", "default": true },
      "json.schemaDownload.trustedDomains": {
        "type": "object",
        "default": {
          "https://schemastore.azurewebsites.net/": true,
          "https://json.schemastore.org/": true
        }
      }
    }
  },
  "jsonValidation": [
    { "fileMatch": "*.schema.json", "url": "http://json-schema.org/draft-07/schema#" }
  ],
  "commands": [
    { "command": "json.clearCache", "title": "%json.command.clearCache%" },
    { "command": "json.sort", "title": "%json.command.sort%" }
  ]
}
```

**Key aspects:**
- Settings localized via i18n keys (`%json.command.clearCache%`)
- `jsonValidation` contribution for schema declarations
- Nested configuration with `fileMatch`, `url`, `schema` structure
- `trustedDomains` security model with defaults
- Activation events scoped to language (`onLanguage:json`)

---

## Summary: Architectural Patterns for Tauri/Rust Porting

### Transport Layer Abstraction
The JSON extension demonstrates **transport-agnostic LSP** via factory pattern:
- Node.js uses IPC (`TransportKind.ipc`)
- Browser uses Web Worker message passing
- Both share identical `LanguageClientOptions` and protocol

**Implication for Tauri:** Replace IPC with Tauri command-based RPC. Server could run as background thread in Tauri Rust core or separate process, communicating via `tauri::invoke()` for request/response and `tauri::emit()` for notifications.

### Runtime Capability Injection
Server receives platform-specific `RuntimeEnvironment` with timer/request services abstracted behind interfaces.

**Implication for Tauri:** Inject Rust equivalents (`tokio::time::sleep`, `reqwest` HTTP client, `async_fs` file operations) into server. Allows single server codebase across platforms.

### Middleware Protocol Conversion
Client middleware intercepts LSP messages to:
- Convert protocol types (Range, Diagnostic)
- Adjust completion ranges for insertion vs. replacement
- Filter diagnostics by trust level

**Implication for Tauri:** Middleware becomes Tauri command interceptors. Custom handlers for `vscode/content` request route to Rust file I/O or HTTP.

### Dynamic Extension Registration
Language participants discovered at runtime from extension manifests, enabling third-party language extensions to hook the JSON server.

**Implication for Tauri:** Plugin system needed. Tauri CLI extensions could register via manifest scanning, similar to VS Code extension host API.

### Custom Protocol Design
LSP extended with domain-specific requests (`json/validate`, `json/sort`, `json/schemaContent`, `vscode/content`).

**Implication for Tauri:** RPC schema would map these to Rust handler functions. Typed request definitions (RequestType<Params, Result>) become function signatures.

### Settings and Configuration
Workspace settings fetched via `workspace.getConfiguration('json').inspect()`, supporting workspace-folder-scoped and user-level configuration.

**Implication for Tauri:** Settings would map to JSON files or SQLite, with folder-scoped precedence rules.

### Trust Boundary Enforcement
Schema downloads gated by `workspace.isTrusted` and per-domain whitelist in `trustedDomains` setting.

**Implication for Tauri:** Trust model must replicate VS Code's untrusted workspace concept. Network requests require explicit user approval or trusted domain list.

---

**Total LOC analyzed:** 3,042 (19 files)  
**Key files:**
- `client/src/node/jsonClientMain.ts` — Node.js activation & server wiring
- `client/src/browser/jsonClientMain.ts` — Browser activation & Web Worker setup
- `client/src/jsonClient.ts` — Core client logic, middleware, request handlers
- `server/src/node/jsonServerMain.ts` — Node.js runtime setup
- `server/src/browser/jsonServerMain.ts` — Browser runtime setup
- `server/src/jsonServer.ts` — Server core, initialization, handlers
- `client/src/languageParticipants.ts` — Language discovery

