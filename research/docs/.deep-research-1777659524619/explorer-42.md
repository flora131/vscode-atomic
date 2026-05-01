# Partition 42 of 79 — Findings

## Scope
`extensions/search-result/` (4 files, 567 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locations for Search Results Extension (`extensions/search-result/`)

## Overview
The search-result extension provides syntax highlighting, symbol navigation, document linking, and definition providers for VS Code's `.code-search` virtual documents. This extension implements the search/navigation surface using a custom language grammar and document link providers—patterns directly relevant to porting search and navigation capabilities to a Tauri/Rust IDE.

## Implementation
- `extensions/search-result/src/extension.ts` - Main extension logic (277 lines)
  - Registers document symbol provider for file-level navigation symbols
  - Implements definition provider with location link generation
  - Registers document link provider for file path linking
  - Implements completion provider for search query directives and flags
  - Parses search result document syntax and maintains parsed result cache
  - Contains `parseSearchResults()` function for document parsing with regex-based line parsing (FILE_LINE_REGEX, RESULT_LINE_REGEX)
  - Implements visual decorations for context and match lines

## Configuration
- `extensions/search-result/package.json` - Extension manifest (68 lines)
  - Declares `search-result` language with `.code-search` file extension
  - Registers activation on `onLanguage:search-result` event
  - Contributes language grammar from `searchResult.tmLanguage.json`
  - Enables virtual workspace and untrusted workspace support
  - Configuration defaults: line numbers disabled for search result editors
  - Main entry point: `./out/extension.js` (Node.js) and `./dist/browser/extension` (browser)

- `extensions/search-result/tsconfig.json` - TypeScript configuration for Node.js build
- `extensions/search-result/tsconfig.browser.json` - TypeScript configuration for browser/web build
- `extensions/search-result/.vscodeignore` - Package exclusion rules

## Types / Interfaces
- Search result line types defined in `src/extension.ts`:
  - `ParsedSearchFileLine` - File header line with location and symbol path
  - `ParsedSearchResultLine` - Result/match line with location links and context flag
  - Type guards: `isFileLine()`, `isResultLine()`
  - `vscode.LocationLink[]` - Standard VS Code location linking interface used extensively
  - `vscode.DefinitionLink[]` - Definition provider return type
  - `vscode.DocumentLink[]` - Document link provider return type

## Syntax / Grammar
- `extensions/search-result/syntaxes/searchResult.tmLanguage.json` - Generated TextMate grammar (syntax coloring)
  - Covers 40+ language scopes (bat, c, clj, cpp, cs, css, dart, diff, dockerfile, fs, go, groovy, html, java, js, json, jsx, less, lua, m, makefile, md, mm, perl, php, ps1, py, r, rb, rs, scala, scss, sh, sql, swift, ts, tsx, vb, xml, yaml, etc.)
  - Header patterns: `# Query:`, `# Flags:`, `# ContextLines:`, `# Including:`, `# Excluding:`
  - Result block patterns with directory, basename, line numbers, elision markers
  - Context/match line differentiation via separators (`:` for matches, ` ` for context)

- `extensions/search-result/syntaxes/generateTMLanguage.js` - Grammar generator (252 lines)
  - Dynamically generates TextMate grammar from language mappings
  - Defines scope naming conventions and capture groups
  - Creates repository entries for each language with file extension patterns

## Build Configuration
- `extensions/search-result/esbuild.mts` - Node.js build configuration
- `extensions/search-result/esbuild.browser.mts` - Browser/web build configuration
- `extensions/search-result/package-lock.json` - Dependency lock file
- Build scripts in package.json:
  - `generate-grammar` - Generates grammar from JavaScript generator
  - `vscode:prepublish` - Gulp-based compilation
  - `compile-web`, `bundle-web`, `typecheck-web` - Web build targets

## Assets
- `extensions/search-result/images/icon.png` - Extension icon
- `extensions/search-result/src/media/refresh-light.svg` - Light theme refresh icon
- `extensions/search-result/src/media/refresh-dark.svg` - Dark theme refresh icon

## Documentation
- `extensions/search-result/README.md` - Brief extension description
  - Notes bundled extension status (disabled but not uninstallable)
  - Lists capabilities: syntax highlighting, symbol information, result highlighting, go-to-definition

## Localization
- `extensions/search-result/package.nls.json` - i18n strings for display name and description

## Notable Implementation Patterns
**Document Link Provider Chain:**
The extension chains three navigation mechanisms:
1. `registerDefinitionProvider` - Jumps to exact match location with character offset tracking
2. `registerDocumentLinkProvider` - Creates clickable file links for each search result file entry
3. `registerDocumentSymbolProvider` - Lists all matched files as symbols for outline navigation

**Search Result Parsing:**
Uses regex-based line-by-line parsing with state tracking:
- Maintains `currentTarget` (target URI) and `currentTargetLocations` across file sections
- Handles elision markers (`⟪ N characters skipped ⟫`) for incomplete result display
- Caches parsed results with version tracking to avoid reparsing unchanged documents
- Supports multi-root workspace path resolution with fallback logic

**Language-Agnostic Syntax:**
The grammar includes 40+ embedded language scopes, enabling syntax highlighting for matched content in multiple languages within the same search results document.

**Workspace-Aware Path Resolution:**
`relativePathToUri()` function handles:
- User data paths (`vscode-userdata://`)
- Absolute paths and untitled files
- Home directory expansions (`~/`)
- Multi-root workspace formatted paths (`workspaceName • relativePath`)
- Fallback path resolution for saved searches across sessions

## Summary
The search-result extension implements a lightweight language server for VS Code's search UI. Core elements for Tauri/Rust porting include:
- Virtual document model with custom syntax (`.code-search` language)
- Multi-provider architecture for navigation (definitions, document links, symbols)
- Language-agnostic syntax highlighting engine (40+ embedded scopes)
- Regex-based document parsing with intelligent caching
- Workspace and multi-root aware path resolution
- VS Code URI scheme handling (file, untitled, vscode-userdata, etc.)

These patterns demonstrate how VS Code's search/navigation surface can be abstracted into a modular extension, making it a solid reference for implementing analogous functionality in a Rust-based IDE.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder Report: Search-Result Extension Document Link Providers
## Partition 42/79 - Tauri/Rust Porting Research

**Research Date:** 2026-05-01  
**Focus:** VS Code search-result extension's document-link providers, virtual documents, and search-result-as-document patterns for IDE porting.

---

## Summary

The search-result extension implements a **document-link-first architecture** for search results presentation. Rather than storing results in a database or custom data structure, it parses a textual format (`.code-search` files) and provides IDE services (document links, definitions, symbols, completions) as language server providers. This pattern is critical for understanding how VS Code bridges search/navigation functionality.

**Key Insight:** The extension treats search results as a **virtual document type** with a dedicated language (`search-result`), enabling tight IDE integration without custom UI components.

---

## Pattern 1: Document Link Provider with Parsed Metadata

**Where:** `extensions/search-result/src/extension.ts:100-106`  
**What:** Registers a document link provider that converts parsed search results into clickable file location links.

```typescript
vscode.languages.registerDocumentLinkProvider(SEARCH_RESULT_SELECTOR, {
    async provideDocumentLinks(document: vscode.TextDocument, token: vscode.CancellationToken): Promise<vscode.DocumentLink[]> {
        return parseSearchResults(document, token)
            .filter(isFileLine)
            .map(({ location }) => ({ range: location.originSelectionRange!, target: location.targetUri }));
    }
}),
```

**Variations:**
- Filter by line type (`isFileLine`), ignoring result detail lines
- Returns `DocumentLink[]` with target URIs resolved during parsing
- Integrates with cancellation token for streaming operations

---

## Pattern 2: Definition Provider with Location Link Mapping

**Where:** `extensions/search-result/src/extension.ts:76-98`  
**What:** Provides file definition links with precise character offset mapping within search result lines.

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

**Variations:**
- File-level locations: maps all occurrences (file header line)
- Result-level locations: maps character position relative to match start
- Handles both "file:" and "line  " separator formats (context lines)

---

## Pattern 3: Document Symbol Provider with File Hierarchy

**Where:** `extensions/search-result/src/extension.ts:39-53`  
**What:** Exposes search result files as document symbols for breadcrumb/outline navigation.

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

**Variations:**
- Uses `SymbolKind.File` for file entries
- `selectionRange` points to file name only
- `range` encompasses all match occurrences in that file

---

## Pattern 4: Completion Item Provider for Search Syntax

**Where:** `extensions/search-result/src/extension.ts:55-74`  
**What:** Provides inline syntax completion for search query headers and flags.

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

**Variations:**
- Directives only in first 4 lines (header section)
- Flags only on `# Flags:` line
- Filters already-present suggestions

---

## Pattern 5: Resilient Path-to-URI Resolution

**Where:** `extensions/search-result/src/extension.ts:130-175`  
**What:** Handles multiple path formats (absolute, relative, home-relative, multi-root, special schemes) for document linking.

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
                return uriFromFolderWithPath(folder, workspacePath);
            }
        }
        else if (vscode.workspace.workspaceFolders.length === 1) {
            return uriFromFolderWithPath(vscode.workspace.workspaceFolders[0], path);
        } else if (resultsUri.scheme !== 'untitled') {
            // Try matching saved search to current workspace folder
            const prefixMatch = vscode.workspace.workspaceFolders.filter(wf => resultsUri.toString().startsWith(wf.uri.toString()))[0];
            if (prefixMatch) {
                return uriFromFolderWithPath(prefixMatch, path);
            }
        }
    }

    console.error(`Unable to resolve path ${path}`);
    return undefined;
}
```

**Variations:**
- Settings/profile paths: custom `vscode-userdata` scheme
- Untitled files: `untitled://` scheme with regex matching
- Home directory expansion: `~` prefix substitution
- Multi-root formatting: `"workspace-name • relative/path"` pattern
- Fallback: match search result's workspace folder

---

## Pattern 6: Regex-Based Search Result Parsing

**Where:** `extensions/search-result/src/extension.ts:9-11, 184-277`  
**What:** Parses textual search results using regex to extract file/line/match structure, then caches parsed metadata for provider access.

```typescript
const FILE_LINE_REGEX = /^(\S.*):$/;
const RESULT_LINE_REGEX = /^(\s+)(\d+)(: |  )(\s*)(.*)$/;
const ELISION_REGEX = /⟪ ([0-9]+) characters skipped ⟫/g;

function parseSearchResults(document: vscode.TextDocument, token?: vscode.CancellationToken): ParsedSearchResults {

    if (cachedLastParse && cachedLastParse.uri === document.uri && cachedLastParse.version === document.version) {
        return cachedLastParse.parse;
    }

    const lines = document.getText().split(/\r?\n/);
    const links: ParsedSearchResults = [];

    let currentTarget: vscode.Uri | undefined = undefined;
    let currentTargetLocations: vscode.LocationLink[] | undefined = undefined;

    for (let i = 0; i < lines.length; i++) {
        if (token?.isCancellationRequested) { return []; }
        const line = lines[i];

        const fileLine = FILE_LINE_REGEX.exec(line);
        if (fileLine) {
            const [, path] = fileLine;

            currentTarget = relativePathToUri(path, document.uri);
            if (!currentTarget) { continue; }
            currentTargetLocations = [];

            const location: vscode.LocationLink = {
                targetRange: new vscode.Range(0, 0, 0, 1),
                targetUri: currentTarget,
                originSelectionRange: new vscode.Range(i, 0, i, line.length),
            };

            links[i] = { type: 'file', location, allLocations: currentTargetLocations, path };
        }

        if (!currentTarget) { continue; }

        const resultLine = RESULT_LINE_REGEX.exec(line);
        if (resultLine) {
            const [, indentation, _lineNumber, separator] = resultLine;
            const lineNumber = +_lineNumber - 1;
            const metadataOffset = (indentation + _lineNumber + separator).length;
            const targetRange = new vscode.Range(Math.max(lineNumber - 3, 0), 0, lineNumber + 3, line.length);

            const locations: Required<vscode.LocationLink>[] = [];

            let lastEnd = metadataOffset;
            let offset = 0;
            ELISION_REGEX.lastIndex = metadataOffset;
            for (let match: RegExpExecArray | null; (match = ELISION_REGEX.exec(line));) {
                locations.push({
                    targetRange,
                    targetSelectionRange: new vscode.Range(lineNumber, offset, lineNumber, offset),
                    targetUri: currentTarget,
                    originSelectionRange: new vscode.Range(i, lastEnd, i, ELISION_REGEX.lastIndex - match[0].length),
                });

                offset += (ELISION_REGEX.lastIndex - lastEnd - match[0].length) + Number(match[1]);
                lastEnd = ELISION_REGEX.lastIndex;
            }

            if (lastEnd < line.length) {
                locations.push({
                    targetRange,
                    targetSelectionRange: new vscode.Range(lineNumber, offset, lineNumber, offset),
                    targetUri: currentTarget,
                    originSelectionRange: new vscode.Range(i, lastEnd, i, line.length),
                });
            }
            // only show result lines in file-level peek
            if (separator.includes(':')) {
                currentTargetLocations?.push(...locations);
            }

            // Allow line number, indentation, etc to take you to definition as well.
            const convenienceLocation: Required<vscode.LocationLink> = {
                targetRange,
                targetSelectionRange: new vscode.Range(lineNumber, 0, lineNumber, 1),
                targetUri: currentTarget,
                originSelectionRange: new vscode.Range(i, 0, i, metadataOffset - 1),
            };
            locations.push(convenienceLocation);
            links[i] = { type: 'result', locations, isContext: separator === ' ', prefixRange: new vscode.Range(i, 0, i, metadataOffset) };
        }
    }

    cachedLastParse = {
        version: document.version,
        parse: links,
        uri: document.uri
    };

    return links;
}
```

**Variations:**
- Document-version-aware caching: invalidates when document changes
- Elision support: handles truncated matches with character count metadata
- Lazy evaluation: parses only when providers request data
- Stateful: maintains `currentTarget` URI for accumulating file matches

---

## Pattern 7: Language-Based Provider Registration

**Where:** `extensions/search-result/src/extension.ts:12-13, 20, 37-126`  
**What:** Registers all providers against a **selector** tied to a custom language (`search-result`), enabling selective activation.

```typescript
const SEARCH_RESULT_SELECTOR = { language: 'search-result', exclusive: true };

export function activate(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.languages.registerDocumentSymbolProvider(SEARCH_RESULT_SELECTOR, { ... }),
        vscode.languages.registerCompletionItemProvider(SEARCH_RESULT_SELECTOR, { ... }, '#'),
        vscode.languages.registerDefinitionProvider(SEARCH_RESULT_SELECTOR, { ... }),
        vscode.languages.registerDocumentLinkProvider(SEARCH_RESULT_SELECTOR, { ... }),
        // ...
    );
}
```

**Variations:**
- `exclusive: true`: prevents other providers from handling this language
- Activation trigger: `"onLanguage:search-result"` in package.json
- All providers share same document type, enabling coordinated responses

---

## Pattern 8: Decoration-Based Visual Rendering

**Where:** `extensions/search-result/src/extension.ts:22-35, 108-123`  
**What:** Uses text editor decorations (not custom UI) to highlight result lines and apply formatting.

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

if (vscode.window.activeTextEditor && vscode.window.activeTextEditor.document.languageId === 'search-result') {
    decorate(vscode.window.activeTextEditor);
}

// ... listen to text editor activation and document changes
vscode.window.onDidChangeActiveTextEditor(editor => {
    if (editor?.document.languageId === 'search-result') {
        decorate(editor);
    }
}),
```

**Variations:**
- Context lines (separator `' '`): reduced opacity
- Match lines (separator `':'`): bold font weight
- Decoration ranges: applied to prefix (line number + indentation) only

---

## Architectural Summary for Tauri/Rust Porting

### What Works Well (Replicable)

1. **Provider Registration Model**: Language selector + provider implementation pairs map cleanly to LSP (Language Server Protocol).
2. **Document Type Abstraction**: Treating search results as a language/document type enables standard IDE features without custom UI.
3. **URI Resolution**: Multi-path-format handling is essential for cross-platform searches (settings, untitled, home, multi-root workspaces).
4. **Caching Strategy**: Version-aware document caching reduces re-parsing overhead.
5. **Stateless Parsing**: Regex-based parsing is deterministic and reproducible in Rust.

### Challenges for Porting

1. **Text Document API**: Depends on VS Code's `TextDocument` interface (line splitting, line-at access). Would need to replicate in Rust.
2. **Extension Context/Subscriptions**: Resource management pattern (`context.subscriptions.push()`) maps to LSP lifecycle but requires refactoring.
3. **Dynamic Decoration**: Real-time text editor decoration updates require renderer integration (harder in Tauri with web frontends).
4. **Workspace Context**: Multi-root workspace folder detection and URI joining are platform-specific.
5. **Elision Handling**: Truncation with character count metadata is custom; needs careful translation to match semantics.

### Key Data Structures

```typescript
type ParsedSearchFileLine = { 
    type: 'file'; 
    location: vscode.LocationLink; 
    allLocations: vscode.LocationLink[]; 
    path: string 
};

type ParsedSearchResultLine = { 
    type: 'result'; 
    locations: Required<vscode.LocationLink>[]; 
    isContext: boolean; 
    prefixRange: vscode.Range 
};
```

### File Extensions & Language Definition

- **File extension:** `.code-search`
- **Language ID:** `search-result`
- **Grammar:** TextMate syntax (`searchResult.tmLanguage.json`)
- **Activation:** `onLanguage:search-result`

---

## Related Files

- **Main implementation:** `/extensions/search-result/src/extension.ts` (278 LOC)
- **Package manifest:** `/extensions/search-result/package.json` (capabilities, grammar, language definition)
- **Grammar file:** `/extensions/search-result/syntaxes/searchResult.tmLanguage.json`

---

## References to Similar Patterns

- **Git extension document links:** `extensions/git/src/main.ts` (custom scheme `git-commit`)
- **Merge conflict virtual docs:** `extensions/merge-conflict/src/contentProvider.ts` (TextDocumentContentProvider pattern)
- **TypeScript tsconfig links:** `extensions/typescript-language-features/src/languageFeatures/tsconfig.ts`

---

## Conclusion

The search-result extension exemplifies VS Code's **document-centric architecture**: every feature (navigation, outline, completion) is a provider attached to a language. Porting to Tauri/Rust requires:

1. Implementing a parallel LSP server that parses `.code-search` documents and returns provider responses.
2. Replicating URI resolution logic for multi-platform path handling.
3. Handling decorations via renderer CSS/DOM instead of editor API.
4. Mapping document version tracking to LSP text synchronization.

The textual format itself (regex-based) is a major strength, as it's inherently language-agnostic and can be re-parsed without database overhead.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
