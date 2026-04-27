# File Locator: `.eslint-plugin-local/` — ESLint Rules for Architectural Invariants

## Overview
The `.eslint-plugin-local/` directory contains 48 custom ESLint rules (3,664 LOC) that encode architectural invariants for VS Code. These rules enforce layering contracts, platform boundaries (Electron/browser/Node), lifecycle management, API conventions, and performance constraints. They are critical documentation of the structural requirements any Rust port must respect.

## Implementation

### Core Architectural Rules
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-layering.ts` — Enforces layer-based import restrictions using dirname matching; prevents breaking upward dependencies
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-import-patterns.ts` — Complex rule defining 7 distinct layers: `common`, `worker`, `browser`, `electron-browser`, `node`, `electron-utility`, `electron-main`; validates ESM compliance and layer-appropriate dependencies
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-deep-import-of-internal.ts` — Prevents circumventing module boundaries; enforces re-export patterns for internal modules

### Platform/Runtime Boundary Rules
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-standalone-editor.ts` — Editor core cannot depend on IDE-specific modules; preserves reusability
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-static-node-module-import.ts` — Requires dynamic `await import(...)` for Node modules to avoid startup performance regression; allows electron and builtins
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-amd-node-module.ts` — Enforces AMD pattern `amdX#importAMDNodeModule` for npm packages; prevents top-level blocking imports
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-http-import.ts` — Restricts `http`/`https` to type-only imports; mandates dynamic `import()` for runtime to prevent slow startup

### Lifecycle & Resource Management Rules
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-must-use-super-dispose.ts` — Overridden `dispose()` must call `super.dispose()`; prevents resource leaks in inheritance chains
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-potentially-unsafe-disposables.ts` — `DisposableStore`/`MutableDisposable` must be `const` or `readonly` to prevent accidental reassignment leaks
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-global-document-listener.ts` — Document listeners must use `targetWindow.document` (not global `document`) to support multi-window scenarios
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-ensure-no-disposables-leak-in-test.ts` — Test-specific rule for disposable cleanup verification

### Service Injection & Runtime Initialization
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-declare-service-brand.ts` — Service interfaces use phantom `_serviceBrand: undefined` for nominal typing; must have no initializer value
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-accessor-after-await.ts` — `ServicesAccessor` (DI parameter) becomes invalid after `await`; detects usage across async boundaries with branch-aware flow analysis
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-reader-after-await.ts` — Reactive `reader` parameter invalid after `await`; complex AST analysis for reactive function contexts

### Reactive/Observables Rules
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-observable-get-in-reactive-context.ts` — Observable `.get()` forbidden in reactive contexts (`derived`, `autorun` callbacks); must use `.read(reader)` for dependency tracking

### Import & Module Pattern Rules
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-runtime-import.ts` — Type imports only for specified modules; supports glob patterns and conditional allow rules by platform
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-http-import.ts` — `http`/`https` as type-only; dynamic import at runtime

### Type System Rules
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-any-casts.ts` — Restricts `as any` type assertions
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-dangerous-type-assertions.ts` — Validates unsafe type casts
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-declare-const-enum.ts` — Prevents `const enum` which don't survive TypeScript compilation to ESM

### Code Quality & Pattern Rules
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-unused-expressions.ts` — Unused expressions (stricter than default)
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-in-operator.ts` — Restricts `in` operator usage
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-static-self-ref.ts` — Static members cannot reference themselves
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-parameter-properties-must-have-explicit-accessibility.ts` — Constructor shorthand properties must be explicit `public`/`private`/`protected`

### Test-Specific Rules
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-test-only.ts` — Prevents checking in `test.only()`
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-test-async-suite.ts` — Test suites cannot be async

### Localization Rules
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-unexternalized-strings.ts` — User-facing strings must use NLS; includes auto-fix capability
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-localization-template-literals.ts` — Prevents template literals in localization keys
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-localized-model-description.ts` — Model descriptions cannot be localized inline
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-icons-in-localized-strings.ts` — Icons must not be embedded in localized strings
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-translation-remind.ts` — Warns on strings needing translation
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-policy-localization-key-match.ts` — Enforces localization key naming conventions
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-nls-in-standalone-editor.ts` — Editor module cannot use NLS

### Miscellaneous
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-must-use-result.ts` — Results of certain calls must be consumed
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-limited-top-functions.ts` — Restricts top-level function definitions per file
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/code-no-telemetry-common-property.ts` — Telemetry key naming validation
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/vscode-dts-*.ts` (9 rules) — Public API conventions in `vscode.d.ts`:
  - `vscode-dts-event-naming.ts` — Event names must follow `onX` pattern
  - `vscode-dts-interface-naming.ts` — Interface naming conventions
  - `vscode-dts-use-export.ts` — Export syntax validation
  - `vscode-dts-use-thenable.ts` — Prefers `Thenable<T>` over `Promise<T>` in public API
  - `vscode-dts-create-func.ts` — `create*` factory function conventions
  - `vscode-dts-literal-or-types.ts` — Literal vs union type preferences
  - `vscode-dts-string-type-literals.ts` — String literal type requirements
  - `vscode-dts-cancellation.ts` — `CancellationToken` usage patterns
  - `vscode-dts-provider-naming.ts` — Provider interfaces enforce `provideX` or `resolveX` methods
  - `vscode-dts-vscode-in-comments.ts` — Comments documentation conventions

### Support Files
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/utils.ts` — Shared utilities for import validation; defines `createImportRuleListener` for detecting all import forms (import, dynamic import(), import=, export from)
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/index.ts` — Auto-discovery of all rule modules via glob; exports as ESLint plugin

## Tests
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/tests/code-no-observable-get-in-reactive-context-test.ts` — Fixtures verifying .get() detection in reactive contexts
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/tests/code-no-reader-after-await-test.ts` — Test cases for reader invalidation after await

## Configuration
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/tsconfig.json` — TypeScript configuration for the plugin
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/package.json` — Package metadata; type: module; script: typecheck

## Documentation
- `/Users/norinlavaee/vscode-atomic/.eslint-plugin-local/README.md` — Comprehensive guide to adding custom ESLint rules; explains rule template, selector syntax, fix API, and configuration in main eslint.config.js

## Notable Clusters

### Layer Architecture (3 files, 500+ LOC)
`code-layering.ts`, `code-import-patterns.ts`, `code-no-deep-import-of-internal.ts` — Collectively define and enforce a 7-layer dependency graph:
- `common` (platform-agnostic core)
- `worker` (shared worker code)
- `browser` (browser-specific code with DOM)
- `electron-browser` (Electron main window renderer)
- `node` (Node.js runtime code)
- `electron-utility` (Electron utility process)
- `electron-main` (Electron main process)

This is the **critical layering contract** a Rust port must preserve to avoid architectural regression.

### Startup Performance & Dynamic Loading (4 files, 250+ LOC)
`code-no-static-node-module-import.ts`, `code-amd-node-module.ts`, `code-no-http-import.ts`, `code-no-runtime-import.ts` — Collectively enforce lazy loading of expensive dependencies (http, https, npm packages). These rules protect VS Code's startup performance (critical for Electron) and reflect deep knowledge of module loading costs.

### Lifecycle & Resource Safety (4 files, 400+ LOC)
`code-must-use-super-dispose.ts`, `code-no-potentially-unsafe-disposables.ts`, `code-no-global-document-listener.ts`, `code-no-accessor-after-await.ts`, `code-no-reader-after-await.ts` — Encode the contract that **resources must be managed synchronously and within scope**. A Rust port would need to translate these constraints to Rust's ownership system.

### Reactive Programming (2 files, 300+ LOC)
`code-no-observable-get-in-reactive-context.ts`, `code-no-reader-after-await.ts` — Enforce a reactive dataflow model where `reader` parameters track dependencies; observable `.get()` breaks this model. Reflects VS Code's migration toward fine-grained reactivity.

### Service Injection (2 files, 150+ LOC)
`code-declare-service-brand.ts`, `code-no-accessor-after-await.ts` — Enforce DI patterns using phantom type branding and synchronous-only accessor access. The `ServicesAccessor` validity window is a complex constraint: must be used only synchronously within the function it's injected into.

### vscode.d.ts Public API (10 files, 500+ LOC)
All `vscode-dts-*.ts` rules collectively enforce a coherent public extension API:
- Naming conventions for events (`onX`), providers (`provideX`/`resolveX`)
- Preference for `Thenable<T>` over `Promise<T>` for compatibility
- Event/interface/type literal strictness
- `CancellationToken` patterns

These define what Rust-based language features must expose to the extension ecosystem.

## Summary

The `.eslint-plugin-local/` directory is a **machine-readable architecture specification** for VS Code. The 48 rules encode:

1. **Layering invariants** preventing architectural entropy across 7 application tiers
2. **Performance constraints** on module initialization (startup is critical)
3. **Resource lifecycle requirements** (disposal, accessor validity windows)
4. **Platform boundaries** (Electron main/renderer, browser, Node.js)
5. **Reactive programming contracts** (reader validity, observable tracking)
6. **API design standards** for the public extension interface
7. **Localization and accessibility requirements**

A successful TypeScript-to-Rust port would need to:
- **Preserve the 7-layer dependency graph** exactly or translate it to a Rust module system
- **Implement lazy-loading equivalents** for the Node module restrictions
- **Map the disposable/lifecycle semantics** to Rust's `Drop`, `Arc<Mutex<T>>`, and scope guards
- **Translate `ServicesAccessor` semantics** to Rust's trait bounds and lifetime parameters
- **Adapt reactive programming** constraints to functional composition or a reactive crate (e.g., `futures::Stream`)
- **Maintain all public API contracts** defined in vscode.d.ts conventions

The rules in this directory are load-bearing for understanding what aspects of VS Code's architecture are non-negotiable.
