# Partition 45 of 79 — Findings

## Scope
`extensions/debug-auto-launch/` (2 files, 425 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
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

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Pattern Finder Research: `extensions/debug-auto-launch/`
## VS Code Debug API & State Management Patterns

Research into the debug-auto-launch extension (425 LOC across 2 files) to identify distinct implementation patterns for Debug API integration, IPC communication, and state management relevant to Tauri/Rust porting.

---

#### Pattern 1: State Enum with Localized Label Maps
**Where:** `src/extension.ts:11-35`
**What:** Defines a state machine with multiple dictionaries mapping state to UI labels (status bar, quick pick, descriptions). This pattern enables consistent state representation across UI surfaces.

```typescript
const enum State {
	Disabled = 'disabled',
	OnlyWithFlag = 'onlyWithFlag',
	Smart = 'smart',
	Always = 'always',
}
const TEXT_STATUSBAR_LABEL = {
	[State.Disabled]: vscode.l10n.t('Auto Attach: Disabled'),
	[State.Always]: vscode.l10n.t('Auto Attach: Always'),
	[State.Smart]: vscode.l10n.t('Auto Attach: Smart'),
	[State.OnlyWithFlag]: vscode.l10n.t('Auto Attach: With Flag'),
};

const TEXT_STATE_LABEL = {
	[State.Disabled]: vscode.l10n.t('Disabled'),
	[State.Always]: vscode.l10n.t('Always'),
	[State.Smart]: vscode.l10n.t('Smart'),
	[State.OnlyWithFlag]: vscode.l10n.t('Only With Flag'),
};
const TEXT_STATE_DESCRIPTION = {
	[State.Disabled]: vscode.l10n.t('Auto attach is disabled and not shown in status bar'),
	[State.Always]: vscode.l10n.t('Auto attach to every Node.js process launched in the terminal'),
	[State.Smart]: vscode.l10n.t("Auto attach when running scripts that aren't in a node_modules folder"),
	[State.OnlyWithFlag]: vscode.l10n.t('Only auto attach when the `--inspect` flag is given')
};
```

**Variations:** Used throughout for UI rendering (status bar at line 333, quick pick at lines 122-124).

---

#### Pattern 2: Promise-Based State Management with Queuing
**Where:** `src/extension.ts:58, 341-355`
**What:** Uses a Promise chain (`currentState`) to serialize state transitions, ensuring state changes are queued and processed in order rather than racing.

```typescript
let currentState: Promise<{ context: vscode.ExtensionContext; state: State | null }>;

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

**Variations:** Initialization at line 64 (`currentState = Promise.resolve(...)`), state refresh at lines 90-93.

---

#### Pattern 3: State Transition Map with Type-Safe Handlers
**Where:** `src/extension.ts:297-313`
**What:** Maps each state to an async handler function that executes when entering that state. Provides type-safe state machine implementation.

```typescript
const transitions: { [S in State]: (context: vscode.ExtensionContext) => Promise<void> } = {
	async [State.Disabled](context) {
		await clearJsDebugAttachState(context);
	},

	async [State.OnlyWithFlag](context) {
		await createAttachServer(context);
	},

	async [State.Smart](context) {
		await createAttachServer(context);
	},

	async [State.Always](context) {
		await createAttachServer(context);
	},
};
```

**Variations:** Called from `updateAutoAttach` at line 351.

---

#### Pattern 4: Configuration Change Detection with Specific Setting Whitelist
**Where:** `src/extension.ts:53-81`
**What:** Maintains a Set of specific configuration keys that trigger state refresh, preventing unnecessary re-initialization on unrelated setting changes.

```typescript
const SETTINGS_CAUSE_REFRESH = new Set(
	['autoAttachSmartPattern', SETTING_STATE].map(s => `${SETTING_SECTION}.${s}`),
);

// ... in activate():
context.subscriptions.push(
	vscode.workspace.onDidChangeConfiguration(e => {
		// Whenever a setting is changed, disable auto attach, and re-enable
		// it (if necessary) to refresh variables.
		if (
			e.affectsConfiguration(`${SETTING_SECTION}.${SETTING_STATE}`) ||
			[...SETTINGS_CAUSE_REFRESH].some(setting => e.affectsConfiguration(setting))
		) {
			refreshAutoAttachVars();
		}
	}),
);
```

**Variations:** Extended check at lines 74-77 for nested configuration detection.

---

#### Pattern 5: Peer Extension Discovery with Fallback Chain
**Where:** `src/extension.ts:372-374`
**What:** Detects related debug extensions (nightly and stable versions) using a fallback pattern, enabling graceful degradation if one is unavailable.

```typescript
const jsDebugPath =
	vscode.extensions.getExtension('ms-vscode.js-debug-nightly')?.extensionPath ||
	vscode.extensions.getExtension('ms-vscode.js-debug')?.extensionPath;
```

**Variations:** Used in IPC address caching validation (lines 377).

---

#### Pattern 6: IPC Server with NUL-Terminated Message Protocol
**Where:** `src/extension.ts:248-275`
**What:** Creates a TCP server listening on a named pipe/socket, parsing messages delimited by NUL bytes (0x00). Executes Debug API commands based on received JSON data.

```typescript
const createServerInstance = (ipcAddress: string) =>
	new Promise<Server>((resolve, reject) => {
		const s = createServer(socket => {
			const data: Buffer[] = [];
			socket.on('data', async chunk => {
				if (chunk[chunk.length - 1] !== 0) {
					// terminated with NUL byte
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
```

**Variations:** Error handling with file cleanup at lines 240-245 (`createServerInner`).

---

#### Pattern 7: Workspace State Caching with Invalidation Strategy
**Where:** `src/extension.ts:287-397`
**What:** Caches IPC configuration in workspace state, invalidating when related extension paths or settings change. Implements cache coherency across VS Code activations.

```typescript
interface CachedIpcState {
	ipcAddress: string;
	jsDebugPath: string | undefined;
	settingsValue: string;
}

async function getIpcAddress(context: vscode.ExtensionContext) {
	const cachedIpc = context.workspaceState.get<CachedIpcState>(STORAGE_IPC);

	const jsDebugPath =
		vscode.extensions.getExtension('ms-vscode.js-debug-nightly')?.extensionPath ||
		vscode.extensions.getExtension('ms-vscode.js-debug')?.extensionPath;

	const settingsValue = getJsDebugSettingKey();
	if (cachedIpc?.jsDebugPath === jsDebugPath && cachedIpc?.settingsValue === settingsValue) {
		return cachedIpc.ipcAddress;
	}

	const result = await vscode.commands.executeCommand<{ ipcAddress: string }>(
		'extension.js-debug.setAutoAttachVariables',
		cachedIpc?.ipcAddress,
	);
	if (!result) {
		return;
	}

	const ipcAddress = result.ipcAddress;
	await context.workspaceState.update(STORAGE_IPC, {
		ipcAddress,
		jsDebugPath,
		settingsValue,
	} satisfies CachedIpcState);

	return ipcAddress;
}

function getJsDebugSettingKey() {
	const o: { [key: string]: unknown } = {};
	const config = vscode.workspace.getConfiguration(SETTING_SECTION);
	for (const setting of SETTINGS_CAUSE_REFRESH) {
		o[setting] = config.get(setting);
	}

	return JSON.stringify(o);
}
```

**Variations:** Cache clearing at lines 197-202.

---

#### Pattern 8: Discriminated Union Type for Quick Pick Results
**Where:** `src/extension.ts:109-110, 150-189`
**What:** Uses TypeScript discriminated unions to handle multiple result types from a single UI interaction (state selection, temp disable toggle, scope switching).

```typescript
type PickResult = { state: State } | { setTempDisabled: boolean } | { scope: vscode.ConfigurationTarget } | undefined;
type PickItem = vscode.QuickPickItem & ({ state: State } | { setTempDisabled: boolean });

// ... later, in result handling:
if ('scope' in result) {
	return await toggleAutoAttachSetting(context, result.scope);
}

if ('state' in result) {
	if (result.state !== current) {
		section.update(SETTING_STATE, result.state, scope);
	} else if (isTemporarilyDisabled) {
		result = { setTempDisabled: false };
	}
}

if ('setTempDisabled' in result) {
	updateStatusBar(context, current, true);
	isTemporarilyDisabled = result.setTempDisabled;
	if (result.setTempDisabled) {
		await destroyAttachServer();
	} else {
		await createAttachServer(context); // unsets temp disabled var internally
	}
	updateStatusBar(context, current, false);
}
```

**Variations:** Item construction at lines 120-132, promise resolution at lines 150-160.

---

## Summary

The debug-auto-launch extension demonstrates eight distinct architectural patterns essential for Debug API integration in VS Code. **Pattern 1** (state-label mapping) enables consistent multilingual UI representation. **Pattern 2** (Promise-based state queuing) prevents race conditions in state transitions. **Pattern 3** (state transition handlers) encapsulates state-specific logic. **Pattern 4** (configuration change detection whitelist) optimizes performance by avoiding unnecessary reinitializations. **Pattern 5** (peer extension discovery) gracefully handles multiple versions of dependent extensions. **Pattern 6** (IPC server with NUL-delimited messages) establishes inter-process communication for debug attachment orchestration. **Pattern 7** (workspace state caching with invalidation) maintains efficient IPC address reuse across sessions. **Pattern 8** (discriminated union types) provides type-safe handling of polymorphic UI results. Together, these patterns form a robust state machine for controlling automatic attachment to Node.js debug sessions, managing both global and workspace-scoped configuration, and coordinating with the js-debug extension through IPC and command-execution APIs. For Tauri/Rust porting, key translation challenges include: (1) maintaining Promise-based serialization in Rust async/await; (2) replicating the state machine with Enum + Handler maps; (3) implementing NUL-delimited IPC protocol over Unix sockets; (4) translating discriminated unions to Rust's enum pattern matching; and (5) managing VS Code extension context and workspace state through FFI/IPC boundaries.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
