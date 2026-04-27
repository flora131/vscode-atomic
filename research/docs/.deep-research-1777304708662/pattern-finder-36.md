# Git-Base Extension Pattern Analysis

## Overview
The `extensions/git-base/` folder implements a versioned extension API pattern that provides remote source provider infrastructure for git operations. This analysis identifies 6 distinct architectural and implementation patterns used throughout the codebase.

---

## Pattern 1: Versioned Public API Definition

**Where:** `extensions/git-base/src/api/git-base.d.ts:9-31`

**What:** Declares a versioned public API contract with method definitions for remote source management and picking operations.

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

**Variations/call-sites:**
- `extensions/git-base/src/api/api1.ts:12` — Implementation of API interface
- `extensions/git-base/src/api/extension.ts:44-54` — Versioned API getter with error handling

---

## Pattern 2: Extension Activation with Dependency Injection

**Where:** `extensions/git-base/src/extension.ts:12-22`

**What:** Extension entry point uses constructor injection to compose model and API implementations, registers commands, and providers via context subscriptions.

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

**Variations/call-sites:**
- `extensions/git-base/src/api/extension.ts:37-42` — Constructor initialization with optional model
- `extensions/git-base/src/api/api1.ts:29-41` — Command registration pattern

---

## Pattern 3: Event-Based Provider Registry with Set Management

**Where:** `extensions/git-base/src/model.ts:11-34`

**What:** Implements a registry pattern with EventEmitter for tracking provider lifecycle, using a Set for deduplication and fires events on add/remove.

```typescript
export class Model implements IRemoteSourceProviderRegistry {

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
}
```

**Variations/call-sites:**
- `extensions/git-base/src/remoteProvider.ts:9-15` — Registry interface contract
- `extensions/git-base/src/api/api1.ts:24-26` — Delegation to model registration

---

## Pattern 4: Method Decorators for Throttle/Debounce

**Where:** `extensions/git-base/src/decorators.ts:8-48`

**What:** TypeScript decorator functions that wrap method execution with debounce and throttle behaviors, using instance properties to track timers and pending promises.

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

function _throttle<T>(fn: Function, key: string): Function {
	const currentKey = `$throttle$current$${key}`;
	const nextKey = `$throttle$next$${key}`;

	const trigger = function (this: any, ...args: any[]) {
		if (this[nextKey]) {
			return this[nextKey];
		}

		if (this[currentKey]) {
			this[nextKey] = done(this[currentKey]).then(() => {
				this[nextKey] = undefined;
				return trigger.apply(this, args);
			});

			return this[nextKey];
		}

		this[currentKey] = fn.apply(this, args) as Promise<T>;

		const clear = () => this[currentKey] = undefined;
		done(this[currentKey]).then(clear, clear);

		return this[currentKey];
	};

	return trigger;
}
```

**Variations/call-sites:**
- `extensions/git-base/src/remoteSource.ts:57-60` — `@debounce(300)` on `onDidChangeValue`
- `extensions/git-base/src/remoteSource.ts:62-63` — `@throttle` on `query` async method

---

## Pattern 5: Stateful QuickPick Manager with Lifecycle

**Where:** `extensions/git-base/src/remoteSource.ts:26-110`

**What:** Encapsulates VS Code QuickPick UI state management with Disposable pattern, using lazy initialization and cleanup in a dedicated class with decorated event handlers.

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
			if (this.provider.supportsQuery) {
				this.quickpick.placeholder = this.provider.placeholder ?? l10n.t('Repository name (type to search)');
				this.disposables.push(this.quickpick.onDidChangeValue(this.onDidChangeValue, this));
			}
		}
	}

	@debounce(300)
	private onDidChangeValue(): void {
		this.query();
	}

	@throttle
	private async query(): Promise<void> {
		try {
			if (this.isDisposed) {
				return;
			}
			this.ensureQuickPick();
			this.quickpick!.busy = true;
			this.quickpick!.show();

			const remoteSources = await this.provider.getRemoteSources(this.quickpick?.value) || [];
			if (this.isDisposed) {
				return;
			}

			if (remoteSources.length === 0) {
				this.quickpick!.items = [{
					label: l10n.t('No remote repositories found.'),
					alwaysShow: true
				}];
			} else {
				this.quickpick!.items = remoteSources.map(remoteSource => ({
					label: remoteSource.icon ? `$(${remoteSource.icon}) ${remoteSource.name}` : remoteSource.name,
					description: remoteSource.description || (typeof remoteSource.url === 'string' ? remoteSource.url : remoteSource.url[0]),
					detail: remoteSource.detail,
					remoteSource,
					alwaysShow: true
				}));
			}
		} catch (err) {
			this.quickpick!.items = [{ label: l10n.t('{0} Error: {1}', '$(error)', err.message), alwaysShow: true }];
			console.error(err);
		} finally {
			if (!this.isDisposed) {
				this.quickpick!.busy = false;
			}
		}
	}

	async pick(): Promise<RemoteSource | undefined> {
		await this.query();
		if (this.isDisposed) {
			return;
		}
		const result = await getQuickPickResult(this.quickpick!);
		return result?.remoteSource;
	}
}
```

**Variations/call-sites:**
- `extensions/git-base/src/remoteSource.ts:130-200` — Multi-provider QuickPick with custom items

---

## Pattern 6: Language Folding Provider Implementation

**Where:** `extensions/git-base/src/foldingProvider.ts:8-92`

**What:** Implements VS Code's FoldingRangeProvider interface to parse git-commit messages, creating folds for comment blocks and diff sections using stateful line iteration.

```typescript
export class GitCommitFoldingProvider implements vscode.FoldingRangeProvider {

	provideFoldingRanges(
		document: vscode.TextDocument,
		_context: vscode.FoldingContext,
		_token: vscode.CancellationToken
	): vscode.ProviderResult<vscode.FoldingRange[]> {
		const ranges: vscode.FoldingRange[] = [];

		let commentBlockStart: number | undefined;
		let currentDiffStart: number | undefined;

		for (let i = 0; i < document.lineCount; i++) {
			const line = document.lineAt(i);
			const lineText = line.text;

			if (lineText.startsWith('#')) {
				if (currentDiffStart !== undefined) {
					if (i - currentDiffStart > 1) {
						ranges.push(new vscode.FoldingRange(currentDiffStart, i - 1));
					}
					currentDiffStart = undefined;
				}

				if (commentBlockStart === undefined) {
					commentBlockStart = i;
				}
			} else {
				if (commentBlockStart !== undefined) {
					if (i - commentBlockStart > 1) {
						ranges.push(new vscode.FoldingRange(
							commentBlockStart,
							i - 1,
							vscode.FoldingRangeKind.Comment
						));
					}
					commentBlockStart = undefined;
				}
			}

			if (lineText.startsWith('diff --git ')) {
				if (currentDiffStart !== undefined) {
					if (i - currentDiffStart > 1) {
						ranges.push(new vscode.FoldingRange(currentDiffStart, i - 1));
					}
				}
				currentDiffStart = i;
			}
		}

		// Handle end-of-document cases...
		return ranges;
	}
}
```

**Variations/call-sites:**
- `extensions/git-base/src/test/foldingProvider.test.ts:11-259` — Comprehensive test suite with 15+ test cases

---

## Pattern 7: Utility Composition for Promise and Version Handling

**Where:** `extensions/git-base/src/util.ts:6-69`

**What:** Exports reusable utility functions and namespaces for disposables, promise composition, and semantic version comparison with parsing.

```typescript
export interface IDisposable {
	dispose(): void;
}

export function toDisposable(dispose: () => void): IDisposable {
	return { dispose };
}

export function done<T>(promise: Promise<T>): Promise<void> {
	return promise.then<void>(() => undefined);
}

export namespace Versions {
	declare type VersionComparisonResult = -1 | 0 | 1;

	export interface Version {
		major: number;
		minor: number;
		patch: number;
		pre?: string;
	}

	export function compare(v1: string | Version, v2: string | Version): VersionComparisonResult {
		if (typeof v1 === 'string') {
			v1 = fromString(v1);
		}
		if (typeof v2 === 'string') {
			v2 = fromString(v2);
		}

		if (v1.major > v2.major) { return 1; }
		if (v1.major < v2.major) { return -1; }

		if (v1.minor > v2.minor) { return 1; }
		if (v1.minor < v2.minor) { return -1; }

		if (v1.patch > v2.patch) { return 1; }
		if (v1.patch < v2.patch) { return -1; }

		if (v1.pre === undefined && v2.pre !== undefined) { return 1; }
		if (v1.pre !== undefined && v2.pre === undefined) { return -1; }

		if (v1.pre !== undefined && v2.pre !== undefined) {
			return v1.pre.localeCompare(v2.pre) as VersionComparisonResult;
		}

		return 0;
	}

	export function fromString(version: string): Version {
		const [ver, pre] = version.split('-');
		const [major, minor, patch] = ver.split('.');
		return from(major, minor, patch, pre);
	}
}
```

**Variations/call-sites:**
- `extensions/git-base/src/decorators.ts:31` — `done()` for promise chaining in throttle
- `extensions/git-base/src/model.ts:25` — `toDisposable()` for factory pattern

---

## Test Pattern

**Where:** `extensions/git-base/src/test/foldingProvider.test.ts:11-259`

**What:** Mocha-based test suite using mock document pattern and assertion-based verification for FoldingRangeProvider edge cases.

```typescript
suite('GitCommitFoldingProvider', () => {

	function createMockDocument(content: string): vscode.TextDocument {
		const lines = content.split('\n');
		return {
			lineCount: lines.length,
			lineAt: (index: number) => ({
				text: lines[index] || '',
				lineNumber: index
			}),
		} as vscode.TextDocument;
	}

	const mockContext: vscode.FoldingContext = {} as vscode.FoldingContext;
	const mockToken: vscode.CancellationToken = { isCancellationRequested: false } as vscode.CancellationToken;

	test('empty document returns no folding ranges', () => {
		const provider = new GitCommitFoldingProvider();
		const doc = createMockDocument('');
		const ranges = provider.provideFoldingRanges(doc, mockContext, mockToken);

		assert.strictEqual(Array.isArray(ranges) ? ranges.length : 0, 0);
	});

	test('two comment lines create one folding range', () => {
		const provider = new GitCommitFoldingProvider();
		const content = '# Comment 1\n# Comment 2';
		const doc = createMockDocument(content);
		const ranges = provider.provideFoldingRanges(doc, mockContext, mockToken) as vscode.FoldingRange[];

		assert.strictEqual(ranges.length, 1);
		assert.strictEqual(ranges[0].start, 0);
		assert.strictEqual(ranges[0].end, 1);
		assert.strictEqual(ranges[0].kind, vscode.FoldingRangeKind.Comment);
	});
});
```

---

## Summary

This extension provides a foundational architecture for git operations in VS Code through seven key patterns:

1. **Versioned API contracts** enable stable evolution of extension APIs with strict versioning
2. **Dependency injection during activation** decouples component composition from implementations
3. **Event-based registries** provide reactive provider management with lifecycle tracking
4. **Decorator-based throttle/debounce** control execution of expensive operations without modifying call sites
5. **Stateful UI managers** encapsulate QuickPick complexity with proper resource cleanup
6. **Language provider implementations** integrate tightly with VS Code's editor folding infrastructure
7. **Utility namespaces** consolidate reusable behaviors for disposal, promises, and versioning

The extension demonstrates a TypeScript/Electron pattern suitable for porting: clear interface separation, event-driven architecture, explicit lifecycle management via Disposable pattern, and focused provider registration mechanics that could map to Rust equivalents with appropriate FFI or IPC boundaries.
