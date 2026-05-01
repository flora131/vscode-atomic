# Partition 44 of 79 — Findings

## Scope
`extensions/tunnel-forwarding/` (4 files, 474 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Partition 44: VS Code Tunnel Forwarding Extension & Port Forwarding Porting Guide

## Overview
The `extensions/tunnel-forwarding/` directory (4 source files, 474 LOC) implements VS Code's port-forwarding extension as a TypeScript/Electron plugin. It interfaces with the Rust CLI `cli/src/tunnels/` module (25 files, 7579 LOC) which provides the underlying tunnel and port-forwarding infrastructure. Together, they manage local port forwarding to internet-accessible URLs via VS Code's tunnel service.

---

## Implementation Files

### TypeScript Extension (4 files)

**Primary Entry Point:**
- `extensions/tunnel-forwarding/src/extension.ts` (344 LOC)
  - `TunnelProvider` class implements `vscode.TunnelProvider` API
  - Spawns CLI child process via `child_process.spawn(cliPath, args)`
  - State machine manages: Inactive → Starting → Active → Error
  - Handles tunnel lifecycle (create, update, dispose) via event emitters
  - Receives JSON port format config from CLI stdout: `{ port_format: string }`
  - Manages cleanup timeout (10s) to avoid CLI restart thrashing
  - Prompts GitHub OAuth via `vscode.authentication.getSession('github', ...)`
  - Sends port list updates to CLI stdin as JSON: `[{number, privacy, protocol}]`

**Utility Modules:**
- `extensions/tunnel-forwarding/src/deferredPromise.ts` (62 LOC)
  - Promise wrapper with explicit resolve/reject callbacks
  - Copied from `src/vs/base/common/async.ts`
  - Provides `DeferredPromise<T>` with state tracking (isResolved/isRejected/isSettled)

- `extensions/tunnel-forwarding/src/split.ts` (52 LOC)
  - Stream utility for splitting newline-delimited data
  - `StreamSplitter extends Transform` for processing CLI stdout/stderr
  - Copied and simplified from `src/vs/base/node/nodeStreams.ts`

### Build Configuration
- `extensions/tunnel-forwarding/esbuild.mts` (19 LOC)
  - esbuild config for TypeScript compilation
  - Entry point: `src/extension.ts` → output: `dist/extension`
  - Platform: Node.js (not browser)

---

## Configuration Files

**Extension Manifest:**
- `extensions/tunnel-forwarding/package.json`
  - Display name: "Local Tunnel Port Forwarding"
  - Version: 10.0.0
  - Publisher: vscode
  - VS Code engine: ^1.82.0
  - API proposals enabled:
    - `resolvers` (for custom resolver API)
    - `tunnelFactory` (for tunnel provider factory)
  - Activation event: `onTunnel`
  - Commands:
    - `tunnel-forwarding.showLog` - Show CLI output channel
    - `tunnel-forwarding.restart` - Force CLI restart
  - Main entry: `./out/extension`

**Localization:**
- `extensions/tunnel-forwarding/package.nls.json`
  - displayName: "Local Tunnel Port Forwarding"
  - description: "Allows forwarding local ports to be accessible over the internet."
  - Category: "Port Forwarding"

**TypeScript Compilation:**
- `extensions/tunnel-forwarding/tsconfig.json`
  - Extends `../tsconfig.base.json`
  - Root dir: `./src`, output: `./out`
  - Type roots: Node.js types + three proposed VS Code APIs:
    - `vscode.d.ts` (base API)
    - `vscode.proposed.resolvers.d.ts`
    - `vscode.proposed.tunnelFactory.d.ts`

**Runtime Configuration:**
- `extensions/tunnel-forwarding/.npmrc`
  - Legacy peer dependency support
  - 180s npm operation timeout
  - Minimum release age: 1 day

**Package Exclusions:**
- `extensions/tunnel-forwarding/.vscodeignore`
  - Excludes source files, tsconfig, build outputs, esbuild config, and lock files from packaged extension

**Debug Configuration:**
- `extensions/tunnel-forwarding/.vscode/launch.json`
  - Extension host launch config
  - Dev flag: `VSCODE_FORWARDING_IS_DEV=1` loads CLI from OSS build
  - Load CLI from `cli/target/debug/code` during development

---

## Rust CLI Tunnel Infrastructure (Integration Boundary)

### Core Port Forwarding (6 files, ~1200 LOC)
- `cli/src/tunnels/port_forwarder.rs`
  - `PortForwardingProcessor` with async message handler pattern
  - Enum: `PortForwardingRec::Forward(port, privacy, oneshot_sender)` / `::Unforward(...)`
  - Port blocking: prevents forwarding control ports (AGENT_HOST_PORT, CONTROL_PORT)
  - Manages HashSet of active forwarded ports
  - Integration point: Called from within `ActiveTunnel` context

- `cli/src/tunnels/local_forwarding.rs` (~250 LOC)
  - `PortCount` struct tracks public/private port counts
  - Singleton server for local port forwarding requests
  - Handles JSON RPC protocol: parses port lists from stdin, responds via protocol

### Protocol & Data Contracts (2 files)
- `cli/src/tunnels/protocol.rs` (~400 LOC)
  - Enum `ClientRequestMethod`: servermsg, serverclose, serverlog, makehttpreq, version
  - Structs: `ForwardParams` (port, public), `UnforwardParams`, `ForwardResult` (uri)
  - Structs: `PortProtocol` (Auto/Http/Https), `PortPrivacy` (Public/Private)
  - HTTP request/response protocol serialization

- `cli/src/tunnels/dev_tunnels.rs` (~2000+ LOC, largest module)
  - `ActiveTunnel` struct: represents live tunnel connection
  - `add_port_tcp(port, privacy, protocol)` - initiates port forwarding
  - `remove_port(port)` - closes forwarding
  - Methods: `add_port_tcp()`, `remove_port()`, manage tunnel lifecycle
  - Trait: `AccessTokenProvider` - token refresh pattern
  - OAuth integration for GitHub authentication
  - Tunnel state persistence: `PersistedTunnel` serialization

### Service & Connectivity (12 files, ~4000+ LOC)
- `cli/src/tunnels/agent_host.rs` - Agent connection management
- `cli/src/tunnels/code_server.rs` - VS Code server integration
- `cli/src/tunnels/control_server.rs` - Control plane communication
- `cli/src/tunnels/server_bridge.rs` - Bridge between local and tunnel server
- `cli/src/tunnels/server_multiplexer.rs` - Multiplex connections
- `cli/src/tunnels/socket_signal.rs` - Signal handling
- `cli/src/tunnels/singleton_server.rs` / `singleton_client.rs` - IPC pattern
- `cli/src/tunnels/challenge.rs` - Authentication challenge protocol
- `cli/src/tunnels/shutdown_signal.rs` - Graceful shutdown signaling

### Platform-Specific & Utilities (9 files)
- `cli/src/tunnels/service.rs` - Core service abstraction
- `cli/src/tunnels/service_windows.rs` - Windows service integration
- `cli/src/tunnels/service_macos.rs` - macOS service integration
- `cli/src/tunnels/service_linux.rs` - Linux service integration
- `cli/src/tunnels/nosleep.rs` - Prevent system sleep
- `cli/src/tunnels/nosleep_windows.rs` / `nosleep_macos.rs` / `nosleep_linux.rs` - Platform sleep prevention
- `cli/src/tunnels/wsl_detect.rs` - WSL environment detection
- `cli/src/tunnels/paths.rs` - Path resolution
- `cli/src/tunnels/legal.rs` - License/legal text

---

## Test Files
None present in TypeScript extension (no test files found with .test.ts/.spec.ts patterns).

---

## Notable Clusters & Architecture

### Data Flow (TypeScript → Rust CLI)
1. **Extension Activation** (`activationEvents: ["onTunnel"]`)
   - Registers with VS Code via `vscode.workspace.registerTunnelProvider(provider)`
   - Subscribes to tunnel requests via `provideTunnel(tunnelOptions)`

2. **Process Spawning**
   - CLI path resolution: dev (`cli/target/debug/code`) or prod (bundled `code-tunnel`/`code-tunnel-insiders`)
   - Spawn args: `['--verbose', 'tunnel', 'forward-internal', '--provider', 'github']`
   - Environment: GitHub OAuth token passed via `VSCODE_CLI_ACCESS_TOKEN`

3. **Protocol (JSON Lines over stdin/stdout)**
   - **Ext→CLI (stdin)**: Port configuration array each update
     ```
     [{"number":3000,"privacy":"private","protocol":"http"}]
     ```
   - **CLI→Ext (stderr)**: JSON log lines parsed for `port_format`
     ```
     {"port_format":"http://localhost:{port}"}
     ```
   - **CLI→Ext (stdout)**: Verbose logging (piped via splitNewLines)

4. **State Transitions**
   - Starting: Child process spawned, progress indicator shown
   - Active: Received `port_format` from CLI, tunnels initialized
   - Error: Process exit or error event, state holds error message
   - Inactive: All tunnels disposed and cleanup timeout expires

### Extension Integration Points
- **API Proposals**: Uses unstable `resolvers` and `tunnelFactory` APIs (may change)
- **VS Code Context**: Sets `tunnelForwardingIsRunning` and `tunnelForwardingHasLog` context keys
- **GitHub Auth**: Automatic session creation via OAuth; error if unavailable
- **Output Channel**: Async log output channel created on first log message

### CLI Integration Contract
- **Input**: Port list updates (JSON) on stdin
- **Output**: Port format string and logs on stderr/stdout
- **Lifecycle**: Stays running while ports exist; cleans up after 10s idle timeout
- **Error Handling**: Process exit/error triggers state transition; displayed to user

---

## Porting Considerations for Tauri/Rust

### What Would Need to Change

**Elimination of Intermediate Layer:**
- Current: VS Code Extension (TypeScript) ↔ Child Process (Rust CLI) ↔ Tunnel Service
- Porting: Embed tunnel logic directly in Tauri app (single-process, no IPC)
- Benefit: No JSON serialization overhead, direct Rust-to-Rust API calls

**API Surface Shifts:**
- TypeScript `vscode.TunnelProvider` API → Custom Tauri command/event system
- Port format strings, privacy enums → Native Rust structures
- GitHub OAuth integration → Tauri plugin or native HTTP client

**Process Management:**
- Current: Extension spawns CLI subprocess, manages streams, handles signals
- Porting: Call Rust tunnel functions directly; no child_process module needed
- Signal handling: Use Tauri lifecycle events instead of OS signals

**Type System Benefits:**
- Current: JSON parsing/validation on each update (overhead)
- Porting: Type-safe function calls with serde serialization only at IPC boundary

### Leverageable Rust Modules
All 25 Rust CLI tunnel files are directly reusable:
1. `port_forwarder.rs` - Core forwarding logic (copy directly)
2. `dev_tunnels.rs` - Tunnel lifecycle (drop CLI parsing, keep tunnel ops)
3. `protocol.rs` - Message types and serialization contracts
4. `local_forwarding.rs` - Port list management
5. Platform services (`service_*.rs`, `nosleep_*.rs`) - Keep as-is for background service

### Remaining Challenges
1. **GitHub OAuth**: Currently handled via CLI child process; would need Tauri oauth plugin
2. **Service Mode**: Windows/macOS/Linux service registration; Tauri has platform-specific APIs
3. **Logging**: Replace VS Code Output Channel with Tauri logging system
4. **Auto-Update**: CLI currently auto-updates; Tauri handles differently (built-in updater)

---

## Compilation & Build

- TypeScript: `gulp compile-extension:tunnel-forwarding` (main), `gulp watch-extension:tunnel-forwarding` (dev watch)
- Output: Compiled to `out/` directory, esbuild config in `esbuild.mts`
- DevDependencies: Only `@types/node@22.x` (minimal)
- Prettier config: 100-char line width, trailing commas, single quotes

---

## Summary

The tunnel-forwarding extension is a **thin TypeScript wrapper** (~470 LOC) around a mature Rust CLI (~7500 LOC) that handles port forwarding via VS Code's tunnels infrastructure. The extension:
- Manages process lifecycle and bidirectional IPC
- Handles OAuth authentication (delegated to CLI)
- Presents port privacy/protocol options in VS Code UI
- Buffers port updates and sends to CLI via JSON serialization

For a Tauri/Rust port, the extension layer could be **eliminated entirely** by embedding the Rust tunnel modules directly in the app. The 25 Rust modules are well-separated and mostly independent of the CLI harness. Key integration points are the `TunnelProvider` API (replaced by Tauri commands) and OAuth (needs Tauri plugin). The port forwarding, protocol, and service logic require minimal adaptation.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Port-Forwarding Extension: Patterns for Tauri/Rust Port

## Pattern 1: TunnelProvider Registration API
**Found in**: `extensions/tunnel-forwarding/src/extension.ts:93-106`
**Used for**: Registering the custom tunnel provider with VS Code's tunnel system

```typescript
await vscode.workspace.registerTunnelProvider(
	provider,
	{
		tunnelFeatures: {
			elevation: false,
			protocol: true,
			privacyOptions: [
				{ themeIcon: 'globe', id: TunnelPrivacyId.Public, label: vscode.l10n.t('Public') },
				{ themeIcon: 'lock', id: TunnelPrivacyId.Private, label: vscode.l10n.t('Private') },
			],
		},
	},
);
```

**Key aspects**:
- Registers with `vscode.workspace.registerTunnelProvider()` which is the main API surface
- Provides `TunnelInformation` object with `tunnelFeatures` metadata
- Privacy options are exposed as user-facing choices (Public/Private)
- Protocol support flag indicates HTTP/HTTPS capability
- Extension must await the registration promise

**Variations**: This is the sole registration point; multiple providers are not allowed (enforced in `extHostTunnelService.ts:152-154`).

---

## Pattern 2: TunnelProvider Implementation Class
**Found in**: `extensions/tunnel-forwarding/src/extension.ts:139-343`
**Used for**: Implementing the vscode.TunnelProvider interface

```typescript
class TunnelProvider implements vscode.TunnelProvider {
	private readonly tunnels = new Set<Tunnel>();
	private readonly stateChange = new vscode.EventEmitter<StateT>();
	private _state: StateT = { state: State.Inactive };

	private get state(): StateT {
		return this._state;
	}

	private set state(state: StateT) {
		this._state = state;
		this.stateChange.fire(state);
	}

	public readonly onDidStateChange = this.stateChange.event;

	constructor(private readonly logger: Logger, private readonly context: vscode.ExtensionContext) { }

	/** @inheritdoc */
	public async provideTunnel(tunnelOptions: vscode.TunnelOptions): Promise<vscode.Tunnel | undefined> {
		// Implementation...
	}
}
```

**Key aspects**:
- Must implement `provideTunnel(tunnelOptions): Promise<vscode.Tunnel | undefined>`
- Manages tunnel lifecycle using event emitters
- Tracks active tunnels in a Set
- State machine pattern (Inactive/Starting/Active/Error)
- Uses internal logger for diagnostic output
- Accesses `ExtensionContext` for persistent state storage

---

## Pattern 3: CLI Process Spawning for Port Forwarding
**Found in**: `extensions/tunnel-forwarding/src/extension.ts:266-342`
**Used for**: Launching and managing the Rust CLI tunnel forwarding process

```typescript
private async setupPortForwardingProcess() {
	const session = await vscode.authentication.getSession('github', ['user:email', 'read:org'], {
		createIfNone: true,
	});

	const args = [
		'--verbose',
		'tunnel',
		'forward-internal',
		'--provider',
		'github',
	];

	this.logger.log('info', '[forwarding] starting CLI');
	const child = spawn(cliPath, args, { stdio: 'pipe', env: { ...process.env, NO_COLOR: '1', VSCODE_CLI_ACCESS_TOKEN: session.accessToken } });
	this.state = { state: State.Starting, process: child };

	// Monitor stdout for port format string
	child.stdout
		.pipe(splitNewLines())
		.on('data', line => this.logger.log('info', `[forwarding] ${line}`))
		.resume();

	// Monitor stderr for JSON output
	child.stderr
		.pipe(splitNewLines())
		.on('data', line => {
			try {
				const l: { port_format: string } = JSON.parse(line);
				if (l.port_format && l.port_format !== lastPortFormat) {
					this.state = {
						state: State.Active,
						portFormat: l.port_format, process: child,
						cleanupTimeout: 'cleanupTimeout' in this.state ? this.state.cleanupTimeout : undefined,
					};
					progressP.complete();
				}
			} catch (e) {
				this.logger.log('error', `[forwarding] ${line}`);
			}
		})
		.resume();
}
```

**Key aspects**:
- Obtains GitHub authentication token via `vscode.authentication.getSession()`
- Spawns CLI with `stdio: 'pipe'` for stream-based communication
- Passes token via environment variable `VSCODE_CLI_ACCESS_TOKEN`
- Listens on both stdout and stderr
- Stderr contains JSON messages with `port_format` field (output is expected to be JSON)
- Uses stream splitting utility to parse newline-delimited output
- Progress UI shown during startup via `vscode.window.withProgress()`
- Handles process exit and error events for state management

**CLI Path Resolution**: `extensions/tunnel-forwarding/src/extension.ts:24-42`
```typescript
const versionFolder = vscode.env.appCommit?.substring(0, 10);
let cliPath: string;
if (process.env.VSCODE_FORWARDING_IS_DEV) {
	cliPath = path.join(__dirname, '../../../cli/target/debug/code');
} else {
	let binPath: string;
	if (process.platform === 'darwin') {
		binPath = 'bin';
	} else if (process.platform === 'win32' && versionFolder && vscode.env.appRoot.includes(versionFolder)) {
		binPath = '../../../bin';
	} else {
		binPath = '../../bin';
	}

	const cliName = vscode.env.appQuality === 'stable' ? 'code-tunnel' : 'code-tunnel-insiders';
	const extension = process.platform === 'win32' ? '.exe' : '';

	cliPath = path.join(vscode.env.appRoot, binPath, cliName) + extension;
}
```

---

## Pattern 4: JSON-Based IPC with CLI Process
**Found in**: `extensions/tunnel-forwarding/src/extension.ts:250-264`
**Used for**: Communicating active port configuration to the CLI process

```typescript
private updateActivePortsIfRunning() {
	if (this.state.state !== State.Starting && this.state.state !== State.Active) {
		return;
	}

	const ports = [...this.tunnels].map(t => ({ number: t.remoteAddress.port, privacy: t.privacy, protocol: t.protocol }));
	this.state.process.stdin.write(`${JSON.stringify(ports)}\n`);

	if (ports.length === 0 && !this.state.cleanupTimeout) {
		this.state.cleanupTimeout = setTimeout(() => this.killRunningProcess(), CLEANUP_TIMEOUT);
	} else if (ports.length > 0 && this.state.cleanupTimeout) {
		clearTimeout(this.state.cleanupTimeout);
		this.state.cleanupTimeout = undefined;
	}
}
```

**Key aspects**:
- Sends newline-delimited JSON arrays to process stdin
- Port message structure: `{ number, privacy, protocol }`
- Privacy field carries the TunnelPrivacyId enum value (e.g., 'private', 'public')
- Automatic cleanup timeout when no active ports (10 seconds as per `CLEANUP_TIMEOUT = 10_000`)
- Allows graceful shutdown without immediate process kill

---

## Pattern 5: Tunnel Object Implementation
**Found in**: `extensions/tunnel-forwarding/src/extension.ts:44-62`
**Used for**: Creating tunnel objects returned to VS Code

```typescript
class Tunnel implements vscode.Tunnel {
	private readonly disposeEmitter = new vscode.EventEmitter<void>();
	public readonly onDidDispose = this.disposeEmitter.event;
	public localAddress!: string;

	constructor(
		public readonly remoteAddress: { port: number; host: string },
		public readonly privacy: TunnelPrivacyId,
		public readonly protocol: 'http' | 'https',
	) { }

	public setPortFormat(formatString: string) {
		this.localAddress = formatString.replace('{port}', String(this.remoteAddress.port));
	}

	dispose() {
		this.disposeEmitter.fire();
	}
}
```

**Key aspects**:
- Implements `vscode.Tunnel` interface with required properties
- `remoteAddress`: target port/host on machine running tunnel CLI
- `localAddress`: formatted URL for client to connect to (e.g., "localhost:5000")
- Port format comes from CLI as template string (e.g., "http://localhost:{port}")
- `privacy` field is a TunnelPrivacyId enum value
- `protocol` indicates HTTP or HTTPS for clients
- Fires `onDidDispose` event for cleanup tracking

---

## Pattern 6: PublicPort Consent UI
**Found in**: `extensions/tunnel-forwarding/src/extension.ts:210-233`
**Used for**: Requiring user confirmation before exposing ports publicly

```typescript
private async consentPublicPort(portNumber: number) {
	const didWarn = this.context.globalState.get(didWarnPublicKey, false);
	if (didWarn) {
		return true;
	}

	const continueOpt = vscode.l10n.t('Continue');
	const dontShowAgain = vscode.l10n.t("Don't show again");
	const r = await vscode.window.showWarningMessage(
		vscode.l10n.t("You're about to create a publicly forwarded port. Anyone on the internet will be able to connect to the service listening on port {0}. You should only proceed if this service is secure and non-sensitive.", portNumber),
		{ modal: true },
		continueOpt,
		dontShowAgain,
	);
	if (r === continueOpt) {
		// continue
	} else if (r === dontShowAgain) {
		await this.context.globalState.update(didWarnPublicKey, true);
	} else {
		return false;
	}

	return true;
}
```

**Key aspects**:
- Checks persistent global state to avoid repeated warnings
- Uses modal warning dialog for security-sensitive action
- Offers "Don't show again" option that persists in global extension state
- Only called when `tunnelOptions.privacy === TunnelPrivacyId.Public`
- Returns false to cancel port forwarding if user declines

---

## Pattern 7: Port Attributes Provider API (Related API Surface)
**Found in**: `src/vscode-dts/vscode.proposed.portsAttributes.d.ts`
**Used for**: Providing metadata about known ports for auto-forwarding behavior

```typescript
export namespace workspace {
	/**
	 * If your extension listens on ports, consider registering a PortAttributesProvider to provide information
	 * about the ports. For example, a debug extension may know about debug ports in it's debuggee. By providing
	 * this information with a PortAttributesProvider the extension can tell the editor that these ports should be
	 * ignored, since they don't need to be user facing.
	 */
	export function registerPortAttributesProvider(
		portSelector: PortAttributesSelector, 
		provider: PortAttributesProvider
	): Disposable;
}
```

**PortAttributesProvider Interface**:
```typescript
export interface PortAttributesProvider {
	providePortAttributes(
		attributes: { port: number; pid?: number; commandLine?: string }, 
		token: CancellationToken
	): ProviderResult<PortAttributes>;
}
```

**PortAttributes Class**:
```typescript
export class PortAttributes {
	autoForwardAction: PortAutoForwardAction;
	constructor(autoForwardAction: PortAutoForwardAction);
}
```

**PortAutoForwardAction Enum**:
```typescript
export enum PortAutoForwardAction {
	Notify = 1,
	OpenBrowser = 2,
	OpenPreview = 3,
	Silent = 4,
	Ignore = 5
}
```

**Key aspects**:
- Separate from TunnelProvider; used for auto-discovery behavior
- Allows extensions to annotate ports with auto-forward actions
- Port selector filters which ports trigger the provider (portRange or commandPattern)
- Results merged with user settings; user settings take precedence
- Referenced in `src/vs/workbench/api/common/extHostTunnelService.ts:106-118`

---

## Pattern 8: Stream Splitting Utility
**Found in**: `extensions/tunnel-forwarding/src/split.ts:1-52`
**Used for**: Parsing newline-delimited output from CLI process

```typescript
export const splitNewLines = () => new StreamSplitter('\n'.charCodeAt(0));

export class StreamSplitter extends Transform {
	private buffer: Buffer | undefined;

	constructor(private readonly splitter: number) {
		super();
	}

	override _transform(chunk: Buffer, _encoding: string, callback: (error?: Error | null, data?: any) => void): void {
		if (!this.buffer) {
			this.buffer = chunk;
		} else {
			this.buffer = Buffer.concat([this.buffer, chunk]);
		}

		let offset = 0;
		while (offset < this.buffer.length) {
			const index = this.buffer.indexOf(this.splitter, offset);
			if (index === -1) {
				break;
			}

			this.push(this.buffer.subarray(offset, index));
			offset = index + 1;
		}

		this.buffer = offset === this.buffer.length ? undefined : this.buffer.subarray(offset);
		callback();
	}

	override _flush(callback: (error?: Error | null, data?: any) => void): void {
		if (this.buffer) {
			this.push(this.buffer);
		}

		callback();
	}
}
```

**Key aspects**:
- Custom Transform stream for splitting on delimiter
- Handles partial chunks that don't end with delimiter
- Efficient buffer management using slicing
- Pattern: consumed by piping: `child.stdout.pipe(splitNewLines()).on('data', ...)`

---

## Pattern 9: Rust CLI Port Forwarder Architecture
**Found in**: `cli/src/tunnels/port_forwarder.rs:20-138`
**Used for**: Understanding the Rust-side port forwarding API structure

```rust
pub enum PortForwardingRec {
	Forward(u16, PortPrivacy, oneshot::Sender<Result<String, AnyError>>),
	Unforward(u16, oneshot::Sender<Result<(), AnyError>>),
}

pub struct PortForwardingProcessor {
	tx: mpsc::Sender<PortForwardingRec>,
	rx: mpsc::Receiver<PortForwardingRec>,
	forwarded: HashSet<u16>,
}

impl PortForwardingProcessor {
	pub async fn process(&mut self, req: PortForwardingRec, tunnel: &mut ActiveTunnel) {
		match req {
			PortForwardingRec::Forward(port, privacy, tx) => {
				tx.send(self.process_forward(port, privacy, tunnel).await)
					.ok();
			}
			PortForwardingRec::Unforward(port, tx) => {
				tx.send(self.process_unforward(port, tunnel).await).ok();
			}
		}
	}

	async fn process_forward(
		&mut self,
		port: u16,
		privacy: PortPrivacy,
		tunnel: &mut ActiveTunnel,
	) -> Result<String, AnyError> {
		if port == CONTROL_PORT || port == AGENT_HOST_PORT {
			return Err(CannotForwardControlPort().into());
		}

		if !self.forwarded.contains(&port) {
			tunnel
				.add_port_tcp(port, privacy, PortProtocol::Auto)
				.await?;
			self.forwarded.insert(port);
		}

		tunnel.get_port_uri(port)
	}
}
```

**Key aspects**:
- Async message-passing pattern using tokio channels
- Port privacy modeled as enum type (matches VS Code's TunnelPrivacyId)
- Returns port URI string on successful forward
- Prevents forwarding of control ports (CONTROL_PORT, AGENT_HOST_PORT)
- Request/response via oneshot channels for per-request async handling
- HashSet tracks already-forwarded ports to avoid duplicates

---

## Pattern 10: Port Forwarding Protocol Messages (Rust)
**Found in**: `cli/src/tunnels/protocol.rs:47-62`
**Used for**: Defining JSON protocol for port forwarding commands

```rust
#[derive(Deserialize, Debug)]
pub struct ForwardParams {
	pub port: u16,
	#[serde(default)]
	pub public: bool,
}

#[derive(Deserialize, Debug)]
pub struct UnforwardParams {
	pub port: u16,
}

#[derive(Serialize)]
pub struct ForwardResult {
	pub uri: String,
}
```

**Key aspects**:
- Serde JSON serialization/deserialization
- ForwardParams has optional `public` boolean (defaults to false)
- ForwardResult returns the complete URI to client
- Matches the port and privacy model from TypeScript side

---

## Integration Pattern Summary

The port-forwarding system uses a **stream-based IPC model**:

1. **Extension spawns CLI process** with GitHub authentication token
2. **CLI initializes and reports back** via stderr JSON with `port_format` template
3. **Extension monitors tunnel creation** via `provideTunnel()` calls
4. **Extension writes active ports** as newline-delimited JSON arrays to CLI stdin
5. **CLI maintains persistent tunnel** with active port list, responding to changes
6. **Tunnel cleanup** happens via timeout when no active ports (10 second grace period)

**Files involved in port-forwarding extension**:
- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/src/extension.ts` (main logic)
- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/src/deferredPromise.ts` (async utilities)
- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/src/split.ts` (stream parsing)
- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/package.json` (enabledApiProposals: tunnelFactory)

**Rust CLI counterparts**:
- `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/port_forwarder.rs`
- `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/protocol.rs`

**VS Code core tunnel APIs**:
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/tunnel/common/tunnel.ts` (ITunnelProvider, ITunnelService)
- `/home/norinlavaee/projects/vscode-atomic/src/vs/workbench/api/common/extHostTunnelService.ts` (extension-side implementation)
- `/home/norinlavaee/projects/vscode-atomic/src/vscode-dts/vscode.proposed.portsAttributes.d.ts` (public API defs)

---

## Porting Considerations for Tauri/Rust

**Direct porting opportunities**:
- The `PortForwardingProcessor` Rust code can largely be reused
- JSON protocol messages are already well-designed for inter-process communication
- Stream-based IPC can be adapted to Tauri's invoke/listen patterns or stdio

**Needed transformations**:
- TunnelProvider interface becomes Tauri command handler (e.g., `tauri::command`)
- CLI process spawning moves to Tauri's subprocess API
- EventEmitter pattern maps to Tauri's event system (`emit_all`, `listen`)
- Stream parsing (newline splitting) replicable in Tauri's line-based protocols
- Extension context (globalState) becomes Tauri's app state or persistent storage

**Authentication integration**:
- GitHub token obtained via `vscode.authentication` in TypeScript
- In Tauri, would need equivalent OAuth/auth token source
- Token passed via environment variable already works cross-platform

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
