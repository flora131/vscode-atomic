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

