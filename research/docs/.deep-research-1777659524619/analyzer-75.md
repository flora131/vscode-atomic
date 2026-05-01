# Analyzer Research: `src/bootstrap-cli.ts` — CLI Bootstrap Shim

## Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-cli.ts` (11 LOC)
- `/home/norinlavaee/projects/vscode-atomic/src/cli.ts` (referenced as importer)
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-node.ts` (lines 34-55, `VSCODE_CWD` producer)

---

## Per-File Notes

### `src/bootstrap-cli.ts` (lines 1–11)

**Role:** A single-statement early-stage environment cleanup shim that removes the `VSCODE_CWD` environment variable from `process.env` before any other CLI module is loaded.

**Key symbol:**

- `delete process.env['VSCODE_CWD']` — `src/bootstrap-cli.ts:11`

**Control flow:**

The file has no imports, no functions, no conditional logic, and no exports. Its sole executable statement executes immediately when the module is imported. The ES module `import` of `./bootstrap-cli.js` in `src/cli.ts:6` is annotated with the comment `// this MUST come before other imports as it changes global state`, confirming that timing is enforced by import order.

**Data flow:**

- **Input:** `process.env['VSCODE_CWD']` — a string that may have been set by a parent process (e.g., by `setupCurrentWorkingDirectory()` in `src/bootstrap-node.ts:42-43`, which writes `process.cwd()` into `VSCODE_CWD` for non-CLI processes).
- **Mutation:** `delete process.env['VSCODE_CWD']` at `src/bootstrap-cli.ts:11` removes the key entirely from the process environment object.
- **Output:** All subsequent modules in the CLI process see `process.env['VSCODE_CWD']` as `undefined`.

**Why this matters:** Without this deletion, `VSCODE_CWD` set by a parent VS Code process (e.g., the Electron main process) can escape into the child CLI process via inherited environment, causing `code .` to resolve the wrong working directory (tracked in microsoft/vscode#126399). The variable is consumed in multiple downstream locations:

- `src/vs/base/common/process.ts:29` — `cwd()` returns `process.env['VSCODE_CWD'] || process.cwd()`
- `src/vs/platform/environment/node/userDataPath.ts:20` — reads `VSCODE_CWD` for user data path resolution
- `src/vs/server/node/remoteTerminalChannel.ts:63` — reads `VSCODE_CWD` from remote terminal env

**Dependencies:** None. No imports. The file relies only on the Node.js built-in `process` global.

---

## Cross-Cutting Synthesis

`src/bootstrap-cli.ts` is a 1-line environment sanitization module. Its entire purpose is to clear `VSCODE_CWD` before any other module can read it, preventing an inherited-env pollution bug where the variable set by `setupCurrentWorkingDirectory()` in `src/bootstrap-node.ts:42-43` (for the GUI/server process) bleeds into the separately spawned CLI process. The import ordering in `src/cli.ts:6` (first import, before `bootstrap-node.js`, `bootstrap-esm.js`, or NLS setup) is the mechanism that guarantees the deletion happens before any code can observe the variable.

For a Rust/Tauri port: the Rust CLI entry point must explicitly unset `VSCODE_CWD` from its inherited environment at process startup, before spawning any subprocesses or resolving any paths. In Rust this maps to `std::env::remove_var("VSCODE_CWD")` called at the top of `main()` before any other initialization, mirroring the import-order guarantee that TypeScript's module system provides here.

---

## Out-of-Partition References

- `src/cli.ts:6` — imports `./bootstrap-cli.js` as its very first statement; sets `VSCODE_NLS_CONFIG` and `VSCODE_CLI` env vars after the cleanup.
- `src/bootstrap-node.ts:34-55` — `setupCurrentWorkingDirectory()` is the producer of `VSCODE_CWD`; runs in non-CLI processes (main Electron process, server) and writes `process.cwd()` into the variable.
- `src/vs/base/common/process.ts:29` — downstream consumer of `VSCODE_CWD` for `cwd()` resolution.
- `src/vs/platform/environment/node/userDataPath.ts:20` — downstream consumer for user data path calculation.
- `src/vs/server/node/remoteTerminalChannel.ts:63` — reads `VSCODE_CWD` from spawned terminal environment.
- `src/vs/base/parts/sandbox/electron-browser/preload.ts:207` — renderer-side consumer of `VSCODE_CWD` for path fallback.
