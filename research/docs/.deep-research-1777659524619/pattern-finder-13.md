# VS Code Launch & Bootstrap Script Patterns (Partition 13/79)

This analysis documents the concrete patterns used in VS Code's `scripts/` directory for launching core IDE functionality across different modalities (Electron desktop, web, server, agent host). These patterns are relevant to understanding the abstraction layers and entry points that would need to be reimplemented in a Tauri/Rust port.

---

## Pattern 1: Cross-Platform Root Resolution

**Where:** `scripts/code.sh:5-14`, `scripts/code-web.sh:3-8`, `scripts/code-server.sh:3-8`, `scripts/code-cli.sh:3-8`

**What:** Bash scripts normalize the repository root path across macOS and Linux by handling platform-specific path resolution.

```bash
if [[ "$OSTYPE" == "darwin"* ]]; then
	realpath() { [[ $1 = /* ]] && echo "$1" || echo "$PWD/${1#./}"; }
	ROOT=$(dirname "$(dirname "$(realpath "$0")")")
else
	ROOT=$(dirname "$(dirname "$(readlink -f $0)")")
	# If the script is running in Docker using the WSL2 engine, powershell.exe won't exist
	if grep -qi Microsoft /proc/version && type powershell.exe > /dev/null 2>&1; then
		IN_WSL=true
	fi
fi
```

**Variations / call-sites:**
- `scripts/test.sh:4-9` — Unit test script
- `scripts/test-integration.sh:4-9` — Integration test entry point
- `scripts/test-remote-integration.sh:4-9` — Remote server integration testing
- `scripts/node-electron.sh:3-18` — Electron node runner
- `scripts/code-agent-host.sh:3-8` — Agent host bootstrap

---

## Pattern 2: Environment Variable Gating with Pre-launch

**Where:** `scripts/code.sh:29-31`, `scripts/code-server.sh:14-16`, `scripts/code-agent-host.sh:14-16`

**What:** Pre-launch compilation/setup is conditionally skipped via the `VSCODE_SKIP_PRELAUNCH` environment variable. This gates expensive operations like node module fetching and Electron binary downloads.

```bash
# Get electron, compile, built-in extensions
if [[ -z "${VSCODE_SKIP_PRELAUNCH}" ]]; then
	node build/lib/preLaunch.ts
fi
```

**Variations / call-sites:**
- `scripts/code-cli.sh:23-26` — CLI pre-launch variant
- `scripts/test.sh:28-30` — Test pre-launch variant
- `scripts/node-electron.sh:22-24` — Node/Electron pre-launch variant

---

## Pattern 3: Executable Path Resolution from product.json

**Where:** `scripts/code.sh:19-26`

**What:** The launcher reads `product.json` metadata at runtime via Node.js to determine the correct Electron binary name and location, enabling platform-specific product naming.

```bash
if [[ "$OSTYPE" == "darwin"* ]]; then
	NAME=`node -p "require('./product.json').nameLong"`
	EXE_NAME=`node -p "require('./product.json').nameShort"`
	CODE="./.build/electron/$NAME.app/Contents/MacOS/$EXE_NAME"
else
	NAME=`node -p "require('./product.json').applicationName"`
	CODE=".build/electron/$NAME"
fi
```

**Variations / call-sites:**
- `scripts/test.sh:13-20` — Test runner variant
- `scripts/code-cli.sh:13-20` — CLI variant
- `scripts/node-electron.sh:12-19` — Node runner variant

---

## Pattern 4: Child Process Spawning with Signal Forwarding

**Where:** `scripts/code-web.js:87-104`

**What:** JavaScript launcher spawns child process via `child_process.spawn()` and forwards SIGINT/SIGTERM signals to enable graceful shutdown while maintaining stdio inheritance.

```javascript
function startServer(runnerArguments) {
	const env = { ...process.env };

	console.log(`Starting @vscode/test-web: ${testWebLocation} ${runnerArguments.join(' ')}`);
	const proc = cp.spawn(process.execPath, [testWebLocation, ...runnerArguments], { env, stdio: 'inherit' });

	proc.on('exit', (code) => process.exit(code));

	process.on('exit', () => proc.kill());
	process.on('SIGINT', () => {
		proc.kill();
		process.exit(128 + 2); // https://nodejs.org/docs/v14.16.0/api/process.html#process_signal_events
	});
	process.on('SIGTERM', () => {
		proc.kill();
		process.exit(128 + 15); // https://nodejs.org/docs/v14.16.0/api/process.html#process_signal_events
	});
}
```

**Variations / call-sites:**
- `scripts/code-server.js:40-68` — Server spawning with stdout capture for ready detection
- `scripts/code-agent-host.js:69-111` — Agent host spawning with READY pattern detection

---

## Pattern 5: Electron as Runtime Node Context

**Where:** `scripts/code-cli.sh:39-44`

**What:** Electron binary is repurposed as a Node.js runtime via `ELECTRON_RUN_AS_NODE=1` environment flag, allowing compiled TypeScript entry points to run before the full IDE boots.

```bash
ELECTRON_RUN_AS_NODE=1 \
NODE_ENV=development \
VSCODE_DEV=1 \
ELECTRON_ENABLE_LOGGING=1 \
ELECTRON_ENABLE_STACK_DUMPING=1 \
"$CODE" --inspect=5874 "$ROOT/out/cli.js" . $DISABLE_TEST_EXTENSION "$@"
```

**Variations / call-sites:**
- `scripts/node-electron.sh:30-36` — Node execution with ulimit on macOS
- `scripts/test.sh:35-42` — Test runner using Electron as Node

---

## Pattern 6: Environment Variable Propagation

**Where:** `scripts/code-server.sh:26-28`, `scripts/code-agent-host.sh:26-28`

**What:** Development environment flags (NODE_ENV, VSCODE_DEV) are set as environment variables before spawning the actual server process, ensuring consistent configuration across the runtime.

```bash
NODE_ENV=development \
VSCODE_DEV=1 \
$NODE $ROOT/scripts/code-server.js "$@"
```

**Variations / call-sites:**
- `scripts/code-web.sh:10-24` — Web server without explicit env setup (inherits from bash)
- `scripts/code-agent-host.sh:26-28` — Agent host explicit env setup
- `scripts/test-integration.sh:139-140` — Test setup with VSCODE_CLI and ELECTRON_ENABLE_LOGGING

---

## Pattern 7: Dynamic Node Runtime Resolution

**Where:** `scripts/code-web.sh:16-23`, `scripts/code-server.sh:18-22`, `scripts/code-agent-host.sh:18-22`

**What:** Node executable is resolved dynamically via `node build/lib/node.ts`, with fallback to npm gulp task to download remote node binary if missing.

```bash
NODE=$(node build/lib/node.ts)
if [ ! -e $NODE ];then
	# Load remote node
	npm run gulp node
fi

NODE=$(node build/lib/node.ts)

$NODE ./scripts/code-web.js "$@"
```

**Variations / call-sites:**
- `scripts/code-sessions-web.sh:16-24` — Sessions web server variant
- All web-based launchers use this pattern for remote node resolution

---

## Pattern 8: Conditional Electron Platform Detection

**Where:** `scripts/code.sh:80-92`

**What:** Bash launcher detects WSL2, Docker, and native environments to apply platform-specific workarounds before delegating to the appropriate runner function.

```bash
if [ "$IN_WSL" == "true" ] && [ -z "$DISPLAY" ]; then
	code-wsl "$@"
elif [ -f /mnt/wslg/versions.txt ]; then
	code --disable-gpu "$@"
elif [ -f /.dockerenv ]; then
	# Workaround for https://bugs.chromium.org/p/chromium/issues/detail?id=1263267
	# Chromium does not release shared memory when streaming scripts
	# which might exhaust the available resources in the container environment
	# leading to failed script loading.
	code --disable-dev-shm-usage "$@"
else
	code "$@"
fi
```

**Variations / call-sites:**
- `scripts/code.sh:55-78` — WSL-specific handler that proxies through Windows PowerShell

---

## Summary: Core Launch Abstraction Layers

The scripts reveal several abstraction layers that a Tauri/Rust port would need to replicate:

1. **Bootstrap Layer**: Platform detection and path resolution (macOS/Linux/WSL/Docker)
2. **Pre-launch Layer**: Conditional compilation and dependency fetching (`VSCODE_SKIP_PRELAUNCH`)
3. **Metadata Layer**: Runtime discovery of product name/version from `product.json`
4. **Runtime Selection**: Multi-modal execution (Electron desktop, Node.js server, web dev server)
5. **Process Management**: Signal handling, stdio inheritance, and graceful shutdown
6. **Environment Gating**: Configuration flags (VSCODE_DEV, NODE_ENV, ELECTRON_ENABLE_LOGGING)
7. **Node Runtime Handling**: Dynamic resolution and remote download of Node binaries
8. **Platform Workarounds**: GPU disabling, shared memory fixes for container/WSL environments

These patterns show that VS Code's launch infrastructure is heavily optimized for Node.js and Electron, with explicit support for development workflows (hot reload via VSCODE_DEV flag, source-based launching, inspector port attachment). A Tauri port would need to replace much of this with Rust-native equivalents while maintaining the same level of configuration flexibility.
