### Files Analysed

- `extensions/markdown-math/src/extension.ts` (46 lines)
- `extensions/markdown-math/notebook/katex.ts` (58 lines)
- `extensions/markdown-math/package.json` (124 lines)
- `extensions/markdown-math/esbuild.notebook.mts` (35 lines)

---

### Per-File Notes

#### extensions/markdown-math/src/extension.ts

- **Role:** VS Code extension entry point; hooks into the markdown-it preview pipeline by returning an `extendMarkdownIt` export that the host markdown extension invokes.

- **Key symbols:**
  - `activate(context: vscode.ExtensionContext)` — line 11, the single export consumed by VS Code's extension host.
  - `isEnabled(): boolean` — line 12 (inner), reads `markdown.math.enabled` from workspace config (default `true`).
  - `getMacros(): { [key: string]: string }` — line 17 (inner), reads `markdown.math.macros` from workspace config (default `{}`).
  - `extendMarkdownIt(md: any)` — line 29 (returned object method), the callback invoked by the markdown preview host.

- **Control flow:**
  1. `activate` registers a `vscode.workspace.onDidChangeConfiguration` listener (line 22) scoped to `markdown.math`; on any change it fires `markdown.api.reloadPlugins` to force a preview rebuild.
  2. `activate` returns an object with `extendMarkdownIt`. When the preview host calls this method, it first calls `isEnabled()` (line 30); if `false`, returns `md` unchanged.
  3. If enabled, it lazily `require`s `@vscode/markdown-it-katex` (line 31), reads current macros, and builds an options object with `enableFencedBlocks: true`, `globalGroup: true`, and the current macro map (lines 33–37).
  4. Pushes a custom core ruler rule `reset-katex-macros` onto `md.core.ruler` (line 38) that resets the `options.macros` to a fresh copy of `settingsMacros` on each render pass, preventing macro bleed between renders.
  5. Returns `md.use(katex, options)` (line 41), registering the katex plugin on the markdown-it instance.

- **Data flow:**
  - VS Code config → `getMacros()` / `isEnabled()` → `options` object → `md.use(katex, options)`.
  - The `options.macros` object is mutated in-place by the katex plugin during rendering; the ruler rule resets it before each new document render.

- **Dependencies:**
  - `vscode` (VS Code extension API)
  - `@vscode/markdown-it-katex` (runtime `require`d, not statically imported, making it compatible with the bundled browser path)

---

#### extensions/markdown-math/notebook/katex.ts

- **Role:** Webview-side notebook renderer activation script. Extends the existing `vscode.markdown-it-renderer` notebook renderer (declared in `package.json` as `"extends"`) with KaTeX math support.

- **Key symbols:**
  - `activate(ctx: RendererContext<void>)` — line 10, async, the entry point called by the notebook renderer host.
  - `styleHref` — line 8, a computed URL pointing to `katex.min.css` relative to the current module's URL, derived by replacing `katex.js` in `import.meta.url`.

- **Control flow:**
  1. Calls `ctx.getRenderer('vscode.markdown-it-renderer')` (line 11); throws if the base renderer is not available (line 13).
  2. Creates a `<link rel="stylesheet">` element with class `markdown-style` (lines 17–20); this class causes the VS Code notebook infrastructure to copy this stylesheet into each cell's shadow DOM.
  3. Creates a second `<link>` (lines 23–29) appended directly to `document.head` to work around a Chromium shadow DOM font-loading bug (Chromium issue #336876).
  4. Creates an inline `<style>` element (lines 31–38) defining `.katex-error` (mapped to `--vscode-editorError-foreground`) and `.katex-block` (resets KaTeX equation counters).
  5. Wraps both `<style>` and the first `<link>` in a `<template>` with class `markdown-style` (lines 41–46), appended to `document.head`, so the notebook renderer infrastructure propagates styles into shadow roots.
  6. `require`s `@vscode/markdown-it-katex` (line 48), initialises an empty shared `macros` object (line 49).
  7. Calls `markdownItRenderer.extendMarkdownIt(md => md.use(katex, { globalGroup: true, enableBareBlocks: true, enableFencedBlocks: true, macros }))` (lines 50–57), delegating plugin registration to the base renderer's extension API.

- **Data flow:**
  - `import.meta.url` → `styleHref` → `<link href>` attributes injected into DOM.
  - Empty `macros` object mutated by KaTeX across notebook cells (shared across cell renders via `globalGroup: true`).
  - `markdownItRenderer.extendMarkdownIt` callback receives the live `markdown-it` instance and returns it after plugin attachment.

- **Dependencies:**
  - `markdown-it` (type import only: `import type * as markdownIt`)
  - `vscode-notebook-renderer` (type import: `RendererContext`)
  - `@vscode/markdown-it-katex` (runtime `require`)
  - `vscode.markdown-it-renderer` (base notebook renderer, accessed via `ctx.getRenderer`)

---

#### extensions/markdown-math/package.json

- **Role:** Extension manifest declaring all VS Code contribution points, build targets, and runtime dependencies.

- **Key symbols:**
  - `"main": "./out/extension"` (line 23) — Node.js entry point for the desktop extension host.
  - `"browser": "./dist/browser/extension"` (line 24) — Browser/web extension entry point.
  - `"contributes.markdown.markdownItPlugins": true` (line 81) — Signals to the markdown extension host that this extension exports `extendMarkdownIt`.
  - `"contributes.notebookRenderer"` (lines 71–79) — Registers `vscode.markdown-it-katex-extension` renderer with `"extends": "vscode.markdown-it-renderer"` and entry point `./notebook-out/katex.js`.
  - `"contributes.markdown.previewStyles"` (lines 82–85) — Contributes `./notebook-out/katex.min.css` and `./preview-styles/index.css` to markdown preview.
  - `"contributes.configuration"` (lines 86–106) — Declares `markdown.math.enabled` (boolean, default `true`) and `markdown.math.macros` (object, default `{}`).
  - `"contributes.grammars"` (lines 33–69) — Four TextMate grammar contributions: `text.html.markdown.math` (standalone language), `markdown.math.block`, `markdown.math.inline`, and `markdown.math.codeblock` (all injected into `text.html.markdown`, mapping embedded language to `latex`).
  - `"activationEvents": []` (line 25) — Empty; extension activates on demand via `markdown.markdownItPlugins`.

- **Control flow:** Declarative manifest; no imperative logic. The `"scripts"` section (lines 108–112) maps `compile` and `watch` to `node ./esbuild.notebook.mts`.

- **Data flow:** Build output (`notebook-out/`) consumed directly by the notebook renderer host at runtime via the `entrypoint.path` declaration.

- **Dependencies:**
  - Runtime: `@vscode/markdown-it-katex ^1.1.2`
  - Dev: `@types/markdown-it`, `@types/vscode-notebook-renderer ^1.60.0`

---

#### extensions/markdown-math/esbuild.notebook.mts

- **Role:** Build script that bundles `notebook/katex.ts` into `notebook-out/katex.js` and copies KaTeX CSS and WOFF2 fonts into the output directory.

- **Key symbols:**
  - `srcDir` — line 9, `import.meta.dirname + '/notebook'`.
  - `outDir` — line 10, `import.meta.dirname + '/notebook-out'`.
  - `postBuild(outDir: string)` — line 12, synchronous post-build callback.
  - `run(config, process.argv, postBuild)` — line 29, invocation of the shared build runner from `extensions/esbuild-webview-common.mts`.

- **Control flow:**
  1. `postBuild` copies `node_modules/katex/dist/katex.min.css` to `outDir/katex.min.css` (line 13–15).
  2. Reads the katex `fonts/` directory (line 22) and copies only `*.woff2` files to `outDir/fonts/` (lines 23–26).
  3. `run` (from `esbuild-webview-common.mts`) compiles `notebook/katex.ts` with esbuild options: `bundle: true`, `minify: true`, `format: 'esm'`, `platform: 'browser'`, `target: ['es2024']`.
  4. In watch mode, `run` wraps `postBuild` in an esbuild `onEnd` plugin; in one-shot mode, it calls `postBuild` after `esbuild.build` resolves.
  5. Supports `--outputRoot` flag (handled inside `run`) to redirect output directory, and `--watch` flag for incremental rebuilds.

- **Data flow:**
  - `notebook/katex.ts` → esbuild bundle → `notebook-out/katex.js`.
  - `node_modules/katex/dist/katex.min.css` → `notebook-out/katex.min.css`.
  - `node_modules/katex/dist/fonts/*.woff2` → `notebook-out/fonts/*.woff2`.

- **Dependencies:**
  - `fs-extra` (file copy utilities)
  - `path` (Node built-in)
  - `../esbuild-webview-common.mts` (shared `run` function at `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-webview-common.mts`)
  - `esbuild` (indirectly, via `esbuild-webview-common.mts`)

---

### Cross-Cutting Synthesis

The `markdown-math` extension provides KaTeX-based math rendering through two independent but structurally parallel surfaces. The first surface is the markdown preview: `extension.ts:activate` returns an `extendMarkdownIt` hook (line 29) that the VS Code markdown extension host calls when building its markdown-it pipeline; it lazily loads `@vscode/markdown-it-katex` and attaches it with fenced-block support and per-render macro reset. The second surface is notebook output cells: `notebook/katex.ts:activate` acquires the base `vscode.markdown-it-renderer` via `ctx.getRenderer` (line 11), injects KaTeX CSS into both the shadow DOM (via a `markdown-style`-classed template) and the root document head (Chromium font-loading workaround, line 29), then registers the same katex plugin via `markdownItRenderer.extendMarkdownIt` (line 50). The manifest wires these surfaces via `markdown.markdownItPlugins: true` (line 81) for the preview hook and a `notebookRenderer` contribution with `"extends": "vscode.markdown-it-renderer"` (lines 71–79) for the notebook renderer. The build pipeline (`esbuild.notebook.mts`) bundles the notebook renderer entry point to ESM targeting ES2024 browsers and co-locates KaTeX's CSS and WOFF2 fonts in the output directory so the self-computed `styleHref` in `katex.ts:8` resolves correctly at runtime. Both rendering paths share `@vscode/markdown-it-katex` as the sole runtime dependency and use `globalGroup: true` to maintain a shared macro namespace across math blocks within a single document render.

---

### Out-of-Partition References

- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-webview-common.mts` — shared `run()` build helper imported by `esbuild.notebook.mts:7`; defines esbuild base options (`bundle`, `minify`, `format: 'esm'`, `platform: 'browser'`, `target: ['es2024']`), `--outputRoot` flag handling, and watch-mode context management.
- `vscode.markdown-it-renderer` (built-in VS Code notebook renderer) — resolved at runtime via `ctx.getRenderer('vscode.markdown-it-renderer')` in `notebook/katex.ts:11`; provides the `extendMarkdownIt(fn)` API used to attach the KaTeX plugin.
- `@vscode/markdown-it-katex` npm package — the actual KaTeX markdown-it plugin; `require`d at runtime in both `src/extension.ts:31` and `notebook/katex.ts:48`; not statically imported so the same source works in both Node.js and browser bundle contexts.
- `extensions/markdown-math/syntaxes/md-math.tmLanguage.json`, `md-math-block.tmLanguage.json`, `md-math-inline.tmLanguage.json`, `md-math-fence.tmLanguage.json` — TextMate grammar files referenced in `package.json` lines 36, 43, 52, 63; provide syntax highlighting for math regions in markdown files (not included in this partition's 5-file scope).
- `extensions/markdown-math/preview-styles/index.css` — contributed as a markdown preview style in `package.json:84`; additional preview styling outside the notebook renderer path.
- `node_modules/katex/dist/` — KaTeX distribution directory accessed by `esbuild.notebook.mts` during `postBuild` (lines 14, 17) to copy `katex.min.css` and WOFF2 fonts into `notebook-out/`.
