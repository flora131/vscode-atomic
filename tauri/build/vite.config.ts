/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

/**
 * Tauri Vite build configuration.
 *
 * Multi-target: set VSCODE_TARGET=desktop|server|server-web|web before
 * invoking, or pass --mode desktop etc. on the CLI.
 *
 *   npm run build:tauri                    # default: desktop
 *   npm run build:tauri -- --mode desktop
 *
 * Output lands in out-tauri/<target>/.
 *
 * Only the 'desktop' target is fully wired; other targets are stubbed with
 * representative entry points — see README.md for remaining work.
 */

import { defineConfig, type UserConfig } from 'vite';
import path from 'path';
import { pathToFileURL } from 'url';

import { nlsPlugin } from './plugins/nls.js';
import { manglePrivatesPlugin } from './plugins/mangle-privates.js';
import { productInjectionPlugin } from './plugins/product-injection.js';
import { builtinExtensionsPlugin } from './plugins/builtin-extensions.js';
import {
	type BuildTarget,
	getEntryPointsForTarget,
	getBootstrapEntryPointsForTarget,
} from './entrypoints.js';

// ---------------------------------------------------------------------------
// Resolve target
// ---------------------------------------------------------------------------

const REPO_ROOT = path.resolve(import.meta.dirname, '..', '..');

function resolveTarget(): BuildTarget {
	const envTarget = process.env['VSCODE_TARGET'];
	// Vite passes --mode as process.env.MODE via define; also check argv
	const modeArg = process.argv.find(a =>
		a === '--mode' || a === '-m'
	);
	const modeValue = modeArg
		? process.argv[process.argv.indexOf(modeArg) + 1]
		: undefined;

	const raw = envTarget ?? modeValue ?? 'desktop';
	const valid: BuildTarget[] = ['desktop', 'server', 'server-web', 'web'];
	if (!valid.includes(raw as BuildTarget)) {
		throw new Error(
			`VSCODE_TARGET must be one of: ${valid.join(', ')}. Got: "${raw}"`
		);
	}
	return raw as BuildTarget;
}

const target = resolveTarget();
const outDir = path.join(REPO_ROOT, 'out-tauri', target);

console.log(`[tauri/build] target=${target}  outDir=${outDir}`);

// ---------------------------------------------------------------------------
// Entry points → Rollup input map
// ---------------------------------------------------------------------------

const mainEntries = getEntryPointsForTarget(target);
const bootstrapEntries = getBootstrapEntryPointsForTarget(target);

/**
 * Build a Rollup `input` object from a list of src-relative entry point
 * names (e.g. 'vs/workbench/workbench.desktop.main').
 *
 * For entry points that exist in src/ we use the .ts source directly.
 * Bootstrap files live at repo root (main.ts, cli.ts, bootstrap-fork.ts).
 */
function buildInputMap(
	entries: string[],
	basePath: string
): Record<string, string> {
	const input: Record<string, string> = {};
	for (const ep of entries) {
		// Use the last path segment as key to avoid Rollup chunk name collisions
		const key = ep.replace(/\//g, '_');
		input[key] = path.join(basePath, `${ep}.ts`);
	}
	return input;
}

const srcDir = path.join(REPO_ROOT, 'src');

const inputMap: Record<string, string> = {
	...buildInputMap(mainEntries, srcDir),
	...buildInputMap(
		bootstrapEntries,
		REPO_ROOT // bootstrap files live at repo root
	),
};

// ---------------------------------------------------------------------------
// Stub input for non-desktop targets
// ---------------------------------------------------------------------------
// If the resolved entry files don't exist (non-desktop upstream modules aren't
// available in this workspace yet) we fall back to a minimal stub so Vite at
// least starts without "file not found" errors.
// The stub path is created at vite startup if needed.

import * as fs from 'fs';

const existingInput: Record<string, string> = {};
const missingEntries: string[] = [];

for (const [key, absPath] of Object.entries(inputMap)) {
	if (fs.existsSync(absPath)) {
		existingInput[key] = absPath;
	} else {
		missingEntries.push(absPath);
	}
}

if (missingEntries.length > 0) {
	console.warn(
		`[tauri/build] ${missingEntries.length} entry point(s) not found — ` +
		`they will be skipped. Run 'yarn compile' first or see README.md.`
	);
	if (missingEntries.length <= 5) {
		missingEntries.forEach(p => console.warn(`  missing: ${p}`));
	} else {
		missingEntries.slice(0, 5).forEach(p => console.warn(`  missing: ${p}`));
		console.warn(`  … and ${missingEntries.length - 5} more`);
	}
}

// If no entry points exist at all (fresh checkout, no compile yet) provide a
// minimal stub so `npm run build:tauri` doesn't hard-fail with 0 inputs.
let finalInput: Record<string, string> = existingInput;
if (Object.keys(existingInput).length === 0) {
	const stubPath = path.join(REPO_ROOT, 'out-tauri', '__stub.ts');
	fs.mkdirSync(path.dirname(stubPath), { recursive: true });
	if (!fs.existsSync(stubPath)) {
		fs.writeFileSync(
			stubPath,
			`// Auto-generated stub: run 'yarn compile' and rebuild.\nexport {};\n`
		);
	}
	finalInput = { __stub: stubPath };
	console.warn(
		'[tauri/build] No entry points found. Using stub input. ' +
		'See tauri/build/README.md for setup instructions.'
	);
}

// ---------------------------------------------------------------------------
// Vite config
// ---------------------------------------------------------------------------

export default defineConfig((): UserConfig => ({
	root: REPO_ROOT,
	base: './',

	plugins: [
		nlsPlugin({ repoRoot: REPO_ROOT }),
		manglePrivatesPlugin({ repoRoot: REPO_ROOT, target }),
		productInjectionPlugin({ repoRoot: REPO_ROOT, target }),
		builtinExtensionsPlugin({ repoRoot: REPO_ROOT, target }),
	],

	resolve: {
		alias: {
			'~@vscode/codicons': path.join(
				REPO_ROOT,
				'node_modules',
				'@vscode',
				'codicons'
			),
		},
		// Allow .ts extensions to be resolved without specifying them
		extensions: ['.ts', '.tsx', '.js', '.jsx', '.json'],
	},

	esbuild: {
		// Match the options used by the esbuild pipeline (build/next/index.ts)
		target: 'es2024',
		tsconfigRaw: {
			compilerOptions: {
				experimentalDecorators: true,
				useDefineForClassFields: false,
			},
		},
	},

	build: {
		outDir,
		emptyOutDir: true,
		minify: process.env['VSCODE_MINIFY'] === '1' ? 'esbuild' : false,
		sourcemap: true,
		target: 'es2024',

		rollupOptions: {
			input: finalInput,
			output: {
				format: 'esm',
				// Preserve directory structure matching src/ layout
				entryFileNames: '[name].js',
				chunkFileNames: 'chunks/[name]-[hash].js',
				assetFileNames: 'media/[name][extname]',
			},
			external(id: string) {
				// Native addons — always external (binary, can't bundle)
				if (id.endsWith('.node')) {
					return true;
				}
				// Node built-ins — always external
				if (
					id.startsWith('node:') ||
					id === 'electron' ||
					[
						'path', 'fs', 'os', 'crypto', 'child_process', 'net',
						'http', 'https', 'url', 'util', 'events', 'stream',
						'buffer', 'assert', 'zlib', 'tls', 'dns', 'dgram',
						'readline', 'repl', 'vm', 'worker_threads', 'perf_hooks',
						'v8', 'module', 'string_decoder', 'querystring', 'punycode',
						'domain', 'timers', 'constants', 'sys', 'tty', 'process',
						'async_hooks', 'inspector', 'http2', 'cluster', 'trace_events',
						'diagnostics_channel', 'wasi', 'readline/promises',
					].includes(id)
				) {
					return true;
				}
				// All npm packages external — matches esbuild `packages: 'external'`
				// Detects bare specifiers that resolve through node_modules
				if (
					!id.startsWith('/') &&
					!id.startsWith('.') &&
					!id.startsWith('\0') &&
					!id.startsWith('virtual:')
				) {
					return true;
				}
				return false;
			},
			onwarn(warning, warn) {
				// Silence known noisy warnings from VSCode source
				if (
					warning.code === 'CIRCULAR_DEPENDENCY' ||
					warning.code === 'THIS_IS_UNDEFINED'
				) {
					return;
				}
				warn(warning);
			},
		},
	},
}));
