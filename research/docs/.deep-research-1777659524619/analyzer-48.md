### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/grunt/src/main.ts` — 365 LOC, full TypeScript extension implementation
- `/home/norinlavaee/projects/vscode-atomic/extensions/grunt/esbuild.mts` — 18 LOC, build configuration

---

### Per-File Notes

#### `extensions/grunt/esbuild.mts`

**Role:** Build script for the grunt extension.

- Line 6: Imports `run` from `'../esbuild-extension-common.mts'` — a shared esbuild runner used by multiple extensions.
- Lines 8–9: Resolves `srcDir` to `<ext-root>/src` and `outDir` to `<ext-root>/dist` using `import.meta.dirname`.
- Lines 11–18: Calls `run(...)` with `platform: 'node'`, a single entry point `main` → `src/main.ts`, and the resolved src/out dirs. `process.argv` is forwarded to allow CLI flags.

No business logic; purely delegates to the shared esbuild pipeline.

---

#### `extensions/grunt/src/main.ts`

**Role:** Implements a VS Code `TaskProvider` for Grunt, performing auto-detection of Grunt tasks by running `grunt --help --no-color` in each workspace folder containing a `Gruntfile.js`.

**Type Alias**

- Line 11: `type AutoDetect = 'on' | 'off'` — mirrors the `grunt.autoDetect` configuration setting.

**Interface**

- Lines 66–70: `GruntTaskDefinition extends vscode.TaskDefinition` — adds `task: string`, optional `args?: string[]`, and optional `file?: string` to the base definition.

**Utility Functions**

- Lines 13–18: `exists(file): Promise<boolean>` — wraps `fs.exists` as a promise.
- Lines 21–30: `exec(command, options): Promise<{stdout, stderr}>` — wraps `cp.exec` as a promise; rejects with `{error, stdout, stderr}` on non-zero exit.
- Lines 32–40: `isBuildTask(name): boolean` — returns `true` if `name` contains any of `['build', 'compile', 'watch']` (case-sensitive on raw name; callers pass lowercased).
- Lines 42–50: `isTestTask(name): boolean` — returns `true` if `name` contains `'test'`.
- Lines 52–57: `getOutputChannel(): vscode.OutputChannel` — lazily creates a singleton `OutputChannel` named `'Grunt Auto Detection'`.
- Lines 60–65: `showError()` — shows a warning notification with a "Go to output" action that reveals the output channel.

**`findGruntCommand(rootPath): Promise<string>` (lines 72–83)**

Resolves the grunt binary path for the current workspace folder:

1. On `win32`: checks for `node_modules/.bin/grunt.cmd`; if found, returns the relative path `./node_modules/.bin/grunt.cmd`.
2. On `linux`/`darwin`: checks for `node_modules/.bin/grunt`; if found, returns `./node_modules/.bin/grunt`.
3. Fallback: returns `'grunt'` (relies on system PATH).

**`class FolderDetector` (lines 85–228)**

Manages task detection for a single `vscode.WorkspaceFolder`.

- **Constructor** (lines 90–93): Accepts a `WorkspaceFolder` and a `Promise<string>` for the grunt command.
- **`isEnabled(): boolean`** (lines 99–101): Reads `grunt.autoDetect` config for the folder URI; returns `true` only if value is `'on'`.
- **`start(): void`** (lines 103–109): Creates a `FileSystemWatcher` on the glob pattern `{node_modules,[Gg]runtfile.js}` inside the folder. Any `change`, `create`, or `delete` event sets `this.promise = undefined`, invalidating the cached task list.
- **`getTasks(): Promise<vscode.Task[]>`** (lines 111–120): Returns cached `this.promise` if set; otherwise calls `computeTasks()` and caches the resulting promise.
- **`getTask(_task): Promise<vscode.Task | undefined>`** (lines 122–134): Reconstructs a single `vscode.Task` from a stored definition. If the task name contains a space (line 128–130), it wraps the name in double quotes in the shell command.
- **`computeTasks(): Promise<vscode.Task[]>`** (lines 136–220): The core detection routine:
  1. Lines 137–144: Guards — returns empty if `rootPath` is not a `file:` URI or if neither `gruntfile.js` nor `Gruntfile.js` exists.
  2. Line 146: Builds command `<gruntCommand> --help --no-color`.
  3. Lines 148–205: Calls `exec(commandLine, {cwd: rootPath})`. Parses `stdout` line by line:
     - Lines 167: Splits on `\r?\n`.
     - Lines 174–176: Scans for the sentinel line `'Available tasks'` to begin capture.
     - Lines 179–180: Stops capture on `'Tasks run in the order specified'`.
     - Lines 182–183: Within the capture region, applies regex `/^\s*(\S.*\S)  \S/g` to extract the task name (group 1) — this matches lines where the name is followed by two or more spaces then a non-space character (description separator).
     - Lines 186–195: Constructs a `GruntTaskDefinition` (`type: 'grunt'`, `task: name`) and a `vscode.Task` with `ShellExecution`. Names containing spaces are double-quoted in the shell string.
     - Lines 196–201: Assigns `task.group = vscode.TaskGroup.Build` if `isBuildTask(name.toLowerCase())`, or `task.group = vscode.TaskGroup.Test` if `isTestTask(name.toLowerCase())`.
  4. Lines 208–218: On exec failure, logs `stderr`, `stdout`, and a localised error message to the output channel, then calls `showError()`.
- **`dispose(): void`** (lines 222–227): Clears the cached promise and disposes the file watcher.

**`class TaskDetector` (lines 230–354)**

Orchestrates `FolderDetector` instances across all workspace folders and manages the `TaskProvider` registration lifecycle.

- **Fields** (lines 232–233): `taskProvider: vscode.Disposable | undefined` holds the registered provider token; `detectors: Map<string, FolderDetector>` keyed by folder URI string.
- **`start(): void`** (lines 238–245): Bootstraps detectors for current `workspaceFolders`, then subscribes to `onDidChangeWorkspaceFolders` and `onDidChangeConfiguration`.
- **`updateWorkspaceFolders(added, removed)`** (lines 255–271):
  - Disposes and removes detectors for removed folders.
  - Creates a new `FolderDetector` per added folder; starts it if `isEnabled()`.
  - Calls `updateProvider()`.
- **`updateConfiguration()`** (lines 273–291): Disposes all detectors, re-creates them from current `workspaceFolders`, restarts enabled ones, then calls `updateProvider()`.
- **`updateProvider()`** (lines 293–309):
  - If no provider exists and `detectors.size > 0`: registers a `TaskProvider` for type `'grunt'` via `vscode.tasks.registerTaskProvider` (line 296). The `provideTasks` callback calls `this.getTasks()`; `resolveTask` calls `this.getTask(_task)`.
  - If a provider exists and `detectors.size === 0`: disposes and unregisters it.
- **`computeTasks()`** (lines 315–335): Aggregates across detectors:
  - 0 detectors → resolves `[]`.
  - 1 detector → directly returns its `getTasks()` promise.
  - N detectors → `Promise.all(...)` across all detector `getTasks()` calls, with per-detector error swallowed to `[]`, flattening into one array.
- **`getTask(task)`** (lines 337–353): Resolves a single task by routing to the detector for `task.scope.uri`; returns `undefined` for `Workspace`/`Global` scope or unknown folder.

**Extension Lifecycle (lines 356–364)**

- `activate(_context)` (line 357): Creates a `TaskDetector` and calls `start()`. The `_context` is not used (subscriptions not tracked on it; the detector manages its own disposal).
- `deactivate()` (line 362): Calls `detector.dispose()`.

---

### Cross-Cutting Synthesis

The grunt extension implements a VS Code `TaskProvider` using a two-layer detector pattern. `TaskDetector` acts as a workspace-level orchestrator: it listens for folder and configuration changes to maintain a `Map<string, FolderDetector>` and conditionally registers or disposes the `vscode.tasks` provider registration token. Each `FolderDetector` operates at the folder level, watching for changes to `Gruntfile.js` or `node_modules` to invalidate a cached promise, then re-running `grunt --help --no-color` and parsing its stdout to enumerate tasks. Task group classification (`Build` vs `Test`) is done by substring matching on the lowercased task name against fixed keyword lists. The `findGruntCommand` helper provides platform-aware resolution of the grunt binary, preferring a locally installed version over the global PATH. The build configuration in `esbuild.mts` delegates entirely to a shared `esbuild-extension-common.mts` helper, bundling `src/main.ts` as a Node.js module into `dist/main.js`.

---

### Out-of-Partition References

- `extensions/esbuild-extension-common.mts` — imported at `esbuild.mts:6` as `'../esbuild-extension-common.mts'`; provides the shared `run()` build function used by all extensions.
- `vscode` API module — `vscode.TaskProvider`, `vscode.Task`, `vscode.ShellExecution`, `vscode.TaskGroup`, `vscode.FileSystemWatcher`, `vscode.OutputChannel`, `vscode.tasks.registerTaskProvider`, `vscode.workspace.*`, `vscode.window.*`, `vscode.l10n.t` — all consumed from the VS Code extension host API, not defined in this partition.
