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
