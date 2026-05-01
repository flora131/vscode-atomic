# File Locations: Bootstrap Server (src/bootstrap-server.ts)

## Implementation

- `src/bootstrap-server.ts` — Server-side bootstrap shim (7 LOC) that disables the Electron process environment variable to prevent it from interfering with server initialization. This minimal module is imported at the start of both server entry points to ensure proper global state setup.

## Related Bootstrap Infrastructure

- `src/bootstrap-node.ts` — Node.js environment initialization with stack trace limit configuration, signal handling (SIGPIPE), and module loading adjustments
- `src/bootstrap-esm.ts` — ESM module resolution hooks that map `fs` to `original-fs` when running under Electron or with Electron module versions
- `src/bootstrap-meta.ts` — Product metadata provider
- `src/bootstrap-cli.ts` — CLI entry point bootstrap with NLS configuration setup
- `src/bootstrap-fork.ts` — Fork process bootstrap
- `src/bootstrap-import.ts` — Import resolution bootstrap

## Server Entry Points

- `src/server-main.ts` — Primary server entry point that imports `bootstrap-server.js` as first step, then initializes HTTP server and extension host infrastructure
- `src/server-cli.ts` — CLI server entry point that imports `bootstrap-server.js`, sets up NLS configuration, and handles extension management commands
- `src/vs/server/node/server.main.ts` — Actual server implementation module
- `src/vs/server/node/server.cli.ts` — Server CLI implementation module

## Tests

- `src/vs/server/test/node/serverMain.test.ts` — Tests for server.main directory creation with recursive mkdir operations
- `src/vs/server/test/node/serverConnectionToken.test.ts` — Connection token functionality tests
- `src/vs/server/test/node/serverAgentHostManager.test.ts` — Agent host manager tests
- `src/vs/server/test/node/serverLifetimeService.test.ts` — Server lifetime service tests

## Server Infrastructure

The following modules in `src/vs/server/node/` comprise the server infrastructure that bootstrap-server.ts hands off to:

- `remoteExtensionHostAgentServer.ts` — Remote extension host agent protocol implementation
- `extensionHostConnection.ts` — Extension host connection management
- `remoteExtensionsScanner.ts` — Extension discovery and scanning
- `extensionsScannerService.ts` — Extension scanner service
- `serverServices.ts` — Service registry and dependency injection
- `serverEnvironmentService.ts` — Server environment configuration
- `serverLifetimeService.ts` — Server lifecycle management
- `serverConnectionToken.ts` — Connection token validation
- `webClientServer.ts` — Web client server interface
- `remoteExtensionManagement.ts` — Extension installation and updates
- `remoteFileSystemProviderServer.ts` — Remote filesystem operations
- `remoteTerminalChannel.ts` — Remote terminal communication
- `remoteLanguagePacks.ts` — Language pack management
- `remoteAgentEnvironmentImpl.ts` — Agent environment implementation
- `remoteExtensionHostAgentCli.ts` — CLI interface for remote extension host
- `serverAgentHostManager.ts` — Agent host session management
- `extensionHostStatusService.ts` — Extension host status tracking

## Summary

The `bootstrap-server.ts` module is a minimal 7-line shim that prevents Electron environment variables from interfering with Node.js server initialization. It is the first import in both server entry points (`server-main.ts` and `server-cli.ts`), ensuring proper global state before any other imports occur. The module then hands off to the full server implementation in `src/vs/server/node/`, which provides extension hosting, language services, debugging, source control, and terminal functionality for the remote VS Code server architecture.
