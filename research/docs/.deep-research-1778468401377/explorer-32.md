# Partition 32 of 80 — Findings

## Scope
`extensions/merge-conflict/` (13 files, 1,463 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

1. `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/mergeConflictMain.ts`
2. `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/interfaces.ts`
3. `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/services.ts`
4. `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/mergeConflictParser.ts`
5. `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/documentTracker.ts`
6. `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/documentMergeConflict.ts`
7. `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/commandHandler.ts`
8. `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/codelensProvider.ts`
9. `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/mergeDecorator.ts`
10. `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/contentProvider.ts`
11. `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/delayer.ts`

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/mergeConflictMain.ts`

- **Role:** Extension activation entry point. Constructs the top-level `ServiceWrapper` and calls `begin()`, then registers it for disposal with the VS Code extension context.
- **Key symbols:**
  - `activate(context)` at line 9 — VS Code lifecycle hook.
  - `MergeConflictServices` import at line 7 — aliased from `./services`.
- **Control flow:** `activate` → `new MergeConflictServices(context)` → `services.begin()` → `context.subscriptions.push(services)`. `deactivate` at line 16 is a no-op.
- **Data flow:** `vscode.ExtensionContext` is passed directly into `ServiceWrapper`, which accesses `context.extension.packageJSON.aiKey` and `context.subscriptions` internally.
- **Dependencies:** `vscode` (API), `./services`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/interfaces.ts`

- **Role:** Defines all shared TypeScript contracts used throughout the extension. Acts as the type layer between parsing, tracking, and UI components.
- **Key symbols:**
  - `IMergeRegion` (line 7) — `{ name, header: Range, content: Range, decoratorContent: Range }`.
  - `CommitType` const enum (line 14) — values `Current=0`, `Incoming=1`, `Both=2`.
  - `IExtensionConfiguration` (line 20) — `{ enableCodeLens, enableDecorations, enableEditorOverview }`.
  - `IDocumentMergeConflict` (line 26) — extends `IDocumentMergeConflictDescriptor`, adds `commitEdit(type, editor, edit?)` and `applyEdit(type, document, edit)`.
  - `IDocumentMergeConflictDescriptor` (line 31) — `{ range, current, incoming, commonAncestors[], splitter }`.
  - `IDocumentMergeConflictTracker` (line 39) — `{ getConflicts(doc), isPending(doc), forget(doc) }`.
  - `IDocumentMergeConflictTrackerService` (line 45) — `{ createTracker(origin), forget(doc) }`.
- **Control flow:** Pure type declarations, no runtime logic.
- **Data flow:** `IMergeRegion` is the atom of conflict geometry; `IDocumentMergeConflictDescriptor` aggregates regions; `IDocumentMergeConflict` adds mutation operations on top.
- **Dependencies:** `vscode` (for `Range`, `TextEditor`, `TextDocument`, `TextEditorEdit`).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/services.ts`

- **Role:** Central service orchestrator (`ServiceWrapper`). Instantiates all subsystems, calls `begin(config)` on each, and propagates configuration-change events.
- **Key symbols:**
  - `ServiceWrapper` class (line 16), implements `vscode.Disposable`.
  - `begin()` (line 27) — constructs `DocumentTracker`, `CommandHandler`, `CodeLensProvider`, `ContentProvider`, `Decorator` in that order; calls `begin(configuration)` on each.
  - `createExtensionConfiguration()` (line 55) — reads workspace configuration section `'merge-conflict'`, returns `IExtensionConfiguration`.
  - `ConfigurationSectionName = 'merge-conflict'` (line 14).
- **Control flow:**
  1. Constructor at line 21 reads `aiKey` from `packageJSON`, creates `TelemetryReporter`.
  2. `begin()` at line 27: creates config snapshot, instantiates all services into `this.services[]`, iterates calling `.begin(configuration)`.
  3. `vscode.workspace.onDidChangeConfiguration` at line 46 triggers `configurationUpdated(newConfig)` on each service.
  4. `dispose()` at line 67 calls `dispose()` on every service.
- **Data flow:** `IExtensionConfiguration` object flows from `createExtensionConfiguration()` → each service's `begin()` and `configurationUpdated()`. `DocumentTracker` instance flows into `CommandHandler`, `CodeLensProvider`, and `Decorator` constructors as the shared `IDocumentMergeConflictTrackerService`.
- **Dependencies:** `vscode`, `./documentTracker`, `./codelensProvider`, `./commandHandler`, `./contentProvider`, `./mergeDecorator`, `./interfaces`, `@vscode/extension-telemetry`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/mergeConflictParser.ts`

- **Role:** Stateless parser that scans a `vscode.TextDocument` line-by-line and emits `DocumentMergeConflict` objects for every `<<<<<<<`/`=======`/`>>>>>>>` block found.
- **Key symbols:**
  - Marker constants at lines 10-13: `startHeaderMarker='<<<<<<<'`, `commonAncestorsMarker='|||||||'`, `splitterMarker='======='`, `endFooterMarker='>>>>>>>'`.
  - `IScanMergedConflict` (line 15) — intermediate accumulator: `{ startHeader, commonAncestors[], splitter?, endFooter? }`.
  - `MergeConflictParser.scanDocument(document, telemetryReporter)` (line 24) — main static entry point.
  - `scanItemTolMergeConflictDescriptor(document, scanned)` (line 88) — converts a `IScanMergedConflict` to `IDocumentMergeConflictDescriptor`.
  - `containsConflict(document)` (line 145) — fast pre-check using `text.includes()`.
  - `shiftBackOneCharacter(document, range, unlessEqual)` (line 154) — adjusts a `Position` one character back to avoid decoration overlap at line boundaries.
- **Control flow:**
  1. `containsConflict` at line 145 checks for both `<<<<<<<` and `>>>>>>>` substrings before full scan.
  2. `scanDocument` at line 35 iterates lines 0..lineCount-1:
     - `startsWith('<<<<<<<')` → create new `IScanMergedConflict`, record `startHeader`.
     - `startsWith('|||||||')` while in conflict, no splitter yet → push to `commonAncestors[]`.
     - `=== '======='` while in conflict, no splitter yet → record `splitter`.
     - `startsWith('>>>>>>>')` while in conflict → record `endFooter`, call `scanItemTolMergeConflictDescriptor`, push result, reset `currentConflict = null`.
     - Nested `<<<<<<<` while already in conflict → abort parse entirely (line 45-51).
  3. Results filtered and mapped to `new DocumentMergeConflict(descriptor, telemetryReporter)` at line 83-85.
- **Data flow:** `vscode.TextDocument` → line iteration → `IScanMergedConflict` accumulator → `IDocumentMergeConflictDescriptor` (with computed `vscode.Range` geometry) → `DocumentMergeConflict[]`.
- **Dependencies:** `vscode`, `./interfaces`, `./documentMergeConflict`, `@vscode/extension-telemetry`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/documentTracker.ts`

- **Role:** Caching scan coordinator (`DocumentMergeConflictTracker`). Coalesces concurrent scan requests from multiple consumers (codelens, decorator, commands) for the same document via `ScanTask`/`Delayer`, deduplicates telemetry, and supports per-origin tracker instances.
- **Key symbols:**
  - `ScanTask` (line 12) — holds a `Set<string>` of origins and a `Delayer<IDocumentMergeConflict[]>`.
  - `OriginDocumentMergeConflictTracker` (line 30) — lightweight façade bound to a specific `origin` string; delegates all calls to the parent `DocumentMergeConflictTracker`.
  - `DocumentMergeConflictTracker` (line 47) — implements `IDocumentMergeConflictTrackerService`; `cache: Map<string, ScanTask>` keyed by `document.uri.toString()`.
  - `getConflicts(document, origin)` (line 53) — cache lookup → create/update `ScanTask` → trigger `Delayer` → invoke `getConflictsOrEmpty`.
  - `createTracker(origin)` (line 99) — factory method returning an `OriginDocumentMergeConflictTracker`.
  - `seenDocumentsWithConflicts: Set<string>` (line 115) — prevents duplicate telemetry events per document URI.
  - `getConflictsOrEmpty(document, origins)` (line 117) — calls `MergeConflictParser.containsConflict` then `MergeConflictParser.scanDocument`; sends `'mergeMarkers.documentWithConflictMarkersOpened'` telemetry at line 139.
  - `delayExpireTime = 0` (line 49) — zero delay means coalescing is purely synchronous within the same event loop turn.
- **Control flow:**
  1. Consumer calls `tracker.getConflicts(document)` on their `OriginDocumentMergeConflictTracker`.
  2. Delegates to parent `getConflicts(document, origin)`.
  3. Checks cache; if missing, creates `ScanTask` with `Delayer(0)`.
  4. Calls `delayTask.trigger(callback)`, which schedules `setTimeout(0)`.
  5. Subsequent calls within same tick add their `origin` to the existing `ScanTask`.
  6. On timer fire, `getConflictsOrEmpty` is called once for all origins, cache entry is deleted, promise resolves to all consumers.
- **Data flow:** `vscode.TextDocument` → cache key (URI string) → `ScanTask` → `Delayer.trigger()` → `MergeConflictParser.scanDocument()` → `IDocumentMergeConflict[]` (resolved to all waiting callers).
- **Dependencies:** `vscode`, `./mergeConflictParser`, `./interfaces`, `./delayer`, `@vscode/extension-telemetry`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/documentMergeConflict.ts`

- **Role:** Concrete model class for a single merge conflict. Holds parsed geometry and provides the two edit operations (`commitEdit`, `applyEdit`) that rewrite the conflict block in the document.
- **Key symbols:**
  - `DocumentMergeConflict` (line 9) — implements `IDocumentMergeConflict`.
  - `applied: boolean = false` (line 16) — idempotency guard; `applyEdit` becomes a no-op after first call.
  - `commitEdit(type, editor, edit?)` (line 26) — sends `'mergeMarkers.accept'` telemetry at line 45, then either calls `applyEdit` directly with a provided `TextEditorEdit` or opens a new `editor.edit()` callback.
  - `applyEdit(type, document, edit)` (line 56) — the core mutation:
    - `CommitType.Current` (line 71): replaces `this.range` with `document.getText(this.current.content)`.
    - `CommitType.Incoming` (line 76): replaces `this.range` with `document.getText(this.incoming.content)`.
    - `CommitType.Both` (line 80): replaces `this.range` with `currentContent + incomingContent` via `edit.replace`.
  - `replaceRangeWithContent(content, edit)` (line 90) — if content is newline-only, replaces with empty string; otherwise replaces `this.range` with content.
  - `isNewlineOnly(text)` (line 100) — checks for `'\n'` or `'\r\n'`.
- **Control flow:** `commitEdit` → check `edit` argument → `applyEdit` → check `applied` → branch on `CommitType` → `edit.replace(this.range, resolvedContent)`.
- **Data flow:** Constructor receives `IDocumentMergeConflictDescriptor` (ranges) and `TelemetryReporter`. `applyEdit` reads text out of `document` via the stored `Range` objects, then writes back through the provided `edit` object (either a `TextEditorEdit` or a `WorkspaceEdit`-compatible interface).
- **Dependencies:** `./interfaces`, `vscode`, `@vscode/extension-telemetry`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/commandHandler.ts`

- **Role:** Registers and handles the 10 VS Code commands exposed by the extension (accept current/incoming/both, accept-all variants, navigate next/previous, compare, accept-selection).
- **Key symbols:**
  - `CommandHandler` (line 19) — creates its own `OriginDocumentMergeConflictTracker` via `trackerService.createTracker('commands')` at line 25.
  - `begin()` (line 28) — registers 10 commands via `registerTextEditorCommand`.
  - `registerTextEditorCommand(command, cb, resourceCB?)` (line 43) — wraps `vscode.commands.registerCommand`; dispatches to `resourceCB` if args are source control resource objects, otherwise to `cb` with the active text editor.
  - `accept(type, editor, ...args)` (line 228) — resolves conflict from cursor position or `'known-conflict'` arg, calls `tracker.forget(editor.document)` then `conflict.commitEdit(type, editor)`; optionally calls `navigateNext` if `autoNavigateNextConflict.enabled`.
  - `acceptAll(type, editor)` (line 258) — fetches all conflicts, calls `editor.edit()` to apply all via `conflict.applyEdit()` in one batch.
  - `acceptAllResources(type, resources)` (line 275) — opens each URI as document, builds a single `WorkspaceEdit`, applies via `vscode.workspace.applyEdit`.
  - `compare(editor, conflict)` (line 85) — builds two virtual URIs with `scheme='merge-conflict.conflict-diff'` and JSON-encoded range queries, then calls `vscode.commands.executeCommand('vscode.diff', leftUri, rightUri, title, opts)`.
  - `navigate(editor, direction)` (line 202) — calls `findConflictForNavigation`, then sets `editor.selection` and calls `editor.revealRange`.
  - `findConflictContainingSelection(editor, conflicts?)` (line 296) — iterates conflicts to find one whose `range.contains(editor.selection.active)`.
  - `findConflictForNavigation(editor, direction, conflicts?)` (line 315) — uses a predicate and fallback to find the next/previous conflict relative to cursor position; reverses the conflict list for backward navigation.
- **Control flow:** Command registration → user gesture → `registerTextEditorCommand` closure → `accept` / `acceptAll` / `compare` / `navigate` → `tracker.getConflicts` → `conflict.commitEdit` or `conflict.applyEdit`.
- **Data flow:** `IDocumentMergeConflictTracker.getConflicts(document)` → `IDocumentMergeConflict[]` → selection matching → `commitEdit`/`applyEdit` writes to editor.
- **Dependencies:** `vscode`, `./interfaces`, `./contentProvider`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/codelensProvider.ts`

- **Role:** `vscode.CodeLensProvider` implementation. Produces four `CodeLens` items (Accept Current, Accept Incoming, Accept Both, Compare Changes) anchored to the start line of each conflict's range.
- **Key symbols:**
  - `MergeConflictCodeLensProvider` (line 9) — implements `CodeLensProvider` and `Disposable`.
  - `begin(config)` (line 18) — stores config; conditionally calls `registerCodeLensProvider()`.
  - `configurationUpdated(updatedConfig)` (line 26) — disposes or re-registers provider handle based on `enableCodeLens` change.
  - `provideCodeLenses(document, _token)` (line 47) — async; fetches conflicts from tracker, calls `setContext('mergeConflictsCount', conflictsCount)` at line 55, then builds 4 `CodeLens` per conflict.
  - `registerCodeLensProvider()` (line 100) — calls `vscode.languages.registerCodeLensProvider` for schemes `file`, `vscode-vfs`, `untitled`, `vscode-userdata`.
  - Each `CodeLens` uses `document.lineAt(conflict.range.start.line).range` (line 88) as the display range.
  - Commands carry `arguments: ['known-conflict', conflict]` so `CommandHandler.accept` can bypass cursor detection.
- **Control flow:** VS Code calls `provideCodeLenses` → tracker returns conflicts → 4 lenses built per conflict → returned to VS Code for display above the `<<<<<<<` line.
- **Data flow:** `IDocumentMergeConflictTracker.getConflicts(document)` → `IDocumentMergeConflict[]` → `vscode.CodeLens[]` with embedded conflict references in `arguments`.
- **Dependencies:** `vscode`, `./interfaces`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/mergeDecorator.ts`

- **Role:** Applies `TextEditorDecorationType` highlights to all visible editors whenever documents open, change, or editors change visibility. Decorates headers, content blocks, splitter, and overview ruler lanes.
- **Key symbols:**
  - `MergeDecorator` (line 9) — holds `decorations: { [key: string]: TextEditorDecorationType }` map and `updating: Map<TextEditor, boolean>` re-entrancy guard.
  - `begin(config)` (line 23) — registers decorations, iterates `vscode.window.visibleTextEditors` to apply initial decorations, subscribes to `onDidOpenTextDocument`, `onDidChangeTextDocument`, `onDidChangeVisibleTextEditors`.
  - `registerDecorationTypes(config)` (line 55) — disposes existing decorations then recreates them via `vscode.window.createTextEditorDecorationType`. Keys: `'current.content'`, `'incoming.content'`, `'commonAncestors.content'`, `'current.header'`, `'commonAncestors.header'`, `'splitter'`, `'incoming.header'`. Theme color tokens used: `merge.currentContentBackground`, `merge.incomingContentBackground`, `merge.commonContentBackground`, `merge.currentHeaderBackground`, `merge.incomingHeaderBackground`, `merge.commonHeaderBackground`, `merge.border`, `editorOverviewRuler.currentContentForeground`, `editorOverviewRuler.incomingContentForeground`, `editorOverviewRuler.commonContentForeground`.
  - `applyDecorations(editor)` (line 163) — async; checks `updating` guard, gets conflicts from tracker, builds `matchDecorations: { [key: string]: Range[] }`, then calls `editor.setDecorations(decorationType, ranges[])` for each key.
  - `generateBlockRenderOptions(backgroundColor, overviewRulerColor, config)` (line 137) — conditionally sets `backgroundColor` and `overviewRulerColor`/`overviewRulerLane` based on config flags.
  - `removeDecorations(editor)` (line 238) — clears all decoration ranges by calling `editor.setDecorations(type, [])` for each type.
  - `after.contentText` at lines 90 and 120 — appends localized `"(Current Change)"` / `"(Incoming Change)"` text to header lines.
- **Control flow:** Event subscription → `applyDecorationsFromEvent` → find editors with matching document → `applyDecorations(editor)` → tracker.getConflicts → build range map → `editor.setDecorations`.
- **Data flow:** `IDocumentMergeConflict[]` → per-conflict iteration extracts `.current.decoratorContent`, `.incoming.decoratorContent`, `.commonAncestors[].decoratorContent`, `.current.header`, `.splitter`, `.incoming.header` → grouped by decoration key → applied to editor.
- **Dependencies:** `vscode`, `./interfaces`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/contentProvider.ts`

- **Role:** `vscode.TextDocumentContentProvider` that synthesizes virtual document content for the diff view. Strips out conflict markers, replacing each conflict block with only the chosen side's content.
- **Key symbols:**
  - `MergeConflictContentProvider` (line 8) — implements `TextDocumentContentProvider` and `Disposable`.
  - `scheme = 'merge-conflict.conflict-diff'` (line 10) — registered with `vscode.workspace.registerTextDocumentContentProvider`.
  - `begin()` (line 15) — registers the provider.
  - `provideTextDocumentContent(uri)` (line 24) — parses `uri.query` as JSON `{ scheme, ranges: [conflictRange, fullRange][] }`, opens the real document via `uri.with({ scheme, query: '' })`, then iterates range pairs to assemble the output text:
    - Emits text from `lastPosition` to `fullRange.start` (content before the conflict block).
    - Emits text from `conflictRange.start` to `conflictRange.end` (the chosen side's content).
    - Advances `lastPosition` to `fullRange.end`.
    - Appends remaining text after the last conflict.
- **Control flow:** `vscode.diff` command requests URIs with `merge-conflict.conflict-diff` scheme → VS Code calls `provideTextDocumentContent` for each side → content assembled → shown in diff editor.
- **Data flow:** URI query JSON `{ scheme, ranges[] }` → `vscode.workspace.openTextDocument(original)` → `document.getText(ranges)` → string concatenation → virtual document text.
- **Dependencies:** `vscode`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/merge-conflict/src/delayer.ts`

- **Role:** Generic debounce/coalescing utility. A `Delayer<T>` defers execution of a task until a configurable timeout expires. Multiple calls to `trigger` within the delay window update the pending task and reset/share the same promise.
- **Key symbols:**
  - `Delayer<T>` (line 10) — holds `timeout`, `completionPromise`, `onSuccess`, `task`.
  - `trigger(task, delay?)` (line 26) — replaces `this.task`, creates `completionPromise` if absent (via a resolver-captured Promise chain), sets a new `setTimeout`. Returns the shared `completionPromise`.
  - `forceDelivery()` (line 54) — cancels the timeout and immediately resolves the pending promise.
  - `cancel()` (line 68) — cancels and nulls the promise.
  - `cancelTimeout()` (line 73) — internal helper calling `clearTimeout`.
- **Control flow:** `trigger` called → if no existing promise, create one capturing `resolve` as `onSuccess` → chain `.then` that calls `this.task()` → `setTimeout` fires → `onSuccess(undefined)` → `.then` callback executes task, returns result to all awaiting consumers.
- **Data flow:** Any `ITask<T>` callback → coalesced into a single `Promise<T>` resolved after delay.
- **Dependencies:** None (pure TypeScript).

---

### Cross-Cutting Synthesis

The `merge-conflict` extension is a tightly layered pipeline: `mergeConflictMain.ts` bootstraps `ServiceWrapper` (services.ts), which acts as a DI root — constructing one `DocumentMergeConflictTracker` and passing it as an `IDocumentMergeConflictTrackerService` to three consumers (`CommandHandler`, `CodeLensProvider`, `MergeDecorator`). Each consumer calls `trackerService.createTracker(origin)` to get an origin-tagged `OriginDocumentMergeConflictTracker` façade. All three then call `tracker.getConflicts(document)`, which funnels through `DocumentMergeConflictTracker.getConflicts` into a shared `ScanTask`/`Delayer` coalescing layer, preventing redundant parses within the same event loop tick. The actual parsing is performed synchronously by `MergeConflictParser.scanDocument`, which walks document lines detecting the four markers (`<<<<<<<`, `|||||||`, `=======`, `>>>>>>>`) and builds `IDocumentMergeConflictDescriptor` geometry objects containing six distinct `vscode.Range` values per conflict (range, current.header, current.content, current.decoratorContent, splitter, incoming.*). These are wrapped in `DocumentMergeConflict` instances that hold the edit logic: `applyEdit` performs a single `edit.replace(this.range, chosenContent)` substitution. The CodeLens layer embeds conflict object references in command arguments so the command handler can bypass cursor-based detection. The Decorator layer maps conflict geometry to named `TextEditorDecorationType` keys driven by `ThemeColor` tokens. The Content Provider synthesizes virtual documents for the diff view by re-assembling the original file text with only the chosen side's ranges included. Porting this to Tauri/Rust requires replacing all seven VS Code API surfaces: `TextDocument`/`TextEditor` (text buffer access), `TextEditorEdit`/`WorkspaceEdit` (transactional edits), `CodeLensProvider` (line-level UI widgets), `TextEditorDecorationType` (range-based background/overlay decorations with overview ruler support), `TextDocumentContentProvider` (virtual URI scheme for diff views), `commands.registerCommand` (command registry), and `TelemetryReporter` (event reporting).

---

### Out-of-Partition References

- `@vscode/extension-telemetry` — npm package used for `TelemetryReporter`; all telemetry call sites are in `documentMergeConflict.ts:45`, `documentTracker.ts:139`.
- `vscode.commands.executeCommand('vscode.diff', ...)` at `commandHandler.ts:147` — invokes VS Code's built-in diff editor command.
- `vscode.commands.executeCommand('workbench.action.newGroupBelow')` at `commandHandler.ts:144` — invokes workbench command to open a split editor pane.
- `vscode.commands.executeCommand('setContext', 'mergeConflictsCount', n)` at `codelensProvider.ts:55` — sets a context key consumed by keybinding `when` clauses defined in `package.json`.
- `package.json` (not in src/) — defines `aiKey` for telemetry, command IDs registered via `contributes.commands`, configuration schema under `merge-conflict.*`, and `activationEvents`.
- `vscode.l10n.t(...)` — localization API used in `commandHandler.ts` (warning messages, diff title) and `mergeDecorator.ts` (after-text labels); localization strings live in `l10n/` bundle files outside this partition.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Merge-Conflict Extension: Porting Patterns & API Usage

**Research Partition**: 32 of 80  
**Scope**: `extensions/merge-conflict/` (13 files, 1,463 LOC)  
**Focus**: CodeLens API usage and core IDE functionality patterns used in VS Code extensions

---

## Core Patterns Found

#### Pattern: CodeLens Provider Registration
**Where:** `extensions/merge-conflict/src/codelensProvider.ts:100-107`
**What:** Registers a CodeLens provider for multiple document schemes with dynamic lifecycle management.
```typescript
private registerCodeLensProvider() {
	this.codeLensRegistrationHandle = vscode.languages.registerCodeLensProvider([
		{ scheme: 'file' },
		{ scheme: 'vscode-vfs' },
		{ scheme: 'untitled' },
		{ scheme: 'vscode-userdata' },
	], this);
}
```
**Variations / call-sites:**
- `codelensProvider.ts:21-23` - Registration during `begin()` initialization
- `codelensProvider.ts:28-34` - Dynamic re-registration on configuration change
- Implementation disposal: `codelensProvider.ts:40-45`

---

#### Pattern: CodeLens Provision & Command Binding
**Where:** `extensions/merge-conflict/src/codelensProvider.ts:47-98`
**What:** Implements `vscode.CodeLensProvider` interface to provide code lenses with associated commands for each conflict range.
```typescript
async provideCodeLenses(document: vscode.TextDocument, _token: vscode.CancellationToken): Promise<vscode.CodeLens[] | null> {
	if (!this.config || !this.config.enableCodeLens) {
		return null;
	}

	const conflicts = await this.tracker.getConflicts(document);
	const conflictsCount = conflicts?.length ?? 0;
	vscode.commands.executeCommand('setContext', 'mergeConflictsCount', conflictsCount);

	if (!conflictsCount) {
		return null;
	}

	const items: vscode.CodeLens[] = [];
	conflicts.forEach(conflict => {
		const acceptCurrentCommand: vscode.Command = {
			command: 'merge-conflict.accept.current',
			title: vscode.l10n.t("Accept Current Change"),
			arguments: ['known-conflict', conflict]
		};
		// ... more commands
		items.push(
			new vscode.CodeLens(range, acceptCurrentCommand),
			new vscode.CodeLens(range, acceptIncomingCommand),
			new vscode.CodeLens(range, acceptBothCommand),
			new vscode.CodeLens(range, diffCommand)
		);
	});
	return items;
}
```
**Variations / call-sites:** Single implementation in extension; each conflict may have 4 associated code lenses

---

#### Pattern: Command Handler Registration
**Where:** `extensions/merge-conflict/src/commandHandler.ts:28-51`
**What:** Registers text editor commands with dual callback support for editor context and resource URI arguments.
```typescript
begin() {
	this.disposables.push(
		this.registerTextEditorCommand('merge-conflict.accept.current', this.acceptCurrent),
		this.registerTextEditorCommand('merge-conflict.accept.incoming', this.acceptIncoming),
		this.registerTextEditorCommand('merge-conflict.accept.selection', this.acceptSelection),
		this.registerTextEditorCommand('merge-conflict.accept.both', this.acceptBoth),
		this.registerTextEditorCommand('merge-conflict.accept.all-current', this.acceptAllCurrent, this.acceptAllCurrentResources),
		this.registerTextEditorCommand('merge-conflict.accept.all-incoming', this.acceptAllIncoming, this.acceptAllIncomingResources),
		this.registerTextEditorCommand('merge-conflict.accept.all-both', this.acceptAllBoth),
		this.registerTextEditorCommand('merge-conflict.next', this.navigateNext),
		this.registerTextEditorCommand('merge-conflict.previous', this.navigatePrevious),
		this.registerTextEditorCommand('merge-conflict.compare', this.compare)
	);
}

private registerTextEditorCommand(command: string, cb: (editor: vscode.TextEditor, ...args: any[]) => Promise<void>, resourceCB?: (uris: vscode.Uri[]) => Promise<void>) {
	return vscode.commands.registerCommand(command, (...args) => {
		if (resourceCB && args.length && args.every(arg => arg && arg.resourceUri)) {
			return resourceCB.call(this, args.map(arg => arg.resourceUri));
		}
		const editor = vscode.window.activeTextEditor;
		return editor && cb.call(this, editor, ...args);
	});
}
```
**Variations / call-sites:** 10 commands registered with polymorphic callback dispatch

---

#### Pattern: Text Decoration Type Management
**Where:** `extensions/merge-conflict/src/mergeDecorator.ts:55-125`
**What:** Creates and manages themed text editor decoration types with color/style overrides, stored by key for reuse.
```typescript
private registerDecorationTypes(config: interfaces.IExtensionConfiguration) {
	Object.keys(this.decorations).forEach(k => this.decorations[k].dispose());
	this.decorations = {};

	if (!config.enableDecorations || !config.enableEditorOverview) {
		return;
	}

	if (config.enableDecorations || config.enableEditorOverview) {
		this.decorations['current.content'] = vscode.window.createTextEditorDecorationType(
			this.generateBlockRenderOptions('merge.currentContentBackground', 'editorOverviewRuler.currentContentForeground', config)
		);
		this.decorations['incoming.content'] = vscode.window.createTextEditorDecorationType(
			this.generateBlockRenderOptions('merge.incomingContentBackground', 'editorOverviewRuler.incomingContentForeground', config)
		);
	}

	if (config.enableDecorations) {
		this.decorations['current.header'] = vscode.window.createTextEditorDecorationType({
			isWholeLine: this.decorationUsesWholeLine,
			backgroundColor: new vscode.ThemeColor('merge.currentHeaderBackground'),
			color: new vscode.ThemeColor('editor.foreground'),
			outlineStyle: 'solid',
			outlineWidth: '1pt',
			outlineColor: new vscode.ThemeColor('merge.border'),
			after: {
				contentText: ' ' + vscode.l10n.t("(Current Change)"),
				color: new vscode.ThemeColor('descriptionForeground')
			}
		});
	}
}
```
**Variations / call-sites:**
- `mergeDecorator.ts:23-26` - Initialization in `begin()`
- `mergeDecorator.ts:44-53` - Re-registration on config change
- Applied to editors via: `mergeDecorator.ts:225-231`

---

#### Pattern: Document Event Listener Registration
**Where:** `extensions/merge-conflict/src/mergeDecorator.ts:23-42`
**What:** Subscribes to workspace/window events with callback handlers, storing disposables in context subscriptions.
```typescript
begin(config: interfaces.IExtensionConfiguration) {
	this.config = config;
	this.registerDecorationTypes(config);

	vscode.window.visibleTextEditors.forEach(e => this.applyDecorations(e));

	vscode.workspace.onDidOpenTextDocument(event => {
		this.applyDecorationsFromEvent(event);
	}, null, this.context.subscriptions);

	vscode.workspace.onDidChangeTextDocument(event => {
		this.applyDecorationsFromEvent(event.document);
	}, null, this.context.subscriptions);

	vscode.window.onDidChangeVisibleTextEditors((e) => {
		e.forEach(e => this.applyDecorations(e));
	}, null, this.context.subscriptions);
}
```
**Variations / call-sites:**
- `codelensProvider.ts` - No explicit event subscriptions (relies on provider callbacks)
- `services.ts:46-52` - Workspace configuration change monitoring

---

#### Pattern: Content Provider Registration
**Where:** `extensions/merge-conflict/src/contentProvider.ts:15-19`
**What:** Registers a custom URI scheme content provider for displaying synthetic documents (diff views).
```typescript
begin() {
	this.context.subscriptions.push(
		vscode.workspace.registerTextDocumentContentProvider(MergeConflictContentProvider.scheme, this)
	);
}
```
**Implementation interface:** Implements `vscode.TextDocumentContentProvider` with `provideTextDocumentContent(uri)` method
- Provider supplies text content for custom URIs: `contentProvider.ts:24-53`
- Used by diff command: `commandHandler.ts:111-118` creates custom URIs with JSON query parameters

---

#### Pattern: Service Lifecycle & Configuration Management
**Where:** `extensions/merge-conflict/src/services.ts:27-53`
**What:** Centralizes service initialization, configuration propagation, and lifecycle events through a ServiceWrapper.
```typescript
begin() {
	const configuration = this.createExtensionConfiguration();
	const documentTracker = new DocumentTracker(this.telemetryReporter);

	this.services.push(
		documentTracker,
		new CommandHandler(documentTracker),
		new CodeLensProvider(documentTracker),
		new ContentProvider(this.context),
		new Decorator(this.context, documentTracker),
	);

	this.services.forEach((service: any) => {
		if (service.begin && service.begin instanceof Function) {
			service.begin(configuration);
		}
	});

	vscode.workspace.onDidChangeConfiguration(() => {
		this.services.forEach((service: any) => {
			if (service.configurationUpdated && service.configurationUpdated instanceof Function) {
				service.configurationUpdated(this.createExtensionConfiguration());
			}
		});
	});
}
```
**Configuration pattern:**
```typescript
createExtensionConfiguration(): interfaces.IExtensionConfiguration {
	const workspaceConfiguration = vscode.workspace.getConfiguration(ConfigurationSectionName);
	const codeLensEnabled: boolean = workspaceConfiguration.get('codeLens.enabled', true);
	const decoratorsEnabled: boolean = workspaceConfiguration.get('decorators.enabled', true);

	return {
		enableCodeLens: codeLensEnabled,
		enableDecorations: decoratorsEnabled,
		enableEditorOverview: decoratorsEnabled
	};
}
```
**Variations / call-sites:**
- All services implement optional `begin(config)` and `configurationUpdated(config)` hooks
- Entry point: `mergeConflictMain.ts:9-14` activates via `activate(context)` extension API

---

## API Surface Summary

### vscode.languages API
- **`registerCodeLensProvider()`** - Bind CodeLens provider to document schemes
- **`CodeLensProvider` interface** - Implement `provideCodeLenses(document, token)`

### vscode.commands API
- **`registerCommand(id, callback)`** - Register command with variadic arguments
- **`executeCommand(command, ...args)`** - Execute command (used for `setContext`)

### vscode.window API
- **`visibleTextEditors`** - Array of currently visible editors
- **`activeTextEditor`** - Current editor with focus
- **`createTextEditorDecorationType(options)`** - Create reusable decoration style
- **`onDidChangeVisibleTextEditors()`** - Event fired when editor visibility changes

### vscode.workspace API
- **`registerTextDocumentContentProvider(scheme, provider)`** - Register custom URI scheme handler
- **`onDidOpenTextDocument()`** - Event for new documents
- **`onDidChangeTextDocument()`** - Event for document edits
- **`onDidChangeConfiguration()`** - Event for settings changes
- **`getConfiguration(section)`** - Read workspace/user settings
- **`openTextDocument(uri)`** - Open document by URI
- **`applyEdit(workspaceEdit)`** - Apply bulk edits across workspace

### vscode.TextDocument API
- **`lineAt(line)`** - Get TextLine at index
- **`getText(range?)`** - Extract text
- **`lineCount`** - Line count

### vscode.TextEditor API
- **`edit(callback)`** - Apply edits to document
- **`setDecorations(type, ranges)`** - Apply decoration type to ranges
- **`revealRange(range, type)`** - Scroll editor to show range
- **`selection`** - Current cursor selection

### vscode.Range & vscode.Position API
- **Constructors**: `new vscode.Range(startLine, startChar, endLine, endChar)`
- **Methods**: `contains()`, `isEqual()`, `start`, `end`

---

## Key Extension Patterns

1. **Provider Dual-Dispatch**: Commands support both editor context and resource URI context (commandHandler)
2. **Cached Conflict Detection**: Document merge conflict parsing cached with delayer to debounce updates (documentTracker)
3. **Decoration Type Pooling**: Decoration types created once, reused across all editors (mergeDecorator)
4. **Configuration Push Pattern**: Services implement `configurationUpdated()` hook for reactive config changes
5. **Custom URI Schemes**: Synthetic documents for diff views use JSON-encoded query parameters
6. **Telemetry Integration**: TelemetryReporter passed to core services for usage tracking

---

## Porting Implications for Tauri/Rust

**UI/Decoration Layer**: Complex theme color resolution and decoration API would require:
- Tauri webview CSS-in-JS or style injection layer
- Theme color token resolution system
- Range-to-coordinate mapping for decoration rendering

**CodeLens/Commands**: Would need equivalent IPC layer:
- Command registration -> RPC handler mapping
- CodeLens provider callbacks -> Async request/response protocol
- Context variable management system

**Document/Event Streaming**: Real-time conflict detection requires:
- Efficient document diff computation
- Event subscription mechanism over IPC
- Debounced update strategy (Delayer pattern used here)

**Content Providers**: Custom URI schemes need:
- URI scheme routing layer
- Synthetic document generation protocol
- Query parameter handling in Rust layer

**Configuration**: Settings propagation pattern would map to:
- Workspace/User config file reading
- Change notification system
- Reactive service initialization

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
