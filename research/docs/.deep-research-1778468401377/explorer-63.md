# Partition 63 of 80 — Findings

## Scope
`extensions/razor/` (1 files, 44 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator 63: VS Code Tauri/Rust Port - Razor Extension Analysis

## Scope
`extensions/razor/` — Grammar/snippets extension for Razor language support (44 LOC total)

## Implementation

### Grammar and Language Configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/razor/syntaxes/cshtml.tmLanguage.json` — TextMate grammar definition for Razor (.cshtml, .razor) files with embedded language support for C#, CSS, and JavaScript
- `/home/norinlavaee/projects/vscode-atomic/extensions/razor/language-configuration.json` — Language configuration defining comment syntax, bracket matching, auto-closing pairs, and surrounding pairs for Razor files
- `/home/norinlavaee/projects/vscode-atomic/extensions/razor/build/update-grammar.mjs` — Build script that patches and updates the Razor grammar from the dotnet/roslyn repository, specifically transforming text.html.basic includes to text.html.derivative

### Package Manifest
- `/home/norinlavaee/projects/vscode-atomic/extensions/razor/package.json` — Extension manifest declaring Razor language support with language IDs, file extensions (.cshtml, .razor), aliases, and grammar contribution points; depends on vscode >= 0.10.x
- `/home/norinlavaee/projects/vscode-atomic/extensions/razor/package.nls.json` — Localization strings for display name and description

## Configuration

- `/home/norinlavaee/projects/vscode-atomic/extensions/razor/cgmanifest.json` — Component governance manifest tracking the grammar source as derived from dotnet/roslyn (commit 79e211a2e4287f9f7508089b81311b2c7fdc169f) under MIT license
- `/home/norinlavaee/projects/vscode-atomic/extensions/razor/.vscodeignore` — Packaging exclusions for test/, cgmanifest.json, and build/ directories

## Relevance to Tauri/Rust Port

The Razor extension represents a **language support plugin** within VS Code's extensibility model. For a Tauri/Rust port, this scope reveals:

1. **TextMate Grammar Dependencies**: The extension relies on TextMate grammar files (.tmLanguage.json) for syntax highlighting. A Rust-based editor would need either a compatible grammar engine (e.g., tree-sitter as an alternative) or a grammar parsing library to handle these declarative syntax definitions.

2. **Language Contribution Architecture**: The extension uses VS Code's declarative contribution system (package.json) to register languages, grammars, and configurations. This demonstrates VS Code's plugin-based IDE structure; a Tauri port must replicate this extensibility surface for language support registration.

3. **External Grammar Source Management**: The build process fetches and patches grammars from external repositories (dotnet/roslyn), showing that VS Code abstracts grammar updates from the core IDE. A Rust port would need to integrate or wrap grammar management tooling.

4. **Embedded Language Nesting**: The grammar configuration demonstrates support for embedded languages (C#, CSS, JavaScript within Razor markup), requiring a grammar engine capable of language composition—a non-trivial feature for any text editor port.

This extension is a thin plugin layer with no TypeScript/Node.js runtime dependencies, making it a good candidate for early porting as a simple grammar registration system for the Rust-based editor. The core challenge lies not in the extension itself, but in building the underlying grammar and language registration infrastructure in Rust.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
