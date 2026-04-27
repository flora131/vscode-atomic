### Files Analysed

- `/Users/norinlavaee/vscode-atomic/extensions/debug-server-ready/src/extension.ts` (394 lines)
- `/Users/norinlavaee/vscode-atomic/extensions/debug-server-ready/package.json` (222 lines)

---

### Per-File Notes

#### `/Users/norinlavaee/vscode-atomic/extensions/debug-server-ready/src/extension.ts`

- **Role:** Implements the entire runtime logic of the `debug-server-ready` extension. Defines the `ServerReadyDetector` class, the `activate()` entry point, and the `startTrackerForType()` helper. No other source files exist in this extension.

- **Key symbols:**
  - `PATTERN` (`extension.ts:10`) — Default regex string `'listening on.* (https?://\\S+|[0-9]+)'` matching port numbers or full URLs.
  - `URI_PORT_FORMAT` / `URI_FORMAT` (`extension.ts:11-12`) — Format strings used when constructing a URI from a captured port (`http://localhost:%s`) or a captured URL (`%s`).
  - `WEB_ROOT` (`extension.ts:13`) — Default `webRoot` value `'${workspaceFolder}'` injected into browser debug sessions.
  - `interface ServerReadyAction` (`extension.ts:15-23`) — Typed shape of the `serverReadyAction` property in a launch configuration: `pattern`, `action`, `uriFormat`, `webRoot`, `name`, `config`, `killOnServerStop`.
  - `CONTROL_SEQUENCES` / `removeAnsiEscapeCodes()` (`extension.ts:26-45`) — A compound regex covering CSI, OSC, and ESC ANSI sequences, used to strip terminal control codes before pattern matching.
  - `class Trigger` (`extension.ts:47-57`) — A one-shot boolean flag (`_fired`). `hasFired` getter and `fire()` method ensure the ready-action fires only once per session.
  - `class ServerReadyDetector extends vscode.Disposable` (`extension.ts:59-321`) — Core detection engine. Holds a static `Map<vscode.DebugSession, ServerReadyDetector>` at `extension.ts:61` and a shared static `terminalDataListener` at `extension.ts:62`.
  - `ServerReadyDetector.start()` (`extension.ts:71-81`) — Factory: creates or retrieves a detector for a session iff `session.configuration.serverReadyAction` is set.
  - `ServerReadyDetector.stop()` (`extension.ts:83-90`) — Removes the detector from the map, fires `sessionStopped`, and disposes it.
  - `ServerReadyDetector.rememberShellPid()` (`extension.ts:92-97`) — Stores the shell PID returned by a `runInTerminal` response for accurate terminal data correlation.
  - `ServerReadyDetector.startListeningTerminalData()` (`extension.ts:99-121`) — Lazily subscribes to `vscode.window.onDidWriteTerminalData`. On each event, strips ANSI codes, resolves terminal PID, and routes the data to the matching detector.
  - `detectPattern()` (`extension.ts:145-155`) — Runs `this.regexp` against stripped terminal output. If matched and trigger not yet fired, calls `openExternalWithString()` and fires the trigger.
  - `openExternalWithString()` (`extension.ts:157-185`) — Constructs a URI from the captured group, dispatching to `openExternalWithUri()`. Validates that `%s` count in `uriFormat` matches availability of a capture group.
  - `openExternalWithUri()` (`extension.ts:187-216`) — Switch on `args.action`: `openExternally` calls `vscode.env.openExternal()`; `debugWithChrome`/`debugWithEdge` calls `debugWithBrowser()`; `startDebugging` calls `startDebugSession()`.
  - `debugWithBrowser()` (`extension.ts:218-248`) — Launches a browser debug session. When `killOnServerStop` is `true`, it uses `randomUUID()` to tag the spawned session and subscribes `onDidSessionStop` to call `vscode.debug.stopDebugging()` on the child.
  - `startBrowserDebugSession()` (`extension.ts:250-259`) — Thin wrapper around `vscode.debug.startDebugging()` constructing a browser launch config (`type`, `name`, `request: 'launch'`, `url`, `webRoot`, optional `_debugServerReadySessionId`).
  - `startDebugSession()` (`extension.ts:268-297`) — Mirrors `debugWithBrowser()` logic for arbitrary configs: starts a debug session by name or by config object, optionally wires `killOnServerStop`.
  - `catchStartedDebugSession()` (`extension.ts:299-320`) — Returns a `Promise<vscode.DebugSession | undefined>` that resolves when `vscode.debug.onDidStartDebugSession` fires a session satisfying the given predicate, or when the cancellation token fires.
  - `activate()` (`extension.ts:323-351`) — Extension entry point. Subscribes to `onDidStartDebugSession`, `onDidTerminateDebugSession`, and registers a wildcard `DebugConfigurationProvider` (`resolveDebugConfigurationWithSubstitutedVariables`) that calls `startTrackerForType()` once per debug type.
  - `startTrackerForType()` (`extension.ts:353-393`) — Registers a `DebugAdapterTrackerFactory` for the given type. The factory's `onDidSendMessage` scans `output` events for console/stderr/stdout categories and calls `detectPattern()`; its `onWillReceiveMessage` intercepts `runInTerminal` responses to call `rememberShellPid()`.

- **Control flow:**
  1. On extension host startup, `activate()` registers three hooks.
  2. When a debug session starts (`onDidStartDebugSession`): if `serverReadyAction` is present, `ServerReadyDetector.start()` creates a detector and `startListeningTerminalData()` arms the terminal listener.
  3. The wildcard `DebugConfigurationProvider` fires during config resolution. Per unique debug type, `startTrackerForType()` registers a `DebugAdapterTrackerFactory`.
  4. Two parallel interception paths run simultaneously:
     - **Debug adapter output path**: The tracker's `onDidSendMessage` receives DAP `output` events and calls `detector.detectPattern()` directly on the output body.
     - **Terminal data path**: `onDidWriteTerminalData` fires; the static listener strips ANSI codes, looks up the detector by shell PID (obtained earlier from `runInTerminal` response), and calls `detectPattern()`.
  5. `detectPattern()` (`extension.ts:145`) checks `trigger.hasFired`. On first match, it invokes the action chain and fires the trigger permanently.
  6. When the debug session ends (`onDidTerminateDebugSession`), `ServerReadyDetector.stop()` cleans up the detector, fires `stoppedEmitter`, and disposes all listeners.

- **Data flow:**
  - Raw terminal bytes → `onDidWriteTerminalData` → `removeAnsiEscapeCodes()` → `detectPattern()` → `regexp.exec()` → captured group string.
  - DAP output event body `.output` string → `detectPattern()` → `regexp.exec()` → captured group string.
  - Captured group string → `openExternalWithString()` → format string substitution via `util.format()` (`extension.ts:181`) → URI string.
  - URI string → `openExternalWithUri()` → `vscode.env.openExternal()` or `vscode.debug.startDebugging()`.
  - `runInTerminal` request/response pair tracked via `runInTerminalRequestSeq` (`extension.ts:377`) → `m.body.shellProcessId` stored on detector as `shellPid` (`extension.ts:385`).
  - `killOnServerStop=true` path: `randomUUID()` tag → `_debugServerReadySessionId` field in child launch config → `catchStartedDebugSession()` predicate → `onDidSessionStop` subscription → `vscode.debug.stopDebugging()`.

- **Dependencies:**
  - `vscode` API: `debug.*`, `env.openExternal`, `window.onDidWriteTerminalData`, `Uri`, `EventEmitter`, `CancellationTokenSource`, `Disposable` (`extension.ts:6`).
  - `util` (Node.js): `util.format()` for `%s` substitution (`extension.ts:7, 181`).
  - `crypto` (Node.js): `randomUUID()` for session tracking (`extension.ts:8, 225`).
  - `terminalDataWriteEvent` API proposal declared in `package.json:22` (required for `onDidWriteTerminalData`).

---

#### `/Users/norinlavaee/vscode-atomic/extensions/debug-server-ready/package.json`

- **Role:** Extension manifest. Declares metadata, activation events, API proposal requirements, build scripts, and the `contributes.debuggers` schema contributions that add `serverReadyAction` to the `launch` configuration of every debug type (`"type": "*"`).

- **Key symbols:**
  - `activationEvents: ["onDebugResolve"]` (`package.json:12-14`) — Extension activates whenever any debug configuration is being resolved, regardless of type.
  - `enabledApiProposals: ["terminalDataWriteEvent"]` (`package.json:21-23`) — Opts into the proposed `vscode.window.onDidWriteTerminalData` API used in `extension.ts:101`.
  - `contributes.debuggers[0].type: "*"` (`package.json:33`) — Injects the `serverReadyAction` JSON schema property into `launch` configurations across all debugger types.
  - Four `oneOf` schema variants for `serverReadyAction` (`package.json:37-205`):
    1. `action: "openExternally"` with optional `pattern`, `uriFormat`, `killOnServerStop` (`package.json:38-74`).
    2. `action: "debugWithChrome" | "debugWithEdge"` with `pattern`, `uriFormat`, `webRoot`, `killOnServerStop` (`package.json:75-119`).
    3. `action: "startDebugging"` requiring `name` (string) with `pattern`, `killOnServerStop` (`package.json:120-161`).
    4. `action: "startDebugging"` requiring `config` (object) with `pattern`, `killOnServerStop` (`package.json:162-205`).
  - `capabilities.virtualWorkspaces: false` (`package.json:16`) — Disabled in virtual workspaces.
  - `capabilities.untrustedWorkspaces.supported: true` (`package.json:17-19`) — Works in untrusted workspace mode.

- **Control flow:** Declarative only; no runtime logic. The JSON schema in `contributes.debuggers` drives IntelliSense and validation in `launch.json` editors.

- **Data flow:** Schema definitions propagate to VS Code's debug configuration resolver, which validates user-authored `serverReadyAction` blocks before passing them to the extension's `resolveDebugConfigurationWithSubstitutedVariables` hook.

- **Dependencies:** No runtime npm dependencies. `devDependencies` lists `@types/node: 22.x` for TypeScript type definitions (`package.json:214-216`). Requires VS Code engine `^1.32.0` (`package.json:9`).

---

### Cross-Cutting Synthesis

The `debug-server-ready` extension implements a two-path interception model entirely within a single TypeScript source file. Pattern detection operates through both the Debug Adapter Protocol output events (via `DebugAdapterTrackerFactory`) and raw terminal writes (via the proposed `onDidWriteTerminalData` API), with the shell PID from a `runInTerminal` DAP response used to correlate terminal streams to sessions. The `Trigger` class enforces one-shot semantics so the action fires exactly once per session (or once per parent-child session group when `parentSession` is set). The four action modes — external URI, Chrome/Edge browser debug, and arbitrary launch config — share a common `killOnServerStop` lifecycle wiring pattern using `randomUUID` tagging and `onDidStartDebugSession` observation. For a Tauri/Rust port, the critical dependencies are: the VS Code Extension API's `vscode.debug.*` namespace (session lifecycle, DAP adapter tracking, configuration providers), the proposed `terminalDataWriteEvent` API, `vscode.env.openExternal`, Node.js's `util.format` and `crypto.randomUUID`, and the JSON schema contribution mechanism in `package.json` for debug configuration IntelliSense. All of these represent VS Code host platform contracts that have no direct equivalent in Tauri's IPC or Rust extension model and would require purpose-built replacements covering DAP session management, terminal stream subscription, and URI launch orchestration.

---

### Out-of-Partition References

The following symbols are referenced by `extension.ts` but defined outside the `extensions/debug-server-ready/` partition:

- `vscode` API module — VS Code Extension Host runtime; implemented in `src/vs/workbench/api/` (multiple files).
- `vscode.debug.registerDebugAdapterTrackerFactory` — Implemented in `src/vs/workbench/api/common/extHostDebugService.ts`.
- `vscode.debug.registerDebugConfigurationProvider` — Implemented in `src/vs/workbench/api/common/extHostDebugService.ts`.
- `vscode.window.onDidWriteTerminalData` — Proposed API, implemented in `src/vs/workbench/api/common/extHostTerminalService.ts`.
- `vscode.debug.startDebugging` / `vscode.debug.stopDebugging` — Implemented in `src/vs/workbench/api/common/extHostDebugService.ts`.
- `vscode.env.openExternal` — Implemented in `src/vs/workbench/api/common/extHostEnv.ts`.
- `src/vs/base/common/strings.ts` — Source of the ANSI escape code regexes copied verbatim at `extension.ts:26-33` (comment at `extension.ts:36-38` cites commit `22a2a0e`).
- `pwa-chrome` / `pwa-msedge` debugger types — Provided by the `vscode-js-debug` extension, not in this partition.
