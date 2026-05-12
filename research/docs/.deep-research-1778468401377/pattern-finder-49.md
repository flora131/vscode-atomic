# Pattern Finder Research: Jake Extension Task Registration

**Scope:** `extensions/jake/` (2 files, 357 LOC)
**Research Question:** Port VS Code core IDE from TS/Electron to Tauri/Rust
**Task Category:** Tasks-API Consumer Patterns

## Patterns Found

#### Pattern 1: Task Provider Registration via vscode.tasks.registerTaskProvider
**Where:** `extensions/jake/src/main.ts:270-277`
**What:** Deferred task provider registration triggered by detector size. Registers provider only when detectors become available, unregisters when they're all removed. Implements the stateful provider lifecycle pattern.

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

**Variations:**
- **Implicit vs Explicit:** Jake/Gulp use deferred registration (only when folders available), while npm registers immediately if workspace folders exist
- **Disposable tracking:** Stored in `taskProvider` field for lifecycle management
- **Provider shape:** Both `provideTasks()` and `resolveTask()` required for full provider contract

#### Pattern 2: Multi-Folder Task Detection via TaskDetector Aggregator
**Where:** `extensions/jake/src/main.ts:204-228`
**What:** Aggregates task detection across workspace folders using a Map. Handles dynamic folder addition/removal and configuration changes. Delegates to per-folder FolderDetector instances.

```typescript
class TaskDetector {
	private taskProvider: vscode.Disposable | undefined;
	private detectors: Map<string, FolderDetector> = new Map();

	public start(): void {
		const folders = vscode.workspace.workspaceFolders;
		if (folders) {
			this.updateWorkspaceFolders(folders, []);
		}
		vscode.workspace.onDidChangeWorkspaceFolders((event) => 
			this.updateWorkspaceFolders(event.added, event.removed));
		vscode.workspace.onDidChangeConfiguration(this.updateConfiguration, this);
	}

	private updateWorkspaceFolders(added: readonly vscode.WorkspaceFolder[], 
									removed: readonly vscode.WorkspaceFolder[]): void {
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
}
```

**Variations:**
- **Key strategy:** Uses `workspaceFolder.uri.toString()` as unique folder identifier
- **Lazy initialization:** FolderDetector only started if extension is enabled for that folder
- **Configuration reactivity:** `onDidChangeConfiguration` triggers full detector rebuild

#### Pattern 3: Per-Folder File System Watching with Caching
**Where:** `extensions/jake/src/main.ts:85-120`
**What:** FolderDetector watches for Jakefile changes and invalidates cached tasks via file watcher events. Caches task computation promise to avoid re-execution.

```typescript
class FolderDetector {
	private fileWatcher: vscode.FileSystemWatcher | undefined;
	private promise: Thenable<vscode.Task[]> | undefined;

	public start(): void {
		const pattern = path.join(this._workspaceFolder.uri.fsPath, 
			'{node_modules,Jakefile,Jakefile.js}');
		this.fileWatcher = vscode.workspace.createFileSystemWatcher(pattern);
		this.fileWatcher.onDidChange(() => this.promise = undefined);
		this.fileWatcher.onDidCreate(() => this.promise = undefined);
		this.fileWatcher.onDidDelete(() => this.promise = undefined);
	}

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
}
```

**Variations:**
- **Glob pattern:** Jake watches `{node_modules,Jakefile,Jakefile.js}` at workspace root
- **Cache invalidation:** All three file watcher events (change/create/delete) clear the cached promise
- **Promise memoization:** Stores Thenable to deduplicate concurrent requests

#### Pattern 4: Shell Execution Task Creation with Workspace-Scoped Options
**Where:** `extensions/jake/src/main.ts:127`, `170`
**What:** Creates vscode.Task instances with shell execution, setting working directory via ShellExecutionOptions. Two creation patterns: one for resolved tasks, one for discovered tasks.

```typescript
// Pattern A: Resolving a task request
const kind = _task.definition as JakeTaskDefinition;
const options: vscode.ShellExecutionOptions = { cwd: this.workspaceFolder.uri.fsPath };
const task = new vscode.Task(kind, this.workspaceFolder, jakeTask, 'jake', 
	new vscode.ShellExecution(await this._jakeCommand, [jakeTask], options));

// Pattern B: Discovered task from command output
const task = new vscode.Task(kind, taskName, 'jake', 
	new vscode.ShellExecution(`${await this._jakeCommand} ${taskName}`, options));
```

**Variations:**
- **Command formation:** Pattern B embeds task name in command string, Pattern A passes as argument array
- **Task scoping:** Both use `this.workspaceFolder` for proper folder-scoped execution
- **Execution type:** Always `vscode.ShellExecution` (not Process execution)

#### Pattern 5: Task Auto-Classification via Heuristic Matching
**Where:** `extensions/jake/src/main.ts:32-50`, `172-177`
**What:** Classifies discovered tasks into TaskGroups (Build, Test) using simple string matching heuristics on task names.

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

// Applied during task discovery:
const lowerCaseLine = line.toLowerCase();
if (isBuildTask(lowerCaseLine)) {
	task.group = vscode.TaskGroup.Build;
} else if (isTestTask(lowerCaseLine)) {
	task.group = vscode.TaskGroup.Test;
}
```

**Variations:**
- **Case insensitive:** Converted to lowercase before matching
- **Substring matching:** Uses `indexOf()` for partial matches (not exact)
- **Fallback:** Unmatched tasks have no group assigned

#### Pattern 6: Command Discovery via Process Execution with Platform Detection
**Where:** `extensions/jake/src/main.ts:67-78`
**What:** Finds the jake command path by checking platform-specific locations first (node_modules/.bin), falling back to system PATH.

```typescript
async function findJakeCommand(rootPath: string): Promise<string> {
	let jakeCommand: string;
	const platform = process.platform;
	if (platform === 'win32' && await exists(path.join(rootPath!, 'node_modules', '.bin', 'jake.cmd'))) {
		jakeCommand = path.join('.', 'node_modules', '.bin', 'jake.cmd');
	} else if ((platform === 'linux' || platform === 'darwin') && 
		await exists(path.join(rootPath!, 'node_modules', '.bin', 'jake'))) {
		jakeCommand = path.join('.', 'node_modules', '.bin', 'jake');
	} else {
		jakeCommand = 'jake';
	}
	return jakeCommand;
}
```

**Variations:**
- **Windows special case:** Uses `.cmd` wrapper for Windows
- **Unix variants:** Treats Linux and macOS identically
- **Async file existence check:** Uses Promise-based `exists()` utility
- **Path construction:** Uses relative paths (`.` prefix) for local installations

#### Pattern 7: Error Handling with User Notifications and Output Channel
**Where:** `extensions/jake/src/main.ts:60-65`, `148-193`
**What:** Captures stderr/stdout from command execution, logs to output channel, and shows user warning. Gracefully returns empty task list on error.

```typescript
function showError() {
	vscode.window.showWarningMessage(
		vscode.l10n.t("Problem finding jake tasks. See the output for more information."),
		vscode.l10n.t("Go to output")).then(() => {
			getOutputChannel().show(true);
		});
}

// Usage in computeTasks:
try {
	const { stdout, stderr } = await exec(commandLine, { cwd: rootPath });
	if (stderr) {
		getOutputChannel().appendLine(stderr);
		showError();
	}
	// ... parse stdout
	return result;
} catch (err) {
	const channel = getOutputChannel();
	if (err.stderr) channel.appendLine(err.stderr);
	if (err.stdout) channel.appendLine(err.stdout);
	channel.appendLine(vscode.l10n.t("Auto detecting Jake for folder {0} failed..."));
	showError();
	return emptyTasks;  // Graceful fallback
}
```

**Variations:**
- **Dual notification:** Both modal warning and output channel
- **Localization:** Uses `vscode.l10n.t()` for all user-facing strings
- **Channel naming:** Output channel created with descriptive label ("Jake Auto Detection")

---

## Summary

The Jake extension implements a stateful, multi-folder task provider following a consistent pattern across Jake/Gulp/Grunt families:

1. **Lifecycle Management:** Task provider registration is deferred and tied to detector availability
2. **Multi-Workspace Support:** Central TaskDetector aggregates per-folder FolderDetector instances
3. **File Watching:** Watches for task definition files (Jakefile) and invalidates cached results on changes
4. **Task Discovery:** Executes `jake --tasks` via shell, parses text output, creates vscode.Task instances
5. **Classification:** Auto-categorizes discovered tasks into Build/Test groups via heuristics
6. **Platform Abstraction:** Detects and uses local or system jake command with platform-specific executables
7. **Error Resilience:** Captures stderr, logs to output channel, shows user notifications, returns gracefully on failure

Key Tauri/Rust porting considerations:
- Task provider registration requires async Tasks API interop layer
- Multi-folder detection and file watching need native file system event handling
- Shell execution model (with CWD scoping) maps to subprocess spawning with environment setup
- Output channel logging needs IPC from worker threads back to UI
- Configuration reading (jake.autoDetect) requires settings provider interface

**Files referenced:**
- `extensions/jake/src/main.ts:332-339` - Extension activation/deactivation lifecycle
- `extensions/jake/package.json:24-26` - Activation event: `onTaskType:jake`
- `extensions/jake/package.json:33-69` - Contributed task definitions and configuration

