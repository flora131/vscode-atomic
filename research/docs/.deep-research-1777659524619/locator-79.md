# Build Orchestration Entry Point (gulpfile.mjs)

## Research Context
Locating the VS Code build system entry point to understand how TypeScript/Electron core functionality is orchestrated for building.

## Implementation

### Root Build Entry Point
- `gulpfile.mjs` — Minimal re-export of the main build orchestration; imports `./build/gulpfile.ts` as the actual build root

### Build System Files
- `build/gulpfile.ts` — Main gulp build orchestration file
- `build/gulpfile.compile.ts` — TypeScript compilation tasks
- `build/gulpfile.extensions.ts` — Extension building tasks
- `build/gulpfile.editor.ts` — Editor-specific build tasks
- `build/gulpfile.vscode.ts` — VS Code application build tasks
- `build/gulpfile.vscode.linux.ts` — Linux platform-specific build
- `build/gulpfile.vscode.win32.ts` — Windows platform-specific build
- `build/gulpfile.vscode.web.ts` — Web version build tasks
- `build/gulpfile.reh.ts` — Remote execution host build tasks
- `build/gulpfile.cli.ts` — CLI tool build tasks
- `build/gulpfile.hygiene.ts` — Code hygiene and linting tasks
- `build/gulpfile.scan.ts` — Dependency scanning tasks

### Related Components
- `extensions/gulp/` — VS Code Gulp extension (10 files): provides UI integration for running gulp tasks within the editor; includes TypeScript source (`src/main.ts`), build configuration (`esbuild.mts`), package metadata, and documentation

## Summary

The root gulpfile.mjs is a minimal entry point that delegates to `build/gulpfile.ts`, which orchestrates the complete TypeScript/Electron build system. The build directory contains 12 specialized gulp files handling compilation, platform-specific packaging (Linux, Windows, Web), extensions, CLI, and code hygiene. A companion VS Code extension (`extensions/gulp/`) provides task runner integration within the IDE itself. For porting to Tauri/Rust, the entire build orchestration layer would need replacement to handle Rust compilation, asset bundling, and cross-platform packaging.
