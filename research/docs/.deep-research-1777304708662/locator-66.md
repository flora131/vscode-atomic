# Locator Report: Partition 66 — Server CLI (Tauri/Rust Port Research)

## Scope
- `src/server-cli.ts` (1 file, 30 LOC)

## Implementation
- `src/server-cli.ts` — Bootstrap entry point for VS Code server mode; loads Node.js runtime configuration (NLS, ESM), module paths, and delegates to core server implementation (`vs/server/node/server.cli.js`)

## Summary
The single file in this partition is `src/server-cli.ts`, a thin TypeScript bootstrapper for VS Code's server CLI mode. It handles:
- Node.js global state initialization via `bootstrap-server.js`
- National Language Support (NLS) configuration
- Development environment path injection for module resolution
- ES Module bootstrapping
- Delegation to the actual server CLI implementation

**Relevance to Tauri/Rust port:** This file is a Node.js runtime initialization shim. In a Tauri/Rust port, the entire bootstrap and runtime initialization sequence would be replaced with Rust initialization code. This represents the JavaScript side of process startup logic that would need architectural redesign for a compiled Rust target. The server CLI module itself (`vs/server/node/server.cli.js`) is outside this partition's scope but would be the critical piece containing the actual server logic needing porting.
