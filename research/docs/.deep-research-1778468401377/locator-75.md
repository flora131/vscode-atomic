# LaTeX Extension Grammar Porting Analysis

## Implementation

### Grammar Files
- `extensions/latex/syntaxes/TeX.tmLanguage.json` — Core TeX syntax definitions (376 lines)
- `extensions/latex/syntaxes/LaTeX.tmLanguage.json` — Extended LaTeX syntax definitions (4446 lines)
- `extensions/latex/syntaxes/Bibtex.tmLanguage.json` — BibTeX syntax definitions (341 lines)
- `extensions/latex/syntaxes/markdown-latex-combined.tmLanguage.json` — Combined Markdown/LaTeX syntax (3276 lines)
- `extensions/latex/syntaxes/cpp-grammar-bailout.tmLanguage.json` — C++ embedded in LaTeX (20054 lines)

### Grammar Builder
- `extensions/latex/build/update-grammars.js` — Automated grammar update script that pulls from upstream jlelong/vscode-latex-basics repository

### Language Configurations
- `extensions/latex/latex-language-configuration.json` — TeX/LaTeX language settings (120 lines)
- `extensions/latex/latex-cpp-embedded-language-configuration.json` — C++ embedded language configuration (33 lines)
- `extensions/latex/markdown-latex-combined-language-configuration.json` — Markdown+LaTeX combined configuration (126 lines)

## Configuration

- `extensions/latex/package.json` — Extension manifest defining language contributions, grammars, and scripts (120 lines)
- `extensions/latex/.vscodeignore` — Files to exclude from package
- `extensions/latex/cgmanifest.json` — Component governance manifest (37 lines)

## Documentation

- `extensions/latex/markdown-latex-combined-license.txt` — License for combined markdown grammar
- `extensions/latex/cpp-bailout-license.txt` — License for C++ bailout grammar
- `extensions/latex/package.nls.json` — Localization strings (4 lines)

## Notable Clusters

The LaTeX extension is a grammar-only extension with no TypeScript/JavaScript runtime code. It provides TextMate grammar definitions for five language variants via the VSCode grammar contribution system. The core contribution mechanism is declarative configuration in `package.json` that references TextMate `.tmLanguage.json` files. An automated build script (`update-grammars.js`) maintains synchronization with the upstream jlelong/vscode-latex-basics repository.

Porting this to Tauri/Rust would require translating TextMate format grammars into Rust-based syntax highlighting (e.g., tree-sitter or comparable Rust grammar framework), replacing the declarative VSCode grammar registration with equivalent Rust plugin API bindings, and maintaining the grammar update pipeline in a Rust context.
