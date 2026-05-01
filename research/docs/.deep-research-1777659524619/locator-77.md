## extensions/sql/ — SQL Language Extension

### Implementation
- `extensions/sql/package.json` — Extension manifest with SQL grammar definition
- `extensions/sql/language-configuration.json` — Language configuration for syntax highlighting and bracket matching
- `extensions/sql/syntaxes/sql.tmLanguage.json` — TextMate grammar for SQL syntax (642 lines)
- `extensions/sql/build/update-grammar.mjs` — Build script for grammar updates (8 lines)

### Configuration
- `extensions/sql/cgmanifest.json` — Component governance manifest
- `extensions/sql/package.nls.json` — Localization strings
- `extensions/sql/.vscodeignore` — VS Code extension ignore patterns

### Notable Clusters
The extension consists of minimal implementation: a TextMate grammar definition, configuration files for bracket matching and indentation, and localization. The directory structure reveals this is a language extension providing syntax highlighting and language services for SQL files through VS Code's built-in language support mechanisms rather than a custom language server.

**Relevance to Tauri/Rust port:** This SQL extension demonstrates the pattern for language support in VS Code. A Rust-based IDE alternative would need to either: (1) reimplement TextMate grammar parsing in Rust, (2) reuse TextMate grammars through a compatibility layer, or (3) adopt a different syntax highlighting approach. The extension's minimal code footprint suggests syntax highlighting is intentionally delegated to VS Code's core engine.
