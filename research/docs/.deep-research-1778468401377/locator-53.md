# Locator Results: extensions/markdown-math

## Scope Assessment

This extension is a specialized markdown math rendering extension that adds syntax highlighting and KaTeX-based math rendering to Markdown files. It has no relevance to core IDE functionality porting.

## Configuration

- `extensions/markdown-math/package.json` — Extension manifest with markdown plugin configuration
- `extensions/markdown-math/tsconfig.json` — TypeScript configuration
- `extensions/markdown-math/tsconfig.browser.json` — Browser build configuration
- `extensions/markdown-math/tsconfig.notebook.json` — Notebook renderer configuration (nested in notebook/)
- `extensions/markdown-math/.npmrc` — NPM configuration
- `extensions/markdown-math/.vscodeignore` — VS Code ignore file
- `extensions/markdown-math/.gitignore` — Git ignore file
- `extensions/markdown-math/cgmanifest.json` — Component manifest
- `extensions/markdown-math/package-lock.json` — Lock file

## Implementation

- `extensions/markdown-math/src/extension.ts` — Main extension entry point (46 LOC)
- `extensions/markdown-math/notebook/katex.ts` — Notebook renderer for KaTeX
- `extensions/markdown-math/esbuild.mts` — Main esbuild configuration
- `extensions/markdown-math/esbuild.browser.mts` — Browser build configuration
- `extensions/markdown-math/esbuild.notebook.mts` — Notebook build configuration

## Documentation

- `extensions/markdown-math/README.md` — Extension documentation
- `extensions/markdown-math/package.nls.json` — Localization strings

## Syntax & Styles

- `extensions/markdown-math/syntaxes/md-math.tmLanguage.json` — Main math language grammar
- `extensions/markdown-math/syntaxes/md-math-block.tmLanguage.json` — Block math grammar
- `extensions/markdown-math/syntaxes/md-math-inline.tmLanguage.json` — Inline math grammar
- `extensions/markdown-math/syntaxes/md-math-fence.tmLanguage.json` — Fenced code block math grammar
- `extensions/markdown-math/preview-styles/index.css` — Math preview styles

## Other

- `extensions/markdown-math/icon.png` — Extension icon

## Notable Clusters

- `extensions/markdown-math/syntaxes/` — Contains 4 TextMate grammar files for math syntax highlighting
- `extensions/markdown-math/notebook/` — Contains notebook renderer configuration

---

**Verdict:** This extension provides only markdown math rendering via KaTeX and syntax highlighting. It contains no core IDE functionality (editors, terminals, workspaces, debugging, language services, etc.) relevant to porting VS Code's core IDE to Tauri/Rust. Confirmed as non-relevant to the research question.
