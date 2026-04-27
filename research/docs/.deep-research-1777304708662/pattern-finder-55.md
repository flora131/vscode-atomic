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

