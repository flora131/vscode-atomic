# Porting VS Code's Core IDE Functionality from TypeScript/Electron to Tauri/Rust

**Date:** 2026-05-11
**Branch:** `flora131/feature/vscode-port`
**Repo:** `vscode-atomic` @ `f91a396d242`
**Inputs:** 80-partition deep-research scan of the VS Code monorepo (~338 shard files, ~78K lines of intermediate findings under `research/docs/.deep-research-1778468401377/`).

---

## Executive Summary

A Tauri/Rust port of VS Code is **structurally feasible but ecosystem-bound**. The work splits into three buckets:

1. **Already done (~20K LOC of production Rust):** `cli/` ships today as a Cargo workspace with tokio, hyper, reqwest, oauth2-style auth, keyring storage, dev-tunnels, port-forwarding, async pipes (Unix sockets / Windows named pipes), state persistence, cross-platform service registration (systemd / LaunchAgent / Windows service), and signed builds across Snap, MSIX, deb, rpm, dmg. **This is the seed of the port — not a CLI tool but a headless backend that already runs in production.**
2. **Reusable in-place (~60% of `src/vs/`, ~1.2M LOC of TypeScript):** Monaco editor, the workbench UI, virtually all `editor/` and `workbench/` contributions, and almost every TS-only built-in extension can run unchanged inside a Tauri WebView. Tauri's WebView is a *better* host for these than Electron's renderer because there is no `original-fs`/ASAR layer to work around.
3. **Must be re-implemented (~40% of `src/vs/`, ~800K LOC):** `base/`, `platform/`, the bootstrap chain, the IPC fabric (`src/vs/base/parts/ipc/`), the service/DI runtime, the file watcher, the PTY host, file-system providers, terminal spawning, and the ~200-message extension-host RPC protocol (`src/workbench/api/common/extHost.protocol.ts`). All have clean Rust analogues.

The single load-bearing constraint is **API fidelity**: `src/vscode-dts/vscode.d.ts` (21,235 LOC, 710+ exported interfaces, 19 stable namespaces, ~125 proposed-API files) is the contract every extension and Copilot itself depends on. The conformance suite at `extensions/vscode-api-tests/` (50 files, 11.4K LOC, 12 architectural categories, 62 enabled proposals) is the acceptance gate.

The single load-bearing **risk** is the **extension host**: extensions are JS/TS, often consume Node.js-native (`.node`) addons, and assume a Node.js runtime in the same process. Three viable answers exist: keep Node.js as a sidecar (recommended for MVP — preserves Copilot, Git, Python, ts-server out of the box), embed Deno isolates with Rust-backed `vscode` bindings (24-month rebuild of the API surface), or compile extensions to WASM (ecosystem doesn't exist).

**Recommended strategy:** *extend `cli/` upward into a Tauri host* rather than greenfield. **Estimated effort to feature-parity:** 12–18 months with a team of 3–5 engineers, assuming Node.js sidecar for extension host. **Estimated calendar to MVP** (Monaco + file I/O + terminal + Git + a small set of language servers, no full extension ecosystem): 4–6 months.

---

## 1. Background and Goal

VS Code is a TypeScript/Electron desktop IDE: ~6,048 files / 1.99M LOC inside `src/vs/`, plus ~50 built-in extensions in `extensions/`, plus an existing `cli/` Rust crate (75 files, ~20K LOC) that ships in every build. The port question is whether the Electron+Node.js execution substrate can be replaced by a Tauri+Rust substrate while preserving:

- The public extension API (`src/vscode-dts/`)
- The extension ecosystem (Copilot in particular — `extensions/copilot/` is 2,908 files / 695K LOC built only on the public API)
- The full feature set (editor, language intelligence, debugging, source control, terminal, navigation, notebooks, remote, tunnels)
- Cross-platform packaging and signing

This document is a synthesis of an 80-partition scoping pass that covered every directory in the monorepo. It distills what the move would actually cost, where the floor is, where the ceiling is, and where the trapdoors are.

---

## 2. Current Repository Snapshot (what the deep scan found)

| Layer | Path | Files | LOC | Role |
|---|---|---:|---:|---|
| Core engine | `src/vs/` | 6,048 | 1.99M | base / platform / editor / workbench / code |
| Public API | `src/vscode-dts/` | 174 | 33.6K | extension API (`vscode.d.ts` is 21,235 LOC alone) |
| Existing Rust | `cli/` | 75 | 20.1K | tunnels, auth, server lifecycle, port-forwarding |
| Build / packaging | `build/` + `gulpfile.mjs` | 195+ | 33.6K | gulp + esbuild + electron-builder + signing |
| Bootstrap | `src/main.ts`, `src/bootstrap-*.ts`, `src/server-main.ts`, `src/cli.ts` | 12 | ~1.7K | process entry points |
| Conformance tests | `extensions/vscode-api-tests/` | 50 | 11.4K | 36 test files / 12 categories / 62 proposed APIs |
| Lint invariants | `.eslint-plugin-local/` | 49 | 3.9K | 34 custom rules encoding architecture |
| Copilot (largest extension) | `extensions/copilot/` | 2,908 | 695K | reference workload — uses 54 proposed APIs |
| Git | `extensions/git/` | 62 | 25.2K | shells out to `git`, primary SCM consumer |
| TypeScript LSP host | `extensions/typescript-language-features/` | 168 | 22.6K | tsserver host — heaviest LSP integration |
| Markdown LSP + preview | `extensions/markdown-language-features/` | 88 | 9.6K | LSP + webview preview |
| HTML / JSON / CSS / PHP LSP | `extensions/{html,json,css,php}-language-features/` | ~200 | ~22K | embedded language client pattern |
| Notebooks | `extensions/ipynb/` | 25 | 4.9K | NotebookSerializer for `.ipynb` |
| Notebook renderers | `extensions/notebook-renderers/` | – | 3.5K | webview-side MIME dispatch |
| GitHub / Microsoft auth | `extensions/{github,microsoft}-authentication/` | 55 | 6.7K | OAuth2 / MSAL / device-code |
| Remote resolver pattern | `extensions/vscode-test-resolver/` | 7 | 0.9K | canonical `RemoteAuthorityResolver` reference |
| Tunnel forwarding | `extensions/tunnel-forwarding/` | 4 | 0.5K | TS shim that drives the Rust `code-tunnel` binary |
| Grammar-only language extensions | `extensions/{typescript-basics,php,html,razor,json,cpp,less,yaml,latex,...}/` | – | ~5K | pure TextMate JSON + language-configuration |

**Surface that maps cleanly to the WebView (no port needed beyond bundling):** `src/vs/editor/` (851 files), most of `src/vs/workbench/` (3,395 files), `src/vs/base/common/`, all grammar-only extensions, all theme JSON, all snippet JSON, all language-configuration JSON.

**Surface that requires Rust reimplementation:** `src/vs/base/node/` and `src/vs/base/parts/ipc/`, `src/vs/platform/{files,terminal,configuration,storage,native,...}/` (~874 files), every `electron-main` directory, the bootstrap chain, the file watcher, the PTY host, the secret-storage layer, and the ~200 protocol messages in `src/workbench/api/common/extHost.protocol.ts`.

---

## 3. Recommended Strategy: Extend `cli/` into a Tauri Host

The single most important finding of the scan is that **`cli/` is not just a CLI**. It is a 20K-LOC tokio-async backend that already does the bulk of what an IDE backend has to do:

- Async I/O foundation (`tokio` 1.52, `hyper` 1, `reqwest` 0.13, `tokio-tungstenite`, `rmp-serde`)
- Cross-platform IPC pipes (`async_pipe.rs` — Unix domain sockets / Windows named pipes / TCP fallback, ~300 LOC)
- A pluggable RPC layer with JSON or MessagePack serialization (`rpc.rs`, `json_rpc.rs`, `msgpack_rpc.rs`, ~290 LOC)
- A multiplexed control server (`tunnels/control_server.rs`, ~1.5K LOC) handling 8+ protocol methods (server spawn, FS ops, process execution, HTTP forwarding, port forwarding, challenge-response auth)
- Server install/cache/update infrastructure (`desktop/version_manager.rs`, ~1.2K LOC) with download caching, atomic writes, file locking
- Authentication: device-code OAuth, token refresh, keyring (`auth.rs`, ~760 LOC)
- Cross-platform service registration: systemd via D-Bus (zbus), LaunchAgent on macOS, Windows service helper (~1.5K LOC across 4 files)
- State persistence with file locking (`state.rs`, ~280 LOC)
- Process management (`util/command.rs`, ~180 LOC) — spawn, stdio capture, signal handling, batch kill
- Production-grade error handling (thiserror, error chains, HTTP-status → CodeError mapping)
- Already shipping signed across **Snap, MSIX, deb, rpm, .app**

**Implication:** the right architecture is to **lift `cli/` into the Tauri main process** (or invoke it as a library from `src-tauri/src/main.rs`) and extend it with the platform services that the workbench needs (FileService, TerminalService, ConfigurationService, ExtensionHostService). The existing tunnel/remote/auth subsystems become **first-class services in the Rust core** rather than external binaries the TS layer drives over stdio.

This collapses what would otherwise be a 12–16-week greenfield Rust foundation into a **4–8-week adaptation**. It also means the project ships with auth, tunnels, and remote resolution working on day one because they already do.

---

## 4. Architecture Mapping (Electron → Tauri)

### 4.1 Process Model

Today's process topology (from `explorer-1`, `explorer-38`, `explorer-50`):

```
Electron Main  ─┬─ Renderer (workbench + Monaco)
                ├─ Extension Host (Node.js subprocess)
                ├─ PTY Host (Node.js subprocess)
                ├─ Shared / Utility processes (file watcher, debug adapters)
                └─ Remote Server (src/server-main.ts → HTTP/WS → ext host)
```

Target Tauri topology:

```
Tauri (Rust)   ─┬─ WebView (workbench + Monaco — TS unchanged)
                ├─ Extension Host (Node.js sidecar — preserved for MVP)
                ├─ PTY Host (in-Rust, pty-process / nix::pty crate)
                ├─ File watcher (in-Rust, notify crate)
                ├─ Debug adapters (Rust DAP relay → spawned adapter binaries)
                └─ Remote Server (already cli/ tunnels + control_server)
```

The collapse from "main + renderer + a swarm of utility subprocesses" to "Rust main + WebView + a Node.js sidecar" is the single largest source of memory and startup-time savings. Native modules (PTY, file watcher) move into Rust crates with no runtime cost.

### 4.2 IPC

| Electron pattern | Tauri equivalent | Notes |
|---|---|---|
| `ipcMain.handle('cmd', handler)` | `#[tauri::command] fn cmd(...)` | Add to `tauri::Builder::invoke_handler!` |
| `ipcRenderer.invoke('cmd', arg)` | `invoke('cmd', { arg })` | `@tauri-apps/api/core` |
| Main → renderer event | `window.emit('event', payload)` | `listen()` on the JS side |
| Child process `process.send()` | Sidecar stdio or Unix socket / named pipe | `cli/src/async_pipe.rs` already does this |
| Channel-based RPC (`IChannel`) | Same — Tauri commands + events | Wrap as `Tauri commands → IChannel` adapter |
| MessagePort transfer | Tauri does **not** support MessagePort transfer between windows | Re-route via Rust mediator |

The largest bulk-of-work item under IPC is the **~200 message types** in `src/workbench/api/common/extHost.protocol.ts`. Each becomes a serde-serializable Rust struct + a Tauri command (for invocations) or event (for callbacks). The work is mechanical and parallelizable.

### 4.3 Service / DI

VS Code uses a TypeScript decorator-based DI container (`src/vs/platform/instantiation/common/instantiation.ts`):

```typescript
const IFileService = createDecorator<IFileService>('fileService');
class Foo { constructor(@IFileService private readonly fs: IFileService) {} }
```

The Rust analogue is a trait-object registry behind `Arc<dyn Trait + Send + Sync>`:

```rust
pub trait IFileService: Send + Sync {
    async fn stat(&self, uri: &Uri) -> Result<Stat>;
    async fn read_file(&self, uri: &Uri) -> Result<Vec<u8>>;
}

pub struct Services { file_service: Arc<dyn IFileService>, /* ... */ }
```

Crates like `injector` or `dependency-injection` exist; for a project this size a hand-rolled typemap + `Arc<dyn Trait>` registry is simpler and has zero macro magic. About 20 platform services (`IFileService`, `ITerminalService`, `IStorageService`, `IConfigurationService`, `IDialogService`, `INativeHostService`, `ILoggerService`, `ITelemetryService`, `IEnvironmentService`, etc.) need Rust trait + impl pairs.

### 4.4 Module Loading

VS Code is mid-migration from AMD to ESM. `src/bootstrap-esm.ts` registers an ESM hook that remaps `fs` → `node:original-fs` to defeat Electron's ASAR fs patching (`bootstrap-esm.ts:14–30`). **Under Tauri this entire layer disappears.** No ASAR, no module loader, no fs-remap shim. The deletion alone removes ~600 LOC and an entire class of bootstrap edge cases.

### 4.5 Bootstrap Chain

Today:

```
src/main.ts (Electron app entry)
  → bootstrap-esm.ts (ESM hooks, NLS, product globals)
    → bootstrap-meta.ts (product.json load)
      → bootstrap-node.ts (SIGPIPE, CWD, portable mode)
        → src/vs/code/electron-main/main.ts
          → src/vs/code/electron-main/app.ts (40+ services)
```

Replacement:

```
src-tauri/src/main.rs
  → tauri::Builder::default()
    .invoke_handler(generate_handler![...])
    .setup(|app| { register_services(app); load_nls(); spawn_ext_host(); Ok(()) })
    .run(tauri::generate_context!())
```

Ten of the twelve bootstrap files (`bootstrap-fork.ts`, `bootstrap-node.ts`, `bootstrap-cli.ts`, `bootstrap-server.ts`, `bootstrap-import.ts`, `bootstrap-meta.ts`, `bootstrap-esm.ts`, `bootstrap-server.ts`, `server-cli.ts`, `cli.ts`) become **deleted**. Only `main.ts` and `server-main.ts` have logical successors — `main.rs` for the Tauri app, and the `cli/` server entry for the remote case.

---

## 5. The Five Hard Surfaces

### 5.1 Public Extension API Contract — `src/vscode-dts/`

This is the contract every extension binds to. The shape:

- 174 files, 33.6K LOC; `vscode.d.ts` alone is 21,235 LOC
- 19 stable namespaces (`window`, `workspace`, `languages`, `commands`, `debug`, `scm`, `terminal`, `tasks`, `tests`, `notebooks`, `chat`, `lm`, `env`, `extensions`, `authentication`, `interactive`, `speech`, `ai`, `power`)
- ~125 proposed-API files covering Chat (15), Terminal (9), LM (10), Search (9), Notebook (11), SCM (8), UI (22), and more
- 710+ exported interfaces / classes / enums / type aliases
- ~30 `*Provider` interfaces all returning `ProviderResult<T> = T | Thenable<T>` (sync-or-promise)

**The hardest contracts to translate to Rust** are not data shapes — those are easy. They are the *control-flow idioms*:

1. **`Event<T>` / `EventEmitter<T>`** — Every namespace exposes `on*Did*Change*` events. Extensions `.subscribe(handler)` and get back a `Disposable`. Rust idiom is `tokio::sync::broadcast` or async streams. Bridging requires a per-event subscription handle that lives across the IPC boundary to JS.
2. **`Disposable` / explicit `dispose()`** — Rust prefers RAII via `Drop`. Bridging means wrapping every Rust resource in a JS-visible Disposable and reference-counting cleanup.
3. **`ProviderResult<T> = T | Thenable<T>`** — Sync return is allowed for hot-path providers (hover, inline hints). Rust `async fn` always yields a Future. Either widen the protocol to async-only (small latency hit) or special-case sync paths.
4. **`Thenable<T>`** — A promise-like that VS Code uses instead of native `Promise<T>` for cross-context portability. Already async; maps to `impl Future`.
5. **`CancellationToken`** — Passed to long-running providers. Rust analogue: `tokio_util::sync::CancellationToken` plus `tokio::select!`. Not transparent to extensions; requires explicit cooperation.
6. **`TextEditorEdit` transactions** — `editor.edit(builder => { ... })` batches edits into a single undo unit. Rust-side requires transactional editor APIs that commit atomically.
7. **`FileSystemProvider`** — Extensions register VFS providers per URI scheme. The core dispatches `workspace.fs.*` calls through them. The dispatch layer must live in the extension host or in Rust with FFI back to JS providers.
8. **`WebviewPanel` message bus + `vscode-resource://`** — Webview message passing is straightforward in Tauri (it's literally `postMessage`), but the URI translation (`asWebviewUri`) and CSP/nonce machinery must be re-implemented.
9. **`ThemeColor` + Semantic Tokens** — Themed colors resolve by ID against an active theme; semantic tokens encode TextMate scope chains plus per-theme color indices. Needs a Rust theme service and a TextMate-compatible scope resolver (syntect crate is the obvious target).
10. **`RemoteAuthorityResolver`** — The cleanest Rust ABI in the entire surface. Already byte-streamed. Could be implemented in pure Rust on top of `cli/`'s tunnels in a few weeks.

### 5.2 Extension Host

VS Code today runs a Node.js process forked from main with an RPC bridge of ~200 message types defined in `src/workbench/api/common/extHost.protocol.ts`. Every `mainThreadFoo.ts` in `src/workbench/api/browser/` has a peer `extHostFoo.ts` in `src/workbench/api/common/`.

Three viable architectures:

| Option | Compatibility | Footprint | Build effort | Recommendation |
|---|---|---|---|---|
| **A. Node.js sidecar** | 100% — Copilot, Git, ts-server, Python all work | +50–100 MB | 6–12 mo (bridge + protocol mapping) | **MVP** |
| **B. Deno / V8 isolates with Rust-backed `vscode` bindings** | ~50% initially; full surface takes years | +20 MB | 18–24 mo + ongoing | Lightweight variant in parallel |
| **C. WASM extensions (rewrite extensions in Rust)** | 0% of existing ecosystem | smallest | 24+ mo, ecosystem doesn't exist | Long-term experiment only |

**Recommendation: use Option A for the first release-candidate path.** The Node.js sidecar preserves the entire ecosystem while parity work proceeds. Bridge protocol can be implemented incrementally — start with the message types Copilot needs (chat, lm, workspace.fs, terminal, languages.registerCompletionItemProvider) and grow.

### 5.3 Editor + Workbench (Monaco)

`src/vs/editor/` is 851 files, almost entirely language-agnostic, DOM-based, with a Web Worker for heavy work. Monaco already runs in browsers (vscode.dev proves this). **No Rust port required.** Bundle TS → JS, ship in WebView, done.

`src/vs/workbench/` is 3,395 files. The vast majority is UI: panels, views, commands, keybindings, themes, status bar, quick pick, walkthrough, settings UI. Same disposition — keep in TS, run in WebView.

The pieces of `workbench/` that **do** need Rust replacements are the platform-touching bits: `workbench/services/files/electron-browser`, `workbench/services/textfile/electron-browser`, `workbench/services/lifecycle/electron-browser`, `workbench/services/integrity/node`, `workbench/services/extensions/electron-browser`, etc. Each is a thin shim — typically <500 LOC — that can be reimplemented as a Tauri command + Rust impl.

ESLint rule `code-layering.ts` already encodes this seam: the `electron-browser` and `electron-main` slices are explicitly disallowed from the browser-layer files. The slice that is allowed only `browser` and `common` is the slice that ports for free.

### 5.4 Language Features (LSP)

Five major LSP integrations follow the same factory pattern (`html-language-features/client/src/htmlClient.ts:75-76`):

```typescript
const serverOptions: ServerOptions = {
  run:   { command: 'node', args: ['./server/out/server.js'] },
  debug: { command: 'node', args: ['./server/out/server.js', '--debug'] }
};
const clientOptions: LanguageClientOptions = { documentSelector: [...], synchronize: {...}, middleware: {...} };
const client = new LanguageClient(name, label, serverOptions, clientOptions);
await client.start();
```

Two viable Tauri options:

- **Keep the TS LanguageClient in the WebView/extension host.** Spawn language servers via the existing `vscode-languageclient` infrastructure (which already supports `node-ipc`, `stdio`, `socket`, and `worker`). This is the path of least resistance; it requires only that the extension host (Node.js sidecar) be able to spawn processes, which is trivial.
- **Reimplement an LSP client in Rust** using `lsp-types` + `lsp-server` + `tokio`, with a trait-based middleware system. This is a 4–6 week effort and produces a faster client, but breaks the existing extensions until they migrate.

**Recommendation:** keep TS LanguageClient for MVP. The marginal latency win is small relative to the porting cost.

**TextMate grammars** (the basis for syntax highlighting in non-LSP languages) are pure JSON. Use the `syntect` crate or the `vscode-textmate` JS library compiled into the WebView — either works. Tree-sitter is faster but requires per-language `.scm` query rewrites.

**tsserver host** (`extensions/typescript-language-features/`) is the heaviest LSP integration: custom request types beyond the standard LSP protocol, incremental semantic indexing, refactoring coordination, project-system integration. tsserver itself is Node.js-only; a WASM build exists but is experimental. **Keep tsserver as a Node.js subprocess spawned from the extension host.**

### 5.5 Notebooks, Webviews, Chat, TreeView

All four subsystems share a common substrate: **WebView-based rendering with discriminated postMessage IPC**. Tauri's WebView is *better* suited to these than Electron's renderer because the IPC is explicit and Rust-controlled.

- **Notebook serialization** (`extensions/ipynb/`): pure JSON; serde + Rust enums with `#[serde(tag="cell_type")]` is 300–500 LOC for full ipynb support.
- **Notebook renderers** (`extensions/notebook-renderers/`): MIME-dispatched HTML/SVG/image/JS rendering with a trust-boundary check (`ctx.workspace.isTrusted`). Trusted-Types CSP, ANSI escape parsing, streaming output consolidation. Ports as-is to a Tauri WebView; the rendering is browser-side.
- **Custom editors** (`extensions/media-preview/`): `CustomReadonlyEditorProvider` two-phase lifecycle (`openCustomDocument` → `resolveCustomEditor`). Maps cleanly to Tauri WebView windows with state serialization via `getState` / `setState`.
- **Webview message protocol**: untyped, discriminated by a `type` field. Maps trivially to Tauri events.
- **Chat API** (`extensions/mermaid-chat-features/`): `vscode.chat.registerChatOutputRenderer`, `vscode.lm.registerTool`. Still in proposed-API churn — pin to a specific revision before porting.
- **TreeView**: lazy `getChildren()` callbacks, virtual scrolling, drag-and-drop URI export. Stays in TypeScript; backing data comes from Rust via Tauri commands.

**Notebook controller execution** (kernel discovery, kernel lifecycle, execution streaming) is handled by *separate* extensions (e.g., `ms-toolsai.jupyter`). That layer is out-of-scope for the core port; if the Node.js sidecar is preserved, those extensions continue to work unchanged.

---

## 6. Subsystem-by-Subsystem Porting Plan

### 6.1 SCM (`extensions/git/`, `extensions/github/`, `extensions/merge-conflict/`)

The git extension shells out to `git` CLI via `cp.spawn` (`extensions/git/src/git.ts:676`). It does **not** call `vscode.scm.createSourceControl` directly — it consumes the `vscode.scm` API which lives in `src/vs/workbench/contrib/scm/`. **The core scm subsystem has to be ported; the git extension itself can stay TS.**

Credential flow uses an askpass Unix-domain-socket IPC (`extensions/git/src/askpass.ts:24-44`, `extensions/git/src/ipc/ipcServer.ts:31-80`). In Rust this becomes `tokio::net::UnixListener` (Unix) / named-pipes-via-`async_pipe.rs` (Windows) — already implemented in `cli/`.

merge-conflict extension demonstrates the `CodeLensProvider` + `TextEditorDecorationType` + `Delayer` pattern. All three port directly: CodeLens registration is a Rust trait impl, decoration types are theme-resolved JSON, `Delayer` becomes `tokio::time::sleep` + atomic flag.

### 6.2 Auth Flows (`extensions/{github,microsoft}-authentication/`)

Three flows: LocalServerFlow (loopback redirect), UrlHandlerFlow (PKCE via `vscode://` URI), DeviceCodeFlow (no local server). PKCE SHA-256 verifier (`src/flows.ts:330`). Tokens cached in `context.secrets` with cross-window sync via `onDidChange`.

Microsoft auth uses MSAL with broker support (Windows WAM, macOS security-framework) and a custom token-persistence layer with async operation queuing (`src/betterSecretStorage.ts:16–248`).

**Rust mapping:**
- `oauth2` crate for OAuth2 (PKCE, auth-code, device-code flows)
- `keyring` crate for secret storage (Windows DPAPI / macOS Keychain / Linux secret-service)
- `tokio::net::TcpListener` for loopback callback
- Tauri's URI handler + `tauri::api::shell::open` for deep links
- `cli/src/auth.rs` already implements device-code + token-refresh + keyring integration — extend it

**Recommendation:** prefer device-code where possible. It avoids loopback-server-binding fragility and works under restrictive firewalls.

### 6.3 Tasks (`extensions/{npm,gulp,grunt,jake}/`)

Four task extensions following the same template: register `TaskProvider`, watch a manifest file (`package.json`, `Gruntfile`, etc.), parse it, fan out across workspace folders. Task execution uses `vscode.ShellExecution`, with the *core* spawning the subprocess (not the extension).

The `TaskProvider` API is portable as a Rust trait. File watching becomes the `notify` crate. The two-level (workspace folder × manifest) aggregation is pure algorithm.

### 6.4 Debug

- **debug-auto-launch** (`extensions/debug-auto-launch/`, 425 LOC): four-state machine, Unix-socket IPC server for Node.js process attachment. Maps to `tokio::net::UnixListener` + a tokio-task state-machine actor.
- **debug-server-ready** (`extensions/debug-server-ready/`, 411 LOC): hooks DAP via `vscode.debug.registerDebugAdapterTrackerFactory`, regex-matches terminal output via the proposed `vscode.window.onDidWriteTerminalData`. Ports as a Rust DAP middleware.

The bigger question is the **debug subsystem in `src/vs/workbench/contrib/debug/`** — DAP session lifecycle, breakpoints, watch expressions, call stack, debug console. About 120+ files. The protocol handling is straightforward (DAP is JSON-RPC); the UI stays in the WebView. Effort: 8–12 weeks for a full Rust DAP host with feature parity.

### 6.5 Terminal / PTY

PTY host today is `src/vs/platform/terminal/node/ptyHostMain.ts` — a Node.js subprocess using `node-pty` (a native addon). Replacement:

- **PTY:** `pty-process` crate or `nix::pty` (Unix) + Windows ConPTY via `windows` crate or `winpty`. Both are mature.
- **Terminal UI:** stays in WebView (xterm.js).
- **Shell integration:** Rust spawns the shell with the same env-var injection pattern (`VSCODE_SHELL_INTEGRATION=1`) and ingests OSC sequences for command tracking.

Risk: TTY signal handling (`SIGWINCH`, `SIGTERM`, process groups) differs Unix vs Windows. The `cli/src/util/command.rs` `new_tokio_command` wrapper already handles process-group setup on Unix.

### 6.6 File System + Watcher

Today: file service with VFS provider registration in `src/vs/platform/files/common/files.ts`; native disk impl using Parcel watcher (a native Node addon).

Replacement:
- File ops: `tokio::fs` + Rust `IFileService` trait
- Watcher: `notify` crate (cross-platform, pure Rust)
- VFS provider dispatch: registry trait by URI scheme, with FFI back to extension host for extension-provided providers

Test against the existing `diskFileService.integrationTest.ts` suite — it covers symlink, rename, atomic-write, and retry edge cases that are easy to get wrong.

### 6.7 Remote / Tunnel

The cleanest Rust ABI in the project. `RemoteAuthorityResolver` (`extensions/vscode-test-resolver/`) is already byte-streamed; `tunnel-forwarding` is already half-Rust (TS extension drives the `code-tunnel` Rust binary over line-delimited JSON on stdin/stderr).

`cli/src/tunnels/` already implements:
- Microsoft dev-tunnels integration (`dev_tunnels.rs`, ~1.2K LOC)
- A multiplexed control server (`control_server.rs`, ~1.5K LOC)
- Bidirectional TCP/Unix port forwarding (`port_forwarder.rs`, ~130 LOC)
- The full RPC protocol (`protocol.rs`, ~340 LOC)

**Inline this into the core.** The extension becomes a thin TS shim that calls Tauri commands. The `code-tunnel` external binary is no longer needed; the same Rust code runs in-process.

### 6.8 Build / Packaging

Current pipeline (`build/`, 195 files / 33.6K LOC, plus `gulpfile.mjs`): gulp orchestration (12 feature-specific gulpfiles), esbuild transpilation (mid-migration from gulp-tsb), Electron packaging via electron-builder, ASAR archive creation with selective unpacking for native modules, per-platform bundlers (Inno Setup on Windows, dpkg/rpm on Linux, dmgbuild + osx-sign + notarization on macOS), policy generation (Windows ADMX, macOS plist, Linux JSON), Azure Pipelines + ESRP for code signing.

**Replacement:**
- Master orchestration: `cargo xtask` or `cargo build` + `build.rs`
- App packaging: Tauri bundler (MSI/NSIS for Windows, .dmg/.app for macOS, AppImage/deb/rpm for Linux)
- Auto-update: Tauri's built-in `tauri-plugin-updater` replaces Squirrel + the custom `update_service.rs`
- ASAR: gone; Tauri uses filesystem layout or embedded resources
- Code signing: same underlying tools (signtool, codesign, dpkg-sig); orchestration moves into Rust build scripts
- Extension bundling (esbuild): unchanged, invoked from the build script
- NLS extraction: unchanged, runs during esbuild
- Policy generation: ports to a Rust tool or stays as a build-time Node.js subprocess

**Effort:** 2–4 weeks to swap the orchestration; CI pipeline rewrite is the long pole (Azure Pipelines configs, ESRP integration). The Snap, MSIX, deb, rpm, .app outputs already work for `cli/` so the recipes exist.

---

## 7. Quick Wins (each <4 weeks)

These deliver visible progress with minimal risk and unblock larger work:

1. **Reuse `cli/` wholesale.** Pull it into `src-tauri/` as a workspace member or as a library crate. Re-export every existing command as a Tauri `#[tauri::command]`. **Day 1**, you have auth, tunnels, port-forwarding, remote server lifecycle.
2. **Workspace File System (`IFileService` trait).** `tokio::fs` impl, URI-scheme dispatch, three Tauri commands (`fs_stat`, `fs_read_file`, `fs_write_file`). 1–2 weeks. Unblocks every UI feature that touches disk.
3. **Diagnostic Collections.** In-memory `HashMap<Uri, Vec<Diagnostic>>` + an event bus. 1 week. Linters work immediately.
4. **Command dispatch.** `HashMap<String, Box<dyn Fn(...) -> ...>>`. 3–5 days. Unblocks the entire keybinding + command-palette surface.
5. **Diagnostics, completions, hover providers as Rust traits.** Stub impls that proxy to the Node.js extension host until that comes online. 1 week.
6. **Configuration management.** JSON load + nested-key lookup + per-language overrides. 2 weeks.
7. **Terminal MVP.** `tokio::process::Command` + `pty-process` crate; shell integration as a phase-2 add-on. 1 week for spawn+stdio, 1 week for PTY.
8. **TextMate grammar loader.** `syntect` crate; load existing `.tmLanguage.json` files unchanged. 50 LOC + tests. 3–5 days.
9. **All grammar-only extensions.** Copy 11 directories of pure JSON (typescript-basics, php, html, razor, json, cpp, less, yaml, latex, ...) — they already work. 0 days of porting; build-system wiring only.
10. **All theme JSON, snippet JSON, language-configuration JSON.** Pure data; load via serde. 3 days.
11. **`ipynb` parser.** serde + tagged enums for cell types; bidirectional serialize/deserialize. 300–500 LOC. 1 week.
12. **Remote Authority Resolver.** Already byte-streamed; wraps a Rust struct around `TcpStream` / `WebSocket`. 2–3 weeks; enables SSH / Codespaces / containers immediately.
13. **OAuth flows.** `oauth2` crate + `keyring` crate; `cli/src/auth.rs` already 60% there. 2 weeks for full feature parity with `microsoft-authentication`.

By the end of the quick-win phase (~12 weeks for a 3-engineer team), you have a functional Tauri shell with Monaco + file I/O + terminal + Git CLI + auth + remote tunnels + a Node.js sidecar booting up. Not a release, but a credible internal demo.

---

## 8. Hard Blockers and Mitigations

### 8.1 Extension Host Isolation (critical)

**Problem:** The extension ecosystem assumes Node.js. Some extensions ship `.node` native addons (better-sqlite3, fsevents, the legacy node-pty). Many depend on npm packages that assume a Node runtime.

**Mitigation (recommended):** Keep Node.js as a sidecar process, spawned and managed by the Rust core, communicating over an IPC socket (Unix domain socket / named pipe). The ~200-message extHost protocol becomes serde-serializable Rust structs on one side and existing TS RPC on the other.

**Cost:** +50–100 MB of binary footprint, +200–500 ms of startup time. Acceptable for MVP.

### 8.2 Native Modules in Built-in Extensions

**Problem:** The git, github, and Copilot extensions all import `electron` or use Electron-specific APIs (`app.getPath()`, `clipboard.readText()`).

**Mitigation:** Provide a shim `electron` module in the sidecar that proxies these calls to Tauri commands. A few hundred lines of glue.

### 8.3 ESM-hook File-System Remapping

**Problem:** `bootstrap-esm.ts:14–30` installs a Node.js Module hook that remaps `fs` → `node:original-fs` because Electron patches `fs` for ASAR access.

**Mitigation:** Delete entirely. Tauri has no ASAR; native fs works directly.

### 8.4 Custom URL Schemes (`vscode-webview://`, `vscode-file://`)

**Problem:** `src/main.ts:96–105` registers privileged protocol schemes via Electron's `protocol.registerSchemesAsPrivileged`. Used for webview asset loading and bypassing CORS.

**Mitigation:** Tauri supports custom protocols (`tauri::Builder::register_uri_scheme_protocol`). Reimplement `asWebviewUri` to produce Tauri-compatible URIs. ~200 LOC.

### 8.5 Crash Reporting

**Problem:** `src/main.ts:527–539` uses Electron's `crashReporter` with AppCenter. No equivalent in Tauri.

**Mitigation:** `sentry` crate or a custom collector that ships minidumps to telemetry. Different API; requires re-design of the telemetry pipeline.

### 8.6 Multi-Window Architecture

**Problem:** ESLint rules (`eslint.config.js:1078–1426`) enforce that DOM access goes through `DOM.getWindow(<element>)` to support multi-window correctly. Extensive call-site discipline.

**Mitigation:** Tauri supports multiple windows natively (`tauri::WindowBuilder`). Map each VS Code window to a Tauri window; preserve the `getWindow()` discipline on the JS side. The risk is off-by-one window-handle bugs; mitigated by integration tests that exercise the multi-window flows.

### 8.7 Cancellation Semantics

**Problem:** `CancellationToken` is checked cooperatively. Rust cannot transparently propagate cancellation across the IPC boundary into JS providers.

**Mitigation:** Pass cancellation tokens as opaque IDs across the IPC; Rust side fires a "cancel" event, JS side observes it and aborts. Same pattern as today's extHost protocol.

### 8.8 ProviderResult Sync-or-Promise

**Problem:** Some hot-path providers return values synchronously to keep latency under 100 ms (hover, inline hints). Rust async always yields a Future.

**Mitigation:** Either widen the protocol to async-only and accept the latency, or maintain a synchronous-result fast path on the Rust side that pre-computes common cases (symbols, definitions).

### 8.9 vscode-webview-ui-toolkit Dependency

**Problem:** Many built-in extensions use `@vscode/webview-ui-toolkit` for theme-consistent web components. Depends on Adaptive UI design tokens and assumes a VS Code theme service.

**Mitigation:** The toolkit is optional. Themed CSS variables can be injected from Rust. No blocker; just discipline.

### 8.10 Proposed-API Churn

**Problem:** Chat, LM, SCM history, terminal-completion, kernel-management — all proposed-API surfaces are in active flux. Pinning to a stable version is hard.

**Mitigation:** Pin proposed-API versions in `package.json` `enabledApiProposals`. Track upstream churn and bump deliberately. Treat the proposed-API surface as a moving target, not a stable contract.

---

## 9. Phased Roadmap

| Phase | Deliverable | Effort (3–5 eng) | Calendar |
|---|---|---|---|
| **P0 — Foundation** | Tauri shell hosting Monaco; `cli/` integrated; Tauri command bridge for IPC; `bootstrap-*` deleted; Node.js sidecar spawning | 4–8 wk | Weeks 1–8 |
| **P1 — Core services** | `IFileService`, `IConfigurationService`, `ILoggerService`, `IStorageService`, terminal MVP; file watcher (`notify`); URI scheme handlers | 8–12 wk | Weeks 6–18 (overlaps P0) |
| **P2 — Frontend adaptation** | Workbench TS adapted; replace Electron IPC call sites with `invoke()`; multi-window plumbing; theme system | 8–12 wk | Weeks 10–22 |
| **P3 — Extension host bridge** | Full extHost protocol mapped (~200 messages); Copilot smoke test passes; conformance suite begins to pass | 12–24 wk | Weeks 14–38 |
| **P4 — Language features + debug** | LSP integrations work via sidecar; DAP relay; Git extension functional; built-in extensions ported | 8–16 wk | Weeks 20–36 |
| **P5 — Build / packaging** | Tauri bundler producing signed Snap, MSIX, deb, rpm, dmg; auto-update; CI rewrite | 4–8 wk | Weeks 24–32 |
| **P6 — Polish + perf + tests** | All vscode-api-tests pass; perf parity with Electron; binary size optimization; remote/tunnels validation | 8–12 wk | Weeks 32–44 |

**Total:** 12–18 calendar months for feature-parity with Electron (assuming Node.js sidecar). MVP demoable at week 16; alpha candidate at week 32; `ReleaseCandidate` only after all migration governance gates are green.

### 9.1 Migration Governance

`Scaffold` is an internal developer milestone only. It permits local smoke and developer-only fallback work, but it is not a release artifact, not migration completion, and not evidence that the Tauri workbench is shippable. `src-tauri/www/index.html` is debug-only developer fallback and must never be treated as a release workbench source. Packaged release must load bundled `src-tauri/www/out/vs/code/browser/workbench/workbench.html` through the Tauri app asset path.

Release governance reaches `ReleaseCandidate` after all required gates are green: real workbench boot, Rust runtime build, core service parity, extension API parity, built-in extension parity, signed package validation, and release validation. No completion claim is allowed before `ReleaseCandidate`. Roadmap phases before `ReleaseCandidate` should be described as internal scaffold, developer smoke, alpha candidate, or migration in progress, not complete migration or release-ready product.

Current validation contract: `npm run test:tauri:api` is the clean VS Code API sidecar preflight. `CODE_TAURI_APP_PATH=<path-to-tauri-app> npm run test-extension:tauri-sidecar:execute` requires compiled `extensions/vscode-api-tests/out` suites and an existing `CODE_TAURI_APP_PATH`. Parity gate promotion requires CI evidence, not local-only pass output.

The schedule is aggressive but defensible because of the `cli/` head start. Without it, add 12–16 weeks to P0.

---

## 10. Open Questions and Risks

1. **Will the JS-on-Rust Foreign Function bridge handle 200+ message types at the latency required for hover/completion?** Benchmark early. Tauri's invoke layer is faster than Electron IPC but introduces a JSON-encode round trip.
2. **How does Monaco's GPU rendering (`src/vs/editor/browser/gpu/`) behave inside Tauri's WebView (WebKit on macOS, WebView2 on Windows, WebKitGTK on Linux)?** Should be fine — WebGPU is broadly supported — but verify on day 1.
3. **What is the Snap confinement story under Tauri?** `cli/` already ships in Snap; the Tauri app bundle is a different packaging surface. May require AppArmor rule updates.
4. **Does the multi-window pattern survive Tauri's per-window WebView model cleanly?** ESLint enforces correct `getWindow()` use today; the audit work is largely done. The runtime work is to map each VS Code window onto a Tauri window with shared services.
5. **What's the strategy for the proposed-API surface?** Pin per release? Maintain a compatibility layer? Track upstream and bump?
6. **What changes in extension manifests?** Today extensions declare `engines.vscode`. They will need either continued compatibility or a new identifier (e.g., `engines.vscode-tauri`).
7. **What is the Copilot team's commitment?** The port lives or dies on Copilot working. Early engagement is non-negotiable.

---

## Appendix A — Source Citations

This document was synthesized from 80 partition-level shard reports under `/home/norinlavaee/projects/vscode-atomic/research/docs/.deep-research-1778468401377/`. Each partition has explorer (synthesis), locator (file index), analyzer (per-file deep dive), pattern-finder (code patterns), and online (external references) shards. Notable file:line references that anchor specific claims:

- **Service / DI**: `src/vs/platform/instantiation/common/instantiation.ts:109–120` (`createDecorator<T>`)
- **IPC contract**: `src/base/parts/ipc/common/ipc.ts:25–38` (`IChannel`, `IServerChannel`)
- **Server bootstrap**: `src/server-main.ts:88–94` (lazy `getRemoteExtensionHostAgentServer`)
- **File system abstraction**: `src/vs/platform/files/common/files.ts:554–566` (`IFileService` interface)
- **ESM remap hook**: `src/bootstrap-esm.ts:14–30` (`fs` → `node:original-fs`)
- **Custom protocol registration**: `src/main.ts:96–105`
- **Crash reporter**: `src/main.ts:527–539`
- **Multi-window enforcement**: `eslint.config.js:1078–1426`
- **Git spawn**: `extensions/git/src/git.ts:676–703`
- **Git askpass IPC**: `extensions/git/src/askpass.ts:24–44`, `extensions/git/src/ipc/ipcServer.ts:31–80`
- **GitHub OAuth**: `extensions/github-authentication/src/github.ts:179`, `src/flows.ts:330` (PKCE SHA-256)
- **Microsoft auth secret storage**: `extensions/microsoft-authentication/src/betterSecretStorage.ts:16–248`
- **NotebookSerializer**: `extensions/ipynb/src/ipynbMain.ts:35–67`
- **Notebook renderer dispatch**: `extensions/notebook-renderers/src/index.ts:419–639`
- **Webview panel**: `extensions/simple-browser/src/simpleBrowserView.ts:40–53, 73–98`
- **Custom editor**: `extensions/media-preview/src/imagePreview/index.ts:15–70, 180–220`
- **Chat output renderer**: `extensions/mermaid-chat-features/src/chatOutputRenderer.ts:22–182`
- **TreeView (references)**: `extensions/references-view/src/tree.ts:49–222`, `references/model.ts:55–325`
- **Tunnel forwarding**: `extensions/tunnel-forwarding/src/extension.ts` (line-delimited JSON over `code-tunnel` stdin/stderr)
- **Auto-launch state machine**: `extensions/debug-auto-launch/src/extension.ts:341–356`
- **Existing Rust foundation**: `cli/src/{rpc,async_pipe,auth,state,util/command,util/http}.rs`, `cli/src/tunnels/{control_server,dev_tunnels,port_forwarder,protocol,code_server}.rs`, `cli/src/desktop/version_manager.rs`, `cli/src/commands/{agent_host,serve_web}.rs`

## Appendix B — What We Did Not Cover in Depth

- **Telemetry pipeline.** `src/vs/platform/telemetry/` is large and ships sensitive code paths; treat as a separate scoping pass.
- **Extension marketplace integration.** Gallery service, signature verification, updates. Not in the scoped 80 partitions.
- **Notification of breaking changes to third-party extension authors.** A deprecation/migration guide will be needed; not a code question.
- **Speech / AI / Power namespaces.** Proposed APIs only; small surface; deferred.
- **Microsoft-internal infrastructure** (CDXP, ESRP, AppCenter) is referenced but not exhaustively documented.
- **The `extensions/copilot/` partition** (695K LOC) was scanned for *what API it consumes*, not for its internals. Its internals are out of scope for the core port.

## Appendix C — Recommended Next Steps

1. **Approve the strategy.** Specifically: extend `cli/` upward; Node.js sidecar for extension host; preserve `vscode.d.ts` byte-for-byte.
2. **Stand up the P0 prototype.** Tauri shell + Monaco bundle + `cli/` as a library + a single `#[tauri::command] fn fs_read_file()`. One engineer, two weeks.
3. **Run the conformance suite under Node.js sidecar.** Whatever passes today is the baseline; whatever fails is the work list for P3 / P4.
4. **Commit to a proposed-API pin.** Freeze a specific revision of every proposed API in `enabledApiProposals` for the duration of the port.
5. **Engage Copilot owners.** The port has no purpose if Copilot doesn't work on it.
