# Partition 78: bootstrap-server.ts

## Implementation

- `/Users/norinlavaee/vscode-atomic/src/bootstrap-server.ts` (7 LOC) — Server process bootstrap shim

The file contains a single statement that removes the `ELECTRON_RUN_AS_NODE` environment variable. This initialization must execute before other imports, as noted in both `/Users/norinlavaee/vscode-atomic/src/server-main.ts` (line 6) and `/Users/norinlavaee/vscode-atomic/src/server-cli.ts` (line 6) where it is imported with the comment "this MUST come before other imports as it changes global state".

## Tests

Related server test files exist but none specifically test bootstrap-server:
- `/Users/norinlavaee/vscode-atomic/src/vs/server/test/node/serverMain.test.ts`
- `/Users/norinlavaee/vscode-atomic/src/vs/server/test/node/serverConnectionToken.test.ts`
- `/Users/norinlavaee/vscode-atomic/src/vs/server/test/node/serverLifetimeService.test.ts`
- `/Users/norinlavaee/vscode-atomic/src/vs/server/test/node/serverAgentHostManager.test.ts`

## Configuration

- `/Users/norinlavaee/vscode-atomic/eslint.config.js` (line 1978) — Lint rules include bootstrap-server.ts in target configuration

## Notable Clusters

**Bootstrap ecosystem**: Five related bootstrap files initialize different node process contexts:
- `bootstrap-cli.ts` — CLI process initialization
- `bootstrap-node.ts` — Generic Node.js process setup (path handling, signal handlers, working directory)
- `bootstrap-esm.ts` — ESM module resolution hooks (fs → original-fs remapping, NLS setup, globals)
- `bootstrap-fork.ts` — Child process forking
- `bootstrap-server.ts` — Server-specific initialization (Electron environment variable removal)

All bootstrap files are imported at the very start of their respective entry points (server-main.ts, server-cli.ts) before any other imports.

**Server entry points** that depend on bootstrap-server:
- `/Users/norinlavaee/vscode-atomic/src/server-main.ts` — Main server bootstrap and initialization
- `/Users/norinlavaee/vscode-atomic/src/server-cli.ts` — CLI spawning and extension management

---

## Summary

The `bootstrap-server.ts` partition is a minimal (~7 lines) Electron-environment cleanup shim. Its singular responsibility is removing the `ELECTRON_RUN_AS_NODE` flag before server initialization. To port VS Code's server bootstrap to Tauri/Rust would require:

1. **Removing Electron environment dependencies** — The deleted env var is only relevant to Electron's dual Node/Electron runtime modes. Tauri runs native Rust, so this flag has no meaning.
2. **Maintaining bootstrap ordering** — The import-order requirement would persist; Rust's module system would need to ensure global state initialization occurs before dependent code paths.
3. **Replacing ESM/CommonJS module hooks** — `bootstrap-esm.ts` uses Node's `register()` API to redirect fs module imports; Rust's module system handles this natively.
4. **Adapting shell environment and path setup** — `bootstrap-node.ts` handles working directory, SIGPIPE signals, and module resolution paths; Rust equivalents exist but require different APIs.

The bootstrap-server partition itself is technically unnecessary in a Tauri context since Electron environment variables wouldn't exist. However, the broader bootstrap pattern—prioritizing global state setup before application logic—remains valid and critical.
