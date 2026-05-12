# TreeView API: References/Calls/Types Navigation (extensions/references-view/)

## Scope Summary
14 files, ~1,938 LOC implementing the References View extension—a public TreeView-based API for displaying code navigation results (references, call hierarchy, type hierarchy) as a dedicated sidebar panel in VS Code.

## Implementation

### Core Extension Architecture
- `src/extension.ts` - Extension activation entry point; instantiates SymbolsTree and registers references/calls/types modules
- `src/tree.ts` - SymbolsTree class: manages TreeView lifecycle, input state, history tracking, context keys, and integration with providers and drag-and-drop

### TreeView API Contract (Interfaces)
- `src/references-view.d.ts` - Defines public extension API:
  - `SymbolTree`: Public interface for external consumers (setInput, getInput)
  - `SymbolTreeInput<T>`: Entry point abstraction (contextValue, title, location, resolve, with)
  - `SymbolTreeModel<T>`: Model contract (provider, message, navigation, highlights, dnd, dispose)
  - `SymbolItemNavigation<T>`: Navigation interface (nearest, next, previous, location)
  - `SymbolItemEditorHighlights<T>`: Editor highlight interface (getEditorHighlights)
  - `SymbolItemDragAndDrop<T>`: Drag-and-drop interface (getDragUri)

### References Navigation Feature
- `src/references/index.ts` - Command registration:
  - findReferences, findImplementations commands
  - Legacy find command mapping
  - Copy/copyAll/copyPath context menu commands
  - references.preferredLocation configuration support (peek vs view)
  - editor.action.showReferences override handling
- `src/references/model.ts` - Data models:
  - ReferencesTreeInput: Input abstraction for references queries
  - ReferencesModel: Implements SymbolItemNavigation, SymbolItemEditorHighlights, SymbolItemDragAndDrop
  - FileItem: Groups references by file
  - ReferenceItem: Individual reference location
  - ReferencesTreeDataProvider: TreeDataProvider implementation

### Call Hierarchy Feature
- `src/calls/index.ts` - Command registration:
  - showCallHierarchy, showOutgoingCalls, showIncomingCalls commands
  - removeCallItem context menu command
  - RichCallsDirection: Persists and manages incoming/outgoing mode via workspaceState
  - Direction-aware context key management
- `src/calls/model.ts` - Data models:
  - CallsTreeInput: Input abstraction for call hierarchy queries
  - CallsDirection: Enum (Incoming=0, Outgoing=1)
  - CallItem: Tree node representing a call hierarchy item
  - CallsModel: Implements SymbolItemNavigation, SymbolItemEditorHighlights, SymbolItemDragAndDrop
  - CallItemDataProvider: TreeDataProvider with lazy-load children resolution

### Type Hierarchy Feature
- `src/types/index.ts` - Command registration:
  - showTypeHierarchy, showSupertypes, showSubtypes commands
  - removeTypeItem context menu command
  - RichTypesDirection: Persists and manages supertypes/subtypes mode via workspaceState
  - Direction-aware context key management
- `src/types/model.ts` - Data models:
  - TypesTreeInput: Input abstraction for type hierarchy queries
  - TypeHierarchyDirection: Enum (Supertypes='supertypes', Subtypes='subtypes')
  - TypeItem: Tree node representing a type hierarchy item
  - TypesModel: Implements SymbolItemNavigation, SymbolItemEditorHighlights, SymbolItemDragAndDrop
  - TypeItemDataProvider: TreeDataProvider with lazy-load children resolution

### Supporting Components
- `src/navigation.ts` - Navigation class:
  - Wraps SymbolItemNavigation interface
  - Implements next/previous commands
  - Anchors navigation to current selection or nearest symbol in active editor
  - Opens location in editor on navigation
  - Manages context key for command enablement
- `src/highlights.ts` - EditorHighlights class:
  - Creates text editor decorations using find-match highlight theme colors
  - Updates decorations on selection/visibility changes
  - Wraps SymbolItemEditorHighlights interface

### Utilities
- `src/utils.ts` - Helper functions and classes:
  - del: Array element removal
  - tail: Get last array element
  - asResourceUrl: Convert location to URI with line/column fragment
  - isValidRequestPosition: Validate request location in document
  - getPreviewChunks: Extract before/inside/after text for display
  - ContextKey: Wrapper for setContext command execution
  - WordAnchor: Tracks symbols across document edits (fuzzy position recovery)
  - getThemeIcon: Maps SymbolKind to ThemeIcon with semantic coloring

## Types / Interfaces

All critical public interfaces defined in `src/references-view.d.ts`:
- SymbolTree: External API contract (2 methods)
- SymbolTreeInput<T>: Generic input template (5 members)
- SymbolTreeModel<T>: Model contract (6 members: provider, message, navigation, highlights, dnd, dispose)
- SymbolItemNavigation<T>: Navigation behavior (4 methods: nearest, next, previous, location)
- SymbolItemEditorHighlights<T>: Editor decoration interface (1 method: getEditorHighlights)
- SymbolItemDragAndDrop<T>: Drag-drop support (1 method: getDragUri)

Enums in model files:
- CallsDirection: 2 enum variants (Incoming, Outgoing as numbers)
- TypeHierarchyDirection: 2 enum variants (Supertypes, Subtypes as strings)

## Configuration

Package-level configuration in `package.json`:
- references.preferredLocation: Enum (peek | view) - Controls default presentation mode for reference searches
- Contributes:
  - ViewsContainer: activitybar with "references-view" container
  - Views: Single tree view "references-view.tree" with when clause "reference-list.isActive"
  - Commands: 22 commands total (findReferences, findImplementations, showCallHierarchy, showTypeHierarchy, clear, refresh, copy variants, navigate, etc.)
  - Menus: Context menu integration in editor, view title, view item context (copy, remove, navigate)
  - Keybindings: Shift+Alt+F12 (find refs), F4/Shift+F4 (navigate), Shift+Alt+H (call hierarchy)
  - Context keys: reference-list.isActive, reference-list.hasResult, reference-list.source, references-view.callHierarchyMode, references-view.typeHierarchyMode, references-view.canNavigate

TypeScript configuration (`tsconfig.json`):
- Extends tsconfig.base.json from parent directory
- Output to ./out
- Includes vscode.d.ts type definitions

Build configuration (`esbuild.mts`, `esbuild.browser.mts`):
- ESBuild configuration for bundling
- Separate browser bundle target

## Documentation

- `README.md` - User-facing documentation:
  - Feature overview (references list view, keyboard navigation, inline removal)
  - Requirements (references providers must be installed separately)
  - Issue tracking via VS Code repository
  - CLA and Open Source Code of Conduct

Package metadata (`package.nls.json`):
- Localization strings for displayName, description, config properties, commands, menus

## Notable Clusters

### TreeView Integration Pattern
All three features (references, calls, types) follow identical composition:
1. SymbolTreeInput subclass (ReferencesTreeInput, CallsTreeInput, TypesTreeInput) - entry point
2. Model class implementing navigation/highlights/dnd interfaces (ReferencesModel, CallsModel, TypesModel)
3. TreeDataProvider wrapping the model (ReferencesTreeDataProvider, CallItemDataProvider, TypeItemDataProvider)
4. Command registration for user interaction (references/index.ts, calls/index.ts, types/index.ts)

### State Persistence
- `src/calls/index.ts` and `src/types/index.ts` implement RichCallsDirection and RichTypesDirection
  - Store direction preference in workspaceState (vscode.Memento)
  - Sync with context keys for menu visibility
  - Survive editor sessions

### Language Provider Integration
- References: Uses vscode.executeReferenceProvider, vscode.executeImplementationProvider
- Calls: Uses vscode.prepareCallHierarchy, vscode.provideIncomingCalls, vscode.provideOutgoingCalls
- Types: Uses vscode.prepareTypeHierarchy, vscode.provideSupertypes, vscode.provideSubtypes
- All delegated via vscode.commands.executeCommand (async)

### Editor Highlighting
- `src/highlights.ts` EditorHighlights class applies visual decorations
- Theme color: editor.findMatchHighlightBackground
- Syncs with selection changes and view visibility
- Per-feature: injected into each model's SymbolTreeModel contract

### Navigation & Selection Management
- `src/navigation.ts` enables F4/Shift+F4 command shortcuts
- Anchors to either current tree selection or nearest symbol to cursor
- Opens results in editor with preserved focus option
- Supports multi-file navigation across result set

### Drag & Drop Support
- SymbolItemDragAndDrop interface provides getDragUri per item
- ReferencesModel returns file URIs with line-column fragments
- Enables dragging references into editor as resource URIs

---

## Porting Requirements Summary

To port this functionality to a Tauri/Rust core, a new core must expose:

1. **TreeView Public API** - Equivalent to vscode.window.createTreeView with:
   - Generic TreeDataProvider pattern
   - Tree view visibility/focus management
   - Selection/reveal operations
   - Drag-and-drop controller binding

2. **Command Infrastructure** - For:
   - Command registration with arbitrary handler functions
   - Command execution (vscode.commands.executeCommand) with type safety
   - Context key management (setContext command)

3. **Editor State & Language Providers** - Access to:
   - Active editor and document content
   - Language-specific providers (reference, implementation, call hierarchy, type hierarchy)
   - Position/range validation utilities

4. **Text Decoration System** - Support for:
   - Creating text editor decoration types
   - Setting decorations by range
   - Theme color references

5. **Configuration & State** - Mechanism for:
   - Reading workspace configuration (vscode.workspace.getConfiguration)
   - WorkspaceState persistence (vscode.Memento equivalent)
   - Context key state (reference-list.isActive, reference-list.hasResult, etc.)

6. **Async/Promise Runtime** - For:
   - Async/await language semantics
   - Promise-based provider callbacks
   - Event emitter pattern (onDidChange, onDidChangeSelection, etc.)

7. **Localization Support** - For:
   - Runtime text translation (vscode.l10n.t)
   - Multi-language message strings

8. **Context Menu Integration** - For:
   - Editor context menu injection
   - View title/item context menu injection
   - Command visibility with when-clauses

