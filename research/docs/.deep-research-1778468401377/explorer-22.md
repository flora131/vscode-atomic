# Partition 22 of 80 — Findings

## Scope
`extensions/notebook-renderers/` (14 files, 3,546 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Notebook Renderers: File Locations for Tauri/Rust Porting Research

## Implementation
- `extensions/notebook-renderers/src/index.ts` — Main renderer activation entry point; exports `activate: ActivationFunction` that initializes renderer hooks and output MIME type dispatch (images, HTML, JavaScript, errors, streams, plaintext); interfaces with `vscode-notebook-renderer` API (the renderer-process contract model needed for Tauri webview)
- `extensions/notebook-renderers/src/rendererTypes.ts` — Type definitions for renderer interfaces: `HtmlRenderingHook`, `JavaScriptRenderingHook`, `RenderOptions`, `IRichRenderContext`; encapsulates the renderer-process <-> main-process contract surface
- `extensions/notebook-renderers/src/textHelper.ts` — Output text rendering utilities; handles line truncation, scrolling, ANSI processing, and dynamic truncation messages
- `extensions/notebook-renderers/src/htmlHelper.ts` — Trusted Types policy for HTML/script content; encapsulates security boundary for dynamic content
- `extensions/notebook-renderers/src/ansi.ts` — ANSI escape sequence parser for terminal output coloring and formatting (colors, styles, inverted text)
- `extensions/notebook-renderers/src/color.ts` — Character code constants and color utilities for terminal output
- `extensions/notebook-renderers/src/colorMap.ts` — Maps ANSI color codes to VS Code theme colors
- `extensions/notebook-renderers/src/linkify.ts` — Detects and linkifies file paths in output text
- `extensions/notebook-renderers/src/stackTraceHelper.ts` — Parses and formats error stack traces with file links

## Tests
- `extensions/notebook-renderers/src/test/notebookRenderer.test.ts` — Comprehensive renderer tests covering MIME type rendering (images, HTML, JS, errors, streams), output appending, streaming consolidation, truncation, linkification, settings changes (3,546 total project LOC, bulk of test coverage)
- `extensions/notebook-renderers/src/test/linkify.test.ts` — File path linkification tests
- `extensions/notebook-renderers/src/test/stackTraceHelper.test.ts` — Error stack trace formatting tests
- `extensions/notebook-renderers/src/test/index.ts` — Test runner/entry point

## Types / Interfaces
- `extensions/notebook-renderers/src/rendererTypes.ts` — `IDisposable`, `HtmlRenderingHook`, `JavaScriptRenderingHook`, `RenderOptions`, `IRichRenderContext`, `OutputElementOptions`, `OutputWithAppend`

## Configuration
- `extensions/notebook-renderers/package.json` — Extension manifest; declares notebookRenderer contribution with MIME type list (images, HTML, JS, errors, stdout/stderr streams, plaintext), entrypoint `./renderer-out/index.js`
- `extensions/notebook-renderers/tsconfig.json` — TypeScript compiler config; extends base, targets es2024 + DOM
- `extensions/notebook-renderers/esbuild.notebook.mts` — Build script using esbuild for bundling renderer code to `renderer-out/`
- `extensions/notebook-renderers/.vscodeignore` — Packaging exclusions
- `extensions/notebook-renderers/.npmrc` — NPM configuration
- `extensions/notebook-renderers/package-lock.json` — Dependency lock
- `extensions/notebook-renderers/package.nls.json` — Localization strings

## Documentation
- `extensions/notebook-renderers/README.md` — Brief note that this is a bundled extension providing image/plaintext/stream/error renderers; cannot be uninstalled

## Notable Clusters

**Renderer Output MIME Type Handlers** (in `index.ts`):
- `text/html` and `image/svg+xml` — rendered via `renderHTML()` with HTML hooks; security-gated by workspace trust
- `application/javascript` — rendered via `renderJavascript()` with JS hooks; security-gated
- Images (`image/png`, `image/jpeg`, `image/gif`, `image/git`) — rendered via `renderImage()` creating blob URLs
- `application/vnd.code.notebook.error` — rendered via `renderError()` with stack trace parsing and minimal error display
- Stream types (`application/vnd.code.notebook.stdout`, `application/vnd.code.notebook.stderr`, etc.) — rendered via `renderStream()` with consolidation logic for adjacent outputs
- `text/plain` and other `text/*` — rendered via `renderText()` with scrolling and truncation

**DOM Manipulation & Disposal Pattern** (throughout `index.ts`):
- Event listeners registered with disposable store cleanup
- Output lifecycle managed via disposable map keyed by output ID
- Settings change listener pattern for reactive re-rendering

**Security Boundaries** (crucial for Tauri migration):
- Trusted Types policy in `htmlHelper.ts` for HTML/script sanitization
- Workspace trust checks before rendering HTML/JavaScript
- ANSI escape sequence filtering in error traces to prevent HTML injection

This extension represents a concrete webview renderer contract implementation that any Tauri-based IDE port must replicate at the IPC boundary between main process and renderer/webview layer. The activation function signature and hook registration pattern are the primary architectural patterns that need a Tauri/Rust equivalent.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Notebook Renderer Patterns: Tauri/Rust Port Research

## Scope: `extensions/notebook-renderers/` (14 TS files, 3,546 LOC)

Research focus: Webview-like rendering contracts and IDE output handling for potential Tauri/Rust porting.

---

## Pattern 1: Activation Function & Renderer API Contract

**Where:** `extensions/notebook-renderers/src/index.ts:419-639`

**What:** The core renderer lifecycle hook that VS Code calls to initialize and register MIME-type output handlers.

```typescript
export const activate: ActivationFunction<void> = (ctx) => {
	const disposables = new Map<string, IDisposable>();
	const htmlHooks = new Set<HtmlRenderingHook>();
	const jsHooks = new Set<JavaScriptRenderingHook>();

	const latestContext = ctx as (RendererContext<void> & { readonly settings: RenderOptions; readonly onDidChangeSettings: Event<RenderOptions> });

	// ... style initialization ...

	return {
		renderOutputItem: async (outputInfo, element, signal?: AbortSignal) => {
			element.classList.add('remove-padding');
			switch (outputInfo.mime) {
				case 'text/html':
				case 'image/svg+xml': {
					if (!ctx.workspace.isTrusted) {
						return;
					}
					await renderHTML(outputInfo, element, signal!, htmlHooks);
					break;
				}
				case 'application/javascript': {
					if (!ctx.workspace.isTrusted) {
						return;
					}
					renderJavascript(outputInfo, element, signal!, jsHooks);
					break;
				}
				// ... image, error, stream cases ...
				default:
					if (outputInfo.mime.indexOf('text/') > -1) {
						disposables.get(outputInfo.id)?.dispose();
						const disposable = renderText(outputInfo, element, latestContext);
						disposables.set(outputInfo.id, disposable);
					}
					break;
			}
		},
		disposeOutputItem: (id: string | undefined) => {
			if (id) {
				disposables.get(id)?.dispose();
			} else {
				disposables.forEach(d => d.dispose());
			}
		},
		experimental_registerHtmlRenderingHook: (hook: HtmlRenderingHook): IDisposable => {
			htmlHooks.add(hook);
			return { dispose: () => htmlHooks.delete(hook); };
		},
		experimental_registerJavaScriptRenderingHook: (hook: JavaScriptRenderingHook): IDisposable => {
			jsHooks.add(hook);
			return { dispose: () => jsHooks.delete(hook); };
		}
	};
};
```

**Variations / call-sites:**
- Import: `import type { ActivationFunction, OutputItem, RendererContext } from 'vscode-notebook-renderer'` (line 6)
- Test activation: `extensions/notebook-renderers/src/test/notebookRenderer.test.ts:145`

---

## Pattern 2: Renderer Type Contracts & Hooks

**Where:** `extensions/notebook-renderers/src/rendererTypes.ts:1-52`

**What:** Interface definitions for the webview-renderer protocol, including hook chains for extensibility.

```typescript
export interface IDisposable {
	dispose(): void;
}

export interface HtmlRenderingHook {
	/**
	 * Invoked after the output item has been rendered but before it has been appended to the document.
	 *
	 * @return A new `HTMLElement` or `undefined` to continue using the provided element.
	 */
	postRender(outputItem: OutputItem, element: HTMLElement, signal: AbortSignal): HTMLElement | undefined | Promise<HTMLElement | undefined>;
}

export interface JavaScriptRenderingHook {
	/**
	 * Invoked before the script is evaluated.
	 *
	 * @return A new string of JavaScript or `undefined` to continue using the provided string.
	 */
	preEvaluate(outputItem: OutputItem, element: HTMLElement, script: string, signal: AbortSignal): string | undefined | Promise<string | undefined>;
}

export interface RenderOptions {
	readonly lineLimit: number;
	readonly outputScrolling: boolean;
	readonly outputWordWrap: boolean;
	readonly linkifyFilePaths: boolean;
	readonly minimalError: boolean;
}

export type IRichRenderContext = RendererContext<void> & { readonly settings: RenderOptions; readonly onDidChangeSettings: Event<RenderOptions> };
```

---

## Pattern 3: Trust & Content Security Boundary

**Where:** `extensions/notebook-renderers/src/index.ts:543-560`

**What:** Workspace trust gates for HTML/JavaScript execution; demonstrates IDE security model.

```typescript
case 'text/html':
case 'image/svg+xml': {
	if (!ctx.workspace.isTrusted) {
		return;
	}

	await renderHTML(outputInfo, element, signal!, htmlHooks);
	break;
}
case 'application/javascript': {
	if (!ctx.workspace.isTrusted) {
		return;
	}

	renderJavascript(outputInfo, element, signal!, jsHooks);
	break;
}
```

---

## Pattern 4: Trusted Types Policy (Content Security)

**Where:** `extensions/notebook-renderers/src/htmlHelper.ts:1-11`

**What:** Sandboxed HTML/script creation through TrustedTypes API for security.

```typescript
export const ttPolicy = (typeof window !== 'undefined') ?
	(window as Window & { trustedTypes?: any }).trustedTypes?.createPolicy('notebookRenderer', {
		createHTML: (value: string) => value,
		createScript: (value: string) => value,
	}) : undefined;
```

**Usage:** `extensions/notebook-renderers/src/index.ts:63,111,141` for HTML injection and script evaluation.

---

## Pattern 5: Output Rendering Dispatch by MIME Type

**Where:** `extensions/notebook-renderers/src/index.ts:541-609`

**What:** Multi-dispatch renderer that routes output items based on MIME type with lifecycle management.

```typescript
renderOutputItem: async (outputInfo, element, signal?: AbortSignal) => {
	element.classList.add('remove-padding');
	switch (outputInfo.mime) {
		case 'image/gif':
		case 'image/png':
		case 'image/jpeg':
		case 'image/git':
			{
				disposables.get(outputInfo.id)?.dispose();
				const disposable = renderImage(outputInfo, element);
				disposables.set(outputInfo.id, disposable);
			}
			break;
		case 'application/vnd.code.notebook.error':
			{
				disposables.get(outputInfo.id)?.dispose();
				const disposable = renderError(outputInfo, element, latestContext, ctx.workspace.isTrusted);
				disposables.set(outputInfo.id, disposable);
			}
			break;
		case 'application/vnd.code.notebook.stdout':
		case 'application/x.notebook.stdout':
		case 'application/x.notebook.stream':
			{
				disposables.get(outputInfo.id)?.dispose();
				const disposable = renderStream(outputInfo, element, false, latestContext);
				disposables.set(outputInfo.id, disposable);
			}
			break;
		case 'text/plain':
			{
				disposables.get(outputInfo.id)?.dispose();
				const disposable = renderText(outputInfo, element, latestContext);
				disposables.set(outputInfo.id, disposable);
			}
			break;
	}
	if (element.querySelector('div')) {
		element.querySelector('div')!.tabIndex = 0;
	}
},
disposeOutputItem: (id: string | undefined) => {
	if (id) {
		disposables.get(id)?.dispose();
	} else {
		disposables.forEach(d => d.dispose());
	}
},
```

**Variations:**
- Errors: `renderError()` at line 167, stack trace parsing via `formatStackTrace()` at line 10
- Streams: `renderStream()` at line 350, supports append-only stdout/stderr
- Text: `renderText()` at line 398
- Images: `renderImage()` at line 18 (blob URL lifecycle)
- HTML: `renderHTML()` at line 107 (post-render hooks)
- JavaScript: `renderJavascript()` at line 126 (pre-eval hooks)

---

## Pattern 6: Settings Change Events & Disposable Management

**Where:** `extensions/notebook-renderers/src/index.ts:201-213 & 387-390`

**What:** Dynamic settings updates with listener cleanup; Event-based reactive pattern.

```typescript
outputElement.classList.toggle('word-wrap', ctx.settings.outputWordWrap);
disposableStore.push(ctx.onDidChangeSettings(e => {
	outputElement.classList.toggle('word-wrap', e.outputWordWrap);
}));
```

**Test pattern:** `extensions/notebook-renderers/src/test/notebookRenderer.test.ts:461-474`

```typescript
test(`Rendered output will wrap on settings change event`, async () => {
	const context = createContext({ outputWordWrap: false, outputScrolling: true });
	const renderer = await activate(context);
	assert.ok(renderer, 'Renderer not created');

	const outputElement = new OutputHtml().getFirstOuputElement();
	const outputItem = createOutputItem('content', stdoutMimeType);
	await renderer!.renderOutputItem(outputItem, outputElement);
	fireSettingsChange({ outputWordWrap: true, outputScrolling: true });

	const inserted = outputElement.firstChild as HTMLElement;
	assert.ok(outputElement.classList.contains('word-wrap') && inserted.classList.contains('scrollable'),
		`output content classList should contain word-wrap and scrollable ${inserted.classList}`);
});
```

---

## Pattern 7: Streaming Output Consolidation & Append

**Where:** `extensions/notebook-renderers/src/textHelper.ts:137-176`

**What:** Efficient appending to scrollable outputs with line-limit enforcement.

```typescript
export function createOutputContent(id: string, outputText: string, options: OutputElementOptions): HTMLElement {
	const { linesLimit, error, scrollable, trustHtml, linkifyFilePaths } = options;
	const linkOptions: LinkOptions = { linkifyFilePaths, trustHtml };
	const buffer = outputText.split(/\r\n|\r|\n/g);
	outputLengths[id] = outputLengths[id] = Math.min(buffer.length, softScrollableLineLimit);

	let outputElement: HTMLElement;
	if (scrollable) {
		outputElement = scrollableArrayOfString(id, buffer, linkOptions);
	} else {
		outputElement = truncatedArrayOfString(id, buffer, linesLimit, linkOptions);
	}

	outputElement.setAttribute('output-item-id', id);
	if (error) {
		outputElement.classList.add('error');
	}

	return outputElement;
}

export function appendOutput(outputInfo: OutputWithAppend, existingContent: HTMLElement, options: OutputElementOptions) {
	const appendedText = outputInfo.appendedText?.();
	const linkOptions = { linkifyFilePaths: options.linkifyFilePaths, trustHtml: options.trustHtml };
	// appending output only supported for scrollable ouputs currently
	if (appendedText && options.scrollable) {
		if (appendScrollableOutput(existingContent, outputInfo.id, appendedText, linkOptions)) {
			return;
		}
	}

	const newContent = createOutputContent(outputInfo.id, outputInfo.text(), options);
	existingContent.replaceWith(newContent);
	while (newContent.nextSibling) {
		// clear out any stale content if we had previously combined streaming outputs into this one
		newContent.nextSibling.remove();
	}
}
```

**Limits:** `softScrollableLineLimit = 5000`, `hardScrollableLineLimit = 8000`

**Test:** `extensions/notebook-renderers/src/test/notebookRenderer.test.ts:199-262`

---

## Pattern 8: ANSI Color & Markup Processing

**Where:** `extensions/notebook-renderers/src/ansi.ts:1-100`

**What:** Character-by-character ANSI escape sequence parser for terminal output styling.

```typescript
export function handleANSIOutput(text: string, linkOptions: LinkOptions): HTMLSpanElement {
	const root: HTMLSpanElement = document.createElement('span');
	const textLength: number = text.length;

	let styleNames: string[] = [];
	let customFgColor: RGBA | string | undefined;
	let customBgColor: RGBA | string | undefined;
	let customUnderlineColor: RGBA | string | undefined;
	let colorsInverted: boolean = false;
	let currentPos: number = 0;
	let buffer: string = '';

	while (currentPos < textLength) {
		let sequenceFound: boolean = false;

		// Potentially an ANSI escape sequence.
		if (text.charCodeAt(currentPos) === 27 && text.charAt(currentPos + 1) === '[') {
			const startPos: number = currentPos;
			currentPos += 2; // Ignore 'Esc[' as it's in every sequence.

			let ansiSequence: string = '';

			while (currentPos < textLength) {
				const char: string = text.charAt(currentPos);
				ansiSequence += char;
				currentPos++;

				// Look for a known sequence terminating character.
				if (char.match(/^[ABCDHIJKfhmpsu]$/)) {
					sequenceFound = true;
					break;
				}
			}

			if (sequenceFound) {
				// Flush buffer with previous styles.
				appendStylizedStringToContainer(root, buffer, linkOptions, styleNames, customFgColor, customBgColor, customUnderlineColor);
				buffer = '';

				// Regex for SGR (Select Graphic Rendition) codes
				if (ansiSequence.match(/^(?:[34][0-8]|9[0-7]|10[0-7]|[0-9]|2[1-5,7-9]|[34]9|5[8,9]|1[0-9])(?:;[349][0-7]|10[0-7]|[013]|[245]|[34]9)?(?:;[012]?[0-9]?[0-9])*;?m$/)) {
					const styleCodes: number[] = ansiSequence.slice(0, -1).split(';').filter(elem => elem !== '').map(elem => parseInt(elem, 10));
					
					if (styleCodes[0] === 38 || styleCodes[0] === 48 || styleCodes[0] === 58) {
						// Advanced color code handling (8-bit, 24-bit RGB)
					} else {
						setBasicFormatters(styleCodes);
					}
				}
			}
		}

		if (sequenceFound === false) {
			buffer += text.charAt(currentPos);
			currentPos++;
		}
	}

	// Flush remaining text buffer if not empty.
	if (buffer) {
		appendStylizedStringToContainer(root, buffer, linkOptions, styleNames, customFgColor, customBgColor, customUnderlineColor);
	}
	// ...
}
```

---

## Pattern 9: Stack Trace Linkification & IPython Detection

**Where:** `extensions/notebook-renderers/src/stackTraceHelper.ts:1-107`

**What:** Regex-based stack trace parsing with VS Code command link injection for Jupyter errors.

```typescript
export function formatStackTrace(stack: string, trustHtml: boolean): { formattedStack: string; errorLocation?: string } {
	let cleaned: string;
	
	// Remove ANSI color codes
	cleaned = stack.replace(/\[4\dm/g, '');
	cleaned = cleaned.replace(/(?<=\[[\d;]*?);4\d(?=m)/g, '');
	cleaned = cleaned.replace(/\[38;.*?\d+m/g, '[39m');
	cleaned = cleaned.replace(/(;32m[ ->]*?)(\d+)(.*)\n/g, (_s, prefix, num, suffix) => {
		suffix = suffix.replace(/\[3\d+m/g, '[39m');
		return `${prefix}${num}${suffix}\n`;
	});

	if (isIpythonStackTrace(cleaned) && trustHtml) {
		return linkifyStack(cleaned);
	}

	return { formattedStack: cleaned };
}

const formatSequence = /\[.+?m/g;
const fileRegex = /File\s+(?:\[.+?m)?(.+):(\d+)/;
const lineNumberRegex = /(-+>(?:\[[\d;]*m|\s)*)(\d+)(.*)/;
const cellRegex = /^(?<prefix>(?:\[[\d;]*m|\s)*Cell(?:\[[\d;]*m|\s)*In(?:\[[\d;]*m|\s)*\[(?<executionCount>\d+)\](?:\[[\d;]*m|\s|,)+)(?<lineLabel>line (?<lineNumber>\d+))[^\n]*$/m;
const inputRegex = /(?<prefix>Input\s+?(?:\[.+?m)(?<cellLabel>In\s*\[(?<executionCount>\d+)\]))(?<postfix>.*)/;

function linkifyStack(stack: string): { formattedStack: string; errorLocation?: string } {
	const lines = stack.split('\n');
	let fileOrCell: location | undefined;
	let locationLink = '';

	for (const i in lines) {
		const original = lines[i];
		if (fileRegex.test(original)) {
			const fileMatch = lines[i].match(fileRegex);
			fileOrCell = { kind: 'file', path: stripFormatting(fileMatch![1]) };
			continue;
		} else if (cellRegex.test(original)) {
			fileOrCell = { kind: 'cell', path: stripFormatting(original.replace(cellRegex, 'vscode-notebook-cell:?execution_count=$<executionCount>')) };
			const link = original.replace(cellRegex, `<a href='${fileOrCell.path}&line=$<lineNumber>'>line $<lineNumber></a>`);
			lines[i] = original.replace(cellRegex, `$<prefix>${link}`);
			locationLink = locationLink || link;
			continue;
		} else if (lineNumberRegex.test(original)) {
			lines[i] = original.replace(lineNumberRegex, (_s, prefix, num, suffix) => {
				return fileOrCell?.kind === 'file' ?
					`${prefix}<a href='${fileOrCell?.path}:${num}'>${num}</a>${suffix}` :
					`${prefix}<a href='${fileOrCell?.path}&line=${num}'>${num}</a>${suffix}`;
			});
			continue;
		}
	}

	const errorLocation = locationLink;
	return { formattedStack: lines.join('\n'), errorLocation };
}
```

---

## Pattern 10: File Path & Web Link Detection

**Where:** `extensions/notebook-renderers/src/linkify.ts:1-80`

**What:** Multi-regex link detection (web URLs, file paths) with OS-aware path normalization.

```typescript
const CONTROL_CODES = '\\u0000-\\u0020\\u007f-\\u009f';
const WEB_LINK_REGEX = new RegExp('(?:[a-zA-Z][a-zA-Z0-9+.-]{2,}:\\/\\/|data:|www\\.)[^\\s' + CONTROL_CODES + '"]{2,}[^\\s' + CONTROL_CODES + '"\')}\\],:;.!?]', 'ug');

const WIN_ABSOLUTE_PATH = /(?<=^|\s)(?:[a-zA-Z]:(?:(?:\\|\/)[\w\.-]*)+)/;
const WIN_RELATIVE_PATH = /(?<=^|\s)(?:(?:\~|\.)(?:(?:\\|\/)[\w\.-]*)+)/;
const WIN_PATH = new RegExp(`(${WIN_ABSOLUTE_PATH.source}|${WIN_RELATIVE_PATH.source})`);
const POSIX_PATH = /(?<=^|\s)((?:\~|\.)?(?:\/[\w\.-]*)+)/;
const LINE_COLUMN = /(?:\:([\d]+))?(?:\:([\d]+))?/;
const isWindows = (typeof navigator !== 'undefined') ? navigator.userAgent && navigator.userAgent.indexOf('Windows') >= 0 : false;
const PATH_LINK_REGEX = new RegExp(`${isWindows ? WIN_PATH.source : POSIX_PATH.source}${LINE_COLUMN.source}`, 'g');
const HTML_LINK_REGEX = /<a\s+(?:[^>]*?\s+)?href=(["'])(.*?)\1[^>]*?>.*?<\/a>/gi;

type LinkKind = 'web' | 'path' | 'html' | 'text';
type LinkPart = {
	kind: LinkKind;
	value: string;
	captures: string[];
};

export type LinkOptions = {
	trustHtml?: boolean;
	linkifyFilePaths: boolean;
};

export class LinkDetector {
	// used by unit tests
	static injectedHtmlCreator: (value: string) => string;

	private shouldGenerateHtml(trustHtml: boolean) {
		return trustHtml && (!!LinkDetector.injectedHtmlCreator || !!ttPolicy);
	}

	private createHtml(value: string) {
		if (LinkDetector.injectedHtmlCreator) {
			return LinkDetector.injectedHtmlCreator(value);
		}
		else {
			return ttPolicy?.createHTML(value).toString();
		}
	}

	linkify(text: string, options: LinkOptions, splitLines?: boolean): HTMLElement {
		if (splitLines) {
			const lines = text.split('\n');
			for (let i = 0; i < lines.length - 1; i++) {
				lines[i] = lines[i] + '\n';
			}
			if (!lines[lines.length - 1]) {
				lines.pop();
			}
			const elements = lines.map(line => this.linkify(line, options, false));
			if (elements.length === 1) {
				return elements[0];
			}
			const container = document.createElement('span');
			elements.forEach(e => container.appendChild(e));
			return container;
		}

		const container = document.createElement('span');
		for (const part of this.detectLinks(text, !!options.trustHtml, options.linkifyFilePaths)) {
			// ... per-link rendering
		}
	}
}
```

**Test:** `extensions/notebook-renderers/src/test/notebookRenderer.test.ts:280-308`

---

## Pattern 11: Extension Manifest & Renderer Registration

**Where:** `extensions/notebook-renderers/package.json:18-42`

**What:** VS Code extension contribution point for notebook renderers with MIME type mapping.

```json
{
	"contributes": {
		"notebookRenderer": [
			{
				"id": "vscode.builtin-renderer",
				"entrypoint": "./renderer-out/index.js",
				"displayName": "VS Code Builtin Notebook Output Renderer",
				"requiresMessaging": "never",
				"mimeTypes": [
					"image/gif",
					"image/png",
					"image/jpeg",
					"image/git",
					"image/svg+xml",
					"text/html",
					"application/javascript",
					"application/vnd.code.notebook.error",
					"application/vnd.code.notebook.stdout",
					"application/x.notebook.stdout",
					"application/x.notebook.stream",
					"application/vnd.code.notebook.stderr",
					"application/x.notebook.stderr",
					"text/plain"
				]
			}
		]
	}
}
```

---

## Pattern 12: Build & Bundling Strategy

**Where:** `extensions/notebook-renderers/esbuild.notebook.mts:1-18`

**What:** ESBuild configuration for webview bundling without postprocessing.

```typescript
import path from 'path';
import { run } from '../esbuild-webview-common.mts';

const srcDir = path.join(import.meta.dirname, 'src');
const outDir = path.join(import.meta.dirname, 'renderer-out');

run({
	entryPoints: [
		path.join(srcDir, 'index.ts'),
	],
	srcDir,
	outdir: outDir,
}, process.argv);
```

**Package.json scripts:**
- `compile`: `npx gulp compile-extension:notebook-renderers && npm run build-notebook`
- `build-notebook`: `node ./esbuild.notebook.mts`

---

## Summary: Core Patterns for Tauri/Rust Port

The notebook-renderers scope reveals **critical webview rendering contracts**:

1. **Activation hook**: Single entry point returning renderer object with MIME dispatch
2. **Disposables pattern**: Explicit resource cleanup (no garbage collection assumption)
3. **Trust boundary**: Workspace trust gates HTML/JS execution
4. **Content security**: TrustedTypes policy for DOM injection
5. **MIME dispatch**: Switch-based output routing (images, text, error, streams)
6. **Settings reactivity**: Event-driven settings updates without re-render
7. **Stream consolidation**: Efficient output appending with size limits
8. **Terminal rendering**: ANSI escape sequence parsing (character-by-character)
9. **Stack trace linking**: Regex-based error location extraction for Jupyter
10. **Link detection**: OS-aware path and URL linkification with separate code paths
11. **Extension manifest**: Static MIME type registration without dynamic discovery
12. **Webview bundling**: ESBuild-based isolation from core extension code

### Key Takeaways for Port:
- **No bidirectional messaging** (`requiresMessaging: "never"`) — renderer is stateless
- **Trust model** must be preserved; workspace trust gates sensitive content
- **Output item protocol** (`.mime`, `.id`, `.metadata`, `.text()`, `.data()`) is the core contract
- **Disposable pattern** can map to Rust's Drop trait
- **ANSI parsing** and **stack trace linkification** are non-trivial; likely need Rust regex crates
- **Settings events** suggest a pub-sub pattern that could use Rust async channels

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
