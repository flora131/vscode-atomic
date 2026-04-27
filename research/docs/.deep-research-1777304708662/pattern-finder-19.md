# Pattern Analysis: VS Code ESLint Plugin Architectural Invariants

## Overview

The `.eslint-plugin-local/` directory (49 files, ~3,952 LOC) contains custom ESLint rules that enforce VS Code's architectural invariants. These rules encode layering constraints, module system requirements, and runtime invariants that a Rust/Tauri port would need to respect. The patterns reveal critical contracts about how code is organized: layering rules prevent cyclic dependencies, import patterns enforce ESM compliance, and service brand patterns ensure type-safe dependency injection.

---

## Pattern Analysis

#### Pattern: Layer-Based Import Restrictions

**Where:** `.eslint-plugin-local/code-layering.ts:15-92`

**What:** Enforces strict layering across codebase partitions by validating import paths against a configuration mapping.

```typescript
export default new class implements eslint.Rule.RuleModule {
	readonly meta: eslint.Rule.RuleMetaData = {
		messages: {
			layerbreaker: 'Bad layering. You are not allowed to access {{from}} from here, allowed layers are: [{{allowed}}]'
		},
		docs: {
			url: 'https://github.com/microsoft/vscode/wiki/Source-Code-Organization'
		}
	};

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
			// resolve path, check against allowed/disallowed sets
		});
	}
};
```

**Variations:** 
- `code-no-runtime-import.ts` extends this with type-only restrictions
- Configuration-driven via `.eslintrc` patterns; no hardcoded layer names

---

#### Pattern: Conditional Layer Definitions by Platform

**Where:** `.eslint-plugin-local/code-import-patterns.ts:42-279`

**What:** Generates different layering rules based on target platform (browser, node, electron) with conditional pattern substitution.

```typescript
interface RawImportPatternsConfig {
	target: string;
	layer?: 'common' | 'worker' | 'browser' | 'electron-browser' | 'node' | 'electron-utility' | 'electron-main';
	test?: boolean;
	restrictions: string | (string | ConditionalPattern)[];
}

const layerRules: ILayerRule[] = [
	{ layer: 'common', deps: 'common' },
	{ layer: 'worker', deps: '{common,worker}' },
	{ layer: 'browser', deps: '{common,browser}', isBrowser: true },
	{ layer: 'electron-browser', deps: '{common,browser,electron-browser}', isBrowser: true },
	{ layer: 'node', deps: '{common,node}', isNode: true },
	{ layer: 'electron-utility', deps: '{common,node,electron-utility}', isNode: true, isElectron: true },
	{ layer: 'electron-main', deps: '{common,node,electron-utility,electron-main}', isNode: true, isElectron: true },
];

function generateConfig(layerRule: ILayerRule, target: string, restrictions: (string | ConditionalPattern)[]): ImportPatternsConfig[] {
	const restrictions: string[] = [];
	if (layerRule.isBrowser) { restrictions.push(...browserAllow); }
	if (layerRule.isNode) { restrictions.push(...nodeAllow); }
	if (layerRule.isElectron) { restrictions.push(...electronAllow); }
	// Pattern substitution: /~/ becomes /common/** or /browser/** based on layer
}
```

**Variations:**
- Test files get separate `test/**` path variants
- Patterns with `when?: 'hasBrowser' | 'hasNode' | 'hasElectron' | 'test'` conditionally apply restrictions
- ESM enforcement: requires `.js` or `.css` extensions, no absolute `vs/` imports for ES modules

---

#### Pattern: Generic AST Query-Based Rule Creation

**Where:** `.eslint-plugin-local/utils.ts:10-41`

**What:** Shared helper that creates listeners for all import statement variations using ESTree selector queries.

```typescript
export function createImportRuleListener(
	validateImport: (node: TSESTree.Literal, value: string) => any
): eslint.Rule.RuleListener {

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
		// export ?? from 'module'
		ExportAllDeclaration: (node: ESTree.ExportAllDeclaration) => {
			_checkImport((node as TSESTree.ExportAllDeclaration).source);
		},
	};
}
```

**Variations:**
- Handles static and dynamic imports uniformly
- All rules that validate imports inherit this listener

---

#### Pattern: Property Definition Enforcement with AST Selectors

**Where:** `.eslint-plugin-local/code-declare-service-brand.ts:16-26`

**What:** Uses precise AST selector queries to enforce property patterns with fixable code transformations.

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

**Variations:**
- `code-no-potentially-unsafe-disposables.ts` uses similar selectors: `'PropertyDefinition[readonly!=true]'`
- `code-no-static-self-ref.ts` combines `PropertyDefinition[static=true]` with ancestor checking
- All leverage ESLint's `fixer` API for automated fixes

---

#### Pattern: Dynamic Listener Generation from Configuration

**Where:** `.eslint-plugin-local/code-must-use-result.ts:20-39`

**What:** Creates a listener object dynamically by iterating configuration arrays to build query selectors.

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

**Variations:**
- `code-no-test-only.ts` uses regex in selector: `'MemberExpression[object.name=/^(test|suite)$/][property.name="only"]'`
- Pattern values are injected into selectors dynamically; enables configuration-driven enforcement

---

#### Pattern: API Naming Convention Validation with Regex

**Where:** `.eslint-plugin-local/vscode-dts-interface-naming.ts:21-35`

**What:** Validates naming conventions by matching AST node identifiers against regex patterns.

```typescript
export default new class ApiInterfaceNaming implements eslint.Rule.RuleModule {
	private static _nameRegExp = /^I[A-Z]/;

	readonly meta: eslint.Rule.RuleMetaData = {
		messages: {
			naming: 'Interfaces must not be prefixed with uppercase `I`',
		}
	};

	create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
		return {
			['TSInterfaceDeclaration Identifier']: (node: ESTree.Identifier) => {
				const name = (node as TSESTree.Identifier).name;
				if (ApiInterfaceNaming._nameRegExp.test(name)) {
					context.report({
						node,
						messageId: 'naming'
					});
				}
			}
		};
	}
};
```

**Variations:**
- `code-amd-node-module.ts` checks package.json dependencies to validate module references
- Regex patterns are statically defined as class properties; metadata messages are localized via `messageId`

---

#### Pattern: Package Metadata Parsing for Runtime Validation

**Where:** `.eslint-plugin-local/code-amd-node-module.ts:25-56`

**What:** Parses `package.json` at plugin initialization to validate that Node module imports use the correct import mechanism.

```typescript
const modules = new Set<string>();

try {
	const packageJson = JSON.parse(readFileSync(join(import.meta.dirname, '../package.json'), 'utf-8'));
	const { dependencies, optionalDependencies } = packageJson;
	const all = Object.keys(dependencies).concat(Object.keys(optionalDependencies));
	for (const key of all) {
		modules.add(key);
	}
} catch (e) {
	console.error(e);
	throw e;
}

const checkImport = (node: ESTree.Literal & { parent?: ESTree.Node & { importKind?: string } }) => {
	if (typeof node.value !== 'string') return;
	if (node.parent?.type === 'ImportDeclaration' && node.parent.importKind === 'type') return;
	if (!modules.has(node.value)) return;
	
	context.report({
		node,
		messageId: 'amdX'
	});
};
```

**Variations:**
- Metadata is loaded once during plugin instantiation; affects all files
- Type imports (`import type`) are exempted from validation
- This pattern ties ESLint rules to runtime package configurations

---

## Cross-Cutting Patterns

### Rule Module Structure
All 49 rules follow an identical export pattern:
```typescript
export default new class implements eslint.Rule.RuleModule {
	readonly meta: eslint.Rule.RuleMetaData = { messages: {...}, docs: {...}, schema: {...} };
	create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener { ... }
};
```

### Error Reporting
All use `context.report()` with either:
- **`messageId`** + **`data`**: For localized messages with variable substitution
- **`message`**: For fixed error strings
- **`fix`**: For auto-fixable violations (via `fixer.replaceText()`, etc.)

### Configuration Driven
Rules are configured via `.eslintrc` and accept:
- Pattern arrays (for layering)
- Conditional restrictions (for platform-specific rules)
- Function name lists (for must-use validation)

### AST Query Languages
Rules leverage:
- **ESTree selectors**: `ImportDeclaration`, `PropertyDefinition[static=true]`, etc.
- **Child combinators**: `> Literal`, `PropertyDefinition > NewExpression`
- **Attribute selectors**: `[key.name="_serviceBrand"]`, `[callee.name=/pattern/]`
- **Pseudo-patterns**: `CallExpression[callee.type="Import"]` for dynamic imports

---

## Architecture Implications for Rust/Tauri Port

The ESLint plugin encodes these critical invariants:

1. **Strict layering** prevents circular dependencies across 7+ architectural partitions (common, worker, browser, electron-browser, node, electron-utility, electron-main)
2. **Module system compliance** enforces ESM (relative imports, `.js` extensions) for code under `src/vs/`
3. **Service brand patterns** enable type-safe dependency injection through sentinel property definitions
4. **Platform-conditional imports** encode how code supports multiple runtimes (browser, Node, Electron)
5. **Disposable safety** prevents resource leaks through readonly and const enforcement
6. **No-op value assertions** ensure critical functions like `.dispose()` are actually called

A Rust port cannot simply replicate this logic—it must adopt equivalent constraints at the module/crate level. Layering becomes crate dependency rules; ESM extensions become module path conventions; service brands become trait-based markers. The 49 rules document what architectural properties the codebase currently guarantees; a rewrite must preserve or strengthen these invariants.

