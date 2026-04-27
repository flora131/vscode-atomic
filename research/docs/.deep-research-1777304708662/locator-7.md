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
