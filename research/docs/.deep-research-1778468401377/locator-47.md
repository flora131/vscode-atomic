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
