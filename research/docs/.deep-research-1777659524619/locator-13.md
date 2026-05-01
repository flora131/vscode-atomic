# VS Code Core IDE Porting Research - Scripts Directory Locator

## Research Question
What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

## Partition 13 Scope Analysis
`scripts/` directory (46 files, 9,079 LOC)

This partition contains development and runtime launch scripts that gate VS Code startup across multiple deployment targets. The files reveal the current architecture's dependency on Electron and Node.js, essential context for understanding a Tauri/Rust port.

---

## Implementation

### Launch Scripts (Electron-Gated)
- `scripts/code.sh` - Primary Electron-based development launch (macOS/Linux), gates `build/lib/preLaunch.ts`
- `scripts/node-electron.sh` - Node-as-Electron wrapper for headless CLI operations
- `scripts/code-cli.sh` - CLI mode with Electron, sets `ELECTRON_RUN_AS_NODE=1`, gates inspection via `--inspect=5874`
- `scripts/code.bat` - Windows equivalent to code.sh, parses `product.json` for executable name
- `scripts/node-electron.bat` - Windows Node-as-Electron wrapper
- `scripts/code-cli.bat` - Windows CLI mode

### Server Runtime Launchers
- `scripts/code-server.sh` - Node.js-based remote server launcher, spawns `out/server-main.js`
- `scripts/code-server.js` - JavaScript wrapper managing server startup, listens on port 9888, opens browser via `open` package
- `scripts/code-server.bat` - Windows server launcher with Node path resolution
- `scripts/code-web.sh` - Web-based VS Code launcher, uses `@vscode/test-web` package
- `scripts/code-web.js` - JavaScript server using `@vscode/test-web`, handles `--playground` mode, configures host/port
- `scripts/code-web.bat` - Windows web launcher
- `scripts/code-sessions-web.sh` - Sessions-based web launcher
- `scripts/code-sessions-web.js` - HTTP server implementation (100+ lines), serves sessions HTML at root path
- `scripts/code-agent-host.sh` - Agent host server launcher for remote execution
- `scripts/code-agent-host.js` - Agent host server wrapper, spawns `out/vs/platform/agentHost/node/agentHostServerMain.js`

### Performance & Utilities
- `scripts/code-perf.js` - Performance benchmarking tool using `@vscode/vscode-perf` package, supports desktop/web builds
- `scripts/xterm-update.js` - Terminal emulator dependency updater for `@xterm/*` packages (1017 lines)
- `scripts/sync-agent-host-protocol.ts` - TypeScript type definition synchronizer from sibling `agent-host-protocol` repo (>100 lines)

---

## Tests

### Integration Test Runners
- `scripts/test-integration.sh` - Main integration test orchestrator (338 lines), gates multiple test suites:
  - Node.js integration tests (mocha-based)
  - Extension host tests via `$INTEGRATION_TEST_ELECTRON_PATH`
  - Supports filtering by `--run`, `--runGlob`, `--grep`, `--suite`
  - Runs API tests, TypeScript, Markdown, Emmet, Git, Ipynb, Configuration editing, GitHub auth, Copilot tests
  - CSS/HTML language server tests via standalone CommonJS
- `scripts/test-integration.bat` - Windows integration test equivalent (12,668 bytes)
- `scripts/test-remote-integration.sh` - Remote integration tests (131 lines), uses `VSCODE_REMOTE_SERVER_PATH`
- `scripts/test-remote-integration.bat` - Windows remote integration variant
- `scripts/test-web-integration.sh` - Browser-based integration tests (67 lines), uses `test/integration/browser/out/index.js`
- `scripts/test-web-integration.bat` - Windows web integration variant
- `scripts/test.sh` - Unit test runner (44 lines), gates Electron launch for `test/unit/electron/index.js`
- `scripts/test.bat` - Windows unit test runner
- `scripts/test-documentation.sh` - Release documentation tests (22 lines), runs `**/*.releaseTest.js` glob
- `scripts/test-documentation.bat` - Windows documentation test variant

### Chat Performance & Memory Tests
- `scripts/chat-simulation/test-chat-perf-regression.js` - Chat performance regression benchmark (1935 lines)
- `scripts/chat-simulation/test-chat-mem-leaks.js` - Memory leak detection for chat (466 lines)
- `scripts/chat-simulation/merge-ci-summary.js` - CI result aggregation tool (566 lines)

---

## Configuration

### Chat Simulation Configuration
- `scripts/chat-simulation/config.jsonc` - Performance regression and memory leak thresholds:
  - `perfRegression.baselineBuild`: "1.116.0" version target
  - `perfRegression.runsPerScenario`: 5 iterations
  - `perfRegression.regressionThreshold`: 20% by default
  - `memLeaks.leakThresholdMB`: 10 MB max acceptable growth
  - Per-metric overrides for timeToFirstToken, timeToComplete, layout metrics

### NPM Configuration
- `scripts/package.json` - Minimal package.json for scripts directory (type: commonjs)

---

## Types / Interfaces

### Chat Simulation Type Fixtures
- `scripts/chat-simulation/fixtures/_chatperf_types.ts` - Type guard functions (isString, isNumber, isBoolean, isUndefined, isDefined)
- `scripts/chat-simulation/fixtures/_chatperf_errors.ts` - Error-related type fixtures
- `scripts/chat-simulation/fixtures/_chatperf_arrays.ts` - Array manipulation type fixtures
- `scripts/chat-simulation/fixtures/_chatperf_event.ts` - Event handling type fixtures
- `scripts/chat-simulation/fixtures/_chatperf_async.ts` - Async/Promise type fixtures
- `scripts/chat-simulation/fixtures/_chatperf_lifecycle.ts` - Lifecycle hook type fixtures
- `scripts/chat-simulation/fixtures/_chatperf_uri.ts` - URI handling type fixtures
- `scripts/chat-simulation/fixtures/_chatperf_strings.ts` - String manipulation type fixtures

---

## Examples / Fixtures

### Chat Performance Scenarios
- `scripts/chat-simulation/common/perf-scenarios.js` - Built-in scenario definitions for chat benchmarks (>100 lines):
  - Text-only scenario (4 paragraphs)
  - Large TypeScript code block scenario
  - Many-small-chunks scenario (200 words at 5ms intervals)
  - Mixed markdown + code + fix suggestion scenario
- `scripts/chat-simulation/common/utils.js` - Shared utilities for benchmarks (>100 lines):
  - Config loading with JSONC comment stripping
  - Electron path resolution
  - Repo root derivation from executable paths
  - Version string detection
  - Built-in extensions directory resolution
- `scripts/chat-simulation/common/mock-llm-server.js` - Mock LLM server for testing (1017 lines)

---

## Documentation

### Inline Documentation
- `scripts/sync-agent-host-protocol.ts` - Header comments detailing transformation pipeline:
  - 2-space to tab indentation conversion
  - Duplicate import merging
  - TypeScript formatting with tsfmt.json
  - Microsoft copyright header addition
- `scripts/code-server.js` - Help text documenting `--launch` flag
- `scripts/code-web.js` - Help text for `--playground`, folder mount path options
- `scripts/code-agent-host.js` - Comprehensive help output for port, host, connection token, mock agent, logging options
- `scripts/code-perf.js` - Build/runtime configuration documentation
- `scripts/code-sessions-web.js` - Help output for host, port, no-open, skip-welcome, mock options

### Script-Level Documentation
- `scripts/test-integration.sh` - Extensive help system (lines 56-94) documenting:
  - `--run` for single file tests
  - `--runGlob` for path pattern selection
  - `--grep` for test name filtering
  - `--suite` for extension host suite selection
  - Known suite list and examples

---

## Notable Clusters

### Multi-Platform Launch Ecosystem
The codebase demonstrates coordinated shell (.sh) and batch (.bat) pairs:
- `code.sh` / `code.bat` - Desktop Electron entry points
- `code-server.sh` / `code-server.bat` - Remote server entry points
- `code-web.sh` / `code-web.bat` - Web entry points
- `node-electron.sh` / `node-electron.bat` - Headless Node/Electron
- `test*.sh` / `test*.bat` - Test runners for each integration variant

This mirrors the current Electron-based architecture with per-platform executables.

### Electron Dependency Graph
Key gating mechanisms reveal Electron tight integration:
1. **Build stage**: `build/lib/preLaunch.ts` (conditional via `VSCODE_SKIP_PRELAUNCH`)
2. **Resolution**: `product.json` parsing for executable name/path
3. **Execution**: Direct Electron binary invocation with environment variables:
   - `NODE_ENV=development`
   - `VSCODE_DEV=1`
   - `ELECTRON_RUN_AS_NODE=1` (for CLI)
   - `ELECTRON_ENABLE_LOGGING=1`
   - `ELECTRON_ENABLE_STACK_DUMPING=1`
4. **Deployment**: `.build/electron/<app>/` directory structure

### Server Runtime Variants
Three distinct runtime modes share Node.js infrastructure:
- **Classic Server**: `out/server-main.js` (remote VS Code server)
- **Web Server**: `@vscode/test-web` package (browser-based)
- **Agent Host**: `out/vs/platform/agentHost/node/agentHostServerMain.js` (remote execution)
- **Sessions Web**: Custom HTTP server (memfs://, file serving, CSS module injection)

Each spawns via `child_process.spawn()` with environment inheritance.

### Performance Testing Infrastructure
The `chat-simulation/` directory (8 files) provides isolated performance benchmarking:
- Configuration-driven thresholds (JSONC)
- Fixture library with 8 TypeScript files for stable perf testing
- Common utilities for Electron path discovery and config loading
- Regression tracking and memory leak detection
- CI result aggregation

This infrastructure is critical for detecting performance regressions during major rewrites.

### Integration Test Orchestration
The test suite is complex and multi-layered (338+ lines for test-integration.sh):
- **Node.js tests**: Run directly via custom test runner
- **Extension host tests**: Launched via Electron (`$INTEGRATION_TEST_ELECTRON_PATH`)
- **Suite filtering**: Pattern-based selection (api-folder, typescript, git, emmet, etc.)
- **Temporary environment**: Isolation via temp directories and user data dirs
- **Crash/log collection**: Automatic directory creation at `.build/crashes` and `.build/logs/`

---

## Porting Implications for Tauri/Rust

### Architecture Dependencies Identified

1. **Electron Runtime Replace**: All `code*.sh/bat` and `node-electron*.sh/bat` scripts directly invoke Electron binaries. A Tauri rewrite would:
   - Replace Electron binary with Tauri CLI output (single executable)
   - Eliminate `product.json` executable name parsing (fixed in Tauri config)
   - Simplify multi-platform logic (Tauri handles compilation per target)
   - Remove `ELECTRON_RUN_AS_NODE` pattern (Rust processes don't need this hack)

2. **Server Mode Launch Changes**: `code-server.js`, `code-web.js`, `code-agent-host.js` and `code-sessions-web.js` all spawn Node.js processes:
   - `code-server.js` spawns `out/server-main.js` (must be Rust-based HTTP server)
   - `code-web.js` uses `@vscode/test-web` npm package (frontend framework, relocate to Rust webview)
   - `code-sessions-web.js` custom HTTP server with CSS module injection (integrate into Rust server)
   - `code-agent-host.js` spawns agent host main (needs Rust reimplementation)

3. **Build/Launch Integration**: All scripts gate on `build/lib/preLaunch.ts`:
   - Current: Pre-flight Electron + dependency checks
   - For Tauri: Could become Rust build-time checks or runtime initialization
   - Alternative: Eliminate if Rust build handles dependencies

4. **Test Infrastructure Rewrite**: 
   - Integration tests use Electron launch via `$INTEGRATION_TEST_ELECTRON_PATH`
   - Would need Tauri/Rust equivalent entry points
   - `test/integration/browser` would leverage Tauri webview instead of @vscode/test-web
   - Performance benchmarking would remain the same (external benchmark tool)

5. **Development Mode Complexity**: 
   - Current: `VSCODE_DEV=1` flag enables debug logging, source maps, etc.
   - Tauri equivalent: Rust debug/release builds, browser DevTools via webview
   - Environment variable pattern could remain but map to Rust feature flags

6. **Chat Simulation Benchmarks**: 
   - Currently run Electron with mock LLM server
   - Tauri version would spawn Rust binary instead
   - Fixture files (TypeScript) could remain as test inputs (no Rust recompilation needed)

### Critical File Rewrites Required

**Must Rewrite**:
- All `code*.sh` and `code*.bat` → Single Tauri CLI with subcommands
- `node-electron*.sh/bat` → Removed (Rust process spawning)
- `code-server.js` → Integrate into main Rust binary
- `code-web.js` → Webview configuration (Tauri builder)
- `code-agent-host.js` → Rust rewrite
- `code-sessions-web.js` → Rust HTTP handler
- `code-perf.js` → Rust benchmark harness (or keep as external)

**Can Keep/Adapt**:
- `sync-agent-host-protocol.ts` → Rust generation tool (minor adaptation)
- `xterm-update.js` → Keep (external dependency manager)
- Test scripts → Rewrite to call Tauri binary instead of Electron
- Chat simulation fixtures → Keep as test inputs
- Configuration files → Adapt to Rust (TOML instead of JSONC)

---

## Summary

The `scripts/` directory reveals VS Code's strong architectural coupling to **Electron and Node.js**. Every execution path—desktop, server, web, agent—chains through shell scripts that invoke Electron binaries or Node.js processes. 

A successful Tauri/Rust port would require:

1. **Consolidating 13+ shell/batch script pairs into 1-2 Rust binaries** with subcommand dispatch
2. **Reimplementing 4 JavaScript launcher wrappers** as Rust HTTP servers or process managers
3. **Rewriting all test infrastructure** to target Rust binaries instead of Electron
4. **Adapting performance benchmarking** to measure Rust code paths
5. **Eliminating ~5,000+ LOC of shell script boilerplate** through unified Rust build/deployment

The performance testing infrastructure (`chat-simulation/`) shows mature benchmark capabilities already in place, which would translate well to measuring Rust-based performance.

**Effort Estimate**: High (6-12 months). Not because of complexity, but because every execution path and test harness must be touched. The scripts directory is the "deployment surface" of the IDE—changing it means revalidating the entire application lifecycle.

