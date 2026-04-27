# VS Code IDE Functionality Architecture: Emmet Extension Case Study

## Research Scope
`extensions/emmet/` — 46 files, 8,451 LOC demonstrating how VS Code core IDE patterns are implemented in a real extension.

---

## Pattern 1: Completion Provider Registration
**Where:** `extensions/emmet/src/emmetCommon.ts:207,220`
**What:** Register language-scoped completion providers with multi-character trigger support via `vscode.languages.registerCompletionItemProvider()`.

```typescript
const explicitProvider = vscode.languages.registerCompletionItemProvider(
	{ language, scheme: '*' },
	completionProvider,
	...LANGUAGE_MODES[includedLanguages[language]]
);
completionProviderDisposables.push(explicitProvider);
```

**Variations / call-sites:**
- `extensions/emmet/src/emmetCommon.ts:203` — Inline completion variant: `registerInlineCompletionItemProvider()`
- `extensions/emmet/src/emmetCommon.ts:216` — Same pattern for unmapped languages

---

## Pattern 2: Completion Item Provider Implementation
**Where:** `extensions/emmet/src/defaultCompletionProvider.ts:13-41`
**What:** Implement `vscode.CompletionItemProvider` interface with async `provideCompletionItems()` returning typed `CompletionList`.

```typescript
export class DefaultCompletionItemProvider implements vscode.CompletionItemProvider {
	private lastCompletionType: string | undefined;

	public provideCompletionItems(
		document: vscode.TextDocument,
		position: vscode.Position,
		_: vscode.CancellationToken,
		context: vscode.CompletionContext
	): Thenable<vscode.CompletionList | undefined> | undefined {
		const completionResult = this.provideCompletionItemsInternal(document, position, context);
		if (!completionResult) {
			this.lastCompletionType = undefined;
			return;
		}
		return completionResult.then(completionList => {
			// Process and return typed CompletionList
			return completionList;
		});
	}
}
```

**Variations / call-sites:**
- `extensions/emmet/src/defaultCompletionProvider.ts:17-211` — Full implementation with validation, syntax detection, and item transformation

---

## Pattern 3: Command Registration with Subscription Management
**Where:** `extensions/emmet/src/emmetCommon.ts:29-122`
**What:** Register commands via `vscode.commands.registerCommand()` and manage lifecycle through `context.subscriptions.push()`.

```typescript
context.subscriptions.push(vscode.commands.registerCommand('editor.emmet.action.wrapWithAbbreviation', (args) => {
	wrapWithAbbreviation(args);
}));

context.subscriptions.push(vscode.commands.registerCommand('emmet.expandAbbreviation', (args) => {
	expandEmmetAbbreviation(args);
}));

context.subscriptions.push(vscode.commands.registerCommand('editor.emmet.action.removeTag', () => {
	return removeTag();
}));
```

**Variations / call-sites:**
- 15+ command registrations across lines 29-122
- Pattern used for: wrapWithAbbreviation, expandAbbreviation, removeTag, updateTag, matchTag, balanceOut, balanceIn, splitJoinTag, mergeLines, toggleComment, nextEditPoint, prevEditPoint, selectNextItem, selectPrevItem, evaluateMathExpression, incrementDecrement variants (6 commands), reflectCSSValue

---

## Pattern 4: Configuration-Driven Feature Activation
**Where:** `extensions/emmet/src/emmetCommon.ts:124-131`
**What:** Monitor configuration changes via `vscode.workspace.onDidChangeConfiguration()` and conditionally reload providers.

```typescript
context.subscriptions.push(vscode.workspace.onDidChangeConfiguration((e) => {
	if (e.affectsConfiguration('emmet.includeLanguages') || e.affectsConfiguration('emmet.useInlineCompletions')) {
		refreshCompletionProviders(context);
	}
	if (e.affectsConfiguration('emmet.extensionsPath')) {
		updateEmmetExtensionsPath();
	}
}));
```

**Variations / call-sites:**
- Configuration watched: `emmet.includeLanguages`, `emmet.useInlineCompletions`, `emmet.extensionsPath`
- Schema defined in `extensions/emmet/package.json:26-264` with 13+ configurable properties

---

## Pattern 5: Document Event Lifecycle Handlers
**Where:** `extensions/emmet/src/emmetCommon.ts:133-154`
**What:** Subscribe to document open/close/save events to maintain internal parse caches.

```typescript
context.subscriptions.push(vscode.workspace.onDidSaveTextDocument((e) => {
	const basefileName: string = getPathBaseName(e.fileName);
	if (basefileName.startsWith('snippets') && basefileName.endsWith('.json')) {
		updateEmmetExtensionsPath(true);
	}
}));

context.subscriptions.push(vscode.workspace.onDidOpenTextDocument((e) => {
	const emmetMode = getEmmetMode(e.languageId, {}, []) ?? '';
	const syntaxes = getSyntaxes();
	if (syntaxes.markup.includes(emmetMode) || syntaxes.stylesheet.includes(emmetMode)) {
		addFileToParseCache(e);
	}
}));

context.subscriptions.push(vscode.workspace.onDidCloseTextDocument((e) => {
	const emmetMode = getEmmetMode(e.languageId, {}, []) ?? '';
	const syntaxes = getSyntaxes();
	if (syntaxes.markup.includes(emmetMode) || syntaxes.stylesheet.includes(emmetMode)) {
		removeFileFromParseCache(e);
	}
}));
```

**Variations / call-sites:**
- Cache initialization: `addFileToParseCache()` at line 144
- Cache cleanup: `removeFileFromParseCache()` at line 152

---

## Pattern 6: Active Editor State Access and Manipulation
**Where:** `extensions/emmet/src/abbreviationActions.ts:37-107`
**What:** Access active editor via `vscode.window.activeTextEditor`, read selections, and modify them atomically.

```typescript
const editor = vscode.window.activeTextEditor!;
const document = editor.document;

const operationRanges = Array.from(editor.selections).sort((a, b) => a.start.compareTo(b.start)).map(selection => {
	let rangeToReplace: vscode.Range = selection;
	// ... process range ...
	return rangeToReplace;
}).reduce((mergedRanges, range) => {
	// Merge overlapping ranges
	if (mergedRanges.length > 0 && range.intersection(mergedRanges[mergedRanges.length - 1])) {
		mergedRanges.push(range.union(mergedRanges.pop()!));
	} else {
		mergedRanges.push(range);
	}
	return mergedRanges;
}, [] as vscode.Range[]);

const oldSelections = editor.selections;
editor.selections = operationRanges.map(range => new vscode.Selection(range.start, range.end));
```

**Variations / call-sites:**
- Selection read/write: `extensions/emmet/src/balance.ts:26,34,54`
- Selection manipulation: `extensions/emmet/src/selectItem.ts:17,25,41-42`
- Direct editor.selections assignment triggers UI update

---

## Pattern 7: Batch Text Editing with Edit Builder
**Where:** `extensions/emmet/src/abbreviationActions.ts:138-145`
**What:** Execute multiple text mutations atomically using `editor.edit()` callback with `TextEditorEdit` builder.

```typescript
function revertPreview(): Thenable<boolean> {
	return editor.edit(builder => {
		for (const rangeToReplace of rangesToReplace) {
			builder.replace(rangeToReplace.previewRange, rangeToReplace.originalContent);
			rangeToReplace.previewRange = rangeToReplace.originalRange;
		}
	}, { undoStopBefore: false, undoStopAfter: false });
}
```

**Variations / call-sites:**
- `extensions/emmet/src/mergeLines.ts:23-26` — Multiple edits in single transaction
- Edit options control undo behavior: `{ undoStopBefore, undoStopAfter }`

---

## Pattern 8: Language Detection and Mode Mapping
**Where:** `extensions/emmet/src/util.ts:86-100,119-138,148-176`
**What:** Define static language-to-trigger-characters map and implement dynamic language mapping via configuration.

```typescript
export const LANGUAGE_MODES: { [id: string]: string[] } = {
	'html': ['!', '.', '}', ':', '*', '$', ']', '/', '>', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
	'jade': ['!', '.', '}', ':', '*', '$', ']', '/', '>', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
	'css': [':', '!', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
	'scss': [':', '!', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
	'javascriptreact': ['!', '.', '}', '*', '$', ']', '/', '>', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
	'typescriptreact': ['!', '.', '}', '*', '$', ']', '/', '>', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
};

export function getMappingForIncludedLanguages(): Record<string, string> {
	const MAPPED_MODES: Record<string, string> = {
		'handlebars': 'html',
		'php': 'html'
	};
	const includeLanguagesConfig = vscode.workspace.getConfiguration('emmet').get<Record<string, string>>('includeLanguages');
	const includeLanguages = Object.assign({}, MAPPED_MODES, includeLanguagesConfig ?? {});
	// Validate and return
	return finalMappedModes;
}
```

**Variations / call-sites:**
- Built-in modes: 12 language types with specific trigger characters
- User config merging: `extensions/emmet/src/util.ts:130-137`
- Mode validation: `extensions/emmet/src/util.ts:132-136`

---

## Pattern 9: Disposable Resource Lifecycle Management
**Where:** `extensions/emmet/src/emmetCommon.ts:157-234`
**What:** Track disposable registrations in arrays and clean up on deactivation.

```typescript
const languageMappingForCompletionProviders: Map<string, string> = new Map<string, string>();
const completionProviderDisposables: vscode.Disposable[] = [];

function refreshCompletionProviders(_: vscode.ExtensionContext) {
	clearCompletionProviderInfo();
	// ... register new providers ...
}

function clearCompletionProviderInfo() {
	languageMappingForCompletionProviders.clear();
	let disposable: vscode.Disposable | undefined;
	while (disposable = completionProviderDisposables.pop()) {
		disposable.dispose();
	}
}

export function deactivate() {
	clearCompletionProviderInfo();
	clearParseCache();
}
```

**Variations / call-sites:**
- Called from: `extensions/emmet/src/emmetCommon.ts:236-239` on extension deactivation
- Pattern ensures no memory leaks from event handlers or provider registrations

---

## Pattern 10: Asynchronous Command Execution with Fallback
**Where:** `extensions/emmet/src/abbreviationActions.ts:264-277`
**What:** Execute async operations with fallback behavior when editor is unavailable.

```typescript
export function expandEmmetAbbreviation(args: any): Thenable<boolean | undefined> {
	if (!validate() || !vscode.window.activeTextEditor) {
		return fallbackTab();
	}

	if (vscode.window.activeTextEditor.selections.length === 1 &&
		vscode.window.activeTextEditor.selection.isEmpty
	) {
		const anchor = vscode.window.activeTextEditor.selection.anchor;
		if (anchor.character === 0) {
			return fallbackTab();
		}
		// ... continue with emmet expansion ...
	}
}
```

**Variations / call-sites:**
- Fallback function: returns `vscode.commands.executeCommand('tab')`
- Pattern prevents command from breaking editor workflow

---

## Pattern 11: Symbol Query Execution via Command API
**Where:** `extensions/emmet/src/defaultCompletionProvider.ts:169-179`
**What:** Query language-specific features (document symbols) via `vscode.commands.executeCommand()` with proper typing.

```typescript
if (!isStyleSheet(syntax) && (document.languageId === 'javascript' || document.languageId === 'javascriptreact' || document.languageId === 'typescript' || document.languageId === 'typescriptreact')) {
	const abbreviation: string = extractAbbreviationResults.abbreviation;
	if (abbreviation.startsWith('this.') || /\[[^\]=]*\]/.test(abbreviation)) {
		isNoisePromise = Promise.resolve(true);
	} else {
		isNoisePromise = vscode.commands.executeCommand<vscode.SymbolInformation[] | undefined>('vscode.executeDocumentSymbolProvider', document.uri).then(symbols => {
			return !!symbols && symbols.some(x => abbreviation === x.name || (abbreviation.startsWith(x.name + '.') && !/>|\*|\+/.test(abbreviation)));
		});
	}
}
```

**Variations / call-sites:**
- Command: `vscode.executeDocumentSymbolProvider` with return type `vscode.SymbolInformation[]`
- Used to prevent false completions in code documents

---

## Pattern 12: Configuration Lookup and Defaults
**Where:** `extensions/emmet/src/defaultCompletionProvider.ts:44-58`
**What:** Query workspace configuration with fallback defaults for multiple settings.

```typescript
const emmetConfig = vscode.workspace.getConfiguration('emmet');
const excludedLanguages = emmetConfig['excludeLanguages'] ? emmetConfig['excludeLanguages'] : [];
if (excludedLanguages.includes(document.languageId)) {
	return;
}

const mappedLanguages = getMappingForIncludedLanguages();
const isSyntaxMapped = mappedLanguages[document.languageId] ? true : false;
const emmetMode = getEmmetMode((isSyntaxMapped ? mappedLanguages[document.languageId] : document.languageId), mappedLanguages, excludedLanguages);

if (!emmetMode
	|| emmetConfig['showExpandedAbbreviation'] === 'never'
	|| ((isSyntaxMapped || emmetMode === 'jsx') && emmetConfig['showExpandedAbbreviation'] !== 'always')) {
	return;
}
```

**Variations / call-sites:**
- `extensions/emmet/src/util.ts:39-51` — Get and validate `emmet.extensionsPath` array
- `extensions/emmet/src/util.ts:61-80` — Inspect and migrate legacy configuration types
- Config properties: 13+ different settings with nested preferences object

---

## Pattern 13: Completion Item Construction and Filtering
**Where:** `extensions/emmet/src/defaultCompletionProvider.ts:188-210`
**What:** Transform provider-specific items to VS Code `CompletionItem` objects with filtering metadata.

```typescript
const newItems: vscode.CompletionItem[] = [];
if (result && result.items) {
	result.items.forEach((item: any) => {
		const newItem = new vscode.CompletionItem(item.label);
		newItem.documentation = item.documentation;
		newItem.detail = item.detail;
		newItem.insertText = new vscode.SnippetString(item.textEdit.newText);
		const oldrange = item.textEdit.range;
		newItem.range = new vscode.Range(oldrange.start.line, oldrange.start.character, oldrange.end.line, oldrange.end.character);
		
		newItem.filterText = item.filterText;
		newItem.sortText = item.sortText;
		
		if (emmetConfig['showSuggestionsAsSnippets'] === true) {
			newItem.kind = vscode.CompletionItemKind.Snippet;
		}
		newItems.push(newItem);
	});
}

return new vscode.CompletionList(newItems, true);
```

**Variations / call-sites:**
- Snippet insertion: `new vscode.SnippetString()`
- Kind filtering: configurable via `emmet.showSuggestionsAsSnippets`
- Returns `CompletionList` with second arg (isIncomplete flag)

---

## Pattern 14: Selection Preservation and Reveal
**Where:** `extensions/emmet/src/selectItem.ts:13-43`
**What:** Batch-update selections across multiple cursors and reveal last selection for UX.

```typescript
export function fetchSelectItem(direction: string): void {
	if (!validate() || !vscode.window.activeTextEditor) {
		return;
	}
	const editor = vscode.window.activeTextEditor;
	const document = editor.document;
	const rootNode = getRootNode(document, true);
	if (!rootNode) {
		return;
	}

	const newSelections: vscode.Selection[] = [];
	editor.selections.forEach(selection => {
		const selectionStart = selection.isReversed ? selection.active : selection.anchor;
		const selectionEnd = selection.isReversed ? selection.anchor : selection.active;

		let updatedSelection;
		if (isStyleSheet(editor.document.languageId)) {
			updatedSelection = direction === 'next' ?
				nextItemStylesheet(document, selectionStart, selectionEnd, <CssNode>rootNode) :
				prevItemStylesheet(document, selectionStart, selectionEnd, <CssNode>rootNode);
		} else {
			updatedSelection = direction === 'next' ?
				nextItemHTML(document, selectionStart, selectionEnd, <HtmlNode>rootNode) :
				prevItemHTML(document, selectionStart, selectionEnd, <HtmlNode>rootNode);
		}
		newSelections.push(updatedSelection ? updatedSelection : selection);
	});
	editor.selections = newSelections;
	editor.revealRange(editor.selections[editor.selections.length - 1]);
}
```

**Variations / call-sites:**
- Selection reversal detection: `selection.isReversed ? selection.active : selection.anchor`
- Reveal operation: `editor.revealRange()` scrolls viewport to keep last selection visible

---

## Pattern 15: Balance Navigation with Stack-Based State
**Where:** `extensions/emmet/src/balance.ts:11-56`
**What:** Maintain stack of selection states to enable reversible navigation actions.

```typescript
let balanceOutStack: Array<readonly vscode.Selection[]> = [];
let lastBalancedSelections: readonly vscode.Selection[] = [];

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

	// check whether we are starting a balance elsewhere
	if (areSameSelections(lastBalancedSelections, editor.selections)) {
		// we are not starting elsewhere, so use the stack as-is
		if (out) {
			// make sure we are able to expand outwards
			if (!areSameSelections(editor.selections, newSelections)) {
				balanceOutStack.push(editor.selections);
			}
		} else if (balanceOutStack.length) {
			newSelections = balanceOutStack.pop()!;
		}
	} else {
		// we are starting elsewhere, so reset the stack
		balanceOutStack = out ? [editor.selections] : [];
	}

	editor.selections = newSelections;
	lastBalancedSelections = editor.selections;
}
```

**Variations / call-sites:**
- Similar pattern used in `extensions/emmet/src/abbreviationActions.ts` for wrap preview

---

## Summary

The Emmet extension demonstrates **15 distinct IDE capability patterns** that would require Tauri/Rust ports:

1. **Language Provider Registration** — Multi-character trigger-based completion registration
2. **Provider Interface Implementation** — Async item generation with cancellation tokens
3. **Command Registration & Lifecycle** — 20+ commands with subscription tracking
4. **Configuration Monitoring** — Real-time config change handlers with feature reload
5. **Document Event Handling** — Open/close/save hooks for cache management
6. **Editor State Access** — Active editor, selections, documents as mutable state
7. **Batch Text Editing** — Transactional multi-range edits with undo control
8. **Language Mapping** — Static + dynamic language mode resolution
9. **Disposable Management** — Resource cleanup on deactivation
10. **Async Fallback Handling** — Graceful degradation when editor unavailable
11. **Command Query API** — Cross-extension symbol/provider queries
12. **Configuration Lookup** — Multi-layer config resolution with validation
13. **Completion Item Generation** — Provider-to-VSCode type adaptation
14. **Selection Batching** — Multi-cursor operations with viewport management
15. **Stack-Based Navigation** — Reversible state for structural navigation

**Key architectural implications for Tauri port:**
- Requires FFI for completion providers (async callbacks)
- Selection/editor state needs bidirectional update binding
- Configuration system must support hierarchical overrides
- Event system for document lifecycle needs weak references to prevent cycles
- Command registry needs dynamic dispatch mechanism
- Disposable pattern suggests resource tracking needed in Rust layer

