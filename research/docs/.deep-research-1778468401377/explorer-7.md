# Partition 7 of 80 — Findings

## Scope
`extensions/git/` (62 files, 25,186 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Git Extension File Locator — Partition 7

## Summary

The `extensions/git/` partition contains 62 files totaling ~25K LOC, with 54 TypeScript source files concentrated in core git operations, VS Code API integration, and user-facing UI providers. This is a mature, self-contained SCM extension demonstrating the patterns that would need to be ported to a Tauri/Rust core: process spawning (git CLI), credential management (askpass), workspace events (file/status watchers), command registration, and source control provider lifecycle.

---

## Implementation

Core git operations and VS Code integration:

- `extensions/git/src/git.ts` — Primary git CLI interface; spawns git processes via `cp.spawn()` and manages all git command execution (clone, commit, status, etc.)
- `extensions/git/src/repository.ts` — Repository abstraction layer; manages per-repo state, event subscriptions, and delegates to Git class for operations
- `extensions/git/src/model.ts` — Aggregates multiple repositories and exposes high-level SCM model to extension consumers
- `extensions/git/src/commands.ts` — Command center binding git operations to VS Code command palette (registerCommand, executeCommand)
- `extensions/git/src/main.ts` — Extension entry point (activate/deactivate); initializes Model, CommandCenter, Providers, and managers; demonstrates lifecycle management
- `extensions/git/src/api/extension.ts` — Exposes versioned Git API to other extensions (getAPI(1))
- `extensions/git/src/api/api1.ts` — API v1 implementation; thin wrapper exposing git.ts interfaces

User-facing providers and UI:

- `extensions/git/src/fileSystemProvider.ts` — Implements TextDocumentContentProvider for git:// URIs (commit history, diff views)
- `extensions/git/src/decorationProvider.ts` — Provides file decoration API (badges on modified files in Explorer)
- `extensions/git/src/quickDiffProvider.ts` — Implements QuickDiffProvider for inline diff rendering in editor gutter
- `extensions/git/src/timelineProvider.ts` — Implements TimelineProvider for commit history view
- `extensions/git/src/historyProvider.ts` — Handles commit/ref history queries
- `extensions/git/src/historyItemDetailsProvider.ts` — Details provider for timeline items
- `extensions/git/src/blame.ts` — Blame annotations and hover information
- `extensions/git/src/hover.ts` — Hover provider showing git status info

Credential and authentication:

- `extensions/git/src/askpass.ts` — Node.js askpass helper; listens on socket for credential requests from git process
- `extensions/git/src/askpassManager.ts` — Manages askpass helper lifecycle; spawns subprocess and routes credential prompts to VS Code UI
- `extensions/git/src/askpass-main.ts` — Entry point for askpass subprocess
- `extensions/git/src/protocolHandler.ts` — Handles git:// and gitprompt:// protocol URIs

Advanced features and integration:

- `extensions/git/src/terminal.ts` — Terminal environment manager; configures Git credential helpers and SSH agents for terminal execution
- `extensions/git/src/staging.ts` — Staging area (index) state and diff operations
- `extensions/git/src/postCommitCommands.ts` — Runs post-commit hooks defined in .git/hooks
- `extensions/git/src/editSessionIdentityProvider.ts` — Edit sessions provider for cloud sync features
- `extensions/git/src/diagnostics.ts` — Commit message validation and quick fixes
- `extensions/git/src/actionButton.ts` — Source control publish/sync button state
- `extensions/git/src/branchProtection.ts` — Branch protection warnings
- `extensions/git/src/cloneManager.ts` — Clone repository UI/workflow
- `extensions/git/src/autofetch.ts` — Auto-fetch background service
- `extensions/git/src/artifactProvider.ts` — Artifact provider integration
- `extensions/git/src/remotePublisher.ts` — Remote repository publishing
- `extensions/git/src/remoteSource.ts` — Remote repository sources
- `extensions/git/src/cache.ts` — In-memory caching layer for expensive git operations
- `extensions/git/src/repositoryCache.ts` — Disk-based repository metadata cache
- `extensions/git/src/watch.ts` — File system watcher integration
- `extensions/git/src/decorators.ts` — Utility decorators
- `extensions/git/src/emoji.ts` — Emoji parsing in commit messages
- `extensions/git/src/uri.ts` — git:// URI helpers
- `extensions/git/src/util.ts` — Shared utilities (IDisposable, EventEmitter, mkdirp, file I/O)
- `extensions/git/src/operation.ts` — Long-running operation tracking
- `extensions/git/src/pushError.ts` — Push error categorization
- `extensions/git/src/gitEditor.ts` — Interactive rebase/commit editor
- `extensions/git/src/git-editor-main.ts` — Entry point for git editor subprocess
- `extensions/git/src/git-base.ts` — Base class for git command execution (not IGit)

IPC/inter-process communication:

- `extensions/git/src/ipc/ipcServer.ts` — IPC server for main-to-worker communication
- `extensions/git/src/ipc/ipcClient.ts` — IPC client helper

---

## Tests

- `extensions/git/src/test/git.test.ts` — Unit tests for Git class and command execution
- `extensions/git/src/test/repository.test.ts` — Repository abstraction tests (referenced in test count)
- `extensions/git/src/test/askpassManager.test.ts` — Credential helper lifecycle tests
- `extensions/git/src/test/repositoryCache.test.ts` — Cache layer tests
- `extensions/git/src/test/smoke.test.ts` — Integration/smoke tests
- `extensions/git/src/test/index.ts` — Test runner entry point

---

## Types / Interfaces

- `extensions/git/src/api/git.d.ts` — Public API type definitions; defines Repository, Commit, Branch, Remote, Change, and SCM model interfaces for external consumers
- `extensions/git/src/api/git.constants.ts` — Constants (RefType, Status, GitErrorCodes, ForcePushMode)
- `extensions/git/src/typings/git-base.d.ts` — Type stubs for native git-base module

---

## Configuration

- `extensions/git/package.json` — Manifest with extension metadata, commands, keybindings, configuration schema, and activation events
- `extensions/git/tsconfig.json` — TypeScript compiler configuration
- `extensions/git/esbuild.mts` — Build script (esbuild configuration)

---

## Documentation

- `extensions/git/README.md` — Overview of git integration features and API usage instructions for dependent extensions

---

## Shell Scripts

Credential and editor integration helpers:

- `extensions/git/src/askpass.sh` — Shell wrapper invoked by git as GIT_ASKPASS; forwards credential prompts to VS Code
- `extensions/git/src/askpass-empty.sh` — Empty askpass stub for scenarios without UI
- `extensions/git/src/ssh-askpass.sh` — SSH askpass wrapper
- `extensions/git/src/ssh-askpass-empty.sh` — Empty SSH askpass stub
- `extensions/git/src/git-editor.sh` — Shell wrapper for interactive rebase editor
- `extensions/git/src/git-editor-empty.sh` — Empty editor stub

---

## Notable Clusters

### Process spawning and git CLI (`extensions/git/src/`)

**Files**: `git.ts`, `repository.ts`, `commands.ts`, `ipc/ipcServer.ts`, `ipc/ipcClient.ts`, `postCommitCommands.ts`, `cloneManager.ts`, `historyProvider.ts`, `fileSystemProvider.ts`

**Why**: All git operations flow through `cp.spawn()` in git.ts. The porting effort must replace process spawning with Rust FFI or shell-out equivalents; IPC patterns must adapt to Tauri's message passing.

### VS Code API integration (`extensions/git/src/`)

**Files**: `main.ts`, `commands.ts`, `decorationProvider.ts`, `quickDiffProvider.ts`, `timelineProvider.ts`, `fileSystemProvider.ts`, `watch.ts`, `statusbar.ts`, `actionButton.ts`, `diagnostics.ts`, `postCommitCommands.ts`, `editSessionIdentityProvider.ts`

**Why**: Demonstrates all the VS Code extension APIs that the new core must expose (commands, workspace events, provider registration, decoration, UI state). Pattern: register callbacks, react to workspace/file changes, update UI via provider updates.

### Credential and auth management (`extensions/git/src/`)

**Files**: `askpass.ts`, `askpassManager.ts`, `askpass-main.ts`, `terminal.ts`, `editSessionIdentityProvider.ts`, plus shell scripts

**Why**: Credential flow (git → GIT_ASKPASS socket → VS Code UI) is critical for SSH/HTTPS operations. Porting requires integrating VS Code's credential storage and modal UI with git process stdin/stdout.

### SCM provider lifecycle (`extensions/git/src/`)

**Files**: `model.ts`, `repository.ts`, `api/extension.ts`, `api/api1.ts`, `api/git.d.ts`

**Why**: Defines the shape of the source control abstraction that dependent extensions (GitHub, GitLab, etc.) consume. No direct `vscode.scm.createSourceControl` call found in git extension itself—it exposes interfaces that dependents use. The core must provide equivalent.

### Caching and performance (`extensions/git/src/`)

**Files**: `cache.ts`, `repositoryCache.ts`, `git.ts`, `model.ts`

**Why**: Expensive git operations (log, status, diff) are cached. Rust port should maintain similar caching strategy.

---

## Key Findings for Porting

1. **Process spawning**: `cp.spawn()` used in git.ts to invoke git CLI. Rust can directly link libgit2 or spawn git via std::process.

2. **No direct `vscode.scm.createSourceControl` in this extension**: The git extension does NOT directly call the SCM API—it exposes its own interfaces (Repository, Commit, etc.) that other extensions consume via `gitExtension.getAPI(1)`. The core port must still provide an equivalent API surface.

3. **IPC patterns**: askpass subprocess communication via sockets. Tauri can use message channels or file-based IPC.

4. **Provider registration**: Command, Timeline, QuickDiff, FileSystem, Decoration, Blame providers all follow VS Code's subscription + update pattern. Core must maintain these contracts.

5. **Workspace integration**: Heavy reliance on `workspace.onDidChangeTextDocument`, `workspace.onDidSaveTextDocument`, file watchers, and configuration change events. Core SCM must stay in sync with editor state.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder Research: VS Code Git Extension Patterns

## Research Scope
Repository section: `extensions/git/` (62 files, 25,186 LOC)
Focus: Code patterns demonstrating how VS Code core IDE functionality is implemented

---

#### Pattern: Process Spawning for External Commands

**Where:** `extensions/git/src/git.ts:87-91`
**What:** Git version detection by spawning a child process and capturing stdout.

```typescript
function findSpecificGit(path: string, onValidate: (path: string) => boolean): Promise<IGit> {
	return new Promise<IGit>((c, e) => {
		if (!onValidate(path)) {
			return e(new Error(`Path "${path}" is invalid.`));
		}

		const buffers: Buffer[] = [];
		const child = cp.spawn(path, ['--version']);
		child.stdout.on('data', (b: Buffer) => buffers.push(b));
		child.on('error', cpErrorHandler(e));
		child.on('close', code => code ? e(new Error(`Not found. Code: ${code}`)) : c({ path, version: parseVersion(Buffer.concat(buffers).toString('utf8').trim()) }));
	});
}
```

**Variations / call-sites:** `extensions/git/src/git.ts:702` (spawn with custom options), `extensions/git/src/git.ts:604-616` (stream method for long-running processes)

---

#### Pattern: Async Exec with Cancellation and Timeout Support

**Where:** `extensions/git/src/git.ts:210-270`
**What:** Wraps child process execution with cancellation token handling, event management, and proper cleanup.

```typescript
async function exec(child: cp.ChildProcess, cancellationToken?: CancellationToken): Promise<IExecutionResult<Buffer>> {
	if (!child.stdout || !child.stderr) {
		throw new GitError({ message: 'Failed to get stdout or stderr from git process.' });
	}

	if (cancellationToken && cancellationToken.isCancellationRequested) {
		throw new CancellationError();
	}

	const disposables: IDisposable[] = [];

	const once = (ee: NodeJS.EventEmitter, name: string, fn: (...args: any[]) => void) => {
		ee.once(name, fn);
		disposables.push(toDisposable(() => ee.removeListener(name, fn)));
	};

	const on = (ee: NodeJS.EventEmitter, name: string, fn: (...args: any[]) => void) => {
		ee.on(name, fn);
		disposables.push(toDisposable(() => ee.removeListener(name, fn)));
	};

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

**Variations / call-sites:** `extensions/git/src/git.ts:595-674` (higher-level `_exec` with logging and error handling)

---

#### Pattern: Configurable Spawn Options with Environment Injection

**Where:** `extensions/git/src/git.ts:676-703`
**What:** Wrapper around `cp.spawn` that injects environment variables and normalizes working directory paths.

```typescript
spawn(args: string[], options: SpawnOptions = {}): cp.ChildProcess {
	if (!this.path) {
		throw new Error('git could not be found in the system.');
	}

	if (!options) {
		options = {};
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

**Variations / call-sites:** `extensions/git/src/git.ts:595-602` (exec variants), `extensions/git/src/git.ts:604-616` (stream variant)

---

#### Pattern: Source Control Integration via SCM API

**Where:** `extensions/git/src/repository.ts:984-1009`
**What:** Creates a SourceControl instance and registers resource groups for merge/index/working-tree/untracked changes.

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
this.disposables.push(this._artifactProvider);

this._sourceControl.acceptInputCommand = { command: 'git.commit', title: l10n.t('Commit'), arguments: [this._sourceControl] };
this._sourceControl.inputBox.validateInput = this.validateInput.bind(this);

this.disposables.push(this._sourceControl);

this.updateInputBoxPlaceholder();
this.disposables.push(this.onDidRunGitStatus(() => this.updateInputBoxPlaceholder()));

this._mergeGroup = this._sourceControl.createResourceGroup('merge', l10n.t('Merge Changes'));
this._indexGroup = this._sourceControl.createResourceGroup('index', l10n.t('Staged Changes'), { multiDiffEditorEnableViewChanges: true });
this._workingTreeGroup = this._sourceControl.createResourceGroup('workingTree', l10n.t('Changes'), { multiDiffEditorEnableViewChanges: true });
this._untrackedGroup = this._sourceControl.createResourceGroup('untracked', l10n.t('Untracked Changes'), { multiDiffEditorEnableViewChanges: true });
```

**Variations / call-sites:** Resource state implementations in `extensions/git/src/repository.ts:56-347`

---

#### Pattern: Event Emitters for State Management

**Where:** `extensions/git/src/repository.ts:706-732`
**What:** Uses EventEmitter for observable state changes across repository operations.

```typescript
private _onDidChangeRepository = new EventEmitter<Uri>();
readonly onDidChangeRepository: Event<Uri> = this._onDidChangeRepository.event;

private _onDidChangeState = new EventEmitter<RepositoryState>();
readonly onDidChangeState: Event<RepositoryState> = this._onDidChangeState.event;

private _onDidChangeStatus = new EventEmitter<void>();
readonly onDidRunGitStatus: Event<void> = this._onDidChangeStatus.event;

private _onDidChangeOriginalResource = new EventEmitter<Uri>();
readonly onDidChangeOriginalResource: Event<Uri> = this._onDidChangeOriginalResource.event;

private _onRunOperation = new EventEmitter<OperationKind>();
readonly onRunOperation: Event<OperationKind> = this._onRunOperation.event;

private _onDidRunOperation = new EventEmitter<OperationResult>();
readonly onDidRunOperation: Event<OperationResult> = this._onDidRunOperation.event;

private _onDidChangeBranchProtection = new EventEmitter<void>();
readonly onDidChangeBranchProtection: Event<void> = this._onDidChangeBranchProtection.event;

@memoize
get onDidChangeOperations(): Event<void> {
	return anyEvent(
		this.onRunOperation as Event<unknown>,
		this.onDidRunOperation as Event<unknown>) as Event<void>;
}
```

**Variations / call-sites:** Event subscription patterns throughout `extensions/git/src/repository.ts`

---

#### Pattern: File System Watching for Repository State

**Where:** `extensions/git/src/watch.ts:13-22`
**What:** Abstracts workspace file system watcher into a simple interface for monitoring .git directory changes.

```typescript
export function watch(location: string): IFileWatcher {
	const watcher = workspace.createFileSystemWatcher(new RelativePattern(location, '*'));

	return new class implements IFileWatcher {
		event = anyEvent(watcher.onDidCreate, watcher.onDidChange, watcher.onDidDelete);
		dispose() {
			watcher.dispose();
		}
	};
}
```

**Variations / call-sites:** Used in repository state monitoring

---

#### Pattern: Diagnostic Collection and Code Actions

**Where:** `extensions/git/src/diagnostics.ts:15-105`
**What:** Manages commit message validation diagnostics and provides quick-fix code actions for formatting issues.

```typescript
export class GitCommitInputBoxDiagnosticsManager {

	private readonly diagnostics: DiagnosticCollection;
	private readonly severity = DiagnosticSeverity.Warning;
	private readonly disposables: Disposable[] = [];

	constructor(private readonly model: Model) {
		this.diagnostics = languages.createDiagnosticCollection();

		this.migrateInputValidationSettings()
			.then(() => {
				mapEvent(filterEvent(workspace.onDidChangeTextDocument, e => e.document.uri.scheme === 'vscode-scm'), e => e.document)(this.onDidChangeTextDocument, this, this.disposables);
				filterEvent(workspace.onDidChangeConfiguration, e => e.affectsConfiguration('git.inputValidation') || e.affectsConfiguration('git.inputValidationLength') || e.affectsConfiguration('git.inputValidationSubjectLength'))(this.onDidChangeConfiguration, this, this.disposables);
			});
	}

	public getDiagnostics(uri: Uri): ReadonlyArray<Diagnostic> {
		return this.diagnostics.get(uri) ?? [];
	}

	private onDidChangeTextDocument(document: TextDocument): void {
		const config = workspace.getConfiguration('git');
		const inputValidation = config.get<boolean>('inputValidation', false);
		if (!inputValidation) {
			this.diagnostics.set(document.uri, undefined);
			return;
		}

		if (/^\s+$/.test(document.getText())) {
			const documentRange = new Range(document.lineAt(0).range.start, document.lineAt(document.lineCount - 1).range.end);
			const diagnostic = new Diagnostic(documentRange, l10n.t('Current commit message only contains whitespace characters'), this.severity);
			diagnostic.code = DiagnosticCodes.empty_message;

			this.diagnostics.set(document.uri, [diagnostic]);
			return;
		}

		const diagnostics: Diagnostic[] = [];
		const inputValidationLength = config.get<number>('inputValidationLength', 50);
		const inputValidationSubjectLength = config.get<number | undefined>('inputValidationSubjectLength', undefined);

		for (let index = 0; index < document.lineCount; index++) {
			const line = document.lineAt(index);
			const threshold = index === 0 ? inputValidationSubjectLength ?? inputValidationLength : inputValidationLength;

			if (line.text.length > threshold) {
				const charactersOver = line.text.length - threshold;
				const lineLengthMessage = charactersOver === 1
					? l10n.t('{0} character over {1} in current line', charactersOver, threshold)
					: l10n.t('{0} characters over {1} in current line', charactersOver, threshold);
				const diagnostic = new Diagnostic(line.range, lineLengthMessage, this.severity);
				diagnostic.code = DiagnosticCodes.line_length;

				diagnostics.push(diagnostic);
			}
		}

		this.diagnostics.set(document.uri, diagnostics);
	}
}
```

**Variations / call-sites:** Code action provider at `extensions/git/src/diagnostics.ts:107-228`

---

#### Pattern: Command Execution and UI Interactions

**Where:** `extensions/git/src/repository.ts:324-342`
**What:** Command resolution and execution through VS Code command palette API.

```typescript
async open(): Promise<void> {
	const command = this.command;
	await commands.executeCommand<void>(command.command, ...(command.arguments || []));
}

async openFile(): Promise<void> {
	const command = this._commandResolver.resolveFileCommand(this);
	await commands.executeCommand<void>(command.command, ...(command.arguments || []));
}

async openChange(): Promise<void> {
	const command = this._commandResolver.resolveChangeCommand(this);
	await commands.executeCommand<void>(command.command, ...(command.arguments || []));
}

async compareWithWorkspace(): Promise<void> {
	const command = this._commandResolver.resolveCompareWithWorkspaceCommand(this);
	await commands.executeCommand<void>(command.command, ...(command.arguments || []));
}
```

**Variations / call-sites:** `extensions/git/src/commands.ts:1197` (dialog choices), `extensions/git/src/commands.ts:1214` (file open dialog), `extensions/git/src/repository.ts:805-836` (context setting)

---

#### Pattern: Task Execution Retargeting

**Where:** `extensions/git/src/repository.ts:3513-3542`
**What:** Transforms task execution objects (ProcessExecution/ShellExecution) to target different working directories.

```typescript
function retargetTaskExecution(execution: ProcessExecution | ShellExecution | CustomExecution | undefined, worktreePath: string): ProcessExecution | ShellExecution | CustomExecution | undefined {
	if (!execution) {
		return undefined;
	}

	if (execution instanceof ProcessExecution) {
		return new ProcessExecution(execution.process, execution.args, {
			...execution.options,
			cwd: worktreePath
		});
	}

	if (execution instanceof ShellExecution) {
		if (execution.commandLine !== undefined) {
			return new ShellExecution(execution.commandLine, {
				...execution.options,
				cwd: worktreePath
			});
		}

		if (execution.command !== undefined) {
			return new ShellExecution(execution.command, execution.args ?? [], {
				...execution.options,
				cwd: worktreePath
			});
		}
	}

	return execution;
}
```

**Variations / call-sites:** `extensions/git/src/repository.ts:1995-2003` (task execution)

---

#### Pattern: Configuration and Event Filtering

**Where:** `extensions/git/src/repository.ts:366-392`
**What:** Uses filterEvent utility for scoped configuration change listening and debounced operation tracking.

```typescript
const onDidChange = filterEvent(workspace.onDidChangeConfiguration, e => e.affectsConfiguration('git', Uri.file(this.repository.root)));
onDidChange(_ => this.updateEnablement());

this.repository.onDidChangeOperations(() => {
	if (!this._modelDisposed) {
		this.repositoryView?.updateRepositoryStatus();
	}
});

const start = onceEvent(filterEvent(this.repository.onDidChangeOperations, () => this.repository.operations.shouldShowProgress()));
const end = onceEvent(filterEvent(debounceEvent(this.repository.onDidChangeOperations, 300), () => !this.repository.operations.shouldShowProgress()));
```

**Variations / call-sites:** `extensions/git/src/repository.ts:1012-1040` (input box visibility), `extensions/git/src/repository.ts:1094-1120` (badge count)

---

#### Pattern: Extension Activation and Initialization

**Where:** `extensions/git/src/main.ts:192-251`
**What:** Extension activation sequence with Git binary discovery, IPC server creation, environment setup, and plugin registration.

```typescript
export async function _activate(context: ExtensionContext): Promise<GitExtensionImpl> {
	const disposables: Disposable[] = [];
	context.subscriptions.push(new Disposable(() => Disposable.from(...disposables).dispose()));

	const logger = window.createOutputChannel('Git', { log: true });
	disposables.push(logger);

	const onDidChangeLogLevel = (logLevel: LogLevel) => {
		logger.appendLine(l10n.t('[main] Log level: {0}', LogLevel[logLevel]));
	};
	disposables.push(logger.onDidChangeLogLevel(onDidChangeLogLevel));
	onDidChangeLogLevel(logger.logLevel);

	const { aiKey } = require('../package.json') as { aiKey: string };
	const telemetryReporter = new TelemetryReporter(aiKey);
	deactivateTasks.push(() => telemetryReporter.dispose());

	const config = workspace.getConfiguration('git', null);
	const enabled = config.get<boolean>('enabled');

	if (!enabled) {
		const onConfigChange = filterEvent(workspace.onDidChangeConfiguration, e => e.affectsConfiguration('git'));
		const onEnabled = filterEvent(onConfigChange, () => workspace.getConfiguration('git', null).get<boolean>('enabled') === true);
		const result = new GitExtensionImpl();

		eventToPromise(onEnabled).then(async () => {
			const { model, cloneManager } = await createModel(context, logger, telemetryReporter, disposables);
			result.model = model;
			result.cloneManager = cloneManager;
		});
		return result;
	}

	try {
		const { model, cloneManager } = await createModel(context, logger, telemetryReporter, disposables);

		return new GitExtensionImpl({ model, cloneManager });
	} catch (err) {
		console.warn(err.message);
		logger.warn(`[main] Failed to create model: ${err}`);

		if (!/Git installation not found/.test(err.message || '')) {
			throw err;
		}

		telemetryReporter.sendTelemetryEvent('git.missing');

		commands.executeCommand('setContext', 'git.missing', true);
		warnAboutMissingGit();

		return new GitExtensionImpl();
	} finally {
		disposables.push(new GitProtocolHandler(logger));
	}
}
```

**Variations / call-sites:** `extensions/git/src/main.ts:41-142` (model creation), `extensions/git/src/main.ts:258-264` (public activation entry)

---

## Summary of Patterns Found

The Git extension demonstrates 10 core patterns for VS Code IDE integration:

1. **Process Spawning**: Async child process execution with Node.js `child_process.spawn()`, custom options, and error handling
2. **Execution with Cancellation**: Promise-based execution wrapping with CancellationToken support and resource cleanup
3. **Environment Injection**: Process spawning with merged environment variables and sanitized working directories
4. **Source Control API**: SCM plugin registration with resource groups, quick diff providers, and history providers
5. **Event-Driven Architecture**: EventEmitter-based observable state management for repository operations
6. **File System Watching**: Abstracted file system watcher for .git directory monitoring
7. **Language Features**: Diagnostic collections and code action providers for commit message validation
8. **Command Execution**: Command palette integration via `commands.executeCommand()` API
9. **Task Retargeting**: Transformation of task execution objects for different working directories
10. **Configuration & Filtering**: Scoped workspace configuration listening with event filtering

These patterns cover source control (main gap for Tauri port), terminal integration (task execution), UI interactions (dialogs, commands), file watching, language features (diagnostics), and process management—all critical for porting VS Code's core functionality to Tauri/Rust.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
