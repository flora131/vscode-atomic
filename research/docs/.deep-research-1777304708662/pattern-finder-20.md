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
