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
