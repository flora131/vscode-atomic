# Partition 11 of 79 — Findings

## Scope
`extensions/vscode-api-tests/` (49 files, 11,374 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# VS Code API Tests Partition Locator

## Research Question
Locate files in `extensions/vscode-api-tests/` that represent behavioral contracts for core VS Code IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) which would be critical to port from TypeScript/Electron to Tauri/Rust.

## Scope Coverage
This partition contains **49 files, 11,374 LOC** distributed across:
- `src/singlefolder-tests/` — 37 test suites focusing on individual API subsystems
- `src/workspace-tests/` — Multi-folder workspace API coverage
- `testWorkspace/` and `testWorkspace2/` — Test fixtures and sample files
- Configuration files for test execution

---

## Core IDE Functionality Tests (by System)

### Text Editing & Editor Management
- `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts` (266 LOC) — Text insertion, snippets, selections, clipboard integration, cursor positioning
- `extensions/vscode-api-tests/src/singlefolder-tests/window.test.ts` (1063 LOC) — Active editor, text document management, view columns, tab management, status bar, message boxes, quick input
- `extensions/vscode-api-tests/src/singlefolder-tests/documentPaste.test.ts` (222 LOC) — Document paste event handlers and custom paste behaviors

### Language Services & IntelliSense
- `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts` (237 LOC) — Language detection, diagnostics collection, completion providers, hover info, code actions, symbol navigation
- `extensions/vscode-api-tests/src/singlefolder-tests/languagedetection.test.ts` (70 LOC) — Automatic language detection mechanisms

### Debugging Infrastructure
- `extensions/vscode-api-tests/src/singlefolder-tests/debug.test.ts` (156 LOC) — Breakpoints, debug sessions, debug adapter protocol (DAP), debug configuration, variables inspection

### Terminal & Process Management
- `extensions/vscode-api-tests/src/singlefolder-tests/terminal.test.ts` (985 LOC) — Terminal creation, process I/O, pseudo-terminals, shell integration, terminal state management
- `extensions/vscode-api-tests/src/singlefolder-tests/terminal.shellIntegration.test.ts` (252 LOC) — Shell integration features, command tracking, exit code handling
- `extensions/vscode-api-tests/src/singlefolder-tests/chat.runInTerminal.test.ts` (426 LOC) — Chat participant terminal execution, environment variable handling

### Workspace & File System
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.test.ts` (1500 LOC) — Workspace configuration, text documents, file system operations, glob patterns, file watchers, folder management
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.fs.test.ts` (263 LOC) — File system provider API, read/write operations, file metadata
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.event.test.ts` (260 LOC) — Workspace change events, document lifecycle
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.watcher.test.ts` (71 LOC) — File system watchers and change detection
- `extensions/vscode-api-tests/src/singlefolder-tests/readonlyFileSystem.test.ts` (63 LOC) — Read-only file system constraints

### Task Execution & Build Integration
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.tasks.test.ts` (451 LOC) — Task definition, execution, result handling, task groups

### Navigation & Symbols
- `extensions/vscode-api-tests/src/singlefolder-tests/tree.test.ts` (455 LOC) — Tree view data provider, symbol hierarchy, tree refresh operations
- `extensions/vscode-api-tests/src/singlefolder-tests/quickInput.test.ts` (389 LOC) — Quick pick, input box, navigation UI

### Source Control Integration
- `extensions/vscode-api-tests/src/singlefolder-tests/commands.test.ts` (167 LOC) — Command execution and extension command handling

### Notebooks & Interactive Computing
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.api.test.ts` (357 LOC) — Notebook serialization, kernel protocol, cell management
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.document.test.ts` (372 LOC) — Notebook document model, cell type support, metadata handling
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.kernel.test.ts` (469 LOC) — Kernel execution, cell output, interruption, restart
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.editor.test.ts` (130 LOC) — Notebook editor bindings, cell editing
- `extensions/vscode-api-tests/src/singlefolder-tests/ipynb.test.ts` (53 LOC) — Jupyter notebook format support
- `extensions/vscode-api-tests/src/singlefolder-tests/interactiveWindow.test.ts` (142 LOC) — Interactive window API (REPLs, notebooks)

### AI & Chat Capabilities
- `extensions/vscode-api-tests/src/singlefolder-tests/chat.test.ts` (245 LOC) — Chat participant registration, message handling, tool integration
- `extensions/vscode-api-tests/src/singlefolder-tests/lm.test.ts` (216 LOC) — Language model API, token counting, embeddings, tool references

### Environment & System Integration
- `extensions/vscode-api-tests/src/singlefolder-tests/env.test.ts` (80 LOC) — Environment variables, clipboard access, UI kind detection
- `extensions/vscode-api-tests/src/singlefolder-tests/env.power.test.ts` (81 LOC) — Battery power state detection

### Multi-Folder Workspace Support
- `extensions/vscode-api-tests/src/workspace-tests/workspace.test.ts` (38 LOC) — Multi-folder workspace configuration tests
- `extensions/vscode-api-tests/src/workspace-tests/index.ts` (42 LOC) — Multi-folder test harness

### Extension System & RPC
- `extensions/vscode-api-tests/src/singlefolder-tests/extensions.test.ts` (26 LOC) — Extension loading and management
- `extensions/vscode-api-tests/src/singlefolder-tests/rpc.test.ts` (126 LOC) — Extension host RPC protocol, proxy verification
- `extensions/vscode-api-tests/src/singlefolder-tests/proxy.test.ts` (263 LOC) — Language model proxy, request forwarding

### Browser/Web Support
- `extensions/vscode-api-tests/src/singlefolder-tests/browser.test.ts` (217 LOC) — Web API compatibility, platform-specific behaviors
- `extensions/vscode-api-tests/src/singlefolder-tests/browser.tools.test.ts` (147 LOC) — Browser developer tools integration
- `extensions/vscode-api-tests/src/singlefolder-tests/browser.cdp.test.ts` (297 LOC) — Chrome DevTools Protocol integration

### Configuration & State
- `extensions/vscode-api-tests/src/singlefolder-tests/configuration.test.ts` (49 LOC) — User and workspace settings, configuration change events
- `extensions/vscode-api-tests/src/singlefolder-tests/state.test.ts` (80 LOC) — Extension global and workspace state persistence
- `extensions/vscode-api-tests/src/singlefolder-tests/types.test.ts` (31 LOC) — VS Code type system contracts

---

## Implementation & Infrastructure

- `extensions/vscode-api-tests/src/extension.ts` (15 LOC) — Test extension activation, context initialization
- `extensions/vscode-api-tests/src/utils.ts` (256 LOC) — Test utilities: file creation, editor management, RPC validation, logging helpers, pseudo-terminal support
- `extensions/vscode-api-tests/src/memfs.ts` (242 LOC) — Memory-based file system provider implementation for isolated testing

---

## Tests By Category

### Large Test Suites (400+ LOC)
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.test.ts` (1500 LOC) — Comprehensive workspace API
- `extensions/vscode-api-tests/src/singlefolder-tests/window.test.ts` (1063 LOC) — Comprehensive UI/window management
- `extensions/vscode-api-tests/src/singlefolder-tests/terminal.test.ts` (985 LOC) — Comprehensive terminal API
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.kernel.test.ts` (469 LOC) — Notebook kernel contract
- `extensions/vscode-api-tests/src/singlefolder-tests/tree.test.ts` (455 LOC) — Tree view provider contract
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.tasks.test.ts` (451 LOC) — Task system contract
- `extensions/vscode-api-tests/src/singlefolder-tests/chat.runInTerminal.test.ts` (426 LOC) — Chat terminal integration
- `extensions/vscode-api-tests/src/singlefolder-tests/quickInput.test.ts` (389 LOC) — Quick pick/input UI contract

### Medium Test Suites (200-399 LOC)
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.document.test.ts` (372 LOC)
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.api.test.ts` (357 LOC)
- `extensions/vscode-api-tests/src/singlefolder-tests/browser.cdp.test.ts` (297 LOC)
- `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts` (266 LOC)
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.fs.test.ts` (263 LOC)
- `extensions/vscode-api-tests/src/singlefolder-tests/proxy.test.ts` (263 LOC)
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.event.test.ts` (260 LOC)
- `extensions/vscode-api-tests/src/singlefolder-tests/terminal.shellIntegration.test.ts` (252 LOC)
- `extensions/vscode-api-tests/src/singlefolder-tests/chat.test.ts` (245 LOC)
- `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts` (237 LOC)
- `extensions/vscode-api-tests/src/singlefolder-tests/documentPaste.test.ts` (222 LOC)
- `extensions/vscode-api-tests/src/singlefolder-tests/browser.test.ts` (217 LOC)
- `extensions/vscode-api-tests/src/singlefolder-tests/lm.test.ts` (216 LOC)

---

## Configuration & Test Harness

- `extensions/vscode-api-tests/package.json` — Extension manifest with 61 enabled API proposals, chat participant definitions, language model definitions, notebook types, task definitions, debugger configs
- `extensions/vscode-api-tests/tsconfig.json` — TypeScript configuration, includes VS Code type definitions
- `extensions/vscode-api-tests/.vscode/launch.json` — Launch configuration for extensionHost test runner
- `extensions/vscode-api-tests/.vscode/tasks.json` — Test build and execution tasks

---

## Test Fixtures & Workspaces

### Test Workspace
- `extensions/vscode-api-tests/testWorkspace/` — Primary test workspace containing:
  - Sample source files: `far.js`, `10linefile.ts`, `30linefile.ts`, `myFile.ts`, `worker.js`, `debug.js`
  - Configuration: `.vscode/settings.json`, `.vscode/launch.json`
  - Test data: `bower.json`, `index.html`, `lorem.txt`, `simple.txt`, `test.ipynb`
  - Images and binary test data: `image.png`, `image%.png`, `image%02.png`, `sub/image.png`
  - File system filter tests: `files-exclude/file.txt`, `search-exclude/file.txt`

### Secondary Test Workspace
- `extensions/vscode-api-tests/testWorkspace2/` — Secondary workspace for multi-folder tests
  - Configuration: `.vscode/settings.json`
  - Test file: `simple.txt`

---

## Notable Test Patterns

**Total test functions: ~1,479 test cases** distributed across 39 test files (based on `test(`, `assert.`, `expect(` pattern counts)

### API Coverage Clusters

1. **Workspace/File System** (1,500 LOC in main workspace.test.ts + supporting files)
   - Document lifecycle, file I/O, glob patterns, watchers, multi-folder support

2. **Editor/UI** (1,063 LOC in window.test.ts + 266 LOC editor.test.ts + 389 LOC quickInput.test.ts)
   - Text editing, view management, UI widgets, status bar, dialogs

3. **Terminal Integration** (985 LOC terminal.test.ts + 252 LOC shell integration + 426 LOC chat terminal)
   - Terminal I/O, pseudo-terminals, shell commands, environment variables

4. **Notebook/Interactive** (357 + 372 + 469 + 130 + 142 = 1,470 LOC across notebook and interactive tests)
   - Notebook document model, kernel protocol, cell execution, output rendering

5. **Debug/Execution** (156 LOC debug.test.ts + 451 LOC tasks.test.ts)
   - Debug protocol, breakpoints, task execution

6. **Language & Navigation** (237 LOC languages.test.ts + 455 LOC tree.test.ts + 389 LOC quickInput.test.ts)
   - Language detection, diagnostics, symbol navigation, semantic features

---

## Summary

This partition contains the definitive behavioral specification for VS Code's extension API across 40+ core subsystems. The **37 test suites in singlefolder-tests/** define the precise contracts for:

- **Text editing and IDE fundamentals** (1,500+ LOC)
- **Terminal and process integration** (1,660+ LOC across 3 files)
- **Workspace and file system** (2,200+ LOC across 5 files)
- **Notebook/interactive computing** (1,470+ LOC across 5 files)
- **UI, dialogs, and navigation** (1,700+ LOC across 4 files)
- **Language services, symbols, diagnostics** (700+ LOC across 3 files)
- **Debug protocol and breakpoints** (156 LOC)
- **Chat, AI models, and tools** (460+ LOC across 3 files)
- **Browser/web compatibility** (660+ LOC across 3 files)

Each test is a concrete, executable assertion about API behavior—any Tauri/Rust port must satisfy all 1,479+ test cases to maintain feature parity.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Core IDE Functionality: API Test Patterns

Research Question: What would it take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust?

This document catalogs concrete API test patterns from `extensions/vscode-api-tests/` that define the behavioral contracts that a Tauri/Rust host would need to satisfy.

---

## Pattern 1: Text Editor Manipulation & Snippet Insertion

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts:37-51`

**What:** Basic snippet insertion with cursor/selection support—foundational editing capability required for any IDE.

```typescript
test('insert snippet', () => {
	const snippetString = new SnippetString()
		.appendText('This is a ')
		.appendTabstop()
		.appendPlaceholder('placeholder')
		.appendText(' snippet');

	return withRandomFileEditor('', (editor, doc) => {
		return editor.insertSnippet(snippetString).then(inserted => {
			assert.ok(inserted);
			assert.strictEqual(doc.getText(), 'This is a placeholder snippet');
			assert.ok(doc.isDirty);
		});
	});
});
```

**Variations / call-sites:**
- `editor.test.ts:79-95` — Snippet with selection replacement
- `editor.test.ts:110-126` — Snippet with indentation preservation
- `editor.test.ts:128-144` — Snippet with explicit selection argument

---

## Pattern 2: Document Edit Operations (Batch Edits)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts:146-156`

**What:** Atomic, batch text editor operations via `editor.edit()` callback—core for incremental modifications.

```typescript
test('make edit', () => {
	return withRandomFileEditor('', (editor, doc) => {
		return editor.edit((builder) => {
			builder.insert(new Position(0, 0), 'Hello World');
		}).then(applied => {
			assert.ok(applied);
			assert.strictEqual(doc.getText(), 'Hello World');
			assert.ok(doc.isDirty);
		});
	});
});
```

**Variations / call-sites:**
- `editor.test.ts:158-168` — Range replacement with `Number.MAX_VALUE` bounds
- `editor.test.ts:198-215` — Overlapping edit detection (must reject)

---

## Pattern 3: Editor Options & Configuration Management

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts:170-196`

**What:** Runtime editor option querying and modification (tab size, insert spaces, cursor style, line numbers).

```typescript
test('issue #16573: Extension API: insertSpaces and tabSize are undefined', () => {
	return withRandomFileEditor('Hello world!\n\tHello world!', (editor, _doc) => {
		assert.strictEqual(editor.options.tabSize, 4);
		assert.strictEqual(editor.options.insertSpaces, false);
		assert.strictEqual(editor.options.cursorStyle, TextEditorCursorStyle.Line);
		assert.strictEqual(editor.options.lineNumbers, TextEditorLineNumbersStyle.On);

		editor.options = { tabSize: 2 };
		assert.strictEqual(editor.options.tabSize, 2);

		return Promise.resolve();
	});
});
```

---

## Pattern 4: Debugging API: Breakpoints & Debug Sessions

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/debug.test.ts:28-53`

**What:** Breakpoint lifecycle management and tracking via event emitters and state queries.

```typescript
test('breakpoints', async function () {
	assert.strictEqual(debug.breakpoints.length, 0);
	let onDidChangeBreakpointsCounter = 0;
	const toDispose: Disposable[] = [];

	toDispose.push(debug.onDidChangeBreakpoints(() => {
		onDidChangeBreakpointsCounter++;
	}));

	debug.addBreakpoints([{ id: '1', enabled: true }, { id: '2', enabled: false, condition: '2 < 5' }]);
	assert.strictEqual(onDidChangeBreakpointsCounter, 1);
	assert.strictEqual(debug.breakpoints.length, 2);
	assert.strictEqual(debug.breakpoints[0].id, '1');
	assert.strictEqual(debug.breakpoints[1].condition, '2 < 5');

	debug.removeBreakpoints([{ id: '1', enabled: true }]);
	assert.strictEqual(onDidChangeBreakpointsCounter, 2);
	assert.strictEqual(debug.breakpoints.length, 1);

	disposeAll(toDispose);
});
```

**Variations / call-sites:**
- `debug.test.ts:55-64` — Function breakpoints with hit conditions and log messages

---

## Pattern 5: Language Services: Diagnostics & Code Actions

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts:76-97`

**What:** Diagnostic collection creation, registration, and querying across document URIs.

```typescript
test('diagnostics, read & event', function () {
	const uri = vscode.Uri.file('/foo/bar.txt');
	const col1 = vscode.languages.createDiagnosticCollection('foo1');
	col1.set(uri, [new vscode.Diagnostic(new vscode.Range(0, 0, 0, 12), 'error1')]);

	const col2 = vscode.languages.createDiagnosticCollection('foo2');
	col2.set(uri, [new vscode.Diagnostic(new vscode.Range(0, 0, 0, 12), 'error1')]);

	const diag = vscode.languages.getDiagnostics(uri);
	assert.strictEqual(diag.length, 2);

	const tuples = vscode.languages.getDiagnostics();
	let found = false;
	for (const [thisUri,] of tuples) {
		if (thisUri.toString() === uri.toString()) {
			found = true;
			break;
		}
	}
	assert.ok(tuples.length >= 1);
	assert.ok(found);
});
```

---

## Pattern 6: Code Actions Provider Registration & Execution

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts:126-170`

**What:** Registration of code action providers filtered by document patterns, and synchronous invocation by URI and range.

```typescript
test('diagnostics & CodeActionProvider', async function () {
	class D2 extends vscode.Diagnostic {
		customProp = { complex() { } };
		constructor() {
			super(new vscode.Range(0, 2, 0, 7), 'sonntag');
		}
	}

	const diag1 = new vscode.Diagnostic(new vscode.Range(0, 0, 0, 5), 'montag');
	const diag2 = new D2();

	let ran = false;
	const uri = vscode.Uri.parse('ttt:path.far');

	const r1 = vscode.languages.registerCodeActionsProvider({ pattern: '*.far', scheme: 'ttt' }, {
		provideCodeActions(_document, _range, ctx): vscode.Command[] {
			assert.strictEqual(ctx.diagnostics.length, 2);
			const [first, second] = ctx.diagnostics;
			assert.ok(first === diag1);
			assert.ok(second === diag2);
			ran = true;
			return [];
		}
	});

	const r2 = vscode.workspace.registerTextDocumentContentProvider('ttt', {
		provideTextDocumentContent() {
			return 'this is some text';
		}
	});

	const r3 = vscode.languages.createDiagnosticCollection();
	r3.set(uri, [diag1]);

	const r4 = vscode.languages.createDiagnosticCollection();
	r4.set(uri, [diag2]);

	await vscode.workspace.openTextDocument(uri);
	await vscode.commands.executeCommand('vscode.executeCodeActionProvider', uri, new vscode.Range(0, 0, 0, 10));
	assert.ok(ran);
	vscode.Disposable.from(r1, r2, r3, r4).dispose();
});
```

**Variations / call-sites:**
- `languages.test.ts:101-124` — Document link provider registration and execution

---

## Pattern 7: Completion Item Provider (IntelliSense)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts:172-192`

**What:** Multi-document-filter completion provider registration and execution via command protocol.

```typescript
test('completions with document filters', async function () {
	let ran = false;
	const uri = vscode.Uri.file(join(vscode.workspace.rootPath || '', './bower.json'));

	const jsonDocumentFilter = [
		{ language: 'json', pattern: '**/package.json' },
		{ language: 'json', pattern: '**/bower.json' },
		{ language: 'json', pattern: '**/.bower.json' }
	];

	const r1 = vscode.languages.registerCompletionItemProvider(jsonDocumentFilter, {
		provideCompletionItems: (_document: vscode.TextDocument, _position: vscode.Position, _token: vscode.CancellationToken): vscode.CompletionItem[] => {
			const proposal = new vscode.CompletionItem('foo');
			proposal.kind = vscode.CompletionItemKind.Property;
			ran = true;
			return [proposal];
		}
	});

	await vscode.workspace.openTextDocument(uri);
	const result = await vscode.commands.executeCommand<vscode.CompletionList>('vscode.executeCompletionItemProvider', uri, new vscode.Position(1, 0));
	r1.dispose();
	assert.ok(ran, 'Provider has not been invoked');
	assert.ok(result!.items.some(i => i.label === 'foo'), 'Results do not include "foo"');
});
```

---

## Pattern 8: Language Document State Changes (onDidCloseTextDocument, onDidOpenTextDocument)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts:29-62`

**What:** Document lifecycle event ordering when changing language via `setTextDocumentLanguage`.

```typescript
test('setTextDocumentLanguage -> close/open event', async function () {
	const file = await createRandomFile('foo\nbar\nbar');
	const doc = await vscode.workspace.openTextDocument(file);
	const langIdNow = doc.languageId;
	let clock = 0;
	const disposables: vscode.Disposable[] = [];

	const close = new Promise<void>(resolve => {
		disposables.push(vscode.workspace.onDidCloseTextDocument(e => {
			if (e === doc) {
				assert.strictEqual(doc.languageId, langIdNow);
				assert.strictEqual(clock, 0);
				clock += 1;
				resolve();
			}
		}));
	});
	const open = new Promise<void>(resolve => {
		disposables.push(vscode.workspace.onDidOpenTextDocument(e => {
			if (e === doc) {
				assert.strictEqual(doc.languageId, 'json');
				assert.strictEqual(clock, 1);
				clock += 1;
				resolve();
			}
		}));
	});
	const change = vscode.languages.setTextDocumentLanguage(doc, 'json');
	await Promise.all([change, close, open]);
	assert.strictEqual(clock, 2);
	assert.strictEqual(doc.languageId, 'json');
	disposables.forEach(disposable => disposable.dispose());
});
```

---

## Pattern 9: Terminal Creation & Output Events

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/terminal.test.ts:43-62`

**What:** Terminal lifecycle management: creation, event subscription, text sending, and disposal.

```typescript
test('sendText immediately after createTerminal should not throw', async () => {
	const terminal = window.createTerminal();
	const result = await new Promise<Terminal>(r => {
		disposables.push(window.onDidOpenTerminal(t => {
			if (t === terminal) {
				r(t);
			}
		}));
	});
	equal(result, terminal);
	doesNotThrow(terminal.sendText.bind(terminal, 'echo "foo"'));
	await new Promise<void>(r => {
		disposables.push(window.onDidCloseTerminal(t => {
			if (t === terminal) {
				r();
			}
		}));
		terminal.dispose();
	});
});
```

**Variations / call-sites:**
- `terminal.test.ts:64-103` — Echo output data capture via `onDidWriteTerminalData` event
- `terminal.test.ts:105-123` — Terminal close event firing on disposal

---

## Pattern 10: Shell Integration (Terminal Execution Events)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/terminal.shellIntegration.test.ts:83-107`

**What:** Shell integration activation and command execution event sequencing (start, output, end).

```typescript
test('window.onDidChangeTerminalShellIntegration should activate for the default terminal', async () => {
	const terminal = await new Promise<Terminal>(r => {
		disposables.push(window.onDidOpenTerminal(t => {
			if (t === terminal) {
				r(terminal);
			}
		}));
		const terminal = window.createTerminal({
			env: { TEST: '`' }
		});
		terminal.show();
	});

	let data = '';
	await new Promise<void>(r => {
		disposables.push(window.onDidWriteTerminalData(e => {
			if (e.terminal === terminal) {
				data += e.data;
				if (data.indexOf('`') !== 0) {
					r();
				}
			}
		}));
		if (process.platform === 'win32') {
			terminal.sendText(`$env:TEST`);
		} else {
			terminal.sendText(`echo $TEST`);
		}
	});

	await new Promise<void>(r => {
		terminal.dispose();
		disposables.push(window.onDidCloseTerminal(t => {
			strictEqual(terminal, t);
			r();
		}));
	});
});
```

**Variations / call-sites:**
- `terminal.shellIntegration.test.ts:121-143` — Exit code reporting in execution events
- `terminal.shellIntegration.test.ts:167-180` — Command output iteration via `TerminalShellExecution.read()`

---

## Pattern 11: Workspace File System (stat, read, write, delete)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/workspace.fs.test.ts:21-40`

**What:** File system operations: stat (metadata), readDirectory, type checking (File vs Directory).

```typescript
test('fs.stat', async function () {
	const stat = await vscode.workspace.fs.stat(root);
	assert.strictEqual(stat.type, vscode.FileType.Directory);

	assert.strictEqual(typeof stat.size, 'number');
	assert.strictEqual(typeof stat.mtime, 'number');
	assert.strictEqual(typeof stat.ctime, 'number');

	assert.ok(stat.mtime > 0);
	assert.ok(stat.ctime > 0);

	const entries = await vscode.workspace.fs.readDirectory(root);
	assert.ok(entries.length > 0);

	const tuple = entries.find(tuple => tuple[0] === 'far.js')!;
	assert.ok(tuple);
	assert.strictEqual(tuple[0], 'far.js');
	assert.strictEqual(tuple[1], vscode.FileType.File);
});
```

**Variations / call-sites:**
- `workspace.fs.test.ts:60-79` — Write, stat, read, delete workflow
- `workspace.fs.test.ts:81-99` — Recursive delete with non-empty folder protection
- `workspace.fs.test.ts:220-250` — Recursive directory creation

---

## Pattern 12: Workspace Events (onWillCreateFiles, onDidCreateFiles, onWillDeleteFiles, onDidDeleteFiles)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/workspace.event.test.ts:20-44`

**What:** Workspace-level file mutation events with event waiting (promise-based lifecycle).

```typescript
test('onWillCreate/onDidCreate', withLogDisabled(async function () {
	const base = await createRandomFile();
	const newUri = base.with({ path: base.path + '-foo' });

	let onWillCreate: vscode.FileWillCreateEvent | undefined;
	let onDidCreate: vscode.FileCreateEvent | undefined;

	disposables.push(vscode.workspace.onWillCreateFiles(e => onWillCreate = e));
	disposables.push(vscode.workspace.onDidCreateFiles(e => onDidCreate = e));

	const edit = new vscode.WorkspaceEdit();
	edit.createFile(newUri);

	const success = await vscode.workspace.applyEdit(edit);
	assert.ok(success);

	assert.ok(onWillCreate);
	assert.strictEqual(onWillCreate?.files.length, 1);
	assert.strictEqual(onWillCreate?.files[0].toString(), newUri.toString());

	assert.ok(onDidCreate);
	assert.strictEqual(onDidCreate?.files.length, 1);
	assert.strictEqual(onDidCreate?.files[0].toString(), newUri.toString());
}));
```

**Variations / call-sites:**
- `workspace.event.test.ts:46-66` — Event interception with cross-document edit mutations
- `workspace.event.test.ts:90-104` — Delete event lifecycle

---

## Pattern 13: Tree Views (Data Providers & Item Rendering)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/tree.test.ts:21-106`

**What:** TreeDataProvider implementation with async element fetching, state management, and reveal/refresh semantics.

```typescript
test('TreeView - element already registered', async function () {
	this.timeout(60_000);

	type TreeElement = { readonly kind: 'leaf' };

	class QuickRefreshTreeDataProvider implements vscode.TreeDataProvider<TreeElement> {
		private readonly changeEmitter = new vscode.EventEmitter<TreeElement | undefined>();
		private readonly requestEmitter = new vscode.EventEmitter<number>();
		private readonly pendingRequests: DeferredPromise<TreeElement[]>[] = [];
		private readonly element: TreeElement = { kind: 'leaf' };

		readonly onDidChangeTreeData = this.changeEmitter.event;

		getChildren(element?: TreeElement): Thenable<TreeElement[]> {
			if (!element) {
				const deferred = new DeferredPromise<TreeElement[]>();
				this.pendingRequests.push(deferred);
				this.requestEmitter.fire(this.pendingRequests.length);
				return deferred.p;
			}
			return Promise.resolve([]);
		}

		getTreeItem(): vscode.TreeItem {
			const item = new vscode.TreeItem('duplicate', vscode.TreeItemCollapsibleState.None);
			item.id = 'dup';
			return item;
		}

		getParent(): TreeElement | undefined {
			return undefined;
		}

		async waitForRequestCount(count: number): Promise<void> {
			while (this.pendingRequests.length < count) {
				await asPromise(this.requestEmitter.event);
			}
		}

		async resolveNextRequest(): Promise<void> {
			const next = this.pendingRequests.shift();
			if (!next) {
				return;
			}
			await next.complete([this.element]);
		}
	}

	const provider = new QuickRefreshTreeDataProvider();
	disposables.push(provider);

	const treeView = vscode.window.createTreeView('test.treeId', { treeDataProvider: provider });
	disposables.push(treeView);

	const revealFirst = (treeView.reveal(provider.getElement(), { expand: true })
		.then(() => ({ error: undefined as Error | undefined })) as Promise<{ error: Error | undefined }>)
		.catch(error => ({ error }));
	const revealSecond = (treeView.reveal(provider.getElement(), { expand: true })
		.then(() => ({ error: undefined as Error | undefined })) as Promise<{ error: Error | undefined }>)
		.catch(error => ({ error }));

	await provider.waitForRequestCount(2);
	await provider.resolveNextRequest();
	await delay(0);
	await provider.resolveNextRequest();

	const [firstResult, secondResult] = await Promise.all([revealFirst, revealSecond]);
	const errors = [firstResult.error, secondResult.error].filter((e): e is Error => !!e);
	assert.strictEqual(errors.length, 1, 'Exactly one reveal should fail from the stale fetch');
	assert.ok(/Cannot resolve tree item/.test(errors[0].message));
});
```

---

## Pattern 14: Window / Editor Group Management

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/window.test.ts:19-34`

**What:** Active editor tracking and document opening via window API.

```typescript
test('editor, active text editor', async () => {
	const doc = await workspace.openTextDocument(join(workspace.rootPath || '', './far.js'));
	await window.showTextDocument(doc);
	const active = window.activeTextEditor;
	assert.ok(active);
	assert.ok(pathEquals(active!.document.uri.fsPath, doc.uri.fsPath));
});
```

**Variations / call-sites:**
- `window.test.ts:27-34` — Document opening via resource URI
- `window.test.ts:41-53` — Editor view column assignment (One, Two, Three)
- `window.test.ts:55-72` — `onDidChangeVisibleTextEditors` event tracking

---

## Pattern 15: Notebook API (Kernel Registration & Cell Execution)

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/notebook.api.test.ts:39-77`

**What:** Notebook kernel controller registration with cell execution handler and output rendering.

```typescript
export class Kernel {
	readonly controller: vscode.NotebookController;
	readonly associatedNotebooks = new Set<string>();

	constructor(id: string, label: string, viewType: string = notebookType) {
		this.controller = vscode.notebooks.createNotebookController(id, viewType, label);
		this.controller.executeHandler = this._execute.bind(this);
		this.controller.supportsExecutionOrder = true;
		this.controller.supportedLanguages = ['typescript', 'javascript'];
		this.controller.onDidChangeSelectedNotebooks(e => {
			if (e.selected) {
				this.associatedNotebooks.add(e.notebook.uri.toString());
			} else {
				this.associatedNotebooks.delete(e.notebook.uri.toString());
			}
		});
	}

	protected async _execute(cells: vscode.NotebookCell[]): Promise<void> {
		for (const cell of cells) {
			await this._runCell(cell);
		}
	}

	protected async _runCell(cell: vscode.NotebookCell) {
		const task = this.controller.createNotebookCellExecution(cell);
		task.start(Date.now());
		task.executionOrder = 1;
		await sleep(10);
		await task.replaceOutput([new vscode.NotebookCellOutput([
			vscode.NotebookCellOutputItem.text(cell.document.getText() || cell.document.uri.toString(), 'text/plain')
		])]);
		task.end(true);
	}
}
```

---

## Pattern 16: Command Registration & Execution

**Where:** `extensions/vscode-api-tests/src/singlefolder-tests/commands.test.ts:48-60`

**What:** Command registration with variable arguments and filtering (getCommands with includeAllEndpointsCommands flag).

```typescript
test('command with args', async function () {
	let args: IArguments;
	const registration = commands.registerCommand('t1', function () {
		args = arguments;
	});

	await commands.executeCommand('t1', 'start');
	registration.dispose();
	assert.ok(args!);
	assert.strictEqual(args!.length, 1);
	assert.strictEqual(args![0], 'start');
});
```

**Variations / call-sites:**
- `commands.test.ts:19-46` — getCommands() with include-internal flag
- `commands.test.ts:62-80` — Text editor commands with injected editor + extra arguments

---

## Pattern 17: Test Harness Utilities (File Creation, Editor Cleanup)

**Where:** `extensions/vscode-api-tests/src/utils.ts:19-29`

**What:** Memory file system for test isolation; deterministic file creation and cleanup.

```typescript
export async function createRandomFile(contents: string | Uint8Array = '', dir: vscode.Uri | undefined = undefined, ext = ''): Promise<vscode.Uri> {
	let fakeFile: vscode.Uri;
	if (dir) {
		assert.strictEqual(dir.scheme, testFs.scheme);
		fakeFile = dir.with({ path: dir.path + '/' + rndName() + ext });
	} else {
		fakeFile = vscode.Uri.parse(`${testFs.scheme}:/${rndName() + ext}`);
	}
	testFs.writeFile(fakeFile, typeof contents === 'string' ? Buffer.from(contents) : Buffer.from(contents), { create: true, overwrite: true });
	return fakeFile;
}
```

**Variations / call-sites:**
- `utils.ts:49-51` — closeAllEditors() command
- `utils.ts:53-55` — saveAllEditors() command
- `utils.ts:57-59` — revertAllDirty() internal command

---

## Summary

The VS Code API test suite defines **17 core patterns** spanning:

1. **Text Editing**: Snippet insertion, batch edits, range operations, editor options
2. **Debugging**: Breakpoint lifecycle, debug sessions, condition/log management
3. **Language Services**: Diagnostics, code actions, completions, document links, folding
4. **Terminal**: Creation, I/O events, shell integration, execution tracking
5. **File System**: stat, read, write, delete, recursive operations, watchers
6. **Workspace Events**: File mutations (create/delete/rename) with event interception
7. **UI Components**: Tree views, editor groups, window state, tab management
8. **Notebooks**: Kernel controllers, cell execution, output rendering
9. **Commands**: Registration, execution, filtering, argument passing

A Tauri/Rust host would need to implement **async event-driven APIs** mirroring these patterns, with particular emphasis on:
- Promise/Promise-like futures for all async operations
- Event emitters for lifecycle tracking (onDidChange, onDidCreate, onDidDelete, etc.)
- Type-safe provider registration by document filter
- Atomic multi-edit transactions
- Precise string/buffer boundary handling in file operations

All patterns use mocha test structure (`suite()`, `test()`, `teardown()`) and assert library for contract validation.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
