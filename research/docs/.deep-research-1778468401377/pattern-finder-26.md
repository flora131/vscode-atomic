# Architectural Layering Patterns in VS Code

## Overview
The ESLint configuration reveals VS Code's core architectural approach for managing dependencies and separation of concerns across different runtime environments. The patterns encode strict module boundary rules essential to porting functionality.

---

## Pattern: Multi-Layer Module Architecture

**Where:** `eslint.config.js:101-125`

**What:** Defines six distinct runtime layers with explicit dependency constraints to isolate environment-specific code.

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
],
```

**Variations / call-sites:**
- Applied globally to `src/**/*.ts:1456`
- Enforced through `local/code-import-patterns` rule at line 1476

---

## Pattern: Import Restrictions by Runtime Environment

**Where:** `eslint.config.js:1476-1562`

**What:** Conditional module import allowlists based on runtime context (browser, node, electron), preventing bloat in browser bundles.

```javascript
'local/code-import-patterns': [
	'warn',
	{
		// imports that are allowed in all files of layers:
		// - browser
		// - electron-browser
		'when': 'hasBrowser',
		'allow': []
	},
	{
		// imports that are allowed in all files of layers:
		// - node
		// - electron-utility
		// - electron-main
		'when': 'hasNode',
		'allow': [
			'@github/copilot-sdk',
			'@microsoft/dev-tunnels-contracts',
			'@vscode/sqlite3',
			'@vscode/vscode-languagedetection',
			'node-pty',
			'ssh2',
			'ws',
			'@xterm/xterm',
			'chrome-remote-interface'
		]
	},
	{
		// imports that are allowed in all files of layers:
		// - electron-utility
		// - electron-main
		'when': 'hasElectron',
		'allow': [
			'electron'
		]
	}
]
```

**Variations / call-sites:**
- Git extension override at line 852-865
- Platform module overrides at line 1640-1650 (additional allowed modules like `tas-client`, `@microsoft/1ds-core-js`)

---

## Pattern: Hierarchical Module Boundary Rules with Layer Expansion

**Where:** `eslint.config.js:1600-1732`

**What:** Uses `/~` template syntax to declare which layers can import from which subsystems, expanding to 14 distinct layer combinations (common, worker, browser, electron-browser, node, electron-main, and their test variants).

```javascript
{
	'target': 'src/vs/base/~',
	'restrictions': [
		'vs/base/~'
	]
},
{
	'target': 'src/vs/editor/~',
	'restrictions': [
		'vs/base/~',
		'vs/base/parts/*/~',
		'vs/platform/*/~',
		'vs/editor/~',
		'@vscode/tree-sitter-wasm' // node module allowed even in /common/
	]
},
{
	'target': 'src/vs/editor/contrib/*/~',
	'restrictions': [
		'vs/base/~',
		'vs/base/parts/*/~',
		'vs/platform/*/~',
		'vs/editor/~',
		'vs/editor/contrib/*/~'
	]
},
{
	'target': 'src/vs/workbench/~',
	'restrictions': [
		'vs/base/~',
		'vs/base/parts/*/~',
		'vs/platform/*/~',
		'vs/editor/~',
		'vs/editor/contrib/*/~',
		'vs/workbench/~',
		'vs/workbench/services/*/~'
	]
}
```

**Variations / call-sites:**
- Services layer: line 1751-1772
- Contrib modules: line 1775-1802
- Terminal contrib: line 1805-1833
- API layer: line 1735-1748
- Workbench main entry: line 1909-1924
- Desktop main entry: line 1961-1975

---

## Pattern: Entry Point Aggregation Rules

**Where:** `eslint.config.js:1685-1715`

**What:** Defines which components can be imported into bundled entry points (editor.all.ts, editor.api.ts, editor.main.ts) to control bundle composition and initialization order.

```javascript
{
	'target': 'src/vs/editor/editor.all.ts',
	'layer': 'browser',
	'restrictions': [
		'vs/base/~',
		'vs/base/parts/*/~',
		'vs/platform/*/~',
		'vs/editor/~',
		'vs/editor/contrib/*/~'
	]
},
{
	'target': 'src/vs/editor/{editor.api.ts,editor.main.ts}',
	'layer': 'browser',
	'restrictions': [
		'vs/base/~',
		'vs/base/parts/*/~',
		'vs/editor/~',
		'vs/editor/contrib/*/~',
		'vs/editor/standalone/~',
		'vs/editor/*'
	]
},
```

**Variations / call-sites:**
- Worker entry: line 1696-1703
- Terminal entry: line 1887-1905
- Workbench common entry: line 1909-1924
- Workbench web entry: line 1927-1941

---

## Pattern: Electron Main Process Startup Optimization

**Where:** `eslint.config.js:1051-1076`

**What:** Enforces strict rules to prevent synchronous loading of heavy dependencies during electron main process startup, with exceptions for isolated processes and safe modules.

```javascript
{
	files: [
		'src/vs/code/electron-main/**/*.ts',
		'src/vs/code/node/**/*.ts',
		'src/vs/platform/*/electron-main/**/*.ts',
		'src/vs/platform/*/node/**/*.ts',
	],
	languageOptions: {
		parser: tseslint.parser,
	},
	plugins: {
		'local': pluginLocal,
	},
	rules: {
		'local/code-no-static-node-module-import': [
			'error',
			// Files that run in separate processes, not on the electron-main startup path
			'src/vs/platform/agentHost/node/**/*.ts',
			'src/vs/platform/files/node/watcher/**/*.ts',
			'src/vs/platform/terminal/node/**/*.ts',
			// Files that use small, safe modules
			'src/vs/platform/environment/node/argv.ts',
		]
	}
}
```

**Variations / call-sites:**
- Referenced as part of electron-main architectural constraints; see also lines 1437-1451 for electron module imports

---

## Pattern: DOM Access Standardization in Multi-Window Contexts

**Where:** `eslint.config.js:1078-1200`

**What:** Enforces window-aware DOM access patterns to support VS Code's multi-window editor architecture, requiring explicit window resolution instead of global `document` references.

```javascript
{
	files: [
		'src/**/{browser,electron-browser}/**/*.ts'
	],
	languageOptions: {
		parser: tseslint.parser,
	},
	plugins: {
		'local': pluginLocal,
	},
	rules: {
		'local/code-no-global-document-listener': 'warn',
		'no-restricted-syntax': [
			'warn',
			{
				'selector': `MemberExpression[object.name='document'][property.name='activeElement']`,
				'message': 'Use <targetWindow>.document.activeElement to support multi-window scenarios. Resolve targetWindow with DOM.getWindow(element) or DOM.getActiveWindow() or use the predefined mainWindow constant.'
			},
			// ... 40+ similar rules for document.* APIs
		]
	}
}
```

**Variations / call-sites:**
- Covers all browser/electron-browser files at line 1080-1082
- Rules extend through line 1200+ with enforcement for Intl, HTMLElement, SVGElement, KeyboardEvent, PointerEvent, DragEvent APIs

---

## Pattern: Extension Module Boundary Enforcement

**Where:** `eslint.config.js:2448-2527`

**What:** Defines strict import zones within copilot extension to enforce layered architecture (common/platform/extension/vscode layers), preventing circular dependencies and ensuring testability.

```javascript
'import/no-restricted-paths': [
	'warn',
	{
		zones: [
			{
				target: '**/common/**',
				from: [
					'**/vscode/**',
					'**/node/**',
					'**/vscode-node/**',
					'**/worker/**',
					'**/vscode-worker/**'
				]
			},
			{
				target: '**/vscode/**',
				from: [
					'**/node/**',
					'**/vscode-node/**',
					'**/worker/**',
					'**/vscode-worker/**'
				]
			},
			{
				target: './extensions/copilot/src/platform',
				from: ['./extensions/copilot/src/extension']
			},
			{
				target: './extensions/copilot/src/util',
				from: ['./extensions/copilot/src/platform', './extensions/copilot/src/extension']
			}
		]
	}
]
```

**Variations / call-sites:**
- Git extension pattern at line 837-865
- Terminal contrib layering at line 2293-2319
- Notebook renderer constraints at line 2263-2291

---

## Summary

The ESLint configuration demonstrates VS Code's core architectural principles for a Tauri/Rust port:

1. **Multi-layer isolation**: Six distinct runtime environments (common, node, browser, electron-browser, electron-utility, electron-main) with strict import boundaries prevent environment-specific code from bleeding into inappropriate contexts.

2. **Entry point control**: Bundled outputs (editor, terminal, workbench modules) aggregate only allowed subsystems, maintaining tree-shaking and startup performance.

3. **Electron-specific optimization**: Heavy node module imports are forbidden during main process startup, with exceptions for isolated processes—critical for app responsiveness.

4. **Window-aware DOM**: Multi-window support requires explicit window resolution rather than global document access, a pattern that would need equivalent memory safety mechanisms in Rust/Tauri.

5. **Extension containment**: Clear import boundaries within extensions (especially Copilot) prevent circular dependencies and facilitate independent testing and deployment.

These patterns express VS Code's dependency inversion and separation-of-concern strategy. A Tauri port would need analogous layer boundaries to maintain these architectural properties, though expressed through Rust's module system and FFI boundaries rather than JavaScript import restrictions.
