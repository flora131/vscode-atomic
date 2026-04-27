# Online Research: `.eslint-plugin-local/` — Custom ESLint Rules

(no external research applicable)

## Justification

The `.eslint-plugin-local/` directory (49 files) contains entirely internal, project-specific ESLint rules written in TypeScript using `eslint` and `@typescript-eslint/utils`. These rules encode VS Code's architectural invariants at the tooling level:

- **`code-layering.ts`** — enforces the `common / worker / browser / electron-browser / node / electron-utility / electron-main` layer dependency graph by inspecting import paths at lint time.
- **`code-import-patterns.ts`** — enforces ESM-compatible import conventions and module boundary restrictions keyed to VS Code's `src/vs/` source tree, with explicit awareness of `electron-browser`, `electron-main`, and `electron-utility` layers.
- **`code-no-standalone-editor.ts`**, **`code-no-nls-in-standalone-editor.ts`**, **`code-no-static-node-module-import.ts`**, etc. — guard against platform-specific leakage (Node built-ins, Electron APIs, global `document` listeners) into layers that must remain portable.
- **`vscode-dts-*.ts`** rules — enforce API surface contracts specific to `vscode.d.ts`.

None of these files reference external libraries that are central to a Tauri/Rust port. Their only runtime dependencies are `eslint`, `@typescript-eslint/utils`, `minimatch`, and `glob` — all standard ESLint plugin tooling. There is no documentation from third-party services, cloud providers, or framework vendors that would be illuminating here.

### Relevance to a Tauri/Rust port assessment

The ESLint plugin is a *meta-artifact*: it statically enforces the architectural invariants of the existing TypeScript codebase rather than implementing any IDE functionality. For a port to Tauri/Rust:

- The rules themselves would not be carried over, since Rust uses `clippy` and its own linting ecosystem rather than ESLint.
- However, the *invariants the rules encode* are highly informative. The layering enforced by `code-layering.ts` and `code-import-patterns.ts` reveals the deliberate separation between `common` (platform-agnostic), `browser` (DOM-dependent), `node` (Node.js-dependent), and `electron-*` (Electron IPC and main/renderer split) concerns. Any port must decide which of those layers can be re-implemented in Rust (e.g., `common` logic, `node`-layer services) versus which must be bridged into the Tauri WebView frontend (e.g., `browser`/`electron-browser` rendering logic).
- The `hasElectron`, `hasNode`, and `hasBrowser` conditional layer flags in `code-import-patterns.ts` map almost directly onto the Tauri architectural split: Rust backend (replacing `electron-main` / `electron-utility` / `node`) versus WebView frontend (replacing `browser` / `electron-browser`).

No external web research is needed or applicable for this scope. The relevant information is entirely contained within the plugin source files themselves and the architectural wiki link (`https://github.com/microsoft/vscode/wiki/Source-Code-Organization`) already cited inline in the rule metadata — which is a documentation resource about VS Code's own source layout, not about Tauri or Rust.

## Summary

The `.eslint-plugin-local/` directory is a set of project-internal lint rules that enforce VS Code's TypeScript architectural invariants. It has no dependency on third-party libraries central to a Tauri/Rust port, and no external documentation is needed to assess it. Its significance to a porting effort is indirect but real: the layer model it encodes (`common`, `worker`, `browser`, `electron-browser`, `node`, `electron-utility`, `electron-main`) defines the boundary lines that any Tauri port would need to respect or redesign — the `electron-*` and `node` layers would migrate into Rust/Tauri backend code, while the `browser` and `electron-browser` layers would remain as WebView-side TypeScript/JavaScript. The plugin itself is purely a TypeScript tooling artifact and will not be ported.
