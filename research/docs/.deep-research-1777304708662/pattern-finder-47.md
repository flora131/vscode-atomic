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

