# Module Loading Architecture: bootstrap-esm.ts and ESM/AMD Transition

## Overview
`src/bootstrap-esm.ts` (112 LOC) is the core ESM initialization module for VS Code's Node.js runtime. It manages the transition from AMD (Asynchronous Module Definition) to ESM (ECMAScript Modules) by setting up globals, registering module resolution hooks, and initializing the National Language Support (NLS) infrastructure.

---

## Implementation Files

### Core Bootstrap System
- **`src/bootstrap-esm.ts`** (112 LOC) - Main ESM bootstrap entry point
  - Registers Node.js module resolution hook via `register()` for 'fs' → 'original-fs' mapping
  - Initializes global objects: `_VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON`, `_VSCODE_FILE_ROOT`
  - Implements `setupNLS()` and `doSetupNLS()` for NLS initialization
  - Exports `bootstrapESM()` async function as contract for host initialization

- **`src/bootstrap-node.ts`** (191 LOC) - Node.js infrastructure setup
  - `devInjectNodeModuleLookupPath()` - Registers ESM loader hook via `Module.register()`
  - `removeGlobalNodeJsModuleLookupPaths()` - Disables global module lookup paths
  - `configurePortable()` - Portable application mode configuration
  - Error stack trace limit configuration (100 frames)
  - SIGPIPE handling for broken pipe states
  - Working directory setup across platforms

- **`src/bootstrap-meta.ts`** (56 LOC) - Product and package metadata
  - Loads `product.json` and `package.json` via `createRequire()`
  - Handles embedded app configurations
  - Supports environment-based overrides via `product.overrides.json`

- **`src/bootstrap-import.ts`** (102 LOC) - Module specifier resolver
  - `initialize()` - Populates specifier-to-URL and specifier-to-format mappings
  - `resolve()` - ESM loader hook for module resolution
  - Resolves package.json `exports["."]` with conditional imports
  - Distinguishes between ESM (.mjs), CommonJS (.cjs), and default module types

- **`src/bootstrap-fork.ts`** (229 LOC) - Forked process bootstrap
  - Initializes global ESM and NLS infrastructure
  - Implements process logging pipeline to parent via `process.send()`
  - Handles uncaught exceptions and unhandled promise rejections
  - Parent process termination monitoring
  - Dynamically imports ESM entrypoint via `process.env['VSCODE_ESM_ENTRYPOINT']`
  - Dynamic import pattern: `await import(['./${env}'.js'].join('/'))`

### Entry Points
- **`src/main.ts`** (742 LOC) - Electron main process entry
  - Calls `bootstrapESM()` at line 208 during Electron app startup
  - Resolves NLS configuration before ESM bootstrap
  - Sets `VSCODE_NLS_CONFIG` environment variable
  - Sets `VSCODE_CODE_CACHE_PATH` for V8 code caching
  - Passes NLS config to `./vs/code/electron-main/main.js` after bootstrap

- **`src/server-main.ts`** - Server runtime entry point (references bootstrap functions)
- **`src/server-cli.ts`** - CLI server entry point
- **`src/cli.ts`** - CLI entry point

---

## Type Definitions

### Global Declarations
- **`src/typings/vscode-globals-nls.d.ts`** (41 LOC)
  - `globalThis._VSCODE_NLS_MESSAGES: string[]` - Flattened NLS message array
  - `globalThis._VSCODE_NLS_LANGUAGE: string | undefined` - Active language code

- **`src/typings/vscode-globals-product.d.ts`** (48 LOC)
  - `globalThis._VSCODE_FILE_ROOT: string` - Base file path for resources
  - `globalThis._VSCODE_PRODUCT_JSON: Record<string, any>` - Product config (deprecated)
  - `globalThis._VSCODE_PACKAGE_JSON: Record<string, any>` - Package config (deprecated)
  - `globalThis._VSCODE_CSS_LOAD` - CSS loader function (dev-time only)
  - `globalThis._VSCODE_DISABLE_CSS_IMPORT_MAP` - CSS import map control (deprecated)
  - `globalThis._VSCODE_USE_RELATIVE_IMPORTS` - Source module reference mode (deprecated)

- **`src/typings/base-common.d.ts`** (41 LOC) - Common browser/Node polyfills

---

## Configuration and Infrastructure

### NLS System
- **`src/vs/nls.ts`** (245 LOC) - Localization API
  - `localize()` / `localize2()` - String localization functions
  - `getNLSMessages()` / `getNLSLanguage()` - Global accessor functions
  - `lookupMessage()` - Index-based message lookup during build
  - `INLSConfiguration` interface - Configuration contract
  - `ILanguagePack` interface - Language pack structure with translations
  - Pseudo-language support (duplicates vowels for testing)

- **`src/vs/base/node/nls.ts`** (150+ LOC) - Node.js NLS resolution
  - `resolveNLSConfiguration()` - Async NLS setup from context
  - Language pack detection and caching in `<userDataPath>/clp/`
  - Translation file format parsing (nls.keys.json, nls.messages.json)
  - Fallback to default English messages if pack unavailable
  - Corrupt language pack detection and cleanup

### Performance Monitoring
- **`src/vs/base/common/performance.ts`** (100+ LOC)
  - `mark()` / `clearMarks()` functions for performance measurement
  - Polyfill for browser contexts using Date.now()
  - Native performance API usage for environments with `performance.mark()`
  - Time origin tracking for accurate relative measurements

---

## AMD-to-ESM Bridge (Legacy Support)

### AMD Module Loader
- **`src/vs/amdX.ts`** (241 LOC) - AMD to ESM compatibility layer
  - `AMDModuleImporter` singleton class
  - `load()` - Dynamically loads AMD modules and instantiates them
  - `_rendererLoadScript()` - Browser `<script>` tag injection
  - `_workerLoadScript()` - Web Worker dynamic import: `await import(scriptSrc)`
  - `_nodeJSLoadScript()` - Node.js vm module evaluation using `module.wrap()`
  - `importAMDNodeModule()` - Factory for importing node_modules AMD bundles
  - Caching mechanism to prevent duplicate loads
  - TrustedTypes policy enforcement for renderer context

---

## Module Resolution Contract

### Node.js Loader Hook Pattern
The module loading contract VS Code uses relies on Node.js loader hooks (ESM API):

1. **Hook Registration** (in bootstrap-node.ts):
   ```typescript
   Module.register('./bootstrap-import.js', { parentURL: import.meta.url, data: injectPath })
   ```

2. **Hook Implementation** (in bootstrap-import.ts):
   ```typescript
   export async function resolve(specifier, context, nextResolve) {
     const newSpecifier = _specifierToUrl[specifier];
     if (newSpecifier !== undefined) {
       return {
         format: _specifierToFormat[specifier] ?? 'commonjs',
         shortCircuit: true,
         url: newSpecifier
       };
     }
     return nextResolve(specifier, context);
   }
   ```

3. **In-Memory Loader Registration** (in bootstrap-esm.ts):
   ```typescript
   const jsCode = `
     export async function resolve(specifier, context, nextResolve) {
       if (specifier === 'fs') {
         return { format: 'builtin', shortCircuit: true, url: 'node:original-fs' };
       }
       return nextResolve(specifier, context);
     }`;
   register(`data:text/javascript;base64,${Buffer.from(jsCode).toString('base64')}`, import.meta.url);
   ```

### Key Features:
- Resolves CommonJS vs ESM formats based on package.json `type` field and file extensions
- Maps specifiers to file URLs for node_modules
- Supports conditional exports (`exports["."].import` vs `exports["."].default`)
- Chains multiple loader hooks via `nextResolve()` callback
- Uses base64 data URLs for in-memory loader registration

---

## Global Contract

The Rust host must reproduce or replace:

1. **Global Setup**:
   - Set `globalThis._VSCODE_PRODUCT_JSON` with product config
   - Set `globalThis._VSCODE_PACKAGE_JSON` with package metadata
   - Set `globalThis._VSCODE_FILE_ROOT` with resource root path

2. **Module Resolution**:
   - Intercept all `await import()` calls
   - Map specifiers to file paths using package.json exports
   - Distinguish CommonJS from ESM based on package metadata
   - Chain resolution through multiple handler layers

3. **NLS Infrastructure**:
   - Parse `VSCODE_NLS_CONFIG` environment variable
   - Load language pack from configured `messagesFile` path
   - Set `globalThis._VSCODE_NLS_MESSAGES` as flat string array
   - Set `globalThis._VSCODE_NLS_LANGUAGE` with resolved language code
   - Handle missing or corrupt language packs with English fallback

4. **Dynamic Module Loading**:
   - Support `await import(dynamicPath)` for plugin/extension loading
   - Execute NLS setup async before importing main bundle
   - Mark performance checkpoints: `code/willLoadNls`, `code/didLoadNls`

---

## Related Directories

- **`src/vs/code/`** - Platform-specific main entry points (electron-main/main.js)
- **`src/vs/base/node/`** - Node.js utilities (nls.ts, unc.js, etc.)
- **`src/vs/platform/environment/`** - Environment variable and user data path management
- **`src/typings/`** - Global type definitions for bootstrap infrastructure
- **`src/vs/nls.ts`** - Core NLS implementation

---

## Notable Implementation Details

**Performance Marking**: bootstrap-esm.ts uses `performance.mark()` to track NLS loading:
- `code/willLoadNls` - Before NLS file I/O
- `code/didLoadNls` - After NLS file loaded and parsed

**Environment Variables**:
- `ELECTRON_RUN_AS_NODE` - Indicates Electron in Node mode
- `VSCODE_NLS_CONFIG` - JSON-serialized NLS configuration (set before ESM bootstrap)
- `VSCODE_CODE_CACHE_PATH` - V8 code cache directory
- `VSCODE_DEV` - Development mode flag (disables NLS support)
- `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH` - Dynamic module path injection for dev

**File System**: Uses `node:fs` → `node:original-fs` mapping to work around Electron's fs override, then transparently proxies to original-fs via module resolution hook.

---

## Summary

The bootstrap-esm.ts module-loading contract is a three-phase initialization system:
1. **Module Resolution**: Register ESM loader hooks that map package specifiers to file URLs and resolve format types
2. **Global Setup**: Initialize product/package/resource globals that dependent code accesses
3. **NLS Bootstrap**: Asynchronously load and parse language pack files, populating the NLS message array and language globals

A Tauri/Rust host must implement equivalent interception of dynamic imports, format detection based on package metadata, and async global initialization before the main application bundle loads.
