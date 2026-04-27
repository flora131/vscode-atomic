# File Locations: git-base Extension Analysis

## Research Context
Analyzing `/extensions/git-base/` (shared git infrastructure consumed by `extensions/git`) for understanding VS Code's source control abstraction layer that would need porting from TypeScript/Electron to Tauri/Rust.

## Implementation

- `extensions/git-base/src/extension.ts` — Entry point; activates extension, registers API commands and folding providers
- `extensions/git-base/src/model.ts` — Core registry for remote source providers; manages provider lifecycle with EventEmitter
- `extensions/git-base/src/remoteSource.ts` — UI logic for remote repository picker (QuickPick); handles provider query/search with debounce/throttle
- `extensions/git-base/src/remoteProvider.ts` — Interface definition for IRemoteSourceProviderRegistry protocol
- `extensions/git-base/src/foldingProvider.ts` — FoldingRangeProvider for git-commit language; implements comment/diff folding logic
- `extensions/git-base/src/util.ts` — Utility functions: toDisposable, done promise helper, Versions comparison namespace
- `extensions/git-base/src/decorators.ts` — Method decorators for debounce and throttle patterns using property descriptors
- `extensions/git-base/src/api/api1.ts` — ApiImpl class implementing API interface; bridges model to external extensions
- `extensions/git-base/src/api/extension.ts` — GitBaseExtensionImpl class; manages enablement state and API version dispatch

## Types / Interfaces

- `extensions/git-base/src/api/git-base.d.ts` — Public API surface: API, GitBaseExtension, RemoteSourceProvider, RemoteSource, PickRemoteSourceOptions, RemoteSourceAction interfaces; serves as single source of truth for extension consumers

## Configuration

- `extensions/git-base/package.json` — Extension manifest; defines git-commit, git-rebase, ignore languages and their syntaxes; activation on "*" event; browser build target and capabilities (virtualWorkspaces, untrustedWorkspaces)
- `extensions/git-base/tsconfig.json` — TypeScript compiler config; references base config, sets rootDir/outDir, includes vscode.d.ts
- `extensions/git-base/tsconfig.browser.json` — Browser-specific TypeScript config (esbuild target)
- `extensions/git-base/.npmrc` — NPM configuration
- `extensions/git-base/.vscodeignore` — Build output exclusions

## Tests

- `extensions/git-base/src/test/foldingProvider.test.ts` — Mocha suite with 16 test cases covering GitCommitFoldingProvider; tests comment block folding, diff block folding, mixed content scenarios, realistic git commit messages

## Syntax / Language Support

- `extensions/git-base/syntaxes/git-commit.tmLanguage.json` — TextMate grammar for git-commit language
- `extensions/git-base/syntaxes/git-rebase.tmLanguage.json` — TextMate grammar for git-rebase language
- `extensions/git-base/syntaxes/ignore.tmLanguage.json` — TextMate grammar for ignore files (.gitignore, etc.)
- `extensions/git-base/languages/git-commit.language-configuration.json` — Language configuration for git-commit (comments, brackets, etc.)
- `extensions/git-base/languages/git-rebase.language-configuration.json` — Language configuration for git-rebase
- `extensions/git-base/languages/ignore.language-configuration.json` — Language configuration for ignore files

## Documentation

- `extensions/git-base/README.md` — Extension overview; documents public API usage pattern; instructs consumers to import git-base.d.ts

## Build & Tooling

- `extensions/git-base/esbuild.mts` — Main esbuild configuration (CommonJS target)
- `extensions/git-base/esbuild.browser.mts` — Browser esbuild configuration for dist/browser/extension.js
- `extensions/git-base/build/update-grammars.js` — Script to update grammar definitions
- `extensions/git-base/package-lock.json` — Dependency lock file
- `extensions/git-base/package.nls.json` — Localization strings for UI
- `extensions/git-base/cgmanifest.json` — Component governance manifest

## Resources

- `extensions/git-base/resources/icons/git.png` — Extension icon

## Notable Clusters

- `extensions/git-base/src/api/` — 3 files (api1.ts, extension.ts, git-base.d.ts) containing the versioned public API contract and implementation
- `extensions/git-base/syntaxes/` — 3 TextMate grammar definitions for git-commit, git-rebase, and ignore file language support
- `extensions/git-base/languages/` — 3 language configuration files providing editor behavior for git-related languages

## Port Relevance Summary

The git-base extension provides a critical abstraction layer for remote repository management and git-related language support. Key porting considerations:

1. **API Surface**: The RemoteSourceProvider interface and registration pattern would need Rust/IPC equivalent
2. **UI Interactions**: QuickPick-based repository selection depends on VS Code's command palette; replacement in Tauri would require custom UI or similar modal mechanism
3. **Folding Provider**: Language feature provider pattern (FoldingRangeProvider) would need reimplementation in Rust with LSP or custom protocol
4. **Event/Disposable Pattern**: VS Code's EventEmitter and Disposable patterns used extensively; Tauri would need equivalent lifecycle/event management
5. **Language Grammar**: TextMate grammars and language configs are VS Code-specific; Tauri port would need compatible syntax highlighting system (likely tree-sitter or similar)
6. **Language Features**: Folding, comment awareness, language detection - all tied to VS Code provider APIs

Total implementation code: ~1,498 LOC across TypeScript, JSON configs, and grammar definitions.
