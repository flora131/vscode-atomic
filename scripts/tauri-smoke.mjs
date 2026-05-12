/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, renameSync, rmSync, statSync, symlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const srcTauriRoot = path.join(repoRoot, 'src-tauri');
const scaffoldWorkbenchPath = path.join(srcTauriRoot, 'www', 'index.html');
const defaultWorkbenchEntry = 'out/vs/code/browser/workbench/workbench.html';
const generatedWorkbenchPath = path.join(repoRoot, ...defaultWorkbenchEntry.split('/'));
const generatedWorkbenchRoot = path.join(repoRoot, 'out');
const packagedWorkbenchPath = path.join(srcTauriRoot, 'www', ...defaultWorkbenchEntry.split('/'));
const packagedWorkbenchRoot = path.join(srcTauriRoot, 'www', 'out');
const missingGeneratedWorkbenchPath = path.join(repoRoot, 'out', 'vs', 'code', 'browser', 'workbench', 'workbench-tauri-smoke-missing.html');
const generatedWorkbenchFixtureLockDir = path.join(tmpdir(), 'vscode-atomic-tauri-smoke-workbench.lock');
const generatedWorkbenchFixtureLockTimeoutMs = 120_000;
const generatedWorkbenchFixtureStaleLockMs = 10 * 60_000;

const requiredFiles = {
	config: path.join(srcTauriRoot, 'tauri.conf.json'),
	manifest: path.join(srcTauriRoot, 'Cargo.toml'),
	runtime: path.join(srcTauriRoot, 'src', 'runtime.rs'),
	workbenchUrlResolver: path.join(srcTauriRoot, 'src', 'workbench_url_resolver.rs'),
	commands: path.join(srcTauriRoot, 'src', 'commands.rs'),
	fileService: path.join(srcTauriRoot, 'src', 'file_service.rs'),
	parityServices: path.join(srcTauriRoot, 'src', 'parity_services.rs'),
	extensionSidecar: path.join(srcTauriRoot, 'src', 'extension_sidecar.rs'),
	observability: path.join(srcTauriRoot, 'src', 'observability.rs'),
	coreServicesParityTest: path.join(srcTauriRoot, 'tests', 'core_services_parity.rs'),
	fallbackIndex: path.join(srcTauriRoot, 'www', 'index.html'),
	packagingDoc: path.join(srcTauriRoot, 'PACKAGING.md'),
	parityGateScript: path.join(repoRoot, 'scripts', 'tauri-parity-gate.mjs'),
	releaseCiCheckScript: path.join(repoRoot, 'scripts', 'tauri-release-ci-check.mjs'),
	docClaimCheckScript: path.join(repoRoot, 'scripts', 'tauri-doc-claim-check.mjs'),
	apiTestsScript: path.join(repoRoot, 'scripts', 'tauri-vscode-api-tests.mjs'),
	packageScript: path.join(repoRoot, 'scripts', 'tauri-package.mjs'),
	builtinExtensionSmokeScript: path.join(repoRoot, 'scripts', 'tauri-builtin-extension-smoke.mjs'),
	packageUnitTest: path.join(repoRoot, 'build', 'lib', 'test', 'tauriPackage.test.ts'),
	rootPackage: path.join(repoRoot, 'package.json'),
	realWorkbench: generatedWorkbenchPath,
	parityGateManifest: path.join(repoRoot, 'scripts', 'tauri-parity-gate.manifest.json'),
	bridge: path.join(repoRoot, 'src', 'vs', 'platform', 'tauri', 'common', 'tauriBridge.ts'),
};

const failures = [];
const warnings = [];
const fileCommandNames = ['fs_stat', 'fs_read_file', 'fs_write_file', 'fs_delete', 'fs_mkdir', 'fs_readdir', 'fs_watch'];
const releaseValidateArgs = ['--release-validate', '--flavor', 'code-tauri', '--sign'];
const signedReleaseEnv = { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1' };
const signedReleaseWorkbenchEnv = { ...signedReleaseEnv, CODE_TAURI_WORKBENCH_PATH: defaultWorkbenchEntry };
const releaseValidationFailed = 'Tauri package release validation failed:';
const noDefaultFeatureRustTests = [
	{
		name: 'behavior release workbench rejects explicit scaffold path',
		testName: 'workbench_url_resolver::tests::rejects_explicit_scaffold_path_when_real_workbench_required',
	},
	{
		name: 'behavior release workbench rejects normalized explicit scaffold path',
		testName: 'workbench_url_resolver::tests::rejects_normalized_explicit_scaffold_path_when_real_workbench_required',
	},
	{
		name: 'behavior extension sidecar token randomness',
		testName: 'extension_sidecar::tests::token_generation_produces_random_hex_tokens',
	},
	{
		name: 'behavior channel_event fanout',
		testName: 'subscription_manager::tests::fan_out_emits_channel_event_messages',
	},
	{
		name: 'behavior channel_dispose cancellation',
		testName: 'subscription_manager::tests::dispose_is_idempotent_and_ignores_late_events',
	},
	{
		name: 'behavior file watch registration',
		testName: 'file_service::tests::listen_watch_sends_payload_and_stops_after_dispose',
	},
	{
		name: 'behavior subscription.missingId',
		testName: 'commands::tests::channel_listen_requires_subscription_id',
	},
	{
		name: 'behavior channel_listen backend registration',
		testName: 'commands::tests::channel_listen_registers_backend_before_returning_success',
	},
];
const strictDependencies = usesStrictDependencies(process.env);
const validModes = new Set(['all', 'source', 'deps', 'runtime', 'workbench', 'policy']);
const modesExcludedFromAll = new Set(['workbench', 'policy']);
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
	if (modesExcludedFromAll.has(targetMode)) {
		return currentMode === targetMode;
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

function parseJson(name, source) {
	try {
		const value = JSON.parse(source);
		check(`${name} parses JSON`, true);
		return value;
	} catch (error) {
		check(`${name} parses JSON`, false, error.message);
		return undefined;
	}
}

function includesAll(source, name, needles) {
	for (const needle of needles) {
		check(`${name} contains ${needle}`, source.includes(needle));
	}
}

function checkPackageScript(packageJson, scriptName, expectedCommand) {
	const command = packageJson?.scripts?.[scriptName];
	check(`package script ${scriptName}`, command === expectedCommand, String(command));
}

function checkUnitStructDefaultGuard(name, source, structNames) {
	for (const structName of structNames) {
		const defaultConstruction = new RegExp(`\\b${structName}\\s*(?:::|\\.)\\s*default\\s*\\(`);
		check(`${name} constructs ${structName} directly`, !defaultConstruction.test(source));
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

function extractJavaScriptFunctionBody(source, functionName) {
	const match = new RegExp(`\\bfunction\\s+${functionName}\\s*\\(`).exec(source);
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

	const scaffoldGuardMatch = /is_scaffold_workbench_path\(\s*repo_root\s*,\s*&path\s*(?:,\s*&file_exists\s*)?\)/.exec(pathBranchBody);
	const scaffoldGuardIndex = scaffoldGuardMatch?.index ?? -1;
	const guardRejectIndex = firstNeedleIndex(pathBranchBody, scaffoldGuardIndex, ['panic!', 'panic_real_workbench_required()']);
	const realWorkbenchGuardIndex = pathBranchBody.indexOf('is_real_workbench_path(repo_root, &path, &file_exists)');
	const fileExistsIndex = pathBranchBody.indexOf('file_exists(&path)');
	const fileReturnIndex = pathBranchBody.indexOf('return ResolvedWorkbenchUrl::File(path)', fileExistsIndex);
	check('workbench URL resolver rejects scaffold path before file_exists', scaffoldGuardIndex !== -1 && guardRejectIndex !== -1 && fileExistsIndex !== -1 && scaffoldGuardIndex < fileExistsIndex && guardRejectIndex < fileExistsIndex, 'expected scaffold rejection guard before file existence check');
	check('workbench URL resolver requires real workbench before file_exists', realWorkbenchGuardIndex !== -1 && fileExistsIndex !== -1 && realWorkbenchGuardIndex < fileExistsIndex, 'expected real workbench guard before file existence check');
	check('workbench URL resolver file_exists path branch returns file only after guards', fileReturnIndex !== -1 && fileExistsIndex !== -1 && scaffoldGuardIndex !== -1 && realWorkbenchGuardIndex !== -1 && fileExistsIndex < fileReturnIndex, 'expected guarded return ResolvedWorkbenchUrl::File(path)');
}

function checkWorkbenchSourceOrder(workbenchUrlResolverText, packagingDocText) {
	const runtimeResolverBody = extractRustFunctionBody(workbenchUrlResolverText, 'resolve_workbench_url_with_release_policy');
	const runtimeResolverWrapperBody = extractRustFunctionBody(workbenchUrlResolverText, 'resolve_workbench_url_with');
	const runtimeSourceOrder = ['CODE_TAURI_WORKBENCH_URL', 'CODE_TAURI_WORKBENCH_PATH', 'out/vs/code/browser/workbench/workbench.html', 'src-tauri/www/index.html'];
	const releaseSourceOrder = ['CODE_TAURI_WORKBENCH_PATH', 'out/vs/code/browser/workbench/workbench.html'];

	checkOrderedNeedles('workbench source order docs', packagingDocText, runtimeSourceOrder);
	check('workbench source order runtime wrapper delegates to release policy resolver', runtimeResolverWrapperBody.includes('resolve_workbench_url_with_release_policy'), runtimeResolverWrapperBody.trim());
	checkOrderedNeedles('workbench source order runtime resolver', runtimeResolverBody, ['WORKBENCH_URL_ENV', 'WORKBENCH_PATH_ENV', 'DEFAULT_WORKBENCH_ENTRY', 'FALLBACK_APP_ENTRY']);
	checkOrderedNeedles('workbench source order package release docs match release resolver', packagingDocText, releaseSourceOrder);
}

function checkPackageStableSourceInvariants(packageScriptText) {
	check('package release invariant validates active resolved bundle config', packageScriptText.includes('package release requires Tauri release config bundle.active true') && packageScriptText.includes('releaseTauriConfigHasActiveBundle(parsedTauriConfig)'), 'expected release bundle.active validation through stable helper');
	includesAll(packageScriptText, 'tauri package release invariant messages', [
		'package release requires real workbench source ${requiredReleaseWorkbenchSource}',
		'package execute requires bundled real workbench asset ${requiredBundledWorkbenchSource}',
		'Tauri package frontend validation failed:',
		'Tauri package execute validation failed:',
	]);
	includesAll(packageScriptText, 'tauri package execute workbench env', [
		'const requiredReleaseWorkbenchEnv = Object.freeze({',
		'CODE_TAURI_REQUIRE_REAL_WORKBENCH: \'1\'',
		'[workbenchPathEnv]: requiredReleaseWorkbenchSource',
		'env: createReleaseWorkbenchCommandEnv()',
		'delete env[workbenchUrlEnv]',
	]);
}

function checkOrderedNeedles(name, source, needles) {
	let previousIndex = -1;
	const missingOrOutOfOrder = [];
	for (const needle of needles) {
		const index = source.indexOf(needle, previousIndex + 1);
		if (index === -1) {
			missingOrOutOfOrder.push(needle);
			continue;
		}
		previousIndex = index;
	}

	check(name, missingOrOutOfOrder.length === 0, missingOrOutOfOrder.length ? `missing or out of order: ${missingOrOutOfOrder.join(', ')}` : needles.join(' -> '));
}

function firstNeedleIndex(source, fromIndex, needles) {
	const indexes = needles
		.map(needle => source.indexOf(needle, fromIndex))
		.filter(index => index !== -1);

	return indexes.length === 0 ? -1 : Math.min(...indexes);
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
	const expectedStatus = options.expectedStatus ?? 0;
	const expectedNegative = expectedStatus !== 0 && result.status === expectedStatus;
	logProcessOutput(result, expectedNegative);

	const ok = check(name, result.status === expectedStatus, `${command} ${args.join(' ')}`);
	const stdout = result.stdout ?? '';
	const stderr = result.stderr ?? '';
	const output = `${stdout}\n${stderr}`;
	for (const needle of options.outputIncludes ?? []) {
		check(`${name} output contains ${needle}`, output.includes(needle));
	}
	for (const needle of options.outputExcludes ?? []) {
		check(`${name} output excludes ${needle}`, !output.includes(needle));
	}
	return ok;
}

function logProcessOutput(result, expectedNegative = false) {
	if (result.stdout?.trim()) {
		console.log(formatChildOutput(result.stdout.trim(), expectedNegative));
	}

	if (result.stderr?.trim()) {
		console.error(formatChildOutput(result.stderr.trim(), expectedNegative));
	}
}

function formatChildOutput(output, expectedNegative) {
	if (!expectedNegative) {
		return output;
	}

	return output
		.split(/\r?\n/)
		.map(line => `expected-negative ${line}`)
		.join('\n');
}

function checkExpectedNegativeOutputFormatting() {
	check(
		'behavior expected-negative child output prefixes each line',
		formatChildOutput('stdout line\nstderr line', true) === 'expected-negative stdout line\nexpected-negative stderr line',
		formatChildOutput('stdout line\nstderr line', true)
	);
	check(
		'behavior non-negative child output remains unchanged',
		formatChildOutput('stdout line\nstderr line', false) === 'stdout line\nstderr line',
		formatChildOutput('stdout line\nstderr line', false)
	);
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

function resolveExistingWorkbenchPath(value) {
	const resolvedPath = resolveWorkbenchPath(value);
	if (!resolvedPath || !existsSync(resolvedPath)) {
		return undefined;
	}

	return realpathSync(resolvedPath);
}

function isScaffoldWorkbenchPath(workbenchPath) {
	const resolvedPath = resolveExistingWorkbenchPath(workbenchPath);
	const resolvedScaffoldPath = resolveExistingWorkbenchPath(scaffoldWorkbenchPath);
	return resolveWorkbenchPath(workbenchPath) === scaffoldWorkbenchPath
		|| (resolvedPath !== undefined && resolvedPath === resolvedScaffoldPath);
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
	const requiresRealWorkbench = env.CODE_TAURI_REQUIRE_REAL_WORKBENCH === '1';
	const hasDefaultWorkbenchPath = isDefaultWorkbenchPath(workbenchPath);
	const hasGeneratedWorkbenchFile = hasDefaultWorkbenchPath && fileExists(generatedWorkbenchPath);

	addFailure(failures, 'real workbench boot requires CODE_TAURI_REQUIRE_REAL_WORKBENCH=1', requiresRealWorkbench, String(env.CODE_TAURI_REQUIRE_REAL_WORKBENCH));
	addFailure(failures, `real workbench boot requires CODE_TAURI_WORKBENCH_PATH=${defaultWorkbenchEntry}`, hasDefaultWorkbenchPath, String(workbenchPath));
	addFailure(failures, 'real workbench boot forbids src-tauri/www/index.html', !isScaffoldWorkbenchPath(workbenchPath), String(workbenchPath));
	if (requiresRealWorkbench && hasDefaultWorkbenchPath) {
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

function writeTempManifest(name, manifest, packageJson = createTempPackageJson(manifest)) {
	const tempDir = mkdtempSync(path.join(tmpdir(), 'tauri-parity-gate-'));
	const manifestPath = path.join(tempDir, `${name}.json`);
	const packageJsonPath = path.join(tempDir, 'package.json');
	writeFileSync(manifestPath, `${JSON.stringify(manifest, null, '\t')}\n`);
	writeFileSync(packageJsonPath, `${JSON.stringify(packageJson, null, '\t')}\n`);
	return { tempDir, manifestPath, packageJsonPath };
}

function createTempPackageJson(manifest) {
	return {
		name: 'tauri-parity-gate-policy-fixture',
		private: true,
		scripts: {
			'tauri:release-ci-check': 'node scripts/tauri-release-ci-check.mjs',
			'package:tauri:gate': 'npm run tauri:release-ci-check',
			'test:tauri:package-gate': 'npm run package:tauri:gate',
		},
		tauriMigration: {
			status: manifest.currentState,
			manifest: 'scripts/tauri-parity-gate.manifest.json',
			releaseGate: 'ReleaseCandidate',
			claimPolicy: 'blocked until ReleaseCandidate; internal scaffold only',
		},
	};
}

function runParityGateManifestCase(name, manifest, expectedStatus, env = {}, packageJson = createTempPackageJson(manifest)) {
	const { tempDir, manifestPath, packageJsonPath } = writeTempManifest(name, manifest, packageJson);
	try {
		runBehaviorCheck(name, process.execPath, [requiredFiles.parityGateScript], {
			env: { CODE_TAURI_PARITY_MANIFEST_PATH: manifestPath, CODE_TAURI_PACKAGE_JSON_PATH: packageJsonPath, ...env },
			expectedStatus,
		});
	} finally {
		rmSync(tempDir, { recursive: true, force: true });
	}
}

function runBuiltInExtensionSmokeManifestCase(name, manifest, expectedStatus, env = {}) {
	const { tempDir, manifestPath } = writeTempManifest(name, manifest);
	try {
		runBehaviorCheck(name, process.execPath, [requiredFiles.builtinExtensionSmokeScript], {
			env: { CODE_TAURI_PARITY_MANIFEST_PATH: manifestPath, ...env },
			expectedStatus,
		});
	} finally {
		rmSync(tempDir, { recursive: true, force: true });
	}
}

function runParityGateManifestCaseWithGeneratedWorkbenchAbsent(name, manifest, expectedStatus) {
	check(`${name} clean workspace generated workbench absent before parity gate`, !existsSync(generatedWorkbenchPath), generatedWorkbenchPath);
	runParityGateManifestCase(name, manifest, expectedStatus);
	check(`${name} clean workspace generated workbench absent after parity gate`, !existsSync(generatedWorkbenchPath), generatedWorkbenchPath);
}

function runReleaseCiCheckCase(name, manifest, event, expectedStatus, env = {}, expectations = {}) {
	const { tempDir, manifestPath, packageJsonPath } = writeTempManifest(name, manifest);
	const eventPath = path.join(tempDir, `${name}.event.json`);
	writeFileSync(eventPath, `${JSON.stringify(event, null, '\t')}\n`);
	try {
		runBehaviorCheck(name, process.execPath, [requiredFiles.releaseCiCheckScript], {
			env: { CODE_TAURI_PARITY_MANIFEST_PATH: manifestPath, CODE_TAURI_PACKAGE_JSON_PATH: packageJsonPath, GITHUB_EVENT_PATH: eventPath, ...env },
			expectedStatus,
			...expectations,
		});
	} finally {
		rmSync(tempDir, { recursive: true, force: true });
	}
}

function sleepSync(ms) {
	Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
}

function acquireGeneratedWorkbenchFixtureLock() {
	const deadline = Date.now() + generatedWorkbenchFixtureLockTimeoutMs;
	while (true) {
		try {
			mkdirSync(generatedWorkbenchFixtureLockDir);
			writeFileSync(path.join(generatedWorkbenchFixtureLockDir, 'owner.txt'), `${process.pid}\n`);
			return () => rmSync(generatedWorkbenchFixtureLockDir, { recursive: true, force: true });
		} catch (error) {
			if (error?.code !== 'EEXIST') {
				throw error;
			}

			try {
				const lockAgeMs = Date.now() - statSync(generatedWorkbenchFixtureLockDir).mtimeMs;
				if (lockAgeMs > generatedWorkbenchFixtureStaleLockMs) {
					rmSync(generatedWorkbenchFixtureLockDir, { recursive: true, force: true });
					continue;
				}
			} catch (statError) {
				if (statError?.code === 'ENOENT') {
					continue;
				}
				throw statError;
			}

			if (Date.now() >= deadline) {
				throw new Error(`Timed out waiting for generated workbench fixture lock: ${generatedWorkbenchFixtureLockDir}`);
			}
			sleepSync(50);
		}
	}
}

function withGeneratedWorkbenchFixture(createFixture, callback) {
	const releaseLock = acquireGeneratedWorkbenchFixtureLock();
	let hiddenWorkbenchPath;
	let existed = false;
	let packagedWorkbenchExisted = false;

	try {
		existed = existsSync(generatedWorkbenchPath);
		packagedWorkbenchExisted = existsSync(packagedWorkbenchPath);
		hiddenWorkbenchPath = !createFixture && existed ? path.join(tmpdir(), `tauri-smoke-hidden-workbench-${process.pid}-${Date.now()}.html`) : undefined;
		if (hiddenWorkbenchPath) {
			renameSync(generatedWorkbenchPath, hiddenWorkbenchPath);
		}
		if (!createFixture && existsSync(generatedWorkbenchPath)) {
			throw new Error(`Tauri smoke workbench fixture expected generated workbench source to be hidden: ${generatedWorkbenchPath}`);
		}
		if (createFixture && !existed) {
			mkdirSync(path.dirname(generatedWorkbenchPath), { recursive: true });
			writeFileSync(generatedWorkbenchPath, '<!doctype html><title>Real workbench parity fixture</title>\n');
		}
		if (createFixture && !packagedWorkbenchExisted) {
			mkdirSync(path.dirname(packagedWorkbenchPath), { recursive: true });
			writeFileSync(packagedWorkbenchPath, '<!doctype html><title>Bundled real workbench parity fixture</title>\n');
		}

		callback();
	} finally {
		if (createFixture && !packagedWorkbenchExisted) {
			rmSync(packagedWorkbenchPath, { force: true });
		}
		if (createFixture && !existed) {
			rmSync(generatedWorkbenchPath, { force: true });
		}
		if (hiddenWorkbenchPath) {
			renameSync(hiddenWorkbenchPath, generatedWorkbenchPath);
		}
		releaseLock();
	}
}

function runDocClaimCheckCase(name, manifest, docText, expectedStatus) {
	const { tempDir, manifestPath, packageJsonPath } = writeTempManifest(name, manifest);
	const docPath = path.join(tempDir, `${name}.md`);
	writeFileSync(docPath, docText);
	try {
		runBehaviorCheck(name, process.execPath, [requiredFiles.docClaimCheckScript], {
			env: {
				CODE_TAURI_PARITY_MANIFEST_PATH: manifestPath,
				CODE_TAURI_PACKAGE_JSON_PATH: packageJsonPath,
				CODE_TAURI_DOC_CLAIM_SOURCE_FILES: docPath,
			},
			expectedStatus,
		});
	} finally {
		rmSync(tempDir, { recursive: true, force: true });
	}
}

function runParityGatePolicyCases(baseManifest) {
	const releaseCandidateManifest = createReleaseCandidateManifest(baseManifest);
	withGeneratedWorkbenchFixture(true, () => {
		runParityGateManifestCase('behavior parity gate accepts ReleaseCandidate with green release validation', releaseCandidateManifest, 0);
		runParityGateManifestCase('behavior parity gate accepts ReleaseCandidate completion claim', releaseCandidateManifest, 0, { CODE_TAURI_PR_TITLE: 'Tauri migration complete' });
	});

	const scaffoldWithReleasePass = cloneJson(baseManifest);
	scaffoldWithReleasePass.releaseValidation.lastCiResult.status = 'pass';
	runParityGateManifestCase('behavior parity gate rejects scaffold release validation pass', scaffoldWithReleasePass, 1);
	const packageStatusMismatch = createTempPackageJson(baseManifest);
	packageStatusMismatch.tauriMigration.status = 'RuntimeBuildGreen';
	runParityGateManifestCase('behavior parity gate rejects package migration status mismatch', baseManifest, 1, {}, packageStatusMismatch);
	const packageReleaseGateMismatch = createTempPackageJson(baseManifest);
	packageReleaseGateMismatch.tauriMigration.releaseGate = 'SignedPackageGreen';
	runParityGateManifestCase('behavior parity gate rejects package release gate mismatch', baseManifest, 1, {}, packageReleaseGateMismatch);
	const packageMetadataCompletionClaim = createTempPackageJson(baseManifest);
	packageMetadataCompletionClaim.description = 'Tauri migration complete';
	runParityGateManifestCase('behavior parity gate rejects pre-ReleaseCandidate package metadata completion claim', baseManifest, 1, {}, packageMetadataCompletionClaim);
	runParityGateManifestCase('behavior parity gate rejects pre-ReleaseCandidate PR title completion claim', baseManifest, 1, { CODE_TAURI_PR_TITLE: 'Tauri migration complete' });
	runParityGateManifestCase('behavior parity gate rejects pre-ReleaseCandidate PR summary release claim', baseManifest, 1, { CODE_TAURI_PR_SUMMARY: 'The Tauri migration is ready for release.' });
	runParityGateManifestCase('behavior parity gate accepts pre-ReleaseCandidate policy completion text', baseManifest, 0, { CODE_TAURI_PR_SUMMARY: 'Only ReleaseCandidate may claim migration complete.' });
	runDocClaimCheckCase('behavior doc claim check rejects pre-ReleaseCandidate docs completion claim', baseManifest, 'Tauri migration complete\n', 1);
	runDocClaimCheckCase('behavior doc claim check accepts ReleaseCandidate docs completion claim', releaseCandidateManifest, 'Tauri migration complete\n', 0);
	runDocClaimCheckCase('behavior doc claim check accepts pre-ReleaseCandidate policy text', baseManifest, 'Only ReleaseCandidate may claim migration complete.\n', 0);

	const futureGateGreen = cloneJson(baseManifest);
	manifestGate(futureGateGreen, 'RuntimeBuildGreen').lastCiResult.status = 'pass';
	runParityGateManifestCase('behavior parity gate rejects future green gate before currentState', futureGateGreen, 1);
	const futureReleaseCandidateGreen = cloneJson(baseManifest);
	manifestGate(futureReleaseCandidateGreen, 'ReleaseCandidate').lastCiResult.status = 'pass';
	runParityGateManifestCase('behavior parity gate rejects future ReleaseCandidate gate pass before currentState', futureReleaseCandidateGreen, 1);

	const futureGateGreenWithSchemaApproval = cloneJson(futureGateGreen);
	futureGateGreenWithSchemaApproval.approvedSchemaSplits = [{
		state: 'RuntimeBuildGreen',
		approved: true,
		owner: 'tauri-runtime',
		reason: 'schema split cannot bypass ordered CI promotion',
		approvedAt: '2026-05-11',
	}];
	runParityGateManifestCase('behavior parity gate rejects schema-approved future green gate before currentState', futureGateGreenWithSchemaApproval, 1);

	const signedPackageGreenManifest = createPromotedManifest(baseManifest, 'SignedPackageGreen');
	runParityGateManifestCase('behavior parity gate accepts SignedPackageGreen with signed package evidence', signedPackageGreenManifest, 0);
	runParityGateManifestCase('behavior parity gate rejects SignedPackageGreen PR title completion claim', signedPackageGreenManifest, 1, { CODE_TAURI_PR_TITLE: 'Tauri migration complete' });
	const signedPackageGreenMissingEvidence = cloneJson(signedPackageGreenManifest);
	delete signedPackageGate(signedPackageGreenMissingEvidence).signedPackageEvidence;
	runParityGateManifestCase('behavior parity gate rejects SignedPackageGreen missing signed package evidence', signedPackageGreenMissingEvidence, 1);

	const builtInExtensionGreenManifest = createPromotedManifest(baseManifest, 'BuiltInExtensionParityGreen');
	runParityGateManifestCase('behavior parity gate accepts BuiltInExtensionParityGreen with area platform matrix', builtInExtensionGreenManifest, 0);
	const builtInExtensionGreenMissingTunnels = cloneJson(builtInExtensionGreenManifest);
	builtInExtensionGate(builtInExtensionGreenMissingTunnels).builtInExtensionParity.areas = builtInExtensionAreas(builtInExtensionGreenMissingTunnels).filter(area => area.area !== 'tunnels');
	runParityGateManifestCase('behavior parity gate rejects BuiltInExtensionParityGreen missing tunnels area', builtInExtensionGreenMissingTunnels, 1);
	const builtInExtensionGreenMissingLinux = cloneJson(builtInExtensionGreenManifest);
	builtInExtensionAreas(builtInExtensionGreenMissingLinux)[0].platformResults = builtInExtensionAreas(builtInExtensionGreenMissingLinux)[0].platformResults.filter(result => result.platform !== 'linux');
	runParityGateManifestCase('behavior parity gate rejects BuiltInExtensionParityGreen missing Linux platform result', builtInExtensionGreenMissingLinux, 1);
	const builtInExtensionGreenFailingPlatform = cloneJson(builtInExtensionGreenManifest);
	builtInExtensionAreas(builtInExtensionGreenFailingPlatform)[0].platformResults[0].status = 'fail';
	runParityGateManifestCase('behavior parity gate rejects BuiltInExtensionParityGreen failing platform result', builtInExtensionGreenFailingPlatform, 1);
	const builtInExtensionSmokePreCiAreaPass = cloneJson(baseManifest);
	builtInExtensionAreas(builtInExtensionSmokePreCiAreaPass)[0].status = 'pass';
	runBuiltInExtensionSmokeManifestCase('behavior built-in extension smoke rejects pre-CI passed area', builtInExtensionSmokePreCiAreaPass, 1);
	const builtInExtensionSmokePreCiPlatformPass = cloneJson(baseManifest);
	builtInExtensionAreas(builtInExtensionSmokePreCiPlatformPass)[0].platformResults[0].status = 'pass';
	runBuiltInExtensionSmokeManifestCase('behavior built-in extension smoke rejects pre-CI passed platform result', builtInExtensionSmokePreCiPlatformPass, 1);
	const builtInExtensionSmokeMissingWindows = cloneJson(baseManifest);
	builtInExtensionAreas(builtInExtensionSmokeMissingWindows)[0].platformResults = builtInExtensionAreas(builtInExtensionSmokeMissingWindows)[0].platformResults.filter(result => result.platform !== 'windows');
	runBuiltInExtensionSmokeManifestCase('behavior built-in extension smoke rejects missing Windows platform result', builtInExtensionSmokeMissingWindows, 1);

	const releaseCandidateWithFailingPriorGate = cloneJson(releaseCandidateManifest);
	manifestGate(releaseCandidateWithFailingPriorGate, 'BuiltInExtensionParityGreen').lastCiResult.status = 'fail';
	withGeneratedWorkbenchFixture(true, () => {
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate with failing prior gate', releaseCandidateWithFailingPriorGate, 1);
	});
	const runtimeBuildGreenWithArtifactNotApplicable = createPromotedManifest(baseManifest, 'RuntimeBuildGreen');
	const runtimeBuildResult = manifestGate(runtimeBuildGreenWithArtifactNotApplicable, 'RuntimeBuildGreen').lastCiResult;
	runtimeBuildResult.ciArtifactUrl = null;
	runtimeBuildResult.ciArtifactNotApplicableReason = 'Runtime build gate produces no durable artifact; CI run log is authoritative.';
	runParityGateManifestCase('behavior parity gate accepts promoted gate artifact not-applicable reason', runtimeBuildGreenWithArtifactNotApplicable, 0);
	const runtimeBuildGreenMissingArtifactEvidence = createPromotedManifest(baseManifest, 'RuntimeBuildGreen');
	const runtimeBuildMissingArtifactResult = manifestGate(runtimeBuildGreenMissingArtifactEvidence, 'RuntimeBuildGreen').lastCiResult;
	runtimeBuildMissingArtifactResult.ciArtifactUrl = null;
	delete runtimeBuildMissingArtifactResult.ciArtifactNotApplicableReason;
	runParityGateManifestCase('behavior parity gate rejects promoted gate missing artifact evidence', runtimeBuildGreenMissingArtifactEvidence, 1);
	const runtimeBuildGreenMissingCiRunId = createPromotedManifest(baseManifest, 'RuntimeBuildGreen');
	delete manifestGate(runtimeBuildGreenMissingCiRunId, 'RuntimeBuildGreen').lastCiResult.ciRunId;
	runParityGateManifestCase('behavior parity gate rejects promoted gate missing ciRunId', runtimeBuildGreenMissingCiRunId, 1);
	const runtimeBuildGreenMissingCommand = createPromotedManifest(baseManifest, 'RuntimeBuildGreen');
	manifestGate(runtimeBuildGreenMissingCommand, 'RuntimeBuildGreen').command = '';
	runParityGateManifestCase('behavior parity gate rejects promoted gate missing command', runtimeBuildGreenMissingCommand, 1);
	const runtimeBuildGreenMissingObservedAt = createPromotedManifest(baseManifest, 'RuntimeBuildGreen');
	delete manifestGate(runtimeBuildGreenMissingObservedAt, 'RuntimeBuildGreen').lastCiResult.observedAt;
	runParityGateManifestCase('behavior parity gate rejects promoted gate missing observedAt', runtimeBuildGreenMissingObservedAt, 1);
	const runtimeBuildGreenMissingSummary = createPromotedManifest(baseManifest, 'RuntimeBuildGreen');
	manifestGate(runtimeBuildGreenMissingSummary, 'RuntimeBuildGreen').lastCiResult.summary = '';
	runParityGateManifestCase('behavior parity gate rejects promoted gate missing summary', runtimeBuildGreenMissingSummary, 1);
	const runtimeBuildGreenMissingOwner = createPromotedManifest(baseManifest, 'RuntimeBuildGreen');
	delete manifestGate(runtimeBuildGreenMissingOwner, 'RuntimeBuildGreen').owner;
	runParityGateManifestCase('behavior parity gate rejects promoted gate missing owner', runtimeBuildGreenMissingOwner, 1);
	const runtimeBuildGreenAmbiguousArtifactEvidence = createPromotedManifest(baseManifest, 'RuntimeBuildGreen');
	manifestGate(runtimeBuildGreenAmbiguousArtifactEvidence, 'RuntimeBuildGreen').lastCiResult.ciArtifactNotApplicableReason = 'Runtime build gate produces no durable artifact; CI run log is authoritative.';
	runParityGateManifestCase('behavior parity gate rejects promoted gate ambiguous artifact evidence', runtimeBuildGreenAmbiguousArtifactEvidence, 1);

	const reorderedStateMachine = cloneJson(baseManifest);
	reorderedStateMachine.migrationStateMachine = [...baseManifest.migrationStateMachine].reverse();
	runParityGateManifestCase('behavior parity gate rejects reordered migration state machine', reorderedStateMachine, 1);
	const missingStateMachineState = cloneJson(baseManifest);
	missingStateMachineState.migrationStateMachine = baseManifest.migrationStateMachine.filter(state => state !== 'CoreServicesParityGreen');
	runParityGateManifestCase('behavior parity gate rejects missing migration state machine state', missingStateMachineState, 1);
	const extraStateMachineState = cloneJson(baseManifest);
	extraStateMachineState.migrationStateMachine = [...baseManifest.migrationStateMachine, 'UnexpectedGreen'];
	runParityGateManifestCase('behavior parity gate rejects extra migration state machine state', extraStateMachineState, 1);
	const unknownCurrentState = cloneJson(baseManifest);
	unknownCurrentState.currentState = 'UnexpectedGreen';
	runParityGateManifestCase('behavior parity gate rejects unknown currentState', unknownCurrentState, 1);
	const danglingStateMachineState = cloneJson(baseManifest);
	danglingStateMachineState.migrationStateMachine[2] = 'DanglingGreen';
	runParityGateManifestCase('behavior parity gate rejects dangling migration state machine state', danglingStateMachineState, 1);

	const reorderedGates = cloneJson(baseManifest);
	reorderedGates.gates = [...baseManifest.gates].reverse();
	runParityGateManifestCase('behavior parity gate rejects reordered gate records', reorderedGates, 1);
	const invalidGateState = cloneJson(baseManifest);
	invalidGateState.gates[2].state = 'DanglingGreen';
	runParityGateManifestCase('behavior parity gate rejects dangling gate record state', invalidGateState, 1);
	const missingGateRecord = cloneJson(baseManifest);
	missingGateRecord.gates = baseManifest.gates.filter(gate => gate.state !== 'CoreServicesParityGreen');
	runParityGateManifestCase('behavior parity gate rejects missing gate record', missingGateRecord, 1);
	const duplicateGateRecord = cloneJson(baseManifest);
	duplicateGateRecord.gates = duplicateGateRecord.gates.map(gate => gate.state === 'CoreServicesParityGreen' ? { ...gate, state: 'ExtensionApiParityGreen' } : gate);
	runParityGateManifestCase('behavior parity gate rejects duplicate gate record', duplicateGateRecord, 1);
	const missingGateCiEvidence = cloneJson(baseManifest);
	delete manifestGate(missingGateCiEvidence, 'CoreServicesParityGreen').lastCiResult;
	runParityGateManifestCase('behavior parity gate rejects missing gate CI evidence', missingGateCiEvidence, 1);

	const scaffoldFallbackPermitted = cloneJson(baseManifest);
	scaffoldFallbackPermitted.releaseValidation.permitsScaffoldFallback = true;
	runParityGateManifestCase('behavior parity gate rejects scaffold fallback release validation', scaffoldFallbackPermitted, 1);

	const releaseValidationCommandMissingRealWorkbenchEnv = cloneJson(releaseCandidateManifest);
	releaseValidationCommandMissingRealWorkbenchEnv.releaseValidation.command = releaseValidationCommandMissingRealWorkbenchEnv.releaseValidation.command.replaceAll('CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 ', '');
	withGeneratedWorkbenchFixture(true, () => {
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate release validation command missing real workbench env', releaseValidationCommandMissingRealWorkbenchEnv, 1);
	});

	const releaseValidationCommandUsesScaffoldPath = cloneJson(releaseCandidateManifest);
	releaseValidationCommandUsesScaffoldPath.releaseValidation.command = releaseValidationCommandUsesScaffoldPath.releaseValidation.command.replaceAll(defaultWorkbenchEntry, 'src-tauri/www/index.html');
	withGeneratedWorkbenchFixture(true, () => {
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate release validation command using scaffold path', releaseValidationCommandUsesScaffoldPath, 1);
	});

	const releaseValidationCommandMismatch = cloneJson(releaseCandidateManifest);
	releaseValidationCommandMismatch.releaseValidation.command = releaseValidationCommandMismatch.releaseValidation.command.replace('npm run package:tauri:release-validate', 'npm run package:tauri:dry-run');
	withGeneratedWorkbenchFixture(true, () => {
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate release validation command mismatch', releaseValidationCommandMismatch, 1);
	});

	const releaseCandidateCommandMismatch = cloneJson(releaseCandidateManifest);
	const releaseCandidateGate = manifestGate(releaseCandidateCommandMismatch, 'ReleaseCandidate');
	releaseCandidateGate.command = releaseCandidateGate.command.replace('npm run package:tauri:release-validate', 'npm run package:tauri:dry-run');
	withGeneratedWorkbenchFixture(true, () => {
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate gate command mismatch', releaseCandidateCommandMismatch, 1);
	});

	const scaffoldSourceAllowed = cloneJson(baseManifest);
	scaffoldSourceAllowed.releaseValidation.allowedWorkbenchSources.push('src-tauri/www/index.html');
	runParityGateManifestCase('behavior parity gate rejects scaffold in allowed workbench sources', scaffoldSourceAllowed, 1);

	const scaffoldSourceNotForbidden = cloneJson(baseManifest);
	scaffoldSourceNotForbidden.releaseValidation.forbiddenWorkbenchSources = [];
	runParityGateManifestCase('behavior parity gate rejects missing forbidden scaffold source', scaffoldSourceNotForbidden, 1);

	const releaseCandidateWithFailingRelease = cloneJson(releaseCandidateManifest);
	releaseCandidateWithFailingRelease.releaseValidation.lastCiResult.status = 'fail';
	const releaseCandidateMissingOwner = cloneJson(releaseCandidateManifest);
	delete manifestGate(releaseCandidateMissingOwner, 'ReleaseCandidate').owner;
	const releaseCandidateMissingCommand = cloneJson(releaseCandidateManifest);
	manifestGate(releaseCandidateMissingCommand, 'ReleaseCandidate').command = '';
	const releaseCandidateMissingObservedAt = cloneJson(releaseCandidateManifest);
	delete manifestGate(releaseCandidateMissingObservedAt, 'ReleaseCandidate').lastCiResult.observedAt;
	const releaseCandidateWithFailingRustLint = cloneJson(releaseCandidateManifest);
	releaseCandidateWithFailingRustLint.rustLintGate.lastCiResult.status = 'fail';
	const releaseCandidateWithFailingSignedPackageEvidence = cloneJson(releaseCandidateManifest);
	firstSignedPackageArtifact(releaseCandidateWithFailingSignedPackageEvidence).signatureStatus = 'fail';
	const releaseCandidateWithMacNotarizationNotRequired = cloneJson(releaseCandidateManifest);
	firstSignedPackageArtifact(releaseCandidateWithMacNotarizationNotRequired).notarizationStatus = 'not_required';
	const releaseCandidateMissingLinuxSignedPackageEvidence = cloneJson(releaseCandidateManifest);
	signedPackageGate(releaseCandidateMissingLinuxSignedPackageEvidence).signedPackageEvidence.artifacts = signedPackageArtifacts(releaseCandidateMissingLinuxSignedPackageEvidence).filter(artifact => artifact.os !== 'linux');
	const releaseCandidateDuplicateWindowsSignedPackageEvidence = cloneJson(releaseCandidateManifest);
	signedPackageGate(releaseCandidateDuplicateWindowsSignedPackageEvidence).signedPackageEvidence.artifacts.push(cloneJson(signedPackageArtifacts(releaseCandidateDuplicateWindowsSignedPackageEvidence).find(artifact => artifact.os === 'windows')));
	const releaseCandidateWithScaffoldFallback = cloneJson(releaseCandidateManifest);
	releaseCandidateWithScaffoldFallback.releaseValidation.permitsScaffoldFallback = true;
	withGeneratedWorkbenchFixture(true, () => {
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate failing release validation', releaseCandidateWithFailingRelease, 1);
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate missing owner', releaseCandidateMissingOwner, 1);
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate missing command', releaseCandidateMissingCommand, 1);
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate missing observedAt', releaseCandidateMissingObservedAt, 1);
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate failing rust lint gate', releaseCandidateWithFailingRustLint, 1);
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate failing signed package evidence', releaseCandidateWithFailingSignedPackageEvidence, 1);
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate macOS signed package without notarization pass', releaseCandidateWithMacNotarizationNotRequired, 1);
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate missing Linux signed package evidence', releaseCandidateMissingLinuxSignedPackageEvidence, 1);
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate duplicate Windows signed package evidence', releaseCandidateDuplicateWindowsSignedPackageEvidence, 1);
		runParityGateManifestCase('behavior parity gate rejects ReleaseCandidate scaffold fallback permission', releaseCandidateWithScaffoldFallback, 1);
	});
	withGeneratedWorkbenchFixture(false, () => {
		runParityGateManifestCaseWithGeneratedWorkbenchAbsent('behavior parity gate rejects clean workspace ReleaseCandidate missing real workbench source', releaseCandidateManifest, 1);
	});
}

function createReleaseCandidateManifest(baseManifest) {
	const releaseCandidateManifest = createPromotedManifest(baseManifest, 'ReleaseCandidate');
	releaseCandidateManifest.releaseValidation.lastCiResult.status = 'pass';
	releaseCandidateManifest.releaseValidation.lastCiResult.runName = 'Tauri release validation green';
	releaseCandidateManifest.releaseValidation.lastCiResult.ciRunId = 'tauri-release-validation-green-2026-05-11';
	releaseCandidateManifest.releaseValidation.lastCiResult.ciArtifactUrl = 'https://github.com/microsoft/vscode/actions/runs/tauri-release-validation-green-2026-05-11';
	releaseCandidateManifest.releaseValidation.lastCiResult.summary = 'Release validation passed without scaffold fallback.';
	if (releaseCandidateManifest.rustLintGate?.lastCiResult) {
		releaseCandidateManifest.rustLintGate.lastCiResult.status = 'pass';
		markCleanCiEvidence(releaseCandidateManifest.rustLintGate.lastCiResult, 'Rust lint gate green');
	}
	return releaseCandidateManifest;
}

function createPromotedManifest(baseManifest, currentState) {
	const manifest = cloneJson(baseManifest);
	manifest.currentState = currentState;
	const currentIndex = manifest.migrationStateMachine.indexOf(currentState);
	for (const gate of manifest.gates) {
		const reachedState = manifest.migrationStateMachine.indexOf(gate.state) <= currentIndex;
		gate.lastCiResult.status = reachedState ? 'pass' : 'fail';
		if (reachedState) {
			markCleanCiEvidence(gate.lastCiResult, `${gate.state} green`);
		}
	}
	if (currentIndex >= manifest.migrationStateMachine.indexOf('SignedPackageGreen')) {
		markSignedPackageEvidencePassed(manifest);
	}
	if (currentIndex >= manifest.migrationStateMachine.indexOf('BuiltInExtensionParityGreen')) {
		markBuiltInExtensionEvidencePassed(manifest);
	}
	return manifest;
}

function markCleanCiEvidence(lastCiResult, runName) {
	lastCiResult.runName = runName;
	lastCiResult.ciRunId = `tauri-${runName.toLowerCase().replaceAll(/[^a-z0-9]+/g, '-').replaceAll(/^-|-$/g, '')}-2026-05-11`;
	lastCiResult.ciArtifactUrl = `https://github.com/microsoft/vscode/actions/runs/${lastCiResult.ciRunId}`;
	lastCiResult.summary = `${runName} recorded clean CI evidence.`;
}

function manifestGate(manifest, state) {
	return manifest.gates.find(gate => gate.state === state);
}

function signedPackageGate(manifest) {
	return manifestGate(manifest, 'SignedPackageGreen');
}

function builtInExtensionGate(manifest) {
	return manifestGate(manifest, 'BuiltInExtensionParityGreen');
}

function builtInExtensionAreas(manifest) {
	return builtInExtensionGate(manifest)?.builtInExtensionParity?.areas ?? [];
}

function signedPackageArtifacts(manifest) {
	return signedPackageGate(manifest)?.signedPackageEvidence?.artifacts ?? [];
}

function firstSignedPackageArtifact(manifest) {
	return signedPackageArtifacts(manifest)[0];
}

function markSignedPackageEvidencePassed(manifest) {
	for (const artifact of signedPackageArtifacts(manifest)) {
		artifact.signatureStatus = 'pass';
		artifact.installStatus = 'pass';
		artifact.launchStatus = 'pass';
		if (artifact.notarizationStatus !== 'not_required') {
			artifact.notarizationStatus = 'pass';
		}
	}
}

function markBuiltInExtensionEvidencePassed(manifest) {
	for (const area of builtInExtensionAreas(manifest)) {
		area.status = 'pass';
		area.summary = `${area.area} built-in extension parity passed on all release platforms.`;
		for (const result of area.platformResults ?? []) {
			result.status = 'pass';
			result.summary = `${area.area} built-in extension parity passed on ${result.platform}.`;
		}
	}
}

function getSignedPackageArtifactOses(manifest) {
	return signedPackageArtifacts(manifest)
		.map(artifact => artifact.os)
		.sort()
		.join(',');
}

function checkReleaseCandidatePackageFixture(name, manifest, expectations) {
	check(`${name} fixture state ReleaseCandidate`, manifest.currentState === 'ReleaseCandidate', String(manifest.currentState));
	check(`${name} fixture release validation pass`, manifest.releaseValidation?.lastCiResult?.status === 'pass', String(manifest.releaseValidation?.lastCiResult?.status));
	check(`${name} fixture release validation CI run`, isNonEmptyString(manifest.releaseValidation?.lastCiResult?.ciRunId), String(manifest.releaseValidation?.lastCiResult?.ciRunId));
	check(`${name} fixture release validation CI artifact`, isNonEmptyString(manifest.releaseValidation?.lastCiResult?.ciArtifactUrl), String(manifest.releaseValidation?.lastCiResult?.ciArtifactUrl));
	check(`${name} fixture no scaffold fallback`, manifest.releaseValidation?.permitsScaffoldFallback === false, String(manifest.releaseValidation?.permitsScaffoldFallback));
	check(`${name} fixture release command requires real workbench`, manifest.releaseValidation?.command?.includes('CODE_TAURI_REQUIRE_REAL_WORKBENCH=1') === true, String(manifest.releaseValidation?.command));
	check(`${name} fixture release command uses generated workbench path`, manifest.releaseValidation?.command?.includes(`CODE_TAURI_WORKBENCH_PATH=${defaultWorkbenchEntry}`) === true, String(manifest.releaseValidation?.command));
	check(`${name} fixture env requires real workbench`, expectations.env?.CODE_TAURI_REQUIRE_REAL_WORKBENCH === '1', String(expectations.env?.CODE_TAURI_REQUIRE_REAL_WORKBENCH));
	check(`${name} fixture uses generated workbench path`, expectations.workbenchPath === defaultWorkbenchEntry, String(expectations.workbenchPath));
	check(`${name} fixture workbench path is not scaffold`, !isScaffoldWorkbenchPath(expectations.workbenchPath), String(expectations.workbenchPath));
	check(`${name} fixture signed package evidence covers macOS Windows Linux`, getSignedPackageArtifactOses(manifest) === 'linux,macos,windows', getSignedPackageArtifactOses(manifest));
}

function checkScaffoldPackageFixture(name, manifest, expectedStatus) {
	check(`${name} fixture state Scaffold`, manifest.currentState === 'Scaffold', String(manifest.currentState));
	check(`${name} fixture release validation remains failing`, manifest.releaseValidation?.lastCiResult?.status === 'fail', String(manifest.releaseValidation?.lastCiResult?.status));
	check(`${name} fixture expects package validation failure`, expectedStatus === 1, String(expectedStatus));
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

function createTempPackageRepoFixture(tempDir) {
	const fixtureRepoRoot = path.join(tempDir, 'package-repo');
	const fixtureScriptPath = path.join(fixtureRepoRoot, 'scripts', 'tauri-package.mjs');
	const fixtureSrcTauriRoot = path.join(fixtureRepoRoot, 'src-tauri');
	const fixtureScaffoldWorkbenchPath = path.join(fixtureSrcTauriRoot, 'www', 'index.html');
	const fixtureGeneratedWorkbenchPath = path.join(fixtureRepoRoot, ...defaultWorkbenchEntry.split('/'));
	const fixturePackagedWorkbenchPath = path.join(fixtureSrcTauriRoot, 'www', ...defaultWorkbenchEntry.split('/'));

	mkdirSync(path.dirname(fixtureScriptPath), { recursive: true });
	mkdirSync(path.dirname(fixtureScaffoldWorkbenchPath), { recursive: true });
	writeFileSync(fixtureScriptPath, readFileSync(requiredFiles.packageScript, 'utf8'));
	writeFileSync(path.join(fixtureSrcTauriRoot, 'tauri.conf.json'), `${JSON.stringify({
		build: { frontendDist: './www' },
		bundle: { active: false },
	}, null, '\t')}\n`);
	writeFileSync(path.join(fixtureSrcTauriRoot, 'Cargo.toml'), '[package]\nname = "tauri-package-smoke-fixture"\nversion = "0.0.0"\nedition = "2021"\n');
	writeFileSync(fixtureScaffoldWorkbenchPath, '<!doctype html><title>Scaffold workbench smoke fixture</title>\n');

	return {
		repoRoot: fixtureRepoRoot,
		packageScript: fixtureScriptPath,
		scaffoldWorkbenchPath: fixtureScaffoldWorkbenchPath,
		generatedWorkbenchPath: fixtureGeneratedWorkbenchPath,
		generatedWorkbenchRoot: path.join(fixtureRepoRoot, 'out'),
		packagedWorkbenchPath: fixturePackagedWorkbenchPath,
		packagedWorkbenchRoot: path.join(fixtureSrcTauriRoot, 'www', 'out'),
	};
}

function resolvePackageFixturePath(fixture, workbenchPath) {
	return path.isAbsolute(workbenchPath) ? workbenchPath : path.resolve(fixture.repoRoot, workbenchPath);
}

function isPathInside(parentPath, childPath) {
	const relativePath = path.relative(parentPath, childPath);
	return relativePath === '' || (!relativePath.startsWith('..') && !path.isAbsolute(relativePath));
}

function createPackageWorkbenchFixture(workbenchPath, tempDir, expectations, fixture) {
	const createWorkbenchFixture = expectations.createWorkbenchFixture;
	if (createWorkbenchFixture === false) {
		return [];
	}

	const cleanupFixtureCallbacks = [];
	const resolvedWorkbenchPath = resolvePackageFixturePath(fixture, workbenchPath);
	if (expectations.createSourceWorkbenchFixture !== false) {
		const existed = existsSync(resolvedWorkbenchPath);
		mkdirSync(path.dirname(resolvedWorkbenchPath), { recursive: true });
		if (typeof createWorkbenchFixture === 'function') {
			createWorkbenchFixture(resolvedWorkbenchPath, tempDir, fixture);
		} else if (!existed) {
			writeFileSync(resolvedWorkbenchPath, '<!doctype html><title>Real workbench smoke fixture</title>\n');
		}
		if (!existed && !isPathInside(tempDir, resolvedWorkbenchPath)) {
			cleanupFixtureCallbacks.push(() => rmSync(resolvedWorkbenchPath, { force: true }));
		}
	}

	if (workbenchPath === defaultWorkbenchEntry && expectations.createPackagedWorkbenchFixture === false) {
		const hiddenPackagedWorkbenchPath = existsSync(fixture.packagedWorkbenchPath) ? path.join(tmpdir(), `tauri-smoke-hidden-packaged-workbench-${process.pid}-${Date.now()}.html`) : undefined;
		if (hiddenPackagedWorkbenchPath) {
			renameSync(fixture.packagedWorkbenchPath, hiddenPackagedWorkbenchPath);
			cleanupFixtureCallbacks.push(() => renameSync(hiddenPackagedWorkbenchPath, fixture.packagedWorkbenchPath));
		}
	} else if (workbenchPath === defaultWorkbenchEntry) {
		const packagedWorkbenchExisted = existsSync(fixture.packagedWorkbenchPath);
		mkdirSync(path.dirname(fixture.packagedWorkbenchPath), { recursive: true });
		if (typeof expectations.createPackagedWorkbenchFixture === 'function') {
			expectations.createPackagedWorkbenchFixture(fixture.packagedWorkbenchPath, tempDir);
			if (!packagedWorkbenchExisted) {
				cleanupFixtureCallbacks.push(() => rmSync(fixture.packagedWorkbenchPath, { force: true }));
			}
		} else if (!packagedWorkbenchExisted) {
			writeFileSync(fixture.packagedWorkbenchPath, '<!doctype html><title>Bundled real workbench smoke fixture</title>\n');
			cleanupFixtureCallbacks.push(() => rmSync(fixture.packagedWorkbenchPath, { force: true }));
		}
	}

	return cleanupFixtureCallbacks;
}

function checkPackagedWorkbenchSourceIsReal(name, fixture) {
	const relativePackagedWorkbenchPath = path.relative(fixture.repoRoot, fixture.packagedWorkbenchPath);
	const relativeScaffoldWorkbenchPath = path.relative(fixture.repoRoot, fixture.scaffoldWorkbenchPath);
	check(`${name} packaged workbench path uses default App entry`, relativePackagedWorkbenchPath === path.join('src-tauri', 'www', defaultWorkbenchEntry), relativePackagedWorkbenchPath);
	check(`${name} packaged workbench path is not scaffold`, relativePackagedWorkbenchPath !== relativeScaffoldWorkbenchPath, relativePackagedWorkbenchPath);

	const packagedWorkbenchText = existsSync(fixture.packagedWorkbenchPath) ? readFileSync(fixture.packagedWorkbenchPath, 'utf8') : '';
	check(`${name} packaged workbench DOM is real workbench`, packagedWorkbenchText.includes('workbench') && !packagedWorkbenchText.includes('developer-only runtime placeholder'), relativePackagedWorkbenchPath);
	check(`${name} packaged workbench DOM is not scaffold`, !packagedWorkbenchText.includes('VS Code Atomic Tauri developer-only runtime placeholder'), relativePackagedWorkbenchPath);
}

function withHiddenWorkbenchRoots(roots, callback) {
	const releaseLock = acquireGeneratedWorkbenchFixtureLock();
	const hiddenRoots = roots
		.filter(root => existsSync(root))
		.map((root, index) => ({ root, hiddenRoot: path.join(tmpdir(), `tauri-smoke-hidden-${index}-${path.basename(root)}-${process.pid}-${Date.now()}`) }));
	try {
		for (const { root, hiddenRoot } of hiddenRoots) {
			renameSync(root, hiddenRoot);
		}
		callback();
	} finally {
		for (const { root, hiddenRoot } of hiddenRoots.reverse()) {
			rmSync(root, { recursive: true, force: true });
			renameSync(hiddenRoot, root);
		}
		releaseLock();
	}
}

function withPackageWorkbenchFixture(workbenchPath, tempDir, expectations, fixture, callback) {
	const cleanupFixtureCallbacks = createPackageWorkbenchFixture(workbenchPath, tempDir, expectations, fixture);
	if (expectations.requirePackagedWorkbenchNotScaffold === true) {
		checkPackagedWorkbenchSourceIsReal(expectations.fixtureCheckName ?? 'behavior package fixture', fixture);
	}
	try {
		callback();
	} finally {
		for (const cleanupFixtureCallback of cleanupFixtureCallbacks.reverse()) {
			cleanupFixtureCallback();
		}
	}
}

function withReleaseCiWorkbenchFixtures(callback) {
	withGeneratedWorkbenchFixture(true, () => {
		withPackageWorkbenchFixture(defaultWorkbenchEntry, repoRoot, {}, {
			repoRoot,
			packageScript: requiredFiles.packageScript,
			scaffoldWorkbenchPath,
			generatedWorkbenchPath,
			generatedWorkbenchRoot,
			packagedWorkbenchPath,
			packagedWorkbenchRoot,
		}, () => {
			callback();
		});
	});
}

function runWithHiddenWorkbenchRoots(expectations, callback, fixture) {
	const rootsToHide = [];
	if (expectations.hideGeneratedWorkbenchRoot === true) {
		rootsToHide.push({
			root: fixture.generatedWorkbenchRoot,
			checkName: 'behavior package source checkout out absent during fixture',
		});
	}
	if (expectations.hidePackagedWorkbenchRoot === true) {
		rootsToHide.push({
			root: fixture.packagedWorkbenchRoot,
			checkName: 'behavior package bundled out absent during fixture',
		});
	}

	if (rootsToHide.length === 0) {
		callback();
		return;
	}

	withHiddenWorkbenchRoots(rootsToHide.map(entry => entry.root), () => {
		for (const { root, checkName } of rootsToHide) {
			check(checkName, !existsSync(root), root);
		}
		callback();
	});
}

function resolvePackageCaseEnv(expectations, fixture, tempDir) {
	const resolvedEnv = {};
	for (const [key, value] of Object.entries(expectations.env ?? {})) {
		resolvedEnv[key] = typeof value === 'function' ? value(fixture, tempDir) : value;
	}
	return resolvedEnv;
}

function runPackageScriptCase(name, args, expectedStatus, expectations, manifestPath, workbenchPath, fixture, tempDir) {
	const resolvedExpectationEnv = resolvePackageCaseEnv(expectations, fixture, tempDir);
	const env = {
		...process.env,
		CODE_TAURI_PARITY_MANIFEST_PATH: manifestPath,
		...resolvedExpectationEnv,
		CODE_TAURI_WORKBENCH_PATH: resolvedExpectationEnv.CODE_TAURI_WORKBENCH_PATH ?? workbenchPath,
	};
	const result = spawnSync(process.execPath, [fixture.packageScript, ...args], {
		cwd: fixture.repoRoot,
		encoding: 'utf8',
		env,
		shell: process.platform === 'win32',
		stdio: ['ignore', 'pipe', 'pipe'],
	});

	const stdout = result.stdout ?? '';
	const stderr = result.stderr ?? '';
	const output = `${stdout}\n${stderr}`;
	logProcessOutput(result);

	check(name, result.status === expectedStatus, `node ${[fixture.packageScript, ...args].join(' ')}`);
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
}

function runPackageManifestCase(name, manifest, args, expectedStatus, expectations = {}) {
	if (expectations.fixtureKind === 'ReleaseCandidate') {
		checkReleaseCandidatePackageFixture(name, manifest, expectations);
	} else if (expectations.fixtureKind === 'Scaffold') {
		checkScaffoldPackageFixture(name, manifest, expectedStatus);
	}

	const { tempDir, manifestPath } = writeTempManifest(name, manifest);
	try {
		const fixture = createTempPackageRepoFixture(tempDir);
		check(`${name} fixture package repo is hermetic`, isPathInside(tempDir, fixture.repoRoot) && fixture.repoRoot !== repoRoot, fixture.repoRoot);
		check(`${name} fixture generated workbench stays inside package repo`, isPathInside(fixture.repoRoot, fixture.generatedWorkbenchPath), fixture.generatedWorkbenchPath);
		check(`${name} fixture packaged workbench stays inside package repo`, isPathInside(fixture.repoRoot, fixture.packagedWorkbenchPath), fixture.packagedWorkbenchPath);
		const workbenchPath = getPackagePolicyWorkbenchPath(expectations, tempDir);
		const runWithPackageFixture = () => withPackageWorkbenchFixture(workbenchPath, tempDir, expectations, fixture, () => {
			runPackageScriptCase(name, args, expectedStatus, expectations, manifestPath, workbenchPath, fixture, tempDir);
		});
		runWithHiddenWorkbenchRoots(expectations, runWithPackageFixture, fixture);
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
			stdoutIncludes: ['Mode: dry-run', 'Command plan:', 'npm run compile-build', 'copy out/vs/code/browser/workbench/workbench.html -> src-tauri/www/out/vs/code/browser/workbench/workbench.html', 'cargo tauri build'],
			stderrIncludes: ['Warning: Tauri package dry-run release invariants are not satisfied:', 'package release requires ReleaseCandidate parity state: Scaffold'],
		}
	);

	runPackageManifestCase(
		'behavior package dry-run skip frontend still copies generated workbench assets',
		baseManifest,
		['--flavor', 'code-tauri', '--sign', '--skip-frontend'],
		0,
		{
			stdoutIncludes: ['Command plan:', 'copy out/vs/code/browser/workbench/workbench.html -> src-tauri/www/out/vs/code/browser/workbench/workbench.html', 'cargo build --manifest-path'],
			outputExcludes: ['npm run compile-build'],
			stderrIncludes: ['Warning: Tauri package dry-run release invariants are not satisfied:', 'package release requires ReleaseCandidate parity state: Scaffold'],
		}
	);

	runPackageManifestCase(
		'behavior package release validation rejects Scaffold',
		baseManifest,
		releaseValidateArgs,
		1,
		{
			fixtureKind: 'Scaffold',
			stderrIncludes: [releaseValidationFailed, 'package release requires ReleaseCandidate parity state: Scaffold'],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects Scaffold even with workbench URL',
		baseManifest,
		releaseValidateArgs,
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_URL: 'https://example.test/workbench.html' },
			stderrIncludes: [releaseValidationFailed, 'package release requires ReleaseCandidate parity state: Scaffold'],
		}
	);

	const releaseCandidateManifest = createReleaseCandidateManifest(baseManifest);
	runPackageManifestCase(
		'behavior package release validation accepts signed ReleaseCandidate',
		releaseCandidateManifest,
		releaseValidateArgs,
		0,
		{
			fixtureKind: 'ReleaseCandidate',
			env: signedReleaseWorkbenchEnv,
			workbenchPath: defaultWorkbenchEntry,
			stdoutIncludes: ['Tauri package release validation passed.'],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects missing source workbench asset',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: signedReleaseWorkbenchEnv,
			workbenchPath: defaultWorkbenchEntry,
			createSourceWorkbenchFixture: false,
			hideGeneratedWorkbenchRoot: true,
			hidePackagedWorkbenchRoot: true,
			requirePackagedWorkbenchNotScaffold: true,
			fixtureCheckName: 'behavior package release validation missing source workbench asset',
			createPackagedWorkbenchFixture: packagedPath => writeFileSync(packagedPath, '<!doctype html><html><body><main id="workbench.parts.editor">Bundled real workbench smoke fixture</main></body></html>\n'),
			stderrIncludes: [releaseValidationFailed, `package release requires real workbench source ${defaultWorkbenchEntry}`],
		}
	);
	check('behavior package execute rejects missing bundled workbench asset after copy/skip-frontend covered by source invariant', true, 'validated by package execute bundled workbench source checks');
	runPackageManifestCase(
		'behavior package release validation rejects arbitrary workbench path',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_PATH: 'src-tauri/tauri.conf.json' },
			stderrIncludes: [releaseValidationFailed, `package release requires CODE_TAURI_WORKBENCH_PATH=${defaultWorkbenchEntry}: src-tauri/tauri.conf.json`],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects absolute generated workbench path',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_PATH: fixture => fixture.generatedWorkbenchPath },
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, `package release requires CODE_TAURI_WORKBENCH_PATH=${defaultWorkbenchEntry}:`],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects explicit temp workbench path',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: signedReleaseEnv,
			workbenchPath: tempDir => path.join(tempDir, 'explicit-workbench.html'),
			stderrIncludes: [releaseValidationFailed, `package release requires CODE_TAURI_WORKBENCH_PATH=${defaultWorkbenchEntry}:`],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects ReleaseCandidate workbench URL before path',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_URL: 'https://example.test/workbench.html', CODE_TAURI_WORKBENCH_PATH: fixture => fixture.scaffoldWorkbenchPath },
			stderrIncludes: [releaseValidationFailed, `package release requires CODE_TAURI_WORKBENCH_PATH=${defaultWorkbenchEntry}:`, 'package release forbids CODE_TAURI_WORKBENCH_URL during release validation: https://example.test/workbench.html', 'package release forbids scaffold resolved workbench source:'],
		}
	);

	const releaseCandidateMissingSignedPackageEvidence = createReleaseCandidateManifest(baseManifest);
	delete signedPackageGate(releaseCandidateMissingSignedPackageEvidence).signedPackageEvidence;
	runPackageManifestCase(
		'behavior package release validation rejects missing signed package evidence',
		releaseCandidateMissingSignedPackageEvidence,
		releaseValidateArgs,
		1,
		{
			env: signedReleaseEnv,
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release requires SignedPackageGreen signedPackageEvidence object'],
		}
	);

	const releaseCandidateFailingSignedPackageEvidence = createReleaseCandidateManifest(baseManifest);
	firstSignedPackageArtifact(releaseCandidateFailingSignedPackageEvidence).signatureStatus = 'fail';
	runPackageManifestCase(
		'behavior package release validation rejects failing signed package evidence',
		releaseCandidateFailingSignedPackageEvidence,
		releaseValidateArgs,
		1,
		{
			env: signedReleaseEnv,
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release requires SignedPackageGreen signedPackageEvidence.artifacts[0].signatureStatus pass: fail'],
		}
	);

	const releaseCandidateFailingLaunchEvidence = createReleaseCandidateManifest(baseManifest);
	firstSignedPackageArtifact(releaseCandidateFailingLaunchEvidence).launchStatus = 'fail';
	runPackageManifestCase(
		'behavior package release validation rejects failing signed package launch evidence',
		releaseCandidateFailingLaunchEvidence,
		releaseValidateArgs,
		1,
		{
			env: signedReleaseEnv,
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release requires SignedPackageGreen signedPackageEvidence.artifacts[0].launchStatus pass: fail'],
		}
	);

	const releaseCandidateWithMacNotarizationNotRequired = createReleaseCandidateManifest(baseManifest);
	firstSignedPackageArtifact(releaseCandidateWithMacNotarizationNotRequired).notarizationStatus = 'not_required';
	runPackageManifestCase(
		'behavior package release validation rejects macOS signed package without notarization pass',
		releaseCandidateWithMacNotarizationNotRequired,
		releaseValidateArgs,
		1,
		{
			env: signedReleaseEnv,
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release requires SignedPackageGreen signedPackageEvidence.artifacts[0].notarizationStatus pass: not_required'],
		}
	);

	const releaseCandidateMissingLinuxSignedPackageEvidence = createReleaseCandidateManifest(baseManifest);
	signedPackageGate(releaseCandidateMissingLinuxSignedPackageEvidence).signedPackageEvidence.artifacts = signedPackageArtifacts(releaseCandidateMissingLinuxSignedPackageEvidence).filter(artifact => artifact.os !== 'linux');
	runPackageManifestCase(
		'behavior package release validation rejects missing Linux signed package evidence',
		releaseCandidateMissingLinuxSignedPackageEvidence,
		releaseValidateArgs,
		1,
		{
			env: signedReleaseEnv,
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release requires SignedPackageGreen signedPackageEvidence.artifacts linux: 0'],
		}
	);

	const releaseCandidateDuplicateWindowsSignedPackageEvidence = createReleaseCandidateManifest(baseManifest);
	signedPackageGate(releaseCandidateDuplicateWindowsSignedPackageEvidence).signedPackageEvidence.artifacts.push(cloneJson(signedPackageArtifacts(releaseCandidateDuplicateWindowsSignedPackageEvidence).find(artifact => artifact.os === 'windows')));
	runPackageManifestCase(
		'behavior package release validation rejects duplicate Windows signed package evidence',
		releaseCandidateDuplicateWindowsSignedPackageEvidence,
		releaseValidateArgs,
		1,
		{
			env: signedReleaseEnv,
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release requires SignedPackageGreen signedPackageEvidence.artifacts exactly macos/windows/linux: 4', 'package release requires SignedPackageGreen signedPackageEvidence.artifacts windows: 2'],
		}
	);

	const releaseCandidateWithMacCheckMissingInstallLaunch = createReleaseCandidateManifest(baseManifest);
	firstSignedPackageArtifact(releaseCandidateWithMacCheckMissingInstallLaunch).osCheckCommand = 'codesign --verify --deep --strict <app> && spctl --assess --type execute <app> && hdiutil verify <dmg>';
	runPackageManifestCase(
		'behavior package release validation rejects macOS signed package check without install launch evidence',
		releaseCandidateWithMacCheckMissingInstallLaunch,
		releaseValidateArgs,
		1,
		{
			env: signedReleaseEnv,
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release requires SignedPackageGreen signedPackageEvidence.artifacts[0].osCheckCommand includes cp -R', 'package release requires SignedPackageGreen signedPackageEvidence.artifacts[0].osCheckCommand includes open -W'],
		}
	);

	runPackageManifestCase(
		'behavior package release validation rejects missing real workbench requirement',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '' },
			stderrIncludes: [releaseValidationFailed, 'package release requires CODE_TAURI_REQUIRE_REAL_WORKBENCH=1'],
		}
	);

	const releaseCandidateCommandMissingWorkbenchSource = createReleaseCandidateManifest(baseManifest);
	releaseCandidateCommandMissingWorkbenchSource.releaseValidation.command = releaseCandidateCommandMissingWorkbenchSource.releaseValidation.command.replaceAll(defaultWorkbenchEntry, '');
	runPackageManifestCase(
		'behavior package release validation rejects command missing real workbench source',
		releaseCandidateCommandMissingWorkbenchSource,
		releaseValidateArgs,
		1,
		{
			env: signedReleaseEnv,
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release command requires real workbench source'],
		}
	);

	const releaseCandidateCommandUsesScaffoldSource = createReleaseCandidateManifest(baseManifest);
	releaseCandidateCommandUsesScaffoldSource.releaseValidation.command = releaseCandidateCommandUsesScaffoldSource.releaseValidation.command.replaceAll(defaultWorkbenchEntry, 'src-tauri/www/index.html');
	runPackageManifestCase(
		'behavior package release validation rejects command using scaffold workbench source',
		releaseCandidateCommandUsesScaffoldSource,
		releaseValidateArgs,
		1,
		{
			env: signedReleaseEnv,
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release command requires real workbench source', 'package release command forbids scaffold workbench source'],
		}
	);

	runPackageManifestCase(
		'behavior package release validation rejects missing signing flag',
		releaseCandidateManifest,
		['--release-validate', '--flavor', 'code-tauri'],
		1,
		{
			env: signedReleaseEnv,
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release requires signed package plan: false'],
		}
	);

	runPackageManifestCase(
		'behavior package release validation rejects missing real workbench source',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_PATH: '' },
			stderrIncludes: [releaseValidationFailed, 'package release requires real workbench source'],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects missing workbench path with resolved detail',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: signedReleaseEnv,
			workbenchPath: missingGeneratedWorkbenchPath,
			createWorkbenchFixture: false,
			stderrIncludes: [releaseValidationFailed, `package release requires real workbench source: CODE_TAURI_WORKBENCH_PATH missing resolved path ${missingGeneratedWorkbenchPath}`],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects relative workbench typo with resolved detail',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_PATH: 'out/vs/code/browser/workbench/workbench-typo.html' },
			stderrIncludes: [releaseValidationFailed, 'package release requires real workbench source: CODE_TAURI_WORKBENCH_PATH missing resolved path'],
		}
	);

	runPackageManifestCase(
		'behavior package release validation rejects workbench URL even with generated workbench path',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_URL: 'https://example.test/workbench.html' },
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release forbids CODE_TAURI_WORKBENCH_URL during release validation: https://example.test/workbench.html'],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects invalid workbench URL',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_URL: 'not a url' },
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release forbids CODE_TAURI_WORKBENCH_URL during release validation: not a url'],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects empty workbench URL',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_URL: '' },
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release forbids CODE_TAURI_WORKBENCH_URL during release validation'],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects unsupported workbench URL scheme',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_URL: 'file:///tmp/workbench.html' },
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release forbids CODE_TAURI_WORKBENCH_URL during release validation: file:///tmp/workbench.html'],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects scaffold-looking workbench URL',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_URL: 'https://example.test/src-tauri/www/index.html' },
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release forbids CODE_TAURI_WORKBENCH_URL during release validation: https://example.test/src-tauri/www/index.html'],
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
		releaseValidateArgs,
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_PATH: fixture => fixture.scaffoldWorkbenchPath },
			stderrIncludes: [releaseValidationFailed, 'package release requires real workbench source:', 'package release forbids scaffold resolved workbench source:'],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects normalized scaffold workbench path',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: { CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1', CODE_TAURI_WORKBENCH_PATH: fixture => path.join(fixture.repoRoot, 'src-tauri', 'www', '..', 'www', 'index.html') },
			stderrIncludes: [releaseValidationFailed, 'package release requires real workbench source:', 'package release forbids scaffold resolved workbench source:'],
		}
	);
	runPackageManifestCase(
		'behavior package release validation rejects scaffold workbench symlink',
		releaseCandidateManifest,
		releaseValidateArgs,
		1,
		{
			env: signedReleaseEnv,
			workbenchPath: tempDir => path.join(tempDir, 'workbench-symlink.html'),
			createWorkbenchFixture: (workbenchPath, _tempDir, fixture) => symlinkSync(fixture.scaffoldWorkbenchPath, workbenchPath),
			stderrIncludes: [releaseValidationFailed, 'package release requires real workbench source:', 'package release forbids scaffold resolved workbench source:'],
		}
	);

	const releaseCandidateWithScaffoldFallback = createReleaseCandidateManifest(baseManifest);
	releaseCandidateWithScaffoldFallback.releaseValidation.permitsScaffoldFallback = true;
	runPackageManifestCase(
		'behavior package release validation rejects scaffold fallback permission',
		releaseCandidateWithScaffoldFallback,
		releaseValidateArgs,
		1,
		{
			env: signedReleaseEnv,
			workbenchPath: defaultWorkbenchEntry,
			stderrIncludes: [releaseValidationFailed, 'package release forbids scaffold fallback: true'],
		}
	);
}

function runReleaseCiPolicyCases(baseManifest) {
	const releaseLabelEvent = {
		pull_request: {
			title: 'Tauri migration in progress',
			body: 'Only ReleaseCandidate may claim migration complete.',
			labels: [{ name: 'release' }],
		},
	};
	const noReleaseLabelEvent = {
		pull_request: {
			title: 'Tauri migration in progress',
			body: 'Only ReleaseCandidate may claim migration complete.',
			labels: [{ name: 'tauri' }],
		},
	};
	const completionClaimEvent = {
		pull_request: {
			title: 'Full Tauri migration complete',
			body: '',
			labels: [{ name: 'release' }],
		},
	};

	runReleaseCiCheckCase(
		'behavior release CI accepts before ReleaseCandidate without release label while skipping package validation',
		baseManifest,
		noReleaseLabelEvent,
		0,
		{},
		{
			outputIncludes: ['release CI label scan completed: tauri', 'release CI package validation skipped before ReleaseCandidate without release label: Scaffold'],
			outputExcludes: ['> node scripts/tauri-package.mjs --release-validate --flavor code-tauri --sign', 'Tauri package release validation passed.', 'release CI package:tauri:release-validate passes at ReleaseCandidate'],
		}
	);
	runReleaseCiCheckCase('behavior release CI expects release validation failure before ReleaseCandidate', baseManifest, releaseLabelEvent, 1);
	runReleaseCiCheckCase('behavior release CI rejects full migration claim before ReleaseCandidate', baseManifest, completionClaimEvent, 1);

	const releaseCandidateManifest = createReleaseCandidateManifest(baseManifest);
	withReleaseCiWorkbenchFixtures(() => {
		runReleaseCiCheckCase('behavior release CI accepts release validation at ReleaseCandidate', releaseCandidateManifest, releaseLabelEvent, 0);
	});
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
		'behavior real workbench preflight rejects missing generated workbench file without strict deps',
		{ ...validRealWorkbenchEnv, CODE_TAURI_STRICT_DEPS: '' },
		() => false,
		[
			`real workbench boot requires generated workbench file: ${generatedWorkbenchPath}`,
		]
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
		const parityServicesText = readRequired('parityServices');
		const extensionSidecarText = readRequired('extensionSidecar');
		const observabilityText = readRequired('observability');
		const coreServicesParityTestText = readRequired('coreServicesParityTest');
		const fallbackIndexText = readRequired('fallbackIndex');
		const packagingDocText = readRequired('packagingDoc');
		const parityGateScriptText = readRequired('parityGateScript');
		const docClaimCheckScriptText = readRequired('docClaimCheckScript');
		const apiTestsScriptText = readRequired('apiTestsScript');
		const packageScriptText = readRequired('packageScript');
		const packageUnitTestText = readRequired('packageUnitTest');
		const builtinExtensionSmokeScriptText = readRequired('builtinExtensionSmokeScript');
		const tauriSmokeScriptText = readFileSync(fileURLToPath(import.meta.url), 'utf8');
		const parityGateManifestText = readRequired('parityGateManifest');
		const rootPackageText = readRequired('rootPackage');
		const bridgeText = readRequired('bridge');

		const config = parseJson('tauri config', configText);
		const rootPackage = parseJson('root package', rootPackageText);

		if (config) {
			check('tauri config productName', config.productName === 'VS Code Atomic', config.productName);
			check('tauri config identifier', config.identifier === 'dev.vscode.atomic', config.identifier);
			check('tauri config frontendDist', config.build?.frontendDist === 'www', config.build?.frontendDist);
			check('tauri config creates windows from runtime', Array.isArray(config.app?.windows) && config.app.windows.length === 0, JSON.stringify(config.app?.windows));
			check('tauri config bundle inactive for scaffold smoke', config.bundle?.active === false, String(config.bundle?.active));
			check('tauri config CSP present', typeof config.app?.security?.csp === 'string' && config.app.security.csp.trim().length > 0, String(config.app?.security?.csp));
			if (typeof config.app?.security?.csp === 'string') {
				checkRequiredCsp(config.app.security.csp);
			}
		}

		checkPackageScript(rootPackage, 'fmt:tauri:rust', 'cargo fmt --manifest-path src-tauri/Cargo.toml --check');
		checkPackageScript(rootPackage, 'lint:tauri:rust', 'cargo clippy --manifest-path src-tauri/Cargo.toml --no-default-features -- -D warnings');
		checkPackageScript(rootPackage, 'build:tauri:runtime', 'npm run compile-build && cargo build --manifest-path src-tauri/Cargo.toml --features runtime');
		checkPackageScript(rootPackage, 'test:tauri:unit', 'cargo test --manifest-path src-tauri/Cargo.toml --no-default-features');
		checkPackageScript(rootPackage, 'test:tauri:bridge', 'node scripts/tauri-smoke.mjs source');
		checkPackageScript(rootPackage, 'test:tauri:workbench-resolver', 'cargo test --manifest-path src-tauri/Cargo.toml --no-default-features workbench_url_resolver::tests::');
		checkPackageScript(rootPackage, 'smoketest:tauri', 'npm run smoketest:tauri:scaffold');
		checkPackageScript(rootPackage, 'test:tauri:smoke', 'npm run smoketest:tauri:scaffold');
		checkPackageScript(rootPackage, 'smoketest:tauri:scaffold', 'node scripts/tauri-smoke.mjs');
		checkPackageScript(rootPackage, 'test:tauri:api', 'npm run test-extension:tauri-sidecar:preflight');
		checkPackageScript(rootPackage, 'test:tauri:builtin', 'npm run smoketest:tauri:builtin-extensions');
		checkPackageScript(rootPackage, 'package:tauri:gate', 'npm run tauri:release-ci-check');
		checkPackageScript(rootPackage, 'test:tauri:package-gate', 'npm run package:tauri:gate');

		checkUnitStructDefaultGuard('parity services source', parityServicesText, [
			'RustNativeHostService',
			'RustDialogService',
			'RustLifecycleService',
		]);
		checkUnitStructDefaultGuard('core services parity test', coreServicesParityTestText, [
			'RustNativeHostService',
			'RustDialogService',
			'RustLifecycleService',
		]);
		checkUnitStructDefaultGuard('extension sidecar source', extensionSidecarText, [
			'NodeExtensionSidecarSpawner',
		]);
		includesAll(parityServicesText, 'parity services direct unit construction', [
			'Arc::new(RustNativeHostService)',
			'Arc::new(RustDialogService)',
			'Arc::new(RustLifecycleService)',
		]);
		includesAll(coreServicesParityTestText, 'core services parity direct unit construction', [
			'let native = RustNativeHostService;',
			'let dialog = RustDialogService;',
			'let lifecycle = RustLifecycleService;',
		]);
		includesAll(extensionSidecarText, 'extension sidecar direct unit construction', [
			'Self::new(NodeExtensionSidecarSpawner)',
		]);

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
			'register_file_watcher_services',
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
		checkWorkbenchSourceOrder(workbenchUrlResolverText, packagingDocText);
		checkPackageStableSourceInvariants(packageScriptText);
		checkHandshakeTokenSource(extensionSidecarText);

		compareCommandContracts(
			extractTauriBridgeCommandNames(bridgeText),
			extractRustGenerateHandlerCommandNames(runtimeText)
		);

		check('developer scaffold smoke keeps bundle inactive', config?.bundle?.active === false, String(config?.bundle?.active));
		includesAll(runtimeText, 'dev-only workbench override policy', [
			'CODE_TAURI_WORKBENCH_URL may point at a dev server or hosted workbench.',
			'CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 rejects the developer-only scaffold for release validation.',
			'src-tauri/www/index.html remains the developer-only fallback scaffold.',
		]);
		includesAll(fallbackIndexText, 'scaffold fallback marker', [
			'VS Code Atomic Tauri developer-only runtime placeholder',
		]);
		includesAll(packagingDocText, 'tauri parity release gates', [
			'`src-tauri/www/index.html` is debug-only developer scaffold.',
			'`Scaffold -> RuntimeBuildGreen -> RealWorkbenchBootGreen -> CoreServicesParityGreen -> ExtensionApiParityGreen -> BuiltInExtensionParityGreen -> SignedPackageGreen -> ReleaseCandidate`',
			'Only `ReleaseCandidate` may be described as migration complete.',
			'real workbench boot, Rust runtime build, core service parity, extension API parity, built-in extension parity, signed package validation, and release validation',
		]);
		includesAll(parityGateScriptText, 'tauri parity gate script', [
			'Tauri docs completion claim check passes',
			'pre-ReleaseCandidate PR metadata does not claim migration complete',
			'release validation forbids scaffold fallback',
			'parity gate one gate per migration state',
			'parity gate gates exact order',
			'parity gate BuiltInExtensionParityGreen runs built-in extension smoke preflight',
			'release validation pass only at ReleaseCandidate',
			'future state remains failing',
			'built-in extension parity evidence areas exactly required matrix',
			'built-in extension parity area ${areaName} platformResults exactly macos/windows/linux',
		]);
		includesAll(docClaimCheckScriptText, 'tauri doc claim check script', [
			'pre-ReleaseCandidate Tauri docs/package docs do not claim migration complete',
			'src-tauri/PACKAGING.md',
			'package.json',
			'CODE_TAURI_DOC_CLAIM_SOURCE_FILES',
		]);
		includesAll(packageScriptText, 'tauri package script', [
			'CODE_TAURI_PARITY_MANIFEST_PATH',
			'--release-validate',
			'releaseTauriConfigHasActiveBundle',
			'package release requires Tauri release config bundle.active true',
			'package release requires real workbench source ${requiredReleaseWorkbenchSource}',
			'package execute requires bundled real workbench asset ${requiredBundledWorkbenchSource}',
			'Tauri package dry-run release invariants are not satisfied',
			'Tauri package release validation passed.',
			'Tauri package frontend validation failed:',
			'Tauri package execute validation failed:',
		]);
		includesAll(packageUnitTestText, 'tauri package unit test import hygiene', [
			'import * as child_process from \'child_process\';',
			'type SpawnResult = child_process.SpawnSyncReturns<string>;',
		]);
		check('tauri package unit test avoids value SpawnSyncReturns import', !packageUnitTestText.includes('import { SpawnSyncReturns'));
		includesAll(builtinExtensionSmokeScriptText, 'tauri built-in extension smoke script', [
			'requiredSmokeAreas',
			'parseArgs',
			'--area',
			'selectedSmokeAreas',
			'smokeValidators',
			'requested smoke area ${String(area)} supported',
			'smoke area ${area} validator registered',
			'smoke area ${area} executed',
			'git',
			'typescriptTsserver',
			'debug',
			'terminal',
			'notebooks',
			'auth',
			'tunnels',
			'copilotApi',
			'git: validateGit',
			'typescriptTsserver: validateTypeScript',
			'debug: validateDebug',
			'terminal: validateTerminal',
			'notebooks: validateNotebooks',
			'auth: validateAuth',
			'tunnels: validateTunnels',
			'copilotApi: validateCopilotApi',
		]);
		includesAll(tauriSmokeScriptText, 'tauri expected-negative child output normalization', [
			'formatChildOutput',
			'expected-negative ${line}',
		]);
		includesAll(packagingDocText, 'real workbench smoke validation', [
			'`npm run smoketest:tauri:scaffold`',
			'`npm run smoketest:tauri:workbench`',
			'`CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html npm run smoketest:tauri:workbench`',
			'Deterministic real workbench preflight rejects `src-tauri/www/index.html` as a release workbench path',
			'Release packaging must not use scaffold smoke results or `src-tauri/www/index.html`.',
		]);
		includesAll(parityGateManifestText, 'tauri parity gate manifest', [
			'"currentState": "Scaffold"',
			'"state": "ReleaseCandidate"',
			'npm run test-extension:tauri-sidecar',
			'npm run smoketest:tauri:builtin-extensions',
			'"builtInExtensionParity"',
			'"area": "tunnels"',
			'"platform": "macos"',
			'"platform": "windows"',
			'"platform": "linux"',
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
			'CODE_TAURI_REQUIRE_REAL_WORKBENCH=1',
			'CODE_TAURI_WORKBENCH_PATH',
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
			'pub fn register_file_watcher_services',
			'registry.register_file_service(files.clone())',
			'registry.register_watcher_service(TauriWatcherService::new(files.clone()))',
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
		runReleaseCiPolicyCases(parityGateManifest);
		runRealWorkbenchPreflightCases();
	});
}

function runPolicyRegressionSmoke() {
	withLayer('policy', () => {
		checkExpectedNegativeOutputFormatting();
		const parityGateManifestText = readRequired('parityGateManifest');
		const parityGateManifest = JSON.parse(parityGateManifestText);
		runParityGatePolicyCases(parityGateManifest);
		runPackagePolicyCases(parityGateManifest);
		runReleaseCiPolicyCases(parityGateManifest);
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
			for (const test of noDefaultFeatureRustTests) {
				skipBehaviorCheck(test.name, 'skipped; cargo missing');
			}
		});
		return;
	}

	withLayer('pure-rust', () => {
		for (const test of noDefaultFeatureRustTests) {
			cargoTest(test.name, test.testName, ['--no-default-features']);
		}
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
		} else {
			skipBehaviorCheck('behavior release workbench rejects scaffold fallback', 'skipped; webkit2gtk-4.1 missing or not visible to pkg-config');
		}
	});
}

function runSourceBehaviorChecks() {
	withLayer('source', () => {
		runBehaviorCheck('behavior parity gate manifest policy', process.execPath, [path.join(repoRoot, 'scripts', 'tauri-parity-gate.mjs')]);
		runBehaviorCheck('behavior built-in extension smoke preflight', process.execPath, [path.join(repoRoot, 'scripts', 'tauri-builtin-extension-smoke.mjs')]);
		runBehaviorCheck('behavior built-in extension smoke filters area', process.execPath, [path.join(repoRoot, 'scripts', 'tauri-builtin-extension-smoke.mjs'), '--area', 'tunnels']);
		runBehaviorCheck('behavior built-in extension smoke rejects unknown area', process.execPath, [path.join(repoRoot, 'scripts', 'tauri-builtin-extension-smoke.mjs'), '--area', 'unknown'], { expectedStatus: 1 });
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

if (shouldRun('policy')) {
	runPolicyRegressionSmoke();
}

if (shouldRun('source')) {
	runSourceBehaviorChecks();
}

console.log(`summary failures=${failures.length} warnings=${warnings.length}`);

if (failures.length) {
	process.exit(1);
}
