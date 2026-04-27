# VS Code Core IDE Architecture Patterns
## Partition 1: DI, Lifecycle, IPC, Process Integration

Research into concrete patterns for porting VS Code core (editing, language intelligence, debugging, source control, terminal, navigation) from TypeScript/Electron to Tauri/Rust.

---

## Pattern 1: Service Decoration & Dependency Injection

#### Pattern: createDecorator<T>(NAME) DI Service Interfaces

**Where:** `src/vs/platform/instantiation/common/instantiation.ts:109-126`

**What:** Creates a typed service identifier that functions as both a parameter decorator and service registry key, enabling compile-time safe dependency injection across the codebase.

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

	id.toString = () => serviceId;

	_util.serviceIds.set(serviceId, id);
	return id;
}
```

**Variations / call-sites:**
- `src/vs/platform/secrets/common/secrets.ts:86` - `ISecretStorageService = createDecorator<ISecretStorageService>('secretStorageService')`
- `src/vs/workbench/services/search/common/search.ts:40` - `ISearchService = createDecorator<ISearchService>('searchService')`
- `src/vs/workbench/contrib/terminal/browser/terminal.ts:39-45` - Multiple terminal services using this pattern

#### Pattern: Service Implementation with Constructor Injection

**Where:** `src/vs/platform/secrets/common/secrets.ts:101-120`

**What:** Service classes extend Disposable and declare injected dependencies as decorated constructor parameters, enabling automatic service resolution and lifecycle management.

```typescript
export class BaseSecretStorageService extends Disposable implements ISecretStorageService {
	declare readonly _serviceBrand: undefined;

	protected readonly onDidChangeSecretEmitter = this._register(new Emitter<string>());
	readonly onDidChangeSecret: Event<string> = this.onDidChangeSecretEmitter.event;

	protected readonly _sequencer = new SequencerByKey<string>();

	constructor(
		private readonly _useInMemoryStorage: boolean,
		@IStorageService private _storageService: IStorageService,
		@IEncryptionService protected _encryptionService: IEncryptionService,
		@ILogService protected readonly _logService: ILogService,
	) {
		super();
	}
	// ...
}
```

**Variations / call-sites:**
- `src/vs/platform/native/electron-main/nativeHostMainService.ts:58-76` - Platform service with 10+ injected dependencies
- `src/vs/editor/browser/services/editorWorkerService.ts:58-79` - Editor worker service pattern
- `src/vs/code/electron-main/app.ts` - Main application bootstrap (100+ services)

---

## Pattern 2: Resource Lifecycle Management

#### Pattern: class extends Disposable with _register() for subresources

**Where:** `src/vs/base/common/lifecycle.ts:526-557`

**What:** Base class that provides tracked resource disposal through a DisposableStore, enabling automatic cleanup of event listeners, timers, and child resources via _register() method.

```typescript
export abstract class Disposable implements IDisposable {
	static readonly None = Object.freeze<IDisposable>({ dispose() { } });

	protected readonly _store = new DisposableStore();

	constructor() {
		trackDisposable(this);
		setParentOfDisposable(this._store, this);
	}

	public dispose(): void {
		markAsDisposed(this);
		this._store.dispose();
	}

	protected _register<T extends IDisposable>(o: T): T {
		if ((o as unknown as Disposable) === this) {
			throw new Error('Cannot register a disposable on itself!');
		}
		return this._store.add(o);
	}
}
```

**Variations / call-sites:**
- `src/vs/editor/browser/services/editorWorkerService.ts:58` - EditorWorkerService extends Disposable
- `src/vs/platform/native/electron-main/nativeHostMainService.ts:58` - NativeHostMainService extends Disposable
- `src/vs/platform/secrets/common/secrets.ts:101` - BaseSecretStorageService extends Disposable

#### Pattern: DisposableStore for managing collections

**Where:** `src/vs/base/common/lifecycle.ts:416-485`

**What:** Container for managing multiple IDisposable resources with tracking of disposed state and automatic cleanup on children when parent is disposed.

```typescript
export class DisposableStore implements IDisposable {
	static DISABLE_DISPOSED_WARNING = false;

	private readonly _toDispose = new Set<IDisposable>();
	private _isDisposed = false;

	public dispose(): void {
		if (this._isDisposed) {
			return;
		}
		markAsDisposed(this);
		this._isDisposed = true;
		this.clear();
	}

	public get isDisposed(): boolean {
		return this._isDisposed;
	}

	public add<T extends IDisposable>(o: T): T {
		if (!o || o === Disposable.None) {
			return o;
		}
		setParentOfDisposable(o, this);
		if (this._isDisposed) {
			if (!DisposableStore.DISABLE_DISPOSED_WARNING) {
				console.warn(new Error('Trying to add a disposable to a DisposableStore that has already been disposed of...').stack);
			}
		} else {
			this._toDispose.add(o);
		}
		return o;
	}

	public delete<T extends IDisposable>(o: T): void {
		this._toDispose.delete(o);
		o.dispose();
	}
}
```

**Variations / call-sites:**
- Used internally by all Disposable subclasses
- Used for event listener management across all major subsystems
- Used for async operation tracking (timers, promises)

---

## Pattern 3: Electron IPC Integration

#### Pattern: ipcRenderer message passing from renderer process

**Where:** `src/vs/workbench/services/lifecycle/electron-browser/lifecycleService.ts:33-77`

**What:** Renderer process listens to IPC messages from main process, handling application lifecycle events (shutdown, unload) with event reply channels.

```typescript
private registerListeners(): void {
	const windowId = this.nativeHostService.windowId;

	// Main side indicates that window is about to unload, check for vetos
	ipcRenderer.on('vscode:onBeforeUnload', async (event: unknown, ...args: unknown[]) => {
		const reply = args[0] as { okChannel: string; cancelChannel: string; reason: ShutdownReason };
		this.logService.trace(`[lifecycle] onBeforeUnload (reason: ${reply.reason})`);

		// trigger onBeforeShutdown events and veto collecting
		const veto = await this.handleBeforeShutdown(reply.reason);

		// veto: cancel unload
		if (veto) {
			this.logService.trace('[lifecycle] onBeforeUnload prevented via veto');
			this._onShutdownVeto.fire();
			ipcRenderer.send(reply.cancelChannel, windowId);
		}
		// no veto: allow unload
		else {
			this.logService.trace('[lifecycle] onBeforeUnload continues without veto');
			this.shutdownReason = reply.reason;
			ipcRenderer.send(reply.okChannel, windowId);
		}
	});

	// Main side indicates that we will indeed shutdown
	ipcRenderer.on('vscode:onWillUnload', async (event: unknown, ...args: unknown[]) => {
		const reply = args[0] as { replyChannel: string; reason: ShutdownReason };
		this.logService.trace(`[lifecycle] onWillUnload (reason: ${reply.reason})`);

		await this.handleWillShutdown(reply.reason);
		this._onDidShutdown.fire();
		ipcRenderer.send(reply.replyChannel, windowId);
	});
}
```

**Variations / call-sites:**
- `src/vs/base/parts/ipc/electron-browser/ipc.mp.ts:17-36` - MessagePort channel acquisition from main process
- `src/vs/base/parts/sandbox/electron-browser/globals.ts` - Safe ipcRenderer proxy through preload

#### Pattern: ValidatedIpcMain handler registration in main process

**Where:** `src/vs/base/parts/ipc/electron-main/ipcMain.ts:13-81`

**What:** Main process wraps Electron ipcMain with validation, supporting .on(), .once(), and .handle() patterns for both send/on and invoke/await patterns.

```typescript
class ValidatedIpcMain implements Event.NodeEventEmitter {
	private readonly mapListenerToWrapper = new WeakMap<ipcMainListener, ipcMainListener>();

	on(channel: string, listener: ipcMainListener): this {
		const wrappedListener = (event: electron.IpcMainEvent, ...args: unknown[]) => {
			if (this.validateEvent(channel, event)) {
				listener(event, ...args);
			}
		};

		this.mapListenerToWrapper.set(listener, wrappedListener);
		electron.ipcMain.on(channel, wrappedListener);
		return this;
	}

	once(channel: string, listener: ipcMainListener): this {
		electron.ipcMain.once(channel, (event: electron.IpcMainEvent, ...args: unknown[]) => {
			if (this.validateEvent(channel, event)) {
				listener(event, ...args);
			}
		});
		return this;
	}

	handle(channel: string, listener: (event: electron.IpcMainInvokeEvent, ...args: any[]) => Promise<unknown>): this {
		electron.ipcMain.handle(channel, (event: electron.IpcMainInvokeEvent, ...args: unknown[]) => {
			if (this.validateEvent(channel, event)) {
				return listener(event, ...args);
			}
			return Promise.reject(`Invalid channel '${channel}' or sender for ipcMain.handle() usage.`);
		});
		return this;
	}

	removeHandler(channel: string): this {
		electron.ipcMain.removeHandler(channel);
		return this;
	}

	removeListener(channel: string, listener: ipcMainListener): this {
		const wrappedListener = this.mapListenerToWrapper.get(listener);
		if (wrappedListener) {
			electron.ipcMain.removeListener(channel, wrappedListener);
			this.mapListenerToWrapper.delete(listener);
		}
		return this;
	}
}
```

**Variations / call-sites:**
- Used by all main→renderer communication in lifecycle, storage, window management
- Referenced in `src/vs/code/electron-main/app.ts` as `validatedIpcMain`

---

## Pattern 4: Node.js Process Integration

#### Pattern: ChildProcess spawning for CLI & subprocess management

**Where:** `src/vs/code/node/cli.ts:44-88`

**What:** CLI launcher spawns child processes with environment variable injection, stdio piping, and exit/error handling for tunnel and server subcommands.

```typescript
export async function main(argv: string[]): Promise<void> {
	let args: NativeParsedArgs;

	try {
		args = parseCLIProcessArgv(argv);
	} catch (err) {
		console.error(err.message);
		return;
	}

	for (const subcommand of NATIVE_CLI_COMMANDS) {
		if (args[subcommand]) {
			if (!product.tunnelApplicationName) {
				console.error(`'${subcommand}' command not supported in ${product.applicationName}`);
				return;
			}
			const env: IProcessEnvironment = {
				...process.env
			};
			// bootstrap-esm.js determines the electron environment based
			// on the following variable. For the server we need to unset
			// it to prevent importing any electron specific modules.
			delete env['ELECTRON_RUN_AS_NODE'];

			const tunnelArgs = argv.slice(argv.indexOf(subcommand) + 1);
			return new Promise((resolve, reject) => {
				let tunnelProcess: ChildProcess;
				const stdio: StdioOptions = ['ignore', 'pipe', 'pipe'];
				if (process.env['VSCODE_DEV']) {
					tunnelProcess = spawn('cargo', ['run', '--', subcommand, ...tunnelArgs], { cwd: join(getAppRoot(), 'cli'), stdio, env });
				} else {
					const appPath = process.platform === 'darwin'
						? join(dirname(dirname(process.execPath)), 'Resources', 'app')
						: dirname(process.execPath);
					const tunnelCommand = join(appPath, 'bin', `${product.tunnelApplicationName}${isWindows ? '.exe' : ''}`);
					tunnelProcess = spawn(tunnelCommand, [subcommand, ...tunnelArgs], { cwd: cwd(), stdio, env });
				}

				tunnelProcess.stdout!.pipe(process.stdout);
				tunnelProcess.stderr!.pipe(process.stderr);
				tunnelProcess.on('exit', resolve);
				tunnelProcess.on('error', reject);
			});
		}
	}
	// ...
}
```

**Variations / call-sites:**
- `src/vs/code/electron-main/app.ts:1-120` - Electron app initialization with process event handling
- `src/vs/platform/native/electron-main/nativeHostMainService.ts:1-50` - Native services using child_process exec/spawn
- Process environment accessed via `process.env`, `process.execPath`, `process.platform`, `process.pid`

#### Pattern: Platform-aware process environment handling

**Where:** `src/vs/code/node/cli.ts:60-81`

**What:** Process environment is cloned and modified before spawning subprocesses, with platform-specific executable paths derived from process.execPath and process.platform.

```typescript
const env: IProcessEnvironment = {
	...process.env
};
delete env['ELECTRON_RUN_AS_NODE'];

const tunnelArgs = argv.slice(argv.indexOf(subcommand) + 1);
return new Promise((resolve, reject) => {
	let tunnelProcess: ChildProcess;
	const stdio: StdioOptions = ['ignore', 'pipe', 'pipe'];
	if (process.env['VSCODE_DEV']) {
		tunnelProcess = spawn('cargo', ['run', '--', subcommand, ...tunnelArgs], { cwd: join(getAppRoot(), 'cli'), stdio, env });
	} else {
		const appPath = process.platform === 'darwin'
			? join(dirname(dirname(process.execPath)), 'Resources', 'app')
			: dirname(process.execPath);
		const tunnelCommand = join(appPath, 'bin', `${product.tunnelApplicationName}${isWindows ? '.exe' : ''}`);
		tunnelProcess = spawn(tunnelCommand, [subcommand, ...tunnelArgs], { cwd: cwd(), stdio, env });
	}

	tunnelProcess.stdout!.pipe(process.stdout);
	tunnelProcess.stderr!.pipe(process.stderr);
	tunnelProcess.on('exit', resolve);
	tunnelProcess.on('error', reject);
});
```

**Variations / call-sites:**
- File path resolution based on `process.platform` (darwin/win32/linux)
- Executable discovery using `process.execPath` as anchor point
- Working directory via `cwd()` utility function wrapping `process.cwd()`

---

## Pattern 5: Service Bootstrap & Configuration

#### Pattern: Main service collection assembly in electron-main

**Where:** `src/vs/code/electron-main/app.ts:1-120` (excerpt)

**What:** Central bootstrap location that instantiates and registers 100+ core services via ServiceCollection, including file system, storage, window management, IPC, lifecycle, and platform services.

```typescript
import { app, Details, GPUFeatureStatus, powerMonitor, protocol, session, Session, systemPreferences, WebFrameMain } from 'electron';
import { validatedIpcMain } from '../../base/parts/ipc/electron-main/ipcMain.js';
import { execFile } from 'child_process';
import { hostname, release } from 'os';
// ... 100+ more imports ...
import { IInstantiationService, ServicesAccessor } from '../../platform/instantiation/common/instantiation.js';
import { ServiceCollection } from '../../platform/instantiation/common/serviceCollection.js';
import { IBackupMainService } from '../../platform/backup/electron-main/backup.js';
import { BackupMainService } from '../../platform/backup/electron-main/backupMainService.js';
// ... service registrations follow ServiceCollection pattern ...
```

**Key service categories registered:**
- File system (DiskFileSystemProvider)
- Storage (ApplicationStorageMainService, StorageMainService)
- Window management (WindowsMainService)
- IPC coordination (multiple channel types)
- Lifecycle (LifecycleMainService)
- Telemetry (TelemetryService)
- Update (platform-specific UpdateService)
- Native integration (NativeHostMainService)

**Variations / call-sites:**
- `src/vs/workbench/test/electron-browser/workbenchTestServices.ts` - Test service collection setup
- `src/vs/sessions/sessions.desktop.main.ts` - Desktop/main process bootstrapping

---

## Key Architectural Insights for Tauri/Rust Port

### 1. **Dependency Injection as Core Pattern**
Every major subsystem (editor, terminal, search, source control, debugging) uses the `createDecorator<T>()` pattern for compile-time safe service registration. This would need Rust equivalent (likely trait-based DI container like `shaku` or custom implementation).

### 2. **Lifecycle Management is Pervasive**
The `Disposable` base class appears in 100+ classes. Resource cleanup through `_register()` and `DisposableStore` is the standard pattern. Rust's ownership system could simplify this but would require rewriting all event/listener registration patterns.

### 3. **IPC is the Process Boundary**
Electron's `ipcRenderer` / `ipcMain` with validated wrapper is the only cross-process communication. In Tauri, this maps to invoke/listen (request/response) or emit (events). Channel names like `'vscode:onBeforeUnload'` are hard-coded strings—would need registry.

### 4. **Process Environment is Mutable**
`process.env` is cloned and modified before subprocess spawning. `process.execPath`, `process.platform`, `process.cwd()` are used for platform-specific path resolution and subprocess discovery.

### 5. **Service Bootstrap Happens Once**
Main process instantiates entire service graph at startup via ServiceCollection. All renderer processes receive proxied access to main services via IPC. No hot reload of services.

