# VS Code CSS Language Features: IDE Integration Patterns

## Research Question
What architectural patterns does VS Code use to port IDE functionality from TypeScript/Electron to alternative runtimes (Node/Browser) while maintaining a unified LSP-based language intelligence layer?

## Patterns Found

#### Pattern 1: Dual-Runtime LSP Client Construction
**Where:** `extensions/css-language-features/client/src/node/cssClientMain.ts:26-32` and `extensions/css-language-features/client/src/browser/cssClientMain.ts:21-23`

**What:** Runtime-specific server transport configuration wrapped by a unified `LanguageClientConstructor` factory function that abstracts transport differences.

```typescript
// Node Runtime (IPC transport)
const serverOptions: ServerOptions = {
  run: { module: serverModule, transport: TransportKind.ipc },
  debug: { module: serverModule, transport: TransportKind.ipc, options: debugOptions }
};
const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
  return new LanguageClient(id, name, serverOptions, clientOptions);
};
```

```typescript
// Browser Runtime (Web Worker)
const worker = new Worker(serverMain.toString());
const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
  return new LanguageClient(id, name, worker, clientOptions);
};
```

**Variations:** Both runtimes pass identical `LanguageClientConstructor` signature to unified `startClient()` function (line 38 Node, line 25 Browser), decoupling transport from language intelligence setup.

---

#### Pattern 2: Abstracted Client Bootstrap with Configuration Layering
**Where:** `extensions/css-language-features/client/src/cssClient.ts:39-103`

**What:** Central `startClient()` function receives injected constructor and runtime services, establishes document selector and client options, then registers providers and notification handlers.

```typescript
export async function startClient(context: ExtensionContext, newLanguageClient: LanguageClientConstructor, runtime: Runtime): Promise<BaseLanguageClient> {
  const customDataSource = getCustomDataSource(context.subscriptions);
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
        // Client-side completion filtering and enrichment
        const r = next(document, position, context, token);
        // ... transform results before returning to editor
        return isThenable(r) ? r.then(updateProposals) : updateProposals(r);
      }
    }
  };

  const client = newLanguageClient('css', l10n.t('CSS Language Server'), clientOptions);
  client.registerProposedFeatures();
  await client.start();
  // ... additional registration
  return client;
}
```

**Variations:** Custom data source initialized from workspace config and extensions (line 41), formatter providers registered conditionally (lines 111-115).

---

#### Pattern 3: Notification-Based Data Provider Updates
**Where:** `extensions/css-language-features/client/src/cssClient.ts:12, 105-108` and `extensions/css-language-features/server/src/cssServer.ts:18-19, 380`

**What:** Client-server notification protocol propagates custom data (CSS properties, pseudo-elements) loaded from configuration and extensions to language service instances.

```typescript
// Client-side: Define notification type
namespace CustomDataChangedNotification {
  export const type: NotificationType<string[]> = new NotificationType('css/customDataChanged');
}

// Send on startup and whenever configuration changes
client.sendNotification(CustomDataChangedNotification.type, customDataSource.uris);
customDataSource.onDidChange(() => {
  client.sendNotification(CustomDataChangedNotification.type, customDataSource.uris);
});
```

```typescript
// Server-side: Register handler
connection.onNotification(CustomDataChangedNotification.type, updateDataProviders);

// Handler fetches and propagates to all language services
function updateDataProviders(dataPaths: string[]) {
  dataProvidersReady = fetchDataProviders(dataPaths, requestService).then(customDataProviders => {
    for (const lang in languageServices) {
      languageServices[lang].setDataProviders(true, customDataProviders);
    }
  });
}
```

**Variations:** Custom data sourced from workspace settings (`css.customData`) and extension manifests (lines 40-55 in customData.ts), loaded asynchronously with event change tracking.

---

#### Pattern 4: File System Request Service Abstraction
**Where:** `extensions/css-language-features/client/src/requests.ts:10-45` and `extensions/css-language-features/server/src/cssServer.ts:82`

**What:** LSP requests tunnel filesystem operations from server back to client, enabling server-side file access through editor-controlled request handlers.

```typescript
// Client: Define request types and handlers
export namespace FsContentRequest {
  export const type: RequestType<{ uri: string; encoding?: string }, string, any> = new RequestType('fs/content');
}
export namespace FsStatRequest {
  export const type: RequestType<string, FileStat, any> = new RequestType('fs/stat');
}
export namespace FsReadDirRequest {
  export const type: RequestType<string, [string, FileType][], any> = new RequestType('fs/readDir');
}

export function serveFileSystemRequests(client: BaseLanguageClient, runtime: Runtime) {
  client.onRequest(FsContentRequest.type, (param: { uri: string; encoding?: string }) => {
    const uri = Uri.parse(param.uri);
    if (uri.scheme === 'file' && runtime.fs) {
      return runtime.fs.getContent(param.uri);
    }
    return workspace.fs.readFile(uri).then(buffer => {
      return new runtime.TextDecoder(param.encoding).decode(buffer);
    });
  });
  // ... FsReadDirRequest and FsStatRequest handlers
}
```

```typescript
// Server: Use requestService that makes reverse-requests to client
const requestService = getRequestService(initializationOptions?.handledSchemas || ['file'], connection, runtime);

// Language services use this to load imports/links
languageServices.css = getCSSLanguageService({ fileSystemProvider: requestService, ... });
```

**Variations:** Runtime-specific implementation (`nodeFs.ts` for Node, `workspace.fs` for browser), URI scheme filtering (line 24).

---

#### Pattern 5: Server Capabilities Declaration and Runtime Adaptation
**Where:** `extensions/css-language-features/server/src/cssServer.ts:69-138`

**What:** Server advertises capabilities based on client capability negotiation and initialization options, conditionally enabling features like formatting.

```typescript
connection.onInitialize((params: InitializeParams): InitializeResult => {
  const initializationOptions = params.initializationOptions || {};
  
  function getClientCapability<T>(name: string, def: T) {
    const keys = name.split('.');
    let c: any = params.capabilities;
    for (let i = 0; c && i < keys.length; i++) {
      if (!c.hasOwnProperty(keys[i])) return def;
      c = c[keys[i]];
    }
    return c;
  }
  
  const snippetSupport = !!getClientCapability('textDocument.completion.completionItem.snippetSupport', false);
  scopedSettingsSupport = !!getClientCapability('workspace.configuration', false);
  formatterMaxNumberOfEdits = initializationOptions?.customCapabilities?.rangeFormatting?.editLimit || Number.MAX_VALUE;

  const capabilities: ServerCapabilities = {
    textDocumentSync: TextDocumentSyncKind.Incremental,
    completionProvider: snippetSupport ? { resolveProvider: false, triggerCharacters: ['/', '-', ':'] } : undefined,
    hoverProvider: true,
    // ... other capabilities
    documentRangeFormattingProvider: initializationOptions?.provideFormatter === true,
  };
  return { capabilities };
});
```

**Variations:** Client initialization options (line 55-59 in cssClient.ts) suppress formatting (provideFormatter: false) and declare range limit (editLimit: 10000) to control server behavior.

---

#### Pattern 6: Document-Scoped Setting Resolution with Pull-Style Configuration
**Where:** `extensions/css-language-features/server/src/cssServer.ts:155-171`

**What:** Server resolves language settings per-document by requesting configuration from client, caching results, and invalidating on configuration change.

```typescript
let documentSettings: { [key: string]: Thenable<LanguageSettings | undefined> } = {};

documents.onDidClose(e => {
  delete documentSettings[e.document.uri];
});

function getDocumentSettings(textDocument: TextDocument): Thenable<LanguageSettings | undefined> {
  if (scopedSettingsSupport) {
    let promise = documentSettings[textDocument.uri];
    if (!promise) {
      const configRequestParam = { items: [{ scopeUri: textDocument.uri, section: textDocument.languageId }] };
      promise = connection.sendRequest(ConfigurationRequest.type, configRequestParam)
        .then(s => s[0] as LanguageSettings | undefined);
      documentSettings[textDocument.uri] = promise;
    }
    return promise;
  }
  return Promise.resolve(undefined);
}

connection.onDidChangeConfiguration(change => {
  updateConfiguration(change.settings as { [languageId: string]: LanguageSettings });
});
```

**Variations:** Scoped settings support detected during initialize (line 96), configuration invalidation clears cache (line 178).

---

#### Pattern 7: Multi-Language Service Registry with Request Handlers
**Where:** `extensions/css-language-features/server/src/cssServer.ts:62, 101-103, 141-147, 198-245`

**What:** Single server process manages multiple language services (CSS/SCSS/LESS) with unified request handling that dispatches based on document language ID.

```typescript
const languageServices: { [id: string]: LanguageService } = {};

// Initialize during handshake
languageServices.css = getCSSLanguageService({ fileSystemProvider: requestService, clientCapabilities: params.capabilities });
languageServices.scss = getSCSSLanguageService({ fileSystemProvider: requestService, clientCapabilities: params.capabilities });
languageServices.less = getLESSLanguageService({ fileSystemProvider: requestService, clientCapabilities: params.capabilities });

// Dispatch function
function getLanguageService(document: TextDocument) {
  let service = languageServices[document.languageId];
  if (!service) {
    connection.console.log('Document type is ' + document.languageId + ', using css instead.');
    service = languageServices['css'];
  }
  return service;
}

// Request handlers
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
```

**Variations:** All handlers (onCompletion, onHover, onDocumentSymbol, onDefinition, etc.) follow identical pattern with async/await and error handling via `runSafeAsync()`.

---

## Architectural Summary

The CSS Language Features extension demonstrates **runtime abstraction through LSP indirection**:

1. **Pluggable Transport Layer**: Both Node (IPC) and Browser (Worker) runtimes implement identical LSP connection semantics, with transport differences encapsulated in `LanguageClientConstructor` factory.

2. **Unified Intelligence Layer**: Core language service logic (`startClient()`) remains agnostic to transport, accepting injected runtime services (`TextDecoder`, `fs` request service).

3. **Bidirectional Messaging Protocol**: Client-to-server for language requests (completion, hover, etc.); server-to-client for filesystem operations (reverse requests) and configuration pull.

4. **Capability Negotiation**: Features dynamically enabled/disabled based on client capabilities and initialization options (e.g., formatting suppressed on client, range limits declared).

5. **Data Provider Federation**: Custom data (properties, pseudo-elements) gathered from workspace configuration and extensions, transmitted via notification, distributed to all language service instances.

6. **Multi-Language Service Consolidation**: Single server process manages CSS/SCSS/LESS with shared infrastructure (document manager, stylesheet cache, request service) and language-specific dispatching.

**Key for Tauri/Rust Porting**: LSP protocol remains transport-agnostic; replacing TypeScript client with Rust/Tauri requires reimplementing only:
- LSP client wire protocol (JSON-RPC over IPC/sockets)
- Document sync and event handlers
- Provider registration (completion, hover, etc.)
- Reverse request handlers (filesystem, configuration)

The language intelligence server could theoretically be rewritten in Rust, consuming the same LSP requests and producing identical responses.

