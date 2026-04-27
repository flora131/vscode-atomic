# Pattern Research: VS Code IDE Functionality Port to Tauri/Rust
## Scope: `extensions/extension-editing/` — Language Provider Registration & Contributions

**Seed Query**: `ast-grep --lang ts -p 'languages.registerHoverProvider($$$)'`

**Focus**: Language service providers, diagnostic collection, file watching, and package.json contribution schema handling within the extension-editing subsystem.

---

## Pattern 1: Language Provider Registration Pattern

**Found in**: `extensions/extension-editing/src/extensionEditingMain.ts:25-31`
**Used for**: Completion item provider registration for package.json files

```typescript
function registerPackageDocumentCompletions(): vscode.Disposable {
	return vscode.languages.registerCompletionItemProvider({ language: 'json', pattern: '**/package.json' }, {
		provideCompletionItems(document, position, token) {
			return new PackageDocument(document).provideCompletionItems(position, token);
		}
	});
}
```

**Key aspects**:
- Registers provider for specific file pattern (`**/package.json`)
- Returns `vscode.Disposable` for cleanup/subscription
- Provider object implements `provideCompletionItems(document, position, token)` method
- Instantiates provider class with document context
- Pattern uses document selector with language ID + glob pattern

**Variations found**: Also used for code actions provider (line 33-39), completion items provider (browser variant at extensionEditingBrowserMain.ts:18-24)

---

## Pattern 2: Code Action Provider with Diagnostic Context

**Found in**: `extensions/extension-editing/src/extensionEditingMain.ts:33-39`
**Used for**: Providing quick-fix code actions for linting diagnostics

```typescript
function registerCodeActionsProvider(): vscode.Disposable {
	return vscode.languages.registerCodeActionsProvider({ language: 'json', pattern: '**/package.json' }, {
		provideCodeActions(document, range, context, token) {
			return new PackageDocument(document).provideCodeActions(range, context, token);
		}
	});
}
```

**Key aspects**:
- Receives `CodeActionContext` containing diagnostics
- Operates on range-based scope within document
- Handles code action generation as `CodeAction[]`
- Disposal pattern for lifecycle management
- Bound to package.json files only

---

## Pattern 3: Multiple Language Provider Registration (Definition & Reference)

**Found in**: `extensions/extension-editing/src/packageDocumentL10nSupport.ts:13-30`
**Used for**: Localization string definition/reference resolution in package.json and package.nls.json

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

**Key aspects**:
- Single class implements multiple provider interfaces (`DefinitionProvider`, `ReferenceProvider`)
- Registers provider for multiple document selectors (two files: package.json, package.nls.json)
- Centralized disposable management array
- Explicit `dispose()` method for cleanup
- Class instance passed as provider (`this`)

---

## Pattern 4: Diagnostic Collection with File System Watcher

**Found in**: `extensions/extension-editing/src/extensionLinter.ts:60-84`
**Used for**: Real-time linting of extension package.json with file watching and document lifecycle management

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

**Key aspects**:
- `languages.createDiagnosticCollection()` for managing diagnostic issues
- `workspace.createFileSystemWatcher()` monitors file changes matching glob pattern
- Hooks into document lifecycle events: open, change, close
- File watcher provides three events: change, create, delete
- Debounced processing with internal queue (`this.queue()`)
- Iterates existing documents on initialization
- All disposables collected in array for cleanup

---

## Pattern 5: Completion Item and Snippet Generation

**Found in**: `extensions/extension-editing/src/packageDocumentHelper.ts:96-112`
**Used for**: Creating context-aware completion items with snippets for package.json contributions

```typescript
private newSimpleCompletionItem(text: string, range: vscode.Range, description?: string, insertText?: string): vscode.CompletionItem {
	const item = new vscode.CompletionItem(text);
	item.kind = vscode.CompletionItemKind.Value;
	item.detail = description;
	item.insertText = insertText ? insertText : text;
	item.range = range;
	return item;
}

private newSnippetCompletionItem(o: { label: string; documentation?: string; snippet: string; range: vscode.Range }): vscode.CompletionItem {
	const item = new vscode.CompletionItem(o.label);
	item.kind = vscode.CompletionItemKind.Value;
	item.documentation = o.documentation;
	item.insertText = new vscode.SnippetString(o.snippet);
	item.range = o.range;
	return item;
}
```

**Key aspects**:
- Two variants: simple text completion and snippet completion
- `CompletionItemKind.Value` for JSON value context
- Replace range specified per item
- Snippet strings use `vscode.SnippetString` with placeholder syntax (`$1`, `$0`)
- Optional documentation for extended help

---

## Pattern 6: Version Parsing and Normalization with Regex

**Found in**: `extensions/extension-editing/src/extensionEngineValidation.ts:31-75`
**Used for**: Parsing semantic version strings in engine requirement specifications

```typescript
const VERSION_REGEXP = /^(\^|>=)?((\d+)|x)\.((\d+)|x)\.((\d+)|x)(\-.*)?$/;
const NOT_BEFORE_REGEXP = /^-(\d{4})(\d{2})(\d{2})$/;

export function parseVersion(version: string): IParsedVersion | null {
	if (!isValidVersionStr(version)) {
		return null;
	}

	version = version.trim();

	if (version === '*') {
		return {
			hasCaret: false,
			hasGreaterEquals: false,
			majorBase: 0,
			majorMustEqual: false,
			minorBase: 0,
			minorMustEqual: false,
			patchBase: 0,
			patchMustEqual: false,
			preRelease: null
		};
	}

	const m = version.match(VERSION_REGEXP);
	if (!m) {
		return null;
	}
	return {
		hasCaret: m[1] === '^',
		hasGreaterEquals: m[1] === '>=',
		majorBase: m[2] === 'x' ? 0 : parseInt(m[2], 10),
		majorMustEqual: (m[2] === 'x' ? false : true),
		minorBase: m[4] === 'x' ? 0 : parseInt(m[4], 10),
		minorMustEqual: (m[4] === 'x' ? false : true),
		patchBase: m[6] === 'x' ? 0 : parseInt(m[6], 10),
		patchMustEqual: (m[6] === 'x' ? false : true),
		preRelease: m[8] || null
	};
}
```

**Key aspects**:
- Regex captures version components with semantic interpretation
- Supports caret (`^`) and greater-equals (`>=`) operators
- Handles `x` wildcard for version components
- Date-based pre-release version parsing with `NOT_BEFORE_REGEXP`
- Structured output with must-equal flags for range validation
- Special case for `*` (any version)

---

## Pattern 7: Implicit Activation Event Detection and Linting

**Found in**: `extensions/extension-editing/src/extensionLinter.ts:500-598`
**Used for**: Parsing extension contributions to generate implicit activation events

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

	// authenticationProviders
	const authenticationProviders = findNodeAtLocation(tree, ['contributes', 'authentication']);
	authenticationProviders?.children?.forEach(child => {
		const id = findNodeAtLocation(child, ['id']);
		if (id && id.type === 'string') {
			activationEvents.add(`onAuthenticationRequest:${id.value}`);
		}
	});

	// languages
	const languageContributions = findNodeAtLocation(tree, ['contributes', 'languages']);
	languageContributions?.children?.forEach(child => {
		const id = findNodeAtLocation(child, ['id']);
		const configuration = findNodeAtLocation(child, ['configuration']);
		if (id && id.type === 'string' && configuration && configuration.type === 'string') {
			activationEvents.add(`onLanguage:${id.value}`);
		}
	});
	
	// [Additional types: customEditors, views, walkthroughs, notebookRenderer, terminalProfiles, terminalQuickFixes, tasks]
	
	return activationEvents;
}
```

**Key aspects**:
- Traverses JSON tree using `findNodeAtLocation()` 
- Multiple contribution types mapped to activation event prefixes
- Set-based deduplication of events
- Conditional checks for node type and required fields
- Pattern: each contribution type has specific id/identifier field
- Returns complete set for validation against explicit declarations

---

## Porting Implications: Key Architectural Patterns

### Language Service Provider Architecture
VS Code uses a **plugin-based language provider system** requiring:
1. Document selector pattern matching (`language` + glob `pattern`)
2. Provider interface implementation (completions, code actions, definitions, references)
3. Async/Promise-based result returns with cancellation tokens
4. Disposable pattern for lifecycle management

### Document & File Watching
The extension-editing subsystem relies on:
1. **Document lifecycle events** (open, change, close) with debounced processing
2. **File system watchers** for non-open file detection
3. **In-memory caching** of diagnostic state per folder
4. **Lazy loading** of markdown and parse5 dependencies

### JSON/Manifest Processing
Core patterns for extension package.json handling:
1. **JSONC parsing** with `jsonc-parser` (preserves comments)
2. **Path-based navigation** in JSON trees (`['contributes', 'commands']`)
3. **Offset-to-position mapping** for precise diagnostic ranges
4. **String encoding reconstruction** for escaped JSON values

### Type System and Validation
Multiple validation layers:
1. **Version string parsing** with regex and semantic flags
2. **Implicit activation event generation** from contribution declarations
3. **Multi-file relationship** management (package.json ↔ package.nls.json)
4. **DiagnosticSeverity levels** (Error, Warning, Information)

---

## Critical File References

- `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/extensionEditingMain.ts` - Entry point and provider registration
- `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/extensionLinter.ts` - Linting engine with diagnostics
- `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/packageDocumentHelper.ts` - Completion and code action providers
- `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/packageDocumentL10nSupport.ts` - Definition/reference providers
- `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/extensionEngineValidation.ts` - Version parsing logic
- `/Users/norinlavaee/vscode-atomic/extensions/extension-editing/src/jsonReconstruct.ts` - JSON string offset handling

---

## Summary

The extension-editing subsystem demonstrates VS Code's **extensible language services architecture** with clear separation of concerns:

1. **Language Providers** register for specific document patterns
2. **Diagnostic Collection** manages all linting issues with real-time updates
3. **File Watching** maintains consistency with filesystem state
4. **JSON Navigation** uses structured AST walking for manifest parsing
5. **Type Validation** ensures version strings and contributions match schema

For a Tauri/Rust port, this would require:
- LSP (Language Server Protocol) implementation for provider equivalents
- Async event handling matching document lifecycle
- Regex-based parsing for version validation
- JSON tree navigation library (serde_json with path resolution)
- Disposable/subscription pattern equivalent in Rust (Arc<Mutex<>> or channel-based)

