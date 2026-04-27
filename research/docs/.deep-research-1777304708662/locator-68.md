# C++ Language Extension (extensions/cpp/)

## Overview
Built-in VS Code extension providing syntax highlighting, language configuration, snippets, and problem matchers for C, C++, and CUDA C++. Entirely declarative—no runtime logic.

### Implementation
- `extensions/cpp/package.json` — Extension manifest defining 3 language contributions (C/C++/CUDA), 5 grammar scopes, snippets, and NVIDIA CUDA problem matchers
- `extensions/cpp/language-configuration.json` — Language settings for bracket matching, auto-closing pairs, indentation rules, and on-enter formatting behavior
- `extensions/cpp/build/update-grammars.js` — Build script pulling grammar definitions from external repos (jeff-hykin, NVIDIA, textmate); C++ grammars frozen due to license compatibility

### Syntax/Snippets
- `extensions/cpp/syntaxes/c.tmLanguage.json` — TextMate grammar for C syntax highlighting
- `extensions/cpp/syntaxes/cpp.tmLanguage.json` — TextMate grammar for C++ syntax highlighting
- `extensions/cpp/syntaxes/cpp.embedded.macro.tmLanguage.json` — TextMate grammar for C++ macros
- `extensions/cpp/syntaxes/cuda-cpp.tmLanguage.json` — TextMate grammar for CUDA C++ syntax
- `extensions/cpp/syntaxes/platform.tmLanguage.json` — Platform-specific C syntax
- `extensions/cpp/snippets/c.code-snippets` — C language code snippets
- `extensions/cpp/snippets/cpp.code-snippets` — C++ language code snippets

### Configuration
- `extensions/cpp/package.nls.json` — Localization strings for display names and descriptions
- `extensions/cpp/.vscodeignore` — Files excluded from extension packaging
- `extensions/cpp/cgmanifest.json` — Component governance manifest tracking dependencies

## Summary

The C++ extension is a minimal, declarative declarative language support module with no runtime code. It contributes language identifiers and file associations for C, C++, and CUDA; five TextMate grammars for syntax highlighting; editor configuration for bracket matching and indentation; NVIDIA problem matchers for CUDA compilation errors; and code snippets. All grammar files are generated externally and updated via a build script, making this extension a thin packaging layer for lexical analysis and IDE feature declarations—no semantic analysis, language server, or debugging functionality. For IDE functionality porting, this extension demonstrates that syntax/snippet support requires only grammar definitions and language metadata, not language-specific runtime logic.

