# Locator Results for Partition 61 (extensions/html/)

## Research Question
What it would take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

## Scope
`extensions/html/` — 9 files total, 61 LOC

---

## Implementation

- `extensions/html/package.json` — Extension manifest declaring HTML language support with grammar definitions and snippet contributions
- `extensions/html/language-configuration.json` — Language behavior configuration for HTML (bracket pairs, auto-closing, folding, indentation patterns, on-enter rules)
- `extensions/html/build/update-grammar.mjs` — Build script for synchronizing TextMate HTML grammar from upstream repository with VS Code-specific patches
- `extensions/html/syntaxes/html.tmLanguage.json` — TextMate grammar definition for HTML syntax highlighting with embedded language support (CSS, JavaScript, Python)
- `extensions/html/syntaxes/html-derivative.tmLanguage.json` — Variant TextMate grammar for HTML derivatives with modified tag matching rules

## Configuration

- `extensions/html/.vscodeignore` — Build artifact exclusion list
- `extensions/html/cgmanifest.json` — Component governance manifest registering external TextMate/html.tmbundle dependency
- `extensions/html/package.nls.json` — Localization strings for display name and description

## Examples / Fixtures

- `extensions/html/snippets/html.code-snippets` — Code snippet definitions for HTML language

---

## Summary

The `extensions/html/` directory contains a minimal language extension for HTML support in VS Code. It comprises grammar definitions (TextMate format), language behavior configuration, a build pipeline for grammar synchronization, and snippet templates. This extension demonstrates the declarative extension model VS Code uses to add language support: a `package.json` manifest registers language contributions, TextMate grammars handle syntax highlighting, and language configuration controls editor behaviors.

From a Tauri/Rust porting perspective, this extension reveals how VS Code abstracts language intelligence through external grammar formats and configuration files. Porting would require: (1) parsing and interpreting TextMate grammars in Rust, (2) reimplementing language configuration behaviors (bracket matching, folding, indentation rules) in the editor core, and (3) establishing a plugin system that can load and manage language extensions declaratively. This is a lower-level concern than the core IDE features (debugging, source control, terminal) but is essential for the editing experience.

