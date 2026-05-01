# Partition 62 of 79 — Findings

## Scope
`extensions/postinstall.mjs/` (1 files, 58 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Partition 62: extensions/postinstall.mjs

## Implementation
- `extensions/postinstall.mjs` — Node.js maintenance script that runs post-npm-install to remove unnecessary TypeScript artifacts from the `node_modules/typescript` package (deletes compiler binaries and redundant type definitions, keeping only library definitions needed for HTML and extension editing)

## Summary

The `extensions/postinstall.mjs` is a single-file build pipeline maintenance script executed after `npm install` completes. Its sole responsibility is to clean up the TypeScript package in `node_modules` by:

1. Removing non-essential TypeScript distribution files from the root (`tsc.js`, `typescriptServices.js`, etc.)
2. Pruning TypeScript lib directory to keep only core type definitions (`lib.d.ts`, `lib.*.d.ts`, `protocol.d.ts`) and the main library module (`typescript.js`, `typescript.d.ts`)

**Porting implication**: A Rust-based VS Code implementation would require equivalent post-build cleanup of the Node.js dependency tree. If Tauri/Rust retains Node.js tooling for built-in extension support, this postinstall behavior must be replicated in the Rust build pipeline (e.g., via a build script or custom installer logic). If the architecture eliminates npm dependencies entirely, this script becomes obsolete.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/postinstall.mjs` (58 LOC)

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/extensions/postinstall.mjs`

- **Role:** A Node.js ESM post-install maintenance script that runs after `npm install` is executed inside the `extensions/` directory. Its sole purpose is to aggressively prune the `typescript` package that was just installed into `extensions/node_modules/typescript`, keeping only the runtime API surface needed by the HTML language server and extension editing infrastructure, and discarding everything else to reduce disk footprint.

- **Key symbols:**
  - `root` (line 10) — computed absolute path to `extensions/node_modules/typescript`, derived at module load time using `import.meta.url` → `fileURLToPath` → `path.dirname` → `path.join`.
  - `processRoot()` (lines 12–24) — function that prunes top-level entries of the `typescript` package directory.
  - `processLib()` (lines 26–55) — function that prunes individual files inside `extensions/node_modules/typescript/lib`.

- **Control flow:**
  1. Module initializes `root` at line 10.
  2. `processRoot()` is called at line 57.
     - Defines a `toKeep` set containing exactly two entries: `'lib'` and `'package.json'` (lines 13–16).
     - Reads all directory entries of `root` with `fs.readdirSync(root)` (line 17).
     - For each entry NOT in `toKeep`, constructs its absolute path and calls `fs.rmSync(filePath, { recursive: true })` (line 21), logging the removal.
     - Net result: everything in the `typescript` package root is deleted except the `lib/` subdirectory and `package.json`.
  3. `processLib()` is called at line 58 (after `processRoot()` completes).
     - Defines a `toDelete` set of four specific filenames (lines 27–33): `tsc.js`, `_tsc.js`, `typescriptServices.js`, `_typescriptServices.js`.
     - Resolves `libRoot` as `path.join(root, 'lib')` (line 35).
     - Reads all entries of `libRoot` with `fs.readdirSync(libRoot)` (line 37).
     - For each entry in `libRoot`, applies three guard conditions in order:
       - **Keep (continue)** if the name equals `'lib.d.ts'`, matches the regex `/^lib\..*\.d\.ts$/`, or equals `'protocol.d.ts'` (line 38) — these are standard TypeScript library declaration files needed at runtime.
       - **Keep (continue)** if the name equals `'typescript.js'` or `'typescript.d.ts'` (lines 41–43) — the comment at line 42 states these are "used by html and extension editing".
       - **Delete** if the name is in `toDelete` OR if it matches `/\.d\.ts$/` (line 46) — non-exempted `.d.ts` files are unlinked via `fs.unlinkSync` (line 48), with any errors caught and printed as warnings (lines 50–52).
     - Net result: inside `lib/`, only `typescript.js`, `typescript.d.ts`, `lib.d.ts`, `lib.*.d.ts` (pattern-matched), and `protocol.d.ts` survive; the four named JS files and all other `.d.ts` files are removed.

- **Data flow:**
  - Input: filesystem state of `extensions/node_modules/typescript/` as installed by npm.
  - Transformation: two sequential destructive filesystem passes — first deletes top-level package artifacts, then selectively deletes files within `lib/`.
  - Output: a slimmed `typescript` package directory containing only `package.json`, `lib/typescript.js`, `lib/typescript.d.ts`, `lib/lib.d.ts`, `lib/lib.*.d.ts` globs, and `lib/protocol.d.ts`.
  - Side effects: each deletion is logged to stdout via `console.log` (lines 20, 49); errors during `processLib()` unlinking are written to stderr via `console.warn` (line 51).

- **Dependencies:**
  - Node.js built-in `fs` module (ESM import, line 6) — for `readdirSync`, `rmSync`, `unlinkSync`.
  - Node.js built-in `path` module (ESM import, line 7) — for `path.join`, `path.dirname`.
  - Node.js built-in `url` module (ESM import, line 8) — for `fileURLToPath`, used to convert `import.meta.url` to a filesystem path.
  - No third-party dependencies; no exports; no async operations.
  - Invoked as a postinstall lifecycle hook, meaning it is triggered automatically by npm after it finishes installing packages in `extensions/`.

---

### Cross-Cutting Synthesis

This script represents a build-time dependency hygiene mechanism that surgically reduces the installed `typescript` package to a minimal API surface. The script preserves `typescript.js` and `typescript.d.ts` (the language service API entry points) along with the standard `lib.*.d.ts` declaration files and `protocol.d.ts`, while stripping the compiler CLI (`tsc.js`), the bundled `typescriptServices.js`, and all other declaration files. This indicates the `extensions/` directory consumes TypeScript purely as a language-service library, not as a compiler toolchain.

For a Tauri/Rust port, this script has no direct runtime relevance — it is purely a Node.js/npm artifact. However, it documents which parts of the TypeScript package are actually consumed at runtime: `typescript.js` (the language service API) and the standard library `.d.ts` declaration files. A Tauri port that embeds language intelligence (e.g., via a TypeScript language server bridged over IPC) would need to bundle precisely these same artifacts, or invoke the TypeScript language server (`tsserver`) as a sidecar process. The script also signals that the HTML language server and extension editing subsystem (`extensions/`) depend on the TypeScript language service API, making those two subsystems the primary consumers to target when designing the Rust-side LSP bridge.

---

### Out-of-Partition References

- `extensions/node_modules/typescript/` — the directory this script operates on; managed by npm and populated before this script runs.
- `extensions/package.json` — must declare `"postinstall": "node postinstall.mjs"` (or equivalent) in its `scripts` field for this script to be triggered by npm. Not read in this partition.
- The HTML language server extension (within `extensions/`) is referenced in-comment at line 42 as a consumer of `typescript.js` and `typescript.d.ts`; its implementation is in a separate partition.
- Extension editing infrastructure (referenced in-comment at line 42) — separate partition; consumes the preserved `typescript.js` API surface.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Postinstall Build Glue: Patterns for a Rust/Tauri Port

## Research Question
What postinstall patterns exist in VS Code that a Rust/Tauri port would need to replicate for build/install glue?

## Summary
VS Code's postinstall hooks perform three core functions: **dependency optimization** (removing unused files), **asset bundling** (copying WASM, tokenizer files, language server data), and **native module wiring** (symlinks, npm configuration for node-gyp builds). A Rust port would need equivalents for asset management and native module linking, though many npm-specific tasks would be unnecessary.

---

## Pattern 1: Recursive Module Cleanup and Dependency Pruning

**Where:** `/home/norinlavaee/projects/vscode-atomic/extensions/postinstall.mjs:12-55`  
**What:** Removes unused files from TypeScript npm package to reduce size; allows selective preservation of only critical files (type definitions, compiler APIs).

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

**Key aspects:**
- Uses blocklist (files to delete) and allowlist (files to keep)
- Preserves only .d.ts type definitions and specific APIs needed at runtime
- Handles errors gracefully (logs warnings instead of failing)
- Directly manipulates node_modules filesystem post-install

**For Rust port:** A Rust equivalent would need logic to strip downloaded dependency artifacts (language server binaries, type stubs) down to essentials. Could use a manifest file specifying what to preserve from each dependency.

---

## Pattern 2: Complex Native Module Installation with Environment-Specific Configuration

**Where:** `/home/norinlavaee/projects/vscode-atomic/build/npm/postinstall.ts:38-99`  
**What:** Handles conditional native module builds (C++ extensions via node-gyp), Docker container isolation for remote dependencies, environment variable inheritance, and npm config propagation.

```typescript
async function npmInstallAsync(dir: string, opts?: child_process.SpawnOptions): Promise<void> {
	const finalOpts: child_process.SpawnOptions = {
		env: { ...process.env },
		...(opts ?? {}),
		cwd: path.join(root, dir),
		shell: true,
	};
	const command = process.env['npm_command'] || 'install';
	if (process.env['VSCODE_REMOTE_DEPENDENCIES_CONTAINER_NAME'] && /^(.build\/distro\/npm\/)?remote$/.test(dir)) {
		const syncOpts: child_process.SpawnSyncOptions = {
			env: finalOpts.env,
			cwd: root,
			stdio: 'inherit',
			shell: true,
		};
		const userinfo = os.userInfo();
		log(dir, `Installing dependencies inside container ${process.env['VSCODE_REMOTE_DEPENDENCIES_CONTAINER_NAME']}...`);
		if (process.env['npm_config_arch'] === 'arm64') {
			run('sudo', ['docker', 'run', '--rm', '--privileged', 'vscodehub.azurecr.io/multiarch/qemu-user-static@sha256:...'], syncOpts);
		}
		run('sudo', ['docker', 'run', '-e', 'GITHUB_TOKEN', '-v', `${process.env['VSCODE_HOST_MOUNT']}:/root/vscode`, ...], syncOpts);
	} else {
		log(dir, 'Installing dependencies...');
		const output = await spawnAsync(npm, command.split(' '), finalOpts);
		if (output.trim()) {
			for (const line of output.trim().split('\n')) {
				log(dir, line);
			}
		}
	}
	removeParcelWatcherPrebuild(dir);
}

function setNpmrcConfig(dir: string, env: NodeJS.ProcessEnv) {
	const npmrcPath = path.join(root, dir, '.npmrc');
	const lines = fs.readFileSync(npmrcPath, 'utf8').split('\n');
	for (const line of lines) {
		const trimmedLine = line.trim();
		if (trimmedLine && !trimmedLine.startsWith('#')) {
			const [key, value] = trimmedLine.split('=');
			env[`npm_config_${key}`] = value.replace(/^"(.*)"$/, '$1');
		}
	}
	env['npm_config_node_gyp'] = process.platform === 'win32'
		? path.join(import.meta.dirname, 'gyp', 'node_modules', '.bin', 'node-gyp.cmd')
		: path.join(import.meta.dirname, 'gyp', 'node_modules', '.bin', 'node-gyp');
}
```

**Key aspects:**
- Separates native (C++ builds) and JS-only installations for sequential vs. parallel execution
- Reads platform-specific compiler configuration from `.npmrc` files
- Supports Docker container isolation for cross-platform builds
- Manages environment variables for build tools (CC, CXX, LDFLAGS)
- Handles node-gyp override to use bundled version instead of system

**For Rust port:** Would need equivalent logic for fetching and configuring platform-specific binaries (language servers, tools). Instead of node-gyp, would use Rust build tools (cargo-build, cross-compilation toolchains). Could leverage Cargo's own platform-specific build features or external build coordination scripts.

---

## Pattern 3: Asset Bundling and Static File Distribution

**Where:** `/home/norinlavaee/projects/vscode-atomic/extensions/copilot/script/postinstall.ts:64-187`  
**What:** Copies WASM files, language server binaries, tokenizer data, and prebuilt native modules from node_modules into dist/ directory; selectively filters files based on naming patterns.

```typescript
async function copyCopilotCliPrebuildFiles() {
	const sourceDir = path.join(REPO_ROOT, 'node_modules', '@github', 'copilot', 'prebuilds');
	const targetDir = path.join(REPO_ROOT, 'node_modules', '@github', 'copilot', 'sdk', 'prebuilds');
	await fs.promises.rm(targetDir, { recursive: true, force: true });
	await fs.promises.mkdir(targetDir, { recursive: true });
	await fs.promises.cp(sourceDir, targetDir, {
		recursive: true, force: true, filter: (src) => {
			try {
				if (fs.statSync(src).isFile()) {
					return src.endsWith('computer.node') || src.endsWith('win32.node');
				}
				return true;
			} catch {
				return true;
			}
		}
	});
}

async function main() {
	await fs.promises.mkdir(path.join(REPO_ROOT, '.build'), { recursive: true });
	const vendoredTiktokenFiles = ['src/platform/tokenizer/node/cl100k_base.tiktoken', 'src/platform/tokenizer/node/o200k_base.tiktoken'];
	for (const tokens of vendoredTiktokenFiles) {
		await compressTikToken(tokens, `dist/${path.basename(tokens)}`);
	}
	await copyStaticAssets([
		...treeSitterGrammars.map(grammar => `node_modules/@vscode/tree-sitter-wasm/wasm/${grammar.name}.wasm`),
		'node_modules/@vscode/tree-sitter-wasm/wasm/tree-sitter.wasm',
		'node_modules/@github/blackbird-external-ingest-utils/pkg/nodejs/external_ingest_utils_bg.wasm',
	], 'dist');
	await removeCopilotCLIShim();
	await copyCopilotCliWorkerFiles();
	await copyCopilotCliSharpFiles();
	// ... more copy operations
}
```

**Key aspects:**
- Centralizes heterogeneous asset types into dist/ directory for packaging
- Applies filter functions to selectively copy platform-specific binaries (e.g., only .node files matching patterns)
- Supports compression pipelines (tiktoken tokenizer files)
- Declares grammar metadata (tree-sitter) to coordinate with copy logic
- Separates removal (cleanup), setup (mkdir), and copying phases

**For Rust port:** Core insight—build output needs a "dist" assembly phase that gathers WASM modules, prebuilt language server binaries (LSP implementations), tokenizer data, and tree-sitter grammars. Tauri's build system would handle this via build scripts, but the separation of concerns (cleanup → setup → asset placement) maps well to Cargo build.rs scripts with manifest definitions.

---

## Integration Points in Codebase

- **Configured via npm scripts:** `/home/norinlavaee/projects/vscode-atomic/package.json:21` defines `postinstall` hook as `node build/npm/postinstall.ts`
- **State tracking:** `/home/norinlavaee/projects/vscode-atomic/build/npm/installStateHash.ts` (not shown) implements caching to skip redundant installs
- **Parallel execution:** Root-level install runs JS-only deps in parallel (8 concurrency max), native builds sequentially to avoid node-gyp file lock conflicts

---

## Conclusion for Rust/Tauri Port

A Rust port would not need npm postinstall hooks, but would require equivalent build-time glue:

1. **Asset bundling phase** (during `cargo build`) to assemble WASM, grammars, and LSP binaries into a dist/ directory
2. **Platform detection** to conditionally include/fetch binaries for the target OS and architecture
3. **Dependency pruning** if vendoring any binary dependencies (less common in Rust than npm)
4. **Symlink/hardlink setup** for development testing (e.g., Claude agent harness mirror paths)

Tauri's build script support and Cargo's built-in platform-specific dependency features would cover most of this without needing custom Node scripts.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
