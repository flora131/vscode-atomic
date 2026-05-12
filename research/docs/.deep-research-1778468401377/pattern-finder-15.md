# Emmet Extension Patterns: VS Code API Usage for Tauri/Rust Porting

**Partition 15 Analysis:** `extensions/emmet/` (46 files, 8,451 LOC)

Query validated: `vscode.commands.registerCommand()` only. Extension relies exclusively on public command registration API—no internal/private APIs detected.

---

## Pattern 1: Command Registration with Subscription Tracking

**Where:** `extensions/emmet/src/emmetCommon.ts:29-122`

**What:** Core activation pattern that registers all editor commands via `vscode.commands.registerCommand()` and manages them through `context.subscriptions`. This is THE fundamental pattern for extending VS Code.

```typescript
export function activateEmmetExtension(context: vscode.ExtensionContext) {
	context.subscriptions.push(vscode.commands.registerCommand('editor.emmet.action.wrapWithAbbreviation', (args) => {
		wrapWithAbbreviation(args);
	}));

	context.subscriptions.push(vscode.commands.registerCommand('editor.emmet.action.removeTag', () => {
		return removeTag();
	}));

	context.subscriptions.push(vscode.commands.registerCommand('editor.emmet.action.updateTag', (inputTag) => {
		if (inputTag && typeof inputTag === 'string') {
			return updateTag(inputTag);
		}
		return updateTag(undefined);
	}));

	// ... 18 more commands registered similarly
	
	context.subscriptions.push(vscode.workspace.onDidChangeConfiguration((e) => {
		if (e.affectsConfiguration('emmet.includeLanguages') || e.affectsConfiguration('emmet.useInlineCompletions')) {
			refreshCompletionProviders(context);
		}
	}));

	context.subscriptions.push(vscode.workspace.onDidSaveTextDocument((e) => {
		const basefileName: string = getPathBaseName(e.fileName);
		if (basefileName.startsWith('snippets') && basefileName.endsWith('.json')) {
			updateEmmetExtensionsPath(true);
		}
	}));
}
```

**Variations:**
- `extensions/emmet/src/node/emmetNodeMain.ts:13-19` — Node-specific entry point with dynamic import for image size module
- 23 distinct commands registered, each with different argument signatures (some take `args`, some take no args, some take `inputTag` strings)
- Event listener pattern: `vscode.workspace.onDidChangeConfiguration()`, `onDidSaveTextDocument()`, `onDidOpenTextDocument()`, `onDidCloseTextDocument()`

---

## Pattern 2: Text Editor Manipulation via editor.edit()

**Where:** `extensions/emmet/src/removeTag.ts:26-30`

**What:** Bulk edit pattern for multiple text transformations. All mutations go through the `editBuilder` callback within `editor.edit()` transaction. Deletions, replacements, and insertions are batched.

```typescript
export function removeTag() {
	if (!validate(false) || !vscode.window.activeTextEditor) {
		return;
	}
	const editor = vscode.window.activeTextEditor;
	const document = editor.document;
	const rootNode = <HtmlFlatNode>getRootNode(document, true);
	if (!rootNode) {
		return;
	}

	const finalRangesToRemove = Array.from(editor.selections).reverse()
		.reduce<vscode.Range[]>((prev, selection) =>
			prev.concat(getRangesToRemove(editor.document, rootNode, selection)), []);

	return editor.edit(editBuilder => {
		finalRangesToRemove.forEach(range => {
			editBuilder.delete(range);
		});
	});
}
```

**Variations:**
- `extensions/emmet/src/incrementDecrement.ts:22-34` — Iterate over `editor.selections` (multi-cursor support) and apply replacements
- `extensions/emmet/src/abbreviationActions.ts:139-152` — Complex case with insertText and deletion within same transaction
- `extensions/emmet/src/updateTag.ts:49` — Direct `editBuilder` mutations for tag replacement
- Pattern always returns `Thenable<boolean>` (promise-based)

---

## Pattern 3: Active Editor State Access

**Where:** `extensions/emmet/src/util.ts:107-117`

**What:** Validation helper that checks if an active editor exists and validates its language. Used as guard clause throughout actions.

```typescript
export function validate(allowStylesheet: boolean = true): boolean {
	const editor = vscode.window.activeTextEditor;
	if (!editor) {
		vscode.window.showInformationMessage('No editor is active');
		return false;
	}
	if (!allowStylesheet && isStyleSheet(editor.document.languageId)) {
		return false;
	}
	return true;
}
```

**Variations:**
- `extensions/emmet/src/balance.ts:23-26` — Gets active editor and validates before proceeding
- `extensions/emmet/src/incrementDecrement.ts:16-20` — Gets editor, shows message if none active
- All 20+ action functions follow this pattern: `if (!validate(...) || !vscode.window.activeTextEditor) return;`
- Demonstrates: `vscode.window.showInformationMessage()` / `showErrorMessage()` UI patterns

---

## Pattern 4: Configuration Access and Event Listeners

**Where:** `extensions/emmet/src/util.ts:37-81` and `extensions/emmet/src/emmetCommon.ts:124-138`

**What:** Configuration system access via `vscode.workspace.getConfiguration()`. Supports inspection of global/workspace/folder-level settings. Event listening for configuration changes and file saves.

```typescript
export function updateEmmetExtensionsPath(forceRefresh: boolean = false) {
	const helper = getEmmetHelper();
	let extensionsPath = vscode.workspace.getConfiguration('emmet').get<string[]>('extensionsPath');
	if (!extensionsPath) {
		extensionsPath = [];
	}
	if (forceRefresh || _currentExtensionsPath !== extensionsPath) {
		_currentExtensionsPath = extensionsPath;
		const rootPaths = vscode.workspace.workspaceFolders?.length ? vscode.workspace.workspaceFolders.map(f => f.uri) : undefined;
		const fileSystem = vscode.workspace.fs;
		helper.updateExtensionsPath(extensionsPath, fileSystem, rootPaths, _homeDir).catch(err => {
			if (Array.isArray(extensionsPath) && extensionsPath.length) {
				vscode.window.showErrorMessage(err.message);
			}
		});
	}
}

export function migrateEmmetExtensionsPath() {
	const config = vscode.workspace.getConfiguration().inspect('emmet.extensionsPath');

	if (typeof config?.globalValue === 'string') {
		vscode.workspace.getConfiguration().update('emmet.extensionsPath', [config.globalValue], true);
	}
	// ... similar for workspace and folder scopes
}
```

**Variations:**
- Configuration access at different scopes: global (`true`), workspace (`false`), folder (undefined)
- `vscode.workspace.workspaceFolders` — access to workspace folder URIs
- `vscode.workspace.fs` — FileSystem API for reading extension paths
- Event pattern: `vscode.workspace.onDidChangeConfiguration()`, `onDidSaveTextDocument()`, `onDidOpenTextDocument()`, `onDidCloseTextDocument()`

---

## Pattern 5: Completion Provider Registration

**Where:** `extensions/emmet/src/emmetCommon.ts:163-226`

**What:** Dynamic language-specific provider registration. Registers both inline and explicit completion providers per language. Uses disposable cleanup pattern for re-registration on config changes.

```typescript
function refreshCompletionProviders(_: vscode.ExtensionContext) {
	clearCompletionProviderInfo();

	const completionProvider = new DefaultCompletionItemProvider();
	const inlineCompletionProvider: vscode.InlineCompletionItemProvider = {
		async provideInlineCompletionItems(document: vscode.TextDocument, position: vscode.Position, _: vscode.InlineCompletionContext, token: vscode.CancellationToken) {
			const items = await completionProvider.provideCompletionItems(document, position, token, { triggerCharacter: undefined, triggerKind: vscode.CompletionTriggerKind.Invoke });
			if (!items) {
				return undefined;
			}
			const item = items.items[0];
			if (!item || !item.insertText) {
				return undefined;
			}
			const range = item.range as vscode.Range;

			if (document.getText(range) !== item.label) {
				return undefined;
			}

			return [{
				insertText: item.insertText,
				filterText: item.label,
				range
			}];
		}
	};

	const useInlineCompletionProvider = vscode.workspace.getConfiguration('emmet').get<boolean>('useInlineCompletions');
	const includedLanguages = getMappingForIncludedLanguages();
	Object.keys(includedLanguages).forEach(language => {
		if (languageMappingForCompletionProviders.has(language) && languageMappingForCompletionProviders.get(language) === includedLanguages[language]) {
			return;
		}

		if (useInlineCompletionProvider) {
			const inlineCompletionsProvider = vscode.languages.registerInlineCompletionItemProvider({ language, scheme: '*' }, inlineCompletionProvider);
			completionProviderDisposables.push(inlineCompletionsProvider);
		}

		const explicitProvider = vscode.languages.registerCompletionItemProvider({ language, scheme: '*' }, completionProvider, ...LANGUAGE_MODES[includedLanguages[language]]);
		completionProviderDisposables.push(explicitProvider);

		languageMappingForCompletionProviders.set(language, includedLanguages[language]);
	});
}
```

**Variations:**
- `vscode.languages.registerCompletionItemProvider()` — register with trigger characters from `LANGUAGE_MODES`
- `vscode.languages.registerInlineCompletionItemProvider()` — modern inline completions API
- Pattern tracks active providers with Map and disposes old ones on re-registration
- Implements both `CompletionItemProvider` interface and inline provider callback

---

## Pattern 6: Selection and Range Manipulation

**Where:** `extensions/emmet/src/balance.ts:22-56`

**What:** Multi-selection handling with stateful stack for balance in/out operations. Demonstrates selection equality checks and programmatic selection assignment.

```typescript
function balance(out: boolean) {
	if (!validate(false) || !vscode.window.activeTextEditor) {
		return;
	}
	const editor = vscode.window.activeTextEditor;
	const document = editor.document;
	const rootNode = <HtmlFlatNode>getRootNode(document, true);
	if (!rootNode) {
		return;
	}

	const rangeFn = out ? getRangeToBalanceOut : getRangeToBalanceIn;
	let newSelections: readonly vscode.Selection[] = editor.selections.map(selection => {
		return rangeFn(document, rootNode, selection);
	});

	if (areSameSelections(lastBalancedSelections, editor.selections)) {
		if (out) {
			if (!areSameSelections(editor.selections, newSelections)) {
				balanceOutStack.push(editor.selections);
			}
		} else if (balanceOutStack.length) {
			newSelections = balanceOutStack.pop()!;
		}
	} else {
		balanceOutStack = out ? [editor.selections] : [];
	}

	editor.selections = newSelections;
	lastBalancedSelections = editor.selections;
}
```

**Variations:**
- `editor.selections` — read/write array of selections (multi-cursor support)
- `vscode.Selection` creation via `offsetRangeToSelection()` helper
- Uses `selection.isEqual()`, `selection.contains()`, `range.intersection()`, `range.union()`
- Stateful tracking of previous selections for undo-like behavior

---

## Pattern 7: Document and Position APIs

**Where:** `extensions/emmet/src/defaultCompletionProvider.ts:17-68`

**What:** Document manipulation patterns including position validation, text extraction, and language mode detection. Completion item provider interface implementation.

```typescript
public provideCompletionItems(document: vscode.TextDocument, position: vscode.Position, _: vscode.CancellationToken, context: vscode.CompletionContext): Thenable<vscode.CompletionList | undefined> | undefined {
	const completionResult = this.provideCompletionItemsInternal(document, position, context);
	if (!completionResult) {
		this.lastCompletionType = undefined;
		return;
	}

	return completionResult.then(completionList => {
		if (!completionList || !completionList.items.length) {
			this.lastCompletionType = undefined;
			return completionList;
		}
		const item = completionList.items[0];
		const expandedText = item.documentation ? item.documentation.toString() : '';

		if (expandedText.startsWith('<')) {
			this.lastCompletionType = 'html';
		} else if (expandedText.indexOf(':') > 0 && expandedText.endsWith(';')) {
			this.lastCompletionType = 'css';
		}
		return completionList;
	});
}
```

**Variations:**
- `document.getText(range)` — extract text from range
- `document.offsetAt(position)` / `document.positionAt(offset)` — convert between offsets and positions
- `document.validatePosition()` — ensure position is valid
- `document.languageId` — get document language
- `document.lineCount`, `document.lineAt(line)` — line access with `.isEmptyOrWhitespace`, `.firstNonWhitespaceCharacterIndex`

---

## Summary: Core API Patterns for Tauri Port

The emmet extension demonstrates these critical VS Code extension patterns that must be replicated in Tauri/Rust:

1. **Command System**: Declarative command registration via public API with lifecycle subscription tracking
2. **Event System**: Configuration/workspace/document change listeners with proper cleanup
3. **Editor State Access**: Active editor, document, selections, ranges—all immutable value types
4. **Bulk Edit Transactions**: Batched edits via `editor.edit()` callback pattern
5. **Provider Registration**: Language-specific dynamic registration with disposable cleanup
6. **Multi-Cursor Support**: Selection arrays, stateful cursor tracking
7. **Configuration System**: Scope-aware settings with workspace folder awareness
8. **UI Messages**: Error/info dialogs via `window.show*Message()`

**Porting Implications**: All core functionality routes through pure, async-capable APIs. No extension-internal or deprecated APIs used. The extension is self-contained: takes editor context in, manipulates selections/text via transactions, returns promises. This architecture transfers well to Tauri—command handlers map to Tauri commands, async returns map to promise-based Rust futures, event listeners map to Tauri event subscriptions.

**File Scope**: 23 commands registered, ~8,400 LOC analysis covers all core patterns. The extension depends on `@vscode/emmet-helper` library (npm package) for abbreviation expansion—this is an external dependency that would need Rust equivalent or FFI binding.
