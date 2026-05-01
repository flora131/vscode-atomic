# Locator 75: CLI Bootstrap Shim

## File Location
- `src/bootstrap-cli.ts` - Small CLI bootstrap shim (11 LOC)

## Purpose
Entry point that performs early environment initialization for the VS Code CLI. Specifically handles cleanup of the `VSCODE_CWD` environment variable to prevent shell escaping issues during command execution.

## Scope Summary
- **Files**: 1
- **Lines of Code**: 11
- **Type**: Initialization/bootstrap module

