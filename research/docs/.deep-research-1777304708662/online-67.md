# Online Research — Partition 67 of 79

**Scope:** `src/cli.ts` (1 file, 26 LOC)

**Research question:** What it would take to port VS Code's core IDE functionality from TypeScript/Electron to Tauri/Rust.

---

(no external research applicable)

`src/cli.ts` is a 26-line CLI bootstrapper that performs NLS (National Language Support) setup, portable-mode configuration, ESM bootstrap, and then delegates entirely to `src/vs/code/node/cli.ts`. Its only imports are internal VS Code bootstrap utilities (`bootstrap-cli`, `bootstrap-node`, `bootstrap-esm`, `bootstrap-meta`). There are no external libraries, frameworks, or runtime APIs in this file whose documentation would be relevant to understanding or planning a Tauri/Rust port. The file's sole purpose is process-entry orchestration using VS Code-private modules, so no external library documentation is central to this scope.
