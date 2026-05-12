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

