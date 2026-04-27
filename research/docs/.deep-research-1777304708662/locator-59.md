# File Locations for esbuild-webview-common Module

## Implementation

- `extensions/esbuild-webview-common.mts` — Shared esbuild build configuration for webview content (82 lines, TypeScript/Node.js)

## Related Build Infrastructure

- `extensions/esbuild-extension-common.mts` — Shared esbuild build configuration for extensions (102 lines, TypeScript/Node.js)

## Consumer Files

The following extension build scripts import and use `esbuild-webview-common.mts`:

- `extensions/simple-browser/esbuild.webview.mts` — Builds simple browser extension preview webview
- `extensions/markdown-language-features/esbuild.webview.mts` — Builds markdown preview webview  
- `extensions/mermaid-chat-features/esbuild.webview.mts` — Builds mermaid chat webview
- `extensions/notebook-renderers/esbuild.notebook.mts` — Builds notebook renderer webview (imports relative path)
- `extensions/markdown-language-features/esbuild.notebook.mts` — Builds notebook content webview
- `extensions/markdown-math/esbuild.notebook.mts` — Builds math notebook webview
- `extensions/ipynb/esbuild.notebook.mts` — Builds Jupyter notebook webview

## Configuration & Build Context

- `extensions/package.json` — Root extensions package manifest
- Various extension-specific `package.json` files (for simple-browser, markdown-language-features, mermaid-chat-features, notebook-renderers, markdown-math, ipynb) define build scripts that invoke esbuild

## Documentation

- `extensions/simple-browser/README.md` — Simple browser extension docs
- `extensions/markdown-language-features/README.md` — Markdown extension docs
- `extensions/mermaid-chat-features/README.md` — Mermaid chat extension docs
- `extensions/notebook-renderers/README.md` — Notebook renderer docs
- `extensions/markdown-math/README.md` — Markdown math extension docs
- `extensions/ipynb/README.md` — Jupyter notebook extension docs

## Key Implementation Details

The `esbuild-webview-common.mts` module exports:

- `BuildOptions` type — Partial esbuild.BuildOptions with required entryPoints and outdir
- `run()` function — Main build orchestrator that accepts:
  - config object with srcDir, outdir, entryPoints, optional additionalOptions
  - process args array for CLI flags (--watch, --outputRoot)
  - optional didBuild callback function
  
Build behavior:
- Bundles and minifies code for ESM format targeting ES2024
- Targets browser platform
- Supports watch mode with plugin-based post-build callbacks
- Allows output directory override via --outputRoot CLI flag
- Treats import-is-undefined as error to catch missing dependencies

## Notable Clusters

**Webview Build Pipeline:**
- Consumer files share common pattern: import from ../esbuild-webview-common.mts, define srcDir/outDir paths, provide entryPoints config, invoke run() with process.argv
- Each webview builder optionally adds loader config (e.g., .ttf as dataurl) via additionalOptions
- All webview builders follow identical architectural pattern for code splitting and optimization

**Build Infrastructure Organization:**
- Two sibling build modules: esbuild-extension-common.mts (for Node platform extensions) and esbuild-webview-common.mts (for browser platform webviews)
- esbuild-extension-common differs in platform/format configuration and adds vscode external dependency
- Both use identical watch/build orchestration logic with didBuild callback pattern

---

## Summary

The `esbuild-webview-common.mts` module is a build-time utility providing reusable esbuild configuration for VS Code extension webviews. It resides as a single TypeScript/ESM file at the extensions directory root level and is imported by 7 extension build scripts that compile webview content (markdown previews, notebook renderers, chat interfaces, browser content). The module exports a type-safe `run()` function that standardizes browser-targeted bundling, minification, and optional post-build callbacks across all webview-based extensions. No tests or specialized fixtures exist for this module; it operates as a straightforward build helper invoked at development/packaging time.
