### Files Analysed

- `src/server-cli.ts` (31 lines)
- `src/bootstrap-server.ts` (7 lines, read for dependency context)
- `src/bootstrap-node.ts` (191 lines, read for `devInjectNodeModuleLookupPath`)
- `src/bootstrap-esm.ts` (113 lines, read for `bootstrapESM`)
- `src/bootstrap-meta.ts` (55 lines, read for `product`)

---

### Per-File Notes

#### `src/server-cli.ts`

- **Role:** Top-level entry point (bootstrapper) for VS Code's remote/server CLI mode. It sequences four initialization steps in order before handing off to the actual server CLI implementation. This file is executed directly by Node.js (uses top-level `await`, so it requires ESM mode or a Node.js version that supports top-level await in `.ts`/`.js` ES modules).

- **Key symbols:**
  - `nlsConfiguration` (line 14) — `await`-resolved result of `resolveNLSConfiguration`, representing the locale/NLS config object.
  - `process.env['VSCODE_NLS_CONFIG']` (line 15) — Environment variable set to `JSON.stringify(nlsConfiguration)`; consumed downstream by `bootstrapESM` (at `src/bootstrap-esm.ts:55`).
  - `process.env['VSCODE_DEV']` (line 17) — Guard controlling the dev-mode node module injection branch.
  - `process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH']` (line 20) — Path to `remote/node_modules`; used to redirect Node module resolution in dev mode via `devInjectNodeModuleLookupPath`.

- **Control flow:**
  1. **Line 6:** Side-effect import of `./bootstrap-server.js`. This runs `bootstrap-server.ts` immediately; its sole effect is `delete process.env['ELECTRON_RUN_AS_NODE']` (bootstrap-server.ts:7), ensuring that `bootstrap-esm.ts` does not install the `fs → original-fs` loader hook (bootstrap-esm.ts:14).
  2. **Lines 7–11:** Static imports of path utilities, `devInjectNodeModuleLookupPath`, `bootstrapESM`, `resolveNLSConfiguration`, and `product`.
  3. **Line 14:** `await resolveNLSConfiguration(...)` — resolves NLS (locale) config synchronously-async. Parameters hard-code `userLocale: 'en'` and `osLocale: 'en'`; `commit` comes from `product.commit` (bootstrap-meta.ts:54); `nlsMetadataPath` is `import.meta.dirname` (the directory of `server-cli.ts`).
  4. **Line 15:** Serializes `nlsConfiguration` to `process.env['VSCODE_NLS_CONFIG']`. This env var must exist before `bootstrapESM()` is called because `bootstrapESM` reads it at `bootstrap-esm.ts:55–64` to configure `globalThis._VSCODE_NLS_LANGUAGE` and `globalThis._VSCODE_NLS_MESSAGES`.
  5. **Lines 17–24:** Dev-mode branch: if `VSCODE_DEV` is set, computes the `remote/node_modules` path relative to `import.meta.dirname` (line 20) and calls `devInjectNodeModuleLookupPath(path)` (bootstrap-node.ts:62–74). This registers a Node.js module loader hook via `Module.register('./bootstrap-import.js', ...)` so that Node resolves native modules from `remote/node_modules` (compiled against Node, not Electron). In non-dev mode (lines 22–24), cleans up the env var.
  6. **Line 27:** `await bootstrapESM()` — finalizes ESM/NLS setup: reads NLS messages file and populates `globalThis._VSCODE_NLS_MESSAGES` (bootstrap-esm.ts:78), marks performance timestamps (bootstrap-esm.ts:50, 101).
  7. **Line 30:** `await import('./vs/server/node/server.cli.js')` — dynamic ESM import of the actual server CLI logic. This is the terminal handoff; `server-cli.ts` itself does nothing more after this point.

- **Data flow:**
  - `product.commit` flows from `bootstrap-meta.ts:54` → `resolveNLSConfiguration` (line 14) → NLS config object → `process.env['VSCODE_NLS_CONFIG']` (line 15) → consumed by `bootstrapESM` at `bootstrap-esm.ts:55`.
  - `import.meta.dirname` (the `src/` directory path) is used both as `nlsMetadataPath` for NLS resolution (line 14) and as the base for computing `remote/node_modules` path (line 20).
  - `VSCODE_DEV` env var controls the module lookup path injection; the injected path (`VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH`) is propagated to `devInjectNodeModuleLookupPath` which registers it into Node's module resolution via `Module.register` (bootstrap-node.ts:73).

- **Dependencies:**
  - `./bootstrap-server.js` (side-effect, `src/bootstrap-server.ts:7`) — deletes `ELECTRON_RUN_AS_NODE` env var.
  - `node:path` — `join` for path construction (line 7).
  - `./bootstrap-node.js` — exports `devInjectNodeModuleLookupPath` (bootstrap-node.ts:62).
  - `./bootstrap-esm.js` — exports `bootstrapESM` (bootstrap-esm.ts:108).
  - `./vs/base/node/nls.js` — exports `resolveNLSConfiguration` (line 10).
  - `./bootstrap-meta.js` — exports `product` object loaded from `product.json` / build-injected config (bootstrap-meta.ts:54).
  - `./vs/server/node/server.cli.js` — dynamically imported at line 30; implements the full server CLI behavior.

---

### Cross-Cutting Synthesis

`src/server-cli.ts` is a thin, sequential bootstrapper of exactly 31 lines. It implements a strict initialization order: first it neutralizes Electron-specific environment state (`ELECTRON_RUN_AS_NODE` deletion via bootstrap-server.ts:7), then resolves and injects NLS configuration into the process environment (lines 14–15), then conditionally redirects Node module resolution to `remote/node_modules` for dev-mode compatibility (lines 17–24), then finalizes the ESM loader setup (line 27), and finally delegates entirely to the real server CLI via a dynamic `import()` (line 30). The file itself contains no business logic — it is purely an initialization sequence. For a Tauri/Rust port of VS Code's remote server functionality, this entire file represents Node.js and JavaScript-ESM-specific concerns: process environment manipulation, Node module loader hook registration, and dynamic import chaining. The equivalent in Rust would involve static binary initialization, environment variable handling via `std::env`, and linking against the Rust server binary directly rather than via a module loader. The NLS configuration step (line 14) would need a Rust equivalent for locale/translation metadata resolution, and the dev-mode module path injection (lines 17–24) has no direct Rust analog since Rust resolves dependencies at compile time.

---

### Out-of-Partition References

- `src/bootstrap-server.ts:7` — Deletes `ELECTRON_RUN_AS_NODE`; imported as side-effect at server-cli.ts:6.
- `src/bootstrap-node.ts:62–74` — `devInjectNodeModuleLookupPath`; registers Node module loader hook via `Module.register('./bootstrap-import.js', ...)`.
- `src/bootstrap-esm.ts:108–112` — `bootstrapESM`; reads `VSCODE_NLS_CONFIG` env var and populates `globalThis._VSCODE_NLS_MESSAGES`.
- `src/bootstrap-esm.ts:14–30` — `fs → original-fs` ESM loader hook; skipped when `ELECTRON_RUN_AS_NODE` is absent.
- `src/bootstrap-meta.ts:12–54` — `product` export; loads `product.json` at runtime (or build-injected value); provides `product.commit` used by NLS resolution.
- `src/vs/base/node/nls.ts` — `resolveNLSConfiguration`; resolves locale, commit, and NLS metadata path to a full NLS config object.
- `src/vs/server/node/server.cli.ts` — Actual server CLI implementation; dynamically imported at server-cli.ts:30 as the terminal handoff.
- `src/bootstrap-node.ts` (via `bootstrap-import.js`) — Custom Node module loader hook script registered at bootstrap-node.ts:73.
