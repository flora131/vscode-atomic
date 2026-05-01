# File Locations for Search Results Extension (`extensions/search-result/`)

## Overview
The search-result extension provides syntax highlighting, symbol navigation, document linking, and definition providers for VS Code's `.code-search` virtual documents. This extension implements the search/navigation surface using a custom language grammar and document link providers—patterns directly relevant to porting search and navigation capabilities to a Tauri/Rust IDE.

## Implementation
- `extensions/search-result/src/extension.ts` - Main extension logic (277 lines)
  - Registers document symbol provider for file-level navigation symbols
  - Implements definition provider with location link generation
  - Registers document link provider for file path linking
  - Implements completion provider for search query directives and flags
  - Parses search result document syntax and maintains parsed result cache
  - Contains `parseSearchResults()` function for document parsing with regex-based line parsing (FILE_LINE_REGEX, RESULT_LINE_REGEX)
  - Implements visual decorations for context and match lines

## Configuration
- `extensions/search-result/package.json` - Extension manifest (68 lines)
  - Declares `search-result` language with `.code-search` file extension
  - Registers activation on `onLanguage:search-result` event
  - Contributes language grammar from `searchResult.tmLanguage.json`
  - Enables virtual workspace and untrusted workspace support
  - Configuration defaults: line numbers disabled for search result editors
  - Main entry point: `./out/extension.js` (Node.js) and `./dist/browser/extension` (browser)

- `extensions/search-result/tsconfig.json` - TypeScript configuration for Node.js build
- `extensions/search-result/tsconfig.browser.json` - TypeScript configuration for browser/web build
- `extensions/search-result/.vscodeignore` - Package exclusion rules

## Types / Interfaces
- Search result line types defined in `src/extension.ts`:
  - `ParsedSearchFileLine` - File header line with location and symbol path
  - `ParsedSearchResultLine` - Result/match line with location links and context flag
  - Type guards: `isFileLine()`, `isResultLine()`
  - `vscode.LocationLink[]` - Standard VS Code location linking interface used extensively
  - `vscode.DefinitionLink[]` - Definition provider return type
  - `vscode.DocumentLink[]` - Document link provider return type

## Syntax / Grammar
- `extensions/search-result/syntaxes/searchResult.tmLanguage.json` - Generated TextMate grammar (syntax coloring)
  - Covers 40+ language scopes (bat, c, clj, cpp, cs, css, dart, diff, dockerfile, fs, go, groovy, html, java, js, json, jsx, less, lua, m, makefile, md, mm, perl, php, ps1, py, r, rb, rs, scala, scss, sh, sql, swift, ts, tsx, vb, xml, yaml, etc.)
  - Header patterns: `# Query:`, `# Flags:`, `# ContextLines:`, `# Including:`, `# Excluding:`
  - Result block patterns with directory, basename, line numbers, elision markers
  - Context/match line differentiation via separators (`:` for matches, ` ` for context)

- `extensions/search-result/syntaxes/generateTMLanguage.js` - Grammar generator (252 lines)
  - Dynamically generates TextMate grammar from language mappings
  - Defines scope naming conventions and capture groups
  - Creates repository entries for each language with file extension patterns

## Build Configuration
- `extensions/search-result/esbuild.mts` - Node.js build configuration
- `extensions/search-result/esbuild.browser.mts` - Browser/web build configuration
- `extensions/search-result/package-lock.json` - Dependency lock file
- Build scripts in package.json:
  - `generate-grammar` - Generates grammar from JavaScript generator
  - `vscode:prepublish` - Gulp-based compilation
  - `compile-web`, `bundle-web`, `typecheck-web` - Web build targets

## Assets
- `extensions/search-result/images/icon.png` - Extension icon
- `extensions/search-result/src/media/refresh-light.svg` - Light theme refresh icon
- `extensions/search-result/src/media/refresh-dark.svg` - Dark theme refresh icon

## Documentation
- `extensions/search-result/README.md` - Brief extension description
  - Notes bundled extension status (disabled but not uninstallable)
  - Lists capabilities: syntax highlighting, symbol information, result highlighting, go-to-definition

## Localization
- `extensions/search-result/package.nls.json` - i18n strings for display name and description

## Notable Implementation Patterns
**Document Link Provider Chain:**
The extension chains three navigation mechanisms:
1. `registerDefinitionProvider` - Jumps to exact match location with character offset tracking
2. `registerDocumentLinkProvider` - Creates clickable file links for each search result file entry
3. `registerDocumentSymbolProvider` - Lists all matched files as symbols for outline navigation

**Search Result Parsing:**
Uses regex-based line-by-line parsing with state tracking:
- Maintains `currentTarget` (target URI) and `currentTargetLocations` across file sections
- Handles elision markers (`⟪ N characters skipped ⟫`) for incomplete result display
- Caches parsed results with version tracking to avoid reparsing unchanged documents
- Supports multi-root workspace path resolution with fallback logic

**Language-Agnostic Syntax:**
The grammar includes 40+ embedded language scopes, enabling syntax highlighting for matched content in multiple languages within the same search results document.

**Workspace-Aware Path Resolution:**
`relativePathToUri()` function handles:
- User data paths (`vscode-userdata://`)
- Absolute paths and untitled files
- Home directory expansions (`~/`)
- Multi-root workspace formatted paths (`workspaceName • relativePath`)
- Fallback path resolution for saved searches across sessions

## Summary
The search-result extension implements a lightweight language server for VS Code's search UI. Core elements for Tauri/Rust porting include:
- Virtual document model with custom syntax (`.code-search` language)
- Multi-provider architecture for navigation (definitions, document links, symbols)
- Language-agnostic syntax highlighting engine (40+ embedded scopes)
- Regex-based document parsing with intelligent caching
- Workspace and multi-root aware path resolution
- VS Code URI scheme handling (file, untitled, vscode-userdata, etc.)

These patterns demonstrate how VS Code's search/navigation surface can be abstracted into a modular extension, making it a solid reference for implementing analogous functionality in a Rust-based IDE.
