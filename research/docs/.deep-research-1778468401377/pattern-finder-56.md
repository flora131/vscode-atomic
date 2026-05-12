# Global Injection Patterns in VS Code Bootstrap

This partition analyzes how VS Code injects globals before application code runs, a critical pattern for porting to Tauri/Rust.

## Patterns Found

#### Pattern 1: Product and Package Metadata Globals
**Where:** `src/bootstrap-esm.ts:33-35`
**What:** Spreads product and package JSON metadata into globalThis for application-wide access.
```typescript
// Prepare globals that are needed for running
globalThis._VSCODE_PRODUCT_JSON = { ...product };
globalThis._VSCODE_PACKAGE_JSON = { ...pkg };
globalThis._VSCODE_FILE_ROOT = import.meta.dirname;
```
**Variations / call-sites:** 
- `src/vs/amdX.ts:207` - reads `globalThis._VSCODE_PRODUCT_JSON` to determine if built
- `src/vs/amdX.ts:232` - reads same to determine built status and ASAR usage

#### Pattern 2: Module Resolution Hook Registration
**Where:** `src/bootstrap-esm.ts:14-30`
**What:** Registers a Node.js module loader hook via data URL to redirect fs module to original-fs in Electron contexts.
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
**Variations / call-sites:** 
- `src/bootstrap-import.ts:87-100` - implements custom resolve hook for redirecting node_modules

#### Pattern 3: NLS Language Configuration Global
**Where:** `src/bootstrap-esm.ts:64`
**What:** Sets NLS resolved language onto globalThis from environment configuration.
```typescript
globalThis._VSCODE_NLS_LANGUAGE = nlsConfig?.resolvedLanguage;
```
**Variations / call-sites:** 
- Multiple references in `src/bootstrap-esm.ts` (lines 64, 78, 94) for NLS messages setup

#### Pattern 4: Async NLS Messages Population
**Where:** `src/bootstrap-esm.ts:77-99`
**What:** Asynchronously loads translated message strings from filesystem and injects into globalThis with fallback handling.
```typescript
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
```
**Variations / call-sites:** 
- Fallback pattern shown at `src/bootstrap-esm.ts:92-98`
- Environment variable parsing at `src/bootstrap-esm.ts:55-67`

#### Pattern 5: Module Import Specifier Redirection Mapping
**Where:** `src/bootstrap-import.ts:19-85`
**What:** Pre-populates `_specifierToUrl` and `_specifierToFormat` maps that are used by module resolver to redirect imports to correct node_modules paths with format detection.
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
**Variations / call-sites:** 
- Hook resolver at `src/bootstrap-esm.ts:14-30` uses `register()` to install this logic

#### Pattern 6: Trusted Types Policy Global
**Where:** `src/vs/amdX.ts:76` and `src/vs/amdX.ts:88`
**What:** Sets trusted types policy from globalThis for AMD loader script URL sanitization, with fallback to browser API.
```typescript
if (this._isRenderer) {
	this._amdPolicy = globalThis._VSCODE_WEB_PACKAGE_TTP ?? window.trustedTypes?.createPolicy('amdLoader', {
		createScriptURL(value: any) {
			if (value.startsWith(window.location.origin)) {
```
**Variations / call-sites:** 
- Web worker variant at `src/vs/amdX.ts:88` uses `globalThis.trustedTypes` directly

## Summary

The bootstrap patterns reveal VS Code's multi-stage initialization strategy:

1. **Static globals** (`_VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON`, `_VSCODE_FILE_ROOT`) are set immediately on globalThis from metadata loaded at startup
2. **Module resolution hooks** are registered at the Node.js level via data URLs and base64 encoding to intercept imports
3. **Async configuration** (NLS messages, language settings) are populated asynchronously with fallback strategies
4. **Import specifier mappings** are built by scanning package.json files and stored in module-level maps accessed by the resolver
5. **Context-specific policies** (trusted types) are set on globals for renderer/webworker contexts

For a Tauri/Rust port, this would require:
- Equivalent globalThis injection mechanism in the webview initialization (likely via JavaScript evaluation or webview API)
- Rust-side module resolution layer that replaces Node.js's module loading hooks
- Async initialization sequence that populates globals before application code executes
- Package metadata scanning and mapping in Rust before webview loads
