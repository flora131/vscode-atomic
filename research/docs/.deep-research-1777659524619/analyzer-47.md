### Files Analysed

1. `/home/norinlavaee/projects/vscode-atomic/extensions/debug-server-ready/src/extension.ts` — 393 LOC
2. `/home/norinlavaee/projects/vscode-atomic/extensions/debug-server-ready/esbuild.mts` — 18 LOC

---

### Per-File Notes

#### `extensions/debug-server-ready/src/extension.ts`

**Role**

Implements the `debug-server-ready` VS Code extension. It monitors debug session output (both DAP event streams and integrated terminal writes) for a configurable regex pattern signalling that a server is ready, then automatically opens a browser, launches a child debug session (Chrome/Edge), or calls an arbitrary launch configuration.

---

**Key Symbols**

- `PATTERN` (line 10) — Default regex string `'listening on.* (https?://\\S+|[0-9]+)'`, compiled into a `RegExp` per session at line 133.
- `URI_PORT_FORMAT` (line 11) — `'http://localhost:%s'`, used when the captured group is a bare port number (line 173).
- `URI_FORMAT` (line 12) — `'%s'`, used when the captured group is already a full URI (line 173).
- `WEB_ROOT` (line 13) — `'${workspaceFolder}'`, the default `webRoot` for browser debug sessions (line 257).
- `interface ServerReadyAction` (lines 15–23) — Describes the `serverReadyAction` configuration block: `pattern`, `action`, `uriFormat`, `webRoot`, `name`, `config`, `killOnServerStop`.
- `CONTROL_SEQUENCES` (lines 29–33) — Composite regex combining CSI, OSC, and ESC ANSI escape code patterns, used by `removeAnsiEscapeCodes`.
- `removeAnsiEscapeCodes(str)` (lines 39–45) — Strips ANSI escape sequences from terminal output before pattern matching.
- `class Trigger` (lines 47–57) — A one-shot boolean latch; `fire()` sets `_fired = true`; `hasFired` getter returns it. Prevents the action from firing more than once per session.
- `class ServerReadyDetector extends vscode.Disposable` (lines 59–321) — Central orchestrator. Holds a static `detectors` Map keyed by `vscode.DebugSession` (line 61) and a static `terminalDataListener` (line 62).

---

**Control Flow — `activate`**

`activate` (lines 323–351) registers three things on `context.subscriptions`:

1. `vscode.debug.onDidStartDebugSession` (line 325) — When any debug session starts with `serverReadyAction` in its configuration, calls `ServerReadyDetector.start(session)` (line 327) to create/fetch a detector, then calls `ServerReadyDetector.startListeningTerminalData()` (line 329).

2. `vscode.debug.onDidTerminateDebugSession` (line 334) — Calls `ServerReadyDetector.stop(session)` (line 335), which removes the detector from the static map, fires `stoppedEmitter`, and disposes the instance (lines 83–89).

3. `vscode.debug.registerDebugConfigurationProvider('*', ...)` (lines 340–350) — The `resolveDebugConfigurationWithSubstitutedVariables` hook checks whether this debug type already has a tracker registered (via the `trackers` Set, line 338). If not, it adds the type and calls `startTrackerForType(context, type)` (line 345).

---

**Control Flow — DAP Tracker Factory (`startTrackerForType`)**

`startTrackerForType` (lines 353–393) registers a `vscode.DebugAdapterTrackerFactory` for the given debug type via `vscode.debug.registerDebugAdapterTrackerFactory` (line 356).

`createDebugAdapterTracker(session)` (line 357):
- Calls `ServerReadyDetector.start(session)` to get the detector.
- Returns a tracker object with two hooks:
  - `onDidSendMessage` (lines 362–380): Receives messages sent *from* the debug adapter to VS Code. If the message is a DAP `output` event (type `'event'`, event `'output'`) with category `console`, `stderr`, or `stdout`, it calls `detector.detectPattern(m.body.output)` (line 369). Additionally, if the message is a `runInTerminal` request with `kind === 'integrated'` (lines 376–379), it records `m.seq` in `runInTerminalRequestSeq`.
  - `onWillReceiveMessage` (lines 382–387): Receives messages going *from* VS Code *to* the debug adapter. If a matching `runInTerminal` response arrives (matched by `request_seq`), it calls `ServerReadyDetector.rememberShellPid(session, m.body.shellProcessId)` (line 385) to associate the terminal's shell PID with this session.

---

**Control Flow — Terminal Data Listening**

`ServerReadyDetector.startListeningTerminalData()` (lines 99–121) is a static method that installs at most one `vscode.window.onDidWriteTerminalData` listener (line 101). On each terminal write:
1. Awaits `e.terminal.processId` (line 104).
2. Strips ANSI codes with `removeAnsiEscapeCodes(e.data)` (line 105).
3. First iterates all detectors looking for one whose `shellPid === pid` (lines 106–110), calls `detectPattern` on the first match, and returns.
4. If no PID-matched detector was found, iterates all detectors trying `detectPattern` until one returns `true` (lines 113–117).

---

**Control Flow — Pattern Detection**

`detectPattern(s: string): boolean` (lines 145–155):
- Short-circuits if `trigger.hasFired` is already true.
- Runs `this.regexp.exec(s)`.
- On a match, calls `openExternalWithString(session, matches[1] || '')` (line 149) and fires the trigger (line 150).

`openExternalWithString` (lines 157–185):
- If `captureString` is empty, uses `args.uriFormat` directly (no substitution).
- Otherwise selects `URI_PORT_FORMAT` or `URI_FORMAT` based on whether the capture is a bare number (line 173), then calls `util.format(format, captureString)` (line 181).
- Delegates to `openExternalWithUri(session, uri)` (line 184).

`openExternalWithUri` (lines 187–216) dispatches on `args.action`:
- `'openExternally'` (line 192–194): `vscode.env.openExternal(vscode.Uri.parse(uri))`.
- `'debugWithChrome'` (line 196–198): Calls `debugWithBrowser('pwa-chrome', session, uri)`.
- `'debugWithEdge'` (line 200–202): Calls `debugWithBrowser('pwa-msedge', session, uri)`.
- `'startDebugging'` (lines 204–210): Calls `startDebugSession` with either `args.config` or `args.name`.

---

**Control Flow — Browser Debug Session Launch**

`debugWithBrowser(type, session, uri)` (lines 218–248):
- If `args.killOnServerStop` is false: directly calls `startBrowserDebugSession` (line 222) and returns.
- If `killOnServerStop` is true:
  1. Generates a `trackerId = randomUUID()` (line 225).
  2. Creates a `CancellationTokenSource` (line 226).
  3. Calls `catchStartedDebugSession` with a predicate checking `session.configuration._debugServerReadySessionId === trackerId` (line 227) — this returns a Promise resolving to the newly started session.
  4. Calls `startBrowserDebugSession(type, session, uri, trackerId)` (line 229), passing `trackerId` embedded as `_debugServerReadySessionId` in the launch config.
  5. On abort, cancels and disposes the CTS.
  6. On success, attaches a `onDidSessionStop` listener (line 242) that calls `vscode.debug.stopDebugging(createdSession)` when the parent session stops.

`startBrowserDebugSession(type, session, uri, trackerId?)` (lines 250–259): Calls `vscode.debug.startDebugging` with a hardcoded `request: 'launch'` config embedding `type`, `url`, `webRoot`, and optionally `_debugServerReadySessionId`.

`startDebugSession(session, name, config?)` (lines 268–297): Mirrors the `killOnServerStop` branching from `debugWithBrowser` but uses `vscode.debug.startDebugging(session.workspaceFolder, config ?? name)` (line 271 / line 278), and the `catchStartedDebugSession` predicate is `x.name === name` (line 276).

`catchStartedDebugSession(predicate, cancellationToken)` (lines 299–320): Returns a Promise that resolves when `vscode.debug.onDidStartDebugSession` fires a session matching `predicate`, or `undefined` if the cancellation token fires first. Both listeners are tracked in `this.disposables` and self-disposed on resolution.

---

**Data Flow**

```
DAP adapter output event
  → onDidSendMessage (line 363)
  → detector.detectPattern(m.body.output) (line 369)
  → openExternalWithString → openExternalWithUri
  → vscode.env.openExternal | startBrowserDebugSession | startDebugSession

Integrated terminal write
  → vscode.window.onDidWriteTerminalData (line 101)
  → removeAnsiEscapeCodes (line 105)
  → detector.detectPattern(str) (line 107 or 115)
  → same path as above

runInTerminal request (seq recorded line 378)
  → runInTerminal response (line 383)
  → ServerReadyDetector.rememberShellPid(session, shellProcessId) (line 385)
  → stored as detector.shellPid (line 96)
  → used to route terminal data to correct detector (line 107)
```

---

**Dependencies**

- `vscode` — Extension API: `debug`, `window`, `env`, `Uri`, `l10n`, `EventEmitter`, `Disposable`, `CancellationTokenSource`.
- `util` (Node.js built-in) — `util.format` for URI substitution (line 181).
- `crypto` (Node.js built-in) — `randomUUID` for `killOnServerStop` tracking IDs (line 8, line 225).

---

#### `extensions/debug-server-ready/esbuild.mts`

**Role**

Build script that bundles `extension.ts` into the `dist/` output directory using a shared esbuild helper.

**Key Symbols**

- `srcDir` (line 8) — Absolute path to `extensions/debug-server-ready/src/`.
- `outDir` (line 9) — Absolute path to `extensions/debug-server-ready/dist/`.
- `run(...)` call (lines 11–18) — Invokes the shared build runner with `platform: 'node'`, a single entry point `'extension'` mapping to `src/extension.ts`, and passes `process.argv` for CLI flags (e.g., `--watch`).

**Control Flow**

Executes immediately on import. No conditional logic; the entire file is a single `run(...)` call delegated to `../esbuild-extension-common.mts`.

**Dependencies**

- `node:path` — `path.join` for constructing `srcDir` and `outDir`.
- `../esbuild-extension-common.mts` — Shared build helper that wraps the esbuild API; provides the `run` function.

---

### Cross-Cutting Synthesis

The `debug-server-ready` extension implements a two-channel approach to detecting a server-ready signal: a DAP tracker intercepts structured `output` events from the debug adapter, while a single shared terminal-data listener covers process output written directly to an integrated terminal. Both channels funnel text through `removeAnsiEscapeCodes` and then into `ServerReadyDetector.detectPattern`, which applies a per-session regex and fires exactly once per session via the one-shot `Trigger` latch. When the pattern fires, `openExternalWithUri` dispatches to three actions — `openExternally` (plain browser open), `debugWithChrome`/`debugWithEdge` (child browser debug session via `pwa-chrome`/`pwa-msedge`), or `startDebugging` (arbitrary named launch config). The `killOnServerStop` flag adds lifecycle coupling: a UUID is embedded in the child session's configuration, `catchStartedDebugSession` waits for that session to appear, and a `stoppedEmitter` listener tears it down when the parent session ends. The build system is entirely delegated to a shared `esbuild-extension-common.mts` helper invoked from `esbuild.mts`.

---

### Out-of-Partition References

- `../esbuild-extension-common.mts` — Shared esbuild runner imported by `esbuild.mts` line 6; provides the `run` function used at line 11.
- `vscode` extension API — `vscode.debug`, `vscode.window`, `vscode.env`, `vscode.Uri`, `vscode.l10n`, `vscode.EventEmitter`, `vscode.Disposable`, `vscode.CancellationTokenSource`; all consumed throughout `extension.ts`.
- `src/vs/base/common/strings.ts` — The ANSI escape code regexes (`CSI_SEQUENCE`, `OSC_SEQUENCE`, `ESC_SEQUENCE`) are copied from this core file, as documented in the comment at `extension.ts:36–37`.
- `pwa-chrome` debug adapter type — Referenced as a string literal at `extension.ts:197`; implemented in the `ms-vscode.js-debug` extension.
- `pwa-msedge` debug adapter type — Referenced as a string literal at `extension.ts:201`; implemented in the `ms-vscode.js-debug` extension.
