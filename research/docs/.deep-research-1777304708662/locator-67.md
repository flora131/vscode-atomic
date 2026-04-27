# CLI Bootstrap Shim (src/cli.ts) - File Location Research

## Summary
VS Code's CLI entry point (`src/cli.ts`) is a minimal 26-line TypeScript bootstrap shim that orchestrates initialization for command-line functionality. It performs NLS configuration, enables portable mode support, and then delegates to the full CLI implementation in `src/vs/code/node/cli.ts`.

## Implementation

### Bootstrap Entry Point
- `src/cli.ts` — Main CLI entry point that orchestrates NLS setup, portable configuration, ESM bootstrapping, and delegates to full CLI server implementation

### Related Bootstrap Files (Imported Dependencies)
- `src/bootstrap-cli.ts` — Clears `VSCODE_CWD` environment variable early to prevent CWD leakage to parent shell
- `src/bootstrap-node.ts` — Configures portable mode support via `configurePortable()`
- `src/bootstrap-esm.ts` — Bootstraps ESM module loading system
- `src/bootstrap-meta.ts` — Provides product metadata used for NLS configuration

### Full CLI Implementation
- `src/vs/code/node/cli.ts` — Complete CLI command handling (50+ lines) including argument parsing, subprocess spawning for install-extension, list-extensions, telemetry, and other CLI-specific operations

## Tests

### Sanity Tests
- `test/sanity/src/cli.test.ts` — Multi-platform CLI sanity tests covering alpine-arm64, alpine-x64, darwin-arm64, darwin-x64, linux-arm64, and linux-x64 architectures

## Configuration / Build

### Build Files
- `build/gulpfile.cli.ts` — Primary CLI build task orchestration
- `build/azure-pipelines/cli/cli-compile.yml` — CI compilation pipeline for CLI
- `build/azure-pipelines/cli/cli-apply-patches.yml` — CI patch application pipeline
- `build/azure-pipelines/alpine/product-build-alpine-cli.yml` — Alpine Linux CLI build pipeline
- `build/azure-pipelines/darwin/product-build-darwin-cli.yml` — macOS CLI build pipeline
- `build/azure-pipelines/linux/product-build-linux-cli.yml` — Linux CLI build pipeline
- `build/azure-pipelines/win32/product-build-win32-cli.yml` — Windows CLI build pipeline

### Package Scripts (package.json)
- `compile-cli` — NPM script for CLI compilation
- `watch-cli` — NPM script for CLI watch mode

## Architecture Notes

The CLI bootstrap pattern involves:
1. Early environment cleanup (`bootstrap-cli.ts`)
2. Node.js configuration (`bootstrap-node.ts`)
3. Module system setup (`bootstrap-esm.ts`)
4. Internationalization (NLS) configuration (`resolveNLSConfiguration`)
5. Portable mode support setup
6. CLI environment flag setting (`VSCODE_CLI=1`)
7. Final delegation to full CLI implementation (`src/vs/code/node/cli.ts`)

The actual CLI command processing happens in `src/vs/code/node/cli.ts` which handles:
- Extension installation/uninstallation/updates
- Extension location lookup
- MCP (Model Context Protocol) integration
- Telemetry operations
- Subprocess spawning and process management
- stdin/stdout handling
- File watching and synchronization

## Porting Implications for Tauri/Rust

To port this CLI bootstrap from TypeScript/Electron to Tauri/Rust would require:
1. Implementing a Rust CLI module replacing `src/cli.ts` and `src/vs/code/node/cli.ts`
2. Replicating NLS configuration system in Rust
3. Porting bootstrap initialization sequence to Rust startup
4. Implementing environment variable and file descriptor management
5. Porting all CLI command handlers (extension management, MCP integration, telemetry) to Rust
6. Maintaining cross-platform compatibility (Alpine, Darwin, Linux, Windows)
7. Replicating process spawning and subprocess management via Rust's std::process or equivalent
8. Porting stdin/stdout handling and file watching mechanisms
