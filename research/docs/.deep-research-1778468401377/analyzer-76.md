### Files Analysed

- `src/bootstrap-cli.ts` (11 LOC)

---

### Per-File Notes

#### `src/bootstrap-cli.ts`

- **Role:** Minimal process-level bootstrap shim that runs as the very first import in the CLI process. Its sole responsibility is to delete the `VSCODE_CWD` environment variable from the current process's environment before any other module has a chance to read it. The file contains no functions, no exports, no imports, and no conditional logic — it is entirely composed of a single top-level side-effectful statement.

- **Key symbols:**
  - `process.env['VSCODE_CWD']` (`src/bootstrap-cli.ts:11`) — The environment variable being unconditionally deleted at module load time.

- **Control flow:** There is no branching or looping. The single statement `delete process.env['VSCODE_CWD']` (`src/bootstrap-cli.ts:11`) executes unconditionally and synchronously at module evaluation time. Execution is complete after this one statement; the module exports nothing and has no lifecycle hooks.

- **Data flow:** The only data involved is the value held in `process.env['VSCODE_CWD']` at the moment the module is evaluated. The `delete` operator removes the key from the `process.env` object entirely. No value is read, returned, or passed anywhere; the effect is purely a mutation (deletion) of the process environment inherited from the shell that launched the process.

  The comment at `src/bootstrap-cli.ts:6–10` references [GitHub issue #126399](https://github.com/microsoft/vscode/issues/126399), describing the scenario: running `code .` from a shell that already has `VSCODE_CWD` set (because a parent `code` process previously wrote it) would cause the CLI child process to inherit the wrong working directory. Deleting the variable before any downstream code reads it breaks that inheritance chain.

- **Dependencies:** None. No imports whatsoever. Relies only on the Node.js global `process` object, which is available in all Node.js module scopes without importing.

---

### Cross-Cutting Synthesis

`src/bootstrap-cli.ts` is the first import statement in `src/cli.ts:6`, annotated with the comment `// this MUST come before other imports as it changes global state`. This ordering constraint exists because `src/bootstrap-node.ts:42–43` (executed later in the same process via `src/cli.ts:7`) reads `process.env['VSCODE_CWD']` and, only if it is absent, sets it to `process.cwd()`. If `bootstrap-cli.ts` did not first delete any shell-inherited value, the stale parent-process value would survive into `bootstrap-node.ts`'s guard check and would never be overwritten. After `bootstrap-node.ts` sets the variable, it is subsequently consumed at `src/vs/base/common/process.ts:29`, `src/vs/platform/environment/node/userDataPath.ts:19`, `src/vs/base/parts/sandbox/electron-browser/preload.ts:207`, and `src/vs/server/node/remoteTerminalChannel.ts:63`. The full lifecycle is: delete (bootstrap-cli) → re-set from fresh `process.cwd()` (bootstrap-node) → read throughout the application. For a Tauri/Rust port, the equivalent is clearing any inherited `VSCODE_CWD` from `std::env` at the earliest point in `main()`, before any downstream initialization reads it.

---

### Out-of-Partition References

- `src/cli.ts:6` — Imports `./bootstrap-cli.js` as its first and most-constrained import; the inline comment makes the ordering requirement explicit.
- `src/bootstrap-node.ts:42–43` — Reads `process.env['VSCODE_CWD']` and sets it to `process.cwd()` only when absent; relies on `bootstrap-cli.ts` having already deleted any stale inherited value.
- `src/vs/base/common/process.ts:29` — Downstream consumer: `cwd()` returns `process.env['VSCODE_CWD'] || process.cwd()`.
- `src/vs/platform/environment/node/userDataPath.ts:19` — Downstream consumer: `const cwd = process.env['VSCODE_CWD'] || process.cwd()`.
- `src/vs/base/parts/sandbox/electron-browser/preload.ts:207` — Downstream consumer in the renderer/sandbox process.
- `src/vs/server/node/remoteTerminalChannel.ts:63` — Downstream consumer in the remote server path: `return env['VSCODE_CWD']`.
- `extensions/copilot/src/util/vs/base/common/process.ts:31` — Extension-local copy of the same pattern.
- `eslint.config.js:1998` — Lint configuration lists `bootstrap-cli.ts` as one of the root-level entry-point files subject to shared lint rules.
