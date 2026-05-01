# Locator 76: extensions/rust/ — Porting to Tauri/Rust Analysis

## Summary
The `extensions/rust/` directory contains only a language support extension for Rust syntax highlighting and language configuration within VS Code's extension system. These are editor grammar and language metadata files, not core IDE functionality implementations.

## Configuration
- `extensions/rust/package.json` — VS Code extension manifest declaring Rust language support, TextMate grammar integration, and build script for grammar updates
- `extensions/rust/language-configuration.json` — Language behavior definitions (auto-closing pairs, indentation rules, folding markers, comment patterns for Rust)
- `extensions/rust/cgmanifest.json` — Component governance manifest for dependency tracking
- `extensions/rust/.vscodeignore` — Packaging exclusions for the extension

## Documentation
- `extensions/rust/package.nls.json` — Localization strings for the extension display name and description

## Syntax / Grammar
- `extensions/rust/syntaxes/rust.tmLanguage.json` — TextMate grammar definition for Rust syntax highlighting

## Build
- `extensions/rust/build/update-grammar.mjs` — Build script to update the Rust grammar file

## Notable Clusters
- `extensions/rust/` — 7 files total (1 implementation file declared in scope), a complete Rust language extension that provides syntax highlighting and editor behavior, not core IDE functionality

## Analysis
The Rust extension in this directory is a pure language support extension—it provides syntax highlighting and editor configuration via TextMate grammar and language configuration metadata. It does not contain any core VS Code IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation) that would be relevant to porting core features to Tauri/Rust. The extension is minimal and limited to declarative configuration and grammar files, making it orthogonal to the porting research question focused on core IDE functionality migration from TypeScript/Electron to Tauri/Rust.

