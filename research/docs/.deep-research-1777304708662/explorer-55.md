# Partition 55 of 79 — Findings

## Scope
`src/bootstrap-esm.ts/` (1 files, 112 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# ESM Loader Bootstrap Location Mapping

## Research Question
Port VS Code core IDE functionality from TypeScript/Electron to Tauri/Rust.

## Scope
- `src/bootstrap-esm.ts` (112 LOC) — ESM loader bootstrap with dynamic module loading patterns

---

## Implementation

### Core Bootstrap Files
- **`src/bootstrap-esm.ts`** — Main ESM bootstrap module (112 LOC)
  - Installs Node.js module resolution hook for 'fs' redirection in Electron environments
  - Registers base64-encoded dynamic import loader via `node:module.register()`
  - Handles NLS (National Language Support) configuration and localization loading
  - Sets up global variables: `_VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON`, `_VSCODE_FILE_ROOT`
  - Implements `setupNLS()` and `doSetupNLS()` with performance marks
  - **Key Dynamic Import Pattern**: `register(data:text/javascript;base64,..., import.meta.url)`

### Bootstrap Ecosystem
- **`src/bootstrap-meta.ts`** (56 LOC) — Metadata initialization
  - Loads product.json and package.json using `createRequire(import.meta.url)`
  - Handles embedded app product overrides
  - Handles development mode overrides from product.overrides.json
  - Exports: `product`, `pkg` objects

- **`src/bootstrap-node.ts`** (191 LOC) — Node.js runtime configuration
  - Sets up current working directory (Windows-specific handling)
  - Exports module loader injection via `Module.register('./bootstrap-import.js')`
  - Implements global module lookup path removal
  - Provides portable mode configuration
  - Exports: `devInjectNodeModuleLookupPath()`, `removeGlobalNodeJsModuleLookupPaths()`, `configurePortable()`

- **`src/bootstrap-import.ts`** (102 LOC) — ESM import redirection hook
  - Implements `initialize()` function to populate module mappings from package.json
  - Handles conditional exports resolution (exports["."].import vs exports["."].default)
  - Implements `resolve()` hook for Node.js module resolution
  - Maps package specifiers to file URLs with format detection (.mjs → ESM, .cjs → CommonJS)
  - **Hook Pattern**: Resolves specifier → URL mapping with module format determination

- **`src/bootstrap-fork.ts`** (229 LOC) — Forked process bootstrap
  - Pipes logging from worker processes to parent with `process.send()`
  - Handles uncaught exceptions and unhandled promise rejections
  - Parent-child process health monitoring via `VSCODE_PARENT_PID`
  - Crash reporter integration for Electron processes
  - Calls `await bootstrapESM()` before loading entrypoint
  - **Dynamic Import**: `await import([`./${process.env['VSCODE_ESM_ENTRYPOINT']}.js`].join('/'))`

- **`src/bootstrap-cli.ts`** (12 LOC) — CLI bootstrap initialization
  - Deletes `VSCODE_CWD` environment variable early to prevent parent shell escape

- **`src/bootstrap-server.ts`** (8 LOC) — Server bootstrap initialization
  - Deletes `ELECTRON_RUN_AS_NODE` to prevent fs redirection in server context

### Entry Point Integration
- **`src/main.ts`** — Electron main process
  - Imports and calls `await bootstrapESM()` (line 211)
  - Sets up portable mode via `configurePortable(product)`
  - Precedes main app bundle loading

- **`src/cli.ts`** — Command-line interface process
  - Imports bootstrap-cli.js first (order critical)
  - Sets `VSCODE_NLS_CONFIG` environment variable for NLS discovery
  - Calls `await bootstrapESM()` (line 23)

- **`src/server-cli.ts`** — Server CLI process
  - Sets `VSCODE_NLS_CONFIG` for NLS discovery
  - Calls `await bootstrapESM()` (dynamic import pattern)

- **`src/server-main.ts`** — Server main process
  - Sets `VSCODE_NLS_CONFIG` before NLS resolution
  - Calls `await bootstrapESM()` before main server startup

---

## Types / Interfaces

### NLS Configuration
- **`src/vs/nls.ts`** (245 LOC)
  - **Interface `INLSConfiguration`** — Runtime NLS settings
    - `userLocale: string` — User-specified locale
    - `osLocale: string` — OS system locale
    - `resolvedLanguage: string` — Final resolved UI language
    - `languagePack?: INLSLanguagePackConfiguration` — Optional language pack
    - `defaultMessagesFile: string` — English fallback messages
    - Deprecated compatibility fields for vscode-nls module support
  
  - **Interface `INLSLanguagePackConfiguration`**
    - `translationsConfigFile: string`
    - `messagesFile: string`
    - `corruptMarkerFile: string`
  
  - **Exports**:
    - `localize()` — Localization function (overloads for key/message and ILocalizeInfo)
    - `localize2()` — Returns ILocalizedString with original and value
    - `getNLSMessages()` — Retrieves global NLS messages array
    - `getNLSLanguage()` — Retrieves resolved language

### Global Type Declarations
- **`src/typings/vscode-globals-product.d.ts`** (48 LOC)
  - Declares global scope variables:
    - `var _VSCODE_FILE_ROOT: string` — File root for resource resolution
    - `var _VSCODE_PRODUCT_JSON: Record<string, any>` — Product configuration
    - `var _VSCODE_PACKAGE_JSON: Record<string, any>` — Package metadata
    - `var _VSCODE_CSS_LOAD: (module: string) => void` — CSS loader
    - `var _VSCODE_DISABLE_CSS_IMPORT_MAP?: boolean` — CSS import map override
    - `var _VSCODE_USE_RELATIVE_IMPORTS?: boolean` — Development relative imports

### Performance Monitoring
- **`src/vs/base/common/performance.ts`** (80+ LOC)
  - `mark(name: string, markOptions?: { startTime?: number })` — Add performance mark
  - `getMarks()` — Retrieve recorded marks
  - `clearMarks(name?: string)` — Clear marks
  - **Marks Used in bootstrap-esm.ts**:
    - `'code/willLoadNls'` — Start of NLS loading
    - `'code/didLoadNls'` — End of NLS loading

### Environment and Platform
- **`src/vs/base/common/platform.ts`** — Platform detection
  - `INodeProcess` interface for Node.js-specific properties
  - Referenced by bootstrap-node.ts for process introspection

---

## Configuration

### Environment Variables
Used throughout bootstrap chain:

- **Module Resolution**:
  - `VSCODE_DEV` — Development mode (skips NLS, enables overrides)
  - `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH` — Injects module lookup redirection
  - `ELECTRON_RUN_AS_NODE` — Electron-as-Node.js mode (triggers fs → original-fs mapping)

- **NLS Configuration**:
  - `VSCODE_NLS_CONFIG` — JSON-stringified INLSConfiguration (set by CLI/server entry points)

- **Process Management**:
  - `VSCODE_PARENT_PID` — Parent process ID for fork health checks
  - `VSCODE_ESM_ENTRYPOINT` — Dynamic entrypoint in forked processes
  - `VSCODE_PIPE_LOGGING` — Enable parent logging in forks
  - `VSCODE_HANDLES_UNCAUGHT_ERRORS` — Skip exception handling in forks
  - `VSCODE_CRASH_REPORTER_PROCESS_TYPE` — Crash reporter type
  - `VSCODE_VERBOSE_LOGGING` — Verbose fork logging

- **Portable Mode**:
  - `VSCODE_PORTABLE` — Portable data directory
  - `VSCODE_CWD` — Current working directory (set by bootstrap-node)

- **Other**:
  - `VSCODE_HANDLES_SIGPIPE` — SIGPIPE handler presence flag

---

## Examples / Fixtures

### Dynamic Import Registration Pattern
```typescript
// From bootstrap-esm.ts (lines 15-29)
const jsCode = `
export async function resolve(specifier, context, nextResolve) {
  if (specifier === 'fs') {
    return {
      format: 'builtin',
      shortCircuit: true,
      url: 'node:original-fs'
    };
  }
  return nextResolve(specifier, context);
}`;
register(`data:text/javascript;base64,${Buffer.from(jsCode).toString('base64')}`, import.meta.url);
```

### NLS Message File Loading
```typescript
// From bootstrap-esm.ts (lines 78, 94)
globalThis._VSCODE_NLS_MESSAGES = JSON.parse(
  (await fs.promises.readFile(messagesFile)).toString()
);
```

### Module Mapping Resolution
```typescript
// From bootstrap-import.ts (lines 28-77)
// Reads package.json exports, determines entry point:
// Preference: exports["."].import → exports["."].default → main → index.js
// Format detection: .mjs (ESM), .cjs (CommonJS), type field (module/commonjs)
```

### Fork Entrypoint Loading
```typescript
// From bootstrap-fork.ts (line 229)
await import([`./${process.env['VSCODE_ESM_ENTRYPOINT']}.js`].join('/'));
```

---

## Documentation

### Code Comments
- **bootstrap-esm.ts**: "Install a hook to module resolution to map 'fs' to 'original-fs'" — explains Electron fs redirection
- **bootstrap-import.ts**: "SEE https://nodejs.org/docs/latest/api/module.html#initialize" — Node.js Module Initialize Hook API reference
- **bootstrap-import.ts**: "Determine the entry point: prefer exports["."].import for ESM..." — complex export resolution logic
- **bootstrap-node.ts**: "We need this to redirect to node_modules from the remote-folder" — use case for import redirection
- **bootstrap-node.ts**: "SIGPIPE workaround for Electron" — cross-platform compatibility note

### Type Comments
- `vscode-globals-product.d.ts`: Documentation for each global variable purpose and deprecation status
- `nls.ts`: Inline JSDoc for localize/localize2 with examples and parameter descriptions

---

## Notable Clusters

### Bootstrap Initialization Chain
```
bootstrap-cli.ts (delete VSCODE_CWD)
    ↓
bootstrap-node.ts (setup CWD, portable mode, module lookup removal)
    ↓
bootstrap-esm.ts (register module hooks, setup NLS)
    ↓
[Main application entry point]
```

### Module Resolution Hook Chain
```
Entry Point Import → bootstrap-esm.ts register() 
    ↓
Data URI Base64 Loader (fs → original-fs)
    ↓
bootstrap-import.js resolve() hook (package.json exports resolution)
    ↓
Node.js default resolver
```

### NLS Loading Workflow
```
CLI/Server Entry Point (set VSCODE_NLS_CONFIG)
    ↓
bootstrapESM() → setupNLS()
    ↓
Read VSCODE_NLS_CONFIG environment variable
    ↓
Load messagesFile (if not dev mode)
    ↓
Set globalThis._VSCODE_NLS_MESSAGES & _VSCODE_NLS_LANGUAGE
    ↓
Application code uses getNLSMessages(), localize()
```

### Fork Process Bootstrap
```
bootstrap-fork.ts (called first)
    ↓
Setup logging pipes, exception handlers, parent monitoring
    ↓
Call bootstrapESM()
    ↓
Dynamic import: VSCODE_ESM_ENTRYPOINT
```

### Global Variable Population
```
bootstrap-esm.ts sets:
  - globalThis._VSCODE_PRODUCT_JSON (from bootstrap-meta.ts)
  - globalThis._VSCODE_PACKAGE_JSON (from bootstrap-meta.ts)
  - globalThis._VSCODE_FILE_ROOT (import.meta.dirname)
  - globalThis._VSCODE_NLS_MESSAGES (from file system)
  - globalThis._VSCODE_NLS_LANGUAGE (from config)

Used by:
  - src/vs/nls.ts (getNLSMessages, getNLSLanguage)
  - src/vs/platform/product/common/product.ts (product discovery)
  - src/vs/amdX.ts (module loading)
  - Web workers (via webWorkerServiceImpl.ts)
```

---

## Port Considerations for Tauri/Rust

The ESM loader bootstrap serves critical functions for VS Code's runtime:

1. **Module Resolution Hook**: Intercepts `import('fs')` to redirect to `original-fs` in Electron contexts. Tauri would need equivalent Rust-side file system APIs.

2. **Dynamic Import Registration**: Uses Node.js `module.register()` with base64-encoded inline modules. Tauri would require a module loading strategy compatible with WebAssembly or native Rust bindings.

3. **NLS Configuration Chain**: Coordinates localization through environment variables and global state. Tauri could use app state management or IPC for equivalent functionality.

4. **Performance Tracking**: Uses `performance.mark()` for instrumentation. Tauri can use web APIs or create equivalent Rust-side timing.

5. **Global Context Setup**: Sets up `globalThis` variables for product metadata and localization. Tauri would need to inject equivalent state into the renderer or use IPC.

6. **Process Bootstrap Patterns**: Different entry points (main, CLI, server, fork) require different initialization. A Tauri port would need variant handling for each context.

7. **Portable Mode Detection**: File system checks for portable data directories. Tauri's `app_root` and path resolution APIs can replicate this.

The bootstrap-esm pattern is fundamentally tied to Node.js ECMAScript module semantics and Electron's runtime. A Rust/Tauri rewrite would require abstracting these concerns into platform-specific initialization modules.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `/Users/norinlavaee/vscode-atomic/src/bootstrap-esm.ts` (112 LOC) — primary subject
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-meta.ts` (55 LOC) — imported for `product` and `pkg` exports
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts` (191 LOC) — imported as side-effect
- `/Users/norinlavaee/vscode-atomic/src/vs/base/common/performance.ts` (150 LOC) — imported for `mark()`
- `/Users/norinlavaee/vscode-atomic/src/vs/nls.ts` (lines 155–230 inspected) — source of `INLSConfiguration` type
- `/Users/norinlavaee/vscode-atomic/src/typings/vscode-globals-product.d.ts` — declares `globalThis._VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON`, `_VSCODE_FILE_ROOT`
- `/Users/norinlavaee/vscode-atomic/src/typings/vscode-globals-nls.d.ts` — declares `globalThis._VSCODE_NLS_MESSAGES`, `_VSCODE_NLS_LANGUAGE`

---

### Per-File Notes

#### `src/bootstrap-esm.ts`

**Imports and static side-effects (lines 6–10)**

The file opens with four imports:
- `node:fs` (line 6) — used only inside `doSetupNLS` for `fs.promises.readFile` and `fs.promises.writeFile`.
- `register` from `node:module` (line 7) — the ESM loader registration API; used once in the Electron guard block (line 29).
- `{ product, pkg }` from `./bootstrap-meta.js` (line 8) — evaluated eagerly when the module is first imported; `product` is a `Partial<IProductConfiguration>` object and `pkg` is a raw package.json object.
- `'./bootstrap-node.js'` (line 9) — imported purely for its side-effects (sets `Error.stackTraceLimit`, handles `SIGPIPE`, calls `setupCurrentWorkingDirectory()`).
- `* as performance` from `./vs/base/common/performance.js` (line 10) — the `mark()` function is the only symbol consumed.
- `INLSConfiguration` from `./vs/nls.js` (line 11) — a type-only import used as the return type annotation of `setupNLS` and `doSetupNLS`.

**Electron `fs` → `original-fs` hook (lines 14–30)**

The guard condition at line 14 checks `process.env['ELECTRON_RUN_AS_NODE']` or `process.versions['electron']`. When either is truthy the code constructs an inline ESM loader as a string literal (`jsCode`, lines 15–28). The loader exports a single `resolve` hook: when `specifier === 'fs'` it returns a synthetic resolution record pointing to `node:original-fs` and sets `shortCircuit: true` to stop further hook traversal; for all other specifiers it delegates to `nextResolve`. The loader is registered at line 29 via:

```
register(`data:text/javascript;base64,${Buffer.from(jsCode).toString('base64')}`, import.meta.url)
```

`Buffer.from(jsCode).toString('base64')` encodes the loader source as Base64; passing a `data:` URI as the module specifier is the documented way to supply inline ESM loader code to `node:module.register()`. The second argument (`import.meta.url`) provides the parent URL context for relative resolution within the loader. This entire block runs synchronously at module evaluation time, before any await.

**Global initialization (lines 33–35)**

Three globals are set on `globalThis` immediately after the Electron hook block:
- `globalThis._VSCODE_PRODUCT_JSON` (line 33) — a shallow copy (`{ ...product }`) of the object imported from `bootstrap-meta.ts`.
- `globalThis._VSCODE_PACKAGE_JSON` (line 34) — a shallow copy of `pkg`.
- `globalThis._VSCODE_FILE_ROOT` (line 35) — assigned `import.meta.dirname`, which is the directory path of the compiled `bootstrap-esm.js` file at runtime.

These three assignments are synchronous and occur before `bootstrapESM()` is ever called by any caller.

**NLS lazy singleton (`setupNLS` / `doSetupNLS`, lines 39–104)**

`setupNLSResult` (line 39) is a module-level variable of type `Promise<INLSConfiguration | undefined> | undefined`. It implements a lazy singleton: the first call to `setupNLS()` (lines 41–47) invokes `doSetupNLS()` and stores the returned promise; subsequent calls return the same cached promise.

`doSetupNLS()` (lines 49–104) is `async` and follows this sequence:

1. **Performance mark** — `performance.mark('code/willLoadNls')` is called synchronously at line 50. This inserts a timestamp entry into VS Code's shared `MonacoPerformanceMarks` store on `globalThis`.

2. **Config parsing** (lines 55–68) — If `process.env['VSCODE_NLS_CONFIG']` is defined, the string is parsed with `JSON.parse` into `nlsConfig: INLSConfiguration | undefined`. Two properties of `nlsConfig` are inspected to select a messages file path, in priority order:
   - `nlsConfig.languagePack.messagesFile` (line 59) — points to a compiled language pack cache file.
   - `nlsConfig.defaultMessagesFile` (line 61) — fallback to the bundled English messages file.
   After determining `messagesFile`, `globalThis._VSCODE_NLS_LANGUAGE` is set at line 64 to `nlsConfig.resolvedLanguage` (the BCP-47 language tag string, e.g. `'de'`, `'pt-br'`).

3. **Early return guard** (lines 71–75) — If `process.env['VSCODE_DEV']` is set (development mode) or `messagesFile` is undefined, the function returns `undefined` immediately, skipping file I/O entirely. `_VSCODE_NLS_MESSAGES` is never written in this branch, meaning built-in English strings remain in the source.

4. **Primary messages file load** (line 78) — `fs.promises.readFile(messagesFile)` is awaited. The buffer is converted to string via `.toString()` and parsed with `JSON.parse`. The result (a flat `string[]`) is assigned to `globalThis._VSCODE_NLS_MESSAGES`.

5. **Error handling for corrupt language pack** (lines 80–98) — If the primary read fails:
   - If `nlsConfig.languagePack.corruptMarkerFile` is defined (line 83), the file path is written with the content `'corrupted'` via `fs.promises.writeFile` (line 85). This marker file signals to the VS Code main process on the next startup to invalidate and regenerate the language pack cache.
   - If `nlsConfig.defaultMessagesFile` differs from the already-failed `messagesFile` (line 92), a second `readFile` attempt is made against `defaultMessagesFile` to load the English fallback messages into `_VSCODE_NLS_MESSAGES`.

6. **Closing performance mark** — `performance.mark('code/didLoadNls')` at line 101 records the end timestamp of the entire NLS loading phase.

7. **Return** — The function returns `nlsConfig` at line 103, which is either the parsed `INLSConfiguration` object or `undefined`.

**Exported entry point (`bootstrapESM`, lines 108–112)**

`bootstrapESM()` is the sole export from this module. It is `async` and its only action is `await setupNLS()` (line 111). It does not return the `INLSConfiguration` value; the resolved value is discarded. Callers use this function purely to guarantee that NLS initialization completes before application code runs. The five call sites are `src/main.ts:208`, `src/cli.ts:23`, `src/bootstrap-fork.ts:226`, `src/server-main.ts:251`, and `src/server-cli.ts:27`.

---

#### `src/bootstrap-meta.ts` (role as imported dependency)

Exports `product` and `pkg`. At lines 12–15 it detects whether the build has been patched (by checking the sentinel string `'BUILD_INSERT_PRODUCT_CONFIGURATION'`) and if not patched falls back to `require('../product.json')` using a CJS `require` created via `createRequire(import.meta.url)`. A similar pattern applies to `pkg` / `package.json` at lines 17–20. For embedded-app processes (`process.isEmbeddedApp`) it additionally loads `product.sub.json` and `package.sub.json` overlays (lines 22–44). In dev mode it also merges `product.overrides.json` (lines 46–52). The final exported values may therefore be merged composites of several JSON files.

---

#### `src/bootstrap-node.ts` (role as imported side-effect)

Imported at `bootstrap-esm.ts:9` with no binding, so all behavior is its top-level side-effects: sets `Error.stackTraceLimit = 100` (line 15), installs a SIGPIPE handler if `VSCODE_HANDLES_SIGPIPE` is not set (lines 17–30), and calls `setupCurrentWorkingDirectory()` (line 55) which stores `process.cwd()` in `VSCODE_CWD` and on Windows calls `process.chdir()`. It also exports `devInjectNodeModuleLookupPath`, `removeGlobalNodeJsModuleLookupPaths`, and `configurePortable` for other consumers, but these are not used within `bootstrap-esm.ts` itself.

---

#### `src/vs/base/common/performance.ts` (role as imported dependency)

Exports three functions — `mark`, `clearMarks`, `getMarks` — backed by a singleton `MonacoPerformanceMarks` object stored on `globalThis`. The implementation chooses among three backends at line 70 via `_define()`: native browser `performance`, a polyfill, or the Node.js branch (line 111) which bootstraps the polyfill using `performance?.timeOrigin` from Node's `perf_hooks`. `bootstrap-esm.ts` uses only `mark`, calling it twice with the names `'code/willLoadNls'` (line 50) and `'code/didLoadNls'` (line 101).

---

#### `src/vs/nls.ts` — `INLSConfiguration` (lines 179–230)

Defines the shape of the object parsed from `VSCODE_NLS_CONFIG`. Key fields consumed by `bootstrap-esm.ts`:
- `resolvedLanguage: string` — assigned to `globalThis._VSCODE_NLS_LANGUAGE` (line 64).
- `languagePack.messagesFile: string` — primary path for the translated messages JSON (line 59).
- `languagePack.corruptMarkerFile: string` — path written with `'corrupted'` on load failure (line 85).
- `defaultMessagesFile: string` — fallback English messages path (lines 61, 92, 94).

---

#### `src/typings/vscode-globals-product.d.ts` and `src/typings/vscode-globals-nls.d.ts`

These files add TypeScript declarations to the `globalThis` scope so that the five globals set in `bootstrap-esm.ts` are type-checked across the codebase. They carry the comment `// AMD2ESM migration relevant`, indicating they were introduced as part of the migration from AMD/Require.js to native ESM modules.

---

### Cross-Cutting Synthesis

`bootstrap-esm.ts` is the shared ESM initialization kernel that every VS Code process entry point (`main.ts`, `cli.ts`, `server-main.ts`, `server-cli.ts`, `bootstrap-fork.ts`) awaits before loading application code. It operates in three sequential layers: (1) synchronous module-evaluation-time setup — the Electron `fs` hook registration and the three `globalThis` product/path globals — which run simply by importing the module; (2) the lazy-once async NLS loader, invoked by `bootstrapESM()`, which parses an environment-provided JSON configuration to locate and read a translated messages file into `globalThis._VSCODE_NLS_MESSAGES`, bracketed by `performance.mark` calls for startup timing instrumentation; and (3) error-recovery logic that writes a corruption marker file and attempts an English fallback read, ensuring a usable `_VSCODE_NLS_MESSAGES` array is available even when the preferred language pack cache is corrupt. The `data:` URI trick for the ESM loader hook is the mechanism that allows Electron's `original-fs` (which bypasses the Electron virtual filesystem layer on ASAR archives) to be substituted transparently for the standard `fs` module in all downstream ESM imports. All five globals written here are consumed broadly across the VS Code source tree as the lowest-level contract between the bootstrap layer and application code.

---

### Out-of-Partition References

The following symbols and files are referenced from `src/bootstrap-esm.ts` but lie outside the analysed partition. Each would need to be traced for a complete port assessment:

| Symbol / File | Role in `bootstrap-esm.ts` | Defined At |
|---|---|---|
| `product`, `pkg` | Shallow-copied into `globalThis._VSCODE_PRODUCT_JSON` / `_VSCODE_PACKAGE_JSON` at lines 33–34 | `src/bootstrap-meta.ts` (exports) backed by `product.json` / `package.json` |
| `bootstrap-node.js` (side-effect import) | Runs process setup (cwd, SIGPIPE, stack limit) before NLS | `src/bootstrap-node.ts` |
| `performance.mark` | Records `'code/willLoadNls'` (line 50) and `'code/didLoadNls'` (line 101) | `src/vs/base/common/performance.ts:133` |
| `INLSConfiguration` | Return type of `setupNLS` / `doSetupNLS`; fields accessed at lines 58–64, 83, 92–94 | `src/vs/nls.ts:179` |
| `globalThis._VSCODE_PRODUCT_JSON` | Type declaration | `src/typings/vscode-globals-product.d.ts:24` |
| `globalThis._VSCODE_PACKAGE_JSON` | Type declaration | `src/typings/vscode-globals-product.d.ts:28` |
| `globalThis._VSCODE_FILE_ROOT` | Type declaration | `src/typings/vscode-globals-product.d.ts:13` |
| `globalThis._VSCODE_NLS_MESSAGES` | Type declaration; consumed by `nls.localize` / `nls.localize2` at build time | `src/typings/vscode-globals-nls.d.ts:30` |
| `globalThis._VSCODE_NLS_LANGUAGE` | Type declaration; consumed by NLS message lookup | `src/typings/vscode-globals-nls.d.ts:36` |
| `node:module` (`register`) | ESM loader hook registration; Tauri/Rust has no equivalent API | Node.js built-in |
| `node:fs` (`promises.readFile`, `promises.writeFile`) | Reads language pack JSON; writes corrupt marker | Node.js built-in |
| `process.env['VSCODE_NLS_CONFIG']` | Environment variable set by the Electron main process before forking renderer/utility | Set upstream in `src/main.ts` or platform services |
| `import.meta.dirname` | Sets `_VSCODE_FILE_ROOT`; ESM-only, unavailable in CJS or Rust | ESM runtime feature |
| `import.meta.url` | Parent URL context for `register()` call (line 29) | ESM runtime feature |

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Dynamic Import Patterns in VS Code

## Pattern Overview

This document catalogs dynamic import patterns found in VS Code's ESM bootstrap and module loading systems, relevant to understanding the current TypeScript/Electron architecture before porting to Tauri/Rust.

---

#### Pattern: Entry Point Dynamic Import

**Where:** `src/bootstrap-fork.ts:229`

**What:** Loads ESM entry points dynamically using environment variables with path string concatenation workaround.

```typescript
await import([`./${process.env['VSCODE_ESM_ENTRYPOINT']}.js`].join('/') /* workaround: esbuild prints some strange warnings when trying to inline? */);
```

Concatenates path fragments to avoid esbuild warnings during bundling. The environment variable `VSCODE_ESM_ENTRYPOINT` determines which module to load at runtime.

---

#### Pattern: Lazy Module Import with Try-Catch

**Where:** `src/vs/code/electron-main/app.ts:1701-1706`

**What:** Conditionally loads Windows-specific native bindings only when needed, wrapped in error handling.

```typescript
try {
    const WindowsMutex = await import('@vscode/windows-mutex');
    const mutex = new WindowsMutex.Mutex(win32MutexName);
    Event.once(this.lifecycleMainService.onWillShutdown)(() => mutex.release());
} catch (error) {
    this.logService.error(error);
}
```

Defers expensive native module loading to method invocation. Imports are conditionally executed only when platform-specific features are needed, reducing startup cost.

---

#### Pattern: Destructured Named Imports

**Where:** `src/vs/base/node/zip.ts:164`

**What:** Extracts specific exports from dynamically imported modules using destructuring.

```typescript
const { open } = await import('yauzl');

return new Promise<ZipFile>((resolve, reject) => {
    open(zipFile, lazy ? { lazyEntries: true } : undefined!, (error: Error | null, zipfile?: ZipFile) => {
        if (error) {
            reject(toExtractError(error));
        } else {
            resolve(assertReturnsDefined(zipfile));
        }
    });
});
```

Destructures named exports directly from the dynamic import result. Commonly used for builtin Node modules and external npm packages.

---

#### Pattern: Script Loading with Webpack Pragmas

**Where:** `src/vs/amdX.ts:174-182`

**What:** Dynamic script loading with bundler directives to bypass processing and enable runtime introspection.

```typescript
private async _workerLoadScript(scriptSrc: string): Promise<DefineCall | undefined> {
    if (this._amdPolicy) {
        scriptSrc = this._amdPolicy.createScriptURL(scriptSrc) as unknown as string;
    }
    await import(/* webpackIgnore: true */ /* @vite-ignore */ scriptSrc);
    return this._defineCalls.pop();
}

private async _nodeJSLoadScript(scriptSrc: string): Promise<DefineCall | undefined> {
    try {
        const fs = (await import(/* webpackIgnore: true */ /* @vite-ignore */ `${'fs'}`)).default;
        const vm = (await import(/* webpackIgnore: true */ /* @vite-ignore */ `${'vm'}`)).default;
        const module = (await import(/* webpackIgnore: true */ /* @vite-ignore */ `${'module'}`)).default;
```

Uses `webpackIgnore` and `@vite-ignore` pragmas to prevent bundler static analysis. The string concatenation pattern `\`${'fs'}\`` obfuscates imports from bundlers while allowing them at runtime. Used for AMD compatibility layer.

---

#### Pattern: Conditional Path Import with Dev Detection

**Where:** `src/vs/code/node/cli.ts:134-142`

**What:** Branches import path based on development mode detection.

```typescript
let cliProcessMain: string;
if (process.env['VSCODE_DEV']) {
    cliProcessMain = './cliProcessMain.js';
} else {
    cliProcessMain = './vs/code/node/cliProcessMain.js';
}

const cli = await import(cliProcessMain);
await cli.main(args);
```

Determines module path from environment variable before dynamic import. Development builds use a different structure than packaged distributions, so the import path is selected at runtime.

---

#### Pattern: Windows Native Module with Error Recovery

**Where:** `src/vs/code/electron-main/main.ts:505-512`

**What:** Safely imports Windows-specific native module with fallback behavior on failure.

```typescript
try {
    const updatingMutexName = `${productService.win32MutexName}-updating`;
    const mutex = await import('@vscode/windows-mutex');
    return mutex.isActive(updatingMutexName);
} catch (error) {
    console.error('Failed to check Inno Setup mutex:', error);
    return false;
}
```

Dynamically imports native bindings and returns sensible defaults on error. The entire operation is async but the error is caught and logged, allowing the application to continue.

---

#### Pattern: Builtin Node Module Lazy Loading

**Where:** `src/vs/platform/debug/electron-main/extensionHostDebugIpc.ts:75`

**What:** Defers Node.js core module loading to avoid initialization overhead.

```typescript
private async openCdpServer(ident: string, onSocket: (socket: ISocket) => void): Promise<{ server: Server; wsUrl: string; port: number }> {
    const { createServer } = await import('http'); // Lazy due to https://github.com/nodejs/node/issues/59686
```

Comment references a specific Node.js issue about the `http` module. Lazily imports builtin modules only when the feature is accessed, addressing known performance issues in Node.js.

---

## Integration Notes for Tauri/Rust Port

These patterns represent the current TypeScript/Electron module loading architecture:

1. **ESM Bootstrap Model**: Entry points are dynamically selected and loaded after bootstrap phase setup
2. **Conditional Platform Loading**: Windows-specific native modules are imported conditionally, avoiding macOS/Linux load failures
3. **Lazy Module Deferral**: Heavy modules (http, compression, native bindings) are loaded on-demand rather than at startup
4. **Bundler Integration**: Webpack and Vite pragmas control tree-shaking and static analysis, critical for maintaining runtime loading semantics
5. **Environment-Driven Architecture**: Build mode, entry points, and module paths are determined by environment variables at runtime
6. **Error Boundaries**: Native module imports include explicit error handling with graceful fallbacks

When porting to Tauri/Rust, the lazy-loading pattern should translate to explicit feature flags and Rust module loading, while conditional platform loading becomes conditional compilation with `#[cfg]` attributes.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
