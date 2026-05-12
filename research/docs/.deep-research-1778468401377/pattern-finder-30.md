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

