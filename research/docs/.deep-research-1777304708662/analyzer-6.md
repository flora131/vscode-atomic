### Files Analysed

- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.d.ts` (21,233 LOC) — stable public API
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.terminalCompletionProvider.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.debugVisualization.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.scmHistoryProvider.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.languageModelCapabilities.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.languageModelSystem.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.languageModelThinkingPart.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.languageModelProxy.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.notebookExecution.d.ts`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.fileSearchProvider2.d.ts`

Total proposed API files in partition: 167 (169 total files - 2 non-`.d.ts`)

---

### Per-File Notes

#### `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.d.ts`

- **Role:** Single `declare module 'vscode'` block (~21 K lines) that is the entire stable public contract between the VS Code host and all extensions. Every namespace, class, interface, enum, and type alias used by extensions is declared here. It is consumed at extension compile time and injected by the host at runtime via the extension host IPC bridge.

- **Key symbols:**
  - `Uri` class (`vscode.d.ts:1439`) — five-component (scheme, authority, path, query, fragment) immutable value that is the universal identifier crossing every host/extension call. Factory methods `Uri.parse`, `Uri.file`, `Uri.joinPath`, `Uri.from` (`vscode.d.ts:1454–1531`).
  - `Position` class (`vscode.d.ts:269`) — zero-based `{line, character}` pair; character offsets in UTF-16 code units.
  - `Range` class (`vscode.d.ts:408`) — ordered `{start: Position, end: Position}` pair, immutable.
  - `Disposable` class (`vscode.d.ts:1712`) — RAII handle; `dispose()` unregisters a provider or listener.
  - `Event<T>` interface (`vscode.d.ts:1755`) — callable `(listener, thisArgs?, disposables?) => Disposable`; all host-to-extension notifications are modeled as typed events.
  - `CancellationToken` interface (`vscode.d.ts:1659`) — `isCancellationRequested: boolean` + `onCancellationRequested: Event<any>`; every async provider call receives one.
  - `ProviderResult<T>` type alias (`vscode.d.ts:2450`) — `T | undefined | null | Thenable<T | undefined | null>`; the universal return type for every extension-side provider method.
  - `TextDocument` interface (`vscode.d.ts:88`) — read-only model of an open document; fields include `uri: Uri`, `languageId: string`, `version: number`, `isDirty: boolean`, `encoding: string` (IANA charset string list at lines 130–133).
  - `FileSystem` interface (`vscode.d.ts:9774`) — `workspace.fs`; exposes `stat`, `readDirectory`, `createDirectory`, `readFile`, `writeFile`, `delete`, `rename`, `copy` (`vscode.d.ts:9782–9861`), all returning `Thenable<T>`.
  - `Extension<T>` interface (`vscode.d.ts:8333`) — reflection on an installed extension: `id`, `extensionUri: Uri`, `isActive`, `packageJSON: any`, `extensionKind: ExtensionKind`, `exports: T`, `activate(): Thenable<T>`.
  - `ExtensionContext` interface (`vscode.d.ts:8415`) — per-extension container of `subscriptions`, storage memento, secrets, extension URI, log output channel, etc.

- **Namespaces and their line numbers:**
  - `tasks` (`vscode.d.ts:9347`) — `registerTaskProvider`, `fetchTasks`, `executeTask`, `taskExecutions`, `onDidStartTask`, `onDidEndTask`, `onDidStartTaskProcess`, `onDidEndTaskProcess`.
  - `env` (`vscode.d.ts:10739`) — `appName`, `appRoot`, `appHost`, `uriScheme`, `language`, `clipboard`, `machineId`, `sessionId`, `isNewAppInstall`, `isAppPortable`, `isTelemetryEnabled`, `onDidChangeTelemetryEnabled`, `onDidChangeShell`, `createTelemetryLogger`.
  - `commands` (`vscode.d.ts:10973`) — `registerCommand`, `registerTextEditorCommand`, `executeCommand<T>`, `getCommands`.
  - `window` (`vscode.d.ts:11069`) — `tabGroups`, `activeTextEditor`, `visibleTextEditors`, `activeNotebookEditor`, `terminals`, `activeTerminal`, `state: WindowState`, `onDidChangeWindowState`, `showTextDocument`, `showNotebookDocument`, `createTextEditorDecorationType`, `showInformationMessage`, `showWarningMessage`, `showErrorMessage`, `showInputBox`, `showQuickPick`, `showOpenDialog`, `showSaveDialog`, `createStatusBarItem`, `createOutputChannel`, `createTerminal`, `registerTreeDataProvider`, `createTreeView`, `registerWebviewPanelSerializer`, `createWebviewPanel`, `withProgress`.
  - `workspace` (`vscode.d.ts:13797`) — `fs: FileSystem`, `workspaceFolders`, `workspaceFile`, `onDidChangeWorkspaceFolders`, `updateWorkspaceFolders`, `createFileSystemWatcher`, `findFiles`, `openTextDocument`, `registerTextDocumentContentProvider`, `applyEdit`, `saveAll`, `getConfiguration`, `registerFileSystemProvider`, `onDidOpenTextDocument`, `onDidCloseTextDocument`, `onDidChangeTextDocument`, `onDidSaveTextDocument`, `onDidCreateFiles`, `onDidDeleteFiles`, `onDidRenameFiles`.
  - `languages` (`vscode.d.ts:14722`) — `getLanguages`, `setTextDocumentLanguage`, `match`, `onDidChangeDiagnostics`, `getDiagnostics`, `createDiagnosticCollection`, `createLanguageStatusItem`, `registerCompletionItemProvider`, `registerInlineCompletionItemProvider`, `registerCodeActionsProvider`, `registerCodeLensProvider`, `registerDefinitionProvider`, `registerImplementationProvider`, `registerTypeDefinitionProvider`, `registerDeclarationProvider`, `registerHoverProvider`, `registerDocumentHighlightProvider`, `registerDocumentSymbolProvider`, `registerWorkspaceSymbolProvider`, `registerReferenceProvider`, `registerRenameProvider`, `registerDocumentFormattingEditProvider`, `registerDocumentRangeFormattingEditProvider`, `registerOnTypeFormattingEditProvider`, `registerSignatureHelpProvider`, `registerDocumentLinkProvider`, `registerColorProvider`, `registerFoldingRangeProvider`, `registerSelectionRangeProvider`, `registerCallHierarchyProvider`, `registerTypeHierarchyProvider`, `registerLinkedEditingRangeProvider`, `registerInlayHintsProvider`, `registerEvaluatableExpressionProvider`, `registerInlineValuesProvider`.
  - `notebooks` (`vscode.d.ts:16350`) — `createNotebookController`, `registerNotebookCellStatusBarItemProvider`, `createRendererMessaging`.
  - `scm` (`vscode.d.ts:16652`) — `inputBox` (deprecated), `createSourceControl`.
  - `debug` (`vscode.d.ts:17283`) — `activeDebugSession`, `activeDebugConsole`, `breakpoints`, `activeStackItem`, `onDidChangeActiveDebugSession`, `onDidStartDebugSession`, `onDidReceiveDebugSessionCustomEvent`, `onDidTerminateDebugSession`, `onDidChangeBreakpoints`, `onDidChangeActiveStackItem`, `registerDebugConfigurationProvider`, `registerDebugAdapterDescriptorFactory`, `registerDebugAdapterTrackerFactory`, `startDebugging`, `stopDebugging`, `addBreakpoints`, `removeBreakpoints`, `asDebugSourceUri`.
  - `extensions` (`vscode.d.ts:17458`) — `getExtension<T>`, `all: readonly Extension<any>[]`, `onDidChange`.
  - `authentication` (`vscode.d.ts:18091`) — `getSession`, `getAccounts`, `onDidChangeSessions`, `registerAuthenticationProvider`.
  - `l10n` (`vscode.d.ts:18192`) — `t(message, ...args)` localization function.
  - `tests` (`vscode.d.ts:18271`) — `createTestController`.
  - `chat` (`vscode.d.ts:20111`) — `createChatParticipant(id, handler): ChatParticipant`.
  - `lm` (`vscode.d.ts:20732`) — `onDidChangeChatModels`, `selectChatModels`, `registerTool`, `tools`, `invokeTool`, `registerMcpServerDefinitionProvider`, `registerLanguageModelChatProvider`.

- **Control flow:** Extensions call `activate(context: ExtensionContext)` at host initialization. All registrations return `Disposable` objects; the host disposes them when the extension deactivates. Events fire from host to extension over the IPC bridge. Provider calls flow host → extension as `Thenable`-returning callbacks, with results flowing back. `CancellationToken` travels with every provider invocation so the host can abort in-flight calls.

- **Data flow:**
  - Host serializes structured values (Uri, Position, Range, TextDocument metadata, etc.) across the extension host IPC boundary; extensions receive deserialized TypeScript class instances.
  - `Thenable<T>` return values from providers are awaited by the host; the extension host (Node.js process in Electron, or web worker in browser) resolves them and posts results back.
  - Events are fire-and-forget from the host; the `Event<T>` listener receives a single typed payload `T`.
  - `workspace.fs` methods pass `Uri` to the host which dispatches to the appropriate `FileSystemProvider` (registered by a different extension or the built-in FS), then returns `Uint8Array` / `FileStat` / `[string, FileType][]` back over IPC.

- **Dependencies:** Declares no external imports; this file is a pure ambient declaration. At runtime it depends on the VS Code host's extension host IPC protocol (JSON-RPC or binary protocol in `src/vs/workbench/services/extensions/`), and on the Node.js runtime provided by the Electron renderer/Node extension host.

---

#### `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.terminalCompletionProvider.d.ts`

- **Role:** Proposed API augmenting `window` namespace with `registerTerminalCompletionProvider` to allow extensions to supply shell-completion suggestions for terminal command lines. Extends the stable `vscode.d.ts` via a second `declare module 'vscode'` block.

- **Key symbols:**
  - `TerminalCompletionProvider<T>` interface (`terminalCompletionProvider.d.ts:23`) — single method `provideTerminalCompletions(terminal: Terminal, context: TerminalCompletionContext, token: CancellationToken): ProviderResult<T[] | TerminalCompletionList<T>>`.
  - `TerminalCompletionItem` class (`terminalCompletionProvider.d.ts:48`) — fields: `label: string | CompletionItemLabel`, `replacementRange: readonly [number, number]`, `detail?: string`, `documentation?: string | MarkdownString`, `kind?: TerminalCompletionItemKind`.
  - `TerminalCompletionItemKind` enum (`terminalCompletionProvider.d.ts:96`) — 17 values: `File=0`, `Folder=1`, `Method=2`, `Alias=3`, `Argument=4`, `Option=5`, `OptionValue=6`, `Flag=7`, `SymbolicLinkFile=8`, `SymbolicLinkFolder=9`, `ScmCommit=10`, `ScmBranch=11`, `ScmTag=12`, `ScmStash=13`, `ScmRemote=14`, `PullRequest=15`, `PullRequestDone=16`.
  - `TerminalCompletionContext` interface (`terminalCompletionProvider.d.ts:189`) — `commandLine: string`, `cursorIndex: number`.
  - `TerminalCompletionList<T>` class (`terminalCompletionProvider.d.ts:210`) — `items: T[]`, `resourceOptions?: TerminalCompletionResourceOptions`.
  - `TerminalCompletionResourceOptions` interface (`terminalCompletionProvider.d.ts:237`) — `showFiles: boolean`, `showDirectories: boolean`, `globPattern?: string`, `cwd: Uri`.
  - Registration: `window.registerTerminalCompletionProvider<T>(provider, ...triggerCharacters: string[]): Disposable` (`terminalCompletionProvider.d.ts:283`).

- **Control flow:** Host invokes `provideTerminalCompletions` on every keystroke in a terminal that matches the registered trigger characters. Returns either a flat array or a `TerminalCompletionList`; if `resourceOptions` is set, the host also injects file/folder entries from the specified `cwd` glob before showing the list.

- **Data flow:** Host → extension: `Terminal` object reference + `TerminalCompletionContext {commandLine, cursorIndex}` + `CancellationToken`. Extension → host: array of `TerminalCompletionItem` (label, `replacementRange [start, end]`, kind) or `TerminalCompletionList` carrying both items and `TerminalCompletionResourceOptions`. Host uses `replacementRange` as indices into `commandLine` to splice the accepted completion into the terminal input buffer.

- **Dependencies:** `Terminal` (from `window` namespace stable API), `CompletionItemLabel` (from `languages` namespace), `MarkdownString`, `Uri`, `CancellationToken`, `Disposable`, `ProviderResult` — all from stable `vscode.d.ts`.

---

#### `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.debugVisualization.d.ts`

- **Role:** Proposed API augmenting the `debug` namespace to allow extensions to register custom variable visualizers shown in the Debug Console / hover panels when a debug session is paused. Wires DAP (Debug Adapter Protocol) variable metadata to extension-provided tree views or commands.

- **Key symbols:**
  - `debug.registerDebugVisualizationProvider<T>(id: string, provider: DebugVisualizationProvider<T>): Disposable` (`debugVisualization.d.ts:15`).
  - `debug.registerDebugVisualizationTreeProvider<T>(id: string, provider: DebugVisualizationTree<T>): Disposable` (`debugVisualization.d.ts:25`).
  - `DebugVisualizationProvider<T>` interface (`debugVisualization.d.ts:119`) — `provideDebugVisualization(context, token): ProviderResult<T[]>`, optional `resolveDebugVisualization(visualization, token): ProviderResult<T>`.
  - `DebugVisualization` class (`debugVisualization.d.ts:94`) — `name: string`, `iconPath?`, `visualization?: Command | { treeId: string }`.
  - `DebugVisualizationContext` interface (`debugVisualization.d.ts:141`) — raw DAP `variable: any`, `containerId?: number`, `frameId?: number`, `threadId: number`, `session: DebugSession`.
  - `DebugTreeItem` interface (`debugVisualization.d.ts:34`) — `label`, `description?`, `collapsibleState?`, `contextValue?`, `canEdit?`.
  - `DebugVisualizationTree<T>` interface (`debugVisualization.d.ts:79`) — `getTreeItem(context): ProviderResult<T>`, `getChildren(element): ProviderResult<T[]>`, optional `editItem(item, value): ProviderResult<T>`.

- **Control flow:** Host evaluates the `when` clause from the extension's `package.json` `debugVisualizers` contribution for each variable when a DAP session stops. If satisfied, it calls `provideDebugVisualization(context, token)`. The returned `DebugVisualization` array enumerates available visualizers; when the user selects one, `resolveDebugVisualization` is called. If the visualization has `{ treeId }`, the host drives the registered `DebugVisualizationTree` for children and edits.

- **Data flow:** Host → extension: raw DAP `variable` payload (any JSON), DAP IDs (`containerId`, `frameId`, `threadId`), and the live `DebugSession` proxy. Extension → host: `DebugVisualization[]` objects containing either a `Command` (dispatched back to the host command registry) or a `treeId` reference to a registered tree provider. Tree item traversal (`getTreeItem`, `getChildren`) flows bidirectionally over IPC.

- **Dependencies:** `DebugSession`, `Command`, `TreeItemCollapsibleState`, `ThemeIcon`, `Uri`, `CancellationToken`, `Disposable`, `ProviderResult` — all from stable `vscode.d.ts`.

---

#### `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.scmHistoryProvider.d.ts`

- **Role:** Proposed API augmenting `SourceControl` (from stable `scm.createSourceControl`) with a `historyProvider` hook. Allows SCM extensions (e.g., Git) to expose commit history, branch/tag refs, file-level diff data, and AI chat context for individual history items.

- **Key symbols:**
  - `SourceControl.historyProvider?: SourceControlHistoryProvider` (`scmHistoryProvider.d.ts:10`).
  - `SourceControlHistoryProvider` interface (`scmHistoryProvider.d.ts:13`) — properties: `currentHistoryItemRef`, `currentHistoryItemRemoteRef`, `currentHistoryItemBaseRef` (all `SourceControlHistoryItemRef | undefined`), `onDidChangeCurrentHistoryItemRefs: Event<void>`, `onDidChangeHistoryItemRefs: Event<SourceControlHistoryItemRefsChangeEvent>`. Methods: `provideHistoryItemRefs(historyItemRefs: string[] | undefined, token): ProviderResult<SourceControlHistoryItemRef[]>`, `provideHistoryItems(options: SourceControlHistoryOptions, token): ProviderResult<SourceControlHistoryItem[]>`, `provideHistoryItemChanges(historyItemId, historyItemParentId | undefined, token): ProviderResult<SourceControlHistoryItemChange[]>`, `resolveHistoryItem(id, token): ProviderResult<SourceControlHistoryItem>`, `resolveHistoryItemChatContext(id, token): ProviderResult<string>`, `resolveHistoryItemChangeRangeChatContext(historyItemId, historyItemParentId, path, token): ProviderResult<string>`, `resolveHistoryItemRefsCommonAncestor(refs: string[], token): ProviderResult<string>`.
  - `SourceControlHistoryOptions` interface (`scmHistoryProvider.d.ts:39`) — `skip?: number`, `limit?: number | { id?: string }`, `historyItemRefs?: readonly string[]`, `filterText?: string`.
  - `SourceControlHistoryItem` interface (`scmHistoryProvider.d.ts:52`) — `id: string`, `parentIds: string[]`, `subject: string`, `message: string`, `displayId?`, `author?`, `authorEmail?`, `authorIcon?: IconPath`, `timestamp?: number`, `statistics?: SourceControlHistoryItemStatistics`, `references?: SourceControlHistoryItemRef[]`, `tooltip?: MarkdownString | MarkdownString[]`.
  - `SourceControlHistoryItemRef` interface (`scmHistoryProvider.d.ts:67`) — `id`, `name`, `description?`, `revision?`, `category?`, `icon?: IconPath`.
  - `SourceControlHistoryItemChange` interface (`scmHistoryProvider.d.ts:76`) — `uri: Uri`, `originalUri: Uri | undefined`, `modifiedUri: Uri | undefined`.
  - `SourceControlHistoryItemRefsChangeEvent` interface (`scmHistoryProvider.d.ts:82`) — `added`, `removed`, `modified` (all `readonly SourceControlHistoryItemRef[]`), `silent: boolean`.

- **Control flow:** Host calls `provideHistoryItems` to populate the timeline/history view; pagination is managed via `skip`/`limit` options. When the user expands a commit's diff, host calls `provideHistoryItemChanges`. AI chat context flows via `resolveHistoryItemChatContext` (whole commit) and `resolveHistoryItemChangeRangeChatContext` (per-file range). `onDidChangeHistoryItemRefs` pushes change events; `silent: true` suppresses automatic UI refresh.

- **Data flow:** Host → extension: raw string IDs (commit hashes / ref names), pagination options, file path strings. Extension → host: structured `SourceControlHistoryItem[]` (with author metadata, timestamps, statistics), `SourceControlHistoryItemRef[]` (branch/tag objects), `SourceControlHistoryItemChange[]` (before/after `Uri` pairs for diff computation), and plain `string` chat context snippets.

- **Dependencies:** `SourceControl` (from stable `scm.createSourceControl`), `Uri`, `MarkdownString`, `CancellationToken`, `Event`, `ProviderResult`, `IconPath` (from stable `vscode.d.ts`).

---

#### `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.languageModelCapabilities.d.ts`, `vscode.proposed.languageModelSystem.d.ts`, `vscode.proposed.languageModelThinkingPart.d.ts`, `vscode.proposed.languageModelProxy.d.ts`

- **Role:** Four small proposed APIs that extend the stable `lm`/`LanguageModelChat` surface in `vscode.d.ts`. Together they cover: model capability discovery, system-role messages, streaming reasoning ("thinking") tokens, and a proxy URI for exposing the local Copilot Chat model endpoint to non-VS Code consumers.

- **Key symbols:**
  - `LanguageModelChat.capabilities` (`languageModelCapabilities.d.ts:14`) — `{ supportsToolCalling: boolean, supportsImageToText: boolean, editToolsHint?: readonly string[] }`. Extends the stable `LanguageModelChat` interface.
  - `LanguageModelChatMessageRole.System = 3` (`languageModelSystem.d.ts:15`) — adds a third value to the stable `LanguageModelChatMessageRole` enum (stable has `User=1`, `Assistant=2`).
  - `LanguageModelThinkingPart` class (`languageModelThinkingPart.d.ts:15`) — `value: string | string[]`, `id?: string`, `metadata?: { [key: string]: any }`. Constructor `(value, id?, metadata?)`. Extended `LanguageModelChatResponse.stream` type includes this part (`languageModelThinkingPart.d.ts:47`).
  - `LanguageModelChatMessage2` class (`languageModelThinkingPart.d.ts:58`) — parallel to stable `LanguageModelChatMessage` but `content` array also allows `LanguageModelThinkingPart`. Static `User(...)` and `Assistant(...)` factory methods.
  - `LanguageModelToolResultPart2` and `LanguageModelToolResult2` (`languageModelThinkingPart.d.ts:105–110`) — temporary alias subclasses for backwards compatibility.
  - `LanguageModelProxy` interface (`languageModelProxy.d.ts:7`) — `Disposable` + `uri: Uri` + `key: string`. Represents a running local model proxy server.
  - `lm.isModelProxyAvailable: boolean`, `lm.onDidChangeModelProxyAvailability: Event<void>`, `lm.getModelProxy(): Thenable<LanguageModelProxy>` (`languageModelProxy.d.ts:13–29`).

- **Control flow (languageModelThinkingPart):** `LanguageModelChat.sendRequest` (stable, `vscode.d.ts:20297`) returns `LanguageModelChatResponse`. With this proposed API, the `response.stream` async iterable can now yield `LanguageModelThinkingPart` items interleaved with `LanguageModelTextPart` items before the final answer. Extensions iterate with `for await` and dispatch on instance type.

- **Data flow (languageModelProxy):** Host checks Copilot extension auth state. If `isModelProxyAvailable` is true, calling `getModelProxy()` starts an HTTP proxy server and returns `{ uri: Uri, key: string }`. The caller uses `uri` and `key` to make OpenAI-compatible requests directly to the local server, bypassing the normal `lm` IPC bridge.

- **Dependencies:** Stable `LanguageModelChat`, `LanguageModelChatResponse`, `LanguageModelChatMessageRole`, `LanguageModelChatMessage`, `LanguageModelTextPart`, `LanguageModelToolCallPart`, `LanguageModelDataPart`, `LanguageModelToolResultPart`, `LanguageModelToolResult`, `CancellationToken`, `Disposable`, `Event`, `Uri` (all from `vscode.d.ts`).

---

#### `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.notebookExecution.d.ts`

- **Role:** Tiny proposed API augmenting `NotebookController` (stable `vscode.d.ts:16041`) with `createNotebookExecution(notebook): NotebookExecution`. Allows a notebook controller to signal that it is "busy" at the notebook level — not tied to any individual cell — so the UI enters an executing state without a cell-level task.

- **Key symbols:**
  - `NotebookExecution` interface (`notebookExecution.d.ts:14`) — `start(): void`, `end(): void`.
  - `NotebookController.createNotebookExecution(notebook: NotebookDocument): NotebookExecution` (`notebookExecution.d.ts:37`). Throws if another `NotebookExecution` or `NotebookCellExecution` is already active for the notebook.

- **Control flow:** Extension calls `createNotebookExecution(notebook)`, then `execution.start()` to transition the notebook to executing state. When done, calls `execution.end()` to return to idle. Only one execution (cell or notebook level) may be active at a time.

- **Data flow:** Host → extension: `NotebookDocument` reference. Extension → host: `start()` and `end()` signals with no payload, triggering UI state transitions.

- **Dependencies:** `NotebookController` (augmented stable interface), `NotebookDocument` (stable `vscode.d.ts`).

---

#### `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.fileSearchProvider2.d.ts`

- **Role:** Proposed API augmenting `workspace` namespace with `registerFileSearchProvider2(scheme, provider)`. Allows extensions to supply file-name fuzzy-search results for custom URI schemes (e.g., virtual file systems, remote file trees) invoked from Quick Open.

- **Key symbols:**
  - `FileSearchProvider2` interface (`fileSearchProvider2.d.ts:76`) — single method `provideFileSearchResults(pattern: string, options: FileSearchProviderOptions, token: CancellationToken): ProviderResult<Uri[]>`.
  - `FileSearchProviderOptions` interface (`fileSearchProvider2.d.ts:13`) — `folderOptions: Array<{ folder: Uri, includes: string[], excludes: GlobPattern[], followSymlinks: boolean, useIgnoreFiles: { local, parent, global: boolean } }>`, `session: object` (WeakRef-safe cache key), `maxResults: number`.
  - `workspace.registerFileSearchProvider2(scheme: string, provider: FileSearchProvider2): Disposable` (`fileSearchProvider2.d.ts:102`).

- **Control flow:** On every Quick Open keystroke, for each workspace folder whose URI scheme matches the registered scheme, host calls `provideFileSearchResults(pattern, options, token)`. The provider should apply a relaxed (case-insensitive, subsequence) match and return up to `maxResults` URIs. The `session` object serves as a cache key; providers should use `WeakRef`/`WeakMap` to avoid memory leaks.

- **Data flow:** Host → extension: fuzzy `pattern` string, per-folder include/exclude glob lists, `maxResults`, and a `session` object. Extension → host: flat `Uri[]` of matching file paths. Host applies its own scoring/highlighting on returned URIs.

- **Dependencies:** `Uri`, `GlobPattern`, `CancellationToken`, `Disposable`, `ProviderResult` (all from stable `vscode.d.ts`).

---

### Cross-Cutting Synthesis

The `src/vscode-dts/` partition defines a **provider-pattern RPC contract** between the VS Code host and any extension. Every interaction follows one of three shapes:

1. **Registration** — extension calls `register*` or `create*` on a namespace, receiving a `Disposable`. The host stores the provider handle keyed by a string ID and extension identity. `Disposable.dispose()` deregisters it.
2. **Provider call** — host invokes a method on the registered provider interface, passing structured value types (`Uri`, `Position`, `Range`, context objects) and a `CancellationToken`. The provider returns `ProviderResult<T>` — either a synchronous value, `undefined`/`null`, or a `Thenable`.
3. **Event push** — host fires a typed `Event<T>` to all subscribers; subscribers are `(e: T) => any` functions attached via `event(listener, thisArgs?, disposables?)`.

For a Rust/Tauri host to reproduce this contract, it must implement:

- A **serialization layer** for all value types that cross the boundary: `Uri` (5-field struct), `Position` (line/character i32 pair, UTF-16 offsets), `Range`, `TextDocument` metadata, `FileStat`, `Diagnostic`, `DebugSession`, `NotebookDocument`, etc. All of these arrive as JSON in the current Electron IPC protocol.
- A **provider registry** per namespace: commands, languages, SCM, debug, notebooks, tasks, authentication, lm, tests, chat — each keyed by a string ID or `DocumentSelector`. The registry dispatches calls to the correct extension-host-side handler.
- An **async call protocol**: every provider method is async (Thenable-returning). The Tauri host must support sending a request with a correlation ID and resolving the response when it arrives, with cancellation propagated via a `CancellationToken` whose `isCancellationRequested` flip is delivered to the extension side.
- **Streaming support** for `LanguageModelChatResponse.stream` — an `AsyncIterable<LanguageModelTextPart | LanguageModelToolCallPart | LanguageModelDataPart | LanguageModelThinkingPart | unknown>`. The host must be able to push incremental parts to the awaiting extension code.
- **FileSystem dispatch**: `workspace.fs` methods must route through any registered `FileSystemProvider` for the relevant URI scheme before falling back to the native FS.
- **DAP proxy**: the `debug` namespace exposes raw DAP messages (`DebugProtocolMessage`, `DebugProtocolSource`, `DebugProtocolBreakpoint`) as opaque pass-through objects; the Tauri host must bridge these between extension and the actual debug adapter process.
- **Notebook execution state machine**: `NotebookCellExecution.start()` / `end()` and `NotebookExecution.start()` / `end()` must atomically flip cell/notebook UI state; the host enforces single-execution-at-a-time invariants.

The 167 proposed API files each augment one of these existing namespaces via additional `declare module 'vscode'` blocks; they do not introduce new IPC patterns, only extend the set of registered provider types and event payloads.

---

### Out-of-Partition References

The types declared in `src/vscode-dts/vscode.d.ts` are implemented in these other partitions:

- **Extension Host IPC / API adapter** — `src/vs/workbench/api/common/extHost*.ts` (e.g., `extHostCommands.ts`, `extHostLanguageFeatures.ts`, `extHostDebugService.ts`, `extHostNotebook.ts`, `extHostLanguageModels.ts`). These translate the TypeScript API surface into IPC message calls.
- **Main thread API stubs** — `src/vs/workbench/api/browser/mainThread*.ts` — receive IPC messages on the workbench side and dispatch to internal services.
- **IPC protocol definitions** — `src/vs/workbench/services/extensions/common/extHostProtocol.ts` — defines the `ProxyIdentifier`s and message schemas that correspond to every registered provider type.
- **Debug Adapter Protocol bridge** — `src/vs/workbench/contrib/debug/` — implements `DebugSession.customRequest`, `startDebugging`, `asDebugSourceUri`, and the breakpoint registry.
- **Notebook execution engine** — `src/vs/workbench/contrib/notebook/` — implements `NotebookController`, `NotebookCellExecution`, and the proposed `NotebookExecution` state machine.
- **Language Model / Copilot service** — `src/vs/workbench/contrib/chat/` and `src/vs/workbench/contrib/languageModels/` — back `lm.selectChatModels`, `lm.invokeTool`, `lm.registerLanguageModelChatProvider`, and the proxy feature.
- **SCM service** — `src/vs/workbench/contrib/scm/` — backs `scm.createSourceControl` and the `historyProvider` hook.
- **Terminal service** — `src/vs/workbench/contrib/terminal/` — backs `window.registerTerminalCompletionProvider` and the `TerminalCompletionContext` delivery.
- **File search service** — `src/vs/workbench/services/search/` — backs `workspace.registerFileSearchProvider2` and drives Quick Open file search.
- **Extension manifest / package.json schema** — `src/vs/workbench/services/extensions/common/extensionsRegistry.ts` — the `debugVisualizers`, `languageModelTools`, `mcpServerDefinitionProviders`, `languageModelChatProviders` contribution points referenced in the proposed API JSDoc are validated here.
