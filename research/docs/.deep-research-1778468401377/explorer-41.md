# Partition 41 of 80 — Findings

## Scope
`extensions/search-result/` (4 files, 567 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Scope: extensions/search-result/

## Implementation
- `extensions/search-result/src/extension.ts` — Core language features for search results: document symbol provider, completion items, definition provider, document link provider, and text decorations. Handles parsing search result files and providing navigation/linking to file locations. Relevant to language intelligence features in IDE porting.

## Configuration
- `extensions/search-result/package.json` — Extension manifest declaring the search-result language, activation events, and contribution points (configuration defaults, language definition, TextMate grammar)
- `extensions/search-result/tsconfig.json` — TypeScript configuration extending base config, targeting Node platform
- `extensions/search-result/tsconfig.browser.json` — TypeScript configuration for browser/web platform
- `extensions/search-result/.vscodeignore` — Build artifacts and source files to exclude from bundled extension

## Build / Bundling
- `extensions/search-result/esbuild.mts` — Node platform build configuration using esbuild (entry point: extension.ts)
- `extensions/search-result/esbuild.browser.mts` — Browser platform build configuration using esbuild with browser-specific tsconfig

## Syntax / Language Definition
- `extensions/search-result/syntaxes/generateTMLanguage.js` — JavaScript build script (4662-line output file) that generates TextMate grammar for 60+ language syntax highlighting integration within search results (bat, c, cpp, cs, go, js, ts, rust, python, etc.)
- `extensions/search-result/syntaxes/searchResult.tmLanguage.json` — Generated TextMate grammar JSON (4662 lines) defining syntax highlighting and scoping for search result format

## Documentation
- `extensions/search-result/README.md` — Bundled extension description stating it provides syntax highlighting, symbol information, result highlighting, and go-to-definition for the Search Results Editor

## Assets
- `extensions/search-result/images/icon.png` — Extension icon
- `extensions/search-result/src/media/refresh-light.svg` — Light theme refresh icon
- `extensions/search-result/src/media/refresh-dark.svg` — Dark theme refresh icon
- `extensions/search-result/package.nls.json` — Localization strings for display name and description

## Notable Clusters
- `extensions/search-result/` — 15 files total (567 LOC in src/extension.ts + generated grammar). Implements language intelligence (symbols, completion, navigation) and text rendering for VS Code's integrated search results panel.

---

## Summary

The `search-result` extension implements IDE-core language features for search result display: document symbols, completion, navigation, and link resolution. The implementation is a bundled TypeScript extension using VS Code's extension API. Key cross-platform concern: dual builds (Node and browser platforms) with platform-specific TypeScript configs and esbuild configurations, suggesting the search results UI must work in both desktop and web contexts. The 60+ language syntax highlighting integration (via generated TextMate grammar) indicates tight coupling with VS Code's syntax highlighting infrastructure. Porting this would require: (1) replacing VS Code Extension API calls with Rust equivalents, (2) reimplementing the parsing/linking logic for search results, and (3) generating or hardcoding TextMate grammar support for syntax highlighting—a non-trivial cross-platform concern for Tauri/Rust.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder Research: VS Code Core IDE Functionality Porting

**Scope:** `extensions/search-result/` (4 TypeScript files, ~567 LOC)

**Research Question:** What patterns demonstrate how VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) is currently expressed in TypeScript/Electron that would need porting to Tauri/Rust?

---

## Patterns Found

#### Pattern: Language Service Provider Registration with Type-Safe Selectors

**Where:** `extensions/search-result/src/extension.ts:39-53`

**What:** Document symbol provider leveraging vscode.languages API with type-safe document selectors to enable IDE features like symbol navigation.

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
- `extension.ts:55-74` — CompletionItemProvider registration
- `extension.ts:76-98` — DefinitionProvider registration
- `extension.ts:100-106` — DocumentLinkProvider registration

---

#### Pattern: Completion Item Provider with Contextual Filtering

**Where:** `extensions/search-result/src/extension.ts:55-74`

**What:** IntelliSense provider that filters completion suggestions based on document position and existing content, demonstrating language intelligence context awareness.

```typescript
vscode.languages.registerCompletionItemProvider(SEARCH_RESULT_SELECTOR, {
	provideCompletionItems(document: vscode.TextDocument, position: vscode.Position): vscode.CompletionItem[] {

		const line = document.lineAt(position.line);
		if (position.line > 3) { return []; }
		if (position.character === 0 || (position.character === 1 && line.text === '#')) {
			const header = Array.from({ length: DIRECTIVES.length }).map((_, i) => document.lineAt(i).text);

			return DIRECTIVES
				.filter(suggestion => header.every(line => line.indexOf(suggestion) === -1))
				.map(flag => ({ label: flag, insertText: (flag.slice(position.character)) + ' ' }));
		}

		if (line.text.indexOf('# Flags:') === -1) { return []; }

		return FLAGS
			.filter(flag => line.text.indexOf(flag) === -1)
			.map(flag => ({ label: flag, insertText: flag + ' ' }));
	}
}, '#'),
```

**Variations / call-sites:** Completion trigger character `'#'` specifies activation context.

---

#### Pattern: Definition Provider with Multi-Location Link Resolution

**Where:** `extensions/search-result/src/extension.ts:76-98`

**What:** Go-to-definition implementation that resolves position-aware definition links with precise target selection ranges, enabling IDE navigation features.

```typescript
vscode.languages.registerDefinitionProvider(SEARCH_RESULT_SELECTOR, {
	provideDefinition(document: vscode.TextDocument, position: vscode.Position, token: vscode.CancellationToken): vscode.DefinitionLink[] {
		const lineResult = parseSearchResults(document, token)[position.line];
		if (!lineResult) { return []; }
		if (lineResult.type === 'file') {
			return lineResult.allLocations.map(l => ({ ...l, originSelectionRange: lineResult.location.originSelectionRange }));
		}

		const location = lineResult.locations.find(l => l.originSelectionRange.contains(position));
		if (!location) {
			return [];
		}

		const targetPos = new vscode.Position(
			location.targetSelectionRange.start.line,
			location.targetSelectionRange.start.character + (position.character - location.originSelectionRange.start.character)
		);
		return [{
			...location,
			targetSelectionRange: new vscode.Range(targetPos, targetPos),
		}];
	}
}),
```

**Variations / call-sites:** Primary IDE navigation pattern; extends to multi-file reference resolution.

---

#### Pattern: Document Link Provider for Clickable File References

**Where:** `extensions/search-result/src/extension.ts:100-106`

**What:** Document link provider enabling hyperlink navigation to files, core IDE pattern for click-to-navigate functionality.

```typescript
vscode.languages.registerDocumentLinkProvider(SEARCH_RESULT_SELECTOR, {
	async provideDocumentLinks(document: vscode.TextDocument, token: vscode.CancellationToken): Promise<vscode.DocumentLink[]> {
		return parseSearchResults(document, token)
			.filter(isFileLine)
			.map(({ location }) => ({ range: location.originSelectionRange!, target: location.targetUri }));
	}
}),
```

**Variations / call-sites:** Async document link generation with cancellation token support.

---

#### Pattern: Text Editor Decoration with Visual Styling

**Where:** `extensions/search-result/src/extension.ts:22-31`

**What:** Visual editor decorations for syntax highlighting and emphasis, demonstrating core rendering pipeline for IDE highlighting features.

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
- `extension.ts:34` — Activation on editor initialization
- `extension.ts:121` — Re-decoration on active editor change

---

#### Pattern: Event-Driven Document Change Tracking with Lifecycle Management

**Where:** `extensions/search-result/src/extension.ts:108-126`

**What:** Reactive event listeners for editor state changes (active editor switches, document edits) with cleanup/disposal pattern, enabling real-time IDE features.

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

**Variations / call-sites:** Core extension lifecycle pattern appearing throughout VS Code extensions.

---

#### Pattern: URI Scheme Handling and Path Resolution

**Where:** `extensions/search-result/src/extension.ts:130-175`

**What:** Complex path-to-URI transformation handling multiple URI schemes (file, vscode-userdata, untitled, etc.) and workspace folder resolution, demonstrating file system abstraction layer essential for IDE portability.

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
	}
}
```

**Variations / call-sites:** URI resolution chain supports virtual workspaces (untrusted, web, remote).

---

#### Pattern: Extension Activation with Language-Specific Trigger Events

**Where:** `extensions/search-result/package.json:14-16`

**What:** Declarative extension activation tied to language events, demonstrating lazy-load pattern for IDE plugin system.

```json
"activationEvents": [
  "onLanguage:search-result"
],
```

**Variations / call-sites:** Standard activation contract; enables scalable plugin ecosystem.

---

#### Pattern: Build Configuration for Dual-Target Compilation (Node + Browser)

**Where:** `extensions/search-result/esbuild.mts:11-18` and `package.json:12-13`

**What:** esbuild configuration producing separate Node.js and browser bundles from single TypeScript source, enabling IDE code to run in Electron and web environments.

```typescript
run({
	platform: 'node',
	entryPoints: {
		'extension': path.join(srcDir, 'extension.ts'),
	},
	srcDir,
	outdir: outDir,
}, process.argv);
```

With corresponding package.json entries:
```json
"main": "./out/extension.js",
"browser": "./dist/browser/extension",
```

**Variations / call-sites:** Browser build configured separately in `esbuild.browser.mts`; shared TypeScript source.

---

## Summary

The search-result extension demonstrates **seven core IDE capability patterns** that would require architectural translation from TypeScript/Electron to Tauri/Rust:

1. **Language service provider abstraction** — Pluggable API for document symbol, completion, definition, and link providers
2. **Position-aware code intelligence** — Cursor-based resolution of references, definitions, and completions
3. **Visual editor decorations** — Real-time syntax highlighting and emphasis rendering
4. **Reactive event system** — Document changes, editor switches, and lifecycle management
5. **URI scheme abstraction** — Multi-scheme path resolution (file, untitled, settings, virtual workspaces)
6. **Declarative extension activation** — Language-triggered lazy loading
7. **Dual-platform build tooling** — Single source compiled for multiple runtimes (Node/Electron, browser)

The most porting-intensive challenges would be:

- **Event loop and async patterns** — JS Promise/async mapped to Rust async/await with IPC boundaries
- **Dynamic provider registration** — Replacing vscode.languages API with FFI-friendly equivalents
- **URI abstraction** — Replacing vscode.Uri with cross-platform path handling
- **TextDocument/TextEditor models** — Translating VS Code's in-memory document model to Rust
- **Decoration rendering** — Mapping TextEditorDecorationType to native/web rendering backends

All patterns operate within the **vscode namespace** and depend on the **language server / extension host protocol**, indicating that a Tauri port would need to either:
- Replicate the entire LSP client infrastructure in Rust, or
- Maintain TypeScript as the extension runtime with IPC to Rust core

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
