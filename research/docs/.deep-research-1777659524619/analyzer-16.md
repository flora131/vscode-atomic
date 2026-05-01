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
