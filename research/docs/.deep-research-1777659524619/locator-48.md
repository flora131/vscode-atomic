# Codebase Locator: Partition 48 of 79

## Scope: `extensions/grunt/`

Task Provider implementation for Grunt task detection and execution in VS Code.

---

## Implementation

### Core Extension Logic
- `extensions/grunt/src/main.ts` — Main extension module providing Grunt task provider integration
  - `TaskDetector` class (lines 230-354) — Manages task provider registration and workspace folder monitoring
  - `FolderDetector` class (lines 85-228) — Detects and computes Grunt tasks per workspace folder
  - `GruntTaskDefinition` interface (lines 66-70) — Type definition extending `vscode.TaskDefinition`
  - `registerTaskProvider('grunt', ...)` invocation (line 296) — Registers the task provider with VS Code
  - Task parsing logic (lines 136-219) — Parses Grunt help output to extract available tasks
  - Task group classification (lines 33-50, 197-201) — Categorizes tasks as Build or Test based on naming patterns

### Utility Functions
- File system helpers: `exists()` (lines 13-19), `exec()` (lines 21-30)
- Task classification: `isBuildTask()` (lines 33-40), `isTestTask()` (lines 43-50)
- Output channel management: `getOutputChannel()`, `showError()` (lines 52-65)
- Grunt command detection: `findGruntCommand()` (lines 72-83)

### Entry Points
- `activate()` function (lines 357-360) — Extension activation hook
- `deactivate()` function (lines 362-364) — Extension cleanup hook

---

## Types / Interfaces

### Type Definitions
- `AutoDetect` type alias (line 11) — Union type for configuration: `'on' | 'off'`
- `GruntTaskDefinition` interface (lines 66-70) — Extends `vscode.TaskDefinition` with properties:
  - `task: string` (required)
  - `args?: string[]` (optional)
  - `file?: string` (optional)

---

## Configuration

### Package Metadata
- `extensions/grunt/package.json` — Extension manifest defining:
  - Activation event: `onTaskType:grunt` (line 26)
  - Configuration schema: `grunt.autoDetect` (lines 40-49) — Enable/disable auto-detection, default is `off`
  - Task definition: Type `grunt` with required `task` property and optional `args`, `file` properties (lines 52-74)
  - Main entry point: `./out/main` (line 24)
  - Capabilities: Virtual workspaces unsupported, untrusted workspaces supported (lines 28-32)
  - Build script: `gulp compile-extension:grunt` (line 17)

### Build Configuration
- `extensions/grunt/tsconfig.json` — TypeScript compiler configuration
  - Source directory: `./src`
  - Output directory: `./out`
  - Node types included

### Build Script
- `extensions/grunt/esbuild.mts` — ESBuild bundler configuration for Node platform
  - Entry point: `src/main.ts` → `dist/main`
  - Uses shared ESBuild common configuration

---

## Documentation

### README
- `extensions/grunt/README.md` — Extension user documentation
  - Feature summary: Integrates Grunt task runner with VS Code task system
  - Task group classification rules (Build, Test)
  - Settings documentation: `grunt.autoDetect` configuration
  - Notice: Extension is bundled with VS Code

---

## Notable Clusters

### Task Detection Pipeline
Files implementing the task detection workflow:
1. `main.ts` — `TaskDetector` orchestrates detection (lines 230-354)
2. `main.ts` — `FolderDetector` computes per-folder tasks (lines 85-228)
3. `main.ts` — Grunt command resolution via `findGruntCommand()` (lines 72-83)
4. `main.ts` — Task parsing from `grunt --help` output (lines 136-219)

### Workspace Monitoring
Task detection with workspace lifecycle integration:
- `FolderDetector.start()` (lines 103-109) — File watcher for Gruntfile changes
- `TaskDetector.updateWorkspaceFolders()` (lines 255-271) — Handles folder additions/removals
- `TaskDetector.updateConfiguration()` (lines 273-291) — Responds to configuration changes
- `TaskDetector.updateProvider()` (lines 293-309) — Manages task provider lifecycle

### Task Provider Implementation
- Registration logic (line 296): `vscode.tasks.registerTaskProvider('grunt', {...})`
- Provider interface implementation (lines 297-302):
  - `provideTasks()` — Returns detected tasks
  - `resolveTask()` — Resolves task definitions to executable tasks

---

## Directory Structure

```
extensions/grunt/
├── src/
│   └── main.ts                  (365 LOC - Core implementation)
├── esbuild.mts                  (19 LOC - Build configuration)
├── tsconfig.json                (18 LOC - TypeScript config)
├── package.json                 (80 LOC - Extension manifest)
├── README.md                     (14 LOC - User documentation)
├── package-lock.json            
├── package.nls.json             (Localization strings)
├── .npmrc                        (npm configuration)
├── .vscodeignore                 (Package exclusions)
└── images/
    └── grunt.png                (Icon asset)
```

**Total: 2 source files (main.ts + build config), 382 LOC**

---

## Summary

The Grunt extension provides VS Code task provider integration for detecting and executing Grunt tasks. The implementation is centered on the `TaskDetector` and `FolderDetector` classes that monitor workspace folders and parse Grunt task definitions via command-line invocation. The extension registers itself with VS Code's task system via `vscode.tasks.registerTaskProvider('grunt', ...)` on line 296 of `main.ts`, enabling automatic task detection when `grunt.autoDetect` configuration is enabled. Task classification rules identify build and test tasks by name pattern matching. The architecture supports multi-folder workspaces with per-folder task detection and file system watching for Gruntfile changes.
