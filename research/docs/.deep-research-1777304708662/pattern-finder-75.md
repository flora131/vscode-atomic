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
