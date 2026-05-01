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

