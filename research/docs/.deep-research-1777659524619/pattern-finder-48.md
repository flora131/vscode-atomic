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
