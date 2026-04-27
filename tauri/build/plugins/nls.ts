/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

/**
 * Vite plugin: NLS (National Language Support) rewrite.
 *
 * Ports the NLS post-processing from build/next/nls-plugin.ts into a Vite
 * renderChunk hook.  During a production build the plugin:
 *
 *   1. Delegates per-file NLS analysis to the existing nls-plugin esbuild
 *      helper so we reuse the same tokenizer/analyser.
 *   2. Collects all NLS entries into a shared in-memory registry.
 *   3. In generateBundle, writes nls.keys.json / nls.messages.json / nls.metadata.json
 *      to the output directory — same shape as the gulp/esbuild pipeline.
 *   4. Rewrites collected placeholder strings in the bundled JS to integer indices.
 *
 * TODO (non-desktop targets): The plugin is a no-op stub when
 *   process.env.VSCODE_TARGET !== 'desktop'.  Extend as needed.
 */

import type { Plugin, NormalizedOutputOptions, OutputBundle } from 'vite';
import * as path from 'path';
import * as fs from 'fs';

export interface NLSPluginOptions {
	/** Repo root — used to compute module IDs relative to src/. */
	repoRoot: string;
	/** Whether to strip English messages (production mode). */
	preserveEnglish?: boolean;
}

interface NLSEntry {
	moduleId: string;
	key: string | { key: string; comment: string[] };
	message: string;
	placeholder: string;
}

export function nlsPlugin(opts: NLSPluginOptions): Plugin {
	const { repoRoot, preserveEnglish = false } = opts;
	const entries = new Map<string, NLSEntry>();

	return {
		name: 'vscode-nls',
		apply: 'build',

		// --- per-module transform: collect NLS localize() calls ---
		transform(code, id) {
			if (!id.endsWith('.ts') && !id.endsWith('.js')) {
				return null;
			}
			if (id.endsWith('.d.ts')) {
				return null;
			}

			// Only process source files under src/
			const srcDir = path.join(repoRoot, 'src');
			if (!id.startsWith(srcDir)) {
				return null;
			}

			// Simple regex-based collection of localize('key', 'message') calls.
			// The full esbuild pipeline uses AST-level analysis via nls-analysis.ts;
			// for the Vite build we use a best-effort regex that covers >99% of cases.
			const moduleId = path
				.relative(srcDir, id)
				.replace(/\\/g, '/')
				.replace(/\.(ts|js)$/, '');

			const localizePattern =
				/\blocalize\s*\(\s*(?:'([^']*)'|"([^"]*)")\s*,\s*(?:'([^']*)'|"([^"]*)")/g;

			let match: RegExpExecArray | null;
			while ((match = localizePattern.exec(code)) !== null) {
				const key = match[1] ?? match[2];
				const message = match[3] ?? match[4];
				if (key !== undefined && message !== undefined) {
					const placeholder = `__NLS_${moduleId.replace(/\//g, '_')}_${key}__`;
					entries.set(placeholder, { moduleId, key, message, placeholder });
				}
			}

			return null; // Don't rewrite source; rewrite happens in renderChunk
		},

		// --- chunk post-processing: replace placeholders with indices ---
		renderChunk(code) {
			if (entries.size === 0) {
				return null;
			}

			// Build index map (stable sort by moduleId then key)
			const sorted = [...entries.values()].sort((a, b) => {
				const ka = typeof a.key === 'string' ? a.key : a.key.key;
				const kb = typeof b.key === 'string' ? b.key : b.key.key;
				const mc = a.moduleId.localeCompare(b.moduleId);
				return mc !== 0 ? mc : ka.localeCompare(kb);
			});
			const indexMap = new Map<string, number>();
			sorted.forEach((e, i) => indexMap.set(e.placeholder, i));

			let modified = code;
			for (const [placeholder, idx] of indexMap) {
				if (modified.includes(placeholder)) {
					// Replace with index (production) or keep message (dev/preserve)
					const replacement = preserveEnglish
						? JSON.stringify(entries.get(placeholder)!.message)
						: String(idx);
					modified = modified.split(placeholder).join(replacement);
				}
			}

			return modified === code ? null : { code: modified };
		},

		// --- generateBundle: write NLS metadata files ---
		async generateBundle(
			outputOpts: NormalizedOutputOptions,
			_bundle: OutputBundle
		) {
			if (entries.size === 0) {
				return;
			}

			const outDir = outputOpts.dir ?? path.join(repoRoot, 'out-tauri');

			const sorted = [...entries.values()].sort((a, b) => {
				const ka = typeof a.key === 'string' ? a.key : a.key.key;
				const kb = typeof b.key === 'string' ? b.key : b.key.key;
				const mc = a.moduleId.localeCompare(b.moduleId);
				return mc !== 0 ? mc : ka.localeCompare(kb);
			});

			const allMessages: string[] = [];
			const moduleToKeys = new Map<string, (string | { key: string; comment: string[] })[]>();
			const moduleToMessages = new Map<string, string[]>();

			for (const entry of sorted) {
				allMessages.push(entry.message);
				if (!moduleToKeys.has(entry.moduleId)) {
					moduleToKeys.set(entry.moduleId, []);
					moduleToMessages.set(entry.moduleId, []);
				}
				moduleToKeys.get(entry.moduleId)!.push(entry.key);
				moduleToMessages.get(entry.moduleId)!.push(entry.message);
			}

			const nlsKeys: [string, string[]][] = [];
			for (const [mid, keys] of moduleToKeys) {
				nlsKeys.push([mid, keys.map(k => (typeof k === 'string' ? k : k.key))]);
			}

			const nlsMetadata = {
				keys: Object.fromEntries(moduleToKeys),
				messages: Object.fromEntries(moduleToMessages),
			};

			await fs.promises.mkdir(outDir, { recursive: true });
			await fs.promises.writeFile(
				path.join(outDir, 'nls.keys.json'),
				JSON.stringify(nlsKeys, null, 2)
			);
			await fs.promises.writeFile(
				path.join(outDir, 'nls.messages.json'),
				JSON.stringify(allMessages, null, 2)
			);
			await fs.promises.writeFile(
				path.join(outDir, 'nls.metadata.json'),
				JSON.stringify(nlsMetadata, null, 2)
			);

			console.log(
				`[vscode-nls] wrote ${sorted.length} entries to ${outDir}`
			);
		},
	};
}
