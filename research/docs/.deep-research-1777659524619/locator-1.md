# VS Code Core IDE Functionality - Tauri/Rust Porting Analysis

## Architecture Overview

The scope analysis covers `src/vs/` (1,943,130 LOC across 5,951 files), organized in layered architecture:
- **base/** - Utility/IPC primitives (common types, protocols)
- **platform/** - Dependency injection + service interfaces (100+ services)
- **editor/** - Monaco editor core (text model, rendering, language features)
- **workbench/** - IDE shell with contributions (UI parts, sidebar, views)
- **code/** - Electron main process, browser/web bootstrap
- **server/** - Remote server support
- **sessions/** - Agent/session management

---

## Implementation

### Core Editor (Text Model & Rendering)
- `src/vs/editor/common/editorCommon.ts` — Base editor interfaces
- `src/vs/editor/common/core/position.ts` — Cursor/selection model
- `src/vs/editor/common/core/range.ts` — Range operations
- `src/vs/editor/common/core/selection.ts` — Selection tracking
- `src/vs/editor/common/core/editOperation.ts` — Edit operations (insert/delete)
- `src/vs/editor/common/core/edits/` — 5+ edit types (textEdit, lineEdit, etc.)
- `src/vs/editor/common/diff/linesDiffComputer.ts` — Diff algorithm core
- `src/vs/editor/common/languageFeatureRegistry.ts` — Language feature registration
- `src/vs/editor/browser/view.ts` — Browser view container
- `src/vs/editor/browser/viewParts/` — 15+ view parts (margins, decorations, scrollbar, line numbers)
- `src/vs/editor/browser/gpu/gpu.ts` — GPU rendering infrastructure
- `src/vs/editor/browser/gpu/atlas/textureAtlas.ts` — Text rasterization cache
- `src/vs/editor/browser/controller/editContext/nativeEditContext.ts` — Native text input
- `src/vs/editor/browser/controller/mouseHandler.ts` — Mouse/pointer input handling

### Language Features & Syntax
- `src/vs/editor/contrib/suggest/` — 10+ files, autocomplete/IntelliSense
- `src/vs/editor/contrib/parameterHints/` — Function signature help
- `src/vs/editor/contrib/rename/` — Symbol rename (LSP)
- `src/vs/editor/contrib/semanticTokens/` — Semantic highlighting
- `src/vs/editor/contrib/codelens/` — CodeLens support
- `src/vs/editor/contrib/documentSymbols/outlineModel.ts` — Document outline/symbols
- `src/vs/editor/contrib/find/` — Find & replace UI
- `src/vs/editor/common/languages.ts` — Language API definitions
- `src/vs/editor/common/languages/languageConfigurationRegistry.ts` — Language config loading
- `src/vs/editor/contrib/codeAction/` — Code actions/quick fixes

### Debugging
- `src/vs/workbench/contrib/debug/browser/debugService.ts` — Main debug service (100+ contributors)
- `src/vs/workbench/contrib/debug/browser/debugSession.ts` — Debug session lifecycle
- `src/vs/workbench/contrib/debug/browser/debugAdapterManager.ts` — DAP protocol manager
- `src/vs/workbench/contrib/debug/common/debugModel.ts` — Breakpoints, stack frames, scopes
- `src/vs/workbench/contrib/debug/common/debugProtocol.d.ts` — DAP types
- `src/vs/workbench/contrib/debug/browser/rawDebugSession.ts` — DAP protocol handler
- `src/vs/workbench/contrib/debug/browser/variables/` — Variables/watch views

### Source Control
- `src/vs/workbench/contrib/scm/common/scmService.ts` — SCM provider manager
- `src/vs/workbench/contrib/scm/browser/scmViewPane.ts` — SCM UI
- `src/vs/workbench/contrib/scm/browser/quickDiffModel.ts` — Quick diff (line decorations)
- `src/vs/workbench/contrib/scm/common/quickDiff.ts` — Quick diff service

### Terminal
- `src/vs/workbench/contrib/terminal/browser/terminalService.ts` — Terminal lifecycle mgmt
- `src/vs/workbench/contrib/terminal/browser/terminalInstance.ts` — Individual terminal instance
- `src/vs/workbench/contrib/terminal/browser/terminalProcessManager.ts` — Process management
- `src/vs/workbench/contrib/terminal/browser/xterm/xtermTerminal.ts` — XTerm integration
- `src/vs/workbench/contrib/terminal/common/basePty.ts` — PTY abstraction
- `src/vs/workbench/contrib/terminal/electron-browser/localTerminalBackend.ts` — Local shell spawning

### File Management & Explorer
- `src/vs/workbench/services/files/common/files.ts` — File service interface
- `src/vs/workbench/services/files/electron-browser/diskFileSystemProvider.ts` — Disk FS provider
- `src/vs/workbench/contrib/files/browser/explorerViewlet.ts` — File explorer UI
- `src/vs/workbench/contrib/files/common/explorerModel.ts` — Explorer tree model
- `src/vs/workbench/services/textfile/common/textFileEditorModel.ts` — Text file buffer mgmt
- `src/vs/workbench/services/textfile/browser/textFileService.ts` — Text file operations (save/load)

### Navigation & Search
- `src/vs/workbench/contrib/search/browser/searchView.ts` — Search UI
- `src/vs/workbench/contrib/search/browser/searchTreeModel/searchModel.ts` — Search model
- `src/vs/workbench/contrib/outline/browser/outline.ts` — Outline/symbols view
- `src/vs/workbench/contrib/search/browser/symbolsQuickAccess.ts` — Go to Symbol quickopen

### Editor Services
- `src/vs/workbench/services/editor/common/editorService.ts` — Editor management service
- `src/vs/workbench/services/editor/common/editorGroupsService.ts` — Editor groups (split layout)
- `src/vs/workbench/services/editor/browser/editorPaneService.ts` — Editor pane registry
- `src/vs/workbench/services/editor/browser/editorResolverService.ts` — Editor type resolution

### Text File Services
- `src/vs/workbench/services/textfile/common/textFileSaveParticipant.ts` — Save participants (format, trim)
- `src/vs/workbench/services/textfile/common/textFileEditorModelManager.ts` — Buffer pool mgmt
- `src/vs/workbench/services/textfile/common/encoding.ts` — Encoding detection & conversion

### Theme & Appearance
- `src/vs/workbench/services/themes/common/workbenchThemeService.ts` — Theme service
- `src/vs/workbench/services/themes/common/colorThemeData.ts` — Color theme loading
- `src/vs/platform/theme/common/colors/editorColors.ts` — Editor color registry

### Configuration & Workspace
- `src/vs/workbench/services/configuration/common/configuration.ts` — Config change tracking
- `src/vs/workbench/services/configuration/browser/configurationService.ts` — Settings service
- `src/vs/workbench/services/configuration/common/configurationEditing.ts` — Edit settings.json
- `src/vs/platform/workspace/common/workspace.ts` — Workspace abstraction
- `src/vs/workbench/services/workspaces/common/workspaceTrust.ts` — Workspace trust model

### Extension System & APIs
- `src/vs/workbench/services/extensions/common/extensionHostProtocol.ts` — Extension host RPC protocol
- `src/vs/workbench/services/extensions/common/rpcProtocol.ts` — Generic RPC transport
- `src/vs/workbench/services/extensions/common/abstractExtensionService.ts` — Extension lifecycle
- `src/vs/workbench/services/extensions/electron-browser/localProcessExtensionHost.ts` — Local extension host
- `src/vs/workbench/api/browser/main*.ts` — 80+ mainThread files for API (documents, terminal, debug, etc.)
- `src/vs/workbench/api/common/extHost*.ts` — Extension-side API stubs

### DI & Instantiation
- `src/vs/platform/instantiation/common/instantiationService.ts` — Service container
- `src/vs/platform/instantiation/common/serviceCollection.ts` — Service registry
- `src/vs/platform/instantiation/common/descriptors.ts` — Service descriptors

### IPC & Communication
- `src/vs/base/parts/ipc/common/ipc.ts` — Base IPC interface
- `src/vs/base/parts/ipc/electron-main/ipcMain.ts` — Electron main IPC
- `src/vs/base/parts/ipc/electron-browser/ipc.electron.ts` — Electron renderer IPC
- `src/vs/base/common/jsonRpcProtocol.ts` — JSON-RPC protocol implementation

### Keybindings
- `src/vs/workbench/services/keybinding/common/keybindingIO.ts` — Keybinding parsing (JSON)
- `src/vs/workbench/services/keybinding/browser/keybindingService.ts` — Keybinding service
- `src/vs/workbench/services/keybinding/common/macLinuxKeyboardMapper.ts` — Platform keyboard mapping
- `src/vs/workbench/services/keybinding/common/windowsKeyboardMapper.ts` — Windows keyboard mapping

### Platform Services (Common)
- `src/vs/platform/notification/common/notification.ts` — Notification service
- `src/vs/platform/progress/common/progress.ts` — Progress service
- `src/vs/platform/quickinput/common/quickInput.ts` — Quick input service
- `src/vs/platform/commands/common/commands.ts` — Command registry
- `src/vs/platform/contextkey/common/contextkey.ts` — Context key service
- `src/vs/platform/clipboard/common/clipboardService.ts` — Clipboard API
- `src/vs/platform/dialogs/common/dialogs.ts` — File dialogs

### Remote & Server
- `src/vs/workbench/services/remote/common/remoteAgentService.ts` — Remote connection
- `src/vs/server/node/server.main.ts` — VS Code Server entry point
- `src/vs/server/node/remoteExtensionHostAgentServer.ts` — Remote extension host server
- `src/vs/platform/remote/common/remoteAgentService.ts` — Remote agent protocol

### Workbench Shell
- `src/vs/workbench/workbench.desktop.main.ts` — Electron desktop bootstrap
- `src/vs/workbench/workbench.web.main.ts` — Web/browser bootstrap
- `src/vs/code/electron-main/main.ts` — Electron main process entry
- `src/vs/code/browser/workbench/workbench.ts` — Browser workbench container

---

## Tests

### Editor Tests
- `src/vs/editor/test/browser/testCodeEditor.ts` — Editor test fixture
- `src/vs/editor/test/common/testTextModel.ts` — Text model test helpers
- `src/vs/editor/contrib/suggest/test/browser/suggestModel.test.ts` — Autocomplete tests
- `src/vs/editor/contrib/suggest/test/browser/suggestController.test.ts` — Suggest controller tests
- `src/vs/editor/contrib/find/test/browser/findModel.test.ts` — Find model tests
- `src/vs/editor/contrib/codeAction/test/browser/codeAction.test.ts` — Code action tests

### Workbench Tests
- `src/vs/workbench/test/browser/codeeditor.test.ts` — Code editor integration
- `src/vs/workbench/test/browser/quickAccess.test.ts` — Quick open/access
- `src/vs/workbench/test/common/workbenchTestServices.ts` — Workbench test helpers
- `src/vs/workbench/services/textfile/test/browser/textFileService.test.ts` — Text file service tests
- `src/vs/workbench/services/editor/test/browser/editorService.test.ts` — Editor service tests
- `src/vs/workbench/contrib/debug/test/browser/callStack.test.ts` — Debug call stack tests
- `src/vs/workbench/contrib/scm/test/browser/scmHistory.test.ts` — SCM history tests

### Configuration Tests
- `src/vs/workbench/services/configuration/test/browser/configurationService.test.ts` — Config service
- `src/vs/workbench/services/configuration/test/common/configurationModels.test.ts` — Config models

### Extension Tests
- `src/vs/workbench/services/extensions/test/common/rpcProtocol.test.ts` — RPC protocol tests
- `src/vs/workbench/services/extensions/test/common/extensionDescriptionRegistry.test.ts` — Extension registry

### Search Tests
- `src/vs/workbench/contrib/search/test/browser/searchModel.test.ts` — Search model
- `src/vs/workbench/contrib/search/test/browser/searchActions.test.ts` — Search actions

### Platform Tests
- `src/vs/platform/instantiation/test/common/instantiationService.test.ts` — DI container
- `src/vs/base/parts/ipc/test/common/ipc.test.ts` — IPC protocol
- `src/vs/base/test/common/jsonRpcProtocol.test.ts` — JSON-RPC tests

---

## Types / Interfaces

### Core Editor Types
- `src/vs/editor/common/languages.ts` — ILanguageExtensionPoint, CompletionItemProvider, DefinitionProvider
- `src/vs/editor/common/languageFeatureRegistry.ts` — Language feature registry pattern
- `src/vs/editor/common/editorCommon.ts` — IEditor, ITextModel, IPosition, IRange, ISelection
- `src/vs/editor/common/encodedTokenAttributes.ts` — Token styling attributes

### Editor Services
- `src/vs/editor/browser/editorBrowser.ts` — IBrowser editor (rich interface)
- `src/vs/editor/browser/services/codeEditorService.ts` — CodeEditorService interface
- `src/vs/editor/browser/services/bulkEditService.ts` — BulkEditService (multi-file edits)

### Workbench Services
- `src/vs/workbench/services/editor/common/editorService.ts` — IEditorService
- `src/vs/workbench/services/editor/common/editorGroupsService.ts` — IEditorGroupsService
- `src/vs/workbench/services/textfile/common/textfiles.ts` — ITextFileService
- `src/vs/workbench/services/configuration/common/configuration.ts` — IConfigurationService
- `src/vs/workbench/services/extensions/common/extensions.ts` — IExtensionService
- `src/vs/workbench/services/themes/common/workbenchThemeService.ts` — IWorkbenchThemeService

### Debug Types
- `src/vs/workbench/contrib/debug/common/debugModel.ts` — IDebugModel, StackFrame, Scope, Variable
- `src/vs/workbench/contrib/debug/common/debug.ts` — IDebugService, IDebugSession, IBreakpoint
- `src/vs/workbench/contrib/debug/common/debugProtocol.d.ts` — DAP TypeScript types

### SCM Types
- `src/vs/workbench/contrib/scm/common/scm.ts` — ISCMService, ISCMRepository, ISCMInput

### Terminal Types
- `src/vs/workbench/contrib/terminal/common/terminal.ts` — ITerminalService, ITerminal, ITerminalInstance

### File System Types
- `src/vs/platform/files/common/files.ts` — IFileService, FileSystemProvider, FileChange

### Platform Service Interfaces
- `src/vs/platform/notification/common/notification.ts` — INotificationService
- `src/vs/platform/quickinput/common/quickInput.ts` — IQuickInputService
- `src/vs/platform/progress/common/progress.ts` — IProgressService
- `src/vs/platform/commands/common/commands.ts` — ICommandService, Command
- `src/vs/platform/contextkey/common/contextkey.ts` — IContextKeyService
- `src/vs/platform/clipboard/common/clipboardService.ts` — IClipboardService
- `src/vs/platform/dialogs/common/dialogs.ts` — IFileDialogService

### Extension Host API
- `src/vs/workbench/api/common/extHost.api.impl.ts` — Extension API surface (vscode namespace)
- `src/vs/workbench/api/common/extHost.protocol.ts` — Extension-main thread RPC contracts

---

## Configuration

### Editor Configuration
- `src/vs/editor/common/config/editorOptions.ts` — Editor setting definitions (150+ options)
- `src/vs/editor/common/config/editorConfigurationSchema.ts` — Schema generation
- `src/vs/editor/common/config/fontInfo.ts` — Font metrics caching

### Keybinding Configuration
- `src/vs/workbench/services/keybinding/common/keybindingIO.ts` — keybindings.json format
- `src/vs/platform/keybinding/common/keybindingIO.ts` — Keybinding serialization

### Language Configuration
- `src/vs/editor/common/languages/languageConfiguration.ts` — Language-specific settings (brackets, indentation)
- `src/vs/editor/common/languages/languageConfigurationRegistry.ts` — Registry for language configs

### Theme Configuration
- `src/vs/workbench/services/themes/common/themeConfiguration.ts` — Theme preference settings

### Terminal Configuration
- `src/vs/workbench/contrib/terminal/common/terminalConfiguration.ts` — Terminal.integrated.* settings

### Debug Configuration
- `src/vs/workbench/contrib/debug/common/debugSchemas.ts` — launch.json schema

### Workspace Settings
- `src/vs/platform/configuration/common/configuration.ts` — settings.json schema definition
- `src/vs/workbench/services/configuration/common/configurationModels.ts` — Config model layers (user/workspace/folder)

---

## Examples / Fixtures

### Editor Fixtures
- `src/vs/editor/test/browser/testCodeEditor.ts` — CodeEditor test fixture
- `src/vs/workbench/test/browser/componentFixtures/editor/inlineChatZoneWidget.fixture.ts` — Inline chat zone

### Workbench Fixtures
- `src/vs/workbench/test/browser/componentFixtures/chat/chatInput.fixture.ts` — Chat input widget
- `src/vs/workbench/test/browser/componentFixtures/chat/chatArtifacts.fixture.ts` — Chat artifacts
- `src/vs/workbench/test/common/workbenchTestServices.ts` — Workbench service mocks

### Search Examples
- `src/vs/workbench/contrib/search/test/browser/mockSearchTree.ts` — Mock search tree

### Terminal Helpers
- `src/vs/workbench/contrib/terminal/browser/terminalTestHelpers.ts` — Terminal testing utilities

---

## Documentation

### Editor & Language Features
- `src/vs/editor/common/languages/highlights/` — Tree-sitter highlight queries (.scm files) for syntax
- `src/vs/workbench/contrib/debug/common/debugProtocol.d.ts` — DAP protocol comments
- `src/vs/platform/agentHost/common/state/AGENTS.md` — Agent protocol documentation

### Extension API
- `src/vs/workbench/api/common/extHost.protocol.ts` — RPC contract documentation
- `src/vs/platform/extensions/common/extensionPoints.json` — Extension point schema

### Architecture
- `src/vs/platform/instantiation/common/instantiation.ts` — DI patterns (decorators, containers)
- `src/vs/base/parts/ipc/common/ipc.ts` — IPC channel protocol docs

---

## Notable Clusters

### Editor Core (500+ files)
`src/vs/editor/`
- **browser/** — 100+ rendering/input files (view, GPU, edit context, mouse, pointer, scrollbar)
- **common/** — 200+ core model files (text model, languages, diff, cursor, selection, range)
- **contrib/** — 200+ feature files (suggest, codeAction, find, folding, rename, etc.)
- **standalone/** — Standalone editor package
- Language features distributed across browser/contrib (completion, hover, signature help, codelens, etc.)

### Workbench Services (400+ files)
`src/vs/workbench/services/`
- **editor/** — 15 files, editor group/pane management
- **textfile/** — 30 files, buffer pooling, save/load, encoding
- **configuration/** — 15 files, settings hierarchical model
- **extensions/** — 50 files, extension host lifecycle & RPC
- **keybinding/** — 100+ files (mostly keyboard layouts), key mapping
- **themes/** — 25 files, color/icon theme loading
- **terminal/** — 10+ files (PTY abstraction, remote terminal)
- **files/** — 10 files, file service & disk provider
- **remote/** — 10 files, remote agent connection

### Workbench Contributions (1000+ files)
`src/vs/workbench/contrib/`
- **debug/** — 100+ files (breakpoints, call stack, variables, REPL, DAP adapter)
- **terminal/** — 150+ files (xterm integration, shell detection, PTY mgmt)
- **files/** — 50+ files (explorer, file commands, editors)
- **scm/** — 30 files (source control views, quick diff)
- **search/** — 60+ files (find/replace, symbol search, AI search)
- **extensions/** — 70+ files (extension marketplace, recommendations, profiling)
- **chat/** — 100+ files (chat widget, agent sessions, editing)

### Platform Services (150+ files)
`src/vs/platform/`
- **instantiation/** — DI container (9 files)
- **keybinding/commands/quickinput/** — Command dispatch & UI (30 files)
- **notification/progress/** — User feedback (10 files)
- **configuration/clipboard/dialogs/** — Core APIs (20 files)
- **extensions/extensionManagement/** — Extension loading (40 files)
- **files/** — File system abstraction (30 files)
- **terminal/externalTerminal/** — Terminal APIs (20 files)
- **windows/workspaces/** — Workspace/window management (20 files)
- **userDataSync/userDataProfile/** — Settings sync (40 files)

### IPC & Protocol (50+ files)
`src/vs/base/parts/ipc/` + `src/vs/workbench/services/extensions/common/`
- **ipc.ts** — Base channel interface
- **ipc.electron.ts** — Electron IPC bridge
- **ipc.net.ts** — TCP/network IPC
- **rpcProtocol.ts** — Generic RPC proxy generation
- **jsonRpcProtocol.ts** — JSON-RPC implementation
- Protocol versions for extension host, remote server, debug adapter

### API Layer (100+ files)
`src/vs/workbench/api/`
- **browser/mainThread*.ts** — 80+ files exposing IDE services to extensions
- **common/extHost*.ts** — Extension-side API stubs (documents, terminal, debug, etc.)
- **common/extHost.protocol.ts** — RPC method signatures (100+ methods)

### Base Utilities (200+ files)
`src/vs/base/`
- **browser/ui/** — Base UI components (buttons, lists, trees, menus, inputs)
- **common/** — Common algorithms (arrays, strings, uri, observable, async)
- **parts/contextmenu/** — Context menu rendering
- **parts/sandbox/** — Electron sandbox/preload

---

## Summary

This comprehensive mapping documents the critical architectural domains required for porting VS Code core functionality:

1. **Text Editing Engine** — 500+ files covering text model (position, range, selection), editing operations, diff algorithms, and cursor management.

2. **Language Services** — 200+ files for autocomplete, hover, parameter hints, code actions, semantic highlighting, and symbol navigation across editor contributions and LSP integration.

3. **Debugging** — 100+ files implementing Debug Adapter Protocol (DAP), breakpoint management, call stacks, variables view, and REPL.

4. **Terminal** — 150+ files for PTY abstraction, shell integration, process spawning, xterm rendering, and remote terminal support.

5. **Source Control** — 30+ files for provider abstraction, diff decorations, and repository UI.

6. **File Management** — 50+ files for file system abstraction, explorer UI, buffer pooling, encoding detection, and save/load.

7. **Extension System** — 50+ files for RPC-based extension host lifecycle, 80+ mainThread proxies exposing IDE services, and extension API surface (vscode namespace).

8. **DI & IPC Infrastructure** — 50+ files for service container, JSON-RPC, Electron IPC, and TCP channel implementations.

9. **Configuration & Workspace** — 40+ files for hierarchical settings model, workspace trust, and keybinding management.

10. **Workbench Shell** — 400+ service files + 1000+ contribution files composing the IDE UI, layout, and integrated experiences.

Key porting challenges: managing the layered dependency structure (common → browser/node → electron-*), serializing complex service contracts across process boundaries via RPC, implementing GPU-accelerated text rendering, and replicating 150+ keyboard layout mappings.
