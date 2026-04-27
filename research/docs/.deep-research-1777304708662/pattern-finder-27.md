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
