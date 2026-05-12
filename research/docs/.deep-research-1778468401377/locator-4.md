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
