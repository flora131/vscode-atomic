### Files Analysed

1. `/Users/norinlavaee/vscode-atomic/gulpfile.mjs` (6 LOC)
2. `/Users/norinlavaee/vscode-atomic/build/gulpfile.ts` (60 LOC)
3. `/Users/norinlavaee/vscode-atomic/build/gulpfile.editor.ts` (300 LOC)
4. `/Users/norinlavaee/vscode-atomic/build/gulpfile.extensions.ts` (342 LOC)
5. `/Users/norinlavaee/vscode-atomic/build/gulpfile.compile.ts` (28 LOC)
6. `/Users/norinlavaee/vscode-atomic/build/gulpfile.vscode.ts` (lines 1-62, task-list scan)
7. `/Users/norinlavaee/vscode-atomic/build/lib/compilation.ts` (464 LOC)
8. `/Users/norinlavaee/vscode-atomic/build/lib/task.ts` (121 LOC)
9. `/Users/norinlavaee/vscode-atomic/build/lib/util.ts` (476 LOC)
10. `/Users/norinlavaee/vscode-atomic/build/lib/tsgo.ts` (85 LOC)

---

### Per-File Notes

#### `gulpfile.mjs` (`/Users/norinlavaee/vscode-atomic/gulpfile.mjs:5`)
A one-line ESM bootstrap. The entire file is a single `import './build/gulpfile.ts'` statement. Its sole purpose is to allow Node/Gulp (launched from the repo root) to enter the TypeScript build world via native ESM `import`. No logic lives here; every task definition is delegated downstream. The `.mjs` extension signals to Node that this is ES-module format so that native `.ts` imports (handled by a tsx/ts-node loader) can be used without a CommonJS wrapper.

---

#### `build/gulpfile.ts` (`/Users/norinlavaee/vscode-atomic/build/gulpfile.ts`)

**Role:** Main Gulp task orchestrator. Wires together tasks from three imported modules and also auto-discovers additional platform/packaging gulpfiles at startup.

**Key mechanics:**

- **EventEmitter ceiling** (`line 6`): Sets `EventEmitter.defaultMaxListeners = 100` to suppress spurious MaxListeners warnings caused by the large number of gulp-stream pipelines.
- **Static imports** (`lines 11-15`): Imports named exports from `./gulpfile.editor.ts`, `./gulpfile.extensions.ts`, `./lib/compilation.ts`, `./lib/task.ts`, and `./lib/util.ts`.
- **API/extension-point name generation** (`lines 18, 23-24`): Registers `compilation.compileExtensionPointNamesTask` and `compilation.compileApiProposalNamesTask` / `watchApiProposalNamesTask` as top-level Gulp tasks. These scan `src/vscode-dts/**` and `src/vs/workbench/**/*.ts` to code-generate files committed to the repo.
- **Transpile-only client task** (`line 27`, `transpile-client-esbuild`): Runs `util.rimraf('out')` then `compilation.transpileTask('src', 'out', true)`. The `true` flag activates esbuild for fast transpile-only output (no type checking).
- **`transpile-client`** (`line 31`): Same pipeline without esbuild — uses SWC/tsb transpiler instead.
- **`compile-client`** (`line 35`): Full type-checked compile: cleans `out`, copies codicons, generates API proposals and extension-point names, then calls `compilation.compileTask('src', 'out', false)`.
- **`watch-client`** (`line 38`): Parallel watcher that runs `watchTypeCheckTask`, `watchApiProposalNamesTask`, `watchExtensionPointNamesTask`, and `watchCodiconsTask` simultaneously.
- **`compile`** (`line 42`): Top-level parallel composite: `monacoTypecheckTask` (TSC type-check of Monaco API surface), `compileClientTask`, `compileExtensionsTask`, and `compileExtensionMediaTask`.
- **`watch`** (`line 45`): Parallel of `watchClientTask` + `watchExtensionsTask` (Monaco watch is commented out).
- **`default`** (`line 48`): Aliased to `_compileTask` (same as `compile`).
- **Unhandled rejection handler** (`lines 50-53`): Logs and calls `process.exit(1)` on any unhandled Promise rejection, preventing silent failures in async gulp tasks.
- **Dynamic gulpfile discovery** (`lines 56-59`): Uses `glob.sync('gulpfile.*.ts', { cwd: import.meta.dirname })` to find every file matching `build/gulpfile.*.ts` and `require()`s each one. This lazily loads platform-specific packaging tasks (e.g., `gulpfile.vscode.ts`, `gulpfile.vscode.linux.ts`, `gulpfile.vscode.win32.ts`, `gulpfile.reh.ts`, `gulpfile.vscode.web.ts`, `gulpfile.hygiene.ts`, `gulpfile.scan.ts`, `gulpfile.cli.ts`) only when the user actually invokes a task that needs them — the `require` at module evaluation time means they are loaded eagerly on any `gulp` invocation, but the comment "only if running tasks other than the editor tasks" in the source is aspirational rather than conditional.

---

#### `build/gulpfile.editor.ts` (`/Users/norinlavaee/vscode-atomic/build/gulpfile.editor.ts`)

**Role:** Monaco Editor extraction and distribution build tasks; also exports the `monacoTypecheckTask` used in the composite `compile` task in `gulpfile.ts`.

**Key tasks:**

- **`extract-editor-src`** (`line 38`): Copies `codicon.ttf`, reads the Monaco API usage recipe (`build/monaco/monaco.usage.recipe`), and calls `standalone.extractEditor()` with entry points `vs/editor/editor.main.ts`, `vs/editor/editor.worker.start.ts`, and `vs/editor/common/services/editorWebWorkerMain.ts`. Uses `shakeLevel: 2` (class-member-level tree shaking) and outputs to `out-editor-src/`.
- **`compile-editor-esm`** (`line 72`): Compiles `out-editor-src` → `out-monaco-editor-core/esm` using `compilation.createCompile` with `build: true, emitError: true, transpileOnly: false, preserveEnglish: true`. Processes NLS files via `i18n.processNlsFiles`.
- **`final-editor-resources`** (`line 132`): Assembles the NPM-publishable `out-monaco-editor-core/` package: copies LICENSE, ThirdPartyNotices.txt, generates `editor.api.d.ts` (via `toExternalDTS` which strips `declare namespace monaco {` wrappers), injects `marked` and `dompurify` versions read from `cgmanifest.json` files into `package.json`, stamps `version.txt` with the git SHA, and renames `README-npm.md` to `README.md`.
- **`editor-distro`** (`line 215`): Sequence: clean both output dirs → `extract-editor-src` → `compile-editor-esm` → `final-editor-resources`.
- **`monacodts`** (`line 227`): Calls `monacoapi.execute()` and writes the results to `src/vs/monaco.d.ts` and `src/vs/editor/common/standalone/standaloneEnums.ts`.
- **Monaco type checking** (`lines 236-285`): `createTscCompileTask(watch)` spawns `node ./node_modules/.bin/tsc -p ./src/tsconfig.monaco.json --noEmit` as a child process. It strips ANSI codes from stdout, parses tsc error lines with a regex (`/(.*\(\d+,\d+\): )(.*: )(.*)/`), and pipes them through a `createReporter('monaco')` instance. Exports `monacoTypecheckTask` (no watch) and `monacoTypecheckWatchTask` (with `-w`).

---

#### `build/gulpfile.extensions.ts` (`/Users/norinlavaee/vscode-atomic/build/gulpfile.extensions.ts`)

**Role:** Compiles and packages all built-in VS Code extensions (both for development and CI builds).

**Static extension list** (`lines 56-101`): A hardcoded array of 40+ `tsconfig.json` paths under `extensions/` and `.vscode/extensions/`. This list avoids a `glob.sync` on startup (saves ~250ms per Gulp invocation per the comment at line 51).

**Per-extension task factory** (`lines 117-236`): For each `tsconfig.json` path, the code computes `srcBase`, `out`, and `baseUrl`, then creates three tasks:

- `transpile-extension:<name>`: clean → `tsb.create` transpile pipeline (esbuild, `transpileOnly: true`) → write to `out/`.
- `compile-extension:<name>` (`line 194`): clean → parallel: copy non-TS files via Vinyl + `spawnTsgo(absolutePath, ...)` for TypeScript compilation. Uses `tsgo` (the Go-based TypeScript compiler) rather than `tsb`.
- `watch-extension:<name>` (`line 202`): clean → merge of: initial non-TS copy, a file-watcher that copies changed non-TS files, and a debounced `createTsgoStream` invocation (200ms debounce). Emits aggregate "Starting compilation" / "Finished compilation" log messages via `onExtensionCompilationStart/End` to match VS Code's problem-matcher pattern in `tasks.json`.

**Aggregate tasks** (`lines 238-245`): `transpile-extensions`, `compile-extensions` (exported as `compileExtensionsTask`), `watch-extensions` (exported as `watchExtensionsTask`) — each fans out in parallel across all per-extension tasks.

**Extension media** (`lines 248-257`): `compile-extension-media` and `watch-extension-media` delegate to `ext.buildExtensionMedia(isWatch)`.

**CI/Azure Pipelines build tasks** (`lines 260-308`):
- `clean-extensions-build`: `util.rimraf('.build/extensions')`.
- `compile-non-native-extensions-build`: bundles marketplace extensions then packages non-native local extensions into `.build/`.
- `compile-native-extensions-build`: packages native local extensions.
- `compile-copilot-extension-build`: packages built-in Copilot extension.
- `compile-extensions-build` (`compileAllExtensionsBuildTask`): clean → marketplace bundle → all-local-extensions package.

**Web extensions** (`lines 311-341`): `compile-web` / `watch-web` tasks glob for `esbuild.browser.mts` config files under `extensions/` and call `ext.esbuildExtensions` plus `ext.typeCheckExtension` in parallel for each found config.

---

#### `build/gulpfile.compile.ts` (`/Users/norinlavaee/vscode-atomic/build/gulpfile.compile.ts`)

**Role:** Defines the two variants of the main source-tree compile used in CI/builds:
- `compile-build-without-mangling` (`line 22`): `copyCodiconsTask` → `rimraf('out-build')` → `date.writeISODate('out-build')` → `compileApiProposalNamesTask` → `compilation.compileTask('src', 'out-build', true, { disableMangle: true })`. Used in PR builds.
- `compile-build-with-mangling` (`line 26`): Same sequence with `disableMangle: false`. Used in production CI. Exports both for use by `gulpfile.vscode.ts`.

---

#### `build/lib/compilation.ts` (`/Users/norinlavaee/vscode-atomic/build/lib/compilation.ts`)

**Role:** Core TypeScript compilation pipeline factory and code-generation utilities.

**`createCompile(src, options)`** (`line 59`): Creates a `tsb` compilation handle from `src/tsconfig.json`. Returns a `pipeline` function and `pipeline.tsProjectSrc()`. The pipeline is a Vinyl stream that: filters `.ts` files, loads sourcemaps, calls `compilation(token)` (the tsb compiler), optionally applies NLS extraction (`nls.nls({preserveEnglish})`), and writes `.js.map` files.

**`transpileTask(src, out, esbuild?)`** (`line 109`): Wraps `createCompile` with `transpileOnly: { esbuild }` and `build: false`. Reads the entire `src/**` glob and pipes through the transpile-only pipeline.

**`compileTask(src, out, build, options)`** (`line 125`): Full compile. Enforces 4GB RAM minimum at line 129. When `build && !disableMangle`, instantiates a `Mangler` (`build/lib/mangle/index.ts`) with `mangleExports: true, manglePrivateFields: true`. The mangle stream intercepts each Vinyl file, looks up the mangled content from `computeNewFileContents`, replaces `data.contents` and `data.sourceMap`, then continues the pipeline.

**`watchTypeCheckTask(src)`** (`line 174`): Creates a watch on `src/**`, debounces at 200ms, then calls `createTsgoStream(projectPath, { taskName: 'watch-client-noEmit', noEmit: true })` — meaning type-check-only (no emit) using the Go-based `tsgo` compiler.

**`MonacoGenerator`** (`line 199`): A class that wraps `monacodts.run3()` to regenerate `src/vs/monaco.d.ts` and `standaloneEnums.ts`. In watch mode it uses `fs.watchFile` on all declaration files read during generation, and on `RECIPE_PATH`. Changes trigger a 20ms debounced re-execution. In non-watch (build) mode, a stale `monaco.d.ts` causes the stream to emit an error halting the build.

**Code-generation tasks:**
- `compileApiProposalNamesTask` (`line 364`): Reads `src/vscode-dts/**`, extracts proposal name + version via regex from each file, sorts them, generates `src/vs/platform/extensions/common/extensionsApiProposals.ts`, and skips writing if content is unchanged.
- `compileExtensionPointNamesTask` (`line 407`): Reads `src/vs/workbench/**/*.ts`, uses TypeScript's AST parser (`ts.createSourceFile`) + `extractExtensionPointNamesFromFile` to collect all `registerExtensionPoint` call names, sorts and JSON-serializes them into `src/vs/workbench/services/extensions/common/extensionPoints.json`.
- `copyCodiconsTask` (`line 452`): File-copies `node_modules/@vscode/codicons/dist/codicon.ttf` to `src/vs/base/browser/ui/codicons/codicon/codicon.ttf`.

---

#### `build/lib/task.ts` (`/Users/norinlavaee/vscode-atomic/build/lib/task.ts`)

**Role:** A thin custom task runner that wraps Gulp's task model with typed TypeScript interfaces and predictable Promise-based execution.

**Types** (`lines 9-24`): `PromiseTask`, `StreamTask`, `CallbackTask` union as `Task`. Each extends `BaseTask` (optional `displayName`, `taskName`, `_tasks` for composite detection).

**`_doExecute(task)`** (`line 48`): Determines the task type by checking `task.length === 1` (callback), then the return value: `undefined` (sync), a Promise (async), or a stream (end/error events). Wraps all in `new Promise(...)`.

**`series(...tasks)`** (`line 82`): Sequential — loops `await _execute(task[i])` for each task. Returns a `PromiseTask` with `_tasks` set.

**`parallel(...tasks)`** (`line 92`): Concurrent — `await Promise.all(tasks.map(t => _execute(t)))`.

**`define(name, task)`** (`line 100`): Assigns `taskName`/`displayName` to a task or its last inner task (for composite tasks). If the last subtask is itself composite, generates a fake no-op series to ensure `taskName` is on a leaf function.

---

#### `build/lib/util.ts` (`/Users/norinlavaee/vscode-atomic/build/lib/util.ts`)

**Role:** Shared Vinyl/stream utility functions used across all gulpfiles.

**Notable exports:**
- `rimraf(dir)` (`line 296`): Wraps the `rimraf` npm package in a retrying Promise (up to 5 retries on `ENOTEMPTY`). Returns a function tagged with `taskName = clean-<basename>`.
- `incremental(streamProvider, initial, supportsCancellation?)` (`line 33`): Implements a stateful incremental build stream: buffers incoming files when a compile is already running; debounces 500ms before re-running. Supports an optional `ICancellationToken` that signals cancellation when the buffer is non-empty.
- `debounce(task, duration)` (`line 82`): Simpler debounce for watch tasks — runs immediately on first invocation, then debounces subsequent triggers while the task is running (marks state as `stale`).
- `loadSourcemaps()` (`line 193`): Inline sourcemap parser — strips `//# sourceMappingURL=` from file contents and reads the referenced `.map` file, attaching it to the Vinyl file object as `f.sourceMap`.
- `$if(test, onTrue, onFalse)` (`line 257`): Conditional stream routing via `ternary-stream`.
- `getElectronVersion()` (`line 377`): Reads `.npmrc` in repo root, extracts `target=` (Electron version) and `ms_build_id=` via regex. Used by packaging tasks.
- `untar()` (`line 441`): Converts a `.tar` Buffer in a Vinyl stream into individual Vinyl files by using `tar.Parser`.
- `VinylStat` (`line 390`): A class implementing `fs.Stats` with all-zero defaults, used when creating synthetic Vinyl files from tar entries.

---

#### `build/lib/tsgo.ts` (`/Users/norinlavaee/vscode-atomic/build/lib/tsgo.ts`)

**Role:** Wrapper around `tsgo` — the experimental Go-based TypeScript compiler (`@typescript/tsgo`) — used for faster compilation of extensions and type-checking the client source.

**`spawnTsgo(projectPath, config, onComplete?)`** (`line 17`): Spawns `npx tsgo --project <path> --pretty false --incremental [--noEmit | --sourceMap --inlineSources]` as a child process (shell: true, cross-platform npx). Collects all stdout+stderr, strips ANSI escape codes and timestamps, filters out "Starting compilation" / "File change detected" / "Compilation complete" noise lines. On exit code 0, calls optional `onComplete()` then resolves. Non-zero exit rejects with an error.

**`createTsgoStream(projectPath, config, onComplete?)`** (`line 74`): Wraps `spawnTsgo` in a pass-through `event-stream`. Resolves → emits `end`; rejects → emits `error`. Returns the stream immediately so it can be piped, while the underlying process runs asynchronously.

---

### Cross-Cutting Synthesis

The VS Code build system is structured as a **multi-layered Gulp pipeline** with explicit TypeScript typing throughout. The layers are:

1. **Bootstrap** (`gulpfile.mjs`): one-line ESM re-export.
2. **Task orchestration** (`build/gulpfile.ts`): assembles the developer-facing tasks (`compile`, `watch`, `transpile-client*`) from composites defined elsewhere; auto-discovers packaging gulpfiles at startup via `glob.sync`.
3. **Functional sub-orchestrators** (`gulpfile.editor.ts`, `gulpfile.extensions.ts`, `gulpfile.compile.ts`): each owns a domain (Monaco editor NPM package, built-in extensions, CI build compile).
4. **Platform packaging** (dynamically required `gulpfile.vscode.ts`, `gulpfile.vscode.linux.ts`, `gulpfile.vscode.win32.ts`, `gulpfile.reh.ts`, `gulpfile.vscode.web.ts`): assembles Electron app artifacts per platform. `gulpfile.vscode.ts` specifically imports `@vscode/gulp-electron` and `./lib/electron.ts` for the Electron download/packaging step.
5. **Core libraries** (`build/lib/`): `task.ts` (typed task runner), `compilation.ts` (TypeScript pipeline factory + code generators), `util.ts` (Vinyl/stream utilities), `tsgo.ts` (Go-compiler wrapper), plus `mangle/`, `optimize/`, `asar/`, `i18n/`, `nls/`, `extensions/`, `standalone/` subdirectories (not read in this partition but referenced throughout).

**Two TypeScript compilers coexist**: the traditional `tsb` (TypeScript compiler wrapped in a Vinyl stream, from `build/lib/tsb/index.ts`) and the newer `tsgo` (Go-based `@typescript/tsgo` binary). `tsb` is used for transpile-only fast paths and the Monaco editor; `tsgo` is used for extension compilation and watch-mode type checking.

**Code generation is build-time**: `extensionsApiProposals.ts` and `extensionPoints.json` are generated from source scans and committed to the repo. `monaco.d.ts` is also generated and verified on each compile. These generated files must be kept current; the build will error if `monaco.d.ts` is stale during a non-watch compile.

**The `_tasks` property is the composite marker**: `task.define` detects composite tasks by checking `task._tasks`. This allows `series` and `parallel` wrappers to propagate `taskName` to the leaf function that Gulp actually calls, rather than to the anonymous wrapper.

---

### Out-of-Partition References

The following files are outside partition 79 but are directly imported and central to understanding the build:

- `/Users/norinlavaee/vscode-atomic/build/lib/mangle/index.ts` — `Mangler` class that performs TypeScript-to-TypeScript property/export name mangling during production builds. Imported in `compilation.ts:19`.
- `/Users/norinlavaee/vscode-atomic/build/lib/tsb/index.ts` — Classic TypeScript compiler Vinyl-stream adapter (`tsb.create`). Used in both `compilation.ts:24` and `gulpfile.extensions.ts:24`.
- `/Users/norinlavaee/vscode-atomic/build/lib/optimize.ts` — Bundle optimization (likely Rollup/esbuild bundling for production output). Imported in `gulpfile.vscode.ts:21`.
- `/Users/norinlavaee/vscode-atomic/build/lib/standalone.ts` — `standalone.extractEditor()` — Monaco standalone editor source extraction logic. Imported in `gulpfile.editor.ts`.
- `/Users/norinlavaee/vscode-atomic/build/lib/monaco-api.ts` — `monacoapi.execute()` / `run3()` / `DeclarationResolver` — generates the public Monaco TypeScript API surface. Imported by both `compilation.ts` and `gulpfile.editor.ts`.
- `/Users/norinlavaee/vscode-atomic/build/buildfile.ts` — Defines `vscodeEntryPoints` (worker entry points, workbench, code). Used in `gulpfile.vscode.ts:20`.
- `/Users/norinlavaee/vscode-atomic/build/lib/extensions.ts` — `ext.buildExtensionMedia`, `ext.packageMarketplaceExtensionsStream`, `ext.packageNativeLocalExtensionsStream`, `ext.esbuildExtensions`, etc. The actual packaging logic for all extension build tasks. Imported in `gulpfile.extensions.ts:20`.
- `/Users/norinlavaee/vscode-atomic/build/lib/electron.ts` — `config` object (Electron download/packaging config). Imported in `gulpfile.vscode.ts:28`.
- `/Users/norinlavaee/vscode-atomic/build/lib/nls.ts` — NLS string extraction and injection pipeline step. Imported in `compilation.ts`.
- `/Users/norinlavaee/vscode-atomic/build/lib/i18n.ts` — `i18n.processNlsFiles`, `defaultLanguages`, `extraLanguages`. Used during Monaco ESM compile in `gulpfile.editor.ts:83`.
- `/Users/norinlavaee/vscode-atomic/build/lib/asar.ts` — ASAR archive creation utility. Imported in `gulpfile.vscode.ts:29`.
- `/Users/norinlavaee/vscode-atomic/build/lib/inlineMeta.ts` — Inlines `product.json` metadata into build artifacts. Imported in `gulpfile.vscode.ts:22`.
- `/Users/norinlavaee/vscode-atomic/build/lib/watch/index.ts` — Vinyl-based file watcher used throughout watch tasks.
- `/Users/norinlavaee/vscode-atomic/product.json` — Read directly by `gulpfile.vscode.ts` (CDN URL, application name, platform packaging config).
- `/Users/norinlavaee/vscode-atomic/.npmrc` — Parsed by `util.getElectronVersion()` to extract the pinned Electron version for download/packaging.

---

**Summary.** The `gulpfile.mjs` / `build/gulpfile.ts` pair forms the entire entry point and top-level task registry for VS Code's TypeScript/Node.js build pipeline. `gulpfile.mjs` is a trivial ESM redirect; `build/gulpfile.ts` registers roughly a dozen developer-facing tasks by composing imported task objects from `gulpfile.editor.ts`, `gulpfile.extensions.ts`, and `lib/compilation.ts`, then dynamically `require`s all other `gulpfile.*.ts` files (found by glob) to load platform packaging and CI tasks. Compilation is handled by two co-existing TypeScript compilers: the traditional `tsb` Vinyl-stream adapter (for transpile-only and Monaco paths) and the newer Go-based `tsgo` binary (for extension compiles and watch-mode type checking). Code-generation for API proposals, extension point names, and `monaco.d.ts` is baked into the compile task graph and produces committed source files that must stay synchronized with the main source tree. Porting this build system to Tauri/Rust would require replacing the entire Gulp/Node.js pipeline with a Rust build tool (e.g., `cargo build` scripts or `xtask`), rewriting the TypeScript-to-JS transpilation step (replacing `tsb`/`tsgo`), eliminating all Electron-specific packaging (the `@vscode/gulp-electron` download, ASAR creation, rcedit PE patching), and replacing sourcemap/NLS/i18n processing with Rust-native equivalents — a ground-up rewrite of the build layer with no reusable components from the existing Gulp infrastructure.
