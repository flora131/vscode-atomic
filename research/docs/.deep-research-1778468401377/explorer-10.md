# Partition 10 of 80 — Findings

## Scope
`test/` (130 files, 17,072 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Test Partition 10 - Behavioral Contracts & Integration Tests

## Overview
The `test/` directory (130 files, ~17K LOC) contains VS Code's integration and behavioral tests that define contractual requirements for the IDE functionality. These tests verify critical behaviors across editing, language intelligence, debugging, source control, terminal, navigation, and other core IDE features.

## Implementation

### Smoke Tests (UI Integration Tests)
Automated end-to-end tests running against live VS Code instances that verify observable behaviors:

**Core Test Areas (24 test files):**
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/accessibility/accessibility.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/chat/chatDisabled.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/extensions/extension-host-restart.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/extensions/extensions.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/languages/languages.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/multiroot/multiroot.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/notebook/notebook.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/preferences/preferences.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/search/search.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/statusbar/statusbar.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/task/task-quick-pick.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/task/task.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/terminal/terminal-editors.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/terminal/terminal-input.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/terminal/terminal-persistence.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/terminal/terminal-profiles.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/terminal/terminal-shellIntegration.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/terminal/terminal-stickyScroll.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/terminal/terminal-splitCwd.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/terminal/terminal-tabs.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/terminal/terminal.test.ts` (orchestrator)
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/workbench/data-loss.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/workbench/launch.test.ts`
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/workbench/localization.test.ts`

**Helper Files:**
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/terminal/terminal-helpers.ts` - Terminal-specific test utilities

### Sanity Tests (Release Validation)
Platform-specific release sanity tests validating builds across different architectures and installation methods:

**Desktop & Server Tests (5 test files):**
- `/home/norinlavaee/projects/vscode-atomic/test/sanity/src/desktop.test.ts` - Validates Electron builds (darwin x64/arm64, linux arm64, windows x64/arm64)
- `/home/norinlavaee/projects/vscode-atomic/test/sanity/src/server.test.ts` - Server build validation
- `/home/norinlavaee/projects/vscode-atomic/test/sanity/src/serverWeb.test.ts` - Web server build validation
- `/home/norinlavaee/projects/vscode-atomic/test/sanity/src/cli.test.ts` - CLI functionality validation
- `/home/norinlavaee/projects/vscode-atomic/test/sanity/src/devTunnel.test.ts` - Tunnel connectivity validation
- `/home/norinlavaee/projects/vscode-atomic/test/sanity/src/wsl.test.ts` - Windows Subsystem for Linux integration tests

### Unit Tests (Browser & Node)
Base functionality tests at the module level:

**Test Runners:**
- `/home/norinlavaee/projects/vscode-atomic/test/unit/browser/index.js` - Browser-based unit test runner (chromium, webkit, firefox via Playwright)
- `/home/norinlavaee/projects/vscode-atomic/test/unit/node/index.js` - Node.js runtime test runner
- `/home/norinlavaee/projects/vscode-atomic/test/unit/electron/index.js` - Electron main process test runner
- `/home/norinlavaee/projects/vscode-atomic/test/unit/electron/renderer.js` - Electron renderer test runner
- `/home/norinlavaee/projects/vscode-atomic/test/unit/electron/preload.js` - Electron preload test runner

**Test Utilities:**
- `/home/norinlavaee/projects/vscode-atomic/test/unit/reporter.js` - Mocha reporter
- `/home/norinlavaee/projects/vscode-atomic/test/unit/fullJsonStreamReporter.js` - JSON streaming reporter
- `/home/norinlavaee/projects/vscode-atomic/test/unit/analyzeSnapshot.js` - Test snapshot analysis
- `/home/norinlavaee/projects/vscode-atomic/test/unit/coverage.js` - Code coverage tracking

### Integration Tests
API-level tests for VS Code extensions and browser scenarios:

**Browser Integration (Playwright-based):**
- `/home/norinlavaee/projects/vscode-atomic/test/integration/browser/src/index.ts` - Web browser test runner supporting chromium, firefox, webkit with optional debugging

**Electron Integration:**
- `/home/norinlavaee/projects/vscode-atomic/test/integration/electron/testrunner.d.ts` - Type definitions for Electron integration runner

### Monaco Editor Tests
Isolated editor core tests:
- `/home/norinlavaee/projects/vscode-atomic/test/monaco/monaco.test.ts` - Monaco editor functionality tests

## Tests

### Test Framework Structure
Uses Mocha with TDD UI (`suite`, `describe`, `it`):
- Configuration: `/home/norinlavaee/projects/vscode-atomic/test/.mocharc.json` (ui: "tdd", timeout: 10000ms)
- Suite definition pattern: `describe('FeatureName', ...)` with nested suites
- Test execution: `setup()` functions export test structures to main runners

### Smoke Test Execution Model
Uses `@vscode/test-electron` to spawn VS Code instances and control via automation driver:
- Entry point: `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/main.ts` - Orchestrates test suite execution
- Each area's `setup(logger)` function registers its describe blocks
- Mocha handlers setup before/after for app lifecycle: start VS Code, run tests, stop, capture logs/crashes
- Runs in Electron (native) or web (with Playwright browser) modes

### Test Lifecycle Utilities
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/utils.ts` - Provides:
  - `installAllHandlers()` - Sets up Mocha before/after hooks for app startup/shutdown
  - `describeRepeat()`, `itRepeat()` - Multi-iteration test helpers
  - `installDiagnosticsHandler()` - Logs test execution and captures failures
  - `suiteLogsPath()`, `suiteCrashPath()` - Log aggregation per suite
  - Captures traces on test failures via app.startTracing/stopTracing

### Sanity Test Infrastructure
- `/home/norinlavaee/projects/vscode-atomic/test/sanity/src/main.ts` - Main test orchestrator
- `/home/norinlavaee/projects/vscode-atomic/test/sanity/src/context.ts` - Test context with platform detection
- `/home/norinlavaee/projects/vscode-atomic/test/sanity/src/uiTest.ts` - UI test base class
- `/home/norinlavaee/projects/vscode-atomic/test/sanity/src/detectors.ts` - Platform/environment detection
- `/home/norinlavaee/projects/vscode-atomic/test/sanity/src/githubAuth.ts` - GitHub authentication for CI

## Types / Interfaces

### Automation API Type Definitions
- `/home/norinlavaee/projects/vscode-atomic/test/integration/electron/testrunner.d.ts` - Electron test runner types

### Playwright Component Testing
- `/home/norinlavaee/projects/vscode-atomic/test/componentFixtures/playwright/playwright.config.ts` - Playwright configuration for component tests

## Configuration

### Mocha Configuration
- `/home/norinlavaee/projects/vscode-atomic/test/.mocharc.json` - Global Mocha TDD config (timeout: 10s)

### Smoke Test Configuration
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/tsconfig.json` - TypeScript build config (target: ES2024, strict mode, sourceMap)
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/package.json` - Build scripts: compile, watch, mocha runner

### Sanity Test Configuration
- `/home/norinlavaee/projects/vscode-atomic/test/sanity/tsconfig.json` - TypeScript config
- `/home/norinlavaee/projects/vscode-atomic/test/sanity/package.json` - Dependencies: playwright, node-fetch, minimist

### Integration Test Configuration
- `/home/norinlavaee/projects/vscode-atomic/test/integration/browser/tsconfig.json` - TypeScript config
- `/home/norinlavaee/projects/vscode-atomic/test/integration/browser/package.json` - Browser test dependencies

### Automation Package Configuration
- `/home/norinlavaee/projects/vscode-atomic/test/automation/tsconfig.json` - TypeScript config
- `/home/norinlavaee/projects/vscode-atomic/test/automation/package.json` - Shared automation dependencies

### Component Fixtures
- `/home/norinlavaee/projects/vscode-atomic/test/componentFixtures/playwright/tsconfig.json` - Component test TypeScript
- `/home/norinlavaee/projects/vscode-atomic/test/componentFixtures/playwright/package.json` - Playwright dependencies

### MCP Server Configuration
- `/home/norinlavaee/projects/vscode-atomic/test/mcp/tsconfig.json` - TypeScript config
- `/home/norinlavaee/projects/vscode-atomic/test/mcp/package.json` - MCP SDK and dependencies

### Unit Test Configuration
- `/home/norinlavaee/projects/vscode-atomic/test/unit/node/package.json` - Node test dependencies

## Examples / Fixtures

### Smoke Test Extension Fixture
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/extensions/vscode-smoketest-ext-host/extension.js` - Test extension for verifying extension host restarts (registers smoketest.getExtensionHostPidAndBlock command)
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/extensions/vscode-smoketest-ext-host/package.json` - Extension manifest

### Component Test Fixtures
- `/home/norinlavaee/projects/vscode-atomic/test/componentFixtures/component-explorer.json` - Component test metadata
- `/home/norinlavaee/projects/vscode-atomic/test/componentFixtures/component-explorer-diff.json` - Diff tracking
- `/home/norinlavaee/projects/vscode-atomic/test/componentFixtures/playwright/tests/imageCarousel.spec.ts` - Playwright component test example

### Playwright Smoke Test
- `/home/norinlavaee/projects/vscode-atomic/test/componentFixtures/blocks-ci-screenshots.md` - CI screenshot documentation

## Documentation

### Test Directories Overview
- `/home/norinlavaee/projects/vscode-atomic/test/README.md` - High-level test structure overview

### Smoke Test Documentation
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/README.md` - Detailed smoke test guide including:
  - Quick overview for different build targets (dev, build, remote, web)
  - Release endgame testing procedures
  - Debug flags: --verbose, -f PATTERN, --headless
  - Pitfalls: state sharing, singletons, focus, timing, waiting

### Automation Framework Documentation
- `/home/norinlavaee/projects/vscode-atomic/test/automation/README.md` - Describes automation package as driver connecting from separate process

### Unit Test Documentation
- `/home/norinlavaee/projects/vscode-atomic/test/unit/README.md` - Run modes:
  - Electron (./scripts/test.[sh|bat])
  - Browser (chromium, webkit, firefox via Playwright)
  - Node.js (npm run test-node)
  - Coverage reporting

### Integration Test Documentation
- `/home/norinlavaee/projects/vscode-atomic/test/integration/browser/README.md` - Browser integration setup and debug modes

### Sanity Test Documentation
- `/home/norinlavaee/projects/vscode-atomic/test/sanity/README.md` - Comprehensive release sanity validation:
  - Platform support matrix (macOS, Windows, Linux, WSL, DevTunnel)
  - Container-based testing (Alpine, CentOS, Debian, Fedora, openSUSE, Red Hat, Ubuntu)
  - Command-line options and filtering
  - Docker setup procedures
  - CI/CD pipeline integration (Azure Pipelines)

### MCP Server Documentation
- `/home/norinlavaee/projects/vscode-atomic/test/mcp/README.md` - Model Context Protocol server for AI automation:
  - Architecture: stdio.ts entry point, automation.ts MCP server, application.ts lifecycle
  - Tools provided: editor, terminal, debug, search, extensions, UI interaction
  - Project structure and development guidance

### Monaco Editor Documentation
- `/home/norinlavaee/projects/vscode-atomic/test/monaco/README.md` - Monaco editor test framework

## Notable Clusters

### Automation Framework (32 files)
Reusable testing infrastructure under `/home/norinlavaee/projects/vscode-atomic/test/automation/src/`:
- **Core:** `code.ts`, `application.ts`, `workbench.ts`, `electron.ts`, `playwrightElectron.ts`, `playwrightDriver.ts`, `playwrightBrowser.ts`
- **UI Components:** `activityBar.ts`, `editor.ts`, `editors.ts`, `explorer.ts`, `viewlet.ts`, `statusbar.ts`, `problems.ts`
- **Features:** `search.ts`, `extensions.ts`, `terminal.ts`, `debug.ts`, `chat.ts`, `notebook.ts`, `task.ts`
- **UX:** `quickaccess.ts`, `quickinput.ts`, `settings.ts`, `keybindings.ts`, `scm.ts`, `localization.ts`, `processes.ts`
- **Monitoring:** `logger.ts`, `profiler.ts`
- **Index:** `index.ts` - Main export aggregating all components

### MCP Automation Tools (22 files)
Modular automation tools under `/home/norinlavaee/projects/vscode-atomic/test/mcp/src/automationTools/`:
- Mirrors automation framework structure for MCP protocol exposure
- Includes: `core.ts`, `editor.ts`, `terminal.ts`, `debug.ts`, `search.ts`, `extensions.ts`, `notebook.ts`, `chat.ts`, `windows.ts`
- Plus: `activityBar.ts`, `explorer.ts`, `keybindings.ts`, `localization.ts`, `problems.ts`, `profiler.ts`, `quickAccess.ts`, `scm.ts`, `settings.ts`, `statusbar.ts`, `task.ts`

### Terminal Testing Subcluster (11 files)
Comprehensive terminal feature tests under `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/areas/terminal/`:
- Main orchestrator: `terminal.test.ts`
- Specialized tests: editors, input, persistence, profiles, shell integration, sticky scroll, split CWD, tabs
- Shared utilities: `terminal-helpers.ts`
- Platform-specific skips (Linux pty host crashes, Windows split CWD)

### Sanity Infrastructure (12 files)
Release validation tools under `/home/norinlavaee/projects/vscode-atomic/test/sanity/src/`:
- Test implementations: 6 platform-specific test files
- Support: context, detectors, uiTest base class, githubAuth, main orchestrator, index

### Smoke Test Entry Point
- `/home/norinlavaee/projects/vscode-atomic/test/smoke/src/main.ts` - Imports all 24+ test suites and orchestrates execution with:
  - Multi-platform support (Electron, Web, Remote)
  - Build sources (dev, stable, custom paths)
  - Browser targets (chromium, webkit, firefox, channel variants)
  - Logging to `.build/logs/smoke-tests-{electron,browser,remote}`
  - Crash collection to `.build/crashes/smoke-tests-{electron,browser,remote}`

---

**Architectural Note for Porting:** The test partition reveals VS Code's behavioral contracts across all major IDE features. A successful Tauri/Rust port must satisfy all 24 smoke test suites plus sanity tests across platforms. The automation driver model (separate process controlling UI) is well-suited for reuse: only the target application launch mechanism would change (Tauri instead of Electron), while test logic remains portable.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Test Suite Patterns - Core IDE Behavioral Contracts

## Pattern 1: Top-Level Suite Orchestration with Conditional Setup

**Where:** `test/smoke/src/main.ts:406-422`

**What:** Root test suite that conditionally registers multiple feature-area tests based on configuration (web/electron, quality level, platform).

```typescript
describe(`VSCode Smoke Tests (${opts.web ? 'Web' : 'Electron'})`, () => {
	if (!opts.web) { setupDataLossTests(() => { return { stableCodePath: opts['stable-build'], stableCodeVersion: opts['stable-version'] } /* Do not change, deferred for a reason! */; }, logger); }
	setupPreferencesTests(logger);
	setupSearchTests(logger);
	if (!opts.web) { setupNotebookTests(logger); }
	setupLanguagesTests(logger);
	setupTerminalTests(logger);
	setupTaskTests(logger);
	setupStatusbarTests(logger);
	if (quality !== Quality.Dev && quality !== Quality.OSS) { setupExtensionTests(logger); }
	if (!opts.web && !opts.remote) { setupExtensionHostRestartTests(logger); }
	if (!(opts.web && process.platform === 'win32' /* TODO@bpasero flaky */)) { setupMultirootTests(logger); }
	if (!opts.web && !opts.remote && quality !== Quality.Dev && quality !== Quality.OSS) { setupLocalizationTests(logger); }
	if (!opts.web && !opts.remote) { setupLaunchTests(logger); }
	if (!opts.web) { setupChatTests(logger); }
	setupAccessibilityTests(logger, opts, quality);
});
```

**Variations / call-sites:** Orchestrates all tests at `test/smoke/src/main.ts:406`, with conditional setup callsites for each area test module.

---

## Pattern 2: Feature Area Setup Function with Install Handlers

**Where:** `test/smoke/src/areas/extensions/extensions.test.ts:9-31`

**What:** Feature area tests use a `setup()` export function that wraps a describe() block and installs shared before/after handlers via `installAllHandlers()`.

```typescript
export function setup(logger: Logger) {
	describe('Extensions', () => {

		// Shared before/after handling
		installAllHandlers(logger, opts => {
			opts.verbose = true; // enable verbose logging for tracing
			opts.snapshots = true; // enable network tab in devtools for tracing since we install an extension
			return opts;
		});

		it.skip('install and enable vscode-smoketest-check extension', async function () {
			const app = this.app as Application;

			await app.workbench.extensions.installExtension('ms-vscode.vscode-smoketest-check', true);

			// Close extension editor because keybindings dispatch is not working when web views are opened and focused
			// https://github.com/microsoft/vscode/issues/110276
			await app.workbench.extensions.closeExtension('vscode-smoketest-check');

			await app.workbench.quickaccess.runCommand('Smoke Test Check');
		});
	});
}
```

**Variations / call-sites:** Used by `extensions.test.ts:9`, `languages.test.ts:10`, `preferences.test.ts:9`, `search.test.ts:10`, `statusbar.test.ts`, and others.

---

## Pattern 3: Shared Handler Installation with Options Transformation

**Where:** `test/smoke/src/utils.ts:22-26`

**What:** `installAllHandlers()` orchestrates three handler types (diagnostics, before, after) and accepts an optional `optionsTransform` function to mutate ApplicationOptions.

```typescript
export function installAllHandlers(logger: Logger, optionsTransform?: (opts: ApplicationOptions) => ApplicationOptions) {
	installDiagnosticsHandler(logger);
	installAppBeforeHandler(optionsTransform);
	installAppAfterHandler();
}
```

**Variations / call-sites:** Called in 14+ feature areas including `extensions.test.ts:13`, `languages.test.ts:14`, `preferences.test.ts:13`, `search.test.ts:14`, with transform customization in `launch.test.ts:14-19`.

---

## Pattern 4: Before/After Test Lifecycle with Application Instance

**Where:** `test/smoke/src/utils.ts:82-106`

**What:** `installAppBeforeHandler()` and `installAppAfterHandler()` manage Application lifecycle, creating fresh instances per suite with log paths tied to test names.

```typescript
function installAppBeforeHandler(optionsTransform?: (opts: ApplicationOptions) => ApplicationOptions) {
	before(async function () {
		const suiteName = this.test?.parent?.title ?? 'unknown';

		this.app = createApp({
			...this.defaultOptions,
			logsPath: suiteLogsPath(this.defaultOptions, suiteName),
			crashesPath: suiteCrashPath(this.defaultOptions, suiteName)
		}, optionsTransform);
		await this.app.start();
	});
}

export function installAppAfterHandler(appFn?: () => Application | undefined, joinFn?: () => Promise<unknown>) {
	after(async function () {
		const app: Application = appFn?.() ?? this.app;
		if (app) {
			await app.stop();
		}

		if (joinFn) {
			await joinFn();
		}
	});
}
```

**Variations / call-sites:** Base pattern used in all features; variants in `data-loss.test.ts:19-20` with custom appFn and joinFn.

---

## Pattern 5: Nested Describe with Platform/Condition-Based Skip

**Where:** `test/smoke/src/areas/terminal/terminal-editors.test.ts:9-81`

**What:** Nested describe blocks use conditional skip via `describe.skip` ternary based on options, with per-test setup/teardown for stateful operations.

```typescript
export function setup(options?: { skipSuite: boolean }) {
	(options?.skipSuite ? describe.skip : describe)('Terminal Editors', () => {
		let app: Application;
		let terminal: Terminal;
		let settingsEditor: SettingsEditor;

		// Acquire automation API
		before(async function () {
			app = this.app as Application;
			terminal = app.workbench.terminal;
			settingsEditor = app.workbench.settingsEditor;
			await setTerminalTestSettings(app);
		});

		after(async function () {
			await settingsEditor.clearUserSettings();
		});

		it('should update color of the tab', async () => {
			// Test body...
		});
	});
}
```

**Variations / call-sites:** Used in terminal sub-tests: `terminal-editors.test.ts:9`, `terminal-input.test.ts`, `terminal-persistence.test.ts`, `terminal-profiles.test.ts`, `terminal-tabs.test.ts`.

---

## Pattern 6: Multi-Suite Nested Describe with Conditional Blocks

**Where:** `test/smoke/src/areas/accessibility/accessibility.test.ts:9-150`

**What:** Deep-nested describe blocks with conditional logic (if statements) to include/exclude sub-suites based on runtime options (web mode, quality level).

```typescript
export function setup(logger: Logger, opts: { web?: boolean }, quality: Quality) {
	describe('Accessibility', function () {

		// Increase timeout for accessibility scans
		this.timeout(2 * 60 * 1000);

		// Retry tests to minimize flakiness
		this.retries(2);

		// Shared before/after handling
		installAllHandlers(logger);

		let app: Application;

		before(async function () {
			app = this.app as Application;
		});

		describe('Workbench', function () {

			(opts.web ? it.skip : it)('workbench has no accessibility violations', async function () {
				// Test body...
			});
		});

		// Chat is not available in web mode
		if (!opts.web) {
			describe('Chat', function () {
				it('chat panel has no accessibility violations', async function () {
					// Test body...
				});
			});
		}
	});
}
```

**Variations / call-sites:** Accessibility test suite at `accessibility.test.ts:9` with nested Workbench and Chat sub-suites, each with conditional skip/include logic.

---

## Pattern 7: Diagnostics Handler with Tracing and Logging

**Where:** `test/smoke/src/utils.ts:28-69`

**What:** `installDiagnosticsHandler()` wraps every test with before/afterEach hooks for logging, tracing, and failure detection across the test lifecycle.

```typescript
export function installDiagnosticsHandler(logger: Logger, appFn?: () => Application | undefined) {

	// Before each suite
	before(async function () {
		const suiteTitle = this.currentTest?.parent?.title;
		logger.log('');
		logger.log(`>>> Suite start: '${suiteTitle ?? 'unknown'}' <<<`);
		logger.log('');
	});

	// Before each test
	beforeEach(async function () {
		const testTitle = this.currentTest?.title;
		logger.log('');
		logger.log(`>>> Test start: '${testTitle ?? 'unknown'}' <<<`);
		logger.log('');

		const app: Application = appFn?.() ?? this.app;
		await app?.startTracing(testTitle ?? 'unknown');
	});

	// After each test
	afterEach(async function () {
		const currentTest = this.currentTest;
		if (!currentTest) {
			return;
		}

		const failed = currentTest.state === 'failed';
		const testTitle = currentTest.title;
		logger.log('');
		if (failed) {
			logger.log(`>>> !!! FAILURE !!! Test end: '${testTitle}' !!! FAILURE !!! <<<`);
		} else {
			logger.log(`>>> Test end: '${testTitle}' <<<`);
		}
		logger.log('');

		const app: Application = appFn?.() ?? this.app;
		await app?.stopTracing(testTitle.replace(/[^a-z0-9\-]/ig, '_'), failed);
	});
}
```

**Variations / call-sites:** Instantiated in `utils.ts:23` via `installAllHandlers()`, with custom appFn in `data-loss.test.ts:19`, `search.test.ts`, and others.

---

## Pattern 8: Complex Multi-Startup Data Migration Test

**Where:** `test/smoke/src/areas/workbench/data-loss.test.ts:10-135`

**What:** Tests involving two VS Code startup cycles use nested describe blocks, shared handlers with custom callbacks, and suite-scoped Application instances.

```typescript
export function setup(ensureStableCode: () => { stableCodePath: string | undefined; stableCodeVersion: { major: number; minor: number; patch: number } | undefined }, logger: Logger) {
	describe('Data Loss (insiders -> insiders)', function () {

		// Double the timeout since these tests involve 2 startups
		this.timeout(4 * 60 * 1000);

		let app: Application | undefined = undefined;

		// Shared before/after handling
		installDiagnosticsHandler(logger, () => app);
		installAppAfterHandler(() => app);

		it('verifies opened editors are restored', async function () {
			app = createApp({
				...this.defaultOptions,
				logsPath: suiteLogsPath(this.defaultOptions, 'test_verifies_opened_editors_are_restored'),
				crashesPath: suiteCrashPath(this.defaultOptions, 'test_verifies_opened_editors_are_restored')
			});
			await app.start();

			// Open 3 editors
			await app.workbench.quickaccess.openFile(join(app.workspacePathOrFolder, 'bin', 'www'));
			await app.workbench.quickaccess.runCommand('View: Keep Editor');
			await app.workbench.quickaccess.openFile(join(app.workspacePathOrFolder, 'app.js'));
			await app.workbench.quickaccess.runCommand('View: Keep Editor');
			await app.workbench.editors.newUntitledFile();

			await app.restart();

			// Verify 3 editors are open
			await app.workbench.editors.selectTab('Untitled-1');
			await app.workbench.editors.selectTab('app.js');
			await app.workbench.editors.selectTab('www');

			await app.stop();
			app = undefined;
		});
	});
}
```

**Variations / call-sites:** Data loss test at `data-loss.test.ts:10-135`, with stable->insiders variant at line 137+ for version migration tests.

---

## Summary of Test Contracts

The test suite documents these core behavioral contracts VS Code must preserve:

1. **Application Lifecycle**: Launch, restart, shutdown with state preservation
2. **Editor State Persistence**: Open editors, unsaved changes, hot exit
3. **Language Intelligence**: Quick outline, problem detection, diagnostics
4. **Terminal Integration**: Creation, configuration, multiple instances, shell integration
5. **Search & Navigation**: File search, quick open, outline
6. **Task Execution**: Task running, output capture
7. **Extensions**: Installation, activation, host restart
8. **Accessibility**: WCAG violations, semantic HTML
9. **Settings & Preferences**: User settings, keybindings, live changes
10. **Notebooks**: Cell execution, state management
11. **Source Control**: Git operations, workspace management
12. **Multi-Version Migration**: Stable to insiders upgrade paths

All tests use the Application automation API (`test/automation/src/`) to exercise UI interactions and verify outcomes.

## External References
<!-- Source: codebase-online-researcher sub-agent -->
# Partition 10 — `test/` — Online Research: Testing Framework Portability

## Executive Summary

External testing-framework documentation is **central** to the porting decision for `test/`. The test suite is built around three interlocking frameworks — `@playwright/test` (via `playwright._electron`), a custom `window.driver` IPC bridge, and `@vscode/test-electron` for artifact download — and each has materially different status in a Tauri world. The biggest finding is that `playwright._electron` has **no Tauri equivalent**, meaning the entire `test/automation/` harness must be rebuilt on a different protocol (W3C WebDriver via `tauri-driver`, or a community alternative). The `window.driver` bridge is actually the most portable component because it is a plain JS object; its interface can be re-exposed under Tauri. Mocha itself is unaffected by the runtime change.

---

## Partition Dependencies (from package.json files)

| Sub-package | Key runtime deps |
|---|---|
| `test/automation/` | `@playwright/test` (devDep via root), `axe-core ^4.10.2`, `tree-kill 1.2.2`, `vscode-uri 3.0.2` |
| `test/smoke/` | `@vscode/test-electron ^2.4.0` (root), `node-fetch ^2.6.7`, Mocha (root) |
| `test/sanity/` | `playwright ^1.57.0`, `mocha ^11.7.5` |
| `test/componentFixtures/playwright/` | `@playwright/test ^1.52.0` |
| `test/unit/electron/` | `electron` directly (`require('electron')` at the top of `index.js`) |
| `test/integration/browser/` | No Playwright/Electron; spawns a server and opens a browser page |
| `test/mcp/` | `@modelcontextprotocol/sdk 1.26.0` |

---

## Library Research

#### @playwright/test + playwright._electron (v1.52 / v1.57)
**Docs:** https://playwright.dev/docs/api/class-electron

**Relevant behaviour:**
Playwright exposes an experimental `_electron` namespace that can launch an Electron binary, connect to it via Chromium DevTools Protocol (CDP), and return an `ElectronApplication` object with typed `windows()` / `context()` / `evaluate()` APIs. The `test/automation/src/playwrightElectron.ts` module calls `playwrightImpl._electron.launch({ executablePath, args, env })` and wraps the resulting window as a standard Playwright `Page`. All subsequent E2E interaction (`click`, `keyboard`, `evaluate`, `tracing`, screenshots, `CDPSession`) flows through the same `PlaywrightDriver` used for the browser smoke-test path.

**Where used / relevance to port:**
`playwright._electron` is the **only** mechanism that launches and attaches to VS Code in the Electron smoke tests. There is no `playwright._tauri` equivalent. On a Tauri port, this entire launch path must be replaced. Three options exist:

1. **tauri-driver + WebdriverIO or Selenium** — the officially supported route (W3C WebDriver). `tauri-driver` spawns as a sidecar process; tests connect via `http://127.0.0.1:4444` with `{ 'tauri:options': { application }, browserName: 'wry' }` capabilities. This requires rewriting `playwrightElectron.ts` entirely and replacing all Playwright `Page` API calls with WebdriverIO/Selenium equivalents.

2. **Playwright + WebKit** — Playwright can drive a Tauri dev-server via its standard browser mode (not `_electron`). This is what `test/componentFixtures/playwright/` already does. However, it only works against the web layer; it cannot launch or lifecycle-manage the Tauri binary, and it does not validate IPC / native features. This maps well to the existing `playwrightBrowser.ts` path (web smoke tests).

3. **tauri-pilot (community)** — a Rust CLI that talks to the app over a Unix socket using the accessibility tree, requiring a plugin in the Tauri binary. Simpler setup than WebDriver; no binary version-pinning needed.

The critical architectural loss is Playwright's `CDPSession` — `playwrightDriver.ts` uses raw CDP (`HeapProfiler.collectGarbage`, `Runtime.queryObjects`, heap snapshot) for the profiler smoke tests. This capability is completely absent from `tauri-driver` (which proxies WebKitWebDriver / msedgedriver, not Chromium DevTools).

---

#### tauri-driver (pre-alpha, cargo-installed)
**Docs:** https://v2.tauri.app/develop/tests/webdriver

**Relevant behaviour:**
`tauri-driver` is a cross-platform W3C WebDriver wrapper that proxies commands to the platform's native WebDriver server: `WebKitWebDriver` on Linux, `msedgedriver` on Windows. It is installed via `cargo install tauri-driver --locked` and listens on `http://127.0.0.1:4444`. It is labeled **pre-alpha** in official documentation.

Platform constraints:
- **Linux**: Works via `WebKitWebDriver` (`webkit2gtk-driver` package). CI requires `xvfb-run` for a virtual display.
- **Windows**: Works via msedgedriver; version must match the installed Edge. `msedgedriver-tool` can automate this.
- **macOS**: **No support** — WKWebView has no native WebDriver tooling. macOS desktop testing has no automated substitute today.

The test capability object must use `browserName: 'wry'` (Tauri's webview renderer name) and supply the compiled binary path via `tauri:options.application`.

**Where used / relevance to port:**
The entire `test/smoke/` and `test/automation/` Electron path would be rewritten to spawn `tauri-driver`, build the Tauri binary in debug mode in `onPrepare`, and run tests using WebdriverIO or Selenium. The `IWindowDriver` interface (`window.driver`) can still be injected into the webview by the Tauri app; WebdriverIO exposes `driver.execute(fn)` to evaluate JS in the window, so the `getElements`, `setValue`, `typeInEditor` pattern can survive. What **cannot** survive is CDP-level introspection (heap profiler, V8 coverage, `Runtime.queryObjects`).

---

#### @vscode/test-electron (^2.4.0)
**Docs:** https://github.com/microsoft/vscode-test (no dedicated hosted docs; API surface in `@vscode/test-electron` npm package)

**Relevant behaviour:**
Used in exactly one place in the partition: `vscodetest.download({ cachePath, version, extractSync })` in `test/smoke/src/main.ts` (line 299) and `test/mcp/src/application.ts` (line 167). Its role is purely artifact management — downloading a specific version of the VS Code Electron binary (typically the previous stable release, used for data-loss migration smoke tests). It does not participate in launching or driving tests.

**Where used / relevance to port:**
In a Tauri port there is no "VS Code stable build" to download for migration tests; the binary would be produced by `cargo tauri build --debug`. The `@vscode/test-electron` import can be dropped and replaced with logic that either calls `cargo` directly or downloads a pre-built Tauri application artifact from a CI store. This is a low-effort replacement with no architectural implications.

---

#### Mocha (^11.7.5 in `test/sanity/`, via root in `test/smoke/`)
**Docs:** https://mochajs.org/

**Relevant behaviour:**
Mocha is the JS test runner used for smoke tests and sanity tests. It is entirely runtime-agnostic; it runs in Node.js and orchestrates async test functions. The `test/unit/electron/index.js` file runs Mocha *inside* an Electron renderer process using `electron` module APIs (`app`, `BrowserWindow`, `ipcMain`), which is specific to Electron and must be rewritten.

**Where used / relevance to port:**
- **`test/smoke/`**: Mocha runs in Node.js, not in Electron. Fully portable.
- **`test/sanity/`**: Same — portable.
- **`test/unit/electron/index.js`**: Requires `electron` directly and constructs a `BrowserWindow` to run unit tests in the renderer. This file is Electron-specific and must be replaced with a Tauri equivalent. Options: (a) run units in Node.js (already done for `test/unit/node/`), (b) inject a Mocha runner into the Tauri webview via a dedicated test build, (c) use `vitest` with JSDOM for DOM-dependent units.

---

#### axe-core (^4.10.2)
**Docs:** https://github.com/dequelabs/axe-core

**Relevant behaviour:**
`playwrightDriver.ts` injects `axe.min.js` source into the page via `page.evaluate(axeSource)` and then calls `window.axe.run(context, opts)`. This is a pure-JS injection pattern that works in any webview.

**Where used / relevance to port:**
Fully portable. Under Tauri, `driver.execute(() => window.axe.run(...))` (WebdriverIO) or any equivalent `evaluate` call achieves the same injection. No changes needed to axe-core itself.

---

#### `window.driver` IPC bridge (IWindowDriver interface)
**Docs:** (internal) `src/vs/workbench/services/driver/common/driver.ts`

**Relevant behaviour:**
The `IWindowDriver` interface defines the test-only API that VS Code exposes on `window.driver` in smoke-test builds (`--enable-smoke-test-driver` CLI flag). Methods: `setValue`, `isActiveElement`, `getElements`, `getElementXY`, `typeInEditor`, `getEditorSelection`, `getTerminalBuffer`, `writeInTerminal`, `getLocaleInfo`, `getLocalizedStrings`, `getLogs`, `whenWorkbenchRestored`. All are pure DOM operations registered on the global window object by the workbench bootstrap code.

**Where used / relevance to port:**
The interface itself is frontend JS and does not depend on Electron. In a Tauri port it survives unchanged — the workbench bootstrap still runs in a webview and can still attach `window.driver`. The calling side (`playwrightDriver.ts` lines like `this.page.evaluate(([driver, selector]) => driver.getElements(selector))`) must change only in how `page.evaluate` is spelled — WebdriverIO uses `browser.execute(fn)` with the same semantics. This is the most portable component in the entire test partition.

---

## Gaps / Platform Limitations to Flag

1. **macOS E2E gap**: `tauri-driver` has no macOS support. There is currently no automated E2E substitute for macOS desktop. Any CI requirement for macOS smoke tests must use a different strategy (e.g., running the web-server path with a real browser, or waiting for WebKit Inspector Protocol support in Tauri).

2. **CDP / profiler tests**: `test/automation/src/playwrightDriver.ts` uses raw CDP sessions (heap snapshot, `Runtime.queryObjects`, `HeapProfiler.collectGarbage`) that are unavailable through W3C WebDriver. These memory profiler tests cannot be ported without either (a) targeting a Chromium-based build on Windows where WebView2 exposes remote debugging, or (b) replacing CDP calls with Tauri's own memory introspection mechanisms.

3. **tauri-driver pre-alpha status**: The tool is officially labeled pre-alpha. Flakiness and API instability should be expected. The community `tauri-pilot` alternative may be more pragmatic for initial E2E coverage while `tauri-driver` matures.

4. **`test/unit/electron/index.js`**: Directly imports `electron` (`app`, `BrowserWindow`, `ipcMain`). This file is the entry point for Electron unit tests that require a real renderer (GPU, DOM APIs not available in Node). Replacing it is non-trivial and likely requires a custom Tauri test plugin or a move to Vitest + JSDOM.

---

## Conclusion

The `test/` partition's testing infrastructure is **substantially but not wholly portable**. The `window.driver` IPC bridge, Mocha, axe-core, and the browser/web-server smoke path are all runtime-agnostic and survive intact. The Electron-specific components — `playwright._electron`, `test/unit/electron/index.js`, and `@vscode/test-electron` artifact download — each require replacement, with `playwright._electron` being the largest rewrite (the entire `test/automation/src/playwrightElectron.ts` launch path). The realistic replacement is `tauri-driver` + WebdriverIO on Linux/Windows CI, accepting that macOS automated E2E and CDP-based memory profiling have no direct equivalent today. Roughly 30–40% of the smoke-test harness code needs rewriting; the test *logic* (the area `.test.ts` files) can be preserved as-is because it operates through the `Code` / `PlaywrightDriver` abstraction layer.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
