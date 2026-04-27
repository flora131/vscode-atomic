# Pattern Research: VS Code IDE Functionality Port to Tauri/Rust

## Scope
PHP language features extension (`extensions/php-language-features/`, 11 files, 7,255 LOC)

---

#### Pattern: Language Server Provider Registration
**Where:** `extensions/php-language-features/src/phpMain.ts:19-21`
**What:** Central activation hook registers multiple language-specific providers (completion, hover, signature help) with VS Code's language API.

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
- Each provider registers independently with `vscode.languages.register*Provider()`
- Subscriptions are pushed to extension context for lifecycle management
- Trigger characters passed for completion/signature providers

---

#### Pattern: CompletionItemProvider Implementation
**Where:** `extensions/php-language-features/src/features/completionItemProvider.ts:10-114`
**What:** Implements VS Code's CompletionItemProvider interface; provides symbol matching, prefix filtering, and documentation integration.

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
		
		// Check trigger character
		if (context.triggerCharacter === '>') {
			const twoBeforeCursor = new Position(position.line, Math.max(0, position.character - 2));
			const previousTwoChars = document.getText(new Range(twoBeforeCursor, position));
			if (previousTwoChars !== '->') {
				return Promise.resolve(result);
			}
		}

		// Build proposals from global functions, variables, keywords
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

		// ... scan document text for variables/functions via regex
		return Promise.resolve(result);
	}
}
```

**Variations / call-sites:**
- `extensions/php-language-features/src/features/hoverProvider.ts:11-35` - Similar interface pattern
- `extensions/php-language-features/src/features/signatureHelpProvider.ts:69-106` - Signature help variant

---

#### Pattern: Event-Driven Lifecycle & Document Listening
**Where:** `extensions/php-language-features/src/features/validationProvider.ts:105-117`
**What:** Manages document lifecycle through VS Code workspace events (open, close, change, save, configuration); uses subscriptions array for cleanup.

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
```

**Variations / call-sites:**
- `validationProvider.ts:148-152` - Conditional listener registration based on `RunTrigger.onType` vs `RunTrigger.onSave`
- Pattern uses `onDidChangeConfiguration` callback (line 108) for settings reloading

---

#### Pattern: Process Spawning for External Tool Integration
**Where:** `extensions/php-language-features/src/features/validationProvider.ts:216-241`
**What:** Spawns child process (PHP interpreter), pipes document content or file path, captures stdout for diagnostics, handles errors.

```typescript
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
}
```

**Variations / call-sites:**
- Line 87-88 define static argument arrays (`BufferArgs`, `FileArgs`)
- Line 195-204 define `processLine` callback that parses stdout via regex and creates `vscode.Diagnostic` objects

---

#### Pattern: Throttled Async Task Queueing
**Where:** `extensions/php-language-features/src/features/utils/async.ts:172-185`
**What:** `ThrottledDelayer<T>` combines throttling (prevent concurrent executions) and delaying (debounce rapid requests) for async operations—common for editor performance.

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
- `validationProvider.ts:169` instantiates `ThrottledDelayer<void>(250)` for on-type validation with 250ms delay
- Base `Throttler<T>` (lines 30-78) queues concurrent requests
- Base `Delayer<T>` (lines 103-163) delays execution with configurable timeout

---

#### Pattern: Configuration-Driven Provider Behavior
**Where:** `extensions/php-language-features/package.json:30-63`
**What:** Extension declares contributes.configuration schema with boolean/enum settings; providers read config via `workspace.getConfiguration('php').get<T>()`.

```json
{
  "contributes": {
    "configuration": {
      "title": "%configuration.title%",
      "type": "object",
      "order": 20,
      "properties": {
        "php.suggest.basic": {
          "type": "boolean",
          "default": true,
          "description": "%configuration.suggest.basic%"
        },
        "php.validate.enable": {
          "type": "boolean",
          "default": true,
          "description": "%configuration.validate.enable%"
        },
        "php.validate.executablePath": {
          "type": ["string", "null"],
          "default": null,
          "description": "%configuration.validate.executablePath%",
          "scope": "machine-overridable"
        },
        "php.validate.run": {
          "type": "string",
          "enum": ["onSave", "onType"],
          "default": "onSave",
          "description": "%configuration.validate.run%"
        }
      }
    }
  }
}
```

**Variations / call-sites:**
- `completionItemProvider.ts:15` - `workspace.getConfiguration('php').get<boolean>('suggest.basic', true)`
- `signatureHelpProvider.ts:72` - Same config read
- `hoverProvider.ts:14` - Same config read
- `validationProvider.ts:133, 311` - Reads enable/run/executablePath settings

---

#### Pattern: Data-Driven Symbol Definitions
**Where:** `extensions/php-language-features/src/features/phpGlobals.ts:8-57`
**What:** Static data structures map symbol names to metadata (description, signature); enables unified lookup across global variables, functions, constants, keywords.

```typescript
export interface IEntry { description?: string; signature?: string }
export interface IEntries { [name: string]: IEntry }

export const globalvariables: IEntries = {
	$GLOBALS: {
		description: 'An associative array containing references to all variables which are currently defined in the global scope of the script...',
	},
	$_SERVER: {
		description: '$_SERVER is an array containing information such as headers, paths, and script locations...',
	},
	// ... 15+ more entries
};

export const compiletimeconstants: IEntries = {
	__CLASS__: {
		description: 'The class name...',
	},
	__DIR__: {
		description: 'The directory of the file...',
	},
	// ... many more
};

export const keywords: IEntries = { /* ... */ };
```

**Variations / call-sites:**
- `completionItemProvider.ts:66-89` - Iterates all four data sources to build proposals
- `hoverProvider.ts:26` - Looks up single symbol in merged data: `phpGlobalFunctions.globalfunctions[name] || phpGlobals.compiletimeconstants[name] || ...`

---

## Summary

This PHP extension demonstrates core VS Code IDE patterns critical for a Tauri/Rust port:

1. **Provider Registration Model**: Language features are registered at activation via context subscriptions—enables dynamic loading and lifecycle management.
2. **Async/Promise-based API**: All provider methods return `Promise<T>` or nullable values, requiring port to support async Rust equivalents.
3. **Event Subscription Architecture**: Heavy use of `workspace.onDid*` events for document/configuration changes; requires pub/sub or signal mechanism in Rust.
4. **Configuration System**: Declarative schema in `package.json`; runtime config reads via `workspace.getConfiguration()` with typed fallbacks.
5. **External Process Integration**: Spawns child processes (PHP binary), pipes stdin, captures structured stdout—requires Rust `std::process` or similar.
6. **Throttling/Debouncing Utilities**: Custom async helpers (`Throttler`, `Delayer`, `ThrottledDelayer`) for performance; Rust port needs equivalent async cancellation tokens.
7. **URI/Document Abstraction**: Document lifecycle tracked via `TextDocument` and `vscode.Uri`; Rust port requires equivalent abstraction for file handles, text buffers, and change tracking.
8. **Diagnostic Publishing**: Providers emit `vscode.Diagnostic[]` to a `DiagnosticCollection`; Rust port needs language server protocol or equivalent diagnostic channel.

Key architectural insight: VS Code separates provider logic (stateless feature implementations) from lifecycle/subscription management (stateful activation context). A Rust port would benefit from similar separation—data models for symbols/configs, pure language logic, and a bridging RPC/IPC layer to communicate with UI.
