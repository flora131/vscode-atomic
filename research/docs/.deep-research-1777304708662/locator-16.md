# File Locations: PHP Language Features Extension (Partition 16)

## Research Context
The PHP Language Features extension provides basic language intelligence for PHP files in VS Code. It implements core IDE features like completions, hover information, signature help, and validation. This extension is relevant for understanding how VS Code implements language-specific intelligence providers that would need porting to a Tauri/Rust architecture.

### Implementation

- `extensions/php-language-features/src/phpMain.ts` — Main extension entry point; registers language providers (completions, hover, signature help, validation) for PHP via VS Code's extension API
- `extensions/php-language-features/src/features/completionItemProvider.ts` — Implements CompletionItemProvider to supply autocomplete suggestions for PHP built-ins, global functions, variables, keywords, and user-defined functions via regex parsing
- `extensions/php-language-features/src/features/hoverProvider.ts` — Implements HoverProvider to display documentation and signatures for PHP globals and functions on hover
- `extensions/php-language-features/src/features/signatureHelpProvider.ts` — Implements SignatureHelpProvider to display function signatures and parameter hints during function calls; includes BackwardIterator for parsing context
- `extensions/php-language-features/src/features/validationProvider.ts` — Implements PHP file validation using `child_process` to spawn external PHP executable; handles on-save and on-type validation modes with diagnostic reporting
- `extensions/php-language-features/src/features/phpGlobalFunctions.ts` — Data file containing built-in PHP function metadata (names, signatures, descriptions)
- `extensions/php-language-features/src/features/phpGlobals.ts` — Data file containing PHP built-in variables, constants, and keywords with descriptions

### Types / Interfaces

- `extensions/php-language-features/src/typings/node.additions.d.ts` — TypeScript type augmentations for Node.js timer functions (setTimeout, setInterval, etc.)

### Utilities

- `extensions/php-language-features/src/features/utils/async.ts` — Async utilities including Throttler, Delayer, and ThrottledDelayer classes for managing sequential async tasks and debouncing validation requests
- `extensions/php-language-features/src/features/utils/markedTextUtil.ts` — Markdown escaping utility for hover content rendering

### Configuration

- `extensions/php-language-features/package.json` — Extension manifest; activates on PHP files; declares configuration options for suggestions (suggest.basic), validation (validate.enable, validate.executablePath, validate.run), and composer.json JSON schema validation
- `extensions/php-language-features/tsconfig.json` — TypeScript compiler configuration extending base config; targets Node.js runtime with output to dist/ directory
- `extensions/php-language-features/esbuild.mts` — Build configuration using esbuild for bundling the extension entry point

### Documentation

- `extensions/php-language-features/README.md` — Minimal documentation noting this is a bundled extension; references external documentation at code.visualstudio.com

### Other Files

- `extensions/php-language-features/package-lock.json` — Dependency lock file
- `extensions/php-language-features/package.nls.json` — Localization strings for configuration UI
- `extensions/php-language-features/.npmrc` — NPM configuration
- `extensions/php-language-features/.vscodeignore` — Packaging exclusion rules
- `extensions/php-language-features/icons/logo.png` — PHP language logo icon

---

## Summary

This partition contains a complete, minimal language extension implementation demonstrating core IDE features relevant to a Tauri/Rust port. The extension shows the pattern for implementing CompletionItemProvider, HoverProvider, SignatureHelpProvider, and DiagnosticCollection-based validation. The validation provider is particularly relevant, as it demonstrates spawning external processes (PHP) for linting/checking. All code is TypeScript and uses VS Code's extension API for registering language providers and handling configuration changes. The extension has no direct dependencies beyond the vscode API and Node.js standard library (child_process, path, string_decoder).

