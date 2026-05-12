# Pattern Finder - Partition 8: TypeScript Language Features Extension
## Process Forking and Language Server Registration Patterns

### Overview
The TypeScript language features extension (extensions/typescript-language-features/) demonstrates patterns for:
1. Spawning and managing TypeScript server processes across different platforms (Electron, Browser/Web)
2. Registering language providers with VS Code's language API
3. Conditional registration based on capabilities and configuration
4. Process communication via IPC and stdio channels

---

## Patterns Found

#### Pattern 1: Process Factory Interface with Platform Implementations
**Where:** `src/tsServer/server.ts:68-78`
**What:** Abstraction for spawning TypeScript server processes with implementations for Electron and Browser/Web.

```typescript
export interface TsServerProcessFactory {
	fork(
		version: TypeScriptVersion,
		args: readonly string[],
		kind: TsServerProcessKind,
		configuration: TypeScriptServiceConfiguration,
		versionManager: TypeScriptVersionManager,
		nodeVersionManager: NodeVersionManager,
		tsServerLog: TsServerLog | undefined,
	): TsServerProcess;
}
```

**Variations / call-sites:** 
- `src/tsServer/serverProcess.electron.ts:342` - ElectronServiceProcessFactory implementation
- `src/tsServer/serverProcess.browser.ts:33` - WorkerServerProcessFactory implementation
- `src/tsServer/spawner.ts:161` - Usage in spawnTsServer method

---

#### Pattern 2: Electron Process Forking with IPC/Stdio Abstraction
**Where:** `src/tsServer/serverProcess.electron.ts:343-386`
**What:** Creates child processes using node.js child_process.fork() with environment setup, graceful shutdown, and dual communication modes (IPC or stdio).

```typescript
fork(
	version: TypeScriptVersion,
	args: readonly string[],
	kind: TsServerProcessKind,
	configuration: TypeScriptServiceConfiguration,
	versionManager: TypeScriptVersionManager,
	nodeVersionManager: NodeVersionManager,
	_tsserverLog: TsServerLog | undefined,
): TsServerProcess {
	let tsServerPath = version.tsServerPath;

	if (!fs.existsSync(tsServerPath)) {
		vscode.window.showWarningMessage(vscode.l10n.t("The path {0} doesn\'t point to a valid tsserver install. Falling back to bundled TypeScript version.", tsServerPath));
		versionManager.reset();
		tsServerPath = versionManager.currentVersion.tsServerPath;
	}

	const execPath = nodeVersionManager.currentVersion;
	const env = generatePatchedEnv(process.env, tsServerPath, !!execPath);
	const runtimeArgs = [...args];
	const execArgv = getExecArgv(kind, configuration);
	const useGracefulShutdown = configuration.heapProfile.enabled;
	const useIpc = !execPath && version.apiVersion?.gte(API.v460);
	if (useIpc) {
		runtimeArgs.push('--useNodeIpc');
	}

	const childProcess = execPath ?
		child_process.spawn(execPath, [...execArgv, tsServerPath, ...runtimeArgs], {
			windowsHide: true,
			cwd: undefined,
			env,
		}) :
		child_process.fork(tsServerPath, runtimeArgs, {
			silent: true,
			cwd: undefined,
			env,
			execArgv,
			stdio: useIpc ? ['pipe', 'pipe', 'pipe', 'ipc'] : undefined,
		});

	return useIpc ? new IpcChildServerProcess(childProcess, useGracefulShutdown) : new StdioChildServerProcess(childProcess, useGracefulShutdown);
}
```

**Variations / call-sites:**
- Uses `child_process.fork()` for direct node process spawning
- Uses `child_process.spawn()` when custom Node version is provided
- `src/tsServer/serverProcess.electron.ts:158-190` - getExecArgv builds debug/memory/profiling arguments

---

#### Pattern 3: Web Worker-Based Process Implementation
**Where:** `src/tsServer/serverProcess.browser.ts:39-59`
**What:** Web/Browser variant using SharedArrayBuffer, Web Workers, and MessageChannel for communication with FileWatcherManager for filesystem events.

```typescript
public fork(
	version: TypeScriptVersion,
	args: readonly string[],
	kind: TsServerProcessKind,
	configuration: TypeScriptServiceConfiguration,
	_versionManager: TypeScriptVersionManager,
	_nodeVersionManager: NodeVersionManager,
	tsServerLog: TsServerLog | undefined,
) {
	const tsServerPath = version.tsServerPath;
	const launchArgs = [
		...args,
		// Explicitly give TS Server its path so it can load local resources
		'--executingFilePath', tsServerPath,
		// Enable/disable web type acquisition
		(configuration.webTypeAcquisitionEnabled && supportsReadableByteStreams() ? '--experimentalTypeAcquisition' : '--disableAutomaticTypingAcquisition'),
	];

	return new WorkerServerProcess(kind, tsServerPath, this._extensionUri, launchArgs, tsServerLog, this._logger);
}
```

**Variations / call-sites:**
- `src/tsServer/serverProcess.browser.ts:61-187` - WorkerServerProcess creates Worker, FileWatcherManager, MessageChannels for tsserver/watcher/syncFs
- Uses MessageChannel for three separate communication channels: sync TS server communication, async watcher events, filesystem sync

---

#### Pattern 4: Graceful Process Shutdown with Timeout
**Where:** `src/tsServer/serverProcess.electron.ts:215-271` (IpcChildServerProcess) and `273-340` (StdioChildServerProcess)
**What:** Implements graceful shutdown protocol with 5-second timeout fallback to force kill.

```typescript
class IpcChildServerProcess extends Disposable implements TsServerProcess {
	private _killTimeout: NodeJS.Timeout | undefined;
	private _isShuttingDown = false;

	kill(): void {
		if (!this._useGracefulShutdown) {
			this._process.kill();
			return;
		}

		if (this._isShuttingDown) {
			return;
		}
		this._isShuttingDown = true;

		try {
			this._process.send(tsServerExitRequest);
		} catch {
			this._process.kill();
			return;
		}

		this._killTimeout = setTimeout(() => this._process.kill(), gracefulExitTimeout);
		this._killTimeout.unref?.();
	}
}
```

**Variations / call-sites:**
- Both IPC and Stdio variants implement identical shutdown protocol
- `src/tsServer/serverProcess.electron.ts:27` - gracefulExitTimeout = 5000ms
- `src/tsServer/serverProcess.electron.ts:28-32` - tsServerExitRequest protocol definition

---

#### Pattern 5: Composite Server Architecture (Semantic/Syntax Routing)
**Where:** `src/tsServer/spawner.ts:56-99`
**What:** Spawns multiple TypeScript server instances (Main/Semantic/Syntax/Diagnostics) and routes requests based on capability and configuration.

```typescript
public spawn(
	version: TypeScriptVersion,
	capabilities: ClientCapabilities,
	configuration: TypeScriptServiceConfiguration,
	pluginManager: PluginManager,
	cancellerFactory: OngoingRequestCancellerFactory,
	delegate: TsServerDelegate,
): ITypeScriptServer {
	let primaryServer: ITypeScriptServer;
	const serverType = this.getCompositeServerType(version, capabilities, configuration);
	const shouldUseSeparateDiagnosticsServer = this.shouldUseSeparateDiagnosticsServer(configuration);

	switch (serverType) {
		case CompositeServerType.SeparateSyntax:
		case CompositeServerType.DynamicSeparateSyntax:
			{
				const enableDynamicRouting = !shouldUseSeparateDiagnosticsServer && serverType === CompositeServerType.DynamicSeparateSyntax;
				primaryServer = new SyntaxRoutingTsServer({
					syntax: this.spawnTsServer(TsServerProcessKind.Syntax, version, configuration, pluginManager, cancellerFactory),
					semantic: this.spawnTsServer(TsServerProcessKind.Semantic, version, configuration, pluginManager, cancellerFactory),
				}, delegate, enableDynamicRouting);
				break;
			}
		case CompositeServerType.Single:
			{
				primaryServer = this.spawnTsServer(TsServerProcessKind.Main, version, configuration, pluginManager, cancellerFactory);
				break;
			}
		case CompositeServerType.SyntaxOnly:
			{
				primaryServer = this.spawnTsServer(TsServerProcessKind.Syntax, version, configuration, pluginManager, cancellerFactory);
				break;
			}
	}

	if (shouldUseSeparateDiagnosticsServer) {
		return new GetErrRoutingTsServer({
			getErr: this.spawnTsServer(TsServerProcessKind.Diagnostics, version, configuration, pluginManager, cancellerFactory),
			primary: primaryServer,
		}, delegate);
	}

	return primaryServer;
}
```

**Variations / call-sites:**
- `src/tsServer/spawner.ts:101-122` - CompositeServerType selection logic
- Routing strategies: SeparateSyntax, DynamicSeparateSyntax (with optional routing), Single, SyntaxOnly, with optional GetErrRoutingTsServer layer

---

#### Pattern 6: Conditional Language Provider Registration
**Where:** `src/languageFeatures/definitions.ts:63-73`
**What:** Wraps vscode.languages.registerDefinitionProvider in conditional registration based on client capabilities and configuration.

```typescript
export function register(
	selector: DocumentSelector,
	client: ITypeScriptServiceClient,
) {
	return conditionalRegistration([
		requireSomeCapability(client, ClientCapability.EnhancedSyntax, ClientCapability.Semantic),
	], () => {
		return vscode.languages.registerDefinitionProvider(selector.syntax,
			new TypeScriptDefinitionProvider(client));
	});
}
```

**Variations / call-sites:**
- Multiple provider types with same pattern:
  - `hover.ts:105-116` - registerHoverProvider with requireSomeCapability
  - `completions.ts:930-946` - registerCompletionItemProvider with requireSomeCapability
  - `refactor.ts:774-782` - registerCodeActionsProvider with requireSomeCapability
  - `formatting.ts:90-103` - registerOnTypeFormattingEditProvider with requireGlobalUnifiedConfig
  - `semanticTokens.ts:15-25` - registerDocumentRangeSemanticTokensProvider

---

#### Pattern 7: Multi-Feature Provider Registration with Disposable Composition
**Where:** `src/languageFeatures/formatting.ts:87-103`
**What:** Registers multiple related providers (on-type formatting + document range formatting) using vscode.Disposable.from() composition.

```typescript
export function register(
	selector: DocumentSelector,
	language: LanguageDescription,
	client: ITypeScriptServiceClient,
	fileConfigurationManager: FileConfigurationManager
) {
	return conditionalRegistration([
		requireGlobalUnifiedConfig('format.enabled', { fallbackSection: language.id, fallbackSubSectionNameOverride: 'format.enable' }),
	], () => {
		const formattingProvider = new TypeScriptFormattingProvider(client, fileConfigurationManager);
		return vscode.Disposable.from(
			vscode.languages.registerOnTypeFormattingEditProvider(selector.syntax, formattingProvider, ';', '}', '\n'),
			vscode.languages.registerDocumentRangeFormattingEditProvider(selector.syntax, formattingProvider),
		);
	});
}
```

**Variations / call-sites:**
- `documentHighlight.ts:86-87` - Registers both registerDocumentHighlightProvider and registerMultiDocumentHighlightProvider

---

#### Pattern 8: Condition-Based Dynamic Registration System
**Where:** `src/languageFeatures/util/dependentRegistration.ts:12-141`
**What:** Implements reactive registration that responds to capability/configuration changes without requiring extension reload.

```typescript
export class Condition extends Disposable {
	private _value: boolean;

	constructor(
		private readonly getValue: () => boolean,
		onUpdate: (handler: () => void) => void,
	) {
		super();
		this._value = this.getValue();

		onUpdate(() => {
			const newValue = this.getValue();
			if (newValue !== this._value) {
				this._value = newValue;
				this._onDidChange.fire();
			}
		});
	}

	public get value(): boolean { return this._value; }

	private readonly _onDidChange = this._register(new vscode.EventEmitter<void>());
	public readonly onDidChange = this._onDidChange.event;
}

export function conditionalRegistration(
	conditions: readonly Condition[],
	doRegister: () => vscode.Disposable,
	elseDoRegister?: () => vscode.Disposable
): vscode.Disposable {
	return new ConditionalRegistration(conditions, doRegister, elseDoRegister);
}

export function requireSomeCapability(
	client: ITypeScriptServiceClient,
	...capabilities: readonly ClientCapability[]
) {
	return new Condition(
		() => capabilities.some(requiredCapability => client.capabilities.has(requiredCapability)),
		client.onDidChangeCapabilities
	);
}
```

**Variations / call-sites:**
- `requireMinVersion()` - Version-based conditions tied to `client.onTsServerStarted`
- `requireHasModifiedUnifiedConfig()` - Configuration modification detection
- `requireGlobalUnifiedConfig()` - Global config value presence
- `requireHasVsCodeExtension()` - Extension availability checks

---

## Summary: Implications for Tauri/Rust Port

### Process Management (High Priority)
The codebase demonstrates sophisticated process management patterns:
- **Platform abstraction**: ElectronServiceProcessFactory vs WorkerServerProcessFactory shows how to abstract platform-specific launching
- **IPC vs Stdio**: Supports both message-based (IPC) and stream-based (stdio) communication, important for choosing Rust communication strategy
- **Graceful shutdown**: 5-second timeout protocol before force kill - would need Rust equivalent using process signals/IPC
- **Composite routing**: Multiple specialized processes (Semantic, Syntax, Diagnostics) running simultaneously with request routing

### Language Provider Registration Pattern
The vscode.languages.register* API is heavily used across 30+ call sites with:
- **Conditional registration**: All providers check capabilities before registering (must translate to Tauri equivalent)
- **Disposable management**: Providers return disposables for cleanup (would map to Rust Drop trait)
- **Multiple trigger patterns**: Providers register with trigger characters, metadata, document selectors

### Process Arguments and Configuration
- Server spawning passes extensive configuration: debug ports, memory limits, heap profiling, tracing directories, locale, plugins, npm locations
- Environmental variable patching for NODE_PATH, ELECTRON_RUN_AS_NODE
- Version-specific features (API version gating: v400, v401, v460, v470, v544, v590)

### Worker/Multi-Process Architecture
Three distinct server types (Main, Syntax, Semantic, Diagnostics) with routing logic based on:
- Version capabilities
- Configuration (useSyntaxServer: Always/Never/Auto)
- Client capabilities (Semantic support)
- enableProjectDiagnostics flag

### Critical APIs to Replace
- `child_process.fork()` / `child_process.spawn()`
- `vscode.languages.register*Provider()` family (30+ variations)
- `vscode.Disposable`, event emitters
- File watching and configuration monitoring
