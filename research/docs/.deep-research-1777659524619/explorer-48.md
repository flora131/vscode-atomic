# Partition 48 of 79 — Findings

## Scope
`extensions/grunt/` (2 files, 382 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Codebase Locator: Partition 48 of 79

## Scope: `extensions/grunt/`

Task Provider implementation for Grunt task detection and execution in VS Code.

---

## Implementation

### Core Extension Logic
- `extensions/grunt/src/main.ts` — Main extension module providing Grunt task provider integration
  - `TaskDetector` class (lines 230-354) — Manages task provider registration and workspace folder monitoring
  - `FolderDetector` class (lines 85-228) — Detects and computes Grunt tasks per workspace folder
  - `GruntTaskDefinition` interface (lines 66-70) — Type definition extending `vscode.TaskDefinition`
  - `registerTaskProvider('grunt', ...)` invocation (line 296) — Registers the task provider with VS Code
  - Task parsing logic (lines 136-219) — Parses Grunt help output to extract available tasks
  - Task group classification (lines 33-50, 197-201) — Categorizes tasks as Build or Test based on naming patterns

### Utility Functions
- File system helpers: `exists()` (lines 13-19), `exec()` (lines 21-30)
- Task classification: `isBuildTask()` (lines 33-40), `isTestTask()` (lines 43-50)
- Output channel management: `getOutputChannel()`, `showError()` (lines 52-65)
- Grunt command detection: `findGruntCommand()` (lines 72-83)

### Entry Points
- `activate()` function (lines 357-360) — Extension activation hook
- `deactivate()` function (lines 362-364) — Extension cleanup hook

---

## Types / Interfaces

### Type Definitions
- `AutoDetect` type alias (line 11) — Union type for configuration: `'on' | 'off'`
- `GruntTaskDefinition` interface (lines 66-70) — Extends `vscode.TaskDefinition` with properties:
  - `task: string` (required)
  - `args?: string[]` (optional)
  - `file?: string` (optional)

---

## Configuration

### Package Metadata
- `extensions/grunt/package.json` — Extension manifest defining:
  - Activation event: `onTaskType:grunt` (line 26)
  - Configuration schema: `grunt.autoDetect` (lines 40-49) — Enable/disable auto-detection, default is `off`
  - Task definition: Type `grunt` with required `task` property and optional `args`, `file` properties (lines 52-74)
  - Main entry point: `./out/main` (line 24)
  - Capabilities: Virtual workspaces unsupported, untrusted workspaces supported (lines 28-32)
  - Build script: `gulp compile-extension:grunt` (line 17)

### Build Configuration
- `extensions/grunt/tsconfig.json` — TypeScript compiler configuration
  - Source directory: `./src`
  - Output directory: `./out`
  - Node types included

### Build Script
- `extensions/grunt/esbuild.mts` — ESBuild bundler configuration for Node platform
  - Entry point: `src/main.ts` → `dist/main`
  - Uses shared ESBuild common configuration

---

## Documentation

### README
- `extensions/grunt/README.md` — Extension user documentation
  - Feature summary: Integrates Grunt task runner with VS Code task system
  - Task group classification rules (Build, Test)
  - Settings documentation: `grunt.autoDetect` configuration
  - Notice: Extension is bundled with VS Code

---

## Notable Clusters

### Task Detection Pipeline
Files implementing the task detection workflow:
1. `main.ts` — `TaskDetector` orchestrates detection (lines 230-354)
2. `main.ts` — `FolderDetector` computes per-folder tasks (lines 85-228)
3. `main.ts` — Grunt command resolution via `findGruntCommand()` (lines 72-83)
4. `main.ts` — Task parsing from `grunt --help` output (lines 136-219)

### Workspace Monitoring
Task detection with workspace lifecycle integration:
- `FolderDetector.start()` (lines 103-109) — File watcher for Gruntfile changes
- `TaskDetector.updateWorkspaceFolders()` (lines 255-271) — Handles folder additions/removals
- `TaskDetector.updateConfiguration()` (lines 273-291) — Responds to configuration changes
- `TaskDetector.updateProvider()` (lines 293-309) — Manages task provider lifecycle

### Task Provider Implementation
- Registration logic (line 296): `vscode.tasks.registerTaskProvider('grunt', {...})`
- Provider interface implementation (lines 297-302):
  - `provideTasks()` — Returns detected tasks
  - `resolveTask()` — Resolves task definitions to executable tasks

---

## Directory Structure

```
extensions/grunt/
├── src/
│   └── main.ts                  (365 LOC - Core implementation)
├── esbuild.mts                  (19 LOC - Build configuration)
├── tsconfig.json                (18 LOC - TypeScript config)
├── package.json                 (80 LOC - Extension manifest)
├── README.md                     (14 LOC - User documentation)
├── package-lock.json            
├── package.nls.json             (Localization strings)
├── .npmrc                        (npm configuration)
├── .vscodeignore                 (Package exclusions)
└── images/
    └── grunt.png                (Icon asset)
```

**Total: 2 source files (main.ts + build config), 382 LOC**

---

## Summary

The Grunt extension provides VS Code task provider integration for detecting and executing Grunt tasks. The implementation is centered on the `TaskDetector` and `FolderDetector` classes that monitor workspace folders and parse Grunt task definitions via command-line invocation. The extension registers itself with VS Code's task system via `vscode.tasks.registerTaskProvider('grunt', ...)` on line 296 of `main.ts`, enabling automatic task detection when `grunt.autoDetect` configuration is enabled. Task classification rules identify build and test tasks by name pattern matching. The architecture supports multi-folder workspaces with per-folder task detection and file system watching for Gruntfile changes.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/grunt/src/main.ts` — 365 LOC, full TypeScript extension implementation
- `/home/norinlavaee/projects/vscode-atomic/extensions/grunt/esbuild.mts` — 18 LOC, build configuration

---

### Per-File Notes

#### `extensions/grunt/esbuild.mts`

**Role:** Build script for the grunt extension.

- Line 6: Imports `run` from `'../esbuild-extension-common.mts'` — a shared esbuild runner used by multiple extensions.
- Lines 8–9: Resolves `srcDir` to `<ext-root>/src` and `outDir` to `<ext-root>/dist` using `import.meta.dirname`.
- Lines 11–18: Calls `run(...)` with `platform: 'node'`, a single entry point `main` → `src/main.ts`, and the resolved src/out dirs. `process.argv` is forwarded to allow CLI flags.

No business logic; purely delegates to the shared esbuild pipeline.

---

#### `extensions/grunt/src/main.ts`

**Role:** Implements a VS Code `TaskProvider` for Grunt, performing auto-detection of Grunt tasks by running `grunt --help --no-color` in each workspace folder containing a `Gruntfile.js`.

**Type Alias**

- Line 11: `type AutoDetect = 'on' | 'off'` — mirrors the `grunt.autoDetect` configuration setting.

**Interface**

- Lines 66–70: `GruntTaskDefinition extends vscode.TaskDefinition` — adds `task: string`, optional `args?: string[]`, and optional `file?: string` to the base definition.

**Utility Functions**

- Lines 13–18: `exists(file): Promise<boolean>` — wraps `fs.exists` as a promise.
- Lines 21–30: `exec(command, options): Promise<{stdout, stderr}>` — wraps `cp.exec` as a promise; rejects with `{error, stdout, stderr}` on non-zero exit.
- Lines 32–40: `isBuildTask(name): boolean` — returns `true` if `name` contains any of `['build', 'compile', 'watch']` (case-sensitive on raw name; callers pass lowercased).
- Lines 42–50: `isTestTask(name): boolean` — returns `true` if `name` contains `'test'`.
- Lines 52–57: `getOutputChannel(): vscode.OutputChannel` — lazily creates a singleton `OutputChannel` named `'Grunt Auto Detection'`.
- Lines 60–65: `showError()` — shows a warning notification with a "Go to output" action that reveals the output channel.

**`findGruntCommand(rootPath): Promise<string>` (lines 72–83)**

Resolves the grunt binary path for the current workspace folder:

1. On `win32`: checks for `node_modules/.bin/grunt.cmd`; if found, returns the relative path `./node_modules/.bin/grunt.cmd`.
2. On `linux`/`darwin`: checks for `node_modules/.bin/grunt`; if found, returns `./node_modules/.bin/grunt`.
3. Fallback: returns `'grunt'` (relies on system PATH).

**`class FolderDetector` (lines 85–228)**

Manages task detection for a single `vscode.WorkspaceFolder`.

- **Constructor** (lines 90–93): Accepts a `WorkspaceFolder` and a `Promise<string>` for the grunt command.
- **`isEnabled(): boolean`** (lines 99–101): Reads `grunt.autoDetect` config for the folder URI; returns `true` only if value is `'on'`.
- **`start(): void`** (lines 103–109): Creates a `FileSystemWatcher` on the glob pattern `{node_modules,[Gg]runtfile.js}` inside the folder. Any `change`, `create`, or `delete` event sets `this.promise = undefined`, invalidating the cached task list.
- **`getTasks(): Promise<vscode.Task[]>`** (lines 111–120): Returns cached `this.promise` if set; otherwise calls `computeTasks()` and caches the resulting promise.
- **`getTask(_task): Promise<vscode.Task | undefined>`** (lines 122–134): Reconstructs a single `vscode.Task` from a stored definition. If the task name contains a space (line 128–130), it wraps the name in double quotes in the shell command.
- **`computeTasks(): Promise<vscode.Task[]>`** (lines 136–220): The core detection routine:
  1. Lines 137–144: Guards — returns empty if `rootPath` is not a `file:` URI or if neither `gruntfile.js` nor `Gruntfile.js` exists.
  2. Line 146: Builds command `<gruntCommand> --help --no-color`.
  3. Lines 148–205: Calls `exec(commandLine, {cwd: rootPath})`. Parses `stdout` line by line:
     - Lines 167: Splits on `\r?\n`.
     - Lines 174–176: Scans for the sentinel line `'Available tasks'` to begin capture.
     - Lines 179–180: Stops capture on `'Tasks run in the order specified'`.
     - Lines 182–183: Within the capture region, applies regex `/^\s*(\S.*\S)  \S/g` to extract the task name (group 1) — this matches lines where the name is followed by two or more spaces then a non-space character (description separator).
     - Lines 186–195: Constructs a `GruntTaskDefinition` (`type: 'grunt'`, `task: name`) and a `vscode.Task` with `ShellExecution`. Names containing spaces are double-quoted in the shell string.
     - Lines 196–201: Assigns `task.group = vscode.TaskGroup.Build` if `isBuildTask(name.toLowerCase())`, or `task.group = vscode.TaskGroup.Test` if `isTestTask(name.toLowerCase())`.
  4. Lines 208–218: On exec failure, logs `stderr`, `stdout`, and a localised error message to the output channel, then calls `showError()`.
- **`dispose(): void`** (lines 222–227): Clears the cached promise and disposes the file watcher.

**`class TaskDetector` (lines 230–354)**

Orchestrates `FolderDetector` instances across all workspace folders and manages the `TaskProvider` registration lifecycle.

- **Fields** (lines 232–233): `taskProvider: vscode.Disposable | undefined` holds the registered provider token; `detectors: Map<string, FolderDetector>` keyed by folder URI string.
- **`start(): void`** (lines 238–245): Bootstraps detectors for current `workspaceFolders`, then subscribes to `onDidChangeWorkspaceFolders` and `onDidChangeConfiguration`.
- **`updateWorkspaceFolders(added, removed)`** (lines 255–271):
  - Disposes and removes detectors for removed folders.
  - Creates a new `FolderDetector` per added folder; starts it if `isEnabled()`.
  - Calls `updateProvider()`.
- **`updateConfiguration()`** (lines 273–291): Disposes all detectors, re-creates them from current `workspaceFolders`, restarts enabled ones, then calls `updateProvider()`.
- **`updateProvider()`** (lines 293–309):
  - If no provider exists and `detectors.size > 0`: registers a `TaskProvider` for type `'grunt'` via `vscode.tasks.registerTaskProvider` (line 296). The `provideTasks` callback calls `this.getTasks()`; `resolveTask` calls `this.getTask(_task)`.
  - If a provider exists and `detectors.size === 0`: disposes and unregisters it.
- **`computeTasks()`** (lines 315–335): Aggregates across detectors:
  - 0 detectors → resolves `[]`.
  - 1 detector → directly returns its `getTasks()` promise.
  - N detectors → `Promise.all(...)` across all detector `getTasks()` calls, with per-detector error swallowed to `[]`, flattening into one array.
- **`getTask(task)`** (lines 337–353): Resolves a single task by routing to the detector for `task.scope.uri`; returns `undefined` for `Workspace`/`Global` scope or unknown folder.

**Extension Lifecycle (lines 356–364)**

- `activate(_context)` (line 357): Creates a `TaskDetector` and calls `start()`. The `_context` is not used (subscriptions not tracked on it; the detector manages its own disposal).
- `deactivate()` (line 362): Calls `detector.dispose()`.

---

### Cross-Cutting Synthesis

The grunt extension implements a VS Code `TaskProvider` using a two-layer detector pattern. `TaskDetector` acts as a workspace-level orchestrator: it listens for folder and configuration changes to maintain a `Map<string, FolderDetector>` and conditionally registers or disposes the `vscode.tasks` provider registration token. Each `FolderDetector` operates at the folder level, watching for changes to `Gruntfile.js` or `node_modules` to invalidate a cached promise, then re-running `grunt --help --no-color` and parsing its stdout to enumerate tasks. Task group classification (`Build` vs `Test`) is done by substring matching on the lowercased task name against fixed keyword lists. The `findGruntCommand` helper provides platform-aware resolution of the grunt binary, preferring a locally installed version over the global PATH. The build configuration in `esbuild.mts` delegates entirely to a shared `esbuild-extension-common.mts` helper, bundling `src/main.ts` as a Node.js module into `dist/main.js`.

---

### Out-of-Partition References

- `extensions/esbuild-extension-common.mts` — imported at `esbuild.mts:6` as `'../esbuild-extension-common.mts'`; provides the shared `run()` build function used by all extensions.
- `vscode` API module — `vscode.TaskProvider`, `vscode.Task`, `vscode.ShellExecution`, `vscode.TaskGroup`, `vscode.FileSystemWatcher`, `vscode.OutputChannel`, `vscode.tasks.registerTaskProvider`, `vscode.workspace.*`, `vscode.window.*`, `vscode.l10n.t` — all consumed from the VS Code extension host API, not defined in this partition.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder Research: Grunt Extension Task Provider Patterns

**Scope**: `extensions/grunt/` (1 TypeScript file, 365 LOC)  
**Query**: `vscode.tasks.registerTaskProvider($$$)`  
**Date**: 2026-05-01

## Patterns Identified

#### Pattern: Task Provider Registration with State Management
**Where**: extensions/grunt/src/main.ts:296
**What**: Registers a task provider inside a conditional check that manages provider lifecycle. The registration stores the disposable reference and properly disposes it when detector count changes.

```typescript
private updateProvider(): void {
	if (!this.taskProvider && this.detectors.size > 0) {
		const thisCapture = this;
		this.taskProvider = vscode.tasks.registerTaskProvider('grunt', {
			provideTasks: (): Promise<vscode.Task[]> => {
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

**Key aspects**:
- Stores provider as instance variable to control lifecycle
- Implements both `provideTasks` and `resolveTask` methods
- Uses closure capture (`thisCapture`) for context binding
- Conditional creation: only creates when detectors exist
- Conditional cleanup: disposes when no detectors
- Lazy initialization pattern

#### Pattern: Multi-Folder Workspace Detection with Per-Folder Detectors
**Where**: extensions/grunt/src/main.ts:238-245
**What**: Initializes task detection across multiple workspace folders and responds to folder changes by updating internal detector map.

```typescript
public start(): void {
	const folders = vscode.workspace.workspaceFolders;
	if (folders) {
		this.updateWorkspaceFolders(folders, []);
	}
	vscode.workspace.onDidChangeWorkspaceFolders((event) => this.updateWorkspaceFolders(event.added, event.removed));
	vscode.workspace.onDidChangeConfiguration(this.updateConfiguration, this);
}
```

**Key aspects**:
- Queries workspace folders at startup
- Listens to folder addition/removal events
- Listens to configuration changes separately
- Delegates work to update methods

#### Pattern: Folder-Level Task Computation with Caching
**Where**: extensions/grunt/src/main.ts:111-120
**What**: Implements promise-based caching for folder-specific task results, invalidating cache on file changes.

```typescript
public async getTasks(): Promise<vscode.Task[]> {
	if (this.isEnabled()) {
		if (!this.promise) {
			this.promise = this.computeTasks();
		}
		return this.promise;
	} else {
		return [];
	}
}
```

**Key aspects**:
- Stores promise as private field
- Avoids duplicate computations during promise lifecycle
- Cache invalidated by file watcher events (lines 106-108)
- Respects enabled/disabled configuration
- Initializes once per folder

#### Pattern: Task Definition with Custom Interface Extension
**Where**: extensions/grunt/src/main.ts:66-70
**What**: Extends the VS Code TaskDefinition interface to define task-specific metadata for Grunt tasks.

```typescript
interface GruntTaskDefinition extends vscode.TaskDefinition {
	task: string;
	args?: string[];
	file?: string;
}
```

**Key aspects**:
- Inherits from vscode.TaskDefinition
- Adds domain-specific fields (task, args, file)
- Optional args array for additional arguments
- Strongly typed for IDE support

#### Pattern: Shell Execution Configuration with Platform-Specific Commands
**Where**: extensions/grunt/src/main.ts:128-130
**What**: Creates task execution with shell commands that handle spaces in task names and platform-specific grunt binary paths.

```typescript
const task = gruntTask.indexOf(' ') === -1
	? new vscode.Task(taskDefinition, this.workspaceFolder, gruntTask, source, new vscode.ShellExecution(`${await this._gruntCommand}`, [gruntTask, ...taskDefinition.args], options))
	: new vscode.Task(taskDefinition, this.workspaceFolder, gruntTask, source, new vscode.ShellExecution(`${await this._gruntCommand}`, [`"${gruntTask}"`, ...taskDefinition.args], options));
```

**Key aspects**:
- Handles multi-word task names with quoted strings
- Uses shellExecution with separate args array
- Spreads additional args from definition
- Awaits platform-specific command resolution

#### Pattern: Aggregated Task Collection from Multiple Detectors
**Where**: extensions/grunt/src/main.ts:315-335
**What**: Implements tiered task aggregation handling single, multiple, or zero detectors with promise composition.

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

**Key aspects**:
- Optimizes single detector case (no aggregation overhead)
- Collects all promises when multiple detectors exist
- Handles rejections gracefully (falls back to empty array)
- Flattens nested task arrays
- Filters out empty results

#### Pattern: Configuration-Driven Extension Activation and Deactivation
**Where**: extensions/grunt/src/main.ts:273-291
**What**: Recreates all detectors when configuration changes, supporting dynamic enable/disable of task detection.

```typescript
private updateConfiguration(): void {
	for (const detector of this.detectors.values()) {
		detector.dispose();
		this.detectors.delete(detector.workspaceFolder.uri.toString());
	}
	const folders = vscode.workspace.workspaceFolders;
	if (folders) {
		for (const folder of folders) {
			if (!this.detectors.has(folder.uri.toString())) {
				const detector = new FolderDetector(folder, findGruntCommand(folder.uri.fsPath));
				this.detectors.set(folder.uri.toString(), detector);
				if (detector.isEnabled()) {
					detector.start();
				}
			}
		}
	}
	this.updateProvider();
}
```

**Key aspects**:
- Disposes all existing detectors on config change
- Rebuilds detector map from scratch
- Respects enabled state of each folder
- Triggers provider update after reconfiguration

---

## Summary

The Grunt extension implements a sophisticated task provider pattern for VS Code with:

1. **Lifecycle Management**: Task provider registration is tied to detector availability, creating and disposing conditionally
2. **Multi-Folder Support**: Per-folder detectors maintain independent task detection with centralized aggregation
3. **Caching Strategy**: Promise-based caching at folder level with file watcher invalidation
4. **Dynamic Configuration**: Full detector rebuild on config changes with folder-level enable/disable support
5. **Promise Composition**: Three-tier optimization for task aggregation (zero, one, multiple detectors)
6. **Type Safety**: Custom interface extending TaskDefinition for Grunt-specific metadata
7. **Shell Execution**: Platform-aware command resolution with safe handling of spaces in task names

All patterns focus on efficient task discovery and stable task execution across variable workspace configurations.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
