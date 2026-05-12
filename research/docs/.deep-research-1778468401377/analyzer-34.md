### Files Analysed

1. `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/extensionEditingMain.ts`
2. `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/extensionEditingBrowserMain.ts`
3. `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/packageDocumentHelper.ts`
4. `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/extensionLinter.ts`
5. `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/packageDocumentL10nSupport.ts`
6. `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/extensionEngineValidation.ts`
7. `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/jsonReconstruct.ts`
8. `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/constants.ts`
9. `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/package.json`

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/extensionEditingMain.ts`

- **Role:** Desktop (Electron/Node) activation entry point. Registers all four language intelligence providers and the linter for package.json editing.
- **Key symbols:**
  - `activate(context)` at line 11 — pushes four disposables into `context.subscriptions`
  - `registerPackageDocumentCompletions()` at line 25 — returns a `vscode.Disposable` wrapping a completion provider
  - `registerCodeActionsProvider()` at line 33 — returns a `vscode.Disposable` wrapping a code-action provider
- **Control flow:**
  1. `activate` calls `registerPackageDocumentCompletions()` (line 14), `registerCodeActionsProvider()` (line 17), constructs `new PackageDocumentL10nSupport()` (line 20), and `new ExtensionLinter()` (line 22); all pushed to subscriptions.
  2. `registerPackageDocumentCompletions` calls `vscode.languages.registerCompletionItemProvider` with selector `{ language: 'json', pattern: '**/package.json' }` (line 26); on each invocation, creates `new PackageDocument(document)` and delegates to `provideCompletionItems` (line 28).
  3. `registerCodeActionsProvider` calls `vscode.languages.registerCodeActionsProvider` with the same selector (line 34); delegates to `PackageDocument.provideCodeActions` (line 36).
- **Data flow:** Activation context -> registered providers -> per-request instantiation of `PackageDocument` with the active `vscode.TextDocument`.
- **Dependencies:** `vscode` API, `./packageDocumentHelper`, `./packageDocumentL10nSupport`, `./extensionLinter`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/extensionEditingBrowserMain.ts`

- **Role:** Web/browser activation entry point. Registers only completion and L10n definition support — omits `ExtensionLinter` (which requires Node.js `fs`, `path`, `URL`, markdown-it, and parse5).
- **Key symbols:**
  - `activate(context)` at line 10 — pushes two disposables
  - `registerPackageDocumentCompletions()` at line 18 — identical to desktop version
- **Control flow:** Same pattern as the desktop entry but no linter registration. The browser build at `dist/browser/extensionEditingBrowserMain` is referenced via the `"browser"` field in `package.json:17`.
- **Data flow:** Same as desktop for the two registered providers; `ExtensionLinter` is entirely absent.
- **Dependencies:** `vscode` API, `./packageDocumentHelper`, `./packageDocumentL10nSupport`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/packageDocumentHelper.ts`

- **Role:** Provides completion items and code actions for `package.json` files. Currently implements only `contributes.configurationDefaults` language-override completions and quick-fix code actions for implicit/redundant activation event diagnostics.
- **Key symbols:**
  - `PackageDocument` class at line 11 — constructor takes `vscode.TextDocument` at line 13
  - `provideCompletionItems(position, _token)` at line 15 — entry for completion
  - `provideCodeActions(_range, context, _token)` at line 25 — entry for code actions
  - `provideLanguageOverridesCompletionItems(location, position)` at line 43 — handles path depth 2 (snippet) and depth 3 (language list)
  - `getReplaceRange(location, position)` at line 85 — computes the replacement range for the current node
  - `newSimpleCompletionItem(text, range, ...)` at line 96 and `newSnippetCompletionItem(o)` at line 105 — completion item factories
- **Control flow:**
  1. `provideCompletionItems` calls `jsonc-parser.getLocation` (line 16) to determine where in the JSON the cursor sits.
  2. If `location.path[1] === 'configurationDefaults'` (line 18), delegates to `provideLanguageOverridesCompletionItems`.
  3. In `provideLanguageOverridesCompletionItems`, path depth 2 returns a snippet item (lines 49–65); path depth 3 calls `vscode.languages.getLanguages()` and maps each language to a simple completion item (lines 73–80).
  4. `provideCodeActions` iterates `context.diagnostics`; for each diagnostic matching `implicitActivationEvent` or `redundantImplicitActivationEvent` (line 28), creates a QuickFix `CodeAction` with a `WorkspaceEdit` deleting the range. Handles trailing comma by extending the delete range one character (lines 31–36).
- **Data flow:** `vscode.TextDocument` text -> `jsonc-parser.getLocation` -> `Location.path`/`Location.previousNode` -> range computation -> `CompletionItem[]` or `CodeAction[]`.
- **Dependencies:** `vscode` API, `jsonc-parser` (`getLocation`, `Location`), `./constants`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/extensionLinter.ts`

- **Role:** The heaviest file (599 lines). Produces `Diagnostic` entries for `package.json` (activation events, image URLs, API proposals, when-clauses) and `README.md`/`CHANGELOG.md` (image URL/SVG validation). Runs on open, change, and close events with a 300ms debounce.
- **Key symbols:**
  - `ExtensionLinter` class at line 60
  - `diagnosticsCollection` at line 62 — `vscode.DiagnosticCollection` named `'extension-editing'`
  - `fileWatcher` at line 63 — `workspace.createFileSystemWatcher('**/package.json')`
  - `folderToPackageJsonInfo` at line 66 — cache mapping folder URI strings to `PackageJsonInfo`
  - `packageJsonQ` / `readmeQ` at line 67/68 — `Set<TextDocument>` queues
  - `timer` at line 69 — `NodeJS.Timeout` for debounce
  - `queue(document)` at line 85 — sorts document into `packageJsonQ` or `readmeQ`
  - `startTimer()` at line 102 — clears/resets 300ms timer calling `lint()`
  - `lint()` at line 112 — calls `lintPackageJson()` and `lintReadme()` in parallel
  - `lintPackageJson()` at line 119 — main package.json linting logic
  - `lintWhenClauses(contributesNode, document)` at line 204 — invokes `_validateWhenClauses` command
  - `lintReadme()` at line 272 — parses markdown with `markdown-it` and HTML fragments with `parse5`
  - `readPackageJsonInfo(folder, tree)` at line 394 — extracts `PackageJsonInfo` from parsed tree
  - `parseImplicitActivationEvents(tree)` at line 500 — builds a `Set<string>` of all implicit activation events derived from `contributes`
  - `addDiagnostics(...)` at line 441 — URL/SVG validation for individual image/badge references
- **Control flow:**
  1. Constructor registers listeners for `onDidOpenTextDocument`, `onDidChangeTextDocument`, `onDidCloseTextDocument`, file watcher events (lines 73–82). Immediately queues all open documents (line 82).
  2. `queue` classifies documents; `startTimer` debounces.
  3. `lintPackageJson` iterates `packageJsonQ`; for each non-closed doc, calls `parseTree` (line 128) then `readPackageJsonInfo` (line 129). Checks `isExtension`; if true, validates `icon`, `badges`, `enabledApiProposals`, `activationEvents`, and `contributes.when/enablement` (lines 132–198). Calls `lintWhenClauses` asynchronously (line 196).
  4. `lintWhenClauses` uses a recursive `findWhens` inner function (line 211) to collect `when`/`enablement` string nodes from `menus`, `views`, `viewsWelcome`, `keybindings`, `commands`. Passes all extracted string values to `commands.executeCommand('_validateWhenClauses', ...)` (line 247). Maps error offsets back to source positions via `JsonStringScanner.getOffsetInEncoded` (lines 253–259).
  5. `lintReadme` dynamically imports `markdown-it` (line 292) and `parse5` (line 341). Parses the full markdown, recurses token children to build `tokensAndPositions` array (lines 295–318). Validates `image` token `src` attributes. For inline text tokens, uses `parse5.SAXParser` to detect `<img>` and `<svg>` tags (lines 344–368).
  6. Engine version for activation-event suppression check: `info.engineVersion.majorBase > 1 || (majorBase === 1 && minorBase >= 75)` (line 165).
  7. `product.json` is read synchronously at module load (line 18) from `env.appRoot` to obtain `extensionAllowedBadgeProviders`, `extensionAllowedBadgeProvidersRegex`, and `extensionEnabledApiProposals`.
- **Data flow:** `TextDocument` -> `jsonc-parser.parseTree` -> `JsonNode` tree -> `findNodeAtLocation` -> field-specific validation -> `Diagnostic[]` -> `diagnosticsCollection.set(uri, diagnostics)`.
- **Dependencies:** Node.js `path`, `fs`, `url.URL`; `jsonc-parser` (`parseTree`, `findNodeAtLocation`, `getNodeValue`); `markdown-it` (dynamic import); `parse5` (dynamic import); `vscode` API; `./extensionEngineValidation`; `./jsonReconstruct`; `./constants`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/packageDocumentL10nSupport.ts`

- **Role:** Implements `DefinitionProvider` and `ReferenceProvider` for NLS (`%key%`) string references between `package.json` and `package.nls.json`. Registered for both file selectors in both directions.
- **Key symbols:**
  - `PackageDocumentL10nSupport` class at line 13 — implements `vscode.DefinitionProvider`, `vscode.ReferenceProvider`, `vscode.Disposable`
  - `packageJsonSelector` at line 10, `packageNlsJsonSelector` at line 11
  - `provideDefinition(document, position, _token)` at line 31 — dispatches based on filename
  - `provideNlsValueDefinition(...)` at line 44 — from `package.json` `%key%` -> definition in `package.nls.json`
  - `provideNlsKeyDefinition(...)` at line 54 — from `package.nls.json` key -> definition in `package.nls.json` itself
  - `resolveNlsDefinition(origin, nlsUri)` at line 62 — calls `findNlsKeyDeclaration` and returns `DefinitionLink[]`
  - `getNlsReferenceAtPosition(...)` at line 75 — extracts `%key%` from the token at cursor using regex `/^%(.+)%$/` (line 86)
  - `provideReferences(...)` at line 96 — dispatches to `provideNlsKeyReferences` or `provideNlsValueReferences`
  - `findNlsReferencesInPackageJson(nlsKey, packageJsonUri)` at line 166 — uses `jsonc-parser.visit` to collect all literal values matching `%key%` (line 178)
  - `findNlsKeyDeclaration(nlsKey, nlsUri)` at line 140 — opens `package.nls.json`, calls `parseTree` + `findNodeAtLocation([nlsKey])`, then navigates to the key node via `node.parent.children[0]`
  - `getNlsKeyDefinitionAtPosition(...)` at line 191 — validates cursor is on a top-level property key (path length === 1, `isAtPropertyKey` true)
- **Control flow:**
  1. Constructor registers `registerDefinitionProvider` twice and `registerReferenceProvider` twice (lines 18–22).
  2. `provideDefinition` branches on basename: `package.json` -> `provideNlsValueDefinition`; `package.nls.json` -> `provideNlsKeyDefinition`.
  3. `provideNlsValueDefinition` extracts the `%key%` pattern from the node at the cursor, builds the sibling `package.nls.json` URI via `Uri.joinPath(doc.uri, '..', 'package.nls.json')` (line 50), then resolves via `findNlsKeyDeclaration`.
  4. `provideReferences` branches similarly: from `package.nls.json` it finds references in `package.json`; from `package.json` it finds the declaration in `package.nls.json` (if `includeDeclaration`).
  5. `findNlsReferencesInPackageJson` walks the entire document with `jsonc-parser.visit` (line 178); each `onLiteralValue` callback checks equality with `%nlsKey%`.
- **Data flow:** Cursor position -> `jsonc-parser.getLocation` -> `previousNode.value` string -> regex match for `%key%` -> `Uri.joinPath` for sibling file -> `workspace.openTextDocument` -> `parseTree` / `visit` -> `vscode.Location[]` or `vscode.DefinitionLink[]`.
- **Dependencies:** `vscode` API, `jsonc-parser` (`getLocation`, `getNodeValue`, `parseTree`, `findNodeAtLocation`, `visit`).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/extensionEngineValidation.ts`

- **Role:** Pure version-string parsing utility. Converts the `engines.vscode` semver-like string to structured `IParsedVersion` and then to `INormalizedVersion` used to gate activation-event diagnostics.
- **Key symbols:**
  - `IParsedVersion` interface at line 8 — fields: `hasCaret`, `hasGreaterEquals`, `majorBase`, `majorMustEqual`, `minorBase`, `minorMustEqual`, `patchBase`, `patchMustEqual`, `preRelease`
  - `INormalizedVersion` interface at line 20 — fields include `notBefore` (timestamp) and `isMinimum`
  - `VERSION_REGEXP` at line 31 — `/^(\^|>=)?((\d+)|x)\.((\d+)|x)\.((\d+)|x)(\-.*)?$/`
  - `NOT_BEFORE_REGEXP` at line 32 — `/^-(\d{4})(\d{2})(\d{2})$/` for pre-release date
  - `isValidVersionStr(version)` at line 34
  - `parseVersion(version)` at line 39 — returns `IParsedVersion | null`; handles `*` wildcard (lines 46–57) and regex match (lines 60–75)
  - `normalizeVersion(version)` at line 77 — returns `INormalizedVersion | null`; applies caret rules (lines 89–96) and decodes `notBefore` from pre-release date string (lines 97–104)
- **Control flow:** `parseVersion` validates string, handles `*`, applies `VERSION_REGEXP`, maps capture groups to struct fields. `normalizeVersion` takes the parsed result and relaxes `minorMustEqual`/`patchMustEqual` when `hasCaret` is true based on whether `majorBase === 0`.
- **Data flow:** Raw version string -> `parseVersion` -> `IParsedVersion` -> `normalizeVersion` -> `INormalizedVersion` -> consumed by `ExtensionLinter.readPackageJsonInfo` (line 396 of `extensionLinter.ts`).
- **Dependencies:** None (pure TypeScript, no imports).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/jsonReconstruct.ts`

- **Role:** Provides `JsonStringScanner`, a stateful scanner that maps a decoded-string character offset back to an encoded-JSON-string byte offset. Used by `ExtensionLinter.lintWhenClauses` to convert error offsets returned by the `_validateWhenClauses` command (which operate on decoded strings) back to document-level character offsets.
- **Key symbols:**
  - `JsonStringScanner` class at line 11
  - `constructor(text, initialPos)` at line 20 — `text` is the full JSON document text; `initialPos` is `stringJSONNode.offset + 1` (past the opening quote)
  - `resultChars` at line 12 — count of decoded characters consumed so far
  - `pos` at line 13 — current position in the encoded text
  - `getOffsetInEncoded(offsetDecoded)` at line 25 — advances through escape sequences until `resultChars > offsetDecoded`, returning the `start` position in the encoded string
  - `scanHexDigits(count, exact?)` at line 68 — reads a hex sequence for `\uXXXX` escapes
  - `CharacterCodes` const enum at line 96 — all used character code constants
- **Control flow:** `getOffsetInEncoded` loops, checking each character. If it encounters a backslash (line 36), it reads the next character and dispatches on escape type: single-char escapes (`\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`) each count as 1 decoded char (line 50); `\uXXXX` sequences call `scanHexDigits(4, true)` and count based on `String.fromCharCode(ch3).length` (lines 53–56). Non-escape chars advance `pos` and `resultChars` by 1 each (lines 62–64).
- **Data flow:** Encoded JSON string + initial byte offset -> stateful scan -> returns encoded offset for a given decoded offset -> used by `ExtensionLinter` at lines 253–259 of `extensionLinter.ts`.
- **Dependencies:** None (pure TypeScript, no imports).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/src/constants.ts`

- **Role:** Exports two localized diagnostic message strings used as shared keys across `extensionLinter.ts` (to emit diagnostics) and `packageDocumentHelper.ts` (to match diagnostics for code actions).
- **Key symbols:**
  - `implicitActivationEvent` at line 8 — `l10n.t("This activation event cannot be explicitly listed by your extension.")`
  - `redundantImplicitActivationEvent` at line 9 — `l10n.t("This activation event can be removed as VS Code generates these automatically from your package.json contribution declarations.")`
- **Control flow:** Module-level constants initialized at import time via `vscode.l10n.t`.
- **Data flow:** String constants consumed by `ExtensionLinter.lintPackageJson` (line 170 for warning, line 179 for error) and matched in `PackageDocument.provideCodeActions` (line 28) to determine whether a quick fix applies.
- **Dependencies:** `vscode` (`l10n`).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/extension-editing/package.json`

- **Role:** Extension manifest. Declares dual entry points, activation triggers, runtime npm dependencies, and static JSON-schema contributions.
- **Key symbols:**
  - `"main": "./out/extensionEditingMain"` at line 16 — Node/Electron entry
  - `"browser": "./dist/browser/extensionEditingBrowserMain"` at line 17 — Web entry
  - `"activationEvents"` at line 12 — `["onLanguage:json", "onLanguage:markdown"]`
  - `"dependencies"` at line 28 — `jsonc-parser ^3.2.0`, `markdown-it ^12.3.2`, `parse5 ^3.0.2`
  - `"contributes.jsonValidation"` at line 34 — five file-pattern/schema-URL pairs linking `package.json`, `*language-configuration.json`, `*icon-theme.json`, `*product-icon-theme.json`, `*color-theme.json` to `vscode://schemas/...` URIs
  - `"contributes.languages"` at line 59 — declares `ignore` language for `.vscodeignore` files
  - `"capabilities.virtualWorkspaces": true` and `"untrustedWorkspaces": { "supported": true }` at lines 20–23
- **Data flow:** The manifest drives VS Code's extension host to load the correct compiled entry point. The `jsonValidation` contributions are consumed directly by VS Code's JSON language service without any TypeScript code in this extension.
- **Dependencies:** None at runtime beyond the npm deps declared; relies on vscode host for JSON schema resolution.

---

### Cross-Cutting Synthesis

The `extension-editing` partition implements a tightly scoped set of language intelligence features — completion, code actions, definition, references, and diagnostics — all targeted at `package.json` and `README.md`/`CHANGELOG.md` in VS Code extension projects. The architecture follows a clean VS Code extension pattern: two entry modules (`extensionEditingMain.ts` for desktop, `extensionEditingBrowserMain.ts` for web) register providers and a linter class into `context.subscriptions`. The `ExtensionLinter` is the only component that cannot run in a browser because it depends on Node.js `fs` (synchronous `product.json` read at line 18 of `extensionLinter.ts`), `path`, `url.URL`, and the dynamically imported `markdown-it` and `parse5` libraries. Everything else is browser-safe. The shared diagnostic-message strings in `constants.ts` form a coupling point between the linter (emitter) and `PackageDocument` (consumer for quick-fix code actions). The `_validateWhenClauses` internal command invoked at `extensionLinter.ts:247` is an out-of-partition IPC call into the VS Code core. The `JsonStringScanner` in `jsonReconstruct.ts` bridges the gap between offset spaces: the `_validateWhenClauses` command reports errors in decoded-string offsets, and `JsonStringScanner.getOffsetInEncoded` translates those back to positions in the raw JSON document text. For a Tauri/Rust port, the entire language-service provider registration model (`registerCompletionItemProvider`, `registerDefinitionProvider`, `registerReferenceProvider`, `createDiagnosticCollection`, `createFileSystemWatcher`) would need equivalent LSP server-side implementations, and the `product.json` synchronous read at startup would need to be replaced by a Rust-side configuration mechanism. The `jsonc-parser` library (a pure JS parser) has no Rust equivalent bundled here and would need to be replaced with a Rust JSONC parser or the `serde_json` + comment-stripping approach. The `markdown-it` and `parse5` libraries are dynamically imported only in the desktop path, so their Rust equivalents (`pulldown-cmark`, `html5ever`) would be needed for the README linting feature.

---

### Out-of-Partition References

- **`_validateWhenClauses` command** — Invoked at `extensionLinter.ts:247` via `commands.executeCommand`. Implemented inside the VS Code core (`src/vs/workbench/` or `src/vs/platform/`), not in this partition. Returns `{ errorMessage, offset, length }[][]`.
- **`vscode://schemas/vscode-extensions`** — Referenced in `package.json:37`. The schema itself is generated by VS Code's extension host, likely in `src/vs/workbench/services/extensions/` or `src/vs/platform/jsonschemas/`.
- **`vscode://schemas/language-configuration`**, **`vscode://schemas/icon-theme`**, **`vscode://schemas/product-icon-theme`**, **`vscode://schemas/color-theme`** — All statically contributed schema URIs resolved by the JSON language service outside this partition.
- **`env.appRoot`** — VS Code runtime global used at `extensionLinter.ts:18` to locate `product.json`. Defined in the VS Code extension host runtime context (`src/vs/workbench/api/`).
- **`vscode.languages.getLanguages()`** — Called at `packageDocumentHelper.ts:73`. Implemented in the VS Code language service core.
- **`src/vs/platform/extensions/common/extensionValidator.ts`** — Acknowledged in a comment at `extensionEngineValidation.ts:6` as the upstream source for the version-parsing logic. The linter's engine-version gate at `extensionLinter.ts:165` mirrors logic that also exists in the platform validator.
- **`jsonc-parser` npm package (version `^3.2.0`)** — Used pervasively across `packageDocumentHelper.ts`, `extensionLinter.ts`, `packageDocumentL10nSupport.ts`. Sourced from `node_modules`; not in this partition.
- **`markdown-it` npm package (version `^12.3.2`)** — Dynamically imported at `extensionLinter.ts:292`. Types provided by `@types/markdown-it ^14` in devDependencies.
- **`parse5` npm package (version `^3.0.2`)** — Dynamically imported at `extensionLinter.ts:341`. Uses `SAXParser` with `locationInfo: true`.
