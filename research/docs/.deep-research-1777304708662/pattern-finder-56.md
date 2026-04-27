# Pattern Finder 56: VS Code Extension Build Configuration

## Scope Analysis
`extensions/esbuild-extension-common.mts` (102 LOC) — This is the shared esbuild configuration helper that all VS Code extensions use for their build process. It demonstrates the extension build infrastructure patterns.

## Patterns Found

#### Pattern 1: Platform-Specific Build Configuration
**Where:** `extensions/esbuild-extension-common.mts:24-56`
**What:** Single configuration model supporting both Node (desktop) and browser (web) platform targets with different field resolution and polyfills.

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

**Variations / call-sites:** 
- Node platform: used by git, npm, typescript-language-features extensions
- Browser platform: used by markdown-language-features, simple-browser (browser variant), json-language-features (browser variant)

#### Pattern 2: Watch Mode with Post-Build Callback
**Where:** `extensions/esbuild-extension-common.mts:59-102`
**What:** Unified async build runner that supports both one-off builds and watch mode with optional post-build callback hooks via esbuild plugin system.

```typescript
export async function run(config: RunConfig, args: string[], didBuild?: (outDir: string) => unknown): Promise<void> {
	let outdir = config.outdir;
	const outputRootIndex = args.indexOf('--outputRoot');
	if (outputRootIndex >= 0) {
		const outputRoot = args[outputRootIndex + 1];
		const outputDirName = path.basename(outdir);
		outdir = path.join(outputRoot, outputDirName);
	}

	const resolvedOptions = resolveOptions(config, outdir);

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
- git extension: Post-build copies non-TS files (`extensions/git/esbuild.mts:38`)
- markdown-language-features: Post-build copies language server worker file (`extensions/markdown-language-features/esbuild.mts:28`)

#### Pattern 3: Multiple Entry Point Configuration
**Where:** `extensions/esbuild-extension-common.mts:20`
**What:** Entry point configuration supporting flexible formats: object map (preferred), array, or explicit in/out format.

```typescript
readonly entryPoints: string[] | Record<string, string> | { in: string; out: string }[];
```

**Variations / call-sites:**
- Single entry point (most common): `extensions/simple-browser/esbuild.mts:13-15`
  ```typescript
  entryPoints: {
    'extension': path.join(srcDir, 'extension.ts'),
  },
  ```

- Multiple named entry points: `extensions/git/esbuild.mts:28-32`
  ```typescript
  entryPoints: {
    'main': path.join(srcDir, 'main.ts'),
    'askpass-main': path.join(srcDir, 'askpass-main.ts'),
    'git-editor-main': path.join(srcDir, 'git-editor-main.ts'),
  },
  ```

- Multiple numbered entry points: `extensions/ipynb/esbuild.mts:13-16`
  ```typescript
  entryPoints: {
    'ipynbMain.node': path.join(srcDir, 'ipynbMain.node.ts'),
    'notebookSerializerWorker': path.join(srcDir, 'notebookSerializerWorker.ts'),
  },
  ```

#### Pattern 4: Extensible Additional Build Options
**Where:** `extensions/esbuild-extension-common.mts:21`
**What:** Configuration supports arbitrary esbuild options via `additionalOptions` to override or extend defaults without modifying the common helper.

```typescript
readonly additionalOptions?: Partial<esbuild.BuildOptions>;
```

**Variations / call-sites:**
- Custom tsconfig for browser: `extensions/simple-browser/esbuild.browser.mts:18-20`
  ```typescript
  additionalOptions: {
    tsconfig: path.join(import.meta.dirname, 'tsconfig.browser.json'),
  },
  ```

- Custom external modules: `extensions/git/esbuild.mts:35-37`
  ```typescript
  additionalOptions: {
    external: ['vscode', '@vscode/fs-copyfile'],
  },
  ```

#### Pattern 5: Dual Build Targets (Node + Browser)
**Where:** `extensions/simple-browser/esbuild.mts` (11 LOC) and `extensions/simple-browser/esbuild.browser.mts` (15 LOC)
**What:** Common pattern where extensions provide separate build scripts for node and browser platforms, sharing the same common helper but different configurations.

Node build:
```typescript
run({
	platform: 'node',
	entryPoints: {
		'extension': path.join(srcDir, 'extension.ts'),
	},
	srcDir,
	outdir: outDir,
}, process.argv);
```

Browser build:
```typescript
run({
	platform: 'browser',
	entryPoints: {
		'extension': path.join(srcDir, 'extension.ts'),
	},
	srcDir,
	outdir: outDir,
	additionalOptions: {
		tsconfig: path.join(import.meta.dirname, 'tsconfig.browser.json'),
	},
}, process.argv);
```

**Variations / call-sites:** 51 files across the extensions directory use this pattern, with ~50 having distinct esbuild.mts and esbuild.browser.mts pairs.

#### Pattern 6: Runtime Argument Parsing
**Where:** `extensions/esbuild-extension-common.mts:61-65`
**What:** CLI argument parsing for output directory override and watch mode detection, allowing build script reuse across different invocation contexts.

```typescript
let outdir = config.outdir;
const outputRootIndex = args.indexOf('--outputRoot');
if (outputRootIndex >= 0) {
	const outputRoot = args[outputRootIndex + 1];
	const outputDirName = path.basename(outdir);
	outdir = path.join(outputRoot, outputDirName);
}

const isWatch = args.indexOf('--watch') >= 0;
```

**Variations / call-sites:** All 51+ consuming extensions pass `process.argv` to support these runtime flags.

#### Pattern 7: Shared External Module List
**Where:** `extensions/esbuild-extension-common.mts:32`
**What:** VS Code API is declared as external to all extensions, preventing bundling of the vscode module and ensuring consistent API access.

```typescript
external: ['vscode'],
```

**Variations / call-sites:** Base configuration; git and some other extensions extend this with `additionalOptions.external`.

## Summary

The esbuild-extension-common.mts file provides a build abstraction layer for 50+ VS Code extensions. Key patterns:

1. **Platform abstraction** - Single config model handling Node vs. browser builds with appropriate polyfills
2. **Watch mode integration** - Plugin-based post-build hooks for asset copying and artifact generation
3. **Entry point flexibility** - Support for single or multiple named entry points
4. **Extensibility** - Additional options override mechanism without modifying the base helper
5. **Dual builds** - Most extensions ship both Node (esbuild.mts) and browser (esbuild.browser.mts) variants
6. **CLI flexibility** - Runtime argument parsing for output root and watch mode
7. **API externalization** - VS Code API kept external across all extensions for dynamic linking

This is fundamentally a **build infrastructure** abstraction showing how extensions are compiled from TypeScript to JavaScript for two different runtime environments. It does not demonstrate core IDE functionality patterns (editing, debugging, language services, etc.), which exist in the broader src/ directory structure.
