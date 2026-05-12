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
