# Partition: extensions/cpp/ — SKIP (Grammar-Only)

SENTINEL: This partition contains only TextMate grammars (.tmLanguage.json), snippets, and language configuration files. There is no VS Code runtime API usage and no TypeScript source code relevant to the porting effort.

## Portability Note

TextMate grammar JSON files are host-agnostic by design and require no modification to port. They would be consumed directly by whatever syntax tokenization engine the Rust/Tauri host adopts — for example, a Rust port of `vscode-textmate` (the tokenizer VS Code itself uses), or an equivalent `tree-sitter` grammar if the new host opts for that approach instead.

## Summary

The `extensions/cpp/` partition covers C, C++, and CUDA syntax highlighting and snippet support for VS Code. All artifacts are declarative data files: grammar definitions expressed as TextMate PList-derived JSON, snippet JSON bundles, and a `language-configuration.json` that governs bracket matching and comment toggling. There is no imperative logic, no activation event handler, and no dependency on the VS Code extension host API surface. Because the grammars are pure JSON consumed by the tokenization layer rather than by the extension host runtime, they sit entirely outside the scope of the TypeScript-to-Rust porting effort. Any Tauri-based host that implements or embeds a compatible TextMate grammar engine (whether `vscode-textmate` compiled to Wasm, a native Rust re-implementation, or a tree-sitter grammar as an alternative) can ingest these files without change.
