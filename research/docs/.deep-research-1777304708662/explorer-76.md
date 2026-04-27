# Partition 76 of 79 — Findings

## Scope
`extensions/rust/` (1 files, 9 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locations: Porting VS Code Core IDE Functionality from TypeScript/Electron to Tauri/Rust
## Partition 76 of 79 — `extensions/rust/`

### Implementation
- `extensions/rust/package.json` — VS Code built-in Rust language support extension; defines language contribution for `.rs` files with grammar and language configuration
- `extensions/rust/build/update-grammar.mjs` — Build script that updates Rust syntax highlighting grammar from upstream dustypomerleau/rust-syntax repository
- `extensions/rust/language-configuration.json` — Language configuration metadata for Rust syntax support (indentation, brackets, comments, etc.)
- `extensions/rust/syntaxes/rust.tmLanguage.json` — TextMate grammar definition for Rust syntax highlighting; provides semantic coloring rules

### Configuration
- `extensions/rust/cgmanifest.json` — Component governance manifest declaring upstream Rust syntax dependency (dustypomerleau/rust-syntax)
- `extensions/rust/.vscodeignore` — NPM package ignore rules for the extension

---

## Summary

The `extensions/rust/` directory contains a minimal built-in language support extension for Rust, consisting of 7 files focused entirely on syntax highlighting and language configuration. This is a **language grammar extension only** — it provides no IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation) and therefore has minimal relevance to a Tauri/Rust port. The extension demonstrates how VS Code plugs in language support via TextMate grammars, but porting core IDE functionality to Tauri would require substantially more investigation into the extensibility APIs, LSP integration, debug adapters, and the Electron/TypeScript core that implements actual editing and intelligence features.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

1. `extensions/rust/package.json` — Extension manifest declaring language contribution for `.rs` files
2. `extensions/rust/language-configuration.json` — Language editor configuration (brackets, comments, indentation, folding)
3. `extensions/rust/syntaxes/rust.tmLanguage.json` — TextMate grammar (1188 lines) providing tokenisation/syntax highlighting rules
4. `extensions/rust/build/update-grammar.mjs` — Build-time grammar sync script
5. `extensions/rust/cgmanifest.json` — Component governance manifest for the upstream grammar dependency
6. `extensions/rust/package.nls.json` — NLS strings for display name and description
7. `extensions/rust/.vscodeignore` — Package exclusion rules

---

### Per-File Notes

#### `extensions/rust/package.json` (42 lines)

- Declares extension `name: "rust"`, `version: "10.0.0"`, `publisher: "vscode"` at lines 2–6.
- Under `contributes.languages` (lines 16–28) registers the language id `"rust"` for file extension `.rs` and points to `./language-configuration.json`.
- Under `contributes.grammars` (lines 29–35) registers the TextMate grammar at `./syntaxes/rust.tmLanguage.json` with scope name `source.rust`.
- Defines a single npm script `"update-grammar": "node ./build/update-grammar.mjs"` at line 12.
- No runtime JavaScript code; the extension contributes purely declarative JSON-based metadata consumed directly by VS Code's extension host.

#### `extensions/rust/language-configuration.json` (91 lines)

- `comments` block (lines 2–8): declares `//` as line comment and `/* */` as block comment delimiters. These strings are used by VS Code's comment-toggle commands.
- `brackets` (lines 9–22): declares three matched pairs — `{}`, `[]`, `()` — enabling bracket-matching highlighting in the editor.
- `autoClosingPairs` (lines 23–43): same three bracket pairs plus `"..."` with `notIn: ["string"]` guard — drives the auto-close-pair editing behaviour.
- `surroundingPairs` (lines 44–65): `{}`, `[]`, `()`, `""`, `<>` — controls which characters wrap a selection when typed.
- `indentationRules` (lines 66–69): `increaseIndentPattern` and `decreaseIndentPattern` as regex strings; VS Code's auto-indent engine evaluates these against the preceding/following line on Enter.
- `folding.markers` (lines 70–75): start `^\\s*//\\s*#?region\\b` / end `^\\s*//\\s*#?endregion\\b` — enables `// #region` / `// #endregion` folding.
- `onEnterRules` (lines 76–90): one rule that appends `"// "` on Enter when the cursor is inside a line comment, matching `beforeText` pattern `\/\/.*` and non-empty `afterText`.

#### `extensions/rust/syntaxes/rust.tmLanguage.json` (1188 lines)

- Top-level `scopeName: "source.rust"` at line 9; consumed by VS Code's tokeniser to match language grammars to files.
- `patterns` array (lines 10–end) contains top-level match rules; a `repository` section (not shown in the excerpt) defines reusable rule groups such as `#block-comments`, `#comments`, `#gtypes`, `#lvariables`, `#lifetimes`, `#punctuation`, `#types`, `#keywords`, `#escapes`, `#interpolations`, `#strings`, `#variables`.
- Representative top-level patterns observed in lines 11–100:
  - Lines 11–51: boxed-slice literal rule using begin/end regex `(<)(\\[)` … `>` with captured groups mapped to scope names `punctuation.brackets.angle.rust`, `punctuation.brackets.square.rust`.
  - Lines 52–78: macro type metavariable rule (`$IDENT:specifier`) matching `(\\$)((crate)|([A-Z]\\w*))(…)` and assigning scopes `keyword.operator.macro.dollar.rust`, `entity.name.type.metavariable.rust`, etc.
  - Lines 79–100: macro value metavariable rule (`$ident:specifier`) assigning `variable.other.metavariable.name.rust`.
- Repository entries observed in lines 1100–1187:
  - `strings` section (lines ~1088–1163): begin/end rules for byte strings, raw strings (`b?r#*"…"#*`), and character literals (`b?'…'`), all including `#escapes` and `#interpolations`.
  - `lvariables` (lines 1165–1178): two simple match rules for `self`/`Self` → `variable.language.self.rust` and `super` → `variable.language.super.rust`.
  - `variables` (lines 1179–1187): catch-all pattern `\\b(?<!(?<!\\.)\\.)(?:r#(?!(crate|[Ss]elf|super)))?[a-z0-9_]+\\b` → `variable.other.rust`.
- The grammar is a verbatim copy from `https://github.com/dustypomerleau/rust-syntax/commit/268fd42cfd4aa96a6ed9024a2850d17d6cd2dc7b` (line 7) and is not hand-authored.

#### `extensions/rust/build/update-grammar.mjs` (9 lines)

- Line 7: `import * as vscodeGrammarUpdater from 'vscode-grammar-updater'` — pulls in a shared build utility from the VS Code monorepo toolchain.
- Line 9: calls `vscodeGrammarUpdater.update('dustypomerleau/rust-syntax', 'syntaxes/rust.tmLanguage.json', './syntaxes/rust.tmLanguage.json', undefined, 'main')` — fetches the grammar JSON from the `main` branch of the upstream GitHub repository, writing the result to the local `./syntaxes/rust.tmLanguage.json` file.
- No other logic; the script is invoked only during development/maintenance via `npm run update-grammar`.

#### `extensions/rust/cgmanifest.json` (18 lines)

- Records a single `registrations` entry (lines 3–15) of `type: "git"` with:
  - `repositoryUrl: "https://github.com/dustypomerleau/rust-syntax"` (line 8)
  - `commitHash: "268fd42cfd4aa96a6ed9024a2850d17d6cd2dc7b"` (line 9)
  - `version: "0.6.1"` (line 14), `license: "MIT"` (line 12)
- This manifest is consumed by Microsoft's component governance tooling to track open-source dependencies; it plays no runtime role.

#### `extensions/rust/package.nls.json` (4 lines)

- Provides English localisation strings for two keys:
  - `"displayName": "Rust Language Basics"` (line 2)
  - `"description": "Provides syntax highlighting and bracket matching in Rust files."` (line 3)
- These values substitute the `%displayName%` and `%description%` tokens in `package.json` at lines 3–4.

#### `extensions/rust/.vscodeignore` (3 lines)

- Excludes `test/**`, `cgmanifest.json`, and `build/**` from the packaged extension artifact. These paths are only needed during development or governance auditing.

---

### Cross-Cutting Synthesis

The `extensions/rust/` partition contains a minimal, declarative language support extension for Rust that operates entirely through VS Code's built-in extension contribution points. There is no TypeScript runtime code, no LSP client, no debugger adapter, and no terminal or source control integration. The extension contributes two things: (1) a `language-configuration.json` consumed by VS Code's core editor to drive bracket matching, auto-close, auto-indent, folding, and comment toggling for `.rs` files; and (2) a TextMate grammar (`rust.tmLanguage.json`, 1188 lines sourced from `dustypomerleau/rust-syntax`) consumed by VS Code's tokeniser to produce syntax scope annotations for `.rs` files. The grammar update workflow is a single-call build script that pulls the upstream file via the shared `vscode-grammar-updater` utility. From a Tauri/Rust porting perspective, this partition contributes nothing architectural — it contains no Electron-specific code, no Node.js APIs, and no TypeScript logic. Porting concerns reside entirely at the host layer (the tokeniser engine that interprets TextMate grammars, and the editor behaviour engine that reads `language-configuration.json`), neither of which is implemented here.

---

### Out-of-Partition References

- `vscode-grammar-updater` — npm package imported in `extensions/rust/build/update-grammar.mjs:7`; the implementation lives in the VS Code monorepo build toolchain, outside `extensions/rust/`.
- `https://github.com/dustypomerleau/rust-syntax` — upstream external repository; `syntaxes/rust.tmLanguage.json` is a vendored copy pinned to commit `268fd42cfd4aa96a6ed9024a2850d17d6cd2dc7b`.
- VS Code's TextMate tokenisation engine — not in this partition; processes `source.rust` scope assignments from `rust.tmLanguage.json`.
- VS Code's language configuration host — not in this partition; reads `language-configuration.json` to drive editor bracket/indent/folding behaviour.

---

This document was produced by reading all 7 files in the `extensions/rust/` partition in full. The partition contains no implementation code relevant to porting VS Code's core IDE functionality to Tauri/Rust — it is a static grammar and editor-configuration bundle only. All file:line references above are drawn directly from the file contents read during this analysis.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Findings: extensions/rust/

## Scope Analysis
The `extensions/rust/` directory contains a minimal language extension package (9 LOC across source files) providing Rust syntax highlighting and bracket matching. This partition contains no runtime code relevant to porting VS Code's core IDE functionality to Tauri/Rust—only declarative language contribution definitions.

---

#### Pattern: Language Extension Contribution Package Structure
**Where:** `extensions/rust/package.json:1-41`
**What:** Declares a language extension with grammar and language configuration contributions to VS Code.
```json
{
  "name": "rust",
  "displayName": "%displayName%",
  "description": "%description%",
  "version": "10.0.0",
  "publisher": "vscode",
  "license": "MIT",
  "engines": {
    "vscode": "*"
  },
  "scripts": {
    "update-grammar": "node ./build/update-grammar.mjs"
  },
  "categories": ["Programming Languages"],
  "contributes": {
    "languages": [
      {
        "id": "rust",
        "extensions": [".rs"],
        "aliases": ["Rust", "rust"],
        "configuration": "./language-configuration.json"
      }
    ],
    "grammars": [
      {
        "language": "rust",
        "path": "./syntaxes/rust.tmLanguage.json",
        "scopeName": "source.rust"
      }
    ]
  }
}
```
**Variations / call-sites:** Identical pattern repeated across all built-in language extensions (`extensions/html/`, `extensions/javascript/`, etc.).

---

#### Pattern: Language Configuration (Bracket Matching, Auto-closing, Indentation)
**Where:** `extensions/rust/language-configuration.json:1-92`
**What:** Defines editor behaviors for Rust: bracket matching, auto-closing pairs, indentation rules, and on-enter rules.
```json
{
  "comments": {
    "lineComment": "//",
    "blockComment": ["/*", "*/"]
  },
  "brackets": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"]
  ],
  "autoClosingPairs": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
    {"open": "\"", "close": "\"", "notIn": ["string"]}
  ],
  "surroundingPairs": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
    ["\"", "\""],
    ["<", ">"]
  ],
  "indentationRules": {
    "increaseIndentPattern": "^.*\\{[^}\"']*$|^.*\\([^\\)\"']*$",
    "decreaseIndentPattern": "^\\s*(\\s*\\/[*].*[*]\\/\\s*)*[})]"
  },
  "folding": {
    "markers": {
      "start": "^\\s*//\\s*#?region\\b",
      "end": "^\\s*//\\s*#?endregion\\b"
    }
  },
  "onEnterRules": [
    {
      "beforeText": {"pattern": "\/\/.*"},
      "afterText": {"pattern": "^(?!\\s*$).+"},
      "action": {"indent": "none", "appendText": "// "}
    }
  ]
}
```
**Variations / call-sites:** Present in all language extensions; defines core editor navigation and editing behaviors.

---

#### Pattern: TextMate Grammar with External Source
**Where:** `extensions/rust/syntaxes/rust.tmLanguage.json:1-30`
**What:** TextMate-compatible grammar definition sourced from external repository, with contributor attribution.
```json
{
  "information_for_contributors": [
    "This file has been converted from https://github.com/dustypomerleau/rust-syntax/blob/master/syntaxes/rust.tmLanguage.json",
    "If you want to provide a fix or improvement, please create a pull request against the original repository.",
    "Once accepted there, we are happy to receive an update request."
  ],
  "version": "https://github.com/dustypomerleau/rust-syntax/commit/268fd42cfd4aa96a6ed9024a2850d17d6cd2dc7b",
  "name": "Rust",
  "scopeName": "source.rust",
  "patterns": [
    {
      "comment": "boxed slice literal",
      "begin": "(<)(\\[)",
      "beginCaptures": {
        "1": {"name": "punctuation.brackets.angle.rust"},
        "2": {"name": "punctuation.brackets.square.rust"}
      },
      "end": ">",
      "endCaptures": {"0": {"name": "punctuation.brackets.angle.rust"}},
      "patterns": [{"include": "#block-comments"}]
    }
  ]
}
```
**Variations / call-sites:** Grammar updater script at `extensions/rust/build/update-grammar.mjs:9` pulls updates from upstream.

---

#### Pattern: Grammar Update Automation Script
**Where:** `extensions/rust/build/update-grammar.mjs:1-10`
**What:** Node.js script using `vscode-grammar-updater` to sync grammar from upstream Rust syntax repository.
```javascript
import * as vscodeGrammarUpdater from 'vscode-grammar-updater';

vscodeGrammarUpdater.update('dustypomerleau/rust-syntax', 'syntaxes/rust.tmLanguage.json', './syntaxes/rust.tmLanguage.json', undefined, 'main');
```
**Variations / call-sites:** `package.json:12` defines npm script `update-grammar` to invoke this build file.

---

#### Pattern: Localization Configuration
**Where:** `extensions/rust/package.nls.json:1-4`
**What:** Translation keys for language extension display name and description.
```json
{
  "displayName": "Rust Language Basics",
  "description": "Provides syntax highlighting and bracket matching in Rust files."
}
```
**Variations / call-sites:** Paired with `package.json` use of `%displayName%` and `%description%` placeholders for localization.

---

## Summary

The `extensions/rust/` partition contains **only language extension declarative metadata** with no runtime code. It demonstrates VS Code's extension architecture for language contributions: a minimal package declares language identifiers, file associations, grammar sources (TextMate JSON), and editor behavior rules (bracket matching, indentation, auto-closing pairs). All patterns are **configuration-driven and platform-agnostic**—they specify *what* the editor should do (e.g., "auto-close braces") without implementation details. No C++, TypeScript, or Electron/Tauri-specific logic is present. This reflects the separation between VS Code's core editor engine and pluggable language support, where porting to Tauri would require reimplementing the declarative-to-behavior translation layer in Rust, not porting code from this extension.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
