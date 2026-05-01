---
date: 2026-05-01 18:35:43 UTC
researcher: deep-research-codebase workflow
git_commit: eaa498ee93bb3000d3d26cf964b872fd5e63fe91
branch: flora131/feature/vscode-final
repository: vscode-atomic
topic: "Research what it would take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust."
tags: [research, codebase, deep-research]
status: complete
last_updated: 2026-05-01
---

# Research: Porting VS Code Core IDE from TypeScript/Electron to Tauri/Rust

## Research Question
Research what it would take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

## Executive Summary

The VS Code monorepo at this commit comprises **3,107,132 LOC across 10,647 source files**, dominated by `src/vs/` (5,951 files / 1,943,130 LOC — the editor + workbench + platform core) and the `extensions/` tree (95 built-in extensions, ~960k LOC). A working **Rust foundation already exists** under `cli/` (75 files / 20,107 LOC) — tokio-based RPC (`cli/src/rpc.rs:49-91`), dev tunnels (`cli/src/tunnels/dev_tunnels.rs`), self-update, OAuth + keyring secret storage, cross-platform service installation — all reusable as the Tauri-host substrate.

The contract a Rust/Tauri port must preserve is the **public extension API** (`src/vscode-dts/vscode.d.ts` plus 169 proposed modules — ~2,000+ functions/properties/events), the **DI/lifecycle invariants** encoded in 49 custom ESLint rules (`.eslint-plugin-local/`), and **wire-level sentinels** parsed by external tools (e.g. `Extension host agent listening on <port>` at `src/server-main.ts:137`, parsed by Remote-SSH).

The **layering lattice** is the single largest structural constraint: `code-import-patterns.ts:19-279` defines a 7-layer dependency graph (`common → worker → browser → electron-browser` and `common → node → electron-utility → electron-main`) that every Rust crate split must reproduce. Disposable lifecycle (`Disposable._store` at `lifecycle.ts:526`) maps naturally to `Drop`; `Event<T>` and `Thenable<T>` need bespoke Rust analogs (channels + futures); `ServicesAccessor`/`reader`-after-`await` rules become borrow-checker concerns.

Three porting phases are evident from the evidence: (1) **already-Rust foundations** (the `cli/` crate, partitions 9/44/67) — collapse the desktop+server launchers, drop ~10K LOC of TS bootstrap shims (partitions 51, 52, 55, 57, 63, 75, 78); (2) **wire-protocol-portable subsystems** — LSP language servers (partitions 12/25/28), DAP debug clients (partitions 45/47), notebook renderers (partition 22), webview bridges (partitions 14/31/35/39); (3) **monolith re-implementation** — Monaco editor core, the workbench DI graph, the extension host RPC bridge (partition 1, ~1.94M LOC), and tsserver IPC (partition 8). The bulk of (3) is `src/vs/editor/` and `src/vs/workbench/api/` — the latter holds 80+ `mainThread*`/`extHost*` proxy pairs that are the spine of the extension host architecture.

The answer to "what would it take" is empirically: replace the Electron lifecycle (~700 LOC `src/main.ts` + Electron-coupled `vs/code/electron-main/`) with Tauri's setup hooks (≈2-4 weeks), preserve the bootstrap env-var/globals contract verbatim (`_VSCODE_PRODUCT_JSON`, `VSCODE_NLS_CONFIG`, etc.), encode the layering rules into Cargo crate visibility, and reimplement the DI container, Disposable lifecycle, IPC channel typing, and Monaco editor surface in Rust — preserving the 38-suite api-tests conformance corpus (`extensions/vscode-api-tests/`) as the gating test bed.

## Detailed Findings

### Core: `src/vs/` — Editor + Workbench + Platform (Partition 1)

**Subdirectories.** `base/` (utilities + IPC primitives), `platform/` (~100 service interfaces + DI), `editor/` (Monaco), `workbench/` (IDE shell + 200+ contributions under `contrib/<feature>/`), `code/` (Electron main + browser bootstrap), `server/` (remote-server), `sessions/` (chat session persistence).

**Dependency injection.**
- `src/vs/platform/instantiation/common/instantiation.ts:41` — `createDecorator<T>(serviceId)`
- `src/vs/platform/instantiation/common/extensions.ts:25` — `registerSingleton`
- `src/vs/platform/instantiation/common/instantiationService.ts:28-120` — `InstantiationService` (`ServiceCollection` at `:769`, `createChild` at `:793`, `invokeFunction` at `:809`, `createInstance` at `:835`)
- Service brand pattern: every service interface declares `readonly _serviceBrand: undefined`; constructor parameters annotated `@IServiceName` (e.g. `DebugService` ctor at `src/vs/workbench/contrib/debug/browser/debugService.ts:63-117`, `TerminalService` at `terminalService.ts:66-120`).
- Batch registration example: `src/vs/sessions/sessions.web.main.ts:109-127`.

**Lifecycle.**
- `src/vs/base/common/lifecycle.ts:526-557` — `Disposable` base (`_store: DisposableStore`, `_register()` cascade, `Disposable.None` at `:570`).
- `Emitter<T>` instances are routinely registered into the parent's `_store`.

**IPC primitives.**
- `src/vs/base/parts/ipc/common/ipc.ts` — base channel interface
- `src/vs/base/parts/ipc/electron-main/ipcMain.ts`, `electron-browser/ipc.electron.ts`, `common/ipc.net.ts` (TCP)
- `src/vs/base/common/jsonRpcProtocol.ts` — JSON-RPC base
- `src/vs/workbench/services/extensions/common/rpcProtocol.ts` — generic RPC proxy generator (used by extension host)
- Channel example: `src/vs/platform/download/common/downloadIpc.ts:12-42` (`DownloadServiceChannel`/`DownloadServiceChannelClient`); registration: `src/vs/code/electron-main/app.ts:1224-1231`.

**Editor core.**
- Models: `src/vs/editor/common/editorCommon.ts` (`IEditor`, `ITextModel`); core types in `core/{position,range,selection,editOperation}.ts` and `core/edits/`.
- Diff: `src/vs/editor/common/diff/linesDiffComputer.ts`.
- Languages: `src/vs/editor/common/languages.ts`, `languageFeatureRegistry.ts`, `languages/languageConfigurationRegistry.ts`.
- Provider aggregator: `src/vs/editor/common/services/languageFeatures.ts:10-81` (`ILanguageFeaturesService`).
- View: `src/vs/editor/browser/view.ts`; 15+ `viewParts/`; GPU rendering at `gpu/gpu.ts`, `gpu/atlas/textureAtlas.ts`.
- Native text input: `src/vs/editor/browser/controller/editContext/nativeEditContext.ts`.
- 200+ contributions: `editor/contrib/{suggest,parameterHints,rename,semanticTokens,codelens,find,codeAction,documentSymbols,...}/`.
- ~150 options at `src/vs/editor/common/config/editorOptions.ts`.

**Workbench shell & services.**
- Bootstraps: `src/vs/workbench/workbench.desktop.main.ts`, `workbench.web.main.ts`.
- Editor service: `services/editor/common/editorService.ts:17`.
- TextFiles: `services/textfile/common/textFileEditorModel.ts`, `textFileSaveParticipant.ts`, `encoding.ts`.
- Themes: `services/themes/common/{workbenchThemeService,colorThemeData,themeConfiguration}.ts`.
- Keybinding: `services/keybinding/common/{keybindingIO,macLinuxKeyboardMapper,windowsKeyboardMapper}.ts` (100+ keyboard-layout files).
- Extensions: `services/extensions/common/{extensionHostProtocol,rpcProtocol,abstractExtensionService}.ts`; `electron-browser/localProcessExtensionHost.ts`.
- API bridge: `workbench/api/browser/main*.ts` (80+ `mainThread*` proxies); `workbench/api/common/extHost*.ts`; `extHost.api.impl.ts`; `extHost.protocol.ts`.
- Debug: `contrib/debug/browser/debugService.ts:63-117`, `debugSession.ts`, `debugAdapterManager.ts`, `rawDebugSession.ts`; common at `debug.ts:1133`, `debugProtocol.d.ts`.
- Terminal: `contrib/terminal/browser/{terminal.ts:39-46,terminalService.ts:66-120,terminalInstance.ts,terminalProcessManager.ts,xterm/xtermTerminal.ts}`; `electron-browser/localTerminalBackend.ts`.
- SCM: `contrib/scm/common/{scm.ts:35-96,scmService.ts,quickDiff.ts}`; `browser/scmViewPane.ts`.
- Files: `services/files/{common/files.ts,electron-browser/diskFileSystemProvider.ts}`.

**Electron-coupled friction points.** `base/parts/ipc/electron-main/ipcMain.ts`, `base/parts/ipc/electron-browser/ipc.electron.ts`, `code/electron-main/main.ts` & `app.ts`, `base/parts/sandbox/`, `services/files/electron-browser/diskFileSystemProvider.ts`, `contrib/terminal/electron-browser/localTerminalBackend.ts` (PTY/shell), `services/extensions/electron-browser/localProcessExtensionHost.ts`, `server/node/*`.

### Public Extension API Contract (`src/vscode-dts/`, Partition 6)

**Surface size.** 1 stable `vscode.d.ts` + 169 proposed `vscode.proposed.*.d.ts` — ~2,000+ functions/properties/events.

**Namespaces (with line ranges).**
- `window` — editors, terminals, notebook editors, dialogs, status bar, tree views, quick input. Lifecycle events `onDidChange{ActiveTextEditor,VisibleTextEditors,…}` at `vscode.d.ts:11069-11175`.
- `workspace` — `fs: FileSystem`, `workspaceFolders`, `getConfiguration`, `createFileSystemWatcher`, `applyEdit`, file lifecycle `vscode.d.ts:13797-13956`.
- `languages` — 40+ providers (`CompletionItemProvider:5189-5223`, `DefinitionProvider:2925-2936`, `Hover:3144-3158`, `DocumentHighlight:3388`, `DocumentSymbol:3638`, `Reference:3717`, `Rename:4209`, `InlayHints:5700`).
- `debug` — `vscode.d.ts:17283-17398` (`registerDebug{ConfigurationProvider,AdapterDescriptorFactory,AdapterTrackerFactory}`).
- `scm` — `createSourceControl(id,label,rootUri)` at `vscode.d.ts:16652-16670`.
- `terminal`, `tasks`, `comments`, `notebooks`, `chat`, `lm`, `authentication`, `tests`, `extensions`, `commands` (`registerCommand` etc. at `:10973-11030`).
- `FileSystemProvider` — `vscode.d.ts:9600-9700` (stat/readDirectory/readFile/writeFile/delete/rename/watch + `onDidChangeFile`).

**Cross-cutting types.** `Thenable<T>`, `Event<T>`, `CancellationToken`, `Disposable`, `ProviderResult<T>`. Provider registration always returns `Disposable`. Edit batching uses callback-style `editor.edit(builder => …)` with explicit undo-stop control at `:1258-1352`.

### Existing Rust Foundation (`cli/`, Partition 9)

**75 files, 20,107 LOC.** Already-shipping production Rust binary.

**Modules.**
- RPC: `cli/src/rpc.rs:49-91` (`RpcBuilder`/`RpcCaller`/`RpcDispatcher` generic over `Serialization` trait); `json_rpc.rs:21-42` (newline-delimited); `msgpack_rpc.rs:24-41` (binary, preferred); `async_pipe.rs:42-44` (Unix socket split).
- Tunnels (25 files / 7,579 LOC): `cli/src/tunnels/dev_tunnels.rs` (~3000 LOC), `control_server.rs:208-310,245-355` (multi-arm `tokio::select!` event loop), `code_server.rs`, `server_bridge.rs:21-62`, `port_forwarder.rs`, `local_forwarding.rs`, `singleton_{server,client}.rs`.
- Cross-platform service: `service.rs` + `service_{linux,macos,windows}.rs` (systemd/launchd/SCM); sleep inhibitor `nosleep.rs` + platform variants.
- Update: `cli/src/self_update.rs`, `update_service.rs`, `download_cache.rs`.
- Auth: `cli/src/auth.rs` (OAuth device-code + `keyring` 2.0); `state.rs` (`LauncherPaths`, `PersistedState` JSON).
- Commands (4,208 LOC): `commands/{context,args,output,agent,agent_host,tunnels,serve_web,update,version}.rs` — Clap derive.
- Util (3,003 LOC): `util/{command,errors,http,input,io,os,machine,prereqs,app_lock,file_lock,ring_buffer,sync,tar,zipper}.rs`. `util/sync.rs:12-68` `Barrier<T>` over `tokio::sync::watch`. `util/command.rs:13-71,124-168` async `tokio::process::Command` with `kill_tree`.

**Tokio usage.** Pervasive: spawn pumps in `rpc.rs:483`, `json_rpc.rs:85-96`, `msgpack_rpc.rs:79-95`, `serve_web.rs:124,171,322,351,378,629,761,776`, `agent_host.rs:141,273,522,542`. Multi-source select at `agent_host.rs:155-295` with `tokio::pin!(startup_deadline)`. `tokio::io::duplex()` for in-memory pipes.

**Crate dependencies.** `tokio` 1.52, `reqwest` 0.13, `serde`/`serde_json`/`rmp-serde`, `tunnels` (custom fork), `clap` 4.3 derive, `zip`/`tar`/`flate2`, `keyring` 2.0, `tokio-tungstenite` 0.29, `winreg`/`winapi`/`windows-sys` (Win), `core-foundation` (macOS), `zbus` (Linux D-Bus).

**Reusable foundations for Tauri host.** RPC dispatcher (transport-agnostic) → ext-host protocol; `Barrier<T>` → startup readiness; cross-platform service/sleep; OAuth + keyring → settings sync; async pipe + duplex → LSP/DAP transports; Clap arg layer.

**Gap.** ~20K Rust LOC vs. partition 2 alone (683K TS LOC for Copilot). No Monaco-equivalent text/edit model exists yet.

### Architectural Invariants (`.eslint-plugin-local/`, Partition 19)

**49 custom ESLint rules** encoding constraints TypeScript can't statically enforce — any Rust port must replicate semantically.

**Layering.**
- `code-layering.ts:15-92` — directory-based layer guard.
- `code-import-patterns.ts:19-279` — full 7-layer lattice `common → worker → browser → electron-browser` and `common → node → electron-utility → electron-main`; `LayerAllowRule.when ∈ {hasBrowser,hasNode,hasElectron,test}` (`:26`); `layerRules` table at `:97`; `/~/` placeholder expansion via `generateConfig()` at `:134`.
- `code-no-deep-import-of-internal.ts`, `code-no-static-node-module-import.ts`.

**Disposable / RAII.**
- `code-must-use-super-dispose.ts:13-32` — `dispose` overrides must call `super.dispose()`.
- `code-no-potentially-unsafe-disposables.ts:32-35` — `DisposableStore`/`MutableDisposable` must be `const`/`readonly`.
- `code-ensure-no-disposables-leak-in-test.ts:9-44` — tests must call `ensureNoDisposablesAreLeakedInTestSuite()`.

**Async / reactive correctness.**
- `code-no-reader-after-await.ts:10-169` — `reader` from `derived`/`autorun*` cannot be used after `await`.
- `code-no-observable-get-in-reactive-context.ts:11-94` — banned `observable.get()` (zero-arg) inside 14 reactive primitives.
- `code-no-accessor-after-await.ts:24-421` — `ServicesAccessor` invalidated by `await`; branch-aware dataflow with separate `sawAwait` per `if`/`switch`/`try`/`for-await`.

**DI / API contract.**
- `code-declare-service-brand.ts:9-29` — `_serviceBrand` must be `declare _serviceBrand: undefined;`.
- `vscode-dts-event-naming.ts:10-86` — `Event<T>` must match `/on(Did|Will)([A-Z][a-z]+)/`.
- `vscode-dts-cancellation.ts:9-31` — `*Provider` `provide*`/`resolve*` methods must include a `token` parameter.

### Bootstrap & Process Model (Partitions 38, 50–52, 55, 57, 63, 66, 67, 75, 78, 79)

**Chain.** `bootstrap-cli.ts | bootstrap-server.ts` (env scrub) → `bootstrap-node.ts` (Node setup) → `bootstrap-meta.ts` (product/pkg) → `bootstrap-esm.ts` (loader hook + globals + NLS) → entrypoint dynamic import. Forked children additionally pass through `bootstrap-fork.ts`.

**`src/main.ts:23-214` — Electron entry.**
- `app.enableSandbox()` at `:175`, `app.setPath('userData', userDataPath)`.
- `protocol.registerSchemesAsPrivileged([{scheme:'vscode-webview',...}, {scheme:'vscode-file',...}])` at `:96-105`.
- `app.once('ready', onReady)` at `:147` → `mkdirpIgnoreError(codeCachePath)` → `resolveNlsConfiguration()` → `startup()` → `bootstrapESM()` → `import('./vs/code/electron-main/main.js')`.
- macOS pre-ready listeners: `:589-621` (`open-file`/`will-finish-launching`/`open-url`); `globalThis.macOpenFiles`, `globalThis.getOpenUrls`.

**`vs/code/electron-main/app.ts` lifecycle.** `accessibility-support-changed:412`, `activate:417`, `web-contents-created:430` (security gate, `setWindowOpenHandler`), `open-file:479` (NFC normalize, 100ms debounce), `new-window-for-tab:509`. Protocol registrations: `protocolMainService.ts:53` `registerFileProtocol(Schemas.vscodeFileResource)`; `app.ts:698` `registerBufferProtocol(Schemas.vscodeManagedRemoteResource)`; `app.ts:1537` `registerHttpProtocol(Schemas.vscodeRemoteResource)`.

**`src/server-main.ts:6-285` — headless server.**
- License prompt at `:65-82` (`product.serverLicense`, `--accept-server-license-terms`).
- `http.createServer` + `'upgrade'` at `:88-108`; perf marks `code/server/firstRequest`, `code/server/firstWebSocket`.
- Listen options at `:110-115` (Unix socket via `--socket-path` or TCP).
- **Sentinel string** at `:137`: `Extension host agent listening on <port>` — Remote-SSH parses this verbatim.
- **Sentinel** at `:181`: `--port: Could not find free port in range:`.
- Lazy singleton via `getRemoteExtensionHostAgentServer` returning `Promise<IServerAPI>`.

**`src/bootstrap-fork.ts:10-229` — universal child process entry.**
- `pipeLoggingToParent()` at `:14-154` — circular-safe stringify, 1MB stream buf, line-by-line `wrapStream('stderr','error')`/`('stdout','log')`, `safeSendConsoleMessage` over `process.send()`.
- `terminateWhenParentTerminates()` at `:169-181` — `setInterval(()=>process.kill(parentPid,0), 5000)` watchdog.
- `configureCrashReporter()` at `:183-201`.
- Final dispatch: `await import(['./${VSCODE_ESM_ENTRYPOINT}.js'].join('/'))` at `:229`.

**Env-var contract.** `VSCODE_CWD`, `VSCODE_CLI`, `VSCODE_DEV`, `VSCODE_NLS_CONFIG`, `VSCODE_PORTABLE`, `VSCODE_HANDLES_SIGPIPE`, `VSCODE_HANDLES_UNCAUGHT_ERRORS`, `VSCODE_PARENT_PID`, `VSCODE_PIPE_LOGGING`, `VSCODE_VERBOSE_LOGGING`, `VSCODE_ESM_ENTRYPOINT`, `VSCODE_CRASH_REPORTER_PROCESS_TYPE`, `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH`, `VSCODE_SERVER_HOST/PORT/ACCEPT_SERVER_LICENSE_TERMS`, `VSCODE_IPC_HOOK_CLI`, `ELECTRON_RUN_AS_NODE`.

**Globals contract.** `_VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON`, `_VSCODE_FILE_ROOT`, `_VSCODE_NLS_LANGUAGE`, `_VSCODE_NLS_MESSAGES` (all set in `bootstrap-esm.ts:33-35` and `:49-104`).

**`src/cli.ts` (26 LOC) and `vs/code/node/cli.ts` (610 LOC).** Subcommands `tunnel`/`serve-web`/`agent`/`chat` already spawn the Rust `cli/` binary via `child_process.spawn` (deleting `ELECTRON_RUN_AS_NODE`). Extension management (`--install-extension` etc.) routes through a CLI subprocess.

### Build Pipeline (`build/`, Partition 5; `gulpfile.mjs`, Partition 79)

`gulpfile.mjs:5` is a single-line `import './build/gulpfile.ts'`. `build/gulpfile.ts:56-59` glob-loads all sibling `gulpfile.*.ts`. Composition via `task.define()` + `task.series()/parallel()` at `build/lib/task.ts:82-98`.

**Specialized gulpfiles.** `gulpfile.compile.ts`, `gulpfile.extensions.ts:117-245`, `gulpfile.editor.ts`, `gulpfile.vscode.ts:230-271` (CI mangle+minify), `gulpfile.vscode.{linux,win32,web}.ts`, `gulpfile.reh.ts`, `gulpfile.cli.ts`, `gulpfile.hygiene.ts`, `gulpfile.scan.ts`.

**Modern esbuild CLI.** `build/next/index.ts:33-60,513-525,700-790,828-880,903-926` — `transpile`/`bundle` modes, `--minify --nls --mangle-privates --target desktop|server|server-web`. Plugins: `nls-plugin.ts`, `private-to-property.ts`, `inlineMeta.ts` (sentinel substitution `BUILD_INSERT_PRODUCT_CONFIGURATION`/`BUILD_INSERT_PACKAGE_CONFIGURATION`).

**Shared esbuild configs.**
- `extensions/esbuild-extension-common.mts:24-93` — bundle/minify/treeShake/sourcemap, `target:['es2024']`, `external:['vscode']`, `format:'cjs'` (Node) or browser variant with `path-browserify` alias and `process.platform:'web'`. Used by all 51 extension `esbuild.mts` files.
- `extensions/esbuild-webview-common.mts:17-82` — strict ESM/browser/es2024, embeds TTF as dataurl, no externals (full bundling). Used by 6-7 webview-bearing extensions.

### Built-in Extensions

#### Source Control (Partitions 7, 36, 23, 24)

**`extensions/git/`** (62 files, 25,181 LOC).
- `extensions/git/src/main.ts:88-124` — activate constructs `Git`, `Model`, `CommandCenter`, registers all subsystems; `:522` `languages.registerDocumentLinkProvider('git-commit', new GitEditorDocumentLinkProvider(model))`.
- `src/git.ts` (~2,900 LOC) — `Git` class wrapping subprocess. `Git.spawn()` at `:676-703` calls `cp.spawn(this.path, args, options)` with merged env (`VSCODE_GIT_COMMAND`, `LANG=en_US.UTF-8`, `GIT_PAGER=cat`). `_exec()` at `:619`, `stream()` at `:606`, `findSpecificGit()` at `:87`, `cpErrorHandler` at `:189` mapping `ENOENT` → `GitErrorCodes.NotAGitRepository`. Streamed null-delimited status: `GitStatusParser` at `:821-885`.
- `src/repository.ts:983-1009` — `scm.createSourceControl('git', 'Git', root, icon, isHidden, parent)` + `quickDiffProvider`, `secondaryQuickDiffProvider`, `historyProvider`, four resource groups (`merge`/`index`/`workingTree`/`untracked`); `acceptInputCommand = 'git.commit'`.
- `src/api/api1.ts`, `src/api/git.d.ts` — public API exported to other extensions.

**`extensions/git-base/`** (14 files, 1,015 LOC). `src/api/git-base.d.ts:9-13` versioned `API` (`registerRemoteSourceProvider`, `getRemoteSourceActions`, `pickRemoteSource`); `src/foldingProvider.ts:8-92` `GitCommitFoldingProvider`; `src/decorators.ts:8-48` `debounce`/`throttle` method decorators.

**`extensions/github/`** (21 files, 3,119 LOC). `src/extension.ts:91-138` registers `registerCredentialsProvider`, `registerBranchProtectionProvider`, `registerPushErrorHandler`, `registerRemoteSourcePublisher`. 9 commands at `src/commands.ts:179-246`.

**`extensions/github-authentication/`** (24 files, 3,104 LOC).
- `src/github.ts:179-189` — `authentication.registerAuthenticationProvider(type, friendlyName, this, { supportsMultipleAccounts, supportedAuthorizationServers })`.
- Four flows in `src/flows.ts`: `LocalServerFlow:287-385`, `UrlHandlerFlow:197-285`, `DeviceCodeFlow:387-454`, `PatFlow:522`; selection at `:614`.
- PKCE SHA-256 at `:121-146` (`generateRandomString(64)`, `generateCodeChallenge`).
- `src/node/authServer.ts:71-210` — `LoopbackAuthServer` on `127.0.0.1:0`, `/signin` and `/callback` with state+nonce.
- `src/common/keychain.ts:16-47` — `setToken`/`getToken`/`deleteToken` over `context.secrets`; service id `type.auth` or `authority.path.ghes.auth`.
- Hardcoded `gitHubClientId='01ab8ac9400c4e429b23'` (`src/config.ts`).

**`extensions/microsoft-authentication/`** (31 files, 3,561 LOC).
- `src/extension.ts:62, 118-130` — two `registerAuthenticationProvider` calls; `microsoft` and `microsoft-sovereign-cloud`.
- `src/node/authProvider.ts:39-566` — `MsalAuthProvider` (MSAL-Node).
- `src/betterSecretStorage.ts:15-248` — `BetterTokenStorage<T>` over `context.secrets` with `onDidChange` cross-window sync.
- `src/common/cachePlugin.ts:9-55` — `SecretStorageCachePlugin` MSAL cache (key `pca:${clientId}`).
- `src/node/flows.ts:40-153` — three flows: `DefaultLoopbackFlow`, `UrlHandlerFlow:76-104`, `DeviceCodeFlow:106-123`.
- Native broker via `@azure/msal-node-extensions` (DPAPI on Windows). Uses proposed `vscode.proposed.nativeWindowHandle`.

**`extensions/merge-conflict/`** (13 files, 1,463 LOC). `src/codelensProvider.ts:9-108` `registerCodeLensProvider` for multi-scheme `[{scheme:'file'},{scheme:'vscode-vfs'},{scheme:'untitled'},{scheme:'vscode-userdata'}]`; markers `<<<<<<<`, `|||||||`, `=======`, `>>>>>>>` (state machine in `src/mergeConflictParser.ts:10-86`).

#### Language Intelligence (Partitions 8, 12, 14, 15, 16, 17, 22, 25, 28, 33, 34)

**TypeScript LSP (Partition 8, `extensions/typescript-language-features/`, 168 files, 22,571 LOC) — NOT vanilla LSP.**
- `src/typescriptServiceClient.ts:108-250` — `TypeScriptServiceClient` central hub.
- Spawns Node child running tsserver via `src/tsServer/serverProcess.electron.ts` (browser variant uses Web Worker).
- **Communication:** JSON-RPC-like over stdio (newline-delimited JSON, NOT LSP). Typed request map at `src/typescriptService.ts:38-99` (`StandardTsServerRequests`, `NoResponseTsServerRequests`, `AsyncTsServerRequests`) — 60+ commands like `'completionInfo'`, `'quickinfo'`, `'definition'`.
- `src/tsServer/requestQueue.ts:7-57` — priority queue (`Normal`/`LowPriority`/`Fence`).
- Multi-process: separate syntax/semantic/diagnostics tsservers.
- `src/tsServer/bufferSyncSupport.ts` — `'open'`/`'change'`/`'close'` notifications; `interruptGetErr()`.
- `src/tsServer/versionManager.ts` — workspace `node_modules/typescript` discovery.
- Provider registrations under `src/languageFeatures/` (25+ files).
- `src/typeConverters.ts:15-68` — TS server is 1-based, VS Code 0-based.

**Standard LSP clients (HTML / JSON / CSS — Partitions 12, 25, 28).** Identical wiring shape:
- Client `vscode-languageclient/node` Node `ServerOptions = { run/debug: { module: serverModule, transport: TransportKind.ipc } }`.
- Browser variant `new Worker(serverMain)` passed to `LanguageClient`.
- Server-side `vscode-languageserver/node` `createConnection()` (Node) or `BrowserMessageReader/Writer` (worker).
- Server-side filesystem requests tunnelled back via custom RequestTypes: `FsContentRequest('fs/content')`, `FsStatRequest('fs/stat')`, `FsReadDirRequest('fs/readDir')`.
- HTML LSP capabilities at `extensions/html-language-features/server/src/htmlServer.ts:88-227`; embedded modes via `server/src/modes/languageModes.ts`.
- JSON LSP at `extensions/json-language-features/server/src/jsonServer.ts:146-208`; custom messages (`jsonClient.ts:27-70`) `VSCodeContentRequest('vscode/content')`, `SchemaContentChangeNotification('json/schemaContent')`, etc.
- CSS LSP at `extensions/css-language-features/server/src/cssServer.ts:69-138`; multiplexes css/scss/less via `languageServices` map.

**Markdown (Partition 14, `extensions/markdown-language-features/`, 86 files, 8,704 LOC).** Hybrid: webview + LSP + extension-host providers.
- `src/preview/previewManager.ts:102-104` — `registerCustomEditorProvider('vscode.markdown.preview.editor', this, { webviewOptions: { enableFindWidget: true } })`; also `registerWebviewPanelSerializer('markdown.preview', this)`.
- `src/preview/preview.ts:495-612` `StaticMarkdownPreview` (custom editor), `:621-830` `DynamicMarkdownPreview` (panel).
- `src/preview/documentRenderer.ts:43-128` — HTML+CSP+initial-data assembly with `<base href={asWebviewUri(uri)}>` and `nonce`.
- Webview message protocol typed in `types/previewMessaging.d.ts` — `FromWebviewMessage`/`ToWebviewMessage` over `webview.postMessage()`/`onDidReceiveMessage`.
- LSP-backed diagnostics via `src/client/client.ts`.

**Notebook (Partitions 17, 22).**
- `extensions/ipynb/`: `src/notebookSerializer.ts:13-88` `NotebookSerializerBase`; `notebookSerializer.node.ts:10` Node subclass with `worker_threads`; `notebookSerializer.web.ts:10` browser subclass with `Worker`. Registration `ipynbMain.ts:53,67` `workspace.registerNotebookSerializer('jupyter-notebook'|'interactive', serializer, { transientOutputs:false, transientCellMetadata:{...} })`.
- `extensions/notebook-renderers/`: pure webview-side renderer (`requiresMessaging:"never"`). Activation contract `src/index.ts:419-639` `export const activate: ActivationFunction<void>`. MIME dispatch at `:541-609` for `text/html`, `image/svg+xml`, `application/javascript`, error/stderr/stdout.

**PHP (Partition 16) — no LSP.** `src/phpMain.ts:13-22` registers `CompletionItemProvider`/`HoverProvider`/`SignatureHelpProvider` directly; `src/features/validationProvider.ts:84-275` spawns `php -l -n -d display_errors=On` with stderr regex `/(?:(?:Parse|Fatal) error): (.*)(?: in )(.*?)(?: on line )(\d+)/`.

**Emmet (Partition 15).** `src/emmetCommon.ts:203-221` `registerCompletionItemProvider(LANGUAGE_MODES, ...)` + `registerInlineCompletionItemProvider`; depends on `@vscode/emmet-helper`, `@emmetio/css-parser`, `@emmetio/html-matcher`.

**Configuration / Extension editing (Partitions 33, 34).** Direct extension-host providers (no LSP). `extensions/configuration-editing/src/configurationEditingMain.ts:32-37` settings completion; `:183-243` `registerContextKeyCompletions` with JSONPath matchers. `extensions/extension-editing/src/extensionLinter.ts:60-82` validates `package.json` (icon URL HTTPS, badge URL allowlist from `product.json`, API proposal allowlist from `product.extensionEnabledApiProposals`, implicit activation events `:500-598`); when-clauses validated via `_validateWhenClauses` core command.

**Markdown-math (Partition 53).** `src/extension.ts:11-45` — `extendMarkdownIt(md)` adding KaTeX; `notebook/katex.ts:10-58` injects KaTeX CSS into shadow DOM.

#### Terminal (Partition 4)

**`extensions/terminal-suggest/`** (197 files, 64,006 LOC).
- `src/terminalSuggestMain.ts:228-299` — `vscode.window.registerTerminalCompletionProvider` with `'/'` and `'\\'` separators; 5-second `Promise.race` timeout.
- `src/env/pathExecutableCache.ts:21-150` — multi-tier PATH caching with FS watchers.
- `src/shell/{bash,zsh,fish,pwsh}.ts` — shell-specific builtin/alias discovery (`getBashGlobals` at `bash.ts:11-68`).
- `src/fig/` — 30+ files: shell-parser, autocomplete-parser, generators forked from withFig.
- `src/completions/git.ts:162-214` — Fig generator with `script`+`postProcess` and `cache.strategy:'stale-while-revalidate'`.

#### Tasks & Debug (Partitions 27, 46, 48, 49, 45, 47)

**TaskProviders (`gulp`, `grunt`, `jake`, `npm`).** Identical pattern: `vscode.tasks.registerTaskProvider(<name>, {provideTasks, resolveTask})`; per-folder `FolderDetector` with `FileSystemWatcher`; `findXxxCommand()` platform-aware.
- `extensions/gulp/src/main.ts:337-344` — `'gulp'`; computes via `gulp --tasks-simple --no-color`.
- `extensions/grunt/src/main.ts:296` — `'grunt'`; parses `grunt --help --no-color` (regex `/^\s*(\S.*\S)  \S/g`).
- `extensions/jake/src/main.ts:270` — `'jake'`; parses `jake --tasks` (regex `/^jake\s+([^\s]+)\s/g`).
- `extensions/npm/src/tasks.ts:46-87` — `NpmTaskProvider`; `npmMain.ts:124-141` registers; `tasks.ts:447-456` calls `extension.js-debug.createDebuggerTerminal`.

**Debug Auto-Launch (Partition 45).**
- `extensions/debug-auto-launch/src/extension.ts:11-16` — `State = Disabled | OnlyWithFlag | Smart | Always`.
- `:248-275` — `net.createServer` listening on platform IPC address; NUL-byte-framed JSON; ACK 0x00 / NAK 0x01; forwards via `commands.executeCommand('extension.js-debug.autoAttachToProcess', ...)`.
- `:362-397` — `getIpcAddress` cached in `context.workspaceState[STORAGE_IPC]`; calls `extension.js-debug.setAutoAttachVariables`.

**Debug Server-Ready (Partition 47).**
- `extensions/debug-server-ready/src/extension.ts:323-336` — `vscode.debug.onDidStartDebugSession`/`onDidTerminateDebugSession`.
- `:340-350` — `registerDebugConfigurationProvider('*', ...)` wildcard.
- `:353-393` — `registerDebugAdapterTrackerFactory(type, ...)`; `onDidSendMessage` inspects DAP `output` events; correlates `runInTerminal` request/response to capture `shellProcessId`.
- `:99-121` — `window.onDidWriteTerminalData` (proposed `terminalDataWriteEvent`); routes by `terminal.processId`.
- `:218-297` — `killOnServerStop` embeds `_debugServerReadySessionId` UUID in child config; awaits matching session via `catchStartedDebugSession`.
- ANSI stripping regex copied from `src/vs/base/common/strings.ts:36-37`.

#### Custom Editors / Webviews / Tree Views (Partitions 31, 39, 30, 42, 35)

**`extensions/media-preview/`** — image/audio/video custom editors. `src/imagePreview/index.ts:244-281` `registerCustomEditorProvider(viewType, previewManager, { supportsMultipleEditorsPerDocument: true })`. CSP/nonce, `cspSource`, `asWebviewUri` at `:182-215`. Watcher-driven re-render at `src/mediaPreview.ts:52-64` via `createFileSystemWatcher(RelativePattern(_resource, '*'))`.

**`extensions/simple-browser/`** — `src/extension.ts:55-59` `registerWebviewPanelSerializer('simpleBrowser.view', { deserializeWebviewPanel })`. `simpleBrowserView.ts:40-61` `createWebviewPanel(..., { retainContextWhenHidden: true })`. `preview-src/index.ts:107` `vscode.setState({ url })` for restore.

**`extensions/references-view/`** — sidebar trees for references / call hierarchy / type hierarchy. `src/tree.ts:31-35` `createTreeView<unknown>(viewId, { treeDataProvider, dragAndDropController, showCollapseAll })`. Drives LSP commands `vscode.executeReferenceProvider`, `vscode.prepareCallHierarchy`/`provideIncomingCalls`/`provideOutgoingCalls`, `vscode.prepareTypeHierarchy`/`provideSupertypes`/`provideSubtypes`.

**`extensions/search-result/`** — `.code-search` virtual-document language. `src/extension.ts:39-106` registers `DocumentSymbolProvider` (`:39-53`), `CompletionItemProvider` (`:55-74`, trigger `'#'`), `DefinitionProvider` (`:76-98`), `DocumentLinkProvider` (`:100-106`). Multi-scheme path resolution at `:130-175` (settings → `vscode-userdata`, `~/`, `name • path`, untitled).

**`extensions/mermaid-chat-features/`** — `src/chatOutputRenderer.ts:147-154` `vscode.lm.registerTool<{markup, title?}>('renderMermaidDiagram', { invoke })`; `:159-160` `vscode.chat.registerChatOutputRenderer(viewType, renderer)` (proposed `chatOutputRenderer`).

#### Authentication (Partitions 21, 24)

Both extensions exercise the same shape: `authentication.registerAuthenticationProvider`, `context.secrets` for keychain-backed token storage with `onDidChange` cross-window sync, and a flow selector across loopback / URL-handler / device-code variants. Key file references already cited above. Both register `window.registerUriHandler` for OAuth redirects on `vscode://...` URIs.

#### Tunnels & Remote (Partitions 37, 44)

**`extensions/vscode-test-resolver/`** (Partition 37, 7 files, 925 LOC) — reference RemoteAuthorityResolver.
- `src/extension.ts:327-344` — `workspace.registerRemoteAuthorityResolver('test', { getCanonicalURI, resolve, tunnelFactory, showCandidatePort })`; re-registers on error at `:364`.
- `:212-234, 237-323` — `doResolve` builds `ResolvedAuthority` with `net.createServer` proxying local↔remote sockets.
- `:85, 105, 369` — `RemoteAuthorityResolverError.{TemporarilyNotAvailable,NotAvailable}`.
- `:509-571` — `tunnelFactory(tunnelOptions, tunnelCreationOptions)` w/ elevation prompt.
- Browser variant `src/extension.browser.ts:9-77` — `ManagedResolvedAuthority` with `InitialManagedMessagePassing implements vscode.ManagedMessagePassing` over WebSocket.

**`extensions/tunnel-forwarding/`** (Partition 44, 4 files, 474 LOC) — wraps existing Rust CLI.
- `src/extension.ts:93-106` — `workspace.registerTunnelProvider(provider, { tunnelFeatures: { elevation:false, protocol:true, privacyOptions:[Public, Private] } })`.
- `:266-342` — `setupPortForwardingProcess` calls `authentication.getSession('github', ['user:email','read:org'], { createIfNone:true })`, spawns `cli/target/debug/code` (or bundled `code-tunnel(-insiders)`) with `VSCODE_CLI_ACCESS_TOKEN` env.
- JSON-line stdin protocol at `:250-264`.
- Direct overlap with Rust `cli/src/tunnels/`: `port_forwarder.rs` (`PortForwardingProcessor`/`PortForwardingRec::{Forward,Unforward}`); `local_forwarding.rs` (matching JSON-RPC); `protocol.rs` (`ForwardParams`/`UnforwardParams`/`ForwardResult`/`PortPrivacy`/`PortProtocol`); `dev_tunnels.rs` (`ActiveTunnel.{add_port_tcp, remove_port}`).

#### Copilot / Chat (Partition 2)

**`extensions/copilot/`** (2,880 files, 682,973 LOC).
- Entry: `src/extension/extension/vscode-node/extension.ts`, `vscode-worker/extension.ts`.
- API contract: `src/extension/api/vscode/api.d.ts` (`CopilotExtensionApi`).
- Tool registry: `src/extension/tools/common/toolsRegistry.ts:122-153` (`registerTool`, `registerModelSpecificTool`).
- Chat participants: `src/extension/chatSessions/vscode-node/chatSessions.ts:158,232,330`; `src/extension/conversation/vscode-node/chatParticipants.ts:204-276`.
- LM provider registration: `src/extension/chatSessions/claude/node/claudeCodeModels.ts:61-78`.
- BYOK providers: `src/extension/byok/vscode-node/{openAIProvider,anthropicProvider,geminiNativeProvider,ollamaProvider}.ts`.
- Platform abstractions (~50 services) under `src/platform/{git,terminal,filesystem,languages,debug,telemetry,authentication,endpoint}/` mirror the workbench services for in-extension use.
- Triple-folder layering: `common/` + `node/` + `vscode-node/` + `vscode-worker/`.
- Prompts as TSX components in `src/extension/prompts/node/` (40+); rendered via `renderPromptElementJSON` with `tokenBudget`/`countTokens`.

### Conformance Test Bed (`extensions/vscode-api-tests/`, Partition 11)

**38 `*.test.ts` modules, 50 files, 11,425 LOC.** Acts as executable specification.

- Entry: `src/extension.ts`, `src/utils.ts:49-134` (`closeAllEditors`, `disposeAll`, `asPromise(event,timeout)`, `assertNoRpc()`).
- In-memory FS provider: `src/memfs.ts`.
- Coverage breakdown: `editor.test.ts:10-95`, `languages.test.ts:29-120`, `debug.test.ts:28-145`, `terminal.test.ts:43-175`, `terminal.shellIntegration.test.ts`, `workspace.fs.test.ts:21-125`, `notebook.api.test.ts:39-77`, `tree.test.ts:21-180`, `chat.test.ts:50-91`, `lm.test.ts`, `commands.test.ts:19-81`.
- `assertNoRpc()` checks no live extension-host proxies leaked — implies extension API objects ARE RPC proxies in the current architecture.
- 62 `enabledApiProposals` declared in `package.json`.

## Architecture & Patterns

**Service brand + decorator-based DI.** Every service interface declares `_serviceBrand: undefined`; injection tokens via `createDecorator<T>('id')`; constructors annotated `@IServiceName`. Hierarchical containers via `InstantiationService.createChild()`. `registerSingleton(IFoo, FooImpl, InstantiationType.Delayed|Eager)` is the canonical inventory of every workbench/platform service a Rust host must replace or proxy.

**RAII Disposable lifecycle.** `Disposable._store: DisposableStore`, cascading `_register()`. Maps directly to Rust `Drop`. ESLint enforces `super.dispose()` calls (`code-must-use-super-dispose.ts`) and leak-tracking in tests (`code-ensure-no-disposables-leak-in-test.ts`).

**Layering lattice.** 7 layers enforced by `code-layering.ts` and `code-import-patterns.ts`: `common → worker → browser → electron-browser`, `common → node → electron-utility → electron-main`. `editor/*` cannot import from `workbench/*`. Maps to Cargo crate visibility.

**Typed IPC channels.** `IServerChannel` (server-side `listen`/`call` switch) + `IChannel` (client proxy). URI marshalling via `URI.revive` and `IURITransformer.transformOutgoingURI` (multi-machine). Generic RPC proxy generation in `src/vs/workbench/services/extensions/common/rpcProtocol.ts`.

**Provider registry pattern.** `LanguageFeatureRegistry<Provider>` — multiple providers per language; aggregator at `src/vs/editor/common/services/languageFeatures.ts:10-81`. Same shape used by 40+ `vscode.languages.register*Provider` API entry points.

**Process model.** Electron main → Electron renderer (workbench browser) → forked Node child (`bootstrap-fork.ts`) for extension host / language servers / terminal / tsserver. Communication: `process.send` (Node fork channel), MessagePort, raw socket (remote), or stdio JSON-RPC (LSP, tsserver).

**Shared bootstrap chain.** Five-stage import sequence with strict env-var contract; sentinel substitution at build-time (`BUILD_INSERT_PRODUCT_CONFIGURATION` etc.); five `globalThis._VSCODE_*` properties set before any application code runs.

**Webview message protocol.** `webview.postMessage()` + `onDidReceiveMessage` with typed message unions (e.g. `previewMessaging.d.ts`). `setState`/`getState` for restore. CSP nonce + `asWebviewUri` for resource serving. `localResourceRoots` ACL.

**Custom URL schemes.** `vscode-webview://`, `vscode-file://`, `vscode-remote-resource://`, `vscode-managed-remote-resource://`, custom-language schemes (`git-commit`, `merge-conflict.conflict-diff`, `search-result`).

## Code References

- `src/vs/platform/instantiation/common/instantiation.ts:41` — `createDecorator<T>(serviceId)`
- `src/vs/platform/instantiation/common/extensions.ts:25` — `registerSingleton`
- `src/vs/platform/instantiation/common/instantiationService.ts:28-120` — `InstantiationService`
- `src/vs/base/common/lifecycle.ts:526-557` — `Disposable` base
- `src/vs/workbench/services/extensions/common/rpcProtocol.ts` — generic RPC proxy
- `src/vs/editor/common/services/languageFeatures.ts:10-81` — `ILanguageFeaturesService`
- `src/vs/editor/browser/gpu/gpu.ts` — GPU text rendering
- `src/vs/editor/browser/controller/editContext/nativeEditContext.ts` — native text input
- `src/vs/code/electron-main/app.ts:1224-1231` — channel registration
- `src/vs/code/electron-main/app.ts:430-473` — `web-contents-created` security gate
- `src/vs/platform/protocol/electron-main/protocolMainService.ts:53` — `vscode-file` registration
- `src/main.ts:96-105` — privileged scheme registration
- `src/main.ts:147` — `app.once('ready')`
- `src/server-main.ts:137` — `Extension host agent listening on <port>` sentinel
- `src/server-main.ts:181` — port-range error sentinel
- `src/bootstrap-fork.ts:14-154` — child-process logging pipe
- `src/bootstrap-fork.ts:169-181` — parent watchdog
- `src/bootstrap-esm.ts:14-30` — fs→original-fs ESM loader hook
- `src/bootstrap-esm.ts:33-35` — `_VSCODE_*` globals injection
- `src/bootstrap-meta.ts:12, 17` — `BUILD_INSERT_*` sentinels
- `src/cli.ts:1-26` — desktop CLI bootstrap
- `cli/src/rpc.rs:49-91` — `RpcBuilder`/`RpcCaller`/`RpcDispatcher`
- `cli/src/json_rpc.rs:21-42` — newline-delimited JSON-RPC
- `cli/src/msgpack_rpc.rs:24-41` — binary RPC
- `cli/src/util/sync.rs:12-68` — `Barrier<T>`
- `cli/src/tunnels/dev_tunnels.rs` — dev-tunnels client
- `cli/src/tunnels/port_forwarder.rs` — port-forwarding processor
- `cli/src/auth.rs` — OAuth + keyring
- `src/vscode-dts/vscode.d.ts:11069-11175` — window lifecycle events
- `src/vscode-dts/vscode.d.ts:13797-13956` — workspace file events
- `src/vscode-dts/vscode.d.ts:17283-17398` — debug API
- `src/vscode-dts/vscode.d.ts:16652-16670` — scm API
- `src/vscode-dts/vscode.d.ts:9600-9700` — FileSystemProvider
- `.eslint-plugin-local/code-layering.ts:15-92` — layer guard
- `.eslint-plugin-local/code-import-patterns.ts:19-279` — 7-layer lattice
- `.eslint-plugin-local/code-no-accessor-after-await.ts:24-421` — branch-aware ServicesAccessor invalidation
- `extensions/git/src/git.ts:676-703` — `Git.spawn`
- `extensions/git/src/repository.ts:983-1009` — `scm.createSourceControl`
- `extensions/typescript-language-features/src/typescriptServiceClient.ts:108-250` — tsserver client hub
- `extensions/typescript-language-features/src/typescriptService.ts:38-99` — typed tsserver request map
- `extensions/html-language-features/server/src/htmlServer.ts:88-227` — HTML LSP capabilities
- `extensions/markdown-language-features/src/preview/previewManager.ts:102-104` — markdown custom editor
- `extensions/ipynb/src/notebookSerializer.ts:13-88` — notebook serializer
- `extensions/notebook-renderers/src/index.ts:419-639` — webview activate fn
- `extensions/copilot/src/extension/tools/common/toolsRegistry.ts:122-153` — tool registry
- `extensions/copilot/src/extension/chatSessions/vscode-node/chatSessions.ts:158,232,330` — chat participants
- `extensions/microsoft-authentication/src/extension.ts:62, 118-130` — MS auth providers
- `extensions/github-authentication/src/github.ts:179-189` — GitHub auth provider
- `extensions/github-authentication/src/node/authServer.ts:71-210` — loopback OAuth server
- `extensions/vscode-test-resolver/src/extension.ts:327-344` — RemoteAuthorityResolver
- `extensions/tunnel-forwarding/src/extension.ts:93-106` — TunnelProvider
- `extensions/debug-auto-launch/src/extension.ts:248-275` — IPC server for js-debug
- `extensions/debug-server-ready/src/extension.ts:353-393` — DAP tracker for "server ready"
- `extensions/vscode-api-tests/src/utils.ts:49-134` — test conformance helpers

## Historical Context (from research/)

No prior research, design documents, ADRs, RFCs, specs, tickets, or notes were found anywhere in this repository addressing the question of porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust. The `research/docs/` directory exists but contained only configuration scaffolding (`.claude/settings.json`) and empty placeholder directories prior to this synthesis.

The research-history pipeline (`codebase-research-locator` → `codebase-research-analyzer`) confirmed an exhaustive sweep of `research/docs/`, `research/tickets/`, `research/notes/`, `specs/`, and ADR/RFC directories returning nothing on Tauri, Rust porting, IDE architecture, language intelligence, or debugging infrastructure. The research question is therefore fully greenfield within this repository, with no prior decisions, constraints, or open threads to inherit.

## Open Questions

- **Extension host process model under Tauri.** Tauri has no first-class `process.send` analog. Whether to retain a Node.js child-process extension host (talking to the Rust core via stdio JSON-RPC) or rebuild ext-host as a Rust subsystem with WASM-hosted JS remains undetermined.
- **GPU-accelerated text rendering.** `src/vs/editor/browser/gpu/{gpu.ts,atlas/textureAtlas.ts}` depends on Chromium's WebGPU/WebGL surface. Tauri's OS-native webviews (WKWebView/WebView2/WebKitGTK) have inconsistent GPU support; whether to retain WebGPU-in-webview, fall back to CPU rendering, or rewrite the view layer in Rust GPU primitives (`wgpu`) is unresolved.
- **Native text input semantics.** `nativeEditContext.ts` uses Chromium's EditContext API; OS-native webviews require platform-specific IME/composition handling.
- **`assertNoRpc()` invariant in `extensions/vscode-api-tests/`.** Implies extension API objects are RPC proxies. Preserving this property in a Rust host requires deliberate proxy generation (likely macro-driven) for every namespace.
- **Layering enforcement in Rust.** ESLint rules at `.eslint-plugin-local/code-import-patterns.ts:19-279` are richer than Cargo's `pub(crate)` model. May require custom `cargo-deny` config, build.rs assertions, or a custom clippy lint.
- **`Thenable<T>` ↔ `Future`/`async fn` bridging across an FFI boundary** for any Rust ext-host bindings.
- **`Event<T>` semantics.** Subscriber lifecycle, leak-tracking via `IDisposableTracker`, and `Emitter<T>.fire()` ordering guarantees need a Rust analog (likely `tokio::sync::broadcast` + manual disposal).
- **No SCM integration test coverage.** `extensions/vscode-api-tests/` has no `scm.test.ts` despite enabled SCM proposals — a Rust port would need to add this surface to its conformance suite.
- **Native auth broker dependencies.** `@azure/msal-node-extensions` (DPAPI on Windows) and platform binaries — Rust replacements via `dpapi-rs`/`security-framework`/`secret-service`.
- **Markdown / Mermaid / Emmet pure-JS parsers.** `markdown-it`, `@emmetio/*`, `morphdom` have no Rust equivalents at parity. Either keep as JS in webviews/sidecar, or accept partial functionality with `pulldown-cmark`/tree-sitter/etc.

## Methodology

Generated by the deep-research-codebase workflow with 79 partitions covering 10,647 source files (3,107,132 LOC). Each partition was investigated by four specialist sub-agents dispatched directly via the provider SDK's native agent parameter: codebase-locator, codebase-pattern-finder, codebase-analyzer, and codebase-online-researcher. A separate research-history pipeline ran codebase-research-locator → codebase-research-analyzer over the project's prior research documents.
