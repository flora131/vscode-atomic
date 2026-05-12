---
source_url: https://github.com/anomalyco/opencode/tree/dev/packages
fetched_at: 2026-05-11
fetch_method: html-parse
topic: OpenCode SDK createOpencode({ port: 0 }) and session.create behavior
---

# OpenCode SDK / server behavior relevant to headless session creation

Authoritative sources consulted:

- SDK source: https://raw.githubusercontent.com/anomalyco/opencode/dev/packages/sdk/js/src/index.ts
- SDK server launcher: https://raw.githubusercontent.com/anomalyco/opencode/dev/packages/sdk/js/src/server.ts
- Generated client error behavior: https://raw.githubusercontent.com/anomalyco/opencode/dev/packages/sdk/js/src/gen/client/client.gen.ts
- Generated SDK session method: https://raw.githubusercontent.com/anomalyco/opencode/dev/packages/sdk/js/src/gen/sdk.gen.ts
- Session API schema: https://raw.githubusercontent.com/anomalyco/opencode/dev/packages/opencode/src/server/routes/instance/httpapi/groups/session.ts
- Session API handler: https://raw.githubusercontent.com/anomalyco/opencode/dev/packages/opencode/src/server/routes/instance/httpapi/handlers/session.ts
- Session service: https://raw.githubusercontent.com/anomalyco/opencode/dev/packages/opencode/src/session/session.ts
- Server lifecycle: https://raw.githubusercontent.com/anomalyco/opencode/dev/packages/opencode/src/server/server.ts
- Database lifecycle: https://raw.githubusercontent.com/anomalyco/opencode/dev/packages/opencode/src/storage/db.ts
- Sync event storage: https://raw.githubusercontent.com/anomalyco/opencode/dev/packages/opencode/src/sync/index.ts
- Session projector: https://raw.githubusercontent.com/anomalyco/opencode/dev/packages/opencode/src/session/projectors.ts
- Instance store: https://raw.githubusercontent.com/anomalyco/opencode/dev/packages/opencode/src/project/instance-store.ts
- GitHub issue/PR search results around database locks, SDK spawn, empty session body, and server close.

Key findings:

- `createOpencode()` creates a server with `createOpencodeServer()` and a client pointed at `server.url`.
- SDK default server options are hostname `127.0.0.1`, port `4096`, timeout `5000`; caller options override these.
- SDK launches `opencode serve --hostname=... --port=...` and waits until stdout starts with `opencode server listening`, parsing the URL from that line.
- OpenCode server treats explicit `port: 0` specially: first tries 4096, then an arbitrary free port.
- SDK server object exposes `close()` but it is synchronous/non-awaitable and calls `stop(proc)`.
- Server-side listener `stop(close?)` is async and can force-close active HTTP/websocket connections, but SDK `close()` does not expose a promise.
- `POST /session` success schema is `Session.Info`; declared error is `400 BadRequest`.
- Session create handler is raw: empty request body maps to `{}`; non-empty body is JSON-parsed and schema-decoded; malformed JSON/schema failure returns 400.
- Current generated client behavior for non-2xx without `throwOnError` returns an object with `error` and no `data`; with `responseStyle: data` it returns `undefined`.
- `session.create()` is generated as `client.post({ url: "/session", Content-Type: application/json, ...options })`.
- Session creation calls `SessionShare.create()` -> `session.create()` -> `createNext()` -> `sync.run(Session.Event.Created, ...)`.
- `Session.Event.Created` projector inserts into `SessionTable`.
- Sync event writes use SQLite transactions, including an `immediate` transaction in `SyncEvent.run()`.
- Database path defaults to `Global.Path.data/opencode.db` unless `OPENCODE_DB` is set; if `OPENCODE_DB` is `:memory:` or absolute it is used as-is, otherwise under `Global.Path.data`.
- DB PRAGMAs currently include `journal_mode = WAL`, `synchronous = NORMAL`, `busy_timeout = 5000`, `cache_size = -64000`, `foreign_keys = ON`, and passive WAL checkpoint.
- InstanceStore caches one instance per resolved directory and disposes entries on finalizer/server shutdown. `serve` loads instances per request using directory query/header/default cwd; long-lived servers can retain per-directory instance state while active.

Relevant issues/PRs:

- #24640 fixed session creation with empty request bodies: https://github.com/anomalyco/opencode/pull/24640
- #25507 notes `serve` does not need an ambient instance and loads per request: https://github.com/anomalyco/opencode/pull/25507
- #3841 reported SDK `server.close()` was unawaitable and caused restart/port reuse issues: https://github.com/anomalyco/opencode/issues/3841
- #20762 / #20763 fixed SDK process spawn on Windows by switching to cross-spawn: https://github.com/anomalyco/opencode/issues/20762 and https://github.com/anomalyco/opencode/pull/20763
- #19521, #21215, #20935, #21000, #21443 document/describe SQLite lock contention and multi-process concurrency problems:
  - https://github.com/anomalyco/opencode/issues/19521
  - https://github.com/anomalyco/opencode/issues/21215
  - https://github.com/anomalyco/opencode/issues/20935
  - https://github.com/anomalyco/opencode/issues/21000
  - https://github.com/anomalyco/opencode/issues/21443
- #20999 mitigated some DB locking from high-volume tool output but notes multi-process locking may still occur: https://github.com/anomalyco/opencode/pull/20999
