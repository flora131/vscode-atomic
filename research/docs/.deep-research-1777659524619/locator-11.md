# Locator Report: Partition 11 — vscode-api-tests Extension

## Research Question
What it would take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

## Scope Summary
- **Directory**: `extensions/vscode-api-tests/` (75 total files, ~13,958 LOC)
- **Focus**: Extension API test suite that exercises core IDE surfaces

## Key Finding
This partition is a comprehensive integration test extension that validates VS Code's public extension API across all major IDE features. It exercises 40+ distinct API modules through 38 test files organized by feature domain. The test corpus directly maps to core IDE functionality that would need to be ported.

---

## Implementation Files

### Test Infrastructure
- `extensions/vscode-api-tests/src/extension.ts` — Entry point; activates and exposes ExtensionContext as global for tests
- `extensions/vscode-api-tests/src/utils.ts` — Common test utilities (file creation, editor management, logging, polling helpers)
- `extensions/vscode-api-tests/src/memfs.ts` — In-memory filesystem provider (implements vscode.FileSystemProvider interface)
- `extensions/vscode-api-tests/src/singlefolder-tests/index.ts` — Test runner configuration for Mocha; handles multiple execution environments (Electron, Web, Remote)
- `extensions/vscode-api-tests/src/workspace-tests/index.ts` — Workspace-scoped test runner configuration

---

## Tests

### Editor & Document Management
- `extensions/vscode-api-tests/src/singlefolder-tests/editor.test.ts` — Editor surface: snippets, selections, cursor styles, line numbers, clipboard integration
- `extensions/vscode-api-tests/src/singlefolder-tests/documentPaste.test.ts` — Document paste handlers and paste content providers
- `extensions/vscode-api-tests/src/singlefolder-tests/types.test.ts` — Type system (Position, Range, Selection, Uri, etc.)

### Language Intelligence & Diagnostics
- `extensions/vscode-api-tests/src/singlefolder-tests/languages.test.ts` — Language APIs: setTextDocumentLanguage, diagnostics, document language associations
- `extensions/vscode-api-tests/src/singlefolder-tests/languagedetection.test.ts` — Automatic language detection/inference
- `extensions/vscode-api-tests/src/singlefolder-tests/lm.test.ts` — Language model integration (AI chat/completions)
- `extensions/vscode-api-tests/src/singlefolder-tests/chat.test.ts` — Chat participant APIs, chat requests, context handling
- `extensions/vscode-api-tests/src/singlefolder-tests/chat.runInTerminal.test.ts` — Chat tool execution in terminal context

### Debug & Breakpoints
- `extensions/vscode-api-tests/src/singlefolder-tests/debug.test.ts` — Debug API: breakpoints, debug sessions, debug adapter protocol tracking

### Terminal & Shell Integration
- `extensions/vscode-api-tests/src/singlefolder-tests/terminal.test.ts` — Terminal creation, PTY terminals, environment variables, terminal dimensions, exit handling
- `extensions/vscode-api-tests/src/singlefolder-tests/terminal.shellIntegration.test.ts` — Shell integration, command tracking, terminal execution events

### File System & Workspace Navigation
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.fs.test.ts` — Workspace file system APIs (openTextDocument, findFiles, file watching)
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.watcher.test.ts` — File system watchers and change events
- `extensions/vscode-api-tests/src/singlefolder-tests/readonlyFileSystem.test.ts` — Read-only filesystem provider registration
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.test.ts` — Workspace structure (rootPath, workspaceFolders, workspaceFile)
- `extensions/vscode-api-tests/src/workspace-tests/workspace.test.ts` — Multi-folder workspace configuration

### Source Control Integration (Not Found)
- Note: SCM APIs are NOT exercised in this partition's test files. No `scm.test.ts` file exists.

### Notebook/Interactive Computing
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.api.test.ts` — Notebook API surface (lifecycle, kernels)
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.document.test.ts` — Notebook document model and cell editing
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.editor.test.ts` — Notebook editor surface
- `extensions/vscode-api-tests/src/singlefolder-tests/notebook.kernel.test.ts` — Notebook kernel execution and communication
- `extensions/vscode-api-tests/src/singlefolder-tests/ipynb.test.ts` — Jupyter notebook serialization
- `extensions/vscode-api-tests/src/singlefolder-tests/interactiveWindow.test.ts` — Interactive window/REPL surface

### Tasks & Commands
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.tasks.test.ts` — Task API: ShellExecution, CustomExecution, task presentation
- `extensions/vscode-api-tests/src/singlefolder-tests/commands.test.ts` — Command registration and execution

### UI & Window Management
- `extensions/vscode-api-tests/src/singlefolder-tests/window.test.ts` — Window API: dialogs, messages, progress, quick input
- `extensions/vscode-api-tests/src/singlefolder-tests/quickInput.test.ts` — Quick pick and input box
- `extensions/vscode-api-tests/src/singlefolder-tests/tree.test.ts` — Tree view and custom view providers

### Configuration & State Management
- `extensions/vscode-api-tests/src/singlefolder-tests/configuration.test.ts` — Settings/configuration API
- `extensions/vscode-api-tests/src/singlefolder-tests/state.test.ts` — Extension context state (globalState, workspaceState)

### Environment & Extensions
- `extensions/vscode-api-tests/src/singlefolder-tests/env.test.ts` — Environment variables, UIKind detection, extension API introspection
- `extensions/vscode-api-tests/src/singlefolder-tests/env.power.test.ts` — Power management/battery status APIs
- `extensions/vscode-api-tests/src/singlefolder-tests/extensions.test.ts` — Extension activation and lifecycle

### Browser & Platform-Specific
- `extensions/vscode-api-tests/src/singlefolder-tests/browser.test.ts` — Browser environment detection and behavior
- `extensions/vscode-api-tests/src/singlefolder-tests/browser.cdp.test.ts` — Chrome DevTools Protocol integration
- `extensions/vscode-api-tests/src/singlefolder-tests/browser.tools.test.ts` — Browser developer tools

### Advanced/Experimental Features
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.event.test.ts` — Workspace event emissions and ordering
- `extensions/vscode-api-tests/src/singlefolder-tests/module.test.ts` — Module/require system and API versioning
- `extensions/vscode-api-tests/src/singlefolder-tests/rpc.test.ts` — RPC communication protocol
- `extensions/vscode-api-tests/src/singlefolder-tests/proxy.test.ts` — Proxy/network configuration

---

## Types / Interfaces

- `extensions/vscode-api-tests/src/singlefolder-tests/types.test.ts` — Tests for built-in types (no separate .d.ts file in this extension)

---

## Configuration

- `extensions/vscode-api-tests/package.json` — Extension manifest with:
  - 62 enabledApiProposals (experimental/unstable features)
  - Language model chat providers (test vendors)
  - Chat participant definitions
  - Notebook type definitions (notebookCoreTest, nbdtest, nbdserializer)
  - Task definitions (custombuildscript)
  - Debug configuration (mock debugger)
  - Status bar items
  - Configuration schema (farboo.* properties for testing)

- `extensions/vscode-api-tests/.vscode/launch.json` — Debug launch configuration
- `extensions/vscode-api-tests/.vscode/tasks.json` — VS Code tasks for test running
- `extensions/vscode-api-tests/tsconfig.json` — TypeScript compilation configuration

---

## Examples / Fixtures

### Test Workspace
- `extensions/vscode-api-tests/testWorkspace/` — Primary workspace for integration tests:
  - `10linefile.ts`, `30linefile.ts`, `myFile.ts` — Source code fixtures
  - `index.html`, `simple.txt`, `lorem.txt` — Text document fixtures
  - `debug.js`, `far.js`, `worker.js` — Script fixtures for debug/execution tests
  - `test.ipynb` — Jupyter notebook fixture
  - `.vscode/settings.json`, `.vscode/launch.json` — Workspace configuration
  - `files-exclude/`, `search-exclude/` — Exclusion pattern test directories
  - `sub/image.png`, `image*.png` — Binary/image fixtures for file system tests
  - `bower.json` — Dependency manifest fixture

- `extensions/vscode-api-tests/testWorkspace2/` — Secondary workspace for multi-folder tests
  - `.vscode/settings.json` — Separate workspace settings

- `extensions/vscode-api-tests/testworkspace.code-workspace` — Multi-root workspace file

---

## Notable Clusters

### API Module Coverage by Suite
The test suite organizes tests into 38 distinct test modules (suites), each exercising a specific vscode.* namespace or feature:

**Core Editor & Document APIs** (5 files)
- editor, types, documentPaste, languages, editor

**Terminal & Shell** (2 files)
- terminal, terminal.shellIntegration

**Debugging** (1 file)
- debug

**Language Intelligence** (3 files)
- languages, languagedetection, lm

**Chat & AI** (2 files)
- chat, chat.runInTerminal

**Notebooks** (6 files)
- notebook.api, notebook.document, notebook.editor, notebook.kernel, ipynb, interactiveWindow

**File System & Workspace** (7 files)
- workspace, workspace.fs, workspace.watcher, workspace.event, workspace.tasks, readonlyFileSystem

**UI Components** (3 files)
- window, quickInput, tree

**Configuration & State** (2 files)
- configuration, state

**Commands** (1 file)
- commands

**Environment** (3 files)
- env, env.power, extensions

**Browser** (3 files)
- browser, browser.cdp, browser.tools

**Advanced** (3 files)
- module, rpc, proxy

### Extension API Proposals Tested
The `package.json` lists 62 `enabledApiProposals`, indicating this extension exercises experimental/upcoming API surfaces including:
- Chat-related: chatParticipantPrivate, chatProvider, chatPromptFiles, defaultChatParticipant
- Notebooks: notebookDeprecated, notebookLiveShare, notebookMessaging, notebookMime
- Terminal: terminalDataWriteEvent, terminalDimensions
- File system: fileSearchProvider, findFiles2, findTextInFiles, fsChunks
- Terminal: terminalDataWriteEvent, terminalDimensions
- SCM: scmActionButton, scmSelectedProvider, scmTextDocument, scmValidation
- Debug/Test: testObserver, textSearchProvider
- Workspace: workspaceTrust
- Language models: languageModelProxy, inlineCompletionsAdditions

### Gap: No Source Control Testing
Despite SCM APIs being enabled in proposals (scmActionButton, scmSelectedProvider, scmTextDocument, scmValidation), **no source control test file exists**. This is a gap in coverage of a stated core IDE feature.

---

## Documentation

- No dedicated README or markdown documentation exists within this partition. Test documentation is embedded in:
  - Test file comments (/// remarks explaining behavior)
  - package.json manifest descriptions
  - Configuration schema comments

---

## Relevance to Research Question

This partition is **highly relevant** to porting VS Code to Tauri/Rust. It directly maps the extension-facing API surface that any IDE built on VS Code's architecture would need to support:

### What Exists (API Contracts to Port)
1. **Editor Surface**: Text editing, snippets, selections, clipboard
2. **Terminal Integration**: Process creation, PTY management, shell integration
3. **Debugging**: Breakpoint management, debug adapter protocol
4. **Language Services**: Syntax highlighting, diagnostics, language detection
5. **File System Abstraction**: Virtual filesystem providers, watchers, glob patterns
6. **Workspace Model**: Multi-folder workspaces, folder-aware configuration
7. **UI Primitives**: Dialogs, quick input, tree views, status bar
8. **Configuration Management**: Settings, state persistence (global/workspace)
9. **Task Execution**: Shell and custom task runners
10. **Notebook/REPL**: Notebook documents, kernel execution, cell evaluation

### What's Missing (Known Gaps)
- **Source Control**: Despite enabled SCM proposals, zero test coverage for SCM APIs
- **Custom Editors**: Not explicitly tested
- **WebView Integration**: Not found in this partition
- **Settings Sync**: Not found in this partition

### Core Porting Challenges Revealed
1. **RPC Layer**: `rpc.test.ts` shows VS Code uses internal RPC for cross-boundary communication
2. **Browser/Desktop Dual Path**: `browser.*.test.ts` files indicate API must work in both Electron and Web contexts
3. **Complex State Management**: Global vs workspace-scoped state requires distributed consistency
4. **Terminal PTY Complexity**: Direct PTY creation and environment variable mutation (EnvironmentVariableMutator)
5. **File System Abstraction Depth**: Custom filesystem providers with full CRUD semantics
6. **Notebook Kernel Protocol**: Multi-step kernel communication and stream handling

---

## Statistics

- **Total Files**: 75 (50 .ts files, 3 .js files, multiple config/fixture files)
- **Total LOC**: ~13,958
- **Test Files**: 38 distinct `*.test.ts` modules
- **Test Suites Defined**: 40+ suites (some files have nested suites)
- **API Modules Exercised**: 40+ distinct vscode.* namespaces
- **Enabled API Proposals**: 62 experimental features

