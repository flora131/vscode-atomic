# Remote Server Entrypoint: File Locator Report

## Implementation

### Core Server Entrypoint
- `/home/norinlavaee/projects/vscode-atomic/src/server-main.ts` (285 LOC) - Primary entry point; orchestrates HTTP server setup, socket lifecycle, license handling, and delegation to remote extension host agent server. Handles CLI vs. server mode branching, port allocation, and graceful shutdown.

### Server Bootstrapping & Setup
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-server.ts` - Minimal bootstrap file that disables `ELECTRON_RUN_AS_NODE` environment variable
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/server.main.ts` (72 LOC) - Exported module providing `spawnCli()` and `createServer(address)` entry points; dynamically imports and instantiates the remote extension host agent server
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/remoteExtensionHostAgentServer.ts` - Core server implementation class `RemoteExtensionHostAgentServer` implementing `IServerAPI` interface; handles HTTP requests, WebSocket upgrades, server errors, connection management (extension host and management connections), and web client serving
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/serverServices.ts` - Service dependency injection and IPC server setup; creates instantiation service with all server dependencies (configuration, logging, telemetry, file service, extension management, terminal, MCP services, etc.)
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/serverEnvironmentService.ts` - Server-specific environment service for parsing and managing command-line arguments and configuration paths
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/serverLifetimeService.ts` - Server lifetime management service

### Remote Protocol & Connection Handling
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/remote/common/remoteAgentConnection.ts` - Wire protocol definitions: `ConnectionType` enum (Management, ExtensionHost, Tunnel), handshake message types (`AuthRequest`, `SignRequest`, `ConnectionTypeRequest`, `ErrorMessage`, `OKMessage`), and connection establishment logic with timeout handling
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/extensionHostConnection.ts` - Extension host connection management; builds process environment, spawns extension host process, handles socket communication
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/remoteExtensionManagement.ts` - Management connection implementation

### Connection & Authentication
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/serverConnectionToken.ts` - Server connection token generation, parsing, and validation logic
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/remote/common/remoteAgentEnvironment.ts` - Remote agent connection context definitions

### Web Client Server
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/webClientServer.ts` - HTTP web client serving (UI assets, configuration endpoints)

### IPC & Networking Foundation
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/parts/ipc/common/ipc.net.ts` - `PersistentProtocol` implementation for message-passing over sockets
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/parts/ipc/node/ipc.net.ts` - Node.js socket implementations (`NodeSocket`, `WebSocketNodeSocket`, server socket setup)
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/parts/ipc/common/ipc.ts` - Base IPC abstractions (`IMessagePassingProtocol`, `IPCServer`, `Client`)

### File System & Utilities
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/remoteFileSystemProviderServer.ts` - File system provider for remote file operations
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/remoteExtensionsScanner.ts` - Extension scanning service on remote
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/remoteTerminalChannel.ts` - Terminal/PTY channel implementation
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/remoteLanguagePacks.ts` - NLS configuration
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/remoteAgentEnvironmentImpl.ts` - Remote environment information provider

### Agent Host Management
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/serverAgentHostManager.ts` - Manages agent host lifecycle and connections

## Tests

### Server Main Tests
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/test/node/serverMain.test.ts` - Tests for server main initialization and request handling

### Connection & Token Tests
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/test/node/serverConnectionToken.test.ts` - Tests for connection token generation and parsing
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/test/node/serverAgentHostManager.test.ts` - Tests for agent host manager

### Lifetime Service Tests
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/test/node/serverLifetimeService.test.ts` - Tests for server lifetime management

### Remote Connection Tests
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/remote/test/common/remoteHosts.test.ts` - Remote host resolution tests
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/agentHost/test/node/serverUrls.test.ts` - Server URL tests

### IPC Tests
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/parts/ipc/test/node/ipc.net.test.ts` - Network IPC protocol tests
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/parts/ipc/test/common/ipc.test.ts` - Common IPC tests

## Types / Interfaces

### Server API Interface
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/remoteExtensionHostAgentServer.ts` - Exports `IServerAPI` interface with methods: `handleRequest(req, res)`, `handleUpgrade(req, socket)`, `handleServerError(err)`, `dispose()`

### Connection Types
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/remote/common/remoteAgentConnection.ts` - Exports handshake message types: `HandshakeMessage`, `ConnectionTypeRequest`, `ErrorMessage`, `OKMessage`, `AuthRequest`, `SignRequest`
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/remote/common/remoteAgentEnvironment.ts` - `RemoteAgentConnectionContext` type

### Server Environment Types
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/serverEnvironmentService.ts` - `IServerEnvironmentService`, `ServerParsedArgs` interfaces

### Remote Extension Host Parameters
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/remote/common/remoteAgentConnection.ts` - `IRemoteExtensionHostStartParams`, `ITunnelConnectionStartParams` interfaces

## Configuration

### Server-specific CLI Arguments
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/serverEnvironmentService.ts` - Defines `serverOptions` object with supported CLI flags: `--host`, `--port`, `--socket-path`, `--compatibility`, `--server-data-dir`, `--extensions-dir`, etc.

### Server License & Greeting
- `/home/norinlavaee/projects/vscode-atomic/src/server-main.ts` - Handles `product.serverLicense` and `product.serverLicensePrompt` from product configuration

### Bootstrap Configuration
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/server.main.ts` - Configures remote data folder path (`VSCODE_AGENT_FOLDER` env variable or `product.serverDataFolderName`)

## Documentation

### Session & Agent Host Architecture
- `/home/norinlavaee/projects/vscode-atomic/src/vs/sessions/contrib/remoteAgentHost/REMOTE_AGENT_HOST_SESSIONS_PROVIDER.md` - Documents remote agent host sessions provider architecture

### Test Scenarios
- `/home/norinlavaee/projects/vscode-atomic/src/vs/sessions/test/e2e/scenarios/` - End-to-end test scenario documentation

### Remote Agent Host Services
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/agentHost/test/node/AGENTS.md` - Documents agent host services structure

## Notable Clusters

### Server Connection Initialization Pipeline
Files involved in bootstrap→listen→request handling flow:
- `src/server-main.ts` (HTTP server setup, lifecycle)
- `src/vs/server/node/server.main.ts` (module exports)
- `src/vs/server/node/remoteExtensionHostAgentServer.ts` (request/upgrade handlers)
- `src/vs/platform/remote/common/remoteAgentConnection.ts` (protocol definitions)
- `src/vs/server/node/serverConnectionToken.ts` (authentication)

### Extension Host Process Management
Files managing extension host lifecycle on remote:
- `src/vs/server/node/extensionHostConnection.ts` (spawning, environment setup)
- `src/vs/server/node/extensionHostStatusService.ts` (status tracking)
- `src/vs/workbench/services/extensions/common/extensionHostProtocol.ts` (IPC messages)

### IPC & Wire Protocol Stack
Network and message-passing foundations:
- `src/vs/base/parts/ipc/common/ipc.net.ts` (PersistentProtocol)
- `src/vs/base/parts/ipc/node/ipc.net.ts` (Node socket implementations)
- `src/vs/platform/remote/common/remoteAgentConnection.ts` (connection handshake)

### Server Dependency Injection
Service instantiation and initialization:
- `src/vs/server/node/serverServices.ts` (service registry and setup)
- `src/vs/server/node/serverEnvironmentService.ts` (environment configuration)
- Contains 15+ service registrations (file service, extension management, configuration, telemetry, terminal, MCP, etc.)

---

## Summary

The remote server entrypoint (`src/server-main.ts`) is a 285-line TypeScript file that serves as the Node.js process entry point for VS Code's remote development server. It orchestrates HTTP server creation, WebSocket upgrade handling, and delegates all protocol work to `RemoteExtensionHostAgentServer`.

The server implements a multi-connection architecture supporting three connection types (Management, ExtensionHost, Tunnel) via a wire protocol defined in `remoteAgentConnection.ts`. Connection establishment includes handshake messages for authentication and connection type negotiation.

The implementation stack layers HTTP/WebSocket at the node-http level, PersistentProtocol for framing at the IPC layer, and domain-specific channels for remote services (file system, extensions, terminal) built on the IPC foundation. Bootstrap files prepare the Node.js module system and global state before the server's service instantiation phase.

Key design decisions for Tauri/Rust porting:
1. **HTTP/WebSocket interface** - Currently node:http; Tauri uses http infrastructure that must support upgrading HTTP connections to WebSocket
2. **Connection handshake protocol** - Defined as JSON message exchange; must be implemented in Rust server with identical message schema
3. **PersistentProtocol framing** - Provides message length-prefixing and chunking; Rust implementation must preserve this wire format
4. **Service dependency graph** - 15+ interdependent services instantiated via IPC patterns; requires careful mapping to Rust type system
5. **Authentication/tokens** - `ServerConnectionToken` validates connection requests; Rust server must replicate token generation and validation logic
6. **Extension host lifecycle** - Child process spawning for extension host; Tauri/Rust must handle subprocess IPC equivalently
