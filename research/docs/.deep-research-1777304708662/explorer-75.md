# Partition 75 of 79 — Findings

## Scope
`src/bootstrap-cli.ts/` (1 files, 11 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Analysis: `src/bootstrap-cli.ts`

## Scope
- `src/bootstrap-cli.ts` (11 LOC)
- Single-purpose bootstrap shim imported at CLI entry point

---

#### Pattern: Early Environment Variable Deletion

**Where:** `src/bootstrap-cli.ts:6-11`

**What:** Preventive deletion of `VSCODE_CWD` environment variable at the earliest point of CLI execution to prevent shell state pollution. This is a defensive measure executed before any other imports.

```typescript
// Delete `VSCODE_CWD` very early. We have seen
// reports where `code .` would use the wrong
// current working directory due to our variable
// somehow escaping to the parent shell
// (https://github.com/microsoft/vscode/issues/126399)
delete process.env['VSCODE_CWD'];
```

**Variations / call-sites:**
- `src/cli.ts:6` - imported as first statement: `import './bootstrap-cli.js'; // this MUST come before other imports as it changes global state`
- Similar pattern used in `src/bootstrap-server.ts:7` for `ELECTRON_RUN_AS_NODE` deletion
- `src/server-cli.ts:23` and `src/server-main.ts:244` also delete `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH`
- `src/vs/platform/terminal/node/ptyHostMain.ts:39-43` deletes multiple terminal-specific env vars at initialization

---

#### Pattern: Mandatory Early Import Guard

**Where:** `src/cli.ts:6` (related to bootstrap-cli)

**What:** Bootstrap modules are imported as side-effect imports before any functional code, with explicit comments indicating this changes global state. The pattern ensures environment cleanup happens before dependent modules load.

```typescript
// cli.ts
import './bootstrap-cli.js'; // this MUST come before other imports as it changes global state
import { configurePortable } from './bootstrap-node.js';
import { bootstrapESM } from './bootstrap-esm.js';
```

**Variations / call-sites:**
- `src/bootstrap-cli.ts` - pure side-effect module (11 LOC, no exports)
- `src/bootstrap-server.ts:7` - another side-effect-only bootstrap that deletes `ELECTRON_RUN_AS_NODE`
- `src/bootstrap-fork.ts:6-8` - imports performance monitoring and other bootstraps
- `src/bootstrap-esm.ts:6-29` - registers module resolution hooks early
- Contrast with conditional setup in `src/bootstrap-node.ts:35-55` which wraps logic in a function

---

#### Pattern: Cross-Process Environment State Management

**Where:** `src/bootstrap-node.ts:32-55` (context for bootstrap-cli)

**What:** Dual-phase environment variable lifecycle: initial capture in `bootstrap-node.ts` sets `VSCODE_CWD` from `process.cwd()`, then `bootstrap-cli.ts` deletes it later in CLI path to prevent escaping to parent shell.

```typescript
// bootstrap-node.ts (runs in all processes)
function setupCurrentWorkingDirectory(): void {
    try {
        // Store the `process.cwd()` inside `VSCODE_CWD`
        // for consistent lookups, but make sure to only
        // do this once unless defined already from e.g.
        // a parent process.
        if (typeof process.env['VSCODE_CWD'] !== 'string') {
            process.env['VSCODE_CWD'] = process.cwd();
        }
        // Windows: always set application folder as current working dir
        if (process.platform === 'win32') {
            process.chdir(path.dirname(process.execPath));
        }
    } catch (err) {
        console.error(err);
    }
}

setupCurrentWorkingDirectory();
```

**Variations / call-sites:**
- `src/bootstrap-node.ts:42-43` - conditional set (only if not already present)
- `src/bootstrap-cli.ts:11` - unconditional delete in CLI variant
- `test/unit/electron/preload.js:68` - reads `VSCODE_CWD` as fallback: `process.env['VSCODE_CWD'] || process.execPath.substr(...)`
- Terminal utilities reference `cwd` from process state

---

## Summary

The `src/bootstrap-cli.ts` file exemplifies a minimal but critical initialization pattern: **defensive early-stage environment cleanup**. It's a 11-line module that exists solely to execute a side effect (deleting `VSCODE_CWD`) before any functional code runs. This is motivated by a specific bug (GitHub issue #126399) where environment variables leaked into parent shell processes.

The pattern demonstrates:

1. **Separation of concerns**: Bootstrap modules contain only initialization logic, no business logic
2. **Execution order guarantees**: The comment "MUST come before other imports" enforces strict sequencing
3. **Bug mitigation through timing**: Rather than complex cleanup logic, the solution is to delete the variable at the right moment in the startup sequence
4. **Process variant handling**: Different bootstrap paths (cli.ts vs. server-cli.ts) have different cleanup requirements
5. **Defensive programming**: Even though `bootstrap-node.ts` sets `VSCODE_CWD` for internal use, the CLI variant unconditionally removes it to prevent leakage

For Tauri/Rust porting, this pattern highlights the need for **environment state isolation** between parent and spawned processes—a concern that may be differently addressed in Rust's systems programming paradigm and Tauri's process model.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
