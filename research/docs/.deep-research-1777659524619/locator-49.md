# Locator 49: extensions/jake/

## Implementation

- `extensions/jake/src/main.ts` - Jake task provider implementation (340 LOC). Core extension logic implementing `vscode.tasks.registerTaskProvider('jake', ...)` at line 270. Includes:
  - `TaskDetector` class managing task provider lifecycle
  - `FolderDetector` class for per-workspace-folder Jake task detection
  - `JakeTaskDefinition` interface extending `vscode.TaskDefinition`
  - Helper functions: `findJakeCommand()`, `isBuildTask()`, `isTestTask()`, `exists()`, `exec()`
  - Jake executable detection across Windows/Linux/Darwin platforms
  - File system watching for Jakefile changes

- `extensions/jake/esbuild.mts` - Build configuration file

## Configuration

- `extensions/jake/package.json` - Extension manifest (76 lines). Declares:
  - Task provider activation on `onTaskType:jake`
  - Task definition type `jake` with required `task` property and optional `file` property
  - Configuration contribution: `jake.autoDetect` (scope: application, default: off)
  - Capability declarations: untrusted workspace support enabled, virtual workspace disabled

- `extensions/jake/tsconfig.json` - TypeScript configuration extending `../tsconfig.base.json`

- `extensions/jake/.npmrc` - NPM configuration
- `extensions/jake/.vscodeignore` - Bundling exclusion patterns

## Documentation

- `extensions/jake/README.md` - User-facing documentation describing Jake task support, feature list, and configuration settings

## Types / Interfaces

- `JakeTaskDefinition` interface in `src/main.ts` (lines 80-83) extending `vscode.TaskDefinition` with properties:
  - `task: string` (required)
  - `file?: string` (optional)

## Notable Clusters

- **Workspace Folder Management**: Lines 229-265 in `src/main.ts` handle dynamic addition/removal of workspace folders and configuration change reactions
- **Task Computation Pipeline**: Lines 133-194 orchestrate Jake command execution, stdout parsing with regex matching, and task metadata assignment (build/test groups)
- **Provider Lifecycle**: Lines 267-283 conditionally register/dispose task provider based on detector population

## Summary

The Jake extension is a focused task provider for VS Code that auto-detects Jake build tasks from `Jakefile` and `Jakefile.js` files. Single TypeScript source file implements a two-tier detection architecture (workspace-level and folder-level) with file system watching, configuration support, and platform-aware Jake executable resolution. The extension registers dynamically using `vscode.tasks.registerTaskProvider('jake', ...)` to provide discovered Jake tasks to VS Code's task system.
