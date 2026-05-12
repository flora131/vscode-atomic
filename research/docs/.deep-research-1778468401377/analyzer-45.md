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
