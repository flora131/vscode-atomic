# File Locations for extensions/git-base/ — VS Code Porting Research

## Implementation

- `extensions/git-base/src/extension.ts` — Extension entry point; registers folding provider for git-commit language and initializes API
- `extensions/git-base/src/model.ts` — Data model for remote source provider registry; manages provider lifecycle with event emitters
- `extensions/git-base/src/remoteSource.ts` — UI layer for remote source picker using VS Code QuickPick API with search and branch selection
- `extensions/git-base/src/remoteProvider.ts` — Registry interface definition for managing remote source providers
- `extensions/git-base/src/foldingProvider.ts` — FoldingRangeProvider implementation for git-commit messages (code folding for comments/diffs)
- `extensions/git-base/src/decorators.ts` — Decorator utilities: debounce and throttle functions for performance optimization
- `extensions/git-base/src/util.ts` — Utility functions: Disposable pattern, Promise utilities, version comparison logic

## API / Extension Interface

- `extensions/git-base/src/api/git-base.d.ts` — Public TypeScript API definitions; exports interfaces for RemoteSourceProvider, RemoteSource, API, GitBaseExtension
- `extensions/git-base/src/api/api1.ts` — API v1 implementation; bridges Model to public API for remote source registration and actions
- `extensions/git-base/src/api/extension.ts` — GitBaseExtensionImpl class; manages extension enablement state and API versioning

## Tests

- `extensions/git-base/src/test/foldingProvider.test.ts` — Comprehensive test suite for GitCommitFoldingProvider (25 test cases covering comments, diffs, edge cases)

## Configuration

- `extensions/git-base/package.json` — VS Code extension manifest; defines contribution points (git-commit, git-rebase, ignore languages), commands, grammars
- `extensions/git-base/tsconfig.json` — TypeScript configuration for Node.js platform build
- `extensions/git-base/tsconfig.browser.json` — TypeScript configuration for browser platform (excludes tests)
- `extensions/git-base/esbuild.mts` — Build configuration for Node.js platform using esbuild
- `extensions/git-base/esbuild.browser.mts` — Build configuration for browser platform using esbuild

## Language Definitions

- `extensions/git-base/languages/git-commit.language-configuration.json` — Language config for git-commit (indentation, brackets, comments)
- `extensions/git-base/languages/git-rebase.language-configuration.json` — Language config for git-rebase (indentation, brackets, comments)
- `extensions/git-base/languages/ignore.language-configuration.json` — Language config for ignore files (gitignore, .git-blame-ignore-revs)

## Syntax Definitions

- `extensions/git-base/syntaxes/git-commit.tmLanguage.json` — TextMate grammar for git-commit language syntax highlighting
- `extensions/git-base/syntaxes/git-rebase.tmLanguage.json` — TextMate grammar for git-rebase language syntax highlighting
- `extensions/git-base/syntaxes/ignore.tmLanguage.json` — TextMate grammar for ignore file syntax highlighting

## Build / Dependencies

- `extensions/git-base/build/update-grammars.js` — Build script for updating TextMate grammar definitions from external sources
- `extensions/git-base/package-lock.json` — Locked dependency versions (only @types/node:22.x in devDependencies)
- `extensions/git-base/cgmanifest.json` — Component governance manifest; documents third-party components (textmate/git.tmbundle, walles/git-commit-message-plus)

## Configuration / Metadata

- `extensions/git-base/.vscodeignore` — VS Code packaging exclusion rules
- `extensions/git-base/.npmrc` — NPM configuration
- `extensions/git-base/package.nls.json` — Internationalization strings for package.json display names
- `extensions/git-base/resources/icons/git.png` — Extension icon asset

## Documentation

- `extensions/git-base/README.md` — Extension documentation; explains git-base static contributions and API usage for other extensions

## Notable Clusters

- `extensions/git-base/src/` — 7 files (~711 LOC) - Core extension logic: extension lifecycle, models, remote source UI, code folding, utilities
- `extensions/git-base/src/api/` — 3 files - Public API surface: type definitions, API implementation, extension wrapper
- `extensions/git-base/languages/` — 3 files - Language configuration declarations for git commit, rebase, and ignore file formats
- `extensions/git-base/syntaxes/` — 3 files - TextMate grammar definitions for syntax highlighting across three git-related languages

## Relevance to VS Code Porting

The git-base extension is **minimally relevant** to porting core IDE functionality from TypeScript/Electron to Tauri/Rust. It provides:
- Static language/syntax contributions for git-related file types
- Remote repository picker UI using VS Code extension APIs
- Code folding for git commit messages
- API surface for other extensions to register custom remote providers

These are **specialized source control features** rather than core IDE functionality. However, porting would require:
- Replicating the **extension API architecture** (provider registration pattern, event-driven design)
- Replicating **QuickPick UI** for remote source selection
- Replicating **FoldingRangeProvider** interface for git-commit folding
- Handling **TextMate grammar** system for syntax highlighting (likely via a grammar engine)
- **Internationalization** system for UI strings
