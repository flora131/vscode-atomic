### Files Analysed

| File | LOC read | Purpose |
|---|---|---|
| `src/cli.ts` | 26 (full) | Desktop CLI entry point |
| `src/bootstrap-cli.ts` | 12 (full) | Earliest environment cleanup |
| `src/bootstrap-node.ts` | 190 (full) | Node.js environment configuration |
| `src/bootstrap-esm.ts` | 113 (full) | ESM module system setup and NLS loading |
| `src/bootstrap-meta.ts` | 55 (full) | Product/package metadata loader |
| `src/vs/base/node/nls.ts` | 60 (partial) | NLS configuration resolution |
| `src/vs/code/node/cli.ts` | 80 (partial, out-of-partition cross-ref) | Main CLI command dispatcher |
| `cli/src/bin/code/main.rs` | 80 (partial, out-of-partition cross-ref) | Rust CLI entry point (existing port) |

---

### Per-File Notes

#### `src/cli.ts`

- **Role:** Top-level Node.js entry point for the VS Code desktop CLI (`code` binary). It is the file Node.js executes when the user runs `code` from a terminal. Its sole job is to sequence five bootstrap phases and then delegate all actual CLI logic to `src/vs/code/node/cli.ts`.

- **Key symbols:**
  - `nlsConfiguration` (`src/cli.ts:13`) — awaited result of `resolveNLSConfiguration()`; typed `INLSConfiguration`.
  - `process.env['VSCODE_NLS_CONFIG']` (`src/cli.ts:14`) — environment variable set to the JSON-serialised NLS config; consumed by `bootstrapESM` in the same process.
  - `process.env['VSCODE_CLI']` (`src/cli.ts:20`) — sentinel flag set to `'1'`; downstream code checks this to know it is running in CLI (not Electron renderer) mode.
  - Top-level `await` at lines 13, 23, and 26 — the file relies on ESM top-level-await, which is why it must be compiled as an ES module.

- **Control flow:**
  1. Line 6: `import './bootstrap-cli.js'` — side-effect import; deletes `VSCODE_CWD` from the process environment before any other import can read it (`src/bootstrap-cli.ts:11`).
  2. Line 7: `import { configurePortable } from './bootstrap-node.js'` — pulls in the portable-mode helper. `bootstrap-node.ts` also immediately runs `setupCurrentWorkingDirectory()` at module load time (`src/bootstrap-node.ts:55`), configuring `VSCODE_CWD` and (on Windows) changing the working directory to the application folder.
  3. Line 8: `import { bootstrapESM } from './bootstrap-esm.js'` — pulls in the ESM setup module. `bootstrap-esm.ts` conditionally registers an ESM loader hook to remap `fs` to `original-fs` when running under Electron (`src/bootstrap-esm.ts:14-29`), and it populates three global variables: `_VSCODE_PRODUCT_JSON`, `_VSCODE_PACKAGE_JSON`, and `_VSCODE_FILE_ROOT` (`src/bootstrap-esm.ts:33-35`).
  4. Lines 9-10: Imports `resolveNLSConfiguration` and `product` — both are pure data-loading utilities.
  5. Line 13: `await resolveNLSConfiguration(...)` — resolves NLS config for locale `'en'`; hardcodes `userLocale` and `osLocale` to `'en'`, passes `product.commit`, empty `userDataPath`, and `import.meta.dirname` as `nlsMetadataPath`.
  6. Line 14: Stores the NLS config in `process.env['VSCODE_NLS_CONFIG']` as a JSON string so that `bootstrapESM` can read it via environment variable when it later calls `doSetupNLS()`.
  7. Line 17: `configurePortable(product)` — inspects the filesystem for a `data/` directory next to the application root and, if found, sets `VSCODE_PORTABLE`, `TMP`/`TEMP`/`TMPDIR` appropriately (`src/bootstrap-node.ts:133-190`).
  8. Line 20: Sets `VSCODE_CLI=1` — a process-wide signal.
  9. Line 23: `await bootstrapESM()` — triggers `setupNLS()` inside `bootstrap-esm.ts`, which reads the NLS messages file from disk and populates `globalThis._VSCODE_NLS_MESSAGES` (`src/bootstrap-esm.ts:108-112`, `src/bootstrap-esm.ts:49-103`).
  10. Line 26: `await import('./vs/code/node/cli.js')` — dynamic import of the real CLI dispatcher; execution transfers there permanently.

- **Data flow:**
  - `product.json` (or patched build literal) → `bootstrap-meta.ts` → `product` export → `src/cli.ts:10` → passed to `resolveNLSConfiguration` (commit field) and `configurePortable` (portable/win32VersionedUpdate fields).
  - NLS resolution output → `process.env['VSCODE_NLS_CONFIG']` (string) → consumed by `bootstrap-esm.ts:doSetupNLS()` → `globalThis._VSCODE_NLS_MESSAGES` and `globalThis._VSCODE_NLS_LANGUAGE`.
  - Portable data path → `process.env['VSCODE_PORTABLE']` and temp dir env vars → available to all subsequently loaded modules.
  - `VSCODE_CLI=1` → process environment → checked by `src/vs/code/node/cli.ts` and potentially by child processes.

- **Dependencies:**
  - `./bootstrap-cli.js` (`src/bootstrap-cli.ts`) — side-effect only; no exported symbols used.
  - `./bootstrap-node.js` (`src/bootstrap-node.ts`) — `configurePortable` function; side-effect `setupCurrentWorkingDirectory()`.
  - `./bootstrap-esm.js` (`src/bootstrap-esm.ts`) — `bootstrapESM` async function; side-effect: sets three globals.
  - `./vs/base/node/nls.js` (`src/vs/base/node/nls.ts`) — `resolveNLSConfiguration` async function.
  - `./bootstrap-meta.js` (`src/bootstrap-meta.ts`) — `product` (typed `Partial<IProductConfiguration>`), loaded from `product.json`.
  - Node.js built-ins accessed indirectly: `node:fs`, `node:path`, `node:module`, `node:os`.
  - Runtime: Node.js ESM with top-level-await support (requires `--input-type=module` or `.mjs`/`package.json` `"type":"module"`).

---

### Cross-Cutting Synthesis

`src/cli.ts` is a pure sequencing shim — 26 lines that exist solely to impose a strict ordering on five side-effectful bootstrap modules before yielding to the real CLI in `src/vs/code/node/cli.ts`. Its Tauri/Rust port implications fall into three categories:

1. **Environment variable protocol.** The entire init chain communicates through `process.env` keys: `VSCODE_CWD`, `VSCODE_CLI`, `VSCODE_NLS_CONFIG`, `VSCODE_PORTABLE`, `VSCODE_DEV`, `ELECTRON_RUN_AS_NODE`. A Tauri port would need to replicate this environment-variable contract, either by porting the same keys into Rust `std::env` or by replacing them with a first-class configuration struct passed through function arguments. The existing Rust CLI (`cli/src/bin/code/main.rs`) already uses `std::env::args_os()` and builds a `CommandContext` struct, which is the Rust idiomatic equivalent of this env-var protocol.

2. **NLS subsystem.** NLS is wired in three steps across two files (resolve config → set env var → load messages file). In Rust this would either be folded into a single synchronous initialisation call using `std::fs::read_to_string` or delegated to a locale crate. The hardcoded `'en'` locale values at `src/cli.ts:13` suggest the CLI path skips language-pack loading for non-English locales, simplifying the Rust equivalent.

3. **ESM loader hook.** The `fs`→`original-fs` remapping in `bootstrap-esm.ts:14-29` is Electron-specific and disappears entirely in a Tauri host, where there is no `original-fs` concept. Portable mode logic (`src/bootstrap-node.ts:133-190`) would need to be re-expressed as Rust path resolution logic, but the algorithm (look for `data/` sibling directory, conditionally set temp paths) is straightforward to translate.

Overall, `src/cli.ts` and its bootstrap layer represent a thin, replaceable glue layer. The real porting cost lies in the downstream `src/vs/code/node/cli.ts` command dispatcher and its deep dependency tree, not in the 26-line entry point itself.

---

### Out-of-Partition References

- `src/vs/code/node/cli.ts` — real CLI command dispatcher; loaded dynamically at `src/cli.ts:26`; handles argument parsing, tunnel/extension/profiler subcommands, and Electron process spawning.
- `src/vs/platform/environment/node/argv.ts` — defines `OPTIONS`, `NATIVE_CLI_COMMANDS`, `buildHelpMessage`, `buildVersionMessage`; used by `src/vs/code/node/cli.ts:18`.
- `src/vs/platform/environment/node/argvHelper.ts` — exports `parseCLIProcessArgv`, `addArg`; used by `src/vs/code/node/cli.ts:19`.
- `src/vs/base/node/nls.ts` — `resolveNLSConfiguration` async function; called at `src/cli.ts:13`.
- `src/bootstrap-meta.ts` — `product` and `pkg` exports; consumed at `src/cli.ts:10`.
- `src/bootstrap-esm.ts` — `bootstrapESM` function and ESM loader hook; called at `src/cli.ts:23`.
- `src/bootstrap-node.ts` — `configurePortable`, `removeGlobalNodeJsModuleLookupPaths`, `devInjectNodeModuleLookupPath`; `configurePortable` called at `src/cli.ts:17`.
- `src/bootstrap-cli.ts` — side-effect-only module; deletes `VSCODE_CWD`; imported first at `src/cli.ts:6`.
- `cli/src/bin/code/main.rs` — Rust equivalent entry point; already exists in the `cli/` subtree; uses `clap` for argument parsing and `tokio` async runtime instead of Node.js top-level-await.
- `cli/src/commands/args.rs` — Rust argument type definitions (`AnyCli`, `IntegratedCli`, `StandaloneCli`, `Commands`); structural counterpart to `src/vs/platform/environment/node/argv.ts`.
