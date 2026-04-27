# Porting VS Code JSON Language Features to Tauri/Rust

## Research Scope
This document analyzes patterns in `extensions/json-language-features/` (17 TypeScript files, ~3,000 LOC) to identify what would be required to port VS Code's core JSON LSP client and server from TypeScript/Electron to Tauri/Rust.

---

## Core Architecture Patterns

### Pattern: Language Client Initialization (Platform-Specific)

**Where:** `extensions/json-language-features/client/src/node/jsonClientMain.ts:20-57`

Node.js variant using IPC transport:
```typescript
const serverOptions: ServerOptions = {
  run: { module: serverModule, transport: TransportKind.ipc },
  debug: { module: serverModule, transport: TransportKind.ipc, options: debugOptions }
};

const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
  return new LanguageClient(id, name, serverOptions, clientOptions);
};
```

**Where:** `extensions/json-language-features/client/src/browser/jsonClientMain.ts:14-22`

Browser variant using Web Workers:
```typescript
const worker = new Worker(serverMain.toString());
worker.postMessage({ i10lLocation: l10n.uri?.toString(false) ?? '' });

const newLanguageClient: LanguageClientConstructor = (id: string, name: string, clientOptions: LanguageClientOptions) => {
  return new LanguageClient(id, name, worker, clientOptions);
};
```

**Variations / call-sites:** Two transport implementations (IPC vs Worker) passed through `LanguageClientConstructor` function type, allowing both to coexist. Activation hooks in both `node/` and `browser/` directories.

---

### Pattern: Language Client Middleware Chain

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:281-384`

Complex middleware stack for intercepting LSP operations:
```typescript
const clientOptions: LanguageClientOptions = {
  documentSelector,
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
    provideCompletionItem(document, position, context, token, next) {
      // ... range adjustment logic
      const r = next(document, position, context, token);
      if (isThenable<CompletionItem[] | CompletionList | null | undefined>(r)) {
        return r.then(updateProposals);
      }
      return updateProposals(r);
    }
  }
};
```

**Variations / call-sites:** Middleware hooks for: `provideDiagnostics`, `handleDiagnostics`, `provideCompletionItem`, `provideHover`, `provideFoldingRanges`, `provideDocumentColors`, `provideDocumentSymbols`. Each intercepts LSP feature before/after server communication.

---

### Pattern: Bidirectional Request/Notification System

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:395-443`

Client handling server requests (reverse RPC):
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
  }
  // ... more scheme handlers
});
```

**Where:** `extensions/json-language-features/server/src/jsonServer.ts:296-316`

Server handling notifications and requests:
```typescript
connection.onNotification(SchemaAssociationNotification.type, associations => {
  schemaAssociations = associations;
  updateConfiguration();
});

connection.onNotification(SchemaContentChangeNotification.type, uriOrUris => {
  let needsRevalidation = false;
  if (Array.isArray(uriOrUris)) {
    for (const uri of uriOrUris) {
      if (languageService.resetSchema(uri)) {
        needsRevalidation = true;
      }
    }
  } else {
    needsRevalidation = languageService.resetSchema(uriOrUris);
  }
  if (needsRevalidation) {
    diagnosticsSupport?.requestRefresh();
  }
});

connection.onRequest(ForceValidateRequest.type, async uri => {
  const document = documents.get(uri);
  if (document) {
    updateConfiguration();
    return await validateTextDocument(document);
  }
  return [];
});
```

**Variations / call-sites:** Eight custom request/notification types defined via `namespace`: `VSCodeContentRequest`, `SchemaContentChangeNotification`, `ForceValidateRequest`, `ForceValidateAllRequest`, `LanguageStatusRequest`, `ValidateContentRequest`, `SchemaAssociationNotification`, `DocumentSortingRequest`.

---

### Pattern: Provider Registration & Dynamic Capability Negotiation

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:504-543`

Dynamic code action provider registration:
```typescript
toDispose.push(languages.registerCodeActionsProvider(documentSelector, {
  provideCodeActions(_document: TextDocument, _range: Range, context: CodeActionContext): CodeAction[] {
    const codeActions: CodeAction[] = [];
    for (const diagnostic of context.diagnostics) {
      if (typeof diagnostic.code !== 'number') {
        continue;
      }
      switch (diagnostic.code) {
        case ErrorCodes.UntrustedSchemaError: {
          const title = l10n.t('Configure Trusted Domains...');
          const action = new CodeAction(title, CodeActionKind.QuickFix);
          // ... action setup
          codeActions.push(action);
        }
        break;
      }
    }
    return codeActions;
  }
}, {
  providedCodeActionKinds: [CodeActionKind.QuickFix]
}));
```

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:578-608`

Dynamic formatter registration based on settings:
```typescript
function updateFormatterRegistration() {
  const formatEnabled = workspace.getConfiguration().get(SettingIds.enableFormatter);
  if (!formatEnabled && rangeFormatting) {
    rangeFormatting.dispose();
    rangeFormatting = undefined;
  } else if (formatEnabled && !rangeFormatting) {
    rangeFormatting = languages.registerDocumentRangeFormattingEditProvider(documentSelector, {
      provideDocumentRangeFormattingEdits(document: TextDocument, range: Range, options: FormattingOptions, token: CancellationToken) {
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
        return client.sendRequest(DocumentRangeFormattingRequest.type, params, token);
      }
    });
  }
}
```

**Variations / call-sites:** Server-side dynamic formatter registration in `jsonServer.ts:278-291` mirrors this pattern using `connection.client.register()`.

---

### Pattern: Runtime Abstraction Layer

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:146-153`

Client-side runtime interface:
```typescript
export interface Runtime {
  schemaRequests: SchemaRequestService;
  telemetry?: TelemetryReporter;
  readonly timer: {
    setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable;
  };
  logOutputChannel: LogOutputChannel;
}
```

**Where:** `extensions/json-language-features/server/src/jsonServer.ts:73-85`

Server-side runtime interface:
```typescript
export interface RuntimeEnvironment {
  file?: RequestService;
  http?: RequestService;
  configureHttpRequests?(proxy: string | undefined, strictSSL: boolean): void;
  readonly timer: {
    setImmediate(callback: (...args: any[]) => void, ...args: any[]): Disposable;
    setTimeout(callback: (...args: any[]) => void, ms: number, ...args: any[]): Disposable;
  };
}
```

**Where:** `extensions/json-language-features/server/src/node/jsonServerMain.ts:58-72`

Node.js runtime implementation:
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
```

**Where:** `extensions/json-language-features/server/src/browser/jsonServerMain.ts:18-29`

Browser runtime implementation (no file/http, stub setImmediate):
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

**Variations / call-sites:** Runtime injected into `startServer()` and `startClient()`. Allows environment-specific (Node vs Browser) implementations without changing core logic.

---

### Pattern: Document/Language Lifecycle Management

**Where:** `extensions/json-language-features/server/src/jsonServer.ts:427-437`

Language model cache with document lifecycle:
```typescript
const jsonDocuments = getLanguageModelCache<JSONDocument>(10, 60, document => languageService.parseJSONDocument(document));
documents.onDidClose(e => {
  jsonDocuments.onDocumentRemoved(e.document);
});
connection.onShutdown(() => {
  jsonDocuments.dispose();
});

function getJSONDocument(document: TextDocument): JSONDocument {
  return jsonDocuments.get(document);
}
```

**Where:** `extensions/json-language-features/client/src/languageParticipants.ts:31-78`

Client-side language participant discovery and tracking:
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

**Variations / call-sites:** `TextDocuments` from LSP manages document lifecycle; paired with custom cache layer for parsed AST reuse.

---

### Pattern: Configuration Cascading & Settings Sync

**Where:** `extensions/json-language-features/client/src/jsonClient.ts:643-905`

Client-side settings computation and distribution:
```typescript
function computeSettings(): Settings {
  const configuration = workspace.getConfiguration();
  const httpSettings = workspace.getConfiguration('http');

  const normalizeLimit = (settingValue: any) => Math.trunc(Math.max(0, Number(settingValue))) || 5000;

  resultLimit = normalizeLimit(workspace.getConfiguration().get(SettingIds.maxItemsComputed));
  const editorJSONSettings = workspace.getConfiguration(SettingIds.editorSection, { languageId: 'json' });
  const editorJSONCSettings = workspace.getConfiguration(SettingIds.editorSection, { languageId: 'jsonc' });

  jsonFoldingLimit = normalizeLimit(editorJSONSettings.get(SettingIds.foldingMaximumRegions));
  jsoncFoldingLimit = normalizeLimit(editorJSONCSettings.get(SettingIds.foldingMaximumRegions));
  // ... more limits
  
  const schemas: JSONSchemaSettings[] = [];

  const settings: Settings = {
    http: {
      proxy: httpSettings.get('proxy'),
      proxyStrictSSL: httpSettings.get('proxyStrictSSL')
    },
    json: {
      validate: { enable: configuration.get(SettingIds.enableValidation) },
      format: { enable: configuration.get(SettingIds.enableFormatter) },
      // ... more settings
    }
  };

  const collectSchemaSettings = (schemaSettings: JSONSchemaSettings[] | undefined, folderUri: string | undefined, settingsLocation: Uri | undefined) => {
    if (schemaSettings) {
      for (const setting of schemaSettings) {
        const url = getSchemaId(setting, settingsLocation);
        if (url) {
          const schemaSetting: JSONSchemaSettings = { url, fileMatch: setting.fileMatch, folderUri, schema: setting.schema };
          schemas.push(schemaSetting);
        }
      }
    }
  };

  const folders = workspace.workspaceFolders ?? [];
  const schemaConfigInfo = workspace.getConfiguration('json', null).inspect<JSONSchemaSettings[]>('schemas');
  
  if (schemaConfigInfo) {
    collectSchemaSettings(schemaConfigInfo.globalValue, undefined, undefined);
    if (workspace.workspaceFile) {
      if (schemaConfigInfo.workspaceValue) {
        const settingsLocation = Uri.joinPath(workspace.workspaceFile, '..');
        collectSchemaSettings(schemaConfigInfo.workspaceValue, undefined, settingsLocation);
      }
      for (const folder of folders) {
        const folderUri = folder.uri;
        const folderSchemaConfigInfo = workspace.getConfiguration('json', folderUri).inspect<JSONSchemaSettings[]>('schemas');
        collectSchemaSettings(folderSchemaConfigInfo?.workspaceFolderValue, folderUri.toString(false), folderUri);
      }
    }
  }
  return settings;
}
```

**Where:** `extensions/json-language-features/server/src/jsonServer.ts:258-293`

Server-side configuration change handling:
```typescript
connection.onDidChangeConfiguration((change) => {
  const settings = <Settings>change.settings;
  runtime.configureHttpRequests?.(settings?.http?.proxy, !!settings.http?.proxyStrictSSL);
  jsonConfigurationSettings = settings.json?.schemas;
  validateEnabled = !!settings.json?.validate?.enable;
  commentsSeverity = settings.json?.validate?.comments;
  trailingCommasSeverity = settings.json?.validate?.trailingCommas;
  schemaValidationSeverity = settings.json?.validate?.schemaValidation;
  schemaRequestSeverity = settings.json?.validate?.schemaRequest;
  keepLinesEnabled = settings.json?.keepLines?.enable || false;
  updateConfiguration();

  // Dynamic formatter registration
  if (dynamicFormatterRegistration) {
    const enableFormatter = settings.json?.format?.enable;
    if (enableFormatter) {
      if (!formatterRegistrations) {
        const documentSelector = [{ language: 'json' }, { language: 'jsonc' }];
        formatterRegistrations = [
          connection.client.register(DocumentRangeFormattingRequest.type, { documentSelector }),
          connection.client.register(DocumentFormattingRequest.type, { documentSelector })
        ];
      }
    } else if (formatterRegistrations) {
      formatterRegistrations.forEach(p => p.then(r => r.dispose()));
      formatterRegistrations = null;
    }
  }
});
```

**Variations / call-sites:** Multi-level configuration inspection (global, workspace, folder); folder-relative schema URL resolution; per-language settings (json vs jsonc).

---

## Key Integration Points Requiring Ports

**Custom Request/Notification Protocol** (8 types):
- `VSCodeContentRequest` - Server reverse-calls client for schema content
- `SchemaAssociationNotification` - Extension schema contributions
- `SchemaContentChangeNotification` - Schema cache invalidation
- `ForceValidateRequest/ForceValidateAllRequest` - Explicit validation triggers
- `LanguageStatusRequest` - Status query
- `ValidateContentRequest` - Adhoc validation
- `DocumentSortingRequest` - JSON sort operation

**Platform Bridging**:
- IPC transport layer (Node.js)
- Web Worker transport layer (Browser)
- File system request service
- HTTP request service with proxy support
- Timer abstraction (setImmediate, setTimeout)

**UI Integration** (14+ registrations):
- Command registration (cache clear, validate, sort, retry, trusted domain config)
- Language status items (schema validation status)
- Code action providers (quick fixes)
- Document range formatting providers
- File system watchers (schema change detection)
- Configuration change listeners
- Extension lifecycle listeners

---

## Summary

The JSON language features extension demonstrates a well-layered architecture separating concerns across:

1. **Transport**: Abstracted through `LanguageClientConstructor` and connection implementations
2. **Runtime**: Environment-specific implementations (Node/Browser) injected as `RuntimeEnvironment`
3. **Middleware**: LSP feature interception for client-side transformations
4. **Bidirectional Communication**: Custom request/notification types for editor-specific operations
5. **Lifecycle Management**: Document parsing cache with automatic cleanup tied to document events
6. **Configuration**: Multi-scope cascade (global → workspace → folder) with dynamic re-registration

Porting to Tauri/Rust would require:
- Rust implementation of LSP client protocol and custom message types
- Tauri message passing bridge replacing IPC/Worker transports
- Native file/HTTP request services
- Configuration management tied to Tauri settings/file system
- Document cache implementation equivalent to `LanguageModelCache<T>`
- Provider registration system for Rust-based language features

