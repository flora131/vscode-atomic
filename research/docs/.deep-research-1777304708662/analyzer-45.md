### Files Analysed

- `/Users/norinlavaee/vscode-atomic/extensions/debug-auto-launch/src/extension.ts` (407 LOC)
- `/Users/norinlavaee/vscode-atomic/extensions/debug-auto-launch/package.json` (48 LOC)

---

### Per-File Notes

#### `extensions/debug-auto-launch/package.json`

- **Activation**: `onStartupFinished` (line 19) — extension activates once after the VS Code workbench is fully loaded, not on demand.
- **Contributed command**: `extension.node-debug.toggleAutoAttach` (line 29) — the single surface area command users or keybindings can invoke.
- **Main entry**: `./out/extension` (line 22), compiled from TypeScript by `gulp compile-extension:debug-auto-launch`.
- **No runtime dependencies** — only `@types/node 22.x` as a devDependency. All runtime surface comes from the built-in `vscode` module and Node.js built-ins (`net`, `fs/promises`, `path`).
- **Capabilities**: `virtualWorkspaces: false` (line 13); `untrustedWorkspaces.supported: true` (line 15).

---

#### `extensions/debug-auto-launch/src/extension.ts`

##### Imports and Module-Level State (lines 1–62)

Four imports drive the whole extension:
- `fs` (promises API) from Node.js `fs` — used only for `fs.access` and `fs.unlink` during socket lifecycle.
- `createServer`, `Server` from Node.js `net` — core IPC mechanism.
- `dirname` from Node.js `path` — used to verify IPC socket directory still exists after unexpected errors.
- `vscode` — the VS Code extension API; provides commands, configuration, status bar, workspace state, quick pick.

Module-level mutable state (lines 58–62):
- `currentState: Promise<{ context, state }>` — a promise chain that serializes all state transitions.
- `statusItem: vscode.StatusBarItem | undefined` — lazily created; `undefined` when disabled.
- `server: Promise<Server | undefined> | undefined` — holds the live IPC server promise.
- `isTemporarilyDisabled: boolean` — session-only flag, reset on every permanent state change.

##### State Enum (lines 11–16)

`const enum State` with four string values: `Disabled`, `OnlyWithFlag`, `Smart`, `Always`. These directly mirror the `debug.javascript.autoAttachFilter` workspace/global setting values.

##### Localization Tables (lines 17–42)

Three record maps keyed by `State`: `TEXT_STATUSBAR_LABEL`, `TEXT_STATE_LABEL`, `TEXT_STATE_DESCRIPTION`. All strings are wrapped in `vscode.l10n.t()`. Two additional l10n strings handle the temporary-disable UX (lines 40–42).

##### Constants (lines 44–55)

- `TOGGLE_COMMAND = 'extension.node-debug.toggleAutoAttach'` (line 44) — matches `package.json` contributes.
- `STORAGE_IPC = 'jsDebugIpcState'` (line 45) — key used with `context.workspaceState` to persist `CachedIpcState`.
- `SETTING_SECTION = 'debug.javascript'` (line 47), `SETTING_STATE = 'autoAttachFilter'` (line 48).
- `SETTINGS_CAUSE_REFRESH` (lines 53–55) — a `Set` of two fully-qualified setting names (`debug.javascript.autoAttachSmartPattern`, `debug.javascript.autoAttachFilter`) that trigger `refreshAutoAttachVars` when changed.

##### `activate` (lines 63–84)

1. Sets `currentState` to a resolved promise of `{ context, state: null }` (line 64) — this seeds the serial promise chain.
2. Registers `TOGGLE_COMMAND` → `toggleAutoAttachSetting` via `context.subscriptions.push` (lines 66–68).
3. Registers `onDidChangeConfiguration` listener (lines 70–81): when any affected setting fires, calls `refreshAutoAttachVars()`.
4. Calls `updateAutoAttach(readCurrentState())` (line 83) to enter the correct initial state.

##### `deactivate` (lines 86–88)

Calls `destroyAttachServer()` and awaits it. This is the only cleanup path.

##### `readCurrentState` (lines 192–195)

Reads `debug.javascript.autoAttachFilter` via `vscode.workspace.getConfiguration`. Falls back to `State.Disabled` if unset.

##### `refreshAutoAttachVars` (lines 90–93)

Sequentially calls `updateAutoAttach(State.Disabled)` then `updateAutoAttach(readCurrentState())`. Because `updateAutoAttach` appends to the `currentState` promise chain, the disable transition always fully completes before the re-enable transition begins.

##### State Machine — `updateAutoAttach` (lines 341–356)

This is the core sequencing mechanism. Each call appends a `.then()` to `currentState`:
1. Compares `newState` to `oldState`; returns immediately if equal (line 344).
2. If `oldState !== null`, calls `updateStatusBar` with `busy = true` (line 348).
3. `await transitions[newState](context)` — dispatches to the transition handler (line 351).
4. Resets `isTemporarilyDisabled = false` (line 352).
5. Calls `updateStatusBar(context, newState, false)` (line 353).
6. Returns `{ context, state: newState }` as the new chain value (line 354).

All transitions are promise-chained through the single `currentState` variable, ensuring no concurrent state changes.

##### Transition Map — `transitions` (lines 297–313)

A `{ [S in State]: (context) => Promise<void> }` object:
- `State.Disabled` (line 298): calls `clearJsDebugAttachState(context)`.
- `State.OnlyWithFlag` (line 302): calls `createAttachServer(context)`.
- `State.Smart` (line 306): calls `createAttachServer(context)`.
- `State.Always` (line 310): calls `createAttachServer(context)`.

The three active states differ only in what environment variables `js-debug` sets when the IPC address is registered; the extension itself does not filter by state beyond this dispatch.

##### `clearJsDebugAttachState` (lines 197–203)

1. Checks if `server` is truthy or workspace state has a stored IPC key (line 198).
2. Clears the workspace state key `STORAGE_IPC` (line 199).
3. Executes command `extension.js-debug.clearAutoAttachVariables` (line 200) — tells js-debug to unset the environment variables it injected into integrated terminals.
4. Calls `destroyAttachServer()` (line 201).

##### IPC Server Lifecycle

`createAttachServer` (lines 209–235):
1. Calls `getIpcAddress(context)` to obtain or negotiate a socket path (line 210).
2. Assigns `server = createServerInner(ipcAddress).catch(...)` (line 215) — catches errors, and on non-Windows checks if the socket directory has disappeared; if so, calls `refreshAutoAttachVars()`.

`createServerInner` (lines 237–245):
1. Attempts `createServerInstance(ipcAddress)`.
2. On failure, calls `fs.unlink(ipcAddress)` (line 243) to remove a leaked Unix socket file, then retries `createServerInstance`.

`createServerInstance` (lines 248–275):
1. Calls Node.js `net.createServer(socket => {...})` (line 250).
2. Per-socket data handler accumulates `Buffer[]` chunks until a NUL byte (`\0`, value `0`) terminates the message (lines 252–259).
3. On message complete: `JSON.parse`s the concatenated buffer, then calls `vscode.commands.executeCommand('extension.js-debug.autoAttachToProcess', parsedData)` (lines 262–264).
4. Writes back a single-byte acknowledgement: `Buffer.from([0])` on success (line 265), `Buffer.from([1])` on error (line 268).
5. Server listens on `ipcAddress` (line 274).

`destroyAttachServer` (lines 280–285): awaits the server promise; if a live instance exists, closes it via `instance.close(r)`.

##### IPC Address Negotiation — `getIpcAddress` (lines 362–397)

1. Reads `CachedIpcState` from `context.workspaceState.get(STORAGE_IPC)` (line 366).
2. Reads the path of either `ms-vscode.js-debug-nightly` or `ms-vscode.js-debug` extension (lines 373–374).
3. Computes `settingsValue = getJsDebugSettingKey()` (line 376) — a JSON serialization of the two refresh-sensitive settings.
4. Cache hit condition (line 377): cached `jsDebugPath` and `settingsValue` both match → returns `cachedIpc.ipcAddress` directly.
5. Cache miss: executes `extension.js-debug.setAutoAttachVariables` command (lines 381–383), passing the old IPC address (if any) so js-debug can clean up stale environment injections.
6. Stores the returned `{ ipcAddress }` together with `jsDebugPath` and `settingsValue` into workspace state (lines 390–394).

##### `getJsDebugSettingKey` (lines 399–407)

Iterates `SETTINGS_CAUSE_REFRESH`, reads each setting value from `vscode.workspace.getConfiguration`, and returns `JSON.stringify` of the result. This is used as a cache key to detect configuration drift between activations.

##### Status Bar — `updateStatusBar` (lines 318–336)

- If `state === State.Disabled && !busy`: hides and returns (line 319).
- Lazily creates the `StatusBarItem` with id `status.debug.autoAttach`, `StatusBarAlignment.Left`, command `TOGGLE_COMMAND` (lines 324–329).
- Computes text: prepends `$(loading) ` when `busy = true`; uses `TEXT_TEMP_DISABLE_LABEL` if `isTemporarilyDisabled`, otherwise `TEXT_STATUSBAR_LABEL[state]` (lines 332–334).

##### Toggle UI — `toggleAutoAttachSetting` (lines 112–190)

1. Determines current config scope via `getDefaultScope` (line 114), which inspects which configuration target has a non-undefined value for `autoAttachFilter`.
2. Builds `QuickPickItem[]` for all four states plus an optional temp-disable/re-enable item (lines 120–133).
3. Shows the quick pick with a scope-toggle button (`folder` or `globe` ThemeIcon) (lines 141–148).
4. Awaits user choice (lines 150–160).
5. If a scope button was pressed, recurses with the opposite scope (line 169).
6. If a state was chosen and differs from current, writes it to `section.update(SETTING_STATE, result.state, scope)` (line 174) — this fires `onDidChangeConfiguration` which drives `refreshAutoAttachVars`.
7. If temp-disable toggled: calls `destroyAttachServer` or `createAttachServer` directly and updates the status bar (lines 180–189).

##### `CachedIpcState` Interface (lines 287–291)

```ts
interface CachedIpcState {
  ipcAddress: string;
  jsDebugPath: string | undefined;
  settingsValue: string;
}
```

Stored in `context.workspaceState` under key `'jsDebugIpcState'`. Persists across VS Code restarts for the same workspace.

---

### Cross-Cutting Synthesis

**Porting debug-auto-launch to Tauri/Rust — key surface areas:**

The extension's architecture has four porting concerns:

1. **Unix socket IPC server.** The entire attach mechanism is a `net.createServer` Unix domain socket (lines 248–275) that relays NUL-terminated JSON messages from Node.js child processes to the debugger. In Rust/Tauri this maps directly to `tokio::net::UnixListener`. The framing protocol (accumulate until `\0`, reply `[0]` or `[1]`) is trivial to replicate. On Windows, Node.js `net` uses named pipes under `\\.\pipe\`; Tauri would need a conditional `tokio::net::windows::named_pipe` path.

2. **Cross-extension command dispatch.** Three VS Code commands act as the glue to the `ms-vscode.js-debug` extension: `setAutoAttachVariables` (negotiates the socket path and injects environment variables into integrated terminal sessions), `clearAutoAttachVariables` (unsets those variables), and `autoAttachToProcess` (triggers attach for a specific process). These are synchronous RPC calls via `vscode.commands.executeCommand`. In a Tauri context there is no equivalent built-in cross-plugin command bus; this inter-plugin communication contract would need to be redesigned — either via Tauri's `invoke`/event system between a Rust backend plugin and a JS-debug equivalent plugin, or via a shared IPC sidecar.

3. **Persistent workspace state.** `context.workspaceState` (a VS Code key-value store scoped to the workspace) is used to cache `CachedIpcState` across sessions (lines 366, 390–394). Tauri's equivalent is `tauri-plugin-store` or direct filesystem persistence in an app-data directory, with manual cache-invalidation logic equivalent to the `jsDebugPath`/`settingsValue` check at line 377.

4. **Configuration and status bar.** `vscode.workspace.getConfiguration` reads from VS Code's layered settings (user/workspace/folder). Tauri has no such layered config system out of the box; a Rust-side config layer or `tauri-plugin-store` with explicit scope resolution would be needed. The status bar item with a command binding (`createStatusBarItem`, line 325) maps to a Tauri window system tray or webview UI element — there is no native IDE status bar primitive in Tauri.

The state machine itself (`updateAutoAttach` serializing transitions through a promise chain, lines 341–356) ports cleanly to Rust as an async task with a `tokio::sync::Mutex<State>` or a state actor. The NUL-byte framing protocol and the `fs.unlink`-on-stale-socket retry (lines 237–245) are both idiomatically expressible in Rust. The fundamental porting challenge is the absence of any equivalent to VS Code's inter-extension command bus and layered workspace configuration, both of which are deeply assumed by this extension's design.

---

### Out-of-Partition References

- `ms-vscode.js-debug` / `ms-vscode.js-debug-nightly` — external extensions providing the three commands this extension delegates to. Source is not in this partition. The three command contracts are: `extension.js-debug.setAutoAttachVariables(oldIpcAddress?) → { ipcAddress }`, `extension.js-debug.clearAutoAttachVariables() → void`, `extension.js-debug.autoAttachToProcess(processData) → void`.
- `vscode.workspace.getConfiguration('debug.javascript')` — the `autoAttachFilter` and `autoAttachSmartPattern` settings are contributed by the js-debug extension's own `package.json`, not by this extension.
- `context.workspaceState` — VS Code's internal SQLite-backed workspace memento, implemented in `src/vs/workbench/api/common/extHostMemento.ts` (outside this partition).
- `vscode.commands.executeCommand` — cross-extension command routing is implemented in `src/vs/workbench/api/common/extHostCommands.ts` (outside this partition).
