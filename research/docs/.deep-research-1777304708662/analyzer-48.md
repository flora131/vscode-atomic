### Files Analysed

1. `/Users/norinlavaee/vscode-atomic/extensions/grunt/src/main.ts` (364 LOC)
2. `/Users/norinlavaee/vscode-atomic/extensions/grunt/package.json` (80 LOC)

---

### Per-File Notes

#### `/Users/norinlavaee/vscode-atomic/extensions/grunt/src/main.ts`

**Role:** Implements the Grunt task auto-detection extension. Discovers Grunt tasks from workspace folders and registers them with VS Code's task system as runnable `ShellExecution` tasks.

**Key Symbols:**

- `AutoDetect` (`main.ts:11`) — string union type `'on' | 'off'` controlling whether auto-detection is enabled per folder.
- `GruntTaskDefinition` (`main.ts:66-70`) — interface extending `vscode.TaskDefinition` with fields `task: string`, optional `args?: string[]`, and optional `file?: string`.
- `exists(file)` (`main.ts:13-19`) — wraps `fs.exists` in a Promise.
- `exec(command, options)` (`main.ts:21-30`) — wraps `cp.exec` in a Promise, rejecting with `{ error, stdout, stderr }` on non-zero exit.
- `isBuildTask(name)` (`main.ts:33-40`) — checks if task name contains any of `['build', 'compile', 'watch']`.
- `isTestTask(name)` (`main.ts:43-50`) — checks if task name contains `'test'`.
- `getOutputChannel()` (`main.ts:53-58`) — lazily creates a singleton `vscode.OutputChannel` named `'Grunt Auto Detection'`.
- `showError()` (`main.ts:60-65`) — shows a warning notification with a link to the output channel.
- `findGruntCommand(rootPath)` (`main.ts:72-83`) — resolves the grunt binary path: on `win32`, checks `node_modules/.bin/grunt.cmd`; on `linux`/`darwin`, checks `node_modules/.bin/grunt`; falls back to global `'grunt'`.
- `FolderDetector` class (`main.ts:85-228`) — per-workspace-folder detector. Holds a `fileWatcher` (`vscode.FileSystemWatcher`) and a cached `promise` of `vscode.Task[]`.
  - `start()` (`main.ts:103-109`) — creates a file system watcher on the glob `{node_modules,[Gg]runtfile.js}` within the folder; any change/create/delete sets `this.promise = undefined` (cache invalidation).
  - `getTasks()` (`main.ts:111-120`) — returns cached promise or calls `computeTasks()`.
  - `getTask(_task)` (`main.ts:122-134`) — resolves a single task by constructing a `ShellExecution` from `_task.definition.task` and `_task.definition.args`, quoting the task name if it contains spaces (`main.ts:129-130`).
  - `computeTasks()` (`main.ts:136-220`) — the core detection logic:
    1. Validates that `rootPath` is a `file:` URI scheme (`main.ts:137-139`).
    2. Checks for the existence of `gruntfile.js` or `Gruntfile.js` (`main.ts:142-144`).
    3. Runs `grunt --help --no-color` via `exec()` (`main.ts:146-148`).
    4. Parses stdout line-by-line (`main.ts:167`), scanning for the `'Available tasks'` sentinel (`main.ts:175`) to start, and `'Tasks run in the order specified'` to stop (`main.ts:179`).
    5. Applies regex `/^\s*(\S.*\S)  \S/g` (`main.ts:182`) to extract each task name from the indented list.
    6. Constructs a `GruntTaskDefinition` (`main.ts:186-189`) and a `vscode.Task` with a `ShellExecution` wrapping `grunt <taskName>`, quoting names containing spaces (`main.ts:192-194`).
    7. Assigns `task.group = vscode.TaskGroup.Build` or `vscode.TaskGroup.Test` based on name classification (`main.ts:197-201`).
    8. On any exec error, logs stdout/stderr to output channel and calls `showError()` (`main.ts:208-218`).
  - `dispose()` (`main.ts:222-227`) — clears cached promise and disposes file watcher.

- `TaskDetector` class (`main.ts:230-354`) — workspace-level coordinator. Maintains a `Map<string, FolderDetector>` keyed by folder URI string, and a single registered `vscode.Disposable` for the task provider.
  - `start()` (`main.ts:238-245`) — iterates current workspace folders, subscribes to `onDidChangeWorkspaceFolders` and `onDidChangeConfiguration`.
  - `updateWorkspaceFolders(added, removed)` (`main.ts:255-271`) — disposes detectors for removed folders; creates new `FolderDetector` instances for added folders; calls `updateProvider()`.
  - `updateConfiguration()` (`main.ts:273-291`) — disposes all detectors and recreates them; calls `updateProvider()`.
  - `updateProvider()` (`main.ts:293-309`) — calls `vscode.tasks.registerTaskProvider('grunt', { provideTasks, resolveTask })` if detectors exist but provider is not yet registered (`main.ts:296`); disposes provider if no detectors remain (`main.ts:305-308`).
  - `computeTasks()` (`main.ts:315-335`) — if one detector, delegates directly; if multiple, fans out with `Promise.all`, flattening results.
  - `getTask(task)` (`main.ts:337-353`) — routes `resolveTask` to the correct `FolderDetector` by matching `task.scope.uri` (`main.ts:346-348`).

- `activate(_context)` (`main.ts:357-360`) — module entry point; instantiates `TaskDetector` and calls `start()`.
- `deactivate()` (`main.ts:362-364`) — calls `detector.dispose()`.

**Control Flow:**

```
activate()
  → new TaskDetector()
  → TaskDetector.start()
      → updateWorkspaceFolders(currentFolders, [])
          → new FolderDetector(folder, findGruntCommand(folder.uri.fsPath))
          → FolderDetector.start()    [if autoDetect=on]
              → createFileSystemWatcher('{node_modules,[Gg]runtfile.js}')
          → updateProvider()
              → vscode.tasks.registerTaskProvider('grunt', { provideTasks, resolveTask })
      → subscribe onDidChangeWorkspaceFolders → updateWorkspaceFolders
      → subscribe onDidChangeConfiguration   → updateConfiguration

provideTasks() [called by VS Code task system]
  → TaskDetector.getTasks()
  → TaskDetector.computeTasks()
  → FolderDetector.getTasks()
  → FolderDetector.computeTasks()   [if cache miss]
      → findGruntCommand(rootPath)
      → exec('grunt --help --no-color', { cwd: rootPath })
      → parse stdout: extract task names between 'Available tasks' and 'Tasks run in the order specified'
      → for each name: new vscode.Task(..., new vscode.ShellExecution(...))
      → classify into TaskGroup.Build or TaskGroup.Test

resolveTask(_task) [called by VS Code when task is run from tasks.json]
  → TaskDetector.getTask(_task)
  → FolderDetector.getTask(_task)
      → new vscode.Task(..., new vscode.ShellExecution('grunt', [taskName, ...args], {cwd}))
```

**Data Flow:**

- Input: workspace folder URIs, `grunt.autoDetect` configuration, filesystem presence of `gruntfile.js`/`Gruntfile.js`, stdout of `grunt --help --no-color`.
- Intermediate: raw stdout string → split by `\r?\n` → filtered section between sentinel lines → regex-extracted task name strings.
- Output: `vscode.Task[]` with `ShellExecution`, assigned `TaskGroup`, contributed to VS Code task picker.

**Dependencies:**

- Node built-ins: `path` (`main.ts:6`), `fs` (`main.ts:7`), `child_process` as `cp` (`main.ts:8`).
- VS Code Extension API: `vscode.workspace`, `vscode.tasks`, `vscode.window`, `vscode.l10n`, `vscode.Task`, `vscode.ShellExecution`, `vscode.TaskGroup`, `vscode.FileSystemWatcher` (all via `import * as vscode from 'vscode'` at `main.ts:9`).
- Runtime dependency on the `grunt` CLI binary, either local (`node_modules/.bin/grunt[.cmd]`) or global PATH.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/grunt/package.json`

**Role:** Extension manifest declaring activation event, task schema contributions, and configuration properties.

**Key Symbols and Declarations:**

- `"activationEvents": ["onTaskType:grunt"]` (`package.json:25-27`) — extension activates only when VS Code encounters a task of type `grunt`, avoiding unnecessary startup cost.
- `"main": "./out/main"` (`package.json:24`) — compiled output entry point.
- `"capabilities"` (`package.json:28-33`):
  - `"virtualWorkspaces": false` — extension does not support virtual file system workspaces (requires real filesystem to run grunt CLI).
  - `"untrustedWorkspaces": { "supported": true }` — allowed in restricted mode workspaces.
- `"contributes"."configuration"` (`package.json:36-51`):
  - Property `grunt.autoDetect` with type `string`, enum `["off","on"]`, default `"off"` (`package.json:41-49`). Controls the `AutoDetect` type used by `FolderDetector.isEnabled()` at `main.ts:100`.
- `"contributes"."taskDefinitions"` (`package.json:52-74`):
  - Defines schema for tasks of type `grunt` with required field `task` (string), optional `args` (array), optional `file` (string).
  - `"when": "shellExecutionSupported"` (`package.json:73`) — conditionally hides task type when shell execution is not available (e.g., web/browser contexts).
- `"devDependencies"`: only `@types/node: "22.x"` (`package.json:22`); no runtime npm dependencies.

---

### Cross-Cutting Synthesis

The `extensions/grunt` partition implements VS Code's built-in Grunt task provider as a self-contained TypeScript extension compiled to `./out/main`. The architecture uses two cooperating classes: `TaskDetector` (`main.ts:230`) acts as the workspace-level lifecycle manager, tracking one `FolderDetector` (`main.ts:85`) per workspace folder in a `Map<string, FolderDetector>`. Task discovery relies entirely on spawning the Grunt CLI via `child_process.exec` with `--help --no-color` (`main.ts:146`) and parsing stdout with a state machine that delimiters on two sentinel strings and a two-space-gap regex (`main.ts:167-204`). Detected tasks are wrapped as `vscode.Task` with `vscode.ShellExecution`, meaning actual execution is entirely delegated to VS Code's integrated terminal. Cache invalidation is event-driven via `vscode.FileSystemWatcher` on `gruntfile.js` and `node_modules` changes (`main.ts:106-108`). The manifest's `onTaskType:grunt` activation event (`package.json:26`) and `"virtualWorkspaces": false` capability (`package.json:29`) constrain this extension to environments with a real filesystem and shell execution. For a Tauri/Rust port, the subprocess invocation pattern (`cp.exec` → parse stdout) would need to map to `std::process::Command` or Tauri's sidecar API, and the `vscode.tasks` registration surface has no direct Tauri equivalent, requiring a custom task discovery and execution subsystem.

---

### Out-of-Partition References

- `vscode.tasks.registerTaskProvider` — VS Code core task API, implemented in `src/vs/workbench/api/common/extHostTask.ts` and `src/vs/workbench/contrib/tasks/`.
- `vscode.ShellExecution` — shell execution type defined in VS Code extension host API (`src/vs/workbench/api/common/extHostTypes.ts`).
- `vscode.TaskGroup.Build` / `vscode.TaskGroup.Test` — task group constants from VS Code API.
- `vscode.workspace.createFileSystemWatcher` — file watcher API from `src/vs/workbench/api/common/extHostFileSystemEventService.ts`.
- `vscode.l10n.t(...)` — localization API from `src/vs/workbench/api/common/extHostLocalization.ts`.
- `gulp compile-extension:grunt` build task — defined in the root `Gulpfile.js` build system.
- Analogous sibling extensions: `extensions/npm/`, `extensions/jake/` follow the same `FolderDetector`/`TaskDetector` pattern for other task runners.
