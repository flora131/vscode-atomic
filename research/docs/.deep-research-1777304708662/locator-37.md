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

