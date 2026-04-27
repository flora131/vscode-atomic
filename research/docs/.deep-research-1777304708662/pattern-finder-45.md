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

