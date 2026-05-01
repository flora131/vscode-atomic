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
