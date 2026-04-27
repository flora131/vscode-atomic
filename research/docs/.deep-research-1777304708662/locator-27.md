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
