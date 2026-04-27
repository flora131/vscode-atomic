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

