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
