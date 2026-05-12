# Partition 66 of 80 — Findings

## Scope
`src/server-cli.ts/` (1 files, 30 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# Locator 66: Server CLI Entry Point

## Scope
- `src/server-cli.ts` (31 LOC) — Server CLI entry point for VS Code server mode

## Implementation
- `src/server-cli.ts` — Root server CLI bootstrap; configures NLS, environment variables, and delegates to `/vs/server/node/server.cli.js` for main server logic

## Key Findings

**What this file does:**
- Acts as the top-level entry point for `code-server` (VS Code in server mode)
- Performs ESM bootstrap and NLS (Localization) configuration
- Sets up environment paths for Node modules in development mode
- Delegates actual server initialization to the compiled JavaScript entry point at `src/vs/server/node/server.cli.js`

**Relevance to Tauri/Rust porting:**
This file represents the minimal Node.js/Electron server CLI bootstrap. For a Rust/Tauri port, this would need to be replaced with:
1. A Rust binary entry point (likely `src/main.rs` in a Cargo project)
2. Rust-based NLS resolution and environment setup
3. Initialization of the core server logic in Rust instead of TypeScript

**No parseArgs pattern found:** The query seed mentioned looking for `parseArgs` calls, but this file contains no explicit argument parsing—it assumes command-line arguments are passed through to the delegated server.cli.js module.

## Summary

Partition 66 contains a single file (`src/server-cli.ts`) that serves as the thin bootstrapping layer for VS Code's server mode. It establishes ESM compatibility, NLS configuration, and development environment setup before forwarding control to the actual server implementation. This would be a key reference point for designing a Rust-based server entry point in a Tauri port.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
_(no analysis produced)_

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# Server CLI Entry Point Patterns (Partition 66)

Research scope: `src/server-cli.ts` — Bootstrap entry point for VS Code server CLI. This file demonstrates patterns that would be essential for porting to Tauri/Rust, including argument parsing, environment variable handling, process spawning, and cross-platform CLI invocation.

## Pattern Analysis

#### Pattern 1: Bootstrap Initialization Chain
**Where:** `src/server-cli.ts:6-30`
**What:** Conditional module loading with global state setup before importing main server logic.
```typescript
import './bootstrap-server.js'; // this MUST come before other imports as it changes global state
import { join } from 'node:path';
import { devInjectNodeModuleLookupPath } from './bootstrap-node.js';
import { bootstrapESM } from './bootstrap-esm.js';
import { resolveNLSConfiguration } from './vs/base/node/nls.js';

const nlsConfiguration = await resolveNLSConfiguration({ userLocale: 'en', osLocale: 'en', commit: product.commit, userDataPath: '', nlsMetadataPath: import.meta.dirname });
process.env['VSCODE_NLS_CONFIG'] = JSON.stringify(nlsConfiguration);

if (process.env['VSCODE_DEV']) {
  process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH'] = process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH'] || join(import.meta.dirname, '..', 'remote', 'node_modules');
  devInjectNodeModuleLookupPath(process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH']);
} else {
  delete process.env['VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH'];
}

await bootstrapESM();
await import('./vs/server/node/server.cli.js');
```

**Variations / call-sites:**
- NLS (National Language Support) configuration resolved at startup
- Environment-conditional module path injection for dev mode
- Lazy loading of server CLI module (line 30)
- Global process environment mutation before execution

#### Pattern 2: Command-Line Argument Parsing with Minimist Adapter
**Where:** `src/vs/platform/environment/node/argv.ts:277-330`
**What:** Type-safe argument parsing using minimist with option descriptor metadata and error reporting hooks.
```typescript
export function parseArgs<T>(args: string[], options: OptionDescriptions<T>, errorReporter: ErrorReporter = ignoringReporter): T {
  const firstPossibleCommand = args.find((a, i) => a.length > 0 && a[0] !== '-' && options.hasOwnProperty(a) && options[a as T].type === 'subcommand');

  const alias: { [key: string]: string } = {};
  const stringOptions: string[] = ['_'];
  const booleanOptions: string[] = [];
  const globalOptions: Record<string, Option<'boolean'> | Option<'string'> | Option<'string[]'>> = {};
  let command: Subcommand<Record<string, unknown>> | undefined = undefined;
  
  for (const optionId in options) {
    const o = options[optionId];
    if (o.type === 'subcommand') {
      if (optionId === firstPossibleCommand) {
        command = o;
      }
    } else {
      if (o.alias) {
        alias[optionId] = o.alias;
      }
      if (o.type === 'string' || o.type === 'string[]') {
        stringOptions.push(optionId);
        if (o.deprecates) {
          stringOptions.push(...o.deprecates);
        }
      }
    }
  }

  const parsedArgs = minimist(args, { string: stringOptions, boolean: booleanOptions, alias });
```

**Variations / call-sites:**
- Used at line 37 in `server.main.ts`: `const args = parseArgs(process.argv.slice(2), serverOptions, errorReporter);`
- Used at line 127 in `server.cli.ts`: `const parsedArgs = parseArgs(args, options, errorReporter);`
- Supports recursive subcommand parsing (lines 313-325)
- Handles deprecated option aliases and validation

#### Pattern 3: Option Descriptor Type System
**Where:** `src/vs/platform/environment/node/argv.ts:21-46, 50-259`
**What:** Comprehensive declarative option definitions with type narrowing, deprecation tracking, and categorization.
```typescript
export interface Option<OptionType> {
  type: OptionType;
  alias?: string;
  deprecates?: string[]; // old deprecated ids
  args?: string | string[];
  description?: string;
  deprecationMessage?: string;
  allowEmptyValue?: boolean;
  cat?: keyof typeof helpCategories;
  global?: boolean;
}

export const OPTIONS: OptionDescriptions<Required<NativeParsedArgs>> = {
  'chat': {
    type: 'subcommand',
    description: 'Pass in a prompt to run in a chat session in the current working directory.',
    options: {
      '_': { type: 'string[]', description: localize('prompt', "The prompt to use as chat.") },
      'mode': { type: 'string', cat: 'o', alias: 'm', args: 'mode', description: localize('chatMode', "...") },
      'add-file': { type: 'string[]', cat: 'o', alias: 'a', args: 'path', description: localize('addFile', "...") },
    }
  },
  'tunnel': {
    type: 'subcommand',
    description: 'Make the current machine accessible from vscode.dev...',
    options: { /* nested subcommands */ }
  },
  'install-extension': { type: 'string[]', cat: 'e', args: 'ext-id | path', description: localize('installExtension', "...") },
  'version': { type: 'boolean', cat: 't', alias: 'v', description: localize('version', "Print version.") },
  _: { type: 'string[]' } // main arguments
};
```

**Variations / call-sites:**
- 100+ options defined in main OPTIONS object (lines 50-259)
- Supports boolean, string, and string[] types
- Hierarchical subcommand structure with recursive option nesting
- Help categories: 'o' (options), 'e' (extensions), 't' (troubleshooting), 'm' (MCP)

#### Pattern 4: Environment-Aware Command Routing
**Where:** `src/vs/server/node/server.cli.ts:86-106`
**What:** Conditional argument support filtering based on execution context (pipe vs. command).
```typescript
const cliPipe = process.env['VSCODE_IPC_HOOK_CLI'] as string;
const cliCommand = process.env['VSCODE_CLIENT_COMMAND'] as string;
const cliCommandCwd = process.env['VSCODE_CLIENT_COMMAND_CWD'] as string;
const cliRemoteAuthority = process.env['VSCODE_CLI_AUTHORITY'] as string;
const cliStdInFilePath = process.env['VSCODE_STDIN_FILE_PATH'] as string;

export async function main(desc: ProductDescription, args: string[]): Promise<void> {
  if (!cliPipe && !cliCommand) {
    console.log('Command is only available in WSL or inside a Visual Studio Code terminal.');
    return;
  }

  const options: OptionDescriptions<Required<RemoteParsedArgs>> = { ...OPTIONS, gitCredential: { type: 'string' }, openExternal: { type: 'boolean' } };
  const isSupported = cliCommand ? isSupportedForCmd : isSupportedForPipe;
  for (const optionId in OPTIONS) {
    const optId = <keyof RemoteParsedArgs>optionId;
    if (!isSupported(optId)) {
      delete options[optId];
    }
  }

  if (cliPipe) {
    options['openExternal'] = { type: 'boolean' };
  }
```

**Variations / call-sites:**
- Two support filter functions: `isSupportedForCmd()` (line 39) and `isSupportedForPipe()` (line 55)
- Dynamically removes unsupported options based on execution context
- Environment variables control execution path: WSL (cliCommand) vs. integrated terminal (cliPipe)

#### Pattern 5: Process Spawning with Platform-Specific Handling
**Where:** `src/vs/server/node/server.cli.ts:230-301`
**What:** Cross-platform child process invocation with output piping, environment setup, and Windows batch file detection.
```typescript
if (cliCommand) {
  const newCommandline: string[] = [];
  for (const key in parsedArgs) {
    const val = parsedArgs[key as keyof typeof parsedArgs];
    if (typeof val === 'boolean') {
      if (val) {
        newCommandline.push('--' + key);
      }
    } else if (Array.isArray(val)) {
      for (const entry of val) {
        newCommandline.push(`--${key}=${entry.toString()}`);
      }
    } else if (val) {
      newCommandline.push(`--${key}=${val.toString()}`);
    }
  }
  if (remote !== null) {
    newCommandline.push(`--remote=${remote || cliRemoteAuthority}`);
  }

  const ext = extname(cliCommand);
  if (ext === '.bat' || ext === '.cmd') {
    const processCwd = cliCommandCwd || cwd();
    if (verbose) {
      console.log(`Invoking: cmd.exe /C ${cliCommand} ${newCommandline.join(' ')} in ${processCwd}`);
    }
    cp.spawn('cmd.exe', ['/C', cliCommand, ...newCommandline], {
      stdio: 'inherit',
      cwd: processCwd
    });
  } else {
    const cliCwd = dirname(cliCommand);
    const env = { ...process.env, ELECTRON_RUN_AS_NODE: '1' };
    const versionFolder = desc.commit.substring(0, 10);
    if (fs.existsSync(join(cliCwd, versionFolder))) {
      newCommandline.unshift(`${versionFolder}/resources/app/out/cli.js`);
    } else {
      newCommandline.unshift('resources/app/out/cli.js');
    }
    if (verbose) {
      console.log(`Invoking: cd "${cliCwd}" && ELECTRON_RUN_AS_NODE=1 "${cliCommand}" "${newCommandline.join('" "')}"`);
    }
    if (runningInWSL2()) {
      if (verbose) {
        console.log(`Using pipes for output.`);
      }
      const childProcess = cp.spawn(cliCommand, newCommandline, { cwd: cliCwd, env, stdio: ['inherit', 'pipe', 'pipe'] });
      childProcess.stdout.on('data', data => process.stdout.write(data));
      childProcess.stderr.on('data', data => process.stderr.write(data));
    } else {
      cp.spawn(cliCommand, newCommandline, { cwd: cliCwd, env, stdio: 'inherit' });
    }
  }
}
```

**Variations / call-sites:**
- Batch file detection (`.bat`, `.cmd`) uses `cmd.exe /C` invocation (line 275)
- Node.js executable spawning with `ELECTRON_RUN_AS_NODE=1` environment variable (line 281)
- WSL2 detection with piped output handling (lines 291-297)
- Version-aware path resolution for version-specific folders (lines 282-286)

#### Pattern 6: Error Reporter Interface and Implementation
**Where:** `src/vs/platform/environment/node/argv.ts:261-275` and `src/vs/server/node/server.cli.ts:112-125`
**What:** Callback-based error handling with specialized reporters for different execution contexts.
```typescript
export interface ErrorReporter {
  onUnknownOption(id: string): void;
  onMultipleValues(id: string, usedValue: string): void;
  onEmptyValue(id: string): void;
  onDeprecatedOption(deprecatedId: string, message: string): void;
  getSubcommandReporter?(command: string): ErrorReporter;
}

// server.cli.ts implementation
const errorReporter: ErrorReporter = {
  onMultipleValues: (id: string, usedValue: string) => {
    console.error(`Option '${id}' can only be defined once. Using value ${usedValue}.`);
  },
  onEmptyValue: (id) => {
    console.error(`Ignoring option '${id}': Value must not be empty.`);
  },
  onUnknownOption: (id: string) => {
    console.error(`Ignoring option '${id}': not supported for ${desc.executableName}.`);
  },
  onDeprecatedOption: (deprecatedOption: string, message: string) => {
    console.warn(`Option '${deprecatedOption}' is deprecated: ${message}`);
  }
};

const parsedArgs = parseArgs(args, options, errorReporter);
```

**Variations / call-sites:**
- No-op reporter: `ignoringReporter` (line 270-275)
- Warning-based reporter in `argvHelper.ts:13-42` for main process
- Server-specific reporter in `server.main.ts:22-35`
- Supports nested subcommand reporters via `getSubcommandReporter()` optional method

#### Pattern 7: Help and Version Message Generation
**Where:** `src/vs/platform/environment/node/argv.ts:466-538`
**What:** Dynamic help text generation with TTY-aware column formatting and categorized options.
```typescript
export function buildHelpMessage(productName: string, executableName: string, version: string, options: OptionDescriptions<unknown> | Record<string, Option<'boolean'> | Option<'string'> | Option<'string[]'> | Subcommand<Record<string, unknown>>>, capabilities?: { noPipe?: boolean; noInputFiles?: boolean; isChat?: boolean }): string {
  const columns = (process.stdout).isTTY && (process.stdout).columns || 80;
  const inputFiles = capabilities?.noInputFiles ? '' : capabilities?.isChat ? ` [${localize('cliPrompt', 'prompt')}]` : ` [${localize('paths', 'paths')}...]`;
  const subcommand = capabilities?.isChat ? ' chat' : '';

  const help = [`${productName} ${version}`];
  help.push('');
  help.push(`${localize('usage', "Usage")}: ${executableName}${subcommand} [${localize('options', "options")}]${inputFiles}`);
  // ... option categorization and formatting ...
  return help.join('\n');
}

export function buildVersionMessage(version: string | undefined, commit: string | undefined): string {
  return `${version || localize('unknownVersion', "Unknown version")}\n${commit || localize('unknownCommit', "Unknown commit")}\n${process.arch}`;
}
```

**Variations / call-sites:**
- Used in server.cli.ts line 133: `console.log(buildHelpMessage(desc.productName, desc.executableName, desc.version, options));`
- Responsive column width detection based on stdout TTY status
- Platform architecture included in version output (process.arch)

#### Pattern 8: Path Translation and Remote URI Mapping
**Where:** `src/vs/server/node/server.cli.ts:168-223, 483-515`
**What:** Conditional URI rewriting for remote execution with file system operations wrapped in error handling.
```typescript
const mapFileUri = cliRemoteAuthority ? mapFileToRemoteUri : (uri: string) => uri;

const folderURIs = (parsedArgs['folder-uri'] || []).map(mapFileUri);
parsedArgs['folder-uri'] = folderURIs;

const fileURIs = (parsedArgs['file-uri'] || []).map(mapFileUri);
parsedArgs['file-uri'] = fileURIs;

// ...

function pathToURI(input: string): url.URL {
  input = input.trim();
  input = resolve(preferredCwd, input);
  return url.pathToFileURL(input);
}

function translatePath(input: string, mapFileUri: (input: string) => string, folderURIS: string[], fileURIS: string[]) {
  const url = pathToURI(input);
  const mappedUri = mapFileUri(url.href);
  try {
    const stat = fs.lstatSync(fs.realpathSync(input));

    if (stat.isFile()) {
      fileURIS.push(mappedUri);
    } else if (stat.isDirectory()) {
      folderURIS.push(mappedUri);
    } else if (input === '/dev/null') {
      fileURIS.push(mappedUri);
    }
  } catch (e) {
    if (e.code === 'ENOENT') {
      fileURIS.push(mappedUri);
    } else {
      console.log(`Problem accessing file ${input}. Ignoring file`, e);
    }
  }
}

function mapFileToRemoteUri(uri: string): string {
  return uri.replace(/^file:\/\//, 'vscode-remote://' + cliRemoteAuthority);
}
```

**Variations / call-sites:**
- Symlink resolution with `fs.realpathSync()` before stat operations
- Graceful handling of missing files (ENOENT treated as valid file URI)
- Special case for `/dev/null` (external tools like git difftool)
- Conditional routing based on `cliRemoteAuthority` environment variable

## Summary

This partition documents 8 core CLI infrastructure patterns essential for porting VS Code's server CLI to Rust/Tauri:

1. **Bootstrap chains** – Multi-stage initialization with global state mutations
2. **Argument parsing** – Type-safe wrapper over minimist with metadata-driven validation
3. **Option descriptors** – Declarative schema with type narrowing and nested subcommands
4. **Context-aware routing** – Dynamic option filtering based on execution environment
5. **Cross-platform process spawning** – Platform-specific batch/shell handling with output piping
6. **Error reporting** – Extensible callback interface with context-aware implementations
7. **Dynamic help generation** – TTY-aware formatting with option categorization
8. **Path translation** – URL conversion with remote authority rewriting and safe file operations

These patterns show how VS Code manages CLI entry point complexity across Windows (batch), Unix-like systems, WSL, and remote execution contexts. A Rust/Tauri port would need equivalent mechanisms for argument parsing (likely using a crate like `clap` or `structopt`), path handling with file system safety, and platform-conditional spawning.

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
