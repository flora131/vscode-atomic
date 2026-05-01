# Deep Research Locator Report — Partition 7
## extensions/git/ — SCM Integration & Git Operations

### Implementation

#### Core Git & Command Execution
- `extensions/git/src/git.ts` — Git class wrapper, spawn() calls for git subprocess execution
- `extensions/git/src/git-base.ts` — Base git operations abstraction, low-level git bindings
- `extensions/git/src/repository.ts` — Repository class with git command execution, scm bindings
- `extensions/git/src/model.ts` — Data model for repositories and git state, exec operations

#### Source Control Management Integration
- `extensions/git/src/commands.ts` — Git command registration and handlers, registerCommand() calls
- `extensions/git/src/main.ts` — Extension entry point, createSourceControl initialization
- `extensions/git/src/staging.ts` — Staging area logic for git add/remove operations
- `extensions/git/src/terminal.ts` — Terminal integration for git operations

#### Credential & Authentication
- `extensions/git/src/askpassManager.ts` — Password/credential manager for git authentication
- `extensions/git/src/askpass.ts` — Git askpass implementation for SSH/HTTPS credentials
- `extensions/git/src/askpass-main.ts` — Main entry point for askpass subprocess
- `extensions/git/src/cloneManager.ts` — Git clone operation manager with authentication

#### Repository Management
- `extensions/git/src/repositoryCache.ts` — Caching layer for repositories
- `extensions/git/src/autofetch.ts` — Automatic fetch background task
- `extensions/git/src/cache.ts` — General git command result caching

#### UI Providers & Decorations
- `extensions/git/src/decorationProvider.ts` — File decoration icons for git status
- `extensions/git/src/quickDiffProvider.ts` — Quick diff provider for inline changes
- `extensions/git/src/timelineProvider.ts` — Git history timeline visualization
- `extensions/git/src/historyProvider.ts` — Repository history and commits visualization
- `extensions/git/src/historyItemDetailsProvider.ts` — Details view for history items
- `extensions/git/src/artifactProvider.ts` — Artifact provider for scm (diffs, files)

#### Blame & Annotations
- `extensions/git/src/blame.ts` — Git blame annotations on file lines
- `extensions/git/src/hover.ts` — Hover information for blamed lines

#### Editor Integration
- `extensions/git/src/gitEditor.ts` — Custom editor for git-specific content
- `extensions/git/src/git-editor-main.ts` — Main entry point for git editor subprocess
- `extensions/git/src/fileSystemProvider.ts` — Virtual filesystem provider for git objects

#### Features & Handlers
- `extensions/git/src/branchProtection.ts` — Branch protection enforcement
- `extensions/git/src/postCommitCommands.ts` — Post-commit hooks and commands
- `extensions/git/src/protocolHandler.ts` — URL protocol handler for git:// schemes
- `extensions/git/src/remoteSource.ts` — Remote repository sources
- `extensions/git/src/remotePublisher.ts` — Publishing to remotes
- `extensions/git/src/pushError.ts` — Push error handling
- `extensions/git/src/actionButton.ts` — Action button UI in scm view
- `extensions/git/src/watch.ts` — File watching for repository changes
- `extensions/git/src/statusbar.ts` — Status bar integration for git info
- `extensions/git/src/diagnostics.ts` — Diagnostic messages for git issues
- `extensions/git/src/editSessionIdentityProvider.ts` — Edit session identity provider
- `extensions/git/src/operation.ts` — Git operation tracking and state
- `extensions/git/src/emoji.ts` — Emoji utilities for commit messages

#### Utilities
- `extensions/git/src/util.ts` — General utility functions
- `extensions/git/src/uri.ts` — URI handling for git resources

#### IPC (Inter-Process Communication)
- `extensions/git/src/ipc/ipcClient.ts` — IPC client for subprocess communication
- `extensions/git/src/ipcServer.ts` — IPC server for subprocess communication

### Tests

- `extensions/git/src/test/git.test.ts` — Unit tests for Git class and operations
- `extensions/git/src/test/repositoryCache.test.ts` — Repository cache tests
- `extensions/git/src/test/askpassManager.test.ts` — Askpass manager tests
- `extensions/git/src/test/smoke.test.ts` — Smoke tests for extension loading
- `extensions/git/src/test/index.ts` — Test entry point and utilities

### Types / Interfaces

- `extensions/git/src/api/git.d.ts` — Main API type definitions exported to other extensions
- `extensions/git/src/typings/git-base.d.ts` — Git base operation type definitions

### Configuration

- `extensions/git/package.json` — Extension manifest with enabled API proposals for scm features
- `extensions/git/package-nls.json` — Localization strings for UI
- `extensions/git/tsconfig.json` — TypeScript compilation configuration
- `extensions/git/resources/emojis.json` — Emoji database for commits

### API & Public Exports

- `extensions/git/src/api/extension.ts` — Extension API public interface
- `extensions/git/src/api/api1.ts` — API v1 implementation for extension consumers
- `extensions/git/src/api/git.constants.ts` — API constants and version definitions

### Resources

- `extensions/git/resources/emojis.json` — Emoji list for commit message suggestions
- `extensions/git/resources/icons/light/` — Light theme status icons (8 SVG files)
- `extensions/git/resources/icons/dark/` — Dark theme status icons (8 SVG files)
- `extensions/git/resources/icons/git.png` — Git logo/icon

### Documentation

- `extensions/git/README.md` — Extension overview, API usage, and feature documentation

### Build & Utilities

- `extensions/git/build/update-emoji.js` — Build script to update emoji database

### Notable Clusters

- `extensions/git/src/api/` — 4 files: Public API surface for consuming extensions (git.d.ts types, api1 implementation, constants, extension export)
- `extensions/git/src/ipc/` — 2 files: Inter-process communication for credential handling and editor subprocess
- `extensions/git/src/test/` — 5 files: Unit and smoke tests for git operations, caching, credentials
- `extensions/git/resources/icons/` — 17 files: SVG status icons for light/dark themes (added, modified, deleted, renamed, untracked, ignored, copied, conflicted, type-changed)
- `extensions/git/src/` — 54 TypeScript files total covering git command execution, ui providers, authentication, repository management, and vscode.scm API integration

## Summary

The extensions/git/ directory contains the complete VS Code Git extension (62 files, 25,181 LOC), which serves as the primary integration point for source control features. The key surface for porting to Tauri/Rust includes:

1. **Git command execution layer** (git.ts, git-base.ts) — Uses spawn() to invoke git binaries; critical subprocess abstraction for Rust IPC
2. **SCM API bindings** (main.ts, commands.ts, repository.ts) — How vscode.scm.createSourceControl() is called and managed
3. **Credential handling** (askpassManager.ts, askpass.ts) — SSH/HTTPS credential flow via git askpass protocol
4. **IPC infrastructure** (ipcClient.ts, ipcServer.ts) — Subprocess communication patterns used for editors and askpass
5. **Provider ecosystem** (decoration, timeline, history, artifact providers) — UI rendering abstraction layer that would need Rust equivalents
6. **Public API** (api/ folder) — Interface exported to other extensions for git operations
