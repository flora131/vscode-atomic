# Partition 13 of 79 — Findings

## Scope
`scripts/` (46 files, 9,079 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Porting VS Code to Tauri/Rust: Architectural Patterns from Scripts

**Research Question**: What patterns exist in the codebase demonstrating how VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation) is currently structured?

**Scope**: `scripts/` directory analysis (46 files, 9,079 LOC)

---

## Findings

The scripts directory reveals several critical architectural patterns that inform what a Tauri/Rust port would need to address:

### Pattern 1: Multi-Process Architecture with Child Process Management
**Where:** `scripts/code-web.js:87-104`, `scripts/code-server.js:40-69`, `scripts/code-agent-host.js:69-111`
**What:** VS Code spawns separate Node.js processes for different concerns (test-web server, server, agent host) using stdio management.

```javascript
function startServer(runnerArguments) {
	const env = { ...process.env };

	console.log(`Starting @vscode/test-web: ${testWebLocation} ${runnerArguments.join(' ')}`);
	const proc = cp.spawn(process.execPath, [testWebLocation, ...runnerArguments], { env, stdio: 'inherit' });

	proc.on('exit', (code) => process.exit(code));

	process.on('exit', () => proc.kill());
	process.on('SIGINT', () => {
		proc.kill();
		process.exit(128 + 2);
	});
	process.on('SIGTERM', () => {
		proc.kill();
		process.exit(128 + 15);
	});
}
```

**Variations / call-sites:**
- `scripts/code-server.js:46` — Spawns Node process with stdin/stderr piping for server
- `scripts/code-agent-host.js:86-89` — Spawns agent host with custom stdio configuration
- `scripts/code-web.js:91` — Test-web server spawn with inherited stdio

**Implication for Tauri/Rust**: The core architecture relies on spawning separate processes for different subsystems. A Rust port would need to replace these with either native Rust tasks/threads or embedded Tauri processes.

---

### Pattern 2: Platform-Specific Executable Resolution
**Where:** `scripts/code-perf.js:59-88`
**What:** VS Code handles platform-specific executable locations (macOS .app bundles, Linux binaries, Windows .exe files) with dynamic product.json lookups.

```javascript
function getExePath(buildPath) {
	buildPath = path.normalize(path.resolve(buildPath));
	if (buildPath === path.normalize(getLocalCLIPath())) {
		return buildPath;
	}
	let relativeExePath;
	switch (process.platform) {
		case 'darwin': {
			const product = require(path.join(buildPath, 'Contents', 'Resources', 'app', 'product.json'));
			relativeExePath = path.join('Contents', 'MacOS', product.nameShort);
			if (!fs.existsSync(path.join(buildPath, relativeExePath))) {
				relativeExePath = path.join('Contents', 'MacOS', 'Electron');
			}
			break;
		}
		case 'linux': {
			const product = require(path.join(buildPath, 'resources', 'app', 'product.json'));
			relativeExePath = product.applicationName;
			break;
		}
		case 'win32': {
			const product = require(path.join(buildPath, 'resources', 'app', 'product.json'));
			relativeExePath = `${product.nameShort}.exe`;
			break;
		}
	}
	return buildPath.endsWith(relativeExePath) ? buildPath : path.join(buildPath, relativeExePath);
}
```

**Variations / call-sites:**
- `scripts/chat-simulation/common/utils.js:62-71` — Platform-specific Electron path construction
- `scripts/chat-simulation/common/utils.js:86-99` — Built-in extensions directory resolution per platform

**Implication**: Tauri inherently handles platform bundling better than Electron, but custom resource paths would still require similar pattern logic.

---

### Pattern 3: Signal Handling and Graceful Process Termination
**Where:** `scripts/code-web.js:95-103`, `scripts/chat-simulation/common/utils.js:515-577`
**What:** VS Code establishes signal handlers (SIGINT, SIGTERM) to coordinate graceful shutdown across multiple processes.

```javascript
process.on('exit', () => proc.kill());
process.on('SIGINT', () => {
	proc.kill();
	process.exit(128 + 2); // https://nodejs.org/docs/v14.16.0/api/process.html#process_signal_events
});
process.on('SIGTERM', () => {
	proc.kill();
	process.exit(128 + 15);
});
```

Extended pattern in launchVSCode:
```javascript
const quitKey = process.platform === 'darwin' ? 'Meta+KeyQ' : 'Alt+F4';
await page.keyboard.press(quitKey);
// ... wait for graceful exit, then kill if necessary
```

**Variations / call-sites:**
- `scripts/code-server.js:58-66` — Signal handling with same pattern
- `scripts/code-agent-host.js:101-109` — Consistent signal coordination
- `scripts/chat-simulation/common/utils.js:556-577` — Complex shutdown with trace flush coordination

**Implication**: Tauri/Rust shutdown must coordinate between the Rust backend and web frontend. Signal handling patterns show the need for explicit cleanup phases.

---

### Pattern 4: Environment Variable Configuration and Server Startup Coordination
**Where:** `scripts/code-agent-host.js:12-62`
**What:** CLI argument parsing and environment variable setup to configure server startup parameters (port, host, logging, authentication).

```javascript
async function main() {
	const args = minimist(process.argv.slice(2), {
		boolean: ['help', 'enable-mock-agent', 'quiet', 'without-connection-token'],
		string: ['port', 'host', 'log', 'connection-token', 'connection-token-file'],
	});

	const port = args.port || process.env['VSCODE_AGENT_HOST_PORT'] || '8081';

	/** @type {string[]} */
	const serverArgs = ['--port', String(port)];
	if (args.host) {
		serverArgs.push('--host', String(args.host));
	}
	if (args['enable-mock-agent']) {
		serverArgs.push('--enable-mock-agent');
	}
	// ... more args
	
	await startServer(serverArgs);
}
```

**Variations / call-sites:**
- `scripts/code-sessions-web.js:17-35` — HTTP server configuration with host/port handling
- `scripts/code-web.js:27-39` — Minimist-based argument parsing for extension paths

**Implication**: Tauri apps need CLI argument and environment configuration patterns, likely through Tauri's command system or a dedicated configuration layer.

---

### Pattern 5: Build-Time Type Synchronization and Protocol File Copying
**Where:** `scripts/sync-agent-host-protocol.ts:25-234`
**What:** TypeScript script that synchronizes type definitions from external repo, normalizes formatting, and applies transformations (indentation, import merging).

```typescript
function processFile(src: string, dest: string): void {
	let content = fs.readFileSync(src, 'utf-8');
	content = stripExistingHeader(content);

	// Merge duplicate imports from the same module
	content = mergeDuplicateImports(content);

	content = convertIndentation(content);
	content = content.split('\n').map(line => line.trimEnd()).join('\n');

	const header = `${COPYRIGHT}\n\n${BANNER}\n`;
	content = header + '\n' + content;

	if (!content.endsWith('\n')) {
		content += '\n';
	}

	const destPath = path.join(DEST_DIR, dest);
	fs.mkdirSync(path.dirname(destPath), { recursive: true });
	content = formatTypeScript(content, dest);
	fs.writeFileSync(destPath, content, 'utf-8');
	console.log(`  ${dest}`);
}
```

**Variations / call-sites:**
- Lines 111-169: Import deduplication and normalization logic
- Lines 92-104: Indentation conversion (spaces to tabs)
- Lines 38-58: TypeScript formatting using language service

**Implication**: Protocol synchronization between services suggests need for code generation and build-time tooling. A Rust port would likely use code generation for IPC contracts, potentially with similar validation and formatting patterns.

---

### Pattern 6: HTTP Server for Development and Testing Infrastructure
**Where:** `scripts/code-sessions-web.js:49-95`, `scripts/chat-simulation/common/mock-llm-server.js:354-594`
**What:** Custom HTTP servers for development (sessions web) and testing (mock LLM) with specific content-type handling and streaming responses.

Sessions Web Server:
```javascript
const server = http.createServer((req, res) => {
	const url = new URL(req.url, `http://${HOST}:${PORT}`);

	// Serve the sessions workbench HTML at the root
	if (url.pathname === '/' || url.pathname === '/index.html') {
		res.writeHead(200, { 'Content-Type': 'text/html' });
		res.end(getSessionsHTML(HOST, PORT, cssModules, args['mock']));
		return;
	}

	// Serve static files from the repo root
	const filePath = path.join(APP_ROOT, url.pathname);
	if (fs.existsSync(filePath) && fs.statSync(filePath).isFile()) {
		const ext = path.extname(filePath);
		const contentType = {
			'.js': 'application/javascript',
			'.mjs': 'application/javascript',
			'.css': 'text/css',
			'.html': 'text/html',
			'.json': 'application/json',
			'.svg': 'image/svg+xml',
			'.png': 'image/png',
			'.ttf': 'font/ttf',
			'.woff': 'font/woff',
			'.woff2': 'font/woff2',
		}[ext] || 'application/octet-stream';

		res.writeHead(200, {
			'Content-Type': contentType,
			'Access-Control-Allow-Origin': '*',
		});
		fs.createReadStream(filePath).pipe(res);
		return;
	}
});

server.listen(PORT, HOST, () => {
	console.log(`\n  Sessions Web running at: http://${HOST}:${PORT}/\n`);
});
```

Mock LLM Server (streaming patterns):
```javascript
async function streamContent(res, chunks, isScenarioRequest) {
	res.write(`data: ${JSON.stringify(makeInitialChunk())}\n\n`);

	for (const chunk of chunks) {
		if (chunk.delayMs > 0) { await sleep(chunk.delayMs); }
		res.write(`data: ${JSON.stringify(makeChunk(chunk.content, 0, false))}\n\n`);
	}

	res.write(`data: ${JSON.stringify(makeChunk('', 0, true))}\n\n`);
	res.write('data: [DONE]\n\n');
	res.end();

	if (isScenarioRequest) {
		serverEvents.emit('scenarioCompletion');
	}
}
```

**Variations / call-sites:**
- `scripts/chat-simulation/common/mock-llm-server.js:813-840` — SSE streaming for thinking + content
- `scripts/chat-simulation/common/mock-llm-server.js:849-879` — Tool call streaming with arguments fragmentation

**Implication**: Web-based features require streaming HTTP infrastructure (SSE, chunked responses). Tauri's built-in web framework would need to handle similar patterns or bridge to HTTP servers for backward compatibility.

---

### Pattern 7: Complex Scenario-Based Testing with Mock Services
**Where:** `scripts/chat-simulation/common/mock-llm-server.js:95-160`, `scripts/chat-simulation/common/perf-scenarios.js:32-160`
**What:** Scenario registry pattern for multi-turn interactions with scenario builder DSL and dynamic scenario lookup.

```javascript
class ScenarioBuilder {
	constructor() {
		/** @type {StreamChunk[]} */
		this.chunks = [];
	}

	emit(content) {
		this.chunks.push({ content, delayMs: 0 });
		return this;
	}

	wait(ms, content) {
		this.chunks.push({ content, delayMs: ms });
		return this;
	}

	stream(contents, delayMs = 15) {
		for (const content of contents) {
			this.chunks.push({ content, delayMs });
		}
		return this;
	}

	burst(contents) {
		return this.stream(contents, 0);
	}

	build() {
		return this.chunks;
	}
}
```

Scenario registration:
```javascript
/** @type {Record<string, StreamChunk[] | MultiTurnScenario>} */
const SCENARIOS = {};

function registerScenario(id, definition) {
	SCENARIOS[id] = definition;
}

function getScenarioIds() {
	return Object.keys(SCENARIOS);
}
```

**Variations / call-sites:**
- `scripts/chat-simulation/common/perf-scenarios.js:35-160` — Content scenario definitions with builder pattern
- `scripts/chat-simulation/common/mock-llm-server.js:708-779` — Dynamic scenario routing based on message tags and tool availability
- `scripts/chat-simulation/common/mock-llm-server.js:973-1015` — User turn and turn count management

**Implication**: Testing infrastructure for IDE features requires sophisticated scenario management, request routing, and state tracking across multi-turn interactions. Essential for validating IPC protocols in Rust backend.

---

### Pattern 8: Electron Binary Manipulation and Version Management
**Where:** `scripts/chat-simulation/common/utils.js:102-151`, `scripts/xterm-update.js:31-100`
**What:** Utilities to resolve Electron executables from version strings, download VS Code versions via @vscode/test-electron, and manage external dependencies.

```javascript
async function resolveBuild(buildArg) {
	if (!buildArg) {
		return getElectronPath();
	}
	if (isVersionString(buildArg)) {
		console.log(`[chat-simulation] Downloading VS Code ${buildArg}...`);
		const { downloadAndUnzipVSCode, resolveCliArgsFromVSCodeExecutablePath } = require('@vscode/test-electron');
		const exePath = await downloadAndUnzipVSCode(buildArg);
		console.log(`[chat-simulation] Downloaded: ${exePath}`);

		// Check if copilot is already bundled as a built-in extension
		const builtinExtDir = getBuiltinExtensionsDir(exePath);
		const hasCopilotBuiltin = builtinExtDir && fs.existsSync(builtinExtDir)
			&& fs.readdirSync(builtinExtDir).some(e => e === 'copilot');

		if (hasCopilotBuiltin) {
			console.log(`[chat-simulation] Copilot is bundled as a built-in extension`);
		} else {
			// Install copilot-chat from marketplace
			const extDir = path.join(DATA_DIR, 'extensions');
			fs.mkdirSync(extDir, { recursive: true });
			const [cli, ...cliArgs] = resolveCliArgsFromVSCodeExecutablePath(exePath);
			const result = spawnSync(cli, [...cliArgs, '--extensions-dir', extDir, '--install-extension', extId], {
				encoding: 'utf-8',
				stdio: 'pipe',
				shell: process.platform === 'win32',
				timeout: 120_000,
			});
		}

		return exePath;
	}
	return path.resolve(buildArg);
}
```

Dependency management pattern:
```javascript
function getLatestModuleVersion(moduleName) {
	return new Promise((resolve, reject) => {
		cp.exec(`npm view ${moduleName} versions --json`, { cwd: vscodeDir }, (err, stdout, stderr) => {
			if (err) { reject(err); }
			let versions = JSON.parse(stdout);
			if (typeof versions === 'string') { versions = [versions]; }
			resolve(versions[versions.length - 1]);
		});
	});
}
```

**Variations / call-sites:**
- `scripts/xterm-update.js:31-44` — npm package version querying and updating
- `scripts/xterm-update.js:47-99` — Multi-directory package.json updating (root, remote, remote/web)
- `scripts/chat-simulation/common/utils.js:163-173` — Storage pre-seeding with sqlite3

**Implication**: Version management, extension discovery, and dependency resolution are crucial IDE concerns. Tauri port would need equivalent mechanisms, potentially with Rust package management via cargo.

---

### Pattern 9: Playwright-Based Browser Automation for Testing
**Where:** `scripts/chat-simulation/common/utils.js:504-577`
**What:** Launch VS Code via Chromium CDP connection, manage browser lifecycle, implement graceful shutdown with trace flushing.

```javascript
async function launchVSCode(executable, launchArgs, env, opts = {}) {
	const { chromium } = require('playwright');
	const port = nextPort++;

	const args = [`--remote-debugging-port=${port}`, ...launchArgs];
	const isShell = process.platform === 'win32';

	const child = spawn(executable, args, {
		cwd: ROOT,
		env,
		shell: isShell,
		stdio: opts.verbose ? 'inherit' : ['ignore', 'ignore', 'ignore'],
	});

	// Track early exit
	let exitError = null;
	child.once('exit', (code, signal) => {
		if (!exitError) {
			exitError = new Error(`VS Code exited before CDP connected (code=${code} signal=${signal})`);
		}
	});

	// Wait for CDP
	try {
		await waitForCDP(port);
	} catch (e) {
		if (exitError) { throw exitError; }
		throw e;
	}

	const browser = await chromium.connectOverCDP(`http://127.0.0.1:${port}`);
	const page = await findWorkbenchPage(browser);

	return {
		page,
		browser,
		close: async () => {
			try {
				const quitKey = process.platform === 'darwin' ? 'Meta+KeyQ' : 'Alt+F4';
				await page.keyboard.press(quitKey);
			} catch {
				// Page may already be closed
			}
			const pid = child.pid;
			// Wait for graceful exit (up to 30s for trace flush)
			await new Promise(resolve => {
				const timer = setTimeout(() => {
					if (pid) {
						try { execSync(`pkill -9 -P ${pid}`, { stdio: 'ignore' }); }
						catch { }
					}
					child.kill('SIGKILL');
					resolve(undefined);
				}, 30_000);
				child.once('exit', () => { clearTimeout(timer); resolve(undefined); });
			});
			await browser.close().catch(() => { });
			// Kill crashpad handler
			await new Promise(r => setTimeout(r, 500));
			try { execSync('pkill -9 -f crashpad_handler.*vscode-chat-simulation', { stdio: 'ignore' }); }
			catch { }
		},
	};
}
```

**Variations / call-sites:**
- `scripts/chat-simulation/common/utils.js:452-463` — CDP endpoint waiting logic
- `scripts/chat-simulation/common/utils.js:473-489` — Workbench page detection across contexts

**Implication**: Browser automation for IDE testing is sophisticated. A Tauri port would need equivalent testing infrastructure, likely through webdriver or Tauri's test utilities.

---

### Pattern 10: Statistical Analysis and Performance Regression Detection
**Where:** `scripts/chat-simulation/common/utils.js:585-707`, `scripts/chat-simulation/test-chat-perf-regression.js:1-22`
**What:** Welch's t-test implementation for comparing performance metrics across builds, outlier removal, and statistical significance testing.

```javascript
function welchTTest(a, b) {
	if (a.length < 2 || b.length < 2) { return null; }
	const meanA = a.reduce((s, v) => s + v, 0) / a.length;
	const meanB = b.reduce((s, v) => s + v, 0) / b.length;
	const varA = a.reduce((s, v) => s + (v - meanA) ** 2, 0) / (a.length - 1);
	const varB = b.reduce((s, v) => s + (v - meanB) ** 2, 0) / (b.length - 1);
	const seA = varA / a.length;
	const seB = varB / b.length;
	const seDiff = Math.sqrt(seA + seB);
	if (seDiff === 0) { return null; }
	const t = (meanB - meanA) / seDiff;
	// Welch-Satterthwaite degrees of freedom
	const df = (seA + seB) ** 2 / ((seA ** 2) / (a.length - 1) + (seB ** 2) / (b.length - 1));
	const pValue = tDistPValue(t, df);
	const significant = pValue < 0.05;
	let confidence;
	if (pValue < 0.01) { confidence = 'high'; }
	else if (pValue < 0.05) { confidence = 'medium'; }
	else if (pValue < 0.1) { confidence = 'low'; }
	else { confidence = 'none'; }
	return { t: Math.round(t * 100) / 100, df: Math.round(df * 10) / 10, pValue: Math.round(pValue * 1000) / 1000, significant, confidence };
}
```

Outlier removal (IQR method):
```javascript
function removeOutliers(values) {
	if (values.length < 4) { return values; }
	const sorted = [...values].sort((a, b) => a - b);
	const q1 = sorted[Math.floor(sorted.length * 0.25)];
	const q3 = sorted[Math.floor(sorted.length * 0.75)];
	const iqr = q3 - q1;
	const lo = q1 - 1.5 * iqr;
	const hi = q3 + 1.5 * iqr;
	return sorted.filter(v => v >= lo && v <= hi);
}
```

**Variations / call-sites:**
- `scripts/chat-simulation/common/utils.js:615-647` — Beta incomplete function for t-distribution CDF
- `scripts/chat-simulation/common/utils.js:654-667` — Log-gamma via Lanczos approximation
- `scripts/chat-simulation/common/utils.js:713-736` — Robust statistics computation

**Implication**: Performance regression detection is mission-critical for IDE quality. Tauri/Rust port would need equivalent statistical infrastructure, likely ported to Rust or integrated via wasm.

---

## Summary

The scripts directory demonstrates that VS Code's architecture relies on:

1. **Multi-process coordination** with careful signal handling and graceful shutdown
2. **Platform-specific bundling** and executable resolution (critical for cross-platform support)
3. **Complex HTTP servers** for both development and production use cases
4. **Sophisticated testing infrastructure** with scenario-based mock services
5. **Build-time code generation** and protocol synchronization across services
6. **Statistical analysis** for performance validation
7. **Extension lifecycle management** and built-in extension handling
8. **CDP/browser automation** for headless testing
9. **Environment-driven configuration** with fallback defaults
10. **Version and dependency management** tied to Electron binary distribution

A Tauri/Rust port would need to address each of these architectural concerns, likely through different mechanisms but with equivalent functionality. The most challenging aspects would be:

- Replicating Electron's multi-process model in Tauri's web-based architecture
- Maintaining extension compatibility and discovery
- Implementing IPC protocols that replace Node.js child process communication
- Porting or reimplementing complex subsystems (language servers, debugging, terminals) from TypeScript to Rust

**Key Files Analyzed:**
- `/Users/norinlavaee/vscode-atomic/scripts/code-web.js` (168 lines)
- `/Users/norinlavaee/vscode-atomic/scripts/code-server.js` (72 lines)
- `/Users/norinlavaee/vscode-atomic/scripts/code-agent-host.js` (114 lines)
- `/Users/norinlavaee/vscode-atomic/scripts/sync-agent-host-protocol.ts` (235 lines)
- `/Users/norinlavaee/vscode-atomic/scripts/code-perf.js` (98 lines)
- `/Users/norinlavaee/vscode-atomic/scripts/code-sessions-web.js` (176 lines)
- `/Users/norinlavaee/vscode-atomic/scripts/chat-simulation/common/mock-llm-server.js` (1,018 lines)
- `/Users/norinlavaee/vscode-atomic/scripts/chat-simulation/common/utils.js` (836 lines)
- `/Users/norinlavaee/vscode-atomic/scripts/chat-simulation/common/perf-scenarios.js` (extensive scenario definitions)
- `/Users/norinlavaee/vscode-atomic/scripts/xterm-update.js` (102 lines)

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
