### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/postinstall.mjs` (58 LOC)

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/extensions/postinstall.mjs`

- **Role:** A Node.js ESM post-install maintenance script that runs after `npm install` is executed inside the `extensions/` directory. Its sole purpose is to aggressively prune the `typescript` package that was just installed into `extensions/node_modules/typescript`, keeping only the runtime API surface needed by the HTML language server and extension editing infrastructure, and discarding everything else to reduce disk footprint.

- **Key symbols:**
  - `root` (line 10) — computed absolute path to `extensions/node_modules/typescript`, derived at module load time using `import.meta.url` → `fileURLToPath` → `path.dirname` → `path.join`.
  - `processRoot()` (lines 12–24) — function that prunes top-level entries of the `typescript` package directory.
  - `processLib()` (lines 26–55) — function that prunes individual files inside `extensions/node_modules/typescript/lib`.

- **Control flow:**
  1. Module initializes `root` at line 10.
  2. `processRoot()` is called at line 57.
     - Defines a `toKeep` set containing exactly two entries: `'lib'` and `'package.json'` (lines 13–16).
     - Reads all directory entries of `root` with `fs.readdirSync(root)` (line 17).
     - For each entry NOT in `toKeep`, constructs its absolute path and calls `fs.rmSync(filePath, { recursive: true })` (line 21), logging the removal.
     - Net result: everything in the `typescript` package root is deleted except the `lib/` subdirectory and `package.json`.
  3. `processLib()` is called at line 58 (after `processRoot()` completes).
     - Defines a `toDelete` set of four specific filenames (lines 27–33): `tsc.js`, `_tsc.js`, `typescriptServices.js`, `_typescriptServices.js`.
     - Resolves `libRoot` as `path.join(root, 'lib')` (line 35).
     - Reads all entries of `libRoot` with `fs.readdirSync(libRoot)` (line 37).
     - For each entry in `libRoot`, applies three guard conditions in order:
       - **Keep (continue)** if the name equals `'lib.d.ts'`, matches the regex `/^lib\..*\.d\.ts$/`, or equals `'protocol.d.ts'` (line 38) — these are standard TypeScript library declaration files needed at runtime.
       - **Keep (continue)** if the name equals `'typescript.js'` or `'typescript.d.ts'` (lines 41–43) — the comment at line 42 states these are "used by html and extension editing".
       - **Delete** if the name is in `toDelete` OR if it matches `/\.d\.ts$/` (line 46) — non-exempted `.d.ts` files are unlinked via `fs.unlinkSync` (line 48), with any errors caught and printed as warnings (lines 50–52).
     - Net result: inside `lib/`, only `typescript.js`, `typescript.d.ts`, `lib.d.ts`, `lib.*.d.ts` (pattern-matched), and `protocol.d.ts` survive; the four named JS files and all other `.d.ts` files are removed.

- **Data flow:**
  - Input: filesystem state of `extensions/node_modules/typescript/` as installed by npm.
  - Transformation: two sequential destructive filesystem passes — first deletes top-level package artifacts, then selectively deletes files within `lib/`.
  - Output: a slimmed `typescript` package directory containing only `package.json`, `lib/typescript.js`, `lib/typescript.d.ts`, `lib/lib.d.ts`, `lib/lib.*.d.ts` globs, and `lib/protocol.d.ts`.
  - Side effects: each deletion is logged to stdout via `console.log` (lines 20, 49); errors during `processLib()` unlinking are written to stderr via `console.warn` (line 51).

- **Dependencies:**
  - Node.js built-in `fs` module (ESM import, line 6) — for `readdirSync`, `rmSync`, `unlinkSync`.
  - Node.js built-in `path` module (ESM import, line 7) — for `path.join`, `path.dirname`.
  - Node.js built-in `url` module (ESM import, line 8) — for `fileURLToPath`, used to convert `import.meta.url` to a filesystem path.
  - No third-party dependencies; no exports; no async operations.
  - Invoked as a postinstall lifecycle hook, meaning it is triggered automatically by npm after it finishes installing packages in `extensions/`.

---

### Cross-Cutting Synthesis

This script represents a build-time dependency hygiene mechanism that surgically reduces the installed `typescript` package to a minimal API surface. The script preserves `typescript.js` and `typescript.d.ts` (the language service API entry points) along with the standard `lib.*.d.ts` declaration files and `protocol.d.ts`, while stripping the compiler CLI (`tsc.js`), the bundled `typescriptServices.js`, and all other declaration files. This indicates the `extensions/` directory consumes TypeScript purely as a language-service library, not as a compiler toolchain.

For a Tauri/Rust port, this script has no direct runtime relevance — it is purely a Node.js/npm artifact. However, it documents which parts of the TypeScript package are actually consumed at runtime: `typescript.js` (the language service API) and the standard library `.d.ts` declaration files. A Tauri port that embeds language intelligence (e.g., via a TypeScript language server bridged over IPC) would need to bundle precisely these same artifacts, or invoke the TypeScript language server (`tsserver`) as a sidecar process. The script also signals that the HTML language server and extension editing subsystem (`extensions/`) depend on the TypeScript language service API, making those two subsystems the primary consumers to target when designing the Rust-side LSP bridge.

---

### Out-of-Partition References

- `extensions/node_modules/typescript/` — the directory this script operates on; managed by npm and populated before this script runs.
- `extensions/package.json` — must declare `"postinstall": "node postinstall.mjs"` (or equivalent) in its `scripts` field for this script to be triggered by npm. Not read in this partition.
- The HTML language server extension (within `extensions/`) is referenced in-comment at line 42 as a consumer of `typescript.js` and `typescript.d.ts`; its implementation is in a separate partition.
- Extension editing infrastructure (referenced in-comment at line 42) — separate partition; consumes the preserved `typescript.js` API surface.
