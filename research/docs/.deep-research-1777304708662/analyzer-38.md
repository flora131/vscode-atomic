### Files Analysed

| File | LOC | Role |
|------|-----|------|
| `/Users/norinlavaee/vscode-atomic/src/main.ts` | 741 | Electron main entry point — bootstraps the entire process |
| `/Users/norinlavaee/vscode-atomic/src/bootstrap-node.ts` | 191 | Node.js / portable environment setup, module resolution patching |
| `/Users/norinlavaee/vscode-atomic/src/bootstrap-esm.ts` | 113 | ESM loader hooks, NLS message file loading into `globalThis` |
| `/Users/norinlavaee/vscode-atomic/src/bootstrap-meta.ts` | 55 | Product / package JSON resolution, patched at build time |
| `/Users/norinlavaee/vscode-atomic/src/vs/code/electron-main/main.ts` | 654 | `CodeMain` class — IPC server, service container, `CodeApplication` launch |

---

### Per-File Notes

#### `src/main.ts`

**Performance marks (synchronous, top-level)**
- `code/didStartMain` — line 23, marks the moment `main.ts` is evaluated.
- `code/willLoadMainBundle` / `code/didLoadMainBundle` — lines 25–31, bracket the bundle load time using `performance.timeOrigin`.

**Portable mode (line 34)**
`configurePortable(product)` is called synchronously. It returns `{ portableDataPath, isPortable }` and may set `VSCODE_PORTABLE`, `TMP`/`TEMP`/`TMPDIR` environment variables if a portable `data/tmp` directory exists.

**CLI argument parsing (line 36)**
`parseCLIArgs()` (line 569) calls `minimist(process.argv, ...)` with:
- string keys: `user-data-dir`, `locale`, `js-flags`, `crash-reporter-directory`
- boolean key: `disable-chromium-sandbox`
- default `sandbox: true`; alias `no-sandbox` → `sandbox`

**Sandbox gate (lines 43–54)**
Three branches:
1. If `args['sandbox']` is truthy and neither `--disable-chromium-sandbox` nor the argv.json key is set → `app.enableSandbox()` (line 46).
2. Else if `--no-sandbox` present but not `--disable-gpu-sandbox` → `app.commandLine.appendSwitch('disable-gpu-sandbox')` (line 50).
3. Else → append both `no-sandbox` and `disable-gpu-sandbox` (lines 52–53).

**userData path (lines 57–64)**
`getUserDataPath(args, product.nameShort)` computes the path. On Windows, if the path is on a UNC share, `addUNCHostToAllowlist` is called (lines 59–62). Then `app.setPath('userData', userDataPath)` (line 64) is called before `ready`.

**Code cache path (line 67)**
`getCodeCachePath()` (line 623) returns `undefined` if `--no-cached-data`, `VSCODE_DEV`, or no commit. Otherwise returns `path.join(userDataPath, 'CachedData', commit)`.

**Menu suppression (line 70)**
`Menu.setApplicationMenu(null)` disables Electron's default menu before `ready`.

**Crash reporter (lines 72–85)**
`configureCrashReporter()` (line 449) is called if `--crash-reporter-directory` is present or if `argv.json` has `enable-crash-reporter: true` and `--disable-crash-reporter` is absent.
Inside `configureCrashReporter`:
- If `--crash-reporter-directory` is given, the directory is normalised, validated as absolute (calls `app.exit(1)` lines 457, 465 on failure), created if missing, then `app.setPath('crashDumps', crashReporterDirectory)` (line 472).
- Otherwise reads `product.appCenter` per-platform URL, concatenates `&uid=&iid=&sid=` crash reporter ID, then calls `crashReporter.start(...)` (line 531) with `compress: true`, `ignoreSystemCrashHandler: true`.

**Portable log path (lines 91–93)**
If portable: `app.setAppLogsPath(path.join(userDataPath, 'logs'))` (line 92) forces logs inside the portable directory.

**Custom protocol registration (lines 96–105)**
`protocol.registerSchemesAsPrivileged(...)` is called before `ready` (Electron requirement):
- `vscode-webview`: `standard, secure, supportFetchAPI, corsEnabled, allowServiceWorkers, codeCache`
- `vscode-file`: `secure, standard, supportFetchAPI, corsEnabled, codeCache`

**Global app event listeners (line 108) — `registerListeners()` (line 589)**
- `open-file` (line 597): pushed into `globalThis.macOpenFiles[]` before `ready` — handles macOS file-drop on dock icon.
- `will-finish-launching` (line 612): registers `open-url` handler that pushes URLs into `openUrls[]` and calls `event.preventDefault()`.
- `getOpenUrls` (line 616) is exposed on `globalThis`; when called it removes the `open-url` listener and returns accumulated URLs.

**NLS pre-resolution (lines 115–131)**
`osLocale` is derived from `app.getPreferredSystemLanguages()?.[0]` (line 121), passed through `processZhLocale()` to normalise zh-hans/zh-hant/zh-XY codes (line 660–681). If `userLocale` is defined from argv.json or `--locale`, `resolveNLSConfiguration(...)` is called immediately as a promise (line 124–130) before `ready`.

**Electron locale switch (lines 141–144)**
On Windows / Linux, `app.commandLine.appendSwitch('lang', electronLocale)` is appended; `qps-ploc` (Microsoft Pseudo Language) is mapped to `'en'`.

**`configureCommandlineSwitchesSync` (line 215)**
Reads `argv.json` synchronously via `readArgvConfigSync()` (line 380). Iterates keys:
- `SUPPORTED_ELECTRON_SWITCHES` keys → `app.commandLine.appendSwitch()` or `app.disableHardwareAcceleration()` for `disable-hardware-acceleration` (line 267).
- `password-store: gnome|gnome-keyring` → migrated to `gnome-libsecret` (lines 274–279).
- `SUPPORTED_MAIN_PROCESS_SWITCHES` keys → pushed to `process.argv` (e.g. `--log`, `--use-inmemory-secretstorage`, `--enable-rdp-display-tracking`).
- Chromium `enable-features` (line 328): appends `NetAdapterMaxBufSizeFeature:...,DocumentPolicyIncludeJSCallStacksInCrashReports,EarlyEstablishGpuChannel,EstablishGpuChannelAsync`.
- `disable-features` (line 334): appends `CalculateNativeWinOcclusion`.
- `disable-blink-features` (line 341): appends `FontMatchingCTMigration,StandardizedBrowserZoom`.
- `xdg-portal-required-version` → `'4'` (line 353).
- `max-active-webgl-contexts` → `'32'` (line 357).
- JS flags merged from CLI and argv.json; on Linux adds `--nodecommit_pooled_pages` (line 563).

**`readArgvConfigSync` (line 380)**
Reads `argv.json` via `original-fs` (bypasses Electron's overlay FS). Calls `createDefaultArgvConfigSync` (line 403) if the file does not exist, writing a commented template.

**`app.once('ready', ...)` handler (line 147)**
If `--trace` is given, `contentTracing.startRecording(...)` is called first (line 179), then `onReady()`.

**`onReady()` (line 185)**
- Marks `code/mainAppReady`.
- Concurrently: `mkdirpIgnoreError(codeCachePath)` + `resolveNlsConfiguration()`.
- Then calls `startup(codeCachePath, nlsConfig)`.

**`startup()` (line 203)**
- Sets `VSCODE_NLS_CONFIG` (stringified JSON) and `VSCODE_CODE_CACHE_PATH` in `process.env`.
- Calls `bootstrapESM()` (awaits NLS message load).
- Dynamic `import('./vs/code/electron-main/main.js')` (line 211) loads `CodeMain`.
- Marks `code/didRunMainBundle`.

---

#### `src/bootstrap-node.ts`

- **`Error.stackTraceLimit = 100`** (line 15): increases V8 stack depth.
- **SIGPIPE handler** (lines 17–30): installs a process-level `SIGPIPE` listener that logs once and prevents infinite error loops in broken-pipe scenarios.
- **`setupCurrentWorkingDirectory()`** (line 35): sets `VSCODE_CWD` to `process.cwd()` once; on Windows calls `process.chdir(path.dirname(process.execPath))` to anchor the CWD to the application folder.
- **`devInjectNodeModuleLookupPath(injectPath)`** (line 62): only active under `VSCODE_DEV`; registers an ESM loader hook via `Module.register('./bootstrap-import.js', ...)` to redirect module resolution.
- **`removeGlobalNodeJsModuleLookupPaths()`** (line 76): patches `Module._resolveLookupPaths` and `Module._nodeModulePaths` to strip global and users-dir paths from resolution, narrowing the module search scope.
- **`configurePortable(product)`** (line 133): resolves `portableDataPath`, checks for the `data/` directory's existence, sets `VSCODE_PORTABLE` / temp env vars, returns `{ portableDataPath, isPortable }`.

---

#### `src/bootstrap-esm.ts`

- **`fs` → `original-fs` hook** (lines 14–30): when running inside Electron, registers an in-memory ESM loader that intercepts `import 'fs'` and redirects to `node:original-fs`, avoiding Electron's patched `fs`.
- **Global setup** (lines 33–35): copies `product` and `pkg` into `globalThis._VSCODE_PRODUCT_JSON` / `globalThis._VSCODE_PACKAGE_JSON`; sets `globalThis._VSCODE_FILE_ROOT` to the source directory path.
- **`doSetupNLS()`** (line 49): reads `VSCODE_NLS_CONFIG` from env, determines the `messagesFile` (from `languagePack.messagesFile` or `defaultMessagesFile`), and loads it into `globalThis._VSCODE_NLS_MESSAGES` via `fs.promises.readFile`. On read failure, writes a corrupt marker file and falls back to the default messages file. Marks `code/willLoadNls` and `code/didLoadNls`.
- **`bootstrapESM()`** (line 108): exported async function, sole body is `await setupNLS()`.

---

#### `src/bootstrap-meta.ts`

- **Product JSON** (lines 12–15): `productObj` is a sentinel object with key `BUILD_INSERT_PRODUCT_CONFIGURATION`. At build time this sentinel is replaced with the actual product JSON inline. Out of sources it falls through to `require('../product.json')`.
- **Package JSON** (lines 17–20): same sentinel pattern for `pkgObj`, falls through to `require('../package.json')` out of sources.
- **Embedded app sub-files** (lines 23–44): if `process.isEmbeddedApp`, preserves the parent policy config, then loads `product.sub.json` and `package.sub.json` via `require`, merging them with `Object.assign`.
- **Dev overrides** (lines 46–52): if `VSCODE_DEV`, loads `product.overrides.json` and merges it last.
- Exports `product` and `pkg` (lines 54–55).

---

#### `src/vs/code/electron-main/main.ts` — `CodeMain` class

**Entry (lines 652–654)**
Module-level: instantiates `new CodeMain()` and calls `.main()`.

**`main()` (line 88)**
Calls `this.startup()` inside a try/catch; on error logs to console and calls `app.exit(1)`.

**`startup()` (line 97)**
1. Installs `setUnexpectedErrorHandler` to log errors instead of showing Electron dialog.
2. `createServices()` (line 162) builds a `ServiceCollection` with: `IProductService`, `IEnvironmentMainService`, `ILoggerMainService`, `ILogService` (buffered), `IFileService` + `DiskFileSystemProvider`, `IUriIdentityService`, `IStateService` (DELAYED save strategy), `IUserDataProfilesMainService`, `FileUserDataProvider` for `vscode-user-data` scheme, `IPolicyService` (platform-specific: `NativePolicyService` on Windows/macOS, `FilePolicyService` on Linux, `NullPolicyService` otherwise), `IConfigurationService`, `ILifecycleMainService`, `IRequestService`, `IThemeMainService`, `ISignService`, `ITunnelService`, `IProtocolMainService`.
3. `initServices()` (line 270): in parallel, creates all required directories (`extensionsPath`, `logsHome`, `globalStorageHome`, `workspaceStorageHome`, `localHistoryHome`, `backupHome`), initialises `StateService` and `ConfigurationService`.
4. `claimInstance()` (line 306): attempts `nodeIPCServe(environmentMainService.mainIPCHandle)`. On `EADDRINUSE`, connects as a client (`nodeIPCConnect`) and forwards the current args/env to the existing instance via `ILaunchMainService.start()`, then throws `ExpectedError` to terminate.
5. Writes `mainLockfile` (line 133), wires spdlog logger to the buffer logger.
6. Checks `checkInnoSetupMutex()` on Windows (line 148) — if active, quits immediately.
7. `instantiationService.createInstance(CodeApplication, mainProcessNodeIpcServer, instanceEnvironment).startup()` (line 155) — hands off to `CodeApplication`.

**`resolveArgs()` / `validatePaths()` / `doValidatePaths()` (lines 518–647)**
Parses `process.argv` with `parseMainProcessArgv`, handles `--wait` marker file creation, maps `--chat` sub-args, normalises and sanitises file paths (trims quotes and whitespace on Windows, resolves relative to cwd, deduplicates case-insensitively).

---

### Cross-Cutting Synthesis

`src/main.ts` operates as a two-phase synchronous/asynchronous bootstrap. The synchronous phase runs entirely before Electron's `ready` event: it calls `configurePortable`, parses CLI args, decides on sandbox mode (`app.enableSandbox()` or GPU-sandbox switches), sets `userData` and optionally `crashDumps`/`AppLogs` paths, registers custom URL schemes (`vscode-webview://`, `vscode-file://`) with `protocol.registerSchemesAsPrivileged`, suppresses the default menu, installs macOS open-file/open-url listeners, and fires off an early NLS resolution promise if a user locale is known. The asynchronous phase starts in `app.once('ready')`, forks into optional content tracing, then `onReady()` concurrently creates the code-cache directory and resolves the final NLS configuration. `startup()` injects `VSCODE_NLS_CONFIG` and `VSCODE_CODE_CACHE_PATH` into `process.env`, calls `bootstrapESM()` (which loads NLS messages into `globalThis` and installs the `fs`→`original-fs` hook), and then dynamically imports `vs/code/electron-main/main.js`. That module immediately instantiates `CodeMain`, which builds the full dependency-injection service container, claims the singleton IPC socket (or forwards to an existing instance), and delegates to `CodeApplication`. The four supporting modules form a strict hierarchy: `bootstrap-meta.ts` provides build-time product/package identity; `bootstrap-node.ts` sets up the Node.js environment (CWD, SIGPIPE, portable mode, module path narrowing); `bootstrap-esm.ts` installs the Electron-specific ESM hooks and materialises NLS strings; and `vs/code/electron-main/main.ts` owns all VS Code application services and the single-instance arbitration protocol.

---

### Out-of-Partition References

The following files are directly called from the analysed partition but fall outside its scope:

- `/Users/norinlavaee/vscode-atomic/src/vs/platform/environment/node/userDataPath.ts` — `getUserDataPath()` used at `main.ts:57`
- `/Users/norinlavaee/vscode-atomic/src/vs/base/node/nls.ts` — `resolveNLSConfiguration()` used at `main.ts:124`, `main.ts:717`
- `/Users/norinlavaee/vscode-atomic/src/vs/base/node/unc.ts` — `getUNCHost()`, `addUNCHostToAllowlist()` used at `main.ts:59–61`
- `/Users/norinlavaee/vscode-atomic/src/vs/base/common/performance.ts` — `perf.mark()` called throughout `main.ts`
- `/Users/norinlavaee/vscode-atomic/src/vs/base/common/jsonc.ts` — `parse()` used to read `argv.json` at `main.ts:386`
- `/Users/norinlavaee/vscode-atomic/src/vs/code/electron-main/app.ts` — `CodeApplication` instantiated at `electron-main/main.ts:155`
- `/Users/norinlavaee/vscode-atomic/src/vs/platform/protocol/electron-main/protocolMainService.ts` — `ProtocolMainService` (handles `vscode-file://` / `vscode-webview://` at the renderer layer) at `electron-main/main.ts:248`
- `/Users/norinlavaee/vscode-atomic/src/vs/platform/lifecycle/electron-main/lifecycleMainService.ts` — `LifecycleMainService` registered at `electron-main/main.ts:233`
- `/Users/norinlavaee/vscode-atomic/src/vs/platform/environment/electron-main/environmentMainService.ts` — `EnvironmentMainService` with all path derivations at `electron-main/main.ts:172`
- `/Users/norinlavaee/vscode-atomic/src/bootstrap-import.js` — ESM loader hook data URL target, registered from `bootstrap-node.ts:73`
