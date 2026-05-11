# VS Code Atomic Tauri packaging

Status: internal scaffold only. `scripts/tauri-parity-gate.manifest.json` currently reports `currentState` as `Scaffold`, so all Tauri package and smoke scripts are release-gated work in progress unless this manifest reaches `ReleaseCandidate`.

`code-tauri` packaging is intentionally separate from existing Electron packaging.

## Entrypoints

- Dry-run command plan: `npm run package:tauri:dry-run`
- Release-only invariant validation: `CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html npm run package:tauri:release-validate`
- Execute package build: `npm run package:tauri -- --target <rust-target-triple> --bundles <bundle-list>`
- Developer scaffold smoke validation: `npm run smoketest:tauri:scaffold`
- Bridge source-smoke validation: `npm run test:tauri:bridge`
- VS Code API sidecar preflight: `npm run test:tauri:api`
- VS Code API sidecar execute gate: `CODE_TAURI_APP_PATH=<path-to-tauri-app> npm run test-extension:tauri-sidecar:execute`
- Release real workbench smoke preflight: `CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html npm run smoketest:tauri:workbench`
- Parity state gate: `node scripts/tauri-parity-gate.mjs`
- Direct script help: `node scripts/tauri-package.mjs --help`

Dry-run is the default planning mode. `npm run package:tauri:dry-run` prints the packaging command plan and signing placeholders without executing package commands. It is usable before `ReleaseCandidate`; when release invariants are not satisfied it emits `Warning: Tauri package dry-run release invariants are not satisfied:` and still exits successfully. Dry-run has no release gate requirement, is not release validation, and must not run `cargo tauri build`.

`npm run package:tauri:release-validate` is release-only. Run it through the explicit real workbench gate: `CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html npm run package:tauri:release-validate`. It validates release packaging invariants without running package commands, requires `CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html`, cannot use URL or scaffold fallback, and is expected to fail while `scripts/tauri-parity-gate.manifest.json` `currentState` remains `Scaffold`. It prints `Tauri package release validation passed.` only when `scripts/tauri-parity-gate.manifest.json` `currentState` equals `ReleaseCandidate` and all release invariants pass.

`npm run package:tauri` passes `--execute --flavor code-tauri --sign` to `scripts/tauri-package.mjs`. Execute mode enforces the same release invariants before distributable packaging. If any invariant fails, it throws `Tauri package execute validation failed:` before running package commands.

Release invariants:

- `scripts/tauri-parity-gate.manifest.json` `currentState === "ReleaseCandidate"`.
- `package.json` `tauriMigration.status` matches `scripts/tauri-parity-gate.manifest.json` `currentState`.
- `releaseValidation.lastCiResult.status === "pass"`.
- `SignedPackageGreen` gate `lastCiResult.status === "pass"`.
- `SignedPackageGreen.signedPackageEvidence.artifacts[]` records exactly one macOS, one Windows, and one Linux release artifact; each platform artifact requires `signatureStatus === "pass"`, `notarizationStatus === "pass"` where required or `"not_required"` where not required, `installStatus === "pass"`, `launchStatus === "pass"`, and a non-empty `osCheckCommand` containing platform install and launch checks. Artifact evidence must not contain signing secret fields or inline key/certificate/password material.
- `ReleaseCandidate` gate `lastCiResult.status === "pass"`.
- Real workbench is required through `CODE_TAURI_REQUIRE_REAL_WORKBENCH=1` and `CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html`; the explicit package release validation command is `CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html npm run package:tauri:release-validate` and the explicit real workbench smoke preflight command is `CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html npm run smoketest:tauri:workbench`.
- Packaged release loads bundled app asset `src-tauri/www/out/vs/code/browser/workbench/workbench.html` through `ResolvedWorkbenchUrl::App("out/vs/code/browser/workbench/workbench.html")`.
- Scaffold fallback is forbidden in release: `src-tauri/www/index.html` must be forbidden and absent from allowed workbench sources.
- Generated fixtures, copied scaffold output, and static placeholder pages do not count as real workbench boot evidence.
- Signing is enabled with `--sign`; signing secrets still come only from CI-provided credentials. `src-tauri/tauri.conf.json` must not store private keys, passwords, tokens, certificate blobs, or certificate secret paths.

Runtime and scaffold-safe smoke validation keep developer fallback source order:

1. `CODE_TAURI_WORKBENCH_URL`
2. `CODE_TAURI_WORKBENCH_PATH`
3. Bundled app asset `src-tauri/www/out/vs/code/browser/workbench/workbench.html` when present.
4. Source checkout `out/vs/code/browser/workbench/workbench.html`.
5. `src-tauri/www/index.html` only when real workbench is not required.

Release validation accepts only `CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html` as the real workbench source. Release validation always rejects URL and scaffold fallback: `src-tauri/www/index.html` must remain in `releaseValidation.forbiddenWorkbenchSources` and must not appear in `releaseValidation.allowedWorkbenchSources`.

Ubuntu Linux packaging requires WebKitGTK development packages before `pkg-config` checks and runtime Cargo builds:

```sh
sudo apt-get update
sudo apt-get install -y \
  pkg-config \
  libglib2.0-dev \
  libgtk-3-dev \
  libwebkit2gtk-4.1-dev
```

These packages provide native libraries pulled by the `src-tauri/Cargo.toml` `runtime` feature through Tauri 2: `glib-2.0`, `gobject-2.0`, `gio-2.0`, `gdk-3.0`, and `webkit2gtk-4.1`.

When execute mode passes release invariants, it runs, in order:

1. `npm run compile-build`
2. Validate `out/vs/code/browser/workbench/workbench.html` exists and copy the generated `out` workbench assets into `src-tauri/www/out`.
3. `cargo build --manifest-path src-tauri/Cargo.toml --features runtime`
4. `cargo tauri build --config src-tauri/tauri.conf.json --bundles all`

Use `--skip-frontend` or `--skip-rust-check` only when release engineering has already produced those artifacts in the current workspace. `--skip-frontend` still validates and copies the generated real workbench assets before Rust/package commands.

## Workbench WebView source

The Tauri runtime creates the main WebView in Rust so it can choose the first available source:

1. `CODE_TAURI_WORKBENCH_URL` for a dev server or hosted workbench URL.
2. `CODE_TAURI_WORKBENCH_PATH` for a generated `workbench.html` file.
3. Bundled app asset `src-tauri/www/out/vs/code/browser/workbench/workbench.html` for packaged builds.
4. `out/vs/code/browser/workbench/workbench.html` when present in a source checkout.
5. `src-tauri/www/index.html` developer-only scaffold fallback for local debug smoke only.

`src-tauri/www/index.html` is debug-only developer scaffold. The scaffold fallback is P0 internal scaffolding only. It is forbidden in release, not a release artifact, not a completed scaffold, not migration completion, and not evidence that the Tauri workbench is shippable. The packaging pipeline copies the generated TypeScript workbench bundle into `src-tauri/www/out`; packaged release must load `src-tauri/www/out/vs/code/browser/workbench/workbench.html` as a Tauri app asset. Never include or describe scaffold-only boot as a release path. Release validation must always reject `src-tauri/www/index.html` as a workbench source.

## Developer scaffold smoke validation

`npm run smoketest:tauri:scaffold` is developer scaffold-safe. `npm run smoketest:tauri` is a compatibility alias for this scaffold smoke. It validates current `code-tauri` source, dependency, and runtime guards without launching a Tauri GUI or requiring real workbench artifacts. It checks:

- `src-tauri/tauri.conf.json` product, identifier, frontend distribution, runtime-created window mode, developer scaffold `bundle.active === false` guard, and non-null strict CSP directives.
- Scaffold guard: `src-tauri/www/index.html` stays developer-only while scaffold config keeps `bundle.active === false`; this check is not a release packaging invariant.
- release validation must use the real workbench bundle, not scaffold fallback.
- WebView boot fallback order: `CODE_TAURI_WORKBENCH_URL`, `CODE_TAURI_WORKBENCH_PATH`, bundled app asset `src-tauri/www/out/vs/code/browser/workbench/workbench.html`, source checkout `out/vs/code/browser/workbench/workbench.html`, then `src-tauri/www/index.html`.
- Core command availability: channel calls, listen/dispose/cancel, and file service commands.
- File service command routing for stat, read, write, delete, mkdir, readdir, and watch.
- Extension sidecar lifecycle state and handshake environment names.
- Local availability of non-GUI tools and Linux GUI build dependency `webkit2gtk-4.1`. Local runs warn when runtime deps are absent; `CODE_TAURI_STRICT_DEPS=1` and CI fail instead.

This smoke test is intentionally source/dependency/runtime-only so CI and headless agent environments can validate scaffold guards before full Tauri WebView packaging is available. It is not real workbench validation, not signed release validation, and not evidence that release packaging is ready. PR CI runs it with `CODE_TAURI_STRICT_DEPS=1`, so missing runtime dependencies or skipped runtime behavior tests fail.

Release packaging must not use scaffold smoke results or `src-tauri/www/index.html`. Release validation requires the real workbench path `out/vs/code/browser/workbench/workbench.html` and either `bundle.active === true` in the generated release config or an equivalent generated release configuration consumed by packaging CI.

`npm run smoketest:tauri:workbench` validates the release workbench boot preflight only when forced through the explicit gate command `CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html npm run smoketest:tauri:workbench`. Deterministic real workbench preflight rejects `src-tauri/www/index.html` as a release workbench path, requires the generated `out/vs/code/browser/workbench/workbench.html` file, and records GUI launch as skipped in headless environments until platform CI can run the actual WebView smoke.

`npm run test:tauri:bridge` intentionally runs `node scripts/tauri-smoke.mjs source` until local TypeScript bridge tests can use an existing checked-in runner without adding new dependencies. This source-smoke validates the `TauriBridgeCommands` contract, Rust `tauri::generate_handler!` registrations, command ordering, package script wiring, and related Tauri source invariants without requiring `mocha` from `node_modules`.

## VS Code API sidecar validation

`npm run test:tauri:api` is the clean preflight. It runs `npm run test-extension:tauri-sidecar:preflight`, does not launch a GUI, does not edit sources, and validates that the Tauri Node extension-host sidecar is wired to the `vscode-api-tests` contract. Missing compiled `extensions/vscode-api-tests/out` suites are warnings in preflight only.

`npm run test-extension:tauri-sidecar:execute` first runs `npm run compile-extension:vscode-api-tests`, then runs `node scripts/tauri-vscode-api-tests.mjs --execute`. Execute mode requires `CODE_TAURI_APP_PATH` to point at an existing Tauri app binary. It runs both `vscode-api-tests-folder` and `vscode-api-tests-workspace` against that app with `--extensionDevelopmentPath=extensions/vscode-api-tests`, `--extensionTestsPath=<compiled-suite>`, proposed API enabled, telemetry disabled, updates disabled, and in-memory secret storage. Execute mode fails if compiled API test output or `CODE_TAURI_APP_PATH` is missing.

## Migration status

`code-tauri` is at `Scaffold` status because `scripts/tauri-parity-gate.manifest.json` `currentState` is `Scaffold`. This status means incomplete P0 internal developer scaffolding only; it does not mean migration complete. No scaffold-only rollout is allowed. Rollout stays blocked until every parity gate below has an owner, a repeatable CI validation command, a recorded pass result with `ciRunId`, `observedAt`, `summary`, and either `ciArtifactUrl` or `ciArtifactNotApplicableReason` clean CI evidence for each promoted gate, release sign-off, and `scripts/tauri-parity-gate.manifest.json` `currentState` equals `ReleaseCandidate`.

`package.json` `tauriMigration.status` is scaffold metadata only and must match `scripts/tauri-parity-gate.manifest.json` `currentState`; the parity gate fails if they diverge.

Migration state machine:

`Scaffold -> RuntimeBuildGreen -> RealWorkbenchBootGreen -> CoreServicesParityGreen -> ExtensionApiParityGreen -> BuiltInExtensionParityGreen -> SignedPackageGreen -> ReleaseCandidate`

Only `ReleaseCandidate` may be described as migration complete. Release notes, changelogs, milestone summaries, and packaging docs must not call the Tauri migration complete before `scripts/tauri-parity-gate.manifest.json` `currentState` equals `ReleaseCandidate`. No completion claim is allowed before `ReleaseCandidate`. Permitted language before then: incomplete internal scaffold, developer-only scaffold, developer smoke, blocked scaffold, or migration in progress.

Release governance requires `scripts/tauri-parity-gate.manifest.json` `currentState` equal `ReleaseCandidate` after all real work is green: real workbench boot, Rust runtime build, core service parity, extension API parity, built-in extension parity, signed package validation, and release validation. Scaffold status never satisfies any release completion claim.

### Parity gates

| Gate | Owner | Command | `lastCiResult` | Rollout blocker |
|---|---|---|---|---|
| Runtime build | `tauri-runtime` | `npm run compile-build && cargo build --manifest-path src-tauri/Cargo.toml --features runtime` | `status=fail`; `runName=Build Tauri runtime`; `observedAt=2026-05-11`; `summary=Not promoted from Scaffold until clean CI runtime build is recorded.` | Block until Rust runtime and TypeScript build artifacts compile from clean CI inputs on Windows, macOS, and Linux. |
| Workbench boot | `workbench-platform` | `CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html npm run smoketest:tauri:workbench` | `status=fail`; `runName=Tauri real workbench boot smoke`; `observedAt=2026-05-11`; `summary=Blocked until real workbench boot passes without src-tauri/www/index.html.` | Block until the real TypeScript workbench bundle boots, renders, restores state, and reaches workbench-ready on Windows, macOS, and Linux CI without scaffold fallback. |
| File command parity | `platform-services` | `npm run smoketest:tauri:scaffold && npm run test:tauri:bridge` | `status=fail`; `runName=Tauri core services parity`; `observedAt=2026-05-11`; `summary=Blocked until file command parity suites pass.` | Block until Tauri file commands match Electron behavior, permissions, error codes, path handling, and remote/local semantics required by built-in workbench flows. |
| Watcher cancellation | `platform-services` | `npm run smoketest:tauri:scaffold && npm run test:tauri:bridge` | `status=fail`; `runName=Tauri core services parity`; `observedAt=2026-05-11`; `summary=Blocked until watcher cancellation suites pass.` | Block until file watchers and channel subscriptions cancel deterministically with no leaked events, orphaned watches, late delivery, or cross-window delivery. |
| Strict CSP | `security` | `npm run smoketest:tauri:scaffold && node scripts/tauri-parity-gate.mjs` | `status=fail`; `runName=Tauri strict CSP validation`; `observedAt=2026-05-11`; `summary=Blocked until packaged WebView CSP audit and unsafe script probes pass.` | Block until packaged workbench uses strict CSP compatible with VS Code security requirements and rejects unsafe inline, eval, remote, and extension-injected script paths. |
| Extension API conformance | `extension-host` | `npm run test:tauri:api && CODE_TAURI_APP_PATH=<path-to-tauri-app> npm run test-extension:tauri-sidecar:execute && npm run smoketest:tauri:scaffold` | `status=fail`; `runName=Tauri extension API parity`; `observedAt=2026-05-11`; `summary=Blocked until extension host bridge, lifecycle, API, and proposed API suites pass.` | Block until extension host bridge API parity, sidecar startup/shutdown, token handling, stdio/rpc handshake, stable API, and required proposed API coverage pass in CI with compiled API tests and an existing `CODE_TAURI_APP_PATH`. |
| Terminal parity | `terminal` | `npm run smoketest:tauri:scaffold && npm run test-node -- --runGlob src/vs/workbench/contrib/terminal/test/**/*.test.ts` | `status=fail`; `runName=Tauri terminal parity`; `observedAt=2026-05-11`; `summary=Blocked until terminal service parity suites pass.` | Block until shell detection, PTY lifecycle, input/output streaming, resize, environment resolution, cwd handling, process cleanup, reconnection, and shutdown match required Electron behavior. |
| GUI smoke | `workbench-platform` | `npm run smoketest && npm run smoketest:tauri:scaffold` | `status=fail`; `runName=Tauri GUI smoke`; `observedAt=2026-05-11`; `summary=Blocked until packaged or served real workbench GUI smoke passes on all release platforms.` | Block until window creation, workbench load, commands, settings, file open/save, terminal, extension activation, shutdown, and reload pass on Windows, macOS, and Linux CI. |
| Startup/perf | `performance` | `npm run perf` | `status=fail`; `runName=Tauri startup and perf`; `observedAt=2026-05-11`; `summary=Blocked until startup, memory, CPU, and extension host budgets are defined and non-regressing.` | Block until startup time, first window ready, extension host ready, memory, and CPU budgets are defined, measured, and non-regressing. |
| Bridge latency | `platform-services` | `npm run smoketest:tauri:scaffold && npm run test:tauri:bridge` | `status=fail`; `runName=Tauri bridge latency`; `observedAt=2026-05-11`; `summary=Blocked until p50/p95/p99 bridge latency budgets pass in CI.` | Block until p50/p95/p99 latency budgets pass in CI and regressions fail the build. |
| Built-in extension parity | `builtin-extensions` | `npm run smoketest:tauri:scaffold && npm run smoketest:tauri:builtin-extensions && node build/lib/builtInExtensions.ts` | `status=fail`; `runName=Tauri built-in extension parity`; `observedAt=2026-05-11`; `summary=Blocked until deterministic built-in extension preflight plus Git, TypeScript/tsserver, debug, terminal, notebooks, auth, tunnels, Copilot-relevant API, and file I/O parity pass.` | Block until `scripts/tauri-parity-gate.manifest.json` `builtInExtensionParity.areas` records one owner, command, pass/fail status, and summary, and macOS/Windows/Linux platform result for Git, TypeScript/tsserver, debug, terminal, notebooks, auth, tunnels, and Copilot-relevant API. `BuiltInExtensionParityGreen` cannot promote until every required area and platform result is `pass`. |
| Packaging/signing | `release-engineering` | `npm run package:tauri -- --target <rust-target-triple> --bundles <bundle-list>` | `status=fail`; `runName=Tauri signed package validation`; `observedAt=2026-05-11`; `summary=Blocked until signed, notarized where required, installable, launchable artifacts pass.` | Block until signed, notarized where required, installable, launchable, and updater-compatible artifacts are produced by CI. |

### Required observability metrics

Tauri parity CI must publish these metrics for every GUI smoke, API conformance, and bridge latency run:

- `channelCall.invocations`: count of workbench-to-sidecar channel calls by command and result.
- `channelListen.active`: active per-window channel subscriptions after listen, dispose, cancel, window close, and shutdown.
- `channelEvent.delivered`: channel events delivered to the owning window and listener.
- `channelEvent.dropped`: channel events rejected, orphaned, late, unauthorized, or sent after disposal.
- `fileWrite.failures`: failed write attempts by reason, path class, and command source.
- `bridge.roundTripMs`: Tauri command, sidecar, and extension host round-trip latency with p50/p95/p99.

CI must fail rollout gates when required metrics are absent, malformed, or outside approved budgets. Metrics must not include file contents, secret values, sidecar tokens, signing material, or user-identifying data.

### Security and privacy blockers

Rollout stays blocked until these controls pass review and CI validation:

- Strict command allowlist for every Tauri command exposed to the WebView; deny unknown commands by default.
- Per-window subscription ownership for `listen`, `dispose`, `cancel`, and event delivery; no cross-window event leakage.
- Sidecar token rotation for each launch/session, with token values never logged, persisted, or exposed to extensions.
- No signing secrets in repo; signing, notarization, updater, and package credentials must come only from release CI secret stores.
- Metrics and smoke logs must redact paths, tokens, credentials, file contents, and extension-provided sensitive values before artifact upload.

### Enforced parity state gate

`node scripts/tauri-parity-gate.mjs` validates `scripts/tauri-parity-gate.manifest.json`. It fails on pre-ReleaseCandidate docs or package metadata that include a migration completion claim, if `package.json` `tauriMigration.status` diverges from manifest `currentState`, if release validation omits `CODE_TAURI_REQUIRE_REAL_WORKBENCH=1`, if release validation permits `src-tauri/www/index.html` scaffold fallback, if release validation allowed sources include `src-tauri/www/index.html`, if any parity gate lacks `owner`, `command`, or `lastCiResult`, or if promoted gates lack clean CI evidence in `lastCiResult.ciRunId`, `lastCiResult.observedAt`, `lastCiResult.summary`, and either `lastCiResult.ciArtifactUrl` or `lastCiResult.ciArtifactNotApplicableReason`. Parity gates need CI evidence before promotion; local-only pass output does not satisfy release governance.

### Validation commands

- Tauri developer scaffold source/dependency/runtime smoke: `npm run smoketest:tauri:scaffold`
- Tauri scaffold smoke compatibility alias: `npm run smoketest:tauri`
- Tauri bridge source-smoke validation: `npm run test:tauri:bridge`
- Tauri API sidecar clean preflight: `npm run test:tauri:api`
- Tauri API sidecar execute gate: `CODE_TAURI_APP_PATH=<path-to-tauri-app> npm run test-extension:tauri-sidecar:execute`
- Tauri release real workbench smoke preflight: `CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html npm run smoketest:tauri:workbench`
- Tauri parity state gate: `node scripts/tauri-parity-gate.mjs`
- Existing Electron smoke: `npm run smoketest`
- Tauri package dry-run: `npm run package:tauri:dry-run`
- Tauri package release-only validation, expected fail before `ReleaseCandidate`: `CODE_TAURI_REQUIRE_REAL_WORKBENCH=1 CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html npm run package:tauri:release-validate`
- Tauri package execute: `npm run package:tauri -- --target <rust-target-triple> --bundles <bundle-list>`
- Direct package script help: `node scripts/tauri-package.mjs --help`
- Rust runtime compile check: `cargo build --manifest-path src-tauri/Cargo.toml --features runtime`

### Known environment blockers

- Linux GUI build/run requires `webkit2gtk-4.1` visible to `pkg-config`; `npm run smoketest:tauri:scaffold` warns locally and fails when `CODE_TAURI_STRICT_DEPS=1` or `CI=true`.
- Dry-run package planning requires local Node.js dependencies and can run before `ReleaseCandidate`; release invariant failures are warnings only in dry-run mode.
- Release validation and full package builds require `scripts/tauri-parity-gate.manifest.json` `currentState === "ReleaseCandidate"`, release validation status pass, `SignedPackageGreen` and `ReleaseCandidate` gates pass, exactly one macOS, one Windows, and one Linux signed package artifact with passing signature/notarization where needed/install/launch evidence, real workbench, no scaffold fallback, and signing enabled.
- Full package builds require local `cargo`, `cargo tauri`, Node.js dependencies, and platform SDKs for target bundles.
- Signing and notarization require CI-owned secrets; no signing secrets live in this repository and signing secrets must not be committed.
- GUI release validation needs `CODE_TAURI_REQUIRE_REAL_WORKBENCH=1` plus `CODE_TAURI_WORKBENCH_PATH=out/vs/code/browser/workbench/workbench.html`. `src-tauri/www/index.html` remains developer-only and is disallowed for release validation.

### Open owner questions

- Bridge owner: who owns 200+ extHost message mappings and bridge latency budget for hover/completion?
- Workbench owner: who validates Monaco GPU/WebView behavior on WebKit, WebView2, and WebKitGTK?
- Linux release owner: who owns Snap confinement and AppArmor updates for the Tauri bundle?
- Windowing owner: who owns multi-window service mapping onto Tauri per-window WebViews?
- API owner: who owns proposed-API pinning, compatibility policy, and upstream churn tracking?
- Extension owner: who owns `engines.vscode` compatibility versus a new Tauri-specific extension manifest marker?
- Copilot owner: who owns Copilot smoke success and release sign-off?
- Release owner: who owns signed Windows, macOS, and Linux packaging CI plus updater credentials?

## Signing and notarization placeholders

No signing secrets live in this repository.

Release CI should provide platform credentials through environment variables or secret files mounted at build time:

- Tauri updater signatures: `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
- Windows code signing: certificate path/password or signing service variables owned by release engineering
- macOS code signing/notarization: Apple Developer ID certificate, `APPLE_TEAM_ID`, and either `APPLE_ID`/`APPLE_PASSWORD` or App Store Connect API key variables
- Linux package signing: distro-specific signing key material from release CI

`node scripts/tauri-package.mjs --sign --notarize` records expected secret placeholders in the command plan without reading or printing secret values. Release validation rejects `SignedPackageGreen.signedPackageEvidence` and checked-in Tauri bundle config that include secret-like fields or inline signing material.
