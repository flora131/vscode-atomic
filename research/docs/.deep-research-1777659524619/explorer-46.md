# Partition 46 of 79 — Findings

## Scope
`extensions/gulp/` (2 files, 424 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locator Report: Partition 46 — extensions/gulp/

## Research Question
What it would take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

## Scope
`extensions/gulp/` (2 files, 424 LOC)

---

## Implementation

- `extensions/gulp/src/main.ts` — Main extension entry point implementing TaskProvider registration via `vscode.tasks.registerTaskProvider('gulp', ...)`. Handles gulp task discovery, execution, and integration with VS Code's task system. Contains utilities for detecting build/test tasks and spawning child processes to execute gulp commands.

- `extensions/gulp/esbuild.mts` — Build configuration for bundling the extension.

---

## Configuration

- `extensions/gulp/package.json` — Extension manifest defining activation events (`onTaskType:gulp`), taskDefinitions for gulp with properties (task, file), and configuration schema for `gulp.autoDetect` setting.

- `extensions/gulp/tsconfig.json` — TypeScript compilation configuration extending base config, targeting Node types.

- `extensions/gulp/package-lock.json` — Dependency lock file.

- `extensions/gulp/.npmrc` — NPM configuration.

- `extensions/gulp/.vscodeignore` — Extension packaging exclusions.

---

## Documentation

- `extensions/gulp/README.md` — User-facing documentation describing gulp task auto-detection and integration with VS Code's task system.

---

## Localization

- `extensions/gulp/package.nls.json` — Localization strings for UI labels.

---

## Images

- `extensions/gulp/images/gulp.png` — Gulp logo asset.

---

## Notable Clusters

**Single source file**: The extension is minimal—one TypeScript implementation file (~350 LOC) handling the entire gulp integration via the TaskProvider API.

**Extension architecture pattern**: 
- Activation: triggered on `onTaskType:gulp`
- Contribution: task definition schema registration
- Runtime: registers task provider callback that discovers and executes gulp tasks

---

## Relevance to Porting Question

This extension exemplifies **extension API integration points** that would require porting in a Tauri/Rust rewrite:

1. **TaskProvider API** (`vscode.tasks.registerTaskProvider`) — Must be replicated in Rust to support task discovery and execution.
2. **Child process spawning** (`cp.exec`) — Build tools like gulp need subprocess integration; Rust equivalent would use `tokio::process::Command` or similar.
3. **File system operations** (`fs.promises.stat`, `path.join`) — Core I/O abstractions needed for gulpfile detection and workspace navigation.
4. **Configuration system** (`gulp.autoDetect` setting) — Configuration infrastructure must support extension-defined settings.
5. **Output channels** (`vscode.window.createOutputChannel`) — Logging/output infrastructure for task execution feedback.
6. **Localization system** (`vscode.l10n.t`) — i18n support must be maintained.

The gulp extension is a **task automation** subsystem—one of the core IDE functionalities mentioned in the research question. Porting would require Rust equivalents for extension registration, task scheduling, subprocess management, and IDE-to-extension communication protocols.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `extensions/gulp/src/main.ts` — 407 LOC: the full runtime extension implementing gulp task auto-detection for VS Code
- `extensions/gulp/esbuild.mts` — 18 LOC: build script that bundles `main.ts` into `dist/` using the shared esbuild helper

---

### Per-File Notes

#### `extensions/gulp/src/main.ts`

- **Role:** A VS Code extension that registers a task provider for gulp. On activation it scans every workspace folder for a `gulpfile.*`, runs `gulp --tasks-simple --no-color` as a child process to enumerate tasks, then surfaces them to VS Code's task system. It also watches for gulpfile changes so the task list is refreshed on demand.

- **Key symbols:**
  - `AutoDetect` type alias (`main.ts:12`) — `'on' | 'off'` union used to represent the `gulp.autoDetect` configuration value.
  - `exists(filename)` (`main.ts:26`) — async helper that calls `fs.promises.stat` and returns `true` only when the path resolves to a regular file.
  - `exec(command, options)` (`main.ts:41`) — wraps `child_process.exec` in a `Promise`, resolving to `{ stdout, stderr }` on success and rejecting with `{ error, stdout, stderr }` on failure.
  - `buildNames` / `isBuildTask(name)` (`main.ts:52–60`) — `['build', 'compile', 'watch']`; checks via `indexOf` whether a task name contains any build-related token; used to assign `vscode.TaskGroup.Build`.
  - `testNames` / `isTestTask(name)` (`main.ts:62–70`) — mirrors `isBuildTask` for `['test']`; assigns `vscode.TaskGroup.Test`.
  - `_channel` / `getOutputChannel()` (`main.ts:72–78`) — lazy singleton `vscode.OutputChannel` named `'Gulp Auto Detection'`.
  - `showError()` (`main.ts:80–87`) — displays a warning toast with a `"Go to output"` action that reveals the output channel.
  - `findGulpCommand(rootPath)` (`main.ts:89–107`) — platform-aware resolution: on `win32` checks `node_modules/.bin/gulp.cmd` then `%APPDATA%/npm/gulp.cmd`; on `linux`/`darwin` checks `node_modules/.bin/gulp`; falls back to bare `gulp`.
  - `GulpTaskDefinition` interface (`main.ts:109–112`) — extends `vscode.TaskDefinition` with `task: string` and optional `file?: string`.
  - `FolderDetector` class (`main.ts:114–269`) — per-workspace-folder unit. Holds a `vscode.FileSystemWatcher` and a lazily-resolved `promise: Thenable<vscode.Task[]>`.
    - `start()` (`main.ts:132–138`) — creates a glob watcher for `{node_modules,gulpfile{.babel.js,.esm.js,.js,.mjs,.cjs,.ts}}`; any `onDidChange/Create/Delete` event nulls out `this.promise` to force re-detection.
    - `getTasks()` (`main.ts:140–150`) — returns cached `this.promise`, or calls `computeTasks()` if cache is absent.
    - `getTask(_task)` (`main.ts:152–161`) — resolves a single named task by constructing a `vscode.ShellExecution` from the gulp command and the task name.
    - `hasGulpfile(root)` (`main.ts:175–201`) — reads the directory, filters for `.js/.mjs/.cjs/.ts` extensions, and checks for basenames `gulpfile`, `gulpfile.esm`, or `gulpfile.babel`.
    - `computeTasks()` (`main.ts:203–261`) — the core logic: validates the root is a `file://` URI, calls `hasGulpfile`, assembles `gulp --tasks-simple --no-color` command line, executes it, splits `stdout` on newlines, and for each non-empty line creates a `vscode.Task` with a `vscode.ShellExecution`; stderr is logged unless every line contains `'No license field'`.
    - `dispose()` (`main.ts:263–268`) — clears the cached promise and disposes the file watcher.
  - `TaskDetector` class (`main.ts:271–396`) — top-level orchestrator. Owns a `Map<string, FolderDetector>` keyed by folder URI string and a single registered `vscode.Disposable` task provider.
    - `start()` (`main.ts:279–286`) — seeds detectors for current workspace folders; subscribes to `onDidChangeWorkspaceFolders` and `onDidChangeConfiguration`.
    - `updateWorkspaceFolders(added, removed)` (`main.ts:296–312`) — disposes removed detectors, creates new `FolderDetector` instances for added folders; calls `updateProvider()`.
    - `updateConfiguration()` (`main.ts:314–332`) — tears down all existing detectors and rebuilds them; handles the `gulp.autoDetect` config toggle.
    - `updateProvider()` (`main.ts:334–350`) — registers or unregisters the gulp task provider with `vscode.tasks.registerTaskProvider('gulp', ...)` depending on whether any detectors exist.
    - `computeTasks()` (`main.ts:356–376`) — aggregates tasks from all `FolderDetector` instances using `Promise.all`.
    - `getTask(task)` (`main.ts:378–395`) — routes a task-resolve request to the correct `FolderDetector` by matching `task.scope.uri`.
  - `activate(_context)` (`main.ts:399–402`) — extension entry point; creates a `TaskDetector` and calls `start()`.
  - `deactivate()` (`main.ts:404–406`) — calls `detector.dispose()`.

- **Control flow:**
  1. VS Code activates the extension on `onTaskType:gulp` (`package.json:25`).
  2. `activate` → `TaskDetector.start()` → `updateWorkspaceFolders(folders, [])`.
  3. For each folder a `FolderDetector` is created; if `gulp.autoDetect === 'on'`, `FolderDetector.start()` is called to begin watching.
  4. `updateProvider()` registers the task provider exactly once (when `detectors.size > 0`).
  5. When VS Code requests tasks, `TaskDetector.getTasks()` → `computeTasks()` fans out to `FolderDetector.getTasks()` per folder.
  6. `FolderDetector.getTasks()` checks and returns `this.promise`; if null it calls `computeTasks()` which spawns the `gulp` subprocess.
  7. File watcher events reset `this.promise = undefined`, causing re-execution on the next `getTasks()` call.
  8. Task group assignment (`Build`/`Test`) happens inside `computeTasks()` at `main.ts:241–245`.
  9. On configuration change, `updateConfiguration()` fully rebuilds all detectors.

- **Data flow:**
  - `findGulpCommand(rootPath): Promise<string>` → stored as constructor argument in `FolderDetector._gulpCommand`.
  - `computeTasks()` awaits `this._gulpCommand` to build the command string, passes it to `exec()`, parses `stdout` line by line into `GulpTaskDefinition` objects, and wraps each in a `vscode.Task` with a `vscode.ShellExecution`.
  - `vscode.Task` objects flow from `FolderDetector.computeTasks()` → `FolderDetector.getTasks()` → `TaskDetector.computeTasks()` → the registered task provider's `provideTasks()` callback → VS Code's task system.
  - Errors flow to `_channel` (output) and surface to the user via `showError()` warning toast.

- **Dependencies:**
  - Node built-ins: `path`, `fs` (async `promises.stat`, `promises.readdir`), `child_process` (`exec`).
  - VS Code extension API: `vscode.workspace`, `vscode.tasks`, `vscode.window`, `vscode.l10n`, `vscode.Task`, `vscode.ShellExecution`, `vscode.TaskGroup`, `vscode.FileSystemWatcher`, `vscode.TaskDefinition`.
  - No npm runtime dependencies (per `package.json:19`).

---

#### `extensions/gulp/esbuild.mts`

- **Role:** Build entry point for the gulp extension. It invokes the shared `run` helper from the monorepo's common esbuild script to bundle `src/main.ts` into `dist/main.js` for the Node.js environment.

- **Key symbols:**
  - `srcDir` (`esbuild.mts:8`) — absolute path to `extensions/gulp/src`, computed via `import.meta.dirname`.
  - `outDir` (`esbuild.mts:9`) — absolute path to `extensions/gulp/dist`.
  - `run(config, args)` (`esbuild.mts:11–18`) — imported from `../esbuild-extension-common.mts`; called with `platform: 'node'`, a single entry point mapping `'main'` → `src/main.ts`, and the `dist/` output directory.

- **Control flow:** Module-level imperative: resolve paths, call `run`. No conditional logic; `process.argv` is forwarded to `run` to allow watch-mode flags.

- **Data flow:** `import.meta.dirname` → string concatenation via `path.join` → `{ platform, entryPoints, srcDir, outdir }` config object → `run()` (external). Output artifact is `dist/main.js`.

- **Dependencies:**
  - Node built-in: `node:path`.
  - Monorepo-internal: `../esbuild-extension-common.mts` (out-of-partition).

---

### Cross-Cutting Synthesis

The `extensions/gulp` partition is a focused VS Code task-provider extension consisting of exactly two files. `esbuild.mts` is purely a thin build harness: it delegates to a shared monorepo helper to produce the distributable `dist/main.js` bundle from `src/main.ts`. All substantive logic lives in `main.ts`, which implements the two-class pattern (`FolderDetector` + `TaskDetector`) common to VS Code's built-in task-detection extensions (npm, grunt, jake, etc.). The extension relies entirely on spawning a `gulp` child process and parsing its stdout; there is no Gulp API import or in-process Gulp execution. The `gulp.autoDetect` configuration setting defaults to `'off'` (per `package.json:46`) and acts as the main gate on whether watchers and process invocations ever occur. For a Tauri/Rust port, this extension would need to be reimplemented as a Tauri plugin or sidecar process: the child-process spawning and filesystem watching map to Rust `std::process::Command` and `notify`-crate watching respectively, but the `vscode.Task` / `vscode.tasks.registerTaskProvider` surface has no Tauri equivalent and would require a new task-system abstraction.

---

### Out-of-Partition References

- `extensions/esbuild-extension-common.mts` — imported by `esbuild.mts:6` as `../esbuild-extension-common.mts`; provides the `run()` build helper used by all built-in extensions.
- `extensions/gulp/package.json` — declares activation event `onTaskType:gulp`, the `gulp.autoDetect` configuration contribution, and the `gulp` task definition schema; read for context but not a `.ts` source file.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
