# GitHub Extension File Locations

## Implementation

### Core Extension Logic
- `extensions/github/src/extension.ts` — Extension activation, initialization of Git/GitBase extensions, context setup
- `extensions/github/src/commands.ts` — Command registration (10 commands: publish, copy links, open on GitHub, PR management)
- `extensions/github/src/publish.ts` — Repository publishing workflow (Octokit integration, user auth, repo creation)
- `extensions/github/src/auth.ts` — GitHub authentication (OctokitService, session management, GraphQL + REST API)
- `extensions/github/src/links.ts` — vscode.dev link generation, file/notebook position handling, range formatting
- `extensions/github/src/util.ts` — Utilities (DisposableStore, URL parsing, repository detection, decorators)

### GitHub Feature Providers
- `extensions/github/src/credentialProvider.ts` — Git credential provider for GitHub authentication
- `extensions/github/src/remoteSourceProvider.ts` — Provides GitHub repositories as remote sources (clone source)
- `extensions/github/src/remoteSourcePublisher.ts` — Wrapper for publish functionality via Git extension
- `extensions/github/src/branchProtection.ts` — Branch protection rules via GitHub GraphQL API queries
- `extensions/github/src/pushErrorHandler.ts` — Push error handling, fork creation, PR template discovery
- `extensions/github/src/shareProviders.ts` — vscode.dev share provider registration and context management
- `extensions/github/src/canonicalUriProvider.ts` — Canonical URI provider for SSH/HTTPS/file scheme conversion
- `extensions/github/src/historyItemDetailsProvider.ts` — History item details with commit author avatars, GraphQL queries

## Tests

- `extensions/github/src/test/github.test.ts` — Smoke tests (template discovery, quick-pick selection)
- `extensions/github/src/test/index.ts` — Test suite entry point
- `extensions/github/testWorkspace/` — Contains PR template fixtures for testing across multiple locations (.github/, docs/, root)

## Types / Interfaces

- `extensions/github/src/typings/git.d.ts` — Git API types (Repository, API, BranchProtectionProvider, etc.)
- `extensions/github/src/typings/git-base.d.ts` — Git-base extension types (RemoteSourceProvider, RemoteSourcePublisher)
- `extensions/github/src/typings/git.constants.ts` — Git error codes and RefType enum
- `extensions/github/src/typings/ref.d.ts` — Git reference types

## Configuration

- `extensions/github/package.json` — Extension metadata, commands, menus, capabilities, API proposals (canonicalUriProvider, shareProvider, scmHistoryProvider, timeline)
- `extensions/github/package.nls.json` — Localization strings (command titles, configuration descriptions, welcome messages)
- `extensions/github/tsconfig.json` — TypeScript configuration (ESM module, NodeNext resolution, includes proposed API types)
- `extensions/github/.vscodeignore` — Files ignored during packaging
- `extensions/github/.npmrc` — NPM configuration
- `extensions/github/esbuild.mts` — ESM build configuration with tunnel module shimming

## Dependencies

- `extensions/github/package.json` declares:
  - `@octokit/graphql` (8.2.0) — GraphQL client for GitHub API
  - `@octokit/graphql-schema` (14.4.0) — TypeScript types for GraphQL schema
  - `@octokit/rest` (21.1.0) — REST client for GitHub API
  - `tunnel` (0.0.6) — HTTPS-over-HTTP proxy support
  - `@vscode/extension-telemetry` (1.0.0) — Telemetry reporting
  - `@types/node` (22.x) — Node.js types

## Examples / Fixtures

- `extensions/github/images/icon.png` — Extension icon asset
- `extensions/github/markdown.css` — Markdown preview styles for PR templates
- `extensions/github/testWorkspace/` — Test workspace with multiple PR template locations:
  - `.github/PULL_REQUEST_TEMPLATE.md` and `.github/PULL_REQUEST_TEMPLATE/` (directory)
  - `docs/PULL_REQUEST_TEMPLATE.md` and `docs/PULL_REQUEST_TEMPLATE/` (directory)
  - Root `PULL_REQUEST_TEMPLATE.md` and `PULL_REQUEST_TEMPLATE/` (directory)

## Documentation

- `extensions/github/README.md` — Feature overview (Publish to GitHub, Clone from GitHub, authentication, fork creation)

## Notable Clusters

### Command Surface (10 total)
Commands registered in `commands.ts` include:
- `github.publish` — Main publish workflow
- `github.copyVscodeDevLink*` — Three variants for copying vscode.dev links (with/without range, file-specific)
- `github.openOnGitHub` — Opens commits/history items on GitHub.com
- `github.graph.openOnGitHub`, `github.timeline.openOnGitHub` — Context-specific GitHub navigation
- `github.openOnVscodeDev` — Opens current repo in vscode.dev
- `github.createPullRequest`, `github.openPullRequest` — PR management

### API Proposals Used (8 total)
- `canonicalUriProvider` — For URI scheme normalization
- `chatSessionsProvider` — For collaborative sessions
- `contribEditSessions` — For edit session integration
- `contribShareMenu` — For share menu integration
- `contribSourceControlHistoryItemMenu` — For SCM history menus
- `scmHistoryProvider` — For source control history
- `shareProvider` — For vscode.dev sharing
- `timeline` — For timeline provider

### Integration Points
- **Git Extension API**: Repository management, branch/commit queries, push/pull operations
- **Git-Base Extension**: Remote source discovery and publishing
- **VS Code Authentication**: GitHub OAuth flow with scope: repo, workflow, user:email, read:user
- **Octokit**: REST and GraphQL clients for repository operations, pull requests, branch rules
- **VS Code UI**: Commands, quick picks, menus, progress indicators

---

The GitHub extension spans 14 TypeScript implementation files plus 4 type definition files, orchestrating VS Code's GitHub integration through providers (credentials, share, canonical URI), commands (publish, link generation, PR management), and deep API integrations. Its architecture bridges VS Code's extension APIs with Octokit clients (both REST and GraphQL) to enable publish, authentication, and repository metadata operations. A Tauri/Rust port would require reimplementing the Octokit clients, command registration system, and UI provider mechanisms at the desktop application layer rather than as an extension.
