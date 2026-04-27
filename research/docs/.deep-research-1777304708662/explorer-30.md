# Partition 30 of 79 — Findings

## Scope
`extensions/references-view/` (14 files, 1,938 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Partition 30: extensions/references-view/ — TreeView API Patterns for Porting

## Overview
The `references-view` extension demonstrates core TreeView API consumption patterns and language intelligence integration. These patterns are critical for porting VS Code's symbol navigation features (References, Implementations, Call Hierarchy, Type Hierarchy) to a Tauri/Rust architecture.

---

## Pattern: TreeView Creation and Configuration

**Where:** `extensions/references-view/src/tree.ts:31-36`

**What:** Core TreeView instantiation with TreeDataProvider delegation and drag-and-drop controller setup. This establishes the foundational UI component that must be reimplemented in Tauri.

```typescript
this._tree = vscode.window.createTreeView<unknown>(this.viewId, {
    treeDataProvider: this._provider,
    showCollapseAll: true,
    dragAndDropController: this._dnd
});
```

**Variations / call-sites:**
- TreeView is created with `viewId: 'references-view.tree'` (line 15)
- Configuration includes collapse-all toggle and DnD support
- Tree view is referenced throughout lifecycle in `setInput()`, `clearInput()`, and `reveal()` operations (lines 65-94)

---

## Pattern: TreeDataProvider Delegation with Async Promise Wrapping

**Where:** `extensions/references-view/src/tree.ts:138-187`

**What:** A delegate pattern that wraps async provider resolution. The `TreeDataProviderDelegate` class intercepts calls to `getTreeItem`, `getChildren`, and `getParent`, forwarding them to a lazily-resolved provider wrapped in a Promise. This pattern enables UI loading states while data is being fetched.

```typescript
class TreeDataProviderDelegate implements vscode.TreeDataProvider<undefined> {
    provider?: Promise<vscode.TreeDataProvider<any>>;
    private _onDidChange = new vscode.EventEmitter<any>();
    readonly onDidChangeTreeData = this._onDidChange.event;

    update(provider: Promise<vscode.TreeDataProvider<any>>) {
        this._sessionDispoables?.dispose();
        this._onDidChange.fire(undefined);
        this.provider = provider;
        provider.then(value => {
            if (this.provider === provider && value.onDidChangeTreeData) {
                this._sessionDispoables = value.onDidChangeTreeData(
                    this._onDidChange.fire, this._onDidChange
                );
            }
        }).catch(err => {
            this.provider = undefined;
            console.error(err);
        });
    }

    async getTreeItem(element: unknown) {
        this._assertProvider();
        return (await this.provider).getTreeItem(element);
    }

    async getChildren(parent?: unknown) {
        this._assertProvider();
        return (await this.provider).getChildren(parent);
    }
}
```

**Variations / call-sites:**
- Used in `setInput()` at line 71: `this._provider.update(modelPromise.then(...))`
- Subscribes to model's `onDidChangeTreeData` event delegation (lines 156-159)
- Propagates change events to tree (line 152: `this._onDidChange.fire(undefined)`)

---

## Pattern: SymbolTreeInput Interface and Resolve Pattern

**Where:** `extensions/references-view/src/references-view.d.ts:46-81`

**What:** Abstract input contract that decouples input specification from model resolution. Each input type (References, Calls, Types) implements this interface, enabling polymorphic handling in the tree controller.

```typescript
export interface SymbolTreeInput<T> {
    readonly contextValue: string;
    readonly title: string;
    readonly location: vscode.Location;
    resolve(): vscode.ProviderResult<SymbolTreeModel<T>>;
    with(location: vscode.Location): SymbolTreeInput<T>;
}
```

**Variations / call-sites:**
- `ReferencesTreeInput` (references/model.ts:10-53): Resolves by executing `vscode.executeReferenceProvider` or `vscode.executeImplementationProvider`
- `CallsTreeInput` (calls/model.ts:10-49): Executes `vscode.prepareCallHierarchy` then lazy-loads call children
- `TypesTreeInput` (types/model.ts:10-49): Executes `vscode.prepareTypeHierarchy` then lazy-loads type children
- Each input caches results and returns undefined if no results found (references/model.ts:33-35, calls/model.ts:30-32)

---

## Pattern: Command Registration for executeCommand Integration

**Where:** `extensions/references-view/src/references/index.ts:10-54`

**What:** Registration of commands that call into VS Code language features via `vscode.commands.executeCommand`. These commands bridge extension UI to language server capabilities.

```typescript
export function register(tree: SymbolsTree, context: vscode.ExtensionContext): void {
    function findLocations(title: string, command: string) {
        if (vscode.window.activeTextEditor) {
            const input = new ReferencesTreeInput(
                title, 
                new vscode.Location(
                    vscode.window.activeTextEditor.document.uri, 
                    vscode.window.activeTextEditor.selection.active
                ), 
                command
            );
            tree.setInput(input);
        }
    }

    context.subscriptions.push(
        vscode.commands.registerCommand(
            'references-view.findReferences', 
            () => findLocations('References', 'vscode.executeReferenceProvider')
        ),
        vscode.commands.registerCommand(
            'references-view.findImplementations', 
            () => findLocations('Implementations', 'vscode.executeImplementationProvider')
        )
    );
}
```

**Variations / call-sites:**
- References: `vscode.executeReferenceProvider` (line 20)
- Implementations: `vscode.executeImplementationProvider` (line 21)
- Call Hierarchy: `vscode.prepareCallHierarchy` (calls/model.ts:26), `vscode.provideIncomingCalls` (calls/model.ts:88), `vscode.provideOutgoingCalls` (calls/model.ts:91)
- Type Hierarchy: `vscode.prepareTypeHierarchy` (types/model.ts:26), `vscode.provideSupertypes` (types/model.ts:86), `vscode.provideSubtypes` (types/model.ts:89)

---

## Pattern: EventEmitter-Based Tree State Management

**Where:** `extensions/references-view/src/references/model.ts:55-75`

**What:** Model classes implement `SymbolItemNavigation`, `SymbolItemEditorHighlights`, and `SymbolItemDragAndDrop` interfaces, using EventEmitter to signal changes. Models manage tree hierarchy, filtering, and navigation state.

```typescript
export class ReferencesModel implements 
    SymbolItemNavigation<FileItem | ReferenceItem>, 
    SymbolItemEditorHighlights<FileItem | ReferenceItem>, 
    SymbolItemDragAndDrop<FileItem | ReferenceItem> {

    private _onDidChange = new vscode.EventEmitter<FileItem | ReferenceItem | undefined>();
    readonly onDidChangeTreeData = this._onDidChange.event;
    readonly items: FileItem[] = [];

    constructor(locations: vscode.Location[] | vscode.LocationLink[]) {
        let last: FileItem | undefined;
        for (const item of locations.sort(ReferencesModel._compareLocations)) {
            const loc = item instanceof vscode.Location
                ? item
                : new vscode.Location(item.targetUri, item.targetRange);

            if (!last || ReferencesModel._compareUriIgnoreFragment(last.uri, loc.uri) !== 0) {
                last = new FileItem(loc.uri.with({ fragment: '' }), [], this);
                this.items.push(last);
            }
            last.references.push(new ReferenceItem(loc, last));
        }
    }
}
```

**Variations / call-sites:**
- Fire change events: `this._onDidChange.fire(undefined)` (references/model.ts:230, 327)
- Implement navigation: `nearest()`, `next()`, `previous()` (references/model.ts:133-220)
- Implement highlights: `getEditorHighlights()` (references/model.ts:222-225)
- Implement DnD: `getDragUri()` (references/model.ts:250-256)
- Similar pattern in `CallsModel` (calls/model.ts:75-158) and `TypesModel` (types/model.ts:73-151)

---

## Pattern: Hierarchical Tree Data Provider Implementation

**Where:** `extensions/references-view/src/references/model.ts:259-324`

**What:** TreeDataProvider that converts model data into TreeItem structures. Handles parent-child relationships, context values, and command bindings. Demonstrates lazy loading and item rendering.

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
            const { range } = element.location;
            const doc = await element.getDocument(true);
            const { before, inside, after } = getPreviewChunks(doc, range);
            const label: vscode.TreeItemLabel = {
                label: before + inside + after,
                highlights: [[before.length, before.length + inside.length]]
            };
            const result = new vscode.TreeItem(label);
            result.collapsibleState = vscode.TreeItemCollapsibleState.None;
            result.contextValue = 'reference-item';
            result.command = {
                command: 'vscode.open',
                title: vscode.l10n.t('Open Reference'),
                arguments: [element.location.uri, { 
                    selection: range.with({ end: range.start }) 
                }]
            };
            return result;
        }
    }

    async getChildren(element?: FileItem | ReferenceItem) {
        if (!element) {
            return this._model.items;
        }
        if (element instanceof FileItem) {
            return element.references;
        }
        return undefined;
    }

    getParent(element: FileItem | ReferenceItem) {
        return element instanceof ReferenceItem ? element.file : undefined;
    }
}
```

**Variations / call-sites:**
- CallItemDataProvider (calls/model.ts:160-225): Lazy-loads children via `this._model.getCallChildren(element)` (line 218)
- TypeItemDataProvider (types/model.ts:153-196): Similar lazy-load pattern via `this._model.getTypeChildren(element)` (line 189)

---

## Pattern: Navigation Context Keys and Commands

**Where:** `extensions/references-view/src/tree.ts:17-20`

**What:** Context keys control UI visibility and command enablement. The pattern uses `ContextKey` wrapper to manage boolean and string-based context state that gates navigation and view-specific commands.

```typescript
private readonly _ctxIsActive = new ContextKey<boolean>('reference-list.isActive');
private readonly _ctxHasResult = new ContextKey<boolean>('reference-list.hasResult');
private readonly _ctxInputSource = new ContextKey<string>('reference-list.source');
```

**Variations / call-sites:**
- Set context on input update (tree.ts:56-58): `_ctxInputSource.set(input.contextValue)`, `_ctxIsActive.set(true)`, `_ctxHasResult.set(true)`
- Clear on no results (tree.ts:122-123): `_ctxHasResult.set(false)`, `_ctxInputSource.reset()`
- Navigation context (navigation.ts:13): `_ctxCanNavigate.set(Boolean(this._delegate))`
- Call hierarchy direction (calls/index.ts:55): `_ctxMode.set(this._value === CallsDirection.Incoming ? 'showIncoming' : 'showOutgoing')`
- Type hierarchy direction (types/index.ts:57): `_ctxMode.set(value)`

---

## Pattern: Editor Highlights with Decoration API

**Where:** `extensions/references-view/src/highlights.ts:9-70`

**What:** Uses `vscode.window.createTextEditorDecorationType()` to render range highlights in editors. Listens to tree selection changes and activeTextEditor changes to update highlights.

```typescript
export class EditorHighlights<T> {
    private readonly _decorationType = vscode.window.createTextEditorDecorationType({
        backgroundColor: new vscode.ThemeColor('editor.findMatchHighlightBackground'),
        rangeBehavior: vscode.DecorationRangeBehavior.ClosedClosed,
        overviewRulerLane: vscode.OverviewRulerLane.Center,
        overviewRulerColor: new vscode.ThemeColor('editor.findMatchHighlightBackground'),
    });

    constructor(private readonly _view: vscode.TreeView<T>, 
                private readonly _delegate: SymbolItemEditorHighlights<T>) {
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
    }

    private _show(): void {
        const { activeTextEditor: editor } = vscode.window;
        const [anchor] = this._view.selection;
        if (anchor) {
            const ranges = this._delegate.getEditorHighlights(anchor, editor.document.uri);
            if (ranges) {
                editor.setDecorations(this._decorationType, ranges);
            }
        }
    }
}
```

**Variations / call-sites:**
- Highlights setup (tree.ts:98-112): Creates `EditorHighlights` when model provides highlights capability
- Cleanup on model change (tree.ts:110): `highlights?.update()` triggers redecoration

---

## Pattern: Lazy Child Resolution for Hierarchies

**Where:** `extensions/references-view/src/calls/model.ts:86-101`

**What:** Parent-child relationships are resolved lazily on demand. Models cache children to avoid redundant network calls, implementing a simple memoization pattern for language server queries.

```typescript
private async _resolveCalls(call: CallItem): Promise<CallItem[]> {
    if (this.direction === CallsDirection.Incoming) {
        const calls = await vscode.commands.executeCommand<vscode.CallHierarchyIncomingCall[]>(
            'vscode.provideIncomingCalls', 
            call.item
        );
        return calls ? calls.map(item => new CallItem(
            this, item.from, call, 
            item.fromRanges.map(range => new vscode.Location(item.from.uri, range))
        )) : [];
    } else {
        const calls = await vscode.commands.executeCommand<vscode.CallHierarchyOutgoingCall[]>(
            'vscode.provideOutgoingCalls', 
            call.item
        );
        return calls ? calls.map(item => new CallItem(
            this, item.to, call, 
            item.fromRanges.map(range => new vscode.Location(call.item.uri, range))
        )) : [];
    }
}

async getCallChildren(call: CallItem): Promise<CallItem[]> {
    if (!call.children) {
        call.children = await this._resolveCalls(call);
    }
    return call.children;
}
```

**Variations / call-sites:**
- CallItemDataProvider uses it (calls/model.ts:217-219): `return element ? this._model.getCallChildren(element) : this._model.roots;`
- TypesModel follows same pattern (types/model.ts:84-99)

---

## Pattern: History and Persistence with Memento

**Where:** `extensions/references-view/src/calls/index.ts:51-78`

**What:** Uses `vscode.Memento` (workspace state) to persist user preferences across sessions. The `RichCallsDirection` class wraps CallsDirection enum with persistence.

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
- Initialized with `context.workspaceState` (calls/index.ts:13)
- Used to restore direction on extension activation (calls/index.ts:15-20)
- Updated when user toggles directions (calls/index.ts:22-35, 73-76)
- Similar pattern in types/index.ts:53-80 for type hierarchy direction

---

## Porting Considerations

### Key Challenges for Tauri/Rust

1. **TreeView API Replacement**: Tauri lacks built-in TreeView component. Need custom tree renderer (likely webview-based) with parent-child relationship management, expand/collapse state, selection tracking.

2. **EventEmitter Pattern**: Rust equivalents (channels, Arc<Mutex<>>-based listeners) are more verbose. Consider event-driven architecture with pub/sub pattern or message passing.

3. **Async Provider Delegation**: The promise-based lazy loading pattern requires careful async/await translation to Rust futures and channels.

4. **Language Server Integration**: Commands like `vscode.executeReferenceProvider` must be replaced with LSP client calls (textDocument/references, textDocument/implementation, callHierarchy/prepare, etc.).

5. **Persistence (Memento)**: Replace with Tauri's `tauri::fs` or SQLite for workspace state storage.

6. **Context Key System**: Implement custom conditional UI rendering based on state that maps to webview UI element visibility/enablement.

### Core Components to Rebuild

- TreeView container with virtualization (for performance at scale)
- Async TreeDataProvider interface with event-based refresh
- SymbolTreeInput contract for polymorphic input types
- EventEmitter replacement (channels or observer pattern)
- LSP client for direct language feature queries
- Memento-like state persistence layer
- Navigation and selection management with keyboard shortcuts
- Decoration/highlight rendering in editor (via LSP semantic tokens or editor API)

---

## File Locations

- **Extension entry**: `/extensions/references-view/src/extension.ts`
- **Core tree logic**: `/extensions/references-view/src/tree.ts`
- **References feature**: `/extensions/references-view/src/references/index.ts`, `/extensions/references-view/src/references/model.ts`
- **Call Hierarchy**: `/extensions/references-view/src/calls/index.ts`, `/extensions/references-view/src/calls/model.ts`
- **Type Hierarchy**: `/extensions/references-view/src/types/index.ts`, `/extensions/references-view/src/types/model.ts`
- **Navigation**: `/extensions/references-view/src/navigation.ts`
- **Highlights**: `/extensions/references-view/src/highlights.ts`
- **Public API contract**: `/extensions/references-view/src/references-view.d.ts`

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
