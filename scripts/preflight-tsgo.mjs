/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { spawnSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const expectedNodeVersion = readFileSync(path.join(repoRoot, '.nvmrc'), 'utf8').trim();
const localTsgo = path.join(repoRoot, 'node_modules', '.bin', process.platform === 'win32' ? 'tsgo.cmd' : 'tsgo');
const failures = [];

function check(name, ok, detail) {
	const status = ok ? 'ok' : 'fail';
	console.log(`${status} ${name}${detail ? `: ${detail}` : ''}`);
	if (!ok) {
		failures.push(`${name}${detail ? `: ${detail}` : ''}`);
	}
	return ok;
}

function run(command, args) {
	return spawnSync(command, args, {
		cwd: repoRoot,
		encoding: 'utf8',
		shell: process.platform === 'win32',
	});
}

const actualNodeVersion = process.version.replace(/^v/, '');
check('node version matches .nvmrc', actualNodeVersion === expectedNodeVersion, `expected ${expectedNodeVersion}, got ${actualNodeVersion}`);

if (check('local tsgo binary exists', existsSync(localTsgo), localTsgo)) {
	const result = run(localTsgo, ['--version']);
	const output = result.stdout.trim() || result.stderr.trim();
	check('local tsgo --version', result.status === 0, output || `exit ${result.status}`);
}

if (failures.length > 0) {
	console.error('\ntsgo preflight failed.');
	console.error(`Use Node ${expectedNodeVersion} from .nvmrc, then run npm ci from the repository root.`);
	console.error('If CI or local installs still miss tsgo, clear the node_modules/npm cache for this repo and rerun npm ci.');
	console.error('Expected @typescript/native-preview to provide node_modules/.bin/tsgo before npm run typecheck.');
	process.exit(1);
}
