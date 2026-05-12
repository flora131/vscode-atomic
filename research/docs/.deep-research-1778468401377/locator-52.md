## File Locations for VS Code Core IDE Porting Research (Scope: bootstrap-node.ts)

### Implementation
- `src/bootstrap-node.ts` — Node.js bootstrap initialization for VS Code processes; handles SIGPIPE management, working directory setup, module lookup redirection, and portable mode configuration for Electron-based runtime

### Related Support Files
- `src/bootstrap-import.ts` — Node.js module loader hook for redirecting package imports to local node_modules; enables development-time module redirection via Module.register()
- `src/bootstrap-meta.ts` — Product and package configuration bootstrap that injects build-time metadata and development overrides

---

## Summary

The `bootstrap-node.ts` file (190 LOC) establishes Node.js-level initialization for VS Code's Electron runtime. It handles platform-specific concerns including SIGPIPE signal management (Electron workaround), working directory establishment across Windows/macOS/Linux, module path isolation to prevent global package pollution, and portable installation mode detection. This bootstrap is foundational infrastructure for VS Code's current Node/Electron architecture and demonstrates platform-specific runtime configuration that would require substantial reimplementation in a Tauri/Rust port. The file depends on the product configuration interface and works alongside two complementary bootstrap files: `bootstrap-import.ts` for dynamic module resolution and `bootstrap-meta.ts` for metadata injection.

