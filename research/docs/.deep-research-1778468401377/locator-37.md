# Remote Authority Resolution - vscode-test-resolver Extension

## Implementation

- `extensions/vscode-test-resolver/src/extension.ts` — Node-based resolver implementation registering `registerRemoteAuthorityResolver('test', {...})` with `ResolverResult` and `ResolvedAuthority` types, connection token generation, tunnel feature provisioning, and error handling patterns
- `extensions/vscode-test-resolver/src/extension.browser.ts` — Browser-based resolver activation with `ManagedMessagePassing` and `ManagedResolvedAuthority` patterns for HTTP header negotiation
- `extensions/vscode-test-resolver/src/download.ts` — VS Code Server binary download/extraction utilities for platform-specific builds (win32, darwin, linux-x64) 
- `extensions/vscode-test-resolver/src/util/processes.ts` — Cross-platform process termination interface (`TerminateResponse`) for Windows (taskkill) and Unix/Darwin (shell script)

## Configuration

- `extensions/vscode-test-resolver/package.json` — Extension manifest with `onResolveRemoteAuthority:test` activation, `enabledApiProposals: ["resolvers", "tunnels"]`, remote command palette contributions, `vscode-remote` URI scheme formatter, and test configuration options
- `extensions/vscode-test-resolver/tsconfig.json` — TypeScript configuration referencing proposed type definitions at `../../src/vscode-dts/vscode.proposed.resolvers.d.ts` and `vscode.proposed.tunnels.d.ts`
- `extensions/vscode-test-resolver/tsconfig.browser.json` — Browser-specific TypeScript configuration
- `extensions/vscode-test-resolver/esbuild.mts` — Node/Electron bundler entry point for `extension.ts`
- `extensions/vscode-test-resolver/esbuild.browser.mts` — Browser bundler entry point for `extension.browser.ts`
- `extensions/vscode-test-resolver/.vscode/launch.json` — Debug configuration launching extension with `--remote=test+test` authority

## Notable Clusters

- `extensions/vscode-test-resolver/src/` — 4 TypeScript files implementing remote authority resolution, server provisioning, and cross-platform process management
- `extensions/vscode-test-resolver/src/util/` — Platform abstraction for process lifecycle (1 file)

## Summary

The vscode-test-resolver extension demonstrates the VS Code remote authority resolution pattern through both Node/Electron (`registerRemoteAuthorityResolver` API) and browser contexts (`ManagedMessagePassing`). It showcases tunnel feature negotiation, server binary provisioning across platforms, and connection lifecycle management with error states. This is directly relevant to understanding how a Tauri/Rust-based core would need to replace TypeScript-based resolver logic and authority negotiation while maintaining the same VS Code API contract for remote sessions.
