# Partition 33 of 79 — Findings

## Scope
`extensions/configuration-editing/` (11 files, 1,450 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locator Report: Configuration-Editing Extension

## Scope
`extensions/configuration-editing/` — 11 files, approximately 1,450 LOC

## Implementation

### Core Extension Files
- `extensions/configuration-editing/src/configurationEditingMain.ts` — Main extension activation and completion providers for settings.json, extensions.json, launch.json, tasks.json, and keybindings.json. Registers six completion item providers via `vscode.languages.registerCompletionItemProvider()` with language/pattern filters.
- `extensions/configuration-editing/src/settingsDocumentHelper.ts` — SettingsDocument class providing context-aware completion suggestions for IDE configuration settings (window.title, files.associations, files.exclude, files.defaultLanguage, workbench.editor.label.patterns, settingsSync.ignoredExtensions, remote.extensionKind, remote.portsAttributes). Uses jsonc-parser to parse JSON/JSONC and resolve location paths.
- `extensions/configuration-editing/src/extensionsProposals.ts` — Helper function provideInstalledExtensionProposals() that generates completion items from installed/built-in extensions for extension recommendation fields.
- `extensions/configuration-editing/src/importExportProfiles.ts` — GitHub Gist profile content handler implementing vscode.ProfileContentHandler interface for saving/loading VS Code profiles via GitHub authentication and Octokit API.

### Platform-Specific Networking
- `extensions/configuration-editing/src/node/net.ts` — Node.js implementation: getAgent() function returning HTTPS agent with optional proxy support via tunnel library, reads HTTPS_PROXY environment variable.
- `extensions/configuration-editing/src/browser/net.ts` — Browser implementation: exports undefined agent (no proxy support in browser context).

## Tests

### Integration Test Suite
- `extensions/configuration-editing/src/test/completion.test.ts` — Mocha-based completion tests covering window.title, files.associations, files.exclude, remote.extensionKind and other settings completion scenarios with 60s timeout. Tests create temp folders and validate completion item insertion/replacement ranges.
- `extensions/configuration-editing/src/test/index.ts` — Test runner configuration supporting Electron, Web (VSCODE_BROWSER env), and Remote (REMOTE_VSCODE env) environments. Configures Mocha with optional JUnit reporter for CI (BUILD_ARTIFACTSTAGINGDIRECTORY, GITHUB_WORKSPACE).

## Types / Interfaces

### Type Definitions
- `extensions/configuration-editing/src/typings/ref.d.ts` — Module declaration for 'tunnel' library (empty ambient declaration).

### Configuration TypeScript
- `extensions/configuration-editing/tsconfig.json` — Extends ../tsconfig.base.json, targets ./src with output to ./out, includes vscode.d.ts and vscode.proposed.profileContentHandlers.d.ts.
- `extensions/configuration-editing/tsconfig.browser.json` — Browser-specific TypeScript configuration (referenced by esbuild.browser.mts).

## Configuration

### Build Configuration
- `extensions/configuration-editing/esbuild.mts` — Node platform bundler entry point targeting configurationEditingMain.ts, outputs to dist/ directory.
- `extensions/configuration-editing/esbuild.browser.mts` — Browser platform bundler with custom esbuild plugin (browserNetPlugin) to redirect ./node/net imports to ./browser/net for browser-safe build output to dist/browser/.

### Extension Manifest
- `extensions/configuration-editing/package.json` — Version 10.0.0, activation on onProfile/onLanguage:json/jsonc, exports main entry ./out/configurationEditingMain and browser entry ./dist/browser/configurationEditingMain. Contributes language definitions for jsonc and json, defines jsonValidation entries for 19 file patterns (settings.json, launch.json, tasks.json, keybindings.json, extensions.json, profiles.json, devcontainer.json, etc.) mapping to internal vscode:// schemas. Enables profileContentHandlers proposal API. Dependencies: @octokit/rest, jsonc-parser, tunnel.
- `extensions/configuration-editing/package-lock.json` — Dependency lock file.
- `extensions/configuration-editing/package.nls.json` — Localization strings for displayName and description.
- `extensions/configuration-editing/.npmrc` — npm configuration.

### Packaging
- `extensions/configuration-editing/.vscodeignore` — Excludes test/, src/, tsconfig files, esbuild configs, devContainer schemas from packaged extension.

## Schemas

### JSON Schemas
- `extensions/configuration-editing/schemas/attachContainer.schema.json` — JSON schema for dev container attachment configuration.
- `extensions/configuration-editing/schemas/devContainer.vscode.schema.json` — VS Code dev container schema.
- `extensions/configuration-editing/schemas/devContainer.codespaces.schema.json` — GitHub Codespaces dev container schema.

## Documentation

### Assets
- `extensions/configuration-editing/images/icon.png` — Extension icon.

---

## Notable Clusters

**Language Service Providers**: The extension registers five language completion providers covering JSON/JSONC-based configuration files (settings.json, extensions.json, launch.json/tasks.json variables, keybindings.json/package.json context keys) plus one document symbol provider for launch.json. All providers target specific file patterns and leverage jsonc-parser's location and visitor APIs for context-aware suggestions.

**Dual Platform Support**: Implementation uses conditional module resolution—esbuild.browser.mts plugin redirects node/net to browser/net at build time, enabling the same TypeScript source to work in both Electron/Node and Web environments with appropriate HTTP agent implementations (proxy-aware in Node, undefined in Browser).

**Profile Sync via GitHub**: importExportProfiles.ts implements the vscode.ProfileContentHandler interface for GitHub Gist-based profile persistence, requiring authentication and integrating Octokit REST API—represents a complex VS Code extension feature requiring external service integration.

**Settings Metadata Integration**: SettingsDocument class consumes installed extension list via vscode.extensions API and dynamically generates proposals for extension-list settings (settingsSync.ignoredExtensions, remote.extensionKind recommendations), bridging core settings schema with runtime extension discovery.

---

## Porting Implications for Tauri/Rust

This extension demonstrates three key patterns relevant to a Tauri port:

1. **Language Service Infrastructure**: VS Code's language completion architecture (registerCompletionItemProvider with DocumentFilter patterns) requires a comparable Rust/Tauri equivalent supporting document context, position-aware parsing, and rich completion metadata. The jsonc-parser integration shows the need for efficient incremental JSON/JSONC parsing without full AST reconstruction.

2. **Platform Abstraction**: The dual-build strategy (Node vs Browser) via esbuild plugins reveals VS Code's layered approach to cross-platform functionality. A Tauri equivalent would likely use feature flags or conditional compilation in Rust to handle desktop-native vs web-runtime differences (e.g., HTTP proxies, file I/O permissions).

3. **External Service Integration**: The GitHub Gist profile handler demonstrates that modern VS Code features depend on OAuth flows and external REST APIs. A Tauri rewrite would require maintaining equivalent authentication patterns (vscode.authentication API equivalent) and HTTP client capabilities, likely via Tauri's http plugin or similar bridging.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: VS Code Configuration-Editing Extension
## Partition 33/79: Schema/IntelliSense for `settings.json`

### Research Context
This partition examines the `extensions/configuration-editing/` scope to identify core IDE patterns that would need to be ported from TypeScript/Electron to Tauri/Rust. The extension provides intelligent editing support for VS Code configuration files through a completion provider architecture and schema-based document analysis.

---

## Pattern 1: Language Service Registration with DocumentFilter

**Found in**: `extensions/configuration-editing/src/configurationEditingMain.ts:32-38`
**Used for**: Registering language service providers for specific file patterns

```typescript
function registerSettingsCompletions(): vscode.Disposable {
	return vscode.languages.registerCompletionItemProvider({ language: 'jsonc', pattern: '**/settings.json' }, {
		provideCompletionItems(document, position, token) {
			return new SettingsDocument(document).provideCompletionItems(position, token);
		}
	});
}
```

**Key aspects**:
- Pattern-based provider registration (language + file glob)
- Delegates completion logic to a document-specific class
- Returns disposable for lifecycle management
- Uses `vscode.languages` API (core IDE extension point)

**Variations in codebase**:
- **Variable completion** (lines 40-83): Pattern-matching for multiple file types (`launch.json`, `tasks.json`, workspace files) with conditional logic based on AST location
- **Extension recommendations** (lines 109-135): Separate registrations for `extensions.json` and workspace configuration documents
- **Context key completion** (lines 183-244): Multi-document registration with Map-based path configuration

---

## Pattern 2: JSON/JSONC Document Analysis with AST Traversal

**Found in**: `extensions/configuration-editing/src/settingsDocumentHelper.ts:16-70`
**Used for**: Context-aware completion routing based on document location

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
		// ... more paths
		return this.provideLanguageOverridesCompletionItems(location, position);
	}
}
```

**Key aspects**:
- Uses `jsonc-parser` library (dependency in `package.json`) for AST location
- Path-based routing: `location.path` provides breadcrumb to current position in JSON tree
- Async method signature supports cancellation tokens
- Document text accessed once, offset calculated for position lookup

**Critical dependencies**:
- `jsonc-parser` (line 7): Provides `getLocation` function that maps offset to JSON path
- Returns either array or CompletionList (TypeScript union type)

---

## Pattern 3: Snippet Completion Items with Variable Expansion

**Found in**: `extensions/configuration-editing/src/settingsDocumentHelper.ts:95-132`
**Used for**: Variable substitution suggestions in string values

```typescript
private async provideWindowTitleCompletionItems(location: Location, pos: vscode.Position): Promise<vscode.CompletionItem[]> {
	const completions: vscode.CompletionItem[] = [];

	if (!this.isCompletingPropertyValue(location, pos)) {
		return completions;
	}

	let range = this.document.getWordRangeAtPosition(pos, /\$\{[^"\}]*\}?/);
	if (!range || range.start.isEqual(pos) || range.end.isEqual(pos) && this.document.getText(range).endsWith('}')) {
		range = new vscode.Range(pos, pos);
	}

	const getText = (variable: string) => {
		const text = '${' + variable + '}';
		return location.previousNode ? text : JSON.stringify(text);
	};

	completions.push(this.newSimpleCompletionItem(getText('activeEditorShort'), range, vscode.l10n.t("the file name (e.g. myFile.txt)")));
	completions.push(this.newSimpleCompletionItem(getText('activeEditorMedium'), range, vscode.l10n.t("the path of the file relative to the workspace folder...")));
	// ... 15+ more variables
	return completions;
}
```

**Key aspects**:
- Smart range detection: matches existing `${...}` syntax or creates empty range
- Conditional text format: wraps in JSON quotes if not already in string context
- Localization via `vscode.l10n.t()` for all user-facing strings
- Regex pattern for detecting partial variables: `/\$\{[^"\}]*\}?/`

**Variations**:
- **Simple completion**: Line 346-353 - Basic item factory
- **Snippet completion**: Line 355-362 - For multi-line insertions with placeholders
- **Editor label patterns**: Line 134-157 - Similar structure but different variable set

---

## Pattern 4: Language-Aware Completion with Dynamic Registry Access

**Found in**: `extensions/configuration-editing/src/settingsDocumentHelper.ts:253-275`
**Used for**: Language identifier suggestions from IDE registry

```typescript
private async provideLanguageCompletionItems(location: Location, position: vscode.Position): Promise<vscode.CompletionItem[]> {
	if (location.path.length === 1 && this.isCompletingPropertyValue(location, position)) {
		const range = this.getReplaceRange(location, position);
		const languages = await vscode.languages.getLanguages();
		return [
			this.newSimpleCompletionItem(JSON.stringify('${activeEditorLanguage}'), range, vscode.l10n.t("Use the language of the currently active text editor if any")),
			...languages.map(l => this.newSimpleCompletionItem(JSON.stringify(l), range))
		];
	}
	return [];
}
```

**Key aspects**:
- Queries IDE registry for available languages at completion time
- Special pseudo-variable `${activeEditorLanguage}` always included
- Async call to `vscode.languages.getLanguages()`
- Flat mapping of language IDs to completion items

**Variations**:
- **Language overrides** (lines 277-319): Complex regex-based parsing for `[language]` bracket syntax with multiple override detection

---

## Pattern 5: Dynamic Extension Registry Completion

**Found in**: `extensions/configuration-editing/src/extensionsProposals.ts:9-32`
**Used for**: Installed extension list suggestions

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

**Key aspects**:
- Filters out already-configured extensions from suggestions
- Built-in extension filtering: excludes `vscode.*` namespace and markdown
- Fallback example suggestion when no new extensions available
- Filter text set to full quoted ID for better search

**Usage locations**:
- `settingsDocumentHelper.ts:51` - `settingsSync.ignoredExtensions`
- `settingsDocumentHelper.ts:61` - `remote.extensionKind` with additional text formatting
- `configurationEditingMain.ts:116` - `extensions.json` recommendations array
- `configurationEditingMain.ts:130` - workspace file extensions

---

## Pattern 6: Document Symbol Provider for Configuration Files

**Found in**: `extensions/configuration-editing/src/configurationEditingMain.ts:148-181`
**Used for**: Outline/breadcrumb navigation in launch.json

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

**Key aspects**:
- Single-pass visitor pattern with state machine (depth tracking)
- Extracts `name` property from launch configurations
- Uses offset-to-position conversion for range creation
- Optional provider label: `'Launch Targets'`

---

## Pattern 7: Context Key Completion with Multi-File Path Mapping

**Found in**: `extensions/configuration-editing/src/configurationEditingMain.ts:183-244`
**Used for**: When-clause completions in keybindings and package.json

```typescript
function registerContextKeyCompletions(): vscode.Disposable {
	type ContextKeyInfo = { key: string; type?: string; description?: string };

	const paths = new Map<vscode.DocumentFilter, JSONPath[]>([
		[{ language: 'jsonc', pattern: '**/keybindings.json' }, [
			['*', 'when']
		]],
		[{ language: 'json', pattern: '**/package.json' }, [
			['contributes', 'menus', '*', 'when'],
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

**Key aspects**:
- Multi-registration: single provider covers multiple document filters with array syntax
- Path matching: Uses wildcard `*` to match any object key at depth
- Command-based data source: `vscode.commands.executeCommand` retrieves context key registry
- Split range support: `replacing` (for overwrite) and `inserting` (for cursor position)
- Cancellation token checked before processing results

---

## Pattern 8: Helper Utilities for Common Operations

**Found in**: `extensions/configuration-editing/src/settingsDocumentHelper.ts:72-93`
**Used for**: Shared range and position logic

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

private isCompletingPropertyValue(location: Location, pos: vscode.Position) {
	if (location.isAtPropertyKey) {
		return false;
	}
	const previousNode = location.previousNode;
	if (previousNode) {
		const offset = this.document.offsetAt(pos);
		return offset >= previousNode.offset && offset <= previousNode.offset + previousNode.length;
	}
	return true;
}
```

**Key aspects**:
- Range computation: Returns existing node range if position is within, else empty range at position
- Defensive position comparison: Checks boundaries before range creation
- Stateful context: Depends on `previousNode` from AST location
- Returns sensible defaults (empty range for new insertions)

---

## Pattern 9: Profile Content Handler Implementation

**Found in**: `extensions/configuration-editing/src/importExportProfiles.ts:11-81`
**Used for**: GitHub gist-based profile synchronization

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
				[name]: { content }
			}
		});
		if (result.data.id && result.data.html_url) {
			const link = vscode.Uri.parse(result.data.html_url);
			return { id: result.data.id, link };
		}
		return null;
	}

	async readProfile(id: string): Promise<string | null>;
	async readProfile(uri: vscode.Uri): Promise<string | null>;
	async readProfile(arg: string | vscode.Uri): Promise<string | null> {
		const gist_id = typeof arg === 'string' ? arg : basename(arg.path);
		const octokit = await this.getPublicOctokit();
		try {
			const gist = await octokit.gists.get({ gist_id });
			if (gist.data.files) {
				return gist.data.files[Object.keys(gist.data.files)[0]]?.content ?? null;
			}
		} catch (error) {
			// ignore
		}
		return null;
	}
}

vscode.window.registerProfileContentHandler('github', new GitHubGistProfileContentHandler());
```

**Key aspects**:
- Interface implementation: Conforms to `vscode.ProfileContentHandler`
- Lazy initialization: Promise-cached Octokit instance
- Dual Octokit instances: authenticated for writing, public for reading
- Overloaded methods: `readProfile` accepts string ID or URI
- Network abstraction: Uses `agent` from node/net.ts for proxy support
- Registration via `vscode.window.registerProfileContentHandler`

---

## Pattern 10: Activation and Subscription Management

**Found in**: `extensions/configuration-editing/src/configurationEditingMain.ts:12-30`
**Used for**: Extension lifecycle and provider registration

```typescript
export function activate(context: vscode.ExtensionContext): void {
	//settings.json suggestions
	context.subscriptions.push(registerSettingsCompletions());

	//extensions suggestions
	context.subscriptions.push(...registerExtensionsCompletions());

	// launch.json variable suggestions
	context.subscriptions.push(registerVariableCompletions('**/launch.json'));

	// task.json variable suggestions
	context.subscriptions.push(registerVariableCompletions('**/tasks.json'));

	// Workspace file launch/tasks variable completions
	context.subscriptions.push(registerVariableCompletions('**/*.code-workspace'));

	// keybindings.json/package.json context key suggestions
	context.subscriptions.push(registerContextKeyCompletions());
}
```

**Key aspects**:
- Standard VS Code activation pattern with `ExtensionContext`
- Disposable collection pattern: `context.subscriptions.push(...)`
- Linear registration order: no dependencies between providers
- Spread operator for array providers: `registerExtensionsCompletions()` returns array
- Comments document purpose of each registration

---

## Testing Patterns

**Found in**: `extensions/configuration-editing/src/test/completion.test.ts`

The test suite demonstrates completion verification patterns:

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
		assert.strictEqual(matches.length, 1, `${expected.label} should only existing once`);

		if (expected.resultText) {
			const match = matches[0];
			if (match.range && match.insertText) {
				const range = match.range instanceof vscode.Range ? match.range : match.range.replacing;
				const text = typeof match.insertText === 'string' ? match.insertText : match.insertText.value;
				await editor.edit(eb => eb.replace(range, text));
				assert.strictEqual(editor.document.getText(), expected.resultText);
			}
		}
	}
}
```

**Test structures**:
- **Marker-based positioning**: `|` character marks cursor position in test content
- **Command-based verification**: Uses `vscode.executeCompletionItemProvider` command
- **File I/O**: Creates temporary files with `fs.writeFile`
- **Editor simulation**: Opens documents with language ID override via `setTextDocumentLanguage`
- **Assertion patterns**: Checks presence/absence and verifies insertText result

**Test suites**: 
- `'Completions in settings.json'` (lines 16-294)
- `'Completions in extensions.json'` (lines 296-338)
- `'Completions in launch.json'` (lines 340-426)
- `'Completions in tasks.json'` (lines 428-484)
- `'Completions in keybindings.json'` (lines 486-537)

---

## Porting Considerations for Tauri/Rust

### Core Abstractions to Replicate

1. **Language Service API**: Provider registration pattern with document filters (language + glob pattern matching)
2. **AST Navigation**: JSON path-based location tracking with offset-to-position mapping
3. **Completion Item Construction**: Rich metadata (kind, detail, documentation, range, insertText)
4. **Async Completion**: Token-based cancellation and lazy data fetching
5. **Command Bus**: Two-way communication via `vscode.commands.executeCommand`
6. **Registry Access**: Dynamic queries to IDE registries (languages, extensions, context keys)
7. **Document Event Loop**: Position-based queries requiring real-time document state
8. **Localization**: Framework for translating user-facing strings

### Key Dependencies
- `jsonc-parser` (3.2.0): Stateless JSON/JSONC parsing library with visitor pattern
- `@octokit/rest` (21.1.1): GitHub API client (used in profile sync, not core IDE functionality)
- `tunnel` (0.0.6): Node.js HTTPS proxy support
- `vscode` module: IDE API surface (completion providers, commands, languages, window, authentication)

### File Organization
- Main entry: `configurationEditingMain.ts` - 245 lines of provider registration
- Document logic: `settingsDocumentHelper.ts` - 363 lines of completion implementation
- Utilities: `extensionsProposals.ts` - 32 lines of shared logic
- Platform-specific: `node/net.ts` (Node.js agent), `browser/net.ts` (undefined stub)
- Integration: `importExportProfiles.ts` - 82 lines of GitHub profile handler

### Architecture Pattern
The extension follows a clean separation:
1. **Registry layer** (`configurationEditingMain.ts`): Declares providers and wires activation
2. **Document analysis layer** (`settingsDocumentHelper.ts`): Stateless per-document completion logic
3. **Data source layer**: External APIs (languages, extensions, commands, authentication)

No global state; all analysis is document-scoped. Async operations support cancellation tokens for responsive UI.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
