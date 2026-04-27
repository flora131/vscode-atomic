# Online Research: Porting `src/bootstrap-node.ts` to Tauri/Rust

(no external research applicable)

## Justification

`src/bootstrap-node.ts` (190 LOC) imports exclusively from Node.js built-in modules — `node:path`, `node:fs`, and `node:module` — and patches internal `Module` APIs (`Module._resolveLookupPaths`, `Module._nodeModulePaths`) to control where Node resolves packages at runtime. There are no third-party libraries, no npm dependencies, and no external services involved. Because the entire surface area of this file is Node.js platform APIs and V8/process globals, there is nothing external to research; the porting challenge is purely an analysis of how to replicate or abandon each Node-specific behaviour in a Tauri/Rust host, which falls under code-level analysis rather than external library research.

## What the file actually does (code-level summary for porting context)

| Concern | Node.js mechanism used | Tauri/Rust equivalent notes |
|---|---|---|
| Increase V8 stack-frame limit | `Error.stackTraceLimit = 100` | Not applicable; Rust panics and backtraces are configured separately via `RUST_BACKTRACE` env var or `tracing` crate. |
| Suppress broken-pipe noise | `process.on('SIGPIPE', ...)` | Rust handles `SIGPIPE` at the OS level; the standard idiom is to ignore it with `unsafe { libc::signal(libc::SIGPIPE, libc::SIG_IGN); }` or let the write return an `Err(BrokenPipe)`. |
| Persist working directory in env | `process.env['VSCODE_CWD'] = process.cwd()` | Direct equivalent: `std::env::set_var("VSCODE_CWD", std::env::current_dir()?)`. |
| Force cwd to app folder on Windows | `process.chdir(path.dirname(process.execPath))` | `std::env::set_current_dir(std::env::current_exe()?.parent()?)`. |
| Dev-mode module redirection | `Module.register('./bootstrap-import.js', ...)` — Node 18+ loader hook | No equivalent in Tauri's webview runtime; dev-mode asset serving would be handled by Tauri's `devUrl` config or a custom asset protocol handler. |
| Strip global Node module lookup paths | Monkey-patching `Module._resolveLookupPaths` and `Module._nodeModulePaths` | Entirely Node-specific. In a Tauri build all JS is bundled (esbuild/webpack/vite), so runtime `require()` resolution does not occur and this concern disappears. |
| Portable-mode detection | `fs.existsSync(portableDataPath)` + env-var manipulation | `std::path::Path::exists()` + `std::env::set_var`/`remove_var`. Straightforward 1-to-1 port. |
| Platform-branching (`win32`, `darwin`, `linux`) | `process.platform` | `std::env::consts::OS` returns `"windows"`, `"macos"`, `"linux"` respectively. |

## Porting assessment

`bootstrap-node.ts` is a thin process-setup shim. The concerns it handles — signal handling, cwd normalisation, portable-data detection, and module-loader patching — map cleanly onto Rust standard-library primitives (`std::env`, `std::fs`, `std::path`, `libc`). The only structurally non-trivial concern is the Node module-loader patching (`devInjectNodeModuleLookupPath`, `removeGlobalNodeJsModuleLookupPaths`), which exists solely because VS Code ships a Node.js runtime and dynamically resolves CommonJS modules at startup. A Tauri port would replace Node's dynamic module graph with a compile-time bundle, making those two exported functions entirely obsolete. The portable-mode logic and the SIGPIPE/cwd setup would each require fewer than 30 lines of idiomatic Rust.

The file presents no external-library research questions. The porting work is a straightforward mechanical translation of Node/V8 platform APIs to their `std` and `libc` counterparts, with the module-patching functions simply being deleted because the architectural premise (a live Node runtime resolving CJS packages at process start) does not exist in Tauri.
