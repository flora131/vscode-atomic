# Library Card: TypeScript tsserver Protocol (`typescript/src/server/protocol.ts`)

## Identity

| Field | Value |
|---|---|
| **Library / Spec** | TypeScript Language Server Protocol (`tsserver`) |
| **Canonical source** | `typescript/src/server/protocol.ts` (in the TypeScript compiler repo) |
| **Local mirror** | `extensions/typescript-language-features/src/tsServer/protocol/protocol.d.ts` (thin re-export: `export = ts.server.protocol`) |
| **Version sampled** | TypeScript `main` branch (April 2026) — 3 321-line file |
| **Why central** | Every language intelligence feature in `extensions/typescript-language-features/` — completions, hover, go-to-definition, references, rename, diagnostics, inlay hints, call hierarchy, code actions, refactors — is implemented by sending JSON messages that conform to the interfaces defined in this single file. |

---

## Transport Layer (wire format)

The extension forks `tsserver` as a child process (or worker) and communicates over its **stdin/stdout** using an HTTP-header-style framing protocol identical to LSP:

```
Content-Length: <byte-count>\r\n
\r\n
<JSON body>
```

Key implementation is in `extensions/typescript-language-features/src/tsServer/serverProcess.electron.ts`. The `ProtocolBuffer` class parses the `Content-Length:` header, then slices the exact byte count from a growing `Buffer`. JSON is then parsed and dispatched. This is **not** LSP — it is tsserver's own binary-framing over stdio, predating LSP.

---

## Top-level Message Shape

Every message (request, response, or event) is a subtype of `Message`:

```typescript
export interface Message {
    seq: number;
    type: "request" | "response" | "event";
}
```

**Client-to-server (`Request`):**
```typescript
export interface Request extends Message {
    type: "request";
    command: string;   // value from CommandTypes enum
    arguments?: any;
}
```

**Server-to-client (`Response`):**
```typescript
export interface Response extends Message {
    type: "response";
    request_seq: number;
    success: boolean;
    command: string;
    message?: string;
    body?: any;
    metadata?: unknown;
    performanceData?: PerformanceData;
}
```

**Server-pushed (`Event`):**
```typescript
export interface Event extends Message {
    type: "event";
    event: string;   // e.g. "semanticDiag", "syntaxDiag", "suggestionDiag"
    body?: any;
}
```

---

## `CommandTypes` Enum (full surface, selected)

The complete public API is encoded as `const enum CommandTypes`. Key values used by the VS Code extension (from `typescriptService.ts`'s `TypeScriptRequests` map):

| Command string | Purpose |
|---|---|
| `"completionInfo"` | Trigger completion list |
| `"completionEntryDetails"` | Detailed completion item docs |
| `"quickinfo"` | Hover / type-at-cursor |
| `"definition"` | Go to definition |
| `"definitionAndBoundSpan"` | Go to definition + highlight span |
| `"typeDefinition"` | Go to type definition |
| `"implementation"` | Go to implementation |
| `"references"` | Find all references |
| `"fileReferences"` | Find file-level references |
| `"rename"` | Rename symbol |
| `"signatureHelp"` | Parameter hints |
| `"documentHighlights"` | Highlight all occurrences |
| `"navto"` | Workspace symbol search |
| `"navtree"` | Document symbol outline |
| `"format"` / `"formatonkey"` | Formatting |
| `"getApplicableRefactors"` | List refactors at cursor |
| `"getEditsForRefactor"` | Execute refactor |
| `"organizeImports"` | Organize imports |
| `"getCodeFixes"` | Code fixes (quick fixes) |
| `"getCombinedCodeFix"` | Batch code fix |
| `"geterr"` / `"geterrForProject"` | Pull diagnostics (async) |
| `"semanticDiagnosticsSync"` | Sync semantic diagnostics |
| `"syntacticDiagnosticsSync"` | Sync syntactic diagnostics |
| `"provideInlayHints"` | Inlay hints |
| `"prepareCallHierarchy"` | Call hierarchy root |
| `"provideCallHierarchyIncomingCalls"` | Callers |
| `"provideCallHierarchyOutgoingCalls"` | Callees |
| `"selectionRange"` | Smart selection expand |
| `"open"` / `"close"` / `"change"` / `"updateOpen"` | File lifecycle |
| `"configure"` | Set host/format preferences |
| `"compilerOptionsForInferredProjects"` | Configure inferred project |
| `"getEditsForFileRename"` | Rename file side-effects |
| `"mapCode"` | Copilot code mapping |
| `"getPasteEdits"` / `"preparePasteEdits"` | Smart paste |

---

## Selected Critical Request/Response Pairs

### File Location (basis for most requests)
```typescript
export interface FileLocationRequestArgs extends FileRequestArgs {
    line: number;   // 1-based
    offset: number; // 1-based (character offset on the line)
}
```
Note: **1-based** line and character, unlike LSP which uses 0-based. The `typeConverters.ts` file bridges this:
```typescript
// vscode Position (0-based) -> tsserver Location (1-based)
export const toLocation = (vsPosition: vscode.Position): Proto.Location => ({
    line: vsPosition.line + 1,
    offset: vsPosition.character + 1,
});
```

### Quickinfo (Hover)
```typescript
export interface QuickInfoResponseBody {
    kind: ScriptElementKind;
    kindModifiers: string;
    start: Location;
    end: Location;
    displayString: string;
    documentation: string | SymbolDisplayPart[];
    tags: JSDocTagInfo[];
    canIncreaseVerbosityLevel?: boolean;
}
```

### Completions
```typescript
export interface CompletionsRequestArgs extends FileLocationRequestArgs {
    prefix?: string;
    triggerCharacter?: CompletionsTriggerCharacter;
    triggerKind?: CompletionTriggerKind;
    includeExternalModuleExports?: boolean;
    includeInsertTextCompletions?: boolean;
}
```

### Diagnostics (push, event-based)
Diagnostics are NOT returned synchronously per request. The client sends `geterr` or `geterrForProject`; the server pushes back `Event` messages with `event` set to:
- `"syntaxDiag"` — syntax errors
- `"semanticDiag"` — type errors
- `"suggestionDiag"` — suggestions / hints

### Go-to-definition response
```typescript
export interface DefinitionInfo extends FileSpanWithContext {
    unverified?: boolean;
}
export interface DefinitionInfoAndBoundSpan {
    definitions: readonly DefinitionInfo[];
    textSpan: TextSpan;   // the span at the call site
}
```

### Code Edit (used in all write-back features)
```typescript
export interface CodeEdit {
    start: Location;
    end: Location;
    newText: string;
}
export interface FileCodeEdits {
    fileName: string;
    textChanges: CodeEdit[];
}
```

### References
```typescript
export interface ReferencesResponseItem extends FileSpanWithContext {
    lineText?: string;
    isWriteAccess: boolean;
    isDefinition?: boolean;
}
export interface ReferencesResponseBody {
    refs: readonly ReferencesResponseItem[];
    symbolName: string;
    symbolStartOffset: number;
    symbolDisplayString: string;
}
```

---

## TypeScript Requests Map (complete, from `typescriptService.ts`)

The extension declares its full tsserver API surface in a single TypeScript type:

```typescript
interface StandardTsServerRequests {
    'applyCodeActionCommand': [ApplyCodeActionCommandRequestArgs, ApplyCodeActionCommandResponse];
    'completionEntryDetails': [CompletionDetailsRequestArgs, CompletionDetailsResponse];
    'completionInfo': [CompletionsRequestArgs, CompletionInfoResponse];
    'completions': [CompletionsRequestArgs, CompletionsResponse];
    'configure': [ConfigureRequestArguments, ConfigureResponse];
    'definition': [FileLocationRequestArgs, DefinitionResponse];
    'definitionAndBoundSpan': [FileLocationRequestArgs, DefinitionInfoAndBoundSpanResponse];
    'docCommentTemplate': [FileLocationRequestArgs, DocCommandTemplateResponse];
    'documentHighlights': [DocumentHighlightsRequestArgs, DocumentHighlightsResponse];
    'format': [FormatRequestArgs, FormatResponse];
    'formatonkey': [FormatOnKeyRequestArgs, FormatResponse];
    'getApplicableRefactors': [GetApplicableRefactorsRequestArgs, GetApplicableRefactorsResponse];
    'getCodeFixes': [CodeFixRequestArgs, CodeFixResponse];
    'getCombinedCodeFix': [GetCombinedCodeFixRequestArgs, GetCombinedCodeFixResponse];
    'getEditsForFileRename': [GetEditsForFileRenameRequestArgs, GetEditsForFileRenameResponse];
    'getEditsForRefactor': [GetEditsForRefactorRequestArgs, GetEditsForRefactorResponse];
    'getOutliningSpans': [FileRequestArgs, OutliningSpansResponse];
    'getSupportedCodeFixes': [null, GetSupportedCodeFixesResponse];
    'implementation': [FileLocationRequestArgs, ImplementationResponse];
    'jsxClosingTag': [JsxClosingTagRequestArgs, JsxClosingTagResponse];
    'navto': [NavtoRequestArgs, NavtoResponse];
    'navtree': [FileRequestArgs, NavTreeResponse];
    'organizeImports': [OrganizeImportsRequestArgs, OrganizeImportsResponse];
    'projectInfo': [ProjectInfoRequestArgs, ProjectInfoResponse];
    'quickinfo': [FileLocationRequestArgs, QuickInfoResponse];
    'references': [FileLocationRequestArgs, ReferencesResponse];
    'rename': [RenameRequestArgs, RenameResponse];
    'selectionRange': [SelectionRangeRequestArgs, SelectionRangeResponse];
    'signatureHelp': [SignatureHelpRequestArgs, SignatureHelpResponse];
    'typeDefinition': [FileLocationRequestArgs, TypeDefinitionResponse];
    'updateOpen': [UpdateOpenRequestArgs, Response];
    'prepareCallHierarchy': [FileLocationRequestArgs, PrepareCallHierarchyResponse];
    'provideCallHierarchyIncomingCalls': [FileLocationRequestArgs, ProvideCallHierarchyIncomingCallsResponse];
    'provideCallHierarchyOutgoingCalls': [FileLocationRequestArgs, ProvideCallHierarchyOutgoingCallsResponse];
    'fileReferences': [FileRequestArgs, FileReferencesResponse];
    'provideInlayHints': [InlayHintsRequestArgs, InlayHintsResponse];
    'encodedSemanticClassifications-full': [EncodedSemanticClassificationsRequestArgs, EncodedSemanticClassificationsResponse];
    'findSourceDefinition': [FileLocationRequestArgs, DefinitionResponse];
    'getMoveToRefactoringFileSuggestions': [GetMoveToRefactoringFileSuggestionsRequestArgs, GetMoveToRefactoringFileSuggestions];
    'linkedEditingRange': [FileLocationRequestArgs, LinkedEditingRangeResponse];
    'mapCode': [MapCodeRequestArgs, MapCodeResponse];
    'getPasteEdits': [GetPasteEditsRequestArgs, GetPasteEditsResponse];
    'preparePasteEdits': [PreparePasteEditsRequestArgs, PreparePasteEditsResponse];
}

interface NoResponseTsServerRequests {
    'open': [OpenRequestArgs, null];
    'close': [FileRequestArgs, null];
    'change': [ChangeRequestArgs, null];
    'compilerOptionsForInferredProjects': [SetCompilerOptionsForInferredProjectsArgs, null];
    'reloadProjects': [null, null];
    'configurePlugin': [ConfigurePluginRequest, ConfigurePluginResponse];
    'watchChange': [Request, null];
}

interface AsyncTsServerRequests {
    'geterr': [GeterrRequestArgs, Response];
    'geterrForProject': [GeterrForProjectRequestArgs, Response];
}
```

---

## Relationship to LSP

The tsserver protocol predates and differs from LSP in several ways:

| Dimension | tsserver | LSP |
|---|---|---|
| Transport | stdin/stdout with `Content-Length` framing | Same framing, or sockets |
| Position encoding | 1-based line and 1-based character offset | 0-based line and 0-based character |
| Diagnostics | Push-based events (`geterr` triggers async events) | Push-based notifications (`textDocument/publishDiagnostics`) |
| Method dispatch | `command` string field (`CommandTypes` enum) | `method` string field (`textDocument/completion`, etc.) |
| File lifecycle | Explicit `open`/`close`/`change` commands | `textDocument/didOpen`, `didChange`, `didClose` |
| Spec | Not publicly versioned; defined by `protocol.ts` in TypeScript repo | Versioned JSON Schema at microsoft.github.io/language-server-protocol |

The extension does NOT use LSP internally; it speaks tsserver's own protocol directly. A separate `typescript-language-server` (community project, `typescript-language-features` does not use it) wraps tsserver behind a true LSP facade.

---

## Implications for Tauri/Rust Port

1. **tsserver process must still run**: TypeScript analysis requires the TypeScript compiler runtime. A Tauri/Rust shell cannot replace `tsserver` itself; it must still spawn Node.js running `tsserver` (or the bundled TypeScript worker in the web variant).

2. **IPC bridge required**: The Rust process (Tauri backend) or its WebView (frontend) must replicate the `ProtocolBuffer` framing logic: write `Content-Length: N\r\n\r\n{JSON}` to tsserver's stdin, and parse the same framing from stdout.

3. **Coordinator split**: Today `typescriptServiceClient.ts` runs in the extension host (Node.js). In Tauri it would need to live in either the WebView's JS context (calling through Tauri's IPC bridge to a Rust-owned child process) or in a Rust async task managing the child process.

4. **Coordinate system translation**: All position conversions (1-based tsserver ↔ 0-based VS Code `Position`) are concentrated in `typeConverters.ts`. This translation layer must be preserved.

5. **Multiple server instances**: VS Code runs up to 3 concurrent tsserver instances (`main`, `syntax`, `semantic`, `diagnostics` — see `TsServerProcessKind`). Each instance speaks the same protocol but handles different request subsets. The Tauri port must manage multiple long-lived child processes.

6. **No LSP migration shortcut without loss**: Migrating to `typescript-language-server` (true LSP) would simplify the bridge but drops tsserver-specific commands (`mapCode`, `preparePasteEdits`, `encodedSemanticClassifications-full`, Copilot-related internals) that have no LSP equivalent.

---

## Source References

- `typescript/src/server/protocol.ts` — https://github.com/microsoft/TypeScript/blob/main/src/server/protocol.ts (3 321 lines; canonical)
- Local re-export: `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/tsServer/protocol/protocol.d.ts` (23 lines; thin shim)
- TypeScript request surface: `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/typescriptService.ts` (100 lines; `TypeScriptRequests` type)
- Wire framing: `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/tsServer/serverProcess.electron.ts`
- Coordinate converters: `/Users/norinlavaee/vscode-atomic/extensions/typescript-language-features/src/typeConverters.ts`
