### Files Analysed

- `/Users/norinlavaee/vscode-atomic/extensions/esbuild-webview-common.mts` — 82 lines, shared esbuild build configuration for webview content
- `/Users/norinlavaee/vscode-atomic/extensions/simple-browser/esbuild.webview.mts` — 23 lines, consumer
- `/Users/norinlavaee/vscode-atomic/extensions/notebook-renderers/esbuild.notebook.mts` — 17 lines, consumer
- `/Users/norinlavaee/vscode-atomic/extensions/mermaid-chat-features/esbuild.webview.mts` — 24 lines, consumer
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/esbuild.notebook.mts` — 35 lines, consumer
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-language-features/esbuild.webview.mts` — 18 lines, consumer
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-language-features/esbuild.notebook.mts` — 17 lines, consumer
- `/Users/norinlavaee/vscode-atomic/extensions/ipynb/esbuild.notebook.mts` — 17 lines, consumer

---

### Per-File Notes

#### `extensions/esbuild-webview-common.mts`

**Purpose and Exports**

The file exports two items:

1. `BuildOptions` type (`esbuild-webview-common.mts:12-15`): A TypeScript type alias defined as `Partial<esbuild.BuildOptions>` intersected with required fields `entryPoints` (typed from `esbuild.BuildOptions['entryPoints']`) and `outdir` (plain `string`). This type enforces that consumers must supply at minimum these two fields.

2. `run()` async function (`esbuild-webview-common.mts:17-81`): The sole exported executable. Its signature accepts three parameters:
   - `config` object with fields `srcDir: string`, `outdir: string`, `entryPoints: BuildOptions['entryPoints']`, and optional `additionalOptions?: Partial<esbuild.BuildOptions>`
   - `args: string[]` — the raw CLI argument vector, in practice always `process.argv` from each consumer
   - optional `didBuild?: (outDir: string) => unknown` — post-build callback

**Control Flow in `run()`**

Step 1 — Output directory resolution (`esbuild-webview-common.mts:27-33`): The function reads `config.outdir` into `outdir`. It then scans `args` for the string `'--outputRoot'` using `args.indexOf`. If found (`outputRootIndex >= 0`), it takes the next element as the new root directory, extracts only the final path component of the original `outdir` via `path.basename`, and rebuilds the output path as `path.join(outputRoot, outputDirName)`. This lets a CI build step redirect all outputs under a common root.

Step 2 — Options assembly (`esbuild-webview-common.mts:35-48`): A `resolvedOptions` object of type `BuildOptions` is constructed using an object literal spread. The fixed defaults are:
- `bundle: true`
- `minify: true`
- `sourcemap: false`
- `format: 'esm'`
- `platform: 'browser'`
- `target: ['es2024']`
- `logOverride: { 'import-is-undefined': 'error' }` — promotes the normally-warning import-is-undefined condition to a hard error

Then `config.entryPoints`, the resolved `outdir`, and finally `config.additionalOptions` (spread via `|| {}` to guard against undefined) are merged. Consumer-supplied `additionalOptions` can override any of the fixed defaults because they are spread last.

Step 3 — Watch vs. one-shot dispatch (`esbuild-webview-common.mts:50-80`): The function checks `args.indexOf('--watch') >= 0`.

- **Watch mode** (`esbuild-webview-common.mts:51-73`): If `didBuild` was provided, an esbuild plugin named `'did-build'` is created inline. Its `setup` function registers an `onEnd` hook (`esbuild-webview-common.mts:57`). That hook fires after each build cycle; if the result has no errors (`result.errors.length > 0` short-circuits), it calls `await didBuild(outdir)`, catching and logging any thrown exceptions (`esbuild-webview-common.mts:64-66`). The plugin is appended to any already-present `resolvedOptions.plugins`. After plugin injection, `esbuild.context(resolvedOptions)` creates a persistent build context (`esbuild-webview-common.mts:72`) and `ctx.watch()` is awaited to enter incremental rebuild mode (`esbuild-webview-common.mts:73`).

- **One-shot mode** (`esbuild-webview-common.mts:74-80`): `esbuild.build(resolvedOptions)` is awaited. On success, `didBuild?.(outdir)` is called with optional chaining. If `esbuild.build` throws (build errors), the catch block calls `process.exit(1)` with no error logging (`esbuild-webview-common.mts:79`), relying on esbuild's own stderr output.

**Dependencies**

- `node:path` (line 9): used only for `path.basename` and `path.join` in the output root override path.
- `esbuild` (line 10): both the `esbuild.BuildOptions` type and the `esbuild.build`, `esbuild.context` runtime functions.

---

#### Consumer Files

All seven consumers follow the identical structural pattern: import `run` from `'../esbuild-webview-common.mts'`, derive absolute `srcDir` and `outDir` via `import.meta.dirname`, declare `entryPoints`, and invoke `run(config, process.argv)`.

**`extensions/simple-browser/esbuild.webview.mts`**

Declares two named entry points (`esbuild.webview.mts:12-15`):
- `'index'` → `preview-src/index.ts`
- `'codicon'` → `node_modules/@vscode/codicons/dist/codicon.css`

Passes `additionalOptions` with `loader: { '.ttf': 'dataurl' }` (`esbuild.webview.mts:19-22`) to inline TrueType font files as base64 data URLs. Output goes to `media/`.

**`extensions/mermaid-chat-features/esbuild.webview.mts`**

Declares three named entry points (`esbuild.webview.mts:12-15`):
- `'index'` → `chat-webview-src/index.ts`
- `'index-editor'` → `chat-webview-src/index-editor.ts`
- `'codicon'` → `node_modules/@vscode/codicons/dist/codicon.css`

Also uses `loader: { '.ttf': 'dataurl' }`. Output goes to `chat-webview-out/`.

**`extensions/markdown-math/esbuild.notebook.mts`**

Single entry point `notebook/katex.ts` (`esbuild.notebook.mts:30-32`). Uses the `didBuild` callback (`postBuild` function defined at lines 12-27). `postBuild` copies `katex.min.css` into `outDir` and copies all `.woff2` font files from katex's `dist/fonts/` into a `fonts/` subdirectory within `outDir`, using `fs-extra` (`fse`). This is the only consumer that passes a `didBuild` argument.

**`extensions/markdown-language-features/esbuild.webview.mts`**

Two unnamed array-style entry points: `preview-src/index.ts` and `preview-src/pre` (`esbuild.webview.mts:12-15`). No additionalOptions. Output to `media/`.

**`extensions/markdown-language-features/esbuild.notebook.mts`**

Single entry point `notebook/index.ts`. No additionalOptions. Output to `notebook-out/`.

**`extensions/notebook-renderers/esbuild.notebook.mts`**

Single entry point `src/index.ts`. No additionalOptions. Output to `renderer-out/`.

**`extensions/ipynb/esbuild.notebook.mts`**

Single entry point `notebook-src/cellAttachmentRenderer.ts`. No additionalOptions. Output to `notebook-out/`.

---

### Cross-Cutting Synthesis

`esbuild-webview-common.mts` is a pure build-time utility module with no runtime presence. It centralises the esbuild configuration contract for all VS Code extension webview bundles: browser-targeted (`platform: 'browser'`), ECMAScript module format (`format: 'esm'`), ES2024 syntax target, bundled and minified, with sourcemaps off. The `--outputRoot` argument-override mechanism allows the VS Code packaging pipeline to redirect outputs under a unified distribution tree without changing per-extension build scripts. The `--watch` path integrates an esbuild incremental context with a typed `didBuild` callback so that extensions needing additional asset-copy steps (e.g., `markdown-math` copying KaTeX fonts) can do so on every rebuild cycle without modifying the core watch logic. The seven consumers are all leaf-level scripts invoked directly by npm build commands; none of them re-export or further compose the shared logic. The pattern is a clean Strategy-by-injection design: fixed defaults for all webview bundles, open override surface via `additionalOptions`, and a lifecycle hook (`didBuild`) for post-processing. For a Tauri/Rust port, this module is entirely in the build toolchain layer; the actual artifact it produces — ESM bundles loaded into VS Code's `WebviewPanel` — would need an equivalent browser-content loading mechanism in whatever WebView API Tauri exposes, but the build script itself has no host-platform dependency.

---

### Out-of-Partition References

The following files outside partition 59 import or directly invoke `esbuild-webview-common.mts`:

| Consumer File | Entry Points Compiled | Post-Build Callback | Output Directory |
|---|---|---|---|
| `extensions/simple-browser/esbuild.webview.mts` | `preview-src/index.ts`, codicon CSS | None | `media/` |
| `extensions/mermaid-chat-features/esbuild.webview.mts` | `chat-webview-src/index.ts`, `index-editor.ts`, codicon CSS | None | `chat-webview-out/` |
| `extensions/markdown-math/esbuild.notebook.mts` | `notebook/katex.ts` | `postBuild` (copies KaTeX CSS + `.woff2` fonts) | `notebook-out/` |
| `extensions/markdown-language-features/esbuild.webview.mts` | `preview-src/index.ts`, `preview-src/pre` | None | `media/` |
| `extensions/markdown-language-features/esbuild.notebook.mts` | `notebook/index.ts` | None | `notebook-out/` |
| `extensions/notebook-renderers/esbuild.notebook.mts` | `src/index.ts` | None | `renderer-out/` |
| `extensions/ipynb/esbuild.notebook.mts` | `notebook-src/cellAttachmentRenderer.ts` | None | `notebook-out/` |

No runtime source files in the repository import from `esbuild-webview-common.mts`; consumption is exclusively at build-script level.
