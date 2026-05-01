# Partition 16 of 79 — Findings

## Scope
`extensions/php-language-features/` (11 files, 7,255 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# PHP Language Features Extension - IDE Porting Analysis

## Implementation
- `extensions/php-language-features/src/phpMain.ts` — Extension entry point registering language service providers (hover, completion, signature help, diagnostics)
- `extensions/php-language-features/src/features/hoverProvider.ts` — Hover information provider for PHP language intelligence
- `extensions/php-language-features/src/features/completionItemProvider.ts` — Autocomplete/IntelliSense provider for PHP
- `extensions/php-language-features/src/features/signatureHelpProvider.ts` — Function signature help provider for PHP
- `extensions/php-language-features/src/features/validationProvider.ts` — Diagnostic validation and error reporting tied to workspace/document events
- `extensions/php-language-features/src/features/phpGlobalFunctions.ts` — PHP built-in function definitions for language intelligence
- `extensions/php-language-features/src/features/phpGlobals.ts` — Global PHP constants and built-in identifiers
- `extensions/php-language-features/src/features/utils/async.ts` — Async utility helpers for provider implementations
- `extensions/php-language-features/src/features/utils/markedTextUtil.ts` — Markdown formatting utilities for hover/completion documentation

## Configuration
- `extensions/php-language-features/package.json` — Extension manifest with language server activation events, capabilities (virtualWorkspaces, untrustedWorkspaces), config schema for php.suggest.basic, php.validate.enable/run/executablePath, and composer.json JSON schema
- `extensions/php-language-features/tsconfig.json` — TypeScript compiler configuration extending base config with vscode.d.ts type definitions
- `extensions/php-language-features/esbuild.mts` — Build configuration for extension bundling
- `extensions/php-language-features/.npmrc` — NPM configuration for package management

## Types / Interfaces
- `extensions/php-language-features/src/typings/node.additions.d.ts` — Custom Node.js type augmentations

## Documentation
- `extensions/php-language-features/README.md` — Extension user-facing documentation
- `extensions/php-language-features/package.nls.json` — Localization strings for UI text

## Notable Clusters
- `extensions/php-language-features/src/features/` — 7 provider implementations (completion, hover, signature help, validation) plus utilities for async operations and text rendering; forms the core language service provider layer
- `extensions/php-language-features/src/features/utils/` — 2 utility modules for async handling and markdown formatting used by providers

## Porting Relevance

This extension exemplifies VS Code's language intelligence architecture that would require reimplementation in a Tauri/Rust port:

1. **Language Provider Registration Model** — Uses `vscode.languages.registerXyzProvider()` API with pluggable provider classes; any Tauri port must establish equivalent provider registration and dispatch mechanisms

2. **Configuration System** — Dynamic workspace configuration via `vscode.workspace.getConfiguration()` tied to user settings; Tauri port requires equivalent config resolution and change event propagation

3. **Event-Driven Architecture** — Validation triggered by workspace events (`onDidChangeTextDocument`, `onDidSaveTextDocument`, `onDidChangeConfiguration`); demonstrates event subscription patterns critical for IDE responsiveness

4. **Document/Workspace Abstraction** — Uses `vscode.workspace.textDocuments`, `vscode.workspace.workspaceFolders`, and document URIs for file system abstraction; Tauri port needs equivalent document model

5. **Diagnostics Collection** — `vscode.languages.createDiagnosticCollection()` for reporting validation errors; Tauri port needs diagnostic storage and rendering infrastructure

6. **Async/Promise-Based APIs** — Providers return Promises for hover content, completions, etc.; Tauri port must support async RPC or message-based provider calls

7. **Process Execution** — Uses `child_process` (via `which` package) to locate and execute PHP validator; Tauri port needs subprocess management for external tool integration

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

1. `extensions/php-language-features/src/phpMain.ts` (23 lines)
2. `extensions/php-language-features/src/features/validationProvider.ts` (326 lines)
3. `extensions/php-language-features/src/features/hoverProvider.ts` (35 lines)
4. `extensions/php-language-features/src/features/completionItemProvider.ts` (114 lines)
5. `extensions/php-language-features/src/features/signatureHelpProvider.ts` (173 lines)
6. `extensions/php-language-features/src/features/utils/async.ts` (185 lines)
7. `extensions/php-language-features/src/features/utils/markedTextUtil.ts` (10 lines)
8. `extensions/php-language-features/src/features/phpGlobals.ts` (324 lines)
9. `extensions/php-language-features/src/features/phpGlobalFunctions.ts` (6039 lines, first 60 read)

---

### Per-File Notes

#### `extensions/php-language-features/src/phpMain.ts`

- **Role:** Extension entry point. Instantiates all language feature providers and registers them with the VS Code extension API. Activation is triggered only when a PHP document is opened (declared via `activationEvents: ["onLanguage:php"]` in `package.json`).
- **Key symbols:**
  - `activate(context: vscode.ExtensionContext)` at line 13 — the single exported function called by the VS Code host on activation.
  - `PHPValidationProvider`, `PHPCompletionItemProvider`, `PHPHoverProvider`, `PHPSignatureHelpProvider` — all instantiated here.
- **Control flow:**
  1. `activate` constructs a `PHPValidationProvider` and calls `validator.activate(context.subscriptions)` at line 16.
  2. Three provider registrations are pushed onto `context.subscriptions` (lines 19–21):
     - `vscode.languages.registerCompletionItemProvider('php', new PHPCompletionItemProvider(), '>', '$')` — triggers on `>` and `$` characters.
     - `vscode.languages.registerHoverProvider('php', new PHPHoverProvider())`.
     - `vscode.languages.registerSignatureHelpProvider('php', new PHPSignatureHelpProvider(), '(', ',')` — triggers on `(` and `,`.
- **Data flow:** The `context.subscriptions` array accumulates disposables. VS Code disposes them on extension deactivation, tying provider lifetime to the extension host process lifetime.
- **Dependencies:** `vscode` API (from `@types/vscode`), the four feature modules.

---

#### `extensions/php-language-features/src/features/validationProvider.ts`

- **Role:** Drives PHP syntax validation by spawning the user's locally-installed `php` binary as a child process and parsing its stderr output into VS Code `Diagnostic` objects. This is a "linter" approach using an external process rather than any embedded parser or language server.
- **Key symbols:**
  - `LineDecoder` class (lines 19–63) — wraps Node.js `StringDecoder` to split streaming binary stdout into complete line strings. Handles partial lines via a `remaining` buffer.
  - `RunTrigger` enum (lines 65–82) — `onSave` or `onType` modes. Parsed from the `php.validate.run` setting via `RunTrigger.from()`.
  - `PHPValidationProvider` class (lines 84–275):
    - `MatchExpression` (line 86): `/(?:(?:Parse|Fatal) error): (.*)(?: in )(.*?)(?: on line )(\d+)/` — regex to parse PHP CLI error output.
    - `BufferArgs` (line 87): `['-l', '-n', '-d', 'display_errors=On', '-d', 'log_errors=Off']` for stdin (onType) mode.
    - `FileArgs` (line 88): same plus `-f <filename>` for file (onSave) mode.
    - `activate(subscriptions)` at line 105 — creates `DiagnosticCollection`, subscribes to workspace events.
    - `loadConfiguration()` at line 130 — reads settings, resolves executable path via `which('php')` fallback, sets up document listeners based on trigger mode.
    - `triggerValidate(textDocument)` at line 159 — guard-checks workspace trust (`vscode.workspace.isTrusted` at line 165), then uses `ThrottledDelayer` (250 ms for onType, 0 ms for onSave) to debounce calls to `doValidate`.
    - `doValidate(textDocument)` at line 176 — core logic: spawns `cp.spawn(executable, args, options)`, pipes document text to stdin (onType mode, line 228–230), collects stdout via `LineDecoder`, and calls `diagnosticCollection.set(textDocument.uri, diagnostics)` at line 239.
  - `getConfig()` (line 283) — inspects workspace vs. global settings for `php.validate.executablePath`; resolves relative paths against first workspace folder; falls back to `getPhpPath()`.
  - `getPhpPath()` (line 319) — calls `which('php')` to locate the PHP binary on PATH.
- **Control flow:**
  1. On document open → `triggerValidate` → workspace-trust check → `ThrottledDelayer.trigger(doValidate)`.
  2. `doValidate` → `cp.spawn` → child process stdout events → `LineDecoder.write()` → `MatchExpression.exec()` → build `vscode.Diagnostic[]` → `diagnosticCollection.set()`.
  3. On document close → `diagnosticCollection.delete(uri)` and remove delayer entry.
  4. On config change → `loadConfiguration()` re-runs, clears diagnostics, resubscribes to listeners, re-validates all open documents.
- **Data flow:** `TextDocument.getText()` → child process stdin → PHP CLI stdout → `LineDecoder` → regex parse → `vscode.Diagnostic` → `DiagnosticCollection` (rendered in the Problems panel by VS Code's core).
- **Dependencies:** Node.js `child_process`, `string_decoder`, `which` (npm package for PATH lookup), `path`, `vscode`, `ThrottledDelayer` from `./utils/async`.

---

#### `extensions/php-language-features/src/features/hoverProvider.ts`

- **Role:** Implements `HoverProvider` for PHP. Returns hover documentation for PHP built-in functions, compile-time constants, global variables, and keywords. Entirely static — no dynamic analysis, no LSP calls.
- **Key symbols:**
  - `PHPHoverProvider` class implementing `vscode.HoverProvider` (line 11).
  - `provideHover(document, position, _token)` (line 13) — the single method required by the `HoverProvider` interface.
- **Control flow:**
  1. Checks `php.suggest.basic` config flag; returns `undefined` if disabled (line 14–17).
  2. Calls `document.getWordRangeAtPosition(position)` (line 19) to extract the token under the cursor.
  3. Looks up the token text in priority order: `phpGlobalFunctions.globalfunctions[name]` || `phpGlobals.compiletimeconstants[name]` || `phpGlobals.globalvariables[name]` || `phpGlobals.keywords[name]` (line 26).
  4. If found and has a `description`, builds `MarkedString[]` with: escaped description text via `textToMarkedString`, and a PHP code block with the signature (line 29).
  5. Returns `new Hover(contents, wordRange)` (line 30).
- **Data flow:** `position` → `getWordRangeAtPosition` → `document.getText(range)` (token string) → static dictionary lookup → `Hover` object → VS Code renders tooltip.
- **Dependencies:** `vscode`, `textToMarkedString` (from `./utils/markedTextUtil`), `phpGlobals`, `phpGlobalFunctions`.

---

#### `extensions/php-language-features/src/features/completionItemProvider.ts`

- **Role:** Implements `CompletionItemProvider` for PHP. Provides IntelliSense completions from three sources: static built-in definitions, compile-time constants/keywords, and dynamic extraction of variables/functions from the current document text via regex scan.
- **Key symbols:**
  - `PHPCompletionItemProvider` class implementing `vscode.CompletionItemProvider` (line 10).
  - `provideCompletionItems(document, position, _token, context)` (line 12).
  - `createNewProposal(kind, name, entry)` (inner function, line 35) — constructs a `CompletionItem` with `kind`, optional `documentation` from `entry.description`, optional `detail` from `entry.signature`.
  - `matches(name)` (inner function, line 49) — prefix-match predicate.
- **Control flow:**
  1. Checks `php.suggest.basic` flag (line 15). Returns empty if disabled.
  2. Gets word range and prefix at position (lines 20–24).
  3. Arrow trigger guard: if triggered by `>`, checks for `->` two chars before; bails if not (lines 26–33).
  4. Special case for `<?php` completion when prefix is `php` after `<?` (lines 53–64).
  5. Iterates over `phpGlobals.globalvariables`, `phpGlobalFunctions.globalfunctions`, `phpGlobals.compiletimeconstants`, `phpGlobals.keywords` (lines 66–89), adding matching entries with appropriate `CompletionItemKind`.
  6. If prefix starts with `$`, scans full document text with regex `/\$([a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*)/g` for variable names (lines 92–101).
  7. Scans full document text with regex `/function\s+([a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*)\s*\(/g` for user-defined functions (lines 103–111).
  8. Returns `Promise.resolve(result)`.
- **Data flow:** `document.getText()` (full source) → two regex scans → merged with static dict lookups → `CompletionItem[]` → VS Code completion UI.
- **Dependencies:** `vscode`, `phpGlobalFunctions`, `phpGlobals`.

---

#### `extensions/php-language-features/src/features/signatureHelpProvider.ts`

- **Role:** Implements `SignatureHelpProvider` for PHP. Determines which built-in function is being called at the cursor and which parameter the cursor is on, then returns the function signature with the active parameter highlighted.
- **Key symbols:**
  - `BackwardIterator` class (lines 33–66) — walks the `TextDocument` backward character-by-character, crossing line boundaries. `next()` returns char codes or `_NL`/`BOF` sentinels (line 50–64).
  - `PHPSignatureHelpProvider` class (line 69) implementing `vscode.SignatureHelpProvider`.
  - `provideSignatureHelp(document, position, _token)` (line 71).
  - `readArguments(iterator)` (line 108) — counts commas at the current nesting level to determine active parameter index. Returns `-1` if no enclosing `(` found.
  - `readIdent(iterator)` (line 155) — reads the identifier just before `(` by consuming whitespace then identifier characters in reverse.
- **Control flow:**
  1. Checks `php.suggest.basic` flag (line 72–75).
  2. Constructs `BackwardIterator` starting one char before cursor position (line 77).
  3. Calls `readArguments(iterator)` — if returns < 0, no function call context, returns `null` (line 79–81).
  4. Calls `readIdent(iterator)` — if empty, returns `null` (line 83–86).
  5. Looks up identifier in `phpGlobalFunctions.globalfunctions` or `phpGlobals.keywords` (line 89).
  6. Parses `entry.signature` to extract parameter substring up to last `)` (line 93).
  7. Regex `/\w*\s+\&?\$[\w_\.]+|void/g` extracts individual parameter labels from the signature string (lines 96–99).
  8. Builds `SignatureHelp` with one `SignatureInformation`, sets `activeSignature = 0` and `activeParameter = min(paramCount, params.length - 1)` (lines 101–105).
- **Data flow:** Cursor `Position` → `BackwardIterator` over `TextDocument` → raw char code stream → nesting-aware comma count + identifier extraction → static lookup in `globalfunctions` → `SignatureHelp` → VS Code renders parameter hint popup.
- **Dependencies:** `vscode`, `phpGlobals`, `phpGlobalFunctions`.

---

#### `extensions/php-language-features/src/features/utils/async.ts`

- **Role:** Provides three generic async scheduling primitives used throughout the extension. Fully independent of VS Code API.
- **Key symbols:**
  - `Throttler<T>` (lines 30–78): Ensures only one promise runs at a time; any incoming tasks while one is active are coalesced into a single "next" task. Uses `activePromise` and `queuedPromise` fields. The `queue(promiseFactory)` method (line 42) manages this state machine.
  - `Delayer<T>` (lines 103–163): Debounces task execution. Stores a `timeout` handle (line 106) reset on every `trigger()` call. The actual task runs only after `defaultDelay` ms of inactivity. Uses a stable `completionPromise` resolved by the timeout callback.
  - `ThrottledDelayer<T>` (lines 172–185): Composes `Delayer<Promise<T>>` and `Throttler<T>`. `trigger()` (line 182) calls `super.trigger(() => this.throttler.queue(promiseFactory), delay)`, so the task is first debounced then throttled.
- **Control flow:** `ThrottledDelayer` is the primary type used in `validationProvider.ts:97` (per-document delayers dict). Each document gets its own `ThrottledDelayer<void>` instance. `trigger(doValidate)` is called on every change/save event, which debounces then serialises validation runs.
- **Data flow:** Task factory functions (`ITask<Promise<T>>`) flow in via `trigger()`/`queue()` and are executed after delay/throttle conditions are met. Resolves/rejects propagate back through promise chains to callers.
- **Dependencies:** None (pure TypeScript, only Node.js `Timer` type referenced via `NodeJS.Timer`).

---

#### `extensions/php-language-features/src/features/utils/markedTextUtil.ts`

- **Role:** Single-function utility that escapes special Markdown characters in plain-text strings before embedding them in VS Code `MarkedString` (hover content).
- **Key symbols:**
  - `textToMarkedString(text: string): MarkedString` (line 8) — replaces `` \ ` * _ { } [ ] ( ) # + - . ! `` with backslash-escaped equivalents using the regex `/[\\`*_{}[\]()#+\-.!]/g`.
- **Control flow:** Pure string transformation, no branching.
- **Data flow:** Raw description string from `phpGlobals`/`phpGlobalFunctions` entry → escaped markdown-safe string → returned as `MarkedString` to `hoverProvider.ts:29`.
- **Dependencies:** `vscode` (`MarkedString` type import only).

---

#### `extensions/php-language-features/src/features/phpGlobals.ts`

- **Role:** Static data module. Exports typed dictionaries of PHP global variables, compile-time constants, and keywords, each as `IEntries` (map of name → `IEntry`). Generated from `PHP53Schema.xml` (comment at line 6).
- **Key symbols:**
  - `IEntry` interface (line 8): `{ description?: string; signature?: string }`.
  - `IEntries` interface (line 9): `{ [name: string]: IEntry }`.
  - `globalvariables: IEntries` (line 11) — 13 entries including `$GLOBALS`, `$_SERVER`, `$_GET`, `$_POST`, `$_FILES`, `$_SESSION`, `$this`, etc.
  - `compiletimeconstants: IEntries` (line 58) — includes `__CLASS__`, `__DIR__`, `__FILE__`, `__FUNCTION__`, `__LINE__`, `__METHOD__`, `__NAMESPACE__`, `TRUE`, `FALSE`, `NULL`, etc.
  - `keywords: IEntries` — PHP language keywords with descriptions.
- **Data flow:** Pure in-memory object literal exports. Consumed directly (no copying/transformation) by `hoverProvider.ts`, `completionItemProvider.ts`, and `signatureHelpProvider.ts` via ES module imports.
- **Dependencies:** None.

---

#### `extensions/php-language-features/src/features/phpGlobalFunctions.ts`

- **Role:** Large static data module (6039 lines). Exports `globalfunctions: IEntries` — a dictionary of all PHP built-in functions with `description` and `signature` strings. Auto-generated (header comment: "THIS IS GENERATED FILE. DO NOT MODIFY.").
- **Key symbols:**
  - `globalfunctions: IEntries` (line 10) — hundreds of entries. Each entry: `{ description: string, signature: string }`. Examples: `debug_backtrace`, `error_log`, `set_error_handler`, `opcache_compile_file`.
  - Signature format: PHP-style parameter list string, e.g. `'([ int $options = DEBUG_BACKTRACE_PROVIDE_OBJECT [, int $limit = 0 ]]): array'`.
- **Data flow:** Pure exports consumed by all three feature providers. `signatureHelpProvider.ts` additionally parses the signature strings with a regex to extract individual parameter labels.
- **Dependencies:** `IEntries` from `./phpGlobals`.

---

### Cross-Cutting Synthesis

The PHP language features extension exemplifies VS Code's "built-in extension" pattern for lightweight language support. The entire intelligence layer is stateless and static: hover, completion, and signature help all work by mapping a word at the cursor to pre-baked dictionary entries in `phpGlobals.ts` and `phpGlobalFunctions.ts`. No language server protocol (LSP) daemon is involved for these features. The only dynamic analysis is the regex scan of the open document for user-defined variables and functions in `completionItemProvider.ts`. Diagnostics are the sole feature requiring OS-level process execution: `validationProvider.ts` shells out to the system `php` binary, parses its stderr with a single regex (`MatchExpression`), and feeds results into VS Code's `DiagnosticCollection`. The `ThrottledDelayer` in `async.ts` decouples the event-driven document model from the relatively expensive process-spawn cost.

For a Tauri/Rust port, this partition reveals three concrete coupling surfaces with the VS Code/Electron runtime: (1) the `vscode` extension API itself (provider registration, `DiagnosticCollection`, workspace events, `WorkspaceConfiguration`) — all of which are consumed via TypeScript interfaces that would need Tauri-side equivalents or an IPC bridge; (2) Node.js `child_process.spawn` used in `validationProvider.ts:216` to run the PHP CLI — in Tauri this would map to Rust's `std::process::Command`; (3) the `which` npm package (`validationProvider.ts:8`, `getPhpPath():321`) used for PATH resolution — replaceable with a Rust crate. The static data modules (`phpGlobals.ts`, `phpGlobalFunctions.ts`) could be ported as Rust `HashMap<&str, PhpEntry>` constants or JSON embedded at compile time. The `BackwardIterator`, `Throttler`, `Delayer`, and `LineDecoder` logic is pure algorithmic code with no platform dependencies and translates directly to Rust.

---

### Out-of-Partition References

- `vscode` (the VS Code extension host API) — consumed throughout, defined in `node_modules/@types/vscode`. Core types used: `ExtensionContext`, `languages.registerCompletionItemProvider`, `languages.registerHoverProvider`, `languages.registerSignatureHelpProvider`, `languages.createDiagnosticCollection`, `workspace.onDidChangeConfiguration`, `workspace.onDidOpenTextDocument`, `workspace.onDidCloseTextDocument`, `workspace.onDidChangeTextDocument`, `workspace.onDidSaveTextDocument`, `workspace.isTrusted`, `workspace.getConfiguration`, `workspace.workspaceFolders`, `workspace.textDocuments`, `window.showInformationMessage`, `commands.executeCommand`, `l10n.t`, `Uri`, `Range`, `Position`, `Diagnostic`, `Hover`, `CompletionItem`, `CompletionItemKind`, `SignatureHelp`, `SignatureInformation`, `MarkedString`.
- `which` npm package (`node_modules/which`) — used in `validationProvider.ts:8` and called at line 321.
- Node.js built-ins: `child_process` (line 6 of `validationProvider.ts`), `string_decoder` (line 7), `path` (line 9).

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## External References
<!-- Source: codebase-online-researcher sub-agent -->
#### `which` (v2.0.2)
**Docs:** https://github.com/isaacs/node-which
**Relevant behaviour:** `which(command)` resolves a program name to its absolute path by searching `PATH`, returning a Promise that rejects if the binary is not found. In `extensions/php-language-features/`, it is called as `await which('php')` to auto-locate the PHP interpreter when the user has not set `php.validate.executablePath`. The function relies on Node.js `fs` and `path` APIs under the hood and has no platform-specific Tauri/Rust equivalent built in.
**Where used:** `extensions/php-language-features/src/features/validationProvider.ts:8` — `import which from 'which'`; called at line 321 inside `getPhpPath()`.

When porting `extensions/php-language-features/` to a Tauri/Rust host, the `which` npm package is the only external library dependency worth noting. Its role is narrow: a single async call to locate the `php` binary on `PATH`. In a Tauri context, Node.js is no longer available, so this lookup would need to be reimplemented — either via the `which` Rust crate (`which = "4"`) on the native side, exposed to the frontend through a Tauri command, or by delegating the search to a small Rust helper that walks `PATH` entries and checks for executable files. The VS Code extension API surface (`vscode.DiagnosticCollection`, `vscode.workspace.*`, `vscode.languages.*`) and Node.js `child_process` / `string_decoder` are far larger porting concerns, but those are VS Code API and Node built-ins rather than external library documentation; for the purposes of this partition, only `which` constitutes externally-documented library behaviour that a porter must understand and replace.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
