# Partition 10 — `test/` : Porting VS Code Test Infrastructure from Playwright/Electron to Tauri

> Research date: 2026-04-27  
> Branch: flora131/feature/vscode-2-electric-boogaloo  
> Key files examined:
> - `test/automation/src/playwrightElectron.ts`
> - `test/automation/src/electron.ts`
> - `test/automation/src/playwrightDriver.ts`
> - `test/automation/src/playwrightBrowser.ts`
> - `test/automation/src/code.ts`
> - `test/automation/src/driver.d.ts`
> - `test/automation/out/playwrightTauri.js` (compiled bridge — already authored)
> - `test/automation/out/playwrightDriver.js` (`TauriPlaywrightDriver` — already authored)
> - `test/automation/out/code.d.ts` / `code.js` (Tauri branch already wired)
> - `test/smoke/src/main.ts`

---

## Summary

The `test/` partition is the most automation-technology-sensitive slice of the port.
VS Code's smoke-test harness today relies entirely on `playwright._electron.launch`, which exposes Electron-specific APIs (multi-window enumeration, CDP, IPC evaluation).
Tauri wraps platform WebViews — WKWebView on macOS, WebKitGTK on Linux, WebView2 on Windows — none of which are accessible through Playwright's Electron namespace.
The project's automation layer has therefore been extended with a new `playwrightTauri.ts` module (compiled to `out/playwrightTauri.js`) that bridges Tauri's WebDriver / W3C protocol into the same `PlaywrightDriver`-shaped surface, allowing the 18 workbench automation modules and the `Code` class to remain unchanged.

---

## Library Cards

---

### Card 1 — `@playwright/test` — Electron namespace (`playwright._electron`)

**Package**: `@playwright/test` (also `playwright-core`)  
**Role in VS Code tests**: Primary test runner and browser automation driver for smoke tests (`test/smoke/`) and integration tests (`test/integration/browser/`). The `_electron` sub-namespace is the specific API that boots an Electron binary and returns a `BrowserContext`.

#### How VS Code uses it today

`test/automation/src/playwrightElectron.ts` (line 33):

```typescript
const electron = await measureAndLog(() => playwrightImpl._electron.launch({
    executablePath: configuration.electronPath,
    args: configuration.args,
    recordVideo: options.videosPath ? { dir: options.videosPath, ... } : undefined,
    env: configuration.env as { [key: string]: string },
    timeout: 0
}), 'playwright-electron#launch', logger);
```

`electron.windows()[0]` retrieves the first Electron window as a `playwright.Page`.  
`window.context()` returns a `BrowserContext` — enabling tracing, network interception, and request monitoring.

The resulting `{ electron, context, page }` triple is passed into `PlaywrightDriver`'s constructor (line 25 of `playwrightElectron.ts`):

```typescript
driver: new PlaywrightDriver(electron, context, page, undefined, Promise.resolve(), options)
```

`PlaywrightDriver` in `test/automation/src/playwrightDriver.ts` holds the union type:

```typescript
constructor(
    private readonly application: playwright.Browser | playwright.ElectronApplication,
    ...
)
```

Key methods that are Electron-specific:
- `getAllWindows()` — branches on `'windows' in this.application` to call `.windows()` vs `.pages()`
- `startCDP()` / `collectGarbage()` / `evaluate()` — open a raw CDP session via `page.context().newCDPSession(page)` then send `HeapProfiler.*` and `Runtime.*` commands
- `takeHeapSnapshot()` — streams `HeapProfiler.addHeapSnapshotChunk` events over CDP

#### Electron-specific caveats (from official Playwright docs)

Source: https://playwright.dev/docs/api/class-electron

- The Electron namespace is explicitly **experimental** (`_electron` is prefixed with underscore).
- Supported Electron versions: v12.2.0+, v13.4.0+, v14+.
- Known launch timeout issue: ensure `nodeCliInspect` (FuseV1Options.EnableNodeCliInspectArguments) is not set to `false`.
- `electron.launch()` boots the process by `executablePath`; there is no analog for Tauri binaries.
- `electronApplication.evaluate()` executes code in the **main Electron process** (Node.js context) — no equivalent exists in Tauri's Rust process.
- `electronApplication.browserWindow(page)` maps a Playwright Page back to a BrowserWindow handle — inapplicable in Tauri.

#### What changes when moving to Tauri

`playwright._electron` is entirely inapplicable. The `ElectronApplication` type disappears.
`PlaywrightDriver.application` can no longer be typed as `playwright.ElectronApplication`.
The `getAllWindows()` method's `'windows' in this.application` branch is dead code.
CDP calls (`startCDP`, `collectGarbage`, `takeHeapSnapshot`, `evaluate`, `queryObjects`, `callFunctionOn`, `getProperties`, `releaseObjectGroup`) all depend on `page.context().newCDPSession()` which only works against Chromium-based backends. On macOS/Linux Tauri (WKWebView / WebKitGTK), there is no CDP socket — these must be stubbed or replaced.

---

### Card 2 — `playwright._electron.launch` — Configuration resolution (`electron.ts`)

**File**: `test/automation/src/electron.ts`  
**Role**: Resolves the Electron binary path and CLI args, building `IElectronConfiguration`.

#### Key surface

```typescript
export interface IElectronConfiguration {
    readonly electronPath: string;   // e.g. .build/electron/VSCode.app/Contents/MacOS/Electron
    readonly args: string[];
    readonly env?: NodeJS.ProcessEnv;
}
```

`getDevElectronPath()` (line 133) returns a platform-specific path inside `.build/electron/`.
`getBuildElectronPath(root)` (line 149) reads `product.json` and `package.json` from the built bundle to determine the final executable name and handles a version cutoff (1.109.x used "Electron" binary name on macOS).

#### What changes for Tauri

The entire concept of "Electron path" is replaced by a Tauri bundle path.
The compiled bridge at `test/automation/out/playwrightTauri.js` introduces `resolveTauriBinPath(options)` which:
- Checks `process.env['TAURI_BIN_PATH']` first (CI override)
- Falls back to platform-specific paths under `tauri/target/release/bundle/`

```javascript
case 'darwin':
    return join(root, 'tauri', 'target', 'release', 'bundle', 'macos',
                'vscode-atomic.app', 'Contents', 'MacOS', 'vscode-atomic');
case 'linux':
    return join(root, 'tauri', 'target', 'release', 'vscode-atomic');
case 'win32':
    return join(root, 'tauri', 'target', 'release', 'vscode-atomic.exe');
```

The CLI argument set mirrors the Electron one but passes `WEBDRIVER_PORT` and `TAURI_DISABLE_SANDBOX=1` via environment rather than flags:

```javascript
function spawnTauriProcess(binPath, args, webDriverPort, options) {
    const env = {
        ...process.env,
        WEBDRIVER_PORT: String(webDriverPort),
        TAURI_DISABLE_SANDBOX: '1'
    };
    const proc = cp.spawn(binPath, args, { env, stdio: ['ignore', 'pipe', 'pipe'] });
    ...
}
```

`buildTauriArgs()` retains the same logical flags: `--enable-smoke-test-driver`, `--skip-release-notes`, `--skip-welcome`, `--disable-telemetry`, `--disable-updates`, `--logsPath`, `--crash-reporter-directory`.

---

### Card 3 — `tauri-driver` and the W3C WebDriver Protocol

**Source**: https://v2.tauri.app/develop/tests/webdriver/  
**Install**: `cargo install tauri-driver --locked`

#### What tauri-driver is

`tauri-driver` is a cross-platform wrapper crate that sits between the test runner and the platform-native WebDriver server:

| Platform | Native driver | CDP available |
|---|---|---|
| Linux | WebKitWebDriver (via `webkit2gtk-driver` package) | No |
| Windows | Microsoft Edge Driver (must match Edge version) | Yes (via WebView2) |
| macOS | **No native WKWebView driver** | No |

From the official docs:
> "On desktop, only Windows and Linux are supported due to macOS not having a WKWebView driver tool available."

`tauri-driver` listens (default: port 4444) and proxies W3C WebDriver commands.
The `tauri:options` extension capability tells it which binary to launch:

```javascript
capabilities: [{
    'tauri:options': {
        application: '../src-tauri/target/debug/tauri-app'
    }
}]
```

#### Integration with WebDriverIO (used in `playwrightTauri.js`)

The bridge code does not call tauri-driver separately; instead it spawns the Tauri binary (which internally starts the WebDriver server on `WEBDRIVER_PORT`) and then connects via `webdriverio.remote()`:

```javascript
const capabilities = {
    'tauri:options': {
        application: '@@TAURI_APPLICATION@@'   // filled at runtime
    }
};
return wdio.remote({ hostname: '127.0.0.1', port, capabilities, logLevel: 'error' });
```

WebDriverIO is lazy-required (`require('webdriverio')` at runtime) so the module loads cleanly in environments that only run the Electron path.

#### macOS gap

macOS has no `tauri-driver` support because Apple does not provide a WKWebView remote automation socket analogous to ChromeDriver.
This means smoke tests on macOS can only run against a headless Linux/Windows CI node or use the Windows CDP fast-path.
The compiled comment notes: "tauri-driver planned but unofficial" for Linux (meaning it works via WebKitWebDriver but is not the officially sanctioned path).

---

### Card 4 — Windows CDP fast-path (`USE_WEBVIEW2_CDP`)

**File**: `test/automation/out/playwrightTauri.js` (lines 86-173)

Windows' WebView2 (Chromium-based) does expose a remote debugging port, enabling a second codepath that bypasses tauri-driver entirely and uses `playwright.chromium.connectOverCDP`:

```javascript
if (process.env['USE_WEBVIEW2_CDP'] && process.platform === 'win32') {
    return launchViaWebView2CDP(options);
}
```

The CDP path:
1. Spawns the Tauri binary with `WEBDRIVER_PORT` set (used as CDP port).
2. Polls `http://127.0.0.1:${cdpPort}/status` until ready (same `waitForDriverReady`).
3. Calls `playwrightImpl.chromium.connectOverCDP(...)`.
4. Retrieves the first context and page.
5. Wraps in `TauriPlaywrightDriver` with a `TauriCDPContext = { page, context }`.

When `cdpCtx` is present, `TauriPlaywrightDriver` delegates:
- `startTracing` / `stopTracing` to `cdpCtx.context.tracing`
- `getTitle()` to `cdpCtx.page.title()`
- `sendKeybinding()` to `cdpCtx.page.keyboard.press()`
- `evaluateScript()` to `cdpCtx.page.evaluate()`

This gives full Playwright `Page` semantics on Windows — tracing, CDP heap profiling, screenshots — at the cost of the macOS/Linux divergence.

---

### Card 5 — `TauriPlaywrightDriver` — the compatibility shim

**File**: `test/automation/out/playwrightDriver.js` (`TauriPlaywrightDriver` class)  
**Type declaration**: `test/automation/out/playwrightDriver.d.ts`

`TauriPlaywrightDriver` mirrors every public method of `PlaywrightDriver` so that `Code` and all 18 workbench automation modules (`activityBar`, `editor`, `explorer`, `terminal`, etc.) require zero changes.

#### `window.driver` bridge pattern

All `IWindowDriver` methods (`setValue`, `isActiveElement`, `getElements`, `getElementXY`, `typeInEditor`, `getEditorSelection`, `getTerminalBuffer`, `writeInTerminal`, `getLocaleInfo`, `getLocalizedStrings`, `getLogs`, `whenWorkbenchRestored`) delegate through `evaluateOnDriver`:

```javascript
async evaluateOnDriver(method, ...args) {
    const argsJson = JSON.stringify(args);
    const script = `
        (function() {
            var driver = window.driver;
            if (!driver) { throw new Error('window.driver not available'); }
            var args = ${argsJson};
            var result = driver.${method}.apply(driver, args);
            return result;
        })()
    `;
    return this.evaluateScript(script);
}
```

This is the string-based equivalent of the Playwright path that uses `page.evaluateHandle('window.driver')` and then calls `page.evaluate(([driver, ...]) => driver.method(...))`.

The VS Code workbench registers `window.driver` via `registerWindowDriver()` in `src/vs/workbench/services/driver/browser/driver.ts` (line 274):

```typescript
export function registerWindowDriver(instantiationService: IInstantiationService): void {
    Object.assign(mainWindow, { driver: instantiationService.createInstance(BrowserWindowDriver) });
}
```

This registration is triggered when `--enable-smoke-test-driver` is in argv — the same flag used in both the Electron path (`electron.ts` line 17) and the Tauri path (`buildTauriArgs` in `playwrightTauri.js`).

#### Stubs and deferred items

The following methods are explicitly stubbed with error messages on non-CDP paths:

| Method | Reason |
|---|---|
| `collectGarbage()` | Requires `HeapProfiler.collectGarbage` CDP command |
| `evaluate()` | Requires `Runtime.evaluate` CDP command |
| `startCDP()` | No-op; CDP unavailable on WKWebView |
| `startTracing()` / `stopTracing()` | No-op on non-CDP path; WKWebView has no CDP tracing |

The `getTerminalBuffer` comment notes: "Terminal buffer reads rely on CDP in the Electron path (S9.3 IN_PROGRESS)."
This is a known outstanding gap — the xterm.js canvas renderer writes to off-screen buffers that are accessible via CDP heap queries in Electron but not via plain DOM evaluation.

---

### Card 6 — `PlaywrightDriver` — shared base driver

**File**: `test/automation/src/playwrightDriver.ts`  
**Lines**: 47-762

`PlaywrightDriver` is the class used for both Electron and browser modes.
Its constructor accepts `playwright.Browser | playwright.ElectronApplication` for the `application` field.
All methods that call `this.page.*` work identically regardless of whether the page came from Electron or a real browser.

#### CDP surface (must be adapted or stubbed)

```typescript
private _cdpSession: playwright.CDPSession | undefined;

async startCDP() {
    this._cdpSession = await this.page.context().newCDPSession(this.page);
}

async collectGarbage() {
    await this._cdpSession.send('HeapProfiler.collectGarbage');
}

async evaluate(options: Protocol.Runtime.evaluateParameters) {
    return await this._cdpSession.send('Runtime.evaluate', options);
}
```

These are called from `test/smoke/` for memory leak checks and performance profiling.
On Tauri+WKWebView, `page.context().newCDPSession()` will throw because there is no CDP socket.
On Tauri+WebView2 (Windows CDP path), `TauriPlaywrightDriver.startCDP()` is a no-op and `evaluate()` throws — the `cdpCtx.page` would need to be used instead, via `cdpCtx.page.context().newCDPSession(cdpCtx.page)`.

#### Accessibility scanning (`runAccessibilityScan`)

```typescript
async runAccessibilityScan(options?: AccessibilityScanOptions): Promise<AxeResults> {
    await this.page.evaluate(axeSource);   // inject axe-core
    ...
    const results = await this.page.evaluate(([ctx, opts]) => {
        return window.axe.run(ctx, opts);
    }, [context, runOptions] as const);
    return results as AxeResults;
}
```

axe-core injection via `page.evaluate(axeSource)` works in the Electron path because Electron exposes a standard Chromium page.
On the Tauri WebDriverIO path, `browser.execute` is used instead of `page.evaluate`. `TauriPlaywrightDriver` does not expose `runAccessibilityScan` — it would need a `browser.execute(axeSource)` analog via `evaluateScript`.

---

### Card 7 — `Code` class — launch dispatcher

**File**: `test/automation/src/code.ts` (source) / `test/automation/out/code.js` (compiled)  
**Compiled type declaration**: `test/automation/out/code.d.ts`

`LaunchOptions` (declared in `code.d.ts`) includes the new `tauri?: boolean` flag (line 22):

```typescript
export interface LaunchOptions {
    ...
    readonly remote?: boolean;
    readonly web?: boolean;
    readonly tauri?: boolean;       // NEW: routes to playwrightTauri
    ...
}
```

The `launch()` function dispatches based on this flag (from `code.js`):

```javascript
// Tauri smoke tests (tauri-driver / WebDriverIO bridge)
else if (options.tauri) {
    const { tauriProcess, driver } = await measureAndLog(
        () => launch(options),       // playwrightTauri.launch()
        'launch playwright (tauri)', options.logger
    );
    const { safeToKill } = registerInstance(tauriProcess, options.logger, 'electron');
    return new Code(driver, options.logger, tauriProcess, safeToKill, options.quality, options.version);
}
// Electron smoke tests (playwright)
else {
    const { electronProcess, driver } = await measureAndLog(
        () => launchPlaywrightElectron(options),
        'launch playwright (electron)', options.logger
    );
    ...
}
```

Note: the Tauri path reuses `registerInstance` with `'electron'` as the type string — the `safeToKill` logic listens for `'calling app.quit()'` in stdout, which is an Electron-specific signal. For Tauri this listener will never fire, but the fallback 10-second SIGTERM kill still applies.

---

### Card 8 — Mocha as test framework

**Package**: `mocha` (resolved from `../../node_modules/mocha` relative to `test/smoke/`)  
**Usage**: All smoke tests in `test/smoke/src/areas/*/` use the standard Mocha BDD interface (`describe`, `it`, `before`, `after`, `beforeEach`, `afterEach`).

`test/smoke/package.json`:
```json
"scripts": {
    "mocha": "node ../node_modules/mocha/bin/mocha"
}
```

`test/smoke/src/main.ts` imports each `setup*` function and registers them as Mocha suites.

The Tauri migration has **no impact on Mocha** itself. Mocha is the runner and knows nothing about the underlying driver.
The isolation boundary is entirely between `code.ts` (which provides the `Code` instance) and the test bodies (which call methods like `code.dispatchKeybinding`, `code.waitForElements`).

WebDriverIO's own test runner option (`wdio.conf.js`) uses Mocha as its test framework (`framework: 'mocha'`) — this aligns with VS Code's existing Mocha usage, allowing the same test bodies to run under either runner.

---

### Card 9 — WebDriver endpoint readiness polling

**File**: `test/automation/out/playwrightTauri.js` (lines 136-152)

```javascript
async function waitForDriverReady(port, logger) {
    const deadline = Date.now() + DRIVER_READY_TIMEOUT_MS;   // 30 000 ms
    const http = await import('http');
    while (Date.now() < deadline) {
        const ready = await new Promise(resolve => {
            const req = http.get(
                { hostname: '127.0.0.1', port, path: '/status', timeout: 2000 },
                () => resolve(true)
            );
            req.on('error', () => resolve(false));
            req.on('timeout', () => { req.destroy(); resolve(false); });
        });
        if (ready) {
            logger.log(`[Tauri] WebDriver endpoint ready on port ${port}`);
            return;
        }
        await new Promise(r => setTimeout(r, DRIVER_POLL_INTERVAL_MS));   // 500 ms
    }
    throw new Error(`[Tauri] WebDriver endpoint on port ${port} did not become ready within 30000ms`);
}
```

This pattern replaces Playwright's own "wait for first window" event (`electron.waitForEvent('window', { timeout: 0 })`). It polls the W3C `/status` endpoint, which tauri-driver exposes once it has connected to the native WebDriver server.

Contrast with the Electron path (`playwrightElectron.ts` lines 45-47):
```typescript
let window = electron.windows()[0];
if (!window) {
    window = await measureAndLog(() => electron.waitForEvent('window', { timeout: 0 }), ...);
}
```

The Electron path is event-driven; the Tauri path is poll-based because WebDriver does not emit an async event when ready.

---

### Card 10 — `window.driver` registration and the `--enable-smoke-test-driver` flag

**Source file**: `src/vs/workbench/services/driver/browser/driver.ts` (line 274)  
**Interface**: `test/automation/src/driver.d.ts`

The `IWindowDriver` interface is the contract between the test harness and the in-page driver:

```typescript
export interface IWindowDriver {
    setValue(selector: string, text: string): Promise<void>;
    isActiveElement(selector: string): Promise<boolean>;
    getElements(selector: string, recursive: boolean): Promise<IElement[]>;
    getElementXY(selector: string, xoffset?: number, yoffset?: number): Promise<{ x: number; y: number }>;
    typeInEditor(selector: string, text: string): Promise<void>;
    getEditorSelection(selector: string): Promise<{ selectionStart: number; selectionEnd: number }>;
    getTerminalBuffer(selector: string): Promise<string[]>;
    writeInTerminal(selector: string, text: string): Promise<void>;
    getLocaleInfo(): Promise<ILocaleInfo>;
    getLocalizedStrings(): Promise<ILocalizedStrings>;
    getLogs(): Promise<ILogFile[]>;
    whenWorkbenchRestored(): Promise<void>;
}
```

This interface is **shared between the Electron and Tauri paths** without modification. The workbench registers it via `Object.assign(mainWindow, { driver: ... })` which works identically in any webview (Electron's Chromium, WebView2, or WKWebView) because it is plain DOM JavaScript.

The Tauri binary must accept `--enable-smoke-test-driver` on its command line and forward it to the workbench bootstrap — the same as the Electron binary does via `windowConfig['enable-smoke-test-driver']` (see `src/vs/code/electron-browser/workbench/workbench.ts` line 505 and `src/vs/workbench/browser/window.ts` line 331).

---

## Migration Risk Matrix

| Concern | Electron Path | Tauri macOS/Linux | Tauri Windows (CDP) | Severity |
|---|---|---|---|---|
| Launch API | `playwright._electron.launch` | `cp.spawn` + `wdio.remote` | `cp.spawn` + `playwright.chromium.connectOverCDP` | High — entire boot sequence replaced |
| Multi-window | `electron.windows()` | Not supported (single-window model) | Via `browser.contexts()[0]` | Medium |
| CDP / DevTools | `page.context().newCDPSession()` | Not available (no CDP socket) | Available via WebView2 | High — heap snapshots, GC calls broken |
| Playwright tracing | `context.tracing.start()` | Not available | Available via `cdpCtx.context.tracing` | Medium |
| Terminal buffer reads | CDP heap queries | Best-effort via `window.driver` | Best-effort | Medium |
| Accessibility (`axe`) | `page.evaluate(axeSource)` | `browser.execute(axeSource)` needed | `cdpCtx.page.evaluate(axeSource)` | Low — workaround clear |
| Mocha test bodies | Unchanged | Unchanged | Unchanged | None |
| `window.driver` bridge | `page.evaluateHandle` + typed call | String-eval via `browser.execute` | `cdpCtx.page.evaluate` | Low — functionally equivalent |
| Video recording | `playwright` `recordVideo` option | Not available (no Playwright Page) | Available via `cdpCtx` context | Low |
| Platform support | All three | Linux + Windows only (officially) | Windows only | High for macOS CI |

---

## Key File Locations

- `test/automation/src/playwrightElectron.ts` — Electron launch entry point (keep, paths to replace)
- `test/automation/src/electron.ts` — Electron binary path resolution (to be replaced by Tauri equivalent)
- `test/automation/src/playwrightDriver.ts` — Shared driver; CDP methods must be stubbed or guarded
- `test/automation/src/code.ts` — Main launch dispatcher; `tauri?: boolean` flag added in compiled output
- `test/automation/out/playwrightTauri.js` — Tauri bridge (compiled; source `.ts` not yet committed)
- `test/automation/out/playwrightDriver.js` — Contains `TauriPlaywrightDriver` (compiled extension)
- `test/smoke/src/main.ts` — Top-level smoke test runner; Mocha setup unchanged
- `src/vs/workbench/services/driver/browser/driver.ts:274` — `registerWindowDriver` (no change needed)

---

## Recommended Next Steps

1. **Commit `playwrightTauri.ts` source** — The compiled `.js` exists but the source `.ts` is missing from `test/automation/src/`. Add it so it can be maintained alongside the Electron files.

2. **Guard CDP calls in `PlaywrightDriver`** — `startCDP()`, `collectGarbage()`, `takeHeapSnapshot()`, `evaluate()` etc. should check whether a CDP session is available and throw informative errors (or no-op) when the driver is a `TauriPlaywrightDriver`. Currently, they will fail with a generic "CDP not started" error when called on the Tauri path because `_cdpSession` is always `undefined`.

3. **macOS CI strategy** — Because there is no WKWebView automation driver, macOS smoke tests must either:
   - Run the browser mode (`options.web = true`) against a `vscode-server` sidecar, or
   - Be skipped and covered by Linux/Windows-only CI jobs.

4. **Terminal buffer** — `getTerminalBuffer` in `TauriPlaywrightDriver` attempts a `window.driver` call but VS Code's `BrowserWindowDriver.getTerminalBuffer` reads from the xterm.js buffer via DOM — this likely works but needs validation that the xterm.js DOM API is accessible from `browser.execute`.

5. **`safeToKill` signal** — The Tauri process does not emit `'calling app.quit()'`. Either add a Tauri-specific signal (e.g., `'tauri:shutdown'`) to stdout, or change `registerInstance` to accept a custom signal string for the Tauri case.

6. **webdriverio devDependency** — `webdriverio` is listed as a devDependency in comments but not yet in `test/automation/package.json`. It must be added before Tauri smoke tests can run.

