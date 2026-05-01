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
