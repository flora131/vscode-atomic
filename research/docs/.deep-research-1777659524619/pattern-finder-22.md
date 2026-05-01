# VS Code Notebook Renderers: Activation & Webview Pipeline Patterns

## Research Question
What patterns exist for activating and rendering notebook output in VS Code's webview layer, relevant to porting IDE functionality to Tauri/Rust?

## Key Finding
VS Code's notebook renderer system follows a **declarative activation contract** with explicit lifecycle management, mime-type dispatch, and disposable resource tracking. This is a webview-only extension (no main process access) that must handle rendering across multiple mime types with aggressive memory management.

---

## Patterns Identified

#### Pattern: Activation Function Contract
**Where:** `extensions/notebook-renderers/src/index.ts:419-639`
**What:** Typed activation function returning renderer API with specific lifecycle hooks.
```typescript
export const activate: ActivationFunction<void> = (ctx) => {
	const disposables = new Map<string, IDisposable>();
	const htmlHooks = new Set<HtmlRenderingHook>();
	const jsHooks = new Set<JavaScriptRenderingHook>();

	const latestContext = ctx as (RendererContext<void> & { 
		readonly settings: RenderOptions; 
		readonly onDidChangeSettings: Event<RenderOptions> 
	});

	// Style setup omitted for brevity...
	
	return {
		renderOutputItem: async (outputInfo, element, signal?: AbortSignal) => {
			// MIME type dispatch logic
		},
		disposeOutputItem: (id: string | undefined) => {
			// Explicit cleanup
		},
		experimental_registerHtmlRenderingHook: (hook: HtmlRenderingHook): IDisposable => {
			// Hook registration with disposal
		},
		experimental_registerJavaScriptRenderingHook: (hook: JavaScriptRenderingHook): IDisposable => {
			// Hook registration with disposal
		}
	};
};
```

**Variations / call-sites:** 
- Declared in `package.json:22` as `"entrypoint": "./renderer-out/index.js"`
- Tested in `src/test/notebookRenderer.test.ts:145` by direct invocation
- Initialization occurs in webview context only, never in main process

#### Pattern: MIME Type Dispatch with Trusted Workspace Checks
**Where:** `extensions/notebook-renderers/src/index.ts:541-609`
**What:** Switch statement multiplexing across mime types with security validation before rendering.
```typescript
renderOutputItem: async (outputInfo, element, signal?: AbortSignal) => {
	element.classList.add('remove-padding');
	switch (outputInfo.mime) {
		case 'text/html':
		case 'image/svg+xml': {
			if (!ctx.workspace.isTrusted) {
				return; // Security boundary
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
		case 'image/jpeg':
		case 'image/git': {
			disposables.get(outputInfo.id)?.dispose();
			const disposable = renderImage(outputInfo, element);
			disposables.set(outputInfo.id, disposable);
			break;
		}
		// ... more cases for error, stdout, stderr, text/plain
		default:
			if (outputInfo.mime.indexOf('text/') > -1) {
				disposables.get(outputInfo.id)?.dispose();
				const disposable = renderText(outputInfo, element, latestContext);
				disposables.set(outputInfo.id, disposable);
			}
			break;
	}
};
```

**Variations / call-sites:** 
- 8+ mime types handled: HTML, SVG, JavaScript, images (GIF/PNG/JPEG), errors, stdout, stderr, plain text
- Each type has specialized renderer function (renderHTML, renderImage, renderError, renderStream, renderText)
- Disposable tracking happens per output ID in Map

#### Pattern: Disposable Store (Resource Lifecycle Management)
**Where:** `extensions/notebook-renderers/src/index.ts:151-163`
**What:** Utility for accumulating and batch-disposing multiple resources.
```typescript
function createDisposableStore(): { 
	push(...disposables: IDisposable[]): void; 
	dispose(): void 
} {
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
```

**Variations / call-sites:** 
- Used in `renderError:173`, `renderStream:351`, `renderText:399`
- Event listeners added as disposables in `initializeScroll:321-323`
- Settings change listeners pushed to disposables via `ctx.onDidChangeSettings` callback

#### Pattern: Scroll Event Listener Disposal
**Where:** `extensions/notebook-renderers/src/index.ts:314-325`
**What:** Register DOM event listeners with cleanup via disposables.
```typescript
function initializeScroll(
	scrollableElement: HTMLElement, 
	disposables: DisposableStore, 
	scrollTop?: number
) {
	if (scrollableElement.classList.contains(scrollableClass)) {
		const scrollbarVisible = scrollableElement.scrollHeight > scrollableElement.clientHeight;
		scrollableElement.classList.toggle('scrollbar-visible', scrollbarVisible);
		scrollableElement.scrollTop = scrollTop !== undefined ? scrollTop : scrollableElement.scrollHeight;
		if (scrollbarVisible) {
			scrollableElement.addEventListener('scroll', onScrollHandler);
			disposables.push({ 
				dispose: () => scrollableElement.removeEventListener('scroll', onScrollHandler) 
			});
			scrollableElement.addEventListener('keydown', onKeypressHandler);
			disposables.push({ 
				dispose: () => scrollableElement.removeEventListener('keydown', onKeypressHandler) 
			});
		}
	}
}
```

**Variations / call-sites:** 
- Called from `renderStream:392`, `renderText:414`
- Disposables map at activation level stores per-output cleanup
- `disposeOutputItem` method clears by ID or all if undefined

#### Pattern: Trusted Types Policy for HTML Injection
**Where:** `extensions/notebook-renderers/src/htmlHelper.ts:6-10`
**What:** Conditional Trusted Types API for XSS protection.
```typescript
export const ttPolicy = (typeof window !== 'undefined') ?
	(window as Window & { trustedTypes?: any }).trustedTypes?.createPolicy(
		'notebookRenderer', 
		{
			createHTML: (value: string) => value,
			createScript: (value: string) => value,
		}
	) : undefined;
```

**Variations / call-sites:** 
- Used in `renderHTML:111` to sanitize HTML content
- Fallback in `renderJavascript:141` for script outerHTML
- Linkifier class `linkify.ts:40-50` has conditional usage for HTML creation

#### Pattern: Metadata-Driven Rendering Options
**Where:** `extensions/notebook-renderers/src/textHelper.ts:137-156`
**What:** Output configuration from both item metadata and context settings.
```typescript
export function createOutputContent(
	id: string, 
	outputText: string, 
	options: OutputElementOptions
): HTMLElement {
	const { linesLimit, error, scrollable, trustHtml, linkifyFilePaths } = options;
	const linkOptions: LinkOptions = { linkifyFilePaths, trustHtml };
	const buffer = outputText.split(/\r\n|\r|\n/g);
	outputLengths[id] = Math.min(buffer.length, softScrollableLineLimit);

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
```

**Variations / call-sites:** 
- `RenderOptions` interface defines: lineLimit, outputScrolling, outputWordWrap, linkifyFilePaths, minimalError
- Settings can be overridden per-output via metadata on OutputItem
- Dynamic updates via `onDidChangeSettings` event trigger re-render

#### Pattern: Hook-Based Extensibility
**Where:** `extensions/notebook-renderers/src/rendererTypes.ts:13-29`
**What:** Hook interfaces for post-render and pre-evaluate customization.
```typescript
export interface HtmlRenderingHook {
	/**
	 * Invoked after the output item has been rendered but before 
	 * it has been appended to the document.
	 * @return A new HTMLElement or undefined to continue using provided element.
	 */
	postRender(
		outputItem: OutputItem, 
		element: HTMLElement, 
		signal: AbortSignal
	): HTMLElement | undefined | Promise<HTMLElement | undefined>;
}

export interface JavaScriptRenderingHook {
	/**
	 * Invoked before the script is evaluated.
	 * @return A new string of JavaScript or undefined to continue using provided string.
	 */
	preEvaluate(
		outputItem: OutputItem, 
		element: HTMLElement, 
		script: string, 
		signal: AbortSignal
	): string | undefined | Promise<string | undefined>;
}
```

**Variations / call-sites:** 
- Hooks collected in Sets during activation
- `renderHTML:115-120` calls all registered HTML hooks in sequence
- `renderJavascript:129-134` calls JS hooks before script evaluation
- Experimental (prefixed with underscore): designed for future extension

---

## Critical Observations for Tauri/Rust Port

1. **Activation is Synchronous-to-Async Boundary**: The `activate` function returns an object immediately, but methods return `async` for rendering. Rust would need explicit Future handling.

2. **Resource Tracking is Pervasive**: Every output item must be tracked via disposable map with explicit cleanup. Rust's ownership system could eliminate this boilerplate, but the semantic pattern (allocation/tracking/cleanup) remains.

3. **Settings as Live Observable**: The `onDidChangeSettings` event fires during runtime and causes re-renders. This requires reactive/event-driven architecture, not just one-time config.

4. **Mime Type Enumeration is Extensible via Default Case**: New mime types can be added by checking prefix (e.g., `text/*`). Tauri port should support wildcard matching or trait-based dispatch.

5. **Trusted Types / XSS Protection is Webview-Specific**: Modern browser security features (Trusted Types, CSP) would be unavailable in pure Rust. Alternative sanitization (e.g., ammonia crate) would be required.

6. **Hook System is "Experimental"**: Indicates API stability risk. Third-party extensions depend on this; a Tauri port would need stable ABI guarantees.

---

## Extension Configuration (package.json)

**Relevant metadata:**
- `notebookRenderer.id`: `vscode.builtin-renderer` (namespace-scoped)
- `notebookRenderer.entrypoint`: `./renderer-out/index.js` (compiled output location)
- `notebookRenderer.requiresMessaging`: `never` (fully sandboxed, no extension host communication)
- Supports 13 mime types across images, HTML, JavaScript, text, and notebook-specific error/stdout/stderr formats

---

## Related Utilities
- `src/ansi.ts` - ANSI escape sequence parser for colored terminal output
- `src/linkify.ts` - URL/file path linkifier with regex patterns
- `src/stackTraceHelper.ts` - IPython stack trace formatter and cell/file link generator
- `src/colorMap.ts` - ANSI color identifier mappings
