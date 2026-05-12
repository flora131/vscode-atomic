# Partition 75 of 80 — Findings

## Scope
`extensions/latex/` (1 files, 13 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# LaTeX Extension Grammar Porting Analysis

## Implementation

### Grammar Files
- `extensions/latex/syntaxes/TeX.tmLanguage.json` — Core TeX syntax definitions (376 lines)
- `extensions/latex/syntaxes/LaTeX.tmLanguage.json` — Extended LaTeX syntax definitions (4446 lines)
- `extensions/latex/syntaxes/Bibtex.tmLanguage.json` — BibTeX syntax definitions (341 lines)
- `extensions/latex/syntaxes/markdown-latex-combined.tmLanguage.json` — Combined Markdown/LaTeX syntax (3276 lines)
- `extensions/latex/syntaxes/cpp-grammar-bailout.tmLanguage.json` — C++ embedded in LaTeX (20054 lines)

### Grammar Builder
- `extensions/latex/build/update-grammars.js` — Automated grammar update script that pulls from upstream jlelong/vscode-latex-basics repository

### Language Configurations
- `extensions/latex/latex-language-configuration.json` — TeX/LaTeX language settings (120 lines)
- `extensions/latex/latex-cpp-embedded-language-configuration.json` — C++ embedded language configuration (33 lines)
- `extensions/latex/markdown-latex-combined-language-configuration.json` — Markdown+LaTeX combined configuration (126 lines)

## Configuration

- `extensions/latex/package.json` — Extension manifest defining language contributions, grammars, and scripts (120 lines)
- `extensions/latex/.vscodeignore` — Files to exclude from package
- `extensions/latex/cgmanifest.json` — Component governance manifest (37 lines)

## Documentation

- `extensions/latex/markdown-latex-combined-license.txt` — License for combined markdown grammar
- `extensions/latex/cpp-bailout-license.txt` — License for C++ bailout grammar
- `extensions/latex/package.nls.json` — Localization strings (4 lines)

## Notable Clusters

The LaTeX extension is a grammar-only extension with no TypeScript/JavaScript runtime code. It provides TextMate grammar definitions for five language variants via the VSCode grammar contribution system. The core contribution mechanism is declarative configuration in `package.json` that references TextMate `.tmLanguage.json` files. An automated build script (`update-grammars.js`) maintains synchronization with the upstream jlelong/vscode-latex-basics repository.

Porting this to Tauri/Rust would require translating TextMate format grammars into Rust-based syntax highlighting (e.g., tree-sitter or comparable Rust grammar framework), replacing the declarative VSCode grammar registration with equivalent Rust plugin API bindings, and maintaining the grammar update pipeline in a Rust context.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
## Analysis: extensions/latex — LaTeX/TeX/BibTeX Grammar Extension

### Overview
The `extensions/latex` partition is a purely declarative VS Code extension. It contains zero runtime TypeScript — only static JSON grammar files (TextMate `.tmLanguage.json` format), JSON language-configuration files, a one-time build script that fetches upstream grammars, and the VS Code `package.json` manifest. The extension's sole function is to register five languages and their grammars with the VS Code language server infrastructure.

### Entry Points
- `/home/norinlavaee/projects/vscode-atomic/extensions/latex/package.json:15` — `contributes.languages` array: registers `tex`, `latex`, `bibtex`, `cpp_embedded_latex`, `markdown_latex_combined`
- `/home/norinlavaee/projects/vscode-atomic/extensions/latex/package.json:65` — `contributes.grammars` array: wires each language ID to its `scopeName` and `.tmLanguage.json` file path

### Core Implementation

#### 1. Language Registration (`package.json:16–63`)
Five language IDs are declared. Each entry specifies:
- `id`: the canonical language identifier used elsewhere in VS Code
- `aliases`: display names (e.g. `["LaTeX", "latex"]` at line 35–38)
- `extensions`: file-extension triggers (`.tex`, `.ltx`, `.ctx` for `latex` at line 39–42; `.sty`, `.cls`, `.bbx`, `.cbx` for `tex` at line 23–27; `.bib` for `bibtex` at line 49–51)
- `configuration`: path to the matching language-configuration JSON (omitted for `bibtex`)

Two languages — `cpp_embedded_latex` (line 55) and `markdown_latex_combined` (line 60) — have empty `aliases` arrays and no file-extension triggers. They are auxiliary virtual languages used only for embedded-language coloring.

#### 2. Grammar Registration (`package.json:65–114`)
Five grammars are registered:
- `text.tex` → `./syntaxes/TeX.tmLanguage.json` (376 lines)
- `text.tex.latex` → `./syntaxes/LaTeX.tmLanguage.json` (4 446 lines)
- `text.bibtex` → `./syntaxes/Bibtex.tmLanguage.json` (341 lines)
- `text.tex.markdown_latex_combined` → `./syntaxes/markdown-latex-combined.tmLanguage.json` (3 276 lines)
- `source.cpp.embedded.latex` → `./syntaxes/cpp-grammar-bailout.tmLanguage.json` (20 054 lines — not read per task scope)

The `latex` grammar entry (`package.json:76–98`) carries an `embeddedLanguages` map that binds 13 inner TextMate scope names (e.g. `source.cpp`, `source.python`, `text.html`) to VS Code language IDs. This is how code blocks inside LaTeX source files get syntax-colored as their own language.

Two scopes are declared `unbalancedBracketScopes` for both `tex` and `latex` (`package.json:70–73`, `79–82`): `keyword.control.ifnextchar.tex` and `punctuation.math.operator.tex`. VS Code's bracket-matching engine skips those scopes.

#### 3. TeX Grammar Structure (`syntaxes/TeX.tmLanguage.json:1–376`)
- `scopeName`: `text.tex` (line 9)
- Top-level `patterns` array (lines 10–39) includes eight named rules in order: `#iffalse-block`, `#macro-control`, `#catcode`, `#comment`, a literal bracket match at line 24, `#dollar-math`, a `\\\\` newline match at line 31, `#ifnextchar`, `#macro-general`
- All rule bodies live in the `repository` object starting at line 41, referenced via `#name` includes
- Key repository rules: `catcode` (line 42) uses a single regex capturing `\catcode` with numeric category code; `iffalse-block` (line 60) uses `begin`/`end` for multi-line block; `dollar-math` handles inline math delimited by `$`

#### 4. LaTeX Grammar Structure (`syntaxes/LaTeX.tmLanguage.json:1–80 sampled`)
- `scopeName`: `text.tex.latex` (line 9)
- Top-level `patterns` (lines 10–78) include 18 named rule references plus a final `{"include": "text.tex"}` fallthrough at line 77 — meaning TeX grammar is embedded inside LaTeX grammar as its base layer
- Rule hierarchy: `songs-env`, `embedded-code-env`, `verbatim-env`, `document-env`, `all-balanced-env`, then macro-specific rules (`documentclass-usepackage-macro`, `input-macro`, `sections-macro`, `hyperref-macro`, `newcommand-macro`, `text-font-macro`, `citation-macro`, `references-macro`, `label-macro`, `verb-macro`, `inline-code-macro`, `all-other-macro`), then math modes (`display-math`, `inline-math`), `column-specials`, and finally the TeX base
- Notable pattern: `meta.space-after-command.latex` (line 12) is a lookbehind-triggered zero-width match used to signal command-completion triggers

#### 5. BibTeX Grammar Structure (`syntaxes/Bibtex.tmLanguage.json:1–80 sampled`)
- `scopeName`: `text.bibtex` (line 9)
- Top-level patterns: `@comment` block match (line 13), `#preamble`, `#string`, `#entry`, and a fallback `[^@\n]`…`(?=@)` comment block (line 31)
- Repository has `preamble`, `string`, `entry`, and `field_value` rules; `preamble` supports both brace `{` and paren `(` delimiters (lines 39–64 and 65+)

#### 6. Language Configuration Files

**`latex-language-configuration.json`** (shared by `tex` and `latex`, line 29 and 43 of package.json):
- `comments.lineComment`: `%` (line 3)
- `brackets`: 71 bracket pairs (lines 5–70) covering standard `{}[]()` and all LaTeX-specific `\left(`/`\right)` and size-variant forms (`\bigl`, `\Bigl`, `\biggl`, `\Biggl`)
- `autoClosingPairs` (lines 72–95): 22 pairs including LaTeX size-variant delimiter pairs plus `\(…\)`, `\[…\]`, `\{…\}`, and the typographic `` ` ``/`'` quote pair
- `surroundingPairs` (lines 96–104): includes `$…$` for math wrapping
- `indentationRules` (lines 105–108): `increaseIndentPattern` triggers on `\begin{X}` (excluding `document`); `decreaseIndentPattern` on `\end{`
- `folding.markers` (lines 110–113): start = `% region` or `\begingroup`; end = `% endregion` or `\endgroup`
- `wordPattern` (lines 116–119): Unicode-aware regex using `\p{Alphabetic}`, `\p{Number}`, `\p{Nonspacing_Mark}` with `u` flag

**`latex-cpp-embedded-language-configuration.json`** (for `cpp_embedded_latex`):
- Uses C++ comment conventions (`//` line, `/* */` block), standard ASCII brackets only, and `#pragma region`/`#pragma endregion` folding markers

**`markdown-latex-combined-language-configuration.json`** (for `markdown_latex_combined`):
- Inherits LaTeX bracket pairs (same 71 entries) and LaTeX `autoClosingPairs`
- `comments.blockComment`: `<!--` / `-->` (HTML/Markdown style)
- `surroundingPairs` adds `` ` ``, `_`, `*` for Markdown emphasis
- `wordPattern` wraps with optional leading `*_` or `**_` for Markdown decoration
- `folding`: `offSide: true` with HTML comment region markers

#### 7. Build Script (`build/update-grammars.js:1–14`)
- Requires the `vscode-grammar-updater` npm package (not in this partition)
- Calls `updateGrammar.update(owner, sourcePath, destPath, undefined, 'main')` five times (lines 9–13), one per grammar file
- Source repository: `jlelong/vscode-latex-basics` on GitHub (confirmed by `cgmanifest.json:7`)
- `.vscodeignore` (lines 1–2) excludes `cgmanifest.json` and the entire `build/` directory from the packaged extension

#### 8. Provenance (`cgmanifest.json:1–37`)
- Upstream: `https://github.com/jlelong/vscode-latex-basics` at commit `76dc409348227db00f6779772f7763dc90cdf22e`, version `1.16.0`
- `LaTeX.tmLanguage.json` traces back to the textmate `latex.tmbundle`
- `markdown-latex-combined.tmLanguage.json` is generated from VS Code's own Markdown grammar
- `cpp-grammar-bailout.tmLanguage.json` is generated from `jeff-hykin/better-cpp-syntax`

### Data Flow
There is no runtime data flow. The extension is loaded once by VS Code at startup:
1. `package.json` contributions are read by VS Code's extension host
2. Language IDs are registered with file-extension associations
3. Grammar paths are registered; VS Code's TextMate tokenizer reads the `.tmLanguage.json` files on demand when a matching file is opened
4. Language configuration JSON is read by VS Code's language service for bracket matching, auto-close, indentation, folding, and word detection

### Key Patterns
- **TextMate Grammar Include Chain**: `LaTeX.tmLanguage.json` bottom-includes `text.tex` (TeX grammar), making TeX rules available in all LaTeX files
- **Virtual Embedded Language IDs**: `cpp_embedded_latex` and `markdown_latex_combined` exist solely as targets for `embeddedLanguages` mappings; they have no file extensions of their own
- **Upstream Sync Script**: `build/update-grammars.js` is a fetch-only script with no transformation logic — it pulls verbatim from the upstream repo

### Configuration
- `package.json:8–10`: Engine constraint `"vscode": "*"` (any version)
- `package.json:12`: Grammar update script invoked via `npm run update-grammar`
- Grammar upstream pinned at commit `76dc409348227db00f6779772f7763dc90cdf22e` (`cgmanifest.json:9`, `TeX.tmLanguage.json:7`)

### Out-of-Partition References
- `vscode-grammar-updater` npm package — imported by `build/update-grammars.js:7`; not present in this partition
- VS Code extension host infrastructure that reads `contributes.languages` and `contributes.grammars` — outside this partition
- TextMate tokenization engine inside VS Code core — consumes the `.tmLanguage.json` files at runtime
- `jlelong/vscode-latex-basics` GitHub repository — upstream source for all five grammar files
- `jeff-hykin/better-cpp-syntax` — upstream source for `cpp-grammar-bailout.tmLanguage.json` (via vscode-latex-basics)
- VS Code Markdown grammar — partial source for `markdown-latex-combined.tmLanguage.json`
- `textmate/latex.tmbundle` — original basis for `LaTeX.tmLanguage.json`
- CSS, HTML, Java, JavaScript, Julia, Lua, Python, Ruby, TypeScript, XML, YAML VS Code language extensions — referenced by `embeddedLanguages` map in `package.json:83–97`

### Synthesis
The `extensions/latex` partition is entirely a data-declaration artifact with no executable TypeScript or Rust. Its operational surface is five TextMate JSON grammar files and three language-configuration JSON files consumed passively by VS Code's existing tokenization and language-feature engines. The build script (`build/update-grammars.js`) is a developer-time fetch tool only, excluded from the packaged extension by `.vscodeignore`. The `cpp-grammar-bailout.tmLanguage.json` file (20 054 lines) exists to provide a simplified C++ grammar for use inside LaTeX-embedded C++ blocks, generated from an external repository and pulled verbatim. For a Tauri/Rust port, this entire partition reduces to shipping the same JSON files and configuring whatever TextMate-compatible tokenizer is used in the new host to read them — there is no TypeScript logic to translate.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: VS Code LaTeX Extension Port to Tauri/Rust

## Scope
`extensions/latex/` — Grammar and language configuration only (1 LOC file with build script)

## Sentinel Note
This extension partition contains **only grammar/language metadata and build configuration files**. No TypeScript implementation code is present in this scope to analyze for porting patterns.

## Files Examined

### Build Configuration
**Found in**: `extensions/latex/build/update-grammars.js:1-14`

```javascript
var updateGrammar = require('vscode-grammar-updater');

updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/Bibtex.tmLanguage.json', 'syntaxes/Bibtex.tmLanguage.json', undefined, 'main');
updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/LaTeX.tmLanguage.json', 'syntaxes/LaTeX.tmLanguage.json', undefined, 'main');
updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/TeX.tmLanguage.json', 'syntaxes/TeX.tmLanguage.json', undefined, 'main');
updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/cpp-grammar-bailout.tmLanguage.json', 'syntaxes/cpp-grammar-bailout.tmLanguage.json', undefined, 'main');
updateGrammar.update('jlelong/vscode-latex-basics', 'syntaxes/markdown-latex-combined.tmLanguage.json', 'syntaxes/markdown-latex-combined.tmLanguage.json', undefined, 'main');
```

This is a build script that pulls grammar definitions from an external repository (`jlelong/vscode-latex-basics`).

### Extension Manifest
**Found in**: `extensions/latex/package.json:1-121`

The manifest declares language contributions and grammar scopes:

- **Languages declared**: `tex`, `latex`, `bibtex`, `cpp_embedded_latex`, `markdown_latex_combined`
- **Grammar entries**: 5 grammar definitions with scope names and embedded language support
- **Configuration references**: Language configuration files defining bracket matching, auto-closing pairs, indentation rules, and folding markers

### Language Configuration
**Found in**: `extensions/latex/latex-language-configuration.json:1-120`

This JSON file defines IDE language support features:
- **Comments**: Line comment syntax (`%`)
- **Bracket pairs**: 70+ bracket configurations including LaTeX-specific constructs (`\left(`, `\right)`, `\bigl[`, etc.)
- **Auto-closing pairs**: 26 pairs for bracket completion
- **Surrounding pairs**: Quotes, braces, and math delimiters
- **Indentation rules**: Regex patterns for `\begin{}`/`\end{}` blocks
- **Folding markers**: Region directives and `\begingroup`/`\endgroup`
- **Word pattern**: Unicode-aware regex for identifier matching

## Key Observations for Porting

The LaTeX extension demonstrates how VS Code manages **TextMate grammar** and **language configuration** entirely through:

1. **Declarative metadata** (package.json, JSON configuration files)
2. **External grammar sources** (pulled from GitHub via build script)
3. **Scope-based syntax highlighting** (TextMate scope names)
4. **Structured language features** (bracket matching, indentation, folding)

For a Tauri/Rust port, these patterns would translate to:
- **Grammar system**: Grammar definitions would remain JSON or convert to a Rust-native format (tree-sitter, Ropey, or custom)
- **Language configuration**: Could be embedded as Rust structs or remain as JSON deserialized at startup
- **Build process**: Node.js grammar-updater would be replaced with Rust tooling or direct API integration
- **Manifest system**: Extension metadata would map to Rust trait implementations or TOML configuration

## Files Analyzed
- `/home/norinlavaee/projects/vscode-atomic/extensions/latex/build/update-grammars.js`
- `/home/norinlavaee/projects/vscode-atomic/extensions/latex/package.json`
- `/home/norinlavaee/projects/vscode-atomic/extensions/latex/latex-language-configuration.json`

The LaTeX extension in this partition is a **pure declarative grammar/language extension** with no runtime TypeScript logic, serving as a template for how syntactic and semantic language features can be separated from implementation. A Tauri port would benefit from a similar separation, where language metadata is decoupled from the core editor runtime.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
