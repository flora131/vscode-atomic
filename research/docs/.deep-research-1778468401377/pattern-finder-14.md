# Pattern Finder Report: Partition 14 - `scripts/` Directory
## Porting VS Code TS/Electron to Tauri/Rust

---

#### Pattern 1: Electron Binary Resolution & Launch
**Where:** `scripts/node-electron.sh:12-36`, `scripts/code.sh:19-52`, `scripts/code.bat:14-17`
**What:** Platform-specific Electron executable paths resolved from `.build/electron/` directory, using `product.json` metadata to determine bundle structure (macOS `.app/Contents/MacOS/`, Linux binary, Windows `.exe`).

```bash
# macOS: .app bundle structure
NAME=`node -p "require('./product.json').nameLong"`
EXE_NAME=`node -p "require('./product.json').nameShort"`
CODE="$ROOT/.build/electron/$NAME.app/Contents/MacOS/$EXE_NAME"

# Linux: single binary
NAME=`node -p "require('./product.json').applicationName"`
CODE=".build/electron/$NAME"
```

**Variations:** 
- `scripts/code-cli.sh:13-16` uses identical pattern for CLI launcher
- `scripts/code.bat:12-17` uses Windows batch with registry parsing fallback
- `scripts/code-perf.js:59-88` implements JS-based resolution with `getExePath()` function handling all platforms

**Porting Impact:** Tauri/Rust would define application bundle paths differently. Requires:
- Configuration of Tauri app name/identifier
- Platform-specific binary location mapping (Tauri dist structure, native executable)
- Removal of `.app/Contents/MacOS/` macOS-specific path building

---

#### Pattern 2: Environment Variable Configuration for Electron Runtime
**Where:** `scripts/code.sh:40-44`, `scripts/code.bat:23-26`, `scripts/code-cli.sh:39-43`
**What:** Electron-specific env vars set before launch: `ELECTRON_RUN_AS_NODE`, `ELECTRON_ENABLE_LOGGING`, `ELECTRON_ENABLE_STACK_DUMPING`, `VSCODE_DEV`, `VSCODE_CLI`.

```bash
export NODE_ENV=development
export VSCODE_DEV=1
export VSCODE_CLI=1
export ELECTRON_ENABLE_LOGGING=1
export ELECTRON_ENABLE_STACK_DUMPING=1
```

**Variations:**
- `scripts/code.sh` sets `NODE_ENV=development` and `VSCODE_DEV=1` for dev builds
- `scripts/test.sh:35-41` adds `ELECTRON_ENABLE_LOGGING=1` for test runs with crash reporter
- `scripts/code-web.js` and `scripts/code-server.js` pass env via `process.env` to spawned processes

**Porting Impact:** Tauri/Rust apps don't use these Electron flags. Equivalent configuration needed:
- Replace `ELECTRON_RUN_AS_NODE` with app startup mode flag (dev vs. production)
- Remove Electron-specific debug flags
- Move logging/stack dumping to Rust `env_logger` or tracing subscriber config
- Maintain `VSCODE_DEV` for existing tooling compatibility

---

#### Pattern 3: Child Process Spawning with Node Entry Points
**Where:** `scripts/code-web.js:87-104`, `scripts/code-server.js:40-68`, `scripts/code-agent-host.js:69-111`
**What:** Node.js `child_process.spawn()` pattern used to launch separate server processes (test-web, server-main.js, agentHostServerMain.js) as sibling processes with stdio inheritance.

```javascript
const proc = cp.spawn(process.execPath, [testWebLocation, ...runnerArguments], 
  { env, stdio: 'inherit' });
proc.on('exit', (code) => process.exit(code));

process.on('SIGINT', () => {
  proc.kill();
  process.exit(128 + 2);
});
```

**Variations:**
- `scripts/code-server.js:46-67` captures stdout to parse "Web UI available at" message before resolving Promise
- `scripts/code-agent-host.js:90-97` parses stdout for "READY:\d+" regex to signal readiness
- `scripts/code-web.js` uses `stdio: 'inherit'` for transparent output
- `scripts/xterm-update.js:78-80` uses `execSync()` for synchronous npm install calls
- `scripts/chat-simulation/common/utils.js:134-140` uses `spawnSync()` with 120-second timeout for extension installation

**Porting Impact:** Tauri doesn't typically spawn standalone Node processes. Options:
- Migrate Node entry points to Rust (embed logic in main app)
- Use Tauri's built-in `shell` command for external processes
- For dev: run servers externally, connect via IPC/HTTP
- Maintain Node-based dev servers during transition as optional external services

---

#### Pattern 4: Pre-launch Build Step
**Where:** `scripts/code.sh:29-30`, `scripts/code.bat:9`, `scripts/code-cli.sh:23-24`
**What:** Optional build step executed before main app launch, controlled by `VSCODE_SKIP_PRELAUNCH` env var, delegates to `build/lib/preLaunch.ts`.

```bash
if [[ -z "${VSCODE_SKIP_PRELAUNCH}" ]]; then
  node build/lib/preLaunch.ts
fi
```

**Variations:**
- All three scripts (bash, bat, cli) implement identical pattern
- No args passed to preLaunch; pure side-effect execution
- Early exit mechanism allows skipping in CI/fast-path scenarios

**Porting Impact:** Tauri build process differs. Needed changes:
- Port `build/lib/preLaunch.ts` logic to Rust build script (`build.rs`)
- Or invoke as external tool during `tauri build` phase
- Environment variable toggle (`VSCODE_SKIP_PRELAUNCH`) remains compatible

---

#### Pattern 5: Test Execution with Electron
**Where:** `scripts/test.sh:13-43`, `scripts/test-integration.sh:132-176`
**What:** Electron binary executed directly with test entry point (`test/unit/electron/index.js` for unit tests; special paths for extension host tests). Crash reporting directory configured, special user data dir created.

```bash
VSCODECRASHDIR=$ROOT/.build/crashes
VSCODEUSERDATADIR=`mktemp -d 2>/dev/null`

ELECTRON_ENABLE_LOGGING=1 \
  "$CODE" \
  test/unit/electron/index.js --crash-reporter-directory=$VSCODECRASHDIR
```

**Variations:**
- `scripts/test-integration.sh` uses `mktemp` for ephemeral user data directory
- `MOCHA_GREP` env var passed for test filtering
- Integration tests invoke Electron with paths like `extensions/vscode-api-tests/` and `--extensionTestsPath`
- Suites can be filtered with `should_run_suite()` function (lines 104-117)
- API tests run via `--enable-proposed-api=vscode.vscode-api-tests` flag

**Porting Impact:** Tauri testing requires:
- Migrate unit tests from Electron-specific (`test/unit/electron/`) to web/rust test runners
- Port test harness from `test/unit/electron/index.js` to Tauri test framework
- Replace Electron crash reporting with Rust panic handling/logging
- Extension host tests may require significant refactoring to run without Electron

---

#### Pattern 6: Process Lifecycle & Signal Handling
**Where:** `scripts/code-web.js:93-99`, `scripts/code-server.js:56-66`, `scripts/code-agent-host.js:99-109`
**What:** Uniform signal handling pattern: parent process kills child on SIGINT/SIGTERM, forwards exit code, prevents zombie processes.

```javascript
proc.on('exit', (code) => process.exit(code));

process.on('exit', () => proc.kill());
process.on('SIGINT', () => {
  proc.kill();
  process.exit(128 + 2); // SIGINT exit code
});
process.on('SIGTERM', () => {
  proc.kill();
  process.exit(128 + 15); // SIGTERM exit code
});
```

**Variations:** All three files implement identical pattern with explicit exit codes (128 + signal number).

**Porting Impact:** Tauri's process management differs:
- Rust's `std::process::Command` has different signal semantics
- May use tokio/async process spawning instead
- Exit codes must remain compatible for shell scripts/CI expectations

---

#### Pattern 7: Chat Simulation & Performance Testing Infrastructure
**Where:** `scripts/chat-simulation/test-chat-mem-leaks.js:262-362`
**What:** Complex test harness using Playwright to drive VS Code UI, spawns Electron via `launchVSCode()`, uses Chrome DevTools Protocol (CDP) for heap profiling, measures memory deltas across iterations.

```javascript
const proc = cp.spawn(process.execPath, [testWebLocation, ...runnerArguments], 
  { env, stdio: 'inherit' });

const cdp = await page.context().newCDPSession(page);
await cdp.send('HeapProfiler.enable');
const heapInfo = await cdp.send('Runtime.getHeapUsage');
```

**Variations:**
- `scripts/chat-simulation/common/utils.js:107-148` downloads test builds via `@vscode/test-electron`
- `scripts/chat-simulation/common/utils.js:134-140` uses `spawnSync()` for extension installation
- Electron/CDP integration is core to leak detection methodology

**Porting Impact:** High complexity. Tauri porting requires:
- Replace Playwright + Electron + CDP with Tauri test API or Webdriver integration
- Rust-based memory profiling (potentially using `valgrind`, `heaptrack`, or Rust allocator hooks)
- Performance testing framework redesign; may be incompatible with current Playwright approach
- Consider moving performance tests to Rust unit tests with `criterion` crate

---

## Summary

The `scripts/` directory contains **7 major pattern categories** tied to Electron/Node.js infrastructure:

1. **Electron binary resolution** (platform-specific path building) — Must be replaced with Tauri app structure
2. **Electron-specific environment variables** — Must be mapped to Tauri equivalents or removed
3. **Node process spawning** — Can be migrated to Tauri's command execution or Rust code
4. **Pre-launch build steps** — Can move to Rust `build.rs` or remain external
5. **Test execution** — Requires full test runner rewrite for non-Electron target
6. **Process lifecycle management** — Rust has different signal handling; must adapt
7. **Performance testing infrastructure** — Most complex; likely requires new framework (Playwright → Tauri test API + Rust profiling)

**Overall porting effort:** Medium to High. The launcher scripts are straightforward (can be mostly rewritten in shell or Rust), but the test infrastructure (especially chat simulation with CDP) is deeply coupled to Electron and Chromium internals. Performance testing would require substantial redesign.

**Preserved patterns:** `VSCODE_DEV`, `VSCODE_CLI`, and `product.json` metadata resolution can likely remain compatible during transition.
