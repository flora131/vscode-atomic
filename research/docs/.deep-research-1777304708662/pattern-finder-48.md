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
