# Pattern Analysis: VS Code JSON Extension

## Scope
- `extensions/json/` (4 configuration files, 39 LOC in update-grammars.js)

## Patterns Found

#### Pattern 1: Extension Manifest Declaration
**Where:** `extensions/json/package.json:2-16`
**What:** Declares language extension metadata with VS Code compatibility version pinning.
```json
{
  "name": "json",
  "displayName": "%displayName%",
  "description": "%description%",
  "version": "10.0.0",
  "publisher": "vscode",
  "license": "MIT",
  "engines": {
    "vscode": "0.10.x"
  }
}
```
**Variations:** Version constraints use semver ranges (0.10.x); localization keys use percent-wrapped identifiers (%displayName%).

#### Pattern 2: Multi-Format Language Registration
**Where:** `extensions/json/package.json:18-104`
**What:** Registers multiple related language variants (JSON, JSONC, JSONL, snippets) with distinct file mappings and shared language configuration.
```json
"languages": [
  {
    "id": "json",
    "aliases": ["JSON", "json"],
    "extensions": [".json", ".bowerrc", ".jscsrc", ".webmanifest", ".js.map", ".css.map", ".ts.map", ".har", ".jslintrc", ".jsonld", ".geojson", ".ipynb", ".vuerc"],
    "filenames": ["composer.lock", ".watchmanconfig"],
    "mimetypes": ["application/json", "application/manifest+json"],
    "configuration": "./language-configuration.json"
  },
  {
    "id": "jsonc",
    "aliases": ["JSON with Comments"],
    "extensions": [".jsonc", ".eslintrc", ".eslintrc.json", ".jsfmtrc", ".jshintrc", ".swcrc", ".hintrc", ".babelrc", ".toolset.jsonc"],
    "filenames": ["babel.config.json", "bun.lock", ".babelrc.json", ".ember-cli", "typedoc.json"],
    "filenamePatterns": ["**/.github/hooks/*.json"],
    "configuration": "./language-configuration.json"
  }
]
```
**Variations:** File discovery uses three methods: extensions array (e.g., .json), filenames array (e.g., composer.lock), and glob filenamePatterns (e.g., **/.github/hooks/*.json).

#### Pattern 3: Grammar Contribution and Scope Mapping
**Where:** `extensions/json/package.json:106-127`
**What:** Maps TextMate grammars to language IDs with hierarchical scope names for syntax highlighting variants.
```json
"grammars": [
  {
    "language": "json",
    "scopeName": "source.json",
    "path": "./syntaxes/JSON.tmLanguage.json"
  },
  {
    "language": "jsonc",
    "scopeName": "source.json.comments",
    "path": "./syntaxes/JSONC.tmLanguage.json"
  }
]
```
**Variations:** Scope names form a hierarchy (source.json → source.json.comments → source.json.lines → source.json.comments.snippets) allowing theme and rule inheritance.

#### Pattern 4: Language Configuration Rules
**Where:** `extensions/json/language-configuration.json:2-83`
**What:** Defines auto-closing pairs, bracket matching, indentation rules, and contextual enter behavior via regex patterns.
```json
{
  "comments": {
    "lineComment": "//",
    "blockComment": ["/*", "*/"]
  },
  "brackets": [["{", "}"], ["[", "]"]],
  "autoClosingPairs": [
    {
      "open": "{",
      "close": "}",
      "notIn": ["string"]
    },
    {
      "open": "\"",
      "close": "\"",
      "notIn": ["string", "comment"]
    }
  ],
  "indentationRules": {
    "increaseIndentPattern": "({+(?=((\\\\.|[^\"\\\\])*\"(\\\\.|[^\"\\\\])*\")*[^\"}]*)$)|(\\[+(?=((\\\\.|[^\"\\\\])*\"(\\\\.|[^\"\\\\])*\")*[^\"\\]]*)$)",
    "decreaseIndentPattern": "^\\s*[}\\]],?\\s*$"
  }
}
```
**Variations:** Auto-closing pairs specify context via notIn array (excludes triggering in strings, comments); indentation uses lookahead regex to detect structure boundaries.

#### Pattern 5: Grammar Build-Time Adaptation
**Where:** `extensions/json/build/update-grammars.js:9-32`
**What:** Transforms base JSON grammar through scope name replacement to create variant grammars (JSONC, JSONL, Snippets) via recursive object traversal.
```javascript
function adaptJSON(grammar, name, replacementScope, replaceeScope = 'json') {
  grammar.name = name;
  grammar.scopeName = `source${replacementScope}`;
  const regex = new RegExp(`\\.${replaceeScope}`, 'g');
  var fixScopeNames = function (rule) {
    if (typeof rule.name === 'string') {
      rule.name = rule.name.replace(regex, replacementScope);
    }
    if (typeof rule.contentName === 'string') {
      rule.contentName = rule.contentName.replace(regex, replacementScope);
    }
    for (var property in rule) {
      var value = rule[property];
      if (typeof value === 'object') {
        fixScopeNames(value);
      }
    }
  };
  var repository = grammar.repository;
  for (var key in repository) {
    fixScopeNames(repository[key]);
  }
}
```
**Variations:** Deep tree walk using property iteration; scope replacement via regex with dynamic scope suffix (e.g., .json.comments, .json.lines).

#### Pattern 6: External Grammar Sourcing and Updates
**Where:** `extensions/json/build/update-grammars.js:34-39`
**What:** Pulls TextMate grammars from external git repositories and applies transformations via npm module vscode-grammar-updater.
```javascript
var updateGrammar = require('vscode-grammar-updater');
var tsGrammarRepo = 'microsoft/vscode-JSON.tmLanguage';
updateGrammar.update(tsGrammarRepo, 'JSON.tmLanguage', './syntaxes/JSON.tmLanguage.json');
updateGrammar.update(tsGrammarRepo, 'JSON.tmLanguage', './syntaxes/JSONC.tmLanguage.json', grammar => adaptJSON(grammar, 'JSON with Comments', '.json.comments'));
updateGrammar.update('jeff-hykin/better-snippet-syntax', 'autogenerated/jsonc.tmLanguage.json', './syntaxes/snippets.tmLanguage.json', grammar => adaptJSON(grammar, 'Snippets', '.json.comments.snippets', 'json.comments'));
```
**Variations:** Supports callback transformation during update; sources from multiple repos (microsoft/vscode-JSON.tmLanguage and jeff-hykin/better-snippet-syntax).

---

## Porting Implications

The JSON extension demonstrates several critical VS Code IDE patterns that would require architectural changes for Tauri/Rust:

1. **Extension Manifest System**: The package.json contributes pattern requires a runtime contribution system that dynamically registers languages, file associations, and grammars. A Tauri-based IDE would need either plugin-like binary extension support or a declarative configuration loader.

2. **TextMate Grammar Integration**: VS Code's entire syntax highlighting pipeline depends on TextMate grammar (tmLanguage.json) compilation and scope-based theme application. Porting would require either embedding a TextMate grammar engine in Rust (e.g., using `tree-sitter` or similar) or maintaining a grammar interpretation layer.

3. **Dynamic File Association**: The multi-level file discovery (extensions, filenames, filenamePatterns via glob) would map to Rust pattern matching, but the runtime flexibility of glob patterns and dynamic registration would need a maintained glob library and refresh mechanism.

4. **Language Configuration Metadata**: The declarative rules for auto-closing, bracket matching, and indentation via regex patterns exist across extension boundaries. Porting requires either: (a) moving all rules into core Rust code with compiled regex, or (b) building a rule engine that interprets JSON configuration at runtime.

5. **Build-Time Grammar Transformation**: The update-grammars.js script shows that VS Code extensions expect to perform build-time transformations on grammars. A Tauri port would need to either provide equivalent build tooling or move grammar processing entirely to compile-time procedures.

6. **External Grammar Sourcing**: The vscode-grammar-updater dependency shows extensions assume the ability to pull external resources during installation/build. This would require network access policies, caching, and versioning mechanisms in a Rust-based system.
