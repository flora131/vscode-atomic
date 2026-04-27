# Jake Extension: Tasks API Contributor (Partition 49)

## Implementation

- `/Users/norinlavaee/vscode-atomic/extensions/jake/src/main.ts` (340 LOC) - Core implementation containing:
  - `TaskDetector` class: Main orchestrator that registers the task provider via `vscode.tasks.registerTaskProvider('jake', ...)` at line 270
  - `FolderDetector` class: Per-workspace folder detector that discovers and parses Jakefile tasks
  - `JakeTaskDefinition` interface extending `vscode.TaskDefinition` with `task` and optional `file` properties
  - Helper functions: `findJakeCommand()`, `isBuildTask()`, `isTestTask()`, `exec()`, `exists()`
  - Activation hooks: `activate()` initializes the detector, `deactivate()` cleans up resources

## Configuration

- `/Users/norinlavaee/vscode-atomic/extensions/jake/package.json` (76 lines) - Extension manifest defining:
  - `contributes.taskDefinitions`: Declares 'jake' task type with required 'task' property and optional 'file' property
  - `contributes.configuration`: Exposes 'jake.autoDetect' setting (default: 'off')
  - `activationEvents`: Triggered on `onTaskType:jake`
  - Build scripts: `compile` and `watch` via gulp

- `/Users/norinlavaee/vscode-atomic/extensions/jake/tsconfig.json` - TypeScript compilation targeting Node environment
- `/Users/norinlavaee/vscode-atomic/extensions/jake/esbuild.mts` - ESBuild configuration for bundling main.ts entry point

## Documentation

- `/Users/norinlavaee/vscode-atomic/extensions/jake/README.md` - User-facing documentation describing Jake task detection, features, and settings

## Notable Characteristics

The Jake extension implements the **tasks API contributor pattern** by registering a `TaskProvider` with `vscode.tasks.registerTaskProvider()`. The provider's two methods (`provideTasks` and `resolveTask`) enable dynamic discovery of Jake tasks from `Jakefile` or `Jakefile.js` by:

1. Monitoring workspace folders and configuration changes
2. Executing `jake --tasks` command to parse available tasks
3. Categorizing tasks as build or test groups based on task name heuristics
4. Wrapping discovered tasks in `vscode.Task` objects with shell execution context

This design pattern mirrors the gulp extension and demonstrates how to integrate external build tool ecosystems into VS Code's task infrastructure. The extension uses workspace-scoped detection to handle multi-root workspaces and respects the autoDetect configuration setting for performance optimization.
