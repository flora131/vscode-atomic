# VS Code Build System File Locations

## Implementation / Core Build Files

### Main Gulp Configuration
- `build/gulpfile.ts` - Primary gulpfile defining main compile, watch, and extension tasks
- `build/gulpfile.compile.ts` - Compile-specific tasks (with/without mangling)
- `build/gulpfile.extensions.ts` - Extension compilation and watching
- `build/gulpfile.editor.ts` - Monaco/editor compilation
- `build/gulpfile.hygiene.ts` - Code quality/hygiene checks
- `build/gulpfile.cli.ts` - CLI build tasks
- `build/gulpfile.vscode.ts` - Main VS Code build orchestration
- `build/gulpfile.vscode.web.ts` - Web build variant
- `build/gulpfile.vscode.win32.ts` - Windows-specific build
- `build/gulpfile.vscode.linux.ts` - Linux-specific build
- `build/gulpfile.reh.ts` - Remote execution host builds
- `build/gulpfile.scan.ts` - Scanning/analysis tasks

### Build Library (Transpilation & Compilation)
- `build/lib/compilation.ts` - Core compilation pipeline using gulp-tsb and esbuild
- `build/lib/task.ts` - Gulp task wrapper and executor
- `build/lib/bundle.ts` - Bundle entry point definitions and TypeScript boilerplate removal
- `build/lib/optimize.ts` - ESBuild-based bundling and optimization
- `build/lib/util.ts` - Build utility functions
- `build/lib/extensions.ts` - Extension discovery and compilation helpers

### Esbuild/Next Generation Build System
- `build/next/index.ts` - Modern esbuild-based transpilation and bundling system (45KB)
- `build/next/nls-plugin.ts` - NLS (National Language Support) esbuild plugin
- `build/next/private-to-property.ts` - TypeScript private field transformation
- `build/lib/tsb/index.ts` - TypeScript Source Builder orchestration
- `build/lib/tsb/builder.ts` - TSB builder implementation (23KB)
- `build/lib/tsb/transpiler.ts` - TSB transpiler (11KB)
- `build/lib/tsb/utils.ts` - TSB utilities

### Build Configuration
- `build/buildConfig.ts` - Feature flags (useEsbuildTranspile)
- `build/buildfile.ts` - Module entry points for bundling
- `build/tsconfig.json` - TypeScript configuration for build system itself

### Bundling & Code Processing
- `build/lib/inlineMeta.ts` - Inline metadata processing
- `build/lib/treeshaking.ts` - Tree-shaking utilities
- `build/lib/mangle/index.ts` - Code mangling/minification (20KB)
- `build/lib/mangle/renameWorker.ts` - Rename worker for mangling
- `build/lib/mangle/staticLanguageServiceHost.ts` - Language service for mangling
- `build/lib/nls.ts` - Localization/NLS support
- `build/lib/nls-analysis.ts` - NLS analysis

### Type System & Analysis
- `build/lib/typeScriptLanguageServiceHost.ts` - TypeScript language service
- `build/lib/tsconfigUtils.ts` - TypeScript configuration utilities
- `build/lib/propertyInitOrderChecker.ts` - Property initialization validation
- `build/lib/standalone.ts` - Standalone compilation helpers

### Policies & Configuration Generation
- `build/lib/policies/policyGenerator.ts` - Generate policy definitions
- `build/lib/policies/basePolicy.ts` - Base policy types
- `build/lib/policies/booleanPolicy.ts` - Boolean policy definitions
- `build/lib/policies/numberPolicy.ts` - Numeric policy definitions
- `build/lib/policies/stringPolicy.ts` - String policy definitions
- `build/lib/policies/stringEnumPolicy.ts` - Enum policy definitions
- `build/lib/policies/objectPolicy.ts` - Object policy definitions
- `build/lib/policies/copyPolicyDto.ts` - Policy copying utilities
- `build/lib/policies/exportPolicyData.ts` - Policy export helpers
- `build/lib/policies/render.ts` - Policy rendering
- `build/lib/policies/types.ts` - Policy type definitions

### Code Quality & Checking
- `build/eslint.ts` - ESLint runner
- `build/gulp-eslint.ts` - Gulp ESLint integration
- `build/stylelint.ts` - StyleLint runner
- `build/lib/stylelint/validateVariableNames.ts` - CSS variable validation
- `build/hygiene.ts` - Hygiene checks

### Build Utilities
- `build/lib/date.ts` - Date/version utilities
- `build/lib/getVersion.ts` - Version retrieval from Git
- `build/lib/git.ts` - Git integration
- `build/lib/reporter.ts` - Build error/warning reporter
- `build/lib/fetch.ts` - HTTP fetching utilities
- `build/lib/stats.ts` - Build statistics
- `build/lib/formatter.ts` - Output formatting

### Extension & Built-in Handling
- `build/lib/builtInExtensions.ts` - Built-in extension scanning
- `build/lib/builtInExtensionsCG.ts` - Built-in extension codegen
- `build/lib/extractExtensionPoints.ts` - Extension point extraction
- `build/lib/dependencies.ts` - Dependency resolution
- `build/lib/checkCyclicDependencies.ts` - Cycle detection

### Electron & Binary Integration
- `build/lib/electron.ts` - Electron build integration
- `build/lib/asar.ts` - ASAR archive handling

### CI/CD Integration
- `build/lib/embeddedType.ts` - Type embedding
- `build/lib/monaco-api.ts` - Monaco API generation
- `build/lib/preLaunch.ts` - Pre-launch setup
- `build/lib/watch/index.ts` - File watching with debouncing
- `build/lib/watch/watch-win32.ts` - Windows-specific watching
- `build/lib/screenshotBlocksCi.ts` - Screenshot blocking in CI
- `build/lib/screenshotDiffReport.ts` - Screenshot diff reporting
- `build/copilot-migrate-pr.ts` - Copilot PR migration utility
- `build/filters.ts` - File filtering utilities

### Vite/Modern Bundler Configuration
- `build/vite/vite.config.ts` - Vite bundler configuration (50KB)
- `build/vite/index.ts` - Vite entry point
- `build/vite/index-workbench.ts` - Workbench-specific config
- `build/vite/workbench-vite.ts` - Workbench Vite integration
- `build/vite/setup-dev.ts` - Development setup

### Rspack Configuration
- `build/rspack/rspack.serve-out.config.mts` - Rspack bundler config (50KB)
- `build/rspack/workbench-rspack.html` - Rspack workbench HTML

### Platform-Specific Build Files

**macOS:**
- `build/darwin/create-dmg.ts` - DMG creation script
- `build/darwin/create-universal-app.ts` - Universal app building (Intel + Apple Silicon)
- `build/darwin/sign.ts` - Code signing (6.5KB)
- `build/darwin/sign-server.ts` - Server-side signing
- `build/darwin/verify-macho.ts` - Mach-O verification
- `build/darwin/distribution.provisionprofile` - Provisioning profile
- `build/darwin/dmg-settings.py.template` - DMG settings template
- `build/darwin/patch-dmg.py` - DMG patching script
- `build/darwin/dmg-background-*.tiff` - DMG background images (3 variants)

**Windows:**
- `build/win32/code.iss` - Inno Setup installer script (250KB)
- `build/win32/explorer-dll-fetcher.ts` - Windows Explorer DLL fetching
- `build/win32/inno_updater.exe` - Inno updater executable
- `build/win32/vcruntime140.dll` - Visual C runtime
- `build/win32/Cargo.toml` - Rust project config
- `build/win32/Cargo.lock` - Rust dependencies
- `build/win32/i18n/` - Windows-specific i18n files

**Linux:**
- `build/linux/dependencies-generator.ts` - Dependency generator
- `build/linux/libcxx-fetcher.ts` - libcxx fetching
- `build/linux/debian/` - Debian package support
- `build/linux/rpm/` - RPM package support

### NPM & Package Management
- `build/npm/dirs.ts` - Directory configuration
- `build/npm/postinstall.ts` - Post-install hooks (12.5KB)
- `build/npm/preinstall.ts` - Pre-install hooks (7.7KB)
- `build/npm/fast-install.ts` - Fast installation
- `build/npm/installStateHash.ts` - Installation state tracking
- `build/npm/mixin-telemetry-docs.ts` - Telemetry documentation
- `build/npm/update-all-grammars.ts` - Grammar updates
- `build/npm/update-distro.ts` - Distribution updates
- `build/npm/update-localization-extension.ts` - Localization updates
- `build/npm/gyp/` - Native module building (node-gyp)

### Azure Pipelines CI/CD
- `build/azure-pipelines/common/checkForArtifact.ts` - Artifact checking
- `build/azure-pipelines/common/createBuild.ts` - Build creation
- `build/azure-pipelines/common/codesign.ts` - Generic code signing
- `build/azure-pipelines/common/computeNodeModulesCacheKey.ts` - Cache key generation
- `build/azure-pipelines/common/computeBuiltInDepsCacheKey.ts` - Dependency cache key
- `build/azure-pipelines/common/checkDistroCommit.ts` - Distribution commit checking
- `build/azure-pipelines/common/downloadCopilotVsix.ts` - Copilot VSIX download
- `build/azure-pipelines/common/getPublishAuthTokens.ts` - Auth token retrieval
- `build/azure-pipelines/common/listNodeModules.ts` - Node modules listing
- `build/azure-pipelines/common/publish.ts` - Publishing logic
- `build/azure-pipelines/common/sign.ts` - Build signing
- `build/azure-pipelines/common/sign-win32.ts` - Windows-specific signing
- `build/azure-pipelines/common/retry.ts` - Retry logic
- `build/azure-pipelines/common/releaseBuild.ts` - Release build helpers
- `build/azure-pipelines/common/waitForArtifacts.ts` - Artifact waiting
- `build/azure-pipelines/common/extract-telemetry.ts` - Telemetry extraction
- `build/azure-pipelines/darwin/codesign.ts` - macOS-specific signing
- `build/azure-pipelines/linux/codesign.ts` - Linux-specific signing
- `build/azure-pipelines/win32/codesign.ts` - Windows-specific signing
- `build/azure-pipelines/upload-nlsmetadata.ts` - NLS metadata upload
- `build/azure-pipelines/upload-sourcemaps.ts` - Source map upload
- `build/azure-pipelines/upload-cdn.ts` - CDN upload
- `build/azure-pipelines/publish-types/check-version.ts` - Version checking
- `build/azure-pipelines/publish-types/update-types.ts` - Type definition updates
- `build/azure-pipelines/update-dependencies-check.ts` - Dependency updates
- `build/azure-pipelines/distro/mixin-quality.ts` - Quality distribution mixins
- `build/azure-pipelines/distro/mixin-npm.ts` - NPM distribution mixins

### Configuration & Checker
- `build/checker/layersChecker.ts` - Architecture layer validation
- `build/checker/tsconfig.*.json` - TypeScript configs for different layers:
  - `tsconfig.browser.json` - Browser layer
  - `tsconfig.electron-browser.json` - Electron browser layer
  - `tsconfig.electron-main.json` - Electron main process
  - `tsconfig.electron-utility.json` - Electron utility
  - `tsconfig.node.json` - Node.js layer
  - `tsconfig.worker.json` - Web worker layer

### Built-in Extensions & Monaco
- `build/builtin/` - Built-in extension management
- `build/monaco/` - Monaco Editor integration (package.json, recipes)

## Test Files

- `build/lib/test/booleanPolicy.test.ts` - Boolean policy tests
- `build/lib/test/checkCyclicDependencies.test.ts` - Cycle detection tests
- `build/lib/test/i18n.test.ts` - Internationalization tests
- `build/lib/test/numberPolicy.test.ts` - Number policy tests
- `build/lib/test/objectPolicy.test.ts` - Object policy tests
- `build/lib/test/policyConversion.test.ts` - Policy conversion tests
- `build/lib/test/render.test.ts` - Rendering tests
- `build/lib/test/stringEnumPolicy.test.ts` - String enum policy tests
- `build/lib/test/stringPolicy.test.ts` - String policy tests
- `build/next/test/nls-sourcemap.test.ts` - NLS sourcemap tests
- `build/next/test/private-to-property.test.ts` - Private field transformation tests

## Configuration Files

### TypeScript Configuration
- `build/tsconfig.json` - Root TypeScript config for build system
- `build/vite/tsconfig.json` - Vite-specific TypeScript config
- `build/checker/tsconfig.*.json` - Layer-specific TypeScript configs (6 files)

### JSON Configuration
- `build/package.json` - Build system dependencies
- `build/package-lock.json` - Locked dependency versions
- `build/azure-pipelines/config/tsaoptions.json` - TSA options
- `build/azure-pipelines/config/CredScanSuppressions.json` - Credential scan suppressions
- `build/azure-pipelines/common/telemetry-config.json` - Telemetry configuration
- `build/lib/i18n.resources.json` - NLS resources
- `build/lib/stylelint/vscode-known-variables.json` - CSS variable definitions
- `build/lib/test/fixtures/policies/*/policy.json` - Policy fixtures
- `build/lib/typings/cgmanifest.json` - Component governance manifest
- `build/rspack/package.json` - Rspack dependencies
- `build/vite/package.json` - Vite dependencies
- `build/builtin/package.json` - Built-in extension package
- `build/monaco/package.json` - Monaco package info
- `build/npm/gyp/package.json` - Node-gyp package

### Other Configuration
- `build/.moduleignore` - Module ignore patterns
- `build/.webignore` - Web build ignore patterns

## Type Definitions / Documentation

### Type Definitions
- `build/lib/typings/` - Custom TypeScript type definitions (11 .d.ts files):
  - `gulp-buffer.d.ts`
  - `event-stream.d.ts`
  - `gulp-vinyl-zip.d.ts`
  - `gulp-gunzip.d.ts`
  - `rcedit.d.ts`
  - `ternary-stream.d.ts`
  - `asar.d.ts`
  - `gulp-bom.d.ts`
  - `stream.d.ts`
  - `gulp-azure-storage.d.ts`
  - `vscode-gulp-watch.d.ts`
  - `chromium-pickle-js.d.ts`
  - `@vscode/gulp-electron.d.ts`

### Documentation
- `build/next/working.md` - esbuild implementation notes (26KB)
- `build/monaco/README.md` - Monaco package documentation
- `build/monaco/README-npm.md` - Monaco NPM documentation
- `build/monaco/LICENSE` - Monaco license
- `build/monaco/ThirdPartyNotices.txt` - Third-party notices
- `build/linux/debian/` - Debian packaging docs
- `build/linux/rpm/` - RPM packaging docs

## Notable Clusters

### Build System Core (163 TypeScript files, ~33KB LOC)
- Main orchestration: `gulpfile.ts` + 11 specialized gulpfiles
- Compilation engine: `lib/compilation.ts`, `lib/tsb/`, `next/index.ts`
- Task management: `lib/task.ts` wrapper system

### Multi-Target Build Support
- **Desktop/Electron**: `gulpfile.vscode.ts`, `lib/electron.ts`
- **Web/Browser**: `gulpfile.vscode.web.ts`, `build/vite/`
- **Remote Execution Host**: `gulpfile.reh.ts`
- **CLI**: `gulpfile.cli.ts`
- **Editor/Monaco**: `gulpfile.editor.ts`, `build/monaco/`

### Bundler Integration
- **Gulp+TSB**: Legacy/hybrid approach in `lib/compilation.ts`, `lib/tsb/`
- **Esbuild**: Modern fast transpilation in `build/next/` (45KB entry point)
- **Vite**: Development server in `build/vite/vite.config.ts` (50KB+)
- **Rspack**: Rspack bundler in `build/rspack/rspack.serve-out.config.mts` (50KB+)

### Platform-Specific Building
- **macOS**: `build/darwin/` - 10 files including signing, DMG creation, universal app
- **Windows**: `build/win32/` - Inno Setup scripts, explorer integration, Rust tools (Cargo)
- **Linux**: `build/linux/` - Debian/RPM packaging, dependencies

### CI/CD Pipeline
- **Azure Pipelines**: `build/azure-pipelines/` - 20+ TypeScript integration points
- Platform signing, artifact management, telemetry, NLS metadata
- Architecture: `common/` (shared) + platform-specific (`darwin/`, `linux/`, `win32/`)

### Code Quality & Analysis
- **Policies**: `build/lib/policies/` - 11 files defining build-time policies
- **Layer Checking**: `build/checker/layersChecker.ts` + 6 TypeScript configs
- **Linting**: `build/eslint.ts`, `build/stylelint.ts`
- **Dependency Analysis**: `checkCyclicDependencies.ts`

### Localization System
- **NLS Processing**: `lib/nls.ts`, `lib/nls-analysis.ts`, `next/nls-plugin.ts`
- **Resource Management**: `lib/i18n.ts`, build-time locale handling
- **Metadata**: `build/azure-pipelines/upload-nlsmetadata.ts`

### Built-in Content
- **Extension System**: `lib/builtInExtensions.ts`, `lib/extractExtensionPoints.ts`
- **Built-in Discovery**: `build/builtin/` directory for extension management

## Summary

The VS Code build system spans 195 files organized into:

1. **Core Build Pipeline** (12 gulpfiles + compilation library) - Orchestrates full transpilation, bundling, and optimization using multiple tools (Gulp, TSB, esbuild, Vite, Rspack)

2. **Modern Transpilation** (esbuild-based `build/next/`) - Fast TypeScript-to-JavaScript with plugin support for NLS, private field transformation, and source mapping

3. **Multi-Platform Support** - Dedicated directories for macOS (signing, DMG), Windows (Inno Setup, Explorer), Linux (packaging)

4. **CI/CD Integration** - Azure Pipelines with 20+ helper scripts for signing, publishing, telemetry, and artifact management

5. **Code Quality Tools** - Policy generation, layer validation, linting, cycle detection

6. **Alternative Bundlers** - Emerging support for Vite and Rspack as development alternatives to traditional Gulp+TSB

Key architectural patterns:
- **Modular gulpfiles**: Each specialized gulpfile imported into main orchestrator
- **Library-based approach**: Reusable utilities in `build/lib/` (~70 files)
- **Dual compilation paths**: Legacy Gulp+TSB with Esbuild transpilation option (`useEsbuildTranspile` config)
- **Platform abstraction**: Windows/macOS/Linux specific code isolated in separate directories
