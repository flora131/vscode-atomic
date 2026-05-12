# File Locations: GitHub Extension (SCM API Consumer)

## Summary
The `extensions/github/` directory (21 TypeScript files, 3,119 LOC) implements GitHub-specific functionality as a first-party SCM API consumer. It demonstrates how VS Code's Git API (Git, GitBase) extends the core editor through a comprehensive provider and handler system, including source control history, branch protection, credentials, push error handling, and remote repository management. This extension is a critical reference implementation for understanding the SCM integration surface that would need to be replicated or reimplemented in a Tauri/Rust port.

---

## Implementation

### Core Extension Activation
- **`src/extension.ts`** — Entry point; activates Git API integration, initializes all provider managers, manages lifecycle of telemetry and logging
- **`src/commands.ts`** — Command registration for GitHub operations (publish, pull requests, open on GitHub, copy vscode.dev links); directly uses `vscode.commands.registerCommand()` for command palette integration

### SCM API Providers (Git Extension)
- **`src/historyItemDetailsProvider.ts`** — Implements `SourceControlHistoryItemDetailsProvider`; provides avatar resolution via GraphQL queries, commit author details, message link transformation (issue references → markdown links)
- **`src/credentialProvider.ts`** — Implements `CredentialsProvider`; supplies GitHub auth credentials for git operations, config-driven enablement
- **`src/pushErrorHandler.ts`** — Implements `PushErrorHandler`; detects permission/push protection errors, auto-forks repos, creates pull requests, handles PR template discovery
- **`src/remoteSourcePublisher.ts`** — Implements `RemoteSourcePublisher`; publishes local repos to GitHub via `publishRepository()`
- **`src/branchProtection.ts`** — Implements `BranchProtectionProvider`; queries repository rulesets via GraphQL, caches branch protection rules
- **`src/canonicalUriProvider.ts`** — Implements `CanonicalUriProvider`; maps vscode.dev URIs to GitHub remote URLs

### SCM/Git-Base API Providers
- **`src/remoteSourceProvider.ts`** — Implements `RemoteSourceProvider` (git-base); provides GitHub repo search, user repos, branch detection, remote source actions

### Supporting Services
- **`src/auth.ts`** — Authentication service: manages GitHub OAuth sessions via `vscode.authentication`, constructs Octokit REST/GraphQL clients, proxy handling via `HTTPS_PROXY`
- **`src/links.ts`** — URL generation: builds vscode.dev shareable links, GitHub commit/PR links, branch links (calls Octokit REST API)
- **`src/publish.ts`** — Repository publishing workflow: creates GitHub repos, initializes remotes
- **`src/shareProviders.ts`** — Share menu integration; registers commands for file/editor sharing context menus
- **`src/util.ts`** — Utilities: repository URL parsing, remote detection, disposable cleanup, sequentialization decorator

### Test Suite
- **`src/test/github.test.ts`** — Smoke tests for pull request template discovery and quick-pick UI interaction via `vscode.commands.executeCommand()`

---

## Types / Interfaces

### Git Extension Type Definitions
- **`src/typings/git.d.ts`** — Core git API surface (v1):
  - `API` interface: repositories, state events, registration methods for all providers
  - `Repository` interface: git operations (add, commit, push, fetch, diff, branch management, log, blame)
  - `RepositoryState`: HEAD, refs, remotes, submodules, working tree changes
  - Provider interfaces: `RemoteSourcePublisher`, `RemoteSourceProvider`, `CredentialsProvider`, `PushErrorHandler`, `BranchProtectionProvider`, `SourceControlHistoryItemDetailsProvider`
  - Error codes enum: `GitErrorCodes` (21 distinct error types)

### Git-Base Extension Type Definitions
- **`src/typings/git-base.d.ts`** — Lightweight git foundation API:
  - `RemoteSourceProvider`: query interface for remote repositories
  - `PickRemoteSourceResult`: URL + optional branch selection

### Constants
- **`src/typings/git.constants.ts`** — Git enumeration: `RefType` (Head, RemoteHead, Tag), `Status` (13 file statuses for index/working tree)
- **`src/typings/ref.d.ts`** — (referenced but minimal)

---

## Configuration

### Package Manifest
- **`package.json`** — Bundles GitHub extension:
  - **dependencies**: `@octokit/rest`, `@octokit/graphql`, `@octokit/graphql-schema`, `tunnel`, `@vscode/extension-telemetry`
  - **engine**: vscode ^1.41.0
  - **activationEvents**: `*` (eager activation)
  - **extensionDependencies**: `vscode.git-base`
  - **enabledApiProposals**: canonicalUriProvider, chatSessionsProvider, scmHistoryProvider, shareProvider, timeline (preview features)
  - **contributes**: 9 commands, context menus (file/share, editor context, explorer context, scm/historyItem, timeline/item), configuration (github.branchProtection, github.gitAuthentication, github.gitProtocol, github.showAvatar)

### Build Configuration
- **`tsconfig.json`** — TypeScript compiler config: NodeNext module/resolution, includes vscode type definitions (vscode-dts for proposals)
- **`esbuild.mts`** — ESBuild bundler configuration
- **`package-lock.json`**, **`package.nls.json`** — Dependency lock and localization strings
- **`.vscodeignore`**, **`.npmrc`** — Packaging metadata

---

## Examples / Fixtures

### Test Workspace
- **`testWorkspace/`** — Git repository with pull request template fixtures:
  - `PULL_REQUEST_TEMPLATE.md` / `PULL_REQUEST_TEMPLATE/` directory (root, `docs/`, `.github/`)
  - Test files: `some-markdown.md`, `x.txt`
  - Tests discovery of markdown templates in canonical GitHub locations

---

## Documentation

- **`README.md`** — Extension overview: publish to GitHub, clone from GitHub, git authentication, auto-fork on permission denied
- **`markdown.css`** — Styling for PR template preview (contributes to `markdown.previewStyles`)
- **`images/icon.png`** — Extension icon (GitHub logo)

---

## Notable Clusters

### 1. Provider Registration System
The extension registers 6 distinct providers with the Git API (lines 104-110 in `extension.ts`):
```
- registerCredentialsProvider() → GithubCredentialProvider
- registerPushErrorHandler() → GithubPushErrorHandler
- registerRemoteSourcePublisher() → GithubRemoteSourcePublisher
- registerSourceControlHistoryItemDetailsProvider() → GitHubSourceControlHistoryItemDetailsProvider
- registerBranchProtectionProvider() → GitHubBranchProtectionProviderManager
- (canonicalUriProvider via separate registration)
```
This demonstrates the **extensibility surface** of VS Code's SCM model.

### 2. Authentication Flow
Centralized in `auth.ts`: `getSession()` → GitHub provider → OAuth scopes (repo, workflow, user:email, read:user) → Octokit REST + GraphQL clients. Managed via `vscode.authentication` API (stable) and `OctokitService` for GraphQL caching.

### 3. Error Recovery (Push Error Handler)
`pushErrorHandler.ts` (327 lines) shows sophisticated error handling:
- Permission denied → fork repo, rename remotes (origin ↔ upstream), push to fork, create PR
- Push protection (GH009) → display secret detection error with learn-more link
- PR template discovery in 6 canonical locations (root, docs/, .github/)

### 4. GraphQL Integration
Two major GraphQL queries cached and sequentialized:
- **`ASSIGNABLE_USERS_QUERY`** — Load user avatars for commit authors (historyItemDetailsProvider.ts:15-29)
- **`COMMIT_AUTHOR_QUERY`** — Resolve individual commit author details (historyItemDetailsProvider.ts:31-49)
- **`REPOSITORY_RULESETS_QUERY`** — Branch protection rules (branchProtection.ts:24-49)

### 5. Menu Context Integration
Commands registered for multiple context menus:
- `file/share` — copy vscode.dev link for files
- `editor/context/share`, `editor/lineNumber/context` — copy link with range
- `scm/historyItem/context` — open commit on GitHub
- `timeline/item/context` — timeline item context
- `continueEditSession` — continue in vscode.dev

### 6. Configuration-Driven Features
Three settings govern behavior:
- `github.branchProtection` (bool) — Enable/disable branch protection provider
- `github.gitAuthentication` (bool) — Enable/disable credential provider
- `github.gitProtocol` (https|ssh) — Clone protocol preference
- `github.showAvatar` (bool) — Avatar resolution setting

### 7. Proposed API Surface
Relies on experimental proposals (`enabledApiProposals` in package.json):
- `canonicalUriProvider` — Map vscode.dev URIs to canonical remotes
- `scmHistoryProvider` — Source control history integration
- `shareProvider` — Share menu participation
- `chatSessionsProvider`, `contribEditSessions`, `contribShareMenu`, `contribSourceControlHistoryItemMenu`, `timeline` — Auxiliary proposals

---

## Key API Integration Points

### From `vscode` module (stable APIs used)
- **`commands`**: registerCommand, executeCommand (setContext, vscode.open)
- **`window`**: createOutputChannel, showErrorMessage, withProgress, showInformationMessage, showWarningMessage, showQuickPick
- **`workspace`**: getConfiguration, registerTextDocumentContentProvider, fs operations, onDidChangeConfiguration
- **`extensions`**: getExtension, onDidChange
- **`authentication`**: getSession, onDidChangeSessions (GitHub provider)
- **`Uri`**: file, parse, joinPath, fsPath
- **`Disposable`**, **`EventEmitter`**, **`LogOutputChannel`** — Lifecycle/logging

### From Git API (`src/typings/git.d.ts`)
- **`API.registerXxxProvider()`** — All 6 provider types
- **`Repository.state`** — Read-only access to HEAD, refs, remotes, changes
- **`Repository.XXX()`** — Git command execution (commit, push, pull, branch, checkout, tag, etc.)

---

## Locator Output Summary

This extension is a **critical reference** for SCM integration in VS Code. It demonstrates:

1. **Provider-based extensibility model**: 6 distinct provider interfaces registered at runtime
2. **Event-driven lifecycle**: Repository open/close, configuration changes, authentication session changes
3. **External service integration**: Octokit REST/GraphQL for GitHub API, proxy support
4. **UI integration**: Commands, context menus, dialogs, progress reporting
5. **Error handling & recovery**: Fork creation, PR templates, push protection detection
6. **Telemetry**: TelemetryReporter integration for feature usage tracking
7. **Localization**: l10n.t() for user-facing strings

A Tauri/Rust port would need to replicate the entire `API` surface (repository state queries, 6+ provider registration points) and maintain compatibility with command/menu systems and the authentication model.
