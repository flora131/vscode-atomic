# Partition 20: `.vscode/` — SKIPPED (Out of Scope)

**Status: OUT OF SCOPE for Tauri/Rust port research**

---

### Files Analysed
(none — partition is dev-time configuration / selfhost tools)

---

### Notes

The `.vscode/` directory contains exclusively developer-time editor configuration and a set of selfhost-only VS Code extensions used by the vscode repository contributors themselves. None of the contents represent portable application logic, runtime behaviour, or platform abstractions that would need to be carried into a Tauri/Rust port.

**Top-level configuration files verified:**

| File | Purpose |
|---|---|
| `.vscode/settings.json` | Workspace editor settings (formatting, search excludes, TypeScript SDK path, rust-analyzer linked project, git branch protection, GitHub PR defaults, MCP server config) |
| `.vscode/extensions.json` | Recommended extension list (ESLint, EditorConfig, GitHub PR, test runner, native TS preview) |
| `.vscode/launch.json` | Debug launch configurations for contributors |
| `.vscode/tasks.json` | Workspace-level build/test task definitions |
| `.vscode/mcp.json` | MCP server configuration for contributor tooling |
| `.vscode/cglicenses.schema.json` | JSON schema for license manifest validation |
| `.vscode/shared.code-snippets` | Editor code snippet shortcuts for contributors |
| `.vscode/searches/` | Saved search queries (e.g. `ts36031.code-search`, `no-any-casts.code-search`) |
| `.vscode/notebooks/` | GitHub Issues notebook queries (inbox, grooming, endgame, verification, papercuts, etc.) |

**Selfhost extensions under `.vscode/extensions/`:**

| Extension | Role |
|---|---|
| `vscode-selfhost-import-aid` | Auto-import assistance tailored to the vscode monorepo's module graph |
| `vscode-selfhost-test-provider` | Custom test runner integration exposing Mocha results inside VS Code's Test Explorer; includes coverage wrangling, snapshot support, stack-trace parsing, and a failing `deepStrictEqual` assertion fixer |
| `vscode-pr-pinger` | Notifies contributors when their pull requests need attention |
| `vscode-extras` | Miscellaneous contributor utilities including an npm-package up-to-date checker |

All four extensions are TypeScript projects with their own `package.json`, `tsconfig.json`, and `package-lock.json` files. They target the VS Code Extension API and exist solely to improve the inner-loop developer experience when working on the vscode codebase itself.

None of the content in this partition — settings, launch configs, notebooks, searches, snippets, or the selfhost extensions — represents logic that must be ported, replicated, or even consulted when building a Tauri/Rust equivalent of VS Code. The partition is therefore confirmed as fully out of scope and requires no further analysis in the context of this research effort.
