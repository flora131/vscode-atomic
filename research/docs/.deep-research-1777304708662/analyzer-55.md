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
