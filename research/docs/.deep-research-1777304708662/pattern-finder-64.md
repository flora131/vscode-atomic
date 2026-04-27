# Pattern Analysis: Razor Extension (extensions/razor/)

## Research Question
What patterns exist for porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust?

## Scope Analysis
The Razor extension directory contains 1 meaningful file with 44 LOC across configuration and build scripts. This extension demonstrates VS Code's plugin architecture for language support, specifically for ASP.NET Razor/C# HTML templating.

---

## Patterns Found

#### Pattern: Extension Manifest with Language Contribution
**Where:** `extensions/razor/package.json:1-50`
**What:** Declarative configuration that registers a language with file extensions, aliases, MIME types, and references to language configuration and grammar files.
```json
{
  "name": "razor",
  "engines": { "vscode": "0.10.x" },
  "contributes": {
    "languages": [{
      "id": "razor",
      "extensions": [".cshtml", ".razor"],
      "aliases": ["Razor", "razor"],
      "mimetypes": ["text/x-cshtml"],
      "configuration": "./language-configuration.json"
    }],
    "grammars": [{
      "language": "razor",
      "scopeName": "text.html.cshtml",
      "path": "./syntaxes/cshtml.tmLanguage.json",
      "embeddedLanguages": {
        "section.embedded.source.cshtml": "csharp",
        "source.css": "css",
        "source.js": "javascript"
      }
    }]
  }
}
```
**Variations:** Embedded languages enable syntax highlighting for mixed-language constructs (C#, CSS, JavaScript within Razor templates).

---

#### Pattern: Language Configuration with Editor Behaviors
**Where:** `extensions/razor/language-configuration.json:1-22`
**What:** JSON schema defining comment syntax, bracket pairs, auto-closing pairs, and surrounding pairs to enable editor features like smart bracket matching and auto-completion.
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
    { "open": "{", "close": "}"},
    { "open": "[", "close": "]"},
    { "open": "(", "close": ")" },
    { "open": "'", "close": "'" },
    { "open": "\"", "close": "\"" }
  ],
  "surroundingPairs": [
    { "open": "'", "close": "'" },
    { "open": "\"", "close": "\"" },
    { "open": "<", "close": ">" }
  ]
}
```
**Variations:** Supports both HTML comment syntax and C# braces; distinct configuration for auto-closing (insert pairs) versus surrounding (wrap selection).

---

#### Pattern: Build Script for Grammar Synchronization
**Where:** `extensions/razor/build/update-grammar.mjs:7-42`
**What:** Node script that pulls grammar definitions from upstream dotnet/razor repository and applies localized patches to integrate with VS Code's TextMate grammar system.
```javascript
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
    console.warn(`Expected to patch 4 occurrences: Was ${patchCount}`);
  }
  return grammar;
}

const razorGrammarRepo = 'dotnet/razor';
const grammarPath = 'src/Razor/src/Microsoft.VisualStudio.RazorExtension/EmbeddedGrammars/aspnetcorerazor.tmLanguage.json';
vscodeGrammarUpdater.update(razorGrammarRepo, grammarPath, './syntaxes/cshtml.tmLanguage.json', grammar => patchGrammar(grammar), 'main');
```
**Variations:** Uses recursive visitor pattern for AST traversal; validates patch application via counter assertions.

---

#### Pattern: TextMate Grammar with Embedded Language Injections
**Where:** `extensions/razor/syntaxes/cshtml.tmLanguage.json:1-100`
**What:** TextMate grammar definition that defines syntax rules for Razor-specific constructs (comments, code blocks, expressions, directives) and injects C#, CSS, and JavaScript grammars into appropriate contexts.
```json
{
  "name": "ASP.NET Razor",
  "scopeName": "text.html.cshtml",
  "injections": {
    "string.quoted.double.html": {
      "patterns": [
        { "include": "#explicit-razor-expression" },
        { "include": "#implicit-expression" }
      ]
    },
    "source.cs": {
      "patterns": [
        { "include": "#inline-template" }
      ]
    }
  },
  "patterns": [
    { "include": "#razor-control-structures" },
    { "include": "text.html.derivative" }
  ],
  "repository": {
    "razor-control-structures": {
      "patterns": [
        { "include": "#razor-comment" },
        { "include": "#razor-codeblock" },
        { "include": "#explicit-razor-expression" },
        { "include": "#escaped-transition" },
        { "include": "#directives" },
        { "include": "#transitioned-csharp-control-structures" },
        { "include": "#implicit-expression" }
      ]
    }
  }
}
```
**Variations:** Distinguishes between optionally-transitioned and fully-transitioned C# control structures; uses scope names to target where injections apply.

---

#### Pattern: Package Script for Build Automation
**Where:** `extensions/razor/package.json:11-12`
**What:** NPM script registration that enables convenient grammar updates via `npm run update-grammar`, abstracting build tool complexity.
```json
"scripts": {
  "update-grammar": "node ./build/update-grammar.mjs"
}
```
**Variations:** Minimal wrapper pattern; often used for incremental downstream repository synchronization.

---

#### Pattern: Repository Metadata with External Reference
**Where:** `extensions/razor/package.json:46-49`
**What:** Package metadata linking the extension back to the upstream monorepo and providing version tracking via GitHub commit references.
```json
"repository": {
  "type": "git",
  "url": "https://github.com/microsoft/vscode.git"
}
```
**Variations:** Enables maintainers to trace extension provenance; supports tooling for automated backport detection.

---

## Cross-Pattern Observations

The Razor extension demonstrates a three-tier architecture pattern for language support in VS Code:

1. **Manifest Layer** (package.json): Declarative registration of language metadata, file associations, and paths to configuration and grammar assets.

2. **Configuration Layer** (language-configuration.json): Behavioral rules for editor features like bracket matching, auto-closing, and comment detection—declarative metadata with no code.

3. **Grammar Layer** (TextMate `.tmLanguage.json`): Scope-based syntax rules with support for embedded language injection, enabling polyglot syntax highlighting in mixed-language constructs.

4. **Synchronization Layer** (build/update-grammar.mjs): Build tooling that keeps local grammar definitions synchronized with upstream repositories while applying localized patches for VS Code's TextMate derivative system.

This architecture separates concerns: the manifest declares intent, configuration defines local editor behavior, grammars define syntax rules, and build scripts ensure long-term maintainability against upstream changes.

**For Tauri/Rust porting:** This extension exemplifies how language-specific IDE features can be implemented as modular plugins with minimal runtime coupling. The declarative nature of manifest and configuration files suggests they could be ported as structured configuration without code changes. The TextMate grammar system is a platform-specific implementation detail; a Rust-based IDE would need a native equivalent (e.g., tree-sitter grammars or a custom tokenizer). The build script pattern demonstrates the importance of synchronization tooling for managed external dependencies.
