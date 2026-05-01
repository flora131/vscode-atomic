# Partition 45: extensions/debug-auto-launch — Node.js Auto-Attach Debug Configuration

## File Locations

### Implementation
- `extensions/debug-auto-launch/src/extension.ts` — Core auto-attach state machine (408 LOC)
  - Manages activation/deactivation lifecycle for Node.js debugger auto-attach
  - IPC server creation/destruction for debug session handoff
  - State machine with four modes: Disabled, OnlyWithFlag, Smart, Always
  - UI state bar integration and command registration

### Configuration Files
- `extensions/debug-auto-launch/package.json` — Extension manifest
  - Contributes single command: `extension.node-debug.toggleAutoAttach`
  - Activation event: `onStartupFinished`
  - Declares compatibility with VS Code >= 1.5.0
  - No explicit dependency declarations (lighter footprint)
- `extensions/debug-auto-launch/tsconfig.json` — TypeScript compilation config
  - Inherits from `../tsconfig.base.json`
  - Output directory: `./out/extension`
  - Type roots configured for node_modules/@types
- `extensions/debug-auto-launch/package.nls.json` — Localization strings
  - i18n entries: displayName, description, toggle.auto.attach label

### Build/Tooling
- `extensions/debug-auto-launch/esbuild.mts` — Build configuration
- `extensions/debug-auto-launch/.npmrc` — NPM configuration
- `extensions/debug-auto-launch/.vscodeignore` — Packaging rules
- `extensions/debug-auto-launch/.vscode/launch.json` — Debug launch configuration
- `extensions/debug-auto-launch/package-lock.json` — Dependency lock file

### Assets
- `extensions/debug-auto-launch/media/icon.png` — Extension icon

## Architecture Overview

### State Machine Design
The extension implements a four-state configuration system for Node.js auto-attach behavior:

1. **Disabled** — Auto-attach hidden from status bar; clears js-debug state
2. **OnlyWithFlag** — Attaches only when `--inspect` flag explicitly provided
3. **Smart** — Attaches to non-node_modules scripts only (intelligent filtering)
4. **Always** — Unconditionally attaches to all Node processes

State transitions are orchestrated through `updateAutoAttach()` with queued async operations. Each state transition maps to a state handler in the `transitions` object.

### IPC Communication Pattern
- **Server Creation**: Listens on platform-specific IPC address (Unix socket on Linux/macOS, named pipe on Windows)
- **Data Protocol**: Messages terminated with NUL byte (0x00), deserialized as JSON
- **Response Codes**: 0x00 (success), 0x01 (error)
- **Handoff Mechanism**: Receives process information, delegates to `extension.js-debug.autoAttachToProcess` command

### Configuration Scope Hierarchy
- **Settings Section**: `debug.javascript`
- **Primary Setting**: `debug.javascript.autoAttachFilter` — Stores state (Disabled|OnlyWithFlag|Smart|Always)
- **Dependent Setting**: `debug.javascript.autoAttachSmartPattern` — Regex pattern for Smart mode filtering
- **Scope Resolution**: WorkspaceFolder > Workspace > Global (hierarchical precedence)

### Extension Context Integration
- **Workspace State Storage**: Caches IPC address and js-debug path metadata via `context.workspaceState`
- **Temp Disable State**: In-memory flag `isTemporarilyDisabled` for session-scoped toggles (does not persist)
- **Status Bar Item**: Lifecycle-managed UI element with loading spinner animation during state transitions

## Porting Considerations for Tauri/Rust

### 1. IPC/RPC Layer (Critical)
**Current Implementation**: Native Node.js `net` module for socket listening
- Creates platform-specific listener (Unix socket `/tmp/vscode-*` or Windows named pipe)
- Handles concurrent socket connections with individual data buffers
- NUL-byte message framing

**Tauri Port Requirements**:
- Replace with Tauri's built-in command/event system or custom protocol handler
- Maintain backward compatibility with js-debug extension's IPC expectations
- Handle message serialization (JSON) at Tauri boundary
- Ensure file descriptor cleanup on socket errors (already handled via `fs.unlink()`)

### 2. Configuration/Settings Storage
**Current Implementation**: VS Code Settings API (`vscode.workspace.getConfiguration()`)
- Multi-level scope hierarchy (workspace folder, workspace, global)
- Change listeners with debounce-able refresh logic
- Computed setting validation keys

**Tauri Port Requirements**:
- Abstract settings layer required (not directly provided by Tauri)
- Must interface with VS Code's settings model through language server or extension API
- Preserve hierarchical scope resolution during migration
- Maintain workspace state storage for IPC cache invalidation

### 3. UI Integration Points
**Current Implementation**: 
- Status bar item with dynamic text and loading spinner
- Quick pick menu (4 state options + temp disable toggle + scope switcher button)
- Command palette command registration

**Tauri Port Requirements**:
- WebView-based UI for menu system (vs. native VS Code UI primitives)
- Status bar updates via event bridge to core VS Code UI
- Command dispatch through Tauri invoke() pattern
- Consider native context menu vs. quick pick compatibility layer

### 4. Process Model & Lifecycle
**Current Implementation**:
- Singleton server instance per extension activation
- Graceful server shutdown on `deactivate()`
- Promise-based state sequencing with `currentState` queue

**Tauri Port Requirements**:
- Rust async runtime (tokio) for socket listener and command dispatch
- Clean separation between extension host process and IPC server lifecycle
- Error handling parity for stale IPC files on macOS/Linux (temp dir changes)

### 5. Extension API Dependencies
**Current Implementation**: Minimal core dependencies
- `extension.js-debug.setAutoAttachVariables` — Obtains IPC address
- `extension.js-debug.autoAttachToProcess` — Delegates debug attachment
- `extension.js-debug.clearAutoAttachVariables` — State cleanup

**Tauri Port Requirements**:
- Must maintain protocol compatibility with js-debug extension communication
- May require FFI calls to VS Code core for settings/state access
- Consider whether Tauri extension model supports cross-extension command dispatch

### 6. File System Operations
**Current Implementation**:
- `fs.promises` for async file access (socket cleanup via `unlink()`)
- Directory validation for IPC path existence (macOS temp dir resilience)

**Tauri Port Requirements**:
- Use `std::fs` or `tokio::fs` for async I/O
- Handle platform-specific file paths (Windows UNC paths, Unix sockets)
- Maintain error recovery for stale socket files

### 7. Localization & String Management
**Current Implementation**: `vscode.l10n.t()` with package.nls.json key mapping
- Dynamic status bar text with locale-aware labels
- 4 state descriptions + UI labels + tooltips (16 strings total)

**Tauri Port Requirements**:
- Replace with Tauri's i18n solution or maintained compatibility layer
- Preserve language setting awareness from VS Code context
- Consider bundling nls files with Tauri app distribution

## Code Statistics
- **Single TypeScript File**: 408 lines of implementation
- **Configuration Files**: 5 (package.json, tsconfig.json, package.nls.json, esbuild.mts, .npmrc)
- **Total LOC**: ~425 (including config)
- **Complexity**: Medium (state machine, async promise sequencing, multi-scope settings)
- **Dependencies**: None (uses VS Code API and Node.js built-ins)

## Summary

The debug-auto-launch extension is a lightweight (408 LOC) Node.js debugger auto-attachment facilitator that bridges VS Code's settings system with the js-debug extension through an IPC server. For Tauri porting, the critical challenge is replicating the IPC communication layer (currently via Node.js `net` module) while maintaining protocol compatibility with js-debug. Configuration management requires abstracting VS Code's multi-level scope system, and UI integration demands WebView-based alternatives to native status bar and quick pick widgets. The extension's promise-based state machine can be straightforwardly ported to Rust/Tokio async patterns, but the extension API dependency surface (cross-extension commands) will require careful protocol specification and potential FFI integration for accessing VS Code's settings storage layer.
