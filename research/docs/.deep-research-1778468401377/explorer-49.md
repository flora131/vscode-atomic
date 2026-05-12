# Partition 49 of 80 — Findings

## Scope
`extensions/jake/` (2 files, 357 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locator: extensions/jake/ (Partition 49 of 80)

## Implementation

- `extensions/jake/src/main.ts` (340 LOC) - Jake task detection and VS Code integration
  - Exports `activate()` and `deactivate()` extension entry points
  - Contains `TaskDetector` class for managing Jake task providers
  - Contains `FolderDetector` class for detecting Jakefile in workspace folders
  - Utilities: `findJakeCommand()`, `isBuildTask()`, `isTestTask()`, `exists()`, `exec()`
  - Defines `JakeTaskDefinition` interface extending `vscode.TaskDefinition`

## Configuration

- `extensions/jake/package.json` - VS Code extension manifest
  - Name: `jake` (v10.0.0)
  - Activation event: `onTaskType:jake`
  - Configuration: `jake.autoDetect` setting (string enum: "off" | "on")
  - Task definition: `jake` type with required `task` property and optional `file` property
  - Build scripts: compile and watch gulp tasks

- `extensions/jake/tsconfig.json` - TypeScript compilation configuration
  - Extends `../tsconfig.base.json`
  - Source: `./src`, Output: `./out`
  - Includes vscode type definitions

- `extensions/jake/.npmrc` - NPM configuration file

- `extensions/jake/.vscodeignore` - Files to exclude from VS Code package

- `extensions/jake/esbuild.mts` - esbuild configuration for bundling
  - Entry point: `src/main.ts`
  - Output directory: `dist/`

## Documentation

- `extensions/jake/README.md` - Extension documentation
  - Describes Jake integration as bundled (cannot be uninstalled)
  - Features Jake task execution from Jakefile.js files
  - Settings documentation for auto-detection

## Examples / Fixtures

- `extensions/jake/images/cowboy_hat.png` - Extension icon

## Notable Clusters

- `extensions/jake/` - Contains 10 files total
  - 1 TypeScript source file (main.ts)
  - 4 configuration files (package.json, tsconfig.json, .npmrc, esbuild.mts)
  - 1 package lockfile (package-lock.json)
  - 1 localization file (package.nls.json)
  - 1 README
  - 1 ignore file (.vscodeignore)
  - 1 icon asset (cowboy_hat.png)

## Summary

The `extensions/jake/` directory implements a VS Code extension that provides task provider integration for Jake, a JavaScript build tool. The extension detects and runs Jake tasks (from Jakefile or Jakefile.js) as VS Code tasks, with configurable auto-detection via the `jake.autoDetect` setting. Build and test tasks are automatically categorized. The implementation spans a single TypeScript file (main.ts) with approximately 357 LOC (per scope specification) managing workspace folder detection, Jake command resolution, and task lifecycle through the VS Code task provider API. Configuration-heavy with standard Node/TypeScript tooling setup (esbuild + gulp compilation pipeline).

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/jake/src/main.ts` (340 LOC) — full runtime implementation
- `/home/norinlavaee/projects/vscode-atomic/extensions/jake/package.json` (75 LOC) — extension manifest and contribution points

---

### Per-File Notes

#### `extensions/jake/src/main.ts`

**Role:** Implements a VS Code task-provider extension that auto-detects Jake build tasks by shelling out to the `jake --tasks` command and registering them with the VS Code task system.

**Module-level symbols**

| Symbol | Line | Kind |
|---|---|---|
| `AutoDetect` | 11 | type alias (`'on' \| 'off'`) |
| `exists` | 13 | utility — wraps `fs.exists` in a Promise |
| `exec` | 21 | utility — wraps `cp.exec` in a Promise, rejects on error |
| `buildNames` | 32 | constant — `['build','compile','watch']` |
| `isBuildTask` | 33 | predicate — substring-checks a task name against `buildNames` |
| `testNames` | 42 | constant — `['test']` |
| `isTestTask` | 43 | predicate — substring-checks against `testNames` |
| `_channel` | 52 | module-level lazy `OutputChannel` singleton |
| `getOutputChannel` | 53 | creates/returns singleton `OutputChannel('Jake Auto Detection')` |
| `showError` | 60 | shows warning message with link to output channel |
| `findJakeCommand` | 67 | resolves the jake binary path for the given folder |
| `JakeTaskDefinition` | 80 | interface extending `vscode.TaskDefinition`; fields `task: string`, `file?: string` |
| `FolderDetector` | 85 | class — per-folder detection |
| `TaskDetector` | 204 | class — multi-folder orchestrator and task-provider registrant |
| `detector` | 331 | module-level singleton `TaskDetector` |
| `activate` | 332 | extension entry point |
| `deactivate` | 337 | extension teardown |

---

**`findJakeCommand(rootPath)` — lines 67–78**

Checks `process.platform` then probes for a local `node_modules/.bin/jake.cmd` (Windows) or `node_modules/.bin/jake` (Linux/macOS) via `exists`. Falls back to the bare string `'jake'` if neither is found. Returns a relative `./node_modules/.bin/jake[.cmd]` path or `'jake'`.

---

**`FolderDetector` class — lines 85–202**

Constructed with a `WorkspaceFolder` and a `Promise<string>` (the jake command). Two private fields:
- `fileWatcher: vscode.FileSystemWatcher | undefined` — watches `{node_modules,Jakefile,Jakefile.js}` in the folder root.
- `promise: Thenable<vscode.Task[]> | undefined` — cached result of the last `computeTasks` call.

`isEnabled()` (line 99): reads `jake.autoDetect` workspace configuration; returns `true` only when value is `'on'`.

`start()` (line 103): creates a `FileSystemWatcher` on pattern `{node_modules,Jakefile,Jakefile.js}`. Any `onDidChange`, `onDidCreate`, or `onDidDelete` event resets `this.promise = undefined`, invalidating the cache.

`getTasks()` (line 111): guards on `isEnabled()`, returns cached `this.promise` or calls `computeTasks()`.

`getTask(_task)` (line 122): resolves a single task by reading `_task.definition.task`, building a `ShellExecution` with the awaited jake command plus the task name, scoped to the workspace folder's `fsPath`.

`computeTasks()` (lines 133–194) — core logic:
1. Bails if `rootPath` is not a `file:` URI (line 134).
2. Checks for `Jakefile` then `Jakefile.js` via `exists`; returns empty array if neither present (lines 139–145).
3. Runs `${jakeCommand} --tasks` in a child process via `exec` (line 149).
4. Splits stdout on `/\r{0,1}\n/` (line 156).
5. For each non-empty line applies regex `/^jake\s+([^\s]+)\s/g` (line 161) to extract the task name from capture group 1.
6. Constructs a `JakeTaskDefinition` (`type:'jake'`, `task: taskName`) and a `vscode.Task` wrapping a `ShellExecution` of `${jakeCommand} ${taskName}` (lines 165–170).
7. Classifies the task: line (lowercased) checked with `isBuildTask` → `vscode.TaskGroup.Build`; with `isTestTask` → `vscode.TaskGroup.Test` (lines 172–177).
8. Errors: stderr is appended to the output channel (line 151); exceptions append `err.stderr`, `err.stdout`, and an error message, then call `showError()` (lines 183–193).

`dispose()` (line 196): clears `this.promise` and disposes the file watcher.

---

**`TaskDetector` class — lines 204–329**

Orchestrates one `FolderDetector` per workspace folder, keyed by `folder.uri.toString()` in the `detectors: Map<string, FolderDetector>`.

`start()` (line 212): seeds `detectors` with current `workspaceFolders`, then subscribes to `onDidChangeWorkspaceFolders` → `updateWorkspaceFolders` and `onDidChangeConfiguration` → `updateConfiguration`.

`updateWorkspaceFolders(added, removed)` (line 229): disposes and removes detectors for removed folders; for each added folder creates a `FolderDetector`, calls `detector.start()` if enabled, stores it in `detectors`. Calls `updateProvider()`.

`updateConfiguration()` (line 247): disposes all existing detectors, clears the map, re-creates a `FolderDetector` for every current workspace folder, starts enabled ones, calls `updateProvider()`.

`updateProvider()` (line 267): lazily registers `vscode.tasks.registerTaskProvider('jake', …)` when `detectors.size > 0` and no provider exists yet (line 270). Disposes the provider when `detectors` becomes empty (line 279). The registered provider object has two methods:
- `provideTasks()` → `thisCapture.getTasks()` (line 272)
- `resolveTask(_task)` → `thisCapture.getTask(_task)` (line 274)

`computeTasks()` (lines 289–309): fans out to all `FolderDetector.getTasks()` in parallel via `Promise.all`, flattens results.

`getTask(task)` (lines 311–328): for single-detector workspace, delegates directly. For multi-folder, requires `task.scope` to be a `WorkspaceFolder` (not `Global`/`Workspace`); looks up detector by `task.scope.uri.toString()`.

`activate` (line 332): creates the module-level `TaskDetector` singleton and calls `start()`.
`deactivate` (line 337): calls `detector.dispose()` which disposes the registered task-provider and all folder detectors.

---

#### `extensions/jake/package.json`

**Role:** Declares the extension identity, activation trigger, contributed configuration setting, and task definition schema.

**Key fields**

- `"main": "./out/main"` (line 23) — compiled output of `src/main.ts`.
- `"activationEvents": ["onTaskType:jake"]` (line 24–26) — extension activates only when VS Code encounters a task of type `jake`, keeping startup cost near zero.
- `"capabilities".virtualWorkspaces: false` (line 28) — explicitly opts out of virtual workspace support; `exec`/`fs` calls require real file access.
- `"capabilities".untrustedWorkspaces.supported: true` (line 29–31) — runs in untrusted workspaces; consistent with `autoDetect` defaulting to `"off"` in package.json line 46.
- `jake.autoDetect` setting (lines 39–48): application-scoped, enum `["off","on"]`, default `"off"`.
- `taskDefinitions[0]` (lines 51–69): type `"jake"`, required property `task` (string), optional property `file` (string), guarded by `when: "shellExecutionSupported"` — meaning the definition is only available in contexts where shell execution is possible.
- No runtime `dependencies` (line 19) — all I/O uses Node built-ins (`fs`, `cp`, `path`).

---

### Cross-Cutting Synthesis

The Jake extension implements a two-level auto-detection architecture. `TaskDetector` acts as a multi-folder coordinator: it tracks workspace folder changes and configuration changes, creating one `FolderDetector` per folder. Each `FolderDetector` is responsible for a single workspace folder; it caches the list of discovered tasks in `this.promise` and invalidates that cache whenever the file watcher detects a change to `Jakefile`, `Jakefile.js`, or `node_modules`. Discovery works by spawning `jake --tasks` as a child process and parsing each output line with the regex `/^jake\s+([^\s]+)\s/g` to extract task names. Task group classification (`Build` vs `Test`) is done via substring matching of the raw output line against fixed keyword arrays. The provider is lazily registered with `vscode.tasks.registerTaskProvider` only when at least one detector is active, and is disposed when all folders are removed. The entire flow is gated by the `jake.autoDetect` setting, which defaults to `"off"`, and the `onTaskType:jake` activation event ensures the extension does not load at all unless a Jake task is actually requested.

For a Tauri/Rust port, the critical cross-process boundary is `cp.exec` (main.ts:21) and `fs.exists` (main.ts:13), which replace the VS Code FileSystem API with direct Node.js syscalls. The `vscode.tasks.registerTaskProvider` contract (main.ts:270), `vscode.workspace.createFileSystemWatcher` (main.ts:105), `vscode.workspace.onDidChangeWorkspaceFolders` (main.ts:217), and `vscode.workspace.onDidChangeConfiguration` (main.ts:218) are all VS Code-specific APIs that must be re-expressed in a Tauri equivalent.

---

### Out-of-Partition References

- `vscode.TaskDefinition`, `vscode.Task`, `vscode.ShellExecution`, `vscode.ShellExecutionOptions`, `vscode.TaskGroup`, `vscode.tasks.registerTaskProvider` — core VS Code task API; defined in the VS Code extension host, not in this partition.
- `vscode.workspace.createFileSystemWatcher` — VS Code file-watching API.
- `vscode.workspace.getConfiguration` — configuration service.
- `vscode.window.createOutputChannel`, `vscode.window.showWarningMessage` — UI API.
- `vscode.l10n.t` — localisation API; string resources referenced as `%description%`, `%displayName%`, `%config.jake.autoDetect%`, `%jake.taskDefinition.type.description%`, `%jake.taskDefinition.file.description%` in `package.json` would resolve via the VS Code NLS system (outside this partition).
- The gulp build target `compile-extension:jake` is defined in the root `gulpfile.js` (outside this partition).

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
