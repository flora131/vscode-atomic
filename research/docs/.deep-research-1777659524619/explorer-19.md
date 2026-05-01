# Partition 19 of 79 — Findings

## Scope
`.eslint-plugin-local/` (49 files, 3,952 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
# Analyzer 19: `.eslint-plugin-local/` — VS Code Architectural Invariant Rules

## Files Analysed

1. `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/index.ts`
2. `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/utils.ts`
3. `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-layering.ts`
4. `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-import-patterns.ts`
5. `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-must-use-super-dispose.ts`
6. `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-ensure-no-disposables-leak-in-test.ts`
7. `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-no-potentially-unsafe-disposables.ts`
8. `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-no-reader-after-await.ts`
9. `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-no-observable-get-in-reactive-context.ts`
10. `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-no-accessor-after-await.ts`
11. `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-declare-service-brand.ts`
12. `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/vscode-dts-event-naming.ts`
13. `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/vscode-dts-cancellation.ts`

---

## Per-File Notes

### `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/index.ts`

- **Role:** Plugin entry point. Dynamically discovers and exports all rule modules from the directory.
- **Key symbols:**
  - `rules` (`index.ts:13`) — `Record<string, LooseRuleDefinition>` assembled at load time.
- **Control flow:** `glob.sync()` at line 14 collects all `.ts` files in the directory, filters out `index.ts` and `utils.ts` at line 15, then `require(file).default` at line 17 loads each rule and keys it by base filename (e.g., `code-layering`). The `rules` map is exported at line 20.
- **Data flow:** File system path → base filename string → rule module's `.default` export → `rules` record exported to ESLint config consumers.
- **Dependencies:** `glob` (file discovery), `module.createRequire` (CJS-compatible dynamic loading within ESM), `@typescript-eslint/utils/ts-eslint` (type `LooseRuleDefinition`).

---

### `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/utils.ts`

- **Role:** Shared utility providing `createImportRuleListener`, which normalises all four import-related AST node types into a single callback.
- **Key symbols:**
  - `createImportRuleListener` (`utils.ts:10`) — factory that returns an `eslint.Rule.RuleListener`.
  - `_checkImport` (`utils.ts:12`) — internal helper that guards for `Literal` nodes with string values before invoking the caller-supplied `validateImport`.
- **Control flow:** The returned listener object covers `ImportDeclaration` (line 20), dynamic `import()` via a CSS selector (line 24), `TSImportEqualsDeclaration` (line 28), `ExportAllDeclaration` (line 33), and `ExportNamedDeclaration` (line 37). Each delegates to `_checkImport`, which invokes the caller's `validateImport` callback only if the node is a string literal.
- **Data flow:** ESLint AST node → literal string value → caller-supplied `validateImport(node, value)`.
- **Dependencies:** `eslint`, `estree`, `@typescript-eslint/utils`.

---

### `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-layering.ts`

- **Role:** Enforces VS Code's horizontal layer separation. Given a file's directory path, it determines which architectural layer the file belongs to (by matching the directory name against rule config keys), computes allowed and disallowed layers, then flags any import whose resolved path traverses into a disallowed layer.
- **Key symbols:**
  - `Config` (`code-layering.ts:10`) — `{ allowed: Set<string>; disallowed: Set<string> }`.
  - `create()` (`code-layering.ts:37`) — main rule factory.
  - `'layerbreaker'` message (`code-layering.ts:19`) — error message with `{{from}}` and `{{allowed}}` placeholders.
- **Control flow:** At rule creation time, `context.getFilename()` is split into directory parts (line 40). Walking from the innermost part outward (line 43), the first match against `ruleArgs` keys determines the current layer's `Config`. `createImportRuleListener` at line 63 hooks all import forms. For each import, the import path's directory is similarly walked; if a `disallowed` layer name is found before an `allowed` one, a `layerbreaker` violation is reported (line 79).
- **Data flow:** Source file path → layer name extraction → `Config` computation → import paths → layer name extraction → allow/disallow verdict → optional ESLint report.
- **Dependencies:** `eslint`, `path`, `./utils.ts` (`createImportRuleListener`). Config is supplied by `eslint.config.js` (out-of-partition).

---

### `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-import-patterns.ts`

- **Role:** Enforces which modules a given source file is permitted to import, and enforces ESM hygiene (relative paths with `.js` or `.css` extension). Understands the six named runtime layers (`common`, `worker`, `browser`, `electron-browser`, `node`, `electron-utility`, `electron-main`) and whether the target layer has browser, Node, or Electron capabilities.
- **Key symbols:**
  - `RawImportPatternsConfig` (`code-import-patterns.ts:19`) — per-target configuration object specifying `layer`, `test`, and `restrictions`.
  - `LayerAllowRule` (`code-import-patterns.ts:26`) — conditional allow list (`when: 'hasBrowser' | 'hasNode' | 'hasElectron' | 'test'`).
  - `layerRules` (`code-import-patterns.ts:97`) — static array defining layer capability flags and dependency patterns.
  - `_processOptions()` (`code-import-patterns.ts:78`) — builds `ImportPatternsConfig[]` from raw ESLint options, memoised in a `WeakMap` (line 76).
  - `_checkImport()` (`code-import-patterns.ts:227`) — validates individual import paths; reports `badExtension` (line 234), `badAbsolute` (line 241), or `badImport` (line 271).
  - `getRelativeFilename()` (`code-import-patterns.ts:284`) — strips `REPO_ROOT` from the absolute context filename.
- **Control flow:** `create()` calls `_processOptions()` to materialise configs from rule options, then uses `minimatch` to match the current file against config `target` patterns (line 63). The matching config's `createImportRuleListener` is returned; for `src/vs/` files, ESM extension and absolute-path rules are also checked. The `/~/` placeholder in target and restriction patterns is expanded to concrete layer path segments by `generateConfig()` at line 134.
- **Data flow:** ESLint options → `_processOptions()` memoised → `ImportPatternsConfig[]` → per-file minimatch → listener → each import literal → restriction minimatch loop → optional report.
- **Dependencies:** `eslint`, `@typescript-eslint/utils`, `path`, `minimatch`, `./utils.ts`.

---

### `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-must-use-super-dispose.ts`

- **Role:** Enforces that any class `dispose()` method declared with the `override` keyword must call `super.dispose()`.
- **Key symbols:**
  - `doesCallSuperDispose()` (`code-must-use-super-dispose.ts:13`) — visitor function.
  - AST selector `'MethodDefinition[override][key.name="dispose"]'` (`code-must-use-super-dispose.ts:32`) — targets only override dispose methods.
- **Control flow:** For each matched `MethodDefinition` node, the rule grabs the full source text of the method body via `context.getSourceCode().getText()` (line 19) and performs a simple substring check for `'super.dispose'` (line 21). If absent, a violation is reported at the method node.
- **Data flow:** AST `MethodDefinition` node → raw source text → substring search → optional report.
- **Dependencies:** `@typescript-eslint/utils`, `eslint`, `estree`.

---

### `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-ensure-no-disposables-leak-in-test.ts`

- **Role:** Ensures every top-level `suite(...)` call in test files includes `ensureNoDisposablesAreLeakedInTestSuite()` to detect disposable leaks at test teardown.
- **Key symbols:**
  - `EnsureNoDisposablesAreLeakedInTestSuite` (`code-ensure-no-disposables-leak-in-test.ts:9`) — rule class.
  - Selector `Program > ExpressionStatement > CallExpression[callee.name='suite']` (`code-ensure-no-disposables-leak-in-test.ts:29`).
  - Auto-fix at line 36: inserts `ensureNoDisposablesAreLeakedInTestSuite();` as the first statement of the suite callback.
- **Control flow:** The rule first checks a configurable `exclude` list (line 24) to skip exempt test files. For matched `suite()` calls it reads the entire call's source text and does a substring search for `'ensureNoDisposablesAreLeakedInTestSuite('` (line 31). If absent, it reports with an auto-fix that injects the call.
- **Data flow:** File path → exclude check → AST suite call → source text substring search → optional report + fix.
- **Dependencies:** `eslint`, `estree`.

---

### `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-no-potentially-unsafe-disposables.ts`

- **Role:** Prevents `DisposableStore` and `MutableDisposable` from being declared as mutable (non-`const`, non-`readonly`) bindings, which have historically caused disposable leaks through accidental reassignment.
- **Key symbols:**
  - Three AST selectors at lines 32–35: `VariableDeclaration[kind!="const"]` with `NewExpression[callee.name="DisposableStore"]`, and two `PropertyDefinition[readonly!=true]` selectors for typed properties or direct instantiation.
- **Control flow:** Each matched node directly calls the `checkVariableDeclaration` or `checkProperty` reporter with a descriptive message. No transitive traversal is performed.
- **Data flow:** AST pattern match → report.
- **Dependencies:** `eslint`, `estree`.

---

### `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-no-reader-after-await.ts`

- **Role:** Enforces that observable `reader` objects (the first parameter of reactive functions `derived`, `autorun`, etc.) are not used after an `await` expression, since the reader is invalid after async suspension.
- **Key symbols:**
  - `NoReaderAfterAwait` (`code-no-reader-after-await.ts:10`) — rule class.
  - `readerFunctions` (`code-no-reader-after-await.ts:150`) — `Set` of reactive function names: `derived`, `autorun`, `autorunOpts`, `autorunHandleChanges`, `autorunSelfDisposable`.
  - `checkFunctionForAwaitBeforeReader()` (`code-no-reader-after-await.ts:39`) — traverses the function body collecting `AwaitExpression` positions, then flags any subsequent `reader.read()` or `observable.read(reader)` call.
  - `isReaderMethodCall()` (`code-no-reader-after-await.ts:129`) — matches both `reader.read()`/`reader.readObservable()` and `obs.read(reader)`/`obs.readObservable(reader)` patterns.
- **Control flow:** The `CallExpression` visitor checks whether the callee is in `readerFunctions` (line 16), finds the function argument (line 20), extracts the reader parameter name (line 28), then delegates to `checkFunctionForAwaitBeforeReader`. That function walks the AST linearly, accumulating `awaitPositions` (line 44); whenever a reader method call is found with non-empty `awaitPositions`, a violation is reported (line 60).
- **Data flow:** `CallExpression` → callee name check → function arg → reader param name → linear AST walk accumulating await positions → reader call detection → optional report.
- **Dependencies:** `@typescript-eslint/utils`, `eslint`, `estree`.

---

### `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-no-observable-get-in-reactive-context.ts`

- **Role:** Disallows calling `.get()` (zero-argument) on observables inside reactive callbacks (`derived`, `autorun`, and all variants). Enforces use of `.read(reader)` or `.read(undefined)` instead, to ensure proper dependency tracking.
- **Key symbols:**
  - `NoObservableGetInReactiveContext` (`code-no-observable-get-in-reactive-context.ts:11`) — rule class; marked `fixable: 'code'` (line 18).
  - `reactiveFunctions` (`code-no-observable-get-in-reactive-context.ts:94`) — 14-element `Set` covering all reactive primitives.
  - `checkFunctionForObservableGetCalls()` (`code-no-observable-get-in-reactive-context.ts:48`) — full AST traversal via `walkChildren` from `eslint-visitor-keys`.
  - `isObservableGetCall()` (`code-no-observable-get-in-reactive-context.ts:81`) — matches `something.get()` with zero arguments.
  - Auto-fix at line 67: replaces `obs.get()` with `obs.read(undefined)`.
- **Control flow:** `CallExpression` visitor checks callee against `reactiveFunctions` (line 25), extracts reader parameter name (line 37), then calls `checkFunctionForObservableGetCalls`. That function uses `walkChildren` (powered by `eslint-visitor-keys` at line 8) for a full child traversal, unlike `code-no-reader-after-await` which uses a manual switch. Each `.get()` call inside the reactive body is reported with a fixer.
- **Data flow:** Reactive `CallExpression` → function arg → reader name → full child traversal → `.get()` detection → report + auto-fix.
- **Dependencies:** `@typescript-eslint/utils`, `eslint`, `eslint-visitor-keys`, `estree`.

---

### `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-no-accessor-after-await.ts`

- **Role:** Prevents `ServicesAccessor` (the DI container handle from `IInstantiationService.invokeFunction`) from being used after an `await` expression. The accessor is synchronously scoped and becomes invalid after any async suspension point.
- **Key symbols:**
  - `NoAccessorAfterAwait` (`code-no-accessor-after-await.ts:24`) — rule class.
  - `invokeFunctionNames` (`code-no-accessor-after-await.ts:407`) — `Set{'invokeFunction', 'invokeWithinContext'}`.
  - `checkForAccessorAfterAwait()` (`code-no-accessor-after-await.ts:116`) — branch-aware AST walk with `sawAwait: boolean` state variable; models `IfStatement`, `ConditionalExpression`, `SwitchStatement`, `TryStatement`, and `for await...of` semantics separately.
  - `blockAlwaysExits()` (`code-no-accessor-after-await.ts:260`) — detects unconditional `return`/`throw` to prune dead code paths.
  - `hasServicesAccessorAnnotation()` (`code-no-accessor-after-await.ts:104`) — inspects TypeScript type annotation to detect `ServicesAccessor` typed parameters.
  - `isAccessorUsage()` (`code-no-accessor-after-await.ts:278`) — detects `accessor.get(...)` and passing accessor as call argument.
- **Control flow:** Two detection strategies: (1) `CallExpression` visitor matching `invokeFunction`/`invokeWithinContext`, extracting the callback's first parameter as the accessor name; (2) `FunctionDeclaration`/`FunctionExpression`/`ArrowFunctionExpression` visitors inspecting TypeScript type annotations for `ServicesAccessor`. Strategy 2 skips functions already handled by strategy 1 via `isDirectInvokeFunctionCallback()` (line 94). Both paths call `checkForAccessorAfterAwait()`, which walks the function body with a mutable `sawAwait` boolean, taking care to isolate await state across branches and only propagate it along reachable paths.
- **Data flow:** Function node → accessor name identification → branch-aware sequential AST walk → `sawAwait` state accumulation → accessor usage detection post-await → report.
- **Dependencies:** `@typescript-eslint/utils`, `eslint`.

---

### `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-declare-service-brand.ts`

- **Role:** Enforces that the `_serviceBrand` property — VS Code's nominal typing marker used to brand service interfaces for the DI container — is declared as `declare _serviceBrand: undefined;` with no initializer value.
- **Key symbols:**
  - `DeclareServiceBrand` (`code-declare-service-brand.ts:9`) — rule class; `fixable: 'code'` (line 12).
  - Selector `PropertyDefinition[key.name="_serviceBrand"][value]` (`code-declare-service-brand.ts:18`) — targets any `_serviceBrand` property that has a value.
  - Auto-fix at line 22: replaces the entire node with the canonical `declare _serviceBrand: undefined;`.
- **Control flow:** A single AST selector matches `_serviceBrand` properties with an initializer and immediately reports a violation with a fixer. No traversal logic needed.
- **Data flow:** `PropertyDefinition` AST match → report + replace text fix.
- **Dependencies:** `eslint`, `estree`.

---

### `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/vscode-dts-event-naming.ts`

- **Role:** Enforces the VS Code extension API event naming convention: event-typed properties/variables must follow `on[Did|Will]<Verb><Subject>`, with verbs drawn from an approved configured list and subjects verified to appear elsewhere in the API surface.
- **Key symbols:**
  - `ApiEventNaming` (`vscode-dts-event-naming.ts:10`) — rule class.
  - `_nameRegExp` (`vscode-dts-event-naming.ts:12`) — `/on(Did|Will)([A-Z][a-z]+)([A-Z][a-z]+)?/` pattern.
  - `getIdent()` (`vscode-dts-event-naming.ts:86`) — recursive upward AST traversal to find the identifier attached to an `Event<T>` type annotation.
- **Control flow:** Selector `TSTypeAnnotation TSTypeReference Identifier[name="Event"]` (line 34) fires for every `Event<T>` typed declaration. `getIdent()` walks parent chain to reach the declaring identifier. The rule first checks an `allowed` exception set (line 47), then validates against `_nameRegExp` (line 52). If matched, it checks the verb against the configured `verbs` set (line 62) and verifies the subject string appears at least twice in the file source (line 72–78) to confirm the subject is a real API concept.
- **Data flow:** `Event<T>` type reference → parent chain walk → identifier name → regex match → verb set lookup → subject occurrence count → optional reports.
- **Dependencies:** `eslint`, `estree`, `@typescript-eslint/utils`. Config (`allowed`, `verbs`) from external ESLint config file.

---

### `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/vscode-dts-cancellation.ts`

- **Role:** Enforces that provider interfaces (`*Provider`) have a `token` parameter in all `provide*` and `resolve*` methods, documenting VS Code's invariant that all async provider operations must be cancellable.
- **Key symbols:**
  - `ApiProviderNaming` (`vscode-dts-cancellation.ts:9`) — rule class (named `ApiProviderNaming` in source but file is `vscode-dts-cancellation.ts`).
  - Selector `TSInterfaceDeclaration[id.name=/.+Provider/] TSMethodSignature[key.name=/^(provide|resolve).+/]` (`vscode-dts-cancellation.ts:21`).
- **Control flow:** For each matched method signature, iterates over all parameters looking for an `Identifier` named `token` (line 24–28). If not found, reports `noToken` (line 31).
- **Data flow:** Interface method signature → parameter name scan → optional report.
- **Dependencies:** `@typescript-eslint/utils`, `eslint`.

---

## Cross-Cutting Synthesis

The `.eslint-plugin-local/` plugin is VS Code's machine-executable specification of its runtime invariants, organized across four domains. **Layering invariants** (`code-layering.ts`, `code-import-patterns.ts`) form the most structurally complex rules: `code-import-patterns.ts` encodes the full seven-layer dependency lattice (`common → worker → browser → electron-browser` and `common → node → electron-utility → electron-main`) via its `layerRules` table and the `/~/` pattern expansion mechanism; `code-layering.ts` adds a coarser directory-name-based guard enforcing the same hierarchy at a higher abstraction. **Disposable lifecycle invariants** (`code-must-use-super-dispose.ts`, `code-ensure-no-disposables-leak-in-test.ts`, `code-no-potentially-unsafe-disposables.ts`) collectively enforce the three axes of VS Code's `IDisposable` contract: inheritance chain completeness, test-time leak detection, and mutation-safety of disposable container bindings. **Reactive/async safety invariants** (`code-no-reader-after-await.ts`, `code-no-observable-get-in-reactive-context.ts`, `code-no-accessor-after-await.ts`) protect three co-temporal scoped handles — observable readers, reactive derivation contexts, and `ServicesAccessor` — against use after the async boundary that invalidates them; `code-no-accessor-after-await.ts` is the most sophisticated, implementing a branch-aware dataflow analysis with separate `sawAwait` state for `if`/`switch`/`try`/`for-await` constructs. **API contract invariants** (`code-declare-service-brand.ts`, `vscode-dts-event-naming.ts`, `vscode-dts-cancellation.ts`) encode the nominal typing discipline for the DI container (`_serviceBrand`), the extension event naming grammar (`on[Did|Will]<Verb><Subject>`), and the universal cancellation requirement for provider methods. Together these rules document the precise invariants a Tauri/Rust port must replicate structurally: a strictly stratified module system, RAII-style scope ownership with deterministic cleanup, a reactive primitives system with synchronous-read semantics, a synchronously-scoped DI accessor, and a public API naming and cancellation contract.

---

## Out-of-Partition References

- **`eslint.config.js`** (repo root, outside partition) — supplies all rule options: layer configs for `code-layering`, target/restriction matrices for `code-import-patterns`, `allowed`/`verbs` lists for `vscode-dts-event-naming`, and `exclude` lists for `code-ensure-no-disposables-leak-in-test`. The rules are stateless without this config.
- **`src/vs/base/common/lifecycle.ts`** — defines `IDisposable`, `DisposableStore`, `MutableDisposable`, and `Disposable` base class (with the canonical `super.dispose()` call chain). The three disposable rules are guards against misuse of these types.
- **`src/vs/base/common/observable.ts`** (or the observable subsystem) — defines `derived`, `autorun`, and all reactive primitives named in the `readerFunctions`/`reactiveFunctions` sets; defines the `IReader` interface whose `.read()` method is the approved reactive-context accessor.
- **`src/vs/platform/instantiation/common/instantiation.ts`** — defines `ServicesAccessor`, `IInstantiationService`, `invokeFunction`, and the `_serviceBrand` DI marker. The `code-declare-service-brand.ts` and `code-no-accessor-after-await.ts` rules are both guards on this DI system's contracts.
- **`src/vscode-dts/vscode.d.ts`** — the extension API declaration file; all `vscode-dts-*` rules apply exclusively to this file or files matching its pattern, enforcing naming, cancellation, and typing conventions in the public API surface.
- **`src/vs/base/test/common/utils.ts`** — expected to export `ensureNoDisposablesAreLeakedInTestSuite()`, the function that `code-ensure-no-disposables-leak-in-test.ts` requires to be present in every test suite.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Lint Rule Patterns for VS Code Core Porting

## Overview

The `.eslint-plugin-local/` directory contains 49 TypeScript files implementing ESLint rules that encode fundamental VS Code architectural invariants. These rules enforce critical design patterns that would need semantic equivalents when porting the codebase from TypeScript/Electron to Tauri/Rust. The patterns fall into six key categories: layering constraints, resource lifecycle management, unsafe patterns, async correctness, naming conventions, and internationalization.

---

## Pattern: Layering Enforcement

**Where:** `.eslint-plugin-local/code-layering.ts:15-92`

**What:** Validates that imports across modules respect a layered architecture, preventing upward dependencies that would violate separation of concerns.

```typescript
export default new class implements eslint.Rule.RuleModule {

	readonly meta: eslint.Rule.RuleMetaData = {
		messages: {
			layerbreaker: 'Bad layering. You are not allowed to access {{from}} from here, allowed layers are: [{{allowed}}]'
		},
		docs: {
			url: 'https://github.com/microsoft/vscode/wiki/Source-Code-Organization'
		},
		schema: [
			{
				type: 'object',
				additionalProperties: {
					type: 'array',
					items: { type: 'string' }
				}
			}
		]
	};

	create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
		// Layer configuration loaded from context.options
		// Validates that imports from current file resolve to allowed layers only
		// Uses directory path components to determine current layer
		return createImportRuleListener((node, path) => {
			// Check if imported path belongs to disallowed layer
			// Report layerbreaker if violation detected
		});
	}
};
```

**Variations / call-sites:**
- `.eslint-plugin-local/code-import-patterns.ts:57-279` — More sophisticated variant that handles multi-platform constraints (browser/node/electron) and test-specific rules; uses minimatch patterns and caching for complex layer generation across 7 architecture layers.

**Rust Porting Implication:** Layering must be enforced at module level. Rust's module system and visibility (pub/crate/private) can implement coarse-grained constraints, but a procedural build-time check (like a custom build.rs script) would be needed to enforce cross-layer import restrictions.

---

## Pattern: Disposable Lifecycle Enforcement

**Where:** `.eslint-plugin-local/code-no-potentially-unsafe-disposables.ts:14-38`

**What:** Prevents unsafe reassignment of critical resource containers (`DisposableStore`, `MutableDisposable`) that are sources of leaks; enforces `const` and `readonly` modifiers.

```typescript
export default new class implements eslint.Rule.RuleModule {

	create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
		function checkVariableDeclaration(inNode: ESTree.Node) {
			context.report({
				node: inNode,
				message: `Use const for 'DisposableStore' to avoid leaks by accidental reassignment.`
			});
		}

		function checkProperty(inNode: ESTree.Node) {
			context.report({
				node: inNode,
				message: `Use readonly for DisposableStore/MutableDisposable to avoid leaks through accidental reassignment.`
			});
		}

		return {
			'VariableDeclaration[kind!="const"] > VariableDeclarator > NewExpression[callee.name="DisposableStore"]': checkVariableDeclaration,
			'PropertyDefinition[readonly!=true][typeAnnotation.typeAnnotation.typeName.name=/DisposableStore|MutableDisposable/]': checkProperty,
			'PropertyDefinition[readonly!=true] > NewExpression[callee.name=/DisposableStore|MutableDisposable/]': checkProperty,
		};
	}
};
```

**Variations / call-sites:**
- `.eslint-plugin-local/code-must-use-super-dispose.ts:10-35` — Enforces that overridden `dispose()` methods call `super.dispose()` to prevent resource leak cascades.
- `.eslint-plugin-local/code-ensure-no-disposables-leak-in-test.ts:9-44` — Test-specific variant requiring explicit `ensureNoDisposablesAreLeakedInTestSuite()` calls in suite setup.

**Rust Porting Implication:** Rust's ownership and borrow checker eliminate many leak patterns at compile time. However, Drop trait implementations and RAII patterns must strictly enforce hierarchical cleanup. A custom derive macro or clippy lint could enforce that composite types call drop on all member fields.

---

## Pattern: Async Context Corruption Detection

**Where:** `.eslint-plugin-local/code-no-reader-after-await.ts:10-169`

**What:** Prevents use of reactive computation contexts (reader objects) after `await`, where the context becomes invalid; implements branch-aware control flow analysis.

```typescript
export default new class NoReaderAfterAwait implements eslint.Rule.RuleModule {
	create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
		return {
			'CallExpression': (node: ESTree.CallExpression) => {
				const callExpression = node as TSESTree.CallExpression;

				if (!isFunctionWithReader(callExpression.callee)) {
					return;
				}

				const functionArg = callExpression.arguments.find(arg =>
					arg.type === 'ArrowFunctionExpression' || arg.type === 'FunctionExpression'
				) as TSESTree.ArrowFunctionExpression | TSESTree.FunctionExpression | undefined;

				if (!functionArg) { return; }

				const readerName = getReaderParameterName(functionArg);
				if (!readerName) { return; }

				checkFunctionForAwaitBeforeReader(functionArg, readerName, context);
			}
		};
	}
};

function checkFunctionForAwaitBeforeReader(
	fn: TSESTree.ArrowFunctionExpression | TSESTree.FunctionExpression,
	readerName: string,
	context: eslint.Rule.RuleContext
) {
	const awaitPositions: { line: number; column: number }[] = [];
	const visited = new Set<TSESTree.Node>();
	
	function collectPositions(node: TSESTree.Node) {
		if (visited.has(node)) { return; }
		visited.add(node);
		
		if (node.type === 'AwaitExpression') {
			awaitPositions.push({
				line: node.loc?.start.line || 0,
				column: node.loc?.start.column || 0
			});
		} else if (node.type === 'CallExpression' && isReaderMethodCall(node, readerName)) {
			if (awaitPositions.length > 0) {
				const methodName = getMethodName(node);
				context.report({
					node: node,
					message: `Reader method '${methodName}' should not be called after 'await'.`
				});
			}
		}
		// [... extensive switch for all AST node types ...]
	}
	
	if (fn.body) {
		collectPositions(fn.body);
	}
}

const readerFunctions = new Set(['derived', 'autorun', 'autorunOpts', 'autorunHandleChanges', 'autorunSelfDisposable']);
```

**Variations / call-sites:**
- `.eslint-plugin-local/code-no-accessor-after-await.ts:24-421` — Parallel pattern for `ServicesAccessor` (dependency injection accessor); implements sophisticated branch-aware analysis with special handling for if-statements, switch, try-catch, and for-await loops; includes deduplication and parameter type annotation checking.

**Rust Porting Implication:** Rust's borrow checker and lifetime system make these patterns less likely (borrows cannot outlive their source), but custom Tokio-based async code needs similar analysis. A runtime panic/assertion or async-aware clippy lint could detect accessor use after invalidation if wrapped in a scoped guard type.

---

## Pattern: Service Brand Enforcement

**Where:** `.eslint-plugin-local/code-declare-service-brand.ts:9-29`

**What:** Enforces service interface contracts by requiring `_serviceBrand` as a declaration-only property; auto-fixes incorrect definitions.

```typescript
export default new class DeclareServiceBrand implements eslint.Rule.RuleModule {

	readonly meta: eslint.Rule.RuleMetaData = {
		fixable: 'code',
		schema: false,
	};

	create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
		return {
			['PropertyDefinition[key.name="_serviceBrand"][value]']: (node: ESTree.PropertyDefinition) => {
				return context.report({
					node,
					message: `The '_serviceBrand'-property should not have a value`,
					fix: (fixer) => {
						return fixer.replaceText(node, 'declare _serviceBrand: undefined;');
					}
				});
			}
		};
	}
};
```

**Variations / call-sites:** No direct variations found; this rule stands alone as a contract enforcement mechanism specific to VS Code's branded-type pattern.

**Rust Porting Implication:** Rust's type system using newtype/phantom types (`struct ServiceBrand; impl Service for MyImpl {}`) provides compile-time guarantees without runtime markers. The _serviceBrand pattern could translate to marker traits with compile-time verification via the type system rather than runtime checks.

---

## Pattern: Function Declaration Restrictions

**Where:** `.eslint-plugin-local/code-limited-top-functions.ts:11-71`

**What:** Restricts top-level function declarations in specific modules to an allowlist; prevents proliferation of module-level exports via pattern matching on file paths.

```typescript
export default new class implements eslint.Rule.RuleModule {

	readonly meta: eslint.Rule.RuleMetaData = {
		messages: {
			layerbreaker: 'You are only allowed to define limited top level functions.'
		},
		schema: {
			type: 'array',
			items: {
				type: 'object',
				additionalProperties: {
					type: 'array',
					items: { type: 'string' }
				}
			}
		}
	};

	create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
		let fileRelativePath = relative(dirname(import.meta.dirname), context.getFilename());
		if (!fileRelativePath.endsWith('/')) {
			fileRelativePath += '/';
		}
		const ruleArgs = context.options[0] as Record<string, string[]>;

		const matchingKey = Object.keys(ruleArgs).find(key => 
			fileRelativePath.startsWith(key) || minimatch(fileRelativePath, key));
		if (!matchingKey) { return {}; }

		const restrictedFunctions = ruleArgs[matchingKey];

		return {
			FunctionDeclaration: (node: ESTree.FunctionDeclaration) => {
				const isTopLevel = node.parent?.type === 'Program';
				const functionName = node.id.name;
				if (isTopLevel && !restrictedFunctions.includes(node.id.name)) {
					context.report({
						node,
						message: `Top-level function '${functionName}' is restricted.`
					});
				}
			},
			ExportNamedDeclaration(node: ESTree.ExportNamedDeclaration) {
				if (node.declaration && node.declaration.type === 'FunctionDeclaration') {
					const functionName = node.declaration.id.name;
					const isTopLevel = node.parent?.type === 'Program';
					if (isTopLevel && !restrictedFunctions.includes(node.declaration.id.name)) {
						context.report({
							node,
							message: `Top-level function '${functionName}' is restricted.`
						});
					}
				}
			}
		};
	}
};
```

**Variations / call-sites:** No variants; this rule enforces a single architectural constraint (module-level API surface).

**Rust Porting Implication:** Rust module system and pub visibility can express API boundaries declaratively. A custom build.rs script or cargo-deny configuration could enforce allowlist policies on module exports.

---

## Pattern: Import Rule Listener Abstraction

**Where:** `.eslint-plugin-local/utils.ts:10-41`

**What:** Shared utility implementing a generic rule listener for all import statement types (import/export/dynamic); allows plugin rules to inspect all module dependencies uniformly.

```typescript
export function createImportRuleListener(validateImport: (node: TSESTree.Literal, value: string) => any): eslint.Rule.RuleListener {

	function _checkImport(node: TSESTree.Node | null) {
		if (node && node.type === 'Literal' && typeof node.value === 'string') {
			validateImport(node, node.value);
		}
	}

	return {
		// import ??? from 'module'
		ImportDeclaration: (node: ESTree.ImportDeclaration) => {
			_checkImport((node as TSESTree.ImportDeclaration).source);
		},
		// import('module').then(...) OR await import('module')
		['CallExpression[callee.type="Import"][arguments.length=1] > Literal']: (node: TSESTree.Literal) => {
			_checkImport(node);
		},
		// import foo = ...
		['TSImportEqualsDeclaration > TSExternalModuleReference > Literal']: (node: TSESTree.Literal) => {
			_checkImport(node);
		},
		// export ?? from 'module'
		ExportAllDeclaration: (node: ESTree.ExportAllDeclaration) => {
			_checkImport((node as TSESTree.ExportAllDeclaration).source);
		},
		// export {foo} from 'module'
		ExportNamedDeclaration: (node: ESTree.ExportNamedDeclaration) => {
			_checkImport((node as TSESTree.ExportNamedDeclaration).source);
		},
	};
}
```

**Variations / call-sites:**
- Used by `.eslint-plugin-local/code-layering.ts:63`
- Used by `.eslint-plugin-local/code-import-patterns.ts:64`

**Rust Porting Implication:** Rust's module system is declarative (not statement-based), so a similar analysis would require parsing Cargo.toml dependencies and use statements. A procedural macro or build-time script analyzing the AST of mod declarations could achieve equivalent coverage.

---

## Architectural Invariants Requiring Semantic Preservation

Based on the rule analysis, porting VS Code to Rust/Tauri requires preserving these semantic guarantees:

1. **Layering**: Prevent upward dependencies across 7+ architectural layers (common, worker, browser, electron-browser, node, electron-utility, electron-main).

2. **Disposable Safety**: Enforce lifecycle correctness; async contexts must not access invalidated resources; hierarchical cleanup must be strict.

3. **Service Contracts**: Interface implementations must be statically verifiable (phantom/marker types in Rust).

4. **API Surface Control**: Module exports should be restricted to curated allowlists to prevent accidental public APIs.

5. **Async Correctness**: Context switches (await in JS, .await in async Rust blocks) invalidate certain capabilities; borrowing rules must prevent their use.

The rules themselves are TypeScript-specific (ESLint AST), but their underlying intent—preventing architectural corruption—must be re-encoded using Rust's type system, module visibility, compile-time checks (via build.rs/procedural macros), and runtime assertions where static guarantees are insufficient.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
