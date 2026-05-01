### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/debug-auto-launch/src/extension.ts` (408 LOC)
- `/home/norinlavaee/projects/vscode-atomic/extensions/debug-auto-launch/package.json` (48 LOC)

---

### Per-File Notes

#### `extensions/debug-auto-launch/package.json`

**Manifest role**

- Extension ID: `vscode.debug-auto-launch`, version `10.0.0`.
- `activationEvents`: `["onStartupFinished"]` (line 18–20) — the extension activates once after all other startup work is complete, not on-demand.
- Contributes a single command: `extension.node-debug.toggleAutoAttach` (line 29), titled `"Toggle Auto Attach"` in the `Debug` category.
- `main`: `"./out/extension"` (line 21) — compiled output of `extension.ts`.
- Declares `"virtualWorkspaces": false` (line 14), meaning the extension does not run in virtual (e.g. remote-repository) workspaces.
- `"untrustedWorkspaces": { "supported": true }` (line 15–17) — the extension runs even in restricted-trust workspace mode.
- Only dev-dependency is `@types/node 22.x`; no runtime npm dependencies outside of vscode API.

---

#### `extensions/debug-auto-launch/src/extension.ts`

**Module-level imports and constants**

- Imports `fs.promises` (line 6), `net.createServer` / `net.Server` (line 7), `path.dirname` (line 8), and the `vscode` namespace (line 9).
- Defines a `const enum State` (lines 11–16) with four string-valued members: `Disabled = 'disabled'`, `OnlyWithFlag = 'onlyWithFlag'`, `Smart = 'smart'`, `Always = 'always'`.
- Three lookup tables keyed on `State` provide localized strings for the status bar (lines 17–22), quick-pick labels (lines 24–29), and quick-pick descriptions (lines 30–35).
- Four additional localized string constants cover the toggle UI: `TEXT_TOGGLE_TITLE`, `TEXT_TOGGLE_WORKSPACE`, `TEXT_TOGGLE_GLOBAL`, `TEXT_TEMP_DISABLE`, `TEXT_TEMP_ENABLE`, `TEXT_TEMP_DISABLE_LABEL` (lines 37–42).
- Key constant identifiers:
  - `TOGGLE_COMMAND = 'extension.node-debug.toggleAutoAttach'` (line 44) — the command contributed by `package.json`.
  - `STORAGE_IPC = 'jsDebugIpcState'` (line 45) — workspace-state key used to persist the IPC address.
  - `SETTING_SECTION = 'debug.javascript'` (line 47), `SETTING_STATE = 'autoAttachFilter'` (line 48) — the VS Code settings path polled and written by this extension.
  - `SETTINGS_CAUSE_REFRESH` (lines 53–55) — a `Set<string>` containing `debug.javascript.autoAttachSmartPattern` and `debug.javascript.autoAttachFilter`; changes to either trigger a state refresh.

**Module-level mutable state**

Four module-level variables carry live state across the extension's lifetime:

- `currentState: Promise<{ context, state }>` (line 58) — a promise chain that serialises all state transitions. Each call to `updateAutoAttach()` appends a `.then()` to this chain so transitions never overlap.
- `statusItem: vscode.StatusBarItem | undefined` (line 59) — lazily created; `undefined` until the first non-disabled state.
- `server: Promise<Server | undefined> | undefined` (line 60) — the live IPC server, stored as a promise to allow awaiting creation.
- `isTemporarilyDisabled: boolean` (line 61) — a session-only flag; set by the user via the quick pick; reset to `false` by every call to `updateAutoAttach()`.

**`activate()` (lines 63–84)**

Entry point called by VS Code when the extension activates. Three things happen:

1. `currentState` is initialised to a resolved promise carrying `{ context, state: null }` (line 64).
2. `TOGGLE_COMMAND` is registered; its handler is `toggleAutoAttachSetting` (line 67).
3. A `onDidChangeConfiguration` listener is subscribed (lines 70–81). Whenever a changed key is in `SETTINGS_CAUSE_REFRESH` or is `debug.javascript.autoAttachFilter`, `refreshAutoAttachVars()` is called.
4. `updateAutoAttach(readCurrentState())` (line 83) performs the initial state transition based on whatever setting value is persisted.

**`deactivate()` (lines 86–88)**

Exported async function called by VS Code on deactivation. Calls `destroyAttachServer()` to close the IPC socket.

**`refreshAutoAttachVars()` (lines 90–93)**

Sequences two consecutive `updateAutoAttach` calls: first forces a transition to `Disabled` (which tears down the server and clears env-var state in js-debug), then immediately transitions to the currently configured state. This pattern ensures that the environment variables injected into new terminal sessions are regenerated with fresh settings.

**`getDefaultScope()` (lines 95–107)**

Examines the `inspect()` result of the `autoAttachFilter` configuration entry to determine which `vscode.ConfigurationTarget` (WorkspaceFolder > Workspace > Global) holds the effective value. Returns `Global` when no value has been explicitly set at any scope.

**`toggleAutoAttachSetting()` (lines 112–190)**

Async function invoked when the user triggers `TOGGLE_COMMAND`. Behaviour:

1. Reads the current configuration scope via `getDefaultScope()` if none is provided (line 114).
2. Constructs a `vscode.QuickPick<PickItem>` listing all four states (lines 120–125). If the current state is not `Disabled`, prepends a "Temporarily disable / Re-enable" item (lines 127–133).
3. Adds a toggle-button (`ThemeIcon` of `'folder'` or `'globe'`, lines 141–146) that allows switching between workspace-scope and global-scope edits. When activated, the function recurses with the alternate scope (lines 153–159, 168–169).
4. On item acceptance (line 151): if a `state` result is chosen that differs from the current, writes `section.update(SETTING_STATE, result.state, scope)` (line 174) — VS Code fires `onDidChangeConfiguration`, which then triggers `refreshAutoAttachVars()`. If the same state as the current is chosen while temporarily disabled, converts the result to `setTempDisabled: false` (lines 175–177).
5. If `setTempDisabled` is in the result (lines 180–189): calls `updateStatusBar` in busy mode, sets `isTemporarilyDisabled`, then either destroys or recreates the server.

**`readCurrentState()` (lines 192–195)**

Reads `debug.javascript.autoAttachFilter` via `vscode.workspace.getConfiguration`. Returns `State.Disabled` if the setting is absent or `undefined`.

**`clearJsDebugAttachState()` (lines 197–203)**

If an IPC server is running or `STORAGE_IPC` is set in workspace state:
1. Clears `STORAGE_IPC` in workspace state.
2. Executes `extension.js-debug.clearAutoAttachVariables` — tells the js-debug extension to remove the `NODE_OPTIONS` / `VSCODE_INSPECTOR_OPTIONS` variables from newly spawned terminal environments.
3. Calls `destroyAttachServer()`.

**`createAttachServer()` (lines 209–235)**

Starts the IPC listener. Calls `getIpcAddress(context)` to get the socket path; if none is returned, exits early. Assigns to module-level `server` the result of `createServerInner(ipcAddress)` wrapped in `.catch()`. The catch handler: on non-Windows platforms, attempts `fs.access(dirname(ipcAddress))` to determine whether the temp directory moved; if so, calls `refreshAutoAttachVars()`.

**`createServerInner()` (lines 237–246)**

Helper that tries `createServerInstance()` once; if it throws (typically `EADDRINUSE` from a leaked socket file), unlinks the file with `fs.unlink()` and retries once.

**`createServerInstance()` (lines 248–275)**

Creates the actual `net.Server`. The connection handler accumulates incoming `Buffer` chunks in a `data: Buffer[]` array. Framing protocol: messages are terminated with a NUL byte (`0x00`). When the final chunk's last byte is `0`, the accumulated buffers (minus the NUL) are concatenated, decoded as UTF-8, and `JSON.parse`d. The parsed object is forwarded to `extension.js-debug.autoAttachToProcess` via `vscode.commands.executeCommand` (line 263). On success, the socket receives a single `0x00` byte; on error, it receives `0x01` (lines 265–268). The server listens on `ipcAddress` (a Unix socket path or Windows named pipe) and resolves the outer promise when the `'listening'` event fires (line 274).

**`destroyAttachServer()` (lines 280–285)**

Awaits the `server` promise, then closes the `net.Server` instance via the callback-based `instance.close()` wrapped in a new `Promise`.

**`CachedIpcState` interface (lines 287–291)**

Describes the object stored under `STORAGE_IPC` in workspace state:
- `ipcAddress: string` — the socket/pipe path.
- `jsDebugPath: string | undefined` — path to the active js-debug extension, used as a cache-invalidation key.
- `settingsValue: string` — JSON snapshot of the settings in `SETTINGS_CAUSE_REFRESH`, also used for invalidation.

**`transitions` map (lines 297–313)**

A `{ [S in State]: (context) => Promise<void> }` object implementing the per-state entry actions:
- `Disabled` → `clearJsDebugAttachState()`.
- `OnlyWithFlag`, `Smart`, `Always` → `createAttachServer()`.

There are no explicit exit actions; the `Disabled` transition always performs cleanup unconditionally, and `refreshAutoAttachVars()` ensures `Disabled` is visited before re-entering an active state.

**`updateStatusBar()` (lines 318–336)**

When `state === Disabled` and not `busy`: hides the status bar item (or does nothing if it was never created). Otherwise: lazily creates `statusItem` at alignment `Left`, with ID `'status.debug.autoAttach'`, command `TOGGLE_COMMAND`, and tooltip text (lines 325–329). The displayed text is `$(loading)` prefix when `busy`, then either `TEXT_TEMP_DISABLE_LABEL` (if temporarily disabled) or the state-mapped label from `TEXT_STATUSBAR_LABEL`. The item is then shown.

**`updateAutoAttach()` (lines 341–356)**

Appends to the `currentState` promise chain. When the promise resolves:
1. Returns early if `newState === oldState`.
2. If `oldState !== null`, shows the status bar in busy/loading state.
3. Calls `transitions[newState](context)`.
4. Resets `isTemporarilyDisabled = false`.
5. Updates the status bar to the new steady-state label.
6. Returns `{ context, state: newState }` for the next transition in the chain.

**`getIpcAddress()` (lines 362–397)**

Reads `CachedIpcState` from `context.workspaceState`. Reads the active js-debug extension path by probing for `ms-vscode.js-debug-nightly` before `ms-vscode.js-debug` (lines 373–374). Constructs a `settingsValue` string. If the cached `jsDebugPath` and `settingsValue` both match, returns the cached `ipcAddress` directly (lines 377–379). Otherwise, executes `extension.js-debug.setAutoAttachVariables` with the old IPC address as argument (line 381–384); on success, stores the returned `ipcAddress` plus the new `jsDebugPath` and `settingsValue` in workspace state (lines 390–394) and returns the address.

**`getJsDebugSettingKey()` (lines 399–407)**

Iterates over `SETTINGS_CAUSE_REFRESH`, reads each setting from `vscode.workspace.getConfiguration(SETTING_SECTION)`, accumulates into a plain object, and serialises to JSON. This string is the `settingsValue` used as a cache key in `CachedIpcState`.

---

### Cross-Cutting Synthesis

The `debug-auto-launch` extension is a thin coordination layer between VS Code's configuration/UI surface and the `ms-vscode.js-debug` (or `js-debug-nightly`) extension. It owns exactly one concern: translating the `debug.javascript.autoAttachFilter` setting value into a live IPC server whose existence signals to js-debug that it should inject `NODE_OPTIONS`/`VSCODE_INSPECTOR_OPTIONS` into new terminal processes.

The four-state machine (`Disabled`, `OnlyWithFlag`, `Smart`, `Always`) is entirely flat in its transition logic — there are no exit handlers, and all active states share the same entry action (`createAttachServer`). The only structural difference between active states is their semantic meaning as interpreted by js-debug when it reads the setting. All serialisation of concurrent transitions is achieved by chaining `.then()` on a single module-level promise (`currentState`), a lightweight sequencing mechanism that avoids race conditions without any explicit mutex.

The IPC protocol is minimal: length-framing via a NUL terminator byte, JSON payload, single-byte ACK/NAK response. This design is entirely dictated by what the Node.js child processes (injected with `VSCODE_INSPECTOR_OPTIONS`) send when they start up and want to be attached to.

Porting this partition to Tauri/Rust would require: (1) a Rust equivalent of the `net` Unix-socket/named-pipe server with the same NUL-framed JSON protocol; (2) a Rust or WebView equivalent of the `vscode.workspace.getConfiguration`, `vscode.commands.executeCommand`, and `vscode.window.createStatusBarItem` / `createQuickPick` APIs; (3) a persistent workspace-state store replacing `context.workspaceState`; and (4) a mechanism for cross-extension command dispatch to replace the three `extension.js-debug.*` commands, which are the true orchestration surface — this extension is entirely inert without them.

---

### Out-of-Partition References

The following symbols are invoked by this extension but are defined in external partitions:

**Cross-extension commands dispatched via `vscode.commands.executeCommand`**

| Command | Call site | Purpose |
|---|---|---|
| `extension.js-debug.setAutoAttachVariables` | `extension.ts:381` | Asks js-debug to inject auto-attach env vars into new terminals; returns `{ ipcAddress: string }` (the socket path js-debug has chosen). Accepts the previous IPC address as argument for potential reuse. |
| `extension.js-debug.autoAttachToProcess` | `extension.ts:263` | Forwards the JSON payload received over the IPC socket to js-debug, triggering the actual debugger attach to the Node.js process. |
| `extension.js-debug.clearAutoAttachVariables` | `extension.ts:200` | Instructs js-debug to remove auto-attach environment variable overrides from future terminals. |

**VS Code API surfaces consumed**

| API | Usage location |
|---|---|
| `vscode.workspace.getConfiguration(SETTING_SECTION)` | `readCurrentState():193`, `toggleAutoAttachSetting():113`, `getJsDebugSettingKey():401` |
| `vscode.workspace.onDidChangeConfiguration` | `activate():71` |
| `vscode.commands.registerCommand` | `activate():67` |
| `vscode.window.createQuickPick<PickItem>()` | `toggleAutoAttachSetting():117` |
| `vscode.window.createStatusBarItem()` | `updateStatusBar():325` |
| `vscode.extensions.getExtension('ms-vscode.js-debug-nightly')` | `getIpcAddress():373` |
| `vscode.extensions.getExtension('ms-vscode.js-debug')` | `getIpcAddress():374` |
| `context.workspaceState.get / update` | `getIpcAddress():366,390`, `clearJsDebugAttachState():198–199` |
| `vscode.ConfigurationTarget` (Global / Workspace / WorkspaceFolder) | `getDefaultScope():96–106`, `toggleAutoAttachSetting():116,174` |
| `vscode.ThemeIcon` | `toggleAutoAttachSetting():143` |
| `vscode.l10n.t()` | All user-visible string constants (lines 18–41) |

**Node.js built-in modules**

| Module | Usage |
|---|---|
| `fs.promises` (`fs`) | `createAttachServer():224` — `fs.access(dirname(ipcAddress))` to detect moved temp dir; `createServerInner():243` — `fs.unlink(ipcAddress)` to remove stale socket file. |
| `net` (`createServer`, `Server`) | `createServerInstance():248–274` — creates the Unix socket / named pipe listener. |
| `path` (`dirname`) | `createAttachServer():224` — extracts directory of socket path for accessibility check. |
