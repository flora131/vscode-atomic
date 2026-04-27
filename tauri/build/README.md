# Tauri Build Pipeline (Vite)

Replaces the Gulp+esbuild pipeline (`build/gulpfile.*.ts`, `build/next/index.ts`) with a
Vite-based pipeline aligned with the Tauri idiom.

> **Status**: Desktop target is functional. Server / server-web / web targets are **stubbed** —
> entry points and plugins are wired but the upstream source modules may not exist in this
> workspace yet.

---

## Prerequisites

`vite` must be resolvable from the repo root's `node_modules` (Vite injects a config
shim there during startup). One-time setup after cloning:

```sh
# 1. Install build/vite deps (if not already done)
cd build/vite && npm install && cd ../..

# 2. Symlink vite into root node_modules
ln -sfn "$(pwd)/build/vite/node_modules/vite" node_modules/vite
```

## Quick Start

```sh
# Default (desktop)
npm run build:tauri

# Explicit target
VSCODE_TARGET=desktop npm run build:tauri
npm run build:tauri -- --mode desktop
```

Output lands in `out-tauri/<target>/`.

---

## Target Matrix

| Target      | Status       | Entry points wired | Post-processing |
|-------------|--------------|-------------------|-----------------|
| `desktop`   | **Active**   | Yes               | NLS, mangle-privates, product-injection, builtin-extensions |
| `server`    | Stub         | Yes (may be missing) | Plugins active but untested |
| `server-web`| Stub         | Yes (may be missing) | Plugins active but untested |
| `web`       | Stub         | Yes (may be missing) | Plugins active but untested |

---

## File Structure

```
tauri/build/
├── vite.config.ts          # Multi-target Vite config
├── entrypoints.ts          # Per-target entry point lists (mirrors build/next/index.ts)
├── plugins/
│   ├── nls.ts              # NLS rewrite (localize() → integer index)
│   ├── mangle-privates.ts  # #private field → $a/$b mangling
│   ├── product-injection.ts # /*BUILD->INSERT_PRODUCT_CONFIGURATION*/ replacement
│   └── builtin-extensions.ts # /*BUILD->INSERT_BUILTIN_EXTENSIONS*/ replacement
└── README.md               # This file
```

---

## Post-Processing Passes

### NLS Rewrite (`plugins/nls.ts`)

Ports `build/next/nls-plugin.ts` semantics into Vite hooks:

1. `transform` — collects `localize('key', 'message')` calls per module, assigns stable
   integer indices (sorted by moduleId then key).
2. `renderChunk` — replaces collected placeholders with their indices (production) or
   keeps English strings (dev/preserve mode).
3. `generateBundle` — writes `nls.keys.json`, `nls.messages.json`, `nls.metadata.json`
   to the output directory.

**TODO**: The regex-based collector covers ~99% of call sites. The upstream pipeline uses
AST-level analysis via `build/lib/nls-analysis.ts`. If edge cases surface, wire the full
analyzer in the `transform` hook.

### Mangle Privates (`plugins/mangle-privates.ts`)

Ports `build/next/private-to-property.ts` usage (lines 963–975):

- Runs in `renderChunk` (after bundling, before write).
- Delegates to `build/next/private-to-property.ts` at runtime via dynamic import.
- Skips extension-host bundles (they expose API surface to extensions).
- No-op for non-desktop targets.

**TODO**: Source-map adjustment for mangle edits is not yet wired. The upstream esbuild
pipeline calls `adjustSourceMap` to keep sourcemaps accurate after the string surgery.
Wire it in `generateBundle` once the Vite sourcemap API stabilises.

### Product Injection (`plugins/product-injection.ts`)

Replaces `/*BUILD->INSERT_PRODUCT_CONFIGURATION*/` (inside an object literal) with the
contents of `product.json` merged with `version`, `commit`, and `date`.

- For `server-web` target, `webEndpointUrlTemplate` is stripped (mirrors esbuild pipeline).
- Replacement is computed once and cached.

### Built-in Extensions (`plugins/builtin-extensions.ts`)

Replaces `/*BUILD->INSERT_BUILTIN_EXTENSIONS*/` (inside an array literal) with the JSON
array returned by `build/lib/extensions.ts#scanBuiltinExtensions`.

- `web` target scans `.build/web/extensions`; all others scan `.build/extensions`.

---

## Differences from Gulp+esbuild Pipeline

| Concern               | Gulp+esbuild                          | Vite (this pipeline)                         |
|-----------------------|---------------------------------------|----------------------------------------------|
| Bundler               | esbuild direct                        | Vite (rolldown/rollup under the hood)        |
| NLS analysis          | Full AST via nls-analysis.ts          | Regex-based (fast, ~99% coverage)            |
| Sourcemap NLS/mangle  | Adjusted via adjustSourceMap()        | **TODO** — not yet wired in Vite hooks       |
| CSS bundling          | esbuild `outdir` per CSS entry        | Rollup handles CSS splitting                 |
| Bootstrap files       | Separate esbuild build with minimist  | Same input map, external: [] for bootstrap   |
| Syntax-check pass     | esbuild.transform() after post-proc   | **TODO** — not yet wired                     |

---

## Smoke Check

Running `npm run build:tauri` on a fresh checkout (no `yarn compile` yet) will:

1. Print `[tauri/build] N entry point(s) not found — they will be skipped`.
2. Fall back to a no-op stub (`out-tauri/__stub.ts`).
3. Complete without TS/Vite errors.

To produce a real bundle:
```sh
yarn compile          # or: npm run compile-build
npm run build:tauri
```

---

## Remaining Work

- [ ] Wire `adjustSourceMap` for NLS and mangle-privates edits (Vite sourcemap API).
- [ ] Wire post-bundle syntax-check pass (esbuild.transform as parser).
- [ ] Test server / server-web / web targets end-to-end once upstream modules compile.
- [ ] Add CSS extraction config for web workbench targets.
- [ ] Add `--watch` mode (Vite's `build --watch`).
- [ ] Integrate with Tauri `tauri build` command (frontend dist path).
- [ ] Replace regex NLS collector with full AST analysis from `build/lib/nls-analysis.ts`.
