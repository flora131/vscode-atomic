# LaTeX Grammar Extension - File Locations

## Overview
The `extensions/latex/` directory contains grammar definitions and language support for LaTeX, TeX, and BibTeX file formats. This is a language support extension with syntax highlighting grammars (TextMate format).

## Grammar Files
- `extensions/latex/syntaxes/LaTeX.tmLanguage.json` - Primary LaTeX syntax grammar
- `extensions/latex/syntaxes/TeX.tmLanguage.json` - TeX syntax grammar
- `extensions/latex/syntaxes/Bibtex.tmLanguage.json` - BibTeX bibliography syntax grammar
- `extensions/latex/syntaxes/markdown-latex-combined.tmLanguage.json` - Combined markdown with LaTeX
- `extensions/latex/syntaxes/cpp-grammar-bailout.tmLanguage.json` - C++ embedded in LaTeX fallback grammar

## Configuration Files
- `extensions/latex/package.json` - Extension manifest, defines language contributions and grammar registrations
- `extensions/latex/latex-language-configuration.json` - Language configuration for tex/latex file types
- `extensions/latex/latex-cpp-embedded-language-configuration.json` - Configuration for C++ embedded language
- `extensions/latex/markdown-latex-combined-language-configuration.json` - Configuration for markdown-latex combined mode

## Build & Maintenance
- `extensions/latex/build/update-grammars.js` - Script to update/refresh grammar definitions

## Metadata & Licensing
- `extensions/latex/package.nls.json` - Localization strings
- `extensions/latex/.vscodeignore` - Files excluded from VS Code packaging
- `extensions/latex/cgmanifest.json` - Component governance manifest
- `extensions/latex/markdown-latex-combined-license.txt` - License for markdown-latex component
- `extensions/latex/cpp-bailout-license.txt` - License for C++ bailout grammar

## Scope Summary
This extension provides syntax highlighting support for LaTeX/TeX/BibTeX. It consists primarily of TextMate grammar files (JSON format) with no executable code. The extension registers 5 language modes and 5 corresponding syntax grammars with VS Code's language contribution system.

The embedded languages feature within the LaTeX grammar allows syntax highlighting for embedded code blocks (C++, Python, JavaScript, etc.) within LaTeX documents.
