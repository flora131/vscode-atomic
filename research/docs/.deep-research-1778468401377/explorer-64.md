# Partition 64 of 80 — Findings

## Scope
`extensions/json/` (1 files, 39 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator Report: extensions/json/ (Partition 64 of 80)

## Overview
The `extensions/json/` directory contains a built-in VS Code extension providing JSON language support with syntax highlighting, bracket matching, and code snippet definitions. The extension is minimal in scope and infrastructure-focused rather than logic-heavy.

## Implementation

### Grammar & Syntax Definitions
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/syntaxes/JSON.tmLanguage.json` — TextMate grammar for base JSON syntax
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/syntaxes/JSONC.tmLanguage.json` — TextMate grammar for JSON with Comments variant
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/syntaxes/JSONL.tmLanguage.json` — TextMate grammar for JSON Lines format
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/syntaxes/snippets.tmLanguage.json` — TextMate grammar for VS Code code snippet syntax (314 KB, auto-generated)

### Build Automation
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/build/update-grammars.js` (39 LOC) — Script to pull and adapt grammar definitions from upstream sources (microsoft/vscode-JSON.tmLanguage and jeff-hykin/better-snippet-syntax repositories). Uses `vscode-grammar-updater` module and includes scope-name adaptation logic via `adaptJSON()` function.

## Configuration

### Extension Manifest
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/package.json` (133 LOC) — Extension entry point defining:
  - Contributes 4 languages: `json`, `jsonc`, `jsonl`, `snippets` with associated file extensions (.json, .jsonc, .code-snippets, etc.) and glob patterns
  - Contributes 4 grammar registrations mapping each language to its TextMate syntax file
  - Build script reference for grammar updates
  - Package metadata (version 10.0.0, MIT license)

### Language Configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/language-configuration.json` (84 LOC) — Editor behavior configuration:
  - Comment delimiters (line comment `//'`, block comment `/* */`)
  - Bracket pairs for auto-closing: `{}`, `[]`, `()`, quotes, backticks
  - Indentation rules (patterns for increase/decrease indent)
  - On-enter rules (auto-append `//` in line comments)

### Localization
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/package.nls.json` (4 LOC) — Human-readable display name and description for localization

### Packaging & Compliance
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/.vscodeignore` (3 LOC) — Exclude build, test, and cgmanifest files from packaged extension
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/cgmanifest.json` (28 LOC) — Component governance manifest listing third-party grammar sources:
  - microsoft/vscode-JSON.tmLanguage (commit 9bd83f1c)
  - jeff-hykin/better-snippet-syntax (commit 2b1bb124, version 1.0.2)
  Both MIT licensed.

## Notable Clusters

The extension has two distinct subsystems:
1. **Syntax & Language Metadata**: Grammar files + language-configuration.json define the lexical structure and editor behavior for JSON variants.
2. **Build Infrastructure**: Minimal Node.js automation to fetch and adapt grammars from external sources, ensuring grammar definitions stay in sync with upstream repositories.

The TextMate grammar format (`.tmLanguage.json`) is the primary artifact for syntax highlighting and tokenization, generated from upstream sources and adapted via scope-name transformations. No custom runtime logic or test files exist in this extension—it is purely declarative.

## Porting Implications

For a Tauri/Rust port, the `extensions/json/` extension highlights the dependency on TextMate grammar infrastructure. TextMate grammars are widely supported in editor frameworks (via libraries like `tree-sitter` or `oniguruma`-based tokenizers) and could be reused in a Rust-based editor. The language contributions (file extensions, mimetypes, configuration like bracket matching and indentation rules) would need to be redefined in the new system's language server or plugin architecture. The build automation (grammar updates) would require equivalent tooling in Rust to fetch and adapt upstream sources.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/json/package.json` (133 LOC)
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/language-configuration.json` (84 LOC)
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/build/update-grammars.js` (39 LOC)
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/cgmanifest.json` (28 LOC)
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/syntaxes/JSON.tmLanguage.json` (head, ~40 LOC sampled)
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/syntaxes/JSONC.tmLanguage.json` (head, ~30 LOC sampled)
- `/home/norinlavaee/projects/vscode-atomic/extensions/json/syntaxes/JSONL.tmLanguage.json` (head, ~20 LOC sampled)

---

### Per-File Notes

#### `extensions/json/package.json`

**Language registrations** (`package.json:18-104`): The manifest registers four language IDs under `contributes.languages`:

1. **`json`** (`package.json:20-49`): Maps extensions `.json`, `.bowerrc`, `.jscsrc`, `.webmanifest`, `.js.map`, `.css.map`, `.ts.map`, `.har`, `.jslintrc`, `.jsonld`, `.geojson`, `.ipynb`, `.vuerc`; filenames `composer.lock`, `.watchmanconfig`; mimetypes `application/json`, `application/manifest+json`. Configuration delegated to `./language-configuration.json`.

2. **`jsonc`** (`package.json:51-77`): "JSON with Comments". Extensions `.jsonc`, `.eslintrc`, `.eslintrc.json`, `.jsfmtrc`, `.jshintrc`, `.swcrc`, `.hintrc`, `.babelrc`, `.toolset.jsonc`; filenames `babel.config.json`, `bun.lock`, `.babelrc.json`, `.ember-cli`, `typedoc.json`; filenamePatterns `**/.github/hooks/*.json`. Configuration delegated to `./language-configuration.json`.

3. **`jsonl`** (`package.json:79-89`): "JSON Lines". Extensions `.jsonl`, `.ndjson`. Configuration delegated to `./language-configuration.json`.

4. **`snippets`** (`package.json:91-104`): "Code Snippets". Extension `.code-snippets`; filenamePatterns `**/User/snippets/*.json`, `**/User/profiles/*/snippets/*.json`, `**/snippets*.json`. Configuration delegated to `./language-configuration.json`.

**Grammar registrations** (`package.json:106-127`): Four `contributes.grammars` entries map each language ID to its TextMate grammar file and scope name:

| Language | Scope Name | Grammar Path |
|---|---|---|
| `json` | `source.json` | `./syntaxes/JSON.tmLanguage.json` |
| `jsonc` | `source.json.comments` | `./syntaxes/JSONC.tmLanguage.json` |
| `jsonl` | `source.json.lines` | `./syntaxes/JSONL.tmLanguage.json` |
| `snippets` | `source.json.comments.snippets` | `./syntaxes/snippets.tmLanguage.json` |

The `scripts` field at `package.json:11-13` defines one runnable script: `"update-grammar": "node ./build/update-grammars.js"`.

---

#### `extensions/json/language-configuration.json`

This file is shared by all four language IDs via `"configuration": "./language-configuration.json"` in each language registration.

**Comment support** (`language-configuration.json:2-8`): Defines `lineComment` as `//` and `blockComment` as `/* */`. These are the C-style comment delimiters used by JSONC; they are declared for all four languages even though standard JSON does not permit comments (the grammar files determine actual parse behaviour).

**Brackets** (`language-configuration.json:9-17`): Two bracket pairs registered: `{ }` and `[ ]`.

**Auto-closing pairs** (`language-configuration.json:19-63`): Six pairs defined:
- `{ }` — not auto-closed inside `string`
- `[ ]` — not auto-closed inside `string`
- `( )` — not auto-closed inside `string`
- `' '` — not auto-closed inside `string`
- `" "` — not auto-closed inside `string` or `comment`
- `` ` ` `` — not auto-closed inside `string` or `comment`

**Indentation rules** (`language-configuration.json:65-68`): Two regex patterns control smart indentation:
- `increaseIndentPattern` matches `{` or `[` at end of line (excluding those inside string literals) to trigger indent increase.
- `decreaseIndentPattern` `^\\s*[}\\]],?\\s*$` matches a closing `}` or `]` (optionally followed by `,`) on a line by itself to trigger indent decrease.

**OnEnter rules** (`language-configuration.json:69-83`): One rule: when the cursor is inside a line comment (`// ...`) and there is non-whitespace text after the cursor, pressing Enter appends `// ` on the new line. `indent` is set to `none` so no extra indentation is added beyond what the rule injects.

---

#### `extensions/json/build/update-grammars.js`

This is the sole TypeScript/JS build-time script (39 LOC, plain Node.js CommonJS module).

**Dependency** (`update-grammars.js:7`): Requires `vscode-grammar-updater`, an npm utility that fetches grammar files from GitHub and writes them locally.

**`adaptJSON` helper** (`update-grammars.js:9-32`): Accepts a parsed grammar object, a display name, a replacement scope suffix (e.g., `.json.comments`), and an optional `replaceeScope` (default `'json'`). The function:
1. Sets `grammar.name` to the provided display name (`update-grammars.js:10`).
2. Sets `grammar.scopeName` to `source` + the replacement scope (`update-grammars.js:11`).
3. Builds a regex (`update-grammars.js:12`) matching `.${replaceeScope}` globally.
4. Defines a recursive `fixScopeNames` closure (`update-grammars.js:13-26`) that walks every rule in the grammar tree and replaces all occurrences of `.${replaceeScope}` with the replacement scope in `name` and `contentName` string fields.
5. Iterates the grammar's `repository` keys (`update-grammars.js:28-31`) and calls `fixScopeNames` on each rule.

**Grammar update calls** (`update-grammars.js:34-39`):
- Line 34: Source repo constant `tsGrammarRepo = 'microsoft/vscode-JSON.tmLanguage'`.
- Line 35: Fetches `JSON.tmLanguage` from `microsoft/vscode-JSON.tmLanguage`, writes to `./syntaxes/JSON.tmLanguage.json` with no transformation.
- Line 36: Same source, writes to `./syntaxes/JSONC.tmLanguage.json` via `adaptJSON(..., 'JSON with Comments', '.json.comments')` — renames all `.json` scope segments to `.json.comments`.
- Line 37: Same source, writes to `./syntaxes/JSONL.tmLanguage.json` via `adaptJSON(..., 'JSON Lines', '.json.lines')`.
- Line 39: Fetches `autogenerated/jsonc.tmLanguage.json` from `jeff-hykin/better-snippet-syntax`, writes to `./syntaxes/snippets.tmLanguage.json` via `adaptJSON(..., 'Snippets', '.json.comments.snippets', 'json.comments')` — renames `.json.comments` scope segments to `.json.comments.snippets`.

**Upstream commit pins** (tracked in `cgmanifest.json`):
- `microsoft/vscode-JSON.tmLanguage` pinned to commit `9bd83f1c252b375e957203f21793316203f61f70` (`cgmanifest.json:9`).
- `jeff-hykin/better-snippet-syntax` pinned to commit `2b1bb124cb2b9c75c3c80eae1b8f3a043841d654`, version `1.0.2` (`cgmanifest.json:21-24`).

---

#### `extensions/json/syntaxes/JSON.tmLanguage.json`

Auto-generated TextMate grammar. Top-level scope `source.json` (`JSON.tmLanguage.json:9`). A single pattern at the root (`JSON.tmLanguage.json:11-13`) includes the `#value` rule from the repository. The `repository` begins at line 15 with an `array` rule (lines 17-40+) that matches `[` ... `]` with `meta.structure.array.json` and recurses via `#value` and marks illegal separators.

---

#### `extensions/json/syntaxes/JSONC.tmLanguage.json`

Scope `source.json.comments` (`JSONC.tmLanguage.json:9`). Structurally identical to `JSON.tmLanguage.json`; all scope name segments are `.json.comments` instead of `.json` (e.g., `meta.structure.array.json.comments` at line 29). Contains comment-rule patterns absent from the base JSON grammar.

---

#### `extensions/json/syntaxes/JSONL.tmLanguage.json`

Scope `source.json.lines` (`JSONL.tmLanguage.json:9`). Structurally identical derivation; scope segments use `.json.lines`.

---

### Cross-Cutting Synthesis

The `extensions/json` partition is a purely declarative, grammar-only VS Code extension with a single 39-line build script. It contributes no TypeScript language server, no activation event, and no commands — language intelligence for JSON is provided by a separate extension (`vscode-json-languageservice`, integrated in `extensions/json-language-features/`). The partition's runtime footprint consists entirely of JSON data files consumed by VS Code's TextMate tokeniser engine (vscode-textmate): four `.tmLanguage.json` grammars under `syntaxes/`, one `language-configuration.json` shared across all four language IDs, and the `package.json` manifest binding file extensions/patterns to language IDs and grammar scope names.

The grammar derivation pattern is systematic: a single canonical upstream grammar (`microsoft/vscode-JSON.tmLanguage`) is fetched once and then adapted three times via `adaptJSON` scope-renaming to produce three variants (JSONC, JSONL, snippets), with a fourth variant sourced from a separate upstream repo (`jeff-hykin/better-snippet-syntax`) for snippet-specific syntax. This means JSONC, JSONL, and snippets grammars are always structurally in sync with the base JSON grammar after `npm run update-grammar` is executed; divergence is only possible in the upstream repos themselves.

For a Tauri/Rust port, the TextMate grammar files (`.tmLanguage.json`) are format-portable — any editor that supports vscode-textmate or an equivalent Rust crate (e.g., `syntect`) can consume them directly. The `language-configuration.json` format is VS Code-specific and would need to be mapped to the target editor's equivalent bracket/indentation/comment configuration API. The build script (`update-grammars.js`) is a standalone Node.js tool used only at development time and would remain unchanged unless the grammar update pipeline itself is replaced.

---

### Out-of-Partition References

- `extensions/json-language-features/` — The companion extension that registers the JSON language server (`vscode-json-languageservice`) providing validation, completion, hover, and formatting. This partition contains no language server code.
- `src/vs/workbench/services/textMate/` — VS Code's TextMate tokenisation engine integration (vscode-textmate); consumes the `.tmLanguage.json` files registered by this extension.
- `node_modules/vscode-grammar-updater/` — Runtime dependency of `build/update-grammars.js`; not present in the extension's runtime bundle.
- `node_modules/vscode-textmate/` — The TextMate grammar engine that processes `syntaxes/*.tmLanguage.json` at runtime inside VS Code.
- `extensions/json/syntaxes/snippets.tmLanguage.json` — 314 KB auto-generated grammar sourced from `jeff-hykin/better-snippet-syntax`; not read in full for this analysis due to size.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder Partition 64

## Scope Analysis
- **Path**: `extensions/json/` (1 file scanned, 39 LOC sample)
- **Content Type**: Language extension definitions, grammar files, and syntax highlighting rules

## Findings

**SENTINEL: No relevant patterns for Tauri/Rust port.**

The scope contains only VS Code extension metadata for JSON language support:
- `package.json`: Extension manifest defining language contributions
- `language-configuration.json`: Bracket matching, indentation, auto-closing rules
- Grammar files (`.tmLanguage.json`): TextMate-compatible syntax highlighting patterns

These are static declarative configurations for language features and syntax highlighting—not implementation patterns relevant to porting VS Code's core IDE functionality to Tauri/Rust. The task orientation mark correctly identified this as "Skip."

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
