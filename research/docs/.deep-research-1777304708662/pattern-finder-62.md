# Pattern Analysis: extensions/postinstall.mjs

## Scope
File: `extensions/postinstall.mjs` (58 LOC)
Purpose: Build-time cleanup script for TypeScript node_modules after installation

## Patterns Found

#### Pattern 1: Module Path Resolution via import.meta.url
**Where:** `extensions/postinstall.mjs:10`
**What:** Uses ES module metadata to compute absolute file system paths at runtime.
```javascript
const root = path.join(path.dirname(fileURLToPath(import.meta.url)), 'node_modules', 'typescript');
```
**Variations:** This pattern establishes a root reference that all subsequent file operations depend on; crucial for post-install scripts that must work regardless of working directory.

#### Pattern 2: Selective File System Cleanup via Whitelist
**Where:** `extensions/postinstall.mjs:12-24`
**What:** Recursively removes all files from a directory except those in a whitelist set.
```javascript
function processRoot() {
	const toKeep = new Set([
		'lib',
		'package.json',
	]);
	for (const name of fs.readdirSync(root)) {
		if (!toKeep.has(name)) {
			const filePath = path.join(root, name);
			console.log(`Removed ${filePath}`);
			fs.rmSync(filePath, { recursive: true });
		}
	}
}
```
**Variations:** Whitelist approach (keep known items) rather than blacklist; demonstrates defensive cleanup that preserves only essential artifacts.

#### Pattern 3: Regex-based File Filtering with Continue Guards
**Where:** `extensions/postinstall.mjs:26-55`
**What:** Multi-rule filtering that preserves specific files via pattern matching, with early-exit continue statements.
```javascript
function processLib() {
	const toDelete = new Set([
		'tsc.js',
		'_tsc.js',
		'typescriptServices.js',
		'_typescriptServices.js',
	]);

	const libRoot = path.join(root, 'lib');

	for (const name of fs.readdirSync(libRoot)) {
		if (name === 'lib.d.ts' || name.match(/^lib\..*\.d\.ts$/) || name === 'protocol.d.ts') {
			continue;
		}
		if (name === 'typescript.js' || name === 'typescript.d.ts') {
			// used by html and extension editing
			continue;
		}

		if (toDelete.has(name) || name.match(/\.d\.ts$/)) {
			try {
				fs.unlinkSync(path.join(libRoot, name));
				console.log(`removed '${path.join(libRoot, name)}'`);
			} catch (e) {
				console.warn(e);
			}
		}
	}
}
```
**Variations:** Combines exact string matching, regex patterns, and explicit continue conditions; includes inline comments explaining retention rationale (e.g., "used by html and extension editing").

#### Pattern 4: Try-Catch Error Handling in Bulk Operations
**Where:** `extensions/postinstall.mjs:47-52`
**What:** Wraps destructive file operations in try-catch to log warnings without halting the process.
```javascript
try {
	fs.unlinkSync(path.join(libRoot, name));
	console.log(`removed '${path.join(libRoot, name)}'`);
} catch (e) {
	console.warn(e);
}
```
**Variations:** Error is caught and warned but does not re-throw; allows partial success if some files cannot be deleted.

#### Pattern 5: Sequential Function Invocation for Ordered Cleanup
**Where:** `extensions/postinstall.mjs:57-58`
**What:** Two separate functions called in sequence for hierarchical cleanup (root first, then lib subdirectory).
```javascript
processRoot();
processLib();
```
**Variations:** Both functions operate on the same dependency tree but at different levels; processRoot removes everything except lib directory, processLib then refines lib contents.

---

## Relevance to Tauri/Rust Port

This postinstall script reveals VS Code's dependency management strategy: it aggressively prunes the TypeScript npm package to remove build tools (tsc.js, typescriptServices.js), CLI utilities, and non-essential type definitions while preserving core library files and type declarations needed for runtime. The patterns demonstrate how the Electron/Node.js build process manages third-party dependencies at installation time. A Rust/Tauri port would need equivalent mechanisms to manage native dependencies, but would likely shift this responsibility to Cargo's build system (build.rs) or pre-build hooks rather than post-install npm scripts. The selective preservation logic suggests VS Code only needs specific TypeScript runtime components, not the full compiler toolchain—knowledge useful when identifying which dependencies a Rust rewrite might truly require.

