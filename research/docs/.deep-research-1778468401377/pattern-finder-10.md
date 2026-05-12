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

