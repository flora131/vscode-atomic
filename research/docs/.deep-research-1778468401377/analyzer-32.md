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
