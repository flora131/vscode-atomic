# Partition 4 of 80 — Findings

## Scope
`extensions/terminal-suggest/` (197 files, 64,404 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Terminal Suggest Extension: File Location Index

**Scope:** `extensions/terminal-suggest/` (197 files, ~64,404 LOC)

## Implementation

### Core Entry Point
- `/extensions/terminal-suggest/src/terminalSuggestMain.ts` (23,325 lines) — Extension activation, completion provider registration, terminal integration, caching logic for shell globals, command coordination

### Fig Framework Integration (3,014 LOC)
- `/extensions/terminal-suggest/src/fig/figInterface.ts` (13,850 lines) — Main suggestion orchestration, spec matching, completion item generation
- `/extensions/terminal-suggest/src/fig/execute.ts` (72 lines) — External command execution wrapper around `spawnHelper2`, timeouts
- `/extensions/terminal-suggest/src/fig/autocomplete-parser/parseArguments.ts` (1,188 lines) — Argument parsing engine (forked from AWS Q Developer CLI / Fig tools)
- `/extensions/terminal-suggest/src/fig/shell-parser/parser.ts` (735 lines) — Shell command parsing
- `/extensions/terminal-suggest/src/fig/shell-parser/command.ts` (241 lines) — Command AST construction
- `/extensions/terminal-suggest/src/fig/fig-autocomplete-shared/specMetadata.ts` (106 lines) — Spec metadata handling
- `/extensions/terminal-suggest/src/fig/fig-autocomplete-shared/mixins.ts` (151 lines) — Mixin composition for completions
- `/extensions/terminal-suggest/src/fig/shared/utils.ts` (269 lines) — Utility functions
- `/extensions/terminal-suggest/src/fig/autocomplete-parser/caches.ts` (30 lines) — Parser caching
- `/extensions/terminal-suggest/src/fig/fig-autocomplete-shared/convert.ts` (78 lines) — Type conversion
- `/extensions/terminal-suggest/src/fig/shared/internal.ts` (38 lines) — Internal state
- `/extensions/terminal-suggest/src/fig/fig-autocomplete-shared/utils.ts` (13 lines) — Fig utilities
- `/extensions/terminal-suggest/src/fig/fig-autocomplete-shared/revert.ts` (46 lines) — Spec reversion logic
- `/extensions/terminal-suggest/src/fig/autocomplete-parser/errors.ts` (21 lines) — Error types
- `/extensions/terminal-suggest/src/fig/autocomplete/generators/` (subdirectory) — Generator implementations for suggestions

### Completion Specs (25,011 LOC across 115 files)
**Primary Tools (14 specs):**
- `/extensions/terminal-suggest/src/completions/git.ts` — Git command completions with branch/tracking file parsing
- `/extensions/terminal-suggest/src/completions/npm.ts` — npm/Node.js package manager completions
- `/extensions/terminal-suggest/src/completions/npx.ts` — npx command completions
- `/extensions/terminal-suggest/src/completions/yarn.ts` — Yarn package manager completions
- `/extensions/terminal-suggest/src/completions/pnpm.ts` — pnpm package manager completions
- `/extensions/terminal-suggest/src/completions/gh.ts` — GitHub CLI completions
- `/extensions/terminal-suggest/src/completions/code.ts` — VS Code command completions
- `/extensions/terminal-suggest/src/completions/code-insiders.ts` — VS Code Insiders completions
- `/extensions/terminal-suggest/src/completions/code-tunnel.ts` — VS Code tunnel completions
- `/extensions/terminal-suggest/src/completions/code-tunnel-insiders.ts` — VS Code tunnel Insiders completions
- `/extensions/terminal-suggest/src/completions/copilot.ts` — GitHub Copilot CLI completions
- `/extensions/terminal-suggest/src/completions/azd.ts` — Azure Developer CLI completions
- `/extensions/terminal-suggest/src/completions/cd.ts` — Directory change completions
- `/extensions/terminal-suggest/src/completions/set-location.ts` — PowerShell Set-Location completions

**Upstream POSIX/Unix Tools (100 specs in `upstream/` subdirectory):**
- Common utilities: `ls.ts`, `cd.ts`, `pwd.ts`, `mkdir.ts`, `rm.ts`, `cp.ts`, `mv.ts`, `ln.ts`, `find.ts`, `grep.ts`, `sed.ts`, `awk.ts`
- File operations: `cat.ts`, `head.ts`, `tail.ts`, `cut.ts`, `paste.ts`, `touch.ts`, `chmod.ts`, `chown.ts`, `stat.ts`, `lsof.ts`
- Package managers: `npm.ts`, `pip.ts`, `apt.ts`, `brew.ts`, `docker.ts`, `docker-compose.ts`
- Development tools: `git.ts`, `go.ts`, `python.ts`, `python3.ts`, `ruby.ts`, `node.ts`, `jq.ts`
- System utilities: `ps.ts`, `kill.ts`, `sudo.ts`, `ssh.ts`, `scp.ts`, `rsync.ts`, `curl.ts`, `ping.ts`

### Shell Integration (1,379 LOC)
- `/extensions/terminal-suggest/src/shell/common.ts` (97 lines) — Shared shell utilities, `spawnHelper2` command execution
- `/extensions/terminal-suggest/src/shell/bash.ts` (79 lines) — Bash global variable extraction
- `/extensions/terminal-suggest/src/shell/zsh.ts` (97 lines) — Zsh global variable extraction
- `/extensions/terminal-suggest/src/shell/fish.ts` (91 lines) — Fish shell global variable extraction
- `/extensions/terminal-suggest/src/shell/pwsh.ts` (173 lines) — PowerShell global variable extraction
- `/extensions/terminal-suggest/src/shell/zshBuiltinsCache.ts` (542 lines) — Cached Zsh builtin commands
- `/extensions/terminal-suggest/src/shell/fishBuiltinsCache.ts` (300 lines) — Cached Fish builtin commands

### Environment & File Helpers (545 LOC)
- `/extensions/terminal-suggest/src/env/pathExecutableCache.ts` — Executable discovery and caching, PATH parsing, workspace configuration
- `/extensions/terminal-suggest/src/helpers/executable.ts` (107 lines) — Executable lookup and validation
- `/extensions/terminal-suggest/src/helpers/keyvalue.ts` (356 lines) — Key-value parsing (environment variables, aliases)
- `/extensions/terminal-suggest/src/helpers/completionItem.ts` (19 lines) — Completion item factory
- `/extensions/terminal-suggest/src/helpers/os.ts` (10 lines) — Platform detection (Windows check)
- `/extensions/terminal-suggest/src/helpers/filepaths.ts` (17 lines) — Path normalization
- `/extensions/terminal-suggest/src/helpers/file.ts` (8 lines) — File extension handling
- `/extensions/terminal-suggest/src/helpers/uri.ts` (20 lines) — URI conversion
- `/extensions/terminal-suggest/src/helpers/promise.ts` (8 lines) — Promise utilities

### Core Modules
- `/extensions/terminal-suggest/src/types.ts` — `ICompletionResource` interface (label, documentation, detail, kind)
- `/extensions/terminal-suggest/src/constants.ts` — `TerminalShellType` enum, settings IDs
- `/extensions/terminal-suggest/src/tokens.ts` — Token classification (command, flag, argument, path)
- `/extensions/terminal-suggest/src/upstreamSpecs.ts` — Upstream spec imports/exports

### API Bindings (34 LOC)
- `/extensions/terminal-suggest/src/fig/api-bindings/types.ts` — `EnvironmentVariable`, `ShellContext` interfaces for execution context

### Autocomplete Generators
- `/extensions/terminal-suggest/src/fig/autocomplete/generators/customSuggestionsGenerator.ts` — Custom generator execution
- `/extensions/terminal-suggest/src/fig/autocomplete/generators/scriptSuggestionsGenerator.ts` — Script-based completion generation
- `/extensions/terminal-suggest/src/fig/autocomplete/generators/cache.ts` — Generator result caching
- `/extensions/terminal-suggest/src/fig/autocomplete/generators/helpers.ts` — Generator utilities

### State & Hooks
- `/extensions/terminal-suggest/src/fig/autocomplete/state/generators.ts` — Generator state initialization
- `/extensions/terminal-suggest/src/fig/autocomplete/state/types.ts` — `AutocompleteState`, `Visibility` types
- `/extensions/terminal-suggest/src/fig/autocomplete/fig/hooks.ts` — Fig framework hooks integration

## Tests

**Total: 15 test files (multiple suites)**

### Main Tests
- `/extensions/terminal-suggest/src/test/terminalSuggestMain.test.ts` — End-to-end extension tests with multiple suite specs
- `/extensions/terminal-suggest/src/test/fig.test.ts` — Fig framework generic test suites
- `/extensions/terminal-suggest/src/test/tokens.test.ts` — Token classification tests

### Completion Spec Tests
- `/extensions/terminal-suggest/src/test/completions/cd.test.ts` — cd command completions
- `/extensions/terminal-suggest/src/test/completions/code.test.ts` — VS Code command completions
- `/extensions/terminal-suggest/src/test/completions/code-insiders.test.ts` — VS Code Insiders completions
- `/extensions/terminal-suggest/src/test/completions/git-branch.test.ts` — Git branch completions

### Upstream Command Tests
- `/extensions/terminal-suggest/src/test/completions/upstream/git.test.ts`
- `/extensions/terminal-suggest/src/test/completions/upstream/ls.test.ts`
- `/extensions/terminal-suggest/src/test/completions/upstream/echo.test.ts`
- `/extensions/terminal-suggest/src/test/completions/upstream/mkdir.test.ts`
- `/extensions/terminal-suggest/src/test/completions/upstream/rm.test.ts`
- `/extensions/terminal-suggest/src/test/completions/upstream/rmdir.test.ts`
- `/extensions/terminal-suggest/src/test/completions/upstream/touch.test.ts`

### Environment & Utility Tests
- `/extensions/terminal-suggest/src/test/env/pathExecutableCache.test.ts` — Executable caching tests
- `/extensions/terminal-suggest/src/test/helpers.ts` — Test utilities and fixtures

### Parser Tests
- `/extensions/terminal-suggest/src/fig/shell-parser/test/parser.test.ts`
- `/extensions/terminal-suggest/src/fig/shell-parser/test/command.test.ts`
- `/extensions/terminal-suggest/src/fig/shared/test/utils.test.ts`

### Fixtures
- `/extensions/terminal-suggest/src/test/fixtures/` — Test fixture directory
- `/extensions/terminal-suggest/src/test/fixtures/symlink-test/` — Symlink handling fixtures
- `/extensions/terminal-suggest/fixtures/shell-parser/` — Parser test fixtures with shell inputs and expected outputs

## Types / Interfaces

- `/extensions/terminal-suggest/src/fig/api-bindings/types.ts` — `EnvironmentVariable`, `ShellContext` (shell execution context)
- `/extensions/terminal-suggest/src/fig/autocomplete/state/types.ts` — `AutocompleteState`, `Visibility`, completion state enums
- `/extensions/terminal-suggest/src/fig/shared/test/utils.test.ts` — Test utilities
- `/extensions/terminal-suggest/src/completions/index.d.ts` — Fig.Spec type definitions (imported from vscode.d.ts proposals)
- VSCode API types consumed: `vscode.TerminalCompletionItem`, `vscode.TerminalCompletionItemKind`, `vscode.CompletionItemLabel`

## Configuration

- `/extensions/terminal-suggest/package.json` — Extension manifest with:
  - `terminalCompletionProvider` and `terminalShellEnv` enabled API proposals
  - `terminalSuggestMain` as entry point
  - Activation on `onTerminalShellIntegration:*`
  - Command: `terminal.integrated.suggest.clearCachedGlobals`
  - Attributes: `publisher: vscode`, version `1.0.1`

- `/extensions/terminal-suggest/tsconfig.json` — TypeScript build configuration:
  - Target: Node.js runtime
  - Output: `./out/` → `terminalSuggestMain.js`
  - Includes vscode.d.ts proposals for terminal completion
  - Suppresses strict returns/unused parameters (upstream specs compatibility)

- `/extensions/terminal-suggest/esbuild.mts` — esbuild configuration:
  - Platform: `node`
  - Entry point: `terminalSuggestMain`
  - Output: `dist/`

- `/extensions/terminal-suggest/.vscode/launch.json` — Debug configuration
- `/extensions/terminal-suggest/.vscode/tasks.json` — Build tasks

- `/extensions/terminal-suggest/package-lock.json` — npm lock file
- `/extensions/terminal-suggest/package.nls.json` — Localization strings
- `/extensions/terminal-suggest/cgmanifest.json` — Component governance (third-party licenses, upstream specs)

## Examples / Fixtures

- `/extensions/terminal-suggest/fixtures/shell-parser/basic/` — Basic shell parsing test fixtures
- `/extensions/terminal-suggest/fixtures/shell-parser/variables/` — Variable expansion test cases
- `/extensions/terminal-suggest/fixtures/shell-parser/multipleStatements/` — Multi-statement parsing fixtures
- `/extensions/terminal-suggest/fixtures/shell-parser/primaryExpressions/` — Expression parsing test cases
- Each fixture directory contains `input.sh` and `output.txt` for parser validation

## Documentation

- `/extensions/terminal-suggest/README.md` — Extension documentation
- `/extensions/terminal-suggest/src/fig/README.md` — Explains forked `autocomplete-parser` from AWS Q Developer CLI and Fig tools (MIT licensed)

## Notable Clusters

### Fig Autocomplete Framework (Multi-tier subsystem)
Contains 10+ interdependent modules:
- **Parser layer** (`autocomplete-parser/`, `shell-parser/`): Tokenization and argument parsing
- **Generator layer** (`autocomplete/generators/`): Dynamic completion generation
- **State layer** (`autocomplete/state/`): Completion state machine
- **Shared layer** (`shared/`, `fig-autocomplete-shared/`): Common utilities, spec transformation, error handling

### Completion Specs (115 files, 25+ KLOC)
Organized by:
- **Built-in specs** (14): VS Code, npm ecosystem, git, GitHub, Azure
- **Upstream specs** (100+): POSIX/Unix tools from Fig community

Each spec defines:
- Subcommands and option hierarchies
- Generators for dynamic suggestions (executables, branches, packages)
- Post-processing to transform raw command output into suggestions

### Shell Integration (5 shells supported)
Bash, Zsh, Fish, PowerShell, Windows PowerShell — each with:
- Globals extraction (aliases, environment variables, functions)
- Cached builtin lookups (Zsh 542 LOC, Fish 300 LOC caches)
- Execution harness via `spawnHelper2`

### Test Suite Coverage
- **15 test files** across 3 layers:
  - Unit tests (tokens, parsers, utilities)
  - Integration tests (completion specs, environment)
  - End-to-end tests (full suggestion pipeline)
- **Fixtures** for parser validation with shell script inputs

## Summary

The `terminal-suggest` extension is a sophisticated **terminal completion provider** for VS Code that leverages the **Fig autocomplete framework** (originally from AWS Q Developer CLI). It delivers shell completions across 5 shells (Bash, Zsh, Fish, PowerShell, Windows PowerShell) with:

**Key capabilities:**
- 100+ upstream POSIX command specs + 14 VS Code/npm ecosystem specs
- Dynamic suggestion generation via external command execution (git branches, npm packages, file paths)
- Intelligent argument parsing with state machine-driven completion filtering
- Cross-platform executable discovery with persistent caching (7-day TTL)
- Shell-agnostic suggestion engine with platform-specific execution adapters

**Architecture highlights:**
- Entry point: `terminalSuggestMain.ts` (23 KLOC) registers VS Code completion provider
- Core engine: `figInterface.ts` + `parseArguments.ts` orchestrate spec matching and suggestion generation
- External command execution: Via `spawnHelper2` with timeout management
- Caching layers: Shell globals, generator outputs, executable paths, builtin commands
- Type system: Leverages `Fig.Spec` data structure; outputs `vscode.TerminalCompletionItem` objects

**Porting to Tauri/Rust would require:**
1. **Rewriting the autocomplete parser** (parseArguments.ts 1,188 LOC) from TypeScript to Rust with equivalent AST model
2. **Porting the shell-parser** (shell-parser/ 735+ LOC) for command tokenization
3. **Migrating 115 completion specs** as structured data (JSON/YAML) instead of TypeScript object literals
4. **Translating shell integration** (shell/*.ts, 1,379 LOC) to invoke native shell globals via subprocess
5. **Implementing Tauri command handlers** to replace vscode.window.registerTerminalCompletionProvider IPC
6. **Re-implementing caching** (currently vscode.workspace.fs) using Tauri's filesystem access
7. **Adapting generator execution** to use Tauri's `Command` API instead of Node.js child_process
8. **Maintaining compatibility** with existing completion specs (minimal breaking changes if specs stay JSON)
9. **Testing across platforms** (Windows/macOS/Linux shell variant coverage)

Total effort estimate: **High complexity** due to tight coupling with Node.js runtime (child_process), vscode namespace APIs, and TypeScript-specific patterns in specs. The parser engine and shell integration would be the most effort-intensive components to rewrite from scratch.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
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

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
