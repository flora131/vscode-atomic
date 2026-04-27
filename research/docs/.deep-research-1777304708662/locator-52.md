# Locator 52: Node Bootstrap Architecture — Tauri/Rust Porting Constraints

## Scope
Single file analysis: `src/bootstrap-node.ts` (190 LOC)

## Implementation

**src/bootstrap-node.ts**
Core Node.js bootstrap module that initializes the runtime environment before VS Code main/renderer processes start. Contains:

- **Native Module Loader Setup** (lines 57-74): `devInjectNodeModuleLookupPath()` registers a loader hook via `Module.register()` that redirects Node module resolution paths during development
- **Global Module Path Filtering** (lines 76-128): `removeGlobalNodeJsModuleLookupPaths()` strips global node_modules paths from module lookup to enforce sandboxed resolution; contains Electron-specific branch (line 77-79) that detects and bypasses filtering when running under Electron
- **Platform-Specific SIGPIPE Handling** (lines 17-30): Workaround for Electron's missing SIGPIPE handler, critical for pipe-based communication
- **Working Directory Initialization** (lines 32-55): Cross-platform cwd setup with platform-specific behavior for Windows (lines 47-49)
- **Portable Mode Detection** (lines 130-190): `configurePortable()` enables self-contained installations with platform-specific data paths (macOS: nested app bundles, Windows/Linux: data subdirectories)

## Native Module Dependencies

Direct Node.js module requires:
- `node:path` — filesystem path manipulation
- `node:fs` — filesystem operations (used in portable mode detection, line 167)
- `node:module` — Module loader system, used in two forms:
  - `createRequire()` from module API (line 8)
  - `Module.register()` for loader hooks (line 73)
  - Direct `Module` access for internal APIs: `_resolveLookupPaths()`, `_nodeModulePaths()`, `globalPaths`

## Notable Clusters

**Electron Runtime Integration Points**:
- Line 77: `process.versions.electron` check — code branches on Electron presence
- Line 18: SIGPIPE workaround references Electron issue tracking
- Lines 78-79: Explicit Electron-aware module path filtering

**Module Resolution Hooks**:
- Lines 72-73: Uses ES modules loader API (`Module.register()` with import.meta.url)
- Lines 81, 84, 100-101: Patches internal Node.js Module methods (`_resolveLookupPaths`, `_nodeModulePaths`)

---

## Tauri/Rust Porting Implications

This single bootstrap file exposes critical constraints for TypeScript → Rust migration:

1. **Native Module System Dependency**: The code directly instruments Node's internal module resolution (`Module._resolveLookupPaths`, `Module._nodeModulePaths`) via monkey-patching. Tauri/Rust has no equivalent module system to patch; Rust compilation is static and cannot support runtime module redirection.

2. **ES Module Loader Hooks**: The `Module.register()` call (line 73) uses Node.js's stable loader hooks API, a Node-only runtime capability. Rust compilation cannot support this.

3. **Process-Level Signal Handling**: SIGPIPE interception (line 21) requires process-level signal handlers that must be set up at startup. Rust/Tauri would need equivalent signal handling, but the specific Electron+Node.js interaction is unique.

4. **Electron Version Detection**: Lines 77-79 explicitly gate behavior on Electron presence. A Tauri port would lose this dual-runtime support unless Tauri provides equivalent process introspection.

5. **Portable Mode Cross-Platform Logic**: The OS-specific path logic (macOS bundle nesting vs. Windows data folders) can translate to Rust, but would require equivalent path manipulation libraries and cross-platform testing infrastructure.

6. **Module Search Path Filtering**: The Windows-specific drive and home directory filtering (lines 109-127) patches Node's internal search logic; Rust's static module resolution cannot support equivalent runtime filtering.

The bootstrap-node.ts file is fundamentally tied to Node.js runtime semantics. A Tauri port would require a complete architectural redesign of how modules/libraries are discovered, loaded, and isolated at runtime.
