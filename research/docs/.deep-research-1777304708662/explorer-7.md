# Partition 7 of 79 — Findings

## Scope
`extensions/git/` (62 files, 25,181 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Partition 7: extensions/git/ — Source Control Management (SCM) Integration

## Summary
Comprehensive mapping of VS Code's built-in Git extension (`extensions/git/`), a reference implementation of the SCM API. This extension manages repository operations via `child_process` spawning and provides the public SCM API surface that extensions consume. Key for understanding how a Tauri-based port would expose process spawning and repository management.

---

### Implementation

#### Core Git Process & Execution
- `extensions/git/src/git.ts` — Git CLI wrapper; defines `IGit`, `IExecutionResult<T>`, `SpawnOptions` interface with `onSpawn` callback; main execution flow via `cp.spawn()` and `cp.exec()` (3,431 lines)
- `extensions/git/src/repository.ts` — Main `Repository` class implementing `Disposable`; manages per-repo state via `scm.createSourceControl()` at line 984; coordinates all git operations (3,537 lines)
- `extensions/git/src/model.ts` — Top-level `Model` class; manages collection of repositories, implements `IRepositoryResolver` (1,223 lines)
- `extensions/git/src/main.ts` — Extension entrypoint; instantiates `Git` class from git.ts, creates IPC server, sets up askpass environment (327 lines)

#### Repository & State Management
- `extensions/git/src/cache.ts` — Caching layer for git metadata (485 lines)
- `extensions/git/src/repositoryCache.ts` — Repository caching and memory management (228 lines)
- `extensions/git/src/operation.ts` — Operation queuing and status tracking (290 lines)

#### Git-facing Auxiliary Systems
- `extensions/git/src/askpassManager.ts` — Credential/password prompt handling via spawned askpass subprocess (312 lines)
- `extensions/git/src/askpass.ts` — VSCode extension side of SSH_ASKPASS protocol (imports `child_process`) (5,737 bytes)
- `extensions/git/src/askpass-main.ts` — Spawned askpass subprocess entry point (1,473 bytes)
- `extensions/git/src/gitEditor.ts` — Git editor (GIT_EDITOR) subprocess handling (259 bytes)
- `extensions/git/src/git-editor-main.ts` — Spawned git editor subprocess entry point (759 bytes)

#### IPC & Terminal Integration
- `extensions/git/src/ipc/ipcServer.ts` — IPC server for askpass/git-editor communication (126 lines)
- `extensions/git/src/ipc/ipcClient.ts` — IPC client stub (45 lines)
- `extensions/git/src/terminal.ts` — Terminal environment provider; injects git-related env vars (92 lines)

#### SCM UI & Providers
- `extensions/git/src/commands.ts` — Command center; wires all git commands to VSCode UI (5,774 lines)
- `extensions/git/src/actionButton.ts` — SCM action button (commit/sync) implementation (322 lines)
- `extensions/git/src/decorationProvider.ts` — File decoration provider (git status icons) (342 lines)
- `extensions/git/src/staging.ts` — Index staging state management (239 lines)
- `extensions/git/src/quickDiffProvider.ts` — Quick diff provider for inline diffs (120 lines)
- `extensions/git/src/timelineProvider.ts` — Timeline view provider for commit history (329 lines)
- `extensions/git/src/historyProvider.ts` — Source history view provider (612 lines)
- `extensions/git/src/historyItemDetailsProvider.ts` — History details provider registry (62 lines)

#### Advanced Features
- `extensions/git/src/blame.ts` — Blame information cache and provider (755 lines)
- `extensions/git/src/hover.ts` — Hover information for git URIs (181 lines)
- `extensions/git/src/fileSystemProvider.ts` — Virtual filesystem for git:// URIs (259 lines)
- `extensions/git/src/artifactProvider.ts` — Source control artifact provider (197 lines)
- `extensions/git/src/branchProtection.ts` — Branch protection provider registry (12 lines)
- `extensions/git/src/editSessionIdentityProvider.ts` — Edit session identity for cloud edits (13 lines)
- `extensions/git/src/cloneManager.ts` — Clone repository workflow manager (247 lines)
- `extensions/git/src/autofetch.ts` — Auto-fetch background task (5 lines)
- `extensions/git/src/postCommitCommands.ts` — Post-commit action hooks (239 lines)
- `extensions/git/src/diagnostics.ts` — Commit message diagnostics/code actions (228 lines)
- `extensions/git/src/protocolHandler.ts` — vscode:// URL protocol handler (98 lines)
- `extensions/git/src/remoteSource.ts` — Remote source provider (17 lines)
- `extensions/git/src/remotePublisher.ts` — Remote publisher registry (15 lines)
- `extensions/git/src/pushError.ts` — Push error handler registry (12 lines)
- `extensions/git/src/statusbar.ts` — Status bar sync/branch/upstream indicators (316 lines)
- `extensions/git/src/watch.ts` — .git folder watcher (22 lines)
- `extensions/git/src/decorators.ts` — Decorator utility functions (11 lines)
- `extensions/git/src/emoji.ts` — Emoji helper for commit messages (14 lines)
- `extensions/git/src/uri.ts` — Git URI parsing/construction (88 lines)
- `extensions/git/src/util.ts` — Shared utilities (872 lines)

#### Git Base Integration (Remote Sourcing)
- `extensions/git/src/git-base.ts` — Remote source provider API integration (1,143 bytes)

---

### Tests

- `extensions/git/src/test/git.test.ts` — Unit tests for GitStatusParser, parseGitCommits, parseGitmodules, parseLsTree, parseLsFiles, parseGitRemotes, parseCoAuthors (720 lines)
- `extensions/git/src/test/askpassManager.test.ts` — Askpass manager tests (203 lines)
- `extensions/git/src/test/repositoryCache.test.ts` — Repository cache tests (197 lines)
- `extensions/git/src/test/smoke.test.ts` — Smoke/integration tests (178 lines)
- `extensions/git/src/test/index.ts` — Test runner configuration; runs in multiple environments (Electron, Browser, Remote)

---

### Types / Interfaces

#### Public API
- `extensions/git/src/api/git.d.ts` — Public Git extension API; defines `Git`, `InputBox`, `Ref`, `Branch`, `Remote`, `Worktree`, `Commit`, `Submodule`, `ForcePushMode`, `RefType`, `Status` enums (514 lines)
- `extensions/git/src/api/git.constants.ts` — Git error codes and constants (3,326 bytes)
- `extensions/git/src/api/extension.ts` — GitExtension public interface (2,602 bytes)
- `extensions/git/src/api/api1.ts` — API version 1 implementation (604 lines)

#### Base Extension API
- `extensions/git/src/typings/git-base.d.ts` — RemoteSourceProvider, RemoteSource, PickRemoteSourceOptions types for git-base extension integration (86 lines)

---

### Configuration

- `extensions/git/package.json` — Extension manifest; defines ~80 `git.*` settings (autofetch, path, requireGitUserConfig, branchProtection, postCommitCommand, timeline options, etc.); 15+ SCM commands; colors and UI contributions (v10.0.0)
- `extensions/git/tsconfig.json` — TypeScript compilation config (target: ES2022)
- `extensions/git/.npmrc` — NPM registry configuration
- `extensions/git/.vscodeignore` — Files excluded from .vsix package
- `extensions/git/package-lock.json` — Dependency lock file

---

### Examples / Fixtures

- `extensions/git/resources/emojis.json` — Emoji mapping for commit messages

---

### Documentation

- `extensions/git/README.md` — Extension overview; API usage guide for extension authors; references `src/api/git.d.ts` as the public type definitions

---

### Shell Scripts & Subprocess Helpers

- `extensions/git/src/askpass.sh` — SSH askpass shell wrapper (296 bytes)
- `extensions/git/src/askpass-empty.sh` — Empty askpass stub (17 bytes)
- `extensions/git/src/git-editor.sh` — Git editor shell wrapper (125 bytes)
- `extensions/git/src/git-editor-empty.sh` — Empty git-editor stub (10 bytes)
- `extensions/git/src/ssh-askpass.sh` — SSH askpass bridge (13 bytes)
- `extensions/git/src/ssh-askpass-empty.sh` — Empty SSH askpass stub (13 bytes)

---

### Notable Clusters

#### Git Execution Layer
- `extensions/git/src/git.ts` alone contains 3,431 lines and implements:
  - `findGit()` / `findGitDarwin()` / `findGitWin32()` — Platform-specific git discovery
  - `Git` class with public `exec()` method wrapping `cp.spawn()`
  - `SpawnOptions` interface with `onSpawn` callback (line 207) — allows extensions to hook child processes
  - `IExecutionResult<T>` with `exitCode`, `stdout`, `stderr`
  - `GitError` class with rich error context (command, args, git error codes)

#### Repository & Operations
- `extensions/git/src/repository.ts` (3,537 lines) + `extensions/git/src/model.ts` (1,223 lines) = ~4,760 lines
  - Implements full repository model: HEAD, branches, remotes, submodules, worktrees, status
  - Calls `scm.createSourceControl('git', ...)` at line 984 in repository.ts

#### SCM UI Integration
- `extensions/git/src/commands.ts` (5,774 lines) — Single command center integrating all git operations with VSCode UI
  - Handles pull, push, fetch, commit, branch, rebase, stash, merge, cherry-pick, etc.

#### Credential/Auth Subprocess Layer
- `extensions/git/src/askpassManager.ts` (312 lines) + askpass.ts/git-editor.ts + associated .sh wrappers
  - IPC-based out-of-process credential handling
  - `child_process` spawning of helper binaries

#### Tauri Migration Artifacts (Pre-Implementation)
- `extensions/git/out/git/tauriProcessLauncher.js` — Compiled Tauri process launcher (emulating Node ChildProcess with MiniEventEmitter)
- `extensions/git/out/tauri-shell.js` — Compiled tauri spawn/exec wrappers (exports `tauriExec`, `tauriSpawn`, emulating Node streams)
- `extensions/git/out/git/processLauncher.factory.js` — Factory selecting Node vs Tauri launcher
- `extensions/git/out/git/nodeProcessLauncher.js` — Node-based process launcher for reference
- `extensions/git/out/test/tauriShell.test.js` — Tests for Tauri shell wrapper
- `extensions/git/out/test/processLauncher.test.js` — Process launcher abstraction tests

These compiled JS files (in `out/git/` and `out/`) indicate active work on Tauri platform support, with abstraction layers to bridge Node's `child_process` API to Tauri's command execution model.

---

### Entry Points & Critical Paths

- `extensions/git/src/main.ts` — Extension activation; orchestrates Git discovery → Model creation → IPC server → Command registration
- `extensions/git/src/repository.ts` line 984 — SCM source control creation
- `extensions/git/src/git.ts` line 87–91, 210–270 — Core `cp.spawn()` and execution promise handling
- `extensions/git/src/askpassManager.ts` — Subprocess credential management

---

### File Statistics

Total TypeScript source files: 54 (.ts files)
- Largest: `repository.ts` (3,537 lines), `git.ts` (3,431 lines), `commands.ts` (5,774 lines)
- Total implementation + test LOC: ~25,025 lines (from total count; excludes shell scripts and data files)

---

### Key Insight for Tauri Port

The git extension demonstrates the two critical surfaces a Tauri port must handle:

1. **Process Spawning (SpawnOptions → cp.ChildProcess)**: The `Git.exec()` method and `onSpawn` callback pattern (git.ts:207) show how extensions hook process lifecycle. Tauri's command system must expose an equivalent API.

2. **SCM API (scm.createSourceControl)**: VSCode's built-in SCM API is the contract. The repository.ts and model.ts code show what a Tauri IDE core must provide to extensions. This is the abstraction layer, not the git-specific layer.

3. **Credential/Auth Out-of-Process**: The askpassManager + IPC layer shows that subprocess helpers for SSH/GPG are essential. Tauri must support spawning and IPC-communicating with native binaries.

4. **Active Tauri Support**: The presence of `tauriProcessLauncher.js`, `tauri-shell.js`, and related test files in `out/` indicates the codebase already has infrastructure for Tauri backend swapping, suggesting earlier phase work on abstraction.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Git Extension: Porting Patterns from TypeScript/Electron to Tauri/Rust

## Overview
Analysis of the git extension (`extensions/git/`) to identify code patterns relevant to porting VS Code's source control functionality from TypeScript/Electron to Tauri/Rust.

---

## Core Patterns Found

#### Pattern: Child Process Spawning with Streaming I/O

**Where:** `extensions/git/src/git.ts:676-702`

**What:** Git commands are executed by spawning child processes with environment setup, stdio configuration, and error handling.

```typescript
spawn(args: string[], options: SpawnOptions = {}): cp.ChildProcess {
    if (!this.path) {
        throw new Error('git could not be found in the system.');
    }

    if (!options.stdio && !options.input) {
        options.stdio = ['ignore', null, null];
    }

    options.env = assign({}, process.env, this.env, options.env || {}, {
        VSCODE_GIT_COMMAND: args[0],
        LANGUAGE: 'en',
        LC_ALL: 'en_US.UTF-8',
        LANG: 'en_US.UTF-8',
        GIT_PAGER: 'cat'
    });

    const cwd = this.getCwd(options);
    if (cwd) {
        options.cwd = sanitizePath(cwd);
    }

    return cp.spawn(this.path, args, options);
}
```

**Variations / call-sites:**
- `extensions/git/src/git.ts:604-616` - Stream wrapper for long-running processes
- `extensions/git/src/git.ts:595-602` - Direct `exec` and `exec2` methods

---

#### Pattern: Async Process Execution with Cancellation Support

**Where:** `extensions/git/src/git.ts:210-270`

**What:** Promise-based wrapper around child processes supporting cancellation tokens, output buffering, and error extraction.

```typescript
async function exec(child: cp.ChildProcess, cancellationToken?: CancellationToken): Promise<IExecutionResult<Buffer>> {
    if (!child.stdout || !child.stderr) {
        throw new GitError({ message: 'Failed to get stdout or stderr from git process.' });
    }

    if (cancellationToken && cancellationToken.isCancellationRequested) {
        throw new CancellationError();
    }

    const disposables: IDisposable[] = [];
    let result = Promise.all<any>([
        new Promise<number>((c, e) => {
            once(child, 'error', cpErrorHandler(e));
            once(child, 'exit', c);
        }),
        new Promise<Buffer>(c => {
            const buffers: Buffer[] = [];
            on(child.stdout!, 'data', (b: Buffer) => buffers.push(b));
            once(child.stdout!, 'close', () => c(Buffer.concat(buffers)));
        }),
        new Promise<string>(c => {
            const buffers: Buffer[] = [];
            on(child.stderr!, 'data', (b: Buffer) => buffers.push(b));
            once(child.stderr!, 'close', () => c(Buffer.concat(buffers).toString('utf8')));
        })
    ]) as Promise<[number, Buffer, string]>;

    if (cancellationToken) {
        const cancellationPromise = new Promise<[number, Buffer, string]>((_, e) => {
            onceEvent(cancellationToken.onCancellationRequested)(() => {
                try {
                    child.kill();
                } catch (err) {
                    // noop
                }
                e(new CancellationError());
            });
        });
        result = Promise.race([result, cancellationPromise]);
    }

    try {
        const [exitCode, stdout, stderr] = await result;
        return { exitCode, stdout, stderr };
    } finally {
        dispose(disposables);
    }
}
```

**Variations / call-sites:**
- `extensions/git/src/git.ts:618-674` - High-level `_exec` wrapper with logging and error code detection

---

#### Pattern: Process Spawn Callback for Real-Time Monitoring

**Where:** `extensions/git/src/git.ts:203-208`

**What:** Optional `onSpawn` callback allows callers to attach listeners to process streams immediately after spawn.

```typescript
export interface SpawnOptions extends cp.SpawnOptions {
    input?: string;
    log?: boolean;
    cancellationToken?: CancellationToken;
    onSpawn?: (childProcess: cp.ChildProcess) => void;
}

// Usage at git.ts:621
options.onSpawn?.(child);

// Example implementation at git.ts:451-476
const onSpawn = (child: cp.ChildProcess) => {
    const decoder = new StringDecoder('utf8');
    const lineStream = new byline.LineStream({ encoding: 'utf8' });
    child.stderr!.on('data', (buffer: Buffer) => lineStream.write(decoder.write(buffer)));

    let totalProgress = 0;
    let previousProgress = 0;

    lineStream.on('data', (line: string) => {
        let match: RegExpExecArray | null = null;
        if (match = /Counting objects:\s*(\d+)%/i.exec(line)) {
            totalProgress = Math.floor(parseInt(match[1]) * 0.1);
        } else if (match = /Compressing objects:\s*(\d+)%/i.exec(line)) {
            totalProgress = 10 + Math.floor(parseInt(match[1]) * 0.1);
        } else if (match = /Receiving objects:\s*(\d+)%/i.exec(line)) {
            totalProgress = 20 + Math.floor(parseInt(match[1]) * 0.4);
        } else if (match = /Resolving deltas:\s*(\d+)%/i.exec(line)) {
            totalProgress = 60 + Math.floor(parseInt(match[1]) * 0.4);
        }

        if (totalProgress !== previousProgress) {
            options.progress.report({ increment: totalProgress - previousProgress });
            previousProgress = totalProgress;
        }
    });
};
```

**Variations / call-sites:**
- `extensions/git/src/git.ts:487-491` - Clone operation progress tracking

---

#### Pattern: Source Control Model with Resource Groups

**Where:** `extensions/git/src/repository.ts:984-1009`

**What:** Creates SCM source control with multiple resource groups (merge, index, working tree, untracked) via VS Code's SCM API.

```typescript
const root = Uri.file(repository.root);
this._sourceControl = scm.createSourceControl('git', 'Git', root, icon, this._isHidden, parent);
this._sourceControl.contextValue = repository.kind;

this._sourceControl.quickDiffProvider = new GitQuickDiffProvider(this, this.repositoryResolver, logger);
this._sourceControl.secondaryQuickDiffProvider = new StagedResourceQuickDiffProvider(this, logger);

this._historyProvider = new GitHistoryProvider(historyItemDetailProviderRegistry, this, logger);
this._sourceControl.historyProvider = this._historyProvider;
this.disposables.push(this._historyProvider);

this._artifactProvider = new GitArtifactProvider(this, logger);
this._sourceControl.artifactProvider = this._artifactProvider;

this._sourceControl.acceptInputCommand = { command: 'git.commit', title: l10n.t('Commit'), arguments: [this._sourceControl] };
this._sourceControl.inputBox.validateInput = this.validateInput.bind(this);

this._mergeGroup = this._sourceControl.createResourceGroup('merge', l10n.t('Merge Changes'));
this._indexGroup = this._sourceControl.createResourceGroup('index', l10n.t('Staged Changes'), { multiDiffEditorEnableViewChanges: true });
this._workingTreeGroup = this._sourceControl.createResourceGroup('workingTree', l10n.t('Changes'), { multiDiffEditorEnableViewChanges: true });
this._untrackedGroup = this._sourceControl.createResourceGroup('untracked', l10n.t('Untracked Changes'), { multiDiffEditorEnableViewChanges: true });
```

**Variations / call-sites:**
- `extensions/git/src/repository.ts:700-850` - Repository class definition with SCM integration

---

#### Pattern: IPC Server for External Process Communication

**Where:** `extensions/git/src/ipc/ipcServer.ts:31-61`

**What:** HTTP-based IPC server using Unix domain sockets (or named pipes on Windows) for cross-process communication.

```typescript
export async function createIPCServer(context?: string): Promise<IPCServer> {
    const server = http.createServer();
    const hash = crypto.createHash('sha256');

    if (!context) {
        const buffer = await new Promise<Buffer>((c, e) => crypto.randomBytes(20, (err, buf) => err ? e(err) : c(buf)));
        hash.update(buffer);
    } else {
        hash.update(context);
    }

    const ipcHandlePath = getIPCHandlePath(hash.digest('hex').substring(0, 10));

    if (process.platform !== 'win32') {
        try {
            await fs.promises.unlink(ipcHandlePath);
        } catch {
            // noop
        }
    }

    return new Promise((c, e) => {
        try {
            server.on('error', err => e(err));
            server.listen(ipcHandlePath);
            c(new IPCServer(server, ipcHandlePath));
        } catch (err) {
            e(err);
        }
    });
}
```

**Variations / call-sites:**
- `extensions/git/src/ipc/ipcServer.ts:69-126` - Request handler registration and message processing
- `extensions/git/src/main.ts:70-73` - Server creation in main activation

---

#### Pattern: File System Watching with Regex Filtering

**Where:** `extensions/git/src/repository.ts:920-942`

**What:** Composite file watchers for repository and `.git` directory with regex-based event filtering.

```typescript
const repositoryWatcher = workspace.createFileSystemWatcher(new RelativePattern(Uri.file(repository.root), '**'));
this.disposables.push(repositoryWatcher);

const onRepositoryFileChange = anyEvent(repositoryWatcher.onDidChange, repositoryWatcher.onDidCreate, repositoryWatcher.onDidDelete);
const onRepositoryWorkingTreeFileChange = filterEvent(onRepositoryFileChange, uri => !/\.git($|\\|\/)/.test(relativePath(repository.root, uri.fsPath)));

let onRepositoryDotGitFileChange: Event<Uri>;

try {
    const dotGitFileWatcher = new DotGitWatcher(this, logger);
    onRepositoryDotGitFileChange = dotGitFileWatcher.event;
    this.disposables.push(dotGitFileWatcher);
} catch (err) {
    logger.error(`Failed to watch path:'${this.dotGit.path}' or commonPath:'${this.dotGit.commonPath}', reverting to legacy API file watched. Some events might be lost.\n${err.stack || err}`);
    onRepositoryDotGitFileChange = filterEvent(onRepositoryFileChange, uri => /\.git($|\\|\/)/.test(uri.path));
}

const onFileChange = anyEvent(onRepositoryWorkingTreeFileChange, onRepositoryDotGitFileChange);
onFileChange(this.onFileChange, this, this.disposables);
```

**Variations / call-sites:**
- `extensions/git/src/repository.ts:453-503` - DotGitWatcher implementation with fallback

---

#### Pattern: Git Configuration Parsing

**Where:** `extensions/git/src/git.ts:783-818`

**What:** Regex-based INI-style config file parser for git configuration.

```typescript
class GitConfigParser {
    private static readonly _lineSeparator = /\r?\n/;
    private static readonly _propertyRegex = /^\s*(\w+)\s*=\s*"?([^"]+)"?$/;
    private static readonly _sectionRegex = /^\s*\[\s*([^\]]+?)\s*(\"[^"]+\")*\]\s*$/;

    static parse(raw: string): GitConfigSection[] {
        const config: { sections: GitConfigSection[] } = { sections: [] };
        let section: GitConfigSection = { name: 'DEFAULT', properties: {} };

        const addSection = (section?: GitConfigSection) => {
            if (!section) { return; }
            config.sections.push(section);
        };

        for (const line of raw.split(GitConfigParser._lineSeparator)) {
            const sectionMatch = line.match(GitConfigParser._sectionRegex);
            if (sectionMatch?.length === 3) {
                addSection(section);
                section = { name: sectionMatch[1], subSectionName: sectionMatch[2]?.replaceAll('"', ''), properties: {} };
                continue;
            }

            const propertyMatch = line.match(GitConfigParser._propertyRegex);
            if (propertyMatch?.length === 3 && !Object.keys(section.properties).includes(propertyMatch[1])) {
                section.properties[propertyMatch[1]] = propertyMatch[2];
            }
        }

        addSection(section);
        return config.sections;
    }
}
```

**Variations / call-sites:**
- `extensions/git/src/git.ts:583-593` - Config parsing in dotGit detection

---

#### Pattern: Environment Variable Management for External Processes

**Where:** `extensions/git/src/git.ts:689-695`

**What:** Merge process environment with git-specific overrides, ensuring consistent encoding and pager behavior.

```typescript
options.env = assign({}, process.env, this.env, options.env || {}, {
    VSCODE_GIT_COMMAND: args[0],
    LANGUAGE: 'en',
    LC_ALL: 'en_US.UTF-8',
    LANG: 'en_US.UTF-8',
    GIT_PAGER: 'cat'
});
```

**Variations / call-sites:**
- `extensions/git/src/git.ts:488-490` - Custom user agent injection for HTTP operations
- `extensions/git/src/git.ts:2141` - GIT_EDITOR override for specific operations

---

## Integration Patterns

### Extension Activation Flow
- `extensions/git/src/main.ts:192-251` - Extension activation with Model creation, configuration loading, and capability detection

### Event-Driven Architecture
- Repository state changes trigger updates via `EventEmitter` and VS Code's `Event` system
- File watchers debounced and filtered for `.git` directory changes
- Operations tracked through `OperationManager` with distinct operation types

### Error Handling Strategy
- Git-specific error codes detected from stderr patterns (`extensions/git/src/git.ts:329-364`)
- Custom `GitError` class maintains command, args, stdout, stderr, exit code
- Configuration trust-based path filtering for untrusted workspaces

### Key Dependencies
- `child_process` (cp) - Process spawning and stream handling
- `byline` - Streaming line parser for progress tracking
- `file-type` - Binary file detection
- VS Code's SCM, workspace, window APIs for UI integration
- HTTP for IPC (not IPC/named pipes library)

---

## Rust/Tauri Translation Challenges Identified

1. **Child Process Stream Handling**: The pattern of attaching listeners to stdout/stderr after spawn with real-time buffering and progress parsing requires non-blocking I/O patterns.

2. **Environment Variable Layering**: Complex merging of process.env with custom overrides requires careful environment setup before spawning.

3. **IPC over HTTP**: Using HTTP with Unix domain sockets requires implementing HTTP server in Rust, or switching to more native IPC (e.g., tokio channels, tauri invoke).

4. **Regex-based Configuration Parsing**: Git config parsing relies on regex. Rust equivalent would need `regex` crate.

5. **FileSystemWatcher with Filtering**: VS Code's watcher abstraction with event filtering would need translation to notify/watchman equivalents.

6. **Event System**: The layered EventEmitter pattern (Node.js) + VS Code Events would need async/await equivalents in Rust.

7. **Resource Lifecycle**: Disposable pattern (cleanup registration) is fundamental; Rust Drop trait handles some, but explicit resource management patterns needed.

## External References
<!-- Source: codebase-online-researcher sub-agent -->
# Partition 7: `extensions/git/` — Tauri Shell Plugin Research

## Scope

This document covers `extensions/git/src/git.ts` (Node `child_process` usage), the two Tauri port artifacts that already exist in `extensions/git/out/` (`tauri-shell.js` and `git/tauriProcessLauncher.js`), and focused excerpts from the Tauri 2.x shell plugin documentation needed to complete and validate those artifacts.

---

#### Tauri Shell Plugin (v2.x)

**Docs:**
- https://v2.tauri.app/plugin/shell/ (overview, permissions table)
- https://v2.tauri.app/reference/javascript/shell/ (full JS API reference)
- https://raw.githubusercontent.com/tauri-apps/plugins-workspace/v2/plugins/shell/guest-js/index.ts (authoritative TypeScript source)
- https://v2.tauri.app/develop/sidecar/ (sidecar embedding guide)

**Relevant behaviour:**

### `Command.create(program, args?, options?)` — the Node `cp.spawn` analogue

Signature (from `guest-js/index.ts`, v2 branch):

```typescript
static create<O extends IOPayload>(
  program: string,
  args: string | string[] = [],
  options?: SpawnOptions
): Command<O>
```

- `program` is a **logical name** (scope identifier), not the raw filesystem path.
  Every name must be declared in `src-tauri/capabilities/*.json` under `shell:allow-execute`.
  The actual OS binary path is bound in the capability's `allow[].cmd` field.
- `SpawnOptions` fields: `cwd?: string`, `env?: Record<string,string>`, `encoding?: string`.
  `env` set to `null` clears the inherited process environment (important for VS Code's git
  env-injection pattern at `git.ts:689–695`).
- Returns a `Command<string>` (or `Command<Uint8Array>` when `encoding: 'raw'` is passed).
  The object is an `EventEmitter` subclass; no I/O starts until `.spawn()` or `.execute()`.

### `Command.sidecar(program, args?, options?)` — bundled binary variant

```typescript
static sidecar<O extends IOPayload>(
  program: string,
  args: string | string[] = [],
  options?: SpawnOptions
): Command<O>
```

- Identical signature to `create()`. Internally sets `options.sidecar = true` before
  passing the payload to `plugin:shell|spawn`.
- The `program` string must match the value in `tauri.conf.json > bundle > externalBin`
  (e.g. `"binaries/my-sidecar"`). Tauri resolves the arch-suffixed binary
  (`my-sidecar-aarch64-apple-darwin`) at runtime; the caller never manages path suffixes.
- **Not used by the current git port** (git is a system binary, not a bundled sidecar).
  `sidecar()` is relevant if VS Code ships its own git wrapper or the `askpass` helper
  as a compiled Rust sidecar rather than a shell script.

### `Command.spawn()` — streaming / long-lived processes

```typescript
async spawn(): Promise<Child>
```

- Calls `invoke('plugin:shell|spawn', { program, args, options, onEvent })` over IPC.
- Returns `Promise<Child>` where `Child.pid: number` identifies the process.
- **Streaming events** arrive via the `onEvent` Channel:
  - `'Stdout'` payload → forwarded to `command.stdout.emit('data', payload)`
  - `'Stderr'` payload → forwarded to `command.stderr.emit('data', payload)`
  - `'Terminated'` payload `{ code: number|null, signal: number|null }` → forwarded to
    `command.emit('close', payload)`
  - `'Error'` payload → forwarded to `command.emit('error', payload)`
- The stdout/stderr are **line-buffered strings by default** (one `data` event per line),
  unlike Node streams which emit raw `Buffer` chunks. The Tauri port adapters in `out/`
  compensate by appending `'\n'` to each emitted line (see `tauri-shell.js:155–158`).

### `Command.execute()` — fire-and-forget / short-lived processes

```typescript
async execute(): Promise<ChildProcess<O>>
```

- Calls `invoke('plugin:shell|execute', { program, args, options })`.
- Collects all output before resolving; returns `{ code, signal, stdout, stderr }`.
- Analogous to `cp.exec` / `cp.execFile`. Used in `tauri-shell.js:tauriExec()`.

### `Child.write(data)` — stdin writes

```typescript
async write(data: IOPayload | number[]): Promise<void>
// IOPayload = string | Uint8Array
```

- Calls `invoke('plugin:shell|stdin_write', { pid, buffer: data })`.
- Must hold the `Child` reference returned by `spawn()`.
- In the existing Node code (`git.ts:624`), stdin writes happen via
  `child.stdin!.end(options.input, 'utf8')`. Under Tauri, `Child` has no `.stdin` stream;
  writes are explicit async calls. The current `tauriProcessLauncher.js` exposes `stdin: null`
  (`tauriProcessLauncher.js:89`), so callers relying on `options.input` must be adapted.

### `Child.kill()` — process termination

```typescript
async kill(): Promise<void>
```

- Calls `invoke('plugin:shell|kill', { cmd: 'killChild', pid: this.pid })`.
- Returns `Promise<void>`; resolution indicates the OS kill signal was sent, not that the
  process has exited. Callers should await the `'close'` event on the `Command` for
  confirmation, just as with Node's `child.kill()` + `'exit'` event pattern.
- Requires `shell:allow-kill` permission in the capability configuration.
- The Tauri adapters fire-and-forget: `this._tauriChild.kill().catch(() => {})` (see
  `tauriProcessLauncher.js:106` and `tauri-shell.js:143`).

### Permissions model (critical difference from Node)

Every program that may be executed must be pre-declared in a capabilities JSON:

```json
{
  "permissions": [
    {
      "identifier": "shell:allow-execute",
      "allow": [
        {
          "name": "run-git",
          "cmd": "git",
          "args": true
        }
      ]
    },
    "shell:allow-spawn",
    "shell:allow-kill",
    "shell:allow-stdin-write"
  ]
}
```

- `name` is the logical identifier passed to `Command.create('run-git', ...)`.
- `args: true` allows any argument list; a restricted array can validate per-arg regexes.
- There is no runtime path resolution or `PATH` lookup from the frontend — the `cmd` value
  is resolved by the Rust backend under the security policy.
- The git extension uses many ad-hoc git subcommands with varying argument lists; the
  capability scope therefore needs `args: true` for the git command entry.

---

**Where used in `extensions/git/src/git.ts`:**

| Line(s) | Node API | What it does | Tauri equivalent |
|---------|----------|--------------|-----------------|
| `git.ts:9` | `import * as cp from 'child_process'` | Module import | Replace with `Command` from `@tauri-apps/plugin-shell` |
| `git.ts:87–91` | `cp.spawn(path, ['--version'])` | One-shot version probe; collects stdout buffers, resolves on `'close'` | `Command.create('git', ['--version']).execute()` → read `result.stdout` |
| `git.ts:96` | `cp.exec('which git', ...)` | Discover git path on macOS | `Command.create('which', ['git']).execute()` (or hard-code `cmd: 'which'` in scope) |
| `git.ts:109` | `cp.exec('git --version', ...)` | Confirm git is executable on macOS | `Command.create('git', ['--version']).execute()` |
| `git.ts:124` | `cp.exec('xcode-select -p', ...)` | Check for Xcode on macOS | `Command.create('xcode-select', ['-p']).execute()` |
| `git.ts:210–246` | `exec(child, cancellationToken)` | Collects stdout/stderr buffers from a live `cp.ChildProcess`; listens for `'data'`, `'close'`, `'error'` | For short-lived calls: replace with `Command.execute()`. For streaming: subscribe to `command.stdout.on('data', ...)` + `command.on('close', ...)` as implemented in `tauriProcessLauncher.js:144–161`. |
| `git.ts:249–260` | `child.kill()` | Cancellation token triggers `child.kill()` | `child.kill()` (Tauri `Child.kill()`) — already fire-and-forget; must still wait for `'close'` event to confirm exit |
| `git.ts:604–616` | `this.spawn(args, options)` returns `cp.ChildProcess` used as stream | `stream()` method wraps a live process for callers that consume stdout line-by-line | `Command.create(...).spawn()` returns `Child`; stdout lines arrive through `command.stdout.on('data', ...)` |
| `git.ts:676–702` | `cp.spawn(this.path, args, options)` | Core spawn used by `_exec`, `exec`, `stream` | Wrapped by `TauriGitProcessLauncher.spawn()` in `tauriProcessLauncher.js` |
| `git.ts:624` | `child.stdin!.end(options.input, 'utf8')` | Writes credential/commit-msg data to git stdin | `Child.write(string)` — async; `stdin` is `null` in the Tauri adapter; callers using `options.input` need an explicit `await child.write(input)` after `spawn()` |
| `git.ts:1395–1401` | `Repository.stream()` / `Repository.spawn()` | Delegates back to `Git.stream` / `Git.spawn` | Covered by the same `TauriGitProcessLauncher` path above |

---

**Artifacts already present (`extensions/git/out/`):**

`/Users/norinlavaee/vscode-atomic/extensions/git/out/tauri-shell.js`
- Exports `tauriExec(command, args, options)` — wraps `Command.create().execute()` for
  short-lived commands (analogous to `cp.exec`).
- Exports `tauriSpawn(command, args, options)` — wraps `Command.create().spawn()` for
  streaming processes; returns a duck-typed `IChildProcessLike` with `.stdout`, `.stderr`,
  `.stdin` (null), `.on()`, `.kill()`.

`/Users/norinlavaee/vscode-atomic/extensions/git/out/git/tauriProcessLauncher.js`
- Exports `TauriGitProcessLauncher` implementing an `IGitProcessLauncher` interface.
- `.spawn(command, args, options)` returns a `TauriGitChild` that duck-types `cp.ChildProcess`
  (`.stdout`, `.stderr`, `.stdin` null, `.on('exit'|'error')`, `.kill()`).
- `kill()` delegates to `this._tauriChild.kill().catch(() => {})` (fire-and-forget).
- **Stdin gap:** `TauriGitChild.stdin` is always `null`. Any git operation that pipes input
  (e.g. `git commit` with `options.input` for the commit message, or `git credential`)
  requires a separate `await child.write(data)` call via the underlying `_tauriChild` —
  which is not yet surfaced by the current adapter's public interface.

---

**Gaps / open issues:**

1. **Stdin write support** — `options.input` in `SpawnOptions` (git.ts:204) writes to
   `child.stdin` synchronously in Node. The Tauri adapter exposes `stdin: null`; any code
   path that reaches `git.ts:624` (`child.stdin!.end(options.input, 'utf8')`) will throw.
   Fix: surface `TauriGitChild.writeStdin(data)` and call it from `_exec` when
   `options.input` is set.

2. **Line-buffering** — Tauri shell emits one `data` event per line (no partial chunks).
   The adapters append `'\n'` to restore the line terminator. However, `git.ts:238`
   accumulates raw `Buffer` chunks from Node streams; the Tauri path delivers strings.
   The `bufferResult.stdout.toString('utf8')` coercion at `git.ts:657` would fail because
   there is no `Buffer` — downstream code must work with pre-concatenated strings.

3. **Path discovery on macOS/Windows** — `findGitDarwin` / `findGitWin32` call `cp.exec('which git')` and then validate the resolved path (git.ts:96–134). Under Tauri, executing `which` requires a separate scope entry. An alternative is to hard-code common locations (`/usr/bin/git`, `/usr/local/bin/git`) and use `Command.create().execute()` for each probe, matching the current `findSpecificGit` pattern.

4. **`onSpawn` callback** — `SpawnOptions.onSpawn?(child: cp.ChildProcess)` at git.ts:207/621 passes the raw `ChildProcess` to callers (e.g., `clone()` at git.ts:451 attaches a line stream to `child.stderr`). Under Tauri, the equivalent is attaching listeners to `command.stderr.on('data', ...)` before calling `spawn()`. The `TauriGitProcessLauncher` does not yet expose an `onSpawn` hook.

5. **Cancellation / `child.killed` flag** — git.ts:611 checks `child.killed` for the log message. `TauriGitChild._killed` tracks this internally but it is not yet exposed as a public `.killed` property.

---

**Summary**

The Tauri 2.x shell plugin (`@tauri-apps/plugin-shell`) provides a `Command` class whose `create()` / `sidecar()` factory methods, `spawn()` streaming path, `execute()` fire-and-forget path, `Child.write()` for stdin, and `Child.kill()` for termination cover every Node `child_process` usage found in `extensions/git/src/git.ts`. The two existing Tauri adapter files (`tauri-shell.js` and `tauriProcessLauncher.js`) already implement the core spawn-and-stream pattern correctly for the main `Git.spawn()` / `Git._exec()` hot path (git.ts:676–702 and 210–270). The five gaps listed above — stdin writes via `options.input`, line-buffering/Buffer vs string duality, path-discovery via `which`/`xcode-select`, `onSpawn` callback exposure, and the `killed` flag — are the concrete items that remain to be resolved before the git extension can run fully under Tauri without any Node `child_process` dependency.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
