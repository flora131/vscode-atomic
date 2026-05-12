# Partition 48 of 80 — Findings

## Scope
`extensions/grunt/` (2 files, 382 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locator Research: extensions/grunt/

## Implementation

- `/home/norinlavaee/projects/vscode-atomic/extensions/grunt/src/main.ts` - Main extension activation and Grunt task detection logic. Implements two core classes:
  - `FolderDetector`: Monitors workspace folders for Gruntfile.js, watches for changes, and detects available Grunt tasks by parsing `grunt --help --no-color` output
  - `TaskDetector`: Manages multiple FolderDetectors across workspace folders, registers VS Code task provider for 'grunt' task type
  - Utility functions for command execution, file existence checks, and task categorization (build/test)

## Configuration

- `/home/norinlavaee/projects/vscode-atomic/extensions/grunt/package.json` - Extension metadata including:
  - Activation event: `onTaskType:grunt`
  - Task definition schema with properties: task (required), args, file
  - Configuration: `grunt.autoDetect` (on/off, default off)
  - Dev dependencies: @types/node 22.x
  
- `/home/norinlavaee/projects/vscode-atomic/extensions/grunt/tsconfig.json` - TypeScript compilation config extending base, targeting Node types, outputting to `./out/`

- `/home/norinlavaee/projects/vscode-atomic/extensions/grunt/esbuild.mts` - ESBuild configuration for Node platform, entry point at `src/main.ts`, output to `dist/`

## Documentation

- `/home/norinlavaee/projects/vscode-atomic/extensions/grunt/README.md` - User documentation describing Grunt task runner integration, feature list, and settings

## Notable Clusters

The `extensions/grunt/` directory comprises a lightweight VS Code extension (382 LOC) that integrates external Grunt task execution. The architecture relies on:

1. **Process spawning**: Uses Node.js `child_process.exec()` to invoke grunt binary and parse output
2. **File system watching**: Monitors Gruntfile.js and node_modules changes via VS Code's FileSystemWatcher API
3. **Task provider pattern**: Implements VS Code's TaskProvider interface for task detection and resolution
4. **Workspace folder abstraction**: Handles multi-folder workspaces with per-folder detection instances

Key porting considerations for Tauri/Rust include:
- Process execution via `child_process` → Rust process spawning (tokio/std::process)
- File system watcher → Rust notify crate or Tauri fs watcher
- VS Code API bindings → Would require Tauri bindings to equivalent VS Code extension API or reimplementation
- Node-style promise-based async → Rust async/await with tokio

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Task Provider Pattern Analysis
## Partition 48: extensions/grunt

### Scope: Task Provider Registration & Automation Task Auto-Detection

**Codebase:** `/home/norinlavaee/projects/vscode-atomic/extensions/grunt/`
**Primary file:** `src/main.ts` (365 LOC)

---

#### Pattern: TaskProvider Registration via vscode.tasks.registerTaskProvider()

**Where:** `src/main.ts:296`

**What:** Core task provider registration mechanism that implements the vscode.TaskProvider interface with two key methods: `provideTasks()` for discovery and `resolveTask()` for resolution.

```typescript
this.taskProvider = vscode.tasks.registerTaskProvider('grunt', {
  provideTasks: (): Promise<vscode.Task[]> => {
    return thisCapture.getTasks();
  },
  resolveTask(_task: vscode.Task): Promise<vscode.Task | undefined> {
    return thisCapture.getTask(_task);
  }
});
```

**Variations / call-sites:**
- Registration occurs in `TaskDetector.updateProvider()` (line 293-309)
- Only registers when detectors are available (size > 0)
- Unregisters when no detectors remain (line 305-308)
- Activation event triggered by `"onTaskType:grunt"` in package.json

---

#### Pattern: FolderDetector Class with File Watching

**Where:** `src/main.ts:85-228`

**What:** State machine per workspace folder that watches for Gruntfile changes and caches task discovery. Invalidates cache on file system events (create/delete/change).

```typescript
class FolderDetector {
  private fileWatcher: vscode.FileSystemWatcher | undefined;
  private promise: Thenable<vscode.Task[]> | undefined;

  public start(): void {
    const pattern = path.join(this._workspaceFolder.uri.fsPath, '{node_modules,[Gg]runtfile.js}');
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

**Variations / call-sites:**
- One detector created per workspace folder (line 264)
- Cached via Map using `folder.uri.toString()` as key (line 265)
- Disposed on folder removal (line 259)
- Configuration-aware: checks `grunt.autoDetect` setting (line 100)

---

#### Pattern: External Command Execution for Task Discovery

**Where:** `src/main.ts:136-219` (computeTasks method)

**What:** Spawns `grunt --help --no-color` to discover available tasks by parsing stdout, building Task objects with proper group classification (Build/Test).

```typescript
private async computeTasks(): Promise<vscode.Task[]> {
  const commandLine = `${await this._gruntCommand} --help --no-color`;
  try {
    const { stdout, stderr } = await exec(commandLine, { cwd: rootPath });
    // ... parse stdout ...
    const regExp = /^\s*(\S.*\S)  \S/g;
    const matches = regExp.exec(line);
    if (matches && matches.length === 2) {
      const name = matches[1];
      const kind: GruntTaskDefinition = {
        type: 'grunt',
        task: name
      };
      const task = name.indexOf(' ') === -1
        ? new vscode.Task(kind, this.workspaceFolder, name, source, new vscode.ShellExecution(`${await this._gruntCommand} ${name}`, options))
        : new vscode.Task(kind, this.workspaceFolder, name, source, new vscode.ShellExecution(`${await this._gruntCommand} "${name}"`, options));
      if (isBuildTask(lowerCaseTaskName)) {
        task.group = vscode.TaskGroup.Build;
      } else if (isTestTask(lowerCaseTaskName)) {
        task.group = vscode.TaskGroup.Test;
      }
    }
  } catch (err) { /* error handling */ }
}
```

**Variations / call-sites:**
- Helper function `findGruntCommand()` (line 72-83) locates correct grunt binary (Windows: grunt.cmd, Unix: grunt, fallback: system PATH)
- ShellExecution with proper quoting for task names containing spaces (line 194)
- Error logging to output channel with user-facing warning dialogs (line 149-151, 209-217)

---

#### Pattern: Multi-Folder Workspace Aggregation

**Where:** `src/main.ts:311-335` (computeTasks in TaskDetector)

**What:** Aggregates task discovery across multiple workspace folders with lazy aggregation: returns single folder result directly, otherwise Promise.all() aggregation.

```typescript
public getTasks(): Promise<vscode.Task[]> {
  return this.computeTasks();
}

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

**Variations / call-sites:**
- Task resolution also follows same pattern (line 337-353)
- Scoped task resolution checks TaskScope (Workspace/Global vs folder-specific)
- Error swallowing in aggregation: `.then((value) => value, () => [])` converts failures to empty arrays

---

#### Pattern: Workspace Configuration Listeners with Dynamic Provider Lifecycle

**Where:** `src/main.ts:238-291`

**What:** TaskDetector lifecycle manager reacting to workspace changes and configuration updates. Registers/unregisters task provider based on detector availability.

```typescript
public start(): void {
  const folders = vscode.workspace.workspaceFolders;
  if (folders) {
    this.updateWorkspaceFolders(folders, []);
  }
  vscode.workspace.onDidChangeWorkspaceFolders((event) => this.updateWorkspaceFolders(event.added, event.removed));
  vscode.workspace.onDidChangeConfiguration(this.updateConfiguration, this);
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
        const detector = new FolderDetector(folder, findGruntCommand(folder.uri.fsPath));
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

**Variations / call-sites:**
- Configuration scope: `grunt.autoDetect` from workspace settings (line 100)
- Complete detector recreation on config change (line 273-291)
- Dual event listeners: workspace folder changes + configuration changes
- Cleanup on deactivation (line 362-363)

---

#### Pattern: TaskDefinition Interface with Type Discriminator

**Where:** `src/main.ts:66-70` and package.json contributes

**What:** Custom task definition interface extending vscode.TaskDefinition with 'grunt' type discriminator and optional args/file properties.

```typescript
interface GruntTaskDefinition extends vscode.TaskDefinition {
  task: string;
  args?: string[];
  file?: string;
}
```

**package.json declaration:**
```json
"taskDefinitions": [
  {
    "type": "grunt",
    "required": ["task"],
    "properties": {
      "task": { "type": "string" },
      "args": { "type": "array" },
      "file": { "type": "string" }
    },
    "when": "shellExecutionSupported"
  }
]
```

**Variations / call-sites:**
- Type field set to 'grunt' string literal (line 187)
- Task instantiation with definition as first arg (line 129, 194)
- Manifest registration gates availability to `shellExecutionSupported`

---

#### Pattern: Existence Checking and Command Resolution

**Where:** `src/main.ts:13-30` and 72-83

**What:** Promisified fs.exists() wrapper for async file checks; platform-aware command resolution with fallback chain.

```typescript
function exists(file: string): Promise<boolean> {
  return new Promise<boolean>((resolve, _reject) => {
    fs.exists(file, (value) => {
      resolve(value);
    });
  });
}

async function findGruntCommand(rootPath: string): Promise<string> {
  let command: string;
  const platform = process.platform;
  if (platform === 'win32' && await exists(path.join(rootPath!, 'node_modules', '.bin', 'grunt.cmd'))) {
    command = path.join('.', 'node_modules', '.bin', 'grunt.cmd');
  } else if ((platform === 'linux' || platform === 'darwin') && await exists(path.join(rootPath!, 'node_modules', '.bin', 'grunt'))) {
    command = path.join('.', 'node_modules', '.bin', 'grunt');
  } else {
    command = 'grunt';
  }
  return command;
}
```

**Variations / call-sites:**
- Used during FolderDetector construction (line 92, 264, 282)
- Awaited in ShellExecution construction (line 129, 130, 193, 146)
- Graceful degradation to system PATH 'grunt' command

---

## Summary

The Grunt extension demonstrates a mature task provider implementation pattern for VS Code with the following key architectural concepts:

1. **Provider Registration Pattern**: Single task provider registered/unregistered based on detector availability, implementing both discovery (`provideTasks`) and resolution (`resolveTask`) protocol methods.

2. **Per-Folder State Management**: FolderDetector encapsulates folder-specific lifecycle with file watching, caching, and configuration awareness. One detector per workspace folder with Map-based lookup.

3. **Lazy Task Discovery**: External command execution (`grunt --help`) occurs on-demand with result caching, invalidated on file system changes. Multi-folder aggregation uses Promise.all() with error swallowing.

4. **Dynamic Lifecycle**: Configuration change triggers complete detector recreation. Workspace folder add/remove events update detector map. Task provider registration toggles based on detector count.

5. **Type-Safe Definitions**: Custom TaskDefinition interface with type discriminator registered in manifest with conditional activation based on shell execution capability.

6. **Platform Awareness**: Command resolution checks platform and uses platform-specific binary paths with fallback to system PATH.

For a Tauri/Rust port, these patterns suggest the need for equivalent task provider IPC channels, file system watching abstractions, configuration change listeners, and multi-workspace aggregation logic at the core architecture level.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
