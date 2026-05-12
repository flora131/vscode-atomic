# Partition 19 of 80 — Findings

## Scope
`.eslint-plugin-local/` (49 files, 3,952 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# ESLint Plugin Patterns: Architectural Invariants for Tauri/Rust Port

## Research Scope
`.eslint-plugin-local/` (49 files, ~3,952 LOC)

These lint rules encode critical architectural constraints and invariants that the Rust/Tauri port must preserve or reimplements as checks.

---

## Pattern 1: Layered Architecture Enforcement

**Where:** `.eslint-plugin-local/code-layering.ts:37-92`

**What:** Enforces strict directional dependencies between architectural layers. Files can only import from allowed layers defined by their directory location. This prevents lateral imports and maintains clean dependency hierarchy.

```typescript
create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
    const fileDirname = dirname(context.getFilename());
    const parts = fileDirname.split(/\\|\//);
    const ruleArgs = context.options[0] as Record<string, string[]>;
    let config: Config | undefined;
    for (let i = parts.length - 1; i >= 0; i--) {
        if (ruleArgs[parts[i]]) {
            config = {
                allowed: new Set(ruleArgs[parts[i]]).add(parts[i]),
                disallowed: new Set()
            };
            Object.keys(ruleArgs).forEach(key => {
                if (!config!.allowed.has(key)) {
                    config!.disallowed.add(key);
                }
            });
            break;
        }
    }
    return createImportRuleListener((node, path) => {
        const parts = dirname(path).split(/\\|\//);
        for (let i = parts.length - 1; i >= 0; i--) {
            if (config!.allowed.has(parts[i])) {
                break; // GOOD - same layer
            }
            if (config!.disallowed.has(parts[i])) {
                context.report({...}); // BAD - wrong layer
                break;
            }
        }
    });
}
```

**Variations:**
- `code-no-deep-import-of-internal.ts:29-62` — Prevents deep importing of internal modules (marked with internal pattern). Only direct parents can import internals, others must use re-exports.

---

## Pattern 2: Reactive Dependency Tracking (Observable Pattern)

**Where:** `.eslint-plugin-local/code-no-observable-get-in-reactive-context.ts:20-79`

**What:** Enforces proper dependency tracking in reactive contexts. Prohibits `.get()` (untracked reads) on observables inside reactive functions (`derived`, `autorun`). Requires `.read(reader)` to properly track dependencies or `.read(undefined)` for explicit untracked reads.

```typescript
create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
    return {
        'CallExpression': (node: ESTree.CallExpression) => {
            const callExpression = node as TSESTree.CallExpression;
            if (!isReactiveFunctionWithReader(callExpression.callee)) {
                return;
            }
            const functionArg = callExpression.arguments.find(arg =>
                arg.type === 'ArrowFunctionExpression' || arg.type === 'FunctionExpression'
            ) as TSESTree.ArrowFunctionExpression | TSESTree.FunctionExpression | undefined;
            const readerName = getReaderParameterName(functionArg);
            checkFunctionForObservableGetCalls(functionArg, readerName, context);
        }
    };
}
```

**Variations:**
- `code-no-reader-after-await.ts:12-36` — Reactive contexts require reader parameter for dependency tracking. Reader becomes invalid after await, so calls to `.read()` or `.readObservable()` are flagged post-await.

---

## Pattern 3: Service Accessor Lifecycle Management

**Where:** `.eslint-plugin-local/code-no-accessor-after-await.ts:33-71`

**What:** ServicesAccessor is only valid synchronously during function invocation. Detects two patterns:
1. Direct callbacks to `invokeFunction`/`invokeWithinContext` (accessor is first param)
2. Functions with `ServicesAccessor` type annotation (implies runtime invocation via instantiation service)

Flags any use of accessor after `await`, with branch-aware state tracking (if/else, try/catch, switch, for-of isolation).

```typescript
create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
    return {
        'CallExpression': (node: eslint.Rule.Node) => {
            const callExpression = node as unknown as TSESTree.CallExpression;
            if (!isInvokeFunctionCall(callExpression.callee)) {
                return;
            }
            const functionArg = callExpression.arguments.find(arg =>
                arg.type === 'ArrowFunctionExpression' || arg.type === 'FunctionExpression'
            ) as TSESTree.ArrowFunctionExpression | TSESTree.FunctionExpression | undefined;
            const accessorName = getParamName(functionArg.params[0]);
            checkForAccessorAfterAwait(functionArg, accessorName, context);
        },
        'FunctionDeclaration': (node: eslint.Rule.Node) => {
            checkFunctionWithAccessorParam(node as unknown as TSESTree.FunctionDeclaration, context);
        },
    };
}
```

**Variations:**
- Complex control-flow aware analysis: IfStatement (line 159-192), ConditionalExpression (194-203), SwitchStatement (205-217), TryStatement (219-231) with branch isolation.

---

## Pattern 4: Resource Cleanup and Disposable Management

**Where:** `.eslint-plugin-local/code-ensure-no-disposables-leak-in-test.ts:20-43`

**What:** Test suites MUST explicitly call `ensureNoDisposablesAreLeakedInTestSuite()` to verify no disposables leak. Detects `suite(...)` declarations at program level and flags missing cleanup call.

```typescript
create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
    const config = context.options[0] as { exclude: string[] };
    const needle = context.getFilename().replace(/\\/g, '/');
    if (config.exclude.some((e) => needle.endsWith(e))) {
        return {};
    }
    return {
        [`Program > ExpressionStatement > CallExpression[callee.name='suite']`]: (node: estree.Node) => {
            const src = context.getSourceCode().getText(node);
            if (!src.includes('ensureNoDisposablesAreLeakedInTestSuite(')) {
                context.report({
                    node,
                    messageId: 'ensure',
                    fix: (fixer) => {
                        const updatedSrc = src.replace(/(suite\(.*\n)/, '$1\n\tensureNoDisposablesAreLeakedInTestSuite();\n');
                        return fixer.replaceText(node, updatedSrc);
                    }
                });
            }
        },
    };
}
```

**Variations:**
- `code-must-use-super-dispose.ts:12-34` — Override methods named `dispose()` must call `super.dispose()` to ensure cleanup chain is maintained.
- `code-no-potentially-unsafe-disposables.ts:16-38` — DisposableStore/MutableDisposable must use `const` (not `let`) or `readonly` properties to prevent accidental reassignment leaks.

---

## Pattern 5: Controlled Module Import Patterns

**Where:** `.eslint-plugin-local/code-import-patterns.ts:57-279`

**What:** Multi-layered import validation for ES module support and architectural boundaries:
- Enforces relative imports (not absolute paths like `vs/...`)
- Requires `.js` or `.css` extensions for all imports (ESM requirement)
- Validates imports against per-file allowed restriction patterns
- Dynamically generates layer-specific rules from configuration (common → worker → browser → electron-browser, common → node → electron-utility → electron-main)
- Test files have relaxed restrictions

```typescript
create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
    const options = context.options as RawOption[];
    const configs = this._processOptions(options);
    const relativeFilename = getRelativeFilename(context);
    for (const config of configs) {
        if (minimatch(relativeFilename, config.target)) {
            return createImportRuleListener((node, value) => this._checkImport(context, config, node, value));
        }
    }
    context.report({ loc: { line: 1, column: 0 }, messageId: 'badFilename' });
    return {};
}
```

**Variations:**
- `code-amd-node-module.ts` — Validates AMD module declarations against Node.js module compatibility
- `code-no-http-import.ts` — Restricts HTTP imports (prevents unintended network requests)

---

## Pattern 6: Asynchronous Invariant Tracking (Accessor & Reader Invalidation)

**Where:** `.eslint-plugin-local/code-no-accessor-after-await.ts:116-252` (flow-sensitive analysis)

**What:** Sophisticated control-flow analysis that maintains state across conditionals:
- Tracks `sawAwait` state separately through if/else branches
- Properly merges state when branches exit or continue
- Distinguishes cases where both branches exit (code unreachable) vs. one-sided exits
- Handles nested constructs (for-of, try-catch, switch) with appropriate state isolation

```typescript
function checkForAccessorAfterAwait(fn, accessorName, context) {
    let sawAwait = false;
    const visited = new Set<TSESTree.Node>();
    
    function walk(node: TSESTree.Node) {
        if (node.type === 'IfStatement') {
            walk(node.test);
            const beforeBranches = sawAwait;
            
            walk(node.consequent);
            const awaitAfterConsequent = sawAwait;
            const consequentExits = blockAlwaysExits(node.consequent);
            
            sawAwait = beforeBranches;
            if (node.alternate) { walk(node.alternate); }
            const awaitAfterAlternate = sawAwait;
            const alternateExits = node.alternate ? blockAlwaysExits(node.alternate) : false;
            
            if (consequentExits && alternateExits) {
                sawAwait = awaitAfterConsequent || awaitAfterAlternate;
            } else if (consequentExits) {
                sawAwait = awaitAfterAlternate;
            } else if (alternateExits) {
                sawAwait = awaitAfterConsequent;
            } else {
                sawAwait = awaitAfterConsequent || awaitAfterAlternate;
            }
            return;
        }
        // ... similar for ConditionalExpression, SwitchStatement, TryStatement
    }
}
```

---

## Pattern 7: Function Result Usage Enforcement

**Where:** `.eslint-plugin-local/code-must-use-result.ts:20-39`

**What:** Certain function calls have side effects or return significant values that must be consumed. Rule flags calls not in valid contexts:
- Valid: Used in `await` expression or `VariableDeclarator` (assignment)
- Invalid: Standalone expression statements, passed to void functions

```typescript
create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
    const config = context.options[0] as { message: string; functions: string[] }[];
    const listener: eslint.Rule.RuleListener = {};
    
    for (const { message, functions } of config) {
        for (const fn of functions) {
            const query = `CallExpression[callee.property.name='${fn}'], CallExpression[callee.name='${fn}']`;
            listener[query] = (node: ESTree.Node) => {
                const callExpression = node as TSESTree.CallExpression;
                if (!VALID_USES.has(callExpression.parent?.type)) {
                    context.report({ node, message });
                }
            };
        }
    }
    return listener;
}
```

---

## Pattern 8: Service Dependency Declaration Pattern

**Where:** `.eslint-plugin-local/code-declare-service-brand.ts:16-29`

**What:** Service interfaces must declare `_serviceBrand: undefined;` as a private nominal type marker (no value). Enforces service identity separation to prevent accidental interface compatibility.

```typescript
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
```

---

## Summary

These 8 core architectural patterns encode VS Code's critical runtime invariants:

1. **Layering** — Unidirectional dependency flow prevents circular deps
2. **Reactive Tracking** — Observable system requires explicit dependency declaration via reader parameter
3. **Service Lifecycle** — Accessor validity is synchronous-only, invalidated by async
4. **Resource Cleanup** — Disposable chains must be complete; tests must verify no leaks
5. **Module Boundaries** — ESM + layer restrictions enforce clean API surfaces
6. **Flow-Sensitive Analysis** — Async invalidation rules are control-flow aware (not naive)
7. **Side-Effect Enforcement** — Functions with side effects must be explicitly awaited or assigned
8. **Nominal Typing** — Service interfaces use brand markers for structural type safety

A Rust/Tauri port must:
- Implement compile-time or runtime checks for these 8 invariants
- Preserve layer boundaries (possibly through module system + visibility modifiers)
- Enforce reactive pattern (observable/reader equivalent in Rust, possibly with procedural macros)
- Use type system + lifetimes for accessor/reader invalidation
- Implement disposable pattern (RAII + Drop trait for cleanup)
- Module system must support ESM-equivalent relative imports + layer restrictions

**Files analyzed:**
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-layering.ts`
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-no-observable-get-in-reactive-context.ts`
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-no-accessor-after-await.ts`
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-ensure-no-disposables-leak-in-test.ts`
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-import-patterns.ts`
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-must-use-result.ts`
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-declare-service-brand.ts`
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-no-reader-after-await.ts`
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-must-use-super-dispose.ts`
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/code-no-potentially-unsafe-disposables.ts`
- `/home/norinlavaee/projects/vscode-atomic/.eslint-plugin-local/utils.ts`

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
