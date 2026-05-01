# VS Code Core IDE Functionality Port Patterns: PHP Extension Analysis

## Overview
This research examines the PHP language features extension in VS Code to understand how core IDE features (language intelligence, diagnostics, completion, hover, signatures) are implemented. These patterns demonstrate the VS Code extension API architecture that would need to be ported from TypeScript/Electron to Tauri/Rust.

---

## Patterns Found

#### Pattern: Language Provider Registration Model
**Where:** `extensions/php-language-features/src/phpMain.ts:13-22`
**What:** Extension activation registers multiple language feature providers for a language ID with the VS Code language registry.
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
- `extensions/php-language-features/src/features/hoverProvider.ts:11-35` - HoverProvider implementation
- `extensions/php-language-features/src/features/completionItemProvider.ts:10-114` - CompletionItemProvider implementation
- `extensions/php-language-features/src/features/signatureHelpProvider.ts:69-106` - SignatureHelpProvider implementation

---

#### Pattern: Provider Interface Implementation with Async Support
**Where:** `extensions/php-language-features/src/features/completionItemProvider.ts:10-113`
**What:** Providers implement vscode interfaces and return Promise-based results for non-blocking completion proposals during user typing.
```typescript
export default class PHPCompletionItemProvider implements CompletionItemProvider {

	public provideCompletionItems(document: TextDocument, position: Position, _token: CancellationToken, context: CompletionContext): Promise<CompletionItem[]> {
		const result: CompletionItem[] = [];

		const shouldProvideCompletionItems = workspace.getConfiguration('php').get<boolean>('suggest.basic', true);
		if (!shouldProvideCompletionItems) {
			return Promise.resolve(result);
		}

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

		const added: any = {};
		const createNewProposal = function (kind: CompletionItemKind, name: string, entry: phpGlobals.IEntry | null): CompletionItem {
			const proposal: CompletionItem = new CompletionItem(name);
			proposal.kind = kind;
			if (entry) {
				if (entry.description) {
					proposal.documentation = entry.description;
				}
				if (entry.signature) {
					proposal.detail = entry.signature;
				}
			}
			return proposal;
		};

		const matches = (name: string) => {
			return prefix.length === 0 || name.length >= prefix.length && name.substr(0, prefix.length) === prefix;
		};

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
		return Promise.resolve(result);
	}
}
```
**Variations / call-sites:**
- `extensions/php-language-features/src/features/hoverProvider.ts:11-35` - Hover results wrapped in Hover object
- `extensions/php-language-features/src/features/signatureHelpProvider.ts:71-106` - Signature help wrapped in Promise<SignatureHelp>

---

#### Pattern: Configuration-Driven Feature Enablement
**Where:** `extensions/php-language-features/src/features/hoverProvider.ts:11-35`
**What:** Language providers check workspace configuration before providing intelligence results, allowing fine-grained feature toggling.
```typescript
export default class PHPHoverProvider implements HoverProvider {

	public provideHover(document: TextDocument, position: Position, _token: CancellationToken): Hover | undefined {
		const enable = workspace.getConfiguration('php').get<boolean>('suggest.basic', true);
		if (!enable) {
			return undefined;
		}

		const wordRange = document.getWordRangeAtPosition(position);
		if (!wordRange) {
			return undefined;
		}

		const name = document.getText(wordRange);

		const entry = phpGlobalFunctions.globalfunctions[name] || phpGlobals.compiletimeconstants[name] || phpGlobals.globalvariables[name] || phpGlobals.keywords[name];
		if (entry && entry.description) {
			const signature = name + (entry.signature || '');
			const contents: MarkedString[] = [textToMarkedString(entry.description), { language: 'php', value: signature }];
			return new Hover(contents, wordRange);
		}

		return undefined;
	}
}
```
**Variations / call-sites:**
- `extensions/php-language-features/src/features/completionItemProvider.ts:15` - Completion items check same config
- `extensions/php-language-features/src/features/signatureHelpProvider.ts:72` - Signature help checks same config

---

#### Pattern: Diagnostic Collection for Validation
**Where:** `extensions/php-language-features/src/features/validationProvider.ts:84-275`
**What:** Validation provider manages a DiagnosticCollection that accumulates parse errors from external tools and publishes them back to the editor.
```typescript
export default class PHPValidationProvider {

	private static MatchExpression: RegExp = /(?:(?:Parse|Fatal) error): (.*)(?: in )(.*?)(?: on line )(\d+)/;
	private static BufferArgs: string[] = ['-l', '-n', '-d', 'display_errors=On', '-d', 'log_errors=Off'];
	private static FileArgs: string[] = ['-l', '-n', '-d', 'display_errors=On', '-d', 'log_errors=Off', '-f'];

	private validationEnabled: boolean;
	private pauseValidation: boolean;
	private config: IPhpConfig | undefined;
	private loadConfigP: Promise<void>;

	private documentListener: vscode.Disposable | null = null;
	private diagnosticCollection?: vscode.DiagnosticCollection;
	private delayers?: { [key: string]: ThrottledDelayer<void> };

	constructor() {
		this.validationEnabled = true;
		this.pauseValidation = false;
		this.loadConfigP = this.loadConfiguration();
	}

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

	public dispose(): void {
		if (this.diagnosticCollection) {
			this.diagnosticCollection.clear();
			this.diagnosticCollection.dispose();
		}
		if (this.documentListener) {
			this.documentListener.dispose();
			this.documentListener = null;
		}
	}

	private async loadConfiguration(): Promise<void> {
		const section = vscode.workspace.getConfiguration();
		const oldExecutable = this.config?.executable;
		this.validationEnabled = section.get<boolean>(Setting.Enable, true);

		this.config = await getConfig();

		this.delayers = Object.create(null);
		if (this.pauseValidation) {
			this.pauseValidation = oldExecutable === this.config.executable;
		}
		if (this.documentListener) {
			this.documentListener.dispose();
			this.documentListener = null;
		}
		this.diagnosticCollection!.clear();
		if (this.validationEnabled) {
			if (this.config.trigger === RunTrigger.onType) {
				this.documentListener = vscode.workspace.onDidChangeTextDocument((e) => {
					this.triggerValidate(e.document);
				});
			} else {
				this.documentListener = vscode.workspace.onDidSaveTextDocument(this.triggerValidate, this);
			}
			// Configuration has changed. Reevaluate all documents.
			vscode.workspace.textDocuments.forEach(this.triggerValidate, this);
		}
	}

	private async triggerValidate(textDocument: vscode.TextDocument): Promise<void> {
		await this.loadConfigP;
		if (textDocument.languageId !== 'php' || this.pauseValidation || !this.validationEnabled) {
			return;
		}

		if (vscode.workspace.isTrusted) {
			const key = textDocument.uri.toString();
			let delayer = this.delayers![key];
			if (!delayer) {
				delayer = new ThrottledDelayer<void>(this.config?.trigger === RunTrigger.onType ? 250 : 0);
				this.delayers![key] = delayer;
			}
			delayer.trigger(() => this.doValidate(textDocument));
		}
	}

	private doValidate(textDocument: vscode.TextDocument): Promise<void> {
		return new Promise<void>(resolve => {
			const executable = this.config!.executable;
			if (!executable) {
				this.showErrorMessage(vscode.l10n.t("Cannot validate since a PHP installation could not be found. Use the setting 'php.validate.executablePath' to configure the PHP executable."));
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
}
```
**Variations / call-sites:**
- `extensions/php-language-features/src/features/validationProvider.ts:106` - DiagnosticCollection creation
- `extensions/php-language-features/src/features/validationProvider.ts:239` - Diagnostic publication via set()

---

#### Pattern: Document Event Subscription & Lifecycle Management
**Where:** `extensions/php-language-features/src/features/validationProvider.ts:105-156`
**What:** Validation provider subscribes to document lifecycle events (open, close, change, save) and manages event listeners with disposal pattern for cleanup.
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

private async loadConfiguration(): Promise<void> {
	const section = vscode.workspace.getConfiguration();
	const oldExecutable = this.config?.executable;
	this.validationEnabled = section.get<boolean>(Setting.Enable, true);

	this.config = await getConfig();

	this.delayers = Object.create(null);
	if (this.pauseValidation) {
		this.pauseValidation = oldExecutable === this.config.executable;
	}
	if (this.documentListener) {
		this.documentListener.dispose();
		this.documentListener = null;
	}
	this.diagnosticCollection!.clear();
	if (this.validationEnabled) {
		if (this.config.trigger === RunTrigger.onType) {
			this.documentListener = vscode.workspace.onDidChangeTextDocument((e) => {
				this.triggerValidate(e.document);
			});
		} else {
			this.documentListener = vscode.workspace.onDidSaveTextDocument(this.triggerValidate, this);
		}
		// Configuration has changed. Reevaluate all documents.
		vscode.workspace.textDocuments.forEach(this.triggerValidate, this);
	}
}
```
**Variations / call-sites:**
- `extensions/php-language-features/src/features/validationProvider.ts:110-116` - Open/Close document handling
- `extensions/php-language-features/src/features/validationProvider.ts:148-152` - Change/Save document handling

---

#### Pattern: Throttled & Delayed Async Task Execution
**Where:** `extensions/php-language-features/src/features/utils/async.ts:30-185`
**What:** Throttler and Delayer utilities prevent excessive async operations; ThrottledDelayer combines both to debounce rapid events and batch sequential executions.
```typescript
export class Throttler<T> {

	private activePromise: Promise<T> | null;
	private queuedPromise: Promise<T> | null;
	private queuedPromiseFactory: ITask<Promise<T>> | null;

	constructor() {
		this.activePromise = null;
		this.queuedPromise = null;
		this.queuedPromiseFactory = null;
	}

	public queue(promiseFactory: ITask<Promise<T>>): Promise<T> {
		if (this.activePromise) {
			this.queuedPromiseFactory = promiseFactory;

			if (!this.queuedPromise) {
				const onComplete = () => {
					this.queuedPromise = null;

					const result = this.queue(this.queuedPromiseFactory!);
					this.queuedPromiseFactory = null;

					return result;
				};

				this.queuedPromise = new Promise<T>((resolve) => {
					this.activePromise!.then(onComplete, onComplete).then(resolve);
				});
			}

			return new Promise<T>((resolve, reject) => {
				this.queuedPromise!.then(resolve, reject);
			});
		}

		this.activePromise = promiseFactory();

		return new Promise<T>((resolve, reject) => {
			this.activePromise!.then((result: T) => {
				this.activePromise = null;
				resolve(result);
			}, (err: any) => {
				this.activePromise = null;
				reject(err);
			});
		});
	}
}

export class Delayer<T> {

	public defaultDelay: number;
	private timeout: NodeJS.Timer | null;
	private completionPromise: Promise<T> | null;
	private onResolve: ((value: T | PromiseLike<T> | undefined) => void) | null;
	private task: ITask<T> | null;

	constructor(defaultDelay: number) {
		this.defaultDelay = defaultDelay;
		this.timeout = null;
		this.completionPromise = null;
		this.onResolve = null;
		this.task = null;
	}

	public trigger(task: ITask<T>, delay: number = this.defaultDelay): Promise<T> {
		this.task = task;
		this.cancelTimeout();

		if (!this.completionPromise) {
			this.completionPromise = new Promise<T | undefined>((resolve) => {
				this.onResolve = resolve;
			}).then(() => {
				this.completionPromise = null;
				this.onResolve = null;

				const result = this.task!();
				this.task = null;

				return result;
			});
		}

		this.timeout = setTimeout(() => {
			this.timeout = null;
			this.onResolve!(undefined);
		}, delay);

		return this.completionPromise;
	}
}

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
- `extensions/php-language-features/src/features/validationProvider.ts:169` - ThrottledDelayer instantiated with 250ms delay for onType validation

---

#### Pattern: External Tool Integration via Child Process
**Where:** `extensions/php-language-features/src/features/validationProvider.ts:176-249`
**What:** Validation provider spawns external PHP process to lint documents, either via stdin (onType) or file args (onSave), capturing stderr/stdout for diagnostic reporting.
```typescript
private doValidate(textDocument: vscode.TextDocument): Promise<void> {
	return new Promise<void>(resolve => {
		const executable = this.config!.executable;
		if (!executable) {
			this.showErrorMessage(vscode.l10n.t("Cannot validate since a PHP installation could not be found. Use the setting 'php.validate.executablePath' to configure the PHP executable."));
			this.pauseValidation = true;
			resolve();
			return;
		}

		if (!path.isAbsolute(executable)) {
			// executable should either be resolved to an absolute path or undefined.
			// This is just to be sure.
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
- `extensions/php-language-features/src/features/validationProvider.ts:19-62` - LineDecoder utility for buffered stream handling

---

#### Pattern: Backward Text Parsing for Context Analysis
**Where:** `extensions/php-language-features/src/features/signatureHelpProvider.ts:33-172`
**What:** SignatureHelpProvider uses a BackwardIterator to traverse document text in reverse, parsing syntax (brackets, quotes, commas) to identify parameter position for signature help.
```typescript
class BackwardIterator {
	private lineNumber: number;
	private offset: number;
	private line: string;
	private model: TextDocument;

	constructor(model: TextDocument, offset: number, lineNumber: number) {
		this.lineNumber = lineNumber;
		this.offset = offset;
		this.line = model.lineAt(this.lineNumber).text;
		this.model = model;
	}

	public hasNext(): boolean {
		return this.lineNumber >= 0;
	}

	public next(): number {
		if (this.offset < 0) {
			if (this.lineNumber > 0) {
				this.lineNumber--;
				this.line = this.model.lineAt(this.lineNumber).text;
				this.offset = this.line.length - 1;
				return _NL;
			}
			this.lineNumber = -1;
			return BOF;
		}
		const ch = this.line.charCodeAt(this.offset);
		this.offset--;
		return ch;
	}
}

export default class PHPSignatureHelpProvider implements SignatureHelpProvider {

	public provideSignatureHelp(document: TextDocument, position: Position, _token: CancellationToken): Promise<SignatureHelp> | null {
		const enable = workspace.getConfiguration('php').get<boolean>('suggest.basic', true);
		if (!enable) {
			return null;
		}

		const iterator = new BackwardIterator(document, position.character - 1, position.line);

		const paramCount = this.readArguments(iterator);
		if (paramCount < 0) {
			return null;
		}

		const ident = this.readIdent(iterator);
		if (!ident) {
			return null;
		}

		const entry = phpGlobalFunctions.globalfunctions[ident] || phpGlobals.keywords[ident];
		if (!entry || !entry.signature) {
			return null;
		}
		const paramsString = entry.signature.substring(0, entry.signature.lastIndexOf(')') + 1);
		const signatureInfo = new SignatureInformation(ident + paramsString, entry.description);

		const re = /\w*\s+\&?\$[\w_\.]+|void/g;
		let match: RegExpExecArray | null = null;
		while ((match = re.exec(paramsString)) !== null) {
			signatureInfo.parameters.push({ label: match[0], documentation: '' });
		}
		const ret = new SignatureHelp();
		ret.signatures.push(signatureInfo);
		ret.activeSignature = 0;
		ret.activeParameter = Math.min(paramCount, signatureInfo.parameters.length - 1);
		return Promise.resolve(ret);
	}

	private readArguments(iterator: BackwardIterator): number {
		let parentNesting = 0;
		let bracketNesting = 0;
		let curlyNesting = 0;
		let paramCount = 0;
		while (iterator.hasNext()) {
			const ch = iterator.next();
			switch (ch) {
				case _LParent:
					parentNesting--;
					if (parentNesting < 0) {
						return paramCount;
					}
					break;
				case _RParent: parentNesting++; break;
				case _LCurly: curlyNesting--; break;
				case _RCurly: curlyNesting++; break;
				case _LBracket: bracketNesting--; break;
				case _RBracket: bracketNesting++; break;
				case _DQuote:
				case _Quote:
					while (iterator.hasNext() && ch !== iterator.next()) {
						// find the closing quote or double quote
					}
					break;
				case _Comma:
					if (!parentNesting && !bracketNesting && !curlyNesting) {
						paramCount++;
					}
					break;
			}
		}
		return -1;
	}

	private isIdentPart(ch: number): boolean {
		if (ch === _USC || // _
			ch >= _a && ch <= _z || // a-z
			ch >= _A && ch <= _Z || // A-Z
			ch >= _0 && ch <= _9 || // 0/9
			ch >= 0x80 && ch <= 0xFFFF) { // nonascii

			return true;
		}
		return false;
	}

	private readIdent(iterator: BackwardIterator): string {
		let identStarted = false;
		let ident = '';
		while (iterator.hasNext()) {
			const ch = iterator.next();
			if (!identStarted && (ch === _WSB || ch === _TAB || ch === _NL)) {
				continue;
			}
			if (this.isIdentPart(ch)) {
				identStarted = true;
				ident = String.fromCharCode(ch) + ident;
			} else if (identStarted) {
				return ident;
			}
		}
		return ident;
	}
}
```
**Variations / call-sites:**
- `extensions/php-language-features/src/features/signatureHelpProvider.ts:108-141` - readArguments for parameter counting
- `extensions/php-language-features/src/features/signatureHelpProvider.ts:155-171` - readIdent for function name extraction

---

## Summary

The PHP extension demonstrates seven core patterns essential to IDE functionality that would require port consideration:

1. **Language Provider Registry**: VS Code's pluggable provider model (`registerHoverProvider`, `registerCompletionItemProvider`, `registerSignatureHelpProvider`) that decouples features from the core editor.

2. **Provider Interface Contract**: Standardized interfaces (HoverProvider, CompletionItemProvider, SignatureHelpProvider) with Promise-based async returns, CancellationToken support, and rich return types.

3. **Configuration System**: Workspace-scoped configuration (via `workspace.getConfiguration()`) that enables runtime feature toggling without reloading.

4. **Diagnostic Collection**: Centralized DiagnosticCollection API for publishing validation errors back to the editor with file:line:message structure.

5. **Document Lifecycle Events**: Granular event subscriptions (onDidOpen, onDidClose, onDidChange, onDidSave, onDidChangeConfiguration) with Disposable-based cleanup patterns.

6. **Async Task Control**: Throttler and Delayer utilities prevent resource exhaustion from rapid events (especially during typing), using Promise queuing and timeout debouncing.

7. **External Process Integration**: Child process spawning with stdin/stdout capture for delegating expensive operations (linting) to native tools, with streaming result parsing.

**Porting Implications**: A Tauri/Rust port would need to replicate this extension API surface entirely—the provider registration model, configuration system, document event bus, diagnostic publishing, and async task coordination are all foundational to VS Code's IDE functionality. These patterns suggest that the extension layer itself may be the largest surface area to port, requiring careful design of an IPC boundary between the native Rust core and any remaining TypeScript providers.
