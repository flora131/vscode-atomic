# Online Research: VS Code `.vscode/` Partition — Tauri/Rust Port Relevance

(no external research applicable)

## Justification

The scope for this research partition is the `.vscode/` directory (21 files), which contains exclusively:

- Workspace dogfood configuration files: `launch.json`, `tasks.json`, `settings.json`, and related IDE config
- Developer-only extensions: `selfhost-test-provider` (uses mocha, xterm, node-pty for internal test running) and `pr-pinger` (uses `@octokit/graphql` for GitHub PR notifications)

None of these artifacts are part of VS Code's core IDE functionality that would be subject to a TypeScript/Electron to Tauri/Rust port. The `.vscode/` directory is developer experience infrastructure — it configures the development environment for contributors working on VS Code itself. It does not contain runtime source code, UI components, language server logic, extension host machinery, or any other subsystem that would need to be rewritten in Rust or integrated with Tauri.

The dev-only extensions referenced (`selfhost-test-provider`, `pr-pinger`) are internal CI and developer workflow utilities. They are not shipped as part of VS Code, are not part of the editor's core, and would have no analogue in a Tauri-based port. Their third-party dependencies (mocha, xterm, node-pty, @octokit/graphql) are similarly scoped to the development workflow of the existing codebase.

No external documentation about Tauri, Rust FFI, WebView2, Electron-to-Tauri migration patterns, or native Rust UI frameworks is applicable to this partition because the partition contains no code that would be ported.

## Summary

The `.vscode/` partition is workspace configuration and internal developer tooling. It sits entirely outside the scope of a Tauri/Rust port effort: nothing in this directory is a runtime artifact, a core IDE subsystem, or a third-party library whose Rust equivalent would need to be identified. External research into Tauri or Rust porting strategies provides no actionable guidance for this partition. The correct disposition is to skip external research for this scope.
