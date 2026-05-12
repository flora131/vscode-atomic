# Partition 55 of 80 — Findings

## Scope
`src/bootstrap-esm.ts/` (1 files, 112 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# ESM Bootstrap / Loader Strategy - File Locator (Partition 55)

## Implementation

### Core Bootstrap Files
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts` (112 LOC) — ESM module bootstrapping with NLS setup
  - Installs Node.js module resolution hooks to map `fs` to `original-fs` in Electron contexts
  - Sets up global VS Code configuration (`_VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON`, `_VSCODE_FILE_ROOT`)
  - Handles asynchronous NLS (National Language Support) loading from environment configuration
  - Exports `bootstrapESM()` async function for module initialization

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-meta.ts` (31 LOC) — Static metadata loader
  - Loads `product.json` and `package.json` using CommonJS require
  - Handles build-time configuration injection via marker strings
  - Applies product overrides from `product.overrides.json` in dev mode
  - Exports `product` and `pkg` objects globally

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-node.ts` (191 LOC) — Node.js runtime setup
  - Configures working directory handling across platforms (Windows-specific logic)
  - Provides `devInjectNodeModuleLookupPath()` to register custom module resolution hooks
  - Implements `removeGlobalNodeJsModuleLookupPaths()` to restrict module search paths
  - Provides `configurePortable()` for portable VS Code installations
  - Exports utilities for module path injection and global path cleanup

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-import.ts` (102 LOC) — Custom module resolver hook
  - Implements Node.js loader hook protocol (`initialize`, `resolve` functions)
  - Maps package dependencies to node_modules via file URL paths
  - Handles conditional exports (ESM `.import`, fallback to `.main`)
  - Distinguishes module format (ESM vs CommonJS) via `.mjs`, `.cjs`, or `type: "module"` field
  - Redirects specifiers to actual filesystem locations, enabling dev-mode node_modules overrides

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-fork.ts` (230 LOC) — Child process bootstrap
  - Pipes logging from forked process back to parent via `process.send()`
  - Handles uncaught exceptions and unhandled promise rejections
  - Monitors parent process termination and exits accordingly
  - Configures crash reporter for Electron utility processes
  - Chains to `bootstrapESM()` before loading ESM entrypoint specified by `VSCODE_ESM_ENTRYPOINT` env var

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-server.ts` (8 LOC) — Server mode configuration
  - Removes Electron-specific behavior by deleting `ELECTRON_RUN_AS_NODE` environment variable

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-cli.ts` (12 LOC) — CLI mode configuration
  - Cleans up `VSCODE_CWD` environment variable to prevent cross-shell contamination

### Entry Points Using Bootstrap
- `/home/norinlavaee/projects/vscode-atomic/src/main.ts` — Electron main process entry
  - Imports `bootstrapESM()` from `bootstrap-esm.js`
  - Imports `configurePortable()` from `bootstrap-node.js`
  - Imports `product` from `bootstrap-meta.js`

- `/home/norinlavaee/projects/vscode-atomic/src/server-main.ts` — Server/remote mode entry
- `/home/norinlavaee/projects/vscode-atomic/src/server-cli.ts` — Server CLI entry
- `/home/norinlavaee/projects/vscode-atomic/src/cli.ts` — Local CLI entry

### Legacy Loader Infrastructure
- `/home/norinlavaee/projects/vscode-atomic/build/loader.min` — Minified AMD loader
  - Legacy AMD (Asynchronous Module Definition) module system
  - Contains `AMDLoader` module manager for module resolution and loading
  - Supports both Node.js and browser environments
  - Includes script loading via DOM (`<script>` tags) or Node.js `vm.Script`
  - Handles cached data for V8 code caching in Node.js
  - Supports plugins and custom resolution paths

## Configuration

### Environment Variables Used in Bootstrap
- `ELECTRON_RUN_AS_NODE` — Triggers fs → original-fs mapping in bootstrap-esm.ts
- `VSCODE_NLS_CONFIG` — JSON config for NLS setup (languagePack, defaultMessagesFile, resolvedLanguage)
- `VSCODE_DEV` — Development mode flag (disables NLS, enables product overrides)
- `VSCODE_PORTABLE` — Path to portable data directory
- `VSCODE_CWD` — Cached current working directory for consistent lookups
- `VSCODE_HANDLES_SIGPIPE` — Signal handling configuration
- `VSCODE_PARENT_PID` — Parent process monitoring for forked processes
- `VSCODE_PIPE_LOGGING` — Enable logging via parent process pipe
- `VSCODE_HANDLES_UNCAUGHT_ERRORS` — Error handling delegation
- `VSCODE_CRASH_REPORTER_PROCESS_TYPE` — Crash reporter configuration
- `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH` — Dev mode module path override
- `VSCODE_ESM_ENTRYPOINT` — Dynamic ESM module entry point for forked processes
- `VSCODE_VERBOSE_LOGGING` — Verbose console logging flag

### Build-Time Markers
- `BUILD_INSERT_PRODUCT_CONFIGURATION` — Replaced during build with actual product.json
- `BUILD_INSERT_PACKAGE_CONFIGURATION` — Replaced during build with actual package.json

### Runtime Globals Injected by Bootstrap
- `globalThis._VSCODE_PRODUCT_JSON` — Immutable product configuration object
- `globalThis._VSCODE_PACKAGE_JSON` — Immutable package metadata
- `globalThis._VSCODE_FILE_ROOT` — Current module's directory (via `import.meta.dirname`)
- `globalThis._VSCODE_NLS_LANGUAGE` — Resolved language for NLS
- `globalThis._VSCODE_NLS_MESSAGES` — Loaded translation messages object

## Types / Interfaces

### IProductConfiguration (referenced in bootstrap-meta.ts)
- Located in `src/vs/base/common/product.js`
- Used for type annotations in TypeScript bootstrap files

### INLSConfiguration (referenced in bootstrap-esm.ts)
- Located in `src/vs/nls.js`
- Defines NLS config structure with languagePack, defaultMessagesFile, resolvedLanguage

## Notable Clusters

### Module Resolution Layers
1. **ESM Module System** (bootstrap-esm.ts) — Top-level ESM entry, fs mapping
2. **Node.js Hook System** (bootstrap-import.ts) — Custom loader hook for dev mode
3. **Node Module Paths** (bootstrap-node.ts) — Global and local path manipulation
4. **AMD Loader** (loader.min) — Legacy fallback for web/AMD modules

### Process Initialization Sequence
1. `main.ts` / `server-main.ts` / `cli.ts` — Entry point
2. `bootstrap-meta.ts` — Load static configuration
3. `bootstrap-node.ts` — Configure working directory and portable mode
4. `bootstrap-esm.ts` — Set up NLS and global configuration
5. `bootstrap-fork.ts` (if child process) — Set up logging and error handling
6. Dynamic entrypoint — Load application code

### NLS (Internationalization) Pipeline
- `VSCODE_NLS_CONFIG` env var parsed
- `bootstrap-esm.ts::setupNLS()` reads language pack from disk
- `_VSCODE_NLS_MESSAGES` populated with translations
- Fallback to default English messages on errors
- Corruption marker file written on load failure for cache invalidation

### Portable Mode Support
- `bootstrap-node.ts::configurePortable()` detects portable installations
- Redirects temp directory (`TMP`, `TEMP`, `TMPDIR`) to portable data folder
- Sets `VSCODE_PORTABLE` environment variable

---

## Portability Notes for Tauri/Rust Replacement

The ESM bootstrap and loader strategy handles several critical concerns that a Tauri/Rust port would need to address:

### 1. Module Resolution
The current implementation uses Node.js ESM loader hooks (`register()`) to dynamically intercept module resolution. Tauri/Rust would need to:
- Pre-resolve all module paths at build time (static linking)
- Replace dynamic `devInjectNodeModuleLookupPath` with build-time configuration
- Eliminate the need for runtime module hooks via bundling

### 2. Global Configuration Injection
Currently done via `globalThis` mutations in synchronous bootstrap code. Rust would need to:
- Expose configuration through a structured API instead of globals
- Ensure NLS configuration is loaded before application code runs
- Consider pre-loading translations at compile time

### 3. NLS (Localization) Loading
Async file I/O pattern reading language packs. Rust implementation should:
- Load translations at binary startup before running JS/TS code
- Consider embedding translations in the binary or using memory-mapped files
- Handle corruption detection differently (not via marker files)

### 4. Process Management & Forking
`bootstrap-fork.ts` handles child process setup including logging redirection and error handling. Rust would:
- Use native threading or subprocess APIs instead of Node.js forking
- Replace `process.send()` piping with native IPC
- Simplify exception handling (no Promise rejections in Rust)

### 5. Portable Mode Detection
Runtime filesystem checks. Rust should:
- Detect portable mode at binary startup
- Use environment variables or configuration files instead of disk probing
- Pre-compute paths at initialization

### 6. fs → original-fs Mapping
Electron-specific workaround for fs access. Rust implementation:
- Directly access filesystem without need for mapping
- May need wrapper abstractions for file access in some contexts

The loader is essentially a compatibility layer for ES modules in a Node.js/Electron environment. A Tauri/Rust rewrite would simplify this significantly since Rust has native module support and more direct filesystem/process control.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts` (112 LOC) — primary scope

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts`

**Purpose**

This file is the ESM-layer bootstrap for the VS Code Node process. It runs at startup (before application code) and is responsible for: (1) patching Node's ESM module resolution to redirect `fs` to Electron's `original-fs` when running under Electron, (2) populating a set of well-known `globalThis` properties that downstream modules read without importing, and (3) performing async NLS (i18n message) loading. It exports a single async function `bootstrapESM()` that orchestrates the NLS load.

---

**Imports (lines 6–11)**

| Line | Import | Role |
|------|--------|------|
| 6 | `import * as fs from 'node:fs'` | Used only inside `doSetupNLS()` to read NLS message JSON files from disk via `fs.promises.readFile` / `fs.promises.writeFile` |
| 7 | `import { register } from 'node:module'` | Node 20+ API that registers a custom ESM loader hook at runtime |
| 8 | `import { product, pkg } from './bootstrap-meta.js'` | Provides pre-loaded `product.json` and `package.json` objects (built or sourced from disk) |
| 9 | `import './bootstrap-node.js'` | Side-effect-only import; executes Node-level bootstrap setup (no symbol binding) |
| 10 | `import * as performance from './vs/base/common/performance.js'` | Used to emit `code/willLoadNls` and `code/didLoadNls` performance marks |
| 11 | `import { INLSConfiguration } from './vs/nls.js'` | Type-only import of the interface describing the parsed `VSCODE_NLS_CONFIG` shape |

---

**Module-resolution hook — Electron `fs` → `original-fs` (lines 14–30)**

The outermost guard `if (process.env['ELECTRON_RUN_AS_NODE'] || process.versions['electron'])` at line 14 detects that the process is running inside Electron (either as a Node subprocess or as the renderer/main process). Inside the branch:

- A small inline ESM loader hook is composed as a template literal string `jsCode` (lines 15–28). The hook exports an async `resolve` function that intercepts any `specifier === 'fs'` and returns `{ format: 'builtin', shortCircuit: true, url: 'node:original-fs' }`, routing all `import 'fs'` calls to Electron's `original-fs` built-in instead of the standard Node `fs`. For all other specifiers the hook calls `nextResolve(specifier, context)` to continue normal resolution.
- The hook string is base64-encoded and registered via `register(`data:text/javascript;base64,${...}`, import.meta.url)` at line 29, using a `data:` URL as the loader module so no separate file is needed.

---

**Global property setup (lines 33–35)**

Three `globalThis` properties are written immediately (synchronously) after the hook registration:

| Line | Global | Value |
|------|--------|-------|
| 33 | `globalThis._VSCODE_PRODUCT_JSON` | Shallow copy of `product` (from `bootstrap-meta.ts`) |
| 34 | `globalThis._VSCODE_PACKAGE_JSON` | Shallow copy of `pkg` (from `bootstrap-meta.ts`) |
| 35 | `globalThis._VSCODE_FILE_ROOT` | `import.meta.dirname` — the absolute directory of this file, i.e. the source/dist root |

These globals are consumed by `src/vs/platform/product/common/product.ts:28–46` (reads `_VSCODE_PRODUCT_JSON` and `_VSCODE_PACKAGE_JSON`) and by `src/vs/base/common/network.ts:366–367` and `src/vs/platform/webWorker/browser/webWorkerServiceImpl.ts:105` (reads `_VSCODE_FILE_ROOT`).

---

**NLS helpers (lines 37–106)**

The NLS region implements a lazy singleton load using a module-level `setupNLSResult` variable (line 39, type `Promise<INLSConfiguration | undefined> | undefined`).

`setupNLS()` (lines 41–47) is a synchronous gate that creates the promise exactly once by calling `doSetupNLS()` on first invocation and caching it in `setupNLSResult`. Subsequent calls return the same promise.

`doSetupNLS()` (lines 49–104) is the async implementation:

1. **Performance mark** — emits `performance.mark('code/willLoadNls')` at line 50.
2. **Parse env config** (lines 55–68) — reads `process.env['VSCODE_NLS_CONFIG']` via `JSON.parse`. From the parsed `nlsConfig` (`INLSConfiguration`), it derives `messagesFile` by preferring `nlsConfig.languagePack.messagesFile` (line 59) and falling back to `nlsConfig.defaultMessagesFile` (line 61). Sets `globalThis._VSCODE_NLS_LANGUAGE = nlsConfig?.resolvedLanguage` at line 64.
3. **Dev/missing-file short-circuit** (lines 70–75) — if `process.env['VSCODE_DEV']` is set or `messagesFile` is absent, returns `undefined` without loading any messages.
4. **Primary message load** (lines 77–99) — reads `messagesFile` with `fs.promises.readFile`, parses JSON, and assigns the result to `globalThis._VSCODE_NLS_MESSAGES` (line 78). On error:
   - Writes a `'corrupted'` marker to `nlsConfig.languagePack.corruptMarkerFile` (lines 83–88) so the next startup rebuilds the language pack cache.
   - Falls back to `nlsConfig.defaultMessagesFile` (lines 92–98) and sets `globalThis._VSCODE_NLS_MESSAGES` to the English messages if that secondary read succeeds.
5. **Performance mark** — emits `performance.mark('code/didLoadNls')` at line 101.
6. Returns `nlsConfig` (line 103).

The two globals written here (`_VSCODE_NLS_MESSAGES`, `_VSCODE_NLS_LANGUAGE`) are read by `src/vs/nls.ts:7` and `src/vs/nls.ts:11` in their respective accessor functions, making them the effective i18n data source for all downstream `localize()` calls.

---

**Exported entry point (lines 108–112)**

```typescript
export async function bootstrapESM(): Promise<void> {
    // NLS
    await setupNLS();
}
```

`bootstrapESM()` is the sole export of the file. It awaits `setupNLS()` and therefore ensures NLS messages are loaded before the caller (the main entry point) proceeds. The `globalThis` globals are already set synchronously before this function is ever called, so callers only need to await for NLS readiness.

---

**Data flow summary**

```
process.env['VSCODE_NLS_CONFIG']   ──parse──►  nlsConfig (INLSConfiguration)
  nlsConfig.languagePack.messagesFile  ──readFile──►  globalThis._VSCODE_NLS_MESSAGES
  nlsConfig.resolvedLanguage           ──assign──►   globalThis._VSCODE_NLS_LANGUAGE

bootstrap-meta.ts { product, pkg }  ──spread──►  globalThis._VSCODE_PRODUCT_JSON
                                                  globalThis._VSCODE_PACKAGE_JSON
import.meta.dirname                  ──assign──►  globalThis._VSCODE_FILE_ROOT
```

---

### Cross-Cutting Synthesis

`bootstrap-esm.ts` occupies a well-defined slice of the VS Code startup sequence: it is the first ESM-aware layer that runs after the raw Node/process setup performed by `bootstrap-node.ts` (imported as a side effect at line 9). It depends on `bootstrap-meta.ts` only for the `product`/`pkg` objects that were loaded (or built-in substituted) before ESM resolution even began. The file deliberately avoids importing any platform service or workbench code — all downstream modules instead read the five `globalThis` properties it writes, creating a one-way dependency edge from the entire VS Code module graph back to this file. The Electron hook ensures that any ESM `import 'fs'` in the process (including in lazily loaded extensions) transparently resolves to `original-fs`, which Electron exposes to avoid conflicts with its patched Node version. The `bootstrapESM()` export is the single await point through which callers (typically the main entry point) synchronize on NLS readiness before the workbench or server code executes.

For a Tauri/Rust port: the `register()` + data-URL hook mechanism, `import.meta.dirname`, and the `original-fs` redirect are all Node/Electron-specific. The NLS loading pattern (env var → JSON file → globalThis) and the product/package global pattern could be preserved but would need a different injection mechanism in a Tauri context.

---

### Out-of-Partition References

The following files are referenced by or directly interact with `bootstrap-esm.ts` but are covered by other partitions:

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-meta.ts` — exports `product` and `pkg`; handles build-time patching vs. source-mode `require()` of `product.json` / `package.json`
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-node.ts` — side-effect import at line 9; performs Node-level process setup
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-import.ts` — sibling ESM bootstrap file (not imported here but part of the bootstrap family)
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-fork.ts` — sibling; bootstraps forked child processes
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-server.ts` — sibling; server-side bootstrap
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-cli.ts` — sibling; CLI entrypoint bootstrap
- `/home/norinlavaee/projects/vscode-atomic/src/vs/nls.ts` — defines `INLSConfiguration` (imported as type at line 11); its runtime accessor functions at lines 7 and 11 read `globalThis._VSCODE_NLS_MESSAGES` and `globalThis._VSCODE_NLS_LANGUAGE` written by `doSetupNLS()`
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/common/performance.ts` — provides `performance.mark()` used at lines 50 and 101
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/common/product.ts` (type source via `bootstrap-meta.ts`) — `IProductConfiguration` interface type
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/product/common/product.ts` — consumer of `globalThis._VSCODE_PRODUCT_JSON` and `globalThis._VSCODE_PACKAGE_JSON` at lines 28–46
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/common/network.ts` — consumer of `globalThis._VSCODE_FILE_ROOT` at lines 366–367
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/webWorker/browser/webWorkerServiceImpl.ts` — forwards `_VSCODE_NLS_MESSAGES`, `_VSCODE_NLS_LANGUAGE`, `_VSCODE_FILE_ROOT` into web worker globals at lines 103–105
- `/home/norinlavaee/projects/vscode-atomic/src/vs/amdX.ts` — reads `globalThis._VSCODE_PRODUCT_JSON` at lines 207 and 232

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder Report 55: ESM Bootstrap & Module Resolution Patterns

**Scope**: `src/bootstrap-esm.ts` and related bootstrap infrastructure (112 LOC core file)

**Research Question**: Port VS Code core IDE from TypeScript/Electron to Tauri/Rust - identify ESM bootstrap, dynamic import, NLS injection, and loader configuration patterns that Tauri/Rust will replace with its own asset pipeline.

---

## Pattern Examples: ESM Bootstrap Architecture

### Pattern 1: Node.js Module Resolution Hook Registration
**Found in**: `src/bootstrap-esm.ts:13-30`
**Used for**: Shimming 'fs' module to 'original-fs' in Electron contexts

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

**Key aspects**:
- Uses Node.js `register()` API from `node:module` (introduced in Node 20.6.0)
- Injects resolver hook as base64-encoded data URL for inline execution
- Conditional registration based on Electron detection
- Intercepts module specifiers before default resolution chain
- Used to provide 'original-fs' fallback when Electron hooks standard fs

### Pattern 2: GlobalThis Configuration Injection
**Found in**: `src/bootstrap-esm.ts:32-35`
**Used for**: Global state initialization from file system metadata

```typescript
// Prepare globals that are needed for running
globalThis._VSCODE_PRODUCT_JSON = { ...product };
globalThis._VSCODE_PACKAGE_JSON = { ...pkg };
globalThis._VSCODE_FILE_ROOT = import.meta.dirname;
```

**Key aspects**:
- Stores product/package metadata on globalThis for early access
- Uses `import.meta.dirname` (ESM equivalent of `__dirname`)
- Spreads to prevent external mutations
- Available to all subsequent module loads without re-reading files
- Related globals accessed in `/src/vs/amdX.ts:207,232` and `/src/vs/nls.ts:7,11`

### Pattern 3: NLS Configuration from Environment
**Found in**: `src/bootstrap-esm.ts:49-104`
**Used for**: Lazy-loaded localization with fallback chain

```typescript
async function doSetupNLS(): Promise<INLSConfiguration | undefined> {
	performance.mark('code/willLoadNls');

	let nlsConfig: INLSConfiguration | undefined = undefined;

	let messagesFile: string | undefined;
	if (process.env['VSCODE_NLS_CONFIG']) {
		try {
			nlsConfig = JSON.parse(process.env['VSCODE_NLS_CONFIG']);
			if (nlsConfig?.languagePack?.messagesFile) {
				messagesFile = nlsConfig.languagePack.messagesFile;
			} else if (nlsConfig?.defaultMessagesFile) {
				messagesFile = nlsConfig.defaultMessagesFile;
			}

			globalThis._VSCODE_NLS_LANGUAGE = nlsConfig?.resolvedLanguage;
		} catch (e) {
			console.error(`Error reading VSCODE_NLS_CONFIG from environment: ${e}`);
		}
	}

	if (
		process.env['VSCODE_DEV'] ||	// no NLS support in dev mode
		!messagesFile					// no NLS messages file
	) {
		return undefined;
	}

	try {
		globalThis._VSCODE_NLS_MESSAGES = JSON.parse((await fs.promises.readFile(messagesFile)).toString());
	} catch (error) {
		console.error(`Error reading NLS messages file ${messagesFile}: ${error}`);

		// Mark as corrupt: this will re-create the language pack cache next startup
		if (nlsConfig?.languagePack?.corruptMarkerFile) {
			try {
				await fs.promises.writeFile(nlsConfig.languagePack.corruptMarkerFile, 'corrupted');
			} catch (error) {
				console.error(`Error writing corrupted NLS marker file: ${error}`);
			}
		}

		// Fallback to the default message file to ensure english translation at least
		if (nlsConfig?.defaultMessagesFile && nlsConfig.defaultMessagesFile !== messagesFile) {
			try {
				globalThis._VSCODE_NLS_MESSAGES = JSON.parse((await fs.promises.readFile(nlsConfig.defaultMessagesFile)).toString());
			} catch (error) {
				console.error(`Error reading default NLS messages file ${nlsConfig.defaultMessagesFile}: ${error}`);
			}
		}
	}

	performance.mark('code/didLoadNls');

	return nlsConfig;
}
```

**Key aspects**:
- Reads localization config from `VSCODE_NLS_CONFIG` environment variable (JSON-encoded)
- Supports language packs with per-language message files
- Includes three-tier fallback: language pack → default → skip in dev mode
- Marks corrupted language pack metadata for cache invalidation
- Caches result in setupNLSResult Promise to prevent duplicate loads
- Performance markers track load timing

### Pattern 4: Dynamic Module Loader Hook Registration
**Found in**: `src/bootstrap-node.ts:62-74`
**Used for**: Development-time node_modules path injection

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

**Key aspects**:
- Registers external loader hook file that handles module resolution redirects
- Passes `injectPath` via `data` parameter to loader hook
- Only active in development mode (VSCODE_DEV environment variable)
- Uses new Node.js module loader API (ESM-compatible)
- Hook file (`bootstrap-import.js`) maintains mapping cache

### Pattern 5: AMD Module Loader Shim with Context Detection
**Found in**: `src/vs/amdX.ts:19-94`
**Used for**: Compatibility layer supporting AMD, ESM, and Node.js execution contexts

```typescript
class AMDModuleImporter {
	public static INSTANCE = new AMDModuleImporter();

	private readonly _isWebWorker = (typeof self === 'object' && self.constructor && self.constructor.name === 'DedicatedWorkerGlobalScope');
	private readonly _isRenderer = typeof document === 'object';

	private readonly _defineCalls: DefineCall[] = [];
	private _state = AMDModuleImporterState.Uninitialized;
	private _amdPolicy: Pick<TrustedTypePolicy, 'name' | 'createScriptURL'> | undefined;

	private _initialize(): void {
		if (this._state === AMDModuleImporterState.Uninitialized) {
			if (globalThis.define) {
				this._state = AMDModuleImporterState.InitializedExternal;
				return;
			}
		} else {
			return;
		}

		this._state = AMDModuleImporterState.InitializedInternal;

		globalThis.define = (id: any, dependencies: any, callback: any) => {
			if (typeof id !== 'string') {
				callback = dependencies;
				dependencies = id;
				id = null;
			}
			if (typeof dependencies !== 'object' || !Array.isArray(dependencies)) {
				callback = dependencies;
				dependencies = null;
			}
			this._defineCalls.push(new DefineCall(id, dependencies, callback));
		};

		globalThis.define.amd = true;

		if (this._isRenderer) {
			this._amdPolicy = globalThis._VSCODE_WEB_PACKAGE_TTP ?? window.trustedTypes?.createPolicy('amdLoader', {
				createScriptURL(value: any) {
					if (value.startsWith(window.location.origin)) {
						return value;
					}
					if (value.startsWith(`${Schemas.vscodeFileResource}://${VSCODE_AUTHORITY}`)) {
						return value;
					}
					throw new Error(`[trusted_script_src] Invalid script url: ${value}`);
				}
			});
		} else if (this._isWebWorker) {
			this._amdPolicy = globalThis._VSCODE_WEB_PACKAGE_TTP ?? globalThis.trustedTypes?.createPolicy('amdLoader', {
				createScriptURL(value: string) {
					return value;
				}
			});
		}
	}
}
```

**Key aspects**:
- Detects execution context (browser renderer, web worker, Node.js) at runtime
- Only creates internal AMD shim if external AMD loader not detected
- Implements Trusted Type policies for CSP compliance in browser contexts
- Queues define() calls and resolves with final exported value
- Handles variable argument patterns of AMD define() signature

### Pattern 6: Context-Aware Script Loading with Dynamic Import
**Found in**: `src/vs/amdX.ts:96-194`
**Used for**: Loading AMD-style modules across browser, worker, and Node.js

```typescript
public async load<T>(scriptSrc: string): Promise<T> {
	this._initialize();

	if (this._state === AMDModuleImporterState.InitializedExternal) {
		return new Promise<T>(resolve => {
			const tmpModuleId = generateUuid();
			globalThis.define(tmpModuleId, [scriptSrc], function (moduleResult: T) {
				resolve(moduleResult);
			});
		});
	}

	const defineCall = await (this._isWebWorker ? this._workerLoadScript(scriptSrc) : this._isRenderer ? this._rendererLoadScript(scriptSrc) : this._nodeJSLoadScript(scriptSrc));
	// ... rest of implementation
}

private async _nodeJSLoadScript(scriptSrc: string): Promise<DefineCall | undefined> {
	try {
		const fs = (await import(/* webpackIgnore: true */ /* @vite-ignore */ `${'fs'}`)).default;
		const vm = (await import(/* webpackIgnore: true */ /* @vite-ignore */ `${'vm'}`)).default;
		const module = (await import(/* webpackIgnore: true */ /* @vite-ignore */ `${'module'}`)).default;

		const filePath = URI.parse(scriptSrc).fsPath;
		const content = fs.readFileSync(filePath).toString();
		const scriptSource = module.wrap(content.replace(/^#!.*/, ''));
		const script = new vm.Script(scriptSource);
		const compileWrapper = script.runInThisContext();
		compileWrapper.apply();
		return this._defineCalls.pop();
	} catch (error) {
		throw error;
	}
}
```

**Key aspects**:
- Uses `/* webpackIgnore: true */ /* @vite-ignore */` comments to prevent bundler inlining
- Dynamic import strings constructed with template concatenation to obscure from static analysis
- Node.js path: reads file, wraps in module context via `vm.Script`, executes in current context
- Browser path: creates script element, appends to DOM, waits for load event
- Worker path: uses dynamic import directly
- Collects define() calls from executed script and resolves with exported value

### Pattern 7: Environment-Driven Bootstrap Chain
**Found in**: `src/bootstrap-fork.ts:206-229`, `src/bootstrap-cli.ts`, `src/bootstrap-server.ts`
**Used for**: Conditional execution flow based on process type and environment

```typescript
// From bootstrap-fork.ts
if (process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH']) {
	devInjectNodeModuleLookupPath(process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH']);
}

if (!!process.send && process.env['VSCODE_PIPE_LOGGING'] === 'true') {
	pipeLoggingToParent();
}

if (!process.env['VSCODE_HANDLES_UNCAUGHT_ERRORS']) {
	installUnhandledErrorHandler();
}

if (process.env['VSCODE_PARENT_PID']) {
	installParentPidWatcher(Number(process.env['VSCODE_PARENT_PID']));
}

await import([`./${process.env['VSCODE_ESM_ENTRYPOINT']}.js`].join('/') /* workaround: esbuild prints some strange warnings when trying to inline? */);

// From bootstrap-cli.ts
delete process.env['VSCODE_CWD'];

// From bootstrap-server.ts
delete process.env['ELECTRON_RUN_AS_NODE'];
```

**Key aspects**:
- Environment variables control which bootstrap hooks activate
- Dynamic import path constructed from `VSCODE_ESM_ENTRYPOINT` environment variable
- String concatenation used to hide import path from bundler static analysis
- Early cleanup/deletion of process environment to prevent child process inheritance
- Fork-specific vs. server-specific vs. CLI-specific cleanup patterns

---

## Pattern Categories & Codebase Distribution

### Module Resolution Patterns
- **Hook registration via data URL** (`bootstrap-esm.ts:29`): Inline resolver hook for fs shimming
- **External loader registration** (`bootstrap-node.ts:73`): Development-time module path injection
- **Conditional resolution** (`bootstrap-import.ts:87-100`): Package.json exports field resolution with ESM/CJS detection

### Global State Injection Patterns
- **Product/Package metadata** (`bootstrap-esm.ts:33-34`): Pre-parsed JSON stored on globalThis
- **NLS/Localization config** (`bootstrap-esm.ts:55-104`): Lazy-loaded messages with file fallbacks
- **File root path** (`bootstrap-esm.ts:35`): ESM import.meta.dirname for resource access
- **AMD policy objects** (`amdX.ts:76,88`): Trusted Type policies stored on globalThis

### Dynamic Import Patterns
- **AMD shim fallback** (`amdX.ts:57-71`): Creates define() on globalThis if not present
- **Context detection** (`amdX.ts:36-37`): Runtime feature detection (worker, renderer, Node.js)
- **Bundler-transparent imports** (`amdX.ts:180-182`): Template string concatenation hiding paths from static analysis
- **Inline script execution** (`bootstrap-fork.ts:229`): Path hidden via array join() to prevent bundler inlining

### Environment Configuration Patterns
- **NLS from environment** (`bootstrap-esm.ts:55-60`): JSON.parse of VSCODE_NLS_CONFIG
- **Development mode detection** (`bootstrap-node.ts:63`, `bootstrap-esm.ts:71`): Conditional behavior via VSCODE_DEV variable
- **Portable mode detection** (`bootstrap-node.ts:154-189`): File system and environment variable checks
- **Process type detection** (`bootstrap-fork.ts:170,184`): VSCODE_PARENT_PID, VSCODE_CRASH_REPORTER_PROCESS_TYPE

### Fallback & Error Handling Patterns
- **Three-tier NLS fallback** (`bootstrap-esm.ts:78-99`): Language pack → default → skip with corruption marking
- **Graceful module registration** (`bootstrap-node.ts:62-74`): Early exit if not in dev mode
- **Trusted Type fallback** (`amdX.ts:76,88`): globalThis._VSCODE_WEB_PACKAGE_TTP as override

---

## Key Characteristics for Tauri/Rust Replacement

1. **Module Resolution Interception**: Node.js loader hooks (data URL registration, external hook files) have no direct Tauri equivalent - Rust build system will handle via static asset pipeline instead.

2. **GlobalThis Injection**: Global state initialization from filesystem metadata - Tauri would inject via IPC or at window initialization instead of runtime setup.

3. **Environment-Driven Branching**: Heavy reliance on process.env variables for conditional behavior - Tauri would use configuration structs or IPC messages instead.

4. **AMD Compatibility Shim**: Define() function creation and script execution via vm.Script - Tauri/Rust would use bundled ESM modules directly without AMD wrapper.

5. **NLS File Fallback Chain**: Complex file I/O with fallback paths - Tauri would bundle localization at compile time or use Tauri-native localization APIs.

6. **Bundler-Transparent Dynamic Imports**: String concatenation to hide paths from webpack/vite - Rust equivalent would use conditional compilation or compile-time paths.

7. **Cross-Context Execution**: Runtime detection of browser/worker/Node.js - Tauri runs in single context (webview) so this complexity is eliminated.

---

**File References**:
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts` (112 LOC)
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-node.ts` (191 LOC)
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-fork.ts` (partial)
- `/home/norinlavaee/projects/vscode-atomic/src/vs/amdX.ts` (232 LOC)
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-import.ts` (102 LOC)
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-meta.ts` (31 LOC)

## External References
<!-- Source: codebase-online-researcher sub-agent -->
#### Node.js `node:module` Customization Hooks API (v20.6+ / v18.19+; `register` deprecated in v25.9 / v24.15)
**Docs:** https://nodejs.org/api/module.html#customization-hooks
**Relevant behaviour:**
`module.register(specifier, parentURL)` (imported as `register` from `node:module`) registers a module that exports asynchronous ESM customization hook functions — `resolve`, `load`, and optionally `initialize` — running on a dedicated loader worker thread. The `resolve` hook receives `(specifier, context, nextResolve)` and must return `{ url, format?, shortCircuit? }`. Setting `shortCircuit: true` terminates the hook chain without calling `nextResolve`. The hook module can be passed as a `data:` URL, allowing inline code to be base64-encoded and registered at startup without a separate file on disk. This is how `bootstrap-esm.ts` injects the `fs` → `original-fs` rewrite without creating a loader file. **Deprecation notice (Node >= v25.9 / v24.15):** `module.register()` carries Stability 0 — Deprecated (DEP0205, runtime deprecation in v26). The replacement is `module.registerHooks()` (Stability 1.2 RC), which accepts synchronous hooks running in-thread, eliminating the worker-thread overhead and several CommonJS caveats. A Tauri/Rust port must either replicate this hook mechanism or, since there is no Electron/ASAR layer, eliminate the need for it entirely.
**Where used:** `src/bootstrap-esm.ts:7` (`import { register } from 'node:module'`), `src/bootstrap-esm.ts:14–30` (conditional registration of `data:`-URL-encoded resolve hook).

---

#### Electron `original-fs` built-in module (Electron 1+)
**Docs:** https://www.electronjs.org/docs/latest/tutorial/asar-archives#treating-an-asar-archive-as-a-normal-file
**Relevant behaviour:**
Electron patches the Node.js `fs` module so that all file-system calls transparently read from ASAR virtual archives. `original-fs` is a built-in provided by Electron that exposes the real, un-patched `fs` API, bypassing ASAR interception entirely. It is available as `require('original-fs')` (CommonJS) or, in ESM, as the `node:original-fs` builtin URL. This is why the resolve hook in `bootstrap-esm.ts` maps the bare specifier `'fs'` to `'node:original-fs'`: any code that imports `'fs'` in an Electron/ESM context will receive the unpatched file-system module, ensuring that VS Code's own I/O (reading NLS message files, corrupt-marker writes at lines 78–95) operates on the real filesystem rather than the ASAR virtual one. The `ELECTRON_RUN_AS_NODE` environment variable (and `process.versions['electron']` check at line 14) is the guard: the hook is only installed when running inside Electron. In `ELECTRON_RUN_AS_NODE` mode Electron behaves like a normal Node.js process but still provides `original-fs`, so both execution paths are covered.
**Where used:** `src/bootstrap-esm.ts:13` (comment explaining intent), `src/bootstrap-esm.ts:14` (guard: `process.env['ELECTRON_RUN_AS_NODE'] || process.versions['electron']`), `src/bootstrap-esm.ts:17–21` (resolve hook body remapping `'fs'` → `'node:original-fs'`), `src/bootstrap-esm.ts:29` (`register(...)` call encoding and activating the hook).

---

Both APIs are directly central to `src/bootstrap-esm.ts`. The `node:module` `register` / customization-hooks API is the mechanism by which the file installs an ESM-layer resolve hook at runtime without a separate loader file — the hook body is base64-encoded as a `data:` URL and registered via `register(dataUrl, import.meta.url)` (lines 7 and 29). The Electron `original-fs` builtin is the *target* of that hook: because Electron intercepts all `require('fs')` / `import 'fs'` calls to add ASAR transparency, code that must access raw on-disk files (NLS message bundles, corrupt-marker files) needs the unpatched module. A Tauri/Rust port eliminates the Electron layer entirely, meaning (a) ASAR packaging and therefore `original-fs` become irrelevant — the entire conditional block at lines 14–30 can be removed, and (b) the `node:module` hook infrastructure is only needed if the Rust/Tauri host still embeds a Node.js runtime; if the NLS loading is ported to Rust, the whole `bootstrap-esm.ts` may be superseded. Additionally, as of Node v25/v26 `module.register()` is deprecated in favour of the synchronous `module.registerHooks()` API, so any retained TypeScript bootstrap code should migrate to `registerHooks` to avoid the runtime deprecation warning.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
