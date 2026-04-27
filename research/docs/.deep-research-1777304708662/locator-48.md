# Locator 48: extensions/grunt

## Scope Summary
- **Directory**: `extensions/grunt/`
- **File Count**: 2 source files (plus configuration/metadata)
- **Lines of Code**: ~364 LOC (main.ts)
- **Pattern**: Build task provider extension following same architecture as gulp

## Implementation

### Core Task Provider
- `/Users/norinlavaee/vscode-atomic/extensions/grunt/src/main.ts` (364 lines)
  - Main entry point with `activate()` and `deactivate()` exports
  - **TaskDetector class**: Central orchestrator managing task detection lifecycle across all workspace folders
    - Maintains map of FolderDetector instances keyed by workspace folder URI
    - Registers the task provider via `vscode.tasks.registerTaskProvider('grunt', {...})`
    - Implements `provideTasks()` and `resolveTask()` provider interface
    - Responds to workspace folder additions/removals and configuration changes
  - **FolderDetector class**: Per-folder task discovery for individual workspaces
    - Watches for gruntfile.js/Gruntfile.js presence changes
    - Executes `grunt --help --no-color` to enumerate available tasks
    - Parses text output to extract task names and descriptions
    - Caches computed tasks and invalidates on file changes
    - Classifies tasks as Build or Test based on name patterns
    - Constructs vscode.Task objects with ShellExecution for task invocation
  - **Utility Functions**:
    - `findGruntCommand()`: Resolves platform-specific grunt binary path (grunt.cmd on Windows, grunt on Unix)
    - `isBuildTask()`: Identifies tasks containing 'build', 'compile', or 'watch'
    - `isTestTask()`: Identifies tasks containing 'test'
    - `exists()`, `exec()`: File system and process execution helpers
  - **GruntTaskDefinition Interface**: Extends vscode.TaskDefinition with task name, optional args, and file properties

## Configuration

### Extension Manifest
- `/Users/norinlavaee/vscode-atomic/extensions/grunt/package.json`
  - Activation Event: `onTaskType:grunt` (lazy load when grunt tasks referenced)
  - Contributes:
    - Configuration schema with `grunt.autoDetect` boolean setting (default: off, scope: application)
    - Task definition type `'grunt'` with required `task` field and optional `args`/`file` fields
  - Capabilities: Disabled for virtual workspaces, supported in untrusted workspaces

### Localization Strings
- `/Users/norinlavaee/vscode-atomic/extensions/grunt/package.nls.json`
  - Localized descriptions for settings and task definition properties

### Build Configuration
- `/Users/norinlavaee/vscode-atomic/extensions/grunt/tsconfig.json`: TypeScript compilation config
- `/Users/norinlavaee/vscode-atomic/extensions/grunt/esbuild.mts`: ESBuild build script
- `/Users/norinlavaee/vscode-atomic/extensions/grunt/package-lock.json`: Dependency lock file

## Documentation

- `/Users/norinlavaee/vscode-atomic/extensions/grunt/README.md`
  - User-facing extension overview
  - Feature summary: integrates Grunt task definitions as VS Code tasks
  - Build task classification rule documentation
  - Settings documentation

## Notable Patterns

**Task Provider Registration**: Line 296 shows the core API call:
```typescript
vscode.tasks.registerTaskProvider('grunt', {
  provideTasks: (): Promise<vscode.Task[]> => {...},
  resolveTask(_task: vscode.Task): Promise<vscode.Task | undefined> => {...}
})
```

**Task Discovery Flow**: 
1. FolderDetector monitors workspace folder for gruntfile.js via FileSystemWatcher
2. On detection, executes `grunt --help` to parse available tasks
3. Constructs Task objects with ShellExecution for deferred execution
4. TaskDetector aggregates results across multiple workspace folders

**Key Dependencies**: Only @types/node in devDependencies; uses vscode API for all functionality.

---

## Summary

The Grunt extension (partition 48) implements a TaskProvider following the identical architectural pattern as the gulp extension. It provides automatic Grunt task detection by executing `grunt --help` to enumerate available tasks and expose them to VS Code's task system. The extension uses lazy activation via `onTaskType:grunt` and maintains per-folder detectors to handle multi-root workspaces. The implementation demonstrates clean separation between task discovery logic (FolderDetector) and provider registration lifecycle (TaskDetector), with platform-aware grunt command resolution and task classification based on naming conventions (build vs. test). File count is minimal (2 source files) with configuration defined through package.json's contributes schema.

