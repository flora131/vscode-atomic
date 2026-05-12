/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const manifestPath = process.env.CODE_TAURI_PARITY_MANIFEST_PATH || path.join(repoRoot, 'scripts', 'tauri-parity-gate.manifest.json');
const failures = [];

const requiredSmokeAreas = Object.freeze([
	'git',
	'typescriptTsserver',
	'debug',
	'terminal',
	'notebooks',
	'auth',
	'tunnels',
	'copilotApi',
]);
const requiredSmokePlatforms = Object.freeze(['macos', 'windows', 'linux']);
const builtInExtensionParityState = 'BuiltInExtensionParityGreen';

const requiredSmokeMatrix = Object.freeze(requiredSmokeAreas.map(area => Object.freeze({
	area,
	command: `npm run smoketest:tauri:builtin-extensions -- --area ${area}`,
	platforms: requiredSmokePlatforms,
})));

function parseArgs(args) {
	const selectedAreas = [];
	for (let index = 0; index < args.length; index++) {
		const arg = args[index];
		if (arg === '--area') {
			const area = args[++index];
			if (!check('--area value provided', typeof area === 'string' && area.length > 0, String(area))) {
				continue;
			}
			selectedAreas.push(area);
			continue;
		}

		if (arg?.startsWith('--area=')) {
			selectedAreas.push(arg.slice('--area='.length));
			continue;
		}

		check('CLI argument supported', false, String(arg));
	}

	if (selectedAreas.length === 0) {
		return requiredSmokeAreas;
	}

	for (const area of selectedAreas) {
		check(`requested smoke area ${String(area)} supported`, requiredSmokeAreas.includes(area), String(area));
	}

	return selectedAreas.filter(area => requiredSmokeAreas.includes(area));
}

function check(name, ok, detail) {
	const status = ok ? 'ok' : 'fail';
	console.log(`${status} [builtin-extensions] ${name}${detail ? `: ${detail}` : ''}`);
	if (!ok) {
		failures.push(`${name}${detail ? `: ${detail}` : ''}`);
	}
	return ok;
}

function readJson(relativePath) {
	const filePath = path.isAbsolute(relativePath) ? relativePath : path.join(repoRoot, relativePath);
	if (!check(`file ${relativePath}`, existsSync(filePath), filePath)) {
		return undefined;
	}

	try {
		return JSON.parse(readFileSync(filePath, 'utf8'));
	} catch (error) {
		check(`JSON parses ${relativePath}`, false, error.message);
		return undefined;
	}
}

function readText(relativePath) {
	const filePath = path.join(repoRoot, relativePath);
	if (!check(`file ${relativePath}`, existsSync(filePath), filePath)) {
		return undefined;
	}
	return readFileSync(filePath, 'utf8');
}

function arrayIncludes(name, values, expected) {
	check(name, Array.isArray(values) && values.includes(expected), Array.isArray(values) ? values.join(', ') : String(values));
}

function isNonEmptyString(value) {
	return typeof value === 'string' && value.trim().length > 0;
}

function countByField(records, field) {
	const counts = new Map();
	for (const record of records) {
		counts.set(record?.[field], (counts.get(record?.[field]) ?? 0) + 1);
	}
	return counts;
}

function contributionIncludes(name, manifest, section, field, expected) {
	const contributions = manifest?.contributes?.[section] ?? [];
	check(name, Array.isArray(contributions) && contributions.some(entry => entry?.[field] === expected), expected);
}

function commandsInclude(name, manifest, command) {
	contributionIncludes(name, manifest, 'commands', 'command', command);
}

function authProvidersInclude(name, manifest, id) {
	contributionIncludes(name, manifest, 'authentication', 'id', id);
}

function languageModelToolsInclude(name, manifest, toolName) {
	contributionIncludes(name, manifest, 'languageModelTools', 'name', toolName);
}

function validateGit() {
	const git = readJson('extensions/git/package.json');
	const gitBase = readJson('extensions/git-base/package.json');
	const api = readText('extensions/git/src/api/git.d.ts');

	check('Git extension package name', git?.name === 'git', String(git?.name));
	arrayIncludes('Git activates on git file system', git?.activationEvents, 'onFileSystem:git');
	arrayIncludes('Git depends on git-base', git?.extensionDependencies, 'vscode.git-base');
	commandsInclude('Git command git.clone contributed', git, 'git.clone');
	commandsInclude('Git command git.init contributed', git, 'git.init');
	check('Git public API exposes Repository', api?.includes('export interface Repository') === true);
	check('Git base extension package name', gitBase?.name === 'git-base', String(gitBase?.name));
}

function validateTypeScript() {
	const ts = readJson('extensions/typescript-language-features/package.json');
	const tsBasics = readJson('extensions/typescript-basics/package.json');
	const versionProvider = readText('extensions/typescript-language-features/src/tsServer/versionProvider.electron.ts');

	check('TypeScript extension package name', ts?.name === 'typescript-language-features', String(ts?.name));
	arrayIncludes('TypeScript activates on TypeScript language', ts?.activationEvents, 'onLanguage:typescript');
	arrayIncludes('TypeScript exposes tsserver request command activation', ts?.activationEvents, 'onCommand:typescript.tsserverRequest');
	check('TypeScript extension main entry', ts?.main === './out/extension', String(ts?.main));
	check('TypeScript extension browser entry', ts?.browser === './dist/browser/extension', String(ts?.browser));
	check('TypeScript tsserver resolver references bundled tsserver.js', versionProvider?.includes('\'typescript\', \'lib\', \'tsserver.js\'') === true);
	check('TypeScript basics extension manifest name', tsBasics?.name === 'typescript', String(tsBasics?.name));
}

function validateDebug() {
	const autoLaunch = readJson('extensions/debug-auto-launch/package.json');
	const serverReady = readJson('extensions/debug-server-ready/package.json');

	check('Debug auto-launch package name', autoLaunch?.name === 'debug-auto-launch', String(autoLaunch?.name));
	arrayIncludes('Debug auto-launch activates on startup', autoLaunch?.activationEvents, 'onStartupFinished');
	commandsInclude('Debug auto-attach command contributed', autoLaunch, 'extension.node-debug.toggleAutoAttach');
	check('Debug server-ready package name', serverReady?.name === 'debug-server-ready', String(serverReady?.name));
	arrayIncludes('Debug server-ready activates on debug resolve', serverReady?.activationEvents, 'onDebugResolve');
	check('Debug server-ready debugger contribution exists', Array.isArray(serverReady?.contributes?.debuggers) && serverReady.contributes.debuggers.some(entry => entry?.type === '*'));
}

function validateTerminal() {
	const terminalSuggest = readJson('extensions/terminal-suggest/package.json');
	const debugServerReady = readJson('extensions/debug-server-ready/package.json');
	const copilot = readJson('extensions/copilot/package.json');

	check('Terminal suggest package name', terminalSuggest?.name === 'terminal-suggest', String(terminalSuggest?.name));
	arrayIncludes('Terminal suggest activates on shell integration', terminalSuggest?.activationEvents, 'onTerminalShellIntegration:*');
	check('Terminal completion provider contributed', Array.isArray(terminalSuggest?.contributes?.terminal?.completionProviders) && terminalSuggest.contributes.terminal.completionProviders.length > 0);
	arrayIncludes('Terminal suggest API proposal terminalCompletionProvider', terminalSuggest?.enabledApiProposals, 'terminalCompletionProvider');
	arrayIncludes('Debug server-ready API proposal terminalDataWriteEvent', debugServerReady?.enabledApiProposals, 'terminalDataWriteEvent');
	arrayIncludes('Copilot API proposal terminalExecuteCommandEvent', copilot?.enabledApiProposals, 'terminalExecuteCommandEvent');
}

function validateNotebooks() {
	const ipynb = readJson('extensions/ipynb/package.json');
	const renderers = readJson('extensions/notebook-renderers/package.json');

	check('IPYNB package name', ipynb?.name === 'ipynb', String(ipynb?.name));
	arrayIncludes('IPYNB activates on jupyter notebook', ipynb?.activationEvents, 'onNotebook:jupyter-notebook');
	check('IPYNB contributes jupyter-notebook', Array.isArray(ipynb?.contributes?.notebooks) && ipynb.contributes.notebooks.some(entry => entry?.type === 'jupyter-notebook'));
	check('Notebook renderers package name', renderers?.name === 'builtin-notebook-renderers', String(renderers?.name));
	check('Builtin notebook renderer contributed', Array.isArray(renderers?.contributes?.notebookRenderer) && renderers.contributes.notebookRenderer.some(entry => entry?.id === 'vscode.builtin-renderer'));
}

function validateAuth() {
	const github = readJson('extensions/github-authentication/package.json');
	const microsoft = readJson('extensions/microsoft-authentication/package.json');

	check('GitHub authentication package name', github?.name === 'github-authentication', String(github?.name));
	authProvidersInclude('GitHub authentication provider github contributed', github, 'github');
	authProvidersInclude('GitHub Enterprise authentication provider contributed', github, 'github-enterprise');
	arrayIncludes('GitHub authentication API proposal authProviderSpecific', github?.enabledApiProposals, 'authProviderSpecific');
	check('Microsoft authentication package name', microsoft?.name === 'microsoft-authentication', String(microsoft?.name));
	authProvidersInclude('Microsoft authentication provider contributed', microsoft, 'microsoft');
	arrayIncludes('Microsoft authentication API proposal authenticationChallenges', microsoft?.enabledApiProposals, 'authenticationChallenges');
}

function validateTunnels() {
	const tunnelForwarding = readJson('extensions/tunnel-forwarding/package.json');

	check('Tunnel forwarding package name', tunnelForwarding?.name === 'tunnel-forwarding', String(tunnelForwarding?.name));
	arrayIncludes('Tunnel forwarding activates on tunnel', tunnelForwarding?.activationEvents, 'onTunnel');
	arrayIncludes('Tunnel forwarding API proposal resolvers', tunnelForwarding?.enabledApiProposals, 'resolvers');
	arrayIncludes('Tunnel forwarding API proposal tunnelFactory', tunnelForwarding?.enabledApiProposals, 'tunnelFactory');
	commandsInclude('Tunnel forwarding show log command contributed', tunnelForwarding, 'tunnel-forwarding.showLog');
	commandsInclude('Tunnel forwarding restart command contributed', tunnelForwarding, 'tunnel-forwarding.restart');
}

function validateCopilotApi() {
	const copilot = readJson('extensions/copilot/package.json');

	check('Copilot package name', copilot?.name === 'copilot-chat', String(copilot?.name));
	arrayIncludes('Copilot activates on language model chat', copilot?.activationEvents, 'onLanguageModelChat:copilot');
	languageModelToolsInclude('Copilot codebase tool contributed', copilot, 'copilot_searchCodebase');
	languageModelToolsInclude('Copilot notebook structure tool contributed', copilot, 'copilot_getNotebookSummary');
	arrayIncludes('Copilot API proposal chatProvider', copilot?.enabledApiProposals, 'chatProvider');
	arrayIncludes('Copilot API proposal languageModelToolSupportsModel', copilot?.enabledApiProposals, 'languageModelToolSupportsModel');
	arrayIncludes('Copilot API proposal authLearnMore', copilot?.enabledApiProposals, 'authLearnMore');
}

function validateSmokeMatrix(manifest) {
	const gates = Array.isArray(manifest?.gates) ? manifest.gates : [];
	const stateMachine = Array.isArray(manifest?.migrationStateMachine) ? manifest.migrationStateMachine : [];
	const builtInExtensionGate = gates.find(gate => gate?.state === builtInExtensionParityState);
	const areas = Array.isArray(builtInExtensionGate?.builtInExtensionParity?.areas) ? builtInExtensionGate.builtInExtensionParity.areas : [];
	const currentIndex = stateMachine.indexOf(manifest?.currentState);
	const builtInExtensionIndex = stateMachine.indexOf(builtInExtensionParityState);
	const builtInExtensionReached = currentIndex >= 0 && builtInExtensionIndex >= 0 && currentIndex >= builtInExtensionIndex;
	const gatePassed = builtInExtensionGate?.lastCiResult?.status === 'pass';
	const requirePass = builtInExtensionReached || gatePassed;

	check('BuiltInExtensionParityGreen gate exists', builtInExtensionGate !== undefined);
	check('built-in extension smoke matrix areas array', Array.isArray(builtInExtensionGate?.builtInExtensionParity?.areas), typeof builtInExtensionGate?.builtInExtensionParity?.areas);
	check('built-in extension smoke matrix exactly required areas', areas.length === requiredSmokeMatrix.length, String(areas.length));
	check('BuiltInExtensionParityGreen gate not passed before CI evidence', !gatePassed || builtInExtensionReached, `${String(manifest?.currentState)} / ${String(builtInExtensionGate?.lastCiResult?.status)}`);
	if (requirePass) {
		check('BuiltInExtensionParityGreen gate CI run name', isNonEmptyString(builtInExtensionGate?.lastCiResult?.runName), String(builtInExtensionGate?.lastCiResult?.runName));
		check('BuiltInExtensionParityGreen gate CI run id', isNonEmptyString(builtInExtensionGate?.lastCiResult?.ciRunId) && builtInExtensionGate.lastCiResult.ciRunId !== 'not-yet-run', String(builtInExtensionGate?.lastCiResult?.ciRunId));
		check('BuiltInExtensionParityGreen gate CI artifact URL', isNonEmptyString(builtInExtensionGate?.lastCiResult?.ciArtifactUrl), String(builtInExtensionGate?.lastCiResult?.ciArtifactUrl));
	}

	const areaCounts = countByField(areas, 'area');
	for (const expected of requiredSmokeMatrix) {
		check(`built-in extension smoke matrix area ${expected.area}`, areaCounts.get(expected.area) === 1, String(areaCounts.get(expected.area) ?? 0));
		const area = areas.find(candidate => candidate?.area === expected.area);
		validateSmokeMatrixArea(area, expected, requirePass);
	}
}

function validateSmokeMatrixArea(area, expected, requirePass) {
	const areaName = expected.area;
	const platformResults = Array.isArray(area?.platformResults) ? area.platformResults : [];
	check(`built-in extension smoke matrix ${areaName} owner`, area?.owner === 'builtin-extensions', String(area?.owner));
	check(`built-in extension smoke matrix ${areaName} command`, area?.command === expected.command, String(area?.command));
	check(`built-in extension smoke matrix ${areaName} status`, ['pass', 'fail'].includes(area?.status), String(area?.status));
	check(`built-in extension smoke matrix ${areaName} summary`, isNonEmptyString(area?.summary), String(area?.summary));
	check(`built-in extension smoke matrix ${areaName} platformResults array`, Array.isArray(area?.platformResults), typeof area?.platformResults);
	check(`built-in extension smoke matrix ${areaName} platformResults exactly macos/windows/linux`, platformResults.length === requiredSmokePlatforms.length, String(platformResults.length));
	if (requirePass) {
		check(`built-in extension smoke matrix ${areaName} status pass`, area?.status === 'pass', String(area?.status));
	} else {
		check(`built-in extension smoke matrix ${areaName} status remains fail before CI evidence`, area?.status === 'fail', String(area?.status));
	}

	const platformCounts = countByField(platformResults, 'platform');
	for (const platform of expected.platforms) {
		check(`built-in extension smoke matrix ${areaName} platform ${platform}`, platformCounts.get(platform) === 1, String(platformCounts.get(platform) ?? 0));
		const platformResult = platformResults.find(candidate => candidate?.platform === platform);
		validateSmokeMatrixPlatform(areaName, platform, platformResult, requirePass);
	}
}

function validateSmokeMatrixPlatform(areaName, platform, result, requirePass) {
	check(`built-in extension smoke matrix ${areaName} ${platform} command`, result?.command === 'npm run smoketest:tauri:builtin-extensions', String(result?.command));
	check(`built-in extension smoke matrix ${areaName} ${platform} status`, ['pass', 'fail'].includes(result?.status), String(result?.status));
	check(`built-in extension smoke matrix ${areaName} ${platform} summary`, isNonEmptyString(result?.summary), String(result?.summary));
	if (requirePass) {
		check(`built-in extension smoke matrix ${areaName} ${platform} status pass`, result?.status === 'pass', String(result?.status));
	} else {
		check(`built-in extension smoke matrix ${areaName} ${platform} status remains fail before CI evidence`, result?.status === 'fail', String(result?.status));
	}
}

const smokeValidators = Object.freeze({
	git: validateGit,
	typescriptTsserver: validateTypeScript,
	debug: validateDebug,
	terminal: validateTerminal,
	notebooks: validateNotebooks,
	auth: validateAuth,
	tunnels: validateTunnels,
	copilotApi: validateCopilotApi,
});

const selectedSmokeAreas = parseArgs(process.argv.slice(2));
const visitedSmokeAreas = new Set();

validateSmokeMatrix(readJson(manifestPath));

for (const area of selectedSmokeAreas) {
	const validator = smokeValidators[area];
	if (!check(`smoke area ${area} validator registered`, typeof validator === 'function')) {
		continue;
	}
	visitedSmokeAreas.add(area);
	validator();
}

for (const area of selectedSmokeAreas) {
	check(`smoke area ${area} executed`, visitedSmokeAreas.has(area));
}

console.log(`summary [builtin-extensions] failures=${failures.length}`);

if (failures.length) {
	process.exit(1);
}
