# File Locations for Tauri/Rust Port Research - bootstrap-meta.ts Scope

## Implementation
- `src/bootstrap-meta.ts` - Configuration metadata loader using Node.js `createRequire` and `import.meta` APIs. Loads product and package JSON files at runtime with build-time patching support.

## Types / Interfaces
- `src/vs/base/common/product.ts` - Exports `IProductConfiguration` interface defining platform-specific identifiers (win32RegValueName, darwinBundleIdentifier), application metadata, and configuration for desktop, web, and embedded environments.
- `src/vs/base/common/platform.ts` - Exports `INodeProcess` interface abstracting Node.js process object for cross-platform detection (Windows, macOS, Linux, Electron detection via `versions.electron` field).

## Related Implementation Context
- `src/bootstrap-esm.ts` - ESM module initialization that imports from bootstrap-meta.ts, sets up global product/package metadata, handles NLS (National Language Support) configuration.
- `src/bootstrap-node.ts` - Node.js-specific setup handling SIGPIPE, working directory configuration, and platform detection logic.
- `src/bootstrap-import.ts` - Module resolution hook using Node.js loader API for redirecting node_modules imports.

## Summary

The `src/bootstrap-meta.ts` file represents a critical abstraction point for porting VS Code to alternative runtimes like Tauri/Rust. Currently, it relies on:

1. **Node.js-specific APIs**: Uses `createRequire` from `node:module` and `import.meta.url` for ES module integration
2. **Build-time configuration injection**: Expects product and package JSON to be patched during the build process
3. **Runtime platform detection**: Depends on process object shape defined in `INodeProcess` interface
4. **Embedded application support**: Handles policy inheritance for embedded VS Code instances

A Tauri/Rust port would need to replace this entire bootstrap chain with a Rust-backed equivalent that:
- Provides similar product/package configuration loading
- Implements equivalent platform detection without Node.js process object
- Replaces ESM module hooks with Tauri's IPC-based module system
- Maintains backward compatibility with configuration metadata expected by downstream code

