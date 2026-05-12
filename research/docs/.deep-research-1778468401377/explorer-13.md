# Partition 13 of 80 — Findings

## Scope
`extensions/html-language-features/` (51 files, 9,248 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Location Index: HTML Language Features Extension

Partition 13 scope: `extensions/html-language-features/` (84 files, ~9,248 LOC)

Research Question: What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

## Implementation

### Client Architecture
- `extensions/html-language-features/client/src/htmlClient.ts` — LSP client bootstrapping and feature registration; manages language client lifecycle, providers (semantic tokens, formatting, completion), custom data loading, and middleware for protocol conversion
- `extensions/html-language-features/client/src/node/htmlClientMain.ts` — Node.js client entry point; creates IPC-based LanguageClient with server process spawning and telemetry setup
- `extensions/html-language-features/client/src/browser/htmlClientMain.ts` — Browser client entry point; creates Worker-based LanguageClient for web execution; demonstrates LSP over Web Worker
- `extensions/html-language-features/client/src/requests.ts` — Custom request types (FsStatRequest, FsReadDirRequest); FileSystemProvider abstraction for platform-specific fs operations
- `extensions/html-language-features/client/src/node/nodeFs.ts` — Node.js filesystem adapter; wraps native fs module with FileSystemProvider interface

### Server Architecture  
- `extensions/html-language-features/server/src/htmlServer.ts` — LSP server core; handles connection lifecycle, document synchronization, custom notifications (CustomDataChanged), custom requests (AutoInsert, SemanticTokens), and dispatches to language modes
- `extensions/html-language-features/server/src/node/htmlServerMain.ts` — Node.js server entry point; creates IPC Connection, sets up runtime environment (timers, fs), captures console output
- `extensions/html-language-features/server/src/node/htmlServerNodeMain.ts` — Server CLI wrapper; referenced in build as main entry
- `extensions/html-language-features/server/src/browser/htmlServerMain.ts` — Browser server entry point; creates connection from BrowserMessageReader/Writer
- `extensions/html-language-features/server/src/browser/htmlServerWorkerMain.ts` — Web Worker bootstrap; handles async l10n loading before delegating to htmlServerMain

### Language Mode System
- `extensions/html-language-features/server/src/modes/languageModes.ts` — LanguageMode interface and factory; defines protocol for completions, hover, validation, diagnostics, formatting, semantic tokens, folding ranges, selection ranges across embedded HTML/CSS/JS
- `extensions/html-language-features/server/src/modes/htmlMode.ts` — HTML language mode implementation; wraps vscode-html-languageservice for completions, hover, formatting, symbols, links, folding
- `extensions/html-language-features/server/src/modes/cssMode.ts` — CSS mode for embedded style blocks
- `extensions/html-language-features/server/src/modes/javascriptMode.ts` — JavaScript mode for embedded script blocks; integrates TypeScript library definitions
- `extensions/html-language-features/server/src/modes/embeddedSupport.ts` — Region extraction logic; parses HTML to identify CSS/JS regions and their language contexts
- `extensions/html-language-features/server/src/modes/formatting.ts` — Unified formatting dispatcher for multi-language documents
- `extensions/html-language-features/server/src/modes/htmlFolding.ts` — Code folding regions for HTML structures
- `extensions/html-language-features/server/src/modes/selectionRanges.ts` — Semantic selection range expansion
- `extensions/html-language-features/server/src/modes/semanticTokens.ts` — Token classification provider; integrates HTML and CSS token types
- `extensions/html-language-features/server/src/modes/javascriptLibs.ts` — Inlined TypeScript library definitions for browser environments (handles lib.d.ts and jQuery)
- `extensions/html-language-features/server/src/modes/javascriptSemanticTokens.ts` — JS semantic token classification layer

### Client Features
- `extensions/html-language-features/client/src/autoInsertion.ts` — Auto-quote and auto-close tag insertion; listens to text changes, sends requests to server for intelligent insertion
- `extensions/html-language-features/client/src/languageParticipants.ts` — Extension participant registry pattern; allows other extensions to register languages that integrate with HTML server (e.g., handlebars)
- `extensions/html-language-features/client/src/customData.ts` — Custom data loader; discovers and loads HTML element/attribute definitions from workspace configs and extensions

### Server Utilities
- `extensions/html-language-features/server/src/languageModelCache.ts` — LRU cache for parsed documents; stores HTMLDocument/CSS models with version tracking and automatic cleanup
- `extensions/html-language-features/server/src/requests.ts` — Server-side request handlers (FsStatRequest, FsReadDirRequest); bridges client fs requests to runtime filesystem
- `extensions/html-language-features/server/src/node/nodeFs.ts` — Node.js filesystem implementation; async fs.stat and fs.readdir adapters
- `extensions/html-language-features/server/src/utils/documentContext.ts` — Document context provider for path resolution and completion context
- `extensions/html-language-features/server/src/utils/positions.ts` — Position/offset conversion utilities
- `extensions/html-language-features/server/src/utils/strings.ts` — String manipulation helpers
- `extensions/html-language-features/server/src/utils/arrays.ts` — Array utility functions
- `extensions/html-language-features/server/src/utils/validation.ts` — Diagnostic support; registers push or pull diagnostic providers with debouncing
- `extensions/html-language-features/server/src/utils/runner.ts` — Error formatting and safe execution wrapper

### Server Custom Data
- `extensions/html-language-features/server/src/customData.ts` — Custom data fetching from URIs; mirrors client pattern for server-side extension discovery

## Tests

### Server Tests
- `extensions/html-language-features/server/src/test/completions.test.ts` — Completion scenarios (tags, attributes, values)
- `extensions/html-language-features/server/src/test/formatting.test.ts` — Multi-language formatting
- `extensions/html-language-features/server/src/test/folding.test.ts` — Folding region detection
- `extensions/html-language-features/server/src/test/selectionRanges.test.ts` — Selection range expansion
- `extensions/html-language-features/server/src/test/semanticTokens.test.ts` — Token classification
- `extensions/html-language-features/server/src/test/rename.test.ts` — Element rename operations
- `extensions/html-language-features/server/src/test/words.test.ts` — Word boundary detection
- `extensions/html-language-features/server/src/test/embedded.test.ts` — Embedded language region detection
- `extensions/html-language-features/server/src/test/documentContext.test.ts` — Document context path resolution

### Test Fixtures
- `extensions/html-language-features/server/src/test/pathCompletionFixtures/` — File hierarchy for path completion tests
- `extensions/html-language-features/server/src/test/fixtures/inputs/` — Input HTML files for formatting tests
- `extensions/html-language-features/server/src/test/fixtures/expected/` — Expected output for formatting tests

### Test Infrastructure
- `extensions/html-language-features/server/test/index.js` — Test runner

## Types / Interfaces

### Client Abstractions
- `extensions/html-language-features/client/tsconfig.json` — TypeScript config for client compilation
- `extensions/html-language-features/client/tsconfig.browser.json` — Browser-specific TS config

### Server Abstractions  
- `extensions/html-language-features/server/tsconfig.json` — Server TypeScript config
- `extensions/html-language-features/server/tsconfig.browser.json` — Browser-specific server TS config

### Type Definitions
- `extensions/html-language-features/server/lib/jquery.d.ts` — jQuery type definitions for embedded JS contexts

## Configuration

### Extension Manifest
- `extensions/html-language-features/package.json` — Extension metadata; declares activation events (onLanguage:html, onLanguage:handlebars), client/browser entry points, 30+ HTML settings, contribution schemas for custom data

### Server Package
- `extensions/html-language-features/server/package.json` — Server dependencies (vscode-html-languageservice, vscode-languageserver, vscode-css-languageservice)

### Build Configuration
- `extensions/html-language-features/esbuild.mts` — ESM build script for Node.js client and server; defines entry points and tsconfig paths
- `extensions/html-language-features/esbuild.browser.mts` — ESM build script for browser client and server; includes javaScriptLibsPlugin for inlining TypeScript libs
- `extensions/html-language-features/.npmrc` — NPM config

### Schema
- `extensions/html-language-features/schemas/package.schema.json` — Validation schema for package.json custom data field

### VSCode Config
- `extensions/html-language-features/.vscode/launch.json` — Debug launcher for extension
- `extensions/html-language-features/.vscode/tasks.json` — Build tasks
- `extensions/html-language-features/.vscode/settings.json` — Editor settings
- `extensions/html-language-features/server/.vscode/launch.json` — Server debug launcher
- `extensions/html-language-features/server/.vscode/tasks.json` — Server build tasks

## Documentation

- `extensions/html-language-features/README.md` — Feature overview and extension bundling note
- `extensions/html-language-features/CONTRIBUTING.md` — Development setup, linking vscode-html-languageservice, debugging client/server processes
- `extensions/html-language-features/build/bundleTypeScriptLibraries.js` — Script for bundling TypeScript library definitions

## Examples / Fixtures

- `extensions/html-language-features/icons/html.png` — Extension icon
- `extensions/html-language-features/.vscodeignore` — Package exclusions
- `extensions/html-language-features/cgmanifest.json` — Component governance manifest
- `extensions/html-language-features/server/lib/cgmanifest.json` — Server dependencies governance
- `extensions/html-language-features/package-lock.json` — Client dependency lock
- `extensions/html-language-features/server/package-lock.json` — Server dependency lock
- `extensions/html-language-features/package.nls.json` — Localized strings

## Notable Clusters

- `extensions/html-language-features/client/src/` — 9 files: LSP client bridge layer; connects VS Code extension API to language server via LanguageClient abstraction (IPC or Worker transport); handles custom requests, file system delegation, auto-insertion, and language participant discovery
- `extensions/html-language-features/server/src/modes/` — 9 files: Multi-language mode system; abstracts HTML/CSS/JavaScript parsing and feature provisioning through LanguageMode interface; enables embedded language support via region extraction and document projection
- `extensions/html-language-features/server/src/utils/` — 6 files: Utility layer; provides document caching, validation scheduling, error handling, string/array manipulation, and position conversion
- `extensions/html-language-features/server/src/test/` — 9 test files + fixtures: Comprehensive test coverage for completions, formatting, folding, semantic tokens, diagnostics, and embedded language region detection

---

## Summary

The HTML language features extension exemplifies VS Code's LSP client-server architecture with multi-transport support (Node.js IPC, Web Worker). For a Tauri/Rust port, this structure reveals: (1) **LSP Protocol Adoption** — the extension relies entirely on vscode-languageserver protocol abstractions, making Rust LSP crates (tower-lsp, lsp-types) viable replacements; (2) **Runtime Abstraction** — filesystem operations are abstracted through FileSystemProvider, timer APIs through RuntimeEnvironment, enabling platform-specific Rust implementations; (3) **Multi-Platform Code Paths** — separate node/browser entry points with dual TypeScript configs show dual-platform requirements that Tauri must satisfy (native fs + web platform); (4) **Embedded Language Support** — region extraction and document projection patterns (embeddedSupport.ts, languageModes.ts) form the core complexity of multi-language editing and would require careful porting to Rust's type system; (5) **Dynamic Feature Discovery** — language participants and custom data loaders leverage VS Code's extension registry, requiring Tauri to maintain similar extension integration mechanisms. The server's tight coupling to vscode-html-languageservice and vscode-css-languageservice libraries indicates that Rust ports would either need equivalent language service implementations or FFI bindings to existing services.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: LSP Client Wiring in VS Code HTML Language Features

**Research Question:** What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

**Partition:** 13 — `extensions/html-language-features/` (51 files, 9,248 LOC)

**Focus:** LSP client initialization and protocol wiring patterns that inform how a Rust-based core would need to host language servers.

---

## Pattern: LanguageClient Constructor Abstraction

**Where:** `extensions/html-language-features/client/src/htmlClient.ts:75-76`

**What:** Define a factory function type that abstracts away transport-specific client creation, enabling both Node.js IPC and Web Worker transports.

```typescript
export type LanguageClientConstructor = 
  (name: string, description: string, clientOptions: LanguageClientOptions) => BaseLanguageClient;
```

**Implementation in Node.js transport:**
`extensions/html-language-features/client/src/node/htmlClientMain.ts:36-38`
```typescript
const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
    return new LanguageClient(id, name, serverOptions, clientOptions);
};
```

**Implementation in Browser transport:**
`extensions/html-language-features/client/src/browser/htmlClientMain.ts:20-22`
```typescript
const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
    return new LanguageClient(id, name, worker, clientOptions);
};
```

**Variations / call-sites:**
- Node.js uses `ServerOptions` with `TransportKind.ipc` for local process communication
- Browser uses `Worker` instance for Web Worker communication
- Both pass through `LanguageClientOptions` uniformly
- Client initialization always: `client.registerProposedFeatures()` + `await client.start()` (line 197-199)

---

## Pattern: Bidirectional Request/Response Handler Registration

**Where:** `extensions/html-language-features/server/src/htmlServer.ts:494-501`

**What:** Register custom request handlers on the server that respond to client requests using typed `RequestType` definitions.

```typescript
connection.onRequest(AutoInsertRequest.type, (params, token) => {
    return runSafe(runtime, async () => {
        const document = documents.get(params.textDocument.uri);
        if (document) {
            const pos = params.position;
            if (pos.character > 0) {
                // implementation
            }
        }
        return null;
    }, null, `Error while computing...`, token);
});
```

**Other request handlers in same file:**
- `DocumentColorRequest.type` (line 466)
- `ColorPresentationRequest.type` (line 481)
- `SemanticTokenRequest.type` (line 574)
- `SemanticTokenLegendRequest.type` (line 584)
- `TextDocumentContentRequest.type` (line 596)

**Variations / call-sites:**
- Custom requests use `RequestType<Params, Result, Error>` namespace pattern
- Client-side initiation via `client.sendRequest(RequestType, params)`
- Server response via `connection.onRequest(RequestType, handler)`
- Errors handled via `runSafe` wrapper and `handleFailedRequest` callback

---

## Pattern: Notification One-Way Messaging

**Where:** `extensions/html-language-features/client/src/htmlClient.ts:205-208`

**What:** Send notifications from client to server (fire-and-forget) using typed `NotificationType` definitions.

```typescript
client.sendNotification(CustomDataChangedNotification.type, customDataSource.uris);
customDataSource.onDidChange(() => {
    client.sendNotification(CustomDataChangedNotification.type, customDataSource.uris);
}, undefined, toDispose);
```

**Server-side handler:**
`extensions/html-language-features/server/src/htmlServer.ts:590-593`
```typescript
connection.onNotification(CustomDataChangedNotification.type, dataPaths => {
    fetchHTMLDataProviders(dataPaths, customDataRequestService).then(dataProviders => {
        languageModes.updateDataProviders(dataProviders);
    });
});
```

**Variations / call-sites:**
- Workspace folder changes via `DidChangeWorkspaceFoldersNotification.type` (line 233)
- Both client and server define identical namespace types for compile-time safety
- No response expected; purely event-driven communication

---

## Pattern: Middleware Provider Interception

**Where:** `extensions/html-language-features/client/src/htmlClient.ts:167-191`

**What:** Intercept and transform LSP protocol responses before presentation to VS Code providers.

```typescript
const clientOptions: LanguageClientOptions = {
    documentSelector,
    synchronize: { configurationSection: ['html', 'css', 'javascript', 'js/ts'] },
    initializationOptions: { embeddedLanguages, handledSchemas: ['file'], provideFormatter: false },
    middleware: {
        provideCompletionItem(document, position, context, token, next) {
            function updateRanges(item: CompletionItem) {
                const range = item.range;
                if (range instanceof Range && range.end.isAfter(position) && range.start.isBeforeOrEqual(position)) {
                    item.range = { inserting: new Range(range.start, position), replacing: range };
                }
            }
            const r = next(document, position, context, token);
            if (isThenable(r)) {
                return r.then(updateProposals);
            }
            return updateProposals(r);
        }
    }
};
```

**Key aspects:**
- Middleware wraps the `next` provider function for composition
- Thenable handling for both sync and async responses
- Runs before items are displayed in UI

---

## Pattern: Protocol Converter Bridges

**Where:** `extensions/html-language-features/client/src/htmlClient.ts:212-219`

**What:** Convert between VS Code native types and LSP protocol types using client-provided converters.

```typescript
const insertRequestor = (kind: 'autoQuote' | 'autoClose', document: TextDocument, position: Position): Promise<string | null> => {
    const param: AutoInsertParams = {
        kind,
        textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(document),
        position: client.code2ProtocolConverter.asPosition(position)
    };
    return client.sendRequest(AutoInsertRequest.type, param);
};
```

**Reverse conversion:**
`extensions/html-language-features/client/src/htmlClient.ts:278-279`
```typescript
return client.sendRequest(DocumentRangeFormattingRequest.type, params, token).then(
    client.protocol2CodeConverter.asTextEdits,
    (error) => { /* handle error */ }
);
```

**Variations / call-sites:**
- `code2ProtocolConverter.asTextDocumentIdentifier`, `asPosition`, `asRange`, `asFormattingOptions`
- `protocol2CodeConverter.asTextEdits`
- Semantic tokens: wrapping `new Uint32Array(data)` in `new SemanticTokens()`

---

## Pattern: Runtime Environment Abstraction

**Where:** `extensions/html-language-features/client/src/htmlClient.ts:79-86`

**What:** Define a runtime interface injected at startup to abstract platform-specific capabilities (Node.js vs Browser).

```typescript
export interface Runtime {
    TextDecoder: typeof TextDecoder;
    fileFs?: FileSystemProvider;
    telemetry?: TelemetryReporter;
    readonly timer: {
        setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable;
    };
}
```

**Server-side equivalent:**
`extensions/html-language-features/server/src/htmlServer.ts:73-80`
```typescript
export interface RuntimeEnvironment {
    fileFs?: FileSystemProvider;
    configureHttpRequests?(proxy: string | undefined, strictSSL: boolean): void;
    readonly timer: {
        setImmediate(callback: (...args: any[]) => void, ...args: any[]): Disposable;
        setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable;
    };
}
```

**Node.js implementation:**
`extensions/html-language-features/server/src/node/htmlServerNodeMain.ts:22-34`
```typescript
const runtime: RuntimeEnvironment = {
    timer: {
        setImmediate(callback, ...args) {
            const handle = setImmediate(callback, ...args);
            return { dispose: () => clearImmediate(handle) };
        },
        setTimeout(callback, ms, ...args) {
            const handle = setTimeout(callback, ms, ...args);
            return { dispose: () => clearTimeout(handle) };
        }
    },
    fileFs: getNodeFileFS()
};
```

**Browser implementation:**
`extensions/html-language-features/server/src/browser/htmlServerMain.ts:17-28`
```typescript
const runtime: RuntimeEnvironment = {
    timer: {
        setImmediate(callback, ...args) {
            const handle = setTimeout(callback, 0, ...args);
            return { dispose: () => clearTimeout(handle) };
        },
        setTimeout(callback, ms, ...args) {
            const handle = setTimeout(callback, ms, ...args);
            return { dispose: () => clearTimeout(handle) };
        }
    }
};
```

**Variations / call-sites:**
- Client-side runtime used in `activateAutoInsertion` for debounced requests (line 76-77)
- Server-side runtime passed to `startServer(connection, runtime)` (line 36)

---

## Pattern: Language Participant Discovery & Dynamic Registration

**Where:** `extensions/html-language-features/client/src/languageParticipants.ts:30-74`

**What:** Dynamically discover language extensions that contribute HTML language support and manage document selectors.

```typescript
export function getLanguageParticipants(): LanguageParticipants {
    const onDidChangeEmmiter = new EventEmitter<void>();
    let languages = new Set<string>();
    let autoInsert = new Set<string>();

    function update() {
        languages = new Set();
        languages.add('html');
        autoInsert = new Set();
        autoInsert.add('html');

        for (const extension of extensions.allAcrossExtensionHosts) {
            const htmlLanguageParticipants = extension.packageJSON?.contributes?.htmlLanguageParticipants as LanguageParticipantContribution[];
            if (Array.isArray(htmlLanguageParticipants)) {
                for (const htmlLanguageParticipant of htmlLanguageParticipants) {
                    const languageId = htmlLanguageParticipant.languageId;
                    if (typeof languageId === 'string') {
                        languages.add(languageId);
                        if (htmlLanguageParticipant.autoInsert !== false) {
                            autoInsert.add(languageId);
                        }
                    }
                }
            }
        }
        return !isEqualSet(languages, oldLanguages) || !isEqualSet(autoInsert, oldAutoInsert);
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
        useAutoInsert(languageId: string) { return autoInsert.has(languageId); },
        dispose: () => changeListener.dispose()
    };
}
```

**Usage in client initialization:**
`extensions/html-language-features/client/src/htmlClient.ts:96-98`
```typescript
const languageParticipants = getLanguageParticipants();
context.subscriptions.push(languageParticipants);
let client: Disposable | undefined = await startClientWithParticipants(languageParticipants, newLanguageClient, logOutputChannel, runtime);
```

**Dynamic restart on changes:**
`extensions/html-language-features/client/src/htmlClient.ts:121-135`
```typescript
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
```

**Variations / call-sites:**
- Document selector passed to all language providers (line 150-151)
- Drives semantic tokens, formatting, completion registration

---

## Pattern: Async Disposable Lifecycle Management

**Where:** `extensions/html-language-features/client/src/htmlClient.ts:88-90, 137-143`

**What:** Define and implement async disposal pattern for graceful server shutdown and resource cleanup.

```typescript
export interface AsyncDisposable {
    dispose(): Promise<void>;
}

// Implementation
return {
    dispose: async () => {
        restartTrigger?.dispose();
        await client?.dispose();
        logOutputChannel.dispose();
    }
};
```

**Per-participant cleanup:**
`extensions/html-language-features/client/src/htmlClient.ts:341-347`
```typescript
return {
    dispose: async () => {
        await client.stop();
        toDispose.forEach(d => d.dispose());
        rangeFormatting?.dispose();
    }
};
```

**File System Request Serving:**
`extensions/html-language-features/client/src/requests.ts:18-35`
```typescript
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
```

**Variations / call-sites:**
- All disposables tracked in `toDispose` array and cleaned up together
- Server connection automatically removes document settings cache on close (line 118-120)

---

## Summary: Architectural Patterns for Rust/Tauri Port

The HTML language features extension demonstrates a **modular, transport-agnostic LSP client architecture** that a Rust-based VS Code core would need to replicate:

1. **Abstract Client Creation** — Factory patterns decouple transport (IPC vs WebWorker vs HTTP) from client logic.

2. **Typed Protocol** — `RequestType<P, R, E>` and `NotificationType<P>` provide compile-time type safety across the LSP boundary.

3. **Middleware Composition** — Intercept protocol responses before display to compose transformations without modifying server code.

4. **Bidirectional Handlers** — Both request/response (with error handling) and one-way notifications enable full LSP semantics.

5. **Protocol Conversion** — Explicit converters (`code2ProtocolConverter`, `protocol2CodeConverter`) bridge VS Code native types and LSP JSON-RPC types.

6. **Runtime Abstraction** — Injected interfaces abstract platform capabilities (file system, timers, telemetry), enabling the same language server to run in Node.js and browsers.

7. **Dynamic Registration** — Extension discovery and participant updates trigger server restarts without reloading the host.

8. **Async Lifecycle** — Proper async disposal ensures servers shut down cleanly before extension or workspace reload.

A Tauri/Rust port would need to:
- Implement an LSP JSON-RPC transport over IPC or WebSocket
- Mirror the middleware/converter system in Rust
- Provide equivalent runtime interfaces for async I/O, timers, and file system access
- Support dynamic language server discovery and restart
- Handle both built-in and third-party language server registration

---

**Key Files Referenced:**
- `extensions/html-language-features/client/src/htmlClient.ts` — Core client abstraction
- `extensions/html-language-features/client/src/node/htmlClientMain.ts` — Node.js transport
- `extensions/html-language-features/client/src/browser/htmlClientMain.ts` — Browser transport
- `extensions/html-language-features/server/src/htmlServer.ts` — Server initialization and handlers
- `extensions/html-language-features/client/src/requests.ts` — Custom request/response types
- `extensions/html-language-features/client/src/languageParticipants.ts` — Language discovery

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
