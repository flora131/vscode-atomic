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
