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
const packageJsonPath = process.env.CODE_TAURI_PACKAGE_JSON_PATH || path.join(repoRoot, 'package.json');
const githubEventPath = process.env.GITHUB_EVENT_PATH;
const prWorkflowPath = path.join(repoRoot, '.github', 'workflows', 'pr.yml');
const releaseCiCheckPath = path.join(repoRoot, 'scripts', 'tauri-release-ci-check.mjs');
const packagingDocPath = path.join(repoRoot, 'src-tauri', 'PACKAGING.md');
const tauriRuntimePath = path.join(repoRoot, 'src-tauri', 'src', 'runtime.rs');
const tauriCommandsPath = path.join(repoRoot, 'src-tauri', 'src', 'commands.rs');
const tauriSidecarPath = path.join(repoRoot, 'src-tauri', 'src', 'extension_sidecar.rs');
const tauriSubscriptionManagerPath = path.join(repoRoot, 'src-tauri', 'src', 'subscription_manager.rs');
const tauriConfigPath = path.join(repoRoot, 'src-tauri', 'tauri.conf.json');
const expectedStates = [
	'Scaffold',
	'RuntimeBuildGreen',
	'RealWorkbenchBootGreen',
	'CoreServicesParityGreen',
	'ExtensionApiParityGreen',
	'BuiltInExtensionParityGreen',
	'SignedPackageGreen',
	'ReleaseCandidate',
];
const terminalState = 'ReleaseCandidate';
const exactBlockedClaimPolicy = 'blocked until ReleaseCandidate; internal scaffold only';
const rustLintCommand = 'cargo clippy --manifest-path src-tauri/Cargo.toml --no-default-features -- -D warnings';
const requiredReleaseWorkbenchSource = 'out/vs/code/browser/workbench/workbench.html';
const scaffoldWorkbenchSource = 'src-tauri/www/index.html';
const requiredReleaseWorkbenchPath = path.join(repoRoot, requiredReleaseWorkbenchSource);
const releaseValidationCommandNeedle = 'npm run package:tauri:release-validate';
const signedPackageCommandNeedle = 'npm run package:tauri --';
const parityGateScriptNeedle = 'scripts/tauri-parity-gate.mjs';
const releaseCiCheckScriptNeedle = 'scripts/tauri-release-ci-check.mjs';
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
const requiredSignedPackageArtifactOses = ['macos', 'windows', 'linux'];
const requiredBuiltInExtensionAreas = ['git', 'typescriptTsserver', 'debug', 'terminal', 'notebooks', 'auth', 'tunnels', 'copilotApi'];
const requiredBuiltInExtensionPlatforms = ['macos', 'windows', 'linux'];
const allowedNotarizationStatuses = new Set(['pass', 'not_required']);
const artifactOsCheckRequirements = new Map([
	['macos', {
		notarizationStatuses: new Set(['pass']),
		commandNeedles: ['codesign', 'spctl', 'hdiutil', 'cp -R', 'open -W'],
	}],
	['windows', {
		notarizationStatuses: allowedNotarizationStatuses,
		commandNeedles: ['Get-AuthenticodeSignature', 'Start-Process', '--version'],
	}],
	['linux', {
		notarizationStatuses: allowedNotarizationStatuses,
		commandNeedles: ['gpg --verify', 'chmod +x', '--appimage-extract-and-run', '--version'],
	}],
]);
const failures = [];

function check(name, ok, detail) {
	const status = ok ? 'ok' : 'fail';
	console.log(`${status} ${name}${detail ? `: ${detail}` : ''}`);
	if (!ok) {
		failures.push(`${name}${detail ? `: ${detail}` : ''}`);
	}
	return ok;
}

function readJson(filePath, name = 'parity gate manifest') {
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

function readText(filePath, name) {
	if (!check(`${name} exists`, existsSync(filePath), filePath)) {
		return undefined;
	}

	return readFileSync(filePath, 'utf8');
}

function isNonEmptyString(value) {
	return typeof value === 'string' && value.trim().length > 0;
}

function isPlainObject(value) {
	return value !== null && typeof value === 'object' && !Array.isArray(value);
}

function arraysEqual(left, right) {
	return Array.isArray(left) && left.length === right.length && left.every((value, index) => value === right[index]);
}

function validateLastCiResult(name, lastCiResult) {
	check(`${name} lastCiResult object`, isPlainObject(lastCiResult));
	if (!isPlainObject(lastCiResult)) {
		return;
	}

	check(`${name} lastCiResult.status`, ['pass', 'fail'].includes(lastCiResult.status), String(lastCiResult.status));
	check(`${name} lastCiResult.runName`, isNonEmptyString(lastCiResult.runName), String(lastCiResult.runName));
	check(`${name} lastCiResult.ciRunId field`, Object.hasOwn(lastCiResult, 'ciRunId'), String(lastCiResult.ciRunId));
	check(`${name} lastCiResult.ciArtifactUrl or ciArtifactNotApplicableReason field`, Object.hasOwn(lastCiResult, 'ciArtifactUrl') || Object.hasOwn(lastCiResult, 'ciArtifactNotApplicableReason'), `${String(lastCiResult.ciArtifactUrl)} / ${String(lastCiResult.ciArtifactNotApplicableReason)}`);
	check(`${name} lastCiResult.observedAt`, /^\d{4}-\d{2}-\d{2}$/.test(lastCiResult.observedAt), String(lastCiResult.observedAt));
	check(`${name} lastCiResult.summary`, isNonEmptyString(lastCiResult.summary), String(lastCiResult.summary));
}

function validateCiArtifactEvidence(name, lastCiResult) {
	const hasArtifactUrl = isNonEmptyString(lastCiResult?.ciArtifactUrl);
	const hasNotApplicableReason = isNonEmptyString(lastCiResult?.ciArtifactNotApplicableReason);
	check(`${name} lastCiResult.ciArtifactUrl or ciArtifactNotApplicableReason`, hasArtifactUrl || hasNotApplicableReason, `${String(lastCiResult?.ciArtifactUrl)} / ${String(lastCiResult?.ciArtifactNotApplicableReason)}`);
	if (hasArtifactUrl && hasNotApplicableReason) {
		check(`${name} lastCiResult uses one artifact evidence mode`, false, `${String(lastCiResult?.ciArtifactUrl)} / ${String(lastCiResult?.ciArtifactNotApplicableReason)}`);
	}
	if (!hasArtifactUrl) {
		return;
	}

	let url;
	try {
		url = new URL(lastCiResult.ciArtifactUrl);
	} catch (error) {
		check(`${name} lastCiResult.ciArtifactUrl parses URL`, false, error.message);
		return;
	}

	check(`${name} lastCiResult.ciArtifactUrl uses http`, url.protocol === 'https:' || url.protocol === 'http:', url.protocol);
}

function validateCompleteCiEvidence(name, lastCiResult) {
	check(`${name} lastCiResult object`, isPlainObject(lastCiResult));
	if (!isPlainObject(lastCiResult)) {
		return;
	}

	check(`${name} lastCiResult.runName`, isNonEmptyString(lastCiResult.runName), String(lastCiResult.runName));
	check(`${name} lastCiResult.ciRunId`, isNonEmptyString(lastCiResult.ciRunId), String(lastCiResult.ciRunId));
	check(`${name} lastCiResult.observedAt`, /^\d{4}-\d{2}-\d{2}$/.test(lastCiResult.observedAt), String(lastCiResult.observedAt));
	check(`${name} lastCiResult.summary`, isNonEmptyString(lastCiResult.summary), String(lastCiResult.summary));
	validateCiArtifactEvidence(name, lastCiResult);
}

function validateReachedGateCiEvidence(name, record) {
	check(`${name} owner`, isNonEmptyString(record?.owner), String(record?.owner));
	check(`${name} command`, isNonEmptyString(record?.command), String(record?.command));
	check(`${name} lastCiResult.status pass`, record?.lastCiResult?.status === 'pass', String(record?.lastCiResult?.status));
	validateCompleteCiEvidence(name, record?.lastCiResult);
}

function validateGateRecord(name, record) {
	check(`${name} owner`, isNonEmptyString(record?.owner), String(record?.owner));
	check(`${name} command`, isNonEmptyString(record?.command), String(record?.command));
	validateLastCiResult(name, record?.lastCiResult);
}

function getGate(gates, state) {
	return gates.find(gate => gate?.state === state);
}

function commandRequiresRealWorkbench(command) {
	return isNonEmptyString(command)
		&& command.includes('CODE_TAURI_REQUIRE_REAL_WORKBENCH=1')
		&& !command.includes(scaffoldWorkbenchSource)
		&& command.includes(`CODE_TAURI_WORKBENCH_PATH=${requiredReleaseWorkbenchSource}`);
}

function validateApprovedSchemaSplits(manifest) {
	if (manifest?.approvedSchemaSplits === undefined) {
		return;
	}

	check('approved schema splits array', Array.isArray(manifest.approvedSchemaSplits), typeof manifest.approvedSchemaSplits);
	if (!Array.isArray(manifest.approvedSchemaSplits)) {
		return;
	}

	for (const split of manifest.approvedSchemaSplits) {
		check(`approved schema split ${String(split?.state)} state valid`, expectedStates.includes(split?.state), String(split?.state));
		check(`approved schema split ${String(split?.state)} approved`, split?.approved === true, String(split?.approved));
		check(`approved schema split ${String(split?.state)} owner`, isNonEmptyString(split?.owner), String(split?.owner));
		check(`approved schema split ${String(split?.state)} reason`, isNonEmptyString(split?.reason), String(split?.reason));
		check(`approved schema split ${String(split?.state)} approvedAt`, /^\d{4}-\d{2}-\d{2}$/.test(split?.approvedAt), String(split?.approvedAt));
	}
}

function validateSignedPackageArtifact(artifact, index) {
	const prefix = `signed package evidence artifacts[${index}]`;
	const osRequirements = artifactOsCheckRequirements.get(artifact?.os);
	check(`${prefix}.os`, isNonEmptyString(artifact?.os), String(artifact?.os));
	check(`${prefix}.os supported`, osRequirements !== undefined, String(artifact?.os));
	check(`${prefix}.artifactPath`, isNonEmptyString(artifact?.artifactPath), String(artifact?.artifactPath));
	check(`${prefix}.signatureStatus pass`, artifact?.signatureStatus === 'pass', String(artifact?.signatureStatus));
	check(`${prefix}.notarizationStatus ${notarizationExpectationLabel(artifact?.os)}`, osRequirements?.notarizationStatuses.has(artifact?.notarizationStatus) === true, String(artifact?.notarizationStatus));
	check(`${prefix}.installStatus pass`, artifact?.installStatus === 'pass', String(artifact?.installStatus));
	check(`${prefix}.launchStatus pass`, artifact?.launchStatus === 'pass', String(artifact?.launchStatus));
	check(`${prefix}.osCheckCommand`, isNonEmptyString(artifact?.osCheckCommand), String(artifact?.osCheckCommand));
	for (const needle of osRequirements?.commandNeedles ?? []) {
		check(`${prefix}.osCheckCommand includes ${needle}`, artifact?.osCheckCommand?.includes(needle) === true, String(artifact?.osCheckCommand));
	}
}

function notarizationExpectationLabel(os) {
	return os === 'macos' ? 'pass' : 'pass or not_required';
}

function validateSignedPackageEvidence(evidence) {
	check('signed package evidence object', isPlainObject(evidence));
	if (!isPlainObject(evidence)) {
		return;
	}

	check('signed package evidence artifacts', Array.isArray(evidence.artifacts) && evidence.artifacts.length > 0, JSON.stringify(evidence.artifacts));
	if (!Array.isArray(evidence.artifacts)) {
		return;
	}

	evidence.artifacts.forEach(validateSignedPackageArtifact);
	check('signed package evidence artifacts exactly macos/windows/linux', evidence.artifacts.length === requiredSignedPackageArtifactOses.length, String(evidence.artifacts.length));
	const osCounts = countArtifactOses(evidence.artifacts);
	for (const os of requiredSignedPackageArtifactOses) {
		check(`signed package evidence artifacts ${os}`, osCounts.get(os) === 1, String(osCounts.get(os) ?? 0));
	}
}

function countArtifactOses(artifacts) {
	const osCounts = new Map();
	for (const artifact of artifacts) {
		osCounts.set(artifact?.os, (osCounts.get(artifact?.os) ?? 0) + 1);
	}
	return osCounts;
}

function countByField(records, field) {
	const counts = new Map();
	for (const record of records) {
		counts.set(record?.[field], (counts.get(record?.[field]) ?? 0) + 1);
	}
	return counts;
}

function validateBuiltInExtensionPlatformResult(areaName, result, requirePass) {
	check(`built-in extension parity ${areaName} platform`, requiredBuiltInExtensionPlatforms.includes(result?.platform), String(result?.platform));
	check(`built-in extension parity ${areaName} platform status`, ['pass', 'fail'].includes(result?.status), String(result?.status));
	check(`built-in extension parity ${areaName} platform command`, isNonEmptyString(result?.command), String(result?.command));
	check(`built-in extension parity ${areaName} platform summary`, isNonEmptyString(result?.summary), String(result?.summary));
	if (requirePass) {
		check(`built-in extension parity ${areaName} ${result?.platform} status pass`, result?.status === 'pass', String(result?.status));
	}
}

function validateBuiltInExtensionArea(area, requirePass) {
	const areaName = String(area?.area);
	check(`built-in extension parity area ${areaName} supported`, requiredBuiltInExtensionAreas.includes(area?.area), areaName);
	check(`built-in extension parity area ${areaName} owner`, isNonEmptyString(area?.owner), String(area?.owner));
	check(`built-in extension parity area ${areaName} command`, isNonEmptyString(area?.command), String(area?.command));
	check(`built-in extension parity area ${areaName} status`, ['pass', 'fail'].includes(area?.status), String(area?.status));
	check(`built-in extension parity area ${areaName} summary`, isNonEmptyString(area?.summary), String(area?.summary));
	if (requirePass) {
		check(`built-in extension parity area ${areaName} status pass`, area?.status === 'pass', String(area?.status));
	}

	const platformResults = Array.isArray(area?.platformResults) ? area.platformResults : [];
	check(`built-in extension parity area ${areaName} platformResults array`, Array.isArray(area?.platformResults), typeof area?.platformResults);
	check(`built-in extension parity area ${areaName} platformResults exactly macos/windows/linux`, platformResults.length === requiredBuiltInExtensionPlatforms.length, String(platformResults.length));
	const platformCounts = countByField(platformResults, 'platform');
	for (const platform of requiredBuiltInExtensionPlatforms) {
		check(`built-in extension parity area ${areaName} platform ${platform}`, platformCounts.get(platform) === 1, String(platformCounts.get(platform) ?? 0));
	}
	platformResults.forEach(result => validateBuiltInExtensionPlatformResult(areaName, result, requirePass));
}

function validateBuiltInExtensionParityEvidence(evidence, requirePass) {
	check('built-in extension parity evidence object', isPlainObject(evidence));
	if (!isPlainObject(evidence)) {
		return;
	}

	const areas = Array.isArray(evidence.areas) ? evidence.areas : [];
	check('built-in extension parity evidence areas array', Array.isArray(evidence.areas), typeof evidence.areas);
	check('built-in extension parity evidence areas exactly required matrix', areas.length === requiredBuiltInExtensionAreas.length, String(areas.length));
	const areaCounts = countByField(areas, 'area');
	for (const area of requiredBuiltInExtensionAreas) {
		check(`built-in extension parity evidence area ${area}`, areaCounts.get(area) === 1, String(areaCounts.get(area) ?? 0));
	}
	areas.forEach(area => validateBuiltInExtensionArea(area, requirePass));
}

function validateStateProgression(manifest, gates) {
	const currentIndex = expectedStates.indexOf(manifest.currentState);
	if (currentIndex === -1) {
		return;
	}

	for (const gate of gates) {
		const gateIndex = expectedStates.indexOf(gate?.state);
		if (gateIndex === -1) {
			continue;
		}

		if (gateIndex <= currentIndex) {
			validateReachedGateCiEvidence(`parity gate ${gate.state} current/prior state`, gate);
		} else {
			check(`parity gate ${gate.state} future state remains failing`, gate.lastCiResult?.status === 'fail', String(gate.lastCiResult?.status));
		}
	}

	const releaseValidationStatus = manifest.releaseValidation?.lastCiResult?.status;
	if (manifest.currentState === terminalState) {
		check('ReleaseCandidate release validation passes', releaseValidationStatus === 'pass', String(releaseValidationStatus));
		check('ReleaseCandidate release validation forbids scaffold fallback', manifest.releaseValidation?.permitsScaffoldFallback === false, String(manifest.releaseValidation?.permitsScaffoldFallback));
		check('ReleaseCandidate real workbench source exists', existsSync(requiredReleaseWorkbenchPath), requiredReleaseWorkbenchPath);
	} else {
		check('pre-ReleaseCandidate release validation remains failing', releaseValidationStatus === 'fail', String(releaseValidationStatus));
	}

	if (releaseValidationStatus === 'pass') {
		check('release validation pass only at ReleaseCandidate', manifest.currentState === terminalState, String(manifest.currentState));
		check('release validation pass forbids scaffold fallback', manifest.releaseValidation?.permitsScaffoldFallback === false, String(manifest.releaseValidation?.permitsScaffoldFallback));
		validateCompleteCiEvidence('release validation pass', manifest.releaseValidation?.lastCiResult);
	}
}

function validateReleaseValidationConsistency(manifest, gates) {
	const releaseCandidateGate = getGate(gates, terminalState);
	const releaseValidationCommand = manifest.releaseValidation?.command;
	const releaseCandidateCommand = releaseCandidateGate?.command;
	const releaseValidationStatus = manifest.releaseValidation?.lastCiResult?.status;
	const releaseCandidateStatus = releaseCandidateGate?.lastCiResult?.status;

	check('release validation status matches ReleaseCandidate gate', releaseValidationStatus === releaseCandidateStatus, `${String(releaseValidationStatus)} / ${String(releaseCandidateStatus)}`);
	check('release validation command runs release validation package invariant', releaseValidationCommand?.includes(releaseValidationCommandNeedle) === true, String(releaseValidationCommand));
	check('ReleaseCandidate command includes release validation command', isNonEmptyString(releaseValidationCommand) && releaseCandidateCommand?.includes(releaseValidationCommand) === true, String(releaseCandidateCommand));
	check('ReleaseCandidate command runs signed package command', releaseCandidateCommand?.includes(signedPackageCommandNeedle) === true, String(releaseCandidateCommand));
}

function validatePackageParityScripts(packageJson) {
	const scripts = packageJson.scripts;
	check('package.json scripts object', isPlainObject(scripts));
	if (!isPlainObject(scripts)) {
		return;
	}

	check('package.json tauri:release-ci-check runs release CI guard', scripts['tauri:release-ci-check'] === `node ${releaseCiCheckScriptNeedle}`, String(scripts['tauri:release-ci-check']));
	check('package.json package:tauri:gate requires release CI guard', scripts['package:tauri:gate'] === 'npm run tauri:release-ci-check', String(scripts['package:tauri:gate']));
	check('package.json test:tauri:package-gate requires package gate', scripts['test:tauri:package-gate'] === 'npm run package:tauri:gate', String(scripts['test:tauri:package-gate']));
}

function validateParityGateCiWiring() {
	const prWorkflow = readText(prWorkflowPath, 'PR workflow');
	if (prWorkflow) {
		check('PR workflow requires Tauri release CI guard', prWorkflow.includes('npm run tauri:release-ci-check'), prWorkflowPath);
	}

	const releaseCiCheck = readText(releaseCiCheckPath, 'release CI guard');
	if (releaseCiCheck) {
		check('release CI guard invokes parity gate script', sourceInvokesParityGateScript(releaseCiCheck), releaseCiCheckPath);
	}
}

function sourceInvokesParityGateScript(source) {
	return source.includes(parityGateScriptNeedle)
		|| /path\.join\(repoRoot,\s*['"]scripts['"],\s*['"]tauri-parity-gate\.mjs['"]\)/.test(source);
}

function validatePackageMigrationStatus(manifest) {
	const packageJson = readJson(packageJsonPath, 'package.json');
	if (!packageJson) {
		return;
	}

	const tauriMigration = packageJson.tauriMigration;
	check('package.json tauriMigration object', isPlainObject(tauriMigration));
	if (!isPlainObject(tauriMigration)) {
		return;
	}

	check('package.json tauriMigration.status valid', expectedStates.includes(tauriMigration.status), String(tauriMigration.status));
	check('package.json tauriMigration.status matches manifest currentState', tauriMigration.status === manifest.currentState, `${String(tauriMigration.status)} / ${String(manifest.currentState)}`);
	check('package.json tauriMigration.manifest points to parity manifest', tauriMigration.manifest === 'scripts/tauri-parity-gate.manifest.json', String(tauriMigration.manifest));
	check('package.json tauriMigration.releaseGate is ReleaseCandidate', tauriMigration.releaseGate === terminalState, String(tauriMigration.releaseGate));
	check('package.json tauriMigration.claimPolicy blocks until ReleaseCandidate', tauriMigration.claimPolicy === exactBlockedClaimPolicy, String(tauriMigration.claimPolicy));
	check('package.json tauriMigration.status reaches completion only at ReleaseCandidate', (tauriMigration.status === terminalState) === (manifest.currentState === terminalState), `${String(tauriMigration.status)} / ${String(manifest.currentState)}`);
	if (manifest.currentState !== terminalState) {
		check('package.json pre-ReleaseCandidate status stays non-release', tauriMigration.status !== terminalState, String(tauriMigration.status));
	} else {
		validateReachedGateCiEvidence('package.json ReleaseCandidate status release validation', manifest.releaseValidation);
	}
	validatePackageParityScripts(packageJson);
}

function validateManifest(manifest) {
	if (!manifest) {
		return;
	}

	check('parity gate manifest schemaVersion', manifest.schemaVersion === 1, String(manifest.schemaVersion));
	check('parity gate state machine exact order', arraysEqual(manifest.migrationStateMachine, expectedStates), JSON.stringify(manifest.migrationStateMachine));
	for (const [index, state] of (Array.isArray(manifest.migrationStateMachine) ? manifest.migrationStateMachine : []).entries()) {
		check(`parity gate state machine[${index}] state valid`, expectedStates.includes(state), String(state));
	}
	check('parity gate currentState valid', expectedStates.includes(manifest.currentState), String(manifest.currentState));
	validateApprovedSchemaSplits(manifest);

	const gates = Array.isArray(manifest.gates) ? manifest.gates : [];
	const currentIndex = expectedStates.indexOf(manifest.currentState);
	check('parity gate gates array', Array.isArray(manifest.gates), typeof manifest.gates);
	check('parity gate one gate per migration state', gates.length === expectedStates.length, String(gates.length));
	check('parity gate gates exact order', arraysEqual(gates.map(gate => gate?.state), expectedStates), JSON.stringify(gates.map(gate => gate?.state)));
	for (const [index, gate] of gates.entries()) {
		check(`parity gate gates[${index}] state valid`, expectedStates.includes(gate?.state), String(gate?.state));
	}

	for (const state of expectedStates) {
		const matching = gates.filter(gate => gate?.state === state);
		check(`parity gate ${state} exists once`, matching.length === 1, String(matching.length));
		if (matching.length === 1) {
			validateGateRecord(`parity gate ${state}`, matching[0]);
			if (state === 'RealWorkbenchBootGreen' || state === 'ReleaseCandidate') {
				check(`parity gate ${state} requires real workbench env`, commandRequiresRealWorkbench(matching[0].command), String(matching[0].command));
			}
			if (state === 'BuiltInExtensionParityGreen') {
				check('parity gate BuiltInExtensionParityGreen runs built-in extension smoke preflight', matching[0].command.includes('npm run smoketest:tauri:builtin-extensions'), String(matching[0].command));
				validateBuiltInExtensionParityEvidence(matching[0].builtInExtensionParity, currentIndex >= expectedStates.indexOf('BuiltInExtensionParityGreen') || matching[0].lastCiResult?.status === 'pass');
			}
		}
	}

	validateGateRecord('release validation gate', manifest.releaseValidation);
	check('release validation forbids scaffold fallback', manifest.releaseValidation?.permitsScaffoldFallback === false, String(manifest.releaseValidation?.permitsScaffoldFallback));
	check('release validation requires real workbench env', commandRequiresRealWorkbench(manifest.releaseValidation?.command), String(manifest.releaseValidation?.command));
	check('release validation forbids src-tauri/www/index.html', Array.isArray(manifest.releaseValidation?.forbiddenWorkbenchSources) && manifest.releaseValidation.forbiddenWorkbenchSources.includes('src-tauri/www/index.html'), JSON.stringify(manifest.releaseValidation?.forbiddenWorkbenchSources));
	check('release validation allowed sources exclude src-tauri/www/index.html', Array.isArray(manifest.releaseValidation?.allowedWorkbenchSources) && !manifest.releaseValidation.allowedWorkbenchSources.includes('src-tauri/www/index.html'), JSON.stringify(manifest.releaseValidation?.allowedWorkbenchSources));

	validateGateRecord('rust lint gate', manifest.rustLintGate);
	check('rust lint gate owner matches Tauri runtime', manifest.rustLintGate?.owner === 'tauri-runtime', String(manifest.rustLintGate?.owner));
	check('rust lint gate uses clippy warnings-denied command', manifest.rustLintGate?.command === rustLintCommand, String(manifest.rustLintGate?.command));
	if (currentIndex >= expectedStates.indexOf('SignedPackageGreen')) {
		validateSignedPackageEvidence(gates.find(gate => gate?.state === 'SignedPackageGreen')?.signedPackageEvidence);
	}

	if (manifest.currentState === terminalState) {
		check('ReleaseCandidate rust lint gate passes', manifest.rustLintGate?.lastCiResult?.status === 'pass', String(manifest.rustLintGate?.lastCiResult?.status));
		validateCompleteCiEvidence('ReleaseCandidate rust lint gate', manifest.rustLintGate?.lastCiResult);
	} else {
		check('pre-ReleaseCandidate rust lint gate remains failing', manifest.rustLintGate?.lastCiResult?.status === 'fail', String(manifest.rustLintGate?.lastCiResult?.status));
	}

	validateStateProgression(manifest, gates);
	validateReleaseValidationConsistency(manifest, gates);
	validatePackageMigrationStatus(manifest);
}

function lineIsAllowedPolicy(line) {
	const normalized = line.toLowerCase();
	return allowedCompletionPolicyPhrases.some(phrase => normalized.includes(phrase));
}

function getPullRequestClaimSources() {
	const sources = [];
	for (const [name, value] of Object.entries({
		'pr-title': process.env.CODE_TAURI_PR_TITLE || process.env.PR_TITLE || process.env.GITHUB_PR_TITLE,
		'pr-summary': process.env.CODE_TAURI_PR_SUMMARY || process.env.CODE_TAURI_PR_BODY || process.env.PR_BODY || process.env.GITHUB_PR_BODY,
	})) {
		if (isNonEmptyString(value)) {
			sources.push({ name, text: value });
		}
	}

	if (isNonEmptyString(githubEventPath) && existsSync(githubEventPath)) {
		try {
			const event = JSON.parse(readFileSync(githubEventPath, 'utf8'));
			for (const [name, value] of Object.entries({
				'github-event pull_request.title': event.pull_request?.title,
				'github-event pull_request.body': event.pull_request?.body,
			})) {
				if (isNonEmptyString(value)) {
					sources.push({ name, text: value });
				}
			}
		} catch (error) {
			check('completion claim scan GitHub event parses JSON', false, error.message);
		}
	}

	return sources;
}

function completionClaimPattern() {
	return /\b(?:migration[-\s]+(?:is\s+|fully\s+)?(?:complete|completed|done|finished)|(?:complete|completed|finished)\s+(?:the\s+)?(?:tauri\s+)?migration|full\s+(?:tauri\s+)?migration|(?:tauri\s+)?migration[-\s]+(?:is\s+)?(?:release[-\s]?ready|ready\s+for\s+release)|(?:tauri\s+)?release[-\s]?ready)\b/i;
}

function scanCompletionClaims(currentState) {
	if (currentState === terminalState) {
		check('migration completion claim scan skipped at ReleaseCandidate', true);
		return;
	}

	const claimPattern = completionClaimPattern();
	const matches = [];
	for (const source of getPullRequestClaimSources()) {
		source.text.split(/\r?\n/).forEach((line, index) => {
			if (claimPattern.test(line) && !lineIsAllowedPolicy(line)) {
				matches.push(`${source.name}:${index + 1}: ${line.trim()}`);
			}
		});
	}

	check('pre-ReleaseCandidate PR metadata does not claim migration complete', matches.length === 0, matches.join(' | '));
}

function validateTauriDocClaims() {
	const result = spawnSync(process.execPath, [path.join(repoRoot, 'scripts', 'tauri-doc-claim-check.mjs')], {
		cwd: repoRoot,
		encoding: 'utf8',
		env: { ...process.env, CODE_TAURI_PARITY_MANIFEST_PATH: manifestPath, CODE_TAURI_PACKAGE_JSON_PATH: packageJsonPath },
		shell: process.platform === 'win32',
	});

	if (result.stdout?.trim()) {
		console.log(result.stdout.trim());
	}

	if (result.stderr?.trim()) {
		console.error(result.stderr.trim());
	}

	check('Tauri docs completion claim check passes', result.status === 0, `node scripts/tauri-doc-claim-check.mjs`);
}

function validatePackagingDoc() {
	if (!check('packaging doc exists', existsSync(packagingDocPath), packagingDocPath)) {
		return;
	}

	const text = readFileSync(packagingDocPath, 'utf8');
	check('packaging doc says internal scaffold only', text.includes('internal scaffold only'));
	check('packaging doc records exact migration state machine', text.includes('`Scaffold -> RuntimeBuildGreen -> RealWorkbenchBootGreen -> CoreServicesParityGreen -> ExtensionApiParityGreen -> BuiltInExtensionParityGreen -> SignedPackageGreen -> ReleaseCandidate`'));
	check('packaging doc records package status manifest invariant', text.includes('`package.json` `tauriMigration.status` is scaffold metadata only') && text.includes('must match `scripts/tauri-parity-gate.manifest.json` `currentState`'));
	check('packaging doc forbids scaffold fallback for release validation', text.includes('release validation must use the real workbench bundle, not scaffold fallback') && text.includes('`src-tauri/www/index.html` remains developer-only and is disallowed for release validation.'));
	check('packaging doc requires owner command result', text.includes('owner, a repeatable CI validation command, a recorded pass result'));
	check('packaging doc requires gate CI evidence fields', text.includes('`lastCiResult.ciRunId`') && text.includes('`lastCiResult.ciArtifactUrl`') && text.includes('`lastCiResult.ciArtifactNotApplicableReason`'));
}

function unique(values) {
	return [...new Set(values)];
}

function validateTauriCommandSecurity() {
	const runtime = readText(tauriRuntimePath, 'tauri runtime source');
	const commands = readText(tauriCommandsPath, 'tauri commands source');
	if (!runtime || !commands) {
		return;
	}

	const commandNames = unique([...commands.matchAll(/#\[(?:cfg_attr\(feature = "runtime",\s*)?tauri::command\)?\]\s*pub\s+async\s+fn\s+([A-Za-z0-9_]+)/g)].map(match => match[1])).sort();
	const handlerBlock = runtime.match(/tauri::generate_handler!\[([\s\S]*?)\]/)?.[1] ?? '';
	const handlerNames = unique([...handlerBlock.matchAll(/crate::commands::([A-Za-z0-9_]+)/g)].map(match => match[1])).sort();

	check('tauri command handlers use explicit generate_handler allowlist', handlerNames.length > 0 && !handlerBlock.includes('*'), handlerBlock.trim());
	check('tauri command allowlist matches command exports', arraysEqual(commandNames, handlerNames), `commands=${JSON.stringify(commandNames)} handlers=${JSON.stringify(handlerNames)}`);
}

function validateTauriSidecarTokenLogging() {
	const sidecar = readText(tauriSidecarPath, 'tauri extension sidecar source');
	if (!sidecar) {
		return;
	}

	const loggingBlocks = [...sidecar.matchAll(/tracing::[a-z_]+!\([\s\S]*?\);/g)].map(match => match[0]);
	const tokenLoggingBlocks = loggingBlocks.filter(block => /token|handshake/i.test(block));
	check('extension sidecar tracing excludes token and handshake fields', tokenLoggingBlocks.length === 0, tokenLoggingBlocks.join(' | '));
}

function validateTauriSubscriptionScoping() {
	const subscriptions = readText(tauriSubscriptionManagerPath, 'tauri subscription manager source');
	if (!subscriptions) {
		return;
	}

	check('subscription events emit to window_label scope', /emit_to\(&self\.app_handle,\s*window_label,\s*CHANNEL_EVENT_NAME,\s*message\)/.test(subscriptions), 'expected tauri::Emitter::emit_to window_label');
}

function validateTauriCsp() {
	const config = readJson(tauriConfigPath, 'tauri config');
	const csp = config?.app?.security?.csp;
	check('tauri CSP string configured', isNonEmptyString(csp), String(csp));
	if (!isNonEmptyString(csp)) {
		return;
	}

	for (const directive of [
		`default-src 'self'`,
		`script-src 'self' 'wasm-unsafe-eval'`,
		`frame-ancestors 'none'`,
		`base-uri 'none'`,
		`object-src 'none'`,
	]) {
		check(`tauri CSP includes ${directive}`, csp.includes(directive), csp);
	}
	check('tauri CSP excludes unsafe script inline and eval', !/script-src[^;]*(?:'unsafe-inline'|'unsafe-eval')/.test(csp), csp);
}

const manifest = readJson(manifestPath);
validateManifest(manifest);
scanCompletionClaims(manifest?.currentState);
validateTauriDocClaims();
validatePackagingDoc();
validateParityGateCiWiring();
validateTauriCommandSecurity();
validateTauriSidecarTokenLogging();
validateTauriSubscriptionScoping();
validateTauriCsp();

console.log(`summary failures=${failures.length}`);

if (failures.length) {
	process.exit(1);
}
