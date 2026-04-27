### Files Analysed

- `/Users/norinlavaee/vscode-atomic/extensions/postinstall.mjs` (58 LOC)

---

### Per-File Notes

#### `/Users/norinlavaee/vscode-atomic/extensions/postinstall.mjs`

**Role**

A Node.js ES-module post-install script that prunes the `typescript` npm package installed inside `extensions/node_modules/typescript/`. Its purpose is to strip files that are not required by extension-editing or the HTML language service, reducing the disk footprint of the bundled TypeScript dependency.

**Key Symbols**

| Symbol | Line | Description |
|---|---|---|
| `root` | 10 | Absolute path to `extensions/node_modules/typescript`, resolved via `import.meta.url` → `fileURLToPath` → `path.dirname` → `path.join`. |
| `processRoot()` | 12–24 | Removes everything in the TypeScript package root except the entries in the `toKeep` set: `lib` (directory) and `package.json`. All other top-level entries—including the TypeScript compiler CLI binary—are deleted with `fs.rmSync(..., { recursive: true })`. |
| `toKeep` | 13–16 | `Set<string>` with members `'lib'` and `'package.json'`. Any top-level entry not in this set is deleted. |
| `processLib()` | 26–55 | Selectively removes files from `extensions/node_modules/typescript/lib/`. Two groups of files are preserved: (1) `lib.d.ts`, any file matching `lib.*.d.ts`, and `protocol.d.ts`—these are the standard TypeScript library type-definition files; (2) `typescript.js` and `typescript.d.ts`—the language-service API used by HTML language features and extension editing. Everything else in the `lib/` directory that either matches the `toDelete` set or ends in `.d.ts` is deleted. |
| `toDelete` | 27–33 | `Set<string>` containing `'tsc.js'`, `'_tsc.js'`, `'typescriptServices.js'`, `'_typescriptServices.js'`. These are the TypeScript compiler entrypoints not needed at runtime. |

**Control Flow**

1. `root` is computed at module level (line 10).
2. `processRoot()` is called first (line 57).
   - Reads the top-level directory of the TypeScript package (`fs.readdirSync(root)`).
   - For each entry not in `toKeep`, calls `fs.rmSync(filePath, { recursive: true })` and logs the removal.
3. `processLib()` is called second (line 58).
   - Reads `root/lib` via `fs.readdirSync(libRoot)`.
   - For each file it applies three sequential guards:
     a. Lines 38–39: `continue` (preserve) if the name is `lib.d.ts`, matches `/^lib\..*\.d\.ts$/`, or is `protocol.d.ts`.
     b. Lines 41–44: `continue` (preserve) if the name is `typescript.js` or `typescript.d.ts`.
     c. Lines 46–53: Delete via `fs.unlinkSync` if the name is in `toDelete` OR matches `/\.d\.ts$/`. Deletion errors are caught and logged as warnings (line 51), not thrown.

**Data Flow**

- Input: filesystem state of `extensions/node_modules/typescript/` at the time the script runs.
- Output: a pruned `extensions/node_modules/typescript/` directory containing only:
  - `package.json` (top level)
  - `lib/lib.d.ts`, `lib/lib.*.d.ts`, `lib/protocol.d.ts` (type definitions for language services)
  - `lib/typescript.js`, `lib/typescript.d.ts` (runtime language-service API)
- Side effects: console output (`console.log` for every removed path, `console.warn` on `unlink` failure).

**Dependencies**

- Node.js built-ins only: `fs` (line 6), `path` (line 7), `url.fileURLToPath` (line 8).
- No third-party npm packages are imported.
- Implicitly depends on the presence of `extensions/node_modules/typescript/` having been installed before the script runs (i.e., it is a post-install hook).

**Invocation**

The script is a standalone ESM module (`import.meta.url` at line 10). It is typically wired as the `postinstall` lifecycle script in `extensions/package.json`, making it run automatically after `npm install` completes inside the `extensions/` workspace.

---

### Cross-Cutting Synthesis

`postinstall.mjs` is a build-time artifact management script, not part of any runtime VS Code subsystem. It enforces a deliberate split within the `typescript` npm package: only the language-service surface (`typescript.js` / `typescript.d.ts`) and the standard library type definitions (`lib.*.d.ts`, `protocol.d.ts`) survive; the TypeScript compiler CLI entrypoints (`tsc.js`, `typescriptServices.js`, etc.) are removed. This distinction is meaningful in the context of a Tauri/Rust port: it reveals that VS Code extensions depend on TypeScript's language-service API at runtime (for editor intelligence and HTML language features), not on the full compiler toolchain. Any port would need to either bundle an equivalent language-service binary, expose it via IPC from a native process, or replace it entirely with a Rust-native language server. The script itself—a thin Node.js ESM file using only built-in `fs`/`path` APIs—has no Electron, DOM, or VS Code API dependencies and therefore requires no porting; its build-time pruning role would simply be replicated by equivalent Tauri/Cargo build scripts or asset pipeline steps.

---

### Out-of-Partition References

- `extensions/package.json` — expected to declare `"postinstall": "node postinstall.mjs"` (or equivalent), wiring this script into the npm lifecycle. Not read in this partition.
- `extensions/node_modules/typescript/` — the TypeScript npm package whose layout this script assumes. The preserved files (`typescript.js`, `typescript.d.ts`, `lib.d.ts`, `lib.*.d.ts`, `protocol.d.ts`) are consumed by the HTML language service and extension-editing features elsewhere in the `extensions/` tree.
- HTML language service extension — referenced by inline comment at line 43 (`// used by html and extension editing`). Exact extension path not located in this partition.
- Extension editing infrastructure — also referenced by line 43; consumes `typescript.js` and `typescript.d.ts` for IntelliSense support inside extension source files.
