# Partition 27 of 79 — Findings

## Scope
`extensions/npm/` (14 files, 2,372 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Task/Terminal Execution Patterns for VS Code to Tauri/Rust Migration

Research on npm extension's task execution, terminal handling, script discovery, and package manager detection. These patterns represent the reference implementations for porting VS Code's IDE functionality to Tauri/Rust.

## Patterns Found

#### Pattern: TaskProvider Registration with Auto-Discovery
**Where:** `extensions/npm/src/npmMain.ts:124-141`
**What:** Registers an npm task provider that auto-discovers and caches npm scripts from workspace package.json files.
```typescript
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
**Variations / call-sites:** Registered in `npmMain.ts:36` during extension activation; watcher callbacks invalidate caches which trigger tree data provider refresh (`npmView.ts:22`).

---

#### Pattern: TaskProvider Implementation with provideTasks and resolveTask
**Where:** `extensions/npm/src/tasks.ts:46-87`
**What:** Core TaskProvider implementation that provides available npm scripts and resolves task definitions into executable tasks with ShellExecution.
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
			let packageJsonUri: Uri;
			if (_task.scope === undefined || _task.scope === TaskScope.Global || _task.scope === TaskScope.Workspace) {
				return undefined;
			}
			if (kind.path) {
				packageJsonUri = _task.scope.uri.with({ path: _task.scope.uri.path + '/' + kind.path + `${kind.path.endsWith('/') ? '' : '/'}` + 'package.json' });
			} else {
				packageJsonUri = _task.scope.uri.with({ path: _task.scope.uri.path + '/package.json' });
			}
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
```
**Variations / call-sites:** Instantiated in `npmMain.ts:135`; used by tree data provider `npmView.ts:144-145` and by hover/code lens providers.

---

#### Pattern: ShellExecution with Argument Escaping and Working Directory
**Where:** `extensions/npm/src/tasks.ts:304-360`
**What:** Creates ShellExecution tasks with proper shell quoting and argument escaping for cross-platform compatibility; manages working directory context.
```typescript
function escapeCommandLine(cmd: string[]): (string | ShellQuotedString)[] {
	return cmd.map(arg => {
		if (/\s/.test(arg)) {
			return { value: arg, quoting: arg.includes('--') ? ShellQuoting.Weak : ShellQuoting.Strong };
		} else {
			return arg;
		}
	});
}

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
	}
	return task;
}
```
**Variations / call-sites:** Installation task variant at `tasks.ts:371-387`; used in hover provider `scriptHover.ts:15-17` and commands `commands.ts:25`.

---

#### Pattern: Package.json Script Discovery with Range Tracking
**Where:** `extensions/npm/src/readScripts.ts:21-73`
**What:** Uses jsonc-parser visitor pattern to extract scripts section from package.json, preserving source location (Range/Position) for editor integration.
```typescript
export const readScripts = (document: TextDocument, buffer = document.getText()): INpmScriptInfo | undefined => {
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
**Variations / call-sites:** Called by hover provider `scriptHover.ts:60`, code lens provider `npmScriptLens.ts:69`, tree builder `npmView.ts:164`, and script location finder `tasks.ts:462`.

---

#### Pattern: Workspace File Discovery with Exclusion Patterns
**Where:** `extensions/npm/src/tasks.ts:185-227`
**What:** Finds package.json files across workspace using RelativePattern with exclusion globs; deduplicates results; supports nested workspaces.
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

export async function detectNpmScriptsForFolder(context: ExtensionContext, folder: Uri): Promise<IFolderTaskItem[]> {

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
```
**Variations / call-sites:** Called by `provideNpmScripts` (`tasks.ts:229-239`); quick-pick integration in `commands.ts:38`.

---

#### Pattern: Package Manager Detection with Lockfile Analysis
**Where:** `extensions/npm/src/preferred-pm.ts:71-113`
**What:** Detects preferred package manager by checking for lockfiles (package-lock.json, yarn.lock, pnpm-lock.yaml, bun.lockb) and workspace root indicators; reports multiple lockfiles.
```typescript
export async function findPreferredPM(pkgPath: string): Promise<{ name: string; multipleLockFilesDetected: boolean }> {
	const detectedPackageManagerNames: string[] = [];
	const detectedPackageManagerProperties: PreferredProperties[] = [];

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

	const pmUsedForInstallation: { name: string } | null = await whichPM(pkgPath);

	if (pmUsedForInstallation && !detectedPackageManagerNames.includes(pmUsedForInstallation.name)) {
		detectedPackageManagerNames.push(pmUsedForInstallation.name);
		detectedPackageManagerProperties.push({ isPreferred: true, hasLockfile: false });
	}

	let lockfilesCount = 0;
	detectedPackageManagerProperties.forEach(detected => lockfilesCount += detected.hasLockfile ? 1 : 0);

	return {
		name: detectedPackageManagerNames[0] || 'npm',
		multipleLockFilesDetected: lockfilesCount > 1
	};
}
```
**Variations / call-sites:** Called by `getPackageManager` (`tasks.ts:140-148`), `detectPackageManager` (`tasks.ts:150-166`); used in hover provider `npmView.ts:155` to show warnings.

---

#### Pattern: Task Execution via Command with Task Definition
**Where:** `extensions/npm/src/tasks.ts:438-445` and `commands.ts:32-67`
**What:** Executes tasks via vscode.tasks.executeTask API; integrates with quick-pick UI for user selection.
```typescript
export async function runScript(context: ExtensionContext, script: string, document: TextDocument) {
	const uri = document.uri;
	const folder = workspace.getWorkspaceFolder(uri);
	if (folder) {
		const task = await createScriptRunnerTask(context, script, folder, uri);
		tasks.executeTask(task);
	}
}

export async function selectAndRunScriptFromFolder(context: vscode.ExtensionContext, selectedFolders: vscode.Uri[]) {
	if (selectedFolders.length === 0) {
		return;
	}
	const selectedFolder = selectedFolders[0];

	const taskList: IFolderTaskItem[] = await detectNpmScriptsForFolder(context, selectedFolder);

	if (taskList && taskList.length > 0) {
		const quickPick = vscode.window.createQuickPick<IFolderTaskItem>();
		quickPick.placeholder = 'Select an npm script to run in folder';
		quickPick.items = taskList;

		const toDispose: vscode.Disposable[] = [];

		const pickPromise = new Promise<IFolderTaskItem | undefined>((c) => {
			toDispose.push(quickPick.onDidAccept(() => {
				toDispose.forEach(d => d.dispose());
				c(quickPick.selectedItems[0]);
			}));
			toDispose.push(quickPick.onDidHide(() => {
				toDispose.forEach(d => d.dispose());
				c(undefined);
			}));
		});
		quickPick.show();
		const result = await pickPromise;
		quickPick.dispose();
		if (result) {
			vscode.tasks.executeTask(result.task);
		}
	}
	else {
		vscode.window.showInformationMessage(`No npm scripts found in ${selectedFolder.fsPath}`, { modal: true });
	}
}
```
**Variations / call-sites:** Used in tree provider `npmView.ts:156` and `npmView.ts:186` for script/install execution; hover provider `scriptHover.ts:118` for hover-triggered execution.

---

#### Pattern: Debug Script Integration with JS Debugger
**Where:** `extensions/npm/src/tasks.ts:447-456`
**What:** Invokes the JavaScript debugger extension to create a debug terminal with npm script execution context.
```typescript
export async function startDebugging(context: ExtensionContext, scriptName: string, cwd: string, folder: WorkspaceFolder) {
	const runScriptCommand = await getRunScriptCommand(scriptName, folder.uri, context, true);

	commands.executeCommand(
		'extension.js-debug.createDebuggerTerminal',
		runScriptCommand.join(' '),
		folder,
		{ cwd },
	);
}
```
**Variations / call-sites:** Called by tree provider `npmView.ts:160` and hover provider `scriptHover.ts:82-94` for debug script interaction.

---

## Synthesis

The npm extension demonstrates a complete task/terminal execution framework:

1. **Registration & Discovery**: TaskProvider registration with filesystem watchers to detect package.json changes; auto-caching with invalidation.

2. **Parsing**: jsonc-parser visitor pattern preserves source locations (Range/Position) for bidirectional editor integration (hover, code lens, tree view).

3. **Execution**: ShellExecution abstraction with platform-aware argument escaping (ShellQuoting.Weak vs Strong); working directory context propagation.

4. **Package Management**: Multi-package-manager detection (npm/yarn/pnpm/bun) via lockfile and workspace root heuristics; conflict reporting.

5. **UI Integration**: Tree data provider hierarchical organization (Workspace > Package > Script); quick-pick for folder-level task selection; hover/code-lens for inline execution; debug terminal integration.

For Tauri/Rust porting, key abstractions to replicate:
- **Async task provider interface**: Support provideTasks (discovery) and resolveTask (specialization).
- **Source location tracking**: Map script names to ranges for editor features.
- **Cross-platform shell execution**: Abstract platform-specific quoting (Windows cmd vs Unix sh).
- **Workspace file discovery**: Recursive glob with exclusions and deduplication.
- **Package manager heuristics**: Lockfile-based detection with multi-PM conflict warnings.
- **UI command bridge**: Task execution triggering from tree/hover/quick-pick contexts.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
