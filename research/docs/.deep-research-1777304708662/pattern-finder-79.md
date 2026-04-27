# Pattern Research: gulpfile.mjs Build Entry Point

## Scope
This partition covers the root-level build orchestration pattern in gulpfile.mjs (5 LOC).

## Pattern: ESM Module Re-export for Build Task Delegation

**Where:** `gulpfile.mjs:5`

**What:** Root gulpfile delegates all build tasks to a TypeScript-based gulpfile in the build directory via ESM import.

```javascript
import './build/gulpfile.ts';
```

## Implications for Tauri/Rust Port

### Build System Architecture
The current VS Code build system uses Node.js/npm with Gulp as the orchestration layer:
- **Entry point**: `gulpfile.mjs` (ESM wrapper)
- **Task definitions**: `build/gulpfile.ts` (TypeScript)
- **Task runner**: Gulp 4.x with series/parallel composition
- **Compilation targets**: TypeScript→JavaScript transpilation (src→out), extensions, media assets, codicon fonts

### Tasks Identified in gulpfile.ts
- `compile-client`: Fast development compile (TypeScript→JS, assets, codicons)
- `watch-client`: Incremental compilation with file watching
- `transpile-client`: Full SWC transpilation pipeline
- `compile-extensions`: Extension bundling
- `compile`: Parallel execution of all compilation tasks
- Task composition uses `task.series()` and `task.parallel()` utilities

### Port Considerations

**Build system replacement needed**:
- Gulp/Node.js build orchestration would be replaced by Rust build tools (Cargo, Meson, or similar)
- TypeScript compilation would be handled by `swc` or `esbuild` compiled to Rust/WASM
- File watching and incremental compilation patterns would need equivalent in Rust tooling

**Asset pipeline**:
- Codicons (icon font) compilation
- Media asset bundling
- Extension compilation - these would need equivalent handling in a Rust-based system

**Electron removal**:
- Electron packaging removed; Tauri handles window/runtime via Rust backend
- Build artifacts (JS bundles, assets) would be served differently in Tauri's webview

This minimal entry point masks a complex TypeScript/Node.js build pipeline that would require substantial reengineering for a Rust/Tauri architecture, including new tooling for asset compilation, incremental builds, and extension handling.
