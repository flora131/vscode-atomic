# Pattern Finder Report: Extension Editing Language Services

**Scope:** `extensions/extension-editing/` (8 TypeScript files)
**Seed Query:** Completion providers for language services
**Analysis Date:** May 11, 2026

## Pattern 1: Completion Item Provider Registration (Inline Implementation)

**Where:** `extensions/extension-editing/src/extensionEditingMain.ts:25-31`
**What:** Registers a completion provider with document selector using inline anonymous object implementing the CompletionItemProvider interface.

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
- `extensions/extension-editing/src/extensionEditingBrowserMain.ts:18-24` — identical pattern for browser variant
- Called via `context.subscriptions.push()` in activate function for subscription management

---

## Pattern 2: Code Actions Provider Registration

**Where:** `extensions/extension-editing/src/extensionEditingMain.ts:33-39`
**What:** Registers a code actions provider for the same language/pattern selector using inline anonymous object.

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
- Only in `extensionEditingMain.ts:33` (desktop-only feature, not in browser variant)
- Reuses same document selector as completion provider

---

## Pattern 3: Delegated Provider Implementation (Class-based)

**Where:** `extensions/extension-editing/src/packageDocumentHelper.ts:11-42`
**What:** Dedicated class providing implementation of multiple methods matching vscode provider interfaces, delegating from registration to business logic.

```typescript
export class PackageDocument {

	constructor(private document: vscode.TextDocument) { }

	public provideCompletionItems(position: vscode.Position, _token: vscode.CancellationToken): vscode.ProviderResult<vscode.CompletionItem[]> {
		const location = getLocation(this.document.getText(), this.document.offsetAt(position));

		if (location.path.length >= 2 && location.path[1] === 'configurationDefaults') {
			return this.provideLanguageOverridesCompletionItems(location, position);
		}

		return undefined;
	}

	public provideCodeActions(_range: vscode.Range, context: vscode.CodeActionContext, _token: vscode.CancellationToken): vscode.ProviderResult<vscode.CodeAction[]> {
		const codeActions: vscode.CodeAction[] = [];
		for (const diagnostic of context.diagnostics) {
			if (diagnostic.message === implicitActivationEvent || diagnostic.message === redundantImplicitActivationEvent) {
				const codeAction = new vscode.CodeAction(vscode.l10n.t("Remove activation event"), vscode.CodeActionKind.QuickFix);
				codeAction.edit = new vscode.WorkspaceEdit();
				codeActions.push(codeAction);
			}
		}
		return codeActions;
	}
}
```

**Variations / call-sites:**
- Used by both desktop (`extensionEditingMain.ts:28`) and browser (`extensionEditingBrowserMain.ts:21`) variants
- Contains private helper methods for position-based logic and item construction

---

## Pattern 4: Multi-Provider Class with Constructor Registration

**Where:** `extensions/extension-editing/src/packageDocumentL10nSupport.ts:13-29`
**What:** Class implementing both DefinitionProvider and ReferenceProvider interfaces, with automatic registration in constructor and disposal pattern.

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
```

**Variations / call-sites:**
- `extensions/extension-editing/src/extensionLinter.ts:62-64` — similar pattern with diagnosticsCollection and fileWatcher tracked in disposables
- Implements `vscode.Disposable` interface directly
- Registrations use pre-defined selector constants (lines 10-11)

---

## Pattern 5: Document Selector with Language and Pattern

**Where:** `extensions/extension-editing/src/packageDocumentL10nSupport.ts:10-11`
**What:** DocumentSelector constants combining language filter with glob pattern for targeted provider activation.

```typescript
const packageJsonSelector: vscode.DocumentSelector = { language: 'json', pattern: '**/package.json' };
const packageNlsJsonSelector: vscode.DocumentSelector = { language: 'json', pattern: '**/package.nls.json' };
```

**Variations / call-sites:**
- Inline selectors: `extensionEditingMain.ts:26, 34` and `extensionEditingBrowserMain.ts:19` use `{ language: 'json', pattern: '**/package.json' }` directly
- Reusable selector approach: `packageDocumentL10nSupport.ts:18-22` uses pre-defined constants

---

## Pattern 6: Snippet Completion Items with Document Replacement Ranges

**Where:** `extensions/extension-editing/src/packageDocumentHelper.ts:105-112`
**What:** Constructs completion items with snippet strings and custom replacement ranges to handle quote context and bracket matching.

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

**Variations / call-sites:**
- Called from `provideLanguageOverridesCompletionItems:59` with label, documentation, snippet, and range
- Paired with `newSimpleCompletionItem` (lines 96-103) for non-snippet completions

---

## Pattern 7: Multi-File Handler with File Watcher and Queueing

**Where:** `extensions/extension-editing/src/extensionLinter.ts:60-83`
**What:** Diagnostic linter using file system watcher to queue document changes and process asynchronously with debouncing.

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
		workspace.textDocuments.forEach(document => this.queue(document));
	}
```

**Variations / call-sites:**
- Similar pattern appears with separate handlers for package.json and readme document queues
- Debouncing with timer (lines 102-109)
- Disposables tracked in array for cleanup

---

## Summary

The extension-editing scope demonstrates five core language service patterns for VS Code porting:

1. **Provider Registration**: Inline anonymous objects implementing provider interfaces, registered via `vscode.languages.register*Provider()`
2. **Selector Patterns**: Document selectors using both language filters and glob patterns (e.g., `{ language: 'json', pattern: '**/package.json' }`)
3. **Delegated Architecture**: Business logic classes (PackageDocument, PackageDocumentL10nSupport) separate from registration layer
4. **Multi-Provider Classes**: Single class implementing multiple provider interfaces with constructor-based registration
5. **Completion Context**: Snippet-based completions with smart range handling for quote/bracket context
6. **Reference/Definition Tracking**: Cross-file symbol navigation (NLS key references between package.json and package.nls.json)
7. **Diagnostic Management**: File watcher integration with document change tracking for real-time validation

These patterns would require Tauri/Rust equivalents for:
- Language service registration API (provider registry)
- Document selector filtering and matching
- Completion item generation with snippet support
- Diagnostic collection and management
- File system watching for reactive updates

Key complexity: Multi-file coordination (package.json ↔ package.nls.json) and context-aware completion range calculation based on JSON parsing.
