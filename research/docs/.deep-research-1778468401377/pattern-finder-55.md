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

