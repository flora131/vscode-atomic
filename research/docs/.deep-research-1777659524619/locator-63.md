# Partition 63: bootstrap-meta.ts

## Research Context
Investigating what it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust. The bootstrap layer is critical—it initializes product metadata, configuration, and prepares the runtime environment before any module loading occurs.

## File Scope
Single file examined: `src/bootstrap-meta.ts` (55 LOC)

---

## Implementation

### Core Bootstrap Files
- `src/bootstrap-meta.ts` — Early-stage bootstrap shim that loads and exports product and package metadata (version, name, channel, platform-specific identifiers, build stamps) before module initialization. Patched at build time with `BUILD_INSERT_PRODUCT_CONFIGURATION` and `BUILD_INSERT_PACKAGE_CONFIGURATION` markers. Handles embedded app configurations, sub-product overrides, and development-time product overrides.

### Related Bootstrap Ecosystem
- `src/bootstrap-esm.ts` — Imports `bootstrap-meta.ts` immediately, exposes product/pkg as globals (`globalThis._VSCODE_PRODUCT_JSON`, `globalThis._VSCODE_PACKAGE_JSON`) for downstream consumers. Sets up module resolution hooks, NLS initialization.
- `src/bootstrap-node.ts` — Companion bootstrap file (portable configuration setup).
- `src/main.ts` — Electron entry point that imports and uses `product` from `bootstrap-meta.js` to configure portable paths and platform-specific app behaviors.
- `src/server-cli.ts`, `src/server-main.ts`, `src/cli.ts` — Server/CLI entry points that also import bootstrap-meta metadata.

### Type Definitions
- `src/vs/base/common/product.ts` — Defines `IProductConfiguration` interface used by bootstrap-meta.ts. Comprehensive type covering:
  - Version, commit, quality metadata
  - Platform-specific identifiers (win32RegValueName, darwinBundleIdentifier, urlProtocol)
  - Data folder names and shared data folder names
  - Built-in extensions, walkthroughs, featured extensions metadata
  - URLs (download, update, web endpoints)
  - Sandbox, native support, telemetry configuration
  - Configuration sync stores, policy handling
  
- `src/vs/base/common/platform.ts` — Provides `INodeProcess` type used to detect embedded app environments.

### Product Configuration Consumers
- `src/vs/platform/product/common/product.ts` — Runtime product resolution logic for both web and native contexts. Falls back through: sandbox configuration → global `_VSCODE_PRODUCT_JSON` → build-time injected config → development defaults.

---

## Summary

**bootstrap-meta.ts** is a critical early-stage loader that:

1. **Loads build-stamped metadata**: Reads `product.json` and `package.json` at startup, with build-time placeholder injection points (`BUILD_INSERT_PRODUCT_CONFIGURATION`, `BUILD_INSERT_PACKAGE_CONFIGURATION`).

2. **Handles platform-specific identity**: Preserves OS-specific identifiers (Windows registry values, macOS bundle identifiers, URL protocol handlers) and manages policy configuration inheritance for embedded apps.

3. **Supports configuration layering**: Merges sub-product configs (`product.sub.json`), development overrides (`product.overrides.json`), and embedded app variants when running in embedded contexts.

4. **Exports as module contracts**: Exposes `product` and `pkg` objects that are then globalized by bootstrap-esm.ts and consumed throughout the runtime initialization chain.

**Relevance to Tauri/Rust port**: This metadata-injection pattern is fundamental to VS Code's multi-platform strategy. A Rust/Tauri port would need equivalent bootstrap mechanisms to:
- Inject version, commit hash, and build metadata at startup
- Manage Windows registry values, macOS bundle identifiers, and protocol handler registration
- Support embedded/nested app scenarios
- Provide development vs. production configuration branching
- Make this data available globally before the main app logic executes

The bootstrap-meta pattern demonstrates that VS Code's platform abstraction begins not in the core logic, but at the earliest initialization phase—metadata must be available before modules load.
