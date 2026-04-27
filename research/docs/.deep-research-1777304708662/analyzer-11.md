### Files Analysed

- `extensions/vscode-api-tests/src/extension.ts`
- `extensions/vscode-api-tests/src/utils.ts`
- `extensions/vscode-api-tests/src/memfs.ts`
- `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts`
- `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts`
- `extensions/vscode-api-tests/src/singlefolder-tests/debug.test.ts`
- `extensions/vscode-api-tests/src/singlefolder-tests/terminal.test.ts`
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.tasks.test.ts`
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.test.ts` (first 800 lines)
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.kernel.test.ts` (first 200 lines)

---

### Per-File Notes

#### `extensions/vscode-api-tests/src/extension.ts`

- **Role:** Minimal test-extension entry point that stores the `ExtensionContext` in a global so other test suites can access extension-lifetime APIs (e.g., `environmentVariableCollection`).
- **Key symbols:** `activate` (`extension.ts:12`) — receives `vscode.ExtensionContext`, assigns it to `global.testExtensionContext`.
- **Control flow:** Called once by VS Code's extension host when the test extension activates. Downstream tests in `terminal.test.ts:17` call `extensions.getExtension('vscode.vscode-api-tests')?.activate()` in `suiteSetup` to guarantee the context is available before any test runs.
- **Data flow:** `_context: vscode.ExtensionContext` → `global.testExtensionContext` (`extension.ts:14`). The context carries `environmentVariableCollection`, subscription lists, and extension URI — all things an extension host must provide.
- **Dependencies:** `vscode` module only.

---

#### `extensions/vscode-api-tests/src/utils.ts`

- **Role:** Shared test utilities that encapsulate file creation, editor management, RPC leak detection, async polling, and deferred-promise primitives used by every test suite.
- **Key symbols:**
  - `testFs` (`utils.ts:16`) — a singleton `TestFS` instance registered as a `FileSystemProvider` under the scheme `fake-fs` via `vscode.workspace.registerFileSystemProvider` (`utils.ts:17`).
  - `createRandomFile` (`utils.ts:19`) — writes a file into `testFs` and returns its `vscode.Uri`.
  - `assertNoRpc` (`utils.ts:90`) — walks the entire `vscode` namespace graph checking for `Symbol.for('rpcProxy')` and `Symbol.for('rpcProtocol')` markers, failing if any live RPC objects remain after a test (detects proxy leaks across the extension-host IPC boundary).
  - `asPromise` (`utils.ts:135`) — converts a `vscode.Event<T>` to a `Promise<T>` with a configurable timeout (5 s desktop / 15 s web).
  - `poll` (`utils.ts:164`) — retries an async function up to 200 times at 100 ms intervals, used to await eventual consistency in UI state.
  - `DeferredPromise<T>` (`utils.ts:201`) — imperative promise control (`.complete()`, `.error()`, `.cancel()`).
  - `closeAllEditors` (`utils.ts:49`) — triggers `workbench.action.closeAllEditors` command.
  - `withLogDisabled` / `withVerboseLogs` (`utils.ts:82`, `utils.ts:86`) — toggle log level via internal commands `_extensionTests.getLogLevel` / `_extensionTests.setLogLevel`.
- **Control flow:** `assertNoRpc` performs a deep object walk (`walk` at `utils.ts:102`); it traverses every enumerable key of the `vscode` namespace recursively to look for IPC proxy residue. The `poll` loop at `utils.ts:174` uses `setTimeout`-based retry.
- **Data flow:** File bytes flow into `testFs` in-memory; URIs are returned to callers. `assertNoRpc` reads internal symbols from VS Code proxy objects and accumulates path strings into `proxyPaths`/`rpcPaths` arrays for assertion.
- **Dependencies:** `assert`, `os` (EOL), `crypto` (random names), `vscode`, `./memfs`.

---

#### `extensions/vscode-api-tests/src/memfs.ts`

- **Role:** A complete in-memory `vscode.FileSystemProvider` implementation used to run all file-system tests without touching the real OS filesystem.
- **Key symbols:**
  - `TestFS` (`memfs.ts:51`) — implements `vscode.FileSystemProvider`; holds a `Directory` root (`memfs.ts:58`).
  - `stat` (`memfs.ts:62`), `readDirectory` (`memfs.ts:66`), `readFile` (`memfs.ts:77`), `writeFile` (`memfs.ts:85`), `rename` (`memfs.ts:112`), `delete` (`memfs.ts:134`), `createDirectory` (`memfs.ts:147`) — the full `FileSystemProvider` surface required by VS Code.
  - `onDidChangeFile` (`memfs.ts:223`) — event emitter for file change notifications, batched via a 5 ms `setTimeout` in `_fireSoon` (`memfs.ts:230`).
  - `_lookup` (`memfs.ts:161`) — recursive path segment resolution supporting both case-sensitive and case-insensitive modes.
- **Control flow:** `writeFile` checks whether the parent directory exists (`_lookupParentDirectory`), then either creates or overwrites a `File` entry; fires `Created` or `Changed` change events via `_fireSoon`. `_fireSoon` buffers events and flushes them after 5 ms to coalesce rapid writes (`memfs.ts:237`).
- **Data flow:** File content as `Uint8Array` enters via `writeFile`, stored in `File.data`; re-emitted as-is via `readFile`. Change events are `vscode.FileChangeEvent[]` emitted on `_emitter` (`memfs.ts:219`).
- **Dependencies:** `path` (posix operations), `vscode`.

---

#### `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts`

- **Role:** Contract tests for the `vscode.TextEditor` API covering snippet insertion, range-based edits, editor options, and multi-cursor selection semantics.
- **Key symbols:**
  - `withRandomFileEditor` (`editor.test.ts:17`) — helper that creates a temp file, opens it as a document, shows it in an editor, runs a callback, then saves/deletes the file.
  - Tests: `insert snippet` (`editor.test.ts:37`), `make edit` (`editor.test.ts:146`), `issue #6281 range clamp` (`editor.test.ts:158`), `tabSize/insertSpaces/cursorStyle/lineNumbers` (`editor.test.ts:170`), `overlapping ranges fail` (`editor.test.ts:198`).
- **Control flow:** Each test calls `createRandomFile` → `workspace.openTextDocument` → `window.showTextDocument` → exercises `editor.edit()` or `editor.insertSnippet()` with a builder callback, then asserts `doc.getText()` matches the expected result.
- **Data flow:**
  - In: initial file content as string.
  - Out: mutated `TextDocument.getText()` string verified by `assert.strictEqual`.
  - `editor.edit(builder => builder.insert/replace/setEndOfLine)` returns a `Thenable<boolean>` indicating whether the edit was applied.
  - `editor.options` is a live mutable object; setting `tabSize` to an invalid value is silently ignored (`editor.test.ts:187`).
- **Dependencies:** `vscode` (Position, Range, Selection, SnippetString, TextEditorCursorStyle, TextEditorLineNumbersStyle, env, window, workspace), `../utils`.

---

#### `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts`

- **Role:** Contract tests for language services APIs: language ID assignment, diagnostic collections, document link providers, code action providers, completion item providers, and folding range providers.
- **Key symbols:**
  - `setTextDocumentLanguage` test (`languages.test.ts:29`) — verifies that changing a document's language triggers `onDidCloseTextDocument` then `onDidOpenTextDocument` on the same doc instance (`clock` counter enforces ordering).
  - `diagnostics, read & event` (`languages.test.ts:76`) — creates two `DiagnosticCollection` instances for the same URI and asserts `getDiagnostics(uri).length === 2`.
  - `link detector` (`languages.test.ts:101`) — registers a `DocumentLinkProvider` then invokes `vscode.executeLinkProvider` command and asserts two links are returned (one custom, one URL-detected).
  - `diagnostics & CodeActionProvider` (`languages.test.ts:126`) — verifies that `Diagnostic` subclasses (`D2 extends vscode.Diagnostic`) round-trip identity through the extension-host RPC layer.
  - `completions with document filters` (`languages.test.ts:172`) — registers a `CompletionItemProvider` with JSON-specific document filters and executes `vscode.executeCompletionItemProvider`.
  - `folding command` (`languages.test.ts:194`) — activates `vscode.json-language-features` extension and validates `vscode.executeFoldingRangeProvider` output changes when `editor.foldingStrategy` configuration is toggled.
- **Control flow:** Each provider is registered via `vscode.languages.register*`, then an execute-command is issued to trigger the provider through VS Code's internal language feature dispatch. Assertions check the returned data.
- **Data flow:** Provider callbacks receive `TextDocument` + `Position`/`Range`/`CancellationToken` and return typed results (`CompletionItem[]`, `DocumentLink[]`, `FoldingRange[]`). These cross the extension-host → workbench RPC boundary and return as deserialized objects.
- **Dependencies:** `vscode`, `path`, `../utils`.

---

#### `extensions/vscode-api-tests/src/singlefolder-tests/debug.test.ts`

- **Role:** Contract tests for the Debug API: breakpoint management, `FunctionBreakpoint` construction, debug session lifecycle, and Debug Adapter Protocol (DAP) message tracing.
- **Key symbols:**
  - `debug.breakpoints` array and `debug.onDidChangeBreakpoints` event (`debug.test.ts:29-53`) — tested for add/remove semantics with counter assertions.
  - `FunctionBreakpoint` construction (`debug.test.ts:55-64`) — verifies `condition`, `hitCondition`, `logMessage`, `enabled`, and `functionName` fields survive construction.
  - `debug.registerDebugAdapterTrackerFactory` (`debug.test.ts:84`) — installs a tracker that observes raw DAP messages (`onDidSendMessage`), counting `stopped` events and watching for `initialized` / `configurationDone` / `variables` response messages.
  - `debug.startDebugging` (`debug.test.ts:105`) — initiates a named launch config and awaits an active session.
  - Step commands: `workbench.action.debug.stepOver`, `stepInto`, `stepOut` executed via `commands.executeCommand` (`debug.test.ts:115-135`).
- **Control flow:** The start-debugging test (currently skipped as flaky) follows: start session → wait for `initialized` event → wait for `configurationDone` → wait for first `variables` response → then step three times asserting `stoppedEvents` increments, and that `window.activeTextEditor.document.fileName` ends in `debug.js`.
- **Data flow:** DAP messages flow through `onDidSendMessage` as plain objects with `type`, `event`, `command` properties. Breakpoints are plain value objects held in `debug.breakpoints` array. Debug sessions appear on `debug.activeDebugSession`.
- **Dependencies:** `vscode` (commands, debug, Disposable, FunctionBreakpoint, window, workspace), `../utils`.

---

#### `extensions/vscode-api-tests/src/singlefolder-tests/terminal.test.ts`

- **Role:** Contract tests for the integrated terminal API: `window.createTerminal`, `Pseudoterminal` (extension-owned PTY), environment variable collections, terminal state, exit codes, and data events.
- **Key symbols:**
  - `window.createTerminal` / `window.onDidOpenTerminal` / `window.onDidCloseTerminal` (`terminal.test.ts:44-62`) — baseline lifecycle contract.
  - `window.onDidWriteTerminalData` (`terminal.test.ts:78`) — streams raw terminal output bytes; test verifies echo of `$TEST` env var.
  - `terminal.processId` (`terminal.test.ts:135`) — async property returning the shell process PID; test asserts `pid > 0`.
  - `TerminalExitReason` (`terminal.test.ts:226`) — enum verified on `exitStatus.reason` when terminal is disposed by extension vs. by process.
  - `window.onDidChangeTerminalState` (`terminal.test.ts:234`) — fires when `isInteractedWith` transitions to true after `terminal.sendText`.
  - `Pseudoterminal` (`terminal.test.ts:499`) — interface with `onDidWrite: Event<string>`, `open()`, `close()` required for extension-owned terminals; tests verify `onDidChangeName`, `onDidClose` with exit codes.
  - `extensionContext.environmentVariableCollection` (`terminal.test.ts:742`) — tests `replace`, `append`, `prepend` mutations and verify they affect the spawned shell's environment via `echo` output polling.
  - `EnvironmentVariableMutatorType` enum (`terminal.test.ts:927`) — Replace/Append/Prepend variants tested with `get` and `forEach`.
  - `collection.getScoped(scope)` (`terminal.test.ts:940`) — per-workspace-folder scoping of env var mutations.
- **Control flow:** Suite-level `suiteSetup` disables shell integration, GPU acceleration, exit alerts, and local echo via `workspace.getConfiguration('terminal.integrated').update()`. Each test uses a disposables array cleared in `teardown`. Tests use `poll()` to wait for terminal output rather than fixed delays.
- **Data flow:** `terminal.sendText(str)` → shell process → `window.onDidWriteTerminalData` emits `{terminal, data: string}`. For extension PTYs, `writeEmitter.fire(str)` in the extension drives `onDidWriteTerminalData`. Environment mutations flow via `environmentVariableCollection` → terminal spawn environment → shell variable expansion.
- **Dependencies:** `vscode` (extensive named imports from `vscode`), `../utils` (assertNoRpc, poll).

---

#### `extensions/vscode-api-tests/src/singlefolder-tests/workspace.tasks.test.ts`

- **Role:** Contract tests for the Tasks API: `ShellExecution`, `CustomExecution`, task provider registration, task fetch, execution lifecycle events, and `Pseudoterminal`-backed custom tasks.
- **Key symbols:**
  - `tasks.registerTaskProvider(type, provider)` (`workspace.tasks.test.ts:176`) — provider returns `Task[]` from `provideTasks`, each with a `CustomExecution` that resolves to a `Pseudoterminal`.
  - `tasks.executeTask(task)` (`workspace.tasks.test.ts:74`) — returns a `TaskExecution`; identity equality used to match `onDidStartTaskProcess` / `onDidEndTaskProcess` events.
  - `tasks.fetchTasks({ type })` (`workspace.tasks.test.ts:347`) — retrieves tasks matching a type; verifies task group `isDefault` property from `tasks.json` config.
  - `ShellExecution` (`workspace.tasks.test.ts:57`) — built with command string `'echo'` and args `['hello test']`; tests `onDidStartTaskProcess.processId`.
  - `CustomExecution` (`workspace.tasks.test.ts:184`) — lambda returning `Pseudoterminal`; `open()` fires `writeEmitter`; `close()` sets `isPseudoterminalClosed`; tests assert strict ordering: TerminalOpened → TerminalWritten → TerminalClosed.
  - Back-to-back task execution (`workspace.tasks.test.ts:386`) — runs 8 concurrent `CustomExecution` tasks and asserts exit code identity via `onDidClose` event.
- **Control flow:** `commands.executeCommand('workbench.action.tasks.runTask', ...)` triggers the task runner. The extension-provided `Pseudoterminal` is invoked by the workbench after the task start event fires. Disposables on `window.onDidOpenTerminal`, `window.onDidWriteTerminalData`, `window.onDidCloseTerminal` chain the ordering assertions.
- **Data flow:** `CustomExecution` callback → `Pseudoterminal` instance → `writeEmitter.fire(string)` → `window.onDidWriteTerminalData`. Process-based tasks emit `onDidStartTaskProcess` with a `processId: number`.
- **Dependencies:** `vscode` (commands, ConfigurationTarget, CustomExecution, Pseudoterminal, ShellExecution, Task, TaskDefinition, TaskProcessStartEvent, tasks, TaskScope, Terminal, UIKind, window, workspace), `../utils`.

---

#### `extensions/vscode-api-tests/src/singlefolder-tests/workspace.test.ts` (sampled through line 800)

- **Role:** Comprehensive contract tests for the `vscode.workspace` API: text document lifecycle, content providers, EOL handling, workspace edits, file search, event sequencing, and the `FileSystemProvider` interface.
- **Key symbols:**
  - `workspace.openTextDocument(uri)` (`workspace.test.ts:73`) — tested with real files, scheme-based virtual files, untitled docs, and with language/content options.
  - `workspace.registerTextDocumentContentProvider(scheme, provider)` (`workspace.test.ts:372`) — provider pattern; multiple registrations for the same scheme are both consulted; an erroring provider does not prevent a succeeding one.
  - `workspace.applyEdit(WorkspaceEdit)` (`workspace.test.ts:674`) — atomic multi-file edit; tests cover insert+rename, rename+insert, delete+insert (should fail), overlapping renames.
  - `workspace.findFiles` / `workspace.findFiles2` (`workspace.test.ts:561-641`) — glob-based file search with exclude patterns and cancellation tokens.
  - `workspace.findTextInFiles` (`workspace.test.ts:644`) — full-text search returning `TextSearchResult[]`.
  - `workspace.onWillSaveTextDocument` (`workspace.test.ts:274`) — hook that can inject additional `TextEdit[]` via `e.waitUntil()` before save commits.
  - `workspace.onDidOpenTextDocument`, `onDidChangeTextDocument`, `onDidSaveTextDocument` (`workspace.test.ts:299`) — event ordering asserted via `Set<TextDocument>`.
  - Case sensitivity in `registerFileSystemProvider` (`workspace.test.ts:189`) — `TestFS` used with `isCaseSensitive: false`; verifies that the first-opened casing wins for document identity.
- **Control flow:** `openTextDocument` with `untitled:` scheme produces a dirty document immediately. Saving the untitled file changes its scheme to `file:` and fires `onDidCloseTextDocument` + `onDidOpenTextDocument`. `applyEdit` is atomic: all operations must succeed or the whole edit is rolled back.
- **Data flow:** `WorkspaceEdit` accumulates `TextEdit[]` per URI and file-level operations (createFile, deleteFile, renameFile); `applyEdit` returns `Promise<boolean>`. `TextSearchResult` has a `preview.text` field and `uri`.
- **Dependencies:** `vscode`, `fs` (for untitled→file save verification), `path`, `../memfs` (TestFS), `../utils`.

---

#### `extensions/vscode-api-tests/src/singlefolder-tests/notebook.kernel.test.ts` (first 200 lines)

- **Role:** Contract tests for the Notebook Kernel API: controller creation, cell execution, output replacement, kernel selection, and notebook serializer registration.
- **Key symbols:**
  - `Kernel` class (`notebook.kernel.test.ts:38`) — wraps `vscode.notebooks.createNotebookController(id, viewType, label)` (`line 45`); sets `executeHandler`, `supportsExecutionOrder`, `supportedLanguages`; tracks associated notebooks via `onDidChangeSelectedNotebooks` event.
  - `Kernel._runCell` (`notebook.kernel.test.ts:64`) — creates a `NotebookCellExecution` via `controller.createNotebookCellExecution(cell)`, calls `task.start(Date.now())`, sets `task.executionOrder = 1`, calls `task.replaceOutput([new NotebookCellOutput([...])])`, then `task.end(true)`.
  - `apiTestSerializer` (`notebook.kernel.test.ts:88`) — implements `vscode.NotebookSerializer`; `deserializeNotebook` returns a hardcoded `NotebookData` with two TypeScript cells including pre-set outputs and metadata; `serializeNotebook` returns empty `Uint8Array`.
  - `assertKernel` (`notebook.kernel.test.ts:79`) — invokes `notebook.selectKernel` command with extension+id pair and asserts the kernel's `associatedNotebooks` set includes the notebook URI.
  - Suite setup: `vscode.workspace.registerNotebookSerializer('notebookCoreTest', apiTestSerializer)` (`notebook.kernel.test.ts:143`).
- **Control flow:** Tests open a random `.vsctestnb` file → `workspace.openNotebookDocument` → `window.showNotebookDocument` → `commands.executeCommand('notebook.execute')` → awaits `workspace.onDidChangeNotebookDocument` event → asserts `cell.outputs.length`.
- **Data flow:** Notebook file bytes → `NotebookSerializer.deserializeNotebook` → `NotebookData` → cell `Document` objects. Execution: `NotebookCell[]` → `_execute` → `NotebookCellExecution.replaceOutput` → `NotebookCellOutput` pushed to cell's output array → `onDidChangeNotebookDocument` event.
- **Dependencies:** `vscode`, `mocha`, `util` (TextDecoder), `../utils`.

---

### Cross-Cutting Synthesis

These files collectively define the full observable API contract that VS Code exposes to extensions, and therefore constitute the complete porting surface that any Tauri/Rust reimplementation must replicate or replace.

The `vscode` module (imported everywhere) is the RPC facade: every call from an extension crosses an IPC boundary to the workbench process. `utils.ts:assertNoRpc` (`utils.ts:90`) enforces that no RPC proxy objects leak between tests, revealing that every `vscode.*` object is an IPC proxy under the hood.

The APIs under test decompose into six distinct subsystems, each requiring its own porting strategy:

1. **File system** (`memfs.ts`, `workspace.test.ts`) — the `FileSystemProvider` interface (stat/read/write/rename/delete/watch/createDirectory) is the abstraction layer. In Tauri, this would map to Rust `tauri::fs` or direct OS calls exposed to the webview.
2. **Text editing** (`editor.test.ts`, `workspace.test.ts`) — `WorkspaceEdit`, `TextEdit`, `SnippetString`, `TextEditorOptions`, and `Position`/`Range`/`Selection` must all be re-implemented. The editor model lives in the workbench; extensions see proxy objects.
3. **Language services** (`languages.test.ts`) — all language intelligence flows through `vscode.languages.register*` + execute-command dispatch. Each provider type (completion, diagnostics, code actions, folding, links) would need a corresponding Rust/webview protocol.
4. **Debugging** (`debug.test.ts`) — the DAP adaptor layer (`registerDebugAdapterTrackerFactory`, `startDebugging`, breakpoints) sits on top of a separate DAP protocol implementation entirely independent of Electron.
5. **Terminal** (`terminal.test.ts`) — `Pseudoterminal` and `EnvironmentVariableCollection` require PTY process management and environment-injection hooks; `conpty` on Windows is referenced by the test infrastructure comments.
6. **Tasks & Notebooks** (`workspace.tasks.test.ts`, `notebook.kernel.test.ts`) — both use `CustomExecution`/`Pseudoterminal` to drive output, making them dependent on the same PTY infrastructure as terminals plus serializer registration.

The uniform pattern across all subsystems is: extension host ↔ IPC ↔ workbench renderer. Porting to Tauri would require replacing this IPC with Tauri's `invoke`/`emit` channel and reimplementing the workbench-side state machines in Rust or a Tauri-hosted webview.

---

### Out-of-Partition References

- `src/vs/workbench/api/common/extHost.api.impl.ts` — The actual TypeScript implementation of the `vscode` module namespace that extensions import; defines every `workspace.*`, `window.*`, `languages.*`, `debug.*`, `tasks.*`, `terminal.*` surface tested here.
- `src/vs/workbench/api/common/extHostTerminalService.ts` — Implements the extension-host side of terminal creation, `Pseudoterminal` management, environment variable collection, and `onDidWriteTerminalData` event plumbing.
- `src/vs/workbench/api/common/extHostDebugService.ts` — Implements `debug.startDebugging`, `debug.addBreakpoints`, `registerDebugAdapterTrackerFactory`, and DAP message routing.
- `src/vs/workbench/api/common/extHostLanguageFeatures.ts` — Implements the registration adapters for all `vscode.languages.register*` providers; serializes provider results across IPC.
- `src/vs/workbench/api/common/extHostTask.ts` — Implements `tasks.registerTaskProvider`, `tasks.executeTask`, `tasks.fetchTasks`, and the CustomExecution ↔ Pseudoterminal bridge.
- `src/vs/workbench/api/common/extHostNotebook.ts` — Implements `vscode.notebooks.createNotebookController`, `NotebookCellExecution`, and `NotebookSerializer` registration.
- `src/vs/workbench/api/common/extHostFileSystem.ts` — Implements `workspace.registerFileSystemProvider`, routing `FileSystemProvider` method calls across IPC between extension host and workbench.
- `src/vs/workbench/api/common/extHostWorkspace.ts` — Implements `workspace.applyEdit`, `workspace.findFiles`, `workspace.findTextInFiles`, text document open/close events, and `WorkspaceEdit` serialization.
