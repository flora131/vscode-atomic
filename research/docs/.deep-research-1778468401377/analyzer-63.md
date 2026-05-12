### Files Analysed

1. `extensions/razor/package.json` (50 lines)
2. `extensions/razor/language-configuration.json` (22 lines)
3. `extensions/razor/build/update-grammar.mjs` (44 lines)
4. `extensions/razor/package.nls.json` (4 lines)
5. `extensions/razor/cgmanifest.json` (41 lines)

---

### Per-File Notes

#### `extensions/razor/package.json`

- **Line 2**: Extension name is `"razor"`, version `10.0.0`, publisher `"vscode"`.
- **Lines 11–13**: Single npm script `"update-grammar"` invokes `node ./build/update-grammar.mjs`.
- **Lines 16–32**: Contributes one language entry with `id: "razor"`, file extensions `.cshtml` and `.razor`, mime type `text/x-cshtml`, pointing to `language-configuration.json`.
- **Lines 33–44**: Contributes one grammar entry: scope name `text.html.cshtml`, grammar file `./syntaxes/cshtml.tmLanguage.json`. Declares three embedded languages:
  - `"section.embedded.source.cshtml"` → `"csharp"`
  - `"source.css"` → `"css"`
  - `"source.js"` → `"javascript"`

  This means the Razor grammar file carries embedded-language scope tokens that VS Code's tokenization engine uses to hand off to CSS and JavaScript grammars within a single `.cshtml` file.

#### `extensions/razor/language-configuration.json`

- **Lines 2–4**: Block comment delimiters defined as `<!--` / `-->` (HTML-style, not Razor `@* *@`).
- **Lines 5–9**: Three bracket pairs for matching and folding: `<!--`/`-->`, `{`/`}`, `(`/`)`.
- **Lines 10–16**: Five auto-closing pairs: `{}`, `[]`, `()`, single quotes, double quotes.
- **Lines 17–21**: Three surrounding pairs: single quote, double quote, and `<`/`>`.

  No word pattern or indentation rules are defined; the file delegates all syntax-aware logic to the TextMate grammar.

#### `extensions/razor/build/update-grammar.mjs`

- **Line 7**: Imports `vscode-grammar-updater` — an npm utility shared across VS Code built-in extensions for fetching upstream grammars from GitHub.
- **Lines 9–38**: Defines `patchGrammar(grammar)`:
  - **Line 10**: Overwrites `grammar.scopeName` with `"text.html.cshtml"` to match the scope name declared in `package.json`.
  - **Lines 14–25**: Recursive `visit()` function walks every node in the grammar tree. When it finds a rule whose `include` starts with `"text.html.basic"`, it rewrites it to `"text.html.derivative"`. This redirects the Razor grammar's HTML base-grammar dependency from the vanilla `text.html.basic` scope to VS Code's `text.html.derivative` scope (which layers on top of `text.html.basic`).
  - **Lines 33–35**: Asserts exactly 4 substitutions occurred; emits a `console.warn` if the count differs.
- **Lines 40–42**: Calls `vscodeGrammarUpdater.update()` with:
  - Source repo: `dotnet/roslyn`
  - Upstream grammar path: `src/Razor/src/Razor/src/Microsoft.VisualStudio.RazorExtension/EmbeddedGrammars/aspnetcorerazor.tmLanguage.json`
  - Local destination: `./syntaxes/cshtml.tmLanguage.json`
  - Transform callback: `patchGrammar`
  - Branch: `"main"`

#### `extensions/razor/package.nls.json`

- **Lines 2–3**: Provides localized strings for `%displayName%` (`"Razor Language Basics"`) and `%description%` (`"Provides syntax highlighting, bracket matching and folding in Razor files."`).

#### `extensions/razor/cgmanifest.json`

- **Lines 3–9**: Registers one component of type `git` referencing `dotnet/roslyn` at commit `79e211a2e4287f9f7508089b81311b2c7fdc169f`, under MIT license.
- This component governance entry tracks the third-party grammar source so that Microsoft's open-source compliance tooling can audit it.

---

### Cross-Cutting Synthesis

The Razor extension is a pure grammar/configuration extension with no runtime TypeScript activation code. Its surface area is 44 lines across the build script and a handful of static JSON files.

The extension contributes a single language (`razor`), binding `.cshtml` and `.razor` file extensions to a TextMate grammar (`cshtml.tmLanguage.json`) whose scope name is `text.html.cshtml`. The grammar is not authored in this repository: `update-grammar.mjs` fetches it from the `dotnet/roslyn` upstream on the `main` branch, then applies a single structural patch — rewriting all `text.html.basic` include references to `text.html.derivative` (4 occurrences asserted at line 33) and stamping the correct `scopeName`. The patched grammar is stored locally at `syntaxes/cshtml.tmLanguage.json`.

The embedded-language declarations in `package.json` (lines 38–43) are the mechanism by which VS Code activates C#, CSS, and JavaScript tokenizers inside Razor code islands, enabling token-aware features (bracket matching, folding, semantic tokens) in mixed-language files without any JavaScript extension host process. The `language-configuration.json` adds only bracket/auto-close rules; deeper language intelligence is expected to be provided by a separate Razor language server extension (e.g., the C# Dev Kit).

For a Tauri/Rust port, the grammar file itself is format-neutral (JSON TextMate grammar), but the embedded-language scope mapping mechanism, the `vscodeGrammarUpdater` npm toolchain, and the `text.html.derivative` scope dependency are all VS Code-specific APIs and infrastructure that would need equivalent counterparts in the target platform.

---

### Out-of-Partition References

- `extensions/html/syntaxes/` — defines `text.html.derivative` and `text.html.basic` scopes that the Razor grammar includes via the patched references (`update-grammar.mjs:17`).
- `node_modules/vscode-grammar-updater/` — shared npm utility used at `update-grammar.mjs:7`; governs the fetch-and-patch workflow for all built-in grammar extensions.
- `extensions/css/` and `extensions/javascript/` — provide `source.css` and `source.js` grammars that are activated inside Razor files via the `embeddedLanguages` map in `package.json:38–43`.
- `extensions/csharp/` (if present) or the C# Dev Kit extension — provides the `csharp` grammar backing `section.embedded.source.cshtml` token scope declared at `package.json:39`.
