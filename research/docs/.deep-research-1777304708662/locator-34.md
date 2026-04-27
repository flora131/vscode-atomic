# File Locations for Extension Editing Extension

## Overview
The `extension-editing` extension provides IDE support for extension developers authoring VS Code extensions. It includes schema validation for package.json, completion providers, code actions, and localization support. This extension demonstrates several core VS Code API patterns relevant to understanding how IDE features are implemented.

## Implementation Files

- `extensions/extension-editing/src/extensionEditingMain.ts` - Main entry point for Node.js platform; registers completion item provider and code actions provider for package.json files
- `extensions/extension-editing/src/extensionEditingBrowserMain.ts` - Browser/web platform variant; stripped down version that registers only completion items and L10n support (no code actions)
- `extensions/extension-editing/src/packageDocumentHelper.ts` - Core helper class for package.json document analysis; implements completion item and code action providers for language overrides and activation events
- `extensions/extension-editing/src/packageDocumentL10nSupport.ts` - Implements definition and reference providers for NLS localization mappings between package.json and package.nls.json files
- `extensions/extension-editing/src/extensionLinter.ts` - Linting engine that validates package.json and README/CHANGELOG.md files; manages diagnostics collection, file watching, and validation queue
- `extensions/extension-editing/src/extensionEngineValidation.ts` - Version parsing and validation utilities; defines IParsedVersion and INormalizedVersion interfaces for engine version constraints
- `extensions/extension-editing/src/jsonReconstruct.ts` - JsonStringScanner utility class for converting offsets within decoded JSON strings to offsets in encoded strings; handles escape sequence processing

## Configuration Files

- `extensions/extension-editing/package.json` - Extension manifest defining contribution points (jsonValidation schemas for package.json, language-configuration.json, icon-theme.json, color-theme.json); activation events (onLanguage:json, onLanguage:markdown); dependencies (jsonc-parser, markdown-it, parse5)
- `extensions/extension-editing/package.nls.json` - Localization strings for display name and description
- `extensions/extension-editing/tsconfig.json` - TypeScript compiler configuration extending ../tsconfig.base.json; compiles src/ to out/ directory
- `extensions/extension-editing/tsconfig.browser.json` - Browser-specific TypeScript configuration extending tsconfig.json; excludes tests and compiles only extensionEditingBrowserMain.ts
- `extensions/extension-editing/esbuild.mts` - esbuild configuration for Node.js platform build; entry point: extensionEditingMain.ts
- `extensions/extension-editing/esbuild.browser.mts` - esbuild configuration for browser platform build; entry point: extensionEditingBrowserMain.ts; outputs to dist/browser/
- `extensions/extension-editing/.npmrc` - NPM configuration file
- `extensions/extension-editing/.vscodeignore` - Excludes files from packaged extension

## Type Definitions / Interfaces

Notable type definitions exported by the extension:

**From extensionEngineValidation.ts:**
- `IParsedVersion` - Interface representing parsed version string components (hasCaret, hasGreaterEquals, majorBase/minorBase/patchBase, mustEqual flags, preRelease)
- `INormalizedVersion` - Interface representing normalized version constraints (majorBase/minorBase/patchBase, mustEqual flags, notBefore timestamp, isMinimum flag)

**From packageDocumentL10nSupport.ts:**
- Class implements `vscode.DefinitionProvider` and `vscode.ReferenceProvider` interfaces
- Uses `vscode.DocumentSelector` type for matching package.json and package.nls.json

**From extensionLinter.ts:**
- `PackageJsonInfo` - Internal interface tracking extension folder state (isExtension, hasHttpsRepository, implicitActivationEvents, engineVersion)
- `TokenAndPosition` - Internal interface for markdown token position tracking
- Uses `vscode.DiagnosticCollection`, `vscode.Diagnostic` types

## Documentation

- `extensions/extension-editing/images/icon.png` - Visual icon for the extension

## Notable Clusters and Architecture Patterns

**Language Provider Registration Pattern:**
The extension demonstrates the standard VS Code pattern for registering language service providers:
- `vscode.languages.registerCompletionItemProvider()` - Registers completion items for package.json fields
- `vscode.languages.registerCodeActionsProvider()` - Registers code actions for diagnostic remediation
- `vscode.languages.registerDefinitionProvider()` - Maps NLS references to their definitions
- `vscode.languages.registerReferenceProvider()` - Finds all references to NLS keys
- `vscode.languages.createDiagnosticCollection()` - Creates diagnostic collection for linting
- `vscode.languages.getLanguages()` - Retrieves available language identifiers for completion

All provider registrations use document selector patterns: `{ language: 'json', pattern: '**/package.json' }` and `{ language: 'markdown', pattern: '**.md' }`

**Dual Platform Support:**
- Node.js variant (extensionEditingMain.ts) includes full linting, validation, code actions
- Browser variant (extensionEditingBrowserMain.ts) includes only completion and NLS support
- Separate esbuild configurations and tsconfig files manage platform-specific builds
- Both platforms activate on onLanguage:json and onLanguage:markdown events

**File Watching and Queueing:**
The ExtensionLinter implements a deferred validation pattern:
- Watches package.json files via `workspace.createFileSystemWatcher()`
- Queues documents for processing on open/change/delete events
- Uses timer-based batching to avoid excessive validation passes
- Maintains separate queues for package.json and markdown documents

**Dependency Management:**
- `jsonc-parser` - Parses JSON with comments and provides location/node utilities
- `markdown-it` - Parses and validates markdown (README/CHANGELOG)
- `parse5` - HTML parser for extracting/validating image URLs in markdown

## Entry Points

- `extensions/extension-editing/src/extensionEditingMain.ts:11` - activate() function called on extension load; registers core providers
- `extensions/extension-editing/src/extensionEditingBrowserMain.ts:10` - activate() function for browser platform
- `extensions/extension-editing/package.json:16,17` - Main entry points defined: "./out/extensionEditingMain" (Node.js) and "./dist/browser/extensionEditingBrowserMain" (browser)
- `extensions/extension-editing/package.json:12,13` - Activation events: onLanguage:json and onLanguage:markdown

---

## Research Context: Tauri/Rust Porting Implications

This extension provides critical IDE capabilities for extension authoring workflows. Porting these features to Tauri/Rust would require:

1. **Language Service Server Protocol (LSP) Integration** - The completion, hover, definition, and reference providers use VS Code's native language extension API. A Tauri/Rust implementation would need to establish LSP-compliant servers or equivalent protocol handlers.

2. **JSON Schema Validation Subsystem** - The jsonValidation contribution points validate against vscode://schemas/* URLs. Tauri would need equivalent schema resolution and validation machinery, likely via JSON Schema libraries in Rust (e.g., jsonschema crate).

3. **Document Synchronization** - The file watcher and document queue system synchronizes state across textDocument events. Tauri would need equivalent document lifecycle management integrated with any LSP server implementation.

4. **Diagnostic Collection and Presentation** - The DiagnosticCollection API pushes diagnostics to the editor. Tauri would need a similar mechanism to communicate validation errors back to the UI layer.

5. **Platform-Specific Builds** - The dual Node.js/Browser builds reveal VS Code's architecture challenge. A Tauri port would likely be desktop-only but would need equivalent capability detection and feature gating.

6. **Localization Infrastructure** - The package.nls.json integration and L10n support requires tight coupling with VS Code's localization system. Tauri would need equivalent i18n infrastructure for extension catalog presentation.

The extension serves as a lens into VS Code's extension authoring story—a critical competitive advantage for IDE adoption and ecosystem health. Porting this single extension touches nearly all major subsystems: document parsing, schema validation, language services, file I/O, diagnostic presentation, and localization.
