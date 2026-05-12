/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const manifestPath = process.env.CODE_TAURI_PARITY_MANIFEST_PATH || path.join(repoRoot, 'scripts', 'tauri-parity-gate.manifest.json');
const packageJsonPath = process.env.CODE_TAURI_PACKAGE_JSON_PATH || path.join(repoRoot, 'package.json');
const terminalState = 'ReleaseCandidate';
const scaffoldState = 'Scaffold';
const exactBlockedClaimPolicy = 'blocked until ReleaseCandidate; internal scaffold only';
const requiredClaimSourceFiles = [
	'src-tauri/PACKAGING.md',
	packageJsonPath,
	'src-tauri/tauri.conf.json',
];
const optionalReleaseClaimSourceFiles = [
	'CHANGELOG.md',
	'RELEASE_NOTES.md',
	'release-notes.md',
	'docs/CHANGELOG.md',
	'docs/RELEASE_NOTES.md',
	'docs/release-notes.md',
];
const allowedCompletionPolicyPhrases = [
	'only `releasecandidate`',
	'only releasecandidate',
	'before it is releasecandidate',
	'must not',
	'not migration complete',
	'not migration completion',
	'migration in progress',
	'before `releasecandidate`',
	'before releasecandidate',
	'requires `releasecandidate`',
	'requires releasecandidate',
	'blocked until',
	'never satisfies',
	'forbidden',
	'forbids',
	'does not mean migration complete',
];
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

function sourceFileNames() {
	const override = process.env.CODE_TAURI_DOC_CLAIM_SOURCE_FILES;
	if (!override) {
		return [
			...requiredClaimSourceFiles,
			...optionalReleaseClaimSourceFiles.filter(fileName => existsSync(resolveSourceFile(fileName))),
		];
	}

	return override
		.split(path.delimiter)
		.map(value => value.trim())
		.filter(Boolean);
}

function resolveSourceFile(fileName) {
	return path.isAbsolute(fileName) ? fileName : path.join(repoRoot, fileName);
}

function displaySourceFile(filePath) {
	const relative = path.relative(repoRoot, filePath);
	return relative && !relative.startsWith('..') && !path.isAbsolute(relative) ? relative : filePath;
}

function completionClaimPattern() {
	return /\b(?:migration[-\s]+(?:is\s+|fully\s+)?(?:complete|completed|done|finished)|(?:complete|completed|finished)\s+(?:the\s+)?(?:tauri\s+)?migration|full\s+(?:tauri\s+)?migration|(?:tauri\s+)?migration[-\s]+(?:is\s+)?(?:release[-\s]?ready|ready\s+for\s+release)|(?:tauri\s+)?release[-\s]?ready)\b/i;
}

function lineIsAllowedPolicy(line) {
	const normalized = line.toLowerCase();
	return allowedCompletionPolicyPhrases.some(phrase => normalized.includes(phrase));
}

function scanDocClaims(currentState) {
	if (currentState === terminalState) {
		check('docs migration completion claim scan skipped at ReleaseCandidate', true);
		return;
	}

	const claimPattern = completionClaimPattern();
	const matches = [];
	for (const sourceFile of sourceFileNames()) {
		const filePath = resolveSourceFile(sourceFile);
		if (!check('docs migration completion claim source exists', existsSync(filePath), displaySourceFile(filePath))) {
			continue;
		}

		readFileSync(filePath, 'utf8').split(/\r?\n/).forEach((line, index) => {
			if (claimPattern.test(line) && !lineIsAllowedPolicy(line)) {
				matches.push(`${displaySourceFile(filePath)}:${index + 1}: ${line.trim()}`);
			}
		});
	}

	check('pre-ReleaseCandidate Tauri docs/package docs do not claim migration complete', matches.length === 0, matches.join(' | '));
}

function validatePackageMetadata(manifest) {
	const packageJson = readJson(packageJsonPath, 'package.json');
	if (!packageJson) {
		return;
	}

	const tauriMigration = packageJson.tauriMigration;
	check('package.json tauriMigration object', tauriMigration !== null && typeof tauriMigration === 'object' && !Array.isArray(tauriMigration));
	if (tauriMigration === null || typeof tauriMigration !== 'object' || Array.isArray(tauriMigration)) {
		return;
	}

	check('package.json tauriMigration.status matches parity manifest', tauriMigration.status === manifest?.currentState, `${String(tauriMigration.status)} / ${String(manifest?.currentState)}`);
	if (manifest?.currentState === scaffoldState) {
		check('package.json scaffold tauriMigration.status remains Scaffold', tauriMigration.status === scaffoldState, String(tauriMigration.status));
	}
	check('package.json tauriMigration.releaseGate remains ReleaseCandidate', tauriMigration.releaseGate === terminalState, String(tauriMigration.releaseGate));
	check('package.json tauriMigration.claimPolicy blocks scaffold release claims', tauriMigration.claimPolicy === exactBlockedClaimPolicy, String(tauriMigration.claimPolicy));
}

const manifest = readJson(manifestPath, 'parity gate manifest');
validatePackageMetadata(manifest);
scanDocClaims(manifest?.currentState);

console.log(`summary failures=${failures.length}`);

if (failures.length) {
	process.exit(1);
}
