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

