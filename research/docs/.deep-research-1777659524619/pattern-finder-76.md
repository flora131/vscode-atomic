# Pattern Analysis: VS Code Rust Extension (Grammar-only)

## Scope Findings

The `extensions/rust/` partition contains only grammar definition and configuration files with no runtime code. Total LOC examined: ~140 lines across 7 files.

## Patterns Found

#### Pattern: Language Registration via Extension Manifest
**Where:** `extensions/rust/package.json:15-35`
**What:** VS Code extension declaratively registers language support through the `contributes` manifest.
```json
"contributes": {
  "languages": [
    {
      "id": "rust",
      "extensions": [".rs"],
      "aliases": ["Rust", "rust"],
      "configuration": "./language-configuration.json"
    }
  ],
  "grammars": [
    {
      "language": "rust",
      "path": "./syntaxes/rust.tmLanguage.json",
      "scopeName": "source.rust"
    }
  ]
}
```
**Variations / call-sites:** Standard VS Code extension pattern for all built-in language support.

#### Pattern: TextMate Grammar Integration
**Where:** `extensions/rust/syntaxes/rust.tmLanguage.json` (referenced)
**What:** VS Code uses external TextMate-style syntax grammars sourced from community projects.
```json
{
  "registrations": [
    {
      "component": {
        "type": "git",
        "git": {
          "name": "rust-syntax",
          "repositoryUrl": "https://github.com/dustypomerleau/rust-syntax",
          "commitHash": "268fd42cfd4aa96a6ed9024a2850d17d6cd2dc7b"
        }
      }
    }
  ]
}
```
**Variations / call-sites:** Declared in `extensions/rust/cgmanifest.json:1-17`; updated via build script `extensions/rust/build/update-grammar.mjs:9`.

#### Pattern: Language Configuration Declaration
**Where:** `extensions/rust/language-configuration.json:1-91`
**What:** Declarative JSON configuration for editor behavior (brackets, indentation, folding, comment markers).
```json
{
  "comments": {
    "lineComment": "//",
    "blockComment": ["/*", "*/"]
  },
  "brackets": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"]
  ],
  "autoClosingPairs": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
    {"open": "\"", "close": "\"", "notIn": ["string"]}
  ],
  "indentationRules": {
    "increaseIndentPattern": "^.*\\{[^}\"']*$|^.*\\([^\\)\"']*$",
    "decreaseIndentPattern": "^\\s*(\\s*\\/[*].*[*]\\/\\s*)*[})]"
  },
  "folding": {
    "markers": {
      "start": "^\\s*//\\s*#?region\\b",
      "end": "^\\s*//\\s*#?endregion\\b"
    }
  }
}
```
**Variations / call-sites:** Referenced in package.json via `extensions/rust/package.json:26`.

#### Pattern: Build Script for Grammar Updates
**Where:** `extensions/rust/build/update-grammar.mjs:7-9`
**What:** Node.js build script to synchronize external grammar repository with VS Code's bundled version.
```javascript
import * as vscodeGrammarUpdater from 'vscode-grammar-updater';
vscodeGrammarUpdater.update('dustypomerleau/rust-syntax', 'syntaxes/rust.tmLanguage.json', './syntaxes/rust.tmLanguage.json', undefined, 'main');
```
**Variations / call-sites:** Invoked via `extensions/rust/package.json:12` as `npm run update-grammar`.

## Summary

The Rust extension in this partition demonstrates VS Code's **declarative extension model** and **grammar composition architecture**. Core patterns show:

1. **Manifest-based registration**: Language features (syntax, configuration) are declared in `package.json` rather than imperative code.
2. **External grammar composition**: Syntax highlighting is delegated to TextMate grammars sourced from community repositories.
3. **Declarative editor behavior**: Indentation, folding, bracket matching, and comment rules are specified as structured JSON rather than implemented in TypeScript.
4. **Build-time integration**: Grammars are synchronized externally; no runtime parsing or language server integration occurs in this extension.

**Relevance to Tauri/Rust porting**: This scope contains no core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation). It is purely syntax/grammar definition. Any Tauri port would need to replicate VS Code's extension manifest system, grammar loading pipeline, and editor configuration framework—none of which are present here. The actual IDE functionality would exist in other partitions of the codebase.
