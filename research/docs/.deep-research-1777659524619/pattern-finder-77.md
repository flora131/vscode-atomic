# Pattern Research: VS Code SQL Extension (Grammar-Only)

**Scope:** `extensions/sql/` (7 files, minimal TypeScript)  
**Focus:** Patterns for porting VS Code IDE functionality to Tauri/Rust

---

## Patterns Identified

#### Pattern 1: Grammar Registration via Extension Manifest
**Where:** `extensions/sql/package.json:15-37`
**What:** Declarative extension manifest declaring language support without implementation code. Uses VS Code's extension API to register TextMate grammar and language configuration.

```json
"contributes": {
  "languages": [
    {
      "id": "sql",
      "extensions": [".sql", ".dsql"],
      "aliases": ["MS SQL", "T-SQL"],
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

**Variations:** Grammar registration is purely declarative JSON with no executable code. The extension is a build-time artifact with no runtime TypeScript/JavaScript.

---

#### Pattern 2: Language Configuration for Syntax Features
**Where:** `extensions/sql/language-configuration.json:1-42`
**What:** JSON-based configuration for bracket matching, auto-closing pairs, line/block comments, and code folding regions without executable code.

```json
{
  "comments": {
    "lineComment": "--",
    "blockComment": ["/*", "*/"]
  },
  "brackets": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"]
  ],
  "autoClosingPairs": [
    ["{", "}"],
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
    { "open": "\"", "close": "\"", "notIn": ["string"] },
    { "open": "N'", "close": "'", "notIn": ["string", "comment"] },
    { "open": "'", "close": "'", "notIn": ["string", "comment"] }
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

**Variations:** Static JSON configuration. Comments, bracket matching, and folding are all declarative. No extension logic required at runtime.

---

#### Pattern 3: TextMate Grammar Format (TMLanguage JSON)
**Where:** `extensions/sql/syntaxes/sql.tmLanguage.json:1-100`
**What:** Regex-based pattern matching for syntax highlighting using TextMate scope naming convention. Grammar source tracked from upstream (vscode-mssql).

```json
{
  "information_for_contributors": [
    "This file has been converted from https://github.com/microsoft/vscode-mssql/blob/master/extensions/mssql/syntaxes/SQL.plist",
    "If you want to provide a fix or improvement, please create a pull request against the original repository."
  ],
  "version": "https://github.com/microsoft/vscode-mssql/commit/c002f514dd81fa71fa304d4c36f8d2767dbf2f9d",
  "name": "SQL",
  "scopeName": "source.sql",
  "patterns": [
    {
      "match": "((?<!@)@)\\b(\\w+)\\b",
      "name": "text.variable"
    },
    {
      "match": "(\\[)[^\\]]*(\\])",
      "name": "text.bracketed"
    },
    {
      "include": "#comments"
    },
    {
      "captures": {
        "1": { "name": "keyword.other.create.sql" },
        "2": { "name": "keyword.other.sql" },
        "5": { "name": "entity.name.function.sql" }
      },
      "match": "(?i:^\\s*(create(?:\\s+replace)?)\\s+(aggregate|conversion|database|domain|function|group|(unique\\s+)?index|language|operator class|operator|rule|schema|sequence|table|tablespace|trigger|type|user|view)\\s+)(['\"`]?)(\\w+)\\4",
      "name": "meta.create.sql"
    }
  ]
}
```

**Variations:** Patterns include regex matching, capture groups with scope names, includes, and named rule references. This format is standard across TextMate-compatible editors.

---

#### Pattern 4: Grammar Update Build Process
**Where:** `extensions/sql/build/update-grammar.mjs:1-9`
**What:** Build script that synchronizes grammar from upstream vscode-mssql repository using vscode-grammar-updater utility. Demonstrates grammar maintenance pattern via CI/CD.

```javascript
import * as vscodeGrammarUpdater from 'vscode-grammar-updater';

vscodeGrammarUpdater.update(
  'microsoft/vscode-mssql',
  'extensions/mssql/syntaxes/SQL.plist',
  './syntaxes/sql.tmLanguage.json',
  undefined,
  'main'
);
```

**Variations:** Grammar update is automated via npm script (`"update-grammar": "node ./build/update-grammar.mjs"`). Upstream tracking enables grammar improvements without maintaining parity.

---

#### Pattern 5: Component Attribution and Licensing
**Where:** `extensions/sql/cgmanifest.json:1-17`
**What:** Component governance manifest documenting upstream source (vscode-mssql) with git commit hash and MIT license. Enables license compliance tracking.

```json
{
  "registrations": [
    {
      "component": {
        "type": "git",
        "git": {
          "name": "microsoft/vscode-mssql",
          "repositoryUrl": "https://github.com/microsoft/vscode-mssql",
          "commitHash": "c002f514dd81fa71fa304d4c36f8d2767dbf2f9d"
        }
      },
      "license": "MIT",
      "version": "1.0.0"
    }
  ],
  "version": 1
}
```

**Variations:** Static manifest with git source tracking. Enables transparency about third-party dependencies.

---

#### Pattern 6: Minimal Extension Packaging with Metadata
**Where:** `extensions/sql/package.json:1-42`
**What:** Lightweight extension manifest with no runtime dependencies. Includes localization via nls.json files and version constraints.

```json
{
  "name": "sql",
  "displayName": "%displayName%",
  "description": "%description%",
  "version": "10.0.0",
  "publisher": "vscode",
  "license": "MIT",
  "engines": { "vscode": "*" },
  "scripts": { "update-grammar": "node ./build/update-grammar.mjs" },
  "categories": ["Programming Languages"],
  "repository": {
    "type": "git",
    "url": "https://github.com/microsoft/vscode.git"
  }
}
```

**Variations:** Zero npm dependencies. Localization keys (`%displayName%`, `%description%`) resolved from nls.json. Very minimal extension footprint.

---

## Summary

The SQL extension is a **grammar-only language support package** with no executable TypeScript code. It demonstrates patterns for porting VS Code features to Tauri/Rust:

1. **Declarative Grammar Registration**: Language support can be purely data-driven via manifest JSON without runtime code.

2. **TextMate Grammar Compatibility**: Syntax highlighting uses industry-standard TextMate regex patterns, which can be reused across editors (including Tauri).

3. **Build-Time Code Generation**: Grammar updates are automated via build scripts that pull from upstream repositories, reducing maintenance burden.

4. **Zero Runtime Dependencies**: No npm packages required; grammars are self-contained static files.

5. **Metadata-Driven Localization**: UI strings use placeholder keys resolved at build/runtime, enabling multi-language support without code duplication.

**Implications for Tauri/Rust Port:**
- Language support layers (grammars, syntax highlighting) can be ported as static data structures with minimal runtime overhead
- No need to port extension manifest logic; grammars are language-agnostic
- Build processes can remain in Node.js while core IDE becomes Rust-based
- TextMate grammars require a regex engine in Rust (e.g., `regex` crate) for runtime matching
