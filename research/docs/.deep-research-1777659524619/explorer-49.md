# Partition 49 of 79 — Findings

## Scope
`extensions/jake/` (2 files, 357 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator 49: extensions/jake/

## Implementation

- `extensions/jake/src/main.ts` - Jake task provider implementation (340 LOC). Core extension logic implementing `vscode.tasks.registerTaskProvider('jake', ...)` at line 270. Includes:
  - `TaskDetector` class managing task provider lifecycle
  - `FolderDetector` class for per-workspace-folder Jake task detection
  - `JakeTaskDefinition` interface extending `vscode.TaskDefinition`
  - Helper functions: `findJakeCommand()`, `isBuildTask()`, `isTestTask()`, `exists()`, `exec()`
  - Jake executable detection across Windows/Linux/Darwin platforms
  - File system watching for Jakefile changes

- `extensions/jake/esbuild.mts` - Build configuration file

## Configuration

- `extensions/jake/package.json` - Extension manifest (76 lines). Declares:
  - Task provider activation on `onTaskType:jake`
  - Task definition type `jake` with required `task` property and optional `file` property
  - Configuration contribution: `jake.autoDetect` (scope: application, default: off)
  - Capability declarations: untrusted workspace support enabled, virtual workspace disabled

- `extensions/jake/tsconfig.json` - TypeScript configuration extending `../tsconfig.base.json`

- `extensions/jake/.npmrc` - NPM configuration
- `extensions/jake/.vscodeignore` - Bundling exclusion patterns

## Documentation

- `extensions/jake/README.md` - User-facing documentation describing Jake task support, feature list, and configuration settings

## Types / Interfaces

- `JakeTaskDefinition` interface in `src/main.ts` (lines 80-83) extending `vscode.TaskDefinition` with properties:
  - `task: string` (required)
  - `file?: string` (optional)

## Notable Clusters

- **Workspace Folder Management**: Lines 229-265 in `src/main.ts` handle dynamic addition/removal of workspace folders and configuration change reactions
- **Task Computation Pipeline**: Lines 133-194 orchestrate Jake command execution, stdout parsing with regex matching, and task metadata assignment (build/test groups)
- **Provider Lifecycle**: Lines 267-283 conditionally register/dispose task provider based on detector population

## Summary

The Jake extension is a focused task provider for VS Code that auto-detects Jake build tasks from `Jakefile` and `Jakefile.js` files. Single TypeScript source file implements a two-tier detection architecture (workspace-level and folder-level) with file system watching, configuration support, and platform-aware Jake executable resolution. The extension registers dynamically using `vscode.tasks.registerTaskProvider('jake', ...)` to provide discovered Jake tasks to VS Code's task system.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/jake/src/main.ts` — 340 lines; full Jake TaskProvider implementation
- `/home/norinlavaee/projects/vscode-atomic/extensions/jake/esbuild.mts` — 18 lines; build configuration for the extension

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/extensions/jake/src/main.ts`

**Role:** Implements the Jake task auto-detection extension for VS Code. Exports `activate` and `deactivate` lifecycle hooks. Registers a `vscode.TaskProvider` for the `'jake'` task type.

**Key Symbols (file:line)**

| Symbol | Line | Purpose |
|--------|------|---------|
| `AutoDetect` type alias | 11 | Union `'on' \| 'off'`; maps to `jake.autoDetect` config value |
| `exists(file)` | 13–19 | Promisified `fs.exists`; used to probe for Jakefile and local jake binary |
| `exec(command, options)` | 21–30 | Promisified `cp.exec`; wraps child-process execution; rejects with `{ error, stdout, stderr }` on non-zero exit |
| `buildNames` / `isBuildTask(name)` | 32–40 | String-inclusion check against `['build','compile','watch']`; classifies a task into `TaskGroup.Build` |
| `testNames` / `isTestTask(name)` | 42–50 | String-inclusion check against `['test']`; classifies a task into `TaskGroup.Test` |
| `getOutputChannel()` | 53–58 | Lazy-initializes a singleton `vscode.OutputChannel` named `'Jake Auto Detection'` |
| `showError()` | 60–65 | Shows a warning toast with a "Go to output" action button |
| `findJakeCommand(rootPath)` | 67–78 | Resolves the jake binary path; platform-aware resolution (win32 uses `jake.cmd`, linux/darwin uses `jake`); falls back to bare `'jake'` if no local binary found |
| `JakeTaskDefinition` interface | 80–83 | Extends `vscode.TaskDefinition`; fields: `task: string`, `file?: string` |
| `FolderDetector` class | 85–202 | Per-workspace-folder detection unit |
| `TaskDetector` class | 204–329 | Orchestrates multiple `FolderDetector` instances; registers/unregisters the provider |
| `activate(_context)` | 332–335 | Extension entry point; creates and starts a `TaskDetector` |
| `deactivate()` | 337–339 | Calls `detector.dispose()` |

---

**`FolderDetector` — Control and Data Flow (lines 85–202)**

Constructor (line 90–93): Accepts a `vscode.WorkspaceFolder` and a `Promise<string>` for the resolved jake command. The command promise is computed once by the caller (`findJakeCommand`) and passed in.

`isEnabled()` (line 99–101): Reads `jake.autoDetect` from workspace configuration scoped to the folder URI. Returns `true` only when the value is `'on'`.

`start()` (line 103–109): Creates a `vscode.FileSystemWatcher` matching the glob `{node_modules,Jakefile,Jakefile.js}` inside the folder. Each of `onDidChange`, `onDidCreate`, `onDidDelete` sets `this.promise = undefined`, invalidating the task cache on any relevant filesystem change.

`getTasks()` (line 111–120): Guards on `isEnabled()`. Returns the cached `this.promise` if it exists; otherwise calls `computeTasks()` and stores the result in `this.promise` (memoization). Returns `[]` when detection is disabled.

`getTask(_task)` (line 122–131): Resolves a single task by reading `_task.definition.task` (the task name), then constructing a new `vscode.Task` using `vscode.ShellExecution` with the resolved jake command and task name as arguments. Uses `this.workspaceFolder.uri.fsPath` as `cwd`.

`computeTasks()` (line 133–194) — **Core parsing pipeline**:

1. **Root path guard** (line 134): Only proceeds for `file://` scheme folders.
2. **Jakefile detection** (line 139–145): Checks `Jakefile` first, then `Jakefile.js`; returns `[]` if neither exists.
3. **Command execution** (line 147–149): Runs `<jakeCommand> --tasks` via `exec()` with `{ cwd: rootPath }`.
4. **Stderr handling** (line 150–153): If `stderr` is non-empty, appends to output channel and calls `showError()`.
5. **Output parsing** (line 155–179): Splits `stdout` on `\r?\n`. For each non-empty line, applies regex `/^jake\s+([^\s]+)\s/g` (line 161) to capture the task name in capture group 1. Validates `matches.length === 2` (line 163).
6. **Task construction** (line 165–171): Creates `JakeTaskDefinition` with `type: 'jake'` and the captured task name. Constructs a `vscode.Task` with a `vscode.ShellExecution` of `<jakeCommand> <taskName>`, using `ShellExecutionOptions` with `cwd`.
7. **Group classification** (line 172–177): Lowercases the full output line, then tests via `isBuildTask` / `isTestTask`; assigns `task.group` to `vscode.TaskGroup.Build` or `vscode.TaskGroup.Test`.
8. **Error handling** (line 182–193): Catches exec rejection; appends `err.stderr`, `err.stdout`, and a localized message to the output channel; calls `showError()`; returns `[]`.

`dispose()` (line 196–201): Clears `this.promise` and disposes the file watcher.

---

**`TaskDetector` — Control and Data Flow (lines 204–329)**

State: `taskProvider: vscode.Disposable | undefined` (the registered provider token) and `detectors: Map<string, FolderDetector>` keyed by folder URI string.

`start()` (line 212–219): Calls `updateWorkspaceFolders` with all current folders and an empty removed set. Subscribes to `vscode.workspace.onDidChangeWorkspaceFolders` and `vscode.workspace.onDidChangeConfiguration`.

`updateWorkspaceFolders(added, removed)` (line 229–245):
- For each removed folder: disposes the corresponding `FolderDetector` and deletes it from the map.
- For each added folder: creates a new `FolderDetector(add, findJakeCommand(add.uri.fsPath))`, stores it keyed by `add.uri.toString()`, and calls `detector.start()` only if `detector.isEnabled()`.
- Calls `updateProvider()` at the end.

`updateConfiguration()` (line 247–265): Disposes and deletes all existing detectors, then recreates them from `vscode.workspace.workspaceFolders`, re-enabling only those with `autoDetect === 'on'`. Calls `updateProvider()`.

`updateProvider()` (line 267–283): Registers a new `vscode.tasks.registerTaskProvider('jake', ...)` token if detectors are present and no provider is yet registered (line 268–278). Disposes and nullifies the provider if detectors become empty (line 279–282). The provider object literal has `provideTasks()` delegating to `thisCapture.getTasks()` and `resolveTask(_task)` delegating to `thisCapture.getTask(_task)`.

`computeTasks()` (line 289–309): Dispatches to a single detector if `size === 1` (line 293) or fans out with `Promise.all` for multiple detectors (line 295–308), flattening results.

`getTask(task)` (line 311–328): For a single detector, delegates directly. For multiple detectors, routes by `task.scope.uri.toString()` to the matching `FolderDetector`. Global/Workspace scopes return `undefined` (line 318–319).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/jake/esbuild.mts`

**Role:** Build script that invokes the shared esbuild helper from the monorepo to bundle `src/main.ts` into `dist/main.js`.

**Key Symbols (file:line)**

| Symbol | Line | Purpose |
|--------|------|---------|
| `import { run }` | 6 | Imports the `run` function from `../esbuild-extension-common.mts` (shared across extensions) |
| `srcDir` | 8 | Resolved to `<extension-root>/src` via `import.meta.dirname` |
| `outDir` | 9 | Resolved to `<extension-root>/dist` |
| `run(config, process.argv)` | 11–18 | Invoked with `platform: 'node'`, single entry point `main` mapped to `src/main.ts`, and output directory `dist` |

**Control Flow:** The script is a direct top-level invocation. It computes `srcDir` and `outDir` using Node ESM's `import.meta.dirname` (line 8–9), then calls `run(...)` passing `process.argv` so the shared helper can handle CLI flags (e.g., `--watch`). No conditional logic exists in this file; all bundling behavior is delegated to `esbuild-extension-common.mts`.

---

### Cross-Cutting Synthesis

The Jake extension implements a two-layer TaskProvider architecture. `TaskDetector` (lines 204–329 of `main.ts`) acts as the workspace-level coordinator: it manages a `Map<string, FolderDetector>` keyed by folder URI and lazily registers or unregisters a single `vscode.TaskProvider` token via `updateProvider()`. Each `FolderDetector` (lines 85–202) is scoped to one workspace folder and memoizes its task list in `this.promise`, with cache invalidation driven by a `FileSystemWatcher` monitoring `Jakefile`, `Jakefile.js`, and `node_modules`. The actual task discovery pipeline runs `jake --tasks` as a child process (via the promisified `exec` helper at line 21), parses its stdout line-by-line with the regex `/^jake\s+([^\s]+)\s/g` (line 161), and classifies each discovered task into `Build` or `Test` groups by substring-matching the line against hard-coded name lists. Platform-aware binary resolution happens in `findJakeCommand` (lines 67–78): it checks for local `node_modules/.bin/jake[.cmd]` before falling back to a global `jake` invocation. The build file (`esbuild.mts`) delegates all bundling to the shared `esbuild-extension-common.mts` helper with a `node` platform target and a single `main` entry point, producing output in `dist/`.

---

### Out-of-Partition References

- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-extension-common.mts` — the `run` function imported at `esbuild.mts:6`; handles esbuild invocation, CLI flag parsing (e.g., `--watch`), and output file naming conventions shared across all extensions.
- `vscode` API module — `vscode.tasks.registerTaskProvider`, `vscode.Task`, `vscode.ShellExecution`, `vscode.FileSystemWatcher`, `vscode.TaskGroup`, `vscode.workspace.getConfiguration`, `vscode.l10n.t` are all consumed but implemented by the VS Code extension host runtime.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Analysis: `extensions/jake/` (Partition 49 of 79)

## Scope
- `extensions/jake/src/main.ts` (340 LOC)
- `extensions/jake/package.json` (76 LOC)

## Seed Query
```
ast-grep --lang typescript -p 'vscode.tasks.registerTaskProvider($$$)'
```

---

## Patterns Found

#### Pattern: Task Provider Registration with Conditional Lifecycle

**Where:** `extensions/jake/src/main.ts:270-282`

**What:** Registers a task provider with conditional creation/disposal based on detector count. The provider is only registered when detectors are available and disposed when the last detector is removed.

```typescript
private updateProvider(): void {
	if (!this.taskProvider && this.detectors.size > 0) {
		const thisCapture = this;
		this.taskProvider = vscode.tasks.registerTaskProvider('jake', {
			provideTasks(): Promise<vscode.Task[]> {
				return thisCapture.getTasks();
			},
			resolveTask(_task: vscode.Task): Promise<vscode.Task | undefined> {
				return thisCapture.getTask(_task);
			}
		});
	}
	else if (this.taskProvider && this.detectors.size === 0) {
		this.taskProvider.dispose();
		this.taskProvider = undefined;
	}
}
```

**Variations:** None in this scope; this is the sole task provider registration.

---

#### Pattern: Task Definition with Type and Task Properties

**Where:** `extensions/jake/src/main.ts:80-83`

**What:** Defines a structured task definition interface extending vscode.TaskDefinition with required 'task' property and optional 'file' property. Matches the package.json task definition schema.

```typescript
interface JakeTaskDefinition extends vscode.TaskDefinition {
	task: string;
	file?: string;
}
```

**Variations:** Used in two locations:
- `extensions/jake/src/main.ts:125` (casting with `as JakeTaskDefinition`)
- `extensions/jake/src/main.ts:165-168` (inline object literal)

---

#### Pattern: Task Factory with Shell Execution

**Where:** `extensions/jake/src/main.ts:122-131`

**What:** Creates a Task instance with shell execution options containing workspace-relative cwd. Used in the getTask method to resolve individual tasks from definitions.

```typescript
public async getTask(_task: vscode.Task): Promise<vscode.Task | undefined> {
	const jakeTask = _task.definition.task;
	if (jakeTask) {
		const kind = _task.definition as JakeTaskDefinition;
		const options: vscode.ShellExecutionOptions = { cwd: this.workspaceFolder.uri.fsPath };
		const task = new vscode.Task(kind, this.workspaceFolder, jakeTask, 'jake', new vscode.ShellExecution(await this._jakeCommand, [jakeTask], options));
		return task;
	}
	return undefined;
}
```

**Variations:** Task creation also in `computeTasks()` method (lines 170-171) with similar structure but inline command concatenation.

---

#### Pattern: Multi-Detector Task Aggregation

**Where:** `extensions/jake/src/main.ts:289-309`

**What:** Aggregates tasks from multiple workspace folder detectors using Promise.all, handling empty results and size-based optimization for single detector case.

```typescript
private computeTasks(): Promise<vscode.Task[]> {
	if (this.detectors.size === 0) {
		return Promise.resolve([]);
	} else if (this.detectors.size === 1) {
		return this.detectors.values().next().value!.getTasks();
	} else {
		const promises: Promise<vscode.Task[]>[] = [];
		for (const detector of this.detectors.values()) {
			promises.push(detector.getTasks().then((value) => value, () => []));
		}
		return Promise.all(promises).then((values) => {
			const result: vscode.Task[] = [];
			for (const tasks of values) {
				if (tasks && tasks.length > 0) {
					result.push(...tasks);
				}
			}
			return result;
		});
	}
}
```

**Variations:** Mirror pattern in `getTask()` method (lines 311-328) for resolving individual tasks across detectors.

---

#### Pattern: Workspace Folder Change Listener with Detector Lifecycle

**Where:** `extensions/jake/src/main.ts:229-244`

**What:** Manages detector creation/disposal in response to workspace folder changes, with per-folder enabled state checking. Detectors are mapped by workspace folder URI.

```typescript
private updateWorkspaceFolders(added: readonly vscode.WorkspaceFolder[], removed: readonly vscode.WorkspaceFolder[]): void {
	for (const remove of removed) {
		const detector = this.detectors.get(remove.uri.toString());
		if (detector) {
			detector.dispose();
			this.detectors.delete(remove.uri.toString());
		}
	}
	for (const add of added) {
		const detector = new FolderDetector(add, findJakeCommand(add.uri.fsPath));
		this.detectors.set(add.uri.toString(), detector);
		if (detector.isEnabled()) {
			detector.start();
		}
	}
	this.updateProvider();
}
```

**Variations:** Similar logic in `updateConfiguration()` (lines 247-264) that rebuilds all detectors when configuration changes.

---

#### Pattern: File System Watcher with Invalidation

**Where:** `extensions/jake/src/main.ts:103-109`

**What:** Creates file system watcher on Jakefile/node_modules pattern and invalidates cached promise on any file changes (create, delete, modify).

```typescript
public start(): void {
	const pattern = path.join(this._workspaceFolder.uri.fsPath, '{node_modules,Jakefile,Jakefile.js}');
	this.fileWatcher = vscode.workspace.createFileSystemWatcher(pattern);
	this.fileWatcher.onDidChange(() => this.promise = undefined);
	this.fileWatcher.onDidCreate(() => this.promise = undefined);
	this.fileWatcher.onDidDelete(() => this.promise = undefined);
}
```

**Variations:** None in this scope.

---

#### Pattern: Build and Test Task Classification

**Where:** `extensions/jake/src/main.ts:32-50`

**What:** Task classification based on name matching against predefined lists (build/compile/watch, test). Used to assign TaskGroup.Build or TaskGroup.Test.

```typescript
const buildNames: string[] = ['build', 'compile', 'watch'];
function isBuildTask(name: string): boolean {
	for (const buildName of buildNames) {
		if (name.indexOf(buildName) !== -1) {
			return true;
		}
	}
	return false;
}

const testNames: string[] = ['test'];
function isTestTask(name: string): boolean {
	for (const testName of testNames) {
		if (name.indexOf(testName) !== -1) {
			return true;
		}
	}
	return false;
}
```

**Variations:** Applied at lines 173-177 in the task creation loop to assign task groups.

---

## Summary

The `extensions/jake/` extension demonstrates six distinct patterns for task provider integration with VS Code:

1. **Conditional task provider registration** - Provider lifecycle tied to detector availability
2. **Task definition schema** - Interface-based definition with required and optional properties
3. **Task factory pattern** - Consistent shell execution wrapper for task creation
4. **Multi-detector aggregation** - Promise-based collection of tasks from multiple workspace folders
5. **Workspace folder lifecycle management** - Detector creation/disposal synchronized with workspace changes
6. **File system watching** - Glob-pattern file monitoring with cache invalidation
7. **Task classification** - String-based task grouping (Build/Test) via name matching

The code follows a hierarchical detector pattern with TaskDetector managing multiple FolderDetector instances, each responsible for one workspace folder. The registerTaskProvider call at line 270 is guarded by a size check to avoid registration without valid detectors.

### Port Considerations

For Tauri/Rust porting, key aspects include:
- Task provider registration via extension API (likely similar in Rust bindings)
- Promise-based async resolution pattern (maps to Rust futures)
- File system watching integration (Tauri file watcher support)
- Workspace folder lifecycle events (Tauri extension context)
- Shell execution with per-folder working directories (process management)

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
