# Partition 15: extensions/emmet/ — Porting Analysis for Tauri/Rust Migration

## Scope Summary
- **Size**: 46 files, 8,412 lines of TypeScript
- **Directory**: `extensions/emmet/`
- **Extension Type**: Bundled IDE feature (language editing with Emmet abbreviation expansion)

---

## Implementation Files

### Core Entry Points
- `extensions/emmet/src/node/emmetNodeMain.ts` — Node.js runtime activation; registers updateImageSize command; imports home dir via os module
- `extensions/emmet/src/browser/emmetBrowserMain.ts` — Browser runtime activation (simpler, no file system access)
- `extensions/emmet/src/emmetCommon.ts` — Central activation logic; registers 23 commands via `vscode.commands.registerCommand()`; sets up completion providers

### Core Editor Actions (23 command implementations)
- `extensions/emmet/src/abbreviationActions.ts` — Expand/wrap abbreviation; uses `vscode.window.activeTextEditor`, `showInputBox()`
- `extensions/emmet/src/balance.ts` — Balance in/out tag navigation
- `extensions/emmet/src/editPoint.ts` — Next/previous edit point navigation
- `extensions/emmet/src/matchTag.ts` — Match tag at cursor
- `extensions/emmet/src/removeTag.ts` — Remove tag structure
- `extensions/emmet/src/updateTag.ts` — Update tag; uses `showInputBox()` for tag name input
- `extensions/emmet/src/splitJoinTag.ts` — Split/join tags
- `extensions/emmet/src/mergeLines.ts` — Merge lines
- `extensions/emmet/src/toggleComment.ts` — Toggle comment
- `extensions/emmet/src/selectItem.ts` — Select next/previous item
- `extensions/emmet/src/selectItemHTML.ts` — HTML-specific select item logic
- `extensions/emmet/src/selectItemStylesheet.ts` — CSS-specific select item logic
- `extensions/emmet/src/evaluateMathExpression.ts` — Evaluate math expressions in CSS; uses external @emmetio/math-expression
- `extensions/emmet/src/incrementDecrement.ts` — Increment/decrement numbers
- `extensions/emmet/src/reflectCssValue.ts` — Reflect CSS property values
- `extensions/emmet/src/updateImageSize.ts` — Get and update image dimensions; uses image-size npm module; file I/O via `vscode.Uri`

### Completion Provider
- `extensions/emmet/src/defaultCompletionProvider.ts` — Implements `vscode.CompletionItemProvider` interface; language-sensitive abbreviation completions

### Document & Parsing
- `extensions/emmet/src/parseDocument.ts` — HTML/CSS AST parsing; uses @emmetio/html-matcher and @emmetio/css-parser
- `extensions/emmet/src/util.ts` — Shared utilities; workspace configuration read (`vscode.workspace.getConfiguration()`); environment variable lookup; file system operations
- `extensions/emmet/src/bufferStream.ts` — Text stream abstraction for parsing
- `extensions/emmet/src/imageSizeHelper.ts` — Image dimension detection via image-size library

### Configuration & Settings
- `extensions/emmet/src/locateFile.ts` — File system path resolution (extensions path configuration)

---

## Test Files (13 files)

- `extensions/emmet/src/test/index.ts` — Test suite entry point
- `extensions/emmet/src/test/testUtils.ts` — Shared test utilities; uses `vscode.workspace`, `vscode.window.showTextDocument()`
- `extensions/emmet/src/test/abbreviationAction.test.ts`
- `extensions/emmet/src/test/cssAbbreviationAction.test.ts`
- `extensions/emmet/src/test/completion.test.ts`
- `extensions/emmet/src/test/editPointSelectItemBalance.test.ts`
- `extensions/emmet/src/test/evaluateMathExpression.test.ts`
- `extensions/emmet/src/test/incrementDecrement.test.ts`
- `extensions/emmet/src/test/partialParsingStylesheet.test.ts`
- `extensions/emmet/src/test/reflectCssValue.test.ts`
- `extensions/emmet/src/test/tagActions.test.ts`
- `extensions/emmet/src/test/toggleComment.test.ts`
- `extensions/emmet/src/test/updateImageSize.test.ts`
- `extensions/emmet/src/test/wrapWithAbbreviation.test.ts`

---

## Type Definitions

- `extensions/emmet/src/typings/EmmetNode.d.ts` — Emmet AST node interface
- `extensions/emmet/src/typings/EmmetFlatNode.d.ts` — Flattened AST node structures
- `extensions/emmet/src/typings/emmetio__html-matcher.d.ts` — HTML parser module typings
- `extensions/emmet/src/typings/emmetio__css-parser.d.ts` — CSS parser module typings
- `extensions/emmet/src/typings/refs.d.ts` — Reference typings

---

## Configuration & Metadata

- `extensions/emmet/package.json` — Extension manifest; 24+ commands declared; 11 configuration properties for preferences; virtual workspace capable; untrusted workspace supported
- `extensions/emmet/tsconfig.json` — TypeScript configuration; extends base config
- `extensions/emmet/tsconfig.browser.json` — Separate browser build config
- `extensions/emmet/package.nls.json` — Localization strings
- `extensions/emmet/.npmrc` — NPM registry configuration
- `extensions/emmet/.vscodeignore` — Packaging exclusion rules
- `extensions/emmet/cgmanifest.json` — Third-party component manifest

---

## Documentation

- `extensions/emmet/README.md` — User-facing feature overview
- `extensions/emmet/CONTRIBUTING.md` — Contribution guidelines
- `extensions/emmet/.vscode/launch.json` — Debug configuration
- `extensions/emmet/.vscode/settings.json` — Extension dev settings

---

## Build Artifacts & Dependencies

- `extensions/emmet/esbuild.mts` — Main esbuild configuration
- `extensions/emmet/esbuild.browser.mts` — Browser-specific build config
- `extensions/emmet/package-lock.json` — Dependency lock file

---

## External Dependencies

### Direct NPM Dependencies (package.json)
1. **`@emmetio/css-parser`** (fork: ramya-rao-a/css-parser#vscode) — CSS abbreviation/formatting parsing
2. **`@emmetio/html-matcher`** (^0.3.3) — HTML tag matching/navigation
3. **`@emmetio/math-expression`** (^1.0.5) — CSS math expression evaluation
4. **`@vscode/emmet-helper`** (^2.8.8) — Core Emmet abbreviation expansion (maintained by Microsoft)
5. **`image-size`** (~1.0.0) — Read image dimensions from file
6. **`vscode-languageserver-textdocument`** (^1.0.1) — LSP TextDocument compatibility layer

### Built-in Node Modules (used in Node entry point)
- `os.homedir()` — Determine user home directory

---

## Key VS Code API Usage

### Commands System
- **23 public commands registered** via `vscode.commands.registerCommand()`
- All commands activated via `onCommand` and `onLanguage` event triggers
- Activation events: `onCommand:emmet.expandAbbreviation` + `onLanguage` (lazy-loaded)

### Editor/Window APIs (heavily used)
- `vscode.window.activeTextEditor` — Access active editor (all action files)
- `vscode.window.showInputBox()` — Interactive tag name input (updateTag, abbreviationActions)
- `vscode.window.showInformationMessage()` — Status messages
- `vscode.window.showErrorMessage()` — Error notifications
- `vscode.window.showTextDocument()` — Test utilities

### Language/Completion Provider
- `vscode.languages.registerCompletionItemProvider()` — Register completion suggestions
- `vscode.languages.registerInlineCompletionItemProvider()` — Inline completion (new)
- `vscode.CompletionItemProvider` interface implementation

### Workspace Configuration
- `vscode.workspace.getConfiguration('emmet')` — Read 11+ configuration properties
- `vscode.workspace.getConfiguration().update()` — Persist settings changes
- `vscode.workspace.workspaceFolders` — Multi-root workspace support
- `vscode.workspace.fs` — FileSystemProvider access for extension paths

### Document & TextEditor
- `vscode.TextDocument` — Document abstraction
- `vscode.Position`, `vscode.Range`, `vscode.Selection` — Cursor/selection manipulation
- `vscode.TextEdit` — Buffer mutations

### Miscellaneous
- `vscode.Uri.file()` — File path abstraction (used in node entry point for home dir)
- Extension context subscriptions model

---

## Critical Browser vs. Node Split

### Node-only Features
- `emmetNodeMain.ts` additionally imports `os.homedir()` for file system context
- Image size detection via `image-size` library (file I/O) — node only
- Extensions path resolution from workspace filesystem

### Browser-capable Features
- All abbreviation expansion/parsing logic works browser-side
- Completion provider works in browser
- Tag actions work in browser context

---

## Architectural Summary for Tauri/Rust Porting

**Extension Characteristics:**
- **Purely functional**: No background processes or stateful services
- **Command-driven**: 23 lightweight command handlers
- **Minimal external coupling**: Depends on 6 npm packages; 5 are Emmet-specific abbreviation/parsing libraries
- **Light window/message usage**: ~15 showInputBox/message calls across entire codebase
- **Tight integration**: Completion provider model ties to VS Code's language provider registration

**Porting Impact: LOW**

**Key Considerations:**
1. **Emmet core libraries** (@emmetio/*) would need Rust equivalents OR FFI bindings to Node.js modules
2. **@vscode/emmet-helper** is Microsoft's maintained abbreviation engine — seek parity implementation
3. **Completion provider registration model** (languages.registerCompletionItemProvider) relies on VS Code's extension API contract
4. **File I/O** (image-size) is minimal and localized to single command
5. **Configuration schema** is declarative and portable to Tauri configuration layer

**Candidate Rust Rewrites:**
- Parsing logic (HTML/CSS AST) — can be pure Rust
- Abbreviation expansion — algorithmic, data-driven (feasible in Rust)
- Image dimension reading — use rust image crate
- All action implementations — straightforward logic, no platform-specific code
- Completion provider — bridge via Tauri command system

**Simplification Opportunity:**
This extension is an ideal candidate for early Tauri porting due to:
- Self-contained feature domain
- Minimal cross-extension dependencies
- Mostly algorithmic (no event-driven complexity)
- No background workers or persistent state
- Testable action functions
