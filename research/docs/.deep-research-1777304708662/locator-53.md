# VS Code Markdown Math Extension - File Locations for Tauri/Rust Porting Analysis

## Implementation

- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/src/extension.ts` (46 LOC) — Main extension entry point implementing VS Code Extension API. Exports `activate()` function that returns object with `extendMarkdownIt()` method. Loads `@vscode/markdown-it-katex` plugin, reads configuration from `markdown.math.*` settings, manages macros via workspace configuration, and registers configuration change listener.
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/notebook/katex.ts` (59 LOC) — Notebook renderer activation module for canvas-based rendering. Implements `activate(ctx: RendererContext<void>)` to extend markdown-it renderer. Handles CSS stylesheet injection into shadow DOM and configures KaTeX rendering with macro support for notebook contexts.

## Types / Interfaces

- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/tsconfig.json` — TypeScript configuration extending base config, targeting `./src`, outputs to `./out`. Includes VS Code type definitions from `vscode.d.ts`.
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/tsconfig.browser.json` — Browser-specific TypeScript configuration for bundling extension with browser platform target.
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/notebook/tsconfig.json` — TypeScript configuration for notebook renderer compilation.

## Configuration

- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/package.json` (125 LOC) — Extension manifest declaring:
  - Activation entry points: `./out/extension` (Node.js) and `./dist/browser/extension` (browser)
  - Grammar injections for markdown-math language with 4 TextMate syntax scopes
  - Notebook renderer registration (`vscode.markdown-it-katex-extension`) with entrypoint extending `vscode.markdown-it-renderer`
  - Configuration schema defining `markdown.math.enabled` (boolean, default true) and `markdown.math.macros` (object, resource scope)
  - Preview styles including KaTeX CSS stylesheet
  - Dependencies: `@vscode/markdown-it-katex@^1.1.2`
  - Capabilities: virtual workspaces and untrusted workspace support
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/package-lock.json` — Dependency lock file
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/package.nls.json` — Localization/translation strings

## Documentation

- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/README.md` (6 LOC) — Basic overview noting bundled status with VS Code, describes KaTeX math rendering for markdown preview and notebook cells.
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/cgmanifest.json` — Component governance manifest

## Examples / Fixtures

- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/preview-styles/index.css` — Custom CSS for markdown preview styling

## Notable Clusters

**Syntax Definition Files** (4 TextMate grammar files):
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/syntaxes/md-math.tmLanguage.json`
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/syntaxes/md-math-block.tmLanguage.json`
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/syntaxes/md-math-fence.tmLanguage.json`
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/syntaxes/md-math-inline.tmLanguage.json`

**Build Configuration** (3 esbuild scripts):
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/esbuild.mts` — Node.js platform bundling
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/esbuild.browser.mts` — Browser platform bundling
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/esbuild.notebook.mts` — Notebook renderer bundling

---

## Porting Analysis Summary

The markdown-math extension demonstrates VS Code's dual-platform extension architecture. The extension entry point (`extension.ts`) implements the VS Code Extension API via the activation hook pattern—it exports an `activate()` function returning an object with `extendMarkdownIt()` method. This is the core integration point with VS Code's markdown pipeline.

For Tauri/Rust porting, the critical dependencies are: (1) the markdown-it plugin hook mechanism (the `extendMarkdownIt(md: any)` contract), (2) the VS Code settings API (`vscode.workspace.getConfiguration()`), (3) command execution API (`vscode.commands.executeCommand()`), and (4) workspace configuration change notifications. The actual KaTeX rendering is delegated to the npm package `@vscode/markdown-it-katex`, which is a JavaScript/Node.js library. A Rust port would need to either wrap KaTeX via FFI or port/reimplement the math rendering logic. Additionally, the extension defines 4 TextMate syntax definitions (injected into markdown scope) and a notebook renderer using the `RendererContext` API—both abstractions that would require Rust-side equivalents in a Tauri environment.

