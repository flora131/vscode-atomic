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
