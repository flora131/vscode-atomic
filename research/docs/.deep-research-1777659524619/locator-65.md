# VS Code JSON Extension - File Location Documentation

## Overview
The `extensions/json/` directory contains VS Code's built-in JSON language support extension. This is a language contribution extension that provides syntax highlighting, code completion, and language configuration for JSON, JSON with Comments (JSONC), JSON Lines (JSONL), and VS Code Code Snippets formats.

### Implementation
- `extensions/json/package.json` — Extension manifest declaring language and grammar contributions for JSON-based formats
- `extensions/json/language-configuration.json` — Language configuration defining auto-closing pairs, indentation rules, bracket matching, and enter rules for JSON editing
- `extensions/json/build/update-grammars.js` — Build script that updates TextMate grammar files from upstream repositories using vscode-grammar-updater

### Configuration
- `extensions/json/cgmanifest.json` — Component governance manifest for tracking external dependencies
- `extensions/json/package.nls.json` — Localization/translation strings for UI display names and descriptions
- `extensions/json/.vscodeignore` — Excludes unnecessary files from extension packaging

### Syntax/Grammar Files
- `extensions/json/syntaxes/JSON.tmLanguage.json` — TextMate grammar for standard JSON syntax highlighting
- `extensions/json/syntaxes/JSONC.tmLanguage.json` — TextMate grammar for JSON with Comments (adapted from JSON grammar)
- `extensions/json/syntaxes/JSONL.tmLanguage.json` — TextMate grammar for JSON Lines format (adapted from JSON grammar)
- `extensions/json/syntaxes/snippets.tmLanguage.json` — TextMate grammar for VS Code code snippets syntax (adapted from better-snippet-syntax repository)

## Summary
This extension is purely a grammar and language contribution package—it defines how VS Code recognizes and highlights JSON-based file formats without including any runtime or backend logic. The extension contributes four language modes (json, jsonc, jsonl, snippets) with corresponding TextMate grammars and language configuration rules. The build process automatically updates grammar files from upstream repositories (microsoft/vscode-JSON.tmLanguage and jeff-hykin/better-snippet-syntax), then applies scope name adaptations for variant formats.

**Relevance to Tauri/Rust Port**: This extension demonstrates the syntax highlighting layer of VS Code's IDE functionality. Porting this would require equivalent TextMate grammar support and language configuration parsing in a Rust-based editor framework, though the actual grammar definitions are language-agnostic XML-based formats that could be reused with a compatible syntax highlighting engine.
