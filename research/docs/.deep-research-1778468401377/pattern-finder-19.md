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
