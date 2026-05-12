### Files Analysed

- `src/vscode-dts/vscode.d.ts` — Main stable extension API (21,235 lines)
- `src/vscode-dts/vscode.proposed.debugVisualization.d.ts` — Proposed debug visualization provider API
- `src/vscode-dts/vscode.proposed.languageModelSystem.d.ts` — Proposed LLM system message role enum
- `src/vscode-dts/vscode.proposed.scmHistoryProvider.d.ts` — Proposed SCM history / git graph API
- `src/vscode-dts/vscode.proposed.chatProvider.d.ts` — Proposed language-model chat-provider registration API
- `src/vscode-dts/vscode.proposed.terminalCompletionProvider.d.ts` — Proposed terminal completion provider API
- `src/vscode-dts/vscode.proposed.fileSearchProvider.d.ts` — Proposed file search provider API
- `src/vscode-dts/vscode.proposed.resolvers.d.ts` — Proposed remote-authority resolver API (tunnels, exec servers, remote FS)
- `src/vscode-dts/vscode.proposed.notebookKernelSource.d.ts` — Proposed notebook kernel source action provider
- `src/vscode-dts/vscode.proposed.workspaceTrust.d.ts` — Proposed workspace/resource trust API

---

### Per-File Notes

#### `src/vscode-dts/vscode.d.ts`

- **Role:** The single canonical public contract for all VS Code extensions; every top-level capability that must be preserved (or replaced by Rust ABI) in a Tauri port lives here.

- **Key symbols:**

  **Primitive model types**
  - `TextDocument` (`vscode.d.ts:88`) — immutable document handle: `uri`, `languageId`, `version`, `getText()`, `lineAt()`, `offsetAt()`, `positionAt()`.  All LSP interactions pass through this type.
  - `TextEditor` (`vscode.d.ts:1258`) — bidirectional editor handle: `edit(callback)` at line 1304 takes an `TextEditorEdit` builder (replace/insert/delete/setEndOfLine); `insertSnippet()` at line 1326; `setDecorations()` at line 1352.
  - `Position` / `Range` / `Selection` (`vscode.d.ts:269`, 408, 518) — immutable value objects used ubiquitously as coordinates in documents and the editor.
  - `Uri` (`vscode.d.ts:1439`) — scheme-authority-path-query-fragment abstraction; static factories `Uri.file()` (line 1479) and `Uri.parse()` (line 1454).

  **`namespace window` (`vscode.d.ts:11069`)**
  - State properties: `activeTextEditor` (11081), `visibleTextEditors` (11086), `terminals` (11161), `activeTerminal` (11167), `state: WindowState` (11214).
  - Event bus: `onDidChangeActiveTextEditor` (11093), `onDidChangeTextEditorSelection` (11104), `onDidOpenTerminal` (11180), `onDidCloseTerminal` (11185), `onDidChangeTerminalShellIntegration` (11195), `onDidStartTerminalShellExecution` (11202), `onDidEndTerminalShellExecution` (11209).
  - Factories: `showTextDocument()` (11233), `createTextEditorDecorationType()` (11272), `showQuickPick()` (11418), `showInputBox()` (11488), `createQuickPick()` (11499), `createInputBox()` (11510), `createOutputChannel()` (11523), `createWebviewPanel()` (11544), `createStatusBarItem()` (11638), `createTerminal()` (11662).

  **`namespace workspace` (`vscode.d.ts:13799`)**
  - `fs: FileSystem` (13807) — virtual file system interface.
  - `workspaceFolders: readonly WorkspaceFolder[] | undefined` (13827).
  - `createFileSystemWatcher()` (14076) — file-change event subscription.
  - `findFiles()` (14095), `openTextDocument()` (14172), `applyEdit()` (14146).
  - `textDocuments: readonly TextDocument[]` (14151).

  **`namespace languages` (`vscode.d.ts:14724`)**
  - Provider registration surface: `registerCompletionItemProvider()` (14849), `registerInlineCompletionItemProvider()` (14862), `registerCodeActionsProvider()` (14876), `registerCodeLensProvider()` (14889), `registerDefinitionProvider()` (14902), `registerImplementationProvider()` (14915), `registerTypeDefinitionProvider()` (14928), `registerHoverProvider()` (14954), `registerDocumentHighlightProvider()` (14994), `registerDocumentSymbolProvider()` (15008), `registerWorkspaceSymbolProvider()` (15020), `registerReferenceProvider()` (15033), `registerRenameProvider()` (15046), `registerDocumentSemanticTokensProvider()` (15059), `registerDocumentRangeSemanticTokensProvider()` (15078), `registerDocumentFormattingEditProvider()` (15091), `registerDocumentRangeFormattingEditProvider()` (15108), `registerOnTypeFormattingEditProvider()` (15123), `registerEvaluatableExpressionProvider()` (14966), `registerInlineValuesProvider()` (14981).
  - `getDiagnostics()` (14804/14811), `createDiagnosticCollection()` (14819).

  **`namespace debug` (`vscode.d.ts:17285`)**
  - State: `activeDebugSession: DebugSession | undefined` (17292), `breakpoints: readonly Breakpoint[]` (17303), `activeStackItem: DebugThread | DebugStackFrame | undefined` (17338).
  - Events: `onDidChangeActiveDebugSession` (17310), `onDidStartDebugSession` (17315), `onDidTerminateDebugSession` (17325), `onDidChangeBreakpoints` (17330), `onDidChangeActiveStackItem` (17343).
  - Actions: `registerDebugConfigurationProvider()` (17359), `registerDebugAdapterDescriptorFactory()` (17370), `startDebugging()` (17392), `stopDebugging()` (17400), `addBreakpoints()` (17406), `asDebugSourceUri()` (17425).
  - DAP bridge types: `DebugAdapterExecutable` (16845), `DebugAdapterServer` (16896), `DebugAdapterNamedPipeServer` (16917), `DebugAdapter` interface (16932), `DebugSession.customRequest()` (16764).

  **`namespace scm` (`vscode.d.ts:16654`)**
  - `createSourceControl(id, label, rootUri?)` (16672) — factory for `SourceControl` objects that expose resource groups, commit input, status bar commands, and (proposed) `historyProvider`.

  **`namespace tasks` (`vscode.d.ts:9347`)**
  - `registerTaskProvider()` (9356), `fetchTasks()` (9366), `executeTask()` (9379).
  - Events: `onDidStartTask` (9389), `onDidEndTask` (9394), `onDidStartTaskProcess` (9401), `onDidEndTaskProcess` (9408).

  **`namespace env` (`vscode.d.ts:10739`)**
  - `appName`, `appRoot`, `appHost`, `uriScheme`, `language`, `clipboard`, `machineId`, `shell`, `uiKind: UIKind` (10744–10854).
  - `openExternal()` (10869), `asExternalUri()` (10924) — URI resolution / external link handling critical for both local and remote scenarios.
  - `remoteName: string | undefined` (10840).

  **`namespace lm` (`vscode.d.ts:20734`)**
  - `selectChatModels()` (20766), `registerTool()` (20774), `invokeTool()` (20808), `registerLanguageModelChatProvider()` (20847), `registerMcpServerDefinitionProvider()` (20838).

  **`namespace chat` (`vscode.d.ts:20113`)**
  - `createChatParticipant()` (20121) — entry point for Copilot-style chat integration.

  **`FileSystemProvider` interface** (referenced throughout workspace and `languages` namespaces) — defines `stat`, `readDirectory`, `readFile`, `writeFile`, `rename`, `delete`, `createDirectory` as `Thenable<...>` calls.

- **Control flow:**
  - Extensions activate (`activate(context: ExtensionContext)`) and receive a context with `subscriptions: Disposable[]`. All registrations return `Disposable`s pushed into that array.
  - Provider registrations add handlers into per-language / per-type registries inside the editor host; events propagate back to extensions via `Event<T>` (EventEmitter pattern).
  - Text edits flow: extension calls `TextEditor.edit(builder => ...)` → builder collects operations → returned `Thenable<boolean>` resolves after the edit transaction is applied by the editor core.
  - Debug flow: `debug.startDebugging()` → resolves `DebugConfiguration` via chain of `DebugConfigurationProvider.resolveDebugConfiguration()` calls → spawns `DebugAdapter` (executable, socket, or named pipe) → DAP messages exchanged via `DebugAdapter.handleMessage()` / `onDidSendMessage`.

- **Data flow:**
  - Documents enter as `Uri` → `workspace.openTextDocument()` returns `TextDocument` → `window.showTextDocument()` returns `TextEditor`.
  - Language intelligence: `DocumentSelector` filters providers → provider returns `ProviderResult<CompletionList | Hover | Location[] | …>` → editor core consumes and renders.
  - Filesystem: reads/writes as `Uint8Array` through `FileSystemProvider`; path coordinates expressed as `Uri`.

- **Dependencies:** No imports; this is a `declare module 'vscode'` ambient declaration consumed by extension authors and implemented by `src/vs/workbench/` internals.

---

#### `src/vscode-dts/vscode.proposed.debugVisualization.d.ts`

- **Role:** Extends `namespace debug` with a custom variable-visualization provider API, revealing how debugging state (DAP variable objects) flows to UI components.

- **Key symbols:**
  - `debug.registerDebugVisualizationProvider<T>(id, provider)` (line 15) — registers provider against a `package.json` contribution point `debugVisualizers`.
  - `debug.registerDebugVisualizationTreeProvider<T>(id, provider)` (line 25) — registers a tree used as an in-panel or hover visualization.
  - `DebugVisualizationContext` (line 141) — carries the raw DAP `variable` object, `containerId`, `frameId`, `threadId`, and `session: DebugSession` reference. This is the ABI boundary between the DAP wire format and extension-land.
  - `DebugVisualizationProvider.provideDebugVisualization(context, token)` (line 128) — called every time a session stops; returns `DebugVisualization[]`.
  - `DebugVisualization.visualization` (line 110) — may be a `Command` or `{ treeId: string }` reference.

- **Control flow:** Session pauses → editor iterates registered providers whose `when` clause matches → `provideDebugVisualization()` called → result shown in UI; user picks visualizer → `resolveDebugVisualization()` called → optional `TreeView` returned.

- **Data flow:** Raw DAP JSON (`variable: any` at line 146) in, typed `DebugVisualization[]` out. No document or filesystem involvement.

- **Dependencies:** Uses `DebugSession`, `TreeItemCollapsibleState`, `Command`, `Uri`, `ThemeIcon`, `CancellationToken`, `ProviderResult` from the stable API.

---

#### `src/vscode-dts/vscode.proposed.scmHistoryProvider.d.ts`

- **Role:** Augments `SourceControl` with a `historyProvider` property exposing a git-log–style history graph API, defining the full data contract for SCM history views.

- **Key symbols:**
  - `SourceControl.historyProvider?: SourceControlHistoryProvider` (line 10).
  - `SourceControlHistoryProvider` (line 13): `currentHistoryItemRef` / `currentHistoryItemRemoteRef` / `currentHistoryItemBaseRef` — branch/commit refs; `onDidChangeCurrentHistoryItemRefs`, `onDidChangeHistoryItemRefs` events.
  - `provideHistoryItems(options, token)` (line 30) — paged commit log; `options` carries `skip`, `limit`, `historyItemRefs`, `filterText`.
  - `provideHistoryItemChanges(historyItemId, historyItemParentId, token)` (line 31) — diff for a commit.
  - `resolveHistoryItemChatContext` / `resolveHistoryItemChangeRangeChatContext` (lines 34–35) — AI-specific: returns plain-text context strings for chat participants.
  - `SourceControlHistoryItem` (line 52): `id`, `parentIds`, `subject`, `author`, `timestamp`, `statistics`, `references`.

- **Data flow:** `scm.createSourceControl()` → assign `.historyProvider` → editor calls `provideHistoryItems()` → `SourceControlHistoryItem[]` rendered in Source Control History view; diff requests call `provideHistoryItemChanges()` → `SourceControlHistoryItemChange[]` (uri triples: current/original/modified).

- **Dependencies:** `Uri`, `Event`, `CancellationToken`, `ProviderResult`, `IconPath`, `MarkdownString` from stable API.

---

#### `src/vscode-dts/vscode.proposed.chatProvider.d.ts`

- **Role:** Defines the provider-side contract for registering a language model with the editor's `lm.registerLanguageModelChatProvider()` entry point, including capability negotiation, streaming responses, and per-model configuration schemas.

- **Key symbols:**
  - `LanguageModelChatProvider<T>` (line 134): `provideLanguageModelChatInformation(options, token)` → `T[]`; `provideLanguageModelChatResponse(model, messages, options, progress, token)` → `Thenable<void>` (pushes `LanguageModelResponsePart2` via `progress`).
  - `LanguageModelChatInformation` (line 34): `requiresAuthorization`, `multiplierNumeric`, `isDefault`, `isUserSelectable`, `category`, `configurationSchema`.
  - `LanguageModelChatCapabilities.editTools: string[]` (line 96) — hints which edit tools (`find-replace`, `apply-patch`, `code-rewrite`, etc.) the model prefers; editor selects the appropriate edit flow.
  - `LanguageModelConfigurationSchema` (line 117) — JSON Schema fragment with `properties` and `enumItemLabels` for the model settings UI.
  - `ProvideLanguageModelChatResponseOptions.requestInitiator` (line 19) — identifies calling extension.

- **Data flow:** `lm.registerLanguageModelChatProvider(vendor, provider)` → provider's `provideLanguageModelChatInformation()` called to enumerate models → user selects model → `provideLanguageModelChatResponse()` streams `LanguageModelResponsePart2` (text, tool calls, thinking parts, data parts) via `Progress`.

- **Dependencies:** `LanguageModelChatRequestMessage`, `LanguageModelResponsePart`, `LanguageModelDataPart`, `LanguageModelThinkingPart`, `Progress`, `CancellationToken`, `ThemeIcon`, `ChatLocation`, `ChatRequest` from stable and other proposed APIs.

---

#### `src/vscode-dts/vscode.proposed.terminalCompletionProvider.d.ts`

- **Role:** Adds shell-completion capability to the terminal via `window.registerTerminalCompletionProvider()`, defining the data types for command-line aware completions.

- **Key symbols:**
  - `window.registerTerminalCompletionProvider<T>(provider, ...triggerCharacters)` (line 283) — added to the stable `window` namespace.
  - `TerminalCompletionProvider.provideTerminalCompletions(terminal, context, token)` (line 31) — returns `T[] | TerminalCompletionList<T>`.
  - `TerminalCompletionItem` (line 48): `label`, `replacementRange: readonly [number, number]`, `detail`, `documentation`, `kind: TerminalCompletionItemKind`.
  - `TerminalCompletionItemKind` (line 96) — 17 values: `File`, `Folder`, `Method`, `Alias`, `Argument`, `Option`, `OptionValue`, `Flag`, `SymbolicLinkFile`, `SymbolicLinkFolder`, `ScmCommit`, `ScmBranch`, `ScmTag`, `ScmStash`, `ScmRemote`, `PullRequest`, `PullRequestDone`.
  - `TerminalCompletionContext` (line 189): `commandLine: string`, `cursorIndex: number`.
  - `TerminalCompletionList.resourceOptions?: TerminalCompletionResourceOptions` (line 215) — instructs the terminal to surface file/folder entries from a given `cwd: Uri` with a glob pattern.

- **Data flow:** Keystroke in terminal (or trigger character) → editor calls `provideTerminalCompletions(terminal, { commandLine, cursorIndex }, token)` → provider returns `TerminalCompletionList` → editor merges with resource completions and renders inline.

- **Dependencies:** `Terminal`, `CancellationToken`, `ProviderResult`, `MarkdownString`, `Uri`, `CompletionItemLabel` from stable API.

---

#### `src/vscode-dts/vscode.proposed.fileSearchProvider.d.ts`

- **Role:** Allows extensions to back the Quick Open file picker for custom URI schemes by registering a `FileSearchProvider`.

- **Key symbols:**
  - `workspace.registerFileSearchProvider(scheme, provider)` (line 71) — one provider per scheme.
  - `FileSearchProvider.provideFileSearchResults(query, options, token)` (line 58) — returns `Uri[]`.
  - `FileSearchQuery.pattern: string` (line 23) — raw Quick Open text; provider must apply its own scoring/filtering.
  - `FileSearchOptions extends SearchOptions` (line 29): `maxResults?`, `session?: CancellationToken` for cache keying.

- **Data flow:** Quick Open keystroke → `provideFileSearchResults()` called per scheme with current pattern → `Uri[]` merged with built-in results → displayed in picker.

- **Dependencies:** `Uri`, `CancellationToken`, `ProviderResult`, `SearchOptions` (from stable `workspace` types).

---

#### `src/vscode-dts/vscode.proposed.resolvers.d.ts`

- **Role:** Defines the complete remote-authority resolution ABI — including TCP/pipe tunnels, an `ExecServer` for spawning processes on remote machines, a `RemoteFileSystem`, and the `RemoteAuthorityResolver` interface — which is the most complex platform-integration contract any port must replicate.

- **Key symbols:**
  - `RemoteAuthorityResolver` (line 381): `resolve(authority, context)` → `ResolverResult` (a union of `ResolvedAuthority | ManagedResolvedAuthority` with options and tunnel info).
  - `ResolvedAuthority` (line 28): `host`, `port`, `connectionToken` — TCP handoff to the remote server.
  - `ManagedResolvedAuthority` (line 46): `makeConnection: () => Thenable<ManagedMessagePassing>` — for stream-based transports (WebSocket, named pipe).
  - `ManagedMessagePassing` (line 36): `onDidReceiveMessage: Event<Uint8Array>`, `send(data: Uint8Array)`, `end()`, `drain?()` — the raw byte-stream IPC abstraction.
  - `ExecServer` (line 161): `spawn()` (line 169), `spawnRemoteServerConnector?()` (line 180), `downloadCliExecutable?()` (line 191), `env()` (line 197), `kill()` (line 204), `tcpConnect()` (line 213), `fs: RemoteFileSystem` (line 222).
  - `RemoteFileSystem` (line 291): `stat`, `mkdirp`, `rm`, `read`, `write`, `connect`, `rename`, `readdir` — all returning `Thenable<...>` with `ReadStream`/`WriteStream` (line 255–263).
  - `RemoteServerConnector` (line 239): `connect(params: ServeParams)` → `Thenable<ManagedMessagePassing>` — bootstraps a server-side VS Code CLI.
  - `workspace.registerRemoteAuthorityResolver()` (line 456), `workspace.getRemoteExecServer()` (line 458).
  - `env.remoteAuthority: string | undefined` (line 472).

- **Control flow:** Editor detects `vscode-remote://` URI → calls `RemoteAuthorityResolver.resolve()` → gets `ResolverResult` → establishes either TCP connection (`ResolvedAuthority`) or managed stream (`ManagedResolvedAuthority`) → extension host communicates over that channel → extensions in remote host run against `RemoteFileSystem` for all I/O.

- **Data flow:** All data crosses the wire as `Uint8Array` streams (ReadStream / WriteStream). `ServeParams` (line 265) carries `socketId`, `commit`, `quality`, `extensions`, `compress`, `connectionToken`. `ExecEnvironment` (line 283) returns `env: ProcessEnv`, `osPlatform`, `osRelease`.

- **Dependencies:** `Uri`, `Event`, `Thenable`, `FileStat`, `FileType`, `AuthenticationSession` from stable API.

---

#### `src/vscode-dts/vscode.proposed.notebookKernelSource.d.ts`

- **Role:** Adds discovery and selection of notebook kernels (language runtimes) via a `NotebookKernelSourceActionProvider`, extending the `notebooks` namespace.

- **Key symbols:**
  - `notebooks.createNotebookControllerDetectionTask(notebookType)` (line 39) — signals ongoing kernel discovery.
  - `notebooks.registerKernelSourceActionProvider(notebookType, provider)` (line 44).
  - `NotebookKernelSourceActionProvider.provideNotebookKernelSourceActions(token)` (line 32) — returns `NotebookKernelSourceAction[]`.
  - `NotebookKernelSourceAction` (line 14): `label`, `description`, `detail`, `command: string | Command`, `documentation?: Uri`.

- **Dependencies:** `Command`, `Uri`, `CancellationToken`, `ProviderResult`, `Event` from stable API.

---

#### `src/vscode-dts/vscode.proposed.workspaceTrust.d.ts`

- **Role:** Adds workspace/resource trust gating — allowing extensions to check and request trust before performing sensitive file operations.

- **Key symbols:**
  - `workspace.onDidChangeWorkspaceTrustedFolders: Event<void>` (line 40).
  - `workspace.isResourceTrusted(resource)` (line 46) → `Thenable<boolean>`.
  - `workspace.requestResourceTrust(options)` (line 52) — shows trust dialog, returns `Thenable<boolean | undefined>`.
  - `workspace.requestWorkspaceTrust(options?)` (line 59).

- **Data flow:** Extension checks `isResourceTrusted(uri)` before reading; if false, calls `requestResourceTrust()` → user sees dialog → resolves to `true/false/undefined` → extension conditionally proceeds.

---

### Cross-Cutting Synthesis

The 174 files in `src/vscode-dts/` collectively define the **complete porting contract** for VS Code's IDE surface. The stable `vscode.d.ts` organises this into ~15 top-level namespaces. The porting challenge breaks into distinct layers:

1. **Editor kernel** (`window`, `TextDocument`, `TextEditor`, `TextEditorEdit`, Range/Position/Selection) — ~1,500 lines of type definitions mapping directly to the Monaco editor core. A Tauri/Rust port must either embed Monaco in a WebView or replace it entirely; all decoration, selection, and snippet types must be preserved.

2. **Language intelligence** (`namespace languages`) — 20+ provider registration points each return `ProviderResult<T>` (a Promise or direct value). These are the LSP feature points; a Tauri host must relay provider calls over an IPC bridge to extension host processes (Node.js or native Rust LSP clients).

3. **Debugger** (`namespace debug`) — built entirely on the Debug Adapter Protocol (`DebugProtocolMessage`, `DebugAdapter`, `DebugAdapterExecutable/Server/NamedPipeServer`). A Rust host can implement DAP relay natively; the `DebugVisualizationProvider` proposed API adds a tree-based custom variable viewer on top.

4. **Source control** (`namespace scm`, `scmHistoryProvider`) — `SourceControl` + `SourceControlResourceGroup` + the proposed `historyProvider` interface provide the git-graph, diff, and commit UI contracts.

5. **Terminal** (`window.createTerminal`, `TerminalCompletionProvider`) — terminals are spawned as child processes; the proposed completion provider adds shell-aware autocomplete on the command line.

6. **Remote transport** (`resolvers.d.ts`) — the most Rust-friendly layer: `ManagedMessagePassing` is a raw `Uint8Array` stream, `ExecServer` maps cleanly to Rust's `tokio::process::Command`, and `RemoteFileSystem` maps to `tokio::fs`. This is already the closest to a Rust ABI.

7. **AI/LM** (`chat`, `lm`, `chatProvider`) — streaming provider model with `Progress<LanguageModelResponsePart>`, tool-call round-trips, and MCP server spawning (stdio child process).

A Tauri/Rust port must either faithfully reimplement all `Disposable`-based registration, `Event<T>` pub/sub, `Thenable<T>` async, and `ProviderResult<T>` optional-async patterns in a Rust IPC layer, or maintain a Node.js extension host and expose these APIs over a message bus — the same architecture VS Code's remote development already uses.

---

### Out-of-Partition References

- `src/vs/workbench/api/common/extHost.api.impl.ts` — The TypeScript runtime implementation of every namespace (window, workspace, languages, debug, scm, etc.) declared in `vscode.d.ts`; this is where Disposable registrations, Event emitters, and ProviderResult wrappers are actually wired to core services.
- `src/vs/workbench/api/common/extHostLanguageFeatures.ts` — Implements all `languages.register*Provider()` calls and relays provider results to the language feature subsystem.
- `src/vs/workbench/api/common/extHostDebugService.ts` — Implements `namespace debug` including DAP session lifecycle and `DebugAdapter` proxy.
- `src/vs/workbench/api/common/extHostTerminalService.ts` — Implements `window.createTerminal`, terminal lifecycle events, and shell integration.
- `src/vs/workbench/api/common/extHostSCM.ts` — Implements `scm.createSourceControl()` and the `SourceControl`/`SourceControlResourceGroup` object graph.
- `src/vs/workbench/api/common/extHostFileSystem.ts` — Bridges `FileSystemProvider` registrations to the internal VFS.
- `src/vs/workbench/services/remote/common/remoteAuthorityResolverService.ts` — Hosts the `RemoteAuthorityResolver` call chain and manages connection token / tunnel lifecycle.
- `src/vs/platform/debug/common/debugProtocol.ts` — Defines the full typed DAP message schema that `DebugProtocolMessage`/`DebugProtocolSource`/`DebugProtocolBreakpoint` opaque types stand in for.
- `extensions/git/src/api/git.d.ts` — The concrete Git extension's own API surface, built on top of `scm` namespace and the `scmHistoryProvider` proposed API.
