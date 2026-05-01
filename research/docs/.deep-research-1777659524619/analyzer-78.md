### Files Analysed

- `src/bootstrap-server.ts` (7 LOC, read in full)

---

### Per-File Notes

#### `src/bootstrap-server.ts`

- **Role:** A one-line server-side bootstrap shim that removes the `ELECTRON_RUN_AS_NODE` environment variable from the Node.js process before any other module loads, so that subsequent bootstrap modules do not behave as if they are running inside Electron.

- **Key symbols:**
  - No exported functions, classes, or variables. The file's entire effect is the single side-effecting statement at `src/bootstrap-server.ts:7`.

- **Control flow:**
  - There is no branching or conditional logic. When the module is imported, Node.js executes its top-level code immediately. The single executable statement is:
    ```
    delete process.env['ELECTRON_RUN_AS_NODE'];   // bootstrap-server.ts:7
    ```
  - The inline comment at `bootstrap-server.ts:6` explains the motivation: it prevents `bootstrap-esm.js` from redefining the built-in `fs` module. That re-definition only occurs when `ELECTRON_RUN_AS_NODE` is present (Electron sets this flag to signal that a child Node process is being run under Electron's patched environment).

- **Data flow:**
  - **Input:** `process.env['ELECTRON_RUN_AS_NODE']` — a string value (typically `"1"`) set by Electron when spawning a server child process.
  - **Mutation:** The `delete` operator removes the key from `process.env` in-place; `process.env` is a global singleton shared across the entire Node.js process.
  - **Output:** Nothing is exported; the only observable effect is the absence of the environment variable for all code that runs after this import.
  - **State location:** `process.env` (the Node.js process environment object).

- **Dependencies:**
  - No `import` or `require` statements. The file depends solely on the Node.js built-in global `process`.

---

### Cross-Cutting Synthesis

`src/bootstrap-server.ts` encapsulates a single Electron-isolation concern: the VS Code server (used for remote development over SSH, containers, or Codespaces) runs as a plain Node.js process, but its entry points (`src/server-main.ts`, `src/server-cli.ts`) may be launched by an Electron host that has already injected `ELECTRON_RUN_AS_NODE=1` into the environment. The downstream `bootstrap-esm.ts` module uses the presence of that variable as a signal to patch Node.js internals (notably `fs`), which would be wrong in a pure-server context. By deleting the variable as the very first act of the server process (before any other module loads), this shim keeps the Electron-specific patching path from executing.

For a Tauri/Rust port this is directly instructive: the pattern demonstrates that VS Code's server tier is already architecturally separated from Electron. The server has its own entry points and its own bootstrap chain, and the only Electron coupling is this one environment-variable guard. A Tauri back-end would simply never set `ELECTRON_RUN_AS_NODE`, rendering this shim unnecessary — but the existence of the shim confirms that the server-side Node.js runtime is already designed to run in a non-Electron host.

---

### Out-of-Partition References

- `src/bootstrap-esm.ts` — Contains the `fs`-redefining logic that is suppressed by this shim; its behaviour forks on `ELECTRON_RUN_AS_NODE`.
- `src/bootstrap-node.ts` — Companion bootstrap that handles other Node.js-level patches; called before or alongside `bootstrap-server.ts` in the server entry points.
- `src/server-main.ts` — Primary server entry point; imports `bootstrap-server.ts` as its first statement so the env-var deletion happens before anything else.
- `src/server-cli.ts` — CLI-oriented server entry point; also imports `bootstrap-server.ts` first for the same reason.
- `src/vs/server/node/` — The full VS Code server infrastructure (extension host, remote file system, IPC channels for language intelligence, debugging, terminal) whose correct initialisation depends on this shim running first.
