/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { existsSync, readFileSync, statSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const isMain = process.argv[1] ? path.resolve(process.argv[1]) === fileURLToPath(import.meta.url) : false;
const apiTestsRelativePath = 'extensions/vscode-api-tests';
const apiTestsRoot = path.join(repoRoot, apiTestsRelativePath);
const vscodeDtsPath = path.join(repoRoot, 'src', 'vscode-dts', 'vscode.d.ts');
const realWorkbenchRelativePath = 'out/vs/code/browser/workbench/workbench.html';
const fallbackWorkbenchRelativePath = 'src-tauri/www/index.html';
const sidecarPath = path.join(repoRoot, 'src-tauri', 'src', 'extension_sidecar.rs');
const commandsPath = path.join(repoRoot, 'src-tauri', 'src', 'commands.rs');
const bridgePath = path.join(repoRoot, 'src', 'vs', 'platform', 'tauri', 'common', 'tauriBridge.ts');
const runtimePath = path.join(repoRoot, 'src-tauri', 'src', 'runtime.rs');

const failures = [];
const warnings = [];
const args = new Set(process.argv.slice(2));
const mode = args.has('--execute') ? 'execute' : 'preflight';
const executionTimeoutMs = Number(process.env.CODE_TAURI_VSCODE_API_TEST_TIMEOUT_MS || 10 * 60 * 1000);

const suites = [
	{
		label: 'vscode-api-tests-folder',
		workspace: path.join(apiTestsRoot, 'testWorkspace'),
		testsPath: path.join(apiTestsRoot, 'out', 'singlefolder-tests'),
		outputFolder: 'singlefolder-tests',
	},
	{
		label: 'vscode-api-tests-workspace',
		workspace: path.join(apiTestsRoot, 'testworkspace.code-workspace'),
		testsPath: path.join(apiTestsRoot, 'out', 'workspace-tests'),
		outputFolder: 'workspace-tests',
	},
];

function defaultIsFile(candidate) {
	try {
		return statSync(candidate).isFile();
	} catch {
		return false;
	}
}

function defaultIsExecutable(candidate) {
	try {
		const stat = statSync(candidate);
		return process.platform === 'win32' ? stat.isFile() : stat.isFile() && (stat.mode & 0o111) !== 0;
	} catch {
		return false;
	}
}

function resolveRepoPath(candidate) {
	return path.isAbsolute(candidate) ? candidate : path.join(repoRoot, candidate);
}

export function collectApiTestPrerequisiteDiagnostics({ mode = 'preflight', env = process.env, testSuites = suites, exists = existsSync, isFile = defaultIsFile, isExecutable = defaultIsExecutable } = {}) {
	const prerequisiteFailures = [];
	const prerequisiteWarnings = [];

	for (const suite of testSuites) {
		if (!exists(suite.testsPath)) {
			const detail = `run npm run compile-extension:vscode-api-tests before --execute; missing ${suite.testsPath}`;
			if (mode === 'execute') {
				prerequisiteFailures.push(`${suite.label} compiled tests exist for execute mode: ${suite.testsPath}`);
			} else {
				prerequisiteWarnings.push(`${suite.label} compiled tests unavailable: ${detail}`);
			}
		}
	}

	if (mode === 'execute') {
		const executable = env.CODE_TAURI_APP_PATH;
		if (typeof executable !== 'string' || executable.trim().length === 0) {
			prerequisiteFailures.push(`CODE_TAURI_APP_PATH set for execute mode: ${String(executable)}`);
		} else if (!path.isAbsolute(executable)) {
			prerequisiteFailures.push(`CODE_TAURI_APP_PATH absolute for execute mode: ${executable}`);
		} else if (!exists(executable)) {
			prerequisiteFailures.push(`CODE_TAURI_APP_PATH exists for execute mode: ${executable}`);
		} else if (!isFile(executable)) {
			prerequisiteFailures.push(`CODE_TAURI_APP_PATH file for execute mode: ${executable}`);
		} else if (!isExecutable(executable)) {
			prerequisiteFailures.push(`CODE_TAURI_APP_PATH executable for execute mode: ${executable}`);
		}

		if (env.CODE_TAURI_REQUIRE_REAL_WORKBENCH !== '1') {
			prerequisiteFailures.push(`CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 set for execute mode: ${String(env.CODE_TAURI_REQUIRE_REAL_WORKBENCH)}`);
		}

		const workbenchPath = env.CODE_TAURI_WORKBENCH_PATH;
		if (typeof workbenchPath !== 'string' || workbenchPath.trim().length === 0) {
			prerequisiteFailures.push(`CODE_TAURI_WORKBENCH_PATH set for execute mode: ${String(workbenchPath)}`);
		} else if (workbenchPath.includes(fallbackWorkbenchRelativePath)) {
			prerequisiteFailures.push(`CODE_TAURI_WORKBENCH_PATH rejects scaffold fallback for execute mode: ${workbenchPath}`);
		} else {
			const resolvedWorkbenchPath = resolveRepoPath(workbenchPath);
			if (!exists(resolvedWorkbenchPath)) {
				prerequisiteFailures.push(`CODE_TAURI_WORKBENCH_PATH exists for execute mode: ${workbenchPath}`);
			} else if (!isFile(resolvedWorkbenchPath)) {
				prerequisiteFailures.push(`CODE_TAURI_WORKBENCH_PATH file for execute mode: ${workbenchPath}`);
			}
		}
	}

	return { failures: prerequisiteFailures, warnings: prerequisiteWarnings };
}

if (isMain && args.has('--help')) {
	console.log(`Usage: node scripts/tauri-vscode-api-tests.mjs [--execute]

Validates VS Code API test wiring for the Tauri Node extension-host sidecar.

Default mode is deterministic preflight: no GUI launch, no source edits, no public API surface changes.
--execute requires CODE_TAURI_APP_PATH, CODE_TAURI_REQUIRE_REAL_WORKBENCH=1, and CODE_TAURI_WORKBENCH_PATH=${realWorkbenchRelativePath}; it runs each vscode-api-tests suite against a Tauri binary that accepts VS Code extension-test CLI arguments.`);
	process.exit(0);
}

function check(name, ok, detail) {
	const status = ok ? 'ok' : 'fail';
	console.log(`${status} ${name}${detail ? `: ${detail}` : ''}`);
	if (!ok) {
		failures.push(`${name}${detail ? `: ${detail}` : ''}`);
	}
	return ok;
}

function warn(name, detail) {
	console.log(`warn ${name}: ${detail}`);
	warnings.push(`${name}: ${detail}`);
}

function splitDiagnostic(diagnostic) {
	const separator = diagnostic.indexOf(': ');
	if (separator === -1) {
		return [diagnostic, ''];
	}

	return [diagnostic.slice(0, separator), diagnostic.slice(separator + 2)];
}

function readRequired(filePath, name) {
	return check(`${name} exists`, existsSync(filePath), filePath) ? readFileSync(filePath, 'utf8') : '';
}

function sha256(text) {
	return createHash('sha256').update(text).digest('hex');
}

function parseJson(text, name) {
	try {
		const parsed = JSON.parse(text);
		check(`${name} parses JSON`, true);
		return parsed;
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

function validateApiTestsManifest(packageJson) {
	if (!packageJson) {
		return;
	}

	check('vscode-api-tests package name', packageJson.name === 'vscode-api-tests', String(packageJson.name));
	check('vscode-api-tests extension main', packageJson.main === './out/extension', String(packageJson.main));
	check('vscode-api-tests proposed API list present', Array.isArray(packageJson.enabledApiProposals) && packageJson.enabledApiProposals.length > 0, String(packageJson.enabledApiProposals?.length));
	for (const proposal of ['extensionRuntime', 'fsChunks', 'terminalDataWriteEvent', 'tokenInformation']) {
		check(`vscode-api-tests proposed API ${proposal}`, packageJson.enabledApiProposals?.includes(proposal) === true);
	}
}

function validateApiTestsTsconfig(tsconfig) {
	if (!tsconfig) {
		return;
	}

	check('vscode-api-tests tsconfig rootDir', tsconfig.compilerOptions?.rootDir === './src', String(tsconfig.compilerOptions?.rootDir));
	check('vscode-api-tests tsconfig outDir', tsconfig.compilerOptions?.outDir === './out', String(tsconfig.compilerOptions?.outDir));
	check('vscode-api-tests tsconfig includes public vscode.d.ts', tsconfig.include?.includes('../../src/vscode-dts/vscode.d.ts') === true, JSON.stringify(tsconfig.include));
	check('vscode-api-tests tsconfig includes proposed d.ts glob', tsconfig.include?.includes('../../src/vscode-dts/vscode.proposed.*.d.ts') === true, JSON.stringify(tsconfig.include));
}

function validateSidecarHooks(sidecarText, commandsText, bridgeText, runtimeText) {
	includesAll(sidecarText, 'Tauri extension sidecar token transport hooks', [
		'SIDECAR_TOKEN_ENV_NAME',
		'VSCODE_ATOMIC_EXTENSION_HOST_TRANSPORT',
		'.env(SIDECAR_TOKEN_ENV_NAME, &config.handshake.token)',
		'transport_env_value(&config.handshake.transport)',
		'pub fn generate_handshake_token()',
		'pub fn preferred_transport_endpoint()',
		'"startSidecar" | "start"',
		'"stopSidecar" | "stop"',
		'"status"',
		'"markCrashed"',
		'EVENT_HANDSHAKE',
		'EVENT_LIFECYCLE',
		'EVENT_CALLBACK',
		'EVENT_RPC',
		'extensionHost.invalidSidecarToken',
		'extensionHost.sidecarNotReady',
	]);

	includesAll(commandsText, 'Tauri cancellation command hooks', [
		'pub async fn cancel_request',
		'cancel_request_impl',
		'cancellations.cancel(&request)',
		'fn canceled_response',
		'code: "Canceled".to_string()',
	]);

	includesAll(bridgeText, 'Tauri bridge cancellation contract hooks', [
		'export type CancellationId = string;',
		'readonly cancellationId?: CancellationId;',
		'export interface CancelRequest',
		'readonly cancel_request: CancelRequest;',
		'export type ExtensionSidecarEvent',
		'export const ExtensionHostSidecarRpcEvent',
	]);

	includesAll(runtimeText, 'Tauri runtime sidecar registration hooks', [
		'register_extension_host_service',
		'crate::commands::cancel_request',
	]);
}

function validateLaunchPlan() {
	for (const suite of suites) {
		check(`${suite.label} workspace exists`, existsSync(suite.workspace), suite.workspace);
		check(`${suite.label} extension development path exists`, existsSync(apiTestsRoot), apiTestsRoot);
		check(`${suite.label} expected compiled tests path configured`, suite.testsPath.endsWith(path.join('extensions', 'vscode-api-tests', 'out', suite.outputFolder)), suite.testsPath);
	}

	const prerequisiteDiagnostics = collectApiTestPrerequisiteDiagnostics({ mode });
	for (const diagnostic of prerequisiteDiagnostics.warnings) {
		const [name, detail] = splitDiagnostic(diagnostic);
		warn(name, detail);
	}
}

function sharedLaunchArgs() {
	return [
		'--enable-proposed-api=vscode.vscode-api-tests',
		`--extensionDevelopmentPath=${apiTestsRoot}`,
		'--disable-telemetry',
		'--disable-experiments',
		'--skip-welcome',
		'--skip-release-notes',
		'--no-cached-data',
		'--disable-updates',
		'--use-inmemory-secretstorage',
		'--disable-workspace-trust',
	];
}

function suiteLaunchArgs(suite) {
	return [
		suite.workspace,
		...sharedLaunchArgs(),
		`--extensionTestsPath=${suite.testsPath}`,
	];
}

function printLaunchPlan() {
	const executable = process.env.CODE_TAURI_APP_PATH || '<CODE_TAURI_APP_PATH>';

	console.log(`mode ${mode}`);
	for (const suite of suites) {
		console.log(`plan ${suite.label}: ${[executable, ...suiteLaunchArgs(suite)].join(' ')}`);
	}
}

function runExecuteSuites() {
	const executable = process.env.CODE_TAURI_APP_PATH;
	if (!executable || failures.length) {
		return;
	}

	let executedSuites = 0;
	for (const suite of suites) {
		const launchArgs = suiteLaunchArgs(suite);
		console.log(`execute ${suite.label}: ${[executable, ...launchArgs].join(' ')}`);
		const result = spawnSync(executable, launchArgs, {
			cwd: repoRoot,
			env: {
				...process.env,
				VSCODE_ATOMIC_TAURI_API_TEST_GATE: suite.label,
			},
			encoding: 'utf8',
			maxBuffer: 64 * 1024 * 1024,
			timeout: executionTimeoutMs,
		});

		if (result.stdout) {
			process.stdout.write(result.stdout);
		}
		if (result.stderr) {
			process.stderr.write(result.stderr);
		}

		if (result.error) {
			check(`${suite.label} executed against CODE_TAURI_APP_PATH`, false, result.error.message);
			continue;
		}

		const detail = `status=${result.status} signal=${result.signal ?? 'none'}`;
		if (check(`${suite.label} executed against CODE_TAURI_APP_PATH`, result.status === 0, detail)) {
			executedSuites++;
		}
	}

	check('vscode-api-tests execute gate ran all suites', executedSuites === suites.length, `${executedSuites}/${suites.length}`);
}

if (isMain) {
	const dtsBefore = readRequired(vscodeDtsPath, 'public vscode.d.ts');
	const dtsHashBefore = sha256(dtsBefore);
	const packageJson = parseJson(readRequired(path.join(apiTestsRoot, 'package.json'), 'vscode-api-tests package.json'), 'vscode-api-tests package.json');
	const tsconfig = parseJson(readRequired(path.join(apiTestsRoot, 'tsconfig.json'), 'vscode-api-tests tsconfig'), 'vscode-api-tests tsconfig');
	const sidecarText = readRequired(sidecarPath, 'Tauri extension sidecar source');
	const commandsText = readRequired(commandsPath, 'Tauri commands source');
	const bridgeText = readRequired(bridgePath, 'Tauri bridge source');
	const runtimeText = readRequired(runtimePath, 'Tauri runtime source');

	validateApiTestsManifest(packageJson);
	validateApiTestsTsconfig(tsconfig);
	validateSidecarHooks(sidecarText, commandsText, bridgeText, runtimeText);
	validateLaunchPlan();
	printLaunchPlan();

	const dtsAfter = readRequired(vscodeDtsPath, 'public vscode.d.ts after validation');
	check('public vscode.d.ts hash unchanged', sha256(dtsAfter) === dtsHashBefore, dtsHashBefore);

	if (mode === 'execute') {
		const prerequisiteDiagnostics = collectApiTestPrerequisiteDiagnostics({ mode });
		for (const diagnostic of prerequisiteDiagnostics.failures) {
			const [name, detail] = splitDiagnostic(diagnostic);
			check(name, false, detail);
		}
		runExecuteSuites();
	}

	console.log(`summary failures=${failures.length} warnings=${warnings.length}`);

	if (failures.length) {
		process.exit(1);
	}
}
