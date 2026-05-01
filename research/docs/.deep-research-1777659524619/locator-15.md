# File Location Index: extensions/emmet/

**Scope**: `extensions/emmet/` (25 implementation files, 14 test files)

**Research Focus**: Porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust — specifically examining command registration, provider patterns, and editor integration.

---

## Implementation

### Entry Points & Extension Activation
- `extensions/emmet/src/node/emmetNodeMain.ts` — Node.js entry point (registered in package.json `main` field); bootstraps `updateImageSize` command and calls common activation
- `extensions/emmet/src/browser/emmetBrowserMain.ts` — Browser/web entry point (registered in package.json `browser` field); simpler activation path
- `extensions/emmet/src/emmetCommon.ts` — Core activation logic shared by both environments; registers all 18 vscode.commands and 2 provider types

### Command Registration Pattern
- `extensions/emmet/src/emmetCommon.ts` — Registers commands via `vscode.commands.registerCommand()`:
  - `editor.emmet.action.wrapWithAbbreviation`
  - `editor.emmet.action.removeTag`
  - `editor.emmet.action.updateTag`
  - `editor.emmet.action.matchTag`
  - `editor.emmet.action.balanceOut` / `balanceIn`
  - `editor.emmet.action.splitJoinTag`
  - `editor.emmet.action.mergeLines`
  - `editor.emmet.action.toggleComment`
  - `editor.emmet.action.nextEditPoint` / `prevEditPoint`
  - `editor.emmet.action.selectNextItem` / `selectPrevItem`
  - `editor.emmet.action.evaluateMathExpression`
  - `editor.emmet.action.incrementNumber*` (×3 variants)
  - `editor.emmet.action.decrementNumber*` (×3 variants)
  - `editor.emmet.action.reflectCSSValue`
  - `workbench.action.showEmmetCommands`

### Provider Registration Pattern
- `extensions/emmet/src/emmetCommon.ts` — Registers language providers:
  - `vscode.languages.registerCompletionItemProvider()` — Standard completion for HTML, CSS, and 12 other markup/stylesheet languages
  - `vscode.languages.registerInlineCompletionItemProvider()` — Conditional inline completions (controlled by `emmet.useInlineCompletions` setting)

### Editor Actions (Command Implementations)
- `extensions/emmet/src/abbreviationActions.ts` — Expands Emmet abbreviations and wrap-with-abbreviation logic
- `extensions/emmet/src/removeTag.ts` — HTML tag removal
- `extensions/emmet/src/updateTag.ts` — HTML tag modification
- `extensions/emmet/src/matchTag.ts` — Tag matching/highlighting
- `extensions/emmet/src/balance.ts` — Expand (balanceOut) and contract (balanceIn) selection
- `extensions/emmet/src/splitJoinTag.ts` — Toggle self-closing tags
- `extensions/emmet/src/mergeLines.ts` — Merge multi-line elements
- `extensions/emmet/src/toggleComment.ts` — Toggle HTML/CSS comments
- `extensions/emmet/src/editPoint.ts` — Navigate next/previous edit points
- `extensions/emmet/src/selectItem.ts` — Item selection (delegates to HTML/CSS-specific modules)
- `extensions/emmet/src/selectItemHTML.ts` — HTML-specific item selection
- `extensions/emmet/src/selectItemStylesheet.ts` — CSS-specific item selection
- `extensions/emmet/src/evaluateMathExpression.ts` — Math expression evaluation
- `extensions/emmet/src/incrementDecrement.ts` — Number increment/decrement with configurable steps
- `extensions/emmet/src/reflectCssValue.ts` — Reflect CSS vendor prefixes

### Completion Provider
- `extensions/emmet/src/defaultCompletionProvider.ts` — Implements `vscode.CompletionItemProvider`; handles context detection, language filtering, abbreviation expansion preview

### Document/File Utilities
- `extensions/emmet/src/parseDocument.ts` — Caches parsed HTML/CSS ASTs; manages document lifecycle (open/close handlers)
- `extensions/emmet/src/util.ts` — Centralized utility module:
  - Language mode mappings (LANGUAGE_MODES: 13 supported languages with trigger characters)
  - Configuration access and migration (emmet.* settings)
  - Emmet helper lazy-loading (@vscode/emmet-helper)
  - File system operations (vscode.workspace.fs)
  - Workspace folder enumeration
- `extensions/emmet/src/bufferStream.ts` — Text buffer stream wrapper for parsing
- `extensions/emmet/src/locateFile.ts` — File resolution (likely for image size calculation)
- `extensions/emmet/src/imageSizeHelper.ts` — Image metadata extraction

### Special Image Handling
- `extensions/emmet/src/updateImageSize.ts` — Updates HTML image dimensions; registered separately in node entry point

---

## Tests

### Unit Tests (14 files)
- `extensions/emmet/src/test/abbreviationAction.test.ts` — HTML abbreviation expansion tests
- `extensions/emmet/src/test/cssAbbreviationAction.test.ts` — CSS abbreviation expansion tests
- `extensions/emmet/src/test/wrapWithAbbreviation.test.ts` — Wrapping abbreviation scenarios
- `extensions/emmet/src/test/tagActions.test.ts` — Remove, update, split/join tag operations
- `extensions/emmet/src/test/editPointSelectItemBalance.test.ts` — Edit points, item selection, balance
- `extensions/emmet/src/test/toggleComment.test.ts` — Comment toggle logic
- `extensions/emmet/src/test/incrementDecrement.test.ts` — Number increment/decrement edge cases
- `extensions/emmet/src/test/evaluateMathExpression.test.ts` — Expression evaluation
- `extensions/emmet/src/test/reflectCssValue.test.ts` — CSS vendor prefix reflection
- `extensions/emmet/src/test/updateImageSize.test.ts` — Image size detection
- `extensions/emmet/src/test/completion.test.ts` — Completion provider behavior
- `extensions/emmet/src/test/partialParsingStylesheet.test.ts` — CSS parsing edge cases
- `extensions/emmet/src/test/testUtils.ts` — Test helper functions
- `extensions/emmet/src/test/index.ts` — Test entry point

---

## Types / Interfaces

### Custom Type Definitions (5 files)
- `extensions/emmet/src/typings/EmmetNode.d.ts` — HTML AST node types from @emmetio/html-matcher
- `extensions/emmet/src/typings/EmmetFlatNode.d.ts` — Flat AST representations (Node, HtmlNode, Property, Rule, etc.)
- `extensions/emmet/src/typings/emmetio__css-parser.d.ts` — CSS parser type stubs
- `extensions/emmet/src/typings/emmetio__html-matcher.d.ts` — HTML matcher type stubs
- `extensions/emmet/src/typings/refs.d.ts` — Reference type declarations

### VS Code API Types
- `vscode.commands.registerCommand()` — Command registration
- `vscode.languages.registerCompletionItemProvider()` — Completion
- `vscode.languages.registerInlineCompletionItemProvider()` — Inline completions (1.92+)
- `vscode.TextEditor` — Active editor context (used in all command implementations)
- `vscode.TextDocument` — Document content/language ID
- `vscode.Position` / `vscode.Range` — Selection/cursor tracking
- `vscode.workspace` — Configuration, file system, workspace folders
- `vscode.window` — Error notifications

---

## Configuration

### Extension Manifest
- `extensions/emmet/package.json` — Main extension metadata:
  - Activation events: `onCommand:emmet.expandAbbreviation`, `onLanguage`
  - Contributes: 22 commands, configuration schema (13 settings), command palette menus
  - Dual entry points: `out/node/emmetNodeMain` (Node) and `dist/browser/emmetBrowserMain` (browser)
  - Dependencies: @vscode/emmet-helper, @emmetio/css-parser, @emmetio/html-matcher, image-size

### TypeScript Build Configuration
- `extensions/emmet/tsconfig.json` — Node.js target configuration
- `extensions/emmet/tsconfig.browser.json` — Browser/ES2020 target configuration

### Build Scripts
- `extensions/emmet/esbuild.mts` — Node.js build configuration
- `extensions/emmet/esbuild.browser.mts` — Browser bundle configuration

### Runtime Settings (extension manifest)
- `emmet.showExpandedAbbreviation` — "never" | "always" | "inMarkupAndStylesheetFilesOnly"
- `emmet.showAbbreviationSuggestions` — boolean
- `emmet.includeLanguages` — object (language mappings)
- `emmet.excludeLanguages` — array (default: ["markdown"])
- `emmet.variables` — object (lang, charset, custom)
- `emmet.syntaxProfiles` — object (output formatting)
- `emmet.extensionsPath` — array (custom snippets)
- `emmet.triggerExpansionOnTab` — boolean
- `emmet.useInlineCompletions` — boolean (toggles inline vs. standard completion)
- `emmet.preferences` — nested object (20+ CSS/formatting preferences)
- `emmet.showSuggestionsAsSnippets` — boolean
- `emmet.optimizeStylesheetParsing` — boolean

### Localization
- `extensions/emmet/package.nls.json` — English message strings for configuration properties and commands

### Other Configuration
- `extensions/emmet/cgmanifest.json` — Component governance manifest
- `extensions/emmet/.vscode/settings.json` — Development workspace settings
- `extensions/emmet/.vscode/launch.json` — Debug configuration
- `extensions/emmet/test-workspace/.vscode/settings.json` — Test workspace settings

---

## Documentation

- `extensions/emmet/README.md` — User-facing extension documentation
- `extensions/emmet/CONTRIBUTING.md` — Contribution guidelines for developers

---

## Notable Clusters

### Dual Environment Pattern (Browser + Node)
This extension demonstrates how VS Code extensions support both:
1. **Browser/Web** (`emmetBrowserMain.ts` → minimal entry point)
2. **Desktop/Node** (`emmetNodeMain.ts` → adds updateImageSize command)

Both converge on shared `emmetCommon.ts` activation. This is critical for Tauri porting because Tauri runs on desktop/system APIs (no browser limitations).

### Language Mode Configuration
The `LANGUAGE_MODES` constant in `util.ts` maps 13+ languages to completion trigger characters:
- Markup: html, jade, slim, haml, xml, xsl, javascriptreact, typescriptreact
- Stylesheet: css, scss, sass, less, stylus

Porting requires replacing vscode API calls with Rust equivalents for each language binding.

### Configuration Change Listeners
`emmetCommon.ts` registers workspace configuration watchers:
- `onDidChangeConfiguration()` → reload providers on emmet.includeLanguages or emmet.useInlineCompletions change
- `onDidSaveTextDocument()` → refresh extensions path on snippets.json save
- `onDidOpenTextDocument()` / `onDidCloseTextDocument()` → manage parse cache lifecycle

These patterns show tight coupling to VS Code's runtime extension API.

### Provider Registration Pattern
Two provider types registered for each language:
1. Standard `CompletionItemProvider` (always registered)
2. `InlineCompletionItemProvider` (optional, based on setting)

Both delegate to same `DefaultCompletionItemProvider` implementation, indicating pluggable architecture.

### Heavy Use of Third-Party Parsers
- `@emmetio/html-matcher` — HTML/XML parsing and AST generation
- `@emmetio/css-parser` — CSS parsing and AST generation
- `@vscode/emmet-helper` — High-level Emmet expansion API (lazy-loaded)
- `image-size` — Binary image format analysis

Porting would require Rust equivalents or FFI bindings to these libraries.

---

## Summary

The Emmet extension showcases VS Code's **command + provider model**:
- **18 commands** registered centrally with lazy-loaded implementations
- **2 provider types** (completion + inline completion) with conditional registration
- **Workspace configuration** management with live reload
- **Dual entry points** (browser/node) funneling to shared activation
- **Heavy parser dependency** on JavaScript libraries (html-matcher, css-parser)

**Key Porting Challenges**:
1. Command registration abstraction — requires defining Tauri command API equivalent
2. Provider pattern — InlineCompletionItemProvider (VS Code 1.92+) and CompletionItemProvider need Rust protocol implementations
3. Configuration watchers — workspace.onDidChangeConfiguration requires event system
4. Document lifecycle events — onDidOpenTextDocument, onDidCloseTextDocument need Tauri bindings
5. Parser dependencies — @emmetio libraries are JavaScript; need Rust ports or WASM interop
6. Workspace/file system API — vscode.workspace.fs, workspaceFolders need filesystem abstraction layer
7. UI feedback — vscode.window.showErrorMessage needs notification system in Tauri

The extension is a mid-complexity VS Code extension demonstrating **provider pattern scalability** and **multi-environment support**.

