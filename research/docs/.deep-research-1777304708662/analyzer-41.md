### Files Analysed

1. `extensions/search-result/src/extension.ts` (278 lines)
2. `extensions/search-result/syntaxes/generateTMLanguage.js` (252 lines)
3. `extensions/search-result/package.json` (68 lines)
4. `extensions/search-result/package.nls.json` (4 lines)
5. `extensions/search-result/syntaxes/searchResult.tmLanguage.json` (generated artifact, inspected first 60 lines)

---

### Per-File Notes

#### `extensions/search-result/src/extension.ts`

- **Role:** The sole runtime logic file for the `search-result` extension. It registers four VS Code language feature providers for the virtual `search-result` language, implements a line-level parser for the search-result document format, and applies decorations to context vs. match lines in editors.

- **Key symbols:**
  - `FILE_LINE_REGEX` (`extension.ts:9`) — `/^(\S.*):$/` — matches lines that name a file (e.g., `src/foo.ts:`).
  - `RESULT_LINE_REGEX` (`extension.ts:10`) — `/^(\s+)(\d+)(: |  )(\s*)(.*)$/` — matches indented result lines containing a line number and separator.
  - `ELISION_REGEX` (`extension.ts:11`) — `/⟪ ([0-9]+) characters skipped ⟫/g` — matches elision markers that represent omitted characters in long lines.
  - `SEARCH_RESULT_SELECTOR` (`extension.ts:12`) — `{ language: 'search-result', exclusive: true }` — used as the language selector for all four provider registrations; the `exclusive: true` field requires the `documentFiltersExclusive` API proposal.
  - `DIRECTIVES` / `FLAGS` (`extension.ts:13-14`) — static arrays listing header directive names and flag keywords for completion items.
  - `cachedLastParse` (`extension.ts:16`) — module-level cache storing the most recent `ParsedSearchResults` keyed by `{ version, uri }`.
  - `documentChangeListener` (`extension.ts:17`) — module-level disposable for the active document change listener.
  - `activate` (`extension.ts:20`) — exported entry point called by VS Code when a `search-result`-language document is opened.
  - `relativePathToUri` (`extension.ts:130`) — converts path strings found in the document into `vscode.Uri` instances, handling `(Settings)` prefix, absolute paths, `~/` home-relative paths, multi-root workspace `• ` separator format, and single-root workspace-relative paths.
  - `parseSearchResults` (`extension.ts:184`) — core parser; returns a `ParsedSearchResults` array indexed by document line number.
  - Type aliases `ParsedSearchFileLine` / `ParsedSearchResultLine` / `ParsedSearchResults` (`extension.ts:177-179`).

- **Control flow:**
  1. `activate` runs when the extension is loaded. It creates two `TextEditorDecorationType` objects (opacity 0.7 for context lines, bold for match lines) at `extension.ts:22-23`.
  2. If a `search-result` editor is already active, `decorate` is called immediately (`extension.ts:33-35`).
  3. Four providers are pushed to `context.subscriptions`:
     - **DocumentSymbolProvider** (`extension.ts:39-53`): calls `parseSearchResults`, filters `isFileLine`, emits one `DocumentSymbol` per file entry spanning all its result locations.
     - **CompletionItemProvider** (`extension.ts:55-74`): triggered by `#`; only active on lines 0-3; suggests `DIRECTIVES` not already present in the header, or `FLAGS` when the cursor is on a `# Flags:` line.
     - **DefinitionProvider** (`extension.ts:76-98`): on a file line returns all `allLocations`; on a result line finds the `LocationLink` whose `originSelectionRange` contains the cursor position, then adjusts `targetSelectionRange` to the exact character column.
     - **DocumentLinkProvider** (`extension.ts:100-106`): maps each file line to a `DocumentLink` pointing to `location.targetUri`.
  4. `onDidChangeActiveTextEditor` listener (`extension.ts:108-123`) clears `cachedLastParse`, replaces the document-change listener, and re-decorates on every editor switch.

- **Data flow inside `parseSearchResults` (`extension.ts:184-276`):**
  - Checks the version/URI cache at `extension.ts:186-188`; returns early on hit.
  - Splits document text by `\r?\n` (`extension.ts:190`).
  - Iterates lines. For each line matching `FILE_LINE_REGEX`: resolves to a URI via `relativePathToUri`, creates a `LocationLink` with `originSelectionRange` spanning the whole line, and stores `{ type: 'file', location, allLocations: [], path }` at `links[i]` (`extension.ts:201-217`).
  - For each line matching `RESULT_LINE_REGEX`: extracts `indentation`, `_lineNumber`, `separator` from capture groups; computes `metadataOffset` as the combined length of those groups (`extension.ts:225`); sets `targetRange` to a ±3-line window around the result line number (`extension.ts:226`).
  - Builds per-segment `LocationLink` objects by running `ELISION_REGEX` over the line (`extension.ts:233-244`): each non-elided segment maps to an `originSelectionRange` in the search-result document and a `targetSelectionRange` at the correct offset in the source file (accounting for skipped characters accumulated in `offset`).
  - A trailing segment covering the remainder of the line is added at `extension.ts:245-251`.
  - Match lines (separator `:`): locations are pushed into `currentTargetLocations` on the parent file line (`extension.ts:254-256`).
  - A "convenience location" covering the line-number prefix area is appended at `extension.ts:259-265`.
  - The result is stored as `{ type: 'result', locations, isContext: separator === ' ', prefixRange }` at `extension.ts:266`.
  - The completed parse is memoized into `cachedLastParse` at `extension.ts:270-274`.

- **Dependencies:**
  - `vscode` extension API (all four `languages.register*` APIs, `window.createTextEditorDecorationType`, `window.onDidChangeActiveTextEditor`, `workspace.onDidChangeTextDocument`, `Uri`, `Range`, `Position`, `DocumentSymbol`, `SymbolKind`).
  - Node.js built-in `path` (alias `pathUtils`) for `isAbsolute` and `join`.
  - `process.env.HOME` / `process.env.HOMEPATH` for `~/` resolution.
  - The `documentFiltersExclusive` API proposal (consumed implicitly via `SEARCH_RESULT_SELECTOR.exclusive`).

---

#### `extensions/search-result/syntaxes/generateTMLanguage.js`

- **Role:** A Node.js build-time script (not bundled) that generates `syntaxes/searchResult.tmLanguage.json` from an in-memory JavaScript object. Run via `npm run generate-grammar`.

- **Key symbols:**
  - `mappings` (`generateTMLanguage.js:8-60`) — array of `[extension, tmScope, optionalRegexp]` triples covering ~45 languages. Each entry drives generation of one TextMate repository rule.
  - `scopes` (`generateTMLanguage.js:62-102`) — flat object tree mapping logical role names (e.g., `scopes.resultBlock.result.prefix.lineNumber`) to TextMate scope name strings (e.g., `'meta.resultLinePrefix.lineNumber.search'`).
  - `repository` (`generateTMLanguage.js:104-150`) — populated by iterating `mappings`; each key is the file extension, and the value is a TextMate rule with `begin`/`end`/`patterns` that embeds the language-specific grammar scope (`{ include: scope }`) within search result line patterns. Two sub-patterns handle multi-line context blocks (`begin`/`while`) and single-line match lines.
  - `header` (`generateTMLanguage.js:152-207`) — array of four TextMate rules for `# Query:`, `# Flags:`, `# ContextLines:`, and `# Including:`/`# Excluding:` header lines.
  - `plainText` (`generateTMLanguage.js:209-235`) — fallback patterns applied when no language-specific grammar rule matches; handles file-path lines, result prefix lines, and elision markers with scope names but no embedded grammar.
  - `tmLanguage` (`generateTMLanguage.js:237-247`) — root grammar object with `scopeName: 'text.searchResult'`, patterns ordered: header rules, language-specific includes from `mappings`, then `plainText` fallbacks.
  - `fs.writeFileSync` call (`generateTMLanguage.js:249-251`) — serializes `tmLanguage` as JSON to `./searchResult.tmLanguage.json`.

- **Control flow:** The script is purely imperative. It builds `repository` by iterating `mappings` with `forEach`, assembles the root grammar object, then synchronously writes the output file. No exports, no async paths.

- **Data flow:** `mappings` entries → `repository[ext]` TextMate rule objects, each embedding `scope` via `{ include: scope }`. `header` and `plainText` are static arrays. All three are composed into `tmLanguage.patterns`. The final JSON is the complete TextMate grammar that VS Code's tokenizer loads at runtime.

- **Dependencies:** Node.js `fs` and `path` built-ins only (both via `require`).

---

#### `extensions/search-result/package.json`

- **Role:** Extension manifest declaring metadata, activation trigger, contributed language/grammar, API proposal requirements, and build scripts.

- **Key symbols:**
  - `"activationEvents": ["onLanguage:search-result"]` (`package.json:14-16`) — extension loads only when a document with language ID `search-result` is opened.
  - `"main": "./out/extension.js"` (`package.json:12`) — desktop entry point (compiled from `src/extension.ts`).
  - `"browser": "./dist/browser/extension"` (`package.json:13`) — web worker entry point for VS Code for the Web.
  - `"enabledApiProposals": ["documentFiltersExclusive"]` (`package.json:33-35`) — opts into the proposed API that allows `exclusive: true` on language selectors, preventing other providers from handling `search-result` documents.
  - `"contributes.languages"` (`package.json:42-52`) — registers language ID `search-result` with extension `.code-search`.
  - `"contributes.grammars"` (`package.json:53-59`) — binds grammar file `./syntaxes/searchResult.tmLanguage.json` to `scopeName: text.searchResult` for the `search-result` language.
  - `"contributes.configurationDefaults"` (`package.json:38-40`) — sets `editor.lineNumbers: off` for `search-result` documents by default.
  - `"capabilities.virtualWorkspaces": true` (`package.json:28`) — extension is compatible with virtual (remote/web) workspaces.
  - `"scripts"."generate-grammar"` (`package.json:18`) — `node ./syntaxes/generateTMLanguage.js` for regenerating the grammar artifact.

- **Control flow:** Declarative manifest; no runtime code. Activation is triggered by the VS Code host when matching the `onLanguage` event.

- **Dependencies:** VS Code extension host (API ≥ 1.39.0), `@types/node` dev dependency.

---

#### `extensions/search-result/package.nls.json`

- **Role:** Default (English) localization string table for the extension manifest's `%displayName%` and `%description%` placeholders.

- **Key symbols:**
  - `"displayName": "Search Result"` (`package.nls.json:2`)
  - `"description": "Provides syntax highlighting and language features for tabbed search results."` (`package.nls.json:3`)

- **Control flow / Data flow:** Purely declarative key-value JSON; consumed by VS Code's extension host to substitute `%key%` tokens in `package.json` at load time.

- **Dependencies:** None.

---

#### `extensions/search-result/syntaxes/searchResult.tmLanguage.json`

- **Role:** The generated TextMate grammar artifact consumed by VS Code's tokenizer engine at runtime. It is not hand-authored; the canonical source is `generateTMLanguage.js`.

- **Key symbols:**
  - `"scopeName": "text.searchResult"` — root scope that the grammar manifests contributes.
  - Top-level `"patterns"` array ordering: header directive rules first, then one `{ "include": "#<ext>" }` entry per extension in `mappings`, then `plainText` fallbacks.
  - `"repository"` — one rule per language extension enabling embedded syntax highlighting of search result content using that language's grammar scope.

- **Control flow:** Consumed by VS Code's `vscode-textmate` tokenizer engine; no JavaScript execution at runtime.

- **Dependencies:** References scopes like `source.ts`, `source.js`, `source.rust`, etc. — these are provided by their respective language extension grammars, which must be installed for embedded highlighting to work.

---

### Cross-Cutting Synthesis

The search-result extension is a self-contained virtual document language service built entirely on the VS Code extension API. Its runtime half (`extension.ts`) implements a bespoke line-oriented parser (`parseSearchResults`) that converts the proprietary `.code-search` text format — a sequence of file-path headers followed by indented line-number-prefixed match and context lines — into typed `ParsedSearchResultLine`/`ParsedSearchFileLine` objects. These objects directly back four language feature providers (symbols, completion, definition, document links), all keyed to the `search-result` language selector. The `exclusive: true` flag (via the `documentFiltersExclusive` API proposal) prevents any other provider from handling these documents. The parser is memoized by `(uri, version)` and invalidated on editor switch. The build-time half (`generateTMLanguage.js`) produces the TextMate grammar that enables syntax highlighting (including embedded per-language grammars) without any runtime TypeScript involvement. The extension declares both a desktop (`main`) and a browser (`browser`) entry point, making it compatible with VS Code for the Web.

For a Tauri/Rust port, the critical dependencies on the VS Code extension API surface are: `vscode.languages.register*` (four provider interfaces), `vscode.TextDocument`/`vscode.TextEditor` access, `vscode.Uri` construction including scheme manipulation (e.g., `vscode-userdata`, `untitled`), `vscode.workspace.workspaceFolders`, text decoration types, and the `documentFiltersExclusive` proposal. The TextMate grammar artifact (`searchResult.tmLanguage.json`) could be reused by any tokenizer engine that supports the TextMate grammar format (e.g., `syntect` in Rust). The language feature provider pattern would need to map to whatever LSP or plugin extension mechanism a Tauri host provides.

---

### Out-of-Partition References

- `vscode` extension API — core runtime dependency; defined in `src/vscode.d.ts` and the VS Code extension host, outside this partition.
- `documentFiltersExclusive` API proposal — proposal definition lives in `src/vscode-dts/vscode.proposed.documentFiltersExclusive.d.ts` or equivalent, outside this partition.
- Language grammar scopes (e.g., `source.ts`, `source.rust`) — contributed by their respective extension grammars (e.g., `extensions/typescript-basics/`, `extensions/rust/`), referenced by `repository` rules in `searchResult.tmLanguage.json`.
- `vscode-textmate` tokenizer engine — part of VS Code core, outside this partition.
- `process.env.HOME`/`process.env.HOMEPATH` — Node.js process globals; relevant only in desktop (Electron) builds, not in the browser entry point.
- `build/gulpfile.extensions.mjs` — referenced in `package.json:19`; part of the build system outside this partition.
