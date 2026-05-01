# Pattern Research: VS Code Extension Editing - package.json Validation & Completion

## Research Question
Patterns for implementing language servers and validation systems for extension manifests (`package.json`), as found in the extension-editing extension.

## Scope
`extensions/extension-editing/` — 8 TypeScript files focusing on package.json validation, completion, and linting.

---

## Patterns Found

#### Pattern: Completion Item Provider with DocumentSelector Pattern
**Where:** `extensions/extension-editing/src/extensionEditingMain.ts:25-31`
**What:** Registers a completion provider for package.json files using language and glob pattern matching.
```typescript
function registerPackageDocumentCompletions(): vscode.Disposable {
	return vscode.languages.registerCompletionItemProvider({ language: 'json', pattern: '**/package.json' }, {
		provideCompletionItems(document, position, token) {
			return new PackageDocument(document).provideCompletionItems(position, token);
		}
	});
}
```
**Variations / call-sites:**
- `extensions/extension-editing/src/extensionEditingBrowserMain.ts:18-24` — identical browser variant
- `extensions/extension-editing/src/packageDocumentL10nSupport.ts:10` — DocumentSelector pattern reused for definition/reference providers

---

#### Pattern: Code Actions Provider for Diagnostic Fixes
**Where:** `extensions/extension-editing/src/extensionEditingMain.ts:33-39`
**What:** Registers a code actions provider targeting the same package.json document selector to fix linting diagnostics.
```typescript
function registerCodeActionsProvider(): vscode.Disposable {
	return vscode.languages.registerCodeActionsProvider({ language: 'json', pattern: '**/package.json' }, {
		provideCodeActions(document, range, context, token) {
			return new PackageDocument(document).provideCodeActions(range, context, token);
		}
	});
}
```
**Variations / call-sites:**
- Used in tandem with completion provider in main activation path
- Only appears in `extensionEditingMain.ts:33` (not in browser variant)

---

#### Pattern: Diagnostic Collection with File System Watcher
**Where:** `extensions/extension-editing/src/extensionLinter.ts:60-82`
**What:** Creates a diagnostic collection and watches for package.json changes to trigger async linting.
```typescript
export class ExtensionLinter {
	private diagnosticsCollection = languages.createDiagnosticCollection('extension-editing');
	private fileWatcher = workspace.createFileSystemWatcher('**/package.json');
	private disposables: Disposable[] = [this.diagnosticsCollection, this.fileWatcher];

	constructor() {
		this.disposables.push(
			workspace.onDidOpenTextDocument(document => this.queue(document)),
			workspace.onDidChangeTextDocument(event => this.queue(event.document)),
			workspace.onDidCloseTextDocument(document => this.clear(document)),
			this.fileWatcher.onDidChange(uri => this.packageJsonChanged(this.getUriFolder(uri))),
			this.fileWatcher.onDidCreate(uri => this.packageJsonChanged(this.getUriFolder(uri))),
			this.fileWatcher.onDidDelete(uri => this.packageJsonChanged(this.getUriFolder(uri))),
		);
	}
}
```
**Variations / call-sites:**
- File watching used for both package.json and README/CHANGELOG files
- Debouncing implemented via `startTimer()` at line 102-110

---

#### Pattern: JSON Tree Parsing and Validation
**Where:** `extensions/extension-editing/src/extensionLinter.ts:119-201`
**What:** Parses JSON using jsonc-parser and validates specific nodes (icon, badges, activationEvents, apiProposals).
```typescript
private async lintPackageJson() {
	for (const document of Array.from(this.packageJsonQ)) {
		const diagnostics: Diagnostic[] = [];
		const tree = parseTree(document.getText());
		const info = this.readPackageJsonInfo(this.getUriFolder(document.uri), tree);
		if (tree && info.isExtension) {
			const icon = findNodeAtLocation(tree, ['icon']);
			if (icon && icon.type === 'string') {
				this.addDiagnostics(diagnostics, document, icon.offset + 1, icon.offset + icon.length - 1, icon.value, Context.ICON, info);
			}
			// Similar validation for badges, apiProposals, activationEvents
		}
		this.diagnosticsCollection.set(document.uri, diagnostics);
	}
}
```
**Variations / call-sites:**
- Different validation contexts: ICON, BADGE, MARKDOWN (enum at line 40-44)
- Reused in `lintReadme()` for markdown image validation (lines 272-374)

---

#### Pattern: Implicit Activation Event Detection
**Where:** `extensions/extension-editing/src/extensionLinter.ts:500-598`
**What:** Parses contribution declarations and automatically generates expected activation events for validation.
```typescript
function parseImplicitActivationEvents(tree: JsonNode): Set<string> {
	const activationEvents = new Set<string>();
	
	// commands
	const commands = findNodeAtLocation(tree, ['contributes', 'commands']);
	commands?.children?.forEach(child => {
		const command = findNodeAtLocation(child, ['command']);
		if (command && command.type === 'string') {
			activationEvents.add(`onCommand:${command.value}`);
		}
	});
	
	// Similar patterns for: authenticationProviders, languages, customEditors, views, walkthroughs, notebookRenderers, terminalProfiles, terminalQuickFixes, tasks
	return activationEvents;
}
```
**Variations / call-sites:**
- Referenced at line 399 in `readPackageJsonInfo()`
- Used to detect redundant explicit activation events at lines 167-171

---

#### Pattern: Definition/Reference Provider Interface Implementation
**Where:** `extensions/extension-editing/src/packageDocumentL10nSupport.ts:13-29`
**What:** Implements multiple provider interfaces (DefinitionProvider, ReferenceProvider) with dual DocumentSelector registration for package.json and package.nls.json.
```typescript
export class PackageDocumentL10nSupport implements vscode.DefinitionProvider, vscode.ReferenceProvider, vscode.Disposable {
	private readonly _disposables: vscode.Disposable[] = [];

	constructor() {
		this._disposables.push(vscode.languages.registerDefinitionProvider(packageJsonSelector, this));
		this._disposables.push(vscode.languages.registerDefinitionProvider(packageNlsJsonSelector, this));

		this._disposables.push(vscode.languages.registerReferenceProvider(packageNlsJsonSelector, this));
		this._disposables.push(vscode.languages.registerReferenceProvider(packageJsonSelector, this));
	}

	dispose(): void {
		for (const d of this._disposables) {
			d.dispose();
		}
	}
}
```
**Variations / call-sites:**
- Both providers handle two document types with shared implementation logic
- Used in both main (line 20) and browser (line 15) entry points

---

#### Pattern: When-Clause Validation with External Command Dispatch
**Where:** `extensions/extension-editing/src/extensionLinter.ts:203-270`
**What:** Validates when-clauses by dispatching to VS Code's internal `_validateWhenClauses` command and maps errors back to source positions.
```typescript
private async lintWhenClauses(contributesNode: JsonNode | undefined, document: TextDocument): Promise<Diagnostic[]> {
	// ... recursively find when-clause strings in menus, views, keybindings
	const parseResults = await commands.executeCommand<{ errorMessage: string; offset: number; length: number }[][]>(
		'_validateWhenClauses', 
		whenClauses.map(w => w.value as string)
	);

	const diagnostics: Diagnostic[] = [];
	for (let i = 0; i < parseResults.length; ++i) {
		const whenClauseJSONNode = whenClauses[i];
		const jsonStringScanner = new JsonStringScanner(document.getText(), whenClauseJSONNode.offset + 1);
		for (const error of parseResults[i]) {
			const realOffset = jsonStringScanner.getOffsetInEncoded(error.offset);
			// ... create diagnostic
		}
	}
	return diagnostics;
}
```
**Variations / call-sites:**
- Used during package.json linting at line 196
- Requires JsonStringScanner helper (line 15) to handle escaped strings

---

## Integration Summary

The extension-editing extension demonstrates a layered architecture for manifest validation:

1. **Registration Layer**: Multiple provider types (completion, code actions, definition, reference) registered via DocumentSelector pattern targeting `**/package.json`
2. **Linting Layer**: ExtensionLinter class monitors file changes and performs async validation against contribution declarations
3. **Parsing Layer**: jsonc-parser used for JSON AST traversal; external command dispatch for complex validation (when-clauses)
4. **Localization Layer**: Separate L10n support for mapping NLS references between package.json and package.nls.json
5. **Activation**: Main and browser variants follow identical registration patterns

### Key Implementation Details

- **Document Selector Pattern**: Consistent use of `{ language: 'json', pattern: '**/package.json' }` across all providers
- **Async Debouncing**: 300ms timer debounces rapid document changes before linting (line 106)
- **Disposable Management**: Providers and watchers tracked in subscriptions for proper cleanup
- **Error Context**: Diagnostics include URI targets for external documentation links (e.g., activation-events, when-clause-contexts)
- **Implicit Event Detection**: Automatically generates expected activation events from contribution declarations to detect redundant explicit entries

This pattern hierarchy could be adapted for a Tauri/Rust implementation by:
- Replacing `vscode.languages` registrations with custom language server protocol handlers
- Converting file watchers to Rust file system events
- Porting jsonc-parser or using serde_json with custom validation logic
- Implementing the implicit event detection as a separate validation pass
