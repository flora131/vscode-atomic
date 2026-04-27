# File Locations: Porting VS Code Core IDE Functionality from TypeScript/Electron to Tauri/Rust
## Partition 76 of 79 — `extensions/rust/`

### Implementation
- `extensions/rust/package.json` — VS Code built-in Rust language support extension; defines language contribution for `.rs` files with grammar and language configuration
- `extensions/rust/build/update-grammar.mjs` — Build script that updates Rust syntax highlighting grammar from upstream dustypomerleau/rust-syntax repository
- `extensions/rust/language-configuration.json` — Language configuration metadata for Rust syntax support (indentation, brackets, comments, etc.)
- `extensions/rust/syntaxes/rust.tmLanguage.json` — TextMate grammar definition for Rust syntax highlighting; provides semantic coloring rules

### Configuration
- `extensions/rust/cgmanifest.json` — Component governance manifest declaring upstream Rust syntax dependency (dustypomerleau/rust-syntax)
- `extensions/rust/.vscodeignore` — NPM package ignore rules for the extension

---

## Summary

The `extensions/rust/` directory contains a minimal built-in language support extension for Rust, consisting of 7 files focused entirely on syntax highlighting and language configuration. This is a **language grammar extension only** — it provides no IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation) and therefore has minimal relevance to a Tauri/Rust port. The extension demonstrates how VS Code plugs in language support via TextMate grammars, but porting core IDE functionality to Tauri would require substantially more investigation into the extensibility APIs, LSP integration, debug adapters, and the Electron/TypeScript core that implements actual editing and intelligence features.
