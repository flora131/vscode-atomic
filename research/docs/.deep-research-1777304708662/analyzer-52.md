### Files Analysed

- `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts` — 190 lines, Core Node.js bootstrap module

### Per-File Notes

#### `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts`

**Role**

This module runs early in the VS Code process lifecycle, before any main/renderer logic loads. It is imported as a side-effect by `bootstrap-esm.ts` (line 9) and exports three functions consumed by several entry points: `devInjectNodeModuleLookupPath`, `removeGlobalNodeJsModuleLookupPaths`, and `configurePortable`.

---

**Top-Level Initialization (module load-time side effects)**

1. **Stack trace depth** (`bootstrap-node.ts:15`): `Error.stackTraceLimit = 100` expands V8's default limit of 10 frames to 100 immediately on module load.

2. **SIGPIPE guard** (`bootstrap-node.ts:17-30`): A conditional block checks `process.env['VSCODE_HANDLES_SIGPIPE']`. When the variable is absent, a `process.on('SIGPIPE', ...)` handler is registered. The handler uses a one-shot flag `didLogAboutSIGPIPE` (initialized `false` at line 20) to ensure only one `console.error` is emitted, preventing an infinite async loop that can arise when the console itself is in a broken pipe state (comment at line 23-25).

3. **Working directory setup** (`bootstrap-node.ts:35-53`): `setupCurrentWorkingDirectory()` is defined and called immediately at line 55.
   - If `process.env['VSCODE_CWD']` is not already a string (line 42), it writes `process.cwd()` into that environment variable, preserving the original cwd for later lookup by child processes.
   - On `win32` only (line 47), it calls `process.chdir(path.dirname(process.execPath))`, changing the cwd to the directory containing the VS Code executable. This ensures Windows-specific file resolution is anchored to the application folder rather than whatever directory the user launched from.
   - Errors are caught and routed to `console.error` (line 51).

---

**`devInjectNodeModuleLookupPath(injectPath: string)` — lines 62-74**

Guards with two conditions: (a) `process.env['VSCODE_DEV']` must be set (line 63), and (b) `injectPath` must be truthy (line 67-69). When both are satisfied, it calls `Module.register('./bootstrap-import.js', { parentURL: import.meta.url, data: injectPath })` (line 73) using a `require('node:module')` reference created at line 72.

`Module.register` installs a Node.js loader hook (ESM customization hook API). The hook module `bootstrap-import.ts` (described below) intercepts module resolution to redirect named packages to an alternate `node_modules` tree rooted at `injectPath`. This entire mechanism is described as development-only in the JSDoc comment at line 58.

Callers: `src/server-main.ts:14`, `src/server-cli.ts:8`, `src/bootstrap-fork.ts:7-8`. In `bootstrap-fork.ts` the inject path comes from `process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH']`.

---

**`removeGlobalNodeJsModuleLookupPaths()` — lines 76-128**

Early guard at line 77-79: if `process?.versions?.electron` is a string (i.e., running inside Electron), the function returns immediately because Electron already disables global search paths in its own native bindings (`shell/common/node_bindings.cc:653`, referenced in the comment).

When not in Electron the function monkey-patches two internal Node.js `Module` methods:

**`Module._resolveLookupPaths` patch (lines 84-98)**:
- Captures `Module.globalPaths` (line 82) and the original `_resolveLookupPaths` (line 84).
- The replacement function (line 86) calls the original, then walks backward through the resulting paths array comparing each entry against the global paths array suffix using a while loop (lines 90-92). The variable `commonSuffixLength` counts how many tail entries match. It then calls `paths.slice(0, paths.length - commonSuffixLength)` (line 94) to strip exactly those global entries.

**`Module._nodeModulePaths` patch (lines 100-127)**:
- Captures the original `_nodeModulePaths` (line 100).
- The replacement (line 101) calls the original then applies Windows-only filtering (checked at line 103 via the module-level `isWindows` constant from line 12).
- Drive root filtering (lines 109-113): defines `isDrive` as paths of length ≥ 3 ending with `:\`. If `from` is not itself a drive root, removes all entries whose `path.dirname` is a drive root.
- Home directory filtering (lines 115-123): reads `HOMEDRIVE` and `HOMEPATH` environment variables and computes `userDir` as their joined path's parent. If `from` is not equal to `userDir`, removes paths whose parent equals `userDir`.

---

**`configurePortable(product: Partial<IProductConfiguration>)` — lines 133-190**

Returns `{ portableDataPath: string; isPortable: boolean }`.

**`getApplicationPath()` inner function (lines 136-151)**:
- Dev mode (`VSCODE_DEV` set): returns `appRoot` (computed at line 134 as `path.dirname(import.meta.dirname)`).
- macOS (`darwin`): returns `path.dirname(path.dirname(path.dirname(appRoot)))` — three levels up, traversing the macOS `.app` bundle structure.
- Windows with `product.win32VersionedUpdate` (line 146): also three levels up to account for the versioned installation subfolder: `...\Microsoft VS Code Insiders\<version>\resources\app`.
- All other platforms: `path.dirname(path.dirname(appRoot))` — two levels up.

**`getPortableDataPath()` inner function (lines 153-164)**:
- If `process.env['VSCODE_PORTABLE']` is set (line 154), returns it directly.
- Windows/Linux (line 158): `path.join(getApplicationPath(), 'data')`.
- macOS/other: constructs a name from `product.portable` or falls back to `${product.applicationName}-portable-data` (line 162), then places the folder as a sibling of the application path: `path.join(path.dirname(getApplicationPath()), portableDataName)`.

**Detection and environment mutation (lines 166-184)**:
- `portableDataPath` is set at line 166.
- `isPortable` is `true` when the `product` object has no `'target'` key AND `fs.existsSync(portableDataPath)` returns `true` (line 167).
- `portableTempPath` = `path.join(portableDataPath, 'tmp')` (line 168).
- `isTempPortable` = `isPortable && fs.existsSync(portableTempPath)` (line 169).
- If portable: sets `process.env['VSCODE_PORTABLE']` to `portableDataPath` (line 172). Otherwise deletes it (line 174).
- If portable temp exists: sets `TMP`+`TEMP` on Windows (lines 179-180) or `TMPDIR` on other platforms (line 182).
- Returns `{ portableDataPath, isPortable }` (lines 186-189).

Callers: `src/main.ts:10` (main Electron entry), `src/cli.ts:7` (CLI entry).

---

**Imports and Module-Level State**

| Symbol | Source | Usage |
|---|---|---|
| `path` | `node:path` (line 6) | `dirname`, `join`, `relative` throughout |
| `fs` | `node:fs` (line 7) | `fs.existsSync` in `configurePortable` |
| `createRequire` | `node:module` (line 8) | Creates CJS-compatible `require` at line 11 |
| `IProductConfiguration` | `./vs/base/common/product.js` (line 9) | Type-only import for `configurePortable` parameter |
| `require` | line 11 | Used to `require('node:module')` / `require('module')` inside functions |
| `isWindows` | line 12 | Module-level constant `process.platform === 'win32'` |

The `require` at line 11 is produced via `createRequire(import.meta.url)`, which enables CJS-style `require()` within an ESM module context. This is necessary because the module uses `import` statements (ESM) but needs to access `node:module` internals that expose mutable properties (`_resolveLookupPaths`, `_nodeModulePaths`, `globalPaths`) not available through ESM imports.

---

### Cross-Cutting Synthesis

`bootstrap-node.ts` functions as a universal pre-flight module that runs once during process startup to normalize Node.js runtime behavior across all VS Code process types (main, renderer fork, server, CLI). It operates through three mechanisms: environment variable snapshots (`VSCODE_CWD`, `VSCODE_PORTABLE`), signal and error handler registration (`SIGPIPE`), and monkey-patching of internal Node.js module resolution APIs (`Module._resolveLookupPaths`, `Module._nodeModulePaths`). The Electron guard in `removeGlobalNodeJsModuleLookupPaths` reflects that Electron already performs equivalent global path suppression natively, so the patch is only needed for pure Node.js server and CLI processes. The portable mode detection uses a filesystem existence check rather than a configuration flag, meaning portability is an opt-in triggered by the presence of a `data/` directory alongside the application. The development-only loader hook registered by `devInjectNodeModuleLookupPath` delegates to a companion module (`bootstrap-import.ts`) that implements the actual ESM resolve hook logic. This separation keeps `bootstrap-node.ts` free of async code, since `Module.register` itself is synchronous even though the hook it installs operates asynchronously.

---

### Out-of-Partition References

- `/Users/norinlavaee/vscode-atomic/src/vs/base/common/product.ts` — defines `IProductConfiguration` interface (line 67); fields used in `configurePortable`: `win32VersionedUpdate` (line 81), `portable` (line 162 of bootstrap-node.ts), `applicationName` (line 83), `target` presence check.
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-import.ts` — the ESM loader hook module registered by `devInjectNodeModuleLookupPath`; implements `initialize(injectPath)` and `resolve(specifier, context, nextResolve)` hooks.
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-esm.ts` — imports `bootstrap-node.ts` as a side-effect (line 9) to run its load-time initialization; also installs an `fs`→`original-fs` redirect hook for Electron.
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-fork.ts` — calls `removeGlobalNodeJsModuleLookupPaths()` and `devInjectNodeModuleLookupPath(process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH'])`.
- `/Users/norinlavaee/vscode-atomic/src/main.ts` — calls `configurePortable` from `bootstrap-node.ts` (line 10).
- `/Users/norinlavaee/vscode-atomic/src/cli.ts` — calls `configurePortable` from `bootstrap-node.ts` (line 7).
- `/Users/norinlavaee/vscode-atomic/src/server-main.ts` — calls both `devInjectNodeModuleLookupPath` and `removeGlobalNodeJsModuleLookupPaths` (line 14).
- `/Users/norinlavaee/vscode-atomic/src/server-cli.ts` — calls `devInjectNodeModuleLookupPath` (line 8).
