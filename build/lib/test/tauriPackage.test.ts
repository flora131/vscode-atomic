/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { suite, test } from 'node:test';
import * as assert from 'assert';
import * as child_process from 'child_process';
import * as fs from 'fs';
import * as os from 'os';
import * as path from 'path';

const repoRoot = path.join(import.meta.dirname, '../../..');
const packageScriptPath = path.join(repoRoot, 'scripts/tauri-package.mjs');
const requiredWorkbenchSource = 'out/vs/code/browser/workbench/workbench.html';
type SpawnResult = child_process.SpawnSyncReturns<string>;

interface FixturePaths {
	tempRoot: string;
	binDir: string;
}

interface FixtureOptions {
	createWorkbench?: boolean;
	createBundledWorkbench?: boolean;
}

interface RunOptions {
	env?: NodeJS.ProcessEnv;
}

function assertOutputOrder(output: string, earlier: string, later: string): void {
	const earlierIndex = output.indexOf(earlier);
	const laterIndex = output.indexOf(later);
	assert.notStrictEqual(earlierIndex, -1, `missing ${earlier}\n${output}`);
	assert.notStrictEqual(laterIndex, -1, `missing ${later}\n${output}`);
	assert.ok(earlierIndex < laterIndex, `expected ${earlier} before ${later}\n${output}`);
}

function writeJson(filePath: string, value: unknown): void {
	fs.mkdirSync(path.dirname(filePath), { recursive: true });
	fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`);
}

function createReleaseCandidateManifest(): unknown {
	const passResult = {
		status: 'pass',
		runName: 'fixture',
		ciRunId: 'fixture',
		ciArtifactUrl: 'https://example.invalid/fixture',
		observedAt: '2026-05-11',
		summary: 'fixture',
	};

	return {
		schemaVersion: 1,
		currentState: 'ReleaseCandidate',
		releaseValidation: {
			command: `node scripts/tauri-parity-gate.mjs && CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 CODE_TAURI_WORKBENCH_PATH=${requiredWorkbenchSource} npm run smoketest:tauri:workbench && CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 CODE_TAURI_WORKBENCH_PATH=${requiredWorkbenchSource} npm run package:tauri:release-validate`,
			lastCiResult: passResult,
			permitsScaffoldFallback: false,
			allowedWorkbenchSources: ['CODE_TAURI_WORKBENCH_PATH', requiredWorkbenchSource],
			forbiddenWorkbenchSources: ['src-tauri/www/index.html'],
		},
		gates: [
			{
				state: 'SignedPackageGreen',
				lastCiResult: passResult,
				signedPackageEvidence: {
					artifacts: [
						{
							os: 'macos',
							artifactPath: 'dist/code-tauri.dmg',
							signatureStatus: 'pass',
							notarizationStatus: 'pass',
							installStatus: 'pass',
							launchStatus: 'pass',
							osCheckCommand: 'codesign --verify && spctl --assess && hdiutil verify && cp -R app /Applications && open -W app',
						},
						{
							os: 'windows',
							artifactPath: 'dist/code-tauri.exe',
							signatureStatus: 'pass',
							notarizationStatus: 'not_required',
							installStatus: 'pass',
							launchStatus: 'pass',
							osCheckCommand: 'Get-AuthenticodeSignature code-tauri.exe; Start-Process code-tauri.exe --version',
						},
						{
							os: 'linux',
							artifactPath: 'dist/code-tauri.AppImage',
							signatureStatus: 'pass',
							notarizationStatus: 'not_required',
							installStatus: 'pass',
							launchStatus: 'pass',
							osCheckCommand: 'gpg --verify code-tauri.AppImage.sig && chmod +x code-tauri.AppImage && ./code-tauri.AppImage --appimage-extract-and-run --version',
						},
					],
				},
			},
			{ state: 'ReleaseCandidate', lastCiResult: passResult },
		],
	};
}

function createScaffoldManifest(): unknown {
	const manifest = createReleaseCandidateManifest() as { currentState: string };
	manifest.currentState = 'Scaffold';
	return manifest;
}

function createFixture({ createWorkbench = true, createBundledWorkbench = createWorkbench }: FixtureOptions = {}): FixturePaths {
	const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'vscode-atomic-tauri-package-test-'));
	fs.mkdirSync(path.join(tempRoot, 'scripts'), { recursive: true });
	fs.copyFileSync(packageScriptPath, path.join(tempRoot, 'scripts/tauri-package.mjs'));

	writeJson(path.join(tempRoot, 'package.json'), {
		type: 'module',
		scripts: {
			'compile-build': `node -e "require('fs').writeFileSync('npm-compile-build-invoked','1');require('fs').mkdirSync('${path.dirname(requiredWorkbenchSource)}',{recursive:true});require('fs').writeFileSync('${requiredWorkbenchSource}','<!doctype html>\\n')"`,
			'package:tauri': 'node scripts/tauri-package.mjs --execute --flavor code-tauri --sign --skip-frontend --skip-rust-check',
			'package:tauri:with-all': 'node scripts/tauri-package.mjs --execute --flavor code-tauri --sign',
			'package:tauri:with-frontend': 'node scripts/tauri-package.mjs --execute --flavor code-tauri --sign --skip-rust-check',
			'package:tauri:release-validate': 'node scripts/tauri-package.mjs --release-validate --flavor code-tauri --sign',
		},
	});
	writeJson(path.join(tempRoot, 'src-tauri/tauri.conf.json'), {
		productName: 'VS Code Atomic Fixture',
		version: '0.1.0',
		identifier: 'dev.vscode.atomic.fixture',
		build: { frontendDist: 'www' },
		app: { windows: [], security: { csp: 'default-src \'self\'' } },
		bundle: { active: false, targets: 'all' },
	});
	fs.mkdirSync(path.join(tempRoot, 'src-tauri'), { recursive: true });
	fs.writeFileSync(path.join(tempRoot, 'src-tauri/Cargo.toml'), '[package]\nname = "fixture"\nversion = "0.1.0"\nedition = "2021"\n');
	writeJson(path.join(tempRoot, 'scripts/tauri-parity-gate.manifest.json'), createReleaseCandidateManifest());
	if (createWorkbench) {
		fs.mkdirSync(path.join(tempRoot, path.dirname(requiredWorkbenchSource)), { recursive: true });
		fs.writeFileSync(path.join(tempRoot, requiredWorkbenchSource), '<!doctype html>\n');
	}
	if (createBundledWorkbench) {
		const bundledWorkbenchSource = path.join('src-tauri/www', requiredWorkbenchSource);
		fs.mkdirSync(path.join(tempRoot, path.dirname(bundledWorkbenchSource)), { recursive: true });
		fs.writeFileSync(path.join(tempRoot, bundledWorkbenchSource), '<!doctype html>\n');
	}

	const binDir = path.join(tempRoot, 'bin');
	fs.mkdirSync(binDir, { recursive: true });
	const cargoChecker = path.join(binDir, 'cargo-check.mjs');
	fs.writeFileSync(cargoChecker, `import fs from 'node:fs';\nfs.appendFileSync('cargo-shim-invocations.log', \`\${process.argv.slice(2).join(' ')}\\n\`);\nconst configIndex = process.argv.indexOf('--config');\nif (configIndex === -1) {\n\tconsole.error('missing --config');\n\tprocess.exit(1);\n}\nconst configPath = process.argv[configIndex + 1];\nconst config = JSON.parse(fs.readFileSync(configPath, 'utf8'));\nif (config.bundle?.active !== true) {\n\tconsole.error(\`bundle.active was \${String(config.bundle?.active)}\`);\n\tprocess.exit(42);\n}\nif (!configPath.endsWith('tauri.release.conf.json')) {\n\tconsole.error(\`expected generated release config, got \${configPath}\`);\n\tprocess.exit(43);\n}\nconsole.log(\`validated generated config \${configPath}\`);\n`);
	const cargoShim = path.join(binDir, process.platform === 'win32' ? 'cargo.cmd' : 'cargo');
	if (process.platform === 'win32') {
		fs.writeFileSync(cargoShim, `@echo off\nnode "%~dp0cargo-check.mjs" %*\n`);
	} else {
		fs.writeFileSync(cargoShim, `#!/bin/sh\nexec node "$(dirname "$0")/cargo-check.mjs" "$@"\n`);
		fs.chmodSync(cargoShim, 0o755);
	}

	return { tempRoot, binDir };
}

function withFixture(callback: (fixture: FixturePaths) => void): void;
function withFixture(options: FixtureOptions, callback: (fixture: FixturePaths) => void): void;
function withFixture(optionsOrCallback: FixtureOptions | ((fixture: FixturePaths) => void), callback?: (fixture: FixturePaths) => void): void {
	const options = typeof optionsOrCallback === 'function' ? {} : optionsOrCallback;
	const run = typeof optionsOrCallback === 'function' ? optionsOrCallback : callback;
	assert.ok(run, 'fixture callback required');

	const fixture = createFixture(options);
	try {
		run(fixture);
	} finally {
		fs.rmSync(fixture.tempRoot, { recursive: true, force: true });
	}
}

function runNpmScript(fixture: FixturePaths, script: string, options: RunOptions = {}): SpawnResult {
	return child_process.spawnSync('npm', ['run', script], {
		cwd: fixture.tempRoot,
		encoding: 'utf8',
		env: {
			...process.env,
			PATH: `${fixture.binDir}${path.delimiter}${process.env.PATH ?? ''}`,
			CODE_TAURI_REQUIRE_REAL_WORKBENCH: '1',
			CODE_TAURI_WORKBENCH_PATH: requiredWorkbenchSource,
			CODE_TAURI_WORKBENCH_URL: undefined,
			...options.env,
		},
	});
}

suite('Tauri package release config invariants', () => {

		test('release validation accepts checked-in bundle.active false through resolved release config', () => {
		withFixture(fixture => {
			const result = runNpmScript(fixture, 'package:tauri:release-validate');

			assert.strictEqual(result.status, 0, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stdout, /Tauri package marker: .*"name":"package\.marker".*"marker":"releaseValidation\.start"/);
			assert.match(result.stdout, /Tauri package marker: .*"name":"package\.marker".*"marker":"releaseValidation\.pass"/);
			assert.match(result.stdout, /Tauri package release validation passed\./);
		});
	});

	test('execute packaging consumes generated release config with bundle.active true', () => {
		withFixture(fixture => {
			const result = runNpmScript(fixture, 'package:tauri');

			assert.strictEqual(result.status, 0, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stdout, /Release config: .*tauri\.release\.conf\.json/);
			assert.match(result.stdout, /Tauri package marker: .*"marker":"packageExecute\.start"/);
			assert.match(result.stdout, /Tauri package marker: .*"marker":"packageCommand\.pass".*"step":"tauri-build"/);
			assert.match(result.stdout, /Tauri package marker: .*"marker":"packageExecute\.pass"/);
			assert.match(result.stdout, /validated generated config .*tauri\.release\.conf\.json/);
			assert.doesNotMatch(result.stderr, /bundle\.active was false/);
		});
	});

	test('execute packaging validates bundled workbench asset after copying generated source', () => {
		withFixture({ createBundledWorkbench: false }, fixture => {
			const result = runNpmScript(fixture, 'package:tauri');

			assert.strictEqual(result.status, 0, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stdout, /copy out\/vs\/code\/browser\/workbench\/workbench\.html -> src-tauri\/www\/out\/vs\/code\/browser\/workbench\/workbench\.html/);
			assert.match(result.stdout, /validated generated config .*tauri\.release\.conf\.json/);
			assert.ok(fs.existsSync(path.join(fixture.tempRoot, 'src-tauri/www', requiredWorkbenchSource)));
		});
	});

	test('execute packaging requires release invariants before frontend build commands', () => {
		withFixture({ createWorkbench: false }, fixture => {
			const result = runNpmScript(fixture, 'package:tauri:with-frontend');
			const output = `${result.stdout}\n${result.stderr}`;

			assert.strictEqual(result.status, 1, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stdout, /npm run compile-build/);
			assert.match(result.stderr, /Tauri package release validation failed:/);
			assert.match(result.stderr, /package release requires real workbench source out\/vs\/code\/browser\/workbench\/workbench\.html/);
			assertOutputOrder(output, 'Tauri package release validation failed:', 'package release requires real workbench source');
			assert.doesNotMatch(result.stdout, /validated generated config .*tauri\.release\.conf\.json/);
			assert.ok(!fs.existsSync(path.join(fixture.tempRoot, requiredWorkbenchSource)));
		});
	});

	test('execute packaging with skip frontend fails release invariants before Rust package commands', () => {
		withFixture({ createWorkbench: false }, fixture => {
			const result = runNpmScript(fixture, 'package:tauri');

			assert.strictEqual(result.status, 1, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stderr, /Tauri package release validation failed:/);
			assert.match(result.stderr, /package release requires real workbench source out\/vs\/code\/browser\/workbench\/workbench\.html/);
			assert.doesNotMatch(result.stdout, /validated generated config .*tauri\.release\.conf\.json/);
			assert.ok(!fs.existsSync(path.join(fixture.tempRoot, 'src-tauri/www', requiredWorkbenchSource)));
		});
	});

	test('execute packaging rejects scaffold parity state before npm and cargo command shims', () => {
		withFixture(fixture => {
			writeJson(path.join(fixture.tempRoot, 'scripts/tauri-parity-gate.manifest.json'), createScaffoldManifest());

			const result = runNpmScript(fixture, 'package:tauri:with-all');
			const output = `${result.stderr}\n${result.stdout}`;

			assert.strictEqual(result.status, 1, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stderr, /Tauri package release validation failed:/);
			assert.match(result.stderr, /package release requires ReleaseCandidate parity state: Scaffold/);
			assertOutputOrder(output, 'Tauri package release validation failed:', 'npm run compile-build');
			assertOutputOrder(output, 'Tauri package release validation failed:', 'cargo build');
			assertOutputOrder(output, 'Tauri package release validation failed:', 'cargo tauri build');
			assert.doesNotMatch(result.stdout, /validated generated config .*tauri\.release\.conf\.json/);
			assert.ok(!fs.existsSync(path.join(fixture.tempRoot, 'npm-compile-build-invoked')));
			assert.ok(!fs.existsSync(path.join(fixture.tempRoot, 'cargo-shim-invocations.log')));
		});
	});

	test('release validation remains strict without generated source workbench asset', () => {
		withFixture({ createWorkbench: false }, fixture => {
			const result = runNpmScript(fixture, 'package:tauri:release-validate');

			assert.strictEqual(result.status, 1, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stderr, /Tauri package release validation failed:/);
			assert.match(result.stderr, /package release requires real workbench source out\/vs\/code\/browser\/workbench\/workbench\.html/);
			assert.doesNotMatch(result.stdout, /Tauri package release validation passed\./);
		});
	});

	test('release validation rejects missing bundled workbench asset', () => {
		withFixture({ createBundledWorkbench: false }, fixture => {
			const result = runNpmScript(fixture, 'package:tauri:release-validate');

			assert.strictEqual(result.status, 1, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stderr, /Tauri package release validation failed:/);
			assert.match(result.stderr, /package release requires bundled real workbench asset src-tauri\/www\/out\/vs\/code\/browser\/workbench\/workbench\.html/);
			assert.doesNotMatch(result.stdout, /Tauri package release validation passed\./);
		});
	});

	test('release validation rejects scaffold hard-linked as bundled workbench asset', () => {
		withFixture({ createBundledWorkbench: false }, fixture => {
			const scaffoldPath = path.join(fixture.tempRoot, 'src-tauri/www/index.html');
			const bundledPath = path.join(fixture.tempRoot, 'src-tauri/www', requiredWorkbenchSource);
			fs.mkdirSync(path.dirname(scaffoldPath), { recursive: true });
			fs.mkdirSync(path.dirname(bundledPath), { recursive: true });
			fs.writeFileSync(scaffoldPath, '<!doctype html>\n');
			fs.linkSync(scaffoldPath, bundledPath);

			const result = runNpmScript(fixture, 'package:tauri:release-validate');

			assert.strictEqual(result.status, 1, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stderr, /Tauri package release validation failed:/);
			assert.match(result.stderr, /package release forbids scaffold bundled workbench asset/);
			assert.doesNotMatch(result.stdout, /Tauri package release validation passed\./);
		});
	});

	test('release validation rejects workbench URL override', () => {
		withFixture(fixture => {
			const result = runNpmScript(fixture, 'package:tauri:release-validate', {
				env: { CODE_TAURI_WORKBENCH_URL: 'http://127.0.0.1:3000' },
			});

			assert.strictEqual(result.status, 1, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stderr, /Tauri package release validation failed:/);
			assert.match(result.stderr, /package release forbids CODE_TAURI_WORKBENCH_URL during release validation/);
			assert.doesNotMatch(result.stdout, /Tauri package release validation passed\./);
		});
	});

	test('release validation rejects inline signing secrets in artifact evidence', () => {
		withFixture(fixture => {
			const manifestPath = path.join(fixture.tempRoot, 'scripts/tauri-parity-gate.manifest.json');
			const manifest = createReleaseCandidateManifest() as { gates: Array<{ state: string; signedPackageEvidence?: { artifacts: Array<Record<string, unknown>> } }> };
			const signedPackageGate = manifest.gates.find(gate => gate.state === 'SignedPackageGreen');
			assert.ok(signedPackageGate?.signedPackageEvidence);
			signedPackageGate.signedPackageEvidence.artifacts[0].privateKey = '-----BEGIN PRIVATE KEY-----\nfixture\n-----END PRIVATE KEY-----';
			writeJson(manifestPath, manifest);

			const result = runNpmScript(fixture, 'package:tauri:release-validate');

			assert.strictEqual(result.status, 1, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stderr, /Tauri package release validation failed:/);
			assert.match(result.stderr, /signedPackageEvidence\.artifacts\[0\]\.privateKey must not store signing secret metadata in repository evidence/);
			assert.match(result.stderr, /signedPackageEvidence\.artifacts\[0\]\.privateKey must not contain inline signing secret material/);
			assert.doesNotMatch(result.stdout, /Tauri package release validation passed\./);
		});
	});

	test('release validation rejects signing secrets in checked-in Tauri bundle config', () => {
		withFixture(fixture => {
			const configPath = path.join(fixture.tempRoot, 'src-tauri/tauri.conf.json');
			const config = JSON.parse(fs.readFileSync(configPath, 'utf8')) as { bundle: Record<string, unknown> };
			config.bundle.privateKey = 'TAURI_SIGNING_PRIVATE_KEY=fixture-secret';
			writeJson(configPath, config);

			const result = runNpmScript(fixture, 'package:tauri:release-validate');

			assert.strictEqual(result.status, 1, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stderr, /Tauri package release validation failed:/);
			assert.match(result.stderr, /src-tauri\/tauri\.conf\.json bundle\.privateKey must not store signing secret metadata in repository evidence/);
			assert.match(result.stderr, /src-tauri\/tauri\.conf\.json bundle\.privateKey must not contain inline signing secret material/);
			assert.doesNotMatch(result.stdout, /Tauri package release validation passed\./);
		});
	});

	test('release validation rejects scaffold workbench path override', () => {
		withFixture(fixture => {
			fs.mkdirSync(path.join(fixture.tempRoot, 'src-tauri/www'), { recursive: true });
			fs.writeFileSync(path.join(fixture.tempRoot, 'src-tauri/www/index.html'), '<!doctype html>\n');

			const result = runNpmScript(fixture, 'package:tauri:release-validate', {
				env: { CODE_TAURI_WORKBENCH_PATH: 'src-tauri/www/index.html' },
			});

			assert.strictEqual(result.status, 1, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stderr, /Tauri package release validation failed:/);
			assert.match(result.stderr, /package release requires CODE_TAURI_WORKBENCH_PATH=out\/vs\/code\/browser\/workbench\/workbench\.html/);
			assert.match(result.stderr, /package release forbids scaffold resolved workbench source/);
			assert.doesNotMatch(result.stdout, /Tauri package release validation passed\./);
		});
	});

	test('release validation rejects scaffold hard-linked as required workbench path', () => {
		withFixture({ createWorkbench: false }, fixture => {
			const scaffoldPath = path.join(fixture.tempRoot, 'src-tauri/www/index.html');
			const requiredPath = path.join(fixture.tempRoot, requiredWorkbenchSource);
			fs.mkdirSync(path.dirname(scaffoldPath), { recursive: true });
			fs.mkdirSync(path.dirname(requiredPath), { recursive: true });
			fs.writeFileSync(scaffoldPath, '<!doctype html>\n');
			fs.linkSync(scaffoldPath, requiredPath);

			const result = runNpmScript(fixture, 'package:tauri:release-validate');

			assert.strictEqual(result.status, 1, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stderr, /Tauri package release validation failed:/);
			assert.match(result.stderr, /package release forbids scaffold resolved workbench source/);
			assert.doesNotMatch(result.stdout, /Tauri package release validation passed\./);
		});
	});

	test('release validation rejects required workbench symlink outside repository root', () => {
		withFixture({ createWorkbench: false }, fixture => {
			const externalRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'vscode-atomic-tauri-external-workbench-'));
			try {
				const externalWorkbenchPath = path.join(externalRoot, 'workbench.html');
				const requiredPath = path.join(fixture.tempRoot, requiredWorkbenchSource);
				fs.writeFileSync(externalWorkbenchPath, '<!doctype html>\n');
				fs.mkdirSync(path.dirname(requiredPath), { recursive: true });
				fs.symlinkSync(externalWorkbenchPath, requiredPath);

				const result = runNpmScript(fixture, 'package:tauri:release-validate');

				assert.strictEqual(result.status, 1, `${result.stdout}\n${result.stderr}`);
				assert.match(result.stderr, /Tauri package release validation failed:/);
				assert.match(result.stderr, /package release requires real workbench source out\/vs\/code\/browser\/workbench\/workbench\.html canonical path under repository root/);
				assert.match(result.stderr, /package release requires resolved workbench source canonical path under repository root/);
				assert.doesNotMatch(result.stdout, /Tauri package release validation passed\./);
			} finally {
				fs.rmSync(externalRoot, { recursive: true, force: true });
			}
		});
	});

	test('execute packaging rejects workbench URL override before package commands', () => {
		withFixture(fixture => {
			const result = runNpmScript(fixture, 'package:tauri', {
				env: { CODE_TAURI_WORKBENCH_URL: 'http://127.0.0.1:3000' },
			});

			assert.strictEqual(result.status, 1, `${result.stdout}\n${result.stderr}`);
			assert.match(result.stderr, /Tauri package release validation failed:/);
			assert.match(result.stderr, /package release forbids CODE_TAURI_WORKBENCH_URL during release validation/);
			assert.doesNotMatch(result.stdout, /validated generated config .*tauri\.release\.conf\.json/);
		});
	});
});
