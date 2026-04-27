# Partition 27 of 79 — Findings

## Scope
`extensions/npm/` (14 files, 2,372 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# VS Code npm Extension - Tasks API Surface for Tauri/Rust Port

## Research Question
What it would take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

## Partition 27: `extensions/npm/` - npm Script Tasks Integration

### Overview
The npm extension (14 files, 2,372 LOC) is a bundled VS Code extension that provides npm script task automation. It demonstrates critical patterns for porting VS Code's **Tasks API** to Tauri/Rust: task definition, registration, execution via shell, and task lifecycle management.

---

## Implementation Files

- `src/tasks.ts` (493 LOC) - **Core Tasks API integration**
  - `NpmTaskProvider` class implementing `TaskProvider` interface
  - Task creation: `createScriptRunnerTask()`, `createInstallationTask()`
  - Uses `ShellExecution` API with cwd, arguments, shell quoting
  - Task grouping: Build/Test/Clean/Rebuild groups
  - Package manager detection and script runner configuration
  - Task cache invalidation pattern
  - Imports: `TaskDefinition`, `Task`, `TaskGroup`, `TaskProvider`, `TaskScope`, `ShellExecution`, `ShellQuotedString`, `ShellQuoting` from vscode

- `src/npmMain.ts` (173 LOC) - **Extension entry point and task provider registration**
  - `registerTaskProvider()` - calls `vscode.tasks.registerTaskProvider('npm', taskProvider)`
  - Task cache invalidation on configuration changes
  - HTTP proxy configuration for npm registry requests
  - Explorer view registration
  - Hover provider and script lens registration
  - Terminal quick fix provider for npm command suggestions

- `src/commands.ts` (67 LOC) - **Command handlers**
  - `runSelectedScript()`, `selectAndRunScriptFromFolder()` - execute npm tasks via `vscode.tasks.executeTask()`
  - Script selection UI (QuickPick)
  - Debug script execution

- `src/npmView.ts` (334 LOC) - **Explorer tree view for npm scripts**
  - `NpmScriptsTreeDataProvider` implementing `TreeDataProvider<TreeItem>`
  - Tree structure: Folder → PackageJSON → NpmScript nodes
  - Task execution from tree items via `tasks.executeTask()`
  - Script debugging via `startDebugging()`
  - Context menus for run/debug/open actions

- `src/scriptHover.ts` (130 LOC) - **Hover provider for package.json scripts**
  - `NpmScriptHoverProvider` implementing `HoverProvider`
  - Displays script details and executes tasks on click
  - Caches script hover information

- `src/npmScriptLens.ts` (115 LOC) - **CodeLens provider**
  - Provides inline "Run Script" and "Debug Script" code lenses
  - Integrated with script explorer configuration

- `src/readScripts.ts` (73 LOC) - **Package.json script parsing**
  - Reads and parses "scripts" field from package.json
  - Returns script name/value pairs

- `src/preferred-pm.ts` (113 LOC) - **Package manager detection**
  - Detects yarn/pnpm/bun presence via lockfile checks
  - Falls back to npm
  - Uses `which` module to verify CLI availability

- `src/features/packageJSONContribution.ts` - **JSON language features**
  - Autocomplete and hover for package.json dependencies
  - Fetches metadata from npm registry and bower registry
  - Schema validation integration

- `src/features/jsonContributions.ts` - **JSON provider abstraction**
  - Interfaces: `ISuggestionsCollector`, `IJSONContribution`
  - HTTP request adapter for registry queries

- `src/features/date.ts` - **Date utility for package info**
  - Helper for formatting package timestamps

- `src/npmBrowserMain.ts` (15 LOC) - **Browser/web entry point**
  - Minimal activation - only JSON providers (no task execution in browser)

---

## Configuration & Manifest

- `package.json` - Extension manifest
  - **Activation events**: `onTaskType:npm`, `onLanguage:json`, `workspaceContains:package.json`
  - **Task definition contribution**: Defines `npm` task type with `script` (required) and `path` properties
  - **API proposals**: `terminalQuickFixProvider`
  - **Commands**: npm.runScript, npm.debugScript, npm.openScript, npm.runInstall, npm.refresh, npm.runSelectedScript, npm.runScriptFromFolder, npm.packageManager
  - **Views**: npm explorer in sidebar
  - **Configuration options**: autoDetect, runSilent, packageManager, scriptRunner, exclude, scriptExplorerAction, fetchOnlinePackageInfo, scriptHover
  - **Terminal quick fixes**: npm command error suggestion injection
  - **Main entry**: `out/npmMain` (Node.js), **Browser**: `dist/browser/npmBrowserMain`

- `package.nls.json` - Localization strings

- `tsconfig.json` - TypeScript configuration for Node.js target
- `tsconfig.browser.json` - TypeScript configuration for browser/web target

- `.npmrc` - npm client config

- `esbuild.mts` - Build configuration (Node.js bundle entry: npmMain.ts)
- `esbuild.browser.mts` - Build configuration (browser bundle)

- `.vscode/launch.json` - Debug configuration
- `.vscode/tasks.json` - Local development tasks
- `.vscodeignore` - Package exclusions

---

## Documentation

- `README.md` - Extension overview
  - Task Running: scripts as auto-detected tasks (build/compile/watch = build group)
  - Script Explorer: tree view for npm scripts
  - Run from Editor: hover/command to execute script from package.json
  - Run from Folder: context menu for folder-level script discovery
  - Package info: fetches from npmjs.org and bower.io registries
  - Settings documentation: npm.autoDetect, npm.runSilent, npm.packageManager, npm.scriptRunner, npm.exclude, npm.scriptExplorerAction, npm.fetchOnlinePackageInfo

---

## Types & Interfaces

**Key API Surface Definitions:**

- `INpmTaskDefinition extends TaskDefinition` - npm task type with `script: string` and optional `path: string`
- `IFolderTaskItem extends QuickPickItem` - task selection list item
- `ITaskWithLocation` - task + optional source location (for go-to-definition in package.json)
- `ITaskLocation` - document URI and line position
- `TaskProvider` (vscode API) - interface with `provideTasks()` and `resolveTask()` methods
- `NpmTaskProvider` - concrete implementation (line 46 in tasks.ts)

**Shell Execution Pattern:**
```typescript
new Task(kind, folder, taskName, 'npm', new ShellExecution(scriptRunner, escapeCommandLine(args), { cwd: cwd }))
```

---

## Notable Clusters

### Task Lifecycle Management
- Task registration in `npmMain.ts:registerTaskProvider()` → `vscode.tasks.registerTaskProvider('npm', taskProvider)`
- Task creation: two paths via `NpmTaskProvider.provideTasks()` (discovery) and `resolveTask()` (lazy resolution)
- Task execution: `vscode.tasks.executeTask(task)` called from commands, hover, and view
- Cache invalidation: `invalidateTasksCache()` on config/workspace changes

### Package Manager Abstraction
- Detection hierarchy: configuration → lockfile checks → CLI availability
- Supported: npm, yarn, pnpm, bun
- Dual role: package manager (install dependencies) vs script runner (execute scripts)
- Configuration fallback: auto-detect or explicit user selection

### UI Integration Points
1. **Explorer View** (`npmView.ts`): Tree-based script discovery and execution
2. **Editor Hover** (`scriptHover.ts`): Inline script execution from package.json
3. **Code Lens** (`npmScriptLens.ts`): Inline run/debug buttons
4. **Command Palette**: Quick selection via quick-pick
5. **Context Menus**: Folder-level and view-item context actions
6. **Terminal Quick Fixes**: npm error suggestions in integrated terminal

### Process Execution Model
- **Shell execution wrapper**: `ShellExecution` abstracts shell invocation
- **Arguments**: Properly quoted (handles spaces/special chars via `ShellQuotedString`, `ShellQuoting` enums)
- **CWD management**: Set per task based on package.json location
- **Environment**: Inherits from host (proxy, PATH for detecting package managers)

### Data Flow for Task Execution
```
package.json (parsed via readScripts)
  → npm scripts discovered
  → task definition { type: 'npm', script: 'build', path?: 'packages/foo' }
  → Task created with ShellExecution
  → executeTask() runs via host's shell
```

---

## Summary

The npm extension demonstrates **three critical porting requirements for Tauri/Rust**:

1. **Tasks API Surface**: Must provide `registerTaskProvider()`, support `TaskDefinition`, `TaskProvider`, `Task`, and `ShellExecution` abstractions. Task execution must integrate with the host's shell and process management.

2. **Dynamic Process Spawning**: Heavy reliance on spawning npm/yarn/pnpm/bun CLIs as child processes with configurable arguments, working directories, and environment. Rust equivalent must provide robust shell quoting and process lifecycle.

3. **Configuration Extensibility**: Package manager and script runner detection taps into workspace configuration, lockfile parsing, and PATH lookups. A Rust port must replicate this detection logic and support user overrides.

4. **UI Provider Pattern**: TreeDataProvider, HoverProvider, CodeLens, and command-based UIs must be ported. The npm extension shows how to bind task operations to multiple UI entry points (tree, hover, lens, commands).

5. **Caching & Invalidation**: Task caching with event-driven invalidation (on config changes, file system events) is essential for performance in large monorepos.

The extension is relatively lightweight (1.5K LOC) but relies deeply on vscode APIs (`tasks.*`, `workspace.getConfiguration()`, `workspace.findFiles()`, `TreeDataProvider`, `HoverProvider`, `CodeLensProvider`). A Tauri port would need **native Rust bindings or IPC mechanisms** to expose equivalent task and configuration APIs to the GUI layer.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: VS Code Task System & npm Extension

## Research Question
What patterns in the npm extension exemplify task provider registration, child process execution, and package.json script discovery? These patterns inform porting VS Code's task system from TypeScript/Electron to Rust/Tauri.

## Scope Analysis
**extensions/npm/** (12 TypeScript files, ~2,372 LOC)
- Core task execution: `tasks.ts` (~500 LOC)
- Task provider registration: `npmMain.ts` (~175 LOC)
- Script discovery: `readScripts.ts`, `preferred-pm.ts`
- Execution UI: `npmView.ts`, `commands.ts`, `scriptHover.ts`

---

## Pattern Examples

### Pattern 1: TaskProvider Interface Implementation & Registration

**Where:** `extensions/npm/src/tasks.ts:46-86`, `extensions/npm/src/npmMain.ts:124-141`

**What:** Implements VS Code's TaskProvider interface with `provideTasks()` and `resolveTask()` methods; registers the provider at extension activation.

```typescript
// tasks.ts - TaskProvider class
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
			// Task resolution logic...
			let task: Task;
			if (kind.script === INSTALL_SCRIPT) {
				task = await createInstallationTask(this.context, _task.scope, packageJsonUri);
			} else {
				task = await createScriptRunnerTask(this.context, kind.script, _task.scope, packageJsonUri);
			}
			task.definition = kind;
			return task;
		}
		return undefined;
	}
}

// npmMain.ts - Registration at activation
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

**Variations / call-sites:**
- `npmMain.ts:36` - Called during `activate()`
- File watchers invalidate cached tasks on changes (lines 126-133)
- Workspace folder changes trigger cache invalidation
- Task provider is stored as module-level singleton for UI providers

**Rust/Tauri equivalent challenges:**
- Tauri has no built-in task provider registry—requires custom IPC channel for task discovery
- File system watchers need Rust bindings (e.g., `notify` crate)
- TaskProvider interface is VS Code-specific; port requires custom trait definition

---

### Pattern 2: Task Creation with ShellExecution & Command Building

**Where:** `extensions/npm/src/tasks.ts:334-360`, `extensions/npm/src/tasks.ts:371-387`

**What:** Constructs Task objects with ShellExecution (shell command invocation), handles package manager detection, and applies task grouping (Build/Test/Clean).

```typescript
export async function createScriptRunnerTask(
	context: ExtensionContext,
	script: string,
	folder: WorkspaceFolder,
	packageJsonUri: Uri,
	scriptValue?: string,
	showWarning?: boolean
): Promise<Task> {
	const kind: INpmTaskDefinition = { type: 'npm', script };

	const relativePackageJson = getRelativePath(folder.uri, packageJsonUri);
	if (relativePackageJson.length && !kind.path) {
		kind.path = relativePackageJson.substring(0, relativePackageJson.length - 1);
	}
	const taskName = getTaskName(script, relativePackageJson);
	const cwd = path.dirname(packageJsonUri.fsPath);
	const args = await getRunScriptCommand(script, folder.uri, context, showWarning);
	const scriptRunner = args.shift()!;
	const task = new Task(kind, folder, taskName, 'npm', new ShellExecution(
		scriptRunner,
		escapeCommandLine(args),
		{ cwd: cwd }
	));
	task.detail = scriptValue;

	const lowerCaseTaskName = script.toLowerCase();
	if (isBuildTask(lowerCaseTaskName)) {
		task.group = TaskGroup.Build;
	} else if (isTestTask(lowerCaseTaskName)) {
		task.group = TaskGroup.Test;
	} else if (canHavePrePostScript(lowerCaseTaskName)) {
		task.group = TaskGroup.Clean;
	}
	return task;
}

// Helper: Command builder
export async function getRunScriptCommand(
	script: string,
	folder: Uri,
	context?: ExtensionContext,
	showWarning = true
): Promise<string[]> {
	const scriptRunner = await getScriptRunner(folder, context, showWarning);

	if (scriptRunner === 'node') {
		return ['node', '--run', script];
	} else {
		const result = [scriptRunner, 'run'];
		if (workspace.getConfiguration('npm', folder).get<boolean>('runSilent')) {
			result.push('--silent');
		}
		result.push(script);
		return result;
	}
}

// Helper: Shell quoting for safety
function escapeCommandLine(cmd: string[]): (string | ShellQuotedString)[] {
	return cmd.map(arg => {
		if (/\s/.test(arg)) {
			return { value: arg, quoting: arg.includes('--') ? ShellQuoting.Weak : ShellQuoting.Strong };
		} else {
			return arg;
		}
	});
}
```

**Variations / call-sites:**
- `createInstallationTask()` follows same pattern for `npm install` (lines 371-387)
- `getRunScriptCommand()` branches on `scriptRunner` type (npm, yarn, pnpm, bun, node)
- `runSilent` config controls `--silent` flag appending
- Task grouping detects build/test/debug scripts by name matching

**Rust/Tauri equivalent challenges:**
- ShellExecution maps to `std::process::Command` or async spawn (tokio/tauri-core)
- Quote escaping differs (Windows vs Unix); Rust stdlib handles this
- No TaskGroup abstraction in Rust—requires custom enum
- Package manager detection needs async file I/O (async/await in Rust)

---

### Pattern 3: Package.json Discovery with Workspace Iteration

**Where:** `extensions/npm/src/tasks.ts:185-205`, `extensions/npm/src/tasks.ts:208-227`

**What:** Iterates workspace folders, uses `workspace.findFiles()` with relative patterns to locate all package.json files, applies exclusions, and caches results.

```typescript
async function* findNpmPackages(): AsyncGenerator<Uri> {
	const visitedPackageJsonFiles: Set<string> = new Set();

	const folders = workspace.workspaceFolders;
	if (!folders) {
		return;
	}
	for (const folder of folders) {
		if (isAutoDetectionEnabled(folder) && !excludeRegex.test(Utils.basename(folder.uri))) {
			const relativePattern = new RelativePattern(folder, '**/package.json');
			const paths = await workspace.findFiles(relativePattern, '**/{node_modules,.vscode-test}/**');
			for (const path of paths) {
				if (!isExcluded(folder, path) && !visitedPackageJsonFiles.has(path.fsPath)) {
					yield path;
					visitedPackageJsonFiles.add(path.fsPath);
				}
			}
		}
	}
}

export async function detectNpmScriptsForFolder(
	context: ExtensionContext,
	folder: Uri
): Promise<IFolderTaskItem[]> {
	const folderTasks: IFolderTaskItem[] = [];

	if (excludeRegex.test(Utils.basename(folder))) {
		return folderTasks;
	}
	const relativePattern = new RelativePattern(folder.fsPath, '**/package.json');
	const paths = await workspace.findFiles(relativePattern, '**/node_modules/**');

	const visitedPackageJsonFiles: Set<string> = new Set();
	for (const path of paths) {
		if (!visitedPackageJsonFiles.has(path.fsPath)) {
			const tasks = await provideNpmScriptsForFolder(context, path, true);
			visitedPackageJsonFiles.add(path.fsPath);
			folderTasks.push(...tasks.map(t => ({ label: t.task.name, task: t.task })));
		}
	}
	return folderTasks;
}

// Exclusion pattern matching
function isExcluded(folder: WorkspaceFolder, packageJsonUri: Uri) {
	function testForExclusionPattern(path: string, pattern: string): boolean {
		return minimatch(path, pattern, { dot: true });
	}

	const exclude = workspace.getConfiguration('npm', folder.uri).get<string | string[]>('npm.exclude');
	const packageJsonFolder = path.dirname(packageJsonUri.fsPath);

	if (exclude) {
		if (Array.isArray(exclude)) {
			for (const pattern of exclude) {
				if (testForExclusionPattern(packageJsonFolder, pattern)) {
					return true;
				}
			}
		} else if (testForExclusionPattern(packageJsonFolder, exclude)) {
			return true;
		}
	}
	return false;
}
```

**Variations / call-sites:**
- `findNpmPackages()` uses async generator for streaming results (line 185)
- `detectNpmScriptsForFolder()` is singular-folder variant (line 208)
- Both use deduplication via `Set<string>` to prevent duplicates
- `isExcluded()` supports both string and string array config patterns
- Exclusion excludes `node_modules/**` and `.vscode-test/**` by default

**Rust/Tauri equivalent challenges:**
- `workspace.findFiles()` is VS Code's glob search—needs custom glob walker in Rust (`walkdir`, `globwalk` crates)
- Relative patterns concept is VS Code-specific; requires path-based filtering
- Configuration reading needs Rust settings/config abstraction
- Deduplication using `HashSet<String>` or similar

---

### Pattern 4: Script Definition & Parsing from package.json

**Where:** `extensions/npm/src/readScripts.ts:21-73`

**What:** Parses package.json using JSONC parser, extracts scripts object, and returns script name-value pairs with line/column ranges for hover/navigation.

```typescript
export const readScripts = (
	document: TextDocument,
	buffer = document.getText()
): INpmScriptInfo | undefined => {
	let start: Position | undefined;
	let end: Position | undefined;
	let inScripts = false;
	let buildingScript: { name: string; nameRange: Range } | void;
	let level = 0;

	const scripts: INpmScriptReference[] = [];
	const visitor: JSONVisitor = {
		onError() {
			// no-op
		},
		onObjectBegin() {
			level++;
		},
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
					valueRange: new Range(
						document.positionAt(offset),
						document.positionAt(offset + length)
					),
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
					nameRange: new Range(
						document.positionAt(offset),
						document.positionAt(offset + length)
					)
				};
			}
		},
	};

	visit(buffer, visitor);

	if (start === undefined) {
		return undefined;
	}

	return {
		location: new Location(document.uri, new Range(start, end ?? start)),
		scripts
	};
};

export interface INpmScriptReference {
	name: string;
	value: string;
	nameRange: Range;
	valueRange: Range;
}

export interface INpmScriptInfo {
	location: Location;
	scripts: INpmScriptReference[];
}
```

**Variations / call-sites:**
- Called from `tasks.ts:279` to populate task definitions
- Called from `scriptHover.ts:60` for hover providers
- Called from `npmView.ts:164` for tree item positioning
- Caching: hover provider caches results (lines 20-31 in scriptHover.ts)

**Rust/Tauri equivalent challenges:**
- JSON parsing: use `serde_json` with custom visitor pattern or AST walker
- Position tracking: need byte offset → line/column mapping (similar to VS Code's Rope)
- JSONC support: requires comment-aware parser (use `jsonc-parser` via FFI or port logic)
- Position/Range abstractions: define custom structs matching VS Code semantics

---

### Pattern 5: Task Execution via IPC with Caching & Invalidation

**Where:** `extensions/npm/src/tasks.ts:229-239`, `extensions/npm/src/npmMain.ts:18-24`, `extensions/npm/src/tasks.ts:89-91`

**What:** Provides cached task list with invalidation on file/config changes; executes tasks via `tasks.executeTask()` IPC.

```typescript
// Task caching with invalidation
let cachedTasks: ITaskWithLocation[] | undefined = undefined;

export async function provideNpmScripts(
	context: ExtensionContext,
	showWarning: boolean
): Promise<ITaskWithLocation[]> {
	if (!cachedTasks) {
		const allTasks: ITaskWithLocation[] = [];
		for await (const path of findNpmPackages()) {
			const tasks = await provideNpmScriptsForFolder(context, path, showWarning);
			allTasks.push(...tasks);
		}
		cachedTasks = allTasks;
	}
	return cachedTasks;
}

export function invalidateTasksCache() {
	cachedTasks = undefined;
}

// Invalidation triggers (npmMain.ts)
function invalidateScriptCaches() {
	invalidateHoverScriptsCache();
	invalidateTasksCache();
	if (treeDataProvider) {
		treeDataProvider.refresh();
	}
}

// Registration of cache invalidation hooks
registerTaskProvider(context);  // Creates watcher
context.subscriptions.push(
	vscode.workspace.onDidChangeConfiguration((e) => {
		if (e.affectsConfiguration('npm.exclude')
			|| e.affectsConfiguration('npm.autoDetect')
			|| e.affectsConfiguration('npm.scriptExplorerExclude')
			|| e.affectsConfiguration('npm.runSilent')
			|| e.affectsConfiguration('npm.packageManager')
			|| e.affectsConfiguration('npm.scriptRunner')) {
			invalidateTasksCache();
			if (treeDataProvider) {
				treeDataProvider.refresh();
			}
		}
	})
);

// Task execution (npmView.ts:156)
private async runScript(script: NpmScript) {
	await detectPackageManager(script.getFolder().uri, this.context, true);
	tasks.executeTask(script.task);
}

// Task execution (commands.ts:61)
vscode.tasks.executeTask(result.task);
```

**Variations / call-sites:**
- File watcher invalidates: `npmMain.ts:126-130`
- Config watcher invalidates: `npmMain.ts:40-52`
- Multiple execution paths: tree view, quick pick, hover, script lens
- All converge on `tasks.executeTask()` IPC

**Rust/Tauri equivalent challenges:**
- Cache invalidation: requires event system (Tauri's invoke + listener pattern)
- File watching: use `notify` crate with event filtering
- Config changes: custom config watcher (e.g., file system polling or file hash)
- Task execution: IPC to main Tauri process or subprocess module

---

### Pattern 6: Package Manager Detection & Selection

**Where:** `extensions/npm/src/preferred-pm.ts:26-113`, `extensions/npm/src/tasks.ts:130-166`

**What:** Detects available package managers (npm, yarn, pnpm, bun) by lockfile presence; falls back to `which-pm` to detect currently-installed PM; supports configuration overrides.

```typescript
export async function findPreferredPM(
	pkgPath: string
): Promise<{ name: string; multipleLockFilesDetected: boolean }> {
	const detectedPackageManagerNames: string[] = [];
	const detectedPackageManagerProperties: PreferredProperties[] = [];

	// Check for lockfiles in order of preference
	const npmPreferred = await isNPMPreferred(pkgPath);
	if (npmPreferred.isPreferred) {
		detectedPackageManagerNames.push('npm');
		detectedPackageManagerProperties.push(npmPreferred);
	}

	const pnpmPreferred = await isPNPMPreferred(pkgPath);
	if (pnpmPreferred.isPreferred) {
		detectedPackageManagerNames.push('pnpm');
		detectedPackageManagerProperties.push(pnpmPreferred);
	}

	const yarnPreferred = await isYarnPreferred(pkgPath);
	if (yarnPreferred.isPreferred) {
		detectedPackageManagerNames.push('yarn');
		detectedPackageManagerProperties.push(yarnPreferred);
	}

	const bunPreferred = await isBunPreferred(pkgPath);
	if (bunPreferred.isPreferred) {
		detectedPackageManagerNames.push('bun');
		detectedPackageManagerProperties.push(bunPreferred);
	}

	// Fallback: ask which PM is installed
	const pmUsedForInstallation: { name: string } | null = await whichPM(pkgPath);

	if (pmUsedForInstallation && !detectedPackageManagerNames.includes(pmUsedForInstallation.name)) {
		detectedPackageManagerNames.push(pmUsedForInstallation.name);
		detectedPackageManagerProperties.push({ isPreferred: true, hasLockfile: false });
	}

	let lockfilesCount = 0;
	detectedPackageManagerProperties.forEach(detected =>
		lockfilesCount += detected.hasLockfile ? 1 : 0
	);

	return {
		name: detectedPackageManagerNames[0] || 'npm',
		multipleLockFilesDetected: lockfilesCount > 1
	};
}

// Lockfile detection helpers
async function isNPMPreferred(pkgPath: string): Promise<PreferredProperties> {
	const lockfileExists = await pathExists(path.join(pkgPath, 'package-lock.json'));
	return { isPreferred: lockfileExists, hasLockfile: lockfileExists };
}

async function isYarnPreferred(pkgPath: string): Promise<PreferredProperties> {
	if (await pathExists(path.join(pkgPath, 'yarn.lock'))) {
		return { isPreferred: true, hasLockfile: true };
	}
	try {
		if (typeof findWorkspaceRoot(pkgPath) === 'string') {
			return { isPreferred: true, hasLockfile: false };
		}
	} catch (err) { }
	return { isPreferred: false, hasLockfile: false };
}

// Configuration-based selection
export async function getScriptRunner(
	folder: Uri,
	context?: ExtensionContext,
	showWarning?: boolean
): Promise<string> {
	let scriptRunner = workspace.getConfiguration('npm', folder)
		.get<string>('scriptRunner', 'npm');

	if (scriptRunner === 'auto') {
		scriptRunner = await detectPackageManager(folder, context, showWarning);
	}

	return scriptRunner;
}

export async function detectPackageManager(
	folder: Uri,
	extensionContext?: ExtensionContext,
	showWarning: boolean = false
): Promise<string> {
	const { name, multipleLockFilesDetected } = await findPreferredPM(folder.fsPath);
	// Warning dialog if multiple lockfiles detected...
	return name;
}
```

**Variations / call-sites:**
- `getScriptRunner()` used in `getRunScriptCommand()` (line 320)
- `getPackageManager()` used for install commands (line 140)
- Both support 'auto' mode delegating to `detectPackageManager()`
- Configuration read via `workspace.getConfiguration('npm', folder.uri)`

**Rust/Tauri equivalent challenges:**
- Lockfile detection: `std::fs::metadata()` to check file existence
- `which-pm` npm package needs Rust port or subprocess invocation
- Config reading: Tauri's settings plugin or custom JSON config
- Async file I/O: tokio or Tauri's built-in async runtime

---

### Pattern 7: Multi-Level Task Definition Metadata

**Where:** `extensions/npm/src/tasks.ts:20-44`, `extensions/npm/package.json:352-370`

**What:** Defines a custom TaskDefinition interface (INpmTaskDefinition) and registers it in package.json's taskDefinitions contribution; enables task discovery and resolution.

```typescript
// TypeScript interface (tasks.ts)
export interface INpmTaskDefinition extends TaskDefinition {
	script: string;
	path?: string;
}

export interface ITaskWithLocation {
	task: Task;
	location?: Location;
}

// package.json contribution
"taskDefinitions": [
	{
		"type": "npm",
		"required": ["script"],
		"properties": {
			"script": {
				"type": "string",
				"description": "%taskdef.script%"
			},
			"path": {
				"type": "string",
				"description": "%taskdef.path%"
			}
		},
		"when": "shellExecutionSupported"
	}
]
```

**Variations / call-sites:**
- Task kind set in `createScriptRunnerTask()`: `{ type: 'npm', script }`
- Task kind set in `createInstallationTask()`: `{ type: 'npm', script: INSTALL_SCRIPT }`
- `resolveTask()` casts `_task.definition` to `INpmTaskDefinition` (line 61)
- Path field stores relative path for monorepo support (line 69-70)

**Rust/Tauri equivalent challenges:**
- Custom traits and structs replace TaskDefinition (no interface-based polymorphism in Rust)
- Package.json metadata needs separate schema definition (JSON schema or similar)
- Serialization/deserialization: serde with custom derive macros
- Type safety: strong typing at compile time (advantage over TypeScript)

---

## Summary: Patterns for Rust/Tauri Port

### Key Architectural Patterns Identified:

1. **Async Task Provider Pattern**: Lazy-initialized provider with cache invalidation on file/config changes (requires event bus in Rust)
2. **ShellExecution Abstraction**: Command building, quoting, and subprocess spawning (maps to `std::process::Command` + tokio)
3. **Workspace File Discovery**: Glob-based package.json location with deduplication (requires `walkdir`/`globwalk` crates)
4. **JSON AST Parsing**: Byte-offset-aware script extraction from package.json (requires JSONC library or custom parser)
5. **Caching & Invalidation**: Multi-layer cache (tasks, hover, tree view) invalidated by file watcher + config subscription
6. **Package Manager Detection**: Heuristic-based detection (lockfile → `which` → config) with fallback chain
7. **Task Definition Metadata**: Custom structs extending a base task definition with string script field and optional path

### Critical Implementation Challenges for Rust/Tauri:

- **No built-in task registry**: VS Code's `registerTaskProvider()` is IPC-based; Rust port needs custom async channel or event system
- **Shell execution context**: Tauri's `tauri::process` is limited; may need `tokio::process::Command` or Tauri plugin development
- **File system APIs**: VS Code's `workspace.findFiles()` is a powerful glob engine; Rust needs third-party glob library
- **Configuration management**: No workspace-scoped config in Tauri; requires custom settings abstraction
- **Position/Range tracking**: VS Code's document model with byte offsets and line/column mapping requires custom implementation

### Files Analyzed:
- `/Users/norinlavaee/vscode-atomic/extensions/npm/src/tasks.ts` — Core task logic (494 LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/npm/src/npmMain.ts` — Extension entry point (174 LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/npm/src/readScripts.ts` — JSON parsing (74 LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/npm/src/preferred-pm.ts` — Package manager detection (114 LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/npm/src/npmView.ts` — Tree UI (partial, 200+ LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/npm/src/scriptHover.ts` — Hover provider (131 LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/npm/package.json` — Extension manifest (390 LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/npm/src/commands.ts` — Command routing (68 LOC)

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
