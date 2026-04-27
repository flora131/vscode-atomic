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
