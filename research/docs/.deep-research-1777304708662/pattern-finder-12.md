# Pattern Findings: HTML Language Features Extension

## Research Question
What it would take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

## Scope
`extensions/html-language-features/` — 51 files, 9,248 LOC

---

## LanguageClient Instantiation Patterns

#### Pattern: Node-based Language Client Initialization
**Where:** `extensions/html-language-features/client/src/node/htmlClientMain.ts:36-38`
**What:** Creates a language client for the Node runtime using IPC transport with separate debug/run configurations.
```typescript
const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
	return new LanguageClient(id, name, serverOptions, clientOptions);
};
```

**Context (lines 31-34):**
```typescript
const serverOptions: ServerOptions = {
	run: { module: serverModule, transport: TransportKind.ipc },
	debug: { module: serverModule, transport: TransportKind.ipc, options: debugOptions }
};
```

**Variations / call-sites:**
- `extensions/html-language-features/client/src/browser/htmlClientMain.ts:20-21` — Browser-based client using Web Worker

#### Pattern: Browser-based Language Client (Web Worker)
**Where:** `extensions/html-language-features/client/src/browser/htmlClientMain.ts:20-22`
**What:** Creates a browser client using Web Worker messaging instead of IPC transport.
```typescript
const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
	return new LanguageClient(id, name, worker, clientOptions);
};
```

**Full context (lines 14-22):**
```typescript
const serverMain = Uri.joinPath(context.extensionUri, 'server/dist/browser/htmlServerMain.js');
try {
	const worker = new Worker(serverMain.toString());
	worker.postMessage({ i10lLocation: l10n.uri?.toString(false) ?? '' });

	const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
		return new LanguageClient(id, name, worker, clientOptions);
	};
```

---

## Language Client Configuration & Middleware

#### Pattern: Language Client Options with Custom Middleware
**Where:** `extensions/html-language-features/client/src/htmlClient.ts:156-192`
**What:** Complex clientOptions with synchronization settings, initialization options, and LSP middleware for completion items.
```typescript
const clientOptions: LanguageClientOptions = {
	documentSelector,
	synchronize: {
		configurationSection: ['html', 'css', 'javascript', 'js/ts'],
	},
	initializationOptions: {
		embeddedLanguages,
		handledSchemas: ['file'],
		provideFormatter: false,
		customCapabilities: { rangeFormatting: { editLimit: 10000 } }
	},
	middleware: {
		provideCompletionItem(document: TextDocument, position: Position, context: CompletionContext, token: CancellationToken, next: ProvideCompletionItemsSignature): ProviderResult<CompletionItem[] | CompletionList> {
			function updateRanges(item: CompletionItem) {
				const range = item.range;
				if (range instanceof Range && range.end.isAfter(position) && range.start.isBeforeOrEqual(position)) {
					item.range = { inserting: new Range(range.start, position), replacing: range };
				}
			}
			function updateProposals(r: CompletionItem[] | CompletionList | null | undefined): CompletionItem[] | CompletionList | null | undefined {
				if (r) {
					(Array.isArray(r) ? r : r.items).forEach(updateRanges);
				}
				return r;
			}
			const r = next(document, position, context, token);
			if (isThenable<CompletionItem[] | CompletionList | null | undefined>(r)) {
				return r.then(updateProposals);
			}
			return updateProposals(r);
		}
	}
};
```

**Key aspects:**
- Middleware intercepts LSP responses to transform completion ranges
- Supports embedding multiple languages (CSS, JavaScript in HTML)
- Initialization options pass capabilities and schemas to server

---

## Server-Side Language Mode Architecture

#### Pattern: Pluggable Language Mode Interface
**Where:** `extensions/html-language-features/server/src/modes/languageModes.ts:70-96`
**What:** Interface-driven architecture allowing multiple language modes (HTML, CSS, JavaScript) to implement common LSP operations.
```typescript
export interface LanguageMode {
	getId(): string;
	getSelectionRange?: (document: TextDocument, position: Position) => Promise<SelectionRange>;
	doValidation?: (document: TextDocument, settings?: Settings) => Promise<Diagnostic[]>;
	doComplete?: (document: TextDocument, position: Position, documentContext: DocumentContext, settings?: Settings) => Promise<CompletionList>;
	doResolve?: (document: TextDocument, item: CompletionItem) => Promise<CompletionItem>;
	doHover?: (document: TextDocument, position: Position, settings?: Settings) => Promise<Hover | null>;
	doSignatureHelp?: (document: TextDocument, position: Position) => Promise<SignatureHelp | null>;
	doRename?: (document: TextDocument, position: Position, newName: string) => Promise<WorkspaceEdit | null>;
	doLinkedEditing?: (document: TextDocument, position: Position) => Promise<Range[] | null>;
	findDocumentHighlight?: (document: TextDocument, position: Position) => Promise<DocumentHighlight[]>;
	findDocumentSymbols?: (document: TextDocument) => Promise<SymbolInformation[]>;
	findDocumentLinks?: (document: TextDocument, documentContext: DocumentContext) => Promise<DocumentLink[]>;
	findDefinition?: (document: TextDocument, position: Position) => Promise<Definition | null>;
	findReferences?: (document: TextDocument, position: Position) => Promise<Location[]>;
	format?: (document: TextDocument, range: Range, options: FormattingOptions, settings?: Settings) => Promise<TextEdit[]>;
	findDocumentColors?: (document: TextDocument) => Promise<ColorInformation[]>;
	getColorPresentations?: (document: TextDocument, color: Color, range: Range) => Promise<ColorPresentation[]>;
	doAutoInsert?: (document: TextDocument, position: Position, kind: 'autoClose' | 'autoQuote') => Promise<string | null>;
	findMatchingTagPosition?: (document: TextDocument, position: Position) => Promise<Position | null>;
	getFoldingRanges?: (document: TextDocument) => Promise<FoldingRange[]>;
	onDocumentRemoved(document: TextDocument): void;
	getSemanticTokens?(document: TextDocument): Promise<SemanticTokenData[]>;
	getSemanticTokenLegend?(): { types: string[]; modifiers: string[] };
	getTextDocumentContent?(uri: DocumentUri): Promise<string | undefined>;
	dispose(): void;
}
```

#### Pattern: Language Mode Factory Registration
**Where:** `extensions/html-language-features/server/src/modes/languageModes.ts:116-133`
**What:** Factory function creates and registers individual language modes based on supported languages.
```typescript
export function getLanguageModes(supportedLanguages: { [languageId: string]: boolean }, workspace: Workspace, clientCapabilities: HtmlClientCapabilities, requestService: FileSystemProvider): LanguageModes {
	const htmlLanguageService = getHTMLLanguageService({ clientCapabilities, fileSystemProvider: requestService });
	const cssLanguageService = getCSSLanguageService({ clientCapabilities, fileSystemProvider: requestService });

	const documentRegions = getLanguageModelCache<HTMLDocumentRegions>(10, 60, document => getDocumentRegions(htmlLanguageService, document));

	let modelCaches: LanguageModelCache<any>[] = [];
	modelCaches.push(documentRegions);

	let modes = Object.create(null);
	modes['html'] = getHTMLMode(htmlLanguageService, workspace);
	if (supportedLanguages['css']) {
		modes['css'] = getCSSMode(cssLanguageService, documentRegions, workspace);
	}
	if (supportedLanguages['javascript']) {
		modes['javascript'] = getJavaScriptMode(documentRegions, 'javascript', workspace);
		modes['typescript'] = getJavaScriptMode(documentRegions, 'typescript', workspace);
	}
```

**Key aspects:**
- Languages conditionally registered based on capabilities
- Each mode gets dedicated language service and model caches
- Embedded languages supported within HTML documents

#### Pattern: Language Mode Delegation
**Where:** `extensions/html-language-features/server/src/modes/languageModes.ts:138-143`
**What:** Runtime dispatch to correct language mode based on cursor position.
```typescript
getModeAtPosition(document: TextDocument, position: Position): LanguageMode | undefined {
	const languageId = documentRegions.get(document).getLanguageAtPosition(position);
	if (languageId) {
		return modes[languageId];
	}
	return undefined;
},
```

---

## Language Model Caching

#### Pattern: Versioned Document Cache with LRU Eviction
**Where:** `extensions/html-language-features/server/src/languageModelCache.ts:14-82`
**What:** Intelligent caching of parsed language models with version tracking and LRU-based cleanup.
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
		},
```

**Key aspects:**
- Dual validation: version + languageId check
- Time-based background cleanup with configurable intervals
- Size-bounded cache with LRU eviction when full
- Used for HTML documents, CSS, and JavaScript ASTs

---

## Connection & LSP Handler Registration

#### Pattern: Node-based Server Connection
**Where:** `extensions/html-language-features/server/src/node/htmlServerMain.ts:6-36`
**What:** Low-level server setup for Node runtime using stdio-based message protocol.
```typescript
import { createConnection, Connection, Disposable } from 'vscode-languageserver/node';
import { formatError } from '../utils/runner.js';
import { RuntimeEnvironment, startServer } from '../htmlServer.js';
import { getNodeFileFS } from './nodeFs.js';

// Create a connection for the server.
const connection: Connection = createConnection();

console.log = connection.console.log.bind(connection.console);
console.error = connection.console.error.bind(connection.console);

process.on('unhandledRejection', (e: any) => {
	connection.console.error(formatError(`Unhandled exception`, e));
});

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
	fileFs: getNodeFileFS()
};

startServer(connection, runtime);
```

#### Pattern: Browser-based Server Connection
**Where:** `extensions/html-language-features/server/src/browser/htmlServerMain.ts:6-30`
**What:** Web Worker message-based server using browser APIs instead of Node syscalls.
```typescript
import { createConnection, BrowserMessageReader, BrowserMessageWriter, Disposable } from 'vscode-languageserver/browser';
import { RuntimeEnvironment, startServer } from '../htmlServer.js';

const messageReader = new BrowserMessageReader(self);
const messageWriter = new BrowserMessageWriter(self);

const connection = createConnection(messageReader, messageWriter);

console.log = connection.console.log.bind(connection.console);
console.error = connection.console.error.bind(connection.console);

const runtime: RuntimeEnvironment = {
	timer: {
		setImmediate(callback: (...args: any[]) => void, ...args: any[]): Disposable {
			const handle = setTimeout(callback, 0, ...args);
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

#### Pattern: LSP Handler Registration Pattern
**Where:** `extensions/html-language-features/server/src/htmlServer.ts:139-200, 307-325`
**What:** Comprehensive LSP request/notification handler registration covering all IDE features.
```typescript
connection.onInitialize((params: InitializeParams): InitializeResult => {
	// ... initialization ...
	return { capabilities };
});

connection.onCompletion(async (textDocumentPosition, token) => {
	return runSafe(runtime, async () => {
		const document = documents.get(textDocumentPosition.textDocument.uri);
		if (!document) {
			return null;
		}
		const mode = languageModes.getModeAtPosition(document, textDocumentPosition.position);
		if (mode?.doComplete) {
			return mode.doComplete(document, textDocumentPosition.position, getDocumentContext(document.uri, workspaceFolders, languageModes), getDocumentSettings(document, () => mode.doComplete !== undefined));
		}
		return null;
	}, null, `Error while computing completions for ${textDocumentPosition.textDocument.uri}`, token);
});

connection.onHover((textDocumentPosition, token) => {
	return runSafe(runtime, async () => {
		const document = documents.get(textDocumentPosition.textDocument.uri);
		if (document) {
			const mode = languageModes.getModeAtPosition(document, textDocumentPosition.position);
			if (mode?.doHover) {
				return mode.doHover(document, textDocumentPosition.position, getDocumentSettings(document, () => mode.doHover !== undefined));
			}
		}
		return null;
	}, null, `Error while computing hover for ${textDocumentPosition.textDocument.uri}`, token);
});
```

**Registered handlers include:**
- `onInitialize` — Capability negotiation
- `onCompletion` — Completion items
- `onCompletionResolve` — Full completion details
- `onHover` — Hover information
- `onDocumentHighlight` — Selection highlights
- `onDefinition` — Go to definition
- `onReferences` — Find references
- `onSignatureHelp` — Function signature help
- `onDocumentRangeFormatting` — Range formatting
- `onDocumentFormatting` — Full document formatting
- `onDocumentLinks` — Document link navigation
- `onDocumentSymbol` — Symbol outline
- `onRequest(DocumentColorRequest.type)` — Color information
- `onRequest(AutoInsertRequest.type)` — Smart auto-insertion
- `onFoldingRanges` — Code folding regions
- `onSelectionRanges` — Smart selection expansion
- `onRenameRequest` — Symbol rename

---

## Custom Request/Notification Patterns

#### Pattern: Bidirectional Custom LSP Types
**Where:** `extensions/html-language-features/server/src/htmlServer.ts:34-71`
**What:** Custom namespaced request and notification types extending LSP protocol.
```typescript
namespace CustomDataChangedNotification {
	export const type: NotificationType<string[]> = new NotificationType('html/customDataChanged');
}

namespace CustomDataContent {
	export const type: RequestType<string, string, any> = new RequestType('html/customDataContent');
}

interface AutoInsertParams {
	kind: 'autoQuote' | 'autoClose';
	textDocument: TextDocumentIdentifier;
	position: Position;
}

namespace AutoInsertRequest {
	export const type: RequestType<AutoInsertParams, string | null, any> = new RequestType('html/autoInsert');
}

interface SemanticTokenParams {
	textDocument: TextDocumentIdentifier;
	ranges?: Range[];
}
namespace SemanticTokenRequest {
	export const type: RequestType<SemanticTokenParams, number[] | null, any> = new RequestType('html/semanticTokens');
}
```

**Usage pattern (lines 205-208):**
```typescript
client.sendNotification(CustomDataChangedNotification.type, customDataSource.uris);
customDataSource.onDidChange(() => {
	client.sendNotification(CustomDataChangedNotification.type, customDataSource.uris);
}, undefined, toDispose);
```

**Server-side handler (htmlServer.ts:590-593):**
```typescript
connection.onNotification(CustomDataChangedNotification.type, dataPaths => {
	fetchHTMLDataProviders(dataPaths, customDataRequestService).then(dataProviders => {
		languageModes.updateDataProviders(dataProviders);
	});
});
```

#### Pattern: File System Request Abstraction
**Where:** `extensions/html-language-features/client/src/requests.ts:18-35, server/src/requests.ts:61-78`
**What:** Custom request types for filesystem operations bridging client and server.
```typescript
// Client side
export namespace FsStatRequest {
	export const type: RequestType<string, FileStat, any> = new RequestType('fs/stat');
}

export namespace FsReadDirRequest {
	export const type: RequestType<string, [string, FileType][], any> = new RequestType('fs/readDir');
}

export function serveFileSystemRequests(client: BaseLanguageClient, runtime: Runtime): Disposable {
	const disposables = [];
	disposables.push(client.onRequest(FsReadDirRequest.type, (uriString: string) => {
		const uri = Uri.parse(uriString);
		if (uri.scheme === 'file' && runtime.fileFs) {
			return runtime.fileFs.readDirectory(uriString);
		}
		return workspace.fs.readDirectory(uri);
	}));
	disposables.push(client.onRequest(FsStatRequest.type, (uriString: string) => {
		const uri = Uri.parse(uriString);
		if (uri.scheme === 'file' && runtime.fileFs) {
			return runtime.fileFs.stat(uriString);
		}
		return workspace.fs.stat(uri);
	}));
	return Disposable.from(...disposables);
}

// Server side handler
export function getFileSystemProvider(handledSchemas: string[], connection: Connection, runtime: RuntimeEnvironment): FileSystemProvider {
	const fileFs = runtime.fileFs && handledSchemas.indexOf('file') !== -1 ? runtime.fileFs : undefined;
	return {
		async stat(uri: string): Promise<FileStat> {
			if (fileFs && uri.startsWith('file:')) {
				return fileFs.stat(uri);
			}
			const res = await connection.sendRequest(FsStatRequest.type, uri.toString());
			return res;
		},
		readDirectory(uri: string): Promise<[string, FileType][]> {
			if (fileFs && uri.startsWith('file:')) {
				return fileFs.readDirectory(uri);
			}
			return connection.sendRequest(FsReadDirRequest.type, uri.toString());
		}
	};
}
```

---

## Provider Registration & Dynamic Features

#### Pattern: Dynamic Formatter Registration
**Where:** `extensions/html-language-features/client/src/htmlClient.ts:259-288`
**What:** Runtime registration/deregistration of formatting providers based on configuration changes.
```typescript
function updateFormatterRegistration() {
	const formatEnabled = workspace.getConfiguration().get(SettingIds.formatEnable);
	if (!formatEnabled && rangeFormatting) {
		rangeFormatting.dispose();
		rangeFormatting = undefined;
	} else if (formatEnabled && !rangeFormatting) {
		rangeFormatting = languages.registerDocumentRangeFormattingEditProvider(documentSelector, {
			provideDocumentRangeFormattingEdits(document: TextDocument, range: Range, options: FormattingOptions, token: CancellationToken): ProviderResult<TextEdit[]> {
				const filesConfig = workspace.getConfiguration('files', document);
				const fileFormattingOptions = {
					trimTrailingWhitespace: filesConfig.get<boolean>('trimTrailingWhitespace'),
					trimFinalNewlines: filesConfig.get<boolean>('trimFinalNewlines'),
					insertFinalNewline: filesConfig.get<boolean>('insertFinalNewline'),
				};
				const params: DocumentRangeFormattingParams = {
					textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(document),
					range: client.code2ProtocolConverter.asRange(range),
					options: client.code2ProtocolConverter.asFormattingOptions(options, fileFormattingOptions)
				};
				return client.sendRequest(DocumentRangeFormattingRequest.type, params, token).then(
					client.protocol2CodeConverter.asTextEdits,
					(error) => {
						client.handleFailedRequest(DocumentRangeFormattingRequest.type, undefined, error, []);
						return Promise.resolve([]);
					}
				);
			}
		});
	}
}
```

#### Pattern: Semantic Tokens Provider Registration
**Where:** `extensions/html-language-features/client/src/htmlClient.ts:234-257`
**What:** Asynchronous registration of semantic tokens after server initialization with legend negotiation.
```typescript
client.sendRequest(SemanticTokenLegendRequest.type).then(legend => {
	if (legend) {
		const provider: DocumentSemanticTokensProvider & DocumentRangeSemanticTokensProvider = {
			provideDocumentSemanticTokens(doc) {
				const params: SemanticTokenParams = {
					textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(doc),
				};
				return client.sendRequest(SemanticTokenRequest.type, params).then(data => {
					return data && new SemanticTokens(new Uint32Array(data));
				});
			},
			provideDocumentRangeSemanticTokens(doc, range) {
				const params: SemanticTokenParams = {
					textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(doc),
					ranges: [client.code2ProtocolConverter.asRange(range)]
				};
				return client.sendRequest(SemanticTokenRequest.type, params).then(data => {
					return data && new SemanticTokens(new Uint32Array(data));
				});
			}
		};
		toDispose.push(languages.registerDocumentSemanticTokensProvider(documentSelector, provider, new SemanticTokensLegend(legend.types, legend.modifiers)));
	}
});
```

#### Pattern: Completion Item Provider with Client-side Snippets
**Where:** `extensions/html-language-features/client/src/htmlClient.ts:292-339`
**What:** Client-side only completion provider for HTML structure snippets registered alongside LSP completions.
```typescript
toDispose.push(languages.registerCompletionItemProvider(documentSelector, {
	provideCompletionItems(doc, pos) {
		const results: CompletionItem[] = [];
		const lineUntilPos = doc.getText(new Range(new Position(pos.line, 0), pos));
		const match = lineUntilPos.match(regionCompletionRegExpr);
		if (match) {
			const range = new Range(new Position(pos.line, match[1].length), pos);
			const beginProposal = new CompletionItem('#region', CompletionItemKind.Snippet);
			beginProposal.range = range;
			beginProposal.insertText = new SnippetString('<!-- #region $1-->');
			beginProposal.documentation = l10n.t('Folding Region Start');
			beginProposal.filterText = match[2];
			beginProposal.sortText = 'za';
			results.push(beginProposal);
			// ...
		}
		return results;
	}
}));
```

---

## Auto-Insertion Pattern

#### Pattern: Event-driven Auto-Insert on Text Changes
**Where:** `extensions/html-language-features/client/src/autoInsertion.ts:46-84`
**What:** Configurable auto-insertion of quotes and closing tags triggered by specific character input.
```typescript
function onDidChangeTextDocument({ document, contentChanges, reason }: TextDocumentChangeEvent) {
	if (!anyIsEnabled || contentChanges.length === 0 || reason === TextDocumentChangeReason.Undo || reason === TextDocumentChangeReason.Redo) {
		return;
	}
	const activeDocument = window.activeTextEditor && window.activeTextEditor.document;
	if (document !== activeDocument) {
		return;
	}
	if (timeout) {
		timeout.dispose();
	}

	const lastChange = contentChanges[contentChanges.length - 1];
	if (lastChange.rangeLength === 0 && isSingleLine(lastChange.text)) {
		const lastCharacter = lastChange.text[lastChange.text.length - 1];
		if (isEnabled['autoQuote'] && lastCharacter === '=') {
			doAutoInsert('autoQuote', document, lastChange);
		} else if (isEnabled['autoClose'] && (lastCharacter === '>' || lastCharacter === '/')) {
			doAutoInsert('autoClose', document, lastChange);
		}
	}
}

function doAutoInsert(kind: 'autoQuote' | 'autoClose', document: TextDocument, lastChange: TextDocumentContentChangeEvent) {
	const rangeStart = lastChange.range.start;
	const version = document.version;
	timeout = runtime.timer.setTimeout(() => {
		const position = new Position(rangeStart.line, rangeStart.character + lastChange.text.length);
		provider(kind, document, position).then(text => {
			if (text && isEnabled[kind]) {
				// insertion logic
			}
		});
	}, 25);
}
```

---

## Runtime Abstraction

#### Pattern: Runtime Environment Interface
**Where:** `extensions/html-language-features/server/src/htmlServer.ts:73-80, client/src/htmlClient.ts:79-86`
**What:** Abstraction layer allowing platform-agnostic code to use timers and filesystem regardless of Node/Browser context.
```typescript
// Server-side
export interface RuntimeEnvironment {
	fileFs?: FileSystemProvider;
	configureHttpRequests?(proxy: string | undefined, strictSSL: boolean): void;
	readonly timer: {
		setImmediate(callback: (...args: any[]) => void, ...args: any[]): Disposable;
		setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable;
	};
}

// Client-side
export interface Runtime {
	TextDecoder: typeof TextDecoder;
	fileFs?: FileSystemProvider;
	telemetry?: TelemetryReporter;
	readonly timer: {
		setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable;
	};
}
```

**Node implementation (server/src/node/htmlServerMain.ts:22-34):**
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
	fileFs: getNodeFileFS()
};
```

**Browser implementation (server/src/browser/htmlServerMain.ts:17-28):**
```typescript
const runtime: RuntimeEnvironment = {
	timer: {
		setImmediate(callback: (...args: any[]) => void, ...args: any[]): Disposable {
			const handle = setTimeout(callback, 0, ...args);
			return { dispose: () => clearTimeout(handle) };
		},
		setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable {
			const handle = setTimeout(callback, ms, ...args);
			return { dispose: () => clearTimeout(handle) };
		}
	}
};
```

---

## Summary

The HTML language features extension demonstrates **critical architectural patterns for porting VS Code to Tauri/Rust**:

1. **Language Client Abstraction**: The `LanguageClientConstructor` type enables runtime selection between Node IPC and Web Worker transports, crucial for desktop/web portability.

2. **LSP as Core Protocol**: All IDE features (completions, hover, navigation, formatting, refactoring) are implemented exclusively through Language Server Protocol, not direct API calls.

3. **Pluggable Language Modes**: The `LanguageMode` interface enables adding new language support without core changes—essential for modular architecture across platforms.

4. **Runtime Environment Abstraction**: Filesystem, timers, and messaging are abstracted via `RuntimeEnvironment` interfaces, enabling node/browser/rust implementations to coexist.

5. **Custom LSP Extensions**: Beyond standard LSP, custom request/notification namespaces (`html/autoInsert`, `html/customDataChanged`, `fs/stat`) show how to extend the protocol for features like smart file system bridging.

6. **Caching Strategy**: The `LanguageModelCache` with LRU eviction and version tracking is essential for performance in large documents with embedded languages.

7. **Dynamic Provider Registration**: Configuration-driven registration/deregistration of providers (formatters, semantic tokens) allows runtime feature negotiation without client reload.

8. **Middleware Pattern**: LSP middleware intercepts and transforms completion ranges, demonstrating extensibility without modifying core protocol.

9. **Error Handling & Telemetry**: The `runSafe` wrapper pattern with error callbacks and telemetry reporting is consistent across all LSP handlers.

**Key observation for Rust/Tauri porting**: These patterns are **language-agnostic and transport-agnostic**. A Rust implementation would need to:
- Implement the `vscode-languageclient` semantic contract (or create Rust equivalent)
- Support both direct binary spawn (Node IPC equivalent) and message-based communication
- Implement the same `LanguageMode` plugin interface
- Maintain the same custom LSP request/notification namespaces
- Support runtime environment abstraction for filesystem and async operations
