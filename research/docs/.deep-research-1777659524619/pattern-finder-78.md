# Bootstrap & Module Loading Patterns in VS Code

## Research Question
What patterns exist for bootstrapping VS Code's core IDE functionality from TypeScript/Electron to understand what would need to be ported to Tauri/Rust?

## Scope
`src/bootstrap-server.ts` (7 LOC) and related bootstrap infrastructure files.

---

## Patterns Found

#### Pattern 1: Server-Side Bootstrap Shim
**Where:** `src/bootstrap-server.ts:7`
**What:** The entry point that prevents Electron from interfering with Node.js module resolution by deleting the `ELECTRON_RUN_AS_NODE` environment variable.

```typescript
// Keep bootstrap-esm.js from redefining 'fs'.
delete process.env['ELECTRON_RUN_AS_NODE'];
```

**Variations:** This minimal file acts as a guard that must execute before any other imports in server-main.ts. The comment indicates coordination with bootstrap-esm.js which also manipulates module resolution.

---

#### Pattern 2: Multi-Stage Bootstrap Architecture
**Where:** `src/server-main.ts:6` - Import sequence
**What:** A layered bootstrap approach where each stage handles specific initialization concerns:

```typescript
import './bootstrap-server.js'; // this MUST come before other imports as it changes global state
import * as path from 'node:path';
import * as http from 'node:http';
// ... other imports ...
import { devInjectNodeModuleLookupPath, removeGlobalNodeJsModuleLookupPaths } from './bootstrap-node.js';
import { bootstrapESM } from './bootstrap-esm.js';
```

**Variations:** Three distinct bootstrap modules handle:
1. **bootstrap-server.ts** - Electron environment cleanup (7 LOC)
2. **bootstrap-node.ts** - Node.js module resolution and working directory setup
3. **bootstrap-esm.ts** - ESM hook registration and NLS (Nationalization) setup

The comment emphasizes that bootstrap-server must execute first due to global state modification.

---

#### Pattern 3: Dynamic Module Resolution via createRequire
**Where:** `src/bootstrap-meta.ts:6-20`
**What:** Uses CommonJS `createRequire` to load JSON configuration files in ESM context:

```typescript
import { createRequire } from 'node:module';
import type { IProductConfiguration } from './vs/base/common/product.js';
import type { INodeProcess } from './vs/base/common/platform.js';

const require = createRequire(import.meta.url);

let productObj: Partial<IProductConfiguration> & { BUILD_INSERT_PRODUCT_CONFIGURATION?: string } = { BUILD_INSERT_PRODUCT_CONFIGURATION: 'BUILD_INSERT_PRODUCT_CONFIGURATION' };
if (productObj['BUILD_INSERT_PRODUCT_CONFIGURATION']) {
	productObj = require('../product.json'); // Running out of sources
}

let pkgObj = { BUILD_INSERT_PACKAGE_CONFIGURATION: 'BUILD_INSERT_PACKAGE_CONFIGURATION' };
if (pkgObj['BUILD_INSERT_PACKAGE_CONFIGURATION']) {
	pkgObj = require('../package.json'); // Running out of sources
}
```

**Variations:** 
- Placeholder constants checked at runtime before loading actual files
- Supports conditional sub-configuration loading (product.sub.json, package.sub.json)
- Development overrides (product.overrides.json) loaded when `VSCODE_DEV` is set
- Error handling with try-catch to allow missing optional configs

---

#### Pattern 4: ESM Module Hook Registration
**Where:** `src/bootstrap-esm.ts:14-29`
**What:** Registers an ESM resolve hook to redirect 'fs' to 'original-fs' in Electron contexts:

```typescript
if (process.env['ELECTRON_RUN_AS_NODE'] || process.versions['electron']) {
	const jsCode = `
	export async function resolve(specifier, context, nextResolve) {
		if (specifier === 'fs') {
			return {
				format: 'builtin',
				shortCircuit: true,
				url: 'node:original-fs'
			};
		}

		// Defer to the next hook in the chain, which would be the
		// Node.js default resolve if this is the last user-specified loader.
		return nextResolve(specifier, context);
	}`;
	register(`data:text/javascript;base64,${Buffer.from(jsCode).toString('base64')}`, import.meta.url);
}
```

**Variations:**
- Uses base64-encoded inline JavaScript to register the module hook
- Only activates in Electron contexts
- Defers to nextResolve for other modules (hook chaining pattern)
- Allows Electron to preserve its own 'fs' implementation

---

#### Pattern 5: Global State Initialization
**Where:** `src/bootstrap-esm.ts:33-35`
**What:** Populates globalThis with product metadata and file paths accessible throughout the application:

```typescript
globalThis._VSCODE_PRODUCT_JSON = { ...product };
globalThis._VSCODE_PACKAGE_JSON = { ...pkg };
globalThis._VSCODE_FILE_ROOT = import.meta.dirname;
```

**Variations:**
- Additional globals set for NLS: `_VSCODE_NLS_LANGUAGE`, `_VSCODE_NLS_MESSAGES`
- Used as early configuration available to all modules without imports
- Singleton pattern for environment-wide configuration

---

#### Pattern 6: Process Environment Sanitization
**Where:** `src/bootstrap-cli.ts:11`, `src/bootstrap-fork.ts:4-11`
**What:** Early cleanup of environment variables to prevent state leakage:

```typescript
// bootstrap-cli.ts
delete process.env['VSCODE_CWD'];
```

```typescript
// bootstrap-fork.ts
function terminateWhenParentTerminates(): void {
	const parentPid = Number(process.env['VSCODE_PARENT_PID']);

	if (typeof parentPid === 'number' && !isNaN(parentPid)) {
		setInterval(function () {
			try {
				process.kill(parentPid, 0); // throws an exception if the main process doesn't exist anymore.
			} catch (e) {
				process.exit();
			}
		}, 5000);
	}
}
```

**Variations:**
- Environment cleanup (VSCODE_CWD) to prevent shell state leakage
- Parent process monitoring via environment-passed PID
- Forked process termination coordination
- Working directory setup for cross-platform compatibility

---

#### Pattern 7: Module Import Interception & Redirection
**Where:** `src/bootstrap-import.ts:22-101`
**What:** Node.js ESM loader that redirects module resolution for development scenarios:

```typescript
export async function initialize(injectPath: string): Promise<void> {
	const injectPackageJSONPath = fileURLToPath(new URL('../package.json', pathToFileURL(injectPath)));
	const packageJSON = JSON.parse(String(await promises.readFile(injectPackageJSONPath)));

	for (const [name] of Object.entries(packageJSON.dependencies)) {
		try {
			const path = join(injectPackageJSONPath, `../node_modules/${name}/package.json`);
			const pkgJson = JSON.parse(String(await promises.readFile(path)));

			// Determine the entry point: prefer exports["."].import for ESM, then main.
			let main: string | undefined;
			if (pkgJson.exports?.['.']) {
				const dotExport = pkgJson.exports['.'];
				if (typeof dotExport === 'string') {
					main = dotExport;
				} else if (typeof dotExport === 'object' && dotExport !== null) {
					const resolveCondition = (v: unknown): string | undefined => {
						if (typeof v === 'string') {
							return v;
						}
						if (typeof v === 'object' && v !== null) {
							const d = (v as { default?: unknown }).default;
							if (typeof d === 'string') {
								return d;
							}
						}
						return undefined;
					};
					main = resolveCondition(dotExport.import) ?? resolveCondition(dotExport.default);
				}
			}
			if (typeof main !== 'string') {
				main = typeof pkgJson.main === 'string' ? pkgJson.main : undefined;
			}

			if (!main) {
				main = 'index.js';
			}
			if (!main.endsWith('.js') && !main.endsWith('.mjs') && !main.endsWith('.cjs')) {
				main += '.js';
			}
			const mainPath = join(injectPackageJSONPath, `../node_modules/${name}/${main}`);
			_specifierToUrl[name] = pathToFileURL(mainPath).href;
			const isModule = main.endsWith('.mjs')
				? true
				: main.endsWith('.cjs')
					? false
					: pkgJson.type === 'module';
			_specifierToFormat[name] = isModule ? 'module' : 'commonjs';
		} catch (err) {
			console.error(name);
			console.error(err);
		}
	}
}

export async function resolve(specifier: string | number, context: unknown, nextResolve: (arg0: unknown, arg1: unknown) => unknown) {
	const newSpecifier = _specifierToUrl[specifier];
	if (newSpecifier !== undefined) {
		return {
			format: _specifierToFormat[specifier] ?? 'commonjs',
			shortCircuit: true,
			url: newSpecifier
		};
	}

	return nextResolve(specifier, context);
}
```

**Variations:**
- Conditional module format detection (ESM vs CommonJS)
- Handles complex export definitions from package.json
- Pre-populates resolution cache to avoid repeated lookups
- Graceful error handling for missing modules

---

#### Pattern 8: Server Request/WebSocket Delegation
**Where:** `src/server-main.ts:88-104`
**What:** Lazy initialization of the server handler via closure pattern:

```typescript
let _remoteExtensionHostAgentServer: IServerAPI | null = null;
let _remoteExtensionHostAgentServerPromise: Promise<IServerAPI> | null = null;
const getRemoteExtensionHostAgentServer = () => {
	if (!_remoteExtensionHostAgentServerPromise) {
		_remoteExtensionHostAgentServerPromise = loadCode(nlsConfiguration).then(async (mod) => {
			const server = await mod.createServer(address);
			_remoteExtensionHostAgentServer = server;
			return server;
		});
	}
	return _remoteExtensionHostAgentServerPromise;
};

const server = http.createServer(async (req, res) => {
	if (firstRequest) {
		firstRequest = false;
		perf.mark('code/server/firstRequest');
	}
	const remoteExtensionHostAgentServer = await getRemoteExtensionHostAgentServer();
	return remoteExtensionHostAgentServer.handleRequest(req, res);
});
server.on('upgrade', async (req, socket) => {
	if (firstWebSocket) {
		firstWebSocket = false;
		perf.mark('code/server/firstWebSocket');
	}
	const remoteExtensionHostAgentServer = await getRemoteExtensionHostAgentServer();
	return remoteExtensionHostAgentServer.handleUpgrade(req, socket);
});
```

**Variations:**
- Promise caching to prevent multiple initializations
- Deferred server creation until first request
- Unified delegation for HTTP and WebSocket upgrade paths
- Performance markers for instrumentation

---

#### Pattern 9: Forked Process Bootstrap Chain
**Where:** `src/bootstrap-fork.ts:1-230`
**What:** Multi-step bootstrap for forked/child processes with logging relay, exception handling, and parent process monitoring:

```typescript
performance.mark('code/fork/start');

// Configure: pipe logging to parent process
if (!!process.send && process.env['VSCODE_PIPE_LOGGING'] === 'true') {
	pipeLoggingToParent();
}

// Handle Exceptions
if (!process.env['VSCODE_HANDLES_UNCAUGHT_ERRORS']) {
	handleExceptions();
}

// Terminate when parent terminates
if (process.env['VSCODE_PARENT_PID']) {
	terminateWhenParentTerminates();
}

// Bootstrap ESM
await bootstrapESM();

// Load ESM entry point
await import([`./${process.env['VSCODE_ESM_ENTRYPOINT']}.js`].join('/'));
```

**Variations:**
- Console wrapping to intercept child process logs
- Stream wrapping for stdout/stderr redirection
- Uncaught exception and unhandled rejection handlers
- Dynamic entry point loading via environment variable
- Parent process liveness detection with periodic polling

---

#### Pattern 10: Node.js Module Path Manipulation
**Where:** `src/bootstrap-node.ts:62-98`
**What:** Intercepts Node.js module resolution to remove global paths and support development injection:

```typescript
export function devInjectNodeModuleLookupPath(injectPath: string): void {
	if (!process.env['VSCODE_DEV']) {
		return; // only applies running out of sources
	}

	if (!injectPath) {
		throw new Error('Missing injectPath');
	}

	// register a loader hook
	const Module = require('node:module');
	Module.register('./bootstrap-import.js', { parentURL: import.meta.url, data: injectPath });
}

export function removeGlobalNodeJsModuleLookupPaths(): void {
	if (typeof process?.versions?.electron === 'string') {
		return; // Electron disables global search paths
	}

	const Module = require('module');
	const globalPaths = Module.globalPaths;

	const originalResolveLookupPaths = Module._resolveLookupPaths;

	Module._resolveLookupPaths = function (moduleName: string, parent: unknown): string[] {
		const paths = originalResolveLookupPaths(moduleName, parent);
		if (Array.isArray(paths)) {
			let commonSuffixLength = 0;
			while (commonSuffixLength < paths.length && paths[paths.length - 1 - commonSuffixLength] === globalPaths[globalPaths.length - 1 - commonSuffixLength]) {
				commonSuffixLength++;
			}

			return paths.slice(0, paths.length - commonSuffixLength);
		}

		return paths;
	};
}
```

**Variations:**
- Monkey-patching Module internals for path filtering
- Platform-specific behavior (Electron vs pure Node.js)
- Dev-only injection of custom module loaders
- Preserves original functions and delegates to them

---

## Summary

VS Code's bootstrap architecture demonstrates a sophisticated approach to managing multiple runtime contexts (Electron, Node.js server, CLI, forked processes) with shared TypeScript/ESM codebase. The pattern emphasizes:

1. **Sequenced Initialization**: Multiple bootstrap stages must execute in strict order to modify global state correctly
2. **Environment Detection**: Runtime behavior conditional on `VSCODE_DEV`, `process.versions.electron`, and custom environment variables
3. **Module Resolution Control**: Deep interception at Node.js/ESM loader level for development flexibility
4. **Process Coordination**: Environment variables and IPC for parent-child process communication
5. **Global State Management**: Centralizing configuration via `globalThis` to avoid circular imports
6. **Lazy Initialization**: Deferring expensive setup (NLS, server creation) until actually needed
7. **Hook Chaining**: ESM and CommonJS hooks that delegate to next resolver in chain
8. **Cross-Platform Compatibility**: Working directory and signal handling differences between Windows/Unix

For a Tauri/Rust port, these patterns would need equivalents in Rust's FFI, module system, and process management layers. The heavy reliance on runtime module resolution and dynamic loading would be a significant change from compile-time resolution.

