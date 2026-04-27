/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

/**
 * Vite plugin: built-in extensions list injection.
 *
 * Ports the `fileContentMapperPlugin` from build/next/index.ts (lines 676–686)
 * for the `/*BUILD->INSERT_BUILTIN_EXTENSIONS* /` placeholder.
 *
 * During the Vite transform phase this plugin replaces the placeholder with a
 * JSON-serialised array of scanned built-in extension descriptors so that the
 * workbench startup code can enumerate them without a separate IPC/filesystem
 * call at runtime.
 *
 * The scan is delegated to the existing utility at build/lib/extensions.ts
 * (same as the esbuild pipeline) so descriptor shape stays identical.
 *
 * Target behaviour (mirrors esbuild pipeline):
 *   - 'web' target  → scan .build/web/extensions
 *   - all others    → scan .build/extensions
 *
 * TODO (server / server-web / web targets): The plugin is active for all
 *   targets; adjust extensionsRoot mapping above if web extensions are stored
 *   at a different path in the Tauri workspace layout.
 */

import type { Plugin } from 'vite';
import * as path from 'path';
import { pathToFileURL } from 'url';

export interface BuiltinExtensionsPluginOptions {
	/** Repo root — used to locate .build/extensions and build/lib/extensions.ts. */
	repoRoot: string;
	/** Build target — determines which extensions directory to scan. */
	target: string;
}

export function builtinExtensionsPlugin(opts: BuiltinExtensionsPluginOptions): Plugin {
	const { repoRoot, target } = opts;

	// Cache scanned result
	let replacement: string | undefined;

	async function getReplacement(): Promise<string> {
		if (replacement !== undefined) {
			return replacement;
		}

		// Resolve scan function from existing build utility
		const extensionsUtilPath = path.join(
			repoRoot,
			'build',
			'lib',
			'extensions.ts'
		);
		const extensionsUtil = await import(
			pathToFileURL(extensionsUtilPath).href
		);

		const extensionsRoot =
			target === 'web'
				? path.join(repoRoot, '.build', 'web', 'extensions')
				: path.join(repoRoot, '.build', 'extensions');

		const builtinExtensions: unknown[] =
			extensionsUtil.scanBuiltinExtensions(extensionsRoot);

		const json = JSON.stringify(builtinExtensions);
		// Strip outer brackets — the placeholder is inside an array literal
		replacement = json.substring(1, json.length - 1);
		return replacement;
	}

	return {
		name: 'vscode-builtin-extensions',
		apply: 'build',

		async transform(code: string, id: string) {
			if (!id.endsWith('.ts') && !id.endsWith('.js')) {
				return null;
			}
			if (!code.includes('/*BUILD->INSERT_BUILTIN_EXTENSIONS*/')) {
				return null;
			}

			const rep = await getReplacement();
			const replaced = code.replace(
				'/*BUILD->INSERT_BUILTIN_EXTENSIONS*/',
				() => rep
			);
			return { code: replaced };
		},
	};
}
