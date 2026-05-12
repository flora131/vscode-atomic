# Partition 56 of 80 — Findings

## Scope
`src/bootstrap-import.ts/` (1 files, 101 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator Results: Partition 56

## Scope
- `src/bootstrap-import.ts/` (1 file, 101 LOC)

## Implementation

- `src/bootstrap-import.ts` — Node.js module loader hook for redirecting package resolution to `node_modules` from remote execution context. Provides `initialize()` to build specifier-to-URL mappings from package.json dependencies, and `resolve()` hook to intercept module resolution.

## Summary

This partition contains a single TypeScript file that implements a Node.js module initialization and resolution system. The file is a module loader hook (per Node.js API specification) designed to handle ESM/CommonJS module format detection and path resolution when running in a remote context. It parses package.json exports and main fields to determine entry points and infers module types from file extensions and package.json metadata. While not directly implementing core IDE functionality (editing, language intelligence, debugging, etc.), this utility would be relevant to a Tauri/Rust port insofar as it manages Node.js runtime module dependencies—a layer that might be eliminated or reimplemented differently in a Rust-based architecture that doesn't rely on Node.js module resolution.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `src/bootstrap-import.ts` (101 LOC)

### Per-File Notes

#### `src/bootstrap-import.ts`

- **Role:** Implements a Node.js ESM loader hook (per the Node.js `module.register()` API, https://nodejs.org/docs/latest/api/module.html#initialize). The hook is used exclusively during development (`VSCODE_DEV` mode, see `src/bootstrap-node.ts:62-74`) to redirect bare package specifiers to their resolved file URLs under a specific `node_modules` directory—needed when VS Code is run from a remote folder whose `node_modules` is not on the standard lookup path.

- **Key symbols:**
  - `_specifierToUrl` (`src/bootstrap-import.ts:19`) — module-level `Record<string, string>` mapping a package name (e.g. `"@github/copilot-sdk"`) to its resolved `file://` URL entry point.
  - `_specifierToFormat` (`src/bootstrap-import.ts:20`) — module-level `Record<string, string>` mapping the same package name to either `"module"` (ESM) or `"commonjs"` (CJS).
  - `initialize(injectPath: string)` (`src/bootstrap-import.ts:22`) — async function exported per the Node.js loader-hook contract. Called once by the Node.js runtime after `Module.register()` passes `injectPath` as the `data` argument.
  - `resolve(specifier, context, nextResolve)` (`src/bootstrap-import.ts:87`) — async function exported per the Node.js loader-hook contract. Called for every `import` statement or `require()` that the runtime resolves.
  - `resolveCondition(v)` (`src/bootstrap-import.ts:43`) — inner helper closure inside `initialize` that extracts a string path from a conditional export target (handles both plain-string and `{ default: string }` shapes, added specifically for the `copilot-sdk` package).

- **Control flow:**

  `initialize`:
  1. Converts `injectPath` to a file URL, then resolves `../package.json` relative to it (`src/bootstrap-import.ts:25`).
  2. Reads and JSON-parses that `package.json` (`src/bootstrap-import.ts:26`).
  3. Iterates over every key in `packageJSON.dependencies` (`src/bootstrap-import.ts:28`).
  4. For each dependency, reads the corresponding `node_modules/<name>/package.json` (`src/bootstrap-import.ts:30-31`).
  5. Determines the entry-point path (variable `main`) by preferring `exports["."].import`, then `exports["."].default`, then `exports["."]` as a string, then the `main` field, and finally falling back to `"index.js"` (`src/bootstrap-import.ts:38-64`).
  6. Appends `.js` if the resolved path has no recognized JS extension (`src/bootstrap-import.ts:65-67`).
  7. Writes the resolved `file://` URL into `_specifierToUrl[name]` (`src/bootstrap-import.ts:69`).
  8. Determines module format: `.mjs` → `"module"`, `.cjs` → `"commonjs"`, otherwise inspects `pkgJson.type === "module"` (`src/bootstrap-import.ts:71-76`).
  9. Writes the format into `_specifierToFormat[name]` (`src/bootstrap-import.ts:76`).
  10. Catches per-package errors and logs them without aborting the loop (`src/bootstrap-import.ts:78-81`).
  11. Logs a summary line after all packages are processed (`src/bootstrap-import.ts:84`).

  `resolve`:
  1. Looks up `specifier` in `_specifierToUrl` (`src/bootstrap-import.ts:89`).
  2. If found, returns a short-circuit resolution object with `url`, `format`, and `shortCircuit: true` (`src/bootstrap-import.ts:91-95`); Node.js will not call any further loader hooks.
  3. If not found, delegates to `nextResolve(specifier, context)` — the next hook in the chain, or the Node.js default resolver (`src/bootstrap-import.ts:100`).

- **Data flow:**
  - Input: `injectPath` (string, a filesystem path to a directory containing `package.json` and `node_modules/`) is received by `initialize` from the Node.js runtime, originating from `Module.register()` in `src/bootstrap-node.ts:73`.
  - `initialize` reads two levels of `package.json` from disk (parent and per-package), transforms those JSON structures into URL strings and format strings, and stores them into the two module-level maps.
  - `resolve` reads from those maps at import time and returns either a resolved URL object or passes control forward.
  - No external state is mutated beyond the two module-level records.

- **Dependencies:**
  - `node:url` — `fileURLToPath`, `pathToFileURL` (`src/bootstrap-import.ts:13`)
  - `node:fs` — `promises.readFile` (`src/bootstrap-import.ts:14`)
  - `node:path` — `join` (`src/bootstrap-import.ts:15`)
  - All dependencies are Node.js built-ins; no third-party packages are imported.

### Cross-Cutting Synthesis

`src/bootstrap-import.ts` is a thin development-time shim wired into VS Code's Node.js remote execution bootstrap. It is registered as a loader hook via `src/bootstrap-node.ts:73` (`Module.register('./bootstrap-import.js', ...)`) only when `VSCODE_DEV` is set. Its sole purpose is to redirect package specifiers to the `node_modules` tree of a specific `injectPath`, bypassing the default Node.js module lookup algorithm. The file does not participate in any IDE feature (editing, language intelligence, debugging, SCM, terminal). For a Tauri/Rust port, this layer is relevant only insofar as it demonstrates VS Code's reliance on Node.js's ESM loader hook API and its dynamic `package.json` parsing at startup—a mechanism that would have no direct counterpart in a Rust runtime and would need to be replaced by a Rust-native dependency bundling or linking strategy.

### Out-of-Partition References

- `src/bootstrap-node.ts` — Calls `Module.register('./bootstrap-import.js', ...)` at line 73 inside `devInjectNodeModuleLookupPath()`, which is the sole registration site for this loader hook; also guards registration behind the `VSCODE_DEV` environment variable at line 63.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
