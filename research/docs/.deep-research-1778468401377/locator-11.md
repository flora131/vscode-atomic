# Partition 11: vscode-api-tests — VS Code API Conformance Suite

## Summary

The `extensions/vscode-api-tests/` partition contains the gold conformance test suite for the VS Code Extension API. This 50-file, ~11.4K LOC test extension runs mocha-based integration tests against the vscode module, covering all major IDE features. For a Tauri/Rust port, this test suite represents the executable specification of what MUST be re-implemented to achieve feature parity.

---

## Implementation

- `extensions/vscode-api-tests/src/extension.ts` — Entry point; activates and exposes ExtensionContext globally for tests
- `extensions/vscode-api-tests/src/utils.ts` — Shared test utilities: file operations, event assertions, disposables, RPC validation
- `extensions/vscode-api-tests/src/memfs.ts` — In-memory file system provider implementing vscode.FileSystemProvider for test isolation
- `extensions/vscode-api-tests/src/singlefolder-tests/index.ts` — Test runner configuration for single-folder tests (mocha, reporters, timeouts, multi-environment support)
- `extensions/vscode-api-tests/src/workspace-tests/index.ts` — Test runner configuration for workspace-level tests (multi-folder scenarios)

---

## Tests

### Core Editor & Workspace Tests (36 test files, ~10.7K LOC)

#### Editing & Text Manipulation
- `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts` — Text editor operations: insertSnippet, edit, selection, decoration, column management
- `extensions/vscode-api-tests/src/singlefolder-tests/commands.test.ts` — Command registration and execution; text editor commands with implicit args
- `extensions/vscode-api-tests/src/singlefolder-tests/documentPaste.test.ts` — Document paste provider registration and event handling

#### Workspace & File System
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.test.ts` — MarkdownString, textDocuments, rootPath, workspaceFolders, getWorkspaceFolder, openTextDocument, workspace events
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.fs.test.ts` — File system operations: stat, read, write, delete, copy, rename via workspace.fs API
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.event.test.ts` — Workspace event listeners: onDidCreateFiles, onDidRenameFiles, onDidDeleteFiles, onWillRenameFiles
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.watcher.test.ts` — File watchers with glob patterns and change detection
- `extensions/vscode-api-tests/src/singlefolder-tests/readonlyFileSystem.test.ts` — Read-only file system provider implementation and constraints

#### Language & Diagnostics
- `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts` — Language detection, setTextDocumentLanguage, diagnostic collections, completion, hover, signature help, symbol providers
- `extensions/vscode-api-tests/src/singlefolder-tests/languagedetection.test.ts` — Language detection API with confidence scoring

#### Terminal & Shell Integration
- `extensions/vscode-api-tests/src/singlefolder-tests/terminal.test.ts` — Terminal creation, sendText, dimension changes, exit reasons, pseudoterminals, environment variables (24 tests)
- `extensions/vscode-api-tests/src/singlefolder-tests/terminal.shellIntegration.test.ts` — Shell integration features: command execution, cwd tracking, executable tracking (8 tests)

#### Debug Capabilities
- `extensions/vscode-api-tests/src/singlefolder-tests/debug.test.ts` — Breakpoints (line, function, conditional), debug sessions, variables, stack frames, console output

#### Window & UI
- `extensions/vscode-api-tests/src/singlefolder-tests/window.test.ts` — Active editor, view columns, visible editors, editor selection change events, status bar items, quick input, input box, message boxes (17 tests)
- `extensions/vscode-api-tests/src/singlefolder-tests/quickInput.test.ts` — QuickPick and InputBox with validation, buttons, items, event handling

#### Notebook Support
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.api.test.ts` — Notebook document creation, cell execution, output handling, kernel controller lifecycle
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.kernel.test.ts` — Notebook kernel registration and multi-kernel scenarios (6 tests)
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.editor.test.ts` — Notebook editor selection, cell focusing, visibility management
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.document.test.ts` — Notebook document metadata, cell properties, serialization
- `extensions/vscode-api-tests/src/singlefolder-tests/ipynb.test.ts` — Jupyter notebook (.ipynb) file handling

#### Configuration & State
- `extensions/vscode-api-tests/src/singlefolder-tests/configuration.test.ts` — Workspace.getConfiguration, defaults, language-specific settings, update operations
- `extensions/vscode-api-tests/src/singlefolder-tests/state.test.ts` — Global and workspace state storage (2 tests)
- `extensions/vscode-api-tests/src/singlefolder-tests/env.test.ts` — Environment variables, clipboard, UI kind detection, app name/version
- `extensions/vscode-api-tests/src/singlefolder-tests/env.power.test.ts` — Power state, memory state, context creation/fetch (6 tests)

#### Advanced Features
- `extensions/vscode-api-tests/src/singlefolder-tests/chat.test.ts` — Chat provider registration, messages, participants, tools (4 tests)
- `extensions/vscode-api-tests/src/singlefolder-tests/chat.runInTerminal.test.ts` — Chat integration with terminal execution
- `extensions/vscode-api-tests/src/singlefolder-tests/lm.test.ts` — Language model API integration
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.tasks.test.ts` — Task provider registration and execution (6 tests)
- `extensions/vscode-api-tests/src/singlefolder-tests/tree.test.ts` — Tree view provider with items, icons, children, selection
- `extensions/vscode-api-tests/src/singlefolder-tests/extensions.test.ts` — Extension activation, version checking, API access
- `extensions/vscode-api-tests/src/singlefolder-tests/interactiveWindow.test.ts` — Interactive window (REPL-like) support (1 test)
- `extensions/vscode-api-tests/src/singlefolder-tests/module.test.ts` — Module loading and require patterns
- `extensions/vscode-api-tests/src/singlefolder-tests/types.test.ts` — Type system validation (Uri, Range, Position, etc.)
- `extensions/vscode-api-tests/src/singlefolder-tests/rpc.test.ts` — RPC protocol validation and remote capabilities (2 tests)
- `extensions/vscode-api-tests/src/singlefolder-tests/proxy.test.ts` — Proxy and tunnel scenarios (2 tests)

#### Browser & Platform
- `extensions/vscode-api-tests/src/singlefolder-tests/browser.test.ts` — Browser-specific tests: fetch, storage, clipboard (15 tests)
- `extensions/vscode-api-tests/src/singlefolder-tests/browser.cdp.test.ts` — Chrome DevTools Protocol support
- `extensions/vscode-api-tests/src/singlefolder-tests/browser.tools.test.ts` — Browser tooling API

### Multi-Folder Workspace Tests
- `extensions/vscode-api-tests/src/workspace-tests/workspace.test.ts` — Multi-folder workspace scenarios: workspaceFile, multiple folders, folder resolution

---

## Types / Interfaces

- `extensions/vscode-api-tests/src/memfs.ts` — Implements vscode.FileSystemProvider; defines File and Directory classes
- All test files use vscode module types: TextEditor, TextDocument, Uri, Range, Position, Disposable, Diagnostic, etc.

---

## Configuration

- `extensions/vscode-api-tests/package.json` — Extension manifest with:
  - **enabledApiProposals**: 62 proposed API features under test (activeComment, authSession, browser, chatProvider, notebook*, terminal*, etc.)
  - **Contributes**: Language model chat providers, chat participants, notebook types, debuggers, task definitions, breakpoints, configuration schema
  - **Configuration defaults**: Language-specific editor settings ([abcLang])
- `extensions/vscode-api-tests/tsconfig.json` — TypeScript configuration: src → out, includes vscode.d.ts and proposed API types
- `extensions/vscode-api-tests/.vscode/launch.json` — Debug configuration for running tests in VS Code
- `extensions/vscode-api-tests/.vscode/tasks.json` — Build tasks
- `extensions/vscode-api-tests/.npmrc` — NPM registry configuration
- `extensions/vscode-api-tests/.vscodeignore` — Files excluded from packaging

---

## Examples / Fixtures

- `extensions/vscode-api-tests/testWorkspace/` — Test fixtures for single-folder workspace:
  - Sample files: `10linefile.ts`, `30linefile.ts`, `myFile.ts`, `lorem.txt`, `simple.txt`, `far.js`, `debug.js`
  - Images: `image.png`, `image%.png`, `image%02.png`, `sub/image.png`
  - Configuration: `.vscode/settings.json`, `.vscode/launch.json`, `bower.json`
  - Special: `test.ipynb` (Jupyter notebook), `worker.js`
  - Directories: `files-exclude/`, `search-exclude/` (for exclusion testing)
- `extensions/vscode-api-tests/testWorkspace2/` — Secondary workspace folder:
  - `simple.txt`
  - `.vscode/settings.json`
- `extensions/vscode-api-tests/testworkspace.code-workspace` — Multi-folder workspace definition

---

## Documentation

- `extensions/vscode-api-tests/media/icon.png` — Extension icon
- Files follow MIT license header; part of microsoft/vscode repository

---

## Notable Clusters

- `extensions/vscode-api-tests/src/singlefolder-tests/` — 36 test files; comprehensive coverage of vscode API surface across editing, workspace, terminal, debug, notebook, and UI domains
- `extensions/vscode-api-tests/src/` — 5 core files (extension.ts, utils.ts, memfs.ts + 2 runner configs); provides test infrastructure and isolation layer
- `extensions/vscode-api-tests/testWorkspace/` — Multi-format fixture directory with TypeScript, JavaScript, Jupyter, images, and subdirectories; tests file handling, language detection, debugging
- **API Proposals Coverage**: 62 experimental/proposed APIs across chat, LM, terminal, notebook, authentication, embed, and remote categories — signals where VS Code is evolving
