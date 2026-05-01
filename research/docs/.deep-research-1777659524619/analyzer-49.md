### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/jake/src/main.ts` — 340 lines; full Jake TaskProvider implementation
- `/home/norinlavaee/projects/vscode-atomic/extensions/jake/esbuild.mts` — 18 lines; build configuration for the extension

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/extensions/jake/src/main.ts`

**Role:** Implements the Jake task auto-detection extension for VS Code. Exports `activate` and `deactivate` lifecycle hooks. Registers a `vscode.TaskProvider` for the `'jake'` task type.

**Key Symbols (file:line)**

| Symbol | Line | Purpose |
|--------|------|---------|
| `AutoDetect` type alias | 11 | Union `'on' \| 'off'`; maps to `jake.autoDetect` config value |
| `exists(file)` | 13–19 | Promisified `fs.exists`; used to probe for Jakefile and local jake binary |
| `exec(command, options)` | 21–30 | Promisified `cp.exec`; wraps child-process execution; rejects with `{ error, stdout, stderr }` on non-zero exit |
| `buildNames` / `isBuildTask(name)` | 32–40 | String-inclusion check against `['build','compile','watch']`; classifies a task into `TaskGroup.Build` |
| `testNames` / `isTestTask(name)` | 42–50 | String-inclusion check against `['test']`; classifies a task into `TaskGroup.Test` |
| `getOutputChannel()` | 53–58 | Lazy-initializes a singleton `vscode.OutputChannel` named `'Jake Auto Detection'` |
| `showError()` | 60–65 | Shows a warning toast with a "Go to output" action button |
| `findJakeCommand(rootPath)` | 67–78 | Resolves the jake binary path; platform-aware resolution (win32 uses `jake.cmd`, linux/darwin uses `jake`); falls back to bare `'jake'` if no local binary found |
| `JakeTaskDefinition` interface | 80–83 | Extends `vscode.TaskDefinition`; fields: `task: string`, `file?: string` |
| `FolderDetector` class | 85–202 | Per-workspace-folder detection unit |
| `TaskDetector` class | 204–329 | Orchestrates multiple `FolderDetector` instances; registers/unregisters the provider |
| `activate(_context)` | 332–335 | Extension entry point; creates and starts a `TaskDetector` |
| `deactivate()` | 337–339 | Calls `detector.dispose()` |

---

**`FolderDetector` — Control and Data Flow (lines 85–202)**

Constructor (line 90–93): Accepts a `vscode.WorkspaceFolder` and a `Promise<string>` for the resolved jake command. The command promise is computed once by the caller (`findJakeCommand`) and passed in.

`isEnabled()` (line 99–101): Reads `jake.autoDetect` from workspace configuration scoped to the folder URI. Returns `true` only when the value is `'on'`.

`start()` (line 103–109): Creates a `vscode.FileSystemWatcher` matching the glob `{node_modules,Jakefile,Jakefile.js}` inside the folder. Each of `onDidChange`, `onDidCreate`, `onDidDelete` sets `this.promise = undefined`, invalidating the task cache on any relevant filesystem change.

`getTasks()` (line 111–120): Guards on `isEnabled()`. Returns the cached `this.promise` if it exists; otherwise calls `computeTasks()` and stores the result in `this.promise` (memoization). Returns `[]` when detection is disabled.

`getTask(_task)` (line 122–131): Resolves a single task by reading `_task.definition.task` (the task name), then constructing a new `vscode.Task` using `vscode.ShellExecution` with the resolved jake command and task name as arguments. Uses `this.workspaceFolder.uri.fsPath` as `cwd`.

`computeTasks()` (line 133–194) — **Core parsing pipeline**:

1. **Root path guard** (line 134): Only proceeds for `file://` scheme folders.
2. **Jakefile detection** (line 139–145): Checks `Jakefile` first, then `Jakefile.js`; returns `[]` if neither exists.
3. **Command execution** (line 147–149): Runs `<jakeCommand> --tasks` via `exec()` with `{ cwd: rootPath }`.
4. **Stderr handling** (line 150–153): If `stderr` is non-empty, appends to output channel and calls `showError()`.
5. **Output parsing** (line 155–179): Splits `stdout` on `\r?\n`. For each non-empty line, applies regex `/^jake\s+([^\s]+)\s/g` (line 161) to capture the task name in capture group 1. Validates `matches.length === 2` (line 163).
6. **Task construction** (line 165–171): Creates `JakeTaskDefinition` with `type: 'jake'` and the captured task name. Constructs a `vscode.Task` with a `vscode.ShellExecution` of `<jakeCommand> <taskName>`, using `ShellExecutionOptions` with `cwd`.
7. **Group classification** (line 172–177): Lowercases the full output line, then tests via `isBuildTask` / `isTestTask`; assigns `task.group` to `vscode.TaskGroup.Build` or `vscode.TaskGroup.Test`.
8. **Error handling** (line 182–193): Catches exec rejection; appends `err.stderr`, `err.stdout`, and a localized message to the output channel; calls `showError()`; returns `[]`.

`dispose()` (line 196–201): Clears `this.promise` and disposes the file watcher.

---

**`TaskDetector` — Control and Data Flow (lines 204–329)**

State: `taskProvider: vscode.Disposable | undefined` (the registered provider token) and `detectors: Map<string, FolderDetector>` keyed by folder URI string.

`start()` (line 212–219): Calls `updateWorkspaceFolders` with all current folders and an empty removed set. Subscribes to `vscode.workspace.onDidChangeWorkspaceFolders` and `vscode.workspace.onDidChangeConfiguration`.

`updateWorkspaceFolders(added, removed)` (line 229–245):
- For each removed folder: disposes the corresponding `FolderDetector` and deletes it from the map.
- For each added folder: creates a new `FolderDetector(add, findJakeCommand(add.uri.fsPath))`, stores it keyed by `add.uri.toString()`, and calls `detector.start()` only if `detector.isEnabled()`.
- Calls `updateProvider()` at the end.

`updateConfiguration()` (line 247–265): Disposes and deletes all existing detectors, then recreates them from `vscode.workspace.workspaceFolders`, re-enabling only those with `autoDetect === 'on'`. Calls `updateProvider()`.

`updateProvider()` (line 267–283): Registers a new `vscode.tasks.registerTaskProvider('jake', ...)` token if detectors are present and no provider is yet registered (line 268–278). Disposes and nullifies the provider if detectors become empty (line 279–282). The provider object literal has `provideTasks()` delegating to `thisCapture.getTasks()` and `resolveTask(_task)` delegating to `thisCapture.getTask(_task)`.

`computeTasks()` (line 289–309): Dispatches to a single detector if `size === 1` (line 293) or fans out with `Promise.all` for multiple detectors (line 295–308), flattening results.

`getTask(task)` (line 311–328): For a single detector, delegates directly. For multiple detectors, routes by `task.scope.uri.toString()` to the matching `FolderDetector`. Global/Workspace scopes return `undefined` (line 318–319).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/jake/esbuild.mts`

**Role:** Build script that invokes the shared esbuild helper from the monorepo to bundle `src/main.ts` into `dist/main.js`.

**Key Symbols (file:line)**

| Symbol | Line | Purpose |
|--------|------|---------|
| `import { run }` | 6 | Imports the `run` function from `../esbuild-extension-common.mts` (shared across extensions) |
| `srcDir` | 8 | Resolved to `<extension-root>/src` via `import.meta.dirname` |
| `outDir` | 9 | Resolved to `<extension-root>/dist` |
| `run(config, process.argv)` | 11–18 | Invoked with `platform: 'node'`, single entry point `main` mapped to `src/main.ts`, and output directory `dist` |

**Control Flow:** The script is a direct top-level invocation. It computes `srcDir` and `outDir` using Node ESM's `import.meta.dirname` (line 8–9), then calls `run(...)` passing `process.argv` so the shared helper can handle CLI flags (e.g., `--watch`). No conditional logic exists in this file; all bundling behavior is delegated to `esbuild-extension-common.mts`.

---

### Cross-Cutting Synthesis

The Jake extension implements a two-layer TaskProvider architecture. `TaskDetector` (lines 204–329 of `main.ts`) acts as the workspace-level coordinator: it manages a `Map<string, FolderDetector>` keyed by folder URI and lazily registers or unregisters a single `vscode.TaskProvider` token via `updateProvider()`. Each `FolderDetector` (lines 85–202) is scoped to one workspace folder and memoizes its task list in `this.promise`, with cache invalidation driven by a `FileSystemWatcher` monitoring `Jakefile`, `Jakefile.js`, and `node_modules`. The actual task discovery pipeline runs `jake --tasks` as a child process (via the promisified `exec` helper at line 21), parses its stdout line-by-line with the regex `/^jake\s+([^\s]+)\s/g` (line 161), and classifies each discovered task into `Build` or `Test` groups by substring-matching the line against hard-coded name lists. Platform-aware binary resolution happens in `findJakeCommand` (lines 67–78): it checks for local `node_modules/.bin/jake[.cmd]` before falling back to a global `jake` invocation. The build file (`esbuild.mts`) delegates all bundling to the shared `esbuild-extension-common.mts` helper with a `node` platform target and a single `main` entry point, producing output in `dist/`.

---

### Out-of-Partition References

- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-extension-common.mts` — the `run` function imported at `esbuild.mts:6`; handles esbuild invocation, CLI flag parsing (e.g., `--watch`), and output file naming conventions shared across all extensions.
- `vscode` API module — `vscode.tasks.registerTaskProvider`, `vscode.Task`, `vscode.ShellExecution`, `vscode.FileSystemWatcher`, `vscode.TaskGroup`, `vscode.workspace.getConfiguration`, `vscode.l10n.t` are all consumed but implemented by the VS Code extension host runtime.
