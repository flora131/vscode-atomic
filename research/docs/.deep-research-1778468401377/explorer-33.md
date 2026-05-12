# Partition 33 of 80 — Findings

## Scope
`extensions/configuration-editing/` (11 files, 1,450 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Configuration Editing Extension - File Locator

## Scope
`extensions/configuration-editing/` (11 files, ~1,450 LOC)

---

### Implementation
- `extensions/configuration-editing/src/configurationEditingMain.ts` - Main extension entry point
- `extensions/configuration-editing/src/settingsDocumentHelper.ts` - Settings document utilities
- `extensions/configuration-editing/src/extensionsProposals.ts` - Extension proposals handling
- `extensions/configuration-editing/src/importExportProfiles.ts` - Profile import/export functionality
- `extensions/configuration-editing/src/browser/net.ts` - Browser-side network utilities
- `extensions/configuration-editing/src/node/net.ts` - Node-side network utilities

### Tests
- `extensions/configuration-editing/src/test/index.ts` - Test suite entry point
- `extensions/configuration-editing/src/test/completion.test.ts` - Completion tests

### Types / Interfaces
- `extensions/configuration-editing/src/typings/ref.d.ts` - TypeScript type definitions

### Configuration
- `extensions/configuration-editing/package.json` - Extension manifest and dependencies
- `extensions/configuration-editing/package-lock.json` - Locked dependency versions
- `extensions/configuration-editing/package.nls.json` - Localization/translation strings
- `extensions/configuration-editing/tsconfig.json` - TypeScript configuration
- `extensions/configuration-editing/tsconfig.browser.json` - Browser-specific TypeScript config
- `extensions/configuration-editing/.npmrc` - NPM configuration
- `extensions/configuration-editing/.vscodeignore` - Packaging ignore rules
- `extensions/configuration-editing/esbuild.mts` - Node build configuration
- `extensions/configuration-editing/esbuild.browser.mts` - Browser build configuration

### Examples / Fixtures
- `extensions/configuration-editing/schemas/devContainer.vscode.schema.json` - Dev container schema
- `extensions/configuration-editing/schemas/devContainer.codespaces.schema.json` - Codespaces container schema
- `extensions/configuration-editing/schemas/attachContainer.schema.json` - Attach container schema

### Notable Clusters
- **Source Code**: `extensions/configuration-editing/src/` contains 9 TypeScript files with platform-specific code paths (browser/, node/) and test suite
- **Build Configuration**: Dual esbuild configurations support both Node and browser execution paths
- **Container Schemas**: Three JSON schema files in `extensions/configuration-editing/schemas/` for dev container specifications
- **Assets**: `extensions/configuration-editing/images/icon.png` - Extension icon

---

## Summary

The configuration-editing extension provides TypeScript implementation split across a main module and utility modules for settings, profiles, and proposals. It features dual-platform support (browser/node) with separate build configurations, comprehensive TypeScript configurations, and schema definitions for container specifications. The test structure includes a dedicated test suite with completion tests.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
