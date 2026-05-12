# .vscode/ Directory Structure — Runtime Configuration Review

## Summary

The `.vscode/` directory (58 total files, ~3,834 LOC) contains pure editor configuration and development tooling for the VS Code monorepo itself. It includes:

- **Launch configurations** (node, chrome, extensionHost processes)
- **Build & test tasks** (transpilation, type-checking, unit/e2e tests)
- **Extension development environment** (3 internal dev extensions)
- **Workspace settings** (TypeScript, Git, ESLint, Rust analyzer config)
- **Development notebooks & searches** (GitHub issue tracking)

This partition is **irrelevant to a Tauri/Rust port** because it defines how developers interact with the TypeScript/Electron codebase during development, not how the runtime itself is structured. The actual VS Code runtime logic lives elsewhere (in `src/`, `extensions/`).

## Configuration

### Launch Configurations
- `.vscode/launch.json` — 45 debug profiles for:
  - Node.js debugger attachments (Extension Host, Shared Process, Search Process, PTY Host, Agent Host, CLI, Main Process)
  - Chrome DevTools Protocol debugging (main renderer, hot reload variant)
  - VS Code Server (web) debugging
  - Extension test runners (API tests, Git tests, GitHub tests, Emmet tests, etc.)
  - Unit test runners (mocha, smoke tests)
  - Language server debugging (HTML, CSS)
  - Monaco Editor / Component Explorer playgrounds

### Build & Development Tasks
- `.vscode/tasks.json` — 38 npm/shell tasks for:
  - Core transpilation (esbuild) with watchers
  - TypeScript compilation (watch-clientd)
  - Extension build (watch-extensionsd)
  - Copilot build (watch-copilotd)
  - Compound build tasks (hygiene checks, restart workflows)
  - Test execution (unit, integration, smoke)
  - Server launches (code-server, code-web)
  - Vite development server (Monaco Editor playground)
  - Component Explorer server
  - ESLint task
  - Pre-launch dependency checks

### Workspace Settings & Extensions
- `.vscode/settings.json` — Editor and development environment configuration:
  - Chat tool auto-approval for test scripts
  - Editor formatting (no tab indentation, async tokenization)
  - TypeScript/JavaScript language settings (import style, quote style)
  - Rust formatter (rust-analyzer)
  - Files exclusion & readonly patterns
  - Git branch protection rules
  - GitHub PR integration settings
  - Jest/Mocha coverage tracking
  - Built-in ESLint flat config
  - Conventional commits scopes (tree, grid, list, git, sash, etc.)

- `.vscode/extensions.json` — 7 recommended extensions:
  - dbaeumer.vscode-eslint
  - editorconfig.editorconfig
  - github.vscode-pull-request-github
  - ms-vscode.vscode-github-issue-notebooks
  - ms-vscode.extension-test-runner
  - typescriptteam.native-preview
  - ms-vscode.ts-customized-language-service

### Development Snippets
- `.vscode/shared.code-snippets` — Reusable code templates:
  - MSFT Copyright Header (javascript, typescript, css, rust scopes)
  - TypeScript constructor injection pattern (@inject)
  - Emitter/Event property pairs

### MCP Configuration
- `.vscode/mcp.json` — Model Context Protocol server definitions:
  - vscode-automation-mcp (test/mcp directory)
  - component-explorer (component fixtures)

### Schema Files
- `.vscode/cglicenses.schema.json` — JSON schema for component license manifests

### Notebooks (GitHub Issues)
- `.vscode/notebooks/inbox.github-issues`
- `.vscode/notebooks/api.github-issues`
- `.vscode/notebooks/grooming-delta.github-issues`
- `.vscode/notebooks/my-work.github-issues`
- `.vscode/notebooks/grooming.github-issues`
- `.vscode/notebooks/verification.github-issues`
- `.vscode/notebooks/papercuts.github-issues`
- `.vscode/notebooks/vscode-dev.github-issues`
- `.vscode/notebooks/endgame.github-issues`
- `.vscode/notebooks/my-endgame.github-issues`

### Code Searches (Saved Searches)
- `.vscode/searches/ts36031.code-search`
- `.vscode/searches/no-any-casts.code-search`

## Internal Development Extensions

### vscode-pr-pinger (3 files)
- `.vscode/extensions/vscode-pr-pinger/src/extension.ts` — PR notification automation
- `.vscode/extensions/vscode-pr-pinger/package.json`
- `.vscode/extensions/vscode-pr-pinger/tsconfig.json`
- `.vscode/extensions/vscode-pr-pinger/.vscodeignore`
- `.vscode/extensions/vscode-pr-pinger/package-lock.json`

### vscode-extras (3 files)
- `.vscode/extensions/vscode-extras/src/extension.ts` — Development convenience features
- `.vscode/extensions/vscode-extras/src/npmUpToDateFeature.ts` — npm version checking
- `.vscode/extensions/vscode-extras/package.json`
- `.vscode/extensions/vscode-extras/tsconfig.json`
- `.vscode/extensions/vscode-extras/package-lock.json`

### vscode-selfhost-test-provider (13 files)
- `.vscode/extensions/vscode-selfhost-test-provider/src/extension.ts` — Main test provider
- `.vscode/extensions/vscode-selfhost-test-provider/src/failingDeepStrictEqualAssertFixer.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/failureTracker.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/debounce.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/importGraph.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/memoize.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/coverageProvider.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/v8CoverageWrangling.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/metadata.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/stackTraceParser.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/snapshot.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/testTree.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/v8CoverageWrangling.test.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/testOutputScanner.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/vscodeTestRunner.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/sourceUtils.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/src/streamSplitter.ts`
- `.vscode/extensions/vscode-selfhost-test-provider/icon.png`
- `.vscode/extensions/vscode-selfhost-test-provider/package.json`
- `.vscode/extensions/vscode-selfhost-test-provider/tsconfig.json`
- `.vscode/extensions/vscode-selfhost-test-provider/.vscode/settings.json`
- `.vscode/extensions/vscode-selfhost-test-provider/.vscode/launch.json`
- `.vscode/extensions/vscode-selfhost-test-provider/package-lock.json`

### vscode-selfhost-import-aid (5 files)
- `.vscode/extensions/vscode-selfhost-import-aid/src/extension.ts` — Import path resolution helper
- `.vscode/extensions/vscode-selfhost-import-aid/package.json`
- `.vscode/extensions/vscode-selfhost-import-aid/tsconfig.json`
- `.vscode/extensions/vscode-selfhost-import-aid/.vscode/settings.json`
- `.vscode/extensions/vscode-selfhost-import-aid/.vscode/launch.json`
- `.vscode/extensions/vscode-selfhost-import-aid/package-lock.json`

## Relevance to Tauri/Rust Port

**None of the files in `.vscode/` directly inform a port to Tauri/Rust.** This directory is purely developer ergonomics for working on the TypeScript/Electron codebase:

- Debug profiles attach to Node.js processes (Extension Host, Main Process, CLI) — irrelevant if porting to Rust
- Build tasks orchestrate TypeScript/esbuild transpilation — irrelevant if migrating to Rust
- Development extensions provide helpers for navigating TypeScript imports and test execution — not applicable to a Rust project
- Workspace settings optimize the editor for TypeScript development, Git workflows, and coverage tracking — environment-specific

To understand VS Code's architecture for porting, look instead at:
- `src/` — Actual TypeScript source code (main process, renderer, CLI, extensions API)
- `build/` — Build pipeline definitions (esbuild config, webpack rules, electron build scripts)
- `scripts/` — Runtime entry points (code.sh, code-server.sh)
- `extensions/` — Built-in extension implementations
- `product.json` — Feature flags and runtime configuration

