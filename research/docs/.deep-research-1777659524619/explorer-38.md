# Partition 38 of 79 — Findings

## Scope
`src/main.ts/` (1 files, 741 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Partition 38: Electron App Lifecycle & Protocol Registration

## Research Question
What would porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust require?

## Scope
File: `src/main.ts` (741 LOC) — The Electron application entry point handling startup initialization and custom protocol registration.

---

## Implementation

### Entry Point & Lifecycle
- `src/main.ts` — Electron app initialization: sandbox configuration, user data paths, CLI argument parsing, crash reporter setup, and NLS configuration before `app.ready` event.

### App Lifecycle Event Hooks
Key Electron lifecycle events registered in `src/main.ts`:
- Line 147: `app.once('ready')` — Main app readiness trigger; invokes `onReady()` which bootstraps ESM and loads `./vs/code/electron-main/main.js`
- Lines 597–621: Three macOS-specific file/URL handlers:
  - `app.on('open-file')` — Receives dropped file paths pre-launch
  - `app.on('will-finish-launching')` — Setup phase for URL handler registration
  - `app.on('open-url')` — Protocol URL handling (vscode:// scheme)

Additional lifecycle events in `src/vs/code/electron-main/app.ts`:
- Line 412: `app.on('accessibility-support-changed')` — Accessibility state monitoring
- Line 417: `app.on('activate')` — macOS re-activation handler
- Line 430: `app.on('web-contents-created')` — Auxiliary window tracking
- Line 479: `app.on('open-file')` — File opening from native OS dialogs
- Line 509: `app.on('new-window-for-tab')` — macOS tab bar behavior

### Custom Protocol Registration
- **Line 96–105** in `src/main.ts`: Early-stage privileged scheme registration via `protocol.registerSchemesAsPrivileged()`:
  - `vscode-webview` — WebView content isolation (sandbox: true, secure: true, supportFetchAPI: true, corsEnabled: true, allowServiceWorkers: true, codeCache: true)
  - `vscode-file` — File resource serving (secure: true, standard: true, supportFetchAPI: true, corsEnabled: true, codeCache: true)

### Protocol Handlers in Main Process
- `src/vs/platform/protocol/electron-main/protocolMainService.ts` — Implements `IProtocolMainService`:
  - Line 53: `defaultSession.protocol.registerFileProtocol(Schemas.vscodeFileResource, ...)` — Handles `vscode-file://` requests with validation against allowed roots and file extensions
  - Line 56: `defaultSession.protocol.interceptFileProtocol(Schemas.file, ...)` — Blocks raw `file://` access for security

- `src/vs/code/electron-main/app.ts`:
  - Line 698: `protocol.registerBufferProtocol(Schemas.vscodeManagedRemoteResource, ...)` — Proxies managed remote resources via IPC to main process
  - Line 1537: `protocol.registerHttpProtocol(Schemas.vscodeRemoteResource, ...)` — Converts `vscode-remote-resource:` to `http:` for tunneled resources

### Configuration & Arguments
- Lines 36–54: Pre-ready Chromium/Electron CLI switch configuration (hardware acceleration, color profile, proxy, debugging port)
- Lines 215–360: `configureCommandlineSwitchesSync()` handles argv.json config mapping to Electron flags:
  - GPU features (NetAdapterMaxBufSizeFeature, EarlyEstablishGpuChannel, EstablishGpuChannelAsync, CalculateNativeWinOcclusion, FontMatchingCTMigration, StandardizedBrowserZoom)
  - Blink rendering configuration
  - XDG portal version pinning (Linux)
  - WebGL context limits (32 max for terminal usage)

### Crash Reporter
- Lines 73–85, 449–539: Conditional crash reporter initialization with AppCenter integration; supports per-platform URLs (win32-x64, win32-arm64, darwin, darwin-arm64, linux-x64)

---

## Related Files

### Main Application Class
- `src/vs/code/electron-main/main.ts` — CodeMain class that orchestrates startup; imports CodeApplication from `app.ts` after ESM bootstrap
- `src/vs/code/electron-main/app.ts` (2,500+ LOC) — CodeApplication class implementing full app lifecycle (service initialization, window management, IPC servers, protocol handlers, update checks, telemetry, extension host)

### Lifecycle Management
- `src/vs/platform/lifecycle/electron-main/lifecycleMainService.ts` — Manages app shutdown phases and lifecycle events

### NLS & Localization
- `src/vs/base/node/nls.js` — Language resolution and message file loading
- Lines 658–725 in `src/main.ts`: Locale resolution with Chinese variant handling (zh-hans → zh-cn, zh-hant → zh-tw)

### Bootstrap
- `src/bootstrap-node.js` — Portable installation configuration
- `src/bootstrap-esm.js` — ESM module setup
- `src/bootstrap-meta.js` — Product metadata loading

---

## Key Porting Considerations for Tauri/Rust

### 1. Electron → Tauri App Lifecycle Replacement
**Current:** `app.on('ready')`, `app.once()`, `app.on('activate')`, `app.on('accessibility-support-changed')`
**Tauri Equivalent:** `setup()` hook and event system; no direct 1:1 mapping. Must implement platform-specific event listeners in Rust.

### 2. Custom Protocol Handlers
**Current:** 
- `protocol.registerSchemesAsPrivileged()` (pre-app-ready)
- `registerFileProtocol()`, `interceptFileProtocol()`, `registerBufferProtocol()`, `registerHttpProtocol()`

**Tauri Equivalent:** 
- Custom URI schemes via Tauri's `protocol` plugin or IPC-based routing
- File serving through Tauri's `asset` protocol and custom handlers
- No direct equivalent to Electron's per-session protocol interception; requires explicit handler registration

### 3. Sandbox & Security Configuration
**Current:** `app.enableSandbox()`, command-line switches for Chrome features
**Tauri Equivalent:** Sandbox enabled by default; cannot be disabled. Feature flags (GPU, rendering) must be configured in `tauri.conf.json` or Rust code.

### 4. Session/WebContents Management
**Current:** `session.defaultSession` for protocol registration; `app.on('web-contents-created')` for per-window handlers
**Tauri Equivalent:** Single unified WebView context per window; no concept of separate sessions. Protocol handlers are global.

### 5. IPC & Event Distribution
**Current:** Electron's `ipcMain`, `validatedIpcMain`, WebFrameMain contexts
**Tauri Equivalent:** Commands and events; synchronous/asynchronous command invocation model differs from Electron's message-passing.

### 6. File Access & Path Resolution
**Current:** `original-fs` for pre-app-ready file operations; UNC path handling on Windows
**Tauri Equivalent:** All file access through Tauri's `fs` plugin; UNC paths supported but require explicit `allowlist` in config.

### 7. Crash Reporting
**Current:** Electron's `crashReporter.start()` with AppCenter endpoints per platform
**Tauri Equivalent:** No built-in crash reporting; requires custom Rust implementation or third-party integration (e.g., Sentry).

### 8. Command-Line Arguments & argv.json
**Current:** `minimist` parsing; argv.json merged with CLI args; pre-ready synchronous operations
**Tauri Equivalent:** CLI parsing via Tauri's args or `std::env`; no built-in argv.json convention. Config resolution must happen before window creation.

### 9. Locale & NLS Resolution
**Current:** `app.getLocale()`, `app.getPreferredSystemLanguages()`, early promise-based resolution
**Tauri Equivalent:** System locale via Tauri's `window` API; NLS resolution logic must be moved to Rust or require IPC round-trip.

---

## Effort Estimation Summary

| Component | Effort | Notes |
|-----------|--------|-------|
| Electron lifecycle → Tauri setup | **High** | No 1:1 event mapping; requires Rust-side event loop redesign |
| Custom protocol handlers | **High** | Tauri's protocol API limited; may need WebSocket fallback for some schemes |
| Sandbox/security config | **Medium** | Tauri sandbox is non-negotiable; feature flags need mapping |
| IPC & command routing | **Very High** | Fundamental architectural difference; all `ipcMain` handlers must become Tauri commands |
| File I/O pre-app-ready | **Medium** | Tauri's fs plugin works but requires async; may need blocking operations for startup |
| Crash reporting | **High** | Must implement custom telemetry pipeline; AppCenter integration drops |
| Config/CLI arg parsing | **Medium** | Standard CLI parsing; argv.json handling must be custom |

---

## Conclusion

Porting `src/main.ts` from Electron to Tauri requires replacing:
1. **Lifecycle hooks** — Electron's rich event system with Tauri's simpler `setup()` + background tasks
2. **Protocol registration** — Electron's early, per-session scheme registration with Tauri's global, declarative protocol API
3. **IPC architecture** — The synchronous `ipcMain` request/response model with Tauri's command/event pattern
4. **Startup initialization** — Pre-app-ready file and configuration operations (Electron allows sync fs before app ready; Tauri requires async)

The largest surface area for porting lies in **protocol handlers** and **IPC remapping**, not the simple startup sequence. VS Code's deep reliance on Electron's session-based protocol isolation (separate `vscode-webview://` vs `vscode-file://` schemes with distinct permissions) cannot be directly replicated in Tauri's single-session architecture.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Electron App Lifecycle & Protocol Registration Patterns — Partition 38

Research on VS Code's core Electron entry point patterns relevant to a Tauri/Rust port.

## Entry Point Structure

#### Pattern: App Initialization Chain
**Where:** `src/main.ts:23-214`
**What:** Electron main process bootstrap with pre-ready configuration, ready event handler, and async startup chain.

```typescript
perf.mark('code/didStartMain');

// Enable sandbox globally
if (args['sandbox'] &&
	!args['disable-chromium-sandbox'] &&
	!argvConfig['disable-chromium-sandbox']) {
	app.enableSandbox();
}

// Set userData path before app 'ready' event
app.setPath('userData', userDataPath);

// Register custom schemes with privileges (pre-ready)
protocol.registerSchemesAsPrivileged([
	{
		scheme: 'vscode-webview',
		privileges: { standard: true, secure: true, supportFetchAPI: true, corsEnabled: true, allowServiceWorkers: true, codeCache: true }
	},
	{
		scheme: 'vscode-file',
		privileges: { secure: true, standard: true, supportFetchAPI: true, corsEnabled: true, codeCache: true }
	}
]);

// Global app listeners registered before ready
registerListeners();

// Load code once ready
app.once('ready', function () {
	if (args['trace']) {
		contentTracing.startRecording(traceOptions).finally(() => onReady());
	} else {
		onReady();
	}
});

async function onReady() {
	perf.mark('code/mainAppReady');
	try {
		const [, nlsConfig] = await Promise.all([
			mkdirpIgnoreError(codeCachePath),
			resolveNlsConfiguration()
		]);
		await startup(codeCachePath, nlsConfig);
	} catch (error) {
		console.error(error);
	}
}

async function startup(codeCachePath: string | undefined, nlsConfig: INLSConfiguration): Promise<void> {
	process.env['VSCODE_NLS_CONFIG'] = JSON.stringify(nlsConfig);
	process.env['VSCODE_CODE_CACHE_PATH'] = codeCachePath || '';

	// Bootstrap ESM
	await bootstrapESM();

	// Load Main bundle
	await import('./vs/code/electron-main/main.js');
	perf.mark('code/didRunMainBundle');
}
```

**Key aspects:**
- Pre-ready configuration phase (sandbox, paths, protocol schemes)
- Single `app.once('ready')` handler entry point
- Async startup chain after ready
- Optional tracing setup before main bundle load
- Environment variable injection for renderer process access


#### Pattern: Early Listener Registration
**Where:** `src/main.ts:589-621`
**What:** Register macOS-specific event listeners before app ready to capture initial open-file and open-url events.

```typescript
function registerListeners(): void {
	/**
	 * macOS: when someone drops a file to the not-yet running VSCode, 
	 * the open-file event fires even before the app-ready event.
	 */
	const macOpenFiles: string[] = [];
	(globalThis as { macOpenFiles?: string[] }).macOpenFiles = macOpenFiles;
	app.on('open-file', function (event, path) {
		macOpenFiles.push(path);
	});

	/**
	 * macOS: react to open-url requests.
	 */
	const openUrls: string[] = [];
	const onOpenUrl =
		function (event: { preventDefault: () => void }, url: string) {
			event.preventDefault();
			openUrls.push(url);
		};

	app.on('will-finish-launching', function () {
		app.on('open-url', onOpenUrl);
	});

	(globalThis as { getOpenUrls?: () => string[] }).getOpenUrls = function () {
		app.removeListener('open-url', onOpenUrl);
		return openUrls;
	};
}
```

**Variations / call-sites:**
- Pre-ready event handlers required on macOS for file/URL opening
- Global function attachment for main bundle to retrieve collected URLs
- Late listener setup via `will-finish-launching` event
- Event data buffering pattern (accumulate then flush)


## App Lifecycle Event Handlers

#### Pattern: macOS Dock Activation Handler
**Where:** `src/vs/code/electron-main/app.ts:417-424`
**What:** Restore window on dock click; macOS-specific lifecycle event.

```typescript
app.on('activate', async (event, hasVisibleWindows) => {
	this.logService.trace('app#activate');

	// Mac only event: open new window when we get activated
	if (!hasVisibleWindows) {
		await this.windowsMainService?.openEmptyWindow({ context: OpenContext.DOCK });
	}
});
```

**Variations / call-sites:**
- Platform-specific (macOS only)
- Async window opening operation
- Checks window visibility state


#### Pattern: Accessibility Support Change Event
**Where:** `src/vs/code/electron-main/app.ts:412-414`
**What:** Broadcast accessibility state changes to all open windows.

```typescript
app.on('accessibility-support-changed', (event, accessibilitySupportEnabled) => {
	this.windowsMainService?.sendToAll('vscode:accessibilitySupportChanged', accessibilitySupportEnabled);
});
```

**Key aspects:**
- System-wide event broadcast pattern
- IPC message distribution to all renderers


#### Pattern: Web Contents Security & Window Handling
**Where:** `src/vs/code/electron-main/app.ts:430-473`
**What:** Intercept all web content creation to enforce security policies and delegate window/navigation handling.

```typescript
app.on('web-contents-created', (event, contents) => {

	// Auxiliary Window: delegate to `AuxiliaryWindow` class
	if (contents?.opener?.url.startsWith(`${Schemas.vscodeFileResource}://${VSCODE_AUTHORITY}/`)) {
		this.logService.trace('[aux window] app.on("web-contents-created"): Registering auxiliary window');
		this.auxiliaryWindowsMainService?.registerWindow(contents);
	}

	// Handle any in-page navigation
	contents.on('will-navigate', event => {
		if (BrowserViewMainService.isBrowserViewWebContents(contents)) {
			return; // Allow navigation in integrated browser views
		}

		this.logService.error('webContents#will-navigate: Prevented webcontent navigation');
		event.preventDefault(); // Prevent any in-page navigation
	});

	// All Windows: only allow about:blank auxiliary windows to open
	contents.setWindowOpenHandler(details => {
		if (details.url === 'about:blank') {
			this.logService.trace('[aux window] webContents#setWindowOpenHandler: Allowing auxiliary window');
			return {
				action: 'allow',
				overrideBrowserWindowOptions: this.auxiliaryWindowsMainService?.createWindow(details)
			};
		} else {
			this.logService.trace(`webContents#setWindowOpenHandler: Prevented opening window with URL ${details.url}`);
			this.nativeHostMainService?.openExternal(undefined, details.url);
			return { action: 'deny' };
		}
	});
});
```

**Key aspects:**
- Centralized security policy enforcement point
- Content script isolation (only about:blank auxiliary windows allowed)
- Navigation blocking (prevent SPA routing)
- External URL delegation to OS
- Per-renderer event handler setup


#### Pattern: macOS File Drop & Open Handler
**Where:** `src/vs/code/electron-main/app.ts:479-507`
**What:** Buffer macOS open-file events with debounce to batch multiple file drops.

```typescript
let macOpenFileURIs: IWindowOpenable[] = [];
let runningTimeout: Timeout | undefined = undefined;
app.on('open-file', (event, path) => {
	path = normalizeNFC(path); // macOS only: normalize paths to NFC form

	this.logService.trace('app#open-file: ', path);
	event.preventDefault();

	// Keep in array because more might come!
	macOpenFileURIs.push(hasWorkspaceFileExtension(path) ? { workspaceUri: URI.file(path) } : { fileUri: URI.file(path) });

	// Clear previous handler if any
	if (runningTimeout !== undefined) {
		clearTimeout(runningTimeout);
		runningTimeout = undefined;
	}

	// Handle paths delayed in case more are coming!
	runningTimeout = setTimeout(async () => {
		await this.windowsMainService?.open({
			context: OpenContext.DOCK,
			cli: this.environmentMainService.args,
			urisToOpen: macOpenFileURIs,
			gotoLineMode: false,
			preferNewWindow: true
		});

		macOpenFileURIs = [];
		runningTimeout = undefined;
	}, 100);
});
```

**Variations / call-sites:**
- macOS native tab support: `app.on('new-window-for-tab')`
- Batching pattern with 100ms debounce
- Path normalization (NFC form on macOS)


#### Pattern: Power & System Event Monitoring
**Where:** `src/vs/code/electron-main/app.ts:1584-1590`
**What:** Listen to OS power events (suspend/resume) and report telemetry.

```typescript
this._register(Event.fromNodeEventEmitter(powerMonitor, 'suspend')(() => {
	telemetryService.publicLog2<PowerEvent, PowerEventClassification>('power.suspend', getPowerEventData());
}));

this._register(Event.fromNodeEventEmitter(powerMonitor, 'resume')(() => {
	telemetryService.publicLog2<PowerEvent, PowerEventClassification>('power.resume', getPowerEventData());
}));
```

**Key aspects:**
- NodeEventEmitter wrapper for telemetry integration
- Disposal registration pattern
- Typed telemetry events


## Custom Protocol Registration

#### Pattern: Pre-Ready Custom Scheme Registration
**Where:** `src/main.ts:96-105`
**What:** Register custom URL schemes with privileges before app ready event.

```typescript
protocol.registerSchemesAsPrivileged([
	{
		scheme: 'vscode-webview',
		privileges: { 
			standard: true, 
			secure: true, 
			supportFetchAPI: true, 
			corsEnabled: true, 
			allowServiceWorkers: true, 
			codeCache: true 
		}
	},
	{
		scheme: 'vscode-file',
		privileges: { 
			secure: true, 
			standard: true, 
			supportFetchAPI: true, 
			corsEnabled: true, 
			codeCache: true 
		}
	}
]);
```

**Key aspects:**
- Must be called before `app.ready` event
- Privileges define security and capability flags
- Separated webview vs file resource schemes


#### Pattern: File Protocol Handler with Validation
**Where:** `src/vs/platform/protocol/electron-main/protocolMainService.ts:49-63`
**What:** Register vscode-file handler with resource validation and block native file:// access.

```typescript
private handleProtocols(): void {
	const { defaultSession } = session;

	// Register vscode-file:// handler
	defaultSession.protocol.registerFileProtocol(
		Schemas.vscodeFileResource, 
		(request, callback) => this.handleResourceRequest(request, callback)
	);

	// Block any file:// access
	defaultSession.protocol.interceptFileProtocol(
		Schemas.file, 
		(request, callback) => this.handleFileRequest(request, callback)
	);

	// Cleanup
	this._register(toDisposable(() => {
		defaultSession.protocol.unregisterProtocol(Schemas.vscodeFileResource);
		defaultSession.protocol.uninterceptProtocol(Schemas.file);
	}));
}
```

**Handler implementation:**
```typescript
private handleResourceRequest(request: Electron.ProtocolRequest, callback: ProtocolCallback): void {
	const path = this.requestToNormalizedFilePath(request);
	const pathBasename = basename(path);

	let headers: Record<string, string> | undefined;
	if (this.environmentService.crossOriginIsolated) {
		if (pathBasename === 'workbench.html' || pathBasename === 'workbench-dev.html') {
			headers = COI.CoopAndCoep;
		} else {
			headers = COI.getHeadersFromQuery(request.url);
		}
	}

	// Check by valid roots first
	if (this.validRoots.findSubstr(path)) {
		return callback({ path, headers });
	}

	// Then check by valid extensions
	if (this.validExtensions.has(extname(path).toLowerCase())) {
		return callback({ path, headers });
	}

	// Finally block to load the resource
	this.logService.error(`Refused to load resource ${path}`);
	return callback({ error: -3 /* ABORTED */ });
}
```

**Variations / call-sites:**
- Resource validation via allowlist roots and file extensions
- Cross-Origin-Isolation (COI) header injection
- Cleanup via disposal pattern


#### Pattern: Managed Remote Resource Protocol Handler
**Where:** `src/vs/code/electron-main/app.ts:698-710`
**What:** Register buffer protocol for remote resources with IPC channel dispatch.

```typescript
protocol.registerBufferProtocol(Schemas.vscodeManagedRemoteResource, (request, callback) => {
	const url = URI.parse(request.url);
	if (!url.authority.startsWith('window:')) {
		return callback(notFound());
	}

	remoteResourceChannel.value.call<NodeRemoteResourceResponse>(
		NODE_REMOTE_RESOURCE_IPC_METHOD_NAME, 
		[url]
	).then(
		r => callback({ ...r, data: Buffer.from(r.body, 'base64') }),
		err => {
			this.logService.warn('error dispatching remote resource call', err);
			callback({ statusCode: 500, data: String(err) });
		}
	);
});
```

**Key aspects:**
- Buffer-based response (base64 encoded body)
- Authority validation (window: scheme)
- Lazy IPC channel initialization
- Error handling with 500 status fallback


#### Pattern: HTTP Protocol Redirect Handler
**Where:** `src/vs/code/electron-main/app.ts:1537-1542`
**What:** Redirect custom protocol to HTTP for remote resource access.

```typescript
protocol.registerHttpProtocol(Schemas.vscodeRemoteResource, (request, callback) => {
	callback({
		url: request.url.replace(/^vscode-remote-resource:/, 'http:'),
		method: request.method
	});
});
```

**Key aspects:**
- Simple URL rewrite pattern
- Preserves HTTP method
- Decouples remote URL scheme from implementation


## Protocol URL Resolution & Routing

#### Pattern: Initial Protocol URL Collection
**Where:** `src/vs/code/electron-main/app.ts:713-734`
**What:** Collect protocol URLs from both macOS events and Windows CLI args.

```typescript
private async resolveInitialProtocolUrls(
	windowsMainService: IWindowsMainService, 
	dialogMainService: IDialogMainService
): Promise<IInitialProtocolUrls | undefined> {

	// Windows/Linux: protocol handler invokes CLI with --open-url
	const protocolUrlsFromCommandLine = this.environmentMainService.args['open-url'] 
		? this.environmentMainService.args._urls || [] 
		: [];

	// macOS: open-url events that were received before the app is ready
	const protocolUrlsFromEvent = (
		(global as { getOpenUrls?: () => string[] }).getOpenUrls?.() || []
	);

	if (protocolUrlsFromCommandLine.length + protocolUrlsFromEvent.length === 0) {
		return undefined;
	}

	const protocolUrls = [
		...protocolUrlsFromCommandLine,
		...protocolUrlsFromEvent
	].map(url => {
		try {
			return { uri: URI.parse(url), originalUrl: url };
		} catch {
			this.logService.trace('app#resolveInitialProtocolUrls() protocol url failed to parse:', url);
			return undefined;
		}
	});
	// ... continue with routing logic
}
```

**Variations / call-sites:**
- Windows/Linux route: command-line `--open-url` flag with `_urls` array
- macOS route: buffered events from pre-ready listener
- Parse validation with error handling


#### Pattern: Security-Gated URL Routing
**Where:** `src/vs/code/electron-main/app.ts:752-781`
**What:** Route URLs to window opens or IPC handlers with confirmation dialog for external protocol requests.

```typescript
for (const protocolUrl of protocolUrls) {
	if (!protocolUrl) {
		continue; // invalid
	}

	const windowOpenable = this.getWindowOpenableFromProtocolUrl(protocolUrl.uri);
	if (windowOpenable) {
		if ((process as INodeProcess).isEmbeddedApp) {
			continue; // Agents app: skip all window openables
		}

		if (await this.shouldBlockOpenable(windowOpenable, windowsMainService, dialogMainService)) {
			continue; // blocked
		} else {
			openables.push(windowOpenable); // handled as window to open
		}
	} else {
		urls.push(protocolUrl); // handled within active window
	}
}

return { urls, openables };
```

**Security dialog example:**
```typescript
private async shouldBlockOpenable(
	openable: IWindowOpenable, 
	windowsMainService: IWindowsMainService, 
	dialogMainService: IDialogMainService
): Promise<boolean> {
	// ... extract message and URI ...

	if (openableUri.scheme !== Schemas.file && 
		openableUri.scheme !== Schemas.vscodeRemote) {
		return false; // Only confirm for file: and vscode-remote: schemes
	}

	const askForConfirmation = this.configurationService.getValue<unknown>(
		CodeApplication.SECURITY_PROTOCOL_HANDLING_CONFIRMATION_SETTING_KEY[openableUri.scheme]
	);
	
	if (askForConfirmation === false) {
		return false; // not blocked via settings
	}

	const { response, checkboxChecked } = await dialogMainService.showMessageBox({
		type: 'warning',
		buttons: [localize('open', '&&Yes'), localize('cancel', '&&No')],
		message,
		detail: localize('confirmOpenDetail', 
			"If you did not initiate this request, it may represent an attempted attack on your system..."),
		checkboxLabel: openableUri.scheme === Schemas.file 
			? localize('doNotAskAgainLocal', "Allow opening local paths without asking")
			: localize('doNotAskAgainRemote', "Allow opening remote paths without asking"),
		cancelId: 1
	});
	// ...
}
```

**Key aspects:**
- Two-phase routing (window openables vs. IPC protocol URLs)
- Embedded app (agent) bypass for window operations
- User confirmation for file:// and vscode-remote: URLs
- Persistent "remember choice" checkbox


---

## Summary

The VS Code Electron entry point follows a **pre-ready setup → ready event → async startup** pattern with several critical design decisions:

1. **Pre-Ready Initialization** (main.ts): Configuration of sandbox, paths, and custom protocol schemes must occur before `app.ready`.

2. **Early Listener Registration** (macOS): File/URL events are buffered pre-ready and retrieved post-startup via global functions.

3. **Security-First Web Contents**: The `web-contents-created` handler is the single enforcement point for all renderer security policies (no in-app navigation, approved window URLs only).

4. **Platform-Specific Lifecycle**: macOS dock activation, file drops, and tab creation have dedicated handlers with batching/debouncing for multi-file opens.

5. **Protocol Layer Abstraction**: Three protocol types (file, buffer, HTTP) decouple scheme names from handlers, enabling transparent upgrades without changing URL formats.

6. **Protocol URL Routing with Consent**: Initial protocol URLs flow through security-gated routing with confirmation dialogs, separating window-open requests from IPC-routed protocol handlers.

This architecture centralizes security policy, supports cross-platform compatibility, and defers significant initialization work until after the ready event for optimal startup performance.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
