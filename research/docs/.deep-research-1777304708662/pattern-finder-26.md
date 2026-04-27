# ESLint Configuration - Architectural Patterns in VS Code

## Summary

The ESLint configuration (`eslint.config.js` and `.eslint-plugin-local/`) defines critical architectural patterns that enforce VS Code's modular structure across six distinct layers. This layering system is fundamental to the codebase organization and would be a key constraint for any port to Tauri/Rust. The configuration also enforces resource management (disposables), module isolation, localization, and platform-specific import restrictions.

---

## Distinct Patterns Identified

#### Pattern 1: Layer-Based Architectural Boundary Enforcement
**Where:** `eslint.config.js:99-123`
**What:** Core architectural constraint defining six importable layers that prevent circular dependencies and enforce separation of concerns.

```javascript
'local/code-layering': [
  'warn',
  {
    'common': [],
    'node': [
      'common'
    ],
    'browser': [
      'common'
    ],
    'electron-browser': [
      'common',
      'browser'
    ],
    'electron-utility': [
      'common',
      'node'
    ],
    'electron-main': [
      'common',
      'node',
      'electron-utility'
    ]
  }
]
```

**Implementation:** `.eslint-plugin-local/code-layering.ts:15-92`
- Inspects import statements to verify that files in one layer only import from allowed predecessor layers
- Layers are identified by directory names (`/common/`, `/browser/`, `/node/`, etc.)
- Prevents layer-breaker violations with linting errors
- Core algorithm: walks the import path's directory hierarchy and checks if it matches allowed or disallowed layer names

**Variations / call-sites:**
- ESLint rule enforcement in all TypeScript files (`src/**/*.ts`)
- Referenced in docs: `https://github.com/microsoft/vscode/wiki/Source-Code-Organization`

---

#### Pattern 2: Conditional Import Restrictions by Platform
**Where:** `.eslint-plugin-local/code-import-patterns.ts:14-30`
**What:** Runtime platform conditions (browser, node, electron) drive conditional import allowlists.

```typescript
interface ConditionalPattern {
  when?: 'hasBrowser' | 'hasNode' | 'hasElectron' | 'test';
  pattern: string;
}

interface RawImportPatternsConfig {
  target: string;
  layer?: 'common' | 'worker' | 'browser' | 'electron-browser' | 'node' | 'electron-utility' | 'electron-main';
  test?: boolean;
  restrictions: string | (string | ConditionalPattern)[];
}
```

**Enforces:** Different import restrictions per file glob pattern, with runtime conditions determining which imports are allowed.

---

#### Pattern 3: Disposable Resource Management Invariants
**Where:** `.eslint-plugin-local/code-must-use-super-dispose.ts:10-35`
**What:** Enforces the invariant that overridden `dispose()` methods must always call `super.dispose()`.

```typescript
create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
  function doesCallSuperDispose(node: TSESTree.MethodDefinition) {
    if (!node.override) {
      return;
    }

    const body = context.getSourceCode().getText(node as ESTree.Node);

    if (body.includes('super.dispose')) {
      return;
    }

    context.report({
      node,
      message: 'dispose() should call super.dispose()'
    });
  }

  return {
    ['MethodDefinition[override][key.name="dispose"]']: doesCallSuperDispose,
  };
}
```

**Key aspect:** Uses AST selector `MethodDefinition[override][key.name="dispose"]` to target only override methods named `dispose`.

---

#### Pattern 4: Service Brand Declaration Convention
**Where:** `.eslint-plugin-local/code-declare-service-brand.ts:16-26`
**What:** Services must declare a `_serviceBrand` type marker without initialization (declarative, not executable).

```typescript
['PropertyDefinition[key.name="_serviceBrand"][value]']: (node: ESTree.PropertyDefinition) => {
  return context.report({
    node,
    message: `The '_serviceBrand'-property should not have a value`,
    fix: (fixer) => {
      return fixer.replaceText(node, 'declare _serviceBrand: undefined;');
    }
  });
}
```

**Enforced rule:** `eslint.config.js:89` — `'local/code-declare-service-brand': 'warn'`

---

#### Pattern 5: HTTP Module Load-Time Restrictions
**Where:** `.eslint-plugin-local/code-no-http-import.ts:38-77`
**What:** Restricts static imports of slow Node.js modules (`http`, `https`) to type-only imports, requiring dynamic import for runtime use.

```typescript
return createImportRuleListener((node, path) => {
  if (!restrictedModules.has(path)) {
    return;
  }

  const parent = node.parent;
  if (!parent) {
    return;
  }

  // Allow: import type { ... } from 'http'
  // Allow: import type * as http from 'http'
  if (parent.type === TSESTree.AST_NODE_TYPES.ImportDeclaration && parent.importKind === 'type') {
    return;
  }

  // Allow: export type { ... } from 'http'
  if ('exportKind' in parent && parent.exportKind === 'type') {
    return;
  }

  context.report({
    loc: parent.loc,
    messageId: 'notAllowed',
    data: { module: path }
  });
});
```

**Applied rule:** `eslint.config.js:97` — `'local/code-no-http-import': ['warn', { target: 'src/vs/**' }]`

---

#### Pattern 6: Externalizable String Enforcement
**Where:** `.eslint-plugin-local/code-no-unexternalized-strings.ts:27-50`
**What:** User-facing strings must be marked with double-quotes and passed to localization functions; single-quoted strings are forbidden for UI text.

```typescript
export default new class NoUnexternalizedStrings implements eslint.Rule.RuleModule {

  private static _rNlsKeys = /^[_a-zA-Z0-9][ .\-_a-zA-Z0-9]*$/;

  readonly meta: eslint.Rule.RuleMetaData = {
    messages: {
      doubleQuoted: 'Only use double-quoted strings for externalized strings.',
      badKey: 'The key \'{{key}}\' doesn\'t conform to a valid localize identifier.',
      duplicateKey: 'Duplicate key \'{{key}}\' with different message value.',
      badMessage: 'Message argument to \'{{message}}\' must be a string literal.'
    },
    fixable: enableDoubleToSingleQuoteFixes ? 'code' : undefined,
  };

  create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
    const externalizedStringLiterals = new Map<string, { call: TSESTree.CallExpression; message: TSESTree.Node }[]>();
    const doubleQuotedStringLiterals = new Set<TSESTree.Node>();

    function collectDoubleQuotedStrings(node: ESTree.Literal) {
      if (isStringLiteral(node) && isDoubleQuoted(node)) {
        doubleQuotedStringLiterals.add(node);
      }
    }
```

**Enforced rule:** `eslint.config.js:95` — `'local/code-no-localization-template-literals': 'error'` (highest severity)

---

#### Pattern 7: Custom Architectural Rules Infrastructure
**Where:** `.eslint-plugin-local/index.ts`, `.eslint-plugin-local/utils.ts`
**What:** A modular system for implementing domain-specific linting rules via custom plugins.

**Registration pattern:** Each rule in `.eslint-plugin-local/` is a `.ts` file exporting an `eslint.Rule.RuleModule` class. The rules are registered in `eslint.config.js` with file-scope targeting:

```javascript
// From eslint.config.js:31-134
{
  languageOptions: { parser: tseslint.parser },
  plugins: {
    'local': pluginLocal,  // .eslint-plugin-local/index.ts exports all rules
    'header': pluginHeader,
  },
  rules: {
    'local/code-translation-remind': 'warn',
    'local/code-no-declare-const-enum': 'warn',
    'local/code-parameter-properties-must-have-explicit-accessibility': 'warn',
    'local/code-no-nls-in-standalone-editor': 'warn',
    'local/code-no-potentially-unsafe-disposables': 'warn',
    'local/code-no-dangerous-type-assertions': 'warn',
    'local/code-no-any-casts': 'warn',
    'local/code-no-standalone-editor': 'warn',
    'local/code-no-unexternalized-strings': 'warn',
    'local/code-must-use-super-dispose': 'warn',
    'local/code-declare-service-brand': 'warn',
    'local/code-no-reader-after-await': 'warn',
    'local/code-no-accessor-after-await': 'warn',
    'local/code-no-observable-get-in-reactive-context': 'warn',
    'local/code-no-localized-model-description': 'warn',
    'local/code-policy-localization-key-match': 'warn',
    'local/code-no-localization-template-literals': 'error',
    'local/code-no-icons-in-localized-strings': 'warn',
    'local/code-no-http-import': ['warn', { target: 'src/vs/**' }],
    'local/code-no-deep-import-of-internal': ['error', { '.*Internal': true }],
  }
}
```

**Related rules directory:** `.eslint-plugin-local/` contains 50+ rule implementations including:
- `code-amd-node-module.ts` — AMD/ES module conventions
- `code-no-in-operator.ts` — Type-safe object property checking
- `code-no-reader-after-await.ts` — Reactive context invariants
- `vscode-dts-*.ts` — Rules specific to `vscode.d.ts` API surface

---

## Architectural Implications for Tauri/Rust Port

### Layering Architecture is Non-Negotiable
The six-layer dependency graph (`common` → `node`/`browser` → `electron-*`) is enforced at lint-time and represents fundamental separation of concerns. A Rust port would need to preserve this layering in its module organization or risk breaking these invariants.

### Platform-Specific Code Paths
Conditional imports based on runtime platform (`hasBrowser`, `hasNode`, `hasElectron`) suggest the original codebase has significant platform abstraction. A Rust/Tauri port would need equivalent abstractions or feature flags.

### Resource Management Culture
The `dispose()` pattern and `_serviceBrand` convention indicate a mature resource cleanup discipline. These would need Rust equivalents (RAII, trait bounds, or Drop implementations).

### Localization as First-Class
Double-quoted string enforcement and externalizable string validation show localization is baked into the architecture, not an afterthought. The port would need equivalent tooling.

### Custom Linting as Enforcer
VS Code relies on custom ESLint rules to enforce architectural policies at scale. A Rust port would likely need clippy plugins, cargo-check hooks, or similar to enforce equivalent invariants.

---

## Reference Documentation

- **Architecture wiki:** https://github.com/microsoft/vscode/wiki/Source-Code-Organization
- **Custom ESLint rules template:** `.eslint-plugin-local/README.md`
- **ESLint parser:** `typescript-eslint` with flat config API
- **Key plugins:** `@stylistic/eslint-plugin-ts`, `eslint-plugin-jsdoc`, `eslint-plugin-header`
