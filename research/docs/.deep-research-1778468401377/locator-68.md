# Locator 68: JS Shim Launching Electron → Rust CLI Replacement

## File Scope
- **SCOPE**: `src/cli.ts` (~26 LOC)

## Key Finding: JS Shim Dispatch Pattern

The file `/home/norinlavaee/projects/vscode-atomic/src/cli.ts` is a minimal TypeScript entry point that delegates all actual CLI work to downstream modules. It contains **no Electron spawn calls itself** — it is purely a bootstrap harness.

### Implementation

- `/home/norinlavaee/projects/vscode-atomic/src/cli.ts` — 26-line shim that sets up NLS, portability, and CLI environment variables, then imports the actual CLI handler at line 26: `await import('./vs/code/node/cli.js')`

- `/home/norinlavaee/projects/vscode-atomic/src/vs/code/node/cli.ts` — The actual JavaScript CLI entry point. Contains the spawn logic:
  - **Line 6**: imports `spawn`, `SpawnOptions`, `StdioOptions` from `child_process`
  - **Line 73**: `spawn('cargo', ['run', '--', subcommand, ...tunnelArgs], ...)` for dev mode tunnels
  - **Line 80**: `spawn(tunnelCommand, [subcommand, ...tunnelArgs], ...)` for production tunnel binary
  - **Line 496**: `spawn(process.execPath, argv.slice(2), options)` for non-macOS Electron launch
  - **Line 571**: `spawn('open', spawnArgs, ...)` for macOS Electron launch (via `open` command)

### Bootstrap Chain

The scope file (`src/cli.ts`) orchestrates this boot sequence:
1. Line 6: imports `'./bootstrap-cli.js'` (clears VSCODE_CWD)
2. Line 7: imports `configurePortable()` from `bootstrap-node.js`
3. Line 8: imports `bootstrapESM()` from `bootstrap-esm.js`
4. Line 9: resolves NLS configuration
5. Line 17: configures portable support
6. Line 20: sets `VSCODE_CLI='1'` environment variable
7. Line 23: calls `bootstrapESM()`
8. Line 26: imports and executes actual CLI code from `./vs/code/node/cli.js`

### Rust Replacement Locations

The Rust CLI in `cli/` replaces the TypeScript JS shim and CLI logic entirely:

- `/home/norinlavaee/projects/vscode-atomic/cli/src/bin/code/main.rs` — Entry point for the Rust-based code CLI. Uses `std::process::Command` (equivalent to spawn) at function `start_code()` to launch the Electron/desktop application.

- `/home/norinlavaee/projects/vscode-atomic/cli/src/commands/` — Contains 20+ subcommand implementations (agent, tunnels, update, version, etc.) that replace the TypeScript dispatch logic.

## Related Bootstrap Infrastructure

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-cli.ts` — Clears VSCODE_CWD env var (12 LOC)
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-node.ts` — Configures portable mode
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts` — ESM module setup
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-meta.ts` — Product metadata
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-import.ts` — Import hook setup
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-fork.ts` — Fork support

## Summary

The query `ast-grep --lang ts -p 'spawn($$$)'` targets process launching patterns in TypeScript. Within the specified SCOPE (`src/cli.ts`), no spawn calls exist—the file is purely a 26-line bootstrap orchestrator. The actual Electron spawn logic resides downstream in `/home/norinlavaee/projects/vscode-atomic/src/vs/code/node/cli.ts` (4 spawn sites: cargo for tunnels, tunnel binary for production, Node executable for non-macOS, and macOS `open` command).

The Rust CLI port (`cli/src/bin/code/main.rs` and subcommands) replaces both the shim and the underlying TypeScript CLI entirely, providing native process launching without Node.js or Electron's `child_process` module.
