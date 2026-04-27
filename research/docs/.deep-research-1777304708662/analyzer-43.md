## Analysis: `extensions/tunnel-forwarding/` — VS Code Port Forwarding via Native CLI

### Files Analysed

1. `extensions/tunnel-forwarding/src/extension.ts` (343 LOC) — main extension entry point and entire runtime logic
2. `extensions/tunnel-forwarding/src/deferredPromise.ts` (63 LOC) — deferred promise utility
3. `extensions/tunnel-forwarding/src/split.ts` (52 LOC) — newline-delimited stream splitter
4. `extensions/tunnel-forwarding/package.json` — manifest, capabilities, API proposals
5. `extensions/tunnel-forwarding/esbuild.mts` — build configuration (single entry point → `dist/extension`)
6. `src/vscode-dts/vscode.proposed.tunnelFactory.d.ts` — proposed API: `workspace.registerTunnelProvider`
7. `src/vscode-dts/vscode.proposed.tunnels.d.ts` — proposed API: `TunnelOptions`, `Tunnel`, `TunnelDescription`
8. `src/vs/workbench/api/common/extHostTunnelService.ts` — extension host implementation of tunnel RPC
9. `src/vs/workbench/api/browser/mainThreadTunnelService.ts` (partial) — main thread counterpart, receives tunnel registrations
10. `src/vs/platform/tunnel/common/tunnel.ts` (partial) — core tunnel model types

---

### Per-File Notes

#### 1. `extensions/tunnel-forwarding/src/extension.ts`

**Role:** Complete implementation of the tunnel-forwarding extension. Registers a `TunnelProvider` with VS Code, manages the lifecycle of a spawned native CLI binary (`code-tunnel` / `code-tunnel-insiders`), and coordinates port forwarding state.

**CLI path resolution (`extension.ts:24-42`):**
- `cliPath` is computed at module load time (before `activate`), not at runtime.
- Dev mode override: `VSCODE_FORWARDING_IS_DEV` env var points to `cli/target/debug/code` (`extension.ts:27`).
- Production path branches by platform:
  - `darwin`: `<appRoot>/bin/<cliName>` (`extension.ts:30-31`)
  - `win32` with version folder in appRoot: `<appRoot>/../../../bin/<cliName>.exe` (`extension.ts:32-34`)
  - All others: `<appRoot>/../../bin/<cliName>` (`extension.ts:35-36`)
- Binary name: `code-tunnel` for stable quality, `code-tunnel-insiders` otherwise (`extension.ts:38`).

**`activate` function (`extension.ts:77-107`):**
- Returns early if `vscode.env.remoteAuthority` is set (local-only constraint, `extension.ts:78-79`).
- Instantiates `Logger` and `TunnelProvider`.
- Registers two commands: `tunnel-forwarding.showLog` and `tunnel-forwarding.restart` (`extension.ts:86-87`).
- Listens to `provider.onDidStateChange` to set context key `tunnelForwardingIsRunning` (`extension.ts:89-91`).
- Calls `vscode.workspace.registerTunnelProvider(provider, { tunnelFeatures: { elevation: false, protocol: true, privacyOptions: [...] } })` (`extension.ts:93-105`). Privacy options are `TunnelPrivacyId.Public` and `TunnelPrivacyId.Private`.

**`Tunnel` class (`extension.ts:44-62`):**
- Implements `vscode.Tunnel`. Holds `remoteAddress`, `privacy`, `protocol`.
- `localAddress` starts unset; populated via `setPortFormat(formatString)` (`extension.ts:55-57`), which substitutes `{port}` with the remote port number.
- `dispose()` fires `disposeEmitter` (`extension.ts:59-61`).

**State machine (`extension.ts:64-75`):**
- Four states: `Inactive`, `Starting`, `Active`, `Error`.
- `Starting` and `Active` carry a reference to the live `ChildProcessWithoutNullStreams` and an optional `cleanupTimeout` handle.
- `Active` additionally holds `portFormat: string`, the URL template received from the CLI.
- `Error` carries an error message string.

**`TunnelProvider` class (`extension.ts:139-343`):**
- `tunnels: Set<Tunnel>` tracks all active tunnel instances (`extension.ts:140`).
- `state` setter fires `stateChange` EventEmitter on every transition (`extension.ts:148-151`).

**`provideTunnel` (`extension.ts:158-201`):**
1. If privacy is `Public`, calls `consentPublicPort(port)` (`extension.ts:159-163`); returns `undefined` if user declines.
2. Instantiates a new `Tunnel`, registers it in `this.tunnels`, attaches dispose listener that calls `updateActivePortsIfRunning()` (`extension.ts:165-176`).
3. State dispatch:
   - `Inactive` or `Error`: calls `setupPortForwardingProcess()`, falls through to `Starting` case.
   - `Starting`: calls `updateActivePortsIfRunning()`, returns a promise that resolves when state becomes `Active` (setting `portFormat` on the tunnel) or rejects on `Error` (`extension.ts:183-195`).
   - `Active`: immediately sets `portFormat`, calls `updateActivePortsIfRunning()`, returns tunnel (`extension.ts:196-199`).

**`updateActivePortsIfRunning` (`extension.ts:250-264`):**
- Only operates in `Starting` or `Active` states.
- Collects all active tunnels' `{ number, privacy, protocol }` and writes a JSON array followed by `\n` to the CLI's `stdin` (`extension.ts:255-256`). This is the port-sync IPC channel.
- If port list is empty and no cleanup timer exists, sets a `setTimeout(killRunningProcess, CLEANUP_TIMEOUT)` (10 seconds, `extension.ts:258-260`).
- If port list is non-empty and a timer exists, clears it (`extension.ts:261-263`).

**`setupPortForwardingProcess` (`extension.ts:266-342`):**
1. Calls `vscode.authentication.getSession('github', ['user:email', 'read:org'], { createIfNone: true })` to obtain a GitHub access token (`extension.ts:267-269`).
2. Spawns: `spawn(cliPath, ['--verbose', 'tunnel', 'forward-internal', '--provider', 'github'], { stdio: 'pipe', env: { ...process.env, NO_COLOR: '1', VSCODE_CLI_ACCESS_TOKEN: session.accessToken } })` (`extension.ts:271-280`).
3. Transitions to `State.Starting` with the child process reference (`extension.ts:281`).
4. Creates a `DeferredPromise<void>` (`progressP`) and shows a progress notification that resolves when `progressP.p` resolves (`extension.ts:283-294`).
5. `child.stdout` is piped through `splitNewLines()` and each line is logged at `info` level (`extension.ts:314-317`).
6. `child.stderr` is piped through `splitNewLines()` and each line is JSON-parsed (`extension.ts:319-336`):
   - Expects `{ port_format: string }`. When `port_format` changes, transitions to `State.Active` with `portFormat` and calls `progressP.complete()` (`extension.ts:323-329`).
   - Non-JSON lines are logged at `error` level (`extension.ts:332-334`).
7. On `exit` or `error` events, `progressP.complete()` is called and state transitions to `State.Error` if the process is still the current one (`extension.ts:297-312`).
8. Awaits the `spawn` event before returning (`extension.ts:338-341`).

**`consentPublicPort` (`extension.ts:210-233`):**
- Checks `context.globalState.get('didWarnPublic', false)` (`extension.ts:211`).
- If already warned, returns `true` immediately.
- Otherwise shows a modal warning with options "Continue" and "Don't show again"; stores `true` under `didWarnPublicKey` if user picks "Don't show again" (`extension.ts:227`). Returns `false` if dismissed.

**`Logger` class (`extension.ts:111-135`):**
- Lazily creates a `vscode.LogOutputChannel` on first `log()` call.
- When created, sets context key `tunnelForwardingHasLog = true` to enable the "Show Log" command (`extension.ts:131`).
- Delegates to the channel's level methods (`trace`, `debug`, `info`, `warn`, `error`).

---

#### 2. `extensions/tunnel-forwarding/src/deferredPromise.ts`

**Role:** Utility that externalizes the resolve/reject callbacks of a `Promise<T>` so they can be called from outside the constructor.

- Constructor creates `this.p: Promise<T>` and captures callbacks as `completeCallback` and `errorCallback` (`deferredPromise.ts:41-44`).
- `complete(value: T)` invokes `completeCallback`, records `{ outcome: Resolved, value }`, returns a `Promise<void>` that resolves synchronously after the state is set (`deferredPromise.ts:47-52`).
- `error(err)` mirrors this for rejection (`deferredPromise.ts:55-60`).
- Read-only properties `isRejected`, `isResolved`, `isSettled`, `value` inspect `this.outcome` (`deferredPromise.ts:23-36`).
- Used in `extension.ts:283` to control the progress notification lifetime.

---

#### 3. `extensions/tunnel-forwarding/src/split.ts`

**Role:** Node.js `Transform` stream that splits a byte stream on a single delimiter byte (newline by default) and emits each segment as a separate chunk, stripping the delimiter.

- `splitNewLines()` factory at `split.ts:8` instantiates `StreamSplitter('\n'.charCodeAt(0))` (byte value 10).
- `_transform` (`split.ts:22-42`): accumulates incoming `Buffer` chunks into `this.buffer`. Scans for the delimiter byte, calling `this.push(subarray)` for each complete segment. Leaves remainder in `this.buffer`.
- `_flush` (`split.ts:44-50`): pushes any remaining buffered bytes when the stream ends.
- Used twice in `extension.ts:314-335` to convert the CLI's stdout/stderr byte streams into line-by-line string events.

---

#### 4. `extensions/tunnel-forwarding/package.json`

**Role:** Extension manifest.

- `"activationEvents": ["onTunnel"]` (`package.json:23`) — activates only when the tunnel system is invoked.
- `"enabledApiProposals": ["resolvers", "tunnelFactory"]` (`package.json:18-21`) — gates access to the two proposed API namespaces used by the extension.
- Two contributed commands: `tunnel-forwarding.showLog` (gated on `tunnelForwardingHasLog`) and `tunnel-forwarding.restart` (gated on `tunnelForwardingIsRunning`) (`package.json:26-39`).
- Main entry: `./out/extension` (`package.json:42`), built via esbuild.

---

#### 5. `extensions/tunnel-forwarding/esbuild.mts`

**Role:** Build script. Delegates to `esbuild-extension-common.mts` with `platform: 'node'` and a single entry point `src/extension.ts` → `dist/extension`.

---

#### 6. `src/vscode-dts/vscode.proposed.tunnelFactory.d.ts`

**Role:** Declares the proposed `vscode.workspace.registerTunnelProvider(provider, information)` API surface used at `extension.ts:93`.

- `TunnelProvider.provideTunnel(options, creationOptions, token)` is the callback the extension must implement (`tunnelFactory.d.ts:37`).
- `TunnelInformation.tunnelFeatures` carries `elevation`, `privacyOptions`, and optional `protocol` flag (`tunnelFactory.d.ts:19-29`).

---

#### 7. `src/vscode-dts/vscode.proposed.tunnels.d.ts`

**Role:** Declares `TunnelOptions`, `TunnelDescription`, and `Tunnel` interfaces consumed by the extension and tunnel service.

- `TunnelOptions.remoteAddress`, `.privacy`, `.protocol` are the fields read in `provideTunnel` at `extension.ts:159-168`.
- `Tunnel` extends `TunnelDescription` and adds `onDidDispose` and `dispose()` (`tunnels.d.ts:36-40`).

---

#### 8. `src/vs/workbench/api/common/extHostTunnelService.ts`

**Role:** Extension host RPC service that bridges the extension's `vscode.workspace.registerTunnelProvider` call to the main thread.

- `registerTunnelProvider` (`extHostTunnelService.ts:151-171`) stores the provider callback as `_forwardPortProvider`, then calls `_proxy.$setTunnelProvider(tunnelFeatures, true)` to notify the main thread.
- `$forwardPort` (`extHostTunnelService.ts:253-286`) is the main-thread-callable RPC method: invokes `_forwardPortProvider`, awaits the tunnel, stores it in `_extensionTunnels`, and returns a `TunnelDto` via `TunnelDtoConverter.fromApiTunnel` (`extHostTunnelService.ts:274`).
- Tunnel dispose listener at `extHostTunnelService.ts:269-272` calls back to `_proxy.$closeTunnel` when the extension fires `onDidDispose`.
- Only one provider can be registered at a time; a second call throws (`extHostTunnelService.ts:152-154`).

---

#### 9. `src/vs/workbench/api/browser/mainThreadTunnelService.ts` (partial)

**Role:** Main thread RPC counterpart. Receives tunnel provider registrations from the extension host and routes forwarding requests from VS Code's tunnel model to the extension.

- Decorated `@extHostNamedCustomer(MainContext.MainThreadTunnelService)` (`mainThreadTunnelService.ts:24`).
- Constructor subscribes to `tunnelService.onTunnelOpened` and `onTunnelClosed` to fire `$onDidTunnelsChange` on the ext host (`mainThreadTunnelService.ts:42-43`).
- `$setTunnelProvider` (not shown in the read excerpt) ultimately registers an `ITunnelProvider` with `ITunnelService`, which delegates port-open calls back to `$forwardPort` on the ext host.

---

#### 10. `src/vs/platform/tunnel/common/tunnel.ts` (partial)

**Role:** Defines shared types (`RemoteTunnel`, `TunnelOptions`, `TunnelPrivacyId`, `TunnelCreationOptions`, `TunnelProviderFeatures`) used across extension host and main thread.

- `RemoteTunnel.localAddress` (`tunnel.ts:24`) is the string the Ports view displays; the extension populates it via `portFormat` template substitution.
- `TunnelPrivacyId` enum at `tunnel.ts:49-53` includes `ConstantPrivate`, `Private`, `Public`.

---

### Cross-Cutting Synthesis

The tunnel-forwarding extension implements VS Code local port forwarding by acting as a thin orchestration shell over a native Rust binary (`code-tunnel`). The full call chain is:

1. VS Code's Ports view or user action triggers `workspace.registerTunnelProvider` (proposed API) → routes through ext host RPC to `ExtHostTunnelService.registerTunnelProvider` (`extHostTunnelService.ts:151`) → `MainThreadTunnelService.$setTunnelProvider`.
2. When a port-forward request arrives, `MainThreadTunnelService` calls `$forwardPort` on the ext host, which invokes `TunnelProvider.provideTunnel` (`extension.ts:158`).
3. `provideTunnel` either waits for the CLI to start or immediately returns if already `Active`. In both cases it eventually resolves with a `Tunnel` whose `localAddress` is set by template (`{port}` substitution from `port_format`).
4. The CLI is spawned once (`setupPortForwardingProcess`, `extension.ts:266`) with GitHub auth token injected via environment variable. The access token is fetched from `vscode.authentication.getSession('github', ...)` immediately before spawn.
5. After spawn, two IPC channels operate:
   - **stdin → CLI**: JSON array of `{ number, privacy, protocol }` objects sent on every tunnel set change (`updateActivePortsIfRunning`, `extension.ts:250-264`).
   - **CLI stderr → extension**: newline-delimited JSON objects; the extension looks for `{ port_format }` to know the tunnel URL template and transition to `Active` state.
   - **CLI stdout → extension**: plain text log lines, forwarded to the `LogOutputChannel`.
6. `StreamSplitter` (`split.ts`) is the byte-level framing layer that converts raw Node.js stream chunks into per-line `data` events on both stdout and stderr.
7. The 10-second `CLEANUP_TIMEOUT` (`extension.ts:22`) prevents the CLI from being killed during privacy toggle operations that manifest as a dispose+re-create sequence.

**For a Tauri/Rust port:** The native `code-tunnel` binary already exists as a Rust binary. The extension itself is purely a TypeScript orchestration layer: spawn lifecycle, stdin JSON writes, stderr JSON reads, auth token acquisition, and UI interactions (progress notifications, warning dialogs, output channel). All of these would need to be re-expressed in Tauri's plugin/command system. The VS Code proposed API surface (`registerTunnelProvider`, the `Tunnel` interface, privacy options, context keys) has no direct Tauri equivalent and would need a replacement ports-view mechanism. The `DeferredPromise` and `StreamSplitter` utilities are straightforward to replicate. The hardest boundary to cross is the bidirectional RPC chain between the extension host and main thread (`ExtHostTunnelService` ↔ `MainThreadTunnelService`), which is deeply coupled to VS Code's extension host IPC architecture.

---

### Out-of-Partition References

The following files are outside the `extensions/tunnel-forwarding/` partition but are directly invoked or depended upon by its implementation:

- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.tunnelFactory.d.ts` — declares `workspace.registerTunnelProvider` and `TunnelProvider` interface
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.tunnels.d.ts` — declares `TunnelOptions`, `Tunnel`, `TunnelDescription`
- `/Users/norinlavaee/vscode-atomic/src/vscode-dts/vscode.proposed.resolvers.d.ts` — second API proposal listed in `enabledApiProposals`
- `/Users/norinlavaee/vscode-atomic/src/vs/workbench/api/common/extHostTunnelService.ts` — extension host RPC implementation; `registerTunnelProvider` at line 151, `$forwardPort` at line 253
- `/Users/norinlavaee/vscode-atomic/src/vs/workbench/api/browser/mainThreadTunnelService.ts` — main thread counterpart; `$setTunnelProvider` flow, tunnel lifecycle events
- `/Users/norinlavaee/vscode-atomic/src/vs/platform/tunnel/common/tunnel.ts` — shared tunnel types (`RemoteTunnel`, `TunnelPrivacyId`, `TunnelProviderFeatures`)
- `/Users/norinlavaee/vscode-atomic/src/vs/workbench/api/common/extHost.protocol.ts` — RPC message type definitions (`MainThreadTunnelServiceShape`, `ExtHostTunnelServiceShape`, `TunnelDto`)
- `/Users/norinlavaee/vscode-atomic/src/vs/workbench/contrib/remote/browser/tunnelFactory.ts` — workbench-side tunnel factory integration
- `/Users/norinlavaee/vscode-atomic/src/vs/workbench/api/node/extHostTunnelService.ts` — Node.js-specific override of `ExtHostTunnelService` (candidate finder, managed tunnel factory)
- `/Users/norinlavaee/vscode-atomic/cli/` — the `code-tunnel` Rust binary that is spawned; IPC protocol (stdin JSON port lists, stderr JSON `port_format` messages) is implicitly defined there
