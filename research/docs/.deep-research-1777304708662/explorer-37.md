# Partition 37 of 79 — Findings

## Scope
`extensions/vscode-test-resolver/` (7 files, 925 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator 37: vscode-test-resolver Extension

## Research Question
What would it take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust?

## Scope
`extensions/vscode-test-resolver/` — 7 TypeScript files, 925 LOC

---

### Implementation

- `extensions/vscode-test-resolver/src/extension.ts` — Core remote authority resolver that implements the registration seam (`workspace.registerRemoteAuthorityResolver('test', {...})`) at line 327. Demonstrates the contract between local UI and remote extension host: spawning child processes, managing socket proxies, connection state management, and the full lifecycle of a remote development session.

- `extensions/vscode-test-resolver/src/extension.browser.ts` — Browser-based remote authority resolver showcasing the managed connection model via `ManagedResolvedAuthority`. Uses WebSocket message passing instead of native sockets, illustrating how to decouple the local UI from the remote host via abstract message protocols.

- `extensions/vscode-test-resolver/src/download.ts` — Server distribution download and extraction logic. Handles platform-specific (Win32/Darwin/Linux) VS Code server archive retrieval and unpacking, demonstrating the multi-platform deployment pattern for remote hosts.

- `extensions/vscode-test-resolver/src/util/processes.ts` — Cross-platform process termination utility (Windows taskkill, Unix signals). Critical for remote session cleanup and lifecycle management.

### Configuration

- `extensions/vscode-test-resolver/package.json` — Declares remote authority resolver activation event (`onResolveRemoteAuthority:test`), API proposals for `resolvers` and `tunnels`, and tunnel factory implementation. Defines the extension manifest contract between VS Code UI and the remote harness.

- `extensions/vscode-test-resolver/tsconfig.json` — TypeScript configuration; includes type definitions from `vscode.proposed.resolvers.d.ts` and `vscode.proposed.tunnels.d.ts`, pointing to the type seams between UI and remote host.

- `extensions/vscode-test-resolver/tsconfig.browser.json` — Browser-variant TypeScript configuration for the WebSocket-based resolver.

### Build Configuration

- `extensions/vscode-test-resolver/esbuild.mts` — Node.js build entry point for the native resolver.

- `extensions/vscode-test-resolver/esbuild.browser.mts` — Browser build entry point for the managed WebSocket resolver.

### Development / Testing

- `extensions/vscode-test-resolver/.vscode/launch.json` — Debug configuration spawning VS Code with `--remote=test+test`, demonstrating the remote authority scheme convention.

- `extensions/vscode-test-resolver/scripts/terminateProcess.sh` — Unix process tree termination for cleaning up remote sessions on Darwin/Linux.

---

### Notable Clusters

- `extensions/vscode-test-resolver/src/` — 4 files implementing the dual remote authority model:
  - **Native resolver** (`extension.ts`) spawns child processes and uses TCP sockets for the extension host transport.
  - **Managed resolver** (`extension.browser.ts`) uses WebSocket message passing for environments (like web) without native process spawning.
  - **Utilities** (`download.ts`, `processes.ts`) provide cross-platform support for server distribution and lifecycle.

---

## Summary

The `vscode-test-resolver` extension demonstrates the **remote authority resolver architecture**—the critical seam between VS Code's local UI and a remote extension host. It showcases two transport models:

1. **Proxy-based** (native): Local resolver spawns a server process, maintains a TCP socket proxy between client and server, and manages connection state (pausing, slowing, reconnection).

2. **Managed messaging** (browser): Abstract message-passing protocol (send/receive/close events) decouples the UI from transport, enabling WebSocket or other protocols on the backend.

Both implementations register via `workspace.registerRemoteAuthorityResolver()` and fulfill the `ResolverResult` / `ResolvedAuthority` contract. This is the **architectural boundary** that any Rust-based remote backend (Tauri or otherwise) would need to replicate: the resolver must speak the VS Code extension API language (TypeScript interfaces) to negotiate the connection, then hand off to a custom transport layer. The extension demonstrates that this separation is already abstracted in the managed resolver, reducing friction for alternative runtimes.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Remote Authority Resolver Patterns in vscode-test-resolver

This document catalogs concrete code patterns from `extensions/vscode-test-resolver/` that implement VS Code's remote extension host test harness. These patterns show the architecture seam between local UI and remote host.

## Pattern Inventory

#### Pattern: Remote Authority Registration with Resolver Interface
**Where:** `extensions/vscode-test-resolver/src/extension.ts:327-344`
**What:** Core registration of remote authority resolver that implements the authority resolution contract with canonical URI handling, resolve method, tunnel factory, and port forwarding.

```typescript
const authorityResolverDisposable = vscode.workspace.registerRemoteAuthorityResolver('test', {
	async getCanonicalURI(uri: vscode.Uri): Promise<vscode.Uri> {
		return vscode.Uri.file(uri.path);
	},
	resolve(_authority: string): Thenable<vscode.ResolverResult> {
		return vscode.window.withProgress({
			location: vscode.ProgressLocation.Notification,
			title: 'Open TestResolver Remote ([details](command:vscode-testresolver.showLog))',
			cancellable: false
		}, async (progress) => {
			const rr = await doResolve(_authority, progress);
			rr.tunnelFeatures = getTunnelFeatures();
			return rr;
		});
	},
	tunnelFactory,
	showCandidatePort
});
```

**Variations / call-sites:** 
- Browser variant at `extension.browser.ts:9` shows minimal implementation with only resolver method.
- Dynamic re-registration at `extension.ts:364` demonstrates replacing resolver to simulate errors.

#### Pattern: Managed vs. Proxy Authority Resolution
**Where:** `extensions/vscode-test-resolver/src/extension.ts:210-324`
**What:** Two paths for authority resolution: `ManagedResolvedAuthority` (direct event-based messaging via WebSocket-like interface) and traditional proxy-based connection with socket forwarding.

```typescript
if (authority.includes('managed')) {
	console.log('Connecting via a managed authority');
	return Promise.resolve(new vscode.ManagedResolvedAuthority(async () => {
		const remoteSocket = net.createConnection({ port: serverAddr.port });
		const dataEmitter = new vscode.EventEmitter<Uint8Array<ArrayBuffer>>();
		const closeEmitter = new vscode.EventEmitter<Error | undefined>();
		const endEmitter = new vscode.EventEmitter<void>();

		await new Promise((res, rej) => {
			remoteSocket.on('data', d => dataEmitter.fire(d as Uint8Array<ArrayBuffer>))
				.on('error', err => { rej(); closeEmitter.fire(err); })
				.on('close', () => endEmitter.fire())
				.on('end', () => endEmitter.fire())
				.on('connect', res);
		});

		return {
			onDidReceiveMessage: dataEmitter.event,
			onDidClose: closeEmitter.event,
			onDidEnd: endEmitter.event,
			send: d => remoteSocket.write(d),
			end: () => remoteSocket.end(),
		};
	}, connectionToken));
}
```

**Variations / call-sites:**
- Browser variant (`extension.browser.ts:26-77`) implements managed messaging with WebSocket upgrade protocol handling and buffered data queuing.

#### Pattern: Bidirectional Proxy Socket Forwarding
**Where:** `extensions/vscode-test-resolver/src/extension.ts:237-323`
**What:** Traditional proxy server that bidirectionally forwards data between local client socket and remote extension host socket with backpressure handling via pause/resume.

```typescript
return new Promise<vscode.ResolvedAuthority>((res, _rej) => {
	const proxyServer = net.createServer(proxySocket => {
		outputChannel.appendLine(`Proxy connection accepted`);
		let remoteReady = true, localReady = true;
		const remoteSocket = net.createConnection({ port: serverAddr.port });

		proxySocket.on('data', async (data) => {
			await maybeSlowdown();
			remoteReady = remoteSocket.write(data);
			if (!remoteReady) {
				proxySocket.pause();
			}
		});
		remoteSocket.on('data', async (data) => {
			await maybeSlowdown();
			localReady = proxySocket.write(data);
			if (!localReady) {
				remoteSocket.pause();
			}
		});
		proxySocket.on('drain', () => {
			localReady = true;
			if (!isDisconnected) {
				remoteSocket.resume();
			}
		});
		remoteSocket.on('drain', () => {
			remoteReady = true;
			if (!isDisconnected) {
				proxySocket.resume();
			}
		});
	});
	proxyServer.listen(0, '127.0.0.1', () => {
		const port = (<net.AddressInfo>proxyServer.address()).port;
		res(new vscode.ResolvedAuthority('127.0.0.1', port, connectionToken));
	});
});
```

**Variations / call-sites:**
- Tunnel factory at `extension.ts:536-570` uses same pattern with WebSocket upgrade protocol handling in browser variant.

#### Pattern: Connection State Management with EventEmitter
**Where:** `extensions/vscode-test-resolver/src/extension.ts:29-42`
**What:** Pausable and slowdown connection state tracked via EventEmitter for testing reconnection and degraded network scenarios.

```typescript
let connectionPaused = false;
const connectionPausedEvent = new vscode.EventEmitter<boolean>();

let connectionSlowedDown = false;
const connectionSlowedDownEvent = new vscode.EventEmitter<boolean>();
const slowedDownConnections = new Set<Function>();
connectionSlowedDownEvent.event(slowed => {
	if (!slowed) {
		for (const cb of slowedDownConnections) {
			cb();
		}
		slowedDownConnections.clear();
	}
});
```

**Variations / call-sites:**
- Used in proxy handler at `extension.ts:244-270` to pause/resume sockets based on connection state.
- Slowdown applied asynchronously at `extension.ts:272-285` to simulate network latency.

#### Pattern: Server Process Lifecycle Management
**Where:** `extensions/vscode-test-resolver/src/extension.ts:83-207`
**What:** Spawns or downloads remote extension host server, monitors stdout/stderr for readiness signal, handles process errors with user-facing dialogs and error recovery actions.

```typescript
async function doResolve(authority: string, progress: vscode.Progress<{ message?: string; increment?: number }>): Promise<vscode.ResolverResult> {
	if (connectionPaused) {
		throw vscode.RemoteAuthorityResolverError.TemporarilyNotAvailable('Not available right now');
	}
	const connectionToken = String(crypto.randomInt(0xffffffffff));

	const serverPromise = new Promise<vscode.ResolvedAuthority>(async (res, rej) => {
		progress.report({ message: 'Starting Test Resolver' });
		outputChannel = vscode.window.createOutputChannel('TestResolver');

		let isResolved = false;
		async function processError(message: string) {
			outputChannel.appendLine(message);
			if (!isResolved) {
				isResolved = true;
				outputChannel.show();

				const result = await vscode.window.showErrorMessage(message, { modal: true }, ...getActions());
				if (result) {
					await result.execute();
				}
				rej(vscode.RemoteAuthorityResolverError.NotAvailable(message, true));
			}
		}

		let lastProgressLine = '';
		function processOutput(output: string) {
			outputChannel.append(output);
			for (let i = 0; i < output.length; i++) {
				const chr = output.charCodeAt(i);
				if (chr === CharCode.LineFeed) {
					const match = lastProgressLine.match(/Extension host agent listening on (\d+)/);
					if (match) {
						isResolved = true;
						res(new vscode.ResolvedAuthority('127.0.0.1', parseInt(match[1], 10), connectionToken));
					}
					lastProgressLine = '';
				}
			}
		}

		extHostProcess = cp.spawn(serverCommandPath, commandArgs, { env, cwd: vscodePath, shell });
		extHostProcess.stdout!.on('data', (data: Buffer) => processOutput(data.toString()));
		extHostProcess.stderr!.on('data', (data: Buffer) => processOutput(data.toString()));
		extHostProcess.on('error', (error: Error) => {
			processError(`server failed with error:\n${error.message}`);
			extHostProcess = undefined;
		});
		extHostProcess.on('close', (code: number) => {
			processError(`server closed unexpectedly.\nError code: ${code}`);
			extHostProcess = undefined;
		});
	});
}
```

**Variations / call-sites:**
- Dev mode uses local build script at `extension.ts:162-170`.
- Production mode downloads prebuilt server at `extension.ts:171-189` with HTTP redirect following and archive extraction.

#### Pattern: Tunnel Factory with Backpressure Handling
**Where:** `extensions/vscode-test-resolver/src/extension.ts:509-571`
**What:** Factory function that creates tunnel instances with local/remote port mapping, handles privileged port constraints, and implements bidirectional socket piping with drain event handling.

```typescript
async function tunnelFactory(tunnelOptions: vscode.TunnelOptions, tunnelCreationOptions: vscode.TunnelCreationOptions): Promise<vscode.Tunnel> {
	outputChannel.appendLine(`Tunnel factory request: Remote ${tunnelOptions.remoteAddress.port} -> local ${tunnelOptions.localAddressPort}`);
	if (tunnelCreationOptions.elevationRequired) {
		await vscode.window.showInformationMessage('This is a fake elevation message. A real resolver would show a native elevation prompt.', { modal: true }, 'Ok');
	}

	return createTunnelService();

	function newTunnel(localAddress: { host: string; port: number }): vscode.Tunnel {
		const onDidDispose: vscode.EventEmitter<void> = new vscode.EventEmitter();
		let isDisposed = false;
		return {
			localAddress,
			remoteAddress: tunnelOptions.remoteAddress,
			public: !!vscode.workspace.getConfiguration('testresolver').get('supportPublicPorts') && tunnelOptions.public,
			privacy: tunnelOptions.privacy,
			protocol: tunnelOptions.protocol,
			onDidDispose: onDidDispose.event,
			dispose: () => {
				if (!isDisposed) {
					isDisposed = true;
					onDidDispose.fire();
				}
			}
		};
	}

	function createTunnelService(): Promise<vscode.Tunnel> {
		return new Promise<vscode.Tunnel>((res, _rej) => {
			const proxyServer = net.createServer(proxySocket => {
				const remoteSocket = net.createConnection({ host: tunnelOptions.remoteAddress.host, port: tunnelOptions.remoteAddress.port });
				remoteSocket.pipe(proxySocket);
				proxySocket.pipe(remoteSocket);
			});
			// ... port selection and listening logic
		});
	}
}
```

**Variations / call-sites:**
- Port selection logic at `extension.ts:543-561` handles local port conflicts and privileged port constraints per platform.

#### Pattern: Error Handling and Recovery Actions
**Where:** `extensions/vscode-test-resolver/src/extension.ts:443-470`
**What:** User-facing error dialogs with contextual recovery actions (Retry, Close Remote, Ignore) that preserve unsaved work detection.

```typescript
function getActions(): ActionItem[] {
	const actions: ActionItem[] = [];
	const isDirty = vscode.workspace.textDocuments.some(d => d.isDirty) || vscode.workspace.workspaceFile && vscode.workspace.workspaceFile.scheme === 'untitled';

	actions.push({
		title: 'Retry',
		execute: async () => {
			await vscode.commands.executeCommand('workbench.action.reloadWindow');
		}
	});
	if (!isDirty) {
		actions.push({
			title: 'Close Remote',
			execute: async () => {
				await vscode.commands.executeCommand('vscode.newWindow', { reuseWindow: true, remoteAuthority: null });
			}
		});
	}
	actions.push({
		title: 'Ignore',
		isCloseAffordance: true,
		execute: async () => {
			vscode.commands.executeCommand('vscode-testresolver.showLog');
		}
	});
	return actions;
}
```

**Variations / call-sites:**
- Used in error path at `extension.ts:101-105` for user resolution of startup failures.
- Dynamic re-registration scenario at `extension.ts:364-371` shows custom error simulation.

---

## Architectural Insights

**Authority Resolution Flow:**
1. Client invokes `vscode.newWindow({ remoteAuthority: 'test+...' })`
2. VS Code calls registered resolver's `resolve()` method
3. Resolver spawns or downloads remote server, monitors for readiness
4. Returns either `ResolvedAuthority` (with proxy port) or `ManagedResolvedAuthority` (with event-based messaging)
5. All subsequent communication flows through selected transport

**Key Seams Between Local and Remote:**
- **Connection Token**: Cryptographic handshake for authentication (`crypto.randomInt`)
- **Proxy Server**: Traditional TCP forwarding on localhost with backpressure handling
- **Managed Authority**: EventEmitter-based abstraction replacing socket layer entirely
- **Tunnel Factory**: Port forwarding infrastructure with elevation support

**Testing Surface Area:**
- Connection pause/slowdown via command palette commands for reconnection/degraded network scenarios
- Server lifecycle errors (startup delay, intentional failure)
- Multiple authority patterns for testing different transport mechanisms
- HTTP server spawning for forwarded port testing

---

## Cross-File Patterns

**Process Management** (`src/util/processes.ts:13-37`):
- Platform-specific termination: taskkill on Windows, shell script on Unix, SIGKILL fallback
- Used for graceful cleanup in extension disposal

**Server Download** (`src/download.ts:22-116`):
- HTTP redirect following with streaming to file
- Archive extraction (zip/tar.gz) with platform-specific tools
- Caching by commit hash to avoid redundant downloads

**Configuration** (`package.json` lines 154-171):
- Test settings: `startupDelay`, `startupError`, `supportPublicPorts`
- Tunnel feature advertisement in resolver result

---

## Summary

The vscode-test-resolver extension implements a test harness for VS Code's remote development architecture by registering a custom authority resolver. The patterns show:

1. **Remote Authority Registration** defines the contract for resolving logical remote authorities to concrete network addresses and connection factories.
2. **Two Transport Models**: traditional proxy-based forwarding (familiar to SSH/tunnel users) and managed messaging (emerging pattern for WebSocket/HTTP/2 native transports).
3. **Bidirectional Backpressure Handling**: both proxy and managed transports implement proper drain/pause semantics to prevent memory explosion.
4. **Server Lifecycle Management**: spawning/downloading servers, monitoring stdout for readiness signals, providing error recovery UI with contextual actions.
5. **Testing Instrumentation**: connection pause/slowdown for reconnection scenario testing, configurable startup delays/errors, and HTTP server spawning for port forwarding tests.

For porting VS Code to Tauri/Rust, the key challenge is replacing the electron/node.js process spawning and stream handling infrastructure while preserving the authority resolver interface and backpressure semantics. The managed authority pattern shows a cleaner abstraction (event-based vs. socket-based) that could map more naturally to async Rust.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
