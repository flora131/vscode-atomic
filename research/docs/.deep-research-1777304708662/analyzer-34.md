# Analyzer 34: extensions/extension-editing/

## Partition 34 of 79 — 10 files, ~1,326 LOC

---

### Files Analysed

1. `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/extensionEditingMain.ts`
2. `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/extensionEditingBrowserMain.ts`
3. `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/packageDocumentHelper.ts`
4. `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/packageDocumentL10nSupport.ts`
5. `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/extensionLinter.ts`
6. `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/extensionEngineValidation.ts`
7. `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/jsonReconstruct.ts`
8. `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/constants.ts`

---

### Per-File Notes

#### `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/extensionEditingMain.ts`

- **Role:** Node.js (Electron) entry point for the `extension-editing` built-in extension. Wires up all three major features into `context.subscriptions` on activation.
- **Key symbols:**
  - `activate` (`extensionEditingMain.ts:11`) — exports the standard VS Code extension activation hook
  - `registerPackageDocumentCompletions` (`extensionEditingMain.ts:25`) — wraps `PackageDocument.provideCompletionItems` behind `vscode.languages.registerCompletionItemProvider`
  - `registerCodeActionsProvider` (`extensionEditingMain.ts:33`) — wraps `PackageDocument.provideCodeActions` behind `vscode.languages.registerCodeActionsProvider`
- **Control flow:** `activate` → pushes three subscriptions: (1) completion provider, (2) code action provider, both scoped to `{language:'json', pattern:'**/package.json'}`, and (3) instantiates `PackageDocumentL10nSupport` and `ExtensionLinter` which self-register their own event listeners in their constructors.
- **Data flow:** `vscode.TextDocument` + `vscode.Position` passed through the VS Code provider callbacks into `PackageDocument`, which is instantiated per-call (no caching).
- **Dependencies:** `vscode`, `./packageDocumentHelper`, `./packageDocumentL10nSupport`, `./extensionLinter`.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/extensionEditingBrowserMain.ts`

- **Role:** Browser (web worker) entry point. A reduced-feature subset of the Node entry: registers completions and L10n support but omits `ExtensionLinter` (which uses Node `fs` and `path`).
- **Key symbols:**
  - `activate` (`extensionEditingBrowserMain.ts:10`) — same signature as the Node entry but narrower
  - `registerPackageDocumentCompletions` (`extensionEditingBrowserMain.ts:18`) — identical implementation to the Node entry
- **Control flow:** `activate` → pushes two subscriptions: completion provider and `PackageDocumentL10nSupport`. No linting subscription.
- **Data flow:** Identical to Node entry for the two shared features.
- **Dependencies:** `vscode`, `./packageDocumentHelper`, `./packageDocumentL10nSupport`.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/packageDocumentHelper.ts`

- **Role:** Provides IntelliSense completions and quick-fix code actions over `package.json` files for VS Code extension authors. Specifically handles `configurationDefaults` language-override keys and activation event removal code actions.
- **Key symbols:**
  - `PackageDocument` (`packageDocumentHelper.ts:11`) — class constructed per provider invocation; holds `vscode.TextDocument`
  - `provideCompletionItems` (`packageDocumentHelper.ts:15`) — dispatches on JSON path depth/position to `provideLanguageOverridesCompletionItems`
  - `provideCodeActions` (`packageDocumentHelper.ts:25`) — iterates `context.diagnostics`, emits `WorkspaceEdit` delete actions for entries matching `implicitActivationEvent` or `redundantImplicitActivationEvent` message strings
  - `provideLanguageOverridesCompletionItems` (`packageDocumentHelper.ts:43`) — at path depth 2 returns a snippet for `"[language]": { ... }`; at depth 3 calls `vscode.languages.getLanguages()` to enumerate all registered language identifiers
  - `getReplaceRange` (`packageDocumentHelper.ts:85`) — derives the `vscode.Range` to replace from the JSON AST node's `offset` + `length`
  - `newSnippetCompletionItem` (`packageDocumentHelper.ts:105`) — wraps a `vscode.SnippetString` as a `CompletionItem`
- **Control flow:**
  1. Provider callback invokes `new PackageDocument(document).provideCompletionItems(position, token)`.
  2. `getLocation` (jsonc-parser) parses the full document text and returns the JSON path + `previousNode` at the cursor offset.
  3. Path `[*, 'configurationDefaults']` at depth 2 → snippet item; depth 3 with array-bracket prefix → language list from runtime.
  4. All other positions return `undefined` (no completions).
- **Data flow:** `document.getText()` → `jsonc-parser.getLocation` → JSON path array → branch to completion item list → `vscode.CompletionItem[]`.
- **Dependencies:** `vscode`, `jsonc-parser` (`getLocation`, `Location`), `./constants` (`implicitActivationEvent`, `redundantImplicitActivationEvent`).

---

#### `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/packageDocumentL10nSupport.ts`

- **Role:** Implements go-to-definition and find-all-references for NLS (National Language Support) placeholder strings in `package.json` / `package.nls.json` pairs. Supports bidirectional navigation: from a `%key%` value in `package.json` to the key declaration in `package.nls.json`, and vice versa.
- **Key symbols:**
  - `PackageDocumentL10nSupport` (`packageDocumentL10nSupport.ts:13`) — implements `vscode.DefinitionProvider`, `vscode.ReferenceProvider`, `vscode.Disposable`
  - `provideDefinition` (`packageDocumentL10nSupport.ts:31`) — dispatches by filename to `provideNlsValueDefinition` (from `package.json`) or `provideNlsKeyDefinition` (from `package.nls.json`)
  - `getNlsReferenceAtPosition` (`packageDocumentL10nSupport.ts:75`) — uses `getLocation`/`getNodeValue` to read the string at cursor; applies regex `/^%(.+)%$/` to extract the NLS key
  - `resolveNlsDefinition` (`packageDocumentL10nSupport.ts:62`) — calls `findNlsKeyDeclaration` then wraps result in `DefinitionLink` with `originSelectionRange` + `targetRange`
  - `findNlsKeyDeclaration` (`packageDocumentL10nSupport.ts:140`) — opens `package.nls.json` via `vscode.workspace.openTextDocument`, parses tree, calls `findNodeAtLocation(tree, [nlsKey])`, extracts the key node's byte range
  - `findNlsReferencesInPackageJson` (`packageDocumentL10nSupport.ts:166`) — uses `jsonc-parser.visit` with `onLiteralValue` callback to scan all string values matching `%key%`
  - `getNlsKeyDefinitionAtPosition` (`packageDocumentL10nSupport.ts:191`) — requires `location.path.length === 1` and `location.isAtPropertyKey` to confirm cursor is on a top-level key in `package.nls.json`
- **Control flow (definition from package.json):**
  1. `provideDefinition` → `provideNlsValueDefinition` → `getNlsReferenceAtPosition` (regex match) → `resolveNlsDefinition` → `findNlsKeyDeclaration` (opens nls file, AST lookup) → returns `DefinitionLink[]`.
- **Control flow (references from package.nls.json):**
  1. `provideReferences` → `provideNlsKeyReferences` → `getNlsKeyDefinitionAtPosition` → `findAllNlsReferences` → `findNlsReferencesInPackageJson` (visitor scan) + optionally `findNlsKeyDeclaration`.
- **Data flow:** `TextDocument.getText()` → jsonc-parser AST → NLS key string → `Uri.joinPath` to compute sibling file URI → `workspace.openTextDocument` → second AST traversal → `vscode.Location[]` / `vscode.DefinitionLink[]`.
- **Dependencies:** `vscode`, `jsonc-parser` (`getLocation`, `getNodeValue`, `parseTree`, `findNodeAtLocation`, `visit`).

---

#### `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/extensionLinter.ts`

- **Role:** A `DiagnosticCollection`-based linter that validates `package.json` extension manifests and `README.md`/`CHANGELOG.md` markdown files. Runs asynchronously on a 300 ms debounce timer. Produces diagnostics for image URL hygiene, API proposal validity, activation event correctness, and `when`-clause parse errors.
- **Key symbols:**
  - `ExtensionLinter` (`extensionLinter.ts:60`) — main class; constructed once and self-manages listeners
  - `diagnosticsCollection` (`extensionLinter.ts:62`) — `languages.createDiagnosticCollection('extension-editing')`
  - `fileWatcher` (`extensionLinter.ts:63`) — `workspace.createFileSystemWatcher('**/package.json')` watches external disk changes
  - `packageJsonQ` / `readmeQ` (`extensionLinter.ts:67-68`) — `Set<TextDocument>` queues drained on each timer tick
  - `startTimer` (`extensionLinter.ts:102`) — 300 ms debounce; clears existing timer before rescheduling
  - `lintPackageJson` (`extensionLinter.ts:119`) — drains `packageJsonQ`; parses with `jsonc-parser.parseTree`; checks icon/badge URLs, `enabledApiProposals`, activation events, and delegates to `lintWhenClauses`
  - `lintWhenClauses` (`extensionLinter.ts:204`) — recursively walks the `contributes` AST collecting all `when`/`enablement` string values; calls the internal command `_validateWhenClauses` via `commands.executeCommand`; uses `JsonStringScanner` to map decoded offsets back to encoded JSON positions
  - `lintReadme` (`extensionLinter.ts:272`) — dynamically imports `markdown-it` and `parse5`; parses markdown tokens; scans `img.src` attributes and inline HTML `<svg>` / `<img>` tags for URL violations
  - `readPackageJsonInfo` (`extensionLinter.ts:394`) — extracts engine version, repository URL, and implicit activation event set from the AST; caches per folder in `folderToPackageJsonInfo`
  - `parseImplicitActivationEvents` (`extensionLinter.ts:500`) — standalone function; traverses `contributes.*` nodes to derive the full set of activation events that VS Code generates automatically (commands, auth providers, languages, custom editors, views, walkthroughs, notebook renderers, terminal profiles/quickfixes, task definitions)
  - `addDiagnostics` (`extensionLinter.ts:441`) — shared URL validator; checks HTTPS requirement, SVG source validity, data URL prohibition, relative URL constraints; uses the `isTrustedSVGSource` function against `product.json` allow-lists
  - `isTrustedSVGSource` (`extensionLinter.ts:25`) — tests `uri.authority` against `allowedBadgeProviders` (lowercase strings) and `allowedBadgeProvidersRegex` patterns loaded from `product.json` at module load
- **Control flow:**
  1. Constructor registers five event listeners (`onDidOpenTextDocument`, `onDidChangeTextDocument`, `onDidCloseTextDocument`, file-watcher `onChange/Create/Delete`) and immediately queues all currently open documents (`extensionLinter.ts:82`).
  2. Each event calls `queue(document)` → adds to `packageJsonQ` or `readmeQ` → `startTimer` (debounce 300 ms).
  3. `lint()` fires: `lintPackageJson` and `lintReadme` run in parallel via `Promise.all`.
  4. `lintPackageJson` for each document: parse tree → `readPackageJsonInfo` → check icon, badges, `enabledApiProposals`, activation events → `lintWhenClauses` → `diagnosticsCollection.set`.
  5. `lintWhenClauses`: collect `when`/`enablement` nodes → `commands.executeCommand('_validateWhenClauses', ...)` → map errors through `JsonStringScanner.getOffsetInEncoded` → `Diagnostic[]`.
  6. `lintReadme`: dynamic import `markdown-it` → parse → token walk + `parse5.SAXParser` for inline HTML → `addDiagnostics` for each image src.
- **Data flow:**
  - `product.json` (read synchronously at module load, `extensionLinter.ts:18`) → `allowedBadgeProviders`, `allowedBadgeProvidersRegex`, `extensionEnabledApiProposals`.
  - `TextDocument.getText()` → `parseTree` → `JsonNode` tree → field-specific `findNodeAtLocation` calls → `Diagnostic` objects → `diagnosticsCollection.set(uri, diagnostics)`.
  - `when`-clause strings extracted as `string[]` → IPC to `_validateWhenClauses` command (core) → `{errorMessage, offset, length}[][]` → decoded-to-encoded offset mapping via `JsonStringScanner` → `Diagnostic[]`.
- **Dependencies:** `path`, `fs`, `url.URL` (Node built-ins), `jsonc-parser` (`parseTree`, `findNodeAtLocation`, `getNodeValue`), `markdown-it` (dynamic import), `parse5` (dynamic import), `vscode` (commands, languages, workspace, Disposable, TextDocument, Uri, Diagnostic, Range, DiagnosticSeverity, Position, env, l10n), `./extensionEngineValidation` (`normalizeVersion`, `parseVersion`), `./jsonReconstruct` (`JsonStringScanner`), `./constants`.

---

#### `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/extensionEngineValidation.ts`

- **Role:** Pure utility module for parsing and normalizing VS Code engine version strings (semver with optional `^`/`>=` prefix and `x` wildcards). Mirrors the implementation in core at `src/vs/platform/extensions/common/extensionValidator.ts` (per the comment at line 6).
- **Key symbols:**
  - `IParsedVersion` (`extensionEngineValidation.ts:8`) — interface with `hasCaret`, `hasGreaterEquals`, `majorBase/minorBase/patchBase` (numeric), `*MustEqual` (boolean flags), `preRelease` (string|null)
  - `INormalizedVersion` (`extensionEngineValidation.ts:20`) — interface adds `notBefore` (UTC timestamp ms from `-YYYYMMDD` preRelease), `isMinimum` (from `>=`)
  - `VERSION_REGEXP` (`extensionEngineValidation.ts:31`) — `/^(\^|>=)?((\d+)|x)\.((\d+)|x)\.((\d+)|x)(\-.*)?$/`
  - `NOT_BEFORE_REGEXP` (`extensionEngineValidation.ts:32`) — `/^-(\d{4})(\d{2})(\d{2})$/` for the date suffix
  - `isValidVersionStr` (`extensionEngineValidation.ts:34`) — returns `true` for `'*'` or `VERSION_REGEXP` match
  - `parseVersion` (`extensionEngineValidation.ts:39`) — converts version string to `IParsedVersion`; `'*'` maps to all-zero/all-false
  - `normalizeVersion` (`extensionEngineValidation.ts:77`) — converts `IParsedVersion` to `INormalizedVersion`; caret semantics: major=0 → only `patchMustEqual=false`, major>0 → both `minor/patchMustEqual=false`; extracts `notBefore` from `preRelease` using `Date.UTC`
- **Control flow:** `parseVersion` → `VERSION_REGEXP.match` → struct construction; then `normalizeVersion` → adjusts `MustEqual` flags per caret rule → constructs `INormalizedVersion`.
- **Data flow:** Raw engine string (e.g. `"^1.75.0"`) → `IParsedVersion` (parse) → `INormalizedVersion` (normalize) → consumed by `ExtensionLinter.readPackageJsonInfo` to compare against `>=1.75` threshold for implicit activation event support.
- **Dependencies:** None (no imports).

---

#### `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/jsonReconstruct.ts`

- **Role:** A single-purpose scanner class that converts a character offset within a decoded JSON string value (i.e., the unescaped, parsed string) back to the corresponding byte offset within the encoded JSON source text (i.e., the raw `"..."` literal with escape sequences). Used to accurately position `when`-clause parse error markers within the JSON source.
- **Key symbols:**
  - `JsonStringScanner` (`jsonReconstruct.ts:11`) — class; stateful scanner holding `pos` (current position in encoded text) and `resultChars` (count of decoded characters consumed)
  - `constructor` (`jsonReconstruct.ts:20`) — receives full document `text` and `initialPos` (must be `stringJSONNode.offset + 1`, i.e., just past the opening `"`)
  - `getOffsetInEncoded` (`jsonReconstruct.ts:25`) — advances through the encoded text, accounting for escape sequences (`\n`, `\t`, `\uXXXX`, etc.), until `resultChars > offsetDecoded`; returns `start` (the encoded position just before the target decoded character)
  - `scanHexDigits` (`jsonReconstruct.ts:68`) — scans 4 hex digits for `\uXXXX` sequences; multiplies each digit into `value` using base-16 arithmetic; contributes `String.fromCharCode(value).length` decoded chars (handles surrogate pairs)
  - `CharacterCodes` (`jsonReconstruct.ts:96`) — `const enum` mapping JSON-significant characters to their Unicode code points
- **Control flow:** `getOffsetInEncoded(offsetDecoded)` loops character by character through the encoded string. On `\` (backslash), it reads the next escape character and increments `resultChars` by the number of decoded characters the escape represents (always 1 except `\uXXXX` which may be 1 or 2 for surrogates). On plain characters, increments both `pos` and `resultChars` by 1. Returns `start` (the last recorded start position) when `resultChars` exceeds `offsetDecoded`.
- **Data flow:** Encoded JSON source `string` (full document text) + `initialPos` → stateful scan → returns encoded offset `number` for each decoded offset queried.
- **Dependencies:** None (no imports).

---

#### `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/constants.ts`

- **Role:** Declares two shared diagnostic message strings used as sentinel values to match diagnostics in both the linter (where they are emitted) and the code action provider (where they trigger quick-fix generation).
- **Key symbols:**
  - `implicitActivationEvent` (`constants.ts:8`) — l10n string: `"This activation event cannot be explicitly listed by your extension."` (reserved prefix events like `onNotebookSerializer:`)
  - `redundantImplicitActivationEvent` (`constants.ts:9`) — l10n string: `"This activation event can be removed as VS Code generates these automatically from your package.json contribution declarations."`
- **Control flow:** Module-level constants; no logic.
- **Data flow:** Imported by both `extensionLinter.ts` (to emit diagnostics with these messages) and `packageDocumentHelper.ts` (to match `diagnostic.message` in `provideCodeActions`). The string identity is the coupling mechanism.
- **Dependencies:** `vscode` (`l10n`).

---

### Cross-Cutting Synthesis

The `extension-editing` partition is a self-contained VS Code built-in that improves the developer experience when authoring extensions. All eight source files form two layers: an entry/wiring layer (`extensionEditingMain.ts`, `extensionEditingBrowserMain.ts`) and an implementation layer (the remaining six files). The implementation layer separates concerns cleanly: completion suggestions live in `packageDocumentHelper.ts`; NLS navigation lives in `packageDocumentL10nSupport.ts`; diagnostic linting lives in `extensionLinter.ts`; and three pure utilities (`extensionEngineValidation.ts`, `jsonReconstruct.ts`, `constants.ts`) serve the linter without cross-contaminating other features.

The diagnostic pipeline in `ExtensionLinter` is the most complex path: it debounces document events into two queued sets, drains them asynchronously, performs jsonc-parser AST analysis, delegates `when`-clause validation to a core internal command (`_validateWhenClauses`) via IPC, and uses `JsonStringScanner` to translate error offsets from decoded string space back to encoded JSON source positions. The constants module bridges the linter and the code action provider through shared diagnostic message string identity rather than diagnostic codes.

The browser entry point omits `ExtensionLinter` entirely because that class synchronously reads `product.json` using Node's `fs` at module load and later uses `fs`/`path` for file resolution — none of which are available in a browser worker context.

---

### Out-of-Partition References

- **`_validateWhenClauses` internal command** — invoked at `extensionLinter.ts:247` via `commands.executeCommand`; implemented in core (`src/vs/workbench/` or `src/vs/platform/`), outside this partition. Returns `{errorMessage, offset, length}[][]`.
- **`product.json`** — read synchronously at `extensionLinter.ts:18` from `env.appRoot`; its `extensionAllowedBadgeProviders`, `extensionAllowedBadgeProvidersRegex`, and `extensionEnabledApiProposals` fields directly gate diagnostic emission.
- **`src/vs/platform/extensions/common/extensionValidator.ts`** — the canonical implementation that `extensionEngineValidation.ts` explicitly mirrors (comment at line 6); version parsing logic is duplicated here to avoid a cross-package dependency.
- **`jsonc-parser` (npm package)** — used by `packageDocumentHelper.ts`, `packageDocumentL10nSupport.ts`, and `extensionLinter.ts` for all JSON AST operations (`parseTree`, `findNodeAtLocation`, `getLocation`, `getNodeValue`, `visit`).
- **`markdown-it` (npm package)** — dynamically imported in `extensionLinter.ts:291` for README markdown token parsing.
- **`parse5` (npm package)** — dynamically imported in `extensionLinter.ts:341` for SAX-style HTML parsing of inline `<img>` and `<svg>` tags within markdown text tokens.
