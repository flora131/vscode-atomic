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
