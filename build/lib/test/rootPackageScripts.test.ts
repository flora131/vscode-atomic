/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import assert from 'assert';
import { spawnSync } from 'child_process';
import { suite, test } from 'node:test';
import fs from 'fs';
import os from 'os';
import path from 'path';

interface PackageJson {
	scripts?: Record<string, string>;
}

suite('root package scripts', () => {

	test('guards native TypeScript checks with Tauri preflight', () => {
		const packageJsonPath = path.join(import.meta.dirname, '../../../package.json');
		const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8')) as PackageJson;
		const scripts = packageJson.scripts ?? {};

		assert.strictEqual(scripts.typecheck, 'npm run compile-check-ts-native');
		assert.strictEqual(scripts['typecheck:tauri'], 'npm run compile-check-ts-native');
		assert.strictEqual(scripts['tauri:preflight'], 'node scripts/preflight-tsgo.mjs');
		assert.strictEqual(scripts['compile-check-ts-native'], 'npm run tauri:preflight && tsgo --project ./src/tsconfig.json --noEmit --skipLibCheck');
		assert.ok(scripts['compile-check-ts-native']?.startsWith('npm run tauri:preflight && '));
		assert.ok(!scripts.typecheck?.includes('tauri:preflight'));
	});

	test('guards build script tests with build dependency preflight', () => {
		const packageJsonPath = path.join(import.meta.dirname, '../../../package.json');
		const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8')) as PackageJson;
		const scripts = packageJson.scripts ?? {};

		assert.strictEqual(scripts['test-build-scripts'], 'node scripts/preflight-build-deps.mjs && cd build && npm run test');
	});

	test('build dependency preflight verifies without installing', () => {
		const preflightPath = path.join(import.meta.dirname, '../../../scripts/preflight-build-deps.mjs');
		const preflightSource = fs.readFileSync(preflightPath, 'utf8');

		assert.ok(preflightSource.includes("'gulp-merge-json', 'jsonc-parser', 'esbuild'"));
		assert.ok(preflightSource.includes('build dependency preflight failed: missing build/node_modules packages:'));
		assert.ok(preflightSource.includes('Run npm ci --prefix build from the repository root, or equivalent, before npm run test-build-scripts.'));
		assert.ok(!preflightSource.includes('spawnSync'));
		assert.ok(!preflightSource.includes('npm install'));
	});

	test('build dependency preflight fails fast when build packages are missing', () => {
		const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'vscode-atomic-build-preflight-test-'));
		try {
			fs.mkdirSync(path.join(tempRoot, 'scripts'), { recursive: true });
			fs.copyFileSync(path.join(import.meta.dirname, '../../../scripts/preflight-build-deps.mjs'), path.join(tempRoot, 'scripts/preflight-build-deps.mjs'));

			const result = spawnSync(process.execPath, ['scripts/preflight-build-deps.mjs'], {
				cwd: tempRoot,
				encoding: 'utf8',
			});

			assert.strictEqual(result.status, 1, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stderr, /build dependency preflight failed: missing build\/node_modules packages: gulp-merge-json, jsonc-parser, esbuild/);
			assert.match(result.stderr, /Run npm ci --prefix build from the repository root, or equivalent, before npm run test-build-scripts\./);
			assert.ok(!fs.existsSync(path.join(tempRoot, 'build/node_modules')), 'preflight must not install missing dependencies');
		} finally {
			fs.rmSync(tempRoot, { recursive: true, force: true });
		}
	});
});
