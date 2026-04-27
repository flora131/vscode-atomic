# Pattern Finder 57: Module Resolution & Bootstrapping Patterns

## Research Context
Patterns extracted from `/src/bootstrap-import.ts` and related bootstrap infrastructure for porting VS Code from TypeScript/Electron to Tauri/Rust.

---

## Pattern 1: Runtime Module Specifier Resolution Hook
**Found in**: `src/bootstrap-import.ts:87-101`
**Description**: Defers module resolution requests to next loader in chain after mapping specifiers to file URLs.

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

**Key aspects**:
- Implements Node.js module resolution hook protocol
- Maps string specifiers to absolute file URLs
- Declares module format (ESM vs CommonJS)
- Short-circuits resolution chain when match found
- Falls through to default resolver otherwise

---

## Pattern 2: Conditional Package Export Resolution
**Found in**: `src/bootstrap-import.ts:28-76`
**Description**: Resolves correct entry point from package.json with support for conditional exports and fallback chains.

```typescript
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
```

**Key aspects**:
- Reads package.json from dependencies
- Checks `exports["."]` field first (modern conditional exports)
- Handles nested object structures with `default` fallback
- Falls back to `main` field
- Defaults to `index.js` if nothing specified
- Normalizes extensions to .js/.mjs/.cjs

---

## Pattern 3: Module Format Detection
**Found in**: `src/bootstrap-import.ts:70-76`
**Description**: Determines whether module is ESM or CommonJS based on file extension or package.json type field.

```typescript
const isModule = main.endsWith('.mjs')
	? true
	: main.endsWith('.cjs')
		? false
		: pkgJson.type === 'module';
_specifierToFormat[name] = isModule ? 'module' : 'commonjs';
```

**Key aspects**:
- File extension takes precedence (.mjs = ESM, .cjs = CommonJS)
- Falls back to package.json `type` field
- Caches format decision for later resolution
- Defaults to CommonJS if ambiguous

---

## Pattern 4: Cache-Based Lookup Initialization
**Found in**: `src/bootstrap-import.ts:19-26`
**Description**: Pre-populates lookup tables during initialization to enable fast runtime resolution.

```typescript
const _specifierToUrl: Record<string, string> = {};
const _specifierToFormat: Record<string, string> = {};

export async function initialize(injectPath: string): Promise<void> {
	// populate mappings

	const injectPackageJSONPath = fileURLToPath(new URL('../package.json', pathToFileURL(injectPath)));
	const packageJSON = JSON.parse(String(await promises.readFile(injectPackageJSONPath)));

	for (const [name] of Object.entries(packageJSON.dependencies)) {
```

**Key aspects**:
- Maintains two lookup caches: specifier->URL and specifier->format
- Initialization async function scans package.json dependencies
- Pre-caches all resolutions before runtime module resolution occurs
- Enables O(1) resolution during runtime

---

## Pattern 5: Error Isolation During Initialization
**Found in**: `src/bootstrap-import.ts:78-82`
**Description**: Catches and logs resolution errors per dependency without breaking initialization loop.

```typescript
} catch (err) {
	console.error(name);
	console.error(err);
}
```

**Key aspects**:
- Logs package name followed by error
- Continues processing remaining dependencies
- Allows partial initialization success
- Enables graceful degradation

---

## Pattern 6: Node.js Module Hook Registration
**Found in**: `src/bootstrap-node.ts:72-74`
**Description**: Registers custom loader hook using Node.js Module.register API.

```typescript
// register a loader hook
const Module = require('node:module');
Module.register('./bootstrap-import.js', { parentURL: import.meta.url, data: injectPath });
```

**Key aspects**:
- Uses node:module.register() API
- Specifies loader path and parentURL
- Passes injectPath as data to loader context
- Enables module-level customization without process-wide changes

---

## Pattern 7: Electron-Specific File System Remapping
**Found in**: `src/bootstrap-node.ts:13-30`
**Description**: Conditionally installs fs->original-fs redirection for Electron environments.

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
- Detects Electron environment via environment variable or process.versions
- Dynamically creates loader hook as base64-encoded string
- Maps fs module to original-fs to bypass Electron's fs hooks
- Uses data: URL scheme for inline hook registration
- Only applies in Electron runtime, not in Node.js

---

## Bootstrap Infrastructure Summary

The bootstrap-import.ts file operates within a multi-stage initialization system:

1. **bootstrap-meta.ts**: Loads product.json and package.json configuration
2. **bootstrap-node.ts**: Sets up working directory, portable mode, and registers Module.register() hooks
3. **bootstrap-esm.ts**: Initializes NLS (i18n) configuration
4. **bootstrap-import.ts**: Custom module resolution for dependency mapping (registered via Module.register)

This layered approach separates concerns:
- Global process setup (cwd, signal handlers)
- Module resolution customization (loader hooks)
- ESM-specific initialization (NLS)
- Configuration loading (product/package JSON)

The patterns demonstrate TypeScript/Electron-specific module resolution challenges that would require different approaches in a Rust/Tauri port.

