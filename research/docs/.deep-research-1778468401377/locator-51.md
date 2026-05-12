# Locator Report: bootstrap-fork.ts

## Scope Analyzed
- `src/bootstrap-fork.ts` (229 LOC)

## Findings

### Implementation
- `src/bootstrap-fork.ts` — Electron process bootstrap entry point for forked worker processes; handles logging, error handling, parent process termination tracking, and crash reporting for child processes

## Summary

This single file in the scope represents the bootstrapping logic for Electron's forked child processes in VS Code. It is directly tied to Electron's process model and IPC mechanisms (`process.send`, `process.kill`, `process.env`). Porting VS Code from Electron to Tauri/Rust would require replacing this entire process fork orchestration with Tauri's command/event-based IPC model. No additional tests, types, configuration, or documentation related to this mechanism exist within this scope.
