# File Locations: debug-auto-launch Extension

## Implementation

### Core Extension Logic
- `/home/norinlavaee/projects/vscode-atomic/extensions/debug-auto-launch/src/extension.ts` (407 LOC)
  - Main extension activation and deactivation
  - Auto-attach state machine with 4 states: Disabled, OnlyWithFlag, Smart, Always
  - IPC server creation and lifecycle management for Node.js process attachment
  - Configuration change listeners and status bar management
  - vscode.commands API consumers: `extension.js-debug.setAutoAttachVariables`, `extension.js-debug.autoAttachToProcess`, `extension.js-debug.clearAutoAttachVariables`
  - Socket-based communication for passing process attachment data from native Node.js processes

## Configuration & Metadata

### Package Configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/debug-auto-launch/package.json` (48 lines)
  - Extension name: `debug-auto-launch` v10.0.0
  - Single command contribution: `extension.node-debug.toggleAutoAttach`
  - Activation event: `onStartupFinished`
  - Main entry point: `./out/extension`
  - Capabilities: untrusted workspace support, no virtual workspace support
  - Dependencies: @types/node 22.x

### Localization Strings
- `/home/norinlavaee/projects/vscode-atomic/extensions/debug-auto-launch/package.nls.json` (4 strings)
  - Display name, description, and toggle command labels for i18n

### TypeScript Configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/debug-auto-launch/tsconfig.json`
  - Extends base configuration from `../tsconfig.base.json`
  - Output directory: `./out`
  - Type roots: node_modules/@types
  - Includes vscode.d.ts type definitions

## Build & Development

### Build Configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/debug-auto-launch/esbuild.mts` (18 lines)
  - ESBuild configuration using common configuration from `../esbuild-extension-common.mts`
  - Platform: node
  - Entry point: `src/extension.ts` → `dist/extension`

### Development Configuration
- `/home/norinlavaee/projects/vscode-atomic/extensions/debug-auto-launch/.vscode/launch.json`
  - Extension Host debug configuration for development

### Build Scripts
- `/home/norinlavaee/projects/vscode-atomic/extensions/debug-auto-launch/package.json` (scripts section)
  - `compile`: gulp compile-extension:debug-auto-launch
  - `watch`: gulp watch-extension:debug-auto-launch

## Asset & Metadata Files

### Visual Assets
- `/home/norinlavaee/projects/vscode-atomic/extensions/debug-auto-launch/media/icon.png`
  - Extension icon displayed in marketplace

### Additional Metadata
- `/home/norinlavaee/projects/vscode-atomic/extensions/debug-auto-launch/.npmrc`
- `/home/norinlavaee/projects/vscode-atomic/extensions/debug-auto-launch/.vscodeignore`
- `/home/norinlavaee/projects/vscode-atomic/extensions/debug-auto-launch/package-lock.json`

## Notable Features & Dependencies

### Debugging Architecture
This extension implements a minimal debug UI layer that manages auto-attachment to Node.js processes. It:
- Uses an IPC server (net.createServer) listening on platform-specific addresses for process attachment signals
- Communicates with the `js-debug` extension via command API for actual debugging setup
- Manages workspace/global configuration for attachment behavior
- Provides status bar UI for toggling auto-attach modes

### Debug-related vscode API Calls
1. `vscode.commands.executeCommand('extension.js-debug.setAutoAttachVariables')` - Initialize debug environment
2. `vscode.commands.executeCommand('extension.js-debug.autoAttachToProcess')` - Attach debugger to process
3. `vscode.commands.executeCommand('extension.js-debug.clearAutoAttachVariables')` - Clean up debug state
4. `vscode.window.createStatusBarItem()` - UI for debug state visibility
5. `vscode.workspace.onDidChangeConfiguration()` - React to debug settings changes

### Configuration Targets
- Setting section: `debug.javascript`
- Key setting: `autoAttachFilter` with states: disabled, onlyWithFlag, smart, always
- Related setting: `autoAttachSmartPattern` - controls intelligent attachment filtering

## Summary

The `debug-auto-launch` extension (2 files, 425 LOC) is a lightweight debug auxiliary that enables automatic attachment to Node.js debugging sessions without requiring the full js-debug extension to be active. It abstracts the plumbing of process attachment via an IPC server and state machine, delegating actual debug session management to the `ms-vscode.js-debug` extension. The extension consumes three primary debug commands and manages workspace-level configuration. This design allows VS Code to offer auto-attach capabilities as a core feature while keeping the heavyweight debugging machinery optional and extensible.

Key porting considerations:
- IPC server architecture using Node.js net module would need equivalent in Rust/Tauri
- vscode.commands.executeCommand dependency on ms-vscode.js-debug extension requires coordinating with larger debug infrastructure
- Status bar integration and configuration change listeners are OS-agnostic UI patterns
- Socket protocol for process attachment data could be reimplemented in Rust with similar semantics
