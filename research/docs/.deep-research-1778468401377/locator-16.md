# PHP Language Features Extension - File Locator Report

## Implementation
- `extensions/php-language-features/src/phpMain.ts` — Extension entry point registering PHP language providers (completion, hover, signature help, validation)
- `extensions/php-language-features/src/features/completionItemProvider.ts` — Implements CompletionItemProvider for PHP autocomplete using global functions and constants
- `extensions/php-language-features/src/features/hoverProvider.ts` — Implements HoverProvider for PHP hover documentation display
- `extensions/php-language-features/src/features/signatureHelpProvider.ts` — Implements SignatureHelpProvider for PHP function signature display with parameter highlighting
- `extensions/php-language-features/src/features/validationProvider.ts` — Implements diagnostic validation by spawning PHP CLI process to analyze code
- `extensions/php-language-features/src/features/phpGlobalFunctions.ts` — Data module containing PHP built-in function definitions
- `extensions/php-language-features/src/features/phpGlobals.ts` — Data module containing PHP global constants, variables, and keywords
- `extensions/php-language-features/src/features/utils/async.ts` — Utility for throttled async operations (ThrottledDelayer)
- `extensions/php-language-features/src/features/utils/markedTextUtil.ts` — Utility for markdown text formatting in documentation displays

## Types / Interfaces
- `extensions/php-language-features/src/typings/node.additions.d.ts` — Custom type definitions for Node.js additions

## Configuration
- `extensions/php-language-features/package.json` — Extension manifest defining activation events, contributed commands, configuration properties (php.suggest.basic, php.validate.enable, php.validate.executablePath, php.validate.run), and composer.json schema validation
- `extensions/php-language-features/tsconfig.json` — TypeScript compiler configuration extending base config with node types
- `extensions/php-language-features/esbuild.mts` — Build configuration for bundling phpMain entry point with esbuild

## Documentation
- `extensions/php-language-features/README.md` — Extension overview noting it's bundled with VS Code and references external documentation

## Notable Clusters
- `extensions/php-language-features/src/features/` — 7 files implementing language intelligence providers (completion, hover, signature help, validation) and supporting utilities
- `extensions/php-language-features/src/features/utils/` — 2 utility files for async operations and text formatting

---

This partition contains a language support extension for PHP that demonstrates core IDE language intelligence features: code completion, hover documentation, function signature help, and real-time validation. The extension uses VS Code's extension API to register providers that hook into the editor. For a Tauri/Rust port, this extension would require:

1. Native Rust implementations of each provider (CompletionItemProvider, HoverProvider, etc.)
2. A language server communication layer (LSP) to bridge editor UI and language services
3. Process management for spawning and communicating with external tools (PHP CLI for validation)
4. Data structures to replace the static function/constant/keyword registries currently in TypeScript/JavaScript
5. Build tooling to compile Rust code and integrate it with the Tauri framework

The architecture here is typical of VS Code language support: lightweight editor integration with providers that delegate to separate services/data sources.
