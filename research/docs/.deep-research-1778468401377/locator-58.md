# File Locator Report: `extensions/esbuild-common.mts`

## Implementation

**Core Bundler Pipeline**
- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-common.mts` (80 lines) — Central build/watch runner for extension esbuild scripts. Exports two main APIs:
  - `RunConfig` interface: Specifies source directory, output directory, entry points, and optional additional esbuild options
  - `runBuild()` async function: Orchestrates either single-pass esbuild compilation or watch-mode with parcel watcher, with optional post-build callback

**Watch Mode Implementation**
- Uses `@parcel/watcher` (v2.5.6) instead of esbuild's native watch for reduced idle CPU usage
- Implements debounced rebuild pattern (100ms debounce) when source files change
- Ignores `node_modules/**`, `dist/**`, `out/**` directories during watch

**Type Definitions**
- Minimal type interface `RunConfig` with required fields: `srcDir`, `outdir`, `entryPoints` (esbuild format), optional `additionalOptions`
- Relies on esbuild's own types for `BuildOptions` type hints

## Configuration

**Dependencies** (from `/home/norinlavaee/projects/vscode-atomic/extensions/package.json`)
- `esbuild@0.27.2` — Main bundler
- `@parcel/watcher@^2.5.6` — File watching for lower CPU overhead
- `typescript@^6.0.3` — For TS/MTS compilation support
- `node-gyp-build@4.8.1` — Override for native module builds

**Runtime Arguments**
- `--watch` flag: Activates watch mode instead of one-shot build
- `--outputRoot` flag: Overrides output directory root, preserving the original directory name

## Examples / Fixtures

**Consumer Pattern** — Extension build scripts (e.g., `/home/norinlavaee/projects/vscode-atomic/extensions/git/esbuild.mts`)
```
Imports: run() from esbuild-extension-common (which wraps esbuild-common)
Calls with: RunConfig + process.argv + optional post-build callback
Typical usage: Define entry points (main, editor plugins), source/out directories, handle non-TS files separately
```

**Wrapper Modules**
- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-extension-common.mts` (51 lines) — Adds platform-specific (node/browser) and format (cjs/esm) configuration, re-exports runBuild()
- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-webview-common.mts` (29 lines) — Browser-optimized wrapper with ESM/browser platform defaults, used by webview build scripts

**Individual Extension Build Scripts**
74+ extension-specific esbuild scripts found across:
- `extensions/*/esbuild.mts` — Node platform bundles (git, github, markdown, etc.)
- `extensions/*/esbuild.browser.mts` — Browser platform bundles
- `extensions/*/esbuild.webview.mts` — Webview-specific bundles
- `extensions/*/esbuild.notebook.mts` — Notebook renderer bundles
Examples: git/, simple-browser/, markdown-language-features/, typescript-language-features/, etc.

## Notable Clusters

**Extension Build System Architecture**
- **Root**: `esbuild-common.mts` (generic run/watch orchestration)
- **Mid-layer**: `esbuild-extension-common.mts` + `esbuild-webview-common.mts` (platform/format defaults)
- **Leaf nodes**: 74+ extension-specific esbuild.mts scripts (entry point definitions, custom post-build hooks)

**Build Modes**
- Single-pass compilation: `esbuild.build()` for CI/release
- Watch mode: parcel-based file monitoring with debounced rebuilds for development
- Both routed through same `runBuild()` dispatcher

**Output Flexibility**
- Default: `config.outdir` (usually `dist/`)
- Override: `--outputRoot` flag reroutes output while preserving directory structure

## Documentation

No dedicated test files, README, or documentation found within the scope. The module is self-documented via TypeScript interfaces and implementation comments indicating it is a "Shared build/watch runner for extension esbuild scripts."

---

## Porting Implications

For a Tauri/Rust port of VS Code, **the esbuild infrastructure remains necessary** (extensions still compile from TS/JS to bundled modules). This module would need equivalent functionality in Rust's build system:

1. **Core abstraction** (RunConfig + runBuild) → Rust struct + async function wrapping build logic
2. **Watch mode** → Rust file-watching library (notify, watchexec, or similar) replacing parcel
3. **Platform/format variants** → Conditional compilation or config structs for node vs. browser bundles
4. **Post-build hooks** → Async callback pattern for asset copying, code generation, etc.
5. **CLI arg parsing** → Structured argument handling (--watch, --outputRoot) in Rust

The esbuild bundler itself would remain external (Node.js subprocess or WASM), unless a pure Rust bundler (swc, turbopack) is adopted—but that decision impacts all 74+ extension build scripts.

