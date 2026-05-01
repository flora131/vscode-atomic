## File Locations for markdown-math Extension (Markdown-it Plugin Hook)

### Implementation

- `extensions/markdown-math/src/extension.ts` — Main extension entry point implementing `extendMarkdownIt()` API hook; configures KaTeX plugin with user settings (macros, fence blocks, global groups) and listens for configuration changes
- `extensions/markdown-math/notebook/katex.ts` — Webview-side notebook renderer activation; loads KaTeX styles into shadow DOM and extends markdown-it renderer via the vscode.markdown-it-renderer context

### Configuration

- `extensions/markdown-math/package.json` — Extension manifest defining entry points (`main`, `browser`), marketplace metadata, contributes markdown-it plugin hook declaration (`markdown.markdownItPlugins`), notebook renderer entry (`notebookRenderer` with `vscode.markdown-it-katex-extension`), and user-configurable settings (`markdown.math.enabled`, `markdown.math.macros`)
- `extensions/markdown-math/tsconfig.json` — TypeScript compiler configuration extending base config, targeting src/ to out/ compilation

### Build Configuration

- `extensions/markdown-math/esbuild.notebook.mts` — ESbuild configuration for notebook bundle; transpiles notebook/katex.ts to notebook-out/, copies KaTeX CSS and WOFF2 fonts from node_modules

### Syntax Definitions

- `extensions/markdown-math/syntaxes/md-math.tmLanguage.json` — TextMate language grammar for markdown-math
- `extensions/markdown-math/syntaxes/md-math-block.tmLanguage.json` — Injected grammar for block-level math syntax in markdown
- `extensions/markdown-math/syntaxes/md-math-inline.tmLanguage.json` — Injected grammar for inline math syntax in markdown
- `extensions/markdown-math/syntaxes/md-math-fence.tmLanguage.json` — Injected grammar for fenced code block math syntax

### Styling

- `extensions/markdown-math/preview-styles/index.css` — Custom CSS for markdown preview rendering
- `extensions/markdown-math/notebook-out/katex.min.css` — Compiled KaTeX stylesheet (bundled from dependencies)

### Documentation

- `extensions/markdown-math/README.md` — Brief description noting the extension ships bundled with VS Code and adds KaTeX-based math rendering to markdown preview and notebook cells

### Root Configuration

- `extensions/markdown-math/package-lock.json` — Dependency lock file
- `extensions/markdown-math/.npmrc` — NPM configuration
- `extensions/markdown-math/.vscodeignore` — Files excluded from VSIX packaging
- `extensions/markdown-math/.gitignore` — Git ignore rules
- `extensions/markdown-math/cgmanifest.json` — Component governance manifest
- `extensions/markdown-math/icon.png` — Extension icon

---

The markdown-math extension implements the `extendMarkdownIt()` plugin hook across two surfaces: the main extension process handles configuration-driven KaTeX plugin initialization with user-defined LaTeX macros and dynamic reloading, while the notebook renderer activates separately in the webview context to inject KaTeX styles into shadow DOM and extend the markdown-it renderer. Porting this to Tauri/Rust would require bridging the extension API (currently accessed via TypeScript's `vscode` module) to a Rust backend, translating the markdown-it plugin architecture to a Rust markdown processor, managing CSS/font asset distribution in a webview context without Electron's native module loading, and reimplementing the configuration watch and hot-reload pattern. The dual-surface activation pattern (extension + notebook renderer) suggests the Tauri port would need similar separation between a backend configuration handler and a frontend webview renderer component.

