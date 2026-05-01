# Porting `extensions/debug-auto-launch/` from TypeScript/Electron to Tauri/Rust

## External Research Applicability

(no external research applicable) — The extension depends exclusively on Node.js built-ins (`net`, `fs`, `path`) and the VS Code extension API (`vscode`); it carries no third-party npm runtime dependencies, so there are no external library documentation sources whose content would be central to a porting analysis.

---

## Summary

The `debug-auto-launch` extension (one source file, ~407 LOC) manages the Node.js debugger auto-attach feature inside VS Code. It does three things: (1) reads and writes a workspace-scoped configuration setting (`debug.javascript.autoAttachFilter`), (2) creates a Unix-domain socket / Windows named-pipe IPC server using Node's `net` module that listens for NUL-byte-framed JSON messages from Node.js processes launched in the integrated terminal, and (3) relays those messages to the `js-debug` extension via cross-extension command dispatch (`vscode.commands.executeCommand`). It also manages a status bar item and a QuickPick toggle UI. The only compile-time dependency beyond TypeScript itself is `@types/node`.

---

## Detailed Findings

### 1. IPC Transport Layer (`net` module — Unix socket / named pipe)

**What the code does.** `createServerInstance` opens a `net.Server` on a path supplied by the `js-debug` extension (via `extension.js-debug.setAutoAttachVariables`). Each connecting socket accumulates `Buffer` chunks until it sees a trailing NUL byte, at which point the concatenated payload is JSON-parsed and forwarded to `extension.js-debug.autoAttachToProcess`. A single-byte `0x00` or `0x01` acknowledgement is written back.

**Porting to Rust / Tauri.** Rust's standard library and the Tokio async runtime both provide first-class Unix domain socket and Windows named-pipe support:

- `tokio::net::UnixListener` / `tokio::net::UnixStream` (Unix)
- `tokio::net::windows::named_pipe` (Windows)

The NUL-byte framing is trivial to replicate with a `tokio::io::AsyncReadExt` read loop that accumulates bytes until `0x00`. The JSON payload can be deserialized with `serde_json`. There is no semantic complexity here; the logic maps almost one-to-one to async Rust. The primary porting cost is plumbing the socket path from the js-debug counterpart — which is itself a TypeScript extension and would need its own porting story before this layer can work end-to-end.

**Key concern: socket path ownership.** The current code calls `fs.unlink` on the socket file to recover from a leaked socket left by a crashed process (lines 241–245). Rust/Tokio does not automatically clean up socket files on drop either, so the same defensive-unlink pattern must be implemented (e.g., a `std::fs::remove_file` call wrapped in a `catch_unwind`-safe guard).

---

### 2. VS Code Extension API Surface Used

The extension calls the following VS Code API namespaces:

| API | Purpose | Tauri equivalent |
|-----|---------|-----------------|
| `vscode.workspace.getConfiguration` | Read/write `debug.javascript.*` settings | Tauri `tauri-plugin-store` or app config JSON |
| `vscode.workspace.onDidChangeConfiguration` | React to settings changes | Custom config-change event bus |
| `vscode.commands.registerCommand` | Expose toggle command | Tauri command (`#[tauri::command]`) or IPC handler |
| `vscode.commands.executeCommand` | Call into js-debug extension | Inter-plugin message passing (no direct Tauri equivalent) |
| `vscode.window.createStatusBarItem` | Status bar badge | Tauri system tray or custom webview status bar |
| `vscode.window.createQuickPick` | Dropdown toggle UI | Custom webview component |
| `vscode.extensions.getExtension` | Detect js-debug installation path | Plugin registry query |
| `context.workspaceState.get/update` | Persist IPC address across sessions | SQLite via `tauri-plugin-sql` or flat JSON state file |
| `vscode.l10n.t` | Localization | `fluent` or `i18n-embed` crate |

The deepest porting challenge is `vscode.commands.executeCommand` used to cross-extension dispatch. VS Code's extension host provides a shared command registry that any extension can publish to or consume from. Tauri has no built-in equivalent; a custom inter-plugin event bus or a named IPC channel between Tauri plugins would need to be designed. Without this mechanism the auto-attach workflow cannot function regardless of how cleanly the IPC server is ported.

---

### 3. State Machine and Lifecycle

The extension implements a serialized promise queue (`currentState`) to prevent concurrent state transitions. Each state (`Disabled`, `OnlyWithFlag`, `Smart`, `Always`) has an enter-action that either destroys or creates the IPC server. In Rust this translates naturally to a `tokio::sync::Mutex<State>` protecting a task that sequentially awaits server start/stop futures. The `isTemporarilyDisabled` flag, which suppresses the server without changing the persisted setting, maps to a boolean field inside the mutex guard.

---

### 4. Platform-Specific Considerations

- **Windows named pipes.** The existing code branches on `process.platform !== 'win32'` for the socket-directory-accessibility check (lines 220–229). Rust must similarly branch: `tokio::net::windows::named_pipe::ServerOptions` has different error semantics from `UnixListener`, and the named-pipe path format (`\\.\pipe\<name>`) differs from Unix socket paths.
- **Temporary directory stability.** The extension defends against macOS/Linux tmpdir path changes by catching `ENOENT` on the socket's parent directory and triggering a full variable refresh. A Rust port should check `std::io::ErrorKind::NotFound` on `bind` and implement the same recovery.

---

### 5. Overall Porting Effort Estimate

| Concern | Estimated effort |
|---------|-----------------|
| IPC server (NUL-framed Unix socket / named pipe) | Low — direct Tokio translation |
| Settings read/write and change detection | Medium — requires reimplementing VS Code's layered config model |
| Status bar item and QuickPick UI | Medium–High — no Tauri primitive; custom webview required |
| Cross-extension command dispatch | High — no Tauri equivalent; requires new plugin IPC architecture |
| Workspace state persistence | Low — flat JSON or SQLite |
| Localization | Low — standard i18n crate |

The IPC server itself (roughly 30 lines of the 407 total) is the most straightforward piece to port. The bulk of the porting cost falls on the VS Code API surface — particularly the cross-extension command dispatch that ties this extension to `js-debug`, and the layered configuration and UI primitives that VS Code provides out of the box but Tauri does not.

---

## Gaps or Limitations

The js-debug extension (`ms-vscode.js-debug`) that this extension depends on is a large, separately maintained codebase. Any functional port of `debug-auto-launch` is gated on a compatible port of js-debug's `setAutoAttachVariables` and `autoAttachToProcess` command endpoints. That dependency is outside the scope of this extension's two files.

---

## Concluding Assessment

Porting `extensions/debug-auto-launch/` to Tauri/Rust is conceptually straightforward at the transport level — the NUL-byte-framed Unix socket server maps cleanly onto Tokio's async networking primitives, and the state machine is a thin wrapper that translates naturally to a mutex-guarded async task — but the extension's functional value is almost entirely delivered through VS Code API calls (cross-extension command dispatch, layered workspace configuration, QuickPick UI, status bar) for which Tauri provides no direct equivalent, meaning that the IPC server is the easy 10 percent of the work while rebuilding the surrounding VS Code extension host contract — in particular the shared command registry that lets this extension invoke js-debug and vice versa — represents the hard 90 percent, and that architectural gap must be solved at the IDE level before any individual extension port becomes meaningful.
