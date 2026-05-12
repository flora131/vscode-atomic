# Extension Editing Module - Partition 34/80

## Scope
`extensions/extension-editing/` — 18 files, 1,326 LOC

## Research Question
Porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

## Summary
The extension-editing module implements package.json IntelliSense and validation for VS Code extensions. It provides completion items, code actions, and diagnostics for extension manifest files. Two entry points exist: `extensionEditingMain.ts` for Node/desktop and `extensionEditingBrowserMain.ts` for web contexts. The module uses `vscode.languages.*` APIs for editor integration and relies on JSON parsing, Markdown processing, and HTML validation.

---

## Implementation

### Core Services
- `extensions/extension-editing/src/extensionEditingMain.ts` — Desktop activation, registers completion items (line 26) and code actions (line 34) via `vscode.languages.registerCompletionItemProvider` and `vscode.languages.registerCodeActionsProvider`
- `extensions/extension-editing/src/extensionEditingBrowserMain.ts` — Web-context activation, registers only completions (line 19)
- `extensions/extension-editing/src/packageDocumentHelper.ts` — PackageDocument class providing completion and code action logic for package.json
- `extensions/extension-editing/src/packageDocumentL10nSupport.ts` — Definition/reference providers for NLS string localization (lines 18-22)
- `extensions/extension-editing/src/extensionLinter.ts` — ExtensionLinter class implementing diagnostics collection for package.json and README validation

### Parser/Utility
- `extensions/extension-editing/src/extensionEngineValidation.ts` — Version parsing (IParsedVersion, INormalizedVersion interfaces)
- `extensions/extension-editing/src/jsonReconstruct.ts` — JsonStringScanner class for JSON string offset mapping
- `extensions/extension-editing/src/constants.ts` — Localized message constants for activation events

### Dependencies
- `jsonc-parser` (v3.2.0) — JSON with comments parsing
- `markdown-it` (v12.3.2) — Markdown parser
- `parse5` (v3.0.2) — HTML5 parser (for embedded SVG detection in README)

---

## Configuration

### Build Configuration
- `extensions/extension-editing/esbuild.mts` — esbuild entry points: `extensionEditingMain` (Node platform)
- `extensions/extension-editing/esbuild.browser.mts` — Web build configuration (implicitly referenced)
- `extensions/extension-editing/tsconfig.json` — TypeScript configuration, extends `tsconfig.base.json`, outputs to `./out`
- `extensions/extension-editing/tsconfig.browser.json` — Browser-specific TypeScript configuration

### Package Configuration
- `extensions/extension-editing/package.json` — Extension manifest (v10.0.0)
  - Entry points: `./out/extensionEditingMain` (desktop), `./dist/browser/extensionEditingBrowserMain` (web)
  - Activation events: `onLanguage:json`, `onLanguage:markdown`
  - Contributes: JSON schema validation for package.json, language-configuration.json, theme files
  - Virtual workspace and untrusted workspace support enabled

### Runtime Configuration
- `extensions/extension-editing/.npmrc` — NPM registry configuration
- `extensions/extension-editing/.vscodeignore` — File exclusion patterns for packaging

---

## Types / Interfaces

### In `extensionEngineValidation.ts`
- `IParsedVersion` — Parsed version string components (hasCaret, hasGreaterEquals, majorBase, minorBase, patchBase, preRelease)
- `INormalizedVersion` — Normalized version for comparison (majorBase, minorBase, patchBase, notBefore timestamp, isMinimum flag)

### In `extensionLinter.ts`
- `TokenAndPosition` (internal) — Markdown token with begin/end offsets
- `PackageJsonInfo` (internal) — Cached metadata: isExtension, hasHttpsRepository, repository URI, implicit activation events, engine version

### In `jsonReconstruct.ts`
- `JsonStringScanner` — Utility class for string offset encoding/decoding

---

## Examples / Fixtures

### Package.json Contribution Patterns
Validated in extensionLinter:
- `package.json` root — icon, badges, repository, engines.vscode, activationEvents, enabledApiProposals
- `contributes.menus|views|viewsWelcome|keybindings` — when-clause validation
- `contributes.commands` — enablement-clause validation
- Implicit activation event prefixes: `onLanguage:`, `onView:`, `onCommand:`, `onCustomEditor:`, etc.

### README/CHANGELOG Validation
- Image URLs in Markdown (HTTPS requirement, trusted SVG sources)
- Relative image URLs (HTTPS repository requirement)
- Embedded SVGs in HTML (disallowed)
- Data URLs (disallowed)

### Diagnostic Categories
- HTTPS protocol enforcement
- Badge provider whitelist (from product.json)
- API proposal validation (against extensionEnabledApiProposals in product.json)
- When-clause parsing errors
- Star activation warning

---

## Notable Clusters

### Language Service Integration Points
- **Completion Provider** (packageDocumentHelper): Provides language-specific override snippets for configurationDefaults
- **Code Actions Provider** (packageDocumentHelper): Quick-fix removal of implicit/redundant activation events
- **Definition Provider** (packageDocumentL10nSupport): Go-to-definition for NLS references (package.json → package.nls.json)
- **Reference Provider** (packageDocumentL10nSupport): Find references across package.json/package.nls.json files
- **Diagnostic Collection** (extensionLinter): Real-time validation with debounced linting (300ms timer)

### File Monitoring
ExtensionLinter uses:
- `workspace.createFileSystemWatcher('**/package.json')` — Detects manifest changes
- `workspace.onDidOpenTextDocument`, `onDidChangeTextDocument`, `onDidCloseTextDocument` — Document lifecycle tracking
- Queued processing with debounce to avoid redundant linting

### JSON Schema Validation (package.json)
- `vscode://schemas/vscode-extensions` — Extension manifest schema
- `vscode://schemas/language-configuration` — Language definition schema
- `vscode://schemas/icon-theme`, `vscode://schemas/color-theme` — Theme schemas

### Implicit Activation Event Detection
Lines 500-598 in extensionLinter.ts parse contributes and extract:
- Commands → `onCommand:` events
- Authentication providers → `onAuthenticationRequest:` events
- Languages with configuration → `onLanguage:` events
- Custom editors → `onCustomEditor:` events
- Views → `onView:` events
- Walkthroughs → `onWalkthrough:` events
- Notebook renderers → `onRenderer:` events
- Terminal profiles → `onTerminalProfile:` events
- Terminal quick fixes → `onTerminalQuickFixRequest:` events
- Task definitions → `onTaskType:` events

---

## Porting Considerations

### Language Service APIs to Replicate
1. **registerCompletionItemProvider** — Snippet-based completion for language overrides
2. **registerCodeActionsProvider** — Quick-fix code actions for diagnostics
3. **registerDefinitionProvider** — NLS string definition navigation
4. **registerReferenceProvider** — Cross-file reference finding
5. **createDiagnosticCollection** — Real-time diagnostic reporting
6. **createFileSystemWatcher** — Package.json change detection

### External Dependencies
- JSON parsing with comment support (jsonc-parser)
- Markdown tokenization (markdown-it)
- HTML5 parsing for SVG detection (parse5)
- Product.json loading (fs, path, URL APIs)

### Context-Specific Behavior
- Desktop: Full linting (package.json + README/CHANGELOG validation)
- Web: Completions only (no file system watcher, lighter footprint)

### Activation Model
- Lazy: Triggers on `onLanguage:json` and `onLanguage:markdown`
- Subscriptions: All providers registered during activate(), cleaned up on dispose()

