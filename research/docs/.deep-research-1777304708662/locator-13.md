# Partition 13: scripts/ — Runtime & Platform Integration Points

## Implementation

### Core Runtime Launchers
- `scripts/code.sh` — Primary Electron launcher for desktop IDE; sets NODE_ENV, VSCODE_DEV environment, loads built-in extensions via preLaunch
- `scripts/code.bat` — Windows equivalent of code.sh for Electron-based IDE execution
- `scripts/node-electron.sh` — Invokes Electron as Node runtime; used for build/dev tasks requiring ELECTRON_RUN_AS_NODE
- `scripts/node-electron.bat` — Windows variant for Electron-as-Node execution

### Remote & Server Infrastructure
- `scripts/code-server.sh` — Launches VS Code Server (headless) for remote connections; spawns server-main.js via Node
- `scripts/code-server.bat` — Windows launcher for VS Code Server with VSCODE_SERVER_PORT=9888
- `scripts/code-server.js` — Node entry point for server mode; handles --launch flag and server startup via server-main.js

### CLI & Command-Line Interface
- `scripts/code-cli.sh` — Node-based CLI launcher using Electron; runs out/cli.js with ELECTRON_RUN_AS_NODE and inspect debugging
- `scripts/code-cli.bat` — Windows CLI entry point

### Agent Host Protocol & IPC
- `scripts/code-agent-host.sh` — Bash wrapper spawning code-agent-host.js with preLaunch and Node runtime
- `scripts/code-agent-host.js` — Agent host server launcher; spawns agentHostServerMain.js via child_process; handles port, host, connection tokens, mock agent mode
- `scripts/sync-agent-host-protocol.ts` — TypeScript utility syncing protocol definitions from agent-host-protocol repo; applies indentation conversion, import deduplication, Microsoft copyright headers, tsfmt formatting

### Web & Browser-Based Frontends
- `scripts/code-web.sh` — Web IDE launcher using @vscode/test-web; serves VS Code in browser with optional web-playground extension
- `scripts/code-web.bat` — Windows wrapper for web launcher
- `scripts/code-web.js` — Node script spawning test-web server; downloads vscode-web-playground extension (v0.0.13) if needed; supports --playground, --host, --port, --extensionPath
- `scripts/code-sessions-web.sh` — Sessions web interface launcher
- `scripts/code-sessions-web.js` — HTTP server for sessions workbench; injects CSS module import map, supports mock E2E extensions, serves from out/ and node_modules/

### Performance & Profiling
- `scripts/code-perf.js` — Performance profiling wrapper; resolves executable paths for desktop/Electron builds across macOS/Linux/Windows; integrates with @vscode/vscode-perf for benchmarking

### Terminal/Xterm Integration
- `scripts/xterm-update.js` — Package.json updater for xterm and xterm addon modules (@xterm/xterm, @xterm/addon-clipboard, @xterm/addon-image, @xterm/addon-webgl, @xterm/addon-serialize, etc.); updates dependencies across root, remote/, and remote/web/ subdirectories

## Configuration

- `scripts/package.json` — Minimal package config declaring "type": "commonjs" for script execution environment
- `scripts/chat-simulation/config.jsonc` — Performance benchmarking config with perfRegression (baselineBuild, runsPerScenario, regressionThreshold, metricThresholds for timeToFirstToken/timeToComplete/layoutCount/etc.) and memLeaks thresholds

## Tests

### Unit & Integration Test Runners
- `scripts/test.sh` — Electron-based unit test runner; invokes test/unit/electron/index.js with crash reporter; sets ELECTRON_ENABLE_LOGGING
- `scripts/test.bat` — Windows unit test launcher
- `scripts/test-integration.sh` — Electron-based integration test harness; supports --run, --grep, --glob, --runGlob, --suite-filter flags; runs out/cli.js
- `scripts/test-integration.bat` — Windows integration test runner

### Web & Remote Integration Tests
- `scripts/test-web-integration.sh` — Browser-based integration tests via test/integration/browser; tests extension API, workspace, TypeScript, Playwright-based suites
- `scripts/test-web-integration.bat` — Windows web integration test runner
- `scripts/test-remote-integration.sh` — Remote connection integration tests; sets up temporary user data dir, crash dir, logs; tests vscode-remote:// authority with test extensions
- `scripts/test-remote-integration.bat` — Windows remote integration test runner

### Documentation Tests
- `scripts/test-documentation.sh` — Documentation validation script
- `scripts/test-documentation.bat` — Windows documentation test runner

### Chat Simulation & Performance Regression Tests
- `scripts/chat-simulation/test-chat-perf-regression.js` — Chat performance benchmark comparing builds; uses real copilot extension with IS_SCENARIO_AUTOMATION=1 and mock LLM server; measures prompt building, context gathering, tool resolution, rendering, GC, layout overhead; supports --runs, --scenario, --no-baseline, --resume flags
- `scripts/chat-simulation/test-chat-mem-leaks.js` — Memory leak detection for chat feature; cycles through scenarios (text, code blocks, tool calls, thinking, terminal, multi-turn) measuring heap growth; requires V8 heap snapshots
- `scripts/chat-simulation/merge-ci-summary.js` — CI log aggregator for chat simulation results

### Chat Simulation Common Utilities & Scenarios
- `scripts/chat-simulation/common/utils.js` — Shared benchmarking utilities; config loading, Electron path resolution, build environment setup, statistical functions (welchTTest, robustStats), VSCode launcher with ext-host inspector port management, repo root detection
- `scripts/chat-simulation/common/perf-scenarios.js` — Performance test scenario definitions with user interactions and LLM mock responses
- `scripts/chat-simulation/common/mock-llm-server.js` — Mock LLM server returning predefined responses for deterministic performance testing

## Examples / Fixtures

- `scripts/chat-simulation/fixtures/_chatperf_arrays.ts` — TypeScript fixture with array operations for chat perf testing
- `scripts/chat-simulation/fixtures/_chatperf_async.ts` — Async/await code fixture
- `scripts/chat-simulation/fixtures/_chatperf_errors.ts` — Error handling code fixture
- `scripts/chat-simulation/fixtures/_chatperf_event.ts` — Event-driven code fixture
- `scripts/chat-simulation/fixtures/_chatperf_lifecycle.ts` — Lifecycle hook fixture
- `scripts/chat-simulation/fixtures/_chatperf_strings.ts` — String manipulation fixture
- `scripts/chat-simulation/fixtures/_chatperf_types.ts` — Type definition fixture
- `scripts/chat-simulation/fixtures/_chatperf_uri.ts` — URI handling fixture

## Notable Clusters

- `scripts/chat-simulation/` — 15 files; comprehensive chat feature performance benchmarking, memory leak detection, and LLM mock infrastructure; includes config, utilities, fixtures, and regression test harness
- `scripts/` (root) — 31 files total; split between platform launchers (Electron desktop, Node server, web browser), CLI/agent infrastructure, and test runners for electron, integration, web, remote, and documentation

## Summary

The scripts directory contains critical platform integration layers essential to understanding VS Code's runtime architecture. Files are organized by execution context: Electron-based desktop (code.sh), headless Node servers (code-server.js, code-agent-host.js), browser-based web (code-web.js, code-sessions-web.js), and comprehensive test infrastructure spanning unit tests, integration tests (electron/web/remote), and specialized performance/memory benchmarking for the chat feature. Key findings for a Tauri/Rust port include:

1. **Electron Runtime Dependency**: code.sh, code.bat, and node-electron scripts tightly couple to Electron's binary and ELECTRON_RUN_AS_NODE pattern; would require Tauri-based native launcher and process management.

2. **Multi-Runtime Support**: Scripts support three distinct execution models (Electron desktop, headless Node server, browser web) — a Tauri port would need to maintain or consolidate these paths.

3. **Agent Host Protocol**: sync-agent-host-protocol.ts shows VS Code syncs external protocol definitions; indicates complex IPC/RPC architecture between extension host and main process that would need Rust bindings.

4. **Performance Infrastructure**: chat-simulation tests demonstrate CPU-intensive profiling (GC, layout metrics, long tasks) via V8 APIs — Tauri/Rust version would require equivalent Wayland/native profiling hooks.

5. **CLI vs Server Modes**: code-cli.sh and code-server.js show distinct headless pathways; both inherit from Electron entry points, suggesting deep entanglement with Electron's node integration.

6. **Test Matrix Complexity**: Integration tests (test-integration.sh, test-web-integration.sh, test-remote-integration.sh) run against both native Electron and browser targets; any rewrite must maintain backward-compatible test interfaces.
