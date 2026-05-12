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
