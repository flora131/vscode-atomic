# Pattern Research: API Exposure & Service Registration in git-base Extension

## Patterns Found in extensions/git-base/

### Pattern 1: Versioned API Interface Abstraction
**Where:** `extensions/git-base/src/api/git-base.d.ts:9-13`
**What:** Defines public API contract via interface, with getAPI(version) for backwards compatibility.

```typescript
export interface API {
	registerRemoteSourceProvider(provider: RemoteSourceProvider): Disposable;
	getRemoteSourceActions(url: string): Promise<RemoteSourceAction[]>;
	pickRemoteSource(options: PickRemoteSourceOptions): Promise<string | PickRemoteSourceResult | undefined>;
}

export interface GitBaseExtension {
	readonly enabled: boolean;
	readonly onDidChangeEnablement: Event<boolean>;

	getAPI(version: 1): API;
}
```

**Variations / call-sites:** 
- `extensions/git-base/src/api/extension.ts:44-54` implements version-gated API retrieval
- `extensions/git-base/src/api/api1.ts:12-27` implements the API interface

---

### Pattern 2: Provider Registry with Event Emission
**Where:** `extensions/git-base/src/model.ts:15-29`
**What:** Central model maintains provider registry and fires events on registration/removal.

```typescript
private remoteSourceProviders = new Set<RemoteSourceProvider>();

private _onDidAddRemoteSourceProvider = new EventEmitter<RemoteSourceProvider>();
readonly onDidAddRemoteSourceProvider = this._onDidAddRemoteSourceProvider.event;

private _onDidRemoveRemoteSourceProvider = new EventEmitter<RemoteSourceProvider>();
readonly onDidRemoveRemoteSourceProvider = this._onDidRemoveRemoteSourceProvider.event;

registerRemoteSourceProvider(provider: RemoteSourceProvider): Disposable {
	this.remoteSourceProviders.add(provider);
	this._onDidAddRemoteSourceProvider.fire(provider);

	return toDisposable(() => {
		this.remoteSourceProviders.delete(provider);
		this._onDidRemoveRemoteSourceProvider.fire(provider);
	});
}
```

**Variations / call-sites:**
- `extensions/git-base/src/api/api1.ts:24-26` delegates registration to model
- `extensions/git-base/src/remoteProvider.ts:9-15` defines IRemoteSourceProviderRegistry interface

---

### Pattern 3: Optional Provider Methods via Conditional Chaining
**Where:** `extensions/git-base/src/api/git-base.d.ts:72-86`
**What:** Provider interface marks optional methods with `?:` and uses `ProviderResult<T>` for async/sync flexibility.

```typescript
export interface RemoteSourceProvider {
	readonly name: string;
	readonly icon?: string;
	readonly label?: string;
	readonly placeholder?: string;
	readonly supportsQuery?: boolean;

	getBranches?(url: string): ProviderResult<string[]>;
	getRemoteSourceActions?(url: string): ProviderResult<RemoteSourceAction[]>;
	getRecentRemoteSources?(query?: string): ProviderResult<RecentRemoteSource[]>;
	getRemoteSources(query?: string): ProviderResult<RemoteSource[]>;
}
```

**Variations / call-sites:**
- `extensions/git-base/src/remoteSource.ts:117-119` calls optional methods with `?.()` operator
- `extensions/git-base/src/remoteSource.ts:148` uses `getRecentRemoteSources?.()` defensively

---

### Pattern 4: API Implementation Delegation Pattern
**Where:** `extensions/git-base/src/api/api1.ts:12-27`
**What:** Concrete API implementation delegates to model instance, passing through method calls.

```typescript
export class ApiImpl implements API {

	constructor(private _model: Model) { }

	pickRemoteSource(options: PickRemoteSourceOptions): Promise<PickRemoteSourceResult | string | undefined> {
		return pickRemoteSource(this._model, options);
	}

	getRemoteSourceActions(url: string): Promise<RemoteSourceAction[]> {
		return getRemoteSourceActions(this._model, url);
	}

	registerRemoteSourceProvider(provider: RemoteSourceProvider): Disposable {
		return this._model.registerRemoteSourceProvider(provider);
	}
}
```

**Variations / call-sites:**
- `extensions/git-base/src/api/extension.ts:44-54` instantiates ApiImpl during getAPI()

---

### Pattern 5: Extension Activation with API Exposure
**Where:** `extensions/git-base/src/extension.ts:12-22`
**What:** Main activate() function returns GitBaseExtensionImpl for other extensions to consume.

```typescript
export function activate(context: ExtensionContext): GitBaseExtensionImpl {
	const apiImpl = new GitBaseExtensionImpl(new Model());
	context.subscriptions.push(registerAPICommands(apiImpl));

	// Register folding provider for git-commit language
	context.subscriptions.push(
		languages.registerFoldingRangeProvider('git-commit', new GitCommitFoldingProvider())
	);

	return apiImpl;
}
```

**Variations / call-sites:**
- Other extensions call `vscode.extensions.getExtension('ms-vscode.git-base')?.exports.getAPI(1)`

---

### Pattern 6: Command Registration for API Methods
**Where:** `extensions/git-base/src/api/api1.ts:29-41`
**What:** Wraps API methods as VS Code commands to expose via command palette.

```typescript
export function registerAPICommands(extension: GitBaseExtensionImpl): Disposable {
	const disposables: Disposable[] = [];

	disposables.push(commands.registerCommand('git-base.api.getRemoteSources', (opts?: PickRemoteSourceOptions) => {
		if (!extension.model || !opts) {
			return;
		}

		return pickRemoteSource(extension.model, opts);
	}));

	return Disposable.from(...disposables);
}
```

**Variations / call-sites:**
- Called during activation in `extensions/git-base/src/extension.ts:14`

---

### Pattern 7: Enablement State with Event Notification
**Where:** `extensions/git-base/src/api/extension.ts:11-31`
**What:** Extension wrapper tracks enabled state and fires events on state changes.

```typescript
export class GitBaseExtensionImpl implements GitBaseExtension {

	enabled: boolean = false;

	private _onDidChangeEnablement = new EventEmitter<boolean>();
	readonly onDidChangeEnablement: Event<boolean> = this._onDidChangeEnablement.event;

	private _model: Model | undefined = undefined;

	set model(model: Model | undefined) {
		this._model = model;

		const enabled = !!model;

		if (this.enabled === enabled) {
			return;
		}

		this.enabled = enabled;
		this._onDidChangeEnablement.fire(this.enabled);
	}

	get model(): Model | undefined {
		return this._model;
	}
}
```

**Variations / call-sites:**
- `extensions/git-base/src/api/git-base.d.ts:15-19` defines interface contract

---

### Pattern 8: Language Provider Registration
**Where:** `extensions/git-base/src/foldingProvider.ts:8-14`
**What:** Implements FoldingRangeProvider interface for custom language support.

```typescript
export class GitCommitFoldingProvider implements vscode.FoldingRangeProvider {

	provideFoldingRanges(
		document: vscode.TextDocument,
		_context: vscode.FoldingContext,
		_token: vscode.CancellationToken
	): vscode.ProviderResult<vscode.FoldingRange[]> {
		const ranges: vscode.FoldingRange[] = [];
		// ... implementation
		return ranges;
	}
}
```

**Variations / call-sites:**
- Registered in `extensions/git-base/src/extension.ts:18` via `languages.registerFoldingRangeProvider()`

---

### Pattern 9: Decorator-Based Method Control Flow
**Where:** `extensions/git-base/src/decorators.ts:8-17`
**What:** Debounce and throttle decorators for controlling async method execution patterns.

```typescript
export function debounce(delay: number): Function {
	return decorate((fn, key) => {
		const timerKey = `$debounce$${key}`;

		return function (this: any, ...args: any[]) {
			clearTimeout(this[timerKey]);
			this[timerKey] = setTimeout(() => fn.apply(this, args), delay);
		};
	});
}

export const throttle = decorate(_throttle);
```

**Variations / call-sites:**
- Used in `extensions/git-base/src/remoteSource.ts:57-60` for debounced queries
- Used in `extensions/git-base/src/remoteSource.ts:62` for throttled async operations

---

### Pattern 10: Async UI Picker with Disposable Lifecycle
**Where:** `extensions/git-base/src/remoteSource.ts:26-110`
**What:** Manages QuickPick lifecycle with disposal pattern and listener cleanup.

```typescript
class RemoteSourceProviderQuickPick implements Disposable {

	private disposables: Disposable[] = [];
	private isDisposed: boolean = false;

	private quickpick: QuickPick<QuickPickItem & { remoteSource?: RemoteSource }> | undefined;

	constructor(private provider: RemoteSourceProvider) { }

	dispose() {
		this.disposables.forEach(d => d.dispose());
		this.disposables = [];
		this.quickpick = undefined;
		this.isDisposed = true;
	}

	private ensureQuickPick() {
		if (!this.quickpick) {
			this.quickpick = window.createQuickPick();
			this.disposables.push(this.quickpick);
			this.quickpick.ignoreFocusOut = true;
			this.disposables.push(this.quickpick.onDidHide(() => this.dispose()));
		}
	}
}
```

**Variations / call-sites:**
- Used in `extensions/git-base/src/remoteSource.ts:202-205` during provider source picking

---

## Summary

The git-base extension demonstrates **10 distinct API exposure and service integration patterns** relevant to porting VS Code functionality to Tauri/Rust:

1. **Versioned API interfaces** for backwards compatibility
2. **Provider registries** with event-driven updates
3. **Optional provider methods** for extensibility
4. **Delegation patterns** for decoupling API contracts from implementations
5. **Activation functions** returning public API objects
6. **Command wrapping** of API methods
7. **Enablement state tracking** with notifications
8. **Language provider registration** for custom editors
9. **Decorators** for flow control (debounce/throttle)
10. **Disposable UI components** with lifecycle management

These patterns show how VS Code manages cross-extension communication, plugin registration, and state synchronization—critical architectural concerns for any IDE port.
