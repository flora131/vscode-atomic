# Emmet Extension - VS Code API Consumption Analysis

## Implementation

- `extensions/emmet/src/defaultCompletionProvider.ts` — Implements `CompletionItemProvider` interface, core language intelligence for Emmet abbreviation completion using vscode.languages.registerCompletionItemProvider()
- `extensions/emmet/src/emmetCommon.ts` — Extension activation entry point, registers 20+ commands via vscode.commands.registerCommand(), inline completion providers, and configuration change listeners
- `extensions/emmet/src/abbreviationActions.ts` — Implements abbreviation expansion and wrap functionality using editor.edit(), selection manipulation, TextEdit operations
- `extensions/emmet/src/updateTag.ts` — Tag renaming via vscode.window.showInputBox() input dialog and editor.edit() for text modifications
- `extensions/emmet/src/removeTag.ts` — Tag removal operations using TextEdit and Range manipulation via editor.edit()
- `extensions/emmet/src/matchTag.ts` — Tag matching navigation using selection and position management
- `extensions/emmet/src/balance.ts` — Balance in/out navigation maintaining selection state and stack tracking
- `extensions/emmet/src/splitJoinTag.ts` — Tag splitting and joining operations
- `extensions/emmet/src/toggleComment.ts` — HTML/CSS comment toggling via document editing
- `extensions/emmet/src/editPoint.ts` — Navigation to next/previous edit points in markup
- `extensions/emmet/src/selectItem.ts` — Item selection for HTML and stylesheet content (delegates to selectItemHTML, selectItemStylesheet)
- `extensions/emmet/src/selectItemHTML.ts` — HTML-specific item selection logic
- `extensions/emmet/src/selectItemStylesheet.ts` — Stylesheet-specific item selection logic
- `extensions/emmet/src/incrementDecrement.ts` — Number increment/decrement operations on numeric values
- `extensions/emmet/src/evaluateMathExpression.ts` — Math expression evaluation and insertion
- `extensions/emmet/src/reflectCssValue.ts` — CSS vendor prefix value reflection
- `extensions/emmet/src/updateImageSize.ts` — Image dimension detection and insertion via vscode.Uri operations
- `extensions/emmet/src/mergeLines.ts` — Multi-line content merging
- `extensions/emmet/src/util.ts` — Shared utilities including workspace configuration access (vscode.workspace.getConfiguration), file system operations, language mode detection
- `extensions/emmet/src/parseDocument.ts` — Document parsing and caching via workspace event listeners (onDidOpenTextDocument, onDidCloseTextDocument)
- `extensions/emmet/src/bufferStream.ts` — Buffer stream abstraction for document text access
- `extensions/emmet/src/node/emmetNodeMain.ts` — Node.js environment entry point calling activateEmmetExtension()
- `extensions/emmet/src/browser/emmetBrowserMain.ts` — Browser environment entry point calling activateEmmetExtension()

## Tests

- `extensions/emmet/src/test/completion.test.ts` — Completion provider test suite using CancellationTokenSource, CompletionTriggerKind, Selection APIs
- `extensions/emmet/src/test/abbreviationAction.test.ts` — Abbreviation expansion test coverage
- `extensions/emmet/src/test/cssAbbreviationAction.test.ts` — CSS abbreviation-specific tests
- `extensions/emmet/src/test/wrapWithAbbreviation.test.ts` — Wrap operation test scenarios
- `extensions/emmet/src/test/tagActions.test.ts` — Tag manipulation (update, remove, match) test suite
- `extensions/emmet/src/test/toggleComment.test.ts` — Comment toggle test scenarios
- `extensions/emmet/src/test/editPointSelectItemBalance.test.ts` — Edit point, selection, and balance operation tests
- `extensions/emmet/src/test/incrementDecrement.test.ts` — Number increment/decrement test coverage
- `extensions/emmet/src/test/evaluateMathExpression.test.ts` — Math expression evaluation tests
- `extensions/emmet/src/test/reflectCssValue.test.ts` — CSS value reflection tests
- `extensions/emmet/src/test/updateImageSize.test.ts` — Image size detection tests
- `extensions/emmet/src/test/partialParsingStylesheet.test.ts` — Stylesheet partial parsing optimization tests
- `extensions/emmet/src/test/testUtils.ts` — Test infrastructure with withRandomFileEditor, closeAllEditors utilities for editor state management

## Types / Interfaces

- `extensions/emmet/src/typings/EmmetFlatNode.d.ts` — Custom Node, HtmlNode, HtmlToken, CssToken, Attribute, Rule, Property, Stylesheet type definitions for Emmet AST structures
- `extensions/emmet/src/typings/emmetio__html-matcher.d.ts` — Type definitions for @emmetio/html-matcher third-party library
- `extensions/emmet/src/typings/emmetio__css-parser.d.ts` — Type definitions for @emmetio/css-parser third-party library
- `extensions/emmet/src/typings/refs.d.ts` — Global type references and module augmentations

## Configuration

- `extensions/emmet/package.json` — Extension manifest with 22 Emmet commands (wrapWithAbbreviation, removeTag, updateTag, matchTag, balanceIn/Out, splitJoinTag, toggleComment, increment/decrement operations, etc.), configuration schema for emmet.* settings (showExpandedAbbreviation, excludeLanguages, includeLanguages, extensionsPath, preferences, variables, syntaxProfiles, etc.), activation triggers, browser and Node entry points
- `extensions/emmet/.npmrc` — NPM configuration
- `extensions/emmet/.vscode/settings.json` — Development workspace settings
- `extensions/emmet/.vscode/launch.json` — Launch configuration for debugging
- `extensions/emmet/tsconfig.json` — TypeScript configuration for Node target
- `extensions/emmet/tsconfig.browser.json` — TypeScript configuration for browser target
- `extensions/emmet/esbuild.mts` — Build configuration for Node distribution
- `extensions/emmet/esbuild.browser.mts` — Build configuration for browser distribution

## Documentation

- `extensions/emmet/README.md` — Extension overview and feature documentation
- `extensions/emmet/CONTRIBUTING.md` — Contribution guidelines
- `extensions/emmet/package.nls.json` — Localized strings for commands and configuration

## Notable Clusters

- `extensions/emmet/src/` — 22 action/utility TypeScript implementation files providing completion, expansion, navigation, editing, and transformation commands integrated with VS Code's editor APIs
- `extensions/emmet/src/test/` — 13 test files covering all major features using Mocha test framework and VS Code testing utilities
- `extensions/emmet/src/typings/` — 4 custom TypeScript type definition files for Emmet AST structures and third-party library support

## Key API Surface

This extension demonstrates consumption of multiple VS Code core IDE APIs:
- **Language Services**: `languages.registerCompletionItemProvider()`, `languages.registerInlineCompletionItemProvider()`
- **Editor Operations**: `editor.edit()`, position/range manipulation, selection management, text document access
- **Commands**: 22+ registered commands for edit actions, navigation, and transformations
- **Configuration**: Workspace settings access with multiple emmet.* configuration keys
- **UI**: Input dialogs via `window.showInputBox()`, command palette integration
- **Workspace Events**: File open/close tracking, configuration change listeners, document parsing lifecycle
- **File System**: Uri operations for resource access (image size detection)

