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
