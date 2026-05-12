# PHP Language Features Extension: Core IDE Patterns

This analysis identifies concrete code patterns from VS Code's PHP language features extension (`extensions/php-language-features/`, 11 files, ~7,255 LOC). These patterns demonstrate how language intelligence features interface with the VS Code API and architecture.

---

## Pattern 1: Language Provider Registration & Activation

**Where:** `extensions/php-language-features/src/phpMain.ts:13-22`

**What:** Extension activation hook registers language-specific providers via vscode.languages API with trigger characters.

```typescript
export function activate(context: vscode.ExtensionContext): any {

	const validator = new PHPValidationProvider();
	validator.activate(context.subscriptions);

	// add providers
	context.subscriptions.push(vscode.languages.registerCompletionItemProvider('php', new PHPCompletionItemProvider(), '>', '$'));
	context.subscriptions.push(vscode.languages.registerHoverProvider('php', new PHPHoverProvider()));
	context.subscriptions.push(vscode.languages.registerSignatureHelpProvider('php', new PHPSignatureHelpProvider(), '(', ','));
}
```

**Variations / call-sites:**
- `validationProvider.ts:106` – Creates diagnostic collection via `vscode.languages.createDiagnosticCollection()`
- Pattern: Each provider type wraps a concrete implementation class and registers with language ID and optional trigger characters

---

## Pattern 2: Interface-Based Provider Implementation

**Where:** `extensions/php-language-features/src/features/completionItemProvider.ts:10-12`

**What:** Providers implement VS Code interfaces (CompletionItemProvider, HoverProvider, SignatureHelpProvider) with async method signatures returning Promise<T>.

```typescript
export default class PHPCompletionItemProvider implements CompletionItemProvider {

	public provideCompletionItems(document: TextDocument, position: Position, _token: CancellationToken, context: CompletionContext): Promise<CompletionItem[]> {
		const result: CompletionItem[] = [];

		const shouldProvideCompletionItems = workspace.getConfiguration('php').get<boolean>('suggest.basic', true);
		if (!shouldProvideCompletionItems) {
			return Promise.resolve(result);
		}
```

**Variations / call-sites:**
- `hoverProvider.ts:11-13` – HoverProvider returns `Hover | undefined` synchronously
- `signatureHelpProvider.ts:69-71` – SignatureHelpProvider returns `Promise<SignatureHelp> | null`
- Pattern: Providers check configuration, validate position, return typed results or early-exit with null/empty

---

## Pattern 3: Configuration-Driven Feature Toggling

**Where:** `extensions/php-language-features/src/features/hoverProvider.ts:13-17`

**What:** Language features check workspace configuration at invocation time to conditionally enable/disable provider behavior.

```typescript
public provideHover(document: TextDocument, position: Position, _token: CancellationToken): Hover | undefined {
	const enable = workspace.getConfiguration('php').get<boolean>('suggest.basic', true);
	if (!enable) {
		return undefined;
	}
```

**Variations / call-sites:**
- `completionItemProvider.ts:15-18` – Same pattern for completion items
- `signatureHelpProvider.ts:72-75` – Same for signature help
- Pattern: All language feature methods guard with config checks before expensive operations

---

## Pattern 4: Document Text Parsing & Position-Based Introspection

**Where:** `extensions/php-language-features/src/features/completionItemProvider.ts:20-31`

**What:** Providers extract context by reading document text at position, using Range API to determine trigger context.

```typescript
let range = document.getWordRangeAtPosition(position);
const prefix = range ? document.getText(range) : '';
if (!range) {
	range = new Range(position, position);
}

if (context.triggerCharacter === '>') {
	const twoBeforeCursor = new Position(position.line, Math.max(0, position.character - 2));
	const previousTwoChars = document.getText(new Range(twoBeforeCursor, position));
	if (previousTwoChars !== '->') {
		return Promise.resolve(result);
	}
}
```

**Variations / call-sites:**
- `signatureHelpProvider.ts:77-106` – BackwardIterator class traverses document backwards for parameter counting (custom stateful parser)
- Pattern: Extensive use of Position/Range objects; prefix matching against trigger characters; document.getText() for context extraction

---

## Pattern 5: Data-Driven Completion & Hover from Static Maps

**Where:** `extensions/php-language-features/src/features/completionItemProvider.ts:66-89`

**What:** Completion items dynamically generated from static data structures (phpGlobals.globalvariables, phpGlobalFunctions.globalfunctions) by iterating and filtering against matched prefix.

```typescript
for (const globalvariables in phpGlobals.globalvariables) {
	if (phpGlobals.globalvariables.hasOwnProperty(globalvariables) && matches(globalvariables)) {
		added[globalvariables] = true;
		result.push(createNewProposal(CompletionItemKind.Variable, globalvariables, phpGlobals.globalvariables[globalvariables]));
	}
}
for (const globalfunctions in phpGlobalFunctions.globalfunctions) {
	if (phpGlobalFunctions.globalfunctions.hasOwnProperty(globalfunctions) && matches(globalfunctions)) {
		added[globalfunctions] = true;
		result.push(createNewProposal(CompletionItemKind.Function, globalfunctions, phpGlobalFunctions.globalfunctions[globalfunctions]));
	}
}
```

**Variations / call-sites:**
- `hoverProvider.ts:26` – Lookup from same maps to fetch documentation for hover: `phpGlobalFunctions.globalfunctions[name] || phpGlobals.compiletimeconstants[name]`
- `phpGlobals.ts:8-100` – Interface IEntry { description?, signature? } defines data schema
- Pattern: Deduplication via `added` set; multiple independent maps for different symbol categories; filter-then-build rather than pre-filtered data

---

## Pattern 6: Runtime Code Scanning via Regex

**Where:** `extensions/php-language-features/src/features/completionItemProvider.ts:91-111`

**What:** Extraction of user-defined variables and functions from document text via document-wide regex scanning during completion.

```typescript
const text = document.getText();
if (prefix[0] === '$') {
	const variableMatch = /\$([a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*)/g;
	let match: RegExpExecArray | null = null;
	while (match = variableMatch.exec(text)) {
		const word = match[0];
		if (!added[word]) {
			added[word] = true;
			result.push(createNewProposal(CompletionItemKind.Variable, word, null));
		}
	}
}
const functionMatch = /function\s+([a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*)\s*\(/g;
let match2: RegExpExecArray | null = null;
while (match2 = functionMatch.exec(text)) {
	const word2 = match2[1];
	if (!added[word2]) {
		added[word2] = true;
		result.push(createNewProposal(CompletionItemKind.Function, word2, null));
	}
}
```

**Variations / call-sites:**
- Pattern: Full document regex scanning only when prefix indicates $ or function keyword; deduplication against globalfunctions set
- No LSP-based indexing; purely regex-based discovery of user code

---

## Pattern 7: Workspace Event Subscription & Diagnostic Publishing

**Where:** `extensions/php-language-features/src/features/validationProvider.ts:105-156`

**What:** Provider subscribes to workspace document lifecycle events and manages per-document state (delayers) for async validation.

```typescript
public activate(subscriptions: vscode.Disposable[]) {
	this.diagnosticCollection = vscode.languages.createDiagnosticCollection();
	subscriptions.push(this);
	subscriptions.push(vscode.workspace.onDidChangeConfiguration(() => this.loadConfigP = this.loadConfiguration()));

	vscode.workspace.onDidOpenTextDocument(this.triggerValidate, this, subscriptions);
	vscode.workspace.onDidCloseTextDocument((textDocument) => {
		this.diagnosticCollection!.delete(textDocument.uri);
		if (this.delayers) {
			delete this.delayers[textDocument.uri.toString()];
		}
	}, null, subscriptions);
}
```

**Variations / call-sites:**
- `validationProvider.ts:148-155` – Switches between `onDidChangeTextDocument` and `onDidSaveTextDocument` based on configuration
- Pattern: Disposables collected in context.subscriptions; diagnostic collection cleared on config change; per-document state indexed by URI string

---

## Pattern 8: Throttled & Debounced Async Execution

**Where:** `extensions/php-language-features/src/features/utils/async.ts:172-185`

**What:** ThrottledDelayer class combines throttling and debouncing to prevent cascading rapid provider invocations during text changes.

```typescript
export class ThrottledDelayer<T> extends Delayer<Promise<T>> {

	private throttler: Throttler<T>;

	constructor(defaultDelay: number) {
		super(defaultDelay);

		this.throttler = new Throttler<T>();
	}

	public override trigger(promiseFactory: ITask<Promise<T>>, delay?: number): Promise<Promise<T>> {
		return super.trigger(() => this.throttler.queue(promiseFactory), delay);
	}
}
```

**Variations / call-sites:**
- `validationProvider.ts:169-170` – Instantiated with delay 250ms for onType, 0ms for onSave
- `async.ts:30-77` – Throttler class queues async tasks; Delayer class delays execution
- Pattern: Composition of two patterns; Delayer delays task invocation, Throttler prevents accumulation during execution

---

## Pattern 9: External Process Invocation for Validation

**Where:** `extensions/php-language-features/src/features/validationProvider.ts:176-248`

**What:** Spawns PHP CLI process as child process for linting; handles stdin/stdout streams for both buffer (onType) and file (onSave) validation modes.

```typescript
private doValidate(textDocument: vscode.TextDocument): Promise<void> {
	return new Promise<void>(resolve => {
		const executable = this.config!.executable;
		if (!executable) {
			this.showErrorMessage(vscode.l10n.t("Cannot validate since a PHP installation could not be found..."));
			this.pauseValidation = true;
			resolve();
			return;
		}

		const decoder = new LineDecoder();
		const diagnostics: vscode.Diagnostic[] = [];
		const processLine = (line: string) => {
			const matches = line.match(PHPValidationProvider.MatchExpression);
			if (matches) {
				const message = matches[1];
				const line = parseInt(matches[3]) - 1;
				const diagnostic: vscode.Diagnostic = new vscode.Diagnostic(
					new vscode.Range(line, 0, line, 2 ** 31 - 1),
					message
				);
				diagnostics.push(diagnostic);
			}
		};

		const options = (vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders[0]) ? { cwd: vscode.workspace.workspaceFolders[0].uri.fsPath } : undefined;
		let args: string[];
		if (this.config!.trigger === RunTrigger.onSave) {
			args = PHPValidationProvider.FileArgs.slice(0);
			args.push(textDocument.fileName);
		} else {
			args = PHPValidationProvider.BufferArgs;
		}
		try {
			const childProcess = cp.spawn(executable, args, options);
			childProcess.on('error', (error: Error) => {
				if (this.pauseValidation) {
					resolve();
					return;
				}
				this.showError(error, executable);
				this.pauseValidation = true;
				resolve();
			});
			if (childProcess.pid) {
				if (this.config!.trigger === RunTrigger.onType) {
					childProcess.stdin.write(textDocument.getText());
					childProcess.stdin.end();
				}
				childProcess.stdout.on('data', (data: Buffer) => {
					decoder.write(data).forEach(processLine);
				});
				childProcess.stdout.on('end', () => {
					const line = decoder.end();
					if (line) {
						processLine(line);
					}
					this.diagnosticCollection!.set(textDocument.uri, diagnostics);
					resolve();
				});
			} else {
				resolve();
			}
		} catch (error) {
			this.showError(error, executable);
		}
	});
}
```

**Variations / call-sites:**
- `validationProvider.ts:86-87` – Static arrays FileArgs and BufferArgs for PHP CLI arguments
- `validationProvider.ts:19-62` – LineDecoder helper class buffers and decodes streaming output
- Pattern: Spawns process with cwd from workspace folder; streams mode (stdin) vs file mode (file path argument); line-by-line output parsing via regex

---

## Pattern 10: Localization via vscode.l10n API

**Where:** `extensions/php-language-features/src/features/validationProvider.ts:180, 255-272`

**What:** User-facing messages localized through vscode.l10n.t() function with optional interpolation parameters.

```typescript
this.showErrorMessage(vscode.l10n.t("Cannot validate since a PHP installation could not be found. Use the setting 'php.validate.executablePath' to configure the PHP executable."));

// ...

message = vscode.l10n.t("Cannot validate since {0} is not a valid php executable. Use the setting 'php.validate.executablePath' to configure the PHP executable.", executable);

// ...

const openSettings = vscode.l10n.t("Open Settings");
if (await vscode.window.showInformationMessage(message, openSettings) === openSettings) {
	vscode.commands.executeCommand('workbench.action.openSettings', Setting.ExecutablePath);
}
```

**Variations / call-sites:**
- Pattern: Parameterized strings with {0}, {1} placeholders; all user-visible strings wrapped in vscode.l10n.t()
- No hardcoded English strings in source; all externalized for i18n

---

## Summary

The PHP extension demonstrates 10 distinct patterns essential for porting VS Code's language intelligence subsystem:

1. **Registration model**: Providers register against language IDs with optional trigger characters
2. **Interface contracts**: Typed provider interfaces with async/sync method signatures and CancellationToken support
3. **Configuration layer**: Runtime config checks gate feature activation; settings influence behavior triggers
4. **Position-based introspection**: Position/Range API + document.getText() for context extraction
5. **Static data + dynamic scanning**: Hybrid approach—curated global maps + regex scanning of user code
6. **Workspace events**: Lifecycle hooks (onOpen, onClose, onSave, onChange, onConfigChange)
7. **Async coordination**: Throttler + Delayer composition for debouncing/throttling rapid invocations
8. **Child process interaction**: Spawning external validators with stream-based I/O and line parsing
9. **Localization**: All user strings via vscode.l10n.t() with parameter interpolation
10. **Subscription management**: Disposables collected and cleaned up at appropriate lifecycle points

A Tauri/Rust port would need to replicate these contracts, event subscription patterns, and async coordination mechanisms, likely through Rust LSP libraries and custom IPC for position-based introspection and process management.
