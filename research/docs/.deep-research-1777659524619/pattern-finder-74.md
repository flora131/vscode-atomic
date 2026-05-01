# Pattern Analysis: extensions/latex/ (13 LOC)

## Summary

The LaTeX grammar extension (`extensions/latex/`) is a **grammar-only extension** that provides syntax highlighting and language configuration for TeX, LaTeX, and BibTeX files. This partition contains no portable IDE logic—only declarative grammar definitions and build scripts.

## Sentinel Analysis

**Result:** SKIP - Grammar extension only, no core IDE functionality.

---

## Extension Structure

### Language Declarations
**File:** `/home/norinlavaee/projects/vscode-atomic/extensions/latex/package.json` (lines 16-64)

The extension registers five language configurations:

```json
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
  {
    "id": "bibtex",
    "aliases": ["BibTeX", "bibtex"],
    "extensions": [".bib"]
  },
  {
    "id": "cpp_embedded_latex",
    "configuration": "latex-cpp-embedded-language-configuration.json",
    "aliases": []
  },
  {
    "id": "markdown_latex_combined",
    "configuration": "markdown-latex-combined-language-configuration.json",
    "aliases": []
  }
]
```

### Grammar Contributions
**File:** `/home/norinlavaee/projects/vscode-atomic/extensions/latex/package.json` (lines 65-114)

Grammar definitions include:
- **TextMate Grammar paths** for LaTeX, TeX, BibTeX, and hybrid formats
- **Embedded language mappings** declaring which languages can be embedded within LaTeX (cpp, css, html, java, js, julia, lua, python, ruby, ts, xml, yaml)
- **Unbalanced bracket scopes** to handle LaTeX-specific bracket mismatches

Example embedded language configuration:
```json
"embeddedLanguages": {
  "source.cpp": "cpp_embedded_latex",
  "source.css": "css",
  "text.html": "html",
  "source.java": "java",
  "source.js": "javascript",
  "source.julia": "julia",
  "source.lua": "lua",
  "source.python": "python",
  "source.ruby": "ruby",
  "source.ts": "typescript",
  "text.xml": "xml",
  "source.yaml": "yaml"
}
```

### Language Configuration
**File:** `/home/norinlavaee/projects/vscode-atomic/extensions/latex/latex-language-configuration.json` (120 lines)

The configuration provides:

1. **Comment Pattern:** `"%"` (line comment marker)

2. **Bracket Pairs (40+ definitions, lines 5-70):**
   - Standard: `{...}`, `[...]`, `(...)`
   - LaTeX delimiters: `\left(...\right)`, `\bigl(...\bigr)`, etc.
   - Angle brackets: `\langle...\rangle`
   - Fence variants: `\Bigl`, `\Biggl`, etc.

3. **Auto-Closing Pairs (lines 72-95):**
   - 17 pairs including `\left...\right` constructs, standard brackets, and quote pairs

4. **Surrounding Pairs (lines 96-104):**
   - Quotation marks, brackets, dollar signs for math mode: `$...$`

5. **Indentation Rules (lines 105-108):**
   ```
   "increaseIndentPattern": "\\\\begin{(?!document)([^}]*)}(?!.*\\\\end{\\1})"
   "decreaseIndentPattern": "^\\s*\\\\end{(?!document)"
   ```

6. **Code Folding (lines 109-113):**
   - Region markers: `%region` / `%endregion`
   - LaTeX group markers: `\begingroup` / `\endgroup`

7. **Word Pattern (lines 116-119):**
   - Unicode-aware regex using `\p{Alphabetic}`, `\p{Number}`, `\p{Nonspacing_Mark}`

---

## Build Process

**File:** `/home/norinlavaee/projects/vscode-atomic/extensions/latex/build/update-grammars.js` (13 lines)

```javascript
var updateGrammar = require('vscode-grammar-updater');

updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/Bibtex.tmLanguage.json', 'syntaxes/Bibtex.tmLanguage.json', undefined, 'main');
updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/LaTeX.tmLanguage.json', 'syntaxes/LaTeX.tmLanguage.json', undefined, 'main');
updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/TeX.tmLanguage.json', 'syntaxes/TeX.tmLanguage.json', undefined, 'main');
updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/cpp-grammar-bailout.tmLanguage.json', 'syntaxes/cpp-grammar-bailout.tmLanguage.json', undefined, 'main');
updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/markdown-latex-combined.tmLanguage.json', 'syntaxes/markdown-latex-combined.tmLanguage.json', undefined, 'main');
```

Uses `vscode-grammar-updater` to pull TextMate grammar definitions from the external repository `jlelong/vscode-latex-basics` on the `main` branch.

---

## Porting Assessment

### Declarative vs. Imperative

All content is **declarative configuration:**
- Language and file extension mappings
- Grammar rules (TextMate `.tmLanguage.json` format)
- Bracket/pair matching rules
- Folding markers

### What Cannot Port Directly

1. **TextMate Grammars** — These are scope-based syntax rules tied to VS Code's TextMate engine. Tauri/Rust would require an alternative syntax highlighting backend (e.g., Tree-sitter, custom Rust parser).

2. **Grammar Updater Script** — Pulls from external repository; not core logic.

3. **Embedded Language Mappings** — Depends on VS Code's embedded language system; would require equivalent implementation in Tauri.

### What Could Port

1. **Bracket/Delimiter Definitions** — The 40+ bracket pair definitions and auto-closing pairs are pure data structures. These could be ported as Rust configuration data.

2. **Indentation Rules** — The regex patterns for increase/decrease indentation are extractable and could be adapted to Rust regex syntax.

3. **Folding Markers** — Region and group markers are simple regex patterns.

4. **Word Pattern** — Unicode property escapes would need translation to Rust's regex crate syntax.

---

## Conclusion

**This extension contains no core IDE functionality to port.** It is a pure grammar extension providing syntax support for LaTeX documents. All logic is declarative configuration handled by VS Code's built-in grammar system. Porting would require:

1. A replacement syntax highlighting system (e.g., Tree-sitter)
2. Data migration of bracket pairs and indentation rules
3. Equivalent embedded language mechanism in the Tauri implementation

No executable business logic, event handlers, or state management patterns are present in this partition.
