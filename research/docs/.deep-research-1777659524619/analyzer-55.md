### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts` (112 LOC) — primary subject
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-meta.ts` — provides `product` and `pkg` exports
- `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-node.ts` — side-effect import; Node.js environment setup
- `/home/norinlavaee/projects/vscode-atomic/src/vs/nls.ts` — declares `INLSConfiguration` and consumes `globalThis._VSCODE_NLS_MESSAGES`
- `/home/norinlavaee/projects/vscode-atomic/src/vs/base/common/performance.ts` — provides `mark()` used in NLS timing

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-esm.ts`

- **Role:** The ESM-era entry-point bootstrap for VS Code's main process (and Node.js renderer contexts). It runs three initialisation actions unconditionally at module evaluation time: (1) conditionally registers a Node.js ESM loader hook that redirects the bare specifier `'fs'` to `'node:original-fs'` (Electron's unpatched fs), (2) writes three globals onto `globalThis`, and (3) provides `bootstrapESM()` as the single exported async function that callers `await` before any further application code runs. The NLS subsystem is fully initialised inside that `await`.

- **Key symbols:**

  | Symbol | Kind | Location |
  |---|---|---|
  | `bootstrapESM` | exported `async function` | line 108 |
  | `setupNLS` | unexported module-private function | line 41 |
  | `doSetupNLS` | unexported `async function` | line 49 |
  | `setupNLSResult` | module-private `Promise` cache | line 39 |
  | inline `jsCode` string | ESM loader hook source | lines 15–28 |
  | `register(...)` call | Node.js `node:module` registration | line 29 |

- **Control flow:**

  1. **Module evaluation phase (synchronous, lines 14–35):**
     - Lines 14–30: If `process.env['ELECTRON_RUN_AS_NODE']` is set or `process.versions['electron']` is truthy, a string of ESM loader hook JavaScript is assembled inline (`jsCode`, lines 15–28). The hook exports a single `resolve()` function that intercepts the specifier `'fs'` and returns `{ format: 'builtin', shortCircuit: true, url: 'node:original-fs' }`. All other specifiers are forwarded to `nextResolve`. The hook source string is base64-encoded and passed to `register()` as a `data:text/javascript;base64,...` URL with `import.meta.url` as the parent URL (line 29). This registers the hook into the current Node.js module graph before any subsequent `import 'fs'` statements can resolve.
     - Lines 33–35: Three `globalThis` assignments are made unconditionally:
       - `globalThis._VSCODE_PRODUCT_JSON` ← shallow copy of `product` (from `bootstrap-meta.ts`)
       - `globalThis._VSCODE_PACKAGE_JSON` ← shallow copy of `pkg` (from `bootstrap-meta.ts`)
       - `globalThis._VSCODE_FILE_ROOT` ← `import.meta.dirname` (the directory of this file at runtime)

  2. **`bootstrapESM()` — async export (lines 108–112):**
     - Calls `setupNLS()` and `await`s the result. That is the only work performed.

  3. **`setupNLS()` — memoisation wrapper (lines 41–47):**
     - Checks `setupNLSResult`; if `undefined`, calls `doSetupNLS()` and stores the returned `Promise` in `setupNLSResult`. Returns the stored promise. Subsequent calls return the same promise regardless of resolution state.

  4. **`doSetupNLS()` — async NLS initialisation (lines 49–104):**
     - Emits performance mark `'code/willLoadNls'` (line 50).
     - Reads `process.env['VSCODE_NLS_CONFIG']` and JSON-parses it into `nlsConfig: INLSConfiguration | undefined` (lines 55–68). Sets `messagesFile` from `nlsConfig.languagePack.messagesFile` (line 59) or falls back to `nlsConfig.defaultMessagesFile` (line 61). Sets `globalThis._VSCODE_NLS_LANGUAGE` to `nlsConfig.resolvedLanguage` (line 64).
     - Early-return `undefined` (line 75) if `process.env['VSCODE_DEV']` is set or `messagesFile` is still `undefined`.
     - Reads `messagesFile` from disk via `fs.promises.readFile` (line 78) and JSON-parses it into `globalThis._VSCODE_NLS_MESSAGES`.
     - On read failure (catch block, lines 79–98): if `nlsConfig.languagePack.corruptMarkerFile` is defined, writes the string `'corrupted'` to that path (line 85) to signal a cache-rebuild on next startup. Then attempts to fall back to `nlsConfig.defaultMessagesFile` and populate `globalThis._VSCODE_NLS_MESSAGES` from there (lines 92–97).
     - Emits performance mark `'code/didLoadNls'` (line 101).
     - Returns `nlsConfig`.

- **Data flow:**

  - **Input channels:**
    - `process.env['ELECTRON_RUN_AS_NODE']` / `process.versions['electron']` — controls loader hook registration
    - `product` / `pkg` from `bootstrap-meta.ts` — feeds product/package globals
    - `import.meta.dirname` — feeds `_VSCODE_FILE_ROOT`
    - `process.env['VSCODE_NLS_CONFIG']` — JSON string carrying `INLSConfiguration`
    - `process.env['VSCODE_DEV']` — disables NLS loading in dev mode
    - Filesystem: the `messagesFile` and optionally `defaultMessagesFile` paths from within `VSCODE_NLS_CONFIG`

  - **Output channels (all `globalThis` assignments):**
    - `globalThis._VSCODE_PRODUCT_JSON` (line 33)
    - `globalThis._VSCODE_PACKAGE_JSON` (line 34)
    - `globalThis._VSCODE_FILE_ROOT` (line 35)
    - `globalThis._VSCODE_NLS_LANGUAGE` (line 64)
    - `globalThis._VSCODE_NLS_MESSAGES` (line 78 and line 94) — read by `vs/nls.ts:getNLSMessages()` (line 7 of `vs/nls.ts`)

  - **Side effects:**
    - Registers an ESM loader hook into the live Node.js module graph (line 29)
    - Writes `'corrupted'` to `nlsConfig.languagePack.corruptMarkerFile` on NLS read failure (line 85)
    - Two `performance.mark()` calls record timing into the `vs/base/common/performance.ts` polyfill buffer

- **Dependencies:**
  - `node:fs` (import line 6) — `fs.promises.readFile` and `fs.promises.writeFile` in `doSetupNLS`
  - `node:module` `register` (import line 7) — ESM hook registration
  - `./bootstrap-meta.js` — `product`, `pkg` (import line 8)
  - `./bootstrap-node.js` — side-effect import; runs `setupCurrentWorkingDirectory()` and installs SIGPIPE handler (import line 9)
  - `./vs/base/common/performance.js` — `performance.mark()` (import line 10)
  - `./vs/nls.js` — `INLSConfiguration` type only (import line 11); the runtime coupling is inverted: `vs/nls.ts` reads the globals set here

---

### Cross-Cutting Synthesis

`bootstrap-esm.ts` is the **synchronous+async two-phase preamble** that every VS Code main-process entry point must `await` before doing any real work. In a Tauri/Rust port, the three responsibilities map onto different host layers. First, the **`fs` → `original-fs` loader hook** exists solely because Electron monkey-patches Node's built-in `fs` module; in a Tauri context where the renderer runs WebView and the backend is native Rust, this interception has no equivalent and is simply dropped. Second, the **`globalThis` product/package/file-root assignments** (lines 33–35) are consumed by many downstream TypeScript modules that call `globalThis._VSCODE_PRODUCT_JSON` at runtime; a Rust host using a bundled WebView would need to inject an equivalent JSON object into the WebView's JavaScript context before any application JS executes — concretely as a `window.__VSCODE_PRODUCT_JSON__` injection from the Tauri `Window::eval` or `initialization_script` API. Third, the **NLS message injection** (lines 55–104) reads a flat JSON array of translated strings from disk and places it on `globalThis._VSCODE_NLS_MESSAGES`; a Rust host would replicate this by determining the active locale at startup (via `app.getPreferredSystemLanguages()` equivalent in Rust/Tauri), loading the corresponding messages bundle from the app's resource directory, and injecting the stringified array into the WebView's JS context, again before any `localize()` calls execute. The memoised promise pattern in `setupNLS` / `setupNLSResult` (lines 39–47) is a Node.js concurrency guard with no direct Rust equivalent; in Rust this becomes a `once_cell::Lazy` or `tokio::sync::OnceCell` initialised at Tauri startup. The `performance.mark` calls (lines 50, 101) feed VS Code's internal startup timing telemetry and would be replaced by Tauri's tracing spans or simply omitted.

---

### Out-of-Partition References

| Reference | Symbol / path | Used at |
|---|---|---|
| `./bootstrap-meta.js` | `product`, `pkg` | `bootstrap-esm.ts:8, 33–34` |
| `./bootstrap-node.js` | side-effect only (`setupCurrentWorkingDirectory`, SIGPIPE handler, `Error.stackTraceLimit`) | `bootstrap-esm.ts:9` |
| `./vs/base/common/performance.js` | `performance.mark()` | `bootstrap-esm.ts:10, 50, 101` |
| `./vs/nls.js` | `INLSConfiguration` (type); `getNLSMessages()` reads `globalThis._VSCODE_NLS_MESSAGES` set here | `bootstrap-esm.ts:11`; `vs/nls.ts:7–8` |
| `node:module` `register()` | ESM loader hook registration API | `bootstrap-esm.ts:7, 29` |
| `node:fs` `promises.readFile` / `promises.writeFile` | NLS messages file I/O | `bootstrap-esm.ts:6, 78, 85, 94` |
| `globalThis._VSCODE_PRODUCT_JSON` | set here; read by downstream product-config consumers | `bootstrap-esm.ts:33` |
| `globalThis._VSCODE_PACKAGE_JSON` | set here; read by downstream package-config consumers | `bootstrap-esm.ts:34` |
| `globalThis._VSCODE_FILE_ROOT` | set here; used as app-root anchor | `bootstrap-esm.ts:35` |
| `globalThis._VSCODE_NLS_MESSAGES` | set here; read by `vs/nls.ts:getNLSMessages()` on every `localize()` call | `bootstrap-esm.ts:78, 94`; `vs/nls.ts:7` |
| `globalThis._VSCODE_NLS_LANGUAGE` | set here; read by `vs/nls.ts:getNLSLanguage()` for pseudo-locale detection | `bootstrap-esm.ts:64`; `vs/nls.ts:11` |
| `process.env['VSCODE_NLS_CONFIG']` | runtime environment variable; JSON-encoded `INLSConfiguration` | `bootstrap-esm.ts:55` |
| `process.env['VSCODE_DEV']` | dev-mode guard disabling NLS | `bootstrap-esm.ts:71` |
| `process.env['ELECTRON_RUN_AS_NODE']` / `process.versions['electron']` | Electron-context guards for loader hook | `bootstrap-esm.ts:14` |
