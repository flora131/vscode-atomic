# Partition 4 of 79 — Findings

## Scope
`extensions/terminal-suggest/` (197 files, 64,006 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locations for Terminal Suggestion Extension

## Summary
The terminal-suggest extension (197 files, 64,006 LOC) provides terminal autocompletion for VS Code, implementing the `registerTerminalCompletionProvider` API. It supports bash, zsh, fish, and PowerShell shells with completions sourced from Fig specifications and local command definitions.

## Implementation Files

### Core Entry Points
- `src/terminalSuggestMain.ts` - Extension activation and registration of `vscode.window.registerTerminalCompletionProvider` provider

### Type Definitions
- `src/types.ts` - ICompletionResource interface (compatible with vscode.TerminalCompletionItem)
- `src/completions/index.d.ts` - Fig spec type definitions
- `src/constants.ts` - TerminalShellType enum and constants
- `src/tokens.ts` - Token parsing and shell-specific reset character definitions

### Completion Specifications (14 files)
- `src/completions/azd.ts` - Azure Developer CLI completions
- `src/completions/cd.ts` - Directory navigation completions
- `src/completions/code.ts` - VS Code CLI completions
- `src/completions/code-insiders.ts` - VS Code Insiders completions
- `src/completions/code-tunnel.ts` - VS Code Tunnel completions
- `src/completions/code-tunnel-insiders.ts` - VS Code Tunnel Insiders completions
- `src/completions/copilot.ts` - Copilot CLI completions
- `src/completions/git.ts` - Git command completions
- `src/completions/gh.ts` - GitHub CLI completions
- `src/completions/npm.ts` - NPM completions
- `src/completions/npx.ts` - NPX completions
- `src/completions/pnpm.ts` - PNPM completions
- `src/completions/set-location.ts` - PowerShell Set-Location completions
- `src/completions/yarn.ts` - Yarn completions

### Upstream Completions (98 files)
Large collection of standard Unix/Linux command specifications in `src/completions/upstream/`:
- File operations: `cat.ts`, `cp.ts`, `mv.ts`, `rm.ts`, `mkdir.ts`, `rmdir.ts`, `touch.ts`, `ln.ts`
- Directory/path utilities: `cd.ts`, `pwd.ts`, `dirname.ts`, `basename.ts`, `readlink.ts`, `ls.ts`, `find.ts`, `tree.ts`
- Text processing: `grep.ts`, `sed.ts`, `awk.ts`, `cut.ts`, `paste.ts`, `sort.ts`, `uniq.ts`, `wc.ts`, `echo.ts`
- Compression: `tar.ts`, `zip.ts`, `unzip.ts`
- System/process: `ps.ts`, `kill.ts`, `killall.ts`, `pkill.ts`, `top.ts`, `htop.ts`, `lsof.ts`
- Permissions: `chmod.ts`, `chown.ts`, `sudo.ts`, `su.ts`
- User/system info: `whoami.ts`, `who.ts`, `id.ts`, `uname.ts`, `date.ts`
- Network: `ssh.ts`, `scp.ts`, `ping.ts`, `traceroute.ts`, `dig.ts`, `wget.ts`, `curl.ts`
- Disk/storage: `df.ts`, `du.ts`, `mount.ts`, `fdisk.ts`, `lsblk.ts`
- Editors: `vim.ts`, `nano.ts`, `less.ts`, `more.ts`
- Development: `git.ts`, `docker.ts`, `docker-compose.ts`, `npm.ts`, `node.ts`, `python.ts`, `ruby.ts`, `go.ts`, `dotnet.ts`
- Shell: `bash.ts`, `sh.ts`, `export.ts`, `source.ts`
- And 50+ more standard utilities

### Shell Integration (7 files)
- `src/shell/common.ts` - Shared shell utilities
- `src/shell/bash.ts` - Bash shell integration
- `src/shell/zsh.ts` - Zsh shell integration
- `src/shell/fish.ts` - Fish shell integration
- `src/shell/pwsh.ts` - PowerShell integration
- `src/shell/zshBuiltinsCache.ts` - Cached zsh builtins (~142KB)
- `src/shell/fishBuiltinsCache.ts` - Cached fish builtins (~188KB)

### Environment/Caching
- `src/env/pathExecutableCache.ts` - Path executable caching and shell environment handling

### Fig Integration (30+ files in `src/fig/`)
- `src/fig/figInterface.ts` - Main Fig API bindings and suggestion generation
- `src/fig/execute.ts` - Command execution helpers for Fig specs

#### Shell Parser (5 files)
- `src/fig/shell-parser/index.ts` - Shell parsing entry point
- `src/fig/shell-parser/parser.ts` - Command line parser
- `src/fig/shell-parser/command.ts` - Command AST structures
- `src/fig/shell-parser/errors.ts` - Parser error types
- `src/fig/shell-parser/test/` - Parser tests

#### Autocomplete Parser (3 files)
- `src/fig/autocomplete-parser/parseArguments.ts` - Argument parsing logic
- `src/fig/autocomplete-parser/caches.ts` - Parsing caches
- `src/fig/autocomplete-parser/errors.ts` - Error definitions

#### Autocomplete State & Generators (6 files)
- `src/fig/autocomplete/state/types.ts` - State type definitions
- `src/fig/autocomplete/state/generators.ts` - Generator state management
- `src/fig/autocomplete/generators/customSuggestionsGenerator.ts` - Custom suggestion generation
- `src/fig/autocomplete/generators/scriptSuggestionsGenerator.ts` - Script-based suggestions
- `src/fig/autocomplete/generators/cache.ts` - Generator caching
- `src/fig/autocomplete/generators/helpers.ts` - Generator utilities

#### Fig Autocomplete Shared (5 files)
- `src/fig/fig-autocomplete-shared/index.ts` - Main export
- `src/fig/fig-autocomplete-shared/convert.ts` - Spec conversion utilities
- `src/fig/fig-autocomplete-shared/utils.ts` - Shared utilities
- `src/fig/fig-autocomplete-shared/mixins.ts` - Spec mixins
- `src/fig/fig-autocomplete-shared/revert.ts` - Revert functionality
- `src/fig/fig-autocomplete-shared/specMetadata.ts` - Spec metadata handling

#### Shared Utilities (4 files)
- `src/fig/shared/index.ts` - Main export
- `src/fig/shared/utils.ts` - Utility functions
- `src/fig/shared/errors.ts` - Error types
- `src/fig/shared/internal.ts` - Internal utilities

#### API Bindings (1 file)
- `src/fig/api-bindings/types.ts` - Fig API type definitions

### Helpers (8 files)
- `src/helpers/completionItem.ts` - Completion item creation helpers
- `src/helpers/executable.ts` - Executable detection and resolution
- `src/helpers/filepaths.ts` - File path utilities
- `src/helpers/file.ts` - File I/O helpers
- `src/helpers/keyvalue.ts` - Key-value parsing utilities
- `src/helpers/os.ts` - Operating system detection
- `src/helpers/promise.ts` - Promise utilities
- `src/helpers/uri.ts` - URI/path utilities

### Spec Registration
- `src/upstreamSpecs.ts` - Registry of all upstream command specifications

## Test Files (16 files)

### Main Tests
- `src/test/terminalSuggestMain.test.ts` - Extension activation tests
- `src/test/tokens.test.ts` - Token parsing tests
- `src/test/fig.test.ts` - Fig integration tests
- `src/test/helpers.ts` - Test utilities

### Completion Tests (11 files)
- `src/test/completions/cd.test.ts` - CD command tests
- `src/test/completions/code.test.ts` - Code CLI tests
- `src/test/completions/code-insiders.test.ts` - Code Insiders tests
- `src/test/completions/git-branch.test.ts` - Git branch completion tests
- `src/test/completions/upstream/echo.test.ts` - Echo command tests
- `src/test/completions/upstream/git.test.ts` - Git tests
- `src/test/completions/upstream/ls.test.ts` - LS command tests
- `src/test/completions/upstream/mkdir.test.ts` - Mkdir tests
- `src/test/completions/upstream/rmdir.test.ts` - Rmdir tests
- `src/test/completions/upstream/rm.test.ts` - RM command tests
- `src/test/completions/upstream/touch.test.ts` - Touch command tests

### Environment Tests
- `src/test/env/pathExecutableCache.test.ts` - Path cache tests

### Parser Tests (2 files)
- `src/fig/shell-parser/test/parser.test.ts` - Shell parser tests
- `src/fig/shell-parser/test/command.test.ts` - Command AST tests
- `src/fig/shared/test/utils.test.ts` - Shared utility tests

## Configuration Files

- `package.json` - Extension manifest with terminalCompletionProvider API proposal
- `package.nls.json` - Localization strings
- `tsconfig.json` - TypeScript configuration
- `esbuild.mts` - Build configuration
- `.npmrc` - NPM configuration
- `.gitignore` - Git ignore rules
- `.vscodeignore` - Extension packaging rules
- `cgmanifest.json` - Component governance manifest
- `ThirdPartyNotices.txt` - Third-party license notices

## Build & Scripting

### Scripts (5 files)
- `scripts/pullZshBuiltins.ts` - Zsh builtins cache generation
- `scripts/pullFishBuiltins.ts` - Fish builtins cache generation
- `scripts/update-specs.js` - Specification update script
- `scripts/update-specs.ps1` - PowerShell spec update script
- `scripts/update-specs.sh` - Bash spec update script
- `scripts/terminalScriptHelpers.ts` - Script utilities

### VSCode Configuration
- `.vscode/launch.json` - Debug launch configuration
- `.vscode/tasks.json` - Build tasks

## Documentation

- `README.md` - Extension overview (bundled with VS Code, terminal suggestions for zsh/bash/fish/pwsh)
- `src/fig/README.md` - Fig integration notes (autocomplete-parser fork from AWS and withFig)

## Test Fixtures
- `src/test/fixtures/` - Test data directory
- `src/test/fixtures/symlink-test/` - Symlink test fixtures
- `testWorkspace/parent/` - Test workspace structure

## Notable Clusters

### Terminal Shell Support (7 files)
`src/shell/` directory: Modular shell integration for bash, zsh, fish, and PowerShell with cached builtins

### Fig Specification System (30+ files)
`src/fig/` directory: Complete autocomplete specification parsing, parsing, and generation from Fig format (forked from AWS amazon-q-developer-cli and withFig)

### Completion Definitions (112 files total)
14 custom specs + 98 upstream Unix/Linux command specs providing comprehensive CLI autocompletion

### Test Coverage (16 files)
Unit tests across completions, shell integration, parsing, and main extension logic

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Terminal Completion System Patterns
## Porting VS Code Terminal Suggest Extension (TypeScript/Electron → Tauri/Rust)

### Pattern: Provider Registration and Lifecycle

**Where:** `extensions/terminal-suggest/src/terminalSuggestMain.ts:254-320`

**What:** Extension activation hook that registers a terminal completion provider with VS Code API, establishing the core lifecycle and request handling mechanism.

```typescript
export async function activate(context: vscode.ExtensionContext) {
	pathExecutableCache = new PathExecutableCache();
	context.subscriptions.push(pathExecutableCache);
	let currentTerminalEnv: ITerminalEnvironment = process.env;

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
			currentTerminalEnv = terminal.shellIntegration?.env?.value ?? process.env;
			if (token.isCancellationRequested) {
				console.debug('#terminalCompletions token cancellation requested');
				return;
			}

			const shellType: string | undefined = Object.hasOwn(terminal.state, 'shell')
				? (terminal.state.shell as string)
				: undefined;
			const terminalShellType = getTerminalShellType(shellType);
			if (!terminalShellType) {
				console.debug(`#terminalCompletions Shell type ${shellType} not supported`);
				return;
			}

			const commandsInPath = await pathExecutableCache.getExecutablesInPath(
				terminal.shellIntegration?.env?.value,
				terminalShellType
			);
			const shellGlobals = await getShellGlobals(
				terminalShellType,
				commandsInPath?.labels,
				machineId,
				remoteAuthority
			) ?? [];

			if (!commandsInPath?.completionResources) {
				console.debug('#terminalCompletions No commands found in path');
				return;
			}

			const commands = [...shellGlobals, ...commandsInPath.completionResources];
			const currentCommandString = getCurrentCommandAndArgs(
				terminalContext.commandLine,
				terminalContext.cursorIndex,
				terminalShellType
			);

			const result = await Promise.race([
				getCompletionItemsFromSpecs(
					availableSpecs,
					terminalContext,
					commands,
					currentCommandString,
					tokenType,
					terminal.shellIntegration?.cwd,
					getEnvAsRecord(currentTerminalEnv),
					terminal.name,
					token
				),
				createTimeoutPromise(5000, undefined)
			]);

			if (!result) {
				console.debug('#terminalCompletions Timed out fetching completions');
				return;
			}

			return new vscode.TerminalCompletionList(result.items, {
				showFiles: result.showFiles,
				showDirectories: result.showDirectories,
				globPattern: createFileGlobPattern(result.fileExtensions),
				cwd: result.cwd ?? terminal.shellIntegration?.cwd,
			});
		}
	}, '/', '\\'));
}
```

**Variations / call-sites:**
- Provider is registered with separators (`'/', '\\'`) for cross-platform path completion
- Uses `Promise.race()` with 5-second timeout to ensure responsiveness
- Aggregates multiple completion sources: PATH executables, shell builtins, and spec-based completions
- Lifecycle managed through `vscode.ExtensionContext.subscriptions` for proper resource cleanup
- Shell integration environment captured from terminal to preserve execution context
- Cancellation token checked early to respect user interruption requests

---

### Pattern: Completion Resource Data Structure

**Where:** `extensions/terminal-suggest/src/types.ts:8-18`

**What:** Core interface representing completion items with metadata including labels, documentation, and visual hints.

```typescript
export interface ICompletionResource {
	label: string | vscode.CompletionItemLabel;
	/**
	 * The definition command of the completion, this will be the resolved value of an alias
	 * completion.
	 */
	definitionCommand?: string;
	documentation?: string | vscode.MarkdownString;
	detail?: string;
	kind?: vscode.TerminalCompletionItemKind;
}
```

**Variations / call-sites:**
- `label` can be simple string or compound `CompletionItemLabel` with description
- `definitionCommand` tracks actual command when completion is an alias (e.g., alias resolution)
- `documentation` supports both plain text and formatted markdown
- `kind` determines icon and category (Method, Argument, Folder, etc.)
- Used throughout: shell builtins, PATH executables, spec generators, git branches/commits

---

### Pattern: Shell Executable Discovery and Caching

**Where:** `extensions/terminal-suggest/src/env/pathExecutableCache.ts:21-150`

**What:** Multi-tier caching system for discovering executables in PATH directories with file system watcher integration.

```typescript
export class PathExecutableCache implements vscode.Disposable {
	private _disposables: vscode.Disposable[] = [];
	private readonly _windowsExecutableExtensionsCache: WindowsExecutableExtensionsCache | undefined;
	private _cachedExes: Map<string, Set<ICompletionResource> | undefined> = new Map();
	private _inProgressRequest: {
		env: ITerminalEnvironment;
		shellType: TerminalShellType | undefined;
		promise: Promise<IExecutablesInPath | undefined>;
	} | undefined;

	async getExecutablesInPath(
		env: ITerminalEnvironment = process.env,
		shellType?: TerminalShellType
	): Promise<IExecutablesInPath | undefined> {
		if (this._inProgressRequest &&
			this._inProgressRequest.env === env &&
			this._inProgressRequest.shellType === shellType
		) {
			return this._inProgressRequest.promise;
		}

		const promise = this._doGetExecutablesInPath(env, shellType);
		this._inProgressRequest = { env, shellType, promise };
		await promise;
		this._inProgressRequest = undefined;
		return promise;
	}

	private async _doGetExecutablesInPath(
		env: ITerminalEnvironment,
		shellType?: TerminalShellType
	): Promise<IExecutablesInPath | undefined> {
		const paths = pathValue.split(isWindows ? ';' : ':');
		const pathSeparator = isWindows ? '\\' : '/';
		const promisePaths: string[] = [];
		const promises: Promise<Set<ICompletionResource> | undefined>[] = [];
		const labels: Set<string> = new Set<string>();

		for (const pathDir of paths) {
			const cachedExecutables = this._cachedExes.get(pathDir);
			if (cachedExecutables) {
				for (const executable of cachedExecutables) {
					const labelText = typeof executable.label === 'string'
						? executable.label
						: executable.label.label;
					labels.add(labelText);
				}
			} else {
				promisePaths.push(pathDir);
				promises.push(this._getExecutablesInSinglePath(pathDir, pathSeparator, labels));
			}
		}

		if (promises.length > 0) {
			const resultSets = await Promise.all(promises);
			for (const [i, resultSet] of resultSets.entries()) {
				const pathDir = promisePaths[i];
				if (!this._cachedExes.has(pathDir)) {
					this._cachedExes.set(pathDir, resultSet || new Set());
				}
			}
		}

		const executables = new Set<ICompletionResource>();
		const processedPaths: Set<string> = new Set();
		for (const pathDir of paths) {
			if (processedPaths.has(pathDir)) {
				continue;
			}
			processedPaths.add(pathDir);
			const dirExecutables = this._cachedExes.get(pathDir);
			if (dirExecutables) {
				for (const executable of dirExecutables) {
					executables.add(executable);
				}
			}
		}

		return { completionResources: executables, labels };
	}
}
```

**Variations / call-sites:**
- Per-directory caching with deduplication detection
- Parallel directory scanning with `Promise.all()`
- In-flight request deduplication to prevent redundant concurrent scans
- File type filtering (skips directories, symlinks resolved to real paths)
- Windows executable extension support via `WindowsExecutableExtensionsCache`
- Watched via file system watcher for real-time invalidation (line 328-376)

---

### Pattern: Spec-based Completion Generation with Generators

**Where:** `extensions/terminal-suggest/src/completions/git.ts:162-214`

**What:** Declarative completion specification using generators with post-processing for dynamic command output parsing.

```typescript
export const gitGenerators = {
	// Commit history
	commits: {
		script: ["git", "--no-optional-locks", "log", "--oneline", "-n", "1000"],
		postProcess: function (out) {
			const output = filterMessages(out);
			if (output.startsWith("fatal:")) {
				return [];
			}

			const lines = output.split("\n");
			const firstLine = lines.length > 0 ? lines[0] : undefined;
			const hashLength = firstLine && firstLine.length > 0
				? firstLine.indexOf(" ")
				: 7;
			const descriptionStart = hashLength + 1;

			return lines.map((line) => {
				return {
					name: line.substring(0, hashLength),
					icon: `vscode://icon?type=${vscode.TerminalCompletionItemKind.ScmCommit}`,
					description: line.substring(descriptionStart),
				};
			});
		},
	} satisfies Fig.Generator,

	// user aliases
	aliases: {
		script: ["git", "--no-optional-locks", "config", "--get-regexp", "^alias."],
		cache: {
			strategy: "stale-while-revalidate",
		},
		postProcess: (out) => {
			const suggestions = out.split("\n").map((aliasLine) => {
				const [name, ...parts] = aliasLine.slice("alias.".length).split(" ");
				const value = parts.join(" ");
				return {
					name,
					description: `Alias for '${value}'`,
					icon: "fig://icon?type=commandkey",
				};
			});
			const seen = new Set();
			return suggestions.filter((suggestion) => {
				if (seen.has(suggestion.name)) {
					return false;
				}
				seen.add(suggestion.name);
				return true;
			});
		},
	} satisfies Fig.Generator,

	stashes: {
		script: ["git", "--no-optional-locks", "stash", "list"],
		postProcess: function (out) {
			const output = filterMessages(out);
			if (output.startsWith("fatal:")) {
				return [];
			}

			return output.split("\n").map((file) => {
				return {
					name: file.split(":").slice(2).join(":"),
					icon: `vscode://icon?type=${vscode.TerminalCompletionItemKind.ScmStash}`,
				};
			});
		},
	} satisfies Fig.Generator,
};
```

**Variations / call-sites:**
- `script`: Command + args array executed to get completions
- `postProcess`: Function to transform raw output into suggestions
- `custom`: Async function with executeShellCommand for conditional logic (line 304-338)
- `cache`: Strategy for stale-while-revalidate pattern
- Error handling for "fatal:" git errors (returns empty list)
- Deduplication using Set to filter repeated suggestions
- Icon URL patterns: `vscode://icon?type=<Kind>` or `fig://icon?type=<type>`

---

### Pattern: Completion Item Creation with Replacement Range

**Where:** `extensions/terminal-suggest/src/helpers/completionItem.ts:9-19`

**What:** Factory function computing replacement range for completion insertion based on cursor position.

```typescript
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

**Variations / call-sites:**
- Replacement range calculated from cursor position minus last word length
- Handles space-termination case (no word replacement)
- Detail and documentation can override resource defaults
- Kind defaults to Method if not specified
- Used throughout: spec completions, command suggestions, path components

---

### Pattern: Shell-specific Globals Extraction

**Where:** `extensions/terminal-suggest/src/shell/bash.ts:11-68`

**What:** Shell-specific implementation for discovering and documenting builtins with help text extraction.

```typescript
export async function getBashGlobals(
	options: ExecOptionsWithStringEncoding,
	existingCommands?: Set<string>
): Promise<(string | ICompletionResource)[]> {
	return [
		...await getAliases(options),
		...await getBuiltins(options, 'compgen -b', existingCommands)
	];
}

export async function getBuiltins(
	options: ExecOptionsWithStringEncoding,
	scriptToRun: string,
	existingCommands?: Set<string>,
): Promise<(string | ICompletionResource)[]> {
	const compgenOutput = await execHelper(scriptToRun, options);
	const filter = (cmd: string) => cmd && !existingCommands?.has(cmd);
	const builtins: string[] = compgenOutput.split('\n').filter(filter);
	const completions: ICompletionResource[] = [];
	if (builtins.find(r => r === '.')) {
		completions.push({
			label: '.',
			detail: 'Source a file in the current shell',
			kind: vscode.TerminalCompletionItemKind.Method
		});
	}

	for (const cmd of builtins) {
		if (typeof cmd === 'string') {
			try {
				const helpOutput = (await execHelper(`help ${cmd}`, options))?.trim();
				const helpLines = helpOutput?.split('\n');
				const outputDescription = helpLines?.splice(1)
					.map(line => line.trim())
					.join('');
				const args = helpLines?.[0]?.split(' ').slice(1).join(' ').trim();
				const { detail, documentation, description } = generateDetailAndDocs(
					outputDescription,
					args
				);
				completions.push({
					label: { label: cmd, description },
					detail,
					documentation: new vscode.MarkdownString(documentation),
					kind: vscode.TerminalCompletionItemKind.Method
				});
			} catch (e) {
				completions.push({
					label: cmd,
					kind: vscode.TerminalCompletionItemKind.Method
				});
			}
		}
	}

	return completions;
}

export function generateDetailAndDocs(
	description?: string,
	args?: string
): { detail?: string; documentation?: string; description?: string } {
	let detail, documentation = '';
	const firstSentence = (text: string): string => text.split('. ')[0] + '.';
	if (description) {
		description = firstSentence(description);
		detail = args;
		documentation = description;
	}
	return { detail, documentation, description };
}
```

**Variations / call-sites:**
- Shell-specific variants for: Bash, Zsh, Fish, PowerShell (`getBashGlobals`, `getZshGlobals`, etc.)
- Uses `compgen -b` for bash builtins, shell equivalents for others
- Deduplicates against already-discovered PATH commands via `existingCommands` set
- Extracts help text per-builtin: `help <cmd>` (bash), `<cmd> --help` (others)
- Formats help output: first sentence as documentation, args as detail
- Graceful fallback: includes builtin without docs if help extraction fails

---

### Pattern: Token Type Detection for Command vs Argument

**Where:** `extensions/terminal-suggest/src/tokens.ts:9-50`

**What:** Shell-specific reset character definitions to differentiate command positions from argument positions.

```typescript
export const enum TokenType {
	Command,
	Argument,
}

export const shellTypeResetChars = new Map<TerminalShellType, string[]>([
	[TerminalShellType.Bash, ['>', '>>', '<', '2>', '2>>', '&>', '&>>', '|', '|&', '&&', '||', '&', ';', '(', '{', '<<']],
	[TerminalShellType.Zsh, ['>', '>>', '<', '2>', '2>>', '&>', '&>>', '<>', '|', '|&', '&&', '||', '&', ';', '(', '{', '<<', '<<<', '<(']],
	[TerminalShellType.PowerShell, ['>', '>>', '<', '2>', '2>>', '*>', '*>>', '|', ';', ' -and ', ' -or ', ' -not ', '!', '&', ' -eq ', ' -ne ', ' -gt ', ' -lt ', ' -ge ', ' -le ', ' -like ', ' -notlike ', ' -match ', ' -notmatch ', ' -contains ', ' -notcontains ', ' -in ', ' -notin ']]
]);

export const defaultShellTypeResetChars = shellTypeResetChars.get(TerminalShellType.Bash)!;

export function getTokenType(
	ctx: { commandLine: string; cursorIndex: number },
	shellType: TerminalShellType | undefined
): TokenType {
	const commandLine = ctx.commandLine;
	const cursorPosition = ctx.cursorIndex;
	const commandResetChars = shellType === undefined
		? defaultShellTypeResetChars
		: shellTypeResetChars.get(shellType) ?? defaultShellTypeResetChars;

	const beforeCursor = commandLine.substring(0, cursorPosition);
	const wordStart = beforeCursor.lastIndexOf(' ') + 1;
	const beforeWord = commandLine.substring(0, wordStart);

	for (const resetChar of commandResetChars) {
		const pattern = shellType === TerminalShellType.PowerShell
			? `${resetChar}`
			: ` ${resetChar} `;
		if (beforeWord.endsWith(pattern)) {
			return TokenType.Command;
		}
	}

	const spaceIndex = beforeCursor.lastIndexOf(' ');
	if (spaceIndex === -1) {
		return TokenType.Command;
	}
	const previousTokens = beforeCursor.substring(0, spaceIndex + 1).trim();
	if (commandResetChars.some(e => previousTokens.endsWith(e))) {
		return TokenType.Command;
	}
	return TokenType.Argument;
}
```

**Variations / call-sites:**
- Bash/Zsh: pipelines (`|`, `|&`), redirects (`>`, `>>`, `<`), logical operators (`&&`, `||`)
- PowerShell: named operators (`-and`, `-or`, `-eq`, `-like`) without surrounding spaces
- Used to determine completion behavior: command-level specs vs argument generators
- Git bash uses bash character set despite Windows platform
- Falls back to Bash reset chars for unsupported shells

---

### Pattern: Promise Race with Timeout for Responsiveness

**Where:** `extensions/terminal-suggest/src/terminalSuggestMain.ts:281-294`

**What:** Ensures completion UI remains responsive by timing out long-running operations.

```typescript
const result = await Promise.race([
	getCompletionItemsFromSpecs(
		availableSpecs,
		terminalContext,
		commands,
		currentCommandString,
		tokenType,
		terminal.shellIntegration?.cwd,
		getEnvAsRecord(currentTerminalEnv),
		terminal.name,
		token
	),
	createTimeoutPromise(5000, undefined)
]);

if (!result) {
	console.debug('#terminalCompletions Timed out fetching completions from specs');
	return;
}
```

**Variations / call-sites:**
- 5-second timeout for interactive responsiveness
- Returns undefined on timeout rather than partial/stale results
- Integrates with cancellation token for explicit user interruption
- Primary alternative: streaming/incremental results (not implemented here)
- Used in multiple contexts: shell globals fetch, PATH scanning, spec-based suggestions

---

### Pattern: Aggregating Multiple Completion Sources

**Where:** `extensions/terminal-suggest/src/terminalSuggestMain.ts:469-565`

**What:** Combines completions from multiple sources with deduplication and ordering.

```typescript
export async function getCompletionItemsFromSpecs(
	specs: Fig.Spec[],
	terminalContext: vscode.TerminalCompletionContext,
	availableCommands: ICompletionResource[],
	currentCommandString: string,
	tokenType: TokenType,
	shellIntegrationCwd: vscode.Uri | undefined,
	env: Record<string, string>,
	name: string,
	token?: vscode.CancellationToken,
	executeExternals?: IFigExecuteExternals,
): Promise<{ items: vscode.TerminalCompletionItem[]; showFiles: boolean; showDirectories: boolean; fileExtensions?: string[]; cwd?: vscode.Uri }> {
	let items: vscode.TerminalCompletionItem[] = [];
	let showFiles = false;
	let showDirectories = false;
	let hasCurrentArg = false;
	let fileExtensions: string[] | undefined;

	// Get spec-based completions
	const result = await getFigSuggestions(
		specs,
		terminalContext,
		availableCommands,
		currentCommandString,
		tokenType,
		shellIntegrationCwd,
		env,
		name,
		executeExternalsWithFallback,
		token
	);

	if (result) {
		hasCurrentArg ||= result.hasCurrentArg;
		showFiles ||= result.showFiles;
		showDirectories ||= result.showDirectories;
		fileExtensions = result.fileExtensions;
		if (result.items) {
			items = items.concat(result.items);
		}
	}

	if (tokenType === TokenType.Command) {
		// Include builtin/available commands in the results
		const labels = new Set(items.map((i) => typeof i.label === 'string' ? i.label : i.label.label));
		for (const command of availableCommands) {
			const commandTextLabel = typeof command.label === 'string' ? command.label : command.label.label;
			const labelWithoutExtension = isWindows ? commandTextLabel.replace(/\.[^ ]+$/, '') : commandTextLabel;
			if (!labels.has(labelWithoutExtension)) {
				items.push(createCompletionItem(
					terminalContext.cursorIndex,
					currentCommandString,
					command,
					command.detail,
					command.documentation,
					vscode.TerminalCompletionItemKind.Method
				));
				labels.add(commandTextLabel);
			}
			else {
				const existingItem = items.find(i => (typeof i.label === 'string' ? i.label : i.label.label) === commandTextLabel);
				if (!existingItem) {
					continue;
				}
				existingItem.documentation ??= command.documentation;
				existingItem.detail ??= command.detail;
			}
		}
		showFiles = true;
		showDirectories = true;
	} else if (!items.length && !showFiles && !showDirectories && !hasCurrentArg) {
		showFiles = true;
		showDirectories = true;
	}

	let cwd: vscode.Uri | undefined;
	if (shellIntegrationCwd && (showFiles || showDirectories)) {
		cwd = await resolveCwdFromCurrentCommandString(currentCommandString, shellIntegrationCwd);
	}

	return { items, showFiles, showDirectories, fileExtensions, cwd };
}
```

**Variations / call-sites:**
- Prioritizes spec-based completions (first added to items array)
- Adds PATH/builtin commands if not duplicated (deduplication via label set)
- For command position: enriches with PATH commands + enables file/folder suggestions
- For argument position: falls back to file/folder only if specs provide nothing
- Windows extension stripping in deduplication logic
- Merges documentation/detail from PATH commands into spec items when no description exists

---

### Pattern: Test Harness for Spec Validation

**Where:** `extensions/terminal-suggest/src/test/terminalSuggestMain.test.ts:27-77`

**What:** Generic test framework for validating spec completions across multiple scenarios.

```typescript
suite('Terminal Suggest', () => {
	for (const suiteSpec of testSpecs2) {
		suite(suiteSpec.name, () => {
			for (const testSpec of suiteSpec.testSpecs) {
				test(`typing "${testSpec.input.replace(/\|/g, '|')}"`, async () => {
					const cursorIndex = testSpec.input.indexOf('|');
					const input = testSpec.input.replace(/\|/g, '');

					const context: vscode.TerminalCompletionContext = {
						commandLine: input,
						cursorIndex: cursorIndex,
					};

					const result = await getCompletionItemsFromSpecs(
						suiteSpec.completionSpecs,
						context,
						suiteSpec.availableCommands.map(cmd => ({ label: cmd })),
						getCurrentCommandAndArgs(input, cursorIndex, undefined),
						getTokenType(context, undefined),
						vscode.Uri.file(testPaths.cwd),
						{},
						'test-terminal',
						undefined,
						mockExecuteExternals
					);

					const labels = result.items.map(item => typeof item.label === 'string' ? item.label : item.label.label);
					deepStrictEqual(labels, testSpec.expectedCompletions.map(c => c.label));

					if (testSpec.expectedResourceRequests) {
						// Verify file/folder request expectations
						if (testSpec.expectedResourceRequests.type === 'files') {
							strictEqual(result.showFiles, true);
							strictEqual(result.showDirectories, false);
						} else if (testSpec.expectedResourceRequests.type === 'folders') {
							strictEqual(result.showFiles, false);
							strictEqual(result.showDirectories, true);
						} else if (testSpec.expectedResourceRequests.type === 'both') {
							strictEqual(result.showFiles, true);
							strictEqual(result.showDirectories, true);
						}
					}
				});
			}
		});
	}
});
```

**Variations / call-sites:**
- Uses `|` as cursor position marker in input strings
- Tests both command and argument position completions
- Validates file/folder suggestion flags
- Specific test suites for git, cd, code, npm, upstream commands
- Mock execute externals for controlled testing
- Assertions on completion labels, kinds, and resource request types

---

## Summary

The terminal-suggest extension implements a sophisticated completion system centered on:

1. **Provider Registration**: VS Code API integration for async completion delivery with timeout protection
2. **Multi-source Aggregation**: PATH executables, shell builtins, and spec-based completions combined with deduplication
3. **Lazy Caching**: Per-directory PATH scanning with in-flight request deduplication and file system watchers
4. **Declarative Specs**: Fig format generators (script, postProcess, custom) for dynamic command completion
5. **Shell Awareness**: Token type detection and shell-specific builtins/aliases discovery
6. **Responsive UI**: Promise.race with 5-second timeout, cancellation token support
7. **Comprehensive Testing**: Generic test harness validating specs across scenarios

Key cross-cutting patterns suitable for Rust/Tauri porting:
- Async request handling with explicit cancellation
- Multi-tier in-memory caching with external storage
- Declarative generator system for extensibility
- Shell process execution with timeout/error handling
- Deduplication strategies for suggestion aggregation
- File system watching integration

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
