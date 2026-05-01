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

