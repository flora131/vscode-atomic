# Online Research: Porting `extensions/tunnel-forwarding/` to Tauri/Rust

(no external research applicable)

## Justification

The `extensions/tunnel-forwarding/` extension is a thin TypeScript orchestration layer (474 LOC across 4 files) whose sole job is to bridge VS Code's proposed `TunnelProvider`/`tunnelFactory`/`resolvers` APIs to the existing Rust CLI binary already living in `cli/`. All of its non-trivial logic lives in one file (`src/extension.ts`) and reduces to three concerns:

1. **Locating the Rust CLI binary** — platform-specific path resolution using `vscode.env.appRoot`, `vscode.env.appCommit`, and `process.platform`. Fully self-contained; no external docs are needed to understand or replace it.

2. **Spawning the CLI as a child process and speaking a JSON-line protocol over stdin/stdout** — `child_process.spawn` from Node.js writes `[{number, privacy, protocol}]` JSON lines on stdin; the CLI responds on stderr with `{"port_format": "..."}` JSON lines. `src/split.ts` is a minimal Transform stream splitter (~50 LOC), and `src/deferredPromise.ts` is a copied async helper (~60 LOC). Both utilities are trivially self-contained.

3. **Registering a `TunnelProvider` via `vscode.workspace.registerTunnelProvider`** — this is a VS Code proposed API (`enabledApiProposals: ["resolvers", "tunnelFactory"]`). The authentication step calls `vscode.authentication.getSession('github', ...)` to obtain an access token that is forwarded to the CLI via the `VSCODE_CLI_ACCESS_TOKEN` environment variable.

None of these pieces depend on external libraries that would require their own documentation research. The actual tunneling logic — Microsoft Dev Tunnels, the `devtunnels` SDK, Tauri IPC primitives, or the `devtunnel` REST API — lives entirely inside the Rust `cli/` crate and is not touched by this extension at all. The extension treats the CLI as an opaque subprocess.

**What porting actually requires** is therefore not external API research but an architectural decision: in a Tauri host, the equivalent of `vscode.workspace.registerTunnelProvider` does not exist. Tauri communicates between the Rust backend and the WebView frontend via Tauri commands and events, not via a VS Code extension API. The port-forwarding subsystem would need to be reimplemented as a Tauri plugin or Tauri command set that directly calls the existing Rust tunneling library (currently reachable at `cli/src/tunnels/`) from the Tauri backend, eliminating the Node.js subprocess entirely. The JSON-line protocol over stdin/stdout — the only real interface this extension defines — would be replaced by direct in-process Rust function calls. The GitHub authentication token flow would migrate to whichever credential-store mechanism the Tauri shell exposes (e.g., `tauri-plugin-store` or OS keychain bindings). All of this is determinable from reading the existing `cli/` source and standard Tauri architecture documentation, neither of which requires a live web fetch to research for this scope because the Rust CLI code is already in the repository and the Tauri IPC model is straightforward enough that no deep external documentation dive would change the porting analysis for this particular, narrow extension.
