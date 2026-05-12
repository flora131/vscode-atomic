/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { suite, test } from 'node:test';
import * as assert from 'assert';
import * as path from 'path';
import { pathToFileURL } from 'node:url';

interface ApiTestSuiteFixture {
	label: string;
	testsPath: string;
}

interface ApiTestPrerequisiteDiagnostics {
	failures: string[];
	warnings: string[];
}

interface ApiTestScriptModule {
	collectApiTestPrerequisiteDiagnostics(options: {
		mode: 'preflight' | 'execute';
		env?: Record<string, string | undefined>;
		testSuites: ApiTestSuiteFixture[];
		exists(candidate: string): boolean;
		isFile?(candidate: string): boolean;
		isExecutable?(candidate: string): boolean;
	}): ApiTestPrerequisiteDiagnostics;
}

const scriptPath = path.join(import.meta.dirname, '../../../scripts/tauri-vscode-api-tests.mjs');
const { collectApiTestPrerequisiteDiagnostics } = await import(pathToFileURL(scriptPath).href) as ApiTestScriptModule;
const testSuites: ApiTestSuiteFixture[] = [
	{ label: 'vscode-api-tests-folder', testsPath: '/repo/extensions/vscode-api-tests/out/singlefolder-tests' },
	{ label: 'vscode-api-tests-workspace', testsPath: '/repo/extensions/vscode-api-tests/out/workspace-tests' },
];

suite('tauri vscode API test prerequisites', () => {

	test('allows clean preflight without CODE_TAURI_APP_PATH or compiled vscode-api-tests output', () => {
		const diagnostics = collectApiTestPrerequisiteDiagnostics({
			mode: 'preflight',
			env: {},
			testSuites,
			exists: () => false,
		});

		assert.deepStrictEqual(diagnostics.failures, []);
		assert.deepStrictEqual(diagnostics.warnings, [
			'vscode-api-tests-folder compiled tests unavailable: run npm run compile-extension:vscode-api-tests before --execute; missing /repo/extensions/vscode-api-tests/out/singlefolder-tests',
			'vscode-api-tests-workspace compiled tests unavailable: run npm run compile-extension:vscode-api-tests before --execute; missing /repo/extensions/vscode-api-tests/out/workspace-tests',
		]);
	});

	test('requires CODE_TAURI_APP_PATH and compiled vscode-api-tests output for execute mode', () => {
		const diagnostics = collectApiTestPrerequisiteDiagnostics({
			mode: 'execute',
			env: {},
			testSuites,
			exists: () => false,
		});

		assert.deepStrictEqual(diagnostics.failures, [
			'vscode-api-tests-folder compiled tests exist for execute mode: /repo/extensions/vscode-api-tests/out/singlefolder-tests',
			'vscode-api-tests-workspace compiled tests exist for execute mode: /repo/extensions/vscode-api-tests/out/workspace-tests',
			'CODE_TAURI_APP_PATH set for execute mode: undefined',
			'CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 set for execute mode: undefined',
			'CODE_TAURI_WORKBENCH_PATH set for execute mode: undefined',
		]);
		assert.deepStrictEqual(diagnostics.warnings, []);
	});

	test('rejects non-real execute mode app and workbench paths', () => {
		const diagnostics = collectApiTestPrerequisiteDiagnostics({
			mode: 'execute',
			env: {
				CODE_TAURI_APP_PATH: 'src-tauri/www/index.html',
				CODE_TAURI_REQUIRE_REAL_WORKBENCH: '0',
				CODE_TAURI_WORKBENCH_PATH: 'src-tauri/www/index.html',
			},
			testSuites,
			exists: candidate => candidate.includes('/out/'),
			isFile: () => true,
			isExecutable: () => true,
		});

		assert.deepStrictEqual(diagnostics.failures, [
			'CODE_TAURI_APP_PATH absolute for execute mode: src-tauri/www/index.html',
			'CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 set for execute mode: 0',
			'CODE_TAURI_WORKBENCH_PATH rejects scaffold fallback for execute mode: src-tauri/www/index.html',
		]);
		assert.deepStrictEqual(diagnostics.warnings, []);
	});

	test('accepts executable app and real workbench env for execute mode', () => {
		const diagnostics = collectApiTestPrerequisiteDiagnostics({
			mode: 'execute',
			env: {
				CODE_TAURI_APP_PATH: '/repo/.build/tauri/code-oss',
				CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1',
				CODE_TAURI_WORKBENCH_PATH: 'out/vs/code/browser/workbench/workbench.html',
			},
			testSuites,
			exists: candidate => candidate.includes('/out/') || candidate === '/repo/.build/tauri/code-oss',
			isFile: () => true,
			isExecutable: candidate => candidate === '/repo/.build/tauri/code-oss',
		});

		assert.deepStrictEqual(diagnostics.failures, []);
		assert.deepStrictEqual(diagnostics.warnings, []);
	});
});
