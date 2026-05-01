# Partition 12 of 79 — Findings

## Scope
`extensions/html-language-features/` (51 files, 9,248 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# VS Code HTML Language Features Extension - Porting Analysis
## Partition 12: Core LSP Architecture & Implementation

### Implementation

#### Client-Side (LSP Language Client)
- `extensions/html-language-features/client/src/htmlClient.ts` - Main client initialization with startClient function (92+ lines establishing LanguageClient connection, middleware setup, semantic tokens, auto-insert handling)
- `extensions/html-language-features/client/src/node/htmlClientMain.ts` - Node.js entry point using TransportKind.ipc (IPC server spawning, debug server options, telemetry initialization)
- `extensions/html-language-features/client/src/browser/htmlClientMain.ts` - Browser entry point using Web Worker transport (postMessage-based communication)
- `extensions/html-language-features/client/src/autoInsertion.ts` - Auto-quote/auto-close request handling to LSP server
- `extensions/html-language-features/client/src/languageParticipants.ts` - Language ID participant management for multi-language support
- `extensions/html-language-features/client/src/customData.ts` - Custom HTML data provider loading and synchronization

#### Server-Side (LSP Language Server)
- `extensions/html-language-features/server/src/htmlServer.ts` - Core startServer function with complete LSP handler registration (600+ lines; implements onInitialize, onInitialized, onCompletion, onHover, onDocumentSymbol, onDefinition, onReferences, onRename, onFoldingRanges, onSemanticTokens, etc.)
- `extensions/html-language-features/server/src/node/htmlServerMain.ts` - Node.js server using createConnection from vscode-languageserver/node (IPC-based)
- `extensions/html-language-features/server/src/node/htmlServerNodeMain.ts` - Entry point wrapper for node build
- `extensions/html-language-features/server/src/browser/htmlServerMain.ts` - Browser server using BrowserMessageReader/BrowserMessageWriter for Web Worker communication
- `extensions/html-language-features/server/src/browser/htmlServerWorkerMain.ts` - Worker initialization point

#### File System & I/O Abstraction
- `extensions/html-language-features/server/src/requests.ts` - FileSystemProvider interface (stat, readDirectory) with request types (FsStatRequest, FsReadDirRequest) that bridge server→client for workspace filesystem access
- `extensions/html-language-features/client/src/requests.ts` - Client-side request handlers for filesystem operations using workspace.fs (with optional runtime.fileFs for overrides)
- `extensions/html-language-features/server/src/node/nodeFs.ts` - Node.js native fs wrapper providing FileStat interface implementation

#### Language Processing Pipeline
- `extensions/html-language-features/server/src/modes/languageModes.ts` - LanguageModes interface defining 20+ capability methods (doValidation, doComplete, doHover, doRename, getSemanticTokens, etc.) and LanguageMode abstraction
- `extensions/html-language-features/server/src/modes/htmlMode.ts` - HTML language mode via vscode-html-languageservice
- `extensions/html-language-features/server/src/modes/cssMode.ts` - Embedded CSS mode via vscode-css-languageservice
- `extensions/html-language-features/server/src/modes/javascriptMode.ts` - Embedded JavaScript mode
- `extensions/html-language-features/server/src/modes/embeddedSupport.ts` - HTMLDocumentRegions for parsing embedded language blocks
- `extensions/html-language-features/server/src/modes/formatting.ts` - Range/document formatting orchestration
- `extensions/html-language-features/server/src/modes/htmlFolding.ts` - Folding range computation
- `extensions/html-language-features/server/src/modes/selectionRanges.ts` - Selection range provider
- `extensions/html-language-features/server/src/modes/semanticTokens.ts` - Semantic tokens provider
- `extensions/html-language-features/server/src/modes/javascriptSemanticTokens.ts` - JavaScript semantic token specifics
- `extensions/html-language-features/server/src/modes/javascriptLibs.ts` - TypeScript library definitions injector

#### Utility & Infrastructure
- `extensions/html-language-features/server/src/utils/runner.ts` - runSafe function with error handling and token-based cancellation using RuntimeEnvironment.timer
- `extensions/html-language-features/server/src/utils/validation.ts` - Diagnostic support with push/pull modes (registerDiagnosticsPushSupport, registerDiagnosticsPullSupport) on 500ms debounce
- `extensions/html-language-features/server/src/utils/documentContext.ts` - Document context provider for relative path resolution
- `extensions/html-language-features/server/src/utils/positions.ts` - Position/range utilities
- `extensions/html-language-features/server/src/utils/strings.ts` - String manipulation helpers
- `extensions/html-language-features/server/src/utils/arrays.ts` - Array helpers (pushAll)
- `extensions/html-language-features/server/src/languageModelCache.ts` - Document cache with language mode tracking
- `extensions/html-language-features/server/src/customData.ts` - Custom HTML data provider fetching and JSON parsing

### Tests

- `extensions/html-language-features/server/src/test/completions.test.ts` - Completion behavior tests
- `extensions/html-language-features/server/src/test/documentContext.test.ts` - Document context utilities tests
- `extensions/html-language-features/server/src/test/embedded.test.ts` - Embedded language region parsing tests
- `extensions/html-language-features/server/src/test/folding.test.ts` - Folding range tests
- `extensions/html-language-features/server/src/test/selectionRanges.test.ts` - Selection range tests
- `extensions/html-language-features/server/src/test/rename.test.ts` - Rename refactoring tests
- `extensions/html-language-features/server/src/test/words.test.ts` - Word/identifier extraction tests
- `extensions/html-language-features/server/src/test/semanticTokens.test.ts` - Semantic tokens tests
- `extensions/html-language-features/server/src/test/formatting.test.ts` - Formatting tests
- `extensions/html-language-features/server/test/index.js` - Node.js test runner using native node:test with glob pattern discovery, JUnit/spec reporters

#### Test Fixtures
- `extensions/html-language-features/server/src/test/pathCompletionFixtures/` - Directory tree with `.html`, `.css`, `.js` files for path completion testing
- `extensions/html-language-features/server/src/test/fixtures/expected/` - Expected formatting outputs for HTML with various indentation
- `extensions/html-language-features/server/src/test/fixtures/inputs/` - HTML input files for formatting tests

### Types / Interfaces

#### Core Architecture Interfaces
- `RuntimeEnvironment` (htmlServer.ts) - Defines timer abstraction (setImmediate, setTimeout → Disposable), optional fileFs provider, optional configureHttpRequests
- `Runtime` (htmlClient.ts) - Client-side runtime with TextDecoder, fileFs, telemetry, timer
- `LanguageClientConstructor` - Factory function type for creating LanguageClient instances
- `AsyncDisposable` - Promise-based disposal interface
- `TelemetryReporter` - Telemetry event reporting interface
- `CustomDataRequestService` - getContent(uri: string) for fetching HTML data definitions

#### Document & Settings
- `Settings` interface - Scoped configuration with css, html, javascript, 'js/ts' properties
- `Workspace` interface - Settings + WorkspaceFolder[] container
- `FileSystemProvider` - stat() and readDirectory() abstraction

#### Language Mode Protocol
- `LanguageMode` - 20+ optional capability methods including doValidation, doComplete, doHover, doRename, findDocumentSymbols, findReferences, findDefinition, format, findDocumentLinks, getSemanticTokens, getTextDocumentContent
- `LanguageModes` - Aggregator interface for mode coordination (getModeAtPosition, getModeById, getAllModesInDocument, updateDataProviders)
- `CompletionItemData` - Completion item resolution metadata (languageId, uri, offset)
- `SemanticTokenData` - Position, length, typeIdx, modifierSet tuples

#### Request/Response Types
- `AutoInsertParams` - kind, textDocument, position (used for auto-quote/-close)
- `SemanticTokenParams` - textDocument, optional ranges[]
- `FsStatRequest`, `FsReadDirRequest` - File system request types registered in requests namespace
- `FileStat` - type, ctime, mtime, size metadata
- `FileType` enum - Unknown (0), File (1), Directory (2), SymbolicLink (64)

### Configuration

#### Root Extension Manifest
- `extensions/html-language-features/package.json` - Activates on onLanguage:html/handlebars; depends on vscode-languageclient@^10.0.0-next.10, vscode-uri; 44 HTML configuration properties (html.completion.attributeDefaultValue, html.format.*, html.suggest.*, html.validate.*, html.hover.*, html.trace.server, etc.)

#### Server Package
- `extensions/html-language-features/server/package.json` - Depends on vscode-languageserver@^10.0.0-next.16, vscode-css-languageservice@^7.0.0-next.1, vscode-html-languageservice@^6.0.0-next.1, vscode-languageserver-textdocument@^1.0.12, vscode-uri@^3.1.0

#### TypeScript Configuration
- `extensions/html-language-features/client/tsconfig.json` - ES2024 + webworker lib, nodenext module, includes vscode.d.ts type definitions
- `extensions/html-language-features/server/tsconfig.json` - ES2024 + WebWorker lib, esnext modules (for esm build)
- `extensions/html-language-features/client/tsconfig.browser.json` - Browser-specific configuration
- `extensions/html-language-features/server/tsconfig.browser.json` - Browser server configuration

#### Build Configuration
- `extensions/html-language-features/esbuild.mts` - Builds client (cjs→htmlClientMain) and server (esm→htmlServerNodeMain), sets external dependencies (vscode, typescript, fs), injects createRequire for CommonJS compat
- `extensions/html-language-features/esbuild.browser.mts` - Browser builds with javaScriptLibsPlugin that inlines TypeScript lib.d.ts definitions, loads jquery.d.ts
- `extensions/html-language-features/.vscode/launch.json` - Debug launch configurations
- `extensions/html-language-features/.vscode/tasks.json` - Build tasks
- `extensions/html-language-features/.vscode/settings.json` - Extension dev environment settings
- `extensions/html-language-features/.npmrc` - NPM configuration
- `extensions/html-language-features/.vscodeignore` - Files excluded from packaging

#### Schema & Validation
- `extensions/html-language-features/schemas/package.schema.json` - JSON schema for custom HTML data validation

### Examples / Fixtures

- `extensions/html-language-features/server/src/test/pathCompletionFixtures/` - File tree with about/, src/, index.html for path completion tests
- `extensions/html-language-features/server/src/test/fixtures/inputs/19813.html`, `21634.html` - Formatting test inputs
- `extensions/html-language-features/server/src/test/fixtures/expected/19813-tab.html`, `19813.html`, `19813-4spaces.html`, `21634.html` - Expected formatting outputs with tab/4-space/default indentation
- `extensions/html-language-features/icons/html.png` - Language icon

### Documentation

- `extensions/html-language-features/README.md` - Brief overview, points to CONTRIBUTING.md and vscode.com/docs/languages/html
- `extensions/html-language-features/CONTRIBUTING.md` - Setup instructions, debugging with Launch Extension, linking vscode-html-languageservice for development iteration, telemetry debugging via html.trace.server setting

### Notable Clusters

#### LSP Communication Layers
The codebase demonstrates a **three-tier LSP communication architecture**:
1. **IPC Layer (Node)**: htmlClientMain→htmlServerMain via spawn + TransportKind.ipc (stdio pipes)
2. **Worker Layer (Browser)**: htmlClientMain (browser)→htmlServerMain (worker) via postMessage
3. **Shared Abstraction**: All handlers routed through vscode-languageserver Connection interface

**Porting Challenge**: Replacing IPC and postMessage with Tauri's message channel, handling process lifecycle differences (Tauri spawned processes vs Node forking).

#### File System Abstraction Pattern
Server delegates filesystem operations back to client via request types (FsStatRequest, FsReadDirRequest). Client can service locally (runtime.fileFs) or via VS Code workspace.fs.

**Porting Implication**: Requires Tauri equivalent of workspace filesystem API; may need custom file access implementation if Tauri doesn't expose workspace context similarly.

#### Configuration Synchronization
Server receives scoped settings for css, html, javascript, js/ts sections via ConfigurationRequest; documents cached with associated settings for per-file behavior.

**Porting Note**: Tauri configuration binding would need similar section-based resolution.

#### Multi-Environment Builds
Single TypeScript source code split into:
- Node build: CommonJS client + ESM server (with require polyfill)
- Browser build: ESM client + Web Worker server (with inlined TypeScript libs)

**Porting Complexity**: Rust-based server eliminates the need for dual JS/TS builds, but client-side LSP handling remains TypeScript/JavaScript.

#### Embedded Language Mode System
LanguageModes coordinates HTML, CSS, and JavaScript parsing in single document. HTMLDocumentRegions identifies embedded blocks; each mode has independent provider chain (completion, validation, formatting, symbols, etc.).

**Porting Requirement**: Rust server must replicate this multiplexing; vscode-html-languageservice might be replaced with tree-sitter or custom HTML parser.

#### Diagnostics Push/Pull Modes
Two implementations registered based on client capability detection (diagnostic pull support present/absent), with shared validator function and debounced validation on 500ms delay.

**Porting Note**: Both modes depend on Connection interface; Tauri LSP bridge must support both protocol variants.

---

## Summary

The HTML Language Features extension is a textbook **LSP extension**: 600+ line core server with handler registration for 15+ LSP capabilities, two transportation layers (IPC/Worker), shared client logic for UI integration, and a test suite covering completions, formatting, folding, and refactoring. 

A Tauri port would require:
1. **Transport Layer Redesign**: Replace vscode-languageserver's IPC/Browser abstractions with Tauri's invoke/listen channel
2. **Server Rewrite**: Rust implementation replacing vscode-languageserver protocol handlers; likely using lsp-types crate
3. **Language Service Integration**: Replace vscode-html-languageservice (JS) with Rust HTML parser (tree-sitter-html or custom); similar for CSS (vscode-css-languageservice)
4. **File System Bridge**: Implement Tauri command equivalents for FsStatRequest/FsReadDirRequest
5. **Configuration Binding**: Map Tauri settings system to LSP scoped configuration protocol
6. **Build Pipeline**: Single Rust server build (no dual Node/Browser split)

The extension's architecture—separating client UI logic from language server—maps cleanly to Tauri's process model but introduces friction in the LSP transport layer abstraction.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Core IDE Functionality Port: HTML Language Features Patterns

## Research Scope
`extensions/html-language-features/` — Server initialization and Language Client setup patterns for a multi-protocol IDE feature.

## Key Findings

The HTML language features extension demonstrates a **dual-runtime architecture** pattern where:
- **Client** (TypeScript/Electron) connects to server via multiple transports
- **Server** (TypeScript/Node.js or Browser) uses unified core logic
- **Protocol** bridges the gap via Language Server Protocol (LSP)

---

#### Pattern: Node.js Server Connection Setup

**Where:** `extensions/html-language-features/server/src/node/htmlServerMain.ts:1-36`

**What:** Establishes IPC connection for Node.js-based language server with error handling and console binding.

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

**Variations / call-sites:**
- `extensions/html-language-features/server/src/node/htmlServerMain.ts:36` — calls `startServer`
- `extensions/html-language-features/server/src/browser/htmlServerMain.ts:30` — browser variant with Web Worker message passing

---

#### Pattern: Browser Server Connection Setup

**Where:** `extensions/html-language-features/server/src/browser/htmlServerMain.ts:1-30`

**What:** Establishes browser-based server connection with Web Worker message reader/writer for DOM-less environment.

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

**Variations / call-sites:**
- `extensions/html-language-features/server/src/node/htmlServerMain.ts:13` — Node.js variant with IPC

---

#### Pattern: Unified Server Core Entry Point

**Where:** `extensions/html-language-features/server/src/htmlServer.ts:88-227`

**What:** Core server logic that accepts any Connection type and runtime, establishing capabilities/handlers independent of transport layer.

```typescript
export interface RuntimeEnvironment {
	fileFs?: FileSystemProvider;
	configureHttpRequests?(proxy: string | undefined, strictSSL: boolean): void;
	readonly timer: {
		setImmediate(callback: (...args: any[]) => void, ...args: any[]): Disposable;
		setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable;
	};
}

export function startServer(connection: Connection, runtime: RuntimeEnvironment) {
	// Create a text document manager.
	const documents = new TextDocuments(TextDocument);
	documents.listen(connection);

	// After the server has started the client sends an initialize request
	connection.onInitialize((params: InitializeParams): InitializeResult => {
		// ... capability negotiation with client
		const capabilities: ServerCapabilities = {
			textDocumentSync: TextDocumentSyncKind.Incremental,
			completionProvider: clientSnippetSupport ? { resolveProvider: true, triggerCharacters: ['.', ':', '<', '"', '=', '/'] } : undefined,
			hoverProvider: true,
			documentHighlightProvider: true,
			documentRangeFormattingProvider: initializationOptions?.provideFormatter === true,
			documentFormattingProvider: initializationOptions?.provideFormatter === true,
			documentLinkProvider: { resolveProvider: false },
			documentSymbolProvider: true,
			definitionProvider: true,
			signatureHelpProvider: { triggerCharacters: ['('] },
			referencesProvider: true,
			colorProvider: {},
			foldingRangeProvider: true,
			selectionRangeProvider: true,
			renameProvider: true,
			linkedEditingRangeProvider: true
		};
		return { capabilities };
	});

	connection.listen();
}
```

**Variations / call-sites:**
- `extensions/html-language-features/server/src/node/htmlServerMain.ts:36` — Node.js entry
- `extensions/html-language-features/server/src/browser/htmlServerMain.ts:30` — Browser entry

---

#### Pattern: Node Client with Module-Based Server Options

**Where:** `extensions/html-language-features/client/src/node/htmlClientMain.ts:18-51`

**What:** Client extension activation that configures server module path, debug options, and IPC transport; delegates to unified client setup.

```typescript
export async function activate(context: ExtensionContext) {
	const clientPackageJSON = getPackageInfo(context);
	telemetry = new TelemetryReporter(clientPackageJSON.aiKey);

	const serverMain = `./server/${clientPackageJSON.main.indexOf('/dist/') !== -1 ? 'dist' : 'out'}/node/htmlServerMain`;
	const serverModule = context.asAbsolutePath(serverMain);

	// The debug options for the server
	const debugOptions = { execArgv: ['--nolazy', '--inspect=' + (8000 + Math.round(Math.random() * 999))] };

	// If the extension is launch in debug mode the debug server options are use
	// Otherwise the run options are used
	const serverOptions: ServerOptions = {
		run: { module: serverModule, transport: TransportKind.ipc },
		debug: { module: serverModule, transport: TransportKind.ipc, options: debugOptions }
	};

	const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
		return new LanguageClient(id, name, serverOptions, clientOptions);
	};

	const timer = {
		setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable {
			const handle = setTimeout(callback, ms, ...args);
			return { dispose: () => clearTimeout(handle) };
		}
	};

	// pass the location of the localization bundle to the server
	process.env['VSCODE_L10N_BUNDLE_LOCATION'] = l10n.uri?.toString() ?? '';

	client = await startClient(context, newLanguageClient, { fileFs: getNodeFileFS(), TextDecoder, telemetry, timer });
}
```

**Variations / call-sites:**
- `extensions/html-language-features/client/src/node/htmlClientMain.ts:37` — LanguageClient instantiation with ServerOptions

---

#### Pattern: Browser Client with Worker-Based Server Options

**Where:** `extensions/html-language-features/client/src/browser/htmlClientMain.ts:14-31`

**What:** Client activation for browser environment that creates Web Worker and wraps it as LanguageClient transport.

```typescript
export async function activate(context: ExtensionContext) {
	const serverMain = Uri.joinPath(context.extensionUri, 'server/dist/browser/htmlServerMain.js');
	try {
		const worker = new Worker(serverMain.toString());
		worker.postMessage({ i10lLocation: l10n.uri?.toString(false) ?? '' });

		const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
			return new LanguageClient(id, name, worker, clientOptions);
		};

		const timer = {
			setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable {
				const handle = setTimeout(callback, ms, ...args);
				return { dispose: () => clearTimeout(handle) };
			}
		};

		client = await startClient(context, newLanguageClient, { TextDecoder, timer });

	} catch (e) {
		console.log(e);
	}
}
```

**Variations / call-sites:**
- `extensions/html-language-features/client/src/browser/htmlClientMain.ts:21` — LanguageClient instantiation with Worker

---

#### Pattern: Unified Client Initialization with Capability Negotiation

**Where:** `extensions/html-language-features/client/src/htmlClient.ts:92-199`

**What:** Platform-agnostic client setup that accepts a LanguageClientConstructor function, registers capabilities (formatting, semantic tokens, auto-insert), handles language participant changes with restart triggers, and manages disposables.

```typescript
export async function startClient(context: ExtensionContext, newLanguageClient: LanguageClientConstructor, runtime: Runtime): Promise<AsyncDisposable> {
	const logOutputChannel = window.createOutputChannel(languageServerDescription, { log: true });
	const languageParticipants = getLanguageParticipants();
	context.subscriptions.push(languageParticipants);

	let client: Disposable | undefined = await startClientWithParticipants(languageParticipants, newLanguageClient, logOutputChannel, runtime);

	let restartTrigger: Disposable | undefined;
	languageParticipants.onDidChange(() => {
		if (restartTrigger) {
			restartTrigger.dispose();
		}
		restartTrigger = runtime.timer.setTimeout(async () => {
			if (client) {
				logOutputChannel.info('Extensions have changed, restarting HTML server...');
				const oldClient = client;
				client = undefined;
				await oldClient.dispose();
				client = await startClientWithParticipants(languageParticipants, newLanguageClient, logOutputChannel, runtime);
			}
		}, 2000);
	});

	return {
		dispose: async () => {
			restartTrigger?.dispose();
			await client?.dispose();
			logOutputChannel.dispose();
		}
	};
}

async function startClientWithParticipants(languageParticipants: LanguageParticipants, newLanguageClient: LanguageClientConstructor, logOutputChannel: LogOutputChannel, runtime: Runtime): Promise<AsyncDisposable> {
	const toDispose: Disposable[] = [];
	const documentSelector = languageParticipants.documentSelector;
	const embeddedLanguages = { css: true, javascript: true };

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
		middleware: { /* completion item range handling */ }
	};
	clientOptions.outputChannel = logOutputChannel;

	const client = newLanguageClient('html', languageServerDescription, clientOptions);
	client.registerProposedFeatures();

	await client.start();
	
	toDispose.push(serveFileSystemRequests(client, runtime));
	const customDataSource = getCustomDataSource(runtime, toDispose);
	client.sendNotification(CustomDataChangedNotification.type, customDataSource.uris);
	
	return {
		dispose: async () => {
			await client.stop();
			toDispose.forEach(d => d.dispose());
		}
	};
}
```

**Variations / call-sites:**
- `extensions/html-language-features/client/src/htmlClient.ts:196` — Client instantiation via constructor
- `extensions/html-language-features/client/src/htmlClient.ts:199` — client.start() call
- `extensions/html-language-features/client/src/htmlClient.ts:234-256` — Semantic token provider registration

---

## Architectural Insights for Tauri/Rust Porting

### 1. **Transport Abstraction**
The codebase uses the Language Server Protocol as its abstraction layer, with three distinct transport implementations:
- **Node.js IPC**: Child process spawning with stdio
- **Browser Worker**: Web Worker message passing
- **Unified Protocol**: Both use `Connection` interface with identical request/notification handlers

### 2. **Runtime Capability Injection**
Core server (`htmlServer.ts`) accepts a `RuntimeEnvironment` parameter instead of assuming Node.js globals:
- Timer abstractions (setImmediate, setTimeout)
- Filesystem provider interface
- HTTP configuration hooks

This enables protocol-neutral request/response handling.

### 3. **Client-Side Constructor Pattern**
The `LanguageClientConstructor` type allows the client to be constructed differently per platform:
- Node: `new LanguageClient(id, name, serverOptions, clientOptions)` with module/transport
- Browser: `new LanguageClient(id, name, worker, clientOptions)` with Worker
- **Implication**: Rust could provide a similar constructor that wraps native binaries or IPC sockets

### 4. **Disposable Chain Management**
Extensive use of disposable pattern for cleanup:
- Client lifecycle managed via `AsyncDisposable`
- Language participants tracked for dynamic server restart
- Middleware for feature gates (e.g., format provider)
- Array-based accumulation of disposables (`toDispose[]`) with bulk cleanup

### 5. **Capability Negotiation**
Server announces capabilities in `InitializeResult` based on:
- Client capability queries (snippet support, formatting support, diagnostic type)
- Initialization options (embedded language modes, custom data providers)
- Runtime configuration (formatter enabled, max edits limit)

Dynamic registration used for formatter based on settings changes.

### 6. **Dual-Mode Build Targets**
Same TypeScript source compiled to:
- `/dist/node/` for Node.js execution
- `/dist/browser/` for bundled browser/worker execution

Suggests Rust codebase would need conditional compilation or separate binary targets.

---

## Summary

The HTML language features extension exemplifies a **protocol-driven, transport-agnostic architecture** where:
1. LSP serves as the unifying interface
2. Core server logic is platform-neutral
3. Client adapts transport layer per environment
4. Runtime capabilities are injected, not assumed
5. Lifecycle management uses functional disposable chains

For a Tauri/Rust port, the key migration path would be:
- Keep LSP protocol unchanged (well-documented in `vscode-languageserver` npm package)
- Implement Rust equivalents of `Connection`, `RuntimeEnvironment`, and `LanguageClientConstructor`
- Provide Tauri-native process spawning / socket communication replacing IPC
- Maintain the same initialization handshake and capability negotiation

The protocol abstraction means the LSP message format itself is portable; the heavy lifting is reimplementing the bidirectional communication layer and runtime environment capsules in Rust.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
