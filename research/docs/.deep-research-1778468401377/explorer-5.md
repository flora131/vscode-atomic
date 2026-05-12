# Partition 5 of 80 — Findings

## Scope
`build/` (195 files, 33,647 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Build System Porting Analysis: VS Code Electron → Tauri/Rust

**Research Focus:** `build/` directory (195 files, ~13,676 LOC)  
**Query Targets:** Gulp task definitions, Electron-specific tooling, platform-specific packaging

---

## Implementation

### Core Build Orchestration
- `build/gulpfile.ts` — Master Gulp orchestration; defines compile, transpile, watch tasks
- `build/gulpfile.compile.ts` — Compilation with/without mangling (TypeScript → JavaScript)
- `build/gulpfile.extensions.ts` — Extension compilation for multiple platforms
- `build/buildfile.ts` — Entry points definition (workers, workbench, CLI, sessions)
- `build/buildConfig.ts` — Build configuration flags (esbuild transpile mode)

### Electron-Specific Infrastructure
- `build/lib/electron.ts` — Electron packaging config, Darwin document types, Info.plist generation
- `build/lib/asar.ts` — ASAR archive creation (Electron's packaging format); handles unpacking globs for native modules
- `build/lib/preLaunch.ts` — Pre-launch environment setup

### Platform-Specific Packaging

#### Windows
- `build/gulpfile.vscode.win32.ts` — Inno Setup installer generation, EXE icon/version embedding (rcedit)
- `build/win32/explorer-dll-fetcher.ts` — Windows Shell Explorer integration DLL fetcher
- `build/azure-pipelines/common/sign-win32.ts` — Windows code signing via ESRP

#### macOS
- `build/darwin/sign.ts` — Electron app code signing (@electron/osx-sign), entitlements management
- `build/darwin/create-dmg.ts` — DMG volume creation (dmgbuild Python tool via Homebrew)
- `build/darwin/create-universal-app.ts` — Universal binary (x64 + arm64) creation
- `build/darwin/verify-macho.ts` — Mach-O binary verification
- `build/azure-pipelines/darwin/codesign.ts` — Darwin notarization pipeline

#### Linux
- `build/gulpfile.vscode.linux.ts` — DEB/RPM package generation, desktop file templating
- `build/linux/debian/calculate-deps.ts` — Debian dependency calculation from binaries
- `build/linux/debian/dep-lists.ts` — Recommended dependency lists
- `build/linux/debian/install-sysroot.ts` — Sysroot installation
- `build/linux/rpm/dep-lists.ts` — RPM recommended dependencies
- `build/linux/libcxx-fetcher.ts` — Linux libstdc++ fetcher
- `build/linux/dependencies-generator.ts` — Cross-platform dependency generation

### Remote Extension Host (Server)
- `build/gulpfile.reh.ts` — Server bundle compilation for win32, darwin, linux, alpine (multiple architectures)
- Multi-platform binary fetching (Node.js pre-built binaries)

### Web/Browser Build
- `build/gulpfile.vscode.web.ts` — Browser/web build target compilation
- `build/vite/vite.config.ts` — Vite bundler configuration
- `build/vite/workbench-electron.ts` — Electron-specific Vite setup
- `build/vite/setup-dev.ts` — Development server setup

### Code Bundling & Optimization
- `build/lib/bundle.ts` — Module bundling strategy
- `build/lib/optimize.ts` — Minification, tree-shaking, bundle optimization
- `build/lib/esbuild.ts` — esbuild wrapper (transpile/bundle operations)
- `build/next/index.ts` — Modern esbuild-based bundler (transpile and bundle commands)
- `build/next/nls-plugin.ts` — NLS (National Language Support) esbuild plugin
- `build/next/private-to-property.ts` — Private field → property conversion for esbuild

### Asset & Resource Management
- `build/lib/asar.ts` — Archive creation with selective unpacking for native modules
- `build/lib/inlineMeta.ts` — Inline metadata injection
- `build/lib/nls.ts` — NLS resource compilation and analysis
- `build/lib/nls-analysis.ts` — NLS analysis tooling
- `build/lib/compilation.ts` — Icon/codicon compilation (Codicon SVG → font)

### Dependency & Module Analysis
- `build/lib/dependencies.ts` — Production dependency resolution
- `build/lib/checkCyclicDependencies.ts` — Cyclic dependency detection
- `build/lib/extensions.ts` — Built-in extension scanning and filtering
- `build/lib/builtInExtensions.ts` — Built-in extension metadata
- `build/lib/builtInExtensionsCG` — Code generation for built-in extensions

### Policy & Configuration Generation
- `build/lib/policies/policyGenerator.ts` — Group Policy template generation
- `build/lib/policies/basePolicy.ts` — Base policy abstraction
- `build/lib/policies/{booleanPolicy,stringPolicy,numberPolicy,stringEnumPolicy,objectPolicy}.ts` — Policy type handlers
- `build/lib/policies/exportPolicyData.ts` — Policy data export
- `build/lib/policies/render.ts` — Policy XML/JSON rendering

### Validation & Hygiene
- `build/gulpfile.hygiene.ts` — Code hygiene checks (package.json validation)
- `build/lib/propertyInitOrderChecker.ts` — Property initialization order validation
- `build/checker/layersChecker.ts` — Dependency layer enforcement

### Utilities & Helpers
- `build/lib/util.ts` — File manipulation (rimraf, chmod, source map rewriting)
- `build/lib/task.ts` — Gulp task abstraction layer
- `build/lib/getVersion.ts` — Git commit version extraction
- `build/lib/date.ts` — ISO date read/write for build metadata
- `build/lib/git.ts` — Git integration
- `build/lib/node.ts` — Node.js environment utilities
- `build/lib/fetch.ts` — Remote artifact fetching
- `build/lib/reporter.ts` — Build logging/reporting
- `build/lib/stats.ts` — Build statistics collection
- `build/lib/formatter.ts` — Output formatting
- `build/filters.ts` — Gulp filter helpers
- `build/eslint.ts` — ESLint integration
- `build/gulp-eslint.ts` — Custom ESLint Gulp plugin
- `build/lib/stylelint/validateVariableNames.ts` — CSS variable validation

### TypeScript & Compilation Infrastructure
- `build/lib/tsb/builder.ts` — gulp-tsb compilation builder
- `build/lib/tsb/transpiler.ts` — Custom TypeScript transpiler
- `build/lib/tsb/utils.ts` — TSB utilities
- `build/lib/typeScriptLanguageServiceHost.ts` — TS language service integration
- `build/lib/tsconfigUtils.ts` — tsconfig.json utilities
- `build/lib/tsgo.ts` — tsgo type-checking spawner
- `build/checker/tsconfig.*.json` — Type-check configs (electron-main, electron-browser, node, worker, utility, browser)

### CLI & Copilot Support
- `build/gulpfile.cli.ts` — VS Code CLI build
- `build/lib/copilot.ts` — Copilot extension shim generation and filtering
- `build/copilot-migrate-pr.ts` — Copilot PR migration tool

### Scanning & Metadata
- `build/gulpfile.scan.ts` — Symbol scanning (for workbench symbol table)
- `build/lib/monaco-api.ts` — Monaco API generation
- `build/lib/extractExtensionPoints.ts` — Extension point extraction

### Editor-Specific Build
- `build/gulpfile.editor.ts` — Monaco Editor distro build
- `build/lib/standalone.ts` — Standalone editor build support

---

## Tests

### Unit Tests
- `build/lib/test/checkCyclicDependencies.test.ts` — Cyclic dependency detection tests
- `build/lib/test/booleanPolicy.test.ts` — Boolean policy parsing tests
- `build/lib/test/stringPolicy.test.ts` — String policy tests
- `build/lib/test/numberPolicy.test.ts` — Numeric policy tests
- `build/lib/test/stringEnumPolicy.test.ts` — Enum policy tests
- `build/lib/test/objectPolicy.test.ts` — Object policy tests
- `build/lib/test/policyConversion.test.ts` — Policy conversion tests
- `build/lib/test/render.test.ts` — Policy rendering tests
- `build/lib/test/i18n.test.ts` — Internationalization tests

### Integration/E2E Test YAMLs
- `build/azure-pipelines/common/sanity-tests.yml` — Common sanity checks
- `build/azure-pipelines/darwin/steps/product-build-darwin-test.yml` — macOS test steps
- `build/azure-pipelines/linux/steps/product-build-linux-test.yml` — Linux test steps
- `build/azure-pipelines/win32/steps/product-build-win32-test.yml` — Windows test steps
- `build/azure-pipelines/copilot/test-steps.yml` — Copilot tests
- `build/azure-pipelines/copilot/test-integration-steps.yml` — Copilot integration tests

### Build Verification
- `build/next/test/private-to-property.test.ts` — Private field conversion tests
- `build/next/test/nls-sourcemap.test.ts` — NLS sourcemap tests

### Test Fixtures
- `build/lib/test/fixtures/policies/` — Policy test fixtures (win32, darwin, linux with localized variants)

---

## Types / Interfaces

### Type Definitions
- `build/tsconfig.json` — Root build tsconfig
- `build/vite/tsconfig.json` — Vite tsconfig
- `build/checker/tsconfig.electron-main.json` — Electron main type-check config
- `build/checker/tsconfig.electron-browser.json` — Electron browser type-check config
- `build/checker/tsconfig.electron-utility.json` — Electron utility type-check config
- `build/checker/tsconfig.node.json` — Node.js type-check config
- `build/checker/tsconfig.worker.json` — Web worker type-check config
- `build/checker/tsconfig.browser.json` — Browser type-check config

### Custom Type Stubs
- `build/lib/typings/@vscode/gulp-electron.d.ts` — Electron Gulp plugin type definitions
- `build/lib/typings/asar.d.ts` — ASAR archive type definitions
- `build/lib/typings/chromium-pickle-js.d.ts` — Chromium pickle JS types
- `build/lib/typings/rcedit.d.ts` — RC Edit (Windows resource tool) types
- `build/lib/typings/event-stream.d.ts` — Event stream types
- `build/lib/typings/gulp-*.d.ts` — Gulp plugin types (buffer, filter, flatmap, gunzip, bom, azure-storage, vinyl-zip)
- `build/lib/typings/ternary-stream.d.ts` — Stream utility types
- `build/lib/typings/vscode-gulp-watch.d.ts` — VS Code Gulp watch types
- `build/lib/typings/stream.d.ts` — Custom stream types

---

## Configuration

### Build Configuration
- `build/package.json` — Build system dependencies (@electron/*, esbuild, gulp plugins, Azure SDKs, signing tools)
- `build/package-lock.json` — Locked dependencies
- `build/buildConfig.ts` — esbuild vs gulp-tsb transpilation mode flag

### Azure Pipelines CI/CD
- `build/azure-pipelines/product-build.yml` — Main product build pipeline
- `build/azure-pipelines/product-release.yml` — Release pipeline
- `build/azure-pipelines/product-publish.yml` — Publishing pipeline
- `build/azure-pipelines/product-quality-checks.yml` — Quality checks

#### Platform-Specific CI
- `build/azure-pipelines/win32/product-build-win32.yml` — Windows build
- `build/azure-pipelines/win32/product-build-win32-ci.yml` — Windows CI variant
- `build/azure-pipelines/darwin/product-build-darwin.yml` — macOS build
- `build/azure-pipelines/darwin/product-build-darwin-ci.yml` — macOS CI variant
- `build/azure-pipelines/darwin/product-build-darwin-universal.yml` — Universal binary build
- `build/azure-pipelines/linux/product-build-linux.yml` — Linux build
- `build/azure-pipelines/linux/product-build-linux-ci.yml` — Linux CI variant

#### Build Variants
- `build/azure-pipelines/web/product-build-web.yml` — Web/browser variant
- `build/azure-pipelines/alpine/product-build-alpine.yml` — Alpine Linux variant
- `build/azure-pipelines/cli/cli-*.yml` — CLI builds

#### Dependency & Node Modules
- `build/azure-pipelines/win32/product-build-win32-node-modules.yml` — Node modules cache
- `build/azure-pipelines/darwin/product-build-darwin-node-modules.yml`
- `build/azure-pipelines/linux/product-build-linux-node-modules.yml`
- `build/azure-pipelines/web/product-build-web-node-modules.yml`
- `build/azure-pipelines/alpine/product-build-alpine-node-modules.yml`

#### Compilation Steps
- `build/azure-pipelines/darwin/steps/product-build-darwin-compile.yml`
- `build/azure-pipelines/linux/steps/product-build-linux-compile.yml`
- `build/azure-pipelines/win32/steps/product-build-win32-compile.yml`

#### Toolchain Setup
- `build/azure-pipelines/cli/install-rust-posix.yml` — Rust toolchain for CLI
- `build/azure-pipelines/win32/steps/product-build-win32-install-rust.yml` — Windows Rust

#### Signing & Codesign
- `build/azure-pipelines/common/sign.ts` — Generic signing (ESRP parameters for Windows/macOS/Linux)
- `build/azure-pipelines/common/sign-win32.ts` — Windows Authenticode signing
- `build/azure-pipelines/darwin/codesign.ts` — macOS notarization
- `build/azure-pipelines/win32/codesign.ts` — Windows codesigning pipeline
- `build/azure-pipelines/linux/codesign.ts` — Linux signing (minimal)

#### Artifact & Publishing
- `build/azure-pipelines/common/publish.ts` — Artifact publication to Azure Blob Storage
- `build/azure-pipelines/common/checkForArtifact.ts` — Artifact validation
- `build/azure-pipelines/common/waitForArtifacts.ts` — Cross-platform artifact synchronization
- `build/azure-pipelines/common/publishArtifact.yml` — Artifact staging

#### Special Builds
- `build/azure-pipelines/copilot/build-steps.yml` — Copilot extension build
- `build/azure-pipelines/product-copilot.yml` — Full Copilot build
- `build/azure-pipelines/product-copilot-recovery.yml` — Copilot recovery
- `build/azure-pipelines/distro/distro-build.yml` — Distro mixin build
- `build/azure-pipelines/distro/download-distro.yml` — Distro download
- `build/azure-pipelines/distro/mixin-quality.ts` — Quality mixin
- `build/azure-pipelines/distro/mixin-npm.ts` — NPM registry mixin
- `build/azure-pipelines/publish-types/publish-types.yml` — Type definitions publishing

#### Dependency Checks
- `build/azure-pipelines/dependencies-check.yml` — Dependency validation
- `build/azure-pipelines/common/update-dependencies-check.ts` — Dependency updates

#### Utilities
- `build/azure-pipelines/common/createBuild.ts` — Build creation
- `build/azure-pipelines/common/computeNodeModulesCacheKey.ts` — Cache key computation
- `build/azure-pipelines/common/computeBuiltInDepsCacheKey.ts` — Built-in deps cache
- `build/azure-pipelines/common/listNodeModules.ts` — Node modules enumeration
- `build/azure-pipelines/common/retry.ts` — Retry logic
- `build/azure-pipelines/common/releaseBuild.ts` — Release build metadata
- `build/azure-pipelines/common/getPublishAuthTokens.ts` — Auth token retrieval
- `build/azure-pipelines/common/checkDistroCommit.ts` — Distro commit validation
- `build/azure-pipelines/common/downloadCopilotVsix.ts` — Copilot VSIX download
- `build/azure-pipelines/common/extract-telemetry.ts` — Telemetry extraction
- `build/azure-pipelines/upload-sourcemaps.ts` — Source map uploading
- `build/azure-pipelines/upload-cdn.ts` — CDN upload
- `build/azure-pipelines/upload-nlsmetadata.ts` — NLS metadata upload
- `build/azure-pipelines/common/telemetry-config.json` — Telemetry endpoints

### Other Configs
- `build/azure-pipelines/config/CredScanSuppressions.json` — Credential scanning suppressions
- `build/azure-pipelines/config/tsaoptions.json` — TSA (Threat & Security Assessment) options

### Linting & Quality
- `build/lib/stylelint/vscode-known-variables.json` — CSS variable database for linting
- `build/lib/i18n.resources.json` — NLS resource metadata

---

## Examples / Fixtures

### Built-in Resources
- `build/builtin/main.js` — Built-in main process entry (Electron)
- `build/builtin/browser-main.js` — Built-in browser entry
- `build/builtin/package.json` — Built-in package metadata

### Policy Test Fixtures
- `build/lib/test/fixtures/policies/win32/{en-us,fr-fr}/` — Windows Group Policy test data
- `build/lib/test/fixtures/policies/darwin/{en-us,fr-fr}/` — macOS Policy test data
- `build/lib/test/fixtures/policies/linux/` — Linux policy test data

### Monaco/Editor
- `build/monaco/package.json` — Monaco Editor packaging config
- `build/monaco/README.md` — Monaco Editor documentation
- `build/monaco/README-npm.md` — NPM publication guide

### Vite Build
- `build/vite/package.json` — Vite dependencies
- `build/vite/package-lock.json` — Locked Vite versions

### NPM Gyp
- `build/npm/gyp/package.json` — Native module (node-gyp) support
- `build/npm/gyp/package-lock.json`

---

## Documentation

- `build/monaco/README.md` — Monaco Editor build documentation
- `build/monaco/README-npm.md` — NPM publishing guidelines
- Extensive inline comments in build task files (gulpfile*.ts, lib/*.ts)

---

## Notable Clusters

### **Electron-to-Native Bridge (Requires Replacement)**
The following files form a tightly-coupled cluster for Electron packaging that would need Tauri/Rust equivalents:

**Critical Files:**
- `build/lib/electron.ts` (480 LOC) — Defines Electron config, plist generation, Darwin document types
- `build/lib/asar.ts` (180 LOC) — ASAR archive creation (Electron's proprietary package format)
- `build/darwin/sign.ts` (200 LOC) — Uses @electron/osx-sign library for macOS code signing
- `build/darwin/create-dmg.ts` (400 LOC) — DMG creation via Python dmgbuild tool
- `build/gulpfile.vscode.win32.ts` (460 LOC) — Inno Setup installer automation + rcedit integration
- `build/gulpfile.vscode.linux.ts` (500 LOC) — DEB/RPM packaging (fakeroot, dpkg-deb)

**Porting Impact:** 
- ASAR format is Electron-specific; Tauri uses simple filesystem or ZIP archives
- macOS codesigning requires switching from `@electron/osx-sign` to direct `codesign` CLI
- Windows installer generation would shift from Inno Setup to WiX or MSIX
- Linux packaging infrastructure could potentially be reused but requires adaptation for non-Electron layouts

### **Build Orchestration & Gulp Tasks**
A hierarchical network of gulp task definitions across multiple files:

**Core Orchestration:**
- `build/gulpfile.ts` (51 gulp.task calls) → delegates to specialized gulpfiles
- `build/gulpfile.compile.ts` → TypeScript compilation (mangled + unmangled variants)
- `build/gulpfile.extensions.ts` (14 tasks) → extension compilation by platform
- `build/gulpfile.vscode.ts` (177 tasks) → main packaging pipeline
- `build/gulpfile.reh.ts` (193 tasks) → server (remote extension host) builds

**Platform Tasks:**
- Win32: Setup generation, icon embedding, codesigning
- Darwin: Universal binary creation, code signing, notarization
- Linux: DEB/RPM generation, desktop integration
- Web: Browser bundle, Vite integration

**Porting Challenge:** Gulp is JavaScript-centric; Rust/Tauri would likely use Cargo build scripts or shell-based orchestration. The 11 gulpfile variants (~2,000 LOC) would need to be consolidated into Rust build logic.

### **Modern Bundler Integration (Partial Replacement)**
An emerging cluster replacing the older gulp-tsb system:

**New Tooling:**
- `build/next/index.ts` (600 LOC) — esbuild-based transpile/bundle commands
- `build/lib/esbuild.ts` (67 LOC) — esbuild wrapper spawning build/next
- `build/next/nls-plugin.ts` (250 LOC) — esbuild plugin for NLS resource handling
- `build/next/private-to-property.ts` (400 LOC) — esbuild plugin for private field refactoring

**Config:**
- `build/buildConfig.ts` → Flag to switch between esbuild and gulp-tsb

**Observation:** VS Code is transitioning away from gulp. A Tauri port should adopt pure Rust bundling (cargo-based tooling) rather than spawning Node.js processes.

### **Signing & Distribution Pipeline**
A sophisticated multi-stage signing and artifact management system across CI/CD:

**Generic Signing:**
- `build/azure-pipelines/common/sign.ts` (200 LOC) — ESRP (Enterprise Signing Resource Package) client
- Params for: Windows Authenticode, Windows Appx, macOS code signing, Linux packages

**Platform-Specific:**
- Windows: sign-win32.ts spawns signtool via ESRP with SHA256 + timestamping
- macOS: codesign.ts calls Darwin notarization service
- Linux: Minimal signing (GPG for packages)

**Artifact Publishing:**
- `build/azure-pipelines/common/publish.ts` (300+ LOC) — Azure Blob Storage uploads
- Worker thread pool for parallel uploads
- Source map rewriting for CDN distribution
- Supports release gates and JWS signing

**Porting Implication:** Signing workflows are tightly coupled to Azure infrastructure. A Tauri port would need equivalent CI/CD glue code unless it relies on GitHub Releases or alternative CDN (AWS S3, GCS, etc.).

### **Dependency & Module Analysis**
A cluster handling transitive dependency tracking and validation:

**Files:**
- `build/lib/dependencies.ts` (200 LOC) — Production dependency graph extraction
- `build/lib/checkCyclicDependencies.ts` (150 LOC) — Acyclic constraint checker
- `build/lib/extensions.ts` (200 LOC) — Built-in extension scanning with platform filters
- `build/lib/builtInExtensions.ts` (150 LOC) → Extension metadata generation

**Purpose:** Ensures clean dependency isolation between extension host, workbench, and native modules before packaging.

**Porting Note:** This logic is largely platform-agnostic and could be adapted to verify Rust-side dependencies.

### **Policy Generation System**
A complete policy template system for Group Policy and configuration management:

**Files (11 total):**
- `build/lib/policies/policyGenerator.ts` — Main generator
- Type-specific handlers: booleanPolicy, stringPolicy, numberPolicy, stringEnumPolicy, objectPolicy
- Renderers: render.ts (XML/JSON output)
- Code generation: copyPolicyDto.ts, exportPolicyData.ts

**Test Coverage:** 5 test files with fixtures for win32, darwin, linux

**Scope:** Generates Group Policy Administrative Templates, JSON policy files, and TypeScript type definitions for VS Code's policy system.

**Porting Impact:** This is largely self-contained and platform-agnostic; however, it heavily depends on Node.js tooling. A Tauri port might generate policies differently or use existing GPOAT files.

### **Type Checking Infrastructure**
6 separate tsconfig.json files for different build contexts (main/browser/node/worker/electron-main/electron-utility) plus custom type stubs in `build/lib/typings/`.

**Observation:** VS Code maintains strict type boundaries to prevent layer violations. A Tauri port would need to define equivalent Rust module boundaries and trait systems.

### **Web & Server Build Variants**
Parallel build paths for:
- Desktop (Electron) — fully featured
- Web (Vite + browser APIs) — limited scope
- Server (Remote Extension Host) — headless, multi-platform

**Files:**
- `build/gulpfile.vscode.web.ts` (290 LOC)
- `build/vite/` (5 files) — Vite config for web
- `build/gulpfile.reh.ts` (500 LOC) — Server builds for 7+ platform/arch combos

**Porting Challenge:** Tauri focuses on desktop; supporting web and server variants would require separate build configuration.

---

## Summary

The VS Code `build/` directory encodes a **sophisticated, multi-platform packaging and distribution system** built entirely on Node.js/Gulp/Electron tooling (13,676 LOC across 195 files). Porting to Tauri/Rust would require:

### **Must-Replace Components** (~40% of codebase)
1. **Electron packaging** (`electron.ts`, `asar.ts`, platform-specific gulpfiles) → Tauri bundler + native packaging tools
2. **Signing infrastructure** (Darwin/Windows/Linux signing pipelines) → Native OS codesigning CLIs
3. **Build orchestration** (11 gulpfiles, 177+ tasks) → Cargo build scripts or Rust-based task runner
4. **Archive/DMG creation** (ASAR, dmgbuild, Inno Setup automation) → ZIP/native installers

### **Can-Adapt Components** (~40% of codebase)
1. **Bundling logic** (esbuild integration, NLS plugins, minification) → Rust bundler or continue using esbuild via Node invocation
2. **Dependency analysis** (cyclic checks, extension scanning) → Rust equivalents
3. **Type checking** (tsconfig system) → Continue with TypeScript infrastructure or integrate with Rust type system
4. **Policy generation** — Can be preserved or refactored to Rust

### **Keep-As-Is Components** (~20% of codebase)
1. **Extension compilation** (if keeping TypeScript extensions)
2. **Monaco Editor build** (if bundling separately)
3. **Type definitions** (TypeScript-level; unaffected by runtime change)

### **Key Challenges**
- **Electron → Tauri transition** requires rewriting app entry points, IPC, and native module binding
- **Multi-platform complexity:** 8+ build variants (Win32 x64/arm64, Darwin x64/arm64/universal, Linux x64/armhf/arm64, Alpine, Web) would need Tauri/Cargo equivalents
- **CI/CD rewrite:** Azure Pipelines YAML (50+ files) would need adaptation for Rust/Cargo workflows
- **Artifact distribution:** Loss of ASAR means larger package sizes; CDN/update strategy differs
- **Signing complexity:** Platform-specific signing (ESRP, codesign, signtool) remain necessary but coordination changes

**Estimated effort:** The build system alone represents 2–4 weeks of porting work, separate from rewriting the runtime (main process, preload scripts, extension host).

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Build System Patterns: VS Code Electron → Tauri/Rust Port Analysis

## Summary

The VS Code build system (`build/` directory, 195 files, 33,647 LOC) is heavily structured around Gulp task orchestration with platform-specific build chains for Windows, Linux, and macOS. The system manages TypeScript compilation, bundling with esbuild, Electron packaging and signing, ASAR archive creation for native modules, and platform-specific installers (InnoSetup for Windows, DEB/RPM for Linux, DMG for macOS). A Tauri migration would require replacing Electron-specific packaging, signing workflows, and asset bundling while maintaining the compilation and bundling infrastructure.

---

## Key Build Patterns

#### Pattern: Master Gulp Task Orchestration with Task Composition
**Where:** `build/gulpfile.ts:1-63`
**What:** VS Code uses a modular gulp task system where separate gulpfiles are dynamically loaded and composed into named tasks. Tasks are registered with gulp.task() and composed using `task.series()` and `task.parallel()` utilities for sequential or parallel execution.

```typescript
// gulpfile.ts - Core task composition pattern
import gulp from 'gulp';
import { compileExtensionsTask, watchExtensionsTask } from './gulpfile.extensions.ts';
import * as compilation from './lib/compilation.ts';
import * as task from './lib/task.ts';

// Extension point names
gulp.task(compilation.compileExtensionPointNamesTask);

// Client Transpile (series composition)
const transpileClientTask = task.define('transpile-client', task.series(
  util.rimraf('out'), 
  compilation.copyCodiconsTask, 
  compilation.compileTask('src', 'out', false)
));
gulp.task(transpileClientTask);

// All compile tasks (parallel composition)
const _compileTask = task.define('compile', task.parallel(
  monacoTypecheckTask, 
  compileClientTask, 
  compileExtensionsTask, 
  compileExtensionMediaTask
));
gulp.task(_compileTask);

// Watch mode with parallel tasks
gulp.task(task.define('watch', task.parallel(
  watchClientTask, 
  watchExtensionsTask
)));

// Dynamic gulpfile loading
glob.sync('gulpfile.*.ts', { cwd: import.meta.dirname })
  .forEach(f => require(`./${f}`));
```

**Variations:** Tasks are defined in separate files (`gulpfile.vscode.ts`, `gulpfile.vscode.linux.ts`, `gulpfile.extensions.ts`) and loaded dynamically. The task system uses callback/promise/stream variants.

---

#### Pattern: TypeScript Compilation with Mangling and Source Maps
**Where:** `build/lib/compilation.ts:59-150`
**What:** Custom TypeScript compiler pipeline (`tsb`) with property mangling, source map generation, NLS (Natural Language Support) extraction, and optional esbuild transpilation. The build distinguishes between development (no mangling, inline source maps) and production (mangling, external source maps) configurations.

```typescript
// compilation.ts - Compilation pipeline
export function createCompile(src: string, { build, emitError, transpileOnly, preserveEnglish, noEmit }: ICompileTaskOptions) {
  const projectPath = path.join(import.meta.dirname, '../../', src, 'tsconfig.json');
  const overrideOptions = { 
    ...getTypeScriptCompilerOptions(src), 
    inlineSources: Boolean(build)
  };
  if (!build) {
    overrideOptions.inlineSourceMap = true;
  }

  const compilation = tsb.create(projectPath, overrideOptions, {
    verbose: false,
    transpileOnly: Boolean(transpileOnly),
    transpileWithEsbuild: typeof transpileOnly !== 'boolean' && transpileOnly.esbuild
  }, err => reporter(err));

  function pipeline(token?: util.ICancellationToken) {
    const input = es.through();
    const output = input
      .pipe(util.$if(!build && isRuntimeJs, util.appendOwnPathSourceURL()))
      .pipe(tsFilter)
      .pipe(util.loadSourcemaps())
      .pipe(compilation(token))
      .pipe(util.$if(build, nls.nls({ preserveEnglish })))
      .pipe(util.$if(!transpileOnly, sourcemaps.write('.', {
        addComment: false,
        includeContent: !!build,
        sourceRoot: overrideOptions.sourceRoot
      })));
    return es.duplex(input, output);
  }
  return pipeline;
}

export function compileTask(src: string, out: string, build: boolean, options: { disableMangle?: boolean } = {}): task.StreamTask {
  const task = () => {
    const compile = createCompile(src, { build, emitError: true, transpileOnly: false });
    const srcPipe = gulp.src(`${src}/**`, { base: `${src}` });
    
    // mangle: TypeScript to TypeScript
    let mangleStream = es.through();
    if (build && !options.disableMangle) {
      let ts2tsMangler = new Mangler(compile.projectPath, console.log, { 
        mangleExports: true, 
        manglePrivateFields: true 
      });
      const newContentsByFileName = ts2tsMangler.computeNewFileContents(new Set(['saveState']));
      mangleStream = es.through(async function write(data: File) {
        const tsNormalPath = ts.normalizePath(data.path);
        const newContents = (await newContentsByFileName).get(tsNormalPath);
        if (newContents !== undefined) {
          data.contents = Buffer.from(newContents.out);
        }
      });
    }
    
    return srcPipe.pipe(compile()).pipe(mangleStream).pipe(gulp.dest(out));
  };
  task.taskName = `compile-${path.basename(src)}`;
  return task;
}
```

**Variations:** Development path skips mangling; build path applies mangling and external source maps. EsBuild transpilation is optional.

---

#### Pattern: Electron Application Packaging and Bundling
**Where:** `build/gulpfile.vscode.ts:239-435`
**What:** Platform-agnostic package task that bundles VS Code into an Electron app. Handles compression with ASAR (node_modules.asar), file filtering, resource inclusion, platform-specific assets, and metadata injection (version, commit, checksums).

```typescript
// gulpfile.vscode.ts - Core packaging pattern
function packageTask(platform: string, arch: string, sourceFolderName: string, destinationFolderName: string) {
  const destination = path.join(path.dirname(root), destinationFolderName);
  
  const task = () => {
    const out = sourceFolderName;
    const versionedResourcesFolder = util.getVersionedResourcesFolder(platform, commit!);
    
    // Compute checksums for integrity verification
    const checksums = computeChecksums(out, [
      'vs/base/parts/sandbox/electron-browser/preload.js',
      'vs/workbench/workbench.desktop.main.js',
      'vs/workbench/workbench.desktop.main.css',
      // ... more files
    ]);
    
    // Source code
    const src = gulp.src(out + '/**', { base: '.' })
      .pipe(rename(function (path) { 
        path.dirname = path.dirname!.replace(new RegExp('^' + out), 'out'); 
      }))
      .pipe(util.setExecutableBit(['**/*.sh']));
    
    // Built-in extensions
    const extensions = gulp.src(['.build/extensions/**', ...platformSpecificBuiltInExtensionsExclusions], { base: '.build', dot: true });
    
    // Production dependencies (node_modules)
    const depFilterPattern = ['**', `!**/${config.version}/**`, '!**/package-lock.json'];
    if (stripSourceMapsInPackagingTasks) {
      depFilterPattern.push('!**/*.{js,css}.map');
    }
    
    const deps = gulp.src(dependenciesSrc, { base: '.', dot: true })
      .pipe(filter(depFilterPattern))
      .pipe(util.cleanNodeModules(path.join(import.meta.dirname, '.moduleignore')))
      .pipe(util.cleanNodeModules(path.join(import.meta.dirname, `.moduleignore.${process.platform}`)))
      .pipe(createAsar(path.join(process.cwd(), 'node_modules'), [
        '**/*.node',
        '**/@vscode/ripgrep/bin/*',
        '**/@github/copilot-*/**',
        '**/node-pty/build/Release/*',
        '**/*.wasm',
        '**/@vscode/vsce-sign/bin/*',
      ], ['**/*.mk'], ['node_modules/vsda/**'], 'node_modules.asar'));
    
    // Merge product metadata
    const productJsonStream = gulp.src(['product.json'], { base: '.' })
      .pipe(jsonEditor((json: Record<string, unknown>) => {
        json.commit = commit;
        json.date = readISODate(out);
        json.checksums = checksums;
        json.version = version;
        return json;
      }));
    
    let all = es.merge(src, extensions, productJsonStream, deps);
    
    // Platform-specific resources
    if (platform === 'win32') {
      all = es.merge(all, gulp.src(['resources/win32/*.ico', 'resources/win32/*.png'], { base: '.' }));
    } else if (platform === 'darwin') {
      const shortcut = gulp.src('resources/darwin/bin/code.sh')
        .pipe(replace('@@APPNAME@@', product.applicationName));
      all = es.merge(all, shortcut);
    }
    
    // Electron configuration
    const electronConfig = {
      ...config,
      platform,
      arch: arch === 'armhf' ? 'arm' : arch,
      ffmpegChromium: false
    };
    
    let result: NodeJS.ReadWriteStream = all
      .pipe(util.skipDirectories())
      .pipe(util.fixWin32DirectoryPermissions())
      .pipe(electron(electronConfig))
      .pipe(filter(['**', '!LICENSE', '!version'], { dot: true }));
    
    return result.pipe(vfs.dest(destination));
  };
  return task;
}
```

**Variations:** Platform-specific resource handling (Windows .ico files, Darwin .app structure, Linux desktop files); optional source map stripping for production.

---

#### Pattern: Electron Configuration and Signing
**Where:** `build/lib/electron.ts:104-211`
**What:** Centralized Electron configuration object with macOS-specific document types, code signing credentials, and update mechanisms. Configured from product.json with version management and repository integration.

```typescript
// electron.ts - Electron configuration
const { electronVersion, msBuildId } = util.getElectronVersion();

export const config = {
  version: electronVersion,
  tag: product.electronRepository ? `v${electronVersion}-${msBuildId}` : undefined,
  productAppName: product.nameLong,
  companyName: 'Microsoft Corporation',
  copyright: 'Copyright (C) 2026 Microsoft. All rights reserved',
  darwinExecutable: product.nameShort,
  darwinIcon: 'resources/darwin/code.icns',
  darwinBundleIdentifier: product.darwinBundleIdentifier,
  darwinApplicationCategoryType: 'public.app-category.developer-tools',
  darwinBundleDocumentTypes: [
    ...darwinBundleDocumentTypes({ 'C header file': 'h', 'C source code': 'c' }, 'c'),
    darwinBundleDocumentType(['bat', 'cmd'], 'bat', 'Windows command script'),
    // ... 40+ document type definitions
  ],
  darwinBundleURLTypes: [{
    role: 'Viewer',
    name: product.nameLong,
    urlSchemes: [product.urlProtocol]
  }],
  linuxExecutableName: product.applicationName,
  winIcon: 'resources/win32/code.ico',
  token: process.env['GITHUB_TOKEN'],
  repo: product.electronRepository || undefined,
  validateChecksum: true,
  checksumFile: path.join(root, 'build', 'checksums', 'electron.txt'),
  createVersionedResources: useVersionedUpdate,
  productVersionString: versionedResourcesFolder,
};

function getElectron(arch: string): () => NodeJS.ReadWriteStream {
  return () => {
    const electronOpts = {
      ...config,
      platform: process.platform,
      arch: arch === 'armhf' ? 'arm' : arch,
      ffmpegChromium: false,
      keepDefaultApp: true
    };
    return vfs.src('package.json')
      .pipe(json({ name: product.nameShort }))
      .pipe(electron(electronOpts))
      .pipe(filter(['**', '!**/app/package.json']))
      .pipe(vfs.dest('.build/electron'));
  };
}
```

**Variations:** Includes 40+ Darwin document type definitions for file associations. Environment variables configure signing credentials and repositories.

---

#### Pattern: Platform-Specific Code Signing (macOS)
**Where:** `build/darwin/sign.ts:34-135`
**What:** macOS code signing with entitlements configuration per Electron helper process type (GPU, Renderer, Plugin). Includes keychain error retry logic and plist mutation for privacy descriptions.

```typescript
// darwin/sign.ts - Code signing pattern
function getEntitlementsForFile(filePath: string): string {
  if (filePath.includes(' Helper (GPU).app')) {
    return path.join(baseDir, 'azure-pipelines', 'darwin', 'helper-gpu-entitlements.plist');
  } else if (filePath.includes(' Helper (Renderer).app')) {
    return path.join(baseDir, 'azure-pipelines', 'darwin', 'helper-renderer-entitlements.plist');
  } else if (filePath.includes(' Helper (Plugin).app')) {
    return path.join(baseDir, 'azure-pipelines', 'darwin', 'helper-plugin-entitlements.plist');
  } else if (filePath.includes(' Helper.app')) {
    return path.join(baseDir, 'azure-pipelines', 'darwin', 'helper-entitlements.plist');
  }
  return path.join(baseDir, 'azure-pipelines', 'darwin', 'app-entitlements.plist');
}

async function retrySignOnKeychainError<T>(fn: () => Promise<T>, maxRetries: number = 3): Promise<T> {
  let lastError: Error | undefined;
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      return await fn();
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      const isKeychainError = errorMessage.includes('The specified item could not be found in the keychain.');
      if (!isKeychainError || attempt === maxRetries) {
        throw error;
      }
      const delay = 1000 * Math.pow(2, attempt - 1);
      await new Promise(resolve => setTimeout(resolve, delay));
    }
  }
  throw lastError;
}

async function main(buildDir?: string): Promise<void> {
  const tempDir = process.env['AGENT_TEMPDIRECTORY'];
  const arch = process.env['VSCODE_ARCH'];
  const identity = process.env['CODESIGN_IDENTITY'];
  
  const appOpts: SignOptions = {
    app: path.join(appRoot, appName),
    platform: 'darwin',
    optionsForFile: (filePath) => ({
      entitlements: getEntitlementsForFile(filePath),
      hardenedRuntime: true,
    }),
    keychain: path.join(tempDir, 'buildagent.keychain'),
    version: getElectronVersion(),
    identity,
  };
  
  // Update plist with privacy descriptions
  await spawn('plutil', ['-insert', 'NSAppleEventsUsageDescription', '-string', 
    'An application in Visual Studio Code wants to use AppleScript.', infoPlistPath]);
  
  await retrySignOnKeychainError(() => sign(appOpts));
}
```

**Variations:** Different entitlements per process type; plutil mutations for privacy descriptions; keychain retry with exponential backoff.

---

#### Pattern: Windows Installer Build with InnoSetup
**Where:** `build/gulpfile.vscode.win32.ts:28-127`
**What:** InnoSetup script generation with dynamic variable substitution. Includes definitions for architectures (x64, arm64), installation targets (system/user), quality levels (stable/insider), and APPX manifest generation.

```typescript
// gulpfile.vscode.win32.ts - Windows setup pattern
function packageInnoSetup(iss: string, options: { definitions?: Record<string, unknown> }, cb: (err?: Error | null) => void) {
  const definitions = options.definitions || {};
  if (process.argv.some(arg => arg === '--debug-inno')) {
    definitions['Debug'] = 'true';
  }
  if (process.argv.some(arg => arg === '--sign')) {
    definitions['Sign'] = 'true';
  }

  const keys = Object.keys(definitions);
  const defs = keys.map(key => `/d${key}=${definitions[key]}`);
  const args = [
    iss,
    ...defs,
    `/sesrp=node ${signWin32Path} $f`  // Signing callback
  ];

  cp.spawn(innoSetupPath, args, { stdio: ['ignore', 'inherit', 'inherit'] })
    .on('error', cb)
    .on('exit', code => {
      if (code === 0) { cb(null); } 
      else { cb(new Error(`InnoSetup returned exit code: ${code}`)); }
    });
}

function buildWin32Setup(arch: string, target: string): task.CallbackTask {
  return (cb) => {
    const definitions: Record<string, unknown> = {
      NameLong: product.nameLong,
      NameShort: product.nameShort,
      DirName: product.win32DirName,
      Version: pkg.version,
      RawVersion: pkg.version.replace(/-\w+$/, ''),
      Commit: commit,
      NameVersion: product.win32NameVersion + (target === 'user' ? ' (User)' : ''),
      ExeBasename: product.nameShort,
      RegValueName: product.win32RegValueName,
      Arch: arch,
      AppId: { 'x64': x64AppId, 'arm64': arm64AppId }[arch],
      AppxPackage: `${quality === 'stable' ? 'code' : 'code_insider'}_${arch}.appx`,
      AppxPackageName: `${product.win32AppUserModelId}`,
      SourceDir: sourcePath,
      OutputDir: outputPath,
      Quality: quality
    };
    packageInnoSetup(issPath, { definitions }, cb);
  };
}

// Define tasks for all combinations
defineWin32SetupTasks('x64', 'system');
defineWin32SetupTasks('arm64', 'system');
defineWin32SetupTasks('x64', 'user');
defineWin32SetupTasks('arm64', 'user');
```

**Variations:** Separate tasks for x64/arm64 and system/user installation targets; dynamic APPX package generation for Store submission.

---

#### Pattern: Linux Package Generation (DEB/RPM)
**Where:** `build/gulpfile.vscode.linux.ts:38-120`
**What:** Dynamic dependency resolution and control file generation for Debian packages. Aggregates desktop files, appdata, icons, shell completions, and binary assets with calculated installed size metrics.

```typescript
// gulpfile.vscode.linux.ts - Linux packaging pattern
function prepareDebPackage(arch: string) {
  const binaryDir = '../VSCode-linux-' + arch;
  const debArch = getDebPackageArch(arch);
  const destination = '.build/linux/deb/' + debArch + '/' + product.applicationName + '-' + debArch;

  return async function () {
    const dependencies = await getDependencies('deb', binaryDir, product.applicationName, debArch);
    
    const desktop = gulp.src('resources/linux/code.desktop', { base: '.' })
      .pipe(rename('usr/share/applications/' + product.applicationName + '.desktop'));
    
    const appdata = gulp.src('resources/linux/code.appdata.xml', { base: '.' })
      .pipe(replace('@@NAME_LONG@@', product.nameLong))
      .pipe(rename('usr/share/appdata/' + product.applicationName + '.appdata.xml'));
    
    const icon = gulp.src('resources/linux/code.png', { base: '.' })
      .pipe(rename('usr/share/pixmaps/' + product.linuxIconName + '.png'));
    
    const bash_completion = gulp.src('resources/completions/bash/code')
      .pipe(replace('@@APPNAME@@', product.applicationName))
      .pipe(rename('usr/share/bash-completion/completions/' + product.applicationName));
    
    const code = gulp.src(binaryDir + '/**/*', { base: binaryDir })
      .pipe(rename(function (p) { p.dirname = 'usr/share/' + product.applicationName + '/' + p.dirname; }));
    
    // Calculate package size
    let size = 0;
    const control = code.pipe(es.through(
      function (f) { size += f.isDirectory() ? 4096 : f.contents.length; },
      function () {
        gulp.src('resources/linux/debian/control.template', { base: '.' })
          .pipe(replace('@@NAME@@', product.applicationName))
          .pipe(replace('@@VERSION@@', packageJson.version + '-' + linuxPackageRevision))
          .pipe(replace('@@ARCHITECTURE@@', debArch))
          .pipe(replace('@@DEPENDS@@', dependencies.join(', ')))
          .pipe(replace('@@INSTALLEDSIZE@@', Math.ceil(size / 1024).toString()))
          .pipe(rename('DEBIAN/control'))
          .pipe(es.through(function (f) { this.emit('data', f); }));
      }));
    
    const all = es.merge(control, desktop, appdata, icon, bash_completion, code);
    return all.pipe(vfs.dest(destination));
  };
}
```

**Variations:** Architecture-specific builds (amd64, armhf, arm64); dependency list resolution; calculated file sizes for control metadata.

---

#### Pattern: ASAR Archive Creation for Native Modules
**Where:** `build/lib/asar.ts:13-100`
**What:** Custom ASAR archive builder using chromium-pickle-js that selectively unpacks native modules (.node, .wasm, binaries) while keeping JavaScript archived. Supports duplicate file tracking for vsda module.

```typescript
// asar.ts - ASAR bundling pattern
export function createAsar(folderPath: string, unpackGlobs: string[], skipGlobs: string[], duplicateGlobs: string[], destFilename: string): NodeJS.ReadWriteStream {

  const shouldUnpackFile = (file: VinylFile): boolean => {
    for (let i = 0; i < unpackGlobs.length; i++) {
      if (minimatch(file.relative, unpackGlobs[i])) {
        return true;
      }
    }
    return false;
  };

  const shouldSkipFile = (file: VinylFile): boolean => {
    for (const skipGlob of skipGlobs) {
      if (minimatch(file.relative, skipGlob)) {
        return true;
      }
    }
    return false;
  };

  // Used in gulpfile.vscode.ts
  return createAsar(path.join(process.cwd(), 'node_modules'), [
    '**/*.node',                          // Native binaries
    '**/@vscode/ripgrep/bin/*',          // Ripgrep native
    '**/@github/copilot-*/**',           // Copilot native modules
    '**/node-pty/build/Release/*',       // Terminal native
    '**/*.wasm',                         // WebAssembly
    '**/@vscode/vsce-sign/bin/*',        // Code signing tool
  ], [
    '**/*.mk'                            // Skip makefiles
  ], [
    'node_modules/vsda/**'               // Duplicate for compatibility
  ], 'node_modules.asar');
```

**Variations:** Whitelist unpacking for known native modules; skip list for non-essential files; duplicate tracking for compatibility.

---

#### Pattern: ESBuild-based Bundling (Newer Approach)
**Where:** `build/lib/esbuild.ts:13-66`
**What:** Modern drop-in replacement for older gulp-based bundling. Uses esbuild for transpilation and bundling with optional minification, mangling, NLS extraction, and source map generation.

```typescript
// esbuild.ts - Modern bundling pattern
export function runEsbuildTranspile(outDir: string, excludeTests: boolean): Promise<void> {
  return new Promise((resolve, reject) => {
    const scriptPath = path.join(root, 'build/next/index.ts');
    const args = [scriptPath, 'transpile', '--out', outDir];
    if (excludeTests) {
      args.push('--exclude-tests');
    }

    const proc = cp.spawn(process.execPath, args, {
      cwd: root,
      stdio: 'inherit'
    });

    proc.on('error', reject);
    proc.on('close', code => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`esbuild transpile failed with exit code ${code}`));
      }
    });
  });
}

export function runEsbuildBundle(outDir: string, minify: boolean, nls: boolean, target: 'desktop' | 'server' | 'server-web' = 'desktop', sourceMapBaseUrl?: string): Promise<void> {
  return new Promise((resolve, reject) => {
    const scriptPath = path.join(root, 'build/next/index.ts');
    const args = [scriptPath, 'bundle', '--out', outDir, '--target', target];
    if (minify) {
      args.push('--minify');
      args.push('--mangle-privates');
    }
    if (nls) {
      args.push('--nls');
    }
    if (sourceMapBaseUrl) {
      args.push('--source-map-base-url', sourceMapBaseUrl);
    }

    const proc = cp.spawn(process.execPath, args, {
      cwd: root,
      stdio: 'inherit'
    });

    proc.on('error', reject);
    proc.on('close', code => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`esbuild bundle failed with exit code ${code}`));
      }
    });
  });
}

// Usage in gulpfile.vscode.ts
task.parallel(
  task.define('esbuild-vscode-min', () => runEsbuildBundle('out-vscode-min', true, true, 'desktop', `${sourceMappingURLBase}/core`)),
  task.define('esbuild-vscode-reh-min', () => runEsbuildBundle('out-vscode-reh-min', true, true, 'server', `${sourceMappingURLBase}/core`)),
  task.define('esbuild-vscode-reh-web-min', () => runEsbuildBundle('out-vscode-reh-web-min', true, true, 'server-web', `${sourceMappingURLBase}/core`)),
)
```

**Variations:** Spawn child process running separate esbuild orchestrator; supports transpile-only vs full bundle; configurable targets (desktop, server, server-web).

---

## Architectural Implications for Tauri/Rust Migration

**Compilation & Bundling**: The TypeScript compilation chain and esbuild bundling are TypeScript/Node tooling but are independent of Electron. These can largely remain unchanged, though Cargo would become the primary build orchestrator instead of Gulp.

**Electron-Specific Components Requiring Replacement**:
1. `@vscode/gulp-electron` - Electron packaging (12 files using this plugin)
2. `@electron/osx-sign` - macOS code signing (darwin/sign.ts)
3. InnoSetup integration - Windows installer (gulpfile.vscode.win32.ts)
4. ASAR bundling - Electron asset compression (lib/asar.ts, lib/util.ts)
5. Electron preload/IPC infrastructure - Native window and module loading
6. Platform-specific app packaging (DMG, DEB, RPM)

**Tauri Integration Points**:
- Tauri provides Cargo-based build system (replaces Gulp)
- Tauri handles cross-platform app packaging and signing
- Tauri manages IPC between frontend and Rust backend
- Tauri supports code signing natively (Windows, macOS, Linux)
- Tauri generates installers (MSI for Windows, DMG for macOS, AppImage/Deb for Linux)

**Preserved Infrastructure**:
- TypeScript/JavaScript compilation pipeline
- ESBuild bundling and minification
- NLS extraction and localization system
- Extension compilation and bundling
- Source map generation and CDN delivery
- Version management and checksums

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
