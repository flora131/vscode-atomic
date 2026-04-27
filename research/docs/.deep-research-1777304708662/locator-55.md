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
