# Partition 37: `extensions/vscode-test-resolver/` — External Library Research

**Decision: (no external research applicable)**

---

## Justification

### Dependency inventory

| File | Imports used | Nature |
|---|---|---|
| `src/extension.ts` | `child_process`, `net`, `http`, `https`, `crypto`, `fs`, `os`, `path` (all Node built-ins); `vscode` proposed API (`registerRemoteAuthorityResolver`, `ManagedResolvedAuthority`, `TunnelOptions`, `ResolvedAuthority`) | All Node stdlib + VS Code host API |
| `src/extension.browser.ts` | `vscode` (same proposed API); browser-global `WebSocket`, `URL`, `TextDecoder`, `TextEncoder` | VS Code host API + Web Platform globals only |
| `src/download.ts` | `https`, `fs`, `path`, `child_process`, `url` (all Node built-ins); delegates archive extraction to OS tools (`powershell.exe Expand-Archive`, `unzip`, `tar`) via `cp.spawnSync` | Node stdlib + shell commands |
| `src/util/processes.ts` | `child_process`, `path` (Node built-ins); `taskkill` (Windows) / `terminateProcess.sh` (Unix) | Node stdlib + OS utilities |

### Why no external library docs are central

Every dependency in this partition falls into one of three well-understood categories:

1. **Node.js built-in modules** (`child_process`, `net`, `http`, `https`, `crypto`, `fs`, `os`, `path`, `url`). These are standard Node APIs with stable, authoritative documentation at nodejs.org. They contain no third-party library abstractions that require separate research. In a Tauri/Rust port these are replaced wholesale by Rust equivalents (`std::process`, `tokio::net`, `reqwest`, `std::fs`, etc.); the mapping is mechanical and the target Rust docs are well-known.

2. **VS Code proposed/stable API** (`vscode.workspace.registerRemoteAuthorityResolver`, `vscode.ManagedResolvedAuthority`, `vscode.ResolvedAuthority`, `vscode.TunnelOptions`, etc.). These are VS Code host interfaces, not external libraries. Their behaviour is defined by VS Code itself. Porting to Tauri means these interfaces are eliminated entirely and replaced by Tauri IPC/plugin equivalents; the VS Code API docs are irrelevant to the Rust rewrite.

3. **Web Platform globals** (`WebSocket`, `URL`, `TextDecoder`, `TextEncoder`). These are standardised browser APIs available without any npm dependency. In a Tauri/Rust context the WebSocket client side can be handled by Tauri's built-in `tauri-plugin-websocket` or `tokio-tungstenite`; the Web Platform API itself needs no external research for porting purposes.

There are **zero third-party npm packages** required by this partition (no `package.json` runtime dependencies beyond the VS Code extension host itself). Archive extraction is delegated to OS-level tools (`tar`, `unzip`, PowerShell) invoked via `child_process.spawnSync`, so there is no archiving library to research.

### What actually matters for porting

The porting challenges in this partition are architectural, not library-documentation problems:

- **Remote authority resolution** (`registerRemoteAuthorityResolver`, `ManagedResolvedAuthority`, `ResolvedAuthority`) is a VS Code-specific protocol. Tauri has no equivalent; this entire subsystem must be redesigned around Tauri's IPC model. The VS Code extension API docs do not help here.
- **TCP proxy/tunnel creation** using `net.createServer` / `net.createConnection` maps to `tokio::net::TcpListener` / `TcpStream`. This is standard Tokio knowledge, not an exotic library.
- **Child process lifecycle** (`cp.spawn`, `taskkill`, `SIGKILL`) maps to `std::process::Command` / `tokio::process::Command`. Again, standard Rust stdlib/Tokio.
- **HTTPS download with redirect follow** (`https.get` with 302 redirect) maps to `reqwest`. The reqwest docs are a single, brief lookup and not central enough to justify a dedicated research artifact.

Because all dependencies are either Node built-ins, VS Code host APIs, or Web Platform globals — and because the porting decision for each is dictated by Tauri/Rust architecture rather than by third-party library nuances — no external library documentation fetch is warranted for this partition.

---

*Summary: `extensions/vscode-test-resolver/` depends exclusively on Node.js built-in modules, VS Code proposed extension-host APIs, and browser platform globals. There are no third-party npm runtime packages whose documentation is central to a Tauri/Rust port. The porting work is an architectural replacement (VS Code remote authority protocol → Tauri IPC; Node `net`/`child_process` → Tokio/`std::process`), not a library migration requiring external doc research.*
