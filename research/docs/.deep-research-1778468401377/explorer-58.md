# Partition 58 of 80 — Findings

## Scope
`extensions/esbuild-common.mts/` (1 files, 80 LOC)

## Files in Scope
<!-- Source: codebase-locator sub-agent -->
# File Locator Report: `extensions/esbuild-common.mts`

## Implementation

**Core Bundler Pipeline**
- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-common.mts` (80 lines) — Central build/watch runner for extension esbuild scripts. Exports two main APIs:
  - `RunConfig` interface: Specifies source directory, output directory, entry points, and optional additional esbuild options
  - `runBuild()` async function: Orchestrates either single-pass esbuild compilation or watch-mode with parcel watcher, with optional post-build callback

**Watch Mode Implementation**
- Uses `@parcel/watcher` (v2.5.6) instead of esbuild's native watch for reduced idle CPU usage
- Implements debounced rebuild pattern (100ms debounce) when source files change
- Ignores `node_modules/**`, `dist/**`, `out/**` directories during watch

**Type Definitions**
- Minimal type interface `RunConfig` with required fields: `srcDir`, `outdir`, `entryPoints` (esbuild format), optional `additionalOptions`
- Relies on esbuild's own types for `BuildOptions` type hints

## Configuration

**Dependencies** (from `/home/norinlavaee/projects/vscode-atomic/extensions/package.json`)
- `esbuild@0.27.2` — Main bundler
- `@parcel/watcher@^2.5.6` — File watching for lower CPU overhead
- `typescript@^6.0.3` — For TS/MTS compilation support
- `node-gyp-build@4.8.1` — Override for native module builds

**Runtime Arguments**
- `--watch` flag: Activates watch mode instead of one-shot build
- `--outputRoot` flag: Overrides output directory root, preserving the original directory name

## Examples / Fixtures

**Consumer Pattern** — Extension build scripts (e.g., `/home/norinlavaee/projects/vscode-atomic/extensions/git/esbuild.mts`)
```
Imports: run() from esbuild-extension-common (which wraps esbuild-common)
Calls with: RunConfig + process.argv + optional post-build callback
Typical usage: Define entry points (main, editor plugins), source/out directories, handle non-TS files separately
```

**Wrapper Modules**
- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-extension-common.mts` (51 lines) — Adds platform-specific (node/browser) and format (cjs/esm) configuration, re-exports runBuild()
- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-webview-common.mts` (29 lines) — Browser-optimized wrapper with ESM/browser platform defaults, used by webview build scripts

**Individual Extension Build Scripts**
74+ extension-specific esbuild scripts found across:
- `extensions/*/esbuild.mts` — Node platform bundles (git, github, markdown, etc.)
- `extensions/*/esbuild.browser.mts` — Browser platform bundles
- `extensions/*/esbuild.webview.mts` — Webview-specific bundles
- `extensions/*/esbuild.notebook.mts` — Notebook renderer bundles
Examples: git/, simple-browser/, markdown-language-features/, typescript-language-features/, etc.

## Notable Clusters

**Extension Build System Architecture**
- **Root**: `esbuild-common.mts` (generic run/watch orchestration)
- **Mid-layer**: `esbuild-extension-common.mts` + `esbuild-webview-common.mts` (platform/format defaults)
- **Leaf nodes**: 74+ extension-specific esbuild.mts scripts (entry point definitions, custom post-build hooks)

**Build Modes**
- Single-pass compilation: `esbuild.build()` for CI/release
- Watch mode: parcel-based file monitoring with debounced rebuilds for development
- Both routed through same `runBuild()` dispatcher

**Output Flexibility**
- Default: `config.outdir` (usually `dist/`)
- Override: `--outputRoot` flag reroutes output while preserving directory structure

## Documentation

No dedicated test files, README, or documentation found within the scope. The module is self-documented via TypeScript interfaces and implementation comments indicating it is a "Shared build/watch runner for extension esbuild scripts."

---

## Porting Implications

For a Tauri/Rust port of VS Code, **the esbuild infrastructure remains necessary** (extensions still compile from TS/JS to bundled modules). This module would need equivalent functionality in Rust's build system:

1. **Core abstraction** (RunConfig + runBuild) → Rust struct + async function wrapping build logic
2. **Watch mode** → Rust file-watching library (notify, watchexec, or similar) replacing parcel
3. **Platform/format variants** → Conditional compilation or config structs for node vs. browser bundles
4. **Post-build hooks** → Async callback pattern for asset copying, code generation, etc.
5. **CLI arg parsing** → Structured argument handling (--watch, --outputRoot) in Rust

The esbuild bundler itself would remain external (Node.js subprocess or WASM), unless a pure Rust bundler (swc, turbopack) is adopted—but that decision impacts all 74+ extension build scripts.

## How It Works
<!-- Source: codebase-analyzer sub-agent -->
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

## Patterns
<!-- Source: codebase-pattern-finder sub-agent -->
# VS Code Extension Bundling with esbuild: Core Patterns

## Summary

VS Code's extension bundling architecture centers on a shared esbuild abstraction (`esbuild-common.mts`) that provides unified build/watch logic for all ~33 extensions. All patterns call `esbuild.build()` for one-shot builds and `esbuild.context()` for watch mode. The architecture demonstrates how a Tauri/Rust port would still require a JS bundler: extensions are written in TypeScript and must be bundled to JavaScript for distribution. The patterns show configuration composition, plugin architecture, and parallel multi-target builds (node/browser).

---

## Pattern Examples: esbuild Integration in Extension Bundling

#### Pattern 1: Shared Build Runner with Watch/Build Modes
**Where:** `extensions/esbuild-common.mts:18-51`
**What:** Core abstraction providing unified build/watch orchestration for all extensions. Accepts config, switches between `esbuild.build()` (one-shot) and `esbuild.context()` + watch (incremental).

```typescript
export async function runBuild(
	config: RunConfig,
	baseOptions: esbuild.BuildOptions,
	args: string[],
	didBuild?: (outDir: string) => unknown,
): Promise<void> {
	let outdir = config.outdir;
	const outputRootIndex = args.indexOf('--outputRoot');
	if (outputRootIndex >= 0) {
		const outputRoot = args[outputRootIndex + 1];
		const outputDirName = path.basename(outdir);
		outdir = path.join(outputRoot, outputDirName);
	}

	const resolvedOptions: esbuild.BuildOptions = {
		...baseOptions,
		entryPoints: config.entryPoints,
		outdir,
		...(config.additionalOptions || {}),
	};

	const isWatch = args.indexOf('--watch') >= 0;
	if (isWatch) {
		const ctx = await esbuild.context(resolvedOptions);
		await watchWithParcel(ctx, config.srcDir, () => didBuild?.(outdir));
	} else {
		try {
			await esbuild.build(resolvedOptions);
			await didBuild?.(outdir);
		} catch {
			process.exit(1);
		}
	}
}
```

**Variations / call-sites:**
- Used by `esbuild-extension-common.mts:49` via `runBuild()` wrapper
- Called directly by 33+ extension build scripts
- `didBuild` callback used for post-build actions (e.g., copying files, moving source maps)

---

#### Pattern 2: Platform-Aware Base Configuration Composition
**Where:** `extensions/esbuild-extension-common.mts:16-46`
**What:** Wraps `runBuild()` with extension-specific defaults, adding platform-aware configuration (node vs. browser). Platform determines mainFields, aliases, and environment defines.

```typescript
function resolveBaseOptions(config: ExtensionRunConfig): esbuild.BuildOptions {
	const options: esbuild.BuildOptions = {
		platform: config.platform,
		bundle: true,
		minify: true,
		treeShaking: true,
		sourcemap: true,
		target: ['es2024'],
		external: ['vscode'],
		format: config.format ?? 'cjs',
		logOverride: {
			'import-is-undefined': 'error',
		},
	};

	if (config.platform === 'node') {
		options.mainFields = ['module', 'main'];
	} else if (config.platform === 'browser') {
		options.mainFields = ['browser', 'module', 'main'];
		options.alias = {
			'path': 'path-browserify',
		};
		options.define = {
			'process.platform': JSON.stringify('web'),
			'process.env': JSON.stringify({}),
			'process.env.BROWSER_ENV': JSON.stringify('true'),
		};
	}

	return options;
}
```

**Variations / call-sites:**
- Used by `esbuild-extension-common.mts:49` `run()` export
- All 33 extensions use `run()` indirectly; only complex builds (copilot, html-language-features) override with custom options

---

#### Pattern 3: Simple Extension Build (Single Entry Point)
**Where:** `extensions/configuration-editing/esbuild.mts:11-18`
**What:** Minimal extension pattern—delegates to `run()` helper with single entry point and platform specification.

```typescript
run({
	platform: 'node',
	entryPoints: {
		'configurationEditingMain': path.join(srcDir, 'configurationEditingMain.ts'),
	},
	srcDir,
	outdir: outDir,
}, process.argv);
```

**Variations / call-sites:**
- Standard pattern for ~25 extensions (git, markdown-language-features, json-language-features client, etc.)
- Some extend with `additionalOptions` for externals or tsconfigPath (git, markdown-language-features)

---

#### Pattern 4: Multi-Target Parallel Build
**Where:** `extensions/json-language-features/esbuild.mts:10-43`
**What:** Single build script compiling client and server as separate targets in parallel using `Promise.all()` and different platforms/formats.

```typescript
await Promise.all([
	// Build client
	run({
		platform: 'node',
		entryPoints: {
			'jsonClientMain': path.join(extensionRoot, 'client', 'src', 'node', 'jsonClientMain.ts'),
		},
		srcDir: path.join(extensionRoot, 'client', 'src'),
		outdir: path.join(extensionRoot, 'client', 'dist', 'node'),
		additionalOptions: {
			tsconfig: path.join(extensionRoot, 'client', 'tsconfig.json'),
		},
	}, process.argv),

	// Build server
	run({
		platform: 'node',
		format: 'esm',
		entryPoints: {
			'jsonServerMain': path.join(extensionRoot, 'server', 'src', 'node', 'jsonServerNodeMain.ts'),
		},
		srcDir: path.join(extensionRoot, 'server', 'src'),
		outdir: path.join(extensionRoot, 'server', 'dist', 'node'),
		additionalOptions: {
			tsconfig: path.join(extensionRoot, 'server', 'tsconfig.json'),
			external: ['vscode', 'typescript', 'fs'],
			banner: {
				js: `import { createRequire } from 'module'; const require = createRequire(import.meta.url);`,
			},
		},
	}, process.argv),
]);
```

**Variations / call-sites:**
- Used by json-language-features, html-language-features, markdown-language-features (client/server splits)
- Demonstrates format override (`esm` vs `cjs`), custom tsconfig, and ESM compatibility banners

---

#### Pattern 5: Post-Build Callbacks for File Operations
**Where:** `extensions/git/esbuild.mts:12-38` + `extensions/esbuild-common.mts:44-51`
**What:** Async callback passed to `run()` for non-TS file copying after bundling. Extends `runBuild()` with custom `didBuild` handler.

```typescript
// git/esbuild.mts
async function copyNonTsFiles(outDir: string): Promise<void> {
	const entries = await fs.readdir(srcDir, { withFileTypes: true, recursive: true });
	for (const entry of entries) {
		if (!entry.isFile() || entry.name.endsWith('.ts')) {
			continue;
		}
		const srcPath = path.join(entry.parentPath, entry.name);
		const relativePath = path.relative(srcDir, srcPath);
		const destPath = path.join(outDir, relativePath);
		await fs.mkdir(path.dirname(destPath), { recursive: true });
		await fs.copyFile(srcPath, destPath);
	}
}

run({
	platform: 'node',
	entryPoints: {
		'main': path.join(srcDir, 'main.ts'),
		'askpass-main': path.join(srcDir, 'askpass-main.ts'),
		'git-editor-main': path.join(srcDir, 'git-editor-main.ts'),
	},
	srcDir,
	outdir: outDir,
	additionalOptions: {
		external: ['vscode', '@vscode/fs-copyfile'],
	},
}, process.argv, copyNonTsFiles);
```

**Variations / call-sites:**
- Also used: markdown-language-features (copy server worker JS from npm), json-language-features
- Enables WASM, binary assets, or static files alongside bundled JS

---

#### Pattern 6: Custom esbuild Plugins for Compilation
**Where:** `extensions/html-language-features/esbuild.browser.mts:17-83`
**What:** Advanced pattern showing esbuild plugin API for inline code generation. Custom plugin (`javaScriptLibsPlugin`) inlines TypeScript lib definitions at build time for browser platform.

```typescript
function javaScriptLibsPlugin(): esbuild.Plugin {
	return {
		name: 'javascript-libs',
		setup(build) {
			build.onLoad({ filter: /javascriptLibs\.ts$/ }, () => {
				const TYPESCRIPT_LIB_SOURCE = path.dirname(import.meta.resolve('typescript').replace('file://', ''));
				const JQUERY_DTS = path.join(extensionRoot, 'server', 'lib', 'jquery.d.ts');

				function getFileName(name: string): string {
					return name === '' ? 'lib.d.ts' : `lib.${name}.d.ts`;
				}

				function readLibFile(name: string): string {
					return fs.readFileSync(path.join(TYPESCRIPT_LIB_SOURCE, getFileName(name)), 'utf8');
				}

				const queue: string[] = [];
				const inQueue: Record<string, boolean> = {};

				function enqueue(name: string): void {
					if (inQueue[name]) {
						return;
					}
					inQueue[name] = true;
					queue.push(name);
				}

				enqueue('es2020.full');

				const result: { name: string; content: string }[] = [];
				while (queue.length > 0) {
					const name = queue.shift()!;
					const contents = readLibFile(name);
					const lines = contents.split(/\r\n|\r|\n/);

					const outputLines: string[] = [];
					for (const line of lines) {
						const m = line.match(/\/\/\/\s*<reference\s*lib="([^"]+)"/);
						if (m) {
							enqueue(m[1]);
						}
						outputLines.push(line);
					}

					result.push({
						name: getFileName(name),
						content: outputLines.join('\n'),
					});
				}

				const jquerySource = fs.readFileSync(JQUERY_DTS, 'utf8');
				result.push({
					name: 'jquery',
					content: jquerySource,
				});

				let code = `const libs = {};\n`;
				for (const entry of result) {
					code += `libs[${JSON.stringify(entry.name)}] = ${JSON.stringify(entry.content)};\n`;
				}
				code += `export function loadLibrary(name) { return libs[name] || ''; }\n`;

				return { contents: code, loader: 'js' };
			});
		},
	};
}

run({
	platform: 'browser',
	format: 'esm',
	entryPoints: {
		'htmlServerMain': path.join(extensionRoot, 'server', 'src', 'browser', 'htmlServerWorkerMain.ts'),
	},
	srcDir: path.join(extensionRoot, 'server', 'src'),
	outdir: path.join(extensionRoot, 'server', 'dist', 'browser'),
	additionalOptions: {
		tsconfig: path.join(extensionRoot, 'server', 'tsconfig.browser.json'),
		plugins: [javaScriptLibsPlugin()],
	},
}, process.argv),
```

**Variations / call-sites:**
- Only html-language-features uses custom plugins (requires special TypeScript lib bundling)
- Copilot/.esbuild.mts has testBundlePlugin, sanityTestBundlePlugin, importMetaPlugin (advanced testing infrastructure)
- Plugin system allows arbitrary code generation or asset transformation during bundling

---

#### Pattern 7: Watch Mode with Parcel Watcher (Low CPU)
**Where:** `extensions/esbuild-common.mts:54-80`
**What:** Custom watch implementation using `@parcel/watcher` instead of esbuild's native watch. Reduces CPU when idle via debouncing. Creates `esbuild.BuildContext` and calls `ctx.rebuild()`.

```typescript
async function watchWithParcel(ctx: esbuild.BuildContext, srcDir: string, didBuild?: () => Promise<unknown> | unknown): Promise<void> {
	let debounce: ReturnType<typeof setTimeout> | undefined;
	const rebuild = () => {
		if (debounce) {
			clearTimeout(debounce);
		}
		debounce = setTimeout(async () => {
			try {
				await ctx.cancel();
				const result = await ctx.rebuild();
				if (result.errors.length === 0) {
					await didBuild?.();
				}
			} catch (error) {
				console.error('[watch] build error:', error);
			}
		}, 100);
	};

	const watcher = await import('@parcel/watcher');
	await watcher.subscribe(srcDir, (_err, _events) => {
		rebuild();
	}, {
		ignore: ['**/node_modules/**', '**/dist/**', '**/out/**']
	});
	rebuild();
}
```

**Variations / call-sites:**
- Used by all 33 extensions via `runBuild()` when `--watch` flag is present
- Copilot also has parallel watch for multiple contexts (nodeExtHostContext, webExtHostContext, etc.)
- 100ms debounce prevents thrashing during rapid file saves

---

## Implications for Tauri/Rust Port

**JS Bundler Requirement:** Even in a Tauri/Rust port, extensions remain TypeScript/JavaScript and require bundling. This architecture could translate to:

1. **Bundler choice:** esbuild as external tool (via subprocess) or rewrite in Rust (e.g., swc, Turbopack)
2. **Configuration schema:** Rust struct mirroring `RunConfig` / `ExtensionRunConfig`
3. **Watch mode:** Parcel watcher is language-agnostic; could be wrapped in FFI or replaced with `notify` crate
4. **Plugin system:** esbuild plugins are JS functions; would need plugin API redesign or continued JS-based plugin runtime
5. **Build parallelization:** `Promise.all()` patterns map directly to Tokio `join!()` or `spawn()`

**Core assets:** The ~80 LOC `esbuild-common.mts` is the critical abstraction; its patterns (argument parsing, context creation, rebuild/build branching) are framework-agnostic and would be straightforward to port.

---

## File References

- **Core abstraction:** `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-common.mts` (80 LOC)
- **Extension wrapper:** `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-extension-common.mts` (51 LOC)
- **Simple examples:** configuration-editing, git, markdown-language-features, json-language-features
- **Complex example:** copilot/.esbuild.mts (480 LOC, multi-target with custom plugins)
- **Plugin example:** html-language-features/esbuild.browser.mts (114 LOC)

## Out-of-Partition References
Look for the **Out-of-Partition References** subsection inside the
"How It Works" section above — that is where the analyzer flagged files
outside this partition that other partitions should examine.
