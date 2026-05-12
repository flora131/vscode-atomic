# SQL Extension: Grammar-Only Implementation

## Implementation

- `extensions/sql/syntaxes/sql.tmLanguage.json` (642 LOC) — TextMate grammar definition for SQL syntax highlighting
- `extensions/sql/language-configuration.json` (41 LOC) — Language configuration defining comments, brackets, auto-closing pairs, folding markers, and surrounding pairs for SQL
- `extensions/sql/package.json` (42 LOC) — Extension manifest declaring language registration for `.sql` and `.dsql` files, grammar path, and build scripts

## Configuration

- `extensions/sql/package.nls.json` (4 LOC) — Localization strings for extension display name and description
- `extensions/sql/.vscodeignore` (4 LOC) — Build artifacts and test exclusion patterns for packaging
- `extensions/sql/cgmanifest.json` (17 LOC) — Component governance manifest referencing upstream source from microsoft/vscode-mssql repository

## Build

- `extensions/sql/build/update-grammar.mjs` (8 LOC) — Automated grammar update script that synchronizes sql.tmLanguage.json with upstream vscode-mssql repository

## Notable Clusters

The SQL extension is a minimal grammar-only implementation with no tests, no type definitions, and no examples. It consists of 7 files totaling approximately 758 lines. The core functionality is the TextMate grammar file (642 LOC) for syntax highlighting, supported by a language configuration file that defines editor behaviors (comments, brackets, folding). The extension declaration in package.json registers support for SQL file extensions (.sql, .dsql) and aliases (MS SQL, T-SQL). All grammar maintenance is automated via the update-grammar.mjs script that pulls the latest grammar definition from the upstream vscode-mssql repository.
