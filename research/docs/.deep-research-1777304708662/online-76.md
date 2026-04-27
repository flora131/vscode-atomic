# Partition 76 Online Research — `extensions/rust/`

(no external research applicable)

## Justification

The `extensions/rust/` directory contains exclusively static declaration files — a TextMate grammar (`syntaxes/rust.tmLanguage.json`), a VS Code language configuration (`language-configuration.json`), and a `package.json` manifest — with **zero runtime npm dependencies** and no JavaScript/TypeScript source code that executes at runtime. The `package.json` declares only `"engines": { "vscode": "*" }` and two contribution points (`languages` and `grammars`); there are no `dependencies` or `devDependencies` entries referencing any external library. Because this extension is purely a data-driven syntax-highlighting and bracket-matching descriptor, no third-party library or framework documentation is central to the research question of porting VS Code's core IDE functionality to Tauri/Rust. Fetching external docs would add no value here.

## Files Inspected

| File | Purpose | Runtime code? |
|---|---|---|
| `extensions/rust/package.json` | Extension manifest; declares language/grammar contributions | No |
| `extensions/rust/language-configuration.json` | Bracket pairs, comments, auto-close rules | No (static JSON) |
| `extensions/rust/syntaxes/rust.tmLanguage.json` | TextMate grammar for Rust syntax highlighting | No (static JSON) |
| `extensions/rust/build/update-grammar.mjs` | Dev-time script to refresh the grammar from upstream | Dev-only, not shipped |
| `extensions/rust/cgmanifest.json` | Component-governance provenance record | No |

## Summary

Partition 76 covers the built-in Rust language support extension for VS Code. It is a thin, data-only extension that contributes a TextMate grammar and language configuration; it has no runtime dependencies and no TypeScript/JavaScript logic of its own. No external library documentation is relevant, and no external research was performed.
