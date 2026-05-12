# Tasks API and Task Provider Patterns (extensions/gulp)

## Overview

The gulp extension (424 LOC) demonstrates VS Code's Task Provider API implementation. It registers a task provider for the 'gulp' task type, auto-detects gulpfiles, and provides task execution through shell commands. This pattern is foundational for understanding how to port VS Code's task execution system.

---

## Pattern Catalog

#### Pattern: Task Provider Registration via vscode.tasks.registerTaskProvider

**Where:** `extensions/gulp/src/main.ts:337-344`
**What:** Registers a task provider implementation with two required callbacks: `provideTasks()` and `resolveTask()`.
```typescript
this.taskProvider = vscode.tasks.registerTaskProvider('gulp', {
	provideTasks(): Promise<vscode.Task[]> {
		return thisCapture.getTasks();
	},
	resolveTask(_task: vscode.Task): Promise<vscode.Task | undefined> {
		return thisCapture.getTask(_task);
	}
});
```
**Variations / call-sites:** Called from `updateProvider()` method (line 334) which is invoked whenever workspace folders change or configuration updates. The provider is only registered when detectors exist (line 335) and disposed when none exist (line 347).

---

#### Pattern: Task Definition Interface with Type Discriminator

**Where:** `extensions/gulp/src/main.ts:109-112`
**What:** Defines a typed task definition extending `vscode.TaskDefinition` with a type field that identifies the task system ('gulp') and a task-specific field.
```typescript
interface GulpTaskDefinition extends vscode.TaskDefinition {
	task: string;
	file?: string;
}
```
**Variations / call-sites:** Task definitions are created on lines 233-235 and 238 with `type: 'gulp'` set. The same pattern is declared in `package.json:51-69` via the `taskDefinitions` manifest.

---

#### Pattern: Task Object Construction with ShellExecution

**Where:** `extensions/gulp/src/main.ts:238-239`
**What:** Creates a `vscode.Task` object with shell execution binding to the gulp binary and task arguments.
```typescript
const options: vscode.ShellExecutionOptions = { cwd: this.workspaceFolder.uri.fsPath };
const task = new vscode.Task(kind, this.workspaceFolder, line, 'gulp', new vscode.ShellExecution(await this._gulpCommand, [line], options));
result.push(task);
```
**Variations / call-sites:** Used twice: once in `computeTasks()` (line 238) and once in `getTask()` (line 157). Both pass execution options with `cwd` set to workspace folder path. Task groups are conditionally assigned post-construction (lines 241-245).

---

#### Pattern: Task Grouping via TaskGroup Enum

**Where:** `extensions/gulp/src/main.ts:241-245`
**What:** Assigns semantic task groups (Build, Test) based on task name inspection, used by VS Code for UI organization and default task selection.
```typescript
const lowerCaseLine = line.toLowerCase();
if (isBuildTask(lowerCaseLine)) {
	task.group = vscode.TaskGroup.Build;
} else if (isTestTask(lowerCaseLine)) {
	task.group = vscode.TaskGroup.Test;
}
```
**Variations / call-sites:** Helper functions `isBuildTask()` (line 53) and `isTestTask()` (line 63) check task name patterns. No other task groups are used in this extension.

---

#### Pattern: Multi-Folder Task Aggregation and Folder-Scoped Routing

**Where:** `extensions/gulp/src/main.ts:356-376`
**What:** Aggregates tasks from multiple workspace folders by maintaining a detector per folder and collecting all results, routing task resolution back to the owning folder's detector.
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
**Variations / call-sites:** Mirrored in `getTask()` (lines 378-395) which routes task resolution to the correct folder's detector via `task.scope.uri`. Single-folder case optimized as fast path.

---

#### Pattern: File System Watcher for Invalidation

**Where:** `extensions/gulp/src/main.ts:132-138`
**What:** Creates a file watcher for gulpfile-related patterns and invalidates the memoized task list on any change, triggering re-detection.
```typescript
public start(): void {
	const pattern = path.join(this._workspaceFolder.uri.fsPath, '{node_modules,gulpfile{.babel.js,.esm.js,.js,.mjs,.cjs,.ts}}');
	this.fileWatcher = vscode.workspace.createFileSystemWatcher(pattern);
	this.fileWatcher.onDidChange(() => this.promise = undefined);
	this.fileWatcher.onDidCreate(() => this.promise = undefined);
	this.fileWatcher.onDidDelete(() => this.promise = undefined);
}
```
**Variations / call-sites:** Pattern includes multiple gulpfile extensions (.js, .mjs, .cjs, .ts, .babel.js, .esm.js) and node_modules directory. Watcher is disposed in `dispose()` (line 266).

---

#### Pattern: Extension Lifecycle with activate/deactivate Hooks

**Where:** `extensions/gulp/src/main.ts:399-406`
**What:** Standard VS Code extension entry points that instantiate the task detector on activation and clean up on deactivation.
```typescript
let detector: TaskDetector;
export function activate(_context: vscode.ExtensionContext): void {
	detector = new TaskDetector();
	detector.start();
}

export function deactivate(): void {
	detector.dispose();
}
```
**Variations / call-sites:** Simple pattern with no use of `ExtensionContext` for API registration or storage. The extension activates only on `onTaskType:gulp` event (package.json line 25).

---

## Summary

The gulp extension (406 lines) demonstrates core task execution patterns in VS Code: task provider registration with dual callbacks, typed task definitions, multi-folder task aggregation with folder-scoped routing, shell execution binding, semantic task grouping, filesystem watching for invalidation, and proper lifecycle management. The pattern shows how task providers bridge the extension API to external build tools via shell execution, with file watching enabling live task list updates. To port this to Rust/Tauri would require equivalent APIs for: registering task providers, constructing typed task objects, executing shell commands with working directory context, watching filesystem patterns, and lifecycle management tied to extension activation events.

