# File Locator: references-view Extension

**Partition:** 30/79 | **Scope:** `extensions/references-view/` (14 files, 1,938 LOC)

## Summary

The `references-view` extension implements a dedicated sidebar panel for VS Code's code navigation features: find all references, implementations, call hierarchies, and type hierarchies. It replaces the peek view with a persistent TreeView in the activity bar, enabling structured exploration of code relationships across the codebase.

---

## Implementation

### Core Extension Architecture

- **`src/extension.ts`** (31 lines)
  - Entry point that activates the extension via `activate()` function
  - Registers three feature modules: references, calls, types
  - Exposes `SymbolTree` API for internal access to tree state management
  - Returns `{ setInput, getInput }` interface for programmatic control

### Tree View & State Management

- **`src/tree.ts`** (353 lines)
  - `SymbolsTree` class: Main tree view controller using `vscode.window.createTreeView()`
  - Creates TreeView with ID `references-view.tree` with drag-and-drop support
  - `TreeInputHistory` class: Maintains search history as FILO (First-In-Last-Out) stack
  - `TreeDataProviderDelegate`: Lazy-loads and wraps tree data providers
  - `TreeDndDelegate`: Handles drag-and-drop (text/uri-list mime type)
  - Context key management: `reference-list.isActive`, `reference-list.hasResult`, `reference-list.source`, `reference-list.hasHistory`
  - History item management with position tracking and word anchoring

### Navigation Feature Module

- **`src/references/index.ts`** (96 lines)
  - Command registration for "Find References" (Shift+Alt+F12) and "Find Implementations"
  - Legacy command alias: `references-view.find` → `references-view.findReferences`
  - Copy operations: copy text, copy all, copy file path to clipboard
  - Configuration listener for `references.preferredLocation` setting (peek vs. view)
  - Command handlers: `removeReferenceItem`, `copy`, `copyAll`, `copyPath`
  - Integrates with LSP command: `vscode.executeReferenceProvider`, `vscode.executeImplementationProvider`

- **`src/references/model.ts`** (partial, 100+ lines)
  - `ReferencesTreeInput`: Implements `SymbolTreeInput<FileItem | ReferenceItem>` interface
  - `ReferencesModel`: Implements tree data provider, navigation, and editor highlights
  - `FileItem`: Grouped file-level container for multiple reference locations
  - `ReferenceItem`: Individual reference location with context preview
  - Location comparison and sorting logic
  - Drag-and-drop URI extraction
  - Copy-text formatting with context previews

### Call Hierarchy Feature Module

- **`src/calls/index.ts`** (79 lines)
  - Command registration: `showCallHierarchy`, `showIncomingCalls`, `showOutgoingCalls`
  - Keyboard shortcut: Shift+Alt+H for call hierarchy
  - `RichCallsDirection` class: Persists direction preference to workspace state
  - Direction context key: `references-view.callHierarchyMode` (showIncoming | showOutgoing)
  - Command: `removeCallItem` for item deletion

- **`src/calls/model.ts`** (partial, 100+ lines)
  - `CallsTreeInput`: Implements `SymbolTreeInput<CallItem>` with direction
  - `CallsModel`: Manages incoming/outgoing call tree state
  - `CallItem`: Tree item wrapping `vscode.CallHierarchyItem`
  - Integrates LSP commands: `vscode.prepareCallHierarchy`, `vscode.provideIncomingCalls`, `vscode.provideOutgoingCalls`
  - Direction enum: `CallsDirection.Incoming | Outgoing`

### Type Hierarchy Feature Module

- **`src/types/index.ts`** (81 lines)
  - Command registration: `showTypeHierarchy`, `showSupertypes`, `showSubtypes`
  - `RichTypesDirection` class: Persists direction preference to workspace state
  - Direction context key: `references-view.typeHierarchyMode` (supertypes | subtypes)
  - Command: `removeTypeItem` for item deletion

- **`src/types/model.ts`** (partial, 100+ lines)
  - `TypesTreeInput`: Implements `SymbolTreeInput<TypeItem>` with direction
  - `TypesModel`: Manages supertype/subtype tree state
  - `TypeItem`: Tree item wrapping `vscode.TypeHierarchyItem`
  - Integrates LSP commands: `vscode.prepareTypeHierarchy`, `vscode.provideSupertypes`, `vscode.provideSubtypes`
  - Direction enum: `TypeHierarchyDirection.Supertypes | Subtypes`

### Navigation & Highlights

- **`src/navigation.ts`** (partial, 80+ lines)
  - `Navigation` class: Implements keyboard navigation (F4, Shift+F4)
  - Commands: `references-view.next`, `references-view.prev`
  - Context key: `references-view.canNavigate`
  - Smart anchor point detection (selected item or nearest to cursor)
  - Integrates with `SymbolItemNavigation<T>` interface for item ordering

- **`src/highlights.ts`** (71 lines)
  - `EditorHighlights<T>` class: Manages inline editor decorations
  - Decoration type: Theme color `editor.findMatchHighlightBackground`
  - Listens to text document changes, editor switches, visibility changes
  - Implements `SymbolItemEditorHighlights<T>` interface for range highlighting
  - Ignore set for changed documents to prevent stale highlights

### Utilities

- **`src/utils.ts`** (partial, 80+ lines)
  - Helper functions: `del()`, `tail()`, `asResourceUrl()`
  - `isValidRequestPosition()`: Validates cursor position has a word
  - `getPreviewChunks()`: Extracts context (before/inside/after) text for display
  - `ContextKey<V>` class: Wraps VS Code context key API
  - `WordAnchor` class: Tracks symbol position across document edits using word content and position heuristics

### API Surface

- **`src/references-view.d.ts`** (159 lines)
  - `SymbolTree` interface: Main public API with `setInput()` and `getInput()` methods
  - `SymbolTreeInput<T>` interface: Input specification with location, title, context value, resolve method, and `with()` cloning
  - `SymbolTreeModel<T>` interface: Output model with provider, message, navigation, highlights, dnd, and dispose
  - `SymbolItemNavigation<T>` interface: Item ordering (next/previous) and location lookup
  - `SymbolItemEditorHighlights<T>` interface: Range extraction for editor highlighting
  - `SymbolItemDragAndDrop<T>` interface: URI extraction for drag-and-drop

---

## Configuration

### Package Manifest

- **`package.json`** (411 lines)
  - Extension entry points: `main: ./out/extension`, `browser: ./dist/browser/extension`
  - Activation events: `onCommand:references-view.find`, `onCommand:editor.action.showReferences`
  - Contributes 17 commands: find/show operations, clear, refresh, navigation, removal, copy
  - Menu contexts: editor context menu (top 4 nav items), view title, view item context
  - Keybindings: Shift+Alt+F12 (refs), Shift+Alt+H (calls), F4/Shift+F4 (nav)
  - Configuration: `references.preferredLocation` (peek | view)
  - View container: activity bar with references icon, context guard `reference-list.isActive`
  - Localization strings: `package.nls.json` with titles, descriptions, categories

### TypeScript Configuration

- **`tsconfig.json`** (18 lines)
  - Extends `tsconfig.base.json`
  - `rootDir: ./src`, `outDir: ./out`
  - Includes vscode.d.ts types from `../../src/vscode-dts/vscode.d.ts`

- **`tsconfig.browser.json`** (similar structure)
  - Browser-specific compilation for web worker version

### Build Configuration

- **`esbuild.mts`** (Node.js build)
  - ESBuild bundler config for native extension
  - Output: `./out/extension.js`

- **`esbuild.browser.mts`** (Web worker build)
  - ESBuild bundler config for browser version
  - Output: `./dist/browser/extension.js`
  - Web worker environment setup

---

## Documentation

- **`README.md`** (35 lines)
  - Feature overview: dedicated tree view for reference search results
  - Use cases: List All References, view in sidebar, keyboard navigation
  - Screenshot demo reference
  - Notes on bundling with VS Code 1.29+
  - Contribution guidelines with CLA process

---

## Notable Clusters

### Feature Modules (3 parallel implementations)
- **References Module** (`src/references/`): 2 files
  - Implements "Find All References" and "Find Implementations"
  - Uses `vscode.executeReferenceProvider` and `vscode.executeImplementationProvider`

- **Calls Module** (`src/calls/`): 2 files
  - Implements "Call Hierarchy" with incoming/outgoing direction
  - Uses `vscode.CallHierarchyItem` LSP extension

- **Types Module** (`src/types/`): 2 files
  - Implements "Type Hierarchy" with supertype/subtype direction
  - Uses `vscode.TypeHierarchyItem` LSP extension

All three modules follow identical architectural pattern:
1. `index.ts`: Command registration + direction/state management
2. `model.ts`: Tree input, model, item classes implementing required interfaces

### TreeView API Integration Points
- Single `vscode.window.createTreeView()` call at line 31 of `src/tree.ts`
- Delegates provider updates through `TreeDataProviderDelegate` wrapper
- Lazy-loads model data with promise-based provider swapping

### Context Key Usage (7 keys)
- `reference-list.isActive` (boolean): Panel visibility state
- `reference-list.hasResult` (boolean): Data presence for UI guards
- `reference-list.hasHistory` (boolean): History availability
- `reference-list.source` (string): Current feature (contextValue)
- `references-view.canNavigate` (boolean): Navigation availability
- `references-view.callHierarchyMode` (enum): Incoming/Outgoing
- `references-view.typeHierarchyMode` (enum): Supertypes/Subtypes

### LSP Command Integration (10 commands)
- Reference providers: `vscode.executeReferenceProvider`, `vscode.executeImplementationProvider`
- Call hierarchy: `vscode.prepareCallHierarchy`, `vscode.provideIncomingCalls`, `vscode.provideOutgoingCalls`
- Type hierarchy: `vscode.prepareTypeHierarchy`, `vscode.provideSupertypes`, `vscode.provideSubtypes`
- History/location: `vscode.open` for navigation
- Context: `setContext` for enabling/disabling UI

---

## Architectural Insights for Tauri/Rust Port

### TreeView Pattern
The extension uses VS Code's TreeView API as a reusable abstraction for hierarchical data presentation. The `SymbolTree` interface and `SymbolTreeInput<T>` / `SymbolTreeModel<T>` contracts define the public API for populating tree views. **Porting strategy:** Create a parallel Rust-based TreeView component system with trait-based provider delegation (Rust traits for `TreeDataProvider<T>`, `SymbolItemNavigation<T>`, etc.).

### Feature Modularization
Three distinct code navigation features (references, calls, types) share identical architectural scaffolding (input class, model class, item class, command handlers). This suggests a reusable "symbol tree feature framework" could accelerate porting. **Pattern:** Trait-based feature registration with consistent activation/deactivation hooks.

### LSP Language Intelligence
The extension is purely a UI layer wrapping LSP provider commands. All language-specific logic (finding references, resolving hierarchies) is delegated to language servers via standard VS Code command execution. **Porting implication:** Tauri version depends on LSP client integration—the navigation UI itself is language-agnostic.

### History & Position Tracking
The `WordAnchor` class implements position tracking across document edits by storing surrounding context and word content, enabling "fuzzy" position recovery when re-running searches after edits. This is a sophisticated UX pattern worth preserving. **Rust equivalent:** Document diff tracking with word-context heuristics.

### Clipboard & Drag-Drop Integration
Copy-to-clipboard and drag-and-drop operations require OS integration. The extension uses VS Code APIs (`vscode.env.clipboard`, TreeView drag-and-drop controller). **Port requirements:** System clipboard integration (e.g., `arboard` crate) and file URL handling for drag sources.

