# Port Forwarding Extension Patterns

## Overview
The `extensions/tunnel-forwarding/` directory implements VS Code's tunnel provider interface to enable local port forwarding functionality. Key patterns include tunnel provider registration, process lifecycle management, state machines, and event-driven architecture.

---

## Patterns Found

#### Pattern 1: Tunnel Provider Registration
**Where:** `/Users/norinlavaee/vscode-atomic/extensions/tunnel-forwarding/src/extension.ts:93-105`
**What:** Registration of a tunnel provider implementation with the VS Code workspace API, including feature capability declaration.

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

**Variations:** Also appears via class declaration at line 139 (`class TunnelProvider implements vscode.TunnelProvider`). The registration is awaited and added to `context.subscriptions.push()` for proper lifecycle management.

---

#### Pattern 2: Event Emitter Pattern with Dispose
**Where:** `/Users/norinlavaee/vscode-atomic/extensions/tunnel-forwarding/src/extension.ts:44-62`
**What:** Tunnel class implementing the vscode.Tunnel interface with event emitter for disposal tracking.

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

**Variations:** Consumer pattern at line 172-175 where `tunnel.onDidDispose()` listener updates tunnel set and active ports.

---

#### Pattern 3: Discriminated Union State Machine
**Where:** `/Users/norinlavaee/vscode-atomic/extensions/tunnel-forwarding/src/extension.ts:64-75`
**What:** Type-safe state management using discriminated union types and getter/setter pattern with event emission.

```typescript
const enum State {
	Starting,
	Active,
	Inactive,
	Error,
}

type StateT =
	| { state: State.Inactive }
	| { state: State.Starting; process: ChildProcessWithoutNullStreams; cleanupTimeout?: NodeJS.Timeout }
	| { state: State.Active; portFormat: string; process: ChildProcessWithoutNullStreams; cleanupTimeout?: NodeJS.Timeout }
	| { state: State.Error; error: string };
```

State transitions with observer pattern (lines 144-151):
```typescript
private get state(): StateT {
	return this._state;
}

private set state(state: StateT) {
	this._state = state;
	this.stateChange.fire(state);
}

public readonly onDidStateChange = this.stateChange.event;
```

---

#### Pattern 4: Deferred Promise Pattern
**Where:** `/Users/norinlavaee/vscode-atomic/extensions/tunnel-forwarding/src/deferredPromise.ts:16-62`
**What:** Custom promise wrapper for external resolution, enabling promise creation and resolution separation.

```typescript
export class DeferredPromise<T> {

	private completeCallback!: ValueCallback<T>;
	private errorCallback!: (err: unknown) => void;
	private outcome?: { outcome: DeferredOutcome.Rejected; value: any } | { outcome: DeferredOutcome.Resolved; value: T };

	public get isRejected() {
		return this.outcome?.outcome === DeferredOutcome.Rejected;
	}

	public get isResolved() {
		return this.outcome?.outcome === DeferredOutcome.Resolved;
	}

	public get isSettled() {
		return !!this.outcome;
	}

	public readonly p: Promise<T>;

	constructor() {
		this.p = new Promise<T>((c, e) => {
			this.completeCallback = c;
			this.errorCallback = e;
		});
	}

	public complete(value: T) {
		return new Promise<void>(resolve => {
			this.completeCallback(value);
			this.outcome = { outcome: DeferredOutcome.Resolved, value };
			resolve();
		});
	}
}
```

**Variations:** Used at lines 283-294 for progress notification lifecycle.

---

#### Pattern 5: Child Process Lifecycle and Stream Handling
**Where:** `/Users/norinlavaee/vscode-atomic/extensions/tunnel-forwarding/src/extension.ts:266-342`
**What:** Complete child process spawning with authentication, I/O stream handling, and event listeners.

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

	const progressP = new DeferredPromise<void>();
	vscode.window.withProgress(
		{
			location: vscode.ProgressLocation.Notification,
			title: vscode.l10n.t({
				comment: ['do not change link format [Show Log](command), only change the text "Show Log"'],
				message: 'Starting port forwarding system ([Show Log]({0}))',
				args: ['command:tunnel-forwarding.showLog']
			}),
		},
		() => progressP.p,
	);

	let lastPortFormat: string | undefined;
	child.on('exit', status => {
		const msg = `[forwarding] exited with code ${status}`;
		this.logger.log('info', msg);
		progressP.complete(); // make sure to clear progress on unexpected exit
		if (this.isInStateWithProcess(child)) {
			this.state = { state: State.Error, error: msg };
		}
	});

	child.on('error', err => {
		this.logger.log('error', `[forwarding] ${err}`);
		progressP.complete(); // make sure to clear progress on unexpected exit
		if (this.isInStateWithProcess(child)) {
			this.state = { state: State.Error, error: String(err) };
		}
	});

	child.stdout
		.pipe(splitNewLines())
		.on('data', line => this.logger.log('info', `[forwarding] ${line}`))
		.resume();

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

	await new Promise((resolve, reject) => {
		child.on('spawn', resolve);
		child.on('error', reject);
	});
}
```

---

#### Pattern 6: Stream Splitting Transform
**Where:** `/Users/norinlavaee/vscode-atomic/extensions/tunnel-forwarding/src/split.ts:15-51`
**What:** Custom Node.js Transform stream for splitting input by delimiter without including split character.

```typescript
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

**Variations:** Factory function at line 8: `export const splitNewLines = () => new StreamSplitter('\n'.charCodeAt(0));`

---

#### Pattern 7: Tunnel Provider Interface Implementation
**Where:** `/Users/norinlavaee/vscode-atomic/extensions/tunnel-forwarding/src/extension.ts:139-201`
**What:** Core provider interface implementation with tunnel provisioning logic including state waiting, consent, and lifecycle management.

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
		if (tunnelOptions.privacy === TunnelPrivacyId.Public) {
			if (!(await this.consentPublicPort(tunnelOptions.remoteAddress.port))) {
				return;
			}
		}

		const tunnel = new Tunnel(
			tunnelOptions.remoteAddress,
			(tunnelOptions.privacy as TunnelPrivacyId) || TunnelPrivacyId.Private,
			tunnelOptions.protocol === 'https' ? 'https' : 'http',
		);

		this.tunnels.add(tunnel);
		tunnel.onDidDispose(() => {
			this.tunnels.delete(tunnel);
			this.updateActivePortsIfRunning();
		});

		switch (this.state.state) {
			case State.Error:
			case State.Inactive:
				await this.setupPortForwardingProcess();
			// fall through since state is now starting
			case State.Starting:
				this.updateActivePortsIfRunning();
				return new Promise<Tunnel>((resolve, reject) => {
					const l = this.stateChange.event(state => {
						if (state.state === State.Active) {
							tunnel.setPortFormat(state.portFormat);
							l.dispose();
							resolve(tunnel);
						} else if (state.state === State.Error) {
							l.dispose();
							reject(new Error(state.error));
						}
					});
				});
			case State.Active:
				tunnel.setPortFormat(this.state.portFormat);
				this.updateActivePortsIfRunning();
				return tunnel;
		}
	}
}
```

---

## Summary

The tunnel-forwarding extension demonstrates key patterns for porting VS Code core port forwarding capabilities to Tauri/Rust:

1. **Event-driven architecture** via vscode.EventEmitter for managing tunnel lifecycle and state changes
2. **Discriminated union type state machine** enabling type-safe state transitions with associated data
3. **Process lifecycle management** with proper resource cleanup and timeout-based idle shutdown (CLEANUP_TIMEOUT = 10s)
4. **Stream transformation patterns** for parsing subprocess output (newline-delimited JSON)
5. **External promise resolution** via DeferredPromise for coordinating async operations
6. **Provider registration pattern** showing how to integrate with VS Code's workspace API with feature capabilities
7. **Authentication integration** using vscode.authentication.getSession for GitHub OAuth scoped to email and org read
8. **Tunnel interface implementation** including privacy consent flows, port address formatting, and tunnel disposal tracking

Key architectural notes: The implementation uses a single spawned CLI subprocess that accepts JSON-formatted port configurations over stdin and emits port format strings on stderr. The cleanup timeout prevents subprocess death thrashing during privacy changes. Port listening state transitions are triggered by stderr JSON parsing, establishing the contract between the subprocess and the extension's state machine.

All patterns cluster around the core abstraction of a TunnelProvider that manages a Set of Tunnel instances, each implementing vscode.Tunnel's disposal contract.
