/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

/**
 * Vite plugin: product configuration injection.
 *
 * Ports the `fileContentMapperPlugin` from build/next/index.ts (lines 638–698)
 * for the `/*BUILD->INSERT_PRODUCT_CONFIGURATION* /` placeholder.
 *
 * During the Vite transform phase this plugin replaces the placeholder with the
 * serialised product.json fields (merged with version/commit/date) so that the
 * workbench bootstrap code can read configuration without a separate network
 * request.
 *
 * Shape is identical to the esbuild pipeline:
 *   - The placeholder is inside an object literal, so the outer `{` `}` braces
 *     of the JSON are stripped before injection.
 *   - For the server-web target the `webEndpointUrlTemplate` field is removed.
 *
 * TODO (non-desktop targets): Currently the plugin is fully functional for all
 *   targets; the server-web strip is already implemented.  No further stubs needed.
 */

import type { Plugin } from 'vite';
import * as path from 'path';
import * as fs from 'fs';

export interface ProductInjectionPluginOptions {
	/** Repo root — used to load product.json, package.json, and the build date. */
	repoRoot: string;
	/** Build target — server-web strips webEndpointUrlTemplate. */
	target: string;
}

function readISODate(outDir: string): string {
	try {
		return fs.readFileSync(path.join(outDir, 'date'), 'utf8').trim();
	} catch {
		return new Date().toISOString();
	}
}

export function productInjectionPlugin(opts: ProductInjectionPluginOptions): Plugin {
	const { repoRoot, target } = opts;

	// Cache computed replacement string (computed lazily on first hit)
	let replacement: string | undefined;

	function getReplacement(): string {
		if (replacement !== undefined) {
			return replacement;
		}

		const product = JSON.parse(
			fs.readFileSync(path.join(repoRoot, 'product.json'), 'utf8')
		);
		const packageJson = JSON.parse(
			fs.readFileSync(path.join(repoRoot, 'package.json'), 'utf8')
		);

		// Compute version (matching build/next/index.ts logic)
		const quality: string | undefined = product.quality;
		const version =
			quality && quality !== 'stable'
				? `${packageJson.version}-${quality}`
				: packageJson.version;

		// Compute commit
		let commit = 'unknown';
		try {
			const { execSync } = require('child_process');
			commit = execSync('git rev-parse HEAD', { cwd: repoRoot })
				.toString()
				.trim();
		} catch {
			// ignore
		}

		// Compute build date
		const date = readISODate(path.join(repoRoot, 'out-build'));

		// For server-web: strip webEndpointUrlTemplate
		const productForTarget =
			target === 'server-web'
				? { ...product, webEndpointUrlTemplate: undefined }
				: product;

		const productConfiguration = JSON.stringify({
			...productForTarget,
			version,
			commit,
			date,
		});

		// Strip outer braces — the placeholder is inside an object literal
		replacement = productConfiguration.substring(
			1,
			productConfiguration.length - 1
		);
		return replacement;
	}

	return {
		name: 'vscode-product-injection',
		apply: 'build',

		transform(code: string, id: string) {
			if (!id.endsWith('.ts') && !id.endsWith('.js')) {
				return null;
			}
			if (!code.includes('/*BUILD->INSERT_PRODUCT_CONFIGURATION*/')) {
				return null;
			}

			const replaced = code.replace(
				'/*BUILD->INSERT_PRODUCT_CONFIGURATION*/',
				() => getReplacement()
			);
			return { code: replaced };
		},
	};
}
