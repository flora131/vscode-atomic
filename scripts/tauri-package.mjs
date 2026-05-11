#!/usr/bin/env node
// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const tauriConfig = path.join(repoRoot, 'src-tauri', 'tauri.conf.json');
const tauriManifest = path.join(repoRoot, 'src-tauri', 'Cargo.toml');

const args = new Set(process.argv.slice(2));
const values = new Map();
for (let i = 2; i < process.argv.length; i++) {
	const arg = process.argv[i];
	if (arg.startsWith('--') && process.argv[i + 1] && !process.argv[i + 1].startsWith('--')) {
		values.set(arg, process.argv[++i]);
	}
}

const execute = args.has('--execute');
const flavor = values.get('--flavor') ?? 'code-tauri';
const target = values.get('--target');
const bundles = values.get('--bundles') ?? 'all';

if (args.has('--help')) {
	console.log(`Usage: node scripts/tauri-package.mjs [options]

Creates the VS Code Atomic Tauri packaging command plan. Defaults to dry-run.

Options:
  --execute              Run commands instead of printing them.
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
