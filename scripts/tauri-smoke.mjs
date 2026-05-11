#!/usr/bin/env node
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { spawnSync } from 'node:child_process';
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const srcTauriRoot = path.join(repoRoot, 'src-tauri');
const scaffoldWorkbenchPath = path.join(srcTauriRoot, 'www', 'index.html');
const defaultWorkbenchEntry = 'out/vs/code/browser/workbench/workbench.html';
const generatedWorkbenchPath = path.join(repoRoot, ...defaultWorkbenchEntry.split('/'));

const requiredFiles = {
	config: path.join(srcTauriRoot, 'tauri.conf.json'),
	manifest: path.join(srcTauriRoot, 'Cargo.toml'),
	runtime: path.join(srcTauriRoot, 'src', 'runtime.rs'),
	workbenchUrlResolver: path.join(srcTauriRoot, 'src', 'workbench_url_resolver.rs'),
	commands: path.join(srcTauriRoot, 'src', 'commands.rs'),
	fileService: path.join(srcTauriRoot, 'src', 'file_service.rs'),
	extensionSidecar: path.join(srcTauriRoot, 'src', 'extension_sidecar.rs'),
	observability: path.join(srcTauriRoot, 'src', 'observability.rs'),
	fallbackIndex: path.join(srcTauriRoot, 'www', 'index.html'),
	packagingDoc: path.join(srcTauriRoot, 'PACKAGING.md'),
	parityGateScript: path.join(repoRoot, 'scripts', 'tauri-parity-gate.mjs'),
	apiTestsScript: path.join(repoRoot, 'scripts', 'tauri-vscode-api-tests.mjs'),
	packageScript: path.join(repoRoot, 'scripts', 'tauri-package.mjs'),
	builtinExtensionSmokeScript: path.join(repoRoot, 'scripts', 'tauri-builtin-extension-smoke.mjs'),
	realWorkbench: generatedWorkbenchPath,
	parityGateManifest: path.join(repoRoot, 'scripts', 'tauri-parity-gate.manifest.json'),
	bridge: path.join(repoRoot, 'src', 'vs', 'platform', 'tauri', 'common', 'tauriBridge.ts'),
};

const failures = [];
const warnings = [];
const fileCommandNames = ['fs_stat', 'fs_read_file', 'fs_write_file', 'fs_delete', 'fs_mkdir', 'fs_readdir', 'fs_watch'];
const strictDependencies = usesStrictDependencies(process.env);
const validModes = new Set(['all', 'source', 'deps', 'runtime', 'workbench']);
const mode = parseMode(process.argv.slice(2));
let currentLayer = 'source';

function parseMode(args) {
	const modeArg = args.find(arg => validModes.has(arg))
		?? args.find(arg => arg.startsWith('--mode='))?.slice('--mode='.length)
		?? 'all';

	if (!validModes.has(modeArg)) {
		console.error(`fail [source] smoke mode: expected one of ${[...validModes].join(', ')}, got ${modeArg}`);
		process.exit(1);
	}

	return modeArg;
}

function shouldRunForMode(currentMode, targetMode) {
	if (targetMode === 'workbench') {
		return currentMode === 'workbench';
	}

	return currentMode === 'all' || currentMode === targetMode;
}

function shouldRun(targetMode) {
	return shouldRunForMode(mode, targetMode);
}

function withLayer(layer, callback) {
	const previousLayer = currentLayer;
	currentLayer = layer;
	try {
		return callback();
	} finally {
		currentLayer = previousLayer;
	}
}

function check(name, ok, detail) {
	const status = ok ? 'ok' : 'fail';
	console.log(`${status} [${currentLayer}] ${name}${detail ? `: ${detail}` : ''}`);
	if (!ok) {
		failures.push(`[${currentLayer}] ${name}${detail ? `: ${detail}` : ''}`);
	}
	return ok;
}

function warn(name, detail) {
	console.log(`warn [${currentLayer}] ${name}: ${detail}`);
	warnings.push(`[${currentLayer}] ${name}: ${detail}`);
}

function dependencyCheck(name, ok, warningDetail, failureDetail = warningDetail) {
	if (ok) {
		return check(name, true);
	}

	if (strictDependencies) {
		return check(name, false, failureDetail);
	}

	warn(name, warningDetail);
	return false;
}

function skipBehaviorCheck(name, detail) {
	if (strictDependencies) {
		return check(name, false, detail);
	}

	warn(name, detail);
	return false;
}

function readRequired(name) {
	const filePath = requiredFiles[name];
	if (!check(`file ${name}`, existsSync(filePath), filePath)) {
		return '';
	}

	return readFileSync(filePath, 'utf8');
}

function includesAll(source, name, needles) {
	for (const needle of needles) {
		check(`${name} contains ${needle}`, source.includes(needle));
	}
}

function extractTauriBridgeCommandNames(source) {
	const match = /export\s+interface\s+TauriBridgeCommands\s*\{([\s\S]*?)\n\}/m.exec(source);
	if (!match) {
		check('bridge contract TauriBridgeCommands exists', false, requiredFiles.bridge);
		return [];
	}

	const commandNames = [];
	const memberPattern = /^\s*(?:readonly\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:/gm;
	let member;
	while ((member = memberPattern.exec(match[1])) !== null) {
		commandNames.push(member[1]);
	}

	check('bridge contract TauriBridgeCommands has commands', commandNames.length > 0, commandNames.join(', '));
	return commandNames;
}

function extractRustGenerateHandlerCommandNames(source) {
	const match = /tauri::generate_handler!\s*\[([\s\S]*?)\]/m.exec(source);
	if (!match) {
		check('runtime tauri::generate_handler exists', false, requiredFiles.runtime);
		return [];
	}

	const commandNames = [];
	const commandPattern = /crate::commands::([A-Za-z_][A-Za-z0-9_]*)\b/g;
	let command;
	while ((command = commandPattern.exec(match[1])) !== null) {
		commandNames.push(command[1]);
	}

	check('runtime tauri::generate_handler has commands', commandNames.length > 0, commandNames.join(', '));
	return commandNames;
}

function extractRustBlock(source, openBraceIndex) {
	let depth = 0;
	for (let index = openBraceIndex; index < source.length; index++) {
		const char = source[index];
		if (char === '{') {
			depth++;
		} else if (char === '}') {
			depth--;
			if (depth === 0) {
				return source.slice(openBraceIndex + 1, index);
			}
		}
	}

	return '';
}

function extractRustFunctionBody(source, functionName) {
	const match = new RegExp(`\\bfn\\s+${functionName}\\s*\\(`).exec(source);
	if (!match) {
		return '';
	}

	const openBraceIndex = source.indexOf('{', match.index);
	return openBraceIndex === -1 ? '' : extractRustBlock(source, openBraceIndex);
}

function extractRustIfBlockBody(source, conditionSource) {
	const conditionIndex = source.indexOf(conditionSource);
	if (conditionIndex === -1) {
		return '';
	}

	const openBraceIndex = source.indexOf('{', conditionIndex);
	return openBraceIndex === -1 ? '' : extractRustBlock(source, openBraceIndex);
}

function checkHandshakeTokenSource(extensionSidecarText) {
	const tokenBody = extractRustFunctionBody(extensionSidecarText, 'generate_handshake_token');
	check('extension sidecar generate_handshake_token exists', tokenBody.length > 0, requiredFiles.extensionSidecar);

	const predictableInputs = [
		{ name: 'SystemTime', pattern: /\bSystemTime\b/ },
		{ name: 'UNIX_EPOCH', pattern: /\bUNIX_EPOCH\b/ },
		{ name: 'std::process::id', pattern: /\bstd::process::id\b/ },
		{ name: 'TOKEN_COUNTER', pattern: /\bTOKEN_COUNTER\b/ },
		{ name: 'timestamp', pattern: /\btimestamp\b/i },
		{ name: 'pid', pattern: /\bpid\b/i },
		{ name: 'counter', pattern: /\bcounter\b/i },
	];
	for (const { name, pattern } of predictableInputs) {
		check(`extension sidecar token generator avoids predictable input ${name}`, !pattern.test(tokenBody), name);
	}

	check('extension sidecar token generator uses CSPRNG API', /\bgetrandom::getrandom\b|\bOsRng\b|\bring::rand::SystemRandom\b/.test(tokenBody), 'expected getrandom, OsRng, or ring::rand::SystemRandom');
}

function checkWorkbenchPathResolverGuard(workbenchUrlResolverText) {
	const pathBranchBody = extractRustIfBlockBody(workbenchUrlResolverText, 'if let Some(path) = env_value(WORKBENCH_PATH_ENV)');
	check('workbench URL resolver CODE_TAURI_WORKBENCH_PATH branch exists', pathBranchBody.length > 0, requiredFiles.workbenchUrlResolver);

	const scaffoldGuardIndex = pathBranchBody.indexOf('is_scaffold_workbench_path(repo_root, &path)');
	const guardPanicIndex = pathBranchBody.indexOf('panic!', scaffoldGuardIndex);
	const fileExistsIndex = pathBranchBody.indexOf('file_exists(&path)');
	const fileReturnIndex = pathBranchBody.indexOf('return ResolvedWorkbenchUrl::File(path)', fileExistsIndex);
	check('workbench URL resolver rejects scaffold path before file_exists', scaffoldGuardIndex !== -1 && guardPanicIndex !== -1 && fileExistsIndex !== -1 && scaffoldGuardIndex < fileExistsIndex && guardPanicIndex < fileExistsIndex, 'expected scaffold panic before file_exists(&path)');
	check('workbench URL resolver file_exists path branch returns file only after guard', fileReturnIndex !== -1 && fileExistsIndex !== -1 && scaffoldGuardIndex !== -1 && fileExistsIndex < fileReturnIndex && scaffoldGuardIndex < fileReturnIndex, 'expected guarded return ResolvedWorkbenchUrl::File(path)');
}

function compareCommandContracts(bridgeCommandNames, runtimeCommandNames) {
	const bridgeCommandSet = new Set(bridgeCommandNames);
	const runtimeCommandSet = new Set(runtimeCommandNames);
	const missingInRuntime = bridgeCommandNames.filter(command => !runtimeCommandSet.has(command)).sort();
	const missingInBridge = runtimeCommandNames.filter(command => !bridgeCommandSet.has(command)).sort();
	const duplicateBridgeCommands = duplicateNames(bridgeCommandNames);
	const duplicateRuntimeCommands = duplicateNames(runtimeCommandNames);
	const orderDiff = diffOrderedCommandContracts(bridgeCommandNames, runtimeCommandNames);

	check('bridge contract no duplicate TauriBridgeCommands keys', duplicateBridgeCommands.length === 0, duplicateBridgeCommands.join(', '));
	check('bridge contract no duplicate Rust command registrations', duplicateRuntimeCommands.length === 0, duplicateRuntimeCommands.join(', '));
	check('P0 bridge contract no commands missing from Rust generate_handler', missingInRuntime.length === 0, missingInRuntime.join(', '));
	check('P0 bridge contract no extra Rust generate_handler commands', missingInBridge.length === 0, missingInBridge.join(', '));
	check('bridge contract TauriBridgeCommands order matches Rust generate_handler', orderDiff.length === 0, orderDiff.join('; '));
	check('P0 bridge contract TauriBridgeCommands matches Rust generate_handler exactly', missingInRuntime.length === 0 && missingInBridge.length === 0 && duplicateBridgeCommands.length === 0 && duplicateRuntimeCommands.length === 0 && orderDiff.length === 0, `bridge=[${bridgeCommandNames.join(', ')}] runtime=[${runtimeCommandNames.join(', ')}]`);
}

function diffOrderedCommandContracts(bridgeCommandNames, runtimeCommandNames) {
	const maxLength = Math.max(bridgeCommandNames.length, runtimeCommandNames.length);
	const diffs = [];
	for (let index = 0; index < maxLength; index++) {
		const bridgeCommand = bridgeCommandNames[index] ?? '<missing>';
		const runtimeCommand = runtimeCommandNames[index] ?? '<missing>';
		if (bridgeCommand !== runtimeCommand) {
			diffs.push(`${index}: bridge=${bridgeCommand} runtime=${runtimeCommand}`);
		}
	}
	return diffs;
}

function duplicateNames(names) {
	return [...new Set(names.filter((name, index) => names.indexOf(name) !== index))].sort();
}

function parseCsp(csp) {
	const directives = new Map();

	for (const rawDirective of csp.split(';')) {
		const parts = rawDirective.trim().split(/\s+/).filter(Boolean);
		if (parts.length === 0) {
			continue;
		}

		const [name, ...values] = parts;
		directives.set(name.toLowerCase(), values);
	}

	return directives;
}

function checkRequiredCsp(csp) {
	const directives = parseCsp(csp);
	const requiredDirectives = [
		{ name: 'default-src' },
		{ name: 'script-src' },
		{ name: 'object-src', values: ['\'none\''] },
		{ name: 'frame-ancestors', values: ['\'none\''] },
		{ name: 'base-uri', values: ['\'none\''] },
	];

	for (const directive of requiredDirectives) {
		const values = directives.get(directive.name);
		check(`tauri config CSP has ${directive.name}`, Array.isArray(values), csp);

		for (const value of directive.values ?? []) {
			check(`tauri config CSP ${directive.name} contains ${value}`, values?.includes(value) === true, values?.join(' '));
		}
	}
}

function commandExists(command, args = ['--version']) {
	const result = spawnSync(command, args, {
		cwd: repoRoot,
		encoding: 'utf8',
		shell: process.platform === 'win32',
	});
	return result.status === 0;
}

function runBehaviorCheck(name, command, args, options = {}) {
	const result = spawnSync(command, args, {
		cwd: options.cwd ?? repoRoot,
		encoding: 'utf8',
		env: options.env ? { ...process.env, ...options.env } : process.env,
		shell: process.platform === 'win32',
		stdio: ['ignore', 'pipe', 'pipe'],
	});

	if (result.stdout?.trim()) {
		console.log(result.stdout.trim());
	}

	if (result.stderr?.trim()) {
		console.error(result.stderr.trim());
	}

	return check(name, result.status === (options.expectedStatus ?? 0), `${command} ${args.join(' ')}`);
}

function isNonEmptyString(value) {
	return typeof value === 'string' && value.trim().length > 0;
}

function usesStrictDependencies(env = process.env) {
	return env.CODE_TAURI_STRICT_DEPS === '1' || env.CI === 'true';
}

function resolveWorkbenchPath(value) {
	if (!isNonEmptyString(value)) {
		return undefined;
	}

	const candidate = path.isAbsolute(value) ? value : path.resolve(repoRoot, value);
	return path.normalize(candidate);
}

function isScaffoldWorkbenchPath(workbenchPath) {
	return resolveWorkbenchPath(workbenchPath) === scaffoldWorkbenchPath;
}

function isDefaultWorkbenchPath(workbenchPath) {
	return resolveWorkbenchPath(workbenchPath) === generatedWorkbenchPath;
}

function addFailure(failures, name, ok, detail) {
	if (!ok) {
		failures.push(`${name}${detail === undefined ? '' : `: ${detail}`}`);
	}
}

function collectRealWorkbenchPreflightFailures(env = process.env, fileExists = existsSync) {
	const failures = [];
	const workbenchPath = env.CODE_TAURI_WORKBENCH_PATH;
	const hasDefaultWorkbenchPath = isDefaultWorkbenchPath(workbenchPath);
	const hasGeneratedWorkbenchFile = hasDefaultWorkbenchPath && fileExists(generatedWorkbenchPath);

	addFailure(failures, 'real workbench boot requires CODE_TAURI_REQUIRE_REAL_WORKBENCH=1', env.CODE_TAURI_REQUIRE_REAL_WORKBENCH === '1', String(env.CODE_TAURI_REQUIRE_REAL_WORKBENCH));
	addFailure(failures, `real workbench boot requires CODE_TAURI_WORKBENCH_PATH=${defaultWorkbenchEntry}`, hasDefaultWorkbenchPath, String(workbenchPath));
	addFailure(failures, 'real workbench boot forbids src-tauri/www/index.html', !isScaffoldWorkbenchPath(workbenchPath), String(workbenchPath));
	if (usesStrictDependencies(env)) {
		addFailure(failures, 'real workbench boot requires generated workbench file', hasGeneratedWorkbenchFile, generatedWorkbenchPath);
	}

	return failures;
}

function checkRealWorkbenchPreflight(name, env, fileExists, expectedFailures = []) {
	const preflightFailures = collectRealWorkbenchPreflightFailures(env, fileExists);
	check(name, arraysEqual(preflightFailures, expectedFailures), preflightFailures.join('; '));
}

function arraysEqual(actual, expected) {
	return actual.length === expected.length && actual.every((value, index) => value === expected[index]);
}

function cloneJson(value) {
	return JSON.parse(JSON.stringify(value));
}

function writeTempManifest(name, manifest) {
	const tempDir = mkdtempSync(path.join(tmpdir(), 'tauri-parity-gate-'));
	const manifestPath = path.join(tempDir, `${name}.json`);
	writeFileSync(manifestPath, `${JSON.stringify(manifest, null, '\t')}\n`);
	return { tempDir, manifestPath };
}

function runParityGateManifestCase(name, manifest, expectedStatus, env = {}) {
	const { tempDir, manifestPath } = writeTempManifest(name, manifest);
	try {
		runBehaviorCheck(name, 'node', [requiredFiles.parityGateScript], {
			env: { CODE_TAURI_PARITY_MANIFEST_PATH: manifestPath, ...env },
			expectedStatus,
		});
	} finally {
		rmSync(tempDir, { recursive: true, force: true });
	}
}

function runParityGatePolicyCases(baseManifest) {
	const releaseCandidateManifest = createReleaseCandidateManifest(baseManifest);
	runParityGateManifestCase('behavior parity gate accepts ReleaseCandidate with green release validation', releaseCandidateManifest, 0);
	runParityGateManifestCase('behavior parity gate accepts ReleaseCandidate completion claim', releaseCandidateManifest, 0, { CODE_TAURI_PR_TITLE: 'Tauri migration complete' });

	const scaffoldWithReleasePass = cloneJson(baseManifest);
	scaffoldWithReleasePass.releaseValidation.lastCiResult.status = 'pass';
	runParityGateManifestCase('behavior parity gate rejects scaffold release validation pass', scaffoldWithReleasePass, 1);
	runParityGateManifestCase('behavior parity gate rejects pre-ReleaseCandidate PR title completion claim', baseManifest, 1, { CODE_TAURI_PR_TITLE: 'Tauri migration complete' });
	runParityGateManifestCase('behavior parity gate rejects pre-ReleaseCandidate PR summary release claim', baseManifest, 1, { CODE_TAURI_PR_SUMMARY: 'The Tauri migration is ready for release.' });
	runParityGateManifestCase('behavior parity gate accepts pre-ReleaseCandidate policy completion text', baseManifest, 0, { CODE_TAURI_PR_SUMMARY: 'Only ReleaseCandidate may claim migration complete.' });

	const futureGateGreen = cloneJson(baseManifest);
	futureGateGreen.gates.find(gate => gate.state === 'RuntimeBuildGreen').lastCiResult.status = 'pass';
	runParityGateManifestCase('behavior parity gate rejects future green gate before currentState', futureGateGreen, 1);

	const reorderedStateMachine = cloneJson(baseManifest);
	reorderedStateMachine.migrationStateMachine = [...baseManifest.migrationStateMachine].reverse();
	runParityGateManifestCase('behavior parity gate rejects reordered migration state machine', reorderedStateMachine, 1);

	const reorderedGates = cloneJson(baseManifest);
	reorderedGates.gates = [...baseManifest.gates].reverse();
	runParityGateManifestCase('behavior parity gate rejects reordered gate records', reorderedGates, 1);

	const scaffoldFallbackPermitted = cloneJson(baseManifest);
	scaffoldFallbackPermitted.releaseValidation.permitsScaffoldFallback = true;
	runParityGateManifestCase('behavior parity gate rejects scaffold fallback release validation', scaffoldFallbackPermitted, 1);

	const scaffoldSourceAllowed = cloneJson(baseManifest);
	scaffoldSourceAllowed.releaseValidation.allowedWorkbenchSources.push('src-tauri/www/index.html');
	runParityGateManifestCase('behavior parity gate rejects scaffold in allowed workbench sources', scaffoldSourceAllowed, 1);

	const scaffoldSourceNotForbidden = cloneJson(baseManifest);
	scaffoldSourceNotForbidden.releaseValidation.forbiddenWorkbenchSources = [];
	runParityGateManifestCase('behavior parity gate rejects missing forbidden scaffold source', scaffoldSourceNotForbidden, 1);

	const releaseCandidateWithFailingRelease = cloneJson(releaseCandidateManifest);
	releaseCandidateWithFailingRelease.releaseValidation.lastCiResult.status = 'fail';
	runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate failing release validation', releaseCandidateWithFailingRelease, 1);
}

function createReleaseCandidateManifest(baseManifest) {
	const releaseCandidateManifest = cloneJson(baseManifest);
	releaseCandidateManifest.currentState = 'ReleaseCandidate';
	releaseCandidateManifest.releaseValidation.lastCiResult.status = 'pass';
	releaseCandidateManifest.releaseValidation.lastCiResult.runName = 'Tauri release validation green';
	releaseCandidateManifest.releaseValidation.lastCiResult.summary = 'Release validation passed without scaffold fallback.';
	for (const gate of releaseCandidateManifest.gates) {
		gate.lastCiResult.status = 'pass';
	}
	markSignedPackageEvidencePassed(releaseCandidateManifest);
	return releaseCandidateManifest;
}

function markSignedPackageEvidencePassed(manifest) {
	const signedPackageGate = manifest.gates.find(gate => gate.state === 'SignedPackageGreen');
	for (const artifact of signedPackageGate?.signedPackageEvidence?.artifacts ?? []) {
		artifact.signatureStatus = 'pass';
		artifact.installStatus = 'pass';
		if (artifact.notarizationStatus !== 'not_required') {
			artifact.notarizationStatus = 'pass';
		}
	}
}

function getPackagePolicyWorkbenchPath(expectations, tempDir) {
	if (typeof expectations.workbenchPath === 'function') {
		return expectations.workbenchPath(tempDir);
	}

	if (expectations.workbenchPath !== undefined) {
		return expectations.workbenchPath;
	}

	return path.join(tempDir, 'workbench.html');
}

function runPackageManifestCase(name, manifest, args, expectedStatus, expectations = {}) {
	const { tempDir, manifestPath } = writeTempManifest(name, manifest);
	try {
		const workbenchPath = getPackagePolicyWorkbenchPath(expectations, tempDir);
		if (expectations.createWorkbenchFixture !== false) {
			writeFileSync(workbenchPath, '<!doctype html><title>Real workbench smoke fixture</title>\n');
		}
		const env = {
			...process.env,
			CODE_TAURI_PARITY_MANIFEST_PATH: manifestPath,
			...(expectations.env ?? {}),
			CODE_TAURI_WORKBENCH_PATH: expectations.env?.CODE_TAURI_WORKBENCH_PATH ?? workbenchPath,
		};
		const result = spawnSync('node', [requiredFiles.packageScript, ...args], {
			cwd: repoRoot,
			encoding: 'utf8',
			env,
			shell: process.platform === 'win32',
			stdio: ['ignore', 'pipe', 'pipe'],
		});

		const stdout = result.stdout ?? '';
		const stderr = result.stderr ?? '';
		const output = `${stdout}\n${stderr}`;

		if (stdout.trim()) {
			console.log(stdout.trim());
		}

		if (stderr.trim()) {
			console.error(stderr.trim());
		}

		check(name, result.status === expectedStatus, `node ${[requiredFiles.packageScript, ...args].join(' ')}`);
		for (const needle of expectations.stdoutIncludes ?? []) {
			check(`${name} stdout contains ${needle}`, stdout.includes(needle));
		}
		for (const needle of expectations.stderrIncludes ?? []) {
			check(`${name} stderr contains ${needle}`, stderr.includes(needle));
		}
		for (const needle of expectations.outputIncludes ?? []) {
			check(`${name} output contains ${needle}`, output.includes(needle));
		}
		for (const needle of expectations.outputExcludes ?? []) {
			check(`${name} output excludes ${needle}`, !output.includes(needle));
		}
	} finally {
		rmSync(tempDir, { recursive: true, force: true });
	}
}

function runPackagePolicyCases(baseManifest) {
	runPackageManifestCase(
		'behavior package dry-run scaffold prints plan and release warning',
		baseManifest,
		['--flavor', 'code-tauri', '--sign'],
		0,
		{
			stdoutIncludes: ['Mode: dry-run', 'Command plan:', 'cargo tauri build'],
			stderrIncludes: ['Warning: Tauri package dry-run release invariants are not satisfied:', 'package release requires ReleaseCandidate parity state: Scaffold'],
		}
	);

	runPackageManifestCase(
		'behavior package release validation rejects Scaffold',
		baseManifest,
		['--release-validate', '--flavor', 'code-tauri', '--sign'],
		1,
		{
			stderrIncludes: ['Tauri package release validation failed:', 'package release requires ReleaseCandidate parity state: Scaffold'],
		}
	);

	runPackageManifestCase(
		'behavior package execute rejects Scaffold before cargo tauri build',
		baseManifest,
		['--execute', '--flavor', 'code-tauri', '--sign'],
		1,
		{
			stderrIncludes: ['Tauri package execute validation failed:', 'package release requires ReleaseCandidate parity state: Scaffold'],
			outputExcludes: ['Command plan:', 'cargo tauri build --config'],
		}
	);

	const releaseCandidateManifest = createReleaseCandidateManifest(baseManifest);
	runPackageManifestCase(
		'behavior package release validation accepts signed ReleaseCandidate',
		releaseCandidateManifest,
		['--release-validate', '--flavor', 'code-tauri', '--sign'],
		0,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1' },
			stdoutIncludes: ['Tauri package release validation passed.'],
		}
	);
	runPackageManifestCase(
		'behavior package release validation accepts explicit temp workbench path',
		releaseCandidateManifest,
		['--release-validate', '--flavor', 'code-tauri', '--sign'],
		0,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1' },
			workbenchPath: tempDir => path.join(tempDir, 'explicit-workbench.html'),
			stdoutIncludes: ['Tauri package release validation passed.'],
		}
	);

	const releaseCandidateMissingSignedPackageEvidence = createReleaseCandidateManifest(baseManifest);
	delete releaseCandidateMissingSignedPackageEvidence.gates.find(gate => gate.state === 'SignedPackageGreen').signedPackageEvidence;
	runPackageManifestCase(
		'behavior package release validation rejects missing signed package evidence',
		releaseCandidateMissingSignedPackageEvidence,
		['--release-validate', '--flavor', 'code-tauri', '--sign'],
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1' },
			stderrIncludes: ['Tauri package release validation failed:', 'package release requires SignedPackageGreen signedPackageEvidence object'],
		}
	);

	const releaseCandidateFailingSignedPackageEvidence = createReleaseCandidateManifest(baseManifest);
	releaseCandidateFailingSignedPackageEvidence.gates.find(gate => gate.state === 'SignedPackageGreen').signedPackageEvidence.artifacts[0].signatureStatus = 'fail';
	runPackageManifestCase(
		'behavior package release validation rejects failing signed package evidence',
		releaseCandidateFailingSignedPackageEvidence,
		['--release-validate', '--flavor', 'code-tauri', '--sign'],
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1' },
			stderrIncludes: ['Tauri package release validation failed:', 'package release requires SignedPackageGreen signedPackageEvidence.artifacts[0].signatureStatus pass: fail'],
		}
	);

	runPackageManifestCase(
		'behavior package release validation rejects missing real workbench requirement',
		releaseCandidateManifest,
		['--release-validate', '--flavor', 'code-tauri', '--sign'],
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '' },
			stderrIncludes: ['Tauri package release validation failed:', 'package release requires CODE_TAURI_REQUIRE_REAL_WORKBENCH=1'],
		}
	);

	runPackageManifestCase(
		'behavior package release validation rejects missing signing flag',
		releaseCandidateManifest,
		['--release-validate', '--flavor', 'code-tauri'],
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1' },
			stderrIncludes: ['Tauri package release validation failed:', 'package release requires signed package plan: false'],
		}
	);

	runPackageManifestCase(
		'behavior package release validation rejects missing real workbench source',
		releaseCandidateManifest,
		['--release-validate', '--flavor', 'code-tauri', '--sign'],
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_PATH: '' },
			stderrIncludes: ['Tauri package release validation failed:', 'package release requires real workbench source:'],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects missing explicit non-scaffold path with resolved detail',
		releaseCandidateManifest,
		['--release-validate', '--flavor', 'code-tauri', '--sign'],
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1' },
			workbenchPath: tempDir => path.join(tempDir, 'missing-workbench.html'),
			createWorkbenchFixture: false,
			stderrIncludes: ['Tauri package release validation failed:', 'package release requires real workbench source: missing resolved path'],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects relative workbench typo with resolved detail',
		releaseCandidateManifest,
		['--release-validate', '--flavor', 'code-tauri', '--sign'],
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_PATH: 'out/vs/code/browser/workbench/workbench-typo.html' },
			stderrIncludes: ['Tauri package release validation failed:', `package release requires real workbench source: missing resolved path ${path.join(repoRoot, 'out', 'vs', 'code', 'browser', 'workbench', 'workbench-typo.html')}`],
		}
	);

	runPackageManifestCase(
		'behavior package dry-run warns missing real workbench requirement',
		releaseCandidateManifest,
		['--flavor', 'code-tauri', '--sign'],
		0,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '' },
			stderrIncludes: ['Warning: Tauri package dry-run release invariants are not satisfied:', 'package release requires CODE_TAURI_REQUIRE_REAL_WORKBENCH=1'],
		}
	);

	runPackageManifestCase(
		'behavior package release validation rejects scaffold workbench path',
		releaseCandidateManifest,
		['--release-validate', '--flavor', 'code-tauri', '--sign'],
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_PATH: path.join(repoRoot, 'src-tauri', 'www', 'index.html') },
			stderrIncludes: ['Tauri package release validation failed:', 'package release requires real workbench source:', 'package release forbids scaffold workbench path:'],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects normalized scaffold workbench path',
		releaseCandidateManifest,
		['--release-validate', '--flavor', 'code-tauri', '--sign'],
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_PATH: path.join(repoRoot, 'src-tauri', 'www', '..', 'www', 'index.html') },
			stderrIncludes: ['Tauri package release validation failed:', 'package release requires real workbench source:', 'package release forbids scaffold workbench path:'],
		}
	);

	const releaseCandidateWithScaffoldFallback = createReleaseCandidateManifest(baseManifest);
	releaseCandidateWithScaffoldFallback.releaseValidation.permitsScaffoldFallback = true;
	runPackageManifestCase(
		'behavior package release validation rejects scaffold fallback permission',
		releaseCandidateWithScaffoldFallback,
		['--release-validate', '--flavor', 'code-tauri', '--sign'],
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1' },
			stderrIncludes: ['Tauri package release validation failed:', 'package release forbids scaffold fallback: true'],
		}
	);
}

function runRealWorkbenchPreflightCases() {
	const validRealWorkbenchEnv = {
		CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1',
		CODE_TAURI_WORKBENCH_PATH: defaultWorkbenchEntry,
	};

	check('behavior smoke default mode excludes workbench', shouldRunForMode('all', 'workbench') === false);
	check('behavior smoke workbench mode includes workbench', shouldRunForMode('workbench', 'workbench') === true);
	check('behavior smoke workbench mode excludes source', shouldRunForMode('workbench', 'source') === false);

	checkRealWorkbenchPreflight(
		'behavior real workbench preflight accepts generated workbench path',
		validRealWorkbenchEnv,
		filePath => filePath === generatedWorkbenchPath,
		[]
	);
	checkRealWorkbenchPreflight(
		'behavior real workbench preflight rejects scaffold release path',
		{
			CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1',
			CODE_TAURI_WORKBENCH_PATH: path.join('src-tauri', 'www', 'index.html'),
		},
		filePath => filePath === scaffoldWorkbenchPath,
		[
			`real workbench boot requires CODE_TAURI_WORKBENCH_PATH=${defaultWorkbenchEntry}: src-tauri/www/index.html`,
			'real workbench boot forbids src-tauri/www/index.html: src-tauri/www/index.html',
		]
	);
	checkRealWorkbenchPreflight(
		'behavior real workbench preflight warns locally for missing generated workbench file',
		validRealWorkbenchEnv,
		() => false,
		[]
	);
	checkRealWorkbenchPreflight(
		'behavior real workbench preflight rejects missing generated workbench file in strict deps',
		{ ...validRealWorkbenchEnv, CODE_TAURI_STRICT_DEPS: '1' },
		() => false,
		[
			`real workbench boot requires generated workbench file: ${generatedWorkbenchPath}`,
		]
	);
	checkRealWorkbenchPreflight(
		'behavior real workbench preflight rejects missing real workbench requirement',
		{
			CODE_TAURI_REQUIRE_REAL_WORKBENCH: '',
			CODE_TAURI_WORKBENCH_PATH: defaultWorkbenchEntry,
		},
		filePath => filePath === generatedWorkbenchPath,
		[
			'real workbench boot requires CODE_TAURI_REQUIRE_REAL_WORKBENCH=1: ',
		]
	);
	checkRealWorkbenchPreflight(
		'behavior real workbench preflight rejects workbench mode without env',
		{},
		() => false,
		[
			'real workbench boot requires CODE_TAURI_REQUIRE_REAL_WORKBENCH=1: undefined',
			`real workbench boot requires CODE_TAURI_WORKBENCH_PATH=${defaultWorkbenchEntry}: undefined`,
		]
	);
}

function runWorkbenchSmoke() {
	withLayer('workbench', () => {
		const preflightFailures = collectRealWorkbenchPreflightFailures();
		for (const failure of preflightFailures) {
			check(failure, false);
		}
		if (preflightFailures.length === 0) {
			check('real workbench boot preflight', true, `${defaultWorkbenchEntry} selected and scaffold fallback is forbidden`);
			if (isDefaultWorkbenchPath(process.env.CODE_TAURI_WORKBENCH_PATH) && !existsSync(generatedWorkbenchPath)) {
				warn('real workbench generated file missing', `${generatedWorkbenchPath}; local headless smoke continues unless CODE_TAURI_STRICT_DEPS=1 or CI=true`);
			}
		}
		warn('real workbench GUI launch', 'skipped; deterministic preflight used in headless smoke harness');
	});
}

function cargoTest(name, testName, args = []) {
	return runBehaviorCheck(name, 'cargo', [
		'test',
		'--manifest-path',
		path.join(srcTauriRoot, 'Cargo.toml'),
		...args,
		testName,
		'--',
		'--exact',
		'--nocapture',
	]);
}

function runSourceSmoke() {
	withLayer('source', () => {
		const configText = readRequired('config');
		const manifestText = readRequired('manifest');
		const runtimeText = readRequired('runtime');
		const workbenchUrlResolverText = readRequired('workbenchUrlResolver');
		const commandsText = readRequired('commands');
		const fileServiceText = readRequired('fileService');
		const extensionSidecarText = readRequired('extensionSidecar');
		const observabilityText = readRequired('observability');
		const fallbackIndexText = readRequired('fallbackIndex');
		const packagingDocText = readRequired('packagingDoc');
		const parityGateScriptText = readRequired('parityGateScript');
		const apiTestsScriptText = readRequired('apiTestsScript');
		const packageScriptText = readRequired('packageScript');
		const builtinExtensionSmokeScriptText = readRequired('builtinExtensionSmokeScript');
		const parityGateManifestText = readRequired('parityGateManifest');
		const bridgeText = readRequired('bridge');

		let config;
		try {
			config = JSON.parse(configText);
			check('tauri config parses JSON', true);
		} catch (error) {
			check('tauri config parses JSON', false, error.message);
		}

		if (config) {
			check('tauri config productName', config.productName === 'VS Code Atomic', config.productName);
			check('tauri config identifier', config.identifier === 'dev.vscode.atomic', config.identifier);
			check('tauri config frontendDist', config.build?.frontendDist === 'www', config.build?.frontendDist);
			check('tauri config creates windows from runtime', Array.isArray(config.app?.windows) && config.app.windows.length === 0, JSON.stringify(config.app?.windows));
			check('tauri config bundle inactive for MVP', config.bundle?.active === false, String(config.bundle?.active));
			check('tauri config CSP present', typeof config.app?.security?.csp === 'string' && config.app.security.csp.trim().length > 0, String(config.app?.security?.csp));
			if (typeof config.app?.security?.csp === 'string') {
				checkRequiredCsp(config.app.security.csp);
			}
		}

		includesAll(manifestText, 'cargo manifest', [
			'name = "vscode-atomic-tauri"',
			'default = ["runtime", "terminal-pty"]',
			'runtime = ["auth-tunnels", "dep:tauri"]',
			'tauri = { version = "2"',
		]);

		includesAll(runtimeText, 'runtime boot path', [
			'CODE_TAURI_WORKBENCH_URL',
			'CODE_TAURI_WORKBENCH_PATH',
			'CODE_TAURI_REQUIRE_REAL_WORKBENCH',
			'out/vs/code/browser/workbench/workbench.html',
			'ResolvedWorkbenchUrl::App(path) => tauri::WebviewUrl::App(path.into())',
			'tauri::WebviewWindowBuilder::new(app, "main", resolve_workbench_url())',
			'register_file_service',
			'register_extension_host_service',
			'crate::commands::channel_call',
			...fileCommandNames.map(command => `crate::commands::${command}`),
		]);
		includesAll(workbenchUrlResolverText, 'workbench URL resolver', [
			'CODE_TAURI_WORKBENCH_URL',
			'CODE_TAURI_WORKBENCH_PATH',
			'CODE_TAURI_REQUIRE_REAL_WORKBENCH',
			'out/vs/code/browser/workbench/workbench.html',
			'refusing developer scaffold fallback {FALLBACK_APP_ENTRY_DISPLAY}',
			'ResolvedWorkbenchUrl::App(FALLBACK_APP_ENTRY)',
		]);
		checkWorkbenchPathResolverGuard(workbenchUrlResolverText);
		checkHandshakeTokenSource(extensionSidecarText);

		compareCommandContracts(
			extractTauriBridgeCommandNames(bridgeText),
			extractRustGenerateHandlerCommandNames(runtimeText)
		);

		check('release scaffold fallback disabled by inactive bundle config', config?.bundle?.active === false, String(config?.bundle?.active));
		includesAll(runtimeText, 'dev-only workbench override policy', [
			'CODE_TAURI_WORKBENCH_URL may point at a dev server or hosted workbench.',
			'CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 rejects the developer-only scaffold for release validation.',
			'src-tauri/www/index.html remains the developer-only fallback scaffold until bundle integration lands.',
		]);
		includesAll(fallbackIndexText, 'scaffold fallback marker', [
			'VS Code Atomic Tauri developer-only runtime placeholder',
		]);
		includesAll(packagingDocText, 'tauri parity release gates', [
			'`src-tauri/www/index.html` is developer-only.',
			'`Scaffold -> RuntimeBuildGreen -> RealWorkbenchBootGreen -> CoreServicesParityGreen -> ExtensionApiParityGreen -> BuiltInExtensionParityGreen -> SignedPackageGreen -> ReleaseCandidate`',
			'Only `ReleaseCandidate` may be described as migration complete.',
			'real workbench boot, Rust runtime build, core service parity, extension API parity, built-in extension parity, signed package validation, and release validation',
		]);
		includesAll(parityGateScriptText, 'tauri parity gate script', [
			'pre-ReleaseCandidate docs/metadata do not claim migration complete',
			'release validation forbids scaffold fallback',
			'parity gate one gate per migration state',
			'parity gate gates exact order',
			'parity gate BuiltInExtensionParityGreen runs built-in extension smoke preflight',
			'release validation pass only at ReleaseCandidate',
			'future state remains failing',
		]);
		includesAll(packageScriptText, 'tauri package script', [
			'CODE_TAURI_PARITY_MANIFEST_PATH',
			'--release-validate',
			'Tauri package dry-run release invariants are not satisfied',
			'Tauri package release validation passed.',
			'Tauri package ${mode} validation failed',
		]);
		includesAll(builtinExtensionSmokeScriptText, 'tauri built-in extension smoke script', [
			'validateGit();',
			'validateTypeScript();',
			'validateDebug();',
			'validateTerminal();',
			'validateNotebooks();',
			'validateAuth();',
			'validateCopilotApi();',
		]);
		includesAll(packagingDocText, 'real workbench smoke validation', [
			'`npm run smoketest:tauri:workbench`',
			'`CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html npm run smoketest:tauri:workbench`',
			'Deterministic real workbench preflight rejects `src-tauri/www/index.html` as a release workbench path',
		]);
		includesAll(parityGateManifestText, 'tauri parity gate manifest', [
			'"currentState": "Scaffold"',
			'"state": "ReleaseCandidate"',
			'npm run test-extension:tauri-sidecar',
			'npm run smoketest:tauri:builtin-extensions',
			'"permitsScaffoldFallback": false',
			'"owner"',
			'"command"',
			'"lastCiResult"',
		]);
		includesAll(apiTestsScriptText, 'tauri vscode-api-tests sidecar harness', [
			'extensions/vscode-api-tests',
			'vscode-api-tests-folder',
			'vscode-api-tests-workspace',
			'--enable-proposed-api=vscode.vscode-api-tests',
			'CODE_TAURI_APP_PATH',
			'public vscode.d.ts hash unchanged',
			'VSCODE_ATOMIC_EXTENSION_HOST_TRANSPORT',
			'SIDECAR_TOKEN_ENV_NAME',
			'export interface CancelRequest',
		]);

		includesAll(commandsText, 'core commands', [
			'pub async fn channel_call',
			'pub async fn channel_listen',
			'pub async fn channel_dispose',
			'pub async fn cancel_request',
			...fileCommandNames.map(command => `pub async fn ${command}`),
		]);

		includesAll(fileServiceText, 'file service commands', [
			'"fs_stat" | "stat"',
			'"fs_read_file" | "readFile"',
			'"fs_write_file" | "writeFile"',
			'"fs_delete" | "delete"',
			'"fs_mkdir" | "mkdir"',
			'"fs_readdir" | "readdir"',
			'"fs_watch" | "watch"',
			'pub fn path_from_uri',
		]);

		includesAll(extensionSidecarText, 'extension sidecar state', [
			'pub enum ExtensionSidecarState',
			'NotStarted',
			'Starting',
			'Ready',
			'Crashed',
			'Restarting',
			'Stopped',
			'"startSidecar" | "start"',
			'"stopSidecar" | "stop"',
			'"status"',
			'SIDECAR_TOKEN_ENV_NAME',
			'VSCODE_ATOMIC_EXTENSION_HOST_TRANSPORT',
		]);
		includesAll(observabilityText, 'observability sidecar token', [
			'pub const SIDECAR_TOKEN_ENV_NAME: &str = "VSCODE_ATOMIC_EXTENSION_HOST_TOKEN"',
			'pub const REDACTED_SECRET: &str = "<redacted>"',
		]);

		check('fallback index non-empty', fallbackIndexText.trim().length > 0, requiredFiles.fallbackIndex);
		const parityGateManifest = JSON.parse(parityGateManifestText);
		runParityGatePolicyCases(parityGateManifest);
		runPackagePolicyCases(parityGateManifest);
		runRealWorkbenchPreflightCases();
	});
}

function getDependencyState() {
	const hasCargo = commandExists('cargo');
	const needsLinuxWebkitGtk = process.platform === 'linux';
	const hasPkgConfig = !needsLinuxWebkitGtk || commandExists('pkg-config');
	const hasWebkitGtk = !needsLinuxWebkitGtk || (hasPkgConfig && commandExists('pkg-config', ['--exists', 'webkit2gtk-4.1']));

	return { hasCargo, hasPkgConfig, hasWebkitGtk };
}

function runDependencySmoke() {
	const dependencies = getDependencyState();
	withLayer('deps', () => {
		dependencyCheck('system dependency cargo', dependencies.hasCargo, 'missing; Rust compile/package validation unavailable');
		dependencyCheck('system dependency node', commandExists('node'), 'missing; extension sidecar spawn validation unavailable');
	});
	withLayer('gui', () => {
		dependencyCheck('system dependency pkg-config', dependencies.hasPkgConfig, 'missing; webkit2gtk-4.1 detection unavailable');
		dependencyCheck('system dependency webkit2gtk-4.1', dependencies.hasWebkitGtk, 'missing or not visible to pkg-config; full Tauri GUI build/run may fail');
	});
	return dependencies;
}

function runRuntimeSmoke(dependencies = getDependencyState()) {
	if (!dependencies.hasCargo) {
		withLayer('pure-rust', () => {
			skipBehaviorCheck('behavior channel_event fanout', 'skipped; cargo missing');
			skipBehaviorCheck('behavior channel_dispose cancellation', 'skipped; cargo missing');
			skipBehaviorCheck('behavior file watch registration', 'skipped; cargo missing');
		});
		withLayer('runtime-rust', () => {
			skipBehaviorCheck('behavior subscription.missingId', 'skipped; cargo missing');
			skipBehaviorCheck('behavior channel_listen backend registration', 'skipped; cargo missing');
		});
		return;
	}

	withLayer('pure-rust', () => {
		cargoTest(
			'behavior channel_event fanout',
			'subscription_manager::tests::fan_out_emits_channel_event_messages',
			['--no-default-features']
		);
		cargoTest(
			'behavior channel_dispose cancellation',
			'subscription_manager::tests::dispose_is_idempotent_and_ignores_late_events',
			['--no-default-features']
		);
		cargoTest(
			'behavior file watch registration',
			'file_service::tests::listen_watch_sends_payload_and_stops_after_dispose',
			['--no-default-features']
		);
	});

	withLayer('gui', () => {
		dependencyCheck('system dependency webkit2gtk-4.1', dependencies.hasWebkitGtk, 'missing or not visible to pkg-config; full Tauri GUI build/run may fail');
	});

	withLayer('runtime-rust', () => {
		if (dependencies.hasWebkitGtk) {
			cargoTest(
				'behavior release workbench rejects scaffold fallback',
				'runtime::tests::rejects_scaffold_fallback_when_real_workbench_required'
			);
			cargoTest(
				'behavior subscription.missingId',
				'commands::tests::channel_listen_requires_subscription_id'
			);
			cargoTest(
				'behavior channel_listen backend registration',
				'commands::tests::channel_listen_registers_backend_before_returning_success'
			);
		} else {
			skipBehaviorCheck('behavior release workbench rejects scaffold fallback', 'skipped; webkit2gtk-4.1 missing or not visible to pkg-config');
			skipBehaviorCheck('behavior subscription.missingId', 'skipped; webkit2gtk-4.1 missing or not visible to pkg-config');
			skipBehaviorCheck('behavior channel_listen backend registration', 'skipped; webkit2gtk-4.1 missing or not visible to pkg-config');
		}
	});
}

if (shouldRun('source')) {
	runSourceSmoke();
}

let dependencies;
if (shouldRun('deps')) {
	dependencies = runDependencySmoke();
}

if (shouldRun('runtime')) {
	runRuntimeSmoke(dependencies);
}

if (shouldRun('workbench')) {
	runWorkbenchSmoke();
}

if (shouldRun('source')) {
	withLayer('source', () => {
		runBehaviorCheck('behavior parity gate manifest policy', 'node', [path.join(repoRoot, 'scripts', 'tauri-parity-gate.mjs')]);
		runBehaviorCheck('behavior built-in extension smoke preflight', 'node', [path.join(repoRoot, 'scripts', 'tauri-builtin-extension-smoke.mjs')]);
	});
}

console.log(`summary failures=${failures.length} warnings=${warnings.length}`);

if (failures.length) {
	process.exit(1);
}
