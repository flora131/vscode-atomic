# ESLint Plugin Local Rules - Complete File Inventory

## Implementation

### Core Rules (Code-based)
- `.eslint-plugin-local/code-declare-service-brand.ts`
- `.eslint-plugin-local/code-amd-node-module.ts`
- `.eslint-plugin-local/code-ensure-no-disposables-leak-in-test.ts`
- `.eslint-plugin-local/code-import-patterns.ts`
- `.eslint-plugin-local/code-no-deep-import-of-internal.ts`
- `.eslint-plugin-local/code-no-test-async-suite.ts`
- `.eslint-plugin-local/code-no-localization-template-literals.ts`
- `.eslint-plugin-local/code-no-reader-after-await.ts`
- `.eslint-plugin-local/code-no-nls-in-standalone-editor.ts`
- `.eslint-plugin-local/code-no-observable-get-in-reactive-context.ts`
- `.eslint-plugin-local/code-must-use-result.ts`
- `.eslint-plugin-local/code-no-static-self-ref.ts`
- `.eslint-plugin-local/code-no-dangerous-type-assertions.ts`
- `.eslint-plugin-local/code-no-icons-in-localized-strings.ts`
- `.eslint-plugin-local/code-must-use-super-dispose.ts`
- `.eslint-plugin-local/code-no-runtime-import.ts`
- `.eslint-plugin-local/code-no-any-casts.ts`
- `.eslint-plugin-local/code-no-telemetry-common-property.ts`
- `.eslint-plugin-local/code-no-http-import.ts`
- `.eslint-plugin-local/code-no-localized-model-description.ts`
- `.eslint-plugin-local/code-no-potentially-unsafe-disposables.ts`
- `.eslint-plugin-local/code-no-global-document-listener.ts`
- `.eslint-plugin-local/code-no-test-only.ts`
- `.eslint-plugin-local/code-no-in-operator.ts`
- `.eslint-plugin-local/code-layering.ts`
- `.eslint-plugin-local/code-no-accessor-after-await.ts`
- `.eslint-plugin-local/code-no-declare-const-enum.ts`
- `.eslint-plugin-local/code-no-standalone-editor.ts`
- `.eslint-plugin-local/code-limited-top-functions.ts`
- `.eslint-plugin-local/code-no-static-node-module-import.ts`
- `.eslint-plugin-local/code-policy-localization-key-match.ts`
- `.eslint-plugin-local/code-no-unused-expressions.ts`
- `.eslint-plugin-local/code-parameter-properties-must-have-explicit-accessibility.ts`

### VS Code Definitions Rules (vscode.d.ts-specific)
- `.eslint-plugin-local/vscode-dts-event-naming.ts`
- `.eslint-plugin-local/vscode-dts-literal-or-types.ts`
- `.eslint-plugin-local/vscode-dts-vscode-in-comments.ts`
- `.eslint-plugin-local/vscode-dts-interface-naming.ts`
- `.eslint-plugin-local/vscode-dts-cancellation.ts`
- `.eslint-plugin-local/vscode-dts-create-func.ts`
- `.eslint-plugin-local/vscode-dts-use-export.ts`
- `.eslint-plugin-local/vscode-dts-string-type-literals.ts`
- `.eslint-plugin-local/vscode-dts-provider-naming.ts`
- `.eslint-plugin-local/vscode-dts-use-thenable.ts`

### Entry Point & Utilities
- `.eslint-plugin-local/index.ts` - Rule loader and re-exporter (glob-based dynamic loading)
- `.eslint-plugin-local/utils.ts` - Shared helper utilities for rule implementations

## Tests

- `.eslint-plugin-local/tests/code-no-reader-after-await-test.ts` - Tests for await/reader invariant
- `.eslint-plugin-local/tests/code-no-observable-get-in-reactive-context-test.ts` - Tests for observable usage in reactive contexts

## Configuration

- `.eslint-plugin-local/tsconfig.json` - TypeScript configuration (ESM, strict mode, no emit)
- `.eslint-plugin-local/package.json` - Package metadata, typecheck script

## Documentation

- `.eslint-plugin-local/README.md` - Comprehensive guide to custom ESLint rules, creation patterns, and architectural conventions

## Notable Clusters

**Disposable/Resource Management Rules (5 files)**
- `code-ensure-no-disposables-leak-in-test.ts`
- `code-must-use-super-dispose.ts`
- `code-no-potentially-unsafe-disposables.ts`
- `code-no-accessor-after-await.ts` (related to disposal timing)
- `code-no-reader-after-await.ts` (related to resource safety)

**Observable/Reactive Context Rules (2 files)**
- `code-no-observable-get-in-reactive-context.ts`
- Test file for reactive context validation

**Localization Rules (5 files)**
- `code-no-localization-template-literals.ts`
- `code-no-nls-in-standalone-editor.ts`
- `code-no-icons-in-localized-strings.ts`
- `code-no-localized-model-description.ts`
- `code-policy-localization-key-match.ts`

**Layering & Import Rules (5 files)**
- `code-layering.ts` - Core layering enforcement
- `code-import-patterns.ts` - Import pattern validation
- `code-no-deep-import-of-internal.ts` - Prevents deep internal imports
- `code-no-static-node-module-import.ts` - Restricts static module imports
- `code-amd-node-module.ts` - AMD/Node module compatibility

**Type Safety Rules (5 files)**
- `code-no-any-casts.ts`
- `code-no-dangerous-type-assertions.ts`
- `code-parameter-properties-must-have-explicit-accessibility.ts`
- `code-declare-service-brand.ts`
- `code-no-declare-const-enum.ts`

**Async/Await Safety (3 files)**
- `code-no-reader-after-await.ts`
- `code-no-accessor-after-await.ts`
- `code-no-test-async-suite.ts`

**VS Code Definition Type Rules (10 files)**
- All `vscode-dts-*.ts` files enforce API surface consistency, naming conventions, and type representation

---

## Summary

The `.eslint-plugin-local/` directory contains 49 files implementing VS Code's custom ESLint plugin that encodes critical architectural invariants. The 34 core rules enforce three primary categories of constraints:

1. **Layering & Architecture**: Rules prevent violations of VS Code's modular architecture through import restrictions and layering boundaries
2. **Resource/Lifecycle Management**: Rules around disposables, observables, and async/await safety reflect VS Code's careful resource management patterns
3. **API Surface Consistency**: vscode.d.ts-specific rules ensure the public API maintains naming consistency and type safety

For a Rust port, these rules represent essential architectural patterns that must be either enforced through Rust's type system or replaced with idiomatic Rust equivalents (e.g., RAII for disposables, async/await patterns, lifetime annotations for reactive contexts).
