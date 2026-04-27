# Partition 32 of 79 — Findings

## Scope
`extensions/merge-conflict/` (13 files, 1,463 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locator Index: extensions/merge-conflict/

## Implementation

- `extensions/merge-conflict/src/mergeConflictMain.ts` — Extension entry point with activate/deactivate; demonstrates vscode.ExtensionContext usage
- `extensions/merge-conflict/src/services.ts` — Service orchestrator (vscode.Disposable) that wires together all components and manages configuration
- `extensions/merge-conflict/src/codelensProvider.ts` — CodeLensProvider implementation using vscode.languages.registerCodeLensProvider() and vscode.CodeLens API
- `extensions/merge-conflict/src/commandHandler.ts` — Command registration and execution for merge conflict actions
- `extensions/merge-conflict/src/contentProvider.ts` — Content provider for diff view functionality
- `extensions/merge-conflict/src/mergeDecorator.ts` — Decoration/gutter rendering for merge conflict regions
- `extensions/merge-conflict/src/documentTracker.ts` — Document lifecycle and change tracking
- `extensions/merge-conflict/src/documentMergeConflict.ts` — Merge conflict data model and editing operations
- `extensions/merge-conflict/src/mergeConflictParser.ts` — Parser for detecting and extracting merge conflict regions
- `extensions/merge-conflict/src/delayer.ts` — Debouncing/delay utility

## Types / Interfaces

- `extensions/merge-conflict/src/interfaces.ts` — Defines IMergeRegion, IDocumentMergeConflict, IDocumentMergeConflictTracker, IExtensionConfiguration with vscode.Range/TextEditor references

## Configuration

- `extensions/merge-conflict/package.json` — Extension manifest with vscode engine dependency, contribution points (commands, menus, configuration), telemetry, and dual build targets (node + web)
- `extensions/merge-conflict/tsconfig.json` — TypeScript configuration extending base config, includes vscode.d.ts type definitions
- `extensions/merge-conflict/tsconfig.browser.json` — Browser variant TypeScript config
- `extensions/merge-conflict/package.nls.json` — Localization strings (i18n) for UI text
- `extensions/merge-conflict/esbuild.mts` — Build configuration for Node.js target
- `extensions/merge-conflict/esbuild.browser.mts` — Build configuration for web/browser target
- `extensions/merge-conflict/.npmrc` — npm configuration
- `extensions/merge-conflict/.vscodeignore` — Vscode packaging exclusions

## Documentation

- `extensions/merge-conflict/README.md` — Extension overview noting it is bundled with VS Code

## Examples / Fixtures

- `extensions/merge-conflict/media/icon.png` — Extension icon asset

## Notable Clusters

- `extensions/merge-conflict/src/` — 10 TypeScript files implementing merge conflict detection, CodeLens provider registration, command handlers, decorators, and diff viewing; demonstrates vscode extension API patterns for language intelligence (CodeLensProvider), UI decorations, and editor commands

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Merge Conflict Extension: Core Pattern Analysis

Analysis of VS Code's merge-conflict extension (`extensions/merge-conflict/`) to identify architectural and implementation patterns relevant to IDE functionality porting considerations.

## Key Findings

### Pattern 1: CodeLens Provider Registration with Multi-Scheme Support
**Where:** `extensions/merge-conflict/src/codelensProvider.ts:100-106`
**What:** Registers CodeLens provider across multiple URI schemes to support different document types.

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
- Conditionally registered in `begin()` (line 18-23)
- Dynamically registered/disposed on config changes in `configurationUpdated()` (lines 28-34)
- Implementation pattern shows provider implements `vscode.CodeLensProvider` interface (line 9)

---

### Pattern 2: CodeLens Provider Implementation with Command Arguments
**Where:** `extensions/merge-conflict/src/codelensProvider.ts:47-98`
**What:** Provides array of CodeLens objects with commands, passing conflict objects as arguments.

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
		// ... similar commands for incoming, both, diff ...
		const range = document.lineAt(conflict.range.start.line).range;
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

**Key aspects:**
- Returns null conditionally (feature disabled or no conflicts)
- Updates VS Code context via `setContext` for conditional UI
- Creates multiple CodeLens objects at same range with different commands
- Passes structured data as command arguments

---

### Pattern 3: TextEditorDecorationType Management with Theme Colors
**Where:** `extensions/merge-conflict/src/mergeDecorator.ts:55-125`
**What:** Creates and manages multiple decoration types with theme-aware colors and outline styling.

```typescript
private registerDecorationTypes(config: interfaces.IExtensionConfiguration) {
	// Dispose of existing decorations
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
- Called in `begin()` during initialization (line 23-25)
- Re-called in `configurationUpdated()` with full re-decoration of visible editors (lines 44-53)
- Disposed in `dispose()` method (lines 127-135)
- Applied per-editor in `applyDecorations()` (line 229)

---

### Pattern 4: Document Event-Driven Decoration Application
**Where:** `extensions/merge-conflict/src/mergeDecorator.ts:23-42`
**What:** Attaches workspace and window event listeners to track and apply decorations across document lifecycle.

```typescript
begin(config: interfaces.IExtensionConfiguration) {
	this.config = config;
	this.registerDecorationTypes(config);

	// Check if we already have a set of active windows, attempt to track these.
	vscode.window.visibleTextEditors.forEach(e => this.applyDecorations(e));

	vscode.workspace.onDidOpenTextDocument(event => {
		this.applyDecorationsFromEvent(event);
	}, null, this.context.subscriptions);

	vscode.workspace.onDidChangeTextDocument(event => {
		this.applyDecorationsFromEvent(event.document);
	}, null, this.context.subscriptions);

	vscode.window.onDidChangeVisibleTextEditors((e) => {
		// Any of which could be new (not just the active one).
		e.forEach(e => this.applyDecorations(e));
	}, null, this.context.subscriptions);
}
```

**Key aspects:**
- Initializes decorations on currently visible editors
- Tracks three event types: document open, document change, editor visibility
- Passes `this.context.subscriptions` to automatically dispose listeners
- Deferred application: calls `applyDecorationsFromEvent()` which finds matching editor

---

### Pattern 5: Disposable Service Registration with Begin/Configuration Pattern
**Where:** `extensions/merge-conflict/src/services.ts:27-53`
**What:** Service wrapper that initializes multiple disposable components with uniform lifecycle and config update handling.

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

**Key aspects:**
- Creates shared `DocumentTracker` service injected into multiple providers
- Uses duck-typing pattern checking for `begin` and `configurationUpdated` methods
- Single configuration creation point passed to all services
- Configuration refresh coordinated globally on workspace config change

---

### Pattern 6: Deferred Computation with Debouncing Using Delayer
**Where:** `extensions/merge-conflict/src/documentTracker.ts:53-79`
**What:** Uses Delayer utility to debounce conflict scan requests from multiple origins with cache invalidation.

```typescript
getConflicts(document: vscode.TextDocument, origin: string): PromiseLike<interfaces.IDocumentMergeConflict[]> {
	const key = this.getCacheKey(document);

	if (!key) {
		return Promise.resolve(this.getConflictsOrEmpty(document, [origin]));
	}

	let cacheItem = this.cache.get(key);
	if (!cacheItem) {
		cacheItem = new ScanTask(this.delayExpireTime, origin);
		this.cache.set(key, cacheItem);
	}
	else {
		cacheItem.addOrigin(origin);
	}

	return cacheItem.delayTask.trigger(() => {
		const conflicts = this.getConflictsOrEmpty(document, Array.from(cacheItem!.origins));
		this.cache?.delete(key!);
		return conflicts;
	});
}
```

**Delayer Implementation:** `extensions/merge-conflict/src/delayer.ts:10-79`
- Simple generic delay mechanism using setTimeout
- Supports forced delivery without waiting
- Tracks triggered state
- Allows cancellation

**Variations / call-sites:**
- Called from CodeLensProvider (codelensProvider.ts:53)
- Called from CommandHandler (commandHandler.ts:98, 259, 279, 299)
- Called from Decorator (mergeDecorator.ts:178)

---

### Pattern 7: Command Registration with Multiple Entry Points
**Where:** `extensions/merge-conflict/src/commandHandler.ts:28-51`
**What:** Registers text editor commands with optional resource-based alternative handler for multi-resource operations.

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

**Key aspects:**
- Dual-mode handler: resource-based vs editor-based
- Detects invocation context by inspecting argument structure
- All commands return Promises for async operations

---

### Pattern 8: TextDocumentContentProvider for Virtual Documents
**Where:** `extensions/merge-conflict/src/contentProvider.ts:8-53`
**What:** Implements custom content provider for diff view virtual documents reconstructing content from conflict regions.

```typescript
export default class MergeConflictContentProvider implements vscode.TextDocumentContentProvider, vscode.Disposable {

	static scheme = 'merge-conflict.conflict-diff';

	constructor(private context: vscode.ExtensionContext) {
	}

	begin() {
		this.context.subscriptions.push(
			vscode.workspace.registerTextDocumentContentProvider(MergeConflictContentProvider.scheme, this)
		);
	}

	async provideTextDocumentContent(uri: vscode.Uri): Promise<string | null> {
		try {
			const { scheme, ranges } = JSON.parse(uri.query) as { scheme: string; ranges: [{ line: number; character: number }[], { line: number; character: number }[]][] };

			const document = await vscode.workspace.openTextDocument(uri.with({ scheme, query: '' }));

			let text = '';
			let lastPosition = new vscode.Position(0, 0);

			ranges.forEach(rangeObj => {
				const [conflictRange, fullRange] = rangeObj;
				const [start, end] = conflictRange;
				const [fullStart, fullEnd] = fullRange;

				text += document.getText(new vscode.Range(lastPosition.line, lastPosition.character, fullStart.line, fullStart.character));
				text += document.getText(new vscode.Range(start.line, start.character, end.line, end.character));
				lastPosition = new vscode.Position(fullEnd.line, fullEnd.character);
			});

			const documentEnd = document.lineAt(document.lineCount - 1).range.end;
			text += document.getText(new vscode.Range(lastPosition.line, lastPosition.character, documentEnd.line, documentEnd.character));

			return text;
		}
		catch (ex) {
			await vscode.window.showErrorMessage('Unable to show comparison');
			return null;
		}
	}
}
```

**Key aspects:**
- Custom URI scheme isolation
- Encodes ranges as JSON in URI query string
- Reconstructs document content from fragments
- Error handling with user notification

---

## Architecture Summary

The merge-conflict extension exemplifies a modular, event-driven architecture for IDE features:

1. **Lifecycle Management:** Service wrapper coordinates initialization and configuration updates across multiple providers
2. **Event-Driven Updates:** Workspace and editor events trigger automatic re-computation and re-decoration
3. **Shared State:** DocumentTracker service provides single source of truth for conflict detection with debouncing
4. **UI Integration:** CodeLens and decorations register via VS Code APIs with multi-scheme support
5. **Virtual Documents:** Custom content providers enable diff views through URI-based content generation
6. **Command Routing:** Flexible command handlers support both editor and resource contexts
7. **Resource Management:** Systematic disposal of listeners, decorations, and registrations

## Porting Implications

From a Tauri/Rust perspective, these patterns suggest:

- **Multi-document abstraction** needed (file, vfs, untitled, userdata schemes)
- **Event broker** required for document/editor lifecycle coordination
- **Decoration system** needs theme color resolution and renderer integration
- **Command dispatch** requires context detection (active editor vs file resource)
- **Debouncing primitives** for expensive operations (parsing/scanning)
- **Virtual document protocol** for synthetic content generation
- **Disposable pattern** throughout for memory management

The extension demonstrates that IDE functionality depends heavily on event-driven architecture, with features coordinated through a pub-sub model and centralized configuration management rather than direct module dependencies.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
