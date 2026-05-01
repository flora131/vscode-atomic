# VS Code Debug Auto-Launch Architecture: Patterns for Tauri/Rust Porting

## Research Question
What patterns exist in VS Code's debug auto-launch extension that would be relevant to porting core IDE functionality from TypeScript/Electron to Tauri/Rust?

## Scope
`extensions/debug-auto-launch/` - Node.js process auto-attach detection and management

---

## Pattern Examples: Debug Extension Integration

### Pattern 1: Configuration-Driven State Machine with Transitioned Actions
**Found in**: `extensions/debug-auto-launch/src/extension.ts:297-313`
**Used for**: Managing debug auto-attach states and their corresponding lifecycle actions

```typescript
/**
 * Map of logic that happens when auto attach states are entered and exited.
 * All state transitions are queued and run in order; promises are awaited.
 */
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

**Key aspects**:
- Each state (Disabled, OnlyWithFlag, Smart, Always) maps to async transition handlers
- State machine pattern with explicit state values (enum-like const)
- Guarantees order of promise execution through `.then()` chaining
- Transitions are asynchronous lifecycle methods managing server creation/destruction

### Pattern 2: Configuration Change Listener with Selective Refresh
**Found in**: `extensions/debug-auto-launch/src/extension.ts:70-81`
**Used for**: Detecting setting changes and refreshing auto-attach variables intelligently

```typescript
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

**Key aspects**:
- Listens to `vscode.workspace.onDidChangeConfiguration` event
- Filters changes with `e.affectsConfiguration()` to only react to relevant settings
- Settings are namespaced (e.g., `debug.javascript.autoAttachFilter`)
- Maintains a `Set` of settings that trigger refresh (`SETTINGS_CAUSE_REFRESH`)
- Refresh pattern: disable → re-enable to reset state variables

### Pattern 3: IPC Server for Process Attachment with Message Protocol
**Found in**: `extensions/debug-auto-launch/src/extension.ts:237-275`
**Used for**: Creating a socket-based communication channel for Node.js process auto-attachment

```typescript
const createServerInner = async (ipcAddress: string) => {
	try {
		return await createServerInstance(ipcAddress);
	} catch (e) {
		// On unix/linux, the file can 'leak' if the process exits unexpectedly.
		// If we see this, try to delete the file and then listen again.
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

**Key aspects**:
- Uses Node.js `net.createServer()` for IPC socket communication
- Handles platform-specific cleanup (unlink on Unix/Linux for stale socket files)
- Binary protocol: NUL-terminated messages with chunk accumulation
- Response codes: 0 = success, 1 = error
- Delegates actual attachment via `executeCommand()` to js-debug extension
- Converts binary data to JSON for inter-process communication

### Pattern 4: Extension Dependency Detection and Path Management
**Found in**: `extensions/debug-auto-launch/src/extension.ts:362-397`
**Used for**: Detecting and caching debug extension paths for environment variable setup

```typescript
/**
 * Gets the IPC address for the server to listen on for js-debug sessions. This
 * is cached such that we can reuse the address of previous activations.
 */
async function getIpcAddress(context: vscode.ExtensionContext) {
	// Iff the `cachedData` is present, the js-debug registered environment
	// variables for this workspace--cachedData is set after successfully
	// invoking the attachment command.
	const cachedIpc = context.workspaceState.get<CachedIpcState>(STORAGE_IPC);

	// We invalidate the IPC data if the js-debug path changes, since that
	// indicates the extension was updated or reinstalled and the
	// environment variables will have been lost.
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
```

**Key aspects**:
- Detects dependent extensions by ID with fallback (nightly vs stable)
- Caches IPC configuration in `workspaceState` for reuse
- Invalidates cache when: extension path changes OR settings change
- Serializes settings object to JSON for comparison
- Commands interface to delegate to dependent extension
- Type-safe response extraction with generics

### Pattern 5: Status Bar Integration with Temporary State Override
**Found in**: `extensions/debug-auto-launch/src/extension.ts:318-336`
**Used for**: Displaying auto-attach status and supporting temporary disable/enable

```typescript
/**
 * Ensures the status bar text reflects the current state.
 */
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
- Lazy initialization of status bar item with named ID
- Conditional visibility: hidden when disabled and not busy
- Loading spinner indicator for state transitions
- Independent `isTemporarilyDisabled` flag overlays persistent state
- Command binding for quick-pick interaction
- Localization with `vscode.l10n.t()`

### Pattern 6: Quick Pick UI for State Selection with Scope Toggle
**Found in**: `extensions/debug-auto-launch/src/extension.ts:112-190`
**Used for**: Interactive configuration selection between global and workspace scopes

```typescript
async function toggleAutoAttachSetting(context: vscode.ExtensionContext, scope?: vscode.ConfigurationTarget): Promise<void> {
	const section = vscode.workspace.getConfiguration(SETTING_SECTION);
	scope = scope || getDefaultScope(section.inspect(SETTING_STATE));

	const isGlobalScope = scope === vscode.ConfigurationTarget.Global;
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

	quickPick.dispose();

	if (!result) {
		return;
	}

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
}
```

**Key aspects**:
- Discriminated union pattern for UI result handling (`state` | `setTempDisabled` | `scope`)
- Configuration scope selection (Global vs Workspace/Folder)
- Defaults to most-specific scope with `getDefaultScope(inspect())`
- Button-based scope toggling (switch between global/workspace)
- Recursive call pattern for scope switching
- Updates configuration via `section.update(setting, value, scope)`
- Conditional temporary disable option based on current state

### Pattern 7: Debug Configuration Provider Registration
**Found in**: `extensions/debug-server-ready/src/extension.ts:340-351`
**Used for**: Registering debug configuration provider for wildcard and filtering logic

```typescript
context.subscriptions.push(vscode.debug.registerDebugConfigurationProvider('*', {
	resolveDebugConfigurationWithSubstitutedVariables(_folder: vscode.WorkspaceFolder | undefined, debugConfiguration: vscode.DebugConfiguration) {
		if (debugConfiguration.type && debugConfiguration.serverReadyAction) {
			if (!trackers.has(debugConfiguration.type)) {
				trackers.add(debugConfiguration.type);
				startTrackerForType(context, debugConfiguration.type);
			}
		}
		return debugConfiguration;
	}
}));
```

**Key aspects**:
- Wildcard provider registration for all debug config types
- Lazy registration pattern: initialize trackers only when needed
- Configuration inspection without modification
- Deduplication with Set-based tracking
- Returns modified configuration object

### Pattern 8: Debug Adapter Tracker Factory for Protocol Interception
**Found in**: `extensions/debug-server-ready/src/extension.ts:353-393`
**Used for**: Intercepting debug adapter protocol messages for pattern detection

```typescript
function startTrackerForType(context: vscode.ExtensionContext, type: string) {

	// scan debug console output for a PORT message
	context.subscriptions.push(vscode.debug.registerDebugAdapterTrackerFactory(type, {
		createDebugAdapterTracker(session: vscode.DebugSession) {
			const detector = ServerReadyDetector.start(session);
			if (detector) {
				let runInTerminalRequestSeq: number | undefined;
				return {
					onDidSendMessage: m => {
						if (m.type === 'event' && m.event === 'output' && m.body) {
							switch (m.body.category) {
								case 'console':
								case 'stderr':
								case 'stdout':
									if (m.body.output) {
										detector.detectPattern(m.body.output);
									}
									break;
								default:
									break;
							}
						}
						if (m.type === 'request' && m.command === 'runInTerminal' && m.arguments) {
							if (m.arguments.kind === 'integrated') {
								runInTerminalRequestSeq = m.seq; // remember this to find matching response
							}
						}
					},
					onWillReceiveMessage: m => {
						if (runInTerminalRequestSeq && m.type === 'response' && m.command === 'runInTerminal' && m.body && runInTerminalRequestSeq === m.request_seq) {
							runInTerminalRequestSeq = undefined;
							ServerReadyDetector.rememberShellPid(session, m.body.shellProcessId);
						}
					}
				};
			}
			return undefined;
		}
	}));
}
```

**Key aspects**:
- Per-type tracker factory registration pattern
- Message interception at both send and receive boundaries
- Output category filtering (console, stderr, stdout)
- Sequence-based request/response correlation
- Session-to-detector mapping for state tracking
- Returns undefined when no tracking needed

---

## Pattern Categories Found

### Configuration Management
- **Settings namespace pattern**: Hierarchical setting keys (`section.setting`)
- **Configuration scope resolution**: Global, Workspace, and WorkspaceFolder targets
- **Configuration change listeners**: Selective filtering with `affectsConfiguration()`
- **Cached configuration state**: Invalidation strategies based on dependencies

### State Management
- **State machine with transitions**: Discriminated union for states with mapped handlers
- **Promise-based state queuing**: Sequential async state transitions
- **Temporary state overlays**: `isTemporarilyDisabled` flag independent of persistent state
- **Status bar synchronization**: Real-time UI reflection of internal state

### IPC Communication
- **Socket-based message protocol**: Binary framing with NUL terminators
- **Error recovery**: Platform-aware cleanup (unlink stale files)
- **Type-safe message handling**: JSON serialization with generic typing
- **Bidirectional status codes**: Simple response protocol (0 = success, 1 = error)

### Extension Integration
- **Dependency detection by ID**: Support for nightly and stable variants
- **Cross-extension command execution**: `executeCommand()` for delegation
- **Extension path tracking**: Invalidate cache when dependent extension changes
- **Lazy extension activation**: Initialize only when needed

### Debug Integration
- **Debug configuration provider registration**: Wildcard and type-specific providers
- **Debug session lifecycle hooks**: `onDidStart` and `onDidTerminate`
- **Debug adapter protocol interception**: Message-level inspection and filtering
- **Terminal process tracking**: PID extraction from debug adapter messages

---

## Key Files Referenced

| File | Purpose | Key Patterns |
|------|---------|--------------|
| `extensions/debug-auto-launch/src/extension.ts` | Main auto-launch logic | State machine, IPC server, config listeners, status bar |
| `extensions/debug-auto-launch/package.json` | Extension manifest | Contribution points, activation events |
| `extensions/debug-server-ready/src/extension.ts` | Server-ready detection | Debug configuration providers, tracker factories |
| `extensions/vscode-api-tests/src/singlefolder-tests/debug.test.ts` | Debug API tests | Testing patterns for debug sessions |

---

## Implications for Tauri/Rust Porting

### Architecture Considerations
1. **State Machine Migration**: The transition-based pattern is highly portable to Rust enums + match expressions
2. **IPC Protocol**: Binary protocol can be implemented in Rust using `tokio` or similar async frameworks
3. **Configuration System**: Need equivalent to VS Code's multi-level configuration (Global/Workspace/Folder)
4. **Event System**: Observer pattern for config changes requires Rust event/subscription infrastructure
5. **Extension API**: Debug configuration providers and adapter trackers would need Rust equivalents

### Runtime Considerations
1. **Process Detection**: Node.js auto-attach detection logic depends on environment variables set by js-debug extension
2. **Socket Management**: Unix/Linux socket cleanup logic requires platform-specific Rust implementations
3. **Caching Strategy**: Workspace state persistence maps to Tauri's state management
4. **Status Bar**: Requires equivalent UI framework integration (likely Tauri's webview-based UI)
5. **Async Patterns**: Heavy use of Promises suggests need for Rust async/await throughout

### Integration Points
1. **Debug Adapter Protocol**: Would require DAP client implementation in Rust
2. **Terminal Integration**: Needs access to terminal process IDs and output streams
3. **Configuration Watching**: Real-time config change detection with efficient filtering
4. **Cross-Process Communication**: IPC over Unix sockets (Linux/Mac) and named pipes (Windows)

---

## Summary

The debug-auto-launch extension demonstrates VS Code's architectural patterns for:
- **Reactive configuration management** with fine-grained change detection
- **State machines with async transitions** for managing debugger lifecycle
- **Binary IPC protocols** for low-level process communication
- **Lazy extension dependency resolution** with caching and invalidation
- **Protocol-level debugging integration** via debug adapter tracker factories

These patterns are deeply integrated with VS Code's extension API and would require substantial infrastructure changes for a Tauri/Rust port, particularly around the event system, configuration management, and debug protocol handling.
