# Pattern Analysis: Desktop Launcher CLI & Bootstrap Architecture
## Partition 67/79 - VS Code TypeScript/Electron to Tauri/Rust Porting

### Scope
- `src/cli.ts` (26 LOC)
- Supporting bootstrap infrastructure: `bootstrap-*.ts`, `src/vs/code/node/cli.ts`, `argv.ts`

---

## Pattern 1: Sequential Bootstrap Chain with Environment Initialization

**Found in**: `src/cli.ts:1-27`

**What it does**: Establishes a mandatory initialization sequence where global state modifications must occur before any functional imports. Uses ES module top-level await to enforce ordering.

```typescript
// src/cli.ts - Lines 6-26
import './bootstrap-cli.js'; // this MUST come before other imports as it changes global state
import { configurePortable } from './bootstrap-node.js';
import { bootstrapESM } from './bootstrap-esm.js';
import { resolveNLSConfiguration } from './vs/base/node/nls.js';
import { product } from './bootstrap-meta.js';

// NLS
const nlsConfiguration = await resolveNLSConfiguration({ 
    userLocale: 'en', 
    osLocale: 'en', 
    commit: product.commit, 
    userDataPath: '', 
    nlsMetadataPath: import.meta.dirname 
});
process.env['VSCODE_NLS_CONFIG'] = JSON.stringify(nlsConfiguration);

// Enable portable support
configurePortable(product);

// Signal processes that we got launched as CLI
process.env['VSCODE_CLI'] = '1';

// Bootstrap ESM
await bootstrapESM();

// Load Server
await import('./vs/code/node/cli.js');
```

**Key aspects**:
- Comment-enforced import ordering (side-effect imports first)
- Environment variable as feature flag (`VSCODE_CLI = '1'`)
- Deferred module loading via dynamic `import()`
- NLS configuration serialized into process.env
- Portable mode configuration before ESM bootstrap
- Top-level await gates subsequent execution

**Variations found**:
- `src/server-cli.ts:1-30`: Identical pattern for server CLI entry point
- `src/main.ts:10-100`: Uses synchronous `parseCLIArgs()` instead of async patterns
- `src/server-main.ts`: Electron-specific variant with crash reporter initialization

---

## Pattern 2: Environment-Driven Feature Detection & Conditional Behavior

**Found in**: `src/bootstrap-node.ts:14-30`

**What it does**: Uses process environment variables as feature gates for conditional runtime behavior. Electron detection triggers FSM module resolution hooks.

```typescript
// src/bootstrap-node.ts - Lines 14-30
if (process.env['ELECTRON_RUN_AS_NODE'] || process.versions['electron']) {
    const jsCode = `
    export async function resolve(specifier, context, nextResolve) {
        if (specifier === 'fs') {
            return {
                format: 'builtin',
                shortCircuit: true,
                url: 'node:original-fs'
            };
        }

        return nextResolve(specifier, context);
    }`;
    register(`data:text/javascript;base64,${Buffer.from(jsCode).toString('base64')}`, import.meta.url);
}
```

**Key aspects**:
- Detects Electron runtime via `process.versions.electron`
- Patches module resolution to redirect `fs` imports to `original-fs`
- Uses data: URLs for inline module registration
- Base64 encoding avoids string escaping complexity
- Conditional module hooks prevent unnecessary overhead

**Variations**:
- `src/bootstrap-node.ts:62-74`: `devInjectNodeModuleLookupPath()` for dev-mode module path injection
- `src/bootstrap-node.ts:76-128`: Global module path sanitization for Windows/Linux platforms
- `src/main.ts:36-54`: Sandbox enablement based on CLI flags and argv.json config

---

## Pattern 3: Declarative Option Schema with Type-Safe Parsing

**Found in**: `src/vs/platform/environment/node/argv.ts:21-46, 50-260`

**What it does**: Defines CLI argument schema as a TypeScript type that drives parsing behavior. Schema includes options, subcommands, aliases, deprecation paths, and categorization.

```typescript
// src/vs/platform/environment/node/argv.ts - Core schema structure
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

export interface Subcommand<T> {
    type: 'subcommand';
    description?: string;
    deprecationMessage?: string;
    options: OptionDescriptions<Required<T>>;
}

export const OPTIONS: OptionDescriptions<Required<NativeParsedArgs>> = {
    'chat': {
        type: 'subcommand',
        description: 'Pass in a prompt to run in a chat session...',
        options: {
            '_': { type: 'string[]', description: localize('prompt', "The prompt to use as chat.") },
            'mode': { type: 'string', cat: 'o', alias: 'm', args: 'mode', description: localize(...) },
            'add-file': { type: 'string[]', cat: 'o', alias: 'a', args: 'path', description: localize(...) },
            'help': { type: 'boolean', alias: 'h', description: localize('help', "Print usage.") }
        }
    },
    'diff': { 
        type: 'boolean', 
        cat: 'o', 
        alias: 'd', 
        args: ['file', 'file'], 
        description: localize('diff', "Compare two files...") 
    },
    'version': { 
        type: 'boolean', 
        cat: 't', 
        alias: 'v', 
        description: localize('version', "Print version.") 
    },
    // ... 260+ more option definitions
};
```

**Key aspects**:
- TypeScript-first schema: Options drive type generation, not vice-versa
- Subcommands as nested schema objects
- Alias mapping for short flags
- Deprecation tracking with reason messages
- Help categorization keys ('o', 't', 'e', 'm')
- Localization integrated into schema
- Global vs. command-local options distinction
- Schema has 200+ options across main + subcommands

**Schema categories**:
- 'o' (Options): Main CLI controls
- 'e' (Extensions Management): install-extension, uninstall-extension, etc.
- 't' (Troubleshooting): verbose, log, prof-startup, inspect-*, etc.
- 'm' (Model Context Protocol): add-mcp

---

## Pattern 4: Two-Path Argument Parsing with Process Type Detection

**Found in**: `src/vs/platform/environment/node/argvHelper.ts:64-100`

**What it does**: Implements separate parsers for main process vs. CLI subprocess, handling different argv array structures based on launcher type.

```typescript
// src/vs/platform/environment/node/argvHelper.ts - Lines 64-100

/**
 * Use this to parse raw code process.argv such as: `Electron . --verbose --wait`
 */
export function parseMainProcessArgv(processArgv: string[]): NativeParsedArgs {
    let [, ...args] = processArgv; // Remove node/electron executable path
    
    // Windows admin elevation: code.cmd sets ELECTRON_RUN_AS_NODE=1, passes cli.js as arg
    // Elevated process doesn't inherit env var, so Electron starts as GUI with cli.js stray
    if (isWindows && args.length > 0) {
        const resolvedArg = resolve(args[0]).toLowerCase();
        const installDir = dirname(process.execPath).toLowerCase() + '\\';
        if (resolvedArg.startsWith(installDir) && resolvedArg.endsWith('\\resources\\app\\out\\cli.js')) {
            args.shift(); // Remove stray cli.js from Windows admin elevation
        }
    }
    
    // If dev, remove the first non-option argument: it's the app location
    if (process.env['VSCODE_DEV']) {
        args = stripAppPath(args) || [];
    }
    
    const reportWarnings = !isLaunchedFromCli(process.env);
    return parseAndValidate(args, reportWarnings);
}

/**
 * Use this to parse raw code CLI process.argv such as: `Electron cli.js . --verbose --wait`
 */
export function parseCLIProcessArgv(processArgv: string[]): NativeParsedArgs {
    let [, , ...args] = processArgv; // Remove node AND electron AND cli.js paths
    
    if (process.env['VSCODE_DEV']) {
        args = stripAppPath(args) || [];
    }
    
    return parseAndValidate(args, true); // Always report warnings in CLI mode
}
```

**Key aspects**:
- `parseMainProcessArgv`: Removes 1 element (node/electron path)
- `parseCLIProcessArgv`: Removes 2 elements (node path + cli.js path)
- Windows-specific elevation workaround detects cli.js in installation directory
- Strips first non-option arg in dev mode
- Different warning reporting: main process checks `VSCODE_CLI` flag, CLI mode always reports
- `stripAppPath()` helper uses regex to find first non-flag argument

**Usage pattern**:
```typescript
// src/vs/code/node/cli.ts:48
args = parseCLIProcessArgv(argv); // Called with raw process.argv

// src/main.ts:36
const args = parseCLIArgs(); // For main process
```

---

## Pattern 5: Portable Mode Detection with Multi-Platform Path Resolution

**Found in**: `src/bootstrap-node.ts:133-190`

**What it does**: Detects and configures portable installation mode by checking for `data/` or named directory adjacency. Adjusts process environment for temp directories.

```typescript
// src/bootstrap-node.ts - Lines 133-190

export function configurePortable(product: Partial<IProductConfiguration>): 
    { portableDataPath: string; isPortable: boolean } {
    
    const appRoot = path.dirname(import.meta.dirname);

    function getApplicationPath(): string {
        if (process.env['VSCODE_DEV']) {
            return appRoot;
        }

        if (process.platform === 'darwin') {
            // macOS: .../VS Code.app/Contents/Resources/app
            // -> .../VS Code.app
            return path.dirname(path.dirname(path.dirname(appRoot)));
        }

        // Windows versioned update: .../Microsoft VS Code Insiders/<version>/resources/app
        // -> .../<version>
        if (process.platform === 'win32' && product.win32VersionedUpdate) {
            return path.dirname(path.dirname(path.dirname(appRoot)));
        }

        // Default: resources/app -> . (Linux standard layout)
        return path.dirname(path.dirname(appRoot));
    }

    function getPortableDataPath(): string {
        if (process.env['VSCODE_PORTABLE']) {
            return process.env['VSCODE_PORTABLE'];
        }

        if (process.platform === 'win32' || process.platform === 'linux') {
            return path.join(getApplicationPath(), 'data');
        }

        // macOS uses named directory adjacent to app bundle
        const portableDataName = product.portable || `${product.applicationName}-portable-data`;
        return path.join(path.dirname(getApplicationPath()), portableDataName);
    }

    const portableDataPath = getPortableDataPath();
    const isPortable = !('target' in product) && fs.existsSync(portableDataPath);
    const portableTempPath = path.join(portableDataPath, 'tmp');
    const isTempPortable = isPortable && fs.existsSync(portableTempPath);

    if (isPortable) {
        process.env['VSCODE_PORTABLE'] = portableDataPath;
    } else {
        delete process.env['VSCODE_PORTABLE'];
    }

    if (isTempPortable) {
        if (process.platform === 'win32') {
            process.env['TMP'] = portableTempPath;
            process.env['TEMP'] = portableTempPath;
        } else {
            process.env['TMPDIR'] = portableTempPath;
        }
    }

    return {
        portableDataPath,
        isPortable
    };
}
```

**Key aspects**:
- Platform-specific path calculations (3-level traversals differ by OS)
- Windows versioned installs require special handling
- macOS uses named sibling directory (not nested)
- Linux/Windows use `data/` subdirectory
- Detection via `fs.existsSync()` on computed path
- Checks for `tmp/` subdirectory separately
- Overrides `TMP`/`TEMP` on Windows, `TMPDIR` on Unix
- Returns both path and boolean flag for conditional logic
- Used in `src/cli.ts:17` and `src/main.ts:34`

---

## Pattern 6: Subcommand Routing with Child Process Spawning

**Found in**: `src/vs/code/node/cli.ts:33-90`

**What it does**: Routes recognized subcommands to separate processes (Rust-based tunnel CLI, cargo dev mode). Handles stdio piping and exit code propagation.

```typescript
// src/vs/code/node/cli.ts - Lines 33-90

function shouldSpawnCliProcess(argv: NativeParsedArgs): boolean {
    return !!argv['install-source']
        || !!argv['list-extensions']
        || !!argv['install-extension']
        || !!argv['uninstall-extension']
        || !!argv['update-extensions']
        || !!argv['locate-extension']
        || !!argv['add-mcp']
        || !!argv['telemetry'];
}

export async function main(argv: string[]): Promise<void> {
    let args: NativeParsedArgs;

    try {
        args = parseCLIProcessArgv(argv);
    } catch (err) {
        console.error(err.message);
        return;
    }

    for (const subcommand of NATIVE_CLI_COMMANDS) {
        if (args[subcommand]) {
            if (!product.tunnelApplicationName) {
                console.error(`'${subcommand}' command not supported in ${product.applicationName}`);
                return;
            }
            
            const env: IProcessEnvironment = { ...process.env };
            delete env['ELECTRON_RUN_AS_NODE']; // Prevent Electron-specific imports
            
            const tunnelArgs = argv.slice(argv.indexOf(subcommand) + 1); // Get args after subcommand
            
            return new Promise((resolve, reject) => {
                let tunnelProcess: ChildProcess;
                const stdio: StdioOptions = ['ignore', 'pipe', 'pipe'];
                
                if (process.env['VSCODE_DEV']) {
                    // Dev: spawn cargo process
                    tunnelProcess = spawn('cargo', ['run', '--', subcommand, ...tunnelArgs], { 
                        cwd: join(getAppRoot(), 'cli'), 
                        stdio, 
                        env 
                    });
                } else {
                    // Prod: use pre-built tunnel executable
                    const appPath = process.platform === 'darwin'
                        ? join(dirname(dirname(process.execPath)), 'Resources', 'app')
                        : dirname(process.execPath);
                    const tunnelCommand = join(appPath, 'bin', 
                        `${product.tunnelApplicationName}${isWindows ? '.exe' : ''}`);
                    tunnelProcess = spawn(tunnelCommand, [subcommand, ...tunnelArgs], { 
                        cwd: cwd(), 
                        stdio, 
                        env 
                    });
                }

                tunnelProcess.stdout!.pipe(process.stdout);
                tunnelProcess.stderr!.pipe(process.stderr);
                tunnelProcess.on('exit', resolve);
                tunnelProcess.on('error', reject);
            });
        }
    }
    // ... rest of handler logic (help, version, shell-integration, etc.)
}
```

**Key aspects**:
- `NATIVE_CLI_COMMANDS` includes 'tunnel', 'serve-web', 'agent'
- `shouldSpawnCliProcess()` checks for operations requiring separate process
- Arguments after subcommand extracted via `argv.indexOf(subcommand) + 1`
- Electron environment variable deleted to prevent importing Electron modules
- Dev mode: spawns `cargo run` in `./cli` directory
- Prod mode: looks for pre-built binary in platform-specific locations
  - macOS: `../../../Resources/app/bin/`
  - Linux/Windows: sibling to execPath
- Binary name includes `.exe` suffix on Windows
- Stdio: ignores stdin, pipes stdout/stderr to parent
- Exit code propagates via Promise resolution

---

## Pattern 7: Minimist-Based Recursive Subcommand Parsing

**Found in**: `src/vs/platform/environment/node/argv.ts:278-331`

**What it does**: Recursively parses subcommands, builds option maps on-the-fly, delegates to minimist with computed string/boolean arrays.

```typescript
// src/vs/platform/environment/node/argv.ts - Lines 278-331

export function parseArgs<T>(
    args: string[], 
    options: OptionDescriptions<T>, 
    errorReporter: ErrorReporter = ignoringReporter
): T {
    // Find first non-option arg that's a recognized subcommand
    const firstPossibleCommand = args.find((a, i) => 
        a.length > 0 && 
        a[0] !== '-' && 
        options.hasOwnProperty(a) && 
        options[a as T].type === 'subcommand'
    );

    const alias: { [key: string]: string } = {};
    const stringOptions: string[] = ['_'];
    const booleanOptions: string[] = [];
    const globalOptions: Record<string, Option<...>> = {};
    let command: Subcommand<Record<string, unknown>> | undefined = undefined;
    
    // Build option maps from schema
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
            } else if (o.type === 'boolean') {
                booleanOptions.push(optionId);
                if (o.deprecates) {
                    booleanOptions.push(...o.deprecates);
                }
            }
            if (o.global) {
                globalOptions[optionId] = o;
            }
        }
    }
    
    // If subcommand found, recurse with merged options
    if (command && firstPossibleCommand) {
        const mergedOptions: Record<string, ...> = globalOptions;
        for (const optionId in command.options) {
            mergedOptions[optionId] = command.options[optionId];
        }
        const newArgs = args.filter(a => a !== firstPossibleCommand);
        const reporter = errorReporter.getSubcommandReporter?.(firstPossibleCommand);
        const subcommandOptions = parseArgs(newArgs, mergedOptions, reporter);
        return <T>{
            [firstPossibleCommand]: subcommandOptions,
            _: []
        };
    }

    // Parse with computed option maps
    const parsedArgs = minimist(args, { string: stringOptions, boolean: booleanOptions, alias });
    // ... cleanup and validation logic
}
```

**Key aspects**:
- First pass: identifies subcommand by name before parsing
- Schema-driven: computes option type arrays during traversal
- Deprecation tracking: deprecated IDs added to type arrays
- Global options merged into subcommand scope
- Recursive: subcommand parsing with filtered args and merged schema
- Minimist: receives computed string/boolean/alias maps
- No ambiguity: minimist knows exact type for each option
- Subcommand returns nested object structure

---

## Summary: Desktop Launcher Architecture

The VS Code CLI architecture demonstrates **layered initialization with environment-driven configuration**:

1. **Bootstrap Chain** (`src/cli.ts`): Side-effect imports + sequential initialization before functional code execution
2. **Feature Detection** (`bootstrap-*.ts`): Environment variables and runtime checks (Electron detection, VSCODE_DEV)
3. **Schema-First CLI** (`argv.ts`): TypeScript types drive minimist parser configuration; 200+ options with categorization and deprecation
4. **Dual-Path Parsing** (`argvHelper.ts`): Separate entry points for main process vs. CLI subprocess, handling Windows elevation quirks
5. **Portable Mode** (`bootstrap-node.ts`): Multi-platform path logic with optional temp directory isolation
6. **Subcommand Routing** (`cli.ts`): Routes recognized commands to child processes (Rust tunnel CLI, cargo dev mode)

**Key portability considerations for Tauri/Rust**:
- Environment variables as feature gates replicate as compile-time features or runtime flags
- Portable mode detection pattern applicable to Rust (fs::exists checks)
- Minimist-style recursive parsing translates to clap/structopt with subcommand traits
- Child process spawning can use `std::process::Command`
- Windows path handling requires cross-platform abstractions (e.g., `std::path::Path`)
- NLS configuration serialization pattern works in Rust (serde_json)
