# Razor Extension - File Location Report

## Scope
`extensions/razor/` — 7 files (1 directory with 44 LOC total)

## Findings

### Configuration & Manifest
- `extensions/razor/package.json` — Extension manifest declaring Razor language support for `.cshtml` and `.razor` files; registers syntax highlighting and language configuration contributions
- `extensions/razor/package.nls.json` — Localization file providing display name and description translations for the extension
- `extensions/razor/language-configuration.json` — Language configuration defining comments, bracket pairs, auto-closing, and surrounding pairs for Razor files
- `extensions/razor/cgmanifest.json` — Component governance manifest referencing the dotnet/razor repository (commit hash 743f32a68c61809b22fd84e8748c3686ef1bb8b8) as a licensed dependency

### Grammar & Syntax
- `extensions/razor/syntaxes/cshtml.tmLanguage.json` — TextMate grammar for CSHTML/Razor syntax highlighting; supports embedded C#, CSS, and JavaScript languages

### Build Tools
- `extensions/razor/build/update-grammar.mjs` — Build script (44 LOC) that updates the grammar from the dotnet/razor repository; patches grammar rules to replace `text.html.basic` with `text.html.derivative` includes

### Metadata
- `extensions/razor/.vscodeignore` — Excludes test, build, and cgmanifest files from the published extension package

## Summary

The `extensions/razor/` directory is a pure language grammar extension with no runtime code or backend service. It provides syntax highlighting and language support for Razor (`.cshtml`, `.razor`) files by:

1. **Grammar source**: Maintains a TextMate grammar pulled from the dotnet/razor repository via an automated build script
2. **Language registration**: Declares Razor as a VS Code language with proper aliases and MIME types
3. **Editor configuration**: Defines bracket matching, auto-close behavior, and comment patterns for the Razor syntax
4. **No IDE functionality**: This extension contains only grammar definitions and configuration—it provides syntax highlighting only, with no language services, IntelliSense, diagnostics, or debugging capabilities

**Relevance to Tauri/Rust port**: This extension would require minimal porting. A Rust-based VSCodium alternative would need:
- To preserve or regenerate the TextMate grammar (language-agnostic, can stay as-is)
- To reimplement the extension API hooks (less critical than full IDE functionality)
- A Rust-based language service provider if enhanced Razor support (IntelliSense, diagnostics) is desired in the future

Current scope is declarative grammar and configuration, not executable code—making it a low-priority item for a Tauri/Rust IDE rewrite.
