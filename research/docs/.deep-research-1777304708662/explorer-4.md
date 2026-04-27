# Partition 4 of 79 — Findings

## Scope
`extensions/terminal-suggest/` (197 files, 64,005 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Terminal Suggest Extension - File Locations for Tauri/Rust Port Analysis

**Partition 4 of 79**: `extensions/terminal-suggest/` (197 files, 64,005 LOC)

---

## Implementation

### Core Extension Entry
- `extensions/terminal-suggest/src/terminalSuggestMain.ts` - Extension activation and `registerTerminalCompletionProvider` API usage; TerminalShellType enum (Bash, Fish, Zsh, PowerShell, WindowsPowerShell, GitBash); shell-specific globals caching
- `extensions/terminal-suggest/src/types.ts` - ICompletionResource interface defining label, documentation, detail, kind
- `extensions/terminal-suggest/src/constants.ts` - SettingsIds enum for terminal.integrated.suggest configuration
- `extensions/terminal-suggest/src/tokens.ts` - TokenType enum; shell-specific reset character definitions; getTokenType() for command vs argument context

### Shell Integration & Execution
- `extensions/terminal-suggest/src/env/pathExecutableCache.ts` - ITerminalEnvironment interface; path-based executable discovery and caching
- `extensions/terminal-suggest/src/shell/bash.ts` - getBashGlobals() builtin extraction
- `extensions/terminal-suggest/src/shell/zsh.ts` - getZshGlobals() zsh-specific builtins
- `extensions/terminal-suggest/src/shell/fish.ts` - getFishGlobals() fish shell integration
- `extensions/terminal-suggest/src/shell/pwsh.ts` - getPwshGlobals() PowerShell enumeration
- `extensions/terminal-suggest/src/shell/common.ts` - Shared shell utilities
- `extensions/terminal-suggest/src/shell/zshBuiltinsCache.ts`
- `extensions/terminal-suggest/src/shell/fishBuiltinsCache.ts`

### Fig Spec System
- `extensions/terminal-suggest/src/fig/figInterface.ts` - getFigSuggestions() entry point; IFigSpecSuggestionsResult interface; spec label resolution
- `extensions/terminal-suggest/src/fig/execute.ts` - executeCommand(), executeCommandTimeout(); IFigExecuteExternals interface
- `extensions/terminal-suggest/src/fig/shell-parser/index.ts`
- `extensions/terminal-suggest/src/fig/shell-parser/parser.ts` - Command tokenization and parsing
- `extensions/terminal-suggest/src/fig/shell-parser/command.ts` - Command AST representation
- `extensions/terminal-suggest/src/fig/shell-parser/errors.ts`
- `extensions/terminal-suggest/src/fig/autocomplete-parser/parseArguments.ts` - ArgumentParserResult type
- `extensions/terminal-suggest/src/fig/autocomplete/state/types.ts` - AutocompleteState interface; Visibility enum
- `extensions/terminal-suggest/src/fig/autocomplete/state/generators.ts` - createGeneratorState()
- `extensions/terminal-suggest/src/fig/autocomplete/generators/customSuggestionsGenerator.ts`
- `extensions/terminal-suggest/src/fig/autocomplete/generators/scriptSuggestionsGenerator.ts`
- `extensions/terminal-suggest/src/fig/autocomplete/generators/helpers.ts`
- `extensions/terminal-suggest/src/fig/autocomplete/generators/cache.ts`

### Command Specifications (84 files)
- `extensions/terminal-suggest/src/completions/index.d.ts` - Fig.Spec type definition
- **Custom specs (14 files)**: `extensions/terminal-suggest/src/completions/{azd,cd,code,code-insiders,code-tunnel,code-tunnel-insiders,copilot,gh,git,npm,npx,pnpm,set-location,yarn}.ts`
- `extensions/terminal-suggest/src/upstreamSpecs.ts` - Re-exports 70+ upstream specs
- **Upstream specs (70 files)**: `extensions/terminal-suggest/src/completions/upstream/{adb,apt,basename,brew,bundle,cat,chmod,chown,clear,cp,curl,cut,date,dd,df,diff,dig,dirname,docker,docker-compose,dotnet,du,echo,env,export,fdisk,find,fmt,fold,go,grep,head,htop,id,jq,kill,killall,less,ln,ls,lsblk,lsof,mkdir,more,mount,mv,nano,nl,node,nvm,od,paste,ping,pkill,ps,pwd,python,python3,readlink,rm,rmdir,rsync,ruby,ruff,scp,sed,seq,shred,sort,source,split,ssh,stat,su,sudo,tac,tail,tar,tee,time,top,touch,tr,traceroute,tree,truncate,...}.ts`

### Shared Libraries & Utilities
- `extensions/terminal-suggest/src/fig/fig-autocomplete-shared/{index,convert,revert,mixins,utils,specMetadata}.ts`
- `extensions/terminal-suggest/src/fig/shared/{index,utils,internal,errors}.ts`
- `extensions/terminal-suggest/src/fig/api-bindings/types.ts` - EnvironmentVariable interface
- `extensions/terminal-suggest/src/fig/autocomplete/fig/hooks.ts` - FigState interface
- `extensions/terminal-suggest/src/fig/autocomplete-parser/{errors,caches}.ts`
- **Helpers (8 files)**: `extensions/terminal-suggest/src/helpers/{completionItem,executable,file,filepaths,keyvalue,os,promise,uri}.ts`

---

## Tests

- `extensions/terminal-suggest/src/test/terminalSuggestMain.test.ts` - Main extension tests
- `extensions/terminal-suggest/src/test/tokens.test.ts` - Token classification tests
- `extensions/terminal-suggest/src/test/fig.test.ts` - Fig integration tests
- `extensions/terminal-suggest/src/test/env/pathExecutableCache.test.ts` - Executable cache tests
- `extensions/terminal-suggest/src/test/helpers.ts` - Test utilities
- **Completion specs tests (11 files)**: `extensions/terminal-suggest/src/test/completions/{cd,code,code-insiders,git-branch}.test.ts` and `extensions/terminal-suggest/src/test/completions/upstream/{echo,git,ls,mkdir,rm,rmdir,touch}.test.ts`
- **Parser tests (3 files)**: `extensions/terminal-suggest/src/fig/{shell-parser/test/{parser,command},shared/test/utils}.test.ts`

---

## Types / Interfaces

### Terminal API Surface (Critical for Tauri Port)
- `vscode.window.registerTerminalCompletionProvider()` - Provider registration API
- `vscode.Terminal` - Terminal object with `state.shell` property detection
- `vscode.TerminalShellIntegration` - `env.value`, `cwd` properties
- `vscode.TerminalCompletionContext` - `commandLine`, `cursorIndex` context
- `vscode.TerminalCompletionItem` - Individual completion items
- `vscode.TerminalCompletionItemKind` - Item classification
- `vscode.TerminalCompletionList` - Batch completion results
- `vscode.CancellationToken` - Request cancellation

### Shell Types
- `TerminalShellType` enum (6 variants): `bash`, `zsh`, `fish`, `pwsh`, `powershell`, `gitbash`

### Fig Domain Types
- `Fig.Spec` - Command specification format
- `AutocompleteState` - Suggestion generation context
- `FigState` - Fig hook state
- `Command` - Shell command AST
- `ArgumentParserResult` - Argument parsing output
- `ICompletionResource` - Unified completion item format

---

## Configuration

- `extensions/terminal-suggest/package.json` - enabledApiProposals: `terminalCompletionProvider`, `terminalShellEnv`; activationEvents: `onTerminalShellIntegration:*`; main entry: `out/terminalSuggestMain`
- `extensions/terminal-suggest/tsconfig.json`
- `extensions/terminal-suggest/.vscode/{launch,tasks}.json` - Debug configuration
- `extensions/terminal-suggest/cgmanifest.json` - Manifest
- `extensions/terminal-suggest/package.nls.json` - Localization
- **Build scripts**: `extensions/terminal-suggest/scripts/{update-specs,pullZshBuiltins,pullFishBuiltins,terminalScriptHelpers}.{js,ts}`

---

## Documentation

- `extensions/terminal-suggest/README.md` - Feature summary for zsh, bash, fish, pwsh completion support
- `extensions/terminal-suggest/src/fig/README.md` - Fig autocomplete system documentation

---

## Notable Clusters

### Terminal API Surface (Porting Dependency)
The extension directly exercises these VS Code terminal APIs essential for any Tauri port:
- Provider registration via `vscode.window.registerTerminalCompletionProvider()`
- Shell type detection from `terminal.state.shell` (must support at least 6 shell variants)
- Environment access via `terminal.shellIntegration?.env?.value`
- Working directory via `terminal.shellIntegration?.cwd`
- Command line context: `commandLine` string and `cursorIndex` position
- Completion item kinds (`vscode.TerminalCompletionItemKind`) for semantic classification

### Shell Integration Points
- **Shell detection**: Bash (including Git Bash), Zsh, Fish, PowerShell, Windows PowerShell
- **Builtin command extraction**: Shell-specific commands discovered via `declare -F`, `builtin`, `function` enumeration
- **Environment variables**: Extracted from terminal's shell integration layer
- **Path-based discovery**: Executable files enumerated from PATH with shell-specific filtering (Windows extensions)

### Completion Spec System
The extension uses a declarative command specification system with 84 total specs (14 custom + 70 upstream):
- Custom specs for VS Code-specific commands (`code`, `code-tunnel`, `code-insiders`, `code-tunnel-insiders`)
- Package managers: npm, pnpm, yarn, npx
- VCS: git, gh (GitHub CLI)
- Dev tools: docker, dotnet, go, ruby, python, node, nvm, ruff
- Unix utilities: 50+ standard commands (ls, rm, cp, find, grep, tar, etc.)
- Specs define subcommands, options, arguments with validation and completion generators

### Caching Architecture
- **GlobalStorage persistence**: 7-day TTL cache for shell builtins per (machineId, remoteAuthority, shellType)
- **In-flight deduplication**: Multiple concurrent requests for same cache key reuse promise
- **Path executable cache**: Deduplicates file system lookups with shell-specific executable detection
- **Background refresh disabled**: Issue #259343 - process spawning blocked extension host on Windows

### Parser Chain
The completion flow chains these parsers:
1. **Shell parser**: Tokenizes command line into Command AST respecting shell-specific operators
2. **Argument parser**: Parses flags and positional arguments into ArgumentParserResult
3. **Spec matcher**: Finds applicable Fig.Spec from available commands list
4. **Generator chain**: Invokes custom suggestion generators (custom, script-based) for context-aware completions
5. **Result assembly**: Creates vscode.TerminalCompletionItem[] with proper kinds and formatting

The extension demonstrates that a Tauri/Rust port must provide equivalent APIs for terminal integration, shell detection, environment access, and asynchronous completion suggestion. The 182 TypeScript files represent significant domain logic for shell command completion that cannot simply be ported mechanically—shell-specific behaviors (reset characters, builtin enumeration, argument parsing) require deep shell knowledge.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Terminal API Surface Patterns - Partition 4

## Research Focus
Porting VS Code core from TypeScript/Electron to Tauri/Rust requires understanding the terminal completion provider API surface and shell abstraction patterns used in the terminal-suggest extension.

---

#### Pattern: Terminal Completion Provider Registration

**Where:** `extensions/terminal-suggest/src/terminalSuggestMain.ts:261-327`

**What:** Core API for registering a terminal completion provider with shell-specific separator handling.

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
		// Fetch completions based on context
		const result = await Promise.race([
			getCompletionItemsFromSpecs(...),
			createTimeoutPromise(5000, undefined)
		]);
		if (!result) {
			return;
		}
		return result.items;
	}
}, '/', '\\'));
```

**Variations:**
- Returns either `TerminalCompletionItem[]` or `TerminalCompletionList` with file glob options
- Supports race condition handling with timeout (5000ms)
- Respects cancellation tokens throughout async execution
- Registers with forward and backward slash separators

---

#### Pattern: Shell Type Enumeration and Detection

**Where:** `extensions/terminal-suggest/src/terminalSuggestMain.ts:38-45, 587-602`

**What:** Enumerated shell types and lookup function that maps shell strings to typed constants.

```typescript
export const enum TerminalShellType {
	Bash = 'bash',
	Fish = 'fish',
	Zsh = 'zsh',
	PowerShell = 'pwsh',
	WindowsPowerShell = 'powershell',
	GitBash = 'gitbash',
}

function getTerminalShellType(shellType: string | undefined): TerminalShellType | undefined {
	switch (shellType) {
		case 'bash':
			return TerminalShellType.Bash;
		case 'gitbash':
			return TerminalShellType.GitBash;
		case 'zsh':
			return TerminalShellType.Zsh;
		case 'pwsh':
			return basename(vscode.env.shell, '.exe') === 'powershell'
				? TerminalShellType.WindowsPowerShell
				: TerminalShellType.PowerShell;
		case 'fish':
			return TerminalShellType.Fish;
		default:
			return undefined;
	}
}
```

**Variations:**
- PowerShell detection uses OS-specific environment to differentiate `pwsh` vs `powershell`
- Git Bash has special handling (treated as Bash for execution but tracked separately)
- Shell type extracted from `terminal.state.shell` property

---

#### Pattern: Token Type Determination by Shell

**Where:** `extensions/terminal-suggest/src/tokens.ts:14-50`

**What:** Shell-specific reset character patterns for parsing command vs argument context.

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

export function getTokenType(ctx: { commandLine: string; cursorIndex: number }, shellType: TerminalShellType | undefined): TokenType {
	const resetChars = shellType ? shellTypeResetChars.get(shellType) ?? defaultShellTypeResetChars : defaultShellTypeResetChars;
	const beforeWord = commandLine.substring(0, wordStart);
	for (const resetChar of resetChars) {
		const pattern = shellType === TerminalShellType.PowerShell ? `${resetChar}` : ` ${resetChar} `;
		if (beforeWord.endsWith(pattern)) {
			return TokenType.Command;
		}
	}
	return TokenType.Argument;
}
```

**Variations:**
- PowerShell reset patterns include operators with spaces (` -and `, ` -or `)
- Bash/Zsh share most operators but Zsh adds `<>`, `<<<`, `<(`
- Pattern matching differs: PowerShell tests bare operator, others test with surrounding spaces

---

#### Pattern: Completion Resource Interface

**Where:** `extensions/terminal-suggest/src/types.ts:8-18`

**What:** Unified resource type for all completion items from various sources (shell globals, specs, path executables).

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

**Variations:**
- Labels can be simple strings or rich `CompletionItemLabel` objects with descriptions
- Kind maps to VS Code's `TerminalCompletionItemKind` enum (Method, Alias, Folder, ScmBranch, etc.)
- Documentation supports both plain strings and markdown formatting

---

#### Pattern: Shell-Specific Globals Caching with Invalidation

**Where:** `extensions/terminal-suggest/src/terminalSuggestMain.ts:93-133, 135-200`

**What:** Multi-level caching strategy with machine ID, remote authority, and TTL-based invalidation.

```typescript
type ShellGlobalsCacheEntryWithMeta = ShellGlobalsCacheEntry & { timestamp: number };
const cachedGlobals: Map<string, ShellGlobalsCacheEntryWithMeta> = new Map();
const inflightRequests: Map<string, Promise<ICompletionResource[] | undefined>> = new Map();
const CACHE_MAX_AGE_MS = 1000 * 60 * 60 * 24 * 7; // 7 days

function getCacheKey(machineId: string, remoteAuthority: string | undefined, shellType: TerminalShellType): string {
	return `${machineId}:${remoteAuthority ?? 'local'}:${shellType}`;
}

async function getShellGlobals(
	shellType: TerminalShellType,
	existingCommands?: Set<string>,
	machineId?: string,
	remoteAuthority?: string
): Promise<ICompletionResource[] | undefined> {
	if (!machineId) {
		return await fetchAndCacheShellGlobals(shellType, existingCommands, undefined, undefined);
	}
	const cacheKey = getCacheKey(machineId, remoteAuthority, shellType);
	const cached = cachedGlobals.get(cacheKey);
	const now = Date.now();
	if (cached && now - cached.timestamp > CACHE_MAX_AGE_MS) {
		cachedGlobals.delete(cacheKey);
	}
	// Check if existing commands differ from cached
	if (cached.existingCommands && existingCommandsArr.length !== cached.existingCommands.length) {
		shouldRefresh = true;
	}
	return cached.commands ?? await fetchAndCacheShellGlobals(shellType, existingCommands, machineId, remoteAuthority);
}
```

**Variations:**
- In-flight request deduplication prevents concurrent shell process spawning
- Cache key includes remote authority for remote development scenarios
- TTL-based eviction (7 days) with explicit invalidation on PATH changes
- Falls back to uncached path if no machine ID available

---

#### Pattern: Completion Item Creation with Replacement Range

**Where:** `extensions/terminal-suggest/src/helpers/completionItem.ts:9-19`

**What:** Transforms internal resource to VS Code completion item with cursor-aware replacement range.

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

**Variations:**
- Replacement range calculated from cursor position minus last word length
- Defaults to `Method` kind if not specified
- Supports both string and rich CompletionItemLabel for label field

---

#### Pattern: PATH Executable Discovery with Platform-Specific Handling

**Where:** `extensions/terminal-suggest/src/env/pathExecutableCache.ts:22-80, 82-120`

**What:** Caches executables from PATH with platform-specific path separator and case handling.

```typescript
export interface IExecutablesInPath {
	completionResources: Set<ICompletionResource> | undefined;
	labels: Set<string> | undefined;
}

export class PathExecutableCache implements vscode.Disposable {
	private _cachedExes: Map<string, Set<ICompletionResource> | undefined> = new Map();
	private _inProgressRequest: {
		env: ITerminalEnvironment;
		shellType: TerminalShellType | undefined;
		promise: Promise<IExecutablesInPath | undefined>;
	} | undefined;

	async getExecutablesInPath(env: ITerminalEnvironment = process.env, shellType?: TerminalShellType): Promise<IExecutablesInPath | undefined> {
		if (this._inProgressRequest?.env === env && this._inProgressRequest?.shellType === shellType) {
			return this._inProgressRequest.promise;
		}
		const promise = this._doGetExecutablesInPath(env, shellType);
		this._inProgressRequest = { env, shellType, promise };
		await promise;
		this._inProgressRequest = undefined;
		return promise;
	}

	private async _doGetExecutablesInPath(env: ITerminalEnvironment, shellType?: TerminalShellType): Promise<IExecutablesInPath | undefined> {
		let pathValue: string | undefined;
		if (shellType === TerminalShellType.GitBash) {
			pathValue = process.env.PATH;
		} else if (isWindows) {
			const caseSensitivePathKey = Object.keys(env).find(key => key.toLowerCase() === 'path');
			pathValue = env[caseSensitivePathKey];
		} else {
			pathValue = env.PATH;
		}
		const paths = pathValue.split(isWindows ? ';' : ':');
		const pathSeparator = isWindows ? '\\' : '/';
		// Scan directories, populate labels set and completion resources
	}
}
```

**Variations:**
- Git Bash uses process.env.PATH directly (path separator regression issue noted)
- Windows uses case-insensitive PATH key lookup
- Implements request deduplication to prevent concurrent scans
- Returns both label set (for filter) and completion resource set (for rendering)

---

#### Pattern: Shell-Specific Globals Extraction

**Where:** `extensions/terminal-suggest/src/shell/bash.ts:11-22, 23-65` and `extensions/terminal-suggest/src/shell/pwsh.ts:11-16, 34-50`

**What:** Shell-specific strategies for extracting aliases and builtins with rich documentation.

```typescript
// Bash pattern
export async function getBashGlobals(options: ExecOptionsWithStringEncoding, existingCommands?: Set<string>): Promise<(string | ICompletionResource)[]> {
	return [
		...await getAliases(options),
		...await getBuiltins(options, 'compgen -b', existingCommands)
	];
}

async function getAliases(options: ExecOptionsWithStringEncoding): Promise<ICompletionResource[]> {
	const args = process.platform === 'darwin' ? ['-icl', 'alias'] : ['-ic', 'alias'];
	return getAliasesHelper('bash', args, /^alias (?<alias>[a-zA-Z0-9\.:-]+)='(?<resolved>.+)'$/, options);
}

// PowerShell pattern
const enum PwshCommandType {
	Alias = 1,
	Function = 2,
	Filter = 4,
	Cmdlet = 8,
	ExternalScript = 16,
	Application = 32,
	Script = 64,
	Configuration = 256,
}

const pwshCommandTypeToCompletionKind: Map<PwshCommandType, vscode.TerminalCompletionItemKind> = new Map([
	[PwshCommandType.Alias, vscode.TerminalCompletionItemKind.Alias],
	[PwshCommandType.Function, vscode.TerminalCompletionItemKind.Method],
	[PwshCommandType.Filter, vscode.TerminalCompletionItemKind.Method],
	[PwshCommandType.Cmdlet, vscode.TerminalCompletionItemKind.Method],
]);
```

**Variations:**
- Bash uses `compgen -b` for builtins and `alias` command with shell-specific flags
- macOS uses `-icl` (interactive, login, command) vs Linux `-ic` (interactive, command)
- PowerShell maps internal CommandTypes enum to VS Code completion kinds
- Each shell extracts both aliases and built-in commands separately

---

#### Pattern: File Glob Pattern Generation for Completion Lists

**Where:** `extensions/terminal-suggest/src/terminalSuggestMain.ts:628-637, 315-324`

**What:** Generates glob patterns from file extensions for terminal file completion filtering.

```typescript
function createFileGlobPattern(fileExtensions?: string[]): string | undefined {
	if (!fileExtensions || fileExtensions.length === 0) {
		return undefined;
	}
	const exts = fileExtensions.map(ext => ext.startsWith('.') ? ext.slice(1) : ext);
	if (exts.length === 1) {
		return `**/*.${exts[0]}`;
	}
	return `**/*.{${exts.join(',')}}`;
}

// Usage in provider
const cwd = result.cwd ?? terminal.shellIntegration?.cwd;
if (cwd && (result.showFiles || result.showDirectories)) {
	const globPattern = createFileGlobPattern(result.fileExtensions);
	return new vscode.TerminalCompletionList(result.items, {
		showFiles: result.showFiles,
		showDirectories: result.showDirectories,
		globPattern,
		cwd,
	});
}
```

**Variations:**
- Single extension: simple glob `**/*.ext`
- Multiple extensions: brace expansion glob `**/*.{ext1,ext2,ext3}`
- Normalizes leading dots on extensions (removes them)
- Only returned when explicit file/directory completion requested

---

#### Pattern: Terminal Context Extraction with Cursor-Safe Parsing

**Where:** `extensions/terminal-suggest/src/terminalSuggestMain.ts:285-287, 438-468`

**What:** Extracts current command string from full command line respecting shell operators.

```typescript
export function getCurrentCommandAndArgs(
	commandLine: string,
	cursorIndex: number,
	shellType: TerminalShellType | undefined
): string {
	if (commandLine.trim() === '') {
		return '';
	}
	// Check if cursor is mid-word
	if (cursorIndex < commandLine.length && /\S/.test(commandLine[cursorIndex])) {
		return '';
	}
	
	const beforeCursor = commandLine.slice(0, cursorIndex);
	const resetChars = shellType ? shellTypeResetChars.get(shellType) ?? defaultShellTypeResetChars : defaultShellTypeResetChars;
	
	let lastResetIndex = -1;
	for (const char of resetChars) {
		const idx = beforeCursor.lastIndexOf(char);
		if (idx > lastResetIndex) {
			lastResetIndex = idx;
		}
	}
	
	const currentCommandStart = lastResetIndex + 1;
	const currentCommandString = beforeCursor.slice(currentCommandStart).replace(/^\s+/, '');
	return currentCommandString;
}
```

**Variations:**
- Returns empty string if cursor is mid-word (prevents premature completion)
- Searches for last occurrence of any reset character in the command line
- Trims leading whitespace after extracting command portion
- Falls back to Bash reset chars if shell type unknown

---

## Cross-Pattern Summary

The terminal-suggest extension establishes a cohesive API surface for terminal completion built on five core abstractions:

1. **Shell Type Enumeration**: Type-safe shell identification with platform-specific detection
2. **Context Extraction**: Cursor-aware parsing that respects shell-specific operator semantics
3. **Token Classification**: Dynamic determination of command vs. argument position using reset characters
4. **Resource Unification**: Single `ICompletionResource` type consumed from shells, specs, and PATH
5. **Caching Strategy**: Multi-level cache with TTL, machine identity, and request deduplication

A Tauri/Rust port must replicate: (1) the `registerTerminalCompletionProvider` API contract with shell separator registration, (2) `TerminalShellType` enumeration and detection logic, (3) token/reset character maps with shell-specific semantics, (4) timeout and cancellation token handling patterns, and (5) the `ICompletionResource` and `TerminalCompletionItem` data flow.

The implementation shows strong separation between shell-specific knowledge (globals extraction), spec-based command resolution, and generic PATH executables—suggesting a port should maintain these modules as separate Rust subsystems.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
