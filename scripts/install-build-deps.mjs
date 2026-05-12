/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const npmCommand = process.platform === 'win32' ? 'npm.cmd' : 'npm';
const env = { ...process.env };

for (const name of ['disturl', 'target', 'ms_build_id', 'runtime']) {
	delete env[`npm_config_${name}`];
	delete env[`NPM_CONFIG_${name.toUpperCase()}`];
}

const result = spawnSync(npmCommand, ['ci', '--prefix', 'build', '--userconfig', 'build/.npmrc'], {
	cwd: repoRoot,
	env,
	stdio: 'inherit',
});

if (result.error) {
	console.error(result.error.message);
	process.exit(1);
}

process.exit(result.status ?? 1);
