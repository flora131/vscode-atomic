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
