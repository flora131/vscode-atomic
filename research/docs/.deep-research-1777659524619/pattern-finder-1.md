# VS Code Core IDE Architecture Patterns

## Research Question
Porting VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

## Architecture Overview
VS Code's codebase uses a highly modularized architecture centered on:
- **Dependency Injection (DI)** via custom platform/instantiation system
- **IPC/RPC** via base/parts/ipc for process communication
- **Disposable Lifecycle** with Disposable base class
- **Service Architecture** with service decorators and registrations
- **Contribution System** via workbench contributions

---

## Pattern Examples

#### Pattern 1: Service Declaration and DI Registration
**Where:** `src/vs/platform/instantiation/common/instantiation.ts:41` and `src/vs/platform/instantiation/common/extensions.ts:25`
**What:** Core pattern for declaring injectable services using createDecorator and registering them with registerSingleton.

```typescript
// Service interface declaration with _serviceBrand marker
export const IEditorService = createDecorator<IEditorService>('editorService');

// Service registration (from debug.service.contribution.ts)
registerSingleton(IDebugService, DebugService, InstantiationType.Delayed);
registerSingleton(IDebugVisualizerService, DebugVisualizerService, InstantiationType.Delayed);
```

**Key aspects:**
- Service identifiers are created via `createDecorator<T>(serviceId: string)`
- Interfaces include `readonly _serviceBrand: undefined` marker for type safety
- InstantiationType controls eager vs. delayed instantiation (Eager=0, Delayed=1)
- Services are singletons registered globally in a registry

**Call-sites:**
- `src/vs/workbench/contrib/debug/browser/debug.service.contribution.ts:11-12`
- `src/vs/sessions/sessions.web.main.ts:109-127` (batch registrations)
- `src/vs/workbench/contrib/share/browser/share.contribution.ts:157`

---

#### Pattern 2: Service Implementation with Disposable Lifecycle
**Where:** `src/vs/workbench/contrib/debug/browser/debugService.ts:63-117`
**What:** Service implementation pattern showing constructor injection, disposable management, and state initialization.

```typescript
export class DebugService implements IDebugService {
	declare readonly _serviceBrand: undefined;

	private readonly _onDidChangeState: Emitter<State>;
	private readonly _onDidNewSession: Emitter<IDebugSession>;
	private readonly disposables = new DisposableStore();
	private debugType!: IContextKey<string>;
	private debugState!: IContextKey<string>;

	constructor(
		@IEditorService private readonly editorService: IEditorService,
		@IPaneCompositePartService private readonly paneCompositeService: IPaneCompositePartService,
		@IViewsService private readonly viewsService: IViewsService,
		@INotificationService private readonly notificationService: INotificationService,
		@IDialogService private readonly dialogService: IDialogService,
		@ILifecycleService private readonly lifecycleService: ILifecycleService,
		@IInstantiationService private readonly instantiationService: IInstantiationService,
		// ... more service dependencies
	) {
		this.breakpointsToSendOnResourceSaved = new Set<URI>();
		this._onDidChangeState = this.disposables.add(new Emitter<State>());
	}
}
```

**Key aspects:**
- Constructor parameters use `@ServiceDecorator` syntax for DI
- Services declare `_serviceBrand: undefined` for type identity
- Private `DisposableStore` manages lifecycle cleanup
- Emitters for events are added to disposables
- Services compose other services without direct instantiation

**Variations / call-sites:**
- `src/vs/workbench/contrib/terminal/browser/terminalService.ts:66-120` (TerminalService)
- `src/vs/sessions/contrib/codeReview/browser/codeReviewService.ts:309`
- `src/vs/sessions/contrib/agentFeedback/browser/agentFeedbackService.ts:128`

---

#### Pattern 3: IPC Channel Server/Client Pattern
**Where:** `src/vs/platform/download/common/downloadIpc.ts:12-42`
**What:** Bi-directional RPC pattern for cross-process service communication.

```typescript
// Server-side channel implementation
export class DownloadServiceChannel implements IServerChannel {

	constructor(private readonly service: IDownloadService) { }

	listen(_: unknown, event: string, arg?: any): Event<any> {
		throw new Error('Invalid listen');
	}

	call(context: any, command: string, args?: any): Promise<any> {
		switch (command) {
			case 'download': return this.service.download(
				URI.revive(args[0]), 
				URI.revive(args[1]), 
				args[2] ?? 'downloadIpc'
			);
		}
		throw new Error('Invalid call');
	}
}

// Client-side proxy implementation
export class DownloadServiceChannelClient implements IDownloadService {

	declare readonly _serviceBrand: undefined;

	constructor(private channel: IChannel, private getUriTransformer: () => IURITransformer | null) { }

	async download(from: URI, to: URI, _callSite?: string): Promise<void> {
		const uriTransformer = this.getUriTransformer();
		if (uriTransformer) {
			from = uriTransformer.transformOutgoingURI(from);
			to = uriTransformer.transformOutgoingURI(to);
		}
		await this.channel.call('download', [from, to]);
	}
}
```

**Key aspects:**
- Server-side channel marshals local service calls to IPC
- Client-side channel proxy unmarshals responses
- URI transformation for multi-machine scenarios
- Channels registered with `registerChannel(name, channel)` on servers
- Base implementation: `src/vs/base/parts/ipc/common/ipc.ts`

**Call-sites:**
- `src/vs/platform/policy/common/policyIpc.ts:14` (PolicyChannel)
- `src/vs/platform/meteredConnection/electron-main/meteredConnectionChannel.ts:14`
- `src/vs/code/electron-main/app.ts:1224-1231` (channel registration)

---

#### Pattern 4: Disposable Base Class Lifecycle
**Where:** `src/vs/base/common/lifecycle.ts:526-557`
**What:** Core lifecycle management pattern used by all stateful services.

```typescript
export abstract class Disposable implements IDisposable {

	/**
	 * A disposable that does nothing when disposed of.
	 */
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

	/**
	 * Adds `o` to the collection of disposables managed by this object.
	 */
	protected _register<T extends IDisposable>(o: T): T {
		if ((o as unknown as Disposable) === this) {
			throw new Error('Cannot register a disposable on itself!');
		}
		return this._store.add(o);
	}
}
```

**Key aspects:**
- All major components (services, UI, workbench) extend Disposable
- Protected `_register()` adds disposables to internal store
- `dispose()` cascades cleanup to all registered children
- Leak tracking support via optional `IDisposableTracker`
- Automatic parent-child tracking for memory safety

**Variations / call-sites:**
- Used by all service implementations (DebugService, TerminalService, etc.)
- Extended by UI components: `src/vs/sessions/contrib/chat/browser/newChatViewPane.ts:35`
- Part of base architecture: `src/vs/base/common/lifecycle.ts:416-530`

---

#### Pattern 5: Terminal Service Multi-Faceted Interface
**Where:** `src/vs/workbench/contrib/terminal/browser/terminal.ts:39-46`
**What:** Complex feature area using multiple service decorators for different concerns.

```typescript
export const ITerminalService = createDecorator<ITerminalService>('terminalService');
export const ITerminalConfigurationService = createDecorator<ITerminalConfigurationService>('terminalConfigurationService');
export const ITerminalEditorService = createDecorator<ITerminalEditorService>('terminalEditorService');
export const ITerminalEditingService = createDecorator<ITerminalEditingService>('terminalEditingService');
export const ITerminalGroupService = createDecorator<ITerminalGroupService>('terminalGroupService');
export const ITerminalInstanceService = createDecorator<ITerminalInstanceService>('terminalInstanceService');
export const ITerminalChatService = createDecorator<ITerminalChatService>('terminalChatService');
```

**Implementation:**
```typescript
export class TerminalService extends Disposable implements ITerminalService {
	declare _serviceBrand: undefined;

	private _hostActiveTerminals: Map<ITerminalInstanceHost, ITerminalInstance | undefined> = new Map();
	private _detachedXterms = new Set<IDetachedTerminalInstance>();
	private _isShuttingDown: boolean = false;
	private _backgroundedTerminalInstances: IBackgroundTerminal[] = [];
	private _terminalShellTypeContextKey: IContextKey<string>;

	constructor(
		@ITerminalConfigurationService private readonly terminalConfigService: ITerminalConfigurationService,
		@ITerminalEditorService private readonly terminalEditorService: ITerminalEditorService,
		@ITerminalGroupService private readonly terminalGroupService: ITerminalGroupService,
		// ... more services
	) {
		super();
		// initialization
	}

	get instances(): ITerminalInstance[] {
		return this._terminalGroupService.instances
			.concat(this._terminalEditorService.instances)
			.concat(this._backgroundedTerminalInstances.map(bg => bg.instance));
	}
}
```

**Key aspects:**
- Feature areas split into logical service facades
- Each service handles specific concerns (configuration, editor, grouping, etc.)
- Main service composes sub-services
- Event multiplexing across backends
- State management for multiple terminal instances

**Variations / call-sites:**
- `src/vs/workbench/contrib/terminal/browser/terminalService.ts:66-120` (main implementation)
- `src/vs/editor/common/services/languageFeatures.ts:10-81` (language features registry pattern)

---

#### Pattern 6: Language Features Service (Registry Pattern)
**Where:** `src/vs/editor/common/services/languageFeatures.ts:10-81`
**What:** Service providing registries for pluggable language features.

```typescript
export const ILanguageFeaturesService = createDecorator<ILanguageFeaturesService>('ILanguageFeaturesService');

export interface ILanguageFeaturesService {

	readonly _serviceBrand: undefined;

	readonly referenceProvider: LanguageFeatureRegistry<ReferenceProvider>;
	readonly definitionProvider: LanguageFeatureRegistry<DefinitionProvider>;
	readonly typeDefinitionProvider: LanguageFeatureRegistry<TypeDefinitionProvider>;
	readonly declarationProvider: LanguageFeatureRegistry<DeclarationProvider>;
	readonly implementationProvider: LanguageFeatureRegistry<ImplementationProvider>;
	readonly codeActionProvider: LanguageFeatureRegistry<CodeActionProvider>;
	readonly renameProvider: LanguageFeatureRegistry<RenameProvider>;
	readonly documentFormattingEditProvider: LanguageFeatureRegistry<DocumentFormattingEditProvider>;
	readonly documentRangeFormattingEditProvider: LanguageFeatureRegistry<DocumentRangeFormattingEditProvider>;
	readonly documentSymbolProvider: LanguageFeatureRegistry<DocumentSymbolProvider>;
	readonly hoverProvider: LanguageFeatureRegistry<HoverProvider>;
	readonly completionProvider: LanguageFeatureRegistry<CompletionItemProvider>;
	readonly codeLensProvider: LanguageFeatureRegistry<CodeLensProvider>;
	readonly signatureHelpProvider: LanguageFeatureRegistry<SignatureHelpProvider>;
	// ... more providers
}
```

**Key aspects:**
- Centralizes access to all language feature providers
- Uses LanguageFeatureRegistry for per-language provider management
- Supports multiple simultaneous providers per feature
- Enables extensions to register new capabilities
- Pattern used throughout editor subsystem

---

#### Pattern 7: Source Control Service (Complex Domain Model)
**Where:** `src/vs/workbench/contrib/scm/common/scm.ts:35-96`
**What:** Domain-driven service for version control integration.

```typescript
export const ISCMService = createDecorator<ISCMService>('scm');

// Complex domain model
export interface ISCMResource {
	readonly resourceGroup: ISCMResourceGroup;
	readonly sourceUri: URI;
	readonly decorations: ISCMResourceDecorations;
	readonly contextValue: string | undefined;
	readonly command: Command | undefined;
	open(preserveFocus: boolean): Promise<void>;
}

export interface ISCMResourceGroup {
	readonly id: string;
	readonly provider: ISCMProvider;
	readonly resources: readonly ISCMResource[];
	readonly resourceTree: ResourceTree<ISCMResource, ISCMResourceGroup>;
	readonly onDidChangeResources: Event<void>;
	readonly label: string;
	readonly hideWhenEmpty: boolean;
}

export interface ISCMProvider extends IDisposable {
	readonly id: string;
	readonly providerId: string;
	readonly label: string;
	readonly groups: readonly ISCMResourceGroup[];
	readonly rootUri?: URI;
	readonly inputBoxTextModel: ITextModel;
	readonly acceptInputCommand?: Command;
	getOriginalResource(uri: URI): Promise<URI | null>;
}
```

**Key aspects:**
- Rich domain model for source control providers
- Tree structure for organizing resources
- Observable state (IObservable<T>) for reactive updates
- Event-based change notifications
- Multi-provider support (Git, Hg, Perforce, etc.)

---

#### Pattern 8: InstantiationService Dependency Injection
**Where:** `src/vs/platform/instantiation/common/instantiationService.ts:28-120`
**What:** Core DI container implementation for service creation and lifecycle.

```typescript
export class InstantiationService implements IInstantiationService {

	declare readonly _serviceBrand: undefined;

	readonly _globalGraph?: Graph<string>;
	private _globalGraphImplicitDependency?: string;

	private _isDisposed = false;
	private readonly _servicesToMaybeDispose = new Set<any>();
	private readonly _children = new Set<InstantiationService>();

	constructor(
		private readonly _services: ServiceCollection = new ServiceCollection(),
		private readonly _strict: boolean = false,
		private readonly _parent?: InstantiationService,
		private readonly _enableTracing: boolean = _enableAllTracing
	) {
		this._services.set(IInstantiationService, this);
		this._globalGraph = _enableTracing ? _parent?._globalGraph ?? new Graph(e => e) : undefined;
	}

	dispose(): void {
		if (!this._isDisposed) {
			this._isDisposed = true;
			dispose(this._children);
			this._children.clear();

			for (const candidate of this._servicesToMaybeDispose) {
				if (isDisposable(candidate)) {
					candidate.dispose();
				}
			}
			this._servicesToMaybeDispose.clear();
		}
	}

	createChild(services: ServiceCollection, store?: DisposableStore): IInstantiationService {
		this._throwIfDisposed();

		const that = this;
		const result = new class extends InstantiationService {
			override dispose(): void {
				that._children.delete(result);
				super.dispose();
			}
		}(services, this._strict, this, this._enableTracing);
		this._children.add(result);

		store?.add(result);
		return result;
	}

	invokeFunction<R, TS extends any[] = []>(fn: (accessor: ServicesAccessor, ...args: TS) => R, ...args: TS): R {
		this._throwIfDisposed();

		const _trace = Trace.traceInvocation(this._enableTracing, fn);
		let _done = false;
		try {
			const accessor: ServicesAccessor = {
				get: <T>(id: ServiceIdentifier<T>) => {
					if (_done) {
						throw illegalState('service accessor is only valid during invocation');
					}

					const result = this._getOrCreateServiceInstance(id, _trace);
					if (!result) {
						this._throwIfStrict(`[invokeFunction] unknown service '${id}'`, false);
					}
					return result;
				}
			};
			return fn(accessor, ...args);
		} finally {
			_done = true;
			_trace.stop();
		}
	}

	createInstance<T>(descriptor: SyncDescriptor0<T>): T;
	createInstance<Ctor extends new (...args: any[]) => unknown, R extends InstanceType<Ctor>>(ctor: Ctor, ...args: GetLeadingNonServiceArgs<ConstructorParameters<Ctor>>): R;
	createInstance(ctorOrDescriptor: any | SyncDescriptor<any>, ...rest: unknown[]): unknown {
		this._throwIfDisposed();
		// ... implementation
	}
}
```

**Key aspects:**
- Service collection holds instance registry
- Parent-child hierarchy for scoped injection
- Graph-based cycle detection in dependency resolution
- Lazy instantiation with optional tracing
- Automatic disposal of created services
- ServicesAccessor for function invocation with DI

---

#### Pattern 9: Service Declaration Variants
**Where:** Multiple locations showing pattern variations
**What:** Different ways to declare services across codebase.

```typescript
// Basic service (from platform/contextview/browser/contextView.ts:17)
export const IContextViewService = createDecorator<IContextViewService>('contextViewService');

// Feature group services (from workbench/contrib/terminal/browser/terminal.ts)
export const ITerminalService = createDecorator<ITerminalService>('terminalService');
export const ITerminalConfigurationService = createDecorator<ITerminalConfigurationService>('terminalConfigurationService');
export const ITerminalEditorService = createDecorator<ITerminalEditorService>('terminalEditorService');

// Debug service (from workbench/contrib/debug/common/debug.ts:1133)
export const IDebugService = createDecorator<IDebugService>('debugService');

// Editor service (from workbench/services/editor/common/editorService.ts:17)
export const IEditorService = createDecorator<IEditorService>('editorService');

// Source control service (from workbench/contrib/scm/common/scm.ts:35)
export const ISCMService = createDecorator<ISCMService>('scm');

// Language features service (from editor/common/services/languageFeatures.ts:10)
export const ILanguageFeaturesService = createDecorator<ILanguageFeaturesService>('ILanguageFeaturesService');
```

**Registration patterns:**
```typescript
// Immediate registration (from debug.service.contribution.ts:11)
registerSingleton(IDebugService, DebugService, InstantiationType.Delayed);

// Batch registration (from sessions.web.main.ts:109-127)
registerSingleton(IWorkbenchExtensionManagementService, ExtensionManagementService, InstantiationType.Delayed);
registerSingleton(IAccessibilityService, AccessibilityService, InstantiationType.Delayed);
registerSingleton(IContextMenuService, ContextMenuService, InstantiationType.Delayed);
// ... many more

// Descriptor registration
registerSingleton(IDataChannelService, descriptor, InstantiationType.Delayed);
```

---

## Summary: Architecture Patterns for Porting

### Core Infrastructure (Must Port)
1. **Dependency Injection System** - Custom createDecorator/registerSingleton pattern
2. **Disposable Lifecycle** - _store/dispose() hierarchy throughout
3. **IPC Channels** - Channel/ServerChannel protocol for process communication
4. **Service Registry** - Global singleton registry with Eager/Delayed instantiation

### Major Subsystems Patterns
1. **Editor Service** - Central editor group/pane management
2. **Debug Service** - Multi-session debugging, breakpoint management
3. **Terminal Service** - Multi-instance terminal backend abstraction
4. **Source Control (SCM)** - Multi-provider VCS integration
5. **Language Features** - Pluggable provider registries

### Cross-Cutting Patterns
1. **Decorators for DI** - @ServiceName in constructor parameters
2. **Event emitters** - Emitter<T> for state changes
3. **Context keys** - IContextKey<T> for menu/command enablement
4. **DisposableStore** - Manages event/listener cleanup
5. **URI handling** - Deep URI serialization/deserialization for IPC

### Key File Locations
- DI System: `src/vs/platform/instantiation/common/` (120 LOC)
- IPC Base: `src/vs/base/parts/ipc/common/ipc.ts` (150+ LOC)
- Lifecycle: `src/vs/base/common/lifecycle.ts` (900+ LOC)
- Service Examples: `src/vs/workbench/contrib/{debug,terminal,scm}/`

