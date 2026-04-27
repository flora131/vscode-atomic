# Partition 78 of 79 — Findings

## Scope
`src/bootstrap-server.ts/` (1 files, 7 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Partition 78: bootstrap-server.ts

## Implementation

- `/Users/norinlavaee/vscode-atomic/src/bootstrap-server.ts` (7 LOC) — Server process bootstrap shim

The file contains a single statement that removes the `ELECTRON_RUN_AS_NODE` environment variable. This initialization must execute before other imports, as noted in both `/Users/norinlavaee/vscode-atomic/src/server-main.ts` (line 6) and `/Users/norinlavaee/vscode-atomic/src/server-cli.ts` (line 6) where it is imported with the comment "this MUST come before other imports as it changes global state".

## Tests

Related server test files exist but none specifically test bootstrap-server:
- `/Users/norinlavaee/vscode-atomic/src/vs/server/test/node/serverMain.test.ts`
- `/Users/norinlavaee/vscode-atomic/src/vs/server/test/node/serverConnectionToken.test.ts`
- `/Users/norinlavaee/vscode-atomic/src/vs/server/test/node/serverLifetimeService.test.ts`
- `/Users/norinlavaee/vscode-atomic/src/vs/server/test/node/serverAgentHostManager.test.ts`

## Configuration

- `/Users/norinlavaee/vscode-atomic/eslint.config.js` (line 1978) — Lint rules include bootstrap-server.ts in target configuration

## Notable Clusters

**Bootstrap ecosystem**: Five related bootstrap files initialize different node process contexts:
- `bootstrap-cli.ts` — CLI process initialization
- `bootstrap-node.ts` — Generic Node.js process setup (path handling, signal handlers, working directory)
- `bootstrap-esm.ts` — ESM module resolution hooks (fs → original-fs remapping, NLS setup, globals)
- `bootstrap-fork.ts` — Child process forking
- `bootstrap-server.ts` — Server-specific initialization (Electron environment variable removal)

All bootstrap files are imported at the very start of their respective entry points (server-main.ts, server-cli.ts) before any other imports.

**Server entry points** that depend on bootstrap-server:
- `/Users/norinlavaee/vscode-atomic/src/server-main.ts` — Main server bootstrap and initialization
- `/Users/norinlavaee/vscode-atomic/src/server-cli.ts` — CLI spawning and extension management

---

## Summary

The `bootstrap-server.ts` partition is a minimal (~7 lines) Electron-environment cleanup shim. Its singular responsibility is removing the `ELECTRON_RUN_AS_NODE` flag before server initialization. To port VS Code's server bootstrap to Tauri/Rust would require:

1. **Removing Electron environment dependencies** — The deleted env var is only relevant to Electron's dual Node/Electron runtime modes. Tauri runs native Rust, so this flag has no meaning.
2. **Maintaining bootstrap ordering** — The import-order requirement would persist; Rust's module system would need to ensure global state initialization occurs before dependent code paths.
3. **Replacing ESM/CommonJS module hooks** — `bootstrap-esm.ts` uses Node's `register()` API to redirect fs module imports; Rust's module system handles this natively.
4. **Adapting shell environment and path setup** — `bootstrap-node.ts` handles working directory, SIGPIPE signals, and module resolution paths; Rust equivalents exist but require different APIs.

The bootstrap-server partition itself is technically unnecessary in a Tauri context since Electron environment variables wouldn't exist. However, the broader bootstrap pattern—prioritizing global state setup before application logic—remains valid and critical.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Bootstrap Server Analysis: Porting VS Code Core IDE to Tauri/Rust

## Research Question
What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust, focusing on the bootstrap-server entry point.

## Overview
The `bootstrap-server.ts` file is an extremely thin entry point (7 LOC) that serves a critical initialization function: preventing the `ELECTRON_RUN_AS_NODE` environment variable from being inherited. This variable, if present, would cause node modules to be incorrectly resolved. The file acts as a guard before any other imports execute, ensuring clean module resolution for server-side VS Code processes.

---

## Pattern Examples: Bootstrap Entry Point Architecture

### Pattern 1: Environment State Sanitization Guard
**Where:** `src/bootstrap-server.ts:1-8`
**What:** Minimal entry point that sanitizes process environment before downstream module loading.

```typescript
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

// Keep bootstrap-esm.js from redefining 'fs'.
delete process.env['ELECTRON_RUN_AS_NODE'];
```

**Key aspects:**
- Executes as first statement in import chain (before all other imports)
- Deletes a process environment variable that interferes with module resolution
- Protects ESM modules from Electron-specific behavior
- No other logic, pure environment state management

### Pattern 2: Bootstrap Chain Architecture
**Where:** `src/server-main.ts:6` and `src/server-cli.ts:6`
**What:** Bootstrap files are imported first, before any application logic.

```typescript
import './bootstrap-server.js'; // this MUST come before other imports as it changes global state
```

**Key aspects:**
- Explicit comment documents ordering requirement
- Bootstrap must execute before other imports
- Changes global state that affects all downstream modules
- Used consistently in both server-main.ts and server-cli.ts

### Pattern 3: Multi-Stage Bootstrap System
The codebase uses multiple bootstrap files for different execution contexts:

**Where:** `src/bootstrap-*.ts` (6 files total)
**What:** Context-specific initialization modules.

1. **bootstrap-server.ts** (7 LOC) - Server-specific environment cleanup
2. **bootstrap-cli.ts** (12 LOC) - CLI entry point environment cleanup
3. **bootstrap-node.ts** (191 LOC) - Core Node.js runtime setup
4. **bootstrap-esm.ts** (113 LOC) - ESM loader hooks and NLS setup
5. **bootstrap-fork.ts** (230 LOC) - Forked worker process initialization
6. **bootstrap-import.ts** (102 LOC) - Dev-mode module redirect resolver

Each addresses a specific execution context while maintaining consistent patterns.

### Pattern 4: Node.js Module System Interception
**Where:** `src/bootstrap-esm.ts:14-30`
**What:** Registers module resolution hooks for fs module aliasing in Electron contexts.

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

**Key aspects:**
- Uses Node.js Module.register() API for loader hooks
- Detects Electron via environment variable or process.versions
- Remaps 'fs' to 'node:original-fs' to bypass Electron's module proxying
- Uses base64-encoded inline hook definition
- Allows downstream code to use standard 'fs' without Electron interference

### Pattern 5: Development Mode Environment Injection
**Where:** `src/bootstrap-node.ts:62-74`
**What:** Dev-mode node module lookup path injection for local development.

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
```

**Key aspects:**
- Only activates in VSCODE_DEV mode
- Registers custom module loader for development builds
- Injects custom node_modules resolution path
- Uses Module.register() API to hook into Node.js loader chain

### Pattern 6: Process Environment Variable Sanitization
**Where:** `src/bootstrap-cli.ts:1-12`
**What:** CLI-specific cleanup of process environment to prevent state leakage.

```typescript
// Delete `VSCODE_CWD` very early. We have seen
// reports where `code .` would use the wrong
// current working directory due to our variable
// somehow escaping to the parent shell
// (https://github.com/microsoft/vscode/issues/126399)
delete process.env['VSCODE_CWD'];
```

**Key aspects:**
- Addresses specific bug where env vars leak to parent shell
- Executes extremely early to prevent propagation
- Includes issue reference for context
- Similar pattern to bootstrap-server but different variable

### Pattern 7: Product Configuration Metadata Loading
**Where:** `src/bootstrap-meta.ts:10-55`
**What:** Configuration metadata loading with fallback chaining and dev mode overrides.

```typescript
let productObj: Partial<IProductConfiguration> & { BUILD_INSERT_PRODUCT_CONFIGURATION?: string } = { BUILD_INSERT_PRODUCT_CONFIGURATION: 'BUILD_INSERT_PRODUCT_CONFIGURATION' }; // DO NOT MODIFY, PATCHED DURING BUILD
if (productObj['BUILD_INSERT_PRODUCT_CONFIGURATION']) {
	productObj = require('../product.json'); // Running out of sources
}

let pkgObj = { BUILD_INSERT_PACKAGE_CONFIGURATION: 'BUILD_INSERT_PACKAGE_CONFIGURATION' }; // DO NOT MODIFY, PATCHED DURING BUILD
if (pkgObj['BUILD_INSERT_PACKAGE_CONFIGURATION']) {
	pkgObj = require('../package.json'); // Running out of sources
}

// Load sub files
if ((process as INodeProcess).isEmbeddedApp) {
	// Preserve the parent VS Code's policy identity before the
	// embedded app overrides win32RegValueName / darwinBundleIdentifier.
	productObj.parentPolicyConfig = {
		win32RegValueName: productObj.win32RegValueName,
		darwinBundleIdentifier: productObj.darwinBundleIdentifier,
		urlProtocol: productObj.urlProtocol,
	};

	try {
		const productSubObj = require('../product.sub.json');
		if (productObj.embedded && productSubObj.embedded) {
			Object.assign(productObj.embedded, productSubObj.embedded);
			delete productSubObj.embedded;
		}
		Object.assign(productObj, productSubObj);
	} catch (error) { /* ignore */ }
	try {
		const pkgSubObj = require('../package.sub.json');
		pkgObj = Object.assign(pkgObj, pkgSubObj);
	} catch (error) { /* ignore */ }
}

let productOverridesObj = {};
if (process.env['VSCODE_DEV']) {
	try {
		productOverridesObj = require('../product.overrides.json');
		productObj = Object.assign(productObj, productOverridesObj);
	} catch (error) { /* ignore */ }
}

export const product = productObj;
export const pkg = pkgObj;
```

**Key aspects:**
- Build-time injection of configuration (marked DO NOT MODIFY)
- Fallback to file-based loading during development
- Support for embedded apps with policy preservation
- Platform-specific configuration overrides
- Silent failure on missing optional config files

### Pattern 8: Global State Initialization
**Where:** `src/bootstrap-esm.ts:32-35`
**What:** Global namespace pollution for runtime metadata access.

```typescript
// Prepare globals that are needed for running
globalThis._VSCODE_PRODUCT_JSON = { ...product };
globalThis._VSCODE_PACKAGE_JSON = { ...pkg };
globalThis._VSCODE_FILE_ROOT = import.meta.dirname;
```

**Key aspects:**
- Attaches product/package metadata to global scope
- Makes configuration available to all modules without imports
- Uses globalThis for universal access
- Copies values rather than storing references

### Pattern 9: Current Working Directory Normalization
**Where:** `src/bootstrap-node.ts:35-55`
**What:** Platform-specific working directory setup to ensure consistency.

```typescript
function setupCurrentWorkingDirectory(): void {
	try {

		// Store the `process.cwd()` inside `VSCODE_CWD`
		// for consistent lookups, but make sure to only
		// do this once unless defined already from e.g.
		// a parent process.
		if (typeof process.env['VSCODE_CWD'] !== 'string') {
			process.env['VSCODE_CWD'] = process.cwd();
		}

		// Windows: always set application folder as current working dir
		if (process.platform === 'win32') {
			process.chdir(path.dirname(process.execPath));
		}
	} catch (err) {
		console.error(err);
	}
}

setupCurrentWorkingDirectory();
```

**Key aspects:**
- Stores CWD in environment variable for inter-process consistency
- Windows-specific: changes CWD to application folder
- Idempotent: checks if already set by parent
- Error resilience: logs but doesn't throw

### Pattern 10: Forked Process Communication Pipeline
**Where:** `src/bootstrap-fork.ts:14-154`
**What:** Wraps console and stream I/O to route output to parent process.

```typescript
function pipeLoggingToParent(): void {
	const MAX_STREAM_BUFFER_LENGTH = 1024 * 1024;
	const MAX_LENGTH = 100000;

	// ... helper functions ...

	function safeSendConsoleMessage(severity: 'log' | 'warn' | 'error', args: string): void {
		safeSend({ type: '__$console', severity, arguments: args });
	}

	function wrapConsoleMethod(method: 'log' | 'info' | 'warn' | 'error', severity: 'log' | 'warn' | 'error'): void {
		Object.defineProperty(console, method, {
			set: () => { },
			get: () => function () { safeSendConsoleMessage(severity, safeToString(arguments)); },
		});
	}

	function wrapStream(streamName: 'stdout' | 'stderr', severity: 'log' | 'warn' | 'error'): void {
		const stream = process[streamName];
		const original = stream.write;

		let buf = '';

		Object.defineProperty(stream, 'write', {
			set: () => { },
			get: () => (chunk: string | Buffer | Uint8Array, encoding: BufferEncoding | undefined, callback: ((err?: Error | null) => void) | undefined) => {
				buf += chunk.toString(encoding);
				const eol = buf.length > MAX_STREAM_BUFFER_LENGTH ? buf.length : buf.lastIndexOf('\n');
				if (eol !== -1) {
					console[severity](buf.slice(0, eol));
					buf = buf.slice(eol + 1);
				}

				original.call(stream, chunk, encoding, callback);
			},
		});
	}

	if (process.env['VSCODE_VERBOSE_LOGGING'] === 'true') {
		wrapConsoleMethod('info', 'log');
		wrapConsoleMethod('log', 'log');
		wrapConsoleMethod('warn', 'warn');
		wrapConsoleMethod('error', 'error');
	} else {
		console.log = function () { /* ignore */ };
		console.warn = function () { /* ignore */ };
		console.info = function () { /* ignore */ };
		wrapConsoleMethod('error', 'error');
	}

	wrapStream('stderr', 'error');
	wrapStream('stdout', 'log');
}
```

**Key aspects:**
- Intercepts console methods and stream.write() calls
- Buffers output and routes to parent via process.send()
- Handles circular references in JSON serialization
- Limits output to 100KB to prevent flooding
- Buffers streams to avoid splitting lines mid-output
- Verbose mode controlled by environment variable
- Errors always transmitted regardless of verbosity

### Pattern 11: Module Redirect Resolution for Dev Mode
**Where:** `src/bootstrap-import.ts:22-85`
**What:** ESM loader hook that redirects imports to local node_modules during development.

```typescript
export async function initialize(injectPath: string): Promise<void> {
	// populate mappings

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
			// Determine module format: .mjs is always ESM, .cjs always CJS, otherwise check type field
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

	console.log(`[bootstrap-import] Initialized node_modules redirector for: ${injectPath}`);
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

	// Defer to the next hook in the chain, which would be the
	// Node.js default resolve if this is the last user-specified loader.
	return nextResolve(specifier, context);
}
```

**Key aspects:**
- Pre-computes module mappings during initialize phase
- Respects package.json exports field (Node.js ESM spec)
- Handles conditional exports (import vs default)
- Determines module format (.mjs, .cjs, or type field)
- Silent error handling per-module to prevent full failure
- Resolve hook uses shortCircuit to avoid double-resolution

---

## Critical Dependencies and Modules Required

### Direct Imports in bootstrap-server.ts
- **None** - Only environment mutation, no module imports

### Related Bootstrap Modules
- `bootstrap-node.ts` - Core Node.js runtime setup (191 LOC)
- `bootstrap-esm.ts` - ESM module resolution hooks (113 LOC)
- `bootstrap-meta.ts` - Product/package configuration loading (56 LOC)

### Node.js APIs Used Across Bootstrap Chain
- `process.env` - Environment variable access/mutation
- `process.versions` - Electron detection
- `process.platform` - Platform detection (win32, darwin, linux)
- `process.cwd()` / `process.chdir()` - Working directory management
- `process.kill()` - Parent process monitoring
- `node:module.Module.register()` - Loader hook registration
- `node:module.createRequire()` - CommonJS module creation
- `node:fs` and `node:fs/promises` - File I/O
- `node:path` - Path manipulation
- `Object.defineProperty()` - Wrapping console/streams

---

## Tauri/Rust Porting Implications

### What bootstrap-server.ts Does (7 LOC)
Removes a single Electron-specific environment variable that would interfere with module resolution. This is a **guard operation** that must execute before other modules load.

### Equivalent Rust/Tauri Implementation Considerations

1. **Environment Cleanup Phase**
   - In Rust, this would be part of the initialization sequence before spawning the Node.js worker/server
   - Could be handled in Tauri's pre-app setup or as part of environment preparation for subprocess spawning
   - If using a Node.js VM or worker thread, environment variables would need sanitization before creation

2. **Loader Hook Replacement**
   - The actual module resolution remapping (fs → original-fs) is in bootstrap-esm.ts, not bootstrap-server.ts
   - In a Tauri architecture, this would be handled by:
     - Using Tauri's own file system APIs instead of Node.js fs
     - Or maintaining Node.js workers but pre-configuring their module cache
     - Or using a shim layer that provides fs compatibility without Electron interference

3. **Global State Initialization**
   - bootstrap-esm.ts sets `globalThis._VSCODE_*` values
   - In Tauri, this could be:
     - Passed as initialization parameters to Node.js workers
     - Stored in Tauri's state management system
     - Provided via IPC communication

4. **Process Communication**
   - bootstrap-fork.ts handles parent-child process communication via process.send()
   - In Tauri, this would be replaced with:
     - Tauri's IPC system for main-to-worker communication
     - Or native Rust channels between threads

5. **Module Resolution Hooks**
   - The custom loader hooks in bootstrap-import.ts and bootstrap-esm.ts
   - Would need to be reimplemented as:
     - Tauri plugin hooks
     - Pre-built module cache in Rust
     - Or maintained but simplified Node.js loader hooks

### Summary
The bootstrap chain orchestrates:
1. Environment sanitization (critical before module loading)
2. Module resolution interception (to provide fs compatibility)
3. Global state initialization (configuration/metadata)
4. Process lifecycle management (parent-child communication, crash reporting)
5. Working directory normalization (cross-platform consistency)

A Tauri/Rust port would need equivalent mechanisms in each category, likely shifting responsibility from Node.js module hooks to Tauri plugins and Rust-level process management.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
