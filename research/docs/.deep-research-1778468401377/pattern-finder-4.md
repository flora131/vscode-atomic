# Terminal-Suggest Extension: VS Code API Patterns for Tauri/Rust Porting

## Overview
The `extensions/terminal-suggest/` extension provides shell completion suggestions for the integrated terminal. It heavily relies on VS Code's APIs for terminal integration, file system operations, event management, and completion providers.

---

## Core API Patterns Found

#### Pattern: Extension Activation and Context Management
**Where:** `extensions/terminal-suggest/src/terminalSuggestMain.ts:242-326`
**What:** Extension lifecycle management using `vscode.ExtensionContext`, including subscription management and global storage initialization.
```typescript
export async function activate(context: vscode.ExtensionContext) {
	pathExecutableCache = new PathExecutableCache();
	context.subscriptions.push(pathExecutableCache);
	
	globalStorageUri = context.globalStorageUri;
	await readGlobalsCache();

	const machineId = await vscode.env.machineId;
	const remoteAuthority = vscode.env.remoteName;

	context.subscriptions.push(vscode.window.registerTerminalCompletionProvider({
		async provideTerminalCompletions(
			terminal: vscode.Terminal, 
			terminalContext: vscode.TerminalCompletionContext, 
			token: vscode.CancellationToken
		): Promise<vscode.TerminalCompletionItem[] | vscode.TerminalCompletionList | undefined> {
			// Completion logic
		}
	}, '/', '\\'));

	context.subscriptions.push(vscode.commands.registerCommand(
		'terminal.integrated.suggest.clearCachedGlobals', 
		() => { cachedGlobals.clear(); }
	));
}
```
**Variations:** 
- Extensions must implement `activate()` function exported from root module
- `context.subscriptions` array manages disposable resources
- `context.globalStorageUri` provides persistent storage location
- `vscode.env` provides machine ID and remote environment info

---

#### Pattern: Terminal Completion Provider Registration
**Where:** `extensions/terminal-suggest/src/terminalSuggestMain.ts:254-320`
**What:** Registering a completion provider with the terminal, handling async completions and cancellation tokens.
```typescript
context.subscriptions.push(vscode.window.registerTerminalCompletionProvider({
	async provideTerminalCompletions(
		terminal: vscode.Terminal, 
		terminalContext: vscode.TerminalCompletionContext, 
		token: vscode.CancellationToken
	): Promise<vscode.TerminalCompletionItem[] | vscode.TerminalCompletionList | undefined> {
		if (token.isCancellationRequested) {
			return;
		}

		// Access terminal state
		const shellType: string | undefined = Object.hasOwn(terminal.state, 'shell') 
			? terminal.state.shell as string 
			: undefined;

		// Retrieve shell integration context
		const commandsInPath = await pathExecutableCache.getExecutablesInPath(
			terminal.shellIntegration?.env?.value, 
			terminalShellType
		);

		// Return completion items with optional file/directory context
		return new vscode.TerminalCompletionList(result.items, {
			showFiles: result.showFiles,
			showDirectories: result.showDirectories,
			globPattern,
			cwd,
		});
	}
}, '/', '\\'));
```
**Variations:**
- Provider takes shell separators as second and third arguments
- `vscode.TerminalCompletionItem[]` for simple list
- `vscode.TerminalCompletionList` for rich context with file globbing
- Supports cancellation via `token.isCancellationRequested`
- Terminal provides `shellIntegration` with environment and cwd info

---

#### Pattern: File System Operations via vscode.workspace
**Where:** `extensions/terminal-suggest/src/terminalSuggestMain.ts:188-237`
**What:** Reading and writing cache files with directory creation, error handling for missing files.
```typescript
async function writeGlobalsCache(): Promise<void> {
	if (!globalStorageUri) {
		return;
	}
	try {
		const terminalSuggestDir = vscode.Uri.joinPath(globalStorageUri, 'terminal-suggest');
		await vscode.workspace.fs.createDirectory(terminalSuggestDir);
		const cacheFile = vscode.Uri.joinPath(terminalSuggestDir, `${CACHE_KEY}.json`);
		const data = Buffer.from(JSON.stringify(obj), 'utf8');
		await vscode.workspace.fs.writeFile(cacheFile, data);
	} catch (err) {
		console.error('Failed to write terminal suggest globals cache:', err);
	}
}

async function readGlobalsCache(): Promise<void> {
	try {
		const terminalSuggestDir = vscode.Uri.joinPath(globalStorageUri, 'terminal-suggest');
		const cacheFile = vscode.Uri.joinPath(terminalSuggestDir, `${CACHE_KEY}.json`);
		const data = await vscode.workspace.fs.readFile(cacheFile);
		const obj = JSON.parse(data.toString());
		// Process cache
	} catch (err) {
		if (err instanceof vscode.FileSystemError && err.code === 'FileNotFound') {
			return;
		}
		console.error('Failed to read terminal suggest globals cache:', err);
	}
}
```
**Variations:**
- `vscode.Uri.joinPath()` for cross-platform path construction
- `vscode.workspace.fs.createDirectory()` creates directories recursively
- `vscode.workspace.fs.writeFile()` accepts `Uint8Array` or `Buffer`
- `vscode.workspace.fs.readFile()` returns `Uint8Array` requiring `.toString()`
- `vscode.FileSystemError` with `.code` property for specific error types

---

#### Pattern: File System Watching and Event Handling
**Where:** `extensions/terminal-suggest/src/terminalSuggestMain.ts:328-376`
**What:** Watching PATH directories for executable changes, debouncing file system events.
```typescript
async function watchPathDirectories(
	context: vscode.ExtensionContext, 
	env: ITerminalEnvironment, 
	pathExecutableCache: PathExecutableCache
): Promise<void> {
	const pathDirectories = new Set<string>();
	const activeWatchers = new Set<string>();
	let debounceTimer: NodeJS.Timeout | undefined;

	function handleChange() {
		if (debounceTimer) {
			clearTimeout(debounceTimer);
		}
		debounceTimer = setTimeout(() => {
			pathExecutableCache?.refresh();
			debounceTimer = undefined;
		}, 300);
	}

	for (const dir of pathDirectories) {
		const watcher = vscode.workspace.createFileSystemWatcher(
			new vscode.RelativePattern(vscode.Uri.file(dir), '*')
		);
		context.subscriptions.push(
			watcher,
			watcher.onDidCreate(() => handleChange()),
			watcher.onDidChange(() => handleChange()),
			watcher.onDidDelete(() => handleChange())
		);
		activeWatchers.add(dir);
	}
}
```
**Variations:**
- `vscode.RelativePattern` scopes watcher to specific directory
- File watcher provides three events: `onDidCreate`, `onDidChange`, `onDidDelete`
- Watcher itself is disposable and must be added to subscriptions
- Event callbacks need debouncing for batch operations

---

#### Pattern: Configuration Observation and Environment Variables
**Where:** `extensions/terminal-suggest/src/env/pathExecutableCache.ts:21-42`
**What:** Observing configuration changes and responding to environment variable updates.
```typescript
export class PathExecutableCache implements vscode.Disposable {
	private _disposables: vscode.Disposable[] = [];
	private _windowsExecutableExtensionsCache: WindowsExecutableExtensionsCache | undefined;

	constructor() {
		if (isWindows) {
			this._windowsExecutableExtensionsCache = new WindowsExecutableExtensionsCache(
				this._getConfiguredWindowsExecutableExtensions()
			);
			this._disposables.push(vscode.workspace.onDidChangeConfiguration(e => {
				if (e.affectsConfiguration(SettingsIds.CachedWindowsExecutableExtensions)) {
					this._windowsExecutableExtensionsCache?.update(
						this._getConfiguredWindowsExecutableExtensions()
					);
					this._cachedExes.clear();
				}
			}));
		}
	}

	dispose() {
		for (const d of this._disposables) {
			d.dispose();
		}
	}

	private _getConfiguredWindowsExecutableExtensions(): { [key: string]: boolean | undefined } | undefined {
		return vscode.workspace
			.getConfiguration(SettingsIds.SuggestPrefix)
			.get(SettingsIds.CachedWindowsExecutableExtensionsSuffixOnly);
	}
}
```
**Variations:**
- Implement `vscode.Disposable` interface for cleanup
- `vscode.workspace.onDidChangeConfiguration()` provides events with filter
- `ConfigurationChangeEvent.affectsConfiguration()` checks specific settings
- `.getConfiguration()` scoped to setting prefix
- `.get()` retrieves typed configuration values

---

#### Pattern: Directory Reading and File Type Inspection
**Where:** `extensions/terminal-suggest/src/env/pathExecutableCache.ts:152-218`
**What:** Scanning directories for executables using file system APIs, handling symbolic links.
```typescript
private async _getExecutablesInSinglePath(
	path: string, 
	pathSeparator: string, 
	labels: Set<string>
): Promise<Set<ICompletionResource> | undefined> {
	const fileResource = vscode.Uri.file(path);
	const files = await vscode.workspace.fs.readDirectory(fileResource);
	
	await Promise.all(
		files.map(([file, fileType]) => (async () => {
			let kind: vscode.TerminalCompletionItemKind | undefined;
			const resource = vscode.Uri.joinPath(fileResource, file);

			// Skip directories early
			if (fileType === vscode.FileType.Unknown || fileType === vscode.FileType.Directory) {
				return;
			}

			try {
				const lstat = await fs.lstat(resource.fsPath);
				if (lstat.isSymbolicLink()) {
					const symlinkRealPath = await fs.realpath(resource.fsPath);
					const isExec = await isExecutable(symlinkRealPath, windowsExecutableExtensions);
					if (!isExec) {
						return;
					}
					kind = vscode.TerminalCompletionItemKind.Method;
					formattedPath = `${resource.fsPath} -> ${symlinkRealPath}`;
				}
			} catch {
				return;
			}

			result.add({
				label: file,
				documentation: formattedPath,
				kind: kind ?? vscode.TerminalCompletionItemKind.Method
			});
			labels.add(file);
		})())
	);
	return result;
}
```
**Variations:**
- `vscode.workspace.fs.readDirectory()` returns `[name, type]` tuples
- `vscode.FileType` enum for filtering (Directory, Unknown, etc.)
- `.fsPath` property on Uri for Node.js fs module operations
- Supports mixing vscode filesystem API with Node.js fs
- Terminal completion item kinds: Method, Alias, Folder, Option, Flag, etc.

---

#### Pattern: Completion Item Data Structures
**Where:** `extensions/terminal-suggest/src/helpers/completionItem.ts:9-19` and `src/types.ts:8-18`
**What:** Building completion items with various label formats, replacement ranges, and documentation.
```typescript
export interface ICompletionResource {
	label: string | vscode.CompletionItemLabel;
	definitionCommand?: string;
	documentation?: string | vscode.MarkdownString;
	detail?: string;
	kind?: vscode.TerminalCompletionItemKind;
}

export function createCompletionItem(
	cursorPosition: number, 
	currentCommandString: string, 
	commandResource: ICompletionResource, 
	detail?: string, 
	documentation?: string | vscode.MarkdownString, 
	kind?: vscode.TerminalCompletionItemKind
): vscode.TerminalCompletionItem {
	const endsWithSpace = currentCommandString.endsWith(' ');
	const lastWord = endsWithSpace ? '' : currentCommandString.split(' ').at(-1) ?? '';
	return {
		label: commandResource.label,
		detail: detail ?? commandResource.detail ?? '',
		documentation,
		replacementRange: [cursorPosition - lastWord.length, cursorPosition],
		kind: kind ?? commandResource.kind ?? vscode.TerminalCompletionItemKind.Method
	};
}
```
**Variations:**
- `label` can be string or complex `CompletionItemLabel` with description
- `documentation` accepts plain strings or `vscode.MarkdownString` objects
- `replacementRange` is [start, end] indices for cursor-based replacement
- `kind` from `TerminalCompletionItemKind` enum (Method, Alias, Folder, Option, Flag, etc.)
- `detail` provides inline detail text

---

#### Pattern: Markdown Strings for Rich Documentation
**Where:** `extensions/terminal-suggest/src/shell/bash.ts:50-53`
**What:** Using `vscode.MarkdownString` for formatted documentation in completions.
```typescript
completions.push({
	label: { label: cmd, description },
	detail,
	documentation: new vscode.MarkdownString(documentation),
	kind: vscode.TerminalCompletionItemKind.Method
});
```
**Variations:**
- Constructor accepts raw markdown string
- Can include code blocks, formatting, links
- Supports isTrusted flag for HTML rendering

---

#### Pattern: Icon Type Mapping for SCM and PR Items
**Where:** `extensions/terminal-suggest/src/fig/figInterface.ts:374-385`
**What:** Converting icon URIs to terminal completion item kinds for version control items.
```typescript
function convertIconToKind(icon: string | undefined): vscode.TerminalCompletionItemKind | undefined {
	switch (icon) {
		case 'vscode://icon?type=10': return vscode.TerminalCompletionItemKind.ScmCommit;
		case 'vscode://icon?type=11': return vscode.TerminalCompletionItemKind.ScmBranch;
		case 'vscode://icon?type=12': return vscode.TerminalCompletionItemKind.ScmTag;
		case 'vscode://icon?type=13': return vscode.TerminalCompletionItemKind.ScmStash;
		case 'vscode://icon?type=14': return vscode.TerminalCompletionItemKind.ScmRemote;
		case 'vscode://icon?type=15': return vscode.TerminalCompletionItemKind.PullRequest;
		case 'vscode://icon?type=16': return vscode.TerminalCompletionItemKind.PullRequestDone;
		default: return undefined;
	}
}
```
**Variations:**
- Icon type numbers map to specific completion item kinds
- Used for git branch/tag/commit suggestions
- Pull request status variants (open vs closed)

---

#### Pattern: Shell Integration and Terminal Context Access
**Where:** `extensions/terminal-suggest/src/terminalSuggestMain.ts:255-307`
**What:** Accessing terminal shell integration data like current working directory and environment.
```typescript
async provideTerminalCompletions(
	terminal: vscode.Terminal, 
	terminalContext: vscode.TerminalCompletionContext, 
	token: vscode.CancellationToken
): Promise<vscode.TerminalCompletionItem[] | vscode.TerminalCompletionList | undefined> {
	currentTerminalEnv = terminal.shellIntegration?.env?.value ?? process.env;
	
	const shellType: string | undefined = Object.hasOwn(terminal.state, 'shell') 
		? terminal.state.shell as string 
		: undefined;

	// Access completion context
	const currentCommandString = getCurrentCommandAndArgs(
		terminalContext.commandLine, 
		terminalContext.cursorIndex, 
		terminalShellType
	);

	// Access cwd from shell integration
	if (terminal.shellIntegration?.env) {
		const homeDirCompletion = result.items.find(i => i.label === '~');
		if (homeDirCompletion && terminal.shellIntegration.env?.value?.HOME) {
			homeDirCompletion.documentation = getFriendlyResourcePath(
				vscode.Uri.file(terminal.shellIntegration.env.value.HOME), 
				pathSeparator, 
				vscode.TerminalCompletionItemKind.Folder
			);
		}
	}

	const cwd = result.cwd ?? terminal.shellIntegration?.cwd;
}
```
**Variations:**
- `terminal.state` is object type accessible via `hasOwn()`
- `terminal.shellIntegration` provides environment and cwd if available
- `terminalContext` contains command line and cursor position
- Shell integration may be undefined for certain scenarios
- Environment variables accessed via `.env.value` dictionary

---

#### Pattern: Command Registration for Cache Management
**Where:** `extensions/terminal-suggest/src/terminalSuggestMain.ts:323-325`
**What:** Registering extension commands for user-invoked actions.
```typescript
context.subscriptions.push(vscode.commands.registerCommand(
	'terminal.integrated.suggest.clearCachedGlobals', 
	() => {
		cachedGlobals.clear();
	}
));
```
**Variations:**
- Command IDs use dot notation (e.g., `terminal.integrated.suggest.clearCachedGlobals`)
- Commands can be invoked from command palette or programmatically
- Returned disposable must be added to context subscriptions

---

## Summary

The terminal-suggest extension requires these core VS Code APIs to function:

### Window/Terminal APIs
- `vscode.window.registerTerminalCompletionProvider()` - Register completion providers
- `vscode.Terminal` - Terminal instance with state and shell integration
- `vscode.TerminalCompletionContext` - Cursor position and command line info
- `vscode.TerminalCompletionItem[List]` - Completion data structures

### Workspace APIs
- `vscode.workspace.fs.readDirectory()` - List directory contents with file types
- `vscode.workspace.fs.readFile()` - Read files for cache persistence
- `vscode.workspace.fs.writeFile()` - Write cache files
- `vscode.workspace.fs.createDirectory()` - Create storage directories
- `vscode.workspace.createFileSystemWatcher()` - Watch directories for changes
- `vscode.workspace.onDidChangeConfiguration()` - Listen for setting changes
- `vscode.workspace.getConfiguration()` - Read user settings

### Environment APIs
- `vscode.env.machineId` - Machine-specific identifier
- `vscode.env.remoteName` - Remote authority (SSH, tunnel, etc.)
- `vscode.env.shell` - Shell executable path

### Infrastructure APIs
- `vscode.ExtensionContext` - Extension lifecycle and storage
- `vscode.Uri` and `vscode.RelativePattern` - Cross-platform paths
- `vscode.CancellationToken` - Async cancellation support
- `vscode.Disposable` - Resource cleanup
- `vscode.MarkdownString` - Rich text formatting
- `vscode.FileSystemError` - Filesystem error handling
- `vscode.FileType` enum - File type discrimination

**Porting Considerations:**
A Tauri/Rust port would need to provide equivalent bindings for terminal completion registration, file system operations with cross-platform path handling, configuration observation, and event-driven APIs. The shell integration APIs (accessing cwd, environment variables from running terminals) would require native IPC with the terminal process or similar mechanism.

