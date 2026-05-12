### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-webview-common.mts` — 29 LOC, primary subject
- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-common.mts` — 81 LOC, upstream dependency
- `/home/norinlavaee/projects/vscode-atomic/extensions/simple-browser/esbuild.webview.mts` — consumer, 23 LOC
- `/home/norinlavaee/projects/vscode-atomic/extensions/mermaid-chat-features/esbuild.webview.mts` — consumer, 24 LOC
- `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-language-features/esbuild.webview.mts` — consumer, 18 LOC
- `/home/norinlavaee/projects/vscode-atomic/extensions/notebook-renderers/esbuild.notebook.mts` — consumer, 17 LOC
- `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/esbuild.notebook.mts` — consumer, 35 LOC
- `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/esbuild.notebook.mts` — consumer, 17 LOC

---

### Per-File Notes

#### `extensions/esbuild-webview-common.mts`

This file is the shared build configuration layer for webview/notebook renderer bundling. It has two logical sections:

**`baseOptions` constant (lines 11–21)**

A static `esbuild.BuildOptions`-shaped object that encodes the canonical settings for all webview bundles:
- `bundle: true` — all imports are inlined into the output
- `minify: true` — output is minified
- `sourcemap: false` — no source maps are produced
- `format: 'esm' as const` (line 15) — output module format is ES Module; the `as const` cast ensures TypeScript narrows the type to the literal string union expected by esbuild
- `platform: 'browser' as const` (line 16) — esbuild resolves browser-specific package fields and avoids Node built-ins
- `target: ['es2024']` (line 17) — code is down-compiled to ES2024; syntax above that level is transformed
- `logOverride: { 'import-is-undefined': 'error' }` (lines 18–20) — promotes the normally-warning `import-is-undefined` diagnostic to an error, making missing imports a build failure

**`run()` function (lines 23–29)**

```ts
export async function run(
  config: RunConfig,
  args: string[],
  didBuild?: (outDir: string) => unknown
): Promise<void>
```

The only export of the module. It accepts:
- `config: RunConfig` — typed in `esbuild-common.mts:8–13`; carries `srcDir`, `outdir`, `entryPoints`, and optional `additionalOptions`
- `args: string[]` — raw `process.argv` from the calling script; passed through to detect `--watch` and `--outputRoot` flags
- `didBuild?: (outDir: string) => unknown` — optional post-build callback invoked with the resolved output directory

The body is a single delegation: `return runBuild(config, baseOptions, args, didBuild)` (line 28). The `baseOptions` constant defined in this file is the only contribution beyond delegation — it supplies all webview-specific esbuild settings that `runBuild` merges with per-caller options.

---

#### `extensions/esbuild-common.mts`

Provides the `RunConfig` interface (lines 8–13) and the `runBuild()` function (lines 18–51) consumed by `esbuild-webview-common.mts`.

**`RunConfig` interface (lines 8–13)**
- `srcDir: string` — source directory watched during `--watch` mode
- `outdir: string` — base output directory; may be overridden by `--outputRoot`
- `entryPoints: esbuild.BuildOptions['entryPoints']` — esbuild's native entry-point type; callers may pass an array of paths or a name→path object
- `additionalOptions?: Partial<esbuild.BuildOptions>` — escape hatch for per-extension overrides (e.g., custom loaders)

**`runBuild()` (lines 18–51)**

Merges `baseOptions` from the caller with per-config settings (lines 32–37):
```ts
const resolvedOptions: esbuild.BuildOptions = {
  ...baseOptions,
  entryPoints: config.entryPoints,
  outdir,
  ...(config.additionalOptions || {}),
};
```
The spread order means `config.additionalOptions` overrides `baseOptions`. The output directory is first resolved: if `--outputRoot <dir>` appears in `args` (lines 25–30), `outdir` becomes `<outputRoot>/<basename of config.outdir>` — allowing CI to redirect output to a staging area.

After merging, `runBuild()` branches on the `--watch` flag (line 39):
- **Watch mode** (lines 40–42): calls `esbuild.context(resolvedOptions)` to get an incremental build context, then delegates to `watchWithParcel()` passing the context and `srcDir`
- **One-shot mode** (lines 44–49): calls `await esbuild.build(resolvedOptions)`, invokes `didBuild?.(outdir)`, and calls `process.exit(1)` on build error

**`watchWithParcel()` (lines 54–80)**

Uses `@parcel/watcher` (imported dynamically at line 73) instead of esbuild's built-in watch mode to reduce idle CPU usage (comment at line 53). Subscribes to file changes in `srcDir`, ignoring `**/node_modules/**`, `**/dist/**`, `**/out/**` (lines 74–78). On each file-system event, a 100 ms debounced rebuild fires (lines 56–71): it cancels any in-progress esbuild context build (`ctx.cancel()`), calls `ctx.rebuild()`, and if there are no errors, invokes `didBuild?.()`.

---

#### Consumer scripts — patterns observed

All consumers follow the same pattern: define `srcDir` and `outDir` using `import.meta.dirname`, construct a `RunConfig` object, and call `run(config, process.argv[, postBuild])`.

**`simple-browser/esbuild.webview.mts` (lines 8–23)**
- `srcDir`: `preview-src/`, `outDir`: `media/`
- Entry points: named object `{ index: ..., codicon: ... }` — the codicon entry points to the `@vscode/codicons` CSS file directly from node_modules
- `additionalOptions.loader['.ttf'] = 'dataurl'` — font files are inlined as data URLs for the browser environment

**`mermaid-chat-features/esbuild.webview.mts` (lines 8–24)**
- `srcDir`: `chat-webview-src/`, `outDir`: `chat-webview-out/`
- Two TypeScript entry points (`index.ts`, `index-editor.ts`) plus a codicon CSS entry
- Same `.ttf` dataurl loader as simple-browser

**`markdown-language-features/esbuild.webview.mts` (lines 8–18)**
- `srcDir`: `preview-src/`, `outDir`: `media/`
- Entry points as an array (not a named object): `[index.ts, pre/]` — esbuild resolves the directory entry via its index file
- No `additionalOptions`

**`notebook-renderers/esbuild.notebook.mts` (lines 8–17)**
- `srcDir`: `src/`, `outDir`: `renderer-out/`
- Single entry point: `src/index.ts`
- No post-build callback

**`markdown-math/esbuild.notebook.mts` (lines 9–35)**
- `srcDir`: `notebook/`, `outDir`: `notebook-out/`
- Entry point: `notebook/katex.ts`
- Defines a `postBuild` callback (lines 12–27): uses `fs-extra` to copy KaTeX's minified CSS and `.woff2` font files from its node_modules into `notebook-out/` after each successful build. This is passed as the third argument to `run()`, which threads it through to `runBuild()` and ultimately to esbuild's build completion.

**`ipynb/esbuild.notebook.mts` (lines 8–17)**
- `srcDir`: `notebook-src/`, `outDir`: `notebook-out/`
- Single entry point: `notebook-src/cellAttachmentRenderer.ts`
- No post-build callback

---

### Cross-Cutting Synthesis

`esbuild-webview-common.mts` is a one-layer adapter: it freezes a fixed set of esbuild options appropriate for browser-targeted ESM bundles (ESM format, browser platform, ES2024 target, minification, no source maps) and exposes a single `run()` entry point that merges those options with per-extension config before handing off to `esbuild-common.mts:runBuild`. The two-file split lets `esbuild-common.mts` remain platform-agnostic (it has no hardcoded browser/ESM assumptions), while this file brands the configuration for the webview use-case. All six consumer scripts are structurally identical: they import `run` from this file, pass `process.argv` directly so flag parsing happens deep in `runBuild`, and optionally supply a `didBuild` callback for post-bundle asset copying. The `additionalOptions` escape hatch in `RunConfig` is the only per-extension customization point, used by simple-browser and mermaid-chat-features for `.ttf` font embedding and by markdown-math for KaTeX static assets. Watch mode relies on `@parcel/watcher` (dynamically imported) rather than esbuild's native watcher to reduce CPU overhead when idle; the 100 ms debounce in `watchWithParcel` batches rapid file-system changes into single rebuilds.

---

### Out-of-Partition References

- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-common.mts` — defines `RunConfig` interface and `runBuild()` consumed directly by `esbuild-webview-common.mts:9,28`
- `esbuild` npm package — `esbuild.build()`, `esbuild.context()`, `esbuild.BuildOptions` types, used throughout `esbuild-common.mts:6,41,45`
- `@parcel/watcher` npm package — dynamically imported in `esbuild-common.mts:73` for file-system watching
- `fs-extra` npm package — used in `extensions/markdown-math/esbuild.notebook.mts:5` for post-build asset copying
- `@vscode/codicons` npm package — referenced as a CSS entry point in `simple-browser/esbuild.webview.mts:14` and `mermaid-chat-features/esbuild.webview.mts:15`
- `katex` npm package — CSS and font assets copied from its node_modules dist in `markdown-math/esbuild.notebook.mts:13–26`
