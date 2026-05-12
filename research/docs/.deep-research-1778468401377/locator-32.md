# Codebase Locator: Extensions/Merge-Conflict (Partition 32 of 80)

## Research Context
Files relevant to porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust, focusing on the merge-conflict extension's use of CodeLens API and other IDE primitives.

## Implementation

### Core Extension Entry Point
- `extensions/merge-conflict/src/mergeConflictMain.ts` - Main activation function, registers services and disposables via vscode.ExtensionContext

### CodeLens Provider (Primary API Focus)
- `extensions/merge-conflict/src/codelensProvider.ts` - Implements `vscode.CodeLensProvider` interface, registers provider via `vscode.languages.registerCodeLensProvider()` for multiple document schemes (file, vscode-vfs, untitled, vscode-userdata). Provides code lens for conflict resolution actions at lines 101-106.

### Command Handling
- `extensions/merge-conflict/src/commandHandler.ts` - Registers 10 text editor commands via `vscode.commands.registerCommand()`. Implements command routing and execution flow. Uses `vscode.window.activeTextEditor` to get active editor context (lines 43-50).

### Document Parsing & Conflict Detection
- `extensions/merge-conflict/src/mergeConflictParser.ts` - Parses merge conflict markers (<<<<<<<, =======, >>>>>>>) from text documents. Returns structured conflict descriptors for IDE presentation.
- `extensions/merge-conflict/src/documentMergeConflict.ts` - Models individual merge conflicts with commit/apply edit operations.
- `extensions/merge-conflict/src/documentTracker.ts` - Implements `IDocumentMergeConflictTrackerService` for caching and tracking document state. Uses `Delayer` for debounced scanning.

### Text Decoration & Visual Rendering
- `extensions/merge-conflict/src/mergeDecorator.ts` - Creates and applies `vscode.TextEditorDecorationType` decorations via `vscode.window.createTextEditorDecorationType()`. Manages visual styling for conflict regions. Responds to document changes via `vscode.workspace.onDidChangeTextDocument` events (lines 34-36). Uses theme colors for decorations (lines 68-79).

### Content Provider for Diff Views
- `extensions/merge-conflict/src/contentProvider.ts` - Implements `vscode.TextDocumentContentProvider` for virtual documents supporting diff comparisons. Registers provider scheme via `vscode.workspace.registerTextDocumentContentProvider()`.

### Service Orchestration
- `extensions/merge-conflict/src/services.ts` - Central service wrapper managing lifecycle of CodeLensProvider, CommandHandler, Decorator, ContentProvider, and DocumentTracker. Coordinates configuration updates via `vscode.workspace.onDidChangeConfiguration` callbacks.

### Utility & Data Structures
- `extensions/merge-conflict/src/interfaces.ts` - TypeScript interfaces defining extension contracts: `IDocumentMergeConflict`, `IDocumentMergeConflictTracker`, `IExtensionConfiguration`, `IMergeRegion`, `CommitType` enum.
- `extensions/merge-conflict/src/delayer.ts` - Generic debounce/delay utility class for async task scheduling.

## Types / Interfaces

- `extensions/merge-conflict/src/interfaces.ts` - Contains all extension-wide interface definitions:
  - `IMergeRegion` - Represents header, content, and decorator content ranges
  - `CommitType` enum - Current, Incoming, Both
  - `IExtensionConfiguration` - CodeLens, decorators, editor overview toggles
  - `IDocumentMergeConflict` - Full conflict with commit/apply operations
  - `IDocumentMergeConflictDescriptor` - Conflict metadata
  - `IDocumentMergeConflictTracker` - Async conflict lookup interface
  - `IDocumentMergeConflictTrackerService` - Factory for trackers

## Configuration

### Extension Configuration
- `extensions/merge-conflict/package.json` - Manifest with:
  - **Activation event**: `onStartupFinished` (lines 22-23)
  - **Commands**: 10 merge conflict resolution commands with enablement contexts (lines 38-110)
  - **Menus**: Integration with SCM and editor title contexts (lines 112-136)
  - **Configuration properties** (lines 138-172):
    - `merge-conflict.codeLens.enabled` - Toggle CodeLens rendering
    - `merge-conflict.decorators.enabled` - Toggle visual decorations
    - `merge-conflict.autoNavigateNextConflict.enabled` - Auto-advance to next conflict
    - `merge-conflict.diffViewPosition` - Control diff view layout (Current/Beside/Below)
  - **Browser entry point**: `dist/browser/mergeConflictMain` (line 26)
  - **Node entry point**: `out/mergeConflictMain` (line 25)

### Build Configuration
- `extensions/merge-conflict/tsconfig.json` - TypeScript compilation for Node target, includes vscode.d.ts types
- `extensions/merge-conflict/tsconfig.browser.json` - Browser-specific TypeScript configuration
- `extensions/merge-conflict/esbuild.mts` - Node platform build configuration
- `extensions/merge-conflict/esbuild.browser.mts` - Browser platform build configuration

### Localization
- `extensions/merge-conflict/package.nls.json` - English localization strings for all commands, menus, configuration labels, and help text

### Package Management
- `extensions/merge-conflict/package.json` - Dependency: `@vscode/extension-telemetry` ^0.9.8
- `extensions/merge-conflict/package-lock.json` - Locked dependency versions
- `extensions/merge-conflict/.npmrc` - NPM configuration
- `extensions/merge-conflict/.vscodeignore` - Files excluded from vsix packaging

## Documentation

- `extensions/merge-conflict/README.md` - Short feature overview and link to external VS Code merge conflict documentation

## Notable Clusters

### API Surface Coverage
The extension exercises 5 major VS Code extension API categories:

1. **CodeLens API** (`vscode.languages.registerCodeLensProvider`) - Line 101
2. **Command API** (`vscode.commands.registerCommand`, `vscode.commands.executeCommand`) - Lines 30-40, 55
3. **Decoration API** (`vscode.window.createTextEditorDecorationType`, `editor.setDecorations`) - Lines 68-79, 229
4. **Workspace API** (`vscode.workspace.onDidChangeConfiguration`, `vscode.workspace.onDidChangeTextDocument`, `vscode.workspace.openTextDocument`) - Lines 46, 34, 276
5. **Window API** (`vscode.window.visibleTextEditors`, `vscode.window.activeTextEditor`, `vscode.window.showWarningMessage`) - Lines 28, 48, 93

### Virtual Document Scheme Handling
Registers for 4 document schemes in CodeLensProvider (line 101-106):
- `file` - Regular file system
- `vscode-vfs` - Virtual file system
- `untitled` - Unsaved documents
- `vscode-userdata` - User data files

Creates custom virtual scheme for conflict diff viewing: `merge-conflict.conflict-diff` (ContentProvider.ts line 10)

### Event Subscription Pattern
Extension uses consistent disposable subscription pattern across:
- `vscode.workspace.onDidOpenTextDocument`
- `vscode.workspace.onDidChangeTextDocument`
- `vscode.window.onDidChangeVisibleTextEditors`
- `vscode.workspace.onDidChangeConfiguration`
All subscriptions added to `context.subscriptions` array for automatic cleanup.

### Telemetry Integration
- Service wrapper (services.ts line 19-24) imports and initializes `TelemetryReporter` from `@vscode/extension-telemetry`
- AI key from extension manifest used for telemetry (package.json line 9)
- DocumentTracker passes telemetry reporter to MergeConflictParser for event tracking

### Configuration Reactivity
Central `createExtensionConfiguration()` method (services.ts lines 55-65) called:
- Once during `begin()` initialization
- Again on each configuration change event
Changes propagated to all services via `configurationUpdated()` callback pattern

## Porting Relevance

### Critical for Tauri/Rust Translation:
1. **CodeLens Provider Interface** - Core IDE extension point; requires Rust equivalent binding
2. **Multi-scheme Document Registration** - Virtual document support layer must handle multiple schemes
3. **TextEditorDecorationType** - Color theming and rendering system tied to VS Code theme colors
4. **Command Registry** - Extensibility point for user-callable actions
5. **Disposable Pattern** - Lifecycle management pattern used throughout (activate/deactivate)
6. **Event Subscription System** - Reactive architecture via workspace/window events
7. **TextDocument/TextEditor Abstractions** - Core document model and editor view layer dependencies
8. **Theme Color Resolution** - Uses vscode.ThemeColor for dynamic styling based on user theme

### Data Model Stability:
- Merge conflict structure is stable and well-isolated
- Parser logic is independent of VS Code APIs (only uses TextLine/TextDocument for input)
- Business logic (CommitType, conflict resolution) is framework-agnostic and translatable
