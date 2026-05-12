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
