# Pattern Finder 56: VS Code Extension Build Configuration (esbuild)

## Research Question
What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust, focusing on extension build configuration patterns.

## Scope Analysis
File: `/extensions/esbuild-extension-common.mts` (102 LOC)

This file defines the unified build orchestration for all VS Code extensions. A Tauri/Rust port must understand how extensions are currently bundled and produce equivalent artifacts.

---

## Pattern Examples

#### Pattern: Platform-Aware Build Configuration Resolver
**Where:** `extensions/esbuild-extension-common.mts:24-57`
**What:** Resolves esbuild options with platform-specific (node/browser) field resolution, aliasing, and environment definitions.
```typescript
function resolveOptions(config: RunConfig, outdir: string): BuildOptions {
	const options: BuildOptions = {
		platform: config.platform,
		bundle: true,
		minify: true,
		treeShaking: true,
		sourcemap: true,
		target: ['es2024'],
		external: ['vscode'],
		format: config.format ?? 'cjs',
		entryPoints: config.entryPoints,
		outdir,
		logOverride: {
			'import-is-undefined': 'error',
		},
		...(config.additionalOptions || {}),
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
**Variations / call-sites:** Used in 51 extension build files; 35 use `platform: 'node'`, 16 use browser variants with `esbuild.browser.mts` files.

---

#### Pattern: Watch Mode with Post-Build Hooks
**Where:** `extensions/esbuild-extension-common.mts:70-93`
**What:** Enables incremental watch mode with optional `didBuild` callback plugin for post-build operations (file copying, refresh, etc.).
```typescript
const isWatch = args.indexOf('--watch') >= 0;
if (isWatch) {
	if (didBuild) {
		resolvedOptions.plugins = [
			...(resolvedOptions.plugins || []),
			{
				name: 'did-build', setup(pluginBuild) {
					pluginBuild.onEnd(async result => {
						if (result.errors.length > 0) {
							return;
						}

						try {
							await didBuild(outdir);
						} catch (error) {
							console.error('didBuild failed:', error);
						}
					});
				},
			}
		];
	}
	const ctx = await esbuild.context(resolvedOptions);
	await ctx.watch();
}
```
**Variations / call-sites:** Used in `extensions/git/esbuild.mts:38` with `copyNonTsFiles` callback; most other extensions omit callbacks and use default watch.

---

#### Pattern: Multiple Entry Points with Named Outputs
**Where:** `extensions/esbuild-extension-common.mts:20, 34`
**What:** Supports three entry point patterns: string array, record (name -> path), or explicit in/out objects for flexible multi-file bundles.
```typescript
readonly entryPoints: string[] | Record<string, string> | { in: string; out: string }[];
```
**Variations / call-sites:**
- Record pattern (most common): `extensions/git/esbuild.mts:28-32` with `'main'`, `'askpass-main'`, `'git-editor-main'`
- Simple object pattern: `extensions/typescript-language-features/esbuild.mts:13-15` with single `'extension'` entry
- Used in all 51 extension files

---

#### Pattern: Output Root Path Override
**Where:** `extensions/esbuild-extension-common.mts:61-66`
**What:** Accepts `--outputRoot` CLI argument to override output directory while preserving relative structure.
```typescript
const outputRootIndex = args.indexOf('--outputRoot');
if (outputRootIndex >= 0) {
	const outputRoot = args[outputRootIndex + 1];
	const outputDirName = path.basename(outdir);
	outdir = path.join(outputRoot, outputDirName);
}
```
**Variations / call-sites:** CLI argument parsing pattern used for build system integration; allows monorepo or CI/CD to stage output to custom directories.

---

#### Pattern: Unified Entry Point (run export)
**Where:** `extensions/esbuild-extension-common.mts:59`
**What:** Exports single `run()` function that all extensions import and invoke, encapsulating the full build lifecycle (resolve options, watch/build, error handling).
```typescript
export async function run(
	config: RunConfig,
	args: string[],
	didBuild?: (outDir: string) => unknown
): Promise<void>
```
**Variations / call-sites:** Invoked identically across all 51 extensions; core abstraction that decouples individual extension build configs from esbuild mechanics.

---

#### Pattern: Optional Additional Options Passthrough
**Where:** `extensions/esbuild-extension-common.mts:21, 39`
**What:** Allows extensions to inject custom esbuild options (tsconfig, external deps, aliases) without modifying the common function.
```typescript
readonly additionalOptions?: Partial<esbuild.BuildOptions>;
...
...(config.additionalOptions || {}),
```
**Variations / call-sites:**
- `extensions/simple-browser/esbuild.browser.mts:18-20` injects custom `tsconfig: path.join(..., 'tsconfig.browser.json')`
- `extensions/git/esbuild.mts:35-37` adds external deps: `'@vscode/fs-copyfile'`
- Most extensions omit this and use defaults

---

#### Pattern: External Dependency Exclusion
**Where:** `extensions/esbuild-extension-common.mts:32`
**What:** Marks `'vscode'` module as external by default (not bundled), with optional per-extension overrides for additional externals.
```typescript
external: ['vscode'],
```
**Variations / call-sites:**
- Default pattern: all 51 extensions exclude `'vscode'`
- Extended in `extensions/git/esbuild.mts:36` to include `'@vscode/fs-copyfile'`
- Indicates extension runtime assumes `vscode` API available; Rust port must provide equivalent module resolution

---

## Summary

The VS Code extension build system centralizes esbuild configuration in a 102-line common module that 51 extensions depend on. The key patterns for a Tauri/Rust port are: (1) **platform-specific resolution** that targets node vs. browser with different import paths and environment stubs; (2) **watch mode with post-build hooks** enabling incremental development and file copying; (3) **flexible entry points** (single or multi-file bundles named arbitrarily); (4) **CLI argument overrides** for output paths; (5) **external module handling** that preserves the `vscode` API boundary; and (6) **optional extensibility** for custom build options per extension. A Rust port would need to replicate these abstractions in a build tool (likely a custom build script or Cargo build script) that produces identical bundle outputs and supports the same CLI interface, ensuring all 51+ extensions continue to build without modification.

