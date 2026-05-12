# Partition 30 of 80 — Findings

## Scope
`extensions/references-view/` (14 files, 1,938 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

1. `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/extension.ts`
2. `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/references-view.d.ts`
3. `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/tree.ts`
4. `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/navigation.ts`
5. `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/highlights.ts`
6. `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/utils.ts`
7. `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/references/index.ts`
8. `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/references/model.ts`
9. `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/calls/index.ts`
10. `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/calls/model.ts`
11. `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/types/index.ts`
12. `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/types/model.ts`

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/extension.ts`

- **Role:** Extension entry point. Instantiates the single `SymbolsTree` and registers the three feature domains (references, calls, types). Returns a `SymbolTree` public API object.
- **Key symbols:**
  - `activate` (line 13) — exported activation function
  - `SymbolsTree` instantiation (line 15)
  - `references.register`, `calls.register`, `types.register` (lines 17–19)
  - `setInput` / `getInput` closure (lines 21–29) — forms the public `SymbolTree` API
- **Control flow:** `activate` creates one `SymbolsTree`, passes it to each feature registrar, then returns `{ setInput, getInput }` delegating directly to the tree instance.
- **Data flow:** The returned object exposes `tree.setInput` and `tree.getInput` to external callers, allowing any VS Code extension to inject a `SymbolTreeInput` into the view.
- **Dependencies:** `vscode`, `./calls`, `./references`, `./types`, `./references-view`, `./tree`

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/references-view.d.ts`

- **Role:** Public API contract for the entire extension. Defines the TypeScript interfaces that external extensions and internal modules program against.
- **Key symbols:**
  - `SymbolTree` (line 24) — top-level API with `setInput` / `getInput`
  - `SymbolTreeInput<T>` (line 46) — input contract: `contextValue`, `title`, `location`, `resolve()`, `with(location)`
  - `SymbolTreeModel<T>` (line 86) — model produced by `resolve()`: `provider`, `message`, optional `navigation`, `highlights`, `dnd`, `dispose`
  - `SymbolItemNavigation<T>` (line 126) — `nearest`, `next`, `previous`, `location`
  - `SymbolItemEditorHighlights<T>` (line 148) — `getEditorHighlights`
  - `SymbolItemDragAndDrop<T>` (line 155) — `getDragUri`
- **Control flow:** Pure interface file; no runtime logic.
- **Data flow:** `SymbolTreeInput.resolve()` returns `ProviderResult<SymbolTreeModel<T>>`, which bundles a `vscode.TreeDataProvider`, optional navigation/highlights/dnd delegates, and a dispose callback.
- **Dependencies:** `vscode` only.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/tree.ts`

- **Role:** Central controller. Owns the `vscode.TreeView`, the delegate `TreeDataProvider`, DnD controller, history, and navigation. Implements `SymbolsTree`.
- **Key symbols:**
  - `SymbolsTree` class (line 13)
  - Context keys: `_ctxIsActive` (`reference-list.isActive`, line 17), `_ctxHasResult` (`reference-list.hasResult`, line 18), `_ctxInputSource` (`reference-list.source`, line 19)
  - `_history: TreeInputHistory` (line 21)
  - `_provider: TreeDataProviderDelegate` (line 22) — async-wrapping delegate
  - `_dnd: TreeDndDelegate` (line 23)
  - `_tree: vscode.TreeView<unknown>` (line 24) — created at line 31 with `viewId = 'references-view.tree'`
  - `_navigation: Navigation` (line 25)
  - `setInput(input)` (line 49) — primary state-change method
  - `clearInput()` (line 119)
  - `TreeDataProviderDelegate` (line 138) — wraps a promise-returning provider; proxies `getTreeItem`, `getChildren`, `getParent`
  - `TreeDndDelegate` (line 191) — exposes `dragMimeTypes: ['text/uri-list']`; on drag builds URI list
  - `TreeInputHistory` (line 240) — implements `vscode.TreeDataProvider<HistoryItem>`, FILO map keyed by serialized `[position, uri, title]`; registers `references-view.clear`, `.clearHistory`, `.refind`, `.refresh`, `.pickFromHistory`, `_references-view.showHistoryItem`
  - `HistoryItem` (line 226) — wraps a `SymbolTreeInput`, a `WordAnchor`, and display strings
- **Control flow:**
  1. `setInput` validates position via `isValidRequestPosition`, sets context keys, focuses the view.
  2. Calls `input.resolve()` as a promise, passes the promise directly to `_provider.update` so the tree shows a loading indicator immediately.
  3. Awaits the model; if stale (another input set meanwhile) returns early.
  4. Calls `_history.add(input)`, sets `_tree.message`, calls `_navigation.update(model.navigation)`.
  5. Computes nearest item, reveals/selects it in the tree.
  6. Creates `EditorHighlights` if `model.highlights` present; subscribes to `onDidChangeTreeData`.
  7. All session disposables collected in `_sessionDisposable`.
- **Data flow:** `SymbolTreeInput` → `resolve()` → `SymbolTreeModel` → `TreeDataProviderDelegate` receives model's `.provider`; `Navigation` receives model's `.navigation`; `EditorHighlights` receives model's `.highlights`.
- **Dependencies:** `vscode`, `./highlights`, `./navigation`, `./references-view`, `./utils`

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/navigation.ts`

- **Role:** Implements keyboard navigation (next/previous) across tree items using the active `SymbolItemNavigation<T>` delegate.
- **Key symbols:**
  - `Navigation` class (line 10)
  - `_ctxCanNavigate` context key `'references-view.canNavigate'` (line 13)
  - Commands registered: `references-view.next` (line 19), `references-view.prev` (line 20)
  - `update(delegate)` (line 28) — swaps the delegate and sets context key
  - `_anchor()` (line 33) — resolves current item from tree selection or `nearest()` on active editor position
  - `previous(preserveFocus)` (line 54) / `next(preserveFocus)` (line 70) — calls delegate, reveals item in tree, opens location in editor
- **Control flow:** `next`/`previous` both call `_anchor()`, pass the resolved item through the delegate's `next`/`previous`, get the location via `delegate.location(newItem)`, call `this._view.reveal(newItem)`, then `vscode.commands.executeCommand('vscode.open', ...)`.
- **Data flow:** Tree selection → delegate item → delegate computes next/prev → `vscode.Location` → editor navigation.
- **Dependencies:** `vscode`, `./references-view`, `./utils`

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/highlights.ts`

- **Role:** Manages editor decorations that highlight symbol ranges in the active editor whenever the tree selection changes.
- **Key symbols:**
  - `EditorHighlights<T>` class (line 9)
  - `_decorationType` (line 11) — `editor.findMatchHighlightBackground` with `OverviewRulerLane.Center`
  - `_ignore: Set<string>` (line 19) — URIs of modified documents (avoids highlighting stale ranges)
  - Constructor (line 21) subscribes to: `onDidChangeTextDocument`, `onDidChangeActiveTextEditor`, `_view.onDidChangeVisibility`, `_view.onDidChangeSelection`
  - `_show()` (line 42) — gets selection anchor, calls `_delegate.getEditorHighlights(anchor, uri)`, calls `editor.setDecorations`
  - `_hide()` (line 60) — clears decorations on all visible editors
  - `update()` (line 66) — `_hide()` then `_show()`
- **Control flow:** On selection change or active editor change → `update()` → clears all decorations → re-queries highlights from delegate for current anchor and URI → applies decoration ranges to active editor.
- **Data flow:** Tree selection item + active editor URI → `SymbolItemEditorHighlights.getEditorHighlights()` returns `vscode.Range[]` → `editor.setDecorations`.
- **Dependencies:** `vscode`, `./references-view`

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/utils.ts`

- **Role:** Shared utility functions and classes used throughout the extension.
- **Key symbols:**
  - `del<T>(array, e)` (line 8) — splice-remove from array
  - `tail<T>(array)` (line 15) — last element
  - `asResourceUrl(uri, range)` (line 19) — builds fragment URI `L{line},{col}-{line},{col}` for DnD
  - `isValidRequestPosition(uri, position)` (line 23) — opens document, checks word range exists at position
  - `getPreviewChunks(doc, range, beforeLen, trim)` (line 32) — extracts `{before, inside, after}` text segments; `previewEnd = range.end.translate(0, 331)` for the "after" window
  - `ContextKey<V>` (line 46) — thin wrapper calling `vscode.commands.executeCommand('setContext', name, value)`
  - `WordAnchor` (line 59) — records document version and word text at construction; `guessedTrackedPosition()` (line 74) scans ±100 lines for the original word text in the edited document
  - `getThemeIcon(kind)` (line 141) — maps `vscode.SymbolKind` enum index to `symbol-*` ThemeIcon ids with `symbolIcon.*Foreground` colors
- **Control flow:** All functions are pure utilities or simple stateful helpers, no event loops.
- **Data flow:** `WordAnchor` captures snapshot at construction; `guessedTrackedPosition` does a spiral line scan outward from the original line.
- **Dependencies:** `vscode` only.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/references/index.ts`

- **Role:** Registers all references/implementations-related commands and the `references.preferredLocation` configuration hook.
- **Key symbols:**
  - `register(tree, context)` (line 10)
  - `findLocations(title, command)` (line 12) — creates `ReferencesTreeInput` from active editor and calls `tree.setInput`
  - Commands: `references-view.findReferences` (line 20), `references-view.findImplementations` (line 21), legacy `references-view.find` (line 23), `references-view.removeReferenceItem` (line 24), `references-view.copy` (line 25), `references-view.copyAll` (line 26), `references-view.copyPath` (line 27)
  - `updateShowReferences()` (line 35) — when config `references.preferredLocation === 'view'`, registers `editor.action.showReferences` to redirect the inline references peek into the tree view
  - `removeReferenceItem` (line 64), `copyCommand` (line 73), `copyPathCommand` (line 87)
- **Control flow:** Configuration listener fires `updateShowReferences` on every config change; the function disposes and re-registers the `editor.action.showReferences` override based on the config value.
- **Data flow:** Active editor position → `ReferencesTreeInput` (carrying LSP command name) → `tree.setInput`.
- **Dependencies:** `vscode`, `../tree`, `./model`

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/references/model.ts`

- **Role:** Data model, tree data provider, and navigation/highlights/DnD adapters for the References feature.
- **Key symbols:**
  - `ReferencesTreeInput` (line 10) — `SymbolTreeInput<FileItem | ReferenceItem>`; `contextValue = _command` (LSP command name)
  - `ReferencesTreeInput.resolve()` (line 23) — executes LSP command (`vscode.executeReferenceProvider` or `vscode.executeImplementationProvider`) via `vscode.commands.executeCommand`, builds `ReferencesModel`, creates `ReferencesTreeDataProvider`
  - `ReferencesModel` (line 55) — implements `SymbolItemNavigation`, `SymbolItemEditorHighlights`, `SymbolItemDragAndDrop`; holds `items: FileItem[]` sorted and grouped by URI
  - Constructor (line 62) — sorts locations by `_compareLocations`, groups into `FileItem[]` using `_compareUriIgnoreFragment`
  - `nearest(uri, position)` (line 133) — three-tier: exact range containment, first after position, longest URI prefix match
  - `next` / `previous` (lines 185–220) — circular navigation across all `FileItem[]` / `ReferenceItem[]` using modular index arithmetic
  - `getEditorHighlights` (line 222) — returns all reference ranges for matching file
  - `remove(item)` (line 227) — fires `onDidChangeTreeData` with `undefined` (full refresh) or `item.file` (subtree refresh)
  - `ReferencesTreeDataProvider` (line 259) — `vscode.TreeDataProvider<FileItem | ReferenceItem>`; `getTreeItem` for `FileItem` uses `TreeItem(element.uri)` with `ThemeIcon.File`; for `ReferenceItem` extracts `getPreviewChunks` and creates highlighted label
  - `FileItem` (line 326), `ReferenceItem` (line 349) — leaf data classes; `ReferenceItem.getDocument` caches the `openTextDocument` promise and eagerly loads the next document
- **Control flow:** `resolve()` → LSP call → `ReferencesModel(locations)` sorts+groups → `ReferencesTreeDataProvider(model)` returned in `SymbolTreeModel`.
- **Data flow:** `vscode.Location[] | vscode.LocationLink[]` → sorted → grouped by URI → `FileItem[]` tree → `TreeDataProvider` renders in TreeView.
- **Dependencies:** `vscode`, `../references-view`, `../utils`

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/calls/index.ts`

- **Role:** Registers call hierarchy commands and manages the persistent direction state (incoming vs. outgoing).
- **Key symbols:**
  - `register(tree, context)` (line 11)
  - `RichCallsDirection` (line 51) — persists direction to `workspaceState` under key `references-view.callHierarchyMode`; sets context key `references-view.callHierarchyMode` to `'showIncoming'` or `'showOutgoing'`
  - Commands: `references-view.showCallHierarchy` (line 38), `references-view.showOutgoingCalls` (line 39), `references-view.showIncomingCalls` (line 40), `references-view.removeCallItem` (line 41)
  - `setCallsDirection(value, anchor)` (line 22) — allows re-anchoring on a specific `CallItem` or reusing old input location
- **Control flow:** User invokes a command → direction state updated → new `CallsTreeInput` constructed → `tree.setInput` called.
- **Data flow:** Active editor position or existing `CallItem` → `CallsTreeInput(location, direction)` → `tree.setInput`.
- **Dependencies:** `vscode`, `../tree`, `../utils`, `./model`

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/calls/model.ts`

- **Role:** Data model, tree data provider, and navigation/highlights/DnD adapters for Call Hierarchy.
- **Key symbols:**
  - `CallsTreeInput` (line 10) — `contextValue = 'callHierarchy'`
  - `CallsTreeInput.resolve()` (line 24) — calls `vscode.prepareCallHierarchy` to get root `CallHierarchyItem[]`; builds `CallsModel` and `CallItemDataProvider`
  - `CallsDirection` enum (line 52) — `Incoming = 0`, `Outgoing = 1`
  - `CallItem` (line 59) — wraps `vscode.CallHierarchyItem`, `parent`, optional pre-fetched `locations: vscode.Location[]`, lazy `children?: CallItem[]`
  - `CallsModel` (line 75) — implements navigation/highlights/DnD
  - `_resolveCalls(call)` (line 86) — executes `vscode.provideIncomingCalls` or `vscode.provideOutgoingCalls`; maps results to `CallItem[]` with fromRanges as locations
  - `getCallChildren(call)` (line 96) — lazy: caches in `call.children`
  - `getEditorHighlights(item, uri)` (line 141) — for items with explicit locations, filters by URI; for root items, returns `selectionRange`
  - `CallItemDataProvider` (line 160) — `getTreeItem` renders name, detail, symbol kind icon; for outgoing opens at `selectionRange.start`; for incoming finds first call location in the URI
  - `getChildren` (line 216) — root returns `_model.roots`; node returns `_model.getCallChildren(element)`
- **Control flow:** `resolve()` → prepare call hierarchy → lazy tree expansion triggers `getCallChildren` → executes LSP calls on demand.
- **Data flow:** `vscode.CallHierarchyItem[]` (roots) → `CallItem[]` → lazy expansion via `vscode.provideIncomingCalls` / `vscode.provideOutgoingCalls` → child `CallItem[]`.
- **Dependencies:** `vscode`, `../references-view`, `../utils`

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/types/index.ts`

- **Role:** Registers type hierarchy commands and manages direction state (supertypes vs. subtypes).
- **Key symbols:**
  - `register(tree, context)` (line 11)
  - `RichTypesDirection` (line 53) — persists to `workspaceState` under `references-view.typeHierarchyMode`; context key `references-view.typeHierarchyMode` set to `TypeHierarchyDirection` string value
  - Commands: `references-view.showTypeHierarchy` (line 40), `references-view.showSupertypes` (line 41), `references-view.showSubtypes` (line 42), `references-view.removeTypeItem` (line 43)
  - `setTypeHierarchyDirection` (line 22) — accepts `TypeItem | vscode.Location | unknown` as anchor, constructing new `TypesTreeInput` accordingly
- **Control flow:** Analogous to calls/index.ts; direction persisted and restored; anchor may be a `TypeItem`, raw `Location`, or falls back to current input's location.
- **Data flow:** Active editor or `TypeItem` → `TypesTreeInput(location, direction)` → `tree.setInput`.
- **Dependencies:** `vscode`, `../tree`, `../utils`, `./model`

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/types/model.ts`

- **Role:** Data model, tree data provider, and navigation/highlights/DnD adapters for Type Hierarchy.
- **Key symbols:**
  - `TypesTreeInput` (line 10) — `contextValue = 'typeHierarchy'`
  - `TypesTreeInput.resolve()` (line 24) — calls `vscode.prepareTypeHierarchy`; builds `TypesModel` and `TypeItemDataProvider`
  - `TypeHierarchyDirection` enum (line 52) — `Subtypes = 'subtypes'`, `Supertypes = 'supertypes'`
  - `TypeItem` (line 58) — wraps `vscode.TypeHierarchyItem`, `parent`, lazy `children`
  - `TypesModel` (line 73) — `_resolveTypes` calls `vscode.provideSupertypes` or `vscode.provideSubtypes` lazily
  - `getEditorHighlights` (line 139) — returns `selectionRange` when URI matches
  - `TypeItemDataProvider` (line 153) — `getTreeItem` renders name, detail, kind icon, opens at `selectionRange.start`; all nodes start `Collapsed`
- **Control flow:** Structurally identical to `calls/model.ts`; lazy child expansion via `getTypeChildren`.
- **Data flow:** `vscode.TypeHierarchyItem[]` → `TypeItem[]` → lazy sub/supertypes via LSP commands → child `TypeItem[]`.
- **Dependencies:** `vscode`, `../references-view`, `../utils`

---

### Cross-Cutting Synthesis

The `references-view` extension is a **strategy-pattern TreeView consumer**. A single `SymbolsTree` instance (`tree.ts:13`) owns one `vscode.TreeView` registered under the view ID `'references-view.tree'`. The tree never contains domain logic; it receives a `SymbolTreeInput<T>` object, calls its `resolve()` method, and delegates all rendering, navigation, highlighting, and DnD to the returned `SymbolTreeModel<T>` interfaces.

Three feature domains — references, call hierarchy, and type hierarchy — each contribute their own `*TreeInput` / `*Model` / `*DataProvider` triplet that plugs into this single slot. All three follow identical structure: `*TreeInput.resolve()` fires a VS Code built-in LSP command (`vscode.executeReferenceProvider`, `vscode.prepareCallHierarchy`, `vscode.prepareTypeHierarchy`), wraps the result in a model class, and returns a `SymbolTreeModel` bundle.

**Context keys** are the primary mechanism for menu/command visibility gating. Five keys are managed:
- `reference-list.isActive` / `reference-list.hasResult` / `reference-list.source` on `SymbolsTree` (`tree.ts:17-19`)
- `reference-list.hasHistory` on `TreeInputHistory` (`tree.ts:246`)
- `references-view.canNavigate` on `Navigation` (`navigation.ts:13`)
- `references-view.callHierarchyMode` on `RichCallsDirection` (`calls/index.ts:55`)
- `references-view.typeHierarchyMode` on `RichTypesDirection` (`types/index.ts:57`)

`TreeDataProviderDelegate` (`tree.ts:138`) is a promise-wrapping proxy: it immediately fires a `onDidChangeTreeData` event when `update()` is called (triggering tree loading UI) and forwards all `getTreeItem` / `getChildren` / `getParent` calls to the awaited underlying provider. This decouples the async `resolve()` cycle from the synchronous TreeView API contract.

`WordAnchor` (`utils.ts:59`) implements position tracking across edits by recording the word text at the original position and performing a ±100-line spiral scan on re-run. `TreeInputHistory` uses this to reconstruct a valid `SymbolTreeInput.with(location)` when re-running past queries.

For a Tauri/Rust port, this partition's surface area consists of: (1) the TreeView widget and its provider/DnD/selection/reveal/message/title APIs; (2) five context-key setters; (3) six built-in LSP proxy commands that must be served by the language server layer; (4) text decoration/overview-ruler APIs used by `EditorHighlights`; and (5) `workspace.openTextDocument`, `window.activeTextEditor`, and `workspace.asRelativePath` utilities.

---

### Out-of-Partition References

- `vscode.commands.executeCommand('vscode.executeReferenceProvider', ...)` — built-in command in `src/vs/workbench/api/browser/mainThreadLanguageFeatures.ts` (main extension host, out of partition)
- `vscode.commands.executeCommand('vscode.executeImplementationProvider', ...)` — same layer
- `vscode.commands.executeCommand('vscode.prepareCallHierarchy', ...)` — main thread call hierarchy provider, `src/vs/workbench/api/common/extHostLanguageFeatures.ts`
- `vscode.commands.executeCommand('vscode.provideIncomingCalls', ...)` — same
- `vscode.commands.executeCommand('vscode.provideOutgoingCalls', ...)` — same
- `vscode.commands.executeCommand('vscode.prepareTypeHierarchy', ...)` — same
- `vscode.commands.executeCommand('vscode.provideSupertypes', ...)` — same
- `vscode.commands.executeCommand('vscode.provideSubtypes', ...)` — same
- `vscode.commands.executeCommand('editor.action.showReferences', ...)` — overrideable editor command, registered in `src/vs/editor/contrib/referenceSearch/` (out of partition)
- `vscode.commands.executeCommand('setContext', name, value)` — workbench context key service, `src/vs/platform/contextkey/`
- `vscode.commands.executeCommand('vscode.open', uri, options)` — built-in navigation command
- `vscode.window.createTreeView('references-view.tree', ...)` — TreeView contribution point declared in `extensions/references-view/package.json` (package manifest, out of partition)
- `context.workspaceState` (Memento) — extension host workspace state storage, `src/vs/workbench/api/common/extHostStorage.ts`
- `vscode.workspace.onDidChangeConfiguration` — configuration service, `src/vs/platform/configuration/`
- `vscode.window.createTextEditorDecorationType(...)` — editor decoration registry, `src/vs/editor/common/model/`

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Core IDE Functionality Port Research: TreeView & Navigation Patterns

## Research Context
Analysis of `extensions/references-view/` (1,938 LOC across 12 TypeScript files) to identify concrete TreeView, navigation, and command patterns that must be ported from TypeScript/Electron to Tauri/Rust.

## Pattern Findings

#### Pattern 1: TreeView Creation with Data Provider & Drag-Drop Controller
**Where:** `src/tree.ts:31-35`
**What:** Creating a TreeView with a dynamic data provider and drag-drop support for symbol navigation.

```typescript
constructor() {
    this._tree = vscode.window.createTreeView<unknown>(this.viewId, {
        treeDataProvider: this._provider,
        showCollapseAll: true,
        dragAndDropController: this._dnd
    });
    this._navigation = new Navigation(this._tree);
}
```

**Variations / call-sites:**
- Used in main tree initialization (`SymbolsTree` class)
- `viewId = 'references-view.tree'` registers the view in the sidebar
- Multiple `TreeDataProvider` implementations: `ReferencesTreeDataProvider`, `CallItemDataProvider`, `TypeItemDataProvider`
- All providers handle `onDidChangeTreeData` event for reactive updates

---

#### Pattern 2: TreeDataProvider Interface Implementation
**Where:** `src/references/model.ts:259-324`
**What:** Concrete implementation of VSCode's TreeDataProvider with two-level hierarchy (files + references).

```typescript
class ReferencesTreeDataProvider implements vscode.TreeDataProvider<FileItem | ReferenceItem> {

    private readonly _listener: vscode.Disposable;
    private readonly _onDidChange = new vscode.EventEmitter<FileItem | ReferenceItem | undefined>();

    readonly onDidChangeTreeData = this._onDidChange.event;

    constructor(private readonly _model: ReferencesModel) {
        this._listener = _model.onDidChangeTreeData(() => this._onDidChange.fire(undefined));
    }

    async getTreeItem(element: FileItem | ReferenceItem) {
        if (element instanceof FileItem) {
            const result = new vscode.TreeItem(element.uri);
            result.contextValue = 'file-item';
            result.description = true;
            result.iconPath = vscode.ThemeIcon.File;
            result.collapsibleState = vscode.TreeItemCollapsibleState.Collapsed;
            return result;
        } else {
            // references with preview & click-to-open command
            const { range } = element.location;
            const doc = await element.getDocument(true);
            const { before, inside, after } = getPreviewChunks(doc, range);
            const label: vscode.TreeItemLabel = {
                label: before + inside + after,
                highlights: [[before.length, before.length + inside.length]]
            };
            const result = new vscode.TreeItem(label);
            result.command = {
                command: 'vscode.open',
                title: vscode.l10n.t('Open Reference'),
                arguments: [element.location.uri, { selection: range.with({ end: range.start }) }]
            };
            return result;
        }
    }

    async getChildren(element?: FileItem | ReferenceItem) {
        if (!element) return this._model.items;
        if (element instanceof FileItem) return element.references;
        return undefined;
    }

    getParent(element: FileItem | ReferenceItem) {
        return element instanceof ReferenceItem ? element.file : undefined;
    }
}
```

**Variations / call-sites:**
- `CallItemDataProvider` in `src/calls/model.ts:160-225` (single-level tree with lazy-loaded children)
- `TypeItemDataProvider` in `src/types/model.ts:153-196` (hierarchy with async resolution)
- All use `getChildren()`, `getTreeItem()`, `getParent()` contract
- TreeItem command pattern: clicking items opens files via `vscode.open` command
- Selection highlighting via `vscode.TreeItemLabel.highlights` array

---

#### Pattern 3: Dynamic Tree Input & Model Resolution
**Where:** `src/tree.ts:49-117`
**What:** Setting tree input with async model resolution, showing loading UI, and updating navigation/highlights.

```typescript
async setInput(input: SymbolTreeInput<unknown>) {

    if (!await isValidRequestPosition(input.location.uri, input.location.range.start)) {
        this.clearInput();
        return;
    }

    this._ctxInputSource.set(input.contextValue);
    this._ctxIsActive.set(true);
    this._ctxHasResult.set(true);
    vscode.commands.executeCommand(`${this.viewId}.focus`);

    const newInputKind = !this._input || Object.getPrototypeOf(this._input) !== Object.getPrototypeOf(input);
    this._input = input;
    this._sessionDisposable?.dispose();

    this._tree.title = input.title;
    this._tree.message = newInputKind ? undefined : this._tree.message;

    const modelPromise = Promise.resolve(input.resolve());

    // set promise to tree data provider to trigger tree loading UI
    this._provider.update(modelPromise.then(model => model?.provider ?? this._history));
    this._dnd.update(modelPromise.then(model => model?.dnd));

    const model = await modelPromise;
    if (this._input !== input) {
        return;
    }

    if (!model) {
        this.clearInput();
        return;
    }

    this._history.add(input);
    this._tree.message = model.message;

    // navigation
    this._navigation.update(model.navigation);

    // reveal & select
    const selection = model.navigation?.nearest(input.location.uri, input.location.range.start);
    if (selection && this._tree.visible) {
        await this._tree.reveal(selection, { select: true, focus: true, expand: true });
    }
    // ... highlights & listener setup
}
```

**Variations / call-sites:**
- Called from `references-view.findReferences`, `.findImplementations`, `.showCallHierarchy`, `.showTypeHierarchy`
- Uses `TreeDataProviderDelegate` to wrap promise-based provider switching
- `model.navigation` implements `SymbolItemNavigation<T>` interface for prev/next
- `model.highlights` implements `SymbolItemEditorHighlights<T>` for range highlighting
- All inputs must implement `SymbolTreeInput<T>` interface with `.resolve()` method

---

#### Pattern 4: Command-Based Tree Navigation
**Where:** `src/navigation.ts:10-85`
**What:** Keyboard-driven navigation through tree items with editor synchronization.

```typescript
export class Navigation {

    private readonly _disposables: vscode.Disposable[] = [];
    private readonly _ctxCanNavigate = new ContextKey<boolean>('references-view.canNavigate');

    private _delegate?: SymbolItemNavigation<unknown>;

    constructor(private readonly _view: vscode.TreeView<unknown>) {
        this._disposables.push(
            vscode.commands.registerCommand('references-view.next', () => this.next(false)),
            vscode.commands.registerCommand('references-view.prev', () => this.previous(false)),
        );
    }

    next(preserveFocus: boolean): void {
        if (!this._delegate) {
            return;
        }
        const item = this._anchor();
        if (!item) {
            return;
        }
        const newItem = this._delegate.next(item);
        const newLocation = this._delegate.location(newItem);
        if (newLocation) {
            this._view.reveal(newItem, { select: true, focus: true });
            this._open(newLocation, preserveFocus);
        }
    }

    private _open(loc: vscode.Location, preserveFocus: boolean) {
        vscode.commands.executeCommand('vscode.open', loc.uri, {
            selection: new vscode.Selection(loc.range.start, loc.range.start),
            preserveFocus
        });
    }
}
```

**Variations / call-sites:**
- `_anchor()` determines current position from tree selection OR active editor
- Calls `delegate.next(item)` and `delegate.previous(item)` to traverse items
- Opens file via `vscode.open` command with editor selection
- Used by all three hierarchy types (References, Calls, Types)

---

#### Pattern 5: TreeDragAndDropController for URI Export
**Where:** `src/tree.ts:191-222`
**What:** Drag-drop support exporting URIs with range fragments to enable external drag targets.

```typescript
class TreeDndDelegate implements vscode.TreeDragAndDropController<undefined> {

    private _delegate: SymbolItemDragAndDrop<undefined> | undefined;

    readonly dropMimeTypes: string[] = [];
    readonly dragMimeTypes: string[] = ['text/uri-list'];

    update(delegate: Promise<SymbolItemDragAndDrop<unknown> | undefined>) {
        this._delegate = undefined;
        delegate.then(value => this._delegate = value);
    }

    handleDrag(source: undefined[], data: vscode.DataTransfer) {
        if (this._delegate) {
            const urls: string[] = [];
            for (const item of source) {
                const uri = this._delegate.getDragUri(item);
                if (uri) {
                    urls.push(uri.toString());
                }
            }
            if (urls.length > 0) {
                data.set('text/uri-list', new vscode.DataTransferItem(urls.join('\r\n')));
            }
        }
    }

    handleDrop(): void | Thenable<void> {
        throw new Error('Method not implemented.');
    }
}
```

**Drag URI implementation** in `src/references/model.ts:250-256`:
```typescript
getDragUri(item: FileItem | ReferenceItem): vscode.Uri | undefined {
    if (item instanceof FileItem) {
        return item.uri;
    } else {
        return asResourceUrl(item.file.uri, item.location.range);
    }
}
```

URI format with range fragment (from `src/utils.ts:19-21`):
```typescript
export function asResourceUrl(uri: vscode.Uri, range: vscode.Range): vscode.Uri {
    return uri.with({ fragment: `L${1 + range.start.line},${1 + range.start.character}-${1 + range.end.line},${1 + range.end.character}` });
}
```

**Variations / call-sites:**
- `ReferencesModel`, `CallsModel`, `TypesModel` all implement `SymbolItemDragAndDrop<T>`
- Fragment format: `L<startLine>,<startChar>-<endLine>,<endChar>`
- Only supports drag (pull-based), drop is stubbed

---

#### Pattern 6: Context Keys for UI State Management
**Where:** `src/tree.ts:17-19`
**What:** VSCode context keys control visibility and enablement of tree commands.

```typescript
private readonly _ctxIsActive = new ContextKey<boolean>('reference-list.isActive');
private readonly _ctxHasResult = new ContextKey<boolean>('reference-list.hasResult');
private readonly _ctxInputSource = new ContextKey<string>('reference-list.source');
```

**ContextKey utility** in `src/utils.ts:46-57`:
```typescript
export class ContextKey<V> {

    constructor(readonly name: string) { }

    async set(value: V) {
        await vscode.commands.executeCommand('setContext', this.name, value);
    }

    async reset() {
        await vscode.commands.executeCommand('setContext', this.name, undefined);
    }
}
```

**Usage example** in `src/calls/index.ts:51-78`:
```typescript
class RichCallsDirection {

    private static _key = 'references-view.callHierarchyMode';

    private _ctxMode = new ContextKey<'showIncoming' | 'showOutgoing'>('references-view.callHierarchyMode');

    constructor(
        private _mem: vscode.Memento,
        private _value: CallsDirection = CallsDirection.Outgoing,
    ) {
        const raw = _mem.get<number>(RichCallsDirection._key);
        if (typeof raw === 'number' && raw >= 0 && raw <= 1) {
            this.value = raw;
        } else {
            this.value = _value;
        }
    }

    get value() {
        return this._value;
    }

    set value(value: CallsDirection) {
        this._value = value;
        this._ctxMode.set(this._value === CallsDirection.Incoming ? 'showIncoming' : 'showOutgoing');
        this._mem.update(RichCallsDirection._key, value);
    }
}
```

**Variations / call-sites:**
- Context keys enable/disable commands in UI (used in `when` clauses in package.json)
- Persistent state stored via `vscode.Memento` (workspace or global state)
- Buttons toggling between "Incoming/Outgoing" calls and "Supertypes/Subtypes" use this pattern

---

#### Pattern 7: Editor Highlights Synchronized with Tree Selection
**Where:** `src/highlights.ts:9-70`
**What:** Decorate editor with ranges from currently selected tree item, updating on selection change.

```typescript
export class EditorHighlights<T> {

    private readonly _decorationType = vscode.window.createTextEditorDecorationType({
        backgroundColor: new vscode.ThemeColor('editor.findMatchHighlightBackground'),
        rangeBehavior: vscode.DecorationRangeBehavior.ClosedClosed,
        overviewRulerLane: vscode.OverviewRulerLane.Center,
        overviewRulerColor: new vscode.ThemeColor('editor.findMatchHighlightBackground'),
    });

    private readonly disposables: vscode.Disposable[] = [];
    private readonly _ignore = new Set<string>();

    constructor(private readonly _view: vscode.TreeView<T>, private readonly _delegate: SymbolItemEditorHighlights<T>) {
        this.disposables.push(
            vscode.workspace.onDidChangeTextDocument(e => this._ignore.add(e.document.uri.toString())),
            vscode.window.onDidChangeActiveTextEditor(() => _view.visible && this.update()),
            _view.onDidChangeVisibility(e => e.visible ? this._show() : this._hide()),
            _view.onDidChangeSelection(() => {
                if (_view.visible) {
                    this.update();
                }
            })
        );
        this._show();
    }

    private _show(): void {
        const { activeTextEditor: editor } = vscode.window;
        if (!editor || !editor.viewColumn) {
            return;
        }
        if (this._ignore.has(editor.document.uri.toString())) {
            return;
        }
        const [anchor] = this._view.selection;
        if (!anchor) {
            return;
        }
        const ranges = this._delegate.getEditorHighlights(anchor, editor.document.uri);
        if (ranges) {
            editor.setDecorations(this._decorationType, ranges);
        }
    }

    update(): void {
        this._hide();
        this._show();
    }
}
```

**Highlight provider implementation** in `src/references/model.ts:222-225`:
```typescript
getEditorHighlights(_item: FileItem | ReferenceItem, uri: vscode.Uri): vscode.Range[] | undefined {
    const file = this.items.find(file => file.uri.toString() === uri.toString());
    return file?.references.map(ref => ref.location.range);
}
```

**Variations / call-sites:**
- Used for References, Calls, Types views
- Ignores highlights when document is actively being edited
- Listens to tree selection, editor focus, and view visibility changes
- Uses theme colors (`editor.findMatchHighlightBackground`) for consistency

---

## Architectural Summary

### Core Abstractions Required for Rust/Tauri Port

1. **SymbolTree API** - Public interface for activating and configuring the references view
   - `setInput(input: SymbolTreeInput<unknown>)`
   - `getInput(): SymbolTreeInput<unknown> | undefined`

2. **SymbolTreeInput<T>** - Entry point for populating tree with any symbol type
   - `contextValue: string`
   - `title: string`
   - `location: vscode.Location`
   - `resolve(): Promise<SymbolTreeModel<T>>`
   - `with(location): SymbolTreeInput<T>` (for history re-runs)

3. **SymbolTreeModel<T>** - Model returned by input resolution
   - `provider: TreeDataProvider<T>` (required)
   - `message?: string` (loading/empty state)
   - `navigation?: SymbolItemNavigation<T>` (prev/next support)
   - `highlights?: SymbolItemEditorHighlights<T>` (range highlighting)
   - `dnd?: SymbolItemDragAndDrop<T>` (drag URIs)

4. **TreeDataProvider<T>** - VSCode interface for tree rendering
   - `getTreeItem(element: T): TreeItem`
   - `getChildren(element?: T): T[]`
   - `getParent(element: T): T | undefined`
   - `onDidChangeTreeData: Event<T | undefined>`

5. **TreeDragAndDropController** - URI drag support
   - `dragMimeTypes: string[]` = `['text/uri-list']`
   - `handleDrag(source: T[], data: DataTransfer): void`

6. **Command Registry Pattern** - All state changes via commands
   - `references-view.findReferences`
   - `references-view.findImplementations`
   - `references-view.showCallHierarchy`
   - `references-view.showTypeHierarchy`
   - `references-view.next` / `.prev`
   - `references-view.showIncomingCalls` / `.showOutgoingCalls`
   - `references-view.showSupertypes` / `.showSubtypes`
   - Context-sensitive item commands (copy, remove, rerun history)

7. **Context Key Pattern** - Command visibility and button state
   - `reference-list.isActive` (bool)
   - `reference-list.hasResult` (bool)
   - `reference-list.source` (string: contextValue)
   - `references-view.canNavigate` (bool)
   - `references-view.callHierarchyMode` (string: 'showIncoming'|'showOutgoing')
   - `references-view.typeHierarchyMode` (string: 'subtypes'|'supertypes')

8. **Search History Management** - History tree provider
   - Stores previous searches with tracked word anchors
   - Supports re-running from history with position tracking
   - Quick picker for history selection

### Type Hierarchy Examples

**References View (two-level flat):**
- Root: Files (grouped by URI)
- Children: ReferenceItems (line:col with code preview)

**Call Hierarchy (multi-level tree):**
- Root: CallItems (function definitions)
- Children: lazy-loaded CallItems (callers or callees)
- Lazy resolution: `vscode.provideIncomingCalls` / `vscode.provideOutgoingCalls`

**Type Hierarchy (multi-level tree):**
- Root: TypeItems (class/interface definitions)
- Children: lazy-loaded TypeItems (supertypes or subtypes)
- Lazy resolution: `vscode.provideSupertypes` / `vscode.provideSubtypes`

### Key Port Considerations

1. **Event-Driven Architecture**: All tree updates flow through `onDidChangeTreeData` events
2. **Promise-Based Async**: Both model resolution and item lazy-loading use Promises
3. **Tree Selection State**: Maintained by TreeView; navigation anchors from tree selection or active editor
4. **Range Metadata**: Ranges attached to TreeItems via `vscode.TreeItemLabel.highlights` for rendering
5. **Drag URI Format**: Standard VSCode range fragment `L<line>,<char>-<line>,<char>` in URI
6. **Theme Integration**: Colors and icons use VSCode theme variables, not hardcoded values
7. **Disposable Pattern**: All event listeners tracked and disposed to prevent leaks

---

## Files in Scope

All files analyzed in `extensions/references-view/src/`:
- `extension.ts` - Extension activation entry point
- `tree.ts` - Main TreeView and delegating providers (353 LOC)
- `navigation.ts` - Keyboard navigation implementation (86 LOC)
- `highlights.ts` - Editor decoration synchronization (71 LOC)
- `references/index.ts` - References search command registration (96 LOC)
- `references/model.ts` - References data model and providers (386 LOC)
- `calls/index.ts` - Call hierarchy command registration (79 LOC)
- `calls/model.ts` - Call hierarchy data model (226 LOC)
- `types/index.ts` - Type hierarchy command registration (81 LOC)
- `types/model.ts` - Type hierarchy data model (197 LOC)
- `utils.ts` - Utilities: ContextKey, decoration, preview chunks (146 LOC)
- `references-view.d.ts` - Public API type definitions (159 LOC)

**Total: 1,938 LOC, 12 files**

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
