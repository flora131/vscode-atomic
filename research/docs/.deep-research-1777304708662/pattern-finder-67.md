# VS Code CLI Bootstrap Patterns

Research partition 67 of 79: Analyzing CLI initialization patterns in `src/cli.ts` and related bootstrap infrastructure that would inform a Tauri/Rust port.

## Patterns Found

#### Pattern 1: Ordered Bootstrap Chain
**Where:** `src/cli.ts:6-23`
**What:** Early execution phases with strict ordering requirements due to global state side effects.

```typescript
import './bootstrap-cli.js'; // this MUST come before other imports as it changes global state
import { configurePortable } from './bootstrap-node.js';
import { bootstrapESM } from './bootstrap-esm.js';
import { resolveNLSConfiguration } from './vs/base/node/nls.js';
import { product } from './bootstrap-meta.js';

const nlsConfiguration = await resolveNLSConfiguration({ ... });
process.env['VSCODE_NLS_CONFIG'] = JSON.stringify(nlsConfiguration);

configurePortable(product);

process.env['VSCODE_CLI'] = '1';

await bootstrapESM();

await import('./vs/code/node/cli.js');
```

**Variations / call-sites:**
- `bootstrap-fork.ts`: Fork process variant removes global node paths, configures crash reporting, pipes logging, handles exceptions, sets parent PID monitoring
- `bootstrap-server.ts`: Server variant disables Electron environment
- Core requirement: Side-effect imports must execute first (environment variable deletion, stream wrapping)

#### Pattern 2: Environment Variable Mutation as Configuration
**Where:** `src/cli.ts:13-20`
**What:** Uses process.env as primary configuration mechanism for downstream modules.

```typescript
const nlsConfiguration = await resolveNLSConfiguration({ 
  userLocale: 'en', 
  osLocale: 'en', 
  commit: product.commit, 
  userDataPath: '', 
  nlsMetadataPath: import.meta.dirname 
});
process.env['VSCODE_NLS_CONFIG'] = JSON.stringify(nlsConfiguration);

configurePortable(product);

process.env['VSCODE_CLI'] = '1';
```

**Variations / call-sites:**
- `bootstrap-node.ts`: Sets `VSCODE_CWD` for consistent path lookups across platforms
- `bootstrap-fork.ts`: Sets `VSCODE_VERBOSE_LOGGING`, `VSCODE_PIPE_LOGGING`, `VSCODE_HANDLES_UNCAUGHT_ERRORS`, `VSCODE_PARENT_PID`, `VSCODE_CRASH_REPORTER_PROCESS_TYPE`, `VSCODE_ESM_ENTRYPOINT`
- `bootstrap-esm.ts`: Reads `VSCODE_NLS_CONFIG`, `VSCODE_DEV`, sets `VSCODE_NLS_LANGUAGE`
- Pattern: Environment is a cross-module communication channel predating module import

#### Pattern 3: Platform-Specific Path Calculation
**Where:** `src/bootstrap-node.ts:133-190`
**What:** Conditional path resolution based on platform and build mode (dev vs. production).

```typescript
export function configurePortable(product: Partial<IProductConfiguration>): { 
  portableDataPath: string; 
  isPortable: boolean 
} {
  const appRoot = path.dirname(import.meta.dirname);

  function getApplicationPath(): string {
    if (process.env['VSCODE_DEV']) {
      return appRoot;
    }
    if (process.platform === 'darwin') {
      return path.dirname(path.dirname(path.dirname(appRoot)));
    }
    if (process.platform === 'win32' && product.win32VersionedUpdate) {
      return path.dirname(path.dirname(path.dirname(appRoot)));
    }
    return path.dirname(path.dirname(appRoot));
  }

  function getPortableDataPath(): string {
    if (process.env['VSCODE_PORTABLE']) {
      return process.env['VSCODE_PORTABLE'];
    }
    if (process.platform === 'win32' || process.platform === 'linux') {
      return path.join(getApplicationPath(), 'data');
    }
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

  return { portableDataPath, isPortable };
}
```

**Variations / call-sites:**
- macOS: Navigates up from `/app/Contents/Resources/app` to find application root
- Windows: Handles versioned update paths and drive letter filtering
- Linux: Uses home directory filtering
- Portable mode detection: Checks for existence of `data/` directory

#### Pattern 4: Global State Injection Before Module Resolution
**Where:** `src/bootstrap-esm.ts:32-35`
**What:** Injects product configuration and file paths into globalThis before ESM modules import.

```typescript
globalThis._VSCODE_PRODUCT_JSON = { ...product };
globalThis._VSCODE_PACKAGE_JSON = { ...pkg };
globalThis._VSCODE_FILE_ROOT = import.meta.dirname;
```

**Variations / call-sites:**
- `bootstrap-esm.ts:64`: Sets `globalThis._VSCODE_NLS_LANGUAGE` from parsed NLS config
- `bootstrap-esm.ts:78`: Sets `globalThis._VSCODE_NLS_MESSAGES` from loaded JSON file
- Used downstream: Modules import from global state instead of requiring file I/O

#### Pattern 5: Lazy Initialization with Memoization
**Where:** `src/bootstrap-esm.ts:39-104`
**What:** NLS setup deferred, cached, and awaited by multiple consumers.

```typescript
let setupNLSResult: Promise<INLSConfiguration | undefined> | undefined = undefined;

function setupNLS(): Promise<INLSConfiguration | undefined> {
  if (!setupNLSResult) {
    setupNLSResult = doSetupNLS();
  }
  return setupNLSResult;
}

async function doSetupNLS(): Promise<INLSConfiguration | undefined> {
  performance.mark('code/willLoadNls');

  let nlsConfig: INLSConfiguration | undefined = undefined;
  let messagesFile: string | undefined;
  
  if (process.env['VSCODE_NLS_CONFIG']) {
    try {
      nlsConfig = JSON.parse(process.env['VSCODE_NLS_CONFIG']);
      if (nlsConfig?.languagePack?.messagesFile) {
        messagesFile = nlsConfig.languagePack.messagesFile;
      } else if (nlsConfig?.defaultMessagesFile) {
        messagesFile = nlsConfig.defaultMessagesFile;
      }
      globalThis._VSCODE_NLS_LANGUAGE = nlsConfig?.resolvedLanguage;
    } catch (e) {
      console.error(`Error reading VSCODE_NLS_CONFIG from environment: ${e}`);
    }
  }

  if (process.env['VSCODE_DEV'] || !messagesFile) {
    return undefined;
  }

  try {
    globalThis._VSCODE_NLS_MESSAGES = JSON.parse((await fs.promises.readFile(messagesFile)).toString());
  } catch (error) {
    console.error(`Error reading NLS messages file ${messagesFile}: ${error}`);
    // Corruption handling...
    if (nlsConfig?.languagePack?.corruptMarkerFile) {
      await fs.promises.writeFile(nlsConfig.languagePack.corruptMarkerFile, 'corrupted');
    }
    // Fallback to default...
  }

  performance.mark('code/didLoadNls');
  return nlsConfig;
}

export async function bootstrapESM(): Promise<void> {
  await setupNLS();
}
```

**Variations / call-sites:**
- Used from `bootstrap-fork.ts:226`: Fork variant awaits same `bootstrapESM()`
- Pattern: Handles dev mode (skip NLS), language pack corruption, fallback to English

#### Pattern 6: Node Module Resolution Hook Injection
**Where:** `src/bootstrap-esm.ts:14-30`
**What:** Conditionally registers ESM loader hooks at runtime to remap module imports.

```typescript
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

**Variations / call-sites:**
- `src/bootstrap-import.ts`: More complex variant that maps all dependencies from package.json
- Supports conditional exports, ESM vs. CJS detection, handles errors gracefully
- Used when `VSCODE_DEV_INJECT_NODE_MODULE_LOOKUP_PATH` environment variable is set

#### Pattern 7: Namespaced Process Configuration for Subprocesses
**Where:** `src/vs/code/node/cli.ts:60-88`
**What:** CLI detects subcommands and spawns separate processes with inherited environment customization.

```typescript
for (const subcommand of NATIVE_CLI_COMMANDS) {
  if (args[subcommand]) {
    if (!product.tunnelApplicationName) {
      console.error(`'${subcommand}' command not supported in ${product.applicationName}`);
      return;
    }
    const env: IProcessEnvironment = {
      ...process.env
    };
    delete env['ELECTRON_RUN_AS_NODE'];

    const tunnelArgs = argv.slice(argv.indexOf(subcommand) + 1);
    return new Promise((resolve, reject) => {
      let tunnelProcess: ChildProcess;
      const stdio: StdioOptions = ['ignore', 'pipe', 'pipe'];
      if (process.env['VSCODE_DEV']) {
        tunnelProcess = spawn('cargo', ['run', '--', subcommand, ...tunnelArgs], { cwd: join(getAppRoot(), 'cli'), stdio, env });
      } else {
        const appPath = process.platform === 'darwin'
          ? join(dirname(dirname(process.execPath)), 'Resources', 'app')
          : dirname(process.execPath);
        const tunnelCommand = join(appPath, 'bin', `${product.tunnelApplicationName}${isWindows ? '.exe' : ''}`);
        tunnelProcess = spawn(tunnelCommand, [subcommand, ...tunnelArgs], { cwd: cwd(), stdio, env });
      }

      tunnelProcess.stdout!.pipe(process.stdout);
      tunnelProcess.stderr!.pipe(process.stderr);
      tunnelProcess.on('exit', resolve);
      tunnelProcess.on('error', reject);
    });
  }
}
```

**Variations / call-sites:**
- Dev mode: Spawns Rust CLI via `cargo run` from `cli/` directory
- Production macOS: Finds binary in `Contents/Resources/app/bin/`
- Production others: Finds binary in sibling directory to executable
- Subcommands: `--install-source`, `--list-extensions`, `--install-extension`, etc.

#### Pattern 8: Process State Cleanup in Bootstrap
**Where:** `src/bootstrap-cli.ts:11`
**What:** Early deletion of environment variables that could escape to parent shell.

```typescript
delete process.env['VSCODE_CWD'];
```

**Variations / call-sites:**
- `bootstrap-server.ts:7`: Deletes `ELECTRON_RUN_AS_NODE` to prevent Electron module loading
- `src/bootstrap-node.ts:17-30`: Conditional SIGPIPE handler for Electron environments
- Pattern: Defensive cleanup to prevent state leakage across process boundaries

## Cross-Cutting Concerns

### Initialization Order Dependencies
1. **Phase 1 (Side-effects)**: `bootstrap-cli.js` - Environment cleanup
2. **Phase 2 (Node setup)**: `bootstrap-node.js` - CWD, stack traces, SIGPIPE handler, module resolution hooks
3. **Phase 3 (ESM setup)**: `bootstrap-esm.js` - ESM loader hooks, NLS initialization, global state
4. **Phase 4 (Product config)**: `bootstrap-meta.js` - Load product.json, package.json, handle embedded/overrides
5. **Phase 5 (CLI bootstrap)**: `cli.ts` - NLS resolution, portable mode, CLI signal, load actual CLI handler

### Porting Considerations for Tauri/Rust

1. **Environment-based configuration**: Current design heavily relies on `process.env` as IPC mechanism. Rust would need equivalent messaging pattern or configuration struct passing.

2. **Module resolution hooking**: Node.js loader hooks (`register()`) remap imports at runtime. Rust would need compile-time or initialization-time path configuration.

3. **Platform-specific resource discovery**: Application path calculation deeply bakes in Electron bundle structure (`.app/Contents/Resources/app` on macOS). Tauri uses different path conventions.

4. **Global state via globalThis**: Product config injected into `globalThis` before module imports. Rust would use static lazy initialization or dependency injection.

5. **Portable mode detection**: Filesystem presence checks (`fs.existsSync(portableDataPath)`) determine behavior. Requires equivalent before any IDE startup.

6. **Subcommand spawning**: CLI spawns tunnel/server as separate processes. Rust version would need equivalent subprocess handling for Cargo dev mode vs. packaged binary paths.

7. **Logging pipe wrapping**: Fork variant wraps `console.*` and `process.std*` at bootstrap time. Rust would need equivalent stream redirection mechanism.

8. **NLS lazy loading with fallback**: JSON language pack files loaded with corruption detection and English fallback. Requires async file I/O during bootstrap sequence.

## File References

- `/Users/norinlavaee/vscode-atomic/src/cli.ts` - Main CLI entry point (26 LOC, ESM, async-first)
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-cli.ts` - Environment cleanup (12 LOC)
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts` - Node.js setup, portable mode (191 LOC)
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-esm.ts` - ESM and NLS setup (113 LOC)
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-meta.ts` - Product/package config loading (56 LOC)
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-fork.ts` - Fork process setup (230 LOC)
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-import.ts` - Module resolution hooking (102 LOC)
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-server.ts` - Server variant (8 LOC)
- `/Users/norinlavaee/vscode-atomic/src/vs/code/node/cli.ts` - CLI command handler (implements `main(argv)`)
