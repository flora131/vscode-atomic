# Pattern Analysis: Razor Extension (extensions/razor/)

## Summary

The Razor extension (`extensions/razor/`) is a **grammar-only language contribution package** with no runtime patterns.

## Scope Findings

The extension contains exclusively:
- **package.json**: Language registration and grammar contribution metadata
- **language-configuration.json**: Bracket matching and auto-closing pair rules (static configuration)
- **syntaxes/cshtml.tmLanguage.json**: TextMate grammar definition for Razor syntax highlighting
- **build/update-grammar.mjs**: Utility script to fetch and patch grammar from upstream (dotnet/razor)
- **cgmanifest.json** and **.vscodeignore**: Build/packaging metadata

## Analysis

This package contributes only declarative VS Code extension features:
1. Language ID registration for `.cshtml` and `.razor` files
2. TextMate grammar rules for syntax highlighting
3. Language configuration (bracket pairs, comment syntax)
4. Embedded language support (C#, CSS, JavaScript)

There is **no TypeScript/JavaScript runtime code** that performs IDE functionality, no activation events, no extension API calls, and no command implementations.

The build script (`update-grammar.mjs`) is a maintenance utility that pulls grammar updates from the upstream dotnet/razor repository—it does not represent VS Code IDE functionality.

## Relevance to Tauri/Rust Port

This extension demonstrates **zero IDE runtime patterns** and thus contributes no insights to a Tauri/Rust port of VS Code's core functionality. Grammar and static language configuration would be handled differently in a Rust-based IDE system (e.g., via tree-sitter grammars or similar), but that is architectural redesign, not pattern extraction.

**Verdict**: Skip—no runtime code or patterns to document.
