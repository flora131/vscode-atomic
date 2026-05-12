# File Locator Report: Partition 76/80 - CLI Bootstrap Shim

## Scope
- `src/bootstrap-cli.ts` (1 file, 11 LOC)

## Summary
The CLI bootstrap shim (`src/bootstrap-cli.ts`) is a minimal entry point file containing only environment variable cleanup. The file does not contain argument parsing logic, command-line option handling, or process argument (`process.argv`) inspection. For a Tauri/Rust port, this file represents minimal surface area — only the `VSCODE_CWD` environment variable deletion would need porting to equivalent Rust initialization logic.

### Implementation
- `src/bootstrap-cli.ts` — Minimal bootstrap shim; deletes `VSCODE_CWD` environment variable to prevent working directory inheritance issues; no CLI argument parsing or option handling present

### Notable Findings
- No `process.argv` usage found (contrary to seed pattern expectation)
- No explicit command-line option parsing in this file
- Actual CLI argument handling likely resides in separate `src/cli.ts` module (outside scope)
- For Tauri/Rust porting: this file is trivial and would require minimal effort (simple env cleanup on startup)

