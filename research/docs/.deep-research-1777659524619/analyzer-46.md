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
