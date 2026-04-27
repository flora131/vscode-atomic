# Partition 48 of 79 — Findings

## Scope
`extensions/grunt/` (2 files, 382 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator 48: extensions/grunt

## Scope Summary
- **Directory**: `extensions/grunt/`
- **File Count**: 2 source files (plus configuration/metadata)
- **Lines of Code**: ~364 LOC (main.ts)
- **Pattern**: Build task provider extension following same architecture as gulp

## Implementation

### Core Task Provider
- `/Users/norinlavaee/vscode-atomic/extensions/grunt/src/main.ts` (364 lines)
  - Main entry point with `activate()` and `deactivate()` exports
  - **TaskDetector class**: Central orchestrator managing task detection lifecycle across all workspace folders
    - Maintains map of FolderDetector instances keyed by workspace folder URI
    - Registers the task provider via `vscode.tasks.registerTaskProvider('grunt', {...})`
    - Implements `provideTasks()` and `resolveTask()` provider interface
    - Responds to workspace folder additions/removals and configuration changes
  - **FolderDetector class**: Per-folder task discovery for individual workspaces
    - Watches for gruntfile.js/Gruntfile.js presence changes
    - Executes `grunt --help --no-color` to enumerate available tasks
    - Parses text output to extract task names and descriptions
    - Caches computed tasks and invalidates on file changes
    - Classifies tasks as Build or Test based on name patterns
    - Constructs vscode.Task objects with ShellExecution for task invocation
  - **Utility Functions**:
    - `findGruntCommand()`: Resolves platform-specific grunt binary path (grunt.cmd on Windows, grunt on Unix)
    - `isBuildTask()`: Identifies tasks containing 'build', 'compile', or 'watch'
    - `isTestTask()`: Identifies tasks containing 'test'
    - `exists()`, `exec()`: File system and process execution helpers
  - **GruntTaskDefinition Interface**: Extends vscode.TaskDefinition with task name, optional args, and file properties

## Configuration

### Extension Manifest
- `/Users/norinlavaee/vscode-atomic/extensions/grunt/package.json`
  - Activation Event: `onTaskType:grunt` (lazy load when grunt tasks referenced)
  - Contributes:
    - Configuration schema with `grunt.autoDetect` boolean setting (default: off, scope: application)
    - Task definition type `'grunt'` with required `task` field and optional `args`/`file` fields
  - Capabilities: Disabled for virtual workspaces, supported in untrusted workspaces

### Localization Strings
- `/Users/norinlavaee/vscode-atomic/extensions/grunt/package.nls.json`
  - Localized descriptions for settings and task definition properties

### Build Configuration
- `/Users/norinlavaee/vscode-atomic/extensions/grunt/tsconfig.json`: TypeScript compilation config
- `/Users/norinlavaee/vscode-atomic/extensions/grunt/esbuild.mts`: ESBuild build script
- `/Users/norinlavaee/vscode-atomic/extensions/grunt/package-lock.json`: Dependency lock file

## Documentation

- `/Users/norinlavaee/vscode-atomic/extensions/grunt/README.md`
  - User-facing extension overview
  - Feature summary: integrates Grunt task definitions as VS Code tasks
  - Build task classification rule documentation
  - Settings documentation

## Notable Patterns

**Task Provider Registration**: Line 296 shows the core API call:
```typescript
vscode.tasks.registerTaskProvider('grunt', {
  provideTasks: (): Promise<vscode.Task[]> => {...},
  resolveTask(_task: vscode.Task): Promise<vscode.Task | undefined> => {...}
})
```

**Task Discovery Flow**: 
1. FolderDetector monitors workspace folder for gruntfile.js via FileSystemWatcher
2. On detection, executes `grunt --help` to parse available tasks
3. Constructs Task objects with ShellExecution for deferred execution
4. TaskDetector aggregates results across multiple workspace folders

**Key Dependencies**: Only @types/node in devDependencies; uses vscode API for all functionality.

---

## Summary

The Grunt extension (partition 48) implements a TaskProvider following the identical architectural pattern as the gulp extension. It provides automatic Grunt task detection by executing `grunt --help` to enumerate available tasks and expose them to VS Code's task system. The extension uses lazy activation via `onTaskType:grunt` and maintains per-folder detectors to handle multi-root workspaces. The implementation demonstrates clean separation between task discovery logic (FolderDetector) and provider registration lifecycle (TaskDetector), with platform-aware grunt command resolution and task classification based on naming conventions (build vs. test). File count is minimal (2 source files) with configuration defined through package.json's contributes schema.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

1. `/Users/norinlavaee/vscode-atomic/extensions/grunt/src/main.ts` (364 LOC)
2. `/Users/norinlavaee/vscode-atomic/extensions/grunt/package.json` (80 LOC)

---

### Per-File Notes

#### `/Users/norinlavaee/vscode-atomic/extensions/grunt/src/main.ts`

**Role:** Implements the Grunt task auto-detection extension. Discovers Grunt tasks from workspace folders and registers them with VS Code's task system as runnable `ShellExecution` tasks.

**Key Symbols:**

- `AutoDetect` (`main.ts:11`) — string union type `'on' | 'off'` controlling whether auto-detection is enabled per folder.
- `GruntTaskDefinition` (`main.ts:66-70`) — interface extending `vscode.TaskDefinition` with fields `task: string`, optional `args?: string[]`, and optional `file?: string`.
- `exists(file)` (`main.ts:13-19`) — wraps `fs.exists` in a Promise.
- `exec(command, options)` (`main.ts:21-30`) — wraps `cp.exec` in a Promise, rejecting with `{ error, stdout, stderr }` on non-zero exit.
- `isBuildTask(name)` (`main.ts:33-40`) — checks if task name contains any of `['build', 'compile', 'watch']`.
- `isTestTask(name)` (`main.ts:43-50`) — checks if task name contains `'test'`.
- `getOutputChannel()` (`main.ts:53-58`) — lazily creates a singleton `vscode.OutputChannel` named `'Grunt Auto Detection'`.
- `showError()` (`main.ts:60-65`) — shows a warning notification with a link to the output channel.
- `findGruntCommand(rootPath)` (`main.ts:72-83`) — resolves the grunt binary path: on `win32`, checks `node_modules/.bin/grunt.cmd`; on `linux`/`darwin`, checks `node_modules/.bin/grunt`; falls back to global `'grunt'`.
- `FolderDetector` class (`main.ts:85-228`) — per-workspace-folder detector. Holds a `fileWatcher` (`vscode.FileSystemWatcher`) and a cached `promise` of `vscode.Task[]`.
  - `start()` (`main.ts:103-109`) — creates a file system watcher on the glob `{node_modules,[Gg]runtfile.js}` within the folder; any change/create/delete sets `this.promise = undefined` (cache invalidation).
  - `getTasks()` (`main.ts:111-120`) — returns cached promise or calls `computeTasks()`.
  - `getTask(_task)` (`main.ts:122-134`) — resolves a single task by constructing a `ShellExecution` from `_task.definition.task` and `_task.definition.args`, quoting the task name if it contains spaces (`main.ts:129-130`).
  - `computeTasks()` (`main.ts:136-220`) — the core detection logic:
    1. Validates that `rootPath` is a `file:` URI scheme (`main.ts:137-139`).
    2. Checks for the existence of `gruntfile.js` or `Gruntfile.js` (`main.ts:142-144`).
    3. Runs `grunt --help --no-color` via `exec()` (`main.ts:146-148`).
    4. Parses stdout line-by-line (`main.ts:167`), scanning for the `'Available tasks'` sentinel (`main.ts:175`) to start, and `'Tasks run in the order specified'` to stop (`main.ts:179`).
    5. Applies regex `/^\s*(\S.*\S)  \S/g` (`main.ts:182`) to extract each task name from the indented list.
    6. Constructs a `GruntTaskDefinition` (`main.ts:186-189`) and a `vscode.Task` with a `ShellExecution` wrapping `grunt <taskName>`, quoting names containing spaces (`main.ts:192-194`).
    7. Assigns `task.group = vscode.TaskGroup.Build` or `vscode.TaskGroup.Test` based on name classification (`main.ts:197-201`).
    8. On any exec error, logs stdout/stderr to output channel and calls `showError()` (`main.ts:208-218`).
  - `dispose()` (`main.ts:222-227`) — clears cached promise and disposes file watcher.

- `TaskDetector` class (`main.ts:230-354`) — workspace-level coordinator. Maintains a `Map<string, FolderDetector>` keyed by folder URI string, and a single registered `vscode.Disposable` for the task provider.
  - `start()` (`main.ts:238-245`) — iterates current workspace folders, subscribes to `onDidChangeWorkspaceFolders` and `onDidChangeConfiguration`.
  - `updateWorkspaceFolders(added, removed)` (`main.ts:255-271`) — disposes detectors for removed folders; creates new `FolderDetector` instances for added folders; calls `updateProvider()`.
  - `updateConfiguration()` (`main.ts:273-291`) — disposes all detectors and recreates them; calls `updateProvider()`.
  - `updateProvider()` (`main.ts:293-309`) — calls `vscode.tasks.registerTaskProvider('grunt', { provideTasks, resolveTask })` if detectors exist but provider is not yet registered (`main.ts:296`); disposes provider if no detectors remain (`main.ts:305-308`).
  - `computeTasks()` (`main.ts:315-335`) — if one detector, delegates directly; if multiple, fans out with `Promise.all`, flattening results.
  - `getTask(task)` (`main.ts:337-353`) — routes `resolveTask` to the correct `FolderDetector` by matching `task.scope.uri` (`main.ts:346-348`).

- `activate(_context)` (`main.ts:357-360`) — module entry point; instantiates `TaskDetector` and calls `start()`.
- `deactivate()` (`main.ts:362-364`) — calls `detector.dispose()`.

**Control Flow:**

```
activate()
  → new TaskDetector()
  → TaskDetector.start()
      → updateWorkspaceFolders(currentFolders, [])
          → new FolderDetector(folder, findGruntCommand(folder.uri.fsPath))
          → FolderDetector.start()    [if autoDetect=on]
              → createFileSystemWatcher('{node_modules,[Gg]runtfile.js}')
          → updateProvider()
              → vscode.tasks.registerTaskProvider('grunt', { provideTasks, resolveTask })
      → subscribe onDidChangeWorkspaceFolders → updateWorkspaceFolders
      → subscribe onDidChangeConfiguration   → updateConfiguration

provideTasks() [called by VS Code task system]
  → TaskDetector.getTasks()
  → TaskDetector.computeTasks()
  → FolderDetector.getTasks()
  → FolderDetector.computeTasks()   [if cache miss]
      → findGruntCommand(rootPath)
      → exec('grunt --help --no-color', { cwd: rootPath })
      → parse stdout: extract task names between 'Available tasks' and 'Tasks run in the order specified'
      → for each name: new vscode.Task(..., new vscode.ShellExecution(...))
      → classify into TaskGroup.Build or TaskGroup.Test

resolveTask(_task) [called by VS Code when task is run from tasks.json]
  → TaskDetector.getTask(_task)
  → FolderDetector.getTask(_task)
      → new vscode.Task(..., new vscode.ShellExecution('grunt', [taskName, ...args], {cwd}))
```

**Data Flow:**

- Input: workspace folder URIs, `grunt.autoDetect` configuration, filesystem presence of `gruntfile.js`/`Gruntfile.js`, stdout of `grunt --help --no-color`.
- Intermediate: raw stdout string → split by `\r?\n` → filtered section between sentinel lines → regex-extracted task name strings.
- Output: `vscode.Task[]` with `ShellExecution`, assigned `TaskGroup`, contributed to VS Code task picker.

**Dependencies:**

- Node built-ins: `path` (`main.ts:6`), `fs` (`main.ts:7`), `child_process` as `cp` (`main.ts:8`).
- VS Code Extension API: `vscode.workspace`, `vscode.tasks`, `vscode.window`, `vscode.l10n`, `vscode.Task`, `vscode.ShellExecution`, `vscode.TaskGroup`, `vscode.FileSystemWatcher` (all via `import * as vscode from 'vscode'` at `main.ts:9`).
- Runtime dependency on the `grunt` CLI binary, either local (`node_modules/.bin/grunt[.cmd]`) or global PATH.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/grunt/package.json`

**Role:** Extension manifest declaring activation event, task schema contributions, and configuration properties.

**Key Symbols and Declarations:**

- `"activationEvents": ["onTaskType:grunt"]` (`package.json:25-27`) — extension activates only when VS Code encounters a task of type `grunt`, avoiding unnecessary startup cost.
- `"main": "./out/main"` (`package.json:24`) — compiled output entry point.
- `"capabilities"` (`package.json:28-33`):
  - `"virtualWorkspaces": false` — extension does not support virtual file system workspaces (requires real filesystem to run grunt CLI).
  - `"untrustedWorkspaces": { "supported": true }` — allowed in restricted mode workspaces.
- `"contributes"."configuration"` (`package.json:36-51`):
  - Property `grunt.autoDetect` with type `string`, enum `["off","on"]`, default `"off"` (`package.json:41-49`). Controls the `AutoDetect` type used by `FolderDetector.isEnabled()` at `main.ts:100`.
- `"contributes"."taskDefinitions"` (`package.json:52-74`):
  - Defines schema for tasks of type `grunt` with required field `task` (string), optional `args` (array), optional `file` (string).
  - `"when": "shellExecutionSupported"` (`package.json:73`) — conditionally hides task type when shell execution is not available (e.g., web/browser contexts).
- `"devDependencies"`: only `@types/node: "22.x"` (`package.json:22`); no runtime npm dependencies.

---

### Cross-Cutting Synthesis

The `extensions/grunt` partition implements VS Code's built-in Grunt task provider as a self-contained TypeScript extension compiled to `./out/main`. The architecture uses two cooperating classes: `TaskDetector` (`main.ts:230`) acts as the workspace-level lifecycle manager, tracking one `FolderDetector` (`main.ts:85`) per workspace folder in a `Map<string, FolderDetector>`. Task discovery relies entirely on spawning the Grunt CLI via `child_process.exec` with `--help --no-color` (`main.ts:146`) and parsing stdout with a state machine that delimiters on two sentinel strings and a two-space-gap regex (`main.ts:167-204`). Detected tasks are wrapped as `vscode.Task` with `vscode.ShellExecution`, meaning actual execution is entirely delegated to VS Code's integrated terminal. Cache invalidation is event-driven via `vscode.FileSystemWatcher` on `gruntfile.js` and `node_modules` changes (`main.ts:106-108`). The manifest's `onTaskType:grunt` activation event (`package.json:26`) and `"virtualWorkspaces": false` capability (`package.json:29`) constrain this extension to environments with a real filesystem and shell execution. For a Tauri/Rust port, the subprocess invocation pattern (`cp.exec` → parse stdout) would need to map to `std::process::Command` or Tauri's sidecar API, and the `vscode.tasks` registration surface has no direct Tauri equivalent, requiring a custom task discovery and execution subsystem.

---

### Out-of-Partition References

- `vscode.tasks.registerTaskProvider` — VS Code core task API, implemented in `src/vs/workbench/api/common/extHostTask.ts` and `src/vs/workbench/contrib/tasks/`.
- `vscode.ShellExecution` — shell execution type defined in VS Code extension host API (`src/vs/workbench/api/common/extHostTypes.ts`).
- `vscode.TaskGroup.Build` / `vscode.TaskGroup.Test` — task group constants from VS Code API.
- `vscode.workspace.createFileSystemWatcher` — file watcher API from `src/vs/workbench/api/common/extHostFileSystemEventService.ts`.
- `vscode.l10n.t(...)` — localization API from `src/vs/workbench/api/common/extHostLocalization.ts`.
- `gulp compile-extension:grunt` build task — defined in the root `Gulpfile.js` build system.
- Analogous sibling extensions: `extensions/npm/`, `extensions/jake/` follow the same `FolderDetector`/`TaskDetector` pattern for other task runners.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder: Grunt Task Provider API Consumption

## Research Question
How does the Grunt extension register and consume VS Code's task provider API? This pattern is critical for understanding task system integration when porting IDE functionality to alternative platforms.

---

## Patterns Found

#### Pattern 1: TaskDefinition Interface with Extension-Specific Fields
**Where:** `extensions/grunt/src/main.ts:66-70`
**What:** Extends the vscode.TaskDefinition interface to define task-specific metadata structure.
```typescript
interface GruntTaskDefinition extends vscode.TaskDefinition {
	task: string;
	args?: string[];
	file?: string;
}
```

**Variations / call-sites:**
- Used in computeTasks() at line 186-189 when creating task kind objects
- Serializes as JSON in package.json taskDefinitions schema (package.json:52-74)
- Matches the `onTaskType:grunt` activation event (package.json:26)

---

#### Pattern 2: registerTaskProvider with Promise-Based Resolution
**Where:** `extensions/grunt/src/main.ts:296-303`
**What:** Registers a task provider with both provideTasks and resolveTask methods using Promises.
```typescript
this.taskProvider = vscode.tasks.registerTaskProvider('grunt', {
	provideTasks: (): Promise<vscode.Task[]> => {
		return thisCapture.getTasks();
	},
	resolveTask(_task: vscode.Task): Promise<vscode.Task | undefined> {
		return thisCapture.getTask(_task);
	}
});
```

**Variations / call-sites:**
- Called conditionally in updateProvider() (line 293-309)
- Registration only happens when detectors.size > 0 (line 294)
- Task provider is disposed and set to undefined when detectors are empty (line 305-308)
- Returns a Disposable that is stored for later cleanup

---

#### Pattern 3: Lazy Task Computation with Memoization
**Where:** `extensions/grunt/src/main.ts:111-120`
**What:** Caches task promise per folder detector to avoid redundant computation.
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

**Variations / call-sites:**
- Promise is cleared on file system changes (lines 106-108)
- Promise is reset during configuration updates (FolderDetector.dispose() at line 223)
- Works per-workspace folder via FolderDetector instances (line 264-265)

---

#### Pattern 4: Task Creation from Shell Execution
**Where:** `extensions/grunt/src/main.ts:128-131, 192-195`
**What:** Creates vscode.Task objects by wrapping ShellExecution with task definition metadata.
```typescript
const task = gruntTask.indexOf(' ') === -1
	? new vscode.Task(taskDefinition, this.workspaceFolder, gruntTask, source, new vscode.ShellExecution(`${await this._gruntCommand}`, [gruntTask, ...taskDefinition.args], options))
	: new vscode.Task(taskDefinition, this.workspaceFolder, gruntTask, source, new vscode.ShellExecution(`${await this._gruntCommand}`, [`"${gruntTask}"`, ...taskDefinition.args], options));
```

**Variations / call-sites:**
- Used in getTask() method (line 122-134) for resolving individual tasks
- Used in computeTasks() method (line 192-194) for building task array from CLI output
- Handles quoting for task names with spaces
- Includes shellExecutionOptions with cwd set to workspace folder path

---

#### Pattern 5: Multi-Folder Aggregation with Promise.all
**Where:** `extensions/grunt/src/main.ts:315-335`
**What:** Aggregates tasks from multiple workspace folders using Promise.all pattern.
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

**Variations / call-sites:**
- Provides fallback to empty array on rejection (then callback with empty array)
- Optimizes single-folder case to avoid unnecessary Promise nesting
- Called from provideTasks via getTasks() at line 311-313

---

#### Pattern 6: Task Categorization by Group
**Where:** `extensions/grunt/src/main.ts:197-201`
**What:** Assigns TaskGroup (Build/Test) based on task name matching patterns.
```typescript
const lowerCaseTaskName = name.toLowerCase();
if (isBuildTask(lowerCaseTaskName)) {
	task.group = vscode.TaskGroup.Build;
} else if (isTestTask(lowerCaseTaskName)) {
	task.group = vscode.TaskGroup.Test;
}
```

**Variations / call-sites:**
- Uses helper functions isBuildTask() (lines 33-40) and isTestTask() (lines 43-50)
- Build tasks matched against: 'build', 'compile', 'watch' (line 32)
- Test tasks matched against: 'test' (line 42)
- Applied only during task creation in computeTasks()

---

#### Pattern 7: Workspace Folder Change Lifecycle
**Where:** `extensions/grunt/src/main.ts:238-245, 255-271`
**What:** Updates task provider registration when workspace folders are added/removed or configuration changes.
```typescript
public start(): void {
	const folders = vscode.workspace.workspaceFolders;
	if (folders) {
		this.updateWorkspaceFolders(folders, []);
	}
	vscode.workspace.onDidChangeWorkspaceFolders((event) => this.updateWorkspaceFolders(event.added, event.removed));
	vscode.workspace.onDidChangeConfiguration(this.updateConfiguration, this);
}

private updateWorkspaceFolders(added: readonly vscode.WorkspaceFolder[], removed: readonly vscode.WorkspaceFolder[]): void {
	for (const remove of removed) {
		const detector = this.detectors.get(remove.uri.toString());
		if (detector) {
			detector.dispose();
			this.detectors.delete(remove.uri.toString());
		}
	}
	for (const add of added) {
		const detector = new FolderDetector(add, findGruntCommand(add.uri.fsPath));
		this.detectors.set(add.uri.toString(), detector);
		if (detector.isEnabled()) {
			detector.start();
		}
	}
	this.updateProvider();
}
```

**Variations / call-sites:**
- detectors Map keyed by workspace folder URI string (line 233, 257, 265)
- Each FolderDetector manages file watchers and task caching independently
- Configuration changes trigger full detector rebuild (lines 273-291)
- updateProvider() ensures task provider registration stays in sync with detector count

---

## Summary

The Grunt extension implements a multi-folder, lazy-evaluated task provider system that:

1. **Registration Model**: Uses `vscode.tasks.registerTaskProvider('grunt', {...})` with a key matching the task type defined in package.json taskDefinitions.

2. **Two-Method Interface**: Implements both provideTasks (discovery) and resolveTask (resolution), allowing VS Code to defer task loading until needed while supporting pre-defined tasks.

3. **Folder-Level Isolation**: Maintains per-workspace-folder FolderDetector instances with independent file watchers, configuration tracking, and task caching via the promise memoization pattern.

4. **Shell Execution Wrapper**: Tasks execute via vscode.ShellExecution, spawning the grunt CLI tool with arguments parsed from task definitions or discovered from `grunt --help` output.

5. **Activation by Task Type**: Extension activates via `onTaskType:grunt` (only when a task of type "grunt" is invoked), reducing startup overhead.

6. **Cleanup on Disposal**: Implements proper disposal chain (extension → TaskDetector → FolderDetector → FileSystemWatcher) with task provider registration toggled based on detector count.

The pattern mirrors gulp's implementation (same directory structure, similar class hierarchy) but is specialized for Grunt's CLI interface and task definition schema. This is foundational for understanding how a Rust/Tauri-based IDE would need to implement task provider plugins—the promise-based async model and workspace folder abstractions are central to VS Code's extensibility architecture.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
