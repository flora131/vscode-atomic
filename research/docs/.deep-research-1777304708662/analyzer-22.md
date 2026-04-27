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
