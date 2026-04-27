### Files Analysed

- `extensions/npm/src/tasks.ts` (493 LOC) — Core TaskProvider, task construction, package-manager detection
- `extensions/npm/src/npmMain.ts` (173 LOC) — Extension activation, provider registration, lifecycle
- `extensions/npm/src/commands.ts` (67 LOC) — `runSelectedScript`, `selectAndRunScriptFromFolder` command handlers
- `extensions/npm/src/npmView.ts` (334 LOC) — `NpmScriptsTreeDataProvider`, tree hierarchy model
- `extensions/npm/src/scriptHover.ts` (130 LOC) — `NpmScriptHoverProvider`, hover action links
- `extensions/npm/src/npmScriptLens.ts` (115 LOC) — `NpmScriptLensProvider`, CodeLens debug buttons
- `extensions/npm/src/readScripts.ts` (73 LOC) — JSON visitor that extracts scripts from `package.json`
- `extensions/npm/src/preferred-pm.ts` (113 LOC) — Lock-file heuristics to pick npm/yarn/pnpm/bun

---

### Per-File Notes

#### `extensions/npm/src/tasks.ts`

- **Role:** Implements `NpmTaskProvider` (the `vscode.TaskProvider` contract) and all factory functions that produce `vscode.Task` objects backed by `ShellExecution`.
- **Key symbols:**
  - `NpmTaskProvider` (`tasks.ts:46`) — class implementing `TaskProvider`; `provideTasks()` at `tasks.ts:55` delegates to `provideNpmScripts()`; `resolveTask()` at `tasks.ts:60` reconstructs a task from a saved definition.
  - `provideNpmScripts()` (`tasks.ts:229`) — module-level cache gate; populates `cachedTasks` by iterating `findNpmPackages()` then calling `provideNpmScriptsForFolder()`.
  - `findNpmPackages()` (`tasks.ts:185`) — async generator; calls `workspace.findFiles(relativePattern, '**/node_modules/**')` for each enabled workspace folder, yielding each `package.json` URI.
  - `provideNpmScriptsForFolder()` (`tasks.ts:272`) — reads scripts via `getScripts()`, calls `createScriptRunnerTask()` per script, appends an install task.
  - `createScriptRunnerTask()` (`tasks.ts:334`) — builds `INpmTaskDefinition`, resolves cwd, calls `getRunScriptCommand()`, constructs `new Task(kind, folder, taskName, 'npm', new ShellExecution(scriptRunner, escapeCommandLine(args), { cwd }))`, assigns `TaskGroup` based on name heuristics (Build/Test/Clean/Rebuild).
  - `createInstallationTask()` (`tasks.ts:371`) — same pattern but uses `getInstallDependenciesCommand()` and always assigns `TaskGroup.Clean`.
  - `getRunScriptCommand()` (`tasks.ts:319`) — returns `['node', '--run', script]` when scriptRunner is `'node'`; otherwise `[runner, 'run', ...optionalSilent, script]`.
  - `escapeCommandLine()` (`tasks.ts:304`) — maps args to `ShellQuotedString` with `ShellQuoting.Weak` for `--` args, `ShellQuoting.Strong` for others containing whitespace.
  - `startDebugging()` (`tasks.ts:447`) — fires `extension.js-debug.createDebuggerTerminal` command with the raw run-script command string, not a Task.
  - `runScript()` (`tasks.ts:438`) — calls `tasks.executeTask(task)` directly.
  - `invalidateTasksCache()` (`tasks.ts:89`) — sets `cachedTasks = undefined`.
  - `getPackageManager()` / `getScriptRunner()` (`tasks.ts:140`, `tasks.ts:130`) — read `npm.packageManager` / `npm.scriptRunner` config, fall back to `detectPackageManager()` when value is `'auto'`.
- **Control flow:** `provideTasks()` → `provideNpmScripts()` → `findNpmPackages()` → `provideNpmScriptsForFolder()` → `getScripts()` (parses JSON) + `createScriptRunnerTask()` per script → returns `Task[]`.
- **Data flow:** `package.json` URI → `readScripts()` yields `{ name, value, nameRange }` tuples → `createScriptRunnerTask()` produces a `Task` with `ShellExecution` holding a shell command array → task stored in module-level `cachedTasks` array.
- **Dependencies:** `vscode` API (`Task`, `ShellExecution`, `ShellQuotedString`, `ShellQuoting`, `TaskGroup`, `workspace`, `tasks`, `commands`, `window`, `env`), Node `path`, Node `fs`, `minimatch`, `vscode-uri`, local `./preferred-pm`, local `./readScripts`.

---

#### `extensions/npm/src/npmMain.ts`

- **Role:** Extension entry point; orchestrates registration of all providers and commands on activation.
- **Key symbols:**
  - `activate()` (`npmMain.ts:26`) — async; called by VS Code runtime when extension activates.
  - `registerTaskProvider()` (`npmMain.ts:124`) — creates a `FileSystemWatcher` on `**/package.json`, hooks `onDidChange/Delete/Create` to `invalidateScriptCaches()`; instantiates `NpmTaskProvider` and calls `vscode.tasks.registerTaskProvider('npm', taskProvider)` at `npmMain.ts:136`.
  - `registerExplorer()` (`npmMain.ts:143`) — calls `vscode.window.createTreeView('npm', { treeDataProvider })` at `npmMain.ts:146`.
  - `registerHoverProvider()` (`npmMain.ts:153`) — calls `vscode.languages.registerHoverProvider()` with a JSON/`**/package.json` selector at `npmMain.ts:160–162`.
  - `invalidateScriptCaches()` (`npmMain.ts:18`) — calls `invalidateHoverScriptsCache()`, `invalidateTasksCache()`, and `treeDataProvider.refresh()`.
  - Terminal quick-fix provider registered at `npmMain.ts:80`; parses npm error output for suggested commands.
  - `getNPMCommandPath()` (`npmMain.ts:105`) — uses `which` to find npm binary; guards on workspace trust and `file`-scheme folders.
- **Control flow:** `activate()` → `configureHttpRequest()` → `getNPMCommandPath()` → `addJSONProviders()` → `registerTaskProvider()` → `registerExplorer()` → register commands → `registerHoverProvider()` → `NpmScriptLensProvider` construction.
- **Data flow:** `ExtensionContext.subscriptions` accumulates all disposables; configuration changes flow through `onDidChangeConfiguration` listeners to cache invalidation.
- **Dependencies:** `vscode`, `request-light`, `which`, local `./features/jsonContributions`, `./commands`, `./npmView`, `./tasks`, `./scriptHover`, `./npmScriptLens`.

---

#### `extensions/npm/src/commands.ts`

- **Role:** Implements the two user-facing command handlers for running scripts via cursor position or folder-level quick-pick.
- **Key symbols:**
  - `runSelectedScript()` (`commands.ts:16`) — reads `editor.selection.anchor`, calls `findScriptAtPosition()` to resolve the script name, then calls `runScript()` (which executes a Task).
  - `selectAndRunScriptFromFolder()` (`commands.ts:32`) — calls `detectNpmScriptsForFolder()` to get all `IFolderTaskItem[]` for a given URI, shows a `QuickPick`, and on accept calls `vscode.tasks.executeTask(result.task)` at `commands.ts:61`.
- **Control flow:** Command invoked by VS Code → handler fetches relevant task(s) → either immediately runs or presents a quick-pick → `vscode.tasks.executeTask()`.
- **Data flow:** `vscode.Uri` (folder) → `detectNpmScriptsForFolder()` returns `IFolderTaskItem[]` → user selects → `Task` object passed to `tasks.executeTask()`.
- **Dependencies:** `vscode`, local `./tasks` (`detectNpmScriptsForFolder`, `findScriptAtPosition`, `runScript`, `IFolderTaskItem`).

---

#### `extensions/npm/src/npmView.ts`

- **Role:** Implements the `NpmScriptsTreeDataProvider` powering the "NPM Scripts" sidebar explorer with a three-level hierarchy: `Folder → PackageJSON → NpmScript`.
- **Key symbols:**
  - `NpmScriptsTreeDataProvider` (`npmView.ts:138`) — implements `TreeDataProvider<TreeItem>`; holds `taskTree` cache, fires `_onDidChangeTreeData` event.
  - `getChildren()` (`npmView.ts:229`) — on first call fetches `taskProvider.tasksWithLocation`, calls `buildTaskTree()`, then `sortTaskTree()`; subsequent calls return cached children.
  - `buildTaskTree()` (`npmView.ts:289`) — iterates `ITaskWithLocation[]`, applies `scriptExplorerExclude` regex filters, groups tasks into `Folder`/`PackageJSON`/`NpmScript` tree nodes; collapses to `PackageJSON[]` when only one folder.
  - `NpmScript` tree item (`npmView.ts:76`) — constructor assigns either `vscode.open` or `npm.runScript` as the default click command based on the `npm.scriptExplorerAction` config.
  - `runScript()` (`npmView.ts:153`) — calls `detectPackageManager()` for the warning side-effect, then `tasks.executeTask(script.task)`.
  - `debugScript()` (`npmView.ts:159`) — calls `startDebugging()` from `./tasks`.
  - `openScript()` (`npmView.ts:189`) — opens the `package.json` document and positions cursor at the script's `nameRange.start`.
  - `runInstall()` (`npmView.ts:177`) — calls `createInstallationTask()` then `tasks.executeTask()`.
  - `refresh()` (`npmView.ts:204`) — nulls `taskTree` and fires the change event.
- **Control flow:** Tree expand event → `getChildren()` → `taskProvider.tasksWithLocation` → `buildTaskTree()` → return node list. User click → command handler → `tasks.executeTask()`.
- **Data flow:** `ITaskWithLocation[]` from `NpmTaskProvider.tasksWithLocation` → grouped into tree nodes → rendered in sidebar; task objects flow unchanged into `tasks.executeTask()`.
- **Dependencies:** `vscode`, local `./readScripts`, `./tasks` (multiple imports).

---

#### `extensions/npm/src/scriptHover.ts`

- **Role:** `HoverProvider` for `**/package.json` files; surfaces "Run Script" and "Debug Script" markdown command links when the cursor is over a script name.
- **Key symbols:**
  - `NpmScriptHoverProvider` (`scriptHover.ts:33`) — implements `HoverProvider`.
  - `provideHover()` (`scriptHover.ts:52`) — checks `cachedScripts` against current document, calls `readScripts()` if stale, iterates scripts to find one whose `nameRange.contains(position)`, builds `MarkdownString` with encoded command URIs.
  - `createMarkdownLink()` (`scriptHover.ts:103`) — encodes args as `encodeURIComponent(JSON.stringify(args))`, formats a `[label](command:cmd?encodedArgs "tooltip")` markdown link.
  - `runScriptFromHover()` (`scriptHover.ts:112`) — resolves folder, calls `createScriptRunnerTask()`, then `tasks.executeTask()`.
  - `debugScriptFromHover()` (`scriptHover.ts:122`) — calls `startDebugging()`.
  - Module-level `cachedDocument`/`cachedScripts` (`scriptHover.ts:20–21`) — single-document parse cache; invalidated by `workspace.onDidChangeTextDocument`.
- **Control flow:** Hover event → `provideHover()` → parse cache check → `readScripts()` if needed → scan `nameRange` → build and return `Hover`.
- **Data flow:** Document text → `readScripts()` → `INpmScriptInfo.scripts[]` → position match → `MarkdownString` with command links; on click, command args decoded → `createScriptRunnerTask()` → `tasks.executeTask()`.
- **Dependencies:** `vscode`, local `./readScripts`, `./tasks` (`createScriptRunnerTask`, `startDebugging`).

---

#### `extensions/npm/src/npmScriptLens.ts`

- **Role:** `CodeLensProvider` for `**/package.json`; inserts a "Debug" CodeLens above each npm script (or the whole scripts block) based on the `debug.javascript.codelens.npmScripts` setting.
- **Key symbols:**
  - `NpmScriptLensProvider` (`npmScriptLens.ts:32`) — implements `CodeLensProvider` and `Disposable`; registers itself via `languages.registerCodeLensProvider` at `npmScriptLens.ts:51–58`.
  - `provideCodeLenses()` (`npmScriptLens.ts:64`) — reads `lensLocation`; if `'top'`, returns a single lens at `tokens.location.range` targeting `extension.js-debug.npmScript`; if `'all'`, maps each script to a lens targeting `extension.js-debug.createDebuggerTerminal` with the full run-script command string.
  - `getRunScriptCommand()` called at `npmScriptLens.ts:93` to build the terminal command for per-script lenses.
  - `changeEmitter` (`npmScriptLens.ts:34`) — `EventEmitter<void>` fires `onDidChangeCodeLenses` when config changes.
- **Control flow:** Document open → `provideCodeLenses()` → `readScripts()` → build `CodeLens[]` → rendered in editor. Config change → `changeEmitter.fire()` → VS Code re-requests lenses.
- **Data flow:** Document → `readScripts()` → script name + range → `getRunScriptCommand()` → `CodeLens` with encoded arguments for js-debug commands.
- **Dependencies:** `vscode`, local `./readScripts`, `./tasks` (`getRunScriptCommand`).

---

#### `extensions/npm/src/readScripts.ts`

- **Role:** Stateless JSON parser that extracts the `scripts` object from a `package.json` document, returning each script's name, value, and their exact document ranges.
- **Key symbols:**
  - `readScripts()` (`readScripts.ts:21`) — exported function; accepts a `TextDocument` and optional string buffer; uses `jsonc-parser`'s `visit()` with a `JSONVisitor` to walk the JSON tree.
  - JSON visitor callbacks: `onObjectProperty` at `readScripts.ts:53` detects the `scripts` key at depth 1 (`level === 1`) and each script name within it; `onLiteralValue` at `readScripts.ts:43` captures the script value string and pushes an `INpmScriptReference`.
  - `INpmScriptReference` (`readScripts.ts:9`) — `{ name, value, nameRange, valueRange }`.
  - `INpmScriptInfo` (`readScripts.ts:16`) — `{ location: Location, scripts: INpmScriptReference[] }`.
- **Control flow:** Called synchronously; `visit()` drives the visitor; returns `undefined` if no `scripts` key found, otherwise `INpmScriptInfo`.
- **Data flow:** Raw document text → `visit()` visitor callbacks → accumulates `INpmScriptReference[]` → returned as `INpmScriptInfo` with a `Location` spanning the scripts block.
- **Dependencies:** `jsonc-parser` (`visit`, `JSONVisitor`), `vscode` (`Location`, `Position`, `Range`, `TextDocument`).

---

#### `extensions/npm/src/preferred-pm.ts`

- **Role:** Heuristically detects which package manager (npm/pnpm/yarn/bun) is preferred for a given directory by checking for lockfiles; also uses `which-pm` to inspect installed packages.
- **Key symbols:**
  - `findPreferredPM()` (`preferred-pm.ts:71`) — async; calls all four detector functions in sequence, collects detected names and properties, consults `whichPM()` for the installation-time manager, counts lockfiles to set `multipleLockFilesDetected`.
  - `isNPMPreferred()` (`preferred-pm.ts:66`) — checks `package-lock.json`.
  - `isPNPMPreferred()` (`preferred-pm.ts:38`) — checks `pnpm-lock.yaml`, `shrinkwrap.yaml`, and `find-up` traversal.
  - `isYarnPreferred()` (`preferred-pm.ts:52`) — checks `yarn.lock`, then `findWorkspaceRoot()`.
  - `isBunPreferred()` (`preferred-pm.ts:26`) — checks `bun.lockb` and `bun.lock`.
  - `pathExists()` (`preferred-pm.ts:17`) — uses `workspace.fs.stat()` (VS Code VFS API) instead of Node `fs`.
- **Control flow:** `findPreferredPM(pkgPath)` → four `is*Preferred()` checks → `whichPM()` → build result; returns first detected name or `'npm'`.
- **Data flow:** Directory path string → lockfile existence checks via VS Code VFS → array of detected names → first element becomes the recommended package manager name.
- **Dependencies:** `find-yarn-workspace-root`, `find-up`, `path`, `which-pm`, `vscode` (`Uri`, `workspace`).

---

### Cross-Cutting Synthesis

The npm extension is a pure VS Code extension that converts `package.json` scripts into first-class VS Code `Task` objects executed as shell processes via `ShellExecution`. Every user-visible feature — the sidebar tree (`npmView.ts`), hover links (`scriptHover.ts`), CodeLens buttons (`npmScriptLens.ts`), and command palette (`commands.ts`) — ultimately calls either `vscode.tasks.executeTask(task)` or `commands.executeCommand('extension.js-debug.createDebuggerTerminal', ...)` with a `Task` produced by `createScriptRunnerTask()` or `createInstallationTask()` in `tasks.ts`. The `NpmTaskProvider` is registered as a named provider `'npm'` via `vscode.tasks.registerTaskProvider`, making these tasks resolvable by name from `tasks.json`. JSON parsing for `package.json` is done entirely in-process via `jsonc-parser`'s visitor API, with results cached per-document. Package manager selection is a lockfile heuristic producing a plain string (`'npm'`, `'yarn'`, `'pnpm'`, `'bun'`, or `'node'`), which directly becomes the shell executable name in `ShellExecution`.

For a Tauri/Rust port, the entire Tasks API (`vscode.Task`, `ShellExecution`, `ShellQuotedString`, `TaskProvider`, `TaskGroup`, `vscode.tasks.registerTaskProvider`, `vscode.tasks.executeTask`) would need a native equivalent. The JSON parsing logic in `readScripts.ts` is self-contained and portable. The tree-view, hover-provider, and CodeLens-provider all rely on `vscode.window` and `vscode.languages` extension APIs that have no direct Tauri equivalent.

---

### Out-of-Partition References

- `extensions/npm/src/features/jsonContributions.ts` — JSON language contribution provider abstraction used in `npmMain.ts:35` (`addJSONProviders`); provides npm package name/version completions via the JSON language service.
- `extensions/npm/src/features/packageJSONContribution.ts` — Concrete `IJSONContribution` implementation that queries npm registry for package metadata completions.
- `extensions/npm/src/npmBrowserMain.ts` — Browser-only activation entry point (15 LOC); registers only `addJSONProviders`, skipping all task/tree/hover providers.
- `vscode` (API package) — `Task`, `ShellExecution`, `ShellQuotedString`, `ShellQuoting`, `TaskGroup`, `TaskProvider`, `TaskScope`, `TreeDataProvider`, `HoverProvider`, `CodeLensProvider`, `languages`, `tasks`, `window`, `workspace`, `commands` — the entire extension surface that would require porting.
- `extension.js-debug.createDebuggerTerminal` (command) — cross-extension command in the js-debug extension; receives script command string and spawns a debugger-attached terminal; called from `tasks.ts:450` and `npmScriptLens.ts:98`.
- `extension.js-debug.npmScript` (command) — js-debug command referenced from `npmScriptLens.ts:83` for the `'top'` lens mode.
- `jsonc-parser` (npm package) — `visit()` / `JSONVisitor` used in `readScripts.ts:6`; provides the fault-tolerant JSON visitor used to parse `package.json`.
- `which-pm` (npm package) — used in `preferred-pm.ts:9` to detect the package manager used during installation.
- `find-yarn-workspace-root` (npm package) — used in `preferred-pm.ts:6` for Yarn workspace detection.
- `request-light` (npm package) — used in `npmMain.ts:6` to configure HTTP proxy for registry requests made by `jsonContributions`.
