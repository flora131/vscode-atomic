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
