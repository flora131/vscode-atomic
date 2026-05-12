# Porting VS Code to Tauri/Rust: Core Architectural Patterns

## Overview
This document captures concrete code patterns from VS Code's `src/vs/` partition (1.99M LOC) demonstrating how core IDE functionality (editing, language intelligence, debugging, terminal, navigation) is implemented in TypeScript/Electron. These patterns identify dependencies critical for any Tauri/Rust port.

---

## Pattern 1: Dependency Injection via Service Decorators

**Where:** `src/vs/platform/instantiation/common/instantiation.ts:109-120`

**What:** Service registration via `createDecorator<T>()` creates typed identifiers for DI container.

```typescript
export function createDecorator<T>(serviceId: string): ServiceIdentifier<T> {
	if (_util.serviceIds.has(serviceId)) {
		return _util.serviceIds.get(serviceId)!;
	}

	const id = function (target: Function, key: string, index: number) {
		if (arguments.length !== 3) {
			throw new Error('@IServiceName-decorator can only be used to decorate a parameter');
		}
		storeServiceDependency(id, target, index);
	} as ServiceIdentifier<T>;

	id.type = undefined as unknown as T;
	_util.serviceIds.set(serviceId, id);
	return id;
}
```

**Call-sites:** ICodeEditorService, ISCMService, IFileService, ICommandService (20+ core services)

---

## Pattern 2: IPC Protocol for Multi-Process Architecture

**Where:** `src/vs/base/parts/ipc/common/ipc.ts:25-38`

**What:** Type-safe RPC abstraction through IChannel and IServerChannel interfaces.

```typescript
export interface IChannel {
	call<T>(command: string, arg?: any, cancellationToken?: CancellationToken): Promise<T>;
	listen<T>(event: string, arg?: any): Event<T>;
}

export interface IServerChannel<TContext = string> {
	call<T>(ctx: TContext, command: string, arg?: any, cancellationToken?: CancellationToken): Promise<T>;
	listen<T>(ctx: TContext, event: string, arg?: any): Event<T>;
}
```

**Call-sites:** Electron IPC, child-process IPC, message-port IPC, agent host server setup

---

## Pattern 3: Electron Main Process Integration

**Where:** `src/vs/code/electron-main/app.ts:5-100`

**What:** Electron entry point bootstraps 40+ platform services including window management, IPC, storage, updates.

```typescript
import { app, protocol, session, powerMonitor } from 'electron';

const productService: IProductService = { _serviceBrand: undefined, ...product };
const environmentService = new NativeEnvironmentService(parseArgs(process.argv, OPTIONS), productService);
```

**Call-sites:** Window lifecycle, platform updates, system integration

---

## Pattern 4: Web Worker for Parallelization

**Where:** `src/vs/platform/webWorker/browser/webWorkerServiceImpl.ts:17-53`

**What:** ESM web worker spawning for CPU-intensive tasks with lifecycle management.

```typescript
export class WebWorkerService implements IWebWorkerService {
	createWorkerClient<T extends object>(workerDescriptor: WebWorkerDescriptor | Worker): IWebWorkerClient<T> {
		const id = ++WebWorkerService._workerIdPool;
		const worker = this._createWorker(workerDescriptor);
		return new WebWorkerClient<T>(new WebWorker(worker, id));
	}

	protected _createWorker(descriptor: WebWorkerDescriptor): Promise<Worker> {
		const workerUrl = this.getWorkerUrl(descriptor);
		const worker = new Worker(workerUrl, { name: descriptor.label, type: 'module' });
		return whenESMWorkerReady(worker);
	}
}
```

---

## Pattern 5: File System Provider Registry

**Where:** `src/vs/platform/files/common/files.ts:26-89`

**What:** Pluggable FS providers by scheme abstraction for disk, remote, git, custom schemes.

```typescript
export const IFileService = createDecorator<IFileService>('fileService');

export interface IFileService {
	readonly _serviceBrand: undefined;
	registerProvider(scheme: string, provider: IFileSystemProvider): IDisposable;
	getProvider(scheme: string): IFileSystemProvider | undefined;
	hasCapability(resource: URI, capability: FileSystemProviderCapabilities): boolean;
	
	readonly onDidFilesChange: Event<FileChangesEvent>;
	stat(resource: URI): Promise<IFileStatWithMetadata>;
	readFile(resource: URI): Promise<Uint8Array>;
	writeFile(resource: URI, buffer: Uint8Array): Promise<IFileStatWithMetadata>;
}
```

---

## Pattern 6: Menu and Action Registry System

**Where:** `src/vs/platform/actions/common/actions.ts:65-100`

**What:** Pre-defined MenuId locations with context-driven conditional rendering.

```typescript
export class MenuId {
	static readonly CommandPalette = new MenuId('CommandPalette');
	static readonly EditorContext = new MenuId('EditorContext');
	static readonly EditorTitle = new MenuId('EditorTitle');
	// ... 60+ menu locations
}

export interface IMenuItem {
	command: ICommandAction;
	when?: ContextKeyExpression;
	group?: 'navigation' | string;
}
```

---

## Pattern 7: Language Features Service

**Where:** `src/vs/editor/browser/services/codeEditorService.ts:15-62`

**What:** Editor capability registration (completion, hover, go-to-def) through language feature providers.

```typescript
export const ICodeEditorService = createDecorator<ICodeEditorService>('codeEditorService');

export interface ICodeEditorService {
	readonly onCodeEditorAdd: Event<ICodeEditor>;
	readonly onCodeEditorRemove: Event<ICodeEditor>;
	listCodeEditors(): readonly ICodeEditor[];
	getFocusedCodeEditor(): ICodeEditor | null;
	openCodeEditor(input: ITextResourceEditorInput, source: ICodeEditor | null, sideBySide?: boolean): Promise<ICodeEditor | null>;
}
```

---

## Pattern 8: Source Control (SCM) Service

**Where:** `src/vs/workbench/contrib/scm/common/scm.ts:35-97`

**What:** Provider-based SCM integration for Git, SVN, etc. with resource groups and change tracking.

```typescript
export const ISCMService = createDecorator<ISCMService>('scm');

export interface ISCMProvider extends IDisposable {
	readonly id: string;
	readonly groups: readonly ISCMResourceGroup[];
	readonly onDidChangeResourceGroups: Event<void>;
	readonly rootUri?: URI;
	readonly inputBoxTextModel: ITextModel;
	
	getOriginalResource(uri: URI): Promise<URI | null>;
}
```

---

## Pattern 9: Terminal Instance Management

**Where:** `src/vs/workbench/contrib/terminal/browser/terminalInstance.ts:60-80`

**What:** Terminal instances coordinate PTY process, xterm rendering, shell integration, capability tracking.

```typescript
export class TerminalInstance extends Disposable implements ITerminalInstance {
	private _processManager: ITerminalProcessManager;
	private _xtermTerminal: XtermTerminal;
	private _shellIntegrationAddon: ShellIntegrationAddon;
	
	readonly onData: Event<string>;
	readonly onExit: Event<number | undefined>;
}
```

---

## Pattern 10: Debug Service and Session Management

**Where:** `src/vs/workbench/contrib/debug/browser/repl.ts:70-72`

**What:** DAP-compliant adapter management with breakpoint, variable, REPL tracking.

```typescript
export interface IDebugSession {
	readonly id: string;
	readonly type: string;
	readonly state: State;
	
	continue(): Promise<void>;
	stepIn(): Promise<void>;
	evaluate(expression: string, frameId?: number): Promise<DebugProtocol.EvaluateResponse>;
	customRequest(request: string, args?: any): Promise<DebugProtocol.Response>;
}
```

---

## Pattern 11: Command Service and Registry

**Where:** `src/vs/platform/commands/common/commands.ts:15-65`

**What:** Centralized command registration and execution with string-based IDs.

```typescript
export const ICommandService = createDecorator<ICommandService>('commandService');

export interface ICommandService {
	readonly onWillExecuteCommand: Event<ICommandEvent>;
	readonly onDidExecuteCommand: Event<ICommandEvent>;
	executeCommand<R = unknown>(commandId: string, ...args: unknown[]): Promise<R | undefined>;
}

export const CommandsRegistry: ICommandRegistry = new class {
	private readonly _commands = new Map<string, LinkedList<ICommand>>();
	registerCommand(idOrCommand: string | ICommand, handler?: ICommandHandler): IDisposable;
}
```

---

## Pattern 12: Native Host Service for OS Integration

**Where:** `src/vs/platform/native/electron-main/nativeHostMainService.ts`

**What:** OS-level APIs (file dialogs, clipboard, process spawn, system info) exposed via IPC.

**Capabilities:**
```typescript
openFileDialog(options: IOpenFileDialogOptions): Promise<string[] | undefined>;
openFolderDialog(options: IOpenFolderDialogOptions): Promise<string[] | undefined>;
clipboard: { readText(): Promise<string>; writeText(text: string): Promise<void> };
exec(command: string): Promise<{ stdout: string; stderr: string }>;
spawn(command: string, args?: string[]): Promise<number>;
getEnv(): IProcessEnvironment;
getHostname(): string;
popupContextMenu(menu: IPopupContextMenuOptions): Promise<number>;
```

---

## Cross-Cutting Architectural Patterns

### Cancellation Token Flow
- Flows through all async RPC calls
- Serializable across IPC boundaries
- Enables timeout and user cancellation

### Event and Observable Pattern
- Event<T> used for all observable changes
- Remote event subscriptions require IPC handling
- Bidirectional update propagation

### URI-Based Resource Abstraction
- All file operations use typed URIs with schemes
- Scheme routing determines filesystem provider
- Deep integration in editor and workbench

### Multi-Window Support
- `getWindow()`, `getDocument()` APIs for window-scoped operations
- Isolated DOM contexts, shared service layer
- Split windows and popup editors

---

## Porting Requirements Summary

Essential for Tauri/Rust port:

1. **DI System** (20+ services): Rust trait objects + registry
2. **IPC/RPC**: serde/bincode serialization with async semantics
3. **Process Model**: Electron (main/renderer/ext/pty) → Tauri backend + frontend + workers
4. **FS Provider Registry**: Async file I/O with scheme routing
5. **Terminal/PTY**: Rust ptyprocess crate
6. **Debug Adapter**: DAP subprocess management
7. **Menu/Command**: Context-key expression evaluation
8. **Native APIs**: Tauri Rust backend (dialogs, clipboard, file ops)
9. **Web Workers**: Thread spawning for browser workers
10. **Event System**: Cross-process event subscription
11. **Language Services**: LSP client/server (transport changes)
12. **DOM Rendering**: Browser UI layer reusable (Monaco, React)

Browser UI (60% of code) is reusable; platform services layer (40%, `src/vs/platform/` and `src/vs/base/`) requires Rust reimplementation with equivalent semantics.
