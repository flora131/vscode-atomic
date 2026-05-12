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
