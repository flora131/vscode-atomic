### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/postinstall.mjs` (58 lines)

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/extensions/postinstall.mjs`

**Role**: A Node.js ES-module post-install script executed after `npm install` inside the `extensions/` workspace. Its sole purpose is to prune unnecessary files from the `node_modules/typescript` package to reduce installation size.

**Imports and top-level setup (lines 6–10)**

- Imports `fs` (Node built-in), `path`, and `fileURLToPath` from `url`.
- Derives `root` as an absolute path to `<script-dir>/node_modules/typescript` using `import.meta.url` (ESM idiom for `__dirname`).

**`processRoot()` — lines 12–24**

- Reads the top-level entries of `node_modules/typescript`.
- Maintains a `toKeep` set containing only `'lib'` and `'package.json'`.
- For every entry NOT in `toKeep` (e.g., `bin/`, `README.md`, `AUTHORS`, etc.), calls `fs.rmSync(filePath, { recursive: true })` and logs the removal.
- This deletes the TypeScript CLI binaries, supplementary docs, and any other top-level artifacts, keeping only the library directory and the manifest.

**`processLib()` — lines 26–54**

- Targets `node_modules/typescript/lib`.
- Defines an explicit `toDelete` set: `tsc.js`, `_tsc.js`, `typescriptServices.js`, `_typescriptServices.js`.
- Iterates every file in `lib/`:
  - **Keeps** any file matching `lib.d.ts`, `lib.*.d.ts`, or `protocol.d.ts` (standard library type definitions) — `continue` at line 38.
  - **Keeps** `typescript.js` and `typescript.d.ts` explicitly (line 41–43), with an inline comment noting these are "used by html and extension editing".
  - **Deletes** anything in `toDelete` OR anything with a `.d.ts` extension not already kept (line 46). Deletion is done via `fs.unlinkSync` with a try/catch that warns on errors (lines 47–53).
- Net effect: the runtime `typescript.js` API bundle and the core `lib.*.d.ts` declaration files are preserved; the TypeScript compiler CLI script (`tsc.js`) and large service bundle (`typescriptServices.js`) are removed.

**Execution (lines 57–58)**

- `processRoot()` then `processLib()` are called unconditionally at module load time. There is no guard, no command-line argument parsing, and no exported API.

**Key observations about retained artifacts**

- `typescript.js` is kept because it provides the TypeScript language API used at runtime by extensions (e.g., HTML language server, TypeScript extension).
- The standard library `.d.ts` files (`lib.d.ts`, `lib.es2015.d.ts`, etc.) and `protocol.d.ts` are kept because language services need them for type checking.
- Everything else — compiler CLI, service worker bundle, extra `.d.ts` files not matching the kept patterns — is deleted.

---

### Cross-Cutting Synthesis

This file is a packaging optimization utility confined to the `extensions/` sub-workspace. It has no bearing on the core IDE runtime, the Electron main/renderer process, language server protocols, debug adapters, the integrated terminal, or source control integrations — all of which are central to any Tauri/Rust porting effort. The script's only indirect relevance is that it reveals which parts of the TypeScript npm package VS Code's extensions actually require at runtime: the language API (`typescript.js`) and the standard library type definitions, but NOT the compiler CLI or the large `typescriptServices` bundle. This distinction could inform decisions about which TypeScript-API surfaces need Rust replacements or WASM wrappers in a Tauri port, though the script itself implements nothing of that kind.

---

### Out-of-Partition References

No cross-partition file references were encountered. The script operates entirely on paths derived from `import.meta.url` (i.e., paths relative to itself within `extensions/node_modules/typescript`) and imports only Node.js built-in modules (`fs`, `path`, `url`). No VS Code source modules, configuration files, or other workspace packages are imported or referenced.
