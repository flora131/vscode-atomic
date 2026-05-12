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
