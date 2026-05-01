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
