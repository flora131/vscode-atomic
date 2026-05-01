# Partition 33 of 79 — Findings

## Scope
`extensions/configuration-editing/` (11 files, 1,450 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Configuration Editing Extension: Core IDE Porting Surface Analysis

## Summary
The `configuration-editing` extension implements VS Code's settings-aware IDE completion system through the `vscode.languages.registerCompletionItemProvider()` API. Porting to Tauri/Rust would require reimplementing 5 completion provider registrations that touch sensitive configuration surfaces (settings.json, extensions.json, launch.json, tasks.json, keybindings.json), plus the underlying JSON/JSONC parsing and language services integration layer.

---

## Implementation
- `extensions/configuration-editing/src/configurationEditingMain.ts` — Extension activation & 5 completion provider registrations (settings, variables, extensions, context keys)
- `extensions/configuration-editing/src/settingsDocumentHelper.ts` — Settings-aware completion logic; handles window.title, files.associations, excludes, language overrides, ports (SettingsDocument class)
- `extensions/configuration-editing/src/extensionsProposals.ts` — Extension recommendation proposals from installed extensions
- `extensions/configuration-editing/src/importExportProfiles.ts` — GitHub Gist profile import/export (ProfileContentHandler interface, Octokit integration)
- `extensions/configuration-editing/src/node/net.ts` — HTTPS proxy agent handling for Node.js
- `extensions/configuration-editing/src/browser/net.ts` — Browser stub for net module

## Tests
- `extensions/configuration-editing/src/test/completion.test.ts` — Mocha integration tests for completion in settings.json (window.title, language overrides, exclude patterns)
- `extensions/configuration-editing/src/test/index.ts` — Test runner configuration with mocha-junit-reporter integration

## Types / Interfaces
- `SettingsDocument` class — Parses JSONC and provides context-aware completions
- `IExtensionsContent` interface — Shape for extensions.json document structure
- `ItemDescription` interface (test) — Completion result verification helper
- `ContextKeyInfo` interface — Context key metadata (key, type, description)

## Configuration
- `extensions/configuration-editing/package.json` — Extension manifest; registers JSONC language for settings.json, launch.json, tasks.json, keybindings.json, extensions.json, .code-workspace files; contributes jsonValidation for 28+ file patterns
- `extensions/configuration-editing/tsconfig.json` — TypeScript config; includes vscode.d.ts and vscode.proposed.profileContentHandlers.d.ts
- `extensions/configuration-editing/tsconfig.browser.json` — Browser build variant
- `extensions/configuration-editing/esbuild.mts` & `esbuild.browser.mts` — Build pipelines for Node.js and browser targets
- `extensions/configuration-editing/.npmrc` & `extensions/configuration-editing/package-lock.json` — npm dependencies (jsonc-parser, @octokit/rest, tunnel)

## Notable Clusters
- `extensions/configuration-editing/schemas/` — 3 schema files for devContainer.json variants (vscode-specific, codespaces-specific, attachContainer configs); external sourced from devcontainers/spec repository

---

## Porting Requirements

### Core API Dependencies
The extension fundamentally depends on:
1. **vscode.languages.registerCompletionItemProvider()** — Pattern-based language selector (language + glob pattern)
2. **vscode.extensions.all** — Enumerable extension registry
3. **vscode.commands.executeCommand()** — Command dispatch for 'getContextKeyInfo'
4. **vscode.workspace.openTextDocument()** — File I/O
5. **vscode.l10n.t()** — Localization/i18n layer

### Configuration Surface Coverage
- **settings.json**: 8 completion contexts (window.title, files.associations, files.exclude, search.exclude, explorer.autoRevealExclude, files.defaultLanguage, workbench.editor.label.patterns, settingsSync.ignoredExtensions, remote.extensionKind, remote.portsAttributes)
- **extensions.json** & **.code-workspace**: Extension ID completion
- **launch.json** & **tasks.json**: ${variable} expansion for paths, workspace context
- **keybindings.json** & **package.json**: Context key (when) completions

### Key Implementation Details
- **JSONC Parsing**: Heavily relies on `jsonc-parser` library (3.2.0+) for location tracking and AST traversal
- **Language-Aware**: Uses vscode.languages.match() for multi-document-type support
- **Async**: All completion providers are async; requires getContextKeyInfo command that blocks on external data
- **Localization**: All user-facing strings use vscode.l10n.t() (proposed API)
- **Platform Split**: Proxy handling differs between node (full HTTPS proxy with auth) and browser (no-op)

### Extension Context Requirement
- Extension requires the 'profileContentHandlers' proposal API
- Activation events: onProfile, onProfile:github, onLanguage:json, onLanguage:jsonc

---

## Porting Complexity Assessment

**High-Impact Areas:**
- Completion provider registration system (needs Tauri RPC-style language service binding)
- JSONC location parsing and context awareness (jsonc-parser porting/rewrite)
- Extension enumeration API (requires Tauri plugin for installed extensions manifest)
- Command dispatch system (vscode.commands.executeCommand must map to Rust backend)
- GitHub Gist profile sync (Octokit -> reqwest + GitHub API)

**Lower-Impact:**
- Test infrastructure (mocha can run in Node.js test harness regardless)
- Schema validation (external refs can be downloaded or embedded)

Porting would require ~8-12 weeks of work to implement equivalent Rust services for language completions, extension registry access, and configuration document parsing.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: Porting VS Code Configuration Editing to Tauri/Rust

## Research Question
What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust, specifically looking at settings-aware completion patterns in the configuration-editing extension.

## Scope
- `extensions/configuration-editing/` (720 LOC across 4 main files)
- Seed: `vscode.languages.registerCompletionItemProvider($SEL, $$$)` — Settings-aware completion touches the configuration service surface.

---

## Pattern Examples Found

#### Pattern 1: Document Filter-Based Completion Registration
**Where:** `configurationEditingMain.ts:32-37`
**What:** Settings completion provider uses language + glob pattern filtering to scope completions to settings.json files.

```typescript
function registerSettingsCompletions(): vscode.Disposable {
	return vscode.languages.registerCompletionItemProvider({ language: 'jsonc', pattern: '**/settings.json' }, {
		provideCompletionItems(document, position, token) {
			return new SettingsDocument(document).provideCompletionItems(position, token);
		}
	});
}
```

**Variations / call-sites:** 
- `configurationEditingMain.ts:40-82` — Variable completions for launch.json, tasks.json, .code-workspace (parametrized pattern)
- `configurationEditingMain.ts:109-120` — Extensions document completions (pattern-only filter)
- `configurationEditingMain.ts:123-134` — Workspace configuration extensions (pattern-only filter)

---

#### Pattern 2: Multi-Filter Completion with Path Matching
**Where:** `configurationEditingMain.ts:183-243`
**What:** Context key completions register against multiple DocumentFilters and validate location paths using JSON AST matching.

```typescript
function registerContextKeyCompletions(): vscode.Disposable {
	type ContextKeyInfo = { key: string; type?: string; description?: string };

	const paths = new Map<vscode.DocumentFilter, JSONPath[]>([
		[{ language: 'jsonc', pattern: '**/keybindings.json' }, [
			['*', 'when']
		]],
		[{ language: 'json', pattern: '**/package.json' }, [
			['contributes', 'menus', '*', '*', 'when'],
			['contributes', 'views', '*', '*', 'when'],
			['contributes', 'viewsWelcome', '*', 'when'],
			['contributes', 'keybindings', '*', 'when'],
			['contributes', 'keybindings', 'when'],
		]]
	]);

	return vscode.languages.registerCompletionItemProvider(
		[...paths.keys()],
		{
			async provideCompletionItems(document: vscode.TextDocument, position: vscode.Position, token: vscode.CancellationToken) {
				const location = getLocation(document.getText(), document.offsetAt(position));
				
				let isValidLocation = false;
				for (const [key, value] of paths) {
					if (vscode.languages.match(key, document)) {
						if (value.some(location.matches.bind(location))) {
							isValidLocation = true;
							break;
						}
					}
				}
				
				if (!isValidLocation || !isCompletingInsidePropertyStringValue(document, location, position)) {
					return;
				}
				// ...completion logic
			}
		}
	);
}
```

**Variations / call-sites:** Core pattern for multi-document, multi-location completion scenarios.

---

#### Pattern 3: JSON AST-Based Location Detection
**Where:** `settingsDocumentHelper.ts:16-69`
**What:** Settings document provider dispatches completions based on JSON path from JSONC parser location, enabling context-aware suggestions.

```typescript
public async provideCompletionItems(position: vscode.Position, _token: vscode.CancellationToken): Promise<vscode.CompletionItem[] | vscode.CompletionList> {
	const location = getLocation(this.document.getText(), this.document.offsetAt(position));

	// window.title
	if (location.path[0] === 'window.title') {
		return this.provideWindowTitleCompletionItems(location, position);
	}

	// files.association
	if (location.path[0] === 'files.associations') {
		return this.provideFilesAssociationsCompletionItems(location, position);
	}

	// files.exclude, search.exclude, explorer.autoRevealExclude
	if (location.path[0] === 'files.exclude' || location.path[0] === 'search.exclude' || location.path[0] === 'explorer.autoRevealExclude') {
		return this.provideExcludeCompletionItems(location, position);
	}

	// files.defaultLanguage
	if (location.path[0] === 'files.defaultLanguage') {
		return this.provideLanguageCompletionItems(location, position);
	}

	// workbench.editor.label
	if (location.path[0] === 'workbench.editor.label.patterns') {
		return this.provideEditorLabelCompletionItems(location, position);
	}

	// settingsSync.ignoredExtensions
	if (location.path[0] === 'settingsSync.ignoredExtensions') {
		let ignoredExtensions = [];
		try {
			ignoredExtensions = parse(this.document.getText())['settingsSync.ignoredExtensions'];
		} catch (e) {/* ignore error */ }
		const range = this.getReplaceRange(location, position);
		return provideInstalledExtensionProposals(ignoredExtensions, '', range, true);
	}

	// remote.extensionKind
	if (location.path[0] === 'remote.extensionKind' && location.path.length === 2 && location.isAtPropertyKey) {
		let alreadyConfigured: string[] = [];
		try {
			alreadyConfigured = Object.keys(parse(this.document.getText())['remote.extensionKind']);
		} catch (e) {/* ignore error */ }
		const range = this.getReplaceRange(location, position);
		return provideInstalledExtensionProposals(alreadyConfigured, location.previousNode ? '' : `: [\n\t"ui"\n]`, range, true);
	}

	return this.provideLanguageOverridesCompletionItems(location, position);
}
```

**Variations / call-sites:** Each setting path branches to specialized provider methods (`provideWindowTitleCompletionItems`, `provideFilesAssociationsCompletionItems`, etc.)

---

#### Pattern 4: Snippet-Based Completion Items
**Where:** `settingsDocumentHelper.ts:355-362`
**What:** Completion items wrap SnippetString insertText for templated code generation with placeholders.

```typescript
private newSnippetCompletionItem(o: { label: string; documentation?: string; snippet: string; range: vscode.Range }): vscode.CompletionItem {
	const item = new vscode.CompletionItem(o.label);
	item.kind = vscode.CompletionItemKind.Value;
	item.documentation = o.documentation;
	item.insertText = new vscode.SnippetString(o.snippet);
	item.range = o.range;
	return item;
}
```

**Example usage** — `settingsDocumentHelper.ts:195-215` (provideExcludeCompletionItems):

```typescript
completions.push(this.newSnippetCompletionItem({
	label: vscode.l10n.t("Files by Extension"),
	documentation: vscode.l10n.t("Match all files of a specific file extension."),
	snippet: location.path.length === 2 ? '"**/*.${1:extension}": true' : '{ "**/*.${1:extension}": true }',
	range
}));
```

**Variations / call-sites:** 
- `settingsDocumentHelper.ts:196-237` — Multiple snippet templates for glob patterns
- `settingsDocumentHelper.ts:321-344` — Port attribute snippets with nested structures
- `configurationEditingMain.ts:73-75` — Dynamic SnippetString building with `appendPlaceholder()` and `appendText()`

---

#### Pattern 5: Extension Proposal Provider (Dynamic Data)
**Where:** `extensionsProposals.ts:9-32`
**What:** Completion provider filters runtime installed extensions against configured lists and synthesizes completion items.

```typescript
export async function provideInstalledExtensionProposals(existing: string[], additionalText: string, range: vscode.Range, includeBuiltinExtensions: boolean): Promise<vscode.CompletionItem[] | vscode.CompletionList> {
	if (Array.isArray(existing)) {
		const extensions = includeBuiltinExtensions ? vscode.extensions.all : vscode.extensions.all.filter(e => !(e.id.startsWith('vscode.') || e.id === 'Microsoft.vscode-markdown'));
		const knownExtensionProposals = extensions.filter(e => existing.indexOf(e.id) === -1);
		if (knownExtensionProposals.length) {
			return knownExtensionProposals.map(e => {
				const item = new vscode.CompletionItem(e.id);
				const insertText = `"${e.id}"${additionalText}`;
				item.kind = vscode.CompletionItemKind.Value;
				item.insertText = insertText;
				item.range = range;
				item.filterText = insertText;
				return item;
			});
		} else {
			const example = new vscode.CompletionItem(vscode.l10n.t("Example"));
			example.insertText = '"vscode.csharp"';
			example.kind = vscode.CompletionItemKind.Value;
			example.range = range;
			return [example];
		}
	}
	return [];
}
```

**Variations / call-sites:**
- `settingsDocumentHelper.ts:45-51` — settingsSync.ignoredExtensions list
- `settingsDocumentHelper.ts:54-61` — remote.extensionKind object keys
- `configurationEditingMain.ts:109-120` — extensions.json recommendations
- `configurationEditingMain.ts:123-134` — Workspace configuration extensions

---

#### Pattern 6: Conditional Range Calculation
**Where:** `settingsDocumentHelper.ts:72-81`
**What:** Range determination logic checks if position overlaps a previousNode and calculates appropriate insertion/replacement bounds.

```typescript
private getReplaceRange(location: Location, position: vscode.Position) {
	const node = location.previousNode;
	if (node) {
		const nodeStart = this.document.positionAt(node.offset), nodeEnd = this.document.positionAt(node.offset + node.length);
		if (nodeStart.isBeforeOrEqual(position) && nodeEnd.isAfterOrEqual(position)) {
			return new vscode.Range(nodeStart, nodeEnd);
		}
	}
	return new vscode.Range(position, position);
}
```

**Variations / call-sites:**
- `configurationEditingMain.ts:137-145` — Shared utility version with identical logic
- Used throughout SettingsDocument for range-aware completions in properties like window.title, files.associations, etc.

---

#### Pattern 7: Variable String Interpolation with Conditional Formatting
**Where:** `settingsDocumentHelper.ts:107-109` and `147-148`
**What:** Completion text formatting adapts based on JSON context (property value vs. literal).

```typescript
const getText = (variable: string) => {
	const text = '${' + variable + '}';
	return location.previousNode ? text : JSON.stringify(text);
};

completions.push(this.newSimpleCompletionItem(getText('activeEditorShort'), range, vscode.l10n.t("the file name (e.g. myFile.txt)")));
```

**Variations / call-sites:**
- `settingsDocumentHelper.ts:146-149` — Editor label pattern variables use same formatting logic
- `configurationEditingMain.ts:72-77` — Variable completion label formatting with parameter binding

---

## Integration Points & Dependencies

### Language Server Protocol Surface
1. **vscode.languages.registerCompletionItemProvider** — Registration of provider with document filters (language + glob patterns)
2. **vscode.languages.match** — Runtime document filter matching against active documents
3. **vscode.languages.getLanguages** — Dynamic language list retrieval for language-aware completions
4. **vscode.extensions.all** — Runtime extension enumeration for extension recommendations

### JSON/JSONC Parsing
1. **jsonc-parser.getLocation()** — AST location from document offset (jsonc-parser library)
2. **jsonc-parser.parse()** — Full document parsing for data extraction (remote.extensionKind, settingsSync.ignoredExtensions)
3. **jsonc-parser.visit()** — Visitor pattern for symbol extraction (launch.json symbol provider at line 148)
4. **Location.path** — JSONPath array for hierarchical position detection
5. **Location.matches()** — Path matching against JSONPath patterns

### Document Model
1. **vscode.TextDocument.getText()** — Full document content retrieval for parsing
2. **vscode.TextDocument.offsetAt()** — Position-to-offset conversion for AST lookup
3. **vscode.TextDocument.positionAt()** — Offset-to-position conversion for range calculation
4. **vscode.TextDocument.getWordRangeAtPosition()** — Regex-based word extraction for variable completions

### Completion Item API
1. **vscode.CompletionItem** — Completion item container
2. **vscode.CompletionItemKind** — Item kind (Value, Property, Constant, etc.)
3. **vscode.SnippetString** — Snippet template with placeholders and escape sequences
4. **range** property — Insertion/replacement range (can be single range or {replacing, inserting})
5. **filterText** property — Custom filter text for sorting/matching

### Localization
1. **vscode.l10n.t()** — String localization helper used throughout for user-facing text

---

## Porting Complexity Assessment

### High Complexity Areas (Requiring Substantial Rewrite)
1. **Document Filter System** — Tauri lacks native `DocumentFilter` with language+glob matching. Would need custom document tracking system and glob pattern matching library.
2. **JSON AST Location Tracking** — Requires JSONC parser equivalent in Rust (jsonc-parser has no Rust counterpart; would need to use `serde_json` + custom AST traversal).
3. **Localization Integration** — vscode.l10n system is Electron-specific; would need independent i18n framework.
4. **Runtime Extension Registry** — vscode.extensions.all requires VS Code's extension host; Tauri would need different plugin/extension discovery mechanism.
5. **SnippetString Rendering** — VS Code's snippet syntax (${1:placeholder} with nested placeholders) is VS Code-specific and would need custom rendering engine.

### Medium Complexity Areas (Adapable with Effort)
1. **Completion Item Provider Pattern** — Can be replicated with Tauri + custom LSP server implementation.
2. **Path-Based Dispatching** — JSONPath-style navigation can be replicated with Rust enum matching on JSON structure.
3. **Range Calculation Logic** — Position/offset conversion is standard in text editors; Rust equivalents exist.
4. **Snippet Templates** — Static templates can be ported; dynamic building would need custom DSL or template engine.

### Lower Complexity Areas (Direct Port)
1. **Conditional Logic** — Path matching, position validation, filtering logic translate directly to Rust pattern matching.
2. **Type Definitions** — Rust structs/enums can replicate SettingsDocument dispatch pattern.
3. **Data Structures** — Maps, arrays, string operations are equivalent in Rust.

---

## Key Technical Challenges for Tauri/Rust

1. **LSP Server Integration** — Would need to embed or communicate with an LSP server (likely tower-lsp crate) rather than using VS Code's native API.
2. **Document Synchronization** — Custom protocol needed to track document changes, positions, and maintain AST state between editor and completion server.
3. **File Pattern Matching** — Glob pattern matching logic (e.g., `**/settings.json`) would need globset or similar crate.
4. **JSONC Parsing** — No direct Rust equivalent to jsonc-parser; would need jsonc crate + custom Location abstraction.
5. **Snippet Processing** — Would need to implement or integrate snippet syntax parser/renderer (VS Code snippet format is non-standard).
6. **Extension/Plugin Discovery** — Tauri plugins ≠ VS Code extensions; different model requires rethinking extension recommendation system.

---

## Code Characteristics Enabling Porting

✓ **Modular Provider Pattern** — Clear separation of concerns (registerSettingsCompletions, registerVariableCompletions, etc.) makes phased porting feasible.

✓ **Pure Data Flow** — Completion providers are mostly pure functions (position → completions), reducing state management burden.

✓ **Stateless Dispatch** — Location-based dispatching requires no mutable state, simplifying concurrent completion requests.

✓ **Regex-Based Word Extraction** — Simple regex patterns for token boundaries can be ported unchanged.

✗ **Tight VS Code API Coupling** — Assumes vscode.languages, vscode.extensions, vscode.l10n are available; no abstraction layer for alternate implementations.

✗ **Async/Promise-Heavy** — Some providers return async results (getLanguages(), executeCommand()); Rust async/await would need careful integration with Tauri event system.

---

## Testing Patterns Found

**File:** `test/completion.test.ts:545-594`

Test framework uses VS Code's built-in test harness:
1. Write test content to temporary file with `|` cursor marker
2. Open file and set language via `vscode.languages.setTextDocumentLanguage()`
3. Execute `vscode.executeCompletionItemProvider` command at cursor position
4. Assert completion items by label matching
5. For result text validation, apply insertion to document and verify content

```typescript
async function testCompletion(testFileName: string, languageId: string, content: string, expected: ItemDescription) {
	const offset = content.indexOf('|');
	content = content.substring(0, offset) + content.substring(offset + 1);

	const docUri = vscode.Uri.file(path.join(await testFolder, testFileName));
	await fs.writeFile(docUri.fsPath, content);

	const editor = await setTestContent(docUri, languageId, content);
	const position = editor.document.positionAt(offset);

	const actualCompletions = (await vscode.commands.executeCommand('vscode.executeCompletionItemProvider', docUri, position)) as vscode.CompletionList;

	const matches = actualCompletions.items.filter(completion => {
		return completion.label === expected.label;
	});
	if (expected.notAvailable) {
		assert.strictEqual(matches.length, 0);
	} else {
		assert.strictEqual(matches.length, 1);
		if (expected.resultText && matches[0].range && matches[0].insertText) {
			const range = matches[0].range instanceof vscode.Range ? matches[0].range : matches[0].range.replacing;
			const text = typeof matches[0].insertText === 'string' ? matches[0].insertText : matches[0].insertText.value;
			await editor.edit(eb => eb.replace(range, text));
			assert.strictEqual(editor.document.getText(), expected.resultText);
		}
	}
}
```

Tests cover 6 scenarios: position within text, before variables, after variables, partial variable replacement, literal insertion, and exclusion cases.

---

## Summary

The configuration-editing extension demonstrates a **highly specialized IDE feature** that deeply integrates with VS Code's language service infrastructure. Porting to Tauri/Rust would require:

1. **Re-architecting the completion pipeline** to work with an external LSP server or Tauri-native plugin system rather than VS Code's registration API.
2. **Implementing or integrating JSONC parsing and AST location tracking** — a non-trivial component in Rust.
3. **Substituting the document filter/matching system** with custom glob-based file association logic.
4. **Building a snippet processor** for inserting templated code with placeholders.
5. **Redesigning extension/language discovery** to work within Tauri's plugin model rather than VS Code's extension host.

The logic itself is cleanly separated and would port well, but the **integration surface with VS Code's language services is the primary blocker**. This is a foundational feature that would need extensive rearchitecting, not a straightforward translation.

**Estimated Effort:** 6-12 weeks for a feature-complete port, assuming:
- LSP server development experience
- Rust ecosystem familiarity (jsonc, tokio, tower-lsp)
- Tauri plugin architecture understanding
- Custom snippet syntax implementation

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
