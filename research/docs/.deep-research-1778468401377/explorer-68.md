# Partition 68 of 80 — Findings

## Scope
`src/cli.ts/` (1 files, 26 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator 68: JS Shim Launching Electron → Rust CLI Replacement

## File Scope
- **SCOPE**: `src/cli.ts` (~26 LOC)

## Key Finding: JS Shim Dispatch Pattern

The file `/home/norinlavaee/projects/vscode-atomic/src/cli.ts` is a minimal TypeScript entry point that delegates all actual CLI work to downstream modules. It contains **no Electron spawn calls itself** — it is purely a bootstrap harness.

### Implementation

- `/home/norinlavaee/projects/vscode-atomic/src/cli.ts` — 26-line shim that sets up NLS, portability, and CLI environment variables, then imports the actual CLI handler at line 26: `await import('./vs/code/node/cli.js')`

- `/home/norinlavaee/projects/vscode-atomic/src/vs/code/node/cli.ts` — The actual JavaScript CLI entry point. Contains the spawn logic:
  - **Line 6**: imports `spawn`, `SpawnOptions`, `StdioOptions` from `child_process`
  - **Line 73**: `spawn('cargo', ['run', '--', subcommand, ...tunnelArgs], ...)` for dev mode tunnels
  - **Line 80**: `spawn(tunnelCommand, [subcommand, ...tunnelArgs], ...)` for production tunnel binary
  - **Line 496**: `spawn(process.execPath, argv.slice(2), options)` for non-macOS Electron launch
  - **Line 571**: `spawn('open', spawnArgs, ...)` for macOS Electron launch (via `open` command)

### Bootstrap Chain

The scope file (`src/cli.ts`) orchestrates this boot sequence:
1. Line 6: imports `'./bootstrap-cli.js'` (clears VSCODE_CWD)
2. Line 7: imports `configurePortable()` from `bootstrap-node.js`
3. Line 8: imports `bootstrapESM()` from `bootstrap-esm.js`
4. Line 9: resolves NLS configuration
5. Line 17: configures portable support
6. Line 20: sets `VSCODE_CLI='1'` environment variable
7. Line 23: calls `bootstrapESM()`
8. Line 26: imports and executes actual CLI code from `./vs/code/node/cli.js`

### Rust Replacement Locations

The Rust CLI in `cli/` replaces the TypeScript JS shim and CLI logic entirely:

- `/home/norinlavaee/projects/vscode-atomic/cli/src/bin/code/main.rs` — Entry point for the Rust-based code CLI. Uses `std::process::Command` (equivalent to spawn) at function `start_code()` to launch the Electron/desktop application.

- `/home/norinlavaee/projects/vscode-atomic/cli/src/commands/` — Contains 20+ subcommand implementations (agent, tunnels, update, version, etc.) that replace the TypeScript dispatch logic.

## Related Bootstrap Infrastructure

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-cli.ts` — Clears VSCODE_CWD env var (12 LOC)
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-node.ts` — Configures portable mode
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts` — ESM module setup
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-meta.ts` — Product metadata
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-import.ts` — Import hook setup
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-fork.ts` — Fork support

## Summary

The query `ast-grep --lang ts -p 'spawn($$$)'` targets process launching patterns in TypeScript. Within the specified SCOPE (`src/cli.ts`), no spawn calls exist—the file is purely a 26-line bootstrap orchestrator. The actual Electron spawn logic resides downstream in `/home/norinlavaee/projects/vscode-atomic/src/vs/code/node/cli.ts` (4 spawn sites: cargo for tunnels, tunnel binary for production, Node executable for non-macOS, and macOS `open` command).

The Rust CLI port (`cli/src/bin/code/main.rs` and subcommands) replaces both the shim and the underlying TypeScript CLI entirely, providing native process launching without Node.js or Electron's `child_process` module.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: Electron Launching, Child Process Spawning, and Argv Handling

## Scope
Single file: `src/cli.ts` (~26 LOC bootstrap loader)

This research catalogs concrete patterns for how VS Code's TypeScript/Electron CLI shim launches child processes and passes arguments. These patterns are relevant to the Tauri/Rust port.

---

## Pattern 1: Direct Process Spawning for Tunnel Commands
**Where:** `src/vs/code/node/cli.ts:69-87`
**What:** Spawning child process with stdio redirection and environment variable handling for tunnel subcommands

```typescript
let tunnelProcess: ChildProcess;
const stdio: StdioOptions = ['ignore', 'pipe', 'pipe'];
if (process.env['VSCODE_DEV']) {
	tunnelProcess = spawn('cargo', ['run', '--', subcommand, ...tunnelArgs], { cwd: join(getAppRoot(), 'cli'), stdio, env });
} else {
	const appPath = process.platform === 'darwin'
		? join(dirname(dirname(process.execPath)), 'Resources', 'app')
		: dirname(process.execPath);
	const tunnelCommand = join(appPath, 'bin', `${product.tunnelApplicationName}${isWindows ? '.exe' : ''}`);
	tunnelProcess = spawn(tunnelCommand, [subcommand, ...tunnelArgs], { cwd: cwd(), stdio, env });
}

tunnelProcess.stdout!.pipe(process.stdout);
tunnelProcess.stderr!.pipe(process.stderr);
tunnelProcess.on('exit', resolve);
tunnelProcess.on('error', reject);
```

**Key patterns:**
- Development mode uses `cargo run` with explicit CLI path
- Production uses resolved executable path with platform-specific extensions
- Slices argv to extract subcommand arguments: `argv.slice(argv.indexOf(subcommand) + 1)`
- stdio configured as `['ignore', 'pipe', 'pipe']` (stdin ignored, stdout/stderr piped)
- Environment variables explicitly passed via `env` parameter
- Child stdout/stderr piped to parent process streams
- Promise-wrapped for async lifecycle handling

---

## Pattern 2: Environment Variable Sanitization
**Where:** `src/vs/code/node/cli.ts:59-66`
**What:** Removing Electron-specific env vars before spawning server processes

```typescript
const env: IProcessEnvironment = {
	...process.env
};
// bootstrap-esm.js determines the electron environment based
// on the following variable. For the server we need to unset
// it to prevent importing any electron specific modules.
// Refs https://github.com/microsoft/vscode/issues/221883
delete env['ELECTRON_RUN_AS_NODE'];
```

**Key patterns:**
- Clone parent process.env using spread operator
- Selectively delete environment variables that affect behavior
- Comments document the rationale for deletion

---

## Pattern 3: Electron Process Spawning with Argument Marshalling
**Where:** `src/vs/code/node/cli.ts:480-496`
**What:** Spawning Electron with argv array and SpawnOptions configuration

```typescript
const options: SpawnOptions = {
	detached: true,
	env
};

if (!args.verbose) {
	options['stdio'] = 'ignore';
}

let child: ChildProcess;
if (!isMacintosh) {
	// We spawn the resolved executable directly
	child = spawn(process.execPath, argv.slice(2), options);
}
```

**Key patterns:**
- SpawnOptions configured with `detached: true` (parent can exit independently)
- stdio selectively set to 'ignore' for non-verbose mode
- argv sliced from position 2 (skips node path and script path)
- `process.execPath` used to spawn Node.js again with same executable
- Conditional platform handling (macOS vs non-macOS)

---

## Pattern 4: macOS-Specific Process Launch via `open` Command
**Where:** `src/vs/code/node/cli.ts:498-570`
**What:** Using native `open` command on macOS to maintain window behavior

```typescript
const spawnArgs = ['-n', '-g'];
spawnArgs.push('-a', process.execPath); // -a opens the given application.

if (args.verbose || args.status) {
	spawnArgs.push('--wait-apps'); // blocks until the launched app is closed
	
	for (const outputType of args.verbose ? ['stdout', 'stderr'] : ['stdout']) {
		const tmpName = randomPath(tmpdir(), `code-${outputType}`);
		writeFileSync(tmpName, '');
		spawnArgs.push(`--${outputType}`, tmpName);
	}
}

for (const e in env) {
	if (e !== '_') {
		spawnArgs.push('--env');
		spawnArgs.push(`${e}=${env[e]}`);
	}
}

spawnArgs.push('--args', ...argv.slice(2));
child = spawn('open', spawnArgs, options);
```

**Key patterns:**
- `-n` flag creates new instance
- `-g` starts in background (prevents macOS auto-foreground)
- `--wait-apps` blocks until app closes (for status/verbose modes)
- Environment variables encoded as separate `--env` flags to `open` command
- stdout/stderr redirected to temp files (limitation of `open` command)
- Arguments collected in array and passed after `--args` separator

---

## Pattern 5: Electron Environment Configuration
**Where:** `src/vs/code/node/cli.ts:223-234`
**What:** Setting Electron-specific environment flags before spawning

```typescript
const env: IProcessEnvironment = {
	...process.env,
	'ELECTRON_NO_ATTACH_CONSOLE': '1'
};

delete env['ELECTRON_RUN_AS_NODE'];

if (args.verbose) {
	env['ELECTRON_ENABLE_LOGGING'] = '1';
}
```

**Key patterns:**
- Clone process.env and add Electron-specific flags
- `ELECTRON_NO_ATTACH_CONSOLE`: suppress console window on Windows
- `ELECTRON_RUN_AS_NODE`: deleted to prevent Node.js compatibility mode
- `ELECTRON_ENABLE_LOGGING`: conditionally set for debug builds
- All environment variables set before spawning child

---

## Pattern 6: Argument Array Mutation via `addArg` Helper
**Where:** `src/vs/code/node/cli.ts:255-262`
**What:** Programmatically building argv array by appending key-value pairs

```typescript
addArg(argv, '--user-data-dir', tempUserDataDir);
addArg(argv, '--extensions-dir', tempExtensionsDir);
addArg(argv, '--shared-data-dir', tempSharedDataDir);
addArg(argv, '--agent-plugins-dir', tempAgentPluginsDir);
addArg(argv, '--agents-user-data-dir', tempAgentsUserDataDir);
addArg(argv, '--agents-extensions-dir', tempAgentsExtensionsDir);
```

**Key patterns:**
- `addArg()` helper function mutates argv array in-place
- Consistent pattern: key followed by value
- Used to inject dynamically-computed paths (temp directories, user data)
- Arguments accumulated before process spawn

---

## Pattern 7: Process Callback Attachment for Async Lifecycle
**Where:** `src/vs/code/node/cli.ts:230-243`
**What:** Registering callbacks to handle process output and events

```typescript
const processCallbacks: ((child: ChildProcess) => Promise<void>)[] = [];

if (args.verbose || args.status) {
	processCallbacks.push(async child => {
		child.stdout?.on('data', (data: Buffer) => console.log(data.toString('utf8').trim()));
		child.stderr?.on('data', (data: Buffer) => console.log(data.toString('utf8').trim()));

		await Event.toPromise(Event.fromNodeEventEmitter(child, 'exit'));
	});
}
```

**Key patterns:**
- Callback array pattern for conditional process handling
- Each callback receives the spawned ChildProcess
- Attaches event listeners for stdout/stderr data
- Awaits exit event using Event utility (RxJS-like pattern)
- Allows multiple independent callbacks to be composed

---

## Pattern 8: Utility Process Spawning via Electron API
**Where:** `src/vs/platform/utilityProcess/electron-main/utilityProcess.ts:261-268`
**What:** Using Electron's native `utilityProcess.fork()` instead of Node's child_process

```typescript
this.process = utilityProcess.fork(modulePath, args, {
	serviceName,
	env,
	execArgv,
	allowLoadingUnsignedLibraries,
	respondToAuthRequestsFromMainProcess,
	stdio
});
```

**Key patterns:**
- `utilityProcess.fork()` (Electron API) replaces `child_process.fork()`
- Takes module path (ESM entry point)
- `execArgv` for Node.js-level flags (like `--js-flags`)
- `serviceName` used for process identification in crash reports
- `allowLoadingUnsignedLibraries` for native module loading
- Environment variables and stdio configured identically to child_process

---

## Pattern 9: Environment Variable Filtering and Validation
**Where:** `src/vs/platform/utilityProcess/electron-main/utilityProcess.ts:276-302`
**What:** Sanitizing and validating environment before passing to subprocess

```typescript
const env: NodeJS.ProcessEnv = configuration.env ? { ...configuration.env } : { ...deepClone(process.env) };

env['VSCODE_ESM_ENTRYPOINT'] = configuration.entryPoint;
if (typeof configuration.parentLifecycleBound === 'number') {
	env['VSCODE_PARENT_PID'] = String(configuration.parentLifecycleBound);
}
env['VSCODE_CRASH_REPORTER_PROCESS_TYPE'] = configuration.type;

if (isWindows) {
	if (isUNCAccessRestrictionsDisabled()) {
		env['NODE_DISABLE_UNC_ACCESS_CHECKS'] = '1';
	} else {
		env['NODE_UNC_HOST_ALLOWLIST'] = getUNCHostAllowlist().join('\\');
	}
}

removeDangerousEnvVariables(env);

// Ensure all values are strings
for (const key of Object.keys(env)) {
	env[key] = String(env[key]);
}
```

**Key patterns:**
- Allow config-provided env or clone process.env
- Inject VS Code-specific variables (VSCODE_*)
- Platform-specific handling (UNC paths on Windows)
- Call `removeDangerousEnvVariables()` security helper
- Coerce all values to strings (requirement for spawn)

---

## Pattern 10: Exit Event Handling and Process Cleanup
**Where:** `src/vs/platform/utilityProcess/electron-main/utilityProcess.ts:334-342`
**What:** Listening to process exit events and emitting structured results

```typescript
this._register(Event.fromNodeEventEmitter<number>(process, 'exit')(code => {
	this.log(`received exit event with code ${code}`, Severity.Info);

	// Event
	this._onExit.fire({ pid: this.processPid!, code, signal: 'unknown' });

	// Cleanup
	this.onDidExitOrCrashOrKill();
}));
```

**Key patterns:**
- Register via disposable pattern to auto-cleanup on dispose
- Listen to 'exit' event from Node process
- Emit structured event with pid, code, signal
- Call cleanup method after event fired
- Log exit code for diagnostics

---

## Summary of Port-Relevant Patterns

### Electron-to-Rust Transitions
1. **Tunnel spawning** (Pattern 1) — currently spawns `cargo run` in dev or binary in prod. In Rust, this is the entry point itself; no subprocess needed.
2. **Argv marshalling** (Patterns 3, 6) — argv arrays are built dynamically, mutated via helpers, and sliced before spawn. Rust CLI receives `std::env::args()` directly; consider argument builder pattern.
3. **Environment variables** (Patterns 2, 5, 9) — Electron-specific vars deleted; VS Code vars injected; sanitization via `removeDangerousEnvVariables()`. Rust equivalent should sanitize similarly.
4. **macOS `open` command** (Pattern 4) — special handling for window lifecycle. Tauri may handle this transparently.
5. **Utility processes** (Pattern 8) — spawned via `utilityProcess.fork()`. Tauri will use OS-level process spawning or Tauri's plugin system.

### Concrete Argv Patterns to Replicate
- `argv.slice(2)` to skip executable and script paths
- Key-value argument pairs: `--name value`
- Argument filtering based on mode (transient, verbose, wait)
- Temp file paths for stdin/stdout redirection
- Marker files for wait behavior (`--waitMarkerFilePath`)

### CLI Entry Point (`src/cli.ts`)
The actual `src/cli.ts` is minimal (26 LOC):
- Imports bootstrap modules
- Sets NLS configuration via `process.env['VSCODE_NLS_CONFIG']`
- Calls `configurePortable()`
- Calls `bootstrapESM()`
- Imports `vs/code/node/cli.js` which contains the main CLI logic

The Rust port should replicate this structure: initialize globals, load config, then invoke the core CLI handler.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
