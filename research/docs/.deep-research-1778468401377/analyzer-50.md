### Files Analysed

1. `/home/norinlavaee/projects/vscode-atomic/src/server-main.ts` (285 LOC) — top-level Node.js entrypoint
2. `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-server.ts` (7 LOC) — pre-import env cleanup
3. `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/server.main.ts` (72 LOC) — post-ESM-load setup and export bridge
4. `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/remoteExtensionHostAgentServer.ts` (814 LOC) — core server class + `createServer` factory
5. `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/serverServices.ts` (421 LOC) — DI container assembly, channel registration
6. `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/serverConnectionToken.ts` (133 LOC) — connection token auth
7. `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/extensionHostConnection.ts` (371 LOC) — per-connection extension host lifecycle
8. `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/serverEnvironmentService.ts` (277 LOC) — CLI option schema + environment service
9. `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/webClientServer.ts` (lines 1–120 read) — static web UI serving
10. `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/remote/common/remoteAgentConnection.ts` (lines 1–100 read) — handshake wire types

---

### Per-File Notes

#### 1. `src/server-main.ts`

**Role**: The single Node.js entrypoint. Parses CLI args, handles license prompt, creates the raw `http.Server`, then lazily delegates every HTTP request and WebSocket upgrade to the IServerAPI implementation loaded via dynamic import.

**Key symbols and line references**:

- `src/server-main.ts:6` — `import './bootstrap-server.js'` must be first because it deletes `ELECTRON_RUN_AS_NODE` before any other import can touch global state.
- `src/server-main.ts:26–30` — `minimist` parse. Boolean flags: `start-server`, `accept-server-license-terms`, `print-ip-address`. String flags: `socket-path`, `host`, `port`, `agent-host-port`, `agent-host-path`, etc.
- `src/server-main.ts:31–38` — Environment variable fallback: `VSCODE_SERVER_HOST`, `VSCODE_SERVER_PORT`, `VSCODE_SERVER_ACCEPT_SERVER_LICENSE_TERMS` are checked if the corresponding flag is absent.
- `src/server-main.ts:43` — `shouldSpawnCli` is `true` when `--help`, `--version`, or an extension management flag is present without `--start-server`.
- `src/server-main.ts:45` — NLS configuration resolved synchronously via top-level `await resolveNLSConfiguration(...)` before any branching.
- `src/server-main.ts:47–50` — CLI path: calls `loadCode(nlsConfig).then(mod => mod.spawnCli())`.
- `src/server-main.ts:52–63` — Server path: `getRemoteExtensionHostAgentServer()` is a memoised promise factory. It calls `loadCode` once then `mod.createServer(address)` where `address` is populated only after `server.listen` fires (`src/server-main.ts:130`).
- `src/server-main.ts:65–82` — License gate: reads `product.serverLicense` array, optionally prompts via `readline` if `product.serverLicensePrompt` is set and `--accept-server-license-terms` was not passed. Exits with code 1 on refusal.
- `src/server-main.ts:88–95` — `http.createServer` callback: forwards every request to `remoteExtensionHostAgentServer.handleRequest(req, res)`. First request fires `perf.mark('code/server/firstRequest')`.
- `src/server-main.ts:96–104` — `server.on('upgrade', ...)`: forwards WebSocket upgrades to `remoteExtensionHostAgentServer.handleUpgrade(req, socket)`.
- `src/server-main.ts:105–108` — `server.on('error', ...)`: delegates to `remoteExtensionHostAgentServer.handleServerError(err)`.
- `src/server-main.ts:110–115` — Listen options: either `{ path: socketPath }` (Unix socket) or `{ host, port }` (TCP). Port parsing calls `parsePort` which supports ranges (e.g. `3000-3010`).
- `src/server-main.ts:116–144` — `server.listen` callback: writes the address to stdout (including the **sentinel line** `Extension host agent listening on <port>` at line 137 that remote-SSH watches for), records `vscodeServerListenTime`, then triggers `getRemoteExtensionHostAgentServer()` eagerly.
- `src/server-main.ts:146–151` — `process.on('exit', ...)`: closes the HTTP server and calls `dispose()` on the agent server.
- `src/server-main.ts:170–190` — `parsePort`: exact integer, `start-end` range (calls `findFreePort`), or default `8000`. The exact error message `Could not find free port in range` at line 181 is documented as a Remote-SSH API dependency.
- `src/server-main.ts:207–225` — `findFreePort`: loops from `start` to `end`, opening a temporary `http.createServer` to test each port.
- `src/server-main.ts:227–255` — `loadCode`: sets `VSCODE_NLS_CONFIG` and `VSCODE_HANDLES_SIGPIPE`, injects dev node_module lookup path when `VSCODE_DEV` is set, calls `removeGlobalNodeJsModuleLookupPaths()`, then `bootstrapESM()`, then returns `import('./vs/server/node/server.main.js')`.

**Control flow summary**: `server-main.ts` creates the HTTP server immediately and starts listening before any VS Code modules are loaded. The heavy DI boot happens lazily on the first incoming connection (HTTP or WebSocket). This means the port is bound and printed before the extension host subsystem initialises.

---

#### 2. `src/bootstrap-server.ts`

**Role**: Single-line guard that deletes `process.env['ELECTRON_RUN_AS_NODE']` at `src/bootstrap-server.ts:7` so that `bootstrap-esm.js` does not redefine `fs` with Electron's patched version. Must be imported first in `server-main.ts`.

---

#### 3. `src/vs/server/node/server.main.ts`

**Role**: Bridge module loaded after `bootstrapESM()`. Runs module-load-time setup, then exports two functions used by `server-main.ts`.

**Key symbols**:

- `src/vs/server/node/server.main.ts:19` — `perf.mark('code/server/codeLoaded')` marks when this module finished loading.
- `src/vs/server/node/server.main.ts:39` — `REMOTE_DATA_FOLDER` resolution order: `--server-data-dir` arg → `VSCODE_AGENT_FOLDER` env var → `os.homedir() + product.serverDataFolderName` (typically `.vscode-remote`).
- `src/vs/server/node/server.main.ts:40–48` — Derives `USER_DATA_PATH`, `APP_SETTINGS_HOME`, `GLOBAL_STORAGE_HOME`, `LOCAL_HISTORY_HOME`, `MACHINE_SETTINGS_HOME`, `BUILTIN_EXTENSIONS_FOLDER_PATH` from `REMOTE_DATA_FOLDER`. Forces `args['user-data-dir']` and `args['builtin-extensions-dir']`.
- `src/vs/server/node/server.main.ts:51–57` — Ensures all data directories exist with mode `0o700` (owner-only access).
- `src/vs/server/node/server.main.ts:62–64` — `export function spawnCli()`: delegates to `runCli(args, REMOTE_DATA_FOLDER, serverOptions)` in `remoteExtensionHostAgentCli.ts`.
- `src/vs/server/node/server.main.ts:69–71` — `export function createServer(address)`: delegates to `doCreateServer(address, args, REMOTE_DATA_FOLDER)` from `remoteExtensionHostAgentServer.ts`.

---

#### 4. `src/vs/server/node/remoteExtensionHostAgentServer.ts`

**Role**: Core HTTP + WebSocket server class and its factory function. Implements `IServerAPI` (the interface the outer layer in `server-main.ts` depends on).

**Class: `RemoteExtensionHostAgentServer` (line 59)**

Private state:
- `_extHostConnections: { [reconnectionToken: string]: ExtensionHostConnection }` — active extension host connections indexed by token (`remoteExtensionHostAgentServer.ts:61`).
- `_managementConnections: { [reconnectionToken: string]: ManagementConnection }` — active management (IPC) connections (`remoteExtensionHostAgentServer.ts:62`).
- `_allReconnectionTokens: Set<string>` — records all tokens ever seen, enabling detection of "seen before" vs "never seen" reconnect errors (`remoteExtensionHostAgentServer.ts:63`).
- `_reconnectionGraceTime: number` — derived from `environmentService.reconnectionGraceTime` (`remoteExtensionHostAgentServer.ts:102`).

Constructor (`remoteExtensionHostAgentServer.ts:72–103`):
- Strips trailing slash from `serverBasePath`.
- Computes `_serverProductPath` via `getServerProductSegment(productService)`.
- Conditionally creates `WebClientServer` only when `hasWebClient` is true.

**`handleRequest` (line 105–190)**:
- Only `GET` requests are accepted; others return 405.
- Strips `serverBasePath` and `serverProductPath` prefixes from `pathname`.
- `/version` → returns `productService.commit` (200, text/plain).
- `/delay-shutdown` → calls `_serverLifetimeService.delay()`.
- All other paths require a valid connection token (checked at line 144 via `httpRequestHasValidConnectionToken`); 403 if absent.
- `/vscode-remote-resource` — serves local files (extensions, workspace assets) with CORS headers for web worker extension host origins. Cache-Control `public, max-age=31536000` when path is within extensions dir (`remoteExtensionHostAgentServer.ts:166–170`).
- All remaining paths are delegated to `WebClientServer.handle(...)` if present, or 404 otherwise.

**`handleUpgrade` (line 192–221)**:
- Reads `reconnectionToken`, `reconnection`, `skipWebSocketFrames` from query string.
- Calls `upgradeToISocket(req, socket, { debugLabel, skipWebSocketFrames, disableWebSocketCompression })` to wrap the raw `net.Socket` into an `ISocket`.
- Passes the upgraded socket to `_handleWebSocketConnection`.

**`_handleWebSocketConnection` (line 258–381)**:
- Creates a `PersistentProtocol` over the upgraded socket.
- Optionally creates `vsda.validator` and `vsda.signer` instances from the native `vsda` module.
- Implements a three-state machine via local `enum State { WaitingForAuth, WaitingForConnectionType, Done, Error }`.
- **State 0 (`WaitingForAuth`)**: waits for a JSON control message with `type: 'auth'`. Validates token if `ServerConnectionTokenType.Mandatory`. Responds with a `SignRequest` message containing `data` (a UUID for the client to sign) and `signedData` (the signer's output of the client's `data` field).
- **State 1 (`WaitingForConnectionType`)**: receives a `connectionType` message. Checks `rendererCommit === myCommit` for built-mode version gating. Validates `msg2.signedData` via `validator.validate()`. On new connection, shortens grace time of all existing connections. Transitions to `_handleConnectionType`.

**`_handleConnectionType` (line 384–499)**:
- Dispatches on `msg.desiredConnectionType`:
  - `ConnectionType.Management (1)`: creates `ManagementConnection`, feeds it to `_socketServer.acceptConnection(con.protocol, con.onClose)`. Reconnection resumes via `managementConnection.acceptReconnection(...)`.
  - `ConnectionType.ExtensionHost (2)`: calls `_updateWithFreeDebugPort(startParams0)` to resolve a free debug port, then creates `ExtensionHostConnection` via the instantiation service. The connection token is registered in `_extHostLifetimeTokens` to keep the server alive. Reconnect resumes via `extHostConnection.acceptReconnection(...)`.
  - `ConnectionType.Tunnel (3)`: calls `_createTunnel` which reads remaining buffered bytes, connects a local TCP socket to `tunnelStartParams.{host,port}`, and pipes the raw `net.Socket` bidirectionally (`remoteExtensionHostAgentServer.ts:514–521`).

**`createServer` factory (line 577–771)**:
- Awaits `determineServerConnectionToken(args)`. If parse error, exits process.
- Installs a SIGPIPE handler to prevent infinite error loops.
- Calls `setupServerServices(connectionToken, args, REMOTE_DATA_FOLDER, disposables)` to build the DI container.
- Loads the optional native `vsda` module from `node_modules/vsda` if present (`remoteExtensionHostAgentServer.ts:675–686`).
- Prints Web UI URL to stdout if web client is present and address is a TCP address.
- Emits performance telemetry (`serverStart` event) with four timestamps: `startTime`, `startedTime`, `codeLoadedTime`, `readyTime`.
- Returns the constructed `RemoteExtensionHostAgentServer` instance.

**`WebEndpointOriginChecker` (line 773–813)**:
- Builds a regex from `productService.webEndpointUrlTemplate` by substituting UUID, commit, quality. Used to allow CORS from web worker extension host origins.

---

#### 5. `src/vs/server/node/serverServices.ts`

**Role**: Assembles the server's dependency injection container and registers all IPC channels on the `SocketServer`.

**`setupServerServices` (line 107–301)**:

Services registered in order:
1. `IProductService` (`serverServices.ts:111`).
2. `IEnvironmentService` / `INativeEnvironmentService` → `ServerEnvironmentService` (`serverServices.ts:114–116`).
3. `ILoggerService` → `LoggerService`; registers a `logger` channel on `socketServer` (`serverServices.ts:118–120`).
4. `ILogService` → `LogService` composed with `ServerLogger` (colored stdout) (`serverServices.ts:123`).
5. `IFileService` → `FileService` + `DiskFileSystemProvider` for `file:` scheme (`serverServices.ts:141–143`).
6. `IUriIdentityService` → `UriIdentityService` (`serverServices.ts:146`).
7. `IConfigurationService` → `ConfigurationService` backed by `machineSettingsResource` (`serverServices.ts:150`).
8. `IUserDataProfilesService` → `ServerUserDataProfilesService` + `userDataProfiles` channel (`serverServices.ts:154–156`).
9. Machine IDs resolved in parallel: `getMachineId`, `getSqmMachineId`, `getDevDeviceId` + `configurationService.initialize()` + `userDataProfilesService.init()` (`serverServices.ts:162–168`).
10. `IRequestService` → `RequestService` (`serverServices.ts:174`).
11. Telemetry: conditionally `ServerTelemetryService` with `OneDataSystemAppender` (ariaKey from `productService.aiConfig`) or `ServerNullTelemetryService` (`serverServices.ts:177–205`).
12. Extension gallery, MCP gallery, download channel client, extension scanning, signing, management (`serverServices.ts:207–219`).
13. `IPtyService` → `PtyHostService` over `NodePtyHostStarter` with reconnection grace time and scrollback from config (`serverServices.ts:225–234`).
14. `IServerLifetimeService` → `ServerLifetimeService` with `enableAutoShutdown` + `shutdownWithoutDelay` flags (`serverServices.ts:236–240`).
15. Optional `ServerAgentHostManager` when `--agent-host-port` or `--agent-host-path` is provided (`serverServices.ts:242–251`).
16. MCP management / gallery / resource scanner services (`serverServices.ts:253–256`).

Channel registrations (`serverServices.ts:258–298`):
- `'remoteextensionsenvironment'` → `RemoteAgentEnvironmentChannel`
- `'telemetry'` → `ServerTelemetryChannel`
- `SANDBOX_HELPER_CHANNEL_NAME` → `SandboxHelperChannel`
- `REMOTE_TERMINAL_CHANNEL_NAME` → `RemoteTerminalChannel` (wraps `PtyHostService`)
- `RemoteExtensionsScannerChannelName` → `RemoteExtensionsScannerChannel`
- `NativeMcpDiscoveryHelperChannelName`, `McpGatewayChannelName`
- `REMOTE_FILE_SYSTEM_CHANNEL_NAME` → `RemoteAgentFileSystemProviderChannel`
- `'request'` → `RequestChannel`
- `'extensions'` → `ExtensionManagementChannel`
- `'mcpManagement'` → `McpManagementChannel`

**`SocketServer<TContext>` (line 312–325)**: Extends `IPCServer<TContext>`. Has an internal `Emitter<ClientConnectionEvent>`. `acceptConnection(protocol, onDidClientDisconnect)` fires this emitter, triggering `IPCServer` to register a new client.

**`getUriTransformer` (line 303–309)**: Lazily creates and caches `IURITransformer` per `remoteAuthority`. Used by all channel factories to rewrite URIs crossing the local/remote boundary.

---

#### 6. `src/vs/server/node/serverConnectionToken.ts`

**Role**: Defines the two connection token types and the resolution logic.

**Types**:
- `NoneServerConnectionToken` (`serverConnectionToken.ts:24`): `validate()` always returns `true`. Used with `--without-connection-token`.
- `MandatoryServerConnectionToken` (`serverConnectionToken.ts:32`): stores a string `value`; `validate()` compares by strict equality.

**`determineServerConnectionToken` (line 93–121)**:
- Delegates to `parseServerConnectionToken` with a `defaultValue` async factory.
- Default factory: reads `<user-data-dir>/token` file; validates with regex `[0-9A-Za-z_-]+`; if absent or invalid, generates a UUID via `generateUuid()` and writes it to the file at mode `0o600` (`serverConnectionToken.ts:113–118`).

**`requestHasValidConnectionToken` (line 123–132)**:
- First checks `connectionTokenQueryName` URL query parameter.
- Falls back to `connectionTokenCookieName` cookie.

---

#### 7. `src/vs/server/node/extensionHostConnection.ts`

**Role**: Manages the full lifecycle of a single remote extension host process, including spawning, socket handoff, reconnection, and teardown.

**`buildUserEnvironment` (line 26–71)**:
- Resolves NLS config for the given `language`.
- Optionally merges user shell environment via `getResolvedShellEnv`.
- Sets `VSCODE_ESM_ENTRYPOINT=vs/workbench/api/node/extensionHostProcess`, `VSCODE_HANDLES_UNCAUGHT_ERRORS`, `VSCODE_NLS_CONFIG`.
- Prepends `remote-cli` bin folder to `PATH` so `code` command works in terminal.
- Sets `BROWSER` to a shell helper script.
- Sets `VSCODE_RECONNECTION_GRACE_TIME` (`extensionHostConnection.ts:66`).
- Calls `removeNulls(env)` to strip any null-valued overrides from `startParamsEnv`.

**`ConnectionData` (line 73–108)**:
- Holds `(socket: NodeSocket | WebSocketNodeSocket, initialDataChunk: VSBuffer)`.
- `toIExtHostSocketMessage()` produces an `IExtHostSocketMessage` with socket framing metadata (`skipWebSocketFrames`, `permessageDeflate`, `inflateBytes` all base64-encoded) for `process.send()` to the child process.

**`ExtensionHostConnection` class (line 110)**:
- `_canSendSocket: boolean` — `false` on Windows with `--socket-path` (`extensionHostConnection.ts:132`).
- `start(startParams)` (line 247–335):
  1. Strips existing `--inspect` args from `process.execArgv`.
  2. Adds `--inspect[=brk]=<port> --experimental-network-inspection` if debug port requested.
  3. Calls `buildUserEnvironment`.
  4. Chooses socket transport: if `_canSendSocket`, writes `SocketExtHostConnection()` to env; otherwise creates a named pipe server and writes `IPCExtHostConnection(pipeName)` to env.
  5. Forks `bootstrap-fork` with args `['--type=extensionHost', '--transformURIs', '--useHostProxy=...']` (`extensionHostConnection.ts:288`).
  6. Captures `stdout`/`stderr` as log events.
  7. Waits for `VSCODE_EXTHOST_IPC_READY` message then calls `_sendSocketToExtensionHost` (or for named pipe, waits for the pipe connection).
- `_sendSocketToExtensionHost` (line 190–201): drains the socket, serialises the `IExtHostSocketMessage`, and sends via `extensionHostProcess.send(msg, rawSocket)` — Node's built-in IPC socket-passing mechanism.
- `acceptReconnection` (line 213–228): updates `_remoteAddress`, creates a new `ConnectionData`, and if the process is already running, immediately calls `_sendSocketToExtensionHost`.
- `shortenReconnectionGraceTimeIfNecessary` (line 203–211): sends `VSCODE_EXTHOST_IPC_REDUCE_GRACE_TIME` to the child process.
- `_cleanResources` (line 230–245): ends the socket, kills the child process, fires `onClose`.

---

#### 8. `src/vs/server/node/serverEnvironmentService.ts`

**Role**: Defines the full CLI option schema (`serverOptions`) and the `ServerEnvironmentService` class.

**`serverOptions` (line 18–104)**: Complete `OptionDescriptions<ServerParsedArgs>` object covering:
- Server binding: `host`, `port`, `socket-path`, `server-base-path`
- Auth: `connection-token`, `connection-token-file`, `without-connection-token`
- Remote dev: `enable-remote-auto-shutdown`, `reconnection-grace-time`, `agent-host-port`, `agent-host-path`
- Extension management: `install-extension`, `uninstall-extension`, `list-extensions`, etc.

**`ServerEnvironmentService` (line 247–257)**:
- Extends `NativeEnvironmentService`.
- `userRoamingDataHome` is overridden to equal `appSettingsHome`.
- `machineSettingsResource` → `<userDataPath>/Machine/settings.json`.
- `mcpResource` → `<userDataPath>/User/mcp.json`.
- `reconnectionGraceTime` → `parseGraceTime(args['reconnection-grace-time'], ProtocolConstants.ReconnectionGraceTime)` at `serverEnvironmentService.ts:256`.

**`parseGraceTime` (line 259–276)**: Parses a seconds string to milliseconds; clamps illegal values to the `fallback`.

---

#### 9. `src/vs/server/node/webClientServer.ts` (partial)

**Role**: Serves the browser workbench HTML and static assets.

- `serveError` (line 43–46): writes `errorCode` header + plain-text body.
- `serveFile` (line 55–110): stats file; handles ETags (weak validator from `ino+size+mtime`), `NO_EXPIRY` (`max-age=31536000`), `NO_CACHING`. Pipes via `createReadStream` after the `open` event fires to avoid sending a 200 header before confirming the file exists.
- Path constants: `STATIC_PATH = '/static'`, `CALLBACK_PATH = '/callback'`, `WEB_EXTENSION_PATH = '/web-extension-resource'` (`webClientServer.ts:114–116`).
- `WebClientServer` class starts at line 118.

---

#### 10. `src/vs/platform/remote/common/remoteAgentConnection.ts` (partial)

**Role**: Defines the wire protocol types shared between client and server.

- `ConnectionType` enum (line 26–30): `Management = 1`, `ExtensionHost = 2`, `Tunnel = 3`.
- Handshake message union (line 72): `HandshakeMessage = AuthRequest | SignRequest | ConnectionTypeRequest | ErrorMessage | OKMessage`.
- `AuthRequest` (line 43–47): `{ type: 'auth', auth: string, data: string }` — client sends token + random bytes.
- `SignRequest` (line 49–53): `{ type: 'sign', data: string, signedData: string }` — server replies with data for client to sign back.
- `ConnectionTypeRequest` (line 55–61): `{ type: 'connectionType', commit?, signedData, desiredConnectionType?, args? }` — client declares intent.
- `RECONNECT_TIMEOUT = 30 * 1000` (line 24).

---

### Cross-Cutting Synthesis

The VS Code remote server is structured as a thin HTTP/WebSocket layer (`server-main.ts`) that deliberately starts listening before any heavyweight modules are loaded. `loadCode` dynamically imports `server.main.ts` only after port binding, deferring ESM bootstrap, NLS loading, and directory provisioning to the first connection. Once loaded, `createServer` (in `remoteExtensionHostAgentServer.ts`) assembles a DI container (`serverServices.ts`) containing roughly 20 platform services (file system, terminal pty, extension management, telemetry, MCP gateway, etc.), each exposed as a named IPC channel on the `SocketServer`. Every WebSocket connection goes through a three-message handshake (auth → sign → connectionType) before being promoted into one of three connection types: `Management` (IPC RPC bus), `ExtensionHost` (spawns a child process via `cp.fork` and passes the raw socket to it via Node's built-in socket passing), or `Tunnel` (raw TCP pipe). Reconnection is tracked per token in `_extHostConnections`/`_managementConnections` dictionaries, with a configurable grace-time window. The `serverConnectionToken` module provides auth at both the HTTP level (query param or cookie) and the WebSocket handshake level.

**For a Tauri/Rust port**, the key boundaries are:
- The `IServerAPI` interface (`remoteExtensionHostAgentServer.ts:558–575`) — the three methods `handleRequest`, `handleUpgrade`, `handleServerError` that bridge the raw HTTP server to the server logic.
- The `SocketServer.acceptConnection` path (`serverServices.ts:322–324`) — where a protocol is injected into the IPCServer.
- The `cp.fork` + `process.send(msg, socket)` in `extensionHostConnection.ts:288, 200` — Node-specific socket passing that would need a Rust equivalent (Unix socket FD passing or named pipe).
- `PersistentProtocol` over `NodeSocket`/`WebSocketNodeSocket` — the framing layer that would need reimplementation.
- The DI service tree in `setupServerServices` — the complete list of services the extension host server depends on.

---

### Out-of-Partition References

The following files are central to the server stack but are in other partitions:

- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/parts/ipc/common/ipc.net.ts` — `PersistentProtocol`, `ISocket`, `ChunkStream`, `ProtocolConstants`.
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/parts/ipc/node/ipc.net.ts` — `NodeSocket`, `WebSocketNodeSocket`, `upgradeToISocket`, `createRandomIPCHandle`.
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/parts/ipc/common/ipc.ts` — `IPCServer`, `IMessagePassingProtocol`, `ClientConnectionEvent`, `StaticRouter`.
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/remoteExtensionManagement.ts` — `ManagementConnection` (used at `remoteExtensionHostAgentServer.ts:424`).
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/webClientServer.ts` — `WebClientServer.handle` (full implementation beyond line 120).
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/remoteExtensionHostAgentCli.ts` — `runCli` used by `spawnCli`.
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/terminal/node/ptyHostService.ts` and `nodePtyHostStarter.ts` — pty host lifecycle.
- `/home/norinlavaee/projects/vscode-atomic/src/vs/server/node/serverLifetimeService.ts` — `IServerLifetimeService` auto-shutdown logic.
- `/home/norinlavaee/projects/vscode-atomic/src/vs/workbench/services/extensions/common/extensionHostEnv.ts` — `SocketExtHostConnection`, `IPCExtHostConnection`, `writeExtHostConnection`.
- `/home/norinlavaee/projects/vscode-atomic/src/vs/workbench/services/extensions/common/extensionHostProtocol.ts` — `IExtHostReadyMessage`, `IExtHostSocketMessage`, `IExtHostReduceGraceTimeMessage`.
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts` — ESM module resolution bootstrapping called by `loadCode`.
