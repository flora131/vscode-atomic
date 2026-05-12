/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { spawnSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const manifestPath = process.env.CODE_TAURI_PARITY_MANIFEST_PATH || path.join(repoRoot, 'scripts', 'tauri-parity-gate.manifest.json');
const githubEventPath = process.env.GITHUB_EVENT_PATH;
const releaseCandidateState = 'ReleaseCandidate';
const requiredReleaseWorkbenchSource = 'out/vs/code/browser/workbench/workbench.html';
const scaffoldWorkbenchSource = 'src-tauri/www/index.html';
const failures = [];

function check(name, ok, detail) {
	const status = ok ? 'ok' : 'fail';
	console.log(`${status} ${name}${detail ? `: ${detail}` : ''}`);
	if (!ok) {
		failures.push(`${name}${detail ? `: ${detail}` : ''}`);
	}
	return ok;
}

function readJson(filePath, name) {
	if (!check(`${name} exists`, existsSync(filePath), filePath)) {
		return undefined;
	}

	try {
		return JSON.parse(readFileSync(filePath, 'utf8'));
	} catch (error) {
		check(`${name} parses JSON`, false, error.message);
		return undefined;
	}
}

function isNonEmptyString(value) {
	return typeof value === 'string' && value.trim().length > 0;
}

function readGitHubEvent() {
	if (!isNonEmptyString(githubEventPath) || !existsSync(githubEventPath)) {
		return undefined;
	}

	return readJson(githubEventPath, 'GitHub event');
}

function pullRequestLabelNames(event) {
	return [
		...(Array.isArray(event?.pull_request?.labels) ? event.pull_request.labels : []),
		...(Array.isArray(event?.issue?.labels) ? event.issue.labels : []),
		...(event?.label ? [event.label] : []),
	]
		.map(labelName)
		.filter(isNonEmptyString);
}

function labelName(label) {
	if (typeof label === 'string') {
		return label;
	}

	return label?.name;
}

function isReleaseLabel(label) {
	return /(^|[^a-z0-9])release([^a-z0-9]|$)|releasecandidate|release-candidate|release_ready|release-ready/i.test(label);
}

function hasReleaseLabel(event) {
	if (process.env.CODE_TAURI_FORCE_RELEASE_LABEL === '1') {
		return true;
	}

	return pullRequestLabelNames(event).some(isReleaseLabel);
}

function runCommand(command, args, options = {}) {
	const result = spawnSync(command, args, {
		cwd: repoRoot,
		encoding: 'utf8',
		env: { ...process.env, ...(options.env ?? {}) },
		shell: process.platform === 'win32',
		stdio: ['ignore', 'pipe', 'pipe'],
	});

	if (result.stdout?.trim()) {
		console.log(result.stdout.trim());
	}

	if (result.stderr?.trim()) {
		console.error(result.stderr.trim());
	}

	return { status: result.status ?? 1 };
}

function finish() {
	console.log(`summary failures=${failures.length}`);

	if (failures.length) {
		process.exit(1);
	}
}

function validateReleasePackageCheck(manifest, event) {
	const labels = pullRequestLabelNames(event);
	const releaseLabeled = hasReleaseLabel(event);
	check('release CI label scan completed', true, labels.join(', ') || 'none');

	if (manifest?.currentState !== releaseCandidateState) {
		if (releaseLabeled) {
			check('release-labeled PR blocked before ReleaseCandidate', false, String(manifest?.currentState));
		} else {
			check('release CI package validation skipped before ReleaseCandidate without release label', true, String(manifest?.currentState));
		}
		return;
	}

	check('release CI release validation recorded pass at ReleaseCandidate', manifest?.releaseValidation?.lastCiResult?.status === 'pass', String(manifest?.releaseValidation?.lastCiResult?.status));

	const result = runCommand('npm', ['run', 'package:tauri:release-validate'], {
		env: {
			CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1',
			CODE_TAURI_WORKBENCH_PATH: requiredReleaseWorkbenchSource,
		},
	});
	check('release CI package:tauri:release-validate passes at ReleaseCandidate', result.status === 0, String(result.status));
}

const manifest = readJson(manifestPath, 'parity gate manifest');
const event = readGitHubEvent();

const parityResult = runCommand(process.execPath, [path.join(repoRoot, 'scripts', 'tauri-parity-gate.mjs')], {
	env: {
		CODE_TAURI_PARITY_MANIFEST_PATH: manifestPath,
		CODE_TAURI_PR_TITLE: event?.pull_request?.title ?? process.env.CODE_TAURI_PR_TITLE,
		CODE_TAURI_PR_BODY: event?.pull_request?.body ?? process.env.CODE_TAURI_PR_BODY,
	},
});
check('release CI parity gate passes', parityResult.status === 0, String(parityResult.status));
validateReleasePackageCheck(manifest, event);
check('release CI real workbench source is not scaffold', requiredReleaseWorkbenchSource !== scaffoldWorkbenchSource, requiredReleaseWorkbenchSource);
finish();
