# Partition 22 of 79 — Findings

## Scope
`extensions/notebook-renderers/` (14 files, 3,546 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Notebook Renderers Extension — VS Code to Tauri/Rust Port Analysis

## Implementation

- `extensions/notebook-renderers/src/index.ts` — Core activation function and renderer dispatch logic; handles lifecycle, mime-type routing, DOM manipulation for images, HTML, JavaScript, errors, streams, and plaintext output
- `extensions/notebook-renderers/src/rendererTypes.ts` — TypeScript interfaces for rendering hooks, disposables, render options, and context types
- `extensions/notebook-renderers/src/textHelper.ts` — Output content creation, line truncation/scrolling limits, ANSI processing, container management
- `extensions/notebook-renderers/src/ansi.ts` — ANSI escape sequence parsing and styling (color codes, formatting); stateful sequence parsing for 256-color and 24-bit RGB
- `extensions/notebook-renderers/src/color.ts` — RGBA color representation and conversion utilities
- `extensions/notebook-renderers/src/colorMap.ts` — ANSI color identifier mappings to theme variables
- `extensions/notebook-renderers/src/htmlHelper.ts` — TrustedTypes policy wrapper for HTML/script sanitization
- `extensions/notebook-renderers/src/linkify.ts` — Link detection and URL/file-path hyperlink generation; regex-based web link and file path extraction
- `extensions/notebook-renderers/src/stackTraceHelper.ts` — IPython stack trace formatting, line number linkification, cell/file location extraction

## Tests

- `extensions/notebook-renderers/src/test/notebookRenderer.test.ts` — Main test suite using JSDOM; covers renderer activation, image rendering, HTML/JS output, error handling, stream appending, scrollable output, word wrap, settings changes
- `extensions/notebook-renderers/src/test/linkify.test.ts` — LinkDetector unit tests for URL/path regex matching and hyperlink generation
- `extensions/notebook-renderers/src/test/stackTraceHelper.test.ts` — Stack trace formatting tests for ANSI codes, IPython linkification
- `extensions/notebook-renderers/src/test/index.ts` — Test harness/setup

## Types / Interfaces

- `extensions/notebook-renderers/src/rendererTypes.ts` — Defines `IDisposable`, `HtmlRenderingHook`, `JavaScriptRenderingHook`, `RenderOptions`, `IRichRenderContext`, `OutputElementOptions`, `OutputWithAppend`

## Configuration

- `extensions/notebook-renderers/package.json` — Extension manifest declaring `notebookRenderer` contribution with entry point `renderer-out/index.js`, MIME type list (images, HTML, JS, errors, streams, text), workspace trust settings
- `extensions/notebook-renderers/tsconfig.json` — TypeScript configuration extending base, compiles to `out/`, targets ES2024 + DOM lib
- `extensions/notebook-renderers/esbuild.notebook.mts` — ESBuild configuration for bundling `src/index.ts` to `renderer-out/` using shared webview build rules
- `extensions/notebook-renderers/.npmrc` — NPM configuration
- `extensions/notebook-renderers/.gitignore` — Git ignore patterns

## Documentation

- `extensions/notebook-renderers/README.md` — Minimal feature list (image, error, stream renderers)
- `extensions/notebook-renderers/package.nls.json` — Localization strings for display name and description

## Examples / Fixtures

- `extensions/notebook-renderers/media/icon.png` — Extension icon

## Notable Clusters

- `extensions/notebook-renderers/src/` — 9 core implementation files (2,759 LOC); renderer activation, MIME dispatch, DOM rendering, ANSI/color parsing, HTML sanitization, link detection, stack trace formatting
- `extensions/notebook-renderers/src/test/` — 4 test files using JSDOM for DOM simulation; covers full output rendering pipeline

---

## Port Requirements Summary

**Webview Integration:** The notebook renderers are DOM-bound webview components loaded via `renderer-out/index.js` with no messaging required. Porting requires:

1. **Webview Binding Layer:** Map VS Code's `RendererContext<T>` API (getState/setState, workspace trust checks, onDidChangeSettings events) to Tauri webview JavaScript bridge
2. **DOM Manipulation:** All rendering uses native DOM APIs (createElement, innerHTML, classList, setAttribute). Rust-side would need to either:
   - Keep JavaScript-based rendering and call via webview bridge
   - Rewrite DOM generation in Rust and inject pre-rendered HTML
3. **MIME Type Dispatch:** Core logic is a switch statement on MIME types mapping to render functions. Straightforward to port.
4. **ANSI & Link Processing:** Stateful parsers that walk strings character-by-character. Core parsing logic is portable; HTML generation would tie to webview strategy.
5. **TrustedTypes & Security:** Uses browser TrustedTypes API for script/HTML safety. Tauri would need equivalent sandbox/CSP strategy.

**Key Technical Debt for Tauri:**
- No pure-Rust DOM library; must either bridge to webview JS or use Rust web framework (e.g., egui, leptos, yew compiled to WASM)
- JSDOM mocking in tests; Tauri Rust tests would need alternative (or keep WebDriver integration tests)
- Event channels (onDidChangeSettings) currently use TypeScript Event type; Tauri would use channels/mpsc

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
## Analysis: extensions/notebook-renderers — Partition 22

### Files Analysed

1. `/Users/norinlavaee/vscode-atomic/extensions/notebook-renderers/src/index.ts` (639 lines) — Core activation function and all renderer dispatch logic
2. `/Users/norinlavaee/vscode-atomic/extensions/notebook-renderers/src/rendererTypes.ts` (52 lines) — All shared interface and type definitions
3. `/Users/norinlavaee/vscode-atomic/extensions/notebook-renderers/src/textHelper.ts` (176 lines) — Output element construction, append logic, scroll management
4. `/Users/norinlavaee/vscode-atomic/extensions/notebook-renderers/src/ansi.ts` (452 lines) — Full ANSI escape sequence parser and span builder
5. `/Users/norinlavaee/vscode-atomic/extensions/notebook-renderers/src/linkify.ts` (212 lines) — URL, path, and HTML link detection and injection
6. `/Users/norinlavaee/vscode-atomic/extensions/notebook-renderers/src/stackTraceHelper.ts` (107 lines) — IPython/Jupyter stack trace formatting and linkification
7. `/Users/norinlavaee/vscode-atomic/extensions/notebook-renderers/src/htmlHelper.ts` (10 lines) — TrustedTypes policy creation
8. `/Users/norinlavaee/vscode-atomic/extensions/notebook-renderers/src/colorMap.ts` (62 lines) — 16 ANSI color identifier index mapped to VS Code CSS variables
9. `/Users/norinlavaee/vscode-atomic/extensions/notebook-renderers/src/color.ts` (1054 lines) — Full RGBA/HSLA/HSVA color model and CSS formatting
10. `/Users/norinlavaee/vscode-atomic/extensions/notebook-renderers/package.json` — Extension manifest declaring MIME types and renderer entrypoint

---

### Per-File Notes (with file:line)

#### `index.ts`

**Role:** The single activation entry point for the `vscode.builtin-renderer` notebook renderer. Registers rendering for all built-in MIME types, manages lifecycle disposables, and exposes an experimental hook API for HTML and JavaScript post-processing.

**Key symbols:**

- `activate` (`index.ts:419`) — exported `ActivationFunction<void>` required by the `vscode-notebook-renderer` API. Receives a `RendererContext<void>`. At activation time it injects a `<style>` block into `document.body` (`index.ts:427–537`) and then returns the renderer object.

- Returned renderer object has four methods (`index.ts:540–638`):
  - `renderOutputItem(outputInfo, element, signal?)` (`index.ts:541`) — dispatches on `outputInfo.mime` with a `switch` statement (`index.ts:543–609`).
  - `disposeOutputItem(id?)` (`index.ts:615`) — calls `dispose()` on the matching disposable stored in `Map<string, IDisposable>`.
  - `experimental_registerHtmlRenderingHook(hook)` (`index.ts:622`) — adds to `Set<HtmlRenderingHook> htmlHooks`.
  - `experimental_registerJavaScriptRenderingHook(hook)` (`index.ts:630`) — adds to `Set<JavaScriptRenderingHook> jsHooks`.

- `disposables: Map<string, IDisposable>` (`index.ts:420`) — keyed by `outputInfo.id`; each render call disposes the previous entry before creating a new one.

**MIME dispatch table** (`index.ts:543–609`):

| MIME type(s) | Handler function |
|---|---|
| `text/html`, `image/svg+xml` | `renderHTML` (trusted workspace only) |
| `application/javascript` | `renderJavascript` (trusted workspace only) |
| `image/gif`, `image/png`, `image/jpeg`, `image/git` | `renderImage` |
| `application/vnd.code.notebook.error` | `renderError` |
| `application/vnd.code.notebook.stdout`, `application/x.notebook.stdout`, `application/x.notebook.stream` | `renderStream(…, false, …)` |
| `application/vnd.code.notebook.stderr`, `application/x.notebook.stderr` | `renderStream(…, true, …)` |
| `text/plain` + any `text/*` fallthrough | `renderText` |

**`renderImage`** (`index.ts:18`):
- Calls `outputInfo.data()` returning `Uint8Array<ArrayBuffer>`, constructs a `Blob`, then `URL.createObjectURL`.
- Reuses an existing `<img>` element if already present as `element.firstChild.firstChild` (`index.ts:29`).
- Reads optional `metadata.vscode_altText` via `getAltText` (`index.ts:77`) and sets `image.alt`.
- Attaches `data-vscode-context` JSON attribute for custom context-menu handling (`index.ts:41`).
- Returns a disposable that calls `URL.revokeObjectURL` (`index.ts:23`).

**`renderHTML`** (`index.ts:107`):
- Clears container, creates a `<div>`, sets `innerHTML` via `ttPolicy?.createHTML(htmlContent)` (`index.ts:111`).
- Calls `fixUpSvgElement` (`index.ts:85`) to inject an accessible `<title>` and `data-vscode-context` attribute on SVG roots.
- Iterates `htmlHooks` calling `hook.postRender(outputInfo, element, signal)` sequentially, replacing the element with the returned value (`index.ts:115–119`).
- Appends final element and calls `domEval` (`index.ts:123`).

**`renderJavascript`** (`index.ts:126`):
- Gets script text via `outputInfo.text()`.
- Iterates `jsHooks` calling `hook.preEvaluate(outputInfo, container, scriptText, signal)` (`index.ts:129`).
- Creates a `<script type="module">` tag, wraps in `<div>`, sets via `ttPolicy?.createHTML` (`index.ts:141`), then calls `domEval` to execute.

**`domEval`** (`index.ts:58`):
- Queries all `<script>` tags within the container element.
- Recreates each as a new `<script>` element with text set via `ttPolicy?.createScript(node.innerText)` (`index.ts:63`).
- Preserves `type`, `src`, `nonce`, `noModule`, `async` attributes (`index.ts:54`).
- Appends and immediately removes the new script to trigger execution (`index.ts:73`).

**`renderError`** (`index.ts:167`):
- JSON-parses `outputInfo.text()` as `ErrorLike` (`index.ts:181`).
- If `err.stack` exists, calls `formatStackTrace(err.stack, trustHtml)` from `stackTraceHelper` (`index.ts:193`).
- Checks `ctx.settings.minimalError` (`index.ts:191`) to choose between compact (`createMinimalError`) and full traceback display.
- Calls `createOutputContent(outputInfo.id, formattedStack, outputOptions)` (`index.ts:199`) to produce a DOM element.
- Subscribes to `ctx.onDidChangeSettings` to reactively toggle `word-wrap` class (`index.ts:203`).

**`createMinimalError`** (`index.ts:226`):
- Builds a two-section layout: a header div with optional error location `<a>` link and error message, and a `<ul>` with a "Show Details" toggle link.
- The toggle link uses `onclick` to show/hide the stack trace div and flip its label (`index.ts:262`).
- Applies manual `mouseover`/`mouseout` hover class because `:hover` CSS does not work in the webview (`index.ts:244`).

**`renderStream`** (`index.ts:350`):
- Adds `output-stream` CSS class to `outputElement`.
- Calls `getPreviousMatchingContentGroup` (`index.ts:274`) to check if the previous sibling output cell also produced stream output; if so, appends into that shared DOM subtree rather than the current element.
- Calls `appendOutput` or `createOutputContent` from `textHelper` depending on whether matching content already exists for `outputInfo.id`.
- Calls `initializeScroll` (`index.ts:314`) to set up scroll position and event listeners.

**`renderText`** (`index.ts:398`):
- Clears container, calls `createOutputContent`, adds CSS classes `output-plaintext`, `word-wrap` (reactive), `scrollable`.
- Calls `initializeScroll`.

**Scroll management** (`index.ts:292–343`):
- `onScrollHandler` (`index.ts:292`) — adds/removes `more-above` class based on `scrollTop === 0`.
- `onKeypressHandler` (`index.ts:301`) — calls `e.stopPropagation()` on arrow/page/home/end keys to prevent notebook cell navigation while inside a scrollable output.
- `initializeScroll` (`index.ts:314`) — checks `scrollHeight > clientHeight` to toggle `scrollbar-visible`, sets `scrollTop` to provided value or `scrollHeight`. Attaches and registers disposal for both `scroll` and `keydown` listeners.
- `findScrolledHeight` (`index.ts:329`) — reads existing scroll position; returns `undefined` if already at bottom (within 2px tolerance).

**`scrollingEnabled`** (`index.ts:338`):
- Reads `output.metadata.scrollable` (boolean) if present, otherwise falls back to `options.outputScrolling` setting.

---

#### `rendererTypes.ts`

**Role:** Type contract layer between `index.ts` and `textHelper.ts`/`linkify.ts`. All interfaces are consumed by both renderer internals and by external hook registrants.

**Key symbols:**

- `IDisposable` (`rendererTypes.ts:9`) — `{ dispose(): void }`.

- `HtmlRenderingHook` (`rendererTypes.ts:13`) — `postRender(outputItem, element, signal): HTMLElement | undefined | Promise<...>`. Called after HTML rendering but before DOM insertion.

- `JavaScriptRenderingHook` (`rendererTypes.ts:22`) — `preEvaluate(outputItem, element, script, signal): string | undefined | Promise<...>`. Called before script evaluation.

- `RenderOptions` (`rendererTypes.ts:31`) — five readonly settings drawn from `ctx.settings`:
  - `lineLimit: number` — max lines before truncation in non-scrollable mode.
  - `outputScrolling: boolean` — default scrolling mode.
  - `outputWordWrap: boolean` — controls `word-wrap` CSS class.
  - `linkifyFilePaths: boolean` — enables path link detection.
  - `minimalError: boolean` — enables compact error display.

- `IRichRenderContext` (`rendererTypes.ts:39`) — `RendererContext<void> & { settings: RenderOptions; onDidChangeSettings: Event<RenderOptions> }`. Extends the standard VS Code renderer context with settings access.

- `OutputElementOptions` (`rendererTypes.ts:41`) — shape passed to `createOutputContent` and `appendOutput`: `linesLimit`, `scrollable?`, `error?`, `trustHtml?`, `linkifyFilePaths`.

- `OutputWithAppend` (`rendererTypes.ts:49`) — extends `OutputItem` with optional `appendedText?(): string | undefined` for streaming append support.

---

#### `textHelper.ts`

**Role:** Constructs the actual DOM subtree for text-based outputs. Handles line buffering, soft/hard line limits, truncation messages, and scrollable vs. non-scrollable layout selection.

**Key symbols:**

- `scrollableClass = 'scrollable'` (`textHelper.ts:9`) — single CSS class string shared with `index.ts`.

- `softScrollableLineLimit = 5000` (`textHelper.ts:11`), `hardScrollableLineLimit = 8000` (`textHelper.ts:12`) — upper bounds for scrollable output rendering.

- `outputLengths: Record<string, number>` (`textHelper.ts:117`) — module-level map from output ID to accumulated line count across appends.

- `createOutputContent(id, outputText, options)` (`textHelper.ts:137`):
  - Splits `outputText` on `/\r\n|\r|\n/g`.
  - If `options.scrollable` is true, delegates to `scrollableArrayOfString`; otherwise to `truncatedArrayOfString`.
  - Sets `output-item-id` attribute and optional `error` class on the returned element.

- `truncatedArrayOfString(id, buffer, linesLimit, linkOptions)` (`textHelper.ts:72`):
  - If `buffer.length <= linesLimit`, calls `handleANSIOutput(buffer.join('\n'), linkOptions)` once.
  - Otherwise renders first `linesLimit - 5` lines, then an ellipsis `<div>`, then last 5 lines, then `generateViewMoreElement(id)` (`textHelper.ts:17`) which builds three `command:`-scheme links: `cellOutput.enableScrolling`, `workbench.action.openLargeOutput`, and settings.

- `scrollableArrayOfString(id, buffer, linkOptions)` (`textHelper.ts:101`):
  - If `buffer.length > softScrollableLineLimit` (5000), prepends `generateNestedViewAllElement(id)` — a `...` link to `workbench.action.openLargeOutput`.
  - Renders only the last `softScrollableLineLimit` lines via `handleANSIOutput`.

- `appendOutput(outputInfo, existingContent, options)` (`textHelper.ts:158`):
  - If `appendedText` is available and `options.scrollable` is true, calls `appendScrollableOutput`.
  - If `appendScrollableOutput` returns false (hard limit exceeded), falls through to full `createOutputContent` replacement.
  - Otherwise replaces `existingContent` with new content and removes any subsequent sibling nodes (stale streaming content).

- `appendScrollableOutput(element, id, appended, linkOptions)` (`textHelper.ts:119`):
  - Guards against exceeding `hardScrollableLineLimit` (8000 cumulative lines).
  - Appends result of `handleANSIOutput` for the incremental text.
  - Updates `outputLengths[id]`.
  - Returns `false` when the hard limit would be exceeded.

---

#### `ansi.ts`

**Role:** Parses ANSI SGR (Select Graphic Rendition) escape sequences in text and builds a `<span>` tree with CSS classes and inline styles reflecting all formatting codes.

**Key symbols:**

- `handleANSIOutput(text, linkOptions): HTMLSpanElement` (`ansi.ts:11`) — main entry. Creates a root `<span>`, then iterates character-by-character:
  - Detects escape sequences by `charCode === 27` followed by `[` (`ansi.ts:30`).
  - Accumulates the sequence until a terminating character matching `/^[ABCDHIJKfhmpsu]$/` (`ansi.ts:44`).
  - On a valid sequence, flushes the current text buffer via `appendStylizedStringToContainer` (`ansi.ts:54`), then parses the SGR codes.
  - Non-escape characters accumulate in `buffer` (`ansi.ts:93`).
  - After the loop, flushes any remaining buffer (`ansi.ts:100`).

- **Closure-scoped state** (`ansi.ts:16–21`): `styleNames: string[]`, `customFgColor`, `customBgColor`, `customUnderlineColor`, `colorsInverted: boolean`, `currentPos: number`, `buffer: string`. All are reassigned by the inner functions below.

- `setBasicFormatters(styleCodes)` (`ansi.ts:151`) — handles codes 0–75 via a switch:
  - Code 0: resets all styles and colors.
  - Codes 1–9, 21–29, 53, 55, 73–75: toggle named CSS classes prefixed with `code-` (e.g., `code-bold`, `code-italic`, `code-underline`, `code-strike-through`, etc.).
  - Codes 10–20: sets alternate font classes `code-font-{N}`.
  - Codes 39/49/59: calls `changeColor` with `undefined` to clear fg/bg/underline.
  - Default fallthrough: delegates to `setBasicColor(code)` for numeric color codes.

- `setBasicColor(styleCode)` (`ansi.ts:356`) — maps ranges 30–37 (dark fg), 90–97 (bright fg), 40–47 (dark bg), 100–107 (bright bg) to `ansiColorIdentifiers[colorIndex].colorValue` via `changeColor`.

- `set24BitColor(styleCodes, colorType)` (`ansi.ts:304`) — constructs `new RGBA(r, g, b)` from codes at indices [2],[3],[4] and calls `changeColor`.

- `set8BitColor(styleCodes, colorType)` (`ansi.ts:323`) — dispatches to `calcANSI8bitColor(colorNumber)` for colors 16–255. For colors 0–15, remaps to basic color code range and calls `setBasicColor`.

- `calcANSI8bitColor(colorNumber)` (`ansi.ts:422`) — exported utility:
  - 16–231: converts to one of 216 RGB cube colors.
  - 232–255: converts to 24 grayscale levels.
  - Returns `undefined` for 0–15 (standard 16 colors).

- `changeColor(colorType, color?)` (`ansi.ts:114`) — updates the three color variables and maintains the `code-{colorType}-colored` class in `styleNames`.

- `reverseForegroundAndBackgroundColors()` (`ansi.ts:132`) — swaps `customFgColor` and `customBgColor` for SGR code 7 (reverse video).

- `appendStylizedStringToContainer(root, stringContent, linkOptions, cssClasses, customTextColor?, customBackgroundColor?, customUnderlineColor?)` (`ansi.ts:381`) — creates a `<span>`, calls `linkify(stringContent, linkOptions, true)` for the text content, sets `className` to `cssClasses.join(' ')`, and applies inline `color`, `backgroundColor`, `textDecorationColor` styles using `Color.Format.CSS.formatRGB`.

---

#### `linkify.ts`

**Role:** Detects and wraps URLs, file paths, and trusted HTML links within plain text, producing a `<span>` subtree.

**Key symbols:**

- `WEB_LINK_REGEX` (`linkify.ts:9`) — matches `http://`, `https://`, `ftp://`, `data:`, and `www.` URLs with lookahead/behind to avoid trailing punctuation.

- `PATH_LINK_REGEX` (`linkify.ts:17`) — platform-conditional: uses `WIN_PATH` on Windows (detected via `navigator.userAgent` at `linkify.ts:16`) or `POSIX_PATH` on other platforms, with optional `:line:column` suffix captured by `LINE_COLUMN` (`linkify.ts:15`).

- `HTML_LINK_REGEX` (`linkify.ts:18`) — matches existing `<a href="...">...</a>` tags for trusted HTML pass-through.

- `MAX_LENGTH = 2000` (`linkify.ts:20`) — texts longer than this are returned as plain text with no link detection.

- `LinkDetector` class (`linkify.ts:34`):
  - `static injectedHtmlCreator` (`linkify.ts:37`) — test seam for TrustedTypes bypass.
  - `linkify(text, options, splitLines?)` (`linkify.ts:59`) — main method. When `splitLines` is true, splits on `\n`, recurses per line, and wraps in a container `<span>`. Each `LinkPart` is handled: `text` nodes as `TextNode`, `web`/`path` via `createWebLink`, `html` via `innerHTML` with `createHtml`.
  - `detectLinks(text, trustHtml, detectFilepaths)` (`linkify.ts:154`) — builds `regexes[]` and `kinds[]` arrays (conditionally including HTML and path regexes), then applies them in priority order via a recursive `splitOne` function (`linkify.ts:175`). Interleaves regex matches with recursive calls on unmatched substrings.
  - `createWebLink(url)` (`linkify.ts:104`) — creates `<a href=url>` via `createLink`.
  - `createHtml(value)` (`linkify.ts:43`) — uses `LinkDetector.injectedHtmlCreator` if set, otherwise `ttPolicy?.createHTML`.

- `linkify(text, linkOptions, splitLines?)` (`linkify.ts:209`) — module-level function delegating to singleton `linkDetector`.

---

#### `stackTraceHelper.ts`

**Role:** Pre-processes IPython/Jupyter stack traces before they reach the ANSI renderer. Strips problematic ANSI background color codes, converts foreground colors after `-->` markers, and optionally injects `<a href=...>` links for cell and file locations.

**Key symbols:**

- `formatStackTrace(stack, trustHtml)` (`stackTraceHelper.ts:6`):
  - Strips ANSI background colors (codes 40–49): `stack.replace(/\[4\dm/g, '')` and negative-lookbehind compound sequence removal (`stackTraceHelper.ts:13–14`).
  - Strips custom foreground (code 38 sequences) by replacing with default foreground (`[39m`) (`stackTraceHelper.ts:17`).
  - Normalizes foreground colors after `-->` line markers to default foreground (`stackTraceHelper.ts:20–23`).
  - Calls `isIpythonStackTrace(cleaned)` and if true and `trustHtml` is set, delegates to `linkifyStack(cleaned)` (`stackTraceHelper.ts:26`).
  - Returns `{ formattedStack, errorLocation? }`.

- `isIpythonStackTrace(stack)` (`stackTraceHelper.ts:42`) — tests the cleaned text against `cellRegex`, `inputRegex`, and `fileRegex`.

- Four regex patterns (`stackTraceHelper.ts:33–40`):
  - `formatSequence` — matches any ANSI escape sequence.
  - `fileRegex` — matches `File <path>:<linenum>` lines.
  - `lineNumberRegex` — matches `-->` prefixes followed by a line number.
  - `cellRegex` — matches IPython's "Cell In[N], line M" format with interleaved ANSI codes.
  - `inputRegex` — matches older IPython "Input In[N]" format.

- `linkifyStack(stack)` (`stackTraceHelper.ts:55`):
  - Iterates lines. Tracks current `fileOrCell: location | undefined`.
  - `fileRegex` match: records a `{ kind: 'file', path }` location.
  - `cellRegex` match: records a `{ kind: 'cell', path }` and rewrites the line to contain `<a href='vscode-notebook-cell:?execution_count=N&line=M'>line M</a>` (`stackTraceHelper.ts:74`).
  - `inputRegex` match: records a cell location with `execution_count` only (older IPython, no line number in cell header).
  - `lineNumberRegex` match within current file/cell context: rewrites the `-->` line to `<a href='<path>:<num>'>num</a>` (file) or `<a href='<path>&line=<num>'>num</a>` (cell) (`stackTraceHelper.ts:94–98`).
  - Returns `{ formattedStack: lines.join('\n'), errorLocation }` where `errorLocation` is the first cell link found.

---

#### `htmlHelper.ts`

**Role:** Creates the TrustedTypes policy used to safely set `innerHTML` and `script.text`.

- `ttPolicy` (`htmlHelper.ts:6`) — created via `window.trustedTypes?.createPolicy('notebookRenderer', { createHTML: v => v, createScript: v => v })`. Both policies are pass-through (no sanitization); the policy's existence enforces TrustedTypes compliance without transforming content. Set to `undefined` if `window` is not available (e.g., in test environments).

---

#### `colorMap.ts`

**Role:** Initialises `ansiColorIdentifiers[]` — the 16-entry lookup table used by `ansi.ts` for SGR color codes 0–15.

- `ansiColorMap` (`colorMap.ts:7`) — object literal mapping VS Code theme token IDs (e.g., `'terminal.ansiBlack'`) to their index (0–15).
- Initialization loop (`colorMap.ts:58–62`) — iterates `ansiColorMap`, builds `colorName` by stripping the `'terminal.'` prefix (13 chars), computes `colorValue` as a CSS custom property reference `var(--vscode-terminal-ansiBlack)` (with `.` replaced by `-`), and stores both as `ansiColorIdentifiers[index]`.
- Result: `ansiColorIdentifiers[0] = { colorName: 'ansiBlack', colorValue: 'var(--vscode-terminal-ansiBlack)' }` through index 15.

---

#### `color.ts`

**Role:** Self-contained color model providing `RGBA`, `HSLA`, `HSVA` value types and the `Color` wrapper with blending, luminance, contrast, and CSS formatting utilities. Also defines the `CharCode` enum used internally by `parseHex`.

**Key symbols:**

- `RGBA` (`color.ts:445`) — `(r, g, b, a=1)` constructor with range-clamping and integer truncation via bitwise OR. `r,g,b` clamped to [0,255]; `a` clamped to [0,1] with 3-decimal rounding.

- `HSLA` (`color.ts:480`) — `fromRGBA(rgba)` (`color.ts:521`) and `toRGBA(hsla)` (`color.ts:574`) static converters.

- `HSVA` (`color.ts:593`) — `fromRGBA(rgba)` (`color.ts:629`) and `toRGBA(hsva)` (`color.ts:653`) static converters.

- `Color` class (`color.ts:688`):
  - Constructor accepts `RGBA | HSLA | HSVA`; stores canonical `rgba` and lazy `_hsla`/`_hsva`.
  - `getRelativeLuminance()` (`color.ts:736`) — W3C formula: `0.2126*R + 0.7152*G + 0.0722*B`.
  - `getContrastRatio(another)` (`color.ts:754`) — W3C contrast formula.
  - `blend(c)` (`color.ts:815`) — alpha-compositing over-operator.
  - `makeOpaque(opaqueBackground)` (`color.ts:834`) — flattens semi-transparent color onto opaque background.
  - `flatten(...backgrounds)` (`color.ts:851`) — reduces background stack using `_flatten`.

- `Color.Format.CSS` namespace (`color.ts:907`):
  - `formatRGB(color)` (`color.ts:909`) — returns `rgb(r, g, b)` or `rgba(...)` string. Used in `ansi.ts:appendStylizedStringToContainer`.
  - `parseHex(hex)` (`color.ts:973`) — handles `#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA` formats using `CharCode` enum.

---

### Cross-Cutting Synthesis

The `notebook-renderers` extension is a fully self-contained webview-side renderer that runs entirely in a sandboxed browser context. Its single public API surface is the `ActivationFunction` contract from `vscode-notebook-renderer`. The extension registers one renderer (`vscode.builtin-renderer`) via `package.json` contributions, which handles 13 distinct MIME types through a centralized dispatch switch in `index.ts:541–609`.

The architecture is strictly layered: `index.ts` owns DOM structure, lifecycle, and MIME routing; `textHelper.ts` owns line-based text layout and scroll state; `ansi.ts` owns ANSI token parsing and CSS class application; `linkify.ts` owns regex-based link detection; `stackTraceHelper.ts` owns IPython-specific stack cleaning and hyperlink injection; and `color.ts`/`colorMap.ts`/`htmlHelper.ts` serve as pure utilities. No module has circular dependencies.

For a Tauri port, this layer is the one most likely to transfer without structural changes, since it is already a self-contained webview bundle with no Node.js or Electron APIs. The critical porting considerations are: (1) TrustedTypes policy creation in `htmlHelper.ts:6` requires a `window.trustedTypes` implementation in Tauri's WebView; (2) `command:`-scheme hrefs embedded in `textHelper.ts` (e.g., `cellOutput.enableScrolling`, `workbench.action.openLargeOutput`) must be intercepted and mapped to equivalent Tauri IPC commands; (3) the `data-vscode-context` JSON attribute used for image and text context menus (`index.ts:41`, `textHelper.ts:74`) must be handled by a Tauri-side context menu listener; (4) ANSI color values are expressed as `var(--vscode-terminal-ansi*)` CSS custom properties set by the host frame, so Tauri's webview must inject identical CSS variables; (5) the `appendedText?()` streaming API on `OutputWithAppend` (`rendererTypes.ts:49`) relies on the host calling `renderOutputItem` with an enriched output object — Tauri's notebook controller must preserve this interface.

---

### Out-of-Partition References

- `vscode-notebook-renderer` npm package — provides `ActivationFunction`, `OutputItem`, `RendererContext` types (`index.ts:6`, `rendererTypes.ts:6`). This is a type-only dev dependency; the runtime API is provided by VS Code's webview host frame.
- `vscode` module — provides `Event<T>` type imported in `rendererTypes.ts:7`. This is a type import only; no VS Code extension host IPC is used at runtime.
- `command:cellOutput.enableScrolling` URI scheme (`textHelper.ts:27`) — handled by the VS Code notebook editor (outside this partition).
- `command:workbench.action.openLargeOutput` URI scheme (`textHelper.ts:36`, `textHelper.ts:63`) — handled by VS Code's workbench.
- `command:workbench.action.openSettings` URI scheme (`textHelper.ts:47`) — handled by VS Code's settings editor.
- `vscode-notebook-cell:?execution_count=N` URI scheme (`stackTraceHelper.ts:72`) — handled by VS Code's notebook document model.
- `data-vscode-context` attribute consumer — lives in VS Code's webview host layer (outside this extension), which reads the JSON and constructs the native context menu.
- `var(--vscode-terminal-ansi*)` CSS variables (`colorMap.ts:61`) — injected by VS Code's theme service into the webview's root document.
- `--notebook-cell-output-*` CSS variables (`index.ts:439–445`) — injected by VS Code's notebook editor into the renderer frame.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
