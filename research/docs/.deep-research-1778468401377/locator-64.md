# Locator Report: extensions/json/ (Partition 64 of 80)

## Overview
The `extensions/json/` directory contains a built-in VS Code extension providing JSON language support with syntax highlighting, bracket matching, and code snippet definitions. The extension is minimal in scope and infrastructure-focused rather than logic-heavy.

## Implementation

### Grammar & Syntax Definitions
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/syntaxes/JSON.tmLanguage.json` — TextMate grammar for base JSON syntax
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/syntaxes/JSONC.tmLanguage.json` — TextMate grammar for JSON with Comments variant
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/syntaxes/JSONL.tmLanguage.json` — TextMate grammar for JSON Lines format
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/syntaxes/snippets.tmLanguage.json` — TextMate grammar for VS Code code snippet syntax (314 KB, auto-generated)

### Build Automation
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/build/update-grammars.js` (39 LOC) — Script to pull and adapt grammar definitions from upstream sources (microsoft/vscode-JSON.tmLanguage and jeff-hykin/better-snippet-syntax repositories). Uses `vscode-grammar-updater` module and includes scope-name adaptation logic via `adaptJSON()` function.

## Configuration

### Extension Manifest
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/package.json` (133 LOC) — Extension entry point defining:
  - Contributes 4 languages: `json`, `jsonc`, `jsonl`, `snippets` with associated file extensions (.json, .jsonc, .code-snippets, etc.) and glob patterns
  - Contributes 4 grammar registrations mapping each language to its TextMate syntax file
  - Build script reference for grammar updates
  - Package metadata (version 10.0.0, MIT license)

### Language Configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/language-configuration.json` (84 LOC) — Editor behavior configuration:
  - Comment delimiters (line comment `//'`, block comment `/* */`)
  - Bracket pairs for auto-closing: `{}`, `[]`, `()`, quotes, backticks
  - Indentation rules (patterns for increase/decrease indent)
  - On-enter rules (auto-append `//` in line comments)

### Localization
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/package.nls.json` (4 LOC) — Human-readable display name and description for localization

### Packaging & Compliance
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/.vscodeignore` (3 LOC) — Exclude build, test, and cgmanifest files from packaged extension
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/cgmanifest.json` (28 LOC) — Component governance manifest listing third-party grammar sources:
  - microsoft/vscode-JSON.tmLanguage (commit 9bd83f1c)
  - jeff-hykin/better-snippet-syntax (commit 2b1bb124, version 1.0.2)
  Both MIT licensed.

## Notable Clusters

The extension has two distinct subsystems:
1. **Syntax & Language Metadata**: Grammar files + language-configuration.json define the lexical structure and editor behavior for JSON variants.
2. **Build Infrastructure**: Minimal Node.js automation to fetch and adapt grammars from external sources, ensuring grammar definitions stay in sync with upstream repositories.

The TextMate grammar format (`.tmLanguage.json`) is the primary artifact for syntax highlighting and tokenization, generated from upstream sources and adapted via scope-name transformations. No custom runtime logic or test files exist in this extension—it is purely declarative.

## Porting Implications

For a Tauri/Rust port, the `extensions/json/` extension highlights the dependency on TextMate grammar infrastructure. TextMate grammars are widely supported in editor frameworks (via libraries like `tree-sitter` or `oniguruma`-based tokenizers) and could be reused in a Rust-based editor. The language contributions (file extensions, mimetypes, configuration like bracket matching and indentation rules) would need to be redefined in the new system's language server or plugin architecture. The build automation (grammar updates) would require equivalent tooling in Rust to fetch and adapt upstream sources.
