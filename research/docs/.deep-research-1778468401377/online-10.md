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
