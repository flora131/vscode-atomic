# File Locations: extensions/references-view/

**Scope**: References / Call-Hierarchy / Type-Hierarchy Tree View Extension  
**Total Files in Scope**: 24 files (1,899 LOC in src/)  
**Purpose**: Demonstrates a non-trivial TreeView-based UI consumer of language-feature APIs (references, call hierarchy, type hierarchy)

---

## Implementation Files

### Core Extension Logic
- `extensions/references-view/src/extension.ts` (31 lines)
  - Extension activation entry point; registers references, calls, and types modules
  - Exports SymbolTree API via `activate()` function

### Tree View Container & Infrastructure
- `extensions/references-view/src/tree.ts` (353 lines)
  - `SymbolsTree` class: Main TreeView container managing state, history, navigation, and highlights
  - `TreeDataProviderDelegate`: Wraps provider promises to handle dynamic provider updates
  - `TreeDndDelegate`: Drag-and-drop controller implementation
  - `TreeInputHistory`: Manages search history with FILO ordering, persists history items
  - Manages context keys: `reference-list.isActive`, `reference-list.hasResult`, `reference-list.source`, `reference-list.hasHistory`

### References (Find References / Implementations)
- `extensions/references-view/src/references/index.ts` (96 lines)
  - Command registration for `references-view.findReferences`, `references-view.findImplementations`
  - Implements `references.preferredLocation` configuration handling (peek vs. view)
  - Registers copy/remove commands for reference items
  - Uses `vscode.executeReferenceProvider` and `vscode.executeImplementationProvider` LSP commands

- `extensions/references-view/src/references/model.ts` (200+ lines)
  - `ReferencesTreeInput`: Implements SymbolTreeInput interface for references queries
  - `ReferencesModel`: Aggregates Location[] or LocationLink[] into FileItem/ReferenceItem tree
  - `FileItem`: Tree node representing a file containing references
  - `ReferenceItem`: Tree node representing a single reference location
  - `ReferencesTreeDataProvider`: Provides tree data to TreeView
  - Implements navigation, editor highlights, and drag-and-drop support

### Call Hierarchy
- `extensions/references-view/src/calls/index.ts` (79 lines)
  - Command registration for `references-view.showCallHierarchy`, `references-view.showOutgoingCalls`, `references-view.showIncomingCalls`
  - `RichCallsDirection`: Manages incoming/outgoing direction state with workspace memento persistence
  - Uses `vscode.prepareCallHierarchy`, `vscode.provideIncomingCalls`, `vscode.provideOutgoingCalls` LSP commands

- `extensions/references-view/src/calls/model.ts` (200+ lines)
  - `CallsTreeInput`: Implements SymbolTreeInput for call hierarchy queries
  - `CallItem`: Tree node representing a function/method in the call graph
  - `CallsModel`: Aggregates CallHierarchyItem[] into navigable tree
  - `CallItemDataProvider`: Provides tree data with lazy-loading of call chains
  - `CallsDirection` enum: Incoming vs. Outgoing

### Type Hierarchy
- `extensions/references-view/src/types/index.ts` (81 lines)
  - Command registration for `references-view.showTypeHierarchy`, `references-view.showSupertypes`, `references-view.showSubtypes`
  - `RichTypesDirection`: Manages supertypes/subtypes direction state with workspace memento
  - Uses `vscode.prepareTypeHierarchy`, `vscode.provideSupertypes`, `vscode.provideSubtypes` LSP commands

- `extensions/references-view/src/types/model.ts` (200+ lines)
  - `TypesTreeInput`: Implements SymbolTreeInput for type hierarchy queries
  - `TypeItem`: Tree node representing a type in the hierarchy
  - `TypesModel`: Aggregates TypeHierarchyItem[] into navigable tree
  - `TypeItemDataProvider`: Provides tree data with lazy-loading of type chains
  - `TypeHierarchyDirection` enum: Supertypes vs. Subtypes

### Support & Utilities
- `extensions/references-view/src/navigation.ts` (85 lines)
  - `Navigation` class: Handles next/previous navigation through results
  - Registers `references-view.next` and `references-view.prev` commands (F4, Shift+F4)
  - Manages `references-view.canNavigate` context key
  - Delegates to SymbolItemNavigation for item position tracking

- `extensions/references-view/src/highlights.ts` (71 lines)
  - `EditorHighlights` class: Manages decorations in active text editor
  - Uses `editor.findMatchHighlightBackground` ThemeColor
  - Updates highlights on selection change, visibility change, text document edits
  - Implements SymbolItemEditorHighlights delegation

- `extensions/references-view/src/utils.ts` (100+ lines)
  - `ContextKey<V>`: Wrapper for context key management (setContext command)
  - `WordAnchor`: Tracks word position changes for history re-running
  - Utility functions: `del()`, `tail()`, `asResourceUrl()`, `isValidRequestPosition()`, `getPreviewChunks()`
  - `getThemeIcon()`: Icon theme color mapping

---

## Types / Interfaces

- `extensions/references-view/src/references-view.d.ts` (159 lines)
  - **`SymbolTree`**: Public API interface with `setInput()` and `getInput()` methods
  - **`SymbolTreeInput<T>`**: Input abstraction with contextValue, title, location, resolve(), with() methods
  - **`SymbolTreeModel<T>`**: Model interface aggregating provider, message, optional navigation/highlights/dnd, dispose()
  - **`SymbolItemNavigation<T>`**: Navigation contract (nearest, next, previous, location)
  - **`SymbolItemEditorHighlights<T>`**: Editor decoration contract (getEditorHighlights)
  - **`SymbolItemDragAndDrop<T>`**: DnD contract (getDragUri)

- `extensions/references-view/src/types/index.ts`
  - Contains type exports from model.ts alongside registration logic

---

## Configuration

### Package Manifest
- `extensions/references-view/package.json` (411 lines)
  - **Extension metadata**: name=references-view, version=10.0.0, publisher=vscode
  - **Engine**: vscode ^1.67.0
  - **Capabilities**: virtualWorkspaces: true, untrustedWorkspaces: supported
  - **Activation events**: `onCommand:references-view.find`, `onCommand:editor.action.showReferences`
  - **Entry points**: 
    - Main (Node.js): `./out/extension`
    - Browser: `./dist/browser/extension`
  - **Contributes**:
    - **Configuration**: `references.preferredLocation` (peek | view)
    - **ViewsContainers**: references-view activity bar
    - **Views**: references-view.tree (shown when reference-list.isActive)
    - **Commands**: 20+ commands for find/clear/navigate/hierarchy operations
    - **Menus**: editor/context (3 items), view/title (6 items), view/item/context (9 items), commandPalette (11 items)
    - **Keybindings**: Shift+Alt+F12 (findReferences), F4/Shift+F4 (navigation), Shift+Alt+H (callHierarchy)
  - **Scripts**: compile, watch, compile-web, bundle-web, typecheck-web, watch-web
  - **DevDependencies**: @types/node 22.x

### Localization
- `extensions/references-view/package.nls.json` (32 lines)
  - Localization strings for all commands, configuration options, and UI labels

### TypeScript Configuration
- `extensions/references-view/tsconfig.json` (18 lines)
  - Extends `../tsconfig.base.json`
  - Outdir: ./out
  - Includes ../../src/vscode-dts/vscode.d.ts

- `extensions/references-view/tsconfig.browser.json` (referenced in package.json)
  - Configuration for browser/web bundling

### Build Configuration
- `extensions/references-view/esbuild.mts`
  - Node.js build configuration
- `extensions/references-view/esbuild.browser.mts`
  - Web/browser bundle configuration
- `extensions/references-view/.npmrc`
  - NPM configuration

### Ignore & Metadata
- `extensions/references-view/.vscodeignore`
  - Excludes files from extension package
- `extensions/references-view/package-lock.json`
  - Dependency lock file

---

## Documentation

- `extensions/references-view/README.md` (35 lines)
  - User-facing documentation describing features:
    - List All References (Alt+Shift+F12)
    - Dedicated tree view sidebar
    - Navigation (F4/Shift+F4)
    - Inline item removal
  - Notes that extension ships bundled with VS Code 1.29+
  - Links to GitHub issue tracker
  - Contribution guidelines and CLA information

---

## Media / Examples

- `extensions/references-view/media/icon.png`
  - Activity bar icon for references view container
- `extensions/references-view/media/demo.png`
  - Screenshot demonstrating the extension UI (linked in README)

---

## Notable Clusters

### Tree View Provider Architecture
**Files**: src/tree.ts, src/references-view.d.ts  
**Pattern**: Implements VS Code's TreeDataProvider pattern with lazy loading and dynamic provider switching.  
Demonstrates how to manage multiple inputs (references, calls, types) through a unified TreeView interface.

### Language Feature Integrations
**Files**: src/references/*, src/calls/*, src/types/*  
**Pattern**: Three independent feature modules following identical structure:
- `index.ts`: Command registration and direction/mode state management
- `model.ts`: Input/Model/Item classes implementing SymbolTreeInput and related interfaces

Each module leverages different LSP command groups:
- References: `vscode.executeReferenceProvider` / `vscode.executeImplementationProvider`
- Call Hierarchy: `vscode.prepareCallHierarchy` / `vscode.provideIncomingCalls` / `vscode.provideOutgoingCalls`
- Type Hierarchy: `vscode.prepareTypeHierarchy` / `vscode.provideSupertypes` / `vscode.provideSubtypes`

### State Management
**Files**: src/tree.ts (TreeInputHistory, ContextKey), src/utils.ts (ContextKey, WordAnchor)  
**Pattern**: Context keys drive UI visibility/commands. WordAnchor tracks symbol positions for history replay.  
Workspace memento persists user preferences (call direction, type direction).

### Editor Integration
**Files**: src/highlights.ts, src/navigation.ts, src/utils.ts  
**Pattern**: Highlights active selections in editor using theme-aware decorations.  
Navigation delegates to SymbolItemNavigation for position tracking.  
Utilities provide range computation and URI/position conversion.

---

## Porting Implications for Rust/Tauri Host

This extension reveals essential APIs a Rust/Tauri host must re-implement:

1. **TreeDataProvider SPI**: Async tree node enumeration with lazy loading, change notification events
2. **TreeDragAndDropController**: Drag-and-drop data transfer protocol
3. **TreeView Creation API**: `createTreeView()` with provider, collapse controls, DnD support
4. **LSP Command Execution**: Execute language server commands synchronously/asynchronously via command names
5. **Editor Decorations**: Create/dispose text decorations with background colors, overview ruler integration
6. **Context Key System**: Global state context for conditional UI visibility and command enablement
7. **Workspace Memento**: Persistent key-value storage per workspace
8. **Configuration API**: Read workspace/user configuration with change notifications
9. **Selection/Location APIs**: Reveal tree items, manage selection, track document positions
10. **Clipboard API**: Read/write to system clipboard

The TreeView-based architecture (vs. peek-based) is fundamentally about decoupling result presentation from query execution, enabling richer interactions (history, navigation, filtering, multi-tab support in a dedicated panel).

