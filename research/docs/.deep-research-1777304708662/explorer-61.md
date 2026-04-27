# Partition 61 of 79 — Findings

## Scope
`extensions/html/` (1 files, 61 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator Results for Partition 61 (extensions/html/)

## Research Question
What it would take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

## Scope
`extensions/html/` — 9 files total, 61 LOC

---

## Implementation

- `extensions/html/package.json` — Extension manifest declaring HTML language support with grammar definitions and snippet contributions
- `extensions/html/language-configuration.json` — Language behavior configuration for HTML (bracket pairs, auto-closing, folding, indentation patterns, on-enter rules)
- `extensions/html/build/update-grammar.mjs` — Build script for synchronizing TextMate HTML grammar from upstream repository with VS Code-specific patches
- `extensions/html/syntaxes/html.tmLanguage.json` — TextMate grammar definition for HTML syntax highlighting with embedded language support (CSS, JavaScript, Python)
- `extensions/html/syntaxes/html-derivative.tmLanguage.json` — Variant TextMate grammar for HTML derivatives with modified tag matching rules

## Configuration

- `extensions/html/.vscodeignore` — Build artifact exclusion list
- `extensions/html/cgmanifest.json` — Component governance manifest registering external TextMate/html.tmbundle dependency
- `extensions/html/package.nls.json` — Localization strings for display name and description

## Examples / Fixtures

- `extensions/html/snippets/html.code-snippets` — Code snippet definitions for HTML language

---

## Summary

The `extensions/html/` directory contains a minimal language extension for HTML support in VS Code. It comprises grammar definitions (TextMate format), language behavior configuration, a build pipeline for grammar synchronization, and snippet templates. This extension demonstrates the declarative extension model VS Code uses to add language support: a `package.json` manifest registers language contributions, TextMate grammars handle syntax highlighting, and language configuration controls editor behaviors.

From a Tauri/Rust porting perspective, this extension reveals how VS Code abstracts language intelligence through external grammar formats and configuration files. Porting would require: (1) parsing and interpreting TextMate grammars in Rust, (2) reimplementing language configuration behaviors (bracket matching, folding, indentation rules) in the editor core, and (3) establishing a plugin system that can load and manage language extensions declaratively. This is a lower-level concern than the core IDE features (debugging, source control, terminal) but is essential for the editing experience.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

1. `/Users/norinlavaee/vscode-atomic/extensions/html/package.json`
2. `/Users/norinlavaee/vscode-atomic/extensions/html/language-configuration.json`
3. `/Users/norinlavaee/vscode-atomic/extensions/html/build/update-grammar.mjs`
4. `/Users/norinlavaee/vscode-atomic/extensions/html/syntaxes/html.tmLanguage.json`
5. `/Users/norinlavaee/vscode-atomic/extensions/html/syntaxes/html-derivative.tmLanguage.json`
6. `/Users/norinlavaee/vscode-atomic/extensions/html/snippets/html.code-snippets`
7. `/Users/norinlavaee/vscode-atomic/extensions/html/cgmanifest.json`
8. `/Users/norinlavaee/vscode-atomic/extensions/html/package.nls.json`

---

### Per-File Notes

#### 1. `extensions/html/package.json`

**Role:** Extension manifest declaring all contributions the HTML built-in extension makes to the VS Code host.

**Key symbols and data:**

- `contributes.languages[0]` (lines 16–48): Registers the `html` language ID. Maps 13 file extensions (`.html`, `.htm`, `.shtml`, `.xhtml`, `.xht`, `.mdoc`, `.jsp`, `.asp`, `.aspx`, `.jshtm`, `.volt`, `.ejs`, `.rhtml`) and 5 MIME types (`text/html`, `text/x-jshtm`, `text/template`, `text/ng-template`, `application/xhtml+xml`) to the language ID. Points to `./language-configuration.json` for editor behavior.
- `contributes.grammars[0]` (lines 51–63): Registers `text.html.basic` (scopeName) backed by `./syntaxes/html.tmLanguage.json`. Declares five embedded language scope mappings: `text.html` → `html`, `source.css` → `css`, `source.js` → `javascript`, `source.python` → `python`, `source.smarty` → `smarty`. Sets `tokenTypes` so that strings inside `meta.tag` are classified as `other` rather than `string`, enabling correct semantic behavior.
- `contributes.grammars[1]` (lines 65–79): Registers `text.html.derivative` backed by `./syntaxes/html-derivative.tmLanguage.json` with the same embedded language map. This grammar is the default active grammar for the `html` language ID (it has a `"language": "html"` field while the basic grammar does not).
- `contributes.snippets[0]` (lines 81–85): Attaches `./snippets/html.code-snippets` to the `html` language.
- `scripts.update-grammar` (line 12): Entry point for the grammar refresh workflow, invoking `./build/update-grammar.mjs`.

**Dependencies:** No runtime `dependencies` field; the extension is purely declarative except for the build script, which depends on `vscode-grammar-updater` (external npm package, used only at build time).

**Porting relevance:** The entire contribution mechanism (`contributes.*`) is VS Code's extension API contract. A Tauri/Rust IDE would need an equivalent capability discovery system — either a compatible JSON manifest schema or a reimplemented registration layer — to consume this file's declarations for language IDs, grammar paths, and embedded language maps.

---

#### 2. `extensions/html/language-configuration.json`

**Role:** Configures editor-level language behaviors for the `html` language ID. Consumed at runtime by VS Code's built-in language configuration service (not through an extension activation path — VS Code reads it directly from the `configuration` pointer in `package.json:47`).

**Key behavioral rules:**

- `comments.blockComment` (line 3): Defines `<!-- -->` as the block comment delimiter pair, consumed by the Toggle Block Comment command.
- `brackets` (lines 5–9): Three bracket pairs registered for bracket matching: `<!-- -->`, `{ }`, `( )`.
- `autoClosingPairs` (lines 10–17): Six auto-close rules. The `<!-- --> ` pair (line 16) includes a `"notIn": ["comment", "string"]` guard so it does not fire inside comments or strings.
- `surroundingPairs` (lines 18–25): Six pairs used for the "surround with" selection feature. Includes `< >` (line 24), which enables wrapping selected text in angle brackets.
- `colorizedBracketPairs` (line 26–27): Empty array — bracket pair colorization is explicitly disabled for HTML.
- `folding.markers` (lines 29–32): Region folding uses `<!-- #region ... -->` / `<!-- #endregion ... -->` comment annotations (lines 30–31).
- `wordPattern` (line 34): Custom regex covering CSS numeric values (`-?\\d*\\.\\d\\w*`) plus a broad character-class exclusion for standard word boundaries.
- `onEnterRules` (lines 35–48): Two rules using `beforeText`/`afterText` patterns. Rule 1 (lines 36–42): When the cursor is after an opening non-void tag and before a matching closing tag, pressing Enter triggers `indentOutdent` (new line indented inside, closing tag pushed down). Rule 2 (lines 43–47): When after an opening non-void tag without a matching closing tag, pressing Enter triggers `indent`. Both rules share a void-element exclusion regex in `beforeText` covering `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `keygen`, `link`, `menuitem`, `meta`, `param`, `source`, `track`, `wbr`.
- `indentationRules` (lines 50–53): Two regexes for increase and decrease indent patterns, also excluding void elements and `html` tag from the decrease rule.

**Porting relevance:** This file encodes all of VS Code's `ILanguageConfiguration` interface fields. A Tauri port would need a compatible language configuration loader that can interpret this JSON schema and apply its rules to the editor's text model and input handlers.

---

#### 3. `extensions/html/build/update-grammar.mjs`

**Role:** Build-time Node.js script (ESM) that fetches, patches, and writes the two HTML TextMate grammar files from the upstream `textmate/html.tmbundle` repository. Not shipped in the extension package (listed in `.vscodeignore:1` as `build/**`).

**Control flow:**

1. Imports `vscode-grammar-updater` (line 7), an external utility that handles fetching from GitHub and converting plist to JSON.
2. Defines `patchGrammar(grammar)` (lines 9–36):
   - Performs a recursive depth-first traversal of `grammar.repository` (lines 11–30) via a `visit` function.
   - When a rule's `name` is `source.js` or `source.css` and the rule's parent is in `endCaptures` but the first sibling in the parent node is NOT `punctuation.definition.string.end.html` (line 14), it renames the scope to `source.js-ignored-vscode` or `source.css-ignored-vscode` (line 15). This prevents double-tokenization of end-capture scopes.
   - Asserts exactly 2 patches were made (line 31).
3. Defines `patchGrammarDerivative(grammar)` (lines 38–53):
   - Iterates `grammar.patterns` (line 42) and finds the `meta.tag.other.unrecognized.html.derivative` rule whose `begin` pattern is `(</?)(\\w[^\\s>]*)(?<!/)` (line 43).
   - Replaces the `begin` regex with `(</?)(\\w[^\\s<>]*)(?<!/)` (line 44) — adds `<` to the exclusion character class inside the tag name group, preventing runaway matches into nested tags.
   - Asserts exactly 1 patch (line 49).
4. Calls `vscodeGrammarUpdater.update` twice (lines 57, 60):
   - For `html.tmLanguage.json`: source `textmate/html.tmbundle`, path `Syntaxes/HTML.plist`, output `./syntaxes/html.tmLanguage.json`, using `patchGrammar` as the transform callback.
   - For `html-derivative.tmLanguage.json`: source `textmate/html.tmbundle`, path `Syntaxes/HTML%20%28Derivative%29.tmLanguage`, output `./syntaxes/html-derivative.tmLanguage.json`, using `patchGrammarDerivative`.

**Porting relevance:** The patch logic is VS Code-specific and must be preserved whenever the grammar is re-exported. In a Tauri IDE, if TextMate grammars are still used (e.g., via a Rust-side TextMate engine), this build script and its patches would need to be maintained unchanged or re-applied to the grammar source.

---

#### 4. `extensions/html/syntaxes/html.tmLanguage.json`

**Role:** The authoritative TextMate grammar for HTML (`scopeName: "text.html.basic"`). Consumed by VS Code's Monarch/TextMate tokenization engine to produce syntax tokens for editor highlighting. Upstream source is `textmate/html.tmbundle` at commit `0c3d5ee54de3a993f747f54186b73a4d2d3c44a2` (`html.tmLanguage.json:7`).

**Top-level structure:**

- `injections` (lines 10–19): A global injection rule with selector `R:text.html - (comment.block, text.html meta.embedded, meta.tag.*.*.html, ...)` that marks bare `<` characters as `invalid.illegal.bad-angle-bracket.html`. The `R:` prefix ensures this injection fires after all other injections, acting as a final cleanup pass.
- `patterns` (lines 21–43): Seven top-level includes in order: `#xml-processing`, `#comment`, `#doctype`, `#cdata`, `#tags-valid`, `#tags-invalid`, `#entities`. These define the parse entry points.
- `repository` (line 44+): Named rule bank. Key rules observed in the first 300 lines include:
  - `attribute` (lines 45+): Handles HTML5 standard attributes with a large alternation regex (line 48) for all known attribute names, plus a separate entry for the `style` attribute (line 64) with CSS embedding, plus a separate entry for `on*` event handlers (line 156) with JavaScript embedding.
  - CSS embedding within `style` attribute: content between quotes is assigned `contentName: "source.css"` (line 104), causing VS Code to invoke the CSS grammar for tokenization of that content span.
  - JavaScript embedding within event handlers: content between quotes uses `contentName: "source.js"` (line 203), invoking the JavaScript grammar. Embedded JS comment handling (`//`, `/* */`) is explicitly handled inside these string rules (lines 228–255).

**Embedded language mechanism:** The `embeddedLanguages` map in `package.json:54–60` maps grammar scope names (e.g., `source.css`) to VS Code language IDs (e.g., `css`). When the tokenizer encounters a span with `contentName: "source.css"`, VS Code routes language server features (completions, hover, diagnostics) for that span to the CSS language server.

**Porting relevance:** The TextMate grammar format (JSON representation of `.tmLanguage` PList grammar) is a widely portable format. A Tauri IDE would need a Rust-side TextMate grammar engine (e.g., `syntect` crate) to consume this file. The `embeddedLanguages` metadata is VS Code-specific and would need a parallel concept in the new system to enable multi-language features within HTML files.

---

#### 5. `extensions/html/syntaxes/html-derivative.tmLanguage.json`

**Role:** A second, simpler TextMate grammar (`scopeName: "text.html.derivative"`) designed for HTML-based template languages. This is the grammar actually assigned to the `html` language ID (via `package.json:67` `"language": "html"`). It extends `text.html.basic` by composition.

**Key structure:**

- `injections` (lines 10–19): Identical `R:text.html` injection as the basic grammar for invalid `<` detection.
- `patterns` (lines 21–47): Two entries:
  1. `{ "include": "text.html.basic#core-minus-invalid" }` (line 24): Delegates to a named rule export of the basic grammar (not the full grammar), selectively excluding the invalid-tag patterns. This `core-minus-invalid` rule must be defined as an exported entry point in `html.tmLanguage.json`.
  2. A standalone rule (lines 25–47) for `meta.tag.other.unrecognized.html.derivative`: matches `(</?)(\\w[^\\s<>]*)(?<!/)` as the tag begin (line 26 — this is the patched version of the regex from `update-grammar.mjs:44`). Applies `text.html.basic#attribute` for attribute parsing inside unrecognized tags (line 46).

**Derivative grammar intent:** By including `text.html.basic#core-minus-invalid` first and then adding a catch-all unrecognized-tag rule, this grammar supports template languages (Vue, Angular, Svelte, etc.) that introduce custom HTML-like elements not in the standard HTML5 element set, without marking them as invalid.

**Porting relevance:** The `include: "text.html.basic#core-minus-invalid"` cross-grammar reference is a TextMate grammar composition feature. A Rust TextMate engine would need to support cross-grammar `include` references (with fragment identifiers) for this to work correctly.

---

#### 6. `extensions/html/snippets/html.code-snippets`

**Role:** Defines one snippet for the `html` language. Consumed by VS Code's snippet service, referenced from `package.json:84`.

**Snippet definition (lines 2–17):**

- Key: `"html doc"` — the trigger name displayed in completion UI.
- `"isFileTemplate": true` (line 3): Marks this as a file-template snippet, appearing in the "New File" template picker in addition to the normal completion list.
- `"body"` (lines 4–15): Array of lines forming the full HTML5 document scaffold: `<!DOCTYPE html>`, `<html>`, `<head>` with UTF-8 `<meta charset>` and a tabstop `${1:title}`, `<body>` with final tabstop `$0`.
- `"description": "HTML Document"` (line 16): Display label.

**Porting relevance:** The `isFileTemplate` flag and body array format are VS Code-specific snippet schema fields. A Tauri IDE's snippet service would need to implement this schema, including tabstop (`$0`, `${1:...}`) substitution logic and the `isFileTemplate` concept.

---

#### 7. `extensions/html/cgmanifest.json`

**Role:** Component governance manifest declaring the external open-source dependency consumed in this extension's build process. Consumed by Microsoft's internal supply-chain tooling, not by VS Code at runtime.

**Key data (lines 1–31):**

- Registers `textmate/html.tmbundle` (line 7) at commit hash `0c3d5ee54de3a993f747f54186b73a4d2d3c44a2` (line 9) from `https://github.com/textmate/html.tmbundle` (line 8).
- License (lines 11–28): Free-use permissive license from the TextMate bundle authors.

**Porting relevance:** Documents the precise upstream commit the current grammars are derived from. Any Tauri port consuming these grammar files would need to track the same upstream dependency.

---

#### 8. `extensions/html/package.nls.json`

**Role:** Localization string table for the extension manifest. Keys match `%displayName%` and `%description%` placeholders in `package.json:3–4`.

**Values (lines 2–4):**
- `"displayName": "HTML Language Basics"` — Shown in VS Code's Extensions panel.
- `"description": "Provides syntax highlighting, bracket matching & snippets in HTML files."` — Confirms the three feature areas this extension provides: syntax highlighting (via grammars), bracket matching (via language-configuration), and snippets.

---

### Cross-Cutting Synthesis

The `extensions/html` partition is a purely declarative built-in extension: it contributes no executable TypeScript activation code. Its entire runtime behavior depends on VS Code's host services reading and interpreting three artifact types — a language configuration JSON, two TextMate grammar JSONs, and a snippets JSON — all wired together through the `package.json` `contributes` manifest. The extension boundary is the VS Code extension API contract (`contributes.languages`, `contributes.grammars`, `contributes.snippets`). The embedded language feature (CSS inside `style` attributes, JS inside `on*` attributes, Python inside `<script type="text/python">` blocks) works by combining TextMate `contentName` scope assignments in the grammar with the `embeddedLanguages` map in the manifest, delegating tokenization and language services to other registered extensions/servers at runtime. The grammar is not authored in VS Code; it is fetched from the upstream `textmate/html.tmbundle` repository at build time via `update-grammar.mjs` and modified by two VS Code-specific patches before being committed.

For a Tauri/Rust port, the dependencies introduced by this partition are: (1) a TextMate grammar engine in Rust capable of cross-grammar `include` references and injection selectors (e.g., the `syntect` crate); (2) a language configuration loader implementing the `ILanguageConfiguration` schema for editor behaviors like auto-closing, on-enter rules, and indentation rules; (3) a snippet engine supporting VS Code's snippet body format including `isFileTemplate`; and (4) an extension manifest reader that can interpret the `contributes.grammars.embeddedLanguages` map to route language-server requests for embedded content spans to the correct language server.

---

### Out-of-Partition References

The following symbols, services, and files are referenced by this partition's files but reside outside `extensions/html/`:

- **`vscode-grammar-updater`** (npm package, used in `build/update-grammar.mjs:7`): External build utility for fetching and converting TextMate grammars from GitHub. Source at `https://github.com/microsoft/vscode-grammar-updater`.
- **`text.html.basic#core-minus-invalid`** (referenced in `syntaxes/html-derivative.tmLanguage.json:24`): A named repository export that must be declared inside `syntaxes/html.tmLanguage.json`. The string `core-minus-invalid` does not appear in the first 300 lines read; it is present deeper in that file's `repository` section.
- **VS Code Language Configuration Service**: The runtime consumer of `language-configuration.json`, located in VS Code core (likely `src/vs/editor/common/languages/languageConfigurationRegistry.ts` or similar). Not in this partition.
- **VS Code TextMate Tokenization Engine**: The runtime consumer of `syntaxes/html.tmLanguage.json` and `syntaxes/html-derivative.tmLanguage.json`. Located in `src/vs/editor/` (TextMate integration, possibly via `vscode-textmate` package). Not in this partition.
- **VS Code Snippet Service**: The runtime consumer of `snippets/html.code-snippets`. Located in VS Code core snippet subsystem. Not in this partition.
- **CSS extension grammar** (`source.css` scope): Cross-grammar reference embedded in `html.tmLanguage.json` event-handler and style-attribute rules. Defined in `extensions/css/`.
- **JavaScript extension grammar** (`source.js` scope): Cross-grammar reference embedded in `html.tmLanguage.json` event-handler rules. Defined in `extensions/javascript/` or `extensions/typescript-basics/`.
- **Python extension grammar** (`source.python` scope): Cross-grammar reference for `<script type="text/python">` blocks (declared in `package.json:58`). Defined in `extensions/python/`.
- **Smarty extension grammar** (`source.smarty` scope): Cross-grammar reference declared in `package.json:59`. Likely a third-party extension.
- **`textmate/html.tmbundle`** upstream repo (GitHub): Original grammar source at commit `0c3d5ee54de3a993f747f54186b73a4d2d3c44a2` (`cgmanifest.json:9`).

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Analysis: HTML Language Extension (extensions/html/)

## Scope Overview
The `extensions/html/` directory contains a lightweight HTML language support extension for VS Code, consisting of grammar definitions, language configuration, snippets, and build tooling. Total: 9 files, ~2,740 LOC (mostly grammar JSON).

---

## Patterns Found

#### Pattern 1: Language Registration via Declarative Manifest
**Where:** `extensions/html/package.json:15-49`
**What:** Language declaration in package.json contributes block that registers HTML language with file extensions, MIME types, and configuration reference.
```json
"languages": [
  {
    "id": "html",
    "extensions": [
      ".html",
      ".htm",
      ".shtml",
      ".xhtml",
      ".xht",
      ".mdoc",
      ".jsp",
      ".asp",
      ".aspx",
      ".jshtm",
      ".volt",
      ".ejs",
      ".rhtml"
    ],
    "aliases": ["HTML", "htm", "html", "xhtml"],
    "mimetypes": [
      "text/html",
      "text/x-jshtm",
      "text/template",
      "text/ng-template",
      "application/xhtml+xml"
    ],
    "configuration": "./language-configuration.json"
  }
]
```
**Variations / call-sites:** Same pattern used in core language extensions (css, javascript, etc.). Aliases and MIME types map multiple input forms to a single language ID.

---

#### Pattern 2: TextMate Grammar with Embedded Language Support
**Where:** `extensions/html/package.json:50-79`
**What:** Dual grammar registration where the main grammar includes embedded language injections for CSS, JavaScript, and other languages, with tokenType overrides for special HTML context handling.
```json
"grammars": [
  {
    "scopeName": "text.html.basic",
    "path": "./syntaxes/html.tmLanguage.json",
    "embeddedLanguages": {
      "text.html": "html",
      "source.css": "css",
      "source.js": "javascript",
      "source.python": "python",
      "source.smarty": "smarty"
    },
    "tokenTypes": {
      "meta.tag string.quoted": "other"
    }
  }
]
```
**Variations / call-sites:** The derivative grammar (text.html.derivative) uses the same pattern but references core HTML rules via `#core-minus-invalid` includes, enabling grammar composition/inheritance.

---

#### Pattern 3: Structural Grammar Patching via Node Visitor Pattern
**Where:** `extensions/html/build/update-grammar.mjs:9-36`
**What:** Recursive tree visitor that traverses TextMate grammar repository to patch specific rules (e.g., disabling JavaScript/CSS capture in certain contexts by renaming to `-ignored-vscode` variant).
```javascript
function patchGrammar(grammar) {
  let patchCount = 0;

  let visit = function (rule, parent) {
    if (rule.name === 'source.js' || rule.name === 'source.css') {
      if (parent.node[0].name !== 'punctuation.definition.string.end.html' && 
          parent.parent && parent.parent.property === 'endCaptures') {
        rule.name = rule.name + '-ignored-vscode';
        patchCount++;
      }
    }
    for (let property in rule) {
      let value = rule[property];
      if (typeof value === 'object') {
        visit(value, { node: rule, property: property, parent: parent });
      }
    }
  };

  let repository = grammar.repository;
  for (let key in repository) {
    visit(repository[key], { node: repository, property: key, parent: undefined });
  }
  if (patchCount !== 2) {
    console.warn(`Expected to patch 2 occurrences: Was ${patchCount}`);
  }
  return grammar;
}
```
**Variations / call-sites:** Similar regex-based patching in `patchGrammarDerivative()` for fixing angle-bracket patterns in unrecognized tags (line 38-53).

---

#### Pattern 4: Declarative Language Configuration Rules
**Where:** `extensions/html/language-configuration.json:1-54`
**What:** JSON configuration defining bracket/delimiter pairs, auto-closing behavior, bracket colorization, folding markers, word patterns, and context-aware indentation rules via regex.
```json
{
  "comments": {
    "blockComment": ["<!--", "-->"]
  },
  "brackets": [
    ["<!--", "-->"],
    ["{", "}"],
    ["(", ")"]
  ],
  "autoClosingPairs": [
    {"open": "{", "close": "}"},
    {"open": "[", "close": "]"},
    {"open": "(", "close": ")"},
    {"open": "'", "close": "'"},
    {"open": "\"", "close": "\""},
    {"open": "<!--", "close": "-->", "notIn": ["comment", "string"]}
  ],
  "onEnterRules": [
    {
      "beforeText": {
        "pattern": "<(?!(?:area|base|br|col|embed|hr|img|input|keygen|link|menuitem|meta|param|source|track|wbr))([_:\\w][_:\\w-.\\d]*)(?:(?:[^'\"/>]|\"[^\"]*\"|'[^']*')*?(?!\\/)>)[^<]*$",
        "flags": "i"
      },
      "afterText": {
        "pattern": "^<\\/([_:\\w][_:\\w-.\\d]*)\\s*>",
        "flags": "i"
      },
      "action": {"indent": "indentOutdent"}
    }
  ],
  "indentationRules": {
    "increaseIndentPattern": "<(?!\\?|(?:area|base|br|col|frame|hr|html|img|input|keygen|link|menuitem|meta|param|source|track|wbr)\\b|[^>]*\\/>)([-_\\.A-Za-z0-9]+)(?=\\s|>)\\b[^>]*>(?!.*<\\/\\1>)|<!--(?!.*-->)|\\{[^}\"']*$",
    "decreaseIndentPattern": "^\\s*(<\\/(?!html)[-_\\.A-Za-z0-9]+\\b[^>]*>|-->|\\})"
  }
}
```
**Variations / call-sites:** Used in all language extensions. The negative lookahead patterns handle self-closing tags (area, base, br, etc.) to prevent incorrect indentation.

---

#### Pattern 5: Code Snippet Template Registration
**Where:** `extensions/html/snippets/html.code-snippets:1-18`
**What:** Snippet manifest defining file template with placeholder variables (${1:title}, $0) for guided user input on document creation.
```json
{
  "html doc": {
    "isFileTemplate": true,
    "body": [
      "<!DOCTYPE html>",
      "<html>",
      "<head>",
      "\t<meta charset=\"UTF-8\" />",
      "\t<title>${1:title}</title>",
      "</head>",
      "<body>",
      "\t$0",
      "</body>",
      "</html>"
    ],
    "description": "HTML Document"
  }
}
```
**Variations / call-sites:** Same pattern in other language extensions (cpp, python, etc.). Tabstops ($1, $2) enable cursor jumps; $0 marks final cursor position.

---

#### Pattern 6: Grammar Composition via Reference Inclusion
**Where:** `extensions/html/syntaxes/html-derivative.tmLanguage.json:23-24`
**What:** Derivative grammar reuses core HTML rules by including `text.html.basic#core-minus-invalid`, then adds custom rules for unrecognized tags, demonstrating grammar inheritance without duplication.
```json
"patterns": [
  {
    "include": "text.html.basic#core-minus-invalid"
  },
  {
    "begin": "(</?)(\\w[^\\s<>]*)(?<!/)",
    "beginCaptures": {
      "1": {"name": "punctuation.definition.tag.begin.html"},
      "2": {"name": "entity.name.tag.html"}
    },
    "end": "((?: ?/)?>)",
    "endCaptures": {
      "1": {"name": "punctuation.definition.tag.end.html"}
    },
    "name": "meta.tag.other.unrecognized.html.derivative",
    "patterns": [
      {"include": "text.html.basic#attribute"}
    ]
  }
]
```
**Variations / call-sites:** Extended includes for CSS/JS embedded languages in main grammar (lines 54-79); full grammar repository shows recursive includes throughout.

---

#### Pattern 7: Grammar Update Build Tool with External Source Sync
**Where:** `extensions/html/build/update-grammar.mjs:55-60`
**What:** Build script that fetches and converts TextMate bundles from external GitHub repositories, applies version-tracked patches, and outputs converted JSON syntax files.
```javascript
const tsGrammarRepo = 'textmate/html.tmbundle';
const grammarPath = 'Syntaxes/HTML.plist';
vscodeGrammarUpdater.update(tsGrammarRepo, grammarPath, './syntaxes/html.tmLanguage.json', grammar => patchGrammar(grammar));

const grammarDerivativePath = 'Syntaxes/HTML%20%28Derivative%29.tmLanguage';
vscodeGrammarUpdater.update(tsGrammarRepo, grammarDerivativePath, './syntaxes/html-derivative.tmLanguage.json', grammar => patchGrammarDerivative(grammar));
```
**Variations / call-sites:** Same pattern in css, javascript, and xml extensions. Version URIs (line 7, 8) track exact commit hashes from upstream for reproducibility.

---

## Summary

The `extensions/html/` extension demonstrates six core patterns for porting IDE language support to Tauri/Rust:

1. **Declarative Language Registration** — Extension manifests define language metadata (file types, MIME types, aliases) decoupled from runtime behavior.

2. **TextMate Grammar as Data** — Syntax highlighting is entirely defined in JSON grammar files with embedded language support, patch-tolerant for VS Code customizations.

3. **Stateless Grammar Transformation** — A build-time visitor pattern patches grammar trees without modifying source, enabling clean separation of upstream grammars from customizations.

4. **Declarative Language Behavior** — Indentation, bracket matching, and folding rules are defined as regex-based configurations in JSON, not imperative code.

5. **Composable Grammars** — Derivative grammars inherit from core grammars via `#rule` includes, reducing duplication and enabling language variants (e.g., HTML in templates).

6. **External Grammar Sync** — A build tool fetches upstream TextMate bundles, converts them, applies patches, and tracks source versions, separating community grammar maintenance from editor code.

These patterns suggest a Tauri port could use:
- TOML/JSON for language metadata instead of TypeScript package.json contributions
- Treesitter grammars (Rust-based) or embedded TextMate grammars instead of VS Code's grammar loading
- A separate grammar patch/compilation layer (potentially Rust-based) instead of JavaScript update tools
- Declarative config files for language behavior (bracket pairs, indentation) reusable across platforms

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
