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

