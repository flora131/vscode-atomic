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

