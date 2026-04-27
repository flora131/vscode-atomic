# File Locator Index: extensions/merge-conflict/

## Implementation

- `extensions/merge-conflict/src/mergeConflictMain.ts` — Extension entry point with activate/deactivate; demonstrates vscode.ExtensionContext usage
- `extensions/merge-conflict/src/services.ts` — Service orchestrator (vscode.Disposable) that wires together all components and manages configuration
- `extensions/merge-conflict/src/codelensProvider.ts` — CodeLensProvider implementation using vscode.languages.registerCodeLensProvider() and vscode.CodeLens API
- `extensions/merge-conflict/src/commandHandler.ts` — Command registration and execution for merge conflict actions
- `extensions/merge-conflict/src/contentProvider.ts` — Content provider for diff view functionality
- `extensions/merge-conflict/src/mergeDecorator.ts` — Decoration/gutter rendering for merge conflict regions
- `extensions/merge-conflict/src/documentTracker.ts` — Document lifecycle and change tracking
- `extensions/merge-conflict/src/documentMergeConflict.ts` — Merge conflict data model and editing operations
- `extensions/merge-conflict/src/mergeConflictParser.ts` — Parser for detecting and extracting merge conflict regions
- `extensions/merge-conflict/src/delayer.ts` — Debouncing/delay utility

## Types / Interfaces

- `extensions/merge-conflict/src/interfaces.ts` — Defines IMergeRegion, IDocumentMergeConflict, IDocumentMergeConflictTracker, IExtensionConfiguration with vscode.Range/TextEditor references

## Configuration

- `extensions/merge-conflict/package.json` — Extension manifest with vscode engine dependency, contribution points (commands, menus, configuration), telemetry, and dual build targets (node + web)
- `extensions/merge-conflict/tsconfig.json` — TypeScript configuration extending base config, includes vscode.d.ts type definitions
- `extensions/merge-conflict/tsconfig.browser.json` — Browser variant TypeScript config
- `extensions/merge-conflict/package.nls.json` — Localization strings (i18n) for UI text
- `extensions/merge-conflict/esbuild.mts` — Build configuration for Node.js target
- `extensions/merge-conflict/esbuild.browser.mts` — Build configuration for web/browser target
- `extensions/merge-conflict/.npmrc` — npm configuration
- `extensions/merge-conflict/.vscodeignore` — Vscode packaging exclusions

## Documentation

- `extensions/merge-conflict/README.md` — Extension overview noting it is bundled with VS Code

## Examples / Fixtures

- `extensions/merge-conflict/media/icon.png` — Extension icon asset

## Notable Clusters

- `extensions/merge-conflict/src/` — 10 TypeScript files implementing merge conflict detection, CodeLens provider registration, command handlers, decorators, and diff viewing; demonstrates vscode extension API patterns for language intelligence (CodeLensProvider), UI decorations, and editor commands
