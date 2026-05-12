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
