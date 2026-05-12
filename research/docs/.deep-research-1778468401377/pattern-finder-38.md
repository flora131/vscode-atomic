# Electron-Coupled Patterns in VS Code Main Process

Analysis of `src/main.ts` (741 LOC) - Patterns that a Tauri/Rust port would require replacing.

---

## Pattern 1: App Lifecycle Initialization

**Where:** `src/main.ts:43-54`
**What:** Sandbox security configuration tied to Electron's app object and commandLine API.

```typescript
if (args['sandbox'] &&
	!args['disable-chromium-sandbox'] &&
	!argvConfig['disable-chromium-sandbox']) {
	app.enableSandbox();
} else if (app.commandLine.hasSwitch('no-sandbox') &&
	!app.commandLine.hasSwitch('disable-gpu-sandbox')) {
	app.commandLine.appendSwitch('disable-gpu-sandbox');
} else {
	app.commandLine.appendSwitch('no-sandbox');
	app.commandLine.appendSwitch('disable-gpu-sandbox');
}
```

**Variations / call-sites:**
- `app.enableSandbox()` line 46
- `app.commandLine.hasSwitch()` lines 47-48
- `app.commandLine.appendSwitch()` lines 50, 52-53
- Similar `appendSwitch` calls throughout lines 143, 269, 279, 281, 328, 329, 335, 342, 347, 353, 357

---

## Pattern 2: Chromium Feature Flags Configuration

**Where:** `src/main.ts:262-284`
**What:** Dynamic Chromium feature flag management via app.commandLine API, configuring GPU acceleration, color profiles, and platform-specific features.

```typescript
// Append Electron flags to Electron
if (SUPPORTED_ELECTRON_SWITCHES.indexOf(argvKey) !== -1) {
	if (argvValue === true || argvValue === 'true') {
		if (argvKey === 'disable-hardware-acceleration') {
			app.disableHardwareAcceleration(); // needs to be called explicitly
		} else {
			app.commandLine.appendSwitch(argvKey);
		}
	} else if (typeof argvValue === 'string' && argvValue) {
		if (argvKey === 'password-store') {
			let migratedArgvValue = argvValue;
			if (argvValue === 'gnome' || argvValue === 'gnome-keyring') {
				migratedArgvValue = 'gnome-libsecret';
			}
			app.commandLine.appendSwitch(argvKey, migratedArgvValue);
		} else {
			app.commandLine.appendSwitch(argvKey, argvValue);
		}
	}
}
```

**Variations / call-sites:**
- Feature flag aggregation at lines 327-335 (enable-features, disable-features, disable-blink-features)
- `getSwitchValue()` calls lines 328, 334, 341

---

## Pattern 3: Lifecycle Event Listeners (App Ready)

**Where:** `src/main.ts:147-183`
**What:** Electron app 'ready' event handler that gates initialization of tracing and main bundle loading.

```typescript
app.once('ready', function () {
	if (args['trace']) {
		let traceOptions: Electron.TraceConfig | Electron.TraceCategoriesAndOptions;
		if (args['trace-memory-infra']) {
			const customCategories = args['trace-category-filter']?.split(',') || [];
			customCategories.push('disabled-by-default-memory-infra', 'disabled-by-default-memory-infra.v8.code_stats');
			traceOptions = {
				included_categories: customCategories,
				excluded_categories: ['*'],
				memory_dump_config: {
					allowed_dump_modes: ['light', 'detailed'],
					triggers: [
						{
							type: 'periodic_interval',
							mode: 'detailed',
							min_time_between_dumps_ms: 10000
						},
						{
							type: 'periodic_interval',
							mode: 'light',
							min_time_between_dumps_ms: 1000
						}
					]
				}
			};
		} else {
			traceOptions = {
				categoryFilter: args['trace-category-filter'] || '*',
				traceOptions: args['trace-options'] || 'record-until-full,enable-sampling'
			};
		}

		contentTracing.startRecording(traceOptions).finally(() => onReady());
	} else {
		onReady();
	}
});
```

**Variations / call-sites:**
- `contentTracing.startRecording()` line 179 (Chromium-specific tracing API)

---

## Pattern 4: Custom Protocol Scheme Registration

**Where:** `src/main.ts:96-105`
**What:** Electron protocol API registering vscode-webview and vscode-file custom schemes with security privileges.

```typescript
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
```

**Variations / call-sites:**
- Called once at module initialization before app 'ready'

---

## Pattern 5: macOS-Specific File/URL Event Listeners

**Where:** `src/main.ts:589-621`
**What:** Electron app event handlers for macOS drop-file and open-url events, with globalThis callbacks to expose events to renderer.

```typescript
function registerListeners(): void {
	const macOpenFiles: string[] = [];
	(globalThis as { macOpenFiles?: string[] }).macOpenFiles = macOpenFiles;
	app.on('open-file', function (event, path) {
		macOpenFiles.push(path);
	});

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
- `app.on('open-file')` line 597
- `app.on('will-finish-launching')` line 612
- `app.on('open-url')` line 613
- `app.removeListener()` line 617

---

## Pattern 6: Crash Reporter Initialization

**Where:** `src/main.ts:527-539`
**What:** Electron crashReporter.start() configuration with product metadata, AppCenter integration, and multi-platform submission URLs.

```typescript
const productName = (product.crashReporter ? product.crashReporter.productName : undefined) || product.nameShort;
const companyName = (product.crashReporter ? product.crashReporter.companyName : undefined) || 'Microsoft';
const uploadToServer = Boolean(!process.env['VSCODE_DEV'] && submitURL && !crashReporterDirectory);
crashReporter.start({
	companyName,
	productName: process.env['VSCODE_DEV'] ? `${productName} Dev` : productName,
	submitURL,
	uploadToServer,
	compress: true,
	ignoreSystemCrashHandler: true
});
```

**Variations / call-sites:**
- Full `configureCrashReporter()` function at lines 449-539
- Platform-specific submitURL construction at lines 485-510 (win32, darwin, linux, x64/arm64 variants)

---

## Pattern 7: Path and Menu Management

**Where:** `src/main.ts:64, 70, 92`
**What:** Electron app.setPath() and Menu API for system path configuration and menu management.

```typescript
app.setPath('userData', userDataPath);  // Line 64
Menu.setApplicationMenu(null);          // Line 70
app.setAppLogsPath(path.join(userDataPath, 'logs'));  // Line 92
app.setPath('crashDumps', crashReporterDirectory);    // Line 472
```

**Variations / call-sites:**
- `app.getPreferredSystemLanguages()` line 121 (Electron locale API)
- `app.getLocale()` line 700 (post-ready locale query)
- `app.commandLine.appendSwitch('lang', ...)` line 143

---

## Summary

The main process contains **7 core Electron-coupled patterns** that would require replacement in a Tauri/Rust port:

1. **App lifecycle security** (sandbox, GPU configuration)
2. **Chromium feature flags** (hardware acceleration, color profiles, portal versions, WebGL limits)
3. **App ready-event gating** (initialization sequencing, tracing startup)
4. **Custom protocol schemes** (vscode-webview, vscode-file with CORS/fetch privileges)
5. **Platform-specific IPC events** (macOS file drops, URL schemes, listener cleanup)
6. **Crash reporting** (multi-platform AppCenter integration with architecture-specific URLs)
7. **System path/menu management** (userData, logs, crashDumps directories, application menu)

These patterns are deeply intertwined with Electron's main process architecture. A Tauri/Rust port would need equivalent implementations for security configuration, IPC-based event handling, custom URI schemes, and telemetry integration.

