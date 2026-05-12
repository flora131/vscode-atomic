# VS Code Build Pattern: Gulp Re-export Entry Point

## Pattern: ESM Re-export Build Entry Point

**Where:** `gulpfile.mjs:5`

**What:** The root gulpfile uses ES module syntax to re-export a TypeScript-based gulp configuration, delegating all build orchestration to a compiled gulpfile in the build directory.

```javascript
import './build/gulpfile.ts';
```

**Variations / call-sites:**

The actual build tasks are defined in `build/gulpfile.ts`, which imports and chains together multiple gulp task categories:
- `monacoTypecheckTask` — Editor type checking (from `gulpfile.editor.ts`)
- `compileExtensionsTask` / `watchExtensionsTask` — Extension compilation (from `gulpfile.extensions.ts`)
- `compileTask`, `transpileTask`, `watchTypeCheckTask` — Client transpilation (from `lib/compilation.ts`)
- `copyCodiconsTask`, `compileApiProposalNamesTask`, `compileExtensionPointNamesTask` — Asset/metadata generation

The build system uses `task.series()` and `task.parallel()` to orchestrate concurrent/sequential compilation of:
1. Editor (Monaco)
2. Client (TypeScript → JavaScript in `out/`)
3. Extensions (compiled separately)
4. Extension media (static assets)

## Implications for Tauri/Rust Port

The root gulpfile pattern reveals that VS Code's build is fundamentally a **composition of independent task pipelines**:

1. **Modular task loading** — Tasks are split across multiple files and imported
2. **Runtime orchestration** — Gulp manages dependencies and parallelization
3. **ESM-based entry** — Modern JavaScript module system handles the delegation

A Tauri/Rust port would replace this entire pipeline with a different orchestrator (likely Cargo with custom build scripts, or a unified build system like `build.rs` and custom task runners). The key insight is that the current system treats build composition as a **runtime concern** (gulp tasks registered at startup), whereas Rust builds are typically **static/declarative** (Cargo.toml, build.rs, workspace definitions).

The 5-line gulpfile serves as the **single integration point** for all downstream build configuration—any Rust port must consolidate these scattered task definitions into a coherent build model, likely reducing the number of orchestration layers by moving configuration higher-level (Cargo metadata, workspace interdependencies, feature flags).
