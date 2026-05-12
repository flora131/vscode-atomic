# Pattern Finder Report: Partition 53 — markdown-math Extension

## Summary
The markdown-math extension (177 LOC across 5 files) demonstrates two VSCode extension API patterns: markdown plugin extension and notebook renderer extension. Both are minimal and focused on a single feature.

---

## API Patterns Found

#### Pattern: Markdown Plugin Extension Registration
**Where:** `extensions/markdown-math/src/extension.ts:11-46`
**What:** VSCode markdown extension API using `extendMarkdownIt` to inject a plugin into the markdown processor pipeline.

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

**Variations:**
- Configuration-driven activation: checks `markdown.math.enabled` setting
- Dynamic macro loading: reads `markdown.math.macros` from workspace config
- Configuration change listener: reloads plugin on setting changes via `onDidChangeConfiguration`
- Markdown-it rule injection: uses `md.core.ruler.push()` for custom processing

---

#### Pattern: Notebook Renderer Extension
**Where:** `extensions/markdown-math/notebook/katex.ts:10-58`
**What:** VSCode notebook renderer API using `RendererContext` to integrate with notebook rendering pipeline.

```typescript
export async function activate(ctx: RendererContext<void>) {
	const markdownItRenderer = (await ctx.getRenderer('vscode.markdown-it-renderer')) as undefined | any;
	if (!markdownItRenderer) {
		throw new Error(`Could not load 'vscode.markdown-it-renderer'`);
	}

	// Add katex styles to be copied to shadow dom
	const link = document.createElement('link');
	link.rel = 'stylesheet';
	link.classList.add('markdown-style');
	link.href = styleHref;

	// Add same katex style to root document.
	// This is needed for the font to be loaded correctly inside the shadow dom.
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

	// Put Everything into a template
	const styleTemplate = document.createElement('template');
	styleTemplate.classList.add('markdown-style');
	styleTemplate.content.appendChild(style);
	styleTemplate.content.appendChild(link);
	document.head.appendChild(styleTemplate);

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

**Variations:**
- Async renderer initialization with dependency resolution
- Fetches other renderers via `ctx.getRenderer()` for composition
- Explicit error handling for missing dependencies
- Shadow DOM style injection with document-level fallback
- Uses VSCode theme variables (`--vscode-editorError-foreground`)

---

#### Pattern: Extension Manifest Configuration
**Where:** `extensions/markdown-math/package.json:26-107`
**What:** VSCode `package.json` contributions declaring markdown plugin, grammar injections, notebook renderer, and settings schema.

```json
"contributes": {
	"languages": [
		{
			"id": "markdown-math",
			"aliases": []
		}
	],
	"grammars": [
		{
			"language": "markdown-math",
			"scopeName": "text.html.markdown.math",
			"path": "./syntaxes/md-math.tmLanguage.json"
		},
		{
			"scopeName": "markdown.math.block",
			"path": "./syntaxes/md-math-block.tmLanguage.json",
			"injectTo": ["text.html.markdown"],
			"embeddedLanguages": {
				"meta.embedded.math.markdown": "latex"
			}
		}
	],
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
					"default": true,
					"description": "%config.markdown.math.enabled%"
				},
				"markdown.math.macros": {
					"type": "object",
					"additionalProperties": {"type": "string"},
					"default": {},
					"description": "%config.markdown.math.macros%",
					"scope": "resource"
				}
			}
		}
	]
}
```

**Variations:**
- Grammar injection into markdown via `injectTo`
- Embedded language scoping for syntax highlighting (latex within markdown)
- Notebook renderer registration with entrypoint extension chaining
- Workspace and resource-scoped configuration options
- Declarative markdown plugin enablement via `markdown.markdownItPlugins`

---

## Patterns Demonstrated

These three patterns show the core VSCode extension API surface for markdown:
1. **Activation & Configuration**: Extensions activate with `ExtensionContext`, read workspace config, and listen for changes
2. **Markdown Pipeline**: Plugins integrate via `extendMarkdownIt()` return pattern, modifying the markdown-it processor
3. **Notebook Integration**: Separate renderer activation, dependency resolution, and DOM manipulation for rendering
4. **Manifest Declaration**: Grammar injection, language registration, and configuration schema defined declaratively

This extension provides minimal but complete examples of VSCode's markdown extension API, grammar injection system, and notebook renderer protocol. The patterns are straightforward implementations without complex error handling or state management.
