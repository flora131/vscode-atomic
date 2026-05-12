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
const releaseCiCheckScriptPath = path.join(repoRoot, 'scripts/tauri-release-ci-check.mjs');
const parityManifestPath = path.join(repoRoot, 'scripts/tauri-parity-gate.manifest.json');
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

function runParityGate(manifest: Record<string, unknown>, packageJson: Record<string, unknown>): ReturnType<typeof spawnSync> {
	const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'vscode-atomic-tauri-gate-test-'));
	const manifestPath = path.join(tempRoot, 'tauri-parity-gate.manifest.json');
	const packagePath = path.join(tempRoot, 'package.json');
	writeJson(manifestPath, manifest);
	writeJson(packagePath, packageJson);

	const result = spawnSync(process.execPath, [parityGateScriptPath], {
		cwd: repoRoot,
		encoding: 'utf8',
		maxBuffer: 1024 * 1024,
		env: {
			...process.env,
			CODE_TAURI_PARITY_MANIFEST_PATH: manifestPath,
			CODE_TAURI_PACKAGE_JSON_PATH: packagePath,
		},
	});
	fs.rmSync(tempRoot, { recursive: true, force: true });
	return result;
}

function packageMetadata(status: string): Record<string, unknown> {
	return {
		tauriMigration: {
			status,
			manifest: 'scripts/tauri-parity-gate.manifest.json',
			releaseGate: 'ReleaseCandidate',
			claimPolicy: expectedClaimPolicy,
		},
		scripts: {
			'tauri:release-ci-check': 'node scripts/tauri-release-ci-check.mjs',
			'package:tauri:gate': 'npm run tauri:release-ci-check',
			'test:tauri:package-gate': 'npm run package:tauri:gate',
		},
	};
}

suite('Tauri gate regression checks', () => {

	test('parity gate rejects promoted state missing CI artifact evidence', () => {
		const result = runParityGate(createReleaseCandidateManifestMissingArtifactEvidence(), packageMetadata('ReleaseCandidate'));
		const output = `${result.stdout}\n${result.stderr}`;

		assert.strictEqual(result.status, 1, output);
		assert.match(output, /fail parity gate RuntimeBuildGreen lastCiResult\.ciArtifactUrl or ciArtifactNotApplicableReason field: undefined \/ undefined/);
	});

	test('parity gate rejects package status drift from manifest currentState', () => {
		const manifest = JSON.parse(fs.readFileSync(parityManifestPath, 'utf8')) as Record<string, unknown>;
		const result = runParityGate(manifest, packageMetadata('RuntimeBuildGreen'));
		const output = `${result.stdout}\n${result.stderr}`;

		assert.strictEqual(result.status, 1, output);
		assert.match(output, /fail package\.json tauriMigration\.status matches manifest currentState: RuntimeBuildGreen \/ Scaffold/);
	});

	test('parity gate rejects release validation command with scaffold workbench source', () => {
		const manifest = JSON.parse(fs.readFileSync(parityManifestPath, 'utf8')) as Record<string, unknown>;
		const releaseValidation = manifest.releaseValidation as Record<string, unknown>;
		releaseValidation.command = `${String(releaseValidation.command)} && node src-tauri/www/index.html`;
		const gates = manifest.gates as Array<Record<string, unknown>>;
		const releaseCandidateGate = gates.find(gate => gate.state === 'ReleaseCandidate') as Record<string, unknown>;
		releaseCandidateGate.command = `${String(releaseCandidateGate.command)} && node src-tauri/www/index.html`;

		const result = runParityGate(manifest, packageMetadata('Scaffold'));
		const output = `${result.stdout}\n${result.stderr}`;

		assert.strictEqual(result.status, 1, output);
		assert.match(output, /fail release validation requires real workbench env:/);
		assert.match(output, /fail parity gate ReleaseCandidate requires real workbench env:/);
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

	test('release CI check skips package validation for unlabeled Scaffold PR', () => {
		const result = spawnSync(process.execPath, [releaseCiCheckScriptPath], {
			cwd: repoRoot,
			encoding: 'utf8',
			env: {
				...process.env,
				CODE_TAURI_PARITY_MANIFEST_PATH: parityManifestPath,
			},
		});
		const output = `${result.stdout}\n${result.stderr}`;

		assert.strictEqual(result.status, 0, output);
		assert.match(output, /ok release CI package validation skipped before ReleaseCandidate without release label: Scaffold/);
	});

	test('release CI check blocks release-labeled PR before ReleaseCandidate', () => {
		const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'vscode-atomic-tauri-release-label-test-'));
		try {
			const eventPath = path.join(tempRoot, 'event.json');
			writeJson(eventPath, {
				pull_request: {
					labels: [{ name: 'release-candidate' }],
				},
			});

			const result = spawnSync(process.execPath, [releaseCiCheckScriptPath], {
				cwd: repoRoot,
				encoding: 'utf8',
				env: {
					...process.env,
					CODE_TAURI_PARITY_MANIFEST_PATH: parityManifestPath,
					GITHUB_EVENT_PATH: eventPath,
				},
			});
			const output = `${result.stdout}\n${result.stderr}`;

			assert.strictEqual(result.status, 1, output);
			assert.match(output, /ok release CI label scan completed: release-candidate/);
			assert.match(output, /fail release-labeled PR blocked before ReleaseCandidate: Scaffold/);
		} finally {
			fs.rmSync(tempRoot, { recursive: true, force: true });
		}
	});
});
