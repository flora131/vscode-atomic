/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { spawnSync } from 'node:child_process';
import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const canonicalRepoRoot = realpathSync.native(repoRoot);
const tauriConfig = path.join(repoRoot, 'src-tauri', 'tauri.conf.json');
const tauriManifest = path.join(repoRoot, 'src-tauri', 'Cargo.toml');
const generatedReleaseConfigName = 'tauri.release.conf.json';
const scaffoldWorkbenchSource = 'src-tauri/www/index.html';
const requiredReleaseWorkbenchSource = 'out/vs/code/browser/workbench/workbench.html';
const requiredBundledWorkbenchSource = `src-tauri/www/${requiredReleaseWorkbenchSource}`;
const scaffoldWorkbenchPath = sourcePath(scaffoldWorkbenchSource);
const requiredReleaseWorkbenchParts = requiredReleaseWorkbenchSource.split('/');
const generatedWorkbenchPath = path.join(repoRoot, 'out', 'vs', 'code', 'browser', 'workbench', 'workbench.html');
const generatedWorkbenchRoot = path.join(repoRoot, 'out');
const workbenchUrlEnv = 'CODE_TAURI_WORKBENCH_URL';
const workbenchPathEnv = 'CODE_TAURI_WORKBENCH_PATH';
const parityManifestPath = process.env.CODE_TAURI_PARITY_MANIFEST_PATH || path.join(repoRoot, 'scripts', 'tauri-parity-gate.manifest.json');
const releaseCandidateState = 'ReleaseCandidate';
const requiredReleaseGateStates = ['SignedPackageGreen', releaseCandidateState];
const notarizationStatuses = new Set(['pass', 'not_required']);
const requiredSignedPackageArtifactOses = ['macos', 'windows', 'linux'];
const secretLikeKeyPattern = /(?:private[_-]?key|password|secret|token|credential|certificate[_-]?(?:path|file|password|content|data)|p12|pfx)/i;
const secretLikeValuePattern = /(?:-----BEGIN [A-Z ]*(?:PRIVATE KEY|CERTIFICATE)-----|TAURI_SIGNING_PRIVATE_KEY=|TAURI_SIGNING_PRIVATE_KEY_PASSWORD=|APPLE_PASSWORD=|PFX_PASSWORD=|P12_PASSWORD=|CERTIFICATE_PASSWORD=)/i;
const artifactOsCheckRequirements = new Map([
	['macos', {
		notarizationStatuses: new Set(['pass']),
		commandNeedles: ['codesign', 'spctl', 'hdiutil', 'cp -R', 'open -W'],
	}],
	['windows', {
		notarizationStatuses,
		commandNeedles: ['Get-AuthenticodeSignature', 'Start-Process', '--version'],
	}],
	['linux', {
		notarizationStatuses,
		commandNeedles: ['gpg --verify', 'chmod +x', '--appimage-extract-and-run', '--version'],
	}],
]);
const validationFailurePrefixes = new Map([
	['frontend', 'Tauri package frontend validation failed:'],
	['execute', 'Tauri package execute validation failed:'],
]);
const packageMarkerMetric = 'package.marker';
const requiredReleaseWorkbenchEnv = Object.freeze({
	CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1',
	[workbenchPathEnv]: requiredReleaseWorkbenchSource,
});

const args = new Set(process.argv.slice(2));
const values = new Map();
for (let i = 2; i < process.argv.length; i++) {
	const arg = process.argv[i];
	if (arg.startsWith('--') && process.argv[i + 1] && !process.argv[i + 1].startsWith('--')) {
		values.set(arg, process.argv[++i]);
	}
}

const execute = args.has('--execute');
const releaseValidate = args.has('--release-validate');
const flavor = values.get('--flavor') ?? 'code-tauri';
const target = values.get('--target');
const bundles = values.get('--bundles') ?? 'all';

if (args.has('--help')) {
	console.log([
		'Usage: node scripts/tauri-package.mjs [options]',
		'',
		'Creates the VS Code Atomic Tauri packaging command plan. Defaults to dry-run.',
		'',
		'Options:',
		'  --execute              Run commands instead of printing them.',
		'  --release-validate     Validate release packaging invariants without running commands.',
		'  --flavor code-tauri    Packaging flavor. Only code-tauri is supported.',
		'  --target <triple>      Optional Rust target triple.',
		'  --bundles <list>       Tauri bundle targets, default: all.',
		'  --skip-frontend        Skip TypeScript bundle build.',
		'  --skip-rust-check      Skip Rust compile check before bundle.',
		'  --sign                 Enable signing placeholder checks.',
		'  --notarize             Enable notarization placeholder checks.',
		'',
		'Signing/notarization secrets are supplied by CI environment variables; this script never embeds them.',
	].join('\n'));
	process.exit(0);
}

if (flavor !== 'code-tauri') {
	throw new Error(`Unsupported Tauri packaging flavor: ${flavor}`);
}

if (!existsSync(tauriConfig)) {
	throw new Error(`Missing Tauri config: ${tauriConfig}`);
}

if (!existsSync(tauriManifest)) {
	throw new Error(`Missing Tauri manifest: ${tauriManifest}`);
}

const parsedTauriConfig = readTauriConfig();
const frontendDist = resolveFrontendDist(parsedTauriConfig);
const packagedWorkbenchPath = path.join(frontendDist, ...requiredReleaseWorkbenchParts);

function sourcePath(source) {
	return path.join(repoRoot, ...source.split('/'));
}

function isNonEmptyString(value) {
	return typeof value === 'string' && value.trim().length > 0;
}

function isPlainObject(value) {
	return value !== null && typeof value === 'object' && !Array.isArray(value);
}

function readTauriConfig() {
	try {
		return JSON.parse(readFileSync(tauriConfig, 'utf8'));
	} catch (error) {
		throw new Error(`Invalid Tauri config JSON: ${error.message}`);
	}
}

function emitPackageMarker(marker, context = {}) {
	console.log(`Tauri package marker: ${JSON.stringify({
		name: packageMarkerMetric,
		marker,
		value: 1,
		unit: 'count',
		context: {
			flavor,
			mode: execute ? 'execute' : releaseValidate ? 'release-validate' : 'dry-run',
			...context,
		},
	})}`);
}

function commandMarkerContext(command, commandArgs) {
	return {
		command: [command, ...commandArgs].join(' '),
		step: isCompileBuildStep(command, commandArgs) ? 'frontend'
			: isRustBuildStep(command, commandArgs) ? 'rust-check'
				: isTauriBuildStep(command, commandArgs) ? 'tauri-build'
					: 'unknown',
	};
}

function resolveFrontendDist(config) {
	const configuredFrontendDist = config?.build?.frontendDist;
	if (!isNonEmptyString(configuredFrontendDist)) {
		throw new Error('Missing Tauri build.frontendDist');
	}

	return path.resolve(path.dirname(tauriConfig), configuredFrontendDist);
}

function cloneJson(value) {
	return JSON.parse(JSON.stringify(value));
}

function resolveReleaseTauriConfig(config) {
	const releaseConfig = cloneJson(config);
	if (!isPlainObject(releaseConfig.bundle)) {
		releaseConfig.bundle = {};
	}
	releaseConfig.bundle.active = true;
	return releaseConfig;
}

function releaseTauriConfigHasActiveBundle(config) {
	return resolveReleaseTauriConfig(config).bundle?.active === true;
}

function writeGeneratedReleaseTauriConfig(config) {
	const tempDir = mkdtempSync(path.join(tmpdir(), 'vscode-atomic-tauri-release-'));
	const generatedReleaseConfig = path.join(tempDir, generatedReleaseConfigName);
	writeFileSync(generatedReleaseConfig, `${JSON.stringify(resolveReleaseTauriConfig(config), null, 2)}\n`);
	return { generatedReleaseConfig, tempDir };
}

function resolvePackageTauriConfig() {
	if (parsedTauriConfig.bundle?.active === true) {
		return { configPath: tauriConfig, cleanup() {} };
	}

	const { generatedReleaseConfig, tempDir } = writeGeneratedReleaseTauriConfig(parsedTauriConfig);
	return {
		configPath: generatedReleaseConfig,
		cleanup: () => rmSync(tempDir, { recursive: true, force: true }),
	};
}

function requireGeneratedWorkbenchAsset() {
	requireWorkbenchAsset(generatedWorkbenchPath, `package release requires real workbench source ${requiredReleaseWorkbenchSource}`);
}

function copyGeneratedWorkbenchAssets() {
	mkdirSync(frontendDist, { recursive: true });
	rmSync(path.join(frontendDist, 'out'), { recursive: true, force: true });
	cpSync(generatedWorkbenchRoot, path.join(frontendDist, 'out'), { recursive: true, dereference: false, force: true });
}

function requireGeneratedWorkbenchAssetCopy() {
	requireWorkbenchAsset(packagedWorkbenchPath, `package execute requires bundled real workbench asset ${requiredBundledWorkbenchSource}`);
	if (isScaffoldWorkbenchPath(packagedWorkbenchPath)) {
		throw new Error(`package execute forbids scaffold bundled workbench asset ${requiredBundledWorkbenchSource}`);
	}
}

function requireWorkbenchAsset(filePath, message) {
	if (!existsSync(filePath)) {
		throw new Error(message);
	}

	if (!statSync(filePath).isFile()) {
		throw new Error(`${message}: not a file`);
	}

	if (!isCanonicalPathUnderRepoRoot(filePath)) {
		throw new Error(`${message}: canonical path must stay under repository root`);
	}
}

function isExistingFile(filePath) {
	return existsSync(filePath) && statSync(filePath).isFile();
}

function commandRequiresRealWorkbench(command) {
	return isNonEmptyString(command)
		&& command.includes('CODE_TAURI_REQUIRE_REAL_WORKBENCH=1');
}

function commandUsesRequiredReleaseWorkbenchSource(command) {
	return isNonEmptyString(command)
		&& command.includes(`${workbenchPathEnv}=${requiredReleaseWorkbenchSource}`);
}

function commandUsesRequiredReleaseValidationInvocation(command) {
	return isNonEmptyString(command)
		&& command.includes(`CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 ${workbenchPathEnv}=${requiredReleaseWorkbenchSource} npm run package:tauri:release-validate`);
}

function commandUsesScaffoldWorkbenchSource(command) {
	return isNonEmptyString(command)
		&& command.includes(scaffoldWorkbenchSource);
}

function includesScaffoldWorkbenchSource(sources) {
	return Array.isArray(sources) && sources.includes(scaffoldWorkbenchSource);
}

function resolveWorkbenchPath(workbenchPath) {
	if (!isNonEmptyString(workbenchPath)) {
		return undefined;
	}

	const candidate = path.isAbsolute(workbenchPath) ? workbenchPath : path.resolve(repoRoot, workbenchPath);
	return path.normalize(candidate);
}

function canonicalWorkbenchPath(workbenchPath) {
	const resolvedWorkbenchPath = resolveWorkbenchPath(workbenchPath);
	if (!resolvedWorkbenchPath || !existsSync(resolvedWorkbenchPath)) {
		return undefined;
	}

	return realpathSync.native(resolvedWorkbenchPath);
}

function isCanonicalPathUnderRepoRoot(workbenchPath) {
	const canonicalPath = canonicalWorkbenchPath(workbenchPath);
	if (canonicalPath === undefined) {
		return false;
	}

	const relativePath = path.relative(canonicalRepoRoot, canonicalPath);
	return relativePath === '' || (!relativePath.startsWith('..') && !path.isAbsolute(relativePath));
}

function isSameWorkbenchFile(leftPath, rightPath) {
	if (!existsSync(leftPath) || !existsSync(rightPath)) {
		return false;
	}

	const left = statSync(leftPath);
	const right = statSync(rightPath);
	return left.isFile()
		&& right.isFile()
		&& left.dev === right.dev
		&& left.ino === right.ino;
}

function isScaffoldWorkbenchPath(workbenchPath) {
	const resolvedWorkbenchPath = resolveWorkbenchPath(workbenchPath);
	if (!resolvedWorkbenchPath) {
		return false;
	}

	if (resolvedWorkbenchPath === scaffoldWorkbenchPath) {
		return true;
	}

	if (!existsSync(resolvedWorkbenchPath)) {
		return false;
	}

	const canonicalScaffoldPath = canonicalWorkbenchPath(scaffoldWorkbenchPath);
	const canonicalPath = canonicalWorkbenchPath(resolvedWorkbenchPath);
	return (canonicalScaffoldPath !== undefined && canonicalPath === canonicalScaffoldPath)
		|| isSameWorkbenchFile(resolvedWorkbenchPath, scaffoldWorkbenchPath);
}

function hasOwnEnv(name) {
	return Object.prototype.hasOwnProperty.call(process.env, name);
}

function resolveReleaseWorkbenchPathSource(workbenchPath, sourceName) {
	const resolvedWorkbenchPath = resolveWorkbenchPath(workbenchPath);
	if (resolvedWorkbenchPath === undefined) {
		return {
			ok: false,
			isScaffold: false,
			isUnderRepoRoot: false,
			detail: `${sourceName} is empty`,
		};
	}

	const isScaffold = isScaffoldWorkbenchPath(resolvedWorkbenchPath);
	if (!existsSync(resolvedWorkbenchPath)) {
		return {
			ok: false,
			isScaffold,
			isUnderRepoRoot: false,
			detail: `${sourceName} missing resolved path ${resolvedWorkbenchPath}`,
		};
	}

	return {
		ok: !isScaffold,
		isScaffold,
		isUnderRepoRoot: isCanonicalPathUnderRepoRoot(resolvedWorkbenchPath),
		detail: resolvedWorkbenchPath,
	};
}

function releaseWorkbenchPathUsesRequiredSource(workbenchPath) {
	return workbenchPath === requiredReleaseWorkbenchSource;
}

function isCompileBuildStep(command, commandArgs) {
	return command === 'npm' && commandArgs[0] === 'run' && commandArgs[1] === 'compile-build';
}

function isRustBuildStep(command, commandArgs) {
	return command === 'cargo' && commandArgs[0] === 'build';
}

function isTauriBuildStep(command, commandArgs) {
	return command === 'cargo' && commandArgs[0] === 'tauri' && commandArgs[1] === 'build';
}

function resolveReleaseWorkbenchSource() {
	return resolveReleaseWorkbenchPathSource(process.env[workbenchPathEnv], workbenchPathEnv);
}

function gatePassed(gates, state) {
	return getGate(gates, state)?.lastCiResult?.status === 'pass';
}

function getGate(gates, state) {
	return gates.find(gate => gate?.state === state);
}

function notarizationExpectationLabel(os) {
	return os === 'macos' ? 'pass' : 'pass or not_required';
}

function signedPackageArtifactFailures(artifact, index) {
	const failures = [];
	const prefix = `package release requires SignedPackageGreen signedPackageEvidence.artifacts[${index}]`;
	const osRequirements = artifactOsCheckRequirements.get(artifact?.os);
	addFailure(failures, `${prefix}.os`, isNonEmptyString(artifact?.os), String(artifact?.os));
	addFailure(failures, `${prefix}.os supported`, osRequirements !== undefined, String(artifact?.os));
	addFailure(failures, `${prefix}.artifactPath`, isNonEmptyString(artifact?.artifactPath), String(artifact?.artifactPath));
	addFailure(failures, `${prefix}.signatureStatus pass`, artifact?.signatureStatus === 'pass', String(artifact?.signatureStatus));
	addFailure(failures, `${prefix}.notarizationStatus ${notarizationExpectationLabel(artifact?.os)}`, osRequirements?.notarizationStatuses.has(artifact?.notarizationStatus) === true, String(artifact?.notarizationStatus));
	addFailure(failures, `${prefix}.installStatus pass`, artifact?.installStatus === 'pass', String(artifact?.installStatus));
	addFailure(failures, `${prefix}.launchStatus pass`, artifact?.launchStatus === 'pass', String(artifact?.launchStatus));
	addFailure(failures, `${prefix}.osCheckCommand`, isNonEmptyString(artifact?.osCheckCommand), String(artifact?.osCheckCommand));
	for (const needle of osRequirements?.commandNeedles ?? []) {
		addFailure(failures, `${prefix}.osCheckCommand includes ${needle}`, artifact?.osCheckCommand?.includes(needle) === true, String(artifact?.osCheckCommand));
	}
	failures.push(...secretSafetyFailures(artifact, prefix));
	return failures;
}

function secretSafetyFailures(value, prefix) {
	const failures = [];
	collectSecretSafetyFailures(value, prefix, failures);
	return failures;
}

function collectSecretSafetyFailures(value, prefix, failures) {
	if (Array.isArray(value)) {
		value.forEach((item, index) => collectSecretSafetyFailures(item, `${prefix}[${index}]`, failures));
		return;
	}

	if (isPlainObject(value)) {
		for (const [key, child] of Object.entries(value)) {
			const childPrefix = `${prefix}.${key}`;
			if (secretLikeKeyPattern.test(key)) {
				failures.push(`${childPrefix} must not store signing secret metadata in repository evidence`);
			}
			collectSecretSafetyFailures(child, childPrefix, failures);
		}
		return;
	}

	if (typeof value === 'string' && secretLikeValuePattern.test(value)) {
		failures.push(`${prefix} must not contain inline signing secret material`);
	}
}

function releaseConfigSecretFailures(config) {
	const failures = [];
	collectSecretSafetyFailures(config?.bundle ?? {}, 'src-tauri/tauri.conf.json bundle', failures);
	return failures;
}

function signedPackageEvidenceFailures(evidence) {
	if (!isPlainObject(evidence)) {
		return ['package release requires SignedPackageGreen signedPackageEvidence object'];
	}

	if (!Array.isArray(evidence.artifacts) || evidence.artifacts.length === 0) {
		return ['package release requires SignedPackageGreen signedPackageEvidence.artifacts'];
	}

	const failures = evidence.artifacts.flatMap((artifact, index) => signedPackageArtifactFailures(artifact, index));
	addFailure(failures, 'package release requires SignedPackageGreen signedPackageEvidence.artifacts exactly macos/windows/linux', evidence.artifacts.length === requiredSignedPackageArtifactOses.length, String(evidence.artifacts.length));
	const osCounts = countArtifactOses(evidence.artifacts);
	for (const os of requiredSignedPackageArtifactOses) {
		addFailure(failures, `package release requires SignedPackageGreen signedPackageEvidence.artifacts ${os}`, osCounts.get(os) === 1, String(osCounts.get(os) ?? 0));
	}
	return failures;
}

function countArtifactOses(artifacts) {
	const osCounts = new Map();
	for (const artifact of artifacts) {
		osCounts.set(artifact?.os, (osCounts.get(artifact?.os) ?? 0) + 1);
	}
	return osCounts;
}

function addFailure(failures, name, ok, detail) {
	if (!ok) {
		failures.push(`${name}${detail ? `: ${detail}` : ''}`);
	}
}

function readParityManifest() {
	if (!existsSync(parityManifestPath)) {
		throw new Error(`Missing Tauri parity gate manifest: ${parityManifestPath}`);
	}

	try {
		return JSON.parse(readFileSync(parityManifestPath, 'utf8'));
	} catch (error) {
		throw new Error(`Invalid Tauri parity gate manifest JSON: ${error.message}`);
	}
}

function collectReleaseInvariantFailures({ includeBundledWorkbenchAsset = true } = {}) {
	const manifest = readParityManifest();
	const releaseValidation = manifest.releaseValidation;
	const releaseValidationCommand = releaseValidation?.command;
	const gates = Array.isArray(manifest.gates) ? manifest.gates : [];
	const releaseWorkbenchSource = resolveReleaseWorkbenchSource();
	const bundledWorkbenchSource = resolveReleaseWorkbenchPathSource(requiredBundledWorkbenchSource, requiredBundledWorkbenchSource);
	const failures = [];
	const releaseChecks = [
		['package release requires ReleaseCandidate parity state', manifest.currentState === releaseCandidateState, String(manifest.currentState)],
		['package release requires Tauri release config bundle.active true', releaseTauriConfigHasActiveBundle(parsedTauriConfig), String(resolveReleaseTauriConfig(parsedTauriConfig).bundle?.active)],
		['package release requires release validation pass', releaseValidation?.lastCiResult?.status === 'pass', String(releaseValidation?.lastCiResult?.status)],
		['package release forbids scaffold fallback', releaseValidation?.permitsScaffoldFallback === false, String(releaseValidation?.permitsScaffoldFallback)],
		['package release requires real workbench env', commandRequiresRealWorkbench(releaseValidationCommand), String(releaseValidationCommand)],
		['package release command requires real workbench source', commandUsesRequiredReleaseWorkbenchSource(releaseValidationCommand), String(releaseValidationCommand)],
		['package release command requires real workbench release validation invocation', commandUsesRequiredReleaseValidationInvocation(releaseValidationCommand), String(releaseValidationCommand)],
		['package release command forbids scaffold workbench source', !commandUsesScaffoldWorkbenchSource(releaseValidationCommand), String(releaseValidationCommand)],
		['package release forbids src-tauri/www/index.html', includesScaffoldWorkbenchSource(releaseValidation?.forbiddenWorkbenchSources), JSON.stringify(releaseValidation?.forbiddenWorkbenchSources)],
		['package release allowed sources exclude src-tauri/www/index.html', Array.isArray(releaseValidation?.allowedWorkbenchSources) && !includesScaffoldWorkbenchSource(releaseValidation.allowedWorkbenchSources), JSON.stringify(releaseValidation?.allowedWorkbenchSources)],
		['package release requires signed package plan', args.has('--sign'), String(args.has('--sign'))],
		['package release requires CODE_TAURI_REQUIRE_REAL_WORKBENCH=1', process.env.CODE_TAURI_REQUIRE_REAL_WORKBENCH === '1', String(process.env.CODE_TAURI_REQUIRE_REAL_WORKBENCH)],
		[`package release requires ${workbenchPathEnv}=${requiredReleaseWorkbenchSource}`, releaseWorkbenchPathUsesRequiredSource(process.env[workbenchPathEnv]), String(process.env[workbenchPathEnv])],
		[`package release forbids ${workbenchUrlEnv} during release validation`, !hasOwnEnv(workbenchUrlEnv), String(process.env[workbenchUrlEnv])],
		[`package release requires real workbench source ${requiredReleaseWorkbenchSource}`, isExistingFile(generatedWorkbenchPath), requiredReleaseWorkbenchSource],
		[`package release requires real workbench source ${requiredReleaseWorkbenchSource} canonical path under repository root`, isCanonicalPathUnderRepoRoot(generatedWorkbenchPath), requiredReleaseWorkbenchSource],
		['package release requires real workbench source', releaseWorkbenchSource.ok, releaseWorkbenchSource.detail],
		['package release requires resolved workbench source canonical path under repository root', releaseWorkbenchSource.isUnderRepoRoot, releaseWorkbenchSource.detail],
		['package release forbids scaffold resolved workbench source', !releaseWorkbenchSource.isScaffold, releaseWorkbenchSource.detail],
	];
	if (includeBundledWorkbenchAsset) {
		releaseChecks.push(
			[`package release requires bundled real workbench asset ${requiredBundledWorkbenchSource}`, bundledWorkbenchSource.ok, bundledWorkbenchSource.detail],
			['package release requires bundled real workbench asset canonical path under repository root', bundledWorkbenchSource.isUnderRepoRoot, bundledWorkbenchSource.detail],
			['package release forbids scaffold bundled workbench asset', !bundledWorkbenchSource.isScaffold, bundledWorkbenchSource.detail],
		);
	}

	for (const [name, ok, detail] of releaseChecks) {
		addFailure(failures, name, ok, detail);
	}

	failures.push(...releaseConfigSecretFailures(parsedTauriConfig));

	for (const state of requiredReleaseGateStates) {
		const gate = getGate(gates, state);
		addFailure(failures, `package release requires ${state} gate pass`, gatePassed(gates, state), String(gate?.lastCiResult?.status));
	}

	failures.push(...signedPackageEvidenceFailures(getGate(gates, 'SignedPackageGreen')?.signedPackageEvidence));

	return failures;
}

function formatReleaseInvariantFailures(failures) {
	return failures.map(failure => `- ${failure}`).join('\n');
}

function requireReleaseInvariants() {
	const failures = collectReleaseInvariantFailures();
	if (failures.length) {
		throw new Error(`Tauri package release validation failed:\n${formatReleaseInvariantFailures(failures)}`);
	}
}

function runValidation(label, validate) {
	try {
		validate();
	} catch (error) {
		const prefix = validationFailurePrefixes.get(label) ?? `Tauri package ${label} validation failed:`;
		throw new Error(`${prefix}\n${error.message}`);
	}
}

function runFrontendValidation() {
	runValidation('frontend', () => {
		requireGeneratedWorkbenchAsset();
	});
}

function runExecuteValidation() {
	runValidation('execute', () => {
		requireGeneratedWorkbenchAssetCopy();
	});
}

if (releaseValidate) {
	emitPackageMarker('releaseValidation.start');
	requireReleaseInvariants();
	emitPackageMarker('releaseValidation.pass');
	console.log('Tauri package release validation passed.');
	process.exit(0);
}

emitPackageMarker(execute ? 'packageExecute.start' : 'packageDryRun.start', {
	target: target ?? null,
	bundles,
});

const dryRunReleaseFailures = execute ? [] : collectReleaseInvariantFailures();

const commandPlan = [];

if (!args.has('--skip-frontend')) {
	commandPlan.push(['npm', ['run', 'compile-build']]);
}

if (!args.has('--skip-rust-check')) {
	commandPlan.push(['cargo', ['build', '--manifest-path', tauriManifest, '--features', 'runtime', ...(target ? ['--target', target] : [])]]);
}

const packageTauriConfig = execute ? resolvePackageTauriConfig() : { configPath: tauriConfig, cleanup() {} };

commandPlan.push(['cargo', ['tauri', 'build', '--config', packageTauriConfig.configPath, '--bundles', bundles, ...(target ? ['--target', target] : [])]]);

const signingNotes = [];
if (args.has('--sign')) {
	signingNotes.push('Signing enabled by CI-provided credentials only. Expected placeholders: TAURI_SIGNING_PRIVATE_KEY/TAURI_SIGNING_PRIVATE_KEY_PASSWORD for updater signatures, platform certificate variables from release pipeline for Windows/macOS codesign.');
}

if (args.has('--notarize')) {
	signingNotes.push('Notarization enabled by CI-provided Apple credentials only. Expected placeholders: APPLE_ID, APPLE_PASSWORD or App Store Connect API key variables, APPLE_TEAM_ID.');
}

console.log(`Tauri packaging flavor: ${flavor}`);
console.log(`Mode: ${execute ? 'execute' : 'dry-run'}`);
console.log(`Config: ${tauriConfig}`);
if (packageTauriConfig.configPath !== tauriConfig) {
	console.log(`Release config: ${packageTauriConfig.configPath}`);
}
console.log(`Frontend dist: ${frontendDist}`);
if (dryRunReleaseFailures.length) {
	console.warn(`Warning: Tauri package dry-run release invariants are not satisfied:\n${formatReleaseInvariantFailures(dryRunReleaseFailures)}`);
}
console.log('Command plan:');
if (args.has('--skip-frontend')) {
	console.log(`  copy ${requiredReleaseWorkbenchSource} -> ${path.relative(repoRoot, packagedWorkbenchPath)}`);
}
for (const [command, commandArgs] of commandPlan) {
	console.log(`  ${[command, ...commandArgs].join(' ')}`);
	if (isCompileBuildStep(command, commandArgs)) {
		console.log(`  copy ${requiredReleaseWorkbenchSource} -> ${path.relative(repoRoot, packagedWorkbenchPath)}`);
	}
}

for (const note of signingNotes) {
	console.log(`Note: ${note}`);
}

if (!execute) {
	emitPackageMarker('packageDryRun.pass', { commandCount: commandPlan.length });
	process.exit(0);
}

function prepareFrontendAssets() {
	runFrontendValidation();
	copyGeneratedWorkbenchAssets();
	runExecuteValidation();
}

function runCommand(command, commandArgs) {
	return spawnSync(command, commandArgs, {
		cwd: repoRoot,
		env: createReleaseWorkbenchCommandEnv(),
		stdio: 'inherit',
		shell: process.platform === 'win32',
	});
}

function createReleaseWorkbenchCommandEnv() {
	const env = { ...process.env, ...requiredReleaseWorkbenchEnv };
	delete env[workbenchUrlEnv];
	return env;
}

function runCommandPlan() {
	const releaseFailures = collectReleaseInvariantFailures({ includeBundledWorkbenchAsset: false });
	if (releaseFailures.length) {
		throw new Error(`Tauri package release validation failed:\n${formatReleaseInvariantFailures(releaseFailures)}`);
	}

	let needsBundledWorkbenchValidation = true;
	function validateBundledWorkbenchAssets() {
		if (!needsBundledWorkbenchValidation) {
			return;
		}

		runExecuteValidation();
		needsBundledWorkbenchValidation = false;
	}

	if (args.has('--skip-frontend')) {
		prepareFrontendAssets();
		needsBundledWorkbenchValidation = false;
	}

	for (const [command, commandArgs] of commandPlan) {
		if (isTauriBuildStep(command, commandArgs)) {
			validateBundledWorkbenchAssets();
		}

		emitPackageMarker('packageCommand.start', commandMarkerContext(command, commandArgs));
		const result = runCommand(command, commandArgs);
		if (result.status !== 0) {
			emitPackageMarker('packageCommand.fail', { ...commandMarkerContext(command, commandArgs), status: result.status ?? 1 });
			process.exitCode = result.status ?? 1;
			return;
		}
		emitPackageMarker('packageCommand.pass', commandMarkerContext(command, commandArgs));

		if (isCompileBuildStep(command, commandArgs)) {
			prepareFrontendAssets();
			needsBundledWorkbenchValidation = false;
		}

		if (isRustBuildStep(command, commandArgs)) {
			validateBundledWorkbenchAssets();
		}
	}
	emitPackageMarker('packageExecute.pass', { commandCount: commandPlan.length });
}

try {
	runCommandPlan();
} finally {
	packageTauriConfig.cleanup();
}
