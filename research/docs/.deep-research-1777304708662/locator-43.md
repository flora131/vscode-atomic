# File Locations: Tunnel Forwarding Extension (Partition 43)

## Implementation Files

- `extensions/tunnel-forwarding/src/extension.ts` — Main extension entry point implementing `TunnelProvider` interface; registers tunnel provider with VS Code via `workspace.registerTunnelProvider()` at line 93, manages port forwarding CLI process lifecycle and tunnel lifecycle
- `extensions/tunnel-forwarding/src/deferredPromise.ts` — Deferred promise utility class copied from VS Code core async utilities; used for managing asynchronous setup of port forwarding process
- `extensions/tunnel-forwarding/src/split.ts` — Stream splitter utility for parsing newline-delimited CLI output; provides `splitNewLines()` helper and `StreamSplitter` Transform stream class

## Configuration Files

- `extensions/tunnel-forwarding/package.json` — Extension manifest defining displayName, description, version 10.0.0, VS Code engine requirement (^1.82.0), enabled API proposals (resolvers, tunnelFactory), activation event (onTunnel), and contributed commands (showLog, restart)
- `extensions/tunnel-forwarding/tsconfig.json` — TypeScript configuration extending base config, targeting src/ → out/ directories, including VSCode proposed type definitions for resolvers and tunnelFactory
- `extensions/tunnel-forwarding/esbuild.mts` — ESBuild configuration for bundling extension; outputs to dist/ directory with platform: node entry point at src/extension.ts

## Configuration/Metadata Files

- `extensions/tunnel-forwarding/package.nls.json` — Localization strings for extension UI text
- `extensions/tunnel-forwarding/.npmrc` — NPM configuration
- `extensions/tunnel-forwarding/.vscodeignore` — Extension packaging ignore patterns
- `extensions/tunnel-forwarding/.vscode/launch.json` — Debug launch configuration
- `extensions/tunnel-forwarding/media/icon.png` — Extension icon asset

## Notable Implementation Details

The `TunnelProvider` class (lines 139-343 in extension.ts) implements VS Code's tunnel provider protocol with key responsibilities:

- **Tunnel creation**: `provideTunnel()` method (line 158) handles tunnel provisioning with privacy consent workflow for public ports
- **CLI process management**: Spawns the VS Code CLI (`code-tunnel` or `code-tunnel-insiders`) as a child process, manages stdin/stdout/stderr communication
- **State machine**: Manages four discrete states (Starting, Active, Inactive, Error) tracking CLI process lifecycle and readiness
- **Port synchronization**: Updates active ports list via stdin to running CLI process; implements 10-second cleanup timeout before tearing down idle process
- **Privacy handling**: Implements `TunnelPrivacyId` enum (Private/Public) with user consent for public port forwarding
- **Logging**: Uses `Logger` class wrapping VSCode's LogOutputChannel for structured logging with show/clear methods

The extension activates only on local (non-remote) VS Code instances and requires GitHub authentication (`vscode.authentication.getSession('github')`).

## Summary

The tunnel-forwarding extension (12 files, ~474 LOC) provides VS Code's port forwarding abstraction layer via a tunnel provider implementation. It bridges the VSCode extension API (TypeScript/Electron-based) with a native CLI process for managing SSH/HTTP tunnels. The implementation demonstrates how VS Code's core tunnel infrastructure relies on spawning external native binaries and communicating via JSON-serialized stdin/stdout, a pattern that would require careful translation to Rust/Tauri given differences in process management, IPC mechanisms, and the availability of the underlying `code-tunnel` CLI as a native component.
