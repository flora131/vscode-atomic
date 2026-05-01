### Files Analysed

- `extensions/html/package.json`
- `extensions/html/language-configuration.json`
- `extensions/html/build/update-grammar.mjs`
- `extensions/html/cgmanifest.json`
- `extensions/html/package.nls.json`
- `extensions/html/syntaxes/html.tmLanguage.json` (examined by locator; declarative TextMate JSON, not read in full)
- `extensions/html/syntaxes/html-derivative.tmLanguage.json` (examined by locator; declarative TextMate JSON, not read in full)
- `extensions/html/snippets/html.code-snippets` (examined by locator; declarative snippet JSON)
- `extensions/html/.vscodeignore`

### Per-File Notes

#### `extensions/html/package.json`
- **Role:** Extension manifest declaring language contributions for HTML. No runtime code.
- **Key symbols:** `contributes.languages` registers the `html` language ID with file extensions (`.html`, `.htm`, `.shtml`, `.xhtml`, `.xht`, `.mdoc`, `.jsp`, `.asp`, `.aspx`, `.jshtm`, `.volt`, `.ejs`, `.rhtml`) at lines 17–48. `contributes.grammars` registers two TextMate grammars at lines 50–79: `text.html.basic` (via `syntaxes/html.tmLanguage.json`) and `text.html.derivative` (via `syntaxes/html-derivative.tmLanguage.json`), each with embedded language scopes for `css`, `javascript`, `python`, and `smarty`. `contributes.snippets` at lines 81–86 points to `snippets/html.code-snippets`.
- **Control flow:** None — pure declarative JSON consumed by the VS Code extension host at startup.
- **Data flow:** The `tokenTypes` field at lines 61–63 and 76–78 maps `meta.tag string.quoted` to token type `other`, influencing how the editor tokenizes attribute string values.
- **Dependencies:** No npm runtime dependencies. Build script uses `vscode-grammar-updater` (dev tooling only).

#### `extensions/html/language-configuration.json`
- **Role:** Declarative editor behavior configuration for the `html` language ID. Consumed by VS Code's language configuration service at runtime.
- **Key symbols:**
  - `comments.blockComment` (line 3): defines `<!-- -->` as block comment delimiters.
  - `brackets` (lines 5–9): defines bracket pairs `<!-- -->`, `{ }`, `( )` for bracket matching.
  - `autoClosingPairs` (lines 10–17): defines six pairs including `<!--` / `-->` with a `notIn` guard for `comment` and `string` contexts.
  - `surroundingPairs` (lines 18–25): six pairs used when text is selected and a delimiter is typed.
  - `colorizedBracketPairs` (line 26–27): empty array, disabling bracket colorization.
  - `folding.markers` (lines 28–33): regex patterns `^\s*<!--\s*#region\b.*-->` and `^\s*<!--\s*#endregion\b.*-->` define region folding.
  - `wordPattern` (line 34): regex controlling word boundary detection for double-click selection.
  - `onEnterRules` (lines 35–48): two rules using `beforeText`/`afterText` regex patterns to apply `indentOutdent` or `indent` actions when Enter is pressed inside HTML tags; void elements (area, base, br, col, embed, hr, img, input, keygen, link, menuitem, meta, param, source, track, wbr) are excluded.
  - `indentationRules` (lines 50–53): `increaseIndentPattern` and `decreaseIndentPattern` regexes drive automatic indentation.
- **Control flow:** None — pure declarative JSON.
- **Data flow:** Consumed directly by the VS Code editor core's language configuration service; no transformation occurs within this file.
- **Dependencies:** None.

#### `extensions/html/build/update-grammar.mjs`
- **Role:** Build-time Node.js script that fetches upstream TextMate grammars from `textmate/html.tmbundle` on GitHub and writes patched versions to `syntaxes/`. Not shipped in the extension; excluded via `.vscodeignore`.
- **Key symbols:**
  - `patchGrammar(grammar)` (lines 9–36): Performs a recursive tree walk of the grammar's `repository` object. When it finds a rule whose `name` is `source.js` or `source.css`, and whose parent is not `punctuation.definition.string.end.html`, and whose grandparent's `property` is `endCaptures`, it renames the scope to `source.js-ignored-vscode` or `source.css-ignored-vscode`. Expects exactly 2 such patches; warns otherwise (line 31).
  - `patchGrammarDerivative(grammar)` (lines 38–53): Iterates `grammar.patterns`. Finds the pattern with `name === 'meta.tag.other.unrecognized.html.derivative'` and `begin === '(</?)(\\w[^\\s>]*)(?<!/)` and replaces the `begin` regex with `'(</?)(\\w[^\\s<>]*)(?<!/'` (adding `<` to the negative character class) at line 44. Expects exactly 1 such patch; warns otherwise (line 49).
  - Lines 55–60: Calls `vscodeGrammarUpdater.update()` twice — once for the main HTML grammar (`Syntaxes/HTML.plist` from `textmate/html.tmbundle`) and once for the derivative grammar (`Syntaxes/HTML%20%28Derivative%29.tmLanguage`), passing the respective patch functions as callbacks.
- **Control flow:** Sequential top-level script execution: define patch functions, then invoke `vscodeGrammarUpdater.update` for each grammar file.
- **Data flow:** Upstream grammar (plist/XML format) is fetched and parsed by `vscode-grammar-updater`, converted to a JS object, passed through the patch callback, then serialized to JSON and written to `./syntaxes/*.tmLanguage.json`.
- **Dependencies:** `vscode-grammar-updater` (external npm package, dev-only).

#### `extensions/html/cgmanifest.json`
- **Role:** Component governance manifest tracking the upstream open-source dependency (`textmate/html.tmbundle` at commit `0c3d5ee54de3a993f747f54186b73a4d2d3c44a2`) for license compliance.
- **Key symbols:** `registrations[0].component.git` (lines 5–9) specifies the upstream repo and commit. `licenseDetail` (lines 10–28) contains the full license text (permissive TextMate Bundle License).
- **Control flow:** None — declarative JSON consumed by tooling, not the extension runtime.
- **Dependencies:** None.

#### `extensions/html/package.nls.json`
- **Role:** English localization strings for `package.json` display name and description placeholders (`%displayName%`, `%description%`).
- **Key symbols:** `displayName` → `"HTML Language Basics"` (line 2); `description` → `"Provides syntax highlighting, bracket matching & snippets in HTML files."` (line 3).
- **Control flow:** None.
- **Dependencies:** None.

### Cross-Cutting Synthesis

This partition (`extensions/html/`) is entirely declarative and build-tooling in nature. It contains no runtime TypeScript or Rust code. The extension contributes HTML syntax highlighting, bracket matching, region folding, auto-indentation, and code snippets to VS Code purely through JSON manifests and TextMate grammar files. The only executable file is `build/update-grammar.mjs`, a one-time build script that fetches and patches upstream TextMate grammars into the `syntaxes/` directory; it is excluded from the published extension via `.vscodeignore`. None of these files are relevant to porting VS Code's core IDE runtime functionality (editor, LSP client, debugger, source control, terminal, navigation) to Tauri/Rust. The port effort for this specific partition would be limited to ensuring that the target IDE framework supports TextMate grammar-based tokenization and a language-configuration contract equivalent to VS Code's — both of which are pre-existing concerns in any Electron-to-Tauri migration and are not addressed here.

### Out-of-Partition References

- `extensions/html-language-features/` — The actual runtime HTML language server client (LSP integration, hover, completion, validation) lives in a sibling extension not in this partition; that is where porting effort for HTML intelligence would be concentrated.
- `src/vs/workbench/services/languageFeatures/` — VS Code core language configuration service that consumes `language-configuration.json` at runtime; not in this partition.
