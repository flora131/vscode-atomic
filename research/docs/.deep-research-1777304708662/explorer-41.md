# Partition 41 of 79 — Findings

## Scope
`extensions/search-result/` (4 files, 567 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Search Result Extension - File Location Index

## Research Question
Porting VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

## Partition: extensions/search-result/

This extension implements virtual search result documents with language features (syntax highlighting, symbol navigation, definition linking) for search result editing—a key pattern for IDE core functionality port design.

---

## Implementation

- `extensions/search-result/src/extension.ts` — Main extension entry point; registers 5 language feature providers (document symbols, completion, definition, document links) for virtual search-result documents; demonstrates vscode API patterns for virtual document consumption
- `extensions/search-result/syntaxes/generateTMLanguage.js` — Build script generating TextMate language grammar for search result syntax highlighting; embeds scope mappings for 40+ language syntax inclusion patterns

---

## Configuration

- `extensions/search-result/package.json` — Extension manifest with activation event `onLanguage:search-result`, capabilities for virtualWorkspaces/untrustedWorkspaces, API proposal `documentFiltersExclusive`
- `extensions/search-result/package.nls.json` — Localization strings for display name and description
- `extensions/search-result/tsconfig.json` — TypeScript compilation target extending base config; includes vscode type definitions
- `extensions/search-result/tsconfig.browser.json` — Browser build TypeScript config extending node target
- `extensions/search-result/.vscodeignore` — Package exclusion rules for bundled extension

---

## Build / Generation

- `extensions/search-result/esbuild.mts` — ESM-based build orchestration for platform:node entrypoint
- `extensions/search-result/esbuild.browser.mts` — ESM-based browser bundle build
- `extensions/search-result/syntaxes/searchResult.tmLanguage.json` — Generated TextMate grammar JSON (output artifact from generateTMLanguage.js)

---

## Assets

- `extensions/search-result/images/icon.png` — Extension icon
- `extensions/search-result/src/media/refresh-dark.svg` — UI asset (dark mode)
- `extensions/search-result/src/media/refresh-light.svg` — UI asset (light mode)

---

## Documentation

- `extensions/search-result/README.md` — Feature summary: syntax highlighting, symbol information, result highlighting, go-to-definition for search results editor

---

## Notable Clusters

- `extensions/search-result/src/` — 1 file; core TypeScript implementation of extension activation and virtual document language feature registration
- `extensions/search-result/syntaxes/` — 2 files; grammar generation script + generated grammar JSON artifact
- `extensions/search-result/` — Configuration and build orchestration files at root (5 TypeScript config + esbuild scripts)

---

## Key Architectural Notes for Port

The extension demonstrates critical vscode API patterns that a Tauri/Rust port must replicate:

1. **Language Feature Provider Registration** — Registers document symbol, completion, definition, and document link providers for a virtual document type (`search-result`). These are core IDE intelligence APIs.

2. **Virtual Document URI Scheme** — Handles URI parsing and workspace folder resolution for virtual `untitled`, `vscode-userdata`, and custom file scheme URIs (lines 130–175 in extension.ts).

3. **Document Change Lifecycle** — Implements caching + change listeners for efficient incremental parsing and decoration updates (lines 17, 115–119).

4. **Language Selector** — Uses exclusive document filter (`SEARCH_RESULT_SELECTOR`) with activation event `onLanguage:search-result` to bind features to a non-file-backed document language (line 12, package.json `activationEvents`).

5. **TextMate Grammar Integration** — Integrates syntax highlighting via TextMate scope system with dynamic language inclusion (40+ language syntax scopes embedded in grammar).

6. **API Proposals** — Uses `documentFiltersExclusive` capability (package.json line 34), indicating reliance on cutting-edge vscode extension API for exclusive language document filtering.

---

## File Summary

- **Total files in scope:** 4 (excluding node_modules, package-lock.json)
- **Lines of code:** ~567 (primarily src/extension.ts ~280 lines, generateTMLanguage.js ~250 lines)
- **Languages:** TypeScript, JavaScript, JSON
- **Core Extension Language:** TypeScript with vscode module dependency
- **Build Targets:** Node (Electron) and Browser (web-based VS Code)

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: Search Result Extension (Virtual Document & Language Server Patterns)

## Partition 41: extensions/search-result/

This partition demonstrates how VS Code extends core IDE functionality through language-specific providers and virtual document handling. The search-result extension shows patterns essential for porting core features to Tauri/Rust, particularly around document intelligence, navigation, and editor interactions.

---

#### Pattern: Language Provider Registration Pattern
**Where:** `extensions/search-result/src/extension.ts:39-53`
**What:** Registers a document symbol provider to expose searchable structure for navigation and outline views.
```typescript
vscode.languages.registerDocumentSymbolProvider(SEARCH_RESULT_SELECTOR, {
	provideDocumentSymbols(document: vscode.TextDocument, token: vscode.CancellationToken): vscode.DocumentSymbol[] {
		const results = parseSearchResults(document, token)
			.filter(isFileLine)
			.map(line => new vscode.DocumentSymbol(
				line.path,
				'',
				vscode.SymbolKind.File,
				line.allLocations.map(({ originSelectionRange }) => originSelectionRange!).reduce((p, c) => p.union(c), line.location.originSelectionRange!),
				line.location.originSelectionRange!,
			));

		return results;
	}
}),
```

**Variations / call-sites:** 
- `extension.ts:55-74` - Completion provider (inline trigger `#`)
- `extension.ts:76-98` - Definition provider (jump-to-definition support)
- `extension.ts:100-106` - Document link provider (clickable links)

---

#### Pattern: Editor Event Lifecycle Management
**Where:** `extensions/search-result/src/extension.ts:108-126`
**What:** Manages editor lifecycle with active-editor change detection, listener subscription cleanup, and state reset on tab switch.
```typescript
vscode.window.onDidChangeActiveTextEditor(editor => {
	if (editor?.document.languageId === 'search-result') {
		// Clear the parse whenever we open a new editor.
		// Conservative because things like the URI might remain constant even if the contents change, and re-parsing even large files is relatively fast.
		cachedLastParse = undefined;

		documentChangeListener?.dispose();
		documentChangeListener = vscode.workspace.onDidChangeTextDocument(doc => {
			if (doc.document.uri === editor.document.uri) {
				decorate(editor);
			}
		});

		decorate(editor);
	}
}),

{ dispose() { cachedLastParse = undefined; documentChangeListener?.dispose(); } }
```

**Variations / call-sites:** 
- Pattern shows subscription cleanup pattern (`documentChangeListener?.dispose()`)
- Used across all context.subscriptions.push() registrations for resource management

---

#### Pattern: Text Decoration & Visual Rendering
**Where:** `extensions/search-result/src/extension.ts:22-31`
**What:** Creates and applies text editor decorations (styling) for contextual visualization of match vs. context lines.
```typescript
const contextLineDecorations = vscode.window.createTextEditorDecorationType({ opacity: '0.7' });
const matchLineDecorations = vscode.window.createTextEditorDecorationType({ fontWeight: 'bold' });

const decorate = (editor: vscode.TextEditor) => {
	const parsed = parseSearchResults(editor.document).filter(isResultLine);
	const contextRanges = parsed.filter(line => line.isContext).map(line => line.prefixRange);
	const matchRanges = parsed.filter(line => !line.isContext).map(line => line.prefixRange);
	editor.setDecorations(contextLineDecorations, contextRanges);
	editor.setDecorations(matchLineDecorations, matchRanges);
};
```

**Variations / call-sites:**
- Decoration instances created once per activation (module scope)
- Called on editor activation (`extension.ts:34`), tab switch (`extension.ts:121`), and document change (`extension.ts:117`)

---

#### Pattern: URI Resolution with Multi-Root Workspace Support
**Where:** `extensions/search-result/src/extension.ts:130-175`
**What:** Converts relative search result paths to absolute URIs, handling workspaces, special schemes (untitled, vscode-userdata), and home directories.
```typescript
function relativePathToUri(path: string, resultsUri: vscode.Uri): vscode.Uri | undefined {

	const userDataPrefix = '(Settings) ';
	if (path.startsWith(userDataPrefix)) {
		return vscode.Uri.file(path.slice(userDataPrefix.length)).with({ scheme: 'vscode-userdata' });
	}

	if (pathUtils.isAbsolute(path)) {
		if (/^[\\\/]Untitled-\d*$/.test(path)) {
			return vscode.Uri.file(path.slice(1)).with({ scheme: 'untitled', path: path.slice(1) });
		}
		return vscode.Uri.file(path);
	}

	if (path.indexOf('~/') === 0) {
		const homePath = process.env.HOME || process.env.HOMEPATH || '';
		return vscode.Uri.file(pathUtils.join(homePath, path.slice(2)));
	}

	const uriFromFolderWithPath = (folder: vscode.WorkspaceFolder, path: string): vscode.Uri =>
		vscode.Uri.joinPath(folder.uri, path);

	if (vscode.workspace.workspaceFolders) {
		const multiRootFormattedPath = /^(.*) • (.*)$/.exec(path);
		if (multiRootFormattedPath) {
			const [, workspaceName, workspacePath] = multiRootFormattedPath;
			const folder = vscode.workspace.workspaceFolders.filter(wf => wf.name === workspaceName)[0];
			if (folder) {
				return uriFromFolderWithPath(prefixMatch, workspacePath);
			}
		}
		else if (vscode.workspace.workspaceFolders.length === 1) {
			return uriFromFolderWithPath(vscode.workspace.workspaceFolders[0], path);
		}
	}
}
```

**Variations / call-sites:**
- `extension.ts:205` - Called during document parsing to resolve file targets

---

#### Pattern: Document Parsing with Caching
**Where:** `extensions/search-result/src/extension.ts:16-17, 184-277`
**What:** Implements version-based caching of parsed document contents to avoid redundant re-parsing when document hasn't changed.
```typescript
let cachedLastParse: { version: number; parse: ParsedSearchResults; uri: vscode.Uri } | undefined;

function parseSearchResults(document: vscode.TextDocument, token?: vscode.CancellationToken): ParsedSearchResults {

	if (cachedLastParse && cachedLastParse.uri === document.uri && cachedLastParse.version === document.version) {
		return cachedLastParse.parse;
	}

	const lines = document.getText().split(/\r?\n/);
	const links: ParsedSearchResults = [];

	// ... parsing logic ...

	cachedLastParse = {
		version: document.version,
		parse: links,
		uri: document.uri
	};

	return links;
}
```

**Variations / call-sites:**
- `extension.ts:26` - Decoration rendering calls parseSearchResults
- `extension.ts:41` - Symbol provider calls parseSearchResults
- `extension.ts:78` - Definition provider calls parseSearchResults
- `extension.ts:102` - Document link provider calls parseSearchResults
- Cache invalidation: `extension.ts:112` when editor changes

---

#### Pattern: LocationLink Navigation Infrastructure
**Where:** `extensions/search-result/src/extension.ts:177-179, 209-266`
**What:** Defines location links (source range → target range mappings) for navigation across file boundaries, with multi-location aggregation.
```typescript
type ParsedSearchFileLine = { type: 'file'; location: vscode.LocationLink; allLocations: vscode.LocationLink[]; path: string };
type ParsedSearchResultLine = { type: 'result'; locations: Required<vscode.LocationLink>[]; isContext: boolean; prefixRange: vscode.Range };

const location: vscode.LocationLink = {
	targetRange: new vscode.Range(0, 0, 0, 1),
	targetUri: currentTarget,
	originSelectionRange: new vscode.Range(i, 0, i, line.length),
};

// ...

const convenienceLocation: Required<vscode.LocationLink> = {
	targetRange,
	targetSelectionRange: new vscode.Range(lineNumber, 0, lineNumber, 1),
	targetUri: currentTarget,
	originSelectionRange: new vscode.Range(i, 0, i, metadataOffset - 1),
};
locations.push(convenienceLocation);
```

**Variations / call-sites:**
- `extension.ts:81` - Definition provider returns aggregated locations
- `extension.ts:104` - Document link provider maps file lines to URIs

---

#### Pattern: Cancellation Token Handling
**Where:** `extensions/search-result/src/extension.ts:40, 77, 101, 184, 198`
**What:** Integrates cancellation tokens for async operations to support user-initiated cancellation (e.g., editor close mid-operation).
```typescript
provideDocumentSymbols(document: vscode.TextDocument, token: vscode.CancellationToken): vscode.DocumentSymbol[] {
	const results = parseSearchResults(document, token)
		// ...
}

function parseSearchResults(document: vscode.TextDocument, token?: vscode.CancellationToken): ParsedSearchResults {
	// ...
	for (let i = 0; i < lines.length; i++) {
		// TODO: This is probably always false, given we're pegging the thread...
		if (token?.isCancellationRequested) { return []; }
		// ...
	}
}
```

**Variations / call-sites:**
- All provider functions accept and forward tokens
- Token check present in long-running parsing loop

---

## Summary

The search-result extension demonstrates 7 key patterns essential for IDE functionality ports to Tauri/Rust:

1. **Language Provider Registration**: How language intelligence (symbols, completions, definitions, links) hooks into document events
2. **Editor Lifecycle Management**: Subscription cleanup and state resets across editor switches
3. **Visual Rendering**: Decoration API for document-level styling without virtual documents
4. **URI Resolution**: Multi-root workspace path normalization and scheme conversion
5. **Document Caching**: Version-based optimization for expensive parsing operations
6. **Navigation Infrastructure**: LocationLink abstraction for cross-file navigation
7. **Async Cancellation**: Token-based operation cancellation for responsive UI

These patterns represent the non-virtual-document portions of VS Code's IDE model. Note: the architectural note mentioned `workspace.registerTextDocumentContentProvider()` for virtual documents, but the search-result extension uses direct document parsing instead, indicating this particular feature doesn't implement virtual document generation—only consumption and annotation of existing documents. This is consistent with search results being a read-only, computed view of file content rather than a dynamic virtual filesystem.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
