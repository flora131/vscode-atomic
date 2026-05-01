# Partition 62: extensions/postinstall.mjs

## Implementation
- `extensions/postinstall.mjs` — Node.js maintenance script that runs post-npm-install to remove unnecessary TypeScript artifacts from the `node_modules/typescript` package (deletes compiler binaries and redundant type definitions, keeping only library definitions needed for HTML and extension editing)

## Summary

The `extensions/postinstall.mjs` is a single-file build pipeline maintenance script executed after `npm install` completes. Its sole responsibility is to clean up the TypeScript package in `node_modules` by:

1. Removing non-essential TypeScript distribution files from the root (`tsc.js`, `typescriptServices.js`, etc.)
2. Pruning TypeScript lib directory to keep only core type definitions (`lib.d.ts`, `lib.*.d.ts`, `protocol.d.ts`) and the main library module (`typescript.js`, `typescript.d.ts`)

**Porting implication**: A Rust-based VS Code implementation would require equivalent post-build cleanup of the Node.js dependency tree. If Tauri/Rust retains Node.js tooling for built-in extension support, this postinstall behavior must be replicated in the Rust build pipeline (e.g., via a build script or custom installer logic). If the architecture eliminates npm dependencies entirely, this script becomes obsolete.
