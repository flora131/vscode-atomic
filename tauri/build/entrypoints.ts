/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

// Per-target entry point lists, canonical names from build/next/index.ts.
// All paths relative to src/ (no leading slash, no .ts extension).

export type BuildTarget = 'desktop' | 'server' | 'server-web' | 'web';

// Workers shared between targets
export const workerEntryPoints = [
	'vs/editor/common/services/editorWebWorkerMain',
	'vs/workbench/api/worker/extensionHostWorkerMain',
	'vs/workbench/contrib/notebook/common/services/notebookWebWorkerMain',
	'vs/workbench/services/languageDetection/browser/languageDetectionWebWorkerMain',
	'vs/workbench/services/search/worker/localFileSearchMain',
	'vs/workbench/contrib/output/common/outputLinkComputerMain',
	'vs/workbench/services/textMate/browser/backgroundTokenization/worker/textMateTokenizationWorker.workerMain',
];

// Desktop-only workers (use electron-browser)
export const desktopWorkerEntryPoints = [
	'vs/platform/profiling/electron-browser/profileAnalysisWorkerMain',
];

// Desktop workbench and code entry points
export const desktopEntryPoints = [
	'vs/workbench/workbench.desktop.main',
	'vs/sessions/sessions.desktop.main',
	'vs/workbench/contrib/debug/node/telemetryApp',
	'vs/platform/files/node/watcher/watcherMain',
	'vs/platform/terminal/node/ptyHostMain',
	'vs/platform/agentHost/node/agentHostMain',
	'vs/platform/agentHost/node/diffWorkerMain',
	'vs/workbench/api/node/extensionHostProcess',
];

export const codeEntryPoints = [
	'vs/code/node/cliProcessMain',
	'vs/code/electron-utility/sharedProcess/sharedProcessMain',
	'vs/code/electron-browser/workbench/workbench',
	'vs/sessions/electron-browser/sessions',
];

// Web entry points (server-web and vscode-web)
export const webEntryPoints = [
	'vs/workbench/workbench.web.main.internal',
	'vs/code/browser/workbench/workbench',
];

// Additional web-only entry points (CDN build only, not server-web)
export const webOnlyEntryPoints = [
	'vs/sessions/sessions.web.main.internal',
];

export const keyboardMapEntryPoints = [
	'vs/workbench/services/keybinding/browser/keyboardLayouts/layout.contribution.linux',
	'vs/workbench/services/keybinding/browser/keyboardLayouts/layout.contribution.darwin',
	'vs/workbench/services/keybinding/browser/keyboardLayouts/layout.contribution.win',
];

// Server (reh) entry points
export const serverEntryPoints = [
	'vs/workbench/api/node/extensionHostProcess',
	'vs/platform/files/node/watcher/watcherMain',
	'vs/platform/terminal/node/ptyHostMain',
	'vs/platform/agentHost/node/agentHostMain',
	'vs/platform/agentHost/node/diffWorkerMain',
];

// Extension host bundles — excluded from private-field mangling (expose API surface)
export const extensionHostEntryPoints = [
	'vs/workbench/api/node/extensionHostProcess',
	'vs/workbench/api/worker/extensionHostWorkerMain',
];

/**
 * Returns the main entry points (non-bootstrap) for a given target.
 */
export function getEntryPointsForTarget(target: BuildTarget): string[] {
	switch (target) {
		case 'desktop':
			return [
				...workerEntryPoints,
				...desktopWorkerEntryPoints,
				...desktopEntryPoints,
				...codeEntryPoints,
			];
		case 'server':
			return [...serverEntryPoints];
		case 'server-web':
			return [
				...serverEntryPoints,
				...workerEntryPoints,
				...webEntryPoints,
				...keyboardMapEntryPoints,
			];
		case 'web':
			return [
				...workerEntryPoints,
				...webOnlyEntryPoints,
				'vs/workbench/workbench.web.main.internal',
				...keyboardMapEntryPoints,
			];
		default:
			throw new Error(`Unknown target: ${target}`);
	}
}

/**
 * Bootstrap entry points per target (no src/ prefix, relative to repo root).
 */
export function getBootstrapEntryPointsForTarget(target: BuildTarget): string[] {
	switch (target) {
		case 'desktop':
			return ['main', 'cli', 'bootstrap-fork'];
		case 'server':
		case 'server-web':
			return ['server-main', 'server-cli', 'bootstrap-fork'];
		case 'web':
			return []; // served by external server
		default:
			throw new Error(`Unknown target: ${target}`);
	}
}

/**
 * Workbench entry points that need bundled CSS (outdir rather than outfile in esbuild).
 */
export function getCssBundleEntryPointsForTarget(target: BuildTarget): Set<string> {
	switch (target) {
		case 'desktop':
			return new Set([
				'vs/workbench/workbench.desktop.main',
				'vs/code/electron-browser/workbench/workbench',
			]);
		case 'server-web':
		case 'web':
			return new Set([
				'vs/workbench/workbench.web.main.internal',
				'vs/code/browser/workbench/workbench',
			]);
		case 'server':
			return new Set();
		default:
			throw new Error(`Unknown target: ${target}`);
	}
}

/**
 * Returns true if the bundle at filePath is an extension-host bundle
 * and should NOT have private fields mangled.
 */
export function isExtensionHostBundle(filePath: string): boolean {
	const normalized = filePath.replace(/\\/g, '/');
	return extensionHostEntryPoints.some(ep => normalized.endsWith(`${ep}.js`));
}
