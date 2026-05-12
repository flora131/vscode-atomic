### Files Analysed

1. `/home/norinlavaee/projects/vscode-atomic/extensions/debug-server-ready/src/extension.ts` (394 lines)
2. `/home/norinlavaee/projects/vscode-atomic/extensions/debug-server-ready/package.json` (221 lines)

---

### Per-File Notes

#### `extensions/debug-server-ready/src/extension.ts`

**Role:** Implements the entire runtime logic of the debug-server-ready extension. Registers VS Code extension hooks, creates `ServerReadyDetector` instances per debug session, scans debug adapter output and terminal data for port/URL patterns, and triggers follow-up actions (open browser externally, launch browser debugger, or start another named debug config).

**Key Symbols and Locations**

- `PATTERN` (line 10): Default regex string `'listening on.* (https?://\\S+|[0-9]+)'` used when the user does not supply their own pattern in `serverReadyAction.pattern`.
- `URI_PORT_FORMAT` / `URI_FORMAT` (lines 11–12): Template strings `'http://localhost:%s'` and `'%s'`. The former is applied when the captured group is a bare port number; the latter when a full URL is captured.
- `WEB_ROOT` (line 13): Default `'${workspaceFolder}'` passed as the `webRoot` to browser debug sessions.
- `CONTROL_SEQUENCES` regex (lines 26–33): Compiled from `CSI_SEQUENCE`, `OSC_SEQUENCE`, and `ESC_SEQUENCE` to strip ANSI escape codes. Used by `removeAnsiEscapeCodes` (lines 39–45) before pattern matching on terminal data; the comment at line 37 explicitly ties this to `src/vs/base/common/strings.ts:736` in core.
- `Trigger` class (lines 47–57): A simple boolean latch (`_fired` flag). Guards `detectPattern` so the action fires only once per detector lifetime.
- `ServerReadyDetector` class (lines 59–321): Core class, extends `vscode.Disposable`.
  - Static map `detectors` (line 61): `Map<vscode.DebugSession, ServerReadyDetector>` keyed by session handle.
  - Static `terminalDataListener` (line 62): Shared singleton listener for `vscode.window.onDidWriteTerminalData`. Created lazily in `startListeningTerminalData` (lines 99–121) so it is not registered if no session has `serverReadyAction`.
  - `start(session)` (lines 71–81): Factory. Creates a detector only when `session.configuration.serverReadyAction` exists; idempotent for the same session.
  - `stop(session)` (lines 83–90): Removes from the static map, fires the stopped emitter to trigger any `killOnServerStop` cleanup, then disposes the detector.
  - `rememberShellPid(session, pid)` (lines 92–97): Associates the shell process ID (received via the `runInTerminal` response from the debug adapter) with a detector so that terminal data can be routed to the right session by PID match.
  - Constructor (lines 123–134): Re-uses the parent session's `Trigger` when a child session is created (`session.parentSession` path at line 127–129); compiles the user-supplied or default pattern into `this.regexp`.
  - `detectPattern(s)` (lines 145–155): Executes `this.regexp` against stripped text. On first match calls `openExternalWithString` then fires the trigger.
  - `openExternalWithString` (lines 157–185): Validates the `uriFormat` field for correct `%s` count. Resolves the final URI using `util.format(format, captureString)` (line 181), then delegates to `openExternalWithUri`.
  - `openExternalWithUri` (lines 187–216): Dispatches on `args.action` (defaulting to `'openExternally'`):
    - `'openExternally'` → `vscode.env.openExternal(vscode.Uri.parse(uri))` (line 193)
    - `'debugWithChrome'` → `debugWithBrowser('pwa-chrome', ...)` (line 197)
    - `'debugWithEdge'` → `debugWithBrowser('pwa-msedge', ...)` (line 200)
    - `'startDebugging'` → `startDebugSession(...)` with either `args.config` or `args.name` (lines 205–209)
  - `debugWithBrowser` (lines 218–248): When `killOnServerStop` is false, directly calls `startBrowserDebugSession`. When true, generates a UUID tracker (line 225), races `catchStartedDebugSession` against the UUID predicate (line 227), calls `startBrowserDebugSession` with the tracker embedded in the config as `_debugServerReadySessionId` (line 229), then subscribes to the parent session's stop event to call `vscode.debug.stopDebugging(createdSession)` (line 245).
  - `startBrowserDebugSession` (lines 250–259): Calls `vscode.debug.startDebugging` with a hardcoded config object of type `pwa-chrome` or `pwa-msedge`, supplying `url`, `webRoot`, and optional tracker ID.
  - `startDebugSession` (lines 268–297): Mirrors the same `killOnServerStop` branching but for arbitrary named or inline configs. Uses `x.name === name` as the predicate when catching the created session (line 276).
  - `catchStartedDebugSession` (lines 299–320): Returns a `Promise<vscode.DebugSession | undefined>`. Subscribes to `vscode.debug.onDidStartDebugSession` and a cancellation token. Resolves with the session as soon as the predicate matches, or `undefined` on cancellation.

**`activate` function (lines 323–351)**
- Subscribes to `vscode.debug.onDidStartDebugSession` (line 325): calls `ServerReadyDetector.start(session)` and then `startListeningTerminalData()` if a detector was created.
- Subscribes to `vscode.debug.onDidTerminateDebugSession` (line 334): calls `ServerReadyDetector.stop(session)`.
- Registers a wildcard `DebugConfigurationProvider` via `vscode.debug.registerDebugConfigurationProvider('*', ...)` (line 340): intercepts `resolveDebugConfigurationWithSubstitutedVariables`, and for each new debugger type that has a `serverReadyAction`, calls `startTrackerForType` once (tracked in the local `trackers` Set at line 338).

**`startTrackerForType` function (lines 353–393)**
- Registers a `DebugAdapterTrackerFactory` for the given debugger type via `vscode.debug.registerDebugAdapterTrackerFactory(type, ...)` (line 356).
- `onDidSendMessage` handler (lines 363–379): Inspects DAP messages from the adapter. For `output` events with categories `console`, `stderr`, or `stdout`, calls `detector.detectPattern(m.body.output)`. For `runInTerminal` requests with `kind === 'integrated'`, stores `m.seq` as `runInTerminalRequestSeq`.
- `onWillReceiveMessage` handler (lines 382–387): Matches the `runInTerminal` response by `request_seq` and calls `ServerReadyDetector.rememberShellPid(session, m.body.shellProcessId)` to associate the spawned shell PID with the session for later terminal data routing.

**Control Flow Summary**

1. Debug session starts → `onDidStartDebugSession` fires → `ServerReadyDetector.start(session)` creates detector + installs terminal listener.
2. Simultaneously, `resolveDebugConfigurationWithSubstitutedVariables` fires once per debugger type → registers a `DebugAdapterTracker`.
3. DAP `output` events arrive via `onDidSendMessage` → `detectPattern` scans for the port/URL pattern.
4. Terminal data arrives via `onDidWriteTerminalData` → ANSI stripped → routed by PID or broadcast → `detectPattern` scans.
5. On first match, `openExternalWithUri` fires the configured action and the `Trigger` latch prevents re-firing.
6. If `killOnServerStop`, a stop listener on the parent session tears down the child browser or debug session when the server stops.
7. `onDidTerminateDebugSession` → `ServerReadyDetector.stop` → fires stop emitter, disposes detector.

**Dependencies**
- `vscode` extension API (entire public debug, env, window namespaces)
- Node built-ins: `util` (for `util.format`), `crypto` (for `randomUUID`)
- No external npm packages

---

#### `extensions/debug-server-ready/package.json`

**Role:** Extension manifest declaring activation, capabilities, and the JSON Schema for the `serverReadyAction` property injected into all debugger launch configurations.

**Key Details**

- `activationEvents: ["onDebugResolve"]` (line 13): Extension is activated when any debug configuration is about to be resolved, ensuring the `resolveDebugConfigurationWithSubstitutedVariables` hook is in place before any debug session starts.
- `enabledApiProposals: ["terminalDataWriteEvent"]` (lines 21–23): The extension depends on a proposed VS Code API (`vscode.window.onDidWriteTerminalData`) for reading terminal output. This is a non-stable API surface.
- `contributes.debuggers[0].type: "*"` (line 33): Wildcard applies the `serverReadyAction` configuration schema contribution to every debugger type.
- The `serverReadyAction` property uses a `oneOf` with four variants (lines 37–206):
  1. Object with `action: "openExternally"` — fields: `pattern`, `uriFormat`, `killOnServerStop`.
  2. Object with `action: "debugWithChrome" | "debugWithEdge"` — adds `webRoot`.
  3. Object with `action: "startDebugging"` + required `name` string — references a launch config by name.
  4. Object with `action: "startDebugging"` + required `config` object — inline debug configuration.
- All four variants include `additionalProperties: false`, which enforces strict schema validation.
- `capabilities.virtualWorkspaces: false` (line 17): Extension disabled in virtual workspaces (no filesystem).
- `capabilities.untrustedWorkspaces.supported: true` (line 19): Permitted in restricted/untrusted workspaces.

---

### Cross-Cutting Synthesis

The debug-server-ready extension is a self-contained VS Code extension (~394 TS LOC) that monitors two distinct data streams — DAP adapter messages (via `DebugAdapterTrackerFactory`) and integrated terminal output (via the proposed `onDidWriteTerminalData` API) — to detect a user-configurable regex pattern signaling server startup. Both streams converge at `ServerReadyDetector.detectPattern`, which is guarded by a one-shot `Trigger` latch and an ANSI-stripping function copied from core strings utilities. Once fired, a dispatch table in `openExternalWithUri` routes to one of four outcomes: open URL externally, launch a Chrome/Edge browser debug session, or start an arbitrary named/inline debug session. An optional `killOnServerStop` mode adds child session lifecycle management by coupling a UUID-tagged session promise race with the parent session's stop event.

For a Tauri/Rust port, this extension illustrates two deep VS Code API dependencies that have no direct Tauri equivalents: the `DebugAdapterTracker` intercept pipeline (DAP message interception between VS Code's debug infrastructure and the adapter process) and the proposed `terminalDataWriteEvent` API (read access to terminal I/O streams). Both are provided by the VS Code extension host runtime. In a Tauri environment, the debug adapter protocol would need to be implemented independently (e.g., a Rust DAP client/server bridge), and terminal output interception would require a new IPC mechanism between the Rust shell process manager and any equivalent extension logic. The wildcard `"type": "*"` schema contribution also relies on VS Code's JSON schema merging capability for `launch.json`, which has no counterpart in Tauri's default plugin model.

---

### Out-of-Partition References

- `src/vs/base/common/strings.ts:736` — The `removeAnsiEscapeCodes` function and ANSI control sequence regexes in `extension.ts:26–44` are explicitly noted as copies from this core utility. Any Tauri port that needs the same ANSI stripping would refer to this origin.
- `vscode.debug` namespace (VS Code core) — `onDidStartDebugSession`, `onDidTerminateDebugSession`, `registerDebugConfigurationProvider`, `registerDebugAdapterTrackerFactory`, `startDebugging`, `stopDebugging` are all core debug APIs consumed by this extension. Their implementations live in the VS Code core debug subsystem, not within this partition.
- `vscode.window.onDidWriteTerminalData` (proposed API) — The terminal data event implementation resides in VS Code's terminal subsystem in core. The extension depends on `enabledApiProposals: ["terminalDataWriteEvent"]` to access it.
- `vscode.env.openExternal` — Used at `extension.ts:193`; implemented in core platform layer for cross-platform URL opening (Electron shell on desktop).
