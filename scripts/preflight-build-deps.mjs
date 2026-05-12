/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const buildRoot = path.join(repoRoot, 'build');
const requiredBuildPackages = ['gulp-merge-json', 'jsonc-parser', 'esbuild'];

function missingBuildPackages() {
	return requiredBuildPackages.filter(name => !existsSync(path.join(buildRoot, 'node_modules', name, 'package.json')));
}

const missing = missingBuildPackages();

if (missing.length > 0) {
	console.error(`build dependency preflight failed: missing build/node_modules packages: ${missing.join(', ')}`);
	console.error('Run npm run install-build-deps from the repository root before npm run test-build-scripts.');
	console.error('The install script uses build/.npmrc and clears root Electron npm config so build-only native packages target Node.');
	process.exit(1);
}

console.log(`ok build dependencies: ${requiredBuildPackages.join(', ')}`);
