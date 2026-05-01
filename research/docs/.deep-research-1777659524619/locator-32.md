# File Location Index: merge-conflict Extension

## Implementation
- `extensions/merge-conflict/src/mergeConflictMain.ts` — Extension activation entry point; registers MergeConflictServices
- `extensions/merge-conflict/src/services.ts` — Core service wrapper orchestrating all subcomponents (DocumentTracker, CodeLensProvider, CommandHandler, ContentProvider, Decorator)
- `extensions/merge-conflict/src/codelensProvider.ts` — Implements vscode.CodeLensProvider interface; registers code lenses via vscode.languages.registerCodeLensProvider with multi-scheme support (file, vscode-vfs, untitled, vscode-userdata); provides inline commands for conflict resolution
- `extensions/merge-conflict/src/mergeDecorator.ts` — Implements vscode.Disposable; decorates merge conflict regions using vscode.TextEditorDecorationType; manages lifecycle of decorations across editor visibility changes
- `extensions/merge-conflict/src/commandHandler.ts` — Implements vscode.Disposable; registers text editor commands for conflict acceptance (current/incoming/both) and navigation (next/previous)
- `extensions/merge-conflict/src/contentProvider.ts` — Content provider for diff views; handles side-by-side comparison UI
- `extensions/merge-conflict/src/documentMergeConflict.ts` — Represents individual merge conflict descriptors with commitEdit and applyEdit methods
- `extensions/merge-conflict/src/mergeConflictParser.ts` — Parses merge conflict markers (<<<<<<<, =======, >>>>>>>) from text documents; implements conflict region scanning
- `extensions/merge-conflict/src/documentTracker.ts` — Implements IDocumentMergeConflictTrackerService; caches conflict detection with Delayer for debounced scanning across multiple origins
- `extensions/merge-conflict/src/delayer.ts` — Generic delay/debounce utility for async operations
- `extensions/merge-conflict/src/interfaces.ts` — TypeScript interfaces defining extension contracts (IExtensionConfiguration, IDocumentMergeConflict, IMergeRegion, CommitType enum)

## Configuration
- `extensions/merge-conflict/package.json` — Extension manifest; defines command contributions, menu integrations, configuration schema (codeLens.enabled, decorators.enabled, autoNavigateNextConflict.enabled, diffViewPosition); registers on onStartupFinished activation event; supports virtualWorkspaces and untrustedWorkspaces; dependencies on @vscode/extension-telemetry
- `extensions/merge-conflict/tsconfig.json` — TypeScript compiler configuration extending base config; targets Node types; includes vscode.d.ts type definitions
- `extensions/merge-conflict/tsconfig.browser.json` — Browser-specific TypeScript config for bundled web variant
- `extensions/merge-conflict/.vscodeignore` — Files excluded from extension packaging

## Build / Bundling
- `extensions/merge-conflict/esbuild.mts` — Node.js bundler configuration for desktop build
- `extensions/merge-conflict/esbuild.browser.mts` — Browser bundler configuration for web variant
- `extensions/merge-conflict/package-lock.json` — Dependency lock file

## Documentation
- `extensions/merge-conflict/README.md` — User-facing documentation noting bundling status and referencing official VS Code merge conflict guide
- `extensions/merge-conflict/package.nls.json` — Localization strings for UI labels (command titles, configuration descriptions)

## Notable Clusters
- `extensions/merge-conflict/src/` — 11 TypeScript implementation files focusing on merge conflict detection, UI rendering via CodeLens/decorations, command routing, and conflict resolution patterns
- Relevant for porting: CodeLens provider pattern using vscode.languages.registerCodeLensProvider; text document decoration system; command registration and execution; configuration/context setting via vscode.commands.executeCommand('setContext', ...); event subscription patterns (onDidOpenTextDocument, onDidChangeTextDocument, onDidChangeVisibleTextEditors)

## Extension Integration Points
The merge-conflict extension demonstrates several VS Code IDE features central to porting research:
1. **Language Intelligence** — CodeLensProvider for in-editor actionable hints
2. **Text Document Decorations** — Colored gutter markers and line decorations for visual conflict indicators
3. **Command Palette Integration** — Extensible command registration with context-aware enablement
4. **Configuration System** — Settings schema with runtime updates via configurationUpdated lifecycle
5. **Source Control UI** — Hooks into scm/resourceState/context menus for git integration
6. **Multi-scheme Document Support** — Handles file, vscode-vfs (virtual), untitled, and vscode-userdata schemes
7. **Editor Lifecycle Management** — Tracks editor visibility, text changes, and document state

---

## Summary

The merge-conflict extension (13 files, ~1,463 LOC) is a complete source control feature demonstrating VS Code's extensibility model. It showcases CodeLens provider registration (the partition seed), text-document decoration patterns, command-based UI, and multi-editor state management—all foundational patterns for porting core IDE functionality to Tauri/Rust. The extension's dependency on vscode.languages, vscode.commands, vscode.window, and vscode.workspace APIs represents core extension API contracts that would require equivalent implementations in a Rust-based IDE.
