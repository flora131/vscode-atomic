# Pattern Finder: JSON Language Features Extension

## Research Question
Patterns for porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust, extracted from the JSON language features extension.

## Scope
`extensions/json-language-features/` (19 files, 3,042 LOC)

---

#### Pattern: LSP Host Pattern with Middleware

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:182-211`

**What:** Async client initialization wrapping LanguageClient with middleware for schema caching and diagnostics handling.

```typescript
export async function startClient(context: ExtensionContext, newLanguageClient: LanguageClientConstructor, runtime: Runtime): Promise<AsyncDisposable> {
	const languageParticipants = getLanguageParticipants();
	context.subscriptions.push(languageParticipants);

	let client: Disposable | undefined = await startClientWithParticipants(context, languageParticipants, newLanguageClient, runtime);

	let restartTrigger: Disposable | undefined;
	languageParticipants.onDidChange(() => {
		if (restartTrigger) {
			restartTrigger.dispose();
		}
		restartTrigger = runtime.timer.setTimeout(async () => {
			if (client) {
				runtime.logOutputChannel.info('Extensions have changed, restarting JSON server...');
				const oldClient = client;
				client = undefined;
				await oldClient.dispose();
				client = await startClientWithParticipants(context, languageParticipants, newLanguageClient, runtime);
			}
		}, 2000);
	});

	return {
		dispose: async () => {
			restartTrigger?.dispose();
			await client?.dispose();
		}
	};
}
```

**Variations / call-sites:** 
- Node variant: `extensions/json-language-features/client/src/node/jsonClientMain.ts:20-58`
- Browser variant: `extensions/json-language-features/client/src/browser/jsonClientMain.ts:14-44`
- Both create a `newLanguageClient` constructor and pass runtime + schema services.

---

#### Pattern: Request/Notification Type Definitions

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:27-83`

**What:** Custom LSP request/notification types namespace pattern for schema management and validation.

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

namespace ValidateContentRequest {
	export const type: RequestType<{ schemaUri: string; content: string }, LSPDiagnostic[], any> = new RequestType('json/validateContent');
}

namespace DocumentSortingRequest {
	export interface ITextEdit {
		range: { start: { line: number; character: number }; end: { line: number; character: number } };
		newText: string;
	}
	export const type: RequestType<DocumentSortingParams, ITextEdit[], any> = new RequestType('json/sort');
}
```

**Variations / call-sites:**
- Server handles matching types in `extensions/json-language-features/server/src/jsonServer.ts:23-64`
- Middleware converts protocol responses via `client.protocol2CodeConverter.asDiagnostic` (line 250)

---

#### Pattern: Middleware Chaining for Language Features

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:294-384`

**What:** Middleware object chaining multiple LSP capability providers with protocol conversion and limit checking.

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
		provideCompletionItem: (document, position, context, token, next) => { /* ... */ },
		provideHover: (document, position, token, next) => { /* ... */ },
		provideDocumentSymbols: (document, token, next) => { /* ... */ }
	}
};
```

**Variations / call-sites:**
- Separate middleware for completions, hover, folding, colors, symbols (lines 311-383)
- Document symbol provider includes limit tracking (line 362-383)

---

#### Pattern: Schema Request Service Abstraction

**Where:** `extensions/json-language-features/client/src/node/jsonClientMain.ts:86-175`

**What:** Abstracted schema request service with caching, ETag validation, and HTTP request handling.

```typescript
async function getSchemaRequestService(context: ExtensionContext, log: LogOutputChannel): Promise<SchemaRequestService> {
	let cache: JSONSchemaCache | undefined = undefined;
	const globalStorage = context.globalStorageUri;

	let clearCache: (() => Promise<string[]>) | undefined;
	if (globalStorage.scheme === 'file') {
		const schemaCacheLocation = path.join(globalStorage.fsPath, 'json-schema-cache');
		await fs.mkdir(schemaCacheLocation, { recursive: true });

		const schemaCache = new JSONSchemaCache(schemaCacheLocation, context.globalState);
		cache = schemaCache;
		clearCache = async () => {
			const cachedSchemas = await schemaCache.clearCache();
			log.trace(`[json schema cache] cache cleared. Previously cached schemas: ${cachedSchemas.join(', ')}`);
			return cachedSchemas;
		};
	}

	const request = async (uri: string, etag?: string): Promise<string> => {
		const headers: Headers = {
			'Accept-Encoding': 'gzip, deflate',
			'User-Agent': `${env.appName} (${env.appHost})`
		};
		if (etag) {
			headers['If-None-Match'] = etag;
		}
		try {
			log.trace(`[json schema cache] Requesting schema ${uri} etag ${etag}...`);
			const response = await xhr({ url: uri, followRedirects: 5, headers });
			if (cache) {
				const etag = response.headers['etag'];
				if (typeof etag === 'string') {
					log.trace(`[json schema cache] Storing schema ${uri} etag ${etag} in cache`);
					await cache.putSchema(uri, etag, response.responseText);
				}
			}
			return response.responseText;
		} catch (error: unknown) {
			if (isXHRResponse(error)) {
				if (error.status === 304 && etag && cache) {
					const content = await cache.getSchema(uri, etag, true);
					if (content) {
						return content;
					}
					return request(uri);
				}
				throw status;
			}
			throw error;
		}
	};

	return {
		getContent: async (uri: string) => {
			if (cache && /^https?:\/\/(json|www)\.schemastore\.org\//.test(uri)) {
				const content = await cache.getSchemaIfUpdatedSince(uri, retryTimeoutInHours);
				if (content) {
					return content;
				}
			}
			return request(uri, cache?.getETag(uri));
		},
		clearCache
	};
}
```

**Variations / call-sites:**
- Browser variant uses fetch API: `extensions/json-language-features/client/src/browser/jsonClientMain.ts:24-31`
- Node server implements RequestService in `extensions/json-language-features/server/src/node/jsonServerMain.ts:27-56`

---

#### Pattern: Language Model Cache with TTL

**Where:** `extensions/json-language-features/server/src/languageModelCache.ts:14-82`

**What:** Generic document-keyed cache with version tracking, TTL, and automatic eviction on overflow.

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
			const languageId = document.languageId;
			const languageModelInfo = languageModels[document.uri];
			if (languageModelInfo && languageModelInfo.version === version && languageModelInfo.languageId === languageId) {
				languageModelInfo.cTime = Date.now();
				return languageModelInfo.languageModel;
			}
			const languageModel = parse(document);
			languageModels[document.uri] = { languageModel, version, languageId, cTime: Date.now() };
			if (!languageModelInfo) {
				nModels++;
			}

			if (nModels > maxEntries) {
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
		},
		onDocumentRemoved(document: TextDocument) {
			const uri = document.uri;
			if (languageModels[uri]) {
				delete languageModels[uri];
				nModels--;
			}
		},
		dispose() {
			if (typeof cleanupInterval !== 'undefined') {
				clearInterval(cleanupInterval);
			}
		}
	};
}
```

**Variations / call-sites:**
- Instantiated in server with `getLanguageModelCache<JSONDocument>(10, 60, document => languageService.parseJSONDocument(document))` (line 427)
- Called from request handlers via `jsonDocuments.get(document)` (line 436)

---

#### Pattern: Push vs Pull Diagnostics Registration

**Where:** `extensions/json-language-features/server/src/utils/validation.ts:17-75`

**What:** Push-based diagnostics pattern with debounced validation and pending request tracking.

```typescript
export function registerDiagnosticsPushSupport(documents: TextDocuments<TextDocument>, connection: Connection, runtime: RuntimeEnvironment, validate: Validator): DiagnosticsSupport {

	const pendingValidationRequests: { [uri: string]: Disposable } = {};
	const validationDelayMs = 500;

	const disposables: Disposable[] = [];

	documents.onDidChangeContent(change => {
		triggerValidation(change.document);
	}, undefined, disposables);

	documents.onDidClose(event => {
		cleanPendingValidation(event.document);
		connection.sendDiagnostics({ uri: event.document.uri, diagnostics: [] });
	}, undefined, disposables);

	function cleanPendingValidation(textDocument: TextDocument): void {
		const request = pendingValidationRequests[textDocument.uri];
		if (request) {
			request.dispose();
			delete pendingValidationRequests[textDocument.uri];
		}
	}

	function triggerValidation(textDocument: TextDocument): void {
		cleanPendingValidation(textDocument);
		const request = pendingValidationRequests[textDocument.uri] = runtime.timer.setTimeout(async () => {
			if (request === pendingValidationRequests[textDocument.uri]) {
				try {
					const diagnostics = await validate(textDocument);
					if (request === pendingValidationRequests[textDocument.uri]) {
						connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
					}
					delete pendingValidationRequests[textDocument.uri];
				} catch (e) {
					connection.console.error(formatError(`Error while validating ${textDocument.uri}`, e));
				}
			}
		}, validationDelayMs);
	}

	return {
		requestRefresh: () => {
			documents.all().forEach(triggerValidation);
		},
		dispose: () => {
			disposables.forEach(d => d.dispose());
			const keys = Object.keys(pendingValidationRequests);
			for (const key of keys) {
				pendingValidationRequests[key].dispose();
				delete pendingValidationRequests[key];
			}
		}
	};
}
```

**Variations / call-sites:**
- Pull variant uses `connection.languages.diagnostics.on()` (line 86)
- Server selects at runtime in `jsonServer.ts:177-182` based on client capabilities

---

#### Pattern: Safe Error Handling with Cancellation

**Where:** `extensions/json-language-features/server/src/utils/runner.ts:21-65`

**What:** Async error wrapper with cancellation token checking and safe fallback error values.

```typescript
export function runSafeAsync<T>(runtime: RuntimeEnvironment, func: () => Thenable<T>, errorVal: T, errorMessage: string, token: CancellationToken): Thenable<T | ResponseError<any>> {
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
				resolve(errorVal);
			});
		});
	});
}

export function runSafe<T, E>(runtime: RuntimeEnvironment, func: () => T, errorVal: T, errorMessage: string, token: CancellationToken): Thenable<T | ResponseError<E>> {
	return new Promise<T | ResponseError<E>>((resolve) => {
		runtime.timer.setImmediate(() => {
			if (token.isCancellationRequested) {
				resolve(cancelValue());
			} else {
				try {
					const result = func();
					if (token.isCancellationRequested) {
						resolve(cancelValue());
						return;
					} else {
						resolve(result);
					}
				} catch (e) {
					console.error(formatError(errorMessage, e));
					resolve(errorVal);
				}
			}
		});
	});
}
```

**Variations / call-sites:**
- Used in all LSP handlers: completions (line 440), hover (line 451), symbols (line 462), code actions (line 477), formatting (line 507-511), colors (line 515), links (line 562)
- Sync variant for simple operations, async variant for async handlers

---

#### Pattern: Schema Cache with ETag and Persistence

**Where:** `extensions/json-language-features/client/src/node/schemaCache.ts:23-147`

**What:** Persisted cache layer with ETag validation, expiration windows, and filesystem storage.

```typescript
export class JSONSchemaCache {
	private cacheInfo: CacheInfo;

	constructor(private readonly schemaCacheLocation: string, private readonly globalState: Memento) {
		const infos = globalState.get<CacheInfo>(MEMENTO_KEY, {}) as CacheInfo;
		const validated: CacheInfo = {};
		for (const schemaUri in infos) {
			const { etag, fileName, updateTime } = infos[schemaUri];
			if (typeof etag === 'string' && typeof fileName === 'string' && typeof updateTime === 'number') {
				validated[schemaUri] = { etag, fileName, updateTime };
			}
		}
		this.cacheInfo = validated;
	}

	async putSchema(schemaUri: string, etag: string, schemaContent: string): Promise<void> {
		try {
			const fileName = getCacheFileName(schemaUri);
			await fs.writeFile(path.join(this.schemaCacheLocation, fileName), schemaContent);
			const entry: CacheEntry = { etag, fileName, updateTime: new Date().getTime() };
			this.cacheInfo[schemaUri] = entry;
		} catch (e) {
			delete this.cacheInfo[schemaUri];
		} finally {
			await this.updateMemento();
		}
	}

	async getSchemaIfUpdatedSince(schemaUri: string, expirationDurationInHours: number): Promise<string | undefined> {
		const lastUpdatedInHours = this.getLastUpdatedInHours(schemaUri);
		if (lastUpdatedInHours !== undefined && (lastUpdatedInHours < expirationDurationInHours)) {
			return this.loadSchemaFile(schemaUri, this.cacheInfo[schemaUri], false);
		}
		return undefined;
	}

	async getSchema(schemaUri: string, etag: string, etagValid: boolean): Promise<string | undefined> {
		const cacheEntry = this.cacheInfo[schemaUri];
		if (cacheEntry) {
			if (cacheEntry.etag === etag) {
				return this.loadSchemaFile(schemaUri, cacheEntry, etagValid);
			} else {
				this.deleteSchemaFile(schemaUri, cacheEntry);
			}
		}
		return undefined;
	}

	async clearCache(): Promise<string[]> {
		const uris = Object.keys(this.cacheInfo);
		try {
			const files = await fs.readdir(this.schemaCacheLocation);
			for (const file of files) {
				try {
					await fs.unlink(path.join(this.schemaCacheLocation, file));
				} catch (_e) {
					// ignore
				}
			}
		} catch (e) {
			// ignore
		} finally {
			this.cacheInfo = {};
			await this.updateMemento();
		}
		return uris;
	}
}

function getCacheFileName(uri: string): string {
	return `${createHash('sha256').update(uri).digest('hex')}.schema.json`;
}
```

**Variations / call-sites:**
- Instantiated in `jsonClientMain.ts:95` with global storage path
- Used in request function to validate responses and serve cached content

---

## Summary

The JSON language features extension demonstrates **mature LSP client/server integration patterns** spanning:

1. **Dual-platform adaptation** (Node/Browser) through abstracted request services and runtime environments
2. **Custom LSP extensions** using namespace-scoped request/notification types
3. **Middleware layering** for protocol translation, caching, and diagnostics enhancement
4. **Intelligent caching** with ETag validation, TTL expiration, and hash-based storage
5. **Robust async patterns** with cancellation support, debouncing, and safe error handling
6. **Dynamic capability negotiation** (push vs pull diagnostics, formatter registration)
7. **Configuration-driven behavior** with workspace/folder-scoped settings inspection
8. **Extension ecosystem integration** through package.json contribution discovery

For Tauri/Rust ports, key translation points include:
- LSP middleware chains map to Rust trait implementations or middleware stacks
- Cache patterns port to persistent KV stores (SQLite, RocksDB)
- Runtime abstraction enables both IPC and WebSocket transports
- Cancellation tokens translate directly to async cancel safety mechanisms
