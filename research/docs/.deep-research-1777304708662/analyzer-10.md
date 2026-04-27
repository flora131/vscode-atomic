### Files Analysed

1. `/Users/norinlavaee/vscode-atomic/test/smoke/src/main.ts`
2. `/Users/norinlavaee/vscode-atomic/test/automation/src/index.ts`
3. `/Users/norinlavaee/vscode-atomic/test/automation/src/application.ts`
4. `/Users/norinlavaee/vscode-atomic/test/automation/src/code.ts`
5. `/Users/norinlavaee/vscode-atomic/test/automation/src/workbench.ts`
6. `/Users/norinlavaee/vscode-atomic/test/automation/src/playwrightDriver.ts`
7. `/Users/norinlavaee/vscode-atomic/test/automation/src/playwrightElectron.ts`
8. `/Users/norinlavaee/vscode-atomic/test/automation/src/electron.ts`
9. `/Users/norinlavaee/vscode-atomic/test/automation/src/driver.d.ts`
10. `/Users/norinlavaee/vscode-atomic/test/integration/electron/testrunner.js`

---

### Per-File Notes (file:line)

#### `test/smoke/src/main.ts`

- **Line 12**: Imports `@vscode/test-electron` for downloading stable VS Code builds — this package is Electron-specific.
- **Lines 36–65**: CLI argument parsing exposes three mutually exclusive runtime modes: `--web` (browser), `--remote` (server+Electron client), and default (native Electron). A Tauri port would need to introduce a new mode flag (e.g., `--tauri`) or re-use the `--build` path with a Tauri executable.
- **Lines 67–95**: Log and crash directories are named `smoke-tests-electron`, `smoke-tests-browser`, or `smoke-tests-remote`. No Tauri slot exists.
- **Lines 191–217**: Electron branch locates the application via `getBuildElectronPath` / `getDevElectronPath` and fails if the path does not exist on disk. All smoke tests for Electron mode depend on this executable lookup.
- **Lines 344–392**: `ApplicationOptions` supplied to each test includes `extraArgs` which are passed as CLI flags directly to the Electron binary (line 388). These flags (`--enable-smoke-test-driver`, `--user-data-dir`, etc.) must be honoured by any replacement binary.
- **Lines 406–422**: Full suite registration. Tests skipped under `opts.web` include: `setupDataLossTests`, `setupNotebookTests`, `setupExtensionHostRestartTests`, `setupLocalizationTests`, `setupLaunchTests`, `setupChatTests`. A Tauri port running without a web mode must pass all non-web-skipped suites.

#### `test/automation/src/index.ts`

- **Lines 6–31**: Barrel re-exports every automation module. The final export (line 31) is `getDevElectronPath`, `getBuildElectronPath`, `getBuildVersion` from `./electron`, making Electron path resolution part of the public API. A Tauri port needs parallel `getDevTauriPath` / `getBuildTauriPath` exports or must replace these entirely.

#### `test/automation/src/application.ts`

- **Lines 11–17**: `Quality` enum has five values: `Dev`, `Insiders`, `Stable`, `Exploration`, `OSS`. The smoke runner uses this to gate entire test suites (e.g., extension gallery tests excluded for `Dev` and `OSS`). A Tauri build must self-report one of these quality levels via the same mechanism.
- **Lines 125–148**: `checkWindowReady` waits for three sequential conditions:
  1. `code.didFinishLoad()` — navigation committed.
  2. `code.waitForElement('.monaco-workbench')` — the root CSS class must be present in the rendered DOM.
  3. `code.whenWorkbenchRestored()` — calls `IWindowDriver.whenWorkbenchRestored()` inside the page.
  - For remote mode (line 133), an additional check polls `.statusbar-item[id="status.host"]` to verify the remote connection string no longer reads "Opening Remote". All three gates depend on DOM selectors that must be preserved by a Tauri port.
- **Lines 77–101**: `restart()` and `stop()` call `Code#exit()`, which in turn calls `driver.close()` and ultimately kills the OS process. Tauri's process model must support an equivalent orderly shutdown path.

#### `test/automation/src/driver.d.ts`

- **Lines 32–45**: `IWindowDriver` is the **core automation contract** that must be implemented inside the application window itself. Methods:
  - `setValue(selector, text)` — directly set DOM input values.
  - `isActiveElement(selector)` — query focus state.
  - `getElements(selector, recursive)` — return `IElement[]` with tag, class, textContent, attributes, children, top, left.
  - `getElementXY(selector, xoffset?, yoffset?)` — coordinate lookup for mouse simulation.
  - `typeInEditor(selector, text)` — inject text into the Monaco editor.
  - `getEditorSelection(selector)` — read `{selectionStart, selectionEnd}`.
  - `getTerminalBuffer(selector)` — read terminal lines.
  - `writeInTerminal(selector, text)` — send input to terminal.
  - `getLocaleInfo()` — return `{language, locale?}`.
  - `getLocalizedStrings()` — return `{open, close, find}`.
  - `getLogs()` — return `ILogFile[]`.
  - `whenWorkbenchRestored()` — resolve when workbench is stable.
  - All methods must be exposed on `window.driver` (see `playwrightDriver.ts:633`).

#### `test/automation/src/code.ts`

- **Lines 17–40**: `LaunchOptions` interface. Key fields: `codePath` (path to built app), `userDataDir`, `extensionsPath`, `web`, `remote`, `tracing`, `browser`, `quality`, `version {major, minor, patch}`. All are currently Electron-oriented.
- **Lines 89–109**: `launch()` function dispatches on `options.web` — browser path via `launchPlaywrightBrowser`, Electron path via `launchPlaywrightElectron`. No third branch exists for Tauri.
- **Lines 111–141**: `Code` constructor wraps the `PlaywrightDriver` in a `Proxy` that logs every driver call. `Code.driver` is the single live reference used by all test helpers.
- **Line 143–145**: `editContextEnabled` gates Monaco edit-context behaviour on `Quality.Stable` and version `< 1.101`. Version comparison is performed against the running binary's reported version.
- **Lines 168–169**: `dispatchKeybinding` calls `driver.sendKeybinding` and requires a non-trivial `accept` callback that verifies the keybinding had its expected effect — this is an explicit safety contract for all keyboard-driven tests.
- **Lines 176–233**: `exit()` sends a close signal via `driver.close()`, then polls `process.kill(pid, 0)` every 500 ms. It force-kills after 10 s and gives up after 20 s. Tauri's process must respond to the close signal within this window.
- **Lines 259–312**: High-level polling helpers (`waitForTextContent`, `waitForElements`, `waitForElement`, `waitForActiveElement`, `waitForTitle`, `waitForTypeInEditor`, `waitForTerminalBuffer`, `writeInTerminal`, `whenWorkbenchRestored`) all delegate to the driver and share the same `poll()` retry loop.
- **Lines 330–364**: `poll()` retries up to `retryCount` (default 200) times at 100 ms intervals (default 20 s total timeout). Throws `Timeout: <message> after N seconds` on failure. All test assertions ultimately go through this loop.

#### `test/automation/src/workbench.ts`

- **Lines 31–74**: `Workbench` aggregates 19 subsystem helpers, each backed by DOM selectors:
  - `quickaccess` / `quickinput` — command palette interaction.
  - `editors` / `editor` — tab management and Monaco editing.
  - `explorer` / `activitybar` / `search` / `scm` / `debug` / `statusbar` / `problems` / `settingsEditor` / `keybindingsEditor` — sidebar panels.
  - `terminal` — integrated terminal.
  - `notebook` — Jupyter-style notebooks (Electron-only).
  - `localization` / `task` / `chat` — additional workbench areas.
  - Every helper is constructed by passing the `Code` instance; all ultimately use `Code.waitFor*` and `driver.*` methods against live DOM selectors. All these selectors must survive in a Tauri port.

#### `test/automation/src/playwrightDriver.ts`

- **Lines 47–74**: `PlaywrightDriver` constructor takes `playwright.Browser | playwright.ElectronApplication` as its first argument. This union type is the only place where Electron vs. browser is distinguished at the driver layer.
- **Lines 93–98**: `getAllWindows()` branches on `'windows' in this.application` to detect `ElectronApplication`. A Tauri app wrapped via Playwright WebDriver would appear as a `Browser`; if Playwright cannot wrap Tauri at all, a custom driver implementation is required.
- **Lines 402–468**: CDP (Chrome DevTools Protocol) session management (`startCDP`, `collectGarbage`, `evaluate`, `takeHeapSnapshot`, `queryObjects`). These are used by profiler tests and memory tests. Tauri's WebView must expose a CDP endpoint (possible with WebKit on macOS, Chromium-backed WebView on Windows/Linux via `--remote-debugging-port`).
- **Lines 572–634**: Six driver methods (`setValue`, `isActiveElement`, `getElements`, `getElementXY`, `typeInEditor`, `getEditorSelection`, `getTerminalBuffer`, `writeInTerminal`, `getLocaleInfo`, `getLocalizedStrings`, `getLogs`, `whenWorkbenchRestored`) are all implemented by calling `this.page.evaluate(([driver, ...args]) => driver.<method>(...args), [await this.getDriverHandle(), ...])`. The `getDriverHandle()` (line 632) calls `this.page.evaluateHandle('window.driver')`, meaning the application **must register an `IWindowDriver` implementation on `window.driver`** to pass any automation test.
- **Lines 651–754**: `runAccessibilityScan` / `assertNoAccessibilityViolations` inject `axe-core` via `page.evaluate(axeSource)` and run WCAG 2.1 AA checks. The accessibility test suite calls `assertNoAccessibilityViolations` against selectors `.monaco-workbench`, `.activitybar`, `.sidebar`, `.statusbar`, and the chat panel. Violations cause test failures unless specifically excluded.
- **Lines 537–565**: `sendKeybinding` translates VS Code key names to Playwright key names via `vscodeToPlaywrightKey` map and dispatches via `page.keyboard.down/up`. This is fully portable to any Playwright-accessible window.

#### `test/automation/src/playwrightElectron.ts`

- **Lines 13–27**: `launch()` calls `resolveElectronConfiguration` (from `electron.ts`) and adds `--enable-smoke-test-driver` to args (line 17). This flag is how the main process activates the `IWindowDriver` registration inside the renderer. A Tauri port must honour an equivalent startup flag.
- **Lines 29–86**: `launchElectron()` uses `playwright._electron.launch({executablePath, args, env, timeout: 0})`. The `timeout: 0` means launch never times out at the Playwright level. Page-level error handlers (lines 76–83) log `pageerror`, `crash`, `close`, and HTTP ≥ 400 responses — these become part of the test log but do not by themselves fail a test.

#### `test/automation/src/electron.ts`

- **Lines 21–99**: `resolveElectronConfiguration` builds the CLI args array. Mandatory flags passed to every smoke test run:
  - `--skip-release-notes`
  - `--skip-welcome`
  - `--disable-telemetry`
  - `--disable-experiments`
  - `--no-cached-data`
  - `--disable-updates`
  - `--disable-extension=vscode.vscode-api-tests`
  - `--crash-reporter-directory=<path>`
  - `--disable-workspace-trust`
  - `--logsPath=<path>`
  - `--use-inmemory-secretstorage` (conditional, line 43)
  - `--user-data-dir=<path>` (line 48)
  - `--extensions-dir=<path>` (line 51)
  - A Tauri port must accept the same flags or the harness must be updated.
- **Lines 133–146**: `getDevElectronPath` reads `product.json` from the repo root to find `nameLong` / `nameShort` and constructs the platform-specific path. For macOS: `.build/electron/<nameLong>.app/Contents/MacOS/<nameShort>`. For Linux: `.build/electron/<applicationName>`. For Win32: `.build/electron/<nameShort>.exe`. Tauri produces different output paths and would require a parallel `getDevTauriPath` function.
- **Lines 149–177**: `getBuildElectronPath` reads `product.json` from the installed build. On macOS it also performs a version-based binary name check (line 158–162, versions ≤ 1.109.x use `Electron` as binary name). This is fully Electron-specific.

#### `test/integration/electron/testrunner.js`

- **Lines 19–31**: Creates a Mocha instance with `ui: 'tdd'` and exposes `configure(opts)` for test-specific Mocha configuration. The `MOCHA_GREP` environment variable (line 27) filters test cases by name.
- **Lines 33–54**: `run(testsRoot, clb)` globs all `**.test.js` files under `testsRoot` and adds them to Mocha. This is the entry point for the integration test layer inside the Electron renderer/node context. A Tauri port running Node.js tests in a separate process could reuse this runner unchanged; tests running inside the Tauri webview would need a browser-compatible test runner.

---

### Cross-Cutting Synthesis (≤200 words)

A Tauri port must satisfy every condition the automation harness enforces end-to-end.

**Process contract.** The binary must accept the full set of CLI flags assembled in `electron.ts:25–99` (`--skip-welcome`, `--disable-telemetry`, `--logsPath`, etc.) and must respond to `SIGTERM` / close-signal within 20 s (`code.ts:176–233`).

**In-page driver contract.** The renderer must expose a conformant `IWindowDriver` object on `window.driver` (`driver.d.ts:32–45`). Every automation method — `getElements`, `typeInEditor`, `getTerminalBuffer`, `whenWorkbenchRestored`, etc. — is invoked via `page.evaluate('window.driver')` (`playwrightDriver.ts:632`). Without this, all 19 `Workbench` subsystem helpers fail immediately.

**DOM shape contract.** `checkWindowReady` waits for `.monaco-workbench` in the DOM (`application.ts:129`). Individual tests target `.activitybar`, `.sidebar`, `.statusbar`, `.monaco-list`, `.xterm-screen`, `[id="status.host"]`, and many more. These CSS class and ID selectors must be preserved.

**Playwright accessibility.** The `window.driver` — accessible via CDP or `page.evaluate` — means the Tauri WebView must expose a Playwright-compatible debugging surface (CDP on Chromium WebView, or WebKit remote debugging). Axe-core WCAG 2.1 AA scans must report zero violations for `.monaco-workbench`, `.activitybar`, `.sidebar`, and `.statusbar`.

**Quality and version.** The binary must report a parseable `product.json` version and a recognised `VSCODE_QUALITY` so suite gating works correctly.

---

### Out-of-Partition References

- `/Users/norinlavaee/vscode-atomic/test/smoke/src/areas/workbench/data-loss.test.ts` — data-loss suite tests restart persistence; requires `app.restart()` round-trip and stable editor tab restoration.
- `/Users/norinlavaee/vscode-atomic/test/smoke/src/areas/extensions/extension-host-restart.test.ts` — verifies extension-host process lifecycle (PID change after restart, cleanup within 10 s); closely coupled to Electron process model.
- `/Users/norinlavaee/vscode-atomic/test/smoke/src/areas/accessibility/accessibility.test.ts` — full WCAG 2.1 AA scan suite; calls `driver.assertNoAccessibilityViolations` for workbench, activity bar, sidebar, status bar, and chat panel.
- `/Users/norinlavaee/vscode-atomic/test/smoke/src/utils.ts` — referenced but not read; contains `createApp`, `installAllHandlers`, `suiteLogsPath`, `suiteCrashPath` helpers used by every test area.
- `/Users/norinlavaee/vscode-atomic/test/automation/src/playwrightBrowser.ts` — not read; counterpart to `playwrightElectron.ts` for web mode. Tauri's web-facing entry point might reuse this path.
- `/Users/norinlavaee/vscode-atomic/test/automation/src/profiler.ts` — not read; `Profiler` class uses CDP heap APIs exposed in `playwrightDriver.ts:402–468`. CDP availability in Tauri's WebView determines whether heap profiling tests can run.
