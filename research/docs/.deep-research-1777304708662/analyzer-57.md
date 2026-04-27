### Files Analysed

- `/Users/norinlavaee/vscode-atomic/src/bootstrap-import.ts` (101 LOC)

---

### Per-File Notes

#### `src/bootstrap-import.ts`

- **Role:** A Node.js ESM module loader hook that intercepts `import` specifier resolution at runtime. When VS Code runs out of source (development mode), it redirects bare package specifier imports (e.g., `import 'some-package'`) to absolute file URLs pointing into a particular `node_modules` folder determined by an injected path. This implements the Node.js Module Customization Hooks API (`initialize` + `resolve`) as documented at https://nodejs.org/docs/latest/api/module.html#initialize.

- **Key symbols:**
  - Module-level lookup maps `_specifierToUrl` (`bootstrap-import.ts:19`) — `Record<string, string>` mapping package name to absolute `file://` URL of that package's entry point.
  - Module-level lookup maps `_specifierToFormat` (`bootstrap-import.ts:20`) — `Record<string, string>` mapping package name to either `'module'` or `'commonjs'`.
  - `initialize(injectPath: string): Promise<void>` (`bootstrap-import.ts:22`) — The hook's initialization export, called by Node.js when the loader is registered. Receives `injectPath` (the path of a folder alongside whose `package.json` the `node_modules` to redirect into lives).
  - `resolve(specifier, context, nextResolve)` (`bootstrap-import.ts:87`) — The hook's resolve export, called by Node.js for every `import` statement before default resolution.
  - `resolveCondition(v: unknown): string | undefined` (`bootstrap-import.ts:43`) — inner helper inside `initialize` for reading conditional export values from `package.json`.

- **Control flow:**

  **`initialize` phase (`bootstrap-import.ts:22-85`):**
  1. Converts `injectPath` string to an absolute filesystem path for the adjacent `package.json` using `fileURLToPath` + `pathToFileURL` (`bootstrap-import.ts:25`).
  2. Reads and JSON-parses that `package.json` (`bootstrap-import.ts:26`).
  3. Iterates over every key in `packageJSON.dependencies` (`bootstrap-import.ts:28`).
  4. For each dependency name, constructs a path to `node_modules/<name>/package.json` and reads/parses it (`bootstrap-import.ts:30-31`).
  5. Determines the package entry point via a priority chain (`bootstrap-import.ts:38-64`):
     - If `pkgJson.exports['.']` is a plain string, that is the entry (`bootstrap-import.ts:40-41`).
     - If `pkgJson.exports['.']` is an object, calls `resolveCondition` on `.import` first, then `.default` (`bootstrap-import.ts:55`). `resolveCondition` returns the value if it is a string, or reads its `.default` sub-key if it is an object (`bootstrap-import.ts:43-54`).
     - Falls back to `pkgJson.main` as a string (`bootstrap-import.ts:59`).
     - Ultimate fallback is `'index.js'` (`bootstrap-import.ts:63`).
  6. Appends `.js` extension if the resolved main has no JS-family extension (`bootstrap-import.ts:65-67`).
  7. Stores the fully-resolved `file://` URL into `_specifierToUrl[name]` (`bootstrap-import.ts:69`).
  8. Determines module format (`bootstrap-import.ts:71-76`): `.mjs` → `'module'`; `.cjs` → `'commonjs'`; otherwise checks `pkgJson.type === 'module'`. Stores into `_specifierToFormat[name]`.
  9. Errors per-dependency are caught and logged with `console.error`, not thrown, so one bad dependency doesn't abort the rest (`bootstrap-import.ts:78-81`).
  10. Logs a summary message at the end (`bootstrap-import.ts:84`).

  **`resolve` hook phase (`bootstrap-import.ts:87-101`):**
  1. Looks up `specifier` in `_specifierToUrl` (`bootstrap-import.ts:89`).
  2. If found, returns immediately (`shortCircuit: true`) with the mapped URL and format (`bootstrap-import.ts:91-95`), bypassing all other resolve hooks and Node.js's built-in resolution.
  3. If not found, delegates to `nextResolve(specifier, context)` (`bootstrap-import.ts:100`), which continues down the hook chain to Node.js default resolution.

- **Data flow:**
  - Input: `injectPath` string (arrives via `Module.register` `data` field in `bootstrap-node.ts:73`).
  - `injectPath` → `fileURLToPath(new URL('../package.json', pathToFileURL(injectPath)))` → `injectPackageJSONPath` (absolute path to the controlling `package.json`).
  - `injectPackageJSONPath` + each `name` in `packageJSON.dependencies` → per-package `package.json` path → parsed `pkgJson` object → resolved `main` entry point string → absolute `mainPath` → `file://` URL stored in `_specifierToUrl[name]`.
  - At import time: bare specifier string → lookup in `_specifierToUrl` → either `{ format, shortCircuit: true, url }` returned to Node.js, or forwarded to `nextResolve`.

- **Dependencies:**
  - `node:url` (`fileURLToPath`, `pathToFileURL`) — `bootstrap-import.ts:13`.
  - `node:fs` (`promises.readFile`) — `bootstrap-import.ts:14`.
  - `node:path` (`join`) — `bootstrap-import.ts:15`.
  - Registered as a loader by `src/bootstrap-node.ts:73` via `Module.register('./bootstrap-import.js', { parentURL: import.meta.url, data: injectPath })` inside `devInjectNodeModuleLookupPath()`.
  - The `injectPath` value that is passed determines which `package.json`/`node_modules` tree is used as the redirect target.
  - Activation is gated by `process.env['VSCODE_DEV']` in the caller (`bootstrap-node.ts:63`); this hook only operates when running out of source.

---

### Cross-Cutting Synthesis

`src/bootstrap-import.ts` is a pure Node.js ESM loader hook (Module Customization Hooks API). It serves a single development-time purpose: when VS Code is run directly from its TypeScript source tree, bare npm package specifiers that would normally be resolved via Node's default algorithm are instead redirected to a specific `node_modules` tree derived from a caller-supplied inject path. The hook is registered by `devInjectNodeModuleLookupPath` in `src/bootstrap-node.ts:73`, which is itself only invoked when `VSCODE_DEV` is set. Initialization pre-computes two lookup tables (`_specifierToUrl`, `_specifierToFormat`) by walking all entries in the inject path's `package.json` dependencies and resolving each package entry point through the full `exports` conditional chain (ESM-preferred, with CJS fallback). The `resolve` hook then intercepts each `import` at load time and short-circuits to the precomputed URL if the specifier is a known dependency, otherwise deferring to the next hook. This mechanism has no bearing on VS Code's production Electron runtime and is entirely absent from non-development builds.

---

### Out-of-Partition References

- `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts:62-73` — `devInjectNodeModuleLookupPath()` is the sole caller that registers `bootstrap-import.js` as a loader via `Module.register`, passing `injectPath` as `data`. Guards activation with `process.env['VSCODE_DEV']`.
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts:76-100` — `removeGlobalNodeJsModuleLookupPaths()` — sibling function in the same file that complements this redirection by removing Node.js global module lookup paths, relevant context for understanding the full module resolution strategy.
- The `eslint.config.js:1978` entry lists `bootstrap-import.ts` among the top-level bootstrap scripts, confirming it is treated as a standalone script target distinct from the main extension/service code under `src/vs/`.
