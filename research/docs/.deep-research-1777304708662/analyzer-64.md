### Files Analysed

| File | LOC | Role |
|------|-----|------|
| `extensions/razor/package.json` | 50 | Extension manifest — declares language ID, file associations, grammar, and embedded language mappings |
| `extensions/razor/language-configuration.json` | 22 | Editor behaviour config — comment tokens, bracket pairs, auto-closing pairs, surrounding pairs |
| `extensions/razor/build/update-grammar.mjs` | 44 | Build-time script — fetches upstream grammar from `dotnet/razor` and patches one scope reference |
| `extensions/razor/syntaxes/cshtml.tmLanguage.json` | (generated, large) | TextMate grammar — full tokenisation rules for `.cshtml`/`.razor` files |
| `extensions/razor/package.nls.json` | 4 | NLS string table for display name and description |
| `extensions/razor/cgmanifest.json` | 41 | Component governance manifest — records the upstream `dotnet/razor` Git dependency and its MIT licence |
| `extensions/razor/.vscodeignore` | 3 | Packaging exclusion list — strips `test/`, `cgmanifest.json`, and `build/` from the published VSIX |

---

### Per-File Notes

#### `extensions/razor/package.json`

- **Line 2** — `"name": "razor"` — internal extension identifier used by the VS Code extension host.
- **Lines 8–10** — `"engines": { "vscode": "0.10.x" }` — declares the minimum VS Code engine version; the value `0.10.x` is a historical stub common to all built-in extensions, not a real constraint.
- **Lines 11–13** — `"scripts": { "update-grammar": "node ./build/update-grammar.mjs" }` — the single development-time npm script; running it invokes the grammar updater.
- **Lines 16–32** — `contributes.languages` array registers one language object:
  - `"id": "razor"` (line 18) — the canonical language identifier used everywhere in VS Code to reference this language.
  - `"extensions": [".cshtml", ".razor"]` (lines 19–22) — file-extension triggers that activate the language mode.
  - `"aliases": ["Razor", "razor"]` (lines 23–26) — human-readable names shown in the language picker.
  - `"mimetypes": ["text/x-cshtml"]` (lines 27–29) — MIME type association for content-type-based detection.
  - `"configuration": "./language-configuration.json"` (line 30) — points to the editor behaviour config.
- **Lines 33–44** — `contributes.grammars` array registers one grammar object:
  - `"language": "razor"` (line 35) — ties this grammar to the language registered above.
  - `"scopeName": "text.html.cshtml"` (line 36) — the root TextMate scope name; must match the `scopeName` field at the top of `cshtml.tmLanguage.json`.
  - `"path": "./syntaxes/cshtml.tmLanguage.json"` (line 37) — relative path to the grammar file loaded at runtime.
  - `"embeddedLanguages"` (lines 38–43) — maps three inner scopes to language IDs so the VS Code tokeniser can switch modes:
    - `"section.embedded.source.cshtml"` → `"csharp"` — C# code blocks inside `@{ }` and Razor expressions.
    - `"source.css"` → `"css"` — inline `<style>` content.
    - `"source.js"` → `"javascript"` — inline `<script>` content.

#### `extensions/razor/language-configuration.json`

This file is loaded directly by the VS Code editor host at activation time; it requires no TypeScript code to take effect.

- **Lines 2–4** — `comments.blockComment: ["<!--", "-->"]` — defines the HTML block comment delimiters. No line-comment token is specified, meaning **Toggle Line Comment** is a no-op for Razor files.
- **Lines 5–9** — `brackets` array lists three bracket pairs recognised for indentation and folding:
  1. `["<!--", "-->"]` — HTML comment as a bracket pair.
  2. `["{", "}"]` — C#/Razor code block delimiters.
  3. `["(", ")"]` — expression parentheses.
- **Lines 10–16** — `autoClosingPairs` — five pairs that the editor auto-closes when the opening character is typed: `{}`, `[]`, `()`, `''`, `""`. Note that `<>` and `<!--/-->` are not in this list; they are covered by HTML-specific logic in the grammar rather than here.
- **Lines 17–21** — `surroundingPairs` — three pairs that wrap a selection when the opening character is typed: `''`, `""`, and `<>`.

#### `extensions/razor/build/update-grammar.mjs`

This is a Node.js ES-module script executed only during development to regenerate `syntaxes/cshtml.tmLanguage.json`.

- **Line 7** — `import * as vscodeGrammarUpdater from 'vscode-grammar-updater'` — imports the shared grammar-update utility from the VS Code build infrastructure. This package handles HTTP fetching from GitHub, JSON parsing, and file writing.
- **Lines 9–38** — `patchGrammar(grammar)` function:
  - **Line 10** — `grammar.scopeName = 'text.html.cshtml'` — overwrites whatever scope name is present in the upstream file with the VS Code-specific name required by `package.json` line 36.
  - **Lines 14–25** — `visit(rule, parent)` — a recursive depth-first walk over every node in the grammar object tree. For each node where `rule.include` starts with `'text.html.basic'` (line 15), it replaces the value with `'text.html.derivative'` (line 17) and increments `patchCount`. The recursion descends into any property whose value is an object (lines 19–24).
  - **Lines 27–31** — The walk is seeded from two top-level keys: `grammar.repository` and `grammar.patterns`. Each key of those objects is visited independently.
  - **Lines 33–35** — After the walk, if `patchCount !== 4` a `console.warn` fires, signalling that the upstream grammar structure has changed in an unexpected way. The expected patch count of `4` is hard-coded, implying there are exactly four `text.html.basic` include references in the upstream source.
  - The patched grammar object is returned at line 37.
- **Line 40** — `const razorGrammarRepo = 'dotnet/razor'` — the upstream GitHub repository owner/name.
- **Line 41** — `const grammarPath = 'src/Razor/src/Microsoft.VisualStudio.RazorExtension/EmbeddedGrammars/aspnetcorerazor.tmLanguage.json'` — the exact file path inside that repository.
- **Line 42** — `vscodeGrammarUpdater.update(razorGrammarRepo, grammarPath, './syntaxes/cshtml.tmLanguage.json', grammar => patchGrammar(grammar), 'main')` — calls the updater, passing:
  1. source repo slug
  2. source file path
  3. destination local path `./syntaxes/cshtml.tmLanguage.json`
  4. the `patchGrammar` callback applied after download
  5. branch name `'main'`

#### `extensions/razor/syntaxes/cshtml.tmLanguage.json`

This file is the generated output of `update-grammar.mjs`. It is the primary runtime artefact the VS Code tokeniser loads.

- **Lines 1–6** — `information_for_contributors` array — prose comment embedded in JSON directing contributors to the upstream `dotnet/razor` repository.
- **Line 7** — `"version"` — records the upstream Git commit hash (`743f32a68c61809b22fd84e8748c3686ef1bb8b8`) as a URL. This is the canonical version pin also reflected in `cgmanifest.json` line 9.
- **Line 9** — `"scopeName": "text.html.cshtml"` — set by `patchGrammar` line 10; the root scope matched by `package.json`'s grammar contribution.
- **Lines 10–37** — `"injections"` block — injects Razor expression rules into three foreign scopes without modifying those grammars:
  - `"string.quoted.double.html"` and `"string.quoted.single.html"` (lines 11–30) — allows `@(...)` explicit expressions and `@identifier` implicit expressions inside HTML attribute values.
  - `"source.cs"` (lines 31–36) — allows `@<tag>` inline template syntax inside C# code blocks.
- **Lines 39–46** — top-level `"patterns"` — two entries applied in order: `#razor-control-structures` first, then `text.html.derivative` as a fallback. This establishes Razor-first parsing over an HTML base.
- **Lines 47 onward** — `"repository"` — named rule definitions referenced by `#name` throughout the grammar. Key rules encountered in the first 260 lines:
  - `razor-control-structures` (line 48) — master pattern list combining razor-comment, razor-codeblock, explicit-razor-expression, escaped-transition, directives, transitioned-csharp-control-structures, and implicit-expression.
  - `optionally-transitioned-razor-control-structures` (line 73) — same list but uses `optionally-transitioned-csharp-control-structures` instead; used inside codeblock bodies where `@` is optional.
  - `escaped-transition` (line 98) — matches `@@` as `constant.character.escape.razor.transition`.
  - `transition` (line 102) — matches single `@` as `keyword.control.cshtml.transition`.
  - `razor-codeblock` (line 106) — matches `@{...}` blocks; sets `contentName: "source.cs"` so C# tokenisation applies inside the braces; recurses via `razor-codeblock-body`.
  - `razor-codeblock-body` (line 134) — handles text-tag, inline-template, wellformed-html, razor-single-line-markup, optionally-transitioned-razor-control-structures, and raw `source.cs`.
  - `razor-single-line-markup` (line 156) — matches `@:` followed by rest-of-line, tokenising the remainder as HTML.
  - `text-tag` (line 174) — matches `<text>...</text>` as an HTML escape hatch inside code blocks.
  - `inline-template` (line 196) — dispatches to void or non-void tag variants for `@<tag>` syntax in C# contexts.
  - `inline-template-void-tag` (line 206) — matches `@<area|base|br|col|...>` (HTML void elements) with an optional `!` opt-out prefix.
  - `inline-template-non-void-tag` (line 242) — matches `@<anyTag>` for non-void elements.

#### `extensions/razor/cgmanifest.json`

- **Lines 3–11** — single registration of the `dotnet/razor` Git component. The `commitHash` `743f32a68c61809b22fd84e8748c3686ef1bb8b8` (line 9) must be kept in sync with the `"version"` field of `cshtml.tmLanguage.json` line 7. This file is excluded from the published VSIX by `.vscodeignore`.

#### `extensions/razor/.vscodeignore`

- **Line 1** — `test/**` — excludes any test directory.
- **Line 2** — `cgmanifest.json` — the component governance manifest is build-metadata only.
- **Line 3** — `build/**` — the entire `build/` directory (containing `update-grammar.mjs`) is stripped, as it is a development tool not needed at runtime.

---

### Cross-Cutting Synthesis

The Razor extension is a pure declarative language extension: there is no TypeScript activation code, no `activationEvents`, and no `main` entry point in `package.json`. The entire runtime footprint consists of three artefacts consumed directly by the VS Code extension host:

1. **`package.json`** registers the `razor` language ID and binds it to file extensions, a MIME type, a language configuration, and a grammar. The `embeddedLanguages` map in the grammar contribution (lines 38–43) is the mechanism that allows the editor to switch tokeniser state — and therefore syntax highlighting, bracket matching, and IntelliSense triggers — among Razor, C#, CSS, and JavaScript within a single file.

2. **`language-configuration.json`** provides purely editor-level behaviour. Because Razor is HTML-first, the comment pair is HTML `<!--/-->`. Bracket matching covers `{}` (C# code blocks) and `()` (expressions) in addition to HTML comments. The absence of `<>` in `autoClosingPairs` (it only appears in `surroundingPairs`) means angle brackets are not auto-completed on typing but will wrap selected text.

3. **`syntaxes/cshtml.tmLanguage.json`** is the primary complexity surface. It layers Razor tokenisation on top of `text.html.derivative` (the VS Code derivative of the standard HTML grammar) using two mechanisms:
   - The top-level `patterns` list (lines 39–46) places Razor rules before the HTML fallback so `@`-prefixed constructs are consumed first.
   - The `injections` block (lines 10–37) reaches into `source.cs`, `string.quoted.double.html`, and `string.quoted.single.html` scopes — defined by other grammars — and adds Razor-specific patterns there without modifying those grammars.

   The `@` character is the central lexical pivot: `@@` escapes to a literal `@` (escaped-transition rule), while a single `@` begins a Razor construct whose kind is determined by the following character (`{` for a code block, `(` for an explicit expression, a letter/digit for an implicit expression, a directive keyword, or a known control-structure keyword).

The `build/update-grammar.mjs` script maintains the boundary between the upstream `dotnet/razor` grammar and the VS Code-specific embedding. The single patch it applies — replacing all four occurrences of `text.html.basic` with `text.html.derivative` — ensures the grammar composes with VS Code's own HTML grammar chain rather than directly depending on the generic TextMate HTML grammar bundle. The hard-coded sentinel value of `4` expected patches (line 33) creates an explicit warning if the upstream changes the number of such references.

The component governance manifest (`cgmanifest.json`) and the `"version"` field in `cshtml.tmLanguage.json` together record the exact upstream commit from which the grammar was generated. These must be manually kept in sync when `update-grammar.mjs` is re-run.

For a Tauri/Rust port, the grammar and language-configuration files are format-portable: any editor backend that consumes TextMate grammars (e.g., `syntect` in Rust) can ingest `cshtml.tmLanguage.json` directly. The `language-configuration.json` format is VS Code-specific but maps straightforwardly to equivalent editor configuration structures. The `embeddedLanguages` mapping has no direct equivalent in `syntect` and would need to be re-implemented at the editor level if multi-language tokenisation within a single file is required.

---

### Out-of-Partition References

The following symbols and paths are referenced by files within this partition but are defined outside `extensions/razor/`:

| Reference | Location in partition | Defined / provided by |
|---|---|---|
| `text.html.derivative` | `syntaxes/cshtml.tmLanguage.json` lines 44, 45, 169; `build/update-grammar.mjs` line 17 | `extensions/html/syntaxes/html-derivative.tmLanguage.json` — the VS Code built-in HTML derivative grammar |
| `text.html.basic` | `build/update-grammar.mjs` line 15 (pattern being replaced) | Standard TextMate HTML grammar bundle; present in upstream `dotnet/razor` before patching |
| `source.cs` | `syntaxes/cshtml.tmLanguage.json` lines 31, 121, 152 | `extensions/csharp/` or an external C# grammar extension; not bundled here |
| `source.css` | `package.json` line 40 (`embeddedLanguages`) | `extensions/css/` built-in CSS grammar |
| `source.js` | `package.json` line 41 (`embeddedLanguages`) | `extensions/javascript/` built-in JavaScript grammar |
| `vscode-grammar-updater` npm package | `build/update-grammar.mjs` line 7 | VS Code build infrastructure package; resolved from the workspace `node_modules` at build time |
| `dotnet/razor` GitHub repository | `build/update-grammar.mjs` line 40; `cgmanifest.json` line 7 | External upstream repository `https://github.com/dotnet/razor`, path `src/Razor/src/Microsoft.VisualStudio.RazorExtension/EmbeddedGrammars/aspnetcorerazor.tmLanguage.json` |
| NLS key interpolation (`%displayName%`, `%description%`) | `package.json` lines 3–4 | Resolved at runtime by the VS Code extension host against `package.nls.json` (within partition) and locale-specific `package.nls.*.json` files (none present here) |
