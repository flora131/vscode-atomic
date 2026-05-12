# Partition 77: Extensions/Rust Grammar & Snippets

## Scope Confirmation
The `extensions/rust/` directory contains a language extension providing syntax highlighting and bracket matching for Rust files. As confirmed by the architectural briefing, this scope is grammar/snippets only and marginally relevant to a Tauri/Rust port investigation.

## Implementation
- `extensions/rust/package.json` — Rust language extension manifest declaring `.rs` file support with grammar and language configuration contribution points
- `extensions/rust/language-configuration.json` — Defines language basics: line/block comments, bracket pairs, auto-closing rules, indentation patterns, and folding markers for Rust
- `extensions/rust/syntaxes/rust.tmLanguage.json` — TextMate-style syntax grammar (converted from upstream dustypomerleau/rust-syntax repository, commit 268fd42cfd4aa96a6ed9024a2850d17d6cd2dc7b)

## Configuration
- `extensions/rust/package.nls.json` — Localization strings (display name, description)
- `extensions/rust/cgmanifest.json` — Component governance manifest tracking the external rust-syntax grammar dependency (MIT license, v0.6.1)
- `extensions/rust/.vscodeignore` — Excludes test/, build/, and cgmanifest.json from packaged extension

## Notable Clusters
- `extensions/rust/build/update-grammar.mjs` — Build script using vscode-grammar-updater to sync grammar from upstream repository

## Research Relevance
This partition contains only presentation-layer assets for Rust language support in VS Code's editor surface. It provides no insight into IDE core functionality (editing engines, language servers, debugging infrastructure, source control, terminal, navigation) that would be relevant to a Tauri/Rust port. The grammar definitions themselves are TextMate-based syntax rules with no direct bearing on architectural porting decisions.
