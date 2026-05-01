# File Locations: npm Extension - TaskProvider & Language Intelligence

## Implementation

### Core Task Provider (TaskProvider Registration)
- `extensions/npm/src/tasks.ts` — TaskProvider implementation exposing `registerTaskProvider()` at npmMain.ts:136; implements `provideTasks()` and `resolveTask()` contract; defines `INpmTaskDefinition`, `ITaskWithLocation`, `NpmTaskProvider` class; task resolution logic at lines 60-87

### Main Activation & Registration
- `extensions/npm/src/npmMain.ts` — Extension activation entry point; registers TaskProvider via `vscode.tasks.registerTaskProvider('npm', taskProvider)` at line 136; configures file watchers for package.json; manages subscriptions for all providers and commands

### Language Intelligence Providers (CodeLens)
- `extensions/npm/src/npmScriptLens.ts` — CodeLensProvider for npm scripts; implements debug code lens above scripts in package.json; consumes `debug.javascript.codelens.npmScripts` config; registers via `languages.registerCodeLensProvider()` at line 51

### Language Intelligence Providers (Hover)
- `extensions/npm/src/scriptHover.ts` — HoverProvider for npm scripts; registers hover information when hovering over scripts; implements `HoverProvider` interface; command handlers for script execution from hover (npm.runScriptFromHover, npm.debugScriptFromHover); caching system for script metadata

### JSON Contributions (Completion & Hover)
- `extensions/npm/src/features/jsonContributions.ts` — Multi-provider infrastructure for JSON document language features; implements `IJSONContribution` interface; registers both CompletionItemProvider and HoverProvider for package.json via `languages.registerCompletionItemProvider()` and `languages.registerHoverProvider()`; classes JSONCompletionItemProvider and JSONHoverProvider

- `extensions/npm/src/features/packageJSONContribution.ts` — Concrete implementation of IJSONContribution for package.json; provides completion suggestions for dependencies, devDependencies, scripts; fetches package metadata from npmjs.org; online integration for autocompletion hints

### Script Parsing & Detection
- `extensions/npm/src/readScripts.ts` — Core script parser using jsonc-parser JSONVisitor pattern; extracts script location, name, and value ranges from package.json; types: `INpmScriptReference`, `INpmScriptInfo` (for hover/lens caching)

### Tree View / Explorer UI
- `extensions/npm/src/npmView.ts` — TreeDataProvider for npm Script Explorer view; hierarchical UI showing Folder → PackageJSON → NpmScript items; TreeItem implementations for folder/package/script rendering; integrates with taskProvider for task execution

### Command Registration
- `extensions/npm/src/commands.ts` — Command handlers for user-triggered script execution; `runSelectedScript()` finds script at cursor position in package.json editor; `selectAndRunScriptFromFolder()` detects and executes scripts from folder context menu; uses QuickPick for script selection

### Package Manager Detection
- `extensions/npm/src/preferred-pm.ts` — Multi-PM detection logic checking for npm, yarn, pnpm, bun; inspects lockfile presence (package-lock.json, yarn.lock, pnpm-lock.yaml, bun.lock/lockb); used by task runner to select appropriate PM for script execution

### Browser / Web Extension Entry
- `extensions/npm/src/npmBrowserMain.ts` — Lightweight web/browser entry point; activates only JSON contribution features (no task provider in browser context); minimal feature set for web-based VS Code environments

## Types / Interfaces

Defined in source files:
- `extensions/npm/src/tasks.ts` — Lines 20-44: `INpmTaskDefinition extends TaskDefinition`, `IFolderTaskItem extends QuickPickItem`, `ITaskLocation`, `ITaskWithLocation`
- `extensions/npm/src/readScripts.ts` — Lines 9-19: `INpmScriptReference`, `INpmScriptInfo`
- `extensions/npm/src/features/jsonContributions.ts` — Lines 15-29: `ISuggestionsCollector`, `IJSONContribution` (extensibility interface for document contributions)

All extension provider types (`TaskProvider`, `HoverProvider`, `CodeLensProvider`, `CompletionItemProvider`, `TreeDataProvider`) imported from vscode module.

## Configuration

- `extensions/npm/package.json` — Full extension manifest
  - **activationEvents** (lines 46-50): `onTaskType:npm`, `onLanguage:json`, `workspaceContains:package.json`
  - **contributes.taskDefinitions** (lines 352-369): Defines `npm` task type with `script` property
  - **contributes.languages** (lines 62-74): Registers `.npmignore` and `.npmrc` language IDs
  - **contributes.views** (lines 76-86): Explorer view `npm` for Script Explorer
  - **contributes.commands** (lines 88-123): npm.runScript, npm.debugScript, npm.openScript, npm.runInstall, npm.refresh, etc.
  - **contributes.menus** (lines 125-217): Context menus for command palette, editor, explorer, view items
  - **contributes.configuration** (lines 219-340): npm.autoDetect, npm.runSilent, npm.packageManager, npm.scriptRunner, npm.exclude, npm.enableRunFromFolder, npm.scriptHover, npm.scriptExplorerAction, npm.scriptExplorerExclude, npm.fetchOnlinePackageInfo
  - **contributes.jsonValidation** (lines 342-350): Schema validation for package.json and bower.json
  - **contributes.terminalQuickFixes** (lines 371-383): Quick fix provider for npm command errors in terminal

- `extensions/npm/tsconfig.json` — TypeScript configuration extending base; includes vscode.d.ts and proposed terminalQuickFixProvider API

- `extensions/npm/.vscode/launch.json` — Debug launch configuration for extension development
- `extensions/npm/.vscode/tasks.json` — Build/compile tasks

## Documentation

- `extensions/npm/README.md` — User-facing feature documentation covering Task Running, Script Explorer, Run from Editor, Run from Folder, dependency completion/hover features; Settings reference for all npm.* config properties

## Notable Clusters

- `extensions/npm/src/` (9 files) — Full extension implementation: core task provider, all language features (CodeLens/Hover), command handlers, script parsing, tree view
- `extensions/npm/src/features/` (3 files) — Pluggable JSON document intelligence: completion and hover for package.json with online package info fetching
- `extensions/npm/` (14 files total) — Complete npm extension: 9 source TS files, 1 browser entry point, build configs (esbuild.mts, esbuild.browser.mts), TypeScript config, package manifest, README, NLS strings, license

## Summary

The npm extension (partition 27) is a reference implementation of VS Code's key extension APIs for porting to Tauri/Rust. It demonstrates the **TaskProvider** API contract through full lifecycle management of npm script tasks (discovery, resolution, execution), backed by comprehensive **language intelligence** via CodeLensProvider and HoverProvider. The extension also exemplifies JSON document intelligence registration patterns (CompletionItemProvider, HoverProvider) and workbench integration through TreeDataProvider. Package.json parsing uses an AST visitor pattern over JSONC, not direct regex. Task execution delegates to ShellExecution, allowing external script runners (npm, yarn, pnpm, bun) to execute discovered scripts. The terminalQuickFixProvider API integration shows how extensions can contribute to terminal UX. All core functionality (task registration, language features, commands, menus) flows through the vscode module's public API surface, making this extension a complete reference for an IDE host that must maintain backward compatibility with existing TaskProvider and language intelligence extensions.
