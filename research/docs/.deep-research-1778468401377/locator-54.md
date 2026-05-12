# Locator Task 54: Test Runner Configuration

## Scope
`.vscode-test.js` (1 file, 148 LOC) — test runner config

## Configuration

### `/home/norinlavaee/projects/vscode-atomic/.vscode-test.js`
Top-level test runner configuration file using `@vscode/test-cli`. Defines a list of extension test configurations including:
- markdown-language-features
- ipynb
- notebook-renderers
- vscode-colorize-tests
- terminal-suggest
- vscode-colorize-perf-tests
- configuration-editing
- github-authentication
- microsoft-authentication
- vscode-api-tests-folder
- vscode-api-tests-workspace
- git-base
- copilot

Each extension configuration specifies workspace folder, test file patterns, mocha timeout settings, and launch arguments. Includes conditional reporting configuration for CI/CD environments (BUILD_ARTIFACTSTAGINGDIRECTORY, GITHUB_WORKSPACE) with support for browser and remote testing scenarios.

---

**Summary:** Single test runner configuration file that orchestrates extension testing across the VS Code monorepo. No implementation, test, type, documentation, or fixture files associated with this task scope.
