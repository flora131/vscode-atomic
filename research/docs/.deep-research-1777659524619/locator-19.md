# ESLint Plugin Local Rules for VS Code Tauri/Rust Port

## Overview

The `.eslint-plugin-local/` directory contains 49 custom ESLint rules (TypeScript) that encode VS Code-specific architectural invariants. These rules enforce patterns critical for IDE core functionality, particularly around resource lifecycle management, code layering, and safe abstractions. Any port to Tauri/Rust must preserve these invariants semantically.

## Implementation

### Code Layering & Architecture

- `.eslint-plugin-local/code-layering.ts` - Enforces strict separation of layers (common, base, platform, workbench) with allowed imports per layer
- `.eslint-plugin-local/code-import-patterns.ts` - Validates allowed import patterns and prevents invalid module imports
- `.eslint-plugin-local/code-no-deep-import-of-internal.ts` - Prevents deep imports of internal modules
- `.eslint-plugin-local/code-no-static-node-module-import.ts` - Ensures static imports of node modules follow patterns

### Resource & Lifecycle Management

- `.eslint-plugin-local/code-ensure-no-disposables-leak-in-test.ts` - Prevents resource leaks in test code (critical for Disposable pattern)
- `.eslint-plugin-local/code-must-use-super-dispose.ts` - Enforces super.dispose() calls in override dispose methods
- `.eslint-plugin-local/code-must-use-result.ts` - Ensures must-use results are captured
- `.eslint-plugin-local/code-no-potentially-unsafe-disposables.ts` - Guards against unsafe disposable patterns
- `.eslint-plugin-local/code-declare-service-brand.ts` - Enforces service brand declarations

### Async/Await & Reactive Patterns

- `.eslint-plugin-local/code-no-reader-after-await.ts` - Prevents reading mutable state after await (thread safety analog)
- `.eslint-plugin-local/code-no-accessor-after-await.ts` - Prevents accessor calls on mutable objects after await
- `.eslint-plugin-local/code-no-observable-get-in-reactive-context.ts` - Prevents breaking reactive chains with observable.get()
- `.eslint-plugin-local/code-no-test-async-suite.ts` - Restricts async patterns in test suites

### Module & Type System

- `.eslint-plugin-local/code-amd-node-module.ts` - Validates AMD module module declarations
- `.eslint-plugin-local/code-no-in-operator.ts` - Prevents unsafe in operator usage
- `.eslint-plugin-local/code-no-any-casts.ts` - Bans unsafe any type casts
- `.eslint-plugin-local/code-no-dangerous-type-assertions.ts` - Guards dangerous type assertions
- `.eslint-plugin-local/code-no-declare-const-enum.ts` - Prevents const enum declarations
- `.eslint-plugin-local/code-parameter-properties-must-have-explicit-accessibility.ts` - Enforces visibility on parameter properties
- `.eslint-plugin-local/code-no-static-self-ref.ts` - Prevents static self-references
- `.eslint-plugin-local/code-no-unused-expressions.ts` - Detects unused expressions

### Runtime & Environment Safety

- `.eslint-plugin-local/code-no-http-import.ts` - Prevents http imports (security/compatibility)
- `.eslint-plugin-local/code-no-runtime-import.ts` - Guards against certain runtime imports
- `.eslint-plugin-local/code-no-nls-in-standalone-editor.ts` - Prevents NLS usage in standalone editor
- `.eslint-plugin-local/code-no-standalone-editor.ts` - Restricts standalone editor imports
- `.eslint-plugin-local/code-no-global-document-listener.ts` - Prevents global document listener patterns
- `.eslint-plugin-local/code-limited-top-functions.ts` - Limits size of top-level functions

### Localization & Configuration

- `.eslint-plugin-local/code-no-localization-template-literals.ts` - Prevents template literals in localization
- `.eslint-plugin-local/code-no-unexternalized-strings.ts` - Enforces externalized strings for localization
- `.eslint-plugin-local/code-no-localized-model-description.ts` - Prevents incorrect localization patterns
- `.eslint-plugin-local/code-no-icons-in-localized-strings.ts` - Prevents icon codes in localized strings
- `.eslint-plugin-local/code-translation-remind.ts` - Reminds about translation requirements
- `.eslint-plugin-local/code-policy-localization-key-match.ts` - Validates localization key patterns
- `.eslint-plugin-local/code-no-telemetry-common-property.ts` - Guards telemetry property usage

### Test-Specific Patterns

- `.eslint-plugin-local/code-no-test-only.ts` - Prevents test.only() from being checked in

### VSCode Type Definition Rules (vscode.d.ts)

- `.eslint-plugin-local/vscode-dts-event-naming.ts` - Enforces Event naming conventions
- `.eslint-plugin-local/vscode-dts-interface-naming.ts` - Enforces interface naming patterns
- `.eslint-plugin-local/vscode-dts-provider-naming.ts` - Enforces provider naming patterns
- `.eslint-plugin-local/vscode-dts-cancellation.ts` - Enforces CancellationToken patterns
- `.eslint-plugin-local/vscode-dts-create-func.ts` - Validates create function signatures
- `.eslint-plugin-local/vscode-dts-use-export.ts` - Enforces proper export patterns
- `.eslint-plugin-local/vscode-dts-string-type-literals.ts` - Validates string type literals
- `.eslint-plugin-local/vscode-dts-literal-or-types.ts` - Enforces literal or union type patterns
- `.eslint-plugin-local/vscode-dts-use-thenable.ts` - Enforces Thenable patterns over Promise
- `.eslint-plugin-local/vscode-dts-vscode-in-comments.ts` - Validates vscode comment patterns

## Tests

- `.eslint-plugin-local/tests/code-no-reader-after-await-test.ts` - Test cases for reader-after-await rule
- `.eslint-plugin-local/tests/code-no-observable-get-in-reactive-context-test.ts` - Test cases for observable pattern rule

## Configuration

- `.eslint-plugin-local/tsconfig.json` - TypeScript configuration for rule compilation
- `.eslint-plugin-local/package.json` - NPM package definition (module-type, typecheck script)
- `.eslint-plugin-local/index.ts` - Rule registry (dynamically loads all rule files excluding utils.ts and index.ts)

## Examples / Fixtures

- Test files in `.eslint-plugin-local/tests/` directory contain example code patterns

## Documentation

- `.eslint-plugin-local/README.md` - Comprehensive guide for creating and managing custom ESLint rules with examples

## Notable Clusters

### Service Architecture Rules (8 files)
Code layering, service brand declaration, import patterns - enforce module separation and service discovery patterns essential to VS Code's architecture.

### Disposable Pattern Rules (5 files)
Resource management rules (must-use-super-dispose, ensure-no-disposables-leak-in-test, potentially-unsafe-disposables) - model the memory safety concepts that Rust's ownership system would handle natively.

### Reactive/Async Safety Rules (4 files)
Reader/accessor-after-await, observable patterns - prevent data races and reactivity violations analogous to Rust's borrow checker constraints.

### Type System Strictness Rules (7 files)
No any casts, const enum, dangerous assertions - enforce compile-time safety that complements Rust's strong type system.

### Localization Infrastructure Rules (6 files)
Ensure all user-facing strings are externalized and properly structured for internationalization across the codebase.

### vscode.d.ts API Contract Rules (10 files)
Enforce naming and structural conventions in the public API surface that extension authors depend on.

### Utility Functions (1 file)
- `.eslint-plugin-local/utils.ts` - Provides shared utilities for rules (createImportRuleListener)

---

## Semantic Preservation for Tauri/Rust Port

The 49 custom rules in `.eslint-plugin-local/` encode invariants that a TypeScript codebase relies on for safety. A Rust port must preserve these semantically:

1. **Layering Rules** → Rust module visibility (pub/private) and dependency graph constraints
2. **Disposable Pattern** → RAII and Drop trait semantics
3. **Reader-After-Await** → Borrow checker and mutable reference safety
4. **Observable Patterns** → Stream/reactive trait contracts
5. **Type Strictness** → Leveraged natively by Rust's type system
6. **Localization** → String externalization layer (unchanged architectural pattern)
7. **API Contract** → Equivalent constraints on vscode-rs public API

These rules validate runtime behavior that would be compile-time enforced or runtime-checked in Rust, making them the highest-value artifacts for understanding VS Code's core architecture invariants. The transformation from TypeScript enforcement to Rust semantics represents the conceptual backbone of a safe IDE port, where ESLint catches errors that Rust's type system would prevent at compile time.
