# Partition 47 of 79 — Findings

## Scope
`extensions/debug-server-ready/` (2 files, 411 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Codebase Locator Report: Debug Server Ready Extension (Partition 47/79)

**Research Question:** What it would take to port VS Code's core IDE functionality (editing, language intelligence, debugging, source control, terminal, navigation, etc.) from TypeScript/Electron to Tauri/Rust.

**Scope:** `extensions/debug-server-ready/` (2 files, ~411 LOC)

**Seed Pattern:** `vscode.debug.onDidStartDebugSession($$$)` — Listens on debug lifecycle events; defines public debug-event contract.

---

## Implementation

- **`extensions/debug-server-ready/src/extension.ts`** (393 LOC)
  - Main extension activation entry point (`activate()` function, line 323)
  - `ServerReadyDetector` class (lines 59-321) — Manages debug session state machine and pattern detection
  - Pattern matching engine for detecting server readiness indicators in debug output
  - Debug session lifecycle hooks:
    - `vscode.debug.onDidStartDebugSession()` (line 310, 325) — Listens for new debug sessions
    - `vscode.debug.onDidTerminateDebugSession()` (line 334) — Listens for session termination
  - Terminal data listener: `vscode.window.onDidWriteTerminalData()` (line 101)
  - ANSI escape code removal utility (lines 26-45, copied from core strings.ts)
  - Debug adapter tracker factory registration (line 356) — Intercepts DAP protocol messages
  - URI construction and action invocation logic (lines 157-216):
    - `openExternally` — Opens browser with vscode.env.openExternal()
    - `debugWithChrome` / `debugWithEdge` — Browser debugging via pwa-chrome/pwa-msedge adapters
    - `startDebugging` — Launches child debug configurations (lines 250-297)
  - Cancellation token handling for session lifecycle management (lines 299-320)

---

## Configuration

- **`extensions/debug-server-ready/package.json`** (221 lines)
  - Extension manifest with publisher="vscode", version 10.0.0
  - Activation trigger: `onDebugResolve` (line 13)
  - API proposals: `terminalDataWriteEvent` (line 22)
  - Debugger contribution schema (lines 30-212):
    - `serverReadyAction` configuration attribute supports 4 action types: `openExternally`, `debugWithChrome`, `debugWithEdge`, `startDebugging`
    - Pattern matching configuration (regex-based server readiness detection)
    - URI format templates with substitution placeholders (`%s` for port)
    - WebRoot specification for browser debuggers
    - `killOnServerStop` flag for lifecycle coupling
  - Engine requirement: vscode ^1.32.0
  - Untrusted workspace support enabled (line 18)

- **`extensions/debug-server-ready/tsconfig.json`**
  - Extends ../tsconfig.base.json
  - Includes vscode.d.ts type definitions (core API types)
  - Includes proposed API: `vscode.proposed.terminalDataWriteEvent.d.ts`
  - Output directory: ./out (compiled to dist via esbuild)

- **`extensions/debug-server-ready/.vscodeignore`**
  - Excludes src/, typescript configs, esbuild scripts, and build artifacts from final package

- **`extensions/debug-server-ready/esbuild.mts`**
  - Build script using esbuild-extension-common for Node.js platform
  - Entry point: src/extension.ts → dist/extension.js

- **`extensions/debug-server-ready/package.nls.json`**
  - Localization strings for configuration UI (16 entries)
  - Describes serverReadyAction feature in debug configuration schema

---

## Types / Interfaces

- **`ServerReadyAction` interface** (lines 15-23 in extension.ts)
  - `pattern: string` — Regex pattern for matching server readiness output
  - `action?: 'openExternally' | 'debugWithChrome' | 'debugWithEdge' | 'startDebugging'`
  - `uriFormat?: string` — Format string with %s substitution for port/URL
  - `webRoot?: string` — Document root for web debuggers
  - `name?: string` — Debug configuration name
  - `config?: vscode.DebugConfiguration` — Inline debug configuration object
  - `killOnServerStop?: boolean` — Lifecycle coupling flag

---

## Notable Clusters & Architecture Patterns

### Debug Lifecycle State Machine
The extension implements a sophisticated state machine around debug sessions:
- **Session creation/termination tracking** via `ServerReadyDetector.start()` / `ServerReadyDetector.stop()` (static methods, lines 71-90)
- **Detector registry**: `Map<vscode.DebugSession, ServerReadyDetector>` maintains 1:1 mapping (line 61)
- **Trigger coalescing**: Reuses parent session triggers to prevent duplicate notifications (lines 127-131)
- **Disposable pattern**: Extends vscode.Disposable for resource cleanup (line 59, 124)

### Terminal Output Monitoring & Pattern Matching
- **Shared global listener**: `onDidWriteTerminalData()` listener initialized once, filters by process ID (lines 99-121)
- **ANSI escape code stripping**: Applies CONTROL_SEQUENCES regex to sanitize output before pattern matching (lines 26-45)
- **Pattern detection engine**: Regex.exec() with capture group extraction (lines 145-155)
- **Fallback detection**: If process ID match fails, tries all detectors sequentially (lines 113-118)

### Debug Adapter Protocol (DAP) Integration
- **Tracker factory registration** (line 356): Intercepts low-level DAP messages
- **Message type discrimination** (lines 362-387):
  - `output` event capture (console/stderr/stdout) for pattern detection
  - `runInTerminal` request/response pair tracking to extract shell process IDs
- **Bidirectional hooks**: `onDidSendMessage()` and `onWillReceiveMessage()` (lines 362, 382)

### Child Session Lifecycle Coupling
- **UUID-based session tracking** (lines 225-227): Uses `_debugServerReadySessionId` to correlate parent/child sessions
- **Cancellation-aware promise pattern** (lines 299-320): Wraps vscode.debug.onDidStartDebugSession() with CancellationToken
- **Automatic cleanup**: Stop listener disposes child session when parent stops (lines 242-247, 291-296)

### Browser Debugging Integration
- **Adapter abstraction**: Supports both Chrome (pwa-chrome) and Edge (pwa-msedge) via type parameter (lines 196-201)
- **WebRoot propagation**: Passes workspace-relative paths to browser debuggers (line 256)
- **Browser session management** (lines 218-248): Handles killOnServerStop flag with session correlation

---

## Integration Points with VS Code Core

### Debug API Surface
- **`vscode.debug.onDidStartDebugSession`** — Main entry hook for session initialization
- **`vscode.debug.onDidTerminateDebugSession`** — Cleanup hook
- **`vscode.debug.startDebugging()`** — Launches child debug configurations
- **`vscode.debug.stopDebugging()`** — Terminates child sessions
- **`vscode.debug.registerDebugConfigurationProvider()`** — Intercepts debug configuration resolution
- **`vscode.debug.registerDebugAdapterTrackerFactory()`** — Registers DAP message interceptor

### Window & Environment API
- **`vscode.window.onDidWriteTerminalData()`** — Terminal output monitoring (proposed API)
- **`vscode.window.showErrorMessage()`** — UI error feedback (lines 167, 178)
- **`vscode.env.openExternal()`** — External URI opening (line 193)

### Configuration Access
- **`session.configuration.serverReadyAction`** — Extension-defined launch config attribute
- **`session.configuration._debugServerReadySessionId`** — Internal tracking field

---

## Port-to-Tauri/Rust Implications

### Critical Dependencies on TypeScript/Node Ecosystem
1. **Regex engine**: Uses JavaScript RegExp for pattern matching — would require Rust regex crate equivalents
2. **Process ID tracking**: Relies on `e.terminal.processId` and integration with VS Code's terminal subsystem
3. **UUID generation**: Uses Node.js `crypto.randomUUID()` (line 8)
4. **Localization**: Uses vscode.l10n API (lines 166, 177) — requires core localization system
5. **Event emitter pattern**: vscode.EventEmitter, vscode.Disposable abstractions (lines 64-65, 124)

### Architectural Adaptations Needed
1. **Debug Adapter Protocol (DAP) handling**: Currently relies on VS Code's built-in DAP message routing. Tauri port would need to implement or bind to DAP protocol handler
2. **Terminal integration**: Deep coupling with vscode.window.onDidWriteTerminalData() proposed API — requires integration with Tauri terminal infrastructure
3. **Session state management**: Current Map-based singleton registry would need to be replicated in Rust with thread-safe data structures (Arc<Mutex<>>)
4. **Cancellation system**: vscode.CancellationToken has no direct Rust equivalent; would need custom async cancellation mechanism
5. **Configuration schema**: Package.json debugger contribution schema would need equivalent declarative system in Tauri

### Rust Implementation Challenges
- **Async runtime compatibility**: Extension uses TypeScript async/await with vscode event system; Rust would require tokio or similar
- **IPC/RPC for child debug sessions**: Browser debugger spawning (debugWithChrome/debugWithEdge) currently relies on vscode.debug.startDebugging(); would need RPC mechanism to communicate with other debug adapters
- **Error handling & UX**: Modal error dialogs (vscode.window.showErrorMessage with modal:true) need equivalent in Rust/Tauri UI layer

### Feature Parity Checklist for Porting
- [ ] Pattern matching engine (regex compatibility)
- [ ] Terminal data streaming from running processes
- [ ] Process ID extraction and process lifecycle tracking
- [ ] Event-driven state machine for debug sessions
- [ ] DAP message protocol implementation/binding
- [ ] Browser debugger adapter launching (Chrome/Edge specific)
- [ ] Configuration schema validation for launch.json
- [ ] Localization system integration
- [ ] ANSI escape code filtering/stripping
- [ ] Cancellation token/timeout handling

---

## Summary

The **debug-server-ready extension** is a thin orchestration layer (393 LOC) that glues together three orthogonal systems:
1. **Debug session lifecycle** (vscode.debug API)
2. **Terminal output monitoring** (vscode.window.onDidWriteTerminalData)
3. **Browser/child debugger launching** (vscode.debug.startDebugging)

It uses pattern matching to detect when a server is ready and automatically triggers browser debugging or external URI opening. While the feature itself is relatively lightweight, **porting it to Tauri/Rust would require reimplementing several foundational layers**:
- The entire Debug Adapter Protocol message routing system
- Terminal I/O integration with process monitoring
- The event-driven async event emitter pattern
- Configuration schema parsing and validation
- Session lifecycle management with safe concurrent access

The extension serves as a **microcosm of VS Code's extensibility model** — it demonstrates how core IDE features (debugging) are exposed via public API contracts (`vscode.debug.onDidStartDebugSession`) that extensions hook into. Any Rust/Tauri port would need to maintain similar public contracts while reimplementing the underlying infrastructure.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Debug Server Ready Extension: Core Patterns

## Pattern Overview

The debug-server-ready extension implements a stateful pattern detection system for debug sessions, monitoring terminal output for server ready indicators and triggering associated actions. Six core patterns emerge from the codebase.

---

## Patterns

#### Pattern: Debug Session Lifecycle Registration

**Where:** `extensions/debug-server-ready/src/extension.ts:325-336`

**What:** Extension registers global handlers for debug session lifecycle events (start/terminate) to manage detector instances per session.

```typescript
export function activate(context: vscode.ExtensionContext) {
	context.subscriptions.push(vscode.debug.onDidStartDebugSession(session => {
		if (session.configuration.serverReadyAction) {
			const detector = ServerReadyDetector.start(session);
			if (detector) {
				ServerReadyDetector.startListeningTerminalData();
			}
		}
	}));

	context.subscriptions.push(vscode.debug.onDidTerminateDebugSession(session => {
		ServerReadyDetector.stop(session);
	}));
}
```

**Key aspects:**
- Uses `vscode.debug.onDidStartDebugSession` event for session initialization
- Uses `vscode.debug.onDidTerminateDebugSession` for cleanup
- Stores detectors in a static Map keyed by DebugSession
- Lazy-initializes terminal data listener on first use

---

#### Pattern: Configuration Provider for Dynamic Registration

**Where:** `extensions/debug-server-ready/src/extension.ts:340-350`

**What:** Debug configuration provider resolver tracks debug adapter types and registers trackers per type lazily.

```typescript
const trackers = new Set<string>();

context.subscriptions.push(vscode.debug.registerDebugConfigurationProvider('*', {
	resolveDebugConfigurationWithSubstitutedVariables(_folder: vscode.WorkspaceFolder | undefined, debugConfiguration: vscode.DebugConfiguration) {
		if (debugConfiguration.type && debugConfiguration.serverReadyAction) {
			if (!trackers.has(debugConfiguration.type)) {
				trackers.add(debugConfiguration.type);
				startTrackerForType(context, debugConfiguration.type);
			}
		}
		return debugConfiguration;
	}
}));
```

**Key aspects:**
- Registers provider for wildcard '*' type to intercept all debug configurations
- Maintains Set of already-registered types to avoid duplicate trackers
- Uses resolveDebugConfigurationWithSubstitutedVariables for post-variable-substitution access
- Delegates tracker creation to separate function

---

#### Pattern: Debug Adapter Tracker Factory with Message Inspection

**Where:** `extensions/debug-server-ready/src/extension.ts:353-393`

**What:** Creates debug adapter trackers that inspect protocol messages (output events and terminal requests) to detect server ready patterns and capture shell PIDs.

```typescript
function startTrackerForType(context: vscode.ExtensionContext, type: string) {
	context.subscriptions.push(vscode.debug.registerDebugAdapterTrackerFactory(type, {
		createDebugAdapterTracker(session: vscode.DebugSession) {
			const detector = ServerReadyDetector.start(session);
			if (detector) {
				let runInTerminalRequestSeq: number | undefined;
				return {
					onDidSendMessage: m => {
						if (m.type === 'event' && m.event === 'output' && m.body) {
							switch (m.body.category) {
								case 'console':
								case 'stderr':
								case 'stdout':
									if (m.body.output) {
										detector.detectPattern(m.body.output);
									}
									break;
								default:
									break;
							}
						}
						if (m.type === 'request' && m.command === 'runInTerminal' && m.arguments) {
							if (m.arguments.kind === 'integrated') {
								runInTerminalRequestSeq = m.seq;
							}
						}
					},
					onWillReceiveMessage: m => {
						if (runInTerminalRequestSeq && m.type === 'response' && m.command === 'runInTerminal' && m.body && runInTerminalRequestSeq === m.request_seq) {
							runInTerminalRequestSeq = undefined;
							ServerReadyDetector.rememberShellPid(session, m.body.shellProcessId);
						}
					}
				};
			}
			return undefined;
		}
	}));
}
```

**Key aspects:**
- Stateless message handler that tracks runInTerminal request sequence numbers
- Correlates requests with responses to extract shell process ID
- Inspects output events across console, stderr, stdout categories
- Returns undefined for sessions without serverReadyAction

---

#### Pattern: Terminal Data Event Listener with Fallback Detection

**Where:** `extensions/debug-server-ready/src/extension.ts:99-121`

**What:** Establishes single shared terminal data listener that matches detector by PID, then falls back to trying all detectors.

```typescript
static async startListeningTerminalData() {
	if (!this.terminalDataListener) {
		this.terminalDataListener = vscode.window.onDidWriteTerminalData(async e => {

			// first find the detector with a matching pid
			const pid = await e.terminal.processId;
			const str = removeAnsiEscapeCodes(e.data);
			for (const [, detector] of this.detectors) {
				if (detector.shellPid === pid) {
					detector.detectPattern(str);
					return;
				}
			}

			// if none found, try all detectors until one matches
			for (const [, detector] of this.detectors) {
				if (detector.detectPattern(str)) {
					return;
				}
			}
		});
	}
}
```

**Key aspects:**
- Singleton pattern: only one listener instance despite multiple calls
- Two-phase matching: optimize by PID first, then fallback to regex
- Strips ANSI escape codes before pattern matching
- Early returns to avoid processing by multiple detectors

---

#### Pattern: Async Parent Session Trigger Inheritance

**Where:** `extensions/debug-server-ready/src/extension.ts:123-134`

**What:** Detector constructor reuses parent session's trigger to prevent duplicate actions in compound debug configurations.

```typescript
private constructor(private session: vscode.DebugSession) {
	super(() => this.internalDispose());

	// Re-used the triggered of the parent session, if one exists
	if (session.parentSession) {
		this.trigger = ServerReadyDetector.start(session.parentSession)?.trigger ?? new Trigger();
	} else {
		this.trigger = new Trigger();
	}

	this.regexp = new RegExp(session.configuration.serverReadyAction.pattern || PATTERN, 'i');
}
```

**Key aspects:**
- Checks for parentSession to determine cascade behavior
- Initiates parent detector if not already started (lazy initialization)
- Falls back to new Trigger if parent has no detector
- Compiles pattern as instance field with case-insensitive flag

---

#### Pattern: Promise-Based Debug Session Catch with Cancellation

**Where:** `extensions/debug-server-ready/src/extension.ts:299-320`

**What:** Wraps debug session start events in Promise that resolves when predicate matches, with cancellation support.

```typescript
private catchStartedDebugSession(predicate: (session: vscode.DebugSession) => boolean, cancellationToken: vscode.CancellationToken): Promise<vscode.DebugSession | undefined> {
	return new Promise<vscode.DebugSession | undefined>(_resolve => {
		const done = (value?: vscode.DebugSession) => {
			listener.dispose();
			cancellationListener.dispose();
			this.disposables.delete(listener);
			this.disposables.delete(cancellationListener);
			_resolve(value);
		};

		const cancellationListener = cancellationToken.onCancellationRequested(done);
		const listener = vscode.debug.onDidStartDebugSession(session => {
			if (predicate(session)) {
				done(session);
			}
		});

		// In case the debug session of interest was never caught anyhow.
		this.disposables.add(listener);
		this.disposables.add(cancellationListener);
	});
}
```

**Key aspects:**
- Higher-order function that filters events by custom predicate
- Shared done callback for both event match and cancellation paths
- Cleanup occurs in done callback to avoid double disposal
- Disposables tracked in instance Set for lifecycle management

---

#### Pattern: Multi-Action URI Resolution with Format Validation

**Where:** `extensions/debug-server-ready/src/extension.ts:157-185`

**What:** Converts captured regex group into target URI using format string, with strict validation of format and fallback format selection.

```typescript
private openExternalWithString(session: vscode.DebugSession, captureString: string) {
	const args: ServerReadyAction = session.configuration.serverReadyAction;

	let uri;
	if (captureString === '') {
		// nothing captured by reg exp -> use the uriFormat as the target uri without substitution
		// verify that format does not contain '%s'
		const format = args.uriFormat || '';
		if (format.indexOf('%s') >= 0) {
			const errMsg = vscode.l10n.t("Format uri ('{0}') uses a substitution placeholder but pattern did not capture anything.", format);
			vscode.window.showErrorMessage(errMsg, { modal: true }).then(_ => undefined);
			return;
		}
		uri = format;
	} else {
		// if no uriFormat is specified guess the appropriate format based on the captureString
		const format = args.uriFormat || (/^[0-9]+$/.test(captureString) ? URI_PORT_FORMAT : URI_FORMAT);
		// verify that format only contains a single '%s'
		const s = format.split('%s');
		if (s.length !== 2) {
			const errMsg = vscode.l10n.t("Format uri ('{0}') must contain exactly one substitution placeholder.", format);
			vscode.window.showErrorMessage(errMsg, { modal: true }).then(_ => undefined);
			return;
		}
		uri = util.format(format, captureString);
	}

	this.openExternalWithUri(session, uri);
}
```

**Key aspects:**
- Branching logic: empty capture vs captured string paths
- Smart format defaults: detects numeric port vs full URI string
- Validates format contains exactly one '%s' placeholder
- Shows modal error dialogs with localized messages

---

## Summary

The debug-server-ready extension demonstrates several reusable patterns for VS Code debugging:

1. **Session lifecycle management** via global debug events and per-session detector instances
2. **Configuration interception** through wildcard provider registration with lazy tracker initialization
3. **Debug protocol inspection** using adapter trackers to monitor output events and terminal requests
4. **Unified terminal monitoring** with singleton listener and fallback detection strategy
5. **Trigger cascading** in compound configurations via parent session reference
6. **Promise-based event wrapping** with custom predicate filtering and cancellation support
7. **URI resolution** with format validation, fallback selection, and error handling

All patterns follow VS Code's disposable-based lifecycle pattern and use event-driven architecture with proper cleanup mechanics.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
