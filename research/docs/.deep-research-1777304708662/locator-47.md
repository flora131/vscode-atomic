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
