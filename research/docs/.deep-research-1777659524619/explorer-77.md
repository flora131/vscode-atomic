# Partition 77 of 79 — Findings

## Scope
`extensions/sql/` (1 files, 8 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `extensions/sql/package.json`
- `extensions/sql/language-configuration.json`
- `extensions/sql/syntaxes/sql.tmLanguage.json`
- `extensions/sql/build/update-grammar.mjs`
- `extensions/sql/cgmanifest.json`

---

### Per-File Notes

#### `extensions/sql/package.json`

- **Role:** VS Code extension manifest that registers the SQL language with the editor, wiring up file associations, display aliases, a language configuration, and a TextMate grammar.
- **Key symbols:**
  - `contributes.languages[0]` (`package.json:16-29`) — declares language id `"sql"` with file extensions `.sql` and `.dsql`, aliases `"MS SQL"` / `"T-SQL"`, and points to `./language-configuration.json`.
  - `contributes.grammars[0]` (`package.json:30-35`) — maps `source.sql` scope name to `./syntaxes/sql.tmLanguage.json`.
  - `scripts["update-grammar"]` (`package.json:12`) — invokes `node ./build/update-grammar.mjs` to pull a refreshed grammar from upstream.
- **Control flow:** This file is purely declarative. VS Code's extension host reads it at activation time and uses `contributes` data to register the language; no runtime code is executed from this file itself.
- **Data flow:** Input is the JSON manifest; output is language registration state held by VS Code's language registry (not by this extension). The file references two sibling assets: `language-configuration.json` and `syntaxes/sql.tmLanguage.json`.
- **Dependencies:** None at runtime. The `update-grammar` script depends on the `vscode-grammar-updater` npm package (used only in the build step).

---

#### `extensions/sql/language-configuration.json`

- **Role:** Declares editor-level behavioural rules for SQL files — comment tokens, bracket pairs, auto-close and surrounding-pair rules, and code-folding markers.
- **Key symbols:**
  - `comments` (`language-configuration.json:2-5`) — line comment token `--`, block comment delimiters `/* */`.
  - `brackets` (`language-configuration.json:6-10`) — three bracket pairs: `{}`, `[]`, `()`.
  - `autoClosingPairs` (`language-configuration.json:11-18`) — includes T-SQL–specific `N'…'` national string literal pair (`language-configuration.json:16`).
  - `surroundingPairs` (`language-configuration.json:19-26`) — also covers backtick (`` ` ``).
  - `folding.markers` (`language-configuration.json:29-32`) — region folding triggered by `-- #region` / `-- #endregion` comment sentinels.
- **Control flow:** Purely declarative JSON consumed by VS Code's Monaco editor core; no executable logic.
- **Data flow:** VS Code's language configuration loader reads this file once when the SQL language activates and injects its rules into the editor's tokenisation and bracket-matching subsystems. There is no output beyond the in-memory configuration state.
- **Dependencies:** None; it is a pure data file read by VS Code's built-in language configuration service.

---

#### `extensions/sql/syntaxes/sql.tmLanguage.json`

- **Role:** TextMate grammar (642 lines) that drives syntax highlighting for SQL by defining pattern-match rules, scope name assignments, and repository includes that cover keywords, literals, comments, operators, and DDL/DML constructs.
- **Key symbols:**
  - Top-level `patterns` array (`sql.tmLanguage.json:10`) — ordered list of match rules applied to every line; first match wins.
  - `@variable` pattern (`sql.tmLanguage.json:12-14`) — assigns scope `text.variable` to T-SQL `@param` syntax.
  - `meta.create.sql` pattern (`sql.tmLanguage.json:23-36`) — regex covering `CREATE [OR REPLACE] <object-type> <name>` constructs, assigning `keyword.other.create.sql`, `keyword.other.sql`, and `entity.name.function.sql` scopes.
  - `#comments` repository rule — referenced from the top-level patterns via `{ "include": "#comments" }` (`sql.tmLanguage.json:20-22`); handles `--` and `/* */` comment blocks.
  - `information_for_contributors` (`sql.tmLanguage.json:2-6`) — documents that this file is converted from `vscode-mssql/extensions/mssql/syntaxes/SQL.plist` at commit `c002f514dd81fa71fa304d4c36f8d2767dbf2f9d`.
- **Control flow:** The grammar is loaded by VS Code's Textmate tokeniser (vscode-textmate) at language activation. Each line of an open `.sql` or `.dsql` file is tokenised by iterating through the top-level `patterns` array and their nested `repository` entries.
- **Data flow:** Input is raw SQL source text. Output is a sequence of (offset, scope-stack) tokens consumed by Monaco's renderer for colour theming. No mutable state is produced; the grammar is stateless per-line.
- **Dependencies:** Upstream source is `github.com/microsoft/vscode-mssql` (recorded in `cgmanifest.json:8`). At runtime it depends only on VS Code's built-in vscode-textmate tokeniser; no npm imports.

---

#### `extensions/sql/build/update-grammar.mjs`

- **Role:** Eight-line ESM build script that fetches the latest SQL TextMate grammar from the upstream `vscode-mssql` repository and writes it to `./syntaxes/sql.tmLanguage.json`.
- **Key symbols:**
  - `vscodeGrammarUpdater.update(...)` call (`update-grammar.mjs:8`) — the only runtime statement; arguments are: source repo `'microsoft/vscode-mssql'`, source path `'extensions/mssql/syntaxes/SQL.plist'`, destination `'./syntaxes/sql.tmLanguage.json'`, options `undefined`, branch `'main'`.
- **Control flow:** Linear: import `vscode-grammar-updater`, invoke `update()` which (internally) fetches the `.plist` from GitHub, converts it to JSON, and overwrites the local destination file.
- **Data flow:** Input is a remote `.plist` file from GitHub. Output is the overwritten `syntaxes/sql.tmLanguage.json`. No persistent state other than that file.
- **Dependencies:** `vscode-grammar-updater` npm package (external; build-time only).

---

#### `extensions/sql/cgmanifest.json`

- **Role:** Component governance manifest that pins the exact upstream Git commit of the SQL grammar for open-source component tracking and license compliance.
- **Key symbols:**
  - `registrations[0].component.git.commitHash` (`cgmanifest.json:9`) — `c002f514dd81fa71fa304d4c36f8d2767dbf2f9d`, identifies the vendored grammar version.
  - `registrations[0].license` (`cgmanifest.json:11`) — `"MIT"`.
- **Control flow:** Declarative; consumed by Microsoft's component governance tooling at build/release time, not at editor runtime.
- **Data flow:** Static record; no data flows through this file at runtime.
- **Dependencies:** None.

---

### Cross-Cutting Synthesis

The `extensions/sql/` partition is a self-contained, pure-data language extension. It contributes no executable TypeScript or Rust code. Its entire runtime surface is three declarative assets: `package.json` (language and grammar registration), `language-configuration.json` (editor behaviour rules), and `syntaxes/sql.tmLanguage.json` (TextMate tokeniser grammar). These are consumed at extension-activation time by VS Code's built-in language registry, Monaco bracket-matching subsystem, and vscode-textmate tokeniser — all of which live elsewhere in the core. The build-time script `update-grammar.mjs` and governance file `cgmanifest.json` have no bearing on editor runtime.

Relevance to the Tauri/Rust porting question: this partition represents one of the simplest extension archetypes in VS Code — a language grammar pack with zero JavaScript/TypeScript runtime code. Porting this specific extension to a Tauri host would require only that the target host implement: (1) a TextMate-compatible grammar loader (e.g., the existing `syntect` crate in Rust can consume `.tmLanguage.json` files), (2) a language-configuration reader for bracket/comment/folding rules, and (3) an extension manifest parser for the `contributes.languages` / `contributes.grammars` contract. None of the extension's own files would need to change; only the host runtime that loads them.

---

### Out-of-Partition References

- `extensions/mssql/syntaxes/SQL.plist` — upstream source file that `update-grammar.mjs:8` fetches from `github.com/microsoft/vscode-mssql`; defines the canonical SQL grammar that this partition vendors.
- `node_modules/vscode-grammar-updater` — build-time npm package invoked by `update-grammar.mjs:6-8`; handles plist-to-JSON conversion and GitHub fetch logic.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
