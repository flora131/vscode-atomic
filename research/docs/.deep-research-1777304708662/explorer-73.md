# Partition 73 of 79 — Findings

## Scope
`extensions/objective-c/` (1 files, 14 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Objective-C Extension - File Locator Report

## Configuration
- `extensions/objective-c/package.json` - Extension manifest with language contributions
- `extensions/objective-c/language-configuration.json` - Language configuration for Objective-C
- `extensions/objective-c/package.nls.json` - Localization strings
- `extensions/objective-c/cgmanifest.json` - Component governance manifest
- `extensions/objective-c/.vscodeignore` - VSCode packaging ignore file

## Implementation
- `extensions/objective-c/build/update-grammars.js` - Build script for grammar updates

## Examples / Fixtures
- `extensions/objective-c/syntaxes/objective-c.tmLanguage.json` - TextMate grammar for Objective-C
- `extensions/objective-c/syntaxes/objective-c++.tmLanguage.json` - TextMate grammar for Objective-C++

---

The `extensions/objective-c/` directory contains a grammar-only language extension with no runtime code to port. It defines syntax highlighting for `.m` and `.mm` files through TextMate grammar files and language configuration, along with build automation for grammar updates. This is a declarative extension that provides syntax support without any executable logic—no TypeScript/JavaScript runtime components, no Electron integration, and no core IDE functionality implementation to migrate to Tauri/Rust.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
# Analysis: extensions/objective-c — Grammar-Only Language Extension

> Scope: `extensions/objective-c/` (8 files). No TypeScript/JavaScript runtime code beyond the grammar-update build script. Research question framing (Tauri/Rust port) is addressed in the Cross-Cutting Synthesis section.

---

### Files Analysed

| File | Role |
|---|---|
| `extensions/objective-c/package.json` | Extension manifest; declares language IDs, file associations, grammar paths |
| `extensions/objective-c/language-configuration.json` | Editor behaviour config (comments, brackets, auto-close, on-enter rules) |
| `extensions/objective-c/package.nls.json` | NLS string table for manifest placeholders |
| `extensions/objective-c/cgmanifest.json` | Component governance manifest; records upstream git sources and commit hashes |
| `extensions/objective-c/.vscodeignore` | Extension packaging exclusion list |
| `extensions/objective-c/build/update-grammars.js` | Dev-time script; fetches upstream grammar files via `vscode-grammar-updater` |
| `extensions/objective-c/syntaxes/objective-c.tmLanguage.json` | TextMate grammar for Objective-C (`.m`); scope root `source.objc`; 3 754 lines, 53 repository patterns |
| `extensions/objective-c/syntaxes/objective-c++.tmLanguage.json` | TextMate grammar for Objective-C++ (`.mm`); scope root `source.objcpp`; 7 246 lines, 55 repository patterns |

---

### Per-File Notes

#### `/Users/norinlavaee/vscode-atomic/extensions/objective-c/package.json`

- **Role:** VS Code extension manifest. Registers two language identifiers and two TextMate grammars with the host extension system.
- **Key symbols:**
  - `contributes.languages[0]` (line 17–26): declares `objective-c`, extension `.m`, alias `"Objective-C"`, points configuration to `./language-configuration.json`.
  - `contributes.languages[1]` (line 27–36): declares `objective-cpp`, extension `.mm`, alias `"Objective-C++"`, same configuration file.
  - `contributes.grammars[0]` (line 39–43): binds `objective-c` language to `source.objc` scope, path `./syntaxes/objective-c.tmLanguage.json`.
  - `contributes.grammars[1]` (line 44–48): binds `objective-cpp` language to `source.objcpp` scope, path `./syntaxes/objective-c++.tmLanguage.json`.
  - `scripts.update-grammar` (line 12): `"node ./build/update-grammars.js"` — the sole executable script.
- **Control flow:** No runtime control flow. The manifest is read declaratively by the VS Code extension host at activation time.
- **Data flow:** The extension host reads `contributes.languages` to register file-extension-to-language-ID mappings. It reads `contributes.grammars` to load the TextMate grammar JSON into the tokenization engine for syntax highlighting. The `configuration` field directs the host to `language-configuration.json` for editor-behaviour rules.
- **Dependencies:** `engines.vscode: "*"` — no minimum version constraint. No npm `dependencies` or `devDependencies` declared; `vscode-grammar-updater` is consumed only by the build script and is resolved from the workspace root.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/objective-c/language-configuration.json`

- **Role:** Declarative editor-behaviour configuration consumed by VS Code's language service layer. Governs tokenizer-agnostic features: comment toggling, bracket matching/auto-close/surround, and on-enter rules.
- **Key symbols:**
  - `comments` (lines 2–7): `lineComment: "//"`, `blockComment: ["/*", "*/"]`. Used by the Toggle Line Comment and Toggle Block Comment commands.
  - `brackets` (lines 9–22): three pairs — `{}`, `[]`, `()`. Used by bracket-matching highlighting.
  - `autoClosingPairs` (lines 23–49): same three bracket pairs plus `"` and `'` (both with `notIn: ["string"]` guard to prevent double-closing inside string literals).
  - `surroundingPairs` (lines 51–72): five pairs (`{}`, `[]`, `()`, `""`, `''`) used when text is selected and a delimiter is typed.
  - `onEnterRules[0]` (lines 73–87): regex rule that appends `"// "` when Enter is pressed at the end of a line that begins with `//` and has non-whitespace after it (`afterText` pattern `^(?!\s*$).+`). The `indent` action is `"none"`, so indentation is not altered.
- **Control flow:** Purely declarative JSON; no imperative logic. The VS Code language service reads this file once at language activation and applies the rules reactively on editor events (keystrokes, bracket input, Enter).
- **Data flow:** `lineComment`/`blockComment` values flow into the comment-command handlers. `brackets` flows into the bracket-colouriser and matchBrackets API. `autoClosingPairs` flows into the auto-close subsystem of the editor model. `surroundingPairs` flows into the surround-with-character handler. `onEnterRules` flows into the `EnterAction` computation path in `src/vs/editor/common/languages/languageConfiguration.ts` (outside this partition).
- **Dependencies:** None beyond the VS Code language configuration schema.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/objective-c/package.nls.json`

- **Role:** NLS (National Language Support) string table providing English values for the `%displayName%` and `%description%` placeholders in `package.json`.
- **Key symbols:**
  - `displayName` (line 2): `"Objective-C Language Basics"`.
  - `description` (line 3): `"Provides syntax highlighting and bracket matching in Objective-C files."`.
- **Control flow:** Resolved by the extension host NLS system at manifest load time; no runtime execution.
- **Data flow:** The extension host substitutes `%displayName%` and `%description%` tokens in `package.json` with values from this file before presenting extension metadata in the UI.
- **Dependencies:** None.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/objective-c/cgmanifest.json`

- **Role:** Component governance manifest read by Microsoft's internal component-detection pipeline. Records the provenance (upstream Git repository and pinned commit hash) of vendored grammar files.
- **Key symbols:**
  - `registrations[0]` (lines 3–18): records `jeff-hykin/better-objcpp-syntax` at commit `5a7eb15eee382dd5aa388bc04fdb60a0d2128e14`, version `0.1.0`, MIT licence.
  - `registrations[1]` (lines 19–29): records `jeff-hykin/better-objc-syntax` at commit `119b75fb1f4d3e8726fa62588e3b935e0b719294`, version `0.2.0`, MIT licence.
  - Both `description` fields (lines 15, 28) attribute derivation from `jeff-hykin/cpp-textmate-grammar`.
- **Control flow:** No runtime use. Read only by CI/governance tooling.
- **Data flow:** The pinned commit hashes in this file are the same hashes embedded in the `version` field at line 7 of each `syntaxes/*.tmLanguage.json` file, providing a traceable link from vendored grammar content back to its upstream source.
- **Dependencies:** None at runtime.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/objective-c/.vscodeignore`

- **Role:** Instructs the `vsce` packaging tool to exclude specific directories from the published `.vsix` bundle.
- **Key symbols:**
  - Line 1: `test/**` — excludes any test directory.
  - Line 2: `cgmanifest.json` — excludes the governance manifest (not needed at runtime).
  - Line 3: `build/**` — excludes the `build/` directory containing `update-grammars.js`.
- **Control flow:** Read by `vsce package`; no runtime effect.
- **Data flow:** Determines which files are absent from the installed extension directory on end-user machines. Grammar files under `syntaxes/` and `language-configuration.json` are not excluded and are therefore present at runtime.
- **Dependencies:** None.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/objective-c/build/update-grammars.js`

- **Role:** Dev-time Node.js script. Downloads the latest autogenerated TextMate grammar files from two upstream GitHub repositories and writes them into the local `syntaxes/` directory.
- **Key symbols:**
  - `updateGrammar` (line 7): `require('vscode-grammar-updater')` — external dev utility from the VS Code monorepo toolchain.
  - `updateGrammars()` (lines 9–12): async function with two sequential `await` calls.
  - First `await` (line 10): fetches `jeff-hykin/better-objcpp-syntax` → file `autogenerated/objcpp.tmLanguage.json` on branch `master` → writes to `./syntaxes/objective-c++.tmLanguage.json`.
  - Second `await` (line 11): fetches `jeff-hykin/better-objc-syntax` → file `autogenerated/objc.tmLanguage.json` on branch `master` → writes to `./syntaxes/objective-c.tmLanguage.json`.
  - `updateGrammars()` (line 14): top-level invocation; no error handling wrapping.
- **Control flow:** Sequential async execution. First the Objective-C++ grammar is updated, then the Objective-C grammar. Both calls pass `undefined` as the fourth argument (transformation callback), meaning no post-download mutation of the grammar JSON is applied.
- **Data flow:** `vscode-grammar-updater.update()` performs an HTTP fetch from GitHub raw content endpoints using the repository name, file path, and branch. The downloaded JSON is written directly to the destination path under `syntaxes/`. The resulting files carry the upstream commit hash in their `version` field, which must match the hash recorded in `cgmanifest.json`.
- **Dependencies:** `vscode-grammar-updater` (resolved from monorepo `node_modules`; not declared in this extension's own `package.json`). Network access to `raw.githubusercontent.com` at script execution time.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/objective-c/syntaxes/objective-c.tmLanguage.json`

- **Role:** Vendored TextMate grammar providing tokenization rules for Objective-C source files (`.m`). Consumed by VS Code's built-in TextMate tokenizer engine.
- **Key symbols:**
  - `scopeName: "source.objc"` (line 9): root scope injected into the token stream.
  - `patterns` array (lines 10–n): top-level include references that delegate to named rules in `repository`.
  - `repository` (object): 53 named pattern rules covering Objective-C constructs (method declarations, message sends, preprocessor directives, literals, keywords, etc.).
  - `version` (line 7): embeds the upstream commit hash `119b75fb1f4d3e8726fa62588e3b935e0b719294`, matching `cgmanifest.json` registration.
- **Control flow:** Interpreted at tokenization time by the VS Code TextMate tokenizer (`vscode-textmate` library). Each line of source text is matched against patterns in document order; the first matching pattern assigns a scope name to the token range.
- **Data flow:** The grammar file is loaded into memory by the TextMate tokenizer at language activation. During document tokenization, the tokenizer walks `patterns`, resolves `include` references into `repository` entries, applies regex `match`/`begin`/`end` rules, and emits an array of `(startIndex, scopeStack)` token descriptors for each line. These descriptors flow into the editor's syntax highlighting layer to select theme colours.
- **Dependencies:** `vscode-textmate` (runtime engine, not bundled in this extension). Upstream source: `jeff-hykin/better-objc-syntax`.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/objective-c/syntaxes/objective-c++.tmLanguage.json`

- **Role:** Vendored TextMate grammar for Objective-C++ source files (`.mm`). Structurally similar to the Objective-C grammar but includes a top-level `include: "#cpp_lang"` rule (line 12) that incorporates C++ tokenization patterns.
- **Key symbols:**
  - `scopeName: "source.objcpp"` (line 9): root scope.
  - `patterns[0]: { "include": "#cpp_lang" }` (line 11–13): the first top-level pattern; routes to a `cpp_lang` repository entry that handles C++-specific syntax, making this grammar a superset.
  - `repository`: 55 named pattern rules.
  - `version` (line 7): commit hash `5a7eb15eee382dd5aa388bc04fdb60a0d2128e14`, matching `cgmanifest.json`.
- **Control flow:** Same TextMate evaluation model as the Objective-C grammar. The `#cpp_lang` include at the head of `patterns` means C++ constructs take precedence in matching order before Objective-C-specific rules.
- **Data flow:** Identical pipeline to `objective-c.tmLanguage.json`; token descriptors flow to the syntax highlighting layer using the `source.objcpp` root scope.
- **Dependencies:** `vscode-textmate` runtime engine. Upstream source: `jeff-hykin/better-objcpp-syntax`.

---

### Cross-Cutting Synthesis

The `extensions/objective-c/` extension is a pure data extension — it contains no TypeScript or JavaScript that executes at runtime in the VS Code host process. Its entire runtime surface consists of two JSON grammar files and one JSON language-configuration file, all consumed declaratively by VS Code's built-in subsystems: the TextMate tokenizer (`vscode-textmate`) for syntax highlighting and the language configuration service for editor behaviour (bracket matching, comment toggling, on-enter rules).

The one JavaScript file present (`build/update-grammars.js`) is a dev-time maintenance script excluded from packaged builds by `.vscodeignore`. It uses the `vscode-grammar-updater` monorepo utility to pull grammar updates from upstream GitHub repositories.

From the perspective of the research question — porting VS Code's core IDE to Tauri/Rust — this partition has zero bearing. The extension contributes no Electron APIs, no Node.js APIs, no IPC, and no process-model dependencies. Any TextMate-compatible tokenizer (e.g., `syntect` in Rust) could consume the same `syntaxes/*.tmLanguage.json` files without modification. The `language-configuration.json` schema would need a Tauri-side consumer, but the JSON content itself is host-agnostic. The only coupling is to VS Code's extension manifest schema (`package.json` `contributes` keys), which would need to be re-implemented in any alternative host.

---

### Out-of-Partition References

- `src/vs/editor/common/languages/languageConfiguration.ts` — processes `onEnterRules`, `brackets`, `autoClosingPairs`, and `surroundingPairs` from `language-configuration.json` at runtime (outside this partition).
- `node_modules/vscode-grammar-updater/` (monorepo root) — provides the `update()` function called at lines 10–11 of `build/update-grammars.js`.
- `vscode-textmate` (bundled with VS Code core) — the runtime engine that loads and evaluates `syntaxes/objective-c.tmLanguage.json` and `syntaxes/objective-c++.tmLanguage.json`.
- `jeff-hykin/better-objc-syntax` (GitHub, commit `119b75fb`) — upstream source for `syntaxes/objective-c.tmLanguage.json`.
- `jeff-hykin/better-objcpp-syntax` (GitHub, commit `5a7eb15e`) — upstream source for `syntaxes/objective-c++.tmLanguage.json`.

---

This document covers all eight files in `extensions/objective-c/`. The partition contains no runtime TypeScript or JavaScript code: the extension operates entirely through declarative JSON manifests and TextMate grammar files interpreted by VS Code's built-in tokenizer and language-configuration subsystems. The single JavaScript file (`build/update-grammars.js`) is a dev-time grammar-fetch utility that is excluded from packaged builds. In the context of a Tauri/Rust port, this partition's artifacts (the two `.tmLanguage.json` grammars and `language-configuration.json`) are directly reusable by any host that implements the TextMate grammar protocol and a compatible language-configuration consumer, with no Electron or Node.js runtime dependencies to replace.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Analysis: Objective-C Extension
## Scope: `extensions/objective-c/` (Grammar & Snippet Configuration)

### Research Question
What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

---

## Pattern Findings

#### Pattern 1: Extension Manifest with Language Contribution
**Where:** `extensions/objective-c/package.json:15-50`
**What:** Standard VS Code extension manifest declaring language support with multiple file extensions and associated grammars via TextMate syntax definitions.
```json
"contributes": {
  "languages": [
    {
      "id": "objective-c",
      "extensions": [
        ".m"
      ],
      "aliases": [
        "Objective-C"
      ],
      "configuration": "./language-configuration.json"
    },
    {
      "id": "objective-cpp",
      "extensions": [
        ".mm"
      ],
      "aliases": [
        "Objective-C++"
      ],
      "configuration": "./language-configuration.json"
    }
  ],
  "grammars": [
    {
      "language": "objective-c",
      "scopeName": "source.objc",
      "path": "./syntaxes/objective-c.tmLanguage.json"
    },
    {
      "language": "objective-cpp",
      "scopeName": "source.objcpp",
      "path": "./syntaxes/objective-c++.tmLanguage.json"
    }
  ]
}
```
**Variations / call-sites:** Standard pattern replicated across 50+ language extensions in VS Code. Each language extension declares: language IDs, file extensions, aliases, and grammar scope names referencing TextMate syntax files. Reused in C, C++, Java, Python, and other language packs.

---

#### Pattern 2: Language Configuration with Bracket/Comment Rules
**Where:** `extensions/objective-c/language-configuration.json:1-88`
**What:** Language configuration file defining comment syntax, bracket pairs, auto-closing behavior, and on-enter rules for language-specific formatting rules that apply within the editor.
```json
{
  "comments": {
    "lineComment": "//",
    "blockComment": [
      "/*",
      "*/"
    ]
  },
  "brackets": [
    [
      "{",
      "}"
    ],
    [
      "[",
      "]"
    ],
    [
      "(",
      ")"
    ]
  ],
  "autoClosingPairs": [
    [
      "{",
      "}"
    ],
    [
      "[",
      "]"
    ],
    [
      "(",
      ")"
    ],
    {
      "open": "\"",
      "close": "\"",
      "notIn": [
        "string"
      ]
    },
    {
      "open": "'",
      "close": "'",
      "notIn": [
        "string"
      ]
    }
  ],
  "surroundingPairs": [
    [
      "{",
      "}"
    ],
    [
      "[",
      "]"
    ],
    [
      "(",
      ")"
    ],
    [
      "\"",
      "\""
    ],
    [
      "'",
      "'"
    ]
  ],
  "onEnterRules": [
    {
      "beforeText": {
        "pattern": "\/\/.*"
      },
      "afterText": {
        "pattern": "^(?!\\s*$).+"
      },
      "action": {
        "indent": "none",
        "appendText": "// "
      }
    }
  ]
}
```
**Variations / call-sites:** Core editor behavior system. This configuration is loaded by the core language support and tokenization engines in VS Code. The `onEnterRules` pattern handles automatic comment continuation when pressing Enter inside line comments. Bracket matching drives editor features like bracket highlighting, bracket navigation, and auto-closing pairs.

---

#### Pattern 3: Grammar Update Build Script
**Where:** `extensions/objective-c/build/update-grammars.js:1-14`
**What:** Build script using `vscode-grammar-updater` module to synchronize TextMate grammar definitions from external GitHub repositories, updating local syntax files.
```javascript
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
'use strict';

var updateGrammar = require('vscode-grammar-updater');

async function updateGrammars() {
  await updateGrammar.update('jeff-hykin/better-objcpp-syntax', 'autogenerated/objcpp.tmLanguage.json', './syntaxes/objective-c++.tmLanguage.json', undefined, 'master');
  await updateGrammar.update('jeff-hykin/better-objc-syntax', 'autogenerated/objc.tmLanguage.json', './syntaxes/objective-c.tmLanguage.json', undefined, 'master');
}

updateGrammars();
```
**Variations / call-sites:** Grammar synchronization pattern used in 50+ language extensions. Each invocation specifies: source GitHub repo, remote path to grammar file, local destination path, optional credentials, and branch name. Enables maintenance of grammar files without forking external syntax repositories.

---

## Summary

The Objective-C extension demonstrates three fundamental architectural patterns for language support in VS Code:

1. **Extension Declaration Model**: Extensions register language contributions through `package.json`, mapping file extensions to language IDs and associating TextMate grammar files with scope names.

2. **Language Behavior Configuration**: Language-specific editor behavior (comment syntax, bracket pairing, auto-closing, on-enter formatting) is centralized in `language-configuration.json` using a declarative JSON format. This drives editor features without language-specific code.

3. **Grammar Asset Management**: TextMate syntax definitions are managed as external dependencies, synchronized via build scripts that fetch upstream grammar definitions from community repositories. This separates grammar maintenance from VS Code's release cycle.

**Porting implications**: A Tauri/Rust port would need to:
- Maintain TextMate grammar support as the tokenization basis (grammar formats are language-agnostic)
- Port the language configuration JSON loading and rule interpretation to Rust
- Implement TextMate scope name matching and bracket pair logic in Rust
- Develop equivalent grammar update/sync mechanisms for build pipeline integration

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
