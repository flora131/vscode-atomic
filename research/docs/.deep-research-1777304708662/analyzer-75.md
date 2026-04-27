### Files Analysed

- `/Users/norinlavaee/vscode-atomic/src/bootstrap-cli.ts` (11 LOC, the sole partition file)

---

### Per-File Notes

#### `src/bootstrap-cli.ts`

- **Role:** A single-statement side-effect module whose only purpose is to scrub the `VSCODE_CWD` environment variable from the current Node.js process before any other module is imported. It acts as the first guard in the CLI launch sequence to prevent environment leakage from a parent shell session. The comment in the file explicitly ties this to GitHub issue #126399, where invoking `code .` from a shell that had previously launched VS Code could inherit a stale `VSCODE_CWD`, causing the editor to resolve the working directory incorrectly.

- **Key symbols:**
  - `process.env['VSCODE_CWD']` (`bootstrap-cli.ts:11`) — the environment variable being deleted; no named binding is introduced, the `delete` operator is applied directly to the live `process.env` dictionary.

- **Control flow:** There is no conditional logic, no function declaration, and no exported symbol. The file consists of exactly one executable statement at the module's top level. Because ES module evaluation is sequential and import-order guaranteed in Node.js, placing this import first in `src/cli.ts` ensures the deletion runs before any downstream module can observe or cache the variable's value. The comment in `src/cli.ts:7` explicitly enforces this ordering contract: `import './bootstrap-cli.js'; // this MUST come before other imports as it changes global state`.

- **Data flow:**
  1. Before this module executes: `process.env['VSCODE_CWD']` may hold a string value set by the parent shell from a previous VS Code invocation (the leak described in #126399).
  2. After `delete process.env['VSCODE_CWD']` at line 11: the key is completely removed from the environment dictionary, making any subsequent `process.env['VSCODE_CWD']` lookup return `undefined`.
  3. Downstream in `src/bootstrap-node.ts:42-43`, `setupCurrentWorkingDirectory()` re-stamps `VSCODE_CWD` with the current, correct value of `process.cwd()` — but only when the key is absent. The deletion performed here guarantees that conditional check always triggers a fresh write for CLI launches.

- **Dependencies:** None. The module imports nothing. It relies solely on the Node.js built-in `process` global (no `require`/`import` of `process` is needed in Node.js ESM because it is a global). There are no type imports, no third-party packages, and no VS Code platform abstractions involved.

---

### Cross-Cutting Synthesis (≤200 words)

`bootstrap-cli.ts` is the simplest possible example of an early-bootstrap side-effect pattern: one `delete` statement, no imports, no exports, no branching. Its power comes entirely from import ordering. The broader lifecycle around `VSCODE_CWD` is a three-phase protocol: (1) the CLI bootstrap scrubs any inherited value via this file; (2) `bootstrap-node.ts:setupCurrentWorkingDirectory()` stamps the correct `process.cwd()` into the variable after the scrub; (3) downstream consumers — `src/vs/base/common/process.ts:29`, `src/vs/base/parts/sandbox/electron-browser/preload.ts:207`, and `src/vs/server/node/remoteTerminalChannel.ts:63` — all read `VSCODE_CWD` as the authoritative working directory rather than calling `process.cwd()` directly, because on Windows the CWD is explicitly changed to the application folder in step 2. For a Tauri/Rust port this means the native launcher binary must replicate phase 1 before spawning any renderer or extension-host process: call `std::env::remove_var("VSCODE_CWD")` unconditionally in `main()`, then re-set it to the correct path before the child process inherits the environment.

---

### Out-of-Partition References

- `/Users/norinlavaee/vscode-atomic/src/cli.ts` — imports `bootstrap-cli.js` as its **first** import statement (line 7), enforcing the scrub runs before any other module initializes; also sets `VSCODE_CLI=1` and `VSCODE_NLS_CONFIG` later in the same file.
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts` — contains `setupCurrentWorkingDirectory()` (lines 35–49) which re-stamps `VSCODE_CWD` with `process.cwd()` when the key is absent, and on Windows additionally calls `process.chdir()` to move the process into the application folder; this is the phase-2 write that relies on the phase-1 delete done in `bootstrap-cli.ts`.
- `/Users/norinlavaee/vscode-atomic/src/vs/base/common/process.ts` — exposes a platform-safe `cwd()` shim at line 29 that reads `process.env['VSCODE_CWD']` with a fallback to `process.cwd()`; used throughout the VS Code renderer and extension-host layers as the canonical CWD source.
- `/Users/norinlavaee/vscode-atomic/src/vs/platform/environment/node/userDataPath.ts` — reads `VSCODE_CWD` at line 20 to resolve the user-data path; receives the clean, freshly-stamped value because the scrub and re-stamp have already occurred by the time this platform service initialises.
- `/Users/norinlavaee/vscode-atomic/src/vs/base/parts/sandbox/electron-browser/preload.ts` — reads `VSCODE_CWD` at line 207 inside the Electron sandbox preload to derive the initial working directory for renderer processes, falling back to the directory containing `process.execPath` when absent.
- `/Users/norinlavaee/vscode-atomic/src/vs/server/node/remoteTerminalChannel.ts` — reads `env['VSCODE_CWD']` at line 63 in the remote terminal channel to set the initial working directory for remote shell sessions opened from the CLI path.

---

The entire `bootstrap-cli.ts` file is a ten-word operation that encodes a precise contract: the CLI process must never inherit a `VSCODE_CWD` value from the environment of the shell that invoked it. Porting this to a Tauri/Rust launcher requires reproducing the same guarantee in native code — calling `std::env::remove_var("VSCODE_CWD")` as the very first statement in `main()`, before `tauri::Builder` is constructed or any child process is spawned — because the three downstream consumers (`process.ts`, `userDataPath.ts`, `preload.ts`, `remoteTerminalChannel.ts`) all trust that whatever value reaches them in `VSCODE_CWD` was placed there intentionally by the bootstrap layer, not inherited from an unrelated shell session.
