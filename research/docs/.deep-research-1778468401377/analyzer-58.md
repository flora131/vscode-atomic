# Analyzer 58 — `extensions/esbuild-common.mts`

## Files Analysed

| File | LOC | Role |
|------|-----|------|
| `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-common.mts` | 80 | Central build/watch runner (primary scope) |
| `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-extension-common.mts` | 50 | Extension-specific wrapper that re-exports via `run()` |
| `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-webview-common.mts` | 29 | Webview-specific wrapper that re-exports via `run()` |
| `/home/norinlavaee/projects/vscode-atomic/extensions/package.json` | 20 | Shared `devDependencies` pinning esbuild and @parcel/watcher |

---

## Per-File Notes

### `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-common.mts`

#### Imports (lines 5–6)
- `path` from `node:path` — used for `path.basename` and `path.join` when rewriting the output directory under `--outputRoot`.
- `esbuild` from `esbuild` (version `0.27.2` per `extensions/package.json:14`) — used for `esbuild.context()`, `esbuild.build()`, and the type `esbuild.BuildOptions`.

#### Exported Interface: `RunConfig` (lines 8–13)
Four fields:
- `srcDir: string` — absolute path to the extension's source directory; passed directly to `@parcel/watcher` as the directory to subscribe to.
- `outdir: string` — base output directory; may be overridden at runtime via `--outputRoot` (lines 25–30).
- `entryPoints: esbuild.BuildOptions['entryPoints']` — entry-point map passed to esbuild without transformation.
- `additionalOptions?: Partial<esbuild.BuildOptions>` — optional escape hatch spread into the resolved options last (line 36), allowing per-extension overrides such as extra `external` identifiers or custom `loader` mappings.

#### Exported Function: `runBuild()` (lines 18–51)

Signature:
```typescript
export async function runBuild(
    config: RunConfig,
    baseOptions: esbuild.BuildOptions,
    args: string[],
    didBuild?: (outDir: string) => unknown,
): Promise<void>
```

**Step 1 — `--outputRoot` rewrite (lines 25–30).**
`args` is searched for `--outputRoot`. If found, the directory name portion of `config.outdir` is preserved via `path.basename` but re-joined under the provided root. This allows CI/build pipelines to redirect output without modifying the per-extension config objects.

**Step 2 — Options assembly (lines 32–37).**
A `resolvedOptions` object is composed by spreading `baseOptions` first, then `config.entryPoints` and the (potentially rewritten) `outdir`, then `config.additionalOptions`. The spread order means `additionalOptions` wins over everything.

**Step 3 — Mode dispatch (lines 39–50).**
`args` is searched for `--watch`. Two paths:
- **Watch path (line 40–42):** `esbuild.context(resolvedOptions)` creates an incremental build context, then `watchWithParcel` takes over (see below). `didBuild` is wrapped to receive `outdir`.
- **One-shot path (lines 44–49):** `esbuild.build(resolvedOptions)` is awaited directly. On success `didBuild?.(outdir)` is called. Any thrown error causes `process.exit(1)` (line 48), terminating the build script process without printing an additional message (esbuild already reports diagnostics).

#### Internal Function: `watchWithParcel()` (lines 54–80)

Signature:
```typescript
async function watchWithParcel(
    ctx: esbuild.BuildContext,
    srcDir: string,
    didBuild?: () => Promise<unknown> | unknown
): Promise<void>
```

This function replaces esbuild's own `--watch` mechanism with `@parcel/watcher` (version `^2.5.6`, `extensions/package.json:13`) due to lower CPU usage when idle (comment at line 53).

**Debounce closure (lines 55–71).**
A `debounce` variable of type `ReturnType<typeof setTimeout> | undefined` is closed over by the `rebuild` arrow function. Each call to `rebuild()` clears the previous timeout and sets a new 100 ms timer (line 60). The 100 ms delay coalesces rapid file-system events (e.g., multi-file saves) into a single rebuild.

**Rebuild sequence inside the timeout (lines 62–68):**
1. `ctx.cancel()` — aborts any in-progress incremental build.
2. `ctx.rebuild()` — triggers a new incremental compile.
3. If `result.errors.length === 0`, `didBuild?.()` is invoked (post-build callback, e.g. file-copy tasks).
4. Errors from the build or callback are caught and printed with the `[watch]` prefix (line 68), keeping the watcher alive.

**Watcher subscription (lines 73–78).**
`@parcel/watcher` is loaded via a dynamic `import()` (line 73) to avoid loading the native module in one-shot mode. `watcher.subscribe(srcDir, callback, { ignore: ['**/node_modules/**', '**/dist/**', '**/out/**'] })` registers the filesystem subscription. The ignore patterns prevent re-triggering on output artifacts. Every event fires `rebuild()`, which is also called once immediately after subscribing (line 79) to perform an initial build.

---

### `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-extension-common.mts`

This file is a one-layer adapter. It defines `ExtensionRunConfig` (lines 11–14) which extends `RunConfig` with two additional fields: `platform: 'node' | 'browser'` and optional `format?: 'cjs' | 'esm'`.

`resolveBaseOptions()` (lines 16–46) constructs the `esbuild.BuildOptions` base that is passed to `runBuild()`:
- Common flags: `bundle: true`, `minify: true`, `treeShaking: true`, `sourcemap: true`, `target: ['es2024']`, `external: ['vscode']`, `format: config.format ?? 'cjs'`.
- Node platform (lines 31–32): sets `mainFields: ['module', 'main']`.
- Browser platform (lines 33–43): sets `mainFields: ['browser', 'module', 'main']`, aliases `path` to `path-browserify`, and defines `process.platform`, `process.env`, and `process.env.BROWSER_ENV`.

Its exported `run()` (line 48) simply calls `runBuild(config, resolveBaseOptions(config), args, didBuild)`.

Consumed by: at least 31 leaf `esbuild.mts` scripts under `extensions/*/esbuild.mts` (e.g., `extensions/git/esbuild.mts:7`, `extensions/markdown-language-features/esbuild.mts:7`, `extensions/typescript-language-features/esbuild.mts:6`).

---

### `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-webview-common.mts`

Mirrors `esbuild-extension-common.mts` but hard-codes a browser/ESM base: `bundle: true`, `minify: true`, `sourcemap: false`, `format: 'esm'`, `platform: 'browser'`, `target: ['es2024']`. Note that `sourcemap` is `false` here (line 14) vs. `true` in the extension common.

Accepts the base `RunConfig` directly (no extension). Exports `run()` (line 23) which calls `runBuild()` with the hard-coded base options.

Consumed by: 7 webview/notebook build scripts — `extensions/simple-browser/esbuild.webview.mts`, `extensions/markdown-language-features/esbuild.webview.mts`, `extensions/markdown-language-features/esbuild.notebook.mts`, `extensions/markdown-math/esbuild.notebook.mts`, `extensions/ipynb/esbuild.notebook.mts`, `extensions/notebook-renderers/esbuild.notebook.mts`, `extensions/mermaid-chat-features/esbuild.webview.mts`.

---

## Data Flow Summary

```
process.argv
    │
    ▼
Leaf esbuild.mts (e.g. extensions/git/esbuild.mts)
    │  calls run({platform, entryPoints, srcDir, outdir, ...}, process.argv, didBuild?)
    ▼
esbuild-extension-common.mts::run()        OR    esbuild-webview-common.mts::run()
    │  resolves baseOptions                           │  uses hard-coded baseOptions
    └──────────────────────┬───────────────────────────┘
                           ▼
               esbuild-common.mts::runBuild()
                    │
                    ├── --outputRoot? → rewrite outdir
                    │
                    ├── --watch? ──yes──► esbuild.context() → watchWithParcel()
                    │                          │
                    │                    @parcel/watcher.subscribe(srcDir, ...)
                    │                          │ file event
                    │                    debounce 100ms
                    │                          │
                    │                    ctx.cancel() + ctx.rebuild()
                    │                          │ no errors
                    │                    didBuild?(outdir)
                    │
                    └── no ──────────────► esbuild.build()
                                               │ success
                                          didBuild?(outdir)
                                               │ error
                                          process.exit(1)
```

---

## Cross-Cutting Synthesis

In a Tauri/Rust port, VS Code's bundled JavaScript extensions remain a JavaScript concern regardless of the host runtime. The `esbuild-common.mts` module represents the single, narrow chokepoint through which every first-party extension's TypeScript source is compiled, tree-shaken, and written to a `dist/` directory. Because Tauri's webview still executes JavaScript and the VS Code extension host is itself a Node.js (or web) JavaScript runtime, esbuild's role does not change with the host technology: entry-point `.ts` files still need to be bundled into `.js` artifacts that the Rust-hosted webview or a separate extension-host process can load. The `--outputRoot` parameter at `esbuild-common.mts:25–30` is particularly relevant for a port because it allows the build pipeline to redirect compiled extension bundles to an arbitrary directory — a Tauri build system could inject a different `--outputRoot` pointing inside the Tauri `src-tauri/` or `resources/` tree without touching any per-extension script. The 100 ms debounce watch loop (`esbuild-common.mts:60`) and `@parcel/watcher` integration would continue to serve development rebuilds unchanged, since neither depends on Electron or any Node.js native API beyond `setTimeout`.

---

## Out-of-Partition References

| File | Relationship |
|------|-------------|
| `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-extension-common.mts` | Imports `runBuild` and `RunConfig` from `esbuild-common.mts` (line 9); adds `node`/`browser` platform logic; re-exports as `run()` |
| `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-webview-common.mts` | Imports `runBuild` and `RunConfig` from `esbuild-common.mts` (line 9); hard-codes ESM/browser/no-sourcemap base; re-exports as `run()` |
| `extensions/git/esbuild.mts` | Leaf script consuming `esbuild-extension-common.mts::run()` with `platform: 'node'` and 3 entry points |
| `extensions/markdown-language-features/esbuild.mts` | Leaf script consuming `esbuild-extension-common.mts::run()` with `platform: 'node'` and `didBuild` post-copy step |
| `extensions/typescript-language-features/esbuild.mts` | Minimal leaf script consuming `esbuild-extension-common.mts::run()` |
| `extensions/simple-browser/esbuild.webview.mts` | Leaf webview script consuming `esbuild-webview-common.mts::run()` with `.ttf` dataurl loader override |
| `extensions/markdown-language-features/esbuild.webview.mts` | Leaf webview script consuming `esbuild-webview-common.mts::run()` |
| `extensions/markdown-language-features/esbuild.notebook.mts` | Leaf notebook script consuming `esbuild-webview-common.mts::run()` |
| `extensions/markdown-math/esbuild.notebook.mts` | Leaf notebook script consuming `esbuild-webview-common.mts::run()` |
| `extensions/ipynb/esbuild.notebook.mts` | Leaf notebook script consuming `esbuild-webview-common.mts::run()` |
| `extensions/notebook-renderers/esbuild.notebook.mts` | Leaf notebook script consuming `esbuild-webview-common.mts::run()` |
| `extensions/mermaid-chat-features/esbuild.webview.mts` | Leaf webview script consuming `esbuild-webview-common.mts::run()` |
| `/home/norinlavaee/projects/vscode-atomic/extensions/package.json` | Pins `esbuild@0.27.2` (line 14) and `@parcel/watcher@^2.5.6` (line 13) as shared `devDependencies` for all extension build scripts |
