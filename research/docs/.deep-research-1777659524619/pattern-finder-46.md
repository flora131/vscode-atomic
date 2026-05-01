# Gulp Extension Patterns: Task Provider Registration & Build System Integration

Research scope: `extensions/gulp/` (2 files, 424 LOC)

## Overview

The gulp extension demonstrates how VS Code's Electron/TypeScript architecture provides task provider integration, file system watching, process execution, and workspace folder management. These patterns are fundamental to IDE functionality that would require significant architectural changes in a Tauri/Rust port.

---

## Pattern 1: Task Provider Registration with Adaptive Lifecycle

**Where:** `extensions/gulp/src/main.ts:337-344`

**What:** Registers a task provider for the 'gulp' task type with dual methods for lazy-loading and resolving task definitions dynamically.

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

**Variations / call-sites:**
- Registered only when detectors exist (line 335: `if (!this.taskProvider && this.detectors.size > 0)`)
- Disposed when no detectors remain (line 346-349: cleanup on empty state)
- The pattern mirrors npm and grunt extensions' task registration approach

**Key architectural aspects:**
- Lazy provider binding tied to workspace folder detection
- Promise-based async task resolution for UI non-blocking behavior
- Single provider instance manages all workspace folders via internal state

---

## Pattern 2: Custom Task Definition Interface with vscode.TaskDefinition

**Where:** `extensions/gulp/src/main.ts:109-112`

**What:** Extends VS Code's base TaskDefinition with domain-specific properties for gulp tasks.

```typescript
interface GulpTaskDefinition extends vscode.TaskDefinition {
	task: string;
	file?: string;
}
```

**Variations / call-sites:**
- Used in task creation (line 233: task property set to gulp task name)
- Type property hardcoded to 'gulp' (line 234)
- File property optional for future multi-gulpfile scenarios

**Integration points:**
- Aligns with `package.json` contribution point (line 51-69): taskDefinitions schema
- When clause: `"when": "shellExecutionSupported"` gates availability

---

## Pattern 3: Task Construction with ShellExecution

**Where:** `extensions/gulp/src/main.ts:238` and `extensions/gulp/src/main.ts:157`

**What:** Creates Task objects with shell command execution, working directory handling, and task group classification.

```typescript
const task = new vscode.Task(
	kind,
	this.workspaceFolder,
	gulpTask,
	'gulp',
	new vscode.ShellExecution(await this._gulpCommand, [gulpTask], options)
);
```

**ShellExecutionOptions pattern** (line 237, 156):
```typescript
const options: vscode.ShellExecutionOptions = { 
	cwd: this.workspaceFolder.uri.fsPath 
};
```

**Task grouping classification** (lines 241-245):
```typescript
const lowerCaseLine = line.toLowerCase();
if (isBuildTask(lowerCaseLine)) {
	task.group = vscode.TaskGroup.Build;
} else if (isTestTask(lowerCaseLine)) {
	task.group = vscode.TaskGroup.Test;
}
```

**Variations / call-sites:**
- Two instantiation patterns: resolution path (line 157) and enumeration path (line 238)
- Both use same cwd resolution via `workspaceFolder.uri.fsPath`

---

## Pattern 4: File System Watching with Invalidation

**Where:** `extensions/gulp/src/main.ts:132-138`

**What:** Watches gulp configuration files and node_modules to invalidate cached task lists.

```typescript
public start(): void {
	const pattern = path.join(
		this._workspaceFolder.uri.fsPath,
		'{node_modules,gulpfile{.babel.js,.esm.js,.js,.mjs,.cjs,.ts}}'
	);
	this.fileWatcher = vscode.workspace.createFileSystemWatcher(pattern);
	this.fileWatcher.onDidChange(() => this.promise = undefined);
	this.fileWatcher.onDidCreate(() => this.promise = undefined);
	this.fileWatcher.onDidDelete(() => this.promise = undefined);
}
```

**Cache invalidation pattern** (lines 135-137):
- All three events (change, create, delete) set `this.promise = undefined`
- Forces re-computation on next `getTasks()` call (lines 140-149)

**Glob pattern rationale:**
- Watches gulpfile variants: `.babel.js`, `.esm.js`, `.js`, `.mjs`, `.cjs`, `.ts`
- Watches entire `node_modules` for dependency changes (expensive but necessary)

---

## Pattern 5: Command Path Resolution with Platform Awareness

**Where:** `extensions/gulp/src/main.ts:89-107`

**What:** Locates gulp command across Windows and Unix platforms, checking local node_modules first, then global installs.

```typescript
async function findGulpCommand(rootPath: string): Promise<string> {
	const platform = process.platform;

	if (platform === 'win32' && await exists(path.join(rootPath, 'node_modules', '.bin', 'gulp.cmd'))) {
		const globalGulp = path.join(
			process.env.APPDATA ? process.env.APPDATA : '',
			'npm',
			'gulp.cmd'
		);
		if (await exists(globalGulp)) {
			return `"${globalGulp}"`;
		}
		return path.join('.', 'node_modules', '.bin', 'gulp.cmd');
	}

	if ((platform === 'linux' || platform === 'darwin') && 
		await exists(path.join(rootPath, 'node_modules', '.bin', 'gulp'))) {
		return path.join('.', 'node_modules', '.bin', 'gulp');
	}

	return 'gulp';
}
```

**Dependency resolution strategy:**
1. Windows: Local `.cmd` wrapper (line 98)
2. Windows: Global `APPDATA` npm installation (lines 93-96)
3. Unix: Local shell script (line 103)
4. Fallback: System PATH 'gulp' (line 106)

---

## Pattern 6: Child Process Execution with Error Handling

**Where:** `extensions/gulp/src/main.ts:41-50`

**What:** Wraps Node's `child_process.exec()` in a Promise with unified error/output handling.

```typescript
function exec(
	command: string,
	options: cp.ExecOptions
): Promise<{ stdout: string; stderr: string }> {
	return new Promise<{ stdout: string; stderr: string }>((resolve, reject) => {
		cp.exec(command, options, (error, stdout, stderr) => {
			if (error) {
				reject({ error, stdout, stderr });
			}
			resolve({ stdout, stderr });
		});
	});
}
```

**Usage context** (line 216):
```typescript
const { stdout, stderr } = await exec(
	`${await this._gulpCommand} --tasks-simple --no-color`,
	{ cwd: rootPath }
);
```

**Error handling pattern** (lines 217-260):
- Stderr inspected for warnings vs errors
- "No license field" warnings filtered (line 221)
- stderr written to output channel (line 222)
- stdout parsed line-by-line for task names (line 228)

---

## Pattern 7: Multi-Folder Task Aggregation with Scope Routing

**Where:** `extensions/gulp/src/main.ts:352-395`

**What:** Routes task queries across multiple workspace folders, aggregating results with scope-aware resolution.

```typescript
private computeTasks(): Promise<vscode.Task[]> {
	if (this.detectors.size === 0) {
		return Promise.resolve([]);
	} else if (this.detectors.size === 1) {
		return this.detectors.values().next().value!.getTasks();
	} else {
		const promises: Promise<vscode.Task[]>[] = [];
		for (const detector of this.detectors.values()) {
			promises.push(
				detector.getTasks().then((value) => value, () => [])
			);
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

**Scope resolution logic** (lines 378-395):
```typescript
public async getTask(task: vscode.Task): Promise<vscode.Task | undefined> {
	if (this.detectors.size === 0) {
		return undefined;
	} else if (this.detectors.size === 1) {
		return this.detectors.values().next().value!.getTask(task);
	} else {
		if ((task.scope === vscode.TaskScope.Workspace) || 
			(task.scope === vscode.TaskScope.Global)) {
			return undefined;
		} else if (task.scope) {
			const detector = this.detectors.get(task.scope.uri.toString());
			if (detector) {
				return detector.getTask(task);
			}
		}
		return undefined;
	}
}
```

**Multi-folder orchestration:**
- Map of FolderDetector instances keyed by workspace folder URI (line 274)
- Optimization for single-folder workspaces (early returns)
- Error resilience: rejected promises mapped to empty arrays (line 364)

---

## Pattern 8: Configuration-Driven Provider Lifecycle

**Where:** `extensions/gulp/src/main.ts:279-332`

**What:** Wires workspace folder changes and configuration updates to dynamic provider registration.

```typescript
public start(): void {
	const folders = vscode.workspace.workspaceFolders;
	if (folders) {
		this.updateWorkspaceFolders(folders, []);
	}
	vscode.workspace.onDidChangeWorkspaceFolders(
		(event) => this.updateWorkspaceFolders(event.added, event.removed)
	);
	vscode.workspace.onDidChangeConfiguration(
		this.updateConfiguration,
		this
	);
}

private updateConfiguration(): void {
	for (const detector of this.detectors.values()) {
		detector.dispose();
		this.detectors.delete(detector.workspaceFolder.uri.toString());
	}
	const folders = vscode.workspace.workspaceFolders;
	if (folders) {
		for (const folder of folders) {
			if (!this.detectors.has(folder.uri.toString())) {
				const detector = new FolderDetector(
					folder,
					findGulpCommand(folder.uri.fsPath)
				);
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

**Configuration key** (`package.json`, line 39):
```json
"gulp.autoDetect": {
	"scope": "application",
	"type": "string",
	"enum": ["off", "on"],
	"default": "off"
}
```

**Activation event** (`package.json`, line 25):
```json
"activationEvents": ["onTaskType:gulp"]
```

---

## Cross-Cutting Patterns for Tauri/Rust Porting

### 1. **Process Execution Bridge**
The `exec()` wrapper (Pattern 6) abstracts Node's child_process for controlled task invocation. In Rust, this would require inter-process communication (IPC) or spawning via `std::process::Command`, with stdout/stderr piping and error propagation. Tauri's API lacks direct shell task execution comparable to VS Code's ShellExecution.

### 2. **File System Event Loop**
The FileSystemWatcher pattern (Pattern 4) monitors config file changes to invalidate caches. Tauri provides `tauri::fs::watch`, but it requires tokio async runtime integration and doesn't map cleanly to VS Code's event-driven model.

### 3. **Async Task Resolution with Promises**
All task retrieval is Promise-based (Patterns 1, 2, 5, 7). Rust's futures ecosystem differs fundamentally; async/await requires `tokio` or `async-std`, and Task object serialization for IPC becomes complex.

### 4. **Workspace Multi-Folder Routing**
The scope-aware routing (Pattern 7) relies on VS Code's native WorkspaceFolder API and TaskScope enum. Tauri would require:
- Custom folder enumeration and URI-to-path mapping
- Serializable task definitions (JSON/MessagePack)
- Custom task scope enum implementation

### 5. **Extension Activation on Demand**
The `onTaskType:gulp` activation event (line 25) is declarative and automatic. Tauri plugins lack this activation model; startup would be eager or require custom bootstrap logic.

### 6. **Localization and Configuration Schema**
Uses `vscode.l10n.t()` for UI strings and JSON schema in `package.json` for configuration. Tauri i18n is less integrated; config schema would need custom validation.

---

## Summary

The gulp extension, though modest (424 LOC), exposes **critical IDE patterns**:

- **Task provider abstraction**: Lazy-loading, promise-based resolution, multi-folder aggregation
- **Platform abstraction**: Command path resolution, shell option handling
- **File system reactivity**: Watch patterns, cache invalidation
- **Workspace integration**: Folder enumeration, configuration-driven lifecycle
- **Process management**: Child process wrapping with error handling

A Tauri port would require **new abstractions** for:
1. Serializable task definitions and execution APIs
2. Async/await compatibility with futures instead of Promises
3. Custom file watching and change notification events
4. Explicit multi-workspace coordination (no native support)
5. IPC mechanisms for process spawning and output handling

These patterns repeat across npm, grunt, jake, make extensions—indicating a **systematic architectural dependency** on VS Code's extension API for build-system integration.
