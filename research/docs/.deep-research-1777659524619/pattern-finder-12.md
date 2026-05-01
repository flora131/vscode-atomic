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
