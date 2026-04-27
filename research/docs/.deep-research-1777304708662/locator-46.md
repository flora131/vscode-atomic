# Locator 46: extensions/gulp/

## Overview
The `extensions/gulp/` directory contains a VS Code extension for automatic Gulp task detection and execution. This is a focused, single-purpose extension with minimal scope (2 source files, 424 LOC).

## Implementation
- `extensions/gulp/src/main.ts` — TypeScript extension implementation; registers a task provider via `vscode.tasks.registerTaskProvider('gulp', ...)` that auto-detects gulpfile.js/ts variants and provides shell execution wrappers for Gulp tasks; demonstrates VS Code's built-in Tasks API for declarative task management.

## Configuration
- `extensions/gulp/package.json` — Extension manifest declaring task definition type 'gulp' with `taskDefinitions` and configuration property `gulp.autoDetect`; activates on `onTaskType:gulp` event.
- `extensions/gulp/tsconfig.json` — TypeScript compiler configuration extending base, targets Node types, outputs to ./out.
- `extensions/gulp/package.nls.json` — Localization strings for UI text (description, display name, config descriptions).

## Documentation
- `extensions/gulp/README.md` — User-facing documentation explaining extension purpose, features (Gulp task running, build/test task classification), and the `gulp.autoDetect` setting.

---

**Relevance to VS Code Porting Research:**
The Gulp extension demonstrates VS Code's extensibility architecture around task execution. It uses the public VSCode API (`vscode.tasks.registerTaskProvider`, `vscode.workspace`, `vscode.window`) to provide task detection and execution without being part of core. This shows how task management functionality can be separated from a hypothetical Rust/Tauri core—the extension system itself would need to remain functional (with API compatibility) during a port, and task providers like this would need language bindings or API translation layers in a new platform architecture.
