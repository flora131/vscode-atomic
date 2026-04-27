### Files Analysed

- `extensions/esbuild-extension-common.mts` (102 LOC)

---

### Per-File Notes

#### `extensions/esbuild-extension-common.mts`

- **Role:** Shared build-script utility consumed by individual VS Code extension packages. It abstracts the esbuild invocation for compiling TypeScript extension source code into either CommonJS or ESM bundles targeting either the Node.js or browser runtime. It is pure build infrastructure with no runtime presence in the running IDE.

- **Key symbols:**
  - `RunConfig` interface (`extensions/esbuild-extension-common.mts:15-22`) — caller-supplied configuration shape. Fields: `platform` (`'node' | 'browser'`), optional `format` (`'cjs' | 'esm'`), `srcDir`, `outdir`, `entryPoints`, optional `additionalOptions`.
  - `BuildOptions` type alias (`extensions/esbuild-extension-common.mts:11-13`) — `Partial<esbuild.BuildOptions>` intersection that enforces `outdir` is always present.
  - `resolveOptions(config, outdir): BuildOptions` (`extensions/esbuild-extension-common.mts:24-57`) — internal function that merges caller config with hard-coded defaults and applies platform-specific overrides.
  - `run(config, args, didBuild?): Promise<void>` (`extensions/esbuild-extension-common.mts:59-102`) — sole public export. Entry point that individual extension build scripts call. Handles CLI argument parsing, delegates to `resolveOptions`, then invokes either `esbuild.context(...).watch()` or `esbuild.build(...)`.

- **Control flow:**
  1. `run()` at line 59 receives a `RunConfig`, a raw `process.argv`-style `args` array, and an optional post-build callback `didBuild`.
  2. Lines 61-66: If `--outputRoot <path>` is present in `args`, the final `outdir` is rewritten to `<outputRoot>/<basename of config.outdir>`.
  3. Line 68: `resolveOptions(config, outdir)` is called to produce the full esbuild `BuildOptions` object.
  4. Inside `resolveOptions` (lines 24-57):
     - A baseline options object is assembled at lines 25-40 with `bundle: true`, `minify: true`, `treeShaking: true`, `sourcemap: true`, `target: ['es2024']`, `external: ['vscode']`, and `format` defaulting to `'cjs'` (line 33).
     - For `platform === 'node'` (lines 42-43): `mainFields` is set to `['module', 'main']`.
     - For `platform === 'browser'` (lines 44-54): `mainFields` is `['browser', 'module', 'main']`, `path` is aliased to `path-browserify`, and `process.platform`, `process.env`, and `process.env.BROWSER_ENV` are injected as compile-time constants via `define`.
  5. Back in `run()`, line 70 checks for `--watch` in `args`.
     - Watch path (lines 71-93): If `didBuild` is provided, a custom esbuild plugin named `'did-build'` is pushed onto `resolvedOptions.plugins` (lines 73-90). The plugin registers an `onEnd` hook that calls `didBuild(outdir)` after every successful rebuild (errors are checked at line 79 before invoking the callback). Then `esbuild.context(resolvedOptions)` is awaited and `.watch()` is started (lines 92-93).
     - One-shot build path (lines 94-101): `esbuild.build(resolvedOptions)` is awaited, followed by the optional `didBuild?.(outdir)` call (line 97). Any thrown error causes `process.exit(1)` at line 99.

- **Data flow:**
  - Input: `RunConfig` object + CLI `args` array → `resolveOptions` → `esbuild.BuildOptions` object → passed to `esbuild.build` or `esbuild.context`.
  - The `--outputRoot` flag mutates the local `outdir` variable (line 65) before it reaches `resolveOptions`; the original `config.outdir` is never modified.
  - `additionalOptions` (line 39) is spread last inside `resolveOptions`, allowing callers to override any default (including the platform-specific defaults set at lines 42-54, since `additionalOptions` is applied before the platform branch would later be re-applied — note the platform branches at lines 42-54 follow the spread, so they are applied on top).
  - The compiled output lands in `outdir` as determined by the resolved path; sourcemaps are emitted alongside (line 30).

- **Dependencies:**
  - `node:path` (line 8) — used only for `path.basename` and `path.join` in the `--outputRoot` branch (lines 64-65).
  - `esbuild` (line 9) — the sole build-system dependency. Types `esbuild.BuildOptions` and `esbuild.context`/`esbuild.build` are used directly.
  - No VS Code runtime APIs are imported. The string `'vscode'` appears only as an entry in the `external` array (line 36), instructing esbuild to leave `import 'vscode'` calls unresolved at bundle time.

---

### Cross-Cutting Synthesis

`extensions/esbuild-extension-common.mts` is pure build infrastructure with no presence in the running IDE process. It standardises how each VS Code extension package transpiles its TypeScript source into deployable JavaScript via esbuild, enforcing consistent settings (ES2024 target, minification, tree-shaking, sourcemaps, externalising the `vscode` host API). The platform split between `'node'` and `'browser'` mirrors VS Code's dual-runtime extension model, where extensions may run in either a Node.js extension host or a browser-based web worker host. In the context of a Tauri/Rust port, this file's significance is limited: it captures the assumption that extensions are compiled JavaScript bundles that rely on a `vscode` host module at runtime. Any Tauri port that wishes to retain the extension ecosystem would need to provide an equivalent host module and a compatible extension-loading mechanism; this file itself would require only minor adjustments (e.g., changing `external` entries or adding a Tauri-specific platform branch) and does not encode any Electron-specific assumptions.

---

### Out-of-Partition References

- Individual extension `build.mts` / `build.js` scripts (not in this partition) that call `run()` exported from this file.
- `esbuild` npm package (external dependency, not in the repository).
- `path-browserify` npm package (referenced as an alias target at line 47, external to the repository).
