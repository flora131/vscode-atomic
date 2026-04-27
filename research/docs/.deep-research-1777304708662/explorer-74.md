# Partition 74 of 79 — Findings

## Scope
`extensions/latex/` (1 files, 13 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# LaTeX Extension - File Locations

## Overview
The `extensions/latex/` directory is a pure grammar and snippet extension (no TypeScript/JavaScript implementation code) that provides syntax highlighting, language configuration, and grammar definitions for LaTeX, TeX, and BibTeX file types.

## Implementation
- `extensions/latex/package.json` - Extension manifest defining language contributions, grammar rules, and build scripts
- `extensions/latex/build/update-grammars.js` - Build utility for updating TextMate grammar files

## Configuration
- `extensions/latex/latex-language-configuration.json` - Editor behavior for LaTeX/TeX (bracket matching, indentation, folding, word pattern rules, auto-closing pairs for LaTeX constructs)
- `extensions/latex/latex-cpp-embedded-language-configuration.json` - Editor behavior for C++ embedded in LaTeX (pragma-based folding regions)
- `extensions/latex/markdown-latex-combined-language-configuration.json` - Combined Markdown + LaTeX language configuration
- `extensions/latex/.vscodeignore` - Build packaging ignore rules

## Grammar Definitions (TextMate Format)
- `extensions/latex/syntaxes/LaTeX.tmLanguage.json` (4,446 LOC) - Main LaTeX syntax grammar with embedded language support for C++, JavaScript, TypeScript, Python, Java, Ruby, Lua, Julia, YAML, XML, CSS, HTML
- `extensions/latex/syntaxes/TeX.tmLanguage.json` (376 LOC) - TeX syntax grammar
- `extensions/latex/syntaxes/Bibtex.tmLanguage.json` (341 LOC) - BibTeX bibliography syntax grammar
- `extensions/latex/syntaxes/markdown-latex-combined.tmLanguage.json` (3,276 LOC) - Combined Markdown + LaTeX syntax for code blocks
- `extensions/latex/syntaxes/cpp-grammar-bailout.tmLanguage.json` (20,054 LOC) - C++ embedded grammar fallback for LaTeX document mode

## License & Attribution
- `extensions/latex/cpp-bailout-license.txt` - License for C++ grammar bailout component
- `extensions/latex/markdown-latex-combined-license.txt` - License for markdown-latex-combined grammar

## Metadata
- `extensions/latex/cgmanifest.json` - Component governance manifest
- `extensions/latex/package.nls.json` - Localization strings for UI text (displayName, description)

## Notable Clusters
**Grammar System**: The extension demonstrates complex TextMate grammar composition with:
- Recursive embedding of 10+ programming languages within LaTeX documents
- Advanced bracket matching rules for LaTeX mathematical delimiters (unbalanced scope handling for constructs like `\left(` and `\right)`)
- Folding region markers for both LaTeX (`%region`/`%endregion`, `\begingroup`/`\endgroup`) and C++ (`#pragma region`/`#pragma endregion`)
- Grammar bailout mechanism using a large C++ grammar (20K LOC) as fallback

## Relevance to Tauri/Rust Porting Research
This extension is **grammar-only** with no custom IDE logic, language server, or Electron-specific APIs. Porting implications:
- TextMate grammar files (JSON-based) are platform-agnostic and would require no rewrite
- Language configuration JSON is consumed by the core editor and would transfer directly
- The `update-grammars.js` build script is a simple utility that could be rewritten in Rust if needed
- No dependency on TypeScript runtime, VS Code extension API, or Electron-specific features beyond grammar registration
- Most portable extension type in VS Code's ecosystem (pure declarative syntax definitions)

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
# Analyzer-74: extensions/latex — Grammar-Only LaTeX/TeX/BibTeX Extension

> Scope: `extensions/latex/` (grammar-only partition)
> Research question: What would it take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust?
> Note: This partition contains no TypeScript/Electron application logic. All portability observations are therefore derived exclusively from the declarative configuration and build tooling present in scope.

---

### Files Analysed

| File | Lines | Read in full? |
|---|---|---|
| `/Users/norinlavaee/vscode-atomic/extensions/latex/package.json` | 120 | Yes |
| `/Users/norinlavaee/vscode-atomic/extensions/latex/build/update-grammars.js` | 14 | Yes |
| `/Users/norinlavaee/vscode-atomic/extensions/latex/latex-language-configuration.json` | 120 | Yes |
| `/Users/norinlavaee/vscode-atomic/extensions/latex/latex-cpp-embedded-language-configuration.json` | 33 | Yes |
| `/Users/norinlavaee/vscode-atomic/extensions/latex/markdown-latex-combined-language-configuration.json` | 126 | Yes |
| `/Users/norinlavaee/vscode-atomic/extensions/latex/package.nls.json` | 4 | Yes |
| `/Users/norinlavaee/vscode-atomic/extensions/latex/cgmanifest.json` | 38 | Yes |
| `/Users/norinlavaee/vscode-atomic/extensions/latex/.vscodeignore` | 3 | Yes |

Large TextMate grammar JSONs (`LaTeX.tmLanguage.json` 4446 LOC, `cpp-grammar-bailout.tmLanguage.json` 20054 LOC, `Bibtex.tmLanguage.json`, `TeX.tmLanguage.json`, `markdown-latex-combined.tmLanguage.json`) were not read per scope instructions.

---

### Per-File Notes

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/package.json`

- **Role:** VS Code extension manifest. Declares the five language IDs contributed by this extension and binds each to a grammar file and a language-configuration file. Acts as the sole integration contract between this extension and the VS Code extension host.
- **Key symbols:**
  - `contributes.languages` (lines 16–64): array of five language descriptors — `tex`, `latex`, `bibtex`, `cpp_embedded_latex`, `markdown_latex_combined`.
  - `contributes.grammars` (lines 65–114): array of five grammar descriptors, one per language, linking language IDs to TextMate scope names and `.tmLanguage.json` paths.
  - `scripts["update-grammar"]` (line 12): single npm script that delegates to `./build/update-grammars.js`.
  - `engines.vscode: "*"` (line 9): declares compatibility with all VS Code engine versions.
- **Control flow:** No runtime control flow; the manifest is consumed statically by the VS Code extension host at activation time. There is no `main` entry point, confirming the extension is purely declarative.
- **Data flow:**
  - `contributes.languages[*].configuration` fields (lines 29, 42, 57, 62) point to the three language-configuration JSON files in the extension root.
  - `contributes.grammars[*].path` fields (lines 69, 81, 103, 107, 111) point into `./syntaxes/`.
  - `contributes.grammars[1].embeddedLanguages` (lines 83–97) maps 13 TextMate scope names to VS Code language IDs, enabling embedded-language tokenization inside LaTeX documents (e.g., `source.cpp` → `cpp_embedded_latex`, `source.python` → `python`).
  - `contributes.grammars[0].unbalancedBracketScopes` and `contributes.grammars[1].unbalancedBracketScopes` (lines 70–73, 79–82) suppress false bracket-match highlights for two TeX-specific scopes.
- **Dependencies:** `vscode` engine (host); no npm runtime dependencies declared; dev dependency is the external `vscode-grammar-updater` package (not present in the repo tree, resolved at build time via node_modules).

---

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/build/update-grammars.js`

- **Role:** Build-time utility script. Fetches the five upstream TextMate grammar files from the `jlelong/vscode-latex-basics` GitHub repository and writes them into `syntaxes/`, overwriting local copies.
- **Key symbols:**
  - `updateGrammar` (line 7): the single imported module, `vscode-grammar-updater`, which provides the `update()` function.
  - Five `updateGrammar.update()` calls (lines 9–13), one per grammar file.
- **Control flow:** Linear, top-to-bottom. Each `update()` call is independent; they are not chained or conditioned on each other. The script terminates after all five calls are initiated (the `vscode-grammar-updater` package handles async HTTP internally).
- **Data flow:**
  - Each `update()` invocation receives four arguments: upstream repository slug (`'jlelong/vscode-latex-basics'`), source path within that repo, local destination path (both identical in all five calls), an `undefined` transform function (no post-processing), and the branch name `'main'`.
  - The function fetches the raw file from the GitHub API and writes it to the local `syntaxes/` path.
  - No data is returned or piped between calls.
- **Dependencies:** `vscode-grammar-updater` (external npm package, not vendored in the repository tree). Requires network access to `api.github.com` at build time. No other imports.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/latex-language-configuration.json`

- **Role:** Editor behavior configuration for the `tex` and `latex` language IDs (both share this file via `package.json` lines 29, 42). Consumed by the VS Code extension host to configure editor features without any runtime code.
- **Key symbols:**
  - `comments.lineComment` (line 3): `"%"` — designates `%` as the TeX/LaTeX line comment character.
  - `brackets` (lines 5–71): 70 bracket pairs, covering standard ASCII pairs (`{}`, `[]`, `()`) plus an extensive set of LaTeX math delimiters (`\left(` / `\right)`, `\bigl` / `\bigr` variants at all four size levels, `\langle` / `\rangle`, `\lvert` / `\rvert`, etc.).
  - `autoClosingPairs` (lines 72–95): 22 pairs; a subset of the bracket list used for automatic closing. Includes the backtick/apostrophe pair `` ` `` / `'` (line 94) for TeX-style quoting.
  - `surroundingPairs` (lines 96–103): 7 pairs including `$`/`$` (line 103) for math-mode toggling.
  - `indentationRules` (lines 105–108): two regex patterns that increase indent after `\begin{...}` and decrease it before `\end{...}`, with an explicit exception for the `document` environment.
  - `folding.markers` (lines 110–113): regex-based fold start/end for `%region` / `%endregion` comments and `\begingroup` / `\endgroup` commands.
  - `autoCloseBefore` (line 115): string of characters after which auto-closing is suppressed.
  - `wordPattern` (lines 116–119): Unicode-aware regex (`\p{Alphabetic}|\p{Number}|\p{Nonspacing_Mark}`) with flag `"u"` for double-click word selection.
- **Control flow:** No control flow; purely declarative JSON consumed by the host.
- **Data flow:** Read once by the extension host at language activation; values are wired into editor subsystems (bracket matching, auto-closing, indentation, folding, word selection). No runtime mutations.
- **Dependencies:** Consumed by VS Code extension host. References no external files. Shared by both `tex` and `latex` language IDs.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/latex-cpp-embedded-language-configuration.json`

- **Role:** Editor behavior configuration for the synthetic `cpp_embedded_latex` language ID — the virtual language used when C++ is embedded inside a LaTeX document via the `\begin{cppcode}` family of environments.
- **Key symbols:**
  - `comments` (lines 2–5): C++ comment styles (`//` line comment, `/* */` block comment).
  - `brackets` (lines 6–10): three standard ASCII pairs only (`{}`, `[]`, `()`).
  - `autoClosingPairs` (lines 11–17): five pairs including single/double-quote pairs with `notIn` guards for `string` and `comment` contexts (lines 15–16).
  - `surroundingPairs` (lines 18–25): six pairs including `<>`.
  - `wordPattern` (line 26): C++ identifier regex matching numeric literals and non-operator sequences.
  - `folding.markers` (lines 27–32): `#pragma region` / `#pragma endregion` patterns.
- **Control flow:** No control flow; purely declarative.
- **Data flow:** Consumed by the extension host when the tokenizer encounters the `source.cpp.embedded.latex` scope and switches to the `cpp_embedded_latex` virtual language for editor features. Values govern bracket matching, auto-close, and folding for the C++ sub-document.
- **Dependencies:** Consumed by VS Code extension host. Linked from `package.json` line 57.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/markdown-latex-combined-language-configuration.json`

- **Role:** Editor behavior configuration for the `markdown_latex_combined` virtual language ID — used for Markdown documents with embedded LaTeX math (as used by Markdown-Math or similar extensions that embed LaTeX grammars into Markdown).
- **Key symbols:**
  - `comments.blockComment` (lines 3–6): HTML block comment `<!-- -->`.
  - `brackets` (lines 7–74): the full LaTeX math delimiter set (identical to `latex-language-configuration.json`) plus the three ASCII pairs.
  - `autoClosingPairs` (lines 75–98): identical to `latex-language-configuration.json`.
  - `surroundingPairs` (lines 99–108): extends the LaTeX set with Markdown-specific pairs: backtick (line 106), underscore (line 107), asterisk (line 108).
  - `indentationRules` (lines 110–113): identical LaTeX `\begin{}`/`\end{}` patterns.
  - `folding` (lines 114–119): uses `offSide: true` (line 115) and HTML comment region markers (`<!-- #region -->` / `<!-- #endregion -->`), contrasting with the `%region` markers in the pure LaTeX config.
  - `autoCloseBefore` (line 121): same string as the LaTeX config.
  - `wordPattern` (lines 122–125): extends the LaTeX Unicode word pattern to optionally wrap the match in Markdown emphasis delimiters (`[*_]{1,2}`).
- **Control flow:** No control flow; purely declarative.
- **Data flow:** Consumed by the extension host for the `markdown_latex_combined` embedded virtual language. Provides combined Markdown-and-LaTeX editor behaviors in a single configuration object.
- **Dependencies:** Consumed by VS Code extension host. Linked from `package.json` line 62.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/package.nls.json`

- **Role:** Localization string table for the extension manifest. Provides the English display strings for the `%displayName%` and `%description%` tokens in `package.json` lines 3–4.
- **Key symbols:**
  - `displayName` (line 2): `"LaTeX Language Basics"`
  - `description` (line 3): `"Provides syntax highlighting and bracket matching for TeX, LaTeX and BibTeX."`
- **Control flow:** None; consumed statically by VS Code's extension NLS loader.
- **Data flow:** Values substitute into `package.json` at extension-host load time for display in the Extensions panel and marketplace.
- **Dependencies:** VS Code extension host NLS subsystem.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/cgmanifest.json`

- **Role:** Component Governance manifest. Registers the single upstream open-source component (`jlelong/vscode-latex-basics`) that supplies all five grammar files, for legal/license-tracking tooling used by Microsoft's supply-chain compliance process.
- **Key symbols:**
  - `registrations[0].component.git.repositoryUrl` (line 8): `https://github.com/jlelong/vscode-latex-basics`
  - `registrations[0].component.git.commitHash` (line 9): `76dc409348227db00f6779772f7763dc90cdf22e` — the pinned upstream commit.
  - `registrations[0].version` (line 12): `"1.16.0"`
  - `registrations[0].licenseDetail` (lines 13–33): describes the MIT base license and two grammar-file-specific license overrides (the original LaTeX.tmbundle permissive license and the `cpp-grammar-bailout` file's separate license described in `cpp-bailout-license.txt`).
- **Control flow:** None; consumed by external Microsoft Component Governance tooling, not by VS Code itself.
- **Data flow:** Provides audit metadata linking the local `syntaxes/` files to their upstream source at a specific commit. The commit hash in this file should correspond to what `update-grammars.js` would fetch from the `main` branch at the time of the last grammar update.
- **Dependencies:** External Microsoft CG tooling. Not referenced at runtime.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/latex/.vscodeignore`

- **Role:** Packaging exclusion list. Specifies paths omitted when the extension is packaged into a `.vsix` file by `vsce`.
- **Key symbols:**
  - `cgmanifest.json` (line 1): excluded from the packaged extension.
  - `build/**` (line 2): the entire `build/` directory (containing `update-grammars.js`) is excluded from the package.
- **Control flow:** None; consumed by the `vsce` packaging tool.
- **Data flow:** Ensures that build-time tooling (`update-grammars.js`) and compliance metadata (`cgmanifest.json`) are not shipped inside the published extension artifact.
- **Dependencies:** `vsce` (VS Code Extension packaging tool).

---

### Cross-Cutting Synthesis

The `extensions/latex/` partition is a purely declarative, grammar-only VS Code extension. It contributes five language IDs (`tex`, `latex`, `bibtex`, `cpp_embedded_latex`, `markdown_latex_combined`) to the VS Code extension host. There is no TypeScript, no `main` activation entry point, and no runtime code whatsoever — the extension consists entirely of JSON manifests, JSON language-configuration files, and the five TextMate grammar files in `syntaxes/`. The only imperative code is `build/update-grammars.js`, which is a one-time build-time script (explicitly excluded from the packaged extension by `.vscodeignore`) that pulls upstream grammar files from `jlelong/vscode-latex-basics` via the `vscode-grammar-updater` npm package.

From a Tauri/Rust porting perspective, this partition presents essentially zero porting complexity on its own terms. The TextMate grammars, language configurations, and manifest structure are consumed by VS Code's extension host JavaScript runtime. In a Tauri/Rust host, the equivalent subsystem would need to implement or embed a TextMate grammar engine (such as `syntect` in Rust, which already supports `.tmLanguage` grammars) and a language-configuration loader. The five `.tmLanguage.json` files themselves are format-portable. The `embeddedLanguages` map in `package.json` lines 83–97 is a VS Code-specific extension API concept that a Tauri host would need to re-implement independently. The `build/update-grammars.js` script has no dependency on Electron or VS Code internals and would remain usable as-is for maintaining grammar freshness regardless of the target editor platform.

---

### Out-of-Partition References

- `/Users/norinlavaee/vscode-atomic/extensions/markdown-math/package.json` — references the `latex` language ID (confirmed by grep), indicating a cross-extension dependency where markdown-math embeds LaTeX grammar scopes.
- `/Users/norinlavaee/vscode-atomic/extensions/markdown-basics/package.json` — also references the `latex` language ID, indicating the markdown-basics extension integrates with the LaTeX grammar for combined rendering.
- The `cpp_embedded_latex` and `markdown_latex_combined` virtual language IDs are private to this extension (aliases are empty arrays, `package.json` lines 57, 62), meaning they are not directly user-selectable but exist purely as embedded-language targets inside the LaTeX tokenizer pipeline.
- The `vscode-grammar-updater` npm package is resolved at build time and is not present in the repository tree; its API contract (four-argument `update(repo, srcPath, destPath, transform, branch)`) is the only external interface exercised by this partition.

---

This partition is a sentinel case for the Tauri/Rust porting research question: because the extension contains no TypeScript application logic and no Electron API calls, it represents the easiest category of VS Code extension to carry forward to an alternative host. The full porting burden for language support of this kind shifts entirely onto the host platform's ability to load and execute TextMate grammars and honor the declarative language-configuration contract — both of which are well-defined, documented, and already partially addressed by Rust ecosystem libraries such as `syntect`.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: LaTeX Extension (grammar-only scope)

## Scope Summary
- **Path:** `extensions/latex/`
- **Files analyzed:** 3 language configuration files + 5 TextMate grammar files
- **Total LOC:** 13 LOC in grammar/configuration scope
- **Research focus:** Grammar patterns and snippet configurations for Tauri/Rust porting analysis

---

## Pattern 1: TextMate Grammar with Scope Names and Pattern Includes

**Where:** `extensions/latex/syntaxes/LaTeX.tmLanguage.json:1-80`

**What:** Root-level TextMate grammar structure defining nested pattern inclusions for LaTeX syntax highlighting. Uses `include` directives to compose rules from a central repository, enabling modular syntax definition.

```json
{
  "name": "LaTeX",
  "scopeName": "text.tex.latex",
  "patterns": [
    {
      "match": "(?<=\\\\[\\w@]|\\\\[\\w@]{2}|...",
      "comment": "...",
      "name": "meta.space-after-command.latex"
    },
    { "include": "#songs-env" },
    { "include": "#embedded-code-env" },
    { "include": "#verbatim-env" },
    { "include": "#document-env" },
    { "include": "#all-balanced-env" },
    ...
    { "include": "text.tex" }
  ],
  "repository": { ... }
}
```

**Variations / call-sites:**
- Main LaTeX grammar (`LaTeX.tmLanguage.json`)
- Embedded C++ grammar (`cpp-grammar-bailout.tmLanguage.json`)
- Markdown+LaTeX combined grammar (`markdown-latex-combined.tmLanguage.json`)
- Base TeX grammar (`TeX.tmLanguage.json`)
- BibTeX grammar (`Bibtex.tmLanguage.json`)

**Architectural note:** All five grammar files follow the same compositional pattern—repository-based rule organization with pattern includes for syntax tree building. This pattern enables language-specific extensions and embedded language support.

---

## Pattern 2: Language Configuration with Bracket Matching Rules

**Where:** `extensions/latex/latex-language-configuration.json:5-43`

**What:** Declarative bracket/delimiter matching configuration supporting both standard ASCII brackets and LaTeX-specific delimiters (e.g., `\left(`, `\right)`, `\bigl`, `\biggl`). Uses JSON arrays for paired delimiter definitions, supporting up to 44 distinct bracket pair configurations.

```json
{
  "comments": {
    "lineComment": "%"
  },
  "brackets": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
    ["[", ")"],
    ["(", "]"],
    ["\\left(", "\\right)"],
    ["\\left(", "\\right."],
    ["\\left.", "\\right)"],
    ["\\left[", "\\right]"],
    ...
    ["\\Biggl\\Vert", "\\Biggr\\Vert"]
  ]
}
```

**Variations / call-sites:**
- **LaTeX config** (120 lines): 44 bracket pairs, % lineComment, AutoClosingPairs, surroundingPairs with $ delimiters
- **Markdown+LaTeX config** (126 lines): HTML comments (`<!-- -->`), 44 bracket pairs, HTML folding markers
- **C++ embedded config** (33 lines): C-style comments, 3 bracket pairs, pragma region folding

**Key aspect:** Bracket configurations scale from 3 pairs (C++) to 44 pairs (LaTeX). LaTeX-specific matching handles LaTeX scaling delimiters at multiple sizes (`\bigl`, `\Bigl`, `\biggl`, `\Biggl`), modeling mathematical typesetting conventions.

---

## Pattern 3: Auto-Closing and Surrounding Pairs with Scope Restrictions

**Where:** `extensions/latex/latex-cpp-embedded-language-configuration.json:11-24`

**What:** Advanced auto-closing pair definitions with `notIn` scope restrictions to prevent closure in comments/strings. Contrasts with simple array-based syntax used in pure LaTeX config.

```json
{
  "autoClosingPairs": [
    { "open": "[", "close": "]" },
    { "open": "{", "close": "}" },
    { "open": "(", "close": ")" },
    { "open": "'", "close": "'", "notIn": ["string", "comment"] },
    { "open": "\"", "close": "\"", "notIn": ["string"] }
  ],
  "surroundingPairs": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
    ["\"", "\""],
    ["'", "'"],
    ["<", ">"]
  ]
}
```

**Variations / call-sites:**
- **C++ embedded (33 LOC):** Uses object notation with `notIn` field
- **Pure LaTeX (120 LOC):** Uses simple array notation without scope restrictions
- **Markdown+LaTeX (126 LOC):** Uses array notation; also defines `wordPattern` with Unicode properties

---

## Pattern 4: Indentation Rules with Regex Lookahead/Lookbehind

**Where:** `extensions/latex/latex-language-configuration.json:105-108`

**What:** Regex-based indentation control using positive/negative lookahead patterns. Increases indent on `\begin{}` (except `document`), decreases on `\end{}` (except `document`). Uses named capture groups and negative lookahead to exclude specific environments.

```json
{
  "indentationRules": {
    "increaseIndentPattern": "\\\\begin{(?!document)([^}]*)}(?!.*\\\\end{\\1})",
    "decreaseIndentPattern": "^\\s*\\\\end{(?!document)"
  }
}
```

**Variations / call-sites:**
- **LaTeX config** (line 105-108): Increase on `\begin{}`, decrease on `\end{}`
- **Markdown+LaTeX config** (line 110-113): Identical indentation rules
- **C++ embedded config:** No indentation rules

**Key aspect:** Both LaTeX patterns use negative lookahead `(?!document)` to prevent indentation changes in document environment, preserving global structure.

---

## Pattern 5: Folding Region Markers with Optional Comments

**Where:** `extensions/latex/latex-language-configuration.json:109-114`

**What:** Fold region definitions supporting both explicit region markers and LaTeX-native markers. Allows `%? region` (optional comment char) to match `region` or `% region`.

```json
{
  "folding": {
    "markers": {
      "start": "^\\s*%?\\s*(region|\\\\begingroup)\\b",
      "end": "^\\s*%?\\s*(endregion|\\\\endgroup)\\b"
    }
  }
}
```

**Variations / call-sites:**
- **LaTeX config** (line 109-114): `%? region` / `% endregion`, LaTeX `\begingroup` / `\endgroup`
- **Markdown+LaTeX config** (line 114-120): HTML comment markers `<!-- #?region -->` / `<!-- #?endregion -->`, plus `offSide: true`
- **C++ embedded config** (line 27-31): `#pragma region` / `#pragma endregion` (pragma markers)

---

## Pattern 6: Word Pattern with Unicode Property Support

**Where:** `extensions/latex/latex-language-configuration.json:116-119`

**What:** Unicode-aware word boundary definition using `\p{}` Unicode property escapes. Matches alphabetic/numeric characters plus nonspacing marks, supporting non-ASCII identifiers and mathematical notation.

```json
{
  "wordPattern": {
    "pattern": "(\\p{Alphabetic}|\\p{Number}|\\p{Nonspacing_Mark}){1,}",
    "flags": "u"
  }
}
```

**Variations / call-sites:**
- **LaTeX config** (line 116-119): `\p{Alphabetic}|Number|Nonspacing_Mark` with `"u"` flag
- **Markdown+LaTeX config** (line 122-125): Wraps with optional bold/italic markers `([*_]{1,2})?` at start and end
- **C++ embedded config** (line 26): Regex without Unicode flags: `(-?\\d*\\.\\d\\w*)|([^...]+)`

---

## Pattern 7: Extension Manifest with Language Contributions

**Where:** `extensions/latex/package.json:15-115`

**What:** VS Code extension manifest declaring 5 language definitions (tex, latex, bibtex, cpp_embedded_latex, markdown_latex_combined) with grammar path bindings and embedded language mappings. LaTeX grammar maps 10 embedded languages (cpp, css, html, java, js, julia, lua, python, ruby, typescript, xml, yaml).

```json
{
  "contributes": {
    "languages": [
      {
        "id": "tex",
        "aliases": ["TeX", "tex"],
        "extensions": [".sty", ".cls", ".bbx", ".cbx"],
        "configuration": "latex-language-configuration.json"
      },
      {
        "id": "latex",
        "aliases": ["LaTeX", "latex"],
        "extensions": [".tex", ".ltx", ".ctx"],
        "configuration": "latex-language-configuration.json"
      },
      ...
    ],
    "grammars": [
      {
        "language": "latex",
        "scopeName": "text.tex.latex",
        "path": "./syntaxes/LaTeX.tmLanguage.json",
        "unbalancedBracketScopes": [
          "keyword.control.ifnextchar.tex",
          "punctuation.math.operator.tex"
        ],
        "embeddedLanguages": {
          "source.cpp": "cpp_embedded_latex",
          "source.css": "css",
          "source.js": "javascript",
          ...
        }
      }
    ]
  }
}
```

**Variations / call-sites:**
- **tex** language: `.sty`, `.cls`, `.bbx`, `.cbx` extensions, `text.tex` scope
- **latex** language: `.tex`, `.ltx`, `.ctx` extensions, 10 embedded languages
- **bibtex** language: `.bib` extension, `text.bibtex` scope
- Both LaTeX definitions use `unbalancedBracketScopes` for lookahead/lookbehind contexts

---

## Summary for Tauri/Rust Porting

The LaTeX extension demonstrates **declarative language configuration patterns** suitable for refactoring into Rust:

1. **Grammar composition via includes** — TextMate pattern repositories can be modeled as Rust enums/structs with recursive pattern definitions
2. **Bracket matching tables** — Simple JSON arrays map directly to Rust data structures (e.g., `Vec<(String, String)>`)
3. **Regex-based rules** — Lookahead/lookbehind patterns in indentation and folding can be compiled into static `regex` crate patterns
4. **Embedded language mappings** — Scope name to language ID mappings are static and can be pre-compiled into a Rust map
5. **Unicode-aware tokenization** — The wordPattern using `\p{Alphabetic}` should use Rust's `unicode-segmentation` or similar crates
6. **Configuration schema** — All JSON structures have stable schemas suitable for serde deserialization

**Key porting consideration:** The LaTeX extension is purely declarative (no procedural code). Porting requires:
- A TextMate grammar parser (possibly reusing existing Rust crates like `tree-sitter` or `oniguruma`)
- Configuration deserialization (serde)
- Unicode-aware regex matching (regex + unicode crates)
- No runtime behavior changes needed

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
