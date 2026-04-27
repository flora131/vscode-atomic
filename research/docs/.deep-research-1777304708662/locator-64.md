# File Locations for Razor Extension

## Configuration

- `extensions/razor/package.json` - Extension manifest defining language support for Razor/CSHTML
- `extensions/razor/language-configuration.json` - Language configuration for syntax support (brackets, comments, auto-closing pairs)
- `extensions/razor/.vscodeignore` - Packaging ignore rules
- `extensions/razor/cgmanifest.json` - Component governance manifest

## Build / Tooling

- `extensions/razor/build/update-grammar.mjs` - Build script for updating Razor grammar from dotnet/razor repository

## Grammar / Syntax

- `extensions/razor/syntaxes/cshtml.tmLanguage.json` - TextMate grammar definition for CSHTML syntax highlighting

## Localization

- `extensions/razor/package.nls.json` - Localization strings for UI text

## Notable Clusters

The Razor extension is a minimal language support extension (7 files total, 44 LOC) with no implementation code, tests, or runtime logic. It functions as a pure syntax and language definition module for the Razor/CSHTML file format.

## Relevance to Tauri/Rust Port

The Razor extension provides only language syntax support through TextMate grammars and VS Code extension configuration—functionality tied to VS Code's declarative extension API. A Tauri/Rust migration would need to replicate or reimplement syntax highlighting via alternative grammar systems (Tree-sitter, custom lexers) and provide equivalent language configuration mechanisms, but this extension contains no IDE-specific business logic, editor integration code, or platform-dependent functionality that would be central to a core IDE port.
