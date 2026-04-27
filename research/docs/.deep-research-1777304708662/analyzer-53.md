### Files Analysed

- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/src/extension.ts` (46 LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/notebook/katex.ts` (59 LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/package.json` (125 LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/esbuild.mts` (18 LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/esbuild.browser.mts` (22 LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/esbuild.notebook.mts` (36 LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/syntaxes/md-math.tmLanguage.json` (107 LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/preview-styles/index.css` (9 LOC)

---

### Per-File Notes

#### `src/extension.ts`

**Role**: The main extension entry point for the markdown preview context. It exports a single `activate()` function that hooks into VS Code's built-in markdown rendering pipeline.

**Key Symbols**

- `markdownMathSetting` (line 9): String constant `'markdown.math'` used as the config watch key.
- `activate(context: vscode.ExtensionContext)` (line 11): The sole exported function, called by the VS Code host at extension activation.
- `isEnabled()` (line 12–15): Reads `markdown.math.enabled` from workspace configuration; defaults to `true`.
- `getMacros()` (line 17–20): Reads `markdown.math.macros` as a `{[key:string]:string}` dictionary from workspace configuration; defaults to `{}`.

**Control Flow**

1. `activate()` registers a `vscode.workspace.onDidChangeConfiguration` listener (line 22–26) scoped to `markdownMathSetting`. When that setting changes, it fires the VS Code command `'markdown.api.reloadPlugins'` (line 24), triggering the host markdown preview to re-request the plugin.
2. `activate()` returns a plain object with a single method `extendMarkdownIt(md)` (line 29). This is the extension point defined by VS Code's markdown preview API, declared in `package.json` via `"markdown.markdownItPlugins": true` (line 81 of `package.json`).
3. Inside `extendMarkdownIt(md)` (line 29–44):
   - Calls `isEnabled()` to gate the entire plugin.
   - If enabled, loads `@vscode/markdown-it-katex` via a dynamic `require()` call (line 31). The `declare function require` at line 7 accommodates this runtime require in a TypeScript ESM context.
   - Calls `getMacros()` to get the current macro dictionary (line 32).
   - Constructs an `options` object (lines 33–37) with `enableFencedBlocks: true`, `globalGroup: true`, and a spread copy of `settingsMacros` as the initial `macros` state.
   - Pushes a custom rule `'reset-katex-macros'` onto `md.core.ruler` (line 38–40). This rule executes at the start of each parse cycle and resets `options.macros` to a fresh spread of `settingsMacros`, preventing macro state from leaking across documents.
   - Calls `md.use(katex, options)` (line 41) to register the KaTeX plugin into the markdown-it instance and returns the augmented `md`.
   - If disabled, returns `md` unmodified (line 43).

**Data Flow**

- Configuration values (`math.enabled`, `math.macros`) flow from the VS Code workspace configuration API into the `options` object passed to `md.use()`.
- The returned `extendMarkdownIt` object is consumed by the VS Code host's markdown preview renderer, which owns the `markdown-it` instance (`md`) and calls `extendMarkdownIt` each time the plugin pipeline is rebuilt.

**Dependencies**

- `vscode` API: `workspace.getConfiguration`, `workspace.onDidChangeConfiguration`, `commands.executeCommand`, `ExtensionContext` type.
- `@vscode/markdown-it-katex`: Loaded at runtime via `require()`, not statically imported.

---

#### `notebook/katex.ts`

**Role**: A separate activation module for the notebook renderer context. It extends the notebook's `vscode.markdown-it-renderer` with KaTeX support and injects necessary CSS styles into the shadow DOM and parent document.

**Key Symbols**

- `styleHref` (line 8): Computed at module load time using `import.meta.url` to derive the URL of `katex.min.css` relative to the compiled `katex.js` file by replacing the filename suffix.
- `activate(ctx: RendererContext<void>)` (line 10): Async function exported as the renderer entry point; `RendererContext<void>` is typed from `vscode-notebook-renderer`.

**Control Flow**

1. Calls `ctx.getRenderer('vscode.markdown-it-renderer')` (line 11) to retrieve the base markdown-it renderer instance. If this returns `undefined`, an error is thrown (line 13).
2. Creates two `<link>` elements for the KaTeX CSS stylesheet (lines 16–29):
   - `link` (line 17–21): Tagged with `class="markdown-style"` so the host can copy it into the shadow DOM of each notebook output cell.
   - `linkHead` (line 22–29): Appended directly to `document.head` to work around a Chromium bug (referenced at line 25 via https://bugs.chromium.org/p/chromium/issues/detail?id=336876) where fonts referenced by a shadow DOM stylesheet fail to load unless also requested from the parent document.
3. Creates a `<style>` element (lines 31–39) with two inline rules:
   - `.katex-error`: Sets color to the VS Code editor error foreground CSS variable.
   - `.katex-block`: Resets `counter-reset` for KaTeX equation numbering.
4. Assembles both the `<style>` and `<link>` into a `<template>` element with `class="markdown-style"` (lines 41–46) and appends it to `document.head`. The class name convention signals to the host renderer that this template's content should be cloned into each cell's shadow DOM.
5. Loads `@vscode/markdown-it-katex` via a synchronous `require()` call (line 48).
6. Creates a shared `macros` object (line 49) — intentionally mutable and shared across all notebook cells (the `globalGroup: true` option at line 52 means KaTeX macro definitions accumulate across parse calls within the same session).
7. Calls `markdownItRenderer.extendMarkdownIt(callback)` (line 50–57), passing a callback that invokes `md.use(katex, { globalGroup: true, enableBareBlocks: true, enableFencedBlocks: true, macros })`.

**Data Flow**

- `ctx.getRenderer(...)` returns a renderer API object; its `extendMarkdownIt` method is the injection point.
- The `macros` object is shared by reference across all `md.use()` invocations for the lifetime of the renderer, so macro definitions made during one render cycle persist into the next (enabled by `globalGroup: true`).
- Style content flows from the KaTeX npm package's `katex.min.css` (copied to `notebook-out/` at build time by `esbuild.notebook.mts`) into the notebook's shadow DOM and parent document head.

**Differences from `extension.ts`**

- No `isEnabled()` gate — the notebook renderer is always active once loaded.
- No macro reset rule — macros accumulate globally across notebook cells.
- Adds `enableBareBlocks: true` (absent in the preview extension).
- Uses `RendererContext` from `vscode-notebook-renderer` rather than the `vscode` module.
- Uses `import.meta.url` for CSS path resolution (ESM webview context) rather than `__dirname`-style paths.

---

#### `package.json`

**Role**: Extension manifest declaring all VS Code integration points, dependencies, and build scripts.

**Key Registration Points**

- `"main": "./out/extension"` (line 24): Node.js extension host entry point (compiled output).
- `"browser": "./dist/browser/extension"` (line 25): Browser extension host entry point (built by `esbuild.browser.mts`).
- `"activationEvents": []` (line 25): Empty array means VS Code auto-detects activation from `contributes`.
- `"markdown.markdownItPlugins": true` (line 81): Declares participation in the markdown preview plugin protocol. The VS Code built-in markdown extension calls `extendMarkdownIt()` on the object returned from `activate()`.
- `"markdown.previewStyles"` (lines 82–85): Registers two CSS files for injection into the markdown preview webview:
  - `./notebook-out/katex.min.css` — the full KaTeX stylesheet copied at build time.
  - `./preview-styles/index.css` — a minimal override setting `.katex-error` color via VS Code CSS variable.
- `"notebookRenderer"` (lines 71–80): Registers the notebook renderer with id `vscode.markdown-it-katex-extension`, which extends the base renderer `vscode.markdown-it-renderer` and points to `./notebook-out/katex.js` as its entry point.
- `"grammars"` (lines 33–69): Four TextMate grammar contributions:
  - `text.html.markdown.math` (standalone math language, `md-math.tmLanguage.json`)
  - `markdown.math.block` (injected into `text.html.markdown` for block math delimiters, embeds `latex`)
  - `markdown.math.inline` (injected into `text.html.markdown` for inline math, embeds `latex`)
  - `markdown.math.codeblock` (injected into `text.html.markdown` for fenced math blocks, embeds `latex`)
- `"configuration"` (lines 86–106): Two user-facing settings:
  - `markdown.math.enabled` (boolean, default `true`): Gates the entire plugin.
  - `markdown.math.macros` (object of string→string, default `{}`): User-defined KaTeX macros, scoped to `"resource"`.

**Build Scripts**

- `"compile"` and `"watch"` (lines 109–111): Both point to `node ./esbuild.notebook.mts`. The main extension and browser builds are driven by the monorepo-level build system using `esbuild.mts` and `esbuild.browser.mts` respectively.

**Dependency**

- `"@vscode/markdown-it-katex": "^1.1.2"` (line 122): The sole runtime dependency. This package wraps KaTeX into a markdown-it plugin and also bundles the KaTeX CSS and fonts transitively.

---

#### `esbuild.notebook.mts`

**Role**: Build script for the notebook renderer bundle.

**Key Behavior** (lines 29–35): Calls a shared `run()` utility from `../esbuild-webview-common.mts` with `notebook/katex.ts` as the single entry point, outputting to `notebook-out/`. After bundling, the `postBuild` hook (lines 12–27) copies `katex.min.css` and all `.woff2` font files from the `katex` npm package dist into `notebook-out/` so they are co-located with the bundled renderer script. The CSS URL computed at runtime in `notebook/katex.ts:8` via `import.meta.url` resolves correctly because of this co-location.

---

#### `esbuild.mts` and `esbuild.browser.mts`

**Roles**: Node and browser build scripts for the main extension.

- `esbuild.mts` (lines 11–18): Calls `run()` from `../esbuild-extension-common.mts` with `platform: 'node'`, entry `src/extension.ts`, output to `dist/`.
- `esbuild.browser.mts` (lines 11–21): Same pattern but `platform: 'browser'`, output to `dist/browser/`, with an additional `tsconfig` option pointing to `tsconfig.browser.json`.

Both delegate all bundling logic to the shared `esbuild-extension-common.mts` utility in the parent `extensions/` directory.

---

#### `syntaxes/md-math.tmLanguage.json`

**Role**: TextMate grammar for the `markdown-math` language scope (`text.html.markdown.math`). Defines token patterns for LaTeX math content including comments (`%`), line separators (`\\`), function calls (`\cmd{...}`), commands (`\cmd`), brackets, numeric literals, and operators. This grammar is used for syntax highlighting inside math delimiters in the editor.

---

#### `preview-styles/index.css`

**Role**: Single-rule CSS override for the markdown preview webview. Sets `.katex-error { color: var(--vscode-editorError-foreground); }` using a VS Code theme CSS variable. This mirrors the equivalent rule injected inline in `notebook/katex.ts:32–38` for the notebook renderer context.

---

### Cross-Cutting Synthesis

The `markdown-math` extension operates in two distinct VS Code rendering contexts — the markdown preview webview and the notebook output renderer — using separate but parallel activation paths. The preview path (`src/extension.ts`) participates in VS Code's markdown plugin protocol by returning an `extendMarkdownIt()` method from `activate()`; this method is called by the host's markdown preview engine, which owns the `markdown-it` instance. The notebook path (`notebook/katex.ts`) participates in the notebook renderer extension protocol by receiving a `RendererContext` and calling `extendMarkdownIt` on the resolved base renderer. Both paths inject `@vscode/markdown-it-katex` into a `markdown-it` instance via `md.use()`, but they differ in macro persistence (preview resets per document; notebook accumulates globally), enabled feature flags (`enableBareBlocks` is notebook-only), and CSS delivery (preview uses static `markdown.previewStyles` registration; notebook uses DOM manipulation with shadow DOM awareness and a Chromium font-loading workaround). Four TextMate grammars are injected into the `text.html.markdown` scope to provide editor-side syntax highlighting for inline, block, and fenced math delimiters, independent of either render path. The entire extension has a single runtime npm dependency — `@vscode/markdown-it-katex` — and its build pipeline uses three separate esbuild scripts delegating to shared monorepo utilities.

For a Tauri/Rust port, the coupling points that require the deepest analysis are: (1) the `vscode.workspace.getConfiguration` API and its change notification mechanism, both consumed at `src/extension.ts:13–14` and `22–26`; (2) the `markdown.markdownItPlugins` protocol at `package.json:81`, which is a VS Code-specific extension API with no direct Tauri equivalent; (3) the `RendererContext` from `vscode-notebook-renderer` at `notebook/katex.ts:6,10–11`, which is VS Code's notebook renderer IPC interface; (4) the `vscode.commands.executeCommand('markdown.api.reloadPlugins')` call at `src/extension.ts:24`, which is an internal VS Code command; and (5) the shadow DOM + `markdown-style` template convention for style injection in notebook outputs at `notebook/katex.ts:18–46`, which is specific to VS Code's webview architecture.

---

### Out-of-Partition References

The following symbols and modules are referenced by the analysed files but are defined outside the `extensions/markdown-math/` partition:

- `vscode` module (imported at `src/extension.ts:5`): The VS Code extension host API. Provides `ExtensionContext`, `workspace.getConfiguration`, `workspace.onDidChangeConfiguration`, and `commands.executeCommand`. Defined in the VS Code host process; type declarations come from the `@types/vscode` devDependency (not declared in this extension's `package.json` — inherited from the monorepo).
- `vscode-notebook-renderer` module (type-imported at `notebook/katex.ts:6`): Provides the `RendererContext<T>` interface for notebook renderer activation. Declared in the `@types/vscode-notebook-renderer` devDependency at `package.json:115`.
- `markdown-it` module (type-imported at `notebook/katex.ts:5`): The markdown-it parser type definitions. Declared in the `@types/markdown-it` devDependency at `package.json:114`. The runtime instance is owned by the VS Code host, not this extension.
- `../esbuild-webview-common.mts` (imported at `esbuild.notebook.mts:7`): Shared webview build utility in the parent `extensions/` directory. Exports a `run()` function handling esbuild invocation for webview/renderer bundles.
- `../esbuild-extension-common.mts` (imported at both `esbuild.mts:6` and `esbuild.browser.mts:6`): Shared extension build utility in the parent `extensions/` directory. Exports a `run()` function for Node and browser extension bundles.
- `@vscode/markdown-it-katex` (runtime `require()` at `src/extension.ts:31` and `notebook/katex.ts:48`): npm package wrapping KaTeX as a markdown-it plugin. Listed as a runtime dependency at `package.json:122`. Its bundled KaTeX distribution also provides the `katex.min.css` and `.woff2` fonts copied by `esbuild.notebook.mts:13–26`.
- `vscode.markdown-it-renderer` (resolved via `ctx.getRenderer(...)` at `notebook/katex.ts:11`): The built-in VS Code notebook renderer identified by the string ID `'vscode.markdown-it-renderer'`. This is a separate built-in extension, not part of this partition.
- `markdown.api.reloadPlugins` command (executed at `src/extension.ts:24`): An internal VS Code command registered by the built-in markdown preview extension that triggers a full rebuild of the markdown-it plugin pipeline.
- TextMate grammar injection target `text.html.markdown` (referenced at `package.json:43,53,63`): The base Markdown TextMate scope provided by VS Code's built-in markdown language support extension.
- `katex` npm sub-package (path traversed at `esbuild.notebook.mts:14,17`): The `katex` package nested under `node_modules/katex` (a transitive dependency of `@vscode/markdown-it-katex`) provides the `dist/katex.min.css` and `dist/fonts/*.woff2` files copied into `notebook-out/` at build time.
