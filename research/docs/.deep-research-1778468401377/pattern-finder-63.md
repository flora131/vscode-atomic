# Pattern Finder Research: Razor Extension (Partition 63)

## Scope Analysis
The `extensions/razor/` directory contains a language grammar extension for Razor/CSHTML syntax highlighting. Total LOC: ~44 (package.json, language-configuration.json, package.nls.json).

## Assessment
**Orientation: Skip** - This partition contains only grammar and configuration files with no executable code patterns relevant to porting VS Code core IDE functionality from TypeScript/Electron to Tauri/Rust.

## Findings

### Pattern: Grammar Extension Configuration
**Where:** `extensions/razor/package.json:15-44`
**What:** VS Code extension contribution structure for language grammars and language configuration
```json
"contributes": {
  "languages": [
    {
      "id": "razor",
      "extensions": [".cshtml", ".razor"],
      "aliases": ["Razor", "razor"],
      "mimetypes": ["text/x-cshtml"],
      "configuration": "./language-configuration.json"
    }
  ],
  "grammars": [
    {
      "language": "razor",
      "scopeName": "text.html.cshtml",
      "path": "./syntaxes/cshtml.tmLanguage.json",
      "embeddedLanguages": {
        "section.embedded.source.cshtml": "csharp",
        "source.css": "css",
        "source.js": "javascript"
      }
    }
  ]
}
```

### Pattern: Language Configuration for Bracket Matching and Auto-Completion
**Where:** `extensions/razor/language-configuration.json:1-22`
**What:** Declarative language configuration for editor behavior (bracket matching, comment syntax, auto-closing pairs)
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
    {"open": "\"", "close": "\""}
  ],
  "surroundingPairs": [
    {"open": "'", "close": "'"},
    {"open": "\"", "close": "\""},
    {"open": "<", "close": ">"}
  ]
}
```

### Pattern: Grammar Update Build Script with External Synchronization
**Where:** `extensions/razor/build/update-grammar.mjs:1-42`
**What:** Build script that synchronizes grammar definitions from upstream repository (dotnet/roslyn) and applies VS Code-specific patches
```javascript
import * as vscodeGrammarUpdater from 'vscode-grammar-updater';

function patchGrammar(grammar) {
  grammar.scopeName = 'text.html.cshtml';
  let patchCount = 0;
  let visit = function (rule, parent) {
    if (rule.include?.startsWith('text.html.basic')) {
      patchCount++;
      rule.include = 'text.html.derivative';
    }
    for (let property in rule) {
      let value = rule[property];
      if (typeof value === 'object') {
        visit(value, { node: rule, property: property, parent: parent });
      }
    }
  };
  let roots = [grammar.repository, grammar.patterns];
  for (let root of roots) {
    for (let key in root) {
      visit(root[key], { node: root, property: key, parent: undefined });
    }
  }
  if (patchCount !== 4) {
    console.warn(`Expected to patch 4 occurrences of text.html.basic: Was ${patchCount}`);
  }
  return grammar;
}

const razorGrammarRepo = 'dotnet/roslyn';
const grammarPath = 'src/Razor/src/Razor/src/Microsoft.VisualStudio.RazorExtension/EmbeddedGrammars/aspnetcorerazor.tmLanguage.json';
vscodeGrammarUpdater.update(razorGrammarRepo, grammarPath, './syntaxes/cshtml.tmLanguage.json', grammar => patchGrammar(grammar), 'main');
```

### Pattern: TextMate Grammar with Embedded Language Support
**Where:** `extensions/razor/syntaxes/cshtml.tmLanguage.json:1-46` (2031 LOC total)
**What:** TextMate language grammar with injection patterns for embedded C#, CSS, and JavaScript languages
```json
{
  "information_for_contributors": [
    "This file has been converted from https://github.com/dotnet/roslyn/blob/master/src/Razor/src/Razor/src/Microsoft.VisualStudio.RazorExtension/EmbeddedGrammars/aspnetcorerazor.tmLanguage.json",
    "If you want to provide a fix or improvement, please create a pull request against the original repository.",
    "Once accepted there, we are happy to receive an update request."
  ],
  "version": "https://github.com/dotnet/roslyn/commit/79e211a2e4287f9f7508089b81311b2c7fdc169f",
  "name": "ASP.NET Razor",
  "scopeName": "text.html.cshtml",
  "injections": {
    "string.quoted.double.html": {
      "patterns": [
        {"include": "#explicit-razor-expression"},
        {"include": "#implicit-expression"}
      ]
    },
    "source.cs": {
      "patterns": [
        {"include": "#inline-template"}
      ]
    }
  },
  "patterns": [
    {"include": "#razor-control-structures"},
    {"include": "text.html.derivative"}
  ]
}
```

## Conclusion

The Razor extension is a declarative grammar extension with no portable executable code patterns. It demonstrates how VS Code structures language extensions through package.json contributions, but this is specific to the VS Code extension API and not core IDE functionality. The extension uses TextMate grammar format (XML-like JSON) for syntax highlighting, which is a well-established standard that would require re-implementation in a Tauri/Rust system using a different syntax highlighting engine (e.g., tree-sitter or similar Rust-based solution).

**No core IDE patterns relevant to Tauri/Rust porting are present in this scope.**

