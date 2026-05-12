# Partition 47 of 80 — Findings

## Scope
`extensions/debug-server-ready/` (2 files, 411 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Debug Server Ready Extension - File Locations

## Implementation
- `extensions/debug-server-ready/src/extension.ts` — Core extension logic implementing ServerReadyDetector class that hooks into vscode.debug APIs (`onDidStartDebugSession`, `onDidTerminateDebugSession`) to pattern-match server startup messages and trigger browser debugging, open external URLs, or start child debug sessions

## Configuration
- `extensions/debug-server-ready/package.json` — Extension manifest declaring debugger configuration schema for "serverReadyAction" properties (action, pattern, uriFormat, webRoot, killOnServerStop, config, name) across all debugger types; activation on "onDebugResolve" events; depends on terminalDataWriteEvent API proposal
- `extensions/debug-server-ready/tsconfig.json` — TypeScript configuration extending base config, compiles src/ to out/, includes vscode type definitions and terminalDataWriteEvent proposal types
- `extensions/debug-server-ready/.vscodeignore` — Build artifact exclusion config
- `extensions/debug-server-ready/.npmrc` — NPM configuration

## Examples / Fixtures
- `extensions/debug-server-ready/.vscode/launch.json` — Development configuration for testing the extension via extensionHost

## Notable Clusters
- `extensions/debug-server-ready/` — Contains 11 files total (src extension logic, build scripts, type definitions, media assets); esbuild-based bundling (esbuild.mts), npm lock file

## Summary
The debug-server-ready extension is a TypeScript-based VS Code extension that integrates with the debugging subsystem to automatically detect when servers are ready (via pattern matching on debug output and terminal data) and trigger configurable actions: opening URLs externally, launching browser debugging sessions (Chrome/Edge), or starting secondary debug configurations. It relies heavily on vscode.debug API events (`onDidStartDebugSession`, `onDidTerminateDebugSession`, `onDidWriteTerminalData`) and the debug adapter tracker factory pattern. Key to porting would be replicating these event-driven debugging hooks and the regex-based pattern detection logic in a Rust/Tauri backend while maintaining the same configuration schema for launch.json integration.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Debug Server Ready Extension - Pattern Analysis for Tauri/Rust Port

## Research Scope
Partition 47 of 80: `extensions/debug-server-ready/` (2 files, 411 LOC)

## Core Finding
The debug-server-ready extension demonstrates **essential debug lifecycle patterns** that a Tauri/Rust port must replicate. These patterns involve event emission, session tracking, pattern matching, and cross-session lifecycle management.

---

## Pattern 1: Debug Session Lifecycle Events
**Where:** `extensions/debug-server-ready/src/extension.ts:325-336`
**What:** Subscribe to debug session start/stop events to manage detector instances per session.

```typescript
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
```

**What makes this essential for Tauri port:**
- The core must emit `onDidStartDebugSession` and `onDidTerminateDebugSession` events
- These are foundational Extension API contracts that plugins depend on
- Session lifecycle is used for resource cleanup (detector disposal)
- Non-negotiable: other extensions will hook these same events

---

## Pattern 2: Session-Keyed Resource Management with Static Map
**Where:** `extensions/debug-server-ready/src/extension.ts:59-90`
**What:** Use static Map to track detector instances per debug session with lazy creation.

```typescript
class ServerReadyDetector extends vscode.Disposable {
	private static detectors = new Map<vscode.DebugSession, ServerReadyDetector>();

	static start(session: vscode.DebugSession): ServerReadyDetector | undefined {
		if (session.configuration.serverReadyAction) {
			let detector = ServerReadyDetector.detectors.get(session);
			if (!detector) {
				detector = new ServerReadyDetector(session);
				ServerReadyDetector.detectors.set(session, detector);
			}
			return detector;
		}
		return undefined;
	}

	static stop(session: vscode.DebugSession): void {
		const detector = ServerReadyDetector.detectors.get(session);
		if (detector) {
			ServerReadyDetector.detectors.delete(session);
			detector.sessionStopped();
			detector.dispose();
		}
	}
}
```

**Why this matters for Tauri port:**
- Session objects must be hashable/comparable (keyed in Maps)
- Extensions expect to track per-session state
- Cleanup must be triggered via terminate event
- The pattern is: lazy-init on start, cleanup on terminate

---

## Pattern 3: Debug Adapter Tracker Factory with Message Inspection
**Where:** `extensions/debug-server-ready/src/extension.ts:353-393`
**What:** Register a DebugAdapterTrackerFactory to inspect DAP messages (output, runInTerminal).

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
							}
						}
						if (m.type === 'request' && m.command === 'runInTerminal') {
							if (m.arguments.kind === 'integrated') {
								runInTerminalRequestSeq = m.seq;
							}
						}
					},
					onWillReceiveMessage: m => {
						if (runInTerminalRequestSeq && m.type === 'response' && 
						    m.command === 'runInTerminal' && m.body && 
						    runInTerminalRequestSeq === m.request_seq) {
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

**Core requirement for Tauri port:**
- The debug adapter protocol (DAP) message flow must be interceptable via tracker factories
- Output events must carry stdout/stderr/console categories
- Request/response correlation via `seq`/`request_seq` is essential
- Extensions monitor process spawning (`runInTerminal`) to extract PID

---

## Pattern 4: Terminal Data Listening with Process ID Matching
**Where:** `extensions/debug-server-ready/src/extension.ts:99-121`
**What:** Listen to terminal write events and match output to detectors by shell PID.

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

**What Tauri core must support:**
- `vscode.window.onDidWriteTerminalData` event (proposal: terminalDataWriteEvent)
- Terminal object must expose `processId` property (async)
- Data arrives as strings with ANSI escape codes intact
- ANSI code stripping is critical (Pattern 5)

---

## Pattern 5: ANSI Escape Code Stripping Utility
**Where:** `extensions/debug-server-ready/src/extension.ts:26-45`
**What:** Strip ANSI control sequences before pattern matching (CSI, OSC, ESC).

```typescript
const CSI_SEQUENCE = /(?:\x1b\[|\x9b)[=?>!]?[\d;:]*["$#'* ]?[a-zA-Z@^`{}|~]/;
const OSC_SEQUENCE = /(?:\x1b\]|\x9d).*?(?:\x1b\\|\x07|\x9c)/;
const ESC_SEQUENCE = /\x1b(?:[ #%\(\)\*\+\-\.\/]?[a-zA-Z0-9\|}~@])/;
const CONTROL_SEQUENCES = new RegExp('(?:' + [
	CSI_SEQUENCE.source,
	OSC_SEQUENCE.source,
	ESC_SEQUENCE.source,
].join('|') + ')', 'g');

function removeAnsiEscapeCodes(str: string): string {
	if (str) {
		str = str.replace(CONTROL_SEQUENCES, '');
	}
	return str;
}
```

**Integration requirement:**
- Terminal output includes ANSI codes; patterns won't match without stripping
- This utility is in core (`src/vs/base/common/strings.ts`)
- Extensions will use this to pre-process terminal data before regex matching

---

## Pattern 6: EventEmitter for Custom Session Events
**Where:** `extensions/debug-server-ready/src/extension.ts:64-65, 141-143`
**What:** Create internal EventEmitter to signal when a debug session stops.

```typescript
private readonly stoppedEmitter = new vscode.EventEmitter<void>();
private readonly onDidSessionStop = this.stoppedEmitter.event;

public sessionStopped() {
	this.stoppedEmitter.fire();
}
```

**Usage pattern:**
- Extensions create internal event emitters using `new vscode.EventEmitter<T>()`
- Fire events with `.fire(value)`
- Subscribe via `.event` property
- Used in Pattern 7 to coordinate child session cleanup

---

## Pattern 7: Parent-Child Debug Session Tracking
**Where:** `extensions/debug-server-ready/src/extension.ts:127-131`
**What:** Reuse parent session's trigger to prevent duplicate actions on nested sessions.

```typescript
if (session.parentSession) {
	this.trigger = ServerReadyDetector.start(session.parentSession)?.trigger ?? new Trigger();
} else {
	this.trigger = new Trigger();
}
```

**Structural requirement for Tauri port:**
- `DebugSession` objects must have optional `parentSession` property
- This enables hierarchical debug session relationships
- Extensions use this to coordinate behavior across session trees

---

## Pattern 8: Debug Configuration Provider for Type Registration
**Where:** `extensions/debug-server-ready/src/extension.ts:340-350`
**What:** Use configuration provider to track which debugger types have been initialized.

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

**Core interaction point:**
- Extensions call `registerDebugConfigurationProvider('*', provider)` for all types
- `resolveDebugConfigurationWithSubstitutedVariables` is called before debugging starts
- The `DebugConfiguration` object is passed through (contains user launch.json config)
- This is where extensions inject behavior into the debug flow

---

## Pattern 9: Regex Pattern Matching on Debug Output
**Where:** `extensions/debug-server-ready/src/extension.ts:10, 133, 145-155`
**What:** Define regex patterns to detect server readiness from output, then extract capture groups.

```typescript
const PATTERN = 'listening on.* (https?://\\S+|[0-9]+)';
const URI_PORT_FORMAT = 'http://localhost:%s';

// In constructor:
this.regexp = new RegExp(session.configuration.serverReadyAction.pattern || PATTERN, 'i');

// In detection:
detectPattern(s: string): boolean {
	if (!this.trigger.hasFired) {
		const matches = this.regexp.exec(s);
		if (matches && matches.length >= 1) {
			this.openExternalWithString(this.session, matches.length > 1 ? matches[1] : '');
			this.trigger.fire();
			return true;
		}
	}
	return false;
}
```

**Why this matters:**
- Extensions read `serverReadyAction.pattern` from debug configuration
- Pattern is case-insensitive regex
- Extracted capture groups drive downstream actions
- Configuration schema defined in `package.json` contributes to debuggers

---

## Pattern 10: Cascading Actions (open, debug, launch new session)
**Where:** `extensions/debug-server-ready/src/extension.ts:187-215`
**What:** Dispatch different actions based on serverReadyAction.action field.

```typescript
private async openExternalWithUri(session: vscode.DebugSession, uri: string) {
	const args: ServerReadyAction = session.configuration.serverReadyAction;
	switch (args.action || 'openExternally') {
		case 'openExternally':
			await vscode.env.openExternal(vscode.Uri.parse(uri));
			break;
		case 'debugWithChrome':
			await this.debugWithBrowser('pwa-chrome', session, uri);
			break;
		case 'debugWithEdge':
			await this.debugWithBrowser('pwa-msedge', session, uri);
			break;
		case 'startDebugging':
			if (args.config) {
				await this.startDebugSession(session, args.config.name, args.config);
			} else {
				await this.startDebugSession(session, args.name || 'unspecified');
			}
			break;
	}
}
```

**Extension API surface required:**
- `vscode.env.openExternal(uri)` - open external browser/app
- `vscode.debug.startDebugging(folder, config)` - start child debug session
- `vscode.debug.stopDebugging(session)` - stop specific session

---

## Configuration Schema Integration
**Where:** `extensions/debug-server-ready/package.json:29-212`
**What:** Extensions contribute debugger configuration schemas via package.json.

Example (simplified):
```json
{
  "contributes": {
    "debuggers": [{
      "type": "*",
      "configurationAttributes": {
        "launch": {
          "properties": {
            "serverReadyAction": {
              "pattern": "string",
              "action": "enum: ['openExternally', 'debugWithChrome', 'debugWithEdge', 'startDebugging']",
              "uriFormat": "string",
              "webRoot": "string",
              "killOnServerStop": "boolean"
            }
          }
        }
      }
    }]
  }
}
```

**Tauri core requirement:**
- Core must read and merge extension contribution schemas
- Configuration validation must respect contributed properties
- launch.json IntelliSense must include these properties

---

## Summary: Core IDE Functionality Required for Tauri Port

### Event Emission (Critical)
1. `vscode.debug.onDidStartDebugSession` - fired when debug session begins
2. `vscode.debug.onDidTerminateDebugSession` - fired when session ends
3. `vscode.window.onDidWriteTerminalData` - fired on terminal output

### Debug Session API Requirements
- `DebugSession` object with: `id`, `name`, `type`, `configuration`, `parentSession`, `workspaceFolder`
- `DebugConfiguration` object with: `type`, `name`, user-defined properties
- `vscode.debug.activeDebugSession` property
- `vscode.debug.startDebugging(folder, config)` function
- `vscode.debug.stopDebugging(session)` function

### Debug Adapter Protocol (DAP) Support
- Debug adapter tracker factories: `registerDebugAdapterTrackerFactory(type, factory)`
- Message inspection: `onDidSendMessage`, `onWillReceiveMessage` hooks
- Message format includes: `type`, `event`, `command`, `seq`, `request_seq`, `body`

### Terminal Support
- Terminal objects with async `processId` property
- Terminal output capture with ANSI codes included
- `vscode.env.openExternal(uri)` for external app launching

### Configuration & Contribution System
- Extension contribution schemas (`contributes.debuggers`)
- Debug configuration provider registration: `registerDebugConfigurationProvider(type, provider)`
- Configuration resolution: `resolveDebugConfigurationWithSubstitutedVariables`

### Core Utilities
- `vscode.EventEmitter<T>` for custom events
- `vscode.Disposable` lifecycle management
- `vscode.Uri` parsing and handling
- Regex matching and string manipulation

---

## Cross-File Summary
- **extension.ts (394 lines)**: All extension logic; single-file design allows simple testing
- **package.json (222 lines)**: Schema contributions, activation events, API proposals needed
- **Key insight**: This is a relatively simple extension but exercises _fundamental_ debug APIs that many other extensions depend on

The debug-server-ready extension is a canonical example of how the extension host must surface debug lifecycle and session management to enable higher-level IDE behaviors.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
