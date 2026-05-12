# Partition 76 of 80 — Findings

## Scope
`src/bootstrap-cli.ts/` (1 files, 11 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locator Report: Partition 76/80 - CLI Bootstrap Shim

## Scope
- `src/bootstrap-cli.ts` (1 file, 11 LOC)

## Summary
The CLI bootstrap shim (`src/bootstrap-cli.ts`) is a minimal entry point file containing only environment variable cleanup. The file does not contain argument parsing logic, command-line option handling, or process argument (`process.argv`) inspection. For a Tauri/Rust port, this file represents minimal surface area — only the `VSCODE_CWD` environment variable deletion would need porting to equivalent Rust initialization logic.

### Implementation
- `src/bootstrap-cli.ts` — Minimal bootstrap shim; deletes `VSCODE_CWD` environment variable to prevent working directory inheritance issues; no CLI argument parsing or option handling present

### Notable Findings
- No `process.argv` usage found (contrary to seed pattern expectation)
- No explicit command-line option parsing in this file
- Actual CLI argument handling likely resides in separate `src/cli.ts` module (outside scope)
- For Tauri/Rust porting: this file is trivial and would require minimal effort (simple env cleanup on startup)

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `src/bootstrap-cli.ts` (11 LOC)

---

### Per-File Notes

#### `src/bootstrap-cli.ts`

- **Role:** Minimal process-level bootstrap shim that runs as the very first import in the CLI process. Its sole responsibility is to delete the `VSCODE_CWD` environment variable from the current process's environment before any other module has a chance to read it. The file contains no functions, no exports, no imports, and no conditional logic — it is entirely composed of a single top-level side-effectful statement.

- **Key symbols:**
  - `process.env['VSCODE_CWD']` (`src/bootstrap-cli.ts:11`) — The environment variable being unconditionally deleted at module load time.

- **Control flow:** There is no branching or looping. The single statement `delete process.env['VSCODE_CWD']` (`src/bootstrap-cli.ts:11`) executes unconditionally and synchronously at module evaluation time. Execution is complete after this one statement; the module exports nothing and has no lifecycle hooks.

- **Data flow:** The only data involved is the value held in `process.env['VSCODE_CWD']` at the moment the module is evaluated. The `delete` operator removes the key from the `process.env` object entirely. No value is read, returned, or passed anywhere; the effect is purely a mutation (deletion) of the process environment inherited from the shell that launched the process.

  The comment at `src/bootstrap-cli.ts:6–10` references [GitHub issue #126399](https://github.com/microsoft/vscode/issues/126399), describing the scenario: running `code .` from a shell that already has `VSCODE_CWD` set (because a parent `code` process previously wrote it) would cause the CLI child process to inherit the wrong working directory. Deleting the variable before any downstream code reads it breaks that inheritance chain.

- **Dependencies:** None. No imports whatsoever. Relies only on the Node.js global `process` object, which is available in all Node.js module scopes without importing.

---

### Cross-Cutting Synthesis

`src/bootstrap-cli.ts` is the first import statement in `src/cli.ts:6`, annotated with the comment `// this MUST come before other imports as it changes global state`. This ordering constraint exists because `src/bootstrap-node.ts:42–43` (executed later in the same process via `src/cli.ts:7`) reads `process.env['VSCODE_CWD']` and, only if it is absent, sets it to `process.cwd()`. If `bootstrap-cli.ts` did not first delete any shell-inherited value, the stale parent-process value would survive into `bootstrap-node.ts`'s guard check and would never be overwritten. After `bootstrap-node.ts` sets the variable, it is subsequently consumed at `src/vs/base/common/process.ts:29`, `src/vs/platform/environment/node/userDataPath.ts:19`, `src/vs/base/parts/sandbox/electron-browser/preload.ts:207`, and `src/vs/server/node/remoteTerminalChannel.ts:63`. The full lifecycle is: delete (bootstrap-cli) → re-set from fresh `process.cwd()` (bootstrap-node) → read throughout the application. For a Tauri/Rust port, the equivalent is clearing any inherited `VSCODE_CWD` from `std::env` at the earliest point in `main()`, before any downstream initialization reads it.

---

### Out-of-Partition References

- `src/cli.ts:6` — Imports `./bootstrap-cli.js` as its first and most-constrained import; the inline comment makes the ordering requirement explicit.
- `src/bootstrap-node.ts:42–43` — Reads `process.env['VSCODE_CWD']` and sets it to `process.cwd()` only when absent; relies on `bootstrap-cli.ts` having already deleted any stale inherited value.
- `src/vs/base/common/process.ts:29` — Downstream consumer: `cwd()` returns `process.env['VSCODE_CWD'] || process.cwd()`.
- `src/vs/platform/environment/node/userDataPath.ts:19` — Downstream consumer: `const cwd = process.env['VSCODE_CWD'] || process.cwd()`.
- `src/vs/base/parts/sandbox/electron-browser/preload.ts:207` — Downstream consumer in the renderer/sandbox process.
- `src/vs/server/node/remoteTerminalChannel.ts:63` — Downstream consumer in the remote server path: `return env['VSCODE_CWD']`.
- `extensions/copilot/src/util/vs/base/common/process.ts:31` — Extension-local copy of the same pattern.
- `eslint.config.js:1998` — Lint configuration lists `bootstrap-cli.ts` as one of the root-level entry-point files subject to shared lint rules.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Bootstrap-CLI Patterns: VS Code Process Initialization

Research partition 76 of 80 examines `src/bootstrap-cli.ts` (11 LOC) in context of porting VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust. The file is part of a multi-stage bootstrap system responsible for process initialization, environment setup, and module resolution.

## Patterns Identified

#### Pattern: Early Environment Variable Cleanup
**Where:** `src/bootstrap-cli.ts:11`
**What:** Delete process environment variables early to prevent shell contamination and working directory leaks.

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

**Variations / call-sites:** 
- `src/server-cli.ts:23` — Deletes `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH` in production
- `src/vs/code/node/cli.ts:66` — Deletes `ELECTRON_RUN_AS_NODE` when spawning tunnel processes
- Pattern appears in multiple bootstrap stages to isolate environment state

#### Pattern: Environment Variable Initialization Chain
**Where:** `src/cli.ts:14-20`
**What:** Sequential setup of process environment flags before loading core modules; must execute before ESM bootstrap.

```typescript
// NLS
const nlsConfiguration = await resolveNLSConfiguration({ userLocale: 'en', osLocale: 'en', commit: product.commit, userDataPath: '', nlsMetadataPath: import.meta.dirname });
process.env['VSCODE_NLS_CONFIG'] = JSON.stringify(nlsConfiguration); // required for `bootstrap-esm` to pick up NLS messages

// Enable portable support
configurePortable(product);

// Signal processes that we got launched as CLI
process.env['VSCODE_CLI'] = '1';
```

**Variations / call-sites:**
- `src/server-cli.ts:14-15` — Sets `VSCODE_NLS_CONFIG` before ESM bootstrap
- `src/main.ts:204-205` — Sets both `VSCODE_NLS_CONFIG` and `VSCODE_CODE_CACHE_PATH`
- Pattern consistently places NLS config first, then flags affecting module resolution

#### Pattern: Ordered Import Sequencing with Comments
**Where:** `src/cli.ts:6-8`
**What:** Bootstrap files must import in strict order before any other code; documented with MUST comments.

```typescript
import './bootstrap-cli.js'; // this MUST come before other imports as it changes global state
import { configurePortable } from './bootstrap-node.js';
import { bootstrapESM } from './bootstrap-esm.js';
```

**Variations / call-sites:**
- `src/server-cli.ts:6` — Same pattern: `import './bootstrap-server.js'; // this MUST come before other imports`
- Pattern enforces that environment state is established before module loading

#### Pattern: Current Working Directory Preservation
**Where:** `src/bootstrap-node.ts:35-55`
**What:** Capture process CWD early and restore platform-specific behavior to prevent navigation issues across processes.

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

**Variations / call-sites:**
- `src/main.ts:57` — Calls `getUserDataPath(args, product.nameShort ?? 'code-oss-dev')` with CWD context
- `src/vs/base/parts/sandbox/electron-browser/preload.ts:207` — Retrieves stored `VSCODE_CWD` at runtime

#### Pattern: Conditional Development Path Injection
**Where:** `src/bootstrap-node.ts:62-74`
**What:** Hook module resolution for development builds to redirect node_modules lookups, avoiding electron-compiled modules in CLI contexts.

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

**Variations / call-sites:**
- `src/server-cli.ts:20-21` — Sets and calls with `remote/node_modules` path
- `src/vs/code/node/cli.ts:73` — Spawns `cargo run` in dev mode for tunnel CLI

#### Pattern: Electron Detection and Conditional Module Resolution
**Where:** `src/bootstrap-esm.ts:13-30`
**What:** Register ES module hooks to remap filesystem access based on Electron context at the loader level.

```typescript
// Install a hook to module resolution to map 'fs' to 'original-fs'
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

**Variations / call-sites:**
- Pattern reflects conditional behavior: Electron processes use `original-fs`, pure Node processes use standard `fs`
- Detected via both `process.env['ELECTRON_RUN_AS_NODE']` and `process.versions['electron']`

#### Pattern: Product Configuration Dynamic Loading
**Where:** `src/bootstrap-meta.ts:11-27`
**What:** Lazy-load product.json at module initialization time with build-time substitution fallback; support development overrides.

```typescript
const require = createRequire(import.meta.url);

let productObj: Partial<IProductConfiguration> & { BUILD_INSERT_PRODUCT_CONFIGURATION?: string } = { BUILD_INSERT_PRODUCT_CONFIGURATION: 'BUILD_INSERT_PRODUCT_CONFIGURATION' }; // DO NOT MODIFY, PATCHED DURING BUILD
if (productObj['BUILD_INSERT_PRODUCT_CONFIGURATION']) {
	productObj = require('../product.json'); // Running out of sources
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

**Variations / call-sites:**
- Used throughout bootstrap chain: `src/cli.ts:10`, `src/server-cli.ts:11`, `src/main.ts:14`
- Supports both build-time bundling and source development modes

## Summary

The bootstrap-cli.ts file exemplifies VS Code's multi-stage process initialization pattern. The 11 LOC file performs the critical first task—deleting the `VSCODE_CWD` environment variable before other code executes—to prevent shell contamination. This is part of a coordinated sequence where:

1. **bootstrap-cli.ts** (11 LOC): Immediate environment cleanup
2. **bootstrap-node.ts**: CWD setup, SIGPIPE handling, module injection hooks
3. **bootstrap-meta.ts**: Product configuration loading with build-time substitution
4. **bootstrap-esm.ts**: ES module hook registration and NLS setup
5. **cli.ts / server-cli.ts / main.ts**: Environment configuration and module loading

Key porting concerns for Tauri/Rust include:

- **Environment isolation**: Modern process models must prevent parent shell contamination
- **Module resolution hooks**: Both static (build-time) and dynamic (runtime) configuration paths
- **Platform-specific behavior**: Windows CWD handling differs fundamentally
- **Development vs. production modes**: Clear conditional paths based on `VSCODE_DEV` variable
- **IPC and subprocess communication**: Electron's environment passing patterns must translate to Tauri message passing

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
