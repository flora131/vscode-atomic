# Locator 67: esbuild-webview-common.mts

## Implementation
- `extensions/esbuild-webview-common.mts` — Shared build configuration for webview bundling; exports `run()` function for esbuild configuration with browser/ESM target settings

## Examples / Notable References
- `extensions/simple-browser/esbuild.webview.mts` — Uses webview-common for simple-browser preview bundling
- `extensions/mermaid-chat-features/esbuild.webview.mts` — Uses webview-common for mermaid chat webview bundling
- `extensions/markdown-language-features/esbuild.webview.mts` — Uses webview-common for markdown preview webview
- `extensions/notebook-renderers/esbuild.notebook.mts` — Uses webview-common for notebook renderer bundling
- `extensions/markdown-math/esbuild.notebook.mts` — Uses webview-common for math notebook renderer bundling
- `extensions/ipynb/esbuild.notebook.mts` — Uses webview-common for ipynb notebook bundling

---

The `esbuild-webview-common.mts` module is a thin wrapper around `esbuild-common.mts` that provides standardized build configuration (ESM format, browser target, minification, source maps off, ES2024 target) for bundling extension webview scripts. It is imported by seven extension build scripts for consistent webview/notebook renderer bundling configuration.
