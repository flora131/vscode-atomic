# Bootstrap Import System - File Location Map

## Scope Summary
This partition covers the bootstrap-import module used for early import shimming across VS Code processes (101 LOC).

## Implementation

### Core Bootstrap Module
- `src/bootstrap-import.ts` - ES module loader hook implementing Node.js Module.register protocol for redirecting dependency resolution. Exports `initialize()` and `resolve()` functions that intercept module loading to map package specifiers to node_modules paths.

### Related Bootstrap Infrastructure  
- `src/bootstrap-node.ts` - Node.js initialization module that registers the bootstrap-import loader via `devInjectNodeModuleLookupPath()` at line 62-74. Also provides utility functions for working directory setup, portable mode configuration, and global module path removal.
- `src/bootstrap-fork.ts` - Fork entry point that imports and uses `devInjectNodeModuleLookupPath` and `bootstrapESM` for logging pipe setup in forked processes.
- `src/bootstrap-esm.ts` - ES module setup that handles fs module mapping and NLS configuration for both Electron and Node.js environments.
- `src/main.ts` - Primary entry point that imports `configurePortable()` and `bootstrapESM()` from bootstrap modules during application initialization.

### Entry Point Modules That Reference Bootstrap System
- `src/cli.ts` - CLI entry point
- `src/server-cli.ts` - Server CLI entry point  
- `src/server-main.ts` - Server main entry point

## How It Works

The bootstrap-import system provides a Node.js loader hook mechanism:

1. **Registration**: `devInjectNodeModuleLookupPath()` in bootstrap-node.ts registers bootstrap-import.js as a loader hook using `Module.register()` (line 73)
2. **Initialization**: The `initialize()` function parses package.json files and builds mappings of specifier → file URL and specifier → module format
3. **Resolution**: The `resolve()` hook intercepts module resolution, returning redirected URLs for dependencies found in the mappings, deferring unknowns to the default resolver
4. **Scope**: Only activates when `VSCODE_DEV` environment variable is set (development mode)

## Configuration

No dedicated configuration files. Behavior controlled via:
- `VSCODE_DEV` environment variable - enables/disables the redirect system
- `ELECTRON_RUN_AS_NODE` environment variable - triggers fs → original-fs mapping in bootstrap-esm.ts
- `package.json` dependencies - scanned during initialization to build resolution mappings

## Notable Clusters

### Module Loading Infrastructure  
Contains 4 tightly-coupled bootstrap files in `src/`:
- bootstrap-import.ts (101 LOC) - Module resolution hooks
- bootstrap-node.ts (191 LOC) - Node.js setup and loader registration
- bootstrap-fork.ts - Forked process entry with logging
- bootstrap-esm.ts - ESM and Electron compatibility setup

These are invoked early from main entry points (main.ts, cli.ts, server-main.ts, server-cli.ts) during application startup sequence.

### Loader Hook Implementation Details
Uses Node.js Module.register() API (Node 20.6+) for dynamic loader registration. The system:
- Parses all package.json dependencies for entry points
- Supports both ESM (exports["."].import) and CommonJS (main) entry points  
- Handles conditional exports with fallback logic
- Infers module format from file extension and package.json type field
- Only applies during development (when running from source)

---

*Research context: Porting VS Code IDE functionality from TypeScript/Electron to Tauri/Rust requires understanding how the module loading system redirects dependencies, which would need equivalent resolution in Rust's module system (cargo dependencies, conditional compilation, feature flags).*
