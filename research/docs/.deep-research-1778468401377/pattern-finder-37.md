# Remote Authority Resolver Patterns in VS Code Test Resolver Extension

## Overview
The `vscode-test-resolver` extension demonstrates VS Code's remote authority resolution API. It shows how VS Code establishes connections to remote systems (local test in this case) through authority resolver implementations. This is fundamental infrastructure for porting VS Code's IDE functionality to alternative platforms.

---

## Patterns Found

#### Pattern 1: Basic Remote Authority Resolver Registration
**Where:** `extensions/vscode-test-resolver/src/extension.ts:327-344`
**What:** Registration of a remote authority resolver handler with required resolver methods.
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
**Variations:** The browser variant (line 9-18 of extension.browser.ts) uses a minimal resolver with only a `resolve()` method that returns a `ManagedResolvedAuthority`.

---

#### Pattern 2: ResolvedAuthority with Connection Token
**Where:** `extensions/vscode-test-resolver/src/extension.ts:83-127`
**What:** Construction of a `ResolvedAuthority` with host, port, and connection token after server initialization.
```typescript
function doResolve(authority: string, progress: vscode.Progress<{ message?: string; increment?: number }>): Promise<vscode.ResolverResult> {
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
						res(new vscode.ResolvedAuthority('127.0.0.1', parseInt(match[1], 10), connectionToken)); // success!
					}
					lastProgressLine = '';
				} else if (chr === CharCode.Backspace) {
					lastProgressLine = lastProgressLine.substr(0, lastProgressLine.length - 1);
				} else {
					lastProgressLine += output.charAt(i);
				}
			}
		}
```
**Variations / call-sites:** Also at line 316 for proxy-based resolution, and line 365 for error handling test cases.

---

#### Pattern 3: ManagedResolvedAuthority with Socket Proxying
**Where:** `extensions/vscode-test-resolver/src/extension.ts:210-234`
**What:** Managed authority that returns a message-passing implementation via socket connection.
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
**Variations:** Browser version (extension.browser.ts:14-16) uses a more complex WebSocket-based `InitialManagedMessagePassing` handler.

---

#### Pattern 4: TCP Proxy Server for Connection Routing
**Where:** `extensions/vscode-test-resolver/src/extension.ts:237-324`
**What:** A local TCP proxy server that relays data between client and remote sockets.
```typescript
return new Promise<vscode.ResolvedAuthority>((res, _rej) => {
	const proxyServer = net.createServer(proxySocket => {
		outputChannel.appendLine(`Proxy connection accepted`);
		let remoteReady = true, localReady = true;
		const remoteSocket = net.createConnection({ port: serverAddr.port });

		let isDisconnected = false;
		const handleConnectionPause = () => {
			const newIsDisconnected = connectionPaused;
			if (isDisconnected !== newIsDisconnected) {
				outputChannel.appendLine(`Connection state: ${newIsDisconnected ? 'open' : 'paused'}`);
				isDisconnected = newIsDisconnected;
				if (!isDisconnected) {
					outputChannel.appendLine(`Resume remote and proxy sockets.`);
					if (remoteSocket.isPaused() && localReady) {
						remoteSocket.resume();
					}
					if (proxySocket.isPaused() && remoteReady) {
						proxySocket.resume();
					}
				} else {
					outputChannel.appendLine(`Pausing remote and proxy sockets.`);
					if (!remoteSocket.isPaused()) {
						remoteSocket.pause();
					}
					if (!proxySocket.isPaused()) {
						proxySocket.pause();
					}
				}
			}
		};

		connectionPausedEvent.event(_ => handleConnectionPause());
		handleConnectionPause();

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
		proxySocket.on('close', () => {
			outputChannel.appendLine(`Proxy socket closed, closing remote socket.`);
			remoteSocket.end();
		});
		remoteSocket.on('close', () => {
			outputChannel.appendLine(`Remote socket closed, closing proxy socket.`);
			proxySocket.end();
		});
	});
	proxyServer.listen(0, '127.0.0.1', () => {
		const port = (<net.AddressInfo>proxyServer.address()).port;
		outputChannel.appendLine(`Going through proxy at port ${port}`);
		res(new vscode.ResolvedAuthority('127.0.0.1', port, connectionToken));
	});
});
```
**Variations:** Uses backpressure handling with `pause()`/`resume()` and `drain` events for flow control.

---

#### Pattern 5: Error Handling with RemoteAuthorityResolverError
**Where:** `extensions/vscode-test-resolver/src/extension.ts:85,105,139-142,365-370`
**What:** Error handling strategies for resolution failures with modal error dialogs.
```typescript
// Temporary unavailability
if (connectionPaused) {
	throw vscode.RemoteAuthorityResolverError.TemporarilyNotAvailable('Not available right now');
}

// During async resolution failure
rej(vscode.RemoteAuthorityResolverError.NotAvailable(message, true));

// Configuration-driven failure for testing
if (getConfiguration('startupError') === true) {
	processError('Test Resolver failed for testing purposes (configured by "testresolver.startupError").');
	return;
}

// Re-registration with error handling
vscode.workspace.registerRemoteAuthorityResolver('test', {
	async resolve(_authority: string): Promise<vscode.ResolvedAuthority> {
		setTimeout(async () => {
			await vscode.window.showErrorMessage('Just a custom message.', { modal: true, useCustom: true }, 'OK', 'Great');
		}, 2000);
		throw vscode.RemoteAuthorityResolverError.NotAvailable('Intentional Error', true);
	}
});
```

---

#### Pattern 6: Tunnel Factory and Port Forwarding
**Where:** `extensions/vscode-test-resolver/src/extension.ts:509-571`
**What:** Implementation of a `tunnelFactory` for creating TCP tunnels with local/remote address mapping.
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
			let localPort = 0;

			if (tunnelOptions.localAddressPort) {
				localPort = tunnelOptions.localAddressPort;
			} else {
				localPort = tunnelOptions.remoteAddress.port;
			}

			if (localPort === tunnelOptions.remoteAddress.port) {
				localPort += 1;
			}

			if (localPort < 1024 && process.platform !== 'win32') {
				localPort = 0;
			}
			proxyServer.listen(localPort, '127.0.0.1', () => {
				const localPort = (<net.AddressInfo>proxyServer.address()).port;
				outputChannel.appendLine(`New test resolver tunnel service: Remote ${tunnelOptions.remoteAddress.port} -> local ${localPort}`);
				const tunnel = newTunnel({ host: '127.0.0.1', port: localPort });
				tunnel.onDidDispose(() => proxyServer.close());
				res(tunnel);
			});
		});
	}
}
```

---

#### Pattern 7: Server Process Management and Subprocess Spawning
**Where:** `extensions/vscode-test-resolver/src/extension.ts:144-189`
**What:** Launching VS Code server as subprocess with environment setup and process lifecycle management.
```typescript
const { updateUrl, commit, quality, serverDataFolderName, serverApplicationName, dataFolderName } = getProductConfiguration();
const commandArgs = ['--host=127.0.0.1', '--port=0', '--disable-telemetry', '--disable-experiments', '--use-host-proxy', '--accept-server-license-terms'];
const env = getNewEnv();
const remoteDataDir = process.env['TESTRESOLVER_DATA_FOLDER'] || path.join(os.homedir(), `${serverDataFolderName || dataFolderName}-testresolver`);

if (!commit) { // dev mode
	const serverCommand = process.platform === 'win32' ? 'code-server.bat' : 'code-server.sh';
	const vscodePath = path.resolve(path.join(context.extensionPath, '..', '..'));
	const serverCommandPath = path.join(vscodePath, 'scripts', serverCommand);

	outputChannel.appendLine(`Launching server: "${serverCommandPath}" ${commandArgs.join(' ')}`);
	const shell = (process.platform === 'win32');
	env['VSCODE_SKIP_PRELAUNCH'] = '1';
	extHostProcess = cp.spawn(serverCommandPath, commandArgs, { env, cwd: vscodePath, shell });
} else {
	const serverCommand = `${serverApplicationName}${process.platform === 'win32' ? '.cmd' : ''}`;
	let serverLocation = env['VSCODE_REMOTE_SERVER_PATH'];
	if (!serverLocation) {
		const serverBin = path.join(remoteDataDir, 'bin');
		progress.report({ message: 'Installing VSCode Server' });
		serverLocation = await downloadAndUnzipVSCodeServer(updateUrl, commit, quality, serverBin, m => outputChannel.appendLine(m));
	}

	extHostProcess = cp.spawn(path.join(serverLocation, 'bin', serverCommand), commandArgs, { env, cwd: serverLocation, shell });
}
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
context.subscriptions.push({
	dispose: () => {
		if (extHostProcess) {
			terminateProcess(extHostProcess, context.extensionPath);
		}
	}
});
```
**Variations:** Platform-specific command paths and shell configuration; supports both development and production server binaries.

---

## Summary

The test resolver extension demonstrates VS Code's remote authority resolution architecture, which is central to how VS Code separates the client UI from remote computational environments. Key patterns include:

1. **Authority resolver registration** with protocol-specific handlers
2. **Connection token-based authentication** for secure remote linkage
3. **Bidirectional socket proxying** with backpressure handling for data relay
4. **Managed vs. traditional resolution modes** (socket-based vs. WebSocket-based)
5. **Subprocess lifecycle management** for launching remote servers
6. **Port tunneling and forwarding** infrastructure
7. **Error handling and recovery** with user-facing notifications

For a Tauri/Rust port, these patterns would translate to:
- Implementing a Rust-based remote authority resolver API/plugin system
- TCP/WebSocket proxying in Rust (using tokio, tungstenite)
- Subprocess management via std::process or similar Rust crate
- Port forwarding/tunneling abstraction layer
- Error propagation and UI feedback mechanisms

The extension code reveals that VS Code's remote system is fundamentally a **socket-based relay protocol** with configurable authority resolution and port forwarding capabilities.

