# VS Code Extension Bundling with esbuild: Core Patterns

## Summary

VS Code's extension bundling architecture centers on a shared esbuild abstraction (`esbuild-common.mts`) that provides unified build/watch logic for all ~33 extensions. All patterns call `esbuild.build()` for one-shot builds and `esbuild.context()` for watch mode. The architecture demonstrates how a Tauri/Rust port would still require a JS bundler: extensions are written in TypeScript and must be bundled to JavaScript for distribution. The patterns show configuration composition, plugin architecture, and parallel multi-target builds (node/browser).

---

## Pattern Examples: esbuild Integration in Extension Bundling

#### Pattern 1: Shared Build Runner with Watch/Build Modes
**Where:** `extensions/esbuild-common.mts:18-51`
**What:** Core abstraction providing unified build/watch orchestration for all extensions. Accepts config, switches between `esbuild.build()` (one-shot) and `esbuild.context()` + watch (incremental).

```typescript
export async function runBuild(
	config: RunConfig,
	baseOptions: esbuild.BuildOptions,
	args: string[],
	didBuild?: (outDir: string) => unknown,
): Promise<void> {
	let outdir = config.outdir;
	const outputRootIndex = args.indexOf('--outputRoot');
	if (outputRootIndex >= 0) {
		const outputRoot = args[outputRootIndex + 1];
		const outputDirName = path.basename(outdir);
		outdir = path.join(outputRoot, outputDirName);
	}

	const resolvedOptions: esbuild.BuildOptions = {
		...baseOptions,
		entryPoints: config.entryPoints,
		outdir,
		...(config.additionalOptions || {}),
	};

	const isWatch = args.indexOf('--watch') >= 0;
	if (isWatch) {
		const ctx = await esbuild.context(resolvedOptions);
		await watchWithParcel(ctx, config.srcDir, () => didBuild?.(outdir));
	} else {
		try {
			await esbuild.build(resolvedOptions);
			await didBuild?.(outdir);
		} catch {
			process.exit(1);
		}
	}
}
```

**Variations / call-sites:**
- Used by `esbuild-extension-common.mts:49` via `runBuild()` wrapper
- Called directly by 33+ extension build scripts
- `didBuild` callback used for post-build actions (e.g., copying files, moving source maps)

---

#### Pattern 2: Platform-Aware Base Configuration Composition
**Where:** `extensions/esbuild-extension-common.mts:16-46`
**What:** Wraps `runBuild()` with extension-specific defaults, adding platform-aware configuration (node vs. browser). Platform determines mainFields, aliases, and environment defines.

```typescript
function resolveBaseOptions(config: ExtensionRunConfig): esbuild.BuildOptions {
	const options: esbuild.BuildOptions = {
		platform: config.platform,
		bundle: true,
		minify: true,
		treeShaking: true,
		sourcemap: true,
		target: ['es2024'],
		external: ['vscode'],
		format: config.format ?? 'cjs',
		logOverride: {
			'import-is-undefined': 'error',
		},
	};

	if (config.platform === 'node') {
		options.mainFields = ['module', 'main'];
	} else if (config.platform === 'browser') {
		options.mainFields = ['browser', 'module', 'main'];
		options.alias = {
			'path': 'path-browserify',
		};
		options.define = {
			'process.platform': JSON.stringify('web'),
			'process.env': JSON.stringify({}),
			'process.env.BROWSER_ENV': JSON.stringify('true'),
		};
	}

	return options;
}
```

**Variations / call-sites:**
- Used by `esbuild-extension-common.mts:49` `run()` export
- All 33 extensions use `run()` indirectly; only complex builds (copilot, html-language-features) override with custom options

---

#### Pattern 3: Simple Extension Build (Single Entry Point)
**Where:** `extensions/configuration-editing/esbuild.mts:11-18`
**What:** Minimal extension pattern—delegates to `run()` helper with single entry point and platform specification.

```typescript
run({
	platform: 'node',
	entryPoints: {
		'configurationEditingMain': path.join(srcDir, 'configurationEditingMain.ts'),
	},
	srcDir,
	outdir: outDir,
}, process.argv);
```

**Variations / call-sites:**
- Standard pattern for ~25 extensions (git, markdown-language-features, json-language-features client, etc.)
- Some extend with `additionalOptions` for externals or tsconfigPath (git, markdown-language-features)

---

#### Pattern 4: Multi-Target Parallel Build
**Where:** `extensions/json-language-features/esbuild.mts:10-43`
**What:** Single build script compiling client and server as separate targets in parallel using `Promise.all()` and different platforms/formats.

```typescript
await Promise.all([
	// Build client
	run({
		platform: 'node',
		entryPoints: {
			'jsonClientMain': path.join(extensionRoot, 'client', 'src', 'node', 'jsonClientMain.ts'),
		},
		srcDir: path.join(extensionRoot, 'client', 'src'),
		outdir: path.join(extensionRoot, 'client', 'dist', 'node'),
		additionalOptions: {
			tsconfig: path.join(extensionRoot, 'client', 'tsconfig.json'),
		},
	}, process.argv),

	// Build server
	run({
		platform: 'node',
		format: 'esm',
		entryPoints: {
			'jsonServerMain': path.join(extensionRoot, 'server', 'src', 'node', 'jsonServerNodeMain.ts'),
		},
		srcDir: path.join(extensionRoot, 'server', 'src'),
		outdir: path.join(extensionRoot, 'server', 'dist', 'node'),
		additionalOptions: {
			tsconfig: path.join(extensionRoot, 'server', 'tsconfig.json'),
			external: ['vscode', 'typescript', 'fs'],
			banner: {
				js: `import { createRequire } from 'module'; const require = createRequire(import.meta.url);`,
			},
		},
	}, process.argv),
]);
```

**Variations / call-sites:**
- Used by json-language-features, html-language-features, markdown-language-features (client/server splits)
- Demonstrates format override (`esm` vs `cjs`), custom tsconfig, and ESM compatibility banners

---

#### Pattern 5: Post-Build Callbacks for File Operations
**Where:** `extensions/git/esbuild.mts:12-38` + `extensions/esbuild-common.mts:44-51`
**What:** Async callback passed to `run()` for non-TS file copying after bundling. Extends `runBuild()` with custom `didBuild` handler.

```typescript
// git/esbuild.mts
async function copyNonTsFiles(outDir: string): Promise<void> {
	const entries = await fs.readdir(srcDir, { withFileTypes: true, recursive: true });
	for (const entry of entries) {
		if (!entry.isFile() || entry.name.endsWith('.ts')) {
			continue;
		}
		const srcPath = path.join(entry.parentPath, entry.name);
		const relativePath = path.relative(srcDir, srcPath);
		const destPath = path.join(outDir, relativePath);
		await fs.mkdir(path.dirname(destPath), { recursive: true });
		await fs.copyFile(srcPath, destPath);
	}
}

run({
	platform: 'node',
	entryPoints: {
		'main': path.join(srcDir, 'main.ts'),
		'askpass-main': path.join(srcDir, 'askpass-main.ts'),
		'git-editor-main': path.join(srcDir, 'git-editor-main.ts'),
	},
	srcDir,
	outdir: outDir,
	additionalOptions: {
		external: ['vscode', '@vscode/fs-copyfile'],
	},
}, process.argv, copyNonTsFiles);
```

**Variations / call-sites:**
- Also used: markdown-language-features (copy server worker JS from npm), json-language-features
- Enables WASM, binary assets, or static files alongside bundled JS

---

#### Pattern 6: Custom esbuild Plugins for Compilation
**Where:** `extensions/html-language-features/esbuild.browser.mts:17-83`
**What:** Advanced pattern showing esbuild plugin API for inline code generation. Custom plugin (`javaScriptLibsPlugin`) inlines TypeScript lib definitions at build time for browser platform.

```typescript
function javaScriptLibsPlugin(): esbuild.Plugin {
	return {
		name: 'javascript-libs',
		setup(build) {
			build.onLoad({ filter: /javascriptLibs\.ts$/ }, () => {
				const TYPESCRIPT_LIB_SOURCE = path.dirname(import.meta.resolve('typescript').replace('file://', ''));
				const JQUERY_DTS = path.join(extensionRoot, 'server', 'lib', 'jquery.d.ts');

				function getFileName(name: string): string {
					return name === '' ? 'lib.d.ts' : `lib.${name}.d.ts`;
				}

				function readLibFile(name: string): string {
					return fs.readFileSync(path.join(TYPESCRIPT_LIB_SOURCE, getFileName(name)), 'utf8');
				}

				const queue: string[] = [];
				const inQueue: Record<string, boolean> = {};

				function enqueue(name: string): void {
					if (inQueue[name]) {
						return;
					}
					inQueue[name] = true;
					queue.push(name);
				}

				enqueue('es2020.full');

				const result: { name: string; content: string }[] = [];
				while (queue.length > 0) {
					const name = queue.shift()!;
					const contents = readLibFile(name);
					const lines = contents.split(/\r\n|\r|\n/);

					const outputLines: string[] = [];
					for (const line of lines) {
						const m = line.match(/\/\/\/\s*<reference\s*lib="([^"]+)"/);
						if (m) {
							enqueue(m[1]);
						}
						outputLines.push(line);
					}

					result.push({
						name: getFileName(name),
						content: outputLines.join('\n'),
					});
				}

				const jquerySource = fs.readFileSync(JQUERY_DTS, 'utf8');
				result.push({
					name: 'jquery',
					content: jquerySource,
				});

				let code = `const libs = {};\n`;
				for (const entry of result) {
					code += `libs[${JSON.stringify(entry.name)}] = ${JSON.stringify(entry.content)};\n`;
				}
				code += `export function loadLibrary(name) { return libs[name] || ''; }\n`;

				return { contents: code, loader: 'js' };
			});
		},
	};
}

run({
	platform: 'browser',
	format: 'esm',
	entryPoints: {
		'htmlServerMain': path.join(extensionRoot, 'server', 'src', 'browser', 'htmlServerWorkerMain.ts'),
	},
	srcDir: path.join(extensionRoot, 'server', 'src'),
	outdir: path.join(extensionRoot, 'server', 'dist', 'browser'),
	additionalOptions: {
		tsconfig: path.join(extensionRoot, 'server', 'tsconfig.browser.json'),
		plugins: [javaScriptLibsPlugin()],
	},
}, process.argv),
```

**Variations / call-sites:**
- Only html-language-features uses custom plugins (requires special TypeScript lib bundling)
- Copilot/.esbuild.mts has testBundlePlugin, sanityTestBundlePlugin, importMetaPlugin (advanced testing infrastructure)
- Plugin system allows arbitrary code generation or asset transformation during bundling

---

#### Pattern 7: Watch Mode with Parcel Watcher (Low CPU)
**Where:** `extensions/esbuild-common.mts:54-80`
**What:** Custom watch implementation using `@parcel/watcher` instead of esbuild's native watch. Reduces CPU when idle via debouncing. Creates `esbuild.BuildContext` and calls `ctx.rebuild()`.

```typescript
async function watchWithParcel(ctx: esbuild.BuildContext, srcDir: string, didBuild?: () => Promise<unknown> | unknown): Promise<void> {
	let debounce: ReturnType<typeof setTimeout> | undefined;
	const rebuild = () => {
		if (debounce) {
			clearTimeout(debounce);
		}
		debounce = setTimeout(async () => {
			try {
				await ctx.cancel();
				const result = await ctx.rebuild();
				if (result.errors.length === 0) {
					await didBuild?.();
				}
			} catch (error) {
				console.error('[watch] build error:', error);
			}
		}, 100);
	};

	const watcher = await import('@parcel/watcher');
	await watcher.subscribe(srcDir, (_err, _events) => {
		rebuild();
	}, {
		ignore: ['**/node_modules/**', '**/dist/**', '**/out/**']
	});
	rebuild();
}
```

**Variations / call-sites:**
- Used by all 33 extensions via `runBuild()` when `--watch` flag is present
- Copilot also has parallel watch for multiple contexts (nodeExtHostContext, webExtHostContext, etc.)
- 100ms debounce prevents thrashing during rapid file saves

---

## Implications for Tauri/Rust Port

**JS Bundler Requirement:** Even in a Tauri/Rust port, extensions remain TypeScript/JavaScript and require bundling. This architecture could translate to:

1. **Bundler choice:** esbuild as external tool (via subprocess) or rewrite in Rust (e.g., swc, Turbopack)
2. **Configuration schema:** Rust struct mirroring `RunConfig` / `ExtensionRunConfig`
3. **Watch mode:** Parcel watcher is language-agnostic; could be wrapped in FFI or replaced with `notify` crate
4. **Plugin system:** esbuild plugins are JS functions; would need plugin API redesign or continued JS-based plugin runtime
5. **Build parallelization:** `Promise.all()` patterns map directly to Tokio `join!()` or `spawn()`

**Core assets:** The ~80 LOC `esbuild-common.mts` is the critical abstraction; its patterns (argument parsing, context creation, rebuild/build branching) are framework-agnostic and would be straightforward to port.

---

## File References

- **Core abstraction:** `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-common.mts` (80 LOC)
- **Extension wrapper:** `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-extension-common.mts` (51 LOC)
- **Simple examples:** configuration-editing, git, markdown-language-features, json-language-features
- **Complex example:** copilot/.esbuild.mts (480 LOC, multi-target with custom plugins)
- **Plugin example:** html-language-features/esbuild.browser.mts (114 LOC)
