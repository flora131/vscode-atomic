### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/src/extension.ts` (46 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/notebook/katex.ts` (58 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/esbuild.mts` (18 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/esbuild.browser.mts` (21 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/esbuild.notebook.mts` (35 lines)
- `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/package.json` (124 lines)

---

### Per-File Notes

#### `src/extension.ts`

- **Role**: Node.js extension host entry point. Exports `activate(context: vscode.ExtensionContext)` at line 11.
- **Imports**: `import * as vscode from 'vscode'` at line 5. The KaTeX plugin itself is loaded lazily via a bare `require('@vscode/markdown-it-katex').default` at line 31, inside the `extendMarkdownIt` callback, so it is only required when the markdown preview is actually rendered.
- **Configuration reads**:
  - `vscode.workspace.getConfiguration('markdown').get<boolean>('math.enabled', true)` at line 14, controlling the `isEnabled()` guard.
  - `vscode.workspace.getConfiguration('markdown').get<{[key:string]:string}>('math.macros', {})` at line 19, retrieving user-defined LaTeX macro expansions.
- **Event subscription**: `vscode.workspace.onDidChangeConfiguration` at line 22 listens for changes scoped to the `markdown.math` key and issues `vscode.commands.executeCommand('markdown.api.reloadPlugins')` at line 24 to refresh the preview pipeline.
- **Return value**: The activation function returns a plain object `{ extendMarkdownIt(md) }` at line 29. This is the VS Code Markdown extension API contract: any extension that sets `"markdown.markdownItPlugins": true` in its `package.json` can export this shape from `activate()` and have it called by the built-in markdown extension's preview renderer.
- **KaTeX options passed to the plugin** (lines 33–37): `enableFencedBlocks: true`, `globalGroup: true`, `macros` (merged from user config). A `core.ruler` push at line 38 resets the macro dictionary at the start of each document render so macros do not bleed across documents.
- **No Electron/native API surface**: The file uses only the public `vscode` module and Node.js `require`.

#### `notebook/katex.ts`

- **Role**: Webview / notebook renderer script, executed inside the browser context of VS Code's notebook renderer, not in the extension host. This is declared in `package.json` under `contributes.notebookRenderer` at line 71–79.
- **Imports**:
  - `import type * as markdownIt from 'markdown-it'` at line 5 — type-only import, no runtime dependency on the module itself.
  - `import type { RendererContext } from 'vscode-notebook-renderer'` at line 6 — the VS Code notebook renderer API, available only inside a notebook renderer webview context.
- **Activation signature**: `export async function activate(ctx: RendererContext<void>)` at line 10. This is the standard notebook renderer entry point contract.
- **Style injection** (lines 8, 16–46):
  - `import.meta.url` at line 8 is used to derive the URL of `katex.min.css` relative to the current script bundle; this is a standard ESM browser pattern.
  - Two `<link>` elements are created at lines 17–29: one added to `document.head` so fonts load from the root document (working around a Chromium shadow-DOM font-loading limitation noted in a bug comment at line 25), and a second placed inside a `<template class="markdown-style">` at lines 42–46, which the notebook renderer copies into each output cell's shadow DOM.
  - An inline `<style>` block at lines 31–39 sets `.katex-error` to use `var(--vscode-editorError-foreground)` and resets equation counters on `.katex-block`.
- **Plugin registration**: `markdownItRenderer.extendMarkdownIt` at line 50 calls `md.use(katex, {...})` to attach the KaTeX plugin to the shared `markdown-it` instance managed by the `vscode.markdown-it-renderer` built-in renderer.
- **No Electron/native API surface**: Only DOM APIs, ESM `import.meta.url`, and the `vscode-notebook-renderer` webview API are used.

#### `esbuild.mts`

- **Role**: Build script for the Node.js extension host bundle.
- **Key detail**: Calls `run(...)` from the shared `../esbuild-extension-common.mts` helper at line 6 with `platform: 'node'`, entry point `src/extension.ts`, output into `dist/`. Process arguments are forwarded at line 18 to support `--watch` mode.

#### `esbuild.browser.mts`

- **Role**: Build script for the browser-based extension host bundle (used in VS Code for the Web / vscode.dev).
- **Key detail**: Same `run(...)` helper as above at line 6 but with `platform: 'browser'` at line 12 and a `tsconfig.browser.json` override at line 19. Output goes to `dist/browser/`.

#### `esbuild.notebook.mts`

- **Role**: Build script for the notebook webview renderer bundle (`notebook/katex.ts` → `notebook-out/katex.js`).
- **Uses**: `../esbuild-webview-common.mts` helper at line 7 (distinct from the extension-common helper, reflecting that this target runs in a browser/webview rather than the extension host).
- **Post-build step** (`postBuild` function, lines 12–27): Copies `node_modules/katex/dist/katex.min.css` into `notebook-out/` and copies all `.woff2` font files from `node_modules/katex/dist/fonts/` into `notebook-out/fonts/`. This makes the KaTeX stylesheet and fonts available as static assets served alongside the renderer bundle at runtime.

#### `package.json`

- **`main`**: `./out/extension` (Node.js extension host, line 23).
- **`browser`**: `./dist/browser/extension` (browser extension host, line 24).
- **`activationEvents`**: empty array (line 25) — activation is lazy, triggered by the markdown preview infrastructure.
- **`contributes.markdown.markdownItPlugins`**: `true` at line 81 — opts this extension into the markdown preview plugin API.
- **`contributes.markdown.previewStyles`**: lists `./notebook-out/katex.min.css` and `./preview-styles/index.css` at lines 83–85, injected into the markdown preview webview.
- **`contributes.notebookRenderer`**: id `vscode.markdown-it-katex-extension` at line 73, extending `vscode.markdown-it-renderer` with entry point `./notebook-out/katex.js`.
- **`contributes.grammars`**: Four TextMate grammar injections at lines 34–69 provide syntax highlighting for `$$...$$` block math, `$...$` inline math, and fenced `math` blocks inside Markdown files.
- **Runtime dependency**: `@vscode/markdown-it-katex: ^1.1.2` at line 122 — the sole npm runtime dependency.

---

### Cross-Cutting Synthesis

The extension contains two separate runtime surfaces that are built independently:

1. **Extension host surface** (`src/extension.ts`): Runs in the Node.js (or browser) extension host process. Communicates entirely through the public `vscode` module API — configuration reads, command execution, and the `extendMarkdownIt` callback contract. Has no access to Electron internals, no IPC channels, no native modules.

2. **Notebook renderer surface** (`notebook/katex.ts`): Runs inside a sandboxed webview (the notebook output renderer). Communicates through the `vscode-notebook-renderer` API (`RendererContext`, `getRenderer`). Uses only DOM APIs and ESM module semantics. No Node.js, no Electron.

Both surfaces delegate the actual KaTeX rendering to `@vscode/markdown-it-katex`, which is a `markdown-it` plugin wrapping the KaTeX library. The extension itself contains no rendering logic.

The three esbuild configs are build-time tooling only; they do not affect runtime behavior and depend solely on shared helpers under `extensions/esbuild-extension-common.mts` and `extensions/esbuild-webview-common.mts`.

---

### Out-of-Partition References

- `extensions/esbuild-extension-common.mts` — shared build helper imported by `esbuild.mts` (line 6) and `esbuild.browser.mts` (line 6).
- `extensions/esbuild-webview-common.mts` — shared build helper imported by `esbuild.notebook.mts` (line 7).
- `vscode` (built-in module) — consumed in `src/extension.ts` at line 5.
- `vscode-notebook-renderer` (type package, `@types/vscode-notebook-renderer`) — consumed in `notebook/katex.ts` at line 6.
- `@vscode/markdown-it-katex` — runtime KaTeX plugin, loaded dynamically in both `src/extension.ts` (line 31) and `notebook/katex.ts` (line 48).
- `vscode.markdown-it-renderer` — built-in VS Code notebook renderer referenced by id in `notebook/katex.ts` at line 11 and in `package.json` at line 78.

---

The `extensions/markdown-math/` partition is non-relevant to the Tauri/Rust port. It is a self-contained VS Code extension that communicates exclusively through the public `vscode` extension API (configuration reads, command execution, the `extendMarkdownIt` markdown preview plugin contract, and the `RendererContext` notebook renderer API). It carries no Electron-specific code, no native bindings, no IPC wiring, and no workbench internals. Its sole runtime dependency is the `@vscode/markdown-it-katex` npm package. Both the extension host and the notebook renderer surfaces would function identically inside any VS Code-compatible extension host that implements the same public API surface, making this partition fully portable without modification as long as the target runtime honours the VS Code Extension API and the notebook renderer protocol.
