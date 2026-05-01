## File Locations for extensions/html/ Partition

### Configuration
- `extensions/html/package.json` — Declares HTML language contribution with syntax highlighting (TextMate grammars) and code snippets; defines language ID, file extensions, MIME types, and grammar scopes
- `extensions/html/language-configuration.json` — Editor configuration for HTML bracket matching, auto-closing pairs, indentation rules, and on-enter behaviors
- `extensions/html/package.nls.json` — Localization file with display names and descriptions for the HTML language extension

### Grammar / Syntax Definitions
- `extensions/html/syntaxes/html.tmLanguage.json` — TextMate grammar for HTML with embedded language support (CSS, JavaScript, Python, Smarty)
- `extensions/html/syntaxes/html-derivative.tmLanguage.json` — Alternative HTML TextMate grammar for HTML-derivative languages

### Snippets
- `extensions/html/snippets/html.code-snippets` — HTML code snippets for content assistance

### Build / Maintenance
- `extensions/html/build/update-grammar.mjs` — Script to update the HTML grammar from source
- `extensions/html/.vscodeignore` — Build artifacts exclusion file
- `extensions/html/cgmanifest.json` — Component governance manifest for dependencies

## Summary

The `extensions/html/` partition contains exclusively grammar, snippet, and language configuration definitions. There is no runtime code, and no relevance to porting VS Code's core IDE functionality to Tauri/Rust. This is a declarative language extension that registers HTML syntax highlighting, bracket matching rules, and code snippets with the editor. Porting this would require only a corresponding language server or grammar engine in the Rust/Tauri backend—the configuration files themselves could be reused or transformed to target a new system's grammar format.

