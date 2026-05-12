# Porting VS Code Core IDE Functionality from TypeScript/Electron to Tauri/Rust

## Scope Note

The assigned partition (`extensions/postinstall.mjs`, 58 LOC) is a post-install cleanup script that removes unnecessary TypeScript package files. It has zero external dependencies relevant to a Tauri/Rust porting effort. However, the overarching research question — what it would take to port VS Code's core IDE functionality to Tauri/Rust — has rich, directly applicable external evidence, especially from the SideX project (a live open-source Tauri port of VS Code), the Tauri 2.0 documentation, and the VS Code extension host architecture docs. The findings below synthesize those sources.

---

## Summary

Porting VS Code from Electron/Node.js to Tauri/Rust is technically feasible but represents a major multi-year engineering undertaking. The TypeScript workbench (Monaco editor, UI panels, file explorer) can be reused almost verbatim since Tauri still renders a web frontend. The hard work is replacing Electron's Node.js backend with Rust commands that provide equivalent file I/O, terminal PTY, Git integration, file watching, and process management. The deepest unsolved challenge is the **extension host**: VS Code extensions depend heavily on Node.js APIs and npm packages, and rebuilding that runtime compatibility layer in Rust or as a sandboxed sidecar process remains incomplete in every known attempt as of May 2026.

---

## Detailed Findings

### 1. Tauri 2.0 Architecture

**Source**: [Tauri Full Documentation / llms.txt](https://tauri.app/llms.txt) — [What is Tauri?](https://v2.tauri.app/start) — [Tauri Architecture](https://v2.tauri.app/concept/architecture/) — [Inter-Process Communication](https://v2.tauri.app/concept/inter-process-communication/)

**Relevance**: Authoritative reference for understanding what Tauri provides as a replacement for Electron.

**Key Information**:

- Tauri replaces Electron's bundled Chromium with the OS-native webview: WKWebView on macOS, WebView2 on Windows, WebKitGTK on Linux. This is the single change responsible for the dramatic binary size difference (VS Code: 775 MB installed; SideX Tauri build: ~31 MB).
- The frontend (HTML/CSS/TypeScript) is unchanged. Calls that previously went to Electron's `ipcMain`/`ipcRenderer` are replaced with Tauri's `invoke()` + event system.
- The backend is Rust. Tauri 2.0 also allows plugin code in Swift (macOS/iOS) and Kotlin (Android) for tighter OS integration.
- IPC is structured around **commands** (JS → Rust function calls) and **events** (bidirectional push notifications). This maps directly to how Electron's main-process handlers work.
- Tauri 2.0 (stable, released October 2024) adds mobile support and a refined capability/permission system for fine-grained sandboxing.

**IPC Mapping (Electron → Tauri)**:

| VS Code / Electron concept | Tauri equivalent |
|---|---|
| `ipcMain.handle(channel, handler)` | `#[tauri::command] fn handler(...)` registered in `tauri::Builder::invoke_handler` |
| `ipcRenderer.invoke(channel, ...args)` | `invoke('handler', { ...args })` from `@tauri-apps/api` |
| `BrowserWindow` | `WebviewWindow` |
| Node.js `fs`, `path`, `child_process` | Rust `std::fs`, `std::process`, crates like `tokio`, `portable-pty` |
| `node-pty` (native terminal) | `portable-pty` Rust crate |

---

### 2. SideX — The Live Tauri Port of VS Code

**Source**: [GitHub — Sidenai/sidex](https://github.com/Sidenai/sidex) — [DEV Community writeup](https://dev.to/kendallbooker/i-rebuilt-vs-code-on-tauri-instead-of-electron-and-just-open-sourced-it-53ao) — [HackerNews discussion](https://news.ycombinator.com/item?id=47657699)

**Relevance**: The only known public, open-source, production-quality Tauri port of VS Code. Its README provides the exact architectural mapping and its status page reveals what remains unsolved.

**Key Information**:

SideX preserves VS Code's layered TypeScript workbench (`src/vs/{base,platform,editor,workbench}`) and replaces the Electron process with a Tauri Rust backend in `src-tauri/src/commands/`. The Rust side covers 49 commands across 9 modules:

- **fs** — file I/O via `std::fs`
- **terminal** — full PTY via `portable-pty` crate (replacing `node-pty`)
- **git** — 17 Git commands (status, diff, log, branch, stash, push/pull/fetch, clone) implemented directly in Rust
- **search** — parallel full-text search using `dashmap` + `rayon` + `regex`
- **file watching** — `notify` crate (FSEvents on macOS)
- **storage** — SQLite via `rusqlite`
- **debug**, **process management**, etc.

**What is working** (solid as of early 2025):
- Monaco editor with syntax highlighting and basic IntelliSense
- File explorer (open/create/rename/delete)
- Integrated terminal (full PTY, shell detection, resize, signals)
- Git integration (full status/diff/log/stage/commit workflow)
- Built-in VS Code themes
- Native OS menus (macOS/Windows/Linux)
- Extension installation from the Open VSX registry
- File watching, file search, full-text search (Rust-backed)
- SQLite storage, document management (autosave, undo/redo, encoding)

**What is NOT working**:
- **Extension host** — listed as "sidecar process, in progress." This is the most critical gap.
- **Debugger** — also in progress.
- Extension compatibility for third-party extensions that use Node.js APIs or npm packages is unresolved.

**Tech Stack in SideX**:

| Layer | Technology |
|---|---|
| Frontend | TypeScript, Vite 6, Monaco Editor |
| Terminal UI | xterm.js + WebGL renderer |
| Syntax / Themes | vscode-textmate, vscode-oniguruma (WASM) |
| Backend | Rust, Tauri 2 |
| Terminal | portable-pty (Rust crate) |
| File watching | notify crate |
| Search | dashmap + rayon + regex |
| Storage | SQLite via rusqlite |
| Extensions | Open VSX registry |

---

### 3. VS Code Extension Host Architecture — The Core Porting Challenge

**Source**: [VS Code Extension Host docs](https://code.visualstudio.com/api/advanced-topics/extension-host) — [Rust Forum discussion on VS Code-like extension system](https://users.rust-lang.org/t/is-it-possible-to-create-an-extension-system-similar-to-vs-code-using-rust-and-tauri/77660)

**Relevance**: The extension host is the single most complex subsystem to replace and is the primary reason a full Tauri port is extremely difficult.

**Key Information**:

VS Code runs extensions in a separate **Extension Host process** — a dedicated Node.js process that:
1. Loads and manages extension lifecycle (activation events, deactivation)
2. Provides the `vscode` API surface to extensions
3. Runs Language Server Protocol (LSP) clients and servers as child processes
4. Isolates extension crashes from the editor UI

VS Code actually supports three extension host types:
- **Local Node.js host**: runs extensions on the same machine as the UI (the standard desktop case)
- **Web host**: runs extensions in a browser Web Worker (no Node.js, restricted API surface)
- **Remote Node.js host**: runs in a container or remote machine (used by SSH Remote, Dev Containers)

The porting challenge is that most third-party extensions use at least one Node.js module or npm package, particularly for language features (LSP servers run as Node.js subprocesses). The web extension host model exists in VS Code already and handles the subset of extensions that can run without Node.js — but language extensions (TypeScript, Python, C++, etc.) almost universally require the Node.js host.

A Tauri port has three options for the extension host:
1. **Run a Node.js sidecar** alongside the Tauri process — this is what SideX is building. It preserves extension compatibility but partially negates the memory/size benefits since Node.js still ships.
2. **Web Worker host only** — drop Node.js entirely, only run extensions compatible with the web host model. Loses most language extensions.
3. **Rewrite the extension host in Rust** — expose a Rust API surface compatible with the `vscode` extension API. Theoretically possible but would require re-implementing hundreds of extension APIs and would break binary compatibility with compiled native extensions.

The Rust forum thread specifically confirms: "Building a VS Code-like extension system in Rust/Tauri would be a massive undertaking."

---

### 4. Language Server Protocol (LSP) in a Tauri/Rust Context

**Source**: [Official LSP Specification](https://microsoft.github.io/language-server-protocol/) — [tower-lsp crate](https://github.com/ebkalderon/tower-lsp) — [rust-analyzer LSP integration](https://rust-analyzer.github.io/)

**Relevance**: LSP is how VS Code delivers language intelligence (autocomplete, go-to-definition, diagnostics). In a Tauri port, the Rust backend must either spawn LSP servers as child processes or implement LSP clients natively.

**Key Information**:

LSP defines a JSON-RPC protocol between an editor (client) and a language server. The client side in VS Code is implemented in TypeScript extensions; the server side runs as a separate process (Node.js, Python, Rust binary, etc.).

In a Tauri port:
- **Spawning LSP servers** is straightforward via Rust's `std::process::Command` or `tokio::process`. The Rust backend spawns the language server binary and proxies JSON-RPC messages between it and the TypeScript frontend via Tauri events.
- **Implementing an LSP client in Rust** is supported by the `tower-lsp` crate, which provides async Rust scaffolding for both LSP clients and servers.
- **rust-analyzer** is the production example of a Rust-native LSP server, demonstrating that complex language intelligence can be delivered without Node.js.

The LSP client in VS Code's extension host is the piece that requires Node.js. If the extension host is replaced by a Rust sidecar that speaks LSP natively, language extensions could potentially be decoupled from Node.js entirely — but this requires rebuilding the LSP client infrastructure, not just shelling out.

---

### 5. Tauri vs. Electron: Performance and Feasibility Assessment

**Source**: [Tauri vs Electron — reintech.io](https://reintech.io/blog/tauri-vs-electron-rust-desktop-apps) — [OpenReplay comparison](https://blog.openreplay.com/comparing-electron-tauri-desktop-applications/) — [Tauri 2.0 Stable Release blog](https://v2.tauri.app/blog/tauri-20/)

**Relevance**: Motivating factors for the port and known limitations.

**Key Information**:

| Factor | Electron (VS Code) | Tauri port |
|---|---|---|
| Installer size | ~100–150 MB | ~10–35 MB |
| RAM at idle (macOS) | 150–300 MB | ~30–50 MB target (unverified at full feature parity) |
| Startup time | 1–2 s typical | < 0.5 s typical |
| Security model | Node.js sandbox (coarse) | Tauri capability/permission system (fine-grained) |
| Extension compatibility | Full (Node.js host) | Partial (web host only, or Node.js sidecar) |
| Cross-platform | Windows, macOS, Linux | Windows, macOS, Linux, iOS, Android (Tauri 2.0) |

Adoption grew 35% year-over-year after Tauri 2.0 (October 2024). The 2.0 release involved 2,870+ contributor work-hours and is considered production-stable.

Key limitation from community analysis: if a developer's workflow requires many Node.js-dependent extensions (language servers, debuggers), the RAM savings disappear once the Node.js sidecar starts. The efficiency gains are most pronounced for users with a minimal extension set.

---

## Gaps and Limitations

1. **Extension host compatibility** is the primary unsolved problem. No publicly available Tauri port (including SideX) has achieved full backward-compatible extension host parity as of May 2026.
2. **WebView2 on Windows** has nuanced memory behavior. The "WKWebView is shared with Safari" model that produces macOS savings does not translate cleanly to Windows because WebView2 has per-app memory overhead in some measurement contexts.
3. **Debugger integration** — VS Code's Debug Adapter Protocol (DAP) layer is also a sidecar-dependent feature not yet implemented in SideX.
4. **vscode-textmate / oniguruma** — both already run as WASM in SideX, so syntax highlighting is resolved without Rust reimplementation.
5. **Native extensions** (compiled `.node` addons) — these have no path forward without Node.js. They would require Rust reimplementation or WASM ports.

---

## Additional Resources

- [SideX GitHub Repository](https://github.com/Sidenai/sidex) — live Tauri port of VS Code
- [Tauri 2.0 Documentation Index / llms.txt](https://tauri.app/llms.txt)
- [Tauri Inter-Process Communication](https://v2.tauri.app/concept/inter-process-communication/)
- [VS Code Extension Host](https://code.visualstudio.com/api/advanced-topics/extension-host)
- [tower-lsp — Rust LSP crate](https://github.com/ebkalderon/tower-lsp)
- [portable-pty — Rust PTY crate](https://github.com/wez/wezterm/tree/main/pty) (used in SideX)
- [rust-analyzer LSP integration](https://deepwiki.com/rust-lang/rust-analyzer/3-language-server-protocol-integration)
- [Tauri Rust Forum: VS Code-like extension system](https://users.rust-lang.org/t/is-it-possible-to-create-an-extension-system-similar-to-vs-code-using-rust-and-tauri/77660)
- [Tauri 2.0 Stable Release announcement](https://v2.tauri.app/blog/tauri-20/)
- [HackerNews: SideX discussion](https://news.ycombinator.com/item?id=47657699)

---

## Prose Summary

The assigned partition (`extensions/postinstall.mjs`) is irrelevant to porting research — it is a pure cleanup utility with no external dependencies. The broader research question, however, has substantial and directly applicable external evidence. The SideX project demonstrates the architectural mapping concretely: the TypeScript workbench transfers almost unchanged, while the Electron main process is replaced by a Rust Tauri backend that re-implements file I/O, terminal PTY (`portable-pty`), Git commands, file watching (`notify` crate), full-text search (`rayon` + `regex`), and SQLite storage. Tauri 2.0's `invoke()` + event IPC system is a clean structural replacement for Electron's `ipcMain`/`ipcRenderer`. The one area where no Tauri port has yet succeeded is the **extension host**: VS Code extensions depend on a Node.js runtime environment (the `vscode` API, LSP client infrastructure, native `.node` addons, and arbitrary npm packages), and replicating or replacing that environment in Rust would itself be a multi-year project of comparable scope to building a new runtime. Every current approach either runs Node.js as a sidecar (partially preserving the memory problem) or restricts to the web extension host model (losing most language extensions).
