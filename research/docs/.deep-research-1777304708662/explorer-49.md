# Partition 49 of 79 — Findings

## Scope
`extensions/jake/` (2 files, 357 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Jake Extension: Tasks API Contributor (Partition 49)

## Implementation

- `/Users/norinlavaee/vscode-atomic/extensions/jake/src/main.ts` (340 LOC) - Core implementation containing:
  - `TaskDetector` class: Main orchestrator that registers the task provider via `vscode.tasks.registerTaskProvider('jake', ...)` at line 270
  - `FolderDetector` class: Per-workspace folder detector that discovers and parses Jakefile tasks
  - `JakeTaskDefinition` interface extending `vscode.TaskDefinition` with `task` and optional `file` properties
  - Helper functions: `findJakeCommand()`, `isBuildTask()`, `isTestTask()`, `exec()`, `exists()`
  - Activation hooks: `activate()` initializes the detector, `deactivate()` cleans up resources

## Configuration

- `/Users/norinlavaee/vscode-atomic/extensions/jake/package.json` (76 lines) - Extension manifest defining:
  - `contributes.taskDefinitions`: Declares 'jake' task type with required 'task' property and optional 'file' property
  - `contributes.configuration`: Exposes 'jake.autoDetect' setting (default: 'off')
  - `activationEvents`: Triggered on `onTaskType:jake`
  - Build scripts: `compile` and `watch` via gulp

- `/Users/norinlavaee/vscode-atomic/extensions/jake/tsconfig.json` - TypeScript compilation targeting Node environment
- `/Users/norinlavaee/vscode-atomic/extensions/jake/esbuild.mts` - ESBuild configuration for bundling main.ts entry point

## Documentation

- `/Users/norinlavaee/vscode-atomic/extensions/jake/README.md` - User-facing documentation describing Jake task detection, features, and settings

## Notable Characteristics

The Jake extension implements the **tasks API contributor pattern** by registering a `TaskProvider` with `vscode.tasks.registerTaskProvider()`. The provider's two methods (`provideTasks` and `resolveTask`) enable dynamic discovery of Jake tasks from `Jakefile` or `Jakefile.js` by:

1. Monitoring workspace folders and configuration changes
2. Executing `jake --tasks` command to parse available tasks
3. Categorizing tasks as build or test groups based on task name heuristics
4. Wrapping discovered tasks in `vscode.Task` objects with shell execution context

This design pattern mirrors the gulp extension and demonstrates how to integrate external build tool ecosystems into VS Code's task infrastructure. The extension uses workspace-scoped detection to handle multi-root workspaces and respects the autoDetect configuration setting for performance optimization.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Task Provider API Patterns: Jake Extension

## Overview

The Jake extension demonstrates VS Code's task provider registration and consumption pattern. This is a standard pattern used across multiple build tool integrations (Jake, Gulp, Grunt, NPM) that showcases how extensions register task detection and resolution mechanisms.

---

## Patterns Found

#### Pattern: TaskProvider Interface Implementation via registerTaskProvider
**Where:** `extensions/jake/src/main.ts:270-277`
**What:** Registers a task provider that handles task discovery and resolution for Jake tasks.

```typescript
this.taskProvider = vscode.tasks.registerTaskProvider('jake', {
	provideTasks(): Promise<vscode.Task[]> {
		return thisCapture.getTasks();
	},
	resolveTask(_task: vscode.Task): Promise<vscode.Task | undefined> {
		return thisCapture.getTask(_task);
	}
});
```

**Variations / call-sites:**
- Same pattern in `extensions/gulp/src/main.ts:337-344` (Gulp variant)
- Called from `TaskDetector.updateProvider()` method
- Registration is conditional: only registers if detectors exist
- Uses closure (`thisCapture`) to maintain context

---

#### Pattern: Lazy Initialization with Size-Based Registration
**Where:** `extensions/jake/src/main.ts:267-283`
**What:** Conditionally registers task provider only when workspace folders with Jake enabled exist, and unregisters when none remain.

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

**Variations / call-sites:**
- Called from `updateWorkspaceFolders()` after adding/removing folders
- Called from `updateConfiguration()` when settings change
- Resource management: disposes registration when not needed

---

#### Pattern: FolderDetector with Cached Promise Chain
**Where:** `extensions/jake/src/main.ts:111-120`
**What:** Provides workspace folder-specific task detection with cached promise to avoid re-computation.

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
- Cache is invalidated by file watcher callbacks (lines 106-108)
- Similar pattern in Gulp at `extensions/gulp/src/main.ts:140-150`
- Enables efficient multi-folder workspace support

---

#### Pattern: File Watch Invalidation for Auto-Detection
**Where:** `extensions/jake/src/main.ts:103-109`
**What:** Monitors Jakefile and node_modules changes to invalidate cached tasks and trigger re-detection.

```typescript
public start(): void {
	const pattern = path.join(this._workspaceFolder.uri.fsPath, '{node_modules,Jakefile,Jakefile.js}');
	this.fileWatcher = vscode.workspace.createFileSystemWatcher(pattern);
	this.fileWatcher.onDidChange(() => this.promise = undefined);
	this.fileWatcher.onDidCreate(() => this.promise = undefined);
	this.fileWatcher.onDidDelete(() => this.promise = undefined);
}
```

**Variations / call-sites:**
- Pattern differs in Gulp: monitors `'{node_modules,gulpfile{.babel.js,.esm.js,.js,.mjs,.cjs,.ts}}'`
- File system watcher disposal handled in `FolderDetector.dispose()` (line 199)
- Automatically triggers task re-discovery when files change

---

#### Pattern: Multi-Folder Task Aggregation
**Where:** `extensions/jake/src/main.ts:289-309`
**What:** Combines tasks from multiple workspace folders, with optimized paths for zero, one, or many detectors.

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
- Identical pattern in `extensions/gulp/src/main.ts:356-376`
- Error handling: failed detectors resolve to empty arrays
- Same optimization pattern in `getTask()` method (lines 311-327)

---

#### Pattern: Task Definition with Type Field and Custom Properties
**Where:** `extensions/jake/src/main.ts:80-83`
**What:** Defines Jake task schema extending vscode.TaskDefinition with type and task name.

```typescript
interface JakeTaskDefinition extends vscode.TaskDefinition {
	task: string;
	file?: string;
}
```

**Variations / call-sites:**
- Created at task discovery time (line 165-167):
  ```typescript
  const kind: JakeTaskDefinition = {
  	type: 'jake',
  	task: taskName
  };
  ```
- Declared in `package.json` manifest (lines 51-68 of package.json):
  ```json
  "taskDefinitions": [
    {
      "type": "jake",
      "required": ["task"],
      "properties": { ... }
    }
  ]
  ```
- Similar to Gulp (GulpTaskDefinition) and Grunt patterns

---

#### Pattern: Task Creation from Detected Tasks
**Where:** `extensions/jake/src/main.ts:164-178`
**What:** Parses CLI output to create vscode.Task instances with proper categorization.

```typescript
const taskName = matches[1];
const kind: JakeTaskDefinition = {
	type: 'jake',
	task: taskName
};
const options: vscode.ShellExecutionOptions = { cwd: this.workspaceFolder.uri.fsPath };
const task = new vscode.Task(kind, taskName, 'jake', new vscode.ShellExecution(`${await this._jakeCommand} ${taskName}`, options));
result.push(task);
const lowerCaseLine = line.toLowerCase();
if (isBuildTask(lowerCaseLine)) {
	task.group = vscode.TaskGroup.Build;
} else if (isTestTask(lowerCaseLine)) {
	task.group = vscode.TaskGroup.Test;
}
```

**Variations / call-sites:**
- Similar flow in `getTask()` for task resolution (lines 122-130)
- Task grouping: assign to Build or Test group based on task name keywords
- Consistent with Gulp implementation (lines 228-246)

---

## Key Implementation Details

### Configuration-Driven Auto-Detection
Tasks are auto-detected only when configured:
```typescript
public isEnabled(): boolean {
	return vscode.workspace.getConfiguration('jake', this._workspaceFolder.uri).get<AutoDetect>('autoDetect') === 'on';
}
```

Configuration defined in `package.json`:
```json
"jake.autoDetect": {
  "scope": "application",
  "type": "string",
  "enum": ["off", "on"],
  "default": "off"
}
```

### CLI Output Parsing
Tasks discovered by executing Jake with `--tasks` flag:
```typescript
const commandLine = `${await this._jakeCommand} --tasks`;
const { stdout, stderr } = await exec(commandLine, { cwd: rootPath });
const lines = stdout.split(/\r{0,1}\n/);
const regExp = /^jake\s+([^\s]+)\s/g;
```

### Workspace Event Handling
Three key events trigger task provider updates:
1. `vscode.workspace.onDidChangeWorkspaceFolders()` - folder added/removed
2. `vscode.workspace.onDidChangeConfiguration()` - settings changed
3. File system watcher callbacks - Jakefile or node_modules changed

### Activation
Extension activates on task type:
```json
"activationEvents": ["onTaskType:jake"]
```

## Port Considerations for Tauri/Rust

When porting this pattern to Tauri/Rust:

1. **Task Provider Registration**: The `vscode.tasks.registerTaskProvider()` API would need a Rust equivalent or bridge layer in Tauri
2. **Promise-based APIs**: Rust's async/await is compatible but requires careful Future handling and cancellation token support
3. **File System Watching**: Replace `vscode.workspace.createFileSystemWatcher()` with Tauri's file system watching or Rust's `notify` crate
4. **CLI Execution**: Child process execution (`cp.exec`) maps to Rust's `std::process::Command` or Tauri's command invocation
5. **Configuration Access**: Replace `vscode.workspace.getConfiguration()` with custom config system or Tauri's storage API
6. **Task Metadata**: Task definition schema could be replicated via TypeScript interfaces in the Tauri frontend or JSON schemas
7. **Multi-folder Support**: The folder detector pattern is language-agnostic and could translate directly to Rust structs with similar lifecycle management

The core pattern is highly generalizable: detect tasks → register provider → lazy-load on demand → cache with invalidation. This would work equally well in Rust with appropriate async/concurrency primitives.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
