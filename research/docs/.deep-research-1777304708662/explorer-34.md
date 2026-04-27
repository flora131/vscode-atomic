# Partition 34 of 79 — Findings

## Scope
`extensions/extension-editing/` (10 files, 1,326 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locations for Extension Editing Extension

## Overview
The `extension-editing` extension provides IDE support for extension developers authoring VS Code extensions. It includes schema validation for package.json, completion providers, code actions, and localization support. This extension demonstrates several core VS Code API patterns relevant to understanding how IDE features are implemented.

## Implementation Files

- `extensions/extension-editing/src/extensionEditingMain.ts` - Main entry point for Node.js platform; registers completion item provider and code actions provider for package.json files
- `extensions/extension-editing/src/extensionEditingBrowserMain.ts` - Browser/web platform variant; stripped down version that registers only completion items and L10n support (no code actions)
- `extensions/extension-editing/src/packageDocumentHelper.ts` - Core helper class for package.json document analysis; implements completion item and code action providers for language overrides and activation events
- `extensions/extension-editing/src/packageDocumentL10nSupport.ts` - Implements definition and reference providers for NLS localization mappings between package.json and package.nls.json files
- `extensions/extension-editing/src/extensionLinter.ts` - Linting engine that validates package.json and README/CHANGELOG.md files; manages diagnostics collection, file watching, and validation queue
- `extensions/extension-editing/src/extensionEngineValidation.ts` - Version parsing and validation utilities; defines IParsedVersion and INormalizedVersion interfaces for engine version constraints
- `extensions/extension-editing/src/jsonReconstruct.ts` - JsonStringScanner utility class for converting offsets within decoded JSON strings to offsets in encoded strings; handles escape sequence processing

## Configuration Files

- `extensions/extension-editing/package.json` - Extension manifest defining contribution points (jsonValidation schemas for package.json, language-configuration.json, icon-theme.json, color-theme.json); activation events (onLanguage:json, onLanguage:markdown); dependencies (jsonc-parser, markdown-it, parse5)
- `extensions/extension-editing/package.nls.json` - Localization strings for display name and description
- `extensions/extension-editing/tsconfig.json` - TypeScript compiler configuration extending ../tsconfig.base.json; compiles src/ to out/ directory
- `extensions/extension-editing/tsconfig.browser.json` - Browser-specific TypeScript configuration extending tsconfig.json; excludes tests and compiles only extensionEditingBrowserMain.ts
- `extensions/extension-editing/esbuild.mts` - esbuild configuration for Node.js platform build; entry point: extensionEditingMain.ts
- `extensions/extension-editing/esbuild.browser.mts` - esbuild configuration for browser platform build; entry point: extensionEditingBrowserMain.ts; outputs to dist/browser/
- `extensions/extension-editing/.npmrc` - NPM configuration file
- `extensions/extension-editing/.vscodeignore` - Excludes files from packaged extension

## Type Definitions / Interfaces

Notable type definitions exported by the extension:

**From extensionEngineValidation.ts:**
- `IParsedVersion` - Interface representing parsed version string components (hasCaret, hasGreaterEquals, majorBase/minorBase/patchBase, mustEqual flags, preRelease)
- `INormalizedVersion` - Interface representing normalized version constraints (majorBase/minorBase/patchBase, mustEqual flags, notBefore timestamp, isMinimum flag)

**From packageDocumentL10nSupport.ts:**
- Class implements `vscode.DefinitionProvider` and `vscode.ReferenceProvider` interfaces
- Uses `vscode.DocumentSelector` type for matching package.json and package.nls.json

**From extensionLinter.ts:**
- `PackageJsonInfo` - Internal interface tracking extension folder state (isExtension, hasHttpsRepository, implicitActivationEvents, engineVersion)
- `TokenAndPosition` - Internal interface for markdown token position tracking
- Uses `vscode.DiagnosticCollection`, `vscode.Diagnostic` types

## Documentation

- `extensions/extension-editing/images/icon.png` - Visual icon for the extension

## Notable Clusters and Architecture Patterns

**Language Provider Registration Pattern:**
The extension demonstrates the standard VS Code pattern for registering language service providers:
- `vscode.languages.registerCompletionItemProvider()` - Registers completion items for package.json fields
- `vscode.languages.registerCodeActionsProvider()` - Registers code actions for diagnostic remediation
- `vscode.languages.registerDefinitionProvider()` - Maps NLS references to their definitions
- `vscode.languages.registerReferenceProvider()` - Finds all references to NLS keys
- `vscode.languages.createDiagnosticCollection()` - Creates diagnostic collection for linting
- `vscode.languages.getLanguages()` - Retrieves available language identifiers for completion

All provider registrations use document selector patterns: `{ language: 'json', pattern: '**/package.json' }` and `{ language: 'markdown', pattern: '**.md' }`

**Dual Platform Support:**
- Node.js variant (extensionEditingMain.ts) includes full linting, validation, code actions
- Browser variant (extensionEditingBrowserMain.ts) includes only completion and NLS support
- Separate esbuild configurations and tsconfig files manage platform-specific builds
- Both platforms activate on onLanguage:json and onLanguage:markdown events

**File Watching and Queueing:**
The ExtensionLinter implements a deferred validation pattern:
- Watches package.json files via `workspace.createFileSystemWatcher()`
- Queues documents for processing on open/change/delete events
- Uses timer-based batching to avoid excessive validation passes
- Maintains separate queues for package.json and markdown documents

**Dependency Management:**
- `jsonc-parser` - Parses JSON with comments and provides location/node utilities
- `markdown-it` - Parses and validates markdown (README/CHANGELOG)
- `parse5` - HTML parser for extracting/validating image URLs in markdown

## Entry Points

- `extensions/extension-editing/src/extensionEditingMain.ts:11` - activate() function called on extension load; registers core providers
- `extensions/extension-editing/src/extensionEditingBrowserMain.ts:10` - activate() function for browser platform
- `extensions/extension-editing/package.json:16,17` - Main entry points defined: "./out/extensionEditingMain" (Node.js) and "./dist/browser/extensionEditingBrowserMain" (browser)
- `extensions/extension-editing/package.json:12,13` - Activation events: onLanguage:json and onLanguage:markdown

---

## Research Context: Tauri/Rust Porting Implications

This extension provides critical IDE capabilities for extension authoring workflows. Porting these features to Tauri/Rust would require:

1. **Language Service Server Protocol (LSP) Integration** - The completion, hover, definition, and reference providers use VS Code's native language extension API. A Tauri/Rust implementation would need to establish LSP-compliant servers or equivalent protocol handlers.

2. **JSON Schema Validation Subsystem** - The jsonValidation contribution points validate against vscode://schemas/* URLs. Tauri would need equivalent schema resolution and validation machinery, likely via JSON Schema libraries in Rust (e.g., jsonschema crate).

3. **Document Synchronization** - The file watcher and document queue system synchronizes state across textDocument events. Tauri would need equivalent document lifecycle management integrated with any LSP server implementation.

4. **Diagnostic Collection and Presentation** - The DiagnosticCollection API pushes diagnostics to the editor. Tauri would need a similar mechanism to communicate validation errors back to the UI layer.

5. **Platform-Specific Builds** - The dual Node.js/Browser builds reveal VS Code's architecture challenge. A Tauri port would likely be desktop-only but would need equivalent capability detection and feature gating.

6. **Localization Infrastructure** - The package.nls.json integration and L10n support requires tight coupling with VS Code's localization system. Tauri would need equivalent i18n infrastructure for extension catalog presentation.

The extension serves as a lens into VS Code's extension authoring story—a critical competitive advantage for IDE adoption and ecosystem health. Porting this single extension touches nearly all major subsystems: document parsing, schema validation, language services, file I/O, diagnostic presentation, and localization.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: VS Code IDE Functionality Port to Tauri/Rust
## Scope: `extensions/extension-editing/` — Language Provider Registration & Contributions

**Seed Query**: `ast-grep --lang ts -p 'languages.registerHoverProvider($$$)'`

**Focus**: Language service providers, diagnostic collection, file watching, and package.json contribution schema handling within the extension-editing subsystem.

---

## Pattern 1: Language Provider Registration Pattern

**Found in**: `extensions/extension-editing/src/extensionEditingMain.ts:25-31`
**Used for**: Completion item provider registration for package.json files

```typescript
function registerPackageDocumentCompletions(): vscode.Disposable {
	return vscode.languages.registerCompletionItemProvider({ language: 'json', pattern: '**/package.json' }, {
		provideCompletionItems(document, position, token) {
			return new PackageDocument(document).provideCompletionItems(position, token);
		}
	});
}
```

**Key aspects**:
- Registers provider for specific file pattern (`**/package.json`)
- Returns `vscode.Disposable` for cleanup/subscription
- Provider object implements `provideCompletionItems(document, position, token)` method
- Instantiates provider class with document context
- Pattern uses document selector with language ID + glob pattern

**Variations found**: Also used for code actions provider (line 33-39), completion items provider (browser variant at extensionEditingBrowserMain.ts:18-24)

---

## Pattern 2: Code Action Provider with Diagnostic Context

**Found in**: `extensions/extension-editing/src/extensionEditingMain.ts:33-39`
**Used for**: Providing quick-fix code actions for linting diagnostics

```typescript
function registerCodeActionsProvider(): vscode.Disposable {
	return vscode.languages.registerCodeActionsProvider({ language: 'json', pattern: '**/package.json' }, {
		provideCodeActions(document, range, context, token) {
			return new PackageDocument(document).provideCodeActions(range, context, token);
		}
	});
}
```

**Key aspects**:
- Receives `CodeActionContext` containing diagnostics
- Operates on range-based scope within document
- Handles code action generation as `CodeAction[]`
- Disposal pattern for lifecycle management
- Bound to package.json files only

---

## Pattern 3: Multiple Language Provider Registration (Definition & Reference)

**Found in**: `extensions/extension-editing/src/packageDocumentL10nSupport.ts:13-30`
**Used for**: Localization string definition/reference resolution in package.json and package.nls.json

```typescript
export class PackageDocumentL10nSupport implements vscode.DefinitionProvider, vscode.ReferenceProvider, vscode.Disposable {

	private readonly _disposables: vscode.Disposable[] = [];

	constructor() {
		this._disposables.push(vscode.languages.registerDefinitionProvider(packageJsonSelector, this));
		this._disposables.push(vscode.languages.registerDefinitionProvider(packageNlsJsonSelector, this));

		this._disposables.push(vscode.languages.registerReferenceProvider(packageNlsJsonSelector, this));
		this._disposables.push(vscode.languages.registerReferenceProvider(packageJsonSelector, this));
	}

	dispose(): void {
		for (const d of this._disposables) {
			d.dispose();
		}
	}
```

**Key aspects**:
- Single class implements multiple provider interfaces (`DefinitionProvider`, `ReferenceProvider`)
- Registers provider for multiple document selectors (two files: package.json, package.nls.json)
- Centralized disposable management array
- Explicit `dispose()` method for cleanup
- Class instance passed as provider (`this`)

---

## Pattern 4: Diagnostic Collection with File System Watcher

**Found in**: `extensions/extension-editing/src/extensionLinter.ts:60-84`
**Used for**: Real-time linting of extension package.json with file watching and document lifecycle management

```typescript
export class ExtensionLinter {

	private diagnosticsCollection = languages.createDiagnosticCollection('extension-editing');
	private fileWatcher = workspace.createFileSystemWatcher('**/package.json');
	private disposables: Disposable[] = [this.diagnosticsCollection, this.fileWatcher];

	constructor() {
		this.disposables.push(
			workspace.onDidOpenTextDocument(document => this.queue(document)),
			workspace.onDidChangeTextDocument(event => this.queue(event.document)),
			workspace.onDidCloseTextDocument(document => this.clear(document)),
			this.fileWatcher.onDidChange(uri => this.packageJsonChanged(this.getUriFolder(uri))),
			this.fileWatcher.onDidCreate(uri => this.packageJsonChanged(this.getUriFolder(uri))),
			this.fileWatcher.onDidDelete(uri => this.packageJsonChanged(this.getUriFolder(uri))),
		);
		workspace.textDocuments.forEach(document => this.queue(document));
	}
```

**Key aspects**:
- `languages.createDiagnosticCollection()` for managing diagnostic issues
- `workspace.createFileSystemWatcher()` monitors file changes matching glob pattern
- Hooks into document lifecycle events: open, change, close
- File watcher provides three events: change, create, delete
- Debounced processing with internal queue (`this.queue()`)
- Iterates existing documents on initialization
- All disposables collected in array for cleanup

---

## Pattern 5: Completion Item and Snippet Generation

**Found in**: `extensions/extension-editing/src/packageDocumentHelper.ts:96-112`
**Used for**: Creating context-aware completion items with snippets for package.json contributions

```typescript
private newSimpleCompletionItem(text: string, range: vscode.Range, description?: string, insertText?: string): vscode.CompletionItem {
	const item = new vscode.CompletionItem(text);
	item.kind = vscode.CompletionItemKind.Value;
	item.detail = description;
	item.insertText = insertText ? insertText : text;
	item.range = range;
	return item;
}

private newSnippetCompletionItem(o: { label: string; documentation?: string; snippet: string; range: vscode.Range }): vscode.CompletionItem {
	const item = new vscode.CompletionItem(o.label);
	item.kind = vscode.CompletionItemKind.Value;
	item.documentation = o.documentation;
	item.insertText = new vscode.SnippetString(o.snippet);
	item.range = o.range;
	return item;
}
```

**Key aspects**:
- Two variants: simple text completion and snippet completion
- `CompletionItemKind.Value` for JSON value context
- Replace range specified per item
- Snippet strings use `vscode.SnippetString` with placeholder syntax (`$1`, `$0`)
- Optional documentation for extended help

---

## Pattern 6: Version Parsing and Normalization with Regex

**Found in**: `extensions/extension-editing/src/extensionEngineValidation.ts:31-75`
**Used for**: Parsing semantic version strings in engine requirement specifications

```typescript
const VERSION_REGEXP = /^(\^|>=)?((\d+)|x)\.((\d+)|x)\.((\d+)|x)(\-.*)?$/;
const NOT_BEFORE_REGEXP = /^-(\d{4})(\d{2})(\d{2})$/;

export function parseVersion(version: string): IParsedVersion | null {
	if (!isValidVersionStr(version)) {
		return null;
	}

	version = version.trim();

	if (version === '*') {
		return {
			hasCaret: false,
			hasGreaterEquals: false,
			majorBase: 0,
			majorMustEqual: false,
			minorBase: 0,
			minorMustEqual: false,
			patchBase: 0,
			patchMustEqual: false,
			preRelease: null
		};
	}

	const m = version.match(VERSION_REGEXP);
	if (!m) {
		return null;
	}
	return {
		hasCaret: m[1] === '^',
		hasGreaterEquals: m[1] === '>=',
		majorBase: m[2] === 'x' ? 0 : parseInt(m[2], 10),
		majorMustEqual: (m[2] === 'x' ? false : true),
		minorBase: m[4] === 'x' ? 0 : parseInt(m[4], 10),
		minorMustEqual: (m[4] === 'x' ? false : true),
		patchBase: m[6] === 'x' ? 0 : parseInt(m[6], 10),
		patchMustEqual: (m[6] === 'x' ? false : true),
		preRelease: m[8] || null
	};
}
```

**Key aspects**:
- Regex captures version components with semantic interpretation
- Supports caret (`^`) and greater-equals (`>=`) operators
- Handles `x` wildcard for version components
- Date-based pre-release version parsing with `NOT_BEFORE_REGEXP`
- Structured output with must-equal flags for range validation
- Special case for `*` (any version)

---

## Pattern 7: Implicit Activation Event Detection and Linting

**Found in**: `extensions/extension-editing/src/extensionLinter.ts:500-598`
**Used for**: Parsing extension contributions to generate implicit activation events

```typescript
function parseImplicitActivationEvents(tree: JsonNode): Set<string> {
	const activationEvents = new Set<string>();

	// commands
	const commands = findNodeAtLocation(tree, ['contributes', 'commands']);
	commands?.children?.forEach(child => {
		const command = findNodeAtLocation(child, ['command']);
		if (command && command.type === 'string') {
			activationEvents.add(`onCommand:${command.value}`);
		}
	});

	// authenticationProviders
	const authenticationProviders = findNodeAtLocation(tree, ['contributes', 'authentication']);
	authenticationProviders?.children?.forEach(child => {
		const id = findNodeAtLocation(child, ['id']);
		if (id && id.type === 'string') {
			activationEvents.add(`onAuthenticationRequest:${id.value}`);
		}
	});

	// languages
	const languageContributions = findNodeAtLocation(tree, ['contributes', 'languages']);
	languageContributions?.children?.forEach(child => {
		const id = findNodeAtLocation(child, ['id']);
		const configuration = findNodeAtLocation(child, ['configuration']);
		if (id && id.type === 'string' && configuration && configuration.type === 'string') {
			activationEvents.add(`onLanguage:${id.value}`);
		}
	});
	
	// [Additional types: customEditors, views, walkthroughs, notebookRenderer, terminalProfiles, terminalQuickFixes, tasks]
	
	return activationEvents;
}
```

**Key aspects**:
- Traverses JSON tree using `findNodeAtLocation()` 
- Multiple contribution types mapped to activation event prefixes
- Set-based deduplication of events
- Conditional checks for node type and required fields
- Pattern: each contribution type has specific id/identifier field
- Returns complete set for validation against explicit declarations

---

## Porting Implications: Key Architectural Patterns

### Language Service Provider Architecture
VS Code uses a **plugin-based language provider system** requiring:
1. Document selector pattern matching (`language` + glob `pattern`)
2. Provider interface implementation (completions, code actions, definitions, references)
3. Async/Promise-based result returns with cancellation tokens
4. Disposable pattern for lifecycle management

### Document & File Watching
The extension-editing subsystem relies on:
1. **Document lifecycle events** (open, change, close) with debounced processing
2. **File system watchers** for non-open file detection
3. **In-memory caching** of diagnostic state per folder
4. **Lazy loading** of markdown and parse5 dependencies

### JSON/Manifest Processing
Core patterns for extension package.json handling:
1. **JSONC parsing** with `jsonc-parser` (preserves comments)
2. **Path-based navigation** in JSON trees (`['contributes', 'commands']`)
3. **Offset-to-position mapping** for precise diagnostic ranges
4. **String encoding reconstruction** for escaped JSON values

### Type System and Validation
Multiple validation layers:
1. **Version string parsing** with regex and semantic flags
2. **Implicit activation event generation** from contribution declarations
3. **Multi-file relationship** management (package.json ↔ package.nls.json)
4. **DiagnosticSeverity levels** (Error, Warning, Information)

---

## Critical File References

- `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/extensionEditingMain.ts` - Entry point and provider registration
- `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/extensionLinter.ts` - Linting engine with diagnostics
- `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/packageDocumentHelper.ts` - Completion and code action providers
- `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/packageDocumentL10nSupport.ts` - Definition/reference providers
- `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/extensionEngineValidation.ts` - Version parsing logic
- `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/jsonReconstruct.ts` - JSON string offset handling

---

## Summary

The extension-editing subsystem demonstrates VS Code's **extensible language services architecture** with clear separation of concerns:

1. **Language Providers** register for specific document patterns
2. **Diagnostic Collection** manages all linting issues with real-time updates
3. **File Watching** maintains consistency with filesystem state
4. **JSON Navigation** uses structured AST walking for manifest parsing
5. **Type Validation** ensures version strings and contributions match schema

For a Tauri/Rust port, this would require:
- LSP (Language Server Protocol) implementation for provider equivalents
- Async event handling matching document lifecycle
- Regex-based parsing for version validation
- JSON tree navigation library (serde_json with path resolution)
- Disposable/subscription pattern equivalent in Rust (Arc<Mutex<>> or channel-based)

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
