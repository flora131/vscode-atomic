# Pattern Finder 6: vscode.d.ts - Public Extension API Surface

## Research Question
What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

## Analysis Focus
Partition 6 examines `src/vscode-dts/` — the canonical TypeScript type definitions that define the public extension API. Any Rust/Tauri port must preserve these interfaces to maintain extension compatibility.

---

## Core Patterns Found

#### Pattern 1: Top-Level Namespaces (API Module Organization)
**Where:** `src/vscode-dts/vscode.d.ts:10973`, `11069`, `13797`, `14722`, `17283`, `16652`, `18091`
**What:** Named namespaces that group related functionality (commands, window, workspace, languages, debug, scm, authentication, etc.). These are the primary extension API entry points.

```typescript
export namespace commands {
	export function registerCommand(command: string, callback: (...args: any[]) => any, thisArg?: any): Disposable;
	export function registerTextEditorCommand(command: string, callback: (textEditor: TextEditor, edit: TextEditorEdit, ...args: any[]) => void, thisArg?: any): Disposable;
	export function executeCommand(command: string, ...rest: any[]): Thenable<any>;
}

export namespace window {
	export const tabGroups: TabGroups;
	export let activeTextEditor: TextEditor | undefined;
	export let visibleTextEditors: readonly TextEditor[];
	export const onDidChangeActiveTextEditor: Event<TextEditor | undefined>;
	export const onDidChangeVisibleTextEditors: Event<readonly TextEditor[]>;
	export const onDidChangeTextEditorSelection: Event<TextEditorSelectionChangeEvent>;
}

export namespace workspace {
	export const fs: FileSystem;
	export const workspaceFolders: readonly WorkspaceFolder[] | undefined;
	export const name: string | undefined;
	export function openTextDocument(uri: Uri): Thenable<TextDocument>;
}

export namespace languages {
	export function getLanguages(): Thenable<string[]>;
	export function setTextDocumentLanguage(document: TextDocument, languageId: string): Thenable<TextDocument>;
	export function registerCompletionItemProvider(selector: DocumentSelector, provider: CompletionItemProvider, ...triggerCharacters: string[]): Disposable;
	export function registerDefinitionProvider(selector: DocumentSelector, provider: DefinitionProvider): Disposable;
}

export namespace debug {
	export let activeDebugSession: DebugSession | undefined;
	export let activeDebugConsole: DebugConsole;
	export let breakpoints: readonly Breakpoint[];
	export const onDidChangeActiveDebugSession: Event<DebugSession | undefined>;
	export const onDidStartDebugSession: Event<DebugSession>;
}

export namespace scm {
	export const inputBox: SourceControlInputBox;
	export function createSourceControl(id: string, label: string, rootUri?: Uri): SourceControl;
}

export namespace authentication {
	export function getSession(providerId: string, scopeListOrRequest: ReadonlyArray<string> | AuthenticationWwwAuthenticateRequest, options: AuthenticationGetSessionOptions & { createIfNone: true }): Thenable<AuthenticationSession>;
	export function registerAuthenticationProvider(id: string, label: string, provider: AuthenticationProvider): Disposable;
}
```

**Variations / call-sites:** All major IDE features (tasks:9347, env:10739, l10n:18192, tests:18271, chat:20111, lm:20732) follow this namespace pattern.

---

#### Pattern 2: Event-Driven Architecture with Event<T> Interface
**Where:** `src/vscode-dts/vscode.d.ts:1755`
**What:** Defines the contract for observable state changes. Extensions subscribe to events using a callable interface pattern.

```typescript
export interface Event<T> {
	(listener: (e: T) => any, thisArgs?: any, disposables?: Disposable[]): Disposable;
}

export class EventEmitter<T> {
	event: Event<T>;
	fire(data: T): void;
	dispose(): void;
}
```

Used throughout all namespaces:
- `window.onDidChangeActiveTextEditor: Event<TextEditor | undefined>`
- `window.onDidChangeTextEditorSelection: Event<TextEditorSelectionChangeEvent>`
- `debug.onDidChangeActiveDebugSession: Event<DebugSession | undefined>`
- `debug.onDidStartDebugSession: Event<DebugSession>`
- `workspace.onDidChangeTextDocument: Event<TextDocumentChangeEvent>`

**Variations / call-sites:** 50+ event definitions across all namespaces; EventEmitter used for extension-provided events.

---

#### Pattern 3: Provider Interface Pattern (Language Intelligence)
**Where:** `src/vscode-dts/vscode.d.ts:2925`, `5189`, `3144`, `2745`
**What:** Interfaces that extensions implement to provide IDE capabilities (completion, definition, hover, code actions, etc.). Registration functions return Disposable for cleanup.

```typescript
// Definition Provider
export interface DefinitionProvider {
	provideDefinition(document: TextDocument, position: Position, token: CancellationToken): ProviderResult<Definition | DefinitionLink[]>;
}

// Completion Item Provider
export interface CompletionItemProvider<T extends CompletionItem = CompletionItem> {
	provideCompletionItems(document: TextDocument, position: Position, token: CancellationToken, context: CompletionContext): ProviderResult<T[] | CompletionList<T>>;
	resolveCompletionItem?(item: T, token: CancellationToken): ProviderResult<T>;
}

// Hover Provider
export interface HoverProvider {
	provideHover(document: TextDocument, position: Position, token: CancellationToken): ProviderResult<Hover>;
}

// Code Action Provider
export interface CodeActionProvider<T extends CodeAction = CodeAction> {
	provideCodeActions(document: TextDocument, range: Range, context: CodeActionContext, token: CancellationToken): ProviderResult<(Command | T)[]>;
	resolveCodeAction?(codeAction: T, token: CancellationToken): ProviderResult<T>;
}

// Registration pattern
export function registerCompletionItemProvider(selector: DocumentSelector, provider: CompletionItemProvider, ...triggerCharacters: string[]): Disposable;
export function registerDefinitionProvider(selector: DocumentSelector, provider: DefinitionProvider): Disposable;
export function registerHoverProvider(selector: DocumentSelector, provider: HoverProvider): Disposable;
export function registerCodeActionsProvider(selector: DocumentSelector, provider: CodeActionProvider, metadata?: CodeActionProviderMetadata): Disposable;
```

**Variations / call-sites:** 20+ provider types including ImplementationProvider, TypeDefinitionProvider, ReferenceProvider, RenameProvider, DocumentHighlightProvider, DocumentSymbolProvider, CodeLensProvider, SignatureHelpProvider, OnTypeFormattingEditProvider, DocumentFormattingEditProvider, etc. (lines 2925-5800).

---

#### Pattern 4: Core Data Types (Position, Range, Uri, TextDocument, TextEditor)
**Where:** `src/vscode-dts/vscode.d.ts:269`, `408`, `1439`, `88`, `1258`
**What:** Immutable value types and reference types that represent code coordinates and document state. These are the primitive building blocks for editor operations.

```typescript
export class Position {
	readonly line: number;
	readonly character: number;
	constructor(line: number, character: number);
	isBefore(other: Position): boolean;
	isBeforeOrEqual(other: Position): boolean;
	isAfter(other: Position): boolean;
	isAfterOrEqual(other: Position): boolean;
	isEqual(other: Position): boolean;
	compareTo(other: Position): number;
	translate(lineDelta?: number, characterDelta?: number): Position;
	with(line?: number, character?: number): Position;
}

export class Range {
	readonly start: Position;
	readonly end: Position;
	constructor(start: Position | number, end: Position | number);
	isEmpty: boolean;
	isSingleLine: boolean;
	contains(positionOrRange: Position | Range): boolean;
	isEqual(other: Range): boolean;
	intersection(range: Range): Range | undefined;
	union(other: Range): Range;
	with(start?: Position, end?: Position): Range;
}

export class Uri {
	static parse(value: string, strict?: boolean): Uri;
	static file(path: string): Uri;
	static joinPath(base: Uri, ...pathSegments: string[]): Uri;
	static from(components: { scheme?: string; authority?: string; path?: string; query?: string; fragment?: string }): Uri;
	readonly scheme: string;
	readonly authority: string;
	readonly path: string;
	readonly query: string;
	readonly fragment: string;
	readonly fsPath: string;
	with(change: { scheme?: string; authority?: string; path?: string; query?: string; fragment?: string }): Uri;
	toString(skipEncoding?: boolean): string;
}

export interface TextDocument {
	readonly uri: Uri;
	readonly fileName: string;
	readonly isUntitled: boolean;
	readonly languageId: string;
	readonly version: number;
	readonly isDirty: boolean;
	readonly isClosed: boolean;
	readonly lineCount: number;
	save(): Thenable<boolean>;
	lineAt(line: number): TextLine;
	lineAt(position: Position): TextLine;
	offsetAt(position: Position): number;
	positionAt(offset: number): Position;
	getText(range?: Range): string;
	getWordRangeAtPosition(position: Position, regex?: RegExp): Range | undefined;
	validateRange(range: Range): Range;
	validatePosition(position: Position): Position;
}

export interface TextEditor {
	readonly document: TextDocument;
	selection: Selection;
	selections: readonly Selection[];
	readonly visibleRanges: readonly Range[];
	options: TextEditorOptions;
	readonly viewColumn: ViewColumn | undefined;
	edit(callback: (editBuilder: TextEditorEdit) => void, options?: { undoStopBefore: boolean; undoStopAfter: boolean }): Thenable<boolean>;
	insertSnippet(snippet: SnippetString, location?: Position | Range | readonly Position[], options?: { undoStopBefore: boolean; undoStopAfter: boolean }): Thenable<boolean>;
	setDecorations(decorationType: TextEditorDecorationType, rangesOrOptions: Range[] | TextEditorDecoration[]): void;
	revealRange(range: Range, revealType?: TextEditorRevealType): void;
	show(column?: ViewColumn): void;
	hide(): void;
}
```

**Variations / call-sites:** Selection:448, TextLine:49, TextEditorEdit:1400, Range used in 200+ interface definitions.

---

#### Pattern 5: Terminal Integration (Shell Execution & Management)
**Where:** `src/vscode-dts/vscode.d.ts:7669`, `7828`, `7944`
**What:** Interfaces for terminal creation, execution, and shell integration. Extensions can create terminals, execute commands, and respond to shell events.

```typescript
export interface Terminal {
	readonly name: string;
	readonly processId: Thenable<number | undefined>;
	readonly creationOptions: Readonly<TerminalOptions | ExtensionTerminalOptions>;
	readonly exitStatus: TerminalExitStatus | undefined;
	readonly state: TerminalState;
	readonly shellIntegration: TerminalShellIntegration | undefined;
	sendText(text: string, addNewLine?: boolean): void;
	show(preserveFocus?: boolean): void;
	hide(): void;
	dispose(): void;
}

export interface TerminalShellIntegration {
	readonly nonce: string;
	readonly executeCommand(commandLine: string | TerminalShellExecutionCommandLine, eventEmitter?: EventEmitter<void>): Promise<number>;
	readonly onDidStartTerminalShellExecution: Event<TerminalShellExecutionStartEvent>;
	readonly onDidEndTerminalShellExecution: Event<TerminalShellExecutionEndEvent>;
}

export interface TerminalShellExecution {
	readonly commandLine: TerminalShellExecutionCommandLine;
	readonly exitCode: number | undefined;
	readonly exitReason: TerminalShellExecutionExitReason | undefined;
}

export namespace window {
	export const terminals: readonly Terminal[];
	export let activeTerminal: Terminal | undefined;
	export const onDidOpenTerminal: Event<Terminal>;
	export const onDidCloseTerminal: Event<Terminal>;
	export const onDidChangeActiveTerminal: Event<Terminal | undefined>;
	export const onDidChangeTerminalState: Event<Terminal>;
	export const onDidChangeTerminalDimensions: Event<TerminalDimensionsChangeEvent>;
	export const onDidChangeTerminalShellIntegration: Event<TerminalShellIntegrationChangeEvent>;
	export function createTerminal(name?: string, shellPath?: string, shellArgs?: string[] | string): Terminal;
	export function createTerminalFromOptions(options: TerminalOptions | ExtensionTerminalOptions): Terminal;
}
```

**Variations / call-sites:** TerminalOptions:12462, TerminalExitStatus:12785, TerminalDimensions:12770, TerminalProfileProvider:8220.

---

#### Pattern 6: Debugging Protocol Integration
**Where:** `src/vscode-dts/vscode.d.ts:17283`, `16673`
**What:** DAP (Debug Adapter Protocol) opaque types and session management. Extensions register debug providers and interact with active debug sessions.

```typescript
export namespace debug {
	export let activeDebugSession: DebugSession | undefined;
	export let activeDebugConsole: DebugConsole;
	export let breakpoints: readonly Breakpoint[];
	export const onDidChangeActiveDebugSession: Event<DebugSession | undefined>;
	export const onDidStartDebugSession: Event<DebugSession>;
	export const onDidReceiveDebugSessionCustomEvent: Event<DebugSessionCustomEvent>;
	export const onDidTerminateDebugSession: Event<DebugSession>;
	export const onDidChangeBreakpoints: Event<BreakpointsChangeEvent>;
	export function registerDebugAdapterDescriptorFactory(debugType: string, factory: DebugAdapterDescriptorFactory): Disposable;
	export function registerDebugAdapterTrackerFactory(debugType: string, factory: DebugAdapterTrackerFactory): Disposable;
}

export interface DebugProtocolMessage {
	// Properties: see [ProtocolMessage details](https://microsoft.github.io/debug-adapter-protocol/specification#Base_Protocol_ProtocolMessage).
}

export interface DebugProtocolSource {
	// Properties: see [Source details](https://microsoft.github.io/debug-adapter-protocol/specification#Types_Source).
}

export interface DebugProtocolBreakpoint {
	// Properties: see [Breakpoint details](https://microsoft.github.io/debug-adapter-protocol/specification#Types_Breakpoint).
}

export interface DebugConfiguration {
	type: string;
	name: string;
	request: string;
	[key: string]: any;
}
```

**Variations / call-sites:** DebugSession, DebugConsole, Breakpoint, BreakpointsChangeEvent, DebugAdapterDescriptorFactory, DebugAdapterTrackerFactory.

---

#### Pattern 7: Source Control (SCM) Provider Pattern
**Where:** `src/vscode-dts/vscode.d.ts:16652`
**What:** Abstraction for source control systems (Git, etc.). Extensions create SourceControl instances with InputBox, ResourceGroups, and Commands.

```typescript
export namespace scm {
	export const inputBox: SourceControlInputBox;
	export function createSourceControl(id: string, label: string, rootUri?: Uri): SourceControl;
}

export interface SourceControl {
	readonly id: string;
	readonly label: string;
	readonly rootUri: Uri;
	inputBox: SourceControlInputBox;
	readonly count?: number;
	readonly statusBarCommands?: Command[];
	readonly quickDiffProvider?: QuickDiffProvider;
	createResourceGroup(id: string, label: string): SourceControlResourceGroup;
	createResourceGroup(id: string, label: string, hideWhenEmpty: boolean): SourceControlResourceGroup;
	dispose(): void;
}

export interface SourceControlResourceGroup {
	readonly id: string;
	label: string;
	hideWhenEmpty?: boolean;
	resourceStates: SourceControlResourceState[];
	dispose(): void;
}
```

**Variations / call-sites:** SourceControlInputBox, SourceControlResourceState, QuickDiffProvider.

---

#### Pattern 8: Task Provider & Execution
**Where:** `src/vscode-dts/vscode.d.ts:9347`
**What:** Extension-provided task definitions and execution model. Extensions register providers that return Task objects; execution returns TaskExecution for monitoring.

```typescript
export namespace tasks {
	export function registerTaskProvider(type: string, provider: TaskProvider): Disposable;
	export function fetchTasks(filter?: TaskFilter): Thenable<Task[]>;
	export function executeTask(task: Task): Thenable<TaskExecution>;
	export const taskExecutions: readonly TaskExecution[];
	export const onDidStartTask: Event<TaskStartEvent>;
	export const onDidEndTask: Event<TaskEndEvent>;
	export const onDidStartTaskProcess: Event<TaskProcessStartEvent>;
	export const onDidEndTaskProcess: Event<TaskProcessEndEvent>;
}

export interface TaskProvider {
	provideTasks(token: CancellationToken): ProviderResult<Task[]>;
	resolveTask(task: Task, token: CancellationToken): ProviderResult<Task>;
}

export interface Task {
	definition: TaskDefinition;
	name: string;
	detail?: string;
	execution?: TaskExecution | ProcessExecution | ShellExecution | CustomExecution;
	isBackground?: boolean;
	source: string;
	group?: TaskGroup;
	presentationOptions?: TaskPresentationOptions;
	problemMatchers: string[];
	runOptions?: RunOptions;
}
```

**Variations / call-sites:** TaskExecution, ProcessExecution, ShellExecution, CustomExecution, TaskGroup, TaskDefinition, TaskFilter.

---

#### Pattern 9: Authentication Provider Pattern
**Where:** `src/vscode-dts/vscode.d.ts:18091`
**What:** Extensible authentication system. Extensions implement AuthenticationProvider interface; built-in and third-party providers are queried via namespace.

```typescript
export namespace authentication {
	export function getSession(providerId: string, scopeListOrRequest: ReadonlyArray<string> | AuthenticationWwwAuthenticateRequest, options: AuthenticationGetSessionOptions & { createIfNone: true | AuthenticationGetSessionPresentationOptions }): Thenable<AuthenticationSession>;
	export function registerAuthenticationProvider(id: string, label: string, provider: AuthenticationProvider, options?: AuthenticationProviderOptions): Disposable;
	export const onDidChangeSessions: Event<AuthenticationSessionsChangeEvent>;
}

export interface AuthenticationProvider {
	readonly onDidChangeSessions: Event<AuthenticationSessionsChangeEvent>;
	getSessions(scopes?: string[]): Thenable<readonly AuthenticationSession[]>;
	getSession(scopes: string[], options: AuthenticationGetSessionOptions): Thenable<AuthenticationSession | undefined>;
	createSession(scopes: string[], options: AuthenticationCreateSessionOptions): Thenable<AuthenticationSession>;
	removeSession(sessionId: string): Thenable<void>;
}

export interface AuthenticationSession {
	readonly id: string;
	readonly accessToken: string;
	readonly refreshToken?: string;
	readonly idToken?: string;
	readonly account: AuthenticationSessionAccountInformation;
	readonly scopes: readonly string[];
}
```

**Variations / call-sites:** AuthenticationGetSessionOptions, AuthenticationWwwAuthenticateRequest, AuthenticationSessionsChangeEvent.

---

#### Pattern 10: Disposable Resource Management
**Where:** `src/vscode-dts/vscode.d.ts:1712`
**What:** Universal cleanup pattern. All registration functions return Disposable; extensions manage subscriptions and provider lifetimes via dispose().

```typescript
export class Disposable {
	static from(...disposableLikes: { dispose: () => any }[]): Disposable;
	constructor(callOnDispose: () => any);
	dispose(): any;
}
```

Used in:
- `registerCommand(...): Disposable`
- `registerTextEditorCommand(...): Disposable`
- `registerCompletionItemProvider(...): Disposable`
- `registerDefinitionProvider(...): Disposable`
- `registerHoverProvider(...): Disposable`
- `createSourceControl(...): SourceControl` (has `dispose()` method)
- Event subscriptions: `event(listener): Disposable`

**Variations / call-sites:** 100+ registration and subscription functions return Disposable for cleanup.

---

## Summary

The vscode.d.ts API surface demonstrates 10 core architectural patterns that any Rust/Tauri port must replicate:

1. **Namespace organization** for IDE features (10+ namespaces covering commands, window, workspace, languages, debug, scm, authentication, tasks, tests, chat, LM)
2. **Event-driven reactivity** with typed Event<T> interfaces and EventEmitter
3. **Provider pattern** for language intelligence (20+ provider types for LSP-like capabilities)
4. **Core immutable value types** (Position, Range, Uri, Selection) for coordinate and location handling
5. **Terminal management** with shell integration and execution tracking
6. **Debug protocol abstraction** (DAP opaque types) for debugger integration
7. **SCM abstraction** for version control providers
8. **Task provider pattern** for build/run task registration and execution
9. **Pluggable authentication** for multi-provider credential management
10. **Universal Disposable pattern** for resource lifecycle management

These patterns define the host-to-extension boundary that must be preserved. A Tauri/Rust port would need to:
- Serialize/deserialize these types across process boundaries (host in Rust, extensions in Wasm or RPC)
- Implement async/event bridging for the Event<T> callback pattern
- Map provider registrations to the new host architecture
- Maintain binary/protocol compatibility with Position, Range, Uri serialization
- Implement terminal, debug, and SCM subsystems in Rust while exposing identical APIs

The TypeScript definitions themselves (in src/vscode-dts/) would serve as the canonical API contract that both old (Electron) and new (Tauri) runtimes must honor.
