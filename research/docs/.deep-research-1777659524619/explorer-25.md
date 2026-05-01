# Partition 25 of 79 — Findings

## Scope
`extensions/json-language-features/` (19 files, 3,042 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locations: JSON Language Features Extension

## Partition Summary
The `extensions/json-language-features/` directory (19 TypeScript files, 3,042 LOC) contains VS Code's built-in JSON language support via a client-server LSP architecture. This is a canonical template for how VS Code's core IDE functionality is split between:
- **Client**: VS Code extension API layer (runs in Electron main process on desktop, Web Worker on browser)
- **Server**: Language service implementation (Node process on desktop, Web Worker on browser)

## Implementation Files

### Client Layer (Extension Host)
- `/extensions/json-language-features/client/src/jsonClient.ts` — Core LSP client setup, request/notification handlers, schema management, diagnostics coordination (940 LOC)
- `/extensions/json-language-features/client/src/node/jsonClientMain.ts` — Node.js entry point; spawns LSP server process, telemetry setup, file schema caching (176 LOC)
- `/extensions/json-language-features/client/src/browser/jsonClientMain.ts` — Browser entry point; instantiates Web Worker as LSP server, schema fetching via CORS (56 LOC)
- `/extensions/json-language-features/client/src/languageParticipants.ts` — Discovers language extensions that participate in JSON LSP; monitors extension changes (91 LOC)
- `/extensions/json-language-features/client/src/languageStatus.ts` — Status bar items for schema resolution errors, document symbol limits, language diagnostics
- `/extensions/json-language-features/client/src/node/schemaCache.ts` — ETag-based schema cache for offline access on Node.js

### Utilities (Client)
- `/extensions/json-language-features/client/src/utils/hash.ts` — Custom hash function for schema object identity (59 LOC)
- `/extensions/json-language-features/client/src/utils/urlMatch.ts` — Trusted domain pattern matching (localhost, wildcards, glob patterns) (108 LOC)

### Server Layer (Language Service)
- `/extensions/json-language-features/server/src/jsonServer.ts` — LSP server implementation; defines request/notification types, diagnostics handling, schema associations, validation logic (583 LOC)
- `/extensions/json-language-features/server/src/node/jsonServerMain.ts` — Node.js server entry point; connection setup, HTTP/file request services (76 LOC)
- `/extensions/json-language-features/server/src/node/jsonServerNodeMain.ts` — Node module wrapper (referenced by esbuild config)
- `/extensions/json-language-features/server/src/browser/jsonServerMain.ts` — Browser server entry point; Web Worker message reader/writer (32 LOC)
- `/extensions/json-language-features/server/src/browser/jsonServerWorkerMain.ts` — Worker initialization; defers to jsonServerMain after l10n setup (36 LOC)

### Utilities (Server)
- `/extensions/json-language-features/server/src/utils/validation.ts` — Diagnostic push/pull support registration (100+ LOC)
- `/extensions/json-language-features/server/src/utils/runner.ts` — Safe async/sync execution wrappers for LSP handlers
- `/extensions/json-language-features/server/src/utils/strings.ts` — String utility functions
- `/extensions/json-language-features/server/src/languageModelCache.ts` — Document parse cache with time-based eviction

## Configuration

### Extension Manifest
- `/extensions/json-language-features/package.json` — VS Code extension metadata:
  - **Main entry**: `./client/out/node/jsonClientMain` (Node.js)
  - **Browser entry**: `./client/dist/browser/jsonClientMain` (Web)
  - **Activation events**: `onLanguage:json`, `onLanguage:jsonc`, `onLanguage:snippets`, `onCommand:json.validate`
  - **Contributions**: JSON schema configuration schema, custom JSON sort/validate commands
  - **Dependencies**: `vscode-languageclient@^10.0.0-next.20`, `request-light`, `@vscode/extension-telemetry`

### TypeScript Configuration
- `/extensions/json-language-features/client/tsconfig.json` — Client compilation (ES2024, webworker, Node16 modules)
- `/extensions/json-language-features/client/tsconfig.browser.json` — Browser-specific overrides
- `/extensions/json-language-features/server/tsconfig.json` — Server compilation (ES2024, ESM modules)
- `/extensions/json-language-features/server/tsconfig.browser.json` — Browser server compilation

### Build Configuration
- `/extensions/json-language-features/esbuild.mts` — Parallel esbuild for client (Node.js) and server (Node.js ESM with `require` polyfill)
- `/extensions/json-language-features/esbuild.browser.mts` — Browser builds (client Web Worker + server Web Worker)

### Language Server Manifest
- `/extensions/json-language-features/server/package.json` — Language server package metadata:
  - **Version**: 1.3.4
  - **Main**: `./out/node/jsonServerMain` (CommonJS)
  - **Dependencies**: `vscode-json-languageservice@^6.0.0-next.1`, `vscode-languageserver@^10.0.0-next.16`, `jsonc-parser`, `request-light`

### Runtime Configuration
- `/extensions/json-language-features/.npmrc` — NPM configuration
- `/extensions/json-language-features/server/.npmrc` — Server NPM configuration
- `/extensions/json-language-features/.vscodeignore` — Files excluded from VSIX packaging

### Localization
- `/extensions/json-language-features/package.nls.json` — Localization strings for extension metadata

### Development
- `/extensions/json-language-features/.vscode/launch.json` — Debug launch configurations
- `/extensions/json-language-features/.vscode/tasks.json` — VS Code build tasks
- `/extensions/json-language-features/server/.vscode/launch.json` — Server debug config
- `/extensions/json-language-features/server/.vscode/tasks.json` — Server build tasks

## Documentation

- `/extensions/json-language-features/README.md` — Extension overview (links to VS Code JSON docs)
- `/extensions/json-language-features/CONTRIBUTING.md` — Contribution guidelines
- `/extensions/json-language-features/server/README.md` — Language server details

## Asset
- `/extensions/json-language-features/icons/json.png` — Extension icon

## Key Architecture Patterns

### Client-Server Boundary
The extension demonstrates the canonical LSP split:
1. **Client** (`jsonClient.ts`, `jsonClientMain.ts`) — Implements VS Code extension API:
   - UI commands (validate, sort, clear cache)
   - Status bar items and diagnostics visualization
   - Settings/configuration change handling
   - File system watching for schema updates
   - Trust/security checks (schema download settings, trusted domains)
   
2. **Server** (`jsonServer.ts`, `jsonServerMain.ts`) — Pure language service:
   - Completion, hover, symbol, folding, color, formatting
   - Schema resolution and validation
   - Request types: `vscode/content`, `json/schemaAssociations`, `json/schemaContent`, `json/validate`, `json/sort`
   - No direct UI or file system access (defers to client via LSP requests)

### Platform Abstraction
- **Node.js path**: Uses `ipc` transport, spawned Node process
- **Browser path**: Uses Web Workers, CORS-based schema fetching

### Schema Management
- ETag-based caching (Node.js only)
- Trust domain validation (glob patterns, localhost special-casing)
- Dynamic schema associations from extensions
- Per-folder schema configuration

### Diagnostic Models
- Supports both LSP push (`diagnostics/pull` if unavailable) and pull models
- Schema resolution errors trigger code actions for trust/settings
- Document symbol and folding range limits with status bar feedback

## Notable Implementation Details

### LanguageClient Instantiation
The `new LanguageClient()` pattern is instantiated twice (Node.js and browser):
- **Node.js**: `new LanguageClient(id, name, serverOptions, clientOptions)` (line 42, jsonClientMain.ts)
  - `serverOptions`: IPC transport to spawned Node process
  - `clientOptions`: Editor integration (document selector, middleware, diagnostics handling)
- **Browser**: `new LanguageClient(id, name, worker, clientOptions)` (line 21, jsonClientMain.ts)
  - `worker`: Web Worker instance
  - Same `clientOptions` API

### Request/Notification Protocol
Custom protocol messages defined as TypeScript types:
```typescript
namespace VSCodeContentRequest {
  export const type: RequestType<string, string, any> = new RequestType('vscode/content');
}
namespace SchemaAssociationNotification {
  export const type: NotificationType<ISchemaAssociations | SchemaConfiguration[]> = new NotificationType('json/schemaAssociations');
}
```

### Settings Propagation
Settings flow bidirectionally:
- Client reads `json.*`, `http.*` VS Code settings via `workspace.getConfiguration()`
- Client sends via `DidChangeConfigurationNotification` to server
- Server applies settings to language service state (validation, formatting limits)

## File Count and Organization
- **Total**: 19 files (excluding lock files and node_modules)
- **TypeScript source**: 14 files
- **Configuration**: 5 files (tsconfig, package.json, esbuild)
- **Client split**: Node.js (~8 files) vs. Browser (~4 files)
- **Server split**: Node.js (~5 files) vs. Browser (~3 files)
- **Shared code**: jsonClient.ts, jsonServer.ts, utilities

## Porting Implications for Tauri/Rust

### Dependencies to Replace
- `vscode-languageclient` (Node/browser) → Tauri command/event system + custom LSP transport
- `vscode-json-languageservice` (Node) → Rust JSON language service crate (e.g., `jsonrpc`, custom parser)
- `request-light` (HTTP requests) → Tauri `http` plugin or `reqwest`
- Electron IPC → Tauri `invoke()` / `listen()` for process communication

### Architectural Changes
1. **Transport Layer**: Replace LSP client setup (currently IPC or Web Worker) with Tauri backend process
2. **Schema Caching**: Implement ETag cache in Rust (currently Node.js file-based)
3. **Trust Domain Matching**: Port URL pattern matching logic to Rust
4. **Diagnostics Model**: Preserve LSP protocol; adapt to Tauri command routing
5. **Settings Propagation**: Replace VS Code `workspace.getConfiguration()` with Tauri state/event system
6. **File System Access**: Replace VS Code `workspace.fs` API with Tauri `fs` plugin

### Core Complexity
- **LSP Protocol Compliance**: The server is a full LSP implementation; porting requires preserving request/response/notification semantics
- **Multi-Language Support**: Current client discovers language participants dynamically; Rust server must either hard-code JSON or provide equivalent plugin/configuration mechanism
- **Schema Resolution**: Critical feature with network I/O, ETag caching, trust validation; each must be ported faithfully
- **Incremental Sync**: Clients request document range operations; server state management must handle concurrent edits

The extension's client-server split is intentional and optimized for VS Code's multi-process architecture. A Tauri port would require similar separation (main process host ↔ Rust backend), but with different IPC primitives and runtime constraints.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
