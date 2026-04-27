# Search Result Extension - File Location Index

## Research Question
Porting VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

## Partition: extensions/search-result/

This extension implements virtual search result documents with language features (syntax highlighting, symbol navigation, definition linking) for search result editing—a key pattern for IDE core functionality port design.

---

## Implementation

- `extensions/search-result/src/extension.ts` — Main extension entry point; registers 5 language feature providers (document symbols, completion, definition, document links) for virtual search-result documents; demonstrates vscode API patterns for virtual document consumption
- `extensions/search-result/syntaxes/generateTMLanguage.js` — Build script generating TextMate language grammar for search result syntax highlighting; embeds scope mappings for 40+ language syntax inclusion patterns

---

## Configuration

- `extensions/search-result/package.json` — Extension manifest with activation event `onLanguage:search-result`, capabilities for virtualWorkspaces/untrustedWorkspaces, API proposal `documentFiltersExclusive`
- `extensions/search-result/package.nls.json` — Localization strings for display name and description
- `extensions/search-result/tsconfig.json` — TypeScript compilation target extending base config; includes vscode type definitions
- `extensions/search-result/tsconfig.browser.json` — Browser build TypeScript config extending node target
- `extensions/search-result/.vscodeignore` — Package exclusion rules for bundled extension

---

## Build / Generation

- `extensions/search-result/esbuild.mts` — ESM-based build orchestration for platform:node entrypoint
- `extensions/search-result/esbuild.browser.mts` — ESM-based browser bundle build
- `extensions/search-result/syntaxes/searchResult.tmLanguage.json` — Generated TextMate grammar JSON (output artifact from generateTMLanguage.js)

---

## Assets

- `extensions/search-result/images/icon.png` — Extension icon
- `extensions/search-result/src/media/refresh-dark.svg` — UI asset (dark mode)
- `extensions/search-result/src/media/refresh-light.svg` — UI asset (light mode)

---

## Documentation

- `extensions/search-result/README.md` — Feature summary: syntax highlighting, symbol information, result highlighting, go-to-definition for search results editor

---

## Notable Clusters

- `extensions/search-result/src/` — 1 file; core TypeScript implementation of extension activation and virtual document language feature registration
- `extensions/search-result/syntaxes/` — 2 files; grammar generation script + generated grammar JSON artifact
- `extensions/search-result/` — Configuration and build orchestration files at root (5 TypeScript config + esbuild scripts)

---

## Key Architectural Notes for Port

The extension demonstrates critical vscode API patterns that a Tauri/Rust port must replicate:

1. **Language Feature Provider Registration** — Registers document symbol, completion, definition, and document link providers for a virtual document type (`search-result`). These are core IDE intelligence APIs.

2. **Virtual Document URI Scheme** — Handles URI parsing and workspace folder resolution for virtual `untitled`, `vscode-userdata`, and custom file scheme URIs (lines 130–175 in extension.ts).

3. **Document Change Lifecycle** — Implements caching + change listeners for efficient incremental parsing and decoration updates (lines 17, 115–119).

4. **Language Selector** — Uses exclusive document filter (`SEARCH_RESULT_SELECTOR`) with activation event `onLanguage:search-result` to bind features to a non-file-backed document language (line 12, package.json `activationEvents`).

5. **TextMate Grammar Integration** — Integrates syntax highlighting via TextMate scope system with dynamic language inclusion (40+ language syntax scopes embedded in grammar).

6. **API Proposals** — Uses `documentFiltersExclusive` capability (package.json line 34), indicating reliance on cutting-edge vscode extension API for exclusive language document filtering.

---

## File Summary

- **Total files in scope:** 4 (excluding node_modules, package-lock.json)
- **Lines of code:** ~567 (primarily src/extension.ts ~280 lines, generateTMLanguage.js ~250 lines)
- **Languages:** TypeScript, JavaScript, JSON
- **Core Extension Language:** TypeScript with vscode module dependency
- **Build Targets:** Node (Electron) and Browser (web-based VS Code)
