# Partition 10 of 79 — Findings

## Scope
`test/` (130 files, 17,093 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Test Harness Patterns for Rust/Tauri Port

## Overview
The test directory (`test/`, 130 files, 17,093 LOC) implements an end-to-end smoke test harness that drives VS Code via Playwright. This harness is critical for understanding the infrastructure needed to port VS Code to Tauri/Rust, as it defines the contract between the test automation layer and the application shell.

## Key Patterns for Tauri/Rust Porting

---

#### Pattern 1: Process Launch & Lifecycle Management
**Where:** `test/automation/src/code.ts:48-109`, `test/smoke/src/main.ts:189-217`
**What:** Spawns child processes (Electron or web server), registers signal handlers, monitors process stdout/stderr for ready signals, implements graceful shutdown with timeout-based forceful termination.

```typescript
function registerInstance(process: cp.ChildProcess, logger: Logger, type: 'electron' | 'server'): { safeToKill: Promise<void> } {
	const instance = { kill: () => teardown(process, logger) };
	instances.add(instance);

	const safeToKill = new Promise<void>(resolve => {
		process.stdout?.on('data', data => {
			const output = data.toString();
			if (output.indexOf('calling app.quit()') >= 0 && type === 'electron') {
				setTimeout(() => resolve(), 500);
			}
			logger.log(`[${type}] stdout: ${output}`);
		});
		process.stderr?.on('data', error => logger.log(`[${type}] stderr: ${error}`));
	});

	process.once('exit', (code, signal) => {
		logger.log(`[${type}] Process terminated (pid: ${process.pid}, code: ${code}, signal: ${signal})`);
		instances.delete(instance);
	});

	return { safeToKill };
}
```

**Variations / call-sites:**
- `test/automation/src/code.ts:176-233` — Exit flow with 10s/20s timeout thresholds
- `test/automation/src/processes.ts` — Teardown utilities for signal handling

**Relevance to Tauri port:** A Tauri app replaces the Electron process but must expose equivalent process lifecycle hooks, exit signals (via IPC or native plugins), and stderr/stdout streams for the harness to detect readiness.

---

#### Pattern 2: Playwright-Driven UI Automation with Element Polling
**Where:** `test/automation/src/code.ts:345-379`
**What:** Implements exponential retry pattern with configurable interval (100ms default) and retryCount (200 default = 20s total), with per-attempt acceptance functions for stateful waits.

```typescript
private async poll<T>(
	fn: () => Promise<T>,
	acceptFn: (result: T) => boolean,
	timeoutMessage: string,
	retryCount = 200,
	retryInterval = 100 // millis
): Promise<T> {
	let trial = 1;
	let lastError: string = '';

	while (true) {
		if (trial > retryCount) {
			throw new Error(`Timeout: ${timeoutMessage} after ${(retryCount * retryInterval) / 1000} seconds.`);
		}

		let result;
		try {
			result = await fn();
			if (acceptFn(result)) {
				return result;
			} else {
				lastError = 'Did not pass accept function';
			}
		} catch (e: any) {
			lastError = Array.isArray(e.stack) ? e.stack.join(os.EOL) : e.stack;
		}

		await this.wait(retryInterval);
		trial++;
	}
}
```

**Variations / call-sites:**
- `test/automation/src/code.ts:259-299` — Specialized polls for text content, elements, active element, editor selection
- `test/automation/src/code.ts:270-271` — `waitAndClick(selector, xoffset?, yoffset?, retryCount?)` wraps poll for clicks
- `test/smoke/src/utils.ts:164-186` — Generic `retry<T>(task, delay, retries, onBeforeRetry?)` for external tasks

**Relevance to Tauri port:** Tauri apps will need equivalent DOM/UI inspection via webview APIs. The polling pattern is universal; Tauri/Rust tests would use similar retry logic but calling into the Tauri command/invoke system for app queries.

---

#### Pattern 3: Smoke Test Bootstrap & Configuration Matrix
**Where:** `test/smoke/src/main.ts:35-242`
**What:** Parses CLI arguments (`--build`, `--web`, `--remote`, `--headless`, `--browser`, `--tracing`), resolves Electron binary paths from build or dev sources, manages test data directories, downloads stable builds for migration tests, clones/validates test repositories.

```typescript
const opts = minimist(args, {
	string: ['browser', 'build', 'stable-build', 'wait-time', 'test-repo', 'electronArgs'],
	boolean: ['verbose', 'remote', 'web', 'headless', 'tracing'],
	default: { verbose: false }
}) as {
	verbose?: boolean;
	remote?: boolean;
	headless?: boolean;
	web?: boolean;
	tracing?: boolean;
	build?: string;
	'stable-build'?: string;
	browser?: 'chromium' | 'webkit' | 'firefox' | 'chromium-msedge' | 'chromium-chrome';
	electronArgs?: string;
};

// Path resolution for different platforms and build types
const electronPath = getBuildElectronPath(testCodePath);
version = getBuildVersion(testCodePath);
```

**Variations / call-sites:**
- `test/automation/src/electron.ts:21-100` — Platform-specific Electron binary discovery (macOS `.app/Contents/MacOS`, Linux, Windows `.exe`)
- `test/smoke/src/main.ts:248-271` — Repository setup with git clone/reset/clean
- `test/smoke/src/main.ts:274-342` — Stable build download & caching via `@vscode/test-electron`

**Relevance to Tauri port:** A Tauri test harness needs similar build discovery, platform-specific binary paths (`.dmg`, `.exe`, AppImage), workspace/data directory setup, and optional stable version download for regression testing. The configuration matrix (`--web`, `--remote`) must map to Tauri environments (web-view vs. native, local vs. remote connection modes).

---

#### Pattern 4: Mocha Suite Fixture Setup with Logger Integration
**Where:** `test/smoke/src/utils.ts:22-106`
**What:** Installs before/after hooks that create per-suite Application instances with unique log/crash directories, registers diagnostics handler for test start/stop logging, injects tracing start/stop on each test, manages random user-data directory suffixes to avoid path collisions.

```typescript
export function installAllHandlers(logger: Logger, optionsTransform?: (opts: ApplicationOptions) => ApplicationOptions) {
	installDiagnosticsHandler(logger);
	installAppBeforeHandler(optionsTransform);
	installAppAfterHandler();
}

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

export function getRandomUserDataDir(baseUserDataDir: string): string {
	const userDataPathSuffix = [...Array(8)].map(() => Math.random().toString(36)[3]).join('');
	return baseUserDataDir.concat(`-${userDataPathSuffix}`);
}
```

**Variations / call-sites:**
- `test/smoke/src/utils.ts:28-68` — `installDiagnosticsHandler()` logs suite/test titles, calls `startTracing()`/`stopTracing()` on app, logs failure state
- `test/smoke/src/utils.ts:164-186` — `retry<T>` utility with per-retry callback hooks (e.g., `retryWithRestart()`)
- `test/smoke/src/areas/terminal/terminal.test.ts:17-52` — Suite example: passes logger, calls `installAllHandlers()`, sets retries(3), defines `before`/`afterEach` hooks

**Relevance to Tauri port:** Mocha is cross-platform; Tauri tests will use the same fixture pattern. The key is the Application abstraction (`test/automation/src/application.ts`) which wraps the shell's launch/stop/communication. A Tauri port would implement equivalent methods (start, stop, restart, workbench accessor) using Tauri's invoke/listen APIs instead of Playwright driver calls.

---

#### Pattern 5: Electron Configuration & Argument Injection
**Where:** `test/automation/src/electron.ts:21-100`
**What:** Builds platform-specific command-line args (skip-welcome, disable-telemetry, crash-reporter-directory, logsPath, user-data-dir, extensions-dir), handles remote-mode URI rewriting, copies test-resolver extension for remote connections, injects environment variables for logging/verbosity.

```typescript
export async function resolveElectronConfiguration(options: LaunchOptions): Promise<IElectronConfiguration> {
	const args: string[] = [
		'--skip-release-notes',
		'--skip-welcome',
		'--disable-telemetry',
		'--disable-experiments',
		'--no-cached-data',
		'--disable-updates',
		'--disable-extension=vscode.vscode-api-tests',
		`--crash-reporter-directory=${crashesPath}`,
		'--disable-workspace-trust',
		`--logsPath=${logsPath}`
	];

	if (remote) {
		args[0] = `--${workspacePath.endsWith('.code-workspace') ? 'file' : 'folder'}-uri=vscode-remote://test+test/${URI.file(workspacePath).path}`;
		await measureAndLog(() => copyExtension(root, extensionsPath, 'vscode-test-resolver'), 'copyExtension(vscode-test-resolver)', logger);
		args.push('--enable-proposed-api=vscode.vscode-test-resolver');
	}

	if (extraArgs) {
		args.push(...extraArgs);
	}

	return { env, args, electronPath };
}
```

**Variations / call-sites:**
- `test/automation/src/playwrightElectron.ts:13-26` — Adds `--enable-smoke-test-driver` flag for test harness handshake

**Relevance to Tauri port:** Tauri doesn't use CLI flags for test configuration; instead, it will pass config via environment variables or a test-mode JSON config file loaded at startup. The harness must set equivalent `VSCODE_LOGSDIR`, `VSCODE_EXTENSIONS_DIR`, `VSCODE_CRASH_DIR` env vars and inject a test-mode resolver extension.

---

#### Pattern 6: Smoke Test Driver Integration Flag & Playwright Event Hooks
**Where:** `test/automation/src/playwrightElectron.ts:13-86`
**What:** Launches Electron via Playwright's `_electron.launch()`, monitors window lifecycle events (window, close, page, dialog, load, console, pageerror, crash, response), captures HTTP errors (status >= 400), logs all events when verbose mode enabled.

```typescript
async function launchElectron(configuration: IElectronConfiguration, options: LaunchOptions) {
	const playwrightImpl = options.playwright ?? playwright;
	const electron = await measureAndLog(() => playwrightImpl._electron.launch({
		executablePath: configuration.electronPath,
		args: configuration.args,
		recordVideo: options.videosPath ? { dir: options.videosPath, size: { width: 1920, height: 1080 } } : undefined,
		env: configuration.env as { [key: string]: string },
		timeout: 0
	}), 'playwright-electron#launch', logger);

	let window = electron.windows()[0];
	if (!window) {
		window = await measureAndLog(() => electron.waitForEvent('window', { timeout: 0 }), 'playwright-electron#firstWindow', logger);
	}

	window.on('console', e => logger.log(`Playwright (Electron): window.on('console') [${e.text()}]`));
	window.on('pageerror', async (error) => logger.log(`Playwright (Electron) ERROR: page error: ${error}`));
	window.on('response', async (response) => {
		if (response.status() >= 400) {
			logger.log(`Playwright (Electron) ERROR: HTTP status ${response.status()} for ${response.url()}`);
		}
	});

	return { electron, context, page: window };
}
```

**Variations / call-sites:**
- `test/automation/src/code.ts:89-109` — Wrapper that registers process instance and returns Code object
- `test/smoke/src/main.ts:191-204` — Resolves electronPath from build or dev sources

**Relevance to Tauri port:** Playwright won't work with Tauri (it uses Chromium/Firefox/WebKit). Instead, Tauri apps will be tested via webview inspection APIs or a custom Tauri test driver. The key insight is that `--enable-smoke-test-driver` is a compile-time flag that activates test-specific code paths in the app (see `src/vs/sessions/electron-browser/sessions.ts:324`). A Tauri port will need an equivalent test-mode activation mechanism.

---

#### Pattern 7: Workbench Automation Abstraction Layer
**Where:** `test/automation/src/workbench.ts:31-74`, `test/automation/src/application.ts:23-75`
**What:** Composes facade objects for IDE components (Explorer, Editor, Terminal, Debug, SCM, Search, Extensions, Settings, Keybindings, Problems, Notebook, Chat, Task). Each facade encapsulates a domain (e.g., Terminal with enums for command IDs and selectors).

```typescript
export class Workbench {
	readonly quickaccess: QuickAccess;
	readonly quickinput: QuickInput;
	readonly editors: Editors;
	readonly explorer: Explorer;
	readonly activitybar: ActivityBar;
	readonly search: Search;
	readonly extensions: Extensions;
	readonly editor: Editor;
	readonly scm: SCM;
	readonly debug: Debug;
	readonly statusbar: StatusBar;
	readonly problems: Problems;
	readonly settingsEditor: SettingsEditor;
	readonly keybindingsEditor: KeybindingsEditor;
	readonly terminal: Terminal;
	readonly notebook: Notebook;
	readonly localization: Localization;
	readonly task: Task;
	readonly chat: Chat;

	constructor(code: Code) {
		this.editors = new Editors(code);
		this.quickinput = new QuickInput(code);
		this.quickaccess = new QuickAccess(code, this.editors, this.quickinput);
		// ... compose rest
	}
}
```

**Variations / call-sites:**
- `test/automation/src/application.ts:23-75` — Application wraps Code and Workbench, provides start/stop/restart/profiler access
- `test/smoke/src/areas/terminal/terminal.test.ts:26-32` — Tests access via `this.app.workbench.terminal`
- `test/automation/src/editor.ts:16-30` — Editor facade example: rename, findReferences, peekDefinition via command execution

**Relevance to Tauri port:** This abstraction is the public test API contract. A Tauri port must preserve these facades and their methods, but replace the underlying `Code` class (which drives Playwright) with a Tauri-aware driver that uses webview inspection + IPC commands.

---

#### Pattern 8: Command Execution via Keybinding Dispatch & Acceptance Callbacks
**Where:** `test/automation/src/code.ts:168-170`, `test/automation/src/playwrightDriver.ts:537-565`
**What:** Dispatches keybinding strings (e.g., `'ctrl+shift+p'`) by parsing modifiers and keys, mapping VS Code names to Playwright keys, pressing/releasing in sequence. Includes accept callback to verify keybinding effect before returning (prevents race conditions).

```typescript
async dispatchKeybinding(keybinding: string, accept: () => Promise<void>): Promise<void> {
	await this.driver.sendKeybinding(keybinding, accept);
}

async sendKeybinding(keybinding: string, accept?: () => Promise<void> | void) {
	const chords = keybinding.split(' ');
	for (let i = 0; i < chords.length; i++) {
		const chord = chords[i];
		if (i > 0) {
			await this.wait(100);
		}

		const keys = chord.split('+');
		const keysDown: string[] = [];
		for (let i = 0; i < keys.length; i++) {
			if (keys[i] in PlaywrightDriver.vscodeToPlaywrightKey) {
				keys[i] = PlaywrightDriver.vscodeToPlaywrightKey[keys[i]];
			}
			await this.page.keyboard.down(keys[i]);
			keysDown.push(keys[i]);
		}
		while (keysDown.length > 0) {
			await this.page.keyboard.up(keysDown.pop()!);
		}
	}

	await accept?.();
}
```

**Variations / call-sites:**
- `test/automation/src/editor.ts:31-40` — Example: rename command with accept waiting for rename input to appear
- `test/automation/src/terminal.ts:83-96` — Terminal command execution with location verification

**Relevance to Tauri port:** Keybinding dispatch requires webview access. Tauri can inject scripts to trigger events, but will need equivalent key-mapping tables and accept callback hooks for deterministic sequencing.

---

#### Pattern 9: Terminal Integration Testing with Buffer Polling
**Where:** `test/automation/src/terminal.ts:79-100+`, `test/smoke/src/areas/terminal/terminal-input.test.ts:34-45`
**What:** Polls terminal output buffer for regex patterns, runs commands in terminal with configurable newline handling, monitors command decoration states (placeholder, success, error), manages terminal tabs/splits.

```typescript
async runCommandInTerminal(commandLine: string, enterCommand: boolean = true): Promise<void> {
	// Get xterm selector and focus
	const xpath = await this.getXtermSelector();
	await this.code.waitAndClick(`${xpath} .xterm-screen`, 1, 1);

	// Type the command
	const commandWithCarriage = commandLine.split('\n').join('\r');
	await this.code.waitForTypeInEditor(xpath, commandWithCarriage);

	// Enter if required
	if (enterCommand) {
		await this.code.dispatchKeybinding('enter', async () => {
			await this.code.wait(200);
		});
	}
}

// From test:
await settingsEditor.addUserSetting('terminal.integrated.autoReplies', '{ "foo": "bar" }');
await terminal.createTerminal();
await writeTextForAutoReply('foo');
await terminal.waitForTerminalText(buffer => buffer.some(line => line.match(/foo.*bar/)));
```

**Variations / call-sites:**
- `test/automation/src/terminal.ts` — 350+ LOC of terminal-specific methods (rename, createTerminal, selectProfile, split, kill, etc.)
- `test/smoke/src/areas/terminal/terminal.test.ts:17-52` — Suite-level example with `afterEach` cleanup

**Relevance to Tauri port:** Terminal tests exercise PTY integration. The port must preserve the buffer inspection API (xterm.js or equivalent terminal emulator) and support the same polling/command execution patterns.

---

## Summary

The test harness encodes **7 critical architectural requirements** for a Tauri/Rust port:

1. **Process lifecycle** — Launch, monitor readiness, graceful shutdown with forceful timeout
2. **UI polling** — Retry logic with acceptance functions (20s default timeout)
3. **Configuration matrix** — CLI args for build paths, web/remote/headless modes, tracing
4. **Test fixture setup** — Per-suite Application instances, log directory isolation, random user-data dirs
5. **Electron args injection** — Test-mode flags, crash reporting, logging paths, extensions
6. **Test driver flag** — `--enable-smoke-test-driver` activates test-specific code in the app
7. **Workbench abstraction** — Facades for all IDE components (Explorer, Editor, Terminal, Debug, SCM, etc.)
8. **Keybinding dispatch** — Parse, map, and press keys with deterministic accept callbacks
9. **Terminal integration** — Buffer polling for output verification, command decoration tracking

A Tauri port must preserve this contract. The main substitution is replacing Playwright/Electron process control with Tauri invoke/listen APIs and webview inspection, while retaining the Mocha suite structure, Application/Workbench abstractions, and test data setup patterns.

No deprecated or anti-patterns found in this partition; these patterns are actively maintained and foundational to VS Code's test infrastructure.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
