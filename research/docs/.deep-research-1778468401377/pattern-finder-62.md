# Pattern Analysis: esbuild-extension-common.mts

## File Scope
- **File**: `extensions/esbuild-extension-common.mts`
- **Lines of Code**: 50
- **Purpose**: Common build script configuration for VS Code extensions supporting both Node.js and browser platforms

## Patterns Found

#### Pattern 1: Platform-Conditional Configuration Builder
**Where:** `extensions/esbuild-extension-common.mts:11-46`
**What:** Conditional build options generation based on deployment platform (Node.js vs Browser)

```typescript
interface ExtensionRunConfig extends RunConfig {
	readonly platform: 'node' | 'browser';
	readonly format?: 'cjs' | 'esm';
}

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
- Used by `extensions/git/esbuild.mts:26-38` with `platform: 'node'`
- Used by `extensions/json-language-features/esbuild.mts:12-22` with both `platform: 'node'` and `format: 'esm'`
- Browser builds configure module resolution order (`['browser', 'module', 'main']`) with polyfill aliases and environment variable defines

#### Pattern 2: Polymorphic Export with Extension-Specific Configuration
**Where:** `extensions/esbuild-extension-common.mts:48-50`
**What:** Async wrapper function that delegates to shared build runner with resolved platform-specific options

```typescript
export async function run(config: ExtensionRunConfig, args: string[], didBuild?: (outDir: string) => unknown): Promise<void> {
	return runBuild(config, resolveBaseOptions(config), args, didBuild);
}
```

**Variations / call-sites:**
- Called by 40+ extension build files importing this module
- Similar pattern used in `extensions/esbuild-webview-common.mts:23-29` but with hardcoded browser/ESM configuration instead of platform parameter

#### Pattern 3: Build Configuration Extension Pattern
**Where:** `extensions/esbuild-extension-common.mts:11-14`
**What:** Interface extending base RunConfig to add platform-specific properties while maintaining type safety

```typescript
interface ExtensionRunConfig extends RunConfig {
	readonly platform: 'node' | 'browser';
	readonly format?: 'cjs' | 'esm';
}
```

**Variations / call-sites:**
- Base `RunConfig` defined in `extensions/esbuild-common.mts:8-13` provides core build config (srcDir, outdir, entryPoints, additionalOptions)
- Extension build configs in individual packages (e.g., git, json-language-features) pass this interface to the shared runner
- The `format` property defaults to CommonJS but can be overridden for ESM modules like in json server builds

#### Pattern 4: Environment Polyfill Strategy
**Where:** `extensions/esbuild-extension-common.mts:33-42`
**What:** Browser-specific transforms that replace Node.js global variables with web-compatible substitutes

```typescript
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
```

**Variations / call-sites:**
- Applied uniformly across 40+ browser-targeted extension builds
- Works in conjunction with esbuild's alias and define features to enable Node.js code to run in browsers
- Similar polyfill pattern seen in multiple extension webview builds where additional Node modules are aliased to browser equivalents

#### Pattern 5: Shared Optimization Configuration
**Where:** `extensions/esbuild-extension-common.mts:17-29`
**What:** Consistent build optimization settings applied across all extensions regardless of platform

```typescript
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
```

**Variations / call-sites:**
- All extensions inherit these settings: bundling, minification, tree-shaking, sourcemaps, ES2024 target
- The `external: ['vscode']` prevents the VS Code API from being bundled (always provided by host)
- Additional externals can be added per-extension via the additionalOptions pattern (e.g., `extensions/git/esbuild.mts:36` adds `'@vscode/fs-copyfile'`)

## Key Architectural Insights

The esbuild-extension-common.mts file exemplifies a **layered build configuration pattern** where:

1. **Base layer** (`esbuild-common.mts`) provides the generic watch/build runner and RunConfig interface
2. **Platform-specific layer** (`esbuild-extension-common.mts`) adds platform discrimination and environment-aware defaults
3. **Webview specialization** (`esbuild-webview-common.mts`) hardcodes browser+ESM for isolated UI contexts
4. **Extension layer** (individual esbuild.mts files) provides entry points, source directories, and extension-specific externals

This architecture enables VS Code to maintain consistent build behavior across 40+ extensions while allowing per-extension customization. The pattern shows how TypeScript/JavaScript codebases abstract over platform differences using conditional configuration rather than separate code paths—a pattern that would translate differently in a Rust/Tauri port where compile-time conditional compilation (via feature flags or build scripts) would replace runtime platform checks.
