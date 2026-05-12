# Locator 66: Server CLI Entry Point

## Scope
- `src/server-cli.ts` (31 LOC) — Server CLI entry point for VS Code server mode

## Implementation
- `src/server-cli.ts` — Root server CLI bootstrap; configures NLS, environment variables, and delegates to `/vs/server/node/server.cli.js` for main server logic

## Key Findings

**What this file does:**
- Acts as the top-level entry point for `code-server` (VS Code in server mode)
- Performs ESM bootstrap and NLS (Localization) configuration
- Sets up environment paths for Node modules in development mode
- Delegates actual server initialization to the compiled JavaScript entry point at `src/vs/server/node/server.cli.js`

**Relevance to Tauri/Rust porting:**
This file represents the minimal Node.js/Electron server CLI bootstrap. For a Rust/Tauri port, this would need to be replaced with:
1. A Rust binary entry point (likely `src/main.rs` in a Cargo project)
2. Rust-based NLS resolution and environment setup
3. Initialization of the core server logic in Rust instead of TypeScript

**No parseArgs pattern found:** The query seed mentioned looking for `parseArgs` calls, but this file contains no explicit argument parsing—it assumes command-line arguments are passed through to the delegated server.cli.js module.

## Summary

Partition 66 contains a single file (`src/server-cli.ts`) that serves as the thin bootstrapping layer for VS Code's server mode. It establishes ESM compatibility, NLS configuration, and development environment setup before forwarding control to the actual server implementation. This would be a key reference point for designing a Rust-based server entry point in a Tauri port.

