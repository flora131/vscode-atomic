# Pattern Finder 18: VS Code → Tauri/Rust Port — CI/Build Architecture

**Scope:** `.github/` directory (workflows, instructions, skills).

**Sentinel Finding:** This partition documents **build orchestration and test harness patterns** that reveal the multi-platform architecture any Tauri port must replicate.

---

## Core Patterns from CI/Build

#### Pattern: Multi-Platform Test Matrix
**Where:** `.github/workflows/pr.yml:115-176`
**What:** Parallel test execution across three rendering engines (Electron, Browser, Remote) and three platforms (Linux, macOS, Windows). Each platform runs the same test suite permutations.

```yaml
linux-electron-tests:
  name: Linux
  uses: ./.github/workflows/pr-linux-test.yml
  with:
    job_name: Electron
    electron_tests: true

linux-browser-tests:
  name: Linux
  uses: ./.github/workflows/pr-linux-test.yml
  with:
    job_name: Browser
    browser_tests: true

linux-remote-tests:
  name: Linux
  uses: ./.github/workflows/pr-linux-test.yml
  with:
    job_name: Remote
    remote_tests: true

# Pattern repeats for macos-*-tests and windows-*-tests
```

**Variations:** `.github/workflows/pr-linux-test.yml:282-378` (Electron/Browser/Remote conditional execution), `pr-darwin-test.yml`, `pr-win32-test.yml`.

---

#### Pattern: CLI Rust Build Pipeline
**Where:** `.github/workflows/pr-linux-cli-test.yml:23-48`
**What:** Dedicated Rust toolchain setup and cargo-based build/test. Shows existing Rust infrastructure for CLI tools, distinct from TypeScript/Electron main codebase.

```yaml
- name: Install Rust
  run: |
    set -e
    curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain $RUSTUP_TOOLCHAIN
    echo "$HOME/.cargo/bin" >> $GITHUB_PATH

- name: Set Rust version
  run: |
    set -e
    rustup default $RUSTUP_TOOLCHAIN
    rustup update $RUSTUP_TOOLCHAIN
    rustup component add clippy

- name: Clippy lint
  run: cargo clippy -- -D warnings
  working-directory: cli

- name: 🧪 Run unit tests
  run: cargo test
  working_directory: cli
```

**Key aspects:**
- Separate `cli/` workspace directory
- Rustup pinned to input `toolchain` version (e.g. 1.88)
- Clippy linting required before tests pass
- Cargo workspaces isolated from Node.js build

**Variations:** Same pattern invoked from main workflow at `.github/workflows/pr.yml:108-113`.

---

#### Pattern: Transpile vs. Bundle Phases
**Where:** `.github/instructions/buildNext.instructions.md:6-120`
**What:** Two-stage build system splitting fast iteration (transpile, type-check only) from production (bundle, minify). Critical for understanding how Tauri would need separate dev and release builds.

**Transpile (Dev/Watch):**
```typescript
// node build/next/index.ts transpile --out out-test
// Fast TS → JS via esbuild.transform()
// Copies ALL non-TS files from src/
// Type-checking deferred to separate tsgo process
```

**Bundle (Production):**
```typescript
// node build/next/index.ts bundle --nls --target server-web --out out-vscode-reh-web-test
// TS → bundled JS via esbuild.build()
// Applies minification, NLS placeholder injection, source maps
// Curated resource lists (not catch-all)
```

**Variations:** Desktop, web, server-web targets have different entry points (`.github/instructions/buildNext.instructions.md:88-96`).

---

#### Pattern: Node Modules Caching Strategy
**Where:** `.github/workflows/pr.yml:34-46`, `pr-linux-test.yml:58-70`
**What:** Hash-based caching of `node_modules` across workflow runs using computed package-lock digest. Shows how to avoid expensive reinstalls on platforms with native builds.

```bash
# Compute cache key from package-lock + platform + arch
node build/azure-pipelines/common/computeNodeModulesCacheKey.ts linux $VSCODE_ARCH $(node -p process.arch) > .build/packagelockhash

# Restore from cache
uses: actions/cache/restore@v5
with:
  path: .build/node_modules_cache
  key: "node_modules-linux-${{ hashFiles('.build/packagelockhash') }}"

# Extract and skip npm ci if cache hit
if: steps.cache-node-modules.outputs.cache-hit == 'true'
  run: tar -xzf .build/node_modules_cache/cache.tgz
```

**Variations:** Built-in extensions cached separately (`.github/workflows/pr.yml:86-92`), Copilot extension build cache (`.github/workflows/pr.yml:194-204`).

---

#### Pattern: Resilient Network Operations
**Where:** `.github/workflows/pr-linux-test.yml:261-278`, `pr.yml:54-64`
**What:** Exponential backoff retry loops for npm ci and Electron/Playwright downloads. Critical for flaky network in CI environments.

```bash
for i in {1..5}; do # try 5 times
  npm ci && break
  if [ $i -eq 5 ]; then
    echo "Npm install failed too many times" >&2
    exit 1
  fi
  echo "Npm install failed $i, trying again..."
done

for i in {1..3}; do # Electron/Playwright: 3 retries
  if npm exec -- npm-run-all2 -lp "electron ${{ env.VSCODE_ARCH }}" "playwright-install"; then
    echo "Download successful on attempt $i"
    break
  fi
  if [ $i -eq 3 ]; then
    echo "Download failed after 3 attempts" >&2
    exit 1
  fi
  echo "Download failed on attempt $i, retrying..."
  sleep 5
done
```

**Key aspects:**
- npm ci has 5 retries (compilation-heavy)
- Electron/Playwright has 3 retries with 5s delay
- Clear failure messages and exit codes

---

#### Pattern: Cross-Platform System Setup
**Where:** `.github/workflows/pr-linux-test.yml:36-56` (Linux), `pr-darwin-test.yml:50-76` (macOS)
**What:** Platform-specific dependency installation and environment setup. Shows surface area a Tauri port must handle.

**Linux:**
```bash
./build/azure-pipelines/linux/apt-retry.sh sudo apt-get update
./build/azure-pipelines/linux/apt-retry.sh sudo apt-get install -y pkg-config \
  xvfb libgtk-3-0 libxkbfile-dev libkrb5-dev libgbm1 rpm bubblewrap socat
sudo service xvfb start
```

**macOS:**
```bash
c++ --version
xcode-select -print-path
python3 -m pip install --break-system-packages setuptools
# GYP_DEFINES: "kerberos_use_rtld=false" (Kerberos dlopen workaround)
```

**Windows:** (implied from `pr-win32-test.yml` invocation) — PowerShell-based, 7z archives, native build tools.

---

#### Pattern: Test Suite Organization
**Where:** `.github/workflows/pr-linux-test.yml:282-378`
**What:** Layered test execution: unit tests → integration tests → smoke tests, with platform-specific runners and artifact collection on failure.

```bash
# Unit tests (Electron + node.js)
./scripts/test.sh --tfs "Unit Tests"
npm run test-node

# Unit tests (Browser)
npm run test-browser-no-install -- --browser chromium --tfs "Browser Unit Tests"

# Integration tests (compile first)
npm run gulp compile-extension:* ...
./scripts/test-integration.sh --tfs "Integration Tests"

# Smoke tests
npm run smoketest-no-compile -- --tracing

# Artifact collection on failure
- name: Publish Crash Reports
  uses: actions/upload-artifact@v7
  if: failure()
  with:
    name: crash-dump-linux-${{ env.VSCODE_ARCH }}-...
    path: .build/crashes
```

**Variations:** Browser tests use `--headless` flag, remote tests use `--remote` flag, all test scripts source from `./scripts/test*.sh` or `npm run` commands.

---

#### Pattern: Hygiene and Type Checking
**Where:** `.github/workflows/pr.yml:100-106`
**What:** Pre-compilation validation gates: TypeScript type checking on build scripts, ESLint, dependency cycle detection.

```yaml
- name: Compile & Hygiene
  run: npm exec -- npm-run-all2 -lp core-ci hygiene eslint valid-layers-check define-class-fields-check vscode-dts-compile-check tsec-compile-check test-build-scripts

- name: Check cyclic dependencies
  run: node build/lib/checkCyclicDependencies.ts out-build
```

**Key commands:**
- `core-ci` — main esbuild transpile
- `hygiene` — code style/formatting
- `eslint` — linting
- `valid-layers-check` — architecture validation
- `checkCyclicDependencies` — graph analysis

---

## Architecture Insights for Porting

#### 1. **Multiple Runtime Targets Must Be Testable**
The workflows enforce parity across Electron (desktop), Browser (web), and Remote (SSH/tunnel). A Tauri port would need:
- Separate test matrix for native Tauri desktop builds
- Continued support for web/browser targets
- Remote testing infrastructure (if keeping tunneling)

#### 2. **Rust Toolchain Is Already Isolated**
The `cli/` Rust workspace is tested separately from the main Node.js build. A Tauri port could extend this pattern: keep Rust builds in a designated workspace, with separate rustup toolchain pinning and cargo-based CI.

#### 3. **Build Stages Are Distinct**
Transpile (fast, type-check deferred) vs. Bundle (slow, minified, resource-curated) suggests a Tauri port needs:
- Fast dev iteration via transpile-like Rust incremental builds
- Separate production bundling step
- Type checking in parallel (not blocking transpile)

#### 4. **Platform-Specific Setup Is Inevitable**
Linux (apt + xvfb), macOS (Xcode + GYP), Windows (PowerShell + 7z) require per-OS CI branches. Tauri's native layer would require similar or more platform-specific logic.

#### 5. **Crash Reporting and Diagnostics Are Built-in**
Workflow artifacts capture crash dumps, node modules, and logs on failure. A Tauri port would need equivalent crash reporter integration and artifact collection.

#### 6. **Retry Logic for Flaky Operations Is Essential**
Network operations (npm ci, downloads) use exponential backoff. Any Tauri port CI should inherit this pattern for downloading Rust deps, Tauri CLI, etc.

---

## Summary

The `.github/` directory reveals a **multi-platform, multi-stage CI architecture** with:
- 9 concurrent test jobs (3 platforms × 3 runtimes)
- Separate Rust CLI build pipeline (already in use)
- Two-phase TypeScript build (transpile + bundle)
- Extensive caching and retry logic
- Platform-specific system dependencies

Any Tauri port must replicate these patterns, likely by extending the existing Rust pipeline and introducing native Tauri-specific targets to the test matrix.
