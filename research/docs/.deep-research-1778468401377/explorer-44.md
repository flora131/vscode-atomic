# Partition 44 of 80 — Findings

## Scope
`extensions/tunnel-forwarding/` (4 files, 474 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locator: tunnel-forwarding Extension

**Scope:** `extensions/tunnel-forwarding/` (4 implementation files, 456 LOC in TypeScript)

## Implementation Files

- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/src/extension.ts` (343 LOC)
  - Main extension entry point implementing `vscode.TunnelProvider` interface
  - Spawns CLI tunnel forwarding process (Rust-based `code-tunnel` binary)
  - Manages tunnel lifecycle: creation, privacy options (public/private), protocol selection (HTTP/HTTPS)
  - Implements state machine (Inactive → Starting → Active, with Error states)
  - Communicates with CLI via child process stdin/stdout/stderr
  - Registers `tunnel-forwarding.showLog` and `tunnel-forwarding.restart` commands
  - Handles GitHub authentication via `vscode.authentication.getSession()`
  - Uses deferred promises for async state transitions

- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/src/deferredPromise.ts` (62 LOC)
  - Generic `DeferredPromise<T>` class for manual promise resolution/rejection
  - Used during port forwarding system startup to wait for CLI readiness
  - Copied from `src/vs/base/common/async.ts`

- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/src/split.ts` (51 LOC)
  - `StreamSplitter` Transform stream class for newline-delimited output parsing
  - Parses tunnel provider CLI JSON output line-by-line
  - Copied from `src/vs/base/node/nodeStreams.ts`

## Configuration Files

- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/package.json`
  - Extension metadata: v10.0.0, MIT licensed, VS Code 1.82.0+ required
  - API proposals: `resolvers`, `tunnelFactory` (experimental tunnel provider API)
  - Activation: `onTunnel` event
  - Contributed commands for log viewing and restart
  - Build via gulp compilation pipeline
  - Published by vscode

- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/tsconfig.json`
  - Extends `../tsconfig.base.json`
  - Includes vscode type definitions for tunnel provider and resolver proposals
  - Output compiled to `./out/extension.js`

- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/.npmrc`
  - NPM registry configuration

- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/.vscodeignore`
  - Excludes files from packaged extension

- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/esbuild.mts`
  - Build configuration (esbuild)

- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/package-lock.json`
  - Dependency lock file

- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/package.nls.json`
  - Localization strings (displayName, description, command titles)

## Development / Launch Configuration

- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/.vscode/launch.json`
  - Debug configuration for extension development

## Assets

- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/media/icon.png`
  - Extension icon

## Architecture Summary

The tunnel-forwarding extension is a lightweight wrapper around a Rust-based CLI tunnel provider (`code-tunnel`). Key architectural characteristics:

1. **Extension Bridge Pattern**: TypeScript extension exposes `vscode.TunnelProvider` API to VS Code core, delegating actual port forwarding to a compiled Rust binary via `child_process.spawn()`

2. **CLI Location Resolution**:
   - Dev mode: `cli/target/debug/code`
   - Stable: `vscode.env.appRoot/../../bin/code-tunnel` (macOS) or `../../../bin/code-tunnel` (Windows)
   - Insiders: `code-tunnel-insiders` binary

3. **Protocol Support**: Configurable HTTP/HTTPS with privacy modes (public/internet-accessible vs. private/localhost-only)

4. **Process Communication**:
   - Input: JSON array of `{number, privacy, protocol}` port objects via stdin
   - Output: Newline-delimited JSON from stderr containing `{port_format: string}` when ready
   - Manages process lifecycle with 10-second cleanup timeout after last port disposed

5. **Authentication**: Integrates with VS Code's GitHub authentication provider for CLI access token acquisition

6. **API Surface**: Implements experimental `tunnelFactory` proposal API; enabled via `enabledApiProposals` in manifest

## Porting Considerations for Tauri/Rust

- **Tight C Binding**: The TypeScript extension acts as a thin shim over the Rust CLI, meaning most tunneling logic already exists in Rust
- **Child Process Spawning**: Would need equivalent in Tauri's Rust runtime, likely via `std::process::Command` or tokio subprocess APIs
- **Stream Parsing**: JSON parsing from CLI output could be inlined into Rust, eliminating the TypeScript extension entirely
- **Authentication Flow**: GitHub OAuth integration currently uses VS Code's built-in authentication API; Tauri version would need native credential handling
- **Platform Detection**: macOS/Windows binary selection logic would become Rust conditional compilation or feature gates
- **Telemetry/Logging**: Output channel integration (`vscode.window.createOutputChannel`) would map to Tauri's logging/notification systems

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: Tunneling Architecture (Partition 44/80)

**Scope**: `extensions/tunnel-forwarding/` (4 files, 474 LOC)

**Research Question**: What patterns exist for VS Code's tunneling infrastructure, relevant to porting core IDE functionality to Tauri/Rust?

---

## Pattern 1: Extension-Based Tunnel Provider Registration

**Found in**: `extensions/tunnel-forwarding/src/extension.ts:77-107`

The tunnel-forwarding extension registers itself as a tunnel provider with VS Code's workspace API, following the plugin architecture pattern:

```typescript
export async function activate(context: vscode.ExtensionContext) {
  if (vscode.env.remoteAuthority) {
    return; // forwarding is local-only at the moment
  }

  const logger = new Logger(vscode.l10n.t('Port Forwarding'));
  const provider = new TunnelProvider(logger, context);

  context.subscriptions.push(
    vscode.commands.registerCommand('tunnel-forwarding.showLog', () => logger.show()),
    vscode.commands.registerCommand('tunnel-forwarding.restart', () => provider.restart()),

    provider.onDidStateChange(s => {
      vscode.commands.executeCommand('setContext', 'tunnelForwardingIsRunning', s.state !== State.Inactive);
    }),

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
    ),
  );
}
```

**Key aspects**:
- Implements `vscode.TunnelProvider` interface
- Declares capabilities via `TunnelInformation` parameter (protocol support, privacy options, elevation flags)
- Registers provider only for local environments (skips when `remoteAuthority` is present)
- Uses event emitters for state lifecycle coordination
- Commands registered for user-facing controls (logs, restart)
- State updates push context changes via `setContext` for conditional UI rendering

**API surface**: `vscode.workspace.registerTunnelProvider()` (enabledApiProposal: `tunnelFactory`)

---

## Pattern 2: Stateful Tunnel Lifecycle with Multi-Consumer Management

**Found in**: `extensions/tunnel-forwarding/src/extension.ts:139-201`

The `TunnelProvider` class manages a set of tunnel instances with complex state transitions:

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
      // fall through
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

State machine definition (`extension.ts:64-75`):
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

**Key aspects**:
- Discriminated union types for type-safe state variants
- Fall-through switch logic for lazy initialization (start if inactive, return pending promise during startup, return immediately if active)
- Per-tunnel disposal hooks for cleanup and set management
- Timeout-based cleanup (CLEANUP_TIMEOUT = 10s) to avoid restart overhead during privacy changes
- Multi-consumer pattern: multiple tunnels can coexist; provider manages shared process

---

## Pattern 3: Child Process Management & Stdio Stream Protocol

**Found in**: `extensions/tunnel-forwarding/src/extension.ts:266-342`

The extension spawns a Rust CLI (`code` or `code-tunnel`) as a subprocess and communicates via stdin/stdout:

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

  // ... progress UI ...

  let lastPortFormat: string | undefined;
  child.on('exit', status => {
    const msg = `[forwarding] exited with code ${status}`;
    this.logger.log('info', msg);
    progressP.complete();
    if (this.isInStateWithProcess(child)) {
      this.state = { state: State.Error, error: msg };
    }
  });

  child.on('error', err => {
    this.logger.log('error', `[forwarding] ${err}`);
    progressP.complete();
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
            portFormat: l.port_format,
            process: child,
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

**Key aspects**:
- CLI path determined at load time (dev vs. production, platform-specific binaries): `extensions/tunnel-forwarding/src/extension.ts:24-42`
- Authentication token passed via environment variable (`VSCODE_CLI_ACCESS_TOKEN`)
- Stdout and stderr split into lines via `splitNewLines()` stream transformer
- Stderr parsed as JSON to extract `port_format` string (e.g., `"https://host.domain.vscode.dev:{port}"`)
- Stdout logged for diagnostics
- Process lifecycle: spawn → wait for port_format JSON → Active state
- Progress UI shown during initialization via `vscode.window.withProgress()`

---

## Pattern 4: Stdin-Based Port Configuration Protocol

**Found in**: `extensions/tunnel-forwarding/src/extension.ts:250-264`

Active ports are communicated to the Rust CLI via stdin as JSON, enabling dynamic updates:

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

**Port record schema** (inferred from TypeScript):
```typescript
{ number: u16, privacy: 'private' | 'public', protocol: 'http' | 'https' }
```

**Key aspects**:
- Line-delimited JSON protocol (newline-terminated)
- Stateless: entire port list sent each update (not delta)
- Debounced shutdown: cleanup timeout allows brief window for privacy changes (avoid process restart)
- Sent on: tunnel creation, privacy change, tunnel disposal

---

## Pattern 5: Stream Transformation for Line Splitting

**Found in**: `extensions/tunnel-forwarding/src/split.ts`

A reusable Transform stream that splits binary chunks on delimiter bytes:

```typescript
import { Transform } from 'stream';

export const splitNewLines = () => new StreamSplitter('\n'.charCodeAt(0));

/**
 * Copied and simplified from src\vs\base\node\nodeStreams.ts
 *
 * Exception: does not include the split character in the output.
 */
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
- Stateful buffering for incomplete lines across chunks
- Efficient: uses `indexOf()` and `subarray()` (no copying until necessary)
- Flushes remaining buffer on stream end
- Returns chunks without the delimiter
- Comment notes it's copied from `src/vs/base/node/nodeStreams.ts` (centralized utility)

---

## Pattern 6: Deferred Promise for Async Coordination

**Found in**: `extensions/tunnel-forwarding/src/deferredPromise.ts`

A utility for externally-resolved promises, used to coordinate progress UI with subprocess initialization:

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

  public get value() {
    return this.outcome?.outcome === DeferredOutcome.Resolved ? this.outcome?.value : undefined;
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

  public error(err: unknown) {
    return new Promise<void>(resolve => {
      this.errorCallback(err);
      this.outcome = { outcome: DeferredOutcome.Rejected, value: err };
      resolve();
    });
  }
}
```

**Used in**: `extension.ts:283-294` to coordinate progress UI lifetime with subprocess initialization.

**Key aspects**:
- Generic type-safe implementation
- Discriminated union for outcome tracking
- Provides introspection (`isSettled`, `isResolved`, `isRejected`, `value`)
- Both `complete()` and `error()` return promises (allow awaiting settlement)
- Copied from core (`src/vs/base/common/async.ts`)

---

## Pattern 7: Rust CLI Integration Layer (Compiled Binary Fallback)

**Found in**: `extensions/tunnel-forwarding/src/extension.ts:24-42`

Platform and build-mode detection for CLI binary selection:

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

The Rust side implements the protocol via stdin/stdout. Relevant Rust patterns (`cli/src/tunnels/`):

**Protocol structures** (from `cli/src/tunnels/protocol.rs`):
```rust
pub mod forward_singleton {
  pub const METHOD_SET_PORTS: &str = "set_ports";

  #[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
  pub struct PortRec {
    pub number: u16,
    pub privacy: PortPrivacy,
    pub protocol: PortProtocol,
  }

  pub type PortList = Vec<PortRec>;

  #[derive(Serialize, Deserialize)]
  pub struct SetPortsParams {
    pub ports: PortList,
  }

  #[derive(Serialize, Deserialize)]
  pub struct SetPortsResponse {
    pub port_format: Option<String>,
  }
}
```

**Local forwarding handler** (from `cli/src/tunnels/local_forwarding.rs:100-138`):
```rust
impl PortForwardingSender {
  pub fn set_ports(&self, ports: PortList) {
    let mut current = self.current.lock().unwrap();
    self.sender.lock().unwrap().send_modify(|v| {
      for p in current.iter() {
        if !ports.contains(p) {
          let n = v.get_mut(&p.number).expect("expected port in map");
          n.count[p.privacy] -= 1;
          if n.count.is_empty() {
            v.remove(&p.number);
          }
        }
      }

      for p in ports.iter() {
        if !current.contains(p) {
          match v.get_mut(&p.number) {
            Some(n) => {
              n.count[p.privacy] += 1;
              n.protocol = p.protocol;
            }
            None => {
              let mut count = PortCount::default();
              count[p.privacy] += 1;
              v.insert(
                p.number,
                PortMapRec {
                  count,
                  protocol: p.protocol,
                },
              );
            }
          };
        }
      }

      current.splice(.., ports);
    });
  }
}
```

**Key aspects**:
- Rust CLI compiled into `bin/code-tunnel` or `bin/code-tunnel-insiders`
- Stable vs. insiders variant detection via `vscode.env.appQuality`
- Dev build paths relative to repo structure (`cli/target/debug/code`)
- Port privacy and protocol tracked per-port with reference counting
- Watch-based reactive updates: changes to ports map trigger tunnel forwarding state changes

---

## Integration Pattern: Extension-to-CLI Communication Flow

```
┌─────────────────────────────────────────────────────────────┐
│ VS Code IDE (TypeScript/Electron)                           │
│ extension.ts: TunnelProvider                                │
└───────────────────────────────────────────────────────────┘
                            │
                            │ spawn()
                            │
┌─────────────────────────────────────────────────────────────┐
│ Rust CLI (code or code-tunnel)                              │
│ cli/src/tunnels/local_forwarding.rs                         │
│                                                              │
│ ┌──────────────────────────────────────────────────────┐   │
│ │ stdin: JSON PortList                                 │   │
│ │ { number, privacy, protocol }                        │   │
│ │ ────────────────────────────────────────────────────│   │
│ │ stdout: Logging                                      │   │
│ │ stderr: JSON { port_format: "https://..." }          │   │
│ └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ tunnel.add_port_tcp()
                            │
┌─────────────────────────────────────────────────────────────┐
│ Dev Tunnels (Remote Tunnel Service)                         │
│ cli/src/tunnels/dev_tunnels.rs: ActiveTunnel               │
└─────────────────────────────────────────────────────────────┘
```

---

## Porting Considerations to Tauri/Rust

### Architectural Decisions

1. **Native Integration**: Tunnel forwarding is implemented as a compiled Rust binary (`code-tunnel`) executed by the TypeScript extension. In a Tauri port, this could be:
   - Integrated directly as a Tauri command (eliminating subprocess overhead)
   - Shared library interface via FFI
   - Remains as subprocess if maintaining extension compatibility

2. **State Machine Complexity**: The discriminated-union state machine (`State.Inactive | State.Starting | State.Active | State.Error`) would translate to Rust enum variants with associated data. Tauri's event system could replace `EventEmitter` for state change notifications.

3. **Stream Processing**: The `StreamSplitter` pattern for line-delimited JSON is fundamental to the protocol. In Rust, this would use `tokio::codec::LinesCodec` or similar.

4. **Extension API Surface**: The `vscode.workspace.registerTunnelProvider()` API would need equivalent in Tauri's plugin system or core IDE API.

5. **Authentication**: GitHub auth session retrieval (`vscode.authentication.getSession()`) would require Tauri's credential store integration or direct OAuth flow.

### Protocol-Level Reuse

- **Port record schema** is language-agnostic JSON: directly portable
- **Forward singleton protocol** (set_ports RPC) is already in Rust
- **PortPrivacy and PortProtocol** enums exist in both TypeScript and Rust
- **Port reference counting logic** can be shared or re-implemented

### Code Sharing Opportunities

- Rust CLI logic (local_forwarding.rs, dev_tunnels.rs) can be reused
- Stream transformation patterns (splitNewLines equivalent)
- Port management state machines
- Authentication flow (with Tauri adaptations)

---

## Summary

The tunnel-forwarding extension demonstrates a **binary-separated architecture**: a TypeScript extension provides the IDE interface (state management, user commands, lifecycle coordination), while delegating port forwarding logic to a compiled Rust CLI via line-delimited JSON over stdin/stdout. This pattern reflects VS Code's broader plugin model, where extensions coordinate with external tools.

For a Tauri port, this separation could be:
- **Eliminated** by moving port forwarding directly into the Rust core (tightest integration)
- **Preserved** as a plugin/extension model with Tauri command boundaries
- **Adapted** using Rust FFI or library integration for shared ownership of state

The state machine, stream handling, and protocol definitions are all directly portable. The primary porting work would be in replacing VS Code's extension API with Tauri equivalents and deciding the integration boundary between core and plugin layers.

**Key files**:
- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/src/extension.ts` (244 LOC) — Provider lifecycle, process management, state machine
- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/src/deferredPromise.ts` (62 LOC) — Async coordination utility
- `/home/norinlavaee/projects/vscode-atomic/extensions/tunnel-forwarding/src/split.ts` (52 LOC) — Stream transformer
- `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/local_forwarding.rs` (250+ LOC) — Rust-side port forwarding state machine
- `/home/norinlavaee/projects/vscode-atomic/cli/src/tunnels/protocol.rs` — Shared protocol definitions (forward_singleton, PortPrivacy, PortProtocol)

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
