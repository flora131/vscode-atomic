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

