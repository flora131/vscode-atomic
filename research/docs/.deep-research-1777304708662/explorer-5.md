# Partition 5 of 79 — Findings

## Scope
`build/` (195 files, 33,416 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Build System File Locations (Partition 5)

## Implementation

### Gulp Task Definitions and Entry Points
- `build/gulpfile.ts` - Main gulp orchestrator, defines core build tasks
- `build/gulpfile.cli.ts` - CLI compilation task definitions
- `build/gulpfile.compile.ts` - Compilation pipeline tasks
- `build/gulpfile.editor.ts` - Editor-specific build configurations
- `build/gulpfile.scan.ts` - Code scanning tasks
- `build/gulpfile.hygiene.ts` - Code hygiene and linting tasks
- `build/gulpfile.vscode.ts` - Main VS Code platform build
- `build/gulpfile.vscode.web.ts` - Web version build tasks
- `build/gulpfile.vscode.linux.ts` - Linux-specific bundling
- `build/gulpfile.vscode.win32.ts` - Windows-specific bundling
- `build/gulpfile.reh.ts` - Remote execution host build
- `build/gulpfile.extensions.ts` - Extension bundling tasks
- `build/buildfile.ts` - Build file utilities and helpers

### Build Configuration and Core Libraries
- `build/buildConfig.ts` - Main build configuration schema
- `build/lib/bundle.ts` - Bundling logic for AMD module format
- `build/lib/optimize.ts` - Code optimization pass (treeshaking, mangling)
- `build/lib/compilation.ts` - TypeScript/ESM->AMD compilation orchestration
- `build/lib/treeshaking.ts` - Dead code elimination
- `build/lib/mangle/index.ts` - Name mangling for minification
- `build/lib/mangle/renameWorker.ts` - Worker thread for symbol renaming
- `build/lib/mangle/staticLanguageServiceHost.ts` - TypeScript LSP for symbol analysis

### Module and Import Processing
- `build/lib/inlineMeta.ts` - Inline metadata processing
- `build/lib/monaco-api.ts` - Monaco editor API definitions
- `build/lib/extensions.ts` - Extension manifest processing
- `build/lib/builtInExtensions.ts` - Built-in extension resolution
- `build/lib/builtInExtensionsCG.ts` - Component graph for built-in extensions
- `build/lib/extractExtensionPoints.ts` - Extension point extraction from source

### Localization and i18n
- `build/lib/nls.ts` - Native language support integration
- `build/lib/nls-analysis.ts` - NLS key analysis
- `build/lib/i18n.ts` - Internationalization handling
- `build/lib/i18n.resources.json` - i18n resource manifest

### TypeScript and Transpilation
- `build/lib/tsb/builder.ts` - TypeScript batch builder
- `build/lib/tsb/index.ts` - TSB module entry
- `build/lib/tsb/transpiler.ts` - Custom transpiler for module format conversion
- `build/lib/tsb/utils.ts` - TSB utilities
- `build/lib/typeScriptLanguageServiceHost.ts` - LSP host for type checking
- `build/lib/tsconfigUtils.ts` - tsconfig.json manipulation
- `build/lib/propertyInitOrderChecker.ts` - Property initialization validation

### Platform-Specific Signing and Packaging
- `build/darwin/sign.ts` - macOS code signing
- `build/darwin/sign-server.ts` - macOS signing server integration
- `build/darwin/create-dmg.ts` - DMG disk image creation
- `build/darwin/create-universal-app.ts` - Universal (ARM64/x86) macOS app
- `build/darwin/verify-macho.ts` - Mach-O binary verification
- `build/win32/explorer-dll-fetcher.ts` - Windows explorer DLL dependency fetcher
- `build/linux/libcxx-fetcher.ts` - Linux C++ stdlib fetch
- `build/linux/dependencies-generator.ts` - Debian/RPM dependency generation
- `build/linux/debian/calculate-deps.ts` - Debian dep calculation
- `build/linux/debian/dep-lists.ts` - Debian dependency lists
- `build/linux/debian/install-sysroot.ts` - Sysroot installation
- `build/linux/rpm/calculate-deps.ts` - RPM dependency calculation
- `build/linux/rpm/dep-lists.ts` - RPM dependency lists

### Electron and Native Integration
- `build/lib/electron.ts` - Electron integration (bundling, native modules)
- `build/lib/node.ts` - Node.js runtime integration
- `build/lib/dependencies.ts` - Dependency tracking and resolution

### Watch and Development
- `build/lib/watch/index.ts` - File watching for incremental builds
- `build/lib/watch/watch-win32.ts` - Windows-specific file watching
- `build/lib/task.ts` - Task definition and execution framework
- `build/lib/reporter.ts` - Build progress reporting
- `build/lib/util.ts` - Build utilities

### Policy and Governance Code Generation
- `build/lib/policies/basePolicy.ts` - Base policy class
- `build/lib/policies/booleanPolicy.ts` - Boolean configuration policies
- `build/lib/policies/numberPolicy.ts` - Numeric configuration policies
- `build/lib/policies/stringPolicy.ts` - String configuration policies
- `build/lib/policies/stringEnumPolicy.ts` - Enum configuration policies
- `build/lib/policies/objectPolicy.ts` - Object/complex policies
- `build/lib/policies/policyGenerator.ts` - Policy code generation
- `build/lib/policies/render.ts` - Policy rendering to manifests
- `build/lib/policies/types.ts` - Policy type definitions
- `build/lib/policies/copyPolicyDto.ts` - Policy DTO utilities
- `build/lib/policies/exportPolicyData.ts` - Policy data export
- `build/lib/policies/policyData.jsonc` - Policy definitions

### Code Quality and Validation
- `build/lib/checkCyclicDependencies.ts` - Cyclic import detection
- `build/lib/formatter.ts` - Code formatting utilities
- `build/lib/stats.ts` - Build statistics collection
- `build/lib/date.ts` - Build timestamp utilities
- `build/lib/reporter.ts` - Build error/warning reporting
- `build/lib/preLaunch.ts` - Pre-launch validation
- `build/lib/standalone.ts` - Standalone build mode
- `build/lib/fetch.ts` - HTTP fetch wrapper for remote resources
- `build/lib/git.ts` - Git integration for versioning
- `build/lib/getVersion.ts` - Version extraction
- `build/eslint.ts` - ESLint integration wrapper
- `build/gulp-eslint.ts` - Gulp plugin for ESLint
- `build/stylelint.ts` - StyleLint integration
- `build/lib/stylelint/validateVariableNames.ts` - CSS variable validation
- `build/lib/stylelint/vscode-known-variables.json` - Known CSS variables

### CI/CD and Publishing
- `build/azure-pipelines/common/createBuild.ts` - Build artifact creation
- `build/azure-pipelines/common/publish.ts` - Artifact publishing
- `build/azure-pipelines/common/codesign.ts` - Multi-platform code signing
- `build/azure-pipelines/common/sign.ts` - Generic signing orchestration
- `build/azure-pipelines/common/sign-win32.ts` - Windows-specific signing
- `build/azure-pipelines/darwin/codesign.ts` - macOS signing
- `build/azure-pipelines/linux/codesign.ts` - Linux signing
- `build/azure-pipelines/win32/codesign.ts` - Windows signing
- `build/azure-pipelines/common/computeNodeModulesCacheKey.ts` - npm cache key generation
- `build/azure-pipelines/common/computeBuiltInDepsCacheKey.ts` - Built-in deps cache
- `build/azure-pipelines/common/checkForArtifact.ts` - Artifact existence check
- `build/azure-pipelines/common/checkDistroCommit.ts` - Distro version validation
- `build/azure-pipelines/common/getPublishAuthTokens.ts` - Auth token retrieval
- `build/azure-pipelines/common/listNodeModules.ts` - npm dependency listing
- `build/azure-pipelines/common/extract-telemetry.ts` - Telemetry extraction
- `build/azure-pipelines/common/releaseBuild.ts` - Release build coordination
- `build/azure-pipelines/common/waitForArtifacts.ts` - Artifact polling
- `build/azure-pipelines/common/downloadCopilotVsix.ts` - Copilot extension fetching
- `build/azure-pipelines/common/retry.ts` - Retry logic for CI tasks
- `build/azure-pipelines/upload-cdn.ts` - CDN upload coordination
- `build/azure-pipelines/upload-sourcemaps.ts` - Sourcemap publishing
- `build/azure-pipelines/upload-nlsmetadata.ts` - i18n metadata publishing
- `build/azure-pipelines/publish-types/check-version.ts` - Type definition versioning
- `build/azure-pipelines/publish-types/update-types.ts` - Type definition updates
- `build/azure-pipelines/update-dependencies-check.ts` - Dependency update validation
- `build/azure-pipelines/distro/mixin-npm.ts` - Distro npm integration
- `build/azure-pipelines/distro/mixin-quality.ts` - Quality assurance integration
- `build/azure-pipelines/common/telemetry-config.json` - Telemetry configuration
- `build/azure-pipelines/common/installPlaywright.ts` - Playwright installation
- `build/azure-pipelines/github-check-run.ts` - GitHub check run integration

### Vite / Next-Gen Bundler Support (Experimental)
- `build/vite/vite.config.ts` - Vite bundler configuration
- `build/vite/index.ts` - Vite entry point
- `build/vite/setup-dev.ts` - Development server setup
- `build/vite/index-workbench.ts` - Workbench Vite configuration
- `build/vite/workbench-electron.ts` - Electron workbench bundle
- `build/vite/package.json` - Vite dependencies
- `build/vite/tsconfig.json` - Vite TypeScript config
- `build/next/index.ts` - Next-gen bundler entry
- `build/next/nls-plugin.ts` - NLS plugin for bundler
- `build/next/private-to-property.ts` - Private field to property transformer
- `build/rspack/rspack.serve-out.config.mts` - Rspack bundler configuration (alternative)
- `build/rspack/workbench-rspack.html` - Rspack workbench template

### npm and Package Management
- `build/npm/postinstall.ts` - npm postinstall hook
- `build/npm/preinstall.ts` - npm preinstall hook
- `build/npm/fast-install.ts` - Optimized npm install
- `build/npm/installStateHash.ts` - Install state hashing
- `build/npm/update-distro.ts` - Distro update script
- `build/npm/update-all-grammars.ts` - Grammar dependency update
- `build/npm/update-localization-extension.ts` - Localization extension update
- `build/npm/mixin-telemetry-docs.ts` - Telemetry documentation generation
- `build/npm/dirs.ts` - Directory configuration

### Built-in Bundled Resources
- `build/builtin/main.js` - Main process entry
- `build/builtin/browser-main.js` - Browser main entry
- `build/builtin/package.json` - Built-in package manifest
- `build/builtin/index.html` - Built-in HTML template

### General Build Utilities
- `build/setup-npm-registry.ts` - npm registry configuration
- `build/filters.ts` - Gulp stream filters
- `build/hygiene.ts` - Code hygiene utilities
- `build/copilot-migrate-pr.ts` - Copilot PR migration tool

## Tests

### Unit Tests for Build Infrastructure
- `build/lib/test/booleanPolicy.test.ts` - Boolean policy code generation tests
- `build/lib/test/numberPolicy.test.ts` - Number policy tests
- `build/lib/test/stringPolicy.test.ts` - String policy tests
- `build/lib/test/stringEnumPolicy.test.ts` - String enum policy tests
- `build/lib/test/objectPolicy.test.ts` - Object policy tests
- `build/lib/test/policyConversion.test.ts` - Policy conversion tests
- `build/lib/test/render.test.ts` - Policy rendering tests
- `build/lib/test/i18n.test.ts` - i18n integration tests
- `build/lib/test/checkCyclicDependencies.test.ts` - Cyclic dependency detection tests

### Next-Gen Bundler Tests
- `build/next/test/nls-sourcemap.test.ts` - NLS sourcemap generation tests
- `build/next/test/private-to-property.test.ts` - Property transformation tests

### Test Fixtures and Policies
- `build/lib/test/fixtures/policies/linux/policy.json` - Linux policy fixture
- `build/lib/test/fixtures/policies/darwin/com.visualstudio.code.oss.mobileconfig` - macOS policy fixture
- `build/lib/test/fixtures/policies/darwin/en-us/com.visualstudio.code.oss.plist` - macOS English policy
- `build/lib/test/fixtures/policies/darwin/fr-fr/com.visualstudio.code.oss.plist` - macOS French policy
- `build/lib/test/fixtures/policies/win32/CodeOSS.admx` - Windows policy template
- `build/lib/test/fixtures/policies/win32/en-us/CodeOSS.adml` - Windows English policy
- `build/lib/test/fixtures/policies/win32/fr-fr/CodeOSS.adml` - Windows French policy

## Types / Interfaces

### Gulp Plugin Type Definitions
- `build/lib/typings/@vscode/gulp-electron.d.ts` - Electron gulp plugin types
- `build/lib/typings/vscode-gulp-watch.d.ts` - File watch plugin types

### Archive and Packaging Types
- `build/lib/typings/asar.d.ts` - ASAR archive format types
- `build/lib/typings/chromium-pickle-js.d.ts` - Chrome pickle format types
- `build/lib/typings/rcedit.d.ts` - Windows resource editor types

### Stream Processing Types
- `build/lib/typings/event-stream.d.ts` - Node stream event types
- `build/lib/typings/gulp-azure-storage.d.ts` - Azure storage gulp plugin
- `build/lib/typings/gulp-bom.d.ts` - BOM insertion gulp plugin
- `build/lib/typings/gulp-buffer.d.ts` - Buffer gulp plugin
- `build/lib/typings/gulp-gunzip.d.ts` - Gzip decompression gulp plugin
- `build/lib/typings/gulp-vinyl-zip.d.ts` - ZIP handling gulp plugin
- `build/lib/typings/stream.d.ts` - General stream utilities
- `build/lib/typings/ternary-stream.d.ts` - Ternary branching for streams

## Configuration

### Build System Configuration
- `build/package.json` - Build dependencies and scripts
- `build/package-lock.json` - npm lock file
- `build/tsconfig.json` - TypeScript configuration for build system
- `build/buildConfig.ts` - Build configuration schema

### TypeScript Type Checking Configuration
- `build/checker/tsconfig.browser.json` - Browser bundle type checking
- `build/checker/tsconfig.electron-browser.json` - Electron renderer process
- `build/checker/tsconfig.electron-main.json` - Electron main process
- `build/checker/tsconfig.electron-utility.json` - Electron utility processes
- `build/checker/tsconfig.node.json` - Node.js utilities
- `build/checker/tsconfig.worker.json` - Web worker type checking
- `build/checker/layersChecker.ts` - Type checker for layer boundaries

### Vite Configuration
- `build/vite/package.json` - Vite dependencies
- `build/vite/tsconfig.json` - Vite TypeScript configuration

### Rspack Configuration (Alternative Bundler)
- `build/rspack/package.json` - Rspack dependencies
- `build/rspack/package-lock.json` - Rspack npm lock
- `build/rspack/rspack.serve-out.config.mts` - Rspack dev server

### npm/Gyp Configuration
- `build/npm/gyp/package.json` - Node-gyp build dependencies

### Azure Pipelines Configuration
- `build/azure-pipelines/config/tsaoptions.json` - TSA (Third-party Scan) options
- `build/azure-pipelines/config/CredScanSuppressions.json` - Credential scan suppressions
- `build/azure-pipelines/common/telemetry-config.json` - Telemetry configuration

### Manifest and Metadata
- `build/lib/typings/cgmanifest.json` - Component governance manifest
- `build/builtin/package.json` - Built-in extensions manifest
- `build/monaco/package.json` - Monaco editor dependencies

## Examples / Fixtures

### Windows Installer Configuration
- `build/win32/code.iss` - Inno Setup installer script

### Windows Specific Resources
- `build/win32/vcruntime140.dll` - Visual C++ Runtime
- `build/win32/inno_updater.exe` - Windows installer updater
- `build/win32/i18n/` - Installer UI translations (22 language files)
- `build/win32/Cargo.toml` - Rust build manifest (Windows-specific)
- `build/win32/Cargo.lock` - Rust lock file

### macOS DMG Templates
- `build/darwin/dmg-background-stable.tiff` - DMG background for stable releases
- `build/darwin/dmg-background-insider.tiff` - DMG background for insider builds
- `build/darwin/dmg-background-exploration.tiff` - DMG background for exploration builds
- `build/darwin/dmg-settings.py.template` - DMG creation script template
- `build/darwin/patch-dmg.py` - DMG patching utility
- `build/darwin/distribution.provisionprofile` - macOS provisioning profile

### Linux Sysroot and Dependencies
- `build/linux/debian/` - Debian package configuration
- `build/linux/rpm/` - Red Hat package configuration

### Bundler Templates
- `build/rspack/workbench-rspack.html` - Rspack workbench HTML template

## Documentation

### Build Process Documentation
- `build/next/working.md` - Next-gen bundler development notes
- `build/monaco/README.md` - Monaco editor integration guide
- `build/monaco/README-npm.md` - Monaco npm publication guide

## Notable Clusters

### Gulp Task Definition System
Contains 12 gulpfile.*.ts files defining platform-specific build tasks:
- `gulpfile.ts` (main orchestrator)
- `gulpfile.cli.ts`, `gulpfile.compile.ts`, `gulpfile.editor.ts`, `gulpfile.scan.ts`, `gulpfile.hygiene.ts` (CLI/compilation)
- `gulpfile.vscode.ts`, `gulpfile.vscode.web.ts`, `gulpfile.vscode.linux.ts`, `gulpfile.vscode.win32.ts` (platform targets)
- `gulpfile.reh.ts` (remote execution host)
- `gulpfile.extensions.ts` (extension system)

These files use `gulp.task()` extensively to define bundling, compilation, and asset processing workflows. They orchestrate the AMD module format bundling pipeline that a Tauri/Rust port would replace wholesale.

### TypeScript Build System (tsb)
4-file subsystem implementing custom TypeScript transpilation:
- `build/lib/tsb/builder.ts` - Batch compilation orchestrator
- `build/lib/tsb/transpiler.ts` - Module format transformation (ESM->AMD)
- `build/lib/tsb/index.ts` & `build/lib/tsb/utils.ts` - Supporting utilities

Handles conversion from modern ESM source to AMD-compatible modules, a constraint that disappears in Rust/Tauri.

### Code Mangling and Optimization Pipeline
3-file subsystem for production code optimization:
- `build/lib/mangle/index.ts` - Main mangling orchestration
- `build/lib/mangle/renameWorker.ts` - Worker thread for symbol renaming
- `build/lib/mangle/staticLanguageServiceHost.ts` - Static analysis via TS LSP

Integrated with `build/lib/optimize.ts` and `build/lib/treeshaking.ts` for comprehensive optimization. Becomes unnecessary with Rust's native compilation model.

### Policy Code Generation System
11-file cluster for generating platform policy code:
- Base classes: `basePolicy.ts`
- Type-specific: `booleanPolicy.ts`, `numberPolicy.ts`, `stringPolicy.ts`, `stringEnumPolicy.ts`, `objectPolicy.ts`
- Orchestration: `policyGenerator.ts`, `render.ts`
- Utilities: `copyPolicyDto.ts`, `exportPolicyData.ts`, `types.ts`
- Data: `policyData.jsonc`

Generates Windows ADMX, macOS plist, and Linux JSON policy definitions from unified source.

### Platform-Specific Signing Infrastructure
2-layer signing system (19 files):
- Generic layer: `build/azure-pipelines/common/sign.ts`, `codesign.ts`
- Platform layers: `darwin/codesign.ts`, `linux/codesign.ts`, `win32/codesign.ts`, `common/sign-win32.ts`
- macOS specific: `build/darwin/sign.ts`, `sign-server.ts`, `verify-macho.ts`, `create-universal-app.ts`

Handles notarization, code signing, binary verification, and universal (ARM64/x86_64) app packaging.

### Vite / Next-Gen Bundler Preparation
Experimental alternative to gulp/esbuild:
- `build/vite/vite.config.ts` - Vite configuration
- `build/vite/` - Complete Vite setup (5 TS files + node_modules)
- `build/next/` - Next-gen bundler plugins (3 TS files + 2 test files)
- `build/rspack/` - Alternative Rspack bundler (1 config file)

Represents ongoing exploration of modern bundlers alongside the legacy gulp/AMD system.

### Azure Pipelines CI/CD Infrastructure
27 TypeScript files across build stages:
- Common utilities: 13 files in `build/azure-pipelines/common/`
- Platform-specific: Darwin, Linux, Win32 codesigning
- Type publishing: Version checking and type definition updates
- Distro management: npm/quality mixin scripts
- Telemetry and sourcemap publication

Orchestrates multi-platform builds, signing, publishing, and artifact management.

### npm Hook System
6 files implementing npm install phase integration:
- Hooks: `preinstall.ts`, `postinstall.ts`
- Optimization: `fast-install.ts`, `installStateHash.ts`
- Updates: `update-distro.ts`, `update-all-grammars.ts`, `update-localization-extension.ts`

Manages native module compilation, distro pinning, and grammar/localization synchronization during installation.

---

The build system implements a sophisticated multi-stage pipeline for bundling VS Code from TypeScript source into AMD modules, optimizing them via mangling and tree-shaking, packaging platform-specific distributions (macOS DMG, Windows MSI, Linux deb/rpm), and coordinating code signing and CI/CD publication. A Tauri/Rust port would eliminate the AMD module format constraint, the gulp orchestration layer, the TypeScript transpilation stage, and the complex optimization pipeline, replacing them with Rust's native compilation model and a web-based UI framework.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
