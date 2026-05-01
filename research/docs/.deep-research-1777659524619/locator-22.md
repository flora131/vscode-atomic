# Notebook Renderers Architecture — File Location Report

## Summary

The `extensions/notebook-renderers/` directory contains VS Code's built-in notebook output rendering pipeline — the webview-side code responsible for displaying notebook cell outputs. This extension implements the **Notebook Renderer activation contract** (the `ActivationFunction` pattern) that defines how VS Code extensions can register custom output renderers for various MIME types.

**File count**: 23 files total  
**Primary language**: TypeScript  
**Key entry point**: `src/index.ts` (lines 419+) — exports `activate()` function

---

## Implementation Files

### Core Renderer Engine
- **`extensions/notebook-renderers/src/index.ts`** (640 lines)
  - Main extension entry point; exports `activate: ActivationFunction<void>`
  - Implements `renderOutputItem()` callback handler for 12+ MIME types (images, HTML, JavaScript, error, stdout/stderr streams, text)
  - Manages disposable lifecycle and renderer hooks (HTML rendering hooks, JavaScript pre-evaluation hooks)
  - Defines inline CSS styling for output containers, scrollable elements, word-wrap, error formatting
  - Handles image blob generation, HTML/JS sandbox rendering, stack trace error formatting
  - Manages stream output appending and scrollable container initialization

### Renderer Type Definitions & Contracts
- **`extensions/notebook-renderers/src/rendererTypes.ts`** (52 lines)
  - Exports `IDisposable` interface (dispose contract)
  - Exports `HtmlRenderingHook` interface — `postRender()` hook for HTML post-processing
  - Exports `JavaScriptRenderingHook` interface — `preEvaluate()` hook for script preprocessing
  - Exports `RenderOptions` interface (lineLimit, outputScrolling, outputWordWrap, linkifyFilePaths, minimalError)
  - Type alias `IRichRenderContext` — combines `RendererContext<void>` with settings and event handler
  - Type alias `OutputElementOptions` — configuration for output rendering
  - Extends `OutputWithAppend` interface from vscode-notebook-renderer

### Output Content Creation
- **`extensions/notebook-renderers/src/textHelper.ts`** (176 lines)
  - Exports `createOutputContent()` — renders text/plain, stdout, stderr with optional scrolling and truncation
  - Exports `appendOutput()` — appends streamed output to existing containers (supports scrollable append)
  - Exports `scrollableClass` constant = `'scrollable'`
  - Implements `truncatedArrayOfString()` — limits lines with "View as scrollable" link
  - Implements `scrollableArrayOfString()` — renders lines in scrollable container
  - Handles scroll position preservation across output updates
  - Manages `outputLengths` map for stream appending state

### ANSI Color & Formatting
- **`extensions/notebook-renderers/src/ansi.ts`** (422+ lines)
  - Exports `handleANSIOutput()` — parses ANSI escape codes and applies color/styling
  - Exports `calcANSI8bitColor()` — converts ANSI 8-bit color codes to RGBA
  - Processes escape sequences: SGR (Select Graphic Rendition), cursor movement, etc.
  - Applies text styles: bold, italic, strikethrough, underline with CSS classes
  - Integrates with `linkify()` for URL/path detection in styled output
  - Uses ANSI color map for terminal color palette mapping

- **`extensions/notebook-renderers/src/colorMap.ts`** (100+ lines)
  - Exports `ansiColorIdentifiers` array — ANSI standard color palette
  - Exports `ansiColorMap` object — indexed ANSI color mappings

- **`extensions/notebook-renderers/src/color.ts`** (1000+ lines)
  - Exports `CharCode` enum — ASCII code constants for parsing
  - Exports `RGBA` class — color value representation
  - Exports `HSLA` class — HSL color space
  - Exports `HSVA` class — HSV color space
  - Exports `Color` class — unified color representation with format converters
  - Provides color parsing and conversion utilities (RGB/hex/HSL conversions)

### Link Detection & File Path Parsing
- **`extensions/notebook-renderers/src/linkify.ts`** (209+ lines)
  - Exports `LinkDetector` class — detects web URLs, file paths, HTML anchors
  - Exports `linkify()` function — wraps links in `<a>` tags with proper escaping
  - Supports web links (http/https/www.), file paths (Windows absolute/relative, POSIX), HTML links
  - Configurable trust model: `trustHtml` flag controls HTML injection
  - Regex patterns: `WEB_LINK_REGEX`, `WIN_PATH`, `POSIX_PATH`, `LINE_COLUMN`
  - Max link length 2000 chars; detects platform (Windows vs POSIX)

### Stack Trace Formatting
- **`extensions/notebook-renderers/src/stackTraceHelper.ts`** (40+ lines)
  - Exports `formatStackTrace()` — parses error stack, linkifies IPython/Jupyter cell references
  - Removes conflicting ANSI color codes (background colors, specific foreground)
  - Detects IPython stack format; converts `Cell In[N], line X` references to VS Code notebook URIs
  - Returns formatted stack + error location link
  - Supports IPython 8.3.0+ and 9.0.0+ stack formats

### HTML Security Policy
- **`extensions/notebook-renderers/src/htmlHelper.ts`** (11 lines)
  - Exports `ttPolicy` — Trusted Types policy for HTML injection
  - Conditionally initializes Trusted Types policy if available in webview context
  - Provides safe `createHTML()` and `createScript()` methods

---

## Test Files

### Integration & Unit Tests
- **`extensions/notebook-renderers/src/test/notebookRenderer.test.ts`** (100+ lines)
  - Uses Mocha TDD framework with JSDOM for DOM emulation
  - Tests error rendering (stack trace formatting, minimal error display)
  - Tests stream output rendering (stdout/stderr appending)
  - Tests text output rendering (plain text with scrolling/truncation)
  - Tests settings propagation (outputWordWrap, outputScrolling changes)
  - Creates mock `RendererContext` with settings and event handlers

- **`extensions/notebook-renderers/src/test/linkify.test.ts`** (50+ lines)
  - Tests web link detection (www., http://, https://)
  - Tests HTML link preservation and escaping
  - Tests file path linkification (conditional on trustHtml flag)
  - Tests multi-line link processing

- **`extensions/notebook-renderers/src/test/stackTraceHelper.test.ts`** (50+ lines)
  - Tests non-IPython stack traces (left unchanged)
  - Tests IPython 8.3.6 stack linkification
  - Tests IPython 9.0.0+ stack linkification
  - Verifies cell references and file links in stack output

### Test Infrastructure
- **`extensions/notebook-renderers/src/test/index.ts`** (43 lines)
  - Mocha test runner configuration
  - Multi-environment support: Electron, browser, remote
  - Configures XML reporter for CI/CD (Azure Pipelines, GitHub Actions)
  - Sets test timeout to 60 seconds

---

## Configuration Files

### Build Configuration
- **`extensions/notebook-renderers/package.json`** (59 lines)
  - Package name: `builtin-notebook-renderers`
  - Version: 10.0.0
  - Publisher: `vscode`
  - Contributes single `notebookRenderer`:
    - ID: `vscode.builtin-renderer`
    - Entrypoint: `./renderer-out/index.js` (bundled output)
    - MIME types: images (gif, png, jpeg, svg), HTML, JavaScript, errors, stdout/stderr, text/plain
    - `requiresMessaging: "never"` — no main process communication needed
  - Build scripts: `compile`, `watch`, `build-notebook`
  - Dev dependencies: `@types/vscode-notebook-renderer`, `jsdom`, `@types/node`

- **`extensions/notebook-renderers/esbuild.notebook.mts`** (18 lines)
  - ESBuild configuration for notebook renderer bundling
  - Bundles `src/index.ts` to `renderer-out/index.js`
  - Uses shared `esbuild-webview-common.mts` configuration

### TypeScript Configuration
- **`extensions/notebook-renderers/tsconfig.json`** (23 lines)
  - Extends `../tsconfig.base.json`
  - Targets ES2024 + DOM lib
  - Output to `./out/` directory
  - Includes `vscode.d.ts` type definitions
  - Enables `skipLibCheck`

### Publishing & Distribution
- **`extensions/notebook-renderers/.vscodeignore`** (8 lines)
  - Excludes source TypeScript, tsconfig files, esbuild config
  - Only bundles `renderer-out/` and distributable assets

- **`extensions/notebook-renderers/.npmrc`** (3 lines)
  - `legacy-peer-deps=true`
  - 180-second timeout
  - 1-day min-release-age

- **`extensions/notebook-renderers/.gitignore`** — standard Node.js patterns

### Package Metadata
- **`extensions/notebook-renderers/package-lock.json`** — dependency lock file
- **`extensions/notebook-renderers/package.nls.json`** — localization strings for package.json

---

## Documentation

- **`extensions/notebook-renderers/README.md`** (10 lines)
  - Identifies this as a builtin bundled extension (cannot be uninstalled)
  - Lists provided renderers: image, HTML, JavaScript, error, stream, plain text

---

## Type Definitions & Interfaces

**From `rendererTypes.ts`** (exported types relied upon by index.ts):
- `IDisposable` — minimal cleanup contract
- `HtmlRenderingHook` — async post-render hook with signal support
- `JavaScriptRenderingHook` — async pre-evaluation hook
- `RenderOptions` — user settings object
- `IRichRenderContext` — unified renderer context with settings/events
- `OutputElementOptions` — rendering options per output item
- `OutputWithAppend` — extended OutputItem with optional appended text

**From vscode-notebook-renderer package**:
- `ActivationFunction<T>` — signature for extension activation
- `OutputItem` — notebook output data container
- `RendererContext<T>` — webview-side context object
- `RendererApi` — getRenderer() response type

---

## Notable Clusters

### Output Rendering Pipeline (index.ts)
- Lines 419–639: Main `activate()` function
- Lines 12–75: Image rendering (`renderImage()`, blob URL management)
- Lines 107–124: HTML rendering with hook system
- Lines 126–145: JavaScript rendering with pre-evaluation hooks
- Lines 167–224: Error rendering with stack trace formatting
- Lines 350–396: Stream output appending (stdout/stderr)
- Lines 398–417: Plain text output rendering

### ANSI/Color Processing Pipeline
- `ansi.ts` + `colorMap.ts` + `color.ts` — interconnected ANSI escape processing
- Handles SGR codes → CSS class mapping → RGBA color values
- Integration with `linkify()` for styled link detection

### Scrollable Output Management
- `textHelper.ts`: scroll position tracking, truncation UI, soft/hard limits
- `index.ts`: scroll initialization, event listeners (scroll, keydown)
- Preserves scroll state across output updates

---

## Entry Points & Activation Contracts

**Primary activation contract**:
```typescript
export const activate: ActivationFunction<void> = (ctx) => {
  // Returns object with:
  // - renderOutputItem(outputInfo, element, signal?)
  // - disposeOutputItem(id?)
  // - experimental_registerHtmlRenderingHook(hook)
  // - experimental_registerJavaScriptRenderingHook(hook)
}
```

**Supported MIME types registered in package.json**:
1. `image/gif`, `image/png`, `image/jpeg`, `image/svg+xml`
2. `text/html`
3. `application/javascript`
4. `application/vnd.code.notebook.error`
5. `application/vnd.code.notebook.stdout` / `stderr`
6. `application/x.notebook.stdout` / `stderr` / `stream`
7. `text/plain` (and all `text/*` variants)

---

## Architecture Insights for Tauri/Rust Porting

**Webview-side rendering only**: This extension runs entirely in the notebook output webview context. No main process communication (`requiresMessaging: "never"`). This is critical:
- No IPC bridges needed for output rendering
- Can be ported to pure JavaScript/Wasm in Tauri webview
- DOM manipulation via standard Web APIs (no VS Code-specific APIs)

**Key porting considerations**:

1. **Activation contract abstraction**: The `ActivationFunction` pattern would need a Rust/Wasm equivalent, likely a WASM module exporting `activate(ctx: RendererContext) → RendererCallbacks`

2. **Renderer context**: The `RendererContext<void>` object provides workspace trust state (`workspace.isTrusted`) and settings subscriptions. In Tauri, this would come from message passing to the main process.

3. **Disposable management**: Rust drop traits could replace the `IDisposable` interface; lifecycle management via RAII.

4. **Hook system**: The experimental HTML/JS rendering hooks allow post-processing. Would need event listener abstraction in Rust/Wasm.

5. **Styling**: 600+ lines of inline CSS in index.ts (lines 427–537) would translate directly to CSS files; CSS variables (`--notebook-*`) would need Tauri's CSS variable injection system.

6. **ANSI processing**: Complex regex-based ANSI escape parsing (ansi.ts) could benefit from Rust regex library or pure Wasm port.

7. **Security**: Trusted Types policy (`htmlHelper.ts`) — Tauri has its own HTML sanitization; would need equivalent in Rust/Wasm.

8. **Image handling**: Blob URL generation via `URL.createObjectURL()` — Tauri webview supports same API.

9. **Link detection**: RegEx patterns for path/URL detection are language-agnostic; can be ported directly.

10. **Settings subscriptions**: The `onDidChangeSettings` event pattern would need Tauri's event system or postMessage communication.

