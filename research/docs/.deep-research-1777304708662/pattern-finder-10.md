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

