# Pattern Findings: extensions/rust/

## Scope Analysis
The `extensions/rust/` directory contains a minimal language extension package (9 LOC across source files) providing Rust syntax highlighting and bracket matching. This partition contains no runtime code relevant to porting VS Code's core IDE functionality to Tauri/Rust—only declarative language contribution definitions.

---

#### Pattern: Language Extension Contribution Package Structure
**Where:** `extensions/rust/package.json:1-41`
**What:** Declares a language extension with grammar and language configuration contributions to VS Code.
```json
{
  "name": "rust",
  "displayName": "%displayName%",
  "description": "%description%",
  "version": "10.0.0",
  "publisher": "vscode",
  "license": "MIT",
  "engines": {
    "vscode": "*"
  },
  "scripts": {
    "update-grammar": "node ./build/update-grammar.mjs"
  },
  "categories": ["Programming Languages"],
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
}
```
**Variations / call-sites:** Identical pattern repeated across all built-in language extensions (`extensions/html/`, `extensions/javascript/`, etc.).

---

#### Pattern: Language Configuration (Bracket Matching, Auto-closing, Indentation)
**Where:** `extensions/rust/language-configuration.json:1-92`
**What:** Defines editor behaviors for Rust: bracket matching, auto-closing pairs, indentation rules, and on-enter rules.
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
  "surroundingPairs": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
    ["\"", "\""],
    ["<", ">"]
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
  },
  "onEnterRules": [
    {
      "beforeText": {"pattern": "\/\/.*"},
      "afterText": {"pattern": "^(?!\\s*$).+"},
      "action": {"indent": "none", "appendText": "// "}
    }
  ]
}
```
**Variations / call-sites:** Present in all language extensions; defines core editor navigation and editing behaviors.

---

#### Pattern: TextMate Grammar with External Source
**Where:** `extensions/rust/syntaxes/rust.tmLanguage.json:1-30`
**What:** TextMate-compatible grammar definition sourced from external repository, with contributor attribution.
```json
{
  "information_for_contributors": [
    "This file has been converted from https://github.com/dustypomerleau/rust-syntax/blob/master/syntaxes/rust.tmLanguage.json",
    "If you want to provide a fix or improvement, please create a pull request against the original repository.",
    "Once accepted there, we are happy to receive an update request."
  ],
  "version": "https://github.com/dustypomerleau/rust-syntax/commit/268fd42cfd4aa96a6ed9024a2850d17d6cd2dc7b",
  "name": "Rust",
  "scopeName": "source.rust",
  "patterns": [
    {
      "comment": "boxed slice literal",
      "begin": "(<)(\\[)",
      "beginCaptures": {
        "1": {"name": "punctuation.brackets.angle.rust"},
        "2": {"name": "punctuation.brackets.square.rust"}
      },
      "end": ">",
      "endCaptures": {"0": {"name": "punctuation.brackets.angle.rust"}},
      "patterns": [{"include": "#block-comments"}]
    }
  ]
}
```
**Variations / call-sites:** Grammar updater script at `extensions/rust/build/update-grammar.mjs:9` pulls updates from upstream.

---

#### Pattern: Grammar Update Automation Script
**Where:** `extensions/rust/build/update-grammar.mjs:1-10`
**What:** Node.js script using `vscode-grammar-updater` to sync grammar from upstream Rust syntax repository.
```javascript
import * as vscodeGrammarUpdater from 'vscode-grammar-updater';

vscodeGrammarUpdater.update('dustypomerleau/rust-syntax', 'syntaxes/rust.tmLanguage.json', './syntaxes/rust.tmLanguage.json', undefined, 'main');
```
**Variations / call-sites:** `package.json:12` defines npm script `update-grammar` to invoke this build file.

---

#### Pattern: Localization Configuration
**Where:** `extensions/rust/package.nls.json:1-4`
**What:** Translation keys for language extension display name and description.
```json
{
  "displayName": "Rust Language Basics",
  "description": "Provides syntax highlighting and bracket matching in Rust files."
}
```
**Variations / call-sites:** Paired with `package.json` use of `%displayName%` and `%description%` placeholders for localization.

---

## Summary

The `extensions/rust/` partition contains **only language extension declarative metadata** with no runtime code. It demonstrates VS Code's extension architecture for language contributions: a minimal package declares language identifiers, file associations, grammar sources (TextMate JSON), and editor behavior rules (bracket matching, indentation, auto-closing pairs). All patterns are **configuration-driven and platform-agnostic**—they specify *what* the editor should do (e.g., "auto-close braces") without implementation details. No C++, TypeScript, or Electron/Tauri-specific logic is present. This reflects the separation between VS Code's core editor engine and pluggable language support, where porting to Tauri would require reimplementing the declarative-to-behavior translation layer in Rust, not porting code from this extension.

