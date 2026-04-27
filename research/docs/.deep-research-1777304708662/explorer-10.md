# Partition 10 of 79 — Findings

## Scope
`test/` (130 files, 16,989 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Test Directory Structure: Integration/Smoke Harness (Partition 10)

## Overview
The `test/` directory (scope: 130+ files, ~16,989 LOC across source) contains VS Code's test automation infrastructure. The partition 10 focus is the integration and smoke test harness that defines acceptance criteria for core IDE functionality. These are NOT porting targets, but rather define what functionality must work across different platforms and rendering engines.

## Implementation

### Smoke Tests (Primary UI/Integration Testing)
- `test/smoke/src/main.ts` - Test harness entry point; bootstraps all smoke test suites with environment detection (Electron/Web)
- `test/smoke/src/utils.ts` - Utility functions for smoke tests (retry logic, helpers)
- `test/smoke/src/areas/workbench/launch.test.ts` - VS Code startup and launch scenarios
- `test/smoke/src/areas/workbench/data-loss.test.ts` - Data loss prevention checks
- `test/smoke/src/areas/workbench/localization.test.ts` - Localization/i18n validation
- `test/smoke/src/areas/terminal/terminal.test.ts` - Terminal basic functionality
- `test/smoke/src/areas/terminal/terminal-persistence.test.ts` - Terminal state preservation
- `test/smoke/src/areas/terminal/terminal-input.test.ts` - Terminal input handling
- `test/smoke/src/areas/terminal/terminal-profiles.test.ts` - Terminal profile configuration
- `test/smoke/src/areas/terminal/terminal-shellIntegration.test.ts` - Shell integration features
- `test/smoke/src/areas/terminal/terminal-editors.test.ts` - Terminal editor integration
- `test/smoke/src/areas/terminal/terminal-splitCwd.test.ts` - Terminal split pane directory handling
- `test/smoke/src/areas/terminal/terminal-stickyScroll.test.ts` - Terminal sticky scroll feature
- `test/smoke/src/areas/terminal/terminal-tabs.test.ts` - Terminal tab management
- `test/smoke/src/areas/terminal/terminal-helpers.ts` - Shared terminal test utilities
- `test/smoke/src/areas/preferences/preferences.test.ts` - Settings and preferences UI
- `test/smoke/src/areas/search/search.test.ts` - Search functionality
- `test/smoke/src/areas/extensions/extensions.test.ts` - Extension loading and management
- `test/smoke/src/areas/extensions/extension-host-restart.test.ts` - Extension host restart scenarios
- `test/smoke/src/areas/languages/languages.test.ts` - Language support and IntelliSense
- `test/smoke/src/areas/notebook/notebook.test.ts` - Notebook editor functionality
- `test/smoke/src/areas/task/task.test.ts` - Task execution
- `test/smoke/src/areas/task/task-quick-pick.test.ts` - Task quick pick UI
- `test/smoke/src/areas/statusbar/statusbar.test.ts` - Status bar rendering
- `test/smoke/src/areas/multiroot/multiroot.test.ts` - Multi-root workspace scenarios
- `test/smoke/src/areas/accessibility/accessibility.test.ts` - Accessibility features
- `test/smoke/src/areas/chat/chatDisabled.test.ts` - Chat feature when disabled

### Automation Driver / UI Interaction Library
- `test/automation/src/index.ts` - Main export for automation module (33 feature modules)
- `test/automation/src/application.ts` - Application lifecycle management (start, stop, restart)
- `test/automation/src/code.ts` - VS Code process launching and control
- `test/automation/src/workbench.ts` - Main workbench UI interaction API
- `test/automation/src/logger.ts` - Structured logging (console, file, multi-logger)
- `test/automation/src/profiler.ts` - Performance profiling and measurement
- `test/automation/src/editor.ts` - Editor content manipulation
- `test/automation/src/editors.ts` - Multiple editor management
- `test/automation/src/explorer.ts` - File explorer/sidebar navigation
- `test/automation/src/search.ts` - Search UI interaction
- `test/automation/src/scm.ts` - Source control management UI
- `test/automation/src/debug.ts` - Debug panel and session management
- `test/automation/src/problems.ts` - Problems/diagnostics panel
- `test/automation/src/terminal.ts` - Terminal UI automation
- `test/automation/src/viewlet.ts` - Generic sidebar/viewlet handling
- `test/automation/src/quickaccess.ts` - Quick access/command palette
- `test/automation/src/quickinput.ts` - Quick input boxes
- `test/automation/src/extensions.ts` - Extensions viewlet
- `test/automation/src/notebook.ts` - Notebook editor UI automation
- `test/automation/src/localization.ts` - Localization/language selection
- `test/automation/src/task.ts` - Task execution UI
- `test/automation/src/keybindings.ts` - Keyboard shortcuts
- `test/automation/src/settings.ts` - Settings editor UI
- `test/automation/src/chat.ts` - Chat UI interaction
- `test/automation/src/activityBar.ts` - Activity bar interaction
- `test/automation/src/peek.ts` - Peek definition/reference UI
- `test/automation/src/processes.ts` - Child process management utilities
- `test/automation/src/playwrightDriver.ts` - Playwright browser driver interface
- `test/automation/src/playwrightBrowser.ts` - Playwright browser automation
- `test/automation/src/playwrightElectron.ts` - Playwright Electron integration
- `test/automation/src/electron.ts` - Native Electron API access
- `test/automation/src/driver.d.ts` - Driver interface TypeScript definitions

### Integration Tests (API Testing)
- `test/integration/browser/src/index.ts` - Browser-based API integration tests
- `test/integration/electron/testrunner.js` - Electron test runner harness

### Unit Tests (Component/Module Testing)
- `test/unit/electron/index.js` - Electron-based unit test entry point
- `test/unit/electron/renderer.js` - Renderer process test setup
- `test/unit/electron/preload.js` - Electron preload script for tests
- `test/unit/browser/index.js` - Browser-based unit tests
- `test/unit/node/index.js` - Node.js-based unit tests
- `test/unit/assert.js` - Custom assertion library
- `test/unit/reporter.js` - Test result reporter
- `test/unit/fullJsonStreamReporter.js` - JSON stream reporter
- `test/unit/coverage.js` - Code coverage utilities
- `test/unit/analyzeSnapshot.js` - Snapshot analysis tool

### Sanity Tests (Release Validation)
- `test/sanity/src/main.ts` - Release sanity test entry point
- `test/sanity/src/index.ts` - Test suite initialization
- `test/sanity/src/context.ts` - Test execution context setup
- `test/sanity/src/cli.test.ts` - Command-line interface testing
- `test/sanity/src/desktop.test.ts` - Desktop application scenarios
- `test/sanity/src/server.test.ts` - VS Code server scenarios
- `test/sanity/src/serverWeb.test.ts` - Web server scenarios
- `test/sanity/src/wsl.test.ts` - Windows Subsystem for Linux scenarios
- `test/sanity/src/devTunnel.test.ts` - Dev tunnel functionality
- `test/sanity/src/githubAuth.ts` - GitHub authentication helper
- `test/sanity/src/detectors.ts` - Platform/environment detection
- `test/sanity/src/uiTest.ts` - UI-based test utilities

### MCP Server for Automation (AI Assistant Integration)
- `test/mcp/src/stdio.ts` - MCP server stdio transport
- `test/mcp/src/automation.ts` - MCP server initialization and tool registration
- `test/mcp/src/application.ts` - Application management tools
- `test/mcp/src/automationTools/index.ts` - Tool registry and exports
- `test/mcp/src/automationTools/core.ts` - Core application tools
- `test/mcp/src/automationTools/editor.ts` - Editor manipulation tools
- `test/mcp/src/automationTools/terminal.ts` - Terminal control tools
- `test/mcp/src/automationTools/debug.ts` - Debug session tools
- `test/mcp/src/automationTools/search.ts` - Search/find tools
- `test/mcp/src/automationTools/extensions.ts` - Extension management tools
- `test/mcp/src/automationTools/explorer.ts` - File explorer tools
- `test/mcp/src/automationTools/settings.ts` - Settings modification tools
- `test/mcp/src/automationTools/scm.ts` - Source control tools
- `test/mcp/src/automationTools/problems.ts` - Diagnostics tools
- `test/mcp/src/automationTools/keybindings.ts` - Keyboard binding tools
- `test/mcp/src/automationTools/task.ts` - Task runner tools
- `test/mcp/src/automationTools/statusbar.ts` - Status bar interaction tools
- `test/mcp/src/automationTools/notebook.ts` - Notebook tools
- `test/mcp/src/automationTools/chat.ts` - Chat interface tools
- `test/mcp/src/automationTools/activityBar.ts` - Activity bar tools
- `test/mcp/src/automationTools/quickAccess.ts` - Quick access tools
- `test/mcp/src/automationTools/windows.ts` - Window management tools
- `test/mcp/src/automationTools/localization.ts` - Localization tools
- `test/mcp/src/automationTools/profiler.ts` - Performance profiling tools

### Component Fixtures and Playwright Tests
- `test/componentFixtures/playwright/tests/imageCarousel.spec.ts` - Image carousel component test
- `test/componentFixtures/playwright/tests/utils.ts` - Playwright test utilities
- `test/componentFixtures/playwright/playwright.config.ts` - Playwright configuration

## Tests

All test files follow TypeScript/JavaScript with Mocha test framework (TDD-style). Primary test areas covered:

### Smoke Test Coverage (24 test files)
- **Workbench**: Launch, data loss, localization
- **Terminal**: Basic, persistence, input, profiles, shell integration, editors, split cwd, sticky scroll, tabs
- **Preferences**: Settings editor
- **Search**: Full-text search
- **Extensions**: Loading, management, host restart
- **Languages**: Language support, IntelliSense
- **Notebook**: Notebook editor
- **Tasks**: Task execution, quick pick
- **Statusbar**: Status bar rendering
- **Multiroot**: Multi-root workspaces
- **Accessibility**: A11y features
- **Chat**: Chat when disabled

### Test Execution Models
- **Electron**: Native desktop tests with full DOM access and Node.js APIs
- **Web**: Browser-based tests (Chromium/WebKit via Playwright)
- **Remote**: Remote connection scenarios
- **Headless**: Headless browser testing

## Types / Interfaces

### Driver/Automation Types
- `test/automation/src/driver.d.ts` - Driver interface definitions
- `test/automation/out/driver.d.ts` - Compiled driver definitions
- `test/automation/out/*.d.ts` - All 33 automation module type definitions (compiled from src/)

### Test Configuration Types
- `test/smoke/tsconfig.json` - TypeScript config (target: ES2024, commonjs)
- `test/integration/browser/tsconfig.json` - Integration test TypeScript config
- `test/mcp/tsconfig.json` - MCP server TypeScript config
- `test/automation/tsconfig.json` - Automation library TypeScript config
- `test/sanity/tsconfig.json` - Sanity test TypeScript config
- `test/componentFixtures/playwright/tsconfig.json` - Playwright test TypeScript config

## Configuration

### Test Runners and Harness
- `test/.mocharc.json` - Global Mocha configuration (TDD ui, 10s timeout)
- `test/smoke/package.json` - Smoke test dependencies and scripts
- `test/smoke/tsconfig.json` - TypeScript compilation config
- `test/integration/browser/package.json` - Browser integration test setup
- `test/unit/node/package.json` - Node.js unit test environment
- `test/automation/package.json` - Automation driver dependencies
- `test/mcp/package.json` - MCP server dependencies and scripts
- `test/sanity/package.json` - Release sanity test setup
- `test/package.json` - Root test package manifest
- `test/componentFixtures/playwright/package.json` - Playwright test dependencies

### Build and Test Scripts
- `test/smoke/test/index.js` - Smoke test entry point script
- `test/automation/tools/copy-driver-definition.js` - Build tool: copies driver definitions
- `test/automation/tools/copy-package-version.js` - Build tool: manages version info
- `test/mcp/scripts/` - MCP server build/run scripts

### Manifest and Dependencies
- `test/cgmanifest.json` - Component/dependency manifest
- `test/componentFixtures/component-explorer.json` - Component metadata
- `test/componentFixtures/component-explorer-diff.json` - Component changes tracking
- `test/smoke/extensions/vscode-smoketest-ext-host/package.json` - Test extension for extension host

## Examples / Fixtures

### Smoke Test Extension
- `test/smoke/extensions/vscode-smoketest-ext-host/` - Extension host test fixture
  - `extension.js` - Simple test extension
  - `package.json` - Extension manifest

### Component Fixtures
- `test/componentFixtures/playwright/` - Playwright-based component testing
  - Contains visual regression and interaction tests
  - Integration with CI/CD for screenshot validation

### Release Testing Infrastructure
- `test/sanity/containers/` - Docker container definitions for testing across Linux distributions
  - Alpine, CentOS, Debian 10/12, Fedora, openSUSE, Red Hat, Ubuntu
- `test/sanity/scripts/` - Platform-specific test runners
  - `run-win32.cmd`, `run-macOS.sh`, `run-ubuntu.sh`, `run-docker.sh`

## Documentation

### Test Suite READMEs
- `test/README.md` - Overview of all test suites (unit, integration, smoke, sanity)
- `test/smoke/README.md` - Smoke test guide (execution modes, debugging, troubleshooting)
- `test/automation/README.md` - Automation driver documentation
- `test/integration/browser/README.md` - Browser integration test setup
- `test/unit/README.md` - Unit test execution (Electron, browser, Node.js)
- `test/sanity/README.md` - Release sanity check testing (comprehensive matrix, Docker support)
- `test/mcp/README.md` - MCP server documentation (tools, architecture, usage)
- `test/monaco/README.md` - Monaco editor testing
- `test/componentFixtures/blocks-ci-screenshots.md` - Component screenshot CI documentation

### Test Design Guidelines
- `test/smoke/Audit.md` - Smoke test failure history and best practices
  - Documents DOM selector stability patterns
  - Guides test maintainability

## Notable Clusters

### Smoke Test Area Organization
The `test/smoke/src/areas/` directory contains 14 feature areas with organized test suites:
- **workbench/** (3 tests) - Core IDE functionality
- **terminal/** (9 tests) - Terminal features (most extensive)
- **preferences/** (1 test) - Settings UI
- **search/** (1 test) - Search/find
- **extensions/** (2 tests) - Extension system
- **languages/** (1 test) - Language support
- **notebook/** (1 test) - Notebook editor
- **task/** (2 tests) - Task system
- **statusbar/** (1 test) - Status bar
- **multiroot/** (1 test) - Workspace modes
- **accessibility/** (1 test) - A11y
- **chat/** (1 test) - Chat features

### Automation Driver Modules (33 modules)
`test/automation/src/` contains comprehensive UI automation covering:
- **Process/lifecycle**: code, application, electron, processes
- **Editors**: editor, editors, notebook
- **UI panels**: explorer, search, scm, debug, problems, terminal
- **Settings/config**: settings, preferences, keybindings, localization
- **Quick access**: quickaccess, quickinput
- **Sidebar**: activityBar, viewlet, extensions
- **Analysis**: peek
- **Utilities**: workbench, logger, profiler, driver

### MCP Automation Tools (22 tools)
`test/mcp/src/automationTools/` mirrors automation driver organization but exposes functionality via Model Context Protocol for AI assistants.

### Cross-Platform Testing Matrix
- **Electron**: Native desktop (full API access)
- **Web**: Chromium and WebKit browsers (limited to web APIs)
- **Remote**: SSH remote scenarios
- **Linux containers**: Alpine, CentOS, Debian, Fedora, openSUSE, Red Hat, Ubuntu
- **Platforms**: Windows (native + containers), macOS (native), Linux (native + containers)

### Integration Test Entry Points
- `test/integration/electron/testrunner.js` - Electron runner for API tests
- `test/integration/browser/src/index.ts` - Browser runner for API tests
- Both test the vscode API surface area

### Unit Test Environment Matrix
- **Electron renderer**: Full DOM + Node.js APIs (closest to VS Code environment)
- **Browser**: Web APIs only (Chromium, WebKit)
- **Node.js**: CLI and module tests

## Acceptance Criteria Defined

The test harness explicitly defines what must work across platforms:

1. **Startup/Shutdown** - Launch.test verifies initialization and graceful shutdown
2. **Data Integrity** - Data-loss.test prevents configuration and workspace state loss
3. **UI Stability** - Terminal, search, preferences, extensions tests verify responsive UI
4. **Multi-root Support** - Multiroot.test ensures workspace features work correctly
5. **Extensibility** - Extensions.test validates extension loading and host isolation
6. **Accessibility** - Accessibility.test ensures keyboard and screen reader support
7. **Localization** - Localization.test verifies i18n across UI
8. **Terminal Parity** - 9 terminal tests ensure feature parity (input, persistence, profiles, shell integration, tabs, split handling)
9. **Remote Scenarios** - Sanity tests validate remote server and tunnel setup
10. **Platform Parity** - Smoke tests run on both Electron and Web, sanity tests across all OS/architectures

The smoke test suite (24 integrated test modules, ~2,837 LOC) combined with automation driver (33 modules, ~4,276 LOC) and sanity tests (~2,893 LOC) form VS Code's critical acceptance criteria. These define the baseline functionality that any alternative rendering engine (Tauri/Rust) must replicate to maintain IDE feature parity.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Test Patterns: Integration/Smoke Harness

## Pattern: Suite Setup with Logger Integration

**Where:** `test/smoke/src/main.ts:406-422`

**What:** Master test suite that registers all area-specific test suites via setup functions, with conditional test loading based on platform and build quality.

```typescript
describe(`VSCode Smoke Tests (${opts.web ? 'Web' : 'Electron'})`, () => {
  if (!opts.web) { setupDataLossTests(() => { return { stableCodePath: opts['stable-build'], stableCodeVersion: opts['stable-version'] } }, logger); }
  setupPreferencesTests(logger);
  setupSearchTests(logger);
  if (!opts.web) { setupNotebookTests(logger); }
  setupLanguagesTests(logger);
  setupTerminalTests(logger);
  setupTaskTests(logger);
  setupStatusbarTests(logger);
  if (quality !== Quality.Dev && quality !== Quality.OSS) { setupExtensionTests(logger); }
  if (!opts.web && !opts.remote) { setupExtensionHostRestartTests(logger); }
  if (!(opts.web && process.platform === 'win32')) { setupMultirootTests(logger); }
  if (!opts.web && !opts.remote && quality !== Quality.Dev && quality !== Quality.OSS) { setupLocalizationTests(logger); }
  if (!opts.web && !opts.remote) { setupLaunchTests(logger); }
  if (!opts.web) { setupChatTests(logger); }
  setupAccessibilityTests(logger, opts, quality);
});
```

**Variations / call-sites:**
- `test/smoke/src/areas/workbench/launch.test.ts:10` — Simple describe wrapper
- `test/smoke/src/areas/terminal/terminal.test.ts:17-18` — Describe with retry configuration
- `test/smoke/src/areas/extensions/extensions.test.ts:10` — Setup function pattern

---

## Pattern: Shared Handler Installation for Test Lifecycle

**Where:** `test/smoke/src/utils.ts:22-26`

**What:** Common before/after hook installer that abstracts diagnostics, app startup, and cleanup across all test suites.

```typescript
export function installAllHandlers(logger: Logger, optionsTransform?: (opts: ApplicationOptions) => ApplicationOptions) {
  installDiagnosticsHandler(logger);
  installAppBeforeHandler(optionsTransform);
  installAppAfterHandler();
}
```

**Variations / call-sites:**
- `test/smoke/src/areas/search/search.test.ts:14` — Basic usage
- `test/smoke/src/areas/extensions/extensions.test.ts:13-17` — With options transformation
- `test/smoke/src/areas/workbench/launch.test.ts:14-19` — Modifying userDataDir via transform

---

## Pattern: Platform-Conditional Test Suite Skipping

**Where:** `test/smoke/src/areas/terminal/terminal-input.test.ts:9-10`

**What:** Conditional describe wrapper that skips entire suite based on platform or runtime conditions.

```typescript
export function setup(options?: { skipSuite: boolean }) {
  (options?.skipSuite ? describe.skip : describe)('Terminal Input', () => {
    // suite contents
  });
}
```

**Variations / call-sites:**
- `test/smoke/src/areas/terminal/terminal.test.ts:42-51` — Platform-based skipping for Linux
- `test/smoke/src/areas/task/task.test.ts:22` — Conditional suite setup
- `test/smoke/src/areas/workbench/data-loss.test.ts:11` — Named describe blocks

---

## Pattern: Retry Configuration for Flaky Tests

**Where:** `test/smoke/src/areas/terminal/terminal.test.ts:18-21`

**What:** Suite-level retry configuration with comments documenting why retries are needed.

```typescript
describe('Terminal', function () {
  // Retry tests 3 times to minimize build failures due to any flakiness
  this.retries(3);
  
  // Shared before/after handling
  installAllHandlers(logger);
```

**Variations / call-sites:**
- `test/smoke/src/areas/task/task.test.ts:14` — Task tests with retries
- `test/smoke/src/areas/accessibility/accessibility.test.ts:16` — Accessibility tests
- `test/smoke/src/areas/accessibility/accessibility.test.ts:99-101` — Per-test retry override with timeout

---

## Pattern: Test Setup with Data Isolation and Cleanup

**Where:** `test/smoke/src/areas/workbench/data-loss.test.ts:22-47`

**What:** Individual test creates isolated app instance with per-test logging paths, executes operations, then tears down.

```typescript
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
  
  await app.restart();

  // Verify 3 editors are open
  await app.workbench.editors.selectTab('Untitled-1');
  await app.workbench.editors.selectTab('app.js');
  await app.workbench.editors.selectTab('www');

  await app.stop();
  app = undefined;
});
```

**Variations / call-sites:**
- `test/smoke/src/areas/workbench/data-loss.test.ts:49-76` — Save/restore lifecycle
- `test/smoke/src/areas/preferences/preferences.test.ts:15-23` — Simple app interaction pattern

---

## Pattern: Diagnostics and Tracing Handler Installation

**Where:** `test/smoke/src/utils.ts:28-69`

**What:** Per-test tracing and logging setup with failure state detection for diagnostic artifact collection.

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
    if (!currentTest) return;

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

**Variations / call-sites:**
- `test/smoke/src/main.ts:99-115` — Multi-logger setup (console + file)
- `test/smoke/src/main.ts:368-393` — Global before hook with application defaults

---

## Pattern: Application Lifecycle Management in Tests

**Where:** `test/automation/src/application.ts:73-92`

**What:** Application start/restart pattern with workspace and extra arguments management for test isolation.

```typescript
async start(): Promise<void> {
  await this._start();
}

async restart(options?: { workspaceOrFolder?: string; extraArgs?: string[] }): Promise<void> {
  await measureAndLog(() => (async () => {
    await this.stop();
    await this._start(options?.workspaceOrFolder, options?.extraArgs);
  })(), 'Application#restart()', this.logger);
}

private async _start(workspaceOrFolder = this._workspacePathOrFolder, extraArgs: string[] = []): Promise<void> {
  this._workspacePathOrFolder = workspaceOrFolder;

  // Launch Code...
  const code = await this.startApplication(extraArgs);

  // ...and make sure the window is ready to interact
  await measureAndLog(() => this.checkWindowReady(code), 'Application#checkWindowReady()', this.logger);
}
```

**Variations / call-sites:**
- `test/automation/src/application.ts:125-148` — Window readiness check with remote connection handling
- `test/smoke/src/areas/workbench/data-loss.test.ts:38` — Restart in data-loss tests

---

## Pattern: Conditional Test Skipping at Individual Level

**Where:** `test/smoke/src/areas/preferences/preferences.test.ts:25-33`

**What:** Individual test marked with `it.skip` with issue reference; alternative uses conditional ternary for platform-based skipping.

```typescript
it.skip('changes "workbench.action.toggleSidebarPosition" command key binding and verifies it', async function () {
  const app = this.app as Application;

  await app.workbench.activitybar.waitForActivityBar(ActivityBarPosition.LEFT);

  await app.workbench.keybindingsEditor.updateKeybinding(
    'workbench.action.toggleSidebarPosition', 
    'View: Toggle Primary Side Bar Position', 
    'ctrl+u', 
    'Control+U'
  );

  await app.code.dispatchKeybinding('ctrl+u', () => app.workbench.activitybar.waitForActivityBar(ActivityBarPosition.RIGHT));
});
```

**Variations / call-sites:**
- `test/smoke/src/areas/search/search.test.ts:65` — `it.skip` with TODO comment
- `test/smoke/src/areas/notebook/notebook.test.ts:30-46` — Multiple skip patterns
- `test/smoke/src/areas/terminal/terminal-input.test.ts:40-45` — Active test example

---

This test harness uses Mocha's describe/it pattern with Playwright-based UI automation via the Application abstraction layer, supporting Electron and browser execution with comprehensive diagnostics collection tied to test outcomes.

## External References
<!-- Source: codebase-online-researcher sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
