# LaTeX Extension - File Locations

## Overview
The `extensions/latex/` directory is a pure grammar and snippet extension (no TypeScript/JavaScript implementation code) that provides syntax highlighting, language configuration, and grammar definitions for LaTeX, TeX, and BibTeX file types.

## Implementation
- `extensions/latex/package.json` - Extension manifest defining language contributions, grammar rules, and build scripts
- `extensions/latex/build/update-grammars.js` - Build utility for updating TextMate grammar files

## Configuration
- `extensions/latex/latex-language-configuration.json` - Editor behavior for LaTeX/TeX (bracket matching, indentation, folding, word pattern rules, auto-closing pairs for LaTeX constructs)
- `extensions/latex/latex-cpp-embedded-language-configuration.json` - Editor behavior for C++ embedded in LaTeX (pragma-based folding regions)
- `extensions/latex/markdown-latex-combined-language-configuration.json` - Combined Markdown + LaTeX language configuration
- `extensions/latex/.vscodeignore` - Build packaging ignore rules

## Grammar Definitions (TextMate Format)
- `extensions/latex/syntaxes/LaTeX.tmLanguage.json` (4,446 LOC) - Main LaTeX syntax grammar with embedded language support for C++, JavaScript, TypeScript, Python, Java, Ruby, Lua, Julia, YAML, XML, CSS, HTML
- `extensions/latex/syntaxes/TeX.tmLanguage.json` (376 LOC) - TeX syntax grammar
- `extensions/latex/syntaxes/Bibtex.tmLanguage.json` (341 LOC) - BibTeX bibliography syntax grammar
- `extensions/latex/syntaxes/markdown-latex-combined.tmLanguage.json` (3,276 LOC) - Combined Markdown + LaTeX syntax for code blocks
- `extensions/latex/syntaxes/cpp-grammar-bailout.tmLanguage.json` (20,054 LOC) - C++ embedded grammar fallback for LaTeX document mode

## License & Attribution
- `extensions/latex/cpp-bailout-license.txt` - License for C++ grammar bailout component
- `extensions/latex/markdown-latex-combined-license.txt` - License for markdown-latex-combined grammar

## Metadata
- `extensions/latex/cgmanifest.json` - Component governance manifest
- `extensions/latex/package.nls.json` - Localization strings for UI text (displayName, description)

## Notable Clusters
**Grammar System**: The extension demonstrates complex TextMate grammar composition with:
- Recursive embedding of 10+ programming languages within LaTeX documents
- Advanced bracket matching rules for LaTeX mathematical delimiters (unbalanced scope handling for constructs like `\left(` and `\right)`)
- Folding region markers for both LaTeX (`%region`/`%endregion`, `\begingroup`/`\endgroup`) and C++ (`#pragma region`/`#pragma endregion`)
- Grammar bailout mechanism using a large C++ grammar (20K LOC) as fallback

## Relevance to Tauri/Rust Porting Research
This extension is **grammar-only** with no custom IDE logic, language server, or Electron-specific APIs. Porting implications:
- TextMate grammar files (JSON-based) are platform-agnostic and would require no rewrite
- Language configuration JSON is consumed by the core editor and would transfer directly
- The `update-grammars.js` build script is a simple utility that could be rewritten in Rust if needed
- No dependency on TypeScript runtime, VS Code extension API, or Electron-specific features beyond grammar registration
- Most portable extension type in VS Code's ecosystem (pure declarative syntax definitions)

