## Analysis: `extensions/sql/` — SQL Language Basics Extension

### Files Analysed

1. `/Users/norinlavaee/vscode-atomic/extensions/sql/package.json` (42 lines)
2. `/Users/norinlavaee/vscode-atomic/extensions/sql/language-configuration.json` (41 lines)
3. `/Users/norinlavaee/vscode-atomic/extensions/sql/syntaxes/sql.tmLanguage.json` (642 lines)
4. `/Users/norinlavaee/vscode-atomic/extensions/sql/build/update-grammar.mjs` (8 lines)
5. `/Users/norinlavaee/vscode-atomic/extensions/sql/package.nls.json` (4 lines)
6. `/Users/norinlavaee/vscode-atomic/extensions/sql/cgmanifest.json` (17 lines)
7. `/Users/norinlavaee/vscode-atomic/extensions/sql/.vscodeignore` (3 lines)

---

### Per-File Notes

#### 1. `extensions/sql/package.json` — Extension Manifest

- **Role**: Declares the extension to the VS Code extension host. All runtime behavior is driven by the `contributes` block; there is no compiled entry point.
- **Key symbols**:
  - `contributes.languages` (lines 16–29): Registers language id `"sql"` with file extensions `.sql` and `.dsql`, human-readable aliases `"MS SQL"` and `"T-SQL"`, and points to `./language-configuration.json` for editor affordances.
  - `contributes.grammars` (lines 30–36): Associates the TextMate grammar at `./syntaxes/sql.tmLanguage.json` with language id `sql` under scope name `source.sql`.
  - `scripts.update-grammar` (line 12): Single npm script `node ./build/update-grammar.mjs`, used only during development to refresh the grammar from upstream.
- **Dependencies**: None at runtime. The `engines.vscode: "*"` field (line 9) means the extension imposes no minimum VS Code version constraint.
- **Data flow**: At extension activation the VS Code language service reads `contributes` declaratively; no JavaScript activation function runs.

#### 2. `extensions/sql/language-configuration.json` — Editor Affordances

- **Role**: Configures editor behaviors for the `sql` language id. Consumed by VS Code's built-in language service at startup.
- **Key blocks**:
  - `comments` (lines 2–5): Line comment token `--`; block comment delimiters `/* … */`.
  - `brackets` (lines 6–10): Three bracket pairs: `{}`, `[]`, `()`.
  - `autoClosingPairs` (lines 11–18):
    - `{}`, `[]`, `()` close automatically.
    - Double-quote pair (line 15): `notIn: ["string"]` prevents auto-closing inside a string.
    - T-SQL `N'…'` Unicode string literal (line 16): opens on `N'`, closes on `'`, disabled inside `string` and `comment` scopes.
    - Single-quote pair (line 17): `notIn: ["string", "comment"]`.
  - `surroundingPairs` (lines 19–26): `{}`, `[]`, `()`, `""`, `''`, backtick — used when user selects text and types an opening delimiter.
  - `folding` (lines 27–33):
    - `offSide: true` enables indentation-based folding as a fallback.
    - `markers.start` (line 30): regex `^\s*--\s*#region\b` — matches `-- #region` comment markers.
    - `markers.end` (line 31): regex `^\s*--\s*#endregion\b` — matches `-- #endregion`.
  - Lines 35–39 contain a commented-out `enhancedBrackets` block with `BEGIN/END`, `CASE/END`, `WHEN/THEN` pairs — not active.

#### 3. `extensions/sql/syntaxes/sql.tmLanguage.json` — TextMate Grammar

- **Role**: Provides token-level syntax highlighting for `.sql` and `.dsql` files. Consumed by VS Code's TextMate tokenizer (vscode-textmate). 642 lines of JSON.
- **Provenance**: Converted from `microsoft/vscode-mssql` plist at commit `c002f514dd81fa71fa304d4c36f8d2767dbf2f9d` (line 7). Upstream is `extensions/mssql/syntaxes/SQL.plist`.
- **Top-level structure**:
  - `scopeName: "source.sql"` (line 9) — the root scope.
  - `patterns` array (lines 10–399) — ordered list of match rules applied sequentially.
  - `repository` (lines 400–642) — reusable named rule groups (`comments`, `comment-block`, `regexps`, `string_escape`, `string_interpolation`, `strings`).

- **Pattern sequence in `patterns`**:

  | Lines | Pattern | Scope assigned |
  |-------|---------|----------------|
  | 12–14 | `((?<!@)@)\b(\w+)\b` | `text.variable` — T-SQL `@variable` names |
  | 15–18 | `(\[)[^\]]*(\])` | `text.bracketed` — bracket-quoted identifiers like `[dbo]` |
  | 19–21 | `{"include": "#comments"}` | Delegates to `repository.comments` |
  | 22–36 | `meta.create.sql` | `CREATE [OR REPLACE] <object>` DDL statements; capture groups assign `keyword.other.create.sql` (cap 1), `keyword.other.sql` (cap 2), `entity.name.function.sql` (cap 5) |
  | 37–48 | `meta.drop.sql` | `DROP <object>` — two-word keyword pattern |
  | 49–66 | `meta.drop.sql` (table form) | `DROP TABLE <name> [CASCADE]` with cascade captured separately |
  | 67–78 | `meta.alter.sql` | `ALTER <object>` |
  | 79–128 | `storage.type.sql` / `constant.numeric.sql` | SQL data types: `bigint`, `varchar(n)`, `numeric(p,q)`, `timestamp`, `time with time zone`, etc. Uses verbose regex with named capture groups 1–15 |
  | 129–132 | `storage.modifier.sql` | Constraint keywords: `PRIMARY KEY`, `FOREIGN KEY`, `REFERENCES`, `DEFAULT`, etc. |
  | 133–136 | `constant.numeric.sql` | Bare numeric literals `\b\d+\b` |
  | 137–140 | `keyword.other.DML.sql` | DML verbs: `SELECT`, `INSERT INTO`, `UPDATE`, `DELETE`, `FROM`, `WHERE`, `JOIN` variants, `UNION`, `GROUP BY`, `ORDER BY`, `HAVING`, `LIMIT` |
  | 141–144 | `keyword.other.DDL.create.II.sql` | `ON`, `OFF`, `NULL`, `NOT NULL`, `IS NOT NULL` |
  | 145–148 | `keyword.other.DML.II.sql` | `VALUES` |
  | 149–152 | `keyword.other.LUW.sql` | Transaction control: `BEGIN [WORK]`, `START TRANSACTION`, `COMMIT [WORK]`, `ROLLBACK [WORK]` |
  | 153–156 | `keyword.other.authorization.sql` | `GRANT [WITH GRANT OPTION]`, `REVOKE` |
  | 157–160 | `keyword.other.data-integrity.sql` | `IN` keyword |
  | 161–164 | `keyword.other.object-comments.sql` | `COMMENT ON TABLE/COLUMN/...` |
  | 165–168 | `keyword.other.alias.sql` | `AS` |
  | 169–172 | `keyword.other.order.sql` | `DESC`, `ASC` |
  | 173–176 | `keyword.operator.star.sql` | `*` |
  | 177–180 | `keyword.operator.comparison.sql` | `=`, `<>`, `!=`, `<`, `>`, `<=`, `>=` |
  | 181–184 | `keyword.operator.math.sql` | `-`, `+`, `/` |
  | 185–188 | `keyword.operator.concatenator.sql` | `\|\|` |
  | 189–364 | Function groups | Categorized built-in functions, each with scope `support.function.<category>.sql`: `aggregate`, `analytic`, `bitmanipulation`, `conversion`, `collation`, `cryptographic`, `cursor`, `datetime`, `datatype`, `expression`, `globalvar`, `json`, `logical`, `mathematical`, `metadata`, `ranking`, `rowset`, `security`, `string`, `system`, `textimage`, `vector` |
  | 365–375 | `constant.other.database-name.sql` / `constant.other.table-name.sql` | Two-part dotted identifiers `schema.table` |
  | 376–378 | `{"include": "#strings"}` | Delegates to `repository.strings` |
  | 379–381 | `{"include": "#regexps"}` | Delegates to `repository.regexps` |
  | 382–385 | `keyword.other.sql` | Massive catch-all keyword list (~700+ keywords) covering all T-SQL reserved words, options, and identifiers not matched by earlier patterns |
  | 386–398 | `meta.block.sql` | Empty `()` pair, captures open/close as `punctuation.section.scope.begin/end.sql` |

- **Repository rules**:
  - `comments` (lines 401–437): Two-pass pattern. First tries `--` line comments (scope `comment.line.double-dash.sql`, ends at `\n`). Second tries `#` (no patterns applied, effectively a no-op stub). Both delegate to `#comment-block` for `/* … */`.
  - `comment-block` (lines 439–453): Matches `/* … */` as `comment.block`. Recursively includes `#comment-block` to handle nested block comments.
  - `regexps` (lines 454–502): Two regexp-literal patterns — `/pattern/` and `%r{pattern}` — both scoped `string.regexp.sql`, supporting `#string_interpolation` includes inside.
  - `string_escape` (lines 503–506): `\\.` matched as `constant.character.escape.sql`.
  - `string_interpolation` (lines 507–518): `#{…}` interpolation blocks as `string.interpolated.sql`, with captures on delimiters.
  - `strings` (lines 519–640): Seven patterns for string literals:
    - Fast single-line single-quote with optional `N` prefix (lines 521–531): `N'…'` and `'…'` as `string.quoted.single.sql`.
    - Begin/end single-quote with escape support (lines 532–553).
    - Fast backtick (lines 554–565): `` `…` `` as `string.quoted.other.backtick.sql`.
    - Begin/end backtick with escapes (lines 566–585).
    - Fast double-quote (lines 586–599): `"…"` as `string.quoted.double.sql` (excludes `#` to avoid confusion with interpolation).
    - Begin/end double-quote with interpolation support (lines 600–617).
    - `%{…}` bracket strings with interpolation (lines 618–638).

#### 4. `extensions/sql/build/update-grammar.mjs` — Grammar Updater Script

- **Role**: Development-time utility. Not included in the published extension (`.vscodeignore` excludes `build/**`).
- **Mechanism** (lines 6–8): Imports `vscode-grammar-updater` (an npm package shared across language extensions) and calls `vscodeGrammarUpdater.update(owner, sourcePath, outputPath, undefined, branch)` with:
  - `owner`: `'microsoft/vscode-mssql'`
  - `sourcePath`: `'extensions/mssql/syntaxes/SQL.plist'` — upstream plist grammar file
  - `outputPath`: `'./syntaxes/sql.tmLanguage.json'` — local JSON grammar
  - `branch`: `'main'`
- The function fetches the upstream plist, converts it to JSON, and writes the result to disk, regenerating `sql.tmLanguage.json`.

#### 5. `extensions/sql/package.nls.json` — Localization Strings

- **Role**: Provides default English strings for `%displayName%` and `%description%` tokens referenced in `package.json`.
- `displayName` (line 2): `"SQL Language Basics"`
- `description` (line 3): `"Provides syntax highlighting and bracket matching in SQL files."`

#### 6. `extensions/sql/cgmanifest.json` — Component Governance Manifest

- **Role**: Declares the upstream third-party component for legal/compliance tracking.
- Registers `microsoft/vscode-mssql` at commit `c002f514dd81fa71fa304d4c36f8d2767dbf2f9d` (line 9) as the source for the grammar, licensed MIT.

#### 7. `extensions/sql/.vscodeignore` — Packaging Exclusions

- Excludes `test/**`, `cgmanifest.json`, and `build/**` from the packaged extension artifact. The `build/update-grammar.mjs` script and governance manifest are stripped at publish time.

---

### Cross-Cutting Synthesis

The `extensions/sql/` partition is a pure declarative language extension with exactly 8 lines of custom executable code (all in `build/update-grammar.mjs`) and zero runtime TypeScript or JavaScript. The entire feature surface — syntax highlighting, comment toggling, bracket matching, auto-closing pairs, and code folding — is delivered through two JSON configuration files and one TextMate grammar. The grammar itself is not locally authored: it is fetched from the upstream `microsoft/vscode-mssql` repository via `vscode-grammar-updater` and stored verbatim as `sql.tmLanguage.json`. VS Code's extension host activates this extension implicitly upon opening any `.sql` or `.dsql` file, loading the grammar into the vscode-textmate tokenizer and the language configuration into the editor's language service — both processes are handled entirely by VS Code's core infrastructure with no extension-side activation code. For a Tauri/Rust port, the relevant question is whether the target platform implements (a) a TextMate grammar tokenizer equivalent (e.g., a Rust port of vscode-textmate) and (b) a language-configuration consumer that processes the JSON affordances file; the SQL extension itself requires no TypeScript or Electron APIs and would port at zero cost if those two infrastructure pieces exist.

---

### Out-of-Partition References

- **`vscode-grammar-updater`** (npm package) — imported at `build/update-grammar.mjs:6`; defined outside this repository entirely.
- **`microsoft/vscode-mssql`** GitHub repository — upstream source for `sql.tmLanguage.json` grammar at commit `c002f514dd81fa71fa304d4c36f8d2767dbf2f9d`; referenced in `cgmanifest.json:9` and `syntaxes/sql.tmLanguage.json:7`.
- **VS Code TextMate tokenizer** — consumes `source.sql` grammar scope; implemented in VS Code core (outside `extensions/sql/`), specifically the `vscode-textmate` integration layer.
- **VS Code language service** — consumes `language-configuration.json`; implemented in `src/vs/editor/` (outside this partition).
- **Extension host activation infrastructure** — reads `package.json` `contributes` block; implemented in `src/vs/workbench/services/extensions/` (outside this partition).
