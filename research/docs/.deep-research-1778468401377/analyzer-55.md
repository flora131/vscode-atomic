### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts` (112 LOC) — primary scope

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts`

**Purpose**

This file is the ESM-layer bootstrap for the VS Code Node process. It runs at startup (before application code) and is responsible for: (1) patching Node's ESM module resolution to redirect `fs` to Electron's `original-fs` when running under Electron, (2) populating a set of well-known `globalThis` properties that downstream modules read without importing, and (3) performing async NLS (i18n message) loading. It exports a single async function `bootstrapESM()` that orchestrates the NLS load.

---

**Imports (lines 6–11)**

| Line | Import | Role |
|------|--------|------|
| 6 | `import * as fs from 'node:fs'` | Used only inside `doSetupNLS()` to read NLS message JSON files from disk via `fs.promises.readFile` / `fs.promises.writeFile` |
| 7 | `import { register } from 'node:module'` | Node 20+ API that registers a custom ESM loader hook at runtime |
| 8 | `import { product, pkg } from './bootstrap-meta.js'` | Provides pre-loaded `product.json` and `package.json` objects (built or sourced from disk) |
| 9 | `import './bootstrap-node.js'` | Side-effect-only import; executes Node-level bootstrap setup (no symbol binding) |
| 10 | `import * as performance from './vs/base/common/performance.js'` | Used to emit `code/willLoadNls` and `code/didLoadNls` performance marks |
| 11 | `import { INLSConfiguration } from './vs/nls.js'` | Type-only import of the interface describing the parsed `VSCODE_NLS_CONFIG` shape |

---

**Module-resolution hook — Electron `fs` → `original-fs` (lines 14–30)**

The outermost guard `if (process.env['ELECTRON_RUN_AS_NODE'] || process.versions['electron'])` at line 14 detects that the process is running inside Electron (either as a Node subprocess or as the renderer/main process). Inside the branch:

- A small inline ESM loader hook is composed as a template literal string `jsCode` (lines 15–28). The hook exports an async `resolve` function that intercepts any `specifier === 'fs'` and returns `{ format: 'builtin', shortCircuit: true, url: 'node:original-fs' }`, routing all `import 'fs'` calls to Electron's `original-fs` built-in instead of the standard Node `fs`. For all other specifiers the hook calls `nextResolve(specifier, context)` to continue normal resolution.
- The hook string is base64-encoded and registered via `register(`data:text/javascript;base64,${...}`, import.meta.url)` at line 29, using a `data:` URL as the loader module so no separate file is needed.

---

**Global property setup (lines 33–35)**

Three `globalThis` properties are written immediately (synchronously) after the hook registration:

| Line | Global | Value |
|------|--------|-------|
| 33 | `globalThis._VSCODE_PRODUCT_JSON` | Shallow copy of `product` (from `bootstrap-meta.ts`) |
| 34 | `globalThis._VSCODE_PACKAGE_JSON` | Shallow copy of `pkg` (from `bootstrap-meta.ts`) |
| 35 | `globalThis._VSCODE_FILE_ROOT` | `import.meta.dirname` — the absolute directory of this file, i.e. the source/dist root |

These globals are consumed by `src/vs/platform/product/common/product.ts:28–46` (reads `_VSCODE_PRODUCT_JSON` and `_VSCODE_PACKAGE_JSON`) and by `src/vs/base/common/network.ts:366–367` and `src/vs/platform/webWorker/browser/webWorkerServiceImpl.ts:105` (reads `_VSCODE_FILE_ROOT`).

---

**NLS helpers (lines 37–106)**

The NLS region implements a lazy singleton load using a module-level `setupNLSResult` variable (line 39, type `Promise<INLSConfiguration | undefined> | undefined`).

`setupNLS()` (lines 41–47) is a synchronous gate that creates the promise exactly once by calling `doSetupNLS()` on first invocation and caching it in `setupNLSResult`. Subsequent calls return the same promise.

`doSetupNLS()` (lines 49–104) is the async implementation:

1. **Performance mark** — emits `performance.mark('code/willLoadNls')` at line 50.
2. **Parse env config** (lines 55–68) — reads `process.env['VSCODE_NLS_CONFIG']` via `JSON.parse`. From the parsed `nlsConfig` (`INLSConfiguration`), it derives `messagesFile` by preferring `nlsConfig.languagePack.messagesFile` (line 59) and falling back to `nlsConfig.defaultMessagesFile` (line 61). Sets `globalThis._VSCODE_NLS_LANGUAGE = nlsConfig?.resolvedLanguage` at line 64.
3. **Dev/missing-file short-circuit** (lines 70–75) — if `process.env['VSCODE_DEV']` is set or `messagesFile` is absent, returns `undefined` without loading any messages.
4. **Primary message load** (lines 77–99) — reads `messagesFile` with `fs.promises.readFile`, parses JSON, and assigns the result to `globalThis._VSCODE_NLS_MESSAGES` (line 78). On error:
   - Writes a `'corrupted'` marker to `nlsConfig.languagePack.corruptMarkerFile` (lines 83–88) so the next startup rebuilds the language pack cache.
   - Falls back to `nlsConfig.defaultMessagesFile` (lines 92–98) and sets `globalThis._VSCODE_NLS_MESSAGES` to the English messages if that secondary read succeeds.
5. **Performance mark** — emits `performance.mark('code/didLoadNls')` at line 101.
6. Returns `nlsConfig` (line 103).

The two globals written here (`_VSCODE_NLS_MESSAGES`, `_VSCODE_NLS_LANGUAGE`) are read by `src/vs/nls.ts:7` and `src/vs/nls.ts:11` in their respective accessor functions, making them the effective i18n data source for all downstream `localize()` calls.

---

**Exported entry point (lines 108–112)**

```typescript
export async function bootstrapESM(): Promise<void> {
    // NLS
    await setupNLS();
}
```

`bootstrapESM()` is the sole export of the file. It awaits `setupNLS()` and therefore ensures NLS messages are loaded before the caller (the main entry point) proceeds. The `globalThis` globals are already set synchronously before this function is ever called, so callers only need to await for NLS readiness.

---

**Data flow summary**

```
process.env['VSCODE_NLS_CONFIG']   ──parse──►  nlsConfig (INLSConfiguration)
  nlsConfig.languagePack.messagesFile  ──readFile──►  globalThis._VSCODE_NLS_MESSAGES
  nlsConfig.resolvedLanguage           ──assign──►   globalThis._VSCODE_NLS_LANGUAGE

bootstrap-meta.ts { product, pkg }  ──spread──►  globalThis._VSCODE_PRODUCT_JSON
                                                  globalThis._VSCODE_PACKAGE_JSON
import.meta.dirname                  ──assign──►  globalThis._VSCODE_FILE_ROOT
```

---

### Cross-Cutting Synthesis

`bootstrap-esm.ts` occupies a well-defined slice of the VS Code startup sequence: it is the first ESM-aware layer that runs after the raw Node/process setup performed by `bootstrap-node.ts` (imported as a side effect at line 9). It depends on `bootstrap-meta.ts` only for the `product`/`pkg` objects that were loaded (or built-in substituted) before ESM resolution even began. The file deliberately avoids importing any platform service or workbench code — all downstream modules instead read the five `globalThis` properties it writes, creating a one-way dependency edge from the entire VS Code module graph back to this file. The Electron hook ensures that any ESM `import 'fs'` in the process (including in lazily loaded extensions) transparently resolves to `original-fs`, which Electron exposes to avoid conflicts with its patched Node version. The `bootstrapESM()` export is the single await point through which callers (typically the main entry point) synchronize on NLS readiness before the workbench or server code executes.

For a Tauri/Rust port: the `register()` + data-URL hook mechanism, `import.meta.dirname`, and the `original-fs` redirect are all Node/Electron-specific. The NLS loading pattern (env var → JSON file → globalThis) and the product/package global pattern could be preserved but would need a different injection mechanism in a Tauri context.

---

### Out-of-Partition References

The following files are referenced by or directly interact with `bootstrap-esm.ts` but are covered by other partitions:

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-meta.ts` — exports `product` and `pkg`; handles build-time patching vs. source-mode `require()` of `product.json` / `package.json`
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-node.ts` — side-effect import at line 9; performs Node-level process setup
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-import.ts` — sibling ESM bootstrap file (not imported here but part of the bootstrap family)
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-fork.ts` — sibling; bootstraps forked child processes
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-server.ts` — sibling; server-side bootstrap
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-cli.ts` — sibling; CLI entrypoint bootstrap
- `/home/norinlavaee/projects/vscode-atomic/src/vs/nls.ts` — defines `INLSConfiguration` (imported as type at line 11); its runtime accessor functions at lines 7 and 11 read `globalThis._VSCODE_NLS_MESSAGES` and `globalThis._VSCODE_NLS_LANGUAGE` written by `doSetupNLS()`
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/common/performance.ts` — provides `performance.mark()` used at lines 50 and 101
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/common/product.ts` (type source via `bootstrap-meta.ts`) — `IProductConfiguration` interface type
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/product/common/product.ts` — consumer of `globalThis._VSCODE_PRODUCT_JSON` and `globalThis._VSCODE_PACKAGE_JSON` at lines 28–46
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/common/network.ts` — consumer of `globalThis._VSCODE_FILE_ROOT` at lines 366–367
- `/home/norinlavaee/projects/vscode-atomic/src/vs/platform/webWorker/browser/webWorkerServiceImpl.ts` — forwards `_VSCODE_NLS_MESSAGES`, `_VSCODE_NLS_LANGUAGE`, `_VSCODE_FILE_ROOT` into web worker globals at lines 103–105
- `/home/norinlavaee/projects/vscode-atomic/src/vs/amdX.ts` — reads `globalThis._VSCODE_PRODUCT_JSON` at lines 207 and 232
