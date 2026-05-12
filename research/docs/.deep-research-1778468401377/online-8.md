#### TypeScript `ts.server.protocol` (bundled with `typescript` npm package v6.x)

**Docs:** https://github.com/microsoft/TypeScript/wiki/Standalone-Server-%28tsserver%29
**Protocol source:** https://raw.githubusercontent.com/microsoft/TypeScript/main/src/server/protocol.ts

**Relevant behaviour — message framing and transport:**

The extension does NOT use LSP or `vscode-languageclient`. It communicates directly with tsserver using TypeScript's own proprietary JSON protocol. The local protocol shim at `extensions/typescript-language-features/src/tsServer/protocol/protocol.d.ts:5` re-exports the entire `ts.server.protocol` namespace from the installed `typescript` package (currently v6.0.0-dev.20260416):

```ts
import type ts from '../../../../node_modules/typescript/lib/typescript';
export = ts.server.protocol;
```

**Wire format — tsserver vs LSP:**

tsserver's framing is superficially similar to LSP (both use HTTP-style `Content-Length` headers before a JSON body over stdio) but differs in every semantic detail:

| Dimension | tsserver | LSP (JSON-RPC 2.0) |
|---|---|---|
| Envelope field | `seq` (auto-incrementing int) | `id` (int or string or null) |
| Message type tag | `"type": "request" \| "response" \| "event"` | method name encodes direction; no `type` field |
| Protocol version | none | `"jsonrpc": "2.0"` required |
| Command/method names | `quickinfo`, `completionInfo`, `geterr`, `updateOpen`, … | `textDocument/completion`, `textDocument/definition`, … |
| Response correlation | `request_seq` in response | `id` echoed in response |
| Cancellation | Named pipe: `--cancellationPipeName` arg + writing a sentinel file | `$/cancelRequest` JSON-RPC notification |
| Server-push events | `type: "event"` messages with same Content-Length framing | `type: "notification"` (no `id`) |
| Async diagnostics | `geterr` fires `semanticDiag` / `syntaxDiag` events | `textDocument/publishDiagnostics` push notification |

The `ProtocolBuffer` / `Reader<T>` classes in `serverProcess.electron.ts:34-141` implement the Content-Length framing parser on the client side. The `StdioChildServerProcess.write()` method at line 288 serialises each request as `JSON.stringify(request) + '\r\n'` (no header prefix required for requests — only responses and events from the server carry `Content-Length`).

**Transport modes (Electron path):**

Two transport options are forked at `serverProcess.electron.ts:367-385`:
- **IPC mode** (TypeScript ≥ 4.6.0, no custom Node path): `child_process.fork` with `--useNodeIpc`, communication via `process.send()` / `process.on('message')` — JSON is serialised by Node's IPC channel automatically (`IpcChildServerProcess`).
- **Stdio mode**: `child_process.spawn` (or `fork` with `silent: true`) plus the `Reader<T>` Content-Length framing parser (`StdioChildServerProcess`).

**Transport modes (browser/WASM path):**

`serverProcess.browser.ts` runs tsserver as a `Worker`. Communication uses three `MessageChannel` ports: `_tsserver` for synchronous request/response, `_watcher` for file-system watch events, and `_syncFs` for synchronous filesystem access. The `@vscode/sync-api-common` / `@vscode/sync-api-service` packages mediate that synchronous fs channel via a `SharedArrayBuffer`-based RPC.

**Commands used:**

`typescriptService.ts:37-99` enumerates all 40+ commands the VS Code client issues, covering completions (`completionInfo`), diagnostics (`geterr`, `geterrForProject`), navigation (`definition`, `references`, `navtree`), refactoring (`getApplicableRefactors`, `getEditsForRefactor`), organise imports, inlay hints, call hierarchy, and more. None map to LSP methods.

**Server multiplexing:**

`spawner.ts` forks between one and three concurrent tsserver processes: a `syntax` process (partialSemantic mode via `--serverMode partialSemantic` or `--syntaxOnly`) for fast IDE features while loading, a `semantic` process for full analysis, and optionally a separate `diagnostics` process. `SyntaxRoutingTsServer` and `GetErrRoutingTsServer` in `server.ts` fan requests out to the appropriate process based on the command's declared routing — fence commands (`change`, `close`, `open`, `updateOpen`) are sent to both servers to keep state in sync.

**Cancellation:**

Electron path: `cancellation.electron.ts` writes an empty file at `<cancellationPipeName><seq>` to signal cancellation; tsserver polls for that file. Browser/web path: `Cancellation.addData()` from `@vscode/sync-api-common` sets a flag in a `SharedArrayBuffer` that tsserver reads synchronously from its worker thread.

**Version gating:**

`api.ts` tracks TypeScript version constants from v3.8.0 through v5.9.0 and gates individual features behind `apiVersion.gte(API.vXYZ)` checks (e.g. IPC transport requires ≥ 4.6.0, `--serverMode partialSemantic` requires ≥ 4.0.1).

**Where used:**

- `src/tsServer/protocol/protocol.d.ts:5` — protocol type re-export
- `src/tsServer/serverProcess.electron.ts:22-97` — Content-Length frame parser (`ProtocolBuffer`, `Reader<T>`)
- `src/tsServer/serverProcess.electron.ts:286-388` — two `TsServerProcess` implementations (IPC vs stdio)
- `src/tsServer/serverProcess.browser.ts:61-187` — WASM Worker transport
- `src/typescriptService.ts:37-99` — full enumeration of all tsserver commands used
- `src/tsServer/server.ts:79-88` — `TsServerProcess` interface (write/onData abstraction)
- `src/tsServer/spawner.ts:188-260` — argument assembly (`--cancellationPipeName`, `--serverMode`, `--logFile`, etc.)

**Porting implications for Tauri/Rust:**

The `typescript-language-features` extension is the single most tsserver-specific component in VS Code. Porting it to Tauri presents a hard architectural choice: (a) keep tsserver as an external Node.js subprocess and re-implement the Content-Length framing + request/response/event dispatch in Rust (the wire protocol is documented and stable); or (b) migrate to the `typescript-language-server` bridge project (https://github.com/typescript-language-server/typescript-language-server, maintained, v5.2.0 released May 2026) which wraps tsserver in a standard LSP server, allowing a Rust host to use generic `tower-lsp` or `lsp-types` crates instead of reimplementing the proprietary protocol. Option (b) forfeits some tsserver-exclusive features (e.g. `mapCode`, `preparePasteEdits`, region semantic diagnostics) that have no LSP equivalent yet, but option (a) requires maintaining a bespoke protocol adapter. A third path is to wait for TypeScript 7 (currently being rewritten in Go as `typescript-go`), which Microsoft states will include a native LSP implementation and is intended to supersede the bridge layer entirely.

**Sources consulted:**

- https://github.com/microsoft/TypeScript/wiki/Standalone-Server-%28tsserver%29
- https://raw.githubusercontent.com/microsoft/TypeScript/main/src/server/protocol.ts (3,321 lines, 346 exported types)
- https://github.com/typescript-language-server/typescript-language-server
- https://www.chia1104.dev/en-US/posts/typescript-tsserver-lsp-development-mindset (HTTP 403; content summarised via WebSearch)
- WebSearch: "tsserver protocol vs LSP Language Server Protocol differences JSON framing stdio 2024"
