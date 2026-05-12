# Partition 46 of 80 — Findings

## Scope
`extensions/gulp/` (2 files, 424 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
## Implementation
- `extensions/gulp/src/main.ts` — Gulp task detection and provider for VS Code; relies on vscode API and child_process to execute gulp commands

## Configuration
- `extensions/gulp/package.json` — Extension manifest with task definitions, gulp configuration properties, and auto-detect settings
- `extensions/gulp/esbuild.mts` — Build configuration for bundling the gulp extension
- `extensions/gulp/tsconfig.json` — TypeScript compilation configuration

## Documentation
- `extensions/gulp/README.md` — User-facing documentation describing Gulp task support and auto-detection feature

### Notable Clusters
- `extensions/gulp/` — 2 source files (1 TS implementation + 1 build config), 5 config/metadata files, 1 image asset. A lightweight task provider extension that relies on VS Code's extension API and Node.js child_process module.

## Summary
The gulp extension is a narrow, single-purpose task provider that integrates Gulp build automation into VS Code's task system. It contains only minimal TypeScript code (~400 LOC) focused on detecting gulpfile entries and executing gulp commands via shell execution. The implementation relies heavily on the VS Code Extension API (vscode.tasks, vscode.workspace, vscode.window) and Node.js built-ins (child_process, fs, path). For a Tauri/Rust port, this extension would need significant architectural changes since it depends on dynamic task detection via shell invocation and the VS Code extension system—neither of which directly translate to a Rust-based IDE framework. The extension model itself would require reimplementation in Rust or a different plugin architecture entirely.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/gulp/src/main.ts` (407 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/gulp/package.json` (75 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/gulp/esbuild.mts` (18 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/gulp/tsconfig.json` (17 lines)

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/extensions/gulp/src/main.ts`

- **Role:** Core runtime logic for the Gulp task auto-detection extension. Implements two classes (`FolderDetector` and `TaskDetector`) and exposes the VS Code extension lifecycle hooks (`activate`/`deactivate`).

- **Key symbols:**
  - `exists(filename: string): Promise<boolean>` — lines 26–39. Async wrapper around `fs.promises.stat`, returns `false` on error or non-file.
  - `exec(command: string, options: cp.ExecOptions)` — lines 41–50. Promisifies `child_process.exec`, rejecting with `{ error, stdout, stderr }` on non-zero exit.
  - `findGulpCommand(rootPath: string): Promise<string>` — lines 89–107. Platform-aware resolution: on Win32 checks `node_modules/.bin/gulp.cmd` then global `%APPDATA%/npm/gulp.cmd`; on Linux/Darwin checks `node_modules/.bin/gulp`; falls back to bare `gulp`.
  - `isBuildTask(name: string): boolean` — lines 53–60. Returns true if task name contains any of `['build', 'compile', 'watch']`.
  - `isTestTask(name: string): boolean` — lines 62–70. Returns true if task name contains `'test'`.
  - `GulpTaskDefinition` interface — lines 109–112. Extends `vscode.TaskDefinition` with required `task: string` and optional `file?: string`.
  - `FolderDetector` class — lines 114–269. Per-workspace-folder controller: holds a `fileWatcher` and a cached `promise` of tasks.
  - `TaskDetector` class — lines 271–396. Top-level coordinator: maintains a `Map<string, FolderDetector>` keyed by folder URI string.
  - `activate(_context: vscode.ExtensionContext)` — lines 399–402. Creates a `TaskDetector`, calls `start()`.
  - `deactivate()` — lines 404–406. Calls `detector.dispose()`.

- **Control flow:**
  1. On activation (`activate`, line 399), a `TaskDetector` is instantiated and `start()` called.
  2. `TaskDetector.start()` (line 279) reads `vscode.workspace.workspaceFolders`, calls `updateWorkspaceFolders(folders, [])`, and registers listeners on `onDidChangeWorkspaceFolders` (line 284) and `onDidChangeConfiguration` (line 285).
  3. `updateWorkspaceFolders` (line 296) disposes removed folders' detectors, creates new `FolderDetector` instances for added folders, starts each if enabled (`isEnabled()` checks `gulp.autoDetect === 'on'` at line 129).
  4. `updateProvider` (line 334) registers (or unregisters) a `vscode.tasks.registerTaskProvider('gulp', ...)` (line 337) depending on whether any detectors are active.
  5. When VS Code requests tasks, `provideTasks()` calls `TaskDetector.getTasks()` → `computeTasks()` (line 356), which fans out to per-folder `FolderDetector.getTasks()`.
  6. `FolderDetector.getTasks()` (line 140) returns cached `this.promise` or calls `computeTasks()` (line 203).
  7. `FolderDetector.computeTasks()` (line 203): verifies the root path is a `file:` URI, checks for a gulpfile via `hasGulpfile()` (line 175), then spawns `gulp --tasks-simple --no-color` via `exec` (line 216), parses each line of stdout as a task name, constructs `vscode.Task` with `vscode.ShellExecution` (line 238), and assigns `task.group` to `vscode.TaskGroup.Build` or `vscode.TaskGroup.Test` based on name heuristics (lines 240–245).
  8. `FolderDetector.start()` (line 132) sets up a `FileSystemWatcher` on the pattern `{node_modules,gulpfile{.babel.js,.esm.js,.js,.mjs,.cjs,.ts}}` (line 133); any change/create/delete event clears `this.promise` (lines 135–137), forcing re-detection on next request.
  9. `FolderDetector.hasGulpfile(root)` (line 175) reads the directory listing, checks for files whose basename (case-insensitive) equals `gulpfile`, `gulpfile.esm`, or `gulpfile.babel` with extensions `.js`, `.mjs`, `.cjs`, or `.ts`.

- **Data flow:**
  - Entry: `vscode.workspace.workspaceFolders` → `FolderDetector` per folder.
  - Resolution: filesystem `readdir` + `stat` to find gulpfile; `child_process.exec` to enumerate tasks via gulp CLI stdout.
  - Transformation: raw stdout lines split on `\r?\n` → `GulpTaskDefinition` objects → `vscode.Task` instances with `vscode.ShellExecution`.
  - Output: array of `vscode.Task[]` returned to VS Code task subsystem via `provideTasks` callback.
  - Side effects: stderr lines filtered and written to a lazily created `vscode.OutputChannel` ('Gulp Auto Detection', line 75); warning toast shown via `vscode.window.showWarningMessage` (line 81).

- **Dependencies:**
  - Node built-ins: `path`, `fs` (fs.promises), `child_process` (cp.exec).
  - VS Code API: `vscode.workspace`, `vscode.window`, `vscode.tasks`, `vscode.TaskGroup`, `vscode.Task`, `vscode.ShellExecution`, `vscode.FileSystemWatcher`, `vscode.TaskDefinition`, `vscode.l10n`.
  - No npm runtime dependencies (package.json `"dependencies": {}`).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/gulp/package.json`

- **Role:** Extension manifest declaring activation, capability, and contribution points consumed by the VS Code extension host.

- **Key symbols:**
  - `"activationEvents": ["onTaskType:gulp"]` (line 24–26): Extension activates lazily only when VS Code encounters a task of type `gulp`.
  - `"capabilities".virtualWorkspaces: false` (line 28): Extension explicitly opts out of virtual workspace support; requires a real filesystem to run `child_process.exec`.
  - `"capabilities".untrustedWorkspaces.supported: true` (line 29–31): Extension is permitted in untrusted workspaces.
  - `"gulp.autoDetect"` configuration property (lines 39–49): `application`-scoped string setting, enum `['off', 'on']`, default `'off'`. Controls per-folder auto-detection toggle read in `FolderDetector.isEnabled()`.
  - `"taskDefinitions"` entry (lines 51–69): Declares the `gulp` task type schema. The `task` property is required; `file` is optional. `"when": "shellExecutionSupported"` (line 67) gates the definition on shell execution capability.
  - `"main": "./out/main"` (line 23): Points to the compiled CommonJS output.

- **Control flow:** Declarative manifest only; consumed by VS Code extension host at load time.

- **Data flow:** Configuration values from `gulp.autoDetect` flow into `FolderDetector.isEnabled()` at runtime. The `taskDefinitions` schema drives validation when tasks are serialized in `tasks.json`.

- **Dependencies:** No runtime npm dependencies. Dev dependency: `@types/node` 22.x. Build scripts reference the workspace-level `gulp compile-extension:gulp` task.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/gulp/esbuild.mts`

- **Role:** Build script that bundles `src/main.ts` into `dist/main.js` using a shared esbuild runner.

- **Key symbols:**
  - `run(...)` imported from `../esbuild-extension-common.mts` (line 6): shared runner accepting a config object and `process.argv`.
  - Entry point: `{ main: path.join(srcDir, 'main.ts') }` (lines 13–15).
  - Output directory: `dist/` (line 17), distinct from TypeScript compiler output at `out/` (per tsconfig.json).
  - Platform: `'node'` (line 12), so esbuild targets Node.js CommonJS module format.

- **Control flow:** Single `run(config, process.argv)` call. The shared runner in `esbuild-extension-common.mts` (out of partition) handles watch mode vs. one-shot build based on argv.

- **Data flow:** Source TypeScript from `src/main.ts` → esbuild bundler → `dist/main.js`.

- **Dependencies:** `node:path` built-in; `../esbuild-extension-common.mts` (sibling utility, out of partition).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/gulp/tsconfig.json`

- **Role:** TypeScript compilation configuration for the extension source.

- **Key symbols:**
  - `"extends": "../tsconfig.base.json"` (line 2): Inherits shared strictness and target settings.
  - `"rootDir": "./src"` / `"outDir": "./out"` (lines 4–5): Source-to-output mapping.
  - `"types": ["node"]` (line 6): Only Node.js type declarations are injected; no DOM types.
  - `"include"` (lines 13–16): Includes `src/**/*` plus `../../src/vscode-dts/vscode.d.ts` — the VS Code public API type declarations from the main repo.

- **Control flow:** Consumed by `tsc` at compile time; not executed at runtime.

- **Data flow:** The explicit inclusion of `vscode.d.ts` from `../../src/vscode-dts/` makes VS Code API types available without installing the `@vscode/types` npm package, relying on the monorepo file layout.

- **Dependencies:** `../tsconfig.base.json` (out of partition); `../../src/vscode-dts/vscode.d.ts` (VS Code API declarations, out of partition).

---

### Cross-Cutting Synthesis

The gulp extension exemplifies the canonical VS Code task-provider extension pattern. A single TypeScript file (`src/main.ts`) implements the full runtime: lazy activation via `onTaskType:gulp`, per-folder `FolderDetector` instances that shell out to the `gulp --tasks-simple` CLI using `child_process.exec`, and a `FileSystemWatcher` cache-invalidation scheme so task lists stay fresh without polling. The only VS Code API surface consumed is the task, workspace, window, and filesystem-watcher APIs — no language server protocol, debugger, or editor APIs.

For a Tauri/Rust port, the relevant insight is that this extension is essentially a subprocess launcher and stdout parser wrapped in VS Code's task-provider contract. The `child_process.exec` calls and `fs.promises` filesystem access are Node.js platform primitives. In a Tauri context these would map to Rust's `std::process::Command` and `std::fs`, respectively, while the task-provider registration contract (`vscode.tasks.registerTaskProvider`) has no direct Tauri equivalent and would require designing a new plugin or IPC boundary. The `virtualWorkspaces: false` capability flag further confirms the extension assumes a real OS filesystem, which aligns with Tauri's desktop-first model but means any virtual/remote filesystem support would need explicit re-engineering.

---

### Out-of-Partition References

- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-extension-common.mts` — shared esbuild runner imported by `esbuild.mts:6`.
- `/home/norinlavaee/projects/vscode-atomic/extensions/tsconfig.base.json` — base TypeScript config extended by `tsconfig.json:2`.
- `/home/norinlavaee/projects/vscode-atomic/src/vscode-dts/vscode.d.ts` — VS Code public API type declarations included in `tsconfig.json:15`; defines `vscode.Task`, `vscode.ShellExecution`, `vscode.TaskGroup`, `vscode.FileSystemWatcher`, etc.
- VS Code task subsystem internals (not in this partition) — the `vscode.tasks.registerTaskProvider` API consumed at `main.ts:337` is implemented within VS Code core (`src/vs/workbench/api/...`).

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
