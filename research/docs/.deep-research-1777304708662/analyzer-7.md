# Analyzer-7: `extensions/git/` — Subprocess Management, IPC Askpass, SCM API Wiring

---

## Files Analysed

1. `/Users/norinlavaee/vscode-atomic/extensions/git/src/git.ts` (3,431 LOC)
2. `/Users/norinlavaee/vscode-atomic/extensions/git/src/main.ts`
3. `/Users/norinlavaee/vscode-atomic/extensions/git/src/askpassManager.ts`
4. `/Users/norinlavaee/vscode-atomic/extensions/git/src/askpass.ts`
5. `/Users/norinlavaee/vscode-atomic/extensions/git/src/askpass-main.ts`
6. `/Users/norinlavaee/vscode-atomic/extensions/git/src/ipc/ipcServer.ts`
7. `/Users/norinlavaee/vscode-atomic/extensions/git/src/ipc/ipcClient.ts`
8. `/Users/norinlavaee/vscode-atomic/extensions/git/src/repository.ts` (lines 975–1054)
9. `/Users/norinlavaee/vscode-atomic/extensions/git/src/model.ts`
10. `/Users/norinlavaee/vscode-atomic/extensions/git/src/api/api1.ts`
11. `/Users/norinlavaee/vscode-atomic/extensions/git/src/api/git.d.ts`
12. `/Users/norinlavaee/vscode-atomic/extensions/git/out/git/tauriProcessLauncher.js`
13. `/Users/norinlavaee/vscode-atomic/extensions/git/out/git/nodeProcessLauncher.js`
14. `/Users/norinlavaee/vscode-atomic/extensions/git/out/git/processLauncher.factory.js`
15. `/Users/norinlavaee/vscode-atomic/extensions/git/out/tauri-shell.js`
16. `/Users/norinlavaee/vscode-atomic/extensions/git/out/test/tauriShell.test.js`

---

## Per-File Notes (file:line throughout)

### `src/git.ts` — Core Git subprocess execution

**Imports and spawn surface**

- `git.ts:9` — `import * as cp from 'child_process'` is the root Node.js dependency for all subprocess operations.
- `git.ts:203–208` — `SpawnOptions` extends `cp.SpawnOptions`, adding `input?: string`, `log?: boolean`, `cancellationToken?: CancellationToken`, and `onSpawn?: (childProcess: cp.ChildProcess) => void`. This `onSpawn` callback is used at `git.ts:621` (`options.onSpawn?.(child)`) to hand the live `cp.ChildProcess` to callers (e.g., the `clone` method uses it at `git.ts:451` to attach a `byline.LineStream` to `child.stderr` for progress tracking).
- `git.ts:87–91` — `findSpecificGit`: direct `cp.spawn(path, ['--version'])` — used only for git binary discovery, not for normal command execution.
- `git.ts:96, 109, 124` — `findGitDarwin`: uses `cp.exec('which git', ...)` and `cp.exec('git --version', ...)` and `cp.exec('xcode-select -p', ...)`.

**Primary execution path**

- `git.ts:595–598` — `Git.exec(cwd, args, options)` sets `options.cwd` then calls `this._exec(args, options)`.
- `git.ts:618–673` — `Git._exec(args, options)` calls `this.spawn(args, options)` to get a child, runs `options.onSpawn?.(child)` at line 621, writes `options.input` to `child.stdin` at line 624 if provided, then calls `exec(child, options.cancellationToken)` at line 631.
- `git.ts:676–703` — `Git.spawn(args, options)` builds the full environment at lines 689–695 (merging `process.env`, extension env, and per-call overrides including `VSCODE_GIT_COMMAND`, locale vars, `GIT_PAGER`), sanitizes `cwd` at line 699, then calls `cp.spawn(this.path, args, options)` at line 702, returning `cp.ChildProcess`.
- `git.ts:210–270` — `exec(child: cp.ChildProcess, cancellationToken?)`: collects `stdout` into a `Buffer`, `stderr` into a string, resolves on `child` `exit` event. When `cancellationToken` is provided, a race promise at lines 248–262 calls `child.kill()` on cancellation and throws `CancellationError`.
- `git.ts:604–616` — `Git.stream(cwd, args, options)` calls `this.spawn(args, options)` and returns the raw `cp.ChildProcess` directly, for callers that need streaming output.

### `out/git/nodeProcessLauncher.js` — Node adapter (compiled)

- `nodeProcessLauncher.js:50–61` — `NodeGitProcessLauncher.spawn(command, args, options)` delegates to `this._spawnNode(command, args, options)` at line 51, which calls `cp.spawn(command, args, options)` at line 59.
- `nodeProcessLauncher.js:68–84` — `NodeGitChild` wraps `cp.ChildProcess`, exposing `stdin`, `stdout`, `stderr`, `pid` directly from the child, and delegating `on(event, listener)` and `kill(signal)` to it.

### `out/git/tauriProcessLauncher.js` — Tauri adapter (compiled)

- `tauriProcessLauncher.js:127–135` — `TauriGitProcessLauncher.spawn(command, args, options)` constructs a `TauriGitChild`, kicks off `_spawnAsync` (fire-and-forget), and returns `gitChild` synchronously so the call site is not blocked.
- `tauriProcessLauncher.js:137–163` — `_spawnAsync(command, args, options, gitChild)`: dynamically imports `@tauri-apps/plugin-shell` at line 139 (`require('@tauri-apps/plugin-shell')`), calls `shellPlugin.Command.create(command, args, {cwd, env})`, wires `tauriCommand.stdout.on('data')` at line 144 to push line + `'\n'` into `gitChild.stdout`, `tauriCommand.stderr.on('data')` at line 147, `tauriCommand.on('close')` at line 150 to call `gitChild.stdout._end()`, `gitChild.stderr._end()`, and `gitChild._emitExit(data.code, ...)`, and `tauriCommand.on('error')` at line 155. Spawns via `tauriCommand.spawn()` at line 161.
- `tauriProcessLauncher.js:85–118` — `TauriGitChild`: custom class with `TauriReadableStream` for `stdout`/`stderr`, `stdin = null` (stdin is not supported in Tauri context), `pid = undefined` (PID not exposed), and `kill()` at line 102 calling `this._tauriChild.kill()` fire-and-forget.

### `out/git/processLauncher.factory.js` — Runtime dispatch

- `processLauncher.factory.js:15–17` — `isTauriHost()`: returns `typeof globalThis.__TAURI_INTERNALS__ !== 'undefined'` — the presence of the Tauri JS bridge global is the switch.
- `processLauncher.factory.js:23–25` — `createProcessLauncher()`: returns `new TauriGitProcessLauncher()` when in a Tauri host, else `new NodeGitProcessLauncher()`.

### `out/tauri-shell.js` — Tauri equivalents for `cp.exec` / `cp.spawn`

This is a parallel implementation of the two Node patterns:
- `tauri-shell.js:85–94` — `loadShellPlugin()`: checks `globalThis.__TAURI_SHELL_MOCK__` for test injection, then dynamically requires `@tauri-apps/plugin-shell`.
- `tauri-shell.js:103–111` — `tauriExec(command, args, options)`: maps to `cp.exec` / `cp.execFile`. Calls `plugin.Command.create(...).execute()` and returns `{stdout, stderr}`.
- `tauri-shell.js:120–148` — `tauriSpawn(command, args, options)`: maps to `cp.spawn` for streaming. Builds `TauriReadable` streams and a `MiniEmitter` for `close`/`error`, kicks off `_spawnAsync` asynchronously, and returns an `IChildProcessLike` object with `stdout`, `stderr`, `stdin: null`, `on`, `kill`.
- `tauri-shell.js:149–173` — `_spawnAsync`: wires Tauri event callbacks to streams, appends `'\n'` to each line of stdout/stderr to preserve line boundaries, calls `cmd.spawn()` to get the Tauri child handle.

### `src/ipc/ipcServer.ts` — IPC server for askpass and git-editor

- `ipcServer.ts:15–25` — `getIPCHandlePath(id)`: computes a Unix domain socket path. On Windows: `\\\\.\\pipe\\vscode-git-${id}-sock`. On Linux with `XDG_RUNTIME_DIR`: `$XDG_RUNTIME_DIR/vscode-git-${id}.sock`. Otherwise: `os.tmpdir()/vscode-git-${id}.sock`.
- `ipcServer.ts:31–61` — `createIPCServer(context?)`: creates a `http.Server`, generates a SHA-256 hash (from random bytes or `context`), computes the socket path, unlinks stale socket on non-Windows, listens, returns an `IPCServer` instance.
- `ipcServer.ts:69–126` — `IPCServer`: holds a `Map<string, IIPCHandler>` at line 71. `registerHandler(name, handler)` at line 78 keys handlers by `/${name}`. `onRequest` at line 83 reads `req.url`, looks up handler, accumulates body chunks, parses JSON, calls `handler.handle(request)`, writes JSON response back. `getEnv()` at line 110 returns `{ VSCODE_GIT_IPC_HANDLE: this.ipcHandlePath }` — injected into every spawned git process and into terminal environments.

### `src/ipc/ipcClient.ts` — IPC client (run inside askpass-main.js subprocess)

- `ipcClient.ts:12–19` — constructor reads `VSCODE_GIT_IPC_HANDLE` from environment; throws if absent.
- `ipcClient.ts:22–44` — `call(request)`: sends HTTP POST via `socketPath: this.ipcHandlePath` to `/${handlerName}`, writes JSON body, resolves with parsed JSON response.

### `src/askpass-main.ts` — Askpass helper entry point (separate Node.js process)

- `askpass-main.ts:15–43` — `main(argv)`: reads `VSCODE_GIT_ASKPASS_PIPE` (output file path), `VSCODE_GIT_ASKPASS_TYPE` (`'https'` or `'ssh'`). Bails if `VSCODE_GIT_COMMAND === 'fetch'` and `VSCODE_GIT_FETCH_SILENT` is set. Constructs `IPCClient('askpass')`, calls `ipcClient.call({ askpassType, argv })`, writes the result string to `VSCODE_GIT_ASKPASS_PIPE` via `fs.writeFileSync`, then `process.exit(0)`.

### `src/askpass.ts` — Askpass IPC handler (inside extension host)

- `askpass.ts:13` — `Askpass` implements `IIPCHandler` and `ITerminalEnvironmentProvider`.
- `askpass.ts:28–29` — constructor registers itself with the IPC server as handler `'askpass'` via `ipc.registerHandler('askpass', this)`.
- `askpass.ts:35–48` — builds `this.env` with `GIT_ASKPASS` pointing to `askpassPaths.askpass` (or empty fallback), `VSCODE_GIT_ASKPASS_NODE` set to `process.execPath`, and `VSCODE_GIT_ASKPASS_MAIN` set to the askpass-main.js path. Also sets `SSH_ASKPASS` and `SSH_ASKPASS_REQUIRE: 'force'`.
- `askpass.ts:51–65` — `handle(payload)` dispatches on `payload.askpassType` to `handleAskpass` (HTTPS username/password via `window.showInputBox`) or `handleSSHAskpass` (passphrase or host authenticity via `window.showInputBox` / `window.showQuickPick`).

### `src/askpassManager.ts` — Stable askpass script paths on Windows

- `askpassManager.ts:33–50` — `isWindowsUserOrSystemSetup()`: reads `product.json` via `fs.readFileSync`, returns true if `target === 'user' || 'system'` (Inno Setup installs only).
- `askpassManager.ts:64–84` — `computeContentHash(sourcePaths)`: SHA-256 hashes all five script files (askpass.sh, askpass-main.js, ssh-askpass.sh, askpass-empty.sh, ssh-askpass-empty.sh) in fixed order, returns first 16 hex chars.
- `askpassManager.ts:90–115` — `setWindowsPermissions(filePath)`: calls `cp.execFile('icacls', ...)` to set ACL (`/inheritance:r /grant:r "${username}:F"`).
- `askpassManager.ts:215–283` — `ensureAskpassScripts(sourceDir, storageDir, logger)`: creates a content-addressed directory `storageDir/askpass/${hash}/`, copies all five scripts using `fs.promises.writeFile` + `setWindowsPermissions`, runs garbage collection of directories older than 7 days.
- `askpassManager.ts:290–312` — `getAskpassPaths(sourceDir, storagePath, logger)`: calls `ensureAskpassScripts` on Windows user/system installs, falls back to direct source-directory paths otherwise.

### `src/main.ts` — Extension activation and wiring

- `main.ts:67–76` — `createModel`: calls `createIPCServer(context.storagePath)` (uses `storagePath` as context for deterministic hash), then `getAskpassPaths(__dirname, context.globalStorageUri.fsPath, logger)`, then `new Askpass(ipcServer, logger, askpassPaths)`.
- `main.ts:79–93` — Constructs `GitEditor(ipcServer)`, merges all environment dictionaries (`askpass.getEnv()`, `gitEditor.getEnv()`, `ipcServer.getEnv()`) into a single `environment` object, passes it to `new Git({..., env: environment})`.
- `main.ts:83` — `TerminalEnvironmentManager(context, [askpass, gitEditor, ipcServer])` propagates the IPC handle and askpass paths into integrated terminal sessions.

### `src/repository.ts` — SCM API wiring

- `repository.ts:984` — `scm.createSourceControl('git', 'Git', root, icon, this._isHidden, parent)`: creates the VS Code SCM provider for a repository, binding to the root URI.
- `repository.ts:987–988` — Assigns `GitQuickDiffProvider` and `StagedResourceQuickDiffProvider` to `_sourceControl.quickDiffProvider` / `secondaryQuickDiffProvider`.
- `repository.ts:990–996` — Creates `GitHistoryProvider` and assigns to `_sourceControl.historyProvider`; creates `GitArtifactProvider` and assigns to `_sourceControl.artifactProvider`.
- `repository.ts:998` — Sets `_sourceControl.acceptInputCommand` to `{command: 'git.commit', ...}`.
- `repository.ts:1006–1009` — Creates four resource groups: `merge`, `index`, `workingTree`, `untracked` via `_sourceControl.createResourceGroup`.

### `src/api/api1.ts` — Public extension API layer

- `api1.ts:8` — `ApiRepository` implements the public `Repository` interface from `git.d.ts`, wrapping the internal `BaseRepository`.
- `api1.ts:88–100` — Constructor assigns `rootUri`, `inputBox`, `state` (as `ApiRepositoryState`), `ui` (as `ApiRepositoryUIState`). `onDidCommit` is derived via `filterEvent` on `onDidRunOperation` checking for `OperationKind.Commit` at line 98–99.
- `api1.ts:38–61` — `ApiRepositoryState`: proxies `HEAD`, `remotes`, `submodules`, `worktrees`, `rebaseCommit`, and change groups from the internal repository.

### `src/api/git.d.ts` — Public API contract

- `git.d.ts:9–11` — `Git` interface: only `path: string`.
- `git.d.ts:17–21` — `ForcePushMode` const enum.
- `git.d.ts:23–27` — `RefType` const enum (`Head`, `RemoteHead`, `Tag`).
- `git.d.ts:87–121` — `Change` interface with `uri`, `originalUri`, `renameUri`, `status`.
- `git.d.ts` also declares `Repository`, `API`, `RepositoryState`, `Branch`, `Remote`, `Commit`, `Worktree`, `CredentialsProvider`, `PushErrorHandler`, `PostCommitCommandsProvider`, `BranchProtectionProvider`, and the `GitExtension` interface exported as the extension's main contribution.

---

## Cross-Cutting Synthesis (≤200 words)

The `extensions/git/` partition has three Node.js-specific pillars Tauri must replace:

**1. Subprocess execution (`child_process`)**
Every git command flows through `Git.spawn()` (`git.ts:676`) which calls `cp.spawn`. The migration extracts this behind a `IGitProcessLauncher` / `IGitChild` interface (factory at `processLauncher.factory.js`). `NodeGitProcessLauncher` retains `cp.spawn`; `TauriGitProcessLauncher` routes through `@tauri-apps/plugin-shell Command.create().spawn()`. The factory selects via `globalThis.__TAURI_INTERNALS__` presence. Git binary discovery (`findGitDarwin`, `findGitWin32`) also uses `cp.exec` / `cp.spawn` at `git.ts:87–134` and must be ported to Tauri shell invocations. `stdin` is currently `null` in `TauriGitChild` — any operation writing to stdin (e.g., `git commit` with message via `options.input`) has no Tauri path yet.

**2. IPC askpass (Unix socket / Windows named pipe)**
`IPCServer` (`ipcServer.ts`) runs an `http.Server` over a domain socket. Git subprocesses reach back via `VSCODE_GIT_IPC_HANDLE`. `askpass-main.ts` is a standalone Node.js process that uses `IPCClient` over the same socket. In Tauri, native `http` sockets from extension host to a separate helper process are unavailable; Tauri's own IPC invoke system or a custom Tauri command must replace the HTTP-over-socket pattern.

**3. Windows ACL management**
`askpassManager.ts:103` calls `cp.execFile('icacls', ...)` for file permission hardening — a direct Node child_process dependency that must become a Rust Tauri command.

The SCM API (`scm.createSourceControl`, resource groups, history/artifact providers in `repository.ts`) is VS Code API surface only and is unaffected by Tauri migration.

---

## Out-of-Partition References

- `src/terminal.ts` — `ITerminalEnvironmentProvider` interface implemented by `Askpass` and `IPCServer`; `TerminalEnvironmentManager` and `TerminalShellExecutionManager` consume `ipcServer.getEnv()`.
- `src/gitEditor.ts` — `GitEditor` registers a handler on the IPC server (receives IPC handle, parallels askpass wiring in `main.ts:79`).
- `src/util.ts` — `assign`, `IDisposable`, `toDisposable`, `onceEvent`, `Limiter`, `Versions` used throughout `git.ts`.
- `src/askpass.ts` (shell scripts) — `askpass.sh`, `ssh-askpass.sh`, `askpass-empty.sh`, `ssh-askpass-empty.sh` are shell scripts in `extensions/git/` that set env vars (`VSCODE_GIT_ASKPASS_PIPE`, `VSCODE_GIT_ASKPASS_TYPE`) and invoke `node askpass-main.js`; Tauri cannot run shell scripts natively.
- `@tauri-apps/plugin-shell` — External Tauri plugin, source outside this partition; its `Command.create().spawn()` and `.execute()` are the replacement surface.
- `vscode` extension host API — `scm.createSourceControl`, `window.showInputBox`, `window.showQuickPick`, `workspace.getConfiguration` in `repository.ts`, `askpass.ts`, `model.ts` are VS Code host bindings; unchanged by the Tauri migration but require the extension host to remain present.
- `src/commands.ts` — `CommandCenter` registered in `main.ts:115` uses the `Git` class and `Model`; all command handlers that spawn subprocesses will transitively consume the process launcher abstraction.
- `src/cloneManager.ts` — `CloneManager` uses `Model` and passes `onSpawn` callbacks to `Git.exec` for progress reporting; the `onSpawn` callback receives `cp.ChildProcess` (or its `IGitChild` abstraction) and attaches `byline.LineStream` to `stderr`.
- `byline` npm package — used in `git.ts:18` and the `clone` method's `onSpawn` at `git.ts:453`; not available in the Tauri webview context without bundling.
