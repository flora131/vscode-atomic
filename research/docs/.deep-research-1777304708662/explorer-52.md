# Partition 52 of 79 — Findings

## Scope
`src/bootstrap-node.ts/` (1 files, 190 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator 52: Node Bootstrap Architecture — Tauri/Rust Porting Constraints

## Scope
Single file analysis: `src/bootstrap-node.ts` (190 LOC)

## Implementation

**src/bootstrap-node.ts**
Core Node.js bootstrap module that initializes the runtime environment before VS Code main/renderer processes start. Contains:

- **Native Module Loader Setup** (lines 57-74): `devInjectNodeModuleLookupPath()` registers a loader hook via `Module.register()` that redirects Node module resolution paths during development
- **Global Module Path Filtering** (lines 76-128): `removeGlobalNodeJsModuleLookupPaths()` strips global node_modules paths from module lookup to enforce sandboxed resolution; contains Electron-specific branch (line 77-79) that detects and bypasses filtering when running under Electron
- **Platform-Specific SIGPIPE Handling** (lines 17-30): Workaround for Electron's missing SIGPIPE handler, critical for pipe-based communication
- **Working Directory Initialization** (lines 32-55): Cross-platform cwd setup with platform-specific behavior for Windows (lines 47-49)
- **Portable Mode Detection** (lines 130-190): `configurePortable()` enables self-contained installations with platform-specific data paths (macOS: nested app bundles, Windows/Linux: data subdirectories)

## Native Module Dependencies

Direct Node.js module requires:
- `node:path` — filesystem path manipulation
- `node:fs` — filesystem operations (used in portable mode detection, line 167)
- `node:module` — Module loader system, used in two forms:
  - `createRequire()` from module API (line 8)
  - `Module.register()` for loader hooks (line 73)
  - Direct `Module` access for internal APIs: `_resolveLookupPaths()`, `_nodeModulePaths()`, `globalPaths`

## Notable Clusters

**Electron Runtime Integration Points**:
- Line 77: `process.versions.electron` check — code branches on Electron presence
- Line 18: SIGPIPE workaround references Electron issue tracking
- Lines 78-79: Explicit Electron-aware module path filtering

**Module Resolution Hooks**:
- Lines 72-73: Uses ES modules loader API (`Module.register()` with import.meta.url)
- Lines 81, 84, 100-101: Patches internal Node.js Module methods (`_resolveLookupPaths`, `_nodeModulePaths`)

---

## Tauri/Rust Porting Implications

This single bootstrap file exposes critical constraints for TypeScript → Rust migration:

1. **Native Module System Dependency**: The code directly instruments Node's internal module resolution (`Module._resolveLookupPaths`, `Module._nodeModulePaths`) via monkey-patching. Tauri/Rust has no equivalent module system to patch; Rust compilation is static and cannot support runtime module redirection.

2. **ES Module Loader Hooks**: The `Module.register()` call (line 73) uses Node.js's stable loader hooks API, a Node-only runtime capability. Rust compilation cannot support this.

3. **Process-Level Signal Handling**: SIGPIPE interception (line 21) requires process-level signal handlers that must be set up at startup. Rust/Tauri would need equivalent signal handling, but the specific Electron+Node.js interaction is unique.

4. **Electron Version Detection**: Lines 77-79 explicitly gate behavior on Electron presence. A Tauri port would lose this dual-runtime support unless Tauri provides equivalent process introspection.

5. **Portable Mode Cross-Platform Logic**: The OS-specific path logic (macOS bundle nesting vs. Windows data folders) can translate to Rust, but would require equivalent path manipulation libraries and cross-platform testing infrastructure.

6. **Module Search Path Filtering**: The Windows-specific drive and home directory filtering (lines 109-127) patches Node's internal search logic; Rust's static module resolution cannot support equivalent runtime filtering.

The bootstrap-node.ts file is fundamentally tied to Node.js runtime semantics. A Tauri port would require a complete architectural redesign of how modules/libraries are discovered, loaded, and isolated at runtime.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts` — 190 lines, Core Node.js bootstrap module

### Per-File Notes

#### `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts`

**Role**

This module runs early in the VS Code process lifecycle, before any main/renderer logic loads. It is imported as a side-effect by `bootstrap-esm.ts` (line 9) and exports three functions consumed by several entry points: `devInjectNodeModuleLookupPath`, `removeGlobalNodeJsModuleLookupPaths`, and `configurePortable`.

---

**Top-Level Initialization (module load-time side effects)**

1. **Stack trace depth** (`bootstrap-node.ts:15`): `Error.stackTraceLimit = 100` expands V8's default limit of 10 frames to 100 immediately on module load.

2. **SIGPIPE guard** (`bootstrap-node.ts:17-30`): A conditional block checks `process.env['VSCODE_HANDLES_SIGPIPE']`. When the variable is absent, a `process.on('SIGPIPE', ...)` handler is registered. The handler uses a one-shot flag `didLogAboutSIGPIPE` (initialized `false` at line 20) to ensure only one `console.error` is emitted, preventing an infinite async loop that can arise when the console itself is in a broken pipe state (comment at line 23-25).

3. **Working directory setup** (`bootstrap-node.ts:35-53`): `setupCurrentWorkingDirectory()` is defined and called immediately at line 55.
   - If `process.env['VSCODE_CWD']` is not already a string (line 42), it writes `process.cwd()` into that environment variable, preserving the original cwd for later lookup by child processes.
   - On `win32` only (line 47), it calls `process.chdir(path.dirname(process.execPath))`, changing the cwd to the directory containing the VS Code executable. This ensures Windows-specific file resolution is anchored to the application folder rather than whatever directory the user launched from.
   - Errors are caught and routed to `console.error` (line 51).

---

**`devInjectNodeModuleLookupPath(injectPath: string)` — lines 62-74**

Guards with two conditions: (a) `process.env['VSCODE_DEV']` must be set (line 63), and (b) `injectPath` must be truthy (line 67-69). When both are satisfied, it calls `Module.register('./bootstrap-import.js', { parentURL: import.meta.url, data: injectPath })` (line 73) using a `require('node:module')` reference created at line 72.

`Module.register` installs a Node.js loader hook (ESM customization hook API). The hook module `bootstrap-import.ts` (described below) intercepts module resolution to redirect named packages to an alternate `node_modules` tree rooted at `injectPath`. This entire mechanism is described as development-only in the JSDoc comment at line 58.

Callers: `src/server-main.ts:14`, `src/server-cli.ts:8`, `src/bootstrap-fork.ts:7-8`. In `bootstrap-fork.ts` the inject path comes from `process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH']`.

---

**`removeGlobalNodeJsModuleLookupPaths()` — lines 76-128**

Early guard at line 77-79: if `process?.versions?.electron` is a string (i.e., running inside Electron), the function returns immediately because Electron already disables global search paths in its own native bindings (`shell/common/node_bindings.cc:653`, referenced in the comment).

When not in Electron the function monkey-patches two internal Node.js `Module` methods:

**`Module._resolveLookupPaths` patch (lines 84-98)**:
- Captures `Module.globalPaths` (line 82) and the original `_resolveLookupPaths` (line 84).
- The replacement function (line 86) calls the original, then walks backward through the resulting paths array comparing each entry against the global paths array suffix using a while loop (lines 90-92). The variable `commonSuffixLength` counts how many tail entries match. It then calls `paths.slice(0, paths.length - commonSuffixLength)` (line 94) to strip exactly those global entries.

**`Module._nodeModulePaths` patch (lines 100-127)**:
- Captures the original `_nodeModulePaths` (line 100).
- The replacement (line 101) calls the original then applies Windows-only filtering (checked at line 103 via the module-level `isWindows` constant from line 12).
- Drive root filtering (lines 109-113): defines `isDrive` as paths of length ≥ 3 ending with `:\`. If `from` is not itself a drive root, removes all entries whose `path.dirname` is a drive root.
- Home directory filtering (lines 115-123): reads `HOMEDRIVE` and `HOMEPATH` environment variables and computes `userDir` as their joined path's parent. If `from` is not equal to `userDir`, removes paths whose parent equals `userDir`.

---

**`configurePortable(product: Partial<IProductConfiguration>)` — lines 133-190**

Returns `{ portableDataPath: string; isPortable: boolean }`.

**`getApplicationPath()` inner function (lines 136-151)**:
- Dev mode (`VSCODE_DEV` set): returns `appRoot` (computed at line 134 as `path.dirname(import.meta.dirname)`).
- macOS (`darwin`): returns `path.dirname(path.dirname(path.dirname(appRoot)))` — three levels up, traversing the macOS `.app` bundle structure.
- Windows with `product.win32VersionedUpdate` (line 146): also three levels up to account for the versioned installation subfolder: `...\Microsoft VS Code Insiders\<version>\resources\app`.
- All other platforms: `path.dirname(path.dirname(appRoot))` — two levels up.

**`getPortableDataPath()` inner function (lines 153-164)**:
- If `process.env['VSCODE_PORTABLE']` is set (line 154), returns it directly.
- Windows/Linux (line 158): `path.join(getApplicationPath(), 'data')`.
- macOS/other: constructs a name from `product.portable` or falls back to `${product.applicationName}-portable-data` (line 162), then places the folder as a sibling of the application path: `path.join(path.dirname(getApplicationPath()), portableDataName)`.

**Detection and environment mutation (lines 166-184)**:
- `portableDataPath` is set at line 166.
- `isPortable` is `true` when the `product` object has no `'target'` key AND `fs.existsSync(portableDataPath)` returns `true` (line 167).
- `portableTempPath` = `path.join(portableDataPath, 'tmp')` (line 168).
- `isTempPortable` = `isPortable && fs.existsSync(portableTempPath)` (line 169).
- If portable: sets `process.env['VSCODE_PORTABLE']` to `portableDataPath` (line 172). Otherwise deletes it (line 174).
- If portable temp exists: sets `TMP`+`TEMP` on Windows (lines 179-180) or `TMPDIR` on other platforms (line 182).
- Returns `{ portableDataPath, isPortable }` (lines 186-189).

Callers: `src/main.ts:10` (main Electron entry), `src/cli.ts:7` (CLI entry).

---

**Imports and Module-Level State**

| Symbol | Source | Usage |
|---|---|---|
| `path` | `node:path` (line 6) | `dirname`, `join`, `relative` throughout |
| `fs` | `node:fs` (line 7) | `fs.existsSync` in `configurePortable` |
| `createRequire` | `node:module` (line 8) | Creates CJS-compatible `require` at line 11 |
| `IProductConfiguration` | `./vs/base/common/product.js` (line 9) | Type-only import for `configurePortable` parameter |
| `require` | line 11 | Used to `require('node:module')` / `require('module')` inside functions |
| `isWindows` | line 12 | Module-level constant `process.platform === 'win32'` |

The `require` at line 11 is produced via `createRequire(import.meta.url)`, which enables CJS-style `require()` within an ESM module context. This is necessary because the module uses `import` statements (ESM) but needs to access `node:module` internals that expose mutable properties (`_resolveLookupPaths`, `_nodeModulePaths`, `globalPaths`) not available through ESM imports.

---

### Cross-Cutting Synthesis

`bootstrap-node.ts` functions as a universal pre-flight module that runs once during process startup to normalize Node.js runtime behavior across all VS Code process types (main, renderer fork, server, CLI). It operates through three mechanisms: environment variable snapshots (`VSCODE_CWD`, `VSCODE_PORTABLE`), signal and error handler registration (`SIGPIPE`), and monkey-patching of internal Node.js module resolution APIs (`Module._resolveLookupPaths`, `Module._nodeModulePaths`). The Electron guard in `removeGlobalNodeJsModuleLookupPaths` reflects that Electron already performs equivalent global path suppression natively, so the patch is only needed for pure Node.js server and CLI processes. The portable mode detection uses a filesystem existence check rather than a configuration flag, meaning portability is an opt-in triggered by the presence of a `data/` directory alongside the application. The development-only loader hook registered by `devInjectNodeModuleLookupPath` delegates to a companion module (`bootstrap-import.ts`) that implements the actual ESM resolve hook logic. This separation keeps `bootstrap-node.ts` free of async code, since `Module.register` itself is synchronous even though the hook it installs operates asynchronously.

---

### Out-of-Partition References

- `/Users/norinlavaee/vscode-atomic/src/vs/base/common/product.ts` — defines `IProductConfiguration` interface (line 67); fields used in `configurePortable`: `win32VersionedUpdate` (line 81), `portable` (line 162 of bootstrap-node.ts), `applicationName` (line 83), `target` presence check.
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-import.ts` — the ESM loader hook module registered by `devInjectNodeModuleLookupPath`; implements `initialize(injectPath)` and `resolve(specifier, context, nextResolve)` hooks.
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-esm.ts` — imports `bootstrap-node.ts` as a side-effect (line 9) to run its load-time initialization; also installs an `fs`→`original-fs` redirect hook for Electron.
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-fork.ts` — calls `removeGlobalNodeJsModuleLookupPaths()` and `devInjectNodeModuleLookupPath(process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH'])`.
- `/Users/norinlavaee/vscode-atomic/src/main.ts` — calls `configurePortable` from `bootstrap-node.ts` (line 10).
- `/Users/norinlavaee/vscode-atomic/src/cli.ts` — calls `configurePortable` from `bootstrap-node.ts` (line 7).
- `/Users/norinlavaee/vscode-atomic/src/server-main.ts` — calls both `devInjectNodeModuleLookupPath` and `removeGlobalNodeJsModuleLookupPaths` (line 14).
- `/Users/norinlavaee/vscode-atomic/src/server-cli.ts` — calls `devInjectNodeModuleLookupPath` (line 8).

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Bootstrap Node Module Loading Patterns

Research into core IDE functionality bootstrap in `src/bootstrap-node.ts` reveals several critical patterns for managing native module loading, environment initialization, and Node.js module resolution in an Electron/TypeScript environment.

## Core Patterns from bootstrap-node.ts

#### Pattern: ESM-Native Module Bridge via createRequire

**Where:** `src/bootstrap-node.ts:8, 11`

**What:** Establishes CommonJS require capability in ESM context using Node.js module API for native module access.

```typescript
import { createRequire } from 'node:module';
import type { IProductConfiguration } from './vs/base/common/product.js';

const require = createRequire(import.meta.url);
```

**Variations / call-sites:**
- `src/vs/workbench/api/node/extHostExtensionService.ts:28` - Extension host uses identical pattern
- `src/vs/workbench/api/node/extensionHostProcess.ts:30-31` - Extension process bootstrap reuses this pattern
- `src/bootstrap-fork.ts:7` - Fork bootstrap imports from the pattern but doesn't use createRequire directly
- Referenced in `src/main.ts:10` via imported bootstrap-node functions


#### Pattern: Module Loader Hook Registration via Module.register()

**Where:** `src/bootstrap-node.ts:62-74`

**What:** Injects custom module resolution paths at boot time using Node.js loader hooks for development mode module redirection.

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

**Variations / call-sites:**
- Paired with `src/bootstrap-import.ts` which exports `initialize()` and `resolve()` hooks
- Import hook reads package.json and maps module specifiers to node_modules paths
- Conditional on `VSCODE_DEV` environment flag
- Called from `src/bootstrap-fork.ts:7` via import


#### Pattern: Native Module Blocklist via Module._load Override

**Where:** `src/bootstrap-node.ts:76-128` (removeGlobalNodeJsModuleLookupPaths), extended in `src/vs/workbench/api/node/extensionHostProcess.ts:76-92`

**What:** Intercepts Node.js module loading to filter global paths and block unsafe native modules like 'natives'.

```typescript
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

	const originalNodeModulePaths = Module._nodeModulePaths;
	Module._nodeModulePaths = function (from: string): string[] {
		let paths: string[] = originalNodeModulePaths(from);
		if (!isWindows) {
			return paths;
		}

		// On Windows, remove drive(s) and users' home directory from search paths
		const isDrive = (p: string) => p.length >= 3 && p.endsWith(':\\');

		if (!isDrive(from)) {
			paths = paths.filter(p => !isDrive(path.dirname(p)));
		}

		if (process.env.HOMEDRIVE && process.env.HOMEPATH) {
			const userDir = path.dirname(path.join(process.env.HOMEDRIVE, process.env.HOMEPATH));

			const isUsersDir = (p: string) => path.relative(p, userDir).length === 0;

			if (!isUsersDir(from)) {
				paths = paths.filter(p => !isUsersDir(path.dirname(p)));
			}
		}

		return paths;
	};
}
```

**Variations / call-sites:**
- Extension host in `src/vs/workbench/api/node/extensionHostProcess.ts:85-91` shows simpler override for 'natives' module blocking
- Windows-specific path filtering logic accounts for drive letters and user directory isolation
- Skipped when running in Electron (which already manages global paths)


#### Pattern: Signal Handler Setup for Electron Compatibility

**Where:** `src/bootstrap-node.ts:14-30`

**What:** Configures SIGPIPE handling and error stack trace limits for Electron environment compatibility.

```typescript
// increase number of stack frames(from 10, https://github.com/v8/v8/wiki/Stack-Trace-API)
Error.stackTraceLimit = 100;

if (!process.env['VSCODE_HANDLES_SIGPIPE']) {
	// Workaround for Electron not installing a handler to ignore SIGPIPE
	// (https://github.com/electron/electron/issues/13254)
	let didLogAboutSIGPIPE = false;
	process.on('SIGPIPE', () => {
		// See https://github.com/microsoft/vscode-remote-release/issues/6543
		// In certain situations, the console itself can be in a broken pipe state
		// so logging SIGPIPE to the console will cause an infinite async loop
		if (!didLogAboutSIGPIPE) {
			didLogAboutSIGPIPE = true;
			console.error(new Error(`Unexpected SIGPIPE`));
		}
	});
}
```

**Variations / call-sites:**
- Guard flag `VSCODE_HANDLES_SIGPIPE` allows parent process to skip rehandling
- Similar pattern in `src/vs/workbench/api/node/extensionHostProcess.ts:40-52` for experimental warnings filtering
- Stack trace limit increase (100) enables better debugging across large call stacks


#### Pattern: Platform-Specific Current Working Directory Setup

**Where:** `src/bootstrap-node.ts:32-55`

**What:** Initializes consistent working directory handling across platforms with environment variable caching.

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

**Variations / call-sites:**
- Called at module load time (line 55) - synchronous initialization
- Guards against re-initialization if `VSCODE_CWD` already set by parent process
- Windows-specific behavior changes working directory to app folder immediately


#### Pattern: Portable Mode Configuration with Path Resolution

**Where:** `src/bootstrap-node.ts:130-190`

**What:** Detects and configures portable installation mode with platform-aware path resolution and environment variable setup.

```typescript
export function configurePortable(product: Partial<IProductConfiguration>): { portableDataPath: string; isPortable: boolean } {
	const appRoot = path.dirname(import.meta.dirname);

	function getApplicationPath(): string {
		if (process.env['VSCODE_DEV']) {
			return appRoot;
		}

		if (process.platform === 'darwin') {
			return path.dirname(path.dirname(path.dirname(appRoot)));
		}

		// appRoot = ..\Microsoft VS Code Insiders\<version>\resources\app
		if (process.platform === 'win32' && product.win32VersionedUpdate) {
			return path.dirname(path.dirname(path.dirname(appRoot)));
		}

		return path.dirname(path.dirname(appRoot));
	}

	function getPortableDataPath(): string {
		if (process.env['VSCODE_PORTABLE']) {
			return process.env['VSCODE_PORTABLE'];
		}

		if (process.platform === 'win32' || process.platform === 'linux') {
			return path.join(getApplicationPath(), 'data');
		}

		const portableDataName = product.portable || `${product.applicationName}-portable-data`;
		return path.join(path.dirname(getApplicationPath()), portableDataName);
	}

	const portableDataPath = getPortableDataPath();
	const isPortable = !('target' in product) && fs.existsSync(portableDataPath);
	const portableTempPath = path.join(portableDataPath, 'tmp');
	const isTempPortable = isPortable && fs.existsSync(portableTempPath);

	if (isPortable) {
		process.env['VSCODE_PORTABLE'] = portableDataPath;
	} else {
		delete process.env['VSCODE_PORTABLE'];
	}

	if (isTempPortable) {
		if (process.platform === 'win32') {
			process.env['TMP'] = portableTempPath;
			process.env['TEMP'] = portableTempPath;
		} else {
			process.env['TMPDIR'] = portableTempPath;
		}
	}

	return {
		portableDataPath,
		isPortable
	};
}
```

**Variations / call-sites:**
- Called in `src/main.ts:34` during Electron main process initialization
- Returns detection result AND applies environment variables in single call
- Platform paths differ significantly: macOS uses `../../../` traversal, Windows/Linux use `data/` subdirectory
- Environment variable setup is conditional on actual directory existence (not just environment flags)


#### Pattern: Import.meta-based Runtime Path Resolution

**Where:** `src/bootstrap-node.ts:8-11, 134`

**What:** Uses ESM import.meta globals for runtime file location resolution enabling path-relative operations.

```typescript
import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);

// Later in configurePortable():
const appRoot = path.dirname(import.meta.dirname);
```

**Variations / call-sites:**
- `import.meta.url` passed to createRequire establishes correct module context for require()
- `import.meta.dirname` (newer Node.js) used instead of path.dirname(fileURLToPath(import.meta.url))
- Also in `src/bootstrap-import.ts:25` via fileURLToPath conversion for package.json location
- In `src/vs/workbench/api/node/extHostExtensionService.ts:60` for _nodeModulePaths fallback


## Integration Patterns

These bootstrap functions are consumed in specific patterns:

1. **Main Process Initialization** (`src/main.ts:10, 34`)
   - Imports `configurePortable` from bootstrap-node
   - Calls it early to enable portable data paths before Electron app initialization

2. **Fork Process Setup** (`src/bootstrap-fork.ts:7-8`)
   - Imports both `removeGlobalNodeJsModuleLookupPaths` and `devInjectNodeModuleLookupPath`
   - Sets up isolated module resolution for forked Node processes

3. **Extension Host** (`src/vs/workbench/api/node/extHostExtensionService.ts`)
   - Implements parallel Module._load patching for VSCode API injection
   - Uses Module._resolveLookupPaths override for 'vsda' module compatibility workaround

4. **Extension Process** (`src/vs/workbench/api/node/extensionHostProcess.ts`)
   - Simpler blocking pattern for 'natives' module only
   - Patches process.exit and process.crash to prevent extension crashes


## Implications for Tauri/Rust Port

The bootstrap patterns reveal critical requirements for IDE-level functionality:

- **Native module resolution interception**: Direct Module._load/_resolveLookupPaths patching requires runtime hooking capability that doesn't exist in Tauri's model (no V8 module system)
- **Signal handling**: SIGPIPE workarounds suggest platform-specific process signal management needed at the system level
- **Portable mode detection**: Multi-tier path resolution with filesystem checks would need Rust-level implementation
- **Module loader hooks**: Node.js loader hooks API (Module.register) are specific to Node.js ecosystem and require replacement architecture for Rust FFI
- **Working directory isolation**: Process-level CWD management via process.chdir() would need platform-specific Rust syscalls
- **Environment variable cascading**: The pattern of setting env vars for child processes (VSCODE_PORTABLE, VSCODE_CWD, TMP/TMPDIR) suggests state management would need careful Rust/IPC coordination

A Tauri port would need to replicate these capabilities via:
1. Rust module system equivalents or FFI layer
2. Platform-specific native code for signal handling
3. Configuration file-based portable mode detection
4. IPC-based module resolution instead of in-process hooking
5. Rust process management for working directory and environment setup

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
