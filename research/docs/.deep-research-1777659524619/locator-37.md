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

