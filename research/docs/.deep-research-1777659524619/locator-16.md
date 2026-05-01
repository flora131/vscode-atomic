# PHP Language Features Extension - IDE Porting Analysis

## Implementation
- `extensions/php-language-features/src/phpMain.ts` — Extension entry point registering language service providers (hover, completion, signature help, diagnostics)
- `extensions/php-language-features/src/features/hoverProvider.ts` — Hover information provider for PHP language intelligence
- `extensions/php-language-features/src/features/completionItemProvider.ts` — Autocomplete/IntelliSense provider for PHP
- `extensions/php-language-features/src/features/signatureHelpProvider.ts` — Function signature help provider for PHP
- `extensions/php-language-features/src/features/validationProvider.ts` — Diagnostic validation and error reporting tied to workspace/document events
- `extensions/php-language-features/src/features/phpGlobalFunctions.ts` — PHP built-in function definitions for language intelligence
- `extensions/php-language-features/src/features/phpGlobals.ts` — Global PHP constants and built-in identifiers
- `extensions/php-language-features/src/features/utils/async.ts` — Async utility helpers for provider implementations
- `extensions/php-language-features/src/features/utils/markedTextUtil.ts` — Markdown formatting utilities for hover/completion documentation

## Configuration
- `extensions/php-language-features/package.json` — Extension manifest with language server activation events, capabilities (virtualWorkspaces, untrustedWorkspaces), config schema for php.suggest.basic, php.validate.enable/run/executablePath, and composer.json JSON schema
- `extensions/php-language-features/tsconfig.json` — TypeScript compiler configuration extending base config with vscode.d.ts type definitions
- `extensions/php-language-features/esbuild.mts` — Build configuration for extension bundling
- `extensions/php-language-features/.npmrc` — NPM configuration for package management

## Types / Interfaces
- `extensions/php-language-features/src/typings/node.additions.d.ts` — Custom Node.js type augmentations

## Documentation
- `extensions/php-language-features/README.md` — Extension user-facing documentation
- `extensions/php-language-features/package.nls.json` — Localization strings for UI text

## Notable Clusters
- `extensions/php-language-features/src/features/` — 7 provider implementations (completion, hover, signature help, validation) plus utilities for async operations and text rendering; forms the core language service provider layer
- `extensions/php-language-features/src/features/utils/` — 2 utility modules for async handling and markdown formatting used by providers

## Porting Relevance

This extension exemplifies VS Code's language intelligence architecture that would require reimplementation in a Tauri/Rust port:

1. **Language Provider Registration Model** — Uses `vscode.languages.registerXyzProvider()` API with pluggable provider classes; any Tauri port must establish equivalent provider registration and dispatch mechanisms

2. **Configuration System** — Dynamic workspace configuration via `vscode.workspace.getConfiguration()` tied to user settings; Tauri port requires equivalent config resolution and change event propagation

3. **Event-Driven Architecture** — Validation triggered by workspace events (`onDidChangeTextDocument`, `onDidSaveTextDocument`, `onDidChangeConfiguration`); demonstrates event subscription patterns critical for IDE responsiveness

4. **Document/Workspace Abstraction** — Uses `vscode.workspace.textDocuments`, `vscode.workspace.workspaceFolders`, and document URIs for file system abstraction; Tauri port needs equivalent document model

5. **Diagnostics Collection** — `vscode.languages.createDiagnosticCollection()` for reporting validation errors; Tauri port needs diagnostic storage and rendering infrastructure

6. **Async/Promise-Based APIs** — Providers return Promises for hover content, completions, etc.; Tauri port must support async RPC or message-based provider calls

7. **Process Execution** — Uses `child_process` (via `which` package) to locate and execute PHP validator; Tauri port needs subprocess management for external tool integration
