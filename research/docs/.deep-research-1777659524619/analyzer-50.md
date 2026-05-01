### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/src/server-main.ts` (285 lines, read in full)

---

### Per-File Notes

#### `src/server-main.ts` — Headless Server Launcher (285 LOC)

**Module-level side effects (lines 6–23)**

The file's first import, at line 6, is `'./bootstrap-server.js'`, and the inline comment states it MUST come before other imports because it mutates global state. Immediately after all imports, line 22 records the performance mark `'code/server/start'`, and line 23 writes `globalThis.vscodeServerStartTime` with `performance.now()`.

**Argument Parsing (lines 26–38)**

`minimist` parses `process.argv.slice(2)` at line 26 into `parsedArgs`. Boolean flags declared at line 27 include `start-server`, `list-extensions`, `print-ip-address`, `help`, `version`, `accept-server-license-terms`, and `update-extensions`. String args at line 28 include `install-extension`, `install-builtin-extension`, `uninstall-extension`, `locate-extension`, `socket-path`, `host`, `port`, `compatibility`, `agent-host-port`, and `agent-host-path`. Aliases `help → h` and `version → v` are set at line 29.

Lines 31–38 apply an environment variable fallback for `host`, `port`, and `accept-server-license-terms`: for each, if the parsed arg is falsy, the code reads `process.env['VSCODE_SERVER_<ARG_UPPERCASE>']` and stores it into `parsedArgs`.

**CLI vs Server Routing (lines 40–152)**

Two arg-group arrays are defined at lines 40–41:
- `extensionLookupArgs`: `['list-extensions', 'locate-extension']`
- `extensionInstallArgs`: `['install-extension', 'install-builtin-extension', 'uninstall-extension', 'update-extensions']`

The boolean `shouldSpawnCli` at line 43 is `true` when `--help`, `--version`, any lookup arg is present, or any install arg is present without `--start-server`. NLS configuration is resolved at line 45 via `resolveNLSConfiguration` with a hardcoded `userLocale: 'en'`, `osLocale: 'en'`, `commit: product.commit`, empty `userDataPath`, and `import.meta.dirname` as `nlsMetadataPath`.

If `shouldSpawnCli` is true (line 47), `loadCode(nlsConfiguration)` is called and its returned module's `.spawnCli()` is invoked at line 49. The else branch (lines 51–152) runs the HTTP server path.

**Lazy Server Loading with Singleton Promise (lines 52–63)**

Two variables are declared: `_remoteExtensionHostAgentServer` (typed `IServerAPI | null`, initialised to `null`) at line 52, and `_remoteExtensionHostAgentServerPromise` (typed `Promise<IServerAPI> | null`) at line 53. The factory function `getRemoteExtensionHostAgentServer` at line 54 implements a singleton promise: if `_remoteExtensionHostAgentServerPromise` is null, it calls `loadCode(nlsConfiguration)`, awaits the dynamically imported module, calls `mod.createServer(address)` to obtain the `IServerAPI` instance, stores it in `_remoteExtensionHostAgentServer`, and caches the promise. Subsequent calls return the same promise.

**License Prompt (lines 65–82)**

If `product.serverLicense` is a non-empty array, its lines are printed to stdout at line 66. If `product.serverLicensePrompt` exists and `parsedArgs['accept-server-license-terms']` is not `true`, the code checks `hasStdinWithoutTty()` at line 68. In a non-TTY context (line 69–71), it prints an instruction and calls `process.exit(1)`. Otherwise, it calls `await prompt(product.serverLicensePrompt)` at line 73; if the user declines, `process.exit(1)` is called at line 75; errors also exit at line 79.

**HTTP Server Construction and Event Wiring (lines 84–108)**

Two one-shot flags `firstRequest` and `firstWebSocket` are declared `true` at lines 84–85. A variable `address` typed `string | AddressInfo | null` is declared `null` at line 87.

`http.createServer` is called at line 88 with an async request handler that: flips `firstRequest` and marks `'code/server/firstRequest'` (lines 89–92), then delegates every HTTP request to `remoteExtensionHostAgentServer.handleRequest(req, res)` at line 94.

The `'upgrade'` event (lines 96–104) similarly flips `firstWebSocket`, marks `'code/server/firstWebSocket'`, then delegates to `remoteExtensionHostAgentServer.handleUpgrade(req, socket)` at line 103. A TypeScript `@ts-expect-error` is placed at line 102 for the socket parameter.

The `'error'` event (lines 105–108) delegates to `remoteExtensionHostAgentServer.handleServerError(err)`.

**Listen Options and Binding (lines 110–144)**

`host` is resolved at line 110: the sanitized `parsedArgs['host']`, or `'localhost'` when compatibility mode is not `'1.63'`, or `undefined`. `nodeListenOptions` at lines 111–115 is either `{ path: sanitizeStringArg(parsedArgs['socket-path']) }` (Unix socket) or `{ host, port: await parsePort(host, sanitizeStringArg(parsedArgs['port'])) }` (TCP).

`server.listen(nodeListenOptions, callback)` is called at line 116. Inside the callback (lines 117–144):
- `product.serverGreeting` lines are formatted into `output` at line 117.
- If TCP and `--print-ip-address`, non-internal IPv4 addresses from `os.networkInterfaces()` are enumerated at lines 119–128 and appended.
- `address = server.address()` is stored at line 130; null address throws at line 132.
- Bound address is printed at line 135; line 137 prints the sentinel `Extension host agent listening on <port>` (comment at line 136 notes VS Code parses this exact string).
- Performance marks `'code/server/started'` and `globalThis.vscodeServerListenTime` are set at lines 140–141.
- `getRemoteExtensionHostAgentServer()` is eagerly awaited at line 143 to initialize the server before any request arrives.

**Dispose Lifecycle (lines 146–151)**

`process.on('exit', ...)` registered at line 146 calls `server.close()` and, if `_remoteExtensionHostAgentServer` is non-null, calls `_remoteExtensionHostAgentServer.dispose()`.

**`sanitizeStringArg` (lines 154–159)**

Handles the case where `minimist` produces an array when an argument is repeated: pops the last element. Returns `undefined` for non-string values.

**`parsePort` (lines 170–190)**

Async function receiving `host` and `strPort`. Logic:
1. If `strPort` matches `/^\d+$/` (line 173), returns `parseInt(strPort, 10)` directly.
2. Else if `parseRange(strPort)` returns a range object (line 175), calls `findFreePort(host, range.start, range.end)`. If no port found, prints the exact error message `--port: Could not find free port in range: ...` (line 181, noted as Remote-SSH API contract) and exits.
3. If neither, prints an invalid format warning at line 185 and exits.
4. Default is `8000` at line 189.

**`parseRange` (lines 192–200)**

Parses `strRange` with regex `/^(\d+)-(\d+)$/`. Validates `start > 0 && start <= end && end <= 65535`. Returns `{ start, end }` or `undefined`.

**`findFreePort` (lines 207–225)**

Iterates from `start` to `end` inclusive. For each `port`, creates a temporary `http.createServer()`, attempts `server.listen(port, host)`, resolves `true` on success (and immediately closes the test server), `false` on error. Returns first successful port, or `undefined` if none found.

**`loadCode` (lines 227–255)**

Sets `process.env['VSCODE_NLS_CONFIG']` to the JSON-serialized `nlsConfiguration` at line 230 (required for `bootstrap-esm` NLS pickup). Sets `process.env['VSCODE_HANDLES_SIGPIPE'] = 'true'` at line 236. In dev mode (`process.env['VSCODE_DEV']`, line 238), calls `devInjectNodeModuleLookupPath` with a path to `remote/node_modules` (line 242) to use Node-compiled modules rather than Electron-compiled ones. Calls `removeGlobalNodeJsModuleLookupPaths()` at line 248. Calls `await bootstrapESM()` at line 251. Returns the dynamic `import('./vs/server/node/server.main.js')` at line 254.

**`hasStdinWithoutTty` (lines 257–264)**

Returns `!process.stdin.isTTY`; wraps in try-catch for Windows Node.js bug (issue #11656).

**`prompt` (lines 266–285)**

Creates a `readline` interface on stdin/stdout. Presents the question, recursively calls itself for invalid responses. Returns a Promise resolving to `true` for empty string, `'y'`, or `'yes'`; `false` for `'n'` or `'no'`.

---

### Cross-Cutting Synthesis

`src/server-main.ts` is the top-level headless entry point for the VS Code remote server (code-server analogue). It performs a fast, synchronous argument parse with `minimist` before any expensive module loading, routing to either a CLI path (`spawnCli`) or an HTTP server path based on the detected flags. The HTTP server is created immediately and wired to three handlers (HTTP, WebSocket upgrade, error), but the actual `IServerAPI` implementation is loaded lazily on first use via a singleton promise that calls into the dynamically imported `vs/server/node/server.main.js`. Port binding supports three strategies: a literal TCP port, a scanned range (with an exact error message string treated as a Remote-SSH API contract), or a Unix socket path. The license gate is evaluated before listening, supporting both interactive TTY prompts and the `--accept-server-license-terms` flag for non-TTY automation. NLS and ESM bootstrap occur inside `loadCode` and are sequenced before the dynamic server import. The dispose lifecycle is registered on `process.exit` and calls both `http.Server.close()` and `IServerAPI.dispose()`. Performance marks bracket the full startup from `'code/server/start'` through `'code/server/started'`, with additional marks for first HTTP request and first WebSocket upgrade. A Rust/Tauri port must replicate: the minimist-equivalent argument parsing with env-variable fallbacks, the lazy singleton pattern for the extension host server, the exact sentinel string on line 137, the port-range scanning contract on line 181, and the three-handler HTTP delegation model that maps to `IServerAPI`.

---

### Out-of-Partition References

The following symbols are imported or dynamically loaded by `server-main.ts` but reside outside the single-file partition:

| Symbol / Module | Source Location | Usage Site |
|---|---|---|
| `bootstrap-server.js` (side-effect import) | `src/bootstrap-server.ts` (compiled) | `server-main.ts:6` — global state mutation before all other imports |
| `devInjectNodeModuleLookupPath`, `removeGlobalNodeJsModuleLookupPaths` | `src/bootstrap-node.ts` | `server-main.ts:14`, called at lines 242, 248 |
| `bootstrapESM` | `src/bootstrap-esm.ts` | `server-main.ts:15`, called at line 251 |
| `resolveNLSConfiguration` | `src/vs/base/node/nls.ts` | `server-main.ts:16`, called at line 45 |
| `product` | `src/bootstrap-meta.ts` | `server-main.ts:17`, `product.commit`, `product.serverLicense`, `product.serverLicensePrompt`, `product.serverGreeting` |
| `perf` (performance marking) | `src/vs/base/common/performance.ts` | `server-main.ts:18`, marks at lines 22, 92, 99, 140 |
| `INLSConfiguration` (type) | `src/vs/nls.ts` | `server-main.ts:19`, used as parameter type for `loadCode` and `resolveNLSConfiguration` return |
| `IServerAPI` (interface type) | `src/vs/server/node/remoteExtensionHostAgentServer.ts` | `server-main.ts:20`, type of `_remoteExtensionHostAgentServer`; methods `handleRequest`, `handleUpgrade`, `handleServerError`, `dispose` are delegated at lines 94, 103, 107, 149 |
| `vs/server/node/server.main.js` (dynamic import) | `src/vs/server/node/server.main.ts` | `server-main.ts:254`, exports `spawnCli()` and `createServer(address)` |
