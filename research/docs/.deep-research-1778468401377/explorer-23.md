# Partition 23 of 80 — Findings

## Scope
`extensions/github/` (21 files, 3,119 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# SCM API Consumer Patterns in GitHub Extension

**Scope:** `extensions/github/` (20 TypeScript files, ~2,413 LOC)  
**Pattern Seed:** `vscode.scm.$METHOD($$$)` — SCM-API consumer patterns  
**Research Date:** 2026-05-11

## Overview

The GitHub extension (`extensions/github/`) is a comprehensive consumer of VS Code's Git/SCM API. It demonstrates multiple patterns for integrating external functionality into VS Code's source control system through a plugin architecture. The API uses a registration pattern where extensions implement provider interfaces and register them with the git API.

---

## Pattern 1: Provider Registration via Disposable Pattern

**Where:** `extensions/github/src/extension.ts:101-111`

**What:** Extensions register themselves as providers by implementing interface contracts and calling `gitAPI.register*()` methods, each returning a Disposable for cleanup.

```typescript
const onDidChangeGitExtensionEnablement = (enabled: boolean) => {
    if (enabled) {
        const gitAPI = extension.getAPI(1);

        disposables.add(registerCommands(gitAPI));
        disposables.add(new GithubCredentialProviderManager(gitAPI));
        disposables.add(new GitHubBranchProtectionProviderManager(gitAPI, context.globalState, octokitService, logger, telemetryReporter));
        disposables.add(gitAPI.registerPushErrorHandler(new GithubPushErrorHandler(telemetryReporter)));
        disposables.add(gitAPI.registerRemoteSourcePublisher(new GithubRemoteSourcePublisher(gitAPI)));
        disposables.add(gitAPI.registerSourceControlHistoryItemDetailsProvider(new GitHubSourceControlHistoryItemDetailsProvider(gitAPI, octokitService, logger)));
        disposables.add(new GitHubCanonicalUriProvider(gitAPI));
        disposables.add(new VscodeDevShareProvider(gitAPI));
        setGitHubContext(gitAPI, disposables);

        commands.executeCommand('setContext', 'git-base.gitEnabled', true);
    } else {
        disposables.dispose();
    }
};
```

**Variations / call-sites:**
- `gitAPI.registerPushErrorHandler()` → `extensions/github/src/pushErrorHandler.ts`
- `gitAPI.registerRemoteSourcePublisher()` → `extensions/github/src/remoteSourcePublisher.ts`
- `gitAPI.registerSourceControlHistoryItemDetailsProvider()` → `extensions/github/src/historyItemDetailsProvider.ts`
- `gitAPI.registerCredentialsProvider()` → `extensions/github/src/credentialProvider.ts:38`
- `gitAPI.registerBranchProtectionProvider()` → `extensions/github/src/branchProtection.ts:63-64,79-82`

---

## Pattern 2: Repository State Query and Iteration

**Where:** `extensions/github/src/extension.ts:77`, `extensions/github/src/branchProtection.ts:63`

**What:** Direct access to `gitAPI.repositories` array to query repository state and iterate for bulk operations.

```typescript
if (gitAPI.repositories.find(repo => repositoryHasGitHubRemote(repo))) {
    commands.executeCommand('setContext', 'github.hasGitHubRepo', true);
} else {
    const openRepoDisposable = gitAPI.onDidOpenRepository(async e => {
        await e.status();
        if (repositoryHasGitHubRemote(e)) {
            commands.executeCommand('setContext', 'github.hasGitHubRepo', true);
            openRepoDisposable.dispose();
        }
    });
    disposables.add(openRepoDisposable);
}
```

**Variations / call-sites:**
- `gitAPI.repositories.find()` → `extensions/github/src/branchProtection.ts:63`, `extensions/github/src/commands.ts:212`, `extensions/github/src/links.ts:139`, `extensions/github/src/shareProviders.ts:36,49`
- Bulk iteration: `extensions/github/src/branchProtection.ts:63` iterates all repos to register branch protection providers
- Event-driven: `gitAPI.onDidOpenRepository()` → `extensions/github/src/extension.ts:80`, `extensions/github/src/branchProtection.ts:79`, `extensions/github/src/shareProviders.ts:40`

---

## Pattern 3: Repository Lookup by URI

**Where:** `extensions/github/src/commands.ts:52`, `extensions/github/src/commands.ts:225`

**What:** Resolve a Repository object from a file URI using `gitAPI.getRepository()` to perform git operations.

```typescript
function resolveSessionRepo(gitAPI: GitAPI, sessionMetadata: { worktreePath?: string } | undefined, showErrors: boolean): ResolvedSessionRepo | undefined {
    if (!sessionMetadata?.worktreePath) {
        return undefined;
    }

    const worktreeUri = vscode.Uri.file(sessionMetadata.worktreePath);
    const repository = gitAPI.getRepository(worktreeUri);

    if (!repository) {
        if (showErrors) {
            vscode.window.showErrorMessage(vscode.l10n.t('Could not find a git repository for the session worktree.'));
        }
        return undefined;
    }
    
    // ... subsequent operations on repository object
    const head = repository.state.HEAD;
    // ... access repository.push(), repository.getBranches(), etc.
}
```

**Variations / call-sites:**
- `gitAPI.getRepository(uri)` → `extensions/github/src/commands.ts:52,225`, `extensions/github/src/historyItemDetailsProvider.ts` (indirect)
- Chained with `.rootUri.fsPath` comparison → `extensions/github/src/commands.ts:212`

---

## Pattern 4: Credentials Provider Interface Implementation

**Where:** `extensions/github/src/credentialProvider.ts:12-22`

**What:** Implement `CredentialsProvider` interface with single async method to supply git credentials for a host.

```typescript
class GitHubCredentialProvider implements CredentialsProvider {

    async getCredentials(host: Uri): Promise<Credentials | undefined> {
        if (!/github\.com/i.test(host.authority)) {
            return;
        }

        const session = await getSession();
        return { username: session.account.id, password: session.accessToken };
    }
}
```

**Variations / call-sites:**
- Manager wrapper pattern → `extensions/github/src/credentialProvider.ts:24-64` (`GithubCredentialProviderManager`) provides lazy initialization with config-driven enablement
- Registration: `extensions/github/src/credentialProvider.ts:38` calls `this.gitAPI.registerCredentialsProvider()`

---

## Pattern 5: Push Error Handler with Error Code Dispatch

**Where:** `extensions/github/src/pushErrorHandler.ts:101-169`

**What:** Implement `PushErrorHandler` to handle specific git error codes and take corrective action (fork creation, push protection handling).

```typescript
export class GithubPushErrorHandler implements PushErrorHandler {

    async handlePushError(repository: Repository, remote: Remote, refspec: string, error: Error & { stderr: string; gitErrorCode: GitErrorCodes }): Promise<boolean> {
        if (error.gitErrorCode !== GitErrorCodes.PermissionDenied && error.gitErrorCode !== GitErrorCodes.PushRejected) {
            return false;
        }

        const remoteUrl = remote.pushUrl || (isInCodespaces() ? remote.fetchUrl : undefined);
        if (!remoteUrl) {
            return false;
        }

        const match = /^(?:https:\/\/github\.com\/|git@github\.com:)([^\/]+)\/([^\/.]+)/i.exec(remoteUrl);
        if (!match) {
            return false;
        }

        const [, owner, repo] = match;

        if (error.gitErrorCode === GitErrorCodes.PermissionDenied) {
            await this.handlePermissionDeniedError(repository, remote, refspec, owner, repo);
            this.telemetryReporter.sendTelemetryEvent('pushErrorHandler', { handler: 'PermissionDenied' });
            return true;
        }

        if (/GH009: Secrets detected!/i.test(error.stderr)) {
            await this.handlePushProtectionError(owner, repo, error.stderr);
            this.telemetryReporter.sendTelemetryEvent('pushErrorHandler', { handler: 'PushRejected.PushProtection' });
            return true;
        }

        return false;
    }
}
```

**Variations / call-sites:**
- Registration: `extensions/github/src/extension.ts:106` calls `gitAPI.registerPushErrorHandler()`
- Error handling sub-patterns: `handlePermissionDeniedError()` creates forks via Octokit API; `handlePushProtectionError()` displays error context in editor

---

## Pattern 6: Branch Protection Provider with Event-Driven Updates

**Where:** `extensions/github/src/branchProtection.ts:51-256`

**What:** Implement `BranchProtectionProvider` with event emitter for reactive updates; query GitHub GraphQL for ruleset information; cache results in extension global state.

```typescript
export class GitHubBranchProtectionProviderManager {

    private readonly providerDisposables = new DisposableStore();
    private _enabled = false;

    private set enabled(enabled: boolean) {
        if (this._enabled === enabled) {
            return;
        }

        if (enabled) {
            for (const repository of this.gitAPI.repositories) {
                this.providerDisposables.add(this.gitAPI.registerBranchProtectionProvider(repository.rootUri, new GitHubBranchProtectionProvider(repository, this.globalState, this.octokitService, this.logger, this.telemetryReporter)));
            }
        } else {
            this.providerDisposables.dispose();
        }

        this._enabled = enabled;
    }

    constructor(
        private readonly gitAPI: API,
        private readonly globalState: Memento,
        private readonly octokitService: OctokitService,
        private readonly logger: LogOutputChannel,
        private readonly telemetryReporter: TelemetryReporter) {
        
        this.disposables.add(this.gitAPI.onDidOpenRepository(repository => {
            if (this._enabled) {
                this.providerDisposables.add(gitAPI.registerBranchProtectionProvider(repository.rootUri,
                    new GitHubBranchProtectionProvider(repository, this.globalState, this.octokitService, this.logger, this.telemetryReporter)));
            }
        }));

        this.disposables.add(workspace.onDidChangeConfiguration(e => {
            if (e.affectsConfiguration('github.branchProtection')) {
                this.updateEnablement();
            }
        }));

        this.updateEnablement();
    }
}

export class GitHubBranchProtectionProvider implements BranchProtectionProvider {
    private readonly _onDidChangeBranchProtection = new EventEmitter<Uri>();
    onDidChangeBranchProtection = this._onDidChangeBranchProtection.event;

    async provideBranchProtection(): BranchProtection[] {
        return this.branchProtection;
    }

    private async updateRepositoryBranchProtection(): Promise<void> {
        // Query GraphQL, parse rulesets, fire event
        this._onDidChangeBranchProtection.fire(this.repository.rootUri);
        await this.globalState.update(this.globalStateKey, branchProtection);
    }
}
```

**Variations / call-sites:**
- Manager layer handles multi-repo setup; provider layer handles per-repo GraphQL queries
- Event: `_onDidChangeBranchProtection.fire()` → `extensions/github/src/branchProtection.ts:208`
- State persistence: `globalState.update()` → `extensions/github/src/branchProtection.ts:211`

---

## Pattern 7: Source Control History Item Details Provider

**Where:** `extensions/github/src/historyItemDetailsProvider.ts:80-337`

**What:** Implement `SourceControlHistoryItemDetailsProvider` to provide commit metadata (avatars, hover commands, message links) by querying GitHub GraphQL and caching results.

```typescript
export class GitHubSourceControlHistoryItemDetailsProvider implements SourceControlHistoryItemDetailsProvider {
    private _isUserAuthenticated = true;
    private readonly _store = new Map<string, GitHubRepositoryStore>();
    private readonly _disposables = new DisposableStore();

    async provideAvatar(repository: Repository, query: AvatarQuery): Promise<Map<string, string | undefined> | undefined> {
        const config = workspace.getConfiguration('github', repository.rootUri);
        const showAvatar = config.get<boolean>('showAvatar', true) === true;

        if (!this._isUserAuthenticated || !showAvatar) {
            return undefined;
        }

        const descriptor = getRepositoryDefaultRemote(repository, ['upstream', 'origin']);
        if (!descriptor) {
            return undefined;
        }

        try {
            await this._loadAssignableUsers(descriptor);
            const repositoryStore = this._store.get(this._getRepositoryKey(descriptor));

            const authorQuery = groupBy<AvatarQueryCommit>(query.commits, compareAvatarQuery);
            const results = new Map<string, string | undefined>();

            await Promise.all(authorQuery.map(async commits => {
                if (commits.length === 0) {
                    return;
                }

                // Cache hit
                const avatarUrl = repositoryStore.users.find(
                    user => user.email === commits[0].authorEmail || user.name === commits[0].authorName)?.avatarUrl;

                if (avatarUrl) {
                    commits.forEach(({ hash }) => results.set(hash, `${avatarUrl}&s=${query.size}`));
                    return;
                }

                // Try to extract user ID from GitHub no-reply email
                const userIdFromEmail = getUserIdFromNoReplyEmail(commits[0].authorEmail);
                if (userIdFromEmail) {
                    const avatarUrl = getAvatarLink(userIdFromEmail, query.size);
                    commits.forEach(({ hash }) => results.set(hash, avatarUrl));
                    return;
                }

                // Query commit details via GraphQL
                const commitAuthor = await this._getCommitAuthor(descriptor, commits[0].hash);
                // ...
            }));

            return results;
        } catch (err) {
            if (err instanceof AuthenticationError) {
                this._isUserAuthenticated = false;
            }
            return undefined;
        }
    }

    async provideHoverCommands(repository: Repository): Promise<Command[] | undefined> {
        const url = getRepositoryDefaultRemoteUrl(repository, ['origin', 'upstream']);
        if (!url) {
            return undefined;
        }

        return [{
            title: l10n.t('{0} Open on GitHub', '$(github)'),
            tooltip: l10n.t('Open on GitHub'),
            command: 'github.openOnGitHub',
            arguments: [url]
        }];
    }

    async provideMessageLinks(repository: Repository, message: string): Promise<string | undefined> {
        const descriptor = getRepositoryDefaultRemote(repository, ['upstream', 'origin']);
        if (!descriptor) {
            return undefined;
        }

        return message.replace(ISSUE_EXPRESSION, (match, ...) => {
            // Replace #123 with markdown link to GitHub issue
        });
    }
}
```

**Variations / call-sites:**
- Three provider methods: `provideAvatar()`, `provideHoverCommands()`, `provideMessageLinks()`
- Caching: In-memory `_store` with fallback to config check
- GraphQL queries: `ASSIGNABLE_USERS_QUERY`, `COMMIT_AUTHOR_QUERY` → `extensions/github/src/historyItemDetailsProvider.ts:15-49`
- Sequentialization decorator: `@sequentialize` on `_loadAssignableUsers()` → `extensions/github/src/historyItemDetailsProvider.ts:264`

---

## Pattern 8: Remote Source Provider with Query and Action Support

**Where:** `extensions/github/src/remoteSourceProvider.ts:32-147`

**What:** Implement `RemoteSourceProvider` to enable cloning from GitHub and provide branch-level actions (open on GitHub, checkout on vscode.dev).

```typescript
export class GithubRemoteSourceProvider implements RemoteSourceProvider {

    readonly name = 'GitHub';
    readonly icon = 'github';
    readonly supportsQuery = true;

    private userReposCache: RemoteSource[] = [];

    async getRemoteSources(query?: string): Promise<RemoteSource[]> {
        const octokit = await getOctokit();

        if (query) {
            const repository = getRepositoryFromUrl(query);
            if (repository) {
                const raw = await octokit.repos.get(repository);
                return [asRemoteSource(raw.data)];
            }
        }

        const all = await Promise.all([
            this.getQueryRemoteSources(octokit, query),
            this.getUserRemoteSources(octokit, query),
        ]);

        const map = new Map<string, RemoteSource>();
        for (const group of all) {
            for (const remoteSource of group) {
                map.set(remoteSource.name, remoteSource);
            }
        }

        return [...map.values()];
    }

    async getBranches(url: string): Promise<string[]> {
        const repository = getRepositoryFromUrl(url);
        if (!repository) {
            return [];
        }

        const octokit = await getOctokit();
        const branches: string[] = [];
        let page = 1;

        while (true) {
            const res = await octokit.repos.listBranches({ ...repository, per_page: 100, page });
            if (res.data.length === 0) {
                break;
            }

            branches.push(...res.data.map(b => b.name));
            page++;
        }

        const repo = await octokit.repos.get(repository);
        const defaultBranch = repo.data.default_branch;

        return branches.sort((a, b) => a === defaultBranch ? -1 : b === defaultBranch ? 1 : 0);
    }

    async getRemoteSourceActions(url: string): Promise<RemoteSourceAction[]> {
        const repository = getRepositoryFromUrl(url);
        if (!repository) {
            return [];
        }

        return [{
            label: l10n.t('Open on GitHub'),
            icon: 'github',
            run(branch: string) {
                const link = getBranchLink(url, branch);
                env.openExternal(Uri.parse(link));
            }
        }, {
            label: l10n.t('Checkout on vscode.dev'),
            icon: 'globe',
            run(branch: string) {
                const link = getBranchLink(url, branch, getVscodeDevHost());
                env.openExternal(Uri.parse(link));
            }
        }];
    }
}
```

**Variations / call-sites:**
- Registration via git-base extension: `extensions/github/src/extension.ts:53` calls `gitBaseAPI.registerRemoteSourceProvider()`
- Query modes: User repos, search repos, direct URL parsing
- Pagination: `getBranches()` implements page-based iteration with `per_page: 100`

---

## Pattern Summary

The GitHub extension demonstrates these core SCM API patterns:

1. **Provider Registration:** Implement interface contracts + register via `gitAPI.register*()` methods returning Disposables
2. **Repository Querying:** Direct array access (`gitAPI.repositories`), lookup by URI (`gitAPI.getRepository()`), event-driven discovery (`gitAPI.onDidOpenRepository()`)
3. **Credential Provisioning:** `CredentialsProvider` interface returns username/password for named hosts
4. **Error Handling:** `PushErrorHandler` intercepts git errors with code dispatch to implement recovery (fork creation, etc.)
5. **Branch Protection:** `BranchProtectionProvider` with EventEmitter for reactive ruleset updates cached in global state
6. **History Enhancement:** `SourceControlHistoryItemDetailsProvider` multi-method contract for avatars, hover commands, message link resolution
7. **Remote Sourcing:** `RemoteSourceProvider` for clone integration with branch queries and context-menu actions

All patterns use Disposable cleanup, TypeScript interfaces, async/await, and composition with event emitters for reactivity. GraphQL queries integrated via Octokit for GitHub API access. Configuration-driven enablement via `workspace.onDidChangeConfiguration()`.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
