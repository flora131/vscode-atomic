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

