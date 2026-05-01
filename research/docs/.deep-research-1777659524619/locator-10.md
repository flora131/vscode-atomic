# Test Harness Infrastructure — File Locator

**Scope:** `test/` directory (130 files, 17,093 LOC)

## Implementation

### Smoke Test Framework
- `test/smoke/src/main.ts` — Entry point orchestrating test setup, logging, and quality/environment selection
- `test/smoke/src/utils.ts` — Utility functions for test execution and retries
- `test/smoke/test/index.js` — Test runner initialization

### Automation Driver (UI Interaction Layer)
Core abstractions for driving the VS Code UI via Playwright/Electron:

- `test/automation/src/application.ts` — Application lifecycle management (startup/teardown)
- `test/automation/src/code.ts` — Code instance abstraction and connection
- `test/automation/src/workbench.ts` — Workbench surface area and navigation
- `test/automation/src/electron.ts` — Electron-specific paths and version detection
- `test/automation/src/playwrightDriver.ts` — Playwright driver abstraction
- `test/automation/src/playwrightElectron.ts` — Electron via Playwright binding
- `test/automation/src/playwrightBrowser.ts` — Browser via Playwright binding
- `test/automation/src/logger.ts` — Multi-sink logging (console/file)
- `test/automation/src/profiler.ts` — Performance measurement instrumentation

### IDE Feature Drivers (Smoke Test Automatable Surface)
Modular drivers for testing discrete IDE functionality:

- `test/automation/src/activityBar.ts` — Activity bar interaction
- `test/automation/src/editor.ts` — Single editor tab control
- `test/automation/src/editors.ts` — Editor group/tab management
- `test/automation/src/explorer.ts` — File explorer viewlet
- `test/automation/src/search.ts` — Search viewlet
- `test/automation/src/scm.ts` — Source control viewlet
- `test/automation/src/debug.ts` — Debug session control
- `test/automation/src/terminal.ts` — Terminal management
- `test/automation/src/task.ts` — Task execution
- `test/automation/src/problems.ts` — Problems panel
- `test/automation/src/extensions.ts` — Extension discovery/management
- `test/automation/src/settings.ts` — Settings editor interaction
- `test/automation/src/keybindings.ts` — Keybinding editor
- `test/automation/src/quickinput.ts` — Quick input/picker
- `test/automation/src/quickaccess.ts` — Quick access (Ctrl+P)
- `test/automation/src/peek.ts` — Peek definition/references
- `test/automation/src/viewlet.ts` — Generic viewlet base
- `test/automation/src/localization.ts` — Localization/NLS handling
- `test/automation/src/notebook.ts` — Notebook editor
- `test/automation/src/chat.ts` — Chat UI

### Smoke Test Area Suites (Feature-Specific Test Scenarios)
End-to-end tests exercising IDE subsystems:

- `test/smoke/src/areas/workbench/launch.test.ts` — Application startup, recovery, workspace loading
- `test/smoke/src/areas/workbench/data-loss.test.ts` — Unsaved data handling and crash recovery
- `test/smoke/src/areas/workbench/localization.test.ts` — Language switching and NLS
- `test/smoke/src/areas/preferences/preferences.test.ts` — Settings and keybindings UI
- `test/smoke/src/areas/editor/` — (Referenced in main.ts; not present in glob; likely scoped elsewhere)
- `test/smoke/src/areas/search/search.test.ts` — Find/replace in editor and workspace search
- `test/smoke/src/areas/explorer/` — (Referenced indirectly; explorer is driven from workbench)
- `test/smoke/src/areas/extensions/extensions.test.ts` — Extension discovery and installation
- `test/smoke/src/areas/extensions/extension-host-restart.test.ts` — Extension host lifecycle
- `test/smoke/src/areas/languages/languages.test.ts` — Language server and syntax highlighting
- `test/smoke/src/areas/statusbar/statusbar.test.ts` — Status bar interactions
- `test/smoke/src/areas/task/task.test.ts` — Task execution and quick pick
- `test/smoke/src/areas/task/task-quick-pick.test.ts` — Task quick pick UI
- `test/smoke/src/areas/terminal/terminal.test.ts` — Terminal creation, I/O, profiles
- `test/smoke/src/areas/terminal/terminal-editors.test.ts` — Terminal editor mode
- `test/smoke/src/areas/terminal/terminal-input.test.ts` — Terminal input handling
- `test/smoke/src/areas/terminal/terminal-persistence.test.ts` — Terminal state restoration
- `test/smoke/src/areas/terminal/terminal-profiles.test.ts` — Terminal profile management
- `test/smoke/src/areas/terminal/terminal-shellIntegration.test.ts` — Shell integration protocol
- `test/smoke/src/areas/terminal/terminal-splitCwd.test.ts` — Terminal splitting and cwd
- `test/smoke/src/areas/terminal/terminal-tabs.test.ts` — Terminal tab management
- `test/smoke/src/areas/terminal/terminal-stickyScroll.test.ts` — Terminal sticky scroll
- `test/smoke/src/areas/terminal/terminal-helpers.ts` — Terminal test utilities
- `test/smoke/src/areas/notebook/notebook.test.ts` — Notebook editor and execution
- `test/smoke/src/areas/multiroot/multiroot.test.ts` — Multi-root workspace
- `test/smoke/src/areas/accessibility/accessibility.test.ts` — Keyboard navigation and screen reader
- `test/smoke/src/areas/chat/chatDisabled.test.ts` — Chat feature availability

### Unit Test Infrastructure
Core test framework and runners:

- `test/unit/reporter.js` — Mocha reporter adapter and custom reporter registration
- `test/unit/fullJsonStreamReporter.js` — Streaming JSON reporter for CI parsing
- `test/unit/assert.js` — Custom assertion library for unit tests
- `test/unit/coverage.js` — Coverage instrumentation
- `test/unit/analyzeSnapshot.js` — Snapshot comparison utilities
- `test/unit/electron/index.js` — Electron test runner (main process, renderer, preload)
- `test/unit/electron/renderer.js` — Renderer process test harness
- `test/unit/electron/preload.js` — Preload script for test environment
- `test/unit/browser/index.js` — Browser-based test harness
- `test/unit/node/index.js` — Node.js test runner

### Integration Test Infrastructure
API and extension integration test harness:

- `test/integration/electron/testrunner.js` — Electron integration test runner
- `test/integration/electron/testrunner.d.ts` — Type definitions for testrunner
- `test/integration/browser/src/index.ts` — Browser-based integration test entry point

### Sanity Test Framework (Release Validation)
End-to-end installation and functionality validation:

- `test/sanity/src/main.ts` — Test orchestration, platform detection, cleanup
- `test/sanity/src/index.ts` — Test registry and setup
- `test/sanity/src/context.ts` — Test context and state management
- `test/sanity/src/uiTest.ts` — UI testing base class
- `test/sanity/src/detectors.ts` — Platform/environment capability detection
- `test/sanity/src/githubAuth.ts` — GitHub authentication for release assets
- `test/sanity/src/cli.test.ts` — CLI argument and help testing
- `test/sanity/src/server.test.ts` — VS Code Server startup and basic ops
- `test/sanity/src/serverWeb.test.ts` — Web server validation
- `test/sanity/src/desktop.test.ts` — Desktop/Electron app startup
- `test/sanity/src/wsl.test.ts` — WSL mode functionality
- `test/sanity/src/devTunnel.test.ts` — Dev tunnel remote connectivity

### MCP Server (AI Assistant Test Automation)
Model Context Protocol server exposing VS Code automation to AI clients:

- `test/mcp/src/stdio.ts` — MCP stdio transport entry point
- `test/mcp/src/automation.ts` — MCP server definition and tool registration
- `test/mcp/src/application.ts` — MCP-exposed application lifecycle
- `test/mcp/src/options.ts` — Command-line option parsing for MCP server
- `test/mcp/src/utils.ts` — Utility functions
- `test/mcp/src/automationTools/index.ts` — Tool registration and export
- `test/mcp/src/automationTools/core.ts` — MCP tool wrappers for core operations
- `test/mcp/src/automationTools/editor.ts` — MCP editor tools
- `test/mcp/src/automationTools/terminal.ts` — MCP terminal tools
- `test/mcp/src/automationTools/debug.ts` — MCP debug tools
- `test/mcp/src/automationTools/search.ts` — MCP search tools
- `test/mcp/src/automationTools/scm.ts` — MCP SCM tools
- `test/mcp/src/automationTools/task.ts` — MCP task tools
- `test/mcp/src/automationTools/explorer.ts` — MCP explorer tools
- `test/mcp/src/automationTools/extensions.ts` — MCP extension tools
- `test/mcp/src/automationTools/settings.ts` — MCP settings tools
- `test/mcp/src/automationTools/keybindings.ts` — MCP keybindings tools
- `test/mcp/src/automationTools/activityBar.ts` — MCP activity bar tools
- `test/mcp/src/automationTools/statusbar.ts` — MCP status bar tools
- `test/mcp/src/automationTools/quickAccess.ts` — MCP quick access tools
- `test/mcp/src/automationTools/problems.ts` — MCP problems panel tools
- `test/mcp/src/automationTools/notebook.ts` — MCP notebook tools
- `test/mcp/src/automationTools/chat.ts` — MCP chat tools
- `test/mcp/src/automationTools/localization.ts` — MCP localization tools
- `test/mcp/src/automationTools/windows.ts` — MCP window management
- `test/mcp/src/automationTools/profiler.ts` — MCP profiler tools

### Component/Widget Tests
UI component-focused test fixtures:

- `test/componentFixtures/playwright/tests/imageCarousel.spec.ts` — Image carousel Playwright test
- `test/componentFixtures/playwright/tests/utils.ts` — Playwright fixture utilities

## Configuration

- `test/.mocharc.json` — Mocha runner configuration (TDD ui, 10s timeout)
- `test/cgmanifest.json` — Component governance manifest
- `test/automation/tsconfig.json` — TypeScript config for automation package
- `test/automation/package.json` — Automation package definition (Playwright, logger, Mocha)
- `test/automation/tools/copy-driver-definition.js` — Build tool for driver definition copying
- `test/automation/tools/copy-package-version.js` — Build tool for version information
- `test/smoke/tsconfig.json` — Smoke test TypeScript config
- `test/smoke/package.json` — Smoke test package definition
- `test/smoke/extensions/vscode-smoketest-ext-host/package.json` — Test extension for host validation
- `test/integration/browser/tsconfig.json` — Browser integration test config
- `test/integration/browser/package.json` — Browser integration package
- `test/integration/electron/testrunner.d.ts` — Electron runner type definitions
- `test/sanity/tsconfig.json` — Sanity test TypeScript config
- `test/sanity/package.json` — Sanity test package definition
- `test/monaco/tsconfig.json` — Monaco (editor core) test config
- `test/monaco/.mocharc.json` — Monaco Mocha config
- `test/monaco/package.json` — Monaco test package
- `test/componentFixtures/playwright/tsconfig.json` — Component test TypeScript config
- `test/componentFixtures/playwright/playwright.config.ts` — Playwright browser config
- `test/componentFixtures/playwright/package.json` — Component test package
- `test/mcp/tsconfig.json` — MCP server TypeScript config
- `test/mcp/package.json` — MCP server package definition
- `test/unit/node/package.json` — Node test runner package

## Documentation

- `test/README.md` — Overview of test suites and directory structure
- `test/automation/README.md` — Automation driver package description
- `test/smoke/README.md` — Smoke test execution instructions, pitfalls, debugging
- `test/smoke/Audit.md` — Test coverage audit
- `test/unit/README.md` — Unit test runner options (Electron, browser, Node, coverage)
- `test/integration/browser/README.md` — Browser integration test runner instructions
- `test/sanity/README.md` — Release sanity test framework (platforms, Docker containers, CI/CD)
- `test/mcp/README.md` — MCP server capabilities and architecture
- `test/monaco/README.md` — Monaco editor tests
- `test/componentFixtures/blocks-ci-screenshots.md` — Component fixture CI/screenshot docs

## Examples / Fixtures

- `test/componentFixtures/component-explorer.json` — Component listing for UI testing
- `test/componentFixtures/component-explorer-diff.json` — Component diff fixture
- `test/sanity/containers/` — Docker container definitions for Linux sanity testing
- `test/sanity/scripts/` — Platform-specific test runner scripts (run-win32.cmd, run-macOS.sh, run-ubuntu.sh, run-docker.sh)

## Notable Clusters

### `test/automation/src/` — 34 TypeScript files, 4,335 LOC
Core automation driver library that abstracts VS Code UI interactions. Provides a driver-client interface where smoke tests (or any external tool) can control application launch, navigate workbench elements, and interact with editors, terminals, and dialogs. Uses Playwright for both Electron and browser transports. This layer is central to all smoke and integration test execution.

### `test/smoke/src/areas/` — 12 directories, ~2,837 LOC
Feature-organized end-to-end test scenarios that exercise discrete IDE subsystems (workbench lifecycle, preferences, search, extensions, languages, terminal, task, notebook, multiroot, accessibility, chat). Each area is registered in main.ts as a test suite. Together they verify that cross-process automation can drive the full IDE surface.

### `test/mcp/src/` — 26 TypeScript files, 3,100 LOC
MCP server exposing automation and application lifecycle to external AI clients. Mirrors the automation driver's feature-area organization (editor, terminal, debug, search, SCM, etc.) but wrapped as MCP protocol tools. Demonstrates how the automation layer can be consumed by non-test clients.

### `test/sanity/src/` — 12 TypeScript files, 2,893 LOC
Release-specific end-to-end validation covering installation, CLI, server modes (desktop, web, WSL, remote), and platform-specific setup. Includes platform detection to conditionally run tests on native hosts vs. Docker containers. Validates published builds meet quality gates before release.

### `test/unit/` — 10 JavaScript files, 789 LOC
Electron and browser test runners for unit tests defined elsewhere in the codebase. Provides reporters, custom assertions, coverage instrumentation, and preload/sandbox bridging. Does not contain unit test specs themselves (those live in feature directories), only the harness.

### `test/integration/` — 3 files, 288 LOC
Type definitions and runners for integration tests (Electron and browser variants). Minimal implementation; the actual test specs are registered dynamically from the IDE's extension host and API surface.

## Summary

The test/ directory is a multi-layer end-to-end and integration test harness built on Playwright, Node.js, and Mocha. It exhibits a clean separation of concerns:

1. **Automation Layer** (`test/automation/`) — Playwright-based driver abstracting Electron and browser control; UI interaction primitives for each IDE feature (editor, terminal, search, etc.).

2. **Smoke Tests** (`test/smoke/`) — Feature-area suites using the automation layer to exercise the full IDE surface end-to-end, emulating user workflows (launch, edit, search, debug, task, terminal, extension, notebook, accessibility).

3. **MCP Integration** (`test/mcp/`) — Parallel implementation of automation as MCP tools, allowing AI assistants and external tools to control VS Code programmatically.

4. **Sanity/Release Tests** (`test/sanity/`) — Published build validation across platforms and installation methods (native, WSL, server, containers), verifying installation and critical paths work before release.

5. **Unit Test Infrastructure** (`test/unit/`, `test/integration/`) — Runners and reporters for unit/integration test execution within Electron, browsers, and Node.

A Rust port would need to replicate or adapt:
- The **automation driver abstraction** (UI control interface); likely reimplemented in Rust using a tauri-based UI automation crate or native platform APIs.
- **Smoke test scenarios** (the feature-area workflows); these are test logic and would port as Rust integration test suites.
- **MCP server** as a Rust-native server exposing automation capabilities.
- **Platform-specific sanity validation** (installation, WSL, server modes); mostly shell/platform scripts that remain orthogonal to core IDE implementation.
- **Unit/integration test harness** infrastructure; rewritten for Rust's native test framework (cargo test) and adjusted for the runtime environment (Tauri, webview, native IPC).
