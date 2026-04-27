/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import assert from 'assert';
import { suite, test } from 'node:test';
import {
	getEntryPointsForTarget,
	getBootstrapEntryPointsForTarget,
	getCssBundleEntryPointsForTarget,
	isExtensionHostBundle,
	workerEntryPoints,
	extensionHostEntryPoints,
} from '../entrypoints.ts';

suite('entrypoints', () => {

	test('desktop includes worker and desktop entry points', () => {
		const entries = getEntryPointsForTarget('desktop');
		assert.ok(entries.includes('vs/editor/common/services/editorWebWorkerMain'));
		assert.ok(entries.includes('vs/workbench/workbench.desktop.main'));
		assert.ok(entries.includes('vs/code/electron-browser/workbench/workbench'));
	});

	test('server includes only server entry points', () => {
		const entries = getEntryPointsForTarget('server');
		assert.ok(entries.includes('vs/workbench/api/node/extensionHostProcess'));
		assert.ok(!entries.includes('vs/workbench/workbench.desktop.main'));
		assert.ok(!entries.includes('vs/workbench/workbench.web.main.internal'));
	});

	test('server-web includes server + web + workers + keyboard', () => {
		const entries = getEntryPointsForTarget('server-web');
		assert.ok(entries.includes('vs/workbench/workbench.web.main.internal'));
		assert.ok(entries.includes('vs/workbench/api/node/extensionHostProcess'));
		assert.ok(entries.includes('vs/editor/common/services/editorWebWorkerMain'));
		assert.ok(entries.includes('vs/workbench/services/keybinding/browser/keyboardLayouts/layout.contribution.linux'));
	});

	test('web does not include server or desktop entries', () => {
		const entries = getEntryPointsForTarget('web');
		assert.ok(!entries.includes('vs/workbench/workbench.desktop.main'));
		assert.ok(!entries.includes('vs/workbench/api/node/extensionHostProcess'));
		assert.ok(entries.includes('vs/workbench/workbench.web.main.internal'));
	});

	test('unknown target throws', () => {
		assert.throws(
			() => getEntryPointsForTarget('unknown' as never),
			/Unknown target/
		);
	});

	test('bootstrap desktop returns main, cli, bootstrap-fork', () => {
		const bs = getBootstrapEntryPointsForTarget('desktop');
		assert.deepStrictEqual(bs, ['main', 'cli', 'bootstrap-fork']);
	});

	test('bootstrap server/server-web returns server-main, server-cli, bootstrap-fork', () => {
		const server = getBootstrapEntryPointsForTarget('server');
		const serverWeb = getBootstrapEntryPointsForTarget('server-web');
		assert.deepStrictEqual(server, ['server-main', 'server-cli', 'bootstrap-fork']);
		assert.deepStrictEqual(serverWeb, ['server-main', 'server-cli', 'bootstrap-fork']);
	});

	test('bootstrap web returns empty array', () => {
		assert.deepStrictEqual(getBootstrapEntryPointsForTarget('web'), []);
	});

	test('CSS bundle entries for desktop include workbench.desktop.main', () => {
		const css = getCssBundleEntryPointsForTarget('desktop');
		assert.ok(css.has('vs/workbench/workbench.desktop.main'));
	});

	test('isExtensionHostBundle detects extension host path', () => {
		assert.ok(isExtensionHostBundle('/out/vs/workbench/api/node/extensionHostProcess.js'));
		assert.ok(isExtensionHostBundle('/out/vs/workbench/api/worker/extensionHostWorkerMain.js'));
		assert.ok(!isExtensionHostBundle('/out/vs/workbench/workbench.desktop.main.js'));
	});

	test('workerEntryPoints contains required workers', () => {
		assert.ok(workerEntryPoints.includes('vs/editor/common/services/editorWebWorkerMain'));
		assert.ok(workerEntryPoints.includes('vs/workbench/api/worker/extensionHostWorkerMain'));
	});

	test('extensionHostEntryPoints listed correctly', () => {
		assert.ok(extensionHostEntryPoints.includes('vs/workbench/api/node/extensionHostProcess'));
		assert.ok(extensionHostEntryPoints.includes('vs/workbench/api/worker/extensionHostWorkerMain'));
	});
});
