# Partition 29: extensions/vscode-colorize-tests/

(no external research applicable)

## Justification

This partition is a pure VS Code extension integration-test harness. Its entire runtime surface is:

- `commands.executeCommand('_workbench.captureSyntaxTokens', ...)` — a private workbench command that drives the TextMate tokenization engine already implemented inside `src/vs/workbench/`.
- `commands.executeCommand('_workbench.captureTreeSitterSyntaxTokens', ...)` — a parallel private command that drives the Tree-sitter tokenization path, also entirely within `src/vs/`.
- Mocha (`suite` / `test`) as the test runner, a Node.js-only concern not relevant to a Tauri/Rust port.
- `jsonc-parser` for parsing semantic-token JSON fixtures — a lightweight pure-JS utility with no native bindings.
- A collection of language fixture files (`test/colorize-fixtures/`) and golden JSON result files used only as test data.

None of these components require external library research for a Tauri port. The tokenization engines (TextMate grammar + vscode-textmate, Tree-sitter + the WASM grammars) are vendored in `src/vs/`, not in this partition. The test harness itself does not ship to end users and would simply be re-run against whatever replacement tokenization backend the port provides.

## Optional one-liners for the porter

These are informational only; no fetching was needed or performed.

- **Tree-sitter (Rust-native)**: https://github.com/tree-sitter/tree-sitter — the canonical Rust crate (`tree-sitter`) that the WASM build in `src/vs/` wraps; a Tauri port could use it natively without WASM overhead.
- **oniguruma-rs / vscode-oniguruma**: https://github.com/nickel-lang/oniguruma-rs and https://github.com/microsoft/vscode-oniguruma — the Oniguruma regex engine used by vscode-textmate for TextMate grammar matching; a Rust port would substitute the `onig` crate or `fancy-regex`.

## Summary

The `extensions/vscode-colorize-tests/` partition contains no externally-sourced libraries relevant to a Tauri/Rust port. It is a test harness that calls two private VS Code workbench commands and asserts token output against golden files. All tokenization logic lives upstream in `src/vs/`. No external research was necessary or applicable for this partition.
