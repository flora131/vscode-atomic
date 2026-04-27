# SQL Extension - File Locations

## Scope
`extensions/sql/` (7 files)

### Configuration
- `extensions/sql/package.json` - Extension manifest declaring SQL language support and grammar contribution
- `extensions/sql/language-configuration.json` - Language configuration defining comments, bracket pairs, auto-closing pairs, and folding regions for SQL
- `extensions/sql/package.nls.json` - Localization strings for display name and description
- `extensions/sql/.vscodeignore` - Patterns for exclusion from packaged extension
- `extensions/sql/cgmanifest.json` - Component governance manifest tracking dependency on microsoft/vscode-mssql repository

### Grammars / Syntaxes
- `extensions/sql/syntaxes/sql.tmLanguage.json` - TextMate language grammar for SQL syntax highlighting (642 lines)

### Build Scripts
- `extensions/sql/build/update-grammar.mjs` - Build script importing `vscode-grammar-updater` to sync grammar from microsoft/vscode-mssql repository

## Summary

The SQL extension is a minimal, lightweight language extension providing syntax highlighting and editor affordances for SQL files. It comprises 7 files totaling approximately 8 LOC of custom code (excluding grammar definition). The extension declares support for `.sql` and `.dsql` file extensions with aliases for MS SQL/T-SQL. Language configuration enables standard SQL features like line and block comments, bracket matching, and code folding via region markers. The grammar is maintained downstream from an external repository (microsoft/vscode-mssql) via an automated build script that pulls updates. This extension represents VS Code's approach to language support through declarative configuration and TextMate grammar contribution, with no compiled code—a pattern that would require significant refactoring for a Tauri/Rust port focused on embedding language services directly rather than relying on plugin architecture.

