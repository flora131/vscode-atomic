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

