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

function readRootPackageScripts(): Record<string, string> {
	const packageJsonPath = path.join(import.meta.dirname, '../../../package.json');
	const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8')) as PackageJson;
	return packageJson.scripts ?? {};
}

suite('root package scripts', () => {

	test('guards native TypeScript checks with Tauri preflight', () => {
		const scripts = readRootPackageScripts();

		assert.strictEqual(scripts.typecheck, 'npm run compile-check-ts-native');
		assert.strictEqual(scripts['typecheck:tauri'], 'npm run compile-check-ts-native');
		assert.strictEqual(scripts['tauri:preflight'], 'node scripts/preflight-tsgo.mjs');
		assert.strictEqual(scripts['compile-check-ts-native'], 'npm run tauri:preflight && tsgo --project ./src/tsconfig.json --noEmit --skipLibCheck');
		assert.ok(scripts['compile-check-ts-native']?.startsWith('npm run tauri:preflight && '));
		assert.ok(!scripts.typecheck?.includes('tauri:preflight'));
	});

	test('guards build script tests with build dependency preflight', () => {
		const scripts = readRootPackageScripts();

		assert.strictEqual(scripts['install-build-deps'], 'node scripts/install-build-deps.mjs');
		assert.strictEqual(scripts['test-build-scripts'], 'node scripts/preflight-build-deps.mjs && cd build && npm run test');
		assert.strictEqual(scripts['test-build-scripts:ci'], 'npm run install-build-deps && npm run test-build-scripts');
	});

	test('build dependency installer ignores root Electron npm config', () => {
		const installPath = path.join(import.meta.dirname, '../../../scripts/install-build-deps.mjs');
		const installSource = fs.readFileSync(installPath, 'utf8');

		assert.ok(installSource.includes('[\'ci\', \'--prefix\', \'build\', \'--userconfig\', \'build/.npmrc\']'));
		assert.ok(installSource.includes('delete env[`npm_config_${name}`]'));
		assert.ok(installSource.includes('delete env[`NPM_CONFIG_${name.toUpperCase()}`]'));
		for (const name of ['disturl', 'target', 'ms_build_id', 'runtime']) {
			assert.ok(installSource.includes(`'${name}'`));
		}
	});

	test('build dependency preflight verifies without installing', () => {
		const preflightPath = path.join(import.meta.dirname, '../../../scripts/preflight-build-deps.mjs');
		const preflightSource = fs.readFileSync(preflightPath, 'utf8');

		const requiredBuildPackages = ['gulp' + '-merge-json', 'jsonc' + '-parser', 'es' + 'build'];
		for (const requiredBuildPackage of requiredBuildPackages) {
			assert.ok(preflightSource.includes(`'${requiredBuildPackage}'`));
		}
		assert.ok(preflightSource.includes('build dependency preflight failed: missing build/node_modules packages:'));
		assert.ok(preflightSource.includes('Run npm run install-build-deps from the repository root before npm run test-build-scripts.'));
		assert.ok(preflightSource.includes('The install script uses build/.npmrc and clears root Electron npm config so build-only native packages target Node.'));
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
			assert.match(result.stderr, /Run npm run install-build-deps from the repository root before npm run test-build-scripts\./);
			assert.match(result.stderr, /The install script uses build\/\.npmrc and clears root Electron npm config so build-only native packages target Node\./);
			assert.ok(!fs.existsSync(path.join(tempRoot, 'build/node_modules')), 'preflight must not install missing dependencies');
		} finally {
			fs.rmSync(tempRoot, { recursive: true, force: true });
		}
	});
});
