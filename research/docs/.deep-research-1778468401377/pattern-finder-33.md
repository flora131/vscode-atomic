# VS Code Configuration-Editing Extension: Pattern Analysis

**Scope**: `extensions/configuration-editing/` (1,397 LOC across 9 TypeScript files)
**Task**: Document patterns for porting VS Code core IDE functionality from TypeScript/Electron to Tauri/Rust

## Key Finding

The configuration-editing extension demonstrates core VS Code IDE patterns through:
1. **Provider registration pattern** - Language services registering completion/symbol providers
2. **Authentication integration** - External service integration (GitHub gists)
3. **Document analysis pattern** - JSONC parsing for contextual completions
4. **Command execution pattern** - Querying context from core via command API

These patterns are critical for porting as they show how extensions interact with VS Code's core systems.

---

## Patterns Found

#### Pattern: Language Service Provider Registration (Completion Items)

**Where:** `extensions/configuration-editing/src/configurationEditingMain.ts:32-37`

**What:** Core pattern for registering document completion providers with language + file pattern matching.

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
- `configurationEditingMain.ts:40-82` - Variable completions in launch.json/tasks.json with pattern-based routing
- `configurationEditingMain.ts:109-120` - Extensions completions in extensions.json
- `configurationEditingMain.ts:123-134` - Workspace config completions in .code-workspace files
- `configurationEditingMain.ts:199-243` - Context key completions with DocumentFilter map matching multiple file patterns

**Key aspects:**
- Disposable pattern for lifecycle management
- DocumentFilter with language + pattern matching
- Async provider interface with cancellation token support

---

#### Pattern: Document Symbol Provider Registration

**Where:** `extensions/configuration-editing/src/configurationEditingMain.ts:148-181`

**What:** Registering document symbol providers for outline/navigation in structured JSON files.

```typescript
vscode.languages.registerDocumentSymbolProvider({ pattern: '**/launch.json', language: 'jsonc' }, {
	provideDocumentSymbols(document: vscode.TextDocument, _token: vscode.CancellationToken): vscode.ProviderResult<vscode.SymbolInformation[]> {
		const result: vscode.SymbolInformation[] = [];
		let name: string = '';
		let lastProperty = '';
		let startOffset = 0;
		let depthInObjects = 0;

		visit(document.getText(), {
			onObjectProperty: (property, _offset, _length) => {
				lastProperty = property;
			},
			onLiteralValue: (value: any, _offset: number, _length: number) => {
				if (lastProperty === 'name') {
					name = value;
				}
			},
			onObjectBegin: (offset: number, _length: number) => {
				depthInObjects++;
				if (depthInObjects === 2) {
					startOffset = offset;
				}
			},
			onObjectEnd: (offset: number, _length: number) => {
				if (name && depthInObjects === 2) {
					result.push(new vscode.SymbolInformation(name, vscode.SymbolKind.Object, new vscode.Range(document.positionAt(startOffset), document.positionAt(offset))));
				}
				depthInObjects--;
			},
		});

		return result;
	}
}, { label: 'Launch Targets' });
```

**Variations / call-sites:**
- Only one explicit call-site but establishes outline navigation pattern

**Key aspects:**
- Uses jsonc-parser's visitor pattern for AST traversal
- State machine tracking (depth, name, position)
- Symbol kind classification
- Outline label customization

---

#### Pattern: Context-Aware Completion with Multi-File Pattern Matching

**Where:** `extensions/configuration-editing/src/configurationEditingMain.ts:183-243`

**What:** Advanced completion provider pattern that matches multiple file patterns and uses location path for context routing.

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

				if (location.isAtPropertyKey) {
					return;
				}

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

				const replacing = document.getWordRangeAtPosition(position, /[a-zA-Z.]+/) || new vscode.Range(position, position);
				const inserting = replacing.with(undefined, position);

				const data = await vscode.commands.executeCommand<ContextKeyInfo[]>('getContextKeyInfo');
				if (token.isCancellationRequested || !data) {
					return;
				}

				const result = new vscode.CompletionList();
				for (const item of data) {
					const completion = new vscode.CompletionItem(item.key, vscode.CompletionItemKind.Constant);
					completion.detail = item.type;
					completion.range = { replacing, inserting };
					completion.documentation = item.description;
					result.items.push(completion);
				}
				return result;
			}
		}
	);
}
```

**Variations / call-sites:**
- Core pattern used across multiple file types (keybindings.json, package.json)

**Key aspects:**
- Map-based routing for multiple document types
- JSONPath matching for nested property validation
- Async data fetch via command API (core integration)
- Range tracking for selective replacement vs insertion
- Cancellation token checking for user interruption

---

#### Pattern: Settings Document Helper Class with Path-Based Routing

**Where:** `extensions/configuration-editing/src/settingsDocumentHelper.ts:12-69`

**What:** Object-oriented document analysis pattern routing completions based on JSON path analysis.

```typescript
export class SettingsDocument {

	constructor(private document: vscode.TextDocument) { }

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

**Variations / call-sites:**
- `settingsDocumentHelper.ts:253-262` - Language completion retrieval via `vscode.languages.getLanguages()`
- `settingsDocumentHelper.ts:265-275` - Language override completion items generation
- `settingsDocumentHelper.ts:277-319` - Language override range detection with regex matching

**Key aspects:**
- Cascade-based routing on path[0] (first-level key)
- State extraction and caching from document text
- Try-catch error handling for malformed JSON
- Composition with helper functions

---

#### Pattern: Extension API Integration - Installed Extensions Query

**Where:** `extensions/configuration-editing/src/extensionsProposals.ts:9-31`

**What:** Querying installed extensions from VS Code core and generating completion items.

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
- `settingsDocumentHelper.ts:51` - Ignored extensions filtering
- `settingsDocumentHelper.ts:61` - Remote extension kind configuration
- `configurationEditingMain.ts:116` - Extensions.json recommendations
- `configurationEditingMain.ts:130` - Workspace extensions recommendations

**Key aspects:**
- Access to `vscode.extensions.all` for runtime extension enumeration
- Filtering logic (builtin vs user extensions, already-configured)
- Fallback to example when no proposals available
- Template customization via additionalText parameter

---

#### Pattern: Authentication Integration - GitHub Session Management

**Where:** `extensions/configuration-editing/src/importExportProfiles.ts:11-33`

**What:** Lazy-initialized authentication session management for external API integration.

```typescript
class GitHubGistProfileContentHandler implements vscode.ProfileContentHandler {

	readonly name = vscode.l10n.t('GitHub');
	readonly description = vscode.l10n.t('gist');

	private _octokit: Promise<Octokit> | undefined;
	private getOctokit(): Promise<Octokit> {
		if (!this._octokit) {
			this._octokit = (async () => {
				const session = await vscode.authentication.getSession('github', ['gist', 'user:email'], { createIfNone: true });
				const token = session.accessToken;

				const { Octokit } = await import('@octokit/rest');

				return new Octokit({
					request: { agent },
					userAgent: 'GitHub VSCode',
					auth: `token ${token}`
				});
			})();
		}
		return this._octokit;
	}

	async saveProfile(name: string, content: string): Promise<{ readonly id: string; readonly link: vscode.Uri } | null> {
		const octokit = await this.getOctokit();
		const result = await octokit.gists.create({
			public: false,
			files: {
				[name]: {
					content
				}
			}
		});
		if (result.data.id && result.data.html_url) {
			const link = vscode.Uri.parse(result.data.html_url);
			return { id: result.data.id, link };
		}
		return null;
	}
```

**Variations / call-sites:**
- `importExportProfiles.ts:52-60` - Public Octokit initialization (no auth required)
- `importExportProfiles.ts:81` - Profile content handler registration

**Key aspects:**
- Lazy initialization pattern (Promise memoization)
- Authentication provider abstraction (`vscode.authentication.getSession`)
- Session creation on demand (`createIfNone: true`)
- HTTP agent configuration for proxy support
- Dynamic module import for external dependencies
- Error handling with fallback to public access

---

#### Pattern: Test Framework Integration and Completion Testing

**Where:** `extensions/configuration-editing/src/test/completion.test.ts:545-580`

**What:** Testing completion providers with document content simulation and command API invocation.

```typescript
async function testCompletion(testFileName: string, languageId: string, content: string, expected: ItemDescription) {

	const offset = content.indexOf('|');
	content = content.substring(0, offset) + content.substring(offset + 1);

	const docUri = vscode.Uri.file(path.join(await testFolder, testFileName));
	await fs.writeFile(docUri.fsPath, content);

	const editor = await setTestContent(docUri, languageId, content);
	const position = editor.document.positionAt(offset);

	// Executing the command `vscode.executeCompletionItemProvider` to simulate triggering completion
	const actualCompletions = (await vscode.commands.executeCommand('vscode.executeCompletionItemProvider', docUri, position)) as vscode.CompletionList;

	const matches = actualCompletions.items.filter(completion => {
		return completion.label === expected.label;
	});
	if (expected.notAvailable) {
		assert.strictEqual(matches.length, 0, `${expected.label} should not existing is results`);
	} else {
		assert.strictEqual(matches.length, 1, `${expected.label} should only existing once: Actual: ${actualCompletions.items.map(c => c.label).join(', ')}`);

		if (expected.resultText) {
			const match = matches[0];
			if (match.range && match.insertText) {
				const range = match.range instanceof vscode.Range ? match.range : match.range.replacing;
				const text = typeof match.insertText === 'string' ? match.insertText : match.insertText.value;

				await editor.edit(eb => eb.replace(range, text));
				assert.strictEqual(editor.document.getText(), expected.resultText);
			} else {
				assert.fail(`Range or insertText missing`);
			}
		}
	}
}

async function setTestContent(docUri: vscode.Uri, languageId: string, content: string): Promise<vscode.TextEditor> {
	const ext = vscode.extensions.getExtension('vscode.configuration-editing')!;
	await ext.activate();

	const doc = await vscode.workspace.openTextDocument(docUri);
	await vscode.languages.setTextDocumentLanguage(doc, languageId);
	const editor = await vscode.window.showTextDocument(doc);

	const fullRange = new vscode.Range(new vscode.Position(0, 0), doc.positionAt(doc.getText().length));
	await editor.edit(eb => eb.replace(fullRange, content));
	return editor;
}
```

**Variations / call-sites:**
- Test suites covering: settings.json (line 16), extensions.json (296), launch.json (340), tasks.json (428), keybindings.json (486)
- Extended test cases: window.title (19), files.associations (115), files.exclude (200), files.defaultLanguage (238), remote.extensionKind (268), remote.portsAttributes (281)

**Key aspects:**
- Marker-based position specification using `|` character
- Temporary file creation in temp directory
- Extension activation pattern via `vscode.extensions.getExtension()`
- Document API simulation with `openTextDocument`, `showTextDocument`
- Command execution pattern for provider testing
- Completion item range handling (replacing vs inserting)
- Snippet string handling

---

## Porting Considerations Summary

### Core API Dependencies Identified

1. **Language Services** (`vscode.languages.*`)
   - `registerCompletionItemProvider` - Must be ported to Tauri completion service
   - `registerDocumentSymbolProvider` - Core for outline/navigation
   - `getLanguages()` - Runtime language enumeration
   - `setTextDocumentLanguage()` - Document language override
   - `match()` - DocumentFilter matching logic

2. **Command System** (`vscode.commands.*`)
   - `executeCommand()` - IPC to core for data fetching (context keys, completions)
   - Commands as RPC interface between extension and core

3. **Extension System** (`vscode.extensions.*`)
   - `all` property - Extension enumeration
   - `getExtension()` - Extension lookup for activation

4. **Authentication** (`vscode.authentication.*`)
   - `getSession()` - OAuth flow integration with `createIfNone` option
   - Provider-based architecture for multiple auth backends

5. **Window/UI** (`vscode.window.*`)
   - `registerProfileContentHandler()` - Profile persistence abstraction
   - `showTextDocument()` - Editor UI integration

6. **Workspace** (`vscode.workspace.*`)
   - `openTextDocument()` - Document buffer creation

7. **URI Handling** (`vscode.Uri.*`)
   - `parse()`, `file()` - URI creation and manipulation

### Key Implementation Patterns for Porting

1. **Provider Registration**: Disposable-based lifecycle with DocumentFilter matching
2. **Path-Based Routing**: JSONPath matching for context-aware completions
3. **Async/Await**: Heavy use of async patterns for I/O operations
4. **Error Handling**: Try-catch with silent failures for malformed JSON
5. **Lazy Initialization**: Promise memoization for expensive operations
6. **Visitor Pattern**: AST traversal with jsonc-parser for document analysis
7. **Filtering/Mapping**: Functional composition of completion item arrays
8. **Test Simulation**: Command API as primary test interface

