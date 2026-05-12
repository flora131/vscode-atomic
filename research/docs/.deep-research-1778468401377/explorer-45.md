# Partition 45 of 80 — Findings

## Scope
`extensions/debug-auto-launch/` (2 files, 425 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
### Files Analysed

- `extensions/debug-auto-launch/src/extension.ts` (407 LOC)
- `extensions/debug-auto-launch/package.json` (48 LOC)

---

### Per-File Notes

#### `extensions/debug-auto-launch/src/extension.ts`

- **Role:** Implements the Node.js debug auto-attach feature for VS Code. Manages a four-state machine (`Disabled`, `OnlyWithFlag`, `Smart`, `Always`) that controls whether and how VS Code automatically attaches a debugger to Node.js processes launched in integrated terminals. It operates as a thin orchestration layer: it owns the IPC server lifecycle and status bar UI, while delegating all actual debug attachment logic to the `ms-vscode.js-debug` extension via VS Code commands.

- **Key symbols:**
  - `State` (const enum, line 11–16): The four string-valued states — `Disabled = 'disabled'`, `OnlyWithFlag = 'onlyWithFlag'`, `Smart = 'smart'`, `Always = 'always'`.
  - `TOGGLE_COMMAND` (line 44): `'extension.node-debug.toggleAutoAttach'` — the single command contributed by this extension.
  - `STORAGE_IPC` (line 45): `'jsDebugIpcState'` — workspace state key for caching IPC address between sessions.
  - `SETTING_SECTION` / `SETTING_STATE` (lines 47–48): Configuration path `debug.javascript.autoAttachFilter`.
  - `SETTINGS_CAUSE_REFRESH` (lines 53–55): Set of settings (`debug.javascript.autoAttachSmartPattern`, `debug.javascript.autoAttachFilter`) whose changes trigger a state refresh.
  - `currentState` (line 58): Module-level `Promise` chain that serializes all state transitions.
  - `statusItem` (line 59): Module-level `vscode.StatusBarItem | undefined`.
  - `server` (line 60): Module-level `Promise<Server | undefined> | undefined` holding the active IPC server.
  - `isTemporarilyDisabled` (line 61): Module-level boolean; set to `false` on every state transition (line 353).
  - `activate()` (line 63): Extension entry point.
  - `deactivate()` (line 86): Extension teardown.
  - `updateAutoAttach()` (line 341): Core state-machine driver; enqueues transitions onto `currentState` promise chain.
  - `transitions` (lines 297–313): Map from each `State` to its async handler function.
  - `createAttachServer()` (line 209): Starts the IPC server.
  - `createServerInner()` (line 237): Retry wrapper around `createServerInstance`.
  - `createServerInstance()` (line 248): Builds the `net.Server` and wires socket data handling.
  - `destroyAttachServer()` (line 280): Closes the active IPC server.
  - `getIpcAddress()` (line 362): Retrieves or refreshes the IPC socket path via js-debug command, caching in workspace state.
  - `getJsDebugSettingKey()` (line 399): Serializes the current relevant settings to a JSON string for cache invalidation.
  - `clearJsDebugAttachState()` (line 197): Tears down IPC server and clears workspace state and js-debug environment variables.
  - `toggleAutoAttachSetting()` (line 112): Displays a `QuickPick` to change state or toggle temporary disable; invoked by `TOGGLE_COMMAND`.
  - `updateStatusBar()` (line 318): Creates or updates the left-aligned status bar item.
  - `readCurrentState()` (line 192): Reads `debug.javascript.autoAttachFilter` from workspace configuration.
  - `CachedIpcState` (interface, lines 287–291): Shape of the workspace-state cache: `{ ipcAddress, jsDebugPath, settingsValue }`.

- **Control flow:**

  1. **Activation** (`activate()`, lines 63–84):
     - Initializes `currentState` as a resolved promise with `state: null`.
     - Registers `TOGGLE_COMMAND` → `toggleAutoAttachSetting`.
     - Subscribes to `onDidChangeConfiguration`; if any `SETTINGS_CAUSE_REFRESH` setting changes, calls `refreshAutoAttachVars()` (line 71–81).
     - Calls `updateAutoAttach(readCurrentState())` to enter the initial state (line 83).

  2. **State machine (`updateAutoAttach`, lines 341–356)**:
     - Appends a `.then()` to the `currentState` promise chain (line 342).
     - No-ops if `newState === oldState` (line 343–344).
     - Calls `updateStatusBar(..., busy=true)` while transitioning (line 348–350).
     - Invokes `transitions[newState](context)` which is one of:
       - `Disabled`: calls `clearJsDebugAttachState()` (line 299).
       - `OnlyWithFlag`, `Smart`, `Always`: all call `createAttachServer()` (lines 303, 307, 311).
     - Resets `isTemporarilyDisabled = false` (line 353).
     - Calls `updateStatusBar(..., busy=false)` (line 354).

  3. **IPC server lifecycle**:
     - `createAttachServer()` (lines 209–235): Calls `getIpcAddress()`, then assigns `server = createServerInner(ipcAddress).catch(...)`. On non-Windows, if the parent directory of the IPC address is inaccessible, triggers `refreshAutoAttachVars()` (lines 223–227).
     - `createServerInner()` (lines 237–246): Calls `createServerInstance()`; if it throws (e.g., stale socket file on Unix), unlinks the socket file and retries once.
     - `createServerInstance()` (lines 248–275): Creates a `net.Server` via `createServer()`. On each socket connection, accumulates incoming `Buffer` chunks until a NUL byte (`0x00`) terminator is seen (line 253). On termination, concatenates buffers, parses JSON payload, executes `extension.js-debug.autoAttachToProcess` with the parsed data (lines 261–266), writes `0x00` on success (line 266) or `0x01` on failure (line 268). Server resolves the outer `Promise<Server>` on `listen` (line 274).
     - `destroyAttachServer()` (lines 280–285): Awaits `server`, then calls `instance.close()` wrapped in a promise.

  4. **Toggle command (`toggleAutoAttachSetting`, lines 112–190)**:
     - Determines scope (global vs workspace) via `getDefaultScope()` (line 114).
     - Builds a `QuickPick<PickItem>` with four state options (lines 120–125).
     - Prepends a temporary-disable/re-enable item when state is not `Disabled` (lines 127–133).
     - Buttons on the QuickPick toggle the scope between global and workspace (lines 141–146, 153–159).
     - On accept: if `'scope' in result`, recursively calls itself with the new scope (line 169). If `'state' in result` and different from current, calls `section.update()` (line 174) — this triggers the config change listener and `refreshAutoAttachVars()`. If `'setTempDisabled' in result`, calls `destroyAttachServer()` or `createAttachServer()` directly without a config change (lines 183–188).

  5. **IPC address management (`getIpcAddress`, lines 362–397)**:
     - Reads `CachedIpcState` from `context.workspaceState` (line 366).
     - Detects the installed js-debug extension path (`ms-vscode.js-debug-nightly` first, then `ms-vscode.js-debug`) (lines 372–374).
     - Cache hit: returns `cachedIpc.ipcAddress` if both `jsDebugPath` and `settingsValue` match (lines 377–379).
     - Cache miss: executes `extension.js-debug.setAutoAttachVariables` with the old IPC address (lines 381–384), stores the returned new `ipcAddress` plus `jsDebugPath` and `settingsValue` to workspace state (lines 390–394), and returns the new address.

  6. **Deactivation** (`deactivate()`, lines 86–88): Calls `destroyAttachServer()`.

- **Data flow:**

  ```
  VS Code Config (debug.javascript.autoAttachFilter)
      │
      ▼
  readCurrentState() → State enum value
      │
      ▼
  updateAutoAttach(newState)
      │
      ├─ [Disabled] → clearJsDebugAttachState()
      │       ├─ workspaceState.update(STORAGE_IPC, undefined)
      │       ├─ executeCommand('extension.js-debug.clearAutoAttachVariables')
      │       └─ destroyAttachServer()
      │
      └─ [OnlyWithFlag | Smart | Always] → createAttachServer()
              │
              ▼
          getIpcAddress(context)
              ├─ workspaceState.get(STORAGE_IPC) → CachedIpcState?
              ├─ extensions.getExtension('ms-vscode.js-debug') → jsDebugPath
              ├─ [cache miss] executeCommand('extension.js-debug.setAutoAttachVariables', oldAddr)
              │       → { ipcAddress: string }
              ├─ workspaceState.update(STORAGE_IPC, { ipcAddress, jsDebugPath, settingsValue })
              └─ return ipcAddress
              │
              ▼
          createServerInner(ipcAddress)
              │
              ▼
          net.createServer (UNIX socket / named pipe)
              │
              ▼
          [Node.js process connects, sends NUL-terminated JSON payload]
              │
              ▼
          JSON.parse(Buffer.concat(data).toString())
              │
              ▼
          executeCommand('extension.js-debug.autoAttachToProcess', parsedPayload)
              │
              ▼
          socket.write([0]) on success  /  socket.write([1]) on failure
  ```

- **Dependencies:**
  - `fs` (node built-in, `promises` variant) — used at line 6 to `fs.access()` directory check (line 224) and `fs.unlink()` stale socket cleanup (line 243).
  - `net` (node built-in) — `createServer`, `Server` imported at line 7; used to build the IPC socket server (line 250).
  - `path` (node built-in) — `dirname` imported at line 8; used to check IPC address parent directory (line 224).
  - `vscode` API — imported at line 9; used for:
    - `vscode.commands.registerCommand`, `executeCommand` — command registration and cross-extension IPC.
    - `vscode.workspace.getConfiguration`, `onDidChangeConfiguration` — config reading and change listening.
    - `vscode.window.createStatusBarItem`, `createQuickPick` — UI surfaces.
    - `vscode.extensions.getExtension` — detecting js-debug extension path.
    - `vscode.ExtensionContext.workspaceState` — workspace-scoped persistent storage.
    - `vscode.l10n.t` — localization.
    - `vscode.ThemeIcon`, `vscode.StatusBarAlignment`, `vscode.ConfigurationTarget` — UI constants.

---

#### `extensions/debug-auto-launch/package.json`

- **Role:** Declares the extension's metadata, activation strategy, and its single user-facing command contribution. Acts as the manifest that VS Code reads to know when to activate the extension and what commands it exposes in the command palette.

- **Key symbols:**
  - `activationEvents: ["onStartupFinished"]` (line 18–20): Extension is activated after VS Code finishes its startup sequence, not eagerly. This means the auto-attach state machine initializes only after the IDE is fully ready.
  - `contributes.commands[0]` (lines 27–33): Registers `extension.node-debug.toggleAutoAttach` with title `%toggle.auto.attach%` (localized) under the `"Debug"` category. This is the only command surfaced to users.
  - `capabilities.virtualWorkspaces: false` (line 13): Extension explicitly opts out of virtual workspace support (e.g., GitHub Codespaces browsing mode without a local file system).
  - `capabilities.untrustedWorkspaces.supported: true` (lines 14–16): Extension runs in workspace trust restricted mode.
  - `main: "./out/extension"` (line 21): Entry point after compilation points to the compiled output of `src/extension.ts`.
  - `devDependencies: { "@types/node": "22.x" }` (line 35–37): Only dev dependency is Node.js type definitions; no runtime npm dependencies. All runtime dependencies are either Node built-ins or the `vscode` API injected at runtime.

- **Control flow:** No runtime control flow — purely declarative manifest. VS Code reads this JSON at load time to schedule activation and wire command palette entries.

- **Data flow:** The `command` string `extension.node-debug.toggleAutoAttach` declared here (line 29) corresponds exactly to `TOGGLE_COMMAND` constant in `extension.ts:44`, which is registered in `activate()` at line 67.

- **Dependencies:** None at runtime. Build tooling via `gulp` scripts (lines 22–25).

---

### Cross-Cutting Synthesis

The debug-auto-launch extension is a minimal orchestration shim: it owns the state machine, the IPC socket server, the status bar widget, and the workspace settings integration, but it contains zero actual debug protocol logic. All debugger-side work is delegated to `ms-vscode.js-debug` via three `vscode.commands.executeCommand` calls: `setAutoAttachVariables` (which injects environment variables into new terminal processes so those processes know to connect to the IPC socket), `autoAttachToProcess` (called for each connecting Node.js process), and `clearAutoAttachVariables` (teardown). The IPC socket is a Unix domain socket (or Windows named pipe) whose address is negotiated with js-debug at server creation and cached in `workspaceState`. The state machine is fully serialized through a promise chain (`currentState`), ensuring that concurrent setting changes or command invocations are processed in order without races. The temporary-disable feature bypasses the config system entirely — it directly starts/stops the IPC server without writing any settings — and is reset to `false` on every full state transition. The cache invalidation in `getIpcAddress` uses two signals: the js-debug extension's install path (to detect upgrades) and a JSON snapshot of the relevant settings, ensuring that any change to `autoAttachSmartPattern` or `autoAttachFilter` forces a fresh negotiation with js-debug.

For a Tauri/Rust port, this component maps to: a persistent background task managing a Unix socket (tokio's `UnixListener`), a state enum with four variants driving transitions, IPC message passing to whatever replaces js-debug, and a frontend status indicator. The `workspaceState` cache becomes a file-backed or SQLite-backed key-value store per workspace. The `vscode.commands.executeCommand` cross-extension calls become inter-process or intra-process message-passing to the js-debug Rust/WASM equivalent.

---

### Out-of-Partition References

- **`ms-vscode.js-debug` extension** — The authoritative debugger extension. This partition calls three of its commands:
  - `extension.js-debug.setAutoAttachVariables(oldIpcAddress?)` → returns `{ ipcAddress: string }`: negotiates the IPC socket path and injects the `NODE_OPTIONS` / `VSCODE_INSPECTOR_OPTIONS` environment variable into new terminal shells so spawned Node.js processes connect back to the IPC server.
  - `extension.js-debug.autoAttachToProcess(payload)` → called with the JSON payload sent by a connecting Node.js process over the IPC socket; triggers actual debug session attachment.
  - `extension.js-debug.clearAutoAttachVariables` → removes the injected environment variables from new terminals and cleans up js-debug's internal state.
- **`ms-vscode.js-debug-nightly`** — Nightly channel of js-debug; checked first at `extension.ts:372` for path-based cache invalidation.
- **`vscode` extension host API** — The entire runtime surface (`commands`, `workspace`, `window`, `extensions`, `l10n`, `ExtensionContext`) is provided by the VS Code extension host process; not available in a Tauri context without reimplementation.
- **Node.js terminal integration** — The mechanism by which js-debug injects `NODE_OPTIONS` into terminal environments is not in this partition. The IPC socket address string returned by `setAutoAttachVariables` and the NUL-terminated JSON framing protocol used over the socket are the two interface contracts between this extension and the js-debug extension's terminal integration.

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Debug Auto-Launch Extension: VS Code API Patterns (Partition 45)

## Scope
`extensions/debug-auto-launch/` (425 LOC, 2 files)

---

## Pattern Examples: VS Code API Consumer Implementation

### Pattern 1: Extension Activation and Command Registration
**Found in**: `extensions/debug-auto-launch/src/extension.ts:63-83`
**Used for**: Extension lifecycle and command framework hookup

```typescript
export function activate(context: vscode.ExtensionContext): void {
	currentState = Promise.resolve({ context, state: null });

	context.subscriptions.push(
		vscode.commands.registerCommand(TOGGLE_COMMAND, toggleAutoAttachSetting.bind(null, context)),
	);

	context.subscriptions.push(
		vscode.workspace.onDidChangeConfiguration(e => {
			if (
				e.affectsConfiguration(`${SETTING_SECTION}.${SETTING_STATE}`) ||
				[...SETTINGS_CAUSE_REFRESH].some(setting => e.affectsConfiguration(setting))
			) {
				refreshAutoAttachVars();
			}
		}),
	);

	updateAutoAttach(readCurrentState());
}
```

**Key aspects**:
- Extension context holds subscription lifetime
- Commands registered with context binding for state access
- Configuration change listeners for responsive updates
- Subscriptions pushed to context array for cleanup

### Pattern 2: Command Dispatch to Inter-Extension Communication
**Found in**: `extensions/debug-auto-launch/src/extension.ts:200, 262, 381`
**Used for**: Invoking debugger extension functionality via command protocol

```typescript
// Pattern 1: Clear state command (line 200)
await vscode.commands.executeCommand('extension.js-debug.clearAutoAttachVariables');

// Pattern 2: Process attachment with typed data (line 262-265)
await vscode.commands.executeCommand(
	'extension.js-debug.autoAttachToProcess',
	JSON.parse(Buffer.concat(data).toString()),
);

// Pattern 3: Synchronous variable setup with typed response (line 381-384)
const result = await vscode.commands.executeCommand<{ ipcAddress: string }>(
	'extension.js-debug.setAutoAttachVariables',
	cachedIpc?.ipcAddress,
);
```

**Key aspects**:
- Commands target extension-namespaced endpoints (e.g., `extension.js-debug.*`)
- Generic type parameter for typed responses
- No parameters when only triggering side effects
- JSON serialization for complex data transmission
- Optional previous state passed for cache invalidation

### Pattern 3: Configuration Scoping Strategy
**Found in**: `extensions/debug-auto-launch/src/extension.ts:95-107`
**Used for**: Determining workspace vs. global configuration targets

```typescript
function getDefaultScope(info: ReturnType<vscode.WorkspaceConfiguration['inspect']>) {
	if (!info) {
		return vscode.ConfigurationTarget.Global;
	} else if (info.workspaceFolderValue) {
		return vscode.ConfigurationTarget.WorkspaceFolder;
	} else if (info.workspaceValue) {
		return vscode.ConfigurationTarget.Workspace;
	} else if (info.globalValue) {
		return vscode.ConfigurationTarget.Global;
	}

	return vscode.ConfigurationTarget.Global;
}
```

**Key aspects**:
- Uses `inspect()` to check configuration presence across scopes
- Hierarchical evaluation: WorkspaceFolder → Workspace → Global
- Returns explicit ConfigurationTarget enum values
- Defaults to Global when no value exists

### Pattern 4: Status Bar Item Lifecycle and Updates
**Found in**: `extensions/debug-auto-launch/src/extension.ts:318-336`
**Used for**: Transient UI state reflection during operations

```typescript
function updateStatusBar(context: vscode.ExtensionContext, state: State, busy = false) {
	if (state === State.Disabled && !busy) {
		statusItem?.hide();
		return;
	}

	if (!statusItem) {
		statusItem = vscode.window.createStatusBarItem('status.debug.autoAttach', vscode.StatusBarAlignment.Left);
		statusItem.name = vscode.l10n.t("Debug Auto Attach");
		statusItem.command = TOGGLE_COMMAND;
		statusItem.tooltip = vscode.l10n.t("Automatically attach to node.js processes in debug mode");
		context.subscriptions.push(statusItem);
	}

	let text = busy ? '$(loading) ' : '';
	text += isTemporarilyDisabled ? TEXT_TEMP_DISABLE_LABEL : TEXT_STATUSBAR_LABEL[state];
	statusItem.text = text;
	statusItem.show();
}
```

**Key aspects**:
- Lazy creation of status bar item on first use
- Status bar ID for persistence across reloads
- Subscription registration for automatic cleanup
- Conditional show/hide instead of creation/destruction
- Icon loading indicator `$(loading)` for busy states
- Command binding for interactive toggling

### Pattern 5: QuickPick with Contextual Scope Switching
**Found in**: `extensions/debug-auto-launch/src/extension.ts:112-190`
**Used for**: Multi-level selection with mode toggling

```typescript
const quickPick = vscode.window.createQuickPick<PickItem>();
const current = readCurrentState();

const items: PickItem[] = [State.Always, State.Smart, State.OnlyWithFlag, State.Disabled].map(state => ({
	state,
	label: TEXT_STATE_LABEL[state],
	description: TEXT_STATE_DESCRIPTION[state],
	alwaysShow: true,
}));

if (current !== State.Disabled) {
	items.unshift({
		setTempDisabled: !isTemporarilyDisabled,
		label: isTemporarilyDisabled ? TEXT_TEMP_ENABLE : TEXT_TEMP_DISABLE,
		alwaysShow: true,
	});
}

quickPick.items = items;
quickPick.activeItems = isTemporarilyDisabled
	? [items[0]]
	: quickPick.items.filter(i => 'state' in i && i.state === current);
quickPick.title = TEXT_TOGGLE_TITLE;
quickPick.placeholder = isGlobalScope ? TEXT_TOGGLE_GLOBAL : TEXT_TOGGLE_WORKSPACE;
quickPick.buttons = [
	{
		iconPath: new vscode.ThemeIcon(isGlobalScope ? 'folder' : 'globe'),
		tooltip: isGlobalScope ? TEXT_TOGGLE_WORKSPACE : TEXT_TOGGLE_GLOBAL,
	},
];

quickPick.show();

let result = await new Promise<PickResult>(resolve => {
	quickPick.onDidAccept(() => resolve(quickPick.selectedItems[0]));
	quickPick.onDidHide(() => resolve(undefined));
	quickPick.onDidTriggerButton(() => {
		resolve({
			scope: isGlobalScope
				? vscode.ConfigurationTarget.Workspace
				: vscode.ConfigurationTarget.Global,
		});
	});
});
```

**Key aspects**:
- Discriminated union types for pick items (`state` vs `setTempDisabled`)
- Dynamic item insertion based on current state
- Button toggling between scope levels (Global ↔ Workspace)
- Promise wrapper for modal dialog pattern
- Icon theming for visual scope indication

### Pattern 6: Promise-Based State Sequencing
**Found in**: `extensions/debug-auto-launch/src/extension.ts:341-356`
**Used for**: Ensuring serial execution of state transitions

```typescript
function updateAutoAttach(newState: State) {
	currentState = currentState.then(async ({ context, state: oldState }) => {
		if (newState === oldState) {
			return { context, state: oldState };
		}

		if (oldState !== null) {
			updateStatusBar(context, oldState, true);
		}

		await transitions[newState](context);
		isTemporarilyDisabled = false;
		updateStatusBar(context, newState, false);
		return { context, state: newState };
	});
}
```

**Key aspects**:
- Maintains `currentState` as a promise chain
- Guarantees serialized state transitions
- Comparison prevents redundant operations
- Visual feedback via `updateStatusBar(busy=true)`
- Resets temp-disabled flag on state change
- Returns context for chain continuation

### Pattern 7: Network Server Lifecycle Management
**Found in**: `extensions/debug-auto-launch/src/extension.ts:209-275`
**Used for**: IPC socket server for debug attachment protocol

```typescript
async function createAttachServer(context: vscode.ExtensionContext) {
	const ipcAddress = await getIpcAddress(context);
	if (!ipcAddress) {
		return undefined;
	}

	server = createServerInner(ipcAddress).catch(async err => {
		console.error('[debug-auto-launch] Error creating auto attach server: ', err);

		if (process.platform !== 'win32') {
			try {
				await fs.access(dirname(ipcAddress));
			} catch {
				console.error('[debug-auto-launch] Refreshing variables from error');
				refreshAutoAttachVars();
				return undefined;
			}
		}

		return undefined;
	});

	return await server;
}

const createServerInner = async (ipcAddress: string) => {
	try {
		return await createServerInstance(ipcAddress);
	} catch (e) {
		await fs.unlink(ipcAddress).catch(() => undefined);
		return await createServerInstance(ipcAddress);
	}
};

const createServerInstance = (ipcAddress: string) =>
	new Promise<Server>((resolve, reject) => {
		const s = createServer(socket => {
			const data: Buffer[] = [];
			socket.on('data', async chunk => {
				if (chunk[chunk.length - 1] !== 0) {
					data.push(chunk);
					return;
				}

				data.push(chunk.slice(0, -1));

				try {
					await vscode.commands.executeCommand(
						'extension.js-debug.autoAttachToProcess',
						JSON.parse(Buffer.concat(data).toString()),
					);
					socket.write(Buffer.from([0]));
				} catch (err) {
					socket.write(Buffer.from([1]));
					console.error(err);
				}
			});
		})
			.on('error', reject)
			.listen(ipcAddress, () => resolve(s));
	});

async function destroyAttachServer() {
	const instance = await server;
	if (instance) {
		await new Promise(r => instance.close(r));
	}
}
```

**Key aspects**:
- Platform-specific retry logic (Unix/Linux socket leak cleanup)
- NUL-byte framing for JSON protocol messages
- Immediate acknowledgment pattern (0 = success, 1 = error)
- Chunked buffer reassembly for protocol parsing
- Error recovery with automatic path deletion
- Promise wrapping of Node.js event emitter patterns
- Graceful shutdown with close callback

---

## Related Patterns in Scope

### Extension Context Subscription Management
The extension consistently uses `context.subscriptions.push()` for:
- Command registrations
- Event listeners (onDidChangeConfiguration)
- Status bar items
- Ensures automatic cleanup on deactivation

### Localization Pattern
All user-visible strings use `vscode.l10n.t()`:
- Status bar labels
- QuickPick titles and descriptions
- Tooltips
- Enables string extraction for translations

### Configuration Access Pattern
```typescript
const section = vscode.workspace.getConfiguration(SETTING_SECTION);
const value = section.get<State>(SETTING_STATE) ?? State.Disabled;
section.update(SETTING_STATE, newValue, scope);
```
- Namespace-based configuration groups
- Type-safe reads with fallback defaults
- Scope-aware updates (Global, Workspace, WorkspaceFolder)

---

## Summary

The `debug-auto-launch` extension demonstrates core VS Code extension patterns for managing debugger lifecycle, inter-extension communication, and user configuration. Key architectural patterns include:

1. **Extension activation** establishes command registry and configuration listeners bound to context lifecycle
2. **Inter-extension commands** use namespaced command IDs with typed response generics for js-debug integration
3. **Configuration hierarchy** respects scope precedence (WorkspaceFolder > Workspace > Global) via inspection
4. **Status bar UI** uses lazy initialization with subscription cleanup and dynamic state reflection
5. **QuickPick dialogs** employ discriminated unions for polymorphic item types and button-based navigation
6. **State transitions** serialize via promise chaining to prevent concurrent modifications
7. **Network protocols** use NUL-byte framing for JSON over Unix sockets with error-response codes

For Tauri/Rust porting, these patterns require:
- Command dispatch abstraction (vs. vscode.commands.executeCommand)
- IPC protocol implementation with proper framing and error handling
- Configuration storage layer with scope semantics
- UI component lifecycle management and theming integration
- Localization infrastructure for user-facing strings
- Promise-like concurrency control (async/await equivalent)

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
