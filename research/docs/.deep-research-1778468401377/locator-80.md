# Gulp Build System Architecture - File Locator Report

## Implementation

### Entry Point
- `gulpfile.mjs` - 5 LOC re-export shim that imports `./build/gulpfile.ts` (the actual build entry)

### Primary Gulpfile
- `build/gulpfile.ts` - Main gulp orchestrator (63 LOC) that:
  - Imports EventEmitter and sets defaultMaxListeners to 100
  - Re-exports specific gulp tasks: monacoTypecheckTask, compileExtensionMediaTask, compileExtensionsTask, watchExtensionsTask
  - Defines core compilation pipeline tasks: transpile-client-esbuild, transpile-client, compile-client, watch-client, compile, watch
  - Dynamically loads all `gulpfile.*.ts` modules from `build/` directory via glob pattern matching

### Gulp Task Modules (12 files)
- `build/gulpfile.editor.ts` - Monaco editor compilation and typecheck tasks
- `build/gulpfile.extensions.ts` - Extension compilation and media processing
- `build/gulpfile.cli.ts` - CLI tool building
- `build/gulpfile.vscode.ts` - Main VS Code application build orchestration
- `build/gulpfile.vscode.web.ts` - Web variant build tasks
- `build/gulpfile.vscode.linux.ts` - Linux platform-specific build
- `build/gulpfile.vscode.win32.ts` - Windows platform-specific build
- `build/gulpfile.reh.ts` - Remote Execution Host build tasks
- `build/gulpfile.compile.ts` - Compilation pipeline utilities
- `build/gulpfile.hygiene.ts` - Code hygiene and linting tasks
- `build/gulpfile.scan.ts` - Static code scanning tasks
- `build/gulpfile.extensions.ts` - Extension handling

### Build Support Libraries (163 TypeScript files across build/lib/)
Core build system modules:
- `build/lib/task.ts` - Gulp task definition and composition utilities
- `build/lib/util.ts` - Common build utilities (rimraf, etc.)
- `build/lib/compilation.ts` - Compilation pipeline orchestration for src→out transpilation
- `build/lib/esbuild.ts` - ESBuild transpiler integration and configuration
- `build/lib/extensions.ts` - Extension discovery and compilation
- `build/lib/optimize.ts` - Code optimization and mangling
- `build/lib/bundle.ts` - Module bundling utilities
- `build/lib/treeshaking.ts` - Tree-shaking implementation
- `build/lib/mangle/` - Code name mangling (3 files: index.ts, staticLanguageServiceHost.ts, renameWorker.ts)

### TypeScript Build Infrastructure
- `build/lib/tsb/` - TypeScript compilation builder (4 files: index.ts, builder.ts, transpiler.ts, utils.ts)
- `build/lib/typeScriptLanguageServiceHost.ts` - Language service for compilation
- `build/lib/tsconfigUtils.ts` - TypeScript config processing
- `build/lib/tsgo.ts` - TypeScript Go transpiler wrapper

### Additional Build Modules (spanning 163 total files)
- `build/lib/nls.ts` - National Language Support (i18n) processing
- `build/lib/standalone.ts` - Standalone build artifacts
- `build/lib/policies/` - Policy generation (8 files for boolean/string/number/object/enum policies)
- `build/lib/watch/` - File watching implementation (2 files: watch-win32.ts, index.ts)
- `build/lib/stylelint/` - Style linting (2 files: validateVariableNames.ts, vscode-known-variables.json)

## Configuration

### Build Configuration
- `build/buildConfig.ts` - Master build configuration settings
- `build/buildfile.ts` - Build file processing
- `build/tsconfig.json` - TypeScript configuration for build system (ES2024, strict mode, noEmit)

### Build Package Metadata
- `build/package.json` - Build subsystem dependencies including:
  - gulp@^4.0.0 as core build runner
  - esbuild@0.27.2 for fast transpilation
  - @types/gulp and related gulp type definitions
  - ESLint, Stylelint type definitions
  - Build utility types (@types/rimraf, @types/glob, @types/node)
  - Electron build tools (@electron/get, @electron/osx-sign)

- `build/package-lock.json` - Locked dependency versions for build system

### Platform-Specific Build Configuration
- `build/rspack/rspack.serve-out.config.mts` - Rspack development server configuration
- `build/vite/vite.config.ts` - Vite bundler configuration for alternative builds

## Types / Interfaces

### Build Typing Definitions (build/lib/typings/)
- `build/lib/typings/gulp-*.d.ts` - Type definitions for gulp plugins:
  - gulp-buffer.d.ts
  - gulp-bom.d.ts
  - gulp-gunzip.d.ts
  - gulp-vinyl-zip.d.ts
  - event-stream.d.ts
  - ternary-stream.d.ts
  - vscode-gulp-watch.d.ts
  - stream.d.ts

- `build/lib/typings/asar.d.ts` - ASAR archive packaging types
- `build/lib/typings/rcedit.d.ts` - RC file editor types
- `build/lib/typings/chromium-pickle-js.d.ts` - Chromium binary format types
- `build/lib/typings/@vscode/gulp-electron.d.ts` - VS Code Electron gulp plugin types
- `build/lib/typings/cgmanifest.json` - Component governance manifest schema

## Tests

### Build System Tests
- `build/lib/test/checkCyclicDependencies.test.ts` - Validates no circular dependency chains
- `build/lib/test/booleanPolicy.test.ts` - Tests boolean policy code generation
- `build/lib/test/stringPolicy.test.ts` - Tests string policy generation
- `build/lib/test/numberPolicy.test.ts` - Tests numeric policy generation
- `build/lib/test/objectPolicy.test.ts` - Tests object policy generation
- `build/lib/test/stringEnumPolicy.test.ts` - Tests enum policy generation
- `build/lib/test/policyConversion.test.ts` - Tests policy conversion logic
- `build/lib/test/render.test.ts` - Tests policy rendering
- `build/lib/test/i18n.test.ts` - Tests internationalization processing

### Test Fixtures
- `build/lib/test/fixtures/policies/` - Policy test fixtures for Win32, Darwin, Linux platforms (ADMX, ADML, JSON, plist formats)

### Next-Gen Build Tests
- `build/next/test/private-to-property.test.ts` - Tests property transformation
- `build/next/test/nls-sourcemap.test.ts` - Tests source map generation for i18n

## Documentation

### Policy Generation Metadata
- `build/lib/policies/policyData.jsonc` - JSONC policy data definitions
- `build/lib/i18n.resources.json` - i18n resource mappings
- `build/lib/stylelint/vscode-known-variables.json` - CSS variable reference catalog

### Build System Scripts (package.json npm scripts in root)
- `npm run compile` → `npm run gulp compile`
- `npm run gulp` → node with --experimental-strip-types flag running gulp/bin/gulp.js
- `npm run transpile-client` → Node build/next/index.ts transpile
- `npm run watch-client` → npm run gulp watch-client
- `npm run watch-extensions` → npm run gulp watch-extensions

## Notable Clusters

### Compilation Pipeline
The build system organizes around several key clusters:

1. **Task Orchestration (build/gulpfile.ts + gulpfile.mjs)**
   - Single entry point (gulpfile.mjs) that re-exports main gulp config
   - Main gulpfile defines core compilation tasks and dynamically loads platform/feature-specific tasks
   - 12 feature-specific gulp modules loaded at runtime

2. **Build Infrastructure (build/lib/, 163 TypeScript files)**
   - Core compilation utilities: compilation.ts, esbuild.ts, tsb/ directory (TypeScript builder)
   - Code transformation: mangle/, treeshaking.ts, optimize.ts
   - i18n/localization: nls.ts, i18n.ts
   - Utilities: task.ts, util.ts, extensions.ts, bundle.ts
   - CI/CD integration: azure-pipelines/ directory with 25+ build scripts
   - Platform build tools: darwin/, win32/ signing and packaging

3. **ESBuild Integration**
   - `build/lib/esbuild.ts` handles fast transpilation
   - Configured via npm scripts as `transpile-client-esbuild` task
   - Alternative to slower TypeScript compilation

4. **Next-Gen Build System (build/next/)**
   - 5 TypeScript files providing modern build infrastructure
   - Includes private-to-property transformation and NLS plugin
   - Appears to be successor to traditional gulp pipeline for transpilation

5. **TypeScript Compilation Subsystem (build/lib/tsb/)**
   - Custom TypeScript builder implementation (4 files)
   - Wrapped by tsgo.ts for command-line integration
   - Type checking via TypeScriptLanguageServiceHost

6. **Platform-Specific Build Logic**
   - gulpfile.vscode.linux.ts, gulpfile.vscode.win32.ts (native builds)
   - gulpfile.vscode.web.ts, gulpfile.vscode.ts, gulpfile.reh.ts (variants)
   - Isolated platform concerns in separate gulp modules

7. **Policy Code Generation (build/lib/policies/, 8 files)**
   - Generates policy enforcement code from JSONC definitions
   - Outputs Win32 (ADMX/ADML), macOS (plist), Linux (JSON) formats
   - Full test coverage with fixtures for all platforms

8. **Electron/Native Build Integration (build/darwin/, build/win32/)**
   - Platform-specific code signing (darwin/codesign.ts, win32/codesign.ts)
   - DMG creation (darwin/create-dmg.ts)
   - Universal app bundling (darwin/create-universal-app.ts)
   - Binary verification (darwin/verify-macho.ts)

The gulpfile.mjs shim serves as the minimal re-export entry point that delegates all actual task definition and orchestration to build/gulpfile.ts, which then composes task definitions from 12 feature-specific gulp modules and coordinates with 163 supporting TypeScript build utilities across build/lib/.
