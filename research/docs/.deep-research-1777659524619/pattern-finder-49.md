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
