# Partition 47 of 79 — Findings

## Scope
`extensions/debug-server-ready/` (2 files, 411 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
## File Locations for debug-server-ready Extension

### Implementation
- `extensions/debug-server-ready/src/extension.ts` — Main extension implementing ServerReadyDetector class with auto-attach patterns, debug session lifecycle management, and debug adapter tracking for detecting "server ready" output

### Configuration
- `extensions/debug-server-ready/package.json` — Extension manifest with serverReadyAction contributions supporting four action modes (openExternally, debugWithChrome, debugWithEdge, startDebugging) and pattern matching configuration
- `extensions/debug-server-ready/package.nls.json` — Localized strings for UI labels, descriptions, and schema properties
- `extensions/debug-server-ready/tsconfig.json` — TypeScript compilation config extending base tsconfig with terminalDataWriteEvent proposal types
- `extensions/debug-server-ready/esbuild.mts` — Build configuration for esbuild-extension-common

## Summary

The debug-server-ready extension (699 LOC total) implements automatic debugging activation when a server signals readiness via terminal output patterns. The core module `extension.ts` (394 lines) defines ServerReadyDetector which listens to debug adapter output and terminal events, matches configurable regex patterns (default: "listening on.* (https?://\\S+|[0-9]+)"), and triggers subsequent debug sessions via `vscode.debug.startDebugging()` calls. The extension supports four action modes: external URI launching, Chrome/Edge browser debugging, or starting arbitrary debug configurations. A key architectural element is the DebugAdapterTrackerFactory registration that intercepts debug protocol messages and runInTerminal requests to obtain shell process IDs for accurate pattern matching against terminal data. The configuration schema in package.json supports pattern-driven URI construction with format substitution (%s placeholders) and lifecycle control (killOnServerStop flag) for child session management.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Debug Server-Ready Extension: Core Debug API Patterns

## Overview
The `debug-server-ready` extension demonstrates fundamental debug API touchpoints that a Rust/Tauri host must implement to support VS Code's debugging ecosystem. It intercepts debug session lifecycle, monitors debug adapter output, and automatically launches additional debug sessions based on server readiness patterns.

---

## Patterns

#### Pattern: Debug Session Lifecycle Management
**Where:** `extensions/debug-server-ready/src/extension.ts:325-336`
**What:** Register global listeners for debug session creation and termination to manage per-session state.
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
**Variations / call-sites:** Event-driven pattern used throughout debug API tests (`extensions/vscode-api-tests/src/singlefolder-tests/debug.test.ts:75-79`). Tracks session state in static Map and correlates child sessions with parent sessions via predicate matching.

---

#### Pattern: Debug Adapter Tracker Factory Registration
**Where:** `extensions/debug-server-ready/src/extension.ts:353-393`
**What:** Register a tracker factory to intercept and inspect debug adapter protocol messages flowing between IDE and debugger.
```typescript
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
```
**Variations / call-sites:** Similar tracker pattern used in test suites (`extensions/vscode-api-tests/src/singlefolder-tests/debug.test.ts:84-101`) to monitor 'stopped' events and protocol responses.

---

#### Pattern: Debug Configuration Provider for Interception
**Where:** `extensions/debug-server-ready/src/extension.ts:340-350`
**What:** Implement a configuration provider registered for all debugger types to intercept and modify launch configurations at resolution time.
```typescript
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
**Variations / call-sites:** Configuration provider hook executed per debug session launch. Enables lazy initialization of per-type tracker factories.

---

#### Pattern: Pattern Matching on Debug Output
**Where:** `extensions/debug-server-ready/src/extension.ts:145-155`
**What:** Use regex pattern matching against debug adapter output to detect server readiness events and trigger actions.
```typescript
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
**Variations / call-sites:** Patterns invoked from two sources: (1) Debug adapter output events (`extension.ts:369`), (2) Terminal data stream events (`extension.ts:108, 115`).

---

#### Pattern: Nested Debug Session Lifecycle with Parent Coupling
**Where:** `extensions/debug-server-ready/src/extension.ts:250-259, 268-297`
**What:** Start child debug sessions (browser or custom config) coupled to parent session termination via event listeners.
```typescript
private startBrowserDebugSession(type: string, session: vscode.DebugSession, uri: string, trackerId?: string) {
	return vscode.debug.startDebugging(session.workspaceFolder, {
		type,
		name: 'Browser Debug',
		request: 'launch',
		url: uri,
		webRoot: session.configuration.serverReadyAction.webRoot || WEB_ROOT,
		_debugServerReadySessionId: trackerId,
	});
}

private async startDebugSession(session: vscode.DebugSession, name: string, config?: vscode.DebugConfiguration) {
	// ... killOnServerStop logic ...
	if (!await vscode.debug.startDebugging(session.workspaceFolder, config ?? name)) {
		// ... error handling ...
		return;
	}

	const createdSession = await newSessionPromise;
	// ... couple child lifecycle to parent ...
	const stopListener = this.onDidSessionStop(async () => {
		stopListener.dispose();
		this.disposables.delete(stopListener);
		await vscode.debug.stopDebugging(createdSession);
	});
	this.disposables.add(stopListener);
}
```
**Variations / call-sites:** `catchStartedDebugSession()` pattern uses predicate matching (`extension.ts:299-320`) to filter sessions by name or custom marker ID before coupling lifecycle.

---

#### Pattern: Terminal Data Stream Monitoring with Process ID Correlation
**Where:** `extensions/debug-server-ready/src/extension.ts:99-121`
**What:** Listen to raw terminal output stream and correlate with debug session process IDs to capture server readiness messages.
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
**Variations / call-sites:** Process ID obtained from debug adapter runInTerminal response tracking (`extension.ts:382-386`). Fallback to pattern matching all detectors when PID correlation fails.

---

#### Pattern: ANSI Escape Code Stripping for Clean Pattern Matching
**Where:** `extensions/debug-server-ready/src/extension.ts:26-45`
**What:** Remove terminal control sequences before regex matching to ensure patterns match semantic content, not formatting.
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
**Variations / call-sites:** Applied before terminal data processing (`extension.ts:105`) and implicitly via debug adapter protocol (which doesn't include ANSI codes in structured output events).

---

#### Pattern: Stateful Trigger Management to Prevent Duplicate Actions
**Where:** `extensions/debug-server-ready/src/extension.ts:47-57, 126-134`
**What:** Use a single-fire trigger per session (or shared with parent) to ensure the server-ready action fires only once.
```typescript
class Trigger {
	private _fired = false;

	public get hasFired() {
		return this._fired;
	}

	public fire() {
		this._fired = true;
	}
}

// In ServerReadyDetector constructor:
if (session.parentSession) {
	this.trigger = ServerReadyDetector.start(session.parentSession)?.trigger ?? new Trigger();
} else {
	this.trigger = new Trigger();
}
```
**Variations / call-sites:** Trigger state checked in `detectPattern()` (`extension.ts:146`) before any action execution. Parent session sharing prevents redundant actions on child sessions.

---

## Core API Requirements for Rust Host

A Rust/Tauri-based VS Code host must implement:

1. **Debug Session Event Model**: `onDidStartDebugSession`, `onDidTerminateDebugSession` events with session context (type, configuration, workspaceFolder).

2. **Debug Adapter Tracker Factory**: Ability to register factories that intercept bidirectional debug adapter protocol (DAP) messages with hooks for `onDidSendMessage` and `onWillReceiveMessage`.

3. **Debug Configuration Provider**: Hook to intercept and modify debug configurations before launch, callable per debug type.

4. **startDebugging / stopDebugging APIs**: Primitives to programmatically launch and terminate debug sessions by name or configuration object.

5. **Terminal Data Stream API**: Access to raw terminal output (`onDidWriteTerminalData`) with process ID correlation, supporting UA proposal `terminalDataWriteEvent`.

6. **Disposable & EventEmitter**: Subscription management and event emission primitives for extension-level event handling.

7. **Configuration Object Access**: Deep access to `vscode.DebugConfiguration` properties including custom extension fields like `serverReadyAction`, `webRoot`, `config`, `name`.

The extension demonstrates three debug API touchpoints: (1) debug protocol interception for message monitoring, (2) terminal stream integration for shell output capture, (3) nested session management with lifecycle coupling.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
