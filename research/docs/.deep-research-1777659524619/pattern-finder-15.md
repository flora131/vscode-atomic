# VS Code Command Registration Patterns - Emmet Extension

Research into command registration patterns for porting VS Code functionality to Tauri/Rust.
Partition 15 of 79: `extensions/emmet/` (46 files, 8,451 LOC)

## Pattern 1: Basic Command Registration Without Parameters

**Where:** `extensions/emmet/src/emmetCommon.ts:37-39`
**What:** Register a command with no parameters that delegates to a function.

```typescript
context.subscriptions.push(vscode.commands.registerCommand('editor.emmet.action.removeTag', () => {
	return removeTag();
}));
```

**Variations / call-sites:**
- `extensions/emmet/src/emmetCommon.ts:48-50` (matchTag)
- `extensions/emmet/src/emmetCommon.ts:52-54` (balanceOut)
- `extensions/emmet/src/emmetCommon.ts:56-58` (balanceIn)
- `extensions/emmet/src/emmetCommon.ts:60-62` (splitJoinTag)
- `extensions/emmet/src/emmetCommon.ts:64-66` (mergeLines)
- `extensions/emmet/src/emmetCommon.ts:68-70` (toggleComment)
- `extensions/emmet/src/emmetCommon.ts:72-78` (nextEditPoint/prevEditPoint)
- `extensions/emmet/src/emmetCommon.ts:80-86` (selectNextItem/selectPrevItem)
- `extensions/emmet/src/emmetCommon.ts:88-90` (evaluateMathExpression)
- `extensions/emmet/src/emmetCommon.ts:116-118` (reflectCSSValue)


## Pattern 2: Command Registration with Optional Parameters

**Where:** `extensions/emmet/src/emmetCommon.ts:29-31`
**What:** Register a command that accepts optional arguments and forwards them to the handler.

```typescript
context.subscriptions.push(vscode.commands.registerCommand('editor.emmet.action.wrapWithAbbreviation', (args) => {
	wrapWithAbbreviation(args);
}));
```

**Variations / call-sites:**
- `extensions/emmet/src/emmetCommon.ts:33-35` (emmet.expandAbbreviation with args)


## Pattern 3: Command Registration with Type-Checked Parameters

**Where:** `extensions/emmet/src/emmetCommon.ts:41-46`
**What:** Register a command that validates parameter types before execution.

```typescript
context.subscriptions.push(vscode.commands.registerCommand('editor.emmet.action.updateTag', (inputTag) => {
	if (inputTag && typeof inputTag === 'string') {
		return updateTag(inputTag);
	}
	return updateTag(undefined);
}));
```

**Variations / call-sites:**
- Only this command uses inline type validation in emmetCommon.ts


## Pattern 4: Command Registration with Parameter Transformation

**Where:** `extensions/emmet/src/emmetCommon.ts:92-114`
**What:** Register multiple commands that call the same handler with different numeric parameters.

```typescript
context.subscriptions.push(vscode.commands.registerCommand('editor.emmet.action.incrementNumberByOneTenth', () => {
	return incrementDecrement(0.1);
}));

context.subscriptions.push(vscode.commands.registerCommand('editor.emmet.action.incrementNumberByOne', () => {
	return incrementDecrement(1);
}));

context.subscriptions.push(vscode.commands.registerCommand('editor.emmet.action.incrementNumberByTen', () => {
	return incrementDecrement(10);
}));
```

**Variations / call-sites:**
- `extensions/emmet/src/emmetCommon.ts:104-114` (decrement variants with negative deltas: -0.1, -1, -10)


## Pattern 5: Command Registration with Deferred Module Import

**Where:** `extensions/emmet/src/node/emmetNodeMain.ts:13-15`
**What:** Register a command that dynamically imports the handler module before execution (lazy loading).

```typescript
context.subscriptions.push(vscode.commands.registerCommand('editor.emmet.action.updateImageSize', () => {
	return import('../updateImageSize').then(uis => uis.updateImageSize());
}));
```

**Variations / call-sites:**
- Only used in node-specific emmetNodeMain.ts (not available in browser version)


## Pattern 6: Command Registration with Nested Commands

**Where:** `extensions/emmet/src/emmetCommon.ts:120-122`
**What:** Register a command that delegates to another VS Code command via `executeCommand`.

```typescript
context.subscriptions.push(vscode.commands.registerCommand('workbench.action.showEmmetCommands', () => {
	vscode.commands.executeCommand('workbench.action.quickOpen', '>Emmet: ');
}));
```

**Variations / call-sites:**
- Only this command uses command delegation pattern


## Pattern 7: Subscription Management

**Where:** `extensions/emmet/src/emmetCommon.ts:24-155`
**What:** All commands are registered via `context.subscriptions.push()` to ensure cleanup on deactivation.

```typescript
export function activateEmmetExtension(context: vscode.ExtensionContext) {
	// ... initialization code ...
	
	context.subscriptions.push(vscode.commands.registerCommand('command.id', () => {
		// handler
	}));
	
	// Event listeners also pushed to subscriptions
	context.subscriptions.push(vscode.workspace.onDidChangeConfiguration((e) => {
		if (e.affectsConfiguration('emmet.includeLanguages')) {
			refreshCompletionProviders(context);
		}
	}));
}

export function deactivate() {
	clearCompletionProviderInfo();
	clearParseCache();
}
```

**Variations / call-sites:**
- Workspace event handlers: `onDidChangeConfiguration` (line 124), `onDidSaveTextDocument` (line 133), `onDidOpenTextDocument` (line 140), `onDidCloseTextDocument` (line 148)
- Completion providers also registered via subscriptions: line 203-221


## Summary

The emmet extension demonstrates 7 distinct command registration patterns in the VS Code API:

1. **Simple passthrough**: Commands without parameters that directly call handlers
2. **Args forwarding**: Commands that pass caller arguments to handlers
3. **Type validation**: Commands that validate parameter types inline before delegating
4. **Parameter transformation**: Commands that apply fixed transformations to parameters (e.g., numeric variants)
5. **Lazy loading**: Commands using dynamic imports for performance (node-only feature)
6. **Command delegation**: Commands that invoke other VS Code commands rather than direct handlers
7. **Subscription lifecycle**: All handlers registered through ExtensionContext.subscriptions for proper cleanup

**For Tauri/Rust porting**, key considerations:
- Command registration requires ExtensionContext coupling (requires RPC or IPC mechanism)
- Event subscriptions follow observer pattern (translatable to Rust event channels)
- Lazy loading via dynamic imports would require Rust module system design
- Type validation happens in TypeScript layer (would move to Rust compile-time in Tauri)
- All 23 Emmet commands follow declarative pattern in package.json (line 266-382)

**File references:**
- Activation: `extensions/emmet/src/node/emmetNodeMain.ts:12-19`, `extensions/emmet/src/browser/emmetBrowserMain.ts:9-11`
- Common: `extensions/emmet/src/emmetCommon.ts:24-155`
- Manifest: `extensions/emmet/package.json:266-382` (command declarations)
