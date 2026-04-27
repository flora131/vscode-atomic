# Analyzer 32: extensions/merge-conflict/

## Files Analysed

1. `/Users/norinlavaee/vscode-atomic/extensions/merge-conflict/src/mergeConflictMain.ts`
2. `/Users/norinlavaee/vscode-atomic/extensions/merge-conflict/src/interfaces.ts`
3. `/Users/norinlavaee/vscode-atomic/extensions/merge-conflict/src/services.ts`
4. `/Users/norinlavaee/vscode-atomic/extensions/merge-conflict/src/mergeConflictParser.ts`
5. `/Users/norinlavaee/vscode-atomic/extensions/merge-conflict/src/documentTracker.ts`
6. `/Users/norinlavaee/vscode-atomic/extensions/merge-conflict/src/documentMergeConflict.ts`
7. `/Users/norinlavaee/vscode-atomic/extensions/merge-conflict/src/codelensProvider.ts`
8. `/Users/norinlavaee/vscode-atomic/extensions/merge-conflict/src/commandHandler.ts`
9. `/Users/norinlavaee/vscode-atomic/extensions/merge-conflict/src/mergeDecorator.ts`
10. `/Users/norinlavaee/vscode-atomic/extensions/merge-conflict/src/contentProvider.ts`
11. `/Users/norinlavaee/vscode-atomic/extensions/merge-conflict/src/delayer.ts`

---

## Per-File Notes

### `extensions/merge-conflict/src/mergeConflictMain.ts`

- **Role:** Extension entry point. Bootstraps the service layer and attaches it to VS Code's extension lifecycle.
- **Key symbols:**
  - `activate` (`mergeConflictMain.ts:9`) — VS Code lifecycle hook; instantiates `MergeConflictServices` and calls `services.begin()`.
  - `deactivate` (`mergeConflictMain.ts:16`) — empty stub; cleanup is fully handled via the `Disposable` pattern on the `services` object.
- **Control flow:** `activate` → `new MergeConflictServices(context)` → `services.begin()` → `context.subscriptions.push(services)`. All cleanup is delegated to the disposable chain.
- **Data flow:** The `vscode.ExtensionContext` object is passed into `MergeConflictServices` and then forwarded to child services. No data transformation occurs at this layer.
- **Dependencies:** `vscode` (VS Code API), `./services`.

---

### `extensions/merge-conflict/src/interfaces.ts`

- **Role:** Central type contract for the entire extension. Defines all shared interfaces and enums used across parsing, tracking, display, and command layers.
- **Key symbols:**
  - `IMergeRegion` (`interfaces.ts:7`) — describes one side of a conflict: `name`, `header` (a `vscode.Range` for the marker line), `content` (range of the text content), `decoratorContent` (range shifted back one character to avoid overlap with adjacent decorations).
  - `CommitType` (`interfaces.ts:14`) — const enum with values `Current`, `Incoming`, `Both` used by all resolution paths.
  - `IExtensionConfiguration` (`interfaces.ts:20`) — flat bag of three booleans: `enableCodeLens`, `enableDecorations`, `enableEditorOverview`.
  - `IDocumentMergeConflict` (`interfaces.ts:26`) — extends `IDocumentMergeConflictDescriptor` and adds two mutation methods: `commitEdit` and `applyEdit`.
  - `IDocumentMergeConflictDescriptor` (`interfaces.ts:31`) — pure data shape: `range`, `current: IMergeRegion`, `incoming: IMergeRegion`, `commonAncestors: IMergeRegion[]`, `splitter: vscode.Range`.
  - `IDocumentMergeConflictTracker` (`interfaces.ts:39`) — consumer-facing tracker: `getConflicts`, `isPending`, `forget`.
  - `IDocumentMergeConflictTrackerService` (`interfaces.ts:45`) — factory for creating per-origin trackers via `createTracker(origin: string)`.
- **Control flow:** Pure type declarations; no runtime code.
- **Data flow:** Types flow from `interfaces.ts` outward to every other module.
- **Dependencies:** `vscode` only.

---

### `extensions/merge-conflict/src/services.ts`

- **Role:** Service orchestrator and composition root. Creates all subsystems, calls `begin()` on each, and wires configuration change propagation.
- **Key symbols:**
  - `ServiceWrapper` (`services.ts:16`) — the class exported as default; holds the `services: vscode.Disposable[]` array.
  - `begin` (`services.ts:27`) — constructs `DocumentTracker`, `CommandHandler`, `CodeLensProvider`, `ContentProvider`, and `Decorator`, pushes them all into `this.services`, then calls `begin(configuration)` on each that supports it. Also registers a `vscode.workspace.onDidChangeConfiguration` listener at line 46 that calls `configurationUpdated(config)` on every service that has that method.
  - `createExtensionConfiguration` (`services.ts:55`) — reads `merge-conflict.codeLens.enabled` and `merge-conflict.decorators.enabled` from workspace config to produce an `IExtensionConfiguration` value object.
  - `ConfigurationSectionName` (`services.ts:14`) — constant `'merge-conflict'` used as the workspace config namespace.
- **Control flow:** `begin()` reads configuration → creates subsystems → calls `begin(config)` on each → registers config listener. `dispose()` tears down all services in order.
- **Data flow:** `IExtensionConfiguration` produced in `createExtensionConfiguration` flows to every service's `begin()` and `configurationUpdated()` methods. `DocumentTracker` is injected into `CommandHandler`, `CodeLensProvider`, and `Decorator` as an `IDocumentMergeConflictTrackerService`.
- **Dependencies:** `vscode`, `./documentTracker`, `./codelensProvider`, `./commandHandler`, `./contentProvider`, `./mergeDecorator`, `./interfaces`, `@vscode/extension-telemetry`.

---

### `extensions/merge-conflict/src/mergeConflictParser.ts`

- **Role:** Stateless scanner that tokenizes document text into `IDocumentMergeConflict[]` by detecting the four conflict markers.
- **Key symbols:**
  - Marker constants `startHeaderMarker = '<<<<<<<'` (`mergeConflictParser.ts:10`), `commonAncestorsMarker = '|||||||'` (`mergeConflictParser.ts:11`), `splitterMarker = '======='` (`mergeConflictParser.ts:12`), `endFooterMarker = '>>>>>>>'` (`mergeConflictParser.ts:13`).
  - `IScanMergedConflict` (`mergeConflictParser.ts:15`) — internal accumulator holding lines for the four boundary tokens; not exported.
  - `MergeConflictParser.scanDocument` (`mergeConflictParser.ts:24`) — static method; iterates all lines of a `vscode.TextDocument`, builds `IScanMergedConflict` states, converts completed states to `IDocumentMergeConflictDescriptor` via `scanItemTolMergeConflictDescriptor`, and wraps each in `new DocumentMergeConflict(descriptor, telemetryReporter)`.
  - `MergeConflictParser.scanItemTolMergeConflictDescriptor` (`mergeConflictParser.ts:88`) — private static; constructs the final descriptor by computing all `vscode.Range` values. Uses `rangeIncludingLineBreak.end` as start points so line-break characters are excluded from content ranges.
  - `MergeConflictParser.containsConflict` (`mergeConflictParser.ts:145`) — quick-check using `text.includes()` on raw document text; avoids line-by-line scan when no markers are present.
  - `MergeConflictParser.shiftBackOneCharacter` (`mergeConflictParser.ts:154`) — private static; adjusts a `vscode.Position` one character leftward to prevent double-coloring of the boundary between current-content and the splitter decorations.
- **Control flow:** `scanDocument` drives a single-pass linear scan. On seeing `<<<<<<<` a new `IScanMergedConflict` is opened. On `|||||||` common-ancestor lines are accumulated. On `=======` the splitter is recorded. On `>>>>>>>` the conflict is closed and converted. If a second `<<<<<<<` appears before a `>>>>>>>`, parsing aborts at that point (`mergeConflictParser.ts:46-51`).
- **Data flow:** Raw `vscode.TextDocument` → `IScanMergedConflict[]` (ephemeral, per-document scan state) → `IDocumentMergeConflictDescriptor[]` → `IDocumentMergeConflict[]` (returned).
- **Dependencies:** `vscode`, `./interfaces`, `./documentMergeConflict`, `@vscode/extension-telemetry`.

---

### `extensions/merge-conflict/src/documentTracker.ts`

- **Role:** Caching and debouncing layer between consumers (CodeLens, Decorator, CommandHandler) and the parser. Implements both `IDocumentMergeConflictTrackerService` (factory) and `IDocumentMergeConflictTracker` (consumer).
- **Key symbols:**
  - `ScanTask` (`documentTracker.ts:12`) — internal class holding a `Set<string>` of origin names and a `Delayer<IDocumentMergeConflict[]>`. Multiple origins requesting the same document within the debounce window share the same `Delayer`.
  - `OriginDocumentMergeConflictTracker` (`documentTracker.ts:30`) — thin wrapper that pairs a `DocumentMergeConflictTracker` with a fixed `origin` string, implementing `IDocumentMergeConflictTracker`.
  - `DocumentMergeConflictTracker` (`documentTracker.ts:47`) — main exported class; holds `cache: Map<string, ScanTask>` keyed by `document.uri.toString()` and `delayExpireTime: number = 0` (effectively no delay).
  - `getConflicts` (`documentTracker.ts:53`) — looks up or creates a `ScanTask` for the document URI, registers the calling `origin`, then calls `cacheItem.delayTask.trigger(...)` which schedules `getConflictsOrEmpty`. After the task fires the cache entry is deleted (`documentTracker.ts:75`).
  - `getConflictsOrEmpty` (`documentTracker.ts:117`) — calls `MergeConflictParser.containsConflict` first; if false returns `[]` immediately. Otherwise calls `MergeConflictParser.scanDocument`. Fires a one-time telemetry event `mergeMarkers.documentWithConflictMarkersOpened` per document URI via `seenDocumentsWithConflicts` set (`documentTracker.ts:128`).
  - `createTracker` (`documentTracker.ts:99`) — factory method; returns a new `OriginDocumentMergeConflictTracker` bound to the given `origin` string.
  - `forget` (`documentTracker.ts:103`) — removes a document's cache entry, used before performing edit operations so the next read triggers a fresh parse.
- **Control flow:** Consumer calls `getConflicts(document)` → cache lookup → `Delayer.trigger()` → (after delay) → `getConflictsOrEmpty` → parser → result. Multiple consumers calling with the same document within the delay window share one scheduled parse.
- **Data flow:** `vscode.TextDocument` → `document.uri.toString()` (cache key) → `MergeConflictParser.scanDocument` → `IDocumentMergeConflict[]` returned to all waiting callers via the shared `Promise`.
- **Dependencies:** `vscode`, `./mergeConflictParser`, `./interfaces`, `./delayer`, `@vscode/extension-telemetry`.

---

### `extensions/merge-conflict/src/documentMergeConflict.ts`

- **Role:** Data model and mutation engine for a single conflict. Implements `IDocumentMergeConflict` by storing all ranges from the descriptor and providing edit application logic.
- **Key symbols:**
  - `DocumentMergeConflict` (`documentMergeConflict.ts:9`) — class with fields mirroring `IDocumentMergeConflictDescriptor` plus a private `applied: boolean = false` guard.
  - `commitEdit` (`documentMergeConflict.ts:26`) — fires telemetry event `mergeMarkers.accept` with a string resolution label, then either calls `applyEdit` directly if an `edit` token was provided or opens a new editor edit session via `editor.edit(...)`.
  - `applyEdit` (`documentMergeConflict.ts:56`) — checks and sets `this.applied` to prevent double-application; dispatches on `CommitType`: `Current` replaces `this.range` with `document.getText(this.current.content)`, `Incoming` replaces with `document.getText(this.incoming.content)`, `Both` concatenates current and incoming content texts and replaces `this.range` with the concatenation.
  - `replaceRangeWithContent` (`documentMergeConflict.ts:90`) — if the resolved content is purely `'\n'` or `'\r\n'` it replaces the range with an empty string to avoid leaving a lone newline; otherwise performs the range replacement.
  - `isNewlineOnly` (`documentMergeConflict.ts:100`) — checks if a string equals `'\n'` or `'\r\n'`.
- **Control flow:** `commitEdit` → `applyEdit` → `replaceRangeWithContent` (or direct `edit.replace` for `Both`). The `applied` flag makes `applyEdit` idempotent.
- **Data flow:** `IDocumentMergeConflictDescriptor` ranges are consumed by `document.getText(range)` to extract string content, then written back via `edit.replace(this.range, content)`.
- **Dependencies:** `./interfaces`, `vscode`, `@vscode/extension-telemetry`.

---

### `extensions/merge-conflict/src/codelensProvider.ts`

- **Role:** Implements `vscode.CodeLensProvider` to render inline action links ("Accept Current Change", "Accept Incoming Change", "Accept Both Changes", "Compare Changes") above each conflict's opening marker line.
- **Key symbols:**
  - `MergeConflictCodeLensProvider` (`codelensProvider.ts:9`) — class; holds `codeLensRegistrationHandle?: vscode.Disposable` and a `tracker: IDocumentMergeConflictTracker` created via `trackerService.createTracker('codelens')`.
  - `begin` (`codelensProvider.ts:18`) — stores config and conditionally calls `registerCodeLensProvider()` if `config.enableCodeLens`.
  - `configurationUpdated` (`codelensProvider.ts:26`) — dynamically disposes or re-registers the CodeLens provider registration handle when `enableCodeLens` changes.
  - `provideCodeLenses` (`codelensProvider.ts:47`) — called by VS Code per document; calls `this.tracker.getConflicts(document)`, then for each conflict emits four `vscode.CodeLens` instances attached to `document.lineAt(conflict.range.start.line).range`. Also calls `vscode.commands.executeCommand('setContext', 'mergeConflictsCount', conflictsCount)` at line 55 to update the context key used by when-clauses.
  - `registerCodeLensProvider` (`codelensProvider.ts:100`) — registers `this` against schemes `file`, `vscode-vfs`, `untitled`, and `vscode-userdata`.
- **Control flow:** VS Code triggers `provideCodeLenses` → tracker fetch (async) → iterate conflicts → produce `CodeLens[]`.
- **Data flow:** `IDocumentMergeConflict[]` from tracker → 4 `vscode.CodeLens` objects per conflict, each carrying a `vscode.Command` with argument `['known-conflict', conflict]` passed to `merge-conflict.accept.*` commands.
- **Dependencies:** `vscode`, `./interfaces`.

---

### `extensions/merge-conflict/src/commandHandler.ts`

- **Role:** Registers all user-facing commands and implements their resolution logic including single-conflict acceptance, bulk acceptance, cross-document bulk acceptance, cursor-based selection detection, and conflict navigation.
- **Key symbols:**
  - `CommandHandler` (`commandHandler.ts:19`) — class; tracker created as `trackerService.createTracker('commands')`.
  - `begin` (`commandHandler.ts:28`) — registers 10 commands: `merge-conflict.accept.current`, `.accept.incoming`, `.accept.selection`, `.accept.both`, `.accept.all-current`, `.accept.all-incoming`, `.accept.all-both`, `.next`, `.previous`, `.compare`.
  - `registerTextEditorCommand` (`commandHandler.ts:43`) — wrapper that checks if the call came from the SCM resource context (args with `.resourceUri`) and routes to a `resourceCB` if so; otherwise falls through to the normal editor callback.
  - `accept` (`commandHandler.ts:228`) — private; if args include `'known-conflict'` uses the passed conflict directly (fast path from CodeLens); otherwise calls `findConflictContainingSelection`. Calls `this.tracker.forget` before `conflict.commitEdit()`. If `autoNavigateNextConflict.enabled` is set, calls `this.navigateNext` afterwards.
  - `acceptAll` (`commandHandler.ts:258`) — fetches all conflicts, forgets the document, then applies all edits in a single `editor.edit()` callback to batch them.
  - `acceptAllResources` (`commandHandler.ts:275`) — multi-document variant; opens each document, collects edits into a `vscode.WorkspaceEdit`, calls `vscode.workspace.applyEdit(edit)` to apply atomically across files.
  - `compare` (`commandHandler.ts:85`) — constructs two virtual URIs with scheme `merge-conflict.conflict-diff` carrying the ranges as JSON in the query string, then invokes `vscode.diff` to open a diff editor. Computes `mergeConflictLineOffsets` at line 120 to compensate for line-count differences between the full document and the stripped diff view.
  - `acceptSelection` (`commandHandler.ts:158`) — determines which side the cursor is on by comparing `editor.selection.active` against `conflict.splitter.start` and `conflict.commonAncestors[0].header`.
  - `findConflictForNavigation` (`commandHandler.ts:315`) — for Forwards direction uses a `isBefore(conflict.range.start)` predicate and falls back to `conflicts[0]`; for Backwards reverses the array and falls back to `conflicts[conflicts.length - 1]`.
- **Control flow:** Command invocation → `registerTextEditorCommand` wrapper → specific handler → tracker fetch + forget → `conflict.commitEdit` or navigation.
- **Data flow:** `IDocumentMergeConflict` objects travel from tracker into command handlers; `commitEdit` and `applyEdit` consume the stored `vscode.Range` fields to write back to the document.
- **Dependencies:** `vscode`, `./interfaces`, `./contentProvider`.

---

### `extensions/merge-conflict/src/mergeDecorator.ts`

- **Role:** Applies visual decorations (background colors, outline borders, overview ruler markers) to conflict regions in all visible text editors.
- **Key symbols:**
  - `MergeDecorator` (`mergeDecorator.ts:9`) — class; holds `decorations: { [key: string]: vscode.TextEditorDecorationType }` dictionary; tracker created via `trackerService.createTracker('decorator')`.
  - `begin` (`mergeDecorator.ts:23`) — calls `registerDecorationTypes`, applies decorations to currently visible editors, and subscribes to `onDidOpenTextDocument`, `onDidChangeTextDocument`, and `onDidChangeVisibleTextEditors`.
  - `registerDecorationTypes` (`mergeDecorator.ts:55`) — disposes existing decoration types, then creates new ones. Content-block decorations (`current.content`, `incoming.content`, `commonAncestors.content`) are created via `generateBlockRenderOptions` which conditionally adds `backgroundColor` and `overviewRulerColor`. Header and splitter decorations (`current.header`, `commonAncestors.header`, `splitter`, `incoming.header`) are created unconditionally when `enableDecorations` is true, with `after.contentText` labels "(Current Change)" and "(Incoming Change)" on lines 90 and 120 respectively.
  - `applyDecorations` (`mergeDecorator.ts:163`) — async; uses an `updating: Map<vscode.TextEditor, boolean>` guard to prevent re-entrant decoration while a previous scan is in flight. Fetches conflicts, builds `matchDecorations: { [key: string]: vscode.Range[] }` by iterating each conflict's regions, then calls `editor.setDecorations(decorationType, ranges)` for each key.
  - `removeDecorations` (`mergeDecorator.ts:238`) — clears all decoration types for an editor by calling `editor.setDecorations(type, [])`.
- **Control flow:** VS Code event → `applyDecorationsFromEvent` → `applyDecorations(editor)` → tracker fetch → build range arrays → `editor.setDecorations`.
- **Data flow:** `IDocumentMergeConflict[]` from tracker → `conflict.current.decoratorContent`, `conflict.incoming.decoratorContent`, `conflict.current.header`, `conflict.splitter`, `conflict.incoming.header`, `conflict.commonAncestors[*].decoratorContent` → accumulated into `matchDecorations` keyed by decoration type name → applied per editor.
- **Dependencies:** `vscode`, `./interfaces`.

---

### `extensions/merge-conflict/src/contentProvider.ts`

- **Role:** Implements `vscode.TextDocumentContentProvider` for the virtual scheme `merge-conflict.conflict-diff`, generating the stripped document text used as one side of the diff view.
- **Key symbols:**
  - `MergeConflictContentProvider` (`contentProvider.ts:8`) — class; static `scheme = 'merge-conflict.conflict-diff'` at line 10.
  - `begin` (`contentProvider.ts:15`) — registers the provider with `vscode.workspace.registerTextDocumentContentProvider`.
  - `provideTextDocumentContent` (`contentProvider.ts:24`) — parses `uri.query` as JSON to extract `{ scheme, ranges }` where `ranges` is an array of `[conflictRange, fullRange]` pairs. Reopens the original document with the real `scheme` and empty query. Then iterates the ranges array: for text between conflicts it uses `document.getText(new vscode.Range(lastPosition, fullStart))`, and for the conflict region itself uses `document.getText(new vscode.Range(start, end))` (the content-only range, without markers). Appends the tail after the last conflict range at line 44.
- **Control flow:** VS Code diff infrastructure calls `provideTextDocumentContent` with a virtual URI → JSON parse → original document open → text assembly loop → string returned.
- **Data flow:** URI query JSON (containing serialized `vscode.Range` objects as plain position objects) → deserialized → `document.getText(range)` calls → assembled string representing the document with conflict markers stripped.
- **Dependencies:** `vscode`.

---

### `extensions/merge-conflict/src/delayer.ts`

- **Role:** Generic debouncing utility; defers task execution until a quiet period has elapsed, coalescing multiple triggers into a single execution.
- **Key symbols:**
  - `Delayer<T>` (`delayer.ts:10`) — generic class; holds `timeout`, `completionPromise`, `onSuccess`, and `task` fields.
  - `trigger` (`delayer.ts:26`) — stores the new `task`, cancels any pending timeout, creates a `completionPromise` if none exists (a Promise resolved via the stored `onSuccess` callback), then sets a new `setTimeout`. Returns the shared `completionPromise` so multiple callers to the same `Delayer` all await the same eventual result.
  - `forceDelivery` (`delayer.ts:54`) — cancels the timeout and synchronously resolves the pending promise by calling `onSuccess(undefined)`.
  - `cancel` (`delayer.ts:68`) — cancels the timeout and nulls `completionPromise` to discard the pending result entirely.
- **Control flow:** `trigger(task, delay)` → cancel old timeout → create or reuse `completionPromise` → set new `setTimeout` → return promise. When timer fires → `onSuccess(undefined)` → `.then()` callback executes `task()` → result delivered to all awaiting callers.
- **Data flow:** Caller passes a `() => T` task function; after the delay the task is invoked and its return value is the resolved value of the shared `Promise<T>`. In `DocumentMergeConflictTracker.getConflicts` the delay is `0`, so tasks execute as microtasks/macrotasks rather than after a real wait, but still coalesce within the current event-loop tick.
- **Dependencies:** None (no imports).

---

## Cross-Cutting Synthesis

The merge-conflict extension is a tightly layered pipeline built entirely on VS Code's extension API. At its base, `MergeConflictParser` performs a single-pass line scan of the raw `vscode.TextDocument`, turning four marker strings (`<<<<<<<`, `|||||||`, `=======`, `>>>>>>>`) into typed `IDocumentMergeConflictDescriptor` objects composed entirely of `vscode.Range` values. These descriptors are wrapped in `DocumentMergeConflict` objects which hold the edit-application logic.

`DocumentMergeConflictTracker` adds a per-URI `Map` cache backed by `Delayer` instances, so that `CodeLensProvider`, `MergeDecorator`, and `CommandHandler` — three independent origin strings — all share the same parse result within a debounce window rather than each triggering independent scans. The cache is eagerly invalidated via `forget()` before any edit operation to ensure subsequent reads see the post-edit document state.

`ServiceWrapper` acts as the composition root: it creates one `DocumentMergeConflictTracker` and passes it as `IDocumentMergeConflictTrackerService` to each of the three consumer subsystems, which call `createTracker(origin)` to get their own `OriginDocumentMergeConflictTracker` facade. Configuration is read from `vscode.workspace.getConfiguration('merge-conflict')` at startup and propagated on change via the `configurationUpdated` protocol, allowing CodeLens registration and decoration types to be added or torn down at runtime without reloading the extension.

The `ContentProvider` stands apart from the tracker pipeline: it is only invoked on demand by VS Code's diff infrastructure when `compare` constructs a `merge-conflict.conflict-diff://` URI carrying serialized range pairs in the query string.

Porting to Tauri/Rust would require: (1) a Rust reimplementation of the line-scan parser (straightforward string processing); (2) a Rust equivalent of the `vscode.Range`/`vscode.Position` geometry types used pervasively; (3) equivalents of `TextEditorDecorationType` (gutter/highlight rendering), `CodeLensProvider` (inline actions), `TextDocumentContentProvider` (virtual documents), and the `commands.registerCommand` / `editor.edit` APIs — all of which are deep VS Code workbench surface area with no direct analogue in Tauri, requiring custom frontend rendering infrastructure.

---

## Out-of-Partition References

- `vscode` (VS Code extension host API) — all files; provides `TextDocument`, `Range`, `Position`, `TextEditor`, `TextEditorDecorationType`, `CodeLens`, `CodeLensProvider`, `TextDocumentContentProvider`, `workspace`, `window`, `commands`, `languages`, `l10n`, `ThemeColor`, `OverviewRulerLane`, `WorkspaceEdit`, `Uri`.
- `@vscode/extension-telemetry` — `services.ts:12`, `documentTracker.ts:10`, `mergeConflictParser.ts:8`, `documentMergeConflict.ts:7`; provides `TelemetryReporter` used to emit GDPR-annotated events `mergeMarkers.documentWithConflictMarkersOpened` and `mergeMarkers.accept`.
- `package.json` (extension manifest, not in `src/`) — contributes the `merge-conflict.*` command identifiers consumed in `commandHandler.ts:29-40` and the `merge-conflict` configuration section read in `services.ts:56-59` and `commandHandler.ts:136,207,251`.
