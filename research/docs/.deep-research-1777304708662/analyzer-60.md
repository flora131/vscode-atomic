### Files Analysed

| File | Lines | Role |
|---|---|---|
| `extensions/php/package.json` | 69 | Extension manifest — language/grammar/snippet contribution points |
| `extensions/php/language-configuration.json` | 174 | Editor behaviour rules (comments, brackets, indentation, folding, on-enter) |
| `extensions/php/build/update-grammar.mjs` | 75 | Build-time grammar updater with upstream patches |
| `extensions/php/snippets/php.code-snippets` | 371 | ~35 tabstop-based code templates |
| `extensions/php/syntaxes/php.tmLanguage.json` | 4191 | Pure PHP TextMate grammar (`source.php`) |
| `extensions/php/syntaxes/html.tmLanguage.json` | 97 | HTML+PHP mixed-file TextMate grammar (`text.html.php`) |
| `extensions/php/package.nls.json` | 4 | Localisation strings for `displayName` and `description` |
| `extensions/php/cgmanifest.json` | 16 | Component governance — pins upstream `language-php` at commit `cd607a5` |

---

### Per-File Notes

#### `extensions/php/package.json` (lines 1–69)

- **Language registration** (lines 13–32): declares language id `"php"`, maps file extensions `.php`, `.php4`, `.php5`, `.phtml`, `.ctp`, the shebang first-line pattern `^#!\s*/.*\bphp\b` (line 27), and MIME type `application/x-php` (line 29). Points to `language-configuration.json` at line 31.
- **Grammar registrations** (lines 34–53): two grammars are contributed.
  - `source.php` scope at `./syntaxes/php.tmLanguage.json` (line 38) — standalone PHP scope used when the file is pure PHP.
  - `text.html.php` scope at `./syntaxes/html.tmLanguage.json` (lines 43–53) — HTML-embedded PHP. The `embeddedLanguages` map at lines 44–52 binds seven inner scopes (`text.html`, `source.php`, `source.sql`, `text.xml`, `source.js`, `source.json`, `source.css`) to their respective language ids, enabling tokenisation-based language switching inside `.php` HTML templates.
- **Snippet registration** (lines 55–59): `./snippets/php.code-snippets` linked to language id `"php"`.
- **Build script** (line 63): `"update-grammar": "node ./build/update-grammar.mjs"` — the only developer-facing script in the extension.

#### `extensions/php/language-configuration.json` (lines 1–174)

- **Comments** (lines 2–8): line comment `//` (with inline annotation noting `#` is also valid in PHP), block comment `/* … */`.
- **Brackets** (lines 9–22): three pairs `{}`, `[]`, `()`.
- **Auto-closing pairs** (lines 23–67): `{`, `[`, `(` excluded in `"string"` scope; single/double quotes excluded in both `"string"` and `"comment"` scopes; `/**` closes to ` */` outside strings (lines 62–67).
- **Surrounding pairs** (lines 69–94): same three bracket pairs plus `'`, `"`, and backtick.
- **Indentation rules** (lines 95–101):
  - `increaseIndentPattern` (line 96) matches `if`, `else`, `for`, `foreach`, `while`, `switch`, `case` followed by `:` on a line-end (covering PHP alternative syntax).
  - `decreaseIndentPattern` (line 97) matches `endif`, `endfor`, `endforeach`, `endwhile`, `endswitch` terminator keywords.
  - `unIndentedLinePattern` (line 99) handles continuation lines inside block comments.
  - `indentNextLinePattern` (line 100) applies single-level indent after braceless `if`/`else`/`while`/`for`/`foreach` lines.
- **Folding markers** (lines 102–107): start `#region` / `// #region`, end `#endregion` / `// #endregion`.
- **Word pattern** (line 108): captures floating-point literals and identifiers; excludes most punctuation characters.
- **On-enter rules** (lines 109–173): six rules.
  - Lines 110–117: inside `/** … */` (both before and after on same line) → `indentOutdent` + append ` * `.
  - Lines 118–124: at end of `/**` line → `none` + append ` * `.
  - Lines 125–134: inside ` * …` continuation → `none` + append `* `.
  - Lines 135–143: after ` */` → `none` + `removeText: 1`.
  - Lines 144–150: after ` *…*/` (inline close) → `none` + `removeText: 1`.
  - Lines 151–159: after single-line braceless control construct → `outdent`.
  - Lines 160–172: enter pressed within a `//` line comment when text follows cursor → `none` + append `// `.

#### `extensions/php/build/update-grammar.mjs` (lines 1–75)

The module imports `vscode-grammar-updater` (line 7), a shared VS Code build utility that fetches CSON grammar files from GitHub, converts them to JSON, applies a callback, and writes to the local `syntaxes/` path.

Three patch functions are defined before the two `update()` calls:

1. **`adaptInjectionScope`** (lines 9–22): modifies the injection scope key in the HTML grammar's `injections` map. The old key (line 12) used `source.js.embedded.html`; the new key (line 13) replaces it with `source.js` and adds an additional `source.css` injection scope. This enables PHP injection to apply inside `<script>` and `<style>` tags as they appear in VS Code's HTML grammar. The function throws if the expected old key is absent (line 18), acting as a structural assertion.

2. **`includeDerivativeHtml`** (lines 24–30): walks the `patterns` array of the HTML grammar and replaces any `include: 'text.html.basic'` with `include: 'text.html.derivative'`, directing the grammar to use VS Code's derivative HTML scope instead of the upstream TextMate HTML scope.

3. **`fixBadRegex`** (lines 32–69): works around issues filed as vscode#40279 and vscode-textmate#59. Both issues relate to the upstream grammar using `(?i)` inline case-insensitive flags in PCRE regex syntax which vscode-textmate's Oniguruma binding does not accept in the same way.
   - Patches `scope-resolution` (lines 39–49): replaces `(?i)([a-z_…)` with an explicit `[A-Za-z_…)` character class.
   - Patches `function-call` patterns[0].begin (lines 51–58): removes `(?xi)` and replaces `[a-z…` ranges with `[a-zA-Z…` ranges.
   - Patches `function-call` patterns[1].begin (lines 60–65): removes `(?i)` and expands character classes to include uppercase letters.

Two `vscodeGrammarUpdater.update()` calls (lines 71–75) drive the actual fetch:
- Line 71: fetches `grammars/php.cson` from `KapitanOczywisty/language-php`, writes `./syntaxes/php.tmLanguage.json`, applying `fixBadRegex`.
- Lines 72–75: fetches `grammars/html.cson`, writes `./syntaxes/html.tmLanguage.json`, applying both `adaptInjectionScope` and `includeDerivativeHtml`.

#### `extensions/php/snippets/php.code-snippets` (lines 1–371)

~35 named snippets for PHP 8.x idioms. Key entries:

- Lines 2–5: ternary `if?` → `$retVal = (condition) ? a : b ;`
- Lines 60–70: `class` snippet uses `TM_FILENAME_BASE` variable, optional `final`/`readonly`, `extends`, `implements`.
- Lines 71–78: `construct` generates `__construct` with `$this->var = $var` body.
- Lines 94–100: `enum` covers PHP 8.1 enum syntax.
- Lines 194–203: `match` generates PHP 8.0 match expression.
- Lines 214–246: `doc_class` / `doc_fun` snippets generate PHPDoc blocks with `@param`, `@return`, `@throws`.
- Lines 277–289: `#region` / `#endregion` markers matching the folding rules in `language-configuration.json`.
- Lines 329–339: `try` generates `try { } catch (\Throwable $th) { }` — uses the `\Throwable` base type.
- Lines 341–361: `use_fun`, `use_const`, `use_group`, `use_as` — namespace use statement variants.

#### `extensions/php/syntaxes/php.tmLanguage.json` (4191 lines)

- Header (lines 0–6): upstream commit `cd607a522b79c457fe78bf7a1b04511d1c49f693` from `KapitanOczywisty/language-php`, scope name `source.php`.
- Top-level `patterns` array (lines 8–onward) includes entries for `#attribute`, `#comments`, `#namespace`, and all other PHP constructs, delegating to the `repository` section via include references.
- The `scope-resolution` and `function-call` repository entries are the targets of the `fixBadRegex` patches in `update-grammar.mjs`.

#### `extensions/php/syntaxes/html.tmLanguage.json` (97 lines)

- Upstream commit `ff64523c94c014d68f5dec189b05557649c5872a`, scope name `text.html.php` (lines 6–8).
- `injections` (lines 9–27): two injection entries. The first (lines 10–18) injects `text.html.basic` and the Blade template grammar into `meta.embedded.php.blade` contexts. The second (lines 20–26) is the patched key from `adaptInjectionScope`, applying `#php-tag` patterns in HTML/JS/CSS regions.
- `patterns` (line 28+): includes `text.html.derivative` (patched by `includeDerivativeHtml`) and PHP-specific overrides.

---

### Cross-Cutting Synthesis

The `extensions/php` extension is a pure declarative language contribution with no runtime TypeScript code. Its entire surface consists of three categories of static data: TextMate grammars for tokenisation, a language configuration for editor mechanics, and code snippets for template insertion. The grammar layer is split into two scopes — `source.php` for standalone PHP files and `text.html.php` for HTML-embedded PHP — with the latter using the `embeddedLanguages` map in `package.json` to hand off inner regions (JavaScript, CSS, SQL, XML, JSON) to their respective language servers and tokenisers. The build script `update-grammar.mjs` acts as a thin integration layer: it pulls the upstream CSON grammar from the `KapitanOczywisty/language-php` repository, converts it, and applies three deterministic patches to fix regex syntax incompatibilities with vscode-textmate's Oniguruma engine and to align injection scope keys with VS Code's own HTML grammar scope names. No language intelligence (hover, completion, diagnostics) is provided here; that is delegated to the separate `extensions/php-language-features` extension which implements the PHP language server integration. In a Tauri/Rust port, the grammar data files (`.tmLanguage.json`) and snippet files are engine-agnostic and reusable directly; the language configuration rules would need to be parsed and applied by the Rust editor engine; and the build script's `vscode-grammar-updater` dependency would need a Rust or Node equivalent for grammar update workflows.

---

### Out-of-Partition References

- `extensions/php-language-features/` — the companion extension that provides IntelliSense, diagnostics, hover, and go-to-definition for PHP; consumes the `"php"` language id registered here.
- `extensions/html/syntaxes/` — registers `text.html.derivative` and `text.html.basic` scopes that `html.tmLanguage.json` includes after the `includeDerivativeHtml` patch (`update-grammar.mjs:25-29`).
- `extensions/vscode-colorize-tests/test/colorize-results/test_php.json`, `issue-28354_php.json`, `issue-76997_php.json` — integration colorisation test fixtures that validate token output from `source.php` and `text.html.php` grammars.
- `extensions/razor/build/update-grammar.mjs` — parallel build script using the same `vscode-grammar-updater` pattern for the Razor/C# HTML grammar, and also references `text.html.derivative`.
- `extensions/search-result/syntaxes/generateTMLanguage.js` — references `source.php` scope as one of the known language scopes in search result highlighting.
- `extensions/theme-monokai-dimmed/themes/dimmed-monokai-color-theme.json` and `extensions/theme-tomorrow-night-blue/themes/tomorrow-night-blue-color-theme.json` — both contain per-scope colour rules targeting `source.php` and `text.html.php` token scopes.
