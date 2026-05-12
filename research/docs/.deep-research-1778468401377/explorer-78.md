# Partition 78 of 80 — Findings

## Scope
`extensions/sql/` (1 files, 8 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# SQL Extension: Grammar-Only Implementation

## Implementation

- `extensions/sql/syntaxes/sql.tmLanguage.json` (642 LOC) — TextMate grammar definition for SQL syntax highlighting
- `extensions/sql/language-configuration.json` (41 LOC) — Language configuration defining comments, brackets, auto-closing pairs, folding markers, and surrounding pairs for SQL
- `extensions/sql/package.json` (42 LOC) — Extension manifest declaring language registration for `.sql` and `.dsql` files, grammar path, and build scripts

## Configuration

- `extensions/sql/package.nls.json` (4 LOC) — Localization strings for extension display name and description
- `extensions/sql/.vscodeignore` (4 LOC) — Build artifacts and test exclusion patterns for packaging
- `extensions/sql/cgmanifest.json` (17 LOC) — Component governance manifest referencing upstream source from microsoft/vscode-mssql repository

## Build

- `extensions/sql/build/update-grammar.mjs` (8 LOC) — Automated grammar update script that synchronizes sql.tmLanguage.json with upstream vscode-mssql repository

## Notable Clusters

The SQL extension is a minimal grammar-only implementation with no tests, no type definitions, and no examples. It consists of 7 files totaling approximately 758 lines. The core functionality is the TextMate grammar file (642 LOC) for syntax highlighting, supported by a language configuration file that defines editor behaviors (comments, brackets, folding). The extension declaration in package.json registers support for SQL file extensions (.sql, .dsql) and aliases (MS SQL, T-SQL). All grammar maintenance is automated via the update-grammar.mjs script that pulls the latest grammar definition from the upstream vscode-mssql repository.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/sql/package.json` (42 LOC)
- `/home/norinlavaee/projects/vscode-atomic/extensions/sql/language-configuration.json` (41 LOC)
- `/home/norinlavaee/projects/vscode-atomic/extensions/sql/syntaxes/sql.tmLanguage.json` (643 LOC)
- `/home/norinlavaee/projects/vscode-atomic/extensions/sql/build/update-grammar.mjs` (8 LOC)

---

### Per-File Notes

#### `extensions/sql/package.json` (lines 1–42)

- Extension identity: `name: "sql"`, `version: "10.0.0"`, `publisher: "vscode"` (lines 2–6).
- Engine constraint: `"vscode": "*"` (line 9) — no minimum version pinned.
- Single npm script: `"update-grammar": "node ./build/update-grammar.mjs"` (line 12) — the only build action in this extension.
- Language contribution (lines 16–28): registers the `sql` language ID, maps file extensions `.sql` and `.dsql` (lines 19–22), declares aliases `"MS SQL"` and `"T-SQL"` (lines 23–26), and points to `./language-configuration.json` (line 27).
- Grammar contribution (lines 30–36): binds the `sql` language ID to scope name `source.sql` and the TextMate grammar file `./syntaxes/sql.tmLanguage.json`.
- No `activationEvents`, no `main` entry point, and no runtime dependencies — this is a declarative-only extension.

#### `extensions/sql/language-configuration.json` (lines 1–41)

- Comment tokens (lines 2–5): line comment `--`; block comment delimiters `/* */`.
- Bracket pairs (lines 6–10): `{}`, `[]`, `()`.
- Auto-closing pairs (lines 11–18):
  - Same three bracket pairs.
  - `"` auto-closes unless already inside a string (line 15).
  - `N'` (T-SQL Unicode string prefix) closes with `'`, excluded inside strings and comments (line 16).
  - `'` closes with `'`, excluded inside strings and comments (line 17).
- Surrounding pairs (lines 19–26): `{}`, `[]`, `()`, `""`, `''`, ` `` `.
- Folding (lines 27–33):
  - `offSide: true` (line 28) — indentation-based folding is active.
  - Region markers: `start` regex `^\s*--\s*#region\b` and `end` regex `^\s*--\s*#endregion\b` (lines 30–31), enabling explicit fold regions via SQL line-comment syntax.
- Commented-out `enhancedBrackets` block (lines 35–39) references `begin/end`, `case/end`, and `when/then` pairings — present as documentation notes but not active.

#### `extensions/sql/syntaxes/sql.tmLanguage.json` (lines 1–643)

Upstream provenance declared at line 3: converted from `https://github.com/microsoft/vscode-mssql/blob/master/extensions/mssql/syntaxes/SQL.plist`. Upstream commit pinned at line 7.

Top-level `patterns` array (lines 10–399) defines the token scan order:

1. **Variables** (line 12): match `@word` (single `@` prefix, not `@@`) → scope `text.variable`.
2. **Bracketed identifiers** (line 16): match `[…]` → scope `text.bracketed`.
3. **Comments include** (line 20): delegates to `#comments` repository rule.
4. **DDL CREATE** (lines 23–36): case-insensitive regex captures `create [or replace]` + object type keyword (aggregate, table, view, etc.) + optional-quoted identifier. Named captures: group 1 → `keyword.other.create.sql`; group 2 → `keyword.other.sql`; group 5 → `entity.name.function.sql`. Whole match scope: `meta.create.sql`.
5. **DDL DROP** (lines 38–48): captures `drop` + object type keyword → `meta.drop.sql`.
6. **DROP TABLE with CASCADE** (lines 49–66): specialised sub-case extracting table name (capture 3 → `entity.name.function.sql`) and optional `cascade` (capture 4 → `keyword.other.cascade.sql`).
7. **DDL ALTER** (lines 67–78): captures `alter` + object type keyword including `proc(edure)?` → `meta.alter.sql`.
8. **Data types** (lines 79–128): a multi-branch regex (extended mode, `(?xi)`) covering 15 capture groups:
   - Groups 1: bare types (`bigint`, `boolean`, `date`, `text`, etc.).
   - Groups 2+3: types with mandatory numeric precision in parentheses (`bit varying`, `float(n)`).
   - Groups 4+5: types with optional precision (`char`, `number`, `varchar`).
   - Groups 6+7+8: `numeric`/`decimal` with optional `(precision,scale)`.
   - Groups 9+10+11: `time[s]` with optional precision and `with/without time zone`.
   - Groups 12+13+14+15: `timestamp` with optional `s`/`tz` suffix, optional precision, and `with/without time zone`.
   - All type captures → `storage.type.sql`; numeric literals within type specs → `constant.numeric.sql`.
9. **Constraint keywords** (line 130): `PRIMARY KEY`, `FOREIGN KEY`, `REFERENCES`, `ON DELETE/UPDATE [CASCADE]`, `CONSTRAINT`, `COLLATE`, `DEFAULT` → `storage.modifier.sql`.
10. **Numeric literals** (line 134): `\b\d+\b` → `constant.numeric.sql`.
11. **DML keywords** (line 138): `SELECT [ALL|DISTINCT]`, `INSERT [IGNORE] INTO`, `UPDATE`, `DELETE`, `FROM`, `WHERE`, `GROUP BY`, `ORDER BY`, `LIMIT`, all JOIN variants → `keyword.other.DML.sql`.
12. **NULL/ON/OFF** (line 142): `ON`, `OFF`, `NULL`, `NOT NULL`, `IS NOT NULL` → `keyword.other.DDL.create.II.sql`.
13. **VALUES** (line 146) → `keyword.other.DML.II.sql`.
14. **Transaction control** (line 150): `BEGIN [WORK]`, `START TRANSACTION`, `COMMIT [WORK]`, `ROLLBACK [WORK]` → `keyword.other.LUW.sql`.
15. **Authorization** (line 154): `GRANT [WITH GRANT OPTION]`, `REVOKE` → `keyword.other.authorization.sql`.
16. **IN keyword** (line 158) → `keyword.other.data-integrity.sql`.
17. **COMMENT ON** (line 162) → `keyword.other.object-comments.sql`.
18. **AS alias** (line 166) → `keyword.other.alias.sql`.
19. **DESC/ASC** (line 170) → `keyword.other.order.sql`.
20. **Operators**: `*` → `keyword.operator.star.sql` (line 174); comparison operators `=`, `!=`, `<>`, `<`, `>` → `keyword.operator.comparison.sql` (line 178); arithmetic `-`, `+`, `/` → `keyword.operator.math.sql` (line 182); concatenation `||` → `keyword.operator.concatenator.sql` (line 186).
21. **Built-in function categories** (lines 190–364), each matching the function name followed by `\s*(` so only actual call sites are tokenised:
   - Aggregate functions (line 190): `avg`, `count`, `max`, `min`, `sum`, `stdev`, etc. → `support.function.aggregate.sql`.
   - Analytic / window functions (line 198): `cume_dist`, `lag`, `lead`, `percent_rank`, etc. → `support.function.analytic.sql`.
   - Bit manipulation (line 206): `bit_count`, `left_shift`, `right_shift`, etc. → `support.function.bitmanipulation.sql`.
   - Conversion (line 214): `cast`, `convert`, `try_cast`, etc. → `support.function.conversion.sql`.
   - Collation (line 222): `collationproperty`, `tertiary_weights` → `support.function.collation.sql`.
   - Cryptographic (line 230): extensive list of encryption/decryption/signing functions → `support.function.cryptographic.sql`.
   - Cursor (line 238): `cursor_status` → `support.function.cursor.sql`.
   - Date/time (line 246): `sysdatetime`, `getdate`, `dateadd`, `datediff`, `datetrunc`, `date_bucket`, etc. → `support.function.datetime.sql`.
   - Data type inspection (line 254): `datalength`, `ident_current`, `identity` → `support.function.datatype.sql`.
   - Expression (line 262): `coalesce`, `nullif` → `support.function.expression.sql`.
   - Global variables (line 270): `@@error`, `@@rowcount`, `@@servername`, etc. — pattern requires `(?<!@)@@` to avoid matching `@@@` → `support.function.globalvar.sql`.
   - JSON (line 278): `isjson`, `json_value`, `json_query`, `json_modify`, `json_path_exists` → `support.function.json.sql`.
   - Logical (line 286): `choose`, `iif`, `greatest`, `least` → `support.function.logical.sql`.
   - Mathematical (line 294): `abs`, `sin`, `cos`, `sqrt`, `round`, etc. → `support.function.mathematical.sql`.
   - Metadata (line 302): `object_id`, `object_name`, `db_id`, `schema_name`, `type_name`, etc. → `support.function.metadata.sql`.
   - Ranking (line 310): `rank`, `dense_rank`, `ntile`, `row_number` → `support.function.ranking.sql`.
   - Rowset (line 318): `openjson`, `openrowset`, `string_split`, `generate_series`, `predict` → `support.function.rowset.sql`.
   - Security (line 326): `current_user`, `session_user`, `suser_sname`, `is_member`, `is_rolemember`, etc. → `support.function.security.sql`.
   - String (line 334): `ascii`, `charindex`, `len`, `ltrim`, `rtrim`, `substring`, `string_agg`, `translate`, `trim`, etc. → `support.function.string.sql`.
   - System (line 342): `newid`, `isnull`, `isnumeric`, `error_message`, `error_number`, `xact_state`, etc. → `support.function.system.sql`.
   - Text/image (line 350): `patindex`, `textptr`, `textvalid` → `support.function.textimage.sql`.
   - Vector (line 358): `vector_distance`, `vector_norm`, `vector_normalize` → `support.function.vector.sql`.
22. **Database/table reference** (lines 366–375): match `word.word` two-part names → capture 1 `constant.other.database-name.sql`, capture 2 `constant.other.table-name.sql`.
23. **Strings include** (line 377): delegates to `#strings`.
24. **Regexps include** (line 380): delegates to `#regexps`.
25. **General keyword blob** (lines 382–385): a massive case-insensitive alternation covering hundreds of T-SQL keywords (`abort` through `zone`) → `keyword.other.sql`. This is the catch-all for any DDL/DCL/procedural keyword not handled by the more specific patterns above.
26. **Empty parentheses** (lines 386–398): `()` — two captured punctuation scopes `punctuation.section.scope.begin.sql` and `punctuation.section.scope.end.sql` — whole match is `meta.block.sql`.

`repository` section (lines 400–642) defines four reusable rule sets:

- **`comments`** (lines 401–437): two-phase patterns. Phase 1 (lines 403–421): optional leading whitespace capture, then a nested `--` line-comment rule that ends at `\n` → scope `comment.line.double-dash.sql`. Phase 2 (lines 423–433): same structure for `#`-prefixed comments, but nested `patterns` array is empty (no tokenisation inside `#` comments). Phase 3 (line 435): includes `#comment-block`.
- **`comment-block`** (lines 439–453): `/* … */` block comment → scope `comment.block`; recursively includes itself at line 450 to support nested `/* … /* … */ … */` blocks.
- **`regexps`** (lines 454–501): two patterns for regex literals — `/pattern/` (lines 456–479) and `%r{pattern}` (lines 481–499) — each including `#string_interpolation` and the escape rule.
- **`strings`** (lines 519–641): six patterns:
  - Single-line `N'…'` or `'…'` optimised fast path (line 531) → `string.quoted.single.sql`.
  - Multi-line `'…'` with `#string_escape` fallback (lines 534–552).
  - Single-line backtick fast path (line 564) → `string.quoted.other.backtick.sql`.
  - Multi-line backtick with `#string_escape` (lines 567–585).
  - Single-line `"…"` fast path (line 597) → `string.quoted.double.sql`.
  - Multi-line `"…"` with `#string_interpolation` (lines 600–618).
  - `%{…}` string form (lines 620–638) → `string.other.quoted.brackets.sql`, includes `#string_interpolation`.

#### `extensions/sql/build/update-grammar.mjs` (lines 1–8)

- Imports from `vscode-grammar-updater` (line 6) — a shared VS Code tooling package.
- Line 8 calls `vscodeGrammarUpdater.update(...)` with four positional arguments:
  1. GitHub repo path: `'microsoft/vscode-mssql'`
  2. Source plist path within that repo: `'extensions/mssql/syntaxes/SQL.plist'`
  3. Local output path: `'./syntaxes/sql.tmLanguage.json'`
  4. `undefined` (no version override)
  5. Branch: `'main'`
- This script fetches the upstream Plist grammar, converts it to JSON, and writes it to the local `syntaxes/` directory. Running `npm run update-grammar` is the only mechanism by which the grammar is refreshed.

---

### Cross-Cutting Synthesis

The `extensions/sql/` extension is entirely grammar-only. There is no TypeScript source, no `main` activation entry, no language server, and no runtime process. All functionality is delivered through three static JSON/config files registered via `package.json` contributions:

- **Syntax highlighting** — delegated to `sql.tmLanguage.json`, which encodes the full T-SQL tokeniser as a TextMate grammar. VS Code's built-in `vscode-textmate` engine evaluates the grammar at runtime inside the renderer process; the extension itself contributes zero executable code.
- **Editor behaviour** — `language-configuration.json` feeds VS Code's language configuration service, which governs bracket matching, auto-closing, surrounding pairs, comment toggling, and fold markers. All of this is processed by the editor's built-in services from the static JSON.
- **Grammar maintenance** — `update-grammar.mjs` is a developer tool only, executed manually via `npm run update-grammar`. It has no role in the packaged extension or runtime behaviour.

The extension has no dependency on Electron APIs, no Node.js runtime requirement (the grammar script is only a devtool), and no IPC. Its contribution is purely declarative configuration consumed by the VS Code platform's own tokenisation and editing subsystems.

---

### Out-of-Partition References

- `vscode-grammar-updater` package (imported at `build/update-grammar.mjs:6`) — lives in `node_modules`; implementation is outside this partition.
- Upstream grammar source: `https://github.com/microsoft/vscode-mssql/blob/main/extensions/mssql/syntaxes/SQL.plist` — external repository; the pinned commit SHA is recorded in `syntaxes/sql.tmLanguage.json:7`.
- The TextMate grammar engine that interprets `sql.tmLanguage.json` at runtime is implemented in `src/` under `vscode-textmate` — outside this partition.
- The language configuration service that reads `language-configuration.json` is implemented in `src/vs/editor/common/languages/languageConfigurationRegistry.ts` — outside this partition.

---

The `extensions/sql/` partition contains no IDE core logic and presents no Tauri/Rust porting surface of its own. Its entire contribution is static data consumed by the VS Code editor platform: a TextMate grammar JSON for tokenisation and a language-configuration JSON for editor behaviours. The only porting consideration this partition raises is that any replacement editor host (Tauri or otherwise) must supply a TextMate grammar evaluation engine and a language-configuration consumer equivalent to `vscode-textmate` and VS Code's `languageConfigurationRegistry` — both of which are outside this partition.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: VS Code Core IDE Porting (Tauri/Rust)

## Scope Analysis

**Research Question:** What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

**Scope:** `extensions/sql/` (1 file with substantive code, 8 LOC)

## Findings

**Sentinel:** No substantive patterns found for porting IDE core functionality.

The scope is limited to the SQL language extension, which contains only declarative configuration files and a single 8-line build script (`extensions/sql/build/update-grammar.mjs`) that wraps a grammar updater utility.

**Contents Examined:**
- `package.json` (42 LOC) — Extension metadata and language registration
- `language-configuration.json` (41 LOC) — Declarative language rules (comments, brackets, auto-closing pairs, folding)
- `syntaxes/sql.tmLanguage.json` (642 LOC) — TextMate grammar definition
- `build/update-grammar.mjs` (8 LOC) — Script wrapper calling `vscode-grammar-updater`

**Conclusion:** The SQL extension is purely a language grammar definition extension with no core IDE functionality code (no editors, renderers, language servers, UI frameworks, state management, or runtime architecture). It cannot serve as a reference for porting VS Code's core IDE functionality to Tauri/Rust. A broader scope would be required to identify relevant architectural and implementation patterns.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
