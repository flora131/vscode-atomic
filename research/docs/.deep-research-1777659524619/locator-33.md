# Configuration Editing Extension: Core IDE Porting Surface Analysis

## Summary
The `configuration-editing` extension implements VS Code's settings-aware IDE completion system through the `vscode.languages.registerCompletionItemProvider()` API. Porting to Tauri/Rust would require reimplementing 5 completion provider registrations that touch sensitive configuration surfaces (settings.json, extensions.json, launch.json, tasks.json, keybindings.json), plus the underlying JSON/JSONC parsing and language services integration layer.

---

## Implementation
- `extensions/configuration-editing/src/configurationEditingMain.ts` — Extension activation & 5 completion provider registrations (settings, variables, extensions, context keys)
- `extensions/configuration-editing/src/settingsDocumentHelper.ts` — Settings-aware completion logic; handles window.title, files.associations, excludes, language overrides, ports (SettingsDocument class)
- `extensions/configuration-editing/src/extensionsProposals.ts` — Extension recommendation proposals from installed extensions
- `extensions/configuration-editing/src/importExportProfiles.ts` — GitHub Gist profile import/export (ProfileContentHandler interface, Octokit integration)
- `extensions/configuration-editing/src/node/net.ts` — HTTPS proxy agent handling for Node.js
- `extensions/configuration-editing/src/browser/net.ts` — Browser stub for net module

## Tests
- `extensions/configuration-editing/src/test/completion.test.ts` — Mocha integration tests for completion in settings.json (window.title, language overrides, exclude patterns)
- `extensions/configuration-editing/src/test/index.ts` — Test runner configuration with mocha-junit-reporter integration

## Types / Interfaces
- `SettingsDocument` class — Parses JSONC and provides context-aware completions
- `IExtensionsContent` interface — Shape for extensions.json document structure
- `ItemDescription` interface (test) — Completion result verification helper
- `ContextKeyInfo` interface — Context key metadata (key, type, description)

## Configuration
- `extensions/configuration-editing/package.json` — Extension manifest; registers JSONC language for settings.json, launch.json, tasks.json, keybindings.json, extensions.json, .code-workspace files; contributes jsonValidation for 28+ file patterns
- `extensions/configuration-editing/tsconfig.json` — TypeScript config; includes vscode.d.ts and vscode.proposed.profileContentHandlers.d.ts
- `extensions/configuration-editing/tsconfig.browser.json` — Browser build variant
- `extensions/configuration-editing/esbuild.mts` & `esbuild.browser.mts` — Build pipelines for Node.js and browser targets
- `extensions/configuration-editing/.npmrc` & `extensions/configuration-editing/package-lock.json` — npm dependencies (jsonc-parser, @octokit/rest, tunnel)

## Notable Clusters
- `extensions/configuration-editing/schemas/` — 3 schema files for devContainer.json variants (vscode-specific, codespaces-specific, attachContainer configs); external sourced from devcontainers/spec repository

---

## Porting Requirements

### Core API Dependencies
The extension fundamentally depends on:
1. **vscode.languages.registerCompletionItemProvider()** — Pattern-based language selector (language + glob pattern)
2. **vscode.extensions.all** — Enumerable extension registry
3. **vscode.commands.executeCommand()** — Command dispatch for 'getContextKeyInfo'
4. **vscode.workspace.openTextDocument()** — File I/O
5. **vscode.l10n.t()** — Localization/i18n layer

### Configuration Surface Coverage
- **settings.json**: 8 completion contexts (window.title, files.associations, files.exclude, search.exclude, explorer.autoRevealExclude, files.defaultLanguage, workbench.editor.label.patterns, settingsSync.ignoredExtensions, remote.extensionKind, remote.portsAttributes)
- **extensions.json** & **.code-workspace**: Extension ID completion
- **launch.json** & **tasks.json**: ${variable} expansion for paths, workspace context
- **keybindings.json** & **package.json**: Context key (when) completions

### Key Implementation Details
- **JSONC Parsing**: Heavily relies on `jsonc-parser` library (3.2.0+) for location tracking and AST traversal
- **Language-Aware**: Uses vscode.languages.match() for multi-document-type support
- **Async**: All completion providers are async; requires getContextKeyInfo command that blocks on external data
- **Localization**: All user-facing strings use vscode.l10n.t() (proposed API)
- **Platform Split**: Proxy handling differs between node (full HTTPS proxy with auth) and browser (no-op)

### Extension Context Requirement
- Extension requires the 'profileContentHandlers' proposal API
- Activation events: onProfile, onProfile:github, onLanguage:json, onLanguage:jsonc

---

## Porting Complexity Assessment

**High-Impact Areas:**
- Completion provider registration system (needs Tauri RPC-style language service binding)
- JSONC location parsing and context awareness (jsonc-parser porting/rewrite)
- Extension enumeration API (requires Tauri plugin for installed extensions manifest)
- Command dispatch system (vscode.commands.executeCommand must map to Rust backend)
- GitHub Gist profile sync (Octokit -> reqwest + GitHub API)

**Lower-Impact:**
- Test infrastructure (mocha can run in Node.js test harness regardless)
- Schema validation (external refs can be downloaded or embedded)

Porting would require ~8-12 weeks of work to implement equivalent Rust services for language completions, extension registry access, and configuration document parsing.

