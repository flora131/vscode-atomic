# Partition 28 of 79 — Findings

## Scope
`extensions/css-language-features/` (30 files, 2,261 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# CSS Language Features Extension - File Locator

**Partition:** 28/79 | **Scope:** `extensions/css-language-features/` (30 files, ~2,261 LOC)

**Research Question:** What would it take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust?

**Relevance:** This partition documents the CSS LSP client/server architecture as a reference implementation. Any Tauri/Rust port must preserve the LSP client wiring contract for built-in language extensions.

---

## Implementation

### Client-Side (Extension Host Entry Points)

**Platform-specific activation:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/node/cssClientMain.ts` — Node.js environment; instantiates `LanguageClient` with IPC transport; registers drop-or-paste resource support
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/browser/cssClientMain.ts` — Browser/web environment; spawns Worker thread; uses Worker-based transport instead of IPC

**Core LSP Client Bootstrap:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/cssClient.ts` — Shared LSP client initialization logic (both platforms). Exports `startClient()` function and `LanguageClientConstructor` type. Registers completion providers, formatter registration, custom data change notifications, and document formatting support.

**Request/Response Contracts:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/requests.ts` — Defines LSP filesystem request types: `FsContentRequest`, `FsStatRequest`, `FsReadDirRequest`. Implements bidirectional request handling (client serves FS requests to server).

**Feature Implementations:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/dropOrPaste/dropOrPasteResource.ts` — Document drop/paste edit provider for CSS URLs (relative path handling, snippet generation)
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/dropOrPaste/shared.ts` — Shared utilities for drop/paste (mime types, schemes, document directory resolution)
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/dropOrPaste/uriList.ts` — URI list parsing from clipboard/drag data
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/customData.ts` — Custom CSS data loader; monitors workspace config changes and extension contributions
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/src/node/nodeFs.ts` — Node.js filesystem request service implementation

### Server-Side (Language Server)

**Core Server Logic:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/cssServer.ts` — Main LSP server. Implements initialization protocol, handler registration for LSP methods (completion, hover, document symbols, definition, highlights, links, references, code actions, color, formatting, folding ranges, selection ranges, diagnostics). Manages stylesheet caching, settings management, and data provider lifecycle.

**Platform-specific Entry Points:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/node/cssServerMain.ts` — Node.js server entry point; creates IPC connection
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/node/cssServerNodeMain.ts` — Wrapper for ESM build
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/browser/cssServerMain.ts` — Browser worker entry point; uses BrowserMessageReader/Writer
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/browser/cssServerWorkerMain.ts` — Worker thread variant

**Request/Response Contracts:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/requests.ts` — Server-side filesystem request handler; dispatches to built-in handlers (file, http/https) or delegates to client via LSP protocol

**Supporting Utilities:**
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/languageModelCache.ts` — LRU cache for parsed CSS/SCSS/LESS stylesheets with time-based eviction
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/customData.ts` — Server-side custom CSS data provider fetching
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/node/nodeFs.ts` — Node.js filesystem provider
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/utils/strings.ts` — String utilities
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/utils/documentContext.ts` — Document context builder for path resolution
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/utils/validation.ts` — Diagnostic push/pull support (both LSP 3.16+ pull and push semantics)
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/utils/runner.ts` — Safe async execution wrapper with error handling

---

## Tests

### Server Tests (Node.js)
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/test/completion.test.ts` — Completion feature tests using node:test framework; validates CSS url() path completion, fixture-based assertions
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/src/test/links.test.ts` — Document link tests

### Test Fixtures
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/` — Path completion test data (HTML, CSS, SCSS, JS files; nested directories)
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/linksTestFixtures/` — Links test fixtures

### Test Runner
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/index.js` — Node.js native test runner (node:test module); outputs spec/JUnit formats

---

## Types / Interfaces

### Client Types
- `LanguageClientConstructor` (exported from `cssClient.ts`) — Function signature for platform-specific language client creation
- `Runtime` (exported from `cssClient.ts`) — Abstraction for platform-specific runtime: TextDecoder and optional RequestService (filesystem)

### Server Types
- `Settings` (exported from `cssServer.ts`) — Nested language settings for css/scss/less
- `RuntimeEnvironment` (exported from `cssServer.ts`) — Server-side runtime: file/http RequestService, timer (setImmediate/setTimeout)

### Shared Request/Response Types
- `FsContentRequest.type` — RequestType<{uri, encoding?}, string>
- `FsStatRequest.type` — RequestType<string, FileStat>
- `FsReadDirRequest.type` — RequestType<string, [string, FileType][]>
- `FileStat` interface — {type, ctime, mtime, size}
- `FileType` enum — Unknown, File, Directory, SymbolicLink
- `RequestService` interface — {getContent, stat, readDirectory}
- `ItemDescription` interface (in tests) — {label, resultText?}

### Cache Types
- `LanguageModelCache<T>` interface — {get, onDocumentRemoved, dispose}

---

## Configuration

### Extension Manifest
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/package.json` — Defines 3 language configurations (css, scss, less) with ~40 lint rules, format settings, completion options. Activation events: onLanguage:css/scss/less. Entry points: `client/out/node/cssClientMain` (node), `client/dist/browser/cssClientMain` (browser)

### Language Server Manifest
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/package.json` — Declares vscode-css-languageservice and vscode-languageserver dependencies; provides ESM module type

### Build Configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/esbuild.mts` — esbuild configuration for dual platform builds (node client/server). Uses esm format for server, builds with vscode-uri external
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/esbuild.browser.mts` — Browser build configuration (if present)

### TypeScript Configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/tsconfig.json` — Client TypeScript settings
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/client/tsconfig.browser.json` — Browser client TypeScript settings
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/tsconfig.json` — Server TypeScript settings
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/tsconfig.browser.json` — Browser server TypeScript settings

### VSCode Dev Workspace
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/.vscode/launch.json` — Debug launch configurations
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/.vscode/tasks.json` — Build/compile tasks
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/.vscode/settings.json` — Workspace settings

### Metadata
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/.npmrc` — NPM configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/.vscodeignore` — Files excluded from extension packaging
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/package-lock.json` — Locked dependencies
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/package-lock.json` — Server dependencies locked
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/package.nls.json` — Localization strings

### JSON Schema
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/schemas/package.schema.json` — JSON validation schema for package.json

---

## Examples / Fixtures

### Test Fixtures (Path Completion)
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/about/about.html`
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/about/about.css`
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/scss/_foo.scss`
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/scss/main.scss`
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/src/test.js`
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/src/data/foo.asar`
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/src/feature.js`
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/server/test/pathCompletionFixtures/index.html`

### Asset
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/icons/css.png` — Extension icon

---

## Documentation

- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/README.md` — Basic readme (bundled extension notice, links to main docs)
- `/home/norinlavaee/projects/vscode-atomic/extensions/css-language-features/CONTRIBUTING.md` — Contribution guidelines

---

## Notable Clusters

**LSP Client Platform Abstraction:**
The extension implements dual-platform LSP client activation (Node.js and Browser). The core `startClient()` function is platform-agnostic and accepts a `LanguageClientConstructor` callback, allowing `cssClientMain.ts` (node) and `cssClientMain.ts` (browser) to provide platform-specific LanguageClient instances with appropriate transport (IPC vs. Worker).

**Bidirectional Request Contract:**
The filesystem request service establishes a bidirectional request pattern: the client exposes `FsContentRequest`, `FsStatRequest`, and `FsReadDirRequest` handlers, while the server uses these to read import paths, stylesheets, and custom data files. This allows the server to remain platform-agnostic while delegating filesystem access to the client.

**Stylesheet Caching Layer:**
The `LanguageModelCache<Stylesheet>` maintains an LRU cache of parsed stylesheets (max 10 entries, 60-second eviction). This optimization prevents reparsing unchanged documents across multiple language service operations (completion, hover, validation).

**Configuration Synchronization:**
The client monitors both workspace configuration (`css.customData`, `css.format.*`, `css.lint.*`) and extension contribution points. Changes trigger server notifications via `CustomDataChangedNotification`, allowing dynamic hot-reload of custom CSS property definitions without server restart.

**Diagnostics Dual-Mode Support:**
The server registers either pull-based (LSP 3.16+) or push-based diagnostic support depending on client capabilities, allowing graceful fallback to older clients while supporting modern diagnostic pull semantics.

**Multi-Format Language Server:**
The same server (cssServer.ts) handles CSS, SCSS, and LESS languages. Document language ID routes to the appropriate language service (getCSSLanguageService, getSCSSLanguageService, getLESSLanguageService).

---

## Porting Implications for Tauri/Rust

**Critical LSP Client Wiring Contract:**
- The CSS extension uses `vscode-languageclient/node` (TypeScript) which wraps LSP protocol over IPC. A Tauri/Rust port must provide equivalent LSP client scaffolding for built-in extensions, exposing the same initialization, activation, and request/response interfaces.

**Platform Abstraction Patterns:**
- Dual entry points (Node.js + Browser) demonstrate how VS Code abstracts platforms. Tauri would require a similar abstraction layer for browser/tauri-specific transport (likely WebSocket or message-passing).

**Runtime Dependencies:**
- The TextDecoder abstraction and filesystem RequestService are platform bridges. Tauri would need to expose equivalent APIs to the LSP client layer.

**Filesystem and IPC:**
- Current: IPC for Node.js process communication, Worker for browser. Tauri: Would require WebSocket, shared memory, or Tauri command protocol for client-server communication within same process or separate threads.

**Configuration/Settings Propagation:**
- Currently: LSP ConfigurationRequest and workspace change events. Tauri/Rust port must preserve this two-way settings sync.

---

## Summary

The CSS language features extension (30 files, ~2,261 LOC) exemplifies VS Code's LSP client/server architecture. It uses a clean abstraction where the core `startClient()` function accepts a `LanguageClientConstructor` callback, enabling both Node.js (IPC transport) and Browser (Worker transport) implementations from a single codebase. The server manages three CSS-variant languages (CSS, SCSS, LESS) through a delegating language service pattern and implements bidirectional filesystem requests to remain platform-agnostic. Configuration and diagnostics support LSP 3.16+ modern semantics with fallback support. Any Tauri/Rust port must preserve the LSP client wiring contract—specifically the initialization sequence, request/response types, and the ability for built-in extensions to activate and communicate with language servers via standardized LSP methods.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
