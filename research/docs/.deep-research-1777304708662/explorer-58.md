# Partition 58 of 79 — Findings

## Scope
`extensions/typescript-basics/` (1 files, 95 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locations: extensions/typescript-basics/

## Implementation

- `extensions/typescript-basics/syntaxes/TypeScript.tmLanguage.json` - TextMate grammar for TypeScript syntax highlighting (derived from TypeScript-TmLanguage repository)
- `extensions/typescript-basics/syntaxes/TypeScriptReact.tmLanguage.json` - TextMate grammar for TypeScript JSX/React syntax highlighting
- `extensions/typescript-basics/syntaxes/jsdoc.ts.injection.tmLanguage.json` - JSDoc documentation syntax injection for TypeScript
- `extensions/typescript-basics/syntaxes/jsdoc.js.injection.tmLanguage.json` - JSDoc documentation syntax injection for JavaScript
- `extensions/typescript-basics/language-configuration.json` - Language configuration defining brackets, auto-closing pairs, folding rules, indentation rules, and editor behaviors for TypeScript files
- `extensions/typescript-basics/snippets/typescript.code-snippets` - Code snippets for TypeScript and TypeScript React languages

## Configuration

- `extensions/typescript-basics/package.json` - Extension manifest with language and grammar contributions, semantic token scopes, and build scripts
- `extensions/typescript-basics/package.nls.json` - Localization strings for display name and description
- `extensions/typescript-basics/.vscodeignore` - Files to exclude from packaging
- `extensions/typescript-basics/cgmanifest.json` - Component governance manifest for tracking dependencies

## Build & Maintenance

- `extensions/typescript-basics/build/update-grammars.mjs` - Build script for updating grammar files from TypeScript-TmLanguage repository

## Documentation

- `extensions/typescript-basics/syntaxes/Readme.md` - Documentation on grammar source, update procedures, and migration notes for scope naming improvements

## Summary

The `typescript-basics` extension is a grammar and snippet-only language contribution for VS Code. It provides TextMate grammars for TypeScript syntax highlighting (both standard and JSX variants), language configuration for editor behaviors (bracket matching, auto-closing, indentation), and code snippets. The extension has no executable code or language server integration—it focuses entirely on static syntax highlighting and editor features. The grammars are derived from and maintained synchronously with the external TypeScript-TmLanguage repository via the update-grammars build script.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/Users/norinlavaee/vscode-atomic/extensions/typescript-basics/package.json` (203 lines)
- `/Users/norinlavaee/vscode-atomic/extensions/typescript-basics/language-configuration.json` (271 lines)
- `/Users/norinlavaee/vscode-atomic/extensions/typescript-basics/build/update-grammars.mjs` (96 lines)
- `/Users/norinlavaee/vscode-atomic/extensions/typescript-basics/snippets/typescript.code-snippets` (320 lines)
- `/Users/norinlavaee/vscode-atomic/extensions/typescript-basics/syntaxes/TypeScript.tmLanguage.json` (5751 lines)
- `/Users/norinlavaee/vscode-atomic/extensions/typescript-basics/syntaxes/TypeScriptReact.tmLanguage.json` (6000 lines)
- `/Users/norinlavaee/vscode-atomic/extensions/typescript-basics/syntaxes/jsdoc.ts.injection.tmLanguage.json` (22 lines)
- `/Users/norinlavaee/vscode-atomic/extensions/typescript-basics/syntaxes/jsdoc.js.injection.tmLanguage.json` (22 lines)
- `/Users/norinlavaee/vscode-atomic/extensions/typescript-basics/cgmanifest.json` (18 lines)
- `/Users/norinlavaee/vscode-atomic/extensions/typescript-basics/package.nls.json` (4 lines)
- `/Users/norinlavaee/vscode-atomic/extensions/typescript-basics/syntaxes/Readme.md` (18 lines)
- `/Users/norinlavaee/vscode-atomic/extensions/typescript-basics/.vscodeignore` (8 lines)

---

### Per-File Notes

#### `package.json`

**Role**: The VS Code extension manifest. Declares all static contributions for the TypeScript and TSX language modes. No runtime TypeScript code is present; every contribution is a purely declarative JSON structure.

**Language registrations** (`package.json:17-63`):
- `"typescript"` language id (`package.json:19`), mapped to extensions `.ts`, `.cts`, `.mts` (`package.json:26-29`), with a `firstLine` shebang pattern matching `deno`, `bun`, or `ts-node` runtimes (`package.json:30`). Both `typescript` and `typescriptreact` reference the same `./language-configuration.json` (`package.json:31`, `43`).
- `"typescriptreact"` language id (`package.json:34`), mapped to `.tsx` extension.
- `"jsonc"` language re-registration (`package.json:46`) for `tsconfig.json`, `jsconfig.json` and glob variants (`tsconfig.*.json`, `jsconfig.*.json`, `tsconfig-*.json`, `jsconfig-*.json`), assigning these files to the JSON-with-comments mode.
- `"json"` re-registration (`package.json:58`) for `.tsbuildinfo` files.

**Grammar registrations** (`package.json:65-127`):
- `typescript` → `source.ts` scope, grammar at `./syntaxes/TypeScript.tmLanguage.json` (`package.json:68-69`).
  - `unbalancedBracketScopes` list (`package.json:70-77`) suppresses bracket-colorization for relational operators, arrow storage types, bitwise shift, angle-brace meta-scopes, JSX tag punctuations, and compound bitwise assignment — preventing false bracket-pair highlights on `<`, `>`, and `=>`.
  - `tokenTypes` (`package.json:78-84`) overrides semantic token classification for template-expression punctuation, JSDoc instance names, tagged-template function names, import string literals, and JSDoc variable references — all forced to type `"other"` to prevent them from being treated as string/comment tokens by semantic highlighting.
- `typescriptreact` → `source.tsx` scope, grammar at `./syntaxes/TypeScriptReact.tmLanguage.json` (`package.json:87-88`).
  - `embeddedLanguages` (`package.json:97-102`) maps four TSX meta-scopes to embedded language ids: `meta.tag.tsx` → `jsx-tags`, `meta.tag.without-attributes.tsx` → `jsx-tags`, `meta.tag.attributes.tsx` → `typescriptreact`, `meta.embedded.expression.tsx` → `typescriptreact`. This enables correct language server routing for embedded JSX content.
- Two JSDoc injection grammars (`package.json:111-127`):
  - `documentation.injection.ts` injects into `source.ts` and `source.tsx`.
  - `documentation.injection.js.jsx` injects into `source.js` and `source.js.jsx`.

**Semantic token scope mappings** (`package.json:128-187`):
Defined separately for `typescript` and `typescriptreact`. Maps semantic token types produced by the TypeScript language server to TextMate scope names, so themes can style them with existing CSS-like selectors. Key mappings:
- `property` → `variable.other.property.ts` / `.tsx`
- `property.readonly` → `variable.other.constant.property.ts`
- `variable` → `variable.other.readwrite.ts`
- `variable.readonly` → `variable.other.constant.object.ts`
- `function` → `entity.name.function.ts`
- `namespace` → `entity.name.type.module.ts`
- `variable.defaultLibrary` → `support.variable.ts`
- `function.defaultLibrary` → `support.function.ts`

**Snippets** (`package.json:188-197`): Same `./snippets/typescript.code-snippets` file registered for both `typescript` and `typescriptreact`.

---

#### `language-configuration.json`

**Role**: Defines editor mechanical behaviors for both `typescript` and `typescriptreact` language modes (both reference this single file, as noted in `language-configuration.json:2`).

**Comments** (`language-configuration.json:3-9`): Line comment `//` and block comment `/* */`.

**Brackets** (`language-configuration.json:10-27`): Four pairs registered: `${ }`, `{ }`, `[ ]`, `( )`. The `${` / `}` pair handles template-literal expression bracket matching.

**Auto-closing pairs** (`language-configuration.json:28-74`):
- `${` → `}` (no context restriction)
- `{` → `}`, `[` → `]`, `(` → `)` (no restriction)
- `'` → `'` with `notIn: ["string", "comment"]` (`language-configuration.json:48-51`)
- `"` → `"` with `notIn: ["string"]` (`language-configuration.json:55-58`)
- `` ` `` → `` ` `` with `notIn: ["string", "comment"]` (`language-configuration.json:62-66`)
- `/**` → ` */` with `notIn: ["string"]` (`language-configuration.json:68-73`) — auto-closes JSDoc block starts.

**Surrounding pairs** (`language-configuration.json:76-113`): Includes `${ }`, `$ ` (dollar-only pair), `{ }`, `[ ]`, `( )`, `' '`, `" "`, `` ` `` `` ` ``, `< >`. The `< >` pair appears only here (not in brackets), enabling selection-wrapping for angle brackets without affecting bracket matching.

**Colorized bracket pairs** (`language-configuration.json:114-131`): `( )`, `[ ]`, `{ }`, `< >` — the four pairs eligible for rainbow bracket colorization.

**Auto-close before** (`language-configuration.json:132`): String `";:.,=}])>\` \n\t"` — auto-close is suppressed when the cursor is positioned immediately before any of these characters.

**Folding** (`language-configuration.json:133-137`): Region markers `// #region` / `// #endregion` (and `//region` / `//endregion` variants, matched by `#?` in the regex at `language-configuration.json:135-136`).

**Word pattern** (`language-configuration.json:139-141`): Regex `(-?\d*\.\d\w*)|([^\`\@\~\!\%\^\&\*\(\)\-\=\+\[\{\]\}\\\|\;\:\'\"\,\.\<\>\/\?\s]+)` — matches numeric literals and identifier-like tokens (excluding operators and whitespace).

**Indentation rules** (`language-configuration.json:142-155`):
- `decreaseIndentPattern`: triggers on lines that are `}`, `]`, or `)` prefixed by whitespace.
- `increaseIndentPattern`: triggers on lines ending with an open `{`, `(`, or `[` without matching close.
- `unIndentedLinePattern`: handles `* ... */` and `*/` lines inside block comments.
- `indentNextLinePattern`: handles single-line `if`, `while`, `for`, `else` without braces, and `=>` arrow expressions.

**onEnterRules** (`language-configuration.json:157-270`): Ten rules controlling what happens on Enter key:
1. `language-configuration.json:159-170`: `/** ... */` on same line → `indentOutdent` + append ` * `.
2. `language-configuration.json:172-180`: `/**` open without close → append ` * `.
3. `language-configuration.json:182-192`: continuation line `* ...` inside JSDoc → append `* `.
4. `language-configuration.json:194-202`: ` */` closing line → `removeText: 1`.
5. `language-configuration.json:204-212`: `* ... */` → `removeText: 1`.
6. `language-configuration.json:214-224`: `case X:` / `default:` → indent next line.
7. `language-configuration.json:226-232`: single-line `if`/`for`/`while`/`else` → `outdent` on subsequent non-brace line.
8. `language-configuration.json:234-241`: Enter inside `(...)` → `indentOutdent` + tab.
9. `language-configuration.json:243-250`: Enter inside `{...}` → `indentOutdent` + tab.
10. `language-configuration.json:252-259`: Enter inside `[...]` → `indentOutdent` + tab.
11. `language-configuration.json:261-268`: Enter after `// comment` with following content → append `// `.

---

#### `build/update-grammars.mjs`

**Role**: Offline build/maintenance script for pulling upstream grammar updates from `microsoft/TypeScript-TmLanguage` and writing patched versions into the `syntaxes/` directory. Also generates the derived JavaScript grammars in the sibling `extensions/javascript/` extension.

**Import** (`update-grammars.mjs:7`): Consumes `update` from the `vscode-grammar-updater` npm package, which fetches a `.tmLanguage` file from a GitHub repository, converts it from XML PList to JSON, applies a transform callback, and writes the output file.

**`removeDom(grammar)`** (`update-grammars.mjs:9-28`): Filters `grammar.repository['support-objects'].patterns` by removing:
- Patterns whose `match` field references `HTMLElement`, `ATTRIBUTE_NODE`, `stopImmediatePropagation`, `JSON`, or `Math` (`update-grammars.mjs:11-15`).
- Patterns whose `name` starts with `support.class.error.`, `support.class.builtin.`, or `support.function.` (`update-grammars.mjs:19-23`). This strips DOM API and built-in type highlighting from the upstream grammar.

**`removeNodeTypes(grammar)`** (`update-grammars.mjs:31-49`): Filters `grammar.repository['support-objects'].patterns` to remove:
- Patterns named with `support.variable.object.node` or `support.class.node.` prefixes (`update-grammars.mjs:34-36`).
- Patterns whose captures contain `support.variable.object.process` or `support.class.console` (`update-grammars.mjs:39-44`). Removes Node.js-specific globals.

**`patchJsdoctype(grammar)`** (`update-grammars.mjs:51-59`): Filters `grammar.repository['jsdoctype'].patterns` to remove any pattern whose `name` includes the string `"illegal"`, preventing illegal-token highlighting in JSDoc type expressions from propagating from the upstream grammar.

**`patchGrammar(grammar)`** (`update-grammars.mjs:61-63`): Composes the three filters as `removeNodeTypes(removeDom(patchJsdoctype(grammar)))`.

**`adaptToJavaScript(grammar, replacementScope)`** (`update-grammars.mjs:65-89`): Mutates the TSX grammar to produce a JavaScript grammar by:
- Renaming `grammar.name` to `"JavaScript (with React support)"` (`update-grammars.mjs:66`).
- Setting `grammar.fileTypes` to `['.js', '.jsx', '.es6', '.mjs', '.cjs']` (`update-grammars.mjs:67`).
- Replacing `grammar.scopeName` with `source${replacementScope}` (e.g., `source.js` or `source.js.jsx`) (`update-grammars.mjs:68`).
- Walking the entire `repository` recursively via `fixScopeNames()` (`update-grammars.mjs:70-88`), replacing all occurrences of `.tsx` in `rule.name` and `rule.contentName` strings with the `replacementScope` string, so TSX-scoped token names become JS-scoped.

**Grammar update invocations** (`update-grammars.mjs:91-95`):
- `update('microsoft/TypeScript-TmLanguage', 'TypeScript.tmLanguage', './syntaxes/TypeScript.tmLanguage.json', patchGrammar)` — pulls `TypeScript.tmLanguage`, patches, writes `TypeScript.tmLanguage.json`.
- `update(..., 'TypeScriptReact.tmLanguage', './syntaxes/TypeScriptReact.tmLanguage.json', patchGrammar)` — pulls TSX grammar.
- `update(..., 'TypeScriptReact.tmLanguage', '../javascript/syntaxes/JavaScript.tmLanguage.json', grammar => adaptToJavaScript(patchGrammar(grammar), '.js'))` — derives JS grammar.
- `update(..., 'TypeScriptReact.tmLanguage', '../javascript/syntaxes/JavaScriptReact.tmLanguage.json', grammar => adaptToJavaScript(patchGrammar(grammar), '.js.jsx'))` — derives JSX grammar.

This reveals that `extensions/javascript/syntaxes/JavaScript.tmLanguage.json` and `JavaScriptReact.tmLanguage.json` are generated from the TSX grammar by `update-grammars.mjs`, making `typescript-basics` the upstream source for JavaScript highlighting too.

---

#### `syntaxes/TypeScript.tmLanguage.json`

**Role**: The primary TextMate grammar for `.ts`, `.cts`, `.mts` files. 5751 lines, 146 named rules in the `repository` object. Scope name `source.ts` (`TypeScript.tmLanguage.json:9`). Upstream provenance from `microsoft/TypeScript-TmLanguage` at commit `48f608692aa6d6ad7bd65b478187906c798234a8` (`TypeScript.tmLanguage.json:7`).

**Top-level patterns** (`TypeScript.tmLanguage.json:10-20`): Three include rules — `#directives`, `#statements`, `#shebang`.

**Repository structure** (146 rules, first 20): `shebang`, `statements`, `declaration`, `control-statement`, `label`, `expression`, `expressionWithoutIdentifiers`, `expressionPunctuations`, `decorator`, `var-expr`, `var-single-variable`, `var-single-const`, `var-single-variable-type-annotation`, `destructuring-variable`, `destructuring-const`, `object-binding-element`, `object-binding-element-const`, `object-binding-element-propertyName`, `binding-element`, `binding-element-const`.

Key sub-hierarchies:
- **`declaration`**: Includes `#decorator`, `#var-expr`, `#function-declaration`, `#class-declaration`, `#interface-declaration`, `#enum-declaration`, `#namespace-declaration`, `#type-alias-declaration`, `#import-equals-declaration`.
- **`docblock`**: Large rule handling all JSDoc tags (`@param`, `@returns`, `@template`, `@typedef`, `@example`, `@author`, `@see`, etc.) with detailed capture groups producing `storage.type.class.jsdoc`, `variable.other.jsdoc`, `entity.name.type.instance.jsdoc` scopes. Referenced by both JSDoc injection grammars via `source.ts#docblock`.
- **`jsdoctype`**: Type expression rule used within `docblock`, filtered by `patchJsdoctype` in the build script.

---

#### `syntaxes/TypeScriptReact.tmLanguage.json`

**Role**: TextMate grammar for `.tsx` files. 6000 lines, scope name `source.tsx` (`TypeScriptReact.tmLanguage.json:9`). Extends the TypeScript grammar with JSX/TSX tag recognition rules. The embedded language mappings declared in `package.json:97-102` connect TSX meta-scopes to VS Code's language routing system for JSX-specific features.

---

#### `syntaxes/jsdoc.ts.injection.tmLanguage.json`

**Role**: Grammar injection into `comment.block.documentation` scope within `source.ts` and `source.tsx`. The `injectionSelector` `"L:comment.block.documentation"` (`jsdoc.ts.injection.tmLanguage.json:3`) ensures this grammar activates inside JSDoc block comments. The `jsdocbody` rule (`jsdoc.ts.injection.tmLanguage.json:8-17`) uses a `begin`/`while` pair to match multi-line `/** ... */` blocks, then includes `source.ts#docblock` to apply JSDoc tag highlighting using the main grammar's `docblock` repository rule.

---

#### `syntaxes/jsdoc.js.injection.tmLanguage.json`

**Role**: Identical in structure to `jsdoc.ts.injection.tmLanguage.json` but with `scopeName: "documentation.injection.js.jsx"` (`jsdoc.js.injection.tmLanguage.json:21`) and injects into `source.js` / `source.js.jsx`. It also delegates to `source.ts#docblock` (`jsdoc.js.injection.tmLanguage.json:15`) — the TypeScript grammar's `docblock` rule is shared for JavaScript JSDoc highlighting.

---

#### `snippets/typescript.code-snippets`

**Role**: Code snippets registered for both `typescript` and `typescriptreact`. 32 snippet entries covering: `ctor`, `class`, `public method`, `private method`, `import`, `get`, `log`, `warn`, `error`, `prop`, `ref`, `set`, `throw`, `for`, `foreach =>`, `forin`, `forof`, `forawaitof`, `function`, `if`, `ifelse`, `new`, `switch`, `while`, `dowhile`, `trycatch`, `settimeout`, `setinterval`, `#region`, `#endregion`, `newpromise`, `async function`, `async arrow function`.

Notable: `"Class Definition"` has `"isFileTemplate": true` (`typescript.code-snippets:17`), marking it as a file-level scaffolding template. Snippets use `$TM_SELECTED_TEXT` for wrapping selected text within loop bodies and similar constructs.

---

#### `cgmanifest.json`

**Role**: Component governance manifest for open-source compliance tracking. Records the upstream `TypeScript-TmLanguage` repository (`github.com/microsoft/TypeScript-TmLanguage`), commit hash `48f608692aa6d6ad7bd65b478187906c798234a8` (`cgmanifest.json:10`), and MIT license (`cgmanifest.json:12`).

---

#### `.vscodeignore`

Excludes from packaged VSIX: `build/**`, `src/**`, `test/**`, `tsconfig*.json`, `**/*.tsbuildinfo`, `cgmanifest.json`, `syntaxes/Readme.md`. The build tooling and component manifest are development-only artifacts not shipped to end users.

---

### Cross-Cutting Synthesis

The `typescript-basics` extension is a pure declarative grammar/snippet contribution with no runtime activation. Its architecture consists of four layers: (1) language identity and file association declared in `package.json:17-63`; (2) editor mechanical behavior (bracket pairing, auto-close, indentation, folding) in `language-configuration.json`; (3) TextMate tokenization via two large upstream-derived grammars (`TypeScript.tmLanguage.json` at 5751 lines and `TypeScriptReact.tmLanguage.json` at 6000 lines) supplemented by two JSDoc injection grammars; (4) a build-time maintenance script (`update-grammars.mjs`) that fetches grammars from `microsoft/TypeScript-TmLanguage`, strips DOM and Node.js globals, patches JSDoc illegal-token rules, and also derives the sibling JavaScript extension's grammars by scope-name substitution from the TSX grammar. Semantic token scope mappings in `package.json:128-187` bridge the language server's semantic token stream to TextMate scope names, enabling theme-based styling of server-provided token kinds without additional extension code. The `jsdoc.js.injection.tmLanguage.json` grammar extends this extension's reach into `source.js` and `source.js.jsx` scopes by re-using the `source.ts#docblock` repository rule, making the TypeScript grammar's JSDoc rule set the canonical JSDoc highlighter for both TypeScript and JavaScript.

---

### Out-of-Partition References

- `/Users/norinlavaee/vscode-atomic/extensions/javascript/syntaxes/JavaScript.tmLanguage.json` — Generated by `update-grammars.mjs:94` from the TSX grammar via `adaptToJavaScript(..., '.js')`. Belongs to `extensions/javascript/` (partition 57 or adjacent).
- `/Users/norinlavaee/vscode-atomic/extensions/javascript/syntaxes/JavaScriptReact.tmLanguage.json` — Generated by `update-grammars.mjs:95` via `adaptToJavaScript(..., '.js.jsx')`. Same cross-partition dependency.
- `source.ts#docblock` repository rule — The JSDoc injection grammars for both `ts` and `js` scopes reference this internal rule from `TypeScript.tmLanguage.json`. Any consumer of `documentation.injection.js.jsx` (within the `extensions/javascript/` partition) depends on this rule being present in the TypeScript grammar's repository.
- `vscode-grammar-updater` npm package — External build-time dependency used in `update-grammars.mjs:7`. Not part of any partition; lives in `node_modules`.
- `microsoft/TypeScript-TmLanguage` GitHub repository — Upstream grammar source at commit `48f608692aa6d6ad7bd65b478187906c798234a8`, tracked in `cgmanifest.json` and fetched by `update-grammars.mjs:91-95`.
- `extensions/typescript-language-features/` — The language server extension (TypeScript Language Features) that produces semantic tokens consumed via the `semanticTokenScopes` mappings defined in `package.json:128-187`. That partition owns the LSP-based intelligence; this partition owns only the static token scope name mapping.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: TypeScript Basics Extension
## Partition 58/79 - Language Grammar, Snippets, and Configuration

---

## Pattern 1: Extension Configuration and Language Declaration

**Found in**: `/extensions/typescript-basics/package.json:1-64`  
**Used for**: Declaring TypeScript and TypeScript React language support with grammar bindings

```json
{
  "name": "typescript",
  "description": "%description%",
  "displayName": "%displayName%",
  "version": "10.0.0",
  "author": "vscode",
  "publisher": "vscode",
  "license": "MIT",
  "engines": {
    "vscode": "*"
  },
  "scripts": {
    "update-grammar": "node ./build/update-grammars.mjs"
  },
  "categories": ["Programming Languages"],
  "contributes": {
    "languages": [
      {
        "id": "typescript",
        "aliases": ["TypeScript", "ts", "typescript"],
        "extensions": [".ts", ".cts", ".mts"],
        "firstLine": "^#!.*\\b(deno|bun|ts-node)\\b",
        "configuration": "./language-configuration.json"
      },
      {
        "id": "typescriptreact",
        "aliases": ["TypeScript JSX", "TypeScript React", "tsx"],
        "extensions": [".tsx"],
        "configuration": "./language-configuration.json"
      }
    ]
  }
}
```

**Key aspects**:
- Multi-variant language support: `typescript`, `typescriptreact`, `jsonc`, `json`
- File extension mapping with multiple variants (`.ts`, `.cts`, `.mts`)
- Shebang-based language detection for runtime-specific files
- Centralized language configuration reference
- Grammar contributions for syntax highlighting

---

## Pattern 2: Grammar Registration and Token Mapping

**Found in**: `/extensions/typescript-basics/package.json:65-127`  
**Used for**: Registering TextMate grammars with semantic token scope mappings

```json
"grammars": [
  {
    "language": "typescript",
    "scopeName": "source.ts",
    "path": "./syntaxes/TypeScript.tmLanguage.json",
    "unbalancedBracketScopes": [
      "keyword.operator.relational",
      "storage.type.function.arrow",
      "keyword.operator.bitwise.shift",
      "meta.brace.angle",
      "punctuation.definition.tag",
      "keyword.operator.assignment.compound.bitwise.ts"
    ],
    "tokenTypes": {
      "punctuation.definition.template-expression": "other",
      "entity.name.type.instance.jsdoc": "other",
      "entity.name.function.tagged-template": "other",
      "meta.import string.quoted": "other",
      "variable.other.jsdoc": "other"
    }
  },
  {
    "language": "typescriptreact",
    "scopeName": "source.tsx",
    "path": "./syntaxes/TypeScriptReact.tmLanguage.json",
    "unbalancedBracketScopes": [
      "keyword.operator.relational",
      "storage.type.function.arrow",
      "keyword.operator.bitwise.shift",
      "punctuation.definition.tag",
      "keyword.operator.assignment.compound.bitwise.ts"
    ],
    "embeddedLanguages": {
      "meta.tag.tsx": "jsx-tags",
      "meta.tag.without-attributes.tsx": "jsx-tags",
      "meta.tag.attributes.tsx": "typescriptreact",
      "meta.embedded.expression.tsx": "typescriptreact"
    }
  }
]
```

**Key aspects**:
- Grammar files derived from external source (microsoft/TypeScript-TmLanguage)
- Unbalanced bracket scopes for angle bracket matching
- Language-specific embedded language support (JSX)
- Token type overrides for semantic accuracy

---

## Pattern 3: Code Snippets with Placeholder Substitution

**Found in**: `/extensions/typescript-basics/snippets/typescript.code-snippets:1-50`  
**Used for**: Defining reusable code templates with variable placeholders

```json
{
  "Constructor": {
    "prefix": "ctor",
    "body": [
      "/**",
      " *",
      " */",
      "constructor() {",
      "\tsuper();",
      "\t$0",
      "}",
    ],
    "description": "Constructor"
  },
  "Class Definition": {
    "prefix": "class",
    "isFileTemplate": true,
    "body": [
      "class ${1:name} {",
      "\tconstructor(${2:parameters}) {",
      "\t\t$0",
      "\t}",
      "}"
    ],
    "description": "Class Definition"
  },
  "Import Statement": {
    "prefix": "import",
    "body": [
      "import { $0 } from \"${1:module}\";"
    ],
    "description": "Import external module"
  },
  "Property getter": {
    "prefix": "get",
    "body": [
      "",
      "public get ${1:value}() : ${2:string} {",
      "\t${3:return $0}",
      "}",
      ""
    ],
    "description": "Property getter"
  }
}
```

**Key aspects**:
- Numbered placeholder syntax: `${1:name}`, `${2:parameters}`, `${3:element}`
- Final cursor position marker: `$0`
- File template flag for class definitions
- Multi-line bodies with tab-based indentation
- Descriptions for IDE tooltips

**Variations**: Console logging, loops, async functions, promise patterns:

```json
{
  "Log to the console": {
    "prefix": "log",
    "body": ["console.log($1);", "$0"],
    "description": "Log to the console"
  },
  "For Loop": {
    "prefix": "for",
    "body": [
      "for (let ${1:index} = 0; ${1:index} < ${2:array}.length; ${1:index}++) {",
      "\tconst ${3:element} = ${2:array}[${1:index}];",
      "\t$TM_SELECTED_TEXT$0",
      "}"
    ],
    "description": "For Loop"
  },
  "Async Function Statement": {
    "prefix": "async function",
    "body": [
      "async function ${1:name}(${2:params}:${3:type}) {",
      "\t$TM_SELECTED_TEXT$0",
      "}"
    ],
    "description": "Async Function Statement"
  }
}
```

---

## Pattern 4: Language Configuration with Bracket Matching and Auto-Closing

**Found in**: `/extensions/typescript-basics/language-configuration.json:1-75`  
**Used for**: Configuring editor behavior for bracket matching, auto-closing, and auto-indentation

```json
{
  "comments": {
    "lineComment": "//",
    "blockComment": ["/*", "*/"]
  },
  "brackets": [
    ["${", "}"],
    ["{", "}"],
    ["[", "]"],
    ["(", ")"]
  ],
  "autoClosingPairs": [
    { "open": "${", "close": "}" },
    { "open": "{", "close": "}" },
    { "open": "[", "close": "]" },
    { "open": "(", "close": ")" },
    { "open": "'", "close": "'", "notIn": ["string", "comment"] },
    { "open": "\"", "close": "\"", "notIn": ["string"] },
    { "open": "`", "close": "`", "notIn": ["string", "comment"] },
    { "open": "/**", "close": " */", "notIn": ["string"] }
  ],
  "surroundingPairs": [
    ["${", "}"],
    ["$", ""],
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
    ["'", "'"],
    ["\"", "\""],
    ["`", "`"],
    ["<", ">"]
  ],
  "colorizedBracketPairs": [
    ["(", ")"],
    ["[", "]"],
    ["{", "}"],
    ["<", ">"]
  ],
  "autoCloseBefore": ";:.,=}])>` \n\t"
}
```

**Key aspects**:
- Template expression brackets: `${...}`
- Context-aware auto-closing (not in strings/comments)
- JSDoc block comment pattern
- Surrounding pairs for wrapping selection
- Colorized bracket pair highlighting
- Close-before characters to prevent double insertion

---

## Pattern 5: OnEnter Rules for Comment and Indentation Behavior

**Found in**: `/extensions/typescript-basics/language-configuration.json:157-271`  
**Used for**: Configuring automatic indentation and text insertion on Enter key press

```json
"onEnterRules": [
  {
    // e.g. /** | */
    "beforeText": {
      "pattern": "^\\s*/\\*\\*(?!/)([^\\*]|\\*(?!/))*$"
    },
    "afterText": {
      "pattern": "^\\s*\\*/$"
    },
    "action": {
      "indent": "indentOutdent",
      "appendText": " * "
    }
  },
  {
    // e.g.  * ...|
    "beforeText": {
      "pattern": "^(\\t|[ ])*\\*([ ]([^\\*]|\\*(?!/))*)?$"
    },
    "previousLineText": {
      "pattern": "(?=^(\\s*(/\\*\\*|\\*)).*)(?=(?!(\\s*\\*/))"
    },
    "action": {
      "indent": "none",
      "appendText": "* "
    }
  },
  {
    "beforeText": {
      "pattern": "^\\s*(\\bcase\\s.+:|\\bdefault:)$"
    },
    "afterText": {
      "pattern": "^(?!\\s*(\\bcase\\b|\\bdefault\\b))"
    },
    "action": {
      "indent": "indent"
    }
  },
  {
    // Indent when pressing enter from inside ()
    "beforeText": "^.*\\([^\\)]*$",
    "afterText": "^\\s*\\).*$",
    "action": {
      "indent": "indentOutdent",
      "appendText": "\t"
    }
  },
  {
    // Add // when pressing enter from inside line comment
    "beforeText": "(?<!\\\\|\\w:)\/\/\\s*\\S",
    "afterText": "^(?!\\s*$).+",
    "action": {
      "indent": "none",
      "appendText": "// "
    }
  }
]
```

**Key aspects**:
- Multi-condition rules (beforeText, afterText, previousLineText)
- JSDoc continuation pattern matching
- Switch/case indentation handling
- Context-aware actions: indent, indentOutdent, outdent
- Comment prefix continuation
- Parenthesis-aware indentation

---

## Pattern 6: Grammar File Patching and Adaptation Build Script

**Found in**: `/extensions/typescript-basics/build/update-grammars.mjs:1-96`  
**Used for**: Automating grammar file updates from upstream and removing unnecessary patterns

```javascript
import { update } from 'vscode-grammar-updater';

function removeDom(grammar) {
  grammar.repository['support-objects'].patterns = grammar.repository['support-objects'].patterns.filter(pattern => {
    if (pattern.match && (
      /\b(HTMLElement|ATTRIBUTE_NODE|stopImmediatePropagation)\b/g.test(pattern.match)
      || /\bJSON\b/g.test(pattern.match)
      || /\bMath\b/g.test(pattern.match)
    )) {
      return false;
    }
    if (pattern.name?.startsWith('support.class.error.')
      || pattern.name?.startsWith('support.class.builtin.')
      || pattern.name?.startsWith('support.function.')
    ) {
      return false;
    }
    return true;
  });
  return grammar;
}

function removeNodeTypes(grammar) {
  grammar.repository['support-objects'].patterns = grammar.repository['support-objects'].patterns.filter(pattern => {
    if (pattern.name) {
      if (pattern.name.startsWith('support.variable.object.node') || pattern.name.startsWith('support.class.node.')) {
        return false;
      }
    }
    if (pattern.captures) {
      if (Object.values(pattern.captures).some(capture =>
        capture.name && (capture.name.startsWith('support.variable.object.process')
          || capture.name.startsWith('support.class.console'))
      )) {
        return false;
      }
    }
    return true;
  });
  return grammar;
}

function patchJsdoctype(grammar) {
  grammar.repository['jsdoctype'].patterns = grammar.repository['jsdoctype'].patterns.filter(pattern => {
    if (pattern.name && pattern.name.includes('illegal')) {
      return false;
    }
    return true;
  });
  return grammar;
}

function patchGrammar(grammar) {
  return removeNodeTypes(removeDom(patchJsdoctype(grammar)));
}

var tsGrammarRepo = 'microsoft/TypeScript-TmLanguage';
update(tsGrammarRepo, 'TypeScript.tmLanguage', './syntaxes/TypeScript.tmLanguage.json', grammar => patchGrammar(grammar));
update(tsGrammarRepo, 'TypeScriptReact.tmLanguage', './syntaxes/TypeScriptReact.tmLanguage.json', grammar => patchGrammar(grammar));
update(tsGrammarRepo, 'TypeScriptReact.tmLanguage', '../javascript/syntaxes/JavaScript.tmLanguage.json', grammar => adaptToJavaScript(patchGrammar(grammar), '.js'));
```

**Key aspects**:
- Grammar updater from external source (microsoft/TypeScript-TmLanguage)
- Filtering pipelines for removing bloat (DOM objects, Node.js types)
- Name-based scope filtering for built-in types
- Composition of patch functions
- Batch updates for multiple output files

---

## Pattern 7: Semantic Token Scope Mapping for Language Intelligence

**Found in**: `/extensions/typescript-basics/package.json:128-187`  
**Used for**: Mapping semantic token types to TextMate scopes for IDE features

```json
"semanticTokenScopes": [
  {
    "language": "typescript",
    "scopes": {
      "property": [
        "variable.other.property.ts"
      ],
      "property.readonly": [
        "variable.other.constant.property.ts"
      ],
      "variable": [
        "variable.other.readwrite.ts"
      ],
      "variable.readonly": [
        "variable.other.constant.object.ts"
      ],
      "function": [
        "entity.name.function.ts"
      ],
      "namespace": [
        "entity.name.type.module.ts"
      ],
      "variable.defaultLibrary": [
        "support.variable.ts"
      ],
      "function.defaultLibrary": [
        "support.function.ts"
      ]
    }
  },
  {
    "language": "typescriptreact",
    "scopes": {
      "property": [
        "variable.other.property.tsx"
      ],
      "property.readonly": [
        "variable.other.constant.property.tsx"
      ],
      "variable": [
        "variable.other.readwrite.tsx"
      ],
      "variable.readonly": [
        "variable.other.constant.object.tsx"
      ],
      "function": [
        "entity.name.function.tsx"
      ],
      "namespace": [
        "entity.name.type.module.tsx"
      ]
    }
  }
]
```

**Key aspects**:
- Bidirectional mapping: semantic tokens → scopes
- Read-only property differentiation
- Default library distinction for built-in APIs
- Language-specific scope variants (ts vs tsx)
- Supports IDE features like semantic highlighting and symbol navigation

---

## Summary

The `typescript-basics` extension demonstrates the foundational patterns for language support in VS Code, structured as static assets and configuration rather than runtime logic. Key patterns include:

1. **Extension Metadata Pattern**: Clean separation of language declaration, grammar registration, and snippet contributions in `package.json`

2. **Grammar Management Pattern**: Derived TextMate grammars from upstream source with systematic patching to customize scope names, remove bloat, and maintain synchronization

3. **Snippet Definition Pattern**: Template-based code generation with numbered placeholders, context awareness, and IDE integration hooks

4. **Editor Behavior Configuration Pattern**: Declarative rules for bracket matching, auto-closing, auto-indentation, and comment continuation without code

5. **Semantic Token Bridge Pattern**: Mapping semantic language features to TextMate scopes to connect language intelligence with syntax highlighting

6. **Build Automation Pattern**: Composition of filter functions for grammar patching, enabling reproducible updates from upstream sources

These patterns are relevant for a Tauri/Rust port because:
- Language grammar systems could be abstracted and ported independently
- Configuration schema could be replicated in Rust data structures
- Snippet and scope mapping systems are language-agnostic
- The grammar patching pipeline demonstrates data transformation patterns applicable to Rust
- Semantic token mapping shows how to bridge language servers to UI rendering across architectures

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
