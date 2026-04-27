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

