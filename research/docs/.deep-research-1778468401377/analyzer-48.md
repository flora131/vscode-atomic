### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/grunt/src/main.ts` (365 LOC, full read)
- `/home/norinlavaee/projects/vscode-atomic/extensions/grunt/package.json` (80 LOC, full read)

---

### Per-File Notes

#### `extensions/grunt/src/main.ts`

**Role:** Implements the VS Code extension entry point for Grunt task auto-detection. Provides a `TaskProvider` registered with the VS Code task system under the `'grunt'` type.

**Top-level Symbols:**

- `AutoDetect` (type alias, line 11): `'on' | 'off'` — mirrors the `grunt.autoDetect` config value.
- `exists(file)` (line 13–19): Wraps `fs.exists` in a `Promise<boolean>`.
- `exec(command, options)` (line 21–30): Wraps `cp.exec` in a `Promise<{stdout, stderr}>`, rejecting on error with `{error, stdout, stderr}`.
- `buildNames` / `isBuildTask(name)` (lines 32–40): Tests if a task name substring-matches any of `['build', 'compile', 'watch']`.
- `testNames` / `isTestTask(name)` (lines 42–50): Tests if a task name substring-matches `['test']`.
- `_channel` / `getOutputChannel()` (lines 52–57): Lazily creates and returns a singleton `vscode.OutputChannel` named `'Grunt Auto Detection'`.
- `showError()` (lines 60–65): Shows a warning message with a button to open the output channel.
- `GruntTaskDefinition` (interface, lines 66–70): Extends `vscode.TaskDefinition` with `task: string`, optional `args?: string[]`, optional `file?: string`.
- `findGruntCommand(rootPath)` (lines 72–83): Platform-aware function that resolves the grunt binary path. On `win32`, checks for `node_modules/.bin/grunt.cmd`; on `linux`/`darwin`, checks for `node_modules/.bin/grunt`; otherwise falls back to the global `'grunt'` string.
- `FolderDetector` class (lines 85–228): Per-workspace-folder detector.
- `TaskDetector` class (lines 230–354): Orchestrates multiple `FolderDetector` instances.
- `activate(_context)` (lines 357–360): Extension entry point; instantiates and starts a `TaskDetector`.
- `deactivate()` (lines 362–364): Calls `detector.dispose()`.

**`FolderDetector` (lines 85–228):**

- Constructor (line 90–93): Accepts a `vscode.WorkspaceFolder` and a `Promise<string>` for the grunt command.
- `isEnabled()` (line 99–101): Reads `grunt.autoDetect` workspace config; returns `true` when equal to `'on'`.
- `start()` (lines 103–109): Registers a `FileSystemWatcher` on the glob `{node_modules,[Gg]runtfile.js}` within the workspace folder root. On any watcher event (change/create/delete), sets `this.promise = undefined` to invalidate the task cache.
- `getTasks()` (lines 111–120): Returns cached `this.promise` if already computed; otherwise calls `computeTasks()` and caches the result. Returns `[]` if not enabled.
- `getTask(_task)` (lines 122–134): Reconstructs a single `vscode.Task` for an existing `TaskDefinition` using `vscode.ShellExecution`. Quotes task names containing spaces.
- `computeTasks()` (lines 136–220): The core discovery logic.
  1. Verifies the folder is a `file://` URI (line 137–141).
  2. Checks for existence of `gruntfile.js` or `Gruntfile.js` (line 142–144).
  3. Runs `grunt --help --no-color` via the promisified `exec()` (line 146–148).
  4. Parses stdout line by line (line 167): splits on `\r?\n`, looks for the `'Available tasks'` sentinel to start scanning (line 175), and `'Tasks run in the order specified'` to stop (line 179).
  5. For each task line, applies regex `/^\s*(\S.*\S)  \S/g` (line 182) to extract the task name.
  6. Builds a `vscode.Task` with `vscode.ShellExecution` for each name (lines 192–194), quoting names with spaces.
  7. Assigns `task.group = vscode.TaskGroup.Build` or `.Test` based on `isBuildTask` / `isTestTask` (lines 197–201).
  8. On error, logs stderr/stdout to the output channel and calls `showError()` (lines 208–219).
- `dispose()` (lines 222–227): Clears `this.promise` and disposes the file watcher.

**`TaskDetector` (lines 230–354):**

- `detectors: Map<string, FolderDetector>` (line 233): Keyed by workspace folder URI string.
- `start()` (lines 238–245): Iterates current `vscode.workspace.workspaceFolders`, calls `updateWorkspaceFolders`. Subscribes to `onDidChangeWorkspaceFolders` and `onDidChangeConfiguration`.
- `updateWorkspaceFolders(added, removed)` (lines 255–271): Disposes detectors for removed folders; creates new `FolderDetector` instances for added folders (calling `findGruntCommand`). Conditionally calls `detector.start()` when enabled. Calls `updateProvider()`.
- `updateConfiguration()` (lines 273–291): Disposes and re-creates all detectors. Called on any VS Code configuration change. Calls `updateProvider()`.
- `updateProvider()` (lines 293–309): Lazily registers `vscode.tasks.registerTaskProvider('grunt', {...})` when `detectors.size > 0`; disposes the registration when size drops to 0. The registered provider object implements `provideTasks()` and `resolveTask()` by delegating to `getTasks()` / `getTask()`.
- `computeTasks()` (lines 315–335): When one detector exists, delegates directly. When multiple exist, calls all in parallel via `Promise.all`, then flattens results into a single array.
- `getTask(task)` (lines 337–353): When multiple detectors exist, routes by `task.scope` URI to find the right `FolderDetector`; returns `undefined` for global/workspace scoped tasks.

---

#### `extensions/grunt/package.json`

**Role:** Extension manifest declaring activation events, contributed configuration, and task definitions.

**Key fields:**

- `"activationEvents": ["onTaskType:grunt"]` (line 26–28): The extension activates only when VS Code encounters a task of type `grunt`, keeping startup impact minimal.
- `"capabilities".virtualWorkspaces: false` (line 29): Explicitly opts out of virtual workspace support.
- `"contributes".configuration` (lines 35–50): Adds `grunt.autoDetect` setting scoped to `"application"` with `enum: ["off", "on"]` and default `"off"`.
- `"contributes".taskDefinitions` (lines 52–74): Registers the `grunt` task type schema with required property `task` (string), optional `args` (array), optional `file` (string). The definition is gated on `"when": "shellExecutionSupported"`.
- `"main": "./out/main"` (line 24): Points to compiled output; the TypeScript source is compiled (not bundled via esbuild in this manifest).
- No runtime `dependencies` (line 18); only `@types/node` as a dev dependency.

---

### Cross-Cutting Synthesis

The grunt extension implements the VS Code task provider pattern across two cooperating classes. `TaskDetector` acts as the multi-workspace coordinator: it maintains one `FolderDetector` per workspace folder (keyed by URI string), responds to folder and configuration change events, and lazily registers or unregisters the single `vscode.tasks.registerTaskProvider` call as the detector count changes. `FolderDetector` handles per-folder logic: it watches the filesystem for changes to `Gruntfile.js` / `node_modules` to invalidate a promise-cached task list, and when `computeTasks()` is called it shells out to `grunt --help --no-color` via `child_process.exec`, parses the stdout between the `'Available tasks'` and `'Tasks run in the order specified'` sentinels using a two-space-separator regex, and constructs `vscode.Task` objects backed by `vscode.ShellExecution`. Task group assignment (`Build` vs. `Test`) is done by substring-matching the lowercased task name against known build/test name prefixes. The extension defaults to `autoDetect: off`, activating only on `onTaskType:grunt`.

For a Tauri/Rust port, the core equivalents needed are: a file-system watcher (e.g., `notify` crate), async process execution (`tokio::process::Command`), workspace folder tracking, and a task provider API that maps to whatever task abstraction the Rust host exposes. The stdout parsing logic (line 167–205 of `main.ts`) is purely algorithmic and is directly portable to Rust string processing.

---

### Out-of-Partition References

- `vscode.tasks.registerTaskProvider` — VS Code extension host API; implemented in `src/vs/workbench/api/common/extHostTask.ts` (out of partition).
- `vscode.workspace.createFileSystemWatcher` — VS Code extension host API; implemented in `src/vs/workbench/api/common/extHostFileSystemEventService.ts` (out of partition).
- `vscode.ShellExecution` / `vscode.Task` — Task model types defined in the VS Code extension host; see `src/vs/workbench/api/common/extHostTypes.ts` (out of partition).
- `vscode.TaskGroup.Build` / `vscode.TaskGroup.Test` — Also in `extHostTypes.ts` (out of partition).
- `vscode.l10n.t(...)` — Localization utility from the VS Code extension API surface (out of partition).
- `gulp compile-extension:grunt` in `package.json` scripts — Build pipeline defined in the root `Gulpfile.js` / `build/` directory (out of partition).
