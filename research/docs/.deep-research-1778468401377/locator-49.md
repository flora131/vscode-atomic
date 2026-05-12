# File Locator: extensions/jake/ (Partition 49 of 80)

## Implementation

- `extensions/jake/src/main.ts` (340 LOC) - Jake task detection and VS Code integration
  - Exports `activate()` and `deactivate()` extension entry points
  - Contains `TaskDetector` class for managing Jake task providers
  - Contains `FolderDetector` class for detecting Jakefile in workspace folders
  - Utilities: `findJakeCommand()`, `isBuildTask()`, `isTestTask()`, `exists()`, `exec()`
  - Defines `JakeTaskDefinition` interface extending `vscode.TaskDefinition`

## Configuration

- `extensions/jake/package.json` - VS Code extension manifest
  - Name: `jake` (v10.0.0)
  - Activation event: `onTaskType:jake`
  - Configuration: `jake.autoDetect` setting (string enum: "off" | "on")
  - Task definition: `jake` type with required `task` property and optional `file` property
  - Build scripts: compile and watch gulp tasks

- `extensions/jake/tsconfig.json` - TypeScript compilation configuration
  - Extends `../tsconfig.base.json`
  - Source: `./src`, Output: `./out`
  - Includes vscode type definitions

- `extensions/jake/.npmrc` - NPM configuration file

- `extensions/jake/.vscodeignore` - Files to exclude from VS Code package

- `extensions/jake/esbuild.mts` - esbuild configuration for bundling
  - Entry point: `src/main.ts`
  - Output directory: `dist/`

## Documentation

- `extensions/jake/README.md` - Extension documentation
  - Describes Jake integration as bundled (cannot be uninstalled)
  - Features Jake task execution from Jakefile.js files
  - Settings documentation for auto-detection

## Examples / Fixtures

- `extensions/jake/images/cowboy_hat.png` - Extension icon

## Notable Clusters

- `extensions/jake/` - Contains 10 files total
  - 1 TypeScript source file (main.ts)
  - 4 configuration files (package.json, tsconfig.json, .npmrc, esbuild.mts)
  - 1 package lockfile (package-lock.json)
  - 1 localization file (package.nls.json)
  - 1 README
  - 1 ignore file (.vscodeignore)
  - 1 icon asset (cowboy_hat.png)

## Summary

The `extensions/jake/` directory implements a VS Code extension that provides task provider integration for Jake, a JavaScript build tool. The extension detects and runs Jake tasks (from Jakefile or Jakefile.js) as VS Code tasks, with configurable auto-detection via the `jake.autoDetect` setting. Build and test tasks are automatically categorized. The implementation spans a single TypeScript file (main.ts) with approximately 357 LOC (per scope specification) managing workspace folder detection, Jake command resolution, and task lifecycle through the VS Code task provider API. Configuration-heavy with standard Node/TypeScript tooling setup (esbuild + gulp compilation pipeline).
