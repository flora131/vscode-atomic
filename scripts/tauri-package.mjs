#!/usr/bin/env node
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { spawnSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const tauriConfig = path.join(repoRoot, 'src-tauri', 'tauri.conf.json');
const tauriManifest = path.join(repoRoot, 'src-tauri', 'Cargo.toml');
const scaffoldWorkbenchPath = path.join(repoRoot, 'src-tauri', 'www', 'index.html');
const generatedWorkbenchPath = path.join(repoRoot, 'out', 'vs', 'code', 'browser', 'workbench', 'workbench.html');
const parityManifestPath = process.env.CODE_TAURI_PARITY_MANIFEST_PATH || path.join(repoRoot, 'scripts', 'tauri-parity-gate.manifest.json');
const releaseCandidateState = 'ReleaseCandidate';
const requiredReleaseGateStates = ['SignedPackageGreen', releaseCandidateState];

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
	console.log(`Usage: node scripts/tauri-package.mjs [options]

Creates the VS Code Atomic Tauri packaging command plan. Defaults to dry-run.

Options:
  --execute              Run commands instead of printing them.
  --release-validate     Validate release packaging invariants without running commands.
  --flavor code-tauri    Packaging flavor. Only code-tauri is supported.
  --target <triple>      Optional Rust target triple.
  --bundles <list>       Tauri bundle targets, default: all.
  --skip-frontend        Skip TypeScript bundle build.
  --skip-rust-check      Skip Rust compile check before bundle.
  --sign                 Enable signing placeholder checks.
  --notarize             Enable notarization placeholder checks.

Signing/notarization secrets are supplied by CI environment variables; this script never embeds them.`);
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

function isNonEmptyString(value) {
	return typeof value === 'string' && value.trim().length > 0;
}

function commandRequiresRealWorkbench(command) {
	return isNonEmptyString(command)
		&& command.includes('CODE_TAURI_REQUIRE_REAL_WORKBENCH=1')
		&& (
			command.includes('CODE_TAURI_WORKBENCH_URL=')
			|| command.includes('CODE_TAURI_WORKBENCH_PATH=')
			|| command.includes('out/vs/code/browser/workbench/workbench.html')
		);
}

function includesScaffoldFallback(sources) {
	return Array.isArray(sources) && sources.includes('src-tauri/www/index.html');
}

function isScaffoldWorkbenchPath(workbenchPath) {
	return isNonEmptyString(workbenchPath) && path.resolve(repoRoot, workbenchPath) === scaffoldWorkbenchPath;
}

function hasReleaseWorkbenchSource() {
	return isNonEmptyString(process.env.CODE_TAURI_WORKBENCH_URL)
		|| (isNonEmptyString(process.env.CODE_TAURI_WORKBENCH_PATH) && !isScaffoldWorkbenchPath(process.env.CODE_TAURI_WORKBENCH_PATH))
		|| existsSync(generatedWorkbenchPath);
}

function gatePassed(gates, state) {
	return gates.find(gate => gate?.state === state)?.lastCiResult?.status === 'pass';
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

function collectReleaseInvariantFailures() {
	const manifest = readParityManifest();
	const releaseValidation = manifest.releaseValidation;
	const gates = Array.isArray(manifest.gates) ? manifest.gates : [];
	const failures = [];

	addFailure(failures, 'package release requires ReleaseCandidate parity state', manifest.currentState === releaseCandidateState, String(manifest.currentState));
	addFailure(failures, 'package release requires release validation pass', releaseValidation?.lastCiResult?.status === 'pass', String(releaseValidation?.lastCiResult?.status));
	addFailure(failures, 'package release forbids scaffold fallback', releaseValidation?.permitsScaffoldFallback === false, String(releaseValidation?.permitsScaffoldFallback));
	addFailure(failures, 'package release requires real workbench env', commandRequiresRealWorkbench(releaseValidation?.command), String(releaseValidation?.command));
	addFailure(failures, 'package release forbids src-tauri/www/index.html', includesScaffoldFallback(releaseValidation?.forbiddenWorkbenchSources), JSON.stringify(releaseValidation?.forbiddenWorkbenchSources));
	addFailure(failures, 'package release allowed sources exclude src-tauri/www/index.html', Array.isArray(releaseValidation?.allowedWorkbenchSources) && !includesScaffoldFallback(releaseValidation.allowedWorkbenchSources), JSON.stringify(releaseValidation?.allowedWorkbenchSources));
	addFailure(failures, 'package release requires signed package plan', args.has('--sign'), String(args.has('--sign')));
	addFailure(failures, 'package release requires CODE_TAURI_REQUIRE_REAL_WORKBENCH=1', process.env.CODE_TAURI_REQUIRE_REAL_WORKBENCH === '1', String(process.env.CODE_TAURI_REQUIRE_REAL_WORKBENCH));
	addFailure(failures, 'package release requires real workbench source', hasReleaseWorkbenchSource(), process.env.CODE_TAURI_WORKBENCH_URL || process.env.CODE_TAURI_WORKBENCH_PATH || generatedWorkbenchPath);
	addFailure(failures, 'package release forbids scaffold workbench path', !isScaffoldWorkbenchPath(process.env.CODE_TAURI_WORKBENCH_PATH), String(process.env.CODE_TAURI_WORKBENCH_PATH));

	for (const state of requiredReleaseGateStates) {
		addFailure(failures, `package release requires ${state} gate pass`, gatePassed(gates, state), String(gates.find(gate => gate?.state === state)?.lastCiResult?.status));
	}

	return failures;
}

function formatReleaseInvariantFailures(failures) {
	return failures.map(failure => `- ${failure}`).join('\n');
}

function requireReleaseInvariants(mode) {
	const failures = collectReleaseInvariantFailures();
	if (failures.length) {
		throw new Error(`Tauri package ${mode} validation failed:\n${formatReleaseInvariantFailures(failures)}`);
	}
}

if (releaseValidate) {
	requireReleaseInvariants('release');
	console.log('Tauri package release validation passed.');
	process.exit(0);
}

let dryRunReleaseFailures = [];
if (!execute) {
	dryRunReleaseFailures = collectReleaseInvariantFailures();
} else {
	requireReleaseInvariants('execute');
}

const commandPlan = [];

if (!args.has('--skip-frontend')) {
	commandPlan.push(['npm', ['run', 'compile-build']]);
}

if (!args.has('--skip-rust-check')) {
	commandPlan.push(['cargo', ['build', '--manifest-path', tauriManifest, '--features', 'runtime', ...(target ? ['--target', target] : [])]]);
}

commandPlan.push(['cargo', ['tauri', 'build', '--config', tauriConfig, '--bundles', bundles, ...(target ? ['--target', target] : [])]]);

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
if (dryRunReleaseFailures.length) {
	console.warn(`Warning: Tauri package dry-run release invariants are not satisfied:\n${formatReleaseInvariantFailures(dryRunReleaseFailures)}`);
}
console.log('Command plan:');
for (const [command, commandArgs] of commandPlan) {
	console.log(`  ${[command, ...commandArgs].join(' ')}`);
}

for (const note of signingNotes) {
	console.log(`Note: ${note}`);
}

if (!execute) {
	process.exit(0);
}

for (const [command, commandArgs] of commandPlan) {
	const result = spawnSync(command, commandArgs, { cwd: repoRoot, stdio: 'inherit', shell: process.platform === 'win32' });
	if (result.status !== 0) {
		process.exit(result.status ?? 1);
	}
}
