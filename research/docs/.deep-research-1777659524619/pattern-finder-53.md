# Pattern Finder: VS Code Markdown-Math Extension Architecture
## Port: TypeScript/Electron → Tauri/Rust

**Research Question:** Port VS Code core IDE functionality from TypeScript/Electron to Tauri/Rust.

**Scope:** `extensions/markdown-math/` (5 files, 177 LOC). Markdown-it extension hook — webview-side rendering plug-in.

---

#### Pattern: Extension Activation & Plugin Export Protocol
**Where:** `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/src/extension.ts:11-45`
**What:** Core extension entry point implementing the `extendMarkdownIt` contract that markdown-language-features discovers and invokes.

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

	vscode.workspace.onDidChangeConfiguration(e => {
		if (e.affectsConfiguration(markdownMathSetting)) {
			vscode.commands.executeCommand('markdown.api.reloadPlugins');
		}
	}, undefined, context.subscriptions);

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
}
```

**Variations:** `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-language-features/src/markdownExtensions.ts:80-94` (plugin discovery contract).

---

#### Pattern: Configuration Change Reactivity & Plugin Reloading
**Where:** `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/src/extension.ts:22-26`
**What:** Watch configuration changes and trigger markdown engine reload via command invocation.

```typescript
vscode.workspace.onDidChangeConfiguration(e => {
	if (e.affectsConfiguration(markdownMathSetting)) {
		vscode.commands.executeCommand('markdown.api.reloadPlugins');
	}
}, undefined, context.subscriptions);
```

**Variations:** `/home/norinlavaee/projects/vscode-atomic/extensions/references-view/src/references/index.ts:35-51` (generic configuration listener pattern with computed updates).

---

#### Pattern: Markdown-it Plugin Integration via Core Ruler
**Where:** `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/src/extension.ts:38-41`
**What:** Register custom markdown-it processing rules via `md.core.ruler.push` to inject state mutation hooks.

```typescript
md.core.ruler.push('reset-katex-macros', () => {
	options.macros = { ...settingsMacros };
});
return md.use(katex, options);
```

**Variations:** `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-language-features/src/markdownEngine.ts:19-39` (pluginSourceMap pattern with token iteration and attribute mutation).

---

#### Pattern: Notebook Renderer Context Activation
**Where:** `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/notebook/katex.ts:10-58`
**What:** Webview-side renderer initialization retrieving markdown-it renderer via `RendererContext.getRenderer()` and extending it asynchronously.

```typescript
export async function activate(ctx: RendererContext<void>) {
	const markdownItRenderer = (await ctx.getRenderer('vscode.markdown-it-renderer')) as undefined | any;
	if (!markdownItRenderer) {
		throw new Error(`Could not load 'vscode.markdown-it-renderer'`);
	}

	const katex = require('@vscode/markdown-it-katex').default;
	const macros = {};
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

**Variations:** `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/notebook-src/cellAttachmentRenderer.ts:14-46` (pattern with renderer rule override and metadata access).

---

#### Pattern: Dynamic Stylesheet Injection with Shadow DOM Scoping
**Where:** `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/notebook/katex.ts:8-46`
**What:** Create stylesheet links and template fragments for shadow DOM distribution and global head injection.

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

const style = document.createElement('style');
style.textContent = `
	.katex-error {
		color: var(--vscode-editorError-foreground);
	}
	.katex-block {
		counter-reset: katexEqnNo mmlEqnNo;
	}
`;

const styleTemplate = document.createElement('template');
styleTemplate.classList.add('markdown-style');
styleTemplate.content.appendChild(style);
styleTemplate.content.appendChild(link);
document.head.appendChild(styleTemplate);
```

**Variations:** None found; unique to notebook renderer context.

---

#### Pattern: Extension Manifest Plugin Declaration
**Where:** `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/package.json:71-81,81`
**What:** Manifest declarations for notebook renderer entrypoint and markdown-it plugin flag.

```json
"notebookRenderer": [
  {
    "id": "vscode.markdown-it-katex-extension",
    "displayName": "Markdown it KaTeX renderer",
    "entrypoint": {
      "extends": "vscode.markdown-it-renderer",
      "path": "./notebook-out/katex.js"
    }
  }
],
"markdown.markdownItPlugins": true,
```

**Variations:** `/home/norinlavaee/projects/vscode-atomic/extensions/ipynb/package.json:98-106` (notebook renderer without markdown-it plugin flag).

---

#### Pattern: Build Orchestration with Browser & Notebook Targets
**Where:** `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/esbuild.browser.mts:1-21` and `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-math/esbuild.notebook.mts:1-35`
**What:** Dual-target esbuild configurations with post-build asset copying (KaTeX fonts).

**Browser build** (esbuild.browser.mts):
```typescript
import * as path from 'node:path';
import { run } from '../esbuild-extension-common.mts';

const srcDir = path.join(import.meta.dirname, 'src');
const outDir = path.join(import.meta.dirname, 'dist', 'browser');

run({
	platform: 'browser',
	entryPoints: {
		'extension': path.join(srcDir, 'extension.ts'),
	},
	srcDir,
	outdir: outDir,
	additionalOptions: {
		tsconfig: path.join(import.meta.dirname, 'tsconfig.browser.json'),
	},
}, process.argv);
```

**Notebook build** (esbuild.notebook.mts):
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

**Variations:** `/home/norinlavaee/projects/vscode-atomic/extensions/markdown-language-features/` uses single build target pattern.

---

## Pattern Analysis Summary

The markdown-math extension exemplifies VS Code's plugin architecture through six distinct patterns: (1) **Extension Activation Protocol** defines the contract for contributing markdown-it plugins via `extendMarkdownIt` exports; (2) **Configuration Reactivity** watches workspace settings and triggers engine reloads when math settings change; (3) **Markdown-it Integration** uses `md.core.ruler.push()` to register stateful processing hooks that mutate options at render time; (4) **Notebook Renderer Context** demonstrates async renderer discovery and delegation within the webview-side activation boundary; (5) **Shadow DOM Stylesheet Scoping** handles CSS injection for both global and shadow-scoped rendering contexts; (6) **Dual-Target Build System** orchestrates separate esbuild configurations for browser extension and notebook renderer with asset post-processing. These patterns reflect VS Code's separation of concerns between extension host (Node.js/Electron), preview/webview host (Chromium), and notebook renderer contexts. Porting to Tauri/Rust would require translating the manifest-driven plugin discovery, configuration state management, and renderer context APIs into Tauri's webview FFI layer, while markdown-it itself would need either WASM binding or Rust-native markdown processing (e.g., pulldown-cmark with KaTeX rendering hooks).

