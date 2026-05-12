### Files Analysed

1. `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/src/configurationEditingMain.ts` (245 LOC)
2. `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/src/settingsDocumentHelper.ts` (363 LOC)
3. `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/src/extensionsProposals.ts` (32 LOC)
4. `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/src/importExportProfiles.ts` (81 LOC)
5. `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/src/node/net.ts` (29 LOC)
6. `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/src/browser/net.ts` (6 LOC)
7. `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/src/test/completion.test.ts` (595 LOC)
8. `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/package.json`
9. `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/esbuild.browser.mts`

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/src/configurationEditingMain.ts`

- **Role:** Extension entry point. Registers all completion providers and a document symbol provider for VS Code's JSON configuration files (`settings.json`, `extensions.json`, `launch.json`, `tasks.json`, `*.code-workspace`, `keybindings.json`, `package.json`).
- **Key symbols:**
  - `activate` (line 12): Extension lifecycle hook; subscribes all providers to `context.subscriptions`.
  - `registerSettingsCompletions` (line 32): Registers a `CompletionItemProvider` for `{ language: 'jsonc', pattern: '**/settings.json' }`, delegating to `SettingsDocument.provideCompletionItems`.
  - `registerVariableCompletions` (line 40): Generic provider for `${…}` variable completions in `launch.json`, `tasks.json`, and `*.code-workspace`. Uses `getLocation` from `jsonc-parser` to determine cursor position, then returns a hard-coded list of 18 VS Code variable labels (lines 54–77).
  - `isCompletingInsidePropertyStringValue` (line 85): Helper; returns true when cursor offset falls strictly inside the span of a string-type `previousNode`.
  - `isLocationInsideTopLevelProperty` (line 97): Checks `location.path[0]` against an allowlist.
  - `registerExtensionsCompletions` (line 105): Returns two disposables — one for `**/extensions.json` (checks `path[0] === 'recommendations'`) and one for `**/*.code-workspace` (checks `path[0] === 'extensions' && path[1] === 'recommendations'`).
  - `getReplaceRange` (line 137): Utility to compute a `vscode.Range` spanning the token under the cursor, falling back to a zero-length range at position.
  - Document symbol provider (line 148–181): Registered at module top level (not inside `activate`); uses `visit` to walk the `launch.json` AST and emits `SymbolInformation` for every depth-2 object that has a `name` literal property.
  - `registerContextKeyCompletions` (line 183): Registers completions in `keybindings.json` (`['*','when']` path) and `package.json` (`contributes.menus|views|viewsWelcome|keybindings.*.when` paths). At line 227 it executes the internal command `getContextKeyInfo` to obtain the live list of registered context keys, then builds a `CompletionList` from the result.
- **Control flow:** `activate` → registers providers sequentially. Each provider's `provideCompletionItems` fires on editor events: it calls `getLocation(document.getText(), document.offsetAt(position))` from `jsonc-parser`, then dispatches on `location.path[0]` or other path segments.
- **Data flow:** Raw document text → `getLocation` → `Location` object (path, previousNode, isAtPropertyKey) → provider-specific branch → `vscode.CompletionItem[]`.
- **Dependencies:** `jsonc-parser` (external npm), `vscode` API, `./settingsDocumentHelper`, `./extensionsProposals`, `./importExportProfiles` (side-effect import).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/src/settingsDocumentHelper.ts`

- **Role:** Provides rich completion logic specifically for `settings.json` values. Handles a fixed set of known setting keys with custom completion strategies; falls back to language-override completions for all others.
- **Key symbols:**
  - `SettingsDocument` class (line 12): Wraps a `vscode.TextDocument`.
  - `provideCompletionItems` (line 16): Top-level dispatcher; uses `location.path[0]` to branch into eight specialized handlers.
  - `provideWindowTitleCompletionItems` (line 95): Produces completions for `${variable}` tokens within `window.title` string values. Uses `document.getWordRangeAtPosition(pos, /\$\{[^"\}]*\}?/)` at line 102 to find the replace range. Hard-codes 18 window-title variables (lines 113–130).
  - `provideEditorLabelCompletionItems` (line 134): Same pattern for `workbench.editor.label.patterns`; hard-codes 4 variables.
  - `provideFilesAssociationsCompletionItems` (line 159): At path depth 2, key position → snippet items for glob patterns; at value position → calls `provideLanguageCompletionItemsForLanguageOverrides`.
  - `provideExcludeCompletionItems` (line 189): Covers `files.exclude`, `search.exclude`, `explorer.autoRevealExclude`. Key positions at depth 1 or 2 → 6 snippet items for common glob patterns; value at depth 2 → sibling-match snippet.
  - `provideLanguageCompletionItems` (line 253): Calls `vscode.languages.getLanguages()` to fetch the live language list; prepends the `${activeEditorLanguage}` pseudo-language item.
  - `provideLanguageCompletionItemsForLanguageOverrides` (line 265): Same language API call; returns items with `CompletionItemKind.Property`.
  - `provideLanguageOverridesCompletionItems` (line 277): Handles completions inside `["language"]` bracket-notation override keys. Uses `OVERRIDE_IDENTIFIER_REGEX` (`/\[([^\[\]]*)\]/g`, line 10) to parse the existing overrides in `previousNode.value`, builds per-override ranges, and suppresses languages already configured. Skips the first override range because the JSON language server already handles it (comment at line 301–303).
  - `providePortsAttributesCompletionItem` (line 321): Returns 3 hard-coded snippet items for port number, port range, and command-pattern keys.
  - `getReplaceRange` (line 72): Mirrors the helper in `configurationEditingMain.ts`.
  - `isCompletingPropertyValue` (line 83): Similar to `isCompletingInsidePropertyStringValue` in main but uses `>=` / `<=` (inclusive) bounds.
  - `newSimpleCompletionItem` / `newSnippetCompletionItem` (lines 346, 355): Factory helpers.
- **Control flow:** Single async `provideCompletionItems` → if/else chain on `location.path[0]` → specific provider method → return items.
- **Data flow:** `vscode.TextDocument` text → `getLocation` → `Location` → per-setting branch → optional async `vscode.languages.getLanguages()` or `provideInstalledExtensionProposals` → `CompletionItem[]`.
- **Dependencies:** `vscode`, `jsonc-parser` (`getLocation`, `Location`, `parse`), `./extensionsProposals`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/src/extensionsProposals.ts`

- **Role:** Single exported function providing completion items from the set of currently installed VS Code extensions, filtered to exclude already-listed entries.
- **Key symbols:**
  - `provideInstalledExtensionProposals` (line 9): Accepts `existing: string[]`, `additionalText`, `range`, `includeBuiltinExtensions`. Reads `vscode.extensions.all` at line 11; when `includeBuiltinExtensions` is false, filters out extensions whose ID starts with `vscode.` or equals `Microsoft.vscode-markdown`. Builds a `CompletionItem` per extension where `insertText` is `"${e.id}"${additionalText}` and `filterText` matches the insert text. If no proposals remain, returns a single example item (lines 24–28).
- **Control flow:** Synchronous filter on `vscode.extensions.all` → map to `CompletionItem[]`.
- **Data flow:** `vscode.Extension[]` (runtime) → filter → map → `CompletionItem[]`.
- **Dependencies:** `vscode`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/src/importExportProfiles.ts`

- **Role:** Implements the `vscode.ProfileContentHandler` interface for GitHub Gist-based profile import/export. Registers itself via the proposed API `vscode.window.registerProfileContentHandler('github', ...)` at line 81 as a side effect of the import in `configurationEditingMain.ts`.
- **Key symbols:**
  - `GitHubGistProfileContentHandler` (line 11): Implements `vscode.ProfileContentHandler`. Fields: `name = 'GitHub'`, `description = 'gist'`.
  - `getOctokit` (line 17): Lazily initializes an authenticated `Octokit` instance. Calls `vscode.authentication.getSession('github', ['gist', 'user:email'], { createIfNone: true })` at line 20 to obtain an OAuth token. Passes `{ request: { agent } }` from `./node/net` at line 25 to route requests through the system HTTP proxy.
  - `saveProfile` (line 35): Calls `octokit.gists.create` with `public: false` and the profile content as a file; extracts `result.data.id` and `result.data.html_url` into a `{ id, link }` return value.
  - `getPublicOctokit` (line 52): Separate unauthenticated `Octokit` instance for reading public gists.
  - `readProfile` (lines 63–77): Two overloads — one taking a string gist ID, one a `vscode.Uri`. Uses `basename(uri.path)` to extract the gist ID from a URL path, then calls `octokit.gists.get`. Returns the content of the first file in the gist.
- **Control flow:** Passive at startup (registration only). On `onProfile` activation events, `saveProfile` or `readProfile` is invoked by VS Code's profile sync machinery.
- **Data flow:** Profile JSON string → `octokit.gists.create` → GitHub API → `{ id, link: vscode.Uri }`. For reading: gist ID/URI → `octokit.gists.get` → raw content string.
- **Dependencies:** `@octokit/rest` (dynamic import at runtime), `vscode` API, `./node/net` (for `agent`), `path` (Node.js built-in `basename`).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/src/node/net.ts`

- **Role:** Node.js-only network agent for proxying HTTPS requests through a system-configured HTTP proxy.
- **Key symbols:**
  - `agent` (line 11): Module-level export; value is the result of `getAgent()` called at import time.
  - `getAgent` (line 17): Reads `process.env.HTTPS_PROXY`; if set, parses it as a `URL` and constructs an `httpsOverHttp` tunnel agent (from the `tunnel` npm package) at line 24. Falls back to Node's `globalAgent` on parse failure or absent env var.
- **Control flow:** Executes synchronously at module load; `getAgent` is called once.
- **Data flow:** `process.env.HTTPS_PROXY` → `URL` parse → `httpsOverHttp({ proxy: { host, port, proxyAuth } })` → `Agent`.
- **Dependencies:** Node.js built-ins `https`, `url`; npm `tunnel`; `vscode` (for `window.showErrorMessage`).

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/src/browser/net.ts`

- **Role:** Browser-platform stub for the `agent` export. Exports `undefined` so that the browser build of `importExportProfiles.ts` compiles without Node.js dependencies.
- **Key symbols:**
  - `agent` (line 6): `export const agent = undefined`.
- **Control flow:** None — constant export.
- **Data flow:** None.
- **Dependencies:** None.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/esbuild.browser.mts`

- **Role:** Browser bundle configuration. Defines the `browserNetPlugin` esbuild plugin that redirects all `./node/net` import resolutions to `./browser/net.ts` at bundle time, enabling the single source tree to build for both Node and browser targets.
- **Key symbols:**
  - `browserNetPlugin` (line 15): `Plugin` with `build.onResolve({ filter: /\/node\/net$/ }, ...)` at line 18; resolves to the absolute path of `src/browser/net.ts`.
  - `run` (line 24): Imported from `../esbuild-extension-common.mts`; receives `{ platform: 'browser', entryPoints: { configurationEditingMain }, outdir: 'dist/browser', additionalOptions: { plugins: [browserNetPlugin] } }`.
- **Control flow:** Build-time only; plugin fires during esbuild's resolve phase for the browser bundle.
- **Data flow:** Source import string `./node/net` → plugin intercepts → resolved to `src/browser/net.ts` path → bundled as `undefined` export.
- **Dependencies:** `esbuild`, `node:path`, `../esbuild-extension-common.mts`.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/configuration-editing/src/test/completion.test.ts`

- **Role:** Integration test suite that exercises all registered completion providers by writing real files to a temp directory, activating the extension via `vscode.extensions.getExtension('vscode.configuration-editing')!.activate()` (line 583), and invoking the `vscode.executeCompletionItemProvider` command.
- **Key symbols:**
  - `testFolder` (line 14): `fs.mkdtemp(path.join(os.tmpdir(), 'conf-editing-'))` — created once per suite run.
  - `testCompletion` (line 545): Core helper; strips `|` cursor marker from content, writes the file, opens the document, sets language mode, calls `executeCompletionItemProvider`, and either asserts presence/absence of the expected label or applies the insert text and compares document text.
  - `setTestContent` (line 582): Activates the extension, opens the temp file in an editor, sets language ID via `vscode.languages.setTextDocumentLanguage`, replaces content.
  - Test suites cover: `settings.json` (window.title, files.associations, files.exclude, files.defaultLanguage, remote.extensionKind, remote.portsAttributes), `extensions.json` (recommendations), `launch.json` (variable completions), `tasks.json` (variable completions), `keybindings.json` (context key insert and replace).
- **Control flow:** Mocha `suite`/`test` structure; each test calls `testCompletion` one or more times with inline content strings using `|` as cursor marker.
- **Data flow:** String template → write to temp file → `vscode.workspace.openTextDocument` → `vscode.commands.executeCommand('vscode.executeCompletionItemProvider')` → `CompletionList` → assertion.
- **Dependencies:** `vscode`, `assert`, `fs.promises`, `path`, `os`, `mocha`.

---

### Cross-Cutting Synthesis

The `configuration-editing` extension is a pure VS Code API consumer: all intelligence lives in TypeScript and is delivered as a VS Code extension that registers `CompletionItemProvider`, `DocumentSymbolProvider`, and `ProfileContentHandler` instances against the VS Code extension host. The entire completion system is built on `jsonc-parser`'s `getLocation` function, which maps a byte offset in a JSON(C) document to a typed `Location` object carrying `path`, `isAtPropertyKey`, and `previousNode`; all providers dispatch solely on this object's fields. Platform bifurcation is handled at build time — an esbuild plugin transparently redirects `./node/net` to `./browser/net` for the browser bundle, so the single TypeScript source tree produces two independent bundles (`out/configurationEditingMain` for Node/Electron, `dist/browser/configurationEditingMain` for browser). The only runtime I/O beyond the VS Code API is the `importExportProfiles.ts` module, which dynamically imports `@octokit/rest` and calls the GitHub REST API via an HTTPS proxy agent constructed from `process.env.HTTPS_PROXY`. For a Tauri/Rust port, the JSON path-analysis work (`getLocation`, `visit`) maps to a Rust JSONC parser; the VS Code `CompletionItemProvider` / `DocumentSymbolProvider` registration surface needs to be re-implemented as an LSP server capability; the GitHub Gist integration requires an HTTP client in Rust (e.g., `reqwest`) with OS proxy detection; and the dual-platform agent stub pattern (`node/net` vs `browser/net`) can be replaced with Tauri's `http` plugin or conditional compilation.

---

### Out-of-Partition References

- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-extension-common.mts` — shared esbuild runner invoked by `esbuild.browser.mts:7`.
- `src/vscode-dts/vscode.d.ts` — VS Code API type declarations (included in both tsconfigs).
- `src/vscode-dts/vscode.proposed.profileContentHandlers.d.ts` — proposed API type for `registerProfileContentHandler` / `ProfileContentHandler`; required for `importExportProfiles.ts`.
- `vscode://schemas/settings/user`, `vscode://schemas/keybindings`, etc. — internal VS Code schema URIs referenced by `jsonValidation` entries in `package.json`; these schemas are generated elsewhere in the VS Code core (not in this extension).
- `getContextKeyInfo` command (called at `configurationEditingMain.ts:227`) — implemented in VS Code core, not this extension.
