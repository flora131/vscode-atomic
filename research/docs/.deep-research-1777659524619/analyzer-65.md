# Partition 65: `extensions/json/` — JSON Grammar/Language-Config Extension

## Files Analysed

| File | Lines | Role |
|------|-------|------|
| `extensions/json/package.json` | 133 | Extension manifest; language/grammar contributions |
| `extensions/json/package.nls.json` | 4 | Display string localisation keys |
| `extensions/json/language-configuration.json` | 84 | Shared language configuration for all four language IDs |
| `extensions/json/cgmanifest.json` | 28 | Component governance — upstream git hashes for both sourced grammars |
| `extensions/json/build/update-grammars.js` | 39 | Dev-time script: pulls upstream grammars and adapts scope names |
| `extensions/json/syntaxes/JSON.tmLanguage.json` | 212 | TextMate grammar for `source.json` |
| `extensions/json/syntaxes/JSONC.tmLanguage.json` | 212 | TextMate grammar for `source.json.comments` |
| `extensions/json/syntaxes/JSONL.tmLanguage.json` | 212 | TextMate grammar for `source.json.lines` |
| `extensions/json/syntaxes/snippets.tmLanguage.json` | 7 462 | TextMate grammar for `source.json.comments.snippets` |
| `extensions/json/.vscodeignore` | 3 | Excludes `build/` and `test/` from VSIX packaging |

---

## Per-File Notes

### `package.json`

The manifest contributes four language IDs (lines 18–104) and four grammars (lines 106–127), strictly one-to-one:

- **`json`** (scope `source.json`) — activated by 14 file extensions (`.json`, `.har`, `.ipynb`, `.geojson`, etc.) and 2 named filenames (`composer.lock`, `.watchmanconfig`), plus MIME types `application/json` and `application/manifest+json`. Configuration points to `./language-configuration.json`.
- **`jsonc`** (scope `source.json.comments`) — activated by 10 extensions (`.jsonc`, `.eslintrc`, `.swcrc`, etc.), 5 filenames (`babel.config.json`, `bun.lock`, `typedoc.json`, etc.), and the glob `**/.github/hooks/*.json`.
- **`jsonl`** (scope `source.json.lines`) — activated by `.jsonl` and `.ndjson` only.
- **`snippets`** (scope `source.json.comments.snippets`) — activated by `.code-snippets` and the globs `**/User/snippets/*.json`, `**/User/profiles/*/snippets/*.json`, and `**/snippets*.json`.

The extension version is `10.0.0` (line 5). There is no `main` entry point; the extension is pure-data.

### `package.nls.json`

Two keys: `displayName` → `"JSON Language Basics"` and `description` → `"Provides syntax highlighting & bracket matching in JSON files."`. These surface in the Extensions view.

### `language-configuration.json`

A single file shared across all four language IDs via the `configuration` field in each language contribution. Key sections:

- **`comments`** (lines 2–7): declares `//` line comments and `/* */` block comments. This applies even to strict `json` — the grammar already contains the comment rules in its `#comments` repository entry (though JSON strictly disallows them, the grammar tokenises them for tolerated usage).
- **`brackets`** (lines 9–17): registers `{}` and `[]` pairs for bracket matching and auto-indentation. Note `()` is absent from this list but appears in `autoClosingPairs`.
- **`autoClosingPairs`** (lines 19–63): six pairs — `{}`, `[]`, `()`, `''`, `""`, and backtick — each with a `notIn` guard of `"string"` (and additionally `"comment"` for `"` and `` ` ``).
- **`indentationRules`** (lines 65–68): two regex patterns. `increaseIndentPattern` matches an opening `{` or `[` that is not inside a string (using a double-quoted string detector). `decreaseIndentPattern` matches lines that start with `}` or `]` optionally followed by `,`.
- **`onEnterRules`** (lines 69–83): one rule — when pressing Enter from within a `//` line comment that has non-empty text after it, appends `"// "` on the new line with `indent: "none"`. The file uses JSONC format (has inline comments at line 70) even though its name lacks `.jsonc`.

### `cgmanifest.json`

Records two upstream component registrations for Component Governance tooling:

1. `microsoft/vscode-JSON.tmLanguage` at commit `9bd83f1c252b375e957203f21793316203f61f70` — source for `JSON.tmLanguage.json`, `JSONC.tmLanguage.json`, and `JSONL.tmLanguage.json`.
2. `jeff-hykin/better-snippet-syntax` at commit `2b1bb124cb2b9c75c3c80eae1b8f3a043841d654`, version `1.0.2` — source for `snippets.tmLanguage.json`.

### `build/update-grammars.js`

A Node.js dev script (not shipped in VSIX). It imports `vscode-grammar-updater` and defines an `adaptJSON` function (lines 9–32) that performs in-place scope-name rewriting on a downloaded grammar object:

- Sets `grammar.name` and `grammar.scopeName` directly (lines 10–11).
- Builds a regex from the old scope suffix (default `'json'`) and applies it to every `name` and `contentName` string found by recursive traversal of the `repository` object (lines 12–31).

Four `updateGrammar.update()` calls (lines 35–39):

1. `JSON.tmLanguage` → `./syntaxes/JSON.tmLanguage.json` (no adaptation; raw upstream).
2. `JSON.tmLanguage` → `./syntaxes/JSONC.tmLanguage.json` with `adaptJSON(…, 'JSON with Comments', '.json.comments')` — rewrites every `.json` scope suffix to `.json.comments`.
3. `JSON.tmLanguage` → `./syntaxes/JSONL.tmLanguage.json` with `adaptJSON(…, 'JSON Lines', '.json.lines')`.
4. `jeff-hykin/better-snippet-syntax` `autogenerated/jsonc.tmLanguage.json` → `./syntaxes/snippets.tmLanguage.json` with `adaptJSON(…, 'Snippets', '.json.comments.snippets', 'json.comments')` — this variant replaces `.json.comments` rather than `.json` as the replacee scope.

### `syntaxes/JSON.tmLanguage.json`

212-line TextMate grammar (upstream commit `9bd83f1c`). Root pattern at line 10 includes `#value`. The `repository` object defines seven rules:

- **`array`** — begin/end `\[`/`\]` delimiters; inner patterns are `#value`, comma separator, and an `invalid.illegal` catch-all.
- **`comments`** — three alternations: JSDoc block (`/** … */`), block (`/* … */`), and line (`//…`). The line comment match pattern is `(//).*$\n?` with capture group 1 named `punctuation.definition.comment.json`.
- **`constant`** — single match `\b(?:true|false|null)\b` → `constant.language.json`.
- **`number`** — verbose-mode regex (line 82) matching the full JSON number grammar including optional sign, decimal, and exponent parts → `constant.numeric.json`.
- **`object`** — begin/end `\{`/`\}`; inner patterns are `#objectkey`, `#comments`, and a colon-delimited key-value entry that ends at `,` or lookahead `}`.
- **`objectkey`** — like `#string` but uses `punctuation.support.type.property-name.*` captures and the compound scope `string.json support.type.property-name.json` (line 171).
- **`stringcontent`** — matches recognised escape sequences (`[\"\\/bfnrt]` or `u[0-9a-fA-F]{4}`) as `constant.character.escape.json`; unrecognised escapes (`\\.`) as `invalid.illegal`.
- **`string`** — begin/end `"` with `#stringcontent` inside.
- **`value`** — ordered alternation: `#constant`, `#number`, `#string`, `#array`, `#object`, `#comments`.

### `syntaxes/JSONC.tmLanguage.json` and `syntaxes/JSONL.tmLanguage.json`

Both are structurally identical to `JSON.tmLanguage.json` (same 212 lines, same seven repository rules). All scope names have been mechanically rewritten by `adaptJSON`: every `.json` suffix becomes `.json.comments` (JSONC) or `.json.lines` (JSONL). The grammar logic — number regex, escape regex, object/array structures — is byte-for-byte the same. Scope names are the only difference.

### `syntaxes/snippets.tmLanguage.json`

7,462 lines sourced from `jeff-hykin/better-snippet-syntax`. Scope name is `source.json.comments.snippets`. The grammar layers full VS Code snippet variable/tabstop syntax on top of the JSONC base. Key additional repository entries visible in the initial lines include:

- **`basic_escape`** (line 44–47): standard JSON string escape, named `constant.character.escape.json.comments.snippets`.
- **`bnf_any`** (lines 48–): an extremely large single `match` pattern (spanning most of the file) that recognises the full snippet syntax grammar — tabstops (`$0`–`$N`), variables (`$TM_SELECTED_TEXT`, `$CURSOR_INDEX`, `RANDOM_HEX`, `UUID`, etc.), transform syntax (`${N:/upcase}`, `${N:+if}`, `${N:?if:else}`, `${N:-default}`), and choice syntax (`${N|opt1,opt2|}`). Capture groups assign named scopes such as `meta.insertion.tabstop`, `punctuation.section.insertion.dollar`, `variable.other.normal`, and `keyword.operator.insertion` under the `.json.comments.snippets` namespace.

---

## Cross-Cutting Synthesis

This extension is pure static data — no TypeScript entry point, no Node.js runtime. It contributes only language registration records and TextMate grammar files consumed by the VS Code host's built-in tokenisation engine (vscode-textmate / oniguruma). When porting to Tauri/Rust, the grammars themselves are host-agnostic; any runtime that can load `.tmLanguage.json` and execute Oniguruma regex patterns (e.g., the `syntect` crate used by many Rust editors) can consume these files without modification.

The `language-configuration.json` spec is a VS Code–specific format. Its bracket, auto-close, indentation-rule, and `onEnterRules` fields map roughly to the language-server-agnostic capabilities that Tauri-side editor components must re-implement independently. The indentation regex pair at lines 66–67 and the `onEnterRules` entry at lines 70–83 are the only behavioural logic in the entire partition; everything else is declarative.

All language intelligence — completion, hover, diagnostics, schema validation — is entirely absent here and lives in the separate `extensions/json-language-features/` extension. The `languageParticipants.ts` file in that extension (`extensions/json-language-features/client/src/languageParticipants.ts:40–45`) hard-codes `'json'`, `'jsonc'`, and `'snippets'` as default LSP participants, consuming the language IDs registered here. The `jsonl` language ID is intentionally excluded from LSP participation. This two-extension split — grammar/config here, LSP there — is a deliberate VS Code architectural boundary that a Tauri port must preserve or re-combine according to the host editor's plugin model.

---

## Out-of-Partition References

- `/home/norinlavaee/projects/vscode-atomic/extensions/json-language-features/client/src/languageParticipants.ts` — hardcodes `'json'`, `'jsonc'`, `'snippets'` as default LSP language participants (lines 40–45); `'jsonl'` is absent.
- `/home/norinlavaee/projects/vscode-atomic/extensions/json-language-features/client/src/jsonClient.ts` — JSON LSP client that activates for the language IDs registered by this extension.
- `/home/norinlavaee/projects/vscode-atomic/extensions/vscode-colorize-tests/test/colorize-results/test_json.json` and `test_code-snippets.json` — integration test baselines that reference tokenisation output from `source.json` and `source.json.comments.snippets`.
- `/home/norinlavaee/projects/vscode-atomic/extensions/php/syntaxes/php.tmLanguage.json` and `extensions/markdown-basics/syntaxes/markdown.tmLanguage.json` — embed `source.json` as a grammar injection scope for code-fence and heredoc content.
