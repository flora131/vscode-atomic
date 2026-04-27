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
