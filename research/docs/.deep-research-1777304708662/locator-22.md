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
