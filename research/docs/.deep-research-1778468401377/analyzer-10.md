### Files Analysed

- `test/smoke/src/main.ts` — smoke test orchestrator (423 LOC)
- `test/smoke/src/utils.ts` — Mocha lifecycle helpers (187 LOC)
- `test/automation/src/application.ts` — Application lifecycle facade (149 LOC)
- `test/automation/src/code.ts` — Code launcher + polling engine (414 LOC)
- `test/automation/src/playwrightDriver.ts` — Playwright-backed driver (833 LOC)
- `test/automation/src/playwrightElectron.ts` — Electron launch via Playwright (86 LOC)
- `test/automation/src/playwrightBrowser.ts` — Browser/server launch via Playwright (207 LOC)
- `test/automation/src/electron.ts` — Electron path resolution + CLI args (190 LOC)
- `test/automation/src/workbench.ts` — Workbench component facade (74 LOC)
- `test/smoke/src/areas/terminal/terminal.test.ts` — terminal test suite entry (53 LOC)
- `test/smoke/src/areas/extensions/extension-host-restart.test.ts` — ext-host lifecycle contract (219 LOC)
- `test/sanity/src/uiTest.ts` — sanity UI test base class (207 LOC)
- `test/sanity/src/main.ts` — sanity orchestrator (57 LOC)
- `test/integration/browser/src/index.ts` — browser integration runner (257 LOC)
- `src/vs/workbench/services/driver/common/driver.ts` — IWindowDriver contract (49 LOC) [out-of-partition]

---

### Per-File Notes

#### `test/smoke/src/main.ts`
- **Role:** Top-level Mocha entrypoint that parses CLI flags, sets up test data (git clone of `vscode-smoketest-express`), optionally downloads a stable VS Code build for migration tests, and then registers all suite `setup()` functions under a single `describe` block. Chooses Electron vs. Web codepath at lines 191–242.
- **Key symbols:** `setup()` (`main.ts:344`), `ensureStableCode()` (`main.ts:274`), `before()` hook (`main.ts:368`), `ApplicationOptions` assembled at (`main.ts:371–389`), `describe('VSCode Smoke Tests')` (`main.ts:406`), `parseQuality()` (`main.ts:167`)
- **Control flow:** Process args → set `logsRootPath`/`crashesRootPath` → if not web: find `electronPath` via `getBuildElectronPath` or `getDevElectronPath` → Mocha `before`: call `setup()` which calls `ensureStableCode()` + `setupRepository()` → register suite `setup` fns → Mocha runs suites → Mocha `after`: `rimraf(testDataPath)`.
- **Data flow:** `ApplicationOptions` object (`main.ts:371`) flows into `this.defaultOptions` on the Mocha context at `main.ts:391`, then propagates into each suite via `installAppBeforeHandler` in `utils.ts`. `opts['stable-build']` path and `opts['stable-version']` are injected into the data-loss suite deferred closure at `main.ts:407`.
- **Dependencies:** `@vscode/test-electron` (download), `minimist`, `graceful-fs`, `../../automation` (all public exports), `node-fetch`.

---

#### `test/smoke/src/utils.ts`
- **Role:** Provides Mocha lifecycle scaffolding shared by all smoke suites: `installAllHandlers` wires diagnostics + app creation + teardown; `createApp` instantiates `Application`; `retryWithRestart` and `retry` give flakiness tolerance.
- **Key symbols:** `installAllHandlers()` (`utils.ts:22`), `installDiagnosticsHandler()` (`utils.ts:28`), `installAppBeforeHandler()` (`utils.ts:82`), `installAppAfterHandler()` (`utils.ts:95`), `createApp()` (`utils.ts:108`), `retryWithRestart()` (`utils.ts:139`), `retry()` (`utils.ts:164`), `getRandomUserDataDir()` (`utils.ts:121`)
- **Control flow:** `installAllHandlers` → `installDiagnosticsHandler` (Mocha `before`/`beforeEach`/`afterEach`) + `installAppBeforeHandler` (`before`: creates `Application`, calls `app.start()`) + `installAppAfterHandler` (`after`: calls `app.stop()`). `beforeEach` calls `app.startTracing(testTitle)`. `afterEach` calls `app.stopTracing(name, failed)`.
- **Data flow:** `this.defaultOptions` (set in `main.ts` global `before`) is read by `installAppBeforeHandler` at `utils.ts:87`. Each suite gets its own `Application` stored on `this.app`. Logs/crashes paths are scoped per suite via `suiteLogsPath()` / `suiteCrashPath()` (`utils.ts:74–80`).
- **Dependencies:** `mocha`, `../../automation` (`Application`, `ApplicationOptions`, `Logger`).

---

#### `test/automation/src/application.ts`
- **Role:** High-level lifecycle object. Owns the `Code` instance and the `Workbench` facade. Provides `start()`, `stop()`, `restart()`. Blocks startup until `.monaco-workbench` element is present in DOM.
- **Key symbols:** `Application` class (`application.ts:23`), `_start()` (`application.ts:84`), `startApplication()` (`application.ts:112`), `checkWindowReady()` (`application.ts:125`), `Quality` enum (`application.ts:11`)
- **Control flow:** `start()` → `_start()` → `startApplication()` calls `launch(options)` from `code.ts` → returns `Code` → `new Workbench(code)` → `checkWindowReady(code)`: polls `didFinishLoad()`, waits for `.monaco-workbench` element, waits for `whenWorkbenchRestored()`. If `remote=true`, additionally waits for `.statusbar-item[id="status.host"]` text to leave "Opening Remote" state (`application.ts:134–146`).
- **Data flow:** `ApplicationOptions` → `launch()` → `Code` object → `Workbench` object. `workspacePathOrFolder` threaded through `_start()` so `restart()` can reopen a different folder.
- **Dependencies:** `code.ts` (`launch`, `Code`, `LaunchOptions`), `workbench.ts` (`Workbench`), `profiler.ts` (`Profiler`), `logger.ts`.

---

#### `test/automation/src/code.ts`
- **Role:** The primary seam between the test harness and the running application. The `launch()` function forks into `launchPlaywrightBrowser` or `launchPlaywrightElectron` paths and returns a `Code` instance wrapping a `PlaywrightDriver`. `Code` exposes all polling helpers (`waitForElement`, `waitForTextContent`, etc.) and terminal helpers (`waitForTerminalBuffer`, `writeInTerminal`). All `PlaywrightDriver` calls are wrapped in a logging proxy at `code.ts:123–140`.
- **Key symbols:** `launch()` (`code.ts:89`), `Code` class (`code.ts:111`), `Code.poll()` (`code.ts:345`) — retries fn every 100ms up to 200 times (20s default), `registerInstance()` (`code.ts:48`), `teardownAll()` (`code.ts:72`), `Code.exit()` (`code.ts:176`), `Code.dispatchKeybinding()` (`code.ts:168`), `ICodeInstance` (`code.ts:42`), `Code.editContextEnabled` getter (`code.ts:143`)
- **Control flow:** `launch()` checks `options.web` flag → calls `launchPlaywrightBrowser` or `launchPlaywrightElectron` → registers the child process in `instances` Set → wraps driver in Proxy for logging → returns `Code`. Process `exit`/`SIGINT`/`SIGTERM` handlers call `teardownAll()` to kill all registered instances. `Code.exit()` calls `driver.close()`, then polls `process.kill(pid, 0)` every 500ms; forcibly kills after 10s, gives up after 20s.
- **Data flow:** `LaunchOptions` → `launchPlaywrightElectron` or `launchPlaywrightBrowser` → `{ electronProcess/serverProcess, driver: PlaywrightDriver }` → `Code(driver, logger, mainProcess, safeToKill, quality, version)`. All DOM-querying methods delegate to `this.driver.*` with the `poll()` retry loop as the reliability layer.
- **Dependencies:** `playwrightBrowser.ts`, `playwrightElectron.ts`, `playwrightDriver.ts`, `processes.ts`, `logger.ts`.

---

#### `test/automation/src/playwrightDriver.ts`
- **Role:** Concrete implementation of all driver operations using the Playwright API. Two distinct interaction modes coexist: (a) direct Playwright API calls (clicks, keyboard, screenshots, tracing, accessibility scanning via axe-core injection) and (b) `window.driver` in-page JS bridge calls (DOM queries, editor operations, terminal buffer access). Also manages CDP sessions for heap profiling.
- **Key symbols:** `PlaywrightDriver` class (`playwrightDriver.ts:47`), `getDriverHandle()` (`playwrightDriver.ts:704`) — evaluates `window.driver` as a JSHandle, `IWindowDriver` import (`playwrightDriver.ts:10`), `sendKeybinding()` (`playwrightDriver.ts:537`), `robustClick()` (`playwrightDriver.ts:592`), `clickAtStablePosition()` (`playwrightDriver.ts:627`), `runAccessibilityScan()` (`playwrightDriver.ts:723`), `assertNoAccessibilityViolations()` (`playwrightDriver.ts:777`), `startTracing()` / `stopTracing()` (`playwrightDriver.ts:361–396`), `startCDP()` (`playwrightDriver.ts:404`), `evaluate()` (CDP) (`playwrightDriver.ts:420`), `vscodeToPlaywrightKey` map (`playwrightDriver.ts:52–64`)
- **Control flow:** Methods like `getElements()`, `typeInEditor()`, `getTerminalBuffer()`, `writeInTerminal()` call `page.evaluate([driver, ...args])` against the `window.driver` handle — executing inside the renderer process. Direct Playwright methods (`click`, `keyboard.press`, `page.screenshot`) execute in the test process. `robustClick` tries `page.click()` first; on pointer-intercepted error falls back to `clickAtStablePosition()` which polls for position stability.
- **Data flow:** `PlaywrightDriver` constructor receives `playwright.Browser | playwright.ElectronApplication`, `BrowserContext`, `Page`, optional `serverProcess`, and `whenLoaded: Promise`. Tracing chunks written to `options.logsPath` as `.zip`. Screenshots to `options.logsPath` as `.png`. Web client logs fetched via `getLogs()` and written in `saveWebClientLogs()` on `close()`.
- **Dependencies:** `@playwright/test`, `playwright-core/types/protocol` (CDP types), `axe-core`, `./driver` (generated `IWindowDriver` type), `./processes`, `./logger`, `./code` (LaunchOptions).

---

#### `test/automation/src/electron.ts`
- **Role:** Resolves the Electron binary path and constructs the CLI argument list for launching VS Code under test. Platform-specific path detection for macOS app bundle, Linux binary, and Windows `.exe`.
- **Key symbols:** `resolveElectronConfiguration()` (`electron.ts:21`), `getDevElectronPath()` (`electron.ts:133`), `getBuildElectronPath()` (`electron.ts:149`), `getBuildVersion()` (`electron.ts:179`), `IElectronConfiguration` (`electron.ts:15`)
- **Control flow:** `resolveElectronConfiguration()` builds a `string[]` of CLI args starting with workspace path, appends `--skip-release-notes`, `--disable-telemetry`, `--disable-experiments`, `--no-cached-data`, `--disable-updates`, `--crash-reporter-directory`, `--logsPath`, `--user-data-dir`, `--extensions-dir`. If `remote`, replaces workspace arg with `vscode-remote://test+test/...` URI and copies `vscode-test-resolver` extension. Returns `{ electronPath, args, env }`.
- **Data flow:** `LaunchOptions` in → `IElectronConfiguration` out → consumed by `playwrightElectron.ts:launchElectron()`. The `--enable-smoke-test-driver` flag is appended by `playwrightElectron.ts:17` (not here), which is what causes VS Code to expose `window.driver`.
- **Dependencies:** `vscode-uri`, `extensions.ts` (`copyExtension`), `logger.ts`.

---

#### `test/automation/src/playwrightElectron.ts`
- **Role:** Launches VS Code as an Electron app via `playwright._electron.launch()`. Adds `--enable-smoke-test-driver` to args (line 17), which is the flag that makes VS Code install `window.driver`. Returns `{ electronProcess, driver: PlaywrightDriver }`.
- **Key symbols:** `launch()` (`playwrightElectron.ts:13`), `launchElectron()` (`playwrightElectron.ts:29`), `playwright._electron.launch()` call (`playwrightElectron.ts:33`)
- **Control flow:** `launch()` → `resolveElectronConfiguration()` → appends `--enable-smoke-test-driver` → `launchElectron()`: calls `playwrightImpl._electron.launch({ executablePath, args, env, timeout: 0 })` → waits for first window event → sets up tracing (if enabled) → installs console/error/crash event listeners → returns `{ electron, context, page: window }` → outer `launch()` wraps into `PlaywrightDriver`.
- **Data flow:** The `ChildProcess` exposed by `electron.process()` is returned as `electronProcess` for registration in `code.ts:registerInstance()`. The Playwright `Page` (first Electron window) becomes `_currentPage` in `PlaywrightDriver`.
- **Dependencies:** `@playwright/test`, `./playwrightDriver`, `./electron`, `./logger`, `child_process`.

---

#### `test/automation/src/playwrightBrowser.ts`
- **Role:** Launches a VS Code server process (via `scripts/code-server.sh` or a built binary), waits for the "Web UI available at" line on stdout to extract the endpoint URL, then launches a Playwright browser navigating to that URL. Port increments from 9000 per test run.
- **Key symbols:** `launch()` (`playwrightBrowser.ts:19`), `launchServer()` (`playwrightBrowser.ts:33`), `launchBrowser()` (`playwrightBrowser.ts:98`), `waitForEndpoint()` (`playwrightBrowser.ts:179`), `port` counter (`playwrightBrowser.ts:17`)
- **Control flow:** `launch()` → `launchServer()`: spawns `code-server.sh` with `--enable-smoke-test-driver` and `--port=N`; reads stdout until regex `/Web UI available at (.+)/` → `launchBrowser()`: `playwright[browserType].launch()` → `browser.newContext()` → `context.newPage()` → `page.goto(url)` where url encodes workspace path, `skipWelcome`, `skipReleaseNotes`, `logLevel` as `payload` param.
- **Data flow:** `serverProcess` (ChildProcess) is registered in `code.ts:registerInstance()` as `'server'` type. `pageLoadedPromise = page.waitForLoadState('load')` becomes the `whenLoaded` param to `PlaywrightDriver`, consumed by `didFinishLoad()`.
- **Dependencies:** `@playwright/test`, `child_process`, `vscode-uri`, `./playwrightDriver`, `./logger`, `./code`.

---

#### `test/automation/src/workbench.ts`
- **Role:** Aggregates all feature-area automation objects (20 components) into a single facade instantiated once per `Application`. Each component receives a `Code` instance and cross-dependencies (e.g., `Debug` receives `QuickAccess`, `Editors`, `Editor`).
- **Key symbols:** `Workbench` class (`workbench.ts:31`), constructor (`workbench.ts:53`), `Commands` interface (`workbench.ts:27`)
- **Control flow:** Pure construction — `new Workbench(code)` instantiates all 20 feature objects in order of dependency (QuickInput before QuickAccess; Editors before Editor; Editor before Debug, etc.). No async logic.
- **Data flow:** `Code` instance flows into every feature object. Cross-dependencies injected explicitly (e.g., `new Task(code, this.editor, this.editors, this.quickaccess, this.quickinput, this.terminal)` at `workbench.ts:71`).
- **Dependencies:** All feature automation modules: `activityBar`, `quickaccess`, `quickinput`, `editors`, `explorer`, `search`, `extensions`, `editor`, `scm`, `debug`, `statusbar`, `problems`, `settings`, `keybindings`, `terminal`, `notebook`, `localization`, `task`, `chat`.

---

#### `test/smoke/src/areas/terminal/terminal.test.ts`
- **Role:** Entry point for the terminal test suite. Orchestrates 8 sub-suites via `setup*()` calls, installs common `before`/`afterEach` handlers (kill all terminals after each test), and marks all sub-suites as skipped on Linux (pty host crash issue #216564).
- **Key symbols:** `setup()` export (`terminal.test.ts:17`), `installAllHandlers(logger)` (`terminal.test.ts:24`), `terminal.runCommand(TerminalCommandId.KillAll)` (`terminal.test.ts:36`), `this.retries(3)` (`terminal.test.ts:21`)
- **Control flow:** Mocha `describe('Terminal')` → `installAllHandlers` (app lifecycle) → `before`: fetch `app.workbench.terminal` → `afterEach`: kill all terminals → register 8 sub-suites with `skipSuite: process.platform === 'linux'`.
- **Dependencies:** `../../../../automation` (`Application`, `Terminal`, `TerminalCommandId`), `../../utils`, 8 sub-test modules.

---

#### `test/smoke/src/areas/extensions/extension-host-restart.test.ts`
- **Role:** Asserts behavioral contracts for extension host process lifecycle: (1) a blocked ext-host is killed on window reload; (2) an ext-host that is not blocked gets time to gracefully deactivate; (3) a blocked ext-host is killed when "Restart Extension Host" command is issued. Uses PID files written by a bundled smoke test extension.
- **Key symbols:** `setup()` (`extension-host-restart.test.ts:16`), test at `extension-host-restart.test.ts:30`, `quickaccess.runCommand('smoketest.getExtensionHostPidAndBlock')` (`extension-host-restart.test.ts:40`), `quickaccess.runCommand('Developer: Reload Window', { keepOpen: true })` (`extension-host-restart.test.ts:57`), `code.whenWorkbenchRestored()` (`extension-host-restart.test.ts:58`), `processExists(pid)` helper (`extension-host-restart.test.ts:21`)
- **Control flow:** Run command via quick-access → poll for PID file (500ms interval, max 20 retries) → trigger window reload/restart → `whenWorkbenchRestored()` → poll `processExists(oldPid)` until gone (max 10s). Each test has a 60–90s Mocha timeout.
- **Data flow:** PID communicated from in-process extension to test runner via filesystem temp files (`vscode-ext-host-pid.txt`, `vscode-ext-host-deactivated.txt`). New PID verified via `activationPidFile` in the third test.
- **Dependencies:** `../../../../automation` (`Application`, `Logger`), `../../utils` (`installAllHandlers`, `timeout`), `fs`, `os`, `path`.

---

#### `test/sanity/src/uiTest.ts`
- **Role:** Sanity-test base class for release validation. Uses raw Playwright `Page` (not the `Code`/`PlaywrightDriver` abstraction). Exercises: dismiss welcome dialog, dismiss workspace trust dialog, create a text file via command palette, install GitHub Pull Requests extension from Marketplace. Validates file contents on disk and extension directory presence.
- **Key symbols:** `UITest` class (`uiTest.ts:15`), `run(page)` (`uiTest.ts:54`), `validate()` (`uiTest.ts:69`), `runCommand(page, command)` (`uiTest.ts:102`) — opens command palette via F1, `createTextFile()` (`uiTest.ts:115`), `installExtension()` (`uiTest.ts:147`), `verifyTextFileCreated()` (`uiTest.ts:137`), `verifyExtensionInstalled()` (`uiTest.ts:201`)
- **Control flow:** `run()` → sequential: `dismissWelcomeDialog` (locator for `.onboarding-a-close-btn`, timeout 8s) → `dismissWorkspaceTrustDialog` (locator for "Yes, I trust the authors") → `createTextFile` (F1 → "View: Show Explorer" → new file → type → F1 → "File: Save") → `installExtension` (F1 → "View: Show Extensions" → search → click Install → wait for Uninstall button). Retry install up to 3 times.
- **Data flow:** `TestContext` provides `createTempDir()` for `extensionsDir`, `workspaceDir`, `userDataDir`. Playwright `Page` is passed in from the platform-specific test (e.g., `desktop.test.ts`). File assertions read from `workspaceDir` and `extensionsDir` after `run()` completes.
- **Dependencies:** `playwright` (Page), `./context.js` (TestContext), `assert`, `fs`, `path`.

---

#### `test/integration/browser/src/index.ts`
- **Role:** Runner for browser-based unit/integration tests. Spawns a VS Code server, then launches a Playwright browser navigating to the server with `extensionDevelopmentPath` and `extensionTestsPath` URL params. Exposes `codeAutomationLog` and `codeAutomationExit` functions to the page for test result reporting.
- **Key symbols:** `runTestsInBrowser()` (`index.ts:66`), `launchServer()` (`index.ts:180`), `gotoWithRetry()` (`index.ts:147`), `codeAutomationExit` expose (`index.ts:97`), `payloadParam` construction (`index.ts:135`)
- **Control flow:** `launchServer()` → spawn `code-server.sh` → parse stdout for "Web UI available at" → `runTestsInBrowser()`: launch browser → new context + page → expose `codeAutomationLog`/`codeAutomationExit` → construct target URL with `extensionDevelopmentPath` and `extensionTestsPath` as `vscode-remote://` URIs → `gotoWithRetry()` with exponential backoff on `ECONNREFUSED`. Test results arrive via `codeAutomationExit(code, logs)` which writes logs and calls `process.exit(code)`.
- **Data flow:** Test result exit code flows from inside VS Code's renderer (via `window.codeAutomationExit()`) out to the Node.js runner process. Log files are extracted the same way.
- **Dependencies:** `@playwright/test`, `child_process`, `tree-kill`, `tmp`, `rimraf`, `vscode-uri`, `minimist`.

---

### Cross-Cutting Synthesis

The `test/` partition implements a three-layer automation stack. At the bottom is `IWindowDriver` — a nine-method JavaScript interface (`setValue`, `getElements`, `typeInEditor`, `getTerminalBuffer`, `writeInTerminal`, `whenWorkbenchRestored`, etc.) defined in `src/vs/workbench/services/driver/common/driver.ts` and copied into the automation package by a build script. VS Code exposes this interface as `window.driver` when launched with `--enable-smoke-test-driver`. The middle layer is `PlaywrightDriver` (`playwrightDriver.ts`), which proxies calls either through `page.evaluate([window.driver, ...])` (for VS Code-specific operations) or direct Playwright API calls (for input, screenshots, accessibility). The top layer is `Code` + `Application` + `Workbench`, which wrap `PlaywrightDriver` with a 200-retry polling loop, Mocha lifecycle hooks, and 20 feature-area facades.

Three launch paths exist and are selected by flags: Electron via `playwrightElectron.ts` (spawns `Code.exe` or `code`), Web via `playwrightBrowser.ts` (spawns `code-server.sh` + navigates a browser), and Remote (Electron host with `vscode-remote://` workspace URI). All three converge on the same `PlaywrightDriver` and `Code` API. For a Tauri/Rust port, the critical seam is **`playwrightElectron.ts:13`** and **`electron.ts:21`**: these are the only files that know the application is Electron. The rest of the stack — `Code`, `PlaywrightDriver`, the `IWindowDriver` bridge, and all 200+ smoke test assertions — requires only that (a) a process with the right binary exists, (b) it accepts the same CLI flags, and (c) its window exposes `window.driver`. The smoke tests behaviorally mandate: `whenWorkbenchRestored()` resolves, `.monaco-workbench` element appears in DOM, extension host process lifecycle (kill-on-reload, graceful deactivation), terminal PTY operations, and WCAG 2.1 AA accessibility compliance.

---

### Out-of-Partition References

- `src/vs/workbench/services/driver/common/driver.ts` — Defines `IWindowDriver` (the 9-method JS contract that `window.driver` must implement). The build step at `test/automation/tools/copy-driver-definition.js` extracts the `//\*START...//\*END` block and writes it as `driver.d.ts` in the automation package. Any Tauri port must expose this exact interface from its webview.
- `src/vs/workbench/services/driver/browser/driver.ts` — Browser-side implementation of `IWindowDriver` that VS Code registers as `window.driver` when `--enable-smoke-test-driver` is present.
- `test/smoke/extensions/vscode-smoketest-ext-host/` — Bundled extension whose activation writes PID files and can block or gracefully exit; copied to `extensionsPath` at `main.ts:356`. Required for extension-host-restart tests.
- `scripts/code-server.sh` / `scripts/code-server.bat` — Server launch scripts used as fallback when `VSCODE_REMOTE_SERVER_PATH` is not set, called from `playwrightBrowser.ts:75` and `integration/browser/src/index.ts:205`.
