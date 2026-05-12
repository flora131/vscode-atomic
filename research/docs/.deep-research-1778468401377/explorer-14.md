# Partition 14 of 80 — Findings

## Scope
`scripts/` (46 files, 9,079 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locations for Tauri/Rust Porting Analysis: scripts/ Directory

## Overview
The `scripts/` directory contains 46 files with ~9,079 LOC across multiple categories. These are development launcher scripts, test runners, and build automation. Key finding: **No direct Electron require() statements detected**, but extensive Electron path resolution and spawning logic throughout.

## Implementation

### Launcher & Dev Scripts (Electron-dependent)
- `scripts/code.sh` — Main launcher (2.8K); compiles, runs pre-launch tasks, invokes `.build/electron/<NAME>` binary
- `scripts/code.bat` — Windows equivalent; references `.build\electron\%NAMESHORT%`
- `scripts/code-cli.sh` — CLI entry point; invokes Electron via shell
- `scripts/code-cli.bat` — Windows CLI launcher
- `scripts/node-electron.sh` — ELECTRON_RUN_AS_NODE wrapper; spawns Electron binary as Node.js host
- `scripts/node-electron.bat` — Windows Electron-as-Node runner
- `scripts/code-web.sh` — Web dev server launcher (uses @vscode/test-web npm package)
- `scripts/code-web.bat` — Windows web launcher
- `scripts/code-web.js` — (168 LOC) Spawns `@vscode/test-web` via child_process; manages web playground extension downloads

### Server & Agent Scripts
- `scripts/code-server.sh` — Spawns server-main.js via Node.js (no direct Electron reference)
- `scripts/code-server.bat` — Windows server launcher
- `scripts/code-server.js` — (72 LOC) Spawns `out/server-main.js` via child_process; opens browser
- `scripts/code-agent-host.sh` — Agent host dev launcher; invokes pre-launch
- `scripts/code-agent-host.js` — (114 LOC) Spawns agentHostServerMain.js; CLI for --port, --host, --connection-token args

### Performance & Benchmarking
- `scripts/code-perf.js` — (98 LOC) Wrapper for @vscode/vscode-perf; resolves Electron build paths for macOS/Linux/Windows
  - Desktop path resolution logic: `.app/Contents/MacOS/<EXE>` (macOS), `<NAME>` (Linux), `<NAME>.exe` (Windows)
- `scripts/chat-simulation/test-chat-perf-regression.js` — (600+ LOC) Complex perf test framework
  - Heavy Electron path detection logic (detectBuildMode function)
  - Spawns VS Code via child_process with performance telemetry
  - Mock LLM server interaction
  - Handles version detection from filesystem paths

### Web & Sessions Scripts
- `scripts/code-sessions-web.sh` — Sessions web dev launcher
- `scripts/code-sessions-web.js` — (176 LOC) HTTP server; serves sessions workbench; CSS module collection; import maps

### Maintenance & Sync Scripts
- `scripts/sync-agent-host-protocol.ts` — (235 LOC) TypeScript → TypeScript protocol sync utility
  - Copies/transforms type definitions from sibling `agent-host-protocol` repo
  - Indentation conversion (spaces→tabs), import merging, TypeScript formatting
  - **Not Electron-dependent**
- `scripts/xterm-update.js` — (102 LOC) NPM dependency updater for xterm modules
  - Platform-agnostic; no Electron references

## Tests

### Integration Test Runners
- `scripts/test-integration.sh` — (80+ LOC) Node.js integration test orchestrator
  - Spawns `code.sh` (Electron) with test paths; handles suite filtering
  - Runs CSS/HTML language servers via node-electron.sh
- `scripts/test-integration.bat` — Windows integration test runner
- `scripts/test-remote-integration.sh` — (80+ LOC) Remote integration tests
  - Uses `INTEGRATION_TEST_ELECTRON_PATH` (defaults to `./scripts/code.sh`)
  - Spawns Electron via child_process with remote resolver extensions
- `scripts/test-remote-integration.bat` — Windows remote integration tests
- `scripts/test-web-integration.sh` — Web integration test runner
- `scripts/test-web-integration.bat` — Windows web integration tests
- `scripts/test.sh` — (44+ LOC) Unit test runner
  - Invokes Electron at `.build/electron/<NAME>` with test/unit/electron/index.js
  - Platform-aware path resolution
- `scripts/test.bat` — Windows unit test runner
  - Same Electron path logic; uses product.json for app name
- `scripts/test-documentation.sh` — Documentation tests (non-Electron)
- `scripts/test-documentation.bat` — Windows documentation tests

### Chat Simulation Tests
- `scripts/chat-simulation/test-chat-perf-regression.js` — See Implementation above
- `scripts/chat-simulation/test-chat-mem-leaks.js` — (300+ LOC) Memory leak detection during chat scenarios

## Configuration

### Chat Simulation Config
- `scripts/chat-simulation/config.jsonc` — Performance thresholds, baseline versions, leak detection limits
  - Baseline: 1.116.0
  - Regression threshold: 20% by default
  - Per-metric overrides for timeToFirstToken, layoutCount, etc.

## Types / Interfaces

### Chat Perf Fixtures (TypeScript)
- `scripts/chat-simulation/fixtures/_chatperf_types.ts` — Type definitions for chat perf tests
- `scripts/chat-simulation/fixtures/_chatperf_errors.ts` — Error scenario payloads
- `scripts/chat-simulation/fixtures/_chatperf_arrays.ts` — Array/collection test data
- `scripts/chat-simulation/fixtures/_chatperf_event.ts` — Event/lifecycle fixtures
- `scripts/chat-simulation/fixtures/_chatperf_async.ts` — Async/promise test fixtures
- `scripts/chat-simulation/fixtures/_chatperf_strings.ts` — String processing fixtures
- `scripts/chat-simulation/fixtures/_chatperf_lifecycle.ts` — Lifecycle/hook fixtures
- `scripts/chat-simulation/fixtures/_chatperf_uri.ts` — URI/path fixtures

## Examples / Fixtures

### Chat Simulation Utils & Mocks
- `scripts/chat-simulation/common/perf-scenarios.js` — (100+ LOC) Scenario builders
  - Text-only, large-codeblock, many-small-chunks, mixed-content scenarios
  - ScenarioBuilder class for streaming chunk simulation
- `scripts/chat-simulation/common/mock-llm-server.js` — Mock LLM server for testing
- `scripts/chat-simulation/merge-ci-summary.js` — CI log aggregator

## Notable Clusters

### Electron Integration Points
The scripts directory has **systematic Electron spawning patterns**:
1. **Shell launchers** (`code.sh`, `code.bat`) → read product.json → resolve `.build/electron/<APP>` path → exec
2. **JS wrappers** (`code-server.js`, `code-agent-host.js`, `code-perf.js`) → child_process.spawn() → out/ entry points
3. **Test runners** (`test*.sh`, `test*.bat`) → Electron path resolution → pass test entry points
4. **Build mode detection** (test-chat-perf-regression.js) → filesystem path heuristics

### Platform Abstraction Layer
- All scripts use conditional logic for macOS, Linux, Windows
- Use `product.json` for executable name resolution
- Shell scripts with OSX Darwin conditionals; batch files for Windows
- Critical for Tauri: must replace product.json lookup + Electron path resolution with Tauri binary path

### Package.json
- `scripts/package.json` — Minimal; only `"type": "commonjs"`

## Research Question Alignment: Porting to Tauri/Rust

**What would need to be rewritten:**

1. **Shell launchers** (code.sh, code.bat, node-electron.sh)
   - Replace Electron binary path resolution with Tauri binary path
   - Tauri CLI may provide replacement commands or must be integrated into shell scripts
   - ELECTRON_RUN_AS_NODE logic → Tauri native Rust process execution

2. **JS launcher wrappers** (code-server.js, code-agent-host.js, code-perf.js, code-web.js)
   - child_process.spawn() calls → must be rewritten for Tauri command execution
   - Product.json-based path resolution → Tauri app manifest or compiled binary path
   - Build artifact paths (.build/electron/) → may migrate to dist/ or Tauri build output

3. **Test runners** (test*.sh, test*.bat, test-chat-perf-regression.js)
   - Heavy Electron path detection → replace with Tauri build artifact discovery
   - Electron-specific env vars (ELECTRON_ENABLE_LOGGING, ELECTRON_RUN_AS_NODE) → replace with Tauri equivalents
   - test-chat-perf-regression.js's detectBuildMode() function → adapt to Tauri version detection

4. **Minimal rewrite needed:**
   - sync-agent-host-protocol.ts (pure TypeScript transformation)
   - xterm-update.js (package update utility)
   - Chat simulation fixtures and scenarios (test data)
   - config.jsonc (configuration remains relevant)

**Architectural impact:**
- Removes dependency on Electron binary in development workflow
- Requires Tauri binary build system integration
- May simplify cross-platform scripting (Tauri handles platform abstraction)
- Performance testing infrastructure must target Tauri app instead of Electron

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
