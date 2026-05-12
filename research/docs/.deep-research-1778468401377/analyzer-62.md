### Files Analysed

- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-extension-common.mts` (51 LOC) — primary scope file
- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-common.mts` (80 LOC) — imported dependency providing `RunConfig` and `runBuild`
- `/home/norinlavaee/projects/vscode-atomic/extensions/git/esbuild.mts` — representative node-platform consumer
- `/home/norinlavaee/projects/vscode-atomic/extensions/typescript-language-features/esbuild.browser.mts` — representative browser-platform consumer

---

### Per-File Notes

#### `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-extension-common.mts`

**Role**
Acts as a thin, platform-aware configuration layer sitting between individual extension build scripts and the lower-level `runBuild` runner in `esbuild-common.mts`. It is the single place where node-vs-browser esbuild divergence is encoded.

**Imports (lines 8–9)**
- `esbuild` (npm) — used only for its `BuildOptions` type; no direct esbuild API calls happen here.
- `runBuild`, `RunConfig` from `./esbuild-common.mts` — the actual build/watch runner.

**`ExtensionRunConfig` interface (lines 11–14)**
Extends `RunConfig` (which carries `srcDir`, `outdir`, `entryPoints`, and optional `additionalOptions`) with two fields:
- `platform: 'node' | 'browser'` — required discriminant controlling all platform-specific logic.
- `format?: 'cjs' | 'esm'` — optional output module format, defaults to `'cjs'` if omitted (enforced at line 25).

**`resolveBaseOptions(config)` (lines 16–46)**
Builds an `esbuild.BuildOptions` object that is common to every extension build:

| Option | Value | Line |
|---|---|---|
| `platform` | `config.platform` | 18 |
| `bundle` | `true` | 19 |
| `minify` | `true` | 20 |
| `treeShaking` | `true` | 21 |
| `sourcemap` | `true` | 22 |
| `target` | `['es2024']` | 23 |
| `external` | `['vscode']` | 24 |
| `format` | `config.format ?? 'cjs'` | 25 |
| `logOverride['import-is-undefined']` | `'error'` | 27–28 |

Then a platform branch runs (lines 31–43):

- **Node branch (lines 31–32):** Sets `mainFields = ['module', 'main']`. This prefers ESM exports over CJS when a package ships both.
- **Browser branch (lines 33–43):**
  - `mainFields = ['browser', 'module', 'main']` (line 34) — prefers the `browser` field in `package.json` for web-safe polyfills.
  - `alias = { 'path': 'path-browserify' }` (lines 35–37) — rewrites all `import 'path'` to the browser-compatible polyfill.
  - `define` (lines 38–42) — inlines three compile-time constants:
    - `process.platform` → `"web"`
    - `process.env` → `{}`
    - `process.env.BROWSER_ENV` → `"true"`

**`run(config, args, didBuild?)` (lines 48–50)**
The sole export. Takes an `ExtensionRunConfig`, the raw `process.argv` array, and an optional post-build callback. Calls `runBuild(config, resolveBaseOptions(config), args, didBuild)` — forwarding both the config (for `srcDir`/`outdir`/`entryPoints`/`additionalOptions`) and the resolved base options. No logic of its own; it is purely a composition point.

---

#### `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-common.mts`

**Role**
Generic build-and-watch runner consumed by `esbuild-extension-common.mts` and, directly, any extension build script that needs lower-level control.

**`RunConfig` interface (lines 8–13)**
- `srcDir: string` — root directory watched by `@parcel/watcher` in watch mode.
- `outdir: string` — output directory; may be overridden at runtime via `--outputRoot` CLI arg.
- `entryPoints: esbuild.BuildOptions['entryPoints']` — map or array of entry files.
- `additionalOptions?: Partial<esbuild.BuildOptions>` — merged last, so callers can override any base option.

**`runBuild(config, baseOptions, args, didBuild?)` (lines 18–51)**

1. **Output directory resolution (lines 24–30):** Checks `args` for `--outputRoot <path>`. If present, takes the basename of `config.outdir` and joins it onto the provided `outputRoot`, replacing the configured path. This supports the VS Code build system's ability to redirect output without changing each extension's config.

2. **Options merging (lines 32–37):**
   ```
   resolvedOptions = { ...baseOptions, entryPoints, outdir, ...additionalOptions }
   ```
   `additionalOptions` is spread last, giving individual extensions the ability to override any field set by `resolveBaseOptions` (e.g., the git extension overrides `external` to add `'@vscode/fs-copyfile'` at `extensions/git/esbuild.mts:35–37`).

3. **Mode switch (lines 39–50):**
   - **Watch mode** (`--watch` in args, line 39): Calls `esbuild.context(resolvedOptions)` then hands the context to `watchWithParcel`.
   - **Build mode** (default, lines 44–49): Calls `esbuild.build(resolvedOptions)` then invokes `didBuild?.(outdir)`. Calls `process.exit(1)` on any esbuild error.

**`watchWithParcel(ctx, srcDir, didBuild?)` (lines 54–80)**
Uses `@parcel/watcher` (dynamically imported at line 73 to keep it optional) instead of esbuild's built-in watch. A 100 ms debounce (lines 55–71) prevents cascading rebuilds on rapid file saves. On each debounce fire: cancels any in-flight build (`ctx.cancel()`), calls `ctx.rebuild()`, and invokes `didBuild` only if `result.errors.length === 0`. The watcher ignores `node_modules`, `dist`, and `out` directories (line 77).

---

#### Consumer Pattern (51 extension build scripts)

All 51 consumer scripts follow the same pattern, illustrated by `extensions/git/esbuild.mts`:

```
import { run } from '../esbuild-extension-common.mts';
run({ platform, entryPoints, srcDir, outdir, additionalOptions? }, process.argv, optionalPostBuildCallback);
```

Browser-targeting extensions (e.g., `extensions/typescript-language-features/esbuild.browser.mts`) may call `run()` multiple times in parallel (via `Promise.all`) to produce separate bundles from different entry points or tsconfigs.

---

### Cross-Cutting Synthesis

`esbuild-extension-common.mts` is the standardization layer for all VS Code built-in extension builds. It encodes two invariants in one place: (1) every extension bundle is minified, tree-shaken, ES2024-targeted, and externalises the `vscode` API module; (2) the `node` vs `browser` platform split is handled by a single `if/else` branch that sets `mainFields`, the `path` alias, and three `process.*` compile-time defines. The actual build machinery lives one level down in `esbuild-common.mts`, which adds the `--outputRoot` CLI override, the options-merge order (base then additionalOptions), and the `@parcel/watcher`-powered debounced watch mode. The 51 consumer scripts are uniform wrappers: they supply entry points, directories, and an optional post-build hook (for copying non-TS assets or TypeScript lib `.d.ts` files) and delegate everything else to `run()`. This architecture means build behaviour changes — target version, minification policy, external modules, watch debounce — can be made in one or two files rather than in each of the 51 extension build scripts.

---

### Out-of-Partition References

- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-common.mts` — `RunConfig` interface and `runBuild` function imported directly at line 9 of the scope file.
- `esbuild` npm package — `BuildOptions` type used throughout `resolveBaseOptions`.
- `@parcel/watcher` npm package — dynamically imported inside `watchWithParcel` in `esbuild-common.mts:73`; not referenced in the scope file itself but is a transitive runtime dependency of every `run()` call in watch mode.
- All 51 `extensions/*/esbuild.mts` and `extensions/*/esbuild.browser.mts` files — consumers of the exported `run()` function. Representative files read: `extensions/git/esbuild.mts` and `extensions/typescript-language-features/esbuild.browser.mts`.
