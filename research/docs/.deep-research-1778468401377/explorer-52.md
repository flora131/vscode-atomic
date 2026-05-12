# Partition 52 of 80 — Findings

## Scope
`src/bootstrap-node.ts/` (1 files, 190 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
## File Locations for VS Code Core IDE Porting Research (Scope: bootstrap-node.ts)

### Implementation
- `src/bootstrap-node.ts` — Node.js bootstrap initialization for VS Code processes; handles SIGPIPE management, working directory setup, module lookup redirection, and portable mode configuration for Electron-based runtime

### Related Support Files
- `src/bootstrap-import.ts` — Node.js module loader hook for redirecting package imports to local node_modules; enables development-time module redirection via Module.register()
- `src/bootstrap-meta.ts` — Product and package configuration bootstrap that injects build-time metadata and development overrides

---

## Summary

The `bootstrap-node.ts` file (190 LOC) establishes Node.js-level initialization for VS Code's Electron runtime. It handles platform-specific concerns including SIGPIPE signal management (Electron workaround), working directory establishment across Windows/macOS/Linux, module path isolation to prevent global package pollution, and portable installation mode detection. This bootstrap is foundational infrastructure for VS Code's current Node/Electron architecture and demonstrates platform-specific runtime configuration that would require substantial reimplementation in a Tauri/Rust port. The file depends on the product configuration interface and works alongside two complementary bootstrap files: `bootstrap-import.ts` for dynamic module resolution and `bootstrap-meta.ts` for metadata injection.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `src/bootstrap-node.ts` (190 LOC)

---

### Per-File Notes

#### `src/bootstrap-node.ts`

- **Role:** Module-level bootstrap that runs once when any Node.js/Electron process in VS Code starts. It installs a SIGPIPE guard, normalises the current working directory, and exports three utilities consumed downstream: `devInjectNodeModuleLookupPath` (dev-only module redirection), `removeGlobalNodeJsModuleLookupPaths` (scope-narrowing for the Node module resolver), and `configurePortable` (portable-install detection and environment setup).

- **Key symbols:**
  - `Error.stackTraceLimit = 100` (`src/bootstrap-node.ts:15`) — module-level side effect raising V8 stack-trace depth.
  - `setupCurrentWorkingDirectory` (`src/bootstrap-node.ts:35`) — IIFE-style private function called immediately at line 55.
  - `devInjectNodeModuleLookupPath(injectPath: string): void` (`src/bootstrap-node.ts:62`) — exported; registers a Node ESM loader hook via `Module.register`.
  - `removeGlobalNodeJsModuleLookupPaths(): void` (`src/bootstrap-node.ts:76`) — exported; monkey-patches `Module._resolveLookupPaths` and `Module._nodeModulePaths`.
  - `configurePortable(product: Partial<IProductConfiguration>): { portableDataPath: string; isPortable: boolean }` (`src/bootstrap-node.ts:133`) — exported; returns portable state and mutates env vars.
  - `isWindows` (`src/bootstrap-node.ts:12`) — module-scoped boolean used in `removeGlobalNodeJsModuleLookupPaths` and `configurePortable`.

- **Control flow:**

  1. **SIGPIPE guard** (`src/bootstrap-node.ts:17-30`): If `process.env['VSCODE_HANDLES_SIGPIPE']` is absent, registers a `process.on('SIGPIPE', …)` handler. The handler uses a `didLogAboutSIGPIPE` boolean latch (declared at line 20) so only the first SIGPIPE event logs an error, preventing an infinite async loop caused by a broken console pipe.

  2. **`setupCurrentWorkingDirectory()`** (`src/bootstrap-node.ts:35-53`): If `VSCODE_CWD` env var is not already a string, writes `process.cwd()` into it. On Windows, additionally calls `process.chdir(path.dirname(process.execPath))` to force the process directory to the application folder. The whole body is wrapped in a `try/catch` that logs to stderr.

  3. **`devInjectNodeModuleLookupPath`** (`src/bootstrap-node.ts:62-74`): Returns early when `VSCODE_DEV` is not set. When active, calls `Module.register('./bootstrap-import.js', { parentURL: import.meta.url, data: injectPath })` at line 73 to attach an ESM loader hook that rewrites module resolution to the injected path.

  4. **`removeGlobalNodeJsModuleLookupPaths`** (`src/bootstrap-node.ts:76-128`): Returns early when running inside Electron (`process.versions.electron` is a string). Otherwise:
     - Saves a reference to `Module.globalPaths` and `Module._resolveLookupPaths` (lines 82-84).
     - Replaces `Module._resolveLookupPaths` with a wrapper (lines 86-98) that strips the common suffix of global paths from any returned paths array by counting `commonSuffixLength` from the tail.
     - Replaces `Module._nodeModulePaths` with a wrapper (lines 100-127) that, on Windows only, filters out drive-root directories (paths whose parent ends with `:\`) unless the originating `from` path is itself a drive root, and also filters out the Windows users directory (`HOMEDRIVE` + `HOMEPATH` parent) unless `from` is that directory.

  5. **`configurePortable`** (`src/bootstrap-node.ts:133-190`):
     - `getApplicationPath()` (line 136): Returns the install root depending on platform/product config. Dev mode returns `appRoot` directly; macOS returns three `path.dirname` levels up; Windows versioned-update and Linux return two or three levels up.
     - `getPortableDataPath()` (line 153): If `VSCODE_PORTABLE` env var is set, returns it directly. On Win32/Linux returns `<applicationPath>/data`. On macOS computes a sibling directory named by `product.portable` or `<applicationName>-portable-data`.
     - Checks `isPortable` at line 167: true when the product has no `target` field **and** the `portableDataPath` directory exists on disk (`fs.existsSync`).
     - Checks `isTempPortable` at line 169: true when portable and a `tmp` sub-directory exists inside `portableDataPath`.
     - Mutates environment: sets or deletes `VSCODE_PORTABLE` (lines 171-175); sets `TMP`/`TEMP` (Win32) or `TMPDIR` (others) to `portableTempPath` when `isTempPortable` (lines 177-184).
     - Returns `{ portableDataPath, isPortable }`.

- **Data flow:**
  - `setupCurrentWorkingDirectory` reads `process.env['VSCODE_CWD']`, `process.cwd()`, and `process.execPath`; writes `process.env['VSCODE_CWD']`; may mutate `process.cwd` via `chdir`.
  - `devInjectNodeModuleLookupPath` receives `injectPath` as a string argument and forwards it as loader `data` to the `bootstrap-import.js` ESM hook via `Module.register`.
  - `removeGlobalNodeJsModuleLookupPaths` reads `process.versions.electron`, `Module.globalPaths`, `process.env.HOMEDRIVE`, `process.env.HOMEPATH`; produces no return value; side effect is patched `Module` internals.
  - `configurePortable` receives `product: Partial<IProductConfiguration>` (specifically fields `win32VersionedUpdate`, `portable`, `applicationName`, `target`); reads `import.meta.dirname`, `process.env['VSCODE_DEV']`, `process.env['VSCODE_PORTABLE']`, `process.platform`; calls `fs.existsSync` twice; writes `process.env['VSCODE_PORTABLE']`, `process.env['TMP']`, `process.env['TEMP']`, or `process.env['TMPDIR']`; returns `{ portableDataPath, isPortable }`.

- **Dependencies:**
  - `node:path` (`src/bootstrap-node.ts:6`) — `path.dirname`, `path.join`, `path.relative`.
  - `node:fs` (`src/bootstrap-node.ts:7`) — `fs.existsSync`.
  - `node:module` (`src/bootstrap-node.ts:8`) — `createRequire` (used to create the module-scoped `require` at line 11); `Module` internals accessed dynamically inside functions.
  - `IProductConfiguration` from `src/vs/base/common/product.ts` (`src/bootstrap-node.ts:9`) — type-only import used as the parameter type for `configurePortable`.

---

### Cross-Cutting Synthesis

`src/bootstrap-node.ts` is a thin but critical process-initialisation layer that fires before any application logic. Its design is entirely imperative: three module-level side effects (stack limit, SIGPIPE handler, CWD normalisation) execute unconditionally at import time, while the three exported functions act as configurable, call-site-driven extensions that callers invoke during their own startup sequences. The SIGPIPE guard and CWD setup paper over OS and Electron differences so that all downstream code can rely on stable process state. `devInjectNodeModuleLookupPath` bridges the ESM loader hook mechanism to dev-time source overrides, delegating the actual path rewriting logic to `bootstrap-import.js`. `removeGlobalNodeJsModuleLookupPaths` enforces hermetic module resolution by surgically trimming Node's built-in global search path lists, with additional Windows-specific filtering for drive roots and user directories. `configurePortable` is the most stateful function: it performs two filesystem probes and then commits portable-mode decisions into environment variables that every other subsystem will observe thereafter. All three exported functions accept the fact that they may be called from different process types (renderer, extension host, CLI) and guard themselves appropriately (dev-only checks, Electron detection).

---

### Out-of-Partition References

The following sibling bootstrap files are referenced by or closely related to `src/bootstrap-node.ts` but are covered by other partitions:

- `src/bootstrap-import.ts` — the ESM loader hook registered at `src/bootstrap-node.ts:73` via `Module.register('./bootstrap-import.js', …)`.
- `src/bootstrap-meta.ts` — sibling bootstrap providing `import.meta`-level metadata (out of scope for this partition).
- `src/bootstrap-esm.ts` — ESM entry-point bootstrap (out of scope).
- `src/bootstrap-fork.ts` — fork-process bootstrap (out of scope).
- `src/bootstrap-cli.ts` — CLI-process bootstrap (out of scope).
- `src/bootstrap-server.ts` — server-process bootstrap (out of scope).
- `src/vs/base/common/product.ts:67` — defines `IProductConfiguration`, the interface used as the parameter type of `configurePortable`.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder: Node Bootstrap Module Loading (Partition 52)

## Patterns Found in `src/bootstrap-node.ts`

#### Pattern: CommonJS Module Loading via createRequire

**Where:** `src/bootstrap-node.ts:8-11`

**What:** ES module imports createRequire from node:module to enable dynamic require() calls in ESM context.

```typescript
import { createRequire } from 'node:module';
import type { IProductConfiguration } from './vs/base/common/product.js';

const require = createRequire(import.meta.url);
```

**Variations / call-sites:**
- Used throughout the file to dynamically load Node.js internal modules (`require('node:module')`, `require('module')`)
- Necessary bridge for ESM-to-CJS interop in bootstrap context
- `import.meta.url` provides ESM context for require resolution


#### Pattern: Loader Hook Registration with Module.register()

**Where:** `src/bootstrap-node.ts:72-73`

**What:** Registers a loader hook for custom module resolution during development using Node's experimental loader API.

```typescript
// register a loader hook
const Module = require('node:module');
Module.register('./bootstrap-import.js', { parentURL: import.meta.url, data: injectPath });
```

**Variations / call-sites:**
- Wrapped in `devInjectNodeModuleLookupPath()` export
- Only executes when `VSCODE_DEV` environment variable is set
- Passes `parentURL` context and custom `data` payload to the loader hook
- Hooks into module resolution for development workflows (alternative to ASAR)


#### Pattern: Module Path Resolution Override

**Where:** `src/bootstrap-node.ts:84-98`

**What:** Monkey-patches Node's internal `Module._resolveLookupPaths()` to filter out global module paths.

```typescript
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
```

**Variations / call-sites:**
- Filters trailing global paths by comparing suffix arrays
- Skipped entirely when running under Electron (which disables global search paths natively)
- Part of `removeGlobalNodeJsModuleLookupPaths()` export


#### Pattern: Platform-Specific Module Path Filtering

**Where:** `src/bootstrap-node.ts:100-127`

**What:** Overrides `Module._nodeModulePaths()` to remove system-wide search paths on Windows.

```typescript
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
```

**Variations / call-sites:**
- Windows-specific filtering to prevent loading from system drives and user home directories
- Uses `process.env.HOMEDRIVE` and `process.env.HOMEPATH` for user directory detection
- Skips filtering if module request originates from the filtered path itself
- Part of `removeGlobalNodeJsModuleLookupPaths()` export


#### Pattern: Environment Variable Initialization at Bootstrap

**Where:** `src/bootstrap-node.ts:17-30, 42-43`

**What:** Initializes critical environment variables before module loading and sets up signal handling.

```typescript
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

if (typeof process.env['VSCODE_CWD'] !== 'string') {
	process.env['VSCODE_CWD'] = process.cwd();
}
```

**Variations / call-sites:**
- Deduplicates initialization by checking for existing environment variables
- `VSCODE_HANDLES_SIGPIPE` flag prevents duplicate signal handlers
- `VSCODE_CWD` cached from earliest process launch to maintain consistent cwd across child processes
- Signal handling specifically addresses Electron SIGPIPE issue (#13254)


#### Pattern: Portable Mode Configuration with Environment Variables

**Where:** `src/bootstrap-node.ts:154-183`

**What:** Detects portable mode and conditionally configures temp/data paths via environment variables.

```typescript
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
```

**Variations / call-sites:**
- Platform-specific temp directory names: `TMP`/`TEMP` on Windows, `TMPDIR` on Unix
- Respects pre-existing `VSCODE_PORTABLE` environment variable (from parent process)
- Detection logic: checks for portable directory existence and absence of 'target' in product config
- Nested temp detection: separately checks for `tmp/` subdirectory within portable path


#### Pattern: Import.meta Usage for ESM Path Resolution

**Where:** `src/bootstrap-node.ts:11, 73-74, 134`

**What:** Uses `import.meta.url` and `import.meta.dirname` for ESM-native path resolution without __dirname/__filename.

```typescript
const require = createRequire(import.meta.url);
// ... later ...
Module.register('./bootstrap-import.js', { parentURL: import.meta.url, data: injectPath });
// ... and ...
const appRoot = path.dirname(import.meta.dirname);
```

**Variations / call-sites:**
- `import.meta.url`: File URL for createRequire and Module.register context
- `import.meta.dirname`: Direct directory path (newer Node feature, used for appRoot calculation)
- Eliminates need for CJS-era `__dirname` global in ESM files
- Three separate use patterns: require creation, loader registration, and app path calculation

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
