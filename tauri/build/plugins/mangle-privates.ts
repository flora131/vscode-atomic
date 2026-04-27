/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

/**
 * Vite plugin: mangle private fields (#foo → $a, $b, …).
 *
 * Ports the mangle-privates post-processing step from build/next/index.ts
 * (lines 963–975) into a Vite renderChunk hook.
 *
 * The actual transformation is performed by the existing TypeScript module at
 * build/next/private-to-property.ts.  We import it at runtime so we stay in
 * sync with any upstream changes without duplicating code.
 *
 * Scope:
 *   - Only applied to chunks whose fileName maps to a non-extension-host bundle.
 *   - Extension-host bundles expose public API surface to extensions and must
 *     preserve true native #private encapsulation.
 *
 * TODO (non-desktop targets): For targets other than 'desktop' this plugin is
 *   a no-op.  Private field mangling is a desktop-build optimisation aligned
 *   with the Electron V8 version.  Revisit if enabling for web targets.
 */

import type { Plugin, NormalizedOutputOptions } from 'vite';
import * as path from 'path';
import { pathToFileURL } from 'url';
import { isExtensionHostBundle } from '../entrypoints.js';

export interface ManglePrivatesPluginOptions {
	/** Repo root used to resolve build/next/private-to-property.ts. */
	repoRoot: string;
	/** Only mangle for this target; no-op for non-desktop. */
	target: string;
}

export function manglePrivatesPlugin(opts: ManglePrivatesPluginOptions): Plugin {
	const { repoRoot, target } = opts;

	// Only active for desktop builds
	if (target !== 'desktop') {
		return { name: 'vscode-mangle-privates-stub' };
	}

	// Lazily resolved converter function
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	let convertPrivateFields: ((code: string, filePath: string) => any) | undefined;

	async function getConverter() {
		if (!convertPrivateFields) {
			const modulePath = path.join(repoRoot, 'build', 'next', 'private-to-property.ts');
			const moduleUrl = pathToFileURL(modulePath).href;
			// Dynamic import with .ts extension — works with tsx/ts-node/native-ts-node
			const mod = await import(moduleUrl);
			convertPrivateFields = mod.convertPrivateFields;
		}
		return convertPrivateFields!;
	}

	return {
		name: 'vscode-mangle-privates',
		apply: 'build',

		async renderChunk(
			code: string,
			chunk: { fileName: string },
			_outputOpts: NormalizedOutputOptions
		) {
			// Skip non-JS outputs (CSS, sourcemaps handled separately)
			if (!chunk.fileName.endsWith('.js')) {
				return null;
			}

			// Skip extension-host bundles
			if (isExtensionHostBundle(chunk.fileName)) {
				return null;
			}

			try {
				const converter = await getConverter();
				const result = converter(code, chunk.fileName);
				if (result.editCount === 0) {
					return null;
				}
				return { code: result.code };
			} catch (err) {
				// Non-fatal: log and continue without mangling
				console.warn(
					`[vscode-mangle-privates] Skipping ${chunk.fileName}: ${err}`
				);
				return null;
			}
		},
	};
}
