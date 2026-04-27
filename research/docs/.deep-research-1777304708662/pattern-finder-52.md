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

