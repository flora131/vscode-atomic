# Online Research: markdown-math / KaTeX in a Tauri/Rust Port

**Scope:** `extensions/markdown-math/` (5 source files, ~177 LOC)
**Dependency:** `@vscode/markdown-it-katex@^1.1.2`
**Research date:** 2026-04-27

---

## Summary

The `markdown-math` extension is a thin, renderer-side plugin that delegates all mathematical typesetting to KaTeX via the `@vscode/markdown-it-katex` markdown-it plugin. Its own code is trivially small: `src/extension.ts` simply registers the plugin with VS Code's markdown-it pipeline and surfaces two configuration settings (`markdown.math.enabled`, `markdown.math.macros`); `notebook/katex.ts` injects the KaTeX CSS into the notebook renderer's shadow DOM and extends the notebook's `vscode.markdown-it-renderer` in the same way.

Because KaTeX is a pure JavaScript library that renders LaTeX to HTML+CSS, the critical question for a Tauri/Rust port is not whether KaTeX can be rewritten in Rust, but how KaTeX's existing JS bundle is hosted and invoked in Tauri's webview.

---

## Detailed Findings

### 1. KaTeX is a JS-only, browser-native library

**Source:** https://katex.org/docs/browser

KaTeX ships as a self-contained JavaScript + CSS bundle. Its own documentation reads:

> "KaTeX supports all major browsers, including Chrome, Safari, Firefox, Opera, and Edge."

The library has no native binary component; rendering happens entirely in the browser's JavaScript engine. In a VS Code Electron context, KaTeX runs inside Electron's Chromium-based webview without any special ceremony. In a Tauri context it would run inside Tauri's webview (WKWebView on macOS, WebView2 on Windows, WebKitGTK on Linux) under identical conditions, because all three are standards-compliant browsers that execute JavaScript and CSS normally.

This means KaTeX itself requires **no porting work**. The `katex.min.js` and `katex.min.css` assets bundled by `@vscode/markdown-it-katex` can be loaded into a Tauri webview with no modification.

### 2. The markdown-it plugin layer (`@vscode/markdown-it-katex`) is also pure JS

The npm package `@vscode/markdown-it-katex` is a thin wrapper that:
- Parses `$...$` (inline) and `$$...$$` / fenced `math` blocks (block) using markdown-it's rule/core pipeline
- Calls `katex.renderToString()` for each matched token
- Returns the resulting HTML string back into the markdown-it output

All of this is standard Node.js/browser JavaScript. It does not use any VS Code SDK API, Electron API, or Node.js native addon. It is a pure-JS npm package that can run in any JavaScript environment — including the JS engine embedded in Tauri's webview.

### 3. What actually binds the extension to VS Code's extension host

The non-portable pieces in `src/extension.ts` are limited to three VS Code extension-host APIs:

- `vscode.workspace.getConfiguration(...)` — reads user settings
- `vscode.workspace.onDidChangeConfiguration(...)` — listens for setting changes
- `vscode.commands.executeCommand('markdown.api.reloadPlugins')` — tells the markdown preview to re-render

And the `extendMarkdownIt(md)` callback, which is a VS Code-specific protocol for the built-in markdown extension to collect third-party markdown-it plugins.

None of these have a Tauri equivalent; they are all VS Code extension-host IPC calls. The analog in Tauri would be:

- Configuration: Tauri's `tauri::plugin::Builder` or a JSON config file read via `tauri::api::path`; or simply a Tauri command exposed to the frontend that returns the user's math settings.
- Re-render trigger: a Tauri event emitted from Rust to the webview (`window.emit(...)` / `appHandle.emit_to(...)`), or a frontend-only reactive state update.
- `extendMarkdownIt`: replaced by directly initializing markdown-it with the KaTeX plugin in the webview's JS bundle (no plugin protocol needed — the frontend owns its own markdown-it instance).

### 4. Notebook renderer (`notebook/katex.ts`) maps to Tauri's webview injection

The notebook renderer code is more tightly coupled to VS Code's notebook renderer API (`vscode-notebook-renderer`, `RendererContext`, `ctx.getRenderer('vscode.markdown-it-renderer')`). These are VS Code-specific interfaces for the notebook output renderer sandbox.

In a Tauri application there is no notebook renderer protocol, but the functional goal — loading KaTeX CSS into a sandboxed shadow DOM and applying the markdown-it-katex plugin — can be achieved by:

1. Bundling `katex.min.css` and `katex.min.js` as Tauri static assets (via the `distDir` or custom protocol handler).
2. Injecting a `<link>` tag for the stylesheet and initializing markdown-it with the katex plugin inside the webview's JavaScript init code.
3. If notebook-style cell isolation is needed, using Shadow DOM directly in frontend JS — Tauri does not constrain this.

The Chromium bug referenced in `notebook/katex.ts` (https://bugs.chromium.org/p/chromium/issues/detail?id=336876, about font loading inside shadow DOM) is a browser-level issue. Tauri's WebView2 and WKWebView may or may not reproduce it; this would need empirical testing per platform.

### 5. Tauri webview compatibility with KaTeX

Tauri's architecture (from `https://v2.tauri.app/llms.txt` / concept docs) uses the OS's built-in webview:

- macOS/iOS: WKWebView (WebKit)
- Windows: WebView2 (Chromium-based)
- Linux: WebKitGTK

KaTeX supports all of these (Chrome, Safari, Firefox, Opera, Edge per KaTeX docs). WKWebView (Safari engine) can occasionally lag on CSS features; KaTeX's font loading via `@font-face` and its use of CSS `display: contents` etc. are generally well-supported in modern WebKit, but the shadow-DOM font-loading workaround already present in `notebook/katex.ts` may still be relevant.

### 6. Alternative: server-side (Rust-side) math rendering

Rather than running KaTeX in the webview, a Tauri application could render LaTeX to SVG or HTML server-side in Rust. The main Rust option is the `latex2mathml` crate (limited scope) or calling KaTeX via a JS runtime embedded in Rust:

- **`deno_core` / `v8`**: Execute KaTeX's `renderToString()` in a V8 isolate from Rust, return the HTML string to the frontend as plain HTML. This avoids shipping KaTeX JS to the webview but adds significant complexity and binary size.
- **MathJax / `mathjax-node`**: Similar approach via a Node.js sidecar.
- **`typst`**: A Rust-native typesetting system that supports math; not LaTeX-compatible but a full Rust alternative for new projects.

For a port of the existing VS Code `markdown-math` extension, the in-webview approach (simply reusing KaTeX JS) is by far the lowest-effort path and has no meaningful downside.

---

## Port Complexity Assessment

| Component | VS Code mechanism | Tauri equivalent | Effort |
|---|---|---|---|
| KaTeX rendering | `@vscode/markdown-it-katex` npm pkg in webview | Same npm pkg, loaded in Tauri webview JS bundle | Trivial — zero changes to KaTeX |
| markdown-it plugin registration | `extendMarkdownIt()` extension-host protocol | Direct `md.use(katex, opts)` call in frontend JS init | Low |
| Settings read (`math.enabled`, `math.macros`) | `vscode.workspace.getConfiguration` | Tauri command or config file read from Rust side | Low |
| Settings change event | `onDidChangeConfiguration` | Tauri event from backend, or frontend reactive state | Low |
| Re-render trigger | `markdown.api.reloadPlugins` command | Frontend event / reactive update | Low |
| Notebook renderer | `vscode-notebook-renderer` API | Custom webview component in frontend JS | Medium (protocol re-design needed only if notebooks are in scope) |
| KaTeX CSS / font assets | Bundled in extension directory, served by VS Code | Bundled in Tauri `distDir`, served via custom protocol | Low |
| Shadow DOM font-loading workaround | Chromium bug workaround in notebook renderer | Needs per-platform testing on WKWebView/WebView2 | Low–medium |

**Overall:** The `markdown-math` extension is one of the easiest components to port. Its rendering logic is entirely JS-in-webview and carries across to Tauri without modification. The only porting work is replacing the VS Code extension-host API calls (settings, events, plugin registration) with Tauri-native equivalents, and those APIs are simple enough that the total rewrite is measured in tens of lines, not hundreds.

---

## Additional Resources

- KaTeX browser docs: https://katex.org/docs/browser
- `@vscode/markdown-it-katex` npm package: https://www.npmjs.com/package/@vscode/markdown-it-katex
- Tauri architecture overview: https://v2.tauri.app/concept/architecture
- Tauri IPC (calling frontend from Rust): https://v2.tauri.app/develop/calling-frontend
- Tauri static asset embedding: https://v2.tauri.app/develop/resources
- `typst` Rust math typesetting alternative: https://github.com/typst/typst

---

## Gaps or Limitations

- No empirical testing was performed to verify KaTeX font loading behavior inside Tauri's WKWebView shadow DOM specifically; the Chromium-specific workaround in `notebook/katex.ts` may or may not transfer to WebKit cleanly.
- The scope of "notebook support" in a Tauri port is undefined; if notebooks (`.ipynb`) are not in scope, the `notebook/katex.ts` renderer can be ignored entirely, eliminating the medium-effort item above.
- `@vscode/markdown-it-katex` is a Microsoft-maintained fork of `markdown-it-katex`. A pure-upstream alternative (the community `markdown-it-katex` or `@iktakahiro/markdown-it-katex`) could be used instead if the VS Code-specific fork is not desired in a non-VS-Code codebase.
