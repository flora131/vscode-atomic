# Partition 27 of 80 — Findings

## Scope
`extensions/npm/` (14 files, 2,372 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# VS Code npm Extension: Tasks API Consumer Mapping

## Summary

The `extensions/npm/` extension (14 files, ~2,372 LOC) is an in-tree consumer of the VS Code Tasks API that demonstrates how a Rust/Tauri-based port of VS Code's core would need to support task automation. The extension registers itself as an npm task provider, spawns child processes for npm/yarn/pnpm/bun package managers, and integrates with VS Code's UI through commands, hovers, code lenses, and a tree view explorer.

---

## Implementation

- `extensions/npm/src/npmMain.ts` — Extension entry point; calls `vscode.tasks.registerTaskProvider('npm', taskProvider)` at line 136, registers file watchers for package.json changes, manages task caching and invalidation
- `extensions/npm/src/tasks.ts` — Core task provider implementation; defines `NpmTaskProvider` class implementing `TaskProvider` interface; creates `Task` objects with `ShellExecution` for spawning npm/yarn/pnpm/bun; handles script detection from package.json via `readScripts()`, pre/post script hooks, build/test/debug task classification, and installation task generation
- `extensions/npm/src/commands.ts` — Command handlers for running scripts; `runSelectedScript()` executes tasks via `tasks.executeTask()`, folder-based script execution
- `extensions/npm/src/npmView.ts` — Tree data provider (`NpmScriptsTreeDataProvider`) for script explorer UI; displays packages and scripts as tree items; provides commands to open, run, and debug scripts
- `extensions/npm/src/scriptHover.ts` — Hover provider for package.json script names; offers run/debug commands inline
- `extensions/npm/src/npmScriptLens.ts` — Code lens provider for debug commands above script definitions; respects configuration for lens placement
- `extensions/npm/src/preferred-pm.ts` — Package manager detection via lockfile presence (npm, yarn, pnpm, bun); uses `workspace.fs.stat()` and `whichPM` library
- `extensions/npm/src/readScripts.ts` — JSON parsing logic to extract scripts section from package.json using `jsonc-parser` visitor pattern
- `extensions/npm/src/features/packageJSONContribution.ts` — JSON schema contributions for package.json autocomplete; spawns `child_process` (line 304) to run npm commands for fetching dependency info
- `extensions/npm/src/features/jsonContributions.ts` — Completion and hover provider registration for JSON files
- `extensions/npm/src/features/date.ts` — Localization utility for time-based messages

---

## Types / Interfaces

- `extensions/npm/src/tasks.ts` — `INpmTaskDefinition` (extends `TaskDefinition`) with `script` and optional `path` properties; `ITaskWithLocation` wrapping tasks with optional location; `IFolderTaskItem` for quick-pick UI
- `extensions/npm/src/readScripts.ts` — `INpmScriptReference` with name, value, nameRange, valueRange; `INpmScriptInfo` with location and scripts array
- `extensions/npm/src/features/jsonContributions.ts` — `IJSONContribution` interface defining document selector, suggestions, and hover contributions; `ISuggestionsCollector` for completion items

---

## Configuration

- `extensions/npm/package.json` — Declares activation events `onTaskType:npm`, `onLanguage:json`, `workspaceFolders:package.json`; registers task definition type `npm` with required `script` property and optional `path`; contributes configuration settings for auto-detection, silent mode, package manager selection (npm/yarn/pnpm/bun), script runner, exclusion patterns, explorer visibility; registers UI commands, menu contributions, code lenses, and terminal quick fixes
- `extensions/npm/tsconfig.json` — Extends base config, targets Node types, references vscode.d.ts type definitions and proposed terminalQuickFixProvider API
- `extensions/npm/.vscode/launch.json` — Debug configuration for extension development
- `extensions/npm/esbuild.browser.mts` — Build config marking `child_process` and `vscode` as external (not bundled for browser)
- `extensions/npm/.npmrc` — NPM runtime config
- `extensions/npm/.vscodeignore` — Specifies files excluded from extension packaging

---

## Notable Clusters

- `extensions/npm/src/features/` — 3 files providing JSON schema contributions (package.json autocomplete and hover), child process spawning for npm commands, and time formatting utilities
- `extensions/npm/src/` — Core extension logic organized by concern: tasks, commands, UI providers (view, hover, lens), script parsing, and package manager detection

---

## Key Relevance to Tauri/Rust Port

**Tasks API Contract:** The extension demonstrates the full contract a Rust core must honor:
- `TaskProvider` interface: `provideTasks()` returning array of tasks, `resolveTask()` for task details
- `Task` construction: scope (WorkspaceFolder), name, definition, execution (ShellExecution with cwd and args)
- File system watchers: `vscode.workspace.createFileSystemWatcher()` for detecting package.json changes
- Process spawning: `child_process.execFile()` with arguments and working directory control
- UI integration: Commands, tree views, hover providers, code lenses all tied to task state

**Process Control:** Uses `cp.execFile()` with shell quoting, environment control, and working directory isolation—a pattern the Rust core must replicate for all task execution.

**Package Manager Abstraction:** Detects and switches between npm, yarn, pnpm, and bun—useful reference for extensibility model in a Rust core.

**Configuration Layer:** Tasks-aware settings for auto-detection, silent mode, and script runner selection show how IDE configuration flows to task execution.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

1. `extensions/npm/src/npmMain.ts` — 174 lines
2. `extensions/npm/src/tasks.ts` — 494 lines
3. `extensions/npm/src/commands.ts` — 67 lines
4. `extensions/npm/src/readScripts.ts` — 73 lines
5. `extensions/npm/src/npmView.ts` — 334 lines
6. `extensions/npm/src/scriptHover.ts` — 130 lines
7. `extensions/npm/src/npmScriptLens.ts` — 115 lines
8. `extensions/npm/src/features/packageJSONContribution.ts` — 442 lines
9. `extensions/npm/src/features/jsonContributions.ts` — 174 lines
10. `extensions/npm/src/preferred-pm.ts` — 113 lines

---

### Per-File Notes

#### `extensions/npm/src/npmMain.ts`

- **Role:** Extension entry point; wires all providers, commands, file watchers, and the task provider into the VS Code extension lifecycle via `activate()`.

- **Key symbols:**
  - `activate(context)` at line 26 — top-level lifecycle entry
  - `registerTaskProvider(context)` at line 124 — creates `NpmTaskProvider` and `FileSystemWatcher`
  - `registerExplorer(context)` at line 143 — creates `NpmScriptsTreeDataProvider` and tree view
  - `registerHoverProvider(context)` at line 153 — registers `NpmScriptHoverProvider` on `**/package.json`
  - `invalidateScriptCaches()` at line 18 — calls `invalidateHoverScriptsCache()`, `invalidateTasksCache()`, and `treeDataProvider.refresh()`
  - `getNPMCommandPath()` at line 105 — resolves the `npm` binary using `which` (platform-aware: `npm.cmd` on win32)
  - Terminal quick-fix provider registered at line 80 — parses npm ERR! lines and offers `TerminalQuickFixTerminalCommand` fixes

- **Control flow:**
  1. `activate` → `configureHttpRequest()` → sets proxy on `request-light`
  2. `getNPMCommandPath()` → guarded by `workspace.isTrusted` and `canRunNpmInCurrentWorkspace()` (scheme `file` check at line 118)
  3. `addJSONProviders(httpRequest.xhr, npmCommandPath)` at line 35 — registers JSON completion/hover
  4. `registerTaskProvider` → `createFileSystemWatcher('**/package.json')` at line 126 — all three watcher events (`onDidChange`, `onDidDelete`, `onDidCreate`) call `invalidateScriptCaches()`
  5. Configuration change listener at line 40 watches `npm.exclude`, `npm.autoDetect`, `npm.scriptExplorerExclude`, `npm.runSilent`, `npm.packageManager`, `npm.scriptRunner`
  6. `hasPackageJson()` at line 58 → sets context key `npm:showScriptExplorer`

- **Data flow:**
  - `NpmCommandPath` string flows from `which` → `addJSONProviders` → `PackageJSONContribution` constructor
  - `NpmTaskProvider` instance stored in module-scope `taskProvider` at line 123, passed to `NpmScriptsTreeDataProvider` at line 145

- **Dependencies:** `request-light` (HTTP proxy), `which` (binary location), `vscode` API (workspace, tasks, commands, window, languages)

---

#### `extensions/npm/src/tasks.ts`

- **Role:** Defines `NpmTaskProvider` and all task construction helpers; implements package.json discovery, script parsing, and `ShellExecution`-based task creation.

- **Key symbols:**
  - `NpmTaskProvider` class at line 46 — implements `TaskProvider`
  - `provideTasks()` at line 55 — calls `provideNpmScripts(context, true)` and strips location info
  - `resolveTask(_task)` at line 60 — reconstructs a task from stored `INpmTaskDefinition`; delegates to `createInstallationTask` or `createScriptRunnerTask`
  - `provideNpmScripts(context, showWarning)` at line 229 — cache layer over `findNpmPackages()` async generator
  - `findNpmPackages()` at line 185 — async generator; iterates workspace folders, calls `workspace.findFiles` with `RelativePattern`, excludes `**/node_modules/.vscode-test/**`
  - `createScriptRunnerTask(context, script, folder, packageJsonUri, scriptValue, showWarning)` at line 334 — central task factory
  - `createInstallationTask(context, folder, packageJsonUri, ...)` at line 371
  - `getScriptRunner(folder, context, showWarning)` at line 130 — reads `npm.scriptRunner` config; delegates to `detectPackageManager` if `'auto'`
  - `getPackageManager(folder, context, showWarning)` at line 140 — reads `npm.packageManager` config; same `'auto'` delegation
  - `detectPackageManager(folder, extensionContext, showWarning)` at line 150 — calls `findPreferredPM`, shows information message if multiple lockfiles
  - `getRunScriptCommand(script, folder, context, showWarning)` at line 319 — returns `['node', '--run', script]` for `scriptRunner === 'node'`; otherwise `[scriptRunner, 'run', ...optional '--silent'..., script]`
  - `escapeCommandLine(cmd)` at line 304 — applies `ShellQuoting.Weak` for args with `--`, `ShellQuoting.Strong` otherwise, when whitespace detected
  - `cachedTasks` at line 32 — module-scope cache; `invalidateTasksCache()` at line 89 sets it to `undefined`
  - `isDebugScript(script)` at line 267 — regex matches `--inspect` or `--debug` flags
  - `startDebugging(context, scriptName, cwd, folder)` at line 447 — delegates entirely to `commands.executeCommand('extension.js-debug.createDebuggerTerminal', ...)`

- **Control flow:**
  1. `provideNpmScripts` → checks `cachedTasks`; if absent, iterates `findNpmPackages()` generator
  2. Each package URI → `provideNpmScriptsForFolder` at line 272 → `getScripts(packageJsonUri)` → opens `TextDocument` via `workspace.openTextDocument` → calls `readScripts(document)`
  3. Per script: `createScriptRunnerTask` → `getRunScriptCommand` → `getScriptRunner` → config lookup or `detectPackageManager`
  4. `new Task(kind, folder, taskName, 'npm', new ShellExecution(scriptRunner, escapeCommandLine(args), { cwd }))` at line 345

- **Data flow:**
  - `package.json` URI → `workspace.openTextDocument` → `readScripts` returns `INpmScriptInfo` with `scripts[]`
  - Each `{name, value, nameRange}` → `createScriptRunnerTask` → `Task` with `ShellExecution`
  - Task group assignment at lines 349-357: build names → `TaskGroup.Build`, "test" → `TaskGroup.Test`, pre/post scripts → `TaskGroup.Clean`, debug scripts → `TaskGroup.Rebuild`

- **Dependencies:** `vscode` (Task, ShellExecution, TaskGroup, workspace, etc.), `path`, `fs`, `minimatch`, `vscode-uri` (Utils), `./preferred-pm`, `./readScripts`

---

#### `extensions/npm/src/commands.ts`

- **Role:** Implements the two user-facing command handlers: `runSelectedScript` (runs script at cursor) and `selectAndRunScriptFromFolder` (quick-pick from folder).

- **Key symbols:**
  - `runSelectedScript(context)` at line 16 — reads `activeTextEditor`, calls `findScriptAtPosition` to identify script by cursor position, then `runScript(context, script, document)`
  - `selectAndRunScriptFromFolder(context, selectedFolders)` at line 32 — calls `detectNpmScriptsForFolder`, builds `QuickPick`, resolves to `vscode.tasks.executeTask(result.task)`

- **Control flow:**
  1. `runSelectedScript`: `activeTextEditor` guard → `findScriptAtPosition(document, contents, anchor)` → if found, `runScript(context, script, document)` → `createScriptRunnerTask` → `tasks.executeTask(task)`
  2. `selectAndRunScriptFromFolder`: `detectNpmScriptsForFolder` (finds all package.json under folder) → `createQuickPick` → `onDidAccept` → `tasks.executeTask(result.task)`

- **Data flow:**
  - Editor cursor position → `findScriptAtPosition` (which calls `readScripts` then range checks) → script name string → new `Task` via `createScriptRunnerTask` → `tasks.executeTask`

- **Dependencies:** `vscode` API, `./tasks` (detectNpmScriptsForFolder, findScriptAtPosition, runScript, IFolderTaskItem)

---

#### `extensions/npm/src/readScripts.ts`

- **Role:** Parses the `scripts` section of a `package.json` document using `jsonc-parser`'s visitor API and returns typed range-annotated script entries.

- **Key symbols:**
  - `readScripts(document, buffer)` at line 21 — exported pure function; takes a `TextDocument` and optional raw buffer string
  - `INpmScriptReference` at line 9 — `{ name, value, nameRange: Range, valueRange: Range }`
  - `INpmScriptInfo` at line 16 — `{ location: Location, scripts: INpmScriptReference[] }`
  - `JSONVisitor` (from `jsonc-parser`) at line 29 — visitor with `onObjectBegin`, `onObjectEnd`, `onLiteralValue`, `onObjectProperty` callbacks

- **Control flow:**
  1. `visit(buffer, visitor)` traverses JSON tokens
  2. `onObjectProperty`: at `level === 1`, property `'scripts'` sets `inScripts = true` and records `start` offset; within `inScripts`, each property name creates a `buildingScript` record
  3. `onLiteralValue`: if `buildingScript` is set, finalizes `INpmScriptReference` entry with the string value and range
  4. `onObjectEnd`: clears `inScripts`, records `end` offset
  5. Returns `{ location, scripts }` or `undefined` if no `scripts` key found

- **Data flow:**
  - Raw JSON string → `jsonc-parser.visit` → visitor callbacks accumulate `scripts[]` array → returned as `INpmScriptInfo`
  - All positions are derived from `document.positionAt(offset)` converting byte offsets to `Position` objects

- **Dependencies:** `jsonc-parser` (`JSONVisitor`, `visit`), `vscode` (Location, Position, Range, TextDocument)

---

#### `extensions/npm/src/npmView.ts`

- **Role:** Implements the NPM Scripts explorer tree view using `TreeDataProvider`; builds a three-level hierarchy of `Folder → PackageJSON → NpmScript` tree items.

- **Key symbols:**
  - `NpmScriptsTreeDataProvider` at line 138 — implements `TreeDataProvider<TreeItem>`
  - `Folder` (line 25), `PackageJSON` (line 44), `NpmScript` (line 76), `NoScripts` (line 129) — tree item subclasses
  - `getChildren(element)` at line 229 — lazy load: fetches from `taskProvider.tasksWithLocation`, calls `buildTaskTree` then `sortTaskTree`
  - `buildTaskTree(tasks)` at line 289 — groups `ITaskWithLocation[]` by workspace folder and package path into `Folder`/`PackageJSON`/`NpmScript` hierarchy; respects `npm.scriptExplorerExclude` regex array
  - `refresh()` at line 204 — sets `taskTree = null`, fires `_onDidChangeTreeData`
  - `runScript(script)` at line 153 — calls `detectPackageManager` (for warning side-effect), then `tasks.executeTask`
  - `debugScript(script)` at line 159 — calls `startDebugging` (delegates to `extension.js-debug.createDebuggerTerminal`)
  - `openScript(selection)` at line 189 — opens text document and moves cursor to script name position via `findScriptPosition` → `readScripts`

- **Control flow:**
  1. Tree view created in `npmMain.ts:146` → `getChildren(undefined)` called
  2. `taskProvider.tasksWithLocation` → `provideNpmScripts(context, false)` (no warning)
  3. `buildTaskTree` iterates tasks, filters by `scriptExplorerExclude` regex, builds maps `folders` and `packages`, creates `NpmScript` items; if only one folder exists, returns flat `packages` list (line 329)
  4. `NpmScript` constructor at line 81: command action determined by `npm.scriptExplorerAction` setting (`'open'` or `'run'`); icon set to `wrench-subaction` for `TaskGroup.Clean` items at line 113

- **Data flow:**
  - `ITaskWithLocation[]` → `buildTaskTree` → `TaskTree` (Folder[]|PackageJSON[]|NoScripts[]) → returned by `getChildren` → rendered by VS Code tree view
  - `NpmScript.task` holds the `Task` object; `NpmScript.taskLocation` holds `Location` for navigation

- **Dependencies:** `vscode` API, `path`, `./readScripts`, `./tasks`

---

#### `extensions/npm/src/scriptHover.ts`

- **Role:** Provides inline hover actions ("Run Script" / "Debug Script") over script name keys in `package.json` files.

- **Key symbols:**
  - `NpmScriptHoverProvider` at line 33 — implements `HoverProvider`
  - `provideHover(document, position, token)` at line 52 — checks `enabled`, reads cached scripts, matches position to `nameRange`
  - `cachedDocument` / `cachedScripts` at lines 20-21 — module-scope single-entry cache; invalidated when the document changes via `workspace.onDidChangeTextDocument`
  - `invalidateHoverScriptsCache(document?)` at line 23 — exported for use by `npmMain.ts`
  - `createRunScriptMarkdown` at line 76 / `createDebugScriptMarkdown` at line 89 — build command URI markdown links
  - `createMarkdownLink` at line 103 — encodes args as `encodeURIComponent(JSON.stringify(args))` and returns `[Label](command:cmd?args "tooltip")`
  - `runScriptFromHover(args)` at line 112 — reconstructs task via `createScriptRunnerTask`, calls `tasks.executeTask`
  - `debugScriptFromHover(args)` at line 122 — calls `startDebugging`

- **Control flow:**
  1. `provideHover` called by VS Code on hover over `**/package.json`
  2. Cache check: if `cachedDocument.fsPath !== document.uri.fsPath`, call `readScripts(document)` and update cache
  3. Iterate `cachedScripts.scripts`, check `nameRange.contains(position)`, build `MarkdownString` with trusted command links
  4. Return `new Hover(contents)` or `undefined`

- **Data flow:**
  - `TextDocument` → `readScripts` → `INpmScriptInfo.scripts[].nameRange` → hover match → `MarkdownString` with encoded command args → VS Code renders hover popup

- **Dependencies:** `vscode` API, `path`, `./readScripts`, `./tasks` (createScriptRunnerTask, startDebugging)

---

#### `extensions/npm/src/npmScriptLens.ts`

- **Role:** Code lens provider that places a "Debug" lens above either the entire scripts section or each individual script in `package.json`, controlled by `debug.javascript.codelens.npmScripts` config.

- **Key symbols:**
  - `NpmScriptLensProvider` at line 32 — implements `CodeLensProvider` and `Disposable`
  - `Constants.ConfigKey = 'debug.javascript.codelens.npmScripts'` at line 23
  - `provideCodeLenses(document)` at line 64 — reads `lensLocation` setting (`'never'`, `'top'`, `'all'`)
  - `lensLocation === 'top'` branch at line 76 — single `CodeLens` at `tokens.location.range`, command `extension.js-debug.npmScript`
  - `lensLocation === 'all'` branch at line 89 — one `CodeLens` per script at `nameRange`, command `extension.js-debug.createDebuggerTerminal` with run script command string

- **Control flow:**
  1. `languages.registerCodeLensProvider` at line 51 with pattern `**/package.json`, language `json`
  2. Config change listener at line 44 re-reads setting and fires `changeEmitter` to invalidate lenses
  3. `provideCodeLenses` → `readScripts(document)` → for `'all'` mode, calls `getRunScriptCommand(name, folder)` per script to compute the terminal command string

- **Data flow:**
  - `TextDocument` → `readScripts` → `INpmScriptInfo.scripts[].nameRange` → `CodeLens` with command args → VS Code renders lens; clicking invokes `extension.js-debug.*` commands (js-debug extension)

- **Dependencies:** `vscode` API, `path`, `./readScripts`, `./tasks` (getRunScriptCommand)

---

#### `extensions/npm/src/features/packageJSONContribution.ts`

- **Role:** Provides IntelliSense completions and hover documentation for npm package names and versions in `package.json` dependency fields, using the npm CLI and/or the npmjs registry REST API.

- **Key symbols:**
  - `PackageJSONContribution` at line 19 — implements `IJSONContribution`
  - `collectPropertySuggestions(...)` at line 61 — for dependency fields: if query starts with `@`, lists known scopes or calls `collectScopedPackages`; otherwise queries `https://registry.npmjs.org/-/v1/search` via `this.xhr`
  - `collectValueSuggestions(...)` at line 182 — fetches version info for a named package and offers exact, `^`, and `~` version completions
  - `fetchPackageInfo(pack, resource)` at line 276 — validates npm name, then calls `npmView` (CLI) and `npmListInstalledVersion` (CLI) in parallel; falls back to `npmjsView` (HTTP) if CLI unavailable
  - `runNpmCommand(npmCommandPath, args, resource)` at line 303 — uses **dynamic `import('child_process')`** at line 304; calls `cp.execFile(commandPath, args, options, callback)` at line 319
    - Sets `COREPACK_ENABLE_AUTO_PIN=0` and `COREPACK_ENABLE_PROJECT_SPEC=0` env vars at line 312
    - On win32: uses `shell: true` and wraps `commandPath` in quotes at lines 315-316
  - `npmView(npmCommandPath, pack, resource)` at line 325 — runs `npm view --json -- <pack>@latest description homepage version time`
  - `npmListInstalledVersion(npmCommandPath, pack, resource)` at line 369 — runs `npm ls --json --depth=0 -- <pack>`
  - `npmjsView(pack)` at line 347 — HTTP GET to `https://registry.npmjs.org/<pack>` via `this.xhr`
  - `resolveSuggestion(resource, item)` at line 239 — lazily resolves documentation for a completion item via `fetchPackageInfo`
  - `getInfoContribution(resource, location)` at line 384 — hover info for dependency nodes

- **Control flow:**
  1. `collectPropertySuggestions` guards on `location.matches(['dependencies'])` etc.
  2. Empty word: adds `mostDependedOn` hardcoded list; non-empty: XHR to npmjs search API
  3. `fetchPackageInfo` → parallel `npmView` + `npmListInstalledVersion` if `npmCommandPath` set; fallback to XHR

- **Data flow:**
  - User types in dependency field → `collectPropertySuggestions` → XHR search → response parsed → `CompletionItem` list built
  - Completion item resolved → `fetchPackageInfo` → npm CLI stdout (JSON) parsed → `MarkdownString` documentation attached

- **Dependencies:** `vscode` API, `jsonc-parser` (Location), `request-light` (XHRRequest), `child_process` (dynamic import), `path`, `./jsonContributions` (IJSONContribution), `./date` (fromNow)

---

#### `extensions/npm/src/features/jsonContributions.ts`

- **Role:** Generic JSON contribution orchestration layer: registers `CompletionItemProvider` and `HoverProvider` for any `IJSONContribution` implementation, and provides the `JSONCompletionItemProvider` / `JSONHoverProvider` adapter classes.

- **Key symbols:**
  - `addJSONProviders(xhr, npmCommandPath)` at line 31 — creates `PackageJSONContribution`, registers completion (triggered on `'"'` and `':'`) and hover providers
  - `IJSONContribution` interface at line 22 — contract for contribution modules
  - `ISuggestionsCollector` at line 15 — `{ add, error, log, setAsIncomplete }` collector interface
  - `JSONCompletionItemProvider.provideCompletionItems` at line 88 — uses `getLocation` from `jsonc-parser` to determine JSON path, delegates to `collectPropertySuggestions` or `collectValueSuggestions`
  - `JSONHoverProvider.provideHover` at line 47 — uses `getLocation` to find node, delegates to `getInfoContribution`
  - `xhrDisabled` at line 174 — exported constant returning a rejected promise with disable message

- **Control flow:**
  1. `provideCompletionItems` → `document.offsetAt(position)` → `getLocation(text, offset)` → `location.isAtPropertyKey` branch → `collectPropertySuggestions` or `collectValueSuggestions` → collector accumulates items → returns `CompletionList`
  2. Overwrite range computed: if previous node covers offset, use node bounds; otherwise use current word bounds (line 102-106)

- **Data flow:**
  - `TextDocument` + cursor `Position` → `document.offsetAt` → `jsonc-parser.getLocation` → `Location.path` → delegates to `PackageJSONContribution` → `CompletionItem[]` collected → `CompletionList` returned

- **Dependencies:** `vscode` API, `jsonc-parser` (Location, getLocation, createScanner, SyntaxKind, ScanError), `request-light`, `./packageJSONContribution`

---

#### `extensions/npm/src/preferred-pm.ts`

- **Role:** Heuristically detects the preferred package manager for a given path by inspecting lockfile presence and using `which-pm`.

- **Key symbols:**
  - `findPreferredPM(pkgPath)` at line 71 — exported; returns `{ name, multipleLockFilesDetected }`
  - `isNPMPreferred` (line 66), `isPNPMPreferred` (line 38), `isYarnPreferred` (line 52), `isBunPreferred` (line 26) — per-manager detection functions
  - `pathExists(filePath)` at line 17 — uses `workspace.fs.stat(Uri.file(filePath))` (VS Code's FS API) for file existence
  - `findUp` (from `find-up` package) at line 7 — used in `isPNPMPreferred` to walk up for `pnpm-lock.yaml`
  - `findWorkspaceRoot` (from `find-yarn-workspace-root`) at line 6 — used in `isYarnPreferred` to detect yarn workspace root
  - `whichPM` (from `which-pm`) at line 9 — reads `packageManager` field in `package.json`

- **Control flow:**
  1. All four detection functions called in parallel (sequential awaits, lines 75-97)
  2. `whichPM(pkgPath)` called last; its result appended if not already in detected list
  3. First detected manager wins (`detectedPackageManagerNames[0] || 'npm'`)
  4. `multipleLockFilesDetected` = `lockfilesCount > 1`

- **Data flow:**
  - `pkgPath` string → lockfile stat checks via `workspace.fs.stat` → collected `detectedPackageManagerNames[]` → `{ name, multipleLockFilesDetected }`

- **Dependencies:** `vscode` (workspace.fs), `find-yarn-workspace-root`, `find-up`, `which-pm`, `path`

---

### Cross-Cutting Synthesis

**Tasks API Contract**

The extension's entire execution model is built on `vscode.Task` + `vscode.ShellExecution`. Every script invocation — whether triggered from the explorer tree, a hover link, a code lens, or a command — flows through `createScriptRunnerTask` in `tasks.ts:334`. That function constructs a `Task` with:
- `kind: INpmTaskDefinition` — `{ type: 'npm', script, path? }`
- `new ShellExecution(scriptRunner, escapeCommandLine(args), { cwd })` — where `scriptRunner` is the package manager binary name and `args` includes `run` + optional `--silent` + script name

The `ShellExecution` object wraps arguments with `ShellQuoting.Weak` or `ShellQuoting.Strong` based on whitespace and `--` prefix detection (`escapeCommandLine` at `tasks.ts:304`). The task is never spawned directly — it is always handed to `tasks.executeTask(task)` (`vscode.tasks.executeTask`), which is VS Code's own task runner. The extension does not directly create processes for script execution; the VS Code shell task runner owns that.

**Process Spawning**

The single exception to the above is in `packageJSONContribution.ts:303-322` (`runNpmCommand`), where `child_process.execFile` is used directly via a dynamic `import('child_process')` for IntelliSense purposes only — specifically `npm view` and `npm ls` calls to fetch package metadata. These run synchronously behind a `new Promise` wrapper and their output is JSON-parsed, never used as interactive processes. The `which` call in `npmMain.ts:108` similarly uses Node's `which` module synchronously only to locate the npm binary path.

**File Watching**

`npmMain.ts:126-130` creates a `FileSystemWatcher` on `'**/package.json'` using `vscode.workspace.createFileSystemWatcher`. All three watcher events (`onDidChange`, `onDidDelete`, `onDidCreate`) call `invalidateScriptCaches()`, which sets the module-scope `cachedTasks = undefined` in `tasks.ts:32` and `cachedDocument = undefined` in `scriptHover.ts:20`. A separate `workspace.onDidChangeWorkspaceFolders` listener at `npmMain.ts:132` also invalidates caches. `preferred-pm.ts:17-24` notably uses `workspace.fs.stat` (the VS Code virtual FS API) rather than Node's `fs` for file existence checks, enabling the heuristic to work over remote/virtual workspaces.

**JSON Parsing Architecture**

All `package.json` script extraction uses `jsonc-parser`'s `visit` API (`readScripts.ts:66`) — a streaming visitor that does not build an AST, operating instead on offset callbacks. This keeps `INpmScriptReference` entries accurately range-annotated for hover, code lens, and tree navigation. The `features/jsonContributions.ts` layer uses `jsonc-parser.getLocation` (builds an AST-backed location) for completion and hover in dependency fields, a distinct code path from script detection.

**Porting Implications for Tauri/Rust**

The following VS Code API surface areas used in this partition would require native replacements in a Tauri port:

- `vscode.tasks.registerTaskProvider` / `vscode.Task` / `vscode.ShellExecution` — the entire task runner abstraction; Tauri has no equivalent; would need a custom process management layer
- `vscode.workspace.createFileSystemWatcher` — file watching; maps to `notify` crate in Rust
- `vscode.workspace.findFiles` / `RelativePattern` — workspace file search; would need Rust glob/walkdir implementation
- `vscode.workspace.openTextDocument` — document model; would need Tauri's own document model
- `vscode.languages.registerHoverProvider`, `registerCodeLensProvider`, `registerCompletionItemProvider` — language feature registration; maps to LSP server protocol in Rust
- `vscode.window.createTreeView` — custom UI widget; Tauri frontend would need a tree component
- `vscode.commands.registerCommand`, `commands.executeCommand` — command palette integration; needs equivalent command bus
- `request-light` XHR adapter — HTTP client; maps to `reqwest` in Rust
- `child_process.execFile` — process spawning for `npm view`/`npm ls`; maps to `std::process::Command` in Rust
- `workspace.fs.stat` — virtual FS API; maps to Tauri's FS plugin or `tokio::fs`

---

### Out-of-Partition References

- `extension.js-debug.createDebuggerTerminal` — command implemented in the `js-debug` extension (not in this partition); invoked from `tasks.ts:451`, `npmView.ts:160`, `npmScriptLens.ts:98`
- `extension.js-debug.npmScript` — command in `js-debug`; invoked from `npmScriptLens.ts:82`
- `vscode.open` — built-in VS Code command; used in `npmView.ts:92`
- `npm.runScript` — command registered inside `NpmScriptsTreeDataProvider` constructor at `npmView.ts:147`
- `request-light` (`httpRequest.xhr`) — external package at `extensions/npm/node_modules/request-light`; HTTP abstraction for XHR in extension host
- `jsonc-parser` — external package; used in `readScripts.ts:6`, `features/jsonContributions.ts:6`, `features/packageJSONContribution.ts:9`
- `minimatch` — external package; used in `tasks.ts:13` for glob exclusion matching
- `vscode-uri` (`Utils`) — external package; used in `tasks.ts:14` for `Utils.basename`
- `find-up`, `find-yarn-workspace-root`, `which-pm`, `which` — external packages in `preferred-pm.ts` and `npmMain.ts`
- `./features/date` (`fromNow`) — sibling file in `features/` sub-directory of this partition, referenced from `packageJSONContribution.ts:13`

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder: VS Code npm Extension (Partition 27)

## Scope
Extensions: `extensions/npm/` (14 files, ~2,372 LOC)

## Task Provider Registration Pattern

#### Pattern: Task Provider Registration via Extension Activation
**Where:** `extensions/npm/src/npmMain.ts:124-141`
**What:** Registers an npm task provider that discovers and provides npm scripts as executable tasks.

```typescript
let taskProvider: NpmTaskProvider;
function registerTaskProvider(context: vscode.ExtensionContext): vscode.Disposable | undefined {
	if (vscode.workspace.workspaceFolders) {
		const watcher = vscode.workspace.createFileSystemWatcher('**/package.json');
		watcher.onDidChange((_e) => invalidateScriptCaches());
		watcher.onDidDelete((_e) => invalidateScriptCaches());
		watcher.onDidCreate((_e) => invalidateScriptCaches());
		context.subscriptions.push(watcher);

		const workspaceWatcher = vscode.workspace.onDidChangeWorkspaceFolders((_e) => invalidateScriptCaches());
		context.subscriptions.push(workspaceWatcher);

		taskProvider = new NpmTaskProvider(context);
		const disposable = vscode.tasks.registerTaskProvider('npm', taskProvider);
		context.subscriptions.push(disposable);
		return disposable;
	}
	return undefined;
}
```

**Variations / call-sites:** `npmMain.ts:36` (called from activate function)

---

## Task Provider Implementation Pattern

#### Pattern: TaskProvider Interface Implementation with Task Discovery
**Where:** `extensions/npm/src/tasks.ts:46-87`
**What:** Implements VS Code's TaskProvider interface with async task discovery and resolution.

```typescript
export class NpmTaskProvider implements TaskProvider {

	constructor(private context: ExtensionContext) {
	}

	get tasksWithLocation(): Promise<ITaskWithLocation[]> {
		return provideNpmScripts(this.context, false);
	}

	public async provideTasks() {
		const tasks = await provideNpmScripts(this.context, true);
		return tasks.map(task => task.task);
	}

	public async resolveTask(_task: Task): Promise<Task | undefined> {
		const npmTask = _task.definition.script;
		if (npmTask) {
			const kind = _task.definition as INpmTaskDefinition;
			// ... task resolution logic
			return task;
		}
		return undefined;
	}
}
```

**Variations / call-sites:** `npmMain.ts:145` (instantiated), `npmView.ts:144` (used in TreeDataProvider)

---

## Shell Execution Pattern

#### Pattern: Task Creation with ShellExecution
**Where:** `extensions/npm/src/tasks.ts:334-360`
**What:** Creates executable npm script tasks with shell execution, proper argument escaping, and task grouping.

```typescript
export async function createScriptRunnerTask(context: ExtensionContext, script: string, folder: WorkspaceFolder, packageJsonUri: Uri, scriptValue?: string, showWarning?: boolean): Promise<Task> {
	const kind: INpmTaskDefinition = { type: 'npm', script };

	const relativePackageJson = getRelativePath(folder.uri, packageJsonUri);
	if (relativePackageJson.length && !kind.path) {
		kind.path = relativePackageJson.substring(0, relativePackageJson.length - 1);
	}
	const taskName = getTaskName(script, relativePackageJson);
	const cwd = path.dirname(packageJsonUri.fsPath);
	const args = await getRunScriptCommand(script, folder.uri, context, showWarning);
	const scriptRunner = args.shift()!;
	const task = new Task(kind, folder, taskName, 'npm', new ShellExecution(scriptRunner, escapeCommandLine(args), { cwd: cwd }));
	task.detail = scriptValue;

	const lowerCaseTaskName = script.toLowerCase();
	if (isBuildTask(lowerCaseTaskName)) {
		task.group = TaskGroup.Build;
	} else if (isTestTask(lowerCaseTaskName)) {
		task.group = TaskGroup.Test;
	} else if (canHavePrePostScript(lowerCaseTaskName)) {
		task.group = TaskGroup.Clean;
	} else if (scriptValue && isDebugScript(scriptValue)) {
		task.group = TaskGroup.Rebuild;
	}
	return task;
}
```

**Variations / call-sites:** `tasks.ts:371-387` (installation task variant), `commands.ts:25`, `npmView.ts:185`, `scriptHover.ts:117`

---

## Command Registration Pattern

#### Pattern: Command Registration with Multiple Commands
**Where:** `extensions/npm/src/npmMain.ts:56-77`
**What:** Multiple commands registered via vscode.commands.registerCommand, some returning contextual values.

```typescript
context.subscriptions.push(vscode.commands.registerCommand('npm.runSelectedScript', runSelectedScript));

if (await hasPackageJson()) {
	vscode.commands.executeCommand('setContext', 'npm:showScriptExplorer', true);
}

context.subscriptions.push(vscode.commands.registerCommand('npm.runScriptFromFolder', selectAndRunScriptFromFolder));
context.subscriptions.push(vscode.commands.registerCommand('npm.refresh', () => {
	invalidateScriptCaches();
}));
context.subscriptions.push(vscode.commands.registerCommand('npm.scriptRunner', (args) => {
	if (args instanceof vscode.Uri) {
		return getScriptRunner(args, context, true);
	}
	return '';
}));
context.subscriptions.push(vscode.commands.registerCommand('npm.packageManager', (args) => {
	if (args instanceof vscode.Uri) {
		return getPackageManager(args, context, true);
	}
	return '';
}));
```

**Variations / call-sites:** `npmView.ts:147-150` (tree explorer commands), `scriptHover.ts:37-38` (hover provider commands)

---

## Child Process Execution Pattern

#### Pattern: Process Execution via child_process.execFile
**Where:** `extensions/npm/src/features/packageJSONContribution.ts:303-323`
**What:** Async npm command execution with cwd resolution, environment variable setup for corepack compatibility, and platform-specific shell handling.

```typescript
private async runNpmCommand(npmCommandPath: string, args: string[], resource: Uri | undefined): Promise<string | undefined> {
	const cp = await import('child_process');
	return new Promise((resolve, _reject) => {
		const cwd = resource && resource.scheme === 'file' ? dirname(resource.fsPath) : undefined;

		// corepack npm wrapper would automatically update package.json. disable that behavior.
		const env = { ...process.env, COREPACK_ENABLE_AUTO_PIN: '0', COREPACK_ENABLE_PROJECT_SPEC: '0' };
		let options: cp.ExecFileOptions = { cwd, env };
		let commandPath: string = npmCommandPath;
		if (process.platform === 'win32') {
			options = { cwd, env, shell: true };
			commandPath = `"${npmCommandPath}"`;
		}
		cp.execFile(commandPath, args, options, (error, stdout) => {
			resolve(error ? undefined : stdout);
		});
	});
}
```

**Variations / call-sites:** `packageJSONContribution.ts:325-344` (npmView variant), `packageJSONContribution.ts:369-381` (npmListInstalledVersion variant)

---

## JSON Parsing Pattern for Script Discovery

#### Pattern: Declarative JSON Visitor Pattern with jsonc-parser
**Where:** `extensions/npm/src/readScripts.ts:21-73`
**What:** Uses jsonc-parser's visitor pattern to traverse package.json AST and extract npm scripts with position tracking.

```typescript
export const readScripts = (document: TextDocument, buffer = document.getText()): INpmScriptInfo | undefined => {
	let start: Position | undefined;
	let end: Position | undefined;
	let inScripts = false;
	let buildingScript: { name: string; nameRange: Range } | void;
	let level = 0;

	const scripts: INpmScriptReference[] = [];
	const visitor: JSONVisitor = {
		onError() { /* no-op */ },
		onObjectBegin() { level++; },
		onObjectEnd(offset) {
			if (inScripts) {
				end = document.positionAt(offset);
				inScripts = false;
			}
			level--;
		},
		onLiteralValue(value: unknown, offset: number, length: number) {
			if (buildingScript && typeof value === 'string') {
				scripts.push({
					...buildingScript,
					value,
					valueRange: new Range(document.positionAt(offset), document.positionAt(offset + length)),
				});
				buildingScript = undefined;
			}
		},
		onObjectProperty(property: string, offset: number, length: number) {
			if (level === 1 && property === 'scripts') {
				inScripts = true;
				start = document.positionAt(offset);
			} else if (inScripts) {
				buildingScript = {
					name: property,
					nameRange: new Range(document.positionAt(offset), document.positionAt(offset + length))
				};
			}
		},
	};

	visit(buffer, visitor);

	if (start === undefined) {
		return undefined;
	}

	return { location: new Location(document.uri, new Range(start, end ?? start)), scripts };
};
```

**Variations / call-sites:** `npmView.ts:164`, `scriptHover.ts:60`, `npmScriptLens.ts:69`

---

## Hover Provider Pattern

#### Pattern: Hover Provider with Markdown Commands
**Where:** `extensions/npm/src/scriptHover.ts:33-74, 112-119`
**What:** Implements HoverProvider with markdown-based command links and hover lifecycle management.

```typescript
export class NpmScriptHoverProvider implements HoverProvider {
	private enabled: boolean;

	constructor(private context: ExtensionContext) {
		context.subscriptions.push(commands.registerCommand('npm.runScriptFromHover', this.runScriptFromHover, this));
		context.subscriptions.push(commands.registerCommand('npm.debugScriptFromHover', this.debugScriptFromHover, this));
		context.subscriptions.push(workspace.onDidChangeTextDocument((e) => {
			invalidateHoverScriptsCache(e.document);
		}));

		const isEnabled = () => workspace.getConfiguration('npm').get<boolean>('scriptHover', true);
		this.enabled = isEnabled();
		context.subscriptions.push(workspace.onDidChangeConfiguration((e) => {
			if (e.affectsConfiguration('npm.scriptHover')) {
				this.enabled = isEnabled();
			}
		}));
	}

	public provideHover(document: TextDocument, position: Position, _token: CancellationToken): ProviderResult<Hover> {
		if (!this.enabled) {
			return;
		}

		let hover: Hover | undefined = undefined;

		if (!cachedDocument || cachedDocument.fsPath !== document.uri.fsPath) {
			cachedScripts = readScripts(document);
			cachedDocument = document.uri;
		}

		cachedScripts?.scripts.forEach(({ name, nameRange }) => {
			if (nameRange.contains(position)) {
				const contents: MarkdownString = new MarkdownString();
				contents.isTrusted = true;
				contents.appendMarkdown(this.createRunScriptMarkdown(name, document.uri));
				contents.appendMarkdown(this.createDebugScriptMarkdown(name, document.uri));
				hover = new Hover(contents);
			}
		});
		return hover;
	}

	public async runScriptFromHover(args: any) {
		const script = args.script;
		const documentUri = args.documentUri;
		const folder = workspace.getWorkspaceFolder(documentUri);
		if (folder) {
			const task = await createScriptRunnerTask(this.context, script, folder, documentUri);
			await tasks.executeTask(task);
		}
	}
}
```

**Variations / call-sites:** `npmMain.ts:153-164` (registration), `npmMain.ts:54` (registration call)

---

## Configuration-Driven Package Manager Detection

#### Pattern: Auto-Detection with Configuration Fallback
**Where:** `extensions/npm/src/tasks.ts:130-166`
**What:** Detects package manager from lock files or configuration, with multi-PM detection warnings.

```typescript
export async function getScriptRunner(folder: Uri, context?: ExtensionContext, showWarning?: boolean): Promise<string> {
	let scriptRunner = workspace.getConfiguration('npm', folder).get<string>('scriptRunner', 'npm');

	if (scriptRunner === 'auto') {
		scriptRunner = await detectPackageManager(folder, context, showWarning);
	}

	return scriptRunner;
}

export async function detectPackageManager(folder: Uri, extensionContext?: ExtensionContext, showWarning: boolean = false): Promise<string> {
	const { name, multipleLockFilesDetected: multiplePMDetected } = await findPreferredPM(folder.fsPath);
	const neverShowWarning = 'npm.multiplePMWarning.neverShow';
	if (showWarning && multiplePMDetected && extensionContext && !extensionContext.globalState.get<boolean>(neverShowWarning)) {
		const multiplePMWarning = l10n.t('Using {0} as the preferred package manager. Found multiple lockfiles for {1}...', name, folder.fsPath);
		const neverShowAgain = l10n.t("Do not show again");
		const learnMore = l10n.t("Learn more");
		window.showInformationMessage(multiplePMWarning, learnMore, neverShowAgain).then(result => {
			switch (result) {
				case neverShowAgain: extensionContext.globalState.update(neverShowWarning, true); break;
				case learnMore: env.openExternal(Uri.parse('https://docs.npmjs.com/cli/v9/configuring-npm/package-lock-json'));
			}
		});
	}

	return name;
}
```

**Variations / call-sites:** `tasks.ts:140-148` (getPackageManager variant), `npmView.ts:155` (usage in explorer)

---

## Tree Data Provider Pattern

#### Pattern: Hierarchical Tree Data Provider with Filtering
**Where:** `extensions/npm/src/npmView.ts:138-334`
**What:** Custom TreeDataProvider that builds hierarchical task tree from flat task list with exclusion pattern support.

```typescript
export class NpmScriptsTreeDataProvider implements TreeDataProvider<TreeItem> {
	private taskTree: TaskTree | null = null;
	private extensionContext: ExtensionContext;
	private _onDidChangeTreeData: EventEmitter<TreeItem | null> = new EventEmitter<TreeItem | null>();
	readonly onDidChangeTreeData: Event<TreeItem | null> = this._onDidChangeTreeData.event;

	constructor(private context: ExtensionContext, public taskProvider: NpmTaskProvider) {
		const subscriptions = context.subscriptions;
		this.extensionContext = context;
		subscriptions.push(commands.registerCommand('npm.runScript', this.runScript, this));
		subscriptions.push(commands.registerCommand('npm.debugScript', this.debugScript, this));
		subscriptions.push(commands.registerCommand('npm.openScript', this.openScript, this));
		subscriptions.push(commands.registerCommand('npm.runInstall', this.runInstall, this));
	}

	private async runScript(script: NpmScript) {
		await detectPackageManager(script.getFolder().uri, this.context, true);
		tasks.executeTask(script.task);
	}

	async getChildren(element?: TreeItem): Promise<TreeItem[]> {
		if (!this.taskTree) {
			const taskItems = await this.taskProvider.tasksWithLocation;
			if (taskItems) {
				const taskTree = this.buildTaskTree(taskItems);
				this.taskTree = this.sortTaskTree(taskTree);
				if (this.taskTree.length === 0) {
					let message = l10n.t("No scripts found.");
					if (!isAutoDetectionEnabled()) {
						message = l10n.t('The setting "npm.autoDetect" is "off".');
					}
					this.taskTree = [new NoScripts(message)];
				}
			}
		}
		if (element instanceof Folder) {
			return element.packages;
		}
		if (element instanceof PackageJSON) {
			return element.scripts;
		}
		if (!element) {
			if (this.taskTree) {
				return this.taskTree;
			}
		}
		return [];
	}

	private buildTaskTree(tasks: ITaskWithLocation[]): TaskTree {
		const folders: Map<String, Folder> = new Map();
		const packages: Map<String, PackageJSON> = new Map();

		tasks.forEach(each => {
			const location = each.location;
			if (location && !excludeConfig.has(location.uri.toString())) {
				const regularExpressionsSetting = workspace.getConfiguration('npm', location.uri).get<string[]>('scriptExplorerExclude', []);
				excludeConfig.set(location.uri.toString(), regularExpressionsSetting?.map(value => RegExp(value)));
			}
			const regularExpressions = (location && excludeConfig.has(location.uri.toString())) ? excludeConfig.get(location.uri.toString()) : undefined;

			if (regularExpressions && regularExpressions.some((regularExpression) => (<INpmTaskDefinition>each.task.definition).script.match(regularExpression))) {
				return; // skip excluded scripts
			}
			// ... build tree structure
		});
		return [...folders.values()];
	}
}
```

**Variations / call-sites:** `npmMain.ts:143-150` (tree view creation and registration)

---

## CodeLens Provider Pattern

#### Pattern: Configuration-Driven CodeLens with Debug Integration
**Where:** `extensions/npm/src/npmScriptLens.ts:32-107`
**What:** CodeLens provider that conditionally displays debug links based on configuration location setting.

```typescript
export class NpmScriptLensProvider implements CodeLensProvider, Disposable {
	private lensLocation = getFreshLensLocation();
	private readonly changeEmitter = new EventEmitter<void>();
	private subscriptions: Disposable[] = [];

	public readonly onDidChangeCodeLenses = this.changeEmitter.event;

	constructor() {
		this.subscriptions.push(
			this.changeEmitter,
			workspace.onDidChangeConfiguration(evt => {
				if (evt.affectsConfiguration(Constants.ConfigKey)) {
					this.lensLocation = getFreshLensLocation();
					this.changeEmitter.fire();
				}
			}),
			languages.registerCodeLensProvider(
				{
					language: 'json',
					pattern: '**/package.json',
				},
				this,
			)
		);
	}

	public async provideCodeLenses(document: TextDocument): Promise<CodeLens[]> {
		if (this.lensLocation === 'never') {
			return [];
		}

		const tokens = readScripts(document);
		if (!tokens) {
			return [];
		}

		const title = '$(debug-start) ' + l10n.t("Debug");
		const cwd = path.dirname(document.uri.fsPath);
		if (this.lensLocation === 'top') {
			return [
				new CodeLens(
					tokens.location.range,
					{
						title,
						command: 'extension.js-debug.npmScript',
						arguments: [cwd],
					},
				),
			];
		}

		if (this.lensLocation === 'all') {
			const folder = Uri.joinPath(document.uri, '..');
			return Promise.all(tokens.scripts.map(
				async ({ name, nameRange }) => {
					const runScriptCommand = await getRunScriptCommand(name, folder);
					return new CodeLens(
						nameRange,
						{
							title,
							command: 'extension.js-debug.createDebuggerTerminal',
							arguments: [runScriptCommand.join(' '), workspace.getWorkspaceFolder(document.uri), { cwd }],
						},
					);
				},
			));
		}

		return [];
	}
}
```

**Variations / call-sites:** `npmMain.ts:78` (subscription push in activate)

---

## Extension Activation Lifecycle

#### Pattern: Full Extension Activation with Async Setup and Subscriptions
**Where:** `extensions/npm/src/npmMain.ts:26-103`
**What:** Comprehensive activation showing configuration listeners, provider registration, command setup, and subscription management.

```typescript
export async function activate(context: vscode.ExtensionContext): Promise<void> {
	configureHttpRequest();
	context.subscriptions.push(vscode.workspace.onDidChangeConfiguration(e => {
		if (e.affectsConfiguration('http.proxy') || e.affectsConfiguration('http.proxyStrictSSL')) {
			configureHttpRequest();
		}
	}));

	const npmCommandPath = await getNPMCommandPath();
	context.subscriptions.push(addJSONProviders(httpRequest.xhr, npmCommandPath));
	registerTaskProvider(context);

	treeDataProvider = registerExplorer(context);

	context.subscriptions.push(vscode.workspace.onDidChangeConfiguration((e) => {
		if (e.affectsConfiguration('npm.exclude') || e.affectsConfiguration('npm.autoDetect') || e.affectsConfiguration('npm.scriptExplorerExclude') || e.affectsConfiguration('npm.runSilent') || e.affectsConfiguration('npm.packageManager') || e.affectsConfiguration('npm.scriptRunner')) {
			invalidateTasksCache();
			if (treeDataProvider) {
				treeDataProvider.refresh();
			}
		}
		if (e.affectsConfiguration('npm.scriptExplorerAction')) {
			if (treeDataProvider) {
				treeDataProvider.refresh();
			}
		}
	}));

	registerHoverProvider(context);

	context.subscriptions.push(vscode.commands.registerCommand('npm.runSelectedScript', runSelectedScript));

	if (await hasPackageJson()) {
		vscode.commands.executeCommand('setContext', 'npm:showScriptExplorer', true);
	}

	// ... more command registrations

	context.subscriptions.push(vscode.window.registerTerminalQuickFixProvider('ms-vscode.npm-command', {
		provideTerminalQuickFixes({ outputMatch }) {
			if (!outputMatch) {
				return;
			}

			const lines = outputMatch.regexMatch[1];
			const fixes: vscode.TerminalQuickFixTerminalCommand[] = [];
			for (const line of lines.split('\n')) {
				const begin = line.indexOf('npm', 1);
				if (begin === -1) {
					continue;
				}

				const end = line.lastIndexOf('#');
				fixes.push({ terminalCommand: line.slice(begin, end === -1 ? undefined : end - 1) });
			}

			return fixes;
		},
	}));
}
```

---

## Summary

The `extensions/npm/` codebase demonstrates integrated patterns for a complete npm script management system:

1. **Task Provider Pattern**: Full TaskProvider implementation with file watching, caching, and dynamic task discovery
2. **Process Execution**: Platform-aware subprocess execution with environment variable configuration
3. **JSON Parsing**: Declarative visitor-based parsing for extracting npm scripts with position tracking
4. **Command Integration**: Multi-level command registration (UI commands, hover commands, explorer commands)
5. **Tree Exploration**: Hierarchical TreeDataProvider with dynamic filtering and exclusion patterns
6. **Configuration-Driven Behavior**: Extensive use of workspace configuration with reactive updates
7. **Hover/CodeLens Integration**: Rich editor integration with markdown-based command links
8. **Package Manager Detection**: Auto-detection with fallback and multi-PM warnings

**Key architectural insights for porting:**
- Task execution uses `ShellExecution` abstraction (high-level API, not direct child_process)
- Configuration drives behavior (package manager, auto-detection, script exclusions, hover settings)
- Script discovery decoupled from execution via TaskProvider interface
- JSON parsing uses declarative visitors, not regex
- Process spawning wrapped in Promise-based utilities
- Subscription-based lifecycle management for all resources

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
