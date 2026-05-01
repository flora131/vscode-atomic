# Partition 36: extensions/git-base/ — File Locator Report

## Summary
This partition contains the git-base extension, a foundational VS Code extension that provides:
- Git-specific language support (git-commit, git-rebase, ignore file syntax)
- Remote repository picker API for source control extensions
- Folding ranges for git commit messages
- Inter-extension API contracts for git operations

Relevance to Tauri/Rust port: Documents the extension API contract patterns that would need preservation, including event emitters, disposables, and provider interfaces for extensibility.

## Implementation

- `extensions/git-base/src/api/extension.ts` — GitBaseExtensionImpl class implementing GitBaseExtension interface; manages model state and API versioning
- `extensions/git-base/src/api/api1.ts` — ApiImpl class implementing API v1 contract; delegates to model methods and registers API commands
- `extensions/git-base/src/extension.ts` — Main extension activation; creates model, registers API commands, and folds provider for git-commit language
- `extensions/git-base/src/model.ts` — Model class implementing IRemoteSourceProviderRegistry; manages registered remote source providers with event emission
- `extensions/git-base/src/remoteSource.ts` — Core UI implementation: pickRemoteSource() and getRemoteSourceActions() functions; QuickPick-based provider selection flow
- `extensions/git-base/src/remoteProvider.ts` — IRemoteSourceProviderRegistry interface defining provider registration contract and events
- `extensions/git-base/src/foldingProvider.ts` — GitCommitFoldingProvider class implementing FoldingRangeProvider for git-commit syntax folding
- `extensions/git-base/src/decorators.ts` — Throttle and debounce decorator implementations for async method throttling
- `extensions/git-base/src/util.ts` — Utility functions: toDisposable(), done() promise helper, Versions namespace for semantic version comparison

## Types / Interfaces

- `extensions/git-base/src/api/git-base.d.ts` — Master TypeScript definition file exporting all public API types: API, GitBaseExtension, RemoteSourceProvider, RemoteSource, RemoteSourceAction, PickRemoteSourceOptions, PickRemoteSourceResult, RecentRemoteSource

## Configuration

- `extensions/git-base/package.json` — Extension manifest (v10.0.0); defines git-commit, git-rebase, ignore language contributions; registers commands; specifies virtualWorkspaces and untrustedWorkspaces support
- `extensions/git-base/tsconfig.json` — TypeScript configuration extending tsconfig.base.json; compiles src/ to out/
- `extensions/git-base/esbuild.mts` — ESBuild configuration for bundling extension to dist/ (Node platform)
- `extensions/git-base/esbuild.browser.mts` — Browser-specific ESBuild configuration (reference only)
- `extensions/git-base/.vscodeignore` — Exclusion patterns for packaged extension
- `extensions/git-base/.npmrc` — NPM configuration
- `extensions/git-base/cgmanifest.json` — Component governance manifest

## Tests

- `extensions/git-base/src/test/foldingProvider.test.ts` — Mocha test suite for GitCommitFoldingProvider; 14 tests covering empty docs, single/multiple comment blocks, diff blocks, mixed content

## Documentation

- `extensions/git-base/README.md` — Extension overview; documents public API usage pattern with example code for consuming extensions (getAPI(1) entry point)

## Examples / Fixtures

- `extensions/git-base/languages/git-commit.language-configuration.json` — Language configuration for git-commit: comment syntax (#), bracket pairs, auto-closing behavior
- `extensions/git-base/languages/git-rebase.language-configuration.json` — Language configuration for git-rebase syntax
- `extensions/git-base/languages/ignore.language-configuration.json` — Language configuration for gitignore-family files

## Notable Clusters

- `extensions/git-base/src/api/` — 3 files (extension.ts, git-base.d.ts, api1.ts): API boundary layer defining extension contract and implementation
- `extensions/git-base/languages/` — 3 files: Static language configuration for git-related file types
- `extensions/git-base/syntaxes/` — 3 files (tmLanguage.json): TextMate grammar definitions for git-commit, git-rebase, ignore languages
- `extensions/git-base/src/test/` — 1 file: Unit test suite for folding provider

## Key Architectural Patterns

**Extension API Contract**: The git-base.d.ts defines a versioned API (v1) with provider registration. Porting would require translating TypeScript interfaces to Rust trait definitions while maintaining the provider pattern.

**Event/Disposable Pattern**: Heavy use of VS Code's Event<T> and Disposable types for resource management. A Tauri port would need equivalent event pub-sub and lifecycle management mechanisms.

**Remote Source Provider Interface**: Extensible provider pattern with optional methods (getBranches, getRemoteSourceActions, getRecentRemoteSources). This demonstrates how built-in extensions expect third-party extensions to plug in.

**Language Configuration**: Static contributions (languages, grammars) defined in package.json. These would likely transfer to a Tauri-equivalent manifest or configuration system.
