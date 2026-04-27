# Build System Patterns for VS Code Bundling

## Entry Point Configuration Pattern

#### Pattern: Multi-Target Entry Point Definition
**Where:** `build/buildfile.ts:1-86`
**What:** Modular entry point definitions organized by build target (desktop, server, web).
```typescript
function createModuleDescription(name: string): IEntryPoint {
	return {
		name
	};
}

export const workerEditor = createModuleDescription('vs/editor/common/services/editorWebWorkerMain');
export const workerExtensionHost = createModuleDescription('vs/workbench/api/worker/extensionHostWorkerMain');
export const workbenchDesktop = [
	createModuleDescription('vs/workbench/contrib/debug/node/telemetryApp'),
	createModuleDescription('vs/platform/files/node/watcher/watcherMain'),
	createModuleDescription('vs/platform/terminal/node/ptyHostMain'),
	createModuleDescription('vs/code/electron-browser/workbench/workbench'),
	createModuleDescription('vs/sessions/electron-browser/sessions')
];

export const code = [
	createModuleDescription('vs/code/node/cliProcessMain'),
	createModuleDescription('vs/code/electron-utility/sharedProcess/sharedProcessMain'),
	createModuleDescription('vs/code/electron-browser/workbench/workbench'),
];
```

**Variations:**
- Workers: Shared across all targets (editorWebWorkerMain, extensionHostWorkerMain)
- Desktop workers: Platform-specific (profileAnalysisWorkerMain with electron-browser)
- Bootstrap entry points: Minimal per-target (main, cli, bootstrap-fork for desktop/server)
- Web entry points: No bootstrap files, server provides HTML hosting

---

## esbuild.build Configuration Pattern

#### Pattern: Platform-Aware Bundle Configuration
**Where:** `build/next/index.ts:847-878`
**What:** Target-specific esbuild bundling with conditional CSS handling and plugins.
```typescript
const buildOptions: esbuild.BuildOptions = {
	entryPoints: needsCssBundling
		? [{ in: entryPath, out: entryPoint }]
		: [entryPath],
	...(needsCssBundling
		? { outdir: path.join(REPO_ROOT, outDir) }
		: { outfile: outPath }),
	bundle: true,
	format: 'esm',
	platform: 'neutral',
	target: ['es2024'],
	packages: 'external',
	sourcemap: 'linked',
	sourcesContent: true,
	minify: doMinify,
	treeShaking: true,
	banner,
	loader: {
		'.ttf': 'file',
		'.svg': 'file',
		'.png': 'file',
		'.sh': 'file',
	},
	assetNames: 'media/[name]',
	plugins,
	write: false,
	logLevel: 'warning',
};
```

**Variations:**
- Desktop main bundles: `platform: 'neutral'` with CSS bundling
- Bootstrap/server bundles: `platform: 'node'` with ESM format
- Standalone preload scripts: `format: 'cjs'` with `bundle: false`
- Minification: Conditional flag with source map adjustment
- Asset handling: Absolute paths like `media/[name]` for relative imports

---

## Plugin Pipeline Pattern

#### Pattern: Modular Plugin Stack with Conditional Composition
**Where:** `build/next/index.ts:833-841`
**What:** Dynamic plugin selection based on entry point type and build options.
```typescript
const plugins: esbuild.Plugin[] = bundleCssEntryPoints.has(entryPoint) ? [] : [cssExternalPlugin()];
plugins.push(contentMapperPlugin);
if (doNls) {
	plugins.unshift(nlsPlugin({
		baseDir: path.join(REPO_ROOT, SRC_DIR),
		collector: nlsCollector,
	}));
}
```

**Variations:**
- CSS external plugin: Marks `.css` imports as external for separate CSS handling
- Content mapper plugin: Injects product configuration and builtin extensions list
- NLS plugin: Collects localizable strings (prepended for early processing)
- Inline minimist: Bootstrap-only plugin to inline the CLI argument parser

---

## External Package Resolution Pattern

#### Pattern: Selective External Override
**Where:** `build/lib/optimize.ts:129-138`
**What:** Hooks into esbuild resolution to inline specific dependencies.
```typescript
const externalOverride: esbuild.Plugin = {
	name: 'external-override',
	setup(build) {
		// Inline selected modules that are we depend on on startup without
		// a conditional `await import(...)` by hooking into the resolution.
		build.onResolve({ filter: /^minimist$/ }, () => {
			return { path: path.join(REPO_ROOT_PATH, 'node_modules', 'minimist', 'index.js'), external: false };
		});
	},
};
```

**Variations:**
- `packages: 'external'` global setting: Default behavior marks all node_modules as external
- `external: false` override: Forces inlining for critical startup dependencies
- Minimist inlining: Required for CLI parsing before modules load
- Relative file paths: Used when absolute external mark is inappropriate

---

## Resource and Asset Configuration Pattern

#### Pattern: Target-Specific Resource Inclusion with Glob Patterns
**Where:** `build/next/index.ts:240-398`
**What:** Curated resource patterns per build target (desktop, server, server-web, web).
```typescript
const desktopResourcePatterns = [
	...commonResourcePatterns,
	'vs/code/electron-browser/workbench/workbench.html',
	'vs/code/electron-browser/workbench/workbench-dev.html',
	'vs/workbench/contrib/webview/browser/pre/*.js',
	'vs/workbench/contrib/webview/browser/pre/*.html',
	'vs/base/node/*.sh',
	'vs/workbench/contrib/terminal/common/scripts/*.sh',
	'vs/workbench/contrib/terminal/common/scripts/*.ps1',
	'vs/platform/accessibilitySignal/browser/media/*.mp3',
	'vs/workbench/contrib/welcomeGettingStarted/common/media/**/*.svg',
	'vs/workbench/contrib/welcomeGettingStarted/common/media/**/*.png',
];

const serverResourcePatterns = [
	'vs/base/node/cpuUsage.sh',
	'vs/base/node/ps.sh',
	'vs/workbench/contrib/terminal/common/scripts/shellIntegration*.sh',
];

function getResourcePatternsForTarget(target: BuildTarget): string[] {
	switch (target) {
		case 'desktop':
			return desktopResourcePatterns;
		case 'server':
			return serverResourcePatterns;
		// ...
	}
}
```

**Variations:**
- Desktop: Includes HTML, shell scripts, CSS media
- Server: Minimal (process monitoring scripts, shell integration)
- Server-web: Server resources + web UI HTML and assets
- Web: Browser-only resources (HTML, JS, CSS, SVG)

---

## Bootstrap File Compilation Pattern

#### Pattern: Standalone TypeScript Compilation for Preload Scripts
**Where:** `build/next/index.ts:497-529`
**What:** Non-bundled CommonJS compilation for Electron preload contexts.
```typescript
const desktopStandaloneFiles = [
	'vs/base/parts/sandbox/electron-browser/preload.ts',
	'vs/base/parts/sandbox/electron-browser/preload-aux.ts',
	'vs/platform/browserView/electron-browser/preload-browserView.ts',
];

async function compileStandaloneFiles(outDir: string, doMinify: boolean, target: BuildTarget): Promise<void> {
	if (target !== 'desktop') {
		return;
	}
	await Promise.all(desktopStandaloneFiles.map(async (file) => {
		await esbuild.build({
			entryPoints: [entryPath],
			outfile: outPath,
			bundle: false,
			format: 'cjs',
			platform: 'node',
			target: ['es2024'],
			sourcemap: 'linked',
			minify: doMinify,
		});
	}));
}
```

**Variations:**
- Desktop-only: Electron preload scripts need special handling
- CJS format: Required for require() compatibility in Electron context
- No bundling: Scripts operate in isolated sandbox context
- Conditional minification: Matches parent bundle minify flag
- Source map linking: Maps to original TS during development

---

## Gulp Task Orchestration Pattern

#### Pattern: Task Definition and Composition
**Where:** `build/gulpfile.ts:18-48`
**What:** Gulp tasks for build orchestration with series/parallel composition.
```typescript
gulp.task(compilation.compileExtensionPointNamesTask);
gulp.task(compilation.compileApiProposalNamesTask);

const transpileClientSWCTask = task.define('transpile-client-esbuild', 
	task.series(util.rimraf('out'), compilation.transpileTask('src', 'out', true))
);
gulp.task(transpileClientSWCTask);

const compileClientTask = task.define('compile-client', 
	task.series(
		util.rimraf('out'), 
		compilation.copyCodiconsTask, 
		compilation.compileApiProposalNamesTask, 
		compilation.compileExtensionPointNamesTask, 
		compilation.compileTask('src', 'out', false)
	)
);
gulp.task(compileClientTask);

const _compileTask = task.define('compile', 
	task.parallel(monacoTypecheckTask, compileClientTask, compileExtensionsTask)
);
gulp.task(_compileTask);
```

**Variations:**
- Series: Sequential tasks (cleanup, then compile, then optimize)
- Parallel: Independent tasks (type-check, extensions, client compile)
- File-level operations: rimraf for cleanup, codicon copying
- Task chaining: Composition into meta-tasks (compile, watch, bundle)

---

## Multi-Target Build Configuration Pattern

#### Pattern: Target Matrix with Conditional Entry Points
**Where:** `build/next/index.ts:158-188`
**What:** Function-based target selection for entry points with shared and platform-specific bundles.
```typescript
function getEntryPointsForTarget(target: BuildTarget): string[] {
	switch (target) {
		case 'desktop':
			return [
				...workerEntryPoints,
				...desktopWorkerEntryPoints,
				...desktopEntryPoints,
				...codeEntryPoints,
			];
		case 'server':
			return [
				...serverEntryPoints,
			];
		case 'server-web':
			return [
				...serverEntryPoints,
				...workerEntryPoints,
				...webEntryPoints,
				...keyboardMapEntryPoints,
			];
		case 'web':
			return [
				...workerEntryPoints,
				...webOnlyEntryPoints,
				'vs/workbench/workbench.web.main.internal',
				...keyboardMapEntryPoints,
			];
	}
}
```

**Variations:**
- Desktop: Workers + Electron + code/CLI (complete app)
- Server: Node-only entry points (headless backend)
- Server-web: Server + web workers + web workbench (hybrid)
- Web: Workers + web-only entries + keyboard maps (browser-only)
- Bootstrap points: Desktop/server have CLI bootstrap; web has none

---

## Post-Process Pipeline Pattern

#### Pattern: Multi-Stage Transformation After Bundling
**Where:** `build/next/index.ts:940-1032`
**What:** Chain of transformations on bundled output files (mangle, NLS, sourcemap rewrite).
```typescript
for (const file of result.outputFiles) {
	if (file.path.endsWith('.js') || file.path.endsWith('.css')) {
		let content = file.text;

		// Stage 1: Convert #private fields
		if (file.path.endsWith('.js') && doManglePrivates && !isExtensionHostBundle(file.path)) {
			const mangleResult = convertPrivateFields(content, file.path);
			content = mangleResult.code;
			mangleEdits.set(file.path, { preMangleCode, edits: mangleResult.edits });
		}

		// Stage 2: NLS post-processing
		if (file.path.endsWith('.js') && doNls && indexMap.size > 0) {
			const nlsResult = postProcessNLS(content, indexMap, preserveEnglish);
			content = nlsResult.code;
			nlsEdits.set(file.path, { preNLSCode, edits: nlsResult.edits });
		}

		// Stage 3: Sourcemap URL rewrite
		if (sourceMapBaseUrl) {
			content = content.replace(
				/\/\/# sourceMappingURL=.+$/m,
				`//# sourceMappingURL=${sourceMapBaseUrl}/${relativePath}.map`
			);
		}

		await fs.promises.writeFile(file.path, content);
	}
}
```

**Variations:**
- Mangle stage: Conditional (skips extension host for API surface)
- NLS stage: Applies localization string replacement with index mapping
- Sourcemap URL stage: Rewrites to CDN path for production builds
- Edit tracking: Collects offsets for subsequent sourcemap adjustment

---

## Key Patterns for Tauri/Rust Port

The esbuild configuration reveals these critical aspects a Rust/Tauri build system must replicate:

1. **Entry point multiplexing**: Four distinct target platforms (desktop, server, server-web, web) with shared worker bundles
2. **Conditional asset inclusion**: Target-specific resource patterns (shell scripts, HTML, media)
3. **Platform-aware settings**: `platform: 'neutral' | 'node'` and `format: 'esm' | 'cjs'` vary by bundle type
4. **External package strategy**: Global `packages: 'external'` with per-package overrides (e.g., minimist inlined)
5. **Plugin pipeline composition**: Dynamic plugin stack based on entry point and build flags (NLS, minify, mangling)
6. **Post-processing chain**: Three-stage transformation (mangle → NLS → sourcemap rewrite) with edit tracking
7. **Bootstrap isolation**: Electron preload scripts compiled separately as CJS with `bundle: false`
8. **Standalone resources**: Glob-based file copying with target-specific patterns per `getResourcePatternsForTarget()`
