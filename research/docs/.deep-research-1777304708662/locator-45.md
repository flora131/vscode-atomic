# File Locations: extensions/debug-auto-launch

## Implementation

- `extensions/debug-auto-launch/src/extension.ts` (407 LOC) — Core auto-attach debug server implementation. Manages the extension lifecycle, state machine for auto-attach modes (Disabled, OnlyWithFlag, Smart, Always), status bar UI, command registration, and IPC socket server for communicating with the js-debug extension. Primary debug API touchpoints include:
  - `vscode.commands.executeCommand('extension.js-debug.clearAutoAttachVariables')` (line 200)
  - `vscode.commands.executeCommand('extension.js-debug.autoAttachToProcess', ...)` (line 263)
  - `vscode.commands.executeCommand('extension.js-debug.setAutoAttachVariables', ...)` (line 382)
  - `vscode.window.createStatusBarItem()` (line 325)
  - `vscode.commands.registerCommand()` (line 67)
  - `vscode.workspace.onDidChangeConfiguration()` (line 71)

## Configuration

- `extensions/debug-auto-launch/package.json` — Extension manifest declaring the extension as `debug-auto-launch`, version 10.0.0. Declares single contributed command `extension.node-debug.toggleAutoAttach` in Debug category. Activation event: `onStartupFinished`. Main entry: `./out/extension`.

- `extensions/debug-auto-launch/tsconfig.json` — TypeScript compilation config extending base tsconfig, targeting Node 22.x types. Output directory: `./out`, with vscode type definitions included from repository root.

- `extensions/debug-auto-launch/.vscode/launch.json` — Debug launch configuration for extensionHost type debugging, pointing to built output at `${workspaceFolder}/out/**/*.js`.

- `extensions/debug-auto-launch/.npmrc` — NPM configuration file (not read).

- `extensions/debug-auto-launch/.vscodeignore` — VS Code extension packaging ignore rules (not read).

- `extensions/debug-auto-launch/package-lock.json` — Lockfile for npm dependencies.

## Types/Interfaces

No dedicated type definition files. Type information is defined inline in `extension.ts`:
- `State` enum (lines 11-16): Disabled, OnlyWithFlag, Smart, Always
- `CachedIpcState` interface (lines 287-291): Holds cached IPC server address, js-debug path, and settings hash
- `PickResult` type union (line 109): Discriminated union for quick-pick result handling
- `PickItem` type (line 110): QuickPickItem extending with state or temp-disabled discriminants

## Tests

No test files found. This extension has no unit or integration tests in the repository.

## Examples/Fixtures

- `extensions/debug-auto-launch/media/icon.png` — Extension icon asset (referenced in package.json at line 11).

## Documentation

- `extensions/debug-auto-launch/package.nls.json` — Localization strings file (not read, but referenced in package.json for i18n keys like `%displayName%`, `%description%`, `%toggle.auto.attach%`, etc.).

## Notable Clusters

The extension is minimal—a single TypeScript source file (407 LOC) with no test or additional implementation files. The debug-auto-launch extension acts as a bridge between the VS Code IDE and the `ms-vscode.js-debug` extension, providing:

1. **State Management**: Tracks and persists auto-attach modes via VS Code workspace configuration (`debug.javascript.autoAttachFilter`).
2. **Status Bar UI**: Displays current auto-attach state and allows toggling via QuickPick.
3. **IPC Communication**: Creates a Unix socket server (`createServer` from Node.js `net` module, line 250) that receives process attach requests from Node.js runtime and forwards them to js-debug via `extension.js-debug.autoAttachToProcess` command.
4. **Settings Synchronization**: Caches js-debug extension path and settings keys to detect invalidation (lines 362–397).

The architecture is event-driven: activation via `onStartupFinished`, configuration changes trigger refresh cycles, and the state machine (line 297) ensures all state transitions (Disabled → OnlyWithFlag/Smart/Always) execute their corresponding server creation/destruction logic.

## Summary

The `extensions/debug-auto-launch/` directory contains a lightweight, single-file extension (407 LOC) that manages Node.js auto-attach debugging in VS Code. Its primary role is lifecycle and IPC server management for the `ms-vscode.js-debug` extension, exposing a toggle command and status bar indicator. The extension demonstrates typical VS Code extension patterns: command registration, workspace configuration management, activation events, and inter-extension communication via the command API. No tests or standalone type definitions are present; types are colocated inline. The extension requires TypeScript compilation and depends on Node.js types and the vscode API definitions included from the repository root.

