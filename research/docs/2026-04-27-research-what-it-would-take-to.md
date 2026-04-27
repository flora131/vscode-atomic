---
date: 2026-04-27 09:01:24 PDT
researcher: deep-research-codebase workflow
git_commit: cb297c54fcc51f62fba0b1a3cd692a7cacdce68a
branch: flora131/feature/vscode-2-electric-boogaloo
repository: vscode-atomic
topic: "Research what it would take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust."
tags: [research, codebase, deep-research]
status: complete
last_updated: 2026-04-27
---

# Research: Porting VS Code's Core IDE Functionality from TypeScript/Electron to Tauri/Rust

## Research Question
Research what it would take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

## Executive Summary

VS Code is a TypeScript-first monorepo at ~3.07M LOC across ~10,580 source files. The IDE itself lives almost entirely under `src/vs/` (5,906 files, 1.92M LOC) plus first-party extensions under `extensions/`. The runtime is overwhelmingly TypeScript executing inside Electron (Chromium renderer + Node.js main/utility processes), with a small but significant existing Rust footprint at `cli/` (70 files, 18,723 LOC) used today only for the launcher and the dev-tunnels relay. A port to Tauri/Rust is therefore not a single migration but a coordinated replacement of three independent layers: (a) the Electron host (windowing, custom protocols, menus, IPC, native dialogs, crash reporter, sandbox preload, utility-process spawning), (b) the Node.js services backing the workbench (file system, watcher, ripgrep, PTY, debug-adapter spawn, askpass IPC), and (c) the build/bundling system that today produces AMD-and-ESM-mixed bundles via gulp + esbuild.

The public extension API in `src/vscode-dts/vscode.d.ts` (21,233 LOC, 15 namespaces) plus 167 proposed-API files (~33,096 LOC total) is the load-bearing contract: the workbench-side code under `src/vs/workbench/api/` materializes every namespace into IPC messages that travel between the workbench renderer and the extension host process. Any port must preserve this surface byte-for-byte to keep the extension ecosystem viable; the `extensions/vscode-api-tests/` partition (49 files, 11,374 LOC, ~1,479 test cases) is the precise contract to be re-satisfied.

A small but important amount of Tauri-bridge work has already landed in this repo. `extensions/git/out/git/tauriProcessLauncher.js`, `extensions/git/out/git/processLauncher.factory.js`, and `extensions/git/out/tauri-shell.js` (compiled artifacts present in source control) implement an abstraction over Node `child_process` that delegates to `@tauri-apps/plugin-shell` when `globalThis.__TAURI_INTERNALS__` is set; the existing `cli/src/rpc.rs`, `cli/src/async_pipe.rs`, and `cli/src/state.rs` provide a transport-agnostic RPC framework, cross-platform Unix-socket/Named-pipe abstraction, and JSON-on-disk state container that map directly onto Tauri command/event/state primitives. The architecture is therefore not greenfield: a layered port that swaps process management and IPC at well-defined seams (the `IUtilityProcessConfiguration` boundary, the `process.send`/`ipc.cp.ts` boundary, the `vscode-file://`/`vscode-webview://` boundary) is feasible.

The deep work — replacing Monaco's editor model and view, the extension host RPC engine (`src/vs/workbench/services/extensions/common/rpcProtocol.ts`), the debug service, the SCM subsystem, the integrated terminal's PTY-host/xterm.js stack, search/ripgrep, and the notebook execution state machine — remains TypeScript-resident regardless of which host wraps the renderer. A pragmatic Tauri/Rust port therefore looks like: keep TypeScript for the workbench, replace Electron with Tauri+wry as the chrome and process supervisor, replace Node.js services with Rust services exposed via Tauri commands, and progressively rewrite hot paths (file watching, search, PTY, file I/O) in Rust behind the existing service interfaces.

## Detailed Findings

### Core Architecture (`src/vs/`) — Partition 1

**Source-code organization (`src/vs/`, 5,906 files, 1.92M LOC):**
- `src/vs/base/` — utilities, async primitives, `Disposable` lifecycle, observables.
- `src/vs/platform/` — DI services (`createDecorator`), files, log, telemetry, configuration, IPC.
- `src/vs/editor/` — Monaco: `src/vs/editor/common/model.ts` (`ITextModel`), `src/vs/editor/common/languages.ts`, `src/vs/editor/common/languageFeatureRegistry.ts`, `src/vs/editor/browser/services/codeEditorService.ts`.
- `src/vs/workbench/` — UI shell, parts, panes, contrib features.
- `src/vs/code/` — Electron main + renderer entry points (`src/vs/code/electron-main/main.ts`, `src/vs/code/electron-main/app.ts`, `src/vs/code/electron-browser/workbench/workbench.ts`).
- `src/vs/server/` — remote/web server (`src/vs/server/node/server.main.ts`, `src/vs/server/node/serverServices.ts`, `src/vs/server/node/remoteExtensionHostAgentServer.ts`).

**Dependency Injection.** `src/vs/platform/instantiation/common/instantiation.ts:109-126` defines `createDecorator<T>(serviceId)` which produces a typed identifier that doubles as parameter decorator and registry key. Services extend `Disposable` and declare injected dependencies as decorated constructor parameters — see `src/vs/platform/secrets/common/secrets.ts:101-120` (`BaseSecretStorageService` with four injected services). `src/vs/code/electron-main/app.ts` instantiates ~100 services via `ServiceCollection`. The `_serviceBrand: undefined` phantom property is enforced by `.eslint-plugin-local/code-declare-service-brand.ts`.

**Lifecycle.** `src/vs/base/common/lifecycle.ts:526-557` defines `Disposable` with a `_store: DisposableStore` and `_register<T extends IDisposable>(o: T): T`. `DisposableStore` (lines 416-485) tracks children for cascade cleanup with parent/child wiring and a `setParentOfDisposable` graph used by leak detection. The `code-must-use-super-dispose.ts` and `code-no-potentially-unsafe-disposables.ts` ESLint rules enforce safe usage.

**IPC layer.** 20+ files under `src/vs/base/parts/ipc/`:
- `src/vs/base/parts/ipc/common/ipc.ts` — `IChannel`, `IMessagePassingProtocol`, `IServer`.
- `src/vs/base/parts/ipc/electron-main/ipcMain.ts:13-81` — `ValidatedIpcMain` wraps Electron `ipcMain.on/once/handle` with sender validation.
- `src/vs/base/parts/ipc/electron-browser/ipc.electron.ts` and `ipc.mp.ts` — renderer side.
- `src/vs/base/parts/ipc/node/ipc.cp.ts` — child-process IPC; `Server` class wraps `process.send` (base64-encoded buffers) and `process.on('message')` events. The 21–23 line comment notes the base64 encoding is a perf concern with intent to migrate to named sockets.
- `src/vs/base/parts/ipc/node/ipc.net.ts` — Unix domain socket / TCP IPC.
- `src/vs/base/parts/ipc/electron-main/ipc.mp.ts` — MessagePort-based IPC.

**RPC for the extension host.** `src/vs/workbench/services/extensions/common/rpcProtocol.ts` defines the binary protocol; `src/vs/workbench/api/common/extHost.protocol.ts` defines `MainThread*`/`ExtHost*` `ProxyIdentifier`s for every namespace (commands, languages, terminal, debug, scm, notebooks, tasks, fs, workspace, lm, chat, tests, authentication).

**Process couplings (`process.*`)** are pervasive: every file path that reads `process.execPath`, `process.platform`, `process.env`, `process.argv`, `process.cwd()` is a porting touchpoint. `src/vs/code/node/cli.ts:44-88` builds a child-process spawn with cloned `process.env`, deletes `ELECTRON_RUN_AS_NODE`, and conditionally invokes `cargo run` (dev) or the bundled tunnel binary.

### Public Extension API (`src/vscode-dts/`) — Partition 6

The single file `src/vscode-dts/vscode.d.ts` (21,233 LOC) is the entire stable contract between host and extensions. 15 namespaces, with line numbers for each:

- `tasks` (`vscode.d.ts:9347`), `env` (`10739`), `commands` (`10973`), `window` (`11069`), `workspace` (`13797`), `languages` (`14722`), `notebooks` (`16350`), `scm` (`16652`), `debug` (`17283`), `extensions` (`17458`), `authentication` (`18091`), `l10n` (`18192`), `tests` (`18271`), `chat` (`20111`), `lm` (`20732`).

Foundational types declared once and used everywhere across the boundary:
- `Uri` (`vscode.d.ts:1439`) — five-component value `(scheme, authority, path, query, fragment)`; factories `Uri.parse`, `Uri.file`, `Uri.joinPath`, `Uri.from` (lines 1454–1531).
- `Position` (`vscode.d.ts:269`) and `Range` (`vscode.d.ts:408`) — UTF-16-code-unit-indexed.
- `Disposable` (`vscode.d.ts:1712`).
- `Event<T>` (`vscode.d.ts:1755`) — callable subscription returning `Disposable`.
- `CancellationToken` (`vscode.d.ts:1659`) — every async provider call receives one.
- `ProviderResult<T>` (`vscode.d.ts:2450`) — `T | undefined | null | Thenable<T | undefined | null>`.
- `TextDocument` (`vscode.d.ts:88`), `FileSystem` (`vscode.d.ts:9774`).

Pattern-uniformity across all namespaces:
1. **Registration** — extension calls `register*` or `create*`, gets a `Disposable`. Host stores provider keyed by string ID and extension identity.
2. **Provider call** — host invokes a method on the registered provider with structured value types and `CancellationToken`. Provider returns `ProviderResult<T>`.
3. **Event push** — host fires a typed `Event<T>` to all subscribers.

The 167 proposed-API files (`src/vscode-dts/vscode.proposed.*.d.ts`) augment existing namespaces with `declare module 'vscode'` blocks; they introduce no new IPC patterns but add provider/event surface — chat (14 files, ~6,000 LOC), notebooks (10), terminal (8), SCM (8), language model (6), search (8), UI contribs (28).

### Existing Rust Precedent (`cli/`) — Partition 9

`cli/` (70 files, 18,723 LOC) is the most directly applicable Rust precedent:
- `cli/Cargo.toml` — `tokio = { features = ["full"] }`, `serde`, `serde_json`, `rmp-serde`, `reqwest`, `hyper 0.14`, `clap (derive)`, `keyring 2.0.3` (macOS Keychain / Win Credential Manager / Linux libsecret), `tunnels` from `microsoft/dev-tunnels` git, `cfg-if`, `pin-project`, `opentelemetry`, `async-trait`.
- `cli/src/bin/code/main.rs` — `#[tokio::main]` entry; argument dispatch via `clap`; `CommandContext` struct (`reqwest::Client`, `LauncherPaths`, `Logger`, `args`) is the DI carrier — directly maps to Tauri `State<T>`.
- `cli/src/rpc.rs` — generic transport-agnostic RPC framework. `Serialization` trait (lines 42–45) decouples encoding from framing; `JsonRpcSerializer` and `MsgPackSerializer` both implement it. Method types: `SyncMethod`, `AsyncMethod = Arc<dyn Send + Sync + Fn(...) -> BoxFuture<'static, ...>>`, `Duplex` (returns `(Option<StreamDto>, BoxFuture)`). `RpcBuilder<S>` / `RpcCaller<S>` / `RpcDispatcher<S, C>` builder pattern at lines 49–301. `tokio::spawn` for write-loop management; `oneshot::Receiver` for call results (lines 342–381).
- `cli/src/json_rpc.rs` — newline-delimited JSON framing; `tokio::select!` over read/write/shutdown.
- `cli/src/msgpack_rpc.rs` — length-prefixed msgpack via `tokio_util::codec::Decoder`.
- `cli/src/async_pipe.rs` — cross-platform: `cfg_if!` selects `tokio::net::UnixStream` on Unix or `#[pin_project]` `enum AsyncPipe { NamedPipeClient, NamedPipeServer }` on Windows. `AsyncRWAccepter` trait abstracts over both pipe and TCP listeners.
- `cli/src/state.rs` — `PersistedState<T>` generic JSON-on-disk container with `Mutex` lock, Unix `mode 0o600`, `LauncherPaths` for cache directories.
- `cli/src/auth.rs` — OAuth device flow for Microsoft/GitHub; `StorageImplementation` trait with `KeyringStorage`, `FileStorage`, `ThreadKeyringStorage` (runs blocking keyring on dedicated thread with 5s timeout).
- `cli/src/tunnels/control_server.rs` (25+ async fns, 8+ `tokio::spawn`) — main message dispatcher; per-connection `tokio::spawn`; `tokio::select!` over shutdown, port-forwarding, agent-host connections (served via `hyper::server::conn::Http`), and control-port incoming.

The cli's Rust footprint already includes ~44 `tokio::spawn` call-sites, OpenTelemetry tracing, OAuth device flow, encrypted credential persistence, and platform service managers (`service_linux.rs` via zbus systemd, `service_macos.rs` via launchd, `service_windows.rs` via Windows Service). These are reusable as the foundation of any Rust core for VS Code.

### Electron Main Entry — Partitions 38 & 1

`src/main.ts` (741 LOC) is the synchronous-then-async Electron bootstrap. Key Electron-specific calls (all need Tauri equivalents):

- `protocol.registerSchemesAsPrivileged([...])` (`src/main.ts:96-105`) for schemes `vscode-webview` and `vscode-file` with privileges `{standard, secure, supportFetchAPI, corsEnabled, allowServiceWorkers, codeCache}`.
- `app.enableSandbox()` (`src/main.ts:46`), `app.disableHardwareAcceleration()`, `app.commandLine.appendSwitch('disable-gpu-sandbox')`.
- `app.setPath('userData', userDataPath)` (`src/main.ts:64`), `app.setPath('crashDumps', ...)`, `app.setAppLogsPath(...)`.
- `crashReporter.start({...})` (`src/main.ts:531`) with platform/arch-specific AppCenter URLs.
- `Menu.setApplicationMenu(null)` (`src/main.ts:70`).
- `app.once('ready', ...)` (`src/main.ts:147`); `app.on('open-file', ...)` (macOS file drop, `src/main.ts:597`); `app.on('open-url', ...)` (vscode:// protocol, `src/main.ts:613`); `app.on('will-finish-launching', ...)` (`src/main.ts:612`).
- `contentTracing.startRecording(...)` (`src/main.ts:179`).

Custom protocol handlers in the renderer (`src/vs/platform/protocol/electron-main/protocolMainService.ts`) call `defaultSession.protocol.registerFileProtocol(vscode-file://)` and `defaultSession.protocol.interceptFileProtocol(file://)` to map URIs to validated filesystem paths and block raw `file://` access.

The wider electron-main service constellation (46+ files) covers: `windowsMainService.ts`, `lifecycleMainService.ts`, `nativeHostMainService.ts`, `dialogMainService.ts`, `electronUrlListener.ts`, `webviewMainService.ts`, `webviewProtocolProvider.ts`, `menubarMainService.ts`, `storageMainService.ts`, `backup.ts`, `updateService.{darwin,linux,win32,snap}.ts`, `macOSCrossAppSecretSharing.ts`, `extensionHostStarter.ts`, `electronPtyHostStarter.ts`, `utilityProcess.ts`, `sharedProcess.ts`, `diagnosticsMainService.ts`, `extensionHostDebugIpc.ts`, `windowProfiling.ts`, `diskFileSystemProviderServer.ts`, `launchMainService.ts`, `workspacesMainService.ts`, `protocolMainService.ts`, `auxiliaryWindowsMainService.ts`, `crossAppIpcService.ts`, `loggerService.ts`, `themeMainService.ts`, etc.

### Bootstrap Sequence — Partitions 51, 52, 55, 67

VS Code uses a layered bootstrap with strict load order:

1. `src/bootstrap-cli.ts` — single side effect: `delete process.env.VSCODE_CWD` to prevent cwd leakage.
2. `src/bootstrap-meta.ts` (55 LOC) — sentinel `BUILD_INSERT_PRODUCT_CONFIGURATION` replaced inline at build time; out-of-source falls through to `require('../product.json')` and `require('../package.json')`. Embedded-app and dev-overrides merging via `Object.assign`.
3. `src/bootstrap-node.ts` (190 LOC) — `Error.stackTraceLimit = 100`; SIGPIPE one-shot guard (`bootstrap-node.ts:17-30`); `setupCurrentWorkingDirectory()` writes `VSCODE_CWD` and on Windows calls `process.chdir(path.dirname(process.execPath))`; `removeGlobalNodeJsModuleLookupPaths()` (lines 76-128) monkey-patches `Module._resolveLookupPaths` and `Module._nodeModulePaths` to strip global paths and Windows drive-root/home-dir paths; `devInjectNodeModuleLookupPath(injectPath)` (lines 62-74) calls `Module.register('./bootstrap-import.js', ...)` to install an ESM loader hook; `configurePortable(product)` (lines 133-190) resolves portable data path and sets `VSCODE_PORTABLE`/`TMP`/`TEMP`/`TMPDIR`.
4. `src/bootstrap-esm.ts` (113 LOC) — registers an in-memory ESM loader (lines 14-30) that intercepts `import 'fs'` and redirects to `node:original-fs` (avoids Electron's patched `fs`); `doSetupNLS()` reads `VSCODE_NLS_CONFIG`, loads localized messages via `fs.promises.readFile`, stores in `globalThis._VSCODE_NLS_MESSAGES`; `bootstrapESM()` is the exported async entry.
5. `src/bootstrap-fork.ts` (229 LOC) — runs inside every forked utility/extension/PTY/watcher child. `pipeLoggingToParent()` (lines 14-154): `safeSend()` wraps `process.send()` with try/catch; `wrapConsoleMethod()` uses `Object.defineProperty` to replace `console.log/warn/info/error` with closures that send `{type: '__$console', severity, arguments}` envelopes; `wrapStream()` similarly intercepts `process.stdout.write`/`process.stderr.write` with line buffering. `terminateWhenParentTerminates()` (lines 169-181) polls `process.kill(VSCODE_PARENT_PID, 0)` every 5s — parent supervision via signal-0 existence check. `handleExceptions()` registers `uncaughtException` and `unhandledRejection` handlers. Final dynamic `await import('./'+process.env.VSCODE_ESM_ENTRYPOINT+'.js')` loads the actual worker.
6. `src/cli.ts` (26 LOC) — top-level CLI orchestrator: imports `bootstrap-cli.js` (side-effect first), calls `resolveNLSConfiguration({userLocale: 'en', osLocale: 'en', commit: product.commit})`, sets `VSCODE_NLS_CONFIG`, calls `configurePortable(product)`, sets `VSCODE_CLI=1`, awaits `bootstrapESM()`, dynamic imports `./vs/code/node/cli.js` which calls `main(process.argv)`.
7. `src/vs/code/node/cli.ts` (~611 LOC) — full CLI: dispatches `tunnel`/`serve-web`/install-extension/etc.; spawns `cargo run -- subcommand` (dev) or the bundled tunnel binary.

### Extension-Host RPC and Workbench API Implementation — Partitions 1, 6, 11

The bridge between extensions and the workbench lives at `src/vs/workbench/api/`:
- `src/vs/workbench/api/common/extHost.protocol.ts` — RPC method definitions (`MainThreadCommands`, `ExtHostLanguageFeatures`, `MainThreadDebugService`, etc.).
- `src/vs/workbench/api/common/extHost.api.impl.ts` — TypeScript implementation of the `vscode` namespace that extensions import.
- `src/vs/workbench/api/common/extHostCommands.ts`, `extHostLanguageFeatures.ts`, `extHostDebugService.ts`, `extHostNotebook.ts`, `extHostLanguageModels.ts`, `extHostTerminalService.ts`, `extHostFileSystem.ts`, `extHostWorkspace.ts`, `extHostTask.ts`.
- `src/vs/workbench/api/browser/mainThreadLanguageFeatures.ts` and siblings — workbench-side stubs that receive RPC calls.
- `src/vs/workbench/services/extensions/common/rpcProtocol.ts` — the binary IPC engine.
- `src/vs/workbench/services/extensions/common/extensionHostProtocol.ts` — protocol definition.
- `src/vs/workbench/services/extensions/electron-browser/localProcessExtensionHost.ts` — Electron utility-process spawner; sets `VSCODE_ESM_ENTRYPOINT`, `VSCODE_PIPE_LOGGING`, `VSCODE_PARENT_PID`, `VSCODE_HANDLES_UNCAUGHT_ERRORS`.

The extension-host child process is spawned via Electron's utility-process API (`src/vs/platform/utilityProcess/electron-main/utilityProcess.ts`); the `IUtilityProcessConfiguration` contract (lines 22-85) carries `entryPoint`, `env`, `parentLifecycleBound: number` (parent PID for supervision), `execArgv`. `IWindowUtilityProcessConfiguration` (lines 87-110) adds MessagePort-based renderer↔utility-process communication: `responseWindowId`, `responseChannel`, `responseNonce`, `windowLifecycleBound`, `windowLifecycleGraceTime`.

### Architectural Invariants — Partition 19

`.eslint-plugin-local/` (49 files, 3,952 LOC) encodes the architecture as enforceable rules. The most load-bearing are:

- `.eslint-plugin-local/code-import-patterns.ts:42-279` — defines 7 layers `common → worker → browser → electron-browser → node → electron-utility → electron-main` and the dependency graph between them, with platform-conditional restrictions (`when: 'hasBrowser' | 'hasNode' | 'hasElectron' | 'test'`). Enforces `.js` or `.css` extensions on relative imports; forbids absolute `vs/` imports.
- `.eslint-plugin-local/code-layering.ts:15-92` — layer-based dirname matching against an `.eslintrc` configuration map.
- `.eslint-plugin-local/code-no-static-node-module-import.ts` — requires `await import(...)` for Node modules to keep startup fast.
- `.eslint-plugin-local/code-amd-node-module.ts:25-56` — reads `package.json` deps at lint time; npm packages must be loaded via `amdX#importAMDNodeModule`.
- `.eslint-plugin-local/code-must-use-super-dispose.ts` — overridden `dispose()` must call `super.dispose()`.
- `.eslint-plugin-local/code-no-potentially-unsafe-disposables.ts` — `DisposableStore`/`MutableDisposable` must be `const`/`readonly`.
- `.eslint-plugin-local/code-no-accessor-after-await.ts`, `code-no-reader-after-await.ts` — AST flow analysis: `ServicesAccessor` and reactive `reader` parameters become invalid across `await`.
- `.eslint-plugin-local/code-no-observable-get-in-reactive-context.ts` — observable `.get()` forbidden inside `derived`/`autorun`; must use `.read(reader)`.
- `.eslint-plugin-local/code-declare-service-brand.ts:16-26` — `_serviceBrand` property must be `declare ... : undefined` (no value).
- 9 `vscode-dts-*.ts` rules — public API conventions: `IInterface` prefix forbidden (`vscode-dts-interface-naming.ts:21-35`), events must be `onX`, providers must use `provideX`/`resolveX`, prefer `Thenable<T>` over `Promise<T>`, `CancellationToken` patterns.

### Test Contract — Partition 11

`extensions/vscode-api-tests/` (49 files, 11,374 LOC, ~1,479 test cases) is the behavioral contract:
- `extensions/vscode-api-tests/src/singlefolder-tests/workspace.test.ts` (1,500 LOC), `window.test.ts` (1,063), `terminal.test.ts` (985), `notebook.kernel.test.ts` (469), `tree.test.ts` (455), `workspace.tasks.test.ts` (451), `chat.runInTerminal.test.ts` (426), `quickInput.test.ts` (389).
- `extensions/vscode-api-tests/src/utils.ts:90` — `assertNoRpc` walks the entire `vscode` namespace looking for `Symbol.for('rpcProxy')` / `Symbol.for('rpcProtocol')` markers, failing if any live RPC objects leak between tests. This makes explicit that every `vscode.*` object is an IPC proxy.
- `extensions/vscode-api-tests/src/memfs.ts` — `TestFS` implements the full `vscode.FileSystemProvider` for in-memory tests; provides `stat/readDirectory/readFile/writeFile/rename/delete/createDirectory` plus `onDidChangeFile` with 5ms event coalescing.

A Tauri/Rust port must satisfy all 1,479 test cases unchanged.

### TypeScript Language Features (LSP Pattern) — Partition 8

`extensions/typescript-language-features/` (168 files, 22,571 LOC) is the flagship LSP integration and the model for porting language intelligence:

- `extensions/typescript-language-features/src/typescriptServiceClient.ts` — `ServerState` enum (None/Running/Errored). `execute()` (line 858), `executeWithoutWaitingForResponse()` (line 914), `executeAsync()` (line 922) are the three flavors; all delegate to private `executeImpl()` which calls `bufferSyncSupport.beforeCommand(command)` then `serverState.server.executeImpl(...)`.
- `extensions/typescript-language-features/src/tsServer/server.ts` — `ITypeScriptServer` interface (lines 39-55); `TsServerProcess` interface (lines 80-88) with `write`, `onData`, `onExit`, `onError`, `kill`. `SingleTsServer.dispatchMessage()` (lines 147-185) routes `'response'` via `request_seq` lookup; `'event'` with `requestCompleted` resolves matching callback. `RequestRouter` (lines 389-471) routes shared commands (`change`, `close`, `open`, `updateOpen`, `configure`) to all servers but returns the first result. `SyntaxRoutingTsServer` (lines 547-685) splits requests by command kind: `syntaxAlwaysCommands`, `semanticCommands`, `syntaxAllowedCommands` (upgrade after `projectLoadingFinish`).
- `extensions/typescript-language-features/src/tsServer/serverProcess.electron.ts` — `ProtocolBuffer` (lines 34-98) parses `Content-Length: N\r\n\r\n` framed JSON. `IpcChildServerProcess` (lines 215-271) uses `child_process.send`/`process.on('message')`. `StdioChildServerProcess` (lines 273-340) uses `JSON.stringify(req) + '\r\n'` to stdin, reads from stdout via `Reader`. `ElectronServiceProcessFactory.fork()` (lines 342-387) chooses `child_process.fork(tsServerPath, args, {silent: true, stdio: useIpc ? ['pipe','pipe','pipe','ipc'] : undefined})`.
- `extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts` — `WorkerServerProcess`: spawns Web Worker with three `MessageChannel` pairs (`_tsserver` for RPC, `_watcher` for file events, `_syncFs` via `@vscode/sync-api-service`).
- `extensions/typescript-language-features/src/tsServer/requestQueue.ts` — three priorities (`Normal=1`, `LowPriority=2`, `Fence=3`); `Normal` items insert before the last contiguous `LowPriority` block; `Fence` (`change`/`close`/`open`/`updateOpen`) appends to tail to preserve mutation ordering.
- `extensions/typescript-language-features/src/tsServer/bufferSyncSupport.ts` — `BufferSynchronizer` batches open/close/change ops; `flush()` produces one `updateOpen` RPC. `GetErrRequest` for diagnostics; `interruptGetErr()` cancels in-flight `geterr` to prioritize interactive requests.
- `extensions/typescript-language-features/src/typeConverters.ts` — coordinate translation: tsserver 1-based ↔ VS Code 0-based.
- `extensions/typescript-language-features/src/languageProvider.ts:64-100` — `Promise.all([import('./languageFeatures/...')...])` lazy-loads ~25 feature modules; each exports `register(selector, client, ...)`.

### Source Control / Git — Partition 7

`extensions/git/` (62 files, 25,181 LOC) is the SCM API reference implementation, and notably **already has Tauri-bridge code in source control**:

- `extensions/git/out/git/processLauncher.factory.js:15-17` — `isTauriHost()` returns `typeof globalThis.__TAURI_INTERNALS__ !== 'undefined'`; `createProcessLauncher()` returns `new TauriGitProcessLauncher()` or `new NodeGitProcessLauncher()`.
- `extensions/git/out/git/tauriProcessLauncher.js:127-163` — `TauriGitProcessLauncher.spawn(command, args, options)` constructs a `TauriGitChild` and kicks off `_spawnAsync` which dynamically requires `@tauri-apps/plugin-shell`, calls `shellPlugin.Command.create(command, args, {cwd, env})`, wires `tauriCommand.stdout.on('data')` / `.stderr.on('data')` / `.on('close')` / `.on('error')`, and calls `tauriCommand.spawn()`. The `TauriGitChild` class exposes `TauriReadableStream` for stdout/stderr, `stdin = null`, `pid = undefined`, and `kill()` calling `_tauriChild.kill()`.
- `extensions/git/out/tauri-shell.js:103-173` — `tauriExec` (maps to `cp.exec/execFile` via `Command.create(...).execute()`) and `tauriSpawn` (maps to `cp.spawn` for streaming with `TauriReadable`/`MiniEmitter`).
- `extensions/git/out/git/nodeProcessLauncher.js` — the alternative Node implementation.
- `extensions/git/out/test/{tauriShell,processLauncher}.test.js` — tests for both.

Source-side equivalents: `extensions/git/src/git.ts:9` `import * as cp from 'child_process'`; `git.ts:203-208` `SpawnOptions extends cp.SpawnOptions` adding `input?`, `log?`, `cancellationToken?`, and `onSpawn?: (childProcess: cp.ChildProcess) => void`. `Git._exec()` (lines 618-673) calls `this.spawn()` then runs `options.onSpawn?.(child)` at line 621. `Git.spawn()` (lines 676-703) builds env merging `process.env` + extension env + per-call overrides + `VSCODE_GIT_COMMAND`/locale/`GIT_PAGER`, then calls `cp.spawn(this.path, args, options)`.

Askpass IPC (out-of-process credential prompts):
- `extensions/git/src/ipc/ipcServer.ts:15-25` — `getIPCHandlePath(id)`: Windows `\\.\pipe\vscode-git-${id}-sock`; Linux with `XDG_RUNTIME_DIR`: `$XDG_RUNTIME_DIR/vscode-git-${id}.sock`; else `os.tmpdir()/vscode-git-${id}.sock`. `createIPCServer()` uses `http.createServer` over the Unix socket.
- `extensions/git/src/askpass.ts` — `Askpass implements IIPCHandler, ITerminalEnvironmentProvider`. Sets `GIT_ASKPASS`, `VSCODE_GIT_ASKPASS_NODE = process.execPath`, `VSCODE_GIT_ASKPASS_MAIN = askpass-main.js`, `SSH_ASKPASS`, `SSH_ASKPASS_REQUIRE='force'`. `handle(payload)` dispatches to `handleAskpass` (HTTPS user/pass via `window.showInputBox`) or `handleSSHAskpass` (passphrase / host authenticity).
- `extensions/git/src/askpass-main.ts` — separate Node process; reads `VSCODE_GIT_IPC_HANDLE`, calls `IPCClient.call({askpassType, argv})`, writes result to `VSCODE_GIT_ASKPASS_PIPE` via `fs.writeFileSync`.

`extensions/git/src/repository.ts:984` — `scm.createSourceControl('git', ...)` is the SCM API entry.

### Authentication / Secrets — Partition 21

`extensions/microsoft-authentication/` (31 files, 3,561 LOC) demonstrates the OAuth + secrets stack:
- `MsalAuthProvider` (`src/node/authProvider.ts:39`) implements `vscode.AuthenticationProvider` and is registered via `authentication.registerAuthenticationProvider('microsoft', 'Microsoft', authProvider, ...)` (`extension.ts:118`).
- `CachedPublicClientApplication` (`src/node/cachedPublicClientApplication.ts:16`) wraps `@azure/msal-node`'s `PublicClientApplication`; `SecretStorageCachePlugin` (`src/common/cachePlugin.ts`) adapts MSAL's cache to VS Code's `SecretStorage`.
- Three OAuth flows (`src/node/flows.ts`):
  - `DefaultLoopbackFlow` — MSAL's built-in localhost server; opens browser via `env.openExternal`.
  - `UrlHandlerFlow` — uses `UriHandlerLoopbackClient` (`src/common/loopbackClientAndOpener.ts`) with `DEFAULT_REDIRECT_URI = 'https://vscode.dev/redirect'`; relies on `vscode.window.registerUriHandler` to receive the callback.
  - `DeviceCodeFlow` — fallback for headless/portable.
- `UriEventHandler.ts` (`src/UriEventHandler.ts:8`) extends `vscode.EventEmitter<vscode.Uri>` and implements `vscode.UriHandler`; OS routes `vscode://vscode.microsoft-authentication?code=...&state=...` through it.
- `BetterTokenStorage<T>` (`src/betterSecretStorage.ts:15`) maintains a serialized `Map<string, T>` over `SecretStorage` with cross-window change events.
- `extensions/microsoft-authentication/packageMocks/{keytar,dpapi}/` — mock native modules for the keychain and Windows DPAPI.

Underlying secret storage is backed by `keytar` (Node native module) on the workbench side, surfacing via `context.secrets` on the extension side.

### Markdown, Webviews, Notebook Renderers — Partitions 14, 22, 17

- `extensions/markdown-language-features/src/preview/preview.ts` and `previewManager.ts` exemplify `vscode.WebviewPanel` use: `window.createWebviewPanel`, `webview.html = ...`, `webview.postMessage`, `webview.onDidReceiveMessage`. Preview-side counterpart at `extensions/markdown-language-features/preview-src/messaging.ts` uses `acquireVsCodeApi()`/`postMessage`. CSP injection at `preview-src/csp.ts` and `src/preview/security.ts`. Language-server client at `src/client/client.ts` uses `vscode-languageclient`.
- `extensions/notebook-renderers/src/index.ts` (639 LOC) is a webview-loaded renderer (`renderer-out/index.js`) using native DOM APIs (`document.createElement`, `innerHTML`, `classList`). MIME dispatch table at lines 543-609 routes `image/*` → `renderImage`, `text/html` → `renderHTML` (trusted only), `application/javascript` → `renderJavascript` (trusted only), `application/vnd.code.notebook.error` → `renderError`, etc. `htmlHelper.ts` creates a `TrustedTypePolicy` wrapper. The `disposables: Map<string, IDisposable>` keyed by `outputInfo.id` ensures cleanup.
- `extensions/ipynb/` — Jupyter notebook serializer. `notebookSerializer.ts` implements `vscode.NotebookSerializer` (`deserialize`/`serialize` with `Uint8Array`). Platform-specific variants `notebookSerializer.node.ts` and `notebookSerializer.web.ts` use Node `worker_threads` and Web Workers respectively for parallel serialization. Registered at `ipynbMain.ts` via `workspace.registerNotebookSerializer('jupyter-notebook', ...)`.

### Terminal — Partitions 1, 4

- Workbench side: `src/vs/workbench/contrib/terminal/browser/terminalProcessManager.ts`, `electron-browser/localPty.ts`, `electron-browser/terminalProfileResolverService.ts`. Backing PTY runs in a separate process via `src/vs/platform/terminal/electron-main/electronPtyHostStarter.ts` and `src/vs/platform/terminal/node/terminalProcess.ts` (uses node-pty native module).
- Extension API: `vscode.window.createTerminal`, `Pseudoterminal` (extension-owned PTY with `onDidWrite: Event<string>`, `open()`, `close()`), `EnvironmentVariableCollection` with `replace`/`append`/`prepend` and `getScoped(scope)` for per-folder mutations.
- `extensions/terminal-suggest/src/terminalSuggestMain.ts:261-327` registers via `vscode.window.registerTerminalCompletionProvider({async provideTerminalCompletions(terminal, terminalContext, token) {...}}, '/', '\\')`. `TerminalShellType` covers Bash, Zsh, Fish, PowerShell, WindowsPowerShell, GitBash. 84 declarative Fig command specs + custom shell-builtin enumeration via `getBashGlobals/getZshGlobals/getFishGlobals/getPwshGlobals` cached in `globalState` per `(machineId, remoteAuthority, shellType)` with 7-day TTL.

### Server / Remote — Partition 50

`src/server-main.ts` (285 LOC) is the remote/web entry:
- Order-sensitive bootstrap: `import './bootstrap-server.js'` first (deletes `ELECTRON_RUN_AS_NODE` to prevent `fs` redirection); then `bootstrap-node`, `bootstrap-esm`, `bootstrap-meta`.
- `minimist` argument parsing (lines 26-38); env overrides via `VSCODE_SERVER_HOST`, `VSCODE_SERVER_PORT`, etc.
- CLI vs server branching (lines 43-51) — `--help`/`--version`/extension management without `--start-server` triggers `spawnCli()`.
- HTTP server at lines 88-104: `http.createServer(handleRequest)`, `server.on('upgrade', handleUpgrade)`, `server.on('error', handleServerError)`. All three handlers `await getRemoteExtensionHostAgentServer()` (lazily memoized).
- `IServerAPI` interface (`remoteExtensionHostAgentServer.ts:558-575`): `handleRequest(req, res): Promise<void>`, `handleUpgrade(req, socket): void`, `handleServerError(err): void`, `dispose(): void`. The comment "Do not remove!! Called from server-main.js" explicitly marks this as the stable contract.
- Port resolution (lines 170-225): single port, range (`8000-9000`), or `findFreePort()` iterating until `http.createServer` succeeds.
- Performance marks: `code/server/start`, `code/server/firstRequest`, `code/server/firstWebSocket`, `code/server/started`.

Workbench-side server services include `IConfigurationService`, `IFileService`, `IRequestService`, `IPtyService`, `IExtensionManagementService`, `ILanguagePackService`, telemetry channels, user profile services, extension scanner, logger channels — see `src/vs/server/node/serverServices.ts`.

### Build System — Partition 5

`build/` (195 files, 33,416 LOC):
- `build/gulpfile.ts`, `gulpfile.compile.ts`, `gulpfile.vscode.ts`, `gulpfile.vscode.web.ts`, `gulpfile.extensions.ts`, `gulpfile.cli.ts`, `gulpfile.reh.ts`, `gulpfile.editor.ts` define top-level tasks.
- `build/lib/bundle.ts` — AMD bundling. `build/lib/optimize.ts` — treeshaking + mangling. `build/lib/treeshaking.ts`, `build/lib/mangle/{index,renameWorker,staticLanguageServiceHost}.ts` use the TS LSP for symbol analysis.
- `build/lib/tsb/{builder,index,transpiler,utils}.ts` — custom TypeScript batch builder for module-format conversion.
- `build/lib/nls.ts`, `build/lib/i18n.ts` — NLS/i18n integration; messages are stripped from source and replaced with array indices, looked up at runtime via `globalThis._VSCODE_NLS_MESSAGES`.
- `build/darwin/{sign,sign-server,create-dmg,create-universal-app,verify-macho}.ts` — macOS signing/packaging.
- `build/win32/explorer-dll-fetcher.ts`, `build/linux/{libcxx-fetcher,dependencies-generator}.ts`, `build/linux/{debian,rpm}/{calculate-deps,dep-lists}.ts`.
- `build/lib/policies/*.ts` — group-policy ADMX/ADML codegen (Windows enterprise).
- **Experimental next-gen bundlers already present**: `build/vite/{vite.config,index,setup-dev,index-workbench,workbench-electron}.ts`, `build/next/{index,nls-plugin,private-to-property}.ts`, `build/rspack/rspack.serve-out.config.mts` and `workbench-rspack.html`. The team is actively prototyping non-AMD bundling.

### Ambient Type Declarations — Partition 44

`src/typings/` (9 files, 430 LOC) declares the runtime contract:
- `electron-cross-app-ipc.d.ts` (62 LOC) — `Electron.CrossAppIPC` for code-signed inter-app IPC (macOS Mach ports / Windows named pipes); used by `src/vs/platform/secrets/electron-main/macOSCrossAppSecretSharing.ts`, `src/vs/platform/update/electron-main/crossAppUpdateIpc.ts`, `src/vs/platform/crossAppIpc/electron-main/crossAppIpcService.ts`.
- `vscode-globals-product.d.ts` (48 LOC) — `_VSCODE_FILE_ROOT`, `_VSCODE_CSS_LOAD`, `_VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON`, `_VSCODE_DISABLE_CSS_IMPORT_MAP`, `_VSCODE_USE_RELATIVE_IMPORTS`.
- `vscode-globals-nls.d.ts` (41 LOC) — `_VSCODE_NLS_MESSAGES: string[]`, `_VSCODE_NLS_LANGUAGE: string | undefined`.
- `vscode-globals-ttp.d.ts` (15 LOC) — Trusted Types policy.
- `base-common.d.ts` (41 LOC) — `requestIdleCallback`, `setTimeout`/`clearTimeout` (with nominal `Timeout` type), `Error.captureStackTrace`/`stackTraceLimit`.
- `crypto.d.ts` (84 LOC) — partial Web Crypto: `SubtleCrypto.digest`, `Crypto.getRandomValues`, `Crypto.randomUUID`.
- `editContext.d.ts` (124 LOC) — experimental W3C EditContext API.
- `css.d.ts` (10 LOC) — `declare module "vs/css!*"` for the AMD CSS loader.
- `thenable.d.ts` (13 LOC) — `Thenable<T> extends PromiseLike<T>`.

## Architecture & Patterns

Cross-cutting patterns observed across the partitions:

**1. Process supervision via parent-PID polling.** Every forked Node child (`src/bootstrap-fork.ts:169-181`) polls `process.kill(VSCODE_PARENT_PID, 0)` every 5s to self-exit when the parent dies. Equivalent in Rust would be `prctl(PR_SET_PDEATHSIG, SIGTERM)` on Linux, Job Objects on Windows, and `kqueue(EVFILT_PROC, NOTE_EXIT)` on macOS — all OS-level rather than polling.

**2. Console interception for IPC.** `bootstrap-fork.ts:105-153` replaces `console.log/warn/info/error` and `process.stdout.write`/`process.stderr.write` with closures that send `{type: '__$console', severity, arguments}` envelopes via `process.send()`. Verbose-logging gate strips `log/warn/info` to no-ops by default. Equivalent in Rust: `tracing::Subscriber` writing to a Tauri command channel.

**3. Base64-over-stdin/stdout IPC framing.** `src/vs/base/parts/ipc/node/ipc.cp.ts:29-32` base64-encodes `VSBuffer` payloads. Comment at line 21-23 documents intent to migrate to named sockets (`ipc.net`).

**4. RPC proxy markers.** Every `vscode.*` object exposed to extensions carries `Symbol.for('rpcProxy')` / `Symbol.for('rpcProtocol')` markers; `extensions/vscode-api-tests/src/utils.ts:90-130` walks the namespace verifying no proxies leak across test boundaries.

**5. Provider-pattern RPC contract.** Every namespace follows the same shape: extension calls `register*` returning `Disposable`; host invokes provider with `(documentlike, position-or-range, CancellationToken)` returning `ProviderResult<T>`; events are typed `Event<T>` callable subscriptions returning `Disposable`.

**6. `IDisposable._register()` for resource graph.** Every service extends `Disposable` and pipes child resources through `_register()` so the entire dependency tree disposes in reverse-order on parent dispose. Cycle detection at `lifecycle.ts:setParentOfDisposable`.

**7. Lazy module initialization.** `Promise.all([import('./featureA'), import('./featureB'), ...])` is the dominant pattern in extensions (`extensions/typescript-language-features/src/languageProvider.ts:64-100`, `extensions/markdown-language-features/src/extension.shared.ts`). The `code-no-static-node-module-import.ts` ESLint rule enforces this for performance.

**8. Platform-conditional file naming.** Files like `serverProcess.electron.ts` / `serverProcess.browser.ts`, `configuration.electron.ts` / `configuration.browser.ts`, `extension.ts` / `extension.browser.ts` provide parallel implementations. The build picks the variant; the `.eslint-plugin-local/code-import-patterns.ts` layer rules enforce platform-correct deps.

**9. Single utility-process pattern.** Extension host, PTY host, watcher host, telemetry host, shared process all use the same `IUtilityProcessConfiguration` (`src/vs/platform/utilityProcess/electron-main/utilityProcess.ts:22-85`) with `entryPoint`, `env`, `parentLifecycleBound`, `responseChannel` (when bound to a window). `bootstrap-fork.ts` is the universal entry running inside each.

**10. Existing Tauri shim layer in `extensions/git/out/`.** Active engineering work demonstrates the chosen abstraction strategy: a feature-detected runtime selector (`isTauriHost()` via `globalThis.__TAURI_INTERNALS__`) chooses between `NodeGitProcessLauncher` and `TauriGitProcessLauncher`; the latter delegates to `@tauri-apps/plugin-shell`'s `Command.create(...).spawn()` and synthesizes Node-shaped `ChildProcess` interfaces (`stdin: null`, `pid: undefined`, `kill()`, `on('exit'|'error')`, `TauriReadable` streams).

## Code References

- `src/main.ts:46` — `app.enableSandbox()` gate.
- `src/main.ts:96-105` — `protocol.registerSchemesAsPrivileged([{vscode-webview, vscode-file}])`.
- `src/main.ts:147-211` — `app.once('ready', onReady)`; `startup()` dynamic `import('./vs/code/electron-main/main.js')`.
- `src/main.ts:589-621` — macOS `open-file`/`will-finish-launching`/`open-url` global event buffering.
- `src/bootstrap-fork.ts:14-154` — `pipeLoggingToParent`: console + stream interception.
- `src/bootstrap-fork.ts:169-181` — `terminateWhenParentTerminates` parent-PID polling.
- `src/bootstrap-fork.ts:229` — dynamic `import('./'+VSCODE_ESM_ENTRYPOINT+'.js')`.
- `src/bootstrap-node.ts:62-74` — `devInjectNodeModuleLookupPath` ESM loader hook via `Module.register`.
- `src/bootstrap-node.ts:76-128` — `removeGlobalNodeJsModuleLookupPaths` patches `Module._resolveLookupPaths` and `Module._nodeModulePaths`.
- `src/bootstrap-esm.ts:14-30` — `fs` → `node:original-fs` ESM hook for Electron contexts.
- `src/cli.ts:13-26` — synchronous bootstrap chain + dynamic CLI import.
- `src/server-main.ts:88-108` — `http.createServer` + `'upgrade'` + `'error'` lazy delegation to `IServerAPI`.
- `src/vscode-dts/vscode.d.ts:9347/10739/10973/11069/13797/14722/16350/16652/17283/17458/18091/18192/18271/20111/20732` — namespace start lines.
- `src/vs/platform/instantiation/common/instantiation.ts:109-126` — `createDecorator<T>`.
- `src/vs/base/common/lifecycle.ts:526-557` — `Disposable` base class.
- `src/vs/base/common/lifecycle.ts:416-485` — `DisposableStore`.
- `src/vs/base/parts/ipc/electron-main/ipcMain.ts:13-81` — `ValidatedIpcMain`.
- `src/vs/base/parts/ipc/node/ipc.cp.ts:24-37` — child-process IPC server with base64 framing.
- `src/vs/platform/utilityProcess/electron-main/utilityProcess.ts:22-110` — `IUtilityProcessConfiguration` and window-bound variant.
- `src/vs/code/electron-main/app.ts` — central service-collection assembly (~100 services).
- `src/vs/server/node/remoteExtensionHostAgentServer.ts:558-575` — `IServerAPI` contract.
- `cli/src/rpc.rs:42-301` — `Serialization` trait + `RpcBuilder<S>` + method registry.
- `cli/src/async_pipe.rs:17-177` — cross-platform pipe `cfg_if!`.
- `cli/src/state.rs:89-134` — `PersistedState<T>` JSON-on-disk container.
- `cli/src/auth.rs:188-285` — `StorageImplementation` trait with thread-isolated keyring.
- `extensions/git/out/git/processLauncher.factory.js:15-25` — `isTauriHost()` runtime dispatch.
- `extensions/git/out/git/tauriProcessLauncher.js:127-163` — `TauriGitProcessLauncher.spawn` via `@tauri-apps/plugin-shell`.
- `extensions/git/out/tauri-shell.js:103-173` — `tauriExec` and `tauriSpawn` Node-shaped wrappers.
- `extensions/git/src/git.ts:203-208` — `SpawnOptions extends cp.SpawnOptions` with `onSpawn`.
- `extensions/git/src/git.ts:618-703` — `Git._exec` and `Git.spawn` with env merging.
- `extensions/git/src/ipc/ipcServer.ts:15-25` — askpass IPC socket-path computation.
- `extensions/git/src/repository.ts:984` — `scm.createSourceControl('git', ...)`.
- `extensions/typescript-language-features/src/tsServer/server.ts:147-185` — `dispatchMessage` response routing.
- `extensions/typescript-language-features/src/tsServer/serverProcess.electron.ts:34-98` — `ProtocolBuffer` `Content-Length` framing.
- `extensions/typescript-language-features/src/tsServer/serverProcess.electron.ts:342-387` — `ElectronServiceProcessFactory.fork`.
- `extensions/typescript-language-features/src/tsServer/serverProcess.browser.ts:61-187` — Web Worker tsserver with three `MessageChannel`s.
- `extensions/microsoft-authentication/src/node/cachedPublicClientApplication.ts:39-66` — MSAL + `SecretStorageCachePlugin` + native broker plugin wiring.
- `extensions/microsoft-authentication/src/UriEventHandler.ts:8-15` — VS Code URI-handler for OAuth deep links.
- `extensions/notebook-renderers/src/index.ts:419-638` — notebook renderer activation + MIME dispatch + experimental hooks.
- `extensions/vscode-api-tests/src/utils.ts:90-130` — `assertNoRpc` proxy-leak detector.
- `.eslint-plugin-local/code-import-patterns.ts:42-279` — 7-layer architectural rule.
- `.eslint-plugin-local/code-no-static-node-module-import.ts` — startup-perf lazy-import enforcement.
- `.eslint-plugin-local/code-must-use-super-dispose.ts` — disposal chain enforcement.

## Historical Context (from research/)

The historical-research locator confirmed that `research/docs/`, `research/tickets/`, `specs/`, `adr/`, and `rfc/` directories contain no prior research documents on this topic. There are no recorded ADRs, RFCs, or tickets investigating a Tauri/Rust port. The single most informative historical signal is in the source tree itself: the compiled `extensions/git/out/git/tauriProcessLauncher.js`, `extensions/git/out/git/nodeProcessLauncher.js`, `extensions/git/out/git/processLauncher.factory.js`, and `extensions/git/out/tauri-shell.js` (with corresponding tests in `extensions/git/out/test/`) demonstrate that incremental Tauri-bridge work has been integrated into the extension tier, abstracting Node `child_process` behind a feature-detected factory. The branch name `flora131/feature/vscode-2-electric-boogaloo` and the absence of formal RFCs suggests the work is exploratory/in-progress rather than driven by a formal architecture document.

## Open Questions

- **Workbench-side language:** is the intent to keep `src/vs/workbench/` in TypeScript (running inside Tauri's wry webview) and replace only the Electron host/Node services with Rust, or to progressively rewrite workbench services in Rust behind the existing TS service interfaces? The existing `extensions/git/out/*.js` Tauri shim suggests the former.
- **Extension host residency:** does the extension host remain a Node.js process (spawned by Tauri's command/sidecar API) or move to a Rust-hosted V8 (e.g., `rusty_v8`/Deno embedding)? `bootstrap-fork.ts`'s `process.send`/`process.on('message')` plumbing assumes a Node parent.
- **Custom-protocol parity:** Tauri's `tauri::WebviewWindowBuilder::register_uri_scheme_protocol` must replicate Electron's `registerSchemesAsPrivileged` privileges (`standard, secure, supportFetchAPI, corsEnabled, allowServiceWorkers, codeCache`). The `vscode-webview://` scheme in particular underpins the entire webview extension API.
- **Native modules:** `keytar`, `node-pty`, `parcel-watcher`, `@vscode/spdlog`, `@vscode/sync-api-service`, native `vsce-sign`, native crash reporter integration — each requires a Rust replacement (`keyring-rs`, `portable-pty`, `notify`, `tracing-appender`) or a Node-side residency that Tauri spawns as a sidecar.
- **AMD-vs-ESM bundling:** the build system mixes AMD (`vs/css!*`) and ESM. The experimental `build/vite/`, `build/next/`, `build/rspack/` directories exist but are not yet primary. A Tauri port likely requires committing to one bundler.
- **`_VSCODE_*` build-time globals:** product/package metadata, NLS messages, file root, CSS loader, Trusted Types policy — all currently injected by the gulp/esbuild pipeline. Any Tauri port needs a parallel mechanism.
- **Multi-process IPC efficiency:** `ipc.cp.ts:29` notes the base64 encoding for `process.send` is a perf concern. Tauri's `invoke`/`emit` is JSON-serialized; for a large extension host RPC traffic profile (every keystroke for completions/diagnostics/inlay hints) this needs benchmarking, possibly with a custom Tauri IPC channel.
- **`context.secrets` cross-window sync:** `BetterTokenStorage` in microsoft-authentication uses `context.secrets.onDidChange` for cross-window sync. The OS keychain backend must support change notifications across processes; `keyring-rs` does not natively, so a notify channel layer would be needed.
- **macOS Keychain / Cross-app IPC:** `Electron.CrossAppIPC` (declared in `src/typings/electron-cross-app-ipc.d.ts` and used by `macOSCrossAppSecretSharing.ts` and `crossAppUpdateIpc.ts`) is a code-signed Mach-port channel between two Electron apps. Tauri has no equivalent — would need a Rust implementation using `mach2` or NSXPCConnection bindings.
- **Update service:** `updateService.{darwin,linux,win32,snap}.ts` integrate with Squirrel.Mac, Snap, and Windows installers via Electron's `autoUpdater`. Tauri's updater plugin handles signed differential updates differently and may not match the expected enterprise flows.
- **Test execution:** `extensions/vscode-api-tests/` runs against an Electron-driven extension host. The harness in `test/` would need rebuilding against a Tauri host.

## Methodology

Generated by the deep-research-codebase workflow with 79 partitions
covering 10,580 source files (3,073,605 LOC).
Each partition was investigated by four specialist sub-agents dispatched
directly via the provider SDK's native agent parameter:
codebase-locator, codebase-pattern-finder, codebase-analyzer, and
codebase-online-researcher. A separate research-history pipeline ran
codebase-research-locator → codebase-research-analyzer over the project's
prior research documents.
