# `.vscode/` Directory Analysis: Out-of-Scope Sentinel

## Status: OUT OF SCOPE

The `.vscode/` directory contains **development-time editor settings and extensions**, not runtime IDE functionality.

## Architectural Classification

Per the architectural briefing, `.vscode/` is marked **Skip** because it represents the **developer workspace configuration** for the VS Code project itself, not the core IDE runtime code that would be ported to Tauri/Rust.

## Directory Contents Overview

### Configuration Files (Development Settings)
- `.vscode/settings.json` — Editor preferences, debug settings, extensions (TypeScript, ESLint, GitHub PRs, etc.)
- `.vscode/launch.json` — Debug configurations for Electron, Node, Chrome processes
- `.vscode/tasks.json` — Build/test scripts
- `.vscode/extensions.json` — Recommended dev extensions
- `.vscode/shared.code-snippets` — Editor code snippets for developers
- `.vscode/cglicenses.schema.json` — JSON schema for licensing
- `.vscode/mcp.json` — MCP (Model Context Protocol) configuration
- `.vscode/searches/` — Saved search queries
- `.vscode/notebooks/` — GitHub issue tracking notebooks

### Development Extensions (4 TypeScript Extensions)
These are internal tooling for the VS Code team, not core IDE features:

1. **vscode-pr-pinger** — Shows PRs to review in statusbar (VS Code Team only)
2. **vscode-extras** — npm version checking utility
3. **vscode-selfhost-import-aid** — Import assistance tool
4. **vscode-selfhost-test-provider** — Test runner UI integration

All packaged as `package.json` files with standard TypeScript compilation to `out/` directories.

## Why This Is Out-of-Scope

- **No runtime code**: These are build-time and editor-time configurations
- **No core IDE logic**: Debug configurations, language settings, and developer utilities
- **No architectural patterns**: These files don't demonstrate how the IDE implements features
- **Team-specific tooling**: Designed for Microsoft's internal development workflow
- **Not portable**: These configurations would be replaced entirely in a Tauri/Rust port

## Implications for Porting Research

The Tauri/Rust port should:
1. **Ignore** all `.vscode/` configurations and extensions
2. **Replace** debug/launch patterns with Rust equivalents (e.g., `rust-analyzer`, `codelldb`)
3. **Rebuild** development workflows from scratch for the Rust/Tauri architecture
4. **Focus instead** on `/src` directory for core IDE functionality patterns

---

**Sentinel Status**: `.vscode/` partition confirmed out-of-scope. Zero runtime IDE patterns to document.
