# VS Code TreeView API & Language Intelligence Patterns
**Research Scope**: `extensions/references-view/` (14 files, 1,938 LOC)  
**Focus**: TreeView API usage, TreeDataProvider implementations, language provider integration patterns

---

## Pattern Analysis: Language Intelligence Integration via TreeView

### Pattern 1: TreeView Creation with Data Provider Delegation
**Where**: `extensions/references-view/src/tree.ts:31-35`
**What**: Initializes TreeView with pluggable TreeDataProvider and DragAndDropController delegates.

```typescript
this._tree = vscode.window.createTreeView<unknown>(this.viewId, {
    treeDataProvider: this._provider,
    showCollapseAll: true,
    dragAndDropController: this._dnd
});
```

**Variations**: 
- All three views (references, calls, types) use identical TreeView setup
- Generic `unknown` type allows type-safe wrapping at higher levels
- Delegates (`_provider`, `_dnd`) support runtime provider swapping

---

### Pattern 2: TreeDataProvider Wrapper with Async Promise-Based Provider Loading
**Where**: `extensions/references-view/src/tree.ts:138-186`
**What**: Delegates TreeDataProvider that wraps a Promise to support lazy provider initialization and hot-swapping.

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
                    this._onDidChange.fire, 
                    this._onDidChange
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
}
```

**Variations**:
- References view (`ReferencesTreeDataProvider`) directly implements TreeDataProvider
- Calls/Types views (`CallItemDataProvider`, `TypeItemDataProvider`) also directly implement
- Wrapper pattern decouples TreeView from model resolution lifecycle

---

### Pattern 3: Command Bridge to Language Providers via executeCommand
**Where**: `extensions/references-view/src/references/model.ts:29`, `src/calls/model.ts:26`, `src/types/model.ts:26`
**What**: Three parallel patterns executing different language provider commands.

**References** (line 29):
```typescript
const resut = await Promise.resolve(vscode.commands.executeCommand<vscode.Location[] | vscode.LocationLink[]>(
    this._command, 
    this.location.uri, 
    this.location.range.start
));
```

**Calls** (line 26):
```typescript
const items = await Promise.resolve(vscode.commands.executeCommand<vscode.CallHierarchyItem[]>(
    'vscode.prepareCallHierarchy', 
    this.location.uri, 
    this.location.range.start
));
```

**Calls Children Resolution** (lines 88-89):
```typescript
const calls = await vscode.commands.executeCommand<vscode.CallHierarchyIncomingCall[]>(
    'vscode.provideIncomingCalls', 
    call.item
);
```

**Types** (line 26):
```typescript
const items = await Promise.resolve(vscode.commands.executeCommand<vscode.TypeHierarchyItem[]>(
    'vscode.prepareTypeHierarchy', 
    this.location.uri, 
    this.location.range.start
));
```

**Variations**:
- References can use static `vscode.executeReferenceProvider` or `vscode.executeImplementationProvider`
- Calls/Types use prepare/provide pattern: prepare returns roots, provide* traverse hierarchy
- All three bridge through command system, not direct provider interfaces

---

### Pattern 4: Multi-Role Model Pattern (Navigation + Highlights + DragAndDrop)
**Where**: `extensions/references-view/src/references/model.ts:55`, `src/calls/model.ts:75`, `src/types/model.ts:73`
**What**: Single model class implements multiple semantic interfaces for UI state.

```typescript
export class ReferencesModel implements 
    SymbolItemNavigation<FileItem | ReferenceItem>, 
    SymbolItemEditorHighlights<FileItem | ReferenceItem>, 
    SymbolItemDragAndDrop<FileItem | ReferenceItem> {
    
    private _onDidChange = new vscode.EventEmitter<FileItem | ReferenceItem | undefined>();
    readonly onDidChangeTreeData = this._onDidChange.event;
    readonly items: FileItem[] = [];

    // Navigation interface
    nearest(uri: vscode.Uri, position: vscode.Position): FileItem | ReferenceItem | undefined { ... }
    next(item: FileItem | ReferenceItem): FileItem | ReferenceItem { ... }
    previous(item: FileItem | ReferenceItem): FileItem | ReferenceItem { ... }

    // Editor highlights interface
    getEditorHighlights(_item: FileItem | ReferenceItem, uri: vscode.Uri): vscode.Range[] | undefined { ... }

    // Drag and drop interface
    getDragUri(item: FileItem | ReferenceItem): vscode.Uri | undefined { ... }
}
```

**Variations**:
- `CallsModel` and `TypesModel` follow identical multi-role pattern
- Each model fires `onDidChange` when structure mutates
- Navigation pre-computes traversal paths from sorted data

---

### Pattern 5: Context Key State Management for UI Visibility
**Where**: `extensions/references-view/src/tree.ts:17-19`, `src/utils.ts:46-57`, `src/calls/index.ts:55`
**What**: Context keys control TreeView visibility and command enablement via `setContext` command.

```typescript
// In SymbolsTree class:
private readonly _ctxIsActive = new ContextKey<boolean>('reference-list.isActive');
private readonly _ctxHasResult = new ContextKey<boolean>('reference-list.hasResult');
private readonly _ctxInputSource = new ContextKey<string>('reference-list.source');

// ContextKey utility:
export class ContextKey<V> {
    constructor(readonly name: string) { }
    
    async set(value: V) {
        await vscode.commands.executeCommand('setContext', this.name, value);
    }
    
    async reset() {
        await vscode.commands.executeCommand('setContext', this.name, undefined);
    }
}

// Usage in setInput():
this._ctxInputSource.set(input.contextValue);
this._ctxIsActive.set(true);
this._ctxHasResult.set(true);
```

**Variations**:
- `RichCallsDirection` persists direction to `workspaceState.memento` and syncs to context key
- `RichTypesDirection` follows same pattern with enum direction values
- Package.json menus use `when` clauses: `reference-list.hasResult && reference-list.source == callHierarchy`

---

### Pattern 6: Input Lifecycle Management with Model Resolution
**Where**: `extensions/references-view/src/tree.ts:49-117`
**What**: Accepts `SymbolTreeInput`, validates, resolves to `SymbolTreeModel`, manages disposables.

```typescript
async setInput(input: SymbolTreeInput<unknown>) {
    // Validate
    if (!await isValidRequestPosition(input.location.uri, input.location.range.start)) {
        this.clearInput();
        return;
    }

    // Update context
    this._ctxInputSource.set(input.contextValue);
    this._ctxIsActive.set(true);
    this._ctxHasResult.set(true);
    vscode.commands.executeCommand(`${this.viewId}.focus`);

    // Store current input
    this._input = input;
    this._sessionDisposable?.dispose();
    this._tree.title = input.title;

    // Resolve to model
    const modelPromise = Promise.resolve(input.resolve());
    this._provider.update(modelPromise.then(model => model?.provider ?? this._history));
    this._dnd.update(modelPromise.then(model => model?.dnd));

    const model = await modelPromise;
    if (this._input !== input) return; // Input changed, abort

    if (!model) {
        this.clearInput();
        return;
    }

    // Setup highlights, navigation, listeners
    this._history.add(input);
    this._tree.message = model.message;
    this._navigation.update(model.navigation);

    // Reveal & select
    const selection = model.navigation?.nearest(input.location.uri, input.location.range.start);
    if (selection && this._tree.visible) {
        await this._tree.reveal(selection, { select: true, focus: true, expand: true });
    }

    // Wire up change listeners
    if (model.provider.onDidChangeTreeData) {
        disposables.push(model.provider.onDidChangeTreeData(() => {
            this._tree.title = input.title;
            this._tree.message = model.message;
            highlights?.update();
        }));
    }
}
```

**Variations**:
- `ReferencesTreeInput.resolve()` calls provider command, wraps in `ReferencesModel`
- `CallsTreeInput.resolve()` calls prepare, lazy-loads children on demand
- `TypesTreeInput.resolve()` mirrors calls pattern

---

### Pattern 7: History Navigation with Position Tracking
**Where**: `extensions/references-view/src/tree.ts:226-352`
**What**: Maintains search history as `TreeDataProvider`, tracks document edits, recovers positions across changes.

```typescript
class HistoryItem {
    readonly description: string;
    constructor(
        readonly key: string,
        readonly word: string,
        readonly anchor: WordAnchor,
        readonly input: SymbolTreeInput<unknown>,
    ) {
        this.description = `${vscode.workspace.asRelativePath(input.location.uri)} • ${input.title.toLocaleLowerCase()}`;
    }
}

class TreeInputHistory implements vscode.TreeDataProvider<HistoryItem> {
    private readonly _inputs = new Map<string, HistoryItem>();

    async add(input: SymbolTreeInput<unknown>) {
        const doc = await vscode.workspace.openTextDocument(input.location.uri);
        const anchor = new WordAnchor(doc, input.location.range.start);
        const range = doc.getWordRangeAtPosition(input.location.range.start) 
            ?? doc.getWordRangeAtPosition(input.location.range.start, /[^\s]+/);
        const word = range ? doc.getText(range) : '???';

        const item = new HistoryItem(
            JSON.stringify([range?.start ?? input.location.range.start, input.location.uri, input.title]),
            word,
            anchor,
            input
        );
        // FILO ordering of native maps
        this._inputs.delete(item.key);
        this._inputs.set(item.key, item);
        this._ctxHasHistory.set(true);
    }

    private _reRunHistoryItem(item: HistoryItem): void {
        this._inputs.delete(item.key);
        const newPosition = item.anchor.guessedTrackedPosition();
        let newInput = item.input;
        if (newPosition && !item.input.location.range.start.isEqual(newPosition)) {
            newInput = item.input.with(new vscode.Location(item.input.location.uri, newPosition));
        }
        this._tree.setInput(newInput);
    }
}
```

**WordAnchor tracking** (`src/utils.ts:59-122`):
```typescript
export class WordAnchor {
    constructor(private readonly _doc: vscode.TextDocument, private readonly _position: vscode.Position) {
        this._version = _doc.version;
        this._word = this._getAnchorWord(_doc, _position);
    }

    guessedTrackedPosition(): vscode.Position | undefined {
        // no changes
        if (this._version === this._doc.version) {
            return this._position;
        }

        // search word downwards and upwards from original line
        const startLine = this._position.line;
        for (let i = 0; i < 100; i++) {
            // nth line down
            let line = startLine + i;
            if (line < this._doc.lineCount) {
                const ch = this._doc.lineAt(line).text.indexOf(this._word);
                if (ch >= 0) return new vscode.Position(line, ch);
            }
            // nth line up
            line = startLine - i;
            if (line >= 0) {
                const ch = this._doc.lineAt(line).text.indexOf(this._word);
                if (ch >= 0) return new vscode.Position(line, ch);
            }
        }
        return this._position;
    }
}
```

**Variations**:
- History doubles as fallback provider when no results
- Commands like `references-view.pickFromHistory` show QuickPick of history
- Position tracking adapts to document edits (line insertions/deletions)

---

### Pattern 8: Navigation Command System with Editor Focus
**Where**: `extensions/references-view/src/navigation.ts:10-85`
**What**: Commands that navigate tree and synchronize editor position, respecting TreeView visibility.

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

    update(delegate: SymbolItemNavigation<unknown> | undefined) {
        this._delegate = delegate;
        this._ctxCanNavigate.set(Boolean(this._delegate));
    }

    next(preserveFocus: boolean): void {
        if (!this._delegate) return;
        
        // Anchor to current selection or nearest to editor position
        const [sel] = this._view.selection;
        const item = sel ?? this._delegate.nearest(
            vscode.window.activeTextEditor.document.uri,
            vscode.window.activeTextEditor.selection.active
        );
        
        const newItem = this._delegate.next(item);
        const newLocation = this._delegate.location(newItem);
        
        if (newLocation) {
            this._view.reveal(newItem, { select: true, focus: true });
            vscode.commands.executeCommand('vscode.open', newLocation.uri, {
                selection: new vscode.Selection(newLocation.range.start, newLocation.range.start),
                preserveFocus
            });
        }
    }
}
```

**Keybindings** (package.json lines 380-388):
```json
{
    "command": "references-view.next",
    "when": "reference-list.hasResult",
    "key": "f4"
},
{
    "command": "references-view.prev",
    "when": "reference-list.hasResult",
    "key": "shift+f4"
}
```

---

### Pattern 9: Editor Highlights via Decorations
**Where**: `extensions/references-view/src/highlights.ts:9-70`
**What**: Synchronizes tree selection to decorations in editors, respects visibility.

```typescript
export class EditorHighlights<T> {
    private readonly _decorationType = vscode.window.createTextEditorDecorationType({
        backgroundColor: new vscode.ThemeColor('editor.findMatchHighlightBackground'),
        rangeBehavior: vscode.DecorationRangeBehavior.ClosedClosed,
        overviewRulerLane: vscode.OverviewRulerLane.Center,
        overviewRulerColor: new vscode.ThemeColor('editor.findMatchHighlightBackground'),
    });

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
        if (!editor || !editor.viewColumn) return;
        if (this._ignore.has(editor.document.uri.toString())) return;

        const [anchor] = this._view.selection;
        if (!anchor) return;

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

**Variations**:
- Ignores changes made by code actions triggered from view
- Highlights clear when view hidden
- Different models return different ranges (references, call locations, selectionRange)

---

## Summary: Porting Considerations

### Core Architectural Patterns to Preserve

1. **Decoupled Provider System**: The TreeDataProvider delegate pattern allows swapping implementations without changing TreeView. A Tauri/Rust port would need async provider loading and hot-swapping.

2. **Command Bridge**: All language intelligence flows through `executeCommand`. This is crucial: implementations don't call providers directly, but through a command dispatch layer. A Tauri port must maintain this async command bridge (likely via IPC).

3. **Multi-Role Models**: Single model class handles navigation, highlights, and drag-and-drop. This tight binding between UI state and model mutations must survive the port.

4. **Context Keys for UI State**: The visibility and enablement logic depends on hierarchical context key state (`reference-list.isActive`, `reference-list.source`, `references-view.callHierarchyMode`). Tauri would need equivalent reactive state system.

5. **Input Lifecycle**: The `setInput(SymbolTreeInput) -> resolve() -> SymbolTreeModel` pattern validates positions, manages disposables, and wires event listeners. This lifecycle is complex and must be faithfully replicated.

6. **Position Tracking Across Edits**: The `WordAnchor` mechanism using document version numbers and bidirectional line search is essential for maintaining references across document changes.

### Implementation Challenges

- **Async Provider Resolution**: Promise-based lazy loading of data providers. Rust async/await and Tokio patterns differ from TypeScript.
- **Command Dispatch**: Language providers are accessed via `vscode.commands.executeCommand`. Tauri would need equivalent command routing (could use `invoke` or similar).
- **Event Subscription**: Chaining `onDidChange` listeners across multiple components (model, provider, highlights, history). Requires robust lifecycle management.
- **Theming**: Decorations use `vscode.ThemeColor` references. Tauri would need theme token system.
- **Editor Integration**: Document APIs, position tracking, selection synchronization. Core functionality not in references-view but required by it.

### File Paths Referenced
- `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/extension.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/tree.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/references/model.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/calls/model.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/types/model.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/utils.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/navigation.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/highlights.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/references-view.d.ts`
- `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/package.json`

