# Pattern Finder Research: bootstrap-cli.ts

## Partition 75 of 79

**Target File**: `src/bootstrap-cli.ts` (11 LOC)  
**Pattern Category**: Pre-AMD Setup / CLI Bootstrap Shim  
**Scope**: Early initialization hooks before AMD module loading  
**Migration Target**: Rust host (Tauri) can drop these entirely

---

## Pattern 1: Early Environment Cleanup

**Found in**: `src/bootstrap-cli.ts:11`

```typescript
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

// Delete `VSCODE_CWD` very early. We have seen
// reports where `code .` would use the wrong
// current working directory due to our variable
// somehow escaping to the parent shell
// (https://github.com/microsoft/vscode/issues/126399)
delete process.env['VSCODE_CWD'];
```

**Used for**: Pre-emptive environment variable cleanup to prevent shell inheritance issues  
**Context**: This executes at the very start of CLI process before any AMD modules load  
**Key aspects**:
- Runs immediately on module import (no function wrapper)
- Clears a single, high-risk environment variable
- Documented bug reference (GitHub issue #126399)
- Prevents cascading failures from inherited state

---

## Pattern 2: Comparative Bootstrap Strategy

The CLI bootstrap chain differs from other entry points:

### CLI Bootstrap Chain (src/cli.ts:6-26)
**Found in**: `src/cli.ts:6-26`

```typescript
import './bootstrap-cli.js'; // this MUST come before other imports as it changes global state
import { configurePortable } from './bootstrap-node.js';
import { bootstrapESM } from './bootstrap-esm.js';
import { resolveNLSConfiguration } from './vs/base/node/nls.js';
import { product } from './bootstrap-meta.js';

// NLS
const nlsConfiguration = await resolveNLSConfiguration({ userLocale: 'en', osLocale: 'en', commit: product.commit, userDataPath: '', nlsMetadataPath: import.meta.dirname });
process.env['VSCODE_NLS_CONFIG'] = JSON.stringify(nlsConfiguration);

// Enable portable support
configurePortable(product);

// Signal processes that we got launched as CLI
process.env['VSCODE_CLI'] = '1';

// Bootstrap ESM
await bootstrapESM();

// Load Server
await import('./vs/code/node/cli.js');
```

**Key aspects**:
- Explicit ordering: `bootstrap-cli.js` first (environment cleanup)
- Then `bootstrap-node.js` functions (portable mode, module paths)
- Then `bootstrap-esm.js` async setup (NLS, imports)
- Finally domain-specific CLI loading

---

## Pattern 3: Server Bootstrap Variant

**Found in**: `src/bootstrap-server.ts:7`

```typescript
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

// Keep bootstrap-esm.js from redefining 'fs'.
delete process.env['ELECTRON_RUN_AS_NODE'];
```

**Used for**: Prevent Electron mode from interfering with Node.js fs module resolution  
**Context**: Server processes don't use Electron  
**Key aspects**:
- Single-statement cleanup file
- Pre-emptive guard against auto-configuration
- Signals ESM bootstrap to skip Electron-specific handling

---

## Pattern 4: Full Bootstrap Context

**Found in**: `src/server-main.ts:6-23`

```typescript
import './bootstrap-server.js'; // this MUST come before other imports as it changes global state
import * as path from 'node:path';
import * as http from 'node:http';
import type { AddressInfo } from 'node:net';
import * as os from 'node:os';
import * as readline from 'node:readline';
import { performance } from 'node:perf_hooks';
import minimist from 'minimist';
import { devInjectNodeModuleLookupPath, removeGlobalNodeJsModuleLookupPaths } from './bootstrap-node.js';
import { bootstrapESM } from './bootstrap-esm.js';
import { resolveNLSConfiguration } from './vs/base/node/nls.js';
import { product } from './bootstrap-meta.js';
import * as perf from './vs/base/common/performance.js';
import { INLSConfiguration } from './vs/nls.js';
import { IServerAPI } from './vs/server/node/remoteExtensionHostAgentServer.js';

perf.mark('code/server/start');
(globalThis as { vscodeServerStartTime?: number }).vscodeServerStartTime = performance.now();
```

**Key aspects**:
- Performance markers set early for telemetry
- Global state mutations (comments explicitly warn about this)
- Three-stage bootstrap: cleanup → configuration → ESM init

---

## Pattern 5: Node Bootstrap Functions

**Found in**: `src/bootstrap-node.ts:35-55`

```typescript
// Setup current working directory in all our node & electron processes
// - Windows: call `process.chdir()` to always set application folder as cwd
// -  all OS: store the `process.cwd()` inside `VSCODE_CWD` for consistent lookups
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

**Related context** (why `bootstrap-cli.ts` deletes this):
- `bootstrap-cli.ts` explicitly deletes the `VSCODE_CWD` that this function sets
- This ensures CLI gets a clean environment when spawned

---

## Pattern 6: Environment Variable Guard Pattern

**Found in**: `src/bootstrap-node.ts:62-74`

```typescript
/**
 * Add support for redirecting the loading of node modules
 *
 * Note: only applies when running out of sources.
 */
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

**Pattern**: Conditional initialization based on environment flags  
**Related flags** (from codebase):
- `VSCODE_DEV` - development mode
- `VSCODE_CLI` - CLI launcher indicator (set by cli.ts)
- `VSCODE_CWD` - working directory (deleted early by bootstrap-cli.ts)
- `VSCODE_NLS_CONFIG` - localization config
- `VSCODE_HANDLES_SIGPIPE` - signal handling state
- `VSCODE_PORTABLE` - portable mode indicator
- `VSCODE_ESM_ENTRYPOINT` - ESM module entry point

---

## Pattern 7: Module Resolution Override

**Found in**: `src/bootstrap-node.ts:76-128`

```typescript
export function removeGlobalNodeJsModuleLookupPaths(): void {
	if (typeof process?.versions?.electron === 'string') {
		return; // Electron disables global search paths in https://github.com/electron/electron/blob/3186c2f0efa92d275dc3d57b5a14a60ed3846b0e/shell/common/node_bindings.cc#L653
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

	const originalNodeModulePaths = Module._nodeModulePaths;
	Module._nodeModulePaths = function (from: string): string[] {
		let paths: string[] = originalNodeModulePaths(from);
		if (!isWindows) {
			return paths;
		}

		// On Windows, remove drive(s) and users' home directory from search paths,
		// UNLESS 'from' is explicitly set to one of those.
		const isDrive = (p: string) => p.length >= 3 && p.endsWith(':\\');

		if (!isDrive(from)) {
			paths = paths.filter(p => !isDrive(path.dirname(p)));
		}

		if (process.env.HOMEDRIVE && process.env.HOMEPATH) {
			const userDir = path.dirname(path.join(process.env.HOMEDRIVE, process.env.HOMEPATH));

			const isUsersDir = (p: string) => path.relative(p, userDir).length === 0;

			// Check if 'from' is the same as 'userDir'
			if (!isUsersDir(from)) {
				paths = paths.filter(p => !isUsersDir(path.dirname(p)));
			}
		}

		return paths;
	};
}
```

**Pattern**: Monkey-patching core Node.js module resolution  
**Purpose**: Sandbox module loading to prevent polluting global node_modules paths  
**Platform-specific**:
- Windows: Filters out drive roots and user home directory
- Non-Windows: Uses defaults

---

## Rust Host Migration Implications

### What Can Be Dropped

1. **Environment variable cleanup** (`bootstrap-cli.ts` pattern)
   - Rust host doesn't inherit TypeScript/Electron quirks
   - No process.env escape issues
   - Tauri/IPC can manage environment cleanly

2. **Module resolution overrides**
   - Node.js-specific (AMD + CommonJS interop)
   - Rust uses declarative module system
   - No monkey-patching needed

3. **Portable mode detection**
   - Currently reads file existence (`fs.existsSync`)
   - Can be runtime detection in Rust
   - Cleaner separation of concerns

### What Requires Port

1. **Working directory normalization** (platform-specific)
   - Windows vs. Unix behavior divergence
   - Port as Rust startup logic with conditional compilation
   - Store cwd in IPC bridge for plugin access

2. **SIGPIPE handling**
   - Port signal handler setup to Rust main
   - Tauri provides signal hooks

3. **NLS/Localization configuration**
   - Keep environment variable bridge pattern
   - IPC message for plugin initialization

4. **Feature flags environment variables**
   - `VSCODE_DEV`, `VSCODE_CLI`, `VSCODE_PORTABLE`
   - Port as Rust enums + IPC bridge

---

## Summary

The `bootstrap-cli.ts` shim represents the minimal early-stage cleanup pattern in VS Code's TypeScript/Node.js stack. Its single-line operation (deleting `VSCODE_CWD`) prevents inherited environment state from cascading into subprocess spawning failures. 

For Rust/Tauri migration:
- **Environmental cleanup**: No longer needed (no shell escape issues)
- **Module paths**: Dropped (Rust has static module system)
- **Working directory**: Port as platform-conditional startup in Rust
- **Signal handling**: Port to Rust signal API
- **Feature flags**: Convert to Rust config + IPC bridge

The three-file bootstrap pattern (`bootstrap-cli.ts` → `bootstrap-node.ts` → `bootstrap-esm.ts`) can consolidate into a single Rust initialization phase with structured config.

