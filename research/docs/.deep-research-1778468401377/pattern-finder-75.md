# Pattern Research: VS Code LaTeX Extension Port to Tauri/Rust

## Scope
`extensions/latex/` — Grammar and language configuration only (1 LOC file with build script)

## Sentinel Note
This extension partition contains **only grammar/language metadata and build configuration files**. No TypeScript implementation code is present in this scope to analyze for porting patterns.

## Files Examined

### Build Configuration
**Found in**: `extensions/latex/build/update-grammars.js:1-14`

```javascript
var updateGrammar = require('vscode-grammar-updater');

updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/Bibtex.tmLanguage.json', 'syntaxes/Bibtex.tmLanguage.json', undefined, 'main');
updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/LaTeX.tmLanguage.json', 'syntaxes/LaTeX.tmLanguage.json', undefined, 'main');
updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/TeX.tmLanguage.json', 'syntaxes/TeX.tmLanguage.json', undefined, 'main');
updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/cpp-grammar-bailout.tmLanguage.json', 'syntaxes/cpp-grammar-bailout.tmLanguage.json', undefined, 'main');
updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/markdown-latex-combined.tmLanguage.json', 'syntaxes/markdown-latex-combined.tmLanguage.json', undefined, 'main');
```

This is a build script that pulls grammar definitions from an external repository (`jlelong/vscode-latex-basics`).

### Extension Manifest
**Found in**: `extensions/latex/package.json:1-121`

The manifest declares language contributions and grammar scopes:

- **Languages declared**: `tex`, `latex`, `bibtex`, `cpp_embedded_latex`, `markdown_latex_combined`
- **Grammar entries**: 5 grammar definitions with scope names and embedded language support
- **Configuration references**: Language configuration files defining bracket matching, auto-closing pairs, indentation rules, and folding markers

### Language Configuration
**Found in**: `extensions/latex/latex-language-configuration.json:1-120`

This JSON file defines IDE language support features:
- **Comments**: Line comment syntax (`%`)
- **Bracket pairs**: 70+ bracket configurations including LaTeX-specific constructs (`\left(`, `\right)`, `\bigl[`, etc.)
- **Auto-closing pairs**: 26 pairs for bracket completion
- **Surrounding pairs**: Quotes, braces, and math delimiters
- **Indentation rules**: Regex patterns for `\begin{}`/`\end{}` blocks
- **Folding markers**: Region directives and `\begingroup`/`\endgroup`
- **Word pattern**: Unicode-aware regex for identifier matching

## Key Observations for Porting

The LaTeX extension demonstrates how VS Code manages **TextMate grammar** and **language configuration** entirely through:

1. **Declarative metadata** (package.json, JSON configuration files)
2. **External grammar sources** (pulled from GitHub via build script)
3. **Scope-based syntax highlighting** (TextMate scope names)
4. **Structured language features** (bracket matching, indentation, folding)

For a Tauri/Rust port, these patterns would translate to:
- **Grammar system**: Grammar definitions would remain JSON or convert to a Rust-native format (tree-sitter, Ropey, or custom)
- **Language configuration**: Could be embedded as Rust structs or remain as JSON deserialized at startup
- **Build process**: Node.js grammar-updater would be replaced with Rust tooling or direct API integration
- **Manifest system**: Extension metadata would map to Rust trait implementations or TOML configuration

## Files Analyzed
- `/home/norinlavaee/projects/vscode-atomic/extensions/latex/build/update-grammars.js`
- `/home/norinlavaee/projects/vscode-atomic/extensions/latex/package.json`
- `/home/norinlavaee/projects/vscode-atomic/extensions/latex/latex-language-configuration.json`

The LaTeX extension in this partition is a **pure declarative grammar/language extension** with no runtime TypeScript logic, serving as a template for how syntactic and semantic language features can be separated from implementation. A Tauri port would benefit from a similar separation, where language metadata is decoupled from the core editor runtime.
