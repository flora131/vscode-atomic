# bootstrap-cli.ts File Location Analysis

## File Location
- `src/bootstrap-cli.ts` (11 LOC)

## Implementation

### Primary File
- `src/bootstrap-cli.ts` — A minimal bootstrap shim that early-deletes the `VSCODE_CWD` environment variable before other imports execute. This addresses a historical issue (https://github.com/microsoft/vscode/issues/126399) where `code .` could use the wrong current working directory due to environment variable leakage to parent shell.

### Integrations
- `src/cli.ts` — The CLI entry point that imports `bootstrap-cli.js` as the first import with an explicit comment: "this MUST come before other imports as it changes global state"

### Related Bootstrap Files
The following bootstrap shims are in the same directory and share similar patterns:
- `src/bootstrap-fork.ts`
- `src/bootstrap-node.ts` 
- `src/bootstrap-server.ts`
- `src/bootstrap-import.ts`
- `src/bootstrap-meta.ts`
- `src/bootstrap-esm.ts`

## Environment References

Files that reference the `VSCODE_CWD` environment variable include:
- `src/vs/platform/environment/node/userDataPath.ts`
- `src/vs/server/node/remoteTerminalChannel.ts`
- `extensions/copilot/src/util/vs/base/common/process.ts`
- `test/unit/electron/preload.js`
- `src/vs/base/parts/sandbox/electron-browser/preload.ts`
- `src/vs/base/common/process.ts`
- `src/bootstrap-node.ts`

## Summary

`bootstrap-cli.ts` is a lightweight initialization module (11 lines of code) that performs a critical early cleanup task: deleting the `VSCODE_CWD` environment variable before any other imports occur. Its primary purpose is to prevent environment variable leakage to parent shells when launching VS Code as a CLI tool with `code .`. This file represents the minimal "guard at the gate" pattern where early initialization must occur before the rest of the codebase loads. No tests, types, configuration, or examples are directly associated with this specific file—it serves as a pure-function bootstrap that modifies process state and then yields control to subsequent modules.

