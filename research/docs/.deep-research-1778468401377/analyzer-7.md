### Files Analysed

- `extensions/git/src/main.ts` — extension entry point; lifecycle, wiring
- `extensions/git/src/git.ts` — core git CLI interface: process spawning, output parsers
- `extensions/git/src/repository.ts` — dual-layered repository abstraction: base repo + SCM-integrated wrapper
- `extensions/git/src/model.ts` — multi-repo aggregator; workspace scanning, event hubs
- `extensions/git/src/askpass.ts` — credential prompt handler (IPC + VS Code UI)
- `extensions/git/src/askpassManager.ts` — Windows-stable content-addressed copy of askpass scripts
- `extensions/git/src/ipc/ipcServer.ts` — HTTP-over-Unix-socket IPC server
- `extensions/git/src/ipc/ipcClient.ts` — thin HTTP client for askpass/editor subprocesses
- `extensions/git/src/decorationProvider.ts` — three `FileDecorationProvider` registrations driven by repo state
- `extensions/git/src/api/extension.ts` + `src/api/api1.ts` — versioned public API surface exposed to other extensions
- `extensions/git/src/terminal.ts` — terminal environment injection and shell-execution listener

---

### Per-File Notes

#### `extensions/git/src/main.ts`

- **Role:** VS Code extension entry point. `activate()` (`main.ts:258`) calls `_activate()` (`main.ts:192`) which orchestrates the full startup sequence: git binary discovery, IPC server creation, askpass wiring, `Git` / `Model` / `CommandCenter` instantiation, and registration of every provider.
- **Key symbols:**
  - `activate` (`main.ts:258`) — called by VS Code runtime; returns `GitExtension` API object
  - `_activate` (`main.ts:192`) — real async startup logic
  - `createModel` (`main.ts:41`) — constructs `Git`, `Model`, `Askpass`, `GitEditor`, all providers, and `CloneManager`
  - `deactivateTasks` (`main.ts:33`) — module-level array of async teardown functions; telemetry reporter disposal is pushed here
  - `findGit` (`main.ts:53`) — resolves git binary path from config + OS
- **Control flow:** `activate` → `_activate` → checks `git.enabled` config; if disabled, defers `createModel` until config changes (`main.ts:213-222`). On success, `createModel` creates `IPCServer` (`main.ts:70`), `Askpass` (`main.ts:76`), `GitEditor` (`main.ts:79`), merges their `getEnv()` outputs (`main.ts:82`), builds `Git` (`main.ts:88-93`) and `Model` (`main.ts:94`), then instantiates every provider in a single `disposables.push` block (`main.ts:116-124`).
- **Data flow:** `findGit` resolves to `IGit` (`{ path, version }`); passed into `Git` constructor. Environment dictionary assembled from askpass + gitEditor + ipcServer `getEnv()` is embedded in the `Git` instance and merged into every spawned child process via `spawn()`.
- **Dependencies:** `vscode`, `./git`, `./model`, `./commands`, `./fileSystemProvider`, `./decorationProvider`, `./askpass`, `./askpassManager`, `./ipc/ipcServer`, `./terminal`, `./api/extension`, `./api/api1`, `@vscode/extension-telemetry`

---

#### `extensions/git/src/git.ts`

- **Role:** All git CLI interaction. Exposes `Git` (the process launcher) and `Repository` (the command executor bound to a root path), plus a suite of pure output parsers.
- **Key symbols:**
  - `Git.spawn` (`git.ts:676`) — wraps `cp.spawn(this.path, args, options)` with env injection (`git.ts:689-695`): forces `LANGUAGE=en`, `LC_ALL=en_US.UTF-8`, `GIT_PAGER=cat`, appends `VSCODE_GIT_COMMAND=<subcommand>`
  - `Git._exec` (`git.ts:618`) — calls `spawn`, pipes stdin if `options.input`, awaits `exec()` promise, converts `Buffer` stdout to `string`, rejects on non-zero exit via `GitError`
  - `exec` (module-level, `git.ts:210`) — collects child-process stdout/stderr as Buffers using `Promise.all`; supports `CancellationToken` by racing against a kill-and-reject promise (`git.ts:249-258`)
  - `Git.stream` (`git.ts:604`) — returns a raw `cp.ChildProcess` for streaming (used by clone progress)
  - `findGit` (`git.ts:159`) — tries explicit hints first, then platform-specific discovery (`darwin`: `which git` + xcode-select check; `win32`: ProgramFiles cascade; default: `which git`)
  - `GitStatusParser` (`git.ts:821`) — streaming NUL-delimited `git status --porcelain=v1 -z` parser; `update(raw)` appends to `lastRaw` buffer and emits complete `IFileStatus` entries
  - `parseGitCommits` (`git.ts:928`) — regex-based parser against `COMMIT_FORMAT = '%H%n%aN%n%aE%n%at%n%ct%n%P%n%D%n%B'`
  - `parseGitBlame` (`git.ts:1219`) — line-by-line porcelain blame parser aggregating ranges per commit hash
  - `parseRefs` (`git.ts:1280`) — NUL-delimited `for-each-ref` parser producing `Ref | Branch` objects
  - `GitConfigParser.parse` (`git.ts:789`) — INI-style section/property parser for `.git/config` and `.gitmodules`
  - `Repository` (inner class, `git.ts:1357`) — thin wrapper that calls `this.git.exec(this.repositoryRoot, args, options)` for every git command; holds no additional state
- **Control flow:** All git operations flow through `Git.exec` → `Git._exec` → `Git.spawn` → `cp.spawn`. Cancellation sends `child.kill()` and throws `CancellationError`. Non-zero exit codes are mapped to `GitError` with `getGitErrorCode(stderr)` translating stderr patterns to typed error codes (`git.ts:329-363`).
- **Data flow:** Arguments array + `SpawnOptions` in; stdout `Buffer` → `string`; parsers convert raw strings to typed objects (`Commit[]`, `IFileStatus[]`, `Ref[]`, etc.). Stderr is always captured but only used for error code detection unless `commandsToLog` includes the subcommand.
- **Dependencies:** `child_process` (`cp`), `fs`, `path`, `os`, `events`, `byline`, `string_decoder`, `file-type`, `which`, `vscode` (for `CancellationToken`, `workspace`, `Uri`), `./util`, `./api/git`, `./api/git.constants`

---

#### `extensions/git/src/repository.ts`

- **Role:** Two-layer design. Inner `Repository` (`git.ts:1357`) holds only a reference to the git process runner. Outer `Repository` (`repository.ts:703`) is the SCM-integrated class: owns `SourceControl`, four `SourceControlResourceGroup`s, file watchers, branch protection matchers, history provider, and operation serializer.
- **Key symbols:**
  - `Repository` class (`repository.ts:703`) — owns `_sourceControl: SourceControl` created via `scm.createSourceControl('git', 'Git', root, ...)` (`repository.ts:984`)
  - `Resource` class (`repository.ts:56`) — implements `SourceControlResourceState`; carries `Status` enum, resolves `resourceDecoration: FileDecoration`, delegates command resolution to `ResourceCommandResolver`
  - `DotGitWatcher` (`repository.ts:453`) — wraps `watch(repository.dotGit.path)` (inotify/FSEvents); filters out `index.lock` and watchman cookies (`repository.ts:470`); also watches upstream ref file transiently (`repository.ts:487-493`)
  - `ProgressManager` (`repository.ts:360`) — bridges `onDidChangeOperations` events to `window.withProgress({ location: ProgressLocation.SourceControl })`
  - `Repository.run` (implied by callers) — routes all operations through `OperationManager` for serialization
  - `ResourceCommandResolver` (`repository.ts:505`) — maps `Status` → `vscode.diff` / `vscode.open` / merge-editor command; produces `leftUri`/`rightUri` using `toGitUri(resource, 'HEAD')` etc.
- **Control flow:** Constructor (`repository.ts:905`) creates file watchers → any workspace or `.git` change fires `onFileChange` → debounced `updateModelState` → calls `git status`, `git diff`, etc. → updates `mergeGroup`, `indexGroup`, `workingTreeGroup`, `untrackedGroup` resource states → fires `_onDidChangeStatus`.
- **Data flow:** File system events arrive from `workspace.createFileSystemWatcher` and `DotGitWatcher`. After debounce, `updateModelState` calls the inner `BaseRepository` exec methods, parses output through the parsers in `git.ts`, and writes `Resource[]` arrays back into the `SourceControlResourceGroup` instances.
- **Dependencies:** `vscode` (heavy: `scm`, `SourceControl`, `SourceControlResourceGroup`, `workspace`, `window`, `commands`, `tasks`), `./git` (inner repo + parsers), `./operation`, `./autofetch`, `./branchProtection`, `./historyProvider`, `./quickDiffProvider`, `./artifactProvider`, `picomatch`, `./util`, `./watch`

---

#### `extensions/git/src/model.ts`

- **Role:** Multi-repository aggregator. Discovers repositories in workspace folders by scanning for `.git`, watching FS events for new repositories, and managing repository lifecycle (open/close). Also serves as a registry for extension-point providers: push error handlers, remote source publishers, post-commit command providers, branch protection providers.
- **Key symbols:**
  - `Model` class (`model.ts:186`) — implements `IRepositoryResolver`, `IBranchProtectionProviderRegistry`, `IRemoteSourcePublisherRegistry`, `IPostCommitCommandsProviderRegistry`, `IPushErrorHandlerRegistry`, `ISourceControlHistoryItemDetailsProviderRegistry`
  - `doInitialScan` (`model.ts:311`) — called in constructor; respects `git.autoRepositoryDetection` and `git.openRepositoryInParentFolders` settings; emits telemetry after completion
  - `scanWorkspaceFolders` (`model.ts:358`) — traverses workspace folders to `repositoryScanMaxDepth` levels using `traverseWorkspaceFolder`; calls `openRepository` for each candidate
  - `onPossibleGitRepositoryChange` (`model.ts:442`) — FS watcher callback; triggered when a path matching `/**/.git` changes; finds repository root and opens it
  - `ClosedRepositoriesManager` (`model.ts:64`) — persists closed-repo paths to `workspaceState`
  - `UnsafeRepositoriesManager` (`model.ts:143`) — tracks directories git rejected as unsafe ownership; calls `git config --global --add safe.directory` when user approves
  - `RepositoryCache` (`model.ts:279`) — caches known repository roots in `globalState`
- **Control flow:** Constructor subscribes to `workspace.onDidChangeWorkspaceFolders`, `onDidChangeWorkspaceTrustedFolders`, `window.onDidChangeVisibleTextEditors`, `workspace.onDidChangeConfiguration`, and a global `workspace.createFileSystemWatcher('**')` filtered to `.git` path changes (`model.ts:298-304`). Each trigger may call `openRepository` which calls `git.getRepositoryRoot` and `git.getRepositoryDotGit` to validate, then instantiates a new `Repository` and fires `onDidOpenRepository`.
- **Data flow:** Workspace folder URIs → `traverseWorkspaceFolder` → candidate paths → `openRepository` → `git rev-parse --show-toplevel` → `IDotGit` struct → new `Repository` instance added to `openRepositories: OpenRepository[]`.
- **Dependencies:** `vscode`, `./repository`, `./git`, `./askpass`, `./api/api1`, `./util`, `./repositoryCache`, `@vscode/extension-telemetry`

---

#### `extensions/git/src/askpass.ts`

- **Role:** Credential prompt handler. Registered with the IPC server as the `askpass` handler. When git needs credentials, the askpass shell script sends an HTTP request to the IPC socket; this class receives it and either calls a registered `CredentialsProvider` or shows a VS Code `InputBox`.
- **Key symbols:**
  - `Askpass` class (`askpass.ts:13`) — implements `IIPCHandler`, `ITerminalEnvironmentProvider`
  - Constructor (`askpass.ts:23`) — registers itself via `ipc.registerHandler('askpass', this)` (`askpass.ts:29`); sets `GIT_ASKPASS` env var pointing to the shell script (`askpass.ts:36-42`); sets `SSH_ASKPASS` and `SSH_ASKPASS_REQUIRE=force` for SSH auth (`askpass.ts:44-48`)
  - `handle` (`askpass.ts:51`) — dispatches to `handleAskpass` (HTTPS) or `handleSSHAskpass` (SSH) based on `payload.askpassType`
  - `handleAskpass` (`askpass.ts:67`) — for username: queries `credentialsProviders` first, caches result for 60 s (`askpass.ts:93`); for password: retrieves from cache or shows `window.showInputBox`
  - `handleSSHAskpass` (`askpass.ts:110`) — handles passphrase or host-key authenticity prompts; uses `window.showInputBox` (passphrase) or `window.showQuickPick` (yes/no authenticity)
  - `getEnv` (`askpass.ts:154`) — returns askpass env vars if `git.useIntegratedAskPass` config is true, otherwise empty
  - `getTerminalEnv` (`askpass.ts:159`) — returns only HTTPS env vars for terminal if `git.terminalAuthentication` is also true
- **Control flow:** git subprocess → executes `$GIT_ASKPASS <prompt>` shell script → `askpass-main.js` → `IPCClient.call({askpassType, argv})` → HTTP POST to IPC socket → `IPCServer.onRequest` → `Askpass.handle` → VS Code UI prompt → string response → HTTP response → shell script prints it → git reads from stdout.
- **Dependencies:** `vscode` (window, workspace, InputBoxOptions), `./ipc/ipcServer`, `./terminal`, `./askpassManager`, `./util`

---

#### `extensions/git/src/askpassManager.ts`

- **Role:** Manages content-addressed stable copies of askpass shell scripts in user storage on Windows user/system VS Code installs (where the installation path changes on every update).
- **Key symbols:**
  - `getAskpassPaths` (`askpassManager.ts:290`) — main exported function; returns `AskpassPaths` either from content-addressed storage (Windows user/system install) or directly from the extension's `__dirname`
  - `ensureAskpassScripts` (`askpassManager.ts:215`) — computes SHA-256 hash of all five script files (`askpassManager.ts:64-83`), checks if content-addressed directory already exists (fast path), copies files with `icacls` ACL hardening if not
  - `computeContentHash` (`askpassManager.ts:64`) — hashes file contents + basenames deterministically across all five askpass scripts
  - `garbageCollectOldDirectories` (`askpassManager.ts:148`) — removes content-addressed directories not accessed in 7 days by checking `stat.mtime`
  - `setWindowsPermissions` (`askpassManager.ts:90`) — runs `icacls <file> /inheritance:r /grant:r "<USERNAME>:F"` via `cp.execFile`
- **Control flow:** `getAskpassPaths` → `isWindowsUserOrSystemSetup()` (reads `product.json`) → if applicable, `ensureAskpassScripts` → compute hash → check/create `~/.../askpass/<hash>/` directory → copy scripts → update mtime → GC old directories.
- **Dependencies:** `crypto`, `fs`, `path`, `child_process` (cp.execFile for icacls), `vscode` (env.appRoot, LogOutputChannel)

---

#### `extensions/git/src/ipc/ipcServer.ts`

- **Role:** HTTP server listening on a Unix domain socket (or Windows named pipe) that routes JSON-encoded requests to registered named handlers.
- **Key symbols:**
  - `createIPCServer` (`ipcServer.ts:31`) — creates `http.Server`, derives socket path from SHA-256 of `context` string (or random bytes), removes stale socket file on non-Windows, listens, returns `IPCServer`
  - `getIPCHandlePath` (`ipcServer.ts:15`) — platform-specific path: `\\\\.\\pipe\\vscode-git-${id}-sock` on Windows, `$XDG_RUNTIME_DIR/vscode-git-${id}.sock` on Linux with XDG, else `os.tmpdir()`
  - `IPCServer.registerHandler` (`ipcServer.ts:78`) — stores handler under `/${name}` key in `handlers: Map`
  - `IPCServer.onRequest` (`ipcServer.ts:83`) — accumulates request body chunks, JSON-parses, dispatches to handler, JSON-serializes result, responds 200/500
  - `IPCServer.getEnv` (`ipcServer.ts:110`) — returns `{ VSCODE_GIT_IPC_HANDLE: this.ipcHandlePath }`; injected into every spawned git process
  - `IPCServer.dispose` (`ipcServer.ts:118`) — closes server, unlinks socket file on non-Windows
- **Control flow:** `createIPCServer` → `server.listen(ipcHandlePath)` → registered handlers respond to path-routed HTTP POST requests from `IPCClient` (askpass-main.js, git-editor-main.js).
- **Dependencies:** `http`, `path`, `os`, `fs`, `crypto`, `vscode` (Disposable), `./terminal` (ITerminalEnvironmentProvider)

---

#### `extensions/git/src/ipc/ipcClient.ts`

- **Role:** Thin HTTP client used inside askpass/editor subprocess scripts to call back into the extension host.
- **Key symbols:**
  - `IPCClient.call` (`ipcClient.ts:22`) — sends `http.request` to `socketPath: this.ipcHandlePath`, path `/${handlerName}`, method POST, body `JSON.stringify(request)`; resolves with parsed JSON response
  - Constructor (`ipcClient.ts:12`) — reads `VSCODE_GIT_IPC_HANDLE` from environment
- **Control flow:** subprocess script → `new IPCClient('askpass').call({...})` → HTTP POST over Unix socket → `IPCServer.onRequest` → handler → HTTP 200 + JSON → returned promise resolves.
- **Dependencies:** `http` only

---

#### `extensions/git/src/decorationProvider.ts`

- **Role:** Registers three `FileDecorationProvider`s with VS Code that add color, badge, and tooltip overlays on file explorer entries.
- **Key symbols:**
  - `GitIgnoreDecorationProvider` (`decorationProvider.ts:25`) — queries `repository.checkIgnore(paths)` in debounced batches (`@debounce(500)` at `decorationProvider.ts:70`); shows `ThemeColor('gitDecoration.ignoredResourceForeground')` for ignored files
  - `GitDecorationProvider` (`decorationProvider.ts:100`) — caches `Map<string, FileDecoration>` from `repository.onDidRunGitStatus`; walks all four resource groups to build decorations; handles submodule badge (`'S'`)
  - `GitIncomingChangesFileDecorationProvider` (`decorationProvider.ts:169`) — reacts to `repository.historyProvider.onDidChangeCurrentHistoryItemRefs`; resolves common ancestor and calls `repository.diffBetweenWithStats` to find incoming changes; shows `↓A/↓D/↓R/↓M/↓~` badges
  - `GitDecorations` (`decorationProvider.ts:277`) — outer coordinator; listens to `workspace.onDidChangeConfiguration` for `git.decorations.enabled`; creates/destroys per-repo providers on `model.onDidOpenRepository` / `model.onDidCloseRepository`
- **Control flow:** `GitDecorations` constructor subscribes to config changes → `update()` enables/disables; when enabled, `model.onDidOpenRepository` fires `onDidOpenRepository` → creates `GitDecorationProvider` + `GitIncomingChangesFileDecorationProvider` per repo; status changes fire `_onDidChangeDecorations` which VS Code calls `provideFileDecoration` on each registered URI.
- **Dependencies:** `vscode` (window.registerFileDecorationProvider, FileDecorationProvider, FileDecoration, ThemeColor, EventEmitter), `./repository`, `./model`, `./util`, `./decorators`

---

#### `extensions/git/src/api/extension.ts`

- **Role:** Implements the `GitExtension` interface returned from `activate()`. Controls the `enabled` flag and gates access to `getAPI(1)`.
- **Key symbols:**
  - `GitExtensionImpl` class (`extension.ts:24`) — holds optional `_model: Model` and `_cloneManager: CloneManager`; setting `model` fires `_onDidChangeEnablement`
  - `getAPI` (`extension.ts:81`) — validates `version === 1`, returns `new ApiImpl({ model, cloneManager })`
  - `getGitPath` / `getRepositories` (`extension.ts:64`, `extension.ts:73`) — decorated `@deprecated`; delegate to `_model`
- **Dependencies:** `./model`, `./api/api1`, `vscode` (Event, EventEmitter), `./cloneManager`

---

#### `extensions/git/src/api/api1.ts`

- **Role:** Wraps internal `Repository` and `Model` classes behind the stable public API (`git.d.ts` types).
- **Key symbols:**
  - `ApiRepository` (`api1.ts:75`) — wraps `BaseRepository`; exposes `state: ApiRepositoryState`, `ui: ApiRepositoryUIState`, all git operation methods, `onDidCommit` / `onDidCheckout` events
  - `ApiRepositoryState` (`api1.ts:38`) — delegates `HEAD`, `remotes`, `submodules`, `worktrees`, `mergeChanges`, `indexChanges`, `workingTreeChanges`, `untrackedChanges` to the internal `Repository`'s resource groups
  - `ApiImpl` — wraps `Model`; exposes `repositories`, `onDidOpenRepository`, `onDidCloseRepository`, `init`, `clone`, `openRepository`, `registerCredentialsProvider`, etc.
- **Dependencies:** `./model`, `./repository`, `./api/git` (type-only), `vscode`, `./uri`, `./util`, `./cloneManager`

---

#### `extensions/git/src/terminal.ts`

- **Role:** Two classes: `TerminalEnvironmentManager` pushes askpass/editor env vars into VS Code's terminal environment collection; `TerminalShellExecutionManager` listens for git subcommand completions in integrated terminals and triggers repository refresh.
- **Key symbols:**
  - `TerminalEnvironmentManager.refresh` (`terminal.ts:26`) — calls `getTerminalEnv()` on each provider (Askpass, GitEditor, IPCServer) and calls `context.environmentVariableCollection.replace(name, value)` for each key
  - `TerminalShellExecutionManager.onDidEndTerminalShellExecution` (`terminal.ts:69`) — receives `TerminalShellExecutionEndEvent`; splits `execution.commandLine.value` to check if executable is `git` and subcommand is in the tracked set (`add`, `commit`, `push`, etc.); if so, finds the matching `Repository` by `cwd` and calls `repository.refresh()`
- **Dependencies:** `vscode` (ExtensionContext, window, workspace, TerminalShellExecutionEndEvent), `./util`, `./model`

---

### Cross-Cutting Synthesis

The `extensions/git/` partition is a self-contained TypeScript extension that acts as the sole bridge between VS Code's SCM UI and the system git binary. All git work goes through `cp.spawn` in `git.ts:676` — there is no use of libgit2 or native bindings. A single shared `Git` instance (constructed in `main.ts:88`) holds the git binary path and a composed environment dictionary (askpass + git-editor + IPC handle env vars). Every repository operation is executed by the inner `Repository` class delegating to `git.exec(root, args)`. The outer `Repository` class in `repository.ts` owns the VS Code SCM objects (`SourceControl`, four resource groups) and reacts to file-system events via `workspace.createFileSystemWatcher` and `DotGitWatcher`.

Credentials flow through a local HTTP server (`IPCServer`, Unix socket): git executes the `GIT_ASKPASS` shell script which calls `IPCClient` → IPC socket → `Askpass.handle` → VS Code `InputBox`. The same socket serves the `git-editor` handler. Terminal integration uses `context.environmentVariableCollection` to propagate IPC and askpass env vars into integrated terminals, and `TerminalShellExecutionManager` to trigger repository refreshes after terminal git commands.

The versioned API (`getAPI(1)` → `ApiImpl`) wraps the internal `Model` and `Repository` with stable proxy classes (`ApiRepository`, `ApiRepositoryState`) so external extensions never touch internal types.

---

### Out-of-Partition References

- `src/vs/workbench/api/common/extHostScm.ts` — host-side implementation of `scm.createSourceControl`, `SourceControlResourceGroup`, and decoration provider registration
- `src/vs/workbench/contrib/scm/` — SCM viewlet and resource rendering that consumes the `SourceControl` and `FileDecorationProvider` data produced here
- `src/vs/platform/ipc/` — VS Code's own IPC infrastructure; the git extension implements its own independent HTTP-over-socket IPC independently
- `extensions/git-base/` — `GitBaseApi` consumed at `src/git-base.ts:…` providing remote source provider abstractions shared with the GitHub extension
- `src/vs/workbench/api/common/extHostFileSystemEventService.ts` — implements `workspace.createFileSystemWatcher` consumed pervasively in `model.ts` and `repository.ts`
- `src/vs/workbench/api/common/extHostTerminalService.ts` — provides `window.onDidEndTerminalShellExecution` consumed by `TerminalShellExecutionManager`
- `src/vs/workbench/api/common/extHostDecorations.ts` — implements `window.registerFileDecorationProvider` consumed by all three decoration provider classes
- `node_modules/@vscode/extension-telemetry` — external telemetry package used in `main.ts` and `model.ts`
