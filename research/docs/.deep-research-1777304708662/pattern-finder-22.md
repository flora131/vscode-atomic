# Notebook Renderers: Webview-Bound Output Rendering Patterns

## Research Context
VS Code's `extensions/notebook-renderers/` provides built-in renderers for notebook cell outputs (markdown, error, text, images, HTML, JavaScript). These are webview-bound components that run in the notebook editor's DOM context. Porting these to Tauri/Rust requires understanding the renderer architecture, MIME type handling, DOM manipulation patterns, and event/settings integration.

---

## Pattern 1: Extension Activation & Renderer Registration

**Where:** `extensions/notebook-renderers/src/index.ts:419-639`

**What:** Core activation function exports a renderer API with output item handlers and hook registration methods.

```typescript
export const activate: ActivationFunction<void> = (ctx) => {
	const disposables = new Map<string, IDisposable>();
	const htmlHooks = new Set<HtmlRenderingHook>();
	const jsHooks = new Set<JavaScriptRenderingHook>();

	const latestContext = ctx as (RendererContext<void> & { readonly settings: RenderOptions; readonly onDidChangeSettings: Event<RenderOptions> });

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
				case 'image/gif':
				case 'image/png':
				case 'image/jpeg': {
					disposables.get(outputInfo.id)?.dispose();
					const disposable = renderImage(outputInfo, element);
					disposables.set(outputInfo.id, disposable);
					break;
				}
				case 'application/vnd.code.notebook.error': {
					disposables.get(outputInfo.id)?.dispose();
					const disposable = renderError(outputInfo, element, latestContext, ctx.workspace.isTrusted);
					disposables.set(outputInfo.id, disposable);
					break;
				}
				default: {
					if (outputInfo.mime.indexOf('text/') > -1) {
						disposables.get(outputInfo.id)?.dispose();
						const disposable = renderText(outputInfo, element, latestContext);
						disposables.set(outputInfo.id, disposable);
					}
				}
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
		experimental_registerHtmlRenderingHook: (hook: HtmlRenderingHook): IDisposable => {
			htmlHooks.add(hook);
			return {
				dispose: () => {
					htmlHooks.delete(hook);
				}
			};
		},
		experimental_registerJavaScriptRenderingHook: (hook: JavaScriptRenderingHook): IDisposable => {
			jsHooks.add(hook);
			return {
				dispose: () => {
					jsHooks.delete(hook);
				}
			};
		}
	};
};
```

**Key aspects:**
- MIME-type-based dispatch switch controls which renderer handles each output type
- Disposable pattern tracks resources per output ID for cleanup
- Extensibility via hook registration (pre/post-render transformations)
- Workspace trust check gates HTML/JS rendering
- Settings context propagated via `ctx` for dynamic behavior

---

## Pattern 2: Trusted HTML & Script Rendering with Content Security Policy

**Where:** `extensions/notebook-renderers/src/htmlHelper.ts:1-11`

**What:** Trusted Types policy wraps HTML/script content to prevent XSS while allowing safe content.

```typescript
export const ttPolicy = (typeof window !== 'undefined') ?
	(window as Window & { trustedTypes?: any }).trustedTypes?.createPolicy('notebookRenderer', {
		createHTML: (value: string) => value,
		createScript: (value: string) => value,
	}) : undefined;
```

And in `index.ts:107-124`:

```typescript
async function renderHTML(outputInfo: OutputItem, container: HTMLElement, signal: AbortSignal, hooks: Iterable<HtmlRenderingHook>): Promise<void> {
	clearContainer(container);
	let element: HTMLElement = document.createElement('div');
	const htmlContent = outputInfo.text();
	const trustedHtml = ttPolicy?.createHTML(htmlContent) ?? htmlContent;
	element.innerHTML = trustedHtml as string;
	fixUpSvgElement(outputInfo, element);

	for (const hook of hooks) {
		element = (await hook.postRender(outputInfo, element, signal)) ?? element;
		if (signal.aborted) {
			return;
		}
	}

	container.appendChild(element);
	domEval(element);
}
```

**Key aspects:**
- Trusted Types (CSP Level 3) gates innerHTML assignment
- Fallback to raw content if policy unavailable
- Hook pipeline allows post-render transformations with cancellation support
- Script re-evaluation via `domEval()` after DOM insertion

---

## Pattern 3: ANSI Code Processing & Color Rendering

**Where:** `extensions/notebook-renderers/src/ansi.ts:11-452`

**What:** Stateful parser processes ANSI escape sequences and maintains style state during terminal output rendering.

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

		// Detect ESC[ (ANSI sequence start)
		if (text.charCodeAt(currentPos) === 27 && text.charAt(currentPos + 1) === '[') {
			const startPos: number = currentPos;
			currentPos += 2;
			let ansiSequence: string = '';

			while (currentPos < textLength) {
				const char: string = text.charAt(currentPos);
				ansiSequence += char;
				currentPos++;

				if (char.match(/^[ABCDHIJKfhmpsu]$/)) {
					sequenceFound = true;
					break;
				}
			}

			if (sequenceFound) {
				appendStylizedStringToContainer(root, buffer, linkOptions, styleNames, customFgColor, customBgColor, customUnderlineColor);
				buffer = '';

				if (ansiSequence.match(/^(?:[34][0-8]|9[0-7]|10[0-7]|[0-9]|2[1-5,7-9]|[34]9|5[8,9]|1[0-9])(?:;[349][0-7]|10[0-7]|[013]|[245]|[34]9)?(?:;[012]?[0-9]?[0-9])*;?m$/)) {

					const styleCodes: number[] = ansiSequence.slice(0, -1).split(';').filter(elem => elem !== '').map(elem => parseInt(elem, 10));

					if (styleCodes[0] === 38 || styleCodes[0] === 48 || styleCodes[0] === 58) {
						const colorType = (styleCodes[0] === 38) ? 'foreground' : ((styleCodes[0] === 48) ? 'background' : 'underline');

						if (styleCodes[1] === 5) {
							set8BitColor(styleCodes, colorType);
						} else if (styleCodes[1] === 2) {
							set24BitColor(styleCodes, colorType);
						}
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

	if (buffer) {
		appendStylizedStringToContainer(root, buffer, linkOptions, styleNames, customFgColor, customBgColor, customUnderlineColor);
	}

	return root;
}
```

**Key aspects:**
- State machine: tracks position, accumulated style names, and colors
- Supports 8-bit and 24-bit ANSI color codes (0-255 and RGB)
- Style accumulation: bold, italic, underline, strikethrough, etc. (case codes 0-9, 21-53, 73-75)
- Color inversion (code 7) swaps foreground/background
- Helper functions apply styles incrementally (changeColor, setBasicFormatters, set8BitColor, set24BitColor)

---

## Pattern 4: Text Output with Scrolling & Truncation

**Where:** `extensions/notebook-renderers/src/textHelper.ts:137-175`

**What:** Adaptive output rendering with soft/hard scrolling limits and line truncation.

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

Truncation thresholds:
```typescript
const softScrollableLineLimit = 5000;
const hardScrollableLineLimit = 8000;
```

**Key aspects:**
- Dual-mode: scrollable vs. truncated (with "View More" link)
- Soft limit (5000 lines): scrollable outputs show latest lines only
- Hard limit (8000 lines): streaming outputs stop accepting appends
- Incremental append for streaming outputs (stdout/stderr)
- Command links for "View as scrollable element" and "open in text editor"

---

## Pattern 5: Error Rendering with Stack Trace Processing

**Where:** `extensions/notebook-renderers/src/index.ts:167-224`

**What:** Error output rendering with optional minimal UI and stack trace linkification.

```typescript
function renderError(
	outputInfo: OutputItem,
	outputElement: HTMLElement,
	ctx: IRichRenderContext,
	trustHtml: boolean
): IDisposable {
	const disposableStore = createDisposableStore();

	clearContainer(outputElement);

	type ErrorLike = Partial<Error>;

	let err: ErrorLike;
	try {
		err = <ErrorLike>JSON.parse(outputInfo.text());
	} catch (e) {
		console.log(e);
		return disposableStore;
	}

	const headerMessage = err.name && err.message ? `${err.name}: ${err.message}` : err.name || err.message;

	if (err.stack) {
		const minimalError = ctx.settings.minimalError && !!headerMessage?.length;
		outputElement.classList.add('traceback');

		const { formattedStack, errorLocation } = formatStackTrace(err.stack, trustHtml);

		const outputScrolling = !minimalError && scrollingEnabled(outputInfo, ctx.settings);
		const lineLimit = minimalError ? 1000 : ctx.settings.lineLimit;
		const outputOptions = { linesLimit: lineLimit, scrollable: outputScrolling, trustHtml, linkifyFilePaths: false };

		const content = createOutputContent(outputInfo.id, formattedStack, outputOptions);
		const stackTraceElement = document.createElement('div');
		stackTraceElement.appendChild(content);
		outputElement.classList.toggle('word-wrap', ctx.settings.outputWordWrap);
		disposableStore.push(ctx.onDidChangeSettings(e => {
			outputElement.classList.toggle('word-wrap', e.outputWordWrap);
		}));

		if (minimalError) {
			createMinimalError(errorLocation, headerMessage, stackTraceElement, outputElement);
		} else {
			stackTraceElement.classList.toggle('scrollable', outputScrolling);
			outputElement.appendChild(stackTraceElement);
			initializeScroll(stackTraceElement, disposableStore);
		}
	} else {
		const header = document.createElement('div');
		if (headerMessage) {
			header.innerText = headerMessage;
			outputElement.appendChild(header);
		}
	}

	outputElement.classList.add('error');
	return disposableStore;
}
```

**Key aspects:**
- JSON parse with graceful fallback
- Dual-mode minimal vs. full error UI
- Stack trace linkification via `formatStackTrace()` (converts file/line references to links)
- Settings-driven behavior (minimal error, line limit, word wrap)
- Dynamic style updates on settings change

---

## Pattern 6: Stream Output Append & Aggregation

**Where:** `extensions/notebook-renderers/src/index.ts:350-396`

**What:** Handles stdout/stderr streaming with batching and scroll position preservation.

```typescript
function renderStream(outputInfo: OutputWithAppend, outputElement: HTMLElement, error: boolean, ctx: IRichRenderContext): IDisposable {
	const disposableStore = createDisposableStore();
	const outputScrolling = scrollingEnabled(outputInfo, ctx.settings);
	const outputOptions = { linesLimit: ctx.settings.lineLimit, scrollable: outputScrolling, trustHtml: false, error, linkifyFilePaths: ctx.settings.linkifyFilePaths };

	outputElement.classList.add('output-stream');

	const scrollTop = outputScrolling ? findScrolledHeight(outputElement) : undefined;

	const previousOutputParent = getPreviousMatchingContentGroup(outputElement);
	// If the previous output item for the same cell was also a stream, append this output to the previous
	if (previousOutputParent) {
		const existingContent = previousOutputParent.querySelector(`[output-item-id="${outputInfo.id}"]`) as HTMLElement | null;
		if (existingContent) {
			appendOutput(outputInfo, existingContent, outputOptions);
		} else {
			const newContent = createOutputContent(outputInfo.id, outputInfo.text(), outputOptions);
			previousOutputParent.appendChild(newContent);
		}
		previousOutputParent.classList.toggle('scrollbar-visible', previousOutputParent.scrollHeight > previousOutputParent.clientHeight);
		previousOutputParent.scrollTop = scrollTop !== undefined ? scrollTop : previousOutputParent.scrollHeight;
	} else {
		const existingContent = outputElement.querySelector(`[output-item-id="${outputInfo.id}"]`) as HTMLElement | null;
		let contentParent = existingContent?.parentElement;
		if (existingContent && contentParent) {
			appendOutput(outputInfo, existingContent, outputOptions);
		} else {
			const newContent = createOutputContent(outputInfo.id, outputInfo.text(), outputOptions);
			contentParent = document.createElement('div');
			contentParent.appendChild(newContent);
			while (outputElement.firstChild) {
				outputElement.firstChild.remove();
			}
			outputElement.appendChild(contentParent);
		}

		contentParent.classList.toggle('scrollable', outputScrolling);
		outputElement.classList.toggle('word-wrap', ctx.settings.outputWordWrap);
		disposableStore.push(ctx.onDidChangeSettings(e => {
			outputElement.classList.toggle('word-wrap', e.outputWordWrap);
		}));

		initializeScroll(contentParent, disposableStore, scrollTop);
	}

	return disposableStore;
}
```

**Key aspects:**
- Batching: combines consecutive stdout/stderr into single container
- Previous output detection via CSS selector (`output-item-id` attribute)
- Scroll position preservation for user experience
- Incremental append via `appendOutput()` for streaming scenarios
- Scrollbar visibility toggling based on content size

---

## Pattern 7: Link Detection & Markup Generation

**Where:** `extensions/notebook-renderers/src/linkify.ts:34-206`

**What:** Multi-regex link detector for web URLs, file paths, and HTML links with platform-aware path patterns.

```typescript
export class LinkDetector {

	static injectedHtmlCreator: (value: string) => string;

	private shouldGenerateHtml(trustHtml: boolean) {
		return trustHtml && (!!LinkDetector.injectedHtmlCreator || !!ttPolicy);
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
			try {
				let span: HTMLSpanElement | null = null;
				switch (part.kind) {
					case 'text':
						container.appendChild(document.createTextNode(part.value));
						break;
					case 'web':
					case 'path':
						container.appendChild(this.createWebLink(part.value));
						break;
					case 'html':
						span = document.createElement('span');
						span.innerHTML = this.createHtml(part.value)!;
						container.appendChild(span);
						break;
				}
			} catch (e) {
				container.appendChild(document.createTextNode(part.value));
			}
		}
		return container;
	}

	private detectLinks(text: string, trustHtml: boolean, detectFilepaths: boolean): LinkPart[] {
		if (text.length > MAX_LENGTH) {
			return [{ kind: 'text', value: text, captures: [] }];
		}

		const regexes: RegExp[] = [];
		const kinds: LinkKind[] = [];
		const result: LinkPart[] = [];

		if (this.shouldGenerateHtml(trustHtml)) {
			regexes.push(HTML_LINK_REGEX);
			kinds.push('html');
		}
		regexes.push(WEB_LINK_REGEX);
		kinds.push('web');
		if (detectFilepaths) {
			regexes.push(PATH_LINK_REGEX);
			kinds.push('path');
		}

		const splitOne = (text: string, regexIndex: number) => {
			if (regexIndex >= regexes.length) {
				result.push({ value: text, kind: 'text', captures: [] });
				return;
			}
			const regex = regexes[regexIndex];
			let currentIndex = 0;
			let match;
			regex.lastIndex = 0;
			while ((match = regex.exec(text)) !== null) {
				const stringBeforeMatch = text.substring(currentIndex, match.index);
				if (stringBeforeMatch) {
					splitOne(stringBeforeMatch, regexIndex + 1);
				}
				const value = match[0];
				result.push({
					value: value,
					kind: kinds[regexIndex],
					captures: match.slice(1)
				});
				currentIndex = match.index + value.length;
			}
			const stringAfterMatches = text.substring(currentIndex);
			if (stringAfterMatches) {
				splitOne(stringAfterMatches, regexIndex + 1);
			}
		};

		splitOne(text, 0);
		return result;
	}
}

const linkDetector = new LinkDetector();
export function linkify(text: string, linkOptions: LinkOptions, splitLines?: boolean) {
	return linkDetector.linkify(text, linkOptions, splitLines);
}
```

Platform-aware path patterns:
```typescript
const WEB_LINK_REGEX = new RegExp('(?:[a-zA-Z][a-zA-Z0-9+.-]{2,}:\\/\\/|data:|www\\.)[^\\s' + CONTROL_CODES + '"]{2,}[^\\s' + CONTROL_CODES + '"\')}\\],:;.!?]', 'ug');
const WIN_ABSOLUTE_PATH = /(?<=^|\s)(?:[a-zA-Z]:(?:(?:\\|\/)[\w\.-]*)+)/;
const POSIX_PATH = /(?<=^|\s)((?:\~|\.)?(?:\/[\w\.-]*)+)/;
const isWindows = (typeof navigator !== 'undefined') ? navigator.userAgent && navigator.userAgent.indexOf('Windows') >= 0 : false;
const PATH_LINK_REGEX = new RegExp(`${isWindows ? WIN_PATH.source : POSIX_PATH.source}${LINE_COLUMN.source}`, 'g');
```

**Key aspects:**
- Multi-pass link detection (HTML → web URLs → file paths)
- Platform-aware path matching (Windows vs. POSIX)
- Line:column markers for file links
- Fallback to plain text on parse errors
- Line-split option for preserving newlines in output

---

## Pattern 8: Settings Integration & Dynamic Re-rendering

**Where:** `extensions/notebook-renderers/src/rendererTypes.ts:31-39`

**What:** Settings context with event-driven updates for output layout adjustments.

```typescript
export interface RenderOptions {
	readonly lineLimit: number;
	readonly outputScrolling: boolean;
	readonly outputWordWrap: boolean;
	readonly linkifyFilePaths: boolean;
	readonly minimalError: boolean;
}

export type IRichRenderContext = RendererContext<void> & { readonly settings: RenderOptions; readonly onDidChangeSettings: Event<RenderOptions> };
```

Usage in error rendering (`index.ts:203-205`):
```typescript
disposableStore.push(ctx.onDidChangeSettings(e => {
	outputElement.classList.toggle('word-wrap', e.outputWordWrap);
}));
```

Usage in stream rendering (`index.ts:388-390`):
```typescript
disposableStore.push(ctx.onDidChangeSettings(e => {
	outputElement.classList.toggle('word-wrap', e.outputWordWrap);
}));
```

**Key aspects:**
- Settings broadcast via event emitter pattern
- Dynamic CSS class toggling (e.g., word-wrap, scrollable, scrollbar-visible)
- Listeners tracked in disposable store for cleanup
- Settings drive behavioral changes without re-render

---

## Pattern 9: Image Rendering with Blob URLs

**Where:** `extensions/notebook-renderers/src/index.ts:18-52`

**What:** Binary image data handling with object URL lifecycle management.

```typescript
function renderImage(outputInfo: OutputItem, element: HTMLElement): IDisposable {
	const blob = new Blob([outputInfo.data() as Uint8Array<ArrayBuffer>], { type: outputInfo.mime });
	const src = URL.createObjectURL(blob);
	const disposable = {
		dispose: () => {
			URL.revokeObjectURL(src);
		}
	};

	if (element.firstChild) {
		const display = element.firstChild as HTMLElement;
		if (display.firstChild && display.firstChild.nodeName === 'IMG' && display.firstChild instanceof HTMLImageElement) {
			display.firstChild.src = src;
			return disposable;
		}
	}

	const image = document.createElement('img');
	image.src = src;
	const alt = getAltText(outputInfo);
	if (alt) {
		image.alt = alt;
	}
	image.setAttribute('data-vscode-context', JSON.stringify({
		webviewSection: 'image',
		outputId: outputInfo.id,
		'preventDefaultContextMenuItems': true
	}));
	const display = document.createElement('div');
	display.classList.add('display');
	display.appendChild(image);
	element.appendChild(display);

	return disposable;
}
```

**Key aspects:**
- Binary-to-Blob conversion using MIME type
- Object URL lifecycle: create → assign → revoke
- Alt text from metadata for accessibility
- Context menu integration via `data-vscode-context` JSON
- Image reuse: update src if img already exists (avoid memory leak)

---

## Pattern 10: Disposable Store & Resource Cleanup

**Where:** `extensions/notebook-renderers/src/index.ts:151-165`

**What:** Simple disposable store for managing multiple cleanup handlers.

```typescript
function createDisposableStore(): { push(...disposables: IDisposable[]): void; dispose(): void } {
	const localDisposables: IDisposable[] = [];
	const disposable = {
		push: (...disposables: IDisposable[]) => {
			localDisposables.push(...disposables);
		},
		dispose: () => {
			localDisposables.forEach(d => d.dispose());
		}
	};

	return disposable;
}

type DisposableStore = ReturnType<typeof createDisposableStore>;
```

Usage:
```typescript
const disposables = new Map<string, IDisposable>();  // Per-output tracking
disposables.set(outputInfo.id, disposable);
disposables.get(outputInfo.id)?.dispose();
```

**Key aspects:**
- Aggregate multiple disposables (event listeners, blob URLs, etc.)
- Keyed by output ID for individual or bulk cleanup
- Pattern prevents memory leaks from lingering DOM listeners

---

## Extension Registration & MIME Types

**Where:** `extensions/notebook-renderers/package.json:18-42`

**What:** Declarative renderer registration with supported MIME types.

```json
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
```

**Key aspects:**
- Explicit MIME type list determines what this renderer handles
- `requiresMessaging: "never"` indicates no IPC with main process
- Multiple aliases for stdout/stderr (legacy and new formats)
- Custom MIME types for VS Code notebook errors

---

## Testing Pattern

**Where:** `extensions/notebook-renderers/src/test/notebookRenderer.test.ts:1-80`

**What:** JSDOM-based unit tests for renderer output handling.

```typescript
import * as assert from 'assert';
import { activate } from '..';
import { RendererApi } from 'vscode-notebook-renderer';
import { IDisposable, IRichRenderContext, OutputWithAppend, RenderOptions } from '../rendererTypes';
import { JSDOM } from 'jsdom';
import { LinkDetector } from '../linkify';

const dom = new JSDOM();
global.document = dom.window.document;

suite('Notebook builtin output renderer', () => {

	const error = {
		name: 'TypeError',
		message: 'Expected type `str`, but received type `<class \'int\'>`',
		stack: '[1;31m---------------------------------------------------------------------------[0m' +
			'[1;31mTypeError[0m                                 Traceback (most recent call last)' +
			'[1;32mc:\\src\\test\\ws1\\testing.py[0m in [0;36mline 2\n[0;32m      <a href=\'file:///c%3A/src/test/ws1/testing.py?line=34\'>35</a>[0m ...'
	};

	function createContext(settings?: optionalRenderOptions): IRichRenderContext {
		settingsChangedHandlers.length = 0;
		return {
			setState(_value: void) { },
			getState() { return undefined; },
			async getRenderer(_id): Promise<RendererApi | undefined> { return undefined; },
			settings: {
				outputWordWrap: true,
				outputScrolling: true,
				lineLimit: 30,
				...settings
			} as RenderOptions,
			onDidChangeSettings(listener: handler, _thisArgs?: any, disposables?: IDisposable[]) {
				settingsChangedHandlers.push(listener);

				const dispose = () => {
					settingsChangedHandlers.splice(settingsChangedHandlers.indexOf(listener), 1);
				};

				disposables?.push({ dispose });
				return {
					dispose
				};
			},
			workspace: {
				isTrusted: true
			}
		};
	}
});
```

**Key aspects:**
- JSDOM provides DOM API in Node.js environment
- Mock context factory for testing different settings combinations
- Error objects with ANSI-formatted stacks (IPython format)
- Settings handlers tracked in module-level array for test isolation

---

## Summary

The notebook-renderers extension demonstrates these core patterns for Tauri porting:

1. **MIME-based dispatch** - Switch on content type to select appropriate renderer
2. **Trusted content handling** - CSP-compliant HTML/script injection with policies
3. **Stateful parsing** - ANSI sequences, color codes, escape-sequence state machines
4. **Adaptive layout** - Scrolling vs. truncation, line limits, incremental streaming
5. **Event-driven updates** - Settings changes trigger CSS class toggling without re-render
6. **Resource lifecycle** - Disposable pattern for DOM listeners, blob URLs, subscriptions
7. **Platform awareness** - Windows vs. POSIX path detection, user agent sniffing
8. **Link detection** - Multi-pass regex for URLs, file paths, and embedded HTML
9. **Accessibility** - Alt text, aria labels, tab indices for keyboard navigation
10. **Extensibility** - Hook registration for pre/post-render transformations

These patterns are critical for replicating notebook output in Tauri, particularly around webview integration, DOM manipulation, and managing renderer lifecycle in a cross-platform GUI framework.

