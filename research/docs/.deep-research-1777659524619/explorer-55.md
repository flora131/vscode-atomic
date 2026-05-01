# Partition 55 of 79 — Findings

## Scope
`src/bootstrap-esm.ts/` (1 files, 112 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Module Loading Architecture: bootstrap-esm.ts and ESM/AMD Transition

## Overview
`src/bootstrap-esm.ts` (112 LOC) is the core ESM initialization module for VS Code's Node.js runtime. It manages the transition from AMD (Asynchronous Module Definition) to ESM (ECMAScript Modules) by setting up globals, registering module resolution hooks, and initializing the National Language Support (NLS) infrastructure.

---

## Implementation Files

### Core Bootstrap System
- **`src/bootstrap-esm.ts`** (112 LOC) - Main ESM bootstrap entry point
  - Registers Node.js module resolution hook via `register()` for 'fs' → 'original-fs' mapping
  - Initializes global objects: `_VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON`, `_VSCODE_FILE_ROOT`
  - Implements `setupNLS()` and `doSetupNLS()` for NLS initialization
  - Exports `bootstrapESM()` async function as contract for host initialization

- **`src/bootstrap-node.ts`** (191 LOC) - Node.js infrastructure setup
  - `devInjectNodeModuleLookupPath()` - Registers ESM loader hook via `Module.register()`
  - `removeGlobalNodeJsModuleLookupPaths()` - Disables global module lookup paths
  - `configurePortable()` - Portable application mode configuration
  - Error stack trace limit configuration (100 frames)
  - SIGPIPE handling for broken pipe states
  - Working directory setup across platforms

- **`src/bootstrap-meta.ts`** (56 LOC) - Product and package metadata
  - Loads `product.json` and `package.json` via `createRequire()`
  - Handles embedded app configurations
  - Supports environment-based overrides via `product.overrides.json`

- **`src/bootstrap-import.ts`** (102 LOC) - Module specifier resolver
  - `initialize()` - Populates specifier-to-URL and specifier-to-format mappings
  - `resolve()` - ESM loader hook for module resolution
  - Resolves package.json `exports["."]` with conditional imports
  - Distinguishes between ESM (.mjs), CommonJS (.cjs), and default module types

- **`src/bootstrap-fork.ts`** (229 LOC) - Forked process bootstrap
  - Initializes global ESM and NLS infrastructure
  - Implements process logging pipeline to parent via `process.send()`
  - Handles uncaught exceptions and unhandled promise rejections
  - Parent process termination monitoring
  - Dynamically imports ESM entrypoint via `process.env['VSCODE_ESM_ENTRYPOINT']`
  - Dynamic import pattern: `await import(['./${env}'.js'].join('/'))`

### Entry Points
- **`src/main.ts`** (742 LOC) - Electron main process entry
  - Calls `bootstrapESM()` at line 208 during Electron app startup
  - Resolves NLS configuration before ESM bootstrap
  - Sets `VSCODE_NLS_CONFIG` environment variable
  - Sets `VSCODE_CODE_CACHE_PATH` for V8 code caching
  - Passes NLS config to `./vs/code/electron-main/main.js` after bootstrap

- **`src/server-main.ts`** - Server runtime entry point (references bootstrap functions)
- **`src/server-cli.ts`** - CLI server entry point
- **`src/cli.ts`** - CLI entry point

---

## Type Definitions

### Global Declarations
- **`src/typings/vscode-globals-nls.d.ts`** (41 LOC)
  - `globalThis._VSCODE_NLS_MESSAGES: string[]` - Flattened NLS message array
  - `globalThis._VSCODE_NLS_LANGUAGE: string | undefined` - Active language code

- **`src/typings/vscode-globals-product.d.ts`** (48 LOC)
  - `globalThis._VSCODE_FILE_ROOT: string` - Base file path for resources
  - `globalThis._VSCODE_PRODUCT_JSON: Record<string, any>` - Product config (deprecated)
  - `globalThis._VSCODE_PACKAGE_JSON: Record<string, any>` - Package config (deprecated)
  - `globalThis._VSCODE_CSS_LOAD` - CSS loader function (dev-time only)
  - `globalThis._VSCODE_DISABLE_CSS_IMPORT_MAP` - CSS import map control (deprecated)
  - `globalThis._VSCODE_USE_RELATIVE_IMPORTS` - Source module reference mode (deprecated)

- **`src/typings/base-common.d.ts`** (41 LOC) - Common browser/Node polyfills

---

## Configuration and Infrastructure

### NLS System
- **`src/vs/nls.ts`** (245 LOC) - Localization API
  - `localize()` / `localize2()` - String localization functions
  - `getNLSMessages()` / `getNLSLanguage()` - Global accessor functions
  - `lookupMessage()` - Index-based message lookup during build
  - `INLSConfiguration` interface - Configuration contract
  - `ILanguagePack` interface - Language pack structure with translations
  - Pseudo-language support (duplicates vowels for testing)

- **`src/vs/base/node/nls.ts`** (150+ LOC) - Node.js NLS resolution
  - `resolveNLSConfiguration()` - Async NLS setup from context
  - Language pack detection and caching in `<userDataPath>/clp/`
  - Translation file format parsing (nls.keys.json, nls.messages.json)
  - Fallback to default English messages if pack unavailable
  - Corrupt language pack detection and cleanup

### Performance Monitoring
- **`src/vs/base/common/performance.ts`** (100+ LOC)
  - `mark()` / `clearMarks()` functions for performance measurement
  - Polyfill for browser contexts using Date.now()
  - Native performance API usage for environments with `performance.mark()`
  - Time origin tracking for accurate relative measurements

---

## AMD-to-ESM Bridge (Legacy Support)

### AMD Module Loader
- **`src/vs/amdX.ts`** (241 LOC) - AMD to ESM compatibility layer
  - `AMDModuleImporter` singleton class
  - `load()` - Dynamically loads AMD modules and instantiates them
  - `_rendererLoadScript()` - Browser `<script>` tag injection
  - `_workerLoadScript()` - Web Worker dynamic import: `await import(scriptSrc)`
  - `_nodeJSLoadScript()` - Node.js vm module evaluation using `module.wrap()`
  - `importAMDNodeModule()` - Factory for importing node_modules AMD bundles
  - Caching mechanism to prevent duplicate loads
  - TrustedTypes policy enforcement for renderer context

---

## Module Resolution Contract

### Node.js Loader Hook Pattern
The module loading contract VS Code uses relies on Node.js loader hooks (ESM API):

1. **Hook Registration** (in bootstrap-node.ts):
   ```typescript
   Module.register('./bootstrap-import.js', { parentURL: import.meta.url, data: injectPath })
   ```

2. **Hook Implementation** (in bootstrap-import.ts):
   ```typescript
   export async function resolve(specifier, context, nextResolve) {
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

3. **In-Memory Loader Registration** (in bootstrap-esm.ts):
   ```typescript
   const jsCode = `
     export async function resolve(specifier, context, nextResolve) {
       if (specifier === 'fs') {
         return { format: 'builtin', shortCircuit: true, url: 'node:original-fs' };
       }
       return nextResolve(specifier, context);
     }`;
   register(`data:text/javascript;base64,${Buffer.from(jsCode).toString('base64')}`, import.meta.url);
   ```

### Key Features:
- Resolves CommonJS vs ESM formats based on package.json `type` field and file extensions
- Maps specifiers to file URLs for node_modules
- Supports conditional exports (`exports["."].import` vs `exports["."].default`)
- Chains multiple loader hooks via `nextResolve()` callback
- Uses base64 data URLs for in-memory loader registration

---

## Global Contract

The Rust host must reproduce or replace:

1. **Global Setup**:
   - Set `globalThis._VSCODE_PRODUCT_JSON` with product config
   - Set `globalThis._VSCODE_PACKAGE_JSON` with package metadata
   - Set `globalThis._VSCODE_FILE_ROOT` with resource root path

2. **Module Resolution**:
   - Intercept all `await import()` calls
   - Map specifiers to file paths using package.json exports
   - Distinguish CommonJS from ESM based on package metadata
   - Chain resolution through multiple handler layers

3. **NLS Infrastructure**:
   - Parse `VSCODE_NLS_CONFIG` environment variable
   - Load language pack from configured `messagesFile` path
   - Set `globalThis._VSCODE_NLS_MESSAGES` as flat string array
   - Set `globalThis._VSCODE_NLS_LANGUAGE` with resolved language code
   - Handle missing or corrupt language packs with English fallback

4. **Dynamic Module Loading**:
   - Support `await import(dynamicPath)` for plugin/extension loading
   - Execute NLS setup async before importing main bundle
   - Mark performance checkpoints: `code/willLoadNls`, `code/didLoadNls`

---

## Related Directories

- **`src/vs/code/`** - Platform-specific main entry points (electron-main/main.js)
- **`src/vs/base/node/`** - Node.js utilities (nls.ts, unc.js, etc.)
- **`src/vs/platform/environment/`** - Environment variable and user data path management
- **`src/typings/`** - Global type definitions for bootstrap infrastructure
- **`src/vs/nls.ts`** - Core NLS implementation

---

## Notable Implementation Details

**Performance Marking**: bootstrap-esm.ts uses `performance.mark()` to track NLS loading:
- `code/willLoadNls` - Before NLS file I/O
- `code/didLoadNls` - After NLS file loaded and parsed

**Environment Variables**:
- `ELECTRON_RUN_AS_NODE` - Indicates Electron in Node mode
- `VSCODE_NLS_CONFIG` - JSON-serialized NLS configuration (set before ESM bootstrap)
- `VSCODE_CODE_CACHE_PATH` - V8 code cache directory
- `VSCODE_DEV` - Development mode flag (disables NLS support)
- `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH` - Dynamic module path injection for dev

**File System**: Uses `node:fs` → `node:original-fs` mapping to work around Electron's fs override, then transparently proxies to original-fs via module resolution hook.

---

## Summary

The bootstrap-esm.ts module-loading contract is a three-phase initialization system:
1. **Module Resolution**: Register ESM loader hooks that map package specifiers to file URLs and resolve format types
2. **Global Setup**: Initialize product/package/resource globals that dependent code accesses
3. **NLS Bootstrap**: Asynchronously load and parse language pack files, populating the NLS message array and language globals

A Tauri/Rust host must implement equivalent interception of dynamic imports, format detection based on package metadata, and async global initialization before the main application bundle loads.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts` (112 LOC) — primary subject
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-meta.ts` — provides `product` and `pkg` exports
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-node.ts` — side-effect import; Node.js environment setup
- `/home/norinlavaee/projects/vscode-atomic/src/vs/nls.ts` — declares `INLSConfiguration` and consumes `globalThis._VSCODE_NLS_MESSAGES`
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/common/performance.ts` — provides `mark()` used in NLS timing

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts`

- **Role:** The ESM-era entry-point bootstrap for VS Code's main process (and Node.js renderer contexts). It runs three initialisation actions unconditionally at module evaluation time: (1) conditionally registers a Node.js ESM loader hook that redirects the bare specifier `'fs'` to `'node:original-fs'` (Electron's unpatched fs), (2) writes three globals onto `globalThis`, and (3) provides `bootstrapESM()` as the single exported async function that callers `await` before any further application code runs. The NLS subsystem is fully initialised inside that `await`.

- **Key symbols:**

  | Symbol | Kind | Location |
  |---|---|---|
  | `bootstrapESM` | exported `async function` | line 108 |
  | `setupNLS` | unexported module-private function | line 41 |
  | `doSetupNLS` | unexported `async function` | line 49 |
  | `setupNLSResult` | module-private `Promise` cache | line 39 |
  | inline `jsCode` string | ESM loader hook source | lines 15–28 |
  | `register(...)` call | Node.js `node:module` registration | line 29 |

- **Control flow:**

  1. **Module evaluation phase (synchronous, lines 14–35):**
     - Lines 14–30: If `process.env['ELECTRON_RUN_AS_NODE']` is set or `process.versions['electron']` is truthy, a string of ESM loader hook JavaScript is assembled inline (`jsCode`, lines 15–28). The hook exports a single `resolve()` function that intercepts the specifier `'fs'` and returns `{ format: 'builtin', shortCircuit: true, url: 'node:original-fs' }`. All other specifiers are forwarded to `nextResolve`. The hook source string is base64-encoded and passed to `register()` as a `data:text/javascript;base64,...` URL with `import.meta.url` as the parent URL (line 29). This registers the hook into the current Node.js module graph before any subsequent `import 'fs'` statements can resolve.
     - Lines 33–35: Three `globalThis` assignments are made unconditionally:
       - `globalThis._VSCODE_PRODUCT_JSON` ← shallow copy of `product` (from `bootstrap-meta.ts`)
       - `globalThis._VSCODE_PACKAGE_JSON` ← shallow copy of `pkg` (from `bootstrap-meta.ts`)
       - `globalThis._VSCODE_FILE_ROOT` ← `import.meta.dirname` (the directory of this file at runtime)

  2. **`bootstrapESM()` — async export (lines 108–112):**
     - Calls `setupNLS()` and `await`s the result. That is the only work performed.

  3. **`setupNLS()` — memoisation wrapper (lines 41–47):**
     - Checks `setupNLSResult`; if `undefined`, calls `doSetupNLS()` and stores the returned `Promise` in `setupNLSResult`. Returns the stored promise. Subsequent calls return the same promise regardless of resolution state.

  4. **`doSetupNLS()` — async NLS initialisation (lines 49–104):**
     - Emits performance mark `'code/willLoadNls'` (line 50).
     - Reads `process.env['VSCODE_NLS_CONFIG']` and JSON-parses it into `nlsConfig: INLSConfiguration | undefined` (lines 55–68). Sets `messagesFile` from `nlsConfig.languagePack.messagesFile` (line 59) or falls back to `nlsConfig.defaultMessagesFile` (line 61). Sets `globalThis._VSCODE_NLS_LANGUAGE` to `nlsConfig.resolvedLanguage` (line 64).
     - Early-return `undefined` (line 75) if `process.env['VSCODE_DEV']` is set or `messagesFile` is still `undefined`.
     - Reads `messagesFile` from disk via `fs.promises.readFile` (line 78) and JSON-parses it into `globalThis._VSCODE_NLS_MESSAGES`.
     - On read failure (catch block, lines 79–98): if `nlsConfig.languagePack.corruptMarkerFile` is defined, writes the string `'corrupted'` to that path (line 85) to signal a cache-rebuild on next startup. Then attempts to fall back to `nlsConfig.defaultMessagesFile` and populate `globalThis._VSCODE_NLS_MESSAGES` from there (lines 92–97).
     - Emits performance mark `'code/didLoadNls'` (line 101).
     - Returns `nlsConfig`.

- **Data flow:**

  - **Input channels:**
    - `process.env['ELECTRON_RUN_AS_NODE']` / `process.versions['electron']` — controls loader hook registration
    - `product` / `pkg` from `bootstrap-meta.ts` — feeds product/package globals
    - `import.meta.dirname` — feeds `_VSCODE_FILE_ROOT`
    - `process.env['VSCODE_NLS_CONFIG']` — JSON string carrying `INLSConfiguration`
    - `process.env['VSCODE_DEV']` — disables NLS loading in dev mode
    - Filesystem: the `messagesFile` and optionally `defaultMessagesFile` paths from within `VSCODE_NLS_CONFIG`

  - **Output channels (all `globalThis` assignments):**
    - `globalThis._VSCODE_PRODUCT_JSON` (line 33)
    - `globalThis._VSCODE_PACKAGE_JSON` (line 34)
    - `globalThis._VSCODE_FILE_ROOT` (line 35)
    - `globalThis._VSCODE_NLS_LANGUAGE` (line 64)
    - `globalThis._VSCODE_NLS_MESSAGES` (line 78 and line 94) — read by `vs/nls.ts:getNLSMessages()` (line 7 of `vs/nls.ts`)

  - **Side effects:**
    - Registers an ESM loader hook into the live Node.js module graph (line 29)
    - Writes `'corrupted'` to `nlsConfig.languagePack.corruptMarkerFile` on NLS read failure (line 85)
    - Two `performance.mark()` calls record timing into the `vs/base/common/performance.ts` polyfill buffer

- **Dependencies:**
  - `node:fs` (import line 6) — `fs.promises.readFile` and `fs.promises.writeFile` in `doSetupNLS`
  - `node:module` `register` (import line 7) — ESM hook registration
  - `./bootstrap-meta.js` — `product`, `pkg` (import line 8)
  - `./bootstrap-node.js` — side-effect import; runs `setupCurrentWorkingDirectory()` and installs SIGPIPE handler (import line 9)
  - `./vs/base/common/performance.js` — `performance.mark()` (import line 10)
  - `./vs/nls.js` — `INLSConfiguration` type only (import line 11); the runtime coupling is inverted: `vs/nls.ts` reads the globals set here

---

### Cross-Cutting Synthesis

`bootstrap-esm.ts` is the **synchronous+async two-phase preamble** that every VS Code main-process entry point must `await` before doing any real work. In a Tauri/Rust port, the three responsibilities map onto different host layers. First, the **`fs` → `original-fs` loader hook** exists solely because Electron monkey-patches Node's built-in `fs` module; in a Tauri context where the renderer runs WebView and the backend is native Rust, this interception has no equivalent and is simply dropped. Second, the **`globalThis` product/package/file-root assignments** (lines 33–35) are consumed by many downstream TypeScript modules that call `globalThis._VSCODE_PRODUCT_JSON` at runtime; a Rust host using a bundled WebView would need to inject an equivalent JSON object into the WebView's JavaScript context before any application JS executes — concretely as a `window.__VSCODE_PRODUCT_JSON__` injection from the Tauri `Window::eval` or `initialization_script` API. Third, the **NLS message injection** (lines 55–104) reads a flat JSON array of translated strings from disk and places it on `globalThis._VSCODE_NLS_MESSAGES`; a Rust host would replicate this by determining the active locale at startup (via `app.getPreferredSystemLanguages()` equivalent in Rust/Tauri), loading the corresponding messages bundle from the app's resource directory, and injecting the stringified array into the WebView's JS context, again before any `localize()` calls execute. The memoised promise pattern in `setupNLS` / `setupNLSResult` (lines 39–47) is a Node.js concurrency guard with no direct Rust equivalent; in Rust this becomes a `once_cell::Lazy` or `tokio::sync::OnceCell` initialised at Tauri startup. The `performance.mark` calls (lines 50, 101) feed VS Code's internal startup timing telemetry and would be replaced by Tauri's tracing spans or simply omitted.

---

### Out-of-Partition References

| Reference | Symbol / path | Used at |
|---|---|---|
| `./bootstrap-meta.js` | `product`, `pkg` | `bootstrap-esm.ts:8, 33–34` |
| `./bootstrap-node.js` | side-effect only (`setupCurrentWorkingDirectory`, SIGPIPE handler, `Error.stackTraceLimit`) | `bootstrap-esm.ts:9` |
| `./vs/base/common/performance.js` | `performance.mark()` | `bootstrap-esm.ts:10, 50, 101` |
| `./vs/nls.js` | `INLSConfiguration` (type); `getNLSMessages()` reads `globalThis._VSCODE_NLS_MESSAGES` set here | `bootstrap-esm.ts:11`; `vs/nls.ts:7–8` |
| `node:module` `register()` | ESM loader hook registration API | `bootstrap-esm.ts:7, 29` |
| `node:fs` `promises.readFile` / `promises.writeFile` | NLS messages file I/O | `bootstrap-esm.ts:6, 78, 85, 94` |
| `globalThis._VSCODE_PRODUCT_JSON` | set here; read by downstream product-config consumers | `bootstrap-esm.ts:33` |
| `globalThis._VSCODE_PACKAGE_JSON` | set here; read by downstream package-config consumers | `bootstrap-esm.ts:34` |
| `globalThis._VSCODE_FILE_ROOT` | set here; used as app-root anchor | `bootstrap-esm.ts:35` |
| `globalThis._VSCODE_NLS_MESSAGES` | set here; read by `vs/nls.ts:getNLSMessages()` on every `localize()` call | `bootstrap-esm.ts:78, 94`; `vs/nls.ts:7` |
| `globalThis._VSCODE_NLS_LANGUAGE` | set here; read by `vs/nls.ts:getNLSLanguage()` for pseudo-locale detection | `bootstrap-esm.ts:64`; `vs/nls.ts:11` |
| `process.env['VSCODE_NLS_CONFIG']` | runtime environment variable; JSON-encoded `INLSConfiguration` | `bootstrap-esm.ts:55` |
| `process.env['VSCODE_DEV']` | dev-mode guard disabling NLS | `bootstrap-esm.ts:71` |
| `process.env['ELECTRON_RUN_AS_NODE']` / `process.versions['electron']` | Electron-context guards for loader hook | `bootstrap-esm.ts:14` |

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# ESM Dynamic Loader & Module Loading Patterns - VS Code

Analysis of module-loading contract patterns for porting VS Code core IDE functionality from TypeScript/Electron to Tauri/Rust. Based on examination of `src/bootstrap-esm.ts` (112 LOC) and related module resolution infrastructure.

## Pattern Patterns Found

#### Pattern: Module Resolution Hook Registration via `register()`

**Where:** src/bootstrap-esm.ts:14-30

**What:** Installs Node.js ESM loader hooks to intercept module resolution, particularly for remapping built-in module specifiers (e.g., 'fs' -> 'original-fs') in Electron contexts using base64-encoded JavaScript source.

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
- src/bootstrap-import.ts:87-101 implements similar resolution hook that maps package specifiers to file URLs for development module injection
- src/bootstrap-node.ts:72-74 registers loader via `Module.register('./bootstrap-import.js', { parentURL: import.meta.url, data: injectPath })`

---

#### Pattern: Global Runtime Configuration via `globalThis`

**Where:** src/bootstrap-esm.ts:32-35

**What:** Establishes global namespace objects for application configuration, metadata, and internationalization that are initialized at bootstrap and available throughout the application lifecycle.

```typescript
globalThis._VSCODE_PRODUCT_JSON = { ...product };
globalThis._VSCODE_PACKAGE_JSON = { ...pkg };
globalThis._VSCODE_FILE_ROOT = import.meta.dirname;
```

**Variations:**
- src/vs/amdX.ts:207-208 reads from `globalThis._VSCODE_PRODUCT_JSON` to determine build vs. development mode
- src/vs/sessions/electron-browser/sessions.ts:219-220 sets `globalThis._VSCODE_NLS_MESSAGES` and `globalThis._VSCODE_NLS_LANGUAGE`
- src/vs/code/electron-browser/workbench/workbench.ts:400-401 initializes NLS globals from configuration
- src/vs/platform/product/common/product.ts:28-46 reads and validates these globals at module load time

---

#### Pattern: Lazy-Evaluated NLS (National Language Support) Setup

**Where:** src/bootstrap-esm.ts:39-104

**What:** Implements memoized async initialization of internationalization configuration with fallback chains: environment variables → language pack file → default messages file. Handles corruption detection and recovery.

```typescript
let setupNLSResult: Promise<INLSConfiguration | undefined> | undefined = undefined;

function setupNLS(): Promise<INLSConfiguration | undefined> {
	if (!setupNLSResult) {
		setupNLSResult = doSetupNLS();
	}
	return setupNLSResult;
}

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
	
	if (process.env['VSCODE_DEV'] || !messagesFile) {
		return undefined;
	}

	try {
		globalThis._VSCODE_NLS_MESSAGES = JSON.parse((await fs.promises.readFile(messagesFile)).toString());
	} catch (error) {
		console.error(`Error reading NLS messages file ${messagesFile}: ${error}`);
		if (nlsConfig?.languagePack?.corruptMarkerFile) {
			try {
				await fs.promises.writeFile(nlsConfig.languagePack.corruptMarkerFile, 'corrupted');
			} catch (error) {
				console.error(`Error writing corrupted NLS marker file: ${error}`);
			}
		}
		// Fallback to the default message file
		if (nlsConfig?.defaultMessagesFile && nlsConfig.defaultMessagesFile !== messagesFile) {
			try {
				globalThis._VSCODE_NLS_MESSAGES = JSON.parse((await fs.promises.readFile(nlsConfig.defaultMessagesFile)).toString());
			} catch (error) {
				console.error(`Error reading default NLS messages file: ${error}`);
			}
		}
	}
	performance.mark('code/didLoadNls');
	return nlsConfig;
}
```

**Variations:**
- src/cli.ts:13 calls `resolveNLSConfiguration()` for CLI context
- src/main.ts:129, 706, 722 sets up NLS paths with `import.meta.dirname`
- src/server-main.ts:45 similar NLS initialization in server context

---

#### Pattern: Dynamic AMD Module Importing with Context Awareness

**Where:** src/vs/amdX.ts:205-229

**What:** Wraps AMD module loading with caching, environment detection (dev vs. built), ASAR path handling, and browser URI conversion. Supports both ESM and AMD module formats during transition period.

```typescript
export async function importAMDNodeModule<T>(nodeModuleName: string, pathInsideNodeModule: string, isBuilt?: boolean): Promise<T> {
	if (isBuilt === undefined) {
		const product = globalThis._VSCODE_PRODUCT_JSON as unknown as IProductConfiguration;
		isBuilt = Boolean((product ?? globalThis.vscode?.context?.configuration()?.product)?.commit);
	}

	const nodeModulePath = pathInsideNodeModule ? `${nodeModuleName}/${pathInsideNodeModule}` : nodeModuleName;
	if (cache.has(nodeModulePath)) {
		return cache.get(nodeModulePath)!;
	}
	let scriptSrc: string;
	if (/^\w[\w\d+.-]*:\/\//.test(nodeModulePath)) {
		// looks like a URL
		scriptSrc = nodeModulePath;
	} else {
		const useASAR = (canASAR && isBuilt && !platform.isWeb);
		const actualNodeModulesPath = (useASAR ? nodeModulesAsarPath : nodeModulesPath);
		const resourcePath: AppResourcePath = `${actualNodeModulesPath}/${nodeModulePath}`;
		scriptSrc = FileAccess.asBrowserUri(resourcePath).toString(true);
	}
	const result = AMDModuleImporter.INSTANCE.load<T>(scriptSrc);
	cache.set(nodeModulePath, result);
	return result;
}
```

**Variations:**
- Used extensively in src/vs/workbench/contrib/terminal/browser/xterm/xtermAddonImporter.ts:44-50 for loading xterm addons
- src/vs/platform/telemetry/common/1dsAppender.ts:27-29 conditionally uses `importAMDNodeModule` for web contexts
- src/vs/workbench/contrib/markdown/browser/markedKatexSupport.ts:134 loads katex dynamically

---

#### Pattern: Development Module Path Injection via Loader Hooks

**Where:** src/bootstrap-import.ts:22-101

**What:** ESM loader hook that builds a runtime mapping of package specifiers to file URLs during development, reading package.json exports and main fields to determine entry points and module format (ESM vs CommonJS).

```typescript
const _specifierToUrl: Record<string, string> = {};
const _specifierToFormat: Record<string, string> = {};

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
						if (typeof v === 'string') return v;
						if (typeof v === 'object' && v !== null) {
							const d = (v as { default?: unknown }).default;
							if (typeof d === 'string') return d;
						}
						return undefined;
					};
					main = resolveCondition(dotExport.import) ?? resolveCondition(dotExport.default);
				}
			}
			if (typeof main !== 'string') {
				main = typeof pkgJson.main === 'string' ? pkgJson.main : undefined;
			}
			if (!main) main = 'index.js';
			if (!main.endsWith('.js') && !main.endsWith('.mjs') && !main.endsWith('.cjs')) {
				main += '.js';
			}
			const mainPath = join(injectPackageJSONPath, `../node_modules/${name}/${main}`);
			_specifierToUrl[name] = pathToFileURL(mainPath).href;
			const isModule = main.endsWith('.mjs') ? true : main.endsWith('.cjs') ? false : pkgJson.type === 'module';
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
	return nextResolve(specifier, context);
}
```

**Variations:**
- src/bootstrap-node.ts:62-74 wraps this with `devInjectNodeModuleLookupPath()` that calls `Module.register()` with the loader

---

#### Pattern: Fork Process ESM Entry Point Resolution

**Where:** src/bootstrap-fork.ts:226-229

**What:** Delays module loading until after bootstrap setup and uses environment variable to determine the ESM entry point to load, with intentional string concatenation to avoid esbuild inlining issues.

```typescript
// Bootstrap ESM
await bootstrapESM();

// Load ESM entry point
await import([`./${process.env['VSCODE_ESM_ENTRYPOINT']}.js`].join('/') /* workaround: esbuild prints some strange warnings when trying to inline? */);
```

**Variations:**
- src/cli.ts:26 directly imports CLI module: `await import('./vs/code/node/cli.js')`
- src/main.ts:211 main process import: `await import('./vs/code/electron-main/main.js')`
- src/server-main.ts:254 server context import: `return import('./vs/server/node/server.main.js')`

---

#### Pattern: Cross-Context Dynamic Import with Build Tool Directives

**Where:** src/vs/amdX.ts:170-194

**What:** Loads scripts in different JavaScript contexts (web workers, renderers, Node.js) with webpackIgnore and vite-ignore directives to prevent bundler analysis. For Node.js contexts, uses VM module to execute code in controlled scope.

```typescript
private async _workerLoadScript(scriptSrc: string): Promise<DefineCall | undefined> {
	if (this._amdPolicy) {
		scriptSrc = this._amdPolicy.createScriptURL(scriptSrc) as unknown as string;
	}
	await import(/* webpackIgnore: true */ /* @vite-ignore */ scriptSrc);
	return this._defineCalls.pop();
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

**Variations:**
- src/vs/workbench/services/keybinding/browser/keyboardLayoutService.ts:459 uses same pattern for keyboard layout imports

---

## Key Integration Points

The module loading contract operates across these layers:

1. **Bootstrap Phase** (src/bootstrap-*.ts): Sets up global state, resolves loader hooks, initializes NLS configuration before any application code runs

2. **Global Configuration** (globalThis._VSCODE_*): Runtime configuration objects seeded at bootstrap that are read throughout the application

3. **Loader Hooks** (Node.js Module.register): Intercept and remap module specifiers during development for flexible path resolution

4. **Dynamic Imports** (await import()): Used with webpackIgnore/vite-ignore directives to bypass bundler analysis and enable runtime dynamic loading

5. **AMD Compatibility Layer** (src/vs/amdX.ts): Bridges between legacy AMD module format and modern ESM, handles caching and context-specific loading strategies

The architecture demonstrates how VS Code maintains flexibility for loading modules across Electron (main/renderer/worker), Node.js, and web contexts while maintaining development-time module resolution overrides.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
