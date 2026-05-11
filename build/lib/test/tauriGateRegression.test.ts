/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import assert from 'assert';
import { spawnSync } from 'child_process';
import fs from 'fs';
import os from 'os';
import path from 'path';
import { suite, test } from 'node:test';

const repoRoot = path.join(import.meta.dirname, '../../..');
const parityGateScriptPath = path.join(repoRoot, 'scripts/tauri-parity-gate.mjs');
const docClaimCheckScriptPath = path.join(repoRoot, 'scripts/tauri-doc-claim-check.mjs');
const parityManifestPath = path.join(repoRoot, 'scripts/tauri-parity-gate.manifest.json');
const packageJsonPath = path.join(repoRoot, 'package.json');
const expectedClaimPolicy = 'blocked until ReleaseCandidate; internal scaffold only';

function writeJson(filePath: string, value: unknown): void {
	fs.mkdirSync(path.dirname(filePath), { recursive: true });
	fs.writeFileSync(filePath, `${JSON.stringify(value, null, '\t')}\n`);
}

function cleanCiResult(overrides: Record<string, unknown> = {}): Record<string, unknown> {
	return {
		status: 'pass',
		runName: 'fixture',
		ciRunId: 'fixture-run',
		ciArtifactUrl: 'https://example.invalid/fixture',
		observedAt: '2026-05-11',
		summary: 'fixture pass',
		...overrides,
	};
}

function createReleaseCandidateManifestMissingArtifactEvidence(): Record<string, unknown> {
	const manifest = JSON.parse(fs.readFileSync(parityManifestPath, 'utf8')) as Record<string, unknown>;
	manifest.currentState = 'ReleaseCandidate';
	manifest.releaseValidation = {
		...(manifest.releaseValidation as Record<string, unknown>),
		lastCiResult: cleanCiResult(),
		permitsScaffoldFallback: false,
	};
	manifest.rustLintGate = {
		...(manifest.rustLintGate as Record<string, unknown>),
		lastCiResult: cleanCiResult(),
	};

	const gates = manifest.gates as Array<Record<string, unknown>>;
	for (const gate of gates) {
		gate.lastCiResult = cleanCiResult();
		const builtInExtensionParity = gate.builtInExtensionParity as { areas?: Array<Record<string, unknown>> } | undefined;
		for (const area of builtInExtensionParity?.areas ?? []) {
			area.status = 'pass';
			for (const platformResult of area.platformResults as Array<Record<string, unknown>>) {
				platformResult.status = 'pass';
			}
		}

		const signedPackageEvidence = gate.signedPackageEvidence as { artifacts?: Array<Record<string, unknown>> } | undefined;
		for (const artifact of signedPackageEvidence?.artifacts ?? []) {
			artifact.signatureStatus = 'pass';
			artifact.notarizationStatus = artifact.os === 'macos' ? 'pass' : 'not_required';
			artifact.installStatus = 'pass';
			artifact.launchStatus = 'pass';
		}
	}

	delete (gates[1].lastCiResult as Record<string, unknown>).ciArtifactUrl;
	delete (gates[1].lastCiResult as Record<string, unknown>).ciArtifactNotApplicableReason;
	return manifest;
}

suite('Tauri gate regression checks', () => {

	test('parity gate rejects promoted state missing CI artifact evidence', () => {
		const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'vscode-atomic-tauri-gate-test-'));
		try {
			const manifestPath = path.join(tempRoot, 'tauri-parity-gate.manifest.json');
			const packagePath = path.join(tempRoot, 'package.json');
			writeJson(manifestPath, createReleaseCandidateManifestMissingArtifactEvidence());
			writeJson(packagePath, {
				tauriMigration: {
					status: 'ReleaseCandidate',
					manifest: 'scripts/tauri-parity-gate.manifest.json',
					releaseGate: 'ReleaseCandidate',
					claimPolicy: expectedClaimPolicy,
				},
			});

			const result = spawnSync(process.execPath, [parityGateScriptPath], {
				cwd: repoRoot,
				encoding: 'utf8',
				env: {
					...process.env,
					CODE_TAURI_PARITY_MANIFEST_PATH: manifestPath,
					CODE_TAURI_PACKAGE_JSON_PATH: packagePath,
				},
			});
			const output = `${result.stdout}\n${result.stderr}`;

			assert.strictEqual(result.status, 1, output);
			assert.match(output, /fail parity gate RuntimeBuildGreen lastCiResult\.ciArtifactUrl or ciArtifactNotApplicableReason field: undefined \/ undefined/);
			assert.match(output, /fail parity gate RuntimeBuildGreen current\/prior state lastCiResult\.ciArtifactUrl or ciArtifactNotApplicableReason: undefined \/ undefined/);
		} finally {
			fs.rmSync(tempRoot, { recursive: true, force: true });
		}
	});

	test('doc claim check enforces exact tauriMigration.claimPolicy', () => {
		const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'vscode-atomic-tauri-doc-claim-test-'));
		try {
			const manifestPath = path.join(tempRoot, 'tauri-parity-gate.manifest.json');
			const packagePath = path.join(tempRoot, 'package.json');
			const docPath = path.join(tempRoot, 'PACKAGING.md');
			fs.writeFileSync(docPath, 'Internal scaffold only. Not migration complete before ReleaseCandidate.\n');
			writeJson(manifestPath, { currentState: 'Scaffold' });
			writeJson(packagePath, {
				tauriMigration: {
					status: 'Scaffold',
					releaseGate: 'ReleaseCandidate',
					claimPolicy: 'blocked until ReleaseCandidate',
				},
			});

			const result = spawnSync(process.execPath, [docClaimCheckScriptPath], {
				cwd: repoRoot,
				encoding: 'utf8',
				env: {
					...process.env,
					CODE_TAURI_PARITY_MANIFEST_PATH: manifestPath,
					CODE_TAURI_PACKAGE_JSON_PATH: packagePath,
					CODE_TAURI_DOC_CLAIM_SOURCE_FILES: docPath,
				},
			});
			const output = `${result.stdout}\n${result.stderr}`;

			assert.strictEqual(result.status, 1, output);
			assert.match(output, /fail package\.json tauriMigration\.claimPolicy blocks scaffold release claims: blocked until ReleaseCandidate/);
		} finally {
			fs.rmSync(tempRoot, { recursive: true, force: true });
		}
	});
});
