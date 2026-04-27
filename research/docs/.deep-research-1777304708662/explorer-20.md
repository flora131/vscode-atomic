# Partition 20 of 79 — Findings

## Scope
`.vscode/` (21 files, 3,834 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# VS Code Core IDE Porting Research — Locator Results (Partition 20/79)

## Scope: `.vscode/` Directory

This partition examines configuration and development infrastructure within `.vscode/`, documenting the launch/debug, build task, editor, and development extension configurations that would need to be understood or recreated in a Tauri/Rust port of VS Code's core IDE functionality.

---

## Configuration

### Debug & Launch Configurations
**File**: `/Users/norinlavaee/vscode-atomic/.vscode/launch.json` (835 lines)

Defines 40+ launch configurations covering multiple runtime targets and debugging strategies:

**Key Process Attachment Targets** (for IDE instrumentation):
- Extension Host (port 5870)
- Shared Process (port 5879)
- Search Process (port 5876)
- PTY Host Process (port 5877)
- Agent Host Process (port 5878)
- CLI Process (port 5874)
- Main Process (port 5875)

**Key Launch Modes**:
- Node.js debugging (Gulp build, Main Process attachment)
- Chrome/Chromium debugging (renderer/workbench debugging)
- Extension Host debugging (for extension isolation)
- Web browser modes (VS Code Server for Web, VS Code Web)
- Smoke testing and unit test execution

**Workbench Launch Variants**:
- `Launch VS Code Internal` — Chrome-based debugging with hot reload disabled
- `Launch VS Code Internal (Hot Reload)` — Development mode with Vite server integration (uses `http://localhost:5199/build/vite/workbench-vite-electron.html`)
- `Launch VS Code Agents Internal` — Agent host variant
- `Launch VS Code Server (Web)` — Web-based deployment target

**Compound Configurations** (stacked debugging):
- `VS Code` — combines Main, Extension Host, Shared Process, Agent Host
- `VS Code Agents` — variant for agents mode
- `VS Code (Hot Reload)` — development iteration mode

**Test Execution Configurations**:
- Git, Configuration Editing, GitHub, API, Tokenizer, Markdown, TypeScript extension test hosts
- Unit tests via Electron and Mocha
- Smoke tests with configurable build paths

**Electron Build Paths** (platform-specific):
- macOS: `.build/electron/Code - OSS.app/Contents/MacOS/Code - OSS`
- Windows: `.build/electron/Code - OSS.exe`
- Linux: `.build/electron/code-oss`

### Build & Watch Tasks
**File**: `/Users/norinlavaee/vscode-atomic/.vscode/tasks.json` (503 lines)

**Core Transpilation & Type-checking**:
- `Core - Transpile` — esbuild-based compilation with watch (npm script: `watch-client-transpiled`)
- `Core - Typecheck` — TypeScript compilation (npm script: `watch-clientd`)
- `Ext - Build` — Extension compilation (npm script: `watch-extensionsd`)
- `Copilot - Build` — esbuild variant (npm script: `watch-copilotd`)
- **Composite**: `VS Code - Build` — depends on all four above tasks

**Build Kill Tasks** (for iterative rebuilding):
- Kill/restart patterns for each build component
- `Restart VS Code - Build` — sequential kill + rebuild

**Runtime Execution**:
- `Run Dev` — shell-based (scripts/code.sh or code.bat)
- `Run Dev Agents` — with agent flags and custom user-data-dir
- `Transpile Client` — npm script wrapper

**Web & Server Variants**:
- `Run code server` — node-based with connection token (port 8080)
- `Run code web` — browser-based server
- Background tasks with regex pattern matchers for stdout/stderr

**Vite Development Server**:
- `Launch Monaco Editor Vite` — npm run dev in ./build/vite/
- Serves `/build/vite/workbench-vite-electron.html`
- Integrates with hot reload configuration

**Testing Infrastructure**:
- `Run tests` — shell script (scripts/test.sh or test.bat)
- `Git Unit Tests` — Mocha test runner in extensions/git
- `HTML Server Unit Tests` — extensions/html-language-features/server
- `CSS Server Unit Tests` — extensions/css-language-features/server

**Linting & Quality**:
- `npm: eslint` — ESLint integration
- `npm: tsec-compile-check` — TypeScript security check (tsec)
- `Ensure Prelaunch Dependencies` — Node build/lib/preLaunch.ts

**Component/Fixture Servers**:
- `Component Explorer Server` — component-explorer CLI (port 5337)
- `Launch MCP Server` — Model Context Protocol server (test/mcp)

### Editor & Language Settings
**File**: `/Users/norinlavaee/vscode-atomic/.vscode/settings.json` (217 lines)

**Chat & AI Features**:
- Inline chat (V2), terminal auto-approval for test scripts, edit suggestion explanations

**Editor Defaults**:
- Tab mode (no spaces), async tokenization, occurrence highlighting delay

**Language-Specific Formatting**:
- TypeScript/JavaScript: format-on-save enabled
- Rust: rust-analyzer as default formatter

**File Handling**:
- Exclude patterns: `.git`, `.build`, `.profile-oss`, `.tsbuildinfo`, build outputs
- Read-only include patterns: `node_modules/**`, Cargo.lock, `out/**`, `extensions/**/dist/**`

**Search Exclusions**:
- Patterns for generated files, test fixtures, snapshots

**TypeScript Intellisense**:
- TSGO experimental mode
- Relative imports, single-quote preference
- Auto-import exclusions (xterm, node-pty, vscode-notebook-renderer)

**Git Configuration**:
- Branch protection on main/release branches
- Random branch naming, merge editor, diagnostic hooks
- Worktree inclusion for product.overrides.json

**GitHub Integration**:
- Pull request experimental views, squash merge default, AI coding agent

**Testing & Debugging**:
- Auto-run on rerun mode, JavaScript debug outFiles
- Coverage (LCOV) watch patterns

**Rust Analyzer**:
- Linked project: `cli/Cargo.toml` (indicates Rust CLI component in VS Code)

**Analysis Tools**:
- ESLint flat config, azure-mcp service integration, component explorer skill locations

### Recommended Extensions
**File**: `/Users/norinlavaee/vscode-atomic/.vscode/extensions.json` (13 lines)

Workspace recommendations:
- `dbaeumer.vscode-eslint` — Linting
- `editorconfig.editorconfig` — EditorConfig support
- `github.vscode-pull-request-github` — GitHub PR integration
- `ms-vscode.vscode-github-issue-notebooks` — GitHub Issues notebook format
- `ms-vscode.extension-test-runner` — Extension testing framework
- `typescriptteam.native-preview` — Native preview rendering
- `ms-vscode.ts-customized-language-service` — Customized TypeScript language service

### Shared Code Snippets
**File**: `/Users/norinlavaee/vscode-atomic/.vscode/shared.code-snippets` (40 lines)

**Development Patterns**:
- `MSFT Copyright Header` — Standard Microsoft/MIT license block
- `TS -> Inject Service` — Constructor injection pattern (`@inject` prefix)
- `TS -> Event & Emitter` — Observable event pattern (Emitter/Event properties)

### MCP (Model Context Protocol) Configuration
**File**: `/Users/norinlavaee/vscode-atomic/.vscode/mcp.json` (29 lines)

**MCP Servers**:
- `vscode-automation-mcp` — Test automation MCP server (stdio, runs `npm run start-stdio` in test/mcp)
- `component-explorer` — Component fixture server via component-explorer CLI

### License Schema
**File**: `/Users/norinlavaee/vscode-atomic/.vscode/cglicenses.schema.json` (50+ lines)

JSON schema for component license generation and tracking.

---

## Implementation

### Selfhost Test Provider Extension
**Directory**: `/Users/norinlavaee/vscode-atomic/.vscode/extensions/vscode-selfhost-test-provider/` (contains 15 TypeScript source files)

**Purpose**: Built-in test infrastructure extension for running VS Code's internal test suite.

**Key Modules**:
- `extension.ts` — Main test controller registration (vscode.tests.createTestController)
- `vscodeTestRunner.ts` — Browser/platform-specific test runners
- `testTree.ts` — Test discovery and structure
- `testOutputScanner.ts` — Parse test output for results
- `stackTraceParser.ts` — Parse error stack traces
- `coverageProvider.ts`, `v8CoverageWrangling.ts` — V8 coverage integration
- `importGraph.ts` — Dependency analysis
- `failingDeepStrictEqualAssertFixer.ts` — Assert failure diagnosis
- `failureTracker.ts` — Test state tracking
- `snapshot.ts` — Snapshot update mechanism
- `sourceUtils.ts` — Source file utilities
- `metadata.ts`, `memoize.ts`, `debounce.ts` — Utility helpers
- `streamSplitter.ts` — Stream parsing

**Test File Pattern**: `src/vs/**/*.{test,integrationTest}.ts`

**Configuration Files**:
- `package.json` — Dependencies (mocha, xterm, node-pty)
- `.vscode/launch.json` — Test debugging configs
- `.vscode/settings.json` — Extension-specific settings
- `tsconfig.json` — TypeScript compilation

### Selfhost Import Aid Extension
**Directory**: `/Users/norinlavaee/vscode-atomic/.vscode/extensions/vscode-selfhost-import-aid/` (contains 5 files)

**Purpose**: TypeScript import suggestion and validation extension for IDE development.

**Key Module**:
- `extension.ts` — File index watcher (scans `src/vs/**/*.ts` excluding node_modules, extensions), provides auto-import suggestions via LSP

### VS Code Extras Extension
**Directory**: `/Users/norinlavaee/vscode-atomic/.vscode/extensions/vscode-extras/` (contains 4 files)

**Purpose**: Workspace-specific utility features for VS Code development.

**Modules**:
- `extension.ts` — Extension lifecycle, LogOutputChannel setup
- `npmUpToDateFeature.ts` — npm package update detection (configuration-driven)

### PR Pinger Extension
**Directory**: `/Users/norinlavaee/vscode-atomic/.vscode/extensions/vscode-pr-pinger/` (contains 5 files)

**Purpose**: GitHub PR notification and monitoring extension.

**Modules**:
- `extension.ts` — Status bar item for PR monitoring, GitHub authentication, GraphQL queries, focus tracking for "nudging" after context switches

**Dependencies**: `@octokit/graphql` for GitHub API access

---

## Types / Interfaces

### Extension Configuration Interfaces
Defined within extension source files:

**vscode-selfhost-test-provider**:
- `TestCase`, `TestFile` — Test item data structures (testTree.ts)
- `V8CoverageFile` — Coverage format (coverageProvider.ts)
- `PlatformTestRunner`, `BrowserTestRunner`, `VSCodeTestRunner` — Runner implementations

**vscode-selfhost-import-aid**:
- File index map structure for symbol lookup

**vscode-pr-pinger**:
- `PrInfo` interface (referenced in commands, likely contains `url` property)

---

## Tests

### vscode-selfhost-test-provider
**File**: `/Users/norinlavaee/vscode-atomic/.vscode/extensions/vscode-selfhost-test-provider/src/v8CoverageWrangling.test.ts`

Unit test for V8 coverage processing (paired with main implementation).

---

## Examples / Fixtures

### Test Workspaces & Fixtures
Referenced in launch.json:
- `/extensions/emmet/test-workspace`
- `/extensions/vscode-api-tests/testWorkspace`
- `/extensions/vscode-colorize-tests/test`
- `/extensions/markdown-language-features/test-workspace`
- `/extensions/typescript-language-features/test-workspace`
- `/extensions/git` (temporary directory `/tmp/my4g9l`)
- `/extensions/github/testWorkspace`
- `./test/componentFixtures/component-explorer.json` (Component Explorer fixture config)

### Development Notebooks (GitHub Issues Integration)
**Directory**: `/Users/norinlavaee/vscode-atomic/.vscode/notebooks/` (8 .github-issues files)

Notebooks using GitHub Issues query language for triage/grooming:
- `api.github-issues` — API finalization tracking
- `endgame.github-issues` — Release endgame checklist
- `grooming.github-issues` — Issue grooming across Microsoft vscode* repos
- `grooming-delta.github-issues` — Historical grooming queries
- `inbox.github-issues` — Inbox triage
- `my-work.github-issues`, `my-endgame.github-issues`, `papercuts.github-issues` — Personal/focused queries
- `vscode-dev.github-issues` — vscode-dev specific

### Code Search Bookmarks
**Directory**: `/Users/norinlavaee/vscode-atomic/.vscode/searches/` (2 .code-search files)

Saved regex searches for code quality:
- `no-any-casts.code-search` — Finds all `// eslint-disable-next-line (local/code-no-any-casts|@typescript-eslint/no-explicit-any)` disables (727 results across 269 files)
- `ts36031.code-search` — Optional chaining + non-null assertion patterns (`?.!`) (8 results across 4 files)

---

## Notable Clusters

### Multi-Process Architecture
The launch.json reveals VS Code's modular process architecture requiring separate debug attachment:

1. **Main Process** (Electron, port 5875) — Application entry point
2. **Renderer/Workbench** (Chrome debugging, port 9222) — UI rendering
3. **Extension Host** (Node.js, port 5870) — Extension isolation
4. **Shared Process** (port 5879) — Shared state/services
5. **Search Process** (port 5876) — Indexing and search
6. **PTY Host** (port 5877) — Terminal management
7. **Agent Host** (port 5878) — Agent/AI features
8. **CLI** (port 5874) — Command-line interface

**Porting Implication**: A Tauri/Rust port would need to either:
- Replicate this process isolation model in Rust (async runtimes, IPC boundaries)
- Redesign as monolithic but maintain internal module boundaries

### Build Toolchain Evolution
Tasks.json shows staged build pipeline:

1. **TypeScript → JavaScript** (esbuild, transpile-client task)
2. **Type Checking** (tsc, watch-clientd)
3. **Extension Compilation** (watch-extensionsd)
4. **Special Handling** (copilot, web variants)
5. **Web/Server Builds** (Vite, rspack)

**Files**: Transpiled to `out/` and `out-build/` directories per settings.json exclusions.

### Development Extensions as Dogfood
Four internal extensions demonstrate VS Code API usage for IDE development:
- **Test infrastructure** (test discovery, coverage, failure diagnosis)
- **Developer tooling** (import resolution, configuration)
- **Utilities** (npm checks, PR notifications)

**Port Consideration**: These extensions provide examples of core features that might be implemented directly in Rust vs. via extension API.

### Vite-Based Editor Development
The hot-reload configuration points to Vite build setup:
- Dev server at `http://localhost:5199/build/vite/`
- Serves `workbench-vite-electron.html`, `workbench-vite.html`, and `index.html` variants
- Separate from main build pipeline (esbuild/tsc)

**Implication**: The Workbench UI is increasingly decoupled; Monaco Editor development can be isolated.

### AI/Agent Integration
Settings and extensions reference agent/AI features:
- `vscode-selfhost-test-provider` provides test failure fixes via Copilot
- `vscode-pr-pinger` monitors PR status with GitHub auth
- Chat tools auto-approve terminal execution
- MCP (Model Context Protocol) servers configured for automation

### GitHub Workflow Integration
`.vscode/notebooks/` and extension integrations show tight GitHub coupling:
- Issue/PR tracking via notebooks (GraphQL queries)
- Branch protection and merge strategies configured
- GitHub authentication required for extensions

---

## Summary

The `.vscode/` directory (21 files, 3,834 LOC) documents VS Code's internal development infrastructure with specific relevance to an IDE port:

**Launch & Debug Configuration**: Multi-process architecture (7 debug endpoints) requires architectural decisions on process isolation vs. monolithic Rust implementation. Electron platform-specific paths and Chrome DevTools Protocol usage would need Tauri equivalents.

**Build System**: Four-stage TypeScript compilation (transpile → typecheck → extensions → copilot) with esbuild, tsc, and Vite. A Rust port would eliminate TypeScript compilation but must retain incremental build patterns, especially for extension/plugin systems.

**Development Extensions**: Four workspace extensions (test provider, import aid, extras, pr-pinger) demonstrate dogfooding of VS Code APIs for IDE development. A Rust port should identify which capabilities require native implementation vs. extension API preservation.

**Vite Integration**: Monaco Editor development is decoupled via Vite server, suggesting that the UI layer (Workbench) has separated concerns. This may simplify ports that target language server/backend first.

**AI/Agent Features**: Recent additions (agent host process, MCP servers, chat integration) indicate active evolution toward AI-assisted development. A port must account for these integration points.

**GitHub Workflow**: Notebook-based GitHub issue tracking and PR integration are organizational tools rather than core IDE functionality, but represent the development team's workflow assumptions.

A Tauri/Rust port would need to establish equivalent configurations for debugging (likely via LSP servers and Tauri's native debugging hooks), build pipelines (Rust build system rather than npm tasks), and process isolation (Rust async/IPC rather than Electron/Node.js).

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Development Configuration Patterns (.vscode/)

This partition documents concrete patterns from the workspace dogfood configuration (`.vscode/`), which shows debug configurations, build tasks, and launch invocations that a Tauri/Rust port would need to mirror or adapt.

## Core Build Architecture

#### Pattern: Multi-Process Debug Attachment Strategy
**Where:** `.vscode/launch.json:14-65`
**What:** VS Code's architecture separates rendering, extension hosting, search, PTY, agents, and CLI into distinct debug-attachable processes.
```json
{
	"type": "node",
	"request": "attach",
	"restart": true,
	"name": "Attach to Extension Host",
	"timeout": 0,
	"port": 5870,
	"outFiles": [
		"${workspaceFolder}/out/**/*.js",
		"${workspaceFolder}/extensions/*/out/**/*.js",
		"${workspaceFolder}/extensions/copilot/dist/**/*.js"
	]
},
{
	"type": "node",
	"request": "attach",
	"restart": true,
	"name": "Attach to Shared Process",
	"timeout": 0,
	"port": 5879,
	"outFiles": [
		"${workspaceFolder}/out/**/*.js"
	]
},
{
	"type": "node",
	"request": "attach",
	"name": "Attach to Search Process",
	"port": 5876,
	"outFiles": [
		"${workspaceFolder}/out/**/*.js"
	]
},
{
	"type": "node",
	"request": "attach",
	"name": "Attach to Pty Host Process",
	"port": 5877,
	"outFiles": [
		"${workspaceFolder}/out/**/*.js"
	]
},
{
	"type": "node",
	"request": "attach",
	"name": "Attach to Agent Host Process",
	"port": 5878,
	"restart": true,
	"timeout": 0,
	"outFiles": [
		"${workspaceFolder}/out/**/*.js"
	]
}
```

**Variations:** 
- Main Process (port 5875) uses `continueOnAttach` instead of `restart`
- Agent Host Process requires `restart: true` and extended timeouts
- CLI Process (port 5874) omits restart

---

#### Pattern: Compound Debug Configuration for Full IDE
**Where:** `.vscode/launch.json:722-738`
**What:** Compounds orchestrate simultaneous launch and attachment of multiple processes with coordinated lifecycle.
```json
{
	"name": "VS Code",
	"stopAll": true,
	"configurations": [
		"Launch VS Code Internal",
		"Attach to Main Process",
		"Attach to Extension Host",
		"Attach to Shared Process",
		"Attach to Agent Host Process"
	],
	"preLaunchTask": "Ensure Prelaunch Dependencies",
	"presentation": {
		"group": "0_vscode",
		"order": 1
	}
}
```

**Variations:** 
- "VS Code Agents" variant includes agents configuration
- "VS Code (Hot Reload)" uses hot reload launcher
- Renderer-focused variants omit Agent/Shared processes

---

#### Pattern: Platform-Specific Launcher Scripts
**Where:** `.vscode/launch.json:252-292`
**What:** Runtime executable varies by OS, delegating to shell scripts that handle electron startup with inspection ports.
```json
{
	"type": "chrome",
	"request": "launch",
	"name": "Launch VS Code Internal",
	"windows": {
		"runtimeExecutable": "${workspaceFolder}/scripts/code.bat"
	},
	"osx": {
		"runtimeExecutable": "${workspaceFolder}/scripts/code.sh"
	},
	"linux": {
		"runtimeExecutable": "${workspaceFolder}/scripts/code.sh"
	},
	"port": 9222,
	"timeout": 0,
	"env": {
		"VSCODE_EXTHOST_WILL_SEND_SOCKET": null,
		"VSCODE_SKIP_PRELAUNCH": "1",
		"VSCODE_DEV_DEBUG_OBSERVABLES": "1"
	},
	"runtimeArgs": [
		"--inspect-brk=5875",
		"--no-cached-data",
		"--crash-reporter-directory=${workspaceFolder}/.profile-oss/crashes",
		"--disable-features=CalculateNativeWinOcclusion",
		"--disable-extension=vscode.vscode-api-tests"
	],
	"userDataDir": "${userHome}/.vscode-oss-dev",
	"webRoot": "${workspaceFolder}",
	"cascadeTerminateToConfigurations": [
		"Attach to Extension Host"
	]
}
```

**Variations:** 
- Agents variant includes `--agents` flag and `.vscode-oss-agents-dev` data dir
- Hot Reload variant adds vite dev server env vars

---

## Build Task Architecture

#### Pattern: Watch-Based Incremental Build System
**Where:** `.vscode/tasks.json:4-119`
**What:** Separate watch tasks for core transpilation, typechecking, extensions, and copilot, with error pattern matching; combined via composite task.
```json
{
	"type": "npm",
	"script": "watch-client-transpiled",
	"label": "Core - Transpile",
	"isBackground": true,
	"presentation": {
		"reveal": "never",
		"group": "buildWatchers",
		"close": false
	},
	"problemMatcher": {
		"owner": "esbuild",
		"applyTo": "closedDocuments",
		"fileLocation": ["relative", "${workspaceFolder}/src"],
		"pattern": {
			"regexp": "^(.+?):(\\d+):(\\d+): ERROR: (.+)$",
			"file": 1,
			"line": 2,
			"column": 3,
			"message": 4
		},
		"background": {
			"beginsPattern": "Starting transpilation\\.\\.\\.",
			"endsPattern": "Finished transpilation with"
		}
	}
}
```

**Variations:** 
- TypeCheck task uses typescript pattern: `^\\] (.+)\\(([\\d,]+)\\): error TS(\\d+): (.*)$`
- Extensions/Copilot variants use esbuild watch patterns
- Each has corresponding "Kill" task to terminate background process

---

#### Pattern: Test Execution with Script Invocation
**Where:** `.vscode/tasks.json:238-248`
**What:** Shell-based test runner with cross-platform batch/shell script handling.
```json
{
	"label": "Run tests",
	"type": "shell",
	"command": "./scripts/test.sh",
	"windows": {
		"command": ".\\scripts\\test.bat"
	},
	"group": "test",
	"presentation": {
		"echo": true,
		"reveal": "always"
	}
}
```

**Variations:** 
- "Run Dev" launches development version
- "Run Dev Agents" adds --agents and custom data/extensions dirs
- Coverage-focused variant in settings.json uses `${workspaceFolder}/scripts/test.sh --coverage --run ${file}`

---

#### Pattern: Server Launch with Port Detection
**Where:** `.vscode/tasks.json:310-362`
**What:** Background shell tasks with pattern matchers detecting server readiness via log patterns.
```json
{
	"type": "shell",
	"command": "./scripts/code-server.sh",
	"windows": {
		"command": ".\\scripts\\code-server.bat"
	},
	"args": [
		"--no-launch",
		"--connection-token",
		"dev-token",
		"--port",
		"8080"
	],
	"label": "Run code server",
	"isBackground": true,
	"problemMatcher": {
		"pattern": {
			"regexp": ""
		},
		"background": {
			"beginsPattern": ".*node .*",
			"endsPattern": "Web UI available at .*"
		}
	}
}
```

**Variations:** 
- Web variant uses `code-web.sh` with different end pattern: `"Listening on .*"`
- Vite dev server matches `"(Local|Network):.*"` pattern

---

## Extension Test Configuration

#### Pattern: Extension Host Test Launch
**Where:** `.vscode/extensions/vscode-selfhost-test-provider/.vscode/launch.json:2-11`
**What:** Single-file extension test runs via extensionHost type with development path and test path args.
```json
{
	"args": ["--extensionDevelopmentPath=${workspaceFolder}", "--enable-proposed-api=ms-ms-vscode.vscode-selfhost-test-provider"],
	"name": "Launch Extension",
	"outFiles": ["${workspaceFolder}/out/**/*.js"],
	"request": "launch",
	"type": "extensionHost"
}
```

**Variations:** 
- Full test suite launches multiple workspace types (single-folder, workspace)
- Some tests include temp directory args or disable-extensions flags
- All use extensionTestsPath pointing to out/test directories

---

## Workspace Configuration

#### Pattern: Language-Specific Editor Settings
**Where:** `.vscode/settings.json:23-36`
**What:** Language-keyed settings for format-on-save and Rust analyzer integration.
```json
"[typescript]": {
	"editor.formatOnSave": true
},
"[javascript]": {
	"editor.formatOnSave": true
},
"[rust]": {
	"editor.defaultFormatter": "rust-lang.rust-analyzer",
	"editor.formatOnSave": true
}
```

**Variations:** 
- Plaintext disables final newline insertion
- GitHub-issues wraps text

---

#### Pattern: Workspace File Exclusions and Read-Only Paths
**Where:** `.vscode/settings.json:43-77`
**What:** Dual-layer exclusion: visible file exclusions and global read-only paths for compiled outputs and dependencies.
```json
"files.exclude": {
	".git": true,
	".build": true,
	".profile-oss": true,
	"**/*.tsbuildinfo": true,
	"**/.DS_Store": true,
	".vscode-test": true,
	"cli/target": true,
	"build/**/*.js.map": true,
	"build/**/*.js": {
		"when": "$(basename).ts"
	}
},
"files.readonlyInclude": {
	"**/node_modules/**/*.*": true,
	"**/yarn.lock": true,
	"**/package-lock.json": true,
	"**/Cargo.lock": true,
	"out/**": true,
	"out-build/**": true,
	"extensions/**/dist/**": true,
	"test/smoke/out/**": true,
	"test/automation/out/**": true
}
```

**Variations:** 
- Multiple `out-*` directories marked read-only
- Conditional `.js` exclusion based on `.ts` presence
- Rust Cargo.lock marked read-only alongside npm lockfiles

---

#### Pattern: Rust Tooling Configuration
**Where:** `.vscode/settings.json:191-192`
**What:** Linked Rust projects referenced for analyzer integration.
```json
"rust-analyzer.linkedProjects": [
	"cli/Cargo.toml"
]
```

**Variations:** None observed; single Cargo.toml reference.

---

## MCP Server Integration

#### Pattern: Workspace MCP Server Definitions
**Where:** `.vscode/mcp.json:2-27`
**What:** Stdio-based MCP servers for test automation and component exploration.
```json
"servers": {
	"vscode-automation-mcp": {
		"type": "stdio",
		"command": "npm",
		"args": [
			"run",
			"start-stdio"
		],
		"cwd": "${workspaceFolder}/test/mcp"
	},
	"component-explorer": {
		"type": "stdio",
		"command": "npm",
		"args": [
			"exec",
			"--no",
			"--",
			"component-explorer",
			"mcp",
			"-p",
			"./test/componentFixtures/component-explorer.json",
			"--use-daemon",
			"-vv"
		]
	}
}
```

**Variations:** None observed in current workspace; extensible pattern for additional servers.

---

## Key Insights for Porting

The `.vscode/` configuration reveals several architectural requirements for a Tauri/Rust port:

1. **Multi-Process Architecture**: VS Code's design requires simultaneous debugging of 5+ processes (main, extension host, search, pty, agent, CLI), each listening on distinct ports (5874-5879). A Tauri port must preserve this isolation and debuggability.

2. **Build System**: Watch-based incremental builds with esbuild (transpilation) and tsc (typechecking) are separate. Problem matchers parse error streams in real-time. Rust builds would need equivalent watchers with cargo-compatible error patterns.

3. **Cross-Platform Script Delegation**: All launchers delegate to platform-specific shell scripts (`code.sh`, `code.bat`). A Tauri port must maintain this abstraction layer to handle OS-specific electron/runtime startup flags (inspect-brk, no-cached-data, etc.).

4. **Environment Variable Injection**: VSCODE_* family of env vars controls feature flags (VSCODE_DEV_DEBUG, VSCODE_EXTHOST_WILL_SEND_SOCKET). Tauri's process spawning must preserve these.

5. **Test Harness Patterns**: Extension tests use extensionHost type with --extensionDevelopmentPath and --extensionTestsPath. Unit tests use mocha/electron directly. Both need test runner parity.

6. **Server Readiness Detection**: Background tasks detect server startup via stdout pattern matching (Web UI available at..., Listening on...). Critical for synchronizing dependent tasks.

7. **Rust Integration**: Existing Cargo.toml linked project shows CLI component already has Rust presence; workspace is prepared for polyglot builds.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
