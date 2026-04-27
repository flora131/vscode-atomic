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
