# Locator-70: extensions/less/

## Scope
Less language grammar/snippet contribution extension (1 file, 19 LOC direct contribution).

### Implementation
- `extensions/less/package.json` — Extension manifest defining Less language support with ID "less", aliases, file extensions (.less), MIME types (text/x-less, text/less), language configuration reference, grammar declaration, and lessc compiler problem matcher pattern
- `extensions/less/language-configuration.json` — Language configuration providing block/line comment markers, bracket pairs, auto-closing pairs, surrounding pairs, folding markers (#region/#endregion), indentation rules, word patterns for Less identifiers and properties, and on-enter rules for line comment continuation
- `extensions/less/syntaxes/less.tmLanguage.json` — TextMate grammar syntax file (converted from radium-v/Better-Less repository) defining Less scope name "source.css.less" with patterns for comments, namespace accessors, extend syntax, at-rules, variable assignments, property lists, and selectors; includes repository with detailed capture groups for angle types, arbitrary repetition, and other Less-specific syntax elements

### Configuration
- `extensions/less/package.nls.json` — Localization file with English strings: "Less Language Basics" (displayName) and syntax highlighting description
- `extensions/less/.vscodeignore` — Packager ignore patterns excluding test/, cgmanifest.json, build/
- `extensions/less/cgmanifest.json` — Component governance manifest (file listed in glob results, content not examined)

### Examples / Build Utilities
- `extensions/less/build/update-grammar.js` — Async grammar update script using vscode-grammar-updater to pull latest Better-Less TextMate grammar from radium-v/Better-Less master branch, converts grammar name and scopeName properties, outputs to syntaxes/less.tmLanguage.json

## Summary

The `extensions/less/` partition contains a self-contained VS Code language extension for Less CSS preprocessing language support. The extension registers a language ID, file association, and syntax highlighting grammar. The package.json declares Less as a programming language category extension with contributions for language definition, TextMate grammar, and a lessc compiler problem matcher for error reporting. Language configuration provides editor behaviors (bracket matching, indentation, comment handling). The syntax grammar is maintained in external repository (Better-Less) with an update script managing synchronization. Localization strings support internationalization of extension metadata.

For porting VS Code core IDE functionality to Tauri/Rust, this partition demonstrates the language extension architecture pattern: metadata-driven registration, grammar-based syntax highlighting, and problem matcher patterns for toolchain integration. The decoupled grammar source (external Better-Less repo) shows how language support can be modularized and maintained separately.

