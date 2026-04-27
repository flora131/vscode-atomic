# Pattern Finder Partition 77: SQL Extension

## Research Question
What patterns exist for porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust?

## Scope
`extensions/sql/` — SQL language grammar and contribution-only extension (8 LOC of actual configuration).

---

## Patterns Found

#### Pattern: Language Contribution via package.json
**Where:** `extensions/sql/package.json:15-36`
**What:** Declarative language and grammar registration through VS Code extension manifest.
```json
"contributes": {
  "languages": [
    {
      "id": "sql",
      "extensions": [
        ".sql",
        ".dsql"
      ],
      "aliases": [
        "MS SQL",
        "T-SQL"
      ],
      "configuration": "./language-configuration.json"
    }
  ],
  "grammars": [
    {
      "language": "sql",
      "scopeName": "source.sql",
      "path": "./syntaxes/sql.tmLanguage.json"
    }
  ]
}
```
**Variations / call-sites:** This pattern repeats across all language extensions in VS Code. The `contributes` schema is extensible and declarative—no runtime code needed.

#### Pattern: Language Configuration for Editor Behavior
**Where:** `extensions/sql/language-configuration.json:1-41`
**What:** JSON configuration defining comments, bracket pairs, auto-closing rules, and folding regions for SQL syntax.
```json
{
	"comments": {
		"lineComment": "--",
		"blockComment": [ "/*", "*/" ]
	},
	"brackets": [
		["{", "}"],
		["[", "]"],
		["(", ")"]
	],
	"autoClosingPairs": [
		["{", "}"],
		[
			"[", "]"],
		["(", ")"],
		{ "open": "\"", "close": "\"", "notIn": ["string"] },
		{ "open": "N'", "close": "'", "notIn": ["string", "comment"] },
		{ "open": "'", "close": "'", "notIn": ["string", "comment"] }
	],
	"surroundingPairs": [
		["{", "}"],
		["[", "]"],
		["(", ")"],
		["\"", "\""],
		["'", "'"],
		["`", "`"]
	],
	"folding": {
		"offSide": true,
		"markers": {
			"start": "^\\s*--\\s*#region\\b",
			"end": "^\\s*--\\s*#endregion\\b"
		}
	}
}
```
**Variations / call-sites:** Standardized format applied across all language extensions. The schema is static; no runtime behavior defined here.

#### Pattern: Build-Time Grammar Generation
**Where:** `extensions/sql/build/update-grammar.mjs:6-8`
**What:** Build script using vscode-grammar-updater to fetch and transform grammar from upstream MSSQL repo.
```javascript
import * as vscodeGrammarUpdater from 'vscode-grammar-updater';

vscodeGrammarUpdater.update('microsoft/vscode-mssql', 'extensions/mssql/syntaxes/SQL.plist', './syntaxes/sql.tmLanguage.json', undefined, 'main');
```
**Variations / call-sites:** Referenced via npm script `update-grammar` in package.json:12. This is a build-time operation, not runtime—grammars are static TextMate `.tmLanguage.json` files bundled with the extension.

---

## Summary

The SQL extension is a **pure declarative language contribution** with no runtime TypeScript code. It consists entirely of:

1. **Extension metadata** (`package.json`) — declares language ID, file associations, aliases, configuration path, and grammar path.
2. **Editor behavior config** (`language-configuration.json`) — static JSON defining comment syntax, bracket matching, auto-closing pairs, and code folding markers.
3. **TextMate grammar** (`syntaxes/sql.tmLanguage.json`) — pre-generated syntax highlighting rules (sourced from upstream vscode-mssql).
4. **Build tooling** (`build/update-grammar.mjs`) — utility to refresh grammar from upstream, runs at build-time, not runtime.

**Porting Implications:** This pattern requires minimal runtime infrastructure—just declarative configuration loading and grammar compilation. A Tauri/Rust port would need to:
- Define equivalent manifest/contribution schema (likely JSON or TOML)
- Embed or load language configuration files at startup
- Register grammars with a syntax highlighting engine (e.g., tree-sitter or similar)
- Maintain build tooling to manage grammar updates

No core IDE functionality is embedded here; it's a pure data/configuration layer over a syntax highlighting system.
