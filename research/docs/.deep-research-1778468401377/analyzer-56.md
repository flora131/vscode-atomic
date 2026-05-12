### Files Analysed

- `src/bootstrap-import.ts` (101 LOC)

### Per-File Notes

#### `src/bootstrap-import.ts`

- **Role:** Implements a Node.js ESM loader hook (per the Node.js `module.register()` API, https://nodejs.org/docs/latest/api/module.html#initialize). The hook is used exclusively during development (`VSCODE_DEV` mode, see `src/bootstrap-node.ts:62-74`) to redirect bare package specifiers to their resolved file URLs under a specific `node_modules` directory—needed when VS Code is run from a remote folder whose `node_modules` is not on the standard lookup path.

- **Key symbols:**
  - `_specifierToUrl` (`src/bootstrap-import.ts:19`) — module-level `Record<string, string>` mapping a package name (e.g. `"@github/copilot-sdk"`) to its resolved `file://` URL entry point.
  - `_specifierToFormat` (`src/bootstrap-import.ts:20`) — module-level `Record<string, string>` mapping the same package name to either `"module"` (ESM) or `"commonjs"` (CJS).
  - `initialize(injectPath: string)` (`src/bootstrap-import.ts:22`) — async function exported per the Node.js loader-hook contract. Called once by the Node.js runtime after `Module.register()` passes `injectPath` as the `data` argument.
  - `resolve(specifier, context, nextResolve)` (`src/bootstrap-import.ts:87`) — async function exported per the Node.js loader-hook contract. Called for every `import` statement or `require()` that the runtime resolves.
  - `resolveCondition(v)` (`src/bootstrap-import.ts:43`) — inner helper closure inside `initialize` that extracts a string path from a conditional export target (handles both plain-string and `{ default: string }` shapes, added specifically for the `copilot-sdk` package).

- **Control flow:**

  `initialize`:
  1. Converts `injectPath` to a file URL, then resolves `../package.json` relative to it (`src/bootstrap-import.ts:25`).
  2. Reads and JSON-parses that `package.json` (`src/bootstrap-import.ts:26`).
  3. Iterates over every key in `packageJSON.dependencies` (`src/bootstrap-import.ts:28`).
  4. For each dependency, reads the corresponding `node_modules/<name>/package.json` (`src/bootstrap-import.ts:30-31`).
  5. Determines the entry-point path (variable `main`) by preferring `exports["."].import`, then `exports["."].default`, then `exports["."]` as a string, then the `main` field, and finally falling back to `"index.js"` (`src/bootstrap-import.ts:38-64`).
  6. Appends `.js` if the resolved path has no recognized JS extension (`src/bootstrap-import.ts:65-67`).
  7. Writes the resolved `file://` URL into `_specifierToUrl[name]` (`src/bootstrap-import.ts:69`).
  8. Determines module format: `.mjs` → `"module"`, `.cjs` → `"commonjs"`, otherwise inspects `pkgJson.type === "module"` (`src/bootstrap-import.ts:71-76`).
  9. Writes the format into `_specifierToFormat[name]` (`src/bootstrap-import.ts:76`).
  10. Catches per-package errors and logs them without aborting the loop (`src/bootstrap-import.ts:78-81`).
  11. Logs a summary line after all packages are processed (`src/bootstrap-import.ts:84`).

  `resolve`:
  1. Looks up `specifier` in `_specifierToUrl` (`src/bootstrap-import.ts:89`).
  2. If found, returns a short-circuit resolution object with `url`, `format`, and `shortCircuit: true` (`src/bootstrap-import.ts:91-95`); Node.js will not call any further loader hooks.
  3. If not found, delegates to `nextResolve(specifier, context)` — the next hook in the chain, or the Node.js default resolver (`src/bootstrap-import.ts:100`).

- **Data flow:**
  - Input: `injectPath` (string, a filesystem path to a directory containing `package.json` and `node_modules/`) is received by `initialize` from the Node.js runtime, originating from `Module.register()` in `src/bootstrap-node.ts:73`.
  - `initialize` reads two levels of `package.json` from disk (parent and per-package), transforms those JSON structures into URL strings and format strings, and stores them into the two module-level maps.
  - `resolve` reads from those maps at import time and returns either a resolved URL object or passes control forward.
  - No external state is mutated beyond the two module-level records.

- **Dependencies:**
  - `node:url` — `fileURLToPath`, `pathToFileURL` (`src/bootstrap-import.ts:13`)
  - `node:fs` — `promises.readFile` (`src/bootstrap-import.ts:14`)
  - `node:path` — `join` (`src/bootstrap-import.ts:15`)
  - All dependencies are Node.js built-ins; no third-party packages are imported.

### Cross-Cutting Synthesis

`src/bootstrap-import.ts` is a thin development-time shim wired into VS Code's Node.js remote execution bootstrap. It is registered as a loader hook via `src/bootstrap-node.ts:73` (`Module.register('./bootstrap-import.js', ...)`) only when `VSCODE_DEV` is set. Its sole purpose is to redirect package specifiers to the `node_modules` tree of a specific `injectPath`, bypassing the default Node.js module lookup algorithm. The file does not participate in any IDE feature (editing, language intelligence, debugging, SCM, terminal). For a Tauri/Rust port, this layer is relevant only insofar as it demonstrates VS Code's reliance on Node.js's ESM loader hook API and its dynamic `package.json` parsing at startup—a mechanism that would have no direct counterpart in a Rust runtime and would need to be replaced by a Rust-native dependency bundling or linking strategy.

### Out-of-Partition References

- `src/bootstrap-node.ts` — Calls `Module.register('./bootstrap-import.js', ...)` at line 73 inside `devInjectNodeModuleLookupPath()`, which is the sole registration site for this loader hook; also guards registration behind the `VSCODE_DEV` environment variable at line 63.
