# Partition 57 of 79 — Findings

## Scope
`src/bootstrap-import.ts/` (1 files, 101 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator Findings: Partition 57 - Bootstrap Import Shim

## Scope Analysis

**File Analyzed:** `src/bootstrap-import.ts` (101 LOC)

This file is a Node.js module loader hook that handles runtime module resolution and redirection of package imports from the remote folder to the `node_modules` directory. It is NOT related to the core IDE functionality of VS Code that would need porting to Tauri/Rust.

## Implementation

- `src/bootstrap-import.ts` — Module loader initialization shim for Node.js ESM/CommonJS resolution redirection

## Purpose for Porting Research

**Not relevant to Tauri/Rust port.** This file serves a very specific purpose: redirecting package imports when VS Code runs from source (development mode). Its functionality is:

- Scanning `package.json` dependencies
- Building a module resolution cache that maps specifier names to file URLs
- Hooking into Node.js module resolution (via the `resolve` hook per Node.js loader API)
- Determining module formats (ESM vs CommonJS) based on package metadata

In a Tauri/Rust port, this entire layer would be unnecessary because:
1. The module resolution system would be replaced by Rust's crate/module system
2. Package management would shift from npm to Cargo
3. Runtime module loading hooks would not exist in the same form

## Summary

The bootstrap-import shim is a development-time utility for Node.js-based development workflows and has no bearing on porting VS Code's core IDE features (editing, language intelligence, debugging, source control, terminal, navigation) to Tauri/Rust. It would simply not be needed in a Rust-based implementation with Cargo package management.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/Users/norinlavaee/vscode-atomic/src/bootstrap-import.ts` (101 LOC)

---

### Per-File Notes

#### `src/bootstrap-import.ts`

- **Role:** A Node.js ESM module loader hook that intercepts `import` specifier resolution at runtime. When VS Code runs out of source (development mode), it redirects bare package specifier imports (e.g., `import 'some-package'`) to absolute file URLs pointing into a particular `node_modules` folder determined by an injected path. This implements the Node.js Module Customization Hooks API (`initialize` + `resolve`) as documented at https://nodejs.org/docs/latest/api/module.html#initialize.

- **Key symbols:**
  - Module-level lookup maps `_specifierToUrl` (`bootstrap-import.ts:19`) — `Record<string, string>` mapping package name to absolute `file://` URL of that package's entry point.
  - Module-level lookup maps `_specifierToFormat` (`bootstrap-import.ts:20`) — `Record<string, string>` mapping package name to either `'module'` or `'commonjs'`.
  - `initialize(injectPath: string): Promise<void>` (`bootstrap-import.ts:22`) — The hook's initialization export, called by Node.js when the loader is registered. Receives `injectPath` (the path of a folder alongside whose `package.json` the `node_modules` to redirect into lives).
  - `resolve(specifier, context, nextResolve)` (`bootstrap-import.ts:87`) — The hook's resolve export, called by Node.js for every `import` statement before default resolution.
  - `resolveCondition(v: unknown): string | undefined` (`bootstrap-import.ts:43`) — inner helper inside `initialize` for reading conditional export values from `package.json`.

- **Control flow:**

  **`initialize` phase (`bootstrap-import.ts:22-85`):**
  1. Converts `injectPath` string to an absolute filesystem path for the adjacent `package.json` using `fileURLToPath` + `pathToFileURL` (`bootstrap-import.ts:25`).
  2. Reads and JSON-parses that `package.json` (`bootstrap-import.ts:26`).
  3. Iterates over every key in `packageJSON.dependencies` (`bootstrap-import.ts:28`).
  4. For each dependency name, constructs a path to `node_modules/<name>/package.json` and reads/parses it (`bootstrap-import.ts:30-31`).
  5. Determines the package entry point via a priority chain (`bootstrap-import.ts:38-64`):
     - If `pkgJson.exports['.']` is a plain string, that is the entry (`bootstrap-import.ts:40-41`).
     - If `pkgJson.exports['.']` is an object, calls `resolveCondition` on `.import` first, then `.default` (`bootstrap-import.ts:55`). `resolveCondition` returns the value if it is a string, or reads its `.default` sub-key if it is an object (`bootstrap-import.ts:43-54`).
     - Falls back to `pkgJson.main` as a string (`bootstrap-import.ts:59`).
     - Ultimate fallback is `'index.js'` (`bootstrap-import.ts:63`).
  6. Appends `.js` extension if the resolved main has no JS-family extension (`bootstrap-import.ts:65-67`).
  7. Stores the fully-resolved `file://` URL into `_specifierToUrl[name]` (`bootstrap-import.ts:69`).
  8. Determines module format (`bootstrap-import.ts:71-76`): `.mjs` → `'module'`; `.cjs` → `'commonjs'`; otherwise checks `pkgJson.type === 'module'`. Stores into `_specifierToFormat[name]`.
  9. Errors per-dependency are caught and logged with `console.error`, not thrown, so one bad dependency doesn't abort the rest (`bootstrap-import.ts:78-81`).
  10. Logs a summary message at the end (`bootstrap-import.ts:84`).

  **`resolve` hook phase (`bootstrap-import.ts:87-101`):**
  1. Looks up `specifier` in `_specifierToUrl` (`bootstrap-import.ts:89`).
  2. If found, returns immediately (`shortCircuit: true`) with the mapped URL and format (`bootstrap-import.ts:91-95`), bypassing all other resolve hooks and Node.js's built-in resolution.
  3. If not found, delegates to `nextResolve(specifier, context)` (`bootstrap-import.ts:100`), which continues down the hook chain to Node.js default resolution.

- **Data flow:**
  - Input: `injectPath` string (arrives via `Module.register` `data` field in `bootstrap-node.ts:73`).
  - `injectPath` → `fileURLToPath(new URL('../package.json', pathToFileURL(injectPath)))` → `injectPackageJSONPath` (absolute path to the controlling `package.json`).
  - `injectPackageJSONPath` + each `name` in `packageJSON.dependencies` → per-package `package.json` path → parsed `pkgJson` object → resolved `main` entry point string → absolute `mainPath` → `file://` URL stored in `_specifierToUrl[name]`.
  - At import time: bare specifier string → lookup in `_specifierToUrl` → either `{ format, shortCircuit: true, url }` returned to Node.js, or forwarded to `nextResolve`.

- **Dependencies:**
  - `node:url` (`fileURLToPath`, `pathToFileURL`) — `bootstrap-import.ts:13`.
  - `node:fs` (`promises.readFile`) — `bootstrap-import.ts:14`.
  - `node:path` (`join`) — `bootstrap-import.ts:15`.
  - Registered as a loader by `src/bootstrap-node.ts:73` via `Module.register('./bootstrap-import.js', { parentURL: import.meta.url, data: injectPath })` inside `devInjectNodeModuleLookupPath()`.
  - The `injectPath` value that is passed determines which `package.json`/`node_modules` tree is used as the redirect target.
  - Activation is gated by `process.env['VSCODE_DEV']` in the caller (`bootstrap-node.ts:63`); this hook only operates when running out of source.

---

### Cross-Cutting Synthesis

`src/bootstrap-import.ts` is a pure Node.js ESM loader hook (Module Customization Hooks API). It serves a single development-time purpose: when VS Code is run directly from its TypeScript source tree, bare npm package specifiers that would normally be resolved via Node's default algorithm are instead redirected to a specific `node_modules` tree derived from a caller-supplied inject path. The hook is registered by `devInjectNodeModuleLookupPath` in `src/bootstrap-node.ts:73`, which is itself only invoked when `VSCODE_DEV` is set. Initialization pre-computes two lookup tables (`_specifierToUrl`, `_specifierToFormat`) by walking all entries in the inject path's `package.json` dependencies and resolving each package entry point through the full `exports` conditional chain (ESM-preferred, with CJS fallback). The `resolve` hook then intercepts each `import` at load time and short-circuits to the precomputed URL if the specifier is a known dependency, otherwise deferring to the next hook. This mechanism has no bearing on VS Code's production Electron runtime and is entirely absent from non-development builds.

---

### Out-of-Partition References

- `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts:62-73` — `devInjectNodeModuleLookupPath()` is the sole caller that registers `bootstrap-import.js` as a loader via `Module.register`, passing `injectPath` as `data`. Guards activation with `process.env['VSCODE_DEV']`.
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts:76-100` — `removeGlobalNodeJsModuleLookupPaths()` — sibling function in the same file that complements this redirection by removing Node.js global module lookup paths, relevant context for understanding the full module resolution strategy.
- The `eslint.config.js:1978` entry lists `bootstrap-import.ts` among the top-level bootstrap scripts, confirming it is treated as a standalone script target distinct from the main extension/service code under `src/vs/`.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
