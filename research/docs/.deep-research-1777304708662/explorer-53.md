# Partition 53 of 79 — Findings

## Scope
`extensions/markdown-math/` (5 files, 177 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# VS Code Markdown Math Extension - File Locations for Tauri/Rust Porting Analysis

## Implementation

- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/src/extension.ts` (46 LOC) — Main extension entry point implementing VS Code Extension API. Exports `activate()` function that returns object with `extendMarkdownIt()` method. Loads `@vscode/markdown-it-katex` plugin, reads configuration from `markdown.math.*` settings, manages macros via workspace configuration, and registers configuration change listener.
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/notebook/katex.ts` (59 LOC) — Notebook renderer activation module for canvas-based rendering. Implements `activate(ctx: RendererContext<void>)` to extend markdown-it renderer. Handles CSS stylesheet injection into shadow DOM and configures KaTeX rendering with macro support for notebook contexts.

## Types / Interfaces

- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/tsconfig.json` — TypeScript configuration extending base config, targeting `./src`, outputs to `./out`. Includes VS Code type definitions from `vscode.d.ts`.
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/tsconfig.browser.json` — Browser-specific TypeScript configuration for bundling extension with browser platform target.
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/notebook/tsconfig.json` — TypeScript configuration for notebook renderer compilation.

## Configuration

- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/package.json` (125 LOC) — Extension manifest declaring:
  - Activation entry points: `./out/extension` (Node.js) and `./dist/browser/extension` (browser)
  - Grammar injections for markdown-math language with 4 TextMate syntax scopes
  - Notebook renderer registration (`vscode.markdown-it-katex-extension`) with entrypoint extending `vscode.markdown-it-renderer`
  - Configuration schema defining `markdown.math.enabled` (boolean, default true) and `markdown.math.macros` (object, resource scope)
  - Preview styles including KaTeX CSS stylesheet
  - Dependencies: `@vscode/markdown-it-katex@^1.1.2`
  - Capabilities: virtual workspaces and untrusted workspace support
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/package-lock.json` — Dependency lock file
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/package.nls.json` — Localization/translation strings

## Documentation

- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/README.md` (6 LOC) — Basic overview noting bundled status with VS Code, describes KaTeX math rendering for markdown preview and notebook cells.
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/cgmanifest.json` — Component governance manifest

## Examples / Fixtures

- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/preview-styles/index.css` — Custom CSS for markdown preview styling

## Notable Clusters

**Syntax Definition Files** (4 TextMate grammar files):
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/syntaxes/md-math.tmLanguage.json`
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/syntaxes/md-math-block.tmLanguage.json`
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/syntaxes/md-math-fence.tmLanguage.json`
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/syntaxes/md-math-inline.tmLanguage.json`

**Build Configuration** (3 esbuild scripts):
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/esbuild.mts` — Node.js platform bundling
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/esbuild.browser.mts` — Browser platform bundling
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/esbuild.notebook.mts` — Notebook renderer bundling

---

## Porting Analysis Summary

The markdown-math extension demonstrates VS Code's dual-platform extension architecture. The extension entry point (`extension.ts`) implements the VS Code Extension API via the activation hook pattern—it exports an `activate()` function returning an object with `extendMarkdownIt()` method. This is the core integration point with VS Code's markdown pipeline.

For Tauri/Rust porting, the critical dependencies are: (1) the markdown-it plugin hook mechanism (the `extendMarkdownIt(md: any)` contract), (2) the VS Code settings API (`vscode.workspace.getConfiguration()`), (3) command execution API (`vscode.commands.executeCommand()`), and (4) workspace configuration change notifications. The actual KaTeX rendering is delegated to the npm package `@vscode/markdown-it-katex`, which is a JavaScript/Node.js library. A Rust port would need to either wrap KaTeX via FFI or port/reimplement the math rendering logic. Additionally, the extension defines 4 TextMate syntax definitions (injected into markdown scope) and a notebook renderer using the `RendererContext` API—both abstractions that would require Rust-side equivalents in a Tauri environment.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Analysis: markdown-math Extension Patterns

Research scope: Port VS Code from TypeScript/Electron to Tauri/Rust
Focus: `extensions/markdown-math/` (5 files, 177 LOC) — KaTeX rendering

## Patterns Found

#### Pattern: Extension API Plugin Hook
**Where:** `extensions/markdown-math/src/extension.ts:28-45`
**What:** Returns object with extendMarkdownIt hook for markdown-it plugin integration.
```typescript
return {
	extendMarkdownIt(md: any) {
		if (isEnabled()) {
			const katex = require('@vscode/markdown-it-katex').default;
			const settingsMacros = getMacros();
			const options = {
				enableFencedBlocks: true,
				globalGroup: true,
				macros: { ...settingsMacros }
			};
			md.core.ruler.push('reset-katex-macros', () => {
				options.macros = { ...settingsMacros };
			});
			return md.use(katex, options);
		}
		return md;
	}
};
```
**Variations:** Also seen in `extensions/markdown-language-features/src/markdownExtensions.ts:86-91` where plugins are discovered via manifest and activated on-demand.

#### Pattern: Configuration-Driven Feature Toggle
**Where:** `extensions/markdown-math/src/extension.ts:11-20`
**What:** Feature enabled/disabled via workspace configuration with default true value; macros loaded from settings.
```typescript
export function activate(context: vscode.ExtensionContext) {
	function isEnabled(): boolean {
		const config = vscode.workspace.getConfiguration('markdown');
		return config.get<boolean>('math.enabled', true);
	}

	function getMacros(): { [key: string]: string } {
		const config = vscode.workspace.getConfiguration('markdown');
		return config.get<{ [key: string]: string }>('math.macros', {});
	}
```
**Variations:** Uses namespaced settings (`markdown.math.enabled`, `markdown.math.macros`).

#### Pattern: Configuration Change Listener with Command Dispatch
**Where:** `extensions/markdown-math/src/extension.ts:22-26`
**What:** Listens for configuration changes and dispatches reload command when affected settings change.
```typescript
vscode.workspace.onDidChangeConfiguration(e => {
	if (e.affectsConfiguration(markdownMathSetting)) {
		vscode.commands.executeCommand('markdown.api.reloadPlugins');
	}
}, undefined, context.subscriptions);
```
**Variations:** See `extensions/markdown-language-features/src/commands/reloadPlugins.ts` which defines the actual reload handler that calls `engine.reloadPlugins()`, `engine.cleanCache()`, and `webviewManager.refresh()`.

#### Pattern: Notebook Renderer Context Pattern
**Where:** `extensions/markdown-math/notebook/katex.ts:10-57`
**What:** Notebook renderers activate with RendererContext, fetch parent renderer via getRenderer, then extend its API.
```typescript
export async function activate(ctx: RendererContext<void>) {
	const markdownItRenderer = (await ctx.getRenderer('vscode.markdown-it-renderer')) as undefined | any;
	if (!markdownItRenderer) {
		throw new Error(`Could not load 'vscode.markdown-it-renderer'`);
	}

	// ... stylesheet setup ...

	markdownItRenderer.extendMarkdownIt((md: markdownIt.MarkdownIt) => {
		return md.use(katex, {
			globalGroup: true,
			enableBareBlocks: true,
			enableFencedBlocks: true,
			macros,
		});
	});
}
```
**Variations:** Similar pattern in `extensions/ipynb/notebook-src/cellAttachmentRenderer.ts:14-46` extends markdown-it to modify image rendering rules.

#### Pattern: Dynamic Stylesheet Injection for Shadow DOM
**Where:** `extensions/markdown-math/notebook/katex.ts:8-46`
**What:** Loads stylesheet once to document head for shadow DOM font loading, adds templates to document head for shadow DOM cloning.
```typescript
const styleHref = import.meta.url.replace(/katex.js$/, 'katex.min.css');

const link = document.createElement('link');
link.rel = 'stylesheet';
link.classList.add('markdown-style');
link.href = styleHref;

const linkHead = document.createElement('link');
linkHead.rel = 'stylesheet';
linkHead.href = styleHref;
document.head.appendChild(linkHead);

const styleTemplate = document.createElement('template');
styleTemplate.classList.add('markdown-style');
styleTemplate.content.appendChild(style);
styleTemplate.content.appendChild(link);
document.head.appendChild(styleTemplate);
```
**Variations:** Pattern addresses Chromium bug #336876 where fonts don't load in shadow DOM without explicit document head registration.

#### Pattern: Post-Build Asset Copy with Font Subsetting
**Where:** `extensions/markdown-math/esbuild.notebook.mts:12-27`
**What:** Post-build step copies CSS and filters font files (woff2 only) from node_modules to output directory.
```typescript
function postBuild(outDir: string) {
	fse.copySync(
		path.join(import.meta.dirname, 'node_modules', 'katex', 'dist', 'katex.min.css'),
		path.join(outDir, 'katex.min.css'));

	const fontsDir = path.join(import.meta.dirname, 'node_modules', 'katex', 'dist', 'fonts');
	const fontsOutDir = path.join(outDir, 'fonts/');

	fse.mkdirSync(fontsOutDir, { recursive: true });

	for (const file of fse.readdirSync(fontsDir)) {
		if (file.endsWith('.woff2')) {
			fse.copyFileSync(path.join(fontsDir, file), path.join(fontsOutDir, file));
		}
	}
}
```
**Variations:** Used in esbuild pipeline via `run(..., process.argv, postBuild)`.

#### Pattern: Manifest-Driven Plugin Discovery
**Where:** `extensions/markdown-math/package.json:81-106`
**What:** Extension declares markdown plugin capability via manifest properties and configuration schema.
```json
"contributes": {
  "markdown.markdownItPlugins": true,
  "markdown.previewStyles": [
    "./notebook-out/katex.min.css",
    "./preview-styles/index.css"
  ],
  "configuration": [
    {
      "title": "Markdown Math",
      "properties": {
        "markdown.math.enabled": {
          "type": "boolean",
          "default": true
        },
        "markdown.math.macros": {
          "type": "object",
          "additionalProperties": { "type": "string" }
        }
      }
    }
  ]
}
```
**Variations:** Used alongside entry point definitions for both Node (`./dist/extension`) and browser (`./dist/browser/extension`).

## Cross-Extension Patterns

The markdown-math extension demonstrates core patterns used throughout VS Code's extension system:

- **Plugin interface discovery**: via `markdown.markdownItPlugins` manifest and `extendMarkdownIt` export detection
- **Lazy activation**: plugins loaded on-demand when needed, not at startup
- **Configuration-driven behavior**: feature flags and customization parameters in workspace settings
- **Incremental updates**: configuration change listeners trigger targeted reload operations
- **Dual-build support**: Node.js (Electron) and browser platforms with separate esbuild configs
- **Asset embedding**: critical CSS/fonts copied to output; lazy-loaded via import.meta.url
- **Renderer composition**: notebook renderers compose via parent renderer API (getRenderer + extendMarkdownIt pattern)

The extension integrates with VS Code's markdown API via the `markdown.api.reloadPlugins` command, establishing a clean contract for plugin lifecycle management. This pattern scales across multiple extensions without direct coupling.

## External References
<!-- Source: codebase-online-researcher sub-agent -->
# Online Research: markdown-math / KaTeX in a Tauri/Rust Port

**Scope:** `extensions/markdown-math/` (5 source files, ~177 LOC)
**Dependency:** `@vscode/markdown-it-katex@^1.1.2`
**Research date:** 2026-04-27

---

## Summary

The `markdown-math` extension is a thin, renderer-side plugin that delegates all mathematical typesetting to KaTeX via the `@vscode/markdown-it-katex` markdown-it plugin. Its own code is trivially small: `src/extension.ts` simply registers the plugin with VS Code's markdown-it pipeline and surfaces two configuration settings (`markdown.math.enabled`, `markdown.math.macros`); `notebook/katex.ts` injects the KaTeX CSS into the notebook renderer's shadow DOM and extends the notebook's `vscode.markdown-it-renderer` in the same way.

Because KaTeX is a pure JavaScript library that renders LaTeX to HTML+CSS, the critical question for a Tauri/Rust port is not whether KaTeX can be rewritten in Rust, but how KaTeX's existing JS bundle is hosted and invoked in Tauri's webview.

---

## Detailed Findings

### 1. KaTeX is a JS-only, browser-native library

**Source:** https://katex.org/docs/browser

KaTeX ships as a self-contained JavaScript + CSS bundle. Its own documentation reads:

> "KaTeX supports all major browsers, including Chrome, Safari, Firefox, Opera, and Edge."

The library has no native binary component; rendering happens entirely in the browser's JavaScript engine. In a VS Code Electron context, KaTeX runs inside Electron's Chromium-based webview without any special ceremony. In a Tauri context it would run inside Tauri's webview (WKWebView on macOS, WebView2 on Windows, WebKitGTK on Linux) under identical conditions, because all three are standards-compliant browsers that execute JavaScript and CSS normally.

This means KaTeX itself requires **no porting work**. The `katex.min.js` and `katex.min.css` assets bundled by `@vscode/markdown-it-katex` can be loaded into a Tauri webview with no modification.

### 2. The markdown-it plugin layer (`@vscode/markdown-it-katex`) is also pure JS

The npm package `@vscode/markdown-it-katex` is a thin wrapper that:
- Parses `$...$` (inline) and `$$...$$` / fenced `math` blocks (block) using markdown-it's rule/core pipeline
- Calls `katex.renderToString()` for each matched token
- Returns the resulting HTML string back into the markdown-it output

All of this is standard Node.js/browser JavaScript. It does not use any VS Code SDK API, Electron API, or Node.js native addon. It is a pure-JS npm package that can run in any JavaScript environment — including the JS engine embedded in Tauri's webview.

### 3. What actually binds the extension to VS Code's extension host

The non-portable pieces in `src/extension.ts` are limited to three VS Code extension-host APIs:

- `vscode.workspace.getConfiguration(...)` — reads user settings
- `vscode.workspace.onDidChangeConfiguration(...)` — listens for setting changes
- `vscode.commands.executeCommand('markdown.api.reloadPlugins')` — tells the markdown preview to re-render

And the `extendMarkdownIt(md)` callback, which is a VS Code-specific protocol for the built-in markdown extension to collect third-party markdown-it plugins.

None of these have a Tauri equivalent; they are all VS Code extension-host IPC calls. The analog in Tauri would be:

- Configuration: Tauri's `tauri::plugin::Builder` or a JSON config file read via `tauri::api::path`; or simply a Tauri command exposed to the frontend that returns the user's math settings.
- Re-render trigger: a Tauri event emitted from Rust to the webview (`window.emit(...)` / `appHandle.emit_to(...)`), or a frontend-only reactive state update.
- `extendMarkdownIt`: replaced by directly initializing markdown-it with the KaTeX plugin in the webview's JS bundle (no plugin protocol needed — the frontend owns its own markdown-it instance).

### 4. Notebook renderer (`notebook/katex.ts`) maps to Tauri's webview injection

The notebook renderer code is more tightly coupled to VS Code's notebook renderer API (`vscode-notebook-renderer`, `RendererContext`, `ctx.getRenderer('vscode.markdown-it-renderer')`). These are VS Code-specific interfaces for the notebook output renderer sandbox.

In a Tauri application there is no notebook renderer protocol, but the functional goal — loading KaTeX CSS into a sandboxed shadow DOM and applying the markdown-it-katex plugin — can be achieved by:

1. Bundling `katex.min.css` and `katex.min.js` as Tauri static assets (via the `distDir` or custom protocol handler).
2. Injecting a `<link>` tag for the stylesheet and initializing markdown-it with the katex plugin inside the webview's JavaScript init code.
3. If notebook-style cell isolation is needed, using Shadow DOM directly in frontend JS — Tauri does not constrain this.

The Chromium bug referenced in `notebook/katex.ts` (https://bugs.chromium.org/p/chromium/issues/detail?id=336876, about font loading inside shadow DOM) is a browser-level issue. Tauri's WebView2 and WKWebView may or may not reproduce it; this would need empirical testing per platform.

### 5. Tauri webview compatibility with KaTeX

Tauri's architecture (from `https://v2.tauri.app/llms.txt` / concept docs) uses the OS's built-in webview:

- macOS/iOS: WKWebView (WebKit)
- Windows: WebView2 (Chromium-based)
- Linux: WebKitGTK

KaTeX supports all of these (Chrome, Safari, Firefox, Opera, Edge per KaTeX docs). WKWebView (Safari engine) can occasionally lag on CSS features; KaTeX's font loading via `@font-face` and its use of CSS `display: contents` etc. are generally well-supported in modern WebKit, but the shadow-DOM font-loading workaround already present in `notebook/katex.ts` may still be relevant.

### 6. Alternative: server-side (Rust-side) math rendering

Rather than running KaTeX in the webview, a Tauri application could render LaTeX to SVG or HTML server-side in Rust. The main Rust option is the `latex2mathml` crate (limited scope) or calling KaTeX via a JS runtime embedded in Rust:

- **`deno_core` / `v8`**: Execute KaTeX's `renderToString()` in a V8 isolate from Rust, return the HTML string to the frontend as plain HTML. This avoids shipping KaTeX JS to the webview but adds significant complexity and binary size.
- **MathJax / `mathjax-node`**: Similar approach via a Node.js sidecar.
- **`typst`**: A Rust-native typesetting system that supports math; not LaTeX-compatible but a full Rust alternative for new projects.

For a port of the existing VS Code `markdown-math` extension, the in-webview approach (simply reusing KaTeX JS) is by far the lowest-effort path and has no meaningful downside.

---

## Port Complexity Assessment

| Component | VS Code mechanism | Tauri equivalent | Effort |
|---|---|---|---|
| KaTeX rendering | `@vscode/markdown-it-katex` npm pkg in webview | Same npm pkg, loaded in Tauri webview JS bundle | Trivial — zero changes to KaTeX |
| markdown-it plugin registration | `extendMarkdownIt()` extension-host protocol | Direct `md.use(katex, opts)` call in frontend JS init | Low |
| Settings read (`math.enabled`, `math.macros`) | `vscode.workspace.getConfiguration` | Tauri command or config file read from Rust side | Low |
| Settings change event | `onDidChangeConfiguration` | Tauri event from backend, or frontend reactive state | Low |
| Re-render trigger | `markdown.api.reloadPlugins` command | Frontend event / reactive update | Low |
| Notebook renderer | `vscode-notebook-renderer` API | Custom webview component in frontend JS | Medium (protocol re-design needed only if notebooks are in scope) |
| KaTeX CSS / font assets | Bundled in extension directory, served by VS Code | Bundled in Tauri `distDir`, served via custom protocol | Low |
| Shadow DOM font-loading workaround | Chromium bug workaround in notebook renderer | Needs per-platform testing on WKWebView/WebView2 | Low–medium |

**Overall:** The `markdown-math` extension is one of the easiest components to port. Its rendering logic is entirely JS-in-webview and carries across to Tauri without modification. The only porting work is replacing the VS Code extension-host API calls (settings, events, plugin registration) with Tauri-native equivalents, and those APIs are simple enough that the total rewrite is measured in tens of lines, not hundreds.

---

## Additional Resources

- KaTeX browser docs: https://katex.org/docs/browser
- `@vscode/markdown-it-katex` npm package: https://www.npmjs.com/package/@vscode/markdown-it-katex
- Tauri architecture overview: https://v2.tauri.app/concept/architecture
- Tauri IPC (calling frontend from Rust): https://v2.tauri.app/develop/calling-frontend
- Tauri static asset embedding: https://v2.tauri.app/develop/resources
- `typst` Rust math typesetting alternative: https://github.com/typst/typst

---

## Gaps or Limitations

- No empirical testing was performed to verify KaTeX font loading behavior inside Tauri's WKWebView shadow DOM specifically; the Chromium-specific workaround in `notebook/katex.ts` may or may not transfer to WebKit cleanly.
- The scope of "notebook support" in a Tauri port is undefined; if notebooks (`.ipynb`) are not in scope, the `notebook/katex.ts` renderer can be ignored entirely, eliminating the medium-effort item above.
- `@vscode/markdown-it-katex` is a Microsoft-maintained fork of `markdown-it-katex`. A pure-upstream alternative (the community `markdown-it-katex` or `@iktakahiro/markdown-it-katex`) could be used instead if the VS Code-specific fork is not desired in a non-VS-Code codebase.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
