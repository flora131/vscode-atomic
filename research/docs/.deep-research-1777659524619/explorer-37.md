# Partition 37 of 79 — Findings

## Scope
`extensions/vscode-test-resolver/` (7 files, 925 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator Report: Partition 37 — `extensions/vscode-test-resolver/`

## Overview
The `vscode-test-resolver` extension demonstrates VS Code's **remote-authority resolver architecture**—a critical abstraction for separating the local UI client from a remote extension host server. This is fundamental to understanding what a Tauri/Rust port must replicate for remote development, SSH, WSL, and container support.

**Total scope:** 16 files, 878 LOC (TypeScript source)

---

## Implementation

- `extensions/vscode-test-resolver/src/extension.ts` — Core remote-authority resolver registration and lifecycle management; demonstrates `vscode.workspace.registerRemoteAuthorityResolver('test', { resolve, getCanonicalURI, tunnelFactory, showCandidatePort })` pattern; spawns local VS Code Server processes; manages connection tokens, socket proxying, and error handling.

- `extensions/vscode-test-resolver/src/extension.browser.ts` — Browser-variant remote resolver using `ManagedResolvedAuthority` with WebSocket message-passing; HTTP header parsing and WebSocket upgrade negotiation; demonstrates cross-platform connection strategy (client ↔ managed proxy ↔ server).

- `extensions/vscode-test-resolver/src/download.ts` — Server binary distribution and installation; downloads platform-specific VS Code Server archives from update endpoints (win32-x64, darwin, linux-x64); manages decompression and cache.

- `extensions/vscode-test-resolver/src/util/processes.ts` — Process lifecycle utilities; platform-specific termination (taskkill on Windows, shell script on Unix); used to clean up spawned extension host processes.

---

## Tests
None present in scope.

---

## Types / Interfaces

- `extensions/vscode-test-resolver/src/extension.ts` (line 473–480) — `IProductConfiguration` interface defining product metadata (updateUrl, commit, quality, dataFolderName, serverApplicationName, serverDataFolderName) required to bootstrap and configure the remote server.

- TypeScript type definitions imported from `../../src/vscode-dts/vscode.proposed.resolvers.d.ts` and `vscode.proposed.tunnels.d.ts` (see tsconfig.json includes).

---

## Configuration

- `extensions/vscode-test-resolver/package.json` — Extension manifest with:
  - **Activation events:** `onResolveRemoteAuthority:test`, command-based activation for resolver control, tunnel, and remote server operations.
  - **API proposals:** `resolvers`, `tunnels` (unstable VS Code APIs required for remote authority and tunnel management).
  - **Contributes:** resource label formatters for `vscode-remote://test+*` scheme; commands for window management, connection control, port tunneling, and logging.
  - **Capabilities:** untrusted workspace support, virtual workspace support.
  - **Configuration schema:** `testresolver.startupDelay`, `testresolver.startupError`, `testresolver.supportPublicPorts` (for testing resolver behavior and tunnel features).

- `extensions/vscode-test-resolver/tsconfig.json` — Extends base config; includes vscode.d.ts and proposed APIs for resolvers and tunnels.

- `extensions/vscode-test-resolver/.vscode/launch.json` — Debug configuration using `extensionHost` debugger type.

- `extensions/vscode-test-resolver/esbuild.mts` — Build configuration using esbuild; compiles node-platform extension from `src/extension.ts`.

- `extensions/vscode-test-resolver/esbuild.browser.mts` — Build configuration for browser variant (browser entrypoint: `testResolverMain`).

- `extensions/vscode-test-resolver/.vscodeignore`, `.npmrc`, `.gitignore` — Standard extension packaging and SCM configuration.

---

## Examples / Fixtures

- `extensions/vscode-test-resolver/media/icon.png` — Extension icon.

- `extensions/vscode-test-resolver/scripts/terminateProcess.sh` — Shell script for process termination on Unix platforms.

---

## Documentation

None explicitly present (no README or .md files); documentation inferred from code comments and VS Code API usage patterns.

---

## Notable Clusters

### `extensions/vscode-test-resolver/src/` — 3 TypeScript files (586 LOC)
Core extension activation and remote resolution logic. Central entry points:
- `extension.ts`: Main server bootstrap, resolver implementation, tunnel/port-forwarding factory, error handling workflows.
- `extension.browser.ts`: Browser-based resolver using WebSocket and managed message passing (no subprocess spawning).
- `download.ts`: Server artifact acquisition.

### `extensions/vscode-test-resolver/src/util/` — 1 TypeScript file (38 LOC)
Process management utilities; cross-platform process termination.

---

## Relevance to Tauri/Rust Port

This extension is a **reference implementation** of VS Code's remote-authority resolver protocol. A Tauri/Rust port must:

1. **Replicate the resolver lifecycle:** Extension activation on `onResolveRemoteAuthority`, registration of resolver handlers, and async resolution returning `ResolverResult` (address, port, connection token).

2. **Implement socket proxying and message passing:** The extension demonstrates both raw socket proxying (lines 238–323) and managed message-passing (WebSocket-based, in browser variant). A Rust backend must support equivalent connection strategies.

3. **Handle process spawning and server bootstrap:** The extension downloads and spawns a VS Code Server binary; a Rust port must either (a) link Rust code directly, or (b) replicate this spawn-and-communicate pattern for backward compatibility.

4. **Support tunnels and port forwarding:** `tunnelFactory` callback (lines 509–571) and `showCandidatePort` callback (line 505) show how VS Code's tunnel infrastructure is integrated. A Rust backend must implement equivalent port management APIs.

5. **Manage connection state and error recovery:** Connection pause/resume (lines 29–42), slow-down simulation (lines 32–42), and error handling (lines 95–107, 365–371) are production concerns.

6. **Support both managed and raw proxy modes:** The extension demonstrates two resolver patterns: `ResolvedAuthority` (raw proxy) and `ManagedResolvedAuthority` (event-based message passing). The Rust port must support both.

7. **Handle platform-specific process management:** Windows (taskkill), macOS/Linux (custom shell script) process termination patterns must be replicated for server lifecycle management.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder Report: Remote Authority Resolver Patterns
## Partition 37 — `extensions/vscode-test-resolver/`

This report documents concrete code patterns for remote authority resolution as implemented in VS Code's test resolver extension. These patterns are relevant to understanding how VS Code's core IDE bridges local clients with remote servers—a critical architecture for porting to Tauri/Rust.

---

#### Pattern: Remote Authority Resolver Registration (Node.js)
**Where:** `extensions/vscode-test-resolver/src/extension.ts:327-344`
**What:** Core registration of a remote authority resolver with canonical URI, resolution, and tunnel factory methods.

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
context.subscriptions.push(authorityResolverDisposable);
```

**Variations / call-sites:**
- Re-registration pattern at `extension.ts:364` for error handling scenarios

---

#### Pattern: Managed Authority Resolution (Browser/WebSocket)
**Where:** `extensions/vscode-test-resolver/src/extension.browser.ts:9-18`
**What:** Browser-based resolver using WebSocket-based ManagedResolvedAuthority for remote communication.

```typescript
vscode.workspace.registerRemoteAuthorityResolver('test', {
	async resolve(_authority: string): Promise<vscode.ResolverResult> {
		console.log(`Resolving ${_authority}`);
		console.log(`Activating vscode.github-authentication to simulate auth`);
		await vscode.extensions.getExtension('vscode.github-authentication')?.activate();
		return new vscode.ManagedResolvedAuthority(async () => {
			return new InitialManagedMessagePassing();
		});
	}
});
```

**Variations / call-sites:**
- Socket-based variant at `extension.ts:212-234`

---

#### Pattern: Resolver Result Construction with Proxy Server
**Where:** `extensions/vscode-test-resolver/src/extension.ts:237-323`
**What:** Creates a proxy TCP server to bridge local and remote sockets, returning ResolvedAuthority with connection token.

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
		// ... drain and close handlers
	});
	proxyServer.listen(0, '127.0.0.1', () => {
		const port = (<net.AddressInfo>proxyServer.address()).port;
		res(new vscode.ResolvedAuthority('127.0.0.1', port, connectionToken));
	});
});
```

---

#### Pattern: RemoteAuthorityResolverError Handling
**Where:** `extensions/vscode-test-resolver/src/extension.ts:85, 105, 369`
**What:** Error type system for resolver failures with user-facing messaging.

```typescript
// Temporarily unavailable (pause scenario)
throw vscode.RemoteAuthorityResolverError.TemporarilyNotAvailable('Not available right now');

// Process failure (caught in promise)
rej(vscode.RemoteAuthorityResolverError.NotAvailable(message, true));

// Intentional error with custom UI
throw vscode.RemoteAuthorityResolverError.NotAvailable('Intentional Error', true);
```

---

#### Pattern: Managed Message Passing with EventEmitter
**Where:** `extensions/vscode-test-resolver/src/extension.browser.ts:26-77`
**What:** Implements vscode.ManagedMessagePassing interface using EventEmitter for data, close, and end events.

```typescript
class InitialManagedMessagePassing implements vscode.ManagedMessagePassing {
	private readonly dataEmitter = new vscode.EventEmitter<Uint8Array<ArrayBuffer>>();
	private readonly closeEmitter = new vscode.EventEmitter<Error | undefined>();
	private readonly endEmitter = new vscode.EventEmitter<void>();

	public readonly onDidReceiveMessage = this.dataEmitter.event;
	public readonly onDidClose = this.closeEmitter.event;
	public readonly onDidEnd = this.endEmitter.event;

	public send(d: Uint8Array): void {
		if (this._actual) {
			this._actual.send(d as Uint8Array<ArrayBuffer>);
			return;
		}
		// parse HTTP headers from d and initialize actual WebSocket
	}
	public end(): void { }
}
```

**Variations / call-sites:**
- Socket-based EventEmitter pattern at `extension.ts:214-216` for ManagedResolvedAuthority

---

#### Pattern: Tunnel Factory Implementation
**Where:** `extensions/vscode-test-resolver/src/extension.ts:509-571`
**What:** Factory function that creates TCP proxy tunnels with local/remote port mapping and elevation handling.

```typescript
async function tunnelFactory(tunnelOptions: vscode.TunnelOptions, tunnelCreationOptions: vscode.TunnelCreationOptions): Promise<vscode.Tunnel> {
	outputChannel.appendLine(`Tunnel factory request: Remote ${tunnelOptions.remoteAddress.port} -> local ${tunnelOptions.localAddressPort}`);
	if (tunnelCreationOptions.elevationRequired) {
		await vscode.window.showInformationMessage('This is a fake elevation message...', { modal: true }, 'Ok');
	}
	return createTunnelService();
}
```

**Variations / call-sites:**
- Referenced at `extension.ts:342` in resolver registration

---

#### Pattern: Server Download and Process Management
**Where:** `extensions/vscode-test-resolver/src/download.ts:22-115`
**What:** HTTPS download, archive extraction, and server binary spawning workflow.

```typescript
export async function downloadAndUnzipVSCodeServer(
	updateUrl: string, 
	commit: string, 
	quality: string = 'stable', 
	destDir: string, 
	log: (messsage: string) => void
): Promise<string> {
	const extractDir = path.join(destDir, commit);
	if (fs.existsSync(extractDir)) {
		log(`Found ${extractDir}. Skipping download.`);
	} else {
		const vscodeArchivePath = await downloadVSCodeServerArchive(updateUrl, commit, quality, destDir, log);
		unzipVSCodeServer(vscodeArchivePath, extractDir, destDir, log);
		fs.unlinkSync(vscodeArchivePath);
	}
	return Promise.resolve(extractDir);
}
```

---

#### Pattern: Cross-Platform Process Termination
**Where:** `extensions/vscode-test-resolver/src/util/processes.ts:13-37`
**What:** Platform-specific process tree termination using taskkill (Windows), shell scripts (Unix), or SIGKILL.

```typescript
export function terminateProcess(p: cp.ChildProcess, extensionPath: string): TerminateResponse {
	if (process.platform === 'win32') {
		try {
			const options: any = { stdio: ['pipe', 'pipe', 'ignore'] };
			cp.execFileSync('taskkill', ['/T', '/F', '/PID', p.pid!.toString()], options);
		} catch (err) {
			return { success: false, error: err };
		}
	} else if (process.platform === 'darwin' || process.platform === 'linux') {
		try {
			const cmd = path.join(extensionPath, 'scripts', 'terminateProcess.sh');
			const result = cp.spawnSync(cmd, [p.pid!.toString()]);
			if (result.error) {
				return { success: false, error: result.error };
			}
		} catch (err) {
			return { success: false, error: err };
		}
	} else {
		p.kill('SIGKILL');
	}
	return { success: true };
}
```

---

## Architecture Summary

The test resolver demonstrates VS Code's remote authority architecture at three levels:

1. **Registration Layer** (`registerRemoteAuthorityResolver`): Hooks into the extension API with schema 'test', specifying canonical URI handling, resolution logic, tunnel creation, and candidate port filtering.

2. **Resolution Layer** (`doResolve`, `resolve`): Spawns local or downloaded server processes, streams their output to monitor lifecycle, and returns either a standard ResolvedAuthority (with proxy) or ManagedResolvedAuthority (with custom message passing).

3. **Transport Layer**: Uses either TCP sockets with a proxy bridge (for Electron) or WebSockets through HTTP upgrade (for browser). Both use EventEmitter for event-driven data flow.

The architecture requires handling:
- Process lifecycle (spawn, monitor output, terminate gracefully across platforms)
- Authentication (implicit in test, but extensible via extension activation)
- Error states with user-facing UI (TemporarilyNotAvailable, NotAvailable)
- Tunnel factory for port forwarding with elevation prompts
- Progress reporting during resolution

For a Tauri/Rust port, the key abstraction points are the resolver interface contract, the ManagedMessagePassing protocol (message-passing over async channels), and the tunnel/port forwarding subsystem—all of which would need Rust equivalents with async/await support.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
