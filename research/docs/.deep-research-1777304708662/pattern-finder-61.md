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
