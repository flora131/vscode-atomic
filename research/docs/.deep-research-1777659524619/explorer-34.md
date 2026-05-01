# Partition 34 of 79 — Findings

## Scope
`extensions/extension-editing/` (10 files, 1,326 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Extension Manifest Validation Layer — Tauri/Rust Porting Implications

## Summary

The `extensions/extension-editing/` extension provides JSON schema validation, linting, and language services for VS Code extension manifests (`package.json`, `package.nls.json`). Porting this to Tauri/Rust requires decomposing five core subsystems: (1) completion item provider for language-specific editor overrides, (2) JSON schema validation engine tied to product-defined schemas, (3) extension linter with manifest-to-activation-event mapping, (4) NLS reference tracking for l10n metadata, and (5) JSON string reconstruction for escape sequence handling.

### Implementation

- `src/extensionEditingMain.ts` — Desktop entry point; registers completion provider and code actions provider for `**/package.json` files
- `src/extensionEditingBrowserMain.ts` — Web/browser entry point; registers completion provider only (no code actions in browser context)
- `src/packageDocumentHelper.ts` — Core completion logic for `configurationDefaults` language overrides; uses `jsonc-parser` to locate position in JSON tree
- `src/extensionLinter.ts` — Large diagnostic aggregator (~486 lines) that lints package.json metadata (icon URLs, badge URLs, repository HTTPS requirements), validates activation events against implicit events, validates API proposals against product allowlist, and lints when-clauses in menu/view/command contributions
- `src/extensionEngineValidation.ts` — Version parser for `engines.vscode` field; supports semver with caret/greater-equals operators, prerelease suffixes, and not-before date constraints
- `src/packageDocumentL10nSupport.ts` — Definition/reference provider for NLS string lookups; resolves `%key%` references in package.json to declarations in package.nls.json
- `src/jsonReconstruct.ts` — Specialized lexer that converts offsets in decoded JSON strings to offsets in encoded form (handles escape sequences, unicode escapes)
- `src/constants.ts` — Localized error/warning messages for implicit activation events

### Tests

No test files found in scope. Extension editing functionality is likely tested in the main VS Code test suite.

### Types / Interfaces

- `IParsedVersion` (extensionEngineValidation.ts) — Semantic version parse result with flags for caret, greater-equals, must-equal constraints
- `INormalizedVersion` (extensionEngineValidation.ts) — Normalized engine version with notBefore timestamp and isMinimum flag
- `PackageJsonInfo` (extensionLinter.ts) — Metadata cache: isExtension boolean, hasHttpsRepository, repository URI, implicit activation events set, normalized engine version
- `TokenAndPosition` (extensionLinter.ts) — Markdown token with begin/end offsets for image validation

### Configuration

- `package.json` — Manifest with `contributes.jsonValidation` entries mapping:
  - `package.json` → `vscode://schemas/vscode-extensions`
  - `*language-configuration.json` → `vscode://schemas/language-configuration`
  - `*icon-theme.json` → `vscode://schemas/icon-theme`
  - `*color-theme.json` → `vscode://schemas/color-theme`
  - Activates on `onLanguage:json` and `onLanguage:markdown` events
- `tsconfig.json` — Node.js build, extends base, compiles src/ → out/
- `tsconfig.browser.json` — Browser build, excludes test/, explicitly includes extensionEditingBrowserMain.ts
- `esbuild.mts` — Entry point: `extensionEditingMain.ts` (Node.js platform)
- `esbuild.browser.mts` — Entry point: `extensionEditingBrowserMain.ts` (browser platform)
- `.vscodeignore` — Excludes build artifacts and non-essential files from packaged extension
- `package.nls.json` — Localization strings for display name and description

### Examples / Fixtures

- `package.json` declares badge provider allowlist (`product.extensionAllowedBadgeProviders`, `product.extensionAllowedBadgeProvidersRegex`) loaded at linter initialization
- `product.json` from `env.appRoot` read at linter construction time to populate allowed badge providers and API proposal lists

### Documentation

- No markdown documentation in scope
- Inline comments reference upstream VS Code validator: `extensionValidator.ts` in platform/extensions/common

### Notable Clusters

- `src/` (7 files, ~1,522 LOC) — Complete extension editing subsystem: completion/linting logic, schema validation, NLS tracking, version parsing, JSON manipulation

---

## Porting Implications

### 1. Completion Provider Architecture

Current flow: VS Code language server registration (`registerCompletionItemProvider`) → `PackageDocument.provideCompletionItems()` → `jsonc-parser.getLocation()` to locate cursor → path inspection → snippet generation.

**Porting challenge**: Tauri/Rust lacks a built-in completion provider registration API equivalent to `vscode.languages.registerCompletionItemProvider`. Must implement:
- JSON AST parser (use `serde_json` + tree navigation)
- Position-to-token mapping (convert LSP line:column → byte offset)
- Snippet expansion engine (format VSCode snippets into LSP snippets)

### 2. JSON Schema Validation

Current: `package.json` contributes `jsonValidation` entries pointing to `vscode://schemas/*` URLs. VS Code's schema service resolves these internally.

**Porting challenge**: Tauri editor must ship or lazy-load JSON schemas for extension manifests. Consider:
- Embed schema files in binary or load from external store
- Implement schema validator (or use existing library like `jsonschema` crate)
- Map schema URIs to schema definitions at startup

### 3. Extension Linter Subsystem

The linter performs ~10 distinct validations on package.json:
- Icon/badge URL validation (HTTPS, trusted SVG sources, no data URLs)
- Activation event validation (implicit vs. explicit, redundant events, reserved prefixes)
- API proposal validation (cross-reference with `product.extensionEnabledApiProposals`)
- When-clause parsing (delegates to `_validateWhenClauses` command)
- Markdown README validation (image URL linting, embedded SVG detection)

**Porting challenge**: 
- **Product config dependency**: Badge provider and API proposal lists are loaded from `product.json` at init. Tauri must expose a similar product-config interface.
- **When-clause validator**: Currently implemented in core VS Code (`_validateWhenClauses` command). Tauri must either call back to VS Code or implement clause parser in Rust.
- **Async linting**: Current linter uses `Promise.all()` and async file I/O; Tauri equivalent uses `async`/`await` Rust patterns.

### 4. NLS (Localization) Tracking

Current: `PackageDocumentL10nSupport` provides definition/reference navigation for `%key%` patterns in package.json → package.nls.json.

**Porting challenge**:
- Must integrate with Tauri's file watching (watch `**/package.nls.json`)
- Implement definition/reference providers for LSP
- Cache parsed NLS trees per workspace folder

### 5. Version Parsing

The `extensionEngineValidation.ts` module parses semver strings with custom operators (`^`, `>=`) and prerelease date constraints. This is relatively isolated and **low-risk to port** — can be a pure Rust library with regex matching (or use existing `semver` crate with custom preprocessing).

### 6. JSON String Reconstruction

`jsonReconstruct.ts` solves a specific problem: mapping offsets in decoded strings to encoded positions (handling `\\`, `\"`, `\u####` escapes). This is **highly specialized** and needed for when-clause error reporting. In Tauri/Rust:
- Use `serde_json::to_string` for encoding positions, or
- Implement a simple state machine similar to the TypeScript version

---

## Key Dependencies & Removals

**Current (TypeScript)**:
- `vscode` API (completion providers, code actions, language services)
- `jsonc-parser` (JSON-with-comments parsing)
- `markdown-it` (Markdown tokenization for README linting)
- `parse5` (HTML/SVG parser for embedded SVG detection)
- `url` stdlib (URI parsing)

**Tauri/Rust equivalents**:
- LSP client/server framework (e.g., `tower-lsp`)
- `serde_json` or `jsonc` crate for JSON parsing
- `markdown` crate or similar for tokenization
- `html5ever` or `scraper` for HTML/SVG parsing
- `url` crate for URI handling

---

## Summary of Porting Scope

A Tauri port requires implementing **5 feature modules**:
1. **Completion Provider** — Snippet generation for language overrides
2. **Schema Validator** — Load and apply extension manifest JSON schemas
3. **Linter** — Manifest validation with product config integration
4. **NLS Tracker** — Definition/reference navigation for localization keys
5. **Helpers** — Version parsing, JSON reconstruction, when-clause integration

**Estimated effort**: Moderate-to-high. The completion and linting logic is moderately complex, with significant product-config coupling. The when-clause validator introduces a dependency on core VS Code functionality or requires implementing a clause parser from scratch.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Research: VS Code Extension Editing - package.json Validation & Completion

## Research Question
Patterns for implementing language servers and validation systems for extension manifests (`package.json`), as found in the extension-editing extension.

## Scope
`extensions/extension-editing/` — 8 TypeScript files focusing on package.json validation, completion, and linting.

---

## Patterns Found

#### Pattern: Completion Item Provider with DocumentSelector Pattern
**Where:** `extensions/extension-editing/src/extensionEditingMain.ts:25-31`
**What:** Registers a completion provider for package.json files using language and glob pattern matching.
```typescript
function registerPackageDocumentCompletions(): vscode.Disposable {
	return vscode.languages.registerCompletionItemProvider({ language: 'json', pattern: '**/package.json' }, {
		provideCompletionItems(document, position, token) {
			return new PackageDocument(document).provideCompletionItems(position, token);
		}
	});
}
```
**Variations / call-sites:**
- `extensions/extension-editing/src/extensionEditingBrowserMain.ts:18-24` — identical browser variant
- `extensions/extension-editing/src/packageDocumentL10nSupport.ts:10` — DocumentSelector pattern reused for definition/reference providers

---

#### Pattern: Code Actions Provider for Diagnostic Fixes
**Where:** `extensions/extension-editing/src/extensionEditingMain.ts:33-39`
**What:** Registers a code actions provider targeting the same package.json document selector to fix linting diagnostics.
```typescript
function registerCodeActionsProvider(): vscode.Disposable {
	return vscode.languages.registerCodeActionsProvider({ language: 'json', pattern: '**/package.json' }, {
		provideCodeActions(document, range, context, token) {
			return new PackageDocument(document).provideCodeActions(range, context, token);
		}
	});
}
```
**Variations / call-sites:**
- Used in tandem with completion provider in main activation path
- Only appears in `extensionEditingMain.ts:33` (not in browser variant)

---

#### Pattern: Diagnostic Collection with File System Watcher
**Where:** `extensions/extension-editing/src/extensionLinter.ts:60-82`
**What:** Creates a diagnostic collection and watches for package.json changes to trigger async linting.
```typescript
export class ExtensionLinter {
	private diagnosticsCollection = languages.createDiagnosticCollection('extension-editing');
	private fileWatcher = workspace.createFileSystemWatcher('**/package.json');
	private disposables: Disposable[] = [this.diagnosticsCollection, this.fileWatcher];

	constructor() {
		this.disposables.push(
			workspace.onDidOpenTextDocument(document => this.queue(document)),
			workspace.onDidChangeTextDocument(event => this.queue(event.document)),
			workspace.onDidCloseTextDocument(document => this.clear(document)),
			this.fileWatcher.onDidChange(uri => this.packageJsonChanged(this.getUriFolder(uri))),
			this.fileWatcher.onDidCreate(uri => this.packageJsonChanged(this.getUriFolder(uri))),
			this.fileWatcher.onDidDelete(uri => this.packageJsonChanged(this.getUriFolder(uri))),
		);
	}
}
```
**Variations / call-sites:**
- File watching used for both package.json and README/CHANGELOG files
- Debouncing implemented via `startTimer()` at line 102-110

---

#### Pattern: JSON Tree Parsing and Validation
**Where:** `extensions/extension-editing/src/extensionLinter.ts:119-201`
**What:** Parses JSON using jsonc-parser and validates specific nodes (icon, badges, activationEvents, apiProposals).
```typescript
private async lintPackageJson() {
	for (const document of Array.from(this.packageJsonQ)) {
		const diagnostics: Diagnostic[] = [];
		const tree = parseTree(document.getText());
		const info = this.readPackageJsonInfo(this.getUriFolder(document.uri), tree);
		if (tree && info.isExtension) {
			const icon = findNodeAtLocation(tree, ['icon']);
			if (icon && icon.type === 'string') {
				this.addDiagnostics(diagnostics, document, icon.offset + 1, icon.offset + icon.length - 1, icon.value, Context.ICON, info);
			}
			// Similar validation for badges, apiProposals, activationEvents
		}
		this.diagnosticsCollection.set(document.uri, diagnostics);
	}
}
```
**Variations / call-sites:**
- Different validation contexts: ICON, BADGE, MARKDOWN (enum at line 40-44)
- Reused in `lintReadme()` for markdown image validation (lines 272-374)

---

#### Pattern: Implicit Activation Event Detection
**Where:** `extensions/extension-editing/src/extensionLinter.ts:500-598`
**What:** Parses contribution declarations and automatically generates expected activation events for validation.
```typescript
function parseImplicitActivationEvents(tree: JsonNode): Set<string> {
	const activationEvents = new Set<string>();
	
	// commands
	const commands = findNodeAtLocation(tree, ['contributes', 'commands']);
	commands?.children?.forEach(child => {
		const command = findNodeAtLocation(child, ['command']);
		if (command && command.type === 'string') {
			activationEvents.add(`onCommand:${command.value}`);
		}
	});
	
	// Similar patterns for: authenticationProviders, languages, customEditors, views, walkthroughs, notebookRenderers, terminalProfiles, terminalQuickFixes, tasks
	return activationEvents;
}
```
**Variations / call-sites:**
- Referenced at line 399 in `readPackageJsonInfo()`
- Used to detect redundant explicit activation events at lines 167-171

---

#### Pattern: Definition/Reference Provider Interface Implementation
**Where:** `extensions/extension-editing/src/packageDocumentL10nSupport.ts:13-29`
**What:** Implements multiple provider interfaces (DefinitionProvider, ReferenceProvider) with dual DocumentSelector registration for package.json and package.nls.json.
```typescript
export class PackageDocumentL10nSupport implements vscode.DefinitionProvider, vscode.ReferenceProvider, vscode.Disposable {
	private readonly _disposables: vscode.Disposable[] = [];

	constructor() {
		this._disposables.push(vscode.languages.registerDefinitionProvider(packageJsonSelector, this));
		this._disposables.push(vscode.languages.registerDefinitionProvider(packageNlsJsonSelector, this));

		this._disposables.push(vscode.languages.registerReferenceProvider(packageNlsJsonSelector, this));
		this._disposables.push(vscode.languages.registerReferenceProvider(packageJsonSelector, this));
	}

	dispose(): void {
		for (const d of this._disposables) {
			d.dispose();
		}
	}
}
```
**Variations / call-sites:**
- Both providers handle two document types with shared implementation logic
- Used in both main (line 20) and browser (line 15) entry points

---

#### Pattern: When-Clause Validation with External Command Dispatch
**Where:** `extensions/extension-editing/src/extensionLinter.ts:203-270`
**What:** Validates when-clauses by dispatching to VS Code's internal `_validateWhenClauses` command and maps errors back to source positions.
```typescript
private async lintWhenClauses(contributesNode: JsonNode | undefined, document: TextDocument): Promise<Diagnostic[]> {
	// ... recursively find when-clause strings in menus, views, keybindings
	const parseResults = await commands.executeCommand<{ errorMessage: string; offset: number; length: number }[][]>(
		'_validateWhenClauses', 
		whenClauses.map(w => w.value as string)
	);

	const diagnostics: Diagnostic[] = [];
	for (let i = 0; i < parseResults.length; ++i) {
		const whenClauseJSONNode = whenClauses[i];
		const jsonStringScanner = new JsonStringScanner(document.getText(), whenClauseJSONNode.offset + 1);
		for (const error of parseResults[i]) {
			const realOffset = jsonStringScanner.getOffsetInEncoded(error.offset);
			// ... create diagnostic
		}
	}
	return diagnostics;
}
```
**Variations / call-sites:**
- Used during package.json linting at line 196
- Requires JsonStringScanner helper (line 15) to handle escaped strings

---

## Integration Summary

The extension-editing extension demonstrates a layered architecture for manifest validation:

1. **Registration Layer**: Multiple provider types (completion, code actions, definition, reference) registered via DocumentSelector pattern targeting `**/package.json`
2. **Linting Layer**: ExtensionLinter class monitors file changes and performs async validation against contribution declarations
3. **Parsing Layer**: jsonc-parser used for JSON AST traversal; external command dispatch for complex validation (when-clauses)
4. **Localization Layer**: Separate L10n support for mapping NLS references between package.json and package.nls.json
5. **Activation**: Main and browser variants follow identical registration patterns

### Key Implementation Details

- **Document Selector Pattern**: Consistent use of `{ language: 'json', pattern: '**/package.json' }` across all providers
- **Async Debouncing**: 300ms timer debounces rapid document changes before linting (line 106)
- **Disposable Management**: Providers and watchers tracked in subscriptions for proper cleanup
- **Error Context**: Diagnostics include URI targets for external documentation links (e.g., activation-events, when-clause-contexts)
- **Implicit Event Detection**: Automatically generates expected activation events from contribution declarations to detect redundant explicit entries

This pattern hierarchy could be adapted for a Tauri/Rust implementation by:
- Replacing `vscode.languages` registrations with custom language server protocol handlers
- Converting file watchers to Rust file system events
- Porting jsonc-parser or using serde_json with custom validation logic
- Implementing the implicit event detection as a separate validation pass

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
