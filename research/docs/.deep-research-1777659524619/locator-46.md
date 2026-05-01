# File Locator Report: Partition 46 — extensions/gulp/

## Research Question
What it would take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

## Scope
`extensions/gulp/` (2 files, 424 LOC)

---

## Implementation

- `extensions/gulp/src/main.ts` — Main extension entry point implementing TaskProvider registration via `vscode.tasks.registerTaskProvider('gulp', ...)`. Handles gulp task discovery, execution, and integration with VS Code's task system. Contains utilities for detecting build/test tasks and spawning child processes to execute gulp commands.

- `extensions/gulp/esbuild.mts` — Build configuration for bundling the extension.

---

## Configuration

- `extensions/gulp/package.json` — Extension manifest defining activation events (`onTaskType:gulp`), taskDefinitions for gulp with properties (task, file), and configuration schema for `gulp.autoDetect` setting.

- `extensions/gulp/tsconfig.json` — TypeScript compilation configuration extending base config, targeting Node types.

- `extensions/gulp/package-lock.json` — Dependency lock file.

- `extensions/gulp/.npmrc` — NPM configuration.

- `extensions/gulp/.vscodeignore` — Extension packaging exclusions.

---

## Documentation

- `extensions/gulp/README.md` — User-facing documentation describing gulp task auto-detection and integration with VS Code's task system.

---

## Localization

- `extensions/gulp/package.nls.json` — Localization strings for UI labels.

---

## Images

- `extensions/gulp/images/gulp.png` — Gulp logo asset.

---

## Notable Clusters

**Single source file**: The extension is minimal—one TypeScript implementation file (~350 LOC) handling the entire gulp integration via the TaskProvider API.

**Extension architecture pattern**: 
- Activation: triggered on `onTaskType:gulp`
- Contribution: task definition schema registration
- Runtime: registers task provider callback that discovers and executes gulp tasks

---

## Relevance to Porting Question

This extension exemplifies **extension API integration points** that would require porting in a Tauri/Rust rewrite:

1. **TaskProvider API** (`vscode.tasks.registerTaskProvider`) — Must be replicated in Rust to support task discovery and execution.
2. **Child process spawning** (`cp.exec`) — Build tools like gulp need subprocess integration; Rust equivalent would use `tokio::process::Command` or similar.
3. **File system operations** (`fs.promises.stat`, `path.join`) — Core I/O abstractions needed for gulpfile detection and workspace navigation.
4. **Configuration system** (`gulp.autoDetect` setting) — Configuration infrastructure must support extension-defined settings.
5. **Output channels** (`vscode.window.createOutputChannel`) — Logging/output infrastructure for task execution feedback.
6. **Localization system** (`vscode.l10n.t`) — i18n support must be maintained.

The gulp extension is a **task automation** subsystem—one of the core IDE functionalities mentioned in the research question. Porting would require Rust equivalents for extension registration, task scheduling, subprocess management, and IDE-to-extension communication protocols.

