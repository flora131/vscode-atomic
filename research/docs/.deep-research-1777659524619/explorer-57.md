# Partition 57 of 79 — Findings

## Scope
`src/bootstrap-import.ts/` (1 files, 101 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Bootstrap Import System - File Location Map

## Scope Summary
This partition covers the bootstrap-import module used for early import shimming across VS Code processes (101 LOC).

## Implementation

### Core Bootstrap Module
- `src/bootstrap-import.ts` - ES module loader hook implementing Node.js Module.register protocol for redirecting dependency resolution. Exports `initialize()` and `resolve()` functions that intercept module loading to map package specifiers to node_modules paths.

### Related Bootstrap Infrastructure  
- `src/bootstrap-node.ts` - Node.js initialization module that registers the bootstrap-import loader via `devInjectNodeModuleLookupPath()` at line 62-74. Also provides utility functions for working directory setup, portable mode configuration, and global module path removal.
- `src/bootstrap-fork.ts` - Fork entry point that imports and uses `devInjectNodeModuleLookupPath` and `bootstrapESM` for logging pipe setup in forked processes.
- `src/bootstrap-esm.ts` - ES module setup that handles fs module mapping and NLS configuration for both Electron and Node.js environments.
- `src/main.ts` - Primary entry point that imports `configurePortable()` and `bootstrapESM()` from bootstrap modules during application initialization.

### Entry Point Modules That Reference Bootstrap System
- `src/cli.ts` - CLI entry point
- `src/server-cli.ts` - Server CLI entry point  
- `src/server-main.ts` - Server main entry point

## How It Works

The bootstrap-import system provides a Node.js loader hook mechanism:

1. **Registration**: `devInjectNodeModuleLookupPath()` in bootstrap-node.ts registers bootstrap-import.js as a loader hook using `Module.register()` (line 73)
2. **Initialization**: The `initialize()` function parses package.json files and builds mappings of specifier → file URL and specifier → module format
3. **Resolution**: The `resolve()` hook intercepts module resolution, returning redirected URLs for dependencies found in the mappings, deferring unknowns to the default resolver
4. **Scope**: Only activates when `VSCODE_DEV` environment variable is set (development mode)

## Configuration

No dedicated configuration files. Behavior controlled via:
- `VSCODE_DEV` environment variable - enables/disables the redirect system
- `ELECTRON_RUN_AS_NODE` environment variable - triggers fs → original-fs mapping in bootstrap-esm.ts
- `package.json` dependencies - scanned during initialization to build resolution mappings

## Notable Clusters

### Module Loading Infrastructure  
Contains 4 tightly-coupled bootstrap files in `src/`:
- bootstrap-import.ts (101 LOC) - Module resolution hooks
- bootstrap-node.ts (191 LOC) - Node.js setup and loader registration
- bootstrap-fork.ts - Forked process entry with logging
- bootstrap-esm.ts - ESM and Electron compatibility setup

These are invoked early from main entry points (main.ts, cli.ts, server-main.ts, server-cli.ts) during application startup sequence.

### Loader Hook Implementation Details
Uses Node.js Module.register() API (Node 20.6+) for dynamic loader registration. The system:
- Parses all package.json dependencies for entry points
- Supports both ESM (exports["."].import) and CommonJS (main) entry points  
- Handles conditional exports with fallback logic
- Infers module format from file extension and package.json type field
- Only applies during development (when running from source)

---

*Research context: Porting VS Code IDE functionality from TypeScript/Electron to Tauri/Rust requires understanding how the module loading system redirects dependencies, which would need equivalent resolution in Rust's module system (cargo dependencies, conditional compilation, feature flags).*

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Bootstrap Import Pattern Analysis

This document catalogs the bootstrap-import patterns used in VS Code for early module resolution shimming, critical for understanding cross-process module loading in a multi-process IDE architecture.

## Pattern Catalog

#### Pattern 1: Node.js Module Resolution Hook Registration
**Where:** `src/bootstrap-node.ts:62-74`
**What:** Registers a custom Node.js module loader hook to redirect module resolution from bundled sources to node_modules during development.

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
- Uses Node.js `Module.register()` API for ESM loader hooks
- Only activates in dev mode (`VSCODE_DEV` environment variable check)
- Passes injectPath via data parameter to the loader
- Loader file path is relative to the parent module URL

**Call sites:**
- `src/bootstrap-fork.ts:206-208` - Forked worker processes
- Multiple entry points during initialization

---

#### Pattern 2: ESM Specifier Resolution Hook
**Where:** `src/bootstrap-import.ts:87-101`
**What:** Intercepts import specifiers and resolves them to absolute file URLs based on pre-computed package.json mappings.

```typescript
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
- Implements Node.js ESM loader `resolve` hook protocol
- Returns `shortCircuit: true` to prevent further resolution chain
- Falls through to nextResolve for unmapped specifiers
- Maps package names to file:// URLs

---

#### Pattern 3: Package Entry Point Detection
**Where:** `src/bootstrap-import.ts:22-85`
**What:** Initialization function that scans package.json files to determine correct entry points, handling conditional exports for ESM/CJS modules.

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
			// Handle conditional export targets where exports["."].import/default
			// can be a string or an object with a string `default` field.
			// (Added for copilot-sdk)
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
```

**Key aspects:**
- Async initialization that scans all dependencies in package.json
- Handles three entry point resolution strategies in priority order:
  1. ESM exports field (`exports["."].import`)
  2. Conditional exports default field
  3. Legacy main field
- Normalizes entry points to always have .js/.mjs/.cjs extension
- Determines module format from file extension or package.json type field
- Caches results in module-scoped dictionaries `_specifierToUrl` and `_specifierToFormat`
- Gracefully handles missing packages (try-catch)

---

#### Pattern 4: Dynamic fs Module Redirection
**Where:** `src/bootstrap-esm.ts:13-30`
**What:** Injects an ESM loader hook that maps `fs` imports to `node:original-fs` to preserve original fs API in Electron contexts.

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

**Key aspects:**
- Uses data: URL with base64-encoded JavaScript to register inline loader
- Platform-specific: only applies in Electron or Electron-as-Node contexts
- Maps fs to node:original-fs to bypass Electron's fs customizations
- Inline approach avoids separate file dependency

---

#### Pattern 5: Module Resolution In Forked Worker Processes
**Where:** `src/bootstrap-fork.ts:204-208`
**What:** Configures module resolution for worker processes spawned via Node.js child_process.fork().

```typescript
// Remove global paths from the node module lookup (node.js only)
removeGlobalNodeJsModuleLookupPaths();

if (process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH']) {
	devInjectNodeModuleLookupPath(process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH']);
}
```

**Key aspects:**
- Removes global Node.js module search paths to isolate resolution
- Conditionally injects node_modules lookup path via environment variable
- Pattern used consistently across forked processes

---

#### Pattern 6: Global Node.js Module Path Scrubbing
**Where:** `src/bootstrap-node.ts:76-128`
**What:** Removes global Node.js module lookup paths to prevent unintended module resolution from system node_modules.

```typescript
export function removeGlobalNodeJsModuleLookupPaths(): void {
	if (typeof process?.versions?.electron === 'string') {
		return; // Electron disables global search paths in...
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

**Key aspects:**
- Monkeypatches Module._resolveLookupPaths to filter out global paths
- Monkeypatches Module._nodeModulePaths with platform-specific filtering
- Windows-specific: removes drive letters and home directory from search
- Non-Electron specific: skips when running under Electron
- Prevents accidental resolution from system node_modules

---

#### Pattern 7: Working Directory Normalization
**Where:** `src/bootstrap-node.ts:35-55`
**What:** Sets up consistent working directory across all Node.js and Electron processes.

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
- Stores initial cwd in VSCODE_CWD environment variable
- Platform-specific: Windows always sets app folder as cwd
- Called at module load time, runs once per process
- Prevents issues with inherited/incorrect working directories

---

## Architectural Context for Tauri/Rust Port

The bootstrap-import patterns reveal several critical architectural considerations for porting VS Code to Tauri/Rust:

### 1. **Multi-Process Module Resolution**
VS Code relies on Node.js ESM loader hooks (Module.register API) to intercept and redirect module resolution across multiple processes. A Tauri/Rust port would need equivalent mechanisms at the Rust level, potentially involving:
- IPC-based module resolution (communicating with a central resolver service)
- Compile-time or runtime dependency resolution mechanisms
- Dynamic linking with ABI versioning strategies

### 2. **Package.json Scanning and Conditional Exports**
The initialization pattern demonstrates complex package entry point detection including support for:
- Modern conditional exports (`exports["."].import|default`)
- Legacy main field fallback
- Module format detection (ESM vs CommonJS)

A Rust port would need embedded or runtime strategies for managing Wasm module dependencies and format negotiation.

### 3. **Environment-Driven Behavior**
Multiple `VSCODE_*` environment variables control behavior:
- `VSCODE_DEV` - Development mode module resolution
- `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH` - Custom module paths
- `ELECTRON_RUN_AS_NODE` - Platform-specific fs redirection
- `VSCODE_CWD` - Consistent working directory across processes

A Rust port would need to replicate this configuration injection pattern, possibly through a configuration service instead of environment variables.

### 4. **Process-Level Isolation**
Each process (main, workers, forked processes) requires:
- Independent module resolution state
- Custom path filtering (global paths scrubbing)
- Environment variable propagation
- fs module redirection in specific contexts

This suggests a Rust implementation would need per-process resolver instances with shared configuration.

### 5. **Platform-Specific Customizations**
Windows-specific behavior for path filtering and working directory management indicates the port would need:
- Platform abstraction layers for path handling
- Process spawning with proper environment propagation
- Different fs implementations per platform

---

## Related Bootstrap Files

- `src/bootstrap-node.ts` - Node.js process setup and module path management
- `src/bootstrap-esm.ts` - ESM globals and fs redirection setup
- `src/bootstrap-fork.ts` - Forked worker process initialization with logging
- `src/bootstrap-meta.ts` - Product and package metadata loading
- `src/bootstrap-server.ts` - Server-specific environment setup
- `src/bootstrap-cli.ts` - CLI process cleanup

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
